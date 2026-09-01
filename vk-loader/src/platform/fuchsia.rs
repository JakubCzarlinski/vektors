use alloc::ffi::CString;
use core::ffi::{CStr, c_char, c_int, c_void};
use std::{os::unix::ffi::OsStrExt as _, path::Path, sync::OnceLock};

use super::{LoaderLibrary, OpenLibraryError};

type ZxHandle = u32;
type ZxStatus = i32;

const ZX_HANDLE_INVALID: ZxHandle = 0;
const ZX_OK: ZxStatus = 0;

#[link(name = "fdio")]
unsafe extern "C" {
    fn fdio_service_connect(path: *const c_char, channel: ZxHandle) -> ZxStatus;
}

unsafe extern "C" {
    fn zx_channel_create(
        options: u32,
        endpoint_0: *mut ZxHandle,
        endpoint_1: *mut ZxHandle,
    ) -> ZxStatus;
    fn zx_handle_close(handle: ZxHandle) -> ZxStatus;
    fn dlopen_vmo(vmo: ZxHandle, mode: c_int) -> *mut c_void;
    fn fuchsia_vulkan_loader_LoaderGet(
        channel: ZxHandle,
        name: *const c_char,
        name_size: usize,
        out_vmo: *mut ZxHandle,
    ) -> ZxStatus;
}

fn loader_service() -> ZxHandle {
    static SERVICE: OnceLock<ZxHandle> = OnceLock::new();
    *SERVICE.get_or_init(|| {
        let mut local = ZX_HANDLE_INVALID;
        let mut remote = ZX_HANDLE_INVALID;
        // SAFETY: Both output handles are writable and initialized on success.
        if unsafe { zx_channel_create(0, &raw mut local, &raw mut remote) } != ZX_OK {
            return ZX_HANDLE_INVALID;
        }
        // SAFETY: The service path is static and the channel endpoint is
        // transferred to `fdio_service_connect`.
        if unsafe { fdio_service_connect(c"/svc/fuchsia.vulkan.loader.Loader".as_ptr(), local) }
            != ZX_OK
        {
            // SAFETY: `remote` remains owned locally after connect fails.
            let _ = unsafe { zx_handle_close(remote) };
            return ZX_HANDLE_INVALID;
        }
        remote
    })
}

fn dynamic_error() -> String {
    // SAFETY: `dlerror` returns either NULL or a thread-local C string.
    let error = unsafe { libc::dlerror() };
    if error.is_null() {
        "dlopen_vmo failed".to_owned()
    } else {
        // SAFETY: A non-null dlerror result is NUL-terminated for this call.
        let bytes = unsafe { CStr::from_ptr(error) }.to_bytes();
        String::from_utf8_lossy(&bytes[..bytes.len().min(127)]).into_owned()
    }
}

pub(super) unsafe fn open(path: &Path, driver: bool) -> Result<LoaderLibrary, OpenLibraryError> {
    if !driver {
        // Fuchsia layers are normally in the application's namespace.
        // SAFETY: The caller owns the foreign initialization contract.
        if let Ok(library) = unsafe { libloading::Library::new(path.as_os_str()) } {
            return Ok(LoaderLibrary(core::mem::ManuallyDrop::new(library)));
        }
    }

    let service = loader_service();
    if service == ZX_HANDLE_INVALID {
        return Err(OpenLibraryError {
            message: "libvulkan.so:dlopen_fuchsia: no connection to loader svc\n".to_owned(),
        });
    }
    let name_bytes = path.as_os_str().as_bytes();
    let name_length = name_bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(name_bytes.len());
    // The prefix cannot contain NUL because `name_length` selects the first.
    let name = unsafe {
        CString::from_vec_with_nul_unchecked(
            name_bytes[..name_length]
                .iter()
                .copied()
                .chain(core::iter::once(0))
                .collect(),
        )
    };
    let mut vmo = ZX_HANDLE_INVALID;
    // SAFETY: The channel is process-global and live, the name is readable,
    // and `vmo` is writable for the returned handle.
    let status = unsafe {
        fuchsia_vulkan_loader_LoaderGet(service, name.as_ptr(), name.as_bytes().len(), &raw mut vmo)
    };
    if status != ZX_OK {
        return Err(OpenLibraryError {
            message: format!("libvulkan.so:dlopen_fuchsia: Get() failed: {status}\n"),
        });
    }
    if vmo == ZX_HANDLE_INVALID {
        return Err(OpenLibraryError {
            message: "libvulkan.so:dlopen_fuchsia: Get() returned invalid vmo\n".to_owned(),
        });
    }
    // SAFETY: The service returned a VMO intended for dynamic loading.
    let handle = unsafe { dlopen_vmo(vmo, libc::RTLD_LAZY | libc::RTLD_LOCAL) };
    // SAFETY: Ownership of the returned VMO remains with this function.
    let _ = unsafe { zx_handle_close(vmo) };
    if handle.is_null() {
        return Err(OpenLibraryError {
            message: dynamic_error(),
        });
    }
    // SAFETY: `handle` is a successful `dlopen_vmo` result transferred to
    // libloading's unique Unix module owner.
    let library = unsafe { libloading::os::unix::Library::from_raw(handle) };
    Ok(LoaderLibrary(core::mem::ManuallyDrop::new(library.into())))
}
