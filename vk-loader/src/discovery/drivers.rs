//! Driver discovery and filtering.

use super::silent_global_loader_settings;

use super::DriverDisposition;
use super::DriverScan;
use super::LoaderSettings;
use super::box_values;
use super::collect_values;
use super::deduplicate_paths;
use super::default_search_paths;
use super::extend_values;
use super::owned_path;
use super::parse_manifest_result;
use super::push_value;
use super::split_paths;
use crate::{pending, platform};
use alloc::vec::Vec;
use core::ffi::CStr;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

fn driver_environment(name: &CStr) -> Option<OsString> {
    if pending::json_allocation_failed() {
        return None;
    }
    // SAFETY: Discovery follows upstream's exclusion of environment mutation
    // during loader configuration reads, including allocation callbacks.
    unsafe { platform::environment_value(name) }.unwrap_or_else(|_| {
        pending::mark_json_allocation_failed();
        None
    })
}

#[cfg(windows)]
fn driver_search_roots_with_diagnostics() -> (Box<[PathBuf]>, Option<platform::RegistryDiagnostics>)
{
    let elevated = platform::has_elevated_privileges();
    let override_paths = (!elevated)
        .then(|| {
            driver_environment(c"VK_DRIVER_FILES")
                .or_else(|| driver_environment(c"VK_ICD_FILENAMES"))
        })
        .flatten();
    if pending::json_allocation_failed() {
        return (Box::default(), None);
    }
    let has_override = override_paths.is_some();
    let (mut roots, diagnostics) = override_paths.map_or_else(
        || {
            let (files, diagnostics) = platform::registry_manifest_files_with_diagnostics(
                platform::ManifestDirectory::Driver,
            );
            (files.into_vec(), Some(diagnostics))
        },
        |value| (split_paths(&value), None),
    );
    if !elevated
        && !has_override
        && let Some(value) = driver_environment(c"VK_ADD_DRIVER_FILES")
    {
        let mut additional = split_paths(&value);
        extend_values(&mut additional, roots);
        roots = additional;
    }
    deduplicate_paths(&mut roots);
    (box_values(roots), diagnostics)
}

#[cold]
#[inline(never)]
pub(crate) fn scan_drivers() -> DriverScan {
    let settings = silent_global_loader_settings();
    scan_drivers_with_settings(settings.as_ref())
}

pub(crate) fn scan_drivers_with_settings(settings: Option<&LoaderSettings>) -> DriverScan {
    let elevated = platform::has_elevated_privileges();
    let use_driver_environment =
        settings.is_none_or(|settings| settings.device_configurations.is_none());
    let environment_override = !elevated
        && use_driver_environment
        && (driver_environment(c"VK_DRIVER_FILES").is_some()
            || driver_environment(c"VK_ICD_FILENAMES").is_some());
    #[cfg(windows)]
    let (roots, registry_diagnostics) = if use_driver_environment {
        driver_search_roots_with_diagnostics()
    } else {
        (
            default_search_paths(platform::ManifestDirectory::Driver),
            None,
        )
    };
    #[cfg(not(windows))]
    let roots = if use_driver_environment {
        driver_search_roots()
    } else {
        default_search_paths(platform::ManifestDirectory::Driver)
    };
    let mut files = Vec::new();
    let mut reported_files = Vec::new();
    for root in &roots {
        extend_values(&mut reported_files, platform::manifest_files(root));
    }
    deduplicate_paths(&mut reported_files);
    if let Some(settings) = settings {
        for path in settings.additional_drivers.iter().rev() {
            extend_values(&mut files, platform::manifest_files(path));
        }
    }
    if settings.is_none_or(|settings| {
        !settings.additional_drivers_use_exclusively || settings.additional_drivers.is_empty()
    }) {
        extend_values(
            &mut files,
            reported_files.iter().filter_map(|path| owned_path(path)),
        );
    }

    deduplicate_paths(&mut files);
    let select = (!elevated && use_driver_environment)
        .then(|| driver_environment(c"VK_LOADER_DRIVERS_SELECT"))
        .flatten()
        .filter(|filters| filters.to_str().is_some_and(|filters| !filters.is_empty()));
    let disable = (!elevated && use_driver_environment)
        .then(|| driver_environment(c"VK_LOADER_DRIVERS_DISABLE"))
        .flatten()
        .filter(|filters| filters.to_str().is_some_and(|filters| !filters.is_empty()));
    let candidates: Box<[_]> = collect_values(files.into_iter().map(|path| {
        let selected = select
            .as_deref()
            .and_then(|filters| filters.to_str())
            .is_some_and(|filters| driver_filter_matches(filters, &path));
        let disposition = if selected {
            DriverDisposition::Accepted
        } else if disable
            .as_deref()
            .and_then(|filters| filters.to_str())
            .is_some_and(|filters| driver_filter_matches(filters, &path))
        {
            DriverDisposition::Disabled
        } else if select.is_some() {
            DriverDisposition::NotSelected
        } else {
            DriverDisposition::Accepted
        };
        (path, disposition)
    }))
    .unwrap_or_default();
    let mut manifests = Vec::new();
    let mut manifest_errors = Vec::new();
    for (path, _) in candidates
        .iter()
        .filter(|(_, disposition)| *disposition == DriverDisposition::Accepted)
    {
        match parse_manifest_result(path) {
            Ok(manifest) => push_value(&mut manifests, manifest),
            Err(error) => {
                if let Some(path) = owned_path(path) {
                    push_value(&mut manifest_errors, (path, error));
                }
            }
        }
    }
    DriverScan {
        manifests: box_values(manifests),
        manifest_errors: box_values(manifest_errors),
        candidates,
        reported_files: box_values(reported_files),
        search_roots: roots,
        environment_override,
        #[cfg(windows)]
        registry_diagnostics,
    }
}

#[cfg(not(windows))]
pub(crate) fn driver_search_roots() -> Box<[PathBuf]> {
    let elevated = platform::has_elevated_privileges();
    let override_paths = (!elevated)
        .then(|| {
            driver_environment(c"VK_DRIVER_FILES")
                .or_else(|| driver_environment(c"VK_ICD_FILENAMES"))
        })
        .flatten();
    if pending::json_allocation_failed() {
        return Box::default();
    }
    let has_override = override_paths.is_some();
    let mut roots = override_paths.map_or_else(
        || default_search_paths(platform::ManifestDirectory::Driver).into_vec(),
        |value| split_paths(&value),
    );
    if !elevated
        && !has_override
        && let Some(value) = driver_environment(c"VK_ADD_DRIVER_FILES")
    {
        let mut additional = split_paths(&value);
        extend_values(&mut additional, roots);
        roots = additional;
    }
    deduplicate_paths(&mut roots);
    box_values(roots)
}

fn driver_filter_matches(filters: &str, path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    filters.split(',').any(|filter| {
        filter.eq_ignore_ascii_case("~all~")
            || wildcard_matches_ascii(filter.as_bytes(), name.as_bytes())
    })
}

fn wildcard_matches_ascii(pattern: &[u8], value: &[u8]) -> bool {
    let (mut pattern_index, mut value_index) = (0, 0);
    let (mut star, mut retry) = (None, 0);
    while value_index < value.len() {
        if pattern_index < pattern.len()
            && pattern[pattern_index] != b'*'
            && pattern[pattern_index].eq_ignore_ascii_case(&value[value_index])
        {
            pattern_index += 1;
            value_index += 1;
        } else if pattern_index < pattern.len() && pattern[pattern_index] == b'*' {
            star = Some(pattern_index);
            pattern_index += 1;
            retry = value_index;
        } else if let Some(star_index) = star {
            pattern_index = star_index + 1;
            retry += 1;
            value_index = retry;
        } else {
            return false;
        }
    }
    pattern[pattern_index..].iter().all(|byte| *byte == b'*')
}
