//! entry implementation.

use crate::{
    CStr, CommandScope, LoaderInstance, PFN_vkVoidFunction, VK_API_VERSION_1_3,
    VkExtensionProperties, VkInstance, VkLayerProperties, VkResult, c_char, command_core_level,
    command_has_device_extension_provider, command_has_enabled_instance_extension, command_lookup,
    discovery, erase_function, exported_proc_addr, fatal_loader_error, global_proc_addr, pending,
    platform, pre_instance, unknown,
};

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
    if let Err(error) = platform::initialize_loader() {
        return error;
    }
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    pending::with_json_error_scope(|| {
        platform::with_elevated_privileges_snapshot(|| {
            discovery::with_loader_settings_absence_cache(|| {
                // SAFETY: The entry-point contract supplies live output storage.
                unsafe {
                    pre_instance::enumerate_extension_properties(
                        layer_name,
                        property_count,
                        properties,
                    )
                }
            })
        })
    })
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
    if let Err(error) = platform::initialize_loader() {
        return error;
    }
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    pending::with_json_error_scope(|| unsafe {
        pre_instance::enumerate_layer_properties(property_count, properties)
    })
}

/// Reports the highest Vulkan core version supported by this loader.
///
/// # Safety
///
/// `api_version` must point to writable `u32` storage.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumerateInstanceVersion(api_version: *mut u32) -> VkResult {
    if let Err(error) = platform::initialize_loader() {
        return error;
    }
    if api_version.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    pending::with_json_error_scope(|| unsafe { pre_instance::enumerate_version(&mut *api_version) })
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
        command_lookup(name.to_bytes()).and_then(|lookup| global_proc_addr(lookup.id))
    } else {
        // SAFETY: A non-null instance supplied to GIPA must be a live loader instance.
        let loader = unsafe { LoaderInstance::from_handle(instance) }.unwrap_or_else(|| {
            fatal_loader_error(
                c"vkGetInstanceProcAddr: Invalid instance [VUID-vkGetInstanceProcAddr-instance-parameter]",
            )
        });
        let Some(lookup) = command_lookup(name.to_bytes()) else {
            return unknown::physical_device_proc_addr(loader, name, true)
                .or_else(|| unknown::device_proc_addr(loader, name, true));
        };
        if lookup.scope == CommandScope::Global {
            if lookup.id == crate::generated::GET_INSTANCE_PROC_ADDR_COMMAND_ID {
                Some(erase_function(
                    vkGetInstanceProcAddr as vk::PFN_vkGetInstanceProcAddr,
                ))
            } else {
                (loader.api_version < VK_API_VERSION_1_3)
                    .then(|| global_proc_addr(lookup.id))
                    .flatten()
            }
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
