//! Synchronization primitives whose storage remains owned by the loader.
//!
//! `parking_lot` keeps process-global and thread-local heap state after lock
//! contention.  A Vulkan loader may be unloaded with `dlclose`, so that state
//! would outlive the code which owns its destructors.  The standard mutex is
//! inline on supported loader platforms and has no such library-lifetime
//! dependency.

pub(crate) struct Mutex<T>(std::sync::Mutex<T>);

impl<T> Mutex<T> {
    pub(crate) const fn new(value: T) -> Self {
        Self(std::sync::Mutex::new(value))
    }

    pub(crate) fn lock(&self) -> std::sync::MutexGuard<'_, T> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}
