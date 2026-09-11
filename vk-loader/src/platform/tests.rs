#[cfg(any(target_os = "linux", target_os = "macos"))]
use super::executable::executable_path_from_bytes;
use super::*;
use super::{logging::loader_debug_filter_matches, windows::extend_registry_paths};
use crate::debug::diagnostics::LogBuffer;
use core::fmt::Write as _;
#[cfg(unix)]
use core::ptr::NonNull;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::os::unix::ffi::OsStrExt as _;
use std::path::Path;

#[cfg(unix)]
#[test]
fn normalization_checks_preserve_failures_and_report_scratch_oom() {
    assert_eq!(path_normalizes(Path::new(".")), Ok(true));
    assert_eq!(path_normalizes(Path::new("invalid\0path")), Ok(false));
    assert_eq!(
        path_normalizes(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/Cargo.toml/not-a-directory"
        ))),
        Ok(false)
    );

    let long_path = "./".repeat(160);
    crate::allocation::fault::sweep_operation(|| match path_normalizes(Path::new(&long_path)) {
        Ok(normalizes) => {
            assert!(normalizes);
            vk::VkResult::SUCCESS
        }
        Err(result) => result,
    });
}

#[cfg(windows)]
#[test]
fn windows_utf16_paths_are_fallible_and_obey_the_executable_byte_limit() {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt as _, path::PathBuf};

    for units in [
        vec![],
        vec![u16::from(b'a')],
        vec![0xd800],
        vec![0xdc00],
        vec![0xd83e, 0xdd80],
        vec![0xd800, 0xd800, 0xdc00],
        vec![u16::from(b'a'); 1023],
        vec![u16::from(b'a'); 1024],
        vec![0x800; 341],
        vec![0x800; 342],
    ] {
        let native = OsString::from_wide(&units);
        let expected = PathBuf::from(native.to_string_lossy().into_owned());
        crate::allocation::fault::sweep_operation(|| match super::windows::paths::path_from_utf16(
            &units,
        ) {
            Ok(actual) => {
                assert_eq!(actual, expected);
                vk::VkResult::SUCCESS
            }
            Err(error) => error,
        });
        crate::allocation::fault::sweep_operation(|| {
            crate::pending::with_json_error_scope(|| {
                let actual = super::executable::executable_path_from_utf16(&units);
                if crate::pending::json_allocation_failed() {
                    return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                }
                assert_eq!(
                    actual.as_ref(),
                    (expected.as_os_str().len() < 1024).then_some(&expected)
                );
                vk::VkResult::SUCCESS
            })
        });
    }
}

#[cfg(windows)]
#[test]
fn windows_normalization_matches_std_and_propagates_allocation_failures() {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt as _, path::PathBuf};

    let absolute = std::env::current_dir().unwrap();
    let verbatim = std::fs::canonicalize(&absolute).unwrap();
    let paths = [
        PathBuf::from(""),
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("Cargo.toml"),
        PathBuf::from("Cargo.toml/child"),
        PathBuf::from("vk-loader-nonexistent-normalization-input"),
        PathBuf::from("invalid\0path"),
        PathBuf::from("./".repeat(320)),
        PathBuf::from(OsString::from_wide(&[0xd800])),
        absolute,
        verbatim,
    ];
    for path in paths {
        let expected = std::fs::canonicalize(&path).is_ok();
        crate::allocation::fault::sweep_operation(|| match path_normalizes(&path) {
            Ok(actual) => {
                assert_eq!(actual, expected, "{}", path.display());
                vk::VkResult::SUCCESS
            }
            Err(result) => result,
        });
        let exists = path.exists();
        crate::allocation::fault::sweep_operation(|| {
            crate::pending::with_json_error_scope(|| {
                let actual = file_exists(&path);
                if crate::pending::json_allocation_failed() {
                    return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                }
                assert_eq!(actual, exists, "{}", path.display());
                vk::VkResult::SUCCESS
            })
        });
    }
}

#[test]
fn registry_path_collection_is_fallible_and_preserves_diagnostics() {
    crate::allocation::fault::sweep_operation(|| {
        let run = || -> Result<(), vk::VkResult> {
            let mut paths = Vec::new();
            let mut located = Vec::new();
            for name in ["first.json", "second.json", "first.json"] {
                let path = crate::allocation::try_path(Path::new(name))?;
                extend_registry_paths(&mut paths, &mut located, core::iter::once(path))?;
            }
            assert_eq!(paths.len(), 2);
            assert_eq!(paths[0], Path::new("first.json"));
            assert_eq!(paths[1], Path::new("second.json"));
            assert_eq!(located.len(), 3);
            assert_eq!(located[0], located[2]);
            Ok(())
        };
        run().map_or_else(|error| error, |()| vk::VkResult::SUCCESS)
    });
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[test]
fn executable_path_copy_is_fallible_and_preserves_non_utf8_bytes() {
    crate::allocation::fault::sweep_operation(|| {
        crate::pending::with_json_error_scope(|| {
            let bytes = b"/application/invalid-\xff-name";
            match executable_path_from_bytes(bytes) {
                Some(path) => {
                    assert_eq!(path.as_os_str().as_bytes(), bytes);
                    vk::VkResult::SUCCESS
                }
                None => vk::VkResult::ERROR_INITIALIZATION_FAILED,
            }
        })
    });
}

const CHILD_MODE: &str = "VK_LOADER_RUST_UNLOADING_POLICY_TEST";

#[cfg(unix)]
#[test]
fn missing_library_error_allocation_is_fallible() {
    crate::allocation::fault::sweep_operation(|| {
        let path = Path::new("/vk-loader-nonexistent-allocation-test/no-library.so");
        // SAFETY: The deliberately nonexistent path cannot execute code.
        match unsafe { LoaderLibrary::open_unix(path) } {
            Ok(library) => {
                drop(library);
                vk::VkResult::ERROR_INITIALIZATION_FAILED
            }
            Err(error) => match error.into_message(path) {
                Ok(message) => {
                    drop(message);
                    vk::VkResult::SUCCESS
                }
                Err(error) => error,
            },
        }
    });
}

#[cfg(unix)]
#[test]
fn missing_symbol_does_not_allocate_rust_error_storage() {
    // SAFETY: A null filename requests a reference to the current process;
    // the returned reference is released by the ordinary library owner.
    let handle = unsafe { libc::dlopen(core::ptr::null(), libc::RTLD_LAZY | libc::RTLD_LOCAL) };
    let library = LoaderLibrary(NonNull::new(handle).expect("current process module"));
    crate::allocation::fault::sweep_operation(|| {
        // SAFETY: Only lookup is performed; the deliberately missing name
        // cannot produce a function that is called with the wrong ABI.
        let symbol = unsafe {
            library.get::<unsafe extern "C" fn()>(b"vk_loader_deliberately_missing_symbol\0")
        };
        if symbol.is_err() {
            vk::VkResult::SUCCESS
        } else {
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        }
    });
}

#[test]
fn log_buffer_preserves_the_utf8_prefix_across_format_fragments() {
    let mut message_storage = [0; 4];
    let mut message = LogBuffer::new(&mut message_storage);
    let _ = message.write_str("abc");
    let _ = message.write_str("é");
    let _ = message.write_str("z");
    assert_eq!(message.as_str(), "abc");
    let mut exact_storage = [0; 4];
    let mut exact = LogBuffer::new(&mut exact_storage);
    let _ = exact.write_str("éé");
    assert_eq!(exact.as_str(), "éé");
}

#[test]
fn debug_filter_matches_upstream_prefix_rules() {
    assert!(loader_debug_filter_matches("all", LogFilter::Info));
    assert!(loader_debug_filter_matches("a", LogFilter::Layer));
    assert!(loader_debug_filter_matches("w", LogFilter::Warning));
    assert!(loader_debug_filter_matches("warn", LogFilter::Warning));
    assert!(loader_debug_filter_matches("implem", LogFilter::Driver));
    assert!(loader_debug_filter_matches("ic", LogFilter::Driver));
    assert!(loader_debug_filter_matches(
        "debug,driver",
        LogFilter::Driver
    ));
    assert!(!loader_debug_filter_matches("", LogFilter::Info));
    assert!(!loader_debug_filter_matches("INFO", LogFilter::Info));
    assert!(!loader_debug_filter_matches("warnings", LogFilter::Warning));
    assert!(!loader_debug_filter_matches("layer", LogFilter::Driver));
}

#[test]
fn unloading_policy_is_exact_and_process_wide() {
    match std::env::var(CHILD_MODE).as_deref() {
        Ok("disabled") => {
            crate::allocation::fault::sweep_operation(|| {
                assert!(dynamic_library_unloading_disabled());
                vk::VkResult::SUCCESS
            });
            // SAFETY: The subprocess runs only this exact test, and the
            // policy has already copied the environment value.
            unsafe { std::env::set_var("VK_LOADER_DISABLE_DYNAMIC_LIBRARY_UNLOADING", "0") };
            assert!(dynamic_library_unloading_disabled());
        }
        Ok("not-disabled") => {
            crate::allocation::fault::sweep_operation(|| {
                assert!(!dynamic_library_unloading_disabled());
                vk::VkResult::SUCCESS
            });
            // SAFETY: Only this exact test runs in the child, and the cached
            // policy no longer reads the environment after initialization.
            unsafe { std::env::set_var("VK_LOADER_DISABLE_DYNAMIC_LIBRARY_UNLOADING", "1") };
            assert!(!dynamic_library_unloading_disabled());
        }
        _ => {
            for (mode, value) in [
                ("disabled", Some("1")),
                ("not-disabled", Some("10")),
                ("not-disabled", Some("0")),
                ("not-disabled", Some("")),
                ("not-disabled", None),
            ] {
                let mut command = std::process::Command::new(
                    std::env::current_exe().expect("test executable path"),
                );
                command
                    .args([
                        "--exact",
                        "platform::tests::unloading_policy_is_exact_and_process_wide",
                    ])
                    .env(CHILD_MODE, mode);
                match value {
                    Some(value) => {
                        command.env("VK_LOADER_DISABLE_DYNAMIC_LIBRARY_UNLOADING", value)
                    }
                    None => command.env_remove("VK_LOADER_DISABLE_DYNAMIC_LIBRARY_UNLOADING"),
                };
                let status = command.status().expect("spawn policy subprocess");
                assert!(status.success());
            }
        }
    }
}
