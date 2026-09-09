//! Layer discovery and override selection.

use super::DiscoveredLayers;
use super::LayerControl;
use super::LayerManifest;
use super::LayerSearch;
use super::LoaderSettings;
use super::box_array;
use super::box_values;
use super::deduplicate_manifests_by_name;
use super::deduplicate_paths;
use super::default_search_paths;
use super::extend_values;
use super::loader_settings;
use super::owned_path;
use super::parse_layer_manifest;
use super::parse_layer_manifest_inner;
use super::push_value;
use super::split_paths;
use super::unique_paths;
use crate::allocation;
use crate::layer;
use crate::{pending, platform};
use alloc::vec::Vec;
use core::ffi::CStr;
use std::path::{Path, PathBuf};

pub(crate) fn layer_search_roots(implicit: bool) -> Box<[PathBuf]> {
    let (override_name, add_name, directory) = if implicit {
        (
            c"VK_IMPLICIT_LAYER_PATH",
            c"VK_ADD_IMPLICIT_LAYER_PATH",
            platform::ManifestDirectory::ImplicitLayer,
        )
    } else {
        (
            c"VK_LAYER_PATH",
            c"VK_ADD_LAYER_PATH",
            platform::ManifestDirectory::ExplicitLayer,
        )
    };
    let elevated = platform::has_elevated_privileges();
    let environment = |name| {
        // SAFETY: Discovery follows upstream's exclusion of environment
        // mutation during loader operations; the returned value is owned.
        unsafe { platform::environment_value(name) }.unwrap_or_else(|_| {
            crate::pending::mark_json_allocation_failed();
            None
        })
    };
    let override_paths = (!elevated).then(|| environment(override_name)).flatten();
    if crate::pending::json_allocation_failed() {
        return Box::default();
    }
    let has_override = override_paths.is_some();
    let mut roots = override_paths.map_or_else(
        || default_search_paths(directory).into_vec(),
        |value| split_paths(&value),
    );
    if !elevated
        && !has_override
        && let Some(value) = environment(add_name)
    {
        let mut additional = split_paths(&value);
        extend_values(&mut additional, roots);
        roots = additional;
    }
    deduplicate_paths(&mut roots);
    box_values(roots)
}

#[cold]
#[inline(never)]
pub(crate) fn discover_layers() -> DiscoveredLayers {
    let settings = loader_settings();
    discover_layers_with_settings(settings.as_ref())
}

pub(crate) fn discover_layers_with_settings(settings: Option<&LoaderSettings>) -> DiscoveredLayers {
    let discovered = discover_layers_with_settings_inner(settings);
    super::resolve_layer_names(&discovered.manifests);
    discovered
}

fn discover_layers_with_settings_inner(settings: Option<&LoaderSettings>) -> DiscoveredLayers {
    if let Some(settings) = settings {
        let Some(configurations) = settings.layer_configurations.as_ref() else {
            let (manifests, searches) = discover_layers_from_search_paths_with_diagnostics();
            return DiscoveredLayers {
                manifests,
                searches,
                configured_manifest_reports: Box::default(),
                implicit_only: false,
            };
        };
        let mut layers = Vec::new();
        let mut searches = Vec::new();
        let mut configured_manifest_reports = Vec::new();
        let is_configured = |name: &CStr| {
            configurations.iter().any(|configuration| {
                configuration.control != LayerControl::UnorderedLayerLocation
                    && configuration.name.as_c_str() == name
            })
        };
        for configuration in configurations {
            if configuration.control == LayerControl::UnorderedLayerLocation {
                let (regular, discovered_searches) =
                    discover_layers_from_search_paths_with_diagnostics();
                let mut regular = regular.into_vec();
                regular.retain(|manifest| !is_configured(manifest.name.as_c_str()));
                extend_values(&mut layers, regular);
                extend_values(&mut searches, discovered_searches.into_vec());
                continue;
            }
            if configuration.control == LayerControl::Off {
                continue;
            }
            // get_settings_layers rejects non-JSON paths before opening them.
            if !platform::is_json_path(&configuration.path) {
                continue;
            }
            let mut configured = parse_layer_manifest(
                &configuration.path,
                configuration.treat_as_implicit_manifest,
            )
            .into_vec();
            if pending::json_allocation_failed() {
                break;
            }
            extend_values(
                &mut configured_manifest_reports,
                configured.iter().filter_map(|layer| {
                    Some((owned_path(&layer.manifest_path)?, layer.manifest_version))
                }),
            );
            configured.retain(|layer| layer.name.as_c_str() == configuration.name.as_c_str());
            for layer in &mut configured {
                layer.settings_control = Some(configuration.control);
            }
            for layer in configured {
                let duplicate = layers.iter().any(|existing: &LayerManifest| {
                    existing.name == layer.name
                        && (!existing.component_layers().is_empty()
                            || existing.manifest_path == layer.manifest_path)
                });
                if !duplicate {
                    push_value(&mut layers, layer);
                }
            }
        }
        return DiscoveredLayers {
            manifests: box_values(layers),
            searches: box_values(searches),
            configured_manifest_reports: box_values(configured_manifest_reports),
            implicit_only: false,
        };
    }
    let (manifests, searches) = discover_layers_from_search_paths_with_diagnostics();
    let mut manifests = manifests.into_vec();
    deduplicate_manifests_by_name(&mut manifests);
    DiscoveredLayers {
        manifests: box_values(manifests),
        searches,
        configured_manifest_reports: Box::default(),
        implicit_only: false,
    }
}

/// Discovers the subset used by pre-instance extension enumeration.
///
/// Upstream scans only implicit manifests on this path. Explicit manifests are
/// needed solely when an active override layer or an implicit meta-layer may
/// reference them.
pub(crate) fn discover_implicit_layers_with_settings(
    settings: Option<&LoaderSettings>,
) -> DiscoveredLayers {
    let discovered = discover_implicit_layers_inner(settings);
    super::resolve_layer_names(&discovered.manifests);
    discovered
}

fn discover_implicit_layers_inner(settings: Option<&LoaderSettings>) -> DiscoveredLayers {
    if settings.is_some() {
        // Loader settings replace ordinary implicit/explicit discovery. Every
        // configured layer participates in the pre-instance chain according
        // to its control value, regardless of its manifest type.
        // The outer wrapper resolves names after this metadata-only filtering.
        let mut discovered = discover_layers_with_settings_inner(settings);
        discovered.implicit_only = true;
        let retain_explicit_search = discovered.iter().any(|manifest| {
            manifest.implicit
                && !manifest.component_layers().is_empty()
                && (manifest.settings_control == Some(LayerControl::On)
                    || layer::implicit_manifest_is_active(manifest))
        });
        let mut searches = discovered.searches.into_vec();
        searches.retain(|search| search.implicit || retain_explicit_search);
        discovered.searches = box_values(searches);
        return discovered;
    }

    let implicit_roots = layer_search_roots(true);
    let (mut manifests, implicit_files, implicit_diagnostic_files) =
        discover_layers_in_roots_with_files(&implicit_roots, true);
    if pending::json_allocation_failed() {
        return DiscoveredLayers {
            manifests: box_values(manifests),
            searches: box_array([LayerSearch {
                implicit: true,
                roots: implicit_roots,
                files: implicit_files,
                diagnostic_files: implicit_diagnostic_files,
            }]),
            configured_manifest_reports: Box::default(),
            implicit_only: true,
        };
    }
    select_override_layer(&mut manifests);
    let active_override = manifests
        .iter()
        .find(|manifest| manifest.is_override() && layer::implicit_manifest_is_active(manifest));
    let has_implicit_meta_layer = manifests.iter().any(|manifest| {
        !manifest.component_layers().is_empty()
            && (layer::implicit_manifest_is_active(manifest)
                || !implicit_diagnostic_files.is_empty())
    });
    let explicit_search = if active_override.is_some() || has_implicit_meta_layer {
        let explicit_roots = active_override
            .filter(|manifest| {
                manifest
                    .override_paths
                    .iter()
                    .any(|path| !path.as_os_str().is_empty())
            })
            .map_or_else(
                || layer_search_roots(false),
                |manifest| unique_paths(&manifest.override_paths),
            );
        let (explicit, explicit_files, explicit_diagnostic_files) =
            discover_layers_in_roots_with_files(&explicit_roots, false);
        extend_values(&mut manifests, explicit);
        retain_implicit_layers_and_components(&mut manifests);
        Some(LayerSearch {
            implicit: false,
            roots: explicit_roots,
            files: explicit_files,
            diagnostic_files: explicit_diagnostic_files,
        })
    } else {
        None
    };
    deduplicate_manifests_by_name(&mut manifests);
    let implicit_search = LayerSearch {
        implicit: true,
        roots: implicit_roots,
        files: implicit_files,
        diagnostic_files: implicit_diagnostic_files,
    };
    let searches = match explicit_search {
        Some(explicit) => box_array([implicit_search, explicit]),
        None => box_array([implicit_search]),
    };
    DiscoveredLayers {
        manifests: box_values(manifests),
        searches,
        configured_manifest_reports: Box::default(),
        implicit_only: true,
    }
}

pub(super) fn retain_implicit_layers_and_components(manifests: &mut Vec<LayerManifest>) {
    super::resolve_layer_names(manifests);
    let Ok(mut keep) = allocation::try_boxed_slice_filled(manifests.len(), false) else {
        pending::mark_json_allocation_failed();
        return;
    };
    for (index, manifest) in manifests.iter().enumerate() {
        keep[index] |= manifest.implicit;
        for component in manifest.component_layers() {
            if let Some(component_index) = component.index() {
                keep[component_index] = true;
            }
        }
    }
    let mut index = 0;
    manifests.retain(|_| {
        let retain = keep[index];
        index += 1;
        retain
    });
}

fn discover_layers_from_search_paths_with_diagnostics() -> (Box<[LayerManifest]>, Box<[LayerSearch]>)
{
    let implicit_roots = layer_search_roots(true);
    let (mut layers, implicit_files, implicit_diagnostic_files) =
        discover_layers_in_roots_with_files(&implicit_roots, true);
    if pending::json_allocation_failed() {
        return (
            box_values(layers),
            box_array([LayerSearch {
                implicit: true,
                roots: implicit_roots,
                files: implicit_files,
                diagnostic_files: implicit_diagnostic_files,
            }]),
        );
    }
    select_override_layer(&mut layers);
    let override_roots = layers
        .iter()
        .find(|layer| {
            layer.is_override()
                && layer::implicit_manifest_is_active(layer)
                && layer
                    .override_paths
                    .iter()
                    .any(|path| !path.as_os_str().is_empty())
        })
        .map(|layer| layer.override_paths.as_ref());
    let explicit_roots = override_roots.map_or_else(|| layer_search_roots(false), unique_paths);
    let (explicit, explicit_files, explicit_diagnostic_files) =
        discover_layers_in_roots_with_files(&explicit_roots, false);
    extend_values(&mut layers, explicit);
    let searches = box_array([
        LayerSearch {
            implicit: true,
            roots: implicit_roots,
            files: implicit_files,
            diagnostic_files: implicit_diagnostic_files,
        },
        LayerSearch {
            implicit: false,
            roots: explicit_roots,
            files: explicit_files,
            diagnostic_files: explicit_diagnostic_files,
        },
    ]);
    (box_values(layers), searches)
}

fn discover_layers_in_roots_with_files(
    roots: &[PathBuf],
    implicit: bool,
) -> (Vec<LayerManifest>, Box<[PathBuf]>, Box<[PathBuf]>) {
    // Callback guards restore the outer set after synchronous reentrant calls.
    let callbacks = pending::instance_allocator();
    let mut layers = Vec::new();
    let mut files = Vec::new();
    let mut diagnostic_files = Vec::new();
    for root in roots {
        for path in platform::manifest_files(root) {
            let (parsed, needs_diagnostics, reparse_for_diagnostics) =
                parse_layer_manifest_inner(&path, implicit, callbacks);
            extend_values(&mut layers, parsed);
            if needs_diagnostics {
                if callbacks.is_some() && !pending::json_allocation_failed() {
                    layer::emit_global_layer_manifest_diagnostic(&path, implicit, true, None);
                }
                if (reparse_for_diagnostics
                    || callbacks.is_none()
                    || pending::json_allocation_failed())
                    && let Some(copy) = owned_path(&path)
                {
                    push_value(&mut diagnostic_files, copy);
                }
            }
            push_value(&mut files, path);
            if pending::json_allocation_failed() {
                return (layers, box_values(files), box_values(diagnostic_files));
            }
        }
    }
    (layers, box_values(files), box_values(diagnostic_files))
}

fn select_override_layer(layers: &mut Vec<LayerManifest>) {
    let executable = platform::executable_path();
    select_override_layer_for_executable(layers, executable.as_deref());
}

pub(super) fn select_override_layer_for_executable(
    layers: &mut Vec<LayerManifest>,
    executable: Option<&Path>,
) {
    let Some(executable) = executable else {
        // Upstream cannot validate app keys without an executable path and
        // consequently leaves the discovered override-layer list unchanged.
        return;
    };
    let matching = layers.iter().position(|layer| {
        layer.is_override() && layer.app_keys().iter().any(|key| key == executable)
    });
    let global = layers
        .iter()
        .position(|layer| layer.is_override() && layer.app_keys().is_empty());
    let selected = matching.or(global);
    let mut index = 0;
    layers.retain(|layer| {
        let keep = !layer.is_override() || Some(index) == selected;
        index += 1;
        keep
    });
}
