//! Loader-layer discovery, interface negotiation, and chain ABI.

mod activation;
mod chain;
mod diagnostics;
mod library;
#[cfg(test)]
mod tests;

use activation::{
    MetaTraversal, available_layer_mask, forced_disabled, forced_enabled, naturally_enabled,
};
#[cfg(test)]
use activation::{append_windows_layer_names, requested_layer_names};
pub(crate) use activation::{
    emit_selected_layer_activation_diagnostics, implicit_manifest_is_active, load_selected_layers,
    select_active_layers,
};

#[cfg(test)]
use chain::append_layer_device_extension;
pub(crate) use chain::{
    available_device_extensions, create_device_chain, create_instance_chain,
    create_instance_terminator, enumerate_active_device_layers, enumerate_instance_layers,
    has_mismatched_device_layers, terminator_enumerate_device_extension_properties,
    terminator_enumerate_device_layer_properties, terminator_get_device_proc_addr,
    terminator_get_instance_proc_addr, terminator_get_physical_device_proc_addr,
    validate_pending_device_output,
};

pub(crate) use diagnostics::{
    ManifestVersion, MetaDiagnosticSink, emit_explicit_pre_instance_warning,
    emit_found_manifest_version, emit_global_layer_manifest_diagnostic,
    emit_global_layer_search_diagnostics, emit_instance_layer_callstack,
    emit_meta_layer_encountered, emit_nonconforming_layer_name,
};
use diagnostics::{
    compatibility_manifest_graph, emit_create_message, emit_layer_message,
    emit_layer_search_diagnostics, emit_meta_layer_diagnostics,
};

use alloc::ffi::CString;
use core::{
    ffi::{CStr, c_char, c_void},
    ptr,
};
use std::{ffi::OsStr, path::Path};
use vk::{
    PFN_vkEnumerateDeviceExtensionProperties, PFN_vkGetDeviceProcAddr, PFN_vkGetInstanceProcAddr,
    PFN_vkVoidFunction, VkExtensionProperties, VkInstanceCreateInfo, VkResult, VkStructureType,
};

use crate::{
    allocation,
    collections::ScratchArray,
    discovery::{
        LayerControl, LayerExtension, LayerManifest, LayerSearch, LoaderSettings, discover_layers,
        discover_layers_with_settings, valid_layer_mask,
    },
    platform::{self, LoaderLibrary},
};

const STACK_LAYER_LINKS: usize = 8;

#[derive(Clone, Copy)]
enum LayerFilterVariable {
    Enable,
    Disable,
    Allow,
}

impl LayerFilterVariable {
    const fn c_name(self) -> &'static CStr {
        match self {
            Self::Enable => c"VK_LOADER_LAYERS_ENABLE",
            Self::Disable => c"VK_LOADER_LAYERS_DISABLE",
            Self::Allow => c"VK_LOADER_LAYERS_ALLOW",
        }
    }
}

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
    library: Option<LoaderLibrary>,
    manifest_index: usize,
    pub(crate) name: CString,
    load_path: std::path::PathBuf,
    pub(crate) library_path: std::path::PathBuf,
    manifest_path: std::path::PathBuf,
    enable_environment: Option<(std::ffi::OsString, std::ffi::OsString)>,
    disable_environment: Option<std::ffi::OsString>,
    enabled_by: LayerEnabledBy,
    pub(crate) implicit: bool,
    pub(crate) get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    pub(crate) get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    pub(crate) get_physical_device_proc_addr: Option<GetPhysicalDeviceProcAddr>,
    pub(crate) device_extensions: Box<[LayerExtension]>,
}

#[derive(Clone, Copy)]
pub(super) enum LayerEnabledBy {
    Settings,
    MetaSettings,
    InstanceEnvironment,
    Implicit,
    LoaderEnvironment,
    Application,
    MetaLayer,
}

impl LayerEnabledBy {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Settings => "Loader Settings File (Vulkan Configurator)",
            Self::MetaSettings => "Meta Layer (Vulkan Configurator)",
            Self::InstanceEnvironment => "Environment Variable VK_INSTANCE_LAYERS",
            Self::Implicit => "Implicit Layer",
            Self::LoaderEnvironment => "Environment Variable VK_LOADER_LAYERS_ENABLE",
            Self::Application => "By the Application",
            Self::MetaLayer => "Meta-layer",
        }
    }
}

pub(crate) struct ActiveLayers {
    pub(crate) loaded: Box<[LoadedLayer]>,
    pub(crate) reported: Box<[ActiveLayerProperty]>,
    pub(crate) requested: Box<[CString]>,
}

pub(crate) fn unload_layers(layers: &mut [LoadedLayer]) {
    let mut previous = None;
    loop {
        let next = layers
            .iter()
            .enumerate()
            .filter(|(_, layer)| previous.is_none_or(|index| layer.manifest_index > index))
            .min_by_key(|(_, layer)| layer.manifest_index)
            .map(|(index, layer)| (index, layer.manifest_index));
        let Some((index, manifest_index)) = next else {
            break;
        };
        layers[index].unload();
        previous = Some(manifest_index);
    }
}

pub(crate) struct SelectedLayers {
    manifests: Box<[LayerManifest]>,
    selected: Vec<usize>,
    reported: Box<[ActiveLayerProperty]>,
    requested: Box<[CString]>,
    environment_count: usize,
    activation_messages: Vec<String>,
    activation_error_messages: Vec<String>,
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

impl ActiveLayerProperty {
    fn try_new(manifest: &LayerManifest) -> Result<Self, VkResult> {
        Ok(Self {
            name: allocation::try_c_string(&manifest.name)?,
            manifest_path: allocation::try_path(&manifest.manifest_path)?,
            api_version: manifest.api_version,
            implementation_version: manifest.implementation_version,
            description: allocation::try_c_string(&manifest.description)?,
        })
    }
}

#[repr(C, align(8))]
struct DeviceCreateSentinel {
    magic: u64,
    padding: [u8; 120],
}

#[cold]
#[inline(never)]
fn fatal_layer_policy(message: core::fmt::Arguments<'_>) -> ! {
    platform::write_stderr_fmt(message);
    platform::write_stderr("\n");
    // SAFETY: Fatal loader diagnostics terminate immediately, matching
    // upstream's C abort path.
    unsafe { libc::abort() }
}

struct LayerPointer(*mut core::ffi::c_void);

struct LayerNames<'a>(&'a [CString]);

impl core::fmt::Display for LayerNames<'_> {
    fn fmt(&self, output: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (index, name) in self.0.iter().enumerate() {
            if index != 0 {
                output.write_str(":")?;
            }
            write!(
                output,
                "{}",
                crate::debug::diagnostics::LossyBytes(name.to_bytes())
            )?;
        }
        Ok(())
    }
}

impl core::fmt::Display for LayerPointer {
    fn fmt(&self, output: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(windows)]
        {
            // MSVC's `%p`, used by the upstream Windows loader and encoded in its
            // tests, is an uppercase, zero-padded integer without an `0x` prefix.
            write!(
                output,
                "{:01$X}",
                self.0 as usize,
                core::mem::size_of::<usize>() * 2
            )
        }
        #[cfg(not(windows))]
        {
            write!(output, "{:p}", self.0)
        }
    }
}

enum LayerLoadError {
    Initialization(VkResult),
    OutOfMemory,
    OpenLibrary {
        message: String,
        wrong_bit_type: bool,
    },
    Failed,
}

impl From<VkResult> for LayerLoadError {
    fn from(error: VkResult) -> Self {
        match error {
            VkResult::ERROR_OUT_OF_HOST_MEMORY => Self::OutOfMemory,
            error => Self::Initialization(error),
        }
    }
}
