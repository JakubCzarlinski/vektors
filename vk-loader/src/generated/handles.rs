// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.

use super::commands::HANDLE_INFOS;
use crate::HandleInfo;
#[allow(dead_code)]
pub(crate) fn handle_info(name: &str) -> Option<HandleInfo> {
    HANDLE_INFOS
        .binary_search_by_key(&name, |info| info.name)
        .ok()
        .map(|index| HANDLE_INFOS[index])
}
