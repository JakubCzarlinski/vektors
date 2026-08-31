// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.

mod commands;
mod debug;
mod dispatch_tables;
mod extensions;
mod handles;
mod proc_addr;
mod terminators;
mod trampolines;
#[cfg(test)]
pub(crate) use commands::{COMMAND_COUNT, COMMAND_MAX_DISPLACEMENT, COMMAND_NAMES, COMMAND_TABLE};
pub(crate) use commands::{
    command_core_level, command_has_device_extension_provider,
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
    ExtensionSet, VK_EXT_SURFACE_MAINTENANCE1_EXTENSION_ID,
    VK_KHR_SURFACE_MAINTENANCE1_EXTENSION_ID, extension_id, is_known_instance_extension,
    surface_create_info_extension_size, wsi_instance_extension_supported,
};
#[cfg(test)]
pub(crate) use handles::handle_info;
pub(crate) use proc_addr::{
    exported_proc_addr, global_proc_addr, icd_device_terminator_proc_addr,
    instance_terminator_proc_addr, layer_device_dispatch_proc_addr,
    physical_device_terminator_proc_addr,
};
