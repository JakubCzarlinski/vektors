//! Operating-system services used by loader discovery.

use core::ffi::c_void;
use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::{ffi::CString, sync::OnceLock};

fn dynamic_library_unloading_disabled() -> bool {
    static DISABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *DISABLED.get_or_init(|| {
        std::env::var_os("VK_LOADER_DISABLE_DYNAMIC_LIBRARY_UNLOADING")
            .is_some_and(|value| value == "1")
    })
}

pub(crate) fn initialize_loader() {
    std::sync::LazyLock::force(&LOADER_LOCK);
    static LOG_INITIALIZATION: std::sync::Once = std::sync::Once::new();
    LOG_INITIALIZATION.call_once(|| {
        let version = vk::VK_HEADER_VERSION_COMPLETE;
        write_loader_log(
            "info",
            "INFO",
            format_args!(
                "Vulkan Loader Version {}.{}.{}",
                vk::VK_API_VERSION_MAJOR(version),
                vk::VK_API_VERSION_MINOR(version),
                vk::VK_API_VERSION_PATCH(version)
            ),
        );
        write_loader_log(
            "info",
            "INFO",
            format_args!(
                "[Vulkan Loader Git - Tag: {}, Branch/Commit: {}]",
                env!("VK_LOADER_GIT_BRANCH_NAME"),
                env!("VK_LOADER_GIT_TAG_INFO")
            ),
        );
        if dynamic_library_unloading_disabled() {
            write_loader_log(
                "warn",
                "WARNING",
                format_args!("Vulkan Loader: library unloading is disabled"),
            );
        }
    });
}

#[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
fn release_loader() {
    crate::icd::unload_preloaded_icds();
    // SAFETY: Library termination runs only after loader entry points have
    // stopped executing, and the lock was initialized during library startup.
    unsafe { LOADER_LOCK.destroy() };
}

#[cfg(any(
    all(unix, not(target_vendor = "apple")),
    all(target_vendor = "apple", not(feature = "apple-static-loader"))
))]
unsafe extern "C" fn initialize_loader_library() {
    initialize_loader();
}

#[cfg(any(
    all(unix, not(target_vendor = "apple")),
    all(target_vendor = "apple", not(feature = "apple-static-loader"))
))]
unsafe extern "C" fn release_loader_library() {
    release_loader();
}

#[cfg(all(unix, not(target_vendor = "apple")))]
#[used]
#[unsafe(link_section = ".init_array")]
static LOADER_LIBRARY_INITIALIZER: unsafe extern "C" fn() = initialize_loader_library;

#[cfg(all(unix, not(target_vendor = "apple")))]
#[used]
#[unsafe(link_section = ".fini_array")]
static LOADER_LIBRARY_TERMINATOR: unsafe extern "C" fn() = release_loader_library;

#[cfg(all(target_vendor = "apple", not(feature = "apple-static-loader")))]
#[used]
#[unsafe(link_section = "__DATA,__mod_init_func")]
static LOADER_LIBRARY_INITIALIZER: unsafe extern "C" fn() = initialize_loader_library;

#[cfg(all(target_vendor = "apple", not(feature = "apple-static-loader")))]
#[used]
#[unsafe(link_section = "__DATA,__mod_term_func")]
static LOADER_LIBRARY_TERMINATOR: unsafe extern "C" fn() = release_loader_library;

#[cfg(windows)]
unsafe extern "system" fn loader_tls_callback(
    _module: windows_sys::Win32::Foundation::HINSTANCE,
    reason: u32,
    reserved: *mut c_void,
) {
    match reason {
        windows_sys::Win32::System::SystemServices::DLL_PROCESS_ATTACH => {
            std::sync::LazyLock::force(&LOADER_LOCK);
        }
        windows_sys::Win32::System::SystemServices::DLL_PROCESS_DETACH if reserved.is_null() => {
            release_loader();
        }
        _ => {}
    }
}

// A PE TLS callback receives the same process attach/detach notifications and
// reserved argument as DllMain, but remains an internal implementation symbol.
// This preserves upstream's synchronization lifetime without adding DllMain to
// Vulkan-Loader's public export table.
#[cfg(windows)]
#[used]
#[unsafe(link_section = ".CRT$XLB")]
static LOADER_TLS_CALLBACK: unsafe extern "system" fn(
    windows_sys::Win32::Foundation::HINSTANCE,
    u32,
    *mut c_void,
) = loader_tls_callback;

/// A dynamic module with Vulkan-Loader's process-wide unloading policy.
pub(crate) struct LoaderLibrary(core::mem::ManuallyDrop<libloading::Library>);

pub(crate) struct OpenLibraryError {
    #[cfg(windows)]
    code: u32,
    #[cfg(not(windows))]
    message: String,
}

impl OpenLibraryError {
    pub(crate) fn message(&self, path: &Path) -> String {
        #[cfg(windows)]
        {
            format!(
                "Failed to open dynamic library \"{}\" with error {}",
                path.to_string_lossy(),
                self.code
            )
        }
        #[cfg(not(windows))]
        {
            let _ = path;
            self.message.clone()
        }
    }

    pub(crate) fn is_wrong_bit_type(&self) -> bool {
        #[cfg(windows)]
        {
            self.code == windows_sys::Win32::Foundation::ERROR_BAD_EXE_FORMAT
        }
        #[cfg(not(windows))]
        {
            self.message(Path::new("")).contains("wrong ELF class")
        }
    }
}

impl LoaderLibrary {
    /// Opens a module and snapshots the unloading policy before executing its
    /// initialization routines, matching upstream loader initialization.
    #[cfg(not(any(windows, target_os = "fuchsia")))]
    pub(crate) unsafe fn open(path: &Path) -> Result<Self, OpenLibraryError> {
        use std::error::Error as _;

        initialize_loader();
        // SAFETY: The caller owns the contract for executing foreign module
        // initialization and termination routines.
        unsafe { libloading::Library::new(path.as_os_str()) }
            .map(|library| Self(core::mem::ManuallyDrop::new(library)))
            .map_err(|error| OpenLibraryError {
                message: error
                    .source()
                    .map_or_else(|| error.to_string(), ToString::to_string),
            })
    }

    #[cfg(not(target_os = "fuchsia"))]
    pub(crate) unsafe fn open_driver(path: &Path) -> Result<Self, OpenLibraryError> {
        // SAFETY: Non-Fuchsia drivers use the ordinary platform module loader.
        unsafe { Self::open(path) }
    }

    #[cfg(target_os = "fuchsia")]
    pub(crate) unsafe fn open(path: &Path) -> Result<Self, OpenLibraryError> {
        initialize_loader();
        // SAFETY: Layer initialization routines have the same dynamic-module
        // contract as on other Unix platforms.
        unsafe { fuchsia::open(path, false) }
    }

    #[cfg(target_os = "fuchsia")]
    pub(crate) unsafe fn open_driver(path: &Path) -> Result<Self, OpenLibraryError> {
        initialize_loader();
        // SAFETY: ICD initialization routines have the same dynamic-module
        // contract as on other Unix platforms.
        unsafe { fuchsia::open(path, true) }
    }

    #[cfg(windows)]
    pub(crate) unsafe fn open(path: &Path) -> Result<Self, OpenLibraryError> {
        use std::os::windows::ffi::OsStrExt as _;
        use windows_sys::Win32::{
            Foundation::{ERROR_MOD_NOT_FOUND, GetLastError},
            System::LibraryLoader::{
                LOAD_LIBRARY_SEARCH_DEFAULT_DIRS, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR, LoadLibraryExW,
                LoadLibraryW,
            },
        };

        initialize_loader();
        let path = path
            .as_os_str()
            .encode_wide()
            .chain(core::iter::once(0))
            .collect::<Vec<_>>();
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
                code: unsafe { GetLastError() },
            });
        }
        // SAFETY: `handle` came from a successful LoadLibraryW/LoadLibraryExW
        // call and is transferred into the unique libloading owner.
        let library = unsafe { libloading::os::windows::Library::from_raw(handle as isize) };
        Ok(Self(core::mem::ManuallyDrop::new(library.into())))
    }
}

#[cfg(target_os = "fuchsia")]
mod fuchsia {
    use core::ffi::{CStr, c_char, c_int, c_void};
    use std::{ffi::CString, os::unix::ffi::OsStrExt as _, path::Path, sync::OnceLock};

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

    pub(super) unsafe fn open(
        path: &Path,
        driver: bool,
    ) -> Result<LoaderLibrary, OpenLibraryError> {
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
            fuchsia_vulkan_loader_LoaderGet(
                service,
                name.as_ptr(),
                name.as_bytes().len(),
                &raw mut vmo,
            )
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
}

impl core::ops::Deref for LoaderLibrary {
    type Target = libloading::Library;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for LoaderLibrary {
    fn drop(&mut self) {
        if !dynamic_library_unloading_disabled() {
            // SAFETY: This is the unique owning `ManuallyDrop`, and the value
            // is dropped at most once on this branch.
            unsafe { core::mem::ManuallyDrop::drop(&mut self.0) };
        }
    }
}

#[cfg(target_vendor = "apple")]
mod apple {
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
                CFURLGetFileSystemRepresentation(
                    url,
                    1,
                    bytes.as_mut_ptr().cast(),
                    capacity as isize,
                )
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
}

#[cfg(target_vendor = "apple")]
pub(crate) fn bundle_resource_directory() -> Option<PathBuf> {
    apple::resource_directory()
}

#[cfg(windows)]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AdapterLuid {
    pub(crate) low_part: u32,
    pub(crate) high_part: i32,
}

#[cfg(windows)]
pub(crate) struct LoaderLock {
    critical_section:
        core::cell::UnsafeCell<windows_sys::Win32::System::Threading::CRITICAL_SECTION>,
}

#[cfg(windows)]
unsafe impl Send for LoaderLock {}
#[cfg(windows)]
unsafe impl Sync for LoaderLock {}

#[cfg(windows)]
impl LoaderLock {
    fn new() -> Self {
        let mut critical_section = core::mem::MaybeUninit::uninit();
        // SAFETY: The storage is writable and becomes initialized by this
        // infallible Windows API before it is placed in the global lock.
        unsafe {
            windows_sys::Win32::System::Threading::InitializeCriticalSection(
                critical_section.as_mut_ptr(),
            )
        };
        Self {
            // SAFETY: InitializeCriticalSection initialized the structure.
            critical_section: core::cell::UnsafeCell::new(unsafe {
                critical_section.assume_init()
            }),
        }
    }

    fn lock(&self) -> LoaderLockGuard<'_> {
        // SAFETY: The critical section is initialized and remains live for the
        // process lifetime. Windows critical sections are recursive, matching
        // Vulkan-Loader's loader_lock contract.
        unsafe {
            windows_sys::Win32::System::Threading::EnterCriticalSection(self.critical_section.get())
        };
        LoaderLockGuard { lock: self }
    }

    unsafe fn destroy(&self) {
        // SAFETY: The caller guarantees that no loader entry point can still
        // acquire or hold the critical section during DLL termination.
        unsafe {
            windows_sys::Win32::System::Threading::DeleteCriticalSection(
                self.critical_section.get(),
            )
        };
    }
}

#[cfg(windows)]
pub(crate) struct LoaderLockGuard<'a> {
    lock: &'a LoaderLock,
}

#[cfg(windows)]
impl Drop for LoaderLockGuard<'_> {
    fn drop(&mut self) {
        // SAFETY: This guard represents exactly one successful recursive
        // acquisition by the current thread.
        unsafe {
            windows_sys::Win32::System::Threading::LeaveCriticalSection(
                self.lock.critical_section.get(),
            )
        };
    }
}

#[cfg(windows)]
pub(crate) fn lock_loader() -> LoaderLockGuard<'static> {
    LOADER_LOCK.lock()
}

#[cfg(not(windows))]
pub(crate) struct LoaderLock {
    mutex: core::cell::UnsafeCell<libc::pthread_mutex_t>,
}

#[cfg(not(windows))]
unsafe impl Send for LoaderLock {}
#[cfg(not(windows))]
unsafe impl Sync for LoaderLock {}

#[cfg(not(windows))]
impl LoaderLock {
    fn new() -> Self {
        let mut attributes = core::mem::MaybeUninit::<libc::pthread_mutexattr_t>::uninit();
        // SAFETY: The storage is writable and initialized on success.
        if unsafe { libc::pthread_mutexattr_init(attributes.as_mut_ptr()) } != 0 {
            std::process::abort();
        }
        // SAFETY: The successful call above initialized the attributes object.
        let mut attributes = unsafe { attributes.assume_init() };
        #[cfg(target_os = "hurd")]
        let recursive_mutex_type = libc::PTHREAD_MUTEX_RECURSIVE as libc::c_int;
        #[cfg(not(target_os = "hurd"))]
        let recursive_mutex_type = libc::PTHREAD_MUTEX_RECURSIVE;
        // SAFETY: The attributes object is initialized and exclusively owned.
        if unsafe { libc::pthread_mutexattr_settype(&mut attributes, recursive_mutex_type) } != 0 {
            // SAFETY: The attributes object was initialized successfully.
            unsafe { libc::pthread_mutexattr_destroy(&mut attributes) };
            std::process::abort();
        }

        let mut mutex = core::mem::MaybeUninit::<libc::pthread_mutex_t>::uninit();
        // SAFETY: Both arguments point to initialized/writable native objects.
        let result = unsafe { libc::pthread_mutex_init(mutex.as_mut_ptr(), &attributes) };
        // SAFETY: The attributes object is no longer needed after mutex init.
        unsafe { libc::pthread_mutexattr_destroy(&mut attributes) };
        if result != 0 {
            std::process::abort();
        }
        Self {
            // SAFETY: pthread_mutex_init succeeded and initialized the mutex.
            mutex: core::cell::UnsafeCell::new(unsafe { mutex.assume_init() }),
        }
    }

    fn lock(&self) -> LoaderLockGuard<'_> {
        // SAFETY: The native mutex is initialized and remains live for the
        // guard. POSIX recursive mutexes match upstream loader_lock semantics.
        if unsafe { libc::pthread_mutex_lock(self.mutex.get()) } != 0 {
            std::process::abort();
        }
        LoaderLockGuard { lock: self }
    }

    #[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
    unsafe fn destroy(&self) {
        // SAFETY: The caller guarantees that no loader entry point can still
        // acquire or hold the mutex during library termination.
        let result = unsafe { libc::pthread_mutex_destroy(self.mutex.get()) };
        debug_assert_eq!(result, 0);
    }
}

#[cfg(not(windows))]
pub(crate) struct LoaderLockGuard<'a> {
    lock: &'a LoaderLock,
}

#[cfg(not(windows))]
impl Drop for LoaderLockGuard<'_> {
    fn drop(&mut self) {
        // SAFETY: This guard represents exactly one successful recursive
        // acquisition by the current thread.
        if unsafe { libc::pthread_mutex_unlock(self.lock.mutex.get()) } != 0 {
            std::process::abort();
        }
    }
}

#[cfg(not(windows))]
pub(crate) fn lock_loader() -> LoaderLockGuard<'static> {
    LOADER_LOCK.lock()
}

static LOADER_LOCK: std::sync::LazyLock<LoaderLock> = std::sync::LazyLock::new(LoaderLock::new);

/// Returns the current executable path using Vulkan-Loader's 1024-byte platform
/// contract. The fixed bound and unavailable-platform cases are observable in
/// app-keyed loader settings and override-layer selection.
#[cfg(any(target_os = "fuchsia", target_os = "openbsd"))]
pub(crate) const fn executable_path() -> Option<PathBuf> {
    None
}

#[cfg(any(target_os = "linux", target_os = "android", target_os = "hurd"))]
pub(crate) fn executable_path() -> Option<PathBuf> {
    use std::os::unix::ffi::OsStringExt as _;

    let mut bytes = [0_u8; 1024];
    // SAFETY: `bytes` supplies exactly the writable extent passed to readlink.
    let length = unsafe {
        libc::readlink(
            c"/proc/self/exe".as_ptr(),
            bytes.as_mut_ptr().cast(),
            bytes.len(),
        )
    };
    let length = usize::try_from(length).ok()?;
    if length == 0 || length >= bytes.len() {
        return None;
    }
    Some(PathBuf::from(std::ffi::OsString::from_vec(
        bytes[..length].to_vec(),
    )))
}

#[cfg(target_os = "macos")]
pub(crate) fn executable_path() -> Option<PathBuf> {
    use core::ffi::c_int;
    use std::os::unix::ffi::OsStringExt as _;

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
    let length = usize::try_from(length).ok()?;
    if length == 0 || length >= bytes.len() {
        return None;
    }
    let length = bytes[..=length]
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(length);
    Some(PathBuf::from(std::ffi::OsString::from_vec(
        bytes[..length].to_vec(),
    )))
}

// Vulkan-Loader deliberately reports a present but empty executable path on
// Apple's mobile targets.
#[cfg(all(target_vendor = "apple", not(target_os = "macos")))]
pub(crate) fn executable_path() -> Option<PathBuf> {
    Some(PathBuf::new())
}

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd"))]
pub(crate) fn executable_path() -> Option<PathBuf> {
    use std::os::unix::ffi::OsStringExt as _;

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
    Some(PathBuf::from(std::ffi::OsString::from_vec(
        bytes[..length].to_vec(),
    )))
}

#[cfg(any(target_os = "nto", target_os = "qnx"))]
pub(crate) fn executable_path() -> Option<PathBuf> {
    use std::{io::Read as _, os::unix::ffi::OsStringExt as _};

    let mut file = std::fs::File::open("/proc/self/exefile").ok()?;
    let mut bytes = Vec::with_capacity(1024);
    file.by_ref().take(1024).read_to_end(&mut bytes).ok()?;
    if bytes.len() >= 1024 {
        return None;
    }
    Some(PathBuf::from(std::ffi::OsString::from_vec(bytes)))
}

#[cfg(windows)]
pub(crate) fn executable_path() -> Option<PathBuf> {
    use std::os::windows::ffi::OsStringExt as _;
    use windows_sys::Win32::{
        Foundation::{ERROR_INSUFFICIENT_BUFFER, GetLastError},
        System::LibraryLoader::GetModuleFileNameW,
    };

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
    let length = usize::try_from(length).ok()?;
    if length >= units.len() {
        return None;
    }
    Some(PathBuf::from(std::ffi::OsString::from_wide(
        &units[..length],
    )))
}

#[cfg(not(any(
    target_os = "android",
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

#[cfg(windows)]
#[inline]
pub(crate) fn current_thread_key() -> usize {
    // SAFETY: `GetCurrentThreadId` has no preconditions.
    unsafe { windows_sys::Win32::System::Threading::GetCurrentThreadId() as usize }
}

#[cfg(unix)]
#[inline]
pub(crate) fn current_thread_key() -> usize {
    // POSIX guarantees `pthread_self` uniquely identifies the calling live
    // thread; the supported Unix ABIs represent `pthread_t` in one word.
    unsafe { libc::pthread_self() as usize }
}

#[cfg(not(any(unix, windows)))]
#[inline]
pub(crate) fn current_thread_key() -> usize {
    use core::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    hasher.finish() as usize
}

union FunctionPointer<T: Copy, U: Copy> {
    source: T,
    target: U,
}

unsafe fn reinterpret_function_pointer<T: Copy, U: Copy>(source: T) -> U {
    unsafe { FunctionPointer { source }.target }
}

#[cfg(unix)]
fn interposed_symbol<T: Copy>(name: &core::ffi::CStr, fallback: T) -> T {
    // SAFETY: `RTLD_DEFAULT` searches the executable before its dependencies,
    // matching the interposition behavior used by Vulkan-Loader's test shim.
    let address = unsafe { libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr()) };
    if address.is_null() {
        fallback
    } else {
        // SAFETY: The requested symbol name and `T` describe the same libc ABI.
        unsafe { reinterpret_function_pointer(address) }
    }
}

#[cfg(unix)]
fn fopen() -> unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> *mut libc::FILE {
    type Function =
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> *mut libc::FILE;
    static FUNCTION: OnceLock<Function> = OnceLock::new();
    *FUNCTION.get_or_init(|| interposed_symbol(c"fopen", libc::fopen as Function))
}

#[cfg(unix)]
fn access() -> unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int {
    type Function = unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int;
    static FUNCTION: OnceLock<Function> = OnceLock::new();
    *FUNCTION.get_or_init(|| interposed_symbol(c"access", libc::access as Function))
}

#[cfg(unix)]
fn fputs() -> unsafe extern "C" fn(*const libc::c_char, *mut libc::FILE) -> libc::c_int {
    type Function = unsafe extern "C" fn(*const libc::c_char, *mut libc::FILE) -> libc::c_int;
    static FUNCTION: OnceLock<Function> = OnceLock::new();
    *FUNCTION.get_or_init(|| interposed_symbol(c"fputs", libc::fputs as Function))
}

#[cfg(unix)]
fn stderr_stream() -> *mut libc::FILE {
    static ADDRESS: OnceLock<usize> = OnceLock::new();
    let address = *ADDRESS
        .get_or_init(|| unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"stderr".as_ptr()) } as usize);
    if address == 0 {
        return core::ptr::null_mut();
    }
    // SAFETY: `stderr` is an exported C-runtime `FILE *` object.
    unsafe { (address as *const *mut libc::FILE).read() }
}

#[cfg(unix)]
fn opendir() -> unsafe extern "C" fn(*const libc::c_char) -> *mut libc::DIR {
    type Function = unsafe extern "C" fn(*const libc::c_char) -> *mut libc::DIR;
    static FUNCTION: OnceLock<Function> = OnceLock::new();
    *FUNCTION.get_or_init(|| interposed_symbol(c"opendir", libc::opendir as Function))
}

#[cfg(unix)]
fn readdir() -> unsafe extern "C" fn(*mut libc::DIR) -> *mut libc::dirent {
    type Function = unsafe extern "C" fn(*mut libc::DIR) -> *mut libc::dirent;
    static FUNCTION: OnceLock<Function> = OnceLock::new();
    *FUNCTION.get_or_init(|| interposed_symbol(c"readdir", libc::readdir as Function))
}

#[cfg(unix)]
fn closedir() -> unsafe extern "C" fn(*mut libc::DIR) -> libc::c_int {
    type Function = unsafe extern "C" fn(*mut libc::DIR) -> libc::c_int;
    static FUNCTION: OnceLock<Function> = OnceLock::new();
    *FUNCTION.get_or_init(|| interposed_symbol(c"closedir", libc::closedir as Function))
}

#[cfg(unix)]
pub(crate) fn has_elevated_privileges() -> bool {
    // SAFETY: These process identity queries have no pointer contracts. The
    // upstream parity harness interposes them to exercise secure discovery.
    unsafe { libc::geteuid() != libc::getuid() || libc::getegid() != libc::getgid() }
}

#[cfg(windows)]
pub(crate) fn has_elevated_privileges() -> bool {
    use windows_sys::Win32::{
        Foundation::CloseHandle,
        Security::{
            GetSidSubAuthority, GetSidSubAuthorityCount, GetTokenInformation,
            TOKEN_MANDATORY_LABEL, TokenIntegrityLevel,
        },
        System::Threading::{GetCurrentProcess, OpenProcessToken},
    };

    const TOKEN_QUERY: u32 = 0x0008;
    const TOKEN_QUERY_SOURCE: u32 = 0x0010;
    const SECURITY_MANDATORY_HIGH_RID: u32 = 0x0000_3000;
    // SECURITY_MAX_SID_SIZE plus the attributes DWORD, with sufficient
    // alignment for TOKEN_MANDATORY_LABEL.
    #[repr(C, align(8))]
    struct MandatoryLabelBuffer([u8; 72]);

    let mut token = core::ptr::null_mut();
    // SAFETY: The pseudo-handle is process-owned and `token` is writable.
    if unsafe {
        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_QUERY | TOKEN_QUERY_SOURCE,
            &mut token,
        )
    } == 0
    {
        return false;
    }

    let mut buffer = MandatoryLabelBuffer([0; 72]);
    let mut required = 0;
    // SAFETY: `buffer` is aligned, writable, and its exact size is supplied.
    let queried = unsafe {
        GetTokenInformation(
            token,
            TokenIntegrityLevel,
            buffer.0.as_mut_ptr().cast(),
            u32::try_from(buffer.0.len()).expect("fixed token buffer fits DWORD"),
            &mut required,
        )
    } != 0;
    let elevated = if queried {
        // SAFETY: A successful TokenIntegrityLevel query initialized this
        // prefix as TOKEN_MANDATORY_LABEL.
        let label = unsafe { &*buffer.0.as_ptr().cast::<TOKEN_MANDATORY_LABEL>() };
        // SAFETY: Windows supplied the SID in the successful token query.
        let count = unsafe { GetSidSubAuthorityCount(label.Label.Sid) };
        if count.is_null() || unsafe { *count } == 0 {
            false
        } else {
            // SAFETY: The SID reports at least one sub-authority, and the last
            // index is consequently in bounds.
            let level = unsafe { GetSidSubAuthority(label.Label.Sid, u32::from(*count) - 1) };
            !level.is_null() && unsafe { *level } >= SECURITY_MANDATORY_HIGH_RID
        }
    } else {
        false
    };
    // SAFETY: `token` was returned by OpenProcessToken and is owned here.
    unsafe { CloseHandle(token) };
    elevated
}

#[cfg(not(any(unix, windows)))]
pub(crate) const fn has_elevated_privileges() -> bool {
    false
}

#[cfg(windows)]
#[cold]
#[inline(never)]
fn registry_values(location: &core::ffi::CStr, current_user: bool) -> Box<[PathBuf]> {
    use windows_sys::Win32::{
        Foundation::ERROR_SUCCESS,
        System::Registry::{
            HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_QUERY_VALUE, RegCloseKey,
            RegEnumValueA, RegOpenKeyExA,
        },
    };

    let hive = if current_user {
        HKEY_CURRENT_USER
    } else {
        HKEY_LOCAL_MACHINE
    };
    let mut key: HKEY = core::ptr::null_mut();
    // SAFETY: The registry path is NUL-terminated and `key` is writable.
    if unsafe { RegOpenKeyExA(hive, location.as_ptr().cast(), 0, KEY_QUERY_VALUE, &mut key) }
        != ERROR_SUCCESS
    {
        return Box::default();
    }

    let mut paths = Vec::new();
    let mut name = Box::new([0_u8; 2048]);
    for index in 0.. {
        let mut name_length = u32::try_from(name.len()).expect("registry name buffer fits DWORD");
        let mut value = 0_u32;
        let mut value_length =
            u32::try_from(core::mem::size_of_val(&value)).expect("registry DWORD size fits DWORD");
        // SAFETY: All buffers are writable for the lengths supplied. This is
        // the same RegEnumValueA contract used by upstream and its test shim.
        let status = unsafe {
            RegEnumValueA(
                key,
                index,
                name.as_mut_ptr(),
                &mut name_length,
                core::ptr::null(),
                core::ptr::null_mut(),
                (&mut value as *mut u32).cast(),
                &mut value_length,
            )
        };
        if status != ERROR_SUCCESS {
            break;
        }
        if value_length == u32::try_from(core::mem::size_of_val(&value)).unwrap_or(0) && value == 0
        {
            let length = usize::try_from(name_length).unwrap_or(0).min(name.len());
            let length = name[..length]
                .iter()
                .position(|byte| *byte == 0)
                .unwrap_or(length);
            if let Ok(path) = core::str::from_utf8(&name[..length]) {
                paths.push(PathBuf::from(path));
            }
        }
    }
    // SAFETY: `key` was opened successfully and is owned by this function.
    unsafe { RegCloseKey(key) };
    paths.into_boxed_slice()
}

#[cfg(windows)]
pub(crate) struct RegistryDiagnostics {
    pub(crate) located: Box<[PathBuf]>,
    pub(crate) no_unique_files: bool,
}

#[cfg(windows)]
pub(crate) fn registry_manifest_files_with_diagnostics(
    leaf: &str,
) -> (Box<[PathBuf]>, RegistryDiagnostics) {
    let location = match leaf {
        "icd.d" => c"SOFTWARE\\Khronos\\Vulkan\\Drivers",
        "implicit_layer.d" => c"SOFTWARE\\Khronos\\Vulkan\\ImplicitLayers",
        "explicit_layer.d" => c"SOFTWARE\\Khronos\\Vulkan\\ExplicitLayers",
        _ => {
            return (
                Box::default(),
                RegistryDiagnostics {
                    located: Box::default(),
                    no_unique_files: true,
                },
            );
        }
    };
    let mut paths = Vec::new();
    if leaf != "explicit_layer.d"
        && let Some(package) = app_package_manifest_path()
    {
        paths.push(package);
    }
    paths.extend(d3dkmt_manifest_files(leaf));
    let mut located = Vec::new();
    let count_before = paths.len();
    let mut add_registry_values = |values: Box<[PathBuf]>| {
        for path in values {
            located.push(path.clone());
            if !paths.contains(&path) {
                paths.push(path);
            }
        }
    };
    add_registry_values(registry_values(location, false));
    if leaf != "icd.d" && !has_elevated_privileges() {
        add_registry_values(registry_values(location, true));
    }
    let no_unique_files = paths.len() == count_before;
    (
        paths.into_boxed_slice(),
        RegistryDiagnostics {
            located: located.into_boxed_slice(),
            no_unique_files,
        },
    )
}

#[cfg(windows)]
pub(crate) fn registry_manifest_files(leaf: &str) -> Box<[PathBuf]> {
    registry_manifest_files_with_diagnostics(leaf).0
}

#[cfg(windows)]
#[cold]
#[inline(never)]
fn d3dkmt_manifest_files(leaf: &str) -> Box<[PathBuf]> {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};
    use windows_sys::Win32::System::LibraryLoader::{
        GetModuleHandleA, GetProcAddress, LoadLibraryExA,
    };

    const STATUS_SUCCESS: i32 = 0;
    const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;
    const QUERY_TYPE_REGISTRY: u32 = 48;
    const QUERY_REGISTRY_ADAPTER_KEY: u32 = 1;
    const QUERY_REGISTRY_STATUS_SUCCESS: u32 = 0;
    const QUERY_REGISTRY_STATUS_BUFFER_OVERFLOW: u32 = 1;
    const REG_SZ: u32 = 1;
    const REG_MULTI_SZ: u32 = 7;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Luid {
        low_part: u32,
        high_part: i32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Adapter {
        handle: u32,
        luid: Luid,
        source_count: u32,
        present_move_regions_preferred: i32,
    }

    #[repr(C)]
    struct EnumAdapters {
        adapter_count: u32,
        adapters: *mut Adapter,
    }

    #[repr(C)]
    struct QueryAdapterInfo {
        handle: u32,
        kind: u32,
        private_data: *mut core::ffi::c_void,
        private_data_size: u32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct QueryRegistryInfo {
        query_type: u32,
        query_flags: u32,
        value_name: [u16; 260],
        value_type: u32,
        physical_adapter_index: u32,
        output_value_size: u32,
        status: u32,
        output: u64,
    }

    type EnumAdaptersFn = unsafe extern "system" fn(*mut EnumAdapters) -> i32;
    type QueryAdapterInfoFn = unsafe extern "system" fn(*mut QueryAdapterInfo) -> i32;

    fn symbol<T: Copy>(
        module: windows_sys::Win32::Foundation::HMODULE,
        name: &core::ffi::CStr,
    ) -> Option<T> {
        // SAFETY: `module` is live and `name` is NUL-terminated.
        let address = unsafe { GetProcAddress(module, name.as_ptr().cast()) }?;
        // SAFETY: Each caller chooses the function type matching `name`.
        Some(unsafe { reinterpret_function_pointer(address) })
    }

    // Match upstream's system32-only load, avoiding DLL search-path injection.
    // SAFETY: Both module names are NUL-terminated.
    let mut gdi32 = unsafe { GetModuleHandleA(c"gdi32.dll".as_ptr().cast()) };
    if gdi32.is_null() {
        // SAFETY: The system32 search flag prevents application-directory
        // substitution, and the returned module remains process-loaded.
        gdi32 = unsafe {
            LoadLibraryExA(
                c"gdi32.dll".as_ptr().cast(),
                core::ptr::null_mut(),
                LOAD_LIBRARY_SEARCH_SYSTEM32,
            )
        };
    }
    if gdi32.is_null() {
        return Box::default();
    }
    let Some(enum_adapters) = symbol::<EnumAdaptersFn>(gdi32, c"D3DKMTEnumAdapters2") else {
        return Box::default();
    };
    let Some(query_adapter) = symbol::<QueryAdapterInfoFn>(gdi32, c"D3DKMTQueryAdapterInfo") else {
        return Box::default();
    };

    let value_name = match leaf {
        "icd.d" => "VulkanDriverName",
        "implicit_layer.d" => "VulkanImplicitLayers",
        "explicit_layer.d" => "VulkanExplicitLayers",
        _ => return Box::default(),
    };
    let mut enumeration = EnumAdapters {
        adapter_count: 0,
        adapters: core::ptr::null_mut(),
    };
    // SAFETY: `enumeration` is writable for the documented sizing query.
    if unsafe { enum_adapters(&mut enumeration) } != STATUS_SUCCESS
        || enumeration.adapter_count == 0
    {
        return Box::default();
    }
    let mut adapters =
        Box::<[Adapter]>::new_uninit_slice(usize::try_from(enumeration.adapter_count).unwrap_or(0));
    enumeration.adapters = adapters.as_mut_ptr().cast();
    // SAFETY: The adapter array has the count requested by the first query.
    if unsafe { enum_adapters(&mut enumeration) } != STATUS_SUCCESS {
        return Box::default();
    }
    let initialized = usize::try_from(enumeration.adapter_count)
        .unwrap_or(0)
        .min(adapters.len());
    // SAFETY: A successful query initialized the reported adapter prefix. The
    // MaybeUninit backing allocation remains live throughout this borrow.
    let initialized_adapters =
        unsafe { core::slice::from_raw_parts(adapters.as_ptr().cast::<Adapter>(), initialized) };

    let mut paths = Vec::new();
    for adapter in initialized_adapters {
        let mut registry = Box::new(QueryRegistryInfo {
            query_type: QUERY_REGISTRY_ADAPTER_KEY,
            query_flags: 1,
            value_name: [0; 260],
            value_type: REG_MULTI_SZ,
            physical_adapter_index: 0,
            output_value_size: 0,
            status: QUERY_REGISTRY_STATUS_SUCCESS,
            output: 0,
        });
        for (destination, source) in registry
            .value_name
            .iter_mut()
            .zip(value_name.encode_utf16())
        {
            *destination = source;
        }
        let mut query = QueryAdapterInfo {
            handle: adapter.handle,
            kind: QUERY_TYPE_REGISTRY,
            private_data: (&mut *registry as *mut QueryRegistryInfo).cast(),
            private_data_size: u32::try_from(core::mem::size_of::<QueryRegistryInfo>())
                .expect("registry query header fits UINT"),
        };
        // SAFETY: The query header has the exact D3DKMT registry ABI.
        let mut status = unsafe { query_adapter(&mut query) };
        if status != STATUS_SUCCESS {
            registry.value_type = REG_SZ;
            // SAFETY: Same valid header, retrying the alternate registry type.
            status = unsafe { query_adapter(&mut query) };
        }
        if status != STATUS_SUCCESS || registry.status != QUERY_REGISTRY_STATUS_BUFFER_OVERFLOW {
            continue;
        }

        let mut response = None;
        for _ in 0..4 {
            let byte_length = core::mem::size_of::<QueryRegistryInfo>()
                .checked_add(usize::try_from(registry.output_value_size).unwrap_or(usize::MAX));
            let Some(byte_length) = byte_length else {
                break;
            };
            let word_length = byte_length.div_ceil(core::mem::size_of::<u64>());
            let mut storage = vec![0_u64; word_length].into_boxed_slice();
            // SAFETY: `storage` is aligned for QueryRegistryInfo and large
            // enough for the header and requested variable-sized output.
            unsafe {
                storage
                    .as_mut_ptr()
                    .cast::<QueryRegistryInfo>()
                    .write(*registry)
            };
            query.private_data = storage.as_mut_ptr().cast();
            query.private_data_size = match u32::try_from(byte_length) {
                Ok(length) => length,
                Err(_) => break,
            };
            // SAFETY: `query` points at the aligned, writable response buffer.
            if unsafe { query_adapter(&mut query) } != STATUS_SUCCESS {
                break;
            }
            // SAFETY: D3DKMT initialized the response header on success.
            let header = unsafe { &*storage.as_ptr().cast::<QueryRegistryInfo>() };
            if header.status == QUERY_REGISTRY_STATUS_SUCCESS {
                response = Some(storage);
                break;
            }
            if header.status != QUERY_REGISTRY_STATUS_BUFFER_OVERFLOW {
                break;
            }
            *registry = *header;
        }

        let Some(response) = response else { continue };
        // SAFETY: The successful response contains an initialized header.
        let header = unsafe { &*response.as_ptr().cast::<QueryRegistryInfo>() };
        let output_units =
            usize::try_from(header.output_value_size).unwrap_or(0) / core::mem::size_of::<u16>();
        // SAFETY: The allocation included `output_value_size` bytes following
        // the output union, which is aligned for UTF-16.
        let output = unsafe {
            core::slice::from_raw_parts(
                core::ptr::addr_of!(header.output).cast::<u16>(),
                output_units,
            )
        };
        let mut remaining = output;
        while let Some(length) = remaining.iter().position(|unit| *unit == 0) {
            if length == 0 {
                break;
            }
            paths.push(PathBuf::from(OsString::from_wide(&remaining[..length])));
            if header.value_type == REG_SZ {
                break;
            }
            remaining = &remaining[length + 1..];
        }
    }
    paths.into_boxed_slice()
}

#[cfg(windows)]
#[cold]
#[inline(never)]
pub(crate) fn adapter_luids() -> Box<[AdapterLuid]> {
    use windows_sys::{
        Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryExA},
        core::GUID,
    };

    const S_OK: i32 = 0;
    const DXGI_ERROR_NOT_FOUND: i32 = 0x887a_0002_u32 as i32;
    const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;
    const DXGI_GPU_PREFERENCE_UNSPECIFIED: u32 = 0;
    const IID_IDXGI_FACTORY6: GUID = GUID {
        data1: 0xc1b6_694f,
        data2: 0xff09,
        data3: 0x44a9,
        data4: [0xb0, 0x3c, 0x77, 0x90, 0x0a, 0x0a, 0x1d, 0x17],
    };
    const IID_IDXGI_ADAPTER1: GUID = GUID {
        data1: 0x2903_8f61,
        data2: 0x3839,
        data3: 0x4626,
        data4: [0x91, 0xfd, 0x08, 0x68, 0x79, 0x01, 0x1a, 0x05],
    };

    #[repr(C)]
    struct AdapterDescription {
        description: [u16; 128],
        vendor_id: u32,
        device_id: u32,
        subsystem_id: u32,
        revision: u32,
        dedicated_video_memory: usize,
        dedicated_system_memory: usize,
        shared_system_memory: usize,
        adapter_luid: AdapterLuid,
        flags: u32,
    }

    type CreateFactory = unsafe extern "system" fn(*const GUID, *mut *mut core::ffi::c_void) -> i32;
    type EnumAdapter = unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *const GUID,
        *mut *mut core::ffi::c_void,
    ) -> i32;
    type GetDescription =
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdapterDescription) -> i32;
    type Release = unsafe extern "system" fn(*mut core::ffi::c_void) -> u32;

    unsafe fn method<T: Copy>(object: *mut core::ffi::c_void, index: usize) -> T {
        // SAFETY: All DXGI COM interfaces begin with a live vtable pointer.
        let vtable = unsafe { object.cast::<*const *const core::ffi::c_void>().read() };
        // SAFETY: The caller supplies an index and type from the exact interface ABI.
        let address = unsafe { vtable.add(index).read() };
        // SAFETY: The selected vtable slot has function-pointer representation.
        unsafe { reinterpret_function_pointer(address) }
    }

    // SAFETY: Module names are NUL-terminated. Loading only from system32
    // matches upstream's DLL search-path hardening.
    let mut dxgi = unsafe { GetModuleHandleA(c"dxgi.dll".as_ptr().cast()) };
    if dxgi.is_null() {
        // SAFETY: The system32-only flag excludes application-controlled paths.
        dxgi = unsafe {
            LoadLibraryExA(
                c"dxgi.dll".as_ptr().cast(),
                core::ptr::null_mut(),
                LOAD_LIBRARY_SEARCH_SYSTEM32,
            )
        };
    }
    if dxgi.is_null() {
        return Box::default();
    }
    // SAFETY: `dxgi` is live and the symbol name is NUL-terminated.
    let Some(create_factory) =
        (unsafe { GetProcAddress(dxgi, c"CreateDXGIFactory1".as_ptr().cast()) })
    else {
        return Box::default();
    };
    debug_assert_eq!(
        core::mem::size_of::<CreateFactory>(),
        core::mem::size_of_val(&create_factory)
    );
    // SAFETY: GetProcAddress returned CreateDXGIFactory1's documented ABI.
    let create_factory: CreateFactory = unsafe { core::mem::transmute(create_factory) };

    let mut factory = core::ptr::null_mut();
    // SAFETY: The IID is valid and the output interface pointer is writable.
    if unsafe { create_factory(&IID_IDXGI_FACTORY6, &mut factory) } != S_OK || factory.is_null() {
        return Box::default();
    }
    // IDXGIFactory6 inherits 29 methods before EnumAdapterByGpuPreference;
    // IUnknown::Release remains slot 2.
    let enumerate: EnumAdapter = unsafe { method(factory, 29) };
    let release_factory: Release = unsafe { method(factory, 2) };
    let mut luids = Vec::new();
    for index in 0.. {
        let mut adapter = core::ptr::null_mut();
        // SAFETY: The factory and IID are live and `adapter` is writable.
        let result = unsafe {
            enumerate(
                factory,
                index,
                DXGI_GPU_PREFERENCE_UNSPECIFIED,
                &IID_IDXGI_ADAPTER1,
                &mut adapter,
            )
        };
        if result == DXGI_ERROR_NOT_FOUND {
            break;
        }
        if result != S_OK || adapter.is_null() {
            break;
        }
        // IDXGIAdapter1::GetDesc1 is slot 10; Release is slot 2.
        let get_description: GetDescription = unsafe { method(adapter, 10) };
        let release_adapter: Release = unsafe { method(adapter, 2) };
        let mut description = core::mem::MaybeUninit::<AdapterDescription>::uninit();
        // SAFETY: `description` is writable for the complete DXGI structure.
        if unsafe { get_description(adapter, description.as_mut_ptr()) } == S_OK {
            // SAFETY: GetDesc1 initialized the complete structure on success.
            luids.push(unsafe { description.assume_init() }.adapter_luid);
        }
        // SAFETY: This function owns the reference returned by enumeration.
        unsafe { release_adapter(adapter) };
    }
    // SAFETY: This function owns the factory reference.
    unsafe { release_factory(factory) };
    luids.into_boxed_slice()
}

#[cfg(windows)]
#[cold]
#[inline(never)]
fn app_package_manifest_path() -> Option<PathBuf> {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};
    use windows_sys::Win32::{
        Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS},
        System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
    };

    type GetPackagesByPackageFamily =
        unsafe extern "system" fn(*const u16, *mut u32, *mut *mut u16, *mut u32, *mut u16) -> u32;
    type GetPackagePathByFullName =
        unsafe extern "system" fn(*const u16, *mut u32, *mut u16) -> u32;

    // These APIs were introduced after Windows 7, so resolve them lazily just
    // as upstream does instead of adding hard loader imports.
    // SAFETY: The module name is NUL-terminated.
    let kernel = unsafe { GetModuleHandleA(c"kernel32.dll".as_ptr().cast()) };
    if kernel.is_null() {
        return None;
    }
    // SAFETY: The symbol names are NUL-terminated and `kernel` is live.
    let get_packages =
        unsafe { GetProcAddress(kernel, c"GetPackagesByPackageFamily".as_ptr().cast()) }?;
    // SAFETY: Same module and symbol-name contract as above.
    let get_path = unsafe { GetProcAddress(kernel, c"GetPackagePathByFullName".as_ptr().cast()) }?;
    debug_assert_eq!(
        core::mem::size_of::<GetPackagesByPackageFamily>(),
        core::mem::size_of_val(&get_packages)
    );
    debug_assert_eq!(
        core::mem::size_of::<GetPackagePathByFullName>(),
        core::mem::size_of_val(&get_path)
    );
    // SAFETY: GetProcAddress returned these exact Win32 entry points.
    let get_packages: GetPackagesByPackageFamily = unsafe { core::mem::transmute(get_packages) };
    // SAFETY: GetProcAddress returned this exact Win32 entry point.
    let get_path: GetPackagePathByFullName = unsafe { core::mem::transmute(get_path) };

    let family: Box<[u16]> = "Microsoft.D3DMappingLayers_8wekyb3d8bbwe\0"
        .encode_utf16()
        .collect();
    let (mut count, mut buffer_length) = (0, 0);
    // SAFETY: This is the documented sizing query; output buffers are null.
    if unsafe {
        get_packages(
            family.as_ptr(),
            &mut count,
            core::ptr::null_mut(),
            &mut buffer_length,
            core::ptr::null_mut(),
        )
    } != ERROR_INSUFFICIENT_BUFFER
        || count == 0
        || buffer_length == 0
    {
        return None;
    }

    let mut names = Box::<[u16]>::new_uninit_slice(usize::try_from(buffer_length).ok()?);
    let mut packages = Box::<[*mut u16]>::new_uninit_slice(usize::try_from(count).ok()?);
    // SAFETY: Both buffers have exactly the capacities returned by the sizing
    // query, and the API initializes them on success.
    if unsafe {
        get_packages(
            family.as_ptr(),
            &mut count,
            packages.as_mut_ptr().cast(),
            &mut buffer_length,
            names.as_mut_ptr().cast(),
        )
    } != ERROR_SUCCESS
    {
        return None;
    }
    // SAFETY: A successful call initialized at least the first pointer because
    // the sizing query returned a nonzero package count.
    let packages = unsafe { packages.assume_init() };
    let package = *packages.first()?;

    let mut path_length = 0;
    // SAFETY: This is the documented path sizing query.
    if unsafe { get_path(package, &mut path_length, core::ptr::null_mut()) }
        != ERROR_INSUFFICIENT_BUFFER
        || path_length == 0
        || path_length > 260
    {
        return None;
    }
    // Upstream zero-initializes MAX_PATH before the call. The API's returned
    // length includes room for NUL, but providers (including the parity shim)
    // need not overwrite that final unit themselves.
    let mut path = vec![0_u16; usize::try_from(path_length).ok()?].into_boxed_slice();
    // SAFETY: `path` has the capacity returned by the sizing query.
    if unsafe { get_path(package, &mut path_length, path.as_mut_ptr()) } != ERROR_SUCCESS {
        return None;
    }
    let length = path
        .iter()
        .position(|unit| *unit == 0)
        .unwrap_or(path.len());
    Some(PathBuf::from(OsString::from_wide(&path[..length])))
}

#[cfg(windows)]
pub(crate) fn settings_files() -> Box<[PathBuf]> {
    let location = c"SOFTWARE\\Khronos\\Vulkan\\LoaderSettings";
    let mut paths = Vec::new();
    if !has_elevated_privileges() {
        paths.extend(registry_values(location, true));
    }
    paths.extend(registry_values(location, false));
    paths.into_boxed_slice()
}

/// Reads a file through the C runtime.
///
/// The upstream test harness interposes the platform C API to provide an
/// isolated filesystem. Using `fopen` here is consequently part of parity,
/// rather than an interchangeable implementation detail.
#[cfg(unix)]
pub(crate) fn read_file(path: &Path) -> Option<Box<[u8]>> {
    let path = CString::new(path.as_os_str().as_encoded_bytes()).ok()?;
    // SAFETY: Both strings are NUL-terminated and live for the call.
    let file = unsafe { fopen()(path.as_ptr(), c"rb".as_ptr()) };
    if file.is_null() {
        return None;
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
            return None;
        }
        // SAFETY: The preceding `fstat` call initialized the complete object.
        let metadata = unsafe { metadata.assume_init() };
        let length = usize::try_from(metadata.st_size).ok()?;
        let mut bytes = Box::<[u8]>::new_uninit_slice(length);
        // SAFETY: `bytes` has writable capacity for exactly `length` bytes.
        let read = unsafe { libc::fread(bytes.as_mut_ptr().cast::<c_void>(), 1, length, file) };
        // SAFETY: `fread` initialized every element when it returned `length`.
        (read == length).then(|| unsafe { bytes.assume_init() })
    })();

    // SAFETY: This function owns the open stream.
    unsafe { libc::fclose(file) };
    result
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn read_file(path: &Path) -> Option<Box<[u8]>> {
    std::fs::read(path).ok().map(Vec::into_boxed_slice)
}

#[cfg(windows)]
pub(crate) fn read_file(path: &Path) -> Option<Box<[u8]>> {
    std::fs::read(path).ok().map(Vec::into_boxed_slice)
}

/// Tests whether a file exists through the platform API used by upstream.
///
/// Besides matching Vulkan-Loader's discovery semantics, using `access` is
/// required for filesystem interposition in applications such as its original
/// test harness.
#[cfg(unix)]
pub(crate) fn file_exists(path: &Path) -> bool {
    let Ok(path) = CString::new(path.as_os_str().as_encoded_bytes()) else {
        return false;
    };
    // SAFETY: `path` is NUL-terminated and live for the call; `F_OK` only
    // performs an existence check.
    unsafe { access()(path.as_ptr(), libc::F_OK) == 0 }
}

#[cfg(unix)]
pub(crate) fn write_stderr(message: &str) {
    let Ok(message) = CString::new(message) else {
        return;
    };
    let stream = stderr_stream();
    if stream.is_null() {
        return;
    }
    // SAFETY: `message` is NUL-terminated and the C runtime owns `stream`.
    unsafe {
        fputs()(message.as_ptr(), stream);
    }
}

fn loader_debug_filter_matches(filters: &str, name: &str) -> bool {
    filters.split(',').any(|filter| {
        if filter.is_empty() {
            return false;
        }
        // Upstream compares the option length rather than the keyword length.
        // Consequently non-empty, case-sensitive prefixes are accepted.
        if "all".starts_with(filter) {
            return true;
        }
        let canonical = if name == "warning" { "warn" } else { name };
        canonical.starts_with(filter)
            || (name == "driver"
                && ["driver", "implem", "icd"]
                    .into_iter()
                    .any(|keyword| keyword.starts_with(filter)))
    })
}

pub(crate) fn loader_debug_filter_enabled(name: &str) -> bool {
    std::env::var("VK_LOADER_DEBUG")
        .is_ok_and(|filters| loader_debug_filter_matches(&filters, name))
}

pub(crate) fn write_loader_log(filter: &str, label: &str, message: core::fmt::Arguments<'_>) {
    if !loader_debug_filter_enabled(filter) {
        return;
    }
    write_loader_log_enabled(label, message);
}

pub(crate) fn write_loader_log_with_category(
    severity_filter: &str,
    severity_label: &str,
    category_filter: &str,
    category_label: &str,
    message: core::fmt::Arguments<'_>,
) {
    if !loader_debug_filter_enabled(severity_filter)
        && !loader_debug_filter_enabled(category_filter)
    {
        return;
    }
    let label = if severity_label.is_empty() {
        category_label.to_owned()
    } else {
        format!("{severity_label} | {category_label}")
    };
    write_loader_log_enabled(&label, message);
}

pub(crate) fn write_loader_category_log(
    category_filter: &str,
    category_label: &str,
    message: core::fmt::Arguments<'_>,
) {
    if loader_debug_filter_enabled(category_filter) {
        write_loader_log_enabled(category_label, message);
    }
}

pub(crate) fn write_loader_category_log_any(
    category_filters: &[&str],
    category_label: &str,
    message: core::fmt::Arguments<'_>,
) {
    if category_filters
        .iter()
        .any(|filter| loader_debug_filter_enabled(filter))
    {
        write_loader_log_enabled(category_label, message);
    }
}

fn write_loader_log_enabled(label: &str, message: core::fmt::Arguments<'_>) {
    let prefix = format!("[Vulkan Loader] {label}: ");
    let padding = 32_usize.saturating_sub(prefix.len());
    write_stderr(&format!("{prefix}{:padding$}{message}\n", ""));
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn write_stderr(message: &str) {
    use std::io::Write;

    let _ = std::io::stderr().lock().write_all(message.as_bytes());
}

#[cfg(windows)]
pub(crate) fn write_stderr(message: &str) {
    use windows_sys::Win32::{
        Foundation::INVALID_HANDLE_VALUE,
        Storage::FileSystem::WriteFile,
        System::{
            Console::{GetStdHandle, STD_ERROR_HANDLE},
            Diagnostics::Debug::OutputDebugStringA,
        },
    };

    // Upstream writes every loader log message to stderr before mirroring it
    // to the debugger. Death tests and console applications rely on the first
    // channel; GUI debuggers and the Windows parity shim rely on the second.
    // Avoid `std::io::Stderr`, whose reentrant lock uses a Windows TLS index
    // that is not reclaimed when a Rust DLL is unloaded.
    let handle = unsafe { GetStdHandle(STD_ERROR_HANDLE) };
    if !handle.is_null() && handle != INVALID_HANDLE_VALUE {
        let mut remaining = message.as_bytes();
        while !remaining.is_empty() {
            let request = remaining.len().min(u32::MAX as usize) as u32;
            let mut written = 0;
            // SAFETY: The standard-error handle is borrowed, and `remaining`
            // supplies at least `request` readable bytes.
            let succeeded = unsafe {
                WriteFile(
                    handle,
                    remaining.as_ptr().cast(),
                    request,
                    &raw mut written,
                    core::ptr::null_mut(),
                )
            };
            if succeeded == 0 || written == 0 {
                break;
            }
            remaining = &remaining[written as usize..];
        }
    }
    if let Ok(message) = std::ffi::CString::new(message) {
        // SAFETY: The message is NUL-terminated and remains live for the call.
        unsafe { OutputDebugStringA(message.as_ptr().cast()) };
    }
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn file_exists(path: &Path) -> bool {
    path.exists()
}

#[cfg(windows)]
pub(crate) fn file_exists(path: &Path) -> bool {
    path.exists()
}

pub(crate) fn manifest_files(path: &Path) -> Box<[PathBuf]> {
    if path
        .extension()
        .is_some_and(|extension| extension == "json")
    {
        return Box::from([path.to_owned()]);
    }

    let Some(files) = read_directory(path) else {
        return Box::default();
    };
    let mut files = files.into_vec();
    files.retain(|entry| {
        entry
            .extension()
            .is_some_and(|extension| extension == "json")
    });
    files.into_boxed_slice()
}

#[cfg(unix)]
fn read_directory(path: &Path) -> Option<Box<[PathBuf]>> {
    use core::ffi::CStr;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let path_c = CString::new(path.as_os_str().as_bytes()).ok()?;
    // SAFETY: `path_c` is a live, NUL-terminated path.
    let directory = unsafe { opendir()(path_c.as_ptr()) };
    if directory.is_null() {
        return None;
    }
    let mut entries = Vec::new();
    loop {
        // SAFETY: `directory` remains open and access is serialized by this function.
        let entry = unsafe { readdir()(directory) };
        if entry.is_null() {
            break;
        }
        // SAFETY: POSIX guarantees a NUL-terminated `d_name` in a live dirent.
        let name = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
        if name != b"." && name != b".." {
            entries.push(path.join(OsStr::from_bytes(name)));
        }
    }
    // SAFETY: This function owns the open directory stream.
    unsafe { closedir()(directory) };
    Some(entries.into_boxed_slice())
}

#[cfg(not(unix))]
fn read_directory(path: &Path) -> Option<Box<[PathBuf]>> {
    Some(
        path.read_dir()
            .ok()?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHILD_MODE: &str = "VK_LOADER_RUST_UNLOADING_POLICY_TEST";

    #[test]
    fn debug_filter_matches_upstream_prefix_rules() {
        assert!(loader_debug_filter_matches("all", "info"));
        assert!(loader_debug_filter_matches("a", "layer"));
        assert!(loader_debug_filter_matches("w", "warn"));
        assert!(loader_debug_filter_matches("warn", "warning"));
        assert!(loader_debug_filter_matches("implem", "driver"));
        assert!(loader_debug_filter_matches("ic", "driver"));
        assert!(loader_debug_filter_matches("debug,driver", "driver"));
        assert!(!loader_debug_filter_matches("", "info"));
        assert!(!loader_debug_filter_matches("INFO", "info"));
        assert!(!loader_debug_filter_matches("warnings", "warning"));
        assert!(!loader_debug_filter_matches("layer", "driver"));
    }

    #[test]
    fn unloading_policy_is_exact_and_process_wide() {
        match std::env::var(CHILD_MODE).as_deref() {
            Ok("disabled") => {
                assert!(dynamic_library_unloading_disabled());
                // SAFETY: The subprocess runs only this exact test, and the
                // policy has already copied the environment value.
                unsafe { std::env::set_var("VK_LOADER_DISABLE_DYNAMIC_LIBRARY_UNLOADING", "0") };
                assert!(dynamic_library_unloading_disabled());
            }
            Ok("not-disabled") => {
                assert!(!dynamic_library_unloading_disabled());
            }
            _ => {
                for (mode, value) in [("disabled", "1"), ("not-disabled", "10")] {
                    let status = std::process::Command::new(
                        std::env::current_exe().expect("test executable path"),
                    )
                    .args([
                        "--exact",
                        "platform::tests::unloading_policy_is_exact_and_process_wide",
                    ])
                    .env(CHILD_MODE, mode)
                    .env("VK_LOADER_DISABLE_DYNAMIC_LIBRARY_UNLOADING", value)
                    .status()
                    .expect("spawn policy subprocess");
                    assert!(status.success());
                }
            }
        }
    }
}
