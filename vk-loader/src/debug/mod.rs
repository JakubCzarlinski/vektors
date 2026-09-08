//! Debug callbacks, object annotations, and diagnostic formatting.

pub(crate) mod diagnostics;
pub(crate) mod messenger;
mod objects;

pub(crate) use objects::{
    terminator_vkDebugMarkerSetObjectNameEXT, terminator_vkDebugMarkerSetObjectTagEXT,
    terminator_vkSetDebugUtilsObjectNameEXT, terminator_vkSetDebugUtilsObjectTagEXT,
    vkDebugMarkerSetObjectNameEXT, vkDebugMarkerSetObjectTagEXT, vkSetDebugUtilsObjectNameEXT,
    vkSetDebugUtilsObjectTagEXT,
};
