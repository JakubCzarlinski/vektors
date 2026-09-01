//! Loader-owned allocations honoring Vulkan allocation callbacks.

use alloc::{
    alloc::{Layout, alloc, dealloc},
    boxed::Box,
};
use core::{
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use vk::{VkAllocationCallbacks, VkResult, VkSystemAllocationScope};

pub(crate) const LOADER_ALIGNMENT: usize = core::mem::size_of::<u64>();

/// Allocates one value through Rust's global allocator without invoking the
/// process-wide allocation-error handler.
pub(crate) fn try_box<T>(value: T) -> Result<Box<T>, (VkResult, T)> {
    if core::mem::size_of::<T>() == 0 {
        return Ok(Box::new(value));
    }
    let layout = Layout::new::<T>();
    // SAFETY: `layout` is non-zero and valid for `T`.
    let pointer = unsafe { alloc(layout) }.cast::<T>();
    let Some(pointer) = NonNull::new(pointer) else {
        return Err((VkResult::ERROR_OUT_OF_HOST_MEMORY, value));
    };
    // SAFETY: The allocation has the exact layout of `T` and is uniquely owned.
    unsafe { pointer.as_ptr().write(value) };
    // SAFETY: The initialized allocation was made with the global allocator
    // and is transferred directly into `Box` ownership.
    Ok(unsafe { Box::from_raw(pointer.as_ptr()) })
}

/// Allocates uninitialized stable storage without aborting on exhaustion.
pub(crate) fn try_box_uninit<T>() -> Result<Box<MaybeUninit<T>>, VkResult> {
    if core::mem::size_of::<T>() == 0 {
        return Ok(Box::new(MaybeUninit::uninit()));
    }
    let layout = Layout::new::<T>();
    // SAFETY: `layout` is non-zero and valid for `T`.
    let pointer = unsafe { alloc(layout) }.cast::<MaybeUninit<T>>();
    let pointer = NonNull::new(pointer).ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    // SAFETY: `MaybeUninit<T>` has `T`'s layout and permits uninitialized bytes.
    Ok(unsafe { Box::from_raw(pointer.as_ptr()) })
}

/// Allocates an uninitialized boxed slice without aborting on exhaustion.
pub(crate) fn try_box_uninit_slice<T>(len: usize) -> Result<Box<[MaybeUninit<T>]>, VkResult> {
    let layout = Layout::array::<T>(len).map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    if layout.size() == 0 {
        return Ok(Box::new([]));
    }
    // SAFETY: `layout` is non-zero and valid for an array of `len` values.
    let pointer = unsafe { alloc(layout) }.cast::<MaybeUninit<T>>();
    let pointer = NonNull::new(pointer).ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let slice = core::ptr::slice_from_raw_parts_mut(pointer.as_ptr(), len);
    // SAFETY: The allocation describes exactly `len` uninitialized entries and
    // is transferred directly into boxed-slice ownership.
    Ok(unsafe { Box::from_raw(slice) })
}

pub(crate) fn try_boxed_slice_filled<T: Copy>(len: usize, value: T) -> Result<Box<[T]>, VkResult> {
    let mut storage = try_box_uninit_slice::<T>(len)?;
    for entry in &mut storage {
        entry.write(value);
    }
    // SAFETY: Every element was initialized exactly once above.
    Ok(unsafe { storage.assume_init() })
}

pub(crate) struct LoaderAllocation {
    pointer: NonNull<u8>,
    layout: Layout,
    callbacks: Option<VkAllocationCallbacks<'static>>,
}

impl LoaderAllocation {
    pub(crate) fn new(
        callbacks: Option<&VkAllocationCallbacks<'static>>,
        size: usize,
        scope: VkSystemAllocationScope,
    ) -> Result<Self, VkResult> {
        let layout = Layout::from_size_align(size, LOADER_ALIGNMENT)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        let pointer = if let Some(callbacks) = callbacks
            && let Some(allocate_callback) = callbacks.pfnAllocation
        {
            // SAFETY: The callbacks are retained for the allocation lifetime
            // and Vulkan defines this callback's allocation contract.
            NonNull::new(
                unsafe { allocate_callback(callbacks.pUserData, size, LOADER_ALIGNMENT, scope) }
                    .cast(),
            )
        } else {
            // SAFETY: `layout` has non-zero Vulkan-structure size and valid
            // power-of-two alignment.
            NonNull::new(unsafe { alloc(layout) })
        }
        .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        Ok(Self {
            pointer,
            layout,
            callbacks: callbacks.copied(),
        })
    }

    pub(crate) const fn as_ptr(&self) -> *mut u8 {
        self.pointer.as_ptr()
    }
}

/// A stable loader-owned object allocated through Vulkan's internal allocator.
pub(crate) struct LoaderBox<T> {
    allocation: LoaderAllocation,
    marker: PhantomData<T>,
}

impl<T> LoaderBox<T> {
    pub(crate) fn new(
        callbacks: Option<&VkAllocationCallbacks<'static>>,
        value: T,
        scope: VkSystemAllocationScope,
    ) -> Result<Self, VkResult> {
        Self::try_new(callbacks, value, scope).map_err(|(result, _value)| result)
    }

    /// Allocates stable storage while returning ownership of `value` when the
    /// allocation fails, so callers can roll back resources held by it.
    pub(crate) fn try_new(
        callbacks: Option<&VkAllocationCallbacks<'static>>,
        value: T,
        scope: VkSystemAllocationScope,
    ) -> Result<Self, (VkResult, T)> {
        const {
            assert!(core::mem::size_of::<T>() != 0);
            assert!(core::mem::align_of::<T>() <= LOADER_ALIGNMENT);
        }
        let allocation = match LoaderAllocation::new(callbacks, core::mem::size_of::<T>(), scope) {
            Ok(allocation) => allocation,
            Err(result) => return Err((result, value)),
        };
        // SAFETY: The allocation has the size and alignment required for `T`
        // and is uniquely owned until this wrapper is dropped.
        unsafe { allocation.as_ptr().cast::<T>().write(value) };
        Ok(Self {
            allocation,
            marker: PhantomData,
        })
    }

    pub(crate) const fn as_ptr(&self) -> *const T {
        self.allocation.as_ptr().cast()
    }
}

impl<T> Deref for LoaderBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: Construction initializes exactly one `T`, retained for the
        // complete lifetime of this allocation.
        unsafe { &*self.as_ptr() }
    }
}

impl<T> DerefMut for LoaderBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The unique wrapper provides exclusive access to its value.
        unsafe { &mut *self.allocation.as_ptr().cast() }
    }
}

impl<T> Drop for LoaderBox<T> {
    fn drop(&mut self) {
        // SAFETY: The value was initialized once and is dropped once before
        // `LoaderAllocation` releases its backing storage.
        unsafe { core::ptr::drop_in_place(self.allocation.as_ptr().cast::<T>()) };
    }
}

/// Fixed-length loader storage for copyable ABI values.
pub(crate) struct LoaderArray<T: Copy> {
    allocation: Option<LoaderAllocation>,
    len: usize,
    marker: PhantomData<T>,
}

impl<T: Copy> LoaderArray<T> {
    pub(crate) fn filled(
        callbacks: Option<&VkAllocationCallbacks<'static>>,
        len: usize,
        value: T,
        scope: VkSystemAllocationScope,
    ) -> Result<Self, VkResult> {
        const {
            assert!(core::mem::size_of::<T>() != 0);
            assert!(core::mem::align_of::<T>() <= LOADER_ALIGNMENT);
        }
        let Some(size) = core::mem::size_of::<T>().checked_mul(len) else {
            return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
        };
        let allocation = if size == 0 {
            None
        } else {
            let allocation = LoaderAllocation::new(callbacks, size, scope)?;
            let pointer = allocation.as_ptr().cast::<T>();
            for index in 0..len {
                // SAFETY: The checked total allocation contains `len`
                // correctly aligned entries and each is initialized once.
                unsafe { pointer.add(index).write(value) };
            }
            Some(allocation)
        };
        Ok(Self {
            allocation,
            len,
            marker: PhantomData,
        })
    }

    fn as_ptr(&self) -> *const T {
        self.allocation.as_ref().map_or_else(
            || NonNull::<T>::dangling().as_ptr(),
            |allocation| allocation.as_ptr().cast(),
        )
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        self.allocation.as_mut().map_or_else(
            || NonNull::<T>::dangling().as_ptr(),
            |allocation| allocation.as_ptr().cast(),
        )
    }
}

impl<T: Copy> Deref for LoaderArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        // SAFETY: The allocation contains exactly `len` initialized values;
        // the dangling pointer is valid for the empty-slice case.
        unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len) }
    }
}

impl<T: Copy> DerefMut for LoaderArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Exclusive access to the wrapper makes the full slice unique.
        unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
}

// Vulkan permits allocator callbacks to carry application-owned user data.
// Their synchronization and cross-thread validity are application contracts.
unsafe impl Send for LoaderAllocation {}
unsafe impl Sync for LoaderAllocation {}

impl Drop for LoaderAllocation {
    fn drop(&mut self) {
        if let Some(callbacks) = self.callbacks.as_ref()
            && let Some(free_callback) = callbacks.pfnFree
        {
            // SAFETY: This pointer was returned under the matching retained
            // callback set and is released exactly once.
            unsafe { free_callback(callbacks.pUserData, self.pointer.as_ptr().cast()) };
        } else {
            // SAFETY: The fallback allocator created this pointer with the
            // exact retained layout.
            unsafe { dealloc(self.pointer.as_ptr(), self.layout) };
        }
    }
}

#[cfg(test)]
mod tests {
    use core::{
        ffi::c_void,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use super::*;

    struct Counts {
        allocations: AtomicUsize,
        frees: AtomicUsize,
        alignment: AtomicUsize,
    }

    unsafe extern "system" fn allocate(
        user_data: *mut c_void,
        size: usize,
        alignment: usize,
        _scope: VkSystemAllocationScope,
    ) -> *mut c_void {
        let counts = unsafe { &*user_data.cast::<Counts>() };
        counts.allocations.fetch_add(1, Ordering::Relaxed);
        counts.alignment.store(alignment, Ordering::Relaxed);
        unsafe { libc::malloc(size) }
    }

    unsafe extern "system" fn free(user_data: *mut c_void, memory: *mut c_void) {
        let counts = unsafe { &*user_data.cast::<Counts>() };
        counts.frees.fetch_add(1, Ordering::Relaxed);
        unsafe { libc::free(memory) };
    }

    #[test]
    fn uses_and_retains_the_matching_vulkan_callbacks() {
        let counts = Counts {
            allocations: AtomicUsize::new(0),
            frees: AtomicUsize::new(0),
            alignment: AtomicUsize::new(0),
        };
        let callbacks = VkAllocationCallbacks {
            pUserData: core::ptr::from_ref(&counts).cast_mut().cast(),
            pfnAllocation: Some(allocate),
            pfnFree: Some(free),
            ..VkAllocationCallbacks::DEFAULT
        };
        let allocation =
            LoaderAllocation::new(Some(&callbacks), 37, VkSystemAllocationScope::OBJECT).unwrap();
        assert_eq!(counts.allocations.load(Ordering::Relaxed), 1);
        assert_eq!(counts.alignment.load(Ordering::Relaxed), 8);
        drop(allocation);
        assert_eq!(counts.frees.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn loader_box_drops_value_before_freeing_storage() {
        struct DropCounter<'a>(&'a AtomicUsize);
        impl Drop for DropCounter<'_> {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }

        let drops = AtomicUsize::new(0);
        let value =
            LoaderBox::new(None, DropCounter(&drops), VkSystemAllocationScope::OBJECT).unwrap();
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        drop(value);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn loader_array_handles_empty_and_filled_storage() {
        let empty = LoaderArray::filled(None, 0, 7_u64, VkSystemAllocationScope::INSTANCE).unwrap();
        assert!(empty.is_empty());
        let mut filled =
            LoaderArray::filled(None, 3, 7_u64, VkSystemAllocationScope::INSTANCE).unwrap();
        assert_eq!(&*filled, &[7, 7, 7]);
        filled[1] = 9;
        assert_eq!(&*filled, &[7, 9, 7]);
    }

    #[test]
    fn fallible_slice_allocation_rejects_layout_overflow() {
        assert!(matches!(
            try_box_uninit_slice::<u64>(usize::MAX),
            Err(VkResult::ERROR_OUT_OF_HOST_MEMORY)
        ));
    }
}
