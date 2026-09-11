//! Layer selection, filtering, and activation.

use super::diagnostics::record_message;
use super::{
    ActiveLayerProperty, ActiveLayers, CStr, CString, LayerControl, LayerEnabledBy,
    LayerFilterVariable, LayerLoadError, LayerManifest, LayerNames, LoadedLayer, LoaderSettings,
    OsStr, SelectedLayers, VkInstanceCreateInfo, VkResult, compatibility_manifest_graph,
    discover_layers_with_settings, emit_create_message, emit_layer_message,
    emit_layer_search_diagnostics, emit_meta_layer_diagnostics, valid_layer_mask,
};
use crate::LoaderPathExt;
use crate::{allocation, debug::diagnostics, pending};
#[cfg(unix)]
use alloc::borrow::Cow;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt as _;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt as _;

pub(super) fn requested_layer_names(
    create_info: &VkInstanceCreateInfo<'_>,
    environment: Option<&OsStr>,
) -> Result<(Box<[CString]>, usize), VkResult> {
    if create_info.enabledLayerCount != 0 && create_info.ppEnabledLayerNames.is_null() {
        return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
    }
    let mut names = Vec::new();
    #[cfg(unix)]
    if let Some(environment) = environment {
        let environment_names = environment
            .as_bytes()
            .split(|byte| *byte == b':')
            .map(OsStr::from_bytes);
        for name in environment_names {
            use crate::{allocation, debug::diagnostics};

            let text = match name.to_str() {
                Some(text) => Cow::Borrowed(text),
                None => Cow::Owned(diagnostics::try_format(format_args!(
                    "{}",
                    name.loader_display()
                ))?),
            };
            let name =
                allocation::try_c_string_bytes(text.as_bytes()).map_err(|error| match error {
                    allocation::CStringError::InteriorNul => VkResult::ERROR_LAYER_NOT_PRESENT,
                    allocation::CStringError::OutOfMemory => VkResult::ERROR_OUT_OF_HOST_MEMORY,
                })?;
            allocation::try_push(&mut names, name)?;
        }
    }
    #[cfg(windows)]
    if let Some(environment) = environment {
        append_windows_layer_names(&mut names, environment.encode_wide())?;
    }
    let environment_count = names.len();
    for index in 0..create_info.enabledLayerCount as usize {
        // SAFETY: Vulkan requires this array and every string to be live.
        let name = unsafe { create_info.ppEnabledLayerNames.add(index).read() };
        if name.is_null() {
            return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
        }
        // SAFETY: Vulkan requires each non-null layer name to be a live C string.
        let name = allocation::try_c_string(unsafe { CStr::from_ptr(name) })?;
        allocation::try_push(&mut names, name)?;
    }
    Ok((allocation::try_into_boxed_slice(names)?, environment_count))
}

#[cfg(any(windows, test))]
pub(super) fn append_windows_layer_names(
    names: &mut Vec<CString>,
    mut units: impl Iterator<Item = u16>,
) -> Result<(), VkResult> {
    let mut more = true;
    while more {
        more = false;
        let mut quoted = false;
        let mut name = Vec::new();
        for unit in units.by_ref() {
            match unit {
                34 => quoted = !quoted,
                59 if !quoted => {
                    more = true;
                    break;
                }
                _ => allocation::try_push(&mut name, unit)?,
            }
        }
        let mut text = String::new();
        for character in core::char::decode_utf16(name) {
            let character = character.unwrap_or(core::char::REPLACEMENT_CHARACTER);
            text.try_reserve(character.len_utf8())
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            text.push(character);
        }
        let name =
            allocation::try_c_string_bytes(text.as_bytes()).map_err(|error| match error {
                allocation::CStringError::InteriorNul => VkResult::ERROR_LAYER_NOT_PRESENT,
                allocation::CStringError::OutOfMemory => VkResult::ERROR_OUT_OF_HOST_MEMORY,
            })?;
        allocation::try_push(names, name)?;
    }
    Ok(())
}

pub(super) fn environment_matches(environment: &(std::ffi::OsString, std::ffi::OsString)) -> bool {
    // SAFETY: Layer discovery follows upstream's no-concurrent-environment-mutation contract.
    unsafe { crate::platform::environment_matches(&environment.0, Some(&environment.1)) }
}

pub(super) fn environment_is_set(name: &OsStr) -> bool {
    // SAFETY: Layer discovery follows upstream's no-concurrent-environment-mutation contract.
    unsafe { crate::platform::environment_matches(name, None) }
}

pub(super) fn wildcard_matches(pattern: &[u8], name: &[u8]) -> bool {
    let (mut pattern_index, mut name_index) = (0, 0);
    let (mut star, mut retry) = (None, 0);
    while name_index < name.len() {
        if pattern_index < pattern.len()
            && pattern[pattern_index] != b'*'
            && pattern[pattern_index].eq_ignore_ascii_case(&name[name_index])
        {
            pattern_index += 1;
            name_index += 1;
        } else if pattern_index < pattern.len() && pattern[pattern_index] == b'*' {
            star = Some(pattern_index);
            pattern_index += 1;
            retry = name_index;
        } else if let Some(star_index) = star {
            pattern_index = star_index + 1;
            retry += 1;
            name_index = retry;
        } else {
            return false;
        }
    }
    while pattern_index < pattern.len() && pattern[pattern_index] == b'*' {
        pattern_index += 1;
    }
    pattern_index == pattern.len()
}

pub(super) fn filter_value_matches(
    variable: LayerFilterVariable,
    matches: impl Fn(&str) -> bool,
) -> bool {
    if crate::platform::has_elevated_privileges() {
        return false;
    }
    // SAFETY: Layer selection excludes concurrent environment mutation.
    // The internal matching callbacks only inspect the borrowed string.
    unsafe {
        crate::platform::inspect_environment_text(variable.c_name(), |value| {
            value.is_some_and(matches)
        })
    }
    .unwrap_or_else(|_| {
        pending::mark_json_allocation_failed();
        false
    })
}

pub(super) fn filter_matches(variable: LayerFilterVariable, name: &CStr) -> bool {
    filter_value_matches(variable, |filters| {
        filters.split(',').any(|filter| {
            filter.eq_ignore_ascii_case("~all~")
                || wildcard_matches(filter.as_bytes(), name.to_bytes())
        })
    })
}

pub(super) fn forced_enabled(manifest: &LayerManifest) -> bool {
    filter_matches(LayerFilterVariable::Enable, &manifest.name)
}

pub(super) fn forced_disabled(manifest: &LayerManifest) -> bool {
    let disabled = filter_value_matches(LayerFilterVariable::Disable, |filters| {
        filters.split(',').any(|filter| {
            filter.eq_ignore_ascii_case("~all~")
                || filter == "*"
                || filter == "**"
                || (manifest.implicit && filter.eq_ignore_ascii_case("~implicit~"))
                || (!manifest.implicit && filter.eq_ignore_ascii_case("~explicit~"))
                || wildcard_matches(filter.as_bytes(), manifest.name.to_bytes())
        })
    });
    disabled && !filter_matches(LayerFilterVariable::Allow, &manifest.name)
}

pub(super) fn naturally_enabled(manifest: &LayerManifest) -> bool {
    manifest.implicit
        && manifest
            .enable_environment
            .as_ref()
            .is_none_or(environment_matches)
}

pub(crate) fn implicit_manifest_is_active(manifest: &LayerManifest) -> bool {
    if !manifest.implicit {
        return false;
    }
    let natural = naturally_enabled(manifest);
    let filtered = if forced_enabled(manifest) {
        true
    } else {
        natural && !forced_disabled(manifest)
    };
    filtered
        && manifest
            .disable_environment
            .as_ref()
            .is_none_or(|environment| !environment_is_set(&environment.0))
}

pub(super) fn available_layer_mask(manifests: &[LayerManifest]) -> Result<Box<[bool]>, VkResult> {
    let mut available = valid_layer_mask(manifests)?;
    let blacklist = manifests
        .iter()
        .zip(available.iter())
        .find(|(manifest, valid)| {
            **valid && manifest.is_override() && implicit_manifest_is_active(manifest)
        })
        .map(|(manifest, _)| manifest.blacklisted_layers.as_ref());
    if let Some(blacklist) = blacklist {
        for (manifest, available) in manifests.iter().zip(available.iter_mut()) {
            if blacklist.iter().any(|name| name == &manifest.name) {
                *available = false;
            }
        }
    }
    Ok(available)
}

/// Reused only within one manifest snapshot; no discovery state is cached.
#[derive(Default)]
pub(super) struct MetaTraversal {
    visited: Vec<bool>,
    pending: Vec<usize>,
}

impl MetaTraversal {
    pub(super) fn reaches(
        &mut self,
        manifests: &[LayerManifest],
        from: usize,
        target: usize,
    ) -> Result<bool, VkResult> {
        if from == target {
            return Ok(true);
        }
        self.visited.clear();
        self.pending.clear();
        self.visited
            .try_reserve_exact(manifests.len())
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        self.pending
            .try_reserve_exact(manifests.len())
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        self.visited.resize(manifests.len(), false);
        self.visited[from] = true;
        self.pending.push(from);
        while let Some(index) = self.pending.pop() {
            if index == target {
                return Ok(true);
            }
            for name in manifests[index].component_layers() {
                if let Some(component) = name.index()
                    && !self.visited[component]
                {
                    // Each vertex is queued once, bounding growth by the
                    // reservation even when components contain duplicates.
                    self.visited[component] = true;
                    self.pending.push(component);
                }
            }
        }
        Ok(false)
    }
}

#[cold]
fn invalid_layer_names(create_info: &VkInstanceCreateInfo<'_>) -> VkResult {
    emit_create_message(
        create_info,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
        format_args!(
            "loader_validate_layers: ppEnabledLayerNames is NULL but enabledLayerCount is greater than zero"
        ),
    );
    VkResult::ERROR_LAYER_NOT_PRESENT
}

#[cold]
#[inline(never)]
pub(crate) fn select_active_layers(
    create_info: &VkInstanceCreateInfo<'_>,
    settings: Option<&LoaderSettings>,
) -> Result<SelectedLayers, VkResult> {
    let mut activation_messages = Vec::new();
    let manifests = discover_layers_with_settings(settings);
    if pending::take_json_allocation_failed() {
        return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
    }
    let compatibility_manifests = compatibility_manifest_graph(manifests.searches())?;
    emit_layer_search_diagnostics(create_info, manifests.searches(), &manifests);
    if pending::take_json_allocation_failed() {
        return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
    }
    if create_info.enabledLayerCount != 0 && create_info.ppEnabledLayerNames.is_null() {
        return Err(invalid_layer_names(create_info));
    }
    // SAFETY: Layer discovery follows upstream's exclusion of concurrent
    // environment mutation, including from allocation callbacks.
    let environment = unsafe { crate::platform::environment_value(c"VK_INSTANCE_LAYERS") }?;
    let (requested, environment_count) =
        requested_layer_names(create_info, environment.as_deref())?;
    record_environment_layers(
        &requested,
        environment_count,
        settings,
        &mut activation_messages,
    )?;
    record_forced_layers(&manifests, settings, &mut activation_messages)?;
    emit_meta_layer_diagnostics(
        create_info,
        compatibility_manifests.as_deref().unwrap_or(&manifests),
        &mut activation_messages,
    )?;
    emit_disabled_layers(create_info, &manifests);
    let valid = available_layer_mask(&manifests)?;
    emit_blacklisted_layers(create_info, &manifests, &requested, &valid);
    let settings_active = settings.is_some_and(LoaderSettings::has_layer_configurations);
    let mut selection = LayerSelection {
        selected: Vec::new(),
        reported: Vec::new(),
        activation_messages,
        activation_error_messages: Vec::new(),
    };
    selection.activate_implicit(
        &manifests,
        &valid,
        &requested,
        environment_count,
        settings_active,
    )?;
    if !settings_active {
        selection.activate_requested(
            create_info,
            &manifests,
            &valid,
            &requested,
            environment_count,
        )?;
    }
    validate_reported_layers(
        create_info,
        &requested[environment_count..],
        &selection.reported,
        settings,
    )?;
    let LayerSelection {
        selected,
        reported,
        activation_messages,
        activation_error_messages,
        ..
    } = selection;
    Ok(SelectedLayers {
        manifests: manifests.into_manifests(),
        selected,
        reported: allocation::try_into_boxed_slice(reported)?,
        requested,
        environment_count,
        activation_messages,
        activation_error_messages,
    })
}

pub(crate) fn emit_selected_layer_activation_diagnostics(
    create_info: &VkInstanceCreateInfo<'_>,
    selected: &SelectedLayers,
) {
    emit_layer_activation_messages(
        create_info,
        &selected.activation_messages,
        &selected.activation_error_messages,
    );
}

pub(super) fn emit_layer_activation_messages(
    create_info: &VkInstanceCreateInfo<'_>,
    messages: &[String],
    error_messages: &[String],
) {
    for _ in 0..2 {
        for message in messages {
            emit_layer_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!("{}", diagnostics::Text(message)),
            );
        }
        for message in error_messages {
            emit_layer_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                format_args!("{}", diagnostics::Text(message)),
            );
        }
    }
}

#[cold]
#[inline(never)]
pub(crate) fn load_selected_layers(
    create_info: &VkInstanceCreateInfo<'_>,
    selected_layers: SelectedLayers,
) -> Result<ActiveLayers, VkResult> {
    let SelectedLayers {
        mut manifests,
        selected,
        reported,
        requested,
        environment_count,
        ..
    } = selected_layers;
    let application_api_version = if create_info.pApplicationInfo.is_null() {
        vk::VK_API_VERSION_1_0
    } else {
        // SAFETY: The instance-create contract keeps application info readable.
        unsafe { (*create_info.pApplicationInfo).apiVersion }.max(vk::VK_API_VERSION_1_0)
    };
    emit_layer_api_versions(create_info, &selected, &manifests, application_api_version);
    let mut loaded = Vec::new();
    loaded
        .try_reserve_exact(selected.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut requested_layer_failed = false;
    let mut requested_failure_messages = Vec::new();

    // Upstream opens the expanded list from bottom to top while constructing
    // the GIPA chain. Reverse the finished storage so index zero remains the
    // application-facing (topmost) layer used by dispatch.
    for &index in selected.iter().rev() {
        let manifest = &manifests[index];
        if loaded
            .iter()
            .any(|layer: &LoadedLayer| layer.name == manifest.name)
        {
            continue;
        }
        let requested_by_application = requested[environment_count..]
            .iter()
            .any(|name| name.as_c_str() == manifest.name.as_c_str());
        let enabled_by_meta_layer = reported.iter().any(|active| {
            manifests.iter().any(|meta| {
                meta.name == active.name
                    && meta
                        .component_layers()
                        .iter()
                        .any(|component| component == &manifest.name)
            })
        });
        let enabled_by = if manifest.settings_control == Some(LayerControl::On) {
            LayerEnabledBy::Settings
        } else if enabled_by_meta_layer {
            LayerEnabledBy::MetaSettings
        } else if requested[..environment_count]
            .iter()
            .any(|name| name.as_c_str() == manifest.name.as_c_str())
        {
            LayerEnabledBy::InstanceEnvironment
        } else if manifest.implicit
            && (implicit_manifest_is_active(manifest) || !requested_by_application)
        {
            LayerEnabledBy::Implicit
        } else if forced_enabled(manifest) {
            LayerEnabledBy::LoaderEnvironment
        } else if requested_by_application {
            LayerEnabledBy::Application
        } else {
            LayerEnabledBy::MetaLayer
        };
        match LoadedLayer::load(&mut manifests[index], index, enabled_by) {
            Ok(layer) => {
                emit_loaded_layer(create_info, &layer)?;
                allocation::try_push(&mut loaded, layer)?;
            }
            Err(error) => {
                emit_layer_load_error(
                    create_info,
                    &manifests[index],
                    error,
                    requested_by_application,
                    &mut requested_layer_failed,
                    &mut requested_failure_messages,
                )?;
            }
        }
    }
    for message in requested_failure_messages {
        emit_layer_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
            format_args!("{}", diagnostics::Text(&message)),
        );
    }
    if requested_layer_failed {
        return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
    }
    loaded.reverse();
    Ok(ActiveLayers {
        loaded: allocation::try_into_boxed_slice(loaded)?,
        reported,
        requested,
    })
}

struct ActivationPath<'a> {
    name_index: usize,
    parent: Option<&'a ActivationPath<'a>>,
}

fn activate_manifest(
    index: usize,
    manifests: &[LayerManifest],
    selected: &mut Vec<usize>,
    reported: &mut Vec<ActiveLayerProperty>,
    parent: Option<&ActivationPath<'_>>,
    activation_messages: &mut Vec<String>,
) -> Result<bool, VkResult> {
    let manifest = &manifests[index];
    if reported.iter().any(|layer| {
        layer.name == manifest.name
            && (!manifest.component_layers().is_empty()
                || layer.manifest_path == manifest.manifest_path)
    }) {
        return Ok(true);
    }
    let mut ancestor = parent;
    while let Some(path) = ancestor {
        if path.name_index == manifest.name_index() {
            return Ok(false);
        }
        ancestor = path.parent;
    }
    let path = ActivationPath {
        name_index: manifest.name_index(),
        parent,
    };
    let result = if manifest.component_layers().is_empty() {
        selected
            .try_reserve(1)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        reported
            .try_reserve(1)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        let property = ActiveLayerProperty::try_new(manifest)?;
        selected.push(index);
        reported.push(property);
        true
    } else {
        let mut complete = true;
        for component_name in manifest.component_layers() {
            let Some(component_index) = component_name.index() else {
                complete = false;
                continue;
            };
            let component = &manifests[component_index];
            if forced_disabled(component) && !forced_enabled(component) {
                let message = diagnostics::try_format(format_args!(
                    "Failed to find layer name \"{}\" component layer \"{}\" to activate (Policy #LLP_LAYER_7)",
                    crate::debug::diagnostics::LossyBytes(component.name.to_bytes()),
                    crate::debug::diagnostics::LossyBytes(component.name.to_bytes())
                ))?;
                record_message(activation_messages, message)?;
                complete = false;
                continue;
            }
            if !activate_manifest(
                component_index,
                manifests,
                selected,
                reported,
                Some(&path),
                activation_messages,
            )? {
                complete = false;
            }
        }
        if complete {
            reported
                .try_reserve(1)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            reported.push(ActiveLayerProperty::try_new(manifest)?);
        }
        complete
    };
    Ok(result)
}

#[cold]
fn emit_loaded_layer(
    create_info: &VkInstanceCreateInfo<'_>,
    layer: &LoadedLayer,
) -> Result<(), VkResult> {
    emit_layer_message(
        create_info,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
        format_args!("Loading layer library {}", layer.load_path.loader_display()),
    );
    if layer.load_path.is_absolute()
        && layer.load_path != layer.library_path
        && !crate::platform::path_normalizes(&layer.load_path)?
    {
        emit_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
            format_args!(
                "normalize_path: Call to realpath() failed with error code 2 when given the path {}",
                layer.load_path.loader_display()
            ),
        );
        emit_layer_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!(
                "Path to given binary {} was found to differ from OS loaded path {}",
                layer.load_path.loader_display(),
                layer.library_path.loader_display()
            ),
        );
    }
    emit_layer_message(
        create_info,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        format_args!(
            "Insert instance layer \"{}\" ({})",
            crate::debug::diagnostics::LossyBytes(layer.name.to_bytes()),
            layer.library_path.loader_display()
        ),
    );
    Ok(())
}

#[cold]
fn emit_layer_load_error(
    create_info: &VkInstanceCreateInfo<'_>,
    manifest: &LayerManifest,
    error: LayerLoadError,
    explicitly_requested: bool,
    requested_layer_failed: &mut bool,
    requested_failure_messages: &mut Vec<String>,
) -> Result<(), VkResult> {
    let (reason, open_error) = match error {
        LayerLoadError::OpenLibrary {
            message,
            wrong_bit_type: true,
        } => ("was wrong bit-type", Some((message, true))),
        LayerLoadError::OpenLibrary {
            message,
            wrong_bit_type: false,
        } => ("failed to load", Some((message, false))),
        LayerLoadError::Failed => ("failed to load", None),
        LayerLoadError::OutOfMemory => return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY),
        LayerLoadError::Initialization(error) => return Err(error),
    };
    if let Some((message, wrong_bit_type)) = open_error {
        emit_create_message(
            create_info,
            if wrong_bit_type {
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO
            } else {
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR
            },
            format_args!("{}", diagnostics::Text(&message)),
        );
    }
    let severity = if explicitly_requested {
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR
    } else {
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO
    };
    let ending = if explicitly_requested { "!" } else { "." };
    let message = diagnostics::try_format(format_args!(
        "Requested layer \"{}\" {}{}",
        crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes()),
        diagnostics::Text(reason),
        diagnostics::Text(ending),
    ))?;
    if explicitly_requested {
        *requested_layer_failed = true;
        allocation::try_push(requested_failure_messages, message)?;
    } else {
        emit_layer_message(
            create_info,
            severity,
            format_args!("{}", diagnostics::Text(&message)),
        );
    }
    Ok(())
}

#[cold]
fn emit_layer_api_versions(
    create_info: &VkInstanceCreateInfo<'_>,
    selected: &[usize],
    manifests: &[LayerManifest],
    application_api_version: u32,
) {
    for &index in selected {
        let manifest = &manifests[index];
        if manifest.api_version < application_api_version {
            emit_layer_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "Layer {} uses API version {}.{} which is older than the application specified API version of {}.{}. May cause issues.",
                    crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes()),
                    vk::VK_API_VERSION_MAJOR(manifest.api_version),
                    vk::VK_API_VERSION_MINOR(manifest.api_version),
                    vk::VK_API_VERSION_MAJOR(application_api_version),
                    vk::VK_API_VERSION_MINOR(application_api_version),
                ),
            );
        }
    }
}

#[cold]
fn record_environment_layers(
    requested: &[CString],
    environment_count: usize,
    settings: Option<&LoaderSettings>,
    activation_messages: &mut Vec<String>,
) -> Result<(), VkResult> {
    if environment_count != 0 {
        let names = LayerNames(&requested[..environment_count]);
        let message = if settings.is_some() {
            diagnostics::try_format(format_args!(
                "env var 'VK_INSTANCE_LAYERS' defined and adding layers: {names}"
            ))?
        } else {
            diagnostics::try_format(format_args!(
                "env var 'VK_INSTANCE_LAYERS' defined and adding layers \"{names}\""
            ))?
        };
        record_message(activation_messages, message)?;
    }
    Ok(())
}

#[cold]
fn record_forced_layers(
    manifests: &[LayerManifest],
    settings: Option<&LoaderSettings>,
    activation_messages: &mut Vec<String>,
) -> Result<(), VkResult> {
    for manifest in manifests {
        let natural = naturally_enabled(manifest);
        let enabled = forced_enabled(manifest);
        let enabled_by_meta_layer = manifests.iter().any(|meta| {
            !meta.component_layers().is_empty()
                && (implicit_manifest_is_active(meta)
                    || meta.settings_control == Some(LayerControl::On))
                && meta
                    .component_layers()
                    .iter()
                    .any(|component| component == &manifest.name)
        });
        let environment_controlled = matches!(
            manifest.settings_control,
            None | Some(LayerControl::Default)
        );
        if environment_controlled && forced_disabled(manifest) && !forced_enabled(manifest) {
            if manifest.settings_control.is_some() {
                let message = diagnostics::try_format(format_args!(
                    "Layer \"{}\" forced disabled because name matches filter of env var 'VK_LOADER_LAYERS_DISABLE'.",
                    crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
                ))?;
                record_message(activation_messages, message)?;
            }
        } else if environment_controlled && enabled && !natural && !enabled_by_meta_layer {
            let kind = if manifest.implicit {
                "Implicit layer"
            } else {
                "Layer"
            };
            let suffix = if settings.is_some() || manifest.implicit {
                "."
            } else {
                ""
            };
            let message = diagnostics::try_format(format_args!(
                "{} \"{}\" forced enabled due to env var 'VK_LOADER_LAYERS_ENABLE'{}",
                diagnostics::Text(kind),
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes()),
                diagnostics::Text(suffix),
            ))?;
            record_message(activation_messages, message)?;
        } else if settings.is_some()
            && !manifest.implicit
            && manifest.settings_control != Some(LayerControl::On)
            && filter_value_matches(LayerFilterVariable::Disable, |filters| {
                filters
                    .split(',')
                    .any(|filter| filter.eq_ignore_ascii_case("~implicit~"))
            })
        {
            // Upstream applies the implicit-only disable filter while pruning
            // settings and unordered explicit layers from its implicit scan,
            // so these diagnostics are emitted even though the layers remain
            // eligible during the later explicit scan.
            let message = diagnostics::try_format(format_args!(
                "Layer \"{}\" forced disabled because name matches filter of env var 'VK_LOADER_LAYERS_DISABLE'.",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ))?;
            record_message(activation_messages, message)?;
        }
    }
    Ok(())
}

#[cold]
fn emit_disabled_layers(create_info: &VkInstanceCreateInfo<'_>, manifests: &[LayerManifest]) {
    for manifest in manifests.iter().filter(|manifest| {
        manifest.settings_control.is_none()
            && forced_disabled(manifest)
            && !forced_enabled(manifest)
    }) {
        if manifest.implicit && manifest.is_override() {
            emit_layer_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "Implicit layer \"{}\" forced disabled because name matches filter of env var 'VK_LOADER_LAYERS_DISABLE'.",
                    crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
                ),
            );
        }
        emit_layer_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!(
                "Layer \"{}\" forced disabled because name matches filter of env var 'VK_LOADER_LAYERS_DISABLE'.",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
}

#[cold]
fn emit_blacklisted_layers(
    create_info: &VkInstanceCreateInfo<'_>,
    manifests: &crate::discovery::DiscoveredLayers,
    requested: &[CString],
    valid: &[bool],
) {
    let override_layer = manifests
        .iter()
        .zip(valid.iter())
        .find_map(|(manifest, valid)| {
            (*valid && manifest.is_override() && implicit_manifest_is_active(manifest))
                .then_some(manifest)
        });
    if let Some(override_layer) = override_layer {
        for blacklisted in &override_layer.blacklisted_layers {
            if manifests
                .iter()
                .any(|manifest| manifest.name == *blacklisted)
                || requested.iter().any(|name| name == blacklisted)
                || manifests.searches().iter().any(|search| {
                    search.files.iter().any(|file| {
                        crate::discovery::reparse_layer_manifest(file, search.implicit)
                            .iter()
                            .any(|manifest| manifest.name == *blacklisted)
                    })
                })
            {
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                    format_args!(
                        "loader_remove_layers_in_blacklist: Override layer is active and layer {} is in the blacklist inside of it. Removing that layer from current layer list.",
                        crate::debug::diagnostics::LossyBytes(blacklisted.to_bytes())
                    ),
                );
            }
        }
    }
}

/// Owns the existing selection buffers while activation phases update them.
struct LayerSelection {
    selected: Vec<usize>,
    reported: Vec<ActiveLayerProperty>,
    activation_messages: Vec<String>,
    activation_error_messages: Vec<String>,
}

impl LayerSelection {
    #[inline]
    fn activate(&mut self, index: usize, manifests: &[LayerManifest]) -> Result<bool, VkResult> {
        activate_manifest(
            index,
            manifests,
            &mut self.selected,
            &mut self.reported,
            None,
            &mut self.activation_messages,
        )
    }

    fn activate_implicit(
        &mut self,
        manifests: &[LayerManifest],
        valid: &[bool],
        requested: &[CString],
        environment_count: usize,
        settings_active: bool,
    ) -> Result<(), VkResult> {
        if settings_active {
            for (index, (manifest, valid)) in manifests.iter().zip(valid.iter()).enumerate() {
                if !valid {
                    continue;
                }
                let requested_by_name = requested
                    .iter()
                    .any(|name| name.as_c_str() == manifest.name.as_c_str());
                let active = match manifest.settings_control {
                    Some(LayerControl::On) => true,
                    Some(LayerControl::Off) => false,
                    _ => {
                        implicit_manifest_is_active(manifest)
                            || requested_by_name
                            || forced_enabled(manifest)
                    }
                };
                if active {
                    let _ = self.activate(index, manifests)?;
                }
            }
        }
        for (index, (manifest, valid)) in manifests.iter().zip(valid.iter()).enumerate() {
            if settings_active
                || !valid
                || !implicit_manifest_is_active(manifest)
                || requested[..environment_count]
                    .iter()
                    .any(|name| name == &manifest.name)
            {
                continue;
            }
            let _ = self.activate(index, manifests)?;
        }
        for (index, (manifest, valid)) in manifests.iter().zip(valid.iter()).enumerate() {
            if settings_active
                || !valid
                || !forced_enabled(manifest)
                || (manifest.implicit
                    && manifest
                        .disable_environment
                        .as_ref()
                        .is_some_and(|environment| environment_is_set(&environment.0)))
            {
                continue;
            }
            let _ = self.activate(index, manifests)?;
        }
        Ok(())
    }

    fn activate_requested(
        &mut self,
        create_info: &VkInstanceCreateInfo<'_>,
        manifests: &[LayerManifest],
        valid: &[bool],
        requested: &[CString],
        environment_count: usize,
    ) -> Result<(), VkResult> {
        for (requested_index, requested_name) in requested.iter().enumerate() {
            let Some((index, (manifest, _))) =
                manifests
                    .iter()
                    .zip(valid.iter())
                    .enumerate()
                    .find(|(_, (manifest, valid))| {
                        **valid && manifest.name.as_c_str() == requested_name.as_c_str()
                    })
            else {
                if requested_index < environment_count {
                    let message = diagnostics::try_format(format_args!(
                        "Layer \"{}\" was not found but was requested by env var VK_INSTANCE_LAYERS!",
                        crate::debug::diagnostics::LossyBytes(requested_name.to_bytes())
                    ))?;
                    record_message(&mut self.activation_error_messages, message)?;
                    continue;
                }
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    format_args!(
                        "loader_validate_layers: Layer {requested_index} does not exist in the list of available layers"
                    ),
                );
                emit_layer_activation_messages(
                    create_info,
                    &self.activation_messages,
                    &self.activation_error_messages,
                );
                return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
            };
            if forced_disabled(manifest) && !forced_enabled(manifest) {
                if requested_index < environment_count {
                    continue;
                }
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    format_args!(
                        "loader_validate_layers: Layer {} does not exist in the list of available layers",
                        requested_index - environment_count
                    ),
                );
                return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
            }
            if !self.activate(index, manifests)? {
                if requested_index < environment_count {
                    continue;
                }
                emit_layer_activation_messages(
                    create_info,
                    &self.activation_messages,
                    &self.activation_error_messages,
                );
                return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
            }
        }
        Ok(())
    }
}

#[cold]
fn validate_reported_layers(
    create_info: &VkInstanceCreateInfo<'_>,
    requested: &[CString],
    reported: &[ActiveLayerProperty],
    settings: Option<&LoaderSettings>,
) -> Result<(), VkResult> {
    for (requested_index, requested_name) in requested.iter().enumerate() {
        if reported
            .iter()
            .any(|layer| layer.name.as_c_str() == requested_name.as_c_str())
        {
            continue;
        }
        let reason = if settings.is_some_and(|settings| {
            settings.layer_control(requested_name) == Some(LayerControl::Off)
        }) {
            "was explicitly prevented from being enabled by the loader settings file"
        } else {
            "does not exist in the list of available layers"
        };
        emit_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
            format_args!(
                "loader_validate_layers: Layer {requested_index} {}",
                diagnostics::Text(reason)
            ),
        );
        return Err(VkResult::ERROR_LAYER_NOT_PRESENT);
    }
    Ok(())
}
