//! Synchronization primitives whose storage remains owned by the loader.
//!
//! `parking_lot` keeps process-global and thread-local heap state after lock
//! contention.  A Vulkan loader may be unloaded with `dlclose`, so that state
//! would outlive the code which owns its destructors. The standard mutex avoids
//! that parking-lot lifetime dependency, but its pthread backend still allocates
//! stable native storage on first use (Apple, NetBSD, QNX, Hurd, and Cygwin).
//! `ObjectMutex` initializes that storage fallibly during object construction.
//! Global mutexes instead initialize native storage at its final static address.

mod global;
mod global_lazy;
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
mod inline;
#[cfg(all(
    unix,
    any(
        test,
        not(any(
            target_os = "linux",
            target_os = "android",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "dragonfly",
            target_os = "fuchsia"
        ))
    )
))]
mod pthread;

pub(crate) use global::GlobalMutex;
pub(crate) use global_lazy::GlobalLazyMutex;

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
pub(crate) use inline::Mutex as ObjectMutex;
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
pub(crate) use pthread::ObjectMutex;

/// Constructs object-owned synchronization before publishing the object.
pub(crate) trait MutexInit<T>: Sized {
    fn try_new(value: T) -> Result<Self, vk::VkResult>;
}

/// Initializes global synchronization fallibly before acquiring it.
pub(crate) trait MutexAcquire<T> {
    fn try_lock(&'static self) -> Result<impl core::ops::DerefMut<Target = T>, vk::VkResult>;
}
