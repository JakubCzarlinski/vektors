//! Shared adapters for loader-side emulation of newer Vulkan queries.

use core::ffi::c_void;

use crate::collections::ScratchArray;

unsafe fn emulate_array<T: Copy, U, R, const STACK_CAPACITY: usize>(
    count: &mut u32,
    output: Option<&mut [U]>,
    call: impl FnOnce(&mut u32, *mut T) -> R,
    should_copy: impl FnOnce(&R) -> bool,
    mut write: impl FnMut(&mut U, T),
) -> Result<R, ()> {
    let Some(output) = output else {
        return Ok(call(count, core::ptr::null_mut()));
    };
    let capacity = output.len();
    if capacity == 0 {
        return Ok(call(count, core::ptr::null_mut()));
    }
    let mut temporary = ScratchArray::<T, STACK_CAPACITY>::try_new(capacity)?;
    let result = call(count, temporary.as_mut_ptr());
    if should_copy(&result) {
        let written = (*count as usize).min(capacity);
        // SAFETY: The callback reported an initialized prefix on success.
        for (output, &value) in output
            .iter_mut()
            .zip(unsafe { temporary.initialized(written) })
        {
            write(output, value);
        }
    }
    Ok(result)
}

/// Adapts a legacy array query to an array of promoted wrapper structures.
///
/// # Safety
///
/// `call` must initialize the prefix it reports through `count` whenever it
/// returns a non-error result.
pub(crate) unsafe fn emulate_result_array<T: Copy, U, const STACK_CAPACITY: usize>(
    count: &mut u32,
    output: Option<&mut [U]>,
    call: impl FnOnce(&mut u32, *mut T) -> vk::VkResult,
    write: impl FnMut(&mut U, T),
) -> Result<vk::VkResult, ()> {
    // SAFETY: The caller supplies the initialization contract documented above.
    unsafe {
        emulate_array::<T, U, _, STACK_CAPACITY>(count, output, call, |result| result.0 >= 0, write)
    }
}

/// Adapts a legacy void array query to promoted wrapper structures.
///
/// # Safety
///
/// `call` must initialize the prefix it reports through `count`.
pub(crate) unsafe fn emulate_void_array<T: Copy, U, const STACK_CAPACITY: usize>(
    count: &mut u32,
    output: Option<&mut [U]>,
    call: impl FnOnce(&mut u32, *mut T),
    write: impl FnMut(&mut U, T),
) -> Result<(), ()> {
    // SAFETY: The caller supplies the initialization contract documented above.
    unsafe { emulate_array::<T, U, _, STACK_CAPACITY>(count, output, call, |()| true, write) }
}

/// Converts an optional Vulkan output array to its call-scoped slice.
///
/// # Safety
///
/// A non-null `output` must point to `count` writable elements.
pub(crate) unsafe fn optional_output_slice<'a, T>(
    output: *mut T,
    count: u32,
) -> Option<&'a mut [T]> {
    (!output.is_null()).then(|| {
        // SAFETY: The caller guarantees the Vulkan output-array contract.
        unsafe { core::slice::from_raw_parts_mut(output, count as usize) }
    })
}

/// Visits every structure in a writable Vulkan output chain.
///
/// # Safety
///
/// `next` must be null or begin a live, writable, acyclic Vulkan output chain.
pub(crate) unsafe fn for_each_output_chain(
    mut next: *mut c_void,
    mut visit: impl FnMut(&mut vk::VkBaseOutStructure<'_>),
) {
    while !next.is_null() {
        let header = next.cast::<vk::VkBaseOutStructure<'_>>();
        // SAFETY: Every live output-chain node begins with this header.
        let header = unsafe { &mut *header };
        next = header.pNext.cast();
        visit(header);
    }
}

/// Finds the first structure of a requested type in a Vulkan input chain.
///
/// # Safety
///
/// `next` must be null or begin a live, readable, acyclic Vulkan input chain.
pub(crate) unsafe fn find_input_chain<'a>(
    mut next: *const c_void,
    structure_type: vk::VkStructureType,
) -> Option<&'a vk::VkBaseInStructure<'a>> {
    while !next.is_null() {
        // SAFETY: Every live input-chain node begins with this header.
        let header = unsafe { &*next.cast::<vk::VkBaseInStructure<'a>>() };
        if header.sType == structure_type {
            return Some(header);
        }
        next = header.pNext.cast();
    }
    None
}

/// Visits every structure in a readable Vulkan input chain.
///
/// # Safety
///
/// `next` must be null or begin a live, readable, acyclic Vulkan input chain.
pub(crate) unsafe fn for_each_input_chain(
    mut next: *const c_void,
    mut visit: impl FnMut(&vk::VkBaseInStructure<'_>),
) {
    while !next.is_null() {
        // SAFETY: Every live input-chain node begins with this header.
        let header = unsafe { &*next.cast::<vk::VkBaseInStructure<'_>>() };
        next = header.pNext.cast();
        visit(header);
    }
}
