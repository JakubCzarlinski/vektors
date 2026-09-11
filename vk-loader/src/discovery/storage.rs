//! Fallible discovery storage with pending-error propagation.

use crate::{allocation, pending};
use alloc::{ffi::CString, vec::Vec};
use core::mem::MaybeUninit;
use std::path::{Path, PathBuf};

pub(super) fn owned_c_string(bytes: &[u8]) -> Option<CString> {
    match allocation::try_c_string_bytes(bytes) {
        Ok(value) => Some(value),
        Err(allocation::CStringError::InteriorNul) => None,
        Err(allocation::CStringError::OutOfMemory) => {
            pending::mark_json_allocation_failed();
            None
        }
    }
}

pub(super) fn owned_path(path: &Path) -> Option<PathBuf> {
    let Ok(owned) = allocation::try_path(path) else {
        pending::mark_json_allocation_failed();
        return None;
    };
    Some(owned)
}

pub(super) fn owned_box_str(value: &str) -> Option<Box<str>> {
    if let Ok(value) = allocation::try_box_str(value) {
        Some(value)
    } else {
        pending::mark_json_allocation_failed();
        None
    }
}

#[inline(never)]
pub(super) fn box_array<T, const N: usize>(values: [T; N]) -> Box<[T]> {
    if let Ok(values) = allocation::try_box(values) {
        values
    } else {
        pending::mark_json_allocation_failed();
        Box::default()
    }
}

#[inline(never)]
pub(super) fn collect_values<T>(values: impl IntoIterator<Item = T>) -> Option<Box<[T]>> {
    if let Ok(values) = allocation::try_collect(values) {
        Some(values)
    } else {
        pending::mark_json_allocation_failed();
        None
    }
}

#[inline(never)]
pub(super) fn collect_exact_values<T>(
    len: usize,
    values: impl IntoIterator<Item = Option<T>>,
) -> Option<Box<[T]>> {
    struct Initialized<'a, T> {
        storage: &'a mut [MaybeUninit<T>],
        len: usize,
    }
    impl<T> Drop for Initialized<'_, T> {
        fn drop(&mut self) {
            for value in &mut self.storage[..self.len] {
                // SAFETY: The prefix tracks exactly the entries initialized below.
                unsafe { value.assume_init_drop() };
            }
        }
    }

    let Ok(mut storage) = allocation::try_box_uninit_slice(len) else {
        pending::mark_json_allocation_failed();
        return None;
    };
    let mut values = values.into_iter();
    let mut initialized = Initialized {
        storage: &mut storage,
        len: 0,
    };
    while initialized.len < len {
        let value = values.next().flatten()?;
        initialized.storage[initialized.len].write(value);
        initialized.len += 1;
    }
    if values.next().is_some() {
        return None;
    }
    initialized.len = 0;
    drop(initialized);
    // SAFETY: The iterator initialized every entry exactly once.
    Some(unsafe { storage.assume_init() })
}

#[inline(never)]
pub(super) fn box_values<T>(values: Vec<T>) -> Box<[T]> {
    if let Ok(values) = allocation::try_into_boxed_slice(values) {
        values
    } else {
        pending::mark_json_allocation_failed();
        Box::default()
    }
}

#[inline(never)]
pub(super) fn push_value<T>(values: &mut Vec<T>, value: T) {
    if allocation::try_push(values, value).is_err() {
        pending::mark_json_allocation_failed();
    }
}

#[inline(never)]
pub(super) fn extend_values<T>(values: &mut Vec<T>, incoming: impl IntoIterator<Item = T>) {
    for value in incoming {
        if allocation::try_push(values, value).is_err() {
            pending::mark_json_allocation_failed();
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fixed_array_boxing_propagates_oom_without_a_temporary_vector() {
        crate::allocation::fault::sweep_operation(|| {
            crate::pending::with_json_error_scope(|| {
                let values = super::box_array([1_u64, 2]);
                if crate::pending::json_allocation_failed() {
                    assert!(values.is_empty());
                    vk::VkResult::ERROR_OUT_OF_HOST_MEMORY
                } else {
                    assert_eq!(&*values, &[1, 2]);
                    vk::VkResult::SUCCESS
                }
            })
        });
    }
}
