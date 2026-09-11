//! Loader-owned allocations honoring Vulkan allocation callbacks.

use alloc::{
    alloc::{Layout, alloc, dealloc, realloc},
    boxed::Box,
    ffi::CString,
    vec::Vec,
};
use core::{
    ffi::{CStr, c_void},
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};
use vk::{VkAllocationCallbacks, VkResult, VkSystemAllocationScope};

pub(crate) const LOADER_ALIGNMENT: usize = core::mem::size_of::<u64>();

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CStringError {
    InteriorNul,
    OutOfMemory,
}

pub(crate) fn try_c_string_bytes(bytes: &[u8]) -> Result<CString, CStringError> {
    if bytes.contains(&0) {
        return Err(CStringError::InteriorNul);
    }
    // SAFETY: Interior NULs were rejected above.
    unsafe { try_c_string_bytes_unchecked(bytes) }.map_err(|_| CStringError::OutOfMemory)
}

/// Copies bytes whose lack of interior NULs is already established.
///
/// # Safety
///
/// `bytes` must not contain a NUL byte.
pub(crate) unsafe fn try_c_string_bytes_unchecked(bytes: &[u8]) -> Result<CString, VkResult> {
    let length = bytes
        .len()
        .checked_add(1)
        .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut storage = try_boxed_slice_filled(length, 0_u8)?;
    storage[..bytes.len()].copy_from_slice(bytes);
    // SAFETY: The caller guarantees no interior NULs; the final byte remains NUL.
    // The byte slice and CStr have identical allocation layouts.
    let string = unsafe { Box::from_raw(Box::into_raw(storage) as *mut CStr) };
    Ok(CString::from(string))
}

/// Finalizes storage with a fallible shrinking realloc rather than a second
/// allocation and element copy. A failed shrink leaves the original Vec live.
pub(crate) fn try_into_boxed_slice<T>(values: Vec<T>) -> Result<Box<[T]>, VkResult> {
    let len = values.len();
    if len == values.capacity() || core::mem::size_of::<T>() == 0 {
        return Ok(values.into_boxed_slice());
    }
    if len == 0 {
        return Ok(Box::default());
    }
    let layout =
        Layout::array::<T>(values.capacity()).map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    // Disable destruction before realloc can invalidate the Vec's allocation.
    let mut values = ManuallyDrop::new(values);
    // SAFETY: A non-ZST Vec owns a global-allocator block with this layout.
    // len is nonzero and below capacity, so its byte size cannot overflow.
    // realloc preserves the initialized prefix and retains ownership on failure.
    let pointer = unsafe {
        realloc(
            values.as_mut_ptr().cast(),
            layout,
            len * core::mem::size_of::<T>(),
        )
    }
    .cast::<T>();
    let Some(pointer) = NonNull::new(pointer) else {
        // SAFETY: A failed realloc leaves the original allocation unchanged.
        // This is the only path that drops the original Vec.
        unsafe { ManuallyDrop::drop(&mut values) };
        return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
    };
    // SAFETY: The successful realloc owns exactly len initialized elements,
    // with T's original alignment. The old Vec is never used again.
    Ok(unsafe { Box::from_raw(core::ptr::slice_from_raw_parts_mut(pointer.as_ptr(), len)) })
}

pub(crate) fn try_collect<T>(values: impl IntoIterator<Item = T>) -> Result<Box<[T]>, VkResult> {
    try_collect_results(values.into_iter().map(Ok))
}

pub(crate) fn try_collect_results<T>(
    values: impl IntoIterator<Item = Result<T, VkResult>>,
) -> Result<Box<[T]>, VkResult> {
    let values = values.into_iter();
    let mut output = Vec::new();
    output
        .try_reserve_exact(values.size_hint().0)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for value in values {
        let value = value?;
        output
            .try_reserve(1)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        output.push(value);
    }
    try_into_boxed_slice(output)
}

pub(crate) fn try_os_string(value: &OsStr) -> Result<OsString, VkResult> {
    let mut owned = OsString::new();
    owned
        .try_reserve_exact(value.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    owned.push(value);
    Ok(owned)
}

pub(crate) fn try_path(value: &Path) -> Result<PathBuf, VkResult> {
    try_os_string(value.as_os_str()).map(PathBuf::from)
}

pub(crate) fn try_box_str(value: &str) -> Result<Box<str>, VkResult> {
    let mut storage = try_box_uninit_slice::<u8>(value.len())?;
    for (slot, byte) in storage.iter_mut().zip(value.bytes()) {
        slot.write(byte);
    }
    // SAFETY: All bytes were initialized from valid UTF-8. str and [u8]
    // have identical allocation layouts and lengths.
    let bytes = unsafe { storage.assume_init() };
    Ok(unsafe { Box::from_raw(Box::into_raw(bytes) as *mut str) })
}

pub(crate) fn try_push<T>(values: &mut Vec<T>, value: T) -> Result<(), VkResult> {
    values
        .try_reserve(1)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    values.push(value);
    Ok(())
}

/// Copies a C string into exact-sized storage without a shrinking allocation.
pub(crate) fn try_c_string(value: &CStr) -> Result<CString, VkResult> {
    let bytes = value.to_bytes_with_nul();
    let mut storage = try_box_uninit_slice::<u8>(bytes.len())?;
    for (output, byte) in storage.iter_mut().zip(bytes) {
        output.write(*byte);
    }
    // SAFETY: Every byte was initialized from the original C string.
    let storage = unsafe { storage.assume_init() };
    // SAFETY: CStr is a transparent byte slice with a terminating NUL. This
    // exact copy preserves its invariant, allocation layout and slice length.
    let string = unsafe { Box::from_raw(Box::into_raw(storage) as *mut CStr) };
    Ok(CString::from(string))
}

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
        let slice = core::ptr::slice_from_raw_parts_mut(
            NonNull::<MaybeUninit<T>>::dangling().as_ptr(),
            len,
        );
        // SAFETY: A zero-sized allocation requires only an aligned non-null
        // pointer. Preserve `len` for non-empty slices of zero-sized elements.
        return Ok(unsafe { Box::from_raw(slice) });
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

union LoaderAllocationData {
    global_size: usize,
    callback_user_data: *mut c_void,
}

pub(crate) struct LoaderAllocation {
    pointer: NonNull<u8>,
    free: vk::PFN_vkFreeFunction,
    data: LoaderAllocationData,
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
        let free = callbacks.and_then(|callbacks| callbacks.pfnFree);
        let data = if free.is_some() {
            LoaderAllocationData {
                callback_user_data: callbacks
                    .map_or(core::ptr::null_mut(), |callbacks| callbacks.pUserData),
            }
        } else {
            LoaderAllocationData { global_size: size }
        };
        Ok(Self {
            pointer,
            free,
            data,
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
    /// Allocates stable storage without first constructing `T` on the stack.
    pub(crate) fn try_new_uninit(
        callbacks: Option<&VkAllocationCallbacks<'static>>,
        scope: VkSystemAllocationScope,
    ) -> Result<LoaderBox<MaybeUninit<T>>, VkResult> {
        const {
            assert!(core::mem::size_of::<T>() != 0);
            assert!(core::mem::align_of::<T>() <= LOADER_ALIGNMENT);
        }
        Ok(LoaderBox {
            allocation: LoaderAllocation::new(callbacks, core::mem::size_of::<T>(), scope)?,
            marker: PhantomData,
        })
    }

    pub(crate) const fn as_ptr(&self) -> *const T {
        self.allocation.as_ptr().cast()
    }
}

impl<T> LoaderBox<MaybeUninit<T>> {
    pub(crate) const fn as_mut_ptr(&mut self) -> *mut T {
        self.allocation.as_ptr().cast()
    }

    /// Converts this allocation after every field of `T` has been initialized.
    ///
    /// # Safety
    ///
    /// The allocation must contain a valid, fully initialized `T`.
    pub(crate) unsafe fn assume_init(self) -> LoaderBox<T> {
        let this = ManuallyDrop::new(self);
        // SAFETY: `ManuallyDrop` keeps the allocation owned while it is moved
        // into the identically represented initialized wrapper.
        let allocation = unsafe { core::ptr::read(&raw const this.allocation) };
        LoaderBox {
            allocation,
            marker: PhantomData,
        }
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
        if let Some(free) = self.free {
            // SAFETY: This pointer was returned under the matching retained
            // callback set, so the union contains its user-data pointer.
            unsafe {
                free(self.data.callback_user_data, self.pointer.as_ptr().cast());
            }
        } else {
            // SAFETY: A missing callback free function identifies a global
            // allocation, whose union member is its validated nonzero size.
            let layout = unsafe {
                Layout::from_size_align_unchecked(self.data.global_size, LOADER_ALIGNMENT)
            };
            // SAFETY: The fallback allocator created this pointer with the
            // exact retained layout.
            unsafe { dealloc(self.pointer.as_ptr(), layout) };
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

    #[test]
    fn collection_does_not_allocate_for_an_error_item() {
        crate::allocation::fault::sweep_operation(|| {
            let mut item = Some(Err::<u64, _>(VkResult::ERROR_INITIALIZATION_FAILED));
            // An unknown size hint exercises growth at the first yielded item.
            let values = core::iter::from_fn(|| item.take());
            assert_eq!(
                try_collect_results(values),
                Err(VkResult::ERROR_INITIALIZATION_FAILED)
            );
            VkResult::SUCCESS
        });
    }

    struct Counts {
        allocations: AtomicUsize,
        frees: AtomicUsize,
        alignment: AtomicUsize,
    }

    #[test]
    fn shrinking_preserves_values_and_drops_them_once_on_success_or_oom() {
        struct CountDrop<'a>(&'a AtomicUsize);
        impl Drop for CountDrop<'_> {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }
        crate::allocation::fault::sweep_operation(|| {
            let drops = AtomicUsize::new(0);
            let mut values = Vec::new();
            if values.try_reserve_exact(7).is_err() {
                return VkResult::ERROR_OUT_OF_HOST_MEMORY;
            }
            for index in 0..3 {
                values.push((index, CountDrop(&drops)));
            }
            let result = match try_into_boxed_slice(values) {
                Ok(values) => {
                    assert_eq!(values.len(), 3);
                    for (index, (value, _)) in values.iter().enumerate() {
                        assert_eq!(*value, index);
                    }
                    drop(values);
                    VkResult::SUCCESS
                }
                Err(error) => error,
            };
            assert_eq!(drops.load(Ordering::Relaxed), 3);
            result
        });
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
        let mut value =
            LoaderBox::<DropCounter<'_>>::try_new_uninit(None, VkSystemAllocationScope::OBJECT)
                .unwrap();
        // SAFETY: The allocation is valid, aligned, and uniquely owned.
        unsafe { value.as_mut_ptr().write(DropCounter(&drops)) };
        // SAFETY: The allocation was initialized immediately above.
        let value = unsafe { value.assume_init() };
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

    #[test]
    fn fallible_slice_preserves_zero_sized_element_count() {
        assert_eq!(try_boxed_slice_filled(7, ()).unwrap().len(), 7);
    }
}
