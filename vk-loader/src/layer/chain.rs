//! Layer chaining and enumeration entry points.

use crate::{
    CommandScope, LayerDeviceDispatchTable, allocation,
    device::LoaderDevice,
    erase_function,
    instance::{LoaderInstance, LoaderPhysicalDevice},
    pending,
    platform::LogFilter,
    unknown,
};

use super::{
    ActiveLayerProperty, CStr, CString, DeviceCreateSentinel, GetPhysicalDeviceProcAddr,
    LayerDeviceCallbacks, LayerDeviceCreateInfo, LayerDeviceCreateInfoUnion, LayerDeviceLink,
    LayerExtension, LayerFunction, LayerInstanceCreateInfo, LayerInstanceCreateInfoUnion,
    LayerInstanceLink, LayerPointer, PFN_vkEnumerateDeviceExtensionProperties,
    PFN_vkGetDeviceProcAddr, PFN_vkGetInstanceProcAddr, PFN_vkVoidFunction, STACK_LAYER_LINKS,
    ScratchArray, VkExtensionProperties, VkInstanceCreateInfo, VkResult, VkStructureType,
    available_layer_mask, c_char, c_void, discover_layers, emit_global_layer_search_diagnostics,
    fatal_layer_policy, forced_disabled, forced_enabled, ptr,
};

/// Returns whether deprecated device-layer names differ from instance names.
///
/// # Safety
///
/// The device create info must satisfy Vulkan's string-array contract.
// Vulkan-Loader intentionally reads these deprecated fields to diagnose legacy
// device-layer usage; suppress deprecation only for this compatibility check.
pub(crate) unsafe fn has_mismatched_device_layers(
    instance_names: &[CString],
    create_info: &vk::VkDeviceCreateInfo<'_>,
) -> bool {
    if create_info.enabledLayerCount == 0 || create_info.ppEnabledLayerNames.is_null() {
        return false;
    }
    let count = create_info.enabledLayerCount as usize;
    if count != instance_names.len() {
        return true;
    }
    (0..count).any(|index| {
        // SAFETY: The caller guarantees a live array of NUL-terminated names.
        let name = unsafe { create_info.ppEnabledLayerNames.add(index).read() };
        name.is_null()
            // SAFETY: Non-null enabled layer names are NUL-terminated.
            || unsafe { CStr::from_ptr(name) } != instance_names[index].as_c_str()
    })
}

pub(crate) unsafe extern "system" fn create_instance_terminator(
    create_info: *const VkInstanceCreateInfo<'_>,
    allocator: *const vk::VkAllocationCallbacks<'_>,
    instance: *mut vk::VkInstance,
) -> VkResult {
    if instance.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let pending = pending::instance();
    if pending == vk::VkInstance::NULL {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let returned = unsafe { instance.read() };
    if returned == vk::VkInstance::NULL {
        fatal_layer_policy(
            "terminator_CreateInstance: Loader instance pointer null encountered.  Possibly set by active layer. (Policy #LLP_LAYER_21)",
        );
    }
    let magic = unsafe { LoaderInstance::internal_magic(returned) }.unwrap_or(0);
    if unsafe { LoaderInstance::from_internal_handle(returned) }.is_none() {
        let pointer = LayerPointer(returned.0);
        fatal_layer_policy(format_args!(
            "terminator_CreateInstance: Instance pointer ({pointer}) has invalid MAGIC value 0x{magic:08x}. Instance value possibly corrupted by active layer (Policy #LLP_LAYER_21).  ",
        ));
    }
    if create_info.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The active layer chain retains readable create info for this call.
    let create_info = unsafe { &*create_info };
    let Some(loader) = (unsafe { LoaderInstance::from_internal_handle_mut(pending) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    // SAFETY: The layer chain retains the effective create info and allocator
    // for this synchronous call, and `loader` is the pending instance box.
    let result = unsafe { crate::create_pending_icd_instances(loader, create_info, allocator) };
    if result != VkResult::SUCCESS {
        return result;
    }
    // SAFETY: The layer supplied the output pointer from the live create call.
    unsafe { instance.write(pending) };
    VkResult::SUCCESS
}

pub(super) unsafe extern "system" fn set_instance_loader_data(
    instance: vk::VkInstance,
    object: *mut c_void,
) -> VkResult {
    if object.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The callback is installed only for this loader's live instance.
    let Some(instance) = (unsafe { LoaderInstance::from_internal_handle(instance) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    // SAFETY: The layer contract requires `object` to identify writable
    // dispatchable storage whose first word is the dispatch pointer.
    unsafe {
        object
            .cast::<*const crate::LayerInstanceDispatchTable>()
            .write(instance.dispatch());
    };
    VkResult::SUCCESS
}

pub(super) unsafe extern "system" fn layer_create_device_callback(
    instance: vk::VkInstance,
    physical_device: vk::VkPhysicalDevice,
    create_info: *const vk::VkDeviceCreateInfo<'_>,
    allocator: *const vk::VkAllocationCallbacks<'_>,
    device: *mut vk::VkDevice,
    layer_gipa: PFN_vkGetInstanceProcAddr,
    next_gdpa: *mut PFN_vkGetDeviceProcAddr,
) -> VkResult {
    if create_info.is_null() || device.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The active layer chain retains readable create info for this call.
    let create_info = unsafe { &*create_info };
    let loader = unsafe {
        LoaderInstance::from_handle(instance)
            .or_else(|| LoaderInstance::from_internal_handle(instance))
    };
    let Some(loader) = loader else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let Some(calling_index) = loader
        .layers
        .iter()
        .position(|layer| layer.get_instance_proc_addr as usize == layer_gipa as usize)
    else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let first = calling_index + 1;
    if !next_gdpa.is_null() {
        unsafe {
            next_gdpa.write(loader.layers.get(first).map_or(
                terminator_get_device_proc_addr as PFN_vkGetDeviceProcAddr,
                |layer| layer.get_device_proc_addr,
            ));
        };
    }
    unsafe {
        create_device_chain_from(
            loader,
            physical_device,
            create_info,
            allocator,
            device,
            first,
        )
    }
}

pub(super) unsafe extern "system" fn layer_destroy_device_callback(
    device: vk::VkDevice,
    allocator: *const vk::VkAllocationCallbacks<'_>,
    destroy: vk::PFN_vkDestroyDevice,
) {
    if device == vk::VkDevice::NULL {
        return;
    }
    // Capture the stable loader dispatch key before the lower chain destroys
    // the dispatchable object. This mirrors `loader_layer_destroy_device`.
    let dispatch = unsafe { crate::device_dispatch(device.0.cast()) }.map(core::ptr::from_ref);
    unsafe { destroy(device, allocator) };
    if let Some(dispatch) = dispatch {
        drop(LoaderDevice::take_dispatch(dispatch));
    }
}

pub(crate) unsafe extern "system" fn terminator_get_instance_proc_addr(
    instance: vk::VkInstance,
    name: *const c_char,
) -> PFN_vkVoidFunction {
    if name.is_null() {
        return None;
    }
    // SAFETY: The layer ABI requires a live, NUL-terminated command name.
    let name = unsafe { CStr::from_ptr(name) };
    match name.to_bytes() {
        b"vkCreateInstance" => Some(erase_function(
            create_instance_terminator as vk::PFN_vkCreateInstance,
        )),
        b"vkGetInstanceProcAddr" => Some(erase_function(
            terminator_get_instance_proc_addr as PFN_vkGetInstanceProcAddr,
        )),
        b"vk_layerGetPhysicalDeviceProcAddr" => Some(erase_function(
            terminator_get_physical_device_proc_addr as GetPhysicalDeviceProcAddr,
        )),
        b"vkCreateDevice" => Some(erase_function(
            crate::create_device_terminator as vk::PFN_vkCreateDevice,
        )),
        b"vkDestroyInstance" => Some(erase_function(
            crate::destroy_instance_terminator as vk::PFN_vkDestroyInstance,
        )),
        b"vkEnumeratePhysicalDevices" => Some(erase_function(
            crate::terminator_enumerate_physical_devices as vk::PFN_vkEnumeratePhysicalDevices,
        )),
        b"vkEnumeratePhysicalDeviceGroups" => Some(erase_function(
            crate::terminator_enumerate_physical_device_groups
                as vk::PFN_vkEnumeratePhysicalDeviceGroups,
        )),
        b"vkEnumeratePhysicalDeviceGroupsKHR" => Some(erase_function(
            crate::terminator_enumerate_physical_device_groups_khr
                as vk::PFN_vkEnumeratePhysicalDeviceGroupsKHR,
        )),
        b"vkEnumerateDeviceLayerProperties" => Some(erase_function(
            terminator_enumerate_device_layer_properties
                as vk::PFN_vkEnumerateDeviceLayerProperties,
        )),
        b"vkEnumerateDeviceExtensionProperties" => Some(erase_function(
            terminator_enumerate_device_extension_properties
                as vk::PFN_vkEnumerateDeviceExtensionProperties,
        )),
        b"vkCreateDebugUtilsMessengerEXT" => Some(erase_function(
            crate::debug::messenger::terminator_create_debug_utils_messenger
                as vk::PFN_vkCreateDebugUtilsMessengerEXT,
        )),
        b"vkCreateDebugReportCallbackEXT" => Some(erase_function(
            crate::debug::messenger::terminator_create_debug_report_callback
                as vk::PFN_vkCreateDebugReportCallbackEXT,
        )),
        b"vkDestroyDebugUtilsMessengerEXT" => Some(erase_function(
            crate::debug::messenger::terminator_destroy_debug_utils_messenger
                as vk::PFN_vkDestroyDebugUtilsMessengerEXT,
        )),
        b"vkDestroyDebugReportCallbackEXT" => Some(erase_function(
            crate::debug::messenger::terminator_destroy_debug_report_callback
                as vk::PFN_vkDestroyDebugReportCallbackEXT,
        )),
        b"vkSubmitDebugUtilsMessageEXT" => Some(erase_function(
            crate::debug::messenger::terminator_submit_debug_utils_message
                as vk::PFN_vkSubmitDebugUtilsMessageEXT,
        )),
        b"vkDebugReportMessageEXT" => Some(erase_function(
            crate::debug::messenger::terminator_debug_report_message
                as vk::PFN_vkDebugReportMessageEXT,
        )),
        _ if instance == vk::VkInstance::NULL => crate::global_proc_addr(name),
        _ => {
            if let Some(lookup) = crate::command_lookup(name) {
                return crate::instance_terminator_proc_addr(lookup.id)
                    .or_else(|| crate::physical_device_terminator_proc_addr(lookup.id))
                    .or_else(|| crate::exported_proc_addr(lookup.id));
            }
            // SAFETY: During vkCreateInstance the not-yet-registered loader
            // handle is passed down the layer chain; afterwards normal
            // dispatch-table lookup identifies the registered instance.
            let loader = unsafe {
                LoaderInstance::from_handle(instance)
                    .or_else(|| LoaderInstance::from_internal_handle(instance))
            };
            let address = loader.and_then(|loader| {
                unknown::physical_device_proc_addr(loader, name, false)
                    .or_else(|| unknown::device_proc_addr(loader, name, false))
            });
            if address.is_none() {
                unknown::log_unrecognized_instance_command(name);
            }
            address
        }
    }
}

pub(crate) unsafe extern "system" fn terminator_get_device_proc_addr(
    device: vk::VkDevice,
    name: *const c_char,
) -> PFN_vkVoidFunction {
    if name.is_null() {
        return None;
    }
    // SAFETY: The layer ABI requires a live, NUL-terminated command name.
    let name = unsafe { CStr::from_ptr(name) };
    if name == c"vkGetDeviceProcAddr" {
        return Some(erase_function(
            terminator_get_device_proc_addr as PFN_vkGetDeviceProcAddr,
        ));
    }
    match name.to_bytes() {
        b"vkDestroyDevice" => Some(erase_function(
            crate::destroy_device_terminator as vk::PFN_vkDestroyDevice,
        )),
        b"vkCreateSwapchainKHR" => Some(erase_function(
            crate::surface::terminator_create_swapchain as vk::PFN_vkCreateSwapchainKHR,
        )),
        b"vkCreateSharedSwapchainsKHR" => Some(erase_function(
            crate::surface::terminator_create_shared_swapchains
                as vk::PFN_vkCreateSharedSwapchainsKHR,
        )),
        b"vkGetDeviceGroupSurfacePresentModesKHR" => Some(erase_function(
            crate::surface::terminator_get_device_group_surface_present_modes
                as vk::PFN_vkGetDeviceGroupSurfacePresentModesKHR,
        )),
        b"vkDebugMarkerSetObjectNameEXT" => Some(erase_function(
            crate::debug::terminator_vkDebugMarkerSetObjectNameEXT
                as vk::PFN_vkDebugMarkerSetObjectNameEXT,
        )),
        b"vkDebugMarkerSetObjectTagEXT" => Some(erase_function(
            crate::debug::terminator_vkDebugMarkerSetObjectTagEXT
                as vk::PFN_vkDebugMarkerSetObjectTagEXT,
        )),
        b"vkSetDebugUtilsObjectNameEXT" => Some(erase_function(
            crate::debug::terminator_vkSetDebugUtilsObjectNameEXT
                as vk::PFN_vkSetDebugUtilsObjectNameEXT,
        )),
        b"vkSetDebugUtilsObjectTagEXT" => Some(erase_function(
            crate::debug::terminator_vkSetDebugUtilsObjectTagEXT
                as vk::PFN_vkSetDebugUtilsObjectTagEXT,
        )),
        _ => {
            // SAFETY: The device was returned by the lower chain and registered
            // before control returned to the requesting layer.
            let device = unsafe { LoaderDevice::from_handle(device) }?;
            // SAFETY: The stored ICD resolver and device originate together.
            device.resolve(name)
        }
    }
}

pub(super) unsafe extern "system" fn terminator_enumerate_device_layer_properties(
    physical_device: vk::VkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut vk::VkLayerProperties,
) -> VkResult {
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The terminator receives the loader's physical-device wrapper.
    let Some(physical_device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) })
    else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let layers = &physical_device.instance().active_layer_properties;
    unsafe { enumerate_active_device_layers(layers, &mut *property_count, properties) }
}

pub(super) fn device_extension_property(extension: &LayerExtension) -> VkExtensionProperties {
    let mut property = VkExtensionProperties::DEFAULT;
    copy_c_string(&extension.name, &mut property.extensionName);
    property.specVersion = extension.spec_version;
    property
}

pub(super) fn append_unique_device_extension(
    extensions: &mut Vec<VkExtensionProperties>,
    extension: &VkExtensionProperties,
) -> Result<(), VkResult> {
    // `loader_add_to_ext_list` keeps the property encountered first.
    if extensions
        .iter()
        .any(|existing| existing.extensionName == extension.extensionName)
    {
        return Ok(());
    }
    extensions
        .try_reserve(1)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    extensions.push(*extension);
    Ok(())
}

pub(super) fn named_layer_device_extensions(
    name: &CStr,
) -> Result<Vec<VkExtensionProperties>, VkResult> {
    let manifests = discover_layers();
    if pending::json_allocation_failed() {
        return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
    }
    let valid = available_layer_mask(&manifests)?;
    let Some(root) = manifests
        .iter()
        .zip(valid.iter())
        .position(|(manifest, valid)| *valid && manifest.name.as_c_str() == name)
    else {
        return Ok(Vec::new());
    };
    let mut pending = Vec::new();
    pending
        .try_reserve_exact(manifests.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    pending.push(root);
    let mut visited = allocation::try_boxed_slice_filled(manifests.len(), false)?;
    let mut extensions = Vec::new();
    while let Some(index) = pending.pop() {
        if visited[index] {
            continue;
        }
        visited[index] = true;
        let manifest = &manifests[index];
        for extension in &manifest.device_extensions {
            append_unique_device_extension(&mut extensions, &device_extension_property(extension))?;
        }
        for component in manifest.component_layers.iter().rev() {
            if let Some(index) = manifests
                .iter()
                .position(|manifest| manifest.name == *component)
            {
                pending
                    .try_reserve(1)
                    .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
                pending.push(index);
            }
        }
    }
    Ok(extensions)
}

#[cold]
#[inline(never)]
pub(super) unsafe fn enumerate_named_layer_device_extensions(
    layer_name: &CStr,
    property_count: &mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    let extensions = match named_layer_device_extensions(layer_name) {
        Ok(extensions) => extensions,
        Err(result) => return result,
    };
    let total = extensions.len().min(u32::MAX as usize) as u32;
    if properties.is_null() {
        *property_count = total;
        return VkResult::SUCCESS;
    }
    let written = (*property_count as usize).min(extensions.len());
    unsafe { ptr::copy_nonoverlapping(extensions.as_ptr(), properties, written) };
    *property_count = written as u32;
    if written < extensions.len() {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

#[cold]
#[inline(never)]
pub(super) unsafe fn enumerate_icd_device_extensions(
    physical_device: &LoaderPhysicalDevice,
    property_count: &mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    let Some(enumerate) = physical_device
        .icd()
        .dispatch
        .vkEnumerateDeviceExtensionProperties
    else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if !properties.is_null() {
        let capacity = *property_count;
        let mut written = capacity;
        let result = unsafe {
            enumerate(
                physical_device.native,
                ptr::null(),
                &raw mut written,
                properties,
            )
        };
        if result != VkResult::SUCCESS {
            return result;
        }
        written = written.min(capacity);
        for layer in physical_device
            .instance()
            .layers
            .iter()
            .filter(|layer| layer.implicit)
        {
            for extension in &layer.device_extensions {
                let property = device_extension_property(extension);
                let existing = unsafe { core::slice::from_raw_parts(properties, written as usize) }
                    .iter()
                    .any(|existing| existing.extensionName == property.extensionName);
                if existing {
                    continue;
                }
                if written == capacity {
                    *property_count = written;
                    return VkResult::INCOMPLETE;
                }
                unsafe { properties.add(written as usize).write(property) };
                written += 1;
            }
        }
        *property_count = written;
        return VkResult::SUCCESS;
    }
    let mut count = 0;
    let result = unsafe {
        enumerate(
            physical_device.native,
            ptr::null(),
            &raw mut count,
            ptr::null_mut(),
        )
    };
    if result != VkResult::SUCCESS {
        return result;
    }
    let capacity = count as usize;
    let mut extensions = Vec::new();
    if extensions.try_reserve_exact(capacity).is_err() {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    extensions.resize(capacity, VkExtensionProperties::DEFAULT);
    let result = unsafe {
        enumerate(
            physical_device.native,
            ptr::null(),
            &raw mut count,
            extensions.as_mut_ptr(),
        )
    };
    if result != VkResult::SUCCESS {
        return result;
    }
    extensions.truncate((count as usize).min(capacity));
    for layer in physical_device
        .instance()
        .layers
        .iter()
        .filter(|layer| layer.implicit)
    {
        for extension in &layer.device_extensions {
            if let Err(result) = append_unique_device_extension(
                &mut extensions,
                &device_extension_property(extension),
            ) {
                return result;
            }
        }
    }
    *property_count = extensions.len().min(u32::MAX as usize) as u32;
    VkResult::SUCCESS
}

pub(crate) unsafe extern "system" fn terminator_enumerate_device_extension_properties(
    physical_device: vk::VkPhysicalDevice,
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    if property_count.is_null() {
        return VkResult::INCOMPLETE;
    }
    let Some(physical_device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) })
    else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if !layer_name.is_null() && unsafe { layer_name.read() } != 0 {
        // Reserve error state before discovery; this API can run outside an
        // instance-create chain. Do not lose discovery OOM as an empty list.
        return pending::with_json_error_scope(|| unsafe {
            // SAFETY: The entrypoint validated the count pointer; the Vulkan
            // contract supplies a terminated name and writable output extent.
            enumerate_named_layer_device_extensions(
                CStr::from_ptr(layer_name),
                &mut *property_count,
                properties,
            )
        });
    }
    unsafe { enumerate_icd_device_extensions(physical_device, &mut *property_count, properties) }
}

pub(crate) unsafe fn enumerate_active_device_layers(
    layers: &[ActiveLayerProperty],
    property_count: &mut u32,
    properties: *mut vk::VkLayerProperties,
) -> VkResult {
    let total = layers.len().min(u32::MAX as usize) as u32;
    if properties.is_null() {
        *property_count = total;
        return VkResult::SUCCESS;
    }
    let capacity = *property_count as usize;
    let written = capacity.min(layers.len());
    for (index, layer) in layers.iter().take(written).enumerate() {
        let mut property = vk::VkLayerProperties::DEFAULT;
        copy_c_string(&layer.name, &mut property.layerName);
        copy_c_string(&layer.description, &mut property.description);
        property.specVersion = layer.api_version;
        property.implementationVersion = layer.implementation_version;
        unsafe { properties.add(index).write(property) };
    }
    *property_count = written as u32;
    if written < layers.len() {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

pub(crate) unsafe fn enumerate_instance_layers(
    discovered: crate::discovery::DiscoveredLayers,
    property_count: &mut u32,
    properties: *mut vk::VkLayerProperties,
) -> VkResult {
    emit_global_layer_search_diagnostics(&discovered, false);
    let mut manifests = discovered.into_vec();
    let valid = match available_layer_mask(&manifests) {
        Ok(valid) => valid,
        Err(result) => return result,
    };
    let mut index = 0;
    manifests.retain(|_| {
        let keep = valid[index];
        index += 1;
        keep
    });
    manifests.retain(|manifest| {
        manifest.settings_control.is_some()
            || !forced_disabled(manifest)
            || forced_enabled(manifest)
    });
    // Upstream removes only the first later duplicate for each retained
    // position, not every occurrence of a name. Preserve that observable order
    // for malformed multi-layer manifests without allocating an identity set.
    let mut index = 0;
    while index < manifests.len() {
        if manifests[index].settings_control.is_none()
            && let Some(duplicate) = manifests[index + 1..].iter().position(|candidate| {
                candidate.settings_control.is_none() && candidate.name == manifests[index].name
            })
        {
            manifests.remove(index + 1 + duplicate);
        }
        index += 1;
    }
    let total = manifests.len().min(u32::MAX as usize) as u32;
    if properties.is_null() {
        *property_count = total;
        return VkResult::SUCCESS;
    }
    let capacity = *property_count as usize;
    let written = capacity.min(manifests.len());
    for (index, manifest) in manifests.iter().take(written).enumerate() {
        let mut property = vk::VkLayerProperties::DEFAULT;
        copy_c_string(&manifest.name, &mut property.layerName);
        copy_c_string(&manifest.description, &mut property.description);
        property.specVersion = manifest.api_version;
        property.implementationVersion = manifest.implementation_version;
        unsafe { properties.add(index).write(property) };
    }
    *property_count = written as u32;
    if written < manifests.len() {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

pub(super) fn copy_c_string<const N: usize>(source: &CStr, destination: &mut [c_char; N]) {
    let bytes = source.to_bytes_with_nul();
    let count = bytes.len().min(N);
    // SAFETY: Both element types occupy one byte and the slices have `count` entries.
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), destination.as_mut_ptr().cast(), count);
    }
    if count == N {
        destination[N - 1] = 0;
    }
}

pub(crate) unsafe extern "system" fn terminator_get_physical_device_proc_addr(
    instance: vk::VkInstance,
    name: *const c_char,
) -> PFN_vkVoidFunction {
    if instance == vk::VkInstance::NULL || name.is_null() {
        return None;
    }
    // SAFETY: GPDPA requires a live NUL-terminated command name.
    let name = unsafe { CStr::from_ptr(name) };
    if let Some(lookup) = crate::command_lookup(name) {
        if lookup.scope != CommandScope::Instance {
            return None;
        }
        return crate::physical_device_terminator_proc_addr(lookup.id)
            .or_else(|| crate::exported_proc_addr(lookup.id));
    }
    // SAFETY: Creation-time handles are internal and later handles are found
    // through their registered dispatch table.
    let instance = unsafe {
        LoaderInstance::from_handle(instance)
            .or_else(|| LoaderInstance::from_internal_handle(instance))
    }?;
    let address = unknown::physical_device_proc_addr(instance, name, false);
    if address.is_none() {
        unknown::log_unrecognized_physical_device_command(name);
    }
    address
}

/// Executes the activated instance-layer chain around a pre-created loader instance.
///
/// # Safety
///
/// The create structures and output pointer must satisfy `vkCreateInstance`'s
/// contract, and every loaded layer must remain live throughout the call.
pub(crate) unsafe fn create_instance_chain(
    instance: &mut LoaderInstance,
    create_info: &VkInstanceCreateInfo<'_>,
    allocator: *const vk::VkAllocationCallbacks<'_>,
    output: *mut vk::VkInstance,
) -> VkResult {
    debug_assert!(!instance.layers.is_empty());
    let count = instance.layers.len();
    let Ok(mut links) = ScratchArray::<LayerInstanceLink, STACK_LAYER_LINKS>::try_new(count) else {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    };
    let links_ptr = links.as_mut_ptr();
    let mut next_gpdpa = terminator_get_physical_device_proc_addr as GetPhysicalDeviceProcAddr;
    for index in (0..count).rev() {
        let next_gipa = instance.layers.get(index + 1).map_or(
            terminator_get_instance_proc_addr as PFN_vkGetInstanceProcAddr,
            |layer| layer.get_instance_proc_addr,
        );
        // SAFETY: `links_ptr` has `count` writable entries and remains stable.
        unsafe {
            links_ptr.add(index).write(LayerInstanceLink {
                next: if index + 1 == count {
                    ptr::null_mut()
                } else {
                    links_ptr.add(index + 1)
                },
                next_get_instance_proc_addr: next_gipa,
                next_get_physical_device_proc_addr: next_gpdpa,
            });
        }
        if let Some(gpdpa) = instance.layers[index].get_physical_device_proc_addr {
            next_gpdpa = gpdpa;
        }
    }
    let link_info = LayerInstanceCreateInfo {
        s_type: VkStructureType::LOADER_INSTANCE_CREATE_INFO,
        next: create_info.pNext,
        function: LayerFunction::LinkInfo,
        value: LayerInstanceCreateInfoUnion {
            layer_info: links_ptr,
        },
    };
    let data_callback = LayerInstanceCreateInfo {
        s_type: VkStructureType::LOADER_INSTANCE_CREATE_INFO,
        next: core::ptr::from_ref(&link_info).cast(),
        function: LayerFunction::LoaderDataCallback,
        value: LayerInstanceCreateInfoUnion {
            set_instance_loader_data,
        },
    };
    let device_callback = LayerInstanceCreateInfo {
        s_type: VkStructureType::LOADER_INSTANCE_CREATE_INFO,
        next: core::ptr::from_ref(&data_callback).cast(),
        function: LayerFunction::LayerCreateDeviceCallback,
        value: LayerInstanceCreateInfoUnion {
            layer_device: LayerDeviceCallbacks {
                create_device: layer_create_device_callback,
                destroy_device: layer_destroy_device_callback,
            },
        },
    };
    let loader_features = LayerInstanceCreateInfo {
        s_type: VkStructureType::LOADER_INSTANCE_CREATE_INFO,
        next: core::ptr::from_ref(&device_callback).cast(),
        function: LayerFunction::LoaderFeatures,
        value: LayerInstanceCreateInfoUnion { loader_features: 0 },
    };
    let mut layered_create_info = *create_info;
    layered_create_info.pNext = core::ptr::from_ref(&loader_features).cast();

    let top = &instance.layers[0];
    // SAFETY: The negotiated layer GIPA returns functions with Vulkan ABIs.
    let create: Option<vk::PFN_vkCreateInstance> = unsafe {
        crate::load_typed((top.get_instance_proc_addr)(
            instance.handle(),
            c"vkCreateInstance".as_ptr(),
        ))
    };
    let Some(create) = create else {
        return VkResult::ERROR_LAYER_NOT_PRESENT;
    };
    let previous = pending::replace_instance(instance.handle());
    unsafe { output.write(instance.handle()) };
    // SAFETY: The chain nodes, links, and caller structures remain live for the call.
    let result = unsafe { create(&raw const layered_create_info, allocator, output) };
    pending::replace_instance(previous);
    if result == VkResult::SUCCESS {
        // SAFETY: The layer returned this handle and its negotiated resolvers
        // remain loaded in `instance.layers`.
        unsafe { instance.load_dispatch(top.get_instance_proc_addr, next_gpdpa, output.read()) };
    }
    result
}

pub(super) fn extension_property_name(property: &VkExtensionProperties) -> Option<&CStr> {
    let chars = property.extensionName.as_slice();
    // SAFETY: `c_char` is exactly one byte on every supported C ABI.
    let bytes = unsafe { core::slice::from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len()) };
    CStr::from_bytes_until_nul(bytes).ok()
}

/// Collects device extensions advertised by active layer manifests and code.
///
/// # Safety
///
/// `physical_device` must be a live wrapper belonging to `instance`.
pub(crate) unsafe fn available_device_extensions(
    instance: &LoaderInstance,
    physical_device: vk::VkPhysicalDevice,
) -> Result<Box<[CString]>, VkResult> {
    let mut names = Vec::new();
    for extension in instance
        .layers
        .iter()
        .flat_map(|layer| &layer.device_extensions)
    {
        let name = allocation::try_c_string(&extension.name)?;
        allocation::try_push(&mut names, name)?;
    }
    let Some(top) = instance.layers.first() else {
        return allocation::try_into_boxed_slice(names);
    };
    // SAFETY: The top layer remains loaded and the name has static storage.
    let enumerate: Option<PFN_vkEnumerateDeviceExtensionProperties> = unsafe {
        crate::load_typed((top.get_instance_proc_addr)(
            instance.chain_handle(),
            c"vkEnumerateDeviceExtensionProperties".as_ptr(),
        ))
    };
    let Some(enumerate) = enumerate else {
        return allocation::try_into_boxed_slice(names);
    };
    let mut count = 0_u32;
    // SAFETY: The physical device and writable count are live for this query.
    let result = unsafe {
        enumerate(
            physical_device,
            ptr::null(),
            &raw mut count,
            ptr::null_mut(),
        )
    };
    if result != VkResult::SUCCESS {
        return Err(result);
    }
    let capacity = count as usize;
    let mut properties = allocation::try_box_uninit_slice::<VkExtensionProperties>(capacity)?;
    let mut returned = count;
    // SAFETY: `properties` has `capacity` writable entries.
    let result = unsafe {
        enumerate(
            physical_device,
            ptr::null(),
            &raw mut returned,
            properties.as_mut_ptr().cast(),
        )
    };
    if result != VkResult::SUCCESS && result != VkResult::INCOMPLETE {
        return Err(result);
    }
    let initialized = (returned as usize).min(capacity);
    for name in properties[..initialized]
        .iter()
        // SAFETY: The enumeration initialized the reported leading entries.
        .filter_map(|property| extension_property_name(unsafe { property.assume_init_ref() }))
    {
        let name = allocation::try_c_string(name)?;
        allocation::try_push(&mut names, name)?;
    }
    names.sort_unstable_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    names.dedup_by(|left, right| left.as_bytes() == right.as_bytes());
    allocation::try_into_boxed_slice(names)
}

pub(super) unsafe extern "system" fn set_device_loader_data(
    device: vk::VkDevice,
    object: *mut c_void,
) -> VkResult {
    if object.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The callback is installed only in a live loader-created chain.
    let Some(device) = (unsafe { LoaderDevice::from_handle(device) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    // SAFETY: The layer supplies writable dispatchable loader data.
    unsafe { device.set_object_dispatch(object) };
    VkResult::SUCCESS
}

/// Executes the activated device-layer chain.
///
/// # Safety
///
/// Arguments must satisfy `vkCreateDevice`'s contract and belong to `instance`.
pub(crate) unsafe fn create_device_chain(
    instance: &LoaderInstance,
    physical_device: vk::VkPhysicalDevice,
    create_info: &vk::VkDeviceCreateInfo<'_>,
    allocator: *const vk::VkAllocationCallbacks<'_>,
    output: *mut vk::VkDevice,
) -> VkResult {
    unsafe {
        create_device_chain_from(instance, physical_device, create_info, allocator, output, 0)
    }
}

pub(super) unsafe fn create_device_chain_from(
    instance: &LoaderInstance,
    physical_device: vk::VkPhysicalDevice,
    create_info: &vk::VkDeviceCreateInfo<'_>,
    allocator: *const vk::VkAllocationCallbacks<'_>,
    output: *mut vk::VkDevice,
    first: usize,
) -> VkResult {
    let layers = &instance.layers[first..];
    let count = layers.len();
    let Ok(mut links) = ScratchArray::<LayerDeviceLink, STACK_LAYER_LINKS>::try_new(count) else {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    };
    let links_ptr = links.as_mut_ptr();
    for index in (0..count).rev() {
        let (next_instance_proc_addr, next_device_proc_addr) = layers.get(index + 1).map_or(
            (
                terminator_get_instance_proc_addr as PFN_vkGetInstanceProcAddr,
                terminator_get_device_proc_addr as PFN_vkGetDeviceProcAddr,
            ),
            |layer| (layer.get_instance_proc_addr, layer.get_device_proc_addr),
        );
        // SAFETY: `links_ptr` has `count` stable writable entries.
        unsafe {
            links_ptr.add(index).write(LayerDeviceLink {
                next: if index + 1 == count {
                    ptr::null_mut()
                } else {
                    links_ptr.add(index + 1)
                },
                next_get_instance_proc_addr: next_instance_proc_addr,
                next_get_device_proc_addr: next_device_proc_addr,
            });
        }
    }
    let link_info = LayerDeviceCreateInfo {
        s_type: VkStructureType::LOADER_DEVICE_CREATE_INFO,
        next: create_info.pNext,
        function: LayerFunction::LinkInfo,
        value: LayerDeviceCreateInfoUnion {
            layer_info: links_ptr,
        },
    };
    let data_callback = LayerDeviceCreateInfo {
        s_type: VkStructureType::LOADER_DEVICE_CREATE_INFO,
        // Upstream inserts VK_LAYER_LINK_INFO only when at least one device
        // layer is active. ICDs inspect these loader-private chain nodes, so a
        // zero-layer chain must lead directly to the application's pNext.
        next: if count == 0 {
            create_info.pNext
        } else {
            core::ptr::from_ref(&link_info).cast()
        },
        function: LayerFunction::LoaderDataCallback,
        value: LayerDeviceCreateInfoUnion {
            set_device_loader_data,
        },
    };
    let mut layered_create_info = *create_info;
    layered_create_info.pNext = core::ptr::from_ref(&data_callback).cast();
    let sentinel = match allocation::try_box(DeviceCreateSentinel {
        magic: crate::DEVICE_DISPATCH_MAGIC,
        padding: [0; 120],
    }) {
        Ok(sentinel) => sentinel,
        Err((result, _sentinel)) => return result,
    };
    let sentinel_address = core::ptr::from_ref(sentinel.as_ref()) as usize;
    let mut created_device = vk::VkDevice(sentinel_address as *mut c_void);
    let layer_start = match pending::start_device_chain(sentinel_address) {
        Ok(reservation) => reservation,
        Err(result) => return result,
    };
    let (top_instance_proc_addr, top_device_proc_addr) = layers.first().map_or(
        (
            terminator_get_instance_proc_addr as PFN_vkGetInstanceProcAddr,
            terminator_get_device_proc_addr as PFN_vkGetDeviceProcAddr,
        ),
        |layer| (layer.get_instance_proc_addr, layer.get_device_proc_addr),
    );
    // SAFETY: The negotiated layer/terminator GIPA returns Vulkan ABI function pointers.
    let create: Option<vk::PFN_vkCreateDevice> = unsafe {
        crate::load_typed(top_instance_proc_addr(
            instance.chain_handle(),
            c"vkCreateDevice".as_ptr(),
        ))
    };
    let Some(create) = create else {
        let created = pending::pop_created_device();
        debug_assert!(created.is_none());
        let popped = pending::pop_device_sentinel();
        debug_assert_eq!(popped, Some(sentinel_address));
        return VkResult::ERROR_LAYER_NOT_PRESENT;
    };
    for layer in layers.iter().rev() {
        instance.log_loader_category_message_text(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
            LogFilter::Layer,
            format_args!(
                "Inserted device layer \"{}\" ({})",
                crate::debug::diagnostics::LossyBytes(layer.name.to_bytes()),
                layer.library_path.display()
            ),
        );
    }
    pending::DeviceLayerStartReservation::push(layer_start, first);
    // SAFETY: Every chain node and caller-owned structure remains live for the call.
    let result = unsafe {
        create(
            physical_device,
            &raw const layered_create_info,
            allocator,
            &raw mut created_device,
        )
    };
    let popped_first = pending::pop_device_layer_start();
    debug_assert_eq!(popped_first, Some(first));
    let created_dispatch = pending::pop_created_device();
    let popped = pending::pop_device_sentinel();
    debug_assert_eq!(popped, Some(sentinel_address));
    if result == VkResult::SUCCESS {
        // SAFETY: The terminator created this loader device and no aliasing call exists yet.
        let Some(device) =
            (unsafe { created_dispatch.and_then(|key| LoaderDevice::from_dispatch_key_mut(key)) })
        else {
            return VkResult::ERROR_INITIALIZATION_FAILED;
        };
        // SAFETY: Device creation has not returned to the application and the
        // top layer returned this live chain handle.
        if let Err(result) = unsafe { device.set_chain(created_device, top_device_proc_addr) } {
            // The top-level chain succeeded, but loader bookkeeping could not
            // retain its alias. Destroy the completed chain before releasing
            // the direct loader record.
            let destroy: Option<vk::PFN_vkDestroyDevice> = unsafe {
                crate::load_typed(top_device_proc_addr(
                    created_device,
                    c"vkDestroyDevice".as_ptr(),
                ))
            };
            if let Some(destroy) = destroy {
                // SAFETY: This is the live top-level device returned above and
                // the allocator matches its successful creation call.
                unsafe { destroy(created_device, allocator) };
            }
            if let Some(dispatch) = created_dispatch {
                drop(LoaderDevice::take_dispatch(
                    dispatch as *const LayerDeviceDispatchTable,
                ));
            }
            return result;
        }
        // The public output is committed only after the full create chain and
        // dispatch initialization complete successfully, matching upstream.
        unsafe { output.write(created_device) };
    } else if let Some(dispatch) = created_dispatch {
        // A lower layer or the terminator may have created a device before an
        // upper layer failed. Upstream owns and tears down that partial chain;
        // it must never escape through the caller's output parameter.
        let dispatch = dispatch as *const LayerDeviceDispatchTable;
        // As in `loader_layer_create_device`, free the loader record but do not
        // call into the failed chain: a layer which returned failure owns the
        // cleanup of any lower device it successfully created.
        drop(LoaderDevice::take_dispatch(dispatch));
    }
    drop(links);
    drop(sentinel);
    result
}

pub(crate) unsafe fn validate_pending_device_output(output: &mut vk::VkDevice) {
    let Some(expected) = pending::device_sentinel() else {
        return;
    };
    let returned = *output;
    if returned == vk::VkDevice::NULL {
        fatal_layer_policy(
            "terminator_CreateDevice: Loader device pointer null encountered.  Possibly set by active layer. (Policy #LLP_LAYER_22)",
        );
    }
    if returned.0 as usize != expected {
        let pointer = LayerPointer(returned.0);
        fatal_layer_policy(format_args!(
            "terminator_CreateDevice: Device pointer ({pointer}) has invalid MAGIC value 0x00000000. The expected value is 0x10ADED040410ADED. Device value possibly corrupted by active layer (Policy #LLP_LAYER_22).  ",
        ));
    }
    let magic = unsafe { returned.0.cast::<u64>().read() };
    if magic != crate::DEVICE_DISPATCH_MAGIC {
        let pointer = LayerPointer(returned.0);
        fatal_layer_policy(format_args!(
            "terminator_CreateDevice: Device pointer ({pointer}) has invalid MAGIC value 0x{magic:08x}. The expected value is 0x10ADED040410ADED. Device value possibly corrupted by active layer (Policy #LLP_LAYER_22).  ",
        ));
    }
}
