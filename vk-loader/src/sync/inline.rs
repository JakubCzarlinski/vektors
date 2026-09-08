//! Standard mutex backend on targets with inline synchronization storage.

use super::MutexInit;

pub(crate) struct Mutex<T>(std::sync::Mutex<T>);

impl<T> Mutex<T> {
    pub(in crate::sync) const fn new(value: T) -> Self {
        Self(std::sync::Mutex::new(value))
    }

    pub(crate) fn lock(&self) -> std::sync::MutexGuard<'_, T> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

impl<T> MutexInit<T> for Mutex<T> {
    fn try_new(value: T) -> Result<Self, vk::VkResult> {
        Ok(Self::new(value))
    }
}
