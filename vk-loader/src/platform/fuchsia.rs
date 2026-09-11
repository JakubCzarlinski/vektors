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

fn dynamic_error() -> Result<String, vk::VkResult> {
    // SAFETY: `dlerror` returns either NULL or a thread-local C string.
    let error = unsafe { libc::dlerror() };
    if error.is_null() {
        crate::debug::diagnostics::try_format(format_args!("dlopen_vmo failed"))
    } else {
        // SAFETY: A non-null dlerror result is NUL-terminated for this call.
        let bytes = unsafe { CStr::from_ptr(error) }.to_bytes();
        crate::debug::diagnostics::try_format(format_args!(
            "{}",
            crate::debug::diagnostics::LossyBytes(&bytes[..bytes.len().min(127)])
        ))
    }
}

pub(super) unsafe fn open(path: &Path, driver: bool) -> Result<LoaderLibrary, OpenLibraryError> {
    if !driver {
        // Fuchsia layers are normally in the application's namespace.
        // SAFETY: The caller owns the foreign initialization contract.
        match unsafe { LoaderLibrary::open_unix(path) } {
            Ok(library) => return Ok(library),
            Err(error) if error.message.is_err() => return Err(error),
            Err(_) => {}
        }
    }

    let service = loader_service();
    if service == ZX_HANDLE_INVALID {
        return Err(OpenLibraryError {
            message: crate::debug::diagnostics::try_format(format_args!(
                "libvulkan.so:dlopen_fuchsia: no connection to loader svc\n"
            )),
        });
    }
    let name_bytes = path.as_os_str().as_bytes();
    let name_length = name_bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(name_bytes.len());
    let mut vmo = ZX_HANDLE_INVALID;
    // SAFETY: The channel is process-global and live, the name is readable,
    // and `vmo` is writable for the returned handle.
    let status = unsafe {
        fuchsia_vulkan_loader_LoaderGet(
            service,
            name_bytes.as_ptr().cast(),
            name_length,
            &raw mut vmo,
        )
    };
    if status != ZX_OK {
        return Err(OpenLibraryError {
            message: crate::debug::diagnostics::try_format(format_args!(
                "libvulkan.so:dlopen_fuchsia: Get() failed: {status}\n"
            )),
        });
    }
    if vmo == ZX_HANDLE_INVALID {
        return Err(OpenLibraryError {
            message: crate::debug::diagnostics::try_format(format_args!(
                "libvulkan.so:dlopen_fuchsia: Get() returned invalid vmo\n"
            )),
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
    let handle = core::ptr::NonNull::new(handle).ok_or(OpenLibraryError {
        message: Err(vk::VkResult::ERROR_INITIALIZATION_FAILED),
    })?;
    Ok(LoaderLibrary(handle))
}
