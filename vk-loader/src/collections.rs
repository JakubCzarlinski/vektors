//! Fast hash collections whose seed state is safe across loader unloads.
//!
//! `std::collections::hash_map::RandomState` uses a TLS cache on Windows, while
//! `ahash::RandomState::default` retains two heap allocations in process-global
//! `OnceBox` values. A Vulkan loader can be loaded and unloaded repeatedly, so
//! neither lifetime is suitable here. Keep randomized `AHash` performance while
//! storing the base seeds inline in this image instead.

mod seeds;

use alloc::vec::Vec;
use core::{
    ffi::c_void, hash::BuildHasher, mem::MaybeUninit, ptr::NonNull, sync::atomic::AtomicUsize,
};

pub(crate) use std::collections::hash_map::Entry as HashMapEntry;

static SEQUENCE: AtomicUsize = AtomicUsize::new(0);

pub(crate) struct RandomState(ahash::RandomState);

impl Default for RandomState {
    #[inline]
    fn default() -> Self {
        let [k0, k1, k2, k3] = seeds::base_seeds();
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

/// Provenance-preserving common representation for thin owned/alias pointers.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct ErasedPointer(NonNull<c_void>);

impl ErasedPointer {
    /// # Safety
    ///
    /// The caller must ensure moving the erased owner between registry threads
    /// is valid for `T`.
    pub(crate) unsafe fn from_box<T>(value: Box<T>) -> Self {
        Self(NonNull::from(Box::leak(value)).cast())
    }

    pub(crate) const fn as_ptr<T>(self) -> *mut T {
        self.0.cast::<T>().as_ptr()
    }

    /// # Safety
    ///
    /// This pointer must be the unique owner produced by `from_box` for `T`.
    pub(crate) unsafe fn into_box<T>(self) -> Box<T> {
        unsafe { Box::from_raw(self.as_ptr()) }
    }
}

// Construction requires a Send pointee; aliases are only used under registry
// synchronization and the pointee's external synchronization rules.
unsafe impl Send for ErasedPointer {}
unsafe impl Sync for ErasedPointer {}

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
            stack: [const { MaybeUninit::uninit() }; STACK_CAPACITY],
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
