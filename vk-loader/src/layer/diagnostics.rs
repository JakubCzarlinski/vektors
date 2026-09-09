//! Layer discovery and activation diagnostics.

use core::fmt::Write as _;

use crate::{
    allocation,
    debug::diagnostics,
    discovery::{self, DiscoveredLayers, LayerManifestDiagnostic},
    pending,
    platform::{self, LogFilter},
};

use super::{
    CString, LayerManifest, LayerSearch, LoadedLayer, MetaTraversal, Path, VkInstanceCreateInfo,
    VkResult, forced_disabled, forced_enabled, implicit_manifest_is_active, naturally_enabled,
    valid_layer_mask,
};

#[cold]
#[inline(never)]
pub(super) fn emit_create_message(
    create_info: &VkInstanceCreateInfo<'_>,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: core::fmt::Arguments<'_>,
) {
    let filter = LogFilter::from_severity(severity);
    let mut text = diagnostics::LogBuffer::<511>::new();
    let _ = text.write_fmt(message);
    let message = text.as_str();
    platform::write_loader_log(filter, format_args!("{message}"));
    submit_create_message(create_info, severity, message);
}

#[cold]
#[inline(never)]
pub(super) fn emit_layer_only_message(
    create_info: &VkInstanceCreateInfo<'_>,
    message: core::fmt::Arguments<'_>,
) {
    diagnostics::with_text(message, |message| {
        platform::write_loader_category_log(LogFilter::Layer, format_args!("{message}"));
        // Category-only loader messages map to informational debug-utils messages.
        submit_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            message,
        );
    });
}

#[cold]
#[inline(never)]
pub(super) fn emit_layer_message(
    create_info: &VkInstanceCreateInfo<'_>,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: core::fmt::Arguments<'_>,
) {
    let mut text = diagnostics::LogBuffer::<511>::new();
    let _ = text.write_fmt(message);
    let message = text.as_str();
    let filter = LogFilter::from_severity(severity);
    platform::write_loader_log_with_category(filter, LogFilter::Layer, format_args!("{message}"));
    submit_create_message(create_info, severity, message);
}

#[cold]
#[inline(never)]
pub(super) fn submit_create_message(
    create_info: &VkInstanceCreateInfo<'_>,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: &str,
) {
    diagnostics::with_message(format_args!("{message}"), |message| {
        // SAFETY: The create-info chain and formatted message remain live for
        // the synchronous callback invocation during layer activation.
        unsafe {
            crate::debug::messenger::submit_instance_create_message(create_info, severity, message);
        }
    });
}

pub(super) struct ManifestVersion {
    text: Option<String>,
    version: u32,
}

impl core::fmt::Display for ManifestVersion {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.text {
            Some(text) => formatter.write_str(text),
            None => write!(
                formatter,
                "{}.{}.{}",
                vk::VK_API_VERSION_MAJOR(self.version),
                vk::VK_API_VERSION_MINOR(self.version),
                vk::VK_API_VERSION_PATCH(self.version)
            ),
        }
    }
}

pub(super) fn record_duplicate_layer(
    duplicate: &LayerManifest,
    original: &LayerManifest,
    manifests: &[LayerManifest],
    summaries: &mut Vec<(CString, Box<[CString]>)>,
    messages: &mut Vec<String>,
) -> Result<(), VkResult> {
    if duplicate.is_meta_layer()
        && duplicate
            .component_layers()
            .iter()
            .all(|component| manifests.iter().any(|manifest| manifest.name == *component))
    {
        let name = allocation::try_c_string(&duplicate.name)?;
        let components = allocation::try_collect_results(
            duplicate
                .component_layers()
                .iter()
                .map(|name| allocation::try_c_string(name)),
        )?;
        allocation::try_push(summaries, (name, components))?;
    }
    let message = diagnostics::try_format(format_args!(
        "Removing layer {} ({}) because it is a duplicate of {} ({})",
        crate::debug::diagnostics::LossyBytes(duplicate.name.to_bytes()),
        duplicate.manifest_path.display(),
        crate::debug::diagnostics::LossyBytes(original.name.to_bytes()),
        original.manifest_path.display(),
    ))?;
    allocation::try_push(messages, message)
}

pub(super) fn emit_layer_search_diagnostics(
    create_info: &VkInstanceCreateInfo<'_>,
    searches: &[LayerSearch],
    manifests: &[LayerManifest],
) {
    if emit_search_diagnostics(MetaDiagnosticSink::Create(create_info), searches, manifests)
        .is_err()
    {
        pending::mark_json_allocation_failed();
    }
}

#[cold]
#[inline(never)]
fn emit_search_diagnostics(
    sink: MetaDiagnosticSink<'_>,
    searches: &[LayerSearch],
    manifests: &[LayerManifest],
) -> Result<(), VkResult> {
    let mut duplicate_messages = Vec::new();
    let mut duplicate_meta_summaries = Vec::new();
    for search in searches {
        emit_layer_search_locations(sink, search);
        for file in &search.files {
            emit_search_file(
                sink,
                search,
                file,
                manifests,
                &mut duplicate_meta_summaries,
                &mut duplicate_messages,
            )?;
        }
        if search.implicit
            && let Some(override_layer) = manifests.iter().find(|manifest| {
                manifest.is_override()
                    && naturally_enabled(manifest)
                    && manifest
                        .disable_environment
                        .as_ref()
                        .is_none_or(|environment| {
                            !super::activation::environment_is_set(&environment.0)
                        })
            })
        {
            emit_override_layer_diagnostics(sink, override_layer);
            if forced_disabled(override_layer) && !forced_enabled(override_layer) {
                sink.layer_message(
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "Implicit layer \"{}\" forced disabled because name matches filter of env var 'VK_LOADER_LAYERS_DISABLE'.",
                        crate::debug::diagnostics::LossyBytes(override_layer.name.to_bytes())
                    ),
                );
            }
        }
    }
    for (name, components) in duplicate_meta_summaries {
        sink.layer_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!(
                "Meta-layer \"{}\" all {} component layers appear to be valid.",
                crate::debug::diagnostics::LossyBytes(name.to_bytes()),
                components.len()
            ),
        );
        for (index, component) in components.iter().enumerate() {
            sink.layer_only(format_args!(
                "  [{index}] {}",
                crate::debug::diagnostics::LossyBytes(component.to_bytes())
            ));
        }
    }
    for message in duplicate_messages {
        sink.message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!("{message}"),
        );
    }
    Ok(())
}

pub(super) fn emit_global_discovered_manifest(manifest: &LayerManifest, emit_found: bool) {
    emit_discovered_manifest_version(MetaDiagnosticSink::Global, manifest, emit_found);
    if !manifest.name.to_bytes().starts_with(b"VK_LAYER_") {
        platform::write_loader_log(
            LogFilter::Warning,
            format_args!(
                "Layer name {} does not conform to naming standard (Policy #LLP_LAYER_3)",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
    if !manifest.implicit && manifest.has_pre_instance_functions {
        platform::write_loader_log(
            LogFilter::Warning,
            format_args!(
                "Found pre_instance_functions section in explicit layer from \"{}\". This section is only valid in implicit layers. The section will be ignored",
                manifest.manifest_path.display()
            ),
        );
    }
    if manifest.is_meta_layer() && manifest.manifest_version < vk::VK_MAKE_API_VERSION(0, 1, 1, 0) {
        platform::write_loader_log(
            LogFilter::Warning,
            format_args!(
                "Layer \"{}\" contains meta-layer-specific component_layers, but using older JSON file version.",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
    if manifest.is_meta_layer() {
        platform::write_loader_log_with_category(
            LogFilter::Info,
            LogFilter::Layer,
            format_args!(
                "Encountered meta-layer \"{}\"",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
    if !manifest.is_override() && manifest.app_keys.is_some() {
        platform::write_loader_log_with_category(
            LogFilter::Warning,
            LogFilter::Layer,
            format_args!(
                "Layer {} contains app_keys, but any app_keys can only be provided by the override meta layer. These will be ignored.",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
    if !manifest.override_paths.is_empty()
        && manifest.manifest_version < vk::VK_MAKE_API_VERSION(0, 1, 1, 0)
    {
        platform::write_loader_log(
            LogFilter::Warning,
            format_args!(
                "Layer \"{}\" contains meta-layer-specific override paths, but using older JSON file version.",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
}

/// Emits pre-instance layer discovery diagnostics when no instance-create
/// callback chain exists yet.
pub(crate) fn emit_global_layer_search_diagnostics(
    discovered: &DiscoveredLayers,
    emit_implicit_meta_pruning: bool,
) {
    if !platform::loader_debug_logging_enabled() {
        return;
    }
    let searches = discovered.searches();
    let manifests: &[LayerManifest] = discovered;
    let configured_manifest_reports = discovered.configured_manifest_reports();
    let implicit_only = discovered.implicit_only();
    let Ok(compatibility_manifests) = compatibility_manifest_graph(searches) else {
        pending::mark_json_allocation_failed();
        return;
    };
    emit_configured_manifest_reports(configured_manifest_reports, manifests);
    if emit_search_diagnostics(MetaDiagnosticSink::Global, searches, manifests).is_err() {
        pending::mark_json_allocation_failed();
        return;
    }
    if emit_recursive_meta_layer_diagnostics(
        MetaDiagnosticSink::Global,
        compatibility_manifests.as_deref().unwrap_or(manifests),
    )
    .is_err()
    {
        pending::mark_json_allocation_failed();
        return;
    }
    emit_disabled_global_layers(manifests, implicit_only);
    emit_pruned_implicit_layers(
        searches,
        manifests,
        implicit_only,
        emit_implicit_meta_pruning,
    );
}

pub(crate) fn emit_global_layer_manifest_diagnostic(
    path: &Path,
    implicit: bool,
    emit_found: bool,
    source_index: Option<usize>,
) {
    emit_manifest_diagnostics(
        MetaDiagnosticSink::Global,
        path,
        implicit,
        emit_found,
        source_index,
    );
}

pub(crate) fn emit_instance_layer_callstack(
    create_info: &VkInstanceCreateInfo<'_>,
    layers: &[LoadedLayer],
) {
    emit_layer_only_message(
        create_info,
        format_args!("vkCreateInstance layer callstack setup to:"),
    );
    emit_layer_only_message(create_info, format_args!("   <Application>"));
    emit_layer_only_message(create_info, format_args!("     ||"));
    emit_layer_only_message(create_info, format_args!("   <Loader>"));
    emit_layer_only_message(create_info, format_args!("     ||"));
    for layer in layers {
        emit_layer_only_message(
            create_info,
            format_args!(
                "   {}",
                crate::debug::diagnostics::LossyBytes(layer.name.to_bytes())
            ),
        );
        emit_layer_only_message(
            create_info,
            format_args!(
                "           Type: {}",
                if layer.implicit {
                    "Implicit"
                } else {
                    "Explicit"
                }
            ),
        );
        emit_layer_only_message(
            create_info,
            format_args!("           Enabled By: {}", layer.enabled_by),
        );
        if layer.implicit
            && let Some(disable_environment) = &layer.disable_environment
        {
            emit_layer_only_message(
                create_info,
                format_args!(
                    "               Disable Env Var:  {}",
                    std::path::Path::new(disable_environment).display()
                ),
            );
        }
        if let Some((name, value)) = layer.enable_environment() {
            emit_layer_only_message(
                create_info,
                format_args!(
                    "               This layer was enabled because Env Var {} was set to Value {}",
                    std::path::Path::new(name).display(),
                    std::path::Path::new(value).display()
                ),
            );
        }
        emit_layer_only_message(
            create_info,
            format_args!("           Manifest: {}", layer.manifest_path.display()),
        );
        emit_layer_only_message(
            create_info,
            format_args!("           Library:  {}", layer.library_path.display()),
        );
        emit_layer_only_message(create_info, format_args!("     ||"));
    }
    emit_layer_only_message(create_info, format_args!("   <Drivers>"));
}

#[derive(Clone, Copy)]
pub(super) enum MetaDiagnosticSink<'a> {
    Global,
    Create(&'a VkInstanceCreateInfo<'a>),
}

struct MetaDiagnosticState {
    available: Box<[bool]>,
    checked: Box<[bool]>,
}

impl MetaDiagnosticState {
    fn new(count: usize) -> Result<Self, VkResult> {
        Ok(Self {
            available: allocation::try_boxed_slice_filled(count, true)?,
            checked: allocation::try_boxed_slice_filled(count, false)?,
        })
    }
}

impl MetaDiagnosticSink<'_> {
    #[cold]
    #[inline(never)]
    fn message(
        self,
        severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
        message: core::fmt::Arguments<'_>,
    ) {
        match self {
            Self::Global => {
                platform::write_loader_log(LogFilter::from_severity(severity), message);
            }
            Self::Create(create_info) => emit_create_message(create_info, severity, message),
        }
    }

    #[cold]
    #[inline(never)]
    fn layer_message(
        self,
        severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
        message: core::fmt::Arguments<'_>,
    ) {
        match self {
            Self::Global => platform::write_loader_log_with_category(
                LogFilter::from_severity(severity),
                LogFilter::Layer,
                message,
            ),
            Self::Create(create_info) => emit_layer_message(create_info, severity, message),
        }
    }

    #[cold]
    #[inline(never)]
    fn layer_only(self, message: core::fmt::Arguments<'_>) {
        match self {
            Self::Global => {
                platform::write_loader_category_log(LogFilter::Layer, message);
            }
            Self::Create(create_info) => emit_layer_only_message(create_info, message),
        }
    }
}

pub(super) fn emit_override_layer_diagnostics(
    sink: MetaDiagnosticSink<'_>,
    override_layer: &LayerManifest,
) {
    if override_layer.app_keys().is_empty() {
        sink.layer_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!("Using the global override layer"),
        );
    } else if let Some(executable) = platform::executable_path() {
        sink.layer_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!(
                "Using the override layer for app key {}",
                executable.display()
            ),
        );
    }
    if override_layer.override_paths.is_empty() {
        return;
    }
    for path in &override_layer.override_paths {
        sink.layer_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!("Override layer has override path {}", path.display()),
        );
    }
    if !platform::has_elevated_privileges() {
        // SAFETY: Discovery and its diagnostic callbacks must not mutate the
        // environment while upstream-compatible borrowed getenv reads are live.
        let result = unsafe {
            platform::inspect_environment_text(c"VK_LAYER_PATH", |layer_path| {
                if let Some(layer_path) = layer_path {
                    sink.layer_message(
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                        format_args!(
                            "Ignoring VK_LAYER_PATH. The Override layer is active and has override paths set, which takes priority. VK_LAYER_PATH is set to {layer_path}"
                        ),
                    );
                }
            })
        };
        if result.is_err() {
            pending::mark_json_allocation_failed();
        }
    }
    for path in override_layer
        .override_paths
        .iter()
        .filter(|path| !path.as_os_str().is_empty())
    {
        sink.layer_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!("{}", path.display()),
        );
    }
}

pub(super) fn verify_meta_layer_for_diagnostics(
    sink: MetaDiagnosticSink<'_>,
    manifests: &[LayerManifest],
    index: usize,
    available: &mut [bool],
    checked: &mut [bool],
) -> bool {
    let meta = &manifests[index];
    checked[index] = true;
    for (component_index, component_name) in meta.component_layers().iter().enumerate() {
        let Some(component_index_in_manifests) =
            manifests.iter().enumerate().position(|(index, candidate)| {
                available[index] && Some(candidate.name_index()) == component_name.index()
            })
        else {
            sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "verify_meta_layer_component_layers: Meta-layer {} can't find component layer {} at index {}.  Skipping this layer.",
                    crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
                    crate::debug::diagnostics::LossyBytes(component_name.to_bytes()),
                    component_index
                ),
            );
            return false;
        };
        let component = &manifests[component_index_in_manifests];
        let meta_major = vk::VK_API_VERSION_MAJOR(meta.api_version);
        let meta_minor = vk::VK_API_VERSION_MINOR(meta.api_version);
        let component_major = vk::VK_API_VERSION_MAJOR(component.api_version);
        let component_minor = vk::VK_API_VERSION_MINOR(component.api_version);
        if component_major < meta_major
            || (component_major == meta_major && component_minor < meta_minor)
        {
            sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "verify_meta_layer_component_layers: Meta-layer uses API version {meta_major}.{meta_minor}, but component layer {component_index} has API version {component_major}.{component_minor} that is lower.  Skipping this layer."
                ),
            );
            return false;
        }
        if Some(meta.name_index()) == component_name.index() {
            sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "verify_meta_layer_component_layers: Meta-layer {} lists itself in its component layer list at index {}.  Skipping this layer.",
                    crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
                    component_index
                ),
            );
            return false;
        }
        if !component.component_layers().is_empty() {
            let recursive_index = manifests
                .iter()
                .enumerate()
                .rev()
                .find(|(index, candidate)| {
                    available[*index] && candidate.name_index() == component.name_index()
                })
                .map_or(component_index_in_manifests, |(index, _)| index);
            if checked[recursive_index] {
                sink.message(
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "verify_meta_layer_component_layers: Recursive dependency between Meta-layer {} and  Meta-layer {}.  Skipping this layer.",
                        crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
                        crate::debug::diagnostics::LossyBytes(component.name.to_bytes())
                    ),
                );
                return false;
            }
            sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format_args!(
                    "verify_meta_layer_component_layers: Adding meta-layer {} which also contains meta-layer {}",
                    crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
                    crate::debug::diagnostics::LossyBytes(component.name.to_bytes())
                ),
            );
            if !verify_meta_layer_for_diagnostics(
                sink,
                manifests,
                recursive_index,
                available,
                checked,
            ) {
                sink.message(
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "Meta-layer {} component layer {} can not find all component layers.  Skipping this layer.",
                        crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
                        crate::debug::diagnostics::LossyBytes(component.name.to_bytes())
                    ),
                );
                return false;
            }
        }
    }
    emit_valid_meta_layer(sink, manifests, meta);
    true
}

pub(super) fn emit_recursive_meta_layer_diagnostics(
    sink: MetaDiagnosticSink<'_>,
    manifests: &[LayerManifest],
) -> Result<(), VkResult> {
    if !manifests.iter().any(LayerManifest::is_meta_layer) {
        return Ok(());
    }
    crate::discovery::resolve_layer_names(manifests);
    let mut state = MetaDiagnosticState::new(manifests.len())?;
    for index in 0..manifests.len() {
        if !manifests[index].is_meta_layer() {
            continue;
        }
        state.checked.fill(false);
        if !verify_meta_layer_for_diagnostics(
            sink,
            manifests,
            index,
            &mut state.available,
            &mut state.checked,
        ) {
            state.available[index] = false;
            sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                format_args!(
                    "Removing meta-layer {} from instance layer list since it appears invalid.",
                    crate::debug::diagnostics::LossyBytes(manifests[index].name.to_bytes())
                ),
            );
        }
    }
    Ok(())
}

pub(super) fn compatibility_manifest_graph(
    searches: &[LayerSearch],
) -> Result<Option<Box<[LayerManifest]>>, VkResult> {
    if !searches
        .iter()
        .any(|search| !search.diagnostic_files.is_empty())
    {
        return Ok(None);
    }
    allocation::try_collect(searches.iter().flat_map(|search| {
        search
            .files
            .iter()
            .flat_map(|file| discovery::reparse_layer_manifest(file, search.implicit).into_vec())
    }))
    .map(Some)
}

pub(super) fn emit_meta_layer_diagnostics(
    create_info: &VkInstanceCreateInfo<'_>,
    manifests: &[LayerManifest],
    activation_messages: &mut Vec<String>,
    repeated_activation_messages: &mut Vec<String>,
) -> Result<(), VkResult> {
    let sink = MetaDiagnosticSink::Create(create_info);
    if manifests
        .iter()
        .filter(|manifest| manifest.is_meta_layer())
        .all(|manifest| manifest.settings_control.is_none())
    {
        return emit_recursive_meta_layer_diagnostics(sink, manifests);
    }
    let valid = valid_layer_mask(manifests)?;
    let mut traversal = MetaTraversal::default();
    for (meta_index, meta) in manifests.iter().enumerate() {
        if !meta.is_meta_layer() {
            continue;
        }
        let mut configured_recursive = false;
        for (component_index, component_name) in meta.component_layers().iter().enumerate() {
            let Some(component_manifest_index) = component_name.index() else {
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "verify_meta_layer_component_layers: Meta-layer {} can't find component layer {} at index {}.  Skipping this layer.",
                        crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
                        crate::debug::diagnostics::LossyBytes(component_name.to_bytes()),
                        component_index
                    ),
                );
                break;
            };
            if component_manifest_index == meta_index {
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "verify_meta_layer_component_layers: Meta-layer {} lists itself in its component layer list at index {}.  Skipping this layer.",
                        crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
                        component_index
                    ),
                );
                break;
            }
            let component = &manifests[component_manifest_index];
            let meta_major = vk::VK_API_VERSION_MAJOR(meta.api_version);
            let meta_minor = vk::VK_API_VERSION_MINOR(meta.api_version);
            let component_major = vk::VK_API_VERSION_MAJOR(component.api_version);
            let component_minor = vk::VK_API_VERSION_MINOR(component.api_version);
            if component_major < meta_major
                || (component_major == meta_major && component_minor < meta_minor)
            {
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "verify_meta_layer_component_layers: Meta-layer uses API version {meta_major}.{meta_minor}, but component layer {component_index} has API version {component_major}.{component_minor} that is lower.  Skipping this layer."
                    ),
                );
                break;
            }
            if !component.component_layers().is_empty() {
                if traversal.reaches(manifests, component_manifest_index, meta_index)? {
                    configured_recursive = emit_recursive_meta_reference(
                        create_info,
                        meta,
                        component,
                        activation_messages,
                        repeated_activation_messages,
                    )?;
                    break;
                }
                emit_create_message(
                    create_info,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format_args!(
                        "verify_meta_layer_component_layers: Adding meta-layer {} which also contains meta-layer {}",
                        crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
                        crate::debug::diagnostics::LossyBytes(component.name.to_bytes())
                    ),
                );
                if valid[component_manifest_index] {
                    emit_meta_components(sink, component);
                }
            }
        }
        if valid[meta_index] {
            emit_valid_meta_layer(sink, manifests, meta);
        } else if !configured_recursive {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                format_args!(
                    "Removing meta-layer {} from instance layer list since it appears invalid.",
                    crate::debug::diagnostics::LossyBytes(meta.name.to_bytes())
                ),
            );
        }
    }
    Ok(())
}

#[cold]
fn emit_manifest_diagnostics(
    sink: MetaDiagnosticSink<'_>,
    path: &Path,
    implicit: bool,
    emit_found: bool,
    source_index: Option<usize>,
) {
    for (diagnostic_index, (_, diagnostic)) in discovery::layer_manifest_diagnostics(path, implicit)
        .into_iter()
        .filter(|(index, _)| source_index.is_none_or(|source_index| *index == source_index))
        .enumerate()
    {
        emit_manifest_found(sink, path, &diagnostic, emit_found, diagnostic_index);
        match diagnostic {
            LayerManifestDiagnostic::FailedOpen => sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                format_args!(
                    "loader_get_json: Failed to open JSON file {}",
                    path.display()
                ),
            ),
            LayerManifestDiagnostic::InvalidJson => sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                format_args!("loader_get_json: Invalid JSON file {}.", path.display()),
            ),
            LayerManifestDiagnostic::MissingFileFormatVersion => {
                sink.layer_message(
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "loader_add_layer_properties: Manifest {} missing required field file_format_version",
                        path.display()
                    ),
                );
            }
            LayerManifestDiagnostic::MissingLayers { parsed_version, .. } => {
                sink.layer_message(
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format_args!(
                        "loader_add_layer_properties: {} has unknown layer manifest file version {}.{}.{}.  May cause errors.",
                        path.display(),
                        vk::VK_API_VERSION_MAJOR(parsed_version),
                        vk::VK_API_VERSION_MINOR(parsed_version),
                        vk::VK_API_VERSION_PATCH(parsed_version),
                    ),
                );
            }
            LayerManifestDiagnostic::UnknownManifestVersion { parsed_version, .. } => {
                if emit_found {
                    sink.layer_message(
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                        format_args!(
                            "loader_add_layer_properties: {} has unknown layer manifest file version {}.{}.{}.  May cause errors.",
                            path.display(),
                            vk::VK_API_VERSION_MAJOR(parsed_version),
                            vk::VK_API_VERSION_MINOR(parsed_version),
                            vk::VK_API_VERSION_PATCH(parsed_version),
                        ),
                    );
                }
            }
            LayerManifestDiagnostic::UnsupportedLayersArray { version, .. } => {
                sink.layer_message(
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "loader_add_layer_properties: 'layers' tag not supported until file version 1.0.1, but {} is reporting version {version}",
                        path.display()
                    ),
                );
            }
            diagnostic => emit_layer_field_diagnostic(sink, path, diagnostic),
        }
    }
}

#[cold]
fn emit_manifest_found(
    sink: MetaDiagnosticSink<'_>,
    path: &Path,
    diagnostic: &LayerManifestDiagnostic,
    emit_found: bool,
    index: usize,
) {
    if !emit_found {
        return;
    }
    match diagnostic {
        LayerManifestDiagnostic::UnknownManifestVersion { version, .. } => {
            emit_found_manifest_version(sink, path, version);
        }
        _ if index != 0 => {}
        LayerManifestDiagnostic::MissingLayers { version, .. } => {
            emit_found_manifest_version(sink, path, version);
        }
        LayerManifestDiagnostic::UnsupportedLayersArray { found_version, .. } => {
            emit_found_manifest_version(sink, path, found_version);
        }
        LayerManifestDiagnostic::MissingRequiredValue {
            manifest_version, ..
        } => {
            emit_found_manifest_version(
                sink,
                path,
                &ManifestVersion {
                    text: discovery::layer_manifest_version_text(path),
                    version: *manifest_version,
                },
            );
        }
        LayerManifestDiagnostic::NonConformingName {
            manifest_version, ..
        }
        | LayerManifestDiagnostic::MissingDisableEnvironment {
            manifest_version, ..
        }
        | LayerManifestDiagnostic::InvalidDisableEnvironment {
            manifest_version, ..
        }
        | LayerManifestDiagnostic::InvalidLibraryAndComponents {
            manifest_version, ..
        } => {
            emit_found_manifest_version(
                sink,
                path,
                &ManifestVersion {
                    text: None,
                    version: *manifest_version,
                },
            );
        }
        _ => {}
    }
}

#[cold]
fn emit_found_manifest_version(
    sink: MetaDiagnosticSink<'_>,
    path: &Path,
    version: impl core::fmt::Display,
) {
    sink.message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        format_args!(
            "Found manifest file {} (file version {version})",
            path.display()
        ),
    );
}

#[cold]
fn emit_valid_meta_layer(
    sink: MetaDiagnosticSink<'_>,
    manifests: &[LayerManifest],
    meta: &LayerManifest,
) {
    emit_meta_components(sink, meta);
    for component_name in meta.component_layers() {
        let Some(component) = component_name.index().map(|index| &manifests[index]) else {
            continue;
        };
        for extension in &component.instance_extensions {
            sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                format_args!(
                    "Meta-layer {} component layer {} adding instance extension {}",
                    crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
                    crate::debug::diagnostics::LossyBytes(component.name.to_bytes()),
                    crate::debug::diagnostics::LossyBytes(extension.name.to_bytes())
                ),
            );
        }
        for extension in &component.device_extensions {
            sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                format_args!(
                    "Meta-layer {} component layer {} adding device extension {}",
                    crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
                    crate::debug::diagnostics::LossyBytes(component.name.to_bytes()),
                    crate::debug::diagnostics::LossyBytes(extension.name.to_bytes())
                ),
            );
        }
    }
}

#[cold]
fn emit_create_discovered_manifest(
    create_info: &VkInstanceCreateInfo<'_>,
    manifest: &LayerManifest,
    emit_found: bool,
) {
    emit_discovered_manifest_version(
        MetaDiagnosticSink::Create(create_info),
        manifest,
        emit_found,
    );
    if !manifest.name.to_bytes().starts_with(b"VK_LAYER_") {
        emit_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!(
                "Layer name {} does not conform to naming standard (Policy #LLP_LAYER_3)",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
    if !manifest.implicit && manifest.has_pre_instance_functions {
        emit_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!(
                "Found pre_instance_functions section in explicit layer from \"{}\". This section is only valid in implicit layers. The section will be ignored",
                manifest.manifest_path.display()
            ),
        );
    }
    let variant = vk::VK_API_VERSION_VARIANT(manifest.api_version);
    if variant != 0 {
        emit_layer_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!(
                "Layer \"{}\" has an 'api_version' field which contains a non-zero variant value of {variant}.  Skipping Layer.",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes()),
            ),
        );
    }
    if !manifest.architecture_supported {
        emit_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!(
                "The library architecture in layer {} doesn't match the current running architecture, skipping this layer",
                manifest.manifest_path.display(),
            ),
        );
    }
    if manifest.is_meta_layer() && manifest.manifest_version < vk::VK_MAKE_API_VERSION(0, 1, 1, 0) {
        emit_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!(
                "Layer \"{}\" contains meta-layer-specific component_layers, but using older JSON file version.",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
    if manifest.is_meta_layer() {
        emit_layer_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!(
                "Encountered meta-layer \"{}\"",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
    if !manifest.is_override() && manifest.app_keys.is_some() {
        emit_layer_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!(
                "Layer {} contains app_keys, but any app_keys can only be provided by the override meta layer. These will be ignored.",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
    if !manifest.override_paths.is_empty()
        && manifest.manifest_version < vk::VK_MAKE_API_VERSION(0, 1, 1, 0)
    {
        emit_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!(
                "Layer \"{}\" contains meta-layer-specific override paths, but using older JSON file version.",
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
}

#[cold]
fn emit_duplicate_manifest(create_info: &VkInstanceCreateInfo<'_>, duplicate: &LayerManifest) {
    emit_create_message(
        create_info,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        format_args!(
            "Found manifest file {} (file version {}.{}.{})",
            duplicate.manifest_path.display(),
            vk::VK_API_VERSION_MAJOR(duplicate.manifest_version),
            vk::VK_API_VERSION_MINOR(duplicate.manifest_version),
            vk::VK_API_VERSION_PATCH(duplicate.manifest_version),
        ),
    );
    if duplicate.is_meta_layer() {
        if duplicate.manifest_version < vk::VK_MAKE_API_VERSION(0, 1, 1, 0) {
            emit_create_message(
                create_info,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "Layer \"{}\" contains meta-layer-specific component_layers, but using older JSON file version.",
                    crate::debug::diagnostics::LossyBytes(duplicate.name.to_bytes())
                ),
            );
        }
        emit_layer_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!(
                "Encountered meta-layer \"{}\"",
                crate::debug::diagnostics::LossyBytes(duplicate.name.to_bytes())
            ),
        );
    }
    if !duplicate.is_override() && duplicate.app_keys.is_some() {
        emit_layer_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!(
                "Layer {} contains app_keys, but any app_keys can only be provided by the override meta layer. These will be ignored.",
                crate::debug::diagnostics::LossyBytes(duplicate.name.to_bytes())
            ),
        );
    }
    if !duplicate.override_paths.is_empty()
        && duplicate.manifest_version < vk::VK_MAKE_API_VERSION(0, 1, 1, 0)
    {
        emit_create_message(
            create_info,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!(
                "Layer \"{}\" contains meta-layer-specific override paths, but using older JSON file version.",
                crate::debug::diagnostics::LossyBytes(duplicate.name.to_bytes())
            ),
        );
    }
}

#[cold]
fn emit_layer_field_diagnostic(
    sink: MetaDiagnosticSink<'_>,
    path: &Path,
    diagnostic: LayerManifestDiagnostic,
) {
    match diagnostic {
        LayerManifestDiagnostic::NonConformingName { name, .. } => {
            sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "Layer name {name} does not conform to naming standard (Policy #LLP_LAYER_3)"
                ),
            );
        }
        LayerManifestDiagnostic::MissingRequiredValue { name, .. } => {
            sink.message(
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "Layer located at {} didn't find required layer value \"{name}\" in manifest JSON file, skipping this layer",
                        path.display()
                    ),
                );
        }
        LayerManifestDiagnostic::MissingDisableEnvironment {
            name, meta_layer, ..
        } => {
            if meta_layer {
                sink.layer_message(
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                    format_args!("Encountered meta-layer \"{name}\""),
                );
            }
            sink.message(
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "Layer \"{name}\" doesn't contain required layer object disable_environment in the manifest JSON file, skipping this layer"
                    ),
                );
        }
        LayerManifestDiagnostic::InvalidDisableEnvironment { name, .. } => {
            sink.message(
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                    format_args!(
                        "Layer \"{name}\" doesn't contain required child value in object disable_environment in the manifest JSON file, skipping this layer (Policy #LLP_LAYER_9)"
                    ),
                );
        }
        LayerManifestDiagnostic::InvalidLibraryAndComponents {
            manifest_version,
            name,
            both_defined,
        } => {
            if !both_defined && manifest_version < vk::VK_MAKE_API_VERSION(0, 1, 1, 0) {
                sink.message(
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                        format_args!(
                            "Layer \"{name}\" contains meta-layer-specific component_layers, but using older JSON file version."
                        ),
                    );
            }
            let reason = if both_defined {
                "contains meta-layer-specific component_layers, but also defining layer library path.  Both are not compatible, so skipping this layer"
            } else {
                "is missing both library_path and component_layers fields.  One or the other MUST be defined.  Skipping this layer"
            };
            sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!("Layer \"{name}\" {reason}"),
            );
        }
        _ => unreachable!("document diagnostics are handled before layer fields"),
    }
}

#[cold]
fn emit_search_file(
    sink: MetaDiagnosticSink<'_>,
    search: &LayerSearch,
    file: &Path,
    manifests: &[LayerManifest],
    duplicate_meta_summaries: &mut Vec<(CString, Box<[CString]>)>,
    duplicate_messages: &mut Vec<String>,
) -> Result<(), VkResult> {
    let mut found = false;
    let needs_compatibility_diagnostics = search.diagnostic_files.iter().any(|path| path == file);
    let diagnostics = if needs_compatibility_diagnostics {
        discovery::layer_manifest_diagnostics(file, search.implicit)
    } else {
        Box::default()
    };
    let compatibility_manifests = needs_compatibility_diagnostics
        .then(|| discovery::reparse_layer_manifest(file, search.implicit));
    let diagnostic_manifests = compatibility_manifests.as_deref().unwrap_or(manifests);
    let mut diagnostic_indices = diagnostics
        .iter()
        .map(|(source_index, _)| *source_index)
        .peekable();
    for manifest in diagnostic_manifests
        .iter()
        .filter(|manifest| manifest.manifest_path == *file)
    {
        while diagnostic_indices
            .peek()
            .is_some_and(|source_index| *source_index < manifest.source_index)
        {
            let source_index = *diagnostic_indices.peek().unwrap();
            emit_manifest_diagnostics(sink, file, search.implicit, !found, Some(source_index));
            found = true;
            while diagnostic_indices.next_if_eq(&source_index).is_some() {}
        }
        while diagnostic_indices
            .next_if_eq(&manifest.source_index)
            .is_some()
        {}
        match sink {
            MetaDiagnosticSink::Global => emit_global_discovered_manifest(manifest, !found),
            MetaDiagnosticSink::Create(create_info) => {
                emit_create_discovered_manifest(create_info, manifest, !found);
            }
        }
        found = true;
    }
    while let Some(source_index) = diagnostic_indices.next() {
        emit_manifest_diagnostics(sink, file, search.implicit, !found, Some(source_index));
        found = true;
        while diagnostic_indices.next_if_eq(&source_index).is_some() {}
    }
    if needs_compatibility_diagnostics {
        emit_unused_override_layers(sink, file);
    }
    if found {
        return Ok(());
    }
    let duplicates = discovery::reparse_layer_manifest(file, search.implicit);
    if duplicates.is_empty() {
        emit_manifest_diagnostics(sink, file, search.implicit, true, None);
    }
    for duplicate in &duplicates {
        match sink {
            MetaDiagnosticSink::Global => emit_global_discovered_manifest(duplicate, true),
            MetaDiagnosticSink::Create(create_info) => {
                emit_duplicate_manifest(create_info, duplicate);
            }
        }
        if duplicate.is_override()
            && !duplicate.app_keys().is_empty()
            && !manifests.iter().any(|manifest| {
                manifest.name == duplicate.name && manifest.manifest_path == duplicate.manifest_path
            })
            && let Some(executable) = platform::executable_path()
        {
            sink.layer_message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format_args!(
                    "--Override layer found but not used because app '{}' is not in 'app_keys' list!",
                    executable.display()
                ),
            );
        }
        if let Some(original) = manifests.iter().find(|original| {
            (original.settings_control.is_none() || !duplicate.component_layers().is_empty())
                && original.name == duplicate.name
                && original.manifest_path != duplicate.manifest_path
        }) && record_duplicate_layer(
            duplicate,
            original,
            manifests,
            duplicate_meta_summaries,
            duplicate_messages,
        )
        .is_err()
        {
            return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
        }
    }
    Ok(())
}

#[cold]
fn emit_discovered_manifest_version(
    sink: MetaDiagnosticSink<'_>,
    manifest: &LayerManifest,
    emit_found: bool,
) {
    let manifest_major = vk::VK_API_VERSION_MAJOR(manifest.manifest_version);
    let manifest_minor = vk::VK_API_VERSION_MINOR(manifest.manifest_version);
    let manifest_patch = vk::VK_API_VERSION_PATCH(manifest.manifest_version);
    let known_manifest_version = manifest_major == 1
        && ((manifest_minor == 0 && manifest_patch < 2)
            || (manifest_minor == 1 && manifest_patch < 3)
            || (manifest_minor == 2 && manifest_patch < 2));
    if emit_found {
        if !known_manifest_version
            && let Some(version) = discovery::layer_manifest_version_text(&manifest.manifest_path)
        {
            sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format_args!(
                    "Found manifest file {} (file version {version})",
                    manifest.manifest_path.display(),
                ),
            );
        } else {
            sink.message(
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format_args!(
                    "Found manifest file {} (file version {manifest_major}.{manifest_minor}.{manifest_patch})",
                    manifest.manifest_path.display(),
                ),
            );
        }
    }
    if emit_found && !known_manifest_version {
        sink.layer_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!(
                "loader_add_layer_properties: {} has unknown layer manifest file version {manifest_major}.{manifest_minor}.{manifest_patch}.  May cause errors.",
                manifest.manifest_path.display(),
            ),
        );
    }
}

#[cold]
fn emit_unused_override_layers(sink: MetaDiagnosticSink<'_>, file: &Path) {
    if let Some(executable) = platform::executable_path() {
        for _ in 0..discovery::unused_override_layer_count(file, &executable) {
            sink.layer_message(vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format_args!("--Override layer found but not used because app '{}' is not in 'app_keys' list!", executable.display()));
        }
    }
}

#[cold]
fn emit_pruned_implicit_layers(
    searches: &[LayerSearch],
    manifests: &[LayerManifest],
    implicit_only: bool,
    emit_implicit_meta_pruning: bool,
) {
    let mut pruned_layers = Vec::new();
    let implicit_meta_layer_active = manifests.iter().any(|manifest| {
        manifest.implicit
            && !manifest.component_layers().is_empty()
            && implicit_manifest_is_active(manifest)
    });
    let self_referencing_meta_layer = manifests.iter().any(|manifest| {
        manifest
            .component_layers()
            .iter()
            .any(|component| component == &manifest.name)
    });
    if emit_implicit_meta_pruning
        && implicit_only
        && (implicit_meta_layer_active || self_referencing_meta_layer)
    {
        for search in searches.iter().filter(|search| !search.implicit) {
            for file in &search.files {
                for layer in &discovery::reparse_layer_manifest(file, search.implicit) {
                    if !layer.component_layers().is_empty()
                        || (!self_referencing_meta_layer
                            && manifests.iter().any(|meta| {
                                meta.component_layers()
                                    .iter()
                                    .any(|component| component == &layer.name)
                            }))
                    {
                        continue;
                    }
                    let name = allocation::try_c_string(&layer.name)
                        .and_then(|name| allocation::try_push(&mut pruned_layers, name));
                    if name.is_err() {
                        pending::mark_json_allocation_failed();
                        return;
                    }
                }
            }
        }
    }
    for layer in pruned_layers {
        platform::write_loader_log(
            LogFilter::Debug,
            format_args!(
                "loader_remove_layers_not_in_implicit_meta_layers : Implicit meta-layers are active, and layer {} is not list inside of any.  So removing layer from current layer list.",
                crate::debug::diagnostics::LossyBytes(layer.to_bytes()),
            ),
        );
    }
}

#[cold]
fn emit_disabled_global_layers(manifests: &[LayerManifest], implicit_only: bool) {
    if !implicit_only
        && let Some(override_layer) = manifests
            .iter()
            .find(|manifest| manifest.is_override() && naturally_enabled(manifest))
    {
        for blacklisted in &override_layer.blacklisted_layers {
            platform::write_loader_log(
                LogFilter::Debug,
                format_args!(
                    "loader_remove_layers_in_blacklist: Override layer is active and layer {} is in the blacklist inside of it. Removing that layer from current layer list.",
                    crate::debug::diagnostics::LossyBytes(blacklisted.to_bytes())
                ),
            );
        }
    }
    for manifest in manifests.iter().filter(|manifest| {
        manifest.settings_control.is_none()
            && forced_disabled(manifest)
            && !forced_enabled(manifest)
            && !(implicit_only && manifest.is_override())
    }) {
        if manifest.implicit && manifest.is_override() {
            platform::write_loader_log_with_category(
                LogFilter::Warning,
                LogFilter::Layer,
                format_args!(
                    "Implicit layer \"{}\" forced disabled because name matches filter of env var 'VK_LOADER_LAYERS_DISABLE'.",
                    crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
                ),
            );
        }
        platform::write_loader_log_with_category(
            LogFilter::Warning,
            LogFilter::Layer,
            format_args!(
                "{} \"{}\" forced disabled because name matches filter of env var 'VK_LOADER_LAYERS_DISABLE'.",
                if implicit_only {
                    "Implicit layer"
                } else {
                    "Layer"
                },
                crate::debug::diagnostics::LossyBytes(manifest.name.to_bytes())
            ),
        );
    }
}

#[cold]
fn emit_configured_manifest_reports(
    configured_manifest_reports: &[(std::path::PathBuf, u32)],
    manifests: &[LayerManifest],
) {
    for (path, version) in configured_manifest_reports {
        platform::write_loader_log(
            LogFilter::Info,
            format_args!(
                "Found manifest file {} (file version {}.{}.{})",
                path.display(),
                vk::VK_API_VERSION_MAJOR(*version),
                vk::VK_API_VERSION_MINOR(*version),
                vk::VK_API_VERSION_PATCH(*version),
            ),
        );
        for manifest in manifests.iter().filter(|manifest| {
            manifest.manifest_path == *path
                && !manifest.implicit
                && manifest.has_pre_instance_functions
        }) {
            platform::write_loader_log(
                LogFilter::Warning,
                format_args!(
                    "Found pre_instance_functions section in explicit layer from \"{}\". This section is only valid in implicit layers. The section will be ignored",
                    manifest.manifest_path.display()
                ),
            );
        }
    }
}

#[cold]
fn emit_layer_search_locations(sink: MetaDiagnosticSink<'_>, search: &LayerSearch) {
    let kind = if search.implicit {
        "implicit"
    } else {
        "explicit"
    };
    sink.layer_only(format_args!("Searching for {kind} layer manifest files"));
    sink.layer_only(format_args!("   In following locations:"));
    for root in &search.roots {
        sink.layer_only(format_args!("      {}", root.display()));
    }
    if search.files.is_empty() {
        sink.layer_only(format_args!("   Found no files"));
    } else {
        sink.layer_only(format_args!("   Found the following files:"));
        for file in &search.files {
            sink.layer_only(format_args!("      {}", file.display()));
        }
    }
}

#[cold]
fn emit_recursive_meta_reference(
    create_info: &VkInstanceCreateInfo<'_>,
    meta: &LayerManifest,
    component: &LayerManifest,
    activation_messages: &mut Vec<String>,
    repeated_activation_messages: &mut Vec<String>,
) -> Result<bool, VkResult> {
    if meta.settings_control.is_some() {
        let message = diagnostics::try_format(format_args!(
            "loader_add_meta_layer: Meta-layer {} recursively references itself through its component layers. Skipping the recursive reference.",
            crate::debug::diagnostics::LossyBytes(meta.name.to_bytes())
        ))?;
        allocation::try_push(&mut *activation_messages, allocation::try_string(&message)?)?;
        allocation::try_push(&mut *repeated_activation_messages, message)?;
        return Ok(true);
    }
    emit_create_message(
        create_info,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
        format_args!(
            "verify_meta_layer_component_layers: Recursive dependency between Meta-layer {} and  Meta-layer {}.  Skipping this layer.",
            crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
            crate::debug::diagnostics::LossyBytes(component.name.to_bytes())
        ),
    );
    emit_create_message(
        create_info,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
        format_args!(
            "loader_add_meta_layer: Meta-layer {} recursively references itself through its component layers. Skipping the meta-layer.",
            crate::debug::diagnostics::LossyBytes(meta.name.to_bytes())
        ),
    );
    Ok(false)
}

#[cold]
fn emit_meta_components(sink: MetaDiagnosticSink<'_>, meta: &LayerManifest) {
    sink.layer_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        format_args!(
            "Meta-layer \"{}\" all {} component layers appear to be valid.",
            crate::debug::diagnostics::LossyBytes(meta.name.to_bytes()),
            meta.component_layers().len()
        ),
    );
    for (component_index, component) in meta.component_layers().iter().enumerate() {
        sink.layer_only(format_args!(
            "  [{component_index}] {}",
            crate::debug::diagnostics::LossyBytes(component.to_bytes())
        ));
    }
}

#[cfg(test)]
mod allocation_tests {
    use super::MetaDiagnosticState;

    #[test]
    fn meta_diagnostic_storage_rolls_back_each_allocation_failure() {
        for count in [0, 1, 65] {
            crate::allocation::fault::sweep_operation(|| match MetaDiagnosticState::new(count) {
                Ok(mut state) => {
                    assert_eq!(state.available.len(), count);
                    assert!(state.available.iter().all(|value| *value));
                    assert!(state.checked.iter().all(|value| !value));
                    let storage = state.checked.as_ptr();
                    state.checked.fill(true);
                    state.checked.fill(false);
                    assert_eq!(state.checked.as_ptr(), storage);
                    assert!(state.checked.iter().all(|value| !value));
                    vk::VkResult::SUCCESS
                }
                Err(error) => error,
            });
        }
    }
}
