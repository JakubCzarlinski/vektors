//! Loader settings parsing, lookup, and scoped caching.

use super::manifest::{owned_byte_path, printed_bytes};
use super::{
    box_values, owned_box_str, owned_c_string, owned_path, parse_json_value, parse_layer_manifest,
    probe_instance_shrinking_reallocation, shadow_json_allocations,
};
use crate::debug::diagnostics::{self, LossyBytes};
use crate::json::Value;
use crate::platform::LogFilter;
use crate::sync::{GlobalMutex, MutexAcquire};
use crate::{LoaderPathExt, layer};
use crate::{pending, platform};
use alloc::ffi::CString;
use alloc::vec::Vec;
use core::{cell::Cell, ffi::CStr, ops::DerefMut};
#[cfg(unix)]
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt as _;
use std::path::{Path, PathBuf};

thread_local! {
    static SETTINGS_CACHE_STATE: Cell<SettingsCacheState> = const {
        Cell::new(SettingsCacheState::Uncached)
    };
    static LAST_SETTINGS_FILE_FOUND: Cell<bool> = const { Cell::new(false) };
}

static GLOBAL_SETTINGS_BYTES: GlobalMutex<Option<Box<[u8]>>> = GlobalMutex::new(None);

#[derive(Clone, Copy, PartialEq, Eq)]
enum SettingsCacheState {
    Uncached,
    Scoped,
    Absent,
}

struct SettingsCacheGuard(SettingsCacheState);

struct EnabledLogFilters(u8);

impl core::fmt::Display for EnabledLogFilters {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut separator = "";
        for filter in platform::LogFilter::ALL {
            if self.0 & filter.bit() != 0 {
                formatter.write_str(separator)?;
                formatter.write_str(filter.label())?;
                separator = " | ";
            }
        }
        Ok(())
    }
}

impl Drop for SettingsCacheGuard {
    fn drop(&mut self) {
        SETTINGS_CACHE_STATE.set(self.0);
    }
}

pub(crate) fn with_loader_settings_absence_cache<T>(operation: impl FnOnce() -> T) -> T {
    SETTINGS_CACHE_STATE.with(|state| {
        if state.get() != SettingsCacheState::Uncached {
            return operation();
        }
        let previous = state.replace(SettingsCacheState::Scoped);
        let _guard = SettingsCacheGuard(previous);
        operation()
    })
}

#[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
pub(crate) fn release_global_loader_settings() {
    if let Some(mut bytes) = GLOBAL_SETTINGS_BYTES.lock_if_initialized() {
        *bytes = None;
    }
}

#[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
pub(crate) unsafe fn destroy_global_settings_lock() {
    // SAFETY: The caller excludes all loader entry points during termination.
    unsafe { GLOBAL_SETTINGS_BYTES.destroy() };
}

pub(super) struct SettingsLayerConfiguration {
    pub(super) name: CString,
    pub(super) path: PathBuf,
    pub(super) control: LayerControl,
    pub(super) treat_as_implicit_manifest: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LayerControl {
    Default,
    On,
    Off,
    UnorderedLayerLocation,
}

impl LayerControl {
    const fn parse(value: &str) -> Self {
        match value.as_bytes() {
            b"on" => Self::On,
            b"off" => Self::Off,
            b"unordered_layer_location" => Self::UnorderedLayerLocation,
            _ => Self::Default,
        }
    }

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "auto",
            Self::On => "on",
            Self::Off => "off",
            Self::UnorderedLayerLocation => "unordered_layer_location",
        }
    }
}

pub(crate) struct LoaderSettings {
    pub(super) settings_file_path: PathBuf,
    pub(super) layer_configurations: Option<Box<[SettingsLayerConfiguration]>>,
    pub(super) additional_drivers: Box<[PathBuf]>,
    pub(super) additional_drivers_use_exclusively: bool,
    pub(super) device_configurations: Option<Box<[DeviceConfiguration]>>,
}

impl LoaderSettings {
    fn empty() -> Self {
        Self {
            settings_file_path: PathBuf::new(),
            layer_configurations: None,
            additional_drivers: Box::default(),
            additional_drivers_use_exclusively: false,
            device_configurations: None,
        }
    }

    pub(crate) fn settings_file_path(&self) -> &Path {
        &self.settings_file_path
    }

    pub(crate) fn into_device_configurations(self) -> Option<Box<[DeviceConfiguration]>> {
        self.device_configurations
    }

    pub(crate) fn layer_control(&self, name: &CStr) -> Option<LayerControl> {
        self.layer_configurations
            .as_deref()?
            .iter()
            .find_map(|layer| (layer.name.as_c_str() == name).then_some(layer.control))
    }

    pub(crate) const fn has_layer_configurations(&self) -> bool {
        self.layer_configurations.is_some()
    }

    pub(crate) fn has_unordered_layer_location(&self) -> bool {
        self.layer_configurations.as_deref().is_some_and(|layers| {
            layers
                .iter()
                .any(|layer| layer.control == LayerControl::UnorderedLayerLocation)
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DeviceConfiguration {
    pub(crate) device_uuid: [u8; vk::VK_UUID_SIZE as usize],
    pub(crate) driver_uuid: [u8; vk::VK_UUID_SIZE as usize],
    pub(crate) driver_version: u32,
    pub(crate) device_name: Option<Box<str>>,
    pub(crate) driver_name: Option<Box<str>>,
}

struct UuidDisplay([u8; vk::VK_UUID_SIZE as usize]);

impl core::fmt::Display for UuidDisplay {
    fn fmt(&self, output: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let uuid = &self.0;
        write!(
            output,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            uuid[0],
            uuid[1],
            uuid[2],
            uuid[3],
            uuid[4],
            uuid[5],
            uuid[6],
            uuid[7],
            uuid[8],
            uuid[9],
            uuid[10],
            uuid[11],
            uuid[12],
            uuid[13],
            uuid[14],
            uuid[15]
        )
    }
}

pub(crate) fn format_uuid(uuid: &[u8; vk::VK_UUID_SIZE as usize]) -> impl core::fmt::Display {
    UuidDisplay(*uuid)
}

pub(crate) fn loader_settings() -> Option<LoaderSettings> {
    if SETTINGS_CACHE_STATE.get() == SettingsCacheState::Absent {
        return None;
    }
    let settings = load_loader_settings(None, false);
    if settings.is_none() && SETTINGS_CACHE_STATE.get() == SettingsCacheState::Scoped {
        SETTINGS_CACHE_STATE.set(SettingsCacheState::Absent);
    }
    settings
}

pub(crate) fn global_loader_settings() -> Option<LoaderSettings> {
    load_loader_settings(Some(true), false)
}

pub(crate) fn diagnostic_global_loader_settings() -> Option<LoaderSettings> {
    load_loader_settings(Some(true), true)
}

pub(crate) fn loader_settings_file_present() -> bool {
    LAST_SETTINGS_FILE_FOUND.get()
}

pub(crate) fn silent_global_loader_settings() -> Option<LoaderSettings> {
    load_loader_settings(Some(false), false)
}

fn load_loader_settings(global: Option<bool>, force_diagnostics: bool) -> Option<LoaderSettings> {
    let Some((file_path, bytes)) = find_loader_settings_file() else {
        LAST_SETTINGS_FILE_FOUND.set(false);
        return None;
    };
    LAST_SETTINGS_FILE_FOUND.set(true);
    parse_found_loader_settings(global, force_diagnostics, file_path, bytes)
}

#[cold]
#[inline(never)]
fn parse_found_loader_settings(
    global: Option<bool>,
    force_diagnostics: bool,
    file_path: PathBuf,
    bytes: Box<[u8]>,
) -> Option<LoaderSettings> {
    let emit_diagnostics = if force_diagnostics {
        true
    } else if global == Some(true) {
        let previous = GLOBAL_SETTINGS_BYTES.lock_if_initialized();
        previous.as_ref().and_then(|previous| previous.as_deref()) != Some(bytes.as_ref())
    } else {
        global.is_none()
    };
    let Ok(display_path) = diagnostics::settings_path(&file_path) else {
        pending::mark_json_allocation_failed();
        return None;
    };
    let callbacks = pending::instance_allocator();
    let Ok(shadow_root) = shadow_json_allocations(&display_path, &bytes, callbacks) else {
        return None;
    };
    if shadow_root.as_ref().is_some_and(|root| {
        let (settings, _) = select_settings(root.root());
        settings
            .and_then(|settings| settings.get("stderr_log"))
            .and_then(Value::as_array)
            .is_some_and(|filters| !filters.is_empty())
    }) && !probe_instance_shrinking_reallocation(256, 6, callbacks)
    {
        pending::mark_json_allocation_failed();
        return None;
    }
    let (parsed_settings, settings_log_filters, settings_logging_active) = {
        // Callback-allocation emulation already parsed the document when active.
        // Reuse it rather than allocating a second identical tree.
        let root = shadow_root.or_else(|| parse_json_value(&bytes));
        let selected = select_settings_document(root.as_ref().map(crate::json::Document::root));
        if emit_diagnostics {
            emit_settings_document_diagnostics(&selected, &display_path);
        }
        let (settings, _) = selected.ok()?;
        let settings = settings?;
        let settings_log_filters =
            settings
                .get("stderr_log")
                .and_then(Value::as_array)
                .map_or(0, |filters| {
                    filters.iter().fold(0, |mask, value| {
                        let Some(name) = value.as_str() else {
                            return mask;
                        };
                        if name.eq_ignore_ascii_case("all") {
                            return u8::MAX;
                        }
                        platform::LogFilter::ALL
                            .into_iter()
                            .filter(|filter| filter.matches_name(name))
                            .fold(mask, |mask, filter| mask | filter.bit())
                    })
                });
        let settings_logging_active = settings_log_filters != 0;
        platform::set_loader_settings_log_filter(
            settings_logging_active.then_some(settings_log_filters),
        );
        (
            parse_loader_settings(settings),
            settings_log_filters,
            settings_logging_active,
        )
    };
    if pending::json_allocation_failed() {
        return None;
    }
    let parse_valid = parsed_settings.is_some();
    let mut parsed_settings = parsed_settings.unwrap_or_else(LoaderSettings::empty);
    let settings_active = parse_valid
        && (parsed_settings.layer_configurations.is_some()
            || settings_logging_active
            || !parsed_settings.additional_drivers.is_empty()
            || parsed_settings.device_configurations.is_some());
    if emit_diagnostics {
        emit_parsed_settings(
            &parsed_settings,
            &display_path,
            settings_log_filters,
            settings_active,
            global,
        );
    }
    if !settings_active {
        return None;
    }
    // Diagnostic manifest parsing can also fail. Only suppress repeated
    // diagnostics once this load has completed without an allocation failure.
    if pending::json_allocation_failed() {
        return None;
    }
    if global == Some(true)
        && cache_diagnostic_bytes(bytes, GLOBAL_SETTINGS_BYTES.try_lock()).is_err()
    {
        pending::mark_json_allocation_failed();
        return None;
    }
    drop(display_path);
    parsed_settings.settings_file_path = file_path;
    Some(parsed_settings)
}

enum SettingsDocumentError {
    InvalidJson,
    NotObject,
    MissingVersion,
    MissingSettings,
}

fn select_settings_document(
    root: Option<Value<'_>>,
) -> Result<(Option<Value<'_>>, bool), SettingsDocumentError> {
    let root = root.ok_or(SettingsDocumentError::InvalidJson)?;
    if !root.is_object() {
        return Err(SettingsDocumentError::NotObject);
    }
    if root
        .get("file_format_version")
        .and_then(Value::as_bytes)
        .is_none()
    {
        return Err(SettingsDocumentError::MissingVersion);
    }
    if root.get("settings").is_none() && root.get("settings_array").is_none() {
        return Err(SettingsDocumentError::MissingSettings);
    }
    Ok(select_settings(root))
}

#[cold]
fn emit_settings_document_diagnostics(
    selected: &Result<(Option<Value>, bool), SettingsDocumentError>,
    display_path: &impl core::fmt::Display,
) {
    match selected {
        Err(SettingsDocumentError::InvalidJson) => platform::write_loader_log(
            LogFilter::Error,
            format_args!("loader_get_json: Invalid JSON file {display_path}."),
        ),
        Err(SettingsDocumentError::NotObject) => {}
        Err(SettingsDocumentError::MissingVersion) => platform::write_loader_log(
            LogFilter::Debug,
            format_args!(
                "Loader settings file from {display_path} missing required field file_format_version - no loader settings will be active"
            ),
        ),
        Err(SettingsDocumentError::MissingSettings) => platform::write_loader_log(
            LogFilter::Debug,
            format_args!(
                "Loader settings file from {display_path} missing required settings objects: Either one of the \"settings\" or \"settings_array\" objects must be present - no loader settings will be active"
            ),
        ),
        Ok((settings, invalid_element)) => {
            if *invalid_element {
                platform::write_loader_log(
                    LogFilter::Debug,
                    format_args!(
                        "Loader settings file from {display_path} has a settings element that is not an object"
                    ),
                );
            }
            if settings.is_none() {
                platform::write_loader_log(
                    LogFilter::Debug,
                    format_args!(
                        "Loader settings file from {display_path} missing global settings and none of the app specific settings matched the current application - no loader settings will be active"
                    ),
                );
            }
        }
    }
}

fn cache_diagnostic_bytes(
    bytes: Box<[u8]>,
    cache: Result<impl DerefMut<Target = Option<Box<[u8]>>>, vk::VkResult>,
) -> Result<(), vk::VkResult> {
    match cache {
        Ok(mut previous) => *previous = Some(bytes),
        Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY) => {
            return Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY);
        }
        // Losing diagnostic deduplication must not disable parsed settings.
        Err(_) => {}
    }
    Ok(())
}

#[cfg(test)]
#[test]
fn diagnostic_cache_failure_preserves_settings_and_oom_classification() {
    for (error, expected) in [
        (vk::VkResult::ERROR_INITIALIZATION_FAILED, Ok(())),
        (
            vk::VkResult::ERROR_OUT_OF_HOST_MEMORY,
            Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY),
        ),
    ] {
        let cache: Result<&mut Option<Box<[u8]>>, _> = Err(error);
        assert_eq!(cache_diagnostic_bytes(Box::default(), cache), expected);
    }
    let mut cache = None;
    assert_eq!(
        cache_diagnostic_bytes(Box::from(*b"settings"), Ok(&mut cache)),
        Ok(())
    );
    assert_eq!(cache.as_deref(), Some(b"settings".as_slice()));
}

#[inline(never)]
fn parse_device_configuration(value: Value) -> Option<DeviceConfiguration> {
    fn uuid(value: Value) -> Option<[u8; vk::VK_UUID_SIZE as usize]> {
        let mut values = value.as_array()?.iter();
        let mut uuid = [0; vk::VK_UUID_SIZE as usize];
        for destination in &mut uuid {
            let value = values.next()?;
            let value = value.as_u64()?;
            if value > u64::from(u8::MAX) {
                return None;
            }
            *destination = value as u8;
        }
        values.next().is_none().then_some(uuid)
    }

    let driver_version = value.get("driverVersion")?.as_u64()?;
    if driver_version > u64::from(u32::MAX) {
        return None;
    }

    Some(DeviceConfiguration {
        device_uuid: uuid(value.get("deviceUUID")?)?,
        driver_uuid: uuid(value.get("driverUUID")?)?,
        driver_version: driver_version as u32,
        device_name: value
            .get("deviceName")
            .and_then(Value::as_str)
            .filter(|name| !name.is_empty())
            .and_then(owned_box_str),
        driver_name: value
            .get("driverName")
            .and_then(Value::as_str)
            .filter(|name| !name.is_empty())
            .and_then(owned_box_str),
    })
}

pub(super) fn select_settings(root: Value<'_>) -> (Option<Value<'_>>, bool) {
    if root.get("settings_array").is_none()
        && let Some(settings) = root.get("settings")
    {
        return (
            settings.as_object().map(|_| settings),
            !settings.is_object(),
        );
    }
    let Some(settings) = root.get("settings_array").and_then(Value::as_array) else {
        return (None, false);
    };
    let executable = platform::executable_path();
    let mut global = None;
    for settings in settings {
        if !settings.is_object() {
            return (global, true);
        }
        let Some(app_keys) = settings.get("app_keys") else {
            if global.is_none() {
                global = Some(settings);
            }
            continue;
        };
        if executable.as_ref().is_some_and(|executable| {
            app_keys.as_array().is_some_and(|keys| {
                keys.iter().any(|key| {
                    key.as_str()
                        .is_some_and(|key| Path::new(key) == executable.as_path())
                })
            })
        }) {
            return (Some(settings), false);
        }
    }
    (global, false)
}

fn read_settings_file(path: &Path) -> Option<(PathBuf, Box<[u8]>)> {
    platform::file_exists(path)
        .then(|| platform::read_file(path).and_then(|bytes| Some((owned_path(path)?, bytes))))?
}

#[cfg(unix)]
pub(super) fn read_settings_from_root(
    path: &mut PathBuf,
    root: &Path,
    nested: Option<&str>,
) -> Option<(PathBuf, Box<[u8]>)> {
    let required = root
        .as_os_str()
        .len()
        .checked_add(nested.map_or(0, str::len))
        .and_then(|length| length.checked_add("vulkan/loader_settings.d".len()))
        .and_then(|length| length.checked_add("vk_loader_settings.json".len()))
        .and_then(|length| length.checked_add(3));
    path.clear();
    let Some(required) = required else {
        pending::mark_json_allocation_failed();
        return None;
    };
    if path.try_reserve(required).is_err() {
        pending::mark_json_allocation_failed();
        return None;
    }
    // The buffer is empty, so copying the root needs no path-component scan.
    path.as_mut_os_string().push(root.as_os_str());
    if let Some(nested) = nested {
        path.push(nested);
    }
    if cfg!(target_os = "linux") {
        // The suffix is fixed and relative. On Linux only '/' is a separator.
        if path
            .as_os_str()
            .as_bytes()
            .last()
            .is_some_and(|byte| *byte != b'/')
        {
            path.as_mut_os_string().push("/");
        }
        path.as_mut_os_string()
            .push("vulkan/loader_settings.d/vk_loader_settings.json");
    } else {
        path.push("vulkan/loader_settings.d/vk_loader_settings.json");
    }
    read_settings_file(path)
}

#[cfg(unix)]
fn find_loader_settings_file() -> Option<(PathBuf, Box<[u8]>)> {
    let mut candidate = PathBuf::new();
    let secure_environment = !platform::has_elevated_privileges();
    let environment = |name| {
        if !secure_environment {
            return Some(None);
        }
        // SAFETY: Loader discovery requires the process environment not to
        // be mutated concurrently, matching upstream's native lookup contract.
        if let Ok(value) = unsafe { platform::environment_value(name) } {
            Some(value)
        } else {
            crate::pending::mark_json_allocation_failed();
            None
        }
    };
    let config_home = environment(c"XDG_CONFIG_HOME")?;
    let data_home = environment(c"XDG_DATA_HOME")?;
    for path in [config_home.as_deref(), data_home.as_deref()]
        .into_iter()
        .flatten()
    {
        if let Some(settings) = read_settings_from_root(&mut candidate, Path::new(path), None) {
            return Some(settings);
        }
    }

    if let Some(home) = environment(c"HOME")? {
        for nested in [
            config_home.is_none().then_some(".config"),
            data_home.is_none().then_some(".local/share"),
        ]
        .into_iter()
        .flatten()
        {
            if let Some(settings) =
                read_settings_from_root(&mut candidate, Path::new(&home), Some(nested))
            {
                return Some(settings);
            }
        }
    }

    if let Some(config_dirs) = environment(c"XDG_CONFIG_DIRS")?.filter(|paths| !paths.is_empty()) {
        for path in config_dirs.as_bytes().split(|byte| *byte == b':') {
            if let Some(settings) =
                read_settings_from_root(&mut candidate, Path::new(OsStr::from_bytes(path)), None)
            {
                return Some(settings);
            }
        }
    } else if !cfg!(any(
        target_os = "fuchsia",
        target_os = "nto",
        target_os = "qnx"
    )) && let Some(settings) = read_settings_file(Path::new(
        "/etc/xdg/vulkan/loader_settings.d/vk_loader_settings.json",
    )) {
        return Some(settings);
    }

    if cfg!(target_os = "fuchsia") {
        for root in [Path::new("/config"), Path::new("/pkg/data")] {
            if let Some(settings) = read_settings_from_root(&mut candidate, root, None) {
                return Some(settings);
            }
        }
    } else if cfg!(any(target_os = "nto", target_os = "qnx")) {
        if let Some(settings) = read_settings_from_root(&mut candidate, Path::new("/etc"), None) {
            return Some(settings);
        }
    } else {
        // These defaults match an upstream CMake build installed with its
        // default `/usr/local` prefix. `/etc` is EXTRASYSCONFDIR.
        for path in [
            Path::new("/usr/local/etc/vulkan/loader_settings.d/vk_loader_settings.json"),
            Path::new("/etc/vulkan/loader_settings.d/vk_loader_settings.json"),
        ] {
            if let Some(settings) = read_settings_file(path) {
                return Some(settings);
            }
        }
    }

    if let Some(data_dirs) = environment(c"XDG_DATA_DIRS")?.filter(|paths| !paths.is_empty()) {
        for path in data_dirs.as_bytes().split(|byte| *byte == b':') {
            if let Some(settings) =
                read_settings_from_root(&mut candidate, Path::new(OsStr::from_bytes(path)), None)
            {
                return Some(settings);
            }
        }
    } else if !cfg!(any(
        target_os = "fuchsia",
        target_os = "nto",
        target_os = "qnx"
    )) {
        for path in [
            Path::new("/usr/local/share/vulkan/loader_settings.d/vk_loader_settings.json"),
            Path::new("/usr/share/vulkan/loader_settings.d/vk_loader_settings.json"),
        ] {
            if let Some(settings) = read_settings_file(path) {
                return Some(settings);
            }
        }
    }
    None
}

#[cfg(windows)]
pub(super) fn find_loader_settings_file() -> Option<(PathBuf, Box<[u8]>)> {
    platform::settings_files()
        .into_iter()
        .find_map(|path| read_settings_file(&path))
}

#[cfg(not(any(unix, windows)))]
pub(super) fn find_loader_settings_file() -> Option<(PathBuf, Box<[u8]>)> {
    None
}

#[cold]
fn emit_parsed_settings(
    parsed_settings: &LoaderSettings,
    display_path: &impl core::fmt::Display,
    settings_log_filters: u8,
    settings_active: bool,
    global: Option<bool>,
) {
    let settings_logging_active = settings_log_filters != 0;
    let filter_enabled = |filter: platform::LogFilter| {
        if settings_logging_active {
            settings_log_filters & filter.bit() != 0
        } else {
            platform::loader_debug_filter_enabled(filter)
        }
    };
    let stderr_logging_active = settings_logging_active
        || platform::LogFilter::ALL
            .into_iter()
            .any(platform::loader_debug_filter_enabled);
    if !settings_active {
        platform::write_loader_log(
            platform::LogFilter::Info,
            format_args!(
                "vk_loader_settings.json file found at \"{display_path}\" but did not contain any valid settings."
            ),
        );
    } else if stderr_logging_active {
        if filter_enabled(platform::LogFilter::Info) {
            platform::write_stderr_fmt(format_args!(
                "[Vulkan Loader] INFO:           Using layer configurations found in loader settings from {display_path}\n"
            ));
        }
        if settings_logging_active && filter_enabled(platform::LogFilter::Debug) {
            let enabled = EnabledLogFilters(
                platform::LogFilter::ALL
                    .into_iter()
                    .filter(|filter| filter_enabled(*filter))
                    .fold(0, |mask, filter| mask | filter.bit()),
            );
            platform::write_stderr_fmt(format_args!(
                "[Vulkan Loader] DEBUG:          Loader Settings Filters for Logging to Standard Error: {enabled}\n"
            ));
        }
        if filter_enabled(platform::LogFilter::Debug)
            && let Some(configurations) = &parsed_settings.layer_configurations
        {
            emit_layer_configurations(configurations);
        }
        if filter_enabled(platform::LogFilter::Debug)
            && !parsed_settings.additional_drivers.is_empty()
        {
            emit_additional_drivers(parsed_settings);
        }
        if filter_enabled(platform::LogFilter::Debug)
            && let Some(configurations) = &parsed_settings.device_configurations
        {
            emit_device_configurations(configurations);
        }
        if filter_enabled(platform::LogFilter::Debug) {
            platform::write_stderr(
                "[Vulkan Loader] DEBUG:          ---------------------------------\n",
            );
        }
        if let Some(configurations) = &parsed_settings.layer_configurations {
            emit_configured_manifest_diagnostics(
                configurations,
                global,
                settings_logging_active.then_some(settings_log_filters),
            );
        }
    }
}

#[cold]
fn emit_layer_configurations(configurations: &[SettingsLayerConfiguration]) {
    platform::write_stderr_fmt(format_args!(
        "[Vulkan Loader] DEBUG:          Layer Configurations count = {}\n",
        configurations.len()
    ));
    for (index, configuration) in configurations.iter().enumerate() {
        platform::write_stderr_fmt(format_args!(
            "[Vulkan Loader] DEBUG:          ---- Layer Configuration [{index}] ----\n"
        ));
        if configuration.control != LayerControl::UnorderedLayerLocation {
            platform::write_loader_log(
                platform::LogFilter::Debug,
                format_args!("Name: {}", LossyBytes(configuration.name.to_bytes())),
            );
            platform::write_loader_log(
                platform::LogFilter::Debug,
                format_args!("Path: {}", configuration.path.loader_display()),
            );
            platform::write_stderr_fmt(format_args!(
                "[Vulkan Loader] DEBUG:          Layer Type: {}\n",
                diagnostics::Text(if configuration.treat_as_implicit_manifest {
                    "Implicit"
                } else {
                    "Explicit"
                })
            ));
        }
        platform::write_stderr_fmt(format_args!(
            "[Vulkan Loader] DEBUG:          Control: {}\n",
            diagnostics::Text(configuration.control.as_str())
        ));
    }
}

#[cold]
fn emit_additional_drivers(parsed_settings: &LoaderSettings) {
    platform::write_stderr("[Vulkan Loader] DEBUG:          ----\n");
    platform::write_stderr_fmt(format_args!(
        "[Vulkan Loader] DEBUG:          Use Additional Drivers Exclusively = {}\n",
        diagnostics::Text(if parsed_settings.additional_drivers_use_exclusively {
            "true"
        } else {
            "false"
        })
    ));
    platform::write_stderr_fmt(format_args!(
        "[Vulkan Loader] DEBUG:          Additional Driver Configurations count = {}\n",
        parsed_settings.additional_drivers.len()
    ));
    for (index, path) in parsed_settings.additional_drivers.iter().enumerate() {
        platform::write_stderr_fmt(format_args!(
            "[Vulkan Loader] DEBUG:          ---- Driver Configuration [{index}] ----\n"
        ));
        platform::write_stderr_fmt(format_args!(
            "[Vulkan Loader] DEBUG:          Path: {}\n",
            path.loader_display()
        ));
    }
}

#[cold]
fn emit_device_configurations(configurations: &[DeviceConfiguration]) {
    platform::write_stderr("[Vulkan Loader] DEBUG:          ----\n");
    platform::write_stderr_fmt(format_args!(
        "[Vulkan Loader] DEBUG:          Device Configurations count = {}\n",
        configurations.len()
    ));
    for (index, configuration) in configurations.iter().enumerate() {
        platform::write_stderr_fmt(format_args!(
            "[Vulkan Loader] DEBUG:          ---- Device Configuration [{index}] ----\n"
        ));
        platform::write_stderr_fmt(format_args!(
            "[Vulkan Loader] DEBUG:          deviceUUID: {}\n",
            format_uuid(&configuration.device_uuid)
        ));
        platform::write_stderr_fmt(format_args!(
            "[Vulkan Loader] DEBUG:          driverUUID: {}\n",
            format_uuid(&configuration.driver_uuid)
        ));
        platform::write_stderr_fmt(format_args!(
            "[Vulkan Loader] DEBUG:          driverVersion: {}\n",
            configuration.driver_version
        ));
        if let Some(name) = &configuration.device_name {
            platform::write_stderr_fmt(format_args!(
                "[Vulkan Loader] DEBUG:          deviceName: {}\n",
                diagnostics::Text(name)
            ));
        }
        if let Some(name) = &configuration.driver_name {
            platform::write_stderr_fmt(format_args!(
                "[Vulkan Loader] DEBUG:          driverName: {}\n",
                diagnostics::Text(name)
            ));
        }
    }
}

#[cold]
fn emit_configured_manifest_diagnostics(
    configurations: &[SettingsLayerConfiguration],
    global: Option<bool>,
    settings_log_filters: Option<u8>,
) {
    let filter_enabled = |filter| configured_filter_enabled(settings_log_filters, filter);
    for configuration in configurations {
        if global.is_none()
            && platform::is_json_path(&configuration.path)
            && !matches!(
                configuration.control,
                LayerControl::Off | LayerControl::UnorderedLayerLocation
            )
            && [
                platform::LogFilter::Info,
                platform::LogFilter::Warning,
                platform::LogFilter::Layer,
            ]
            .into_iter()
            .any(filter_enabled)
        {
            for manifest in parse_layer_manifest(
                &configuration.path,
                configuration.treat_as_implicit_manifest,
            ) {
                let sink = layer::MetaDiagnosticSink::Global;
                if filter_enabled(platform::LogFilter::Info) {
                    layer::emit_found_manifest_version(
                        sink,
                        &manifest.manifest_path,
                        format_args!("{}", layer::ManifestVersion(manifest.manifest_version)),
                    );
                }
                if filter_enabled(platform::LogFilter::Warning)
                    && !manifest.name.to_bytes().starts_with(b"VK_LAYER_")
                {
                    layer::emit_nonconforming_layer_name(sink, &manifest.name);
                }
                if manifest.is_meta_layer() && filter_enabled(platform::LogFilter::Layer) {
                    layer::emit_meta_layer_encountered(sink, &manifest.name);
                }
                if !configuration.treat_as_implicit_manifest
                    && manifest.has_pre_instance_functions
                    && filter_enabled(platform::LogFilter::Warning)
                {
                    layer::emit_explicit_pre_instance_warning(sink, &manifest.manifest_path);
                }
            }
        }
        if filter_enabled(platform::LogFilter::Error)
            && !matches!(
                configuration.control,
                LayerControl::Off | LayerControl::UnorderedLayerLocation
            )
            && configuration
                .path
                .extension()
                .is_some_and(|extension| extension == "json")
            && !platform::file_exists(&configuration.path)
        {
            platform::write_stderr_fmt(format_args!(
                "[Vulkan Loader] ERROR:          loader_get_json: Failed to open JSON file {}\n",
                configuration.path.loader_display()
            ));
        }
    }
}

#[cold]
#[inline(never)]
fn configured_filter_enabled(settings_log_filters: Option<u8>, filter: LogFilter) -> bool {
    settings_log_filters.map_or_else(
        || platform::loader_debug_filter_enabled(filter),
        |filters| filters & filter.bit() != 0,
    )
}

#[inline(never)]
fn parse_loader_settings(settings: Value) -> Option<LoaderSettings> {
    let layer_configurations = match settings.get("layers") {
        Some(layers) => Some(parse_layer_configurations(layers)?),
        None => None,
    };
    let additional_drivers = parse_additional_drivers(settings.get("additional_drivers"));
    let device_configurations = settings
        .get("device_configurations")
        .map(parse_device_configurations);
    if pending::json_allocation_failed() {
        return None;
    }
    let parsed_settings = LoaderSettings {
        settings_file_path: PathBuf::new(),
        layer_configurations,
        additional_drivers,
        additional_drivers_use_exclusively: settings
            .get("additional_drivers_use_exclusively")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        device_configurations,
    };
    Some(parsed_settings)
}

fn reserve_json_values<T>(values: &mut Vec<T>, len: usize) -> bool {
    if values.try_reserve_exact(len).is_ok() {
        true
    } else {
        pending::mark_json_allocation_failed();
        false
    }
}

#[inline(never)]
fn parse_layer_configurations(value: Value) -> Option<Box<[SettingsLayerConfiguration]>> {
    let layers = value.as_array()?;
    super::collect_exact_values(
        layers.len(),
        layers.into_iter().map(|layer| {
            let control_value = printed_bytes(layer.get("control")?.as_bytes()?);
            let control = LayerControl::parse(core::str::from_utf8(&control_value).unwrap_or(""));
            let configuration = if control == LayerControl::UnorderedLayerLocation {
                SettingsLayerConfiguration {
                    name: owned_c_string(b"")?,
                    path: PathBuf::new(),
                    control,
                    treat_as_implicit_manifest: false,
                }
            } else {
                let name = printed_bytes(layer.get("name")?.as_bytes()?);
                let path = printed_bytes(layer.get("path")?.as_bytes()?);
                SettingsLayerConfiguration {
                    name: owned_c_string(&name)?,
                    path: owned_byte_path(&path)?,
                    control,
                    treat_as_implicit_manifest: layer
                        .get("treat_as_implicit_manifest")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                }
            };
            Some(configuration)
        }),
    )
}

#[inline(never)]
fn parse_additional_drivers(value: Option<Value>) -> Box<[PathBuf]> {
    let Some(drivers) = value.and_then(Value::as_array) else {
        return Box::default();
    };
    for driver in drivers {
        if driver
            .as_object()
            .and_then(|driver| driver.get("path"))
            .and_then(Value::as_bytes)
            .is_none()
        {
            return Box::default();
        }
    }
    super::collect_exact_values(
        drivers.len(),
        drivers.into_iter().map(|driver| {
            let path = driver
                .get("path")
                .and_then(Value::as_bytes)
                .unwrap_or_default();
            owned_byte_path(&printed_bytes(path))
        }),
    )
    .unwrap_or_default()
}

#[inline(never)]
fn parse_device_configurations(value: Value) -> Box<[DeviceConfiguration]> {
    let Some(values) = value.as_array() else {
        return Box::default();
    };
    let mut configurations = Vec::new();
    if !reserve_json_values(&mut configurations, values.len()) {
        return Box::default();
    }
    for value in values {
        if let Some(configuration) = parse_device_configuration(value) {
            configurations.push(configuration);
        }
    }
    box_values(configurations)
}

#[cfg(test)]
#[test]
fn settings_arrays_propagate_every_allocation_failure() {
    let source = br#"{
        "settings": {
            "layers": [
                {"name":"VK_LAYER_test","path":"layer.json","control":"on"},
                {"control":"unordered_layer_location"}
            ],
            "additional_drivers": [
                {"path":"driver-a.json"},
                {"path":"driver-b.json"}
            ],
            "device_configurations": [{
                "deviceUUID":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],
                "driverUUID":[15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0],
                "driverVersion":1,
                "deviceName":"device",
                "driverName":"driver"
            }]
        }
    }"#;
    crate::allocation::fault::sweep_operation(|| {
        pending::with_json_error_scope(|| {
            let root = parse_json_value(source);
            let parsed = root
                .as_ref()
                .and_then(|root| root.root().get("settings"))
                .and_then(parse_loader_settings);
            if pending::json_allocation_failed() {
                assert!(parsed.is_none());
                return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
            }
            let parsed = parsed.unwrap();
            assert_eq!(parsed.layer_configurations.as_deref().unwrap().len(), 2);
            assert_eq!(parsed.additional_drivers.len(), 2);
            assert_eq!(parsed.device_configurations.as_deref().unwrap().len(), 1);
            vk::VkResult::SUCCESS
        })
    });
}
