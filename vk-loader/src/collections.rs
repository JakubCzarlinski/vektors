//! Fast hash collections whose seed state is safe across loader unloads.
//!
//! `std::collections::hash_map::RandomState` uses a TLS cache on Windows, while
//! `ahash::RandomState::default` retains two heap allocations in process-global
//! `OnceBox` values. A Vulkan loader can be loaded and unloaded repeatedly, so
//! neither lifetime is suitable here. Keep randomized `AHash` performance while
//! storing the base seeds inline in this image instead.

use alloc::vec::Vec;
use core::{hash::BuildHasher, mem::MaybeUninit, sync::atomic::AtomicUsize};
use std::{
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

static SEQUENCE: AtomicUsize = AtomicUsize::new(0);
static BASE_SEEDS: LazyLock<[u64; 4]> = LazyLock::new(|| {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let image_address = core::ptr::addr_of!(BASE_SEEDS) as usize as u64;
    let process = u64::from(std::process::id());
    [
        time as u64,
        (time >> 64) as u64 ^ image_address.rotate_left(17),
        process ^ image_address.rotate_right(13),
        crate::platform::current_thread_key() as u64 ^ time as u64,
    ]
});

#[derive(Clone)]
pub(crate) struct RandomState(ahash::RandomState);

impl Default for RandomState {
    #[inline]
    fn default() -> Self {
        let [k0, k1, k2, k3] = *BASE_SEEDS;
        let sequence = SEQUENCE.fetch_add(1, core::sync::atomic::Ordering::Relaxed) as u64;
        Self(ahash::RandomState::with_seeds(
            k0,
            k1 ^ sequence,
            k2,
            k3 ^ sequence.rotate_left(29),
        ))
    }
}

impl BuildHasher for RandomState {
    type Hasher = <ahash::RandomState as BuildHasher>::Hasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        self.0.build_hasher()
    }
}

pub(crate) type HashMap<K, V> = std::collections::HashMap<K, V, RandomState>;
pub(crate) type HashSet<T> = std::collections::HashSet<T, RandomState>;

/// Call-scoped uninitialized storage with a bounded stack fast path.
pub(crate) struct ScratchArray<T, const STACK_CAPACITY: usize> {
    stack: [MaybeUninit<T>; STACK_CAPACITY],
    heap: Vec<MaybeUninit<T>>,
    len: usize,
}

impl<T, const STACK_CAPACITY: usize> ScratchArray<T, STACK_CAPACITY> {
    pub(crate) fn try_new(len: usize) -> Result<Self, ()> {
        let mut heap = Vec::new();
        if len > STACK_CAPACITY {
            heap.try_reserve_exact(len).map_err(|_| ())?;
            // SAFETY: `MaybeUninit<T>` does not require initialization.
            unsafe { heap.set_len(len) };
        }
        Ok(Self {
            // SAFETY: An array of `MaybeUninit<T>` may be left uninitialized.
            stack: unsafe { MaybeUninit::uninit().assume_init() },
            heap,
            len,
        })
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
        match self.heap.as_mut_slice() {
            [] => self.stack.as_mut_ptr().cast(),
            heap => heap.as_mut_ptr().cast(),
        }
    }

    /// Returns the initialized prefix written by an external call.
    ///
    /// # Safety
    ///
    /// The first `initialized` elements must have been initialized as `T`.
    pub(crate) unsafe fn initialized(&self, initialized: usize) -> &[T] {
        debug_assert!(initialized <= self.len);
        // SAFETY: The caller guarantees the prefix is initialized, and both
        // backing stores have at least `self.len` elements.
        unsafe { core::slice::from_raw_parts(self.as_ptr(), initialized) }
    }

    #[inline]
    const fn as_ptr(&self) -> *const T {
        match self.heap.as_slice() {
            [] => self.stack.as_ptr().cast(),
            heap => heap.as_ptr().cast(),
        }
    }
}
