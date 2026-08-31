//! Vulkan loader implementation.
#![allow(non_snake_case)]

#[cfg(all(feature = "apple-static-loader", not(target_vendor = "apple")))]
compile_error!("the `apple-static-loader` feature is only supported on Apple platforms");

extern crate alloc;

mod allocation;
mod collections;
mod debug;
mod debug_messenger;
mod device;
mod discovery;
mod display;
#[path = "generated/mod.rs"]
mod generated;
mod icd;
mod instance;
mod layer;
mod pending;
mod platform;
mod pre_instance;
mod promoted;
mod surface;
mod sync;
mod unknown;

use alloc::borrow::Cow;
use core::{
    cmp::Ordering,
    ffi::{CStr, c_char, c_void},
    mem::MaybeUninit,
};
use debug::{
    vkDebugMarkerSetObjectNameEXT, vkDebugMarkerSetObjectTagEXT, vkSetDebugUtilsObjectNameEXT,
    vkSetDebugUtilsObjectTagEXT,
};
use debug_messenger::{
    vkCreateDebugReportCallbackEXT, vkCreateDebugUtilsMessengerEXT, vkDebugReportMessageEXT,
    vkDestroyDebugReportCallbackEXT, vkDestroyDebugUtilsMessengerEXT, vkSubmitDebugUtilsMessageEXT,
};
use device::{LoaderDevice, maintenance5_version_checks, validate_and_filter_device_extensions};
use display::{
    terminator_vkGetDisplayModeProperties2KHR, terminator_vkGetDisplayPlaneCapabilities2KHR,
    terminator_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
    terminator_vkGetPhysicalDeviceDisplayProperties2KHR,
};
#[cfg(test)]
use generated::{COMMAND_COUNT, COMMAND_MAX_DISPLACEMENT, COMMAND_NAMES, COMMAND_TABLE};
use generated::{
    ExtensionSet, IcdDeviceTerminatorDispatchTable, InstanceDispatchTable,
    LayerDeviceDispatchTable, LayerInstanceDispatchTable, VK_EXT_SURFACE_MAINTENANCE1_EXTENSION_ID,
    VK_KHR_SURFACE_MAINTENANCE1_EXTENSION_ID, command_core_level,
    command_has_device_extension_provider, command_has_enabled_device_extension,
    command_has_enabled_instance_extension, command_lookup, command_must_use_loader_trampoline,
    convert_core_object_to_debug_report_object, convert_debug_report_object_to_core_object,
    exported_proc_addr, extension_id, global_proc_addr, icd_device_terminator_proc_addr,
    instance_terminator_proc_addr, is_known_instance_extension, layer_device_dispatch_proc_addr,
    physical_device_terminator_proc_addr, surface_create_info_extension_size,
    wsi_instance_extension_supported,
};
use icd::{DirectIcdError, IcdInstance, ManifestApiVersionStatus, ScannedIcd, ScannedIcdLoadError};
use instance::{LoaderInstance, LoaderPhysicalDevice, LoaderPhysicalDeviceTrampoline};
use promoted::{
    terminator_vkGetPhysicalDeviceToolProperties, terminator_vkGetPhysicalDeviceToolPropertiesEXT,
};
use surface::{
    create_loader_surface, destroy_all_surfaces, destroy_icd_surfaces,
    terminator_vkDestroySurfaceKHR, terminator_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
    terminator_vkGetPhysicalDeviceSurfaceCapabilities2KHR,
    terminator_vkGetPhysicalDeviceSurfaceFormats2KHR,
    terminator_vkGetPhysicalDeviceSurfaceSupportKHR, translate_physical_device_surface,
    vkCreateSharedSwapchainsKHR, vkCreateSwapchainKHR, vkDestroySurfaceKHR,
    vkGetDeviceGroupSurfacePresentModesKHR,
};
use vk::{
    PFN_vkCreateDevice, PFN_vkDestroyDevice, PFN_vkDestroyInstance, PFN_vkEnumeratePhysicalDevices,
    PFN_vkGetDeviceProcAddr, PFN_vkVoidFunction, VK_API_VERSION_1_0, VK_API_VERSION_1_3,
    VkAllocationCallbacks, VkDevice, VkDeviceCreateInfo, VkDeviceGroupDeviceCreateInfo,
    VkDirectDriverLoadingInfoLUNARG, VkDirectDriverLoadingListLUNARG,
    VkDirectDriverLoadingModeLUNARG, VkExtensionProperties, VkInstance, VkInstanceCreateInfo,
    VkLayerProperties, VkPhysicalDevice, VkPhysicalDeviceGroupProperties,
    VkPhysicalDeviceGroupPropertiesKHR, VkResult, VkStructureType,
};

pub(crate) const DEVICE_DISPATCH_MAGIC: u64 = 0x10AD_ED04_0410_ADED;

struct DeviceGroupChainPatch<'a> {
    _group: Box<VkDeviceGroupDeviceCreateInfo<'a>>,
    _physical_devices: Box<[VkPhysicalDevice]>,
    restore: Option<(*mut *const c_void, *const c_void)>,
}

impl Drop for DeviceGroupChainPatch<'_> {
    fn drop(&mut self) {
        if let Some((field, original)) = self.restore {
            // SAFETY: The patched predecessor belongs to the synchronous,
            // caller-owned create chain and remains live until this guard drops.
            unsafe { field.write(original) };
        }
    }
}

unsafe fn translate_device_group_chain<'a>(
    create_info: &mut VkDeviceCreateInfo<'a>,
    mut translate: impl FnMut(VkPhysicalDevice) -> Option<VkPhysicalDevice>,
) -> Result<Option<DeviceGroupChainPatch<'a>>, VkResult> {
    let root_next = &raw mut create_info.pNext;
    let mut predecessor = root_next;
    let mut current = create_info.pNext.cast::<vk::VkBaseInStructure<'a>>();
    while !current.is_null() {
        // SAFETY: The Vulkan input-chain contract makes the common header readable.
        let header = unsafe { &*current };
        if header.sType == VkStructureType::DEVICE_GROUP_DEVICE_CREATE_INFO {
            // SAFETY: The matching sType identifies this concrete structure.
            let source = unsafe { &*current.cast::<VkDeviceGroupDeviceCreateInfo<'a>>() };
            if source.physicalDeviceCount == 0 || source.pPhysicalDevices.is_null() {
                return Ok(None);
            }
            let count = source.physicalDeviceCount as usize;
            let mut devices = Vec::new();
            devices
                .try_reserve_exact(count)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            for index in 0..count {
                // SAFETY: The source array contains physicalDeviceCount handles.
                let handle = unsafe { source.pPhysicalDevices.add(index).read() };
                devices.push(translate(handle).ok_or(VkResult::ERROR_INITIALIZATION_FAILED)?);
            }
            let devices = devices.into_boxed_slice();
            let mut group = Box::new(*source);
            group.pPhysicalDevices = devices.as_ptr();
            let replacement = core::ptr::from_ref(group.as_ref()).cast::<c_void>();
            let original = current.cast::<c_void>();
            // SAFETY: predecessor is either the local root pNext field or the
            // writable pNext field of a live chain node, matching upstream.
            unsafe { predecessor.write(replacement) };
            let restore = (predecessor != root_next).then_some((predecessor, original));
            return Ok(Some(DeviceGroupChainPatch {
                _group: group,
                _physical_devices: devices,
                restore,
            }));
        }
        predecessor = unsafe { core::ptr::addr_of!((*current).pNext) }
            .cast::<*const c_void>()
            .cast_mut();
        current = header.pNext;
    }
    Ok(None)
}

union FunctionPointer<T: Copy> {
    erased: unsafe extern "system" fn(),
    typed: T,
}

fn erase_function<T: Copy>(typed: T) -> unsafe extern "system" fn() {
    // SAFETY: Vulkan function pointers use a common representation.
    unsafe { FunctionPointer { typed }.erased }
}

unsafe fn load_typed<T: Copy>(function: PFN_vkVoidFunction) -> Option<T> {
    // SAFETY: Vulkan requires compatible representations for all command
    // pointers returned by its proc-address functions.
    function.map(|erased| unsafe { FunctionPointer { erased }.typed })
}

#[inline]
unsafe fn instance_dispatch<'a>(
    dispatchable: *mut c_void,
) -> Option<&'a LayerInstanceDispatchTable> {
    if dispatchable.is_null() {
        return None;
    }
    // SAFETY: Every live Vulkan instance-scope dispatchable begins with its
    // loader dispatch-table pointer.
    let dispatch = unsafe {
        dispatchable
            .cast::<*const LayerInstanceDispatchTable>()
            .read()
    };
    // SAFETY: The dispatch allocation is retained by the owning instance.
    unsafe { dispatch.as_ref() }
}

#[inline]
unsafe fn resolve_physical_device<T: Copy>(
    physical_device: VkPhysicalDevice,
    resolve: impl FnOnce(&InstanceDispatchTable) -> Option<T>,
    name: &CStr,
) -> Option<(T, VkPhysicalDevice)> {
    // SAFETY: The caller supplies a live loader physical-device handle.
    let physical_device = unsafe { LoaderPhysicalDevice::from_handle(physical_device) }?;
    let icd = physical_device.icd();
    let command = resolve(&icd.dispatch);
    let Some(command) = command else {
        if let Ok(message) = alloc::ffi::CString::new(format!(
            "ICD for selected physical device does not export {}!",
            name.to_string_lossy()
        )) {
            physical_device.instance().submit_loader_message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
                &message,
            );
        }
        return None;
    };
    Some((command, physical_device.native))
}

unsafe fn resolve_trampoline_physical_device(
    physical_device: VkPhysicalDevice,
) -> Option<(&'static LayerInstanceDispatchTable, VkPhysicalDevice)> {
    let trampoline = unsafe { LoaderPhysicalDeviceTrampoline::from_handle(physical_device) }?;
    let dispatch = unsafe { instance_dispatch(physical_device.0.cast()) }?;
    Some((dispatch, trampoline.chain))
}

#[inline]
unsafe fn device_dispatch(handle: *mut c_void) -> Option<&'static LayerDeviceDispatchTable> {
    if handle.is_null() {
        return None;
    }
    // SAFETY: Every live Vulkan device dispatchable stores a dispatch-table
    // pointer in its first machine word.
    let dispatch = unsafe { handle.cast::<*const LayerDeviceDispatchTable>().read() };
    if dispatch.is_null() {
        return None;
    }
    // SAFETY: A live dispatchable's first word points to readable dispatch data.
    (unsafe { (*dispatch).magic } == DEVICE_DISPATCH_MAGIC).then(|| unsafe { &*dispatch })
}

#[cold]
#[inline(never)]
fn invalid_device_dispatch() -> ! {
    // SAFETY: A corrupted dispatchable handle is an unrecoverable loader ABI
    // violation and upstream terminates the process on this path.
    unsafe { libc::abort() }
}

#[cold]
#[inline(never)]
fn fatal_loader_error(message: &CStr) -> ! {
    crate::platform::write_stderr(&format!("{}\n", message.to_string_lossy()));
    // SAFETY: A missing required driver entry point is fatal by loader ABI
    // contract; no Rust cleanup can make continued execution valid.
    unsafe { libc::abort() }
}

unsafe fn set_device_dispatchable(object: *mut c_void, dispatch: *const LayerDeviceDispatchTable) {
    if !object.is_null() {
        // SAFETY: The caller passes a live writable dispatchable object.
        unsafe {
            object
                .cast::<*const LayerDeviceDispatchTable>()
                .write(dispatch);
        };
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CommandScope {
    Global,
    Instance,
    Device,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CommandLookup {
    pub(crate) id: u16,
    pub(crate) scope: CommandScope,
}

#[derive(Clone, Copy)]
pub(crate) struct CommandRecord {
    pub(crate) name_offset: u16,
    pub(crate) id: u16,
    pub(crate) name_len: u8,
    pub(crate) scope: CommandScope,
}

#[derive(Clone, Copy)]
pub(crate) struct CommandProviderRange {
    pub(crate) offset: u16,
    pub(crate) len: u8,
}

const fn command_hash(name: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    let mut index = 0;
    while index < name.len() {
        hash = (hash ^ name[index] as u64).wrapping_mul(0x0100_0000_01b3);
        index += 1;
    }
    hash
}

const fn command_slot_hash(mut hash: u64) -> u64 {
    hash ^= hash >> 30;
    hash = hash.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    hash ^= hash >> 27;
    hash = hash.wrapping_mul(0x94d0_49bb_1331_11eb);
    hash ^ (hash >> 31)
}

const fn dispatch_offset(value: usize) -> u16 {
    assert!(value <= 65_535);
    value as u16
}

#[inline]
fn command_name_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if left[index] != right[index] {
            return false;
        }
        index += 1;
    }
    true
}

/// Creates a Vulkan instance across the discovered ICDs.
///
/// # Safety
///
/// All pointers must satisfy the Vulkan API contract.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkCreateInstance(
    create_info: *const VkInstanceCreateInfo<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    instance: *mut VkInstance,
) -> VkResult {
    platform::initialize_loader();
    if create_info.is_null() || instance.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let _loader_guard = platform::lock_loader();
    // SAFETY: Pointer validity is required by the Vulkan entry-point contract.
    let create_info_ref = unsafe { &*create_info };
    // SAFETY: Application strings and name arrays follow the instance-create contract.
    unsafe { log_instance_create_info(create_info_ref) };
    if !create_info_ref.pApplicationInfo.is_null() {
        let requested = unsafe { (*create_info_ref.pApplicationInfo).apiVersion };
        if requested != 0 && requested < VK_API_VERSION_1_0 {
            unsafe {
                emit_driver_create_message(
                    create_info_ref,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    format!(
                        "VkInstanceCreateInfo::pApplicationInfo::apiVersion has value of {requested} which is not permitted. If apiVersion is not 0, then it must be greater than or equal to the value of VK_API_VERSION_1_0 [VUID-VkApplicationInfo-apiVersion]"
                    ),
                );
            };
        }
    }
    let api_version = if create_info_ref.pApplicationInfo.is_null() {
        VK_API_VERSION_1_0
    } else {
        // SAFETY: A non-null application-info pointer must be readable.
        unsafe { (*create_info_ref.pApplicationInfo).apiVersion }.max(VK_API_VERSION_1_0)
    };

    let settings = discovery::loader_settings();
    if let Some(settings) = &settings {
        let display_path = settings
            .settings_file_path()
            .to_string_lossy()
            .replace("/vulkan/loader_settings.d", "/vulkan//loader_settings.d");
        if let Ok(message) = alloc::ffi::CString::new(format!(
            "Using layer configurations found in loader settings from {display_path}"
        )) {
            // SAFETY: The caller retains the complete instance-create pNext chain.
            unsafe {
                debug_messenger::submit_instance_create_message(
                    create_info_ref,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    &message,
                );
            };
        }
    } else {
        unsafe {
            emit_driver_create_message(
                create_info_ref,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                "No valid vk_loader_settings.json file found, no loader settings will be active",
            );
        };
    }
    let selected_layers = match layer::select_active_layers(create_info_ref, settings.as_ref()) {
        Ok(layers) => layers,
        Err(result) => return result,
    };

    // Upstream discovers and opens the ICDs before validating requested
    // extensions, but does not create native instances until the terminator.
    let scanned_icds = match unsafe { scan_icds(create_info_ref, settings.as_ref()) } {
        Ok(icds) => icds,
        Err(result) => return result,
    };

    if let Err(result) =
        unsafe { validate_instance_extensions(create_info_ref, &selected_layers, &scanned_icds) }
    {
        return result;
    }

    let active_layers = match layer::load_selected_layers(create_info_ref, selected_layers) {
        Ok(layers) => layers,
        Err(result) => return result,
    };

    layer::emit_instance_layer_callstack(create_info_ref, &active_layers.loaded);

    // SAFETY: Instance extension names satisfy the create-info string-array contract.
    let enabled_extensions = unsafe {
        ExtensionSet::from_names(
            create_info_ref.enabledExtensionCount,
            create_info_ref.ppEnabledExtensionNames,
        )
    };
    let device_configurations =
        settings.and_then(discovery::LoaderSettings::into_device_configurations);
    let mut loader_instance = LoaderInstance::new(
        api_version,
        enabled_extensions,
        scanned_icds,
        active_layers,
        device_configurations,
        allocator,
    );
    let variant = vk::VK_API_VERSION_VARIANT(api_version);
    if variant != 0
        && let Ok(message) = alloc::ffi::CString::new(format!(
            "vkCreateInstance: The API Variant specified in pCreateInfo->pApplicationInfo.apiVersion is {variant} instead of the expected value of 0."
        ))
    {
        // SAFETY: The caller's complete create-info chain remains live.
        unsafe { debug_messenger::submit_instance_create_warning(create_info_ref, &message) };
    }
    if loader_instance.layers.is_empty() {
        let previous = pending::replace_instance(loader_instance.handle());
        // SAFETY: The public entrypoint validated the output and create-info
        // pointers, and the pending handle identifies this unregistered box.
        unsafe { instance.write(loader_instance.handle()) };
        let result = unsafe { layer::create_instance_terminator(create_info, allocator, instance) };
        pending::replace_instance(previous);
        if result != VkResult::SUCCESS {
            unsafe { instance.write(VkInstance::NULL) };
            return result;
        }
        LoaderInstance::register(loader_instance);
        return VkResult::SUCCESS;
    }
    // SAFETY: The loaded layer interfaces and caller-owned create structures
    // remain live for the duration of the synchronous chain call.
    let result = unsafe {
        layer::create_instance_chain(&mut loader_instance, create_info_ref, allocator, instance)
    };
    if result != VkResult::SUCCESS {
        // The layer ABI receives a preinitialized loader object, but a failed
        // create must not leak that internal handle back to the application.
        unsafe { instance.write(VkInstance::NULL) };
        destroy_icd_instances(&loader_instance.icds, allocator);
        return result;
    }
    LoaderInstance::register(loader_instance);
    VkResult::SUCCESS
}

#[cold]
#[inline(never)]
unsafe fn log_instance_create_info(create_info: &VkInstanceCreateInfo<'_>) {
    let application = unsafe { create_info.pApplicationInfo.as_ref() };
    let application_name = application.map_or(Cow::Borrowed(""), |info| {
        if info.pApplicationName.is_null() {
            Cow::Borrowed("")
        } else {
            unsafe { CStr::from_ptr(info.pApplicationName) }.to_string_lossy()
        }
    });
    let engine_name = application.map_or(Cow::Borrowed(""), |info| {
        if info.pEngineName.is_null() {
            Cow::Borrowed("")
        } else {
            unsafe { CStr::from_ptr(info.pEngineName) }.to_string_lossy()
        }
    });
    let application_version = application.map_or(0, |info| info.applicationVersion);
    let engine_version = application.map_or(0, |info| info.engineVersion);
    let api_version = application.map_or(0, |info| info.apiVersion);
    unsafe {
        emit_driver_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format!(
                "vkCreateInstance: applicationName: \"{application_name}\", applicationVersion: {application_version}, engineName: \"{engine_name}\", engineVersion: {engine_version}, apiVersion: {}.{}.{}",
                vk::VK_API_VERSION_MAJOR(api_version),
                vk::VK_API_VERSION_MINOR(api_version),
                vk::VK_API_VERSION_PATCH(api_version),
            ),
        );
    };
    unsafe {
        log_instance_name_array(
            create_info,
            "layer",
            create_info.enabledLayerCount,
            create_info.ppEnabledLayerNames,
        );
        log_instance_name_array(
            create_info,
            "extension",
            create_info.enabledExtensionCount,
            create_info.ppEnabledExtensionNames,
        );
    }
}

#[cold]
unsafe fn log_instance_name_array(
    create_info: &VkInstanceCreateInfo<'_>,
    kind: &str,
    count: u32,
    names: *const *const core::ffi::c_char,
) {
    unsafe {
        emit_driver_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format!("vkCreateInstance: Requested {count} instance {kind}(s):"),
        );
    };
    if names.is_null() {
        return;
    }
    for index in 0..count as usize {
        let name = unsafe { names.add(index).read() };
        let value = if name.is_null() {
            "<NULL>".into()
        } else {
            unsafe { CStr::from_ptr(name) }.to_string_lossy()
        };
        unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format!("   {value}"),
            );
        };
    }
}

#[cold]
#[inline(never)]
unsafe fn validate_instance_extensions(
    create_info: &VkInstanceCreateInfo<'_>,
    layers: &layer::SelectedLayers,
    scanned_icds: &[ScannedIcdRecord],
) -> Result<(), VkResult> {
    if create_info.enabledExtensionCount == 0 {
        return Ok(());
    }
    if create_info.ppEnabledExtensionNames.is_null() {
        return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
    }
    let mut available = Vec::new();
    for scanned in scanned_icds {
        let properties = unsafe { scanned_icd_instance_extensions(&scanned.icd) }?;
        available
            .try_reserve_exact(properties.len())
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        available.extend(properties);
    }
    let filter_unknown = !std::env::var_os("VK_LOADER_DISABLE_INST_EXT_FILTER")
        .is_some_and(|value| decimal_prefix_nonzero(value.as_encoded_bytes()));
    for index in 0..create_info.enabledExtensionCount as usize {
        let name = unsafe { create_info.ppEnabledExtensionNames.add(index).read() };
        if name.is_null() {
            return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
        }
        let name = unsafe { CStr::from_ptr(name) };
        if filter_unknown && !is_known_instance_extension(name) {
            return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
        }
        let loader_available = loader_instance_extension_supported(name);
        let globally_available = wsi_instance_extension_supported(name)
            && available.iter().any(|property| {
                // SAFETY: Loader-constructed extension properties are NUL-terminated.
                unsafe { CStr::from_ptr(property.extensionName.as_ptr()) == name }
            });
        let layer_available = layers.supports_instance_extension(name);
        if !loader_available && !globally_available && !layer_available {
            return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
        }
    }
    Ok(())
}

fn loader_instance_extension_supported(name: &CStr) -> bool {
    name.to_bytes() == vk::VK_EXT_DEBUG_REPORT_EXTENSION_NAME.to_bytes()
        || name.to_bytes() == vk::VK_EXT_DEBUG_UTILS_EXTENSION_NAME.to_bytes()
        || name.to_bytes() == vk::VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME.to_bytes()
        || name.to_bytes() == vk::VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_NAME.to_bytes()
}

#[cold]
#[inline(never)]
unsafe fn emit_driver_create_message(
    create_info: &VkInstanceCreateInfo<'_>,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: impl AsRef<str>,
) {
    let message = message.as_ref();
    let (filter, label) = if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR {
        ("error", "ERROR")
    } else if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING {
        ("warn", "WARNING")
    } else if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO {
        ("info", "INFO")
    } else {
        ("debug", "DEBUG")
    };
    platform::write_loader_log(filter, label, format_args!("{message}"));
    let Ok(message) = alloc::ffi::CString::new(message) else {
        return;
    };
    // SAFETY: The caller retains the complete instance-create pNext chain.
    unsafe { debug_messenger::submit_instance_create_message(create_info, severity, &message) };
}

#[cold]
#[inline(never)]
unsafe fn emit_driver_category_create_message(
    create_info: &VkInstanceCreateInfo<'_>,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: impl AsRef<str>,
) {
    let message = message.as_ref();
    let (filter, label) = if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR {
        ("error", "ERROR")
    } else if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING {
        ("warn", "WARNING")
    } else if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO {
        ("info", "INFO")
    } else {
        ("debug", "DEBUG")
    };
    platform::write_loader_log_with_category(
        filter,
        label,
        "driver",
        "DRIVER",
        format_args!("{message}"),
    );
    let Ok(message) = alloc::ffi::CString::new(message) else {
        return;
    };
    // SAFETY: The caller retains the complete instance-create pNext chain.
    unsafe { debug_messenger::submit_instance_create_message(create_info, severity, &message) };
}

#[cold]
unsafe fn emit_driver_only_create_message(
    create_info: &VkInstanceCreateInfo<'_>,
    message: impl AsRef<str>,
) {
    let message = message.as_ref();
    platform::write_loader_category_log("driver", "DRIVER", format_args!("{message}"));
    let Ok(message) = alloc::ffi::CString::new(message) else {
        return;
    };
    // Category-only upstream messages are informational debug-utils messages.
    unsafe {
        debug_messenger::submit_instance_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            &message,
        );
    };
}

unsafe fn direct_driver_extension_enabled(create_info: &VkInstanceCreateInfo<'_>) -> bool {
    unsafe {
        instance_extension_enabled(
            create_info,
            vk::VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_NAME,
        )
    }
}

unsafe fn instance_extension_enabled(
    create_info: &VkInstanceCreateInfo<'_>,
    extension: &CStr,
) -> bool {
    if create_info.enabledExtensionCount == 0 || create_info.ppEnabledExtensionNames.is_null() {
        return false;
    }
    for index in 0..create_info.enabledExtensionCount as usize {
        // SAFETY: Vulkan requires this many readable extension-name pointers.
        let name = unsafe { create_info.ppEnabledExtensionNames.add(index).read() };
        if !name.is_null()
            // SAFETY: Enabled extension names are NUL-terminated by contract.
            && unsafe { CStr::from_ptr(name) } == extension
        {
            return true;
        }
    }
    false
}

unsafe fn direct_driver_list<'a>(
    create_info: &VkInstanceCreateInfo<'a>,
) -> Option<&'a VkDirectDriverLoadingListLUNARG<'a>> {
    let mut next = create_info.pNext.cast::<vk::VkBaseInStructure<'a>>();
    while !next.is_null() {
        // SAFETY: The instance-create pNext chain is readable by contract.
        let structure = unsafe { &*next };
        if structure.sType == VkStructureType::DIRECT_DRIVER_LOADING_LIST_LUNARG {
            // SAFETY: sType identifies the concrete structure layout.
            return Some(unsafe { &*next.cast::<VkDirectDriverLoadingListLUNARG<'a>>() });
        }
        next = structure.pNext;
    }
    None
}

fn fatal_direct_driver_scan_error(result: VkResult) -> Option<VkResult> {
    (result == VkResult::ERROR_OUT_OF_HOST_MEMORY).then_some(result)
}

#[cold]
#[inline(never)]
unsafe fn scan_direct_drivers(
    create_info: &VkInstanceCreateInfo<'_>,
) -> Result<(bool, Vec<ScannedIcd>), VkResult> {
    let enabled = unsafe { direct_driver_extension_enabled(create_info) };
    let list = unsafe { direct_driver_list(create_info) };
    let Some(list) = list else {
        if enabled {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    "loader_scan_for_direct_drivers: The VK_LUNARG_direct_driver_loading extension was enabled but the pNext chain of VkInstanceCreateInfo did not contain the VkDirectDriverLoadingListLUNARG structure.",
                );
            };
        }
        return Ok((false, Vec::new()));
    };
    if !enabled {
        unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                "loader_scan_for_direct_drivers: The pNext chain of VkInstanceCreateInfo contained the VkDirectDriverLoadingListLUNARG structure, but the VK_LUNARG_direct_driver_loading extension was not enabled.",
            );
        };
        return Ok((false, Vec::new()));
    }

    let exclusive = list.mode == VkDirectDriverLoadingModeLUNARG::EXCLUSIVE;
    if exclusive {
        unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                "loader_scan_for_direct_drivers: The VK_LUNARG_direct_driver_loading extension is active and specified VK_DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG, skipping system and environment variable driver search mechanisms.",
            );
        };
    }
    if list.pDrivers.is_null() {
        unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                "loader_scan_for_direct_drivers: The VkDirectDriverLoadingListLUNARG structure in the pNext chain of VkInstanceCreateInfo has a NULL pDrivers member.",
            );
        };
        return Ok((exclusive, Vec::new()));
    }
    if list.driverCount == 0 {
        unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                "loader_scan_for_direct_drivers: The VkDirectDriverLoadingListLUNARG structure in the pNext chain of VkInstanceCreateInfo has a non-null pDrivers member but a driverCount member with a value of zero.",
            );
        };
        return Ok((exclusive, Vec::new()));
    }

    let count = list.driverCount as usize;
    let mut drivers = Vec::new();
    drivers
        .try_reserve_exact(count)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for index in 0..count {
        // SAFETY: `pDrivers` names an array of `driverCount` structures.
        let info: &VkDirectDriverLoadingInfoLUNARG<'_> = unsafe { &*list.pDrivers.add(index) };
        let Some(gipa) = info.pfnGetInstanceProcAddr else {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    format!(
                        "loader_add_direct_driver: VkDirectDriverLoadingInfoLUNARG structure at index {index} contains a NULL pointer for the pfnGetInstanceProcAddr member, skipping."
                    ),
                );
            };
            continue;
        };
        // SAFETY: The extension requires this callback and its returned
        // functions to remain live through instance destruction.
        match unsafe { ScannedIcd::load_direct(gipa) } {
            Ok(driver) => {
                unsafe {
                    emit_driver_category_create_message(
                        create_info,
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                        format!(
                            "loader_add_direct_driver: Adding driver found in index {index} of VkDirectDriverLoadingListLUNARG::pDrivers structure. pfnGetInstanceProcAddr was set to {:p}",
                            gipa as *const ()
                        ),
                    );
                };
                drivers.push(driver);
            }
            Err(DirectIcdError::EnumerateVersion(result)) => {
                if let Some(result) = fatal_direct_driver_scan_error(result) {
                    return Err(result);
                }
                // Upstream treats every other per-driver failure as a reason
                // to skip this entry and continue scanning the remaining
                // direct drivers.
            }
            Err(error) => {
                let message = match error {
                    DirectIcdError::MissingNegotiate => format!(
                        "loader_add_direct_driver: Could not get 'vk_icdNegotiateLoaderICDInterfaceVersion' from VkDirectDriverLoadingInfoLUNARG structure at index {index}, skipping."
                    ),
                    DirectIcdError::IncompatibleInterface(version) => format!(
                        "loader_add_direct_driver: VkDirectDriverLoadingInfoLUNARG structure at index {index} supports interface version {version}, which is incompatible with the Loader Driver Interface version that supports the VK_LUNARG_direct_driver_loading extension, skipping."
                    ),
                    DirectIcdError::MissingCreateInstance => format!(
                        "loader_add_direct_driver: Could not get 'vkCreateInstance' from VkDirectDriverLoadingInfoLUNARG structure at index {index}, skipping."
                    ),
                    DirectIcdError::MissingEnumerateExtensions => format!(
                        "loader_add_direct_driver: Could not get 'vkEnumerateInstanceExtensionProperties' from VkDirectDriverLoadingInfoLUNARG structure at index {index}, skipping."
                    ),
                    DirectIcdError::EnumerateVersion(_) => continue,
                };
                unsafe {
                    emit_driver_create_message(
                        create_info,
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                        message,
                    );
                };
            }
        }
    }
    Ok((exclusive, drivers))
}

#[cold]
#[inline(never)]
unsafe fn emit_driver_search_roots(create_info: &VkInstanceCreateInfo<'_>) {
    unsafe { emit_driver_only_create_message(create_info, "Searching for driver manifest files") };
}

#[cold]
#[inline(never)]
unsafe fn emit_driver_scan_diagnostics(
    create_info: &VkInstanceCreateInfo<'_>,
    scan: &discovery::DriverScan,
) {
    unsafe { emit_driver_search_roots(create_info) };
    unsafe { emit_driver_only_create_message(create_info, "   In following locations:") };
    for root in &scan.search_roots {
        unsafe {
            emit_driver_only_create_message(
                create_info,
                format!("      {}", root.to_string_lossy()),
            );
        };
    }
    if scan.candidates.is_empty() {
        unsafe { emit_driver_only_create_message(create_info, "   Found no files") };
    } else {
        unsafe { emit_driver_only_create_message(create_info, "   Found the following files:") };
        for (path, _) in &scan.candidates {
            unsafe {
                emit_driver_only_create_message(
                    create_info,
                    format!("      {}", path.to_string_lossy()),
                );
            };
        }
    }
    #[cfg(windows)]
    if let Some(registry) = &scan.registry_diagnostics {
        for path in &registry.located {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format!(
                        "Located json file \"{}\" from registry \"HKEY_LOCAL_MACHINE\\SOFTWARE\\Khronos\\Vulkan\\Drivers\"",
                        path.to_string_lossy(),
                    ),
                )
            };
        }
        if registry.no_unique_files {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    "Found no registry files in HKEY_LOCAL_MACHINE\\SOFTWARE\\Khronos\\Vulkan\\Drivers",
                )
            };
        }
    }
    for (path, disposition) in &scan.candidates {
        if *disposition != discovery::DriverDisposition::Accepted
            && let Some(manifest) = discovery::parse_manifest(path)
        {
            unsafe { emit_driver_manifest_found(create_info, &manifest) };
        }
        let suffix = match disposition {
            discovery::DriverDisposition::Accepted => None,
            discovery::DriverDisposition::NotSelected => {
                Some(" ignored because not selected by env var")
            }
            discovery::DriverDisposition::Disabled => {
                Some(" ignored because it was disabled by env var")
            }
        };
        if let Some(message) = suffix.map(|suffix| format!("{}{suffix}", path.to_string_lossy())) {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    message,
                );
            };
        }
    }
}

#[cold]
#[inline(never)]
unsafe fn emit_driver_manifest_diagnostics(
    create_info: &VkInstanceCreateInfo<'_>,
    manifests: &[discovery::DriverManifest],
) {
    for manifest in manifests {
        let variant = vk::VK_API_VERSION_VARIANT(manifest.api_version);
        if variant != 0 || !manifest.architecture_supported {
            unsafe { emit_driver_manifest_found(create_info, manifest) };
        }
        if manifest.manifest_version >= vk::VK_MAKE_API_VERSION(0, 1, 0, 2) {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format!(
                        "loader_parse_icd_manifest: {} has unknown icd manifest file version {}.{}.{}. May cause errors.",
                        manifest.manifest_path.to_string_lossy(),
                        vk::VK_API_VERSION_MAJOR(manifest.manifest_version),
                        vk::VK_API_VERSION_MINOR(manifest.manifest_version),
                        vk::VK_API_VERSION_PATCH(manifest.manifest_version),
                    ),
                );
            };
        }
        if variant != 0 {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format!(
                        "loader_parse_icd_manifest: Driver's ICD JSON {} 'api_version' field contains a non-zero variant value of {variant}.  Skipping ICD JSON.",
                        manifest.manifest_path.to_string_lossy(),
                    ),
                );
            };
        }
        if !manifest.architecture_supported {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    "loader_parse_icd_manifest: Driver library architecture doesn't match the current running architecture, skipping this driver",
                );
            };
        }
    }
}

#[cold]
unsafe fn emit_driver_manifest_found(
    create_info: &VkInstanceCreateInfo<'_>,
    manifest: &discovery::DriverManifest,
) {
    unsafe {
        emit_driver_only_create_message(
            create_info,
            format!(
                "Found ICD manifest file {}, version {}.{}.{}",
                manifest.manifest_path.to_string_lossy(),
                vk::VK_API_VERSION_MAJOR(manifest.manifest_version),
                vk::VK_API_VERSION_MINOR(manifest.manifest_version),
                vk::VK_API_VERSION_PATCH(manifest.manifest_version),
            ),
        );
    };
}

#[cold]
#[inline(never)]
unsafe fn scan_icds(
    create_info: &VkInstanceCreateInfo<'_>,
    settings: Option<&discovery::LoaderSettings>,
) -> Result<Box<[ScannedIcdRecord]>, VkResult> {
    let (exclusive, direct_drivers) = unsafe { scan_direct_drivers(create_info) }?;
    let scan = (!exclusive).then(|| discovery::scan_drivers_with_settings(settings));
    if let Some(scan) = &scan {
        unsafe { emit_driver_scan_diagnostics(create_info, scan) };
    }
    let manifests = scan.map_or_else(Box::default, |scan| scan.manifests);
    let portability_flag = create_info
        .flags
        .intersects(vk::VkInstanceCreateFlagBits::ENUMERATE_PORTABILITY_BIT_KHR);
    let portability_extension = unsafe {
        instance_extension_enabled(
            create_info,
            vk::VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME,
        )
    };
    let portability_enabled = portability_flag && portability_extension;
    let mut skipped_portability_drivers = false;
    let mut manifests = manifests.into_vec();
    unsafe { emit_driver_manifest_diagnostics(create_info, &manifests) };
    manifests.retain(|manifest| {
        vk::VK_API_VERSION_VARIANT(manifest.api_version) == 0 && manifest.architecture_supported
    });
    manifests.retain(|manifest| {
        let keep = !manifest.portability_driver || portability_enabled;
        skipped_portability_drivers |= !keep;
        keep
    });
    let mut scanned_icds = Vec::new();
    scanned_icds
        .try_reserve_exact(manifests.len() + direct_drivers.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    // Upstream adds direct drivers before manifest drivers; physical-device
    // enumeration walks the resulting ICD list in reverse insertion order.
    for direct_driver in direct_drivers {
        scanned_icds.push(ScannedIcdRecord {
            icd: direct_driver,
            version_status: ManifestApiVersionStatus::Consistent,
        });
    }
    for manifest in &manifests {
        if let Some(icd) = unsafe { load_scanned_icd(manifest, create_info) } {
            scanned_icds.push(icd);
        }
    }
    if scanned_icds.is_empty() {
        if skipped_portability_drivers {
            let message = match (portability_flag, portability_extension) {
                (true, false) => {
                    "VkInstanceCreateInfo: If flags has the VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR bit set, the list of enabled extensions in ppEnabledExtensionNames must contain VK_KHR_portability_enumeration [VUID-VkInstanceCreateInfo-flags-06559 ]Applications that wish to enumerate portability drivers must enable the VK_KHR_portability_enumeration instance extension."
                }
                (false, true) => {
                    "vkCreateInstance: Found drivers that contain devices which support the portability subset, but the instance does not enumerate portability drivers! Applications that wish to enumerate portability drivers must set the VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR bit in the VkInstanceCreateInfo flags."
                }
                (false, false) => {
                    "vkCreateInstance: Found drivers that contain devices which support the portability subset, but the instance does not enumerate portability drivers! Applications that wish to enumerate portability drivers must set the VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR bit in the VkInstanceCreateInfo flags and enable the VK_KHR_portability_enumeration instance extension."
                }
                (true, true) => unreachable!(),
            };
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    message,
                );
            };
        }
        unsafe {
            debug_messenger::submit_instance_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                c"vkCreateInstance: Found no drivers!",
            );
        };
        Err(VkResult::ERROR_INCOMPATIBLE_DRIVER)
    } else {
        Ok(scanned_icds.into_boxed_slice())
    }
}

pub(crate) struct ScannedIcdRecord {
    icd: ScannedIcd,
    version_status: ManifestApiVersionStatus,
}

#[cold]
#[inline(never)]
unsafe fn load_scanned_icd(
    manifest: &discovery::DriverManifest,
    create_info: &VkInstanceCreateInfo<'_>,
) -> Option<ScannedIcdRecord> {
    unsafe { emit_driver_manifest_found(create_info, manifest) };
    unsafe {
        emit_driver_category_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
            format!(
                "Searching for ICD drivers named {}",
                manifest.library_path.to_string_lossy()
            ),
        );
    };
    let (icd, version_status) = match ScannedIcd::load_manifest(manifest) {
        Ok(loaded) => loaded,
        Err(ScannedIcdLoadError::OpenLibrary(error)) => {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    error,
                );
            };
            return None;
        }
        Err(ScannedIcdLoadError::InvalidInterface) => return None,
    };
    Some(ScannedIcdRecord {
        icd,
        version_status,
    })
}

#[cold]
#[inline(never)]
unsafe fn create_icd_instances(
    scanned_icds: Box<[ScannedIcdRecord]>,
    create_info: &VkInstanceCreateInfo<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    requested_api_version: u32,
    has_device_configurations: bool,
) -> Result<Box<[IcdInstance]>, VkResult> {
    let mut icds = Vec::new();
    icds.try_reserve_exact(scanned_icds.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for scanned in scanned_icds {
        unsafe { emit_icd_version_status(create_info, &scanned) };
        let Some(slot) = icds.spare_capacity_mut().first_mut() else {
            destroy_icd_instances(&icds, allocator);
            return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
        };
        let output = slot.as_mut_ptr();
        match unsafe {
            create_scanned_icd_instance(
                scanned.icd,
                create_info,
                allocator,
                requested_api_version,
                has_device_configurations,
                output,
            )
        } {
            Ok(true) => unsafe { icds.set_len(icds.len() + 1) },
            Err(VkResult::ERROR_OUT_OF_HOST_MEMORY) => {
                destroy_icd_instances(&icds, allocator);
                return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
            }
            Ok(false) | Err(_) => {}
        }
    }
    if icds.is_empty() {
        unsafe {
            debug_messenger::submit_instance_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                c"terminator_CreateInstance: Found no drivers!",
            );
        };
        Err(VkResult::ERROR_INCOMPATIBLE_DRIVER)
    } else {
        Ok(icds.into_boxed_slice())
    }
}

/// Consumes the scanned ICD list at the bottom of an instance layer chain.
///
/// # Safety
///
/// `loader` must be the pending instance for this synchronous create call and
/// `create_info` must be the live, possibly layer-modified Vulkan structure.
pub(crate) unsafe fn create_pending_icd_instances(
    loader: &mut LoaderInstance,
    create_info: &VkInstanceCreateInfo<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
) -> VkResult {
    let Some(scanned_icds) = loader.pending_icds.take() else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let application_info = unsafe { create_info.pApplicationInfo.as_ref() };
    loader.api_version = application_info
        .map_or(VK_API_VERSION_1_0, |info| info.apiVersion)
        .max(VK_API_VERSION_1_0);
    loader.enabled_extensions = unsafe {
        ExtensionSet::from_names(
            create_info.enabledExtensionCount,
            create_info.ppEnabledExtensionNames,
        )
    };
    let has_device_configurations = loader
        .device_configurations
        .as_ref()
        .is_some_and(|configurations| !configurations.is_empty());
    match unsafe {
        create_icd_instances(
            scanned_icds,
            create_info,
            allocator,
            loader.api_version,
            has_device_configurations,
        )
    } {
        Ok(icds) => {
            loader.icds = icds;
            VkResult::SUCCESS
        }
        Err(result) => result,
    }
}

#[cold]
unsafe fn emit_icd_version_status(
    create_info: &VkInstanceCreateInfo<'_>,
    scanned: &ScannedIcdRecord,
) {
    let library_path = scanned
        .icd
        .library_path()
        .map_or(Cow::Borrowed("<direct driver>"), |path| {
            path.to_string_lossy()
        });
    match scanned.version_status {
        ManifestApiVersionStatus::Consistent => {}
        ManifestApiVersionStatus::EnumerateInstanceVersionMissing => unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format!(
                    "terminator_CreateInstance: Manifest ICD for \"{library_path}\" contained a 1.1 or greater API version, but does not support vkEnumerateInstanceVersion, treating as a 1.0 ICD",
                ),
            );
        },
        ManifestApiVersionStatus::EnumerateInstanceVersionReturned(version) => unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format!(
                    "terminator_CreateInstance: Manifest ICD for \"{}\" contained a 1.1 or greater API version, but vkEnumerateInstanceVersion returned {}.{}, treating as a 1.0 ICD",
                    library_path,
                    vk::VK_API_VERSION_MAJOR(version),
                    vk::VK_API_VERSION_MINOR(version),
                ),
            );
        },
    }
}

const fn icd_create_application_api_version(
    requested_api_version: u32,
    driver_api_version: u32,
    has_device_configurations: bool,
) -> Option<u32> {
    if driver_api_version < vk::VK_API_VERSION_1_1 && requested_api_version > vk::VK_API_VERSION_1_0
    {
        Some(driver_api_version)
    } else if has_device_configurations
        && driver_api_version >= vk::VK_API_VERSION_1_1
        && requested_api_version < vk::VK_API_VERSION_1_1
    {
        Some(vk::VK_API_VERSION_1_1)
    } else {
        None
    }
}

#[cold]
unsafe fn create_scanned_icd_instance(
    icd: ScannedIcd,
    create_info: &VkInstanceCreateInfo<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    requested_api_version: u32,
    has_device_configurations: bool,
    output: *mut IcdInstance,
) -> Result<bool, VkResult> {
    let mut icd_create_info = *create_info;
    icd_create_info.enabledLayerCount = 0;
    icd_create_info.ppEnabledLayerNames = core::ptr::null();
    let supported_extensions = unsafe { scanned_icd_instance_extensions(&icd) }?;
    let supports = |name: *const core::ffi::c_char| {
        !name.is_null()
            && supported_extensions.iter().any(|property| unsafe {
                CStr::from_ptr(property.extensionName.as_ptr()) == CStr::from_ptr(name)
            })
    };
    let mut icd_extension_names = Vec::new();
    if create_info.enabledExtensionCount != 0 {
        icd_extension_names
            .try_reserve_exact(create_info.enabledExtensionCount as usize + 1)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        for index in 0..create_info.enabledExtensionCount as usize {
            let name = unsafe { create_info.ppEnabledExtensionNames.add(index).read() };
            if supports(name) {
                icd_extension_names.push(name);
            }
        }
    }
    let needs_sort_properties_extension = LINUX_SORT_PLATFORM_ENABLED
        && linux_sort_requires_properties_extension(requested_api_version, icd.api_version);
    let properties2_name = vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME;
    let properties2_already_enabled = icd_extension_names
        .iter()
        .any(|&name| !name.is_null() && unsafe { CStr::from_ptr(name) == properties2_name });
    if needs_sort_properties_extension
        && !properties2_already_enabled
        && supports(properties2_name.as_ptr())
    {
        icd_extension_names
            .try_reserve(1)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        icd_extension_names.push(properties2_name.as_ptr());
    }
    if !icd_extension_names.is_empty() {
        icd_create_info.enabledExtensionCount = icd_extension_names.len() as u32;
        icd_create_info.ppEnabledExtensionNames = icd_extension_names.as_ptr();
    }
    if icd_create_info
        .flags
        .intersects(vk::VkInstanceCreateFlagBits::ENUMERATE_PORTABILITY_BIT_KHR)
        && !supports(vk::VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME.as_ptr())
    {
        icd_create_info.flags.0 &= !vk::VkInstanceCreateFlagBits::ENUMERATE_PORTABILITY_BIT_KHR.0;
    }
    let enabled_extensions = unsafe {
        ExtensionSet::from_names(
            icd_extension_names.len() as u32,
            icd_extension_names.as_ptr(),
        )
    };
    let mut icd_application_info = if create_info.pApplicationInfo.is_null() {
        vk::VkApplicationInfo::DEFAULT
    } else {
        // SAFETY: A non-null application-info pointer is readable by contract.
        unsafe { *create_info.pApplicationInfo }
    };
    if let Some(api_version) = icd_create_application_api_version(
        requested_api_version,
        icd.api_version,
        has_device_configurations,
    ) {
        icd_application_info.apiVersion = api_version;
        icd_create_info.pApplicationInfo = &raw const icd_application_info;
    }
    let mut handle = VkInstance::NULL;
    // SAFETY: The scanned function has the registry ABI and receives valid structures.
    match unsafe { (icd.create_instance)(&raw const icd_create_info, allocator, &raw mut handle) } {
        VkResult::SUCCESS => {
            let dispatch = unsafe { core::ptr::addr_of_mut!((*output).dispatch) };
            // SAFETY: The reserved vector slot is writable and `handle` was
            // just created by this ICD whose GIPA remains live.
            unsafe {
                InstanceDispatchTable::load_into(dispatch, icd.get_instance_proc_addr, handle);
            };
            // SAFETY: `load_into` initialized the complete dispatch field.
            let dispatch_ref = unsafe { &*dispatch };
            if !dispatch_ref.has_required_core_1_0() {
                let destroy: Option<PFN_vkDestroyInstance> = unsafe {
                    load_typed((icd.get_instance_proc_addr)(
                        handle,
                        c"vkDestroyInstance".as_ptr(),
                    ))
                };
                if let Some(destroy) = destroy {
                    unsafe { destroy(handle, allocator) };
                }
                return Ok(false);
            }
            unsafe {
                core::ptr::addr_of_mut!((*output).icd).write(icd);
                core::ptr::addr_of_mut!((*output).handle).write(handle);
                core::ptr::addr_of_mut!((*output).enabled_extensions).write(enabled_extensions);
                core::ptr::addr_of_mut!((*output).unknown_physical_device_dispatch)
                    .write(unknown::UnknownDispatchTable::new());
                IcdInstance::initialize_active(output);
            }
            Ok(true)
        }
        VkResult::ERROR_OUT_OF_HOST_MEMORY => Err(VkResult::ERROR_OUT_OF_HOST_MEMORY),
        _ => Ok(false),
    }
}

unsafe fn scanned_icd_instance_extensions(
    icd: &ScannedIcd,
) -> Result<Vec<VkExtensionProperties>, VkResult> {
    let Some(enumerate): Option<vk::PFN_vkEnumerateInstanceExtensionProperties> =
        (unsafe { icd.resolve(VkInstance::NULL, c"vkEnumerateInstanceExtensionProperties") })
    else {
        return Ok(Vec::new());
    };
    let mut count = 0;
    let result = unsafe { enumerate(core::ptr::null(), &raw mut count, core::ptr::null_mut()) };
    if result != VkResult::SUCCESS {
        return Err(result);
    }
    let capacity = count as usize;
    let mut properties = Vec::new();
    properties
        .try_reserve_exact(capacity)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    properties.resize(capacity, VkExtensionProperties::DEFAULT);
    let result = unsafe { enumerate(core::ptr::null(), &raw mut count, properties.as_mut_ptr()) };
    if result != VkResult::SUCCESS && result != VkResult::INCOMPLETE {
        return Err(result);
    }
    properties.truncate((count as usize).min(capacity));
    Ok(properties)
}

fn destroy_icd_instances(icds: &[IcdInstance], allocator: *const VkAllocationCallbacks<'_>) {
    for instance in icds {
        if !instance.begin_retire() {
            continue;
        }
        let destroy: Option<PFN_vkDestroyInstance> = instance.dispatch.vkDestroyInstance;
        debug_assert!(destroy.is_some());
        if let Some(destroy) = destroy {
            // SAFETY: Native handle and original allocator belong to this ICD instance.
            unsafe { destroy(instance.handle, allocator) };
        }
        instance.icd.unload_library();
    }
}

/// Destroys a loader instance and every native ICD instance it owns.
///
/// # Safety
///
/// `instance` must be null or a live instance returned by this loader. The
/// allocator must match the allocator supplied at instance creation.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkDestroyInstance(
    instance: VkInstance,
    allocator: *const VkAllocationCallbacks<'_>,
) {
    if instance == VkInstance::NULL {
        return;
    }
    let loader_guard = platform::lock_loader();
    // SAFETY: Validate through the registered dispatch key before dereferencing
    // any table supplied by an untrusted application handle.
    let loader = unsafe { LoaderInstance::from_handle(instance) }.unwrap_or_else(|| {
        fatal_loader_error(
            c"vkDestroyInstance: Invalid instance [VUID-vkDestroyInstance-instance-parameter]",
        )
    });
    let dispatch = unsafe { &*loader.dispatch() };
    debug_assert!(dispatch.vkDestroyInstance.is_some());
    // SAFETY: Core Vulkan 1.0 requires this entry in every conforming chain.
    let destroy = unsafe { dispatch.vkDestroyInstance.unwrap_unchecked() };
    let dispatch_key = core::ptr::from_ref(dispatch);
    // SAFETY: Forward the caller's live chain handle and matching allocator.
    unsafe { destroy(instance, allocator) };
    // The layer libraries must remain loaded until every destroy frame has
    // returned, so ownership is released only after the chain call completes.
    drop(LoaderInstance::take_dispatch(dispatch_key));
    drop(loader_guard);
    // Match upstream's refresh boundary: a later global extension query or
    // instance creation must be able to observe a changed driver set.
    icd::unload_preloaded_icds();
}

pub(crate) unsafe extern "system" fn destroy_instance_terminator(
    instance: VkInstance,
    allocator: *const VkAllocationCallbacks<'_>,
) {
    // SAFETY: The public trampoline retains ownership until the layer chain
    // unwinds, while the terminator performs the actual child/ICD teardown.
    let Some(instance) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return;
    };
    debug_messenger::destroy_all(instance, allocator);
    destroy_all_surfaces(instance);
    destroy_icd_instances(&instance.icds, allocator);
}

/// Creates a loader device backed by the ICD owning `physical_device`.
///
/// # Safety
///
/// All handles and pointers must satisfy the Vulkan API contract.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkCreateDevice(
    physical_device: VkPhysicalDevice,
    create_info: *const VkDeviceCreateInfo<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    device: *mut VkDevice,
) -> VkResult {
    let _loader_guard = platform::lock_loader();
    if create_info.is_null() || device.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The Vulkan entry-point contract requires readable create info.
    let create_info = unsafe { &*create_info };
    let trampoline = unsafe { LoaderPhysicalDeviceTrampoline::from_handle(physical_device) }
        .unwrap_or_else(|| {
            fatal_loader_error(
                c"vkCreateDevice: Invalid physicalDevice [VUID-vkCreateDevice-physicalDevice-parameter]",
            )
        });
    // SAFETY: Every application physical-device wrapper carries the owning
    // instance's loader dispatch key in its first word.
    let instance = unsafe { LoaderInstance::from_dispatchable(physical_device.0.cast()) }
        .unwrap_or_else(|| {
            fatal_loader_error(
                c"vkCreateDevice: Invalid physicalDevice [VUID-vkCreateDevice-physicalDevice-parameter]",
            )
        });
    if unsafe { layer::has_mismatched_device_layers(&instance.enabled_layer_names, create_info) } {
        instance.submit_loader_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
            c"loader_create_device_chain: Using deprecated and ignored 'ppEnabledLayerNames' member of 'VkDeviceCreateInfo' when creating a Vulkan device.",
        );
    }
    // Querying through the layer chain both validates the wrapped physical
    // device and lets layers contribute device extensions.
    let chain_physical_device = trampoline.chain;
    let layer_extensions =
        match unsafe { layer::available_device_extensions(instance, chain_physical_device) } {
            Ok(extensions) => extensions,
            Err(result) => return result,
        };
    let extension_token = pending::push_device_extensions(&layer_extensions);
    let mut chain_create_info = *create_info;
    // Translate application trampoline handles embedded in device-group
    // create info to the corresponding top-of-instance-chain handles.
    let group_patch = match unsafe {
        translate_device_group_chain(&mut chain_create_info, |handle| {
            // SAFETY: Device-group members are live application physical devices.
            LoaderPhysicalDeviceTrampoline::from_handle(handle).map(|device| device.chain)
        })
    } {
        Ok(patch) => patch,
        Err(result) => {
            let popped = pending::pop_device_extensions();
            debug_assert_eq!(popped, Some(extension_token));
            return result;
        }
    };
    // SAFETY: The trampoline retains its terminator wrapper and native ICD.
    unsafe { emit_device_layer_callstack(instance, trampoline) };
    // Upstream executes the chain builder even when no layers are active. In
    // that case it still inserts VK_LOADER_DATA_CALLBACK for the loader/driver
    // dispatchable-object contract before calling the terminator.
    // SAFETY: The activated layers/terminator and caller-owned create
    // structures remain live for the synchronous chain call.
    let result = unsafe {
        layer::create_device_chain(
            instance,
            chain_physical_device,
            &chain_create_info,
            allocator,
            device,
        )
    };
    drop(group_patch);
    let popped = pending::pop_device_extensions();
    debug_assert_eq!(popped, Some(extension_token));
    result
}

#[cold]
unsafe fn emit_device_layer_callstack(
    instance: &LoaderInstance,
    trampoline: &LoaderPhysicalDeviceTrampoline,
) {
    for message in [
        "vkCreateDevice layer callstack setup to:",
        "   <Application>",
        "     ||",
        "   <Loader>",
        "     ||",
    ] {
        emit_instance_category_message(instance, &["layer", "driver"], "DRIVER", message);
    }
    for layer in instance.layers.iter().rev() {
        emit_instance_category_message(
            instance,
            &["layer"],
            "LAYER",
            format!("   {}", layer.name.to_string_lossy()),
        );
        emit_instance_category_message(
            instance,
            &["layer"],
            "LAYER",
            format!(
                "           Type: {}",
                if layer.implicit {
                    "Implicit"
                } else {
                    "Explicit"
                }
            ),
        );
        emit_instance_category_message(instance, &["layer"], "LAYER", "     ||");
    }
    emit_instance_category_message(instance, &["layer", "driver"], "DRIVER", "   <Device>");

    let Some(physical_device) =
        (unsafe { LoaderPhysicalDevice::from_handle(trampoline.terminator) })
    else {
        return;
    };
    let icd = physical_device.icd();
    let mut properties = vk::VkPhysicalDeviceProperties::DEFAULT;
    if let Some(get_properties) = icd.dispatch.vkGetPhysicalDeviceProperties {
        unsafe { get_properties(physical_device.native, &raw mut properties) };
    }
    let name = unsafe { CStr::from_ptr(properties.deviceName.as_ptr()) }.to_string_lossy();
    let path = icd.icd.library_path().map_or_else(
        || Cow::Borrowed(""),
        |path| path.as_os_str().to_string_lossy(),
    );
    emit_instance_category_message(
        instance,
        &["layer", "driver"],
        "DRIVER",
        format!("       Using \"{name}\" with driver: \"{path}\""),
    );
}

pub(crate) unsafe extern "system" fn create_device_terminator(
    physical_device: VkPhysicalDevice,
    create_info: *const VkDeviceCreateInfo<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    device: *mut VkDevice,
) -> VkResult {
    if create_info.is_null() || device.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The device-create contract requires readable create info.
    let create_info = unsafe { &*create_info };
    unsafe { layer::validate_pending_device_output(&mut *device) };
    // SAFETY: A non-null physical device supplied by the chain must be live.
    let Some(physical_device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) })
    else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let icd_instance = physical_device.icd();
    let pending_extensions = pending::device_extensions();
    let fallback_extensions;
    let layer_extensions = if let Some((extensions, extension_count)) = pending_extensions {
        // SAFETY: The public trampoline owns this boxed slice for the entire
        // synchronous device-creation chain.
        unsafe { core::slice::from_raw_parts(extensions, extension_count) }
    } else {
        fallback_extensions = physical_device
            .instance()
            .layers
            .iter()
            .flat_map(|layer| {
                layer
                    .device_extensions
                    .iter()
                    .map(|extension| extension.name.clone())
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        &fallback_extensions
    };
    // SAFETY: The terminator has recovered the ICD's native physical device.
    let icd_extension_names = match unsafe {
        validate_and_filter_device_extensions(
            icd_instance,
            physical_device.native,
            create_info,
            layer_extensions,
        )
    } {
        Ok(names) => names,
        Err(result) => return result,
    };
    // SAFETY: The physical device and native instance belong to this ICD.
    let Some(create_device): Option<PFN_vkCreateDevice> = (unsafe {
        icd_instance
            .icd
            .resolve(icd_instance.handle, c"vkCreateDevice")
    }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let mut native = VkDevice::NULL;
    let mut icd_create_info = *create_info;
    icd_create_info.enabledExtensionCount = icd_extension_names.len() as u32;
    icd_create_info.ppEnabledExtensionNames = if icd_extension_names.is_empty() {
        core::ptr::null()
    } else {
        icd_extension_names.as_ptr()
    };
    // Translate terminator physical-device wrappers embedded in a device-group
    // create-info node to the ICD's native handles.
    let _group_patch = match unsafe {
        translate_device_group_chain(&mut icd_create_info, |handle| {
            // SAFETY: Layers pass the matching loader terminator handles down.
            LoaderPhysicalDevice::from_handle(handle).map(|device| device.native)
        })
    } {
        Ok(patch) => patch,
        Err(result) => return result,
    };
    // SAFETY: The caller owns the create structures and output storage, while
    // the translated physical-device handle belongs to the selected ICD.
    let result = unsafe {
        create_device(
            physical_device.native,
            &raw const icd_create_info,
            allocator,
            &raw mut native,
        )
    };
    if result != VkResult::SUCCESS {
        return result;
    }

    // SAFETY: Device proc-address lookup is exposed through this native instance.
    let get_device_proc_addr: Option<PFN_vkGetDeviceProcAddr> = unsafe {
        icd_instance
            .icd
            .resolve(icd_instance.handle, c"vkGetDeviceProcAddr")
    };
    let Some(get_device_proc_addr) = get_device_proc_addr else {
        // A conforming ICD supplies GDPA. Clean up a device returned by a broken ICD.
        // SAFETY: The native device was created immediately above by this ICD.
        let destroy: Option<PFN_vkDestroyDevice> = unsafe {
            icd_instance
                .icd
                .resolve(icd_instance.handle, c"vkDestroyDevice")
        };
        if let Some(destroy) = destroy {
            // SAFETY: Native handle and allocator match the create call.
            unsafe { destroy(native, allocator) };
        }
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };

    // SAFETY: The validated create info remains live for this call.
    let strict_version_checks = unsafe { maintenance5_version_checks(&*create_info) };
    // SAFETY: Device extension names satisfy the create-info string-array contract.
    let enabled_extensions = unsafe {
        ExtensionSet::from_names(
            (*create_info).enabledExtensionCount,
            (*create_info).ppEnabledExtensionNames,
        )
    };
    // SAFETY: `native` was just returned by this ICD, its GDPA was resolved
    // from the matching live instance, and the parent instance owns this path.
    let loader_device = unsafe {
        LoaderDevice::new(
            native,
            get_device_proc_addr,
            physical_device.instance(),
            physical_device.icd_index,
            physical_device.app_api_version,
            strict_version_checks,
            enabled_extensions,
        )
    };
    // SAFETY: The caller supplied writable device storage.
    unsafe { device.write(LoaderDevice::register(loader_device)) };
    VkResult::SUCCESS
}

/// Destroys a loader device and its native ICD device.
///
/// # Safety
///
/// `device` must be null or a live device returned by this loader. `allocator`
/// must match the allocator used for device creation.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkDestroyDevice(
    device: VkDevice,
    allocator: *const VkAllocationCallbacks<'_>,
) {
    if device == VkDevice::NULL {
        return;
    }
    let _loader_guard = platform::lock_loader();
    // SAFETY: A live device wrapper starts with its loader dispatch table.
    let dispatch = unsafe { device_dispatch(device.0.cast()) };
    let Some(dispatch) = dispatch else {
        return;
    };
    debug_assert!(dispatch.vkDestroyDevice.is_some());
    // SAFETY: Core Vulkan 1.0 requires this entry in every conforming chain.
    let destroy = unsafe { dispatch.vkDestroyDevice.unwrap_unchecked() };
    let dispatch_key = core::ptr::from_ref(dispatch);
    // SAFETY: Forward the caller's live chain handle and matching allocator.
    unsafe { destroy(device, allocator) };
    // Upstream keeps the logical-device record registered until the complete
    // layer/ICD destroy chain has returned. Destruction callbacks may resolve
    // loader state re-entrantly while the native device is being torn down.
    drop(LoaderDevice::take_dispatch(dispatch_key));
}

pub(crate) unsafe extern "system" fn destroy_device_terminator(
    device: VkDevice,
    allocator: *const VkAllocationCallbacks<'_>,
) {
    // SAFETY: The public trampoline retains the loader record until the
    // complete destroy chain returns, matching loader_layer_destroy_device.
    let Some(device) = (unsafe { LoaderDevice::from_handle(device) }) else {
        return;
    };
    // SAFETY: The native device remains live until the resolved destroy call returns.
    let destroy = device.icd_destroy_device();
    if let Some(destroy) = destroy {
        // SAFETY: Native handle and allocator belong to this device.
        unsafe { destroy(device.icd_device, allocator) };
    }
}

/// Resolves a command for a loader device.
///
/// # Safety
///
/// `device` must be a live loader device and `name` must point to a live,
/// NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkGetDeviceProcAddr(
    device: VkDevice,
    name: *const c_char,
) -> PFN_vkVoidFunction {
    if name.is_null() {
        return None;
    }
    // SAFETY: The Vulkan entry-point contract requires a NUL-terminated name.
    let name = unsafe { CStr::from_ptr(name) };
    if name == c"vkGetDeviceProcAddr" {
        return Some(LoaderDevice::loader_proc_addr());
    }
    // SAFETY: A live Vulkan device stores its loader dispatch table in its
    // first machine word. The magic check rejects null or incompatible data.
    let dispatch = unsafe { device_dispatch(device.0.cast()) }?;
    let Some(lookup) = command_lookup(name) else {
        let resolver = dispatch.vkGetDeviceProcAddr?;
        // SAFETY: The resolver was installed from this completed device chain.
        return unsafe { resolver(device, name.as_ptr()) };
    };
    if lookup.scope != CommandScope::Device {
        return None;
    }
    // SAFETY: The generated id maps to the corresponding field in this ABI
    // dispatch table. Creation already masked unavailable core/extension commands.
    let command = unsafe { layer_device_dispatch_proc_addr(dispatch, lookup.id) }?;
    if command_must_use_loader_trampoline(lookup.id) {
        return exported_proc_addr(lookup.id);
    }
    Some(command)
}

/// Enumerates physical devices from every ICD attached to an instance.
///
/// # Safety
///
/// The count and optional output array must satisfy Vulkan's enumeration
/// contract, and `instance` must be a live loader instance.
pub(crate) unsafe extern "system" fn terminator_enumerate_physical_devices(
    instance: VkInstance,
    physical_device_count: *mut u32,
    physical_devices: *mut VkPhysicalDevice,
) -> VkResult {
    // SAFETY: A Vulkan instance entry point requires a live instance.
    let Some(instance) = (unsafe {
        LoaderInstance::from_handle(instance)
            .or_else(|| LoaderInstance::from_internal_handle(instance))
    }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if physical_device_count.is_null() {
        instance.submit_loader_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL
                | vk::VkDebugUtilsMessageTypeFlagBitsEXT::VALIDATION,
            c"vkEnumeratePhysicalDevices: Invalid pPhysicalDeviceCount pointer [VUID-vkEnumeratePhysicalDevices-pPhysicalDeviceCount-parameter]",
        );
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: Every native instance is retained by `instance` for this call.
    let native_devices = match unsafe { discover_active_physical_devices(instance) } {
        Ok(devices) => devices,
        Err(result) => {
            // SAFETY: The caller supplied writable count storage.
            unsafe { physical_device_count.write(0) };
            return result;
        }
    };
    let mut devices = instance.physical_devices.lock();
    let mut active = Vec::new();
    if active.try_reserve_exact(native_devices.len()).is_err() {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    for native_device in &native_devices {
        let key = (native_device.icd_index, native_device.handle.0 as usize);
        let device = devices.owned.entry(key).or_insert_with(|| {
            Box::new(LoaderPhysicalDevice::new(
                native_device.icd_index,
                &instance.icds[native_device.icd_index],
                instance,
                instance.api_version,
                native_device.handle,
            ))
        });
        active.push(device.handle());
    }
    let active = active.into_boxed_slice();
    devices.active = active;

    let total = devices.active.len().min(u32::MAX as usize) as u32;
    if physical_devices.is_null() {
        // SAFETY: The caller supplied writable count storage.
        unsafe { physical_device_count.write(total) };
        return VkResult::SUCCESS;
    }
    // SAFETY: The caller supplied readable/writable count storage.
    let capacity = unsafe { physical_device_count.read() } as usize;
    let written = capacity.min(devices.active.len());
    for (index, device) in devices.active.iter().take(written).enumerate() {
        // SAFETY: Vulkan's contract provides `capacity` writable entries.
        unsafe { physical_devices.add(index).write(*device) };
    }
    // SAFETY: The caller supplied writable count storage.
    unsafe {
        physical_device_count.write(written as u32);
    }
    if written < devices.active.len() {
        if let Ok(message) = alloc::ffi::CString::new(format!(
            "vkEnumeratePhysicalDevices: Trimming device count from {} to {written}",
            devices.active.len()
        )) {
            instance.submit_loader_message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
                &message,
            );
        }
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

/// Enumerates physical devices through the active instance layer chain.
///
/// # Safety
///
/// The count and optional output array must satisfy Vulkan's enumeration
/// contract, and `instance` must be a live loader instance.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumeratePhysicalDevices(
    instance: VkInstance,
    physical_device_count: *mut u32,
    physical_devices: *mut VkPhysicalDevice,
) -> VkResult {
    let _loader_guard = platform::lock_loader();
    // SAFETY: A Vulkan instance entry point requires a live instance wrapper.
    let loader = unsafe { LoaderInstance::from_handle(instance) }.unwrap_or_else(|| {
        fatal_loader_error(
            c"vkEnumeratePhysicalDevices: Invalid instance [VUID-vkEnumeratePhysicalDevices-instance-parameter]",
        )
    });
    if physical_device_count.is_null() {
        loader.submit_loader_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL
                | vk::VkDebugUtilsMessageTypeFlagBitsEXT::VALIDATION,
            c"vkEnumeratePhysicalDevices: Invalid pPhysicalDeviceCount pointer [VUID-vkEnumeratePhysicalDevices-pPhysicalDeviceCount-parameter]",
        );
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The immutable dispatch allocation lives with the instance.
    let dispatch = unsafe { &*loader.dispatch() };
    debug_assert!(dispatch.vkEnumeratePhysicalDevices.is_some());
    // SAFETY: Dispatch loading guarantees this core Vulkan 1.0 command exists.
    let enumerate = unsafe { dispatch.vkEnumeratePhysicalDevices.unwrap_unchecked() };
    let filters = IdFilters::from_environment();
    // SAFETY: The caller's Vulkan contracts are forwarded unchanged.
    let result = if let Some(filters) = filters.as_deref() {
        unsafe {
            enumerate_filtered_physical_devices(
                loader,
                dispatch,
                enumerate,
                instance,
                physical_device_count,
                physical_devices,
                filters,
            )
        }
    } else {
        unsafe { enumerate(instance, physical_device_count, physical_devices) }
    };
    if !physical_devices.is_null() && matches!(result, VkResult::SUCCESS | VkResult::INCOMPLETE) {
        let count = unsafe { physical_device_count.read() } as usize;
        if let Err(error) =
            unsafe { setup_trampoline_physical_devices(loader, physical_devices, count) }
        {
            return error;
        }
        retire_icds_without_physical_devices(loader);
    }
    result
}

#[cold]
#[inline(never)]
fn retire_icds_without_physical_devices(instance: &LoaderInstance) {
    for (icd_index, icd) in instance.active_icds() {
        let has_physical_devices = {
            let devices = instance.physical_devices.lock();
            devices.active.iter().any(|handle| {
                // SAFETY: The terminator owns every handle in its active list.
                unsafe { LoaderPhysicalDevice::from_handle(*handle) }
                    .is_some_and(|device| device.icd_index == icd_index)
            })
        };
        if has_physical_devices || !icd.begin_retire() {
            continue;
        }

        if let Some(path) = icd.icd.library_path()
            && let Ok(message) = alloc::ffi::CString::new(format!(
                "Removing driver {} due to not having any physical devices",
                path.to_string_lossy()
            ))
        {
            instance.submit_loader_message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
                &message,
            );
        }
        destroy_icd_surfaces(instance, icd_index);
        debug_messenger::destroy_icd_objects(instance, icd_index);
        if let Some(destroy) = icd.dispatch.vkDestroyInstance {
            // SAFETY: This call owns the one-time retirement claim and all ICD
            // children were destroyed immediately above.
            unsafe { destroy(icd.handle, instance.forced_destroy_allocator()) };
        }
        if let Some(path) = icd.icd.library_path() {
            icd::unload_preloaded_icd(path);
        }
        icd.icd.unload_library();
    }
}

unsafe fn setup_trampoline_physical_devices(
    instance: &LoaderInstance,
    physical_devices: *mut VkPhysicalDevice,
    count: usize,
) -> Result<(), VkResult> {
    let mut state = instance.physical_devices.lock();
    state
        .trampolines
        .try_reserve(count)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for index in 0..count {
        let chain = unsafe { physical_devices.add(index).read() };
        let terminator = state.active.get(index).copied().unwrap_or(chain);
        let trampoline = state
            .trampolines
            .entry(chain.0 as usize)
            .or_insert_with(|| {
                Box::new(LoaderPhysicalDeviceTrampoline::new(
                    instance, chain, terminator,
                ))
            });
        unsafe { physical_devices.add(index).write(trampoline.handle()) };
    }
    Ok(())
}

unsafe fn setup_trampoline_physical_device_groups(
    instance: &LoaderInstance,
    groups: *mut VkPhysicalDeviceGroupProperties<'_>,
    count: usize,
) -> Result<(), VkResult> {
    for group_index in 0..count {
        let group = unsafe { &mut *groups.add(group_index) };
        let device_count =
            (group.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        unsafe {
            setup_trampoline_physical_devices(
                instance,
                group.physicalDevices.as_mut_ptr(),
                device_count,
            )
        }?;
    }
    Ok(())
}

const MAX_ID_FILTERS: usize = 16;

#[derive(Clone, Copy)]
struct IdRange {
    begin: u32,
    end: u32,
}

#[derive(Default)]
struct IdFilter {
    ranges: Box<[IdRange]>,
}

impl IdFilter {
    fn parse(value: &std::ffi::OsStr) -> Self {
        let mut ranges = Vec::with_capacity(MAX_ID_FILTERS);
        for token in value.to_string_lossy().split(',').take(MAX_ID_FILTERS) {
            let (begin, consumed) = parse_c_u32(token.as_bytes());
            let end = token
                .as_bytes()
                .get(consumed.saturating_add(1)..)
                .map_or(begin, |tail| parse_c_u32(tail).0);
            ranges.push(IdRange { begin, end });
        }
        Self {
            ranges: ranges.into_boxed_slice(),
        }
    }

    fn matches(&self, value: u32) -> bool {
        self.ranges
            .iter()
            .any(|range| range.begin <= value && value <= range.end)
    }

    fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
}

fn parse_c_u32(bytes: &[u8]) -> (u32, usize) {
    let whitespace = bytes
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let mut index = whitespace;
    let negative = bytes.get(index) == Some(&b'-');
    if matches!(bytes.get(index), Some(b'-' | b'+')) {
        index += 1;
    }
    let (radix, prefix) =
        if bytes.get(index) == Some(&b'0') && matches!(bytes.get(index + 1), Some(b'x' | b'X')) {
            (16_u32, 2_usize)
        } else if bytes.get(index) == Some(&b'0') {
            (8, 0)
        } else {
            (10, 0)
        };
    index += prefix;
    let digit_start = index;
    let mut value = 0_u64;
    while let Some(digit) = bytes.get(index).and_then(|byte| match byte {
        b'0'..=b'9' => Some(u32::from(byte - b'0')),
        b'a'..=b'f' => Some(u32::from(byte - b'a') + 10),
        b'A'..=b'F' => Some(u32::from(byte - b'A') + 10),
        _ => None,
    }) {
        if digit >= radix {
            break;
        }
        value = value
            .saturating_mul(u64::from(radix))
            .saturating_add(u64::from(digit));
        index += 1;
    }
    if index == digit_start {
        return (0, whitespace);
    }
    let value = if negative {
        0_u64.wrapping_sub(value)
    } else {
        value
    };
    ((value & u64::from(u32::MAX)) as u32, index)
}

#[derive(Default)]
struct IdFilters {
    device: IdFilter,
    vendor: IdFilter,
    driver: IdFilter,
}

impl IdFilters {
    fn from_environment() -> Option<Box<Self>> {
        if platform::has_elevated_privileges() {
            return None;
        }
        let device = std::env::var_os("VK_LOADER_DEVICE_ID_FILTER");
        let vendor = std::env::var_os("VK_LOADER_VENDOR_ID_FILTER");
        let driver = std::env::var_os("VK_LOADER_DRIVER_ID_FILTER");
        if device.as_ref().is_none_or(|value| value.is_empty())
            && vendor.as_ref().is_none_or(|value| value.is_empty())
            && driver.as_ref().is_none_or(|value| value.is_empty())
        {
            return None;
        }
        Some(Box::new(Self {
            device: device.as_deref().map(IdFilter::parse).unwrap_or_default(),
            vendor: vendor.as_deref().map(IdFilter::parse).unwrap_or_default(),
            driver: driver.as_deref().map(IdFilter::parse).unwrap_or_default(),
        }))
    }
}

struct IdFilterPropertyStorage {
    basic: vk::VkPhysicalDeviceProperties,
    properties2: vk::VkPhysicalDeviceProperties2<'static>,
    driver: vk::VkPhysicalDeviceDriverProperties<'static>,
}

unsafe fn physical_device_matches_id_filters(
    instance: &LoaderInstance,
    dispatch: &LayerInstanceDispatchTable,
    physical_device: VkPhysicalDevice,
    filters: &IdFilters,
    storage: *mut IdFilterPropertyStorage,
) -> bool {
    let Some(get_properties) = dispatch.vkGetPhysicalDeviceProperties else {
        return false;
    };
    unsafe { get_properties(physical_device, core::ptr::addr_of_mut!((*storage).basic)) };
    let device_id = unsafe { core::ptr::addr_of!((*storage).basic.deviceID).read() };
    let vendor_id = unsafe { core::ptr::addr_of!((*storage).basic.vendorID).read() };
    if (!filters.device.is_empty() && !filters.device.matches(device_id))
        || (!filters.vendor.is_empty() && !filters.vendor.matches(vendor_id))
    {
        return false;
    }
    if filters.driver.is_empty() {
        return true;
    }

    unsafe {
        core::ptr::addr_of_mut!((*storage).properties2.sType)
            .write(vk::VkStructureType::PHYSICAL_DEVICE_PROPERTIES_2);
        core::ptr::addr_of_mut!((*storage).properties2.pNext)
            .write(core::ptr::addr_of_mut!((*storage).driver).cast());
        core::ptr::addr_of_mut!((*storage).driver.sType)
            .write(vk::VkStructureType::PHYSICAL_DEVICE_DRIVER_PROPERTIES);
        core::ptr::addr_of_mut!((*storage).driver.pNext).write(core::ptr::null_mut());
    }
    if instance.api_version >= vk::VK_API_VERSION_1_1 {
        let Some(get_properties2) = dispatch.vkGetPhysicalDeviceProperties2 else {
            return false;
        };
        unsafe {
            get_properties2(
                physical_device,
                core::ptr::addr_of_mut!((*storage).properties2),
            );
        };
    } else {
        let extension_enabled =
            extension_id(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
                .is_some_and(|id| instance.enabled_extensions.contains(id));
        let Some(get_properties2) = extension_enabled
            .then_some(dispatch.vkGetPhysicalDeviceProperties2KHR)
            .flatten()
        else {
            return false;
        };
        debug_assert_eq!(
            core::mem::size_of::<vk::VkPhysicalDeviceProperties2KHR<'_>>(),
            core::mem::size_of::<vk::VkPhysicalDeviceProperties2<'_>>()
        );
        debug_assert_eq!(
            core::mem::align_of::<vk::VkPhysicalDeviceProperties2KHR<'_>>(),
            core::mem::align_of::<vk::VkPhysicalDeviceProperties2<'_>>()
        );
        unsafe {
            get_properties2(
                physical_device,
                core::ptr::addr_of_mut!((*storage).properties2).cast(),
            );
        };
    }
    let driver_id = unsafe { core::ptr::addr_of!((*storage).driver.driverID).read() };
    filters.driver.matches(driver_id.0.cast_unsigned())
}

#[cold]
#[inline(never)]
unsafe fn enumerate_filtered_physical_devices(
    loader: &LoaderInstance,
    dispatch: &LayerInstanceDispatchTable,
    enumerate: PFN_vkEnumeratePhysicalDevices,
    instance: VkInstance,
    physical_device_count: *mut u32,
    physical_devices: *mut VkPhysicalDevice,
    filters: &IdFilters,
) -> VkResult {
    let mut available = 0;
    let result = unsafe { enumerate(instance, &raw mut available, core::ptr::null_mut()) };
    if result != VkResult::SUCCESS {
        return result;
    }
    let available = available as usize;
    let mut chain_devices = Vec::new();
    if chain_devices.try_reserve_exact(available).is_err() {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    chain_devices.resize(available, VkPhysicalDevice::NULL);
    let mut returned = available.min(u32::MAX as usize) as u32;
    let result = unsafe { enumerate(instance, &raw mut returned, chain_devices.as_mut_ptr()) };
    if result != VkResult::SUCCESS {
        return result;
    }
    chain_devices.truncate((returned as usize).min(available));

    let capacity = if physical_devices.is_null() {
        usize::MAX
    } else {
        (unsafe { physical_device_count.read() }) as usize
    };
    let mut storage = Box::<IdFilterPropertyStorage>::new_uninit();
    let storage = storage.as_mut_ptr();
    let mut matched = 0_usize;
    for physical_device in chain_devices {
        if !unsafe {
            physical_device_matches_id_filters(loader, dispatch, physical_device, filters, storage)
        } {
            continue;
        }
        if !physical_devices.is_null() && matched < capacity {
            unsafe { physical_devices.add(matched).write(physical_device) };
        }
        matched += 1;
    }
    let written = capacity.min(matched);
    unsafe {
        physical_device_count.write(written as u32);
    }
    if written < matched {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

#[cold]
#[inline(never)]
unsafe fn enumerate_filtered_physical_device_groups(
    loader: &LoaderInstance,
    dispatch: &LayerInstanceDispatchTable,
    enumerate: vk::PFN_vkEnumeratePhysicalDeviceGroups,
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
    filters: &IdFilters,
) -> VkResult {
    let mut available = 0;
    let result = unsafe { enumerate(instance, &raw mut available, core::ptr::null_mut()) };
    if result != VkResult::SUCCESS {
        return result;
    }
    let available = available as usize;
    let mut chain_groups =
        Box::<[VkPhysicalDeviceGroupProperties<'_>]>::new_uninit_slice(available);
    unsafe { chain_groups.as_mut_ptr().write_bytes(0, available) };
    let mut returned = available.min(u32::MAX as usize) as u32;
    let result = unsafe {
        enumerate(
            instance,
            &raw mut returned,
            chain_groups.as_mut_ptr().cast(),
        )
    };
    if result != VkResult::SUCCESS {
        return result;
    }
    let returned = (returned as usize).min(available);
    let chain_groups = unsafe { chain_groups.assume_init() };

    let capacity = if group_properties.is_null() {
        usize::MAX
    } else {
        (unsafe { group_count.read() }) as usize
    };
    let mut storage = Box::<IdFilterPropertyStorage>::new_uninit();
    let storage = storage.as_mut_ptr();
    let mut matched = 0_usize;
    'groups: for group in &chain_groups[..returned] {
        let device_count =
            (group.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        for &physical_device in &group.physicalDevices[..device_count] {
            if !unsafe {
                physical_device_matches_id_filters(
                    loader,
                    dispatch,
                    physical_device,
                    filters,
                    storage,
                )
            } {
                continue 'groups;
            }
        }
        if !group_properties.is_null() && matched < capacity {
            unsafe { group_properties.add(matched).write(*group) };
        }
        matched += 1;
    }
    let written = capacity.min(matched);
    unsafe {
        group_count.write(written as u32);
    }
    if written < matched {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

#[derive(Clone, Copy)]
enum IcdGroupEnumerator {
    Core(vk::PFN_vkEnumeratePhysicalDeviceGroups),
    Khr(vk::PFN_vkEnumeratePhysicalDeviceGroupsKHR),
    PhysicalDevices(PFN_vkEnumeratePhysicalDevices),
}

fn icd_group_enumerator(loader: &LoaderInstance, icd: &IcdInstance) -> Option<IcdGroupEnumerator> {
    let use_khr = extension_id(vk::VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME)
        .is_some_and(|id| loader.enabled_extensions.contains(id));
    if use_khr {
        icd.dispatch
            .vkEnumeratePhysicalDeviceGroupsKHR
            .map(IcdGroupEnumerator::Khr)
    } else {
        icd.dispatch
            .vkEnumeratePhysicalDeviceGroups
            .map(IcdGroupEnumerator::Core)
    }
    .or_else(|| {
        icd.dispatch
            .vkEnumeratePhysicalDevices
            .map(IcdGroupEnumerator::PhysicalDevices)
    })
}

unsafe fn query_icd_group_count(
    icd: &IcdInstance,
    enumerate: IcdGroupEnumerator,
) -> Result<u32, VkResult> {
    let mut count = 0;
    let result = match enumerate {
        IcdGroupEnumerator::Core(enumerate) => unsafe {
            enumerate(icd.handle, &raw mut count, core::ptr::null_mut())
        },
        IcdGroupEnumerator::Khr(enumerate) => unsafe {
            enumerate(icd.handle, &raw mut count, core::ptr::null_mut())
        },
        IcdGroupEnumerator::PhysicalDevices(enumerate) => unsafe {
            enumerate(icd.handle, &raw mut count, core::ptr::null_mut())
        },
    };
    (result == VkResult::SUCCESS).then_some(count).ok_or(result)
}

unsafe fn enumerate_icd_groups(
    icd: &IcdInstance,
    enumerate: IcdGroupEnumerator,
    output: *mut VkPhysicalDeviceGroupProperties<'_>,
    output_capacity: usize,
    output_offset: usize,
) -> Result<Box<[VkPhysicalDeviceGroupProperties<'static>]>, VkResult> {
    let count = unsafe { query_icd_group_count(icd, enumerate) }? as usize;
    match enumerate {
        IcdGroupEnumerator::Core(enumerate) => {
            let mut groups = Vec::new();
            groups
                .try_reserve_exact(count)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            groups.resize(count, VkPhysicalDeviceGroupProperties::DEFAULT);
            for (index, group) in groups.iter_mut().enumerate() {
                if output_offset + index < output_capacity {
                    group.pNext = unsafe { (*output.add(output_offset + index)).pNext };
                }
            }
            let mut returned = count.min(u32::MAX as usize) as u32;
            let result = unsafe { enumerate(icd.handle, &raw mut returned, groups.as_mut_ptr()) };
            if result != VkResult::SUCCESS && result != VkResult::INCOMPLETE {
                return Err(result);
            }
            groups.truncate((returned as usize).min(count));
            Ok(groups.into_boxed_slice())
        }
        IcdGroupEnumerator::Khr(enumerate) => {
            let mut groups = Vec::new();
            groups
                .try_reserve_exact(count)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            groups.resize(count, VkPhysicalDeviceGroupPropertiesKHR::DEFAULT);
            for (index, group) in groups.iter_mut().enumerate() {
                if output_offset + index < output_capacity {
                    group.pNext = unsafe { (*output.add(output_offset + index)).pNext };
                }
            }
            let mut returned = count.min(u32::MAX as usize) as u32;
            let result = unsafe { enumerate(icd.handle, &raw mut returned, groups.as_mut_ptr()) };
            if result != VkResult::SUCCESS && result != VkResult::INCOMPLETE {
                return Err(result);
            }
            let returned = (returned as usize).min(count);
            let mut promoted = Vec::new();
            promoted
                .try_reserve_exact(returned)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            promoted.extend(groups.into_iter().take(returned).map(|group| {
                VkPhysicalDeviceGroupProperties {
                    sType: group.sType,
                    pNext: group.pNext,
                    physicalDeviceCount: group.physicalDeviceCount,
                    physicalDevices: group.physicalDevices,
                    subsetAllocation: group.subsetAllocation,
                    ..VkPhysicalDeviceGroupProperties::DEFAULT
                }
            }));
            Ok(promoted.into_boxed_slice())
        }
        IcdGroupEnumerator::PhysicalDevices(_) => {
            let devices = unsafe { enumerate_icd_physical_devices(icd) }?;
            let mut groups = Vec::new();
            groups
                .try_reserve_exact(devices.len)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            for (index, device) in devices.iter().enumerate() {
                let mut group = VkPhysicalDeviceGroupProperties {
                    physicalDeviceCount: 1,
                    ..VkPhysicalDeviceGroupProperties::DEFAULT
                };
                group.physicalDevices[0] = device;
                if output_offset + index < output_capacity {
                    group.pNext = unsafe { (*output.add(output_offset + index)).pNext };
                }
                groups.push(group);
            }
            Ok(groups.into_boxed_slice())
        }
    }
}

unsafe fn discover_all_physical_devices(
    instance: &LoaderInstance,
) -> Result<Box<[NativePhysicalDevice]>, VkResult> {
    #[cfg(windows)]
    let mut devices = unsafe { windows_sorted_physical_devices(instance) }?.into_vec();
    #[cfg(not(windows))]
    let mut devices = Vec::new();
    let mut successful_icds = usize::from(!devices.is_empty());
    for (icd_index, icd) in instance.active_icds().rev() {
        let Ok(native) = (unsafe { enumerate_icd_physical_devices(icd) }) else {
            continue;
        };
        successful_icds += 1;
        devices
            .try_reserve(native.len)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        for handle in native.iter() {
            let device = NativePhysicalDevice { icd_index, handle };
            if !devices.contains(&device) {
                devices.push(device);
            }
        }
    }
    if successful_icds == 0 || devices.is_empty() {
        Err(VkResult::ERROR_INITIALIZATION_FAILED)
    } else {
        if linux_sort_enabled(instance) {
            unsafe { linux_sort_physical_devices(instance, &mut devices) }?;
        }
        Ok(devices.into_boxed_slice())
    }
}

#[cfg(windows)]
#[cold]
#[inline(never)]
unsafe fn windows_sorted_physical_devices(
    instance: &LoaderInstance,
) -> Result<Box<[NativePhysicalDevice]>, VkResult> {
    struct AdapterDevices {
        luid: platform::AdapterLuid,
        devices: Box<[NativePhysicalDevice]>,
    }

    unsafe fn is_d3d12_layered(instance: &LoaderInstance, group: &AdapterDevices) -> bool {
        let icd = &instance.icds[group.devices[0].icd_index];
        let Some(get_properties) = icd.dispatch.vkGetPhysicalDeviceProperties else {
            return false;
        };
        let mut basic = Box::<vk::VkPhysicalDeviceProperties>::new_uninit();
        for device in &group.devices {
            // SAFETY: `basic` points to writable storage of the exact Vulkan
            // output type, and the native handle belongs to this ICD.
            unsafe { get_properties(device.handle, basic.as_mut_ptr()) };
            // SAFETY: The ICD initialized the output structure before return.
            let api_version = unsafe { core::ptr::addr_of!((*basic.as_ptr()).apiVersion).read() };
            let get_properties2 = if instance.api_version >= vk::VK_API_VERSION_1_1
                && api_version >= vk::VK_API_VERSION_1_1
            {
                icd.dispatch.vkGetPhysicalDeviceProperties2
            } else {
                icd.dispatch
                    .vkGetPhysicalDeviceProperties2KHR
                    .map(|command| {
                        // The promoted and KHR signatures and structures are ABI
                        // aliases by the Vulkan specification.
                        unsafe {
                            core::mem::transmute::<
                                vk::PFN_vkGetPhysicalDeviceProperties2KHR,
                                vk::PFN_vkGetPhysicalDeviceProperties2,
                            >(command)
                        }
                    })
            };
            let Some(get_properties2) = get_properties2 else {
                continue;
            };
            let mut layered = vk::VkPhysicalDeviceLayeredDriverPropertiesMSFT::DEFAULT;
            let mut properties = vk::VkPhysicalDeviceProperties2 {
                pNext: core::ptr::from_mut(&mut layered).cast(),
                ..vk::VkPhysicalDeviceProperties2::DEFAULT
            };
            // SAFETY: Both output structures are initialized, correctly
            // chained, and live for the call.
            unsafe { get_properties2(device.handle, &raw mut properties) };
            if layered.underlyingAPI == vk::VkLayeredDriverUnderlyingApiMSFT::D3D12 {
                return true;
            }
        }
        false
    }

    let mut groups: Vec<AdapterDevices> = Vec::new();
    for luid in platform::adapter_luids() {
        for (icd_index, icd) in instance.active_icds().rev() {
            let Some(enumerate) = icd.icd.enumerate_adapter_physical_devices else {
                continue;
            };
            let mut count = 0;
            // SAFETY: Count is writable; a null output performs the required
            // loader-driver interface sizing query.
            let result =
                unsafe { enumerate(icd.handle, luid, &raw mut count, core::ptr::null_mut()) };
            if result == VkResult::ERROR_OUT_OF_HOST_MEMORY {
                return Err(result);
            }
            if result != VkResult::SUCCESS || count == 0 {
                continue;
            }

            let mut group = None;
            loop {
                let capacity = count as usize;
                let mut storage = Box::<[VkPhysicalDevice]>::new_uninit_slice(capacity);
                let mut returned = count;
                // SAFETY: Storage contains `capacity` writable handles and
                // `returned` supplies that capacity to the ICD.
                let result = unsafe {
                    enumerate(
                        icd.handle,
                        luid,
                        &raw mut returned,
                        storage.as_mut_ptr().cast(),
                    )
                };
                if result == VkResult::INCOMPLETE {
                    count = returned.max(count.saturating_add(1));
                    continue;
                }
                if result == VkResult::ERROR_OUT_OF_HOST_MEMORY {
                    return Err(result);
                }
                if result != VkResult::SUCCESS {
                    break;
                }
                let initialized = (returned as usize).min(capacity);
                let mut devices = Vec::new();
                devices
                    .try_reserve_exact(initialized)
                    .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
                devices.extend(storage[..initialized].iter().map(|device| {
                    // SAFETY: The successful ICD call initialized the reported
                    // prefix, capped to the allocated capacity.
                    NativePhysicalDevice {
                        icd_index,
                        handle: unsafe { device.assume_init() },
                    }
                }));
                group = Some(devices.into_boxed_slice());
                break;
            }
            let Some(group) = group else { continue };
            if !groups.iter().any(|existing| existing.devices == group) {
                groups.push(AdapterDevices {
                    luid,
                    devices: group,
                });
            }
        }
    }

    // Match `sort_physical_devices_with_same_luid`: when two ICDs expose the
    // same adapter, a Vulkan-on-D3D12 implementation follows the native ICD.
    for index in 0..groups.len().saturating_sub(1) {
        for candidate in index + 1..groups.len() {
            if groups[index].luid == groups[candidate].luid
                && unsafe { is_d3d12_layered(instance, &groups[index]) }
            {
                groups.swap(index, candidate);
            }
        }
    }

    let count = groups.iter().map(|group| group.devices.len()).sum();
    let mut devices = Vec::new();
    devices
        .try_reserve_exact(count)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    devices.extend(groups.into_iter().flat_map(|group| group.devices));
    Ok(devices.into_boxed_slice())
}

unsafe fn enumerate_physical_device_groups_impl(
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
) -> VkResult {
    let Some(instance) = (unsafe {
        LoaderInstance::from_handle(instance)
            .or_else(|| LoaderInstance::from_internal_handle(instance))
    }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if group_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let group_count = unsafe { &mut *group_count };

    let mut upper_bound = 0_u32;
    for (_, icd) in instance.active_icds() {
        let Some(enumerate) = icd_group_enumerator(instance, icd) else {
            continue;
        };
        if let Ok(count) = unsafe { query_icd_group_count(icd, enumerate) } {
            upper_bound = upper_bound.saturating_add(count);
        }
    }
    if group_properties.is_null() {
        *group_count = upper_bound;
        return if upper_bound == 0 {
            VkResult::ERROR_INITIALIZATION_FAILED
        } else {
            VkResult::SUCCESS
        };
    }
    unsafe {
        enumerate_physical_device_group_properties(
            instance,
            group_count,
            group_properties,
            upper_bound,
        )
    }
}

#[cold]
#[inline(never)]
unsafe fn enumerate_physical_device_group_properties(
    instance: &LoaderInstance,
    group_count: &mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
    upper_bound: u32,
) -> VkResult {
    let capacity = *group_count as usize;
    let all_devices = match unsafe { discover_all_physical_devices(instance) } {
        Ok(devices) => devices,
        Err(result) => {
            *group_count = 0;
            return result;
        }
    };
    #[cfg(windows)]
    let windows_sorted_devices = match unsafe { windows_sorted_physical_devices(instance) } {
        Ok(devices) => devices,
        Err(result) => {
            *group_count = 0;
            return result;
        }
    };
    let visible_devices = if instance.device_configurations.is_some() {
        match unsafe { discover_active_physical_devices(instance) } {
            Ok(devices) => Some(devices),
            Err(result) => {
                *group_count = 0;
                return result;
            }
        }
    } else {
        None
    };

    let mut native_groups = Vec::new();
    if native_groups
        .try_reserve_exact(upper_bound as usize)
        .is_err()
    {
        *group_count = 0;
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    for (icd_index, icd) in instance.active_icds().rev() {
        let Some(enumerate) = icd_group_enumerator(instance, icd) else {
            continue;
        };
        let groups = match unsafe {
            enumerate_icd_groups(
                icd,
                enumerate,
                group_properties,
                capacity,
                native_groups.len(),
            )
        } {
            Ok(groups) => groups,
            Err(result) => {
                *group_count = 0;
                return result;
            }
        };
        native_groups.extend(groups.into_iter().map(|properties| (icd_index, properties)));
    }
    if linux_sort_enabled(instance) {
        native_groups = match unsafe { linux_sort_physical_device_groups(instance, native_groups) }
        {
            Ok(groups) => groups,
            Err(result) => {
                *group_count = 0;
                return result;
            }
        };
    }
    #[cfg(windows)]
    if !windows_sorted_devices.is_empty() {
        windows_sort_physical_device_groups(&mut native_groups, &windows_sorted_devices);
    }

    let mut state = instance.physical_devices.lock();
    if state.owned.try_reserve(all_devices.len()).is_err() {
        *group_count = 0;
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    for device in &all_devices {
        let key = (device.icd_index, device.handle.0 as usize);
        state.owned.entry(key).or_insert_with(|| {
            Box::new(LoaderPhysicalDevice::new(
                device.icd_index,
                &instance.icds[device.icd_index],
                instance,
                instance.api_version,
                device.handle,
            ))
        });
    }

    let mut visible_groups = Vec::new();
    if visible_groups
        .try_reserve_exact(native_groups.len())
        .is_err()
    {
        *group_count = 0;
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    'groups: for (icd_index, mut properties) in native_groups {
        let device_count =
            (properties.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        properties.physicalDeviceCount = device_count as u32;
        for native in &mut properties.physicalDevices[..device_count] {
            if let Some(visible) = visible_devices.as_deref()
                && !visible
                    .iter()
                    .any(|device| device.icd_index == icd_index && device.handle == *native)
            {
                continue 'groups;
            }
            let key = (icd_index, native.0 as usize);
            let Some(wrapped) = state.owned.get(&key) else {
                *group_count = 0;
                return VkResult::ERROR_INITIALIZATION_FAILED;
            };
            *native = wrapped.handle();
        }
        visible_groups.push(properties);
    }

    let written = capacity.min(visible_groups.len());
    for (index, properties) in visible_groups.iter().take(written).enumerate() {
        unsafe { group_properties.add(index).write(*properties) };
    }
    *group_count = written as u32;
    if written < visible_groups.len() {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

#[cfg(windows)]
fn windows_sort_physical_device_groups(
    groups: &mut [(usize, VkPhysicalDeviceGroupProperties<'static>)],
    sorted_devices: &[NativePhysicalDevice],
) {
    let device_order = |icd_index: usize, handle: VkPhysicalDevice| {
        sorted_devices
            .iter()
            .position(|device| device.icd_index == icd_index && device.handle == handle)
            .unwrap_or(usize::MAX)
    };

    // Device groups are bounded by VK_MAX_DEVICE_GROUP_SIZE. An insertion
    // sort avoids allocation on this cold path and is equivalent to upstream's
    // repeated search-and-swap against the DXGI-prioritized device sequence.
    for (icd_index, properties) in groups.iter_mut() {
        let count =
            (properties.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        for index in 1..count {
            let mut current = index;
            while current != 0
                && device_order(*icd_index, properties.physicalDevices[current])
                    < device_order(*icd_index, properties.physicalDevices[current - 1])
            {
                properties.physicalDevices.swap(current, current - 1);
                current -= 1;
            }
        }
    }

    let group_order = |group: &(usize, VkPhysicalDeviceGroupProperties<'static>)| {
        let count =
            (group.1.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        group.1.physicalDevices[..count]
            .iter()
            .map(|&handle| device_order(group.0, handle))
            .min()
            .unwrap_or(usize::MAX)
    };
    for index in 1..groups.len() {
        let mut current = index;
        while current != 0 && group_order(&groups[current]) < group_order(&groups[current - 1]) {
            groups.swap(current, current - 1);
            current -= 1;
        }
    }
}

pub(crate) unsafe extern "system" fn terminator_enumerate_physical_device_groups(
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
) -> VkResult {
    unsafe { enumerate_physical_device_groups_impl(instance, group_count, group_properties) }
}

pub(crate) unsafe extern "system" fn terminator_enumerate_physical_device_groups_khr(
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupPropertiesKHR<'_>,
) -> VkResult {
    debug_assert_eq!(
        core::mem::size_of::<VkPhysicalDeviceGroupPropertiesKHR<'_>>(),
        core::mem::size_of::<VkPhysicalDeviceGroupProperties<'_>>()
    );
    debug_assert_eq!(
        core::mem::align_of::<VkPhysicalDeviceGroupPropertiesKHR<'_>>(),
        core::mem::align_of::<VkPhysicalDeviceGroupProperties<'_>>()
    );
    unsafe { enumerate_physical_device_groups_impl(instance, group_count, group_properties.cast()) }
}

/// Enumerates physical-device groups through the active instance chain.
///
/// # Safety
///
/// `instance` must be live, `group_count` must be writable, and a non-null
/// `group_properties` must provide the capacity supplied through `group_count`.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumeratePhysicalDeviceGroups(
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
) -> VkResult {
    if group_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let Some(loader) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let dispatch = unsafe { &*loader.dispatch() };
    let Some(enumerate) = dispatch.vkEnumeratePhysicalDeviceGroups else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let filters = IdFilters::from_environment();
    let result = if let Some(filters) = filters.as_deref() {
        unsafe {
            enumerate_filtered_physical_device_groups(
                loader,
                dispatch,
                enumerate,
                instance,
                group_count,
                group_properties,
                filters,
            )
        }
    } else {
        unsafe { enumerate(instance, group_count, group_properties) }
    };
    if !group_properties.is_null() && matches!(result, VkResult::SUCCESS | VkResult::INCOMPLETE) {
        let count = unsafe { group_count.read() } as usize;
        if let Err(error) =
            unsafe { setup_trampoline_physical_device_groups(loader, group_properties, count) }
        {
            return error;
        }
    }
    result
}

/// Enumerates physical-device groups through the KHR instance chain.
///
/// # Safety
///
/// `instance` must be live, `group_count` must be writable, and a non-null
/// `group_properties` must provide the capacity supplied through `group_count`.
pub unsafe extern "system" fn vkEnumeratePhysicalDeviceGroupsKHR(
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupPropertiesKHR<'_>,
) -> VkResult {
    if group_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let Some(loader) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let dispatch = unsafe { &*loader.dispatch() };
    let Some(enumerate) = dispatch.vkEnumeratePhysicalDeviceGroupsKHR else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let filters = IdFilters::from_environment();
    let result = if let Some(filters) = filters.as_deref() {
        let enumerate: vk::PFN_vkEnumeratePhysicalDeviceGroups =
            unsafe { core::mem::transmute(enumerate) };
        unsafe {
            enumerate_filtered_physical_device_groups(
                loader,
                dispatch,
                enumerate,
                instance,
                group_count,
                group_properties.cast(),
                filters,
            )
        }
    } else {
        unsafe { enumerate(instance, group_count, group_properties) }
    };
    if !group_properties.is_null() && matches!(result, VkResult::SUCCESS | VkResult::INCOMPLETE) {
        debug_assert_eq!(
            core::mem::size_of::<VkPhysicalDeviceGroupPropertiesKHR<'_>>(),
            core::mem::size_of::<VkPhysicalDeviceGroupProperties<'_>>()
        );
        debug_assert_eq!(
            core::mem::align_of::<VkPhysicalDeviceGroupPropertiesKHR<'_>>(),
            core::mem::align_of::<VkPhysicalDeviceGroupProperties<'_>>()
        );
        let count = unsafe { group_count.read() } as usize;
        if let Err(error) = unsafe {
            setup_trampoline_physical_device_groups(loader, group_properties.cast(), count)
        } {
            return error;
        }
    }
    result
}

/// Reports the device layers active on the physical device's instance.
///
/// # Safety
///
/// Arguments must satisfy `vkEnumerateDeviceLayerProperties`' Vulkan contract.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumerateDeviceLayerProperties(
    physical_device: VkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut VkLayerProperties,
) -> VkResult {
    let _loader_guard = platform::lock_loader();
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: Every live physical-device wrapper carries the owning instance dispatch key.
    let instance = unsafe { LoaderInstance::from_dispatchable(physical_device.0.cast()) }
        .unwrap_or_else(|| {
            fatal_loader_error(
                c"vkEnumerateDeviceLayerProperties: Invalid physicalDevice [VUID-vkEnumerateDeviceLayerProperties-physicalDevice-parameter]",
            )
        });
    // SAFETY: Forward the caller's enumeration storage contract.
    unsafe {
        layer::enumerate_active_device_layers(
            &instance.active_layer_properties,
            &mut *property_count,
            properties,
        )
    }
}

/// Enumerates ICD extensions or the extensions declared by a named layer.
///
/// # Safety
///
/// Arguments must satisfy `vkEnumerateDeviceExtensionProperties`' Vulkan contract.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumerateDeviceExtensionProperties(
    physical_device: VkPhysicalDevice,
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    let _loader_guard = platform::lock_loader();
    let (dispatch, physical_device) = unsafe {
        resolve_trampoline_physical_device(physical_device)
    }
    .unwrap_or_else(|| {
        fatal_loader_error(
            c"vkEnumerateDeviceExtensionProperties: Invalid physicalDevice [VUID-vkEnumerateDeviceExtensionProperties-physicalDevice-parameter]",
        )
    });
    let command = dispatch.vkEnumerateDeviceExtensionProperties;
    command.map_or_else(
        || unsafe {
            layer::terminator_enumerate_device_extension_properties(
                physical_device,
                layer_name,
                property_count,
                properties,
            )
        },
        |command| unsafe { command(physical_device, layer_name, property_count, properties) },
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct NativePhysicalDevice {
    icd_index: usize,
    handle: VkPhysicalDevice,
}

struct DeviceConfigurationProperties {
    properties: vk::VkPhysicalDeviceProperties2<'static>,
    identifiers: vk::VkPhysicalDeviceIDProperties<'static>,
    driver: vk::VkPhysicalDeviceDriverProperties<'static>,
}

#[derive(Clone, Copy)]
struct LinuxSortedDeviceInfo {
    device: NativePhysicalDevice,
    device_type: vk::VkPhysicalDeviceType,
    vendor_id: u32,
    device_id: u32,
    pci: Option<(u32, u32, u32, u32)>,
    default_device: bool,
    original_order: usize,
}

struct LinuxSortPropertyStorage {
    basic: vk::VkPhysicalDeviceProperties,
    properties2: vk::VkPhysicalDeviceProperties2<'static>,
    pci: vk::VkPhysicalDevicePCIBusInfoPropertiesEXT<'static>,
}

const LINUX_SORT_PLATFORM_ENABLED: bool = cfg!(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "hurd"
));

const fn linux_sort_requires_properties_extension(
    application_api_version: u32,
    driver_api_version: u32,
) -> bool {
    application_api_version < vk::VK_API_VERSION_1_1 || driver_api_version < vk::VK_API_VERSION_1_1
}

fn decimal_prefix_nonzero(bytes: &[u8]) -> bool {
    let mut index = bytes
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let negative = bytes.get(index) == Some(&b'-');
    if matches!(bytes.get(index), Some(b'-' | b'+')) {
        index += 1;
    }

    let limit = if negative {
        libc::c_long::MAX as u64 + 1
    } else {
        libc::c_long::MAX as u64
    };
    let mut magnitude = 0_u64;
    while let Some(digit) = bytes.get(index).and_then(|byte| match byte {
        b'0'..=b'9' => Some(u64::from(byte - b'0')),
        _ => None,
    }) {
        magnitude = magnitude
            .saturating_mul(10)
            .saturating_add(digit)
            .min(limit);
        index += 1;
    }

    let value = if negative {
        if magnitude == limit {
            libc::c_long::MIN
        } else {
            -(magnitude as libc::c_long)
        }
    } else {
        magnitude as libc::c_long
    };
    let low = value.cast_unsigned() & libc::c_ulong::from(u32::MAX);
    low != 0
}

fn scanf_hex(bytes: &[u8], mut index: usize) -> Option<(u32, usize)> {
    const C_ULONG_MAX: u128 = if core::mem::size_of::<libc::c_ulong>() == 4 {
        0xffff_ffff
    } else {
        0xffff_ffff_ffff_ffff
    };

    while bytes.get(index).is_some_and(u8::is_ascii_whitespace) {
        index += 1;
    }
    let negative = bytes.get(index) == Some(&b'-');
    if matches!(bytes.get(index), Some(b'-' | b'+')) {
        index += 1;
    }
    if bytes.get(index) == Some(&b'0')
        && matches!(bytes.get(index + 1), Some(b'x' | b'X'))
        && bytes.get(index + 2).is_some_and(u8::is_ascii_hexdigit)
    {
        index += 2;
    }
    let digit_start = index;
    let mut magnitude = 0_u128;
    let mut overflow = false;
    while let Some(digit) = bytes.get(index).and_then(|byte| match byte {
        b'0'..=b'9' => Some(u128::from(byte - b'0')),
        b'a'..=b'f' => Some(u128::from(byte - b'a') + 10),
        b'A'..=b'F' => Some(u128::from(byte - b'A') + 10),
        _ => None,
    }) {
        match magnitude
            .checked_mul(16)
            .and_then(|value| value.checked_add(digit))
            .filter(|&value| value <= C_ULONG_MAX)
        {
            Some(value) if !overflow => magnitude = value,
            _ => {
                magnitude = C_ULONG_MAX;
                overflow = true;
            }
        }
        index += 1;
    }
    let magnitude = magnitude as libc::c_ulong;
    let value = if negative && !overflow {
        magnitude.wrapping_neg()
    } else {
        magnitude
    };
    let value = (value & libc::c_ulong::from(u32::MAX)) as u32;
    (index != digit_start).then_some((value, index))
}

fn parse_linux_device_selection(bytes: &[u8]) -> Option<(u32, u32)> {
    let (vendor, index) = scanf_hex(bytes, 0)?;
    if bytes.get(index) != Some(&b':') {
        return None;
    }
    let (device, _) = scanf_hex(bytes, index + 1)?;
    Some((vendor, device))
}

fn linux_sort_enabled(instance: &LoaderInstance) -> bool {
    if !LINUX_SORT_PLATFORM_ENABLED {
        return false;
    }
    if std::env::var_os("VK_LOADER_DISABLE_SELECT")
        .is_some_and(|value| decimal_prefix_nonzero(value.as_encoded_bytes()))
    {
        return false;
    }
    instance.api_version >= vk::VK_API_VERSION_1_1
        || instance
            .enabled_extensions
            .contains_name(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
        || instance.active_icds().any(|(_, icd)| {
            icd.enabled_extensions
                .contains_name(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
        })
}

fn linux_device_type_priority(device_type: vk::VkPhysicalDeviceType) -> u8 {
    match device_type {
        vk::VkPhysicalDeviceType::DISCRETE_GPU => 10,
        vk::VkPhysicalDeviceType::INTEGRATED_GPU => 5,
        vk::VkPhysicalDeviceType::VIRTUAL_GPU => 3,
        vk::VkPhysicalDeviceType::OTHER => 2,
        vk::VkPhysicalDeviceType::CPU => 1,
        _ => 0,
    }
}

fn compare_linux_devices(
    left: &LinuxSortedDeviceInfo,
    right: &LinuxSortedDeviceInfo,
) -> core::cmp::Ordering {
    match (left.default_device, right.default_device) {
        (true, false) => return Ordering::Less,
        (false, true) => return Ordering::Greater,
        _ => {}
    }
    let type_order = linux_device_type_priority(right.device_type)
        .cmp(&linux_device_type_priority(left.device_type));
    if type_order != Ordering::Equal {
        return type_order;
    }
    match (left.pci, right.pci) {
        (Some(left), Some(right)) => {
            let order = left.cmp(&right);
            if order != Ordering::Equal {
                return order;
            }
        }
        (Some(_), None) => return Ordering::Less,
        (None, Some(_)) => return Ordering::Greater,
        (None, None) => {}
    }
    (left.device_id ^ left.vendor_id)
        .cmp(&(right.device_id ^ right.vendor_id))
        .then_with(|| left.original_order.cmp(&right.original_order))
}

fn heap_sort_by<T>(values: &mut [T], compare: impl Fn(&T, &T) -> core::cmp::Ordering) {
    fn sift_down<T>(
        values: &mut [T],
        mut root: usize,
        end: usize,
        compare: &impl Fn(&T, &T) -> core::cmp::Ordering,
    ) {
        loop {
            let left = root.saturating_mul(2).saturating_add(1);
            if left >= end {
                return;
            }
            let right = left + 1;
            let child = if right < end
                && compare(&values[left], &values[right]) == core::cmp::Ordering::Less
            {
                right
            } else {
                left
            };
            if compare(&values[root], &values[child]) != core::cmp::Ordering::Less {
                return;
            }
            values.swap(root, child);
            root = child;
        }
    }

    for root in (0..values.len() / 2).rev() {
        sift_down(values, root, values.len(), &compare);
    }
    for end in (1..values.len()).rev() {
        values.swap(0, end);
        sift_down(values, 0, end, &compare);
    }
}

unsafe fn icd_supports_device_extension(
    icd: &IcdInstance,
    physical_device: VkPhysicalDevice,
    name: &CStr,
) -> Result<bool, VkResult> {
    let Some(enumerate) = icd.dispatch.vkEnumerateDeviceExtensionProperties else {
        return Ok(false);
    };
    let mut count = 0;
    // Upstream intentionally ignores the driver's result here and uses the
    // returned count. GPU sorting is auxiliary and must not turn a device
    // extension enumeration error into a physical-device enumeration error.
    let _ = unsafe {
        enumerate(
            physical_device,
            core::ptr::null(),
            &raw mut count,
            core::ptr::null_mut(),
        )
    };
    let capacity = count as usize;
    if capacity == 0 {
        return Ok(false);
    }
    let mut properties = Vec::new();
    properties
        .try_reserve_exact(capacity)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    properties.resize(capacity, VkExtensionProperties::DEFAULT);
    // Match loader_linux.c: bound reads by the allocated count regardless of
    // the result or a driver's over-reported second-call count.
    let _ = unsafe {
        enumerate(
            physical_device,
            core::ptr::null(),
            &raw mut count,
            properties.as_mut_ptr(),
        )
    };
    Ok(properties[..(count as usize).min(capacity)]
        .iter()
        .any(|property| unsafe { CStr::from_ptr(property.extensionName.as_ptr()) == name }))
}

unsafe fn linux_sorted_device_info(
    instance: &LoaderInstance,
    device: NativePhysicalDevice,
    storage: *mut LinuxSortPropertyStorage,
) -> Result<LinuxSortedDeviceInfo, VkResult> {
    let icd = &instance.icds[device.icd_index];
    let Some(get_properties) = icd.dispatch.vkGetPhysicalDeviceProperties else {
        return Err(VkResult::ERROR_INITIALIZATION_FAILED);
    };
    unsafe { get_properties(device.handle, core::ptr::addr_of_mut!((*storage).basic)) };
    let device_type = unsafe { core::ptr::addr_of!((*storage).basic.deviceType).read() };
    let api_version = unsafe { core::ptr::addr_of!((*storage).basic.apiVersion).read() };
    let vendor_id = unsafe { core::ptr::addr_of!((*storage).basic.vendorID).read() };
    let device_id = unsafe { core::ptr::addr_of!((*storage).basic.deviceID).read() };
    let has_pci = unsafe {
        icd_supports_device_extension(icd, device.handle, vk::VK_EXT_PCI_BUS_INFO_EXTENSION_NAME)
    }?;
    let pci = if has_pci {
        unsafe {
            core::ptr::addr_of_mut!((*storage).properties2.sType)
                .write(vk::VkStructureType::PHYSICAL_DEVICE_PROPERTIES_2);
            core::ptr::addr_of_mut!((*storage).properties2.pNext)
                .write(core::ptr::addr_of_mut!((*storage).pci).cast());
            core::ptr::addr_of_mut!((*storage).pci)
                .write(vk::VkPhysicalDevicePCIBusInfoPropertiesEXT::DEFAULT);
        }
        let queried = if instance.api_version >= vk::VK_API_VERSION_1_1
            && api_version >= vk::VK_API_VERSION_1_1
        {
            icd.dispatch
                .vkGetPhysicalDeviceProperties2
                .map(|query| unsafe {
                    query(
                        device.handle,
                        core::ptr::addr_of_mut!((*storage).properties2),
                    );
                })
        } else {
            debug_assert_eq!(
                core::mem::size_of::<vk::VkPhysicalDeviceProperties2KHR<'_>>(),
                core::mem::size_of::<vk::VkPhysicalDeviceProperties2<'_>>()
            );
            debug_assert_eq!(
                core::mem::align_of::<vk::VkPhysicalDeviceProperties2KHR<'_>>(),
                core::mem::align_of::<vk::VkPhysicalDeviceProperties2<'_>>()
            );
            icd.dispatch
                .vkGetPhysicalDeviceProperties2KHR
                .map(|query| unsafe {
                    query(
                        device.handle,
                        core::ptr::addr_of_mut!((*storage).properties2).cast(),
                    );
                })
        };
        queried.map(|()| unsafe {
            (
                core::ptr::addr_of!((*storage).pci.pciDomain).read(),
                core::ptr::addr_of!((*storage).pci.pciBus).read(),
                core::ptr::addr_of!((*storage).pci.pciDevice).read(),
                core::ptr::addr_of!((*storage).pci.pciFunction).read(),
            )
        })
    } else {
        None
    };
    Ok(LinuxSortedDeviceInfo {
        device,
        device_type,
        vendor_id,
        device_id,
        pci,
        default_device: false,
        original_order: 0,
    })
}

fn selected_linux_device() -> Option<(u32, u32)> {
    let selection = std::env::var_os("VK_LOADER_DEVICE_SELECT")?;
    parse_linux_device_selection(selection.as_encoded_bytes())
}

#[cold]
#[inline(never)]
unsafe fn linux_sort_physical_devices(
    instance: &LoaderInstance,
    devices: &mut [NativePhysicalDevice],
) -> Result<(), VkResult> {
    let mut storage = Box::<LinuxSortPropertyStorage>::new_uninit();
    let storage = storage.as_mut_ptr();
    let mut sorted = Vec::new();
    sorted
        .try_reserve_exact(devices.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for (original_order, &device) in devices.iter().enumerate() {
        let mut info = unsafe { linux_sorted_device_info(instance, device, storage) }?;
        info.original_order = original_order;
        sorted.push(info);
    }
    if let Some((vendor_id, device_id)) = selected_linux_device()
        && let Some(selected) = sorted
            .iter_mut()
            .find(|device| device.vendor_id == vendor_id && device.device_id == device_id)
    {
        selected.default_device = true;
    }
    heap_sort_by(&mut sorted, compare_linux_devices);
    for (output, sorted) in devices.iter_mut().zip(sorted) {
        *output = sorted.device;
    }
    Ok(())
}

struct LinuxSortableGroup {
    group_index: usize,
    devices: Box<[LinuxSortedDeviceInfo]>,
    original_order: usize,
}

#[cold]
#[inline(never)]
unsafe fn linux_sort_physical_device_groups(
    instance: &LoaderInstance,
    mut groups: Vec<(usize, VkPhysicalDeviceGroupProperties<'static>)>,
) -> Result<Vec<(usize, VkPhysicalDeviceGroupProperties<'static>)>, VkResult> {
    let selected = selected_linux_device();
    let mut storage = Box::<LinuxSortPropertyStorage>::new_uninit();
    let storage = storage.as_mut_ptr();
    let mut sortable = Vec::new();
    sortable
        .try_reserve_exact(groups.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for (group_index, (icd_index, properties)) in groups.iter_mut().enumerate() {
        let device_count =
            (properties.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        properties.physicalDeviceCount = device_count as u32;
        let mut devices = Vec::new();
        devices
            .try_reserve_exact(device_count)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        for (device_order, &handle) in properties.physicalDevices[..device_count]
            .iter()
            .enumerate()
        {
            let mut info = unsafe {
                linux_sorted_device_info(
                    instance,
                    NativePhysicalDevice {
                        icd_index: *icd_index,
                        handle,
                    },
                    storage,
                )
            }?;
            info.original_order = device_order;
            devices.push(info);
        }
        if let Some((vendor_id, device_id)) = selected
            && let Some(selected) = devices
                .iter_mut()
                .find(|device| device.vendor_id == vendor_id && device.device_id == device_id)
        {
            selected.default_device = true;
        }
        heap_sort_by(&mut devices, compare_linux_devices);
        for (output, sorted) in properties.physicalDevices[..device_count]
            .iter_mut()
            .zip(&devices)
        {
            *output = sorted.device.handle;
        }
        sortable.push(LinuxSortableGroup {
            group_index,
            devices: devices.into_boxed_slice(),
            original_order: group_index,
        });
    }
    heap_sort_by(&mut sortable, |left, right| {
        match (left.devices.first(), right.devices.first()) {
            (Some(left), Some(right)) => compare_linux_devices(left, right),
            (Some(_), None) => core::cmp::Ordering::Less,
            (None, Some(_)) => core::cmp::Ordering::Greater,
            (None, None) => core::cmp::Ordering::Equal,
        }
        .then_with(|| left.original_order.cmp(&right.original_order))
    });
    let mut sorted = Vec::new();
    sorted
        .try_reserve_exact(sortable.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let group_count = sortable.len();
    for (output, group) in sorted.spare_capacity_mut().iter_mut().zip(sortable) {
        output.write(groups[group.group_index]);
    }
    // SAFETY: The loop initializes exactly one output for every sortable group.
    unsafe { sorted.set_len(group_count) };
    Ok(sorted)
}

#[inline(never)]
unsafe fn discover_active_physical_devices(
    instance: &LoaderInstance,
) -> Result<Box<[NativePhysicalDevice]>, VkResult> {
    let devices = unsafe { discover_all_physical_devices(instance) }?;
    let Some(configurations) = instance.device_configurations.as_deref() else {
        return Ok(devices);
    };

    emit_instance_loader_message(
        instance,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        "Selecting and ordering VkPhysicalDevices to match the loader settings device configurations list",
    );

    let mut ordered = Vec::new();
    ordered
        .try_reserve_exact(configurations.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut query = Box::<DeviceConfigurationProperties>::new_uninit();
    let query_pointer = query.as_mut_ptr();
    for configuration in configurations {
        for device in &devices {
            let icd = &instance.icds[device.icd_index];
            let Some(get_properties2) = icd.dispatch.vkGetPhysicalDeviceProperties2 else {
                continue;
            };
            unsafe {
                core::ptr::addr_of_mut!((*query_pointer).properties.sType)
                    .write(vk::VkStructureType::PHYSICAL_DEVICE_PROPERTIES_2);
                core::ptr::addr_of_mut!((*query_pointer).properties.pNext)
                    .write(core::ptr::addr_of_mut!((*query_pointer).identifiers).cast());
                core::ptr::addr_of_mut!((*query_pointer).identifiers.sType)
                    .write(vk::VkStructureType::PHYSICAL_DEVICE_ID_PROPERTIES);
                core::ptr::addr_of_mut!((*query_pointer).identifiers.pNext)
                    .write(core::ptr::null_mut());
                core::ptr::addr_of_mut!((*query_pointer).driver)
                    .write(vk::VkPhysicalDeviceDriverProperties::DEFAULT);
                get_properties2(
                    device.handle,
                    core::ptr::addr_of_mut!((*query_pointer).properties),
                );
            }
            let api_version = unsafe {
                core::ptr::addr_of!((*query_pointer).properties.properties.apiVersion).read()
            };
            let driver_version = unsafe {
                core::ptr::addr_of!((*query_pointer).properties.properties.driverVersion).read()
            };
            let device_uuid =
                unsafe { core::ptr::addr_of!((*query_pointer).identifiers.deviceUUID).read() };
            let driver_uuid =
                unsafe { core::ptr::addr_of!((*query_pointer).identifiers.driverUUID).read() };
            let supports_driver_properties = api_version >= vk::VK_API_VERSION_1_2
                || unsafe {
                    icd_supports_device_extension(
                        icd,
                        device.handle,
                        vk::VK_KHR_DRIVER_PROPERTIES_EXTENSION_NAME,
                    )
                }?;
            if supports_driver_properties {
                unsafe {
                    core::ptr::addr_of_mut!((*query_pointer).identifiers.pNext)
                        .write(core::ptr::addr_of_mut!((*query_pointer).driver).cast());
                    get_properties2(
                        device.handle,
                        core::ptr::addr_of_mut!((*query_pointer).properties),
                    );
                }
            }
            if api_version >= vk::VK_API_VERSION_1_1
                && driver_version == configuration.driver_version
                && device_uuid == configuration.device_uuid
                && driver_uuid == configuration.driver_uuid
            {
                let properties =
                    unsafe { core::ptr::addr_of!((*query_pointer).properties.properties).read() };
                let device_name =
                    unsafe { CStr::from_ptr(properties.deviceName.as_ptr()) }.to_string_lossy();
                let index = ordered.len();
                let detail = if supports_driver_properties {
                    let driver_name = unsafe {
                        CStr::from_ptr(
                            core::ptr::addr_of!((*query_pointer).driver.driverName)
                                .cast::<c_char>(),
                        )
                    }
                    .to_string_lossy();
                    format!(
                        "pPhysicalDevices array index {index} is set to \"{device_name}\" ({driver_name}, version {driver_version}) "
                    )
                } else {
                    format!(
                        "pPhysicalDevices array index {index} is set to \"{device_name}\" (driver version {driver_version}) "
                    )
                };
                emit_instance_loader_message(
                    instance,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                    detail,
                );
                ordered.push(*device);
                break;
            }
        }
    }
    if ordered.is_empty() {
        Err(VkResult::ERROR_INITIALIZATION_FAILED)
    } else {
        Ok(ordered.into_boxed_slice())
    }
}

#[cold]
fn emit_instance_loader_message(
    instance: &LoaderInstance,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: impl AsRef<str>,
) {
    let message = message.as_ref();
    let (filter, label) = if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR {
        ("error", "ERROR")
    } else if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING {
        ("warn", "WARNING")
    } else if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO {
        ("info", "INFO")
    } else {
        ("debug", "DEBUG")
    };
    platform::write_loader_log(filter, label, format_args!("{message}"));
    let Ok(message) = alloc::ffi::CString::new(message) else {
        return;
    };
    instance.submit_loader_message(
        severity,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        &message,
    );
}

#[cold]
fn emit_instance_category_message(
    instance: &LoaderInstance,
    category_filters: &[&str],
    category_label: &str,
    message: impl AsRef<str>,
) {
    let message = message.as_ref();
    platform::write_loader_category_log_any(
        category_filters,
        category_label,
        format_args!("{message}"),
    );
    let Ok(message) = alloc::ffi::CString::new(message) else {
        return;
    };
    instance.submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        &message,
    );
}

struct IcdPhysicalDevices {
    storage: Box<[MaybeUninit<VkPhysicalDevice>]>,
    len: usize,
}

impl IcdPhysicalDevices {
    fn iter(&self) -> impl Iterator<Item = VkPhysicalDevice> + '_ {
        self.storage[..self.len].iter().map(|device| {
            // SAFETY: The ICD reported these leading elements as written.
            unsafe { device.assume_init() }
        })
    }
}

#[cold]
#[inline(never)]
unsafe fn enumerate_icd_physical_devices(
    instance: &IcdInstance,
) -> Result<IcdPhysicalDevices, VkResult> {
    debug_assert!(instance.dispatch.vkEnumeratePhysicalDevices.is_some());
    let enumerate: PFN_vkEnumeratePhysicalDevices = instance
        .dispatch
        .vkEnumeratePhysicalDevices
        .ok_or(VkResult::ERROR_INITIALIZATION_FAILED)?;
    let mut count = 0;
    // SAFETY: Count points to writable local storage.
    let result = unsafe { enumerate(instance.handle, &raw mut count, core::ptr::null_mut()) };
    if result != VkResult::SUCCESS {
        return Err(result);
    }
    let count = count as usize;
    let mut storage = Box::<[VkPhysicalDevice]>::new_uninit_slice(count);
    let mut returned_count = count.min(u32::MAX as usize) as u32;
    // SAFETY: `storage` has `count` writable elements.
    let result = unsafe {
        enumerate(
            instance.handle,
            &raw mut returned_count,
            storage.as_mut_ptr().cast(),
        )
    };
    if result != VkResult::SUCCESS && result != VkResult::INCOMPLETE {
        return Err(result);
    }
    Ok(IcdPhysicalDevices {
        storage,
        len: (returned_count as usize).min(count),
    })
}

/// Enumerates loader-provided instance extensions.
///
/// # Safety
///
/// `property_count` must be writable. If non-null, `properties` must reference
/// the number of elements supplied through `property_count`.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumerateInstanceExtensionProperties(
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    platform::initialize_loader();
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    unsafe { pre_instance::enumerate_extension_properties(layer_name, property_count, properties) }
}

/// Enumerates available explicit and implicit layers.
///
/// # Safety
///
/// `property_count` must be writable. If non-null, `properties` must reference
/// the number of elements supplied through `property_count`.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumerateInstanceLayerProperties(
    property_count: *mut u32,
    properties: *mut VkLayerProperties,
) -> VkResult {
    platform::initialize_loader();
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    unsafe { pre_instance::enumerate_layer_properties(property_count, properties) }
}

/// Reports the highest Vulkan core version supported by this loader.
///
/// # Safety
///
/// `api_version` must point to writable `u32` storage.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumerateInstanceVersion(api_version: *mut u32) -> VkResult {
    platform::initialize_loader();
    if api_version.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    unsafe { pre_instance::enumerate_version(&mut *api_version) }
}

/// Validates a bounded UTF-8-like loader string using Vulkan-Loader's legacy
/// bitmask contract. This helper is exported by upstream test-enabled builds.
///
/// # Safety
///
/// `utf8` must be null or readable through the first terminator or
/// `max_length + 1` bytes, matching the upstream C contract.
// Upstream exposes this test helper from ELF builds through
// `TEST_FUNCTION_EXPORT`, but deliberately omits it from vulkan-1.def.
#[cfg_attr(not(windows), unsafe(no_mangle))]
pub unsafe extern "C" fn vk_string_validate(
    max_length: libc::c_int,
    utf8: *const c_char,
) -> vk::VkFlags {
    const LENGTH: vk::VkFlags = 0x1;
    const BAD_DATA: vk::VkFlags = 0x2;
    const NULL_PTR: vk::VkFlags = 0x4;

    if utf8.is_null() {
        return NULL_PTR;
    }
    let utf8 = utf8.cast::<u8>();
    let mut result = 0;
    let mut index = 0;
    while index <= max_length {
        debug_assert!(index >= 0);
        let offset = index as usize;
        let byte = unsafe { utf8.add(offset).read() };
        if byte == 0 {
            break;
        }
        if index == max_length {
            result |= LENGTH;
            break;
        }
        let continuation_count = if (0x20..0x7f).contains(&byte) {
            0
        } else if byte & 0xe0 == 0xc0 {
            1
        } else if byte & 0xf0 == 0xe0 {
            2
        } else if byte & 0xf8 == 0xf0 {
            3
        } else {
            result = BAD_DATA;
            0
        };
        for _ in 0..continuation_count {
            if index >= max_length {
                break;
            }
            index += 1;
            if index == max_length {
                result |= LENGTH;
                break;
            }
            debug_assert!(index >= 0);
            let offset = index as usize;
            let continuation = unsafe { utf8.add(offset).read() };
            if continuation == 0 {
                return result | BAD_DATA;
            }
            if continuation & 0xc0 != 0x80 {
                result |= BAD_DATA;
            }
        }
        index += 1;
    }
    result
}

/// Resolves a Vulkan command for an instance.
///
/// # Safety
///
/// `p_name` must point to a valid, NUL-terminated C string.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkGetInstanceProcAddr(
    instance: VkInstance,
    p_name: *const core::ffi::c_char,
) -> PFN_vkVoidFunction {
    if p_name.is_null() {
        return None;
    }

    // SAFETY: Required by this function's public contract.
    let name = unsafe { CStr::from_ptr(p_name) };
    if instance == VkInstance::NULL {
        global_proc_addr(name)
    } else {
        // SAFETY: A non-null instance supplied to GIPA must be a live loader instance.
        let loader = unsafe { LoaderInstance::from_handle(instance) }.unwrap_or_else(|| {
            fatal_loader_error(
                c"vkGetInstanceProcAddr: Invalid instance [VUID-vkGetInstanceProcAddr-instance-parameter]",
            )
        });
        let Some(lookup) = command_lookup(name) else {
            return unknown::physical_device_proc_addr(loader, name, true)
                .or_else(|| unknown::device_proc_addr(loader, name, true));
        };
        if name == c"vkGetInstanceProcAddr" {
            Some(erase_function(
                vkGetInstanceProcAddr as vk::PFN_vkGetInstanceProcAddr,
            ))
        } else if lookup.scope == CommandScope::Global {
            (loader.api_version < VK_API_VERSION_1_3)
                .then(|| global_proc_addr(name))
                .flatten()
        } else {
            let available = command_core_level(lookup.id) != 0
                || command_has_device_extension_provider(lookup.id)
                || command_has_enabled_instance_extension(lookup.id, &loader.enabled_extensions);
            if !available {
                return None;
            }
            exported_proc_addr(lookup.id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    static DESTROY_SAW_REGISTERED_DEVICE: AtomicBool = AtomicBool::new(false);
    static DESTROY_DISPATCH_KEY: AtomicUsize = AtomicUsize::new(0);

    #[repr(C)]
    struct FakeNativeDevice {
        dispatch: *const LayerDeviceDispatchTable,
    }

    unsafe extern "system" fn fake_destroy_device(
        device: VkDevice,
        _allocator: *const VkAllocationCallbacks<'_>,
    ) {
        let registered = unsafe { LoaderDevice::from_handle(device) }.is_some();
        DESTROY_SAW_REGISTERED_DEVICE.store(registered, Ordering::SeqCst);
        let dispatch = unsafe { device.0.cast::<*const LayerDeviceDispatchTable>().read() };
        DESTROY_DISPATCH_KEY.store(dispatch as usize, Ordering::SeqCst);
        // A native destroy call is permitted to release the dispatchable's
        // storage before returning. Simulate that by making its first word
        // unreadable to loader handle lookup without actually freeing the
        // test's stack allocation.
        unsafe {
            device
                .0
                .cast::<*const LayerDeviceDispatchTable>()
                .write(core::ptr::null());
        };
    }

    unsafe extern "system" fn fake_get_device_proc_addr(
        _device: VkDevice,
        name: *const c_char,
    ) -> PFN_vkVoidFunction {
        if !name.is_null() && unsafe { CStr::from_ptr(name) } == c"vkDestroyDevice" {
            Some(erase_function(fake_destroy_device as PFN_vkDestroyDevice))
        } else {
            None
        }
    }

    #[test]
    fn null_instance_resolves_the_global_commands() {
        for name in [
            "vkCreateInstance",
            "vkEnumerateInstanceExtensionProperties",
            "vkEnumerateInstanceLayerProperties",
            "vkEnumerateInstanceVersion",
            "vkGetInstanceProcAddr",
        ] {
            let name = CString::new(name).unwrap();
            // SAFETY: `name` is a live, NUL-terminated C string.
            let address = unsafe { vkGetInstanceProcAddr(VkInstance::NULL, name.as_ptr()) };
            assert!(address.is_some(), "{name:?} was not resolved");
        }
    }

    #[test]
    fn device_group_chain_translation_is_scoped_and_restores_the_caller_chain() {
        let source_devices = [VkPhysicalDevice(0x1000_usize as *mut c_void)];
        let group = VkDeviceGroupDeviceCreateInfo {
            physicalDeviceCount: source_devices.len() as u32,
            pPhysicalDevices: source_devices.as_ptr(),
            ..VkDeviceGroupDeviceCreateInfo::DEFAULT
        };
        let mut prefix = vk::VkBaseInStructure {
            sType: VkStructureType::PHYSICAL_DEVICE_FEATURES_2,
            pNext: core::ptr::from_ref(&group).cast(),
            ..vk::VkBaseInStructure::DEFAULT
        };
        let original_group = prefix.pNext;
        let mut create_info = VkDeviceCreateInfo {
            pNext: core::ptr::from_mut(&mut prefix).cast(),
            ..VkDeviceCreateInfo::DEFAULT
        };

        let patch = unsafe {
            translate_device_group_chain(&mut create_info, |handle| {
                Some(VkPhysicalDevice(
                    (handle.0 as usize + 0x1000) as *mut c_void,
                ))
            })
        }
        .unwrap()
        .unwrap();

        assert_ne!(prefix.pNext, original_group);
        let translated = unsafe { &*prefix.pNext.cast::<VkDeviceGroupDeviceCreateInfo<'_>>() };
        assert_eq!(translated.physicalDeviceCount, 1);
        assert_eq!(
            unsafe { translated.pPhysicalDevices.read() }.0 as usize,
            0x2000
        );

        drop(patch);
        assert_eq!(prefix.pNext, original_group);
    }

    #[test]
    fn device_record_outlives_native_destroy_and_does_not_reread_dead_handle() {
        DESTROY_SAW_REGISTERED_DEVICE.store(false, Ordering::SeqCst);
        DESTROY_DISPATCH_KEY.store(0, Ordering::SeqCst);
        let instance = LoaderInstance::new(
            VK_API_VERSION_1_0,
            ExtensionSet::default(),
            Box::default(),
            layer::ActiveLayers {
                loaded: Box::default(),
                reported: Box::default(),
                requested: Box::default(),
            },
            None,
            core::ptr::null(),
        );
        let mut native = FakeNativeDevice {
            dispatch: core::ptr::null(),
        };
        let handle = VkDevice(core::ptr::from_mut(&mut native).cast());
        // SAFETY: `native` is writable for the test lifetime, the fake resolver
        // targets that storage, and `instance` outlives the registered device.
        let device = unsafe {
            LoaderDevice::new(
                handle,
                fake_get_device_proc_addr,
                core::ptr::from_ref(instance.as_ref()),
                0,
                VK_API_VERSION_1_0,
                false,
                ExtensionSet::default(),
            )
        };
        let handle = LoaderDevice::register(device);
        let dispatch_key = native.dispatch as usize;
        // SAFETY: The test has exclusive creation-time access and the fake
        // resolver represents the completed one-element device chain.
        unsafe {
            LoaderDevice::from_dispatch_key_mut(dispatch_key)
                .unwrap()
                .set_chain(handle, fake_get_device_proc_addr);
        };

        unsafe { vkDestroyDevice(handle, core::ptr::null()) };

        assert!(DESTROY_SAW_REGISTERED_DEVICE.load(Ordering::SeqCst));
        let dispatch = DESTROY_DISPATCH_KEY.load(Ordering::SeqCst);
        assert_ne!(dispatch, 0);
        assert!(
            LoaderDevice::take_dispatch(dispatch as *const LayerDeviceDispatchTable).is_none(),
            "the public trampoline must remove the saved registry entry after native teardown"
        );
    }

    #[test]
    fn null_instance_rejects_non_global_commands() {
        let name = CString::new("vkEnumeratePhysicalDevices").unwrap();
        // SAFETY: `name` is a live, NUL-terminated C string.
        assert!(unsafe { vkGetInstanceProcAddr(VkInstance::NULL, name.as_ptr()) }.is_none());
    }

    #[test]
    fn generated_command_table_classifies_dispatch() {
        assert_eq!(
            command_lookup(c"vkCreateInstance").unwrap().scope,
            CommandScope::Global
        );
        assert_eq!(
            command_lookup(c"vkDestroyInstance").unwrap().scope,
            CommandScope::Instance
        );
        assert_eq!(
            command_lookup(c"vkQueueSubmit").unwrap().scope,
            CommandScope::Device
        );
        assert!(command_lookup(c"vkTrimCommandPoolKHR").is_some());
        assert!(command_lookup(c"vkNotACommand").is_none());
        assert!(command_lookup(c"CreateInstance").is_none());
        assert!(command_lookup(c"vCreateInstance").is_none());
        assert!(exported_proc_addr(command_lookup(c"vkQueueSubmit").unwrap().id).is_some());
        assert!(core::hint::black_box(COMMAND_COUNT) > 800);
        assert!(core::hint::black_box(COMMAND_MAX_DISPLACEMENT) < u16::MAX);
    }

    #[test]
    fn generated_surface_commands_separate_trampolines_and_terminators() {
        for name in [c"vkCreateHeadlessSurfaceEXT", c"vkDestroySurfaceKHR"] {
            let id = command_lookup(name).unwrap().id;
            let trampoline = exported_proc_addr(id).map(|function| function as usize);
            let terminator = instance_terminator_proc_addr(id).map(|function| function as usize);
            assert!(
                trampoline.is_some(),
                "missing public trampoline for {name:?}"
            );
            assert!(
                terminator.is_some(),
                "missing instance terminator for {name:?}"
            );
            assert_ne!(trampoline, terminator, "{name:?} bypasses the layer chain");
        }
    }

    #[test]
    fn generated_command_lookup_is_exhaustive_and_exact() {
        let mut populated = 0;
        for record in COMMAND_TABLE.iter().filter(|record| record.id != u16::MAX) {
            let start = usize::from(record.name_offset);
            let end = start + usize::from(record.name_len);
            let suffix = &COMMAND_NAMES[start..end];
            let mut name = Vec::with_capacity(suffix.len() + 3);
            name.extend_from_slice(b"vk");
            name.extend_from_slice(suffix);
            name.push(0);
            let name = CStr::from_bytes_with_nul(&name).unwrap();
            assert_eq!(
                command_lookup(name),
                Some(CommandLookup {
                    id: record.id,
                    scope: record.scope,
                })
            );
            populated += 1;
        }
        assert_eq!(populated, COMMAND_COUNT);

        for name in [
            c"",
            c"vk",
            c"v",
            c"CreateBuffer",
            c"glCreateBuffer",
            c"xkCreateBuffer",
            c"vkCreateBuffe",
            c"vkDestroyInstanc",
            c"vkCreateBufferX",
            c"vkDestroyInstanceX",
            c"vkcreatebuffer",
            c"VKCREATEBUFFER",
            c"vkCREATEBuffer",
            c"vkGetInstanceProcAddx",
            c"vkCmdSetViewpost",
            c"vkCmdBeginRenderPasS",
            c"vkCreateBuffre",
            c"vkTransitionImageLayous",
            c"vkZzTs0sr",
            c"vkF5TyK",
        ] {
            assert!(command_lookup(name).is_none(), "resolved {name:?}");
        }
    }

    #[test]
    fn linux_sort_environment_numbers_match_c_prefix_parsing() {
        for value in [b"1tail".as_slice(), b"  +2", b"-3", b"0009x"] {
            assert!(decimal_prefix_nonzero(value), "rejected {value:?}");
        }
        for value in [b"".as_slice(), b"  ", b"+", b"-0tail", b"word1"] {
            assert!(!decimal_prefix_nonzero(value), "accepted {value:?}");
        }
        assert!(!decimal_prefix_nonzero(b"4294967296"));
        assert!(!decimal_prefix_nonzero(b"-4294967296"));
        assert!(decimal_prefix_nonzero(b"18446744073709551616"));
        assert_eq!(
            decimal_prefix_nonzero(b"-18446744073709551616"),
            core::mem::size_of::<libc::c_long>() == 4
        );
    }

    #[test]
    fn linux_sort_forces_properties_extension_for_either_vulkan_1_0_side() {
        assert!(linux_sort_requires_properties_extension(
            vk::VK_API_VERSION_1_0,
            vk::VK_API_VERSION_1_1
        ));
        assert!(linux_sort_requires_properties_extension(
            vk::VK_API_VERSION_1_1,
            vk::VK_API_VERSION_1_0
        ));
        assert!(!linux_sort_requires_properties_extension(
            vk::VK_API_VERSION_1_1,
            vk::VK_API_VERSION_1_1
        ));
    }

    #[test]
    fn icd_create_version_matches_upstream_device_configuration_policy() {
        assert_eq!(
            icd_create_application_api_version(
                vk::VK_API_VERSION_1_2,
                vk::VK_MAKE_API_VERSION(0, 1, 0, 7),
                false
            ),
            Some(vk::VK_MAKE_API_VERSION(0, 1, 0, 7))
        );
        assert_eq!(
            icd_create_application_api_version(
                vk::VK_API_VERSION_1_0,
                vk::VK_API_VERSION_1_2,
                true
            ),
            Some(vk::VK_API_VERSION_1_1)
        );
        assert_eq!(
            icd_create_application_api_version(
                vk::VK_API_VERSION_1_0,
                vk::VK_API_VERSION_1_2,
                false
            ),
            None
        );
    }

    #[test]
    fn direct_driver_scan_only_propagates_host_oom() {
        assert_eq!(
            fatal_direct_driver_scan_error(VkResult::ERROR_OUT_OF_HOST_MEMORY),
            Some(VkResult::ERROR_OUT_OF_HOST_MEMORY)
        );
        for result in [
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_INCOMPATIBLE_DRIVER,
            VkResult::ERROR_UNKNOWN,
        ] {
            assert_eq!(fatal_direct_driver_scan_error(result), None);
        }
    }

    #[test]
    fn linux_device_selection_matches_scanf_hex_contract() {
        assert_eq!(
            parse_linux_device_selection(b"10de:2684"),
            Some((0x10de, 0x2684))
        );
        assert_eq!(
            parse_linux_device_selection(b" +0x10DE: 0X2684 trailing"),
            Some((0x10de, 0x2684))
        );
        assert_eq!(parse_linux_device_selection(b"-1:+2"), Some((u32::MAX, 2)));
        assert_eq!(parse_linux_device_selection(b"10de :2684"), None);
        assert_eq!(parse_linux_device_selection(b"10de:"), None);
        assert_eq!(parse_linux_device_selection(b"device:2684"), None);
        assert_eq!(
            parse_linux_device_selection(b"100000000:2"),
            Some((
                if libc::c_ulong::BITS == 32 {
                    u32::MAX
                } else {
                    0
                },
                2
            ))
        );
        assert_eq!(
            parse_linux_device_selection(b"ffffffffffffffffffffffff:2"),
            Some((u32::MAX, 2))
        );
        assert_eq!(
            parse_linux_device_selection(b"-ffffffffffffffffffffffff:2"),
            Some((u32::MAX, 2))
        );
    }

    #[test]
    fn generated_wsi_filter_matches_compiled_backends() {
        #[cfg(not(feature = "wsi-directfb"))]
        assert!(!wsi_instance_extension_supported(
            vk::VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME
        ));
        #[cfg(feature = "wsi-directfb")]
        assert!(wsi_instance_extension_supported(
            vk::VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME
        ));
        #[cfg(all(
            feature = "wsi-xcb",
            any(
                target_os = "linux",
                target_os = "freebsd",
                target_os = "openbsd",
                target_os = "netbsd",
                target_os = "dragonfly",
                target_os = "hurd",
                target_os = "cygwin"
            )
        ))]
        assert!(wsi_instance_extension_supported(
            vk::VK_KHR_XCB_SURFACE_EXTENSION_NAME
        ));
    }
}
