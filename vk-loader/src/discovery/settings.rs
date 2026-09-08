//! Loader settings parsing, lookup, and scoped caching.

use super::manifest::{owned_byte_path, printed_bytes};
use super::{
    collect_optional_values, collect_values, owned_box_str, owned_c_string, owned_path,
    parse_json_value, parse_layer_manifest, probe_instance_shrinking_reallocation,
    shadow_json_allocations,
};
use crate::debug::diagnostics::{self, LossyBytes};
use crate::json::Value;
use crate::platform::LogFilter;
use crate::sync::{GlobalMutex, MutexAcquire};
use crate::{pending, platform};
use alloc::ffi::CString;
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
    let Ok(shadow_root) = shadow_json_allocations(&display_path, &bytes) else {
        return None;
    };
    if shadow_root.as_ref().is_some_and(|root| {
        let (settings, _) = select_settings(root);
        settings
            .and_then(|settings| settings.get("stderr_log"))
            .and_then(Value::as_array)
            .is_some_and(|filters| !filters.is_empty())
    }) && !probe_instance_shrinking_reallocation(256, 6)
    {
        pending::mark_json_allocation_failed();
        return None;
    }
    // Callback-allocation emulation already parsed the document when active.
    // Reuse it rather than allocating a second identical tree.
    let root = shadow_root.or_else(|| parse_json_value(&bytes));
    let selected = select_settings_document(root.as_ref());
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
    let layer_configurations: Option<Box<[SettingsLayerConfiguration]>> = match settings
        .get("layers")
    {
        Some(layers) => collect_optional_values(layers.as_array()?.iter().map(|layer| {
            let control_value = printed_bytes(layer.get("control")?.as_bytes()?);
            let control = LayerControl::parse(core::str::from_utf8(&control_value).unwrap_or(""));
            if control == LayerControl::UnorderedLayerLocation {
                return Some(SettingsLayerConfiguration {
                    name: owned_c_string(b"")?,
                    path: PathBuf::new(),
                    control,
                    treat_as_implicit_manifest: false,
                });
            }
            let name = printed_bytes(layer.get("name")?.as_bytes()?);
            let path = printed_bytes(layer.get("path")?.as_bytes()?);
            Some(SettingsLayerConfiguration {
                name: owned_c_string(&name)?,
                path: owned_byte_path(&path)?,
                control,
                treat_as_implicit_manifest: layer
                    .get("treat_as_implicit_manifest")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            })
        })),
        None => None,
    };
    let additional_drivers = settings
        .get("additional_drivers")
        .and_then(Value::as_array)
        .and_then(|drivers| {
            collect_optional_values(drivers.iter().map(|driver| {
                driver
                    .as_object()?
                    .get("path")?
                    .as_bytes()
                    .map(printed_bytes)
                    .and_then(|path| owned_byte_path(&path))
            }))
        })
        .unwrap_or_default();
    let device_configurations: Option<Box<[DeviceConfiguration]>> =
        settings.get("device_configurations").map(|configurations| {
            collect_values(
                configurations
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(parse_device_configuration),
            )
            .unwrap_or_default()
        });
    if pending::json_allocation_failed() {
        return None;
    }
    let settings_active = layer_configurations.is_some()
        || settings_logging_active
        || !additional_drivers.is_empty()
        || device_configurations.is_some();
    if emit_diagnostics {
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
                && let Some(configurations) = &layer_configurations
            {
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
                            format_args!("Path: {}", configuration.path.display()),
                        );
                        platform::write_stderr_fmt(format_args!(
                            "[Vulkan Loader] DEBUG:          Layer Type: {}\n",
                            if configuration.treat_as_implicit_manifest {
                                "Implicit"
                            } else {
                                "Explicit"
                            }
                        ));
                    }
                    platform::write_stderr_fmt(format_args!(
                        "[Vulkan Loader] DEBUG:          Control: {}\n",
                        configuration.control.as_str()
                    ));
                }
            }
            if filter_enabled(platform::LogFilter::Debug) && !additional_drivers.is_empty() {
                platform::write_stderr("[Vulkan Loader] DEBUG:          ----\n");
                platform::write_stderr_fmt(format_args!(
                    "[Vulkan Loader] DEBUG:          Use Additional Drivers Exclusively = {}\n",
                    if settings
                        .get("additional_drivers_use_exclusively")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                    {
                        "true"
                    } else {
                        "false"
                    }
                ));
                platform::write_stderr_fmt(format_args!(
                    "[Vulkan Loader] DEBUG:          Additional Driver Configurations count = {}\n",
                    additional_drivers.len()
                ));
                for (index, path) in additional_drivers.iter().enumerate() {
                    platform::write_stderr_fmt(format_args!(
                        "[Vulkan Loader] DEBUG:          ---- Driver Configuration [{index}] ----\n"
                    ));
                    platform::write_stderr_fmt(format_args!(
                        "[Vulkan Loader] DEBUG:          Path: {}\n",
                        path.display()
                    ));
                }
            }
            if filter_enabled(platform::LogFilter::Debug)
                && let Some(configurations) = &device_configurations
            {
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
                            "[Vulkan Loader] DEBUG:          deviceName: {name}\n"
                        ));
                    }
                    if let Some(name) = &configuration.driver_name {
                        platform::write_stderr_fmt(format_args!(
                            "[Vulkan Loader] DEBUG:          driverName: {name}\n"
                        ));
                    }
                }
            }
            if filter_enabled(platform::LogFilter::Debug) {
                platform::write_stderr(
                    "[Vulkan Loader] DEBUG:          ---------------------------------\n",
                );
            }
            if let Some(configurations) = &layer_configurations {
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
                            if filter_enabled(platform::LogFilter::Info) {
                                platform::write_stderr_fmt(format_args!(
                                    "[Vulkan Loader] INFO:           Found manifest file {} (file version {}.{}.{})\n",
                                    manifest.manifest_path.display(),
                                    vk::VK_API_VERSION_MAJOR(manifest.manifest_version),
                                    vk::VK_API_VERSION_MINOR(manifest.manifest_version),
                                    vk::VK_API_VERSION_PATCH(manifest.manifest_version),
                                ));
                            }
                            if filter_enabled(platform::LogFilter::Warning)
                                && !manifest.name.to_bytes().starts_with(b"VK_LAYER_")
                            {
                                platform::write_stderr_fmt(format_args!(
                                    "[Vulkan Loader] WARNING:        Layer name {} does not conform to naming standard (Policy #LLP_LAYER_3)\n",
                                    LossyBytes(manifest.name.to_bytes())
                                ));
                            }
                            if manifest.has_component_layers
                                && filter_enabled(platform::LogFilter::Layer)
                            {
                                platform::write_stderr_fmt(format_args!(
                                    "[Vulkan Loader] INFO | LAYER:   Encountered meta-layer \"{}\"\n",
                                    LossyBytes(manifest.name.to_bytes())
                                ));
                            }
                            if !configuration.treat_as_implicit_manifest
                                && manifest.has_pre_instance_functions
                                && filter_enabled(platform::LogFilter::Warning)
                            {
                                platform::write_stderr_fmt(format_args!(
                                    "[Vulkan Loader] WARNING:        Found pre_instance_functions section in explicit layer from \"{}\". This section is only valid in implicit layers. The section will be ignored\n",
                                    manifest.manifest_path.display()
                                ));
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
                            configuration.path.display()
                        ));
                    }
                }
            }
        }
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
    Some(LoaderSettings {
        settings_file_path: file_path,
        layer_configurations,
        additional_drivers,
        additional_drivers_use_exclusively: settings
            .get("additional_drivers_use_exclusively")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        device_configurations,
    })
}

enum SettingsDocumentError {
    InvalidJson,
    NotObject,
    MissingVersion,
    MissingSettings,
}

fn select_settings_document(
    root: Option<&Value>,
) -> Result<(Option<&Value>, bool), SettingsDocumentError> {
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
    selected: &Result<(Option<&Value>, bool), SettingsDocumentError>,
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

fn parse_device_configuration(value: &Value) -> Option<DeviceConfiguration> {
    fn uuid(value: &Value) -> Option<[u8; vk::VK_UUID_SIZE as usize]> {
        let values = value.as_array()?;
        if values.len() != vk::VK_UUID_SIZE as usize {
            return None;
        }
        let mut uuid = [0; vk::VK_UUID_SIZE as usize];
        for (destination, value) in uuid.iter_mut().zip(values) {
            let value = value.as_u64()?;
            if value > u64::from(u8::MAX) {
                return None;
            }
            *destination = value as u8;
        }
        Some(uuid)
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

pub(super) fn select_settings(root: &Value) -> (Option<&Value>, bool) {
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
    path.push(root);
    if let Some(nested) = nested {
        path.push(nested);
    }
    path.push("vulkan/loader_settings.d");
    path.push("vk_loader_settings.json");
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
