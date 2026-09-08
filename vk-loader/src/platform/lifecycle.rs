//! Loader initialization, termination, and unloading policy.

#[cfg(unix)]
use super::environment::inspect_environment;
use super::{LogFilter, synchronization::LOADER_LOCK, try_lock_loader, write_loader_log};
#[cfg(windows)]
use core::ffi::c_void;
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};
#[cfg(windows)]
use windows_sys::{Win32::System::Environment::GetEnvironmentVariableW, core::w};

pub(super) fn dynamic_library_unloading_disabled() -> bool {
    // bool converts to 0 or 1, leaving 2 to represent an unread environment.
    const UNINITIALIZED: u8 = 2;
    static DISABLED: AtomicU8 = AtomicU8::new(UNINITIALIZED);
    let cached = DISABLED.load(Ordering::Acquire);
    if cached != UNINITIALIZED {
        return cached != 0;
    }
    let disabled = {
        #[cfg(unix)]
        {
            // SAFETY: Initialization follows upstream's exclusion of concurrent
            // environment mutation; this callback only compares borrowed bytes.
            unsafe {
                inspect_environment(c"VK_LOADER_DISABLE_DYNAMIC_LIBRARY_UNLOADING", |value| {
                    value.is_some_and(|value| value == c"1")
                })
            }
        }
        #[cfg(windows)]
        {
            let mut value = [0_u16; 2];
            // SAFETY: The name is terminated and the stack buffer has exactly
            // the supplied extent. Longer values return their required size;
            // only a successful one-character value can equal "1".
            let length = unsafe {
                GetEnvironmentVariableW(
                    w!("VK_LOADER_DISABLE_DYNAMIC_LIBRARY_UNLOADING"),
                    value.as_mut_ptr(),
                    2,
                )
            };
            length == 1 && value[0] == u16::from(b'1')
        }
        #[cfg(not(any(unix, windows)))]
        {
            std::env::var_os("VK_LOADER_DISABLE_DYNAMIC_LIBRARY_UNLOADING")
                .is_some_and(|value| value == "1")
        }
    };
    // Environment mutation is excluded during loader use. Concurrent first
    // reads may repeat, but every caller observes the first published value.
    // Unlike OnceLock's pthread backend this never creates a thread handle.
    match DISABLED.compare_exchange(
        UNINITIALIZED,
        u8::from(disabled),
        Ordering::AcqRel,
        Ordering::Acquire,
    ) {
        Ok(_) => disabled,
        Err(cached) => cached != 0,
    }
}

pub(crate) fn initialize_loader() -> Result<(), vk::VkResult> {
    static LOG_INITIALIZED: AtomicBool = AtomicBool::new(false);
    LOADER_LOCK.initialize()?;
    if LOG_INITIALIZED.load(Ordering::Acquire) {
        return Ok(());
    }
    // Reuse the native loader lock instead of a second Once waiter queue,
    // which can allocate Rust thread state on foreign pthreads. Keep the lock
    // until all startup messages have been emitted, so concurrent entries
    // cannot return early or print their diagnostics ahead of initialization.
    let _guard = try_lock_loader()?;
    if !LOG_INITIALIZED.load(Ordering::Relaxed) {
        log_initialization();
        LOG_INITIALIZED.store(true, Ordering::Release);
    }
    Ok(())
}

fn log_initialization() {
    let version = vk::VK_HEADER_VERSION_COMPLETE;
    write_loader_log(
        LogFilter::Info,
        format_args!(
            "Vulkan Loader Version {}.{}.{}",
            vk::VK_API_VERSION_MAJOR(version),
            vk::VK_API_VERSION_MINOR(version),
            vk::VK_API_VERSION_PATCH(version)
        ),
    );
    write_loader_log(
        LogFilter::Info,
        format_args!(
            "[Vulkan Loader Git - Tag: {}, Branch/Commit: {}]",
            env!("VK_LOADER_GIT_BRANCH_NAME"),
            env!("VK_LOADER_GIT_TAG_INFO")
        ),
    );
    if dynamic_library_unloading_disabled() {
        write_loader_log(
            LogFilter::Warning,
            format_args!("Vulkan Loader: library unloading is disabled"),
        );
    }
}

#[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
fn release_loader() {
    crate::icd::unload_preloaded_icds();
    crate::discovery::release_global_loader_settings();
    // SAFETY: Library termination excludes loader entries, and unloading above
    // released every retained ICD and its preloaded-list guard.
    unsafe { crate::icd::destroy_preloaded_icd_lock() };
    // SAFETY: The same termination exclusion applies to the cleared settings cache.
    unsafe { crate::discovery::destroy_global_settings_lock() };
    // SAFETY: No instance registry operations or guards can remain at termination.
    unsafe { crate::instance::destroy_instance_registry_lock() };
    // SAFETY: Device registry users are likewise excluded during termination.
    unsafe { crate::device::destroy_device_registry_lock() };
    // SAFETY: Pending create chains and error scopes cannot remain at termination.
    unsafe { crate::pending::destroy_thread_state_lock() };
    // SAFETY: Library termination runs only after loader entry points have
    // stopped executing, and the lock was initialized during library startup.
    unsafe { LOADER_LOCK.destroy() };
    #[cfg(not(windows))]
    // SAFETY: No loader-lock initialization can occur after termination.
    unsafe {
        super::synchronization::destroy_bootstrap();
    };
}

#[cfg(any(
    all(unix, not(target_vendor = "apple")),
    all(target_vendor = "apple", not(feature = "apple-static-loader"))
))]
unsafe extern "C" fn initialize_loader_library() {
    // ELF/Mach-O constructors cannot return a Vulkan error. Leave initialization
    // unpublished on failure so the first entry point can retry and report it.
    let _ = initialize_loader();
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
            // A TLS callback has no error return; Vulkan entry points retry.
            let _ = LOADER_LOCK.initialize();
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
