//! Vulkan loader implementation.
#![allow(non_snake_case)]

#[cfg(all(feature = "apple-static-loader", not(target_vendor = "apple")))]
compile_error!("the `apple-static-loader` feature is only supported on Apple platforms");

extern crate alloc;

mod allocation;
mod collections;
mod debug;
mod device;
mod device_api;
mod discovery;
mod dispatch;
mod display;
mod emulation;
mod entry;
#[path = "generated/mod.rs"]
mod generated;
mod icd;
mod instance;
mod instance_api;
#[path = "../json/mod.rs"]
mod json;
mod layer;
mod pending;
mod physical_device;
mod platform;
mod pre_instance;
mod promoted;
mod surface;
mod sync;
#[cfg(test)]
mod tests;
mod unknown;

use core::{
    cmp::Ordering,
    ffi::{CStr, c_char, c_void},
    mem::MaybeUninit,
};
use debug::messenger::{
    vkCreateDebugReportCallbackEXT, vkCreateDebugUtilsMessengerEXT, vkDebugReportMessageEXT,
    vkDestroyDebugReportCallbackEXT, vkDestroyDebugUtilsMessengerEXT, vkSubmitDebugUtilsMessageEXT,
};
use debug::{
    vkDebugMarkerSetObjectNameEXT, vkDebugMarkerSetObjectTagEXT, vkSetDebugUtilsObjectNameEXT,
    vkSetDebugUtilsObjectTagEXT,
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
    exported_proc_addr, extension_id, extension_name, global_proc_addr,
    icd_device_terminator_proc_addr, instance_terminator_proc_addr, is_known_instance_extension,
    layer_device_dispatch_proc_addr, physical_device_terminator_proc_addr,
    surface_create_info_extension_size, wsi_instance_extension_supported,
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

pub(crate) use device_api::{
    create_device_terminator, destroy_device_terminator, terminator_enumerate_physical_devices,
};
pub use device_api::{vkCreateDevice, vkDestroyDevice, vkGetDeviceProcAddr};
pub(crate) use dispatch::{
    CommandLookup, CommandProviderRange, CommandRecord, CommandScope, DEVICE_DISPATCH_MAGIC,
    command_hash, command_name_eq, command_slot_hash, device_dispatch, dispatch_offset,
    erase_function, fatal_loader_error, invalid_device_dispatch, load_typed,
    resolve_physical_device, resolve_trampoline_physical_device, set_device_dispatchable,
    translate_device_group_chain,
};
pub use entry::{
    vk_string_validate, vkEnumerateInstanceExtensionProperties, vkEnumerateInstanceLayerProperties,
    vkEnumerateInstanceVersion, vkGetInstanceProcAddr,
};
pub(crate) use instance_api::{
    ScannedIcdRecord, create_pending_icd_instances, destroy_instance_terminator,
};
#[cfg(test)]
pub(crate) use instance_api::{fatal_direct_driver_scan_error, icd_create_application_api_version};
pub use instance_api::{vkCreateInstance, vkDestroyInstance};
#[cfg(test)]
pub(crate) use physical_device::parse_linux_device_selection;
pub(crate) use physical_device::{
    LINUX_SORT_PLATFORM_ENABLED, decimal_prefix_nonzero,
    discover_active_physical_devices_with_diagnostics, emit_instance_category_message,
    emit_instance_loader_message, linux_sort_requires_properties_extension,
    terminator_enumerate_physical_device_groups, terminator_enumerate_physical_device_groups_khr,
};
pub use physical_device::{
    vkEnumerateDeviceExtensionProperties, vkEnumerateDeviceLayerProperties,
    vkEnumeratePhysicalDeviceGroups, vkEnumeratePhysicalDeviceGroupsKHR,
    vkEnumeratePhysicalDevices,
};
