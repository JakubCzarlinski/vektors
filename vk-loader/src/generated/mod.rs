// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.

mod commands;
mod debug;
mod dispatch_tables;
mod extensions;
mod proc_addr;
mod promotions;
mod terminators;
mod trampolines;
#[cfg(test)]
pub(crate) use commands::{COMMAND_COUNT, COMMAND_MAX_DISPLACEMENT, COMMAND_NAMES, COMMAND_TABLE};
pub(crate) use commands::{
    GET_INSTANCE_PROC_ADDR_COMMAND_ID, command_core_level, command_has_device_extension_provider,
    command_has_enabled_device_extension, command_has_enabled_instance_extension, command_lookup,
    command_must_use_loader_trampoline,
};
pub(crate) use debug::{
    convert_core_object_to_debug_report_object, convert_debug_report_object_to_core_object,
};
pub(crate) use dispatch_tables::{
    IcdDeviceTerminatorDispatchTable, InstanceDispatchTable, LayerDeviceDispatchTable,
    LayerInstanceDispatchTable,
};
pub(crate) use extensions::{
    ExtensionSet, VK_EXT_DEBUG_REPORT_EXTENSION_ID, VK_EXT_DEBUG_UTILS_EXTENSION_ID,
    VK_EXT_SURFACE_MAINTENANCE1_EXTENSION_ID, VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_ID,
    VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION_ID,
    VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_ID, VK_KHR_SURFACE_MAINTENANCE1_EXTENSION_ID,
    VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_ID, extension_id, extension_id_bytes, extension_name,
    is_instance_extension, is_known_instance_extension, surface_create_info_extension_size,
    wsi_instance_extension_supported,
};
pub(crate) use proc_addr::{
    exported_proc_addr, global_proc_addr, icd_device_terminator_proc_addr,
    instance_terminator_proc_addr, layer_device_dispatch_proc_addr,
    physical_device_terminator_proc_addr,
};
pub(crate) use promotions::{
    EmulatedCommand, PromotedDispatch, dispatch_promoted_external_buffer_properties,
    dispatch_promoted_external_fence_properties, dispatch_promoted_external_semaphore_properties,
    dispatch_promoted_features2, dispatch_promoted_format_properties2,
    dispatch_promoted_image_format_properties2, dispatch_promoted_memory_properties2,
    dispatch_promoted_properties2, dispatch_promoted_queue_family_properties2,
    dispatch_promoted_sparse_image_format_properties2,
};
