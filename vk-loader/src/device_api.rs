//! device implementation.

use crate::{
    CStr, CommandScope, ExtensionSet, LoaderDevice, LoaderInstance, LoaderPhysicalDevice,
    LoaderPhysicalDeviceTrampoline, PFN_vkCreateDevice, PFN_vkDestroyDevice,
    PFN_vkGetDeviceProcAddr, PFN_vkVoidFunction, VkAllocationCallbacks, VkDevice,
    VkDeviceCreateInfo, VkInstance, VkPhysicalDevice, VkResult, allocation, c_char, collections,
    command_lookup, command_must_use_loader_trampoline, debug::diagnostics, device_dispatch,
    discover_active_physical_devices_with_diagnostics, emit_instance_category_message,
    emit_instance_loader_message, exported_proc_addr, fatal_loader_error, layer,
    layer_device_dispatch_proc_addr, maintenance5_version_checks, pending, platform,
    translate_device_group_chain, validate_and_filter_device_extensions,
};

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
    // Querying through the layer chain both validates the wrapped physical
    // device and lets layers contribute device extensions.
    let chain_physical_device = trampoline.chain;
    let layer_extensions =
        match unsafe { layer::available_device_extensions(instance, chain_physical_device) } {
            Ok(extensions) => extensions,
            Err(result) => return result,
        };
    let extension_token = match pending::push_device_extensions(&layer_extensions) {
        Ok(token) => token,
        Err(result) => return result,
    };
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
    if result == VkResult::ERROR_OUT_OF_HOST_MEMORY {
        emit_instance_loader_message(
            instance,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
            format_args!("loader_create_logical_device: Failed to alloc struct loader_device"),
        );
        emit_instance_loader_message(
            instance,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
            format_args!("vkCreateDevice:  Failed to create device chain."),
        );
    }
    result
}

#[cold]
pub(crate) unsafe fn emit_device_layer_callstack(instance: &LoaderInstance) {
    for message in [
        "vkCreateDevice layer callstack setup to:",
        "   <Application>",
        "     ||",
        "   <Loader>",
        "     ||",
    ] {
        emit_instance_category_message(
            instance,
            &[platform::LogFilter::Layer, platform::LogFilter::Driver],
            "DRIVER",
            format_args!("{message}"),
        );
    }
    let first_device_layer = pending::device_layer_start().min(instance.layers.len());
    for layer in &instance.layers[first_device_layer..] {
        emit_instance_category_message(
            instance,
            &[platform::LogFilter::Layer],
            "LAYER",
            format_args!(
                "   {}",
                crate::debug::diagnostics::LossyBytes(layer.name.to_bytes())
            ),
        );
        emit_instance_category_message(
            instance,
            &[platform::LogFilter::Layer],
            "LAYER",
            format_args!(
                "           Type: {}",
                if layer.implicit {
                    "Implicit"
                } else {
                    "Explicit"
                }
            ),
        );
        emit_instance_category_message(
            instance,
            &[platform::LogFilter::Layer],
            "LAYER",
            format_args!("           Enabled By: {}", layer.enabled_by()),
        );
        if let Some(disable_environment) = layer.disable_environment() {
            emit_instance_category_message(
                instance,
                &[platform::LogFilter::Layer],
                "LAYER",
                format_args!(
                    "               Disable Env Var:  {}",
                    std::path::Path::new(disable_environment).display()
                ),
            );
        }
        emit_instance_category_message(
            instance,
            &[platform::LogFilter::Layer],
            "LAYER",
            format_args!("           Manifest: {}", layer.manifest_path().display()),
        );
        emit_instance_category_message(
            instance,
            &[platform::LogFilter::Layer],
            "LAYER",
            format_args!("           Library:  {}", layer.library_path.display()),
        );
        emit_instance_category_message(
            instance,
            &[platform::LogFilter::Layer],
            "LAYER",
            format_args!("     ||"),
        );
    }
    emit_instance_category_message(
        instance,
        &[platform::LogFilter::Layer, platform::LogFilter::Driver],
        "DRIVER",
        format_args!("   <Device>"),
    );
}

#[cold]
pub(crate) unsafe fn emit_device_driver(
    instance: &LoaderInstance,
    physical_device: &LoaderPhysicalDevice,
) {
    let icd = physical_device.icd();
    let mut properties = vk::VkPhysicalDeviceProperties::DEFAULT;
    if let Some(get_properties) = icd.dispatch.vkGetPhysicalDeviceProperties {
        unsafe { get_properties(physical_device.native, &raw mut properties) };
    }
    // SAFETY: The returned Vulkan device name is NUL-terminated; the default
    // properties also contain a terminated empty name when the command is absent.
    let name = unsafe { CStr::from_ptr(properties.deviceName.as_ptr()) };
    let name = diagnostics::LossyBytes(name.to_bytes());
    let path = icd
        .icd
        .library_path()
        .unwrap_or_else(|| std::path::Path::new(""))
        .display();
    emit_instance_category_message(
        instance,
        &[platform::LogFilter::Layer, platform::LogFilter::Driver],
        "DRIVER",
        format_args!("       Using \"{name}\" with driver: \"{path}\""),
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
    let icd_extension_names =
        match unsafe { validated_icd_device_extensions(physical_device, create_info) } {
            Ok(names) => names,
            Err(result) => return result,
        };
    unsafe { emit_device_driver(physical_device.instance(), physical_device) };
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
        unsafe { destroy_unregistered_device(icd_instance, native, allocator) };
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };

    // SAFETY: The validated create info remains live for this call.
    let strict_version_checks = unsafe { maintenance5_version_checks(create_info) };
    // SAFETY: Device extension names satisfy the create-info string-array contract.
    let enabled_extensions = unsafe {
        ExtensionSet::from_names(
            create_info.enabledExtensionCount,
            create_info.ppEnabledExtensionNames,
        )
    };
    // SAFETY: `native` was just returned by this ICD, its GDPA was resolved
    // from the matching live instance, and the parent instance owns this path.
    let loader_device = match unsafe {
        LoaderDevice::new(
            native,
            get_device_proc_addr,
            physical_device.instance(),
            physical_device.icd_index,
            physical_device.app_api_version,
            strict_version_checks,
            enabled_extensions,
        )
    } {
        Ok(device) => device,
        Err(result) => {
            unsafe { destroy_unregistered_device(icd_instance, native, allocator) };
            return result;
        }
    };
    let loader_device = match LoaderDevice::try_register(loader_device) {
        Ok(handle) => handle,
        Err((error, loader_device)) => {
            if let Some(destroy) = loader_device.icd_destroy_device() {
                // SAFETY: The loader record retains the matching live native device.
                unsafe { destroy(loader_device.icd_device, allocator) };
            }
            return error;
        }
    };
    // SAFETY: The caller supplied writable device storage.
    unsafe { device.write(loader_device) };
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
        instance.log_loader_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL
                | vk::VkDebugUtilsMessageTypeFlagBitsEXT::VALIDATION,
            c"vkEnumeratePhysicalDevices: Received NULL pointer for physical device count return value. [VUID-vkEnumeratePhysicalDevices-pPhysicalDeviceCount-parameter]",
        );
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: Every native instance is retained by `instance` for this call.
    let emit_device_diagnostics =
        !physical_devices.is_null() || instance.device_configurations.is_none();
    let native_devices = unsafe {
        discover_active_physical_devices_with_diagnostics(instance, emit_device_diagnostics)
    };
    let native_devices = match native_devices {
        Ok(devices) => devices,
        Err(result) => {
            if result == VkResult::ERROR_INITIALIZATION_FAILED
                && instance.device_configurations.is_none()
            {
                instance.log_loader_message(
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
                    c"setup_loader_term_phys_devs:  Failed to detect any valid GPUs in the current config",
                );
            }
            // SAFETY: The caller supplied writable count storage.
            unsafe { physical_device_count.write(0) };
            return result;
        }
    };
    if native_devices.is_empty() {
        instance.log_loader_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
            c"setup_loader_term_phys_devs:  Failed to detect any valid GPUs in the current config",
        );
    }
    let mut devices = instance.physical_devices.lock();
    let mut active = Vec::new();
    if active.try_reserve_exact(native_devices.len()).is_err()
        || devices.owned.try_reserve(native_devices.len()).is_err()
    {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    for native_device in &native_devices {
        let key = (native_device.icd_index, native_device.handle.0 as usize);
        let device = match devices.owned.entry(key) {
            collections::HashMapEntry::Occupied(entry) => entry.into_mut(),
            collections::HashMapEntry::Vacant(entry) => {
                let device = match allocation::try_box(LoaderPhysicalDevice::new(
                    native_device.icd_index,
                    &instance.icds[native_device.icd_index],
                    instance,
                    instance.api_version,
                    native_device.handle,
                )) {
                    Ok(device) => device,
                    Err((result, _device)) => return result,
                };
                entry.insert(device)
            }
        };
        active.push(device.handle());
    }
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
        emit_trimmed_physical_devices(instance, devices.active.len(), written);
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

#[cold]
unsafe fn destroy_unregistered_device(
    icd_instance: &crate::IcdInstance,
    native: VkDevice,
    allocator: *const VkAllocationCallbacks<'_>,
) {
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
}

#[cold]
unsafe fn emit_validated_device_layers(
    physical_device: &LoaderPhysicalDevice,
    create_info: &VkDeviceCreateInfo<'_>,
) {
    if unsafe {
        layer::has_mismatched_device_layers(
            &physical_device.instance().enabled_layer_names,
            create_info,
        )
    } {
        physical_device.instance().log_loader_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
            c"loader_create_device_chain: Using deprecated and ignored 'ppEnabledLayerNames' member of 'VkDeviceCreateInfo' when creating a Vulkan device.",
        );
    }
    unsafe { emit_device_layer_callstack(physical_device.instance()) };
}

#[cold]
fn emit_trimmed_physical_devices(instance: &LoaderInstance, total: usize, written: usize) {
    diagnostics::with_message(
        format_args!(
            "terminator_EnumeratePhysicalDevices : Trimming device count from {total} to {written}."
        ),
        |message| {
            instance.log_loader_message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
                message,
            );
        },
    );
}

#[cold]
unsafe fn validated_icd_device_extensions(
    physical_device: &LoaderPhysicalDevice,
    create_info: &VkDeviceCreateInfo<'_>,
) -> Result<Vec<*const c_char>, VkResult> {
    let pending_extensions = pending::device_extensions();
    let layer_extensions = pending_extensions.map(|(extensions, extension_count)| {
        // SAFETY: The public trampoline owns this boxed slice for the entire
        // synchronous device-creation chain.
        unsafe { core::slice::from_raw_parts(extensions, extension_count) }
    });
    // SAFETY: The terminator has recovered the ICD's native physical device.
    unsafe {
        validate_and_filter_device_extensions(
            physical_device.instance(),
            physical_device.icd(),
            physical_device.native,
            create_info,
            |requested| match layer_extensions {
                Some(extensions) => extensions.iter().any(|name| name.as_c_str() == requested),
                None => physical_device.instance().layers.iter().any(|layer| {
                    layer
                        .device_extensions
                        .iter()
                        .any(|extension| extension.name.as_c_str() == requested)
                }),
            },
            || {
                emit_validated_device_layers(physical_device, create_info);
            },
        )
    }
}
