//! Fallible global locking with lazy, allocation-free registry construction.

use super::{GlobalMutex, MutexAcquire};
use core::{
    cell::LazyCell,
    ops::{Deref, DerefMut},
};
use vk::VkResult;

pub(crate) struct GlobalLazyMutex<T>(GlobalMutex<LazyCell<T>>);

impl<T> GlobalLazyMutex<T> {
    pub(crate) const fn new(initialize: fn() -> T) -> Self {
        Self(GlobalMutex::new(LazyCell::new(initialize)))
    }

    pub(crate) fn try_lock(&'static self) -> Result<impl DerefMut<Target = T>, VkResult> {
        let mut guard = self.0.try_lock()?;
        LazyCell::force_mut(&mut guard);
        Ok(LazyGuard(guard))
    }

    pub(crate) fn lock_if_initialized(&'static self) -> Option<impl DerefMut<Target = T>> {
        self.0.lock_if_initialized().map(LazyGuard)
    }

    /// # Safety
    /// A successful `try_lock` must have initialized this mutex, and library
    /// termination must not have destroyed it since then.
    pub(crate) unsafe fn lock_initialized(&'static self) -> impl DerefMut<Target = T> {
        // SAFETY: The caller supplies the prior-initialization guarantee.
        LazyGuard(unsafe { self.0.lock_initialized_unchecked() })
    }

    #[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
    pub(crate) unsafe fn destroy(&self) {
        // SAFETY: The caller excludes all guards, waiters, and future use.
        unsafe { self.0.destroy() };
    }
}

struct LazyGuard<G>(G);

impl<T, G: Deref<Target = LazyCell<T>>> Deref for LazyGuard<G> {
    type Target = T;
    fn deref(&self) -> &T {
        LazyCell::force(&self.0)
    }
}

impl<T, G: DerefMut<Target = LazyCell<T>>> DerefMut for LazyGuard<G> {
    fn deref_mut(&mut self) -> &mut T {
        LazyCell::force_mut(&mut self.0)
    }
}
