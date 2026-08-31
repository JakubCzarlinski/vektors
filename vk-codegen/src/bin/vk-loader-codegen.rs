use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    env,
    ffi::CString,
    fs,
    path::PathBuf,
    process,
};
use vk_codegen::{
    codegen::utils::{
        command_param_abi_type_for_registry, resolve_alias, rewrite_command_types_for_providers,
    },
    ir::{ExportScope, TypedefKind},
    parser::{apply_require_extensions, parse_registry},
    types::ctype_to_rust_str,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Scope {
    Global,
    Instance,
    Device,
}

// Mirrors Vulkan-Loader's `WSI_EXT_NAMES`: these extension entry points are
// part of the platform loader ABI. Other extension trampolines are reachable
// through GPA but intentionally remain hidden dynamic symbols.
const PUBLIC_WSI_EXTENSIONS: &[&str] = &[
    "VK_KHR_surface",
    "VK_KHR_display",
    "VK_KHR_xlib_surface",
    "VK_KHR_xcb_surface",
    "VK_KHR_wayland_surface",
    "VK_EXT_directfb_surface",
    "VK_KHR_win32_surface",
    "VK_KHR_android_surface",
    "VK_GGP_stream_descriptor_surface",
    "VK_MVK_macos_surface",
    "VK_MVK_ios_surface",
    "VK_EXT_headless_surface",
    "VK_EXT_metal_surface",
    "VK_FUCHSIA_imagepipe_surface",
    "VK_KHR_swapchain",
    "VK_KHR_display_swapchain",
    "VK_KHR_get_display_properties2",
    "VK_KHR_get_surface_capabilities2",
    "VK_QNX_screen_surface",
    "VK_NN_vi_surface",
];

// Vulkan-Loader filters these instance extensions when their platform WSI
// implementation was not compiled in. Keep this separate from the public WSI
// export list: mandatory platform backends are not filtered this way upstream.
const BUILD_FILTERED_WSI_EXTENSIONS: &[&str] = &[
    "VK_KHR_xlib_surface",
    "VK_KHR_xcb_surface",
    "VK_KHR_wayland_surface",
    "VK_EXT_directfb_surface",
    "VK_QNX_screen_surface",
];

const HANDWRITTEN_TERMINATORS: &[&str] = &[
    "vkCreateDevice",
    "vkCreateDebugReportCallbackEXT",
    "vkCreateDebugUtilsMessengerEXT",
    "vkCreateSharedSwapchainsKHR",
    "vkDestroyDevice",
    "vkDestroyDebugReportCallbackEXT",
    "vkDestroyInstance",
    "vkDestroySurfaceKHR",
    "vkDestroyDebugUtilsMessengerEXT",
    "vkEnumeratePhysicalDeviceGroups",
    "vkEnumeratePhysicalDeviceGroupsKHR",
    "vkEnumeratePhysicalDevices",
    "vkEnumerateDeviceExtensionProperties",
    "vkEnumerateDeviceLayerProperties",
    "vkDebugReportMessageEXT",
    "vkDebugMarkerSetObjectNameEXT",
    "vkDebugMarkerSetObjectTagEXT",
    "vkGetDeviceProcAddr",
    "vkGetDeviceGroupSurfacePresentModesKHR",
    "vkCreateSwapchainKHR",
    "vkSetDebugUtilsObjectNameEXT",
    "vkSetDebugUtilsObjectTagEXT",
    "vkSubmitDebugUtilsMessageEXT",
];

const HANDWRITTEN_INSTANCE_TERMINATORS: &[&str] = &["vkDestroySurfaceKHR"];

// These device commands must return the loader's public trampoline from GDPA.
// They either install loader dispatch data in returned dispatchable handles or
// require loader-owned object translation/cleanup before reaching the chain.
const LOADER_DEVICE_TRAMPOLINES: &[&str] = &[
    "vkAllocateCommandBuffers",
    "vkCreateSharedSwapchainsKHR",
    "vkCreateSwapchainKHR",
    "vkDebugMarkerSetObjectNameEXT",
    "vkDebugMarkerSetObjectTagEXT",
    "vkDestroyDevice",
    "vkGetDeviceGroupSurfacePresentModesKHR",
    "vkGetDeviceQueue",
    "vkGetDeviceQueue2",
    "vkSetDebugUtilsObjectNameEXT",
    "vkSetDebugUtilsObjectTagEXT",
];

// These retain their generated public trampoline, but require loader-owned
// handle translation or emulation at the ICD boundary.
const HANDWRITTEN_PHYSICAL_DEVICE_TERMINATORS: &[&str] = &[
    "vkGetPhysicalDeviceFeatures2",
    "vkGetPhysicalDeviceFeatures2KHR",
    "vkGetDisplayModeProperties2KHR",
    "vkGetDisplayPlaneCapabilities2KHR",
    "vkGetPhysicalDeviceExternalBufferProperties",
    "vkGetPhysicalDeviceExternalBufferPropertiesKHR",
    "vkGetPhysicalDeviceExternalFenceProperties",
    "vkGetPhysicalDeviceExternalFencePropertiesKHR",
    "vkGetPhysicalDeviceExternalSemaphoreProperties",
    "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR",
    "vkGetPhysicalDeviceFormatProperties2",
    "vkGetPhysicalDeviceFormatProperties2KHR",
    "vkGetPhysicalDeviceImageFormatProperties2",
    "vkGetPhysicalDeviceImageFormatProperties2KHR",
    "vkGetPhysicalDeviceMemoryProperties2",
    "vkGetPhysicalDeviceMemoryProperties2KHR",
    "vkGetPhysicalDeviceProperties2",
    "vkGetPhysicalDeviceProperties2KHR",
    "vkGetPhysicalDeviceDisplayPlaneProperties2KHR",
    "vkGetPhysicalDeviceDisplayProperties2KHR",
    "vkGetPhysicalDeviceQueueFamilyProperties2",
    "vkGetPhysicalDeviceQueueFamilyProperties2KHR",
    "vkGetPhysicalDeviceSparseImageFormatProperties2",
    "vkGetPhysicalDeviceSparseImageFormatProperties2KHR",
    "vkGetPhysicalDeviceToolProperties",
    "vkGetPhysicalDeviceToolPropertiesEXT",
    "vkGetPhysicalDeviceSurfaceCapabilities2KHR",
    "vkGetPhysicalDeviceSurfaceCapabilities2EXT",
    "vkGetPhysicalDeviceSurfaceFormats2KHR",
    "vkGetPhysicalDeviceSurfaceSupportKHR",
];

// These terminators share registry-defined core/KHR signatures around a
// handwritten emulation body. Generate the ABI wrappers and retain only the
// loader policy in `promoted.rs`.
const PROMOTED_TERMINATOR_IMPLEMENTATIONS: &[&str] = &[
    "vkGetPhysicalDeviceFeatures2",
    "vkGetPhysicalDeviceProperties2",
    "vkGetPhysicalDeviceFormatProperties2",
    "vkGetPhysicalDeviceMemoryProperties2",
    "vkGetPhysicalDeviceImageFormatProperties2",
    "vkGetPhysicalDeviceExternalBufferProperties",
    "vkGetPhysicalDeviceExternalSemaphoreProperties",
    "vkGetPhysicalDeviceExternalFenceProperties",
    "vkGetPhysicalDeviceQueueFamilyProperties2",
    "vkGetPhysicalDeviceSparseImageFormatProperties2",
];

// These commands are implemented by the loader itself or are legacy device-layer
// discovery, so the loader must not require an ICD to expose them through GIPA.
const ICD_INSTANCE_DISPATCH_EXCEPTIONS: &[&str] = &[
    "vkCreateInstance",
    "vkEnumerateDeviceLayerProperties",
    "vkEnumerateInstanceExtensionProperties",
    "vkEnumerateInstanceLayerProperties",
    "vkEnumerateInstanceVersion",
    "vkGetInstanceProcAddr",
];

// Keep this in lockstep with the commands for which the reference loader
// retains a direct ICD function pointer. `vkDestroyDevice` lives in its core
// dispatch while the remaining entries are generated into
// `loader_device_terminator_dispatch` by Vulkan-Loader.
const ICD_DEVICE_TERMINATOR_COMMANDS: &[&str] = &[
    "vkDestroyDevice",
    "vkCreateSwapchainKHR",
    "vkGetDeviceGroupSurfacePresentModesKHR",
    "vkCreateSharedSwapchainsKHR",
    "vkDebugMarkerSetObjectTagEXT",
    "vkDebugMarkerSetObjectNameEXT",
    "vkSetDebugUtilsObjectNameEXT",
    "vkSetDebugUtilsObjectTagEXT",
    "vkQueueBeginDebugUtilsLabelEXT",
    "vkQueueEndDebugUtilsLabelEXT",
    "vkQueueInsertDebugUtilsLabelEXT",
    "vkCmdBeginDebugUtilsLabelEXT",
    "vkCmdEndDebugUtilsLabelEXT",
    "vkCmdInsertDebugUtilsLabelEXT",
    "vkGetDeviceGroupSurfacePresentModes2EXT",
];

fn command_hash(name: &[u8]) -> u64 {
    name.iter().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

fn command_slot_hash(mut hash: u64) -> u64 {
    hash ^= hash >> 30;
    hash = hash.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    hash ^= hash >> 27;
    hash = hash.wrapping_mul(0x94d0_49bb_1331_11eb);
    hash ^ (hash >> 31)
}

fn qualify_registry_type(ty: &str, base: &str) -> String {
    let mut qualified = String::with_capacity(ty.len() + 4);
    let mut token_start = 0;
    for (index, character) in ty.char_indices() {
        if character.is_ascii_alphanumeric() || character == '_' {
            continue;
        }
        let token = &ty[token_start..index];
        if token == base {
            qualified.push_str("vk::");
        }
        qualified.push_str(token);
        qualified.push(character);
        token_start = index + character.len_utf8();
    }
    let token = &ty[token_start..];
    if token == base {
        qualified.push_str("vk::");
    }
    qualified.push_str(token);
    qualified
}

fn handle_scope(registry: &vk_codegen::ir::Registry, name: &str) -> Option<Scope> {
    match name {
        "VkDevice" | "VkQueue" | "VkCommandBuffer" => return Some(Scope::Device),
        "VkInstance" | "VkPhysicalDevice" => return Some(Scope::Instance),
        _ => {}
    }
    let ty = registry.typedefs.get(name)?.first()?;
    if let Some(alias) = &ty.alias {
        return handle_scope(registry, alias);
    }
    match &ty.kind {
        TypedefKind::Handle { parent, .. } => parent
            .as_deref()
            .and_then(|parent| {
                parent
                    .split(',')
                    .find_map(|name| handle_scope(registry, name))
            })
            .or(Some(Scope::Instance)),
        _ => None,
    }
}

fn command_scope(registry: &vk_codegen::ir::Registry, command: &vk_codegen::ir::Command) -> Scope {
    if let Some(alias) = &command.alias
        && let Some(target) = registry
            .commands
            .get(alias)
            .and_then(|commands| commands.first())
    {
        return command_scope(registry, target);
    }
    if command.name == "vkGetInstanceProcAddr" {
        return Scope::Global;
    }
    let Some(first) = command.params.first() else {
        return Scope::Global;
    };
    handle_scope(registry, &first.ty.base).unwrap_or(Scope::Global)
}

fn is_required_icd_instance_command(
    registry: &vk_codegen::ir::Registry,
    command: &vk_codegen::ir::Command,
) -> bool {
    if command.alias.is_some() || ICD_INSTANCE_DISPATCH_EXCEPTIONS.contains(&command.name.as_str())
    {
        return false;
    }
    let is_instance_dispatch = command_scope(registry, command) == Scope::Instance
        || command.name == "vkGetDeviceProcAddr";
    let is_core_1_0 = command
        .provided_by
        .iter()
        .any(|provider| matches!(provider.as_str(), "VK_BASE_VERSION_1_0" | "VK_VERSION_1_0"));
    let has_extension_provider = command
        .provided_by
        .iter()
        .any(|provider| provider.starts_with("VK_") && !provider.contains("_VERSION_"));
    is_instance_dispatch && is_core_1_0 && !has_extension_provider
}

fn command_platform_protect<'a>(
    registry: &'a vk_codegen::ir::Registry,
    command: &vk_codegen::ir::Command,
) -> Option<&'a str> {
    let mut protect = None;
    for provider in &command.provided_by {
        if provider.contains("_VERSION_") {
            return None;
        }
        let Some(extension) = registry
            .extensions
            .iter()
            .find(|extension| extension.name == *provider)
        else {
            continue;
        };
        let platform = extension.platform.as_deref()?;
        let candidate = registry.platforms.get(platform)?.as_str();
        match protect {
            Some(current) if current != candidate => return None,
            None => protect = Some(candidate),
            Some(_) => {}
        }
    }
    protect
}

fn command_has_vulkan_provider(
    registry: &vk_codegen::ir::Registry,
    command: &vk_codegen::ir::Command,
) -> bool {
    command.provided_by.iter().any(|provider| {
        if provider.contains("_VERSION_") {
            return !provider.starts_with("VKSC_");
        }
        registry.extensions.iter().any(|extension| {
            extension.name == *provider && extension.supports_vulkan() && !extension.is_disabled()
        })
    })
}

fn rust_platform_cfg(protect: &str) -> &'static str {
    match protect {
        "VK_USE_PLATFORM_XLIB_KHR" => {
            "all(feature = \"wsi-xlib\", any(target_os = \"linux\", target_os = \"freebsd\", target_os = \"openbsd\", target_os = \"netbsd\", target_os = \"dragonfly\", target_os = \"hurd\", target_os = \"cygwin\"))"
        }
        "VK_USE_PLATFORM_XLIB_XRANDR_EXT" => {
            "all(feature = \"wsi-xlib-xrandr\", any(target_os = \"linux\", target_os = \"freebsd\", target_os = \"openbsd\", target_os = \"netbsd\", target_os = \"dragonfly\", target_os = \"hurd\", target_os = \"cygwin\"))"
        }
        "VK_USE_PLATFORM_XCB_KHR" => {
            "all(feature = \"wsi-xcb\", any(target_os = \"linux\", target_os = \"freebsd\", target_os = \"openbsd\", target_os = \"netbsd\", target_os = \"dragonfly\", target_os = \"hurd\", target_os = \"cygwin\"))"
        }
        "VK_USE_PLATFORM_WAYLAND_KHR" => {
            "all(feature = \"wsi-wayland\", any(target_os = \"linux\", target_os = \"freebsd\", target_os = \"openbsd\", target_os = \"netbsd\", target_os = \"dragonfly\", target_os = \"hurd\", target_os = \"cygwin\"))"
        }
        "VK_USE_PLATFORM_DIRECTFB_EXT" => "feature = \"wsi-directfb\"",
        "VK_USE_PLATFORM_ANDROID_KHR" => "target_os = \"android\"",
        "VK_USE_PLATFORM_WIN32_KHR" => "target_os = \"windows\"",
        "VK_USE_PLATFORM_VI_NN" => "feature = \"platform-vi\"",
        "VK_USE_PLATFORM_IOS_MVK" => "target_os = \"ios\"",
        "VK_USE_PLATFORM_MACOS_MVK" => "target_os = \"macos\"",
        "VK_USE_PLATFORM_METAL_EXT" => {
            "any(target_os = \"macos\", target_os = \"ios\", target_os = \"tvos\", target_os = \"visionos\")"
        }
        "VK_USE_PLATFORM_FUCHSIA" => "target_os = \"fuchsia\"",
        "VK_USE_PLATFORM_GGP" => "feature = \"platform-ggp\"",
        "VK_USE_PLATFORM_SCI" => "feature = \"platform-sci\"",
        "VK_ENABLE_BETA_EXTENSIONS" => "feature = \"beta-extensions\"",
        "VK_USE_PLATFORM_SCREEN_QNX" => "any(target_os = \"nto\", target_os = \"qnx\")",
        "VK_USE_PLATFORM_OHOS" => "target_env = \"ohos\"",
        "VK_USE_PLATFORM_UBM_SEC" => "feature = \"platform-ubm\"",
        other => panic!("unsupported Vulkan platform protection macro {other}"),
    }
}

fn c_string_literal(value: &str) -> Literal {
    let value = CString::new(value).expect("generated C string must not contain NUL");
    Literal::c_string(&value)
}

fn u64_hex_literal(value: u64) -> syn::LitInt {
    syn::LitInt::new(
        &format!(
            "0x{:04x}_{:04x}_{:04x}_{:04x}",
            value >> 48,
            value >> 32 & 0xffff,
            value >> 16 & 0xffff,
            value & 0xffff
        ),
        Span::call_site(),
    )
}

fn platform_cfg(protect: Option<&str>) -> TokenStream {
    let Some(protect) = protect else {
        return TokenStream::new();
    };
    let cfg = rust_platform_cfg(protect)
        .parse::<TokenStream>()
        .expect("platform cfg must be valid Rust tokens");
    quote! { #[cfg(#cfg)] }
}

fn structure_type_constant<'a>(
    registry: &'a vk_codegen::ir::Registry,
    structure_name: &str,
) -> &'a str {
    let structure = registry
        .structs
        .get(structure_name)
        .and_then(|variants| variants.first())
        .unwrap_or_else(|| panic!("missing structure metadata for {structure_name}"));
    if let Some(alias) = &structure.alias {
        return structure_type_constant(registry, alias);
    }
    structure
        .members
        .iter()
        .find(|member| member.name == "sType")
        .and_then(|member| member.values.as_deref())
        .and_then(|values| values.split(',').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| panic!("missing sType value for {structure_name}"))
}

fn rust_structure_type_constant(constant: &str) -> &str {
    constant
        .strip_prefix("VK_STRUCTURE_TYPE_")
        .unwrap_or_else(|| panic!("invalid Vulkan structure-type constant {constant}"))
}

fn debug_object_type_pairs(registry: &vk_codegen::ir::Registry) -> Vec<(&str, String)> {
    let debug_names = registry
        .enums
        .get("VkDebugReportObjectTypeEXT")
        .into_iter()
        .flatten()
        .flat_map(|enumeration| &enumeration.variants)
        .map(|variant| variant.name.as_str())
        .collect::<HashSet<_>>();
    let mut pairs = registry
        .enums
        .get("VkObjectType")
        .into_iter()
        .flatten()
        .flat_map(|enumeration| &enumeration.variants)
        .filter(|variant| variant.alias.is_none())
        .filter_map(|variant| {
            let suffix = variant.name.strip_prefix("VK_OBJECT_TYPE_")?;
            let debug_name = format!("VK_DEBUG_REPORT_OBJECT_TYPE_{suffix}_EXT");
            debug_names
                .contains(debug_name.as_str())
                .then_some((suffix, debug_name))
        })
        .collect::<Vec<_>>();
    pairs.sort_unstable_by(|left, right| left.0.cmp(right.0));
    pairs.dedup_by(|left, right| left.0 == right.0);
    pairs
}

fn debug_report_variant(name: &str) -> &str {
    name.strip_prefix("VK_DEBUG_REPORT_OBJECT_TYPE_")
        .and_then(|name| name.strip_suffix("_EXT"))
        .unwrap_or_else(|| panic!("invalid debug-report object type {name}"))
}

fn promoted_implementation_name(command_name: &str) -> String {
    let suffix = command_name
        .strip_prefix("vkGetPhysicalDevice")
        .unwrap_or_else(|| panic!("unsupported promoted terminator name {command_name}"));
    let mut implementation = String::with_capacity(suffix.len() + "_impl".len());
    for character in suffix.chars() {
        implementation.push_str(
            match (implementation.is_empty(), character.is_ascii_uppercase()) {
                (false, true) => "_",
                _ => "",
            },
        );
        implementation.push(character.to_ascii_lowercase());
    }
    implementation.push_str("_impl");
    implementation
}

fn screaming_snake_case(name: &str) -> String {
    let characters = name.as_bytes();
    let mut result = String::with_capacity(name.len() + 16);
    for (index, &character) in characters.iter().enumerate() {
        let previous = index.checked_sub(1).map(|previous| characters[previous]);
        let next = characters.get(index + 1).copied();
        let starts_word = character.is_ascii_uppercase()
            && previous.is_some_and(|previous| {
                previous.is_ascii_lowercase()
                    || (previous.is_ascii_uppercase()
                        && next.is_some_and(|next| next.is_ascii_lowercase()))
            });
        if starts_word {
            result.push('_');
        }
        result.push(char::from(character.to_ascii_uppercase()));
    }
    result
}

fn resolved_command_signature(
    command: &vk_codegen::ir::Command,
    registry: &vk_codegen::ir::Registry,
) -> vk_codegen::ir::Command {
    if command.alias.is_none() {
        return command.clone();
    }
    let mut signature = resolve_alias(command, registry);
    let variants = &registry.commands[&command.name];
    let mut providers = variants
        .iter()
        .flat_map(|variant| variant.provided_by.clone())
        .collect::<Vec<_>>();
    providers.sort_unstable();
    providers.dedup();
    signature.provided_by.clone_from(&providers);
    signature.availability = variants
        .iter()
        .flat_map(|variant| variant.availability.clone())
        .collect();
    rewrite_command_types_for_providers(&mut signature, registry, &providers);
    signature
}

const GENERATED_LOADER_PARTS: &[&str] = &[
    "extensions",
    "debug",
    "dispatch_tables",
    "handles",
    "commands",
    "trampolines",
    "terminators",
    "proc_addr",
];

const GENERATED_PARENT_NAMES: &[&str] = &[
    "CStr",
    "CommandLookup",
    "CommandProviderRange",
    "CommandRecord",
    "CommandScope",
    "DEVICE_DISPATCH_MAGIC",
    "HandleInfo",
    "LoaderInstance",
    "PFN_vkVoidFunction",
    "VkStructureType",
    "c_char",
    "c_void",
    "command_hash",
    "command_name_eq",
    "command_slot_hash",
    "create_loader_surface",
    "device_dispatch",
    "dispatch_offset",
    "erase_function",
    "fatal_loader_error",
    "invalid_device_dispatch",
    "load_typed",
    "promoted",
    "resolve_physical_device",
    "resolve_trampoline_physical_device",
    "set_device_dispatchable",
    "terminator_vkDestroySurfaceKHR",
    "terminator_vkGetDisplayModeProperties2KHR",
    "terminator_vkGetDisplayPlaneCapabilities2KHR",
    "terminator_vkGetPhysicalDeviceDisplayPlaneProperties2KHR",
    "terminator_vkGetPhysicalDeviceDisplayProperties2KHR",
    "terminator_vkGetPhysicalDeviceSurfaceCapabilities2EXT",
    "terminator_vkGetPhysicalDeviceSurfaceCapabilities2KHR",
    "terminator_vkGetPhysicalDeviceSurfaceFormats2KHR",
    "terminator_vkGetPhysicalDeviceSurfaceSupportKHR",
    "terminator_vkGetPhysicalDeviceToolProperties",
    "terminator_vkGetPhysicalDeviceToolPropertiesEXT",
    "translate_physical_device_surface",
    "vkCreateDebugReportCallbackEXT",
    "vkCreateDebugUtilsMessengerEXT",
    "vkCreateDevice",
    "vkCreateInstance",
    "vkCreateSharedSwapchainsKHR",
    "vkCreateSwapchainKHR",
    "vkDebugMarkerSetObjectNameEXT",
    "vkDebugMarkerSetObjectTagEXT",
    "vkDebugReportMessageEXT",
    "vkDestroyDebugReportCallbackEXT",
    "vkDestroyDebugUtilsMessengerEXT",
    "vkDestroyDevice",
    "vkDestroyInstance",
    "vkDestroySurfaceKHR",
    "vkEnumerateDeviceExtensionProperties",
    "vkEnumerateDeviceLayerProperties",
    "vkEnumerateInstanceExtensionProperties",
    "vkEnumerateInstanceLayerProperties",
    "vkEnumerateInstanceVersion",
    "vkEnumeratePhysicalDeviceGroups",
    "vkEnumeratePhysicalDeviceGroupsKHR",
    "vkEnumeratePhysicalDevices",
    "vkGetDeviceGroupSurfacePresentModesKHR",
    "vkGetDeviceProcAddr",
    "vkGetInstanceProcAddr",
    "vkSetDebugUtilsObjectNameEXT",
    "vkSetDebugUtilsObjectTagEXT",
    "vkSubmitDebugUtilsMessageEXT",
];

#[derive(Default)]
struct UsedIdentifiers(BTreeSet<String>);

impl<'ast> syn::visit::Visit<'ast> for UsedIdentifiers {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        if path.leading_colon.is_none()
            && let Some(identifier) = path.segments.first()
        {
            self.0.insert(identifier.ident.to_string());
        }
        syn::visit::visit_path(self, path);
    }
}

fn generated_item_identifier(item: &syn::Item) -> Option<&syn::Ident> {
    if matches!(item, syn::Item::Const(item) if item.ident == "_") {
        return None;
    }
    match item {
        syn::Item::Const(item) => Some(&item.ident),
        syn::Item::Enum(item) => Some(&item.ident),
        syn::Item::Fn(item) => Some(&item.sig.ident),
        syn::Item::Static(item) => Some(&item.ident),
        syn::Item::Struct(item) => Some(&item.ident),
        syn::Item::Type(item) => Some(&item.ident),
        _ => None,
    }
}

fn generated_item_attributes(item: &syn::Item) -> &[syn::Attribute] {
    match item {
        syn::Item::Const(item) => &item.attrs,
        syn::Item::Enum(item) => &item.attrs,
        syn::Item::Fn(item) => &item.attrs,
        syn::Item::Static(item) => &item.attrs,
        syn::Item::Struct(item) => &item.attrs,
        syn::Item::Type(item) => &item.attrs,
        _ => &[],
    }
}

fn generated_loader_part(item: &syn::Item) -> &'static str {
    match item {
        syn::Item::Fn(item) => {
            let name = item.sig.ident.to_string();
            match name.as_str() {
                name if name.starts_with("terminator_") => "terminators",
                name if name.starts_with("vk") => "trampolines",
                name if name.contains("proc_addr") => "proc_addr",
                name if name.starts_with("convert_") => "debug",
                "extension_id"
                | "is_known_instance_extension"
                | "surface_create_info_extension_size"
                | "wsi_instance_extension_supported" => "extensions",
                "handle_info" => "handles",
                _ => "commands",
            }
        }
        syn::Item::Struct(item) => {
            let name = item.ident.to_string();
            match name.as_str() {
                name if name.contains("DispatchTable") => "dispatch_tables",
                name if name.contains("Extension") => "extensions",
                _ => "commands",
            }
        }
        syn::Item::Impl(item) => {
            let self_ty = &item.self_ty;
            let self_type = quote! { #self_ty }.to_string();
            match self_type.as_str() {
                name if name.contains("DispatchTable") => "dispatch_tables",
                name if name.contains("ExtensionSet") => "extensions",
                _ => "commands",
            }
        }
        syn::Item::Static(item) => {
            let name = item.ident.to_string();
            match name.as_str() {
                name if name.contains("EXTENSION") => "extensions",
                "HANDLE_INFOS" => "handles",
                _ => "commands",
            }
        }
        syn::Item::Const(item) => {
            let name = item.ident.to_string();
            match name.as_str() {
                name if name.contains("EXTENSION_ID") => "extensions",
                _ => "commands",
            }
        }
        _ => "commands",
    }
}

fn generated_source(syntax: &syn::File) -> String {
    let formatted = prettyplease::unparse(syntax);
    let mut output = String::with_capacity(formatted.len() + 80);
    output.push_str("// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.\n\n");
    output.push_str(&formatted);
    output
}

fn expose_to_generated_siblings(item: &mut syn::Item) {
    if matches!(item, syn::Item::Const(item) if item.ident == "_") {
        return;
    }
    let visibility = match item {
        syn::Item::Const(item) => &mut item.vis,
        syn::Item::Enum(item) => &mut item.vis,
        syn::Item::Fn(item) => &mut item.vis,
        syn::Item::Static(item) => &mut item.vis,
        syn::Item::Struct(item) => &mut item.vis,
        syn::Item::Type(item) => &mut item.vis,
        _ => return,
    };
    if matches!(visibility, syn::Visibility::Inherited) {
        *visibility = syn::parse_quote! { pub(super) };
    }
}

fn main() {
    let mut args = env::args_os().skip(1);
    let usage = "usage: vk-loader-codegen VK_XML OUT_RS LOADER_CARGO_TOML";
    let registry_path = PathBuf::from(args.next().expect(usage));
    let output_path = PathBuf::from(args.next().expect(usage));
    let cargo_path = PathBuf::from(args.next().expect(usage));
    if args.next().is_some() {
        eprintln!("{usage}");
        process::exit(2);
    }

    let xml = fs::read_to_string(&registry_path).expect("read Vulkan registry");
    let mut registry = parse_registry(&xml);
    apply_require_extensions(&mut registry);
    update_loader_features(&cargo_path, &registry);
    let extension_name_constants = registry
        .constants
        .values()
        .flatten()
        .filter(|constant| constant.name.ends_with("_EXTENSION_NAME"))
        .filter_map(|constant| {
            let literal = constant.value.trim().strip_prefix('"')?.strip_suffix('"')?;
            Some((literal.to_owned(), constant.name.clone()))
        })
        .collect::<HashMap<_, _>>();
    let mut extension_records = registry
        .extensions
        .iter()
        .filter(|extension| {
            extension.api.vulkan && extension.supports_vulkan() && !extension.is_disabled()
        })
        .map(|extension| {
            let constant = extension_name_constants
                .get(&extension.name)
                .unwrap_or_else(|| panic!("missing extension-name constant for {}", extension.name))
                .clone();
            (
                extension.name.clone(),
                constant,
                extension.ext_type.as_deref() == Some("device"),
            )
        })
        .collect::<Vec<_>>();
    extension_records.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    let extension_ids = extension_records
        .iter()
        .enumerate()
        .map(|(id, (name, ..))| {
            (name.as_str(), {
                assert!(id <= usize::from(u16::MAX), "extension count exceeds u16");
                id as u16
            })
        })
        .collect::<HashMap<_, _>>();
    let mut compile_time_extension_ids = HashSet::from([
        "VK_EXT_surface_maintenance1".to_owned(),
        "VK_KHR_surface_maintenance1".to_owned(),
    ]);
    let mut surface_create_info_types = HashSet::new();
    for command in registry.commands.values().flatten() {
        let creates_surface = command.params.len() == 4
            && command
                .params
                .first()
                .is_some_and(|parameter| parameter.ty.base == "VkInstance")
            && command
                .params
                .last()
                .is_some_and(|parameter| parameter.ty.base == "VkSurfaceKHR")
            && command.name.starts_with("vkCreate");
        if creates_surface
            && let Some(extension) = command.provided_by.iter().find(|provider| {
                registry
                    .extensions
                    .iter()
                    .any(|extension| extension.name == provider.as_str())
            })
        {
            compile_time_extension_ids.insert(extension.clone());
            surface_create_info_types.insert(command.params[1].ty.base.clone());
        }
    }
    let extension_word_count = extension_records.len().div_ceil(u64::BITS as usize);
    let mut instance_extension_words = vec![0_u64; extension_word_count];
    for (id, (_, _, is_device)) in extension_records.iter().enumerate() {
        if !is_device {
            instance_extension_words[id / 64] |= 1_u64 << (id % 64);
        }
    }
    let mut globals = registry
        .commands
        .values()
        .flatten()
        .filter(|command| command.alias.is_none())
        .filter(|command| command.export.contains(&ExportScope::Vulkan))
        .filter(|command| command_scope(&registry, command) == Scope::Global)
        .map(|command| command.name.as_str())
        .collect::<Vec<_>>();
    globals.sort_unstable();
    globals.dedup();

    let global_arms = globals.iter().map(|name| {
        let bytes = Literal::byte_string(name.as_bytes());
        let name = format_ident!("{name}");
        quote! { #bytes => Some(erase_function(#name as *const ())), }
    });
    let mut generated = quote! {
        pub(crate) fn global_proc_addr(name: &CStr) -> PFN_vkVoidFunction {
            match name.to_bytes() {
                #(#global_arms)*
                _ => None,
            }
        }
    };
    let debug_object_pairs = debug_object_type_pairs(&registry);
    let debug_object_pairs = debug_object_pairs
        .iter()
        .filter(|(core, _)| *core != "UNKNOWN")
        .map(|(core, debug)| {
            (
                format_ident!("{core}"),
                format_ident!("{}", debug_report_variant(debug)),
            )
        })
        .collect::<Vec<_>>();
    let report_to_core = debug_object_pairs.iter().map(|(core, debug)| {
        quote! {
            vk::VkDebugReportObjectTypeEXT::#debug => vk::VkObjectType::#core,
        }
    });
    let core_to_report = debug_object_pairs.iter().map(|(core, debug)| {
        quote! {
            vk::VkObjectType::#core => vk::VkDebugReportObjectTypeEXT::#debug,
        }
    });
    generated.extend(quote! {
            #[allow(dead_code)]
            #[inline]
            pub(crate) const fn convert_debug_report_object_to_core_object(
                object_type: vk::VkDebugReportObjectTypeEXT,
            ) -> vk::VkObjectType {
                match object_type {
                    #(#report_to_core)*
                    _ => vk::VkObjectType::UNKNOWN,
                }
            }

            #[allow(dead_code)]
            #[inline]
            pub(crate) const fn convert_core_object_to_debug_report_object(
                object_type: vk::VkObjectType,
            ) -> vk::VkDebugReportObjectTypeEXT {
                match object_type {
                    #(#core_to_report)*
                    _ => vk::VkDebugReportObjectTypeEXT::UNKNOWN,
                }
            }
    });
    let extension_name_count = Literal::usize_unsuffixed(extension_records.len());
    let extension_word_count = Literal::usize_unsuffixed(extension_word_count);
    let instance_extension_words = instance_extension_words
        .iter()
        .copied()
        .map(u64_hex_literal)
        .collect::<Vec<_>>();
    let extension_names = extension_records.iter().map(|(_, constant, _)| {
        let constant = format_ident!("{constant}");
        quote! { ExtensionName(vk::#constant), }
    });
    let mut surface_chain_extensions = registry
        .structs
        .values()
        .flatten()
        .filter(|structure| structure.alias.is_none() && structure.api.vulkan)
        .flat_map(|structure| {
            structure
                .struct_extends
                .iter()
                .filter(|root| surface_create_info_types.contains(root.as_str()))
                .map(|root| (root.clone(), structure.name.clone()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    surface_chain_extensions.sort_unstable();
    surface_chain_extensions.dedup();
    let surface_extension_arms = surface_chain_extensions.iter().map(|(root, extension)| {
        let root_type = rust_structure_type_constant(structure_type_constant(&registry, root));
        let extension_type =
            rust_structure_type_constant(structure_type_constant(&registry, extension));
        let root_type = format_ident!("{root_type}");
        let extension_type = format_ident!("{extension_type}");
        let extension = format_ident!("{extension}");
        quote! {
            (VkStructureType::#root_type, VkStructureType::#extension_type) => {
                Some(core::mem::size_of::<vk::#extension<'static>>())
            }
        }
    });
    let extension_id_constants = extension_records
        .iter()
        .enumerate()
        .filter(|(_, (name, _, _))| compile_time_extension_ids.contains(name))
        .map(|(id, (name, _, _))| {
            let protect = registry
                .extensions
                .iter()
                .find(|extension| extension.name == *name)
                .and_then(|extension| extension.platform.as_deref())
                .and_then(|platform| registry.platforms.get(platform))
                .map(String::as_str);
            let cfg = platform_cfg(protect);
            let constant = format_ident!("{}_EXTENSION_ID", name.to_ascii_uppercase());
            let id = Literal::usize_unsuffixed(id);
            quote! { #cfg pub(crate) const #constant: u16 = #id; }
        });
    let wsi_guards = BUILD_FILTERED_WSI_EXTENSIONS.iter().map(|extension_name| {
        let extension = registry
            .extensions
            .iter()
            .find(|extension| extension.name == **extension_name)
            .unwrap_or_else(|| panic!("missing WSI extension {extension_name}"));
        let protect = extension
            .platform
            .as_deref()
            .and_then(|platform| registry.platforms.get(platform))
            .unwrap_or_else(|| panic!("missing platform guard for {extension_name}"));
        let constant = extension_name_constants
            .get(*extension_name)
            .unwrap_or_else(|| panic!("missing extension-name constant for {extension_name}"));
        let cfg = rust_platform_cfg(protect)
            .parse::<TokenStream>()
            .expect("platform cfg must be valid Rust tokens");
        let constant = format_ident!("{constant}");
        quote! { #[cfg(not(#cfg))] candidate if candidate == vk::#constant => false, }
    });
    generated.extend(quote! {
        #[derive(Clone, Copy, Default)]
        pub(crate) struct ExtensionSet { words: [u64; #extension_word_count] }
        static INSTANCE_EXTENSION_WORDS: [u64; #extension_word_count] = [#(#instance_extension_words),*];
        #[repr(transparent)]
        struct ExtensionName(&'static CStr);
        static EXTENSION_NAMES: [ExtensionName; #extension_name_count] = [#(#extension_names)*];
        pub(crate) const fn surface_create_info_extension_size(root: VkStructureType, structure_type: VkStructureType) -> Option<usize> {
            match (root, structure_type) { #(#surface_extension_arms)* _ => None }
        }
        #(#extension_id_constants)*
        impl ExtensionSet {
            pub(crate) unsafe fn from_names(count: u32, names: *const *const c_char) -> Self {
                let mut set = Self::default();
                if names.is_null() { return set; }
                for index in 0..count as usize {
                    let name = unsafe { names.add(index).read() };
                    if !name.is_null() && let Some(id) = extension_id(unsafe { CStr::from_ptr(name) }) { set.insert(id); }
                }
                set
            }
            fn insert(&mut self, id: u16) {
                let index = usize::from(id);
                debug_assert!(index < EXTENSION_NAMES.len());
                unsafe { *self.words.get_unchecked_mut(index / 64) |= 1_u64 << (index % 64) };
            }
            pub(crate) fn contains(&self, id: u16) -> bool {
                let index = usize::from(id);
                debug_assert!(index < EXTENSION_NAMES.len());
                (unsafe { *self.words.get_unchecked(index / 64) } & (1_u64 << (index % 64))) != 0
            }
            pub(crate) fn contains_name(&self, name: &CStr) -> bool { extension_id(name).is_some_and(|id| self.contains(id)) }
        }
        #[cold]
        pub(crate) fn extension_id(name: &CStr) -> Option<u16> {
            EXTENSION_NAMES
                .binary_search_by(|candidate| candidate.0.to_bytes().cmp(name.to_bytes()))
                .ok()
                .map(|id| {
                    debug_assert!(id <= usize::from(u16::MAX));
                    id as u16
                })
        }
        #[cold]
        pub(crate) fn is_known_instance_extension(name: &CStr) -> bool {
            let Some(id) = extension_id(name) else { return false; };
            let index = usize::from(id);
            (unsafe { *INSTANCE_EXTENSION_WORDS.get_unchecked(index / 64) } & (1_u64 << (index % 64))) != 0
        }
        #[cold]
        pub(crate) fn wsi_instance_extension_supported(name: &CStr) -> bool {
            match name { #(#wsi_guards)* _ => true }
        }
    });

    let is_vulkan_command = |command: &&vk_codegen::ir::Command| {
        (command.export.is_empty() || command.export.contains(&ExportScope::Vulkan))
            && (command.api.vulkan || command.api.vulkanbase)
            && command_has_vulkan_provider(&registry, command)
    };
    let commands = registry.commands.values().flatten().collect::<Vec<_>>();
    for (scope, table_name) in [
        (Scope::Instance, "InstanceDispatchTable"),
        (Scope::Device, "DeviceDispatchTable"),
    ] {
        let mut scoped = commands
            .iter()
            .copied()
            .filter(is_vulkan_command)
            .filter(|command| {
                let command_scope = command_scope(&registry, command);
                command_scope == scope
                    || (scope == Scope::Instance && command.name == "vkGetDeviceProcAddr")
            })
            .map(|command| command.name.as_str())
            .collect::<Vec<_>>();
        scoped.sort_unstable();
        scoped.dedup();

        let table_name = format_ident!("{table_name}");
        let fields = scoped.iter().map(|name| {
            let name = format_ident!("{name}");
            let pfn = format_ident!("PFN_{name}");
            quote! { pub(crate) #name: Option<vk::#pfn>, }
        });
        let (loader_type, handle_type, loader_name) = match scope {
            Scope::Instance => (
                quote! { vk::PFN_vkGetInstanceProcAddr },
                quote! { vk::VkInstance },
                format_ident!("gipa"),
            ),
            Scope::Device => (
                quote! { vk::PFN_vkGetDeviceProcAddr },
                quote! { vk::VkDevice },
                format_ident!("gdpa"),
            ),
            Scope::Global => unreachable!(),
        };
        let load_fields = scoped.iter().map(|name| {
            let literal = c_string_literal(name);
            let name = format_ident!("{name}");
            quote! { #name: unsafe { load_typed(#loader_name(handle, #literal.as_ptr())) }, }
        });
        let load_into_fields = scoped.iter().map(|name| {
            let literal = c_string_literal(name);
            let name = format_ident!("{name}");
            quote! {
                unsafe { core::ptr::addr_of_mut!((*table).#name).write(load_typed(#loader_name(handle, #literal.as_ptr()))); }
            }
        });
        generated.extend(quote! {
            #[allow(dead_code)]
            #[derive(Clone, Default)]
            pub(crate) struct #table_name { #(#fields)* }
            #[allow(dead_code)]
            impl #table_name {
                #[allow(clippy::too_many_lines)] // One generated field initializer per registry command.
                pub(crate) unsafe fn load(#loader_name: #loader_type, handle: #handle_type) -> Self {
                    Self { #(#load_fields)* }
                }
                #[allow(clippy::too_many_lines)] // One generated field write per registry command.
                pub(crate) unsafe fn load_into(table: *mut Self, #loader_name: #loader_type, handle: #handle_type) {
                    #(#load_into_fields)*
                }
            }
        });

        if scope == Scope::Instance {
            let mut required = commands
                .iter()
                .copied()
                .filter(is_vulkan_command)
                .filter(|command| is_required_icd_instance_command(&registry, command))
                .map(|command| command.name.as_str())
                .collect::<Vec<_>>();
            required.sort_unstable();
            required.dedup();
            let required = required
                .iter()
                .map(|name| format_ident!("{name}"))
                .collect::<Vec<_>>();
            let (first, rest) = required
                .split_first()
                .expect("Vulkan 1.0 has required ICD instance commands");
            generated.extend(quote! {
                    impl InstanceDispatchTable {
                        pub(crate) const fn has_required_core_1_0(&self) -> bool {
                            self.#first.is_some() #(&& self.#rest.is_some())*
                        }
                    }
            });
        }
    }

    let mut ordered_commands = Vec::new();
    let mut ordered_command_names = HashSet::new();
    // Match the Vulkan registry generator's `regSortFeatures` order. The
    // upstream loader's public layer dispatch-table ABI is the insertion order
    // produced while visiting core versions, Khronos extensions by number, and
    // then vendor extensions by number. Raw XML extension order is not that ABI.
    let mut ordered_extensions = registry
        .extensions
        .iter()
        .filter(|extension| {
            extension.api.vulkan && extension.supports_vulkan() && !extension.is_disabled()
        })
        .collect::<Vec<_>>();
    ordered_extensions.sort_by_key(|extension| {
        let vendor = extension.name.split('_').nth(1).unwrap_or_default();
        let category = u8::from(!matches!(vendor, "ARB" | "KHR" | "OES"));
        (extension.sort_order, category, extension.number)
    });
    let ordered_providers = registry
        .features
        .iter()
        .filter(|feature| feature.api.vulkan || feature.api.vulkanbase)
        .flat_map(|feature| feature.requires.iter())
        .chain(
            ordered_extensions
                .iter()
                .flat_map(|extension| extension.requires.iter()),
        );
    for require in ordered_providers {
        if require.api.as_ref().is_some_and(|api| !api.vulkan) {
            continue;
        }
        for name in &require.commands {
            // The Khronos base generator materializes an alias target before
            // the alias command. That insertion order is observable in the
            // layer dispatch-table ABI (notably the NV/KHR ray-tracing alias).
            let mut candidate = name.as_str();
            let mut alias_chain = vec![candidate];
            while let Some(alias) = registry
                .commands
                .get(candidate)
                .and_then(|variants| variants.first())
                .and_then(|command| command.alias.as_deref())
            {
                alias_chain.push(alias);
                candidate = alias;
            }
            for candidate in alias_chain.into_iter().rev() {
                if !ordered_command_names.insert(candidate) {
                    continue;
                }
                let Some(command) = registry.commands.get(candidate).and_then(|variants| {
                    variants
                        .iter()
                        .find(|command| command.api.vulkan || command.api.vulkanbase)
                        .or_else(|| variants.first())
                }) else {
                    continue;
                };
                if is_vulkan_command(&command) {
                    ordered_commands.push(command);
                }
            }
        }
    }
    let layer_instance_commands = ordered_commands
        .iter()
        .copied()
        .filter(|command| command_scope(&registry, command) != Scope::Device)
        .collect::<Vec<_>>();
    let layer_instance_fields = layer_instance_commands.iter().map(|command| {
        let cfg = platform_cfg(command_platform_protect(&registry, command));
        let name = format_ident!("{}", command.name);
        let pfn = format_ident!("PFN_{}", command.name);
        quote! { #cfg pub(crate) #name: Option<vk::#pfn>, }
    });
    let layer_instance_loads = layer_instance_commands.iter().map(|command| {
        let cfg = platform_cfg(command_platform_protect(&registry, command));
        let name = format_ident!("{}", command.name);
        if command.name == "vkGetInstanceProcAddr" {
            quote! {
                #cfg unsafe { core::ptr::addr_of_mut!((*table_ptr).#name).write(Some(gipa)); }
            }
        } else {
            let literal = c_string_literal(&command.name);
            quote! {
                #cfg unsafe { core::ptr::addr_of_mut!((*table_ptr).#name).write(load_typed(gipa(instance, #literal.as_ptr()))); }
            }
        }
    });
    let layer_device_commands = ordered_commands
        .iter()
        .copied()
        .filter(|command| command_scope(&registry, command) == Scope::Device)
        .collect::<Vec<_>>();
    let layer_device_fields = layer_device_commands.iter().map(|command| {
        let cfg = platform_cfg(command_platform_protect(&registry, command));
        let name = format_ident!("{}", command.name);
        let pfn = format_ident!("PFN_{}", command.name);
        quote! { #cfg pub(crate) #name: Option<vk::#pfn>, }
    });
    let layer_device_offsets = layer_device_commands.iter().map(|command| {
        let name = format_ident!("{}", command.name);
        let offset = format_ident!(
            "{}_DEVICE_DISPATCH_OFFSET",
            screaming_snake_case(&command.name)
        );
        if let Some(protect) = command_platform_protect(&registry, command) {
            let cfg = rust_platform_cfg(protect)
                .parse::<TokenStream>()
                .expect("platform cfg must be valid Rust tokens");
            quote! {
                #[cfg(#cfg)]
                const #offset: u16 = dispatch_offset(core::mem::offset_of!(LayerDeviceDispatchTable, #name));
                #[cfg(not(#cfg))]
                const #offset: u16 = u16::MAX;
            }
        } else {
            quote! {
                const #offset: u16 = dispatch_offset(core::mem::offset_of!(LayerDeviceDispatchTable, #name));
            }
        }
    });
    let layer_device_loads = layer_device_commands.iter().map(|command| {
        let cfg = platform_cfg(command_platform_protect(&registry, command));
        let name = format_ident!("{}", command.name);
        if command.name == "vkGetDeviceProcAddr" {
            quote! {
                #cfg unsafe { core::ptr::addr_of_mut!((*table_ptr).#name).write(Some(gdpa)); }
            }
        } else {
            let literal = c_string_literal(&command.name);
            quote! {
                #cfg unsafe { core::ptr::addr_of_mut!((*table_ptr).#name).write(load_typed(gdpa(device, #literal.as_ptr()))); }
            }
        }
    });
    let layer_device_masks = layer_device_commands.iter().map(|command| {
        let cfg = platform_cfg(command_platform_protect(&registry, command));
        let name = format_ident!("{}", command.name);
        let id = format_ident!("{}_COMMAND_ID", screaming_snake_case(&command.name));
        quote! { #cfg if !available(#id) { self.#name = None; } }
    });
    // Preserve the joint `::<` punctuation inside the macro input. `quote!`
    // separates it because `<` can otherwise begin a comparison expression.
    generated.extend(quote! {
        #[repr(C)]
        #[allow(dead_code)]
        pub(crate) struct LayerInstanceDispatchTable {
            pub(crate) vk_layerGetPhysicalDeviceProcAddr: crate::layer::GetPhysicalDeviceProcAddr,
            #(#layer_instance_fields)*
        }
        #[allow(dead_code)]
        impl LayerInstanceDispatchTable {
            #[allow(clippy::too_many_lines)] // One generated field write per registry command.
            pub(crate) unsafe fn load_into(table_ptr: *mut Self, gipa: vk::PFN_vkGetInstanceProcAddr, gpdpa: crate::layer::GetPhysicalDeviceProcAddr, instance: vk::VkInstance) {
                unsafe { core::ptr::addr_of_mut!((*table_ptr).vk_layerGetPhysicalDeviceProcAddr).write(gpdpa); }
                #(#layer_instance_loads)*
            }
            pub(crate) unsafe fn load_boxed(gipa: vk::PFN_vkGetInstanceProcAddr, gpdpa: crate::layer::GetPhysicalDeviceProcAddr, instance: vk::VkInstance) -> Box<Self> {
                let mut table = Box::<Self>::new_uninit();
                unsafe { Self::load_into(table.as_mut_ptr(), gipa, gpdpa, instance) };
                unsafe { table.assume_init() }
            }
        }
        #[repr(C)]
        #[allow(dead_code)]
        pub(crate) struct LayerDeviceDispatchTable {
            pub(crate) magic: u64,
            #(#layer_device_fields)*
        }
        // Every generated byte offset below is stored as `u16`. Keep the cast
        // infallible on every target and feature combination at compile time.
        const _: () = assert!(core::mem::size_of::<LayerDeviceDispatchTable>() <= 65_535);
        #(#layer_device_offsets)*
        #[allow(dead_code)]
        impl LayerDeviceDispatchTable {
            #[allow(clippy::too_many_lines)] // One generated field write per registry command.
            pub(crate) unsafe fn load_into(table_ptr: *mut Self, gdpa: vk::PFN_vkGetDeviceProcAddr, device: vk::VkDevice) {
                unsafe { core::ptr::addr_of_mut!((*table_ptr).magic).write(DEVICE_DISPATCH_MAGIC); }
                #(#layer_device_loads)*
            }
            pub(crate) unsafe fn load_boxed(gdpa: vk::PFN_vkGetDeviceProcAddr, device: vk::VkDevice) -> Box<Self> {
                let mut table = Box::<Self>::new_uninit();
                unsafe { Self::load_into(table.as_mut_ptr(), gdpa, device) };
                unsafe { table.assume_init() }
            }
            #[allow(clippy::too_many_lines)] // One generated availability check per registry command.
            pub(crate) fn mask_unavailable(&mut self, mut available: impl FnMut(u16) -> bool) {
                #(#layer_device_masks)*
            }
        }
    });

    let icd_terminator_commands = ICD_DEVICE_TERMINATOR_COMMANDS
        .iter()
        .map(|name| {
            registry
                .commands
                .get(*name)
                .and_then(|commands| {
                    commands
                        .iter()
                        .find(|command| command.api.vulkan || command.api.vulkanbase)
                        .or_else(|| commands.first())
                })
                .unwrap_or_else(|| panic!("missing ICD terminator command {name}"))
        })
        .collect::<Vec<_>>();
    let icd_terminator_fields = icd_terminator_commands.iter().map(|command| {
        let cfg = platform_cfg(command_platform_protect(&registry, command));
        let name = format_ident!("{}", command.name);
        let pfn = format_ident!("PFN_{}", command.name);
        quote! { #cfg pub(crate) #name: Option<vk::#pfn>, }
    });
    let icd_terminator_loads = icd_terminator_commands.iter().map(|command| {
        let cfg = platform_cfg(command_platform_protect(&registry, command));
        let name = format_ident!("{}", command.name);
        let literal = c_string_literal(&command.name);
        quote! {
            #cfg #name: if available(#literal) {
                unsafe { load_typed(gdpa(device, #literal.as_ptr())) }
            } else { None },
        }
    });
    generated.extend(quote! {
        #[allow(dead_code)]
        pub(crate) struct IcdDeviceTerminatorDispatchTable { #(#icd_terminator_fields)* }
        #[allow(dead_code)]
        impl IcdDeviceTerminatorDispatchTable {
            pub(crate) unsafe fn load_boxed(gdpa: vk::PFN_vkGetDeviceProcAddr, device: vk::VkDevice, mut available: impl FnMut(&CStr) -> bool) -> Box<Self> {
                Box::new(Self { #(#icd_terminator_loads)* })
            }
        }
    });

    let mut handles = registry
        .typedefs
        .values()
        .flatten()
        .filter_map(|ty| match &ty.kind {
            TypedefKind::Handle {
                dispatchable,
                parent,
                objtypeenum,
            } => Some((
                ty.name.as_str(),
                *dispatchable,
                parent.as_deref(),
                objtypeenum.as_deref(),
                ty.alias.as_deref(),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    handles.sort_unstable_by_key(|handle| handle.0);
    let handle_count = Literal::usize_unsuffixed(handles.len());
    let handle_infos =
        handles
            .into_iter()
            .map(|(name, dispatchable, parent, object_type, alias)| {
                let option = |value: Option<&str>| match value {
                    Some(value) => quote! { Some(#value) },
                    None => quote! { None },
                };
                let parent = option(parent);
                let object_type = option(object_type);
                let alias = option(alias);
                quote! {
                    HandleInfo {
                        name: #name,
                        dispatchable: #dispatchable,
                        parent: #parent,
                        object_type: #object_type,
                        alias: #alias,
                    },
                }
            });
    generated.extend(quote! {
        #[allow(dead_code)]
        const HANDLE_INFOS: [HandleInfo; #handle_count] = [#(#handle_infos)*];
        #[allow(dead_code)]
        pub(crate) fn handle_info(name: &str) -> Option<HandleInfo> {
            HANDLE_INFOS.binary_search_by_key(&name, |info| info.name).ok().map(|index| HANDLE_INFOS[index])
        }
    });

    let mut command_names = registry.commands.keys().collect::<Vec<_>>();
    command_names.sort_unstable();
    let mut command_records = Vec::new();
    for name in command_names {
        let Some(command) = registry.commands[name]
            .iter()
            .find(|command| command.api.vulkan || command.api.vulkanbase)
            .or_else(|| registry.commands[name].first())
        else {
            continue;
        };
        if !command.export.is_empty() && !command.export.contains(&ExportScope::Vulkan) {
            continue;
        }
        if !command_has_vulkan_provider(&registry, command) {
            continue;
        }
        let scope = match command_scope(&registry, command) {
            Scope::Global => "CommandScope::Global",
            Scope::Instance => "CommandScope::Instance",
            Scope::Device => "CommandScope::Device",
        };
        let alias = command
            .alias
            .as_ref()
            .map_or_else(|| "None".to_owned(), |alias| format!("Some(\"{alias}\")"));
        let core_version = command
            .provided_by
            .iter()
            .filter(|provider| provider.contains("_VERSION_"))
            .filter_map(|provider| {
                provider
                    .rsplit_once("_VERSION_")
                    .and_then(|(_, version)| version.split_once('_'))
                    .and_then(|(major, minor)| {
                        Some((major.parse::<u16>().ok()?, minor.parse::<u16>().ok()?))
                    })
            })
            .min()
            .map_or(0, |(major, minor)| {
                assert!(
                    major < 64 && minor < 1024,
                    "core API version exceeds packing"
                );
                (major << 10) | minor
            });
        let mut extensions = command
            .provided_by
            .iter()
            .filter(|provider| provider.starts_with("VK_") && !provider.contains("_VERSION_"))
            .filter_map(|provider| extension_ids.get(provider.as_str()).copied())
            .collect::<Vec<_>>();
        extensions.sort_unstable();
        extensions.dedup();
        let (device_extensions, instance_extensions): (Vec<_>, Vec<_>) = extensions
            .into_iter()
            .partition(|id| extension_records[usize::from(*id)].2);
        command_records.push((
            name.as_str(),
            scope,
            alias,
            core_version,
            instance_extensions,
            device_extensions,
        ));
    }
    let device_command_ids = command_records
        .iter()
        .enumerate()
        .filter(|(_, record)| record.1 == "CommandScope::Device")
        .map(|(id, record)| {
            let name = format_ident!("{}_COMMAND_ID", screaming_snake_case(record.0));
            let id = Literal::usize_unsuffixed(id);
            quote! { #[allow(dead_code)] const #name: u16 = #id; }
        })
        .collect::<Vec<_>>();
    generated.extend(quote! { #(#device_command_ids)* });
    let table_len = command_records.len().next_power_of_two();
    let bucket_count = table_len / 2;
    let mut buckets = vec![Vec::new(); bucket_count];
    for (record_index, record) in command_records.iter().enumerate() {
        let suffix = record
            .0
            .as_bytes()
            .strip_prefix(b"vk")
            .expect("Vulkan command name must start with vk");
        let bucket = command_hash(suffix) as usize & (bucket_count - 1);
        buckets[bucket].push(record_index);
    }
    let mut bucket_order = (0..bucket_count).collect::<Vec<_>>();
    bucket_order.sort_unstable_by_key(|bucket| core::cmp::Reverse(buckets[*bucket].len()));
    let mut slots = vec![None; table_len];
    let mut displacements = vec![0_u16; bucket_count];
    let mut candidate_slots = Vec::new();
    let mut max_displacement = 0;
    for bucket in bucket_order {
        if buckets[bucket].is_empty() {
            break;
        }
        let displacement = (0..=u16::MAX)
            .find(|displacement| {
                candidate_slots.clear();
                for record_index in &buckets[bucket] {
                    let suffix = command_records[*record_index].0.as_bytes()[2..].as_ref();
                    let hash = command_hash(suffix);
                    let slot = command_slot_hash(hash ^ u64::from(*displacement)) as usize
                        & (table_len - 1);
                    if slots[slot].is_some() || candidate_slots.contains(&slot) {
                        return false;
                    }
                    candidate_slots.push(slot);
                }
                true
            })
            .expect("could not generate collision-free command lookup");
        for (record_index, slot) in buckets[bucket].iter().zip(&candidate_slots) {
            slots[*slot] = Some(*record_index);
        }
        displacements[bucket] = displacement;
        max_displacement = max_displacement.max(displacement);
    }
    let mut command_names = String::new();
    let mut name_ranges = Vec::with_capacity(command_records.len());
    for (name, ..) in &command_records {
        let suffix = name.strip_prefix("vk").expect("Vulkan command prefix");
        assert!(
            command_names.len() <= usize::from(u16::MAX),
            "command-name blob exceeds u16"
        );
        let offset = command_names.len() as u16;
        assert!(
            suffix.len() <= usize::from(u8::MAX),
            "command name exceeds u8"
        );
        let len = suffix.len() as u8;
        command_names.push_str(suffix);
        name_ranges.push((offset, len));
    }
    let command_names_literal = Literal::byte_string(command_names.as_bytes());
    let table_len_literal = Literal::usize_unsuffixed(table_len);
    let command_table = slots.into_iter().map(|slot| {
        if let Some(record_index) = slot {
            let (_, scope, ..) = &command_records[record_index];
            let (name_offset, name_len) = name_ranges[record_index];
            let name_offset = Literal::u16_unsuffixed(name_offset);
            let record_index = Literal::usize_unsuffixed(record_index);
            let name_len = Literal::u8_unsuffixed(name_len);
            let scope = scope
                .parse::<TokenStream>()
                .expect("command scope must be valid Rust tokens");
            quote! {
                CommandRecord { name_offset: #name_offset, id: #record_index, name_len: #name_len, scope: #scope },
            }
        } else {
            quote! {
                CommandRecord { name_offset: 0, id: u16::MAX, name_len: 0, scope: CommandScope::Global },
            }
        }
    }).collect::<Vec<_>>();
    let bucket_count = Literal::usize_unsuffixed(bucket_count);
    let displacements = displacements
        .into_iter()
        .map(Literal::u16_unsuffixed)
        .collect::<Vec<_>>();
    let core_levels = command_records
        .iter()
        .map(|record| Literal::u16_unsuffixed(record.3))
        .collect::<Vec<_>>();
    let device_dispatch_offsets = command_records
        .iter()
        .map(|(name, scope, ..)| {
            if *scope == "CommandScope::Device" {
                let offset = format_ident!("{}_DEVICE_DISPATCH_OFFSET", screaming_snake_case(name));
                quote! { #offset, }
            } else {
                quote! { u16::MAX, }
            }
        })
        .collect::<Vec<_>>();
    let mut loader_trampoline_words =
        vec![0_u64; command_records.len().div_ceil(u64::BITS as usize)];
    for name in LOADER_DEVICE_TRAMPOLINES {
        let id = command_records
            .iter()
            .position(|record| record.0 == *name)
            .unwrap_or_else(|| panic!("loader device trampoline metadata for {name}"));
        assert_eq!(
            command_records[id].1, "CommandScope::Device",
            "loader trampoline must be a device command: {name}"
        );
        loader_trampoline_words[id / u64::BITS as usize] |= 1_u64 << (id % u64::BITS as usize);
    }
    let loader_trampoline_word_count = Literal::usize_unsuffixed(loader_trampoline_words.len());
    let loader_trampoline_words = loader_trampoline_words
        .into_iter()
        .map(u64_hex_literal)
        .collect::<Vec<_>>();
    let mut extension_metadata = TokenStream::new();
    for (kind, extension_index) in [("INSTANCE", 4_usize), ("DEVICE", 5_usize)] {
        let mut ids = Vec::new();
        let mut ranges = Vec::with_capacity(command_records.len());
        for record in &command_records {
            let extensions = match extension_index {
                4 => &record.4,
                5 => &record.5,
                _ => unreachable!(),
            };
            assert!(
                ids.len() <= usize::from(u16::MAX),
                "command extension links exceed u16"
            );
            let offset = ids.len() as u16;
            assert!(
                extensions.len() <= usize::from(u8::MAX),
                "command has too many providers"
            );
            let len = extensions.len() as u8;
            ids.extend_from_slice(extensions);
            ranges.push((offset, len));
        }
        let ids_name = format_ident!("COMMAND_{kind}_EXTENSION_IDS");
        let ranges_name = format_ident!("COMMAND_{kind}_EXTENSION_RANGES");
        let ids_len = Literal::usize_unsuffixed(ids.len());
        let ids = ids.into_iter().map(Literal::u16_unsuffixed);
        let ranges = ranges.into_iter().map(|(offset, len)| {
            let offset = Literal::u16_unsuffixed(offset);
            let len = Literal::u8_unsuffixed(len);
            quote! { CommandProviderRange { offset: #offset, len: #len }, }
        });
        extension_metadata.extend(quote! {
            static #ids_name: [u16; #ids_len] = [#(#ids),*];
            static #ranges_name: [CommandProviderRange; COMMAND_COUNT] = [#(#ranges)*];
        });
    }
    let command_count = Literal::usize_unsuffixed(command_records.len());
    let max_displacement = Literal::u16_unsuffixed(max_displacement);
    generated.extend(quote! {
        pub(crate) static COMMAND_NAMES: &[u8] = #command_names_literal;
        pub(crate) static COMMAND_TABLE: [CommandRecord; #table_len_literal] = [#(#command_table)*];
        static COMMAND_DISPLACEMENTS: [u16; #bucket_count] = [#(#displacements),*];
        static COMMAND_CORE_LEVELS: [u16; COMMAND_COUNT] = [#(#core_levels),*];
        static COMMAND_DEVICE_DISPATCH_OFFSETS: [u16; COMMAND_COUNT] = [#(#device_dispatch_offsets)*];
        static COMMAND_LOADER_TRAMPOLINE_WORDS: [u64; #loader_trampoline_word_count] = [#(#loader_trampoline_words),*];
        #extension_metadata
        pub(crate) const COMMAND_COUNT: usize = #command_count;
        #[cfg(test)]
        pub(crate) const COMMAND_MAX_DISPLACEMENT: u16 = #max_displacement;
        #[inline(never)]
        pub(crate) fn command_lookup(name: &CStr) -> Option<CommandLookup> {
            let suffix = name.to_bytes().strip_prefix(b"vk")?;
            let hash = command_hash(suffix);
            let bucket_mask = (COMMAND_DISPLACEMENTS.len() - 1) as u64;
            let bucket = (hash & bucket_mask) as usize;
            let displacement = COMMAND_DISPLACEMENTS[bucket];
            let slot_mask = (COMMAND_TABLE.len() - 1) as u64;
            let slot = (command_slot_hash(hash ^ u64::from(displacement)) & slot_mask) as usize;
            let record = COMMAND_TABLE[slot];
            if record.id == u16::MAX { return None; }
            let start = usize::from(record.name_offset);
            let end = start + usize::from(record.name_len);
            debug_assert!(end <= COMMAND_NAMES.len());
            let stored_suffix = unsafe { COMMAND_NAMES.get_unchecked(start..end) };
            command_name_eq(stored_suffix, suffix).then_some(CommandLookup { id: record.id, scope: record.scope })
        }
        #[inline]
        pub(crate) unsafe fn layer_device_dispatch_proc_addr(table: &LayerDeviceDispatchTable, id: u16) -> PFN_vkVoidFunction {
            let index = usize::from(id);
            debug_assert!(index < COMMAND_DEVICE_DISPATCH_OFFSETS.len());
            let offset = unsafe { *COMMAND_DEVICE_DISPATCH_OFFSETS.get_unchecked(index) };
            if offset == u16::MAX { return None; }
            unsafe { core::ptr::from_ref(table).cast::<u8>().add(usize::from(offset)).cast::<PFN_vkVoidFunction>().read() }
        }
        #[inline]
        pub(crate) fn command_must_use_loader_trampoline(id: u16) -> bool {
            let index = usize::from(id);
            debug_assert!(index < COMMAND_COUNT);
            let word = unsafe { *COMMAND_LOADER_TRAMPOLINE_WORDS.get_unchecked(index / u64::BITS as usize) };
            word & (1_u64 << (index % u64::BITS as usize)) != 0
        }
        #[inline]
        pub(crate) fn command_core_level(id: u16) -> u16 {
            let index = usize::from(id);
            debug_assert!(index < COMMAND_CORE_LEVELS.len());
            unsafe { *COMMAND_CORE_LEVELS.get_unchecked(index) }
        }
        #[inline]
        fn command_extension_enabled(id: u16, ranges: &[CommandProviderRange; COMMAND_COUNT], ids: &[u16], enabled: &ExtensionSet) -> bool {
            let index = usize::from(id);
            debug_assert!(index < ranges.len());
            let range = unsafe { *ranges.get_unchecked(index) };
            let start = usize::from(range.offset);
            let end = start + usize::from(range.len);
            debug_assert!(end <= ids.len());
            unsafe { ids.get_unchecked(start..end) }.iter().copied().any(|extension| enabled.contains(extension))
        }
        #[inline]
        pub(crate) fn command_has_enabled_instance_extension(id: u16, enabled: &ExtensionSet) -> bool {
            command_extension_enabled(id, &COMMAND_INSTANCE_EXTENSION_RANGES, &COMMAND_INSTANCE_EXTENSION_IDS, enabled)
        }
        #[inline]
        pub(crate) fn command_has_enabled_device_extension(id: u16, enabled: &ExtensionSet) -> bool {
            command_extension_enabled(id, &COMMAND_DEVICE_EXTENSION_RANGES, &COMMAND_DEVICE_EXTENSION_IDS, enabled)
        }
        #[inline]
        pub(crate) fn command_has_device_extension_provider(id: u16) -> bool {
            let index = usize::from(id);
            debug_assert!(index < COMMAND_DEVICE_EXTENSION_RANGES.len());
            unsafe { COMMAND_DEVICE_EXTENSION_RANGES.get_unchecked(index) }.len != 0
        }
    });

    let mut promoted_wrappers = Vec::new();
    for &core_name in PROMOTED_TERMINATOR_IMPLEMENTATIONS {
        let implementation_name = promoted_implementation_name(core_name);
        let mut commands = registry
            .commands
            .values()
            .flatten()
            .filter(|command| {
                (command.name == core_name || command.alias.as_deref() == Some(core_name))
                    && (command.api.vulkan || command.api.vulkanbase)
                    && command_has_vulkan_provider(&registry, command)
            })
            .collect::<Vec<_>>();
        commands.sort_unstable_by(|left, right| left.name.cmp(&right.name));
        commands.dedup_by(|left, right| left.name == right.name);
        assert!(!commands.is_empty(), "missing promoted command {core_name}");

        for command in commands {
            let is_alias = command.name != core_name;
            let signature = resolved_command_signature(command, &registry);
            let name = format_ident!("terminator_{}", command.name);
            let implementation = format_ident!("{implementation_name}");
            let cfg = platform_cfg(command_platform_protect(&registry, command));
            let params = signature
                .params
                .iter()
                .map(|parameter| {
                    let name = match parameter.name.as_str() {
                        "type" => format_ident!("type_"),
                        "match" => format_ident!("match_"),
                        name => format_ident!("{name}"),
                    };
                    let ty = command_param_abi_type_for_registry(parameter, &registry).to_string();
                    let ty = qualify_registry_type(&ty, &parameter.ty.base);
                    let ty = syn::parse_str::<syn::Type>(&ty)
                        .expect("promoted command parameter type must be valid Rust syntax");
                    quote! { #name: #ty }
                })
                .collect::<Vec<_>>();
            let args = signature
                .params
                .iter()
                .map(|parameter| {
                    let name = match parameter.name.as_str() {
                        "type" => format_ident!("type_"),
                        "match" => format_ident!("match_"),
                        name => format_ident!("{name}"),
                    };
                    match (
                        is_alias,
                        parameter.ty.pointer_depth,
                        parameter.ty.base.starts_with("Vk"),
                    ) {
                        (true, 1.., true) => quote! { #name.cast() },
                        _ => quote! { #name },
                    }
                })
                .collect::<Vec<_>>();
            let returns_void =
                signature.return_type.base == "void" || signature.return_type.base.is_empty();
            let (return_clause, body) = match returns_void {
                true => (
                    TokenStream::new(),
                    quote! { unsafe { promoted::#implementation(#(#args),*); } },
                ),
                false => {
                    let return_type = qualify_registry_type(
                        &ctype_to_rust_str(&signature.return_type),
                        &signature.return_type.base,
                    );
                    let return_type = syn::parse_str::<syn::Type>(&return_type)
                        .expect("promoted command return type must be valid Rust syntax");
                    (
                        quote! { -> #return_type },
                        quote! { unsafe { promoted::#implementation(#(#args),*) } },
                    )
                }
            };
            promoted_wrappers.push(quote! {
                #cfg
                pub(crate) unsafe extern "system" fn #name(#(#params),*) #return_clause {
                    #body
                }
            });
        }
    }
    generated.extend(quote! { #(#promoted_wrappers)* });

    let is_exported = |command: &&vk_codegen::ir::Command| {
        (command.export.is_empty() || command.export.contains(&ExportScope::Vulkan))
            && (command.api.vulkan || command.api.vulkanbase)
            && command_has_vulkan_provider(&registry, command)
    };
    let mut exported_commands = commands
        .iter()
        .copied()
        .filter(is_exported)
        .filter(|command| command_scope(&registry, command) != Scope::Global)
        .collect::<Vec<_>>();
    exported_commands.sort_unstable_by(|left, right| left.name.cmp(&right.name));
    exported_commands.dedup_by(|left, right| left.name == right.name);
    let exported_ids = exported_commands
        .iter()
        .map(|command| {
            let id = command_records
                .iter()
                .position(|record| record.0 == command.name)
                .expect("exported command metadata");
            (
                id,
                command.name.as_str(),
                command_platform_protect(&registry, command),
            )
        })
        .collect::<Vec<_>>();
    let mut instance_terminators = Vec::new();
    let mut physical_device_terminators = Vec::new();
    for command in exported_commands {
        if HANDWRITTEN_TERMINATORS.contains(&command.name.as_str()) {
            continue;
        }
        let signature = resolved_command_signature(command, &registry);
        let name_text = command.name.as_str();
        let name = format_ident!("{name_text}");
        let params = signature
            .params
            .iter()
            .map(|parameter| {
                let parameter_name = match parameter.name.as_str() {
                    "type" => "type_",
                    "match" => "match_",
                    name => name,
                };
                let parameter_type =
                    command_param_abi_type_for_registry(parameter, &registry).to_string();
                let parameter_type = qualify_registry_type(&parameter_type, &parameter.ty.base);
                let parameter_type = syn::parse_str::<syn::Type>(&parameter_type)
                    .expect("command parameter type must be valid Rust syntax");
                let parameter_name = format_ident!("{parameter_name}");
                quote! { #parameter_name: #parameter_type }
            })
            .collect::<Vec<_>>();
        let args = signature
            .params
            .iter()
            .map(|parameter| {
                format_ident!(
                    "{}",
                    match parameter.name.as_str() {
                        "type" => "type_",
                        "match" => "match_",
                        name => name,
                    }
                )
            })
            .collect::<Vec<_>>();
        let first = args
            .first()
            .expect("dispatched command has first parameter");
        let first_type = signature
            .params
            .first()
            .expect("dispatched command has first parameter")
            .ty
            .base
            .as_str();
        let return_type = qualify_registry_type(
            &ctype_to_rust_str(&signature.return_type),
            &signature.return_type.base,
        );
        let return_type = syn::parse_str::<syn::Type>(&return_type)
            .expect("command return type must be valid Rust syntax");
        let returns_void =
            signature.return_type.base == "void" || signature.return_type.base.is_empty();
        let creates_surface = signature.params.len() == 4
            && first_type == "VkInstance"
            && signature
                .params
                .last()
                .is_some_and(|parameter| parameter.ty.base == "VkSurfaceKHR")
            && name_text.starts_with("vkCreate");
        let cfg = platform_cfg(command_platform_protect(&registry, command));
        let return_clause = (!returns_void).then(|| quote! { -> #return_type });
        let core_export = command_records
            .iter()
            .find(|record| record.0 == command.name)
            .is_some_and(|record| record.3 != 0);
        let wsi_export = registry.commands[&command.name].iter().any(|variant| {
            variant
                .provided_by
                .iter()
                .any(|provider| PUBLIC_WSI_EXTENSIONS.contains(&provider.as_str()))
        });
        let export = (core_export || wsi_export).then(|| quote! { #[unsafe(no_mangle)] });
        let mut body = TokenStream::new();
        if first_type == "VkInstance" && !creates_surface {
            let error = c_string_literal(&format!(
                "{name_text}: Invalid {first} [VUID-{name_text}-{first}-parameter]"
            ));
            body.extend(quote! {
                if unsafe { LoaderInstance::from_handle(#first) }.is_none() {
                    fatal_loader_error(#error)
                };
            });
        }
        if creates_surface {
            let extension = command
                .provided_by
                .iter()
                .find(|provider| {
                    registry
                        .extensions
                        .iter()
                        .any(|extension| extension.name == provider.as_str())
                })
                .expect("surface creation command must have an extension provider");
            let extension_id_constant =
                format_ident!("{}_EXTENSION_ID", extension.to_ascii_uppercase());
            let create_info_type = &signature.params[1].ty.base;
            let root_type =
                rust_structure_type_constant(structure_type_constant(&registry, create_info_type));
            let root_type = format_ident!("{root_type}");
            let terminator_name = format_ident!("terminator_{name_text}");
            let command_literal = c_string_literal(name_text);
            let instance = &args[0];
            let create_info = &args[1];
            let allocator = &args[2];
            let surface = &args[3];
            let error = c_string_literal(&format!(
                "{name_text}: Invalid {instance} [VUID-{name_text}-{instance}-parameter]"
            ));
            body.extend(quote! {
                let loader = unsafe { LoaderInstance::from_handle(#instance) }
                    .unwrap_or_else(|| fatal_loader_error(#error));
                let dispatch = unsafe { &*loader.dispatch() };
                let command = dispatch.#name;
                debug_assert!(command.is_some());
                let command = unsafe { command.unwrap_unchecked() };
                unsafe { command(loader.chain_handle(), #create_info, #allocator, #surface) }
            });
            generated.extend(quote! {
                #cfg
                /// Forwards a Vulkan command to the dispatch chain.
                ///
                /// # Safety
                ///
                /// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
                #export
                pub(crate) unsafe extern "system" fn #name(#(#params),*) #return_clause { #body }
                #cfg
                /// Creates loader-owned WSI state at the bottom of an instance layer chain.
                ///
                /// # Safety
                ///
                /// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
                pub(crate) unsafe extern "system" fn #terminator_name(#(#params),*) -> vk::VkResult {
                    unsafe { create_loader_surface(#instance, #create_info, VkStructureType::#root_type, #allocator, #surface, #command_literal, #extension_id_constant) }
                }
            });
            let id = command_records
                .iter()
                .position(|record| record.0 == command.name)
                .expect("surface terminator command metadata");
            instance_terminators.push((
                id,
                command.name.clone(),
                command_platform_protect(&registry, command),
            ));
            continue;
        }
        if first_type == "VkPhysicalDevice" {
            let missing = if signature.return_type.base == "VkResult" {
                quote! { vk::VkResult::ERROR_INITIALIZATION_FAILED }
            } else {
                quote! { unsafe { core::mem::zeroed::<#return_type>() } }
            };
            let error = c_string_literal(&format!(
                "{name_text}: Invalid {first} [VUID-{name_text}-{first}-parameter]"
            ));
            body.extend(quote! {
                let Some((dispatch, #first)) = (unsafe { resolve_trampoline_physical_device(#first) }) else {
                    fatal_loader_error(#error)
                };
                let command = dispatch.#name.map(|command| (command, #first));
            });
            if returns_void {
                body.extend(quote! {
                    if let Some((command, #first)) = command { unsafe { command(#(#args),*); } }
                });
            } else {
                body.extend(quote! {
                    command.map_or_else(|| #missing, |(command, #first)| unsafe { command(#(#args),*) })
                });
            }
            generated.extend(quote! {
                #cfg
                /// Forwards a Vulkan command to the dispatch chain.
                ///
                /// # Safety
                ///
                /// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
                #export
                pub(crate) unsafe extern "system" fn #name(#(#params),*) #return_clause { #body }
            });
            if HANDWRITTEN_PHYSICAL_DEVICE_TERMINATORS.contains(&name_text) {
                let id = command_records
                    .iter()
                    .position(|record| record.0 == command.name)
                    .expect("handwritten physical-device terminator metadata");
                physical_device_terminators.push((
                    id,
                    command.name.clone(),
                    command_platform_protect(&registry, command),
                ));
                continue;
            }
            let id = command_records
                .iter()
                .position(|record| record.0 == command.name)
                .expect("physical-device terminator metadata");
            physical_device_terminators.push((
                id,
                command.name.clone(),
                command_platform_protect(&registry, command),
            ));
            body = TokenStream::new();
        }
        let direct_device_dispatch = command_scope(&registry, command) == Scope::Device;
        if direct_device_dispatch {
            body.extend(quote! {
                let dispatch = unsafe { device_dispatch(#first.0.cast()) }
                    .unwrap_or_else(|| invalid_device_dispatch());
                let command = dispatch.#name;
            });
            let nullable = signature
                .provided_by
                .iter()
                .any(|provider| provider == "VK_EXT_debug_utils");
            if nullable {
                if returns_void {
                    body.extend(quote! {
                        if let Some(command) = command { unsafe { command(#(#args),*); } }
                    });
                } else {
                    body.extend(quote! {
                        command.map_or(vk::VkResult::SUCCESS, |command| unsafe { command(#(#args),*) })
                    });
                }
                generated.extend(quote! {
                    #cfg
                    /// Forwards a Vulkan command to the dispatch chain.
                    ///
                    /// # Safety
                    ///
                    /// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
                    #export
                    pub(crate) unsafe extern "system" fn #name(#(#params),*) #return_clause { #body }
                });
                continue;
            }
            body.extend(quote! {
                debug_assert!(command.is_some());
                let command = unsafe { command.unwrap_unchecked() };
            });
            match name_text {
                "vkGetDeviceQueue" | "vkGetDeviceQueue2" => {
                    let queue_output = format_ident!("pQueue");
                    body.extend(quote! {
                        unsafe { command(#(#args),*); }
                        if !#queue_output.is_null() {
                            let queue = unsafe { #queue_output.read() };
                            if queue != vk::VkQueue::NULL {
                                unsafe { set_device_dispatchable(queue.0.cast(), core::ptr::from_ref(dispatch)); }
                            }
                        }
                    });
                }
                "vkAllocateCommandBuffers" => {
                    let allocate_info = format_ident!("pAllocateInfo");
                    let command_buffers = format_ident!("pCommandBuffers");
                    body.extend(quote! {
                        let result = unsafe { command(#(#args),*) };
                        if result == vk::VkResult::SUCCESS {
                            let count = unsafe { (*#allocate_info).commandBufferCount } as usize;
                            for index in 0..count {
                                let command_buffer = unsafe { #command_buffers.add(index).read() };
                                if command_buffer != vk::VkCommandBuffer::NULL {
                                    unsafe { set_device_dispatchable(command_buffer.0.cast(), core::ptr::from_ref(dispatch)); }
                                }
                            }
                        }
                        result
                    });
                }
                _ if returns_void => {
                    body.extend(quote! { unsafe { command(#(#args),*); } });
                }
                _ => body.extend(quote! { unsafe { command(#(#args),*) } }),
            }
            generated.extend(quote! {
                #cfg
                /// Forwards a Vulkan command to the dispatch chain.
                ///
                /// # Safety
                ///
                /// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
                #export
                pub(crate) unsafe extern "system" fn #name(#(#params),*) #return_clause { #body }
            });
            continue;
        }
        if first_type == "VkPhysicalDevice" {
            for parameter in signature
                .params
                .iter()
                .filter(|parameter| parameter.ty.base == "VkSurfaceKHR")
            {
                assert_eq!(
                    signature.return_type.base, "VkResult",
                    "surface translation requires a VkResult command"
                );
                let parameter = format_ident!("{}", parameter.name);
                body.extend(quote! {
                    let #parameter = match unsafe { translate_physical_device_surface(#first, #parameter) } {
                        Ok(surface) => surface,
                        Err(result) => return result,
                    };
                });
            }
        }
        let command_pfn = format_ident!("PFN_{name_text}");
        let command_literal = c_string_literal(name_text);
        let (command_type, resolver) = match first_type {
            "VkPhysicalDevice" => (
                quote! { Option<(vk::#command_pfn, vk::VkPhysicalDevice)> },
                quote! { resolve_physical_device(#first, |dispatch| dispatch.#name, #command_literal) },
            ),
            "VkDevice" => unreachable!("device commands use direct dispatch above"),
            _ => (
                quote! { Option<vk::#command_pfn> },
                quote! { instance_dispatch(#first.0.cast()).and_then(|dispatch| dispatch.#name) },
            ),
        };
        body.extend(quote! { let command: #command_type = unsafe { #resolver }; });
        let closure_pattern = match first_type {
            "VkPhysicalDevice" | "VkDevice" => quote! { (command, #first) },
            _ => quote! { command },
        };
        let missing_physical_device_extension = first_type == "VkPhysicalDevice"
            && command_records
                .iter()
                .find(|record| record.0 == command.name)
                .is_some_and(|record| !record.5.is_empty());
        if returns_void {
            if missing_physical_device_extension {
                let error =
                    c_string_literal(&format!("{name_text}: Driver's function pointer was NULL"));
                body.extend(quote! {
                    let Some(#closure_pattern) = command else { fatal_loader_error(#error) };
                    unsafe { command(#(#args),*); }
                });
            } else {
                body.extend(quote! {
                    if let Some(#closure_pattern) = command { unsafe { command(#(#args),*); } }
                });
            }
        } else {
            let missing = if matches!(name_text, "vkAcquireDrmDisplayEXT" | "vkGetDrmDisplayEXT") {
                quote! { vk::VkResult::ERROR_EXTENSION_NOT_PRESENT }
            } else if matches!(
                name_text,
                "vkGetPhysicalDeviceDisplayPropertiesKHR"
                    | "vkGetPhysicalDeviceDisplayPlanePropertiesKHR"
                    | "vkGetDisplayModePropertiesKHR"
            ) {
                let count = format_ident!("pPropertyCount");
                quote! {{ if !#count.is_null() { unsafe { #count.write(0); } } vk::VkResult::SUCCESS }}
            } else if name_text == "vkGetDisplayPlaneSupportedDisplaysKHR" {
                let count = format_ident!("pDisplayCount");
                quote! {{ if !#count.is_null() { unsafe { #count.write(0); } } vk::VkResult::SUCCESS }}
            } else if name_text == "vkGetDisplayPlaneCapabilitiesKHR" {
                let capabilities = format_ident!("pCapabilities");
                quote! {{ if !#capabilities.is_null() { unsafe { #capabilities.write(vk::VkDisplayPlaneCapabilitiesKHR::DEFAULT); } } vk::VkResult::SUCCESS }}
            } else if missing_physical_device_extension {
                let error =
                    c_string_literal(&format!("{name_text}: Driver's function pointer was NULL"));
                quote! { fatal_loader_error(#error) }
            } else if signature.return_type.base == "VkResult" {
                quote! { vk::VkResult::ERROR_INITIALIZATION_FAILED }
            } else {
                quote! { unsafe { core::mem::zeroed::<#return_type>() } }
            };
            body.extend(quote! {
                command.map_or_else(|| #missing, |#closure_pattern| unsafe { command(#(#args),*) })
            });
        }
        if first_type == "VkPhysicalDevice" {
            let terminator_name = format_ident!("terminator_{name_text}");
            generated.extend(quote! {
                #cfg
                /// Forwards a loader terminator command to the owning ICD.
                ///
                /// # Safety
                ///
                /// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
                pub(crate) unsafe extern "system" fn #terminator_name(#(#params),*) #return_clause { #body }
            });
        } else {
            generated.extend(quote! {
                #cfg
                /// Forwards a Vulkan command to the dispatch chain.
                ///
                /// # Safety
                ///
                /// The caller must satisfy the pointer, handle, and lifetime requirements of the Vulkan API.
                #export
                pub(crate) unsafe extern "system" fn #name(#(#params),*) #return_clause { #body }
            });
        }
    }
    let exported_arms = exported_ids.iter().map(|&(id, name, protect)| {
        let cfg = platform_cfg(protect);
        let id = Literal::usize_unsuffixed(id);
        let name = format_ident!("{name}");
        quote! { #cfg #id => Some(erase_function(#name as *const ())), }
    });
    let mut instance_terminator_arms = instance_terminators
        .iter()
        .map(|(id, name, protect)| {
            let cfg = platform_cfg(*protect);
            let id = Literal::usize_unsuffixed(*id);
            let terminator = format_ident!("terminator_{name}");
            quote! { #cfg #id => Some(erase_function(#terminator as *const ())), }
        })
        .collect::<Vec<_>>();
    for name in HANDWRITTEN_INSTANCE_TERMINATORS {
        let id = command_records
            .iter()
            .position(|record| record.0 == *name)
            .unwrap_or_else(|| panic!("handwritten instance terminator metadata for {name}"));
        let id = Literal::usize_unsuffixed(id);
        let terminator = format_ident!("terminator_{name}");
        instance_terminator_arms
            .push(quote! { #id => Some(erase_function(#terminator as *const ())), });
    }
    let physical_device_terminator_arms =
        physical_device_terminators
            .iter()
            .map(|(id, name, protect)| {
                let cfg = platform_cfg(*protect);
                let id = Literal::usize_unsuffixed(*id);
                let terminator = format_ident!("terminator_{name}");
                quote! { #cfg #id => Some(erase_function(#terminator as *const ())), }
            });
    let icd_device_terminator_arms = icd_terminator_commands.iter().map(|command| {
        let id = command_records
            .iter()
            .position(|record| record.0 == command.name)
            .expect("ICD terminator command metadata");
        let cfg = platform_cfg(command_platform_protect(&registry, command));
        let id = Literal::usize_unsuffixed(id);
        let name = format_ident!("{}", command.name);
        quote! { #cfg #id => table.#name.map(erase_function), }
    });
    generated.extend(quote! {
        #[inline(never)]
        #[allow(clippy::too_many_lines)] // Exhaustive generated command-ID match.
        pub(crate) fn exported_proc_addr(id: u16) -> PFN_vkVoidFunction {
            match id { #(#exported_arms)* _ => None }
        }
        #[inline(never)]
        pub(crate) fn instance_terminator_proc_addr(id: u16) -> PFN_vkVoidFunction {
            match id { #(#instance_terminator_arms)* _ => None }
        }
        #[inline(never)]
        #[allow(clippy::too_many_lines)] // Exhaustive generated command-ID match.
        pub(crate) fn physical_device_terminator_proc_addr(id: u16) -> PFN_vkVoidFunction {
            match id { #(#physical_device_terminator_arms)* _ => None }
        }
        #[inline(never)]
        pub(crate) fn icd_device_terminator_proc_addr(table: &IcdDeviceTerminatorDispatchTable, id: u16) -> PFN_vkVoidFunction {
            match id { #(#icd_device_terminator_arms)* _ => None }
        }
    });

    let syntax = syn::parse2::<syn::File>(generated).expect("generated loader source must parse");
    let mut parts = GENERATED_LOADER_PARTS
        .iter()
        .map(|name| (*name, Vec::new()))
        .collect::<BTreeMap<_, _>>();
    for item in syntax.items {
        parts
            .get_mut(generated_loader_part(&item))
            .expect("known generated loader part")
            .push(item);
    }
    for items in parts.values_mut() {
        items.iter_mut().for_each(expose_to_generated_siblings);
    }
    let command_import_cfg = registry
        .commands
        .values()
        .flatten()
        .map(|command| {
            (
                format!("{}_COMMAND_ID", screaming_snake_case(&command.name)),
                platform_cfg(command_platform_protect(&registry, command)),
            )
        })
        .collect::<HashMap<_, _>>();
    let mut generated_items = BTreeMap::new();
    for (&part, items) in &parts {
        for item in items {
            if let Some(identifier) = generated_item_identifier(item) {
                let attributes = generated_item_attributes(item)
                    .iter()
                    .filter(|attribute| attribute.path().is_ident("cfg"));
                let cfg = command_import_cfg
                    .get(&identifier.to_string())
                    .cloned()
                    .unwrap_or_else(|| quote! { #(#attributes)* });
                generated_items.insert(identifier.to_string(), (part, cfg));
            }
        }
    }
    let output_directory = output_path.parent().expect("loader output directory");
    for &name in GENERATED_LOADER_PARTS {
        let mut items = parts.get(name).expect("generated loader part").clone();
        let mut used = UsedIdentifiers::default();
        for item in &items {
            syn::visit::Visit::visit_item(&mut used, item);
        }
        let mut used = used.0;
        if name == "commands" {
            used.insert("LayerDeviceDispatchTable".to_owned());
        }
        let local = items
            .iter()
            .filter_map(generated_item_identifier)
            .map(ToString::to_string)
            .collect::<BTreeSet<_>>();
        let mut imports = Vec::new();
        for parent in GENERATED_PARENT_NAMES {
            if used.contains(*parent) && !local.contains(*parent) {
                let identifier = format_ident!("{parent}");
                imports.push(syn::parse_quote! { use crate::#identifier; });
            }
        }
        for (identifier, (part, cfg)) in &generated_items {
            if *part == name || !used.contains(identifier) || local.contains(identifier) {
                continue;
            }
            let part = format_ident!("{part}");
            let identifier = format_ident!("{identifier}");
            let import = quote! {
                #cfg
                use super::#part::#identifier;
            };
            imports.push(
                syn::parse2(import.clone()).unwrap_or_else(|error| {
                    panic!("generated import `{import}` must parse: {error}")
                }),
            );
        }
        imports.extend(items);
        items = imports;
        let syntax = syn::File {
            shebang: None,
            attrs: Vec::new(),
            items,
        };
        fs::write(
            output_directory.join(format!("{name}.rs")),
            generated_source(&syntax),
        )
        .unwrap_or_else(|error| panic!("write generated loader part {name}: {error}"));
    }
    let modules = GENERATED_LOADER_PARTS
        .iter()
        .map(|name| format_ident!("{name}"))
        .collect::<Vec<_>>();
    let entry = syn::parse2::<syn::File>(quote! {
        #(mod #modules;)*

        pub(crate) use commands::{
            command_core_level, command_has_device_extension_provider,
            command_has_enabled_device_extension, command_has_enabled_instance_extension,
            command_lookup, command_must_use_loader_trampoline,
        };
        pub(crate) use debug::{
            convert_core_object_to_debug_report_object,
            convert_debug_report_object_to_core_object,
        };
        pub(crate) use dispatch_tables::{
            IcdDeviceTerminatorDispatchTable, InstanceDispatchTable,
            LayerDeviceDispatchTable, LayerInstanceDispatchTable,
        };
        pub(crate) use extensions::{
            ExtensionSet, VK_EXT_SURFACE_MAINTENANCE1_EXTENSION_ID,
            VK_KHR_SURFACE_MAINTENANCE1_EXTENSION_ID, extension_id,
            is_known_instance_extension, surface_create_info_extension_size,
            wsi_instance_extension_supported,
        };
        pub(crate) use proc_addr::{
            exported_proc_addr, global_proc_addr, icd_device_terminator_proc_addr,
            instance_terminator_proc_addr, layer_device_dispatch_proc_addr,
            physical_device_terminator_proc_addr,
        };
        #[cfg(test)]
        pub(crate) use commands::{
            COMMAND_COUNT, COMMAND_MAX_DISPLACEMENT, COMMAND_NAMES, COMMAND_TABLE,
        };
        #[cfg(test)]
        pub(crate) use handles::handle_info;
    })
    .expect("generated loader module must parse");
    fs::write(output_path, generated_source(&entry)).expect("write generated loader module");
}

fn update_loader_features(cargo_path: &PathBuf, registry: &vk_codegen::ir::Registry) {
    const START: &str = "  # BEGIN GENERATED VULKAN FEATURES";
    const END: &str = "  # END GENERATED VULKAN FEATURES";
    let cargo = fs::read_to_string(cargo_path).expect("read vk-loader Cargo.toml");
    let start = cargo.find(START).expect("generated feature start marker");
    let end = cargo.find(END).expect("generated feature end marker");
    let mut features = registry
        .extensions
        .iter()
        .filter(|extension| extension.api.vulkan && !extension.is_disabled())
        .map(|extension| extension.name.as_str())
        .collect::<Vec<_>>();
    features.push("VK_VERSION_1_4");
    features.sort_unstable();
    features.dedup();
    let generated = features
        .iter()
        .map(|feature| format!("  \"{feature}\","))
        .collect::<Vec<_>>()
        .join("\n");
    let mut output = String::with_capacity(cargo.len() + generated.len());
    output.push_str(&cargo[..start + START.len()]);
    output.push('\n');
    output.push_str(&generated);
    output.push('\n');
    output.push_str(&cargo[end..]);
    fs::write(cargo_path, output).expect("write vk-loader Cargo.toml");
}

#[cfg(test)]
mod tests {
    use super::rust_platform_cfg;

    #[test]
    fn qnx_platform_cfg_supports_legacy_and_modern_rust_targets() {
        assert_eq!(
            rust_platform_cfg("VK_USE_PLATFORM_SCREEN_QNX"),
            "any(target_os = \"nto\", target_os = \"qnx\")"
        );
    }

    #[test]
    fn desktop_unix_wsi_cfg_matches_upstream_cmake_platforms() {
        for protect in [
            "VK_USE_PLATFORM_XLIB_KHR",
            "VK_USE_PLATFORM_XLIB_XRANDR_EXT",
            "VK_USE_PLATFORM_XCB_KHR",
            "VK_USE_PLATFORM_WAYLAND_KHR",
        ] {
            let cfg = rust_platform_cfg(protect);
            for target in [
                "linux",
                "freebsd",
                "openbsd",
                "netbsd",
                "dragonfly",
                "hurd",
                "cygwin",
            ] {
                assert!(
                    cfg.contains(&format!("target_os = \"{target}\"")),
                    "{protect} omitted upstream platform {target}"
                );
            }
        }
    }
}
