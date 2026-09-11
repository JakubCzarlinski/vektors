//! Filesystem access and manifest enumeration.

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::{directory_entry, mark_native_allocation_failure};
    use crate::pending;
    use std::{ffi::OsStr, os::unix::ffi::OsStrExt as _, path::Path};

    #[test]
    fn json_paths_preserve_component_and_byte_semantics() {
        for (bytes, expected) in [
            (b"a.json".as_slice(), true),
            (b"a.json//./", true),
            (b"./a.json/.", true),
            (b"//a.json", true),
            (b"\xff.json", true),
            (b".json", false),
            (b"a.JSON", false),
            (b"a.json/../", false),
            (b"a.json/b", false),
            (b"/./", false),
            (b"", false),
        ] {
            let path = Path::new(OsStr::from_bytes(bytes));
            assert_eq!(super::is_json_path(path), expected, "{path:?}");
            assert_eq!(
                super::is_json_path(path),
                path.file_name()
                    .is_some_and(|name| super::is_json_name(name.as_bytes()))
            );
        }
    }

    #[test]
    fn directory_eof_and_errors_are_distinct() {
        let _guard = crate::allocation::fault::SWEEP_LOCK.lock().unwrap();
        for error in [0, libc::ENOMEM, libc::EIO, libc::EACCES] {
            let result = pending::with_json_error_scope(|| {
                errno::set_errno(errno::Errno(libc::ENOMEM));
                let entry = directory_entry::<u8>(|| {
                    assert_eq!(errno::errno().0, 0);
                    errno::set_errno(errno::Errno(error));
                    core::ptr::null_mut()
                });
                assert_eq!(entry, if error == 0 { Ok(None) } else { Err(()) });
                vk::VkResult::SUCCESS
            });
            assert_eq!(
                result,
                if error == libc::ENOMEM {
                    vk::VkResult::ERROR_OUT_OF_HOST_MEMORY
                } else {
                    vk::VkResult::SUCCESS
                }
            );
        }
    }

    #[test]
    fn native_memory_errors_are_not_confused_with_missing_files() {
        let _guard = crate::allocation::fault::SWEEP_LOCK.lock().unwrap();
        for (error, out_of_memory) in [
            (libc::ENOMEM, true),
            (libc::ENOENT, false),
            (libc::EACCES, false),
            (0, false),
        ] {
            pending::with_json_error_scope(|| {
                // SAFETY: errno is writable thread-local storage. Restore it
                // immediately after the operation, before running assertions.
                unsafe {
                    let errno = libc::__errno_location();
                    let previous = *errno;
                    *errno = error;
                    mark_native_allocation_failure();
                    *errno = previous;
                }
                assert_eq!(pending::json_allocation_failed(), out_of_memory);
                vk::VkResult::SUCCESS
            });
        }
    }
}

use crate::{allocation, pending};

#[cfg(unix)]
use super::{access, closedir, fopen, opendir, readdir};
#[cfg(unix)]
use core::ffi::{CStr, c_void};
#[cfg(not(unix))]
use std::io::Read as _;
use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::{ffi::OsStr, os::unix::ffi::OsStrExt as _};

#[cfg(unix)]
pub(super) fn with_c_path<T>(path: &Path, operation: impl FnOnce(&CStr) -> T) -> Option<T> {
    const STACK_CAPACITY: usize = 256;

    let bytes = path.as_os_str().as_encoded_bytes();
    if bytes.contains(&0) {
        core::hint::cold_path();
        return None;
    }
    if bytes.len() < STACK_CAPACITY {
        let mut storage = [0_u8; STACK_CAPACITY];
        storage[..bytes.len()].copy_from_slice(bytes);
        // SAFETY: The copied path contains no interior NUL and zero-filled
        // storage supplies the terminator immediately after it.
        let path = unsafe { CStr::from_bytes_with_nul_unchecked(&storage[..=bytes.len()]) };
        return Some(operation(path));
    }
    let Some(length) = bytes.len().checked_add(1) else {
        pending::mark_json_allocation_failed();
        return None;
    };
    let Ok(mut storage) = allocation::try_boxed_slice_filled(length, 0_u8) else {
        pending::mark_json_allocation_failed();
        return None;
    };
    storage[..bytes.len()].copy_from_slice(bytes);
    // SAFETY: Interior NULs were rejected above, and the last byte remains NUL.
    let path = unsafe { CStr::from_bytes_with_nul_unchecked(&storage) };
    Some(operation(path))
}

/// Checks path normalization without retaining a normalized path.
#[cfg(unix)]
pub(crate) fn path_normalizes(path: &Path) -> Result<bool, vk::VkResult> {
    if path.as_os_str().as_encoded_bytes().contains(&0) {
        return Ok(false);
    }
    with_c_path(path, |path| {
        // SAFETY: The input is terminated and live. A null output requests
        // libc-owned storage, which is released below without making a Rust copy.
        let normalized = unsafe { libc::realpath(path.as_ptr(), core::ptr::null_mut()) };
        if normalized.is_null() {
            return if std::io::Error::last_os_error().raw_os_error() == Some(libc::ENOMEM) {
                Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)
            } else {
                Ok(false)
            };
        }
        // SAFETY: A successful realpath(NULL) returns a malloc-owned buffer.
        unsafe { libc::free(normalized.cast()) };
        Ok(true)
    })
    .ok_or(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)?
}

#[cfg(windows)]
pub(crate) fn path_normalizes(path: &Path) -> Result<bool, vk::VkResult> {
    super::windows::paths::path_normalizes(path)
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn path_normalizes(path: &Path) -> Result<bool, vk::VkResult> {
    Ok(std::fs::canonicalize(path).is_ok())
}

/// Reads a file through the C runtime.
///
/// The upstream test harness interposes the platform C API to provide an
/// isolated filesystem. Using `fopen` here is consequently part of parity,
/// rather than an interchangeable implementation detail.
#[cfg(unix)]
pub(crate) fn read_file(path: &Path) -> Option<Box<[u8]>> {
    // SAFETY: Both strings are NUL-terminated and live for the call.
    let file = with_c_path(path, |path| unsafe {
        fopen()(path.as_ptr(), c"rb".as_ptr())
    })?;
    if file.is_null() {
        mark_native_allocation_failure();
        return None;
    }

    // The manifest extent is known before the single full-file read below.
    // Avoid libc allocating and copying through a separate `FILE` buffer.
    // SAFETY: No I/O has been performed on this newly opened stream.
    unsafe {
        libc::setvbuf(file, core::ptr::null_mut(), libc::_IONBF, 0);
    }

    let result = (|| {
        // Match upstream's `fstat(fileno(file))` path. Besides avoiding two
        // seeks per manifest, this deliberately rejects streams for which the
        // loader cannot establish an exact extent before allocating.
        // SAFETY: `file` is an open C stream owned by this function.
        let descriptor = unsafe { libc::fileno(file) };
        if descriptor < 0 {
            return None;
        }
        let mut metadata = core::mem::MaybeUninit::<libc::stat>::uninit();
        // SAFETY: `metadata` provides the writable `stat` extent required by
        // `fstat`; it is only assumed initialized after a successful call.
        if unsafe { libc::fstat(descriptor, metadata.as_mut_ptr()) } != 0 {
            mark_native_allocation_failure();
            return None;
        }
        // SAFETY: The preceding `fstat` call initialized the complete object.
        let metadata = unsafe { metadata.assume_init() };
        if metadata.st_size < 0 {
            return None;
        }
        let length = metadata.st_size as usize;
        if length as libc::off_t != metadata.st_size {
            pending::mark_json_allocation_failed();
            return None;
        }
        let Ok(mut bytes) = allocation::try_box_uninit_slice::<u8>(length) else {
            pending::mark_json_allocation_failed();
            return None;
        };
        // SAFETY: `bytes` has writable capacity for exactly `length` bytes.
        let read = unsafe { libc::fread(bytes.as_mut_ptr().cast::<c_void>(), 1, length, file) };
        if read != length {
            core::hint::cold_path();
            // SAFETY: The stream is still owned and open. EOF is not an error;
            // only consult errno when fread actually set the error indicator.
            if unsafe { libc::ferror(file) } != 0 {
                mark_native_allocation_failure();
            }
            return None;
        }
        // SAFETY: `fread` initialized every element when it returned `length`.
        Some(unsafe { bytes.assume_init() })
    })();

    // SAFETY: This function owns the open stream.
    unsafe { libc::fclose(file) };
    result
}

#[cfg(not(unix))]
pub(crate) fn read_file(path: &Path) -> Option<Box<[u8]>> {
    #[cfg(windows)]
    let mut file = super::windows::paths::open_file(path).unwrap_or_else(|_| {
        pending::mark_json_allocation_failed();
        None
    })?;
    #[cfg(not(windows))]
    let mut file = std::fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 4096];
    loop {
        let length = match file.read(&mut chunk) {
            Ok(0) => break,
            Ok(length) => length,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) if error.kind() == std::io::ErrorKind::OutOfMemory => {
                pending::mark_json_allocation_failed();
                return None;
            }
            Err(_) => return None,
        };
        if bytes.try_reserve(length).is_err() {
            pending::mark_json_allocation_failed();
            return None;
        }
        bytes.extend_from_slice(&chunk[..length]);
    }
    match allocation::try_into_boxed_slice(bytes) {
        Ok(bytes) => Some(bytes),
        Err(_) => {
            pending::mark_json_allocation_failed();
            None
        }
    }
}

/// Tests whether a file exists through the platform API used by upstream.
///
/// Besides matching Vulkan-Loader's discovery semantics, using `access` is
/// required for filesystem interposition in applications such as its original
/// test harness.
#[cfg(unix)]
pub(crate) fn file_exists(path: &Path) -> bool {
    // SAFETY: `path` is NUL-terminated and live for the call; `F_OK` only
    // performs an existence check.
    with_c_path(path, |path| unsafe {
        access()(path.as_ptr(), libc::F_OK) == 0
    })
    .unwrap_or(false)
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn file_exists(path: &Path) -> bool {
    path.exists()
}

#[cfg(windows)]
pub(crate) fn file_exists(path: &Path) -> bool {
    super::windows::paths::file_exists(path).unwrap_or_else(|_| {
        pending::mark_json_allocation_failed();
        false
    })
}

pub(crate) fn manifest_files(path: &Path) -> Vec<PathBuf> {
    if is_json_path(path) {
        let mut paths = Vec::new();
        let mut owned = PathBuf::new();
        if paths.try_reserve_exact(1).is_err()
            || owned.try_reserve_exact(path.as_os_str().len()).is_err()
        {
            pending::mark_json_allocation_failed();
            return paths;
        }
        owned.push(path);
        paths.push(owned);
        return paths;
    }

    read_manifest_directory(path).unwrap_or_default()
}

#[cfg(unix)]
pub(crate) fn is_json_path(path: &Path) -> bool {
    // Unix components discard trailing separators and current-directory
    // components. A parent component cannot have the required JSON suffix.
    path.as_os_str()
        .as_bytes()
        .rsplit(|byte| *byte == b'/')
        .find(|component| !component.is_empty() && *component != b".")
        .is_some_and(is_json_name)
}

#[cfg(unix)]
fn is_json_name(name: &[u8]) -> bool {
    name.len() > b".json".len() && name.ends_with(b".json")
}

#[cfg(not(unix))]
pub(crate) fn is_json_path(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension == "json")
}

#[cfg(unix)]
fn read_manifest_directory(path: &Path) -> Option<Vec<PathBuf>> {
    // SAFETY: The converted path is live and NUL-terminated for the call.
    let directory = with_c_path(path, |path| unsafe { opendir()(path.as_ptr()) })?;
    if directory.is_null() {
        mark_native_allocation_failure();
        return None;
    }
    let result = (|| {
        let mut entries = Vec::new();
        // SAFETY: `directory` remains open and access is serialized by this function.
        while let Some(entry) = directory_entry(|| unsafe { readdir()(directory) }).ok()? {
            // SAFETY: POSIX guarantees a NUL-terminated `d_name` in a live dirent.
            let name = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
            if is_json_name(name) {
                let Some(length) = path
                    .as_os_str()
                    .len()
                    .checked_add(name.len())
                    .and_then(|length| length.checked_add(1))
                else {
                    pending::mark_json_allocation_failed();
                    return None;
                };
                let mut entry_path = PathBuf::new();
                if entries.try_reserve(1).is_err() || entry_path.try_reserve_exact(length).is_err()
                {
                    pending::mark_json_allocation_failed();
                    return None;
                }
                // A dirent name is one relative component on Unix. Append its
                // bytes directly instead of reparsing both paths in PathBuf::push.
                let bytes = entry_path.as_mut_os_string();
                bytes.push(path.as_os_str());
                if !path.as_os_str().is_empty() && !path.as_os_str().as_bytes().ends_with(b"/") {
                    bytes.push(OsStr::from_bytes(b"/"));
                }
                bytes.push(OsStr::from_bytes(name));
                entries.push(entry_path);
            }
        }
        Some(entries)
    })();
    // SAFETY: This function owns the open directory stream.
    unsafe { closedir()(directory) };
    result
}

#[cfg(unix)]
fn directory_entry<T>(read: impl FnOnce() -> *mut T) -> Result<Option<*mut T>, ()> {
    // readdir leaves errno unchanged at EOF. Clear it immediately before the
    // call, so a previous filesystem or allocation failure cannot look like EOF failure.
    #[cfg(not(target_os = "fuchsia"))]
    errno::set_errno(errno::Errno(0));
    #[cfg(target_os = "fuchsia")]
    // SAFETY: libc exposes writable errno storage local to the current thread.
    unsafe {
        *libc::__errno_location() = 0
    };
    let entry = read();
    if !entry.is_null() {
        return Ok(Some(entry));
    }
    match std::io::Error::last_os_error().raw_os_error() {
        Some(0) => Ok(None),
        Some(libc::ENOMEM) => {
            pending::mark_json_allocation_failed();
            Err(())
        }
        _ => Err(()),
    }
}

#[cfg(unix)]
fn mark_native_allocation_failure() {
    if std::io::Error::last_os_error().raw_os_error() == Some(libc::ENOMEM) {
        pending::mark_json_allocation_failed();
    }
}

#[cfg(windows)]
fn read_manifest_directory(path: &Path) -> Option<Vec<PathBuf>> {
    super::windows::directory::read_manifest_directory(path).unwrap_or_else(|_| {
        pending::mark_json_allocation_failed();
        None
    })
}

#[cfg(not(any(unix, windows)))]
fn read_manifest_directory(path: &Path) -> Option<Vec<PathBuf>> {
    let mut entries = Vec::new();
    for entry in path.read_dir().ok()?.filter_map(Result::ok) {
        let path = entry.path();
        if is_json_path(&path) && allocation::try_push(&mut entries, path).is_err() {
            pending::mark_json_allocation_failed();
            return None;
        }
    }
    Some(entries)
}
