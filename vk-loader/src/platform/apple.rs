use core::ffi::{c_char, c_void};
use std::os::unix::ffi::OsStringExt as _;

type CFBundleRef = *const c_void;
type CFTypeRef = *const c_void;
type CFURLRef = *const c_void;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFBundleGetMainBundle() -> CFBundleRef;
    fn CFBundleCopyResourcesDirectoryURL(bundle: CFBundleRef) -> CFURLRef;
    fn CFRelease(value: CFTypeRef);
    fn CFURLGetFileSystemRepresentation(
        url: CFURLRef,
        resolve_against_base: u8,
        buffer: *mut c_char,
        max_buffer_length: isize,
    ) -> u8;
}

pub(super) fn resource_directory() -> Option<std::path::PathBuf> {
    // Upstream starts at MAXPATHLEN and retries with progressively larger
    // storage. Keep the same bounded retry contract without putting the
    // scratch buffer on a Vulkan entry point's stack.
    let bundle = unsafe { CFBundleGetMainBundle() };
    if bundle.is_null() {
        return None;
    }
    let url = unsafe { CFBundleCopyResourcesDirectoryURL(bundle) };
    if url.is_null() {
        return None;
    }

    let result = (0..4).find_map(|attempt| {
        let capacity = 1024_usize << attempt;
        let mut bytes = Vec::<u8>::with_capacity(capacity);
        // SAFETY: `bytes` provides `capacity` writable bytes. Core
        // Foundation writes a NUL-terminated file-system representation
        // on success and does not retain the buffer.
        let success = unsafe {
            CFURLGetFileSystemRepresentation(url, 1, bytes.as_mut_ptr().cast(), capacity as isize)
        };
        if success == 0 {
            return None;
        }
        // SAFETY: A successful Core Foundation call initialized a C
        // string within the supplied capacity.
        let length = unsafe {
            core::ffi::CStr::from_ptr(bytes.as_ptr().cast())
                .to_bytes()
                .len()
        };
        // SAFETY: The C string initialized exactly `length` non-NUL bytes.
        unsafe { bytes.set_len(length) };
        Some(std::path::PathBuf::from(std::ffi::OsString::from_vec(
            bytes,
        )))
    });

    // SAFETY: The Copy function returned an owned Core Foundation object.
    unsafe { CFRelease(url) };
    result
}
