//! Loader-layer discovery, interface negotiation, and chain ABI.

use core::{
    ffi::{CStr, c_char, c_void},
    ptr,
};
use std::{env, ffi::CString};

use vk::{
    PFN_vkEnumerateDeviceExtensionProperties, PFN_vkGetDeviceProcAddr, PFN_vkGetInstanceProcAddr,
    PFN_vkVoidFunction, VkExtensionProperties, VkInstanceCreateInfo, VkResult, VkStructureType,
};

use crate::{
    collections::HashSet,
    discovery::{
        LayerExtension, LayerManifest, LayerSearch, LoaderSettings, discover_layers,
        discover_layers_with_settings, valid_layer_mask,
    },
    platform::LoaderLibrary,
};

pub(crate) type GetPhysicalDeviceProcAddr =
    unsafe extern "system" fn(vk::VkInstance, *const c_char) -> PFN_vkVoidFunction;
type NegotiateLoaderLayerInterfaceVersion =
    unsafe extern "system" fn(*mut NegotiateLayerInterface) -> VkResult;

const CURRENT_LAYER_INTERFACE_VERSION: u32 = 2;
const NEGOTIATE_INTERFACE_STRUCT: u32 = 1;

#[repr(C)]
struct NegotiateLayerInterface {
    s_type: u32,
    p_next: *mut c_void,
    loader_layer_interface_version: u32,
    get_instance_proc_addr: Option<PFN_vkGetInstanceProcAddr>,
    get_device_proc_addr: Option<PFN_vkGetDeviceProcAddr>,
    get_physical_device_proc_addr: Option<GetPhysicalDeviceProcAddr>,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub(crate) enum LayerFunction {
    LinkInfo = 0,
    LoaderDataCallback = 1,
    LayerCreateDeviceCallback = 2,
    LoaderFeatures = 3,
}

#[repr(C)]
pub(crate) struct LayerInstanceLink {
    pub(crate) next: *mut Self,
    pub(crate) next_get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    pub(crate) next_get_physical_device_proc_addr: GetPhysicalDeviceProcAddr,
}

pub(crate) type SetInstanceLoaderData =
    unsafe extern "system" fn(vk::VkInstance, *mut c_void) -> VkResult;
pub(crate) type LayerCreateDevice = unsafe extern "system" fn(
    vk::VkInstance,
    vk::VkPhysicalDevice,
    *const vk::VkDeviceCreateInfo<'_>,
    *const vk::VkAllocationCallbacks<'_>,
    *mut vk::VkDevice,
    PFN_vkGetInstanceProcAddr,
    *mut PFN_vkGetDeviceProcAddr,
) -> VkResult;
pub(crate) type LayerDestroyDevice = unsafe extern "system" fn(
    vk::VkDevice,
    *const vk::VkAllocationCallbacks<'_>,
    vk::PFN_vkDestroyDevice,
);

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union LayerInstanceCreateInfoUnion {
    pub(crate) layer_info: *mut LayerInstanceLink,
    pub(crate) set_instance_loader_data: SetInstanceLoaderData,
    pub(crate) layer_device: LayerDeviceCallbacks,
    pub(crate) loader_features: vk::VkFlags,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct LayerDeviceCallbacks {
    pub(crate) create_device: LayerCreateDevice,
    pub(crate) destroy_device: LayerDestroyDevice,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct LayerInstanceCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) next: *const c_void,
    pub(crate) function: LayerFunction,
    pub(crate) value: LayerInstanceCreateInfoUnion,
}

#[repr(C)]
pub(crate) struct LayerDeviceLink {
    pub(crate) next: *mut Self,
    pub(crate) next_get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    pub(crate) next_get_device_proc_addr: PFN_vkGetDeviceProcAddr,
}

pub(crate) type SetDeviceLoaderData =
    unsafe extern "system" fn(vk::VkDevice, *mut c_void) -> VkResult;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union LayerDeviceCreateInfoUnion {
    pub(crate) layer_info: *mut LayerDeviceLink,
    pub(crate) set_device_loader_data: SetDeviceLoaderData,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct LayerDeviceCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) next: *const c_void,
    pub(crate) function: LayerFunction,
    pub(crate) value: LayerDeviceCreateInfoUnion,
}

pub(crate) struct LoadedLayer {
    _library: LoaderLibrary,
    pub(crate) name: CString,
    pub(crate) library_path: std::path::PathBuf,
    manifest_path: std::path::PathBuf,
    disable_environment: Option<std::ffi::OsString>,
    enabled_by: &'static str,
    pub(crate) implicit: bool,
    pub(crate) get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    pub(crate) get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    pub(crate) get_physical_device_proc_addr: Option<GetPhysicalDeviceProcAddr>,
    pub(crate) device_extensions: Box<[LayerExtension]>,
}

pub(crate) struct ActiveLayers {
    pub(crate) loaded: Box<[LoadedLayer]>,
    pub(crate) reported: Box<[ActiveLayerProperty]>,
    pub(crate) requested: Box<[CString]>,
}

pub(crate) struct SelectedLayers {
    manifests: Box<[LayerManifest]>,
    selected: Box<[usize]>,
    reported: Box<[ActiveLayerProperty]>,
    requested: Box<[CString]>,
    environment_count: usize,
}

impl SelectedLayers {
    pub(crate) fn supports_instance_extension(&self, name: &CStr) -> bool {
        self.selected.iter().any(|&index| {
            self.manifests[index]
                .instance_extensions
                .iter()
                .any(|extension| extension.name.as_c_str() == name)
        })
    }
}

pub(crate) struct ActiveLayerProperty {
    name: CString,
    manifest_path: std::path::PathBuf,
    api_version: u32,
    implementation_version: u32,
    description: CString,
}

impl From<&LayerManifest> for ActiveLayerProperty {
    fn from(manifest: &LayerManifest) -> Self {
        Self {
            name: manifest.name.clone(),
            manifest_path: manifest.manifest_path.clone(),
            api_version: manifest.api_version,
            implementation_version: manifest.implementation_version,
            description: manifest.description.clone(),
        }
    }
}

#[repr(C, align(8))]
struct DeviceCreateSentinel {
    magic: u64,
    padding: [u8; 120],
}

#[cold]
#[inline(never)]
fn fatal_layer_policy(message: impl core::fmt::Display) -> ! {
    crate::platform::write_stderr(&format!("{message}\n"));
    // SAFETY: Fatal loader diagnostics terminate immediately, matching
    // upstream's C abort path.
    unsafe { libc::abort() }
}

#[cfg(windows)]
fn format_layer_pointer(pointer: *mut core::ffi::c_void) -> String {
    // MSVC's `%p`, used by the upstream Windows loader and encoded in its
    // tests, is an uppercase, zero-padded integer without an `0x` prefix.
    format!(
        "{:01$X}",
        pointer as usize,
        core::mem::size_of::<usize>() * 2
    )
}

#[cfg(not(windows))]
fn format_layer_pointer(pointer: *mut core::ffi::c_void) -> String {
    format!("{pointer:p}")
}

#[derive(Clone, Copy)]
enum LayerLoadError {
    WrongBitType,
    Failed,
}

impl LoadedLayer {
    fn load(manifest: &LayerManifest, enabled_by: &'static str) -> Result<Self, LayerLoadError> {
        let path = manifest
            .library_path
            .as_ref()
            .ok_or(LayerLoadError::Failed)?;
        // SAFETY: The library is retained for the lifetime of every copied symbol.
        let library = unsafe { LoaderLibrary::open(path) }.map_err(|error| {
            if error.is_wrong_bit_type() {
                LayerLoadError::WrongBitType
            } else {
                LayerLoadError::Failed
            }
        })?;
        let negotiate_name = manifest
            .functions
            .negotiate
            .as_deref()
            .unwrap_or(c"vkNegotiateLoaderLayerInterfaceVersion");
        // SAFETY: The manifest or layer ABI defines the symbol's signature.
        let negotiate = unsafe {
            library
                .get::<NegotiateLoaderLayerInterfaceVersion>(negotiate_name.to_bytes_with_nul())
                .ok()
                .map(|symbol| *symbol)
        };

        let mut negotiated = NegotiateLayerInterface {
            s_type: NEGOTIATE_INTERFACE_STRUCT,
            p_next: ptr::null_mut(),
            loader_layer_interface_version: CURRENT_LAYER_INTERFACE_VERSION,
            get_instance_proc_addr: None,
            get_device_proc_addr: None,
            get_physical_device_proc_addr: None,
        };
        if let Some(negotiate) = negotiate {
            // SAFETY: `negotiated` has the C layout required by `vk_layer.h`.
            if unsafe { negotiate(&raw mut negotiated) } != VkResult::SUCCESS
                || negotiated.loader_layer_interface_version == 0
            {
                return Err(LayerLoadError::Failed);
            }
        }

        let negotiated_functions = negotiate.is_some()
            && negotiated.loader_layer_interface_version >= CURRENT_LAYER_INTERFACE_VERSION;

        let get_instance_proc_addr = negotiated_functions
            .then_some(negotiated.get_instance_proc_addr)
            .flatten()
            .or_else(|| {
                let name = manifest
                    .functions
                    .get_instance_proc_addr
                    .as_deref()
                    .unwrap_or(c"vkGetInstanceProcAddr");
                // SAFETY: The layer ABI defines this symbol's signature.
                unsafe {
                    library
                        .get::<PFN_vkGetInstanceProcAddr>(name.to_bytes_with_nul())
                        .ok()
                        .map(|symbol| *symbol)
                }
            })
            .ok_or(LayerLoadError::Failed)?;
        let get_device_proc_addr = negotiated_functions
            .then_some(negotiated.get_device_proc_addr)
            .flatten()
            .or_else(|| {
                let name = manifest
                    .functions
                    .get_device_proc_addr
                    .as_deref()
                    .unwrap_or(c"vkGetDeviceProcAddr");
                // SAFETY: The layer ABI defines this symbol's signature.
                unsafe {
                    library
                        .get::<PFN_vkGetDeviceProcAddr>(name.to_bytes_with_nul())
                        .ok()
                        .map(|symbol| *symbol)
                }
            })
            .ok_or(LayerLoadError::Failed)?;
        let get_physical_device_proc_addr = negotiated_functions
            .then_some(negotiated.get_physical_device_proc_addr)
            .flatten();

        Ok(Self {
            _library: library,
            name: manifest.name.clone(),
            library_path: path.clone(),
            manifest_path: manifest.manifest_path.clone(),
            disable_environment: manifest
                .disable_environment
                .as_ref()
                .map(|environment| environment.0.clone()),
            enabled_by,
            implicit: manifest.implicit,
            get_instance_proc_addr,
            get_device_proc_addr,
            get_physical_device_proc_addr,
            device_extensions: manifest.device_extensions.clone(),
        })
    }
}

impl Drop for LoadedLayer {
    fn drop(&mut self) {
        crate::platform::write_loader_log_with_category(
            "debug",
            "DEBUG",
            "layer",
            "LAYER",
            format_args!(
                "Unloading layer library {}",
                self.library_path.to_string_lossy()
            ),
        );
    }
}

fn requested_layer_names(
    create_info: &VkInstanceCreateInfo<'_>,
) -> Option<(Box<[CString]>, usize)> {
    if create_info.enabledLayerCount != 0 && create_info.ppEnabledLayerNames.is_null() {
        return None;
    }
    let mut names = Vec::new();
    if let Some(environment) = env::var_os("VK_INSTANCE_LAYERS") {
        for name in env::split_paths(&environment) {
            names.push(CString::new(name.to_string_lossy().as_bytes()).ok()?);
        }
    }
    let environment_count = names.len();
    for index in 0..create_info.enabledLayerCount as usize {
        // SAFETY: Vulkan requires this array and every string to be live.
        let name = unsafe { create_info.ppEnabledLayerNames.add(index).read() };
        if name.is_null() {
            return None;
        }
        names.push(unsafe { CStr::from_ptr(name) }.to_owned());
    }
    Some((names.into_boxed_slice(), environment_count))
}

fn environment_matches(environment: &(std::ffi::OsString, std::ffi::OsString)) -> bool {
    env::var_os(&environment.0).is_some_and(|value| value == environment.1)
}

fn wildcard_matches(pattern: &[u8], name: &[u8]) -> bool {
    let (mut pattern_index, mut name_index) = (0, 0);
    let (mut star, mut retry) = (None, 0);
    while name_index < name.len() {
        if pattern_index < pattern.len()
            && pattern[pattern_index] != b'*'
            && pattern[pattern_index].eq_ignore_ascii_case(&name[name_index])
        {
            pattern_index += 1;
            name_index += 1;
        } else if pattern_index < pattern.len() && pattern[pattern_index] == b'*' {
            star = Some(pattern_index);
            pattern_index += 1;
            retry = name_index;
        } else if let Some(star_index) = star {
            pattern_index = star_index + 1;
            retry += 1;
            name_index = retry;
        } else {
            return false;
        }
    }
    while pattern_index < pattern.len() && pattern[pattern_index] == b'*' {
        pattern_index += 1;
    }
    pattern_index == pattern.len()
}

fn filter_matches(variable: &str, name: &CStr) -> bool {
    if crate::platform::has_elevated_privileges() {
        return false;
    }
    env::var(variable).is_ok_and(|filters| {
        filters.split(',').any(|filter| {
            filter.eq_ignore_ascii_case("~all~")
                || wildcard_matches(filter.as_bytes(), name.to_bytes())
        })
    })
}

fn forced_enabled(manifest: &LayerManifest) -> bool {
    filter_matches("VK_LOADER_LAYERS_ENABLE", &manifest.name)
}

fn forced_disabled(manifest: &LayerManifest) -> bool {
    let disabled = !crate::platform::has_elevated_privileges()
        && env::var("VK_LOADER_LAYERS_DISABLE").is_ok_and(|filters| {
            filters.split(',').any(|filter| {
                filter.eq_ignore_ascii_case("~all~")
                    || filter == "*"
                    || filter == "**"
                    || (manifest.implicit && filter.eq_ignore_ascii_case("~implicit~"))
                    || (!manifest.implicit && filter.eq_ignore_ascii_case("~explicit~"))
                    || wildcard_matches(filter.as_bytes(), manifest.name.to_bytes())
            })
        });
    disabled && !filter_matches("VK_LOADER_LAYERS_ALLOW", &manifest.name)
}

fn naturally_enabled(manifest: &LayerManifest) -> bool {
    manifest.implicit
        && manifest
            .enable_environment
            .as_ref()
            .is_none_or(environment_matches)
}

pub(crate) fn implicit_manifest_is_active(manifest: &LayerManifest) -> bool {
    if !manifest.implicit {
        return false;
    }
    let natural = naturally_enabled(manifest);
    let filtered = if forced_enabled(manifest) {
        true
    } else {
        natural && !forced_disabled(manifest)
    };
    filtered
        && manifest
            .disable_environment
            .as_ref()
            .is_none_or(|environment| env::var_os(&environment.0).is_none())
}

fn available_layer_mask(manifests: &[LayerManifest]) -> Box<[bool]> {
    let mut available = valid_layer_mask(manifests);
    let blacklist = manifests
        .iter()
        .zip(available.iter())
        .find(|(manifest, valid)| {
            **valid
                && manifest.name.as_c_str() == c"VK_LAYER_LUNARG_override"
                && implicit_manifest_is_active(manifest)
        })
        .map(|(manifest, _)| manifest.blacklisted_layers.as_ref());
    if let Some(blacklist) = blacklist {
        for (manifest, available) in manifests.iter().zip(available.iter_mut()) {
            if blacklist.iter().any(|name| name == &manifest.name) {
                *available = false;
            }
        }
    }
    available
}

fn meta_reaches(manifests: &[LayerManifest], from: usize, target: usize) -> bool {
    let mut visited = vec![false; manifests.len()];
    let mut pending = vec![from];
    while let Some(index) = pending.pop() {
        if index == target {
            return true;
        }
        if visited[index] {
            continue;
        }
        visited[index] = true;
        for name in &manifests[index].component_layers {
            if let Some(component) = manifests
                .iter()
                .position(|candidate| candidate.name == *name)
            {
                pending.push(component);
            }
        }
    }
    false
}

fn emit_create_message(
    create_info: &VkInstanceCreateInfo<'_>,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: String,
) {
    let (filter, prefix) = if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR {
        ("error", "[Vulkan Loader] ERROR:          ")
    } else if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING {
        ("warn", "[Vulkan Loader] WARNING:        ")
    } else if severity == vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO {
        ("info", "[Vulkan Loader] INFO:           ")
    } else {
        ("debug", "[Vulkan Loader] DEBUG:          ")
    };
    if crate::platform::loader_debug_filter_enabled(filter) {
        crate::platform::write_stderr(&format!("{prefix}{message}\n"));
    }
    let Ok(message) = CString::new(message) else {
        return;
    };
    // SAFETY: The caller's complete create-info chain is live during discovery.
    unsafe {
        crate::debug_messenger::submit_instance_create_message(create_info, severity, &message);
    };
}

fn emit_layer_only_message(create_info: &VkInstanceCreateInfo<'_>, message: impl AsRef<str>) {
    let message = message.as_ref();
    crate::platform::write_loader_category_log("layer", "LAYER", format_args!("{message}"));
    let Ok(message) = CString::new(message) else {
        return;
    };
    // Category-only loader messages map to informational debug-utils messages.
    unsafe {
        crate::debug_messenger::submit_instance_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            &message,
        );
    };
}

fn emit_layer_message(
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
    crate::platform::write_loader_log_with_category(
        filter,
        label,
        "layer",
        "LAYER",
        format_args!("{message}"),
    );
    let Ok(message) = CString::new(message) else {
        return;
    };
    // SAFETY: The create-info chain remains live throughout layer activation.
    unsafe {
        crate::debug_messenger::submit_instance_create_message(create_info, severity, &message);
    };
}

fn emit_layer_search_diagnostics(
    create_info: &VkInstanceCreateInfo<'_>,
    searches: &[LayerSearch],
    manifests: &[LayerManifest],
) {
    for search in searches {
        let kind = if search.implicit {
            "implicit"
        } else {
            "explicit"
        };
        emit_layer_only_message(
            create_info,
            format!("Searching for {kind} layer manifest files"),
        );
        emit_layer_only_message(create_info, "   In following locations:");
        for root in &search.roots {
            emit_layer_only_message(create_info, format!("      {}", root.to_string_lossy()));
        }
        if search.files.is_empty() {
            emit_layer_only_message(create_info, "   Found no files");
        } else {
            emit_layer_only_message(create_info, "   Found the following files:");
            for file in &search.files {
                emit_layer_only_message(create_info, format!("      {}", file.to_string_lossy()));
            }
        }
        for manifest in manifests
            .iter()
            .filter(|manifest| manifest.implicit == search.implicit)
        {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format!(
                    "Found manifest file {} (file version {}.{}.{})",
                    manifest.manifest_path.to_string_lossy(),
                    vk::VK_API_VERSION_MAJOR(manifest.manifest_version),
                    vk::VK_API_VERSION_MINOR(manifest.manifest_version),
                    vk::VK_API_VERSION_PATCH(manifest.manifest_version),
                ),
            );
        }
    }
}

/// Emits pre-instance layer discovery diagnostics when no instance-create
/// callback chain exists yet.
pub(crate) fn emit_global_layer_search_diagnostics(searches: &[LayerSearch]) {
    for search in searches {
        let kind = if search.implicit {
            "implicit"
        } else {
            "explicit"
        };
        crate::platform::write_loader_category_log(
            "layer",
            "LAYER",
            format_args!("Searching for {kind} layer manifest files"),
        );
        crate::platform::write_loader_category_log(
            "layer",
            "LAYER",
            format_args!("   In following locations:"),
        );
        for root in &search.roots {
            crate::platform::write_loader_category_log(
                "layer",
                "LAYER",
                format_args!("      {}", root.to_string_lossy()),
            );
        }
        if search.files.is_empty() {
            crate::platform::write_loader_category_log(
                "layer",
                "LAYER",
                format_args!("   Found no files"),
            );
        } else {
            crate::platform::write_loader_category_log(
                "layer",
                "LAYER",
                format_args!("   Found the following files:"),
            );
            for file in &search.files {
                crate::platform::write_loader_category_log(
                    "layer",
                    "LAYER",
                    format_args!("      {}", file.to_string_lossy()),
                );
            }
        }
    }
}

pub(crate) fn emit_instance_layer_callstack(
    create_info: &VkInstanceCreateInfo<'_>,
    layers: &[LoadedLayer],
) {
    emit_layer_only_message(create_info, "vkCreateInstance layer callstack setup to:");
    emit_layer_only_message(create_info, "   <Application>");
    emit_layer_only_message(create_info, "     ||");
    emit_layer_only_message(create_info, "   <Loader>");
    emit_layer_only_message(create_info, "     ||");
    for layer in layers {
        emit_layer_only_message(create_info, format!("   {}", layer.name.to_string_lossy()));
        emit_layer_only_message(
            create_info,
            format!(
                "           Type: {}",
                if layer.implicit {
                    "Implicit"
                } else {
                    "Explicit"
                }
            ),
        );
        emit_layer_only_message(
            create_info,
            format!("           Enabled By: {}", layer.enabled_by),
        );
        if layer.implicit
            && let Some(disable_environment) = &layer.disable_environment
        {
            emit_layer_only_message(
                create_info,
                format!(
                    "               Disable Env Var:  {}",
                    disable_environment.to_string_lossy()
                ),
            );
        }
        emit_layer_only_message(
            create_info,
            format!(
                "           Manifest: {}",
                layer.manifest_path.to_string_lossy()
            ),
        );
        emit_layer_only_message(
            create_info,
            format!(
                "           Library:  {}",
                layer.library_path.to_string_lossy()
            ),
        );
        emit_layer_only_message(create_info, "     ||");
    }
    emit_layer_only_message(create_info, "   <Drivers>");
}

fn emit_meta_layer_diagnostics(
    create_info: &VkInstanceCreateInfo<'_>,
    manifests: &[LayerManifest],
) {
    let valid = valid_layer_mask(manifests);
    for (meta_index, meta) in manifests.iter().enumerate() {
        if meta.component_layers.is_empty() {
            continue;
        }
        for (component_index, component_name) in meta.component_layers.iter().enumerate() {
            let Some(component_manifest_index) = manifests
                .iter()
                .position(|candidate| candidate.name == *component_name)
            else {
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format!(
                        "verify_meta_layer_component_layers: Meta-layer {} can't find component layer {} at index {}.  Skipping this layer.",
                        meta.name.to_string_lossy(),
                        component_name.to_string_lossy(),
                        component_index
                    ),
                );
                break;
            };
            if component_manifest_index == meta_index {
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format!(
                        "verify_meta_layer_component_layers: Meta-layer {} lists itself in its component layer list at index {}.  Skipping this layer.",
                        meta.name.to_string_lossy(),
                        component_index
                    ),
                );
                break;
            }
            let component = &manifests[component_manifest_index];
            let meta_major = vk::VK_API_VERSION_MAJOR(meta.api_version);
            let meta_minor = vk::VK_API_VERSION_MINOR(meta.api_version);
            let component_major = vk::VK_API_VERSION_MAJOR(component.api_version);
            let component_minor = vk::VK_API_VERSION_MINOR(component.api_version);
            if component_major < meta_major
                || (component_major == meta_major && component_minor < meta_minor)
            {
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format!(
                        "verify_meta_layer_component_layers: Meta-layer uses API version {meta_major}.{meta_minor}, but component layer {component_index} has API version {component_major}.{component_minor} that is lower.  Skipping this layer."
                    ),
                );
                break;
            }
            if !component.component_layers.is_empty() {
                if meta_reaches(manifests, component_manifest_index, meta_index) {
                    emit_create_message(
                        create_info,
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                        format!(
                            "verify_meta_layer_component_layers: Recursive dependency between Meta-layer {} and  Meta-layer {}.  Skipping this layer.",
                            meta.name.to_string_lossy(),
                            component.name.to_string_lossy()
                        ),
                    );
                    emit_create_message(
                        create_info,
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                        format!(
                            "loader_add_meta_layer: Meta-layer {} recursively references itself through its component layers. Skipping the meta-layer.",
                            meta.name.to_string_lossy()
                        ),
                    );
                    break;
                }
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format!(
                        "verify_meta_layer_component_layers: Adding meta-layer {} which also contains meta-layer {}",
                        meta.name.to_string_lossy(),
                        component.name.to_string_lossy()
                    ),
                );
            }
            if valid[meta_index] {
                for extension in &component.instance_extensions {
                    emit_create_message(
                        create_info,
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                        format!(
                            "Meta-layer {} component layer {} adding instance extension {}",
                            meta.name.to_string_lossy(),
                            component.name.to_string_lossy(),
                            extension.name.to_string_lossy()
                        ),
                    );
                }
                for extension in &component.device_extensions {
                    emit_create_message(
                        create_info,
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                        format!(
                            "Meta-layer {} component layer {} adding device extension {}",
                            meta.name.to_string_lossy(),
                            component.name.to_string_lossy(),
                            extension.name.to_string_lossy()
                        ),
                    );
                }
            }
        }
        if !valid[meta_index] {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                format!(
                    "Removing meta-layer {} from instance layer list since it appears invalid.",
                    meta.name.to_string_lossy()
                ),
            );
        }
    }
}

#[cold]
#[inline(never)]
pub(crate) fn select_active_layers(
    create_info: &VkInstanceCreateInfo<'_>,
    settings: Option<&LoaderSettings>,
) -> Result<SelectedLayers, VkResult> {
    let (requested, environment_count) =
        requested_layer_names(create_info).ok_or(VkResult::ERROR_LAYER_NOT_PRESENT)?;
    if environment_count != 0 {
        let names = requested[..environment_count]
            .iter()
            .map(|name| name.to_string_lossy())
            .collect::<Vec<_>>()
            .join(":");
        emit_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format!("env var 'VK_INSTANCE_LAYERS' defined and adding layers: {names}"),
        );
    }
    let manifests = discover_layers_with_settings(settings);
    emit_layer_search_diagnostics(create_info, manifests.searches(), &manifests);
    for manifest in &manifests {
        let manifest_major = vk::VK_API_VERSION_MAJOR(manifest.manifest_version);
        let manifest_minor = vk::VK_API_VERSION_MINOR(manifest.manifest_version);
        let manifest_patch = vk::VK_API_VERSION_PATCH(manifest.manifest_version);
        let known_manifest_version = manifest_major == 1
            && ((manifest_minor == 0 && manifest_patch < 2)
                || (manifest_minor == 1 && manifest_patch < 3)
                || (manifest_minor == 2 && manifest_patch < 2));
        if !known_manifest_version {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format!(
                    "loader_add_layer_properties: {} has unknown layer manifest file version {manifest_major}.{manifest_minor}.{manifest_patch}.  May cause errors.",
                    manifest.manifest_path.to_string_lossy(),
                ),
            );
        }
        let variant = vk::VK_API_VERSION_VARIANT(manifest.api_version);
        if variant != 0 {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format!(
                    "Layer \"{}\" has an 'api_version' field which contains a non-zero variant value of {variant}.  Skipping Layer.",
                    manifest.name.to_string_lossy(),
                ),
            );
        }
        if !manifest.architecture_supported {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format!(
                    "The library architecture in layer {} doesn't match the current running architecture, skipping this layer",
                    manifest.manifest_path.to_string_lossy(),
                ),
            );
        }
        if !manifest.override_paths.is_empty()
            && manifest.manifest_version < vk::VK_MAKE_API_VERSION(0, 1, 1, 0)
        {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format!(
                    "Layer \"{}\" contains meta-layer-specific override paths, but using older JSON file version.",
                    manifest.name.to_string_lossy()
                ),
            );
        }
        let natural = naturally_enabled(manifest);
        if forced_enabled(manifest) && !natural {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format!(
                    "Layer \"{}\" forced enabled due to env var 'VK_LOADER_LAYERS_ENABLE'.",
                    manifest.name.to_string_lossy()
                ),
            );
        } else if !forced_enabled(manifest) && forced_disabled(manifest) {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format!(
                    "Layer \"{}\" forced disabled because name matches filter of env var 'VK_LOADER_LAYERS_DISABLE'.",
                    manifest.name.to_string_lossy()
                ),
            );
        }
    }
    emit_meta_layer_diagnostics(create_info, &manifests);
    let valid = available_layer_mask(&manifests);
    if let Some(override_layer) =
        manifests
            .iter()
            .zip(valid.iter())
            .find_map(|(manifest, valid)| {
                (*valid
                    && manifest.name.as_c_str() == c"VK_LAYER_LUNARG_override"
                    && implicit_manifest_is_active(manifest))
                .then_some(manifest)
            })
    {
        for blacklisted in &override_layer.blacklisted_layers {
            if manifests
                .iter()
                .any(|manifest| manifest.name == *blacklisted)
            {
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                    format!(
                        "loader_remove_layers_in_blacklist: Override layer is active and layer {} is in the blacklist inside of it. Removing that layer from current layer list.",
                        blacklisted.to_string_lossy()
                    ),
                );
            }
        }
    }
    if let Some(override_layer) =
        manifests
            .iter()
            .zip(valid.iter())
            .find_map(|(manifest, valid)| {
                (*valid
                    && manifest.name.as_c_str() == c"VK_LAYER_LUNARG_override"
                    && implicit_manifest_is_active(manifest)
                    && !manifest.override_paths.is_empty())
                .then_some(manifest)
            })
    {
        if !crate::platform::has_elevated_privileges()
            && let Ok(layer_path) = env::var("VK_LAYER_PATH")
        {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format!(
                    "Ignoring VK_LAYER_PATH. The Override layer is active and has override paths set, which takes priority. VK_LAYER_PATH is set to {layer_path}"
                ),
            );
        }
        for path in &override_layer.override_paths {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format!(
                    "Override layer has override path {}",
                    path.to_string_lossy()
                ),
            );
        }
    }
    let settings_active = manifests
        .iter()
        .any(|manifest| manifest.settings_control.is_some());
    let mut selected = Vec::new();
    let mut reported = Vec::new();
    let mut expanding = HashSet::default();
    if settings_active {
        for (manifest, valid) in manifests.iter().zip(valid.iter()) {
            if !valid {
                continue;
            }
            let requested_by_name = requested
                .iter()
                .any(|name| name.as_c_str() == manifest.name.as_c_str());
            let active = match manifest.settings_control.as_deref() {
                Some("on") => true,
                Some("off") => false,
                _ => {
                    implicit_manifest_is_active(manifest)
                        || requested_by_name
                        || forced_enabled(manifest)
                }
            };
            if active {
                let _ = activate_manifest(
                    manifest,
                    &manifests,
                    &mut selected,
                    &mut reported,
                    &mut expanding,
                );
            }
        }
    }
    for (manifest, valid) in manifests.iter().zip(valid.iter()) {
        if settings_active
            || !valid
            || !implicit_manifest_is_active(manifest)
            || requested[..environment_count]
                .iter()
                .any(|name| name == &manifest.name)
        {
            continue;
        }
        let _ = activate_manifest(
            manifest,
            &manifests,
            &mut selected,
            &mut reported,
            &mut expanding,
        );
    }
    for (manifest, valid) in manifests.iter().zip(valid.iter()) {
        if settings_active
            || !valid
            || !forced_enabled(manifest)
            || (manifest.implicit
                && manifest
                    .disable_environment
                    .as_ref()
                    .is_some_and(|environment| env::var_os(&environment.0).is_some()))
        {
            continue;
        }
        let _ = activate_manifest(
            manifest,
            &manifests,
            &mut selected,
            &mut reported,
            &mut expanding,
        );
    }
    if !settings_active {
        for (requested_index, requested_name) in requested.iter().enumerate() {
            let Some((manifest, _)) =
                manifests
                    .iter()
                    .zip(valid.iter())
                    .find(|(manifest, valid)| {
                        **valid && manifest.name.as_c_str() == requested_name.as_c_str()
                    })
            else {
                if requested_index < environment_count {
                    emit_create_message(
                        create_info,
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                        format!(
                            "Layer \"{}\" was not found but was requested by env var VK_INSTANCE_LAYERS!",
                            requested_name.to_string_lossy()
                        ),
                    );
                    continue;
                }
                return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
            };
            if forced_disabled(manifest) && !forced_enabled(manifest) {
                if requested_index < environment_count {
                    continue;
                }
                return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
            }
            if !activate_manifest(
                manifest,
                &manifests,
                &mut selected,
                &mut reported,
                &mut expanding,
            ) {
                if requested_index < environment_count {
                    continue;
                }
                return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
            }
        }
    }
    if requested[environment_count..].iter().any(|requested_name| {
        !reported
            .iter()
            .any(|layer| layer.name.as_c_str() == requested_name.as_c_str())
    }) {
        return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
    }
    Ok(SelectedLayers {
        manifests: manifests.into_vec().into_boxed_slice(),
        selected: selected.into_boxed_slice(),
        reported: reported.into_boxed_slice(),
        requested,
        environment_count,
    })
}

#[cold]
#[inline(never)]
pub(crate) fn load_selected_layers(
    create_info: &VkInstanceCreateInfo<'_>,
    selected_layers: SelectedLayers,
) -> Result<ActiveLayers, VkResult> {
    let SelectedLayers {
        manifests,
        selected,
        reported,
        requested,
        environment_count,
    } = selected_layers;
    let mut loaded = Vec::new();
    loaded
        .try_reserve_exact(selected.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut requested_layer_failed = false;

    // Upstream opens the expanded list from bottom to top while constructing
    // the GIPA chain. Reverse the finished storage so index zero remains the
    // application-facing (topmost) layer used by dispatch.
    for &index in selected.iter().rev() {
        let manifest = &manifests[index];
        if loaded
            .iter()
            .any(|layer: &LoadedLayer| layer.name == manifest.name)
        {
            continue;
        }
        let enabled_by = if manifest.settings_control.as_deref() == Some("on") {
            "Loader Settings File (Vulkan Configurator)"
        } else if requested[..environment_count]
            .iter()
            .any(|name| name.as_c_str() == manifest.name.as_c_str())
        {
            "Environment Variable VK_INSTANCE_LAYERS"
        } else if forced_enabled(manifest) {
            "Environment Variable VK_LOADER_LAYERS_ENABLE"
        } else if requested[environment_count..]
            .iter()
            .any(|name| name.as_c_str() == manifest.name.as_c_str())
        {
            "Application"
        } else if manifest.implicit {
            "Implicit Layer"
        } else {
            "Meta-layer"
        };
        match LoadedLayer::load(manifest, enabled_by) {
            Ok(layer) => {
                emit_layer_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                    format!(
                        "Loading layer library {}",
                        layer.library_path.to_string_lossy()
                    ),
                );
                emit_layer_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format!(
                        "Insert instance layer \"{}\" ({})",
                        layer.name.to_string_lossy(),
                        layer.library_path.to_string_lossy()
                    ),
                );
                loaded.push(layer);
            }
            Err(error) => {
                let explicitly_requested = requested[environment_count..]
                    .iter()
                    .any(|name| name.as_c_str() == manifest.name.as_c_str());
                let severity = if explicitly_requested {
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR
                } else {
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO
                };
                let ending = if explicitly_requested { '!' } else { '.' };
                let reason = match error {
                    LayerLoadError::WrongBitType => "was wrong bit-type",
                    LayerLoadError::Failed => "failed to load",
                };
                emit_layer_message(
                    create_info,
                    severity,
                    format!(
                        "Requested layer \"{}\" {reason}{ending}",
                        manifest.name.to_string_lossy()
                    ),
                );
                if explicitly_requested {
                    requested_layer_failed = true;
                }
            }
        }
    }
    if requested_layer_failed {
        return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
    }
    loaded.reverse();
    Ok(ActiveLayers {
        loaded: loaded.into_boxed_slice(),
        reported,
        requested,
    })
}

fn activate_manifest(
    manifest: &LayerManifest,
    manifests: &[LayerManifest],
    selected: &mut Vec<usize>,
    reported: &mut Vec<ActiveLayerProperty>,
    expanding: &mut HashSet<CString>,
) -> bool {
    if reported.iter().any(|layer| {
        layer.name == manifest.name
            && (!manifest.component_layers.is_empty()
                || layer.manifest_path == manifest.manifest_path)
    }) {
        return true;
    }
    if !expanding.insert(manifest.name.clone()) {
        return false;
    }
    let result = if manifest.component_layers.is_empty() {
        let Some(index) = manifests
            .iter()
            .position(|candidate| core::ptr::eq(candidate, manifest))
        else {
            expanding.remove(&manifest.name);
            return false;
        };
        selected.push(index);
        reported.push(manifest.into());
        true
    } else {
        let mut complete = true;
        for component_name in &manifest.component_layers {
            let Some(component) = manifests
                .iter()
                .find(|candidate| candidate.name == *component_name)
            else {
                complete = false;
                continue;
            };
            if forced_disabled(component) && !forced_enabled(component) {
                complete = false;
                continue;
            }
            if !activate_manifest(component, manifests, selected, reported, expanding) {
                complete = false;
            }
        }
        if complete {
            reported.push(manifest.into());
        }
        complete
    };
    expanding.remove(&manifest.name);
    result
}

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
    let pending = crate::pending::instance();
    if pending == vk::VkInstance::NULL {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let returned = unsafe { instance.read() };
    if returned == vk::VkInstance::NULL {
        fatal_layer_policy(
            "terminator_CreateInstance: Loader instance pointer null encountered.  Possibly set by active layer. (Policy #LLP_LAYER_21)",
        );
    }
    let magic = unsafe { crate::instance::LoaderInstance::internal_magic(returned) }.unwrap_or(0);
    if unsafe { crate::instance::LoaderInstance::from_internal_handle(returned) }.is_none() {
        let pointer = format_layer_pointer(returned.0);
        fatal_layer_policy(format!(
            "terminator_CreateInstance: Instance pointer ({pointer}) has invalid MAGIC value 0x{magic:08x}. Instance value possibly corrupted by active layer (Policy #LLP_LAYER_21).  ",
        ));
    }
    if create_info.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let Some(loader) =
        (unsafe { crate::instance::LoaderInstance::from_internal_handle_mut(pending) })
    else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    // SAFETY: The layer chain retains the effective create info and allocator
    // for this synchronous call, and `loader` is the pending instance box.
    let result = unsafe { crate::create_pending_icd_instances(loader, &*create_info, allocator) };
    if result != VkResult::SUCCESS {
        return result;
    }
    // SAFETY: The layer supplied the output pointer from the live create call.
    unsafe { instance.write(pending) };
    VkResult::SUCCESS
}

unsafe extern "system" fn set_instance_loader_data(
    instance: vk::VkInstance,
    object: *mut c_void,
) -> VkResult {
    if object.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The callback is installed only for this loader's live instance.
    let Some(instance) =
        (unsafe { crate::instance::LoaderInstance::from_internal_handle(instance) })
    else {
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

unsafe extern "system" fn layer_create_device_callback(
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
    let loader = unsafe {
        crate::instance::LoaderInstance::from_handle(instance)
            .or_else(|| crate::instance::LoaderInstance::from_internal_handle(instance))
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
            &*create_info,
            allocator,
            device,
            first,
        )
    }
}

unsafe extern "system" fn layer_destroy_device_callback(
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
        drop(crate::device::LoaderDevice::take_dispatch(dispatch));
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
        b"vkCreateInstance" => Some(crate::erase_function(
            create_instance_terminator as vk::PFN_vkCreateInstance,
        )),
        b"vkGetInstanceProcAddr" => Some(crate::erase_function(
            terminator_get_instance_proc_addr as PFN_vkGetInstanceProcAddr,
        )),
        b"vk_layerGetPhysicalDeviceProcAddr" => Some(crate::erase_function(
            terminator_get_physical_device_proc_addr as GetPhysicalDeviceProcAddr,
        )),
        b"vkCreateDevice" => Some(crate::erase_function(
            crate::create_device_terminator as vk::PFN_vkCreateDevice,
        )),
        b"vkDestroyInstance" => Some(crate::erase_function(
            crate::destroy_instance_terminator as vk::PFN_vkDestroyInstance,
        )),
        b"vkEnumeratePhysicalDevices" => Some(crate::erase_function(
            crate::terminator_enumerate_physical_devices as vk::PFN_vkEnumeratePhysicalDevices,
        )),
        b"vkEnumeratePhysicalDeviceGroups" => Some(crate::erase_function(
            crate::terminator_enumerate_physical_device_groups
                as vk::PFN_vkEnumeratePhysicalDeviceGroups,
        )),
        b"vkEnumeratePhysicalDeviceGroupsKHR" => Some(crate::erase_function(
            crate::terminator_enumerate_physical_device_groups_khr
                as vk::PFN_vkEnumeratePhysicalDeviceGroupsKHR,
        )),
        b"vkEnumerateDeviceLayerProperties" => Some(crate::erase_function(
            terminator_enumerate_device_layer_properties
                as vk::PFN_vkEnumerateDeviceLayerProperties,
        )),
        b"vkEnumerateDeviceExtensionProperties" => Some(crate::erase_function(
            terminator_enumerate_device_extension_properties
                as vk::PFN_vkEnumerateDeviceExtensionProperties,
        )),
        b"vkCreateDebugUtilsMessengerEXT" => Some(crate::erase_function(
            crate::debug_messenger::terminator_create_debug_utils_messenger
                as vk::PFN_vkCreateDebugUtilsMessengerEXT,
        )),
        b"vkCreateDebugReportCallbackEXT" => Some(crate::erase_function(
            crate::debug_messenger::terminator_create_debug_report_callback
                as vk::PFN_vkCreateDebugReportCallbackEXT,
        )),
        b"vkDestroyDebugUtilsMessengerEXT" => Some(crate::erase_function(
            crate::debug_messenger::terminator_destroy_debug_utils_messenger
                as vk::PFN_vkDestroyDebugUtilsMessengerEXT,
        )),
        b"vkDestroyDebugReportCallbackEXT" => Some(crate::erase_function(
            crate::debug_messenger::terminator_destroy_debug_report_callback
                as vk::PFN_vkDestroyDebugReportCallbackEXT,
        )),
        b"vkSubmitDebugUtilsMessageEXT" => Some(crate::erase_function(
            crate::debug_messenger::terminator_submit_debug_utils_message
                as vk::PFN_vkSubmitDebugUtilsMessageEXT,
        )),
        b"vkDebugReportMessageEXT" => Some(crate::erase_function(
            crate::debug_messenger::terminator_debug_report_message
                as vk::PFN_vkDebugReportMessageEXT,
        )),
        _ if instance == vk::VkInstance::NULL => crate::global_proc_addr(name),
        _ => crate::command_lookup(name)
            .and_then(|lookup| {
                crate::instance_terminator_proc_addr(lookup.id)
                    .or_else(|| crate::physical_device_terminator_proc_addr(lookup.id))
                    .or_else(|| crate::exported_proc_addr(lookup.id))
            })
            .or_else(|| {
                // SAFETY: During vkCreateInstance the not-yet-registered loader
                // handle is passed down the layer chain; afterwards normal
                // dispatch-table lookup identifies the registered instance.
                let instance = unsafe {
                    crate::instance::LoaderInstance::from_handle(instance)
                        .or_else(|| crate::instance::LoaderInstance::from_internal_handle(instance))
                }?;
                crate::unknown::physical_device_proc_addr(instance, name, false)
                    .or_else(|| crate::unknown::device_proc_addr(instance, name, false))
            }),
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
        return Some(crate::erase_function(
            terminator_get_device_proc_addr as PFN_vkGetDeviceProcAddr,
        ));
    }
    match name.to_bytes() {
        b"vkDestroyDevice" => Some(crate::erase_function(
            crate::destroy_device_terminator as vk::PFN_vkDestroyDevice,
        )),
        b"vkCreateSwapchainKHR" => Some(crate::erase_function(
            crate::surface::terminator_create_swapchain as vk::PFN_vkCreateSwapchainKHR,
        )),
        b"vkCreateSharedSwapchainsKHR" => Some(crate::erase_function(
            crate::surface::terminator_create_shared_swapchains
                as vk::PFN_vkCreateSharedSwapchainsKHR,
        )),
        b"vkGetDeviceGroupSurfacePresentModesKHR" => Some(crate::erase_function(
            crate::surface::terminator_get_device_group_surface_present_modes
                as vk::PFN_vkGetDeviceGroupSurfacePresentModesKHR,
        )),
        b"vkDebugMarkerSetObjectNameEXT" => Some(crate::erase_function(
            crate::debug::terminator_vkDebugMarkerSetObjectNameEXT
                as vk::PFN_vkDebugMarkerSetObjectNameEXT,
        )),
        b"vkDebugMarkerSetObjectTagEXT" => Some(crate::erase_function(
            crate::debug::terminator_vkDebugMarkerSetObjectTagEXT
                as vk::PFN_vkDebugMarkerSetObjectTagEXT,
        )),
        b"vkSetDebugUtilsObjectNameEXT" => Some(crate::erase_function(
            crate::debug::terminator_vkSetDebugUtilsObjectNameEXT
                as vk::PFN_vkSetDebugUtilsObjectNameEXT,
        )),
        b"vkSetDebugUtilsObjectTagEXT" => Some(crate::erase_function(
            crate::debug::terminator_vkSetDebugUtilsObjectTagEXT
                as vk::PFN_vkSetDebugUtilsObjectTagEXT,
        )),
        _ => {
            // SAFETY: The device was returned by the lower chain and registered
            // before control returned to the requesting layer.
            let device = unsafe { crate::device::LoaderDevice::from_handle(device) }?;
            // SAFETY: The stored ICD resolver and device originate together.
            device.resolve(name)
        }
    }
}

unsafe extern "system" fn terminator_enumerate_device_layer_properties(
    physical_device: vk::VkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut vk::VkLayerProperties,
) -> VkResult {
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The terminator receives the loader's physical-device wrapper.
    let Some(physical_device) =
        (unsafe { crate::instance::LoaderPhysicalDevice::from_handle(physical_device) })
    else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let layers = &physical_device.instance().active_layer_properties;
    unsafe { enumerate_active_device_layers(layers, property_count, properties) }
}

fn device_extension_property(extension: &LayerExtension) -> VkExtensionProperties {
    let mut property = VkExtensionProperties::DEFAULT;
    copy_c_string(&extension.name, &mut property.extensionName);
    property.specVersion = extension.spec_version;
    property
}

fn append_unique_device_extension(
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

fn named_layer_device_extensions(name: &CStr) -> Result<Vec<VkExtensionProperties>, VkResult> {
    let manifests = discover_layers();
    let valid = available_layer_mask(&manifests);
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
    let mut visited = vec![false; manifests.len()];
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
                pending.push(index);
            }
        }
    }
    Ok(extensions)
}

#[cold]
#[inline(never)]
unsafe fn enumerate_named_layer_device_extensions(
    layer_name: &CStr,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    let extensions = match named_layer_device_extensions(layer_name) {
        Ok(extensions) => extensions,
        Err(result) => return result,
    };
    let total = extensions.len().min(u32::MAX as usize) as u32;
    if properties.is_null() {
        unsafe { property_count.write(total) };
        return VkResult::SUCCESS;
    }
    let written = (unsafe { property_count.read() } as usize).min(extensions.len());
    unsafe {
        ptr::copy_nonoverlapping(extensions.as_ptr(), properties, written);
        property_count.write(written as u32);
    }
    if written < extensions.len() {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

#[cold]
#[inline(never)]
unsafe fn enumerate_icd_device_extensions(
    physical_device: &crate::instance::LoaderPhysicalDevice,
    property_count: *mut u32,
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
        let capacity = unsafe { property_count.read() };
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
                    unsafe { property_count.write(written) };
                    return VkResult::INCOMPLETE;
                }
                unsafe { properties.add(written as usize).write(property) };
                written += 1;
            }
        }
        unsafe { property_count.write(written) };
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
    unsafe { property_count.write(extensions.len().min(u32::MAX as usize) as u32) };
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
    let Some(physical_device) =
        (unsafe { crate::instance::LoaderPhysicalDevice::from_handle(physical_device) })
    else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if !layer_name.is_null() && unsafe { layer_name.read() } != 0 {
        return unsafe {
            enumerate_named_layer_device_extensions(
                CStr::from_ptr(layer_name),
                property_count,
                properties,
            )
        };
    }
    unsafe { enumerate_icd_device_extensions(physical_device, property_count, properties) }
}

pub(crate) unsafe fn enumerate_active_device_layers(
    layers: &[ActiveLayerProperty],
    property_count: *mut u32,
    properties: *mut vk::VkLayerProperties,
) -> VkResult {
    let total = layers.len().min(u32::MAX as usize) as u32;
    if properties.is_null() {
        unsafe { property_count.write(total) };
        return VkResult::SUCCESS;
    }
    let capacity = unsafe { property_count.read() } as usize;
    let written = capacity.min(layers.len());
    for (index, layer) in layers.iter().take(written).enumerate() {
        let mut property = vk::VkLayerProperties::DEFAULT;
        copy_c_string(&layer.name, &mut property.layerName);
        copy_c_string(&layer.description, &mut property.description);
        property.specVersion = layer.api_version;
        property.implementationVersion = layer.implementation_version;
        unsafe { properties.add(index).write(property) };
    }
    unsafe {
        property_count.write(written as u32);
    };
    if written < layers.len() {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

pub(crate) unsafe fn enumerate_instance_layers(
    property_count: *mut u32,
    properties: *mut vk::VkLayerProperties,
) -> VkResult {
    let discovered = discover_layers();
    emit_global_layer_search_diagnostics(discovered.searches());
    let mut manifests = discovered.into_vec();
    let valid = available_layer_mask(&manifests);
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
    let mut identities = HashSet::default();
    manifests.retain(|manifest| {
        identities.insert((
            manifest.name.clone(),
            manifest
                .component_layers
                .is_empty()
                .then(|| manifest.manifest_path.clone()),
        ))
    });
    let total = manifests.len().min(u32::MAX as usize) as u32;
    if properties.is_null() {
        unsafe { property_count.write(total) };
        return VkResult::SUCCESS;
    }
    let capacity = unsafe { property_count.read() } as usize;
    let written = capacity.min(manifests.len());
    for (index, manifest) in manifests.iter().take(written).enumerate() {
        let mut property = vk::VkLayerProperties::DEFAULT;
        copy_c_string(&manifest.name, &mut property.layerName);
        copy_c_string(&manifest.description, &mut property.description);
        property.specVersion = manifest.api_version;
        property.implementationVersion = manifest.implementation_version;
        unsafe { properties.add(index).write(property) };
    }
    unsafe {
        property_count.write(written as u32);
    };
    if written < manifests.len() {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

fn copy_c_string<const N: usize>(source: &CStr, destination: &mut [c_char; N]) {
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
        if lookup.scope != crate::CommandScope::Instance {
            return None;
        }
        return crate::physical_device_terminator_proc_addr(lookup.id)
            .or_else(|| crate::exported_proc_addr(lookup.id));
    }
    // SAFETY: Creation-time handles are internal and later handles are found
    // through their registered dispatch table.
    let instance = unsafe {
        crate::instance::LoaderInstance::from_handle(instance)
            .or_else(|| crate::instance::LoaderInstance::from_internal_handle(instance))
    }?;
    crate::unknown::physical_device_proc_addr(instance, name, false)
}

/// Executes the activated instance-layer chain around a pre-created loader instance.
///
/// # Safety
///
/// The create structures and output pointer must satisfy `vkCreateInstance`'s
/// contract, and every loaded layer must remain live throughout the call.
pub(crate) unsafe fn create_instance_chain(
    instance: &mut crate::instance::LoaderInstance,
    create_info: &VkInstanceCreateInfo<'_>,
    allocator: *const vk::VkAllocationCallbacks<'_>,
    output: *mut vk::VkInstance,
) -> VkResult {
    debug_assert!(!instance.layers.is_empty());
    let count = instance.layers.len();
    let mut links = Box::<[LayerInstanceLink]>::new_uninit_slice(count);
    let links_ptr = links.as_mut_ptr().cast::<LayerInstanceLink>();
    let mut next_gpdpa = terminator_get_physical_device_proc_addr as GetPhysicalDeviceProcAddr;
    for index in (0..count).rev() {
        let next_gipa = instance.layers.get(index + 1).map_or(
            terminator_get_instance_proc_addr as PFN_vkGetInstanceProcAddr,
            |layer| layer.get_instance_proc_addr,
        );
        // SAFETY: `links_ptr` has `count` writable entries and is stable in its box.
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
    // SAFETY: Every element was initialized exactly once above.
    let links = unsafe { links.assume_init() };
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
    let previous = crate::pending::replace_instance(instance.handle());
    unsafe { output.write(instance.handle()) };
    // SAFETY: The chain nodes, links, and caller structures remain live for the call.
    let result = unsafe { create(&raw const layered_create_info, allocator, output) };
    crate::pending::replace_instance(previous);
    if result == VkResult::SUCCESS {
        // SAFETY: The layer returned this handle and its negotiated resolvers
        // remain loaded in `instance.layers`.
        unsafe { instance.load_dispatch(top.get_instance_proc_addr, next_gpdpa, output.read()) };
    }
    drop(links);
    result
}

fn extension_property_name(property: &VkExtensionProperties) -> Option<CString> {
    let chars = property.extensionName.as_slice();
    // SAFETY: `c_char` is exactly one byte on every supported C ABI.
    let bytes = unsafe { core::slice::from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len()) };
    let end = bytes.iter().position(|byte| *byte == 0)?;
    CString::new(&bytes[..end]).ok()
}

/// Collects device extensions advertised by active layer manifests and code.
///
/// # Safety
///
/// `physical_device` must be a live wrapper belonging to `instance`.
pub(crate) unsafe fn available_device_extensions(
    instance: &crate::instance::LoaderInstance,
    physical_device: vk::VkPhysicalDevice,
) -> Result<Box<[CString]>, VkResult> {
    let mut names: Vec<CString> = instance
        .layers
        .iter()
        .flat_map(|layer| {
            layer
                .device_extensions
                .iter()
                .map(|extension| extension.name.clone())
        })
        .collect();
    let Some(top) = instance.layers.first() else {
        return Ok(names.into_boxed_slice());
    };
    // SAFETY: The top layer remains loaded and the name has static storage.
    let enumerate: Option<PFN_vkEnumerateDeviceExtensionProperties> = unsafe {
        crate::load_typed((top.get_instance_proc_addr)(
            instance.chain_handle(),
            c"vkEnumerateDeviceExtensionProperties".as_ptr(),
        ))
    };
    let Some(enumerate) = enumerate else {
        return Ok(names.into_boxed_slice());
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
    let mut properties = Box::<[VkExtensionProperties]>::new_uninit_slice(capacity);
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
    names.extend(
        properties[..initialized]
            .iter()
            // SAFETY: The enumeration initialized the reported leading entries.
            .filter_map(|property| extension_property_name(unsafe { property.assume_init_ref() })),
    );
    names.sort_unstable_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    names.dedup_by(|left, right| left.as_bytes() == right.as_bytes());
    Ok(names.into_boxed_slice())
}

unsafe extern "system" fn set_device_loader_data(
    device: vk::VkDevice,
    object: *mut c_void,
) -> VkResult {
    if object.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The callback is installed only in a live loader-created chain.
    let Some(device) = (unsafe { crate::device::LoaderDevice::from_handle(device) }) else {
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
    instance: &crate::instance::LoaderInstance,
    physical_device: vk::VkPhysicalDevice,
    create_info: &vk::VkDeviceCreateInfo<'_>,
    allocator: *const vk::VkAllocationCallbacks<'_>,
    output: *mut vk::VkDevice,
) -> VkResult {
    unsafe {
        create_device_chain_from(instance, physical_device, create_info, allocator, output, 0)
    }
}

unsafe fn create_device_chain_from(
    instance: &crate::instance::LoaderInstance,
    physical_device: vk::VkPhysicalDevice,
    create_info: &vk::VkDeviceCreateInfo<'_>,
    allocator: *const vk::VkAllocationCallbacks<'_>,
    output: *mut vk::VkDevice,
    first: usize,
) -> VkResult {
    let layers = &instance.layers[first..];
    let count = layers.len();
    let mut links = Box::<[LayerDeviceLink]>::new_uninit_slice(count);
    let links_ptr = links.as_mut_ptr().cast::<LayerDeviceLink>();
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
    // SAFETY: Every boxed element was initialized above.
    let links = unsafe { links.assume_init() };
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
    let sentinel = Box::new(DeviceCreateSentinel {
        magic: crate::DEVICE_DISPATCH_MAGIC,
        padding: [0; 120],
    });
    let sentinel_address = core::ptr::from_ref(sentinel.as_ref()) as usize;
    let mut created_device = vk::VkDevice(sentinel_address as *mut c_void);
    crate::pending::push_device_sentinel(sentinel_address);
    crate::pending::push_created_device_slot();
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
        let created = crate::pending::pop_created_device();
        debug_assert!(created.is_none());
        let popped = crate::pending::pop_device_sentinel();
        debug_assert_eq!(popped, Some(sentinel_address));
        return VkResult::ERROR_LAYER_NOT_PRESENT;
    };
    // SAFETY: Every chain node and caller-owned structure remains live for the call.
    let result = unsafe {
        create(
            physical_device,
            &raw const layered_create_info,
            allocator,
            &raw mut created_device,
        )
    };
    let created_dispatch = crate::pending::pop_created_device();
    let popped = crate::pending::pop_device_sentinel();
    debug_assert_eq!(popped, Some(sentinel_address));
    if result == VkResult::SUCCESS {
        // SAFETY: The terminator created this loader device and no aliasing call exists yet.
        let Some(device) = (unsafe {
            created_dispatch.and_then(|key| crate::device::LoaderDevice::from_dispatch_key_mut(key))
        }) else {
            return VkResult::ERROR_INITIALIZATION_FAILED;
        };
        // SAFETY: Device creation has not returned to the application and the
        // top layer returned this live chain handle.
        unsafe { device.set_chain(created_device, top_device_proc_addr) };
        // The public output is committed only after the full create chain and
        // dispatch initialization complete successfully, matching upstream.
        unsafe { output.write(created_device) };
    } else if let Some(dispatch) = created_dispatch {
        // A lower layer or the terminator may have created a device before an
        // upper layer failed. Upstream owns and tears down that partial chain;
        // it must never escape through the caller's output parameter.
        let dispatch = dispatch as *const crate::LayerDeviceDispatchTable;
        // As in `loader_layer_create_device`, free the loader record but do not
        // call into the failed chain: a layer which returned failure owns the
        // cleanup of any lower device it successfully created.
        drop(crate::device::LoaderDevice::take_dispatch(dispatch));
    }
    drop(links);
    drop(sentinel);
    result
}

pub(crate) unsafe fn validate_pending_device_output(output: *mut vk::VkDevice) {
    let Some(expected) = crate::pending::device_sentinel() else {
        return;
    };
    let returned = unsafe { output.read() };
    if returned == vk::VkDevice::NULL {
        fatal_layer_policy(
            "terminator_CreateDevice: Loader device pointer null encountered.  Possibly set by active layer. (Policy #LLP_LAYER_22)",
        );
    }
    if returned.0 as usize != expected {
        let pointer = format_layer_pointer(returned.0);
        fatal_layer_policy(format!(
            "terminator_CreateDevice: Device pointer ({pointer}) has invalid MAGIC value 0x00000000. The expected value is 0x10ADED040410ADED. Device value possibly corrupted by active layer (Policy #LLP_LAYER_22).  ",
        ));
    }
    let magic = unsafe { returned.0.cast::<u64>().read() };
    if magic != crate::DEVICE_DISPATCH_MAGIC {
        let pointer = format_layer_pointer(returned.0);
        fatal_layer_policy(format!(
            "terminator_CreateDevice: Device pointer ({pointer}) has invalid MAGIC value 0x{magic:08x}. The expected value is 0x10ADED040410ADED. Device value possibly corrupted by active layer (Policy #LLP_LAYER_22).  ",
        ));
    }
}

#[cfg(test)]
mod tests {
    use core::mem::{offset_of, size_of};

    use super::*;

    #[test]
    fn duplicate_device_extension_retains_first_property_like_upstream() {
        let mut extensions = Vec::new();
        append_unique_device_extension(
            &mut extensions,
            &device_extension_property(&LayerExtension {
                name: c"VK_EXT_debug_marker".to_owned(),
                spec_version: 1,
                entrypoints: Box::default(),
            }),
        )
        .unwrap();
        append_unique_device_extension(
            &mut extensions,
            &device_extension_property(&LayerExtension {
                name: c"VK_EXT_debug_marker".to_owned(),
                spec_version: 99,
                entrypoints: Box::default(),
            }),
        )
        .unwrap();

        assert_eq!(extensions.len(), 1);
        assert_eq!(extensions[0].specVersion, 1);
    }

    #[test]
    fn layer_interface_layout_matches_vk_layer_h() {
        assert_eq!(offset_of!(NegotiateLayerInterface, s_type), 0);
        assert_eq!(offset_of!(LayerInstanceLink, next), 0);
        assert_eq!(offset_of!(LayerInstanceCreateInfo, s_type), 0);
        assert_eq!(offset_of!(LayerDeviceLink, next), 0);
        assert_eq!(offset_of!(LayerDeviceCreateInfo, s_type), 0);

        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(size_of::<NegotiateLayerInterface>(), 48);
            assert_eq!(offset_of!(NegotiateLayerInterface, p_next), 8);
            assert_eq!(
                offset_of!(NegotiateLayerInterface, loader_layer_interface_version),
                16
            );
            assert_eq!(
                offset_of!(NegotiateLayerInterface, get_instance_proc_addr),
                24
            );
            assert_eq!(
                offset_of!(NegotiateLayerInterface, get_device_proc_addr),
                32
            );
            assert_eq!(
                offset_of!(NegotiateLayerInterface, get_physical_device_proc_addr),
                40
            );

            assert_eq!(size_of::<LayerInstanceLink>(), 24);
            assert_eq!(
                offset_of!(LayerInstanceLink, next_get_instance_proc_addr),
                8
            );
            assert_eq!(
                offset_of!(LayerInstanceLink, next_get_physical_device_proc_addr),
                16
            );

            assert_eq!(size_of::<LayerInstanceCreateInfo>(), 40);
            assert_eq!(offset_of!(LayerInstanceCreateInfo, next), 8);
            assert_eq!(offset_of!(LayerInstanceCreateInfo, function), 16);
            assert_eq!(offset_of!(LayerInstanceCreateInfo, value), 24);

            assert_eq!(size_of::<LayerDeviceLink>(), 24);
            assert_eq!(offset_of!(LayerDeviceLink, next_get_instance_proc_addr), 8);
            assert_eq!(offset_of!(LayerDeviceLink, next_get_device_proc_addr), 16);

            assert_eq!(size_of::<LayerDeviceCreateInfo>(), 32);
            assert_eq!(offset_of!(LayerDeviceCreateInfo, next), 8);
            assert_eq!(offset_of!(LayerDeviceCreateInfo, function), 16);
            assert_eq!(offset_of!(LayerDeviceCreateInfo, value), 24);
        }

        #[cfg(target_pointer_width = "32")]
        {
            assert_eq!(size_of::<NegotiateLayerInterface>(), 24);
            assert_eq!(offset_of!(NegotiateLayerInterface, p_next), 4);
            assert_eq!(
                offset_of!(NegotiateLayerInterface, loader_layer_interface_version),
                8
            );
            assert_eq!(
                offset_of!(NegotiateLayerInterface, get_instance_proc_addr),
                12
            );
            assert_eq!(
                offset_of!(NegotiateLayerInterface, get_device_proc_addr),
                16
            );
            assert_eq!(
                offset_of!(NegotiateLayerInterface, get_physical_device_proc_addr),
                20
            );

            assert_eq!(size_of::<LayerInstanceLink>(), 12);
            assert_eq!(
                offset_of!(LayerInstanceLink, next_get_instance_proc_addr),
                4
            );
            assert_eq!(
                offset_of!(LayerInstanceLink, next_get_physical_device_proc_addr),
                8
            );

            assert_eq!(size_of::<LayerInstanceCreateInfo>(), 20);
            assert_eq!(offset_of!(LayerInstanceCreateInfo, next), 4);
            assert_eq!(offset_of!(LayerInstanceCreateInfo, function), 8);
            assert_eq!(offset_of!(LayerInstanceCreateInfo, value), 12);

            assert_eq!(size_of::<LayerDeviceLink>(), 12);
            assert_eq!(offset_of!(LayerDeviceLink, next_get_instance_proc_addr), 4);
            assert_eq!(offset_of!(LayerDeviceLink, next_get_device_proc_addr), 8);

            assert_eq!(size_of::<LayerDeviceCreateInfo>(), 16);
            assert_eq!(offset_of!(LayerDeviceCreateInfo, next), 4);
            assert_eq!(offset_of!(LayerDeviceCreateInfo, function), 8);
            assert_eq!(offset_of!(LayerDeviceCreateInfo, value), 12);
        }
    }

    #[test]
    fn layer_function_discriminants_match_vk_layer_h() {
        assert_eq!(LayerFunction::LinkInfo as u32, 0);
        assert_eq!(LayerFunction::LoaderDataCallback as u32, 1);
        assert_eq!(LayerFunction::LayerCreateDeviceCallback as u32, 2);
        assert_eq!(LayerFunction::LoaderFeatures as u32, 3);
    }
}
