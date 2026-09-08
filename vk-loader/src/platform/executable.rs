//! Platform executable-path discovery.

#[cfg(target_os = "macos")]
use core::ffi::{c_int, c_void};
use std::path::PathBuf;
#[cfg(any(
    target_os = "linux",
    target_os = "cygwin",
    target_os = "android",
    target_os = "hurd",
    target_os = "macos",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "nto",
    target_os = "qnx",
    all(test, unix)
))]
use std::{ffi::OsStr, os::unix::ffi::OsStrExt as _, path::Path};
#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{ERROR_INSUFFICIENT_BUFFER, GetLastError},
    System::LibraryLoader::GetModuleFileNameW,
};

#[cfg(any(
    target_os = "linux",
    target_os = "cygwin",
    target_os = "android",
    target_os = "hurd",
    target_os = "macos",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "nto",
    target_os = "qnx",
    all(test, unix)
))]
use crate::{allocation, pending};

/// Returns the current executable path using Vulkan-Loader's 1024-byte platform
/// contract. The fixed bound and unavailable-platform cases are observable in
/// app-keyed loader settings and override-layer selection.
#[cfg(any(target_os = "fuchsia", target_os = "openbsd"))]
pub(crate) const fn executable_path() -> Option<PathBuf> {
    None
}

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "hurd",
    target_os = "cygwin"
))]
pub(crate) fn executable_path() -> Option<PathBuf> {
    let mut bytes = [0_u8; 1024];
    // SAFETY: `bytes` supplies exactly the writable extent passed to readlink.
    let length = unsafe {
        libc::readlink(
            c"/proc/self/exe".as_ptr(),
            bytes.as_mut_ptr().cast(),
            bytes.len(),
        )
    };
    if length < 0 {
        return None;
    }
    let length = length as usize;
    if length == 0 || length >= bytes.len() {
        return None;
    }
    executable_path_from_bytes(&bytes[..length])
}

#[cfg(target_os = "macos")]
pub(crate) fn executable_path() -> Option<PathBuf> {
    unsafe extern "C" {
        fn proc_pidpath(pid: libc::pid_t, buffer: *mut c_void, buffer_size: u32) -> c_int;
    }

    let mut bytes = [0_u8; 1024];
    // SAFETY: `bytes` supplies the complete writable extent and getpid has no
    // preconditions. proc_pidpath does not retain the buffer.
    let length = unsafe {
        proc_pidpath(
            libc::getpid(),
            bytes.as_mut_ptr().cast(),
            bytes.len() as u32,
        )
    };
    if length < 0 {
        return None;
    }
    let length = length as usize;
    if length == 0 || length >= bytes.len() {
        return None;
    }
    let length = bytes[..=length]
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(length);
    executable_path_from_bytes(&bytes[..length])
}

// Vulkan-Loader deliberately reports a present but empty executable path on
// Apple's mobile targets.
#[cfg(all(target_vendor = "apple", not(target_os = "macos")))]
pub(crate) fn executable_path() -> Option<PathBuf> {
    Some(PathBuf::new())
}

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd"))]
pub(crate) fn executable_path() -> Option<PathBuf> {
    #[cfg(target_os = "netbsd")]
    let mut mib = [
        libc::CTL_KERN,
        libc::KERN_PROC_ARGS,
        -1,
        libc::KERN_PROC_PATHNAME,
    ];
    #[cfg(not(target_os = "netbsd"))]
    let mut mib = [
        libc::CTL_KERN,
        libc::KERN_PROC,
        libc::KERN_PROC_PATHNAME,
        -1,
    ];
    let mut bytes = [0_u8; 1024];
    let mut length = bytes.len();
    // SAFETY: Both the MIB and output extent are valid for this synchronous
    // read-only sysctl query.
    if unsafe {
        libc::sysctl(
            mib.as_mut_ptr(),
            mib.len() as u32,
            bytes.as_mut_ptr().cast(),
            &raw mut length,
            core::ptr::null_mut(),
            0,
        )
    } < 0
    {
        return None;
    }
    let length = bytes[..length.min(bytes.len())]
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(length.min(bytes.len()));
    executable_path_from_bytes(&bytes[..length])
}

#[cfg(any(
    target_os = "linux",
    target_os = "cygwin",
    target_os = "android",
    target_os = "hurd",
    target_os = "macos",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "nto",
    target_os = "qnx",
    all(test, unix)
))]
pub(super) fn executable_path_from_bytes(bytes: &[u8]) -> Option<PathBuf> {
    if let Ok(path) = allocation::try_path(Path::new(OsStr::from_bytes(bytes))) {
        Some(path)
    } else {
        pending::mark_json_allocation_failed();
        None
    }
}

#[cfg(any(target_os = "nto", target_os = "qnx"))]
pub(crate) fn executable_path() -> Option<PathBuf> {
    executable_path_from_file(std::fs::File::open("/proc/self/exefile"))
}

#[cfg(any(target_os = "nto", target_os = "qnx", all(test, unix)))]
fn executable_path_from_file(file: std::io::Result<impl std::io::Read>) -> Option<PathBuf> {
    let mut file = match file {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == std::io::ErrorKind::OutOfMemory {
                pending::mark_json_allocation_failed();
            }
            return None;
        }
    };
    let mut bytes = [0_u8; 1024];
    let mut length = 0;
    while length < bytes.len() {
        match file.read(&mut bytes[length..]) {
            Ok(0) => return executable_path_from_bytes(&bytes[..length]),
            Ok(read) => length += read,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
            Err(error) => {
                if error.kind() == std::io::ErrorKind::OutOfMemory {
                    pending::mark_json_allocation_failed();
                }
                return None;
            }
        }
    }
    None
}

#[cfg(all(test, unix))]
mod tests {
    use super::executable_path_from_file;
    use crate::{allocation::fault, pending};
    use std::{
        io::{self, Read},
        path::Path,
    };

    struct Reader<'a> {
        bytes: &'a [u8],
        interrupted: bool,
        error: Option<io::ErrorKind>,
    }

    impl Read for Reader<'_> {
        fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
            if core::mem::take(&mut self.interrupted) {
                return Err(io::ErrorKind::Interrupted.into());
            }
            if let Some(error) = self.error.take() {
                return Err(error.into());
            }
            self.bytes.read(output)
        }
    }

    #[test]
    fn executable_file_open_and_read_errors_preserve_oom() {
        let _guard = fault::SWEEP_LOCK.lock().unwrap();
        for error in [
            io::ErrorKind::OutOfMemory,
            io::ErrorKind::NotFound,
            io::ErrorKind::PermissionDenied,
        ] {
            for opening in [true, false] {
                let result = pending::with_json_error_scope(|| {
                    let file = if opening {
                        Err(error.into())
                    } else {
                        Ok(Reader {
                            bytes: b"",
                            interrupted: true,
                            error: Some(error),
                        })
                    };
                    assert!(executable_path_from_file(file).is_none());
                    vk::VkResult::SUCCESS
                });
                assert_eq!(
                    result,
                    if error == io::ErrorKind::OutOfMemory {
                        vk::VkResult::ERROR_OUT_OF_HOST_MEMORY
                    } else {
                        vk::VkResult::SUCCESS
                    }
                );
            }
        }
    }

    #[test]
    fn executable_file_retries_interruption_and_guards_path_allocation() {
        fault::sweep_operation(|| {
            pending::with_json_error_scope(|| {
                let path = executable_path_from_file(Ok(Reader {
                    bytes: b"/bin/example",
                    interrupted: true,
                    error: None,
                }));
                if !pending::json_allocation_failed() {
                    assert_eq!(path.as_deref(), Some(Path::new("/bin/example")));
                }
                vk::VkResult::SUCCESS
            })
        });
    }
}

#[cfg(windows)]
pub(crate) fn executable_path() -> Option<PathBuf> {
    let mut units = [0_u16; 1024];
    // SAFETY: A null module requests the executable and `units` supplies the
    // exact writable extent passed to Win32.
    let length = unsafe {
        GetModuleFileNameW(
            core::ptr::null_mut(),
            units.as_mut_ptr(),
            units.len() as u32,
        )
    };
    if length == 0 || unsafe { GetLastError() } == ERROR_INSUFFICIENT_BUFFER {
        return None;
    }
    let length = length as usize;
    if length >= units.len() {
        return None;
    }
    executable_path_from_utf16(&units[..length])
}

#[cfg(windows)]
pub(super) fn executable_path_from_utf16(units: &[u16]) -> Option<PathBuf> {
    let result = (|| {
        // Upstream's fixed buffer is 1024 UTF-8 bytes, not UTF-16 units, and
        // must also hold the terminating NUL.
        if super::windows::paths::utf16_path_byte_length(units)? >= 1024 {
            return Ok(None);
        }
        super::windows::paths::path_from_utf16(units).map(Some)
    })();
    result.unwrap_or_else(|_: vk::VkResult| {
        crate::pending::mark_json_allocation_failed();
        None
    })
}

#[cfg(not(any(
    target_os = "android",
    target_os = "cygwin",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "hurd",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "nto",
    target_os = "openbsd",
    target_os = "qnx",
    target_vendor = "apple",
    windows,
)))]
pub(crate) fn executable_path() -> Option<PathBuf> {
    std::env::current_exe().ok()
}
