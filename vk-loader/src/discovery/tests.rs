use super::layers::{retain_implicit_layers_and_components, select_override_layer_for_executable};
use super::manifest::{
    RawLayer, parse_raw_layer, parse_raw_layer_extensions, resolve_library_path, strtoul_prefix,
};
#[cfg(unix)]
use super::paths::append_nested_search_root;
use super::*;
use crate::json::Value;
use std::path::Path;
use vk::VK_MAKE_API_VERSION;

#[test]
fn manifest_c_strings_preserve_non_utf8_bytes_and_rollback_on_oom() {
    let source = b"{\"name\":\"VK_LAYER_test_\x8b\",\"type\":\"GLOBAL\",\"api_version\":\"1.3.127\",\"implementation_version\":\"2\",\"description\":\"description_\xff\",\"library_path\":\"test.so\",\"functions\":{\"vkGetInstanceProcAddr\":\"gipa_\xff\"},\"device_extensions\":[{\"name\":\"VK_test_\xfe\",\"spec_version\":\"1\",\"entrypoints\":[\"vkTest_\xfd\"]}]}";
    crate::allocation::fault::sweep_operation(|| {
        crate::pending::with_json_error_scope(|| {
            let Some(value) = parse_json_value(source) else {
                return vk::VkResult::SUCCESS;
            };
            let raw = RawLayer::from_value(&value).unwrap();
            let layer = parse_raw_layer(
                Path::new("layer.json"),
                raw,
                0,
                false,
                false,
                vk::VK_API_VERSION_1_0,
            );
            if !crate::pending::json_allocation_failed() {
                let layer = layer.unwrap();
                assert_eq!(layer.name.as_bytes(), b"VK_LAYER_test_\x8b");
                assert_eq!(layer.description.as_bytes(), b"description_\xff");
                assert_eq!(
                    layer.functions.get_instance_proc_addr.unwrap().as_bytes(),
                    b"gipa_\xff"
                );
                assert_eq!(layer.device_extensions[0].name.as_bytes(), b"VK_test_\xfe");
                assert_eq!(
                    layer.device_extensions[0].entrypoints[0].as_bytes(),
                    b"vkTest_\xfd"
                );
            }
            vk::VkResult::SUCCESS
        })
    });
}

#[cfg(unix)]
#[test]
fn split_search_paths_preserves_bytes_and_discards_partial_results_on_oom() {
    use std::{ffi::OsStr, os::unix::ffi::OsStrExt as _};

    crate::allocation::fault::sweep_operation(|| {
        crate::pending::with_json_error_scope(|| {
            let paths = split_paths(OsStr::from_bytes(b":first::\xff:last:"));
            if crate::pending::json_allocation_failed() {
                assert!(paths.is_empty());
                return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
            }
            assert_eq!(paths.len(), 3);
            for (path, expected) in paths.iter().zip([b"first".as_slice(), b"\xff", b"last"]) {
                assert_eq!(path.as_os_str().as_bytes(), expected);
            }
            vk::VkResult::SUCCESS
        })
    });
}

#[test]
fn search_root_merge_preserves_order_and_reports_allocation_failure() {
    crate::allocation::fault::sweep_operation(|| {
        crate::pending::with_json_error_scope(|| {
            let mut additional = Vec::new();
            let mut defaults = Vec::new();
            for (paths, names) in [
                (
                    &mut additional,
                    [
                        "additional-a",
                        "additional-b",
                        "additional-c",
                        "additional-d",
                    ],
                ),
                (
                    &mut defaults,
                    ["default-a", "default-b", "default-c", "default-d"],
                ),
            ] {
                for name in names {
                    let Some(path) = owned_path(Path::new(name)) else {
                        return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                    };
                    push_value(paths, path);
                    if crate::pending::json_allocation_failed() {
                        return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                    }
                }
            }
            extend_values(&mut additional, defaults);
            if crate::pending::json_allocation_failed() {
                return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
            }
            let expected = [
                "additional-a",
                "additional-b",
                "additional-c",
                "additional-d",
                "default-a",
                "default-b",
                "default-c",
                "default-d",
            ];
            assert_eq!(additional.len(), expected.len());
            for (path, expected) in additional.iter().zip(expected) {
                assert_eq!(path, Path::new(expected));
            }
            vk::VkResult::SUCCESS
        })
    });
}

#[cfg(windows)]
#[test]
fn windows_search_path_splitting_matches_std_and_reports_oom() {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt as _};

    let surrogate_path = OsString::from_wide(&[0xd800, 0x22, 0x22, 0xdc00, 0x3b, 0xdc00]);
    for input in [
        OsString::from(""),
        OsString::from(";first;;last;"),
        OsString::from("a;som\"e;di\"r;\"unterminated;quote"),
        OsString::from("\"\";\"a\"\"b\";c"),
        surrogate_path,
    ] {
        let expected: Vec<_> = std::env::split_paths(&input)
            .filter(|path| !path.as_os_str().is_empty())
            .collect();
        crate::allocation::fault::sweep_operation(|| {
            crate::pending::with_json_error_scope(|| {
                let paths = split_paths(&input);
                if crate::pending::json_allocation_failed() {
                    assert!(paths.is_empty());
                    return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                }
                assert_eq!(paths, expected);
                vk::VkResult::SUCCESS
            })
        });
    }
}

#[cfg(unix)]
#[test]
fn nested_search_paths_are_fallible_and_deduplicated() {
    for nested in [".config", ".local/share"] {
        let expected = Path::new("/home/test")
            .join(nested)
            .join("vulkan/explicit_layer.d");
        crate::allocation::fault::sweep_operation(|| {
            crate::pending::with_json_error_scope(|| {
                let mut paths = Vec::new();
                for _ in 0..2 {
                    append_nested_search_root(
                        &mut paths,
                        Path::new("/home/test"),
                        Some(nested),
                        "explicit_layer.d",
                    );
                    if crate::pending::take_json_allocation_failed() {
                        return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                    }
                }
                assert_eq!(paths.as_slice(), core::slice::from_ref(&expected));
                vk::VkResult::SUCCESS
            })
        });
    }
}

#[cfg(unix)]
#[test]
fn settings_path_growth_propagates_allocation_failures() {
    for (nested, expected) in [
        (
            None,
            "/vk-loader-test-root/vulkan/loader_settings.d/vk_loader_settings.json",
        ),
        (
            Some(".config"),
            "/vk-loader-test-root/.config/vulkan/loader_settings.d/vk_loader_settings.json",
        ),
        (
            Some(".local/share"),
            "/vk-loader-test-root/.local/share/vulkan/loader_settings.d/vk_loader_settings.json",
        ),
    ] {
        crate::allocation::fault::sweep_operation(|| {
            crate::pending::with_json_error_scope(|| {
                let mut path = PathBuf::new();
                let settings =
                    read_settings_from_root(&mut path, Path::new("/vk-loader-test-root"), nested);
                drop(settings);
                if crate::pending::take_json_allocation_failed() {
                    return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                }
                assert_eq!(path, Path::new(expected));
                vk::VkResult::SUCCESS
            })
        });
    }
}

#[test]
fn nested_manifest_construction_propagates_every_allocation_failure() {
    crate::allocation::fault::sweep_operation(|| {
        crate::pending::with_json_error_scope(|| {
            let bytes = br#"{
            "name":"VK_LAYER_TEST_fallible", "type":"GLOBAL", "library_path":"./layer.so",
            "api_version":"1.3.0", "implementation_version":"1", "description":"escaped\u0001description",
            "instance_extensions":[{"name":"VK_EXT_debug_utils","spec_version":"2"}],
            "device_extensions":[{"name":"VK_EXT_test","spec_version":"1","entrypoints":["vkTestA","vkTestB"]}],
            "enable_environment":{"ENABLE_TEST":"1"}, "disable_environment":{"DISABLE_TEST":"1"},
            "override_paths":["one","two"], "app_keys":["application"]
        }"#;
            let parsed = parse_json_value(bytes).and_then(|root| {
                parse_raw_layer(
                    Path::new("/manifest/layer.json"),
                    RawLayer::from_value(&root)?,
                    0,
                    true,
                    true,
                    VK_MAKE_API_VERSION(0, 1, 2, 0),
                )
            });
            let valid = parsed.as_ref().is_some_and(|layer| {
                layer.name.as_c_str() == c"VK_LAYER_TEST_fallible"
                    && layer.device_extensions.len() == 1
                    && layer.device_extensions[0].entrypoints.len() == 2
                    && layer.description.as_c_str() == c"escapedu0001description"
            });
            drop(parsed);
            if valid {
                vk::VkResult::SUCCESS
            } else {
                vk::VkResult::ERROR_INITIALIZATION_FAILED
            }
        })
    });
}

#[test]
fn relative_library_with_a_directory_is_manifest_relative() {
    let path = Path::new("/tmp/driver/icd.json");
    assert_eq!(
        resolve_library_path(path, PathBuf::from("./driver.so")),
        Some(PathBuf::from("/tmp/driver/driver.so"))
    );
}

#[test]
fn bare_library_name_uses_the_platform_search_path() {
    let path = Path::new("/tmp/driver/icd.json");
    assert_eq!(
        resolve_library_path(path, PathBuf::from("driver.so")),
        Some(PathBuf::from("driver.so"))
    );
}

#[test]
fn parses_standard_and_variant_api_versions() {
    assert_eq!(parse_api_version(Some("1.3.275")), Some(0x0040_3113));
    assert_eq!(parse_api_version(Some("1.1.3.275")), Some(0x2040_3113));
    assert_eq!(parse_api_version(Some("invalid")), Some(0));
    assert_eq!(parse_api_version(None), None);
}

#[test]
fn manifest_integer_overflow_saturates_at_target_ulong_like_strtoul() {
    let overflow = (u128::from(libc::c_ulong::MAX) + 1).to_string();
    assert_eq!(strtoul_prefix(overflow.as_bytes()), libc::c_ulong::MAX);
    assert_eq!(
        strtoul_prefix(format!("-{overflow}").as_bytes()),
        libc::c_ulong::MAX
    );
}

#[test]
fn borrowed_layer_extension_array_preserves_entries() {
    let json = r#"[
        {
            "name": "VK_EXT_debug_utils",
            "spec_version": "1",
            "spec_version": "2",
            "entrypoints": ["vkCreateDebugUtilsMessengerEXT"]
        }
    ]"#;
    let value = crate::json::parse(json.as_bytes()).unwrap();
    let raw = &value;
    let extensions = parse_raw_layer_extensions(Some(raw), false);
    assert_eq!(extensions.len(), 1);
    assert_eq!(extensions[0].name.as_c_str(), c"VK_EXT_debug_utils");
    assert_eq!(extensions[0].spec_version, 1);
    assert_eq!(extensions[0].entrypoints.len(), 1);
}

#[test]
fn borrowed_layer_parser_rejects_missing_api_version() {
    let json = r#"{
        "name": "VK_LAYER_missing_api_version",
        "type": "INSTANCE",
        "library_path": "layer.so",
        "implementation_version": "1",
        "description": "invalid"
    }"#;
    let value = crate::json::parse(json.as_bytes()).unwrap();
    let raw = RawLayer::from_value(&value).unwrap();
    assert!(
        parse_raw_layer(
            Path::new("layer.json"),
            raw,
            0,
            false,
            false,
            vk::VK_API_VERSION_1_0,
        )
        .is_none()
    );
}

#[test]
fn layer_keys_are_case_insensitive_like_upstream_cjson() {
    let json = r#"{
        "NAME": "VK_LAYER_case_insensitive",
        "TYPE": "INSTANCE",
        "LIBRARY_PATH": "layer.so",
        "API_VERSION": "1.0.0",
        "IMPLEMENTATION_VERSION": "1",
        "DESCRIPTION": "valid"
    }"#;
    let value = crate::json::parse(json.as_bytes()).unwrap();
    let raw = RawLayer::from_value(&value).unwrap();
    let layer = parse_raw_layer(
        Path::new("layer.json"),
        raw,
        0,
        false,
        false,
        vk::VK_API_VERSION_1_0,
    )
    .unwrap();
    assert_eq!(layer.name.as_c_str(), c"VK_LAYER_case_insensitive");
}

#[test]
fn device_layer_manifest_is_rejected_like_upstream() {
    let json = r#"{
        "name": "VK_LAYER_device",
        "type": "DEVICE",
        "library_path": "layer.so",
        "api_version": "1.0.0",
        "implementation_version": "1",
        "description": "deprecated"
    }"#;
    let value = crate::json::parse(json.as_bytes()).unwrap();
    let raw = RawLayer::from_value(&value).unwrap();
    assert!(
        parse_raw_layer(
            Path::new("layer.json"),
            raw,
            0,
            false,
            false,
            vk::VK_API_VERSION_1_0,
        )
        .is_none()
    );
}

#[test]
fn component_layers_presence_conflicts_with_library_path() {
    let json = r#"{
        "name": "VK_LAYER_not_a_meta_layer",
        "type": "INSTANCE",
        "library_path": "layer.so",
        "component_layers": [],
        "api_version": "1.0.0",
        "implementation_version": "1",
        "description": "invalid"
    }"#;
    let value = crate::json::parse(json.as_bytes()).unwrap();
    let raw = RawLayer::from_value(&value).unwrap();
    assert!(
        parse_raw_layer(
            Path::new("layer.json"),
            raw,
            0,
            false,
            false,
            vk::VK_API_VERSION_1_0,
        )
        .is_none()
    );
}

#[test]
fn wrong_typed_optional_objects_do_not_reject_explicit_layer() {
    let json = r#"{
        "name": "VK_LAYER_valid",
        "type": "INSTANCE",
        "library_path": "layer.so",
        "api_version": "1.0.0",
        "implementation_version": "1",
        "description": "valid",
        "functions": 7,
        "disable_environment": false,
        "instance_extensions": false
    }"#;
    let value = crate::json::parse(json.as_bytes()).unwrap();
    let raw = RawLayer::from_value(&value).unwrap();
    let layer = parse_raw_layer(
        Path::new("layer.json"),
        raw,
        0,
        false,
        false,
        vk::VK_API_VERSION_1_0,
    )
    .unwrap();
    assert!(layer.instance_extensions.is_empty());
    assert_eq!(layer.disable_environment, None);
    assert_eq!(layer.functions, LayerFunctions::default());
}

pub(crate) fn override_manifest(app_keys: &[&str]) -> LayerManifest {
    LayerManifest {
        source_index: 0,
        name: c"VK_LAYER_LUNARG_override".to_owned(),
        manifest_path: PathBuf::from("override.json"),
        library_path: None,
        manifest_version: vk::VK_API_VERSION_1_0,
        api_version: vk::VK_API_VERSION_1_0,
        architecture_supported: true,
        implementation_version: 0,
        description: CString::default(),
        instance_extensions: Box::default(),
        device_extensions: Box::default(),
        enable_environment: None,
        disable_environment: None,
        component_layers: Box::default(),
        has_component_layers: true,
        blacklisted_layers: Box::default(),
        override_paths: Box::default(),
        app_keys: app_keys.iter().map(PathBuf::from).collect(),
        has_app_keys: true,
        functions: LayerFunctions::default(),
        pre_instance_functions: PreInstanceFunctions::default(),
        has_pre_instance_functions: false,
        implicit: true,
        settings_control: None,
    }
}

#[test]
fn unavailable_executable_path_preserves_override_manifests() {
    let mut layers = vec![
        override_manifest(&["/pkg/bin/application"]),
        override_manifest(&[]),
    ];
    select_override_layer_for_executable(&mut layers, None);
    assert_eq!(layers.len(), 2);
}

#[test]
fn executable_path_selects_matching_override_before_global() {
    let mut layers = vec![
        override_manifest(&["/pkg/bin/application"]),
        override_manifest(&[]),
    ];
    select_override_layer_for_executable(&mut layers, Some(Path::new("/pkg/bin/application")));
    assert_eq!(layers.len(), 1);
    assert_eq!(layers[0].app_keys[0], Path::new("/pkg/bin/application"));
}

#[test]
fn settings_selection_stops_at_a_non_object_after_the_global_entry() {
    let root = parse_json_value(
        br#"{
            "settings_array": [
                {"stderr_log": ["all"]},
                7,
                {"stderr_log": ["error"]}
            ]
        }"#,
    )
    .unwrap();
    let (settings, invalid_element) = select_settings(&root);

    assert!(invalid_element);
    assert_eq!(
        settings
            .unwrap()
            .get("stderr_log")
            .and_then(Value::as_array)
            .and_then(|values| values.first())
            .and_then(Value::as_str),
        Some("all")
    );
}

#[test]
fn implicit_discovery_retains_only_meta_layer_components() {
    let mut meta = override_manifest(&[]);
    meta.name = c"VK_LAYER_implicit_meta".to_owned();
    meta.component_layers = [c"VK_LAYER_explicit_component".to_owned()].into();

    let mut component = override_manifest(&[]);
    component.name = c"VK_LAYER_explicit_component".to_owned();
    component.implicit = false;

    let mut unrelated = override_manifest(&[]);
    unrelated.name = c"VK_LAYER_unrelated_explicit".to_owned();
    unrelated.implicit = false;

    let mut manifests = vec![meta, component, unrelated];
    retain_implicit_layers_and_components(&mut manifests);

    assert_eq!(manifests.len(), 2);
    assert_eq!(manifests[0].name.as_c_str(), c"VK_LAYER_implicit_meta");
    assert_eq!(manifests[1].name.as_c_str(), c"VK_LAYER_explicit_component");
}

#[test]
fn rejected_meta_layer_does_not_hide_later_component_in_same_manifest() {
    let mut invalid = override_manifest(&[]);
    invalid.name = c"VK_LAYER_component".to_owned();
    invalid.component_layers = [c"VK_LAYER_component".to_owned()].into();

    let mut dependent = override_manifest(&[]);
    dependent.name = c"VK_LAYER_dependent".to_owned();
    dependent.component_layers = [c"VK_LAYER_component".to_owned()].into();

    let mut component = override_manifest(&[]);
    component.name = c"VK_LAYER_component".to_owned();
    component.has_component_layers = false;
    component.library_path = Some(PathBuf::from("component.so"));

    let mut manifests = vec![invalid, dependent, component];
    deduplicate_manifests_by_name(&mut manifests);
    assert_eq!(manifests.len(), 3);
    assert_eq!(
        &*valid_layer_mask(&manifests).unwrap(),
        &[false, true, true]
    );
}

#[test]
fn meta_recursion_uses_last_same_name_record_like_upstream() {
    let mut dependent = override_manifest(&[]);
    dependent.name = c"VK_LAYER_dependent".to_owned();
    dependent.component_layers = [c"VK_LAYER_component".to_owned()].into();

    let mut first = override_manifest(&[]);
    first.name = c"VK_LAYER_component".to_owned();
    first.component_layers = [c"VK_LAYER_component".to_owned()].into();

    let mut last = override_manifest(&[]);
    last.name = c"VK_LAYER_component".to_owned();
    let manifests = [dependent, first, last];
    assert_eq!(
        &*valid_layer_mask(&manifests).unwrap(),
        &[true, false, true]
    );
}

#[test]
fn layer_graph_storage_is_fallible() {
    for count in [0, 16, 32, 33, 65] {
        let manifests: Vec<_> = (0..count).map(|_| override_manifest(&[])).collect();
        let mut attempts = 0;
        crate::allocation::fault::sweep_operation(|| {
            attempts += 1;
            match valid_layer_mask(&manifests) {
                Ok(valid) => {
                    assert_eq!(valid.len(), count);
                    assert!(valid.iter().all(|valid| *valid));
                    vk::VkResult::SUCCESS
                }
                Err(result) => result,
            }
        });
        eprintln!(
            "layer graph: {count} layers, {} allocation sites",
            attempts - 1
        );
    }
}
