//! Stable pthread storage allocated before an object is published.

use super::{MutexAcquire, MutexInit};
use alloc::boxed::Box;
use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};
use vk::VkResult;

struct NativeMutex(UnsafeCell<libc::pthread_mutex_t>);
struct NativeOwner(ManuallyDrop<Box<NativeMutex>>);
struct Attributes<'a>(&'a mut MaybeUninit<libc::pthread_mutexattr_t>);

impl Drop for Attributes<'_> {
    fn drop(&mut self) {
        // SAFETY: Construction initialized these attributes and we own them.
        unsafe { libc::pthread_mutexattr_destroy(self.0.as_mut_ptr()) };
    }
}

fn check_initialization(result: libc::c_int) -> Result<(), VkResult> {
    match result {
        0 => Ok(()),
        libc::ENOMEM | libc::EAGAIN => Err(VkResult::ERROR_OUT_OF_HOST_MEMORY),
        _ => Err(VkResult::ERROR_INITIALIZATION_FAILED),
    }
}

impl NativeOwner {
    fn try_new() -> Result<Self, VkResult> {
        let native = crate::allocation::try_box(NativeMutex(UnsafeCell::new(
            libc::PTHREAD_MUTEX_INITIALIZER,
        )))
        .map_err(|(error, _)| error)?;
        // SAFETY: The boxed native mutex has its final, exclusively owned address.
        unsafe { initialize_native(native.0.get()) }?;
        Ok(Self(ManuallyDrop::new(native)))
    }

    fn pointer(&self) -> *mut libc::pthread_mutex_t {
        self.0.0.get()
    }
}

/// Initializes exclusively owned native storage at its permanent address.
unsafe fn initialize_native(native: *mut libc::pthread_mutex_t) -> Result<(), VkResult> {
    let mut attributes = MaybeUninit::uninit();
    // SAFETY: Writable storage has exactly pthread_mutexattr_t's layout.
    check_initialization(unsafe { libc::pthread_mutexattr_init(attributes.as_mut_ptr()) })?;
    let attributes = Attributes(&mut attributes);
    // SAFETY: Set NORMAL explicitly: DEFAULT permits undefined behavior on
    // recursive locking, whereas a safe Rust mutex must deadlock instead.
    check_initialization(unsafe {
        libc::pthread_mutexattr_settype(attributes.0.as_mut_ptr(), libc::PTHREAD_MUTEX_NORMAL)
    })?;
    // SAFETY: The caller's native storage has its final address. Nothing can
    // access this mutex until initialization completes successfully.
    check_initialization(unsafe { libc::pthread_mutex_init(native, attributes.0.as_ptr()) })
}

/// Inline native storage whose address is fixed by the static locking receiver.
pub(crate) struct GlobalMutex<T> {
    native: NativeMutex,
    initialized: AtomicBool,
    value: UnsafeCell<T>,
}

// SAFETY: Only uninitialized storage can move; locking requires a static borrow.
unsafe impl<T: Send> Send for GlobalMutex<T> {}
// SAFETY: Initialization is serialized and the native mutex protects all data.
unsafe impl<T: Send> Sync for GlobalMutex<T> {}

impl<T> GlobalMutex<T> {
    pub(crate) const fn new(value: T) -> Self {
        Self {
            native: NativeMutex(UnsafeCell::new(libc::PTHREAD_MUTEX_INITIALIZER)),
            initialized: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    /// The initializer must initialize a NORMAL mutex on success and leave no
    /// live native resources on failure. It runs with exclusive initialization.
    unsafe fn try_lock_with(
        &'static self,
        initialize: impl FnOnce(*mut libc::pthread_mutex_t) -> Result<(), VkResult>,
    ) -> Result<GlobalMutexGuard<'static, T>, VkResult> {
        if !self.initialized.load(Ordering::Acquire) {
            // Only initialization takes the recursive loader lock. Release it
            // before waiting on this mutex, avoiding a global-to-local lock order.
            let initialization = crate::platform::try_lock_loader()?;
            if !self.initialized.load(Ordering::Acquire) {
                initialize(self.native.0.get())?;
                self.initialized.store(true, Ordering::Release);
            }
            drop(initialization);
        }
        Ok(self.lock_initialized())
    }

    pub(crate) fn lock_if_initialized(&'static self) -> Option<GlobalMutexGuard<'static, T>> {
        self.initialized
            .load(Ordering::Acquire)
            .then(|| self.lock_initialized())
    }

    fn lock_initialized(&self) -> GlobalMutexGuard<'_, T> {
        // SAFETY: Every caller observed completed initialization. The NORMAL,
        // non-robust mutex is live and the guard retains the matching borrow.
        if unsafe { libc::pthread_mutex_lock(self.native.0.get()) } != 0 {
            std::process::abort();
        }
        GlobalMutexGuard {
            mutex: self,
            thread_bound: PhantomData,
        }
    }

    /// The caller guarantees successful initialization and excludes termination.
    pub(crate) unsafe fn lock_initialized_unchecked(&self) -> GlobalMutexGuard<'_, T> {
        self.lock_initialized()
    }

    /// Releases native resources after all loader entry points have stopped.
    #[cfg(any(
        test,
        not(all(target_vendor = "apple", feature = "apple-static-loader"))
    ))]
    pub(crate) unsafe fn destroy(&self) {
        if self.initialized.load(Ordering::Acquire) {
            // SAFETY: The caller excludes guards, waiters, and future accesses.
            let result = unsafe { libc::pthread_mutex_destroy(self.native.0.get()) };
            debug_assert_eq!(result, 0);
            self.initialized.store(false, Ordering::Release);
        }
    }
}

pub(crate) struct GlobalMutexGuard<'a, T> {
    mutex: &'a GlobalMutex<T>,
    thread_bound: PhantomData<std::sync::MutexGuard<'a, T>>,
}

impl<T> Deref for GlobalMutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: The guard holds the native mutex throughout the borrow.
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for GlobalMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: The mutex and mutable guard borrow guarantee exclusive access.
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for GlobalMutexGuard<'_, T> {
    fn drop(&mut self) {
        // SAFETY: This thread-bound guard unlocks on its acquiring thread.
        unsafe { libc::pthread_mutex_unlock(self.mutex.native.0.get()) };
    }
}

impl Drop for NativeOwner {
    fn drop(&mut self) {
        // SAFETY: Exclusive ownership excludes live borrowed guards/waiters.
        // A forgotten guard can still leave the native mutex locked. Like std,
        // leak only its native storage in that case rather than destroy it.
        if unsafe { libc::pthread_mutex_trylock(self.pointer()) } != 0 {
            return;
        }
        // SAFETY: trylock succeeded on this thread; unlock before destruction.
        unsafe {
            libc::pthread_mutex_unlock(self.pointer());
            libc::pthread_mutex_destroy(self.pointer());
            ManuallyDrop::drop(&mut self.0);
        }
    }
}

pub(crate) struct ObjectMutex<T> {
    native: NativeOwner,
    value: UnsafeCell<T>,
}

// SAFETY: Moving the owner never moves the boxed native mutex. Access to T is
// serialized, and transferring its ownership requires T: Send.
unsafe impl<T: Send> Send for ObjectMutex<T> {}
// SAFETY: All access to the UnsafeCell is protected by the native mutex.
unsafe impl<T: Send> Sync for ObjectMutex<T> {}

impl<T> MutexInit<T> for ObjectMutex<T> {
    fn try_new(value: T) -> Result<Self, VkResult> {
        Ok(Self {
            native: NativeOwner::try_new()?,
            value: UnsafeCell::new(value),
        })
    }
}

impl<T> ObjectMutex<T> {
    pub(crate) fn lock(&self) -> ObjectMutexGuard<'_, T> {
        // SAFETY: Construction initialized stable NORMAL mutex storage. The
        // guard borrows this owner until the matching unlock on this thread.
        let result = unsafe { libc::pthread_mutex_lock(self.native.pointer()) };
        if result != 0 {
            // No guard may expose the value after an unsuccessful lock. These
            // non-robust mutexes have no recoverable lock-time failure contract.
            std::process::abort();
        }
        ObjectMutexGuard {
            mutex: self,
            thread_bound: PhantomData,
        }
    }
}

impl<T> MutexAcquire<T> for GlobalMutex<T> {
    fn try_lock(&'static self) -> Result<impl DerefMut<Target = T>, VkResult> {
        // SAFETY: Initialization publishes stable native storage only on success.
        unsafe { self.try_lock_with(|native| initialize_native(native)) }
    }
}

pub(crate) struct ObjectMutexGuard<'a, T> {
    mutex: &'a ObjectMutex<T>,
    thread_bound: PhantomData<std::sync::MutexGuard<'a, T>>,
}

impl<T> Deref for ObjectMutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: This guard holds the lock for the returned borrow's lifetime.
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for ObjectMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: The lock and mutable guard borrow guarantee exclusive access.
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for ObjectMutexGuard<'_, T> {
    fn drop(&mut self) {
        // SAFETY: The !Send guard drops on the thread which acquired this lock.
        unsafe { libc::pthread_mutex_unlock(self.mutex.native.pointer()) };
    }
}

#[cfg(test)]
mod tests {
    use super::{GlobalMutex, MutexAcquire, MutexInit, ObjectMutex, check_initialization};
    use core::cell::Cell;

    struct Dropped<'a>(&'a Cell<usize>);
    impl Drop for Dropped<'_> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn native_initialization_errors_preserve_resource_failure_classification() {
        for (native, expected) in [
            (0, Ok(())),
            (libc::ENOMEM, Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)),
            (libc::EAGAIN, Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)),
            (libc::EINVAL, Err(vk::VkResult::ERROR_INITIALIZATION_FAILED)),
        ] {
            assert_eq!(check_initialization(native), expected);
        }
    }

    #[test]
    fn construction_is_fallible_and_releases_the_value() {
        let dropped = Cell::new(0);
        crate::allocation::fault::sweep_operation(|| {
            let before = dropped.get();
            let result = match ObjectMutex::try_new((0, Dropped(&dropped))) {
                Ok(mutex) => {
                    mutex.lock().0 = 17;
                    assert_eq!(mutex.lock().0, 17);
                    vk::VkResult::SUCCESS
                }
                Err(error) => error,
            };
            assert_eq!(dropped.get(), before + 1);
            result
        });
    }

    #[test]
    fn concurrent_locking_preserves_updates() {
        let mutex = ObjectMutex::try_new(0).unwrap();
        std::thread::scope(|scope| {
            for _ in 0..4 {
                scope.spawn(|| {
                    for _ in 0..1000 {
                        *mutex.lock() += 1;
                    }
                });
            }
        });
        assert_eq!(*mutex.lock(), 4000);
    }

    #[test]
    fn global_mutex_failure_is_unpublished_and_retryable() {
        static MUTEX: GlobalMutex<usize> = GlobalMutex::new(7);
        assert!(MUTEX.lock_if_initialized().is_none());
        // SAFETY: A failed initializer never publishes native storage, so the
        // injected error requires no native initialization or destruction.
        let failed =
            unsafe { MUTEX.try_lock_with(|_| Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)) };
        assert!(matches!(
            failed,
            Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)
        ));
        assert!(MUTEX.lock_if_initialized().is_none());
        assert_eq!(*MUTEX.try_lock().unwrap(), 7);
        // SAFETY: The preceding successful lock initialized this static mutex;
        // its temporary guard was dropped and destruction has not run.
        assert_eq!(*unsafe { MUTEX.lock_initialized_unchecked() }, 7);
        // SAFETY: No guards remain and this test never acquires MUTEX again.
        unsafe { MUTEX.destroy() };
    }

    #[test]
    fn global_mutex_concurrent_first_use_preserves_updates() {
        static MUTEX: GlobalMutex<usize> = GlobalMutex::new(0);
        let start = std::sync::Barrier::new(4);
        std::thread::scope(|scope| {
            for _ in 0..4 {
                scope.spawn(|| {
                    start.wait();
                    for _ in 0..1000 {
                        *MUTEX.try_lock().unwrap() += 1;
                    }
                });
            }
        });
        assert_eq!(*MUTEX.lock_if_initialized().unwrap(), 4000);
        // SAFETY: All threads joined and the final read guard was dropped.
        unsafe { MUTEX.destroy() };
    }
}
