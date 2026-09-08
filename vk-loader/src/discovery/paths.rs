//! Manifest search paths and deduplication.

use super::LayerManifest;
#[cfg(unix)]
use super::box_values;
use super::collect_values;
use super::owned_path;
#[cfg(any(unix, windows))]
use crate::pending;
use crate::platform;
use alloc::vec::Vec;
#[cfg(not(any(unix, windows)))]
use std::env;
#[cfg(windows)]
use std::ffi::OsString;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt as _;
#[cfg(unix)]
use std::path::Path;
use std::{ffi::OsStr, path::PathBuf};

#[cfg(unix)]
pub(super) fn split_paths(value: &OsStr) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for path in value
        .as_bytes()
        .split(|byte| *byte == b':')
        .filter(|path| !path.is_empty())
    {
        let Some(path) = owned_path(Path::new(OsStr::from_bytes(path))) else {
            return Vec::new();
        };
        if crate::allocation::try_push(&mut paths, path).is_err() {
            pending::mark_json_allocation_failed();
            return Vec::new();
        }
    }
    paths
}

#[cfg(windows)]
pub(super) fn split_paths(value: &OsStr) -> Vec<PathBuf> {
    try_split_windows_paths(value).unwrap_or_else(|_| {
        pending::mark_json_allocation_failed();
        Vec::new()
    })
}

#[cfg(windows)]
fn try_split_windows_paths(value: &OsStr) -> Result<Vec<PathBuf>, vk::VkResult> {
    let bytes = value.as_encoded_bytes();
    let mut paths = Vec::new();
    let mut quoted = false;
    let mut start = 0;
    for index in 0..=bytes.len() {
        match bytes.get(index) {
            Some(b'"') => {
                quoted = !quoted;
                continue;
            }
            Some(b';') if !quoted => {}
            None => {}
            _ => continue,
        }
        let mut path = OsString::new();
        path.try_reserve_exact(index - start)
            .map_err(|_| vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        for part in bytes[start..index].split(|byte| *byte == b'"') {
            // SAFETY: These bytes come from this OsStr; every cut is adjacent
            // to an ASCII quote or semicolon, hence at an encoding boundary.
            let part = unsafe { OsStr::from_encoded_bytes_unchecked(part) };
            // The reservation covers all original bytes. Removing quotes and
            // concatenating encoded pieces cannot increase their total length.
            path.push(part);
        }
        if !path.is_empty() {
            crate::allocation::try_push(&mut paths, PathBuf::from(path))?;
        }
        start = index + 1;
    }
    Ok(paths)
}

#[cfg(not(any(unix, windows)))]
pub(super) fn split_paths(value: &OsStr) -> Vec<PathBuf> {
    collect_values(env::split_paths(value).filter(|path| !path.as_os_str().is_empty()))
        .unwrap_or_default()
        .into_vec()
}

pub(super) fn deduplicate_paths(paths: &mut Vec<PathBuf>) {
    let mut index = 1;
    while index < paths.len() {
        let duplicate = paths[..index]
            .iter()
            .any(|existing| existing.as_os_str() == paths[index].as_os_str());
        if duplicate {
            paths.remove(index);
        } else {
            index += 1;
        }
    }
}

pub(super) fn unique_paths(paths: &[PathBuf]) -> Box<[PathBuf]> {
    collect_values(
        paths
            .iter()
            .enumerate()
            .filter(|(index, path)| !paths[..*index].iter().any(|previous| previous == *path))
            .filter_map(|(_, path)| owned_path(path)),
    )
    .unwrap_or_default()
}

pub(super) fn deduplicate_manifests_by_name(manifests: &mut Vec<LayerManifest>) {
    let mut index = 1;
    while index < manifests.len() {
        let duplicate = manifests[..index].iter().any(|existing| {
            existing.name == manifests[index].name
                && existing.manifest_path != manifests[index].manifest_path
        });
        if duplicate {
            manifests.remove(index);
        } else {
            index += 1;
        }
    }
}

#[cfg(unix)]
fn append_search_root(paths: &mut Vec<PathBuf>, root: impl AsRef<Path>, leaf: &str) {
    append_nested_search_root(paths, root.as_ref(), None, leaf);
}

#[cfg(unix)]
pub(super) fn append_nested_search_root(
    paths: &mut Vec<PathBuf>,
    root: &Path,
    nested: Option<&str>,
    leaf: &str,
) {
    if nested.is_none() && platform::is_json_path(root) {
        if !paths.iter().any(|existing| existing == root) {
            if paths.try_reserve(1).is_err() {
                pending::mark_json_allocation_failed();
                return;
            }
            if let Some(root) = owned_path(root) {
                paths.push(root);
            }
        }
        return;
    }
    let mut path = PathBuf::new();
    let length = root
        .as_os_str()
        .len()
        .checked_add(nested.map_or(0, str::len))
        .and_then(|length| length.checked_add(usize::from(nested.is_some())))
        .and_then(|length| length.checked_add("vulkan".len()))
        .and_then(|length| length.checked_add(leaf.len()))
        .and_then(|length| length.checked_add(2));
    if length.is_none_or(|length| path.try_reserve_exact(length).is_err())
        || paths.try_reserve(1).is_err()
    {
        pending::mark_json_allocation_failed();
        return;
    }
    path.push(root);
    if let Some(nested) = nested {
        path.push(nested);
    }
    path.push("vulkan");
    path.push(leaf);
    let path_string = path.as_os_str();
    if !paths
        .iter()
        .any(|existing| existing.as_os_str() == path_string)
    {
        paths.push(path);
    }
}

#[cfg(unix)]
fn append_split_search_roots(paths: &mut Vec<PathBuf>, roots: &OsStr, leaf: &str) {
    for root in roots
        .as_bytes()
        .split(|byte| *byte == b':')
        .filter(|root| !root.is_empty())
    {
        append_search_root(paths, Path::new(OsStr::from_bytes(root)), leaf);
    }
}

#[cfg(unix)]
pub(crate) fn default_search_paths(directory: platform::ManifestDirectory) -> Box<[PathBuf]> {
    try_default_search_paths(directory).unwrap_or_else(|_| {
        pending::mark_json_allocation_failed();
        Box::default()
    })
}

#[cfg(unix)]
fn try_default_search_paths(
    directory: platform::ManifestDirectory,
) -> Result<Box<[PathBuf]>, vk::VkResult> {
    let leaf = directory.as_str();
    let mut paths = Vec::new();
    let elevated = platform::has_elevated_privileges();
    let environment = |name| {
        // SAFETY: Discovery follows upstream's exclusion of concurrent
        // environment mutation, including from allocation callbacks.
        unsafe { platform::environment_value(name) }
    };

    #[cfg(target_vendor = "apple")]
    {
        if let Some(resources) = platform::bundle_resource_directory() {
            append_search_root(&mut paths, resources, leaf);
        }
        // Presence, rather than the environment variable's value, activates
        // the upstream Apple loader's bundle-exclusive mode.
        if !elevated && environment(c"VK_LOADER_SEARCH_ONLY_IN_BUNDLE")?.is_some() {
            return Ok(box_values(paths));
        }
    }

    if elevated {
        if !cfg!(any(
            target_os = "fuchsia",
            target_os = "nto",
            target_os = "qnx"
        )) {
            append_search_root(&mut paths, "/etc/xdg", leaf);
        }
    } else {
        if let Some(value) = environment(c"XDG_CONFIG_HOME")? {
            append_split_search_roots(&mut paths, &value, leaf);
        } else if let Some(value) = environment(c"HOME")? {
            append_nested_search_root(&mut paths, Path::new(&value), Some(".config"), leaf);
        }
        if let Some(value) = environment(c"XDG_CONFIG_DIRS")? {
            append_split_search_roots(&mut paths, &value, leaf);
        } else if !cfg!(any(
            target_os = "fuchsia",
            target_os = "nto",
            target_os = "qnx"
        )) {
            append_search_root(&mut paths, "/etc/xdg", leaf);
        }
    }

    if cfg!(target_os = "fuchsia") {
        append_search_root(&mut paths, "/config", leaf);
        append_search_root(&mut paths, "/pkg/data", leaf);
    } else if cfg!(any(target_os = "nto", target_os = "qnx")) {
        append_search_root(&mut paths, "/etc", leaf);
    } else {
        append_search_root(&mut paths, "/usr/local/etc", leaf);
        append_search_root(&mut paths, "/etc", leaf);
    }

    if !elevated {
        if let Some(value) = environment(c"XDG_DATA_HOME")? {
            append_split_search_roots(&mut paths, &value, leaf);
        } else if let Some(value) = environment(c"HOME")? {
            append_nested_search_root(&mut paths, Path::new(&value), Some(".local/share"), leaf);
        }
        if let Some(value) = environment(c"XDG_DATA_DIRS")? {
            append_split_search_roots(&mut paths, &value, leaf);
        } else if !cfg!(any(
            target_os = "fuchsia",
            target_os = "nto",
            target_os = "qnx"
        )) {
            append_search_root(&mut paths, "/usr/local/share", leaf);
            append_search_root(&mut paths, "/usr/share", leaf);
        }
    } else if !cfg!(any(
        target_os = "fuchsia",
        target_os = "nto",
        target_os = "qnx"
    )) {
        append_search_root(&mut paths, "/usr/local/share", leaf);
        append_search_root(&mut paths, "/usr/share", leaf);
    }
    Ok(box_values(paths))
}

#[cfg(windows)]
pub(crate) fn default_search_paths(directory: platform::ManifestDirectory) -> Box<[PathBuf]> {
    platform::registry_manifest_files(directory)
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn default_search_paths(_directory: platform::ManifestDirectory) -> Box<[PathBuf]> {
    Box::default()
}
