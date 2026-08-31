//! `vk_layer.h` pre-instance enumeration chains.

use alloc::vec::Vec;
use core::{ffi::CStr, mem};

use vk::{
    PFN_vkEnumerateInstanceExtensionProperties, VkExtensionProperties, VkInstance,
    VkLayerProperties, VkResult,
};

use crate::{
    discovery::{LayerManifest, discover_implicit_layers, discover_layers, valid_layer_mask},
    platform::LoaderLibrary,
};

const CURRENT_CHAIN_VERSION: u32 = 1;
const CHAIN_TYPE_EXTENSION_PROPERTIES: u32 = 1;
const CHAIN_TYPE_LAYER_PROPERTIES: u32 = 2;
const CHAIN_TYPE_INSTANCE_VERSION: u32 = 3;

fn update_global_loader_settings() {
    let settings = crate::discovery::loader_settings();
    if let Some(settings) = settings.as_ref() {
        let display_path = settings
            .settings_file_path()
            .to_string_lossy()
            .replace("/vulkan/loader_settings.d", "/vulkan//loader_settings.d");
        crate::platform::write_loader_log(
            "info",
            "INFO",
            format_args!("Using layer configurations found in loader settings from {display_path}"),
        );
    } else {
        crate::platform::write_loader_log(
            "info",
            "INFO",
            format_args!(
                "No valid vk_loader_settings.json file found, no loader settings will be active"
            ),
        );
    }
}

fn emit_layer_searches(layers: &crate::discovery::DiscoveredLayers) {
    crate::layer::emit_global_layer_search_diagnostics(layers.searches());
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
struct LayerPropertiesChain {
    header: ChainHeader,
    next_function: EnumerateLayerProperties,
    next_link: *const Self,
}

type EnumerateVersion = unsafe extern "system" fn(*const VersionChain, *mut u32) -> VkResult;

#[repr(C)]
struct VersionChain {
    header: ChainHeader,
    next_function: EnumerateVersion,
    next_link: *const Self,
}

struct LoadedFunction<F> {
    _library: LoaderLibrary,
    function: F,
}

fn is_enabled_implicit(manifest: &LayerManifest) -> bool {
    crate::layer::implicit_manifest_is_active(manifest)
}

fn load_functions<F: Copy>(
    manifests: &[LayerManifest],
    select: impl Fn(&LayerManifest) -> Option<&CStr>,
) -> Result<Vec<LoadedFunction<F>>, VkResult> {
    let valid = valid_layer_mask(manifests);
    let mut functions = Vec::new();
    functions
        .try_reserve_exact(manifests.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for (manifest, valid) in manifests.iter().zip(valid.iter()) {
        if !valid || !is_enabled_implicit(manifest) {
            continue;
        }
        let Some(name) = select(manifest) else {
            continue;
        };
        let Some(path) = manifest.library_path.as_ref() else {
            continue;
        };
        // SAFETY: The library is retained beside the copied function pointer.
        let Ok(library) = (unsafe { LoaderLibrary::open(path) }) else {
            continue;
        };
        // SAFETY: The manifest names a function with the selected `vk_layer.h` ABI.
        let function = unsafe {
            library
                .get::<F>(name.to_bytes_with_nul())
                .ok()
                .map(|symbol| *symbol)
        };
        if let Some(function) = function {
            functions.push(LoadedFunction {
                _library: library,
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

fn push_extension(
    extensions: &mut Vec<VkExtensionProperties>,
    property: &VkExtensionProperties,
) -> Result<(), VkResult> {
    // `loader_add_to_ext_list` retains the first property for a duplicate name.
    if extensions
        .iter()
        .any(|existing| extension_name(existing) == extension_name(property))
    {
        return Ok(());
    }
    extensions
        .try_reserve(1)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    extensions.push(*property);
    Ok(())
}

fn extension_property(name: &CStr, spec_version: u32) -> VkExtensionProperties {
    let mut property = VkExtensionProperties::DEFAULT;
    let bytes = name.to_bytes_with_nul();
    let count = bytes.len().min(property.extensionName.len());
    // SAFETY: Both element types occupy one byte and the slices have `count` entries.
    unsafe {
        core::ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            property.extensionName.as_mut_ptr().cast(),
            count,
        );
    }
    if count == property.extensionName.len() {
        let last = property.extensionName.len() - 1;
        property.extensionName[last] = 0;
    }
    property.specVersion = spec_version;
    property
}

fn layer_extension_property(extension: &crate::discovery::LayerExtension) -> VkExtensionProperties {
    extension_property(&extension.name, extension.spec_version)
}

fn append_manifest_extensions(
    extensions: &mut Vec<VkExtensionProperties>,
    root: &LayerManifest,
    manifests: &[LayerManifest],
) -> Result<(), VkResult> {
    let root_index = manifests
        .iter()
        .position(|manifest| core::ptr::eq(manifest, root))
        .ok_or(VkResult::ERROR_LAYER_NOT_PRESENT)?;
    let mut visited = vec![false; manifests.len()];
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
            push_extension(extensions, &layer_extension_property(extension))?;
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
    Ok(())
}

fn append_loader_extensions(extensions: &mut Vec<VkExtensionProperties>) -> Result<(), VkResult> {
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
        .try_reserve(LOADER_EXTENSIONS.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for (name, spec_version) in LOADER_EXTENSIONS {
        push_extension(extensions, &extension_property(name, spec_version))?;
    }
    Ok(())
}

unsafe fn append_icd_extensions(
    extensions: &mut Vec<VkExtensionProperties>,
) -> Result<(), VkResult> {
    let filter_unknown = !std::env::var_os("VK_LOADER_DISABLE_INST_EXT_FILTER")
        .is_some_and(|value| decimal_environment_value_is_nonzero(&value.to_string_lossy()));
    crate::icd::preload_icds();
    let icds = crate::icd::scan_global_icds()?;
    (|| {
        for icd in &icds {
            let enumerate: Option<PFN_vkEnumerateInstanceExtensionProperties> =
                unsafe { icd.resolve(VkInstance::NULL, c"vkEnumerateInstanceExtensionProperties") };
            let Some(enumerate) = enumerate else {
                continue;
            };
            let mut count = 0;
            let result =
                unsafe { enumerate(core::ptr::null(), &raw mut count, core::ptr::null_mut()) };
            if result != VkResult::SUCCESS {
                return Err(result);
            }
            let capacity = count as usize;
            let mut properties = Vec::new();
            properties
                .try_reserve_exact(capacity)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            properties.resize(capacity, VkExtensionProperties::DEFAULT);
            let result =
                unsafe { enumerate(core::ptr::null(), &raw mut count, properties.as_mut_ptr()) };
            if result != VkResult::SUCCESS {
                return Err(result);
            }
            properties.truncate((count as usize).min(capacity));
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
    let mut found = false;
    let mut parsed = 0_u64;
    for digit in digits.bytes() {
        if !digit.is_ascii_digit() {
            break;
        }
        found = true;
        parsed = parsed
            .saturating_mul(10)
            .saturating_add(u64::from(digit - b'0'));
    }
    found && parsed != 0
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
        discover_implicit_layers()
    } else {
        discover_layers()
    };
    let global_extensions = layer_name.is_null() || unsafe { layer_name.read() } == 0;
    if !global_extensions {
        emit_layer_searches(&manifests);
    }
    unsafe {
        enumerate_extension_properties_from_manifests(
            &manifests,
            layer_name,
            property_count,
            properties,
            global_extensions.then_some(manifests.searches()),
        )
    }
}

unsafe fn enumerate_extension_properties_from_manifests(
    manifests: &[LayerManifest],
    layer_name: *const core::ffi::c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
    searches_after_icds: Option<&[crate::discovery::LayerSearch]>,
) -> VkResult {
    let valid = valid_layer_mask(manifests);
    let mut extensions = Vec::new();
    if layer_name.is_null() || unsafe { layer_name.read() } == 0 {
        if let Err(result) = unsafe { append_icd_extensions(&mut extensions) } {
            return result;
        }
        if let Some(searches) = searches_after_icds {
            crate::layer::emit_global_layer_search_diagnostics(searches);
        }
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
                    push_extension(&mut extensions, &layer_extension_property(extension))
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
    let total = extensions.len().min(u32::MAX as usize) as u32;
    if properties.is_null() {
        unsafe { property_count.write(total) };
        return VkResult::SUCCESS;
    }
    let capacity = unsafe { property_count.read() } as usize;
    let written = capacity.min(extensions.len());
    unsafe {
        core::ptr::copy_nonoverlapping(extensions.as_ptr(), properties, written);
        property_count.write(written as u32);
    }
    if written < extensions.len() {
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
    unsafe { crate::layer::enumerate_instance_layers(property_count, properties) }
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

pub(crate) unsafe fn enumerate_extension_properties(
    layer_name: *const core::ffi::c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    update_global_loader_settings();
    let manifests = discover_implicit_layers();
    emit_layer_searches(&manifests);
    let functions = match load_functions::<EnumerateExtensionProperties>(&manifests, |manifest| {
        manifest
            .pre_instance_functions
            .extension_properties
            .as_deref()
    }) {
        Ok(functions) => functions,
        Err(result) => return result,
    };
    let tail = ExtensionPropertiesChain {
        header: header(
            CHAIN_TYPE_EXTENSION_PROPERTIES,
            mem::size_of::<ExtensionPropertiesChain>(),
        ),
        next_function: extension_terminator,
        next_link: core::ptr::null(),
    };
    let mut links = Vec::new();
    if links.try_reserve_exact(functions.len()).is_err() {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    let mut head = core::ptr::from_ref(&tail);
    for function in &functions {
        links.push(ExtensionPropertiesChain {
            header: tail.header,
            next_function: function.function,
            next_link: head,
        });
        // SAFETY: The link was appended immediately above.
        head = core::ptr::from_ref(unsafe { links.last().unwrap_unchecked() });
    }
    let head = unsafe { &*head };
    unsafe { (head.next_function)(head.next_link, layer_name, property_count, properties) }
}

pub(crate) unsafe fn enumerate_layer_properties(
    property_count: *mut u32,
    properties: *mut VkLayerProperties,
) -> VkResult {
    update_global_loader_settings();
    let manifests = discover_implicit_layers();
    emit_layer_searches(&manifests);
    let functions = match load_functions::<EnumerateLayerProperties>(&manifests, |manifest| {
        manifest.pre_instance_functions.layer_properties.as_deref()
    }) {
        Ok(functions) => functions,
        Err(result) => return result,
    };
    let tail = LayerPropertiesChain {
        header: header(
            CHAIN_TYPE_LAYER_PROPERTIES,
            mem::size_of::<LayerPropertiesChain>(),
        ),
        next_function: layer_terminator,
        next_link: core::ptr::null(),
    };
    let mut links = Vec::new();
    if links.try_reserve_exact(functions.len()).is_err() {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    let mut head = core::ptr::from_ref(&tail);
    for function in &functions {
        links.push(LayerPropertiesChain {
            header: tail.header,
            next_function: function.function,
            next_link: head,
        });
        // SAFETY: The link was appended immediately above.
        head = core::ptr::from_ref(unsafe { links.last().unwrap_unchecked() });
    }
    let head = unsafe { &*head };
    unsafe { (head.next_function)(head.next_link, property_count, properties) }
}

pub(crate) unsafe fn enumerate_version(api_version: *mut u32) -> VkResult {
    update_global_loader_settings();
    let manifests = discover_implicit_layers();
    emit_layer_searches(&manifests);
    let functions = match load_functions::<EnumerateVersion>(&manifests, |manifest| {
        manifest.pre_instance_functions.version.as_deref()
    }) {
        Ok(functions) => functions,
        Err(result) => return result,
    };
    let tail = VersionChain {
        header: header(CHAIN_TYPE_INSTANCE_VERSION, mem::size_of::<VersionChain>()),
        next_function: version_terminator,
        next_link: core::ptr::null(),
    };
    let mut links = Vec::new();
    if links.try_reserve_exact(functions.len()).is_err() {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    let mut head = core::ptr::from_ref(&tail);
    for function in &functions {
        links.push(VersionChain {
            header: tail.header,
            next_function: function.function,
            next_link: head,
        });
        // SAFETY: The link was appended immediately above.
        head = core::ptr::from_ref(unsafe { links.last().unwrap_unchecked() });
    }
    let head = unsafe { &*head };
    unsafe { (head.next_function)(head.next_link, api_version) }
}

#[cfg(test)]
mod tests {
    use core::mem::{offset_of, size_of};

    use super::*;

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
    fn duplicate_extension_retains_first_property_like_upstream() {
        let mut extensions = Vec::new();
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

        assert_eq!(extensions.len(), 1);
        assert_eq!(extensions[0].specVersion, 1);
    }
}
