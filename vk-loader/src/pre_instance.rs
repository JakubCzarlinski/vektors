//! `vk_layer.h` pre-instance enumeration chains.

use crate::LoaderPathExt;
use crate::{
    collections::ScratchArray,
    discovery::{
        self, LayerManifest, LoaderSettings, discover_implicit_layers_with_settings,
        valid_layer_mask,
    },
    layer, pending,
    platform::{self, LoaderLibrary, LogFilter},
};
use alloc::vec::Vec;
use core::ptr;
use core::{ffi::CStr, mem};
use vk::{
    PFN_vkEnumerateInstanceExtensionProperties, VkExtensionProperties, VkInstance,
    VkLayerProperties, VkResult,
};

const CURRENT_CHAIN_VERSION: u32 = 1;
const CHAIN_TYPE_EXTENSION_PROPERTIES: u32 = 1;
const CHAIN_TYPE_LAYER_PROPERTIES: u32 = 2;
const CHAIN_TYPE_INSTANCE_VERSION: u32 = 3;

fn update_global_loader_settings() -> Option<LoaderSettings> {
    let settings = discovery::global_loader_settings();
    if settings.is_none() && !discovery::loader_settings_file_present() {
        platform::write_loader_log(
            LogFilter::Info,
            format_args!(
                "No valid vk_loader_settings.json file found, no loader settings will be active"
            ),
        );
    }
    settings
}

fn emit_layer_searches(layers: &discovery::DiscoveredLayers) {
    layer::emit_global_layer_search_diagnostics(layers, true);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ChainHeader {
    type_: u32,
    version: u32,
    size: u32,
}

type EnumerateExtensionProperties = unsafe extern "system" fn(
    *const ExtensionPropertiesChain,
    *const core::ffi::c_char,
    *mut u32,
    *mut VkExtensionProperties,
) -> VkResult;

#[repr(C)]
#[derive(Clone, Copy)]
struct ExtensionPropertiesChain {
    header: ChainHeader,
    next_function: EnumerateExtensionProperties,
    next_link: *const Self,
}

type EnumerateLayerProperties = unsafe extern "system" fn(
    *const LayerPropertiesChain,
    *mut u32,
    *mut VkLayerProperties,
) -> VkResult;

#[repr(C)]
#[derive(Clone, Copy)]
struct LayerPropertiesChain {
    header: ChainHeader,
    next_function: EnumerateLayerProperties,
    next_link: *const Self,
}

type EnumerateVersion = unsafe extern "system" fn(*const VersionChain, *mut u32) -> VkResult;

#[repr(C)]
#[derive(Clone, Copy)]
struct VersionChain {
    header: ChainHeader,
    next_function: EnumerateVersion,
    next_link: *const Self,
}

struct LoadedFunction<'a> {
    _library: LoaderLibrary,
    library_path: &'a std::path::Path,
    function: unsafe extern "system" fn(),
}

impl Drop for LoadedFunction<'_> {
    fn drop(&mut self) {
        platform::write_loader_log_with_category(
            LogFilter::Debug,
            LogFilter::Layer,
            format_args!(
                "Unloading layer library {}",
                self.library_path.loader_display()
            ),
        );
    }
}

fn is_enabled_implicit(manifest: &LayerManifest) -> bool {
    layer::implicit_manifest_is_active(manifest)
}

#[derive(Clone, Copy)]
enum PreInstanceFunction {
    ExtensionProperties,
    LayerProperties,
    Version,
}

impl PreInstanceFunction {
    fn name(self, manifest: &LayerManifest) -> Option<&CStr> {
        match self {
            Self::ExtensionProperties => manifest
                .pre_instance_functions
                .extension_properties
                .as_deref(),
            Self::LayerProperties => manifest.pre_instance_functions.layer_properties.as_deref(),
            Self::Version => manifest.pre_instance_functions.version.as_deref(),
        }
    }
}

fn load_functions(
    manifests: &[LayerManifest],
    function: PreInstanceFunction,
) -> Result<Vec<LoadedFunction<'_>>, VkResult> {
    let valid = valid_layer_mask(manifests)?;
    let mut functions = Vec::new();
    for (manifest, valid) in manifests.iter().zip(valid.iter()) {
        if !valid || !is_enabled_implicit(manifest) {
            continue;
        }
        let Some(name) = function.name(manifest) else {
            continue;
        };
        let Some(path) = manifest.library_path() else {
            continue;
        };
        // SAFETY: The library is retained beside the copied function pointer.
        let library = match unsafe { LoaderLibrary::open(path) } {
            Ok(library) => library,
            Err(error) if error.is_out_of_memory() => {
                return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
            }
            Err(_) => continue,
        };
        platform::write_loader_log_with_category(
            LogFilter::Debug,
            LogFilter::Layer,
            format_args!("Loading layer library {}", path.loader_display()),
        );
        // SAFETY: Vulkan function pointers have a common representation. The
        // selected manifest field determines the concrete ABI restored by the
        // corresponding typed chain below.
        let function = unsafe {
            library
                .get::<unsafe extern "system" fn()>(name.to_bytes_with_nul())
                .ok()
                .map(|symbol| *symbol)
        };
        if let Some(function) = function {
            functions
                .try_reserve(1)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            functions.push(LoadedFunction {
                _library: library,
                library_path: path,
                function,
            });
        }
    }
    Ok(functions)
}

const fn header(type_: u32, size: usize) -> ChainHeader {
    assert!(size <= u32::MAX as usize);
    ChainHeader {
        type_,
        version: CURRENT_CHAIN_VERSION,
        size: size as u32,
    }
}

unsafe extern "system" fn extension_terminator(
    _chain: *const ExtensionPropertiesChain,
    layer_name: *const core::ffi::c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    unsafe { enumerate_extension_properties_terminator(layer_name, property_count, properties) }
}

fn extension_name(property: &VkExtensionProperties) -> &[core::ffi::c_char] {
    let end = property
        .extensionName
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(property.extensionName.len());
    &property.extensionName[..end]
}

#[derive(Default)]
struct InstanceExtensions {
    properties: Vec<VkExtensionProperties>,
    known: crate::ExtensionSet,
}

fn push_extension(
    extensions: &mut InstanceExtensions,
    property: &VkExtensionProperties,
) -> Result<(), VkResult> {
    // Preserve the first property, including its spec version, for every name.
    let name = extension_name(property);
    // SAFETY: c_char and u8 have identical size/alignment; the slice is bounded.
    let bytes = unsafe { core::slice::from_raw_parts(name.as_ptr().cast::<u8>(), name.len()) };
    let id = crate::generated::extension_id_bytes(bytes);
    let duplicate = match id {
        Some(id) => extensions.known.contains(id),
        None => extensions
            .properties
            .iter()
            .any(|existing| extension_name(existing) == name),
    };
    if duplicate {
        return Ok(());
    }
    extensions
        .properties
        .try_reserve(1)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    extensions.properties.push(*property);
    // Do not publish membership until fallible growth has succeeded.
    if let Some(id) = id {
        extensions.known.insert(id);
    }
    Ok(())
}

fn push_named_extension(
    extensions: &mut InstanceExtensions,
    name: &CStr,
    spec_version: u32,
) -> Result<(), VkResult> {
    let id = crate::generated::extension_id_bytes(name.to_bytes());
    // SAFETY: `c_char` and `u8` have identical layout, and this view does not
    // outlive the C string.
    let name_chars = unsafe {
        core::slice::from_raw_parts(name.to_bytes().as_ptr().cast(), name.to_bytes().len())
    };
    let duplicate = match id {
        Some(id) => extensions.known.contains(id),
        None => extensions
            .properties
            .iter()
            .any(|existing| extension_name(existing) == name_chars),
    };
    if duplicate {
        return Ok(());
    }
    extensions
        .properties
        .try_reserve(1)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let index = extensions.properties.len();
    // SAFETY: The successful reservation leaves one writable spare slot.
    let property = unsafe { extensions.properties.as_mut_ptr().add(index) };
    unsafe { property.write(VkExtensionProperties::DEFAULT) };
    let bytes = name.to_bytes_with_nul();
    let count = bytes.len().min(vk::VK_MAX_EXTENSION_NAME_SIZE as usize);
    // SAFETY: The spare property is initialized, both element types occupy one
    // byte, and extensionName has at least `count` entries.
    unsafe {
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            (*property).extensionName.as_mut_ptr().cast(),
            count,
        );
    }
    if count == vk::VK_MAX_EXTENSION_NAME_SIZE as usize {
        unsafe { (*property).extensionName[count - 1] = 0 };
    }
    unsafe {
        (*property).specVersion = spec_version;
        extensions.properties.set_len(index + 1);
    }
    if let Some(id) = id {
        extensions.known.insert(id);
    }
    Ok(())
}

#[cfg(test)]
fn extension_property(name: &CStr, spec_version: u32) -> VkExtensionProperties {
    let mut property = VkExtensionProperties::DEFAULT;
    let bytes = name.to_bytes_with_nul();
    let count = bytes.len().min(property.extensionName.len());
    unsafe {
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            property.extensionName.as_mut_ptr().cast(),
            count,
        );
    }
    if count == property.extensionName.len() {
        property.extensionName[count - 1] = 0;
    }
    property.specVersion = spec_version;
    property
}

fn append_manifest_extensions(
    extensions: &mut InstanceExtensions,
    root: &LayerManifest,
    manifests: &[LayerManifest],
) -> Result<(), VkResult> {
    crate::discovery::resolve_layer_names(manifests);
    let root_index = manifests
        .iter()
        .position(|manifest| ptr::eq(manifest, root))
        .ok_or(VkResult::ERROR_LAYER_NOT_PRESENT)?;
    let mut visited = crate::allocation::try_boxed_slice_filled(manifests.len(), false)?;
    let mut pending = Vec::new();
    pending
        .try_reserve_exact(manifests.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    pending.push(root_index);
    while let Some(index) = pending.pop() {
        if visited[index] {
            continue;
        }
        visited[index] = true;
        let manifest = &manifests[index];
        for extension in &manifest.instance_extensions {
            push_named_extension(extensions, &extension.name, extension.spec_version)?;
        }
        for component in manifest.component_layers().iter().rev() {
            if let Some(index) = component.index() {
                // Duplicate edges can queue more entries than there are manifests.
                // Keep the existing DFS order while making overflow growth fallible.
                crate::allocation::try_push(&mut pending, index)?;
            }
        }
    }
    Ok(())
}

fn append_loader_extensions(extensions: &mut InstanceExtensions) -> Result<(), VkResult> {
    const LOADER_EXTENSIONS: [(&CStr, u32); 4] = [
        (
            vk::VK_EXT_DEBUG_REPORT_EXTENSION_NAME,
            vk::VK_EXT_DEBUG_REPORT_SPEC_VERSION,
        ),
        (
            vk::VK_EXT_DEBUG_UTILS_EXTENSION_NAME,
            vk::VK_EXT_DEBUG_UTILS_SPEC_VERSION,
        ),
        (
            vk::VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME,
            vk::VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION,
        ),
        (
            vk::VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_NAME,
            vk::VK_LUNARG_DIRECT_DRIVER_LOADING_SPEC_VERSION,
        ),
    ];
    extensions
        .properties
        .try_reserve(LOADER_EXTENSIONS.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for (name, spec_version) in LOADER_EXTENSIONS {
        push_named_extension(extensions, name, spec_version)?;
    }
    Ok(())
}

/// Enumerates an ICD's instance extensions into owned storage.
///
/// `accept_incomplete` preserves the different error policies of the loader's
/// two callers: instance creation accepts a truncated second query, while the
/// pre-instance layer ABI requires a complete result.
pub(crate) unsafe fn enumerate_icd_extension_properties(
    enumerate: PFN_vkEnumerateInstanceExtensionProperties,
    accept_incomplete: bool,
) -> Result<Vec<VkExtensionProperties>, VkResult> {
    let mut count = 0;
    let result = unsafe { enumerate(ptr::null(), &raw mut count, ptr::null_mut()) };
    if result != VkResult::SUCCESS {
        return Err(result);
    }
    let capacity = count as usize;
    let mut properties = Vec::new();
    properties
        .try_reserve_exact(capacity)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    properties.resize(capacity, VkExtensionProperties::DEFAULT);
    let result = unsafe { enumerate(ptr::null(), &raw mut count, properties.as_mut_ptr()) };
    if result != VkResult::SUCCESS && (!accept_incomplete || result != VkResult::INCOMPLETE) {
        return Err(result);
    }
    properties.truncate((count as usize).min(capacity));
    Ok(properties)
}

unsafe fn append_icd_extensions(extensions: &mut InstanceExtensions) -> Result<(), VkResult> {
    // SAFETY: As in upstream discovery, callers must not mutate the process
    // environment during loader operations; the callback only parses a string.
    let filter_unknown = unsafe {
        platform::inspect_environment_lossy(c"VK_LOADER_DISABLE_INST_EXT_FILTER", |value| {
            !value.is_some_and(decimal_environment_value_is_nonzero)
        })
    }?;
    crate::icd::preload_icds()?;
    let icds = crate::icd::scan_global_icds()?;
    (|| {
        for icd in &icds {
            let enumerate: Option<PFN_vkEnumerateInstanceExtensionProperties> =
                unsafe { icd.resolve(VkInstance::NULL, c"vkEnumerateInstanceExtensionProperties") };
            let Some(enumerate) = enumerate else {
                continue;
            };
            let properties = unsafe { enumerate_icd_extension_properties(enumerate, false) }?;
            for property in properties {
                let name = unsafe { CStr::from_ptr(property.extensionName.as_ptr()) };
                if !crate::wsi_instance_extension_supported(name)
                    || (filter_unknown && !crate::is_known_instance_extension(name))
                {
                    continue;
                }
                push_extension(extensions, &property)?;
            }
        }
        Ok(())
    })()
}

fn decimal_environment_value_is_nonzero(value: &str) -> bool {
    let value = value.trim_start();
    let digits = match value.as_bytes().first() {
        Some(b'-' | b'+') => &value[1..],
        _ => value,
    };
    digits
        .bytes()
        .take_while(u8::is_ascii_digit)
        .any(|digit| digit != b'0')
}

pub(crate) unsafe fn enumerate_extension_properties_terminator(
    layer_name: *const core::ffi::c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let manifests = if layer_name.is_null() || unsafe { layer_name.read() } == 0 {
        let settings = discovery::silent_global_loader_settings();
        discover_implicit_layers_with_settings(settings.as_ref())
    } else {
        let settings = discovery::silent_global_loader_settings();
        discovery::discover_layers_with_settings(settings.as_ref())
    };
    let global_extensions = layer_name.is_null() || unsafe { layer_name.read() } == 0;
    if !global_extensions {
        emit_layer_searches(&manifests);
    }
    unsafe {
        enumerate_extension_properties_from_manifests(
            &manifests,
            layer_name,
            &mut *property_count,
            properties,
        )
    }
}

unsafe fn enumerate_extension_properties_from_manifests(
    manifests: &discovery::DiscoveredLayers,
    layer_name: *const core::ffi::c_char,
    property_count: &mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    let valid = match valid_layer_mask(manifests) {
        Ok(valid) => valid,
        Err(result) => return result,
    };
    let mut extensions = InstanceExtensions::default();
    if layer_name.is_null() || unsafe { layer_name.read() } == 0 {
        if let Err(result) = unsafe { append_icd_extensions(&mut extensions) } {
            return result;
        }
        emit_layer_searches(manifests);
        if let Err(result) = append_loader_extensions(&mut extensions) {
            return result;
        }
        for manifest in manifests
            .iter()
            .zip(valid.iter())
            .filter_map(|(manifest, valid)| (*valid).then_some(manifest))
            .filter(|manifest| is_enabled_implicit(manifest))
        {
            for extension in &manifest.instance_extensions {
                if let Err(result) =
                    push_named_extension(&mut extensions, &extension.name, extension.spec_version)
                {
                    return result;
                }
            }
        }
    } else {
        let name = unsafe { CStr::from_ptr(layer_name) };
        let Some((manifest, _)) = manifests
            .iter()
            .zip(valid.iter())
            .find(|(manifest, valid)| **valid && manifest.name.as_c_str() == name)
        else {
            return VkResult::ERROR_LAYER_NOT_PRESENT;
        };
        if let Err(result) = append_manifest_extensions(&mut extensions, manifest, manifests) {
            return result;
        }
    }
    let total = extensions.properties.len().min(u32::MAX as usize) as u32;
    if properties.is_null() {
        *property_count = total;
        return VkResult::SUCCESS;
    }
    let capacity = *property_count as usize;
    let written = capacity.min(extensions.properties.len());
    unsafe {
        ptr::copy_nonoverlapping(extensions.properties.as_ptr(), properties, written);
    }
    *property_count = written as u32;
    if written < extensions.properties.len() {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

unsafe extern "system" fn layer_terminator(
    _chain: *const LayerPropertiesChain,
    property_count: *mut u32,
    properties: *mut VkLayerProperties,
) -> VkResult {
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let settings = discovery::silent_global_loader_settings();
    let discovered = discovery::discover_layers_with_settings(settings.as_ref());
    unsafe { layer::enumerate_instance_layers(discovered, &mut *property_count, properties) }
}

unsafe extern "system" fn version_terminator(
    _chain: *const VersionChain,
    api_version: *mut u32,
) -> VkResult {
    if api_version.is_null() {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    unsafe { api_version.write(vk::VK_HEADER_VERSION_COMPLETE) };
    VkResult::SUCCESS
}

/// Builds stable links for the synchronous call. Small chains stay on the
/// stack; Copy excludes resources requiring destruction from scratch storage.
fn with_chain<T: Copy>(
    tail: &T,
    functions: &[LoadedFunction<'_>],
    link: impl Fn(&LoadedFunction<'_>, *const T) -> T,
    call: impl FnOnce(&T) -> VkResult,
) -> VkResult {
    let Ok(mut links) = ScratchArray::<T, 8>::try_new(functions.len()) else {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    };
    let mut head = ptr::from_ref(tail);
    for (index, function) in functions.iter().enumerate() {
        // SAFETY: Each slot is within the reserved storage and written once.
        // The storage stays at this address until the synchronous call ends.
        let slot = unsafe { links.as_mut_ptr().add(index) };
        unsafe { slot.write(link(function, head)) };
        head = slot;
    }
    // SAFETY: head is the live tail or the last initialized, stable link.
    call(unsafe { &*head })
}

pub(crate) unsafe fn enumerate_extension_properties(
    layer_name: *const core::ffi::c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let nested_instance_create = pending::instance() != VkInstance::NULL;
    let settings = if nested_instance_create {
        discovery::silent_global_loader_settings()
    } else {
        update_global_loader_settings()
    };
    let manifests = discover_implicit_layers_with_settings(settings.as_ref());
    if !nested_instance_create {
        emit_layer_searches(&manifests);
    }
    let functions = match load_functions(&manifests, PreInstanceFunction::ExtensionProperties) {
        Ok(functions) => functions,
        Err(result) => return result,
    };
    let tail = ExtensionPropertiesChain {
        header: header(
            CHAIN_TYPE_EXTENSION_PROPERTIES,
            mem::size_of::<ExtensionPropertiesChain>(),
        ),
        next_function: extension_terminator,
        next_link: ptr::null(),
    };
    with_chain(
        &tail,
        &functions,
        |function, next_link| ExtensionPropertiesChain {
            header: tail.header,
            // SAFETY: `load_functions` selected the extension-properties field.
            next_function: unsafe { crate::load_typed(Some(function.function)) }
                .unwrap_or(extension_terminator),
            next_link,
        },
        // SAFETY: Negotiated ABI functions and caller-provided output storage
        // remain valid for the complete synchronous chain invocation.
        |head| unsafe {
            (head.next_function)(head.next_link, layer_name, property_count, properties)
        },
    )
}

pub(crate) unsafe fn enumerate_layer_properties(
    property_count: *mut u32,
    properties: *mut VkLayerProperties,
) -> VkResult {
    let nested_instance_create = pending::instance() != VkInstance::NULL;
    let settings = discovery::silent_global_loader_settings();
    let settings = if nested_instance_create {
        settings
    } else if settings
        .as_ref()
        .is_some_and(LoaderSettings::has_unordered_layer_location)
    {
        discovery::diagnostic_global_loader_settings()
    } else {
        update_global_loader_settings()
    };
    let manifests = discover_implicit_layers_with_settings(settings.as_ref());
    if !nested_instance_create {
        emit_layer_searches(&manifests);
    }
    let functions = match load_functions(&manifests, PreInstanceFunction::LayerProperties) {
        Ok(functions) => functions,
        Err(result) => return result,
    };
    let tail = LayerPropertiesChain {
        header: header(
            CHAIN_TYPE_LAYER_PROPERTIES,
            mem::size_of::<LayerPropertiesChain>(),
        ),
        next_function: layer_terminator,
        next_link: ptr::null(),
    };
    with_chain(
        &tail,
        &functions,
        |function, next_link| LayerPropertiesChain {
            header: tail.header,
            // SAFETY: `load_functions` selected the layer-properties field.
            next_function: unsafe { crate::load_typed(Some(function.function)) }
                .unwrap_or(layer_terminator),
            next_link,
        },
        // SAFETY: Negotiated ABI functions and caller-provided output storage
        // remain valid for the complete synchronous chain invocation.
        |head| unsafe { (head.next_function)(head.next_link, property_count, properties) },
    )
}

pub(crate) unsafe fn enumerate_version(api_version: &mut u32) -> VkResult {
    let nested_instance_create = pending::instance() != VkInstance::NULL;
    let settings = if nested_instance_create {
        discovery::silent_global_loader_settings()
    } else {
        update_global_loader_settings()
    };
    let manifests = discover_implicit_layers_with_settings(settings.as_ref());
    if !nested_instance_create {
        emit_layer_searches(&manifests);
    }
    let functions = match load_functions(&manifests, PreInstanceFunction::Version) {
        Ok(functions) => functions,
        Err(result) => return result,
    };
    let tail = VersionChain {
        header: header(CHAIN_TYPE_INSTANCE_VERSION, mem::size_of::<VersionChain>()),
        next_function: version_terminator,
        next_link: ptr::null(),
    };
    with_chain(
        &tail,
        &functions,
        |function, next_link| VersionChain {
            header: tail.header,
            // SAFETY: `load_functions` selected the version field.
            next_function: unsafe { crate::load_typed(Some(function.function)) }
                .unwrap_or(version_terminator),
            next_link,
        },
        // SAFETY: Negotiated ABI functions and caller-provided output storage
        // remain valid for the complete synchronous chain invocation.
        |head| unsafe { (head.next_function)(head.next_link, api_version) },
    )
}

#[cfg(test)]
mod tests {
    use core::mem::{offset_of, size_of};

    use super::*;

    #[test]
    fn decimal_environment_flag_uses_nonzero_decimal_prefix() {
        for value in ["", " ", "+", "-", "0", "-000", "0x1", "word1", "00 1"] {
            assert!(
                !super::decimal_environment_value_is_nonzero(value),
                "{value:?}"
            );
        }
        for value in [
            "1",
            "-1",
            "+001",
            "  10tail",
            "\u{2003}1",
            "18446744073709551616",
            "-18446744073709551616",
        ] {
            assert!(
                super::decimal_environment_value_is_nonzero(value),
                "{value:?}"
            );
        }
    }

    #[test]
    fn duplicate_meta_edges_preserve_dfs_order_and_propagate_allocation_failures() {
        let names = [
            c"VK_LAYER_root",
            c"VK_LAYER_b",
            c"VK_LAYER_c",
            c"VK_LAYER_d",
        ];
        let extension_names = [c"VK_EXT_root", c"VK_EXT_b", c"VK_EXT_c", c"VK_EXT_d"];
        let mut manifests = names.map(|name| {
            let mut manifest = crate::discovery::test_manifest(&[]);
            manifest.name = name.to_owned();
            manifest
        });
        for (manifest, name) in manifests.iter_mut().zip(extension_names) {
            manifest.instance_extensions = [crate::discovery::LayerExtension {
                name: name.to_owned().into(),
                spec_version: 1,
                entrypoints: Box::default(),
            }]
            .into();
        }
        manifests[0].source = crate::discovery::LayerSource::Meta(
            [
                names[1], names[2], names[1], names[1], names[1], names[1], names[1], names[1],
            ]
            .map(|name| crate::discovery::LayerComponent::from(name.to_owned()))
            .into(),
        );
        manifests[1].source = crate::discovery::LayerSource::Meta(
            [names[2], names[3]]
                .map(|name| crate::discovery::LayerComponent::from(name.to_owned()))
                .into(),
        );
        crate::allocation::fault::sweep_operation(|| {
            let mut extensions = InstanceExtensions::default();
            match append_manifest_extensions(&mut extensions, &manifests[0], &manifests) {
                Ok(()) => {
                    assert_eq!(extensions.properties.len(), extension_names.len());
                    for (actual, name) in extensions.properties.iter().zip(extension_names) {
                        assert_eq!(
                            actual.extensionName,
                            extension_property(name, 1).extensionName
                        );
                    }
                    VkResult::SUCCESS
                }
                Err(error) => error,
            }
        });
    }

    #[test]
    fn pre_instance_chain_layout_matches_vk_layer_h() {
        assert_eq!(size_of::<ChainHeader>(), 12);

        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(size_of::<LayerPropertiesChain>(), 32);
            assert_eq!(offset_of!(LayerPropertiesChain, next_function), 16);
            assert_eq!(offset_of!(LayerPropertiesChain, next_link), 24);
        }

        #[cfg(target_pointer_width = "32")]
        {
            assert_eq!(size_of::<LayerPropertiesChain>(), 20);
            assert_eq!(offset_of!(LayerPropertiesChain, next_function), 12);
            assert_eq!(offset_of!(LayerPropertiesChain, next_link), 16);
        }
    }

    #[test]
    fn failed_extension_growth_does_not_publish_membership() {
        crate::allocation::fault::sweep_operation(|| {
            let mut extensions = InstanceExtensions::default();
            let property = extension_property(c"VK_EXT_debug_utils", 1);
            let result = push_extension(&mut extensions, &property);
            if let Err(error) = result {
                assert!(extensions.properties.is_empty());
                push_extension(&mut extensions, &property).unwrap();
                assert_eq!(extensions.properties.len(), 1);
                return error;
            }
            VkResult::SUCCESS
        });
    }

    #[test]
    fn duplicate_extension_retains_first_property_like_upstream() {
        let mut extensions = InstanceExtensions::default();
        push_extension(
            &mut extensions,
            &extension_property(c"VK_EXT_debug_utils", 1),
        )
        .unwrap();
        push_extension(
            &mut extensions,
            &extension_property(c"VK_EXT_debug_utils", 99),
        )
        .unwrap();

        assert_eq!(extensions.properties.len(), 1);
        assert_eq!(extensions.properties[0].specVersion, 1);
        push_extension(
            &mut extensions,
            &extension_property(c"VK_VENDOR_unknown_\xff", 2),
        )
        .unwrap();
        push_extension(
            &mut extensions,
            &extension_property(c"VK_VENDOR_unknown_\xff", 3),
        )
        .unwrap();
        assert_eq!(extensions.properties.len(), 2);
        assert_eq!(extensions.properties[1].specVersion, 2);
    }
}
