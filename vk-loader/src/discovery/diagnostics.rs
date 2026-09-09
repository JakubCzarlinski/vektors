//! Manifest validation diagnostics.

use super::LayerManifestDiagnostic;
use super::box_array;
use super::box_values;
use super::cjson_string;
use super::cjson_value_string;
use super::manifest::printed_bytes_fit;
use super::own_cow;
use super::owned_string;
use super::parse_api_version;
use super::parse_json_value;
use super::push_value;
use crate::json::Value;
use crate::platform;
use alloc::{string::String, vec::Vec};
use std::path::Path;
use vk::VK_MAKE_API_VERSION;

fn diagnostic_text<'a>(value: &'a Value<'_>) -> Option<alloc::borrow::Cow<'a, str>> {
    let bytes = value.as_bytes()?;
    if let Ok(text) = core::str::from_utf8(bytes) {
        return Some(alloc::borrow::Cow::Borrowed(text));
    }
    if let Ok(text) = crate::debug::diagnostics::try_format(format_args!(
        "{}",
        crate::debug::diagnostics::LossyBytes(bytes)
    )) {
        Some(alloc::borrow::Cow::Owned(text))
    } else {
        crate::pending::mark_json_allocation_failed();
        None
    }
}

pub(crate) fn layer_manifest_diagnostics(
    path: &Path,
    implicit: bool,
) -> Box<[(usize, LayerManifestDiagnostic)]> {
    let Some(bytes) = platform::read_file(path) else {
        return box_array([(0, LayerManifestDiagnostic::FailedOpen)]);
    };
    let Some(root) = parse_json_value(&bytes) else {
        return box_array([(0, LayerManifestDiagnostic::InvalidJson)]);
    };
    if !root.is_object() {
        return Box::default();
    }
    let Some(version_text) = root.get("file_format_version").and_then(diagnostic_text) else {
        return box_array([(0, LayerManifestDiagnostic::MissingFileFormatVersion)]);
    };
    let version_text = cjson_value_string(&version_text);
    let Some(version) = parse_api_version(Some(&version_text)) else {
        return box_array([(0, LayerManifestDiagnostic::MissingFileFormatVersion)]);
    };
    let layers = match root.get("layers").and_then(Value::as_array) {
        Some(layers) => layers.as_slice(),
        None => root.get("layer").map_or(&[][..], core::slice::from_ref),
    };
    if layers.is_empty() {
        return box_array([(
            0,
            LayerManifestDiagnostic::MissingLayers {
                version: own_cow(version_text),
                parsed_version: version,
            },
        )]);
    }
    let mut diagnostics = Vec::new();
    let major = vk::VK_API_VERSION_MAJOR(version);
    let minor = vk::VK_API_VERSION_MINOR(version);
    let patch = vk::VK_API_VERSION_PATCH(version);
    let known_version = major == 1
        && ((minor == 0 && patch < 2) || (minor == 1 && patch < 3) || (minor == 2 && patch < 2));
    if !known_version {
        push_value(
            &mut diagnostics,
            (
                0,
                LayerManifestDiagnostic::UnknownManifestVersion {
                    version: owned_string(&version_text),
                    parsed_version: version,
                },
            ),
        );
    }
    if root.get("layers").is_some() && version < VK_MAKE_API_VERSION(0, 1, 0, 1) {
        // `loader_parse_version_string` tokenizes its mutable input in place,
        // so the later compatibility warning observes only its first token.
        let warning_version = version_text
            .split(['.', '"', '\n', '\r'])
            .find(|component| !component.is_empty())
            .unwrap_or_default();
        push_value(
            &mut diagnostics,
            (
                0,
                LayerManifestDiagnostic::UnsupportedLayersArray {
                    found_version: owned_string(&version_text),
                    version: owned_string(warning_version),
                },
            ),
        );
    }
    for (source_index, layer) in layers.iter().enumerate() {
        let Some(diagnostic) = diagnose_layer(layer, implicit, version) else {
            continue;
        };
        if let Some(name) = layer.get("name").and_then(Value::as_str).map(cjson_string)
            && !name.as_bytes().starts_with(b"VK_LAYER_")
        {
            push_value(
                &mut diagnostics,
                (
                    source_index,
                    LayerManifestDiagnostic::NonConformingName {
                        manifest_version: version,
                        name: own_cow(name),
                    },
                ),
            );
        }
        push_value(&mut diagnostics, (source_index, diagnostic));
    }
    box_values(diagnostics)
}

pub(crate) fn layer_manifest_version_text(path: &Path) -> Option<String> {
    let bytes = platform::read_file(path)?;
    let root = parse_json_value(&bytes)?;
    let mut text = own_cow(diagnostic_text(root.get("file_format_version")?)?);
    if let Some(end) = text.find('\0') {
        text.truncate(end);
    }
    Some(text)
}

pub(crate) fn unused_override_layer_count(path: &Path, executable: &Path) -> usize {
    let Some(bytes) = platform::read_file(path) else {
        return 0;
    };
    let Some(root) = parse_json_value(&bytes) else {
        return 0;
    };
    let layers = match root.get("layers").and_then(Value::as_array) {
        Some(layers) => layers,
        None => root.get("layer").map_or(&[][..], core::slice::from_ref),
    };
    layers
        .iter()
        .filter(|layer| {
            let layer_type = layer.get("type").and_then(Value::as_str);
            let api_version = layer
                .get("api_version")
                .and_then(Value::as_str)
                .and_then(|version| parse_api_version(Some(version)));
            let has_library = layer.get("library_path").and_then(Value::as_str).is_some();
            let has_components = layer.get("component_layers").is_some();
            let valid_disable = layer
                .get("disable_environment")
                .and_then(Value::as_object)
                .and_then(|environment| environment.iter().next())
                .is_some_and(|(name, value)| !name.is_empty() && value.as_str().is_some());
            layer.get("name").and_then(Value::as_str) == Some("VK_LAYER_LUNARG_override")
                && matches!(layer_type, Some("INSTANCE" | "GLOBAL"))
                && api_version.is_some_and(|version| vk::VK_API_VERSION_VARIANT(version) == 0)
                && layer
                    .get("implementation_version")
                    .and_then(Value::as_str)
                    .is_some()
                && layer.get("description").and_then(Value::as_str).is_some()
                && has_library != has_components
                && valid_disable
                && layer
                    .get("app_keys")
                    .and_then(Value::as_array)
                    .is_some_and(|keys| {
                        !keys.iter().any(|key| {
                            key.as_str().is_some_and(|key| {
                                Path::new(cjson_string(key).as_ref()) == executable
                            })
                        })
                    })
        })
        .count()
}

fn diagnose_layer(layer: &Value, implicit: bool, version: u32) -> Option<LayerManifestDiagnostic> {
    let Some(layer) = layer.as_object() else {
        return Some(LayerManifestDiagnostic::InvalidJson);
    };
    let missing = |name| {
        Some(LayerManifestDiagnostic::MissingRequiredValue {
            manifest_version: version,
            name,
        })
    };
    for (name, capacity) in [
        ("name", Some(vk::VK_MAX_EXTENSION_NAME_SIZE as usize)),
        ("type", None),
        ("api_version", None),
    ] {
        let value = layer.get(name).and_then(Value::as_bytes);
        let valid = value.is_some_and(|value| {
            capacity.is_none_or(|capacity| printed_bytes_fit(value, capacity))
        });
        if !valid {
            return missing(name);
        }
        if name == "type"
            && !matches!(
                layer.get("type").and_then(Value::as_str),
                Some("INSTANCE" | "GLOBAL")
            )
        {
            return None;
        }
    }
    if layer
        .get("api_version")
        .and_then(Value::as_str)
        .and_then(|version| parse_api_version(Some(version)))
        .is_some_and(|version| vk::VK_API_VERSION_VARIANT(version) != 0)
    {
        return None;
    }
    if layer
        .get("implementation_version")
        .and_then(Value::as_bytes)
        .is_none()
    {
        return missing("implementation_version");
    }
    let description = layer.get("description").and_then(Value::as_bytes);
    if description.is_none_or(|description| {
        !printed_bytes_fit(description, vk::VK_MAX_DESCRIPTION_SIZE as usize)
    }) {
        return missing("description");
    }
    let has_library = layer
        .get("library_path")
        .and_then(Value::as_bytes)
        .is_some();
    let has_components = layer.get("component_layers").is_some();
    let name_text = layer
        .get("name")
        .and_then(diagnostic_text)
        .unwrap_or_default();
    let name = name_text.as_ref();
    if has_library == has_components {
        return Some(LayerManifestDiagnostic::InvalidLibraryAndComponents {
            manifest_version: version,
            name: owned_string(name),
            both_defined: has_library,
        });
    }
    if implicit && layer.get("disable_environment").is_none() {
        let name = owned_string(name);
        let meta_layer = layer
            .get("component_layers")
            .and_then(Value::as_array)
            .is_some_and(|components| !components.is_empty());
        return Some(LayerManifestDiagnostic::MissingDisableEnvironment {
            manifest_version: version,
            name,
            meta_layer,
        });
    }
    if implicit {
        let disable = layer.get("disable_environment").and_then(Value::as_object);
        if disable.is_none_or(|disable| {
            disable
                .iter()
                .next()
                .is_none_or(|(name, value)| name.is_empty() || value.as_str().is_none())
        }) {
            return Some(LayerManifestDiagnostic::InvalidDisableEnvironment {
                manifest_version: version,
                name: owned_string(name),
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{LayerManifestDiagnostic, diagnose_layer};

    #[test]
    fn malformed_layer_diagnostic_names_propagate_allocation_failure() {
        for (extra, expected) in [
            (r#", "component_layers": []"#, 0),
            ("", 1),
            (r#", "disable_environment": {}"#, 2),
        ] {
            let source = format!(
                r#"{{"name":"VK_LAYER_TEST_name","type":"GLOBAL","api_version":"1.0.0","implementation_version":"1","description":"test","library_path":"test.dll"{extra}}}"#
            );
            let layer = crate::json::parse(source.as_bytes()).unwrap();
            crate::allocation::fault::sweep_operation(|| {
                crate::pending::with_json_error_scope(|| {
                    let diagnostic = diagnose_layer(&layer, true, vk::VK_API_VERSION_1_0);
                    if crate::pending::json_allocation_failed() {
                        return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                    }
                    let (kind, name) = match diagnostic.unwrap() {
                        LayerManifestDiagnostic::InvalidLibraryAndComponents { name, .. } => {
                            (0, name)
                        }
                        LayerManifestDiagnostic::MissingDisableEnvironment { name, .. } => {
                            (1, name)
                        }
                        LayerManifestDiagnostic::InvalidDisableEnvironment { name, .. } => {
                            (2, name)
                        }
                        _ => panic!("unexpected diagnostic"),
                    };
                    assert_eq!(kind, expected);
                    assert_eq!(name, "VK_LAYER_TEST_name");
                    vk::VkResult::SUCCESS
                })
            });
        }
    }
}
