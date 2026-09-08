//! Environment predicates without copying values into owned strings.

#[cfg(windows)]
use alloc::boxed::Box;
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt as _;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt as _;
#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{
        ERROR_NOT_ENOUGH_MEMORY, ERROR_OUTOFMEMORY, ERROR_SUCCESS, GetLastError, SetLastError,
    },
    System::Environment::GetEnvironmentVariableW,
};

/// Tests presence, or equality with `expected` when supplied.
///
/// # Safety
/// On Unix, environment mutation must not race this call, including allocator
/// callbacks. This is the same contract as upstream's getenv reads.
pub(crate) unsafe fn environment_matches(name: &OsStr, expected: Option<&OsStr>) -> bool {
    #[cfg(unix)]
    {
        super::filesystem::with_c_path(std::path::Path::new(name), |name| {
            // SAFETY: The caller excludes mutation; the predicate only borrows.
            unsafe {
                super::environment::inspect_environment(name, |value| {
                    value.is_some_and(|value| {
                        expected.is_none_or(|expected| value.to_bytes() == expected.as_bytes())
                    })
                })
            }
        })
        .unwrap_or(false)
    }
    #[cfg(windows)]
    {
        windows_matches(name, expected).unwrap_or_else(|_| {
            crate::pending::mark_json_allocation_failed();
            false
        })
    }
    #[cfg(not(any(unix, windows)))]
    {
        std::env::var_os(name)
            .is_some_and(|value| expected.is_none_or(|expected| value == expected))
    }
}

#[cfg(windows)]
pub(super) struct WideBuffer {
    stack: [u16; 256],
    heap: Option<Box<[u16]>>,
}

#[cfg(windows)]
impl WideBuffer {
    pub(super) fn new(length: usize) -> Result<Self, vk::VkResult> {
        Ok(Self {
            stack: [0; 256],
            heap: if length > 256 {
                Some(crate::allocation::try_boxed_slice_filled(length, 0)?)
            } else {
                None
            },
        })
    }

    pub(super) fn as_mut_slice(&mut self) -> &mut [u16] {
        self.heap.as_deref_mut().unwrap_or(&mut self.stack)
    }
}

#[cfg(windows)]
fn windows_matches(name: &OsStr, expected: Option<&OsStr>) -> Result<bool, vk::VkResult> {
    if name.as_encoded_bytes().contains(&0) {
        return Ok(false);
    }
    let name_length = name
        .encode_wide()
        .count()
        .checked_add(1)
        .ok_or(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut name_buffer = WideBuffer::new(name_length)?;
    let name_buffer = name_buffer.as_mut_slice();
    for (slot, unit) in name_buffer.iter_mut().zip(name.encode_wide()) {
        *slot = unit;
    }
    let expected_length = expected.map_or(0, |value| value.encode_wide().count());
    let length = expected_length
        .checked_add(1)
        .ok_or(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    // Windows accepts DWORD capacities. A larger expected string cannot match
    // a value returned by this API.
    if length > u32::MAX as usize {
        return Ok(false);
    }
    let mut value = WideBuffer::new(length)?;
    let value = &mut value.as_mut_slice()[..length];
    // SAFETY: Both buffers are live; the name is terminated, and the output
    // length exactly describes initialized writable storage. Clear last-error
    // to distinguish an empty value from a missing variable.
    let (written, error) = unsafe {
        SetLastError(ERROR_SUCCESS);
        let written =
            GetEnvironmentVariableW(name_buffer.as_ptr(), value.as_mut_ptr(), length as u32);
        (written as usize, GetLastError())
    };
    if written == 0 && error != ERROR_SUCCESS {
        return match error {
            ERROR_NOT_ENOUGH_MEMORY | ERROR_OUTOFMEMORY => {
                Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)
            }
            _ => Ok(false),
        };
    }
    Ok(expected.is_none_or(|expected| {
        written == expected_length && value[..written].iter().copied().eq(expected.encode_wide())
    }))
}

#[cfg(test)]
mod tests {
    use super::environment_matches;
    use alloc::ffi::CString;
    use std::ffi::{OsStr, OsString};
    #[cfg(unix)]
    use std::os::unix::ffi::OsStringExt as _;
    #[cfg(windows)]
    use std::os::windows::ffi::OsStrExt as _;
    #[cfg(windows)]
    use std::os::windows::ffi::OsStringExt as _;
    #[cfg(windows)]
    use windows_sys::Win32::Globalization::{CP_UTF8, WideCharToMultiByte};

    #[cfg(windows)]
    fn upstream_environment_conversion(value: &OsStr) -> OsString {
        let wide: Vec<u16> = value.encode_wide().chain(core::iter::once(0)).collect();
        // SAFETY: The input is terminated and a null output queries capacity,
        // exactly as upstream loader_getenv does.
        let length = unsafe {
            WideCharToMultiByte(
                CP_UTF8,
                0,
                wide.as_ptr(),
                -1,
                core::ptr::null_mut(),
                0,
                core::ptr::null(),
                core::ptr::null_mut(),
            )
        };
        assert!(length > 0);
        let mut bytes = vec![0; length as usize];
        // SAFETY: The output has the queried capacity and the input is unchanged.
        let written = unsafe {
            WideCharToMultiByte(
                CP_UTF8,
                0,
                wide.as_ptr(),
                -1,
                bytes.as_mut_ptr(),
                length,
                core::ptr::null(),
                core::ptr::null_mut(),
            )
        };
        assert_eq!(written, length);
        assert_eq!(bytes.pop(), Some(0));
        OsString::from(String::from_utf8(bytes).unwrap())
    }

    #[test]
    fn predicates_match_std_and_propagate_scratch_oom() {
        const CHILD: &str = "VK_LOADER_ENVIRONMENT_PREDICATE_CHILD";
        const VALUE: &str = "VK_LOADER_ENVIRONMENT_PREDICATE_VALUE";
        let long_name = "VK_LOADER_MISSING_VARIABLE_".repeat(32);
        if std::env::var_os(CHILD).is_none() {
            let long_value = "long value 🦀".repeat(64);
            #[cfg(unix)]
            let native_value = OsString::from_vec(vec![0xff, b'x']);
            #[cfg(windows)]
            let native_value = OsString::from_wide(&[0xd800, u16::from(b'x')]);
            #[cfg(not(any(unix, windows)))]
            let native_value = OsString::from("native value");
            for value in [
                None,
                Some(OsStr::new("")),
                Some(OsStr::new("value")),
                Some(OsStr::new(&long_value)),
                Some(native_value.as_os_str()),
            ] {
                let mut command = std::process::Command::new(std::env::current_exe().unwrap());
                command.args(["--exact", "platform::environment_predicate::tests::predicates_match_std_and_propagate_scratch_oom"])
                    .env(CHILD, "1").env_remove(&long_name);
                if let Some(value) = value {
                    command.env(VALUE, value);
                    command.env("VK_LOADER_DEBUG", value);
                } else {
                    command.env_remove(VALUE);
                    command.env_remove("VK_LOADER_DEBUG");
                }
                assert!(command.status().unwrap().success());
            }
            return;
        }
        let logging_enabled =
            std::env::var_os("VK_LOADER_DEBUG").is_some_and(|value| !value.is_empty());
        crate::allocation::fault::sweep_operation(|| {
            assert_eq!(
                crate::platform::loader_debug_logging_enabled(),
                logging_enabled
            );
            vk::VkResult::SUCCESS
        });
        for name in [VALUE, "", "invalid\0name", long_name.as_str()] {
            let name = OsStr::new(name);
            let baseline = std::env::var_os(name);
            for expected in [
                None,
                Some(OsStr::new("")),
                Some(OsStr::new("not the value")),
                baseline.as_deref(),
            ] {
                let matches = baseline
                    .as_deref()
                    .is_some_and(|value| expected.is_none_or(|expected| value == expected));
                crate::allocation::fault::sweep_operation(|| {
                    crate::pending::with_json_error_scope(|| {
                        // SAFETY: This test only reads the process environment.
                        let actual = unsafe { environment_matches(name, expected) };
                        if crate::pending::json_allocation_failed() {
                            return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                        }
                        assert_eq!(actual, matches, "{name:?} / {expected:?}");
                        vk::VkResult::SUCCESS
                    })
                });
            }
            if let Ok(name) = CString::new(name.as_encoded_bytes()) {
                #[cfg(windows)]
                let snapshot = baseline.as_deref().map(upstream_environment_conversion);
                #[cfg(not(windows))]
                let snapshot = baseline.clone();
                crate::allocation::fault::sweep_operation(|| {
                    // SAFETY: No environment mutation occurs in the child.
                    match unsafe { crate::platform::environment_value(&name) } {
                        Ok(actual) => {
                            assert_eq!(actual, snapshot);
                            vk::VkResult::SUCCESS
                        }
                        Err(error) => error,
                    }
                });
                let lossy = baseline.as_deref().map(OsStr::to_string_lossy);
                crate::allocation::fault::sweep_operation(|| {
                    // SAFETY: The child never mutates its process environment.
                    let result = unsafe {
                        crate::platform::inspect_environment_lossy(&name, |actual| {
                            assert_eq!(actual, lossy.as_deref());
                        })
                    };
                    result.map_or_else(|error| error, |()| vk::VkResult::SUCCESS)
                });
                crate::allocation::fault::sweep_operation(|| {
                    // SAFETY: Only this test runs in the child process and it
                    // never mutates the environment supplied by its parent.
                    let result = unsafe {
                        crate::platform::inspect_environment_text(&name, |actual| {
                            assert_eq!(actual, baseline.as_deref().and_then(OsStr::to_str));
                        })
                    };
                    result.map_or_else(|error| error, |()| vk::VkResult::SUCCESS)
                });
            }
        }
    }
}
