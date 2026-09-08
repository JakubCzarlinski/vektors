//! Operating-system services used by loader discovery.

#[cfg(target_vendor = "apple")]
mod apple;
mod environment;
mod environment_predicate;
mod environment_text;
mod executable;
mod filesystem;
#[cfg(target_os = "fuchsia")]
mod fuchsia;
mod library;
mod lifecycle;
mod logging;
mod symbols;
mod synchronization;
#[cfg(test)]
mod tests;
#[cfg(any(windows, test))]
mod windows;

pub(crate) use environment::environment_value;
pub(crate) use environment::{has_elevated_privileges, with_elevated_privileges_snapshot};
pub(crate) use environment_predicate::environment_matches;
pub(crate) use environment_text::{inspect_environment_lossy, inspect_environment_text};
pub(crate) use executable::executable_path;
pub(crate) use filesystem::is_json_path;
pub(crate) use filesystem::{file_exists, manifest_files, path_normalizes, read_file};
#[cfg(target_os = "fuchsia")]
use library::OpenLibraryError;
pub(crate) use library::{LoaderLibrary, loaded_library_path};
use lifecycle::dynamic_library_unloading_disabled;
pub(crate) use lifecycle::initialize_loader;
pub(crate) use logging::{
    loader_debug_filter_enabled, loader_debug_logging_enabled, reset_loader_settings_log_filter,
    set_loader_settings_log_filter, write_loader_category_log, write_loader_category_log_any,
    write_loader_log, write_loader_log_with_category, write_stderr, write_stderr_fmt,
};
#[cfg(target_vendor = "apple")]
use std::path::PathBuf;
#[cfg(windows)]
use symbols::reinterpret_function_pointer;
#[cfg(unix)]
use symbols::{access, closedir, fopen, fputs, opendir, readdir, stderr_stream};
pub(crate) use synchronization::{
    LoaderLockGuard, current_thread_key, lock_loader, try_lock_loader,
};
#[cfg(windows)]
pub(crate) use windows::{
    RegistryDiagnostics, adapter_luids, registry_manifest_files,
    registry_manifest_files_with_diagnostics, settings_files,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ManifestDirectory {
    Driver,
    ExplicitLayer,
    ImplicitLayer,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum LogFilter {
    Error,
    Warning,
    Info,
    Debug,
    Performance,
    Driver,
    Layer,
}

impl LogFilter {
    pub(crate) const ALL: [Self; 7] = [
        Self::Error,
        Self::Warning,
        Self::Info,
        Self::Debug,
        Self::Performance,
        Self::Driver,
        Self::Layer,
    ];

    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warn",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Performance => "perf",
            Self::Driver => "driver",
            Self::Layer => "layer",
        }
    }

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Error => "ERROR",
            Self::Warning => "WARNING",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Performance => "PERF",
            Self::Driver => "DRIVER",
            Self::Layer => "LAYER",
        }
    }

    pub(crate) const fn bit(self) -> u8 {
        match self {
            Self::Error => 1 << 0,
            Self::Warning => 1 << 1,
            Self::Info => 1 << 2,
            Self::Debug => 1 << 3,
            Self::Performance => 1 << 4,
            Self::Driver => 1 << 5,
            Self::Layer => 1 << 6,
        }
    }

    pub(crate) fn matches_name(self, name: &str) -> bool {
        name.eq_ignore_ascii_case(self.name())
            || (self == Self::Warning && name.eq_ignore_ascii_case("warning"))
    }

    pub(crate) fn from_severity(severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT) -> Self {
        match severity {
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR => Self::Error,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING => Self::Warning,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO => Self::Info,
            _ => Self::Debug,
        }
    }
}

impl ManifestDirectory {
    #[cfg(unix)]
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Driver => "icd.d",
            Self::ExplicitLayer => "explicit_layer.d",
            Self::ImplicitLayer => "implicit_layer.d",
        }
    }
}

#[cfg(target_vendor = "apple")]
pub(crate) fn bundle_resource_directory() -> Option<PathBuf> {
    apple::resource_directory()
}

#[cfg(windows)]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AdapterLuid {
    pub(crate) low_part: u32,
    pub(crate) high_part: i32,
}
