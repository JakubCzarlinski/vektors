//! Fallible Windows path preparation and normalization checks.

#[cfg(test)]
mod tests {
    use super::{finish_native_path, package_path_from_utf16};
    use std::{ffi::OsString, os::windows::ffi::OsStringExt as _, path::PathBuf};
    use windows_sys::Win32::Globalization::{CP_ACP, MultiByteToWideChar, WideCharToMultiByte};

    #[test]
    fn native_prefix_insertion_preserves_utf16_and_reports_growth_failure() {
        for (input, expected) in [
            (r"C:\雪", r"\\?\C:\雪"),
            (r"\\server\share\雪", r"\\?\UNC\server\share\雪"),
            (r"\\.\C:\雪", r"\\?\C:\雪"),
            (r"\\?\C:\雪", r"\\?\C:\雪"),
            (r"\??\C:\雪", r"\??\C:\雪"),
            (r"\雪", r"\雪"),
        ] {
            let input: Vec<_> = input.encode_utf16().chain([0]).collect();
            let expected: Vec<_> = expected.encode_utf16().chain([0]).collect();
            crate::allocation::fault::sweep_operation(|| {
                let mut buffer = Vec::new();
                if buffer.try_reserve_exact(input.len()).is_err() {
                    return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                }
                buffer.extend_from_slice(&input);
                match finish_native_path(buffer) {
                    Ok(actual) => {
                        assert_eq!(actual, expected);
                        vk::VkResult::SUCCESS
                    }
                    Err(error) => error,
                }
            });
        }
    }

    #[test]
    fn package_paths_match_native_ansi_conversion_and_report_oom() {
        for content in [
            vec![],
            vec![u16::from(b'a'); 259],
            vec![0xe9],
            vec![0x6c34],
            vec![0xd800],
            vec![0xd83e, 0xdd80],
        ] {
            let mut input = [0_u16; 260];
            input[..content.len()].copy_from_slice(&content);
            // SAFETY: Like upstream, query the size of a terminated input.
            let bytes_length = unsafe {
                WideCharToMultiByte(
                    CP_ACP,
                    0,
                    input.as_ptr(),
                    -1,
                    core::ptr::null_mut(),
                    0,
                    core::ptr::null(),
                    core::ptr::null_mut(),
                )
            };
            assert!(bytes_length > 0);
            let mut bytes = vec![0; bytes_length as usize];
            // SAFETY: The output has the queried size and input is unchanged.
            assert_eq!(
                unsafe {
                    WideCharToMultiByte(
                        CP_ACP,
                        0,
                        input.as_ptr(),
                        -1,
                        bytes.as_mut_ptr(),
                        bytes_length,
                        core::ptr::null(),
                        core::ptr::null_mut(),
                    )
                },
                bytes_length
            );
            let mut native = vec![0; bytes.len()];
            // SAFETY: The initialized bytes are terminated; one UTF-16 unit per
            // byte is enough for any system ANSI code page.
            let written = unsafe {
                MultiByteToWideChar(
                    CP_ACP,
                    0,
                    bytes.as_ptr(),
                    -1,
                    native.as_mut_ptr(),
                    bytes_length,
                )
            };
            assert!(written > 0);
            let expected = PathBuf::from(OsString::from_wide(&native[..written as usize - 1]));
            crate::allocation::fault::sweep_operation(|| match package_path_from_utf16(&input) {
                Ok(actual) => {
                    assert_eq!(actual.as_ref(), Some(&expected));
                    vk::VkResult::SUCCESS
                }
                Err(error) => error,
            });
        }
    }
}

use alloc::{string::String, vec::Vec};
use std::{
    fs::File,
    os::windows::ffi::OsStrExt as _,
    os::windows::io::FromRawHandle as _,
    path::{Path, PathBuf},
};
use vk::VkResult;
use windows_sys::Win32::{
    Foundation::{
        CloseHandle, ERROR_CANT_ACCESS_FILE, ERROR_NOT_ENOUGH_MEMORY, ERROR_OUTOFMEMORY,
        ERROR_SHARING_VIOLATION, GENERIC_READ, GetLastError, INVALID_HANDLE_VALUE,
    },
    Globalization::{CP_ACP, MultiByteToWideChar, WideCharToMultiByte},
    Storage::FileSystem::{
        CreateFileW, FILE_FLAG_BACKUP_SEMANTICS, FILE_SHARE_DELETE, FILE_SHARE_READ,
        FILE_SHARE_WRITE, GetFinalPathNameByHandleW, GetFullPathNameW, OPEN_EXISTING,
    },
};

pub(in crate::platform) fn utf16_path_byte_length(value: &[u16]) -> Result<usize, VkResult> {
    core::char::decode_utf16(value.iter().copied()).try_fold(0_usize, |length, character| {
        length
            .checked_add(
                character
                    .unwrap_or(core::char::REPLACEMENT_CHARACTER)
                    .len_utf8(),
            )
            .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)
    })
}

/// Matches upstream's `WideCharToMultiByte(CP_UTF8, 0)` path conversion.
pub(in crate::platform) fn path_from_utf16(value: &[u16]) -> Result<PathBuf, VkResult> {
    let length = utf16_path_byte_length(value)?;
    let mut path = String::new();
    path.try_reserve_exact(length)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for character in core::char::decode_utf16(value.iter().copied()) {
        path.push(character.unwrap_or(core::char::REPLACEMENT_CHARACTER));
    }
    // String -> OsString -> PathBuf transfers the UTF-8 buffer without a copy.
    Ok(PathBuf::from(path))
}

/// Package discovery supplies an ANSI path to upstream's narrow CRT directory
/// iterator. Preserve that code-page round trip before using a Rust Unicode path.
pub(in crate::platform) fn package_path_from_utf16(
    value: &[u16; 260],
) -> Result<Option<PathBuf>, VkResult> {
    let length = value
        .iter()
        .position(|unit| *unit == 0)
        .unwrap_or(value.len());
    if length == 0 {
        return Ok(Some(PathBuf::new()));
    }
    // Four bytes per input UTF-16 unit bounds the Windows ANSI encodings,
    // including machines whose system ANSI code page is UTF-8.
    let mut bytes = [0_u8; 1040];
    // SAFETY: The explicit input length is bounded by the array and the output
    // buffer has exactly the supplied extent. No terminator is required.
    let length = unsafe {
        WideCharToMultiByte(
            CP_ACP,
            0,
            value.as_ptr(),
            length as i32,
            bytes.as_mut_ptr(),
            1040,
            core::ptr::null(),
            core::ptr::null_mut(),
        )
    };
    if length == 0 {
        return native_failure().map(|_| None);
    }
    // Decoding cannot require more UTF-16 units than the encoded byte count.
    let mut units = [0_u16; 1040];
    // SAFETY: The previous call initialized `length` bytes; `units` supplies
    // enough writable space even for a single-byte code page.
    let length =
        unsafe { MultiByteToWideChar(CP_ACP, 0, bytes.as_ptr(), length, units.as_mut_ptr(), 1040) };
    if length == 0 {
        return native_failure().map(|_| None);
    }
    path_from_utf16(&units[..length as usize]).map(Some)
}

fn native_failure() -> Result<bool, VkResult> {
    // SAFETY: The caller invokes this immediately after a failed Windows call.
    match unsafe { GetLastError() } {
        ERROR_NOT_ENOUGH_MEMORY | ERROR_OUTOFMEMORY => Err(VkResult::ERROR_OUT_OF_HOST_MEMORY),
        _ => Ok(false),
    }
}

pub(in crate::platform) fn open_file(path: &Path) -> Result<Option<File>, VkResult> {
    let Some(path) = native_path(path)? else {
        return Ok(None);
    };
    // SAFETY: The path is terminated. These are File::open's read-only access,
    // sharing and creation flags, without its infallible path conversion.
    let handle = unsafe {
        CreateFileW(
            path.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            core::ptr::null(),
            OPEN_EXISTING,
            0,
            core::ptr::null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return native_failure().map(|_| None);
    }
    // SAFETY: Transfer the unique live handle to File, whose Drop closes it.
    // This conversion stores the handle directly and performs no allocation.
    Ok(Some(unsafe { File::from_raw_handle(handle) }))
}

pub(super) fn native_path(path: &Path) -> Result<Option<Vec<u16>>, VkResult> {
    let mut input = Vec::new();
    let capacity = path
        .as_os_str()
        .len()
        .checked_add(1)
        .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    // The encoded byte length bounds the UTF-16 length, including surrogate
    // code units. Reserve once for conversion and its terminating NUL.
    input
        .try_reserve_exact(capacity)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for unit in path.as_os_str().encode_wide() {
        if unit == 0 {
            return Ok(None);
        }
        input.push(unit);
    }
    input.push(0);
    let verbatim: &[u16] = &[92, 92, 63, 92];
    let nt: &[u16] = &[92, 63, 63, 92];
    let short_absolute = input.len() < 248
        && match input.as_slice() {
            [drive, 58, 0] | [drive, 58, 47 | 92, ..] => !matches!(*drive, 47 | 92),
            [47 | 92, 47 | 92, ..] => true,
            _ => false,
        };
    if input == [0] || input.starts_with(verbatim) || input.starts_with(nt) || short_absolute {
        return Ok(Some(input));
    }
    let mut absolute = Vec::new();
    let mut capacity = 512;
    loop {
        absolute
            .try_reserve_exact(capacity)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        // SAFETY: Input is terminated. Spare storage covers the supplied size;
        // the output length is used only after a successful, nontruncated call.
        let length = unsafe {
            GetFullPathNameW(
                input.as_ptr(),
                capacity as u32,
                absolute.as_mut_ptr(),
                core::ptr::null_mut(),
            )
        } as usize;
        if length == 0 {
            return native_failure().map(|_| None);
        }
        if length >= capacity {
            capacity = length;
            continue;
        }
        // SAFETY: The successful call initialized length characters plus NUL.
        unsafe { absolute.set_len(length + 1) };
        break;
    }
    finish_native_path(absolute).map(Some)
}

fn finish_native_path(mut absolute: Vec<u16>) -> Result<Vec<u16>, VkResult> {
    let (prefix, skip): (&[u16], usize) = match absolute.as_slice() {
        [_, 58, 92, ..] | [92, 92, 46, 92, ..] => {
            (&[92, 92, 63, 92], if absolute[0] == 92 { 4 } else { 0 })
        }
        [92, 92 | 63, 63, 92, ..] => (&[], 0),
        [92, 92, ..] => (&[92, 92, 63, 92, 85, 78, 67, 92], 2),
        _ => (&[], 0),
    };
    let original_length = absolute.len();
    let required = prefix
        .len()
        .checked_add(original_length - skip)
        .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    // Every prefix replacement above preserves or increases the length.
    // Reuse the native output buffer rather than grow and copy back into input.
    absolute
        .try_reserve_exact(required - original_length)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    // The reservation covers this resize; copy_within handles overlapping
    // prefix insertion and retains the native call's terminating NUL.
    absolute.resize(required, 0);
    absolute.copy_within(skip..original_length, prefix.len());
    absolute[..prefix.len()].copy_from_slice(prefix);
    Ok(absolute)
}

pub(crate) fn path_normalizes(path: &Path) -> Result<bool, VkResult> {
    let Some(path) = native_path(path)? else {
        return Ok(false);
    };
    // SAFETY: The input is terminated. This mirrors canonicalize's permission-
    // free open, including directory support and read/write/delete sharing.
    let handle = unsafe {
        CreateFileW(
            path.as_ptr(),
            0,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            core::ptr::null(),
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS,
            core::ptr::null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return native_failure();
    }
    // SAFETY: A null buffer with size zero queries the normalized name length.
    // We only need success, not an allocated copy of the resulting name.
    let length = unsafe { GetFinalPathNameByHandleW(handle, core::ptr::null_mut(), 0, 0) };
    let result = if length == 0 {
        native_failure()
    } else {
        Ok(true)
    };
    // SAFETY: This function owns the successfully opened handle.
    unsafe { CloseHandle(handle) };
    result
}

pub(in crate::platform) fn file_exists(path: &Path) -> Result<bool, VkResult> {
    let Some(path) = native_path(path)? else {
        return Ok(false);
    };
    // SAFETY: The converted path is terminated and remains live for this call.
    // Opening follows reparse points, unlike an attribute-only existence test.
    let handle = unsafe {
        CreateFileW(
            path.as_ptr(),
            0,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            core::ptr::null(),
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS,
            core::ptr::null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        // SAFETY: Query the error immediately after the failed native operation.
        return match unsafe { GetLastError() } {
            ERROR_SHARING_VIOLATION | ERROR_CANT_ACCESS_FILE => Ok(true),
            ERROR_NOT_ENOUGH_MEMORY | ERROR_OUTOFMEMORY => Err(VkResult::ERROR_OUT_OF_HOST_MEMORY),
            _ => Ok(false),
        };
    }
    // SAFETY: The handle was opened successfully and is owned by this function.
    unsafe { CloseHandle(handle) };
    Ok(true)
}
