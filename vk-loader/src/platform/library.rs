//! Dynamic library ownership, symbol lookup, and module paths.

#[cfg(unix)]
use super::filesystem::with_c_path;
#[cfg(target_os = "fuchsia")]
use super::fuchsia;
use super::{dynamic_library_unloading_disabled, initialize_loader};
use core::{
    ffi::{CStr, c_void},
    marker::PhantomData,
    ptr::NonNull,
};
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt as _;
use std::path::{Path, PathBuf};
#[cfg(all(unix, not(any(target_os = "fuchsia", target_os = "nto"))))]
use std::{ffi::OsStr, os::unix::ffi::OsStrExt as _};
#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{ERROR_MOD_NOT_FOUND, GetLastError},
    System::LibraryLoader::{
        GetProcAddress, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
        LoadLibraryExW, LoadLibraryW,
    },
};

/// A dynamic module with Vulkan-Loader's process-wide unloading policy.
pub(crate) struct LoaderLibrary(pub(super) NonNull<c_void>);

// SAFETY: Module ownership can move between threads; symbol lookup and module
// reference counting are synchronized by the supported operating-system loaders.
unsafe impl Send for LoaderLibrary {}
// SAFETY: Shared access only performs OS-synchronized symbol lookup. Unloading
// requires ownership, and returned symbols borrow this module owner.
unsafe impl Sync for LoaderLibrary {}

pub(crate) struct LibrarySymbol<'a, T> {
    value: T,
    library: PhantomData<&'a LoaderLibrary>,
}

impl<T> core::ops::Deref for LibrarySymbol<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

pub(crate) struct OpenLibraryError {
    #[cfg(windows)]
    code: Result<u32, vk::VkResult>,
    #[cfg(not(windows))]
    pub(super) message: Result<String, vk::VkResult>,
}

impl OpenLibraryError {
    fn initialization(error: vk::VkResult) -> Self {
        Self {
            #[cfg(windows)]
            code: Err(error),
            #[cfg(not(windows))]
            message: Err(error),
        }
    }

    pub(crate) fn is_out_of_memory(&self) -> bool {
        #[cfg(windows)]
        {
            matches!(
                self.code,
                Ok(windows_sys::Win32::Foundation::ERROR_NOT_ENOUGH_MEMORY
                    | windows_sys::Win32::Foundation::ERROR_OUTOFMEMORY)
                    | Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)
            )
        }
        #[cfg(not(windows))]
        {
            matches!(self.message, Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY))
        }
    }

    pub(crate) fn into_message(self, path: &Path) -> Result<String, vk::VkResult> {
        #[cfg(windows)]
        {
            use crate::debug::diagnostics;

            if self.is_out_of_memory() {
                return Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY);
            }
            let code = self.code?;
            diagnostics::try_format(format_args!(
                "Failed to open dynamic library \"{}\" with error {}",
                path.display(),
                code
            ))
        }
        #[cfg(not(windows))]
        {
            let _ = path;
            self.message
        }
    }

    pub(crate) fn is_wrong_bit_type(&self) -> bool {
        #[cfg(windows)]
        {
            self.code == Ok(windows_sys::Win32::Foundation::ERROR_BAD_EXE_FORMAT)
        }
        #[cfg(not(windows))]
        {
            self.message
                .as_ref()
                .is_ok_and(|message| message.contains("wrong ELF class"))
        }
    }
}

impl LoaderLibrary {
    /// Opens a module and snapshots the unloading policy before executing its
    /// initialization routines, matching upstream loader initialization.
    #[cfg(not(any(windows, target_os = "fuchsia")))]
    pub(crate) unsafe fn open(path: &Path) -> Result<Self, OpenLibraryError> {
        initialize_loader().map_err(OpenLibraryError::initialization)?;
        // SAFETY: The caller owns the foreign initialization contract.
        unsafe { Self::open_unix(path) }
    }

    #[cfg(unix)]
    pub(super) unsafe fn open_unix(path: &Path) -> Result<Self, OpenLibraryError> {
        if let Some(index) = path
            .as_os_str()
            .as_encoded_bytes()
            .iter()
            .position(|byte| *byte == 0)
        {
            use crate::debug::diagnostics;

            return Err(OpenLibraryError {
                message: diagnostics::try_format(format_args!(
                    "nul byte found in provided data at position: {index}"
                )),
            });
        }
        with_c_path(path, |path| {
            // SAFETY: The caller permits foreign initialization; path is live
            // and terminated. These flags match the previous libloading call.
            let handle = unsafe { libc::dlopen(path.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL) };
            if let Some(handle) = NonNull::new(handle) {
                return Ok(Self(handle));
            }
            // SAFETY: dlopen just failed; consume its thread-local error before
            // another dynamic-loader operation can replace it.
            let error = unsafe { libc::dlerror() };
            let message = if error.is_null() {
                crate::allocation::try_string("dlopen failed, but system did not report the error")
            } else {
                // SAFETY: A non-null dlerror result is a live terminated string.

                use crate::debug::diagnostics;
                let bytes = unsafe { CStr::from_ptr(error) }.to_bytes();
                diagnostics::try_format(format_args!(
                    "{}",
                    crate::debug::diagnostics::LossyBytes(bytes)
                ))
            };
            Err(OpenLibraryError { message })
        })
        .unwrap_or(Err(OpenLibraryError {
            message: Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY),
        }))
    }

    /// Looks up a terminated symbol without allocating a discarded error.
    ///
    /// # Safety
    /// T must be the symbol's pointer-sized ABI type. Calling it requires the
    /// corresponding foreign contract, and copies must not outlive the module.
    pub(crate) unsafe fn get<T>(&self, name: &[u8]) -> Result<LibrarySymbol<'_, T>, ()> {
        const {
            assert!(core::mem::size_of::<T>() == core::mem::size_of::<*mut c_void>());
        }
        const {
            assert!(core::mem::align_of::<T>() <= core::mem::align_of::<*mut c_void>());
        }
        let name = CStr::from_bytes_with_nul(name).map_err(|_| ())?;
        #[cfg(unix)]
        // SAFETY: The module is live and the symbol name is terminated.
        let pointer = unsafe {
            libc::dlerror();
            let pointer = libc::dlsym(self.0.as_ptr(), name.as_ptr());
            if pointer.is_null() {
                libc::dlerror();
            }
            pointer
        };
        #[cfg(windows)]
        // SAFETY: The module is live and the symbol name is terminated.
        let pointer = unsafe { GetProcAddress(self.0.as_ptr(), name.as_ptr().cast()) }
            .map_or(core::ptr::null_mut(), |function| function as *mut c_void);
        if pointer.is_null() {
            return Err(());
        }
        // SAFETY: Compile-time assertions establish the representation size
        // and alignment; the caller guarantees the symbol's ABI type.
        let value = unsafe { (&raw const pointer).cast::<T>().read() };
        Ok(LibrarySymbol {
            value,
            library: PhantomData,
        })
    }

    #[cfg(not(target_os = "fuchsia"))]
    pub(crate) unsafe fn open_driver(path: &Path) -> Result<Self, OpenLibraryError> {
        // SAFETY: Non-Fuchsia drivers use the ordinary platform module loader.
        unsafe { Self::open(path) }
    }

    #[cfg(target_os = "fuchsia")]
    pub(crate) unsafe fn open(path: &Path) -> Result<Self, OpenLibraryError> {
        initialize_loader().map_err(OpenLibraryError::initialization)?;
        // SAFETY: Layer initialization routines have the same dynamic-module
        // contract as on other Unix platforms.
        unsafe { fuchsia::open(path, false) }
    }

    #[cfg(target_os = "fuchsia")]
    pub(crate) unsafe fn open_driver(path: &Path) -> Result<Self, OpenLibraryError> {
        initialize_loader().map_err(OpenLibraryError::initialization)?;
        // SAFETY: ICD initialization routines have the same dynamic-module
        // contract as on other Unix platforms.
        unsafe { fuchsia::open(path, true) }
    }

    #[cfg(windows)]
    pub(crate) unsafe fn open(path: &Path) -> Result<Self, OpenLibraryError> {
        initialize_loader().map_err(OpenLibraryError::initialization)?;
        let path = crate::allocation::try_collect(
            path.as_os_str().encode_wide().chain(core::iter::once(0)),
        )
        .map_err(|_| OpenLibraryError {
            code: Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY),
        })?;
        // SAFETY: `path` is NUL-terminated and remains live for both calls.
        let mut handle = unsafe { LoadLibraryW(path.as_ptr()) };
        if handle.is_null() && unsafe { GetLastError() } == ERROR_MOD_NOT_FOUND {
            // SAFETY: This is upstream's dependency-directory fallback for a
            // module that the default LoadLibraryW search could not resolve.
            handle = unsafe {
                LoadLibraryExW(
                    path.as_ptr(),
                    core::ptr::null_mut(),
                    LOAD_LIBRARY_SEARCH_DEFAULT_DIRS | LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
                )
            };
        }
        if handle.is_null() {
            // SAFETY: No intervening Windows call has changed the failure code.
            return Err(OpenLibraryError {
                code: Ok(unsafe { GetLastError() }),
            });
        }
        let handle = NonNull::new(handle).ok_or(OpenLibraryError { code: Ok(0) })?;
        Ok(Self(handle))
    }
}

#[cfg(all(
    unix,
    not(any(target_os = "cygwin", target_os = "fuchsia", target_os = "nto"))
))]
pub(crate) fn loaded_library_path(address: *const c_void) -> Result<Option<PathBuf>, vk::VkResult> {
    let mut info = core::mem::MaybeUninit::<libc::Dl_info>::uninit();
    // SAFETY: `info` is writable and `address` points into a currently loaded module.
    if unsafe { libc::dladdr(address, info.as_mut_ptr()) } == 0 {
        return Ok(None);
    }
    // SAFETY: A successful `dladdr` call initialized the complete structure.
    let info = unsafe { info.assume_init() };
    if info.dli_fname.is_null() {
        return Ok(None);
    }
    // SAFETY: `dli_fname` is a NUL-terminated path owned by the dynamic loader.
    let path = unsafe { CStr::from_ptr(info.dli_fname) }.to_bytes();
    crate::allocation::try_path(Path::new(OsStr::from_bytes(path))).map(Some)
}

#[cfg(target_os = "cygwin")]
pub(crate) fn loaded_library_path(address: *const c_void) -> Result<Option<PathBuf>, vk::VkResult> {
    let mut info = core::mem::MaybeUninit::<libc::Dl_info>::uninit();
    // SAFETY: `info` is writable and `address` points into a currently loaded module.
    if unsafe { libc::dladdr(address, info.as_mut_ptr()) } == 0 {
        return Ok(None);
    }
    // SAFETY: A successful `dladdr` call initialized the complete structure.
    let info = unsafe { info.assume_init() };
    if info.dli_fname[0] == 0 {
        return Ok(None);
    }
    // SAFETY: Cygwin's inline `dli_fname` array is NUL-terminated after a
    // successful `dladdr` call and remains live for this conversion.
    let path = unsafe { CStr::from_ptr(info.dli_fname.as_ptr()) }.to_bytes();
    crate::allocation::try_path(Path::new(OsStr::from_bytes(path))).map(Some)
}

#[cfg(any(not(unix), target_os = "fuchsia", target_os = "nto"))]
pub(crate) const fn loaded_library_path(
    _address: *const c_void,
) -> Result<Option<PathBuf>, vk::VkResult> {
    Ok(None)
}

impl Drop for LoaderLibrary {
    fn drop(&mut self) {
        if !dynamic_library_unloading_disabled() {
            #[cfg(unix)]
            // SAFETY: This object owns one successful dlopen reference.
            unsafe {
                libc::dlclose(self.0.as_ptr())
            };
            #[cfg(windows)]
            // SAFETY: This object owns one successful LoadLibrary reference.
            unsafe {
                windows_sys::Win32::Foundation::FreeLibrary(self.0.as_ptr())
            };
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn loader_initialization_errors_preserve_their_result_without_formatting() {
        for result in [
            vk::VkResult::ERROR_OUT_OF_HOST_MEMORY,
            vk::VkResult::ERROR_INITIALIZATION_FAILED,
        ] {
            crate::allocation::fault::sweep_operation(|| {
                let error = super::OpenLibraryError::initialization(result);
                assert_eq!(
                    error.is_out_of_memory(),
                    result == vk::VkResult::ERROR_OUT_OF_HOST_MEMORY
                );
                assert_eq!(
                    error.into_message(std::path::Path::new("module")),
                    Err(result)
                );
                vk::VkResult::SUCCESS
            });
        }
    }

    #[cfg(windows)]
    #[test]
    fn foreign_dll_initialization_error_is_still_a_library_diagnostic() {
        let error = super::OpenLibraryError {
            code: Ok(windows_sys::Win32::Foundation::ERROR_DLL_INIT_FAILED),
        };
        assert!(!error.is_out_of_memory());
        assert!(
            error
                .into_message(std::path::Path::new("module"))
                .unwrap()
                .contains("1114")
        );
    }

    #[cfg(unix)]
    #[test]
    fn library_error_allocation_failure_remains_distinguishable() {
        crate::allocation::fault::sweep_operation(|| {
            // SAFETY: The embedded NUL is rejected before invoking dlopen,
            // so no foreign module initialization can occur.
            match unsafe {
                super::LoaderLibrary::open_unix(std::path::Path::new("invalid\0module"))
            } {
                Err(error) if error.is_out_of_memory() => vk::VkResult::ERROR_OUT_OF_HOST_MEMORY,
                Err(_) => vk::VkResult::SUCCESS,
                Ok(_) => panic!("a module path containing NUL must be rejected"),
            }
        });
    }

    #[cfg(windows)]
    #[test]
    fn library_memory_errors_are_classified_without_formatting() {
        for code in [
            windows_sys::Win32::Foundation::ERROR_NOT_ENOUGH_MEMORY,
            windows_sys::Win32::Foundation::ERROR_OUTOFMEMORY,
        ] {
            let error = super::OpenLibraryError { code: Ok(code) };
            assert!(error.is_out_of_memory());
            assert_eq!(
                error.into_message(std::path::Path::new("module")),
                Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)
            );
        }
        assert!(
            !super::OpenLibraryError {
                code: Ok(windows_sys::Win32::Foundation::ERROR_MOD_NOT_FOUND),
            }
            .is_out_of_memory()
        );
    }
}
