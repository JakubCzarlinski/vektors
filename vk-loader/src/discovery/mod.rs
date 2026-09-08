//! Driver manifest discovery and parsing.

mod diagnostics;
mod drivers;
mod graph;
mod layers;
mod manifest;
mod paths;
mod settings;
mod shadow;
mod storage;
#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) use tests::override_manifest as test_manifest;

pub(crate) use diagnostics::layer_manifest_diagnostics;
pub(crate) use diagnostics::layer_manifest_version_text;
pub(crate) use diagnostics::unused_override_layer_count;
pub(crate) use drivers::scan_drivers;
pub(crate) use drivers::scan_drivers_with_settings;
pub(crate) use graph::valid_layer_mask;
pub(crate) use layers::discover_implicit_layers_with_settings;
pub(crate) use layers::discover_layers;
pub(crate) use layers::discover_layers_with_settings;
use manifest::cjson_string;
use manifest::cjson_value_string;
use manifest::parse_api_version;
pub(crate) use manifest::parse_layer_manifest;
use manifest::parse_layer_manifest_inner;
pub(crate) use manifest::parse_manifest;
use manifest::parse_manifest_result;
pub(crate) use manifest::reparse_layer_manifest;
use paths::deduplicate_manifests_by_name;
use paths::deduplicate_paths;
pub(crate) use paths::default_search_paths;
use paths::split_paths;
use paths::unique_paths;
use shadow::LayerAllocationShadow;
use shadow::parse_json_value;
pub(crate) use shadow::probe_instance_allocation;
use shadow::probe_instance_shrinking_reallocation;
use shadow::shadow_json_allocations;
use shadow::shadow_layer_json_allocations;
use storage::box_array;
use storage::box_values;
use storage::collect_optional_values;
use storage::collect_values;
use storage::extend_values;
use storage::own_cow;
use storage::owned_box_str;
use storage::owned_c_string;
use storage::owned_path;
use storage::owned_string;
use storage::push_value;

#[cfg(all(test, unix))]
use settings::read_settings_from_root;
#[cfg(test)]
use settings::select_settings;
pub(crate) use settings::{
    DeviceConfiguration, LayerControl, LoaderSettings, diagnostic_global_loader_settings,
    format_uuid, global_loader_settings, loader_settings, loader_settings_file_present,
    silent_global_loader_settings, with_loader_settings_absence_cache,
};
#[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
pub(crate) use settings::{destroy_global_settings_lock, release_global_loader_settings};

#[cfg(windows)]
use crate::platform;
use alloc::{ffi::CString, string::String, vec::Vec};
use core::ops::Deref;
use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DriverManifest {
    pub(crate) manifest_path: PathBuf,
    pub(crate) library_path: PathBuf,
    pub(crate) manifest_version: u32,
    pub(crate) api_version: u32,
    pub(crate) architecture_supported: bool,
    pub(crate) portability_driver: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LayerExtension {
    pub(crate) name: CString,
    pub(crate) spec_version: u32,
    pub(crate) entrypoints: Box<[CString]>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LayerManifest {
    pub(crate) source_index: usize,
    pub(crate) name: CString,
    pub(crate) manifest_path: PathBuf,
    pub(crate) library_path: Option<PathBuf>,
    pub(crate) manifest_version: u32,
    pub(crate) api_version: u32,
    pub(crate) architecture_supported: bool,
    pub(crate) implementation_version: u32,
    pub(crate) description: CString,
    pub(crate) instance_extensions: Box<[LayerExtension]>,
    pub(crate) device_extensions: Box<[LayerExtension]>,
    pub(crate) enable_environment: Option<(OsString, OsString)>,
    pub(crate) disable_environment: Option<(OsString, OsString)>,
    pub(crate) component_layers: Box<[CString]>,
    pub(crate) has_component_layers: bool,
    pub(crate) blacklisted_layers: Box<[CString]>,
    pub(crate) override_paths: Box<[PathBuf]>,
    pub(crate) app_keys: Box<[PathBuf]>,
    pub(crate) has_app_keys: bool,
    pub(crate) functions: LayerFunctions,
    pub(crate) pre_instance_functions: PreInstanceFunctions,
    pub(crate) has_pre_instance_functions: bool,
    pub(crate) implicit: bool,
    pub(crate) settings_control: Option<LayerControl>,
}

pub(crate) struct DiscoveredLayers {
    manifests: Box<[LayerManifest]>,
    searches: Box<[LayerSearch]>,
    configured_manifest_reports: Box<[(PathBuf, u32)]>,
    implicit_only: bool,
}

pub(crate) struct LayerSearch {
    pub(crate) implicit: bool,
    pub(crate) roots: Box<[PathBuf]>,
    pub(crate) files: Box<[PathBuf]>,
    pub(crate) diagnostic_files: Box<[PathBuf]>,
}

pub(crate) enum LayerManifestDiagnostic {
    FailedOpen,
    InvalidJson,
    MissingFileFormatVersion,
    MissingLayers {
        version: String,
        parsed_version: u32,
    },
    UnknownManifestVersion {
        version: String,
        parsed_version: u32,
    },
    UnsupportedLayersArray {
        found_version: String,
        version: String,
    },
    NonConformingName {
        manifest_version: u32,
        name: String,
    },
    MissingRequiredValue {
        manifest_version: u32,
        name: &'static str,
    },
    MissingDisableEnvironment {
        manifest_version: u32,
        name: String,
        meta_layer: bool,
    },
    InvalidDisableEnvironment {
        manifest_version: u32,
        name: String,
    },
    InvalidLibraryAndComponents {
        manifest_version: u32,
        name: String,
        both_defined: bool,
    },
}

impl DiscoveredLayers {
    pub(crate) fn searches(&self) -> &[LayerSearch] {
        &self.searches
    }

    pub(crate) fn configured_manifest_reports(&self) -> &[(PathBuf, u32)] {
        &self.configured_manifest_reports
    }

    pub(crate) const fn implicit_only(&self) -> bool {
        self.implicit_only
    }

    pub(crate) fn into_vec(self) -> Vec<LayerManifest> {
        self.into_manifests().into_vec()
    }

    pub(crate) fn into_manifests(self) -> Box<[LayerManifest]> {
        self.manifests
    }
}

impl Deref for DiscoveredLayers {
    type Target = [LayerManifest];

    fn deref(&self) -> &Self::Target {
        &self.manifests
    }
}

impl<'a> IntoIterator for &'a DiscoveredLayers {
    type Item = &'a LayerManifest;
    type IntoIter = core::slice::Iter<'a, LayerManifest>;

    fn into_iter(self) -> Self::IntoIter {
        self.manifests.iter()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct LayerFunctions {
    pub(crate) negotiate: Option<CString>,
    pub(crate) get_instance_proc_addr: Option<CString>,
    pub(crate) get_device_proc_addr: Option<CString>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct PreInstanceFunctions {
    pub(crate) extension_properties: Option<CString>,
    pub(crate) layer_properties: Option<CString>,
    pub(crate) version: Option<CString>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DriverDisposition {
    Accepted,
    NotSelected,
    Disabled,
}

pub(crate) struct DriverScan {
    pub(crate) manifests: Box<[DriverManifest]>,
    pub(crate) manifest_errors: Box<[(PathBuf, DriverManifestError)]>,
    pub(crate) candidates: Box<[(PathBuf, DriverDisposition)]>,
    pub(crate) reported_files: Box<[PathBuf]>,
    pub(crate) search_roots: Box<[PathBuf]>,
    pub(crate) environment_override: bool,
    #[cfg(windows)]
    pub(crate) registry_diagnostics: Option<platform::RegistryDiagnostics>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DriverManifestError {
    OutOfMemory,
    FailedOpen,
    InvalidJson,
    MissingFileFormatVersion,
    EmptyLibraryPath { manifest_version: u32 },
    Invalid,
}
