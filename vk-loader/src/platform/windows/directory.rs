//! Native manifest enumeration with fallible path storage.

#[cfg(test)]
mod tests {
    use super::{manifest_entry, read_manifest_directory};
    use std::{
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn manifest_entry_conversion_matches_owned_paths_and_reports_oom() {
        for name in [
            vec![],
            ".json".encode_utf16().collect(),
            "skip.JSON".encode_utf16().collect(),
            "雪🦀.json".encode_utf16().collect(),
            ".hidden.json".encode_utf16().collect(),
            [vec![0xd800], ".json".encode_utf16().collect()].concat(),
            [vec![0x6c34; 255], ".json".encode_utf16().collect()].concat(),
        ] {
            let mut native = [0; 260];
            native[..name.len()].copy_from_slice(&name);
            let converted = super::super::paths::path_from_utf16(&name).unwrap();
            for base in ["relative", r"C:\directory", r"\\?\C:\directory\."] {
                let base = Path::new(base);
                let expected = converted
                    .extension()
                    .is_some_and(|extension| extension == "json")
                    .then(|| base.join(&converted));
                crate::allocation::fault::sweep_operation(|| match manifest_entry(base, &native) {
                    Ok(actual) => {
                        assert_eq!(
                            actual.as_deref().map(Path::as_os_str),
                            expected.as_deref().map(Path::as_os_str)
                        );
                        vk::VkResult::SUCCESS
                    }
                    Err(error) => error,
                });
            }
        }
    }

    #[test]
    fn directory_paths_match_std_and_release_storage_on_allocation_failure() {
        let root = Path::new("target/test/allocations").join(format!(
            "windows-directory-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        for name in ["first.json", "雪.json", ".json", "skip.JSON", "skip.txt"] {
            std::fs::write(root.join(name), b"{}").unwrap();
        }
        std::fs::create_dir(root.join("directory.json")).unwrap();
        std::fs::create_dir(root.join("empty")).unwrap();
        std::fs::write(root.join("empty.json"), []).unwrap();
        std::fs::write(root.join("large.json"), vec![b'x'; 9000]).unwrap();
        for path in [
            root.clone(),
            std::fs::canonicalize(&root).unwrap(),
            root.join("empty"),
            root.join("missing"),
            root.join("first.json"),
            PathBuf::new(),
        ] {
            let expected = std::fs::read_dir(&path).ok().map(|entries| {
                let mut entries: Vec<_> = entries
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .filter(|path| {
                        path.extension()
                            .is_some_and(|extension| extension == "json")
                    })
                    .collect();
                entries.sort_unstable();
                entries
            });
            crate::allocation::fault::sweep_operation(|| match read_manifest_directory(&path) {
                Ok(mut actual) => {
                    if let Some(entries) = &mut actual {
                        entries.sort_unstable();
                    }
                    assert_eq!(actual, expected, "{}", path.display());
                    vk::VkResult::SUCCESS
                }
                Err(error) => error,
            });
        }
        for path in [
            root.join("first.json"),
            root.join("雪.json"),
            root.join("empty.json"),
            root.join("large.json"),
            root.join("missing.json"),
            root.clone(),
            std::fs::canonicalize(root.join("first.json")).unwrap(),
        ] {
            let expected = std::fs::read(&path).ok();
            crate::allocation::fault::sweep_operation(|| {
                crate::pending::with_json_error_scope(|| {
                    let actual = crate::platform::read_file(&path);
                    if crate::pending::json_allocation_failed() {
                        return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                    }
                    assert_eq!(actual.as_deref(), expected.as_deref(), "{}", path.display());
                    vk::VkResult::SUCCESS
                })
            });
        }
        // This uniquely named directory contains only this test's fixtures.
        std::fs::remove_dir_all(root).unwrap();
    }
}

use super::paths::native_path;
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use vk::VkResult;
use windows_sys::Win32::{
    Foundation::{
        ERROR_FILE_NOT_FOUND, ERROR_NOT_ENOUGH_MEMORY, ERROR_OUTOFMEMORY, GetLastError, HANDLE,
        INVALID_HANDLE_VALUE,
    },
    Storage::FileSystem::{FindClose, FindFirstFileW, FindNextFileW, WIN32_FIND_DATAW},
};

struct DirectoryHandle(HANDLE);

impl Drop for DirectoryHandle {
    fn drop(&mut self) {
        // SAFETY: This owner is created only from a successful find call.
        unsafe { FindClose(self.0) };
    }
}

fn allocation_error(error: u32) -> Result<(), VkResult> {
    match error {
        ERROR_NOT_ENOUGH_MEMORY | ERROR_OUTOFMEMORY => Err(VkResult::ERROR_OUT_OF_HOST_MEMORY),
        _ => Ok(()),
    }
}

/// Converts a bounded native filename directly into the retained full path.
fn manifest_entry(path: &Path, name: &[u16; 260]) -> Result<Option<PathBuf>, VkResult> {
    let length = name
        .iter()
        .position(|unit| *unit == 0)
        .unwrap_or(name.len());
    let name = &name[..length];
    // A leading-dot-only name has no extension, matching Path::extension.
    if name.len() <= 5 || !name.ends_with(&b".json".map(u16::from)) {
        return Ok(None);
    }
    // A UTF-16 unit needs at most three UTF-8 bytes, including replacement
    // characters. A surrogate pair needs only four bytes for its two units.
    let mut utf8 = [0; 260 * 3];
    let mut written = 0;
    for character in core::char::decode_utf16(name.iter().copied()) {
        written += character
            .unwrap_or(core::char::REPLACEMENT_CHARACTER)
            .encode_utf8(&mut utf8[written..])
            .len();
    }
    // SAFETY: Every byte in this prefix was written by char::encode_utf8;
    // complete encodings were concatenated without gaps or truncation.
    let name = unsafe { core::str::from_utf8_unchecked(&utf8[..written]) };
    crate::allocation::try_join_path(path, Path::new(name)).map(Some)
}

pub(in crate::platform) fn read_manifest_directory(
    path: &Path,
) -> Result<Option<Vec<PathBuf>>, VkResult> {
    if path.as_os_str().is_empty() {
        return Ok(None);
    }
    let pattern = crate::allocation::try_join_path(path, Path::new("*"))?;
    let Some(pattern) = native_path(&pattern)? else {
        return Ok(None);
    };
    let mut data = MaybeUninit::<WIN32_FIND_DATAW>::uninit();
    // SAFETY: The pattern is terminated, and data is writable for the full
    // native structure. It is read only after a successful find operation.
    let handle = unsafe { FindFirstFileW(pattern.as_ptr(), data.as_mut_ptr()) };
    if handle == INVALID_HANDLE_VALUE {
        // SAFETY: Read the error immediately after the failed call.
        let error = unsafe { GetLastError() };
        allocation_error(error)?;
        return Ok((error == ERROR_FILE_NOT_FOUND).then(Vec::new));
    }
    let handle = DirectoryHandle(handle);
    let mut entries = Vec::new();
    loop {
        // SAFETY: FindFirstFileW or FindNextFileW initialized the entire data
        // structure. The borrow ends before the next native write.
        let name = &unsafe { data.assume_init_ref() }.cFileName;
        if let Some(entry) = manifest_entry(path, name)? {
            crate::allocation::try_push(&mut entries, entry)?;
        }
        // SAFETY: The search handle remains owned and data is fully writable.
        if unsafe { FindNextFileW(handle.0, data.as_mut_ptr()) } == 0 {
            // SAFETY: No intervening native call has changed last-error.
            allocation_error(unsafe { GetLastError() })?;
            break;
        }
    }
    Ok(Some(entries))
}
