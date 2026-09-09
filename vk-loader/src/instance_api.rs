//! instance implementation.

use crate::{
    CStr, DirectIcdError, ExtensionSet, IcdInstance, InstanceDispatchTable,
    LINUX_SORT_PLATFORM_ENABLED, LoaderInstance, ManifestApiVersionStatus, PFN_vkDestroyInstance,
    ScannedIcd, ScannedIcdLoadError, VK_API_VERSION_1_0, VkAllocationCallbacks,
    VkDirectDriverLoadingInfoLUNARG, VkDirectDriverLoadingListLUNARG,
    VkDirectDriverLoadingModeLUNARG, VkExtensionProperties, VkInstance, VkInstanceCreateInfo,
    VkResult, VkStructureType,
    debug::{self, diagnostics},
    decimal_prefix_nonzero, destroy_all_surfaces, discovery, emulation, fatal_loader_error, icd,
    instance, layer, linux_sort_requires_properties_extension, load_typed, pending, platform,
    unknown, wsi_instance_extension_supported,
};
use std::path::Path;

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
    if let Err(error) = platform::initialize_loader() {
        return error;
    }
    if create_info.is_null() || instance.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let _allocator_guard = match pending::push_instance_allocator(allocator) {
        Ok(guard) => guard,
        Err(result) => return result,
    };
    if !discovery::probe_instance_allocation(core::mem::size_of::<instance::LoaderInstance>()) {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    let _loader_guard = platform::lock_loader();
    let _settings_log_filter = platform::reset_loader_settings_log_filter();
    // SAFETY: Pointer validity is required by the Vulkan entry-point contract.
    let create_info_ref = unsafe { &*create_info };
    // SAFETY: Application strings and name arrays follow the instance-create contract.
    unsafe { log_instance_create_info(create_info_ref) };
    let invalid_api_version = if create_info_ref.pApplicationInfo.is_null() {
        None
    } else {
        let requested = unsafe { (*create_info_ref.pApplicationInfo).apiVersion };
        (requested != 0 && requested < VK_API_VERSION_1_0).then_some(requested)
    };
    let api_version = if create_info_ref.pApplicationInfo.is_null() {
        VK_API_VERSION_1_0
    } else {
        // SAFETY: A non-null application-info pointer must be readable.
        unsafe { (*create_info_ref.pApplicationInfo).apiVersion }.max(VK_API_VERSION_1_0)
    };

    let settings = discovery::loader_settings();
    if pending::take_json_allocation_failed() {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    if let Err(error) = unsafe {
        emit_instance_configuration(
            create_info_ref,
            settings.as_ref(),
            invalid_api_version,
            api_version,
        )
    } {
        return error;
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
    if pending::take_json_allocation_failed() {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    layer::emit_selected_layer_activation_diagnostics(create_info_ref, &selected_layers);

    if let Err(result) =
        unsafe { validate_instance_extensions(create_info_ref, &selected_layers, &scanned_icds) }
    {
        icd::unload_preloaded_icds();
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
    let loader_instance = match LoaderInstance::new(
        api_version,
        enabled_extensions,
        scanned_icds,
        active_layers,
        device_configurations,
        allocator,
    ) {
        Ok(instance) => instance,
        Err(result) => return result,
    };
    unsafe { finish_instance_creation(loader_instance, create_info_ref, allocator, instance) }
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn log_instance_create_info(create_info: &VkInstanceCreateInfo<'_>) {
    let application = unsafe { create_info.pApplicationInfo.as_ref() };
    let application_name = application.map_or(c"", |info| {
        if info.pApplicationName.is_null() {
            c""
        } else {
            // SAFETY: Non-null application names are NUL-terminated by the create-info contract.
            unsafe { CStr::from_ptr(info.pApplicationName) }
        }
    });
    let engine_name = application.map_or(c"", |info| {
        if info.pEngineName.is_null() {
            c""
        } else {
            // SAFETY: Non-null engine names are NUL-terminated by the create-info contract.
            unsafe { CStr::from_ptr(info.pEngineName) }
        }
    });
    let application_name = diagnostics::LossyBytes(application_name.to_bytes());
    let engine_name = diagnostics::LossyBytes(engine_name.to_bytes());
    let application_version = application.map_or(0, |info| info.applicationVersion);
    let engine_version = application.map_or(0, |info| info.engineVersion);
    let api_version = application.map_or(0, |info| info.apiVersion);
    unsafe {
        emit_driver_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!(
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
pub(crate) unsafe fn log_instance_name_array(
    create_info: &VkInstanceCreateInfo<'_>,
    kind: &str,
    count: u32,
    names: *const *const core::ffi::c_char,
) {
    unsafe {
        emit_driver_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!("vkCreateInstance: Requested {count} instance {kind}(s):"),
        );
    };
    if names.is_null() {
        return;
    }
    for index in 0..count as usize {
        let name = unsafe { names.add(index).read() };
        let value = if name.is_null() {
            c"<NULL>"
        } else {
            // SAFETY: Each non-null requested name is NUL-terminated by the
            // Vulkan create-info contract and remains live during this call.
            unsafe { CStr::from_ptr(name) }
        };
        let value = diagnostics::LossyBytes(value.to_bytes());
        unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format_args!("   {value}"),
            );
        };
    }
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn validate_instance_extensions(
    create_info: &VkInstanceCreateInfo<'_>,
    layers: &layer::SelectedLayers,
    scanned_icds: &[ScannedIcdRecord],
) -> Result<(), VkResult> {
    if create_info.enabledExtensionCount == 0 {
        return Ok(());
    }
    if create_info.ppEnabledExtensionNames.is_null() {
        // SAFETY: The caller retains the complete instance-create chain.
        unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                format_args!(
                    "loader_validate_instance_extensions: Instance ppEnabledExtensionNames is NULL but enabledExtensionCount is greater than zero"
                ),
            );
        };
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
    // SAFETY: As with upstream discovery, environment mutation must not race
    // loader configuration reads, including from allocation callbacks.
    let filter_unknown = !unsafe {
        platform::inspect_environment_lossy(c"VK_LOADER_DISABLE_INST_EXT_FILTER", |value| {
            value.is_some_and(|value| decimal_prefix_nonzero(value.as_bytes()))
        })
    }?;
    for index in 0..create_info.enabledExtensionCount as usize {
        let name = unsafe { create_info.ppEnabledExtensionNames.add(index).read() };
        if name.is_null() {
            return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
        }
        let name = unsafe { CStr::from_ptr(name) };
        let id = crate::extension_id(name);
        if filter_unknown && !id.is_some_and(crate::generated::is_instance_extension) {
            // SAFETY: The caller retains the complete instance-create chain.
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    format_args!(
                        "loader_validate_instance_extensions: Extension {} not found in list of known instance extensions.",
                        crate::debug::diagnostics::LossyBytes(name.to_bytes())
                    ),
                );
            };
            return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
        }
        let loader_available = id.is_some_and(loader_instance_extension_supported);
        let globally_available = wsi_instance_extension_supported(name)
            && available.iter().any(|property| {
                // SAFETY: Loader-constructed extension properties are NUL-terminated.
                unsafe { CStr::from_ptr(property.extensionName.as_ptr()) == name }
            });
        let layer_available = layers.supports_instance_extension(name);
        if !loader_available && !globally_available && !layer_available {
            // SAFETY: The caller retains the complete instance-create chain.
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    format_args!(
                        "loader_validate_instance_extensions: Instance extension {} not supported by available ICDs or enabled layers.",
                        crate::debug::diagnostics::LossyBytes(name.to_bytes())
                    ),
                );
            };
            return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
        }
    }
    Ok(())
}

const fn loader_instance_extension_supported(id: u16) -> bool {
    matches!(
        id,
        crate::generated::VK_EXT_DEBUG_REPORT_EXTENSION_ID
            | crate::generated::VK_EXT_DEBUG_UTILS_EXTENSION_ID
            | crate::generated::VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_ID
            | crate::generated::VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_ID
    )
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn emit_driver_create_message(
    create_info: &VkInstanceCreateInfo<'_>,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: core::fmt::Arguments<'_>,
) {
    let filter = platform::LogFilter::from_severity(severity);
    platform::write_loader_log(filter, message);
    diagnostics::with_message(message, |message| {
        // SAFETY: The caller retains the complete instance-create pNext chain.
        unsafe { debug::messenger::submit_instance_create_message(create_info, severity, message) };
    });
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn emit_driver_category_create_message(
    create_info: &VkInstanceCreateInfo<'_>,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: core::fmt::Arguments<'_>,
) {
    let filter = platform::LogFilter::from_severity(severity);
    platform::write_loader_log_with_category(filter, platform::LogFilter::Driver, message);
    diagnostics::with_message(message, |message| {
        // SAFETY: The caller retains the complete instance-create pNext chain.
        unsafe { debug::messenger::submit_instance_create_message(create_info, severity, message) };
    });
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn emit_layer_category_create_message(
    create_info: &VkInstanceCreateInfo<'_>,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: core::fmt::Arguments<'_>,
) {
    let filter = platform::LogFilter::from_severity(severity);
    platform::write_loader_log_with_category(filter, platform::LogFilter::Layer, message);
    diagnostics::with_message(message, |message| {
        unsafe { debug::messenger::submit_instance_create_message(create_info, severity, message) };
    });
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn emit_driver_only_create_message(
    create_info: &VkInstanceCreateInfo<'_>,
    message: core::fmt::Arguments<'_>,
) {
    platform::write_loader_category_log(platform::LogFilter::Driver, message);
    diagnostics::with_message(message, |message| {
        // Category-only upstream messages are informational debug-utils messages.
        unsafe {
            debug::messenger::submit_instance_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                message,
            );
        };
    });
}

pub(crate) unsafe fn direct_driver_extension_enabled(
    create_info: &VkInstanceCreateInfo<'_>,
) -> bool {
    unsafe {
        instance_extension_enabled(
            create_info,
            vk::VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_NAME,
        )
    }
}

pub(crate) unsafe fn instance_extension_enabled(
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

pub(crate) unsafe fn direct_driver_list<'a>(
    create_info: &VkInstanceCreateInfo<'a>,
) -> Option<&'a VkDirectDriverLoadingListLUNARG<'a>> {
    // SAFETY: The instance-create pNext chain is readable by contract.
    let structure = unsafe {
        emulation::find_input_chain(
            create_info.pNext,
            VkStructureType::DIRECT_DRIVER_LOADING_LIST_LUNARG,
        )
    }?;
    // SAFETY: sType identifies the concrete structure layout.
    Some(unsafe { &*core::ptr::from_ref(structure).cast::<VkDirectDriverLoadingListLUNARG<'a>>() })
}

pub(crate) fn fatal_direct_driver_scan_error(result: VkResult) -> Option<VkResult> {
    (result == VkResult::ERROR_OUT_OF_HOST_MEMORY).then_some(result)
}

#[cold]
unsafe fn emit_empty_direct_driver_list(
    create_info: &VkInstanceCreateInfo<'_>,
    missing_drivers: bool,
) {
    let message = if missing_drivers {
        "loader_scan_for_direct_drivers: The VkDirectDriverLoadingListLUNARG structure in the pNext chain of VkInstanceCreateInfo has a NULL pDrivers member."
    } else {
        "loader_scan_for_direct_drivers: The VkDirectDriverLoadingListLUNARG structure in the pNext chain of VkInstanceCreateInfo has a non-null pDrivers member but a driverCount member with a value of zero."
    };
    unsafe {
        emit_driver_category_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!("{message}"),
        );
    }
}

#[cold]
unsafe fn emit_exclusive_direct_driver(create_info: &VkInstanceCreateInfo<'_>) {
    unsafe {
        emit_driver_category_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!(
                "loader_scan_for_direct_drivers: The VK_LUNARG_direct_driver_loading extension is active and specified VK_DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG, skipping system and environment variable driver search mechanisms."
            ),
        );
    };
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn scan_direct_drivers(
    create_info: &VkInstanceCreateInfo<'_>,
) -> Result<(bool, Vec<ScannedIcd>), VkResult> {
    let enabled = unsafe { direct_driver_extension_enabled(create_info) };
    let list = unsafe { direct_driver_list(create_info) };
    let Some(list) = list else {
        if enabled {
            unsafe {
                emit_driver_category_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "loader_scan_for_direct_drivers: The VK_LUNARG_direct_driver_loading extension was enabled but the pNext chain of VkInstanceCreateInfo did not contain the VkDirectDriverLoadingListLUNARG structure."
                    ),
                );
            };
        }
        return Ok((false, Vec::new()));
    };
    if !enabled {
        unsafe {
            emit_driver_category_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "loader_scan_for_direct_drivers: The pNext chain of VkInstanceCreateInfo contained the VkDirectDriverLoadingListLUNARG structure, but the VK_LUNARG_direct_driver_loading extension was not enabled."
                ),
            );
        };
        return Ok((false, Vec::new()));
    }

    let exclusive = list.mode == VkDirectDriverLoadingModeLUNARG::EXCLUSIVE;
    if exclusive {
        unsafe { emit_exclusive_direct_driver(create_info) };
    }
    if list.pDrivers.is_null() || list.driverCount == 0 {
        unsafe { emit_empty_direct_driver_list(create_info, list.pDrivers.is_null()) };
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
                emit_driver_category_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    format_args!(
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
                        format_args!(
                            "loader_add_direct_driver: Adding driver found in index {index} of VkDirectDriverLoadingListLUNARG::pDrivers structure. pfnGetInstanceProcAddr was set to {:p}",
                            gipa as *const ()
                        ),
                    );
                };
                drivers.push(driver);
            }
            Err(DirectIcdError::MutexInitialization(result)) => return Err(result),
            Err(DirectIcdError::EnumerateVersion(result)) => {
                if let Some(result) = fatal_direct_driver_scan_error(result) {
                    return Err(result);
                }
                // Upstream treats every other per-driver failure as a reason
                // to skip this entry and continue scanning the remaining
                // direct drivers.
            }
            Err(error) => {
                unsafe { emit_direct_driver_error(create_info, index, &error) };
            }
        }
    }
    Ok((exclusive, drivers))
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn emit_driver_search_roots(create_info: &VkInstanceCreateInfo<'_>) {
    unsafe {
        emit_driver_only_create_message(
            create_info,
            format_args!("Searching for driver manifest files"),
        );
    };
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn emit_driver_scan_preamble(
    create_info: &VkInstanceCreateInfo<'_>,
    scan: &discovery::DriverScan,
) {
    unsafe { emit_driver_search_roots(create_info) };
    unsafe {
        emit_driver_only_create_message(create_info, format_args!("   In following locations:"));
    };
    for root in &scan.search_roots {
        unsafe {
            emit_driver_only_create_message(create_info, format_args!("      {}", root.display()));
        };
    }
    if scan.reported_files.is_empty() {
        unsafe { emit_driver_only_create_message(create_info, format_args!("   Found no files")) };
    } else {
        unsafe {
            emit_driver_only_create_message(
                create_info,
                format_args!("   Found the following files:"),
            );
        };
        for path in &scan.reported_files {
            let display_path = if !scan.environment_override
                && scan.manifest_errors.iter().any(|(failed_path, error)| {
                    failed_path == path && *error == discovery::DriverManifestError::FailedOpen
                }) {
                path.file_name().unwrap_or_else(|| path.as_os_str())
            } else {
                path.as_os_str()
            };
            unsafe {
                emit_driver_only_create_message(
                    create_info,
                    format_args!("      {}", display_path.display()),
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
                    format_args!(
                        "Located json file \"{}\" from registry \"HKEY_LOCAL_MACHINE\\SOFTWARE\\Khronos\\Vulkan\\Drivers\"",
                        path.display(),
                    ),
                )
            };
        }
        if registry.no_unique_files {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format_args!(
                        "Found no registry files in HKEY_LOCAL_MACHINE\\SOFTWARE\\Khronos\\Vulkan\\Drivers"
                    ),
                )
            };
        }
    }
}

#[cold]
pub(crate) unsafe fn emit_driver_candidate_diagnostics(
    create_info: &VkInstanceCreateInfo<'_>,
    scan: &discovery::DriverScan,
    path: &std::path::Path,
    disposition: discovery::DriverDisposition,
) {
    let manifest = scan
        .manifests
        .iter()
        .find(|manifest| manifest.manifest_path == path);
    if let Some(manifest) = manifest {
        unsafe {
            emit_driver_candidate_manifest_diagnostics(create_info, manifest);
        }
    } else if let Some(manifest) = discovery::parse_manifest(path) {
        unsafe {
            emit_driver_candidate_manifest_diagnostics(create_info, &manifest);
        }
    } else if let Some((_, error)) = scan
        .manifest_errors
        .iter()
        .find(|(failed_path, _)| failed_path == path)
    {
        unsafe {
            emit_driver_manifest_error(create_info, path, *error, scan.environment_override);
        }
    }
    if disposition != discovery::DriverDisposition::Accepted {
        let (reason, variable) = match disposition {
            discovery::DriverDisposition::NotSelected => {
                ("not selected", "VK_LOADER_DRIVERS_SELECT")
            }
            discovery::DriverDisposition::Disabled => {
                ("it was disabled", "VK_LOADER_DRIVERS_DISABLE")
            }
            discovery::DriverDisposition::Accepted => return,
        };
        let name = path.file_name().unwrap_or(path.as_os_str());
        unsafe {
            emit_driver_category_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "Driver \"{}\" ignored because {reason} by env var '{variable}'",
                    Path::new(name).display()
                ),
            );
        }
    }
}

#[cold]
pub(crate) unsafe fn emit_driver_candidate_manifest_diagnostics(
    create_info: &VkInstanceCreateInfo<'_>,
    manifest: &discovery::DriverManifest,
) {
    unsafe {
        emit_driver_manifest_found(create_info, manifest);
        emit_driver_manifest_diagnostics(create_info, core::slice::from_ref(manifest));
        if vk::VK_API_VERSION_VARIANT(manifest.api_version) == 0 && manifest.architecture_supported
        {
            let displayed_library_path = manifest
                .library_path
                .to_str()
                .and_then(|path| path.rfind("/./").map(|index| Path::new(&path[index + 1..])))
                .unwrap_or(&manifest.library_path);
            emit_driver_category_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                format_args!(
                    "Searching for ICD drivers named {}",
                    displayed_library_path.display()
                ),
            );
        }
    }
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn emit_driver_manifest_diagnostics(
    create_info: &VkInstanceCreateInfo<'_>,
    manifests: &[discovery::DriverManifest],
) {
    for manifest in manifests {
        let variant = vk::VK_API_VERSION_VARIANT(manifest.api_version);
        if manifest.manifest_version >= vk::VK_MAKE_API_VERSION(0, 1, 0, 2) {
            unsafe {
                emit_driver_category_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format_args!(
                        "loader_parse_icd_manifest: {} has unknown icd manifest file version {}.{}.{}. May cause errors.",
                        manifest.manifest_path.display(),
                        vk::VK_API_VERSION_MAJOR(manifest.manifest_version),
                        vk::VK_API_VERSION_MINOR(manifest.manifest_version),
                        vk::VK_API_VERSION_PATCH(manifest.manifest_version),
                    ),
                );
            };
        }
        if variant != 0 {
            unsafe {
                emit_driver_category_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                    format_args!(
                        "Searching for ICD drivers named {}",
                        manifest.library_path.display()
                    ),
                );
                emit_driver_category_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format_args!(
                        "loader_parse_icd_manifest: Driver's ICD JSON {} 'api_version' field contains a non-zero variant value of {variant}.  Skipping ICD JSON.",
                        manifest.manifest_path.display(),
                    ),
                );
            };
        }
        if !manifest.architecture_supported {
            unsafe {
                emit_driver_category_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                    format_args!(
                        "Searching for ICD drivers named {}",
                        manifest.library_path.display()
                    ),
                );
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format_args!(
                        "loader_parse_icd_manifest: Driver library architecture doesn't match the current running architecture, skipping this driver"
                    ),
                );
            };
        }
    }
}

#[cold]
#[inline(never)]
unsafe fn emit_driver_manifest_error(
    create_info: &VkInstanceCreateInfo<'_>,
    path: &Path,
    error: discovery::DriverManifestError,
    environment_override: bool,
) {
    match error {
        discovery::DriverManifestError::FailedOpen => unsafe {
            let path = if environment_override {
                path.as_os_str()
            } else {
                path.file_name().unwrap_or(path.as_os_str())
            };
            let path = Path::new(path).display();
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                format_args!("loader_get_json: Failed to open JSON file {path}"),
            );
        },
        discovery::DriverManifestError::InvalidJson => unsafe {
            let path = path.display();
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                format_args!("loader_get_json: Invalid JSON file {path}."),
            );
        },
        discovery::DriverManifestError::MissingFileFormatVersion => unsafe {
            let path = path.display();
            emit_driver_category_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "loader_parse_icd_manifest: ICD JSON {path} does not have a 'file_format_version' field. Skipping ICD JSON."
                ),
            );
        },
        discovery::DriverManifestError::EmptyLibraryPath { manifest_version } => unsafe {
            let path = path.display();
            emit_driver_only_create_message(
                create_info,
                format_args!(
                    "Found ICD manifest file {path}, version {}.{}.{}",
                    vk::VK_API_VERSION_MAJOR(manifest_version),
                    vk::VK_API_VERSION_MINOR(manifest_version),
                    vk::VK_API_VERSION_PATCH(manifest_version),
                ),
            );
            if manifest_version >= vk::VK_MAKE_API_VERSION(0, 1, 0, 2) {
                emit_driver_category_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format_args!(
                        "loader_parse_icd_manifest: {path} has unknown icd manifest file version {}.{}.{}. May cause errors.",
                        vk::VK_API_VERSION_MAJOR(manifest_version),
                        vk::VK_API_VERSION_MINOR(manifest_version),
                        vk::VK_API_VERSION_PATCH(manifest_version),
                    ),
                );
            }
            emit_driver_category_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "loader_parse_icd_manifest: ICD JSON {path} 'library_path' field is empty. Skipping ICD JSON."
                ),
            );
        },
        discovery::DriverManifestError::Invalid | discovery::DriverManifestError::OutOfMemory => {}
    }
}

#[cold]
pub(crate) unsafe fn emit_driver_manifest_found(
    create_info: &VkInstanceCreateInfo<'_>,
    manifest: &discovery::DriverManifest,
) {
    unsafe {
        emit_driver_only_create_message(
            create_info,
            format_args!(
                "Found ICD manifest file {}, version {}.{}.{}",
                manifest.manifest_path.display(),
                vk::VK_API_VERSION_MAJOR(manifest.manifest_version),
                vk::VK_API_VERSION_MINOR(manifest.manifest_version),
                vk::VK_API_VERSION_PATCH(manifest.manifest_version),
            ),
        );
    };
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn scan_icds(
    create_info: &VkInstanceCreateInfo<'_>,
    settings: Option<&discovery::LoaderSettings>,
) -> Result<Vec<ScannedIcdRecord>, VkResult> {
    let (exclusive, direct_drivers) = unsafe { scan_direct_drivers(create_info) }?;
    let scan = (!exclusive).then(|| discovery::scan_drivers_with_settings(settings));
    if let Some(scan) = &scan {
        unsafe { emit_driver_scan_preamble(create_info, scan) };
    }
    if pending::json_allocation_failed() {
        return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
    }
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
    let mut scanned_icds = Vec::new();
    let capacity = scan
        .as_ref()
        .map_or(0, |scan| scan.manifests.len())
        .checked_add(direct_drivers.len())
        .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    scanned_icds
        .try_reserve_exact(capacity)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    // Upstream adds direct drivers before manifest drivers; physical-device
    // enumeration walks the resulting ICD list in reverse insertion order.
    for direct_driver in direct_drivers {
        scanned_icds.push(ScannedIcdRecord {
            icd: direct_driver,
            version_status: ManifestApiVersionStatus::Consistent,
        });
    }
    if let Some(scan) = &scan {
        for (path, disposition) in &scan.candidates {
            unsafe { emit_driver_candidate_diagnostics(create_info, scan, path, *disposition) };
            if *disposition != discovery::DriverDisposition::Accepted {
                continue;
            }
            let Some(manifest) = scan
                .manifests
                .iter()
                .find(|manifest| manifest.manifest_path == *path)
            else {
                continue;
            };
            if vk::VK_API_VERSION_VARIANT(manifest.api_version) != 0
                || !manifest.architecture_supported
            {
                continue;
            }
            if manifest.portability_driver && !portability_enabled {
                skipped_portability_drivers = true;
                continue;
            }
            if let Some(icd) = unsafe { load_scanned_icd(manifest, create_info) }? {
                scanned_icds.push(icd);
            }
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
                emit_driver_category_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    format_args!("{message}"),
                );
            };
        }
        unsafe {
            emit_driver_category_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                format_args!("vkCreateInstance: Found no drivers!"),
            );
        };
        Err(VkResult::ERROR_INCOMPATIBLE_DRIVER)
    } else {
        Ok(scanned_icds)
    }
}

pub(crate) struct ScannedIcdRecord {
    icd: ScannedIcd,
    version_status: ManifestApiVersionStatus,
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn load_scanned_icd(
    manifest: &discovery::DriverManifest,
    create_info: &VkInstanceCreateInfo<'_>,
) -> Result<Option<ScannedIcdRecord>, VkResult> {
    let (icd, version_status, uses_deprecated_interface) = match ScannedIcd::load(manifest) {
        Ok(loaded) => loaded,
        Err(ScannedIcdLoadError::OpenLibrary {
            message,
            wrong_bit_type,
        }) => {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format_args!("{message}"),
                );
            };
            if wrong_bit_type {
                unsafe {
                    emit_driver_only_create_message(
                        create_info,
                        format_args!(
                            "Requested ICD {} was wrong bit-type. Ignoring this JSON",
                            manifest.library_path.display()
                        ),
                    );
                };
            }
            return Ok(None);
        }
        Err(ScannedIcdLoadError::MissingPrefixedGetInstanceProcAddr(interface_version)) => {
            unsafe {
                emit_driver_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    format_args!(
                        "loader_scanned_icd_add: ICD {} reports an interface version of {interface_version} but doesn't export vk_icdGetInstanceProcAddr, skip this ICD.",
                        manifest.library_path.display()
                    ),
                );
                emit_driver_category_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    format_args!(
                        "loader_icd_scan: Failed loading library associated with ICD JSON {}. Ignoring this JSON",
                        manifest.library_path.display()
                    ),
                );
            }
            return Ok(None);
        }
        Err(ScannedIcdLoadError::InvalidInterface) => return Ok(None),
        Err(ScannedIcdLoadError::OutOfMemory) => return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY),
        Err(ScannedIcdLoadError::MutexInitialization(error)) => return Err(error),
    };
    if manifest.library_path.is_absolute()
        && icd.library_path() != Some(manifest.library_path.as_path())
        && !platform::path_normalizes(&manifest.library_path)?
    {
        unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                format_args!(
                    "normalize_path: Call to realpath() failed with error code 2 when given the path {}",
                    manifest.library_path.display()
                ),
            );
            if let Some(loaded_path) = icd.library_path() {
                emit_layer_category_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "Path to given binary {} was found to differ from OS loaded path {}",
                        manifest.library_path.display(),
                        loaded_path.display()
                    ),
                );
            }
        }
    }
    if uses_deprecated_interface {
        unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "loader_scanned_icd_add: Using deprecated ICD interface of 'vkGetInstanceProcAddr' instead of 'vk_icdGetInstanceProcAddr' for ICD {}",
                    manifest.library_path.display()
                ),
            );
        };
    }
    Ok(Some(ScannedIcdRecord {
        icd,
        version_status,
    }))
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn create_icd_instances(
    scanned_icds: Vec<ScannedIcdRecord>,
    create_info: &VkInstanceCreateInfo<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    requested_api_version: u32,
    has_device_configurations: bool,
) -> Result<Vec<IcdInstance>, VkResult> {
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
            emit_driver_category_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                format_args!("terminator_CreateInstance: Found no drivers!"),
            );
        };
        Err(VkResult::ERROR_INCOMPATIBLE_DRIVER)
    } else {
        Ok(icds)
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
pub(crate) unsafe fn emit_icd_version_status(
    create_info: &VkInstanceCreateInfo<'_>,
    scanned: &ScannedIcdRecord,
) {
    let library_path = scanned
        .icd
        .library_path()
        .unwrap_or_else(|| Path::new("<direct driver>"))
        .display();
    match scanned.version_status {
        ManifestApiVersionStatus::Consistent => {}
        ManifestApiVersionStatus::EnumerateInstanceVersionMissing => unsafe {
            emit_driver_category_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "terminator_CreateInstance: Manifest ICD for \"{library_path}\" contained a 1.1 or greater API version, but does not support vkEnumerateInstanceVersion, treating as a 1.0 ICD",
                ),
            );
        },
        ManifestApiVersionStatus::EnumerateInstanceVersionReturned(version) => unsafe {
            emit_driver_category_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "terminator_CreateInstance: Manifest ICD for \"{}\" contained a 1.1 or greater API version, but vkEnumerateInstanceVersion returned {}.{}, treating as a 1.0 ICD",
                    library_path,
                    vk::VK_API_VERSION_MAJOR(version),
                    vk::VK_API_VERSION_MINOR(version),
                ),
            );
        },
    }
}

pub(crate) const fn icd_create_application_api_version(
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
pub(crate) unsafe fn create_scanned_icd_instance(
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
    let icd_extension_names = unsafe {
        filtered_icd_instance_extensions(
            create_info,
            requested_api_version,
            icd.api_version,
            supports,
        )
    }?;
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
    let unknown_physical_device_dispatch = unknown::UnknownDispatchTable::try_new()?;
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
                unsafe {
                    discard_incomplete_icd_instance(
                        create_info,
                        &icd,
                        dispatch_ref,
                        handle,
                        allocator,
                    );
                };
                return Ok(false);
            }
            unsafe {
                core::ptr::addr_of_mut!((*output).icd).write(icd);
                core::ptr::addr_of_mut!((*output).handle).write(handle);
                core::ptr::addr_of_mut!((*output).enabled_extensions).write(enabled_extensions);
                core::ptr::addr_of_mut!((*output).unknown_physical_device_dispatch)
                    .write(unknown_physical_device_dispatch);
                IcdInstance::initialize_active(output);
            }
            Ok(true)
        }
        VkResult::ERROR_OUT_OF_HOST_MEMORY => Err(VkResult::ERROR_OUT_OF_HOST_MEMORY),
        _ => Ok(false),
    }
}

pub(crate) unsafe fn scanned_icd_instance_extensions(
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

pub(crate) fn destroy_icd_instances(
    icds: &[IcdInstance],
    allocator: *const VkAllocationCallbacks<'_>,
) {
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
    debug::messenger::destroy_all(instance, allocator);
    destroy_all_surfaces(instance);
    destroy_icd_instances(&instance.icds, allocator);
}

#[cold]
unsafe fn emit_instance_configuration(
    create_info_ref: &VkInstanceCreateInfo<'_>,
    settings: Option<&discovery::LoaderSettings>,
    invalid_api_version: Option<u32>,
    api_version: u32,
) -> Result<(), VkResult> {
    if let Some(settings) = &settings {
        let display_path = diagnostics::settings_path(settings.settings_file_path())?;
        diagnostics::with_message(
            format_args!("Using layer configurations found in loader settings from {display_path}"),
            |message| {
                // SAFETY: The caller retains the complete instance-create pNext chain.
                unsafe {
                    debug::messenger::submit_instance_create_message(
                        create_info_ref,
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                        message,
                    );
                };
            },
        );
    } else if !discovery::loader_settings_file_present() {
        unsafe {
            emit_driver_create_message(
                create_info_ref,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format_args!(
                    "No valid vk_loader_settings.json file found, no loader settings will be active"
                ),
            );
        };
    }
    if let Some(requested) = invalid_api_version {
        unsafe {
            emit_driver_create_message(
                create_info_ref,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                format_args!(
                    "VkInstanceCreateInfo::pApplicationInfo::apiVersion has value of {requested} which is not permitted. If apiVersion is not 0, then it must be greater than or equal to the value of VK_API_VERSION_1_0 [VUID-VkApplicationInfo-apiVersion]"
                ),
            );
        };
    }
    let portability_flag = create_info_ref
        .flags
        .intersects(vk::VkInstanceCreateFlagBits::ENUMERATE_PORTABILITY_BIT_KHR);
    let portability_extension = unsafe {
        instance_extension_enabled(
            create_info_ref,
            vk::VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME,
        )
    };
    if portability_flag && portability_extension {
        // SAFETY: The caller's complete create-info chain remains live.
        unsafe {
            emit_driver_create_message(
                create_info_ref,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format_args!(
                    "Portability enumeration bit was set, enumerating portability drivers."
                ),
            );
        };
    }
    let variant = vk::VK_API_VERSION_VARIANT(api_version);
    if variant != 0 {
        // SAFETY: The caller's complete create-info chain remains live.
        unsafe {
            emit_driver_create_message(
                create_info_ref,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "vkCreateInstance: The API Variant specified in pCreateInfo->pApplicationInfo.apiVersion is {variant} instead of the expected value of 0."
                ),
            );
        };
    }
    Ok(())
}

#[cold]
unsafe fn emit_direct_driver_error(
    create_info: &VkInstanceCreateInfo<'_>,
    index: usize,
    error: &DirectIcdError,
) {
    let emit = |message: core::fmt::Arguments<'_>| {
        // SAFETY: The caller retains the instance-create chain throughout this scan.
        unsafe {
            emit_driver_category_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                message,
            );
        }
    };
    match error {
        DirectIcdError::MissingNegotiate => emit(format_args!(
            "loader_add_direct_driver: Could not get 'vk_icdNegotiateLoaderICDInterfaceVersion' from VkDirectDriverLoadingInfoLUNARG structure at index {index}, skipping."
        )),
        DirectIcdError::IncompatibleInterface(version) => emit(format_args!(
            "loader_add_direct_driver: VkDirectDriverLoadingInfoLUNARG structure at index {index} supports interface version {version}, which is incompatible with the Loader Driver Interface version that supports the VK_LUNARG_direct_driver_loading extension, skipping."
        )),
        DirectIcdError::MissingCreateInstance => emit(format_args!(
            "loader_add_direct_driver: Could not get 'vkCreateInstance' from VkDirectDriverLoadingInfoLUNARG structure at index {index}, skipping."
        )),
        DirectIcdError::MissingEnumerateExtensions => emit(format_args!(
            "loader_add_direct_driver: Could not get 'vkEnumerateInstanceExtensionProperties' from VkDirectDriverLoadingInfoLUNARG structure at index {index}, skipping."
        )),
        DirectIcdError::EnumerateVersion(_) | DirectIcdError::MutexInitialization(_) => {}
    }
}

#[cold]
unsafe fn discard_incomplete_icd_instance(
    create_info: &VkInstanceCreateInfo<'_>,
    icd: &ScannedIcd,
    dispatch_ref: &InstanceDispatchTable,
    handle: VkInstance,
    allocator: *const VkAllocationCallbacks<'_>,
) {
    if dispatch_ref.vkGetPhysicalDeviceFeatures.is_none()
        && let Some(path) = icd.library_path()
    {
        unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "Unable to load vkGetPhysicalDeviceFeatures from ICD {}",
                    path.display()
                ),
            );
        };
    }
    if let Some(path) = icd.library_path() {
        unsafe {
            emit_driver_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "terminator_CreateInstance: Failed to find required entrypoints in ICD {}. Skipping this driver.",
                    path.display()
                ),
            );
        };
    }
    let destroy: Option<PFN_vkDestroyInstance> = unsafe {
        load_typed((icd.get_instance_proc_addr)(
            handle,
            c"vkDestroyInstance".as_ptr(),
        ))
    };
    if let Some(destroy) = destroy {
        unsafe { destroy(handle, allocator) };
    }
}

unsafe fn finish_instance_creation(
    mut loader_instance: alloc::boxed::Box<LoaderInstance>,
    create_info_ref: &VkInstanceCreateInfo<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    instance: *mut VkInstance,
) -> VkResult {
    if loader_instance.layers.is_empty() {
        let previous = pending::replace_instance(loader_instance.handle());
        // SAFETY: The public entrypoint validated the output and create-info
        // pointers, and the pending handle identifies this unregistered box.
        unsafe { instance.write(loader_instance.handle()) };
        let result =
            unsafe { layer::create_instance_terminator(create_info_ref, allocator, instance) };
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
unsafe fn filtered_icd_instance_extensions(
    create_info: &VkInstanceCreateInfo<'_>,
    requested_api_version: u32,
    driver_api_version: u32,
    supports: impl Fn(*const core::ffi::c_char) -> bool,
) -> Result<Vec<*const core::ffi::c_char>, VkResult> {
    let mut icd_extension_names = Vec::new();
    if create_info.enabledExtensionCount != 0 {
        let capacity = (create_info.enabledExtensionCount as usize)
            .checked_add(1)
            .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        icd_extension_names
            .try_reserve_exact(capacity)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        for index in 0..create_info.enabledExtensionCount as usize {
            let name = unsafe { create_info.ppEnabledExtensionNames.add(index).read() };
            if supports(name) {
                icd_extension_names.push(name);
            }
        }
    }
    let needs_sort_properties_extension = LINUX_SORT_PLATFORM_ENABLED
        && linux_sort_requires_properties_extension(requested_api_version, driver_api_version);
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
    Ok(icd_extension_names)
}
