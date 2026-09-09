use super::*;
use crate::{allocation::fault, discovery};
use core::mem::{offset_of, size_of};

#[test]
fn meta_reachability_handles_cycles_duplicates_and_allocation_failures() {
    let names = [c"VK_LAYER_a", c"VK_LAYER_b", c"VK_LAYER_isolated"];
    let mut manifests = names.map(|name| {
        let mut manifest = discovery::test_manifest(&[]);
        manifest.name = name.to_owned();
        manifest
    });
    manifests[0].source = crate::discovery::LayerSource::Meta(
        [names[1], names[1], names[0], names[1]]
            .map(|name| crate::discovery::LayerComponent::from(name.to_owned()))
            .into(),
    );
    manifests[1].source = crate::discovery::LayerSource::Meta(
        [names[0]]
            .map(|name| crate::discovery::LayerComponent::from(name.to_owned()))
            .into(),
    );
    discovery::resolve_layer_names(&manifests);
    fault::sweep_operation(|| {
        let mut traversal = MetaTraversal::default();
        for (from, target, expected) in [
            (0, 0, true),
            (0, 1, true),
            (0, 2, false),
            (1, 0, true),
            (2, 0, false),
        ] {
            match traversal.reaches(&manifests, from, target) {
                Ok(actual) => assert_eq!(actual, expected),
                Err(error) => return error,
            }
        }
        VkResult::SUCCESS
    });
}

#[test]
fn windows_layer_names_preserve_quotes_empty_entries_and_lossy_unicode() {
    let input = "first;sec\"ond;lay\"er;;last;";
    fault::sweep_operation(|| {
        let mut names = Vec::new();
        match append_windows_layer_names(&mut names, input.encode_utf16()) {
            Ok(()) => {
                let expected = [c"first", c"second;layer", c"", c"last", c""];
                assert_eq!(names.len(), expected.len());
                for (actual, expected) in names.iter().zip(expected) {
                    assert_eq!(actual.as_c_str(), expected);
                }
                VkResult::SUCCESS
            }
            Err(error) => error,
        }
    });
    fault::sweep_operation(|| {
        let mut names = Vec::new();
        match append_windows_layer_names(&mut names, [0xd800, 59, 0xd83d, 0xde00].into_iter()) {
            Ok(()) => {
                assert_eq!(names[0].as_bytes(), "�".as_bytes());
                assert_eq!(names[1].as_bytes(), "😀".as_bytes());
                VkResult::SUCCESS
            }
            Err(error) => error,
        }
    });
}

#[test]
fn requested_layers_propagate_allocation_failures() {
    let application_names = [c"VK_LAYER_application".as_ptr(), c"VK_LAYER_other".as_ptr()];
    let create_info = VkInstanceCreateInfo {
        enabledLayerCount: 2,
        ppEnabledLayerNames: application_names.as_ptr(),
        ..VkInstanceCreateInfo::DEFAULT
    };
    for environment in [None, Some(OsStr::new("VK_LAYER_environment"))] {
        fault::sweep_operation(|| match requested_layer_names(&create_info, environment) {
            Ok((names, environment_count)) => {
                assert_eq!(environment_count, usize::from(environment.is_some()));
                assert_eq!(names[environment_count].as_c_str(), c"VK_LAYER_application");
                assert_eq!(names[environment_count + 1].as_c_str(), c"VK_LAYER_other");
                if environment_count != 0 {
                    assert_eq!(names[0].as_c_str(), c"VK_LAYER_environment");
                }
                VkResult::SUCCESS
            }
            Err(error) => error,
        });
    }
}

#[test]
fn duplicate_device_extension_retains_first_property_like_upstream() {
    let mut extensions = Vec::new();
    append_unique_device_extension(
        &mut extensions,
        &device_extension_property(&LayerExtension {
            name: c"VK_EXT_debug_marker".to_owned().into(),
            spec_version: 1,
            entrypoints: Box::default(),
        }),
    )
    .unwrap();
    append_unique_device_extension(
        &mut extensions,
        &device_extension_property(&LayerExtension {
            name: c"VK_EXT_debug_marker".to_owned().into(),
            spec_version: 99,
            entrypoints: Box::default(),
        }),
    )
    .unwrap();

    assert_eq!(extensions.len(), 1);
    assert_eq!(extensions[0].specVersion, 1);
}

#[test]
fn layer_interface_layout_matches_vk_layer_h() {
    assert_eq!(offset_of!(NegotiateLayerInterface, s_type), 0);
    assert_eq!(offset_of!(LayerInstanceLink, next), 0);
    assert_eq!(offset_of!(LayerInstanceCreateInfo, s_type), 0);
    assert_eq!(offset_of!(LayerDeviceLink, next), 0);
    assert_eq!(offset_of!(LayerDeviceCreateInfo, s_type), 0);

    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(size_of::<NegotiateLayerInterface>(), 48);
        assert_eq!(offset_of!(NegotiateLayerInterface, p_next), 8);
        assert_eq!(
            offset_of!(NegotiateLayerInterface, loader_layer_interface_version),
            16
        );
        assert_eq!(
            offset_of!(NegotiateLayerInterface, get_instance_proc_addr),
            24
        );
        assert_eq!(
            offset_of!(NegotiateLayerInterface, get_device_proc_addr),
            32
        );
        assert_eq!(
            offset_of!(NegotiateLayerInterface, get_physical_device_proc_addr),
            40
        );

        assert_eq!(size_of::<LayerInstanceLink>(), 24);
        assert_eq!(
            offset_of!(LayerInstanceLink, next_get_instance_proc_addr),
            8
        );
        assert_eq!(
            offset_of!(LayerInstanceLink, next_get_physical_device_proc_addr),
            16
        );

        assert_eq!(size_of::<LayerInstanceCreateInfo>(), 40);
        assert_eq!(offset_of!(LayerInstanceCreateInfo, next), 8);
        assert_eq!(offset_of!(LayerInstanceCreateInfo, function), 16);
        assert_eq!(offset_of!(LayerInstanceCreateInfo, value), 24);

        assert_eq!(size_of::<LayerDeviceLink>(), 24);
        assert_eq!(offset_of!(LayerDeviceLink, next_get_instance_proc_addr), 8);
        assert_eq!(offset_of!(LayerDeviceLink, next_get_device_proc_addr), 16);

        assert_eq!(size_of::<LayerDeviceCreateInfo>(), 32);
        assert_eq!(offset_of!(LayerDeviceCreateInfo, next), 8);
        assert_eq!(offset_of!(LayerDeviceCreateInfo, function), 16);
        assert_eq!(offset_of!(LayerDeviceCreateInfo, value), 24);
    }

    #[cfg(target_pointer_width = "32")]
    {
        assert_eq!(size_of::<NegotiateLayerInterface>(), 24);
        assert_eq!(offset_of!(NegotiateLayerInterface, p_next), 4);
        assert_eq!(
            offset_of!(NegotiateLayerInterface, loader_layer_interface_version),
            8
        );
        assert_eq!(
            offset_of!(NegotiateLayerInterface, get_instance_proc_addr),
            12
        );
        assert_eq!(
            offset_of!(NegotiateLayerInterface, get_device_proc_addr),
            16
        );
        assert_eq!(
            offset_of!(NegotiateLayerInterface, get_physical_device_proc_addr),
            20
        );

        assert_eq!(size_of::<LayerInstanceLink>(), 12);
        assert_eq!(
            offset_of!(LayerInstanceLink, next_get_instance_proc_addr),
            4
        );
        assert_eq!(
            offset_of!(LayerInstanceLink, next_get_physical_device_proc_addr),
            8
        );

        assert_eq!(size_of::<LayerInstanceCreateInfo>(), 20);
        assert_eq!(offset_of!(LayerInstanceCreateInfo, next), 4);
        assert_eq!(offset_of!(LayerInstanceCreateInfo, function), 8);
        assert_eq!(offset_of!(LayerInstanceCreateInfo, value), 12);

        assert_eq!(size_of::<LayerDeviceLink>(), 12);
        assert_eq!(offset_of!(LayerDeviceLink, next_get_instance_proc_addr), 4);
        assert_eq!(offset_of!(LayerDeviceLink, next_get_device_proc_addr), 8);

        assert_eq!(size_of::<LayerDeviceCreateInfo>(), 16);
        assert_eq!(offset_of!(LayerDeviceCreateInfo, next), 4);
        assert_eq!(offset_of!(LayerDeviceCreateInfo, function), 8);
        assert_eq!(offset_of!(LayerDeviceCreateInfo, value), 12);
    }
}

#[test]
fn layer_function_discriminants_match_vk_layer_h() {
    assert_eq!(LayerFunction::LinkInfo as u32, 0);
    assert_eq!(LayerFunction::LoaderDataCallback as u32, 1);
    assert_eq!(LayerFunction::LayerCreateDeviceCallback as u32, 2);
    assert_eq!(LayerFunction::LoaderFeatures as u32, 3);
}
