//! Debug-marker and debug-utils object terminators.

use core::ffi::c_void;

use vk::{
    PFN_vkDebugMarkerSetObjectNameEXT, PFN_vkDebugMarkerSetObjectTagEXT,
    PFN_vkSetDebugUtilsObjectNameEXT, PFN_vkSetDebugUtilsObjectTagEXT,
    VkDebugMarkerObjectNameInfoEXT, VkDebugMarkerObjectTagInfoEXT, VkDebugReportObjectTypeEXT,
    VkDebugUtilsObjectNameInfoEXT, VkDebugUtilsObjectTagInfoEXT, VkDevice, VkInstance,
    VkObjectType, VkPhysicalDevice, VkResult, VkSurfaceKHR,
};

use crate::{
    command_has_enabled_device_extension, command_has_enabled_instance_extension, command_lookup,
    device::LoaderDevice,
    instance::{LoaderInstance, LoaderPhysicalDevice, LoaderPhysicalDeviceTrampoline},
    load_typed,
    surface::native_surface,
};

fn abort_invalid_dispatch() -> ! {
    // The reference loader intentionally aborts when an always-trampoline
    // command reaches an invalid or disabled device dispatch slot.
    unsafe { libc::abort() }
}

unsafe fn checked_device(
    device: VkDevice,
    command_name: &'static core::ffi::CStr,
) -> &'static LoaderDevice {
    // SAFETY: A valid call supplies a live loader device.
    let Some(device) = (unsafe { LoaderDevice::from_handle(device) }) else {
        abort_invalid_dispatch();
    };
    let Some(command) = command_lookup(command_name) else {
        abort_invalid_dispatch();
    };
    let enabled =
        command_has_enabled_instance_extension(command.id, &device.instance().enabled_extensions)
            || command_has_enabled_device_extension(command.id, device.enabled_extensions());
    if !enabled {
        abort_invalid_dispatch();
    }
    device
}

unsafe fn translate_instance(device: &LoaderDevice, handle: u64) -> Option<u64> {
    let address = handle as usize;
    // SAFETY: Debug object metadata identifies this integer as a live instance handle.
    let instance = unsafe { LoaderInstance::from_handle(VkInstance(address as *mut c_void)) }?;
    let native = instance.icds.get(device.icd_index())?.handle;
    Some(native.0 as usize as u64)
}

unsafe fn translate_physical_device(device: &LoaderDevice, handle: u64) -> Option<u64> {
    let address = handle as usize;
    // SAFETY: Debug object metadata identifies this integer as a live physical-device handle.
    let handle = VkPhysicalDevice(address as *mut c_void);
    let physical = if let Some(physical) = unsafe { LoaderPhysicalDevice::from_handle(handle) } {
        physical
    } else {
        let trampoline = unsafe { LoaderPhysicalDeviceTrampoline::from_handle(handle) }?;
        unsafe { LoaderPhysicalDevice::from_handle(trampoline.terminator) }?
    };
    (physical.icd_index == device.icd_index()).then_some(physical.native.0 as usize as u64)
}

unsafe fn translate_surface(device: &LoaderDevice, handle: u64) -> Option<u64> {
    // SAFETY: The debug object type identifies this as a surface owned by the instance.
    unsafe { native_surface(device.instance(), device.icd_index(), VkSurfaceKHR(handle)) }
        .ok()
        .map(|surface| surface.0)
}

unsafe fn translate_instance_chain(handle: u64) -> Option<u64> {
    let address = handle as usize;
    let instance = unsafe { LoaderInstance::from_handle(VkInstance(address as *mut c_void)) }?;
    Some(instance.chain_handle().0 as usize as u64)
}

unsafe fn translate_physical_device_chain(handle: u64) -> Option<u64> {
    let address = handle as usize;
    let trampoline = unsafe {
        LoaderPhysicalDeviceTrampoline::from_handle(VkPhysicalDevice(address as *mut c_void))
    }?;
    Some(trampoline.chain.0 as usize as u64)
}

unsafe fn translate_debug_report_object_chain(
    device: &LoaderDevice,
    object_type: VkDebugReportObjectTypeEXT,
    handle: u64,
) -> u64 {
    match object_type {
        VkDebugReportObjectTypeEXT::INSTANCE => {
            unsafe { translate_instance_chain(handle) }.unwrap_or(handle)
        }
        VkDebugReportObjectTypeEXT::PHYSICAL_DEVICE => {
            unsafe { translate_physical_device_chain(handle) }.unwrap_or(handle)
        }
        VkDebugReportObjectTypeEXT::DEVICE => device.chain_device.0 as usize as u64,
        _ => handle,
    }
}

unsafe fn translate_debug_utils_object_chain(
    device: &LoaderDevice,
    object_type: VkObjectType,
    handle: u64,
) -> u64 {
    match object_type {
        VkObjectType::INSTANCE => unsafe { translate_instance_chain(handle) }.unwrap_or(handle),
        VkObjectType::PHYSICAL_DEVICE => {
            unsafe { translate_physical_device_chain(handle) }.unwrap_or(handle)
        }
        VkObjectType::DEVICE => device.chain_device.0 as usize as u64,
        _ => handle,
    }
}

unsafe fn translate_debug_report_object(
    device: &LoaderDevice,
    object_type: VkDebugReportObjectTypeEXT,
    handle: u64,
) -> u64 {
    match object_type {
        VkDebugReportObjectTypeEXT::INSTANCE => {
            // SAFETY: The object type supplies the concrete handle kind.
            unsafe { translate_instance(device, handle) }.unwrap_or(handle)
        }
        VkDebugReportObjectTypeEXT::PHYSICAL_DEVICE => {
            // SAFETY: The object type supplies the concrete handle kind.
            unsafe { translate_physical_device(device, handle) }.unwrap_or(handle)
        }
        VkDebugReportObjectTypeEXT::DEVICE => device.icd_device.0 as usize as u64,
        VkDebugReportObjectTypeEXT::SURFACE_KHR => {
            // SAFETY: The object type supplies the concrete handle kind.
            unsafe { translate_surface(device, handle) }.unwrap_or(handle)
        }
        _ => handle,
    }
}

unsafe fn translate_debug_utils_object(
    device: &LoaderDevice,
    object_type: VkObjectType,
    handle: u64,
) -> u64 {
    match object_type {
        VkObjectType::INSTANCE => {
            // SAFETY: The object type supplies the concrete handle kind.
            unsafe { translate_instance(device, handle) }.unwrap_or(handle)
        }
        VkObjectType::PHYSICAL_DEVICE => {
            // SAFETY: The object type supplies the concrete handle kind.
            unsafe { translate_physical_device(device, handle) }.unwrap_or(handle)
        }
        VkObjectType::DEVICE => device.icd_device.0 as usize as u64,
        VkObjectType::SURFACE_KHR => {
            // SAFETY: The object type supplies the concrete handle kind.
            unsafe { translate_surface(device, handle) }.unwrap_or(handle)
        }
        _ => handle,
    }
}

/// Sets driver-private debug tag data after translating loader-owned handles.
///
/// # Safety
///
/// Arguments must satisfy `vkDebugMarkerSetObjectTagEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn terminator_vkDebugMarkerSetObjectTagEXT(
    device: VkDevice,
    tag_info: *const VkDebugMarkerObjectTagInfoEXT<'_>,
) -> VkResult {
    // SAFETY: Validation and the intentional fatal path mirror the loader trampoline.
    let loader_device = unsafe { checked_device(device, c"vkDebugMarkerSetObjectTagEXT") };
    if tag_info.is_null() {
        abort_invalid_dispatch();
    }
    // SAFETY: The caller supplies one readable tag-info structure.
    let tag_info = unsafe { &*tag_info };
    let mut native_info = *tag_info;
    // SAFETY: `objectType` identifies the encoded handle kind.
    native_info.object = unsafe {
        translate_debug_report_object(loader_device, native_info.objectType, native_info.object)
    };
    // SAFETY: Resolver and native device belong to the same ICD.
    let native: Option<PFN_vkDebugMarkerSetObjectTagEXT> =
        unsafe { load_typed(loader_device.resolve(c"vkDebugMarkerSetObjectTagEXT")) };
    native.map_or(VkResult::SUCCESS, |native| {
        // SAFETY: The translated structure remains live for the native call.
        unsafe { native(loader_device.icd_device, &raw const native_info) }
    })
}

/// Assigns a debug name after translating loader-owned handles.
///
/// # Safety
///
/// Arguments must satisfy `vkDebugMarkerSetObjectNameEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn terminator_vkDebugMarkerSetObjectNameEXT(
    device: VkDevice,
    name_info: *const VkDebugMarkerObjectNameInfoEXT<'_>,
) -> VkResult {
    // SAFETY: Validation and the intentional fatal path mirror the loader trampoline.
    let loader_device = unsafe { checked_device(device, c"vkDebugMarkerSetObjectNameEXT") };
    if name_info.is_null() {
        abort_invalid_dispatch();
    }
    // SAFETY: The caller supplies one readable name-info structure.
    let name_info = unsafe { &*name_info };
    let mut native_info = *name_info;
    // SAFETY: `objectType` identifies the encoded handle kind.
    native_info.object = unsafe {
        translate_debug_report_object(loader_device, native_info.objectType, native_info.object)
    };
    // SAFETY: Resolver and native device belong to the same ICD.
    let native: Option<PFN_vkDebugMarkerSetObjectNameEXT> =
        unsafe { load_typed(loader_device.resolve(c"vkDebugMarkerSetObjectNameEXT")) };
    native.map_or(VkResult::SUCCESS, |native| {
        // SAFETY: The translated structure remains live for the native call.
        unsafe { native(loader_device.icd_device, &raw const native_info) }
    })
}

/// Sets a debug-utils name after translating loader-owned handles.
///
/// # Safety
///
/// Arguments must satisfy `vkSetDebugUtilsObjectNameEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn terminator_vkSetDebugUtilsObjectNameEXT(
    device: VkDevice,
    name_info: *const VkDebugUtilsObjectNameInfoEXT<'_>,
) -> VkResult {
    // SAFETY: Validation and the intentional fatal path mirror the loader trampoline.
    let loader_device = unsafe { checked_device(device, c"vkSetDebugUtilsObjectNameEXT") };
    if name_info.is_null() {
        abort_invalid_dispatch();
    }
    // SAFETY: The caller supplies one readable name-info structure.
    let name_info = unsafe { &*name_info };
    let mut native_info = *name_info;
    // SAFETY: `objectType` identifies the encoded handle kind.
    native_info.objectHandle = unsafe {
        translate_debug_utils_object(
            loader_device,
            native_info.objectType,
            native_info.objectHandle,
        )
    };
    // SAFETY: Resolver and native device belong to the same ICD.
    let native: Option<PFN_vkSetDebugUtilsObjectNameEXT> =
        unsafe { load_typed(loader_device.resolve(c"vkSetDebugUtilsObjectNameEXT")) };
    native.map_or(VkResult::SUCCESS, |native| {
        // SAFETY: The translated structure remains live for the native call.
        unsafe { native(loader_device.icd_device, &raw const native_info) }
    })
}

/// Sets debug-utils tag data after translating loader-owned handles.
///
/// # Safety
///
/// Arguments must satisfy `vkSetDebugUtilsObjectTagEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn terminator_vkSetDebugUtilsObjectTagEXT(
    device: VkDevice,
    tag_info: *const VkDebugUtilsObjectTagInfoEXT<'_>,
) -> VkResult {
    // SAFETY: Validation and the intentional fatal path mirror the loader trampoline.
    let loader_device = unsafe { checked_device(device, c"vkSetDebugUtilsObjectTagEXT") };
    if tag_info.is_null() {
        abort_invalid_dispatch();
    }
    // SAFETY: The caller supplies one readable tag-info structure.
    let tag_info = unsafe { &*tag_info };
    let mut native_info = *tag_info;
    // SAFETY: `objectType` identifies the encoded handle kind.
    native_info.objectHandle = unsafe {
        translate_debug_utils_object(
            loader_device,
            native_info.objectType,
            native_info.objectHandle,
        )
    };
    // SAFETY: Resolver and native device belong to the same ICD.
    let native: Option<PFN_vkSetDebugUtilsObjectTagEXT> =
        unsafe { load_typed(loader_device.resolve(c"vkSetDebugUtilsObjectTagEXT")) };
    native.map_or(VkResult::SUCCESS, |native| {
        // SAFETY: The translated structure remains live for the native call.
        unsafe { native(loader_device.icd_device, &raw const native_info) }
    })
}

/// Dispatches debug-marker tag data through layers before the loader terminator.
///
/// # Safety
///
/// Arguments must satisfy `vkDebugMarkerSetObjectTagEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn vkDebugMarkerSetObjectTagEXT(
    device: VkDevice,
    tag_info: *const VkDebugMarkerObjectTagInfoEXT<'_>,
) -> VkResult {
    // SAFETY: Validation and fatal behavior match the loader trampoline.
    let loader_device = unsafe { checked_device(device, c"vkDebugMarkerSetObjectTagEXT") };
    if loader_device.instance().layers.is_empty() {
        // SAFETY: Forwarded from this function's contract.
        return unsafe { terminator_vkDebugMarkerSetObjectTagEXT(device, tag_info) };
    }
    // SAFETY: The active layer chain created this device.
    let layer: Option<PFN_vkDebugMarkerSetObjectTagEXT> =
        unsafe { load_typed(loader_device.resolve_chain(c"vkDebugMarkerSetObjectTagEXT")) };
    layer.map_or(VkResult::SUCCESS, |layer| {
        if tag_info.is_null() {
            return unsafe { layer(loader_device.chain_device, tag_info) };
        }
        let mut layer_info = unsafe { tag_info.read() };
        layer_info.object = unsafe {
            translate_debug_report_object_chain(
                loader_device,
                layer_info.objectType,
                layer_info.object,
            )
        };
        unsafe { layer(loader_device.chain_device, &raw const layer_info) }
    })
}

/// Dispatches debug-marker object names through layers before the terminator.
///
/// # Safety
///
/// Arguments must satisfy `vkDebugMarkerSetObjectNameEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn vkDebugMarkerSetObjectNameEXT(
    device: VkDevice,
    name_info: *const VkDebugMarkerObjectNameInfoEXT<'_>,
) -> VkResult {
    // SAFETY: Validation and fatal behavior match the loader trampoline.
    let loader_device = unsafe { checked_device(device, c"vkDebugMarkerSetObjectNameEXT") };
    if loader_device.instance().layers.is_empty() {
        // SAFETY: Forwarded from this function's contract.
        return unsafe { terminator_vkDebugMarkerSetObjectNameEXT(device, name_info) };
    }
    // SAFETY: The active layer chain created this device.
    let layer: Option<PFN_vkDebugMarkerSetObjectNameEXT> =
        unsafe { load_typed(loader_device.resolve_chain(c"vkDebugMarkerSetObjectNameEXT")) };
    layer.map_or(VkResult::SUCCESS, |layer| {
        if name_info.is_null() {
            return unsafe { layer(loader_device.chain_device, name_info) };
        }
        let mut layer_info = unsafe { name_info.read() };
        layer_info.object = unsafe {
            translate_debug_report_object_chain(
                loader_device,
                layer_info.objectType,
                layer_info.object,
            )
        };
        unsafe { layer(loader_device.chain_device, &raw const layer_info) }
    })
}

/// Dispatches debug-utils object names through layers before the terminator.
///
/// # Safety
///
/// Arguments must satisfy `vkSetDebugUtilsObjectNameEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn vkSetDebugUtilsObjectNameEXT(
    device: VkDevice,
    name_info: *const VkDebugUtilsObjectNameInfoEXT<'_>,
) -> VkResult {
    // SAFETY: Validation and fatal behavior match the loader trampoline.
    let loader_device = unsafe { checked_device(device, c"vkSetDebugUtilsObjectNameEXT") };
    if loader_device.instance().layers.is_empty() {
        // SAFETY: Forwarded from this function's contract.
        return unsafe { terminator_vkSetDebugUtilsObjectNameEXT(device, name_info) };
    }
    // SAFETY: The active layer chain created this device.
    let layer: Option<PFN_vkSetDebugUtilsObjectNameEXT> =
        unsafe { load_typed(loader_device.resolve_chain(c"vkSetDebugUtilsObjectNameEXT")) };
    layer.map_or(VkResult::SUCCESS, |layer| {
        if name_info.is_null() {
            return unsafe { layer(loader_device.chain_device, name_info) };
        }
        let mut layer_info = unsafe { name_info.read() };
        layer_info.objectHandle = unsafe {
            translate_debug_utils_object_chain(
                loader_device,
                layer_info.objectType,
                layer_info.objectHandle,
            )
        };
        unsafe { layer(loader_device.chain_device, &raw const layer_info) }
    })
}

/// Dispatches debug-utils tag data through layers before the terminator.
///
/// # Safety
///
/// Arguments must satisfy `vkSetDebugUtilsObjectTagEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn vkSetDebugUtilsObjectTagEXT(
    device: VkDevice,
    tag_info: *const VkDebugUtilsObjectTagInfoEXT<'_>,
) -> VkResult {
    // SAFETY: Validation and fatal behavior match the loader trampoline.
    let loader_device = unsafe { checked_device(device, c"vkSetDebugUtilsObjectTagEXT") };
    if loader_device.instance().layers.is_empty() {
        // SAFETY: Forwarded from this function's contract.
        return unsafe { terminator_vkSetDebugUtilsObjectTagEXT(device, tag_info) };
    }
    // SAFETY: The active layer chain created this device.
    let layer: Option<PFN_vkSetDebugUtilsObjectTagEXT> =
        unsafe { load_typed(loader_device.resolve_chain(c"vkSetDebugUtilsObjectTagEXT")) };
    layer.map_or(VkResult::SUCCESS, |layer| {
        if tag_info.is_null() {
            return unsafe { layer(loader_device.chain_device, tag_info) };
        }
        let mut layer_info = unsafe { tag_info.read() };
        layer_info.objectHandle = unsafe {
            translate_debug_utils_object_chain(
                loader_device,
                layer_info.objectType,
                layer_info.objectHandle,
            )
        };
        unsafe { layer(loader_device.chain_device, &raw const layer_info) }
    })
}
