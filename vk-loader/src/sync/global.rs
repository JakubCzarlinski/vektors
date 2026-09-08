//! Global mutex selection: static pthread storage or an inline standard backend.

#[cfg(not(all(
    unix,
    not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "fuchsia"
    ))
)))]
mod inline {
    use super::super::MutexAcquire;
    use super::super::inline::Mutex;
    use core::sync::atomic::{AtomicBool, Ordering};
    use std::sync::MutexGuard;

    pub(crate) struct GlobalMutex<T> {
        mutex: Mutex<T>,
        used: AtomicBool,
    }

    impl<T> GlobalMutex<T> {
        pub(crate) const fn new(value: T) -> Self {
            Self {
                mutex: Mutex::new(value),
                used: AtomicBool::new(false),
            }
        }

        pub(crate) fn lock_if_initialized(&self) -> Option<MutexGuard<'_, T>> {
            self.used.load(Ordering::Acquire).then(|| self.mutex.lock())
        }

        /// The caller has already successfully initialized this global mutex.
        pub(crate) unsafe fn lock_initialized_unchecked(&self) -> MutexGuard<'_, T> {
            self.mutex.lock()
        }

        /// Inline standard backends own no separately allocated native storage.
        pub(crate) unsafe fn destroy(&self) {
            self.used.store(false, Ordering::Release);
        }
    }

    impl<T> MutexAcquire<T> for GlobalMutex<T> {
        fn try_lock(&'static self) -> Result<impl core::ops::DerefMut<Target = T>, vk::VkResult> {
            let guard = self.mutex.lock();
            self.used.store(true, Ordering::Release);
            Ok(guard)
        }
    }
}

#[cfg(all(
    unix,
    not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "fuchsia"
    ))
))]
pub(crate) use super::pthread::GlobalMutex;

#[cfg(not(all(
    unix,
    not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "fuchsia"
    ))
)))]
pub(crate) use inline::GlobalMutex;
