//! Recursive loader synchronization and thread identity.

mod bootstrap;

#[cfg(not(any(unix, windows)))]
use core::hash::{Hash, Hasher};
use core::{cell::UnsafeCell, marker::PhantomData, mem::MaybeUninit};
use std::sync::MutexGuard;
#[cfg(windows)]
use windows_sys::Win32::Foundation::{ERROR_NOT_ENOUGH_MEMORY, ERROR_OUTOFMEMORY, GetLastError};

#[cfg(windows)]
pub(crate) struct LoaderLock {
    critical_section:
        UnsafeCell<MaybeUninit<windows_sys::Win32::System::Threading::CRITICAL_SECTION>>,
    initialized: bootstrap::Initialization,
}

#[cfg(windows)]
// SAFETY: The uninitialized value can move, but initialization requires a
// static receiver, preventing subsequent movement of the native lock.
unsafe impl Send for LoaderLock {}
#[cfg(windows)]
// SAFETY: The bootstrap mutex serializes initialization; the OS serializes locking.
unsafe impl Sync for LoaderLock {}

#[cfg(windows)]
impl LoaderLock {
    const fn new() -> Self {
        Self {
            critical_section: UnsafeCell::new(MaybeUninit::uninit()),
            initialized: bootstrap::Initialization::new(),
        }
    }

    pub(super) fn initialize(&'static self) -> Result<(), vk::VkResult> {
        self.initialized.run(|| {
            // SAFETY: The bootstrap guard provides exclusive initialization of writable
            // storage at its final static address, before any lock operation.
            let initialized = unsafe {
                windows_sys::Win32::System::Threading::InitializeCriticalSectionEx(
                    self.critical_section.get().cast(),
                    0,
                    0,
                )
            };
            if initialized == 0 {
                // SAFETY: Read the error immediately after the failed native call.
                return Err(match unsafe { GetLastError() } {
                    ERROR_NOT_ENOUGH_MEMORY | ERROR_OUTOFMEMORY => {
                        vk::VkResult::ERROR_OUT_OF_HOST_MEMORY
                    }
                    _ => vk::VkResult::ERROR_INITIALIZATION_FAILED,
                });
            }
            Ok(())
        })
    }

    fn lock(&'static self) -> LoaderLockGuard<'static> {
        self.initialize().unwrap_or_else(|_| std::process::abort());
        // SAFETY: The critical section is initialized and remains live for the
        // process lifetime. Windows critical sections are recursive, matching
        // Vulkan-Loader's loader_lock contract.
        unsafe {
            windows_sys::Win32::System::Threading::EnterCriticalSection(
                self.critical_section.get().cast(),
            )
        };
        LoaderLockGuard {
            lock: self,
            thread_bound: PhantomData,
        }
    }

    pub(super) unsafe fn destroy(&self) {
        if !self.initialized.is_completed() {
            return;
        }
        // SAFETY: The caller guarantees that no loader entry point can still
        // acquire or hold the critical section during DLL termination.
        unsafe {
            windows_sys::Win32::System::Threading::DeleteCriticalSection(
                self.critical_section.get().cast(),
            )
        };
    }
}

pub(crate) struct LoaderLockGuard<'a> {
    lock: &'a LoaderLock,
    thread_bound: PhantomData<MutexGuard<'a, ()>>,
}

#[cfg(windows)]
impl Drop for LoaderLockGuard<'_> {
    fn drop(&mut self) {
        // SAFETY: This guard represents exactly one successful recursive
        // acquisition by the current thread.
        unsafe {
            windows_sys::Win32::System::Threading::LeaveCriticalSection(
                self.lock.critical_section.get().cast(),
            )
        };
    }
}

pub(crate) fn lock_loader() -> LoaderLockGuard<'static> {
    LOADER_LOCK.lock()
}

pub(crate) fn try_lock_loader() -> Result<LoaderLockGuard<'static>, vk::VkResult> {
    LOADER_LOCK.initialize()?;
    Ok(LOADER_LOCK.lock())
}

#[cfg(not(windows))]
pub(crate) struct LoaderLock {
    mutex: UnsafeCell<MaybeUninit<libc::pthread_mutex_t>>,
    initialized: bootstrap::Initialization,
}

#[cfg(not(windows))]
// SAFETY: Initialization requires a static receiver, so only uninitialized
// native storage can move through this safe interface.
unsafe impl Send for LoaderLock {}
#[cfg(not(windows))]
// SAFETY: The bootstrap mutex serializes initialization and pthread serializes locking.
unsafe impl Sync for LoaderLock {}

#[cfg(not(windows))]
impl LoaderLock {
    const fn new() -> Self {
        Self {
            mutex: UnsafeCell::new(MaybeUninit::uninit()),
            initialized: bootstrap::Initialization::new(),
        }
    }

    pub(super) fn initialize(&'static self) -> Result<(), vk::VkResult> {
        self.initialized.run(|| {
            let mut attributes = core::mem::MaybeUninit::<libc::pthread_mutexattr_t>::uninit();
            // SAFETY: The storage is writable and initialized on success.
            initialization_result(unsafe {
                libc::pthread_mutexattr_init(attributes.as_mut_ptr())
            })?;
            #[cfg(target_os = "hurd")]
            let recursive_mutex_type = libc::PTHREAD_MUTEX_RECURSIVE as libc::c_int;
            #[cfg(not(target_os = "hurd"))]
            let recursive_mutex_type = libc::PTHREAD_MUTEX_RECURSIVE;
            // SAFETY: The attributes object is initialized and exclusively owned.
            let result = unsafe {
                libc::pthread_mutexattr_settype(attributes.as_mut_ptr(), recursive_mutex_type)
            };
            if result != 0 {
                // SAFETY: The attributes object was initialized successfully.
                unsafe { libc::pthread_mutexattr_destroy(attributes.as_mut_ptr()) };
                return initialization_result(result);
            }

            // SAFETY: The bootstrap guard excludes concurrent initialization. The static receiver
            // keeps the native mutex at its final address, and attributes is live.
            let result =
                unsafe { libc::pthread_mutex_init(self.mutex.get().cast(), attributes.as_ptr()) };
            // SAFETY: The attributes object is no longer needed after mutex init.
            unsafe { libc::pthread_mutexattr_destroy(attributes.as_mut_ptr()) };
            initialization_result(result)
        })
    }

    fn lock(&'static self) -> LoaderLockGuard<'static> {
        self.initialize().unwrap_or_else(|_| std::process::abort());
        // SAFETY: The native mutex is initialized and remains live for the
        // guard. POSIX recursive mutexes match upstream loader_lock semantics.
        if unsafe { libc::pthread_mutex_lock(self.mutex.get().cast()) } != 0 {
            std::process::abort();
        }
        LoaderLockGuard {
            lock: self,
            thread_bound: PhantomData,
        }
    }

    #[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
    pub(super) unsafe fn destroy(&self) {
        if !self.initialized.is_completed() {
            return;
        }
        // SAFETY: The caller guarantees that no loader entry point can still
        // acquire or hold the mutex during library termination.
        let result = unsafe { libc::pthread_mutex_destroy(self.mutex.get().cast()) };
        debug_assert_eq!(result, 0);
    }
}

#[cfg(not(windows))]
impl Drop for LoaderLockGuard<'_> {
    fn drop(&mut self) {
        // SAFETY: This guard represents exactly one successful recursive
        // acquisition by the current thread.
        if unsafe { libc::pthread_mutex_unlock(self.lock.mutex.get().cast()) } != 0 {
            std::process::abort();
        }
    }
}

pub(super) static LOADER_LOCK: LoaderLock = LoaderLock::new();

#[cfg(not(windows))]
fn initialization_result(result: libc::c_int) -> Result<(), vk::VkResult> {
    match result {
        0 => Ok(()),
        libc::ENOMEM | libc::EAGAIN => Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY),
        _ => Err(vk::VkResult::ERROR_INITIALIZATION_FAILED),
    }
}

#[cfg(all(
    not(windows),
    not(all(target_vendor = "apple", feature = "apple-static-loader"))
))]
pub(super) unsafe fn destroy_bootstrap() {
    // SAFETY: The caller excludes all current and future loader entries.
    unsafe { bootstrap::destroy() };
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
    let thread = unsafe { libc::pthread_self() };
    thread as usize
}

#[cfg(not(any(unix, windows)))]
#[inline]
pub(crate) fn current_thread_key() -> usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    hasher.finish() as usize
}

#[cfg(test)]
mod tests {
    use super::LoaderLock;
    use core::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn recursive_lock_serializes_concurrent_initialization_and_access() {
        static LOCK: LoaderLock = LoaderLock::new();
        let count = AtomicUsize::new(0);
        let start = std::sync::Barrier::new(4);
        std::thread::scope(|scope| {
            for _ in 0..4 {
                scope.spawn(|| {
                    start.wait();
                    for _ in 0..1000 {
                        let outer = LOCK.lock();
                        let inner = LOCK.lock();
                        let previous = count.load(Ordering::Relaxed);
                        drop(inner);
                        // Dropping the nested guard must leave the outer lock
                        // held, excluding other threads across this update.
                        count.store(previous + 1, Ordering::Relaxed);
                        drop(outer);
                    }
                });
            }
        });
        assert_eq!(count.load(Ordering::Relaxed), 4000);
        #[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
        // SAFETY: All threads have joined; this test alone owns use of LOCK
        // and never acquires it again after destruction.
        unsafe {
            LOCK.destroy();
        }
    }
}
