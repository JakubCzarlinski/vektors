use crate::*;
use alloc::ffi::CString;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static DESTROY_SAW_REGISTERED_DEVICE: AtomicBool = AtomicBool::new(false);
static DESTROY_DISPATCH_KEY: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
struct FakeNativeDevice {
    dispatch: *const LayerDeviceDispatchTable,
}

#[test]
fn truncated_multibyte_strings_stop_at_the_terminator() {
    const MAX_LOADER_STRING_LENGTH: libc::c_int = 256;
    const BAD_DATA: vk::VkFlags = 0x2;

    for truncated in [[0xc2_u8, 0], [0xe2, 0], [0xf0, 0]] {
        assert_eq!(
            unsafe {
                vk_string_validate(
                    MAX_LOADER_STRING_LENGTH,
                    truncated.as_ptr().cast::<c_char>(),
                )
            },
            BAD_DATA
        );
    }
    let short_sequence = [0xe2_u8, 0x82, 0];
    assert_eq!(
        unsafe {
            vk_string_validate(
                MAX_LOADER_STRING_LENGTH,
                short_sequence.as_ptr().cast::<c_char>(),
            )
        },
        BAD_DATA
    );
    for valid in [&b"abc\0"[..], &[0xc2_u8, 0xa9, 0]] {
        assert_eq!(
            unsafe {
                vk_string_validate(MAX_LOADER_STRING_LENGTH, valid.as_ptr().cast::<c_char>())
            },
            0
        );
    }
}

unsafe extern "system" fn fake_destroy_device(
    device: VkDevice,
    _allocator: *const VkAllocationCallbacks<'_>,
) {
    let registered = unsafe { LoaderDevice::from_handle(device) }.is_some();
    DESTROY_SAW_REGISTERED_DEVICE.store(registered, Ordering::SeqCst);
    let dispatch = unsafe { device.0.cast::<*const LayerDeviceDispatchTable>().read() };
    DESTROY_DISPATCH_KEY.store(dispatch as usize, Ordering::SeqCst);
    // A native destroy call is permitted to release the dispatchable's
    // storage before returning. Simulate that by making its first word
    // unreadable to loader handle lookup without actually freeing the
    // test's stack allocation.
    unsafe {
        device
            .0
            .cast::<*const LayerDeviceDispatchTable>()
            .write(core::ptr::null());
    };
}

unsafe extern "system" fn fake_get_device_proc_addr(
    _device: VkDevice,
    name: *const c_char,
) -> PFN_vkVoidFunction {
    if !name.is_null() && unsafe { CStr::from_ptr(name) } == c"vkDestroyDevice" {
        Some(erase_function(fake_destroy_device as PFN_vkDestroyDevice))
    } else {
        None
    }
}

#[test]
fn null_instance_resolves_the_global_commands() {
    for name in [
        "vkCreateInstance",
        "vkEnumerateInstanceExtensionProperties",
        "vkEnumerateInstanceLayerProperties",
        "vkEnumerateInstanceVersion",
        "vkGetInstanceProcAddr",
    ] {
        let name = CString::new(name).unwrap();
        // SAFETY: `name` is a live, NUL-terminated C string.
        let address = unsafe { vkGetInstanceProcAddr(VkInstance::NULL, name.as_ptr()) };
        assert!(address.is_some(), "{name:?} was not resolved");
    }
}

#[test]
fn device_group_chain_translation_is_scoped_and_restores_the_caller_chain() {
    let source_devices = [VkPhysicalDevice(0x1000_usize as *mut c_void)];
    let group = VkDeviceGroupDeviceCreateInfo {
        physicalDeviceCount: source_devices.len() as u32,
        pPhysicalDevices: source_devices.as_ptr(),
        ..VkDeviceGroupDeviceCreateInfo::DEFAULT
    };
    let mut prefix = vk::VkBaseInStructure {
        sType: VkStructureType::PHYSICAL_DEVICE_FEATURES_2,
        pNext: core::ptr::from_ref(&group).cast(),
        ..vk::VkBaseInStructure::DEFAULT
    };
    let original_group = prefix.pNext;
    let mut create_info = VkDeviceCreateInfo {
        pNext: core::ptr::from_mut(&mut prefix).cast(),
        ..VkDeviceCreateInfo::DEFAULT
    };

    let patch = unsafe {
        translate_device_group_chain(&mut create_info, |handle| {
            Some(VkPhysicalDevice(
                (handle.0 as usize + 0x1000) as *mut c_void,
            ))
        })
    }
    .unwrap()
    .unwrap();

    assert_ne!(prefix.pNext, original_group);
    let translated = unsafe { &*prefix.pNext.cast::<VkDeviceGroupDeviceCreateInfo<'_>>() };
    assert_eq!(translated.physicalDeviceCount, 1);
    assert_eq!(
        unsafe { translated.pPhysicalDevices.read() }.0 as usize,
        0x2000
    );

    drop(patch);
    assert_eq!(prefix.pNext, original_group);
}

#[test]
fn device_record_outlives_native_destroy_and_does_not_reread_dead_handle() {
    DESTROY_SAW_REGISTERED_DEVICE.store(false, Ordering::SeqCst);
    DESTROY_DISPATCH_KEY.store(0, Ordering::SeqCst);
    let instance = LoaderInstance::new(
        VK_API_VERSION_1_0,
        ExtensionSet::default(),
        Vec::new(),
        layer::ActiveLayers {
            loaded: Box::default(),
            reported: Box::default(),
            requested: Box::default(),
        },
        None,
        core::ptr::null(),
    )
    .unwrap();
    let mut native = FakeNativeDevice {
        dispatch: core::ptr::null(),
    };
    let handle = VkDevice(core::ptr::from_mut(&mut native).cast());
    // SAFETY: `native` is writable for the test lifetime, the fake resolver
    // targets that storage, and `instance` outlives the registered device.
    let device = unsafe {
        LoaderDevice::new(
            handle,
            fake_get_device_proc_addr,
            instance.as_ref(),
            0,
            VK_API_VERSION_1_0,
            false,
            ExtensionSet::default(),
        )
    }
    .unwrap();
    let Ok(handle) = LoaderDevice::try_register(device) else {
        panic!("test device registry allocation failed");
    };
    let dispatch_key = native.dispatch as usize;
    // SAFETY: The test has exclusive creation-time access and the fake
    // resolver represents the completed one-element device chain.
    unsafe {
        LoaderDevice::from_dispatch_key_mut(dispatch_key)
            .unwrap()
            .set_chain(handle, fake_get_device_proc_addr)
            .unwrap();
    };

    unsafe { vkDestroyDevice(handle, core::ptr::null()) };

    assert!(DESTROY_SAW_REGISTERED_DEVICE.load(Ordering::SeqCst));
    let dispatch = DESTROY_DISPATCH_KEY.load(Ordering::SeqCst);
    assert_ne!(dispatch, 0);
    assert!(
        LoaderDevice::take_dispatch(dispatch as *const LayerDeviceDispatchTable).is_none(),
        "the public trampoline must remove the saved registry entry after native teardown"
    );
}

#[test]
fn null_instance_rejects_non_global_commands() {
    let name = CString::new("vkEnumeratePhysicalDevices").unwrap();
    // SAFETY: `name` is a live, NUL-terminated C string.
    assert!(unsafe { vkGetInstanceProcAddr(VkInstance::NULL, name.as_ptr()) }.is_none());
}

#[test]
fn generated_command_table_classifies_dispatch() {
    assert_eq!(
        command_lookup(c"vkCreateInstance").unwrap().scope,
        CommandScope::Global
    );
    assert_eq!(
        command_lookup(c"vkDestroyInstance").unwrap().scope,
        CommandScope::Instance
    );
    assert_eq!(
        command_lookup(c"vkQueueSubmit").unwrap().scope,
        CommandScope::Device
    );
    assert!(command_lookup(c"vkTrimCommandPoolKHR").is_some());
    assert!(command_lookup(c"vkNotACommand").is_none());
    assert!(command_lookup(c"CreateInstance").is_none());
    assert!(command_lookup(c"vCreateInstance").is_none());
    assert!(exported_proc_addr(command_lookup(c"vkQueueSubmit").unwrap().id).is_some());
    assert!(core::hint::black_box(COMMAND_COUNT) > 800);
    assert!(core::hint::black_box(COMMAND_MAX_DISPLACEMENT) < u8::MAX);
}

#[test]
fn generated_surface_commands_separate_trampolines_and_terminators() {
    for name in [c"vkCreateHeadlessSurfaceEXT", c"vkDestroySurfaceKHR"] {
        let id = command_lookup(name).unwrap().id;
        let trampoline = exported_proc_addr(id).map(|function| function as usize);
        let terminator = instance_terminator_proc_addr(id).map(|function| function as usize);
        assert!(
            trampoline.is_some(),
            "missing public trampoline for {name:?}"
        );
        assert!(
            terminator.is_some(),
            "missing instance terminator for {name:?}"
        );
        assert_ne!(trampoline, terminator, "{name:?} bypasses the layer chain");
    }
}

#[test]
fn generated_command_lookup_is_exhaustive_and_exact() {
    let mut populated = 0;
    for record in COMMAND_TABLE.iter().filter(|record| record.id != u16::MAX) {
        let start = usize::from(record.name_offset);
        let end = start + usize::from(record.name_len);
        let suffix = &COMMAND_NAMES[start..end];
        let mut name = Vec::with_capacity(suffix.len() + 3);
        name.extend_from_slice(b"vk");
        name.extend_from_slice(suffix);
        name.push(0);
        let name = CStr::from_bytes_with_nul(&name).unwrap();
        assert_eq!(
            command_lookup(name),
            Some(CommandLookup {
                id: record.id,
                scope: record.scope,
            })
        );
        populated += 1;
    }
    assert_eq!(populated, COMMAND_COUNT);

    for name in [
        c"",
        c"vk",
        c"v",
        c"CreateBuffer",
        c"glCreateBuffer",
        c"xkCreateBuffer",
        c"vkCreateBuffe",
        c"vkDestroyInstanc",
        c"vkCreateBufferX",
        c"vkDestroyInstanceX",
        c"vkcreatebuffer",
        c"VKCREATEBUFFER",
        c"vkCREATEBuffer",
        c"vkGetInstanceProcAddx",
        c"vkCmdSetViewpost",
        c"vkCmdBeginRenderPasS",
        c"vkCreateBuffre",
        c"vkTransitionImageLayous",
        c"vkZzTs0sr",
        c"vkF5TyK",
    ] {
        assert!(command_lookup(name).is_none(), "resolved {name:?}");
    }
}

#[test]
fn numeric_environment_parsing_preserves_results_after_unicode_replacement() {
    for prefix in [b"".as_slice(), b"  +", b"12", b"10de:", b"10de:2684"] {
        for byte in 0_u8..=u8::MAX {
            let mut value = prefix.to_vec();
            value.push(byte);
            value.extend_from_slice(b"34:56");
            let lossy = String::from_utf8_lossy(&value);
            assert_eq!(
                decimal_prefix_nonzero(&value),
                decimal_prefix_nonzero(lossy.as_bytes()),
                "{value:?}"
            );
            assert_eq!(
                parse_linux_device_selection(&value),
                parse_linux_device_selection(lossy.as_bytes()),
                "{value:?}"
            );
        }
    }
}

#[test]
fn linux_sort_environment_numbers_match_c_prefix_parsing() {
    for value in [b"1tail".as_slice(), b"  +2", b"-3", b"0009x"] {
        assert!(decimal_prefix_nonzero(value), "rejected {value:?}");
    }
    for value in [b"".as_slice(), b"  ", b"+", b"-0tail", b"word1"] {
        assert!(!decimal_prefix_nonzero(value), "accepted {value:?}");
    }
    // A 32-bit C long saturates before the low-32-bit check; a 64-bit
    // C long represents these values exactly and their low bits are zero.
    for value in [b"4294967296".as_slice(), b"-4294967296"] {
        assert_eq!(
            decimal_prefix_nonzero(value),
            core::mem::size_of::<libc::c_long>() == 4
        );
    }
    assert!(decimal_prefix_nonzero(b"18446744073709551616"));
    assert_eq!(
        decimal_prefix_nonzero(b"-18446744073709551616"),
        core::mem::size_of::<libc::c_long>() == 4
    );
}

#[test]
fn linux_sort_forces_properties_extension_for_either_vulkan_1_0_side() {
    assert!(linux_sort_requires_properties_extension(
        vk::VK_API_VERSION_1_0,
        vk::VK_API_VERSION_1_1
    ));
    assert!(linux_sort_requires_properties_extension(
        vk::VK_API_VERSION_1_1,
        vk::VK_API_VERSION_1_0
    ));
    assert!(!linux_sort_requires_properties_extension(
        vk::VK_API_VERSION_1_1,
        vk::VK_API_VERSION_1_1
    ));
}

#[test]
fn icd_create_version_matches_upstream_device_configuration_policy() {
    assert_eq!(
        icd_create_application_api_version(
            vk::VK_API_VERSION_1_2,
            vk::VK_MAKE_API_VERSION(0, 1, 0, 7),
            false
        ),
        Some(vk::VK_MAKE_API_VERSION(0, 1, 0, 7))
    );
    assert_eq!(
        icd_create_application_api_version(vk::VK_API_VERSION_1_0, vk::VK_API_VERSION_1_2, true),
        Some(vk::VK_API_VERSION_1_1)
    );
    assert_eq!(
        icd_create_application_api_version(vk::VK_API_VERSION_1_0, vk::VK_API_VERSION_1_2, false),
        None
    );
}

#[test]
fn direct_driver_scan_only_propagates_host_oom() {
    assert_eq!(
        fatal_direct_driver_scan_error(VkResult::ERROR_OUT_OF_HOST_MEMORY),
        Some(VkResult::ERROR_OUT_OF_HOST_MEMORY)
    );
    for result in [
        VkResult::ERROR_INITIALIZATION_FAILED,
        VkResult::ERROR_INCOMPATIBLE_DRIVER,
        VkResult::ERROR_UNKNOWN,
    ] {
        assert_eq!(fatal_direct_driver_scan_error(result), None);
    }
}

#[test]
fn linux_device_selection_matches_scanf_hex_contract() {
    assert_eq!(
        parse_linux_device_selection(b"10de:2684"),
        Some((0x10de, 0x2684))
    );
    assert_eq!(
        parse_linux_device_selection(b" +0x10DE: 0X2684 trailing"),
        Some((0x10de, 0x2684))
    );
    assert_eq!(parse_linux_device_selection(b"-1:+2"), Some((u32::MAX, 2)));
    assert_eq!(parse_linux_device_selection(b"10de :2684"), None);
    assert_eq!(parse_linux_device_selection(b"10de:"), None);
    assert_eq!(parse_linux_device_selection(b"device:2684"), None);
    assert_eq!(
        parse_linux_device_selection(b"100000000:2"),
        Some((
            if libc::c_ulong::BITS == 32 {
                u32::MAX
            } else {
                0
            },
            2
        ))
    );
    assert_eq!(
        parse_linux_device_selection(b"ffffffffffffffffffffffff:2"),
        Some((u32::MAX, 2))
    );
    assert_eq!(
        parse_linux_device_selection(b"-ffffffffffffffffffffffff:2"),
        Some((u32::MAX, 2))
    );
}

#[test]
fn generated_wsi_filter_matches_compiled_backends() {
    #[cfg(not(feature = "wsi-directfb"))]
    assert!(!wsi_instance_extension_supported(
        vk::VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME
    ));
    #[cfg(feature = "wsi-directfb")]
    assert!(wsi_instance_extension_supported(
        vk::VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME
    ));
    #[cfg(all(
        feature = "wsi-xcb",
        any(
            target_os = "linux",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "dragonfly",
            target_os = "hurd",
            target_os = "cygwin"
        )
    ))]
    assert!(wsi_instance_extension_supported(
        vk::VK_KHR_XCB_SURFACE_EXTENSION_NAME
    ));
}
