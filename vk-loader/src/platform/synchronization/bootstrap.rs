//! Initialization serialization independent of the recursive loader lock.

use core::sync::atomic::{AtomicBool, Ordering};
#[cfg(not(windows))]
use core::{cell::UnsafeCell, marker::PhantomData};
#[cfg(not(windows))]
use std::sync::MutexGuard;

pub(super) struct Initialization(AtomicBool);

impl Initialization {
    pub(super) const fn new() -> Self {
        Self(AtomicBool::new(false))
    }

    pub(super) fn is_completed(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    /// Runs native initialization exclusively and publishes only success.
    /// The operation must not recursively initialize another loader lock.
    pub(super) fn run(
        &self,
        initialize: impl FnOnce() -> Result<(), vk::VkResult>,
    ) -> Result<(), vk::VkResult> {
        if self.is_completed() {
            return Ok(());
        }
        #[cfg(not(windows))]
        let _guard = lock()?;
        #[cfg(windows)]
        let _guard = lock();
        if !self.0.load(Ordering::Relaxed) {
            initialize()?;
            self.0.store(true, Ordering::Release);
        }
        Ok(())
    }
}

#[cfg(windows)]
static BOOTSTRAP: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(not(windows))]
struct Bootstrap(UnsafeCell<libc::pthread_mutex_t>);

#[cfg(not(windows))]
// SAFETY: The native mutex is accessed only through pthread operations, stays
// at its static address, and is destroyed only after all loader calls stop.
unsafe impl Sync for Bootstrap {}

#[cfg(not(windows))]
static BOOTSTRAP: Bootstrap = Bootstrap(UnsafeCell::new(libc::PTHREAD_MUTEX_INITIALIZER));

#[cfg(not(windows))]
struct Guard(PhantomData<MutexGuard<'static, ()>>);

#[cfg(not(windows))]
impl Drop for Guard {
    fn drop(&mut self) {
        // SAFETY: This thread acquired the native bootstrap mutex exactly once.
        let result = unsafe { libc::pthread_mutex_unlock(BOOTSTRAP.0.get()) };
        debug_assert_eq!(result, 0);
    }
}

#[cfg(not(windows))]
fn lock() -> Result<Guard, vk::VkResult> {
    // SAFETY: The static initializer creates valid, stationary mutex storage.
    // No recursive initializer acquires this lock while it is held.
    let result = unsafe { libc::pthread_mutex_lock(BOOTSTRAP.0.get()) };
    super::initialization_result(result)?;
    Ok(Guard(PhantomData))
}

#[cfg(windows)]
fn lock() -> std::sync::MutexGuard<'static, ()> {
    // Windows' standard mutex uses inline native storage, not Once's thread
    // waiter queue. No loader state is exposed through this guard.
    BOOTSTRAP
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(all(
    not(windows),
    not(all(target_vendor = "apple", feature = "apple-static-loader"))
))]
pub(super) unsafe fn destroy() {
    // SAFETY: The caller excludes all current and future loader entries.
    let result = unsafe { libc::pthread_mutex_destroy(BOOTSTRAP.0.get()) };
    debug_assert_eq!(result, 0);
}

#[cfg(test)]
mod tests {
    use super::Initialization;
    use core::sync::atomic::{AtomicUsize, Ordering};
    use vk::VkResult;

    #[test]
    fn failed_initialization_is_unpublished_and_retryable() {
        let initialization = Initialization::new();
        for error in [
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
        ] {
            assert_eq!(initialization.run(|| Err(error)), Err(error));
            assert!(!initialization.is_completed());
        }
        assert_eq!(initialization.run(|| Ok(())), Ok(()));
        assert!(initialization.is_completed());
        assert_eq!(initialization.run(|| panic!("already initialized")), Ok(()));
    }

    #[test]
    fn concurrent_initializers_run_once_and_observe_published_state() {
        let initialization = Initialization::new();
        let calls = AtomicUsize::new(0);
        let published = AtomicUsize::new(0);
        let start = std::sync::Barrier::new(4);
        std::thread::scope(|scope| {
            for _ in 0..4 {
                scope.spawn(|| {
                    start.wait();
                    initialization
                        .run(|| {
                            calls.fetch_add(1, Ordering::Relaxed);
                            published.store(42, Ordering::Relaxed);
                            Ok(())
                        })
                        .unwrap();
                    assert_eq!(published.load(Ordering::Relaxed), 42);
                });
            }
        });
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }
}
