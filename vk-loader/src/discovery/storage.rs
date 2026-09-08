//! Fallible discovery storage with pending-error propagation.

use crate::{allocation, pending};
use alloc::{borrow::Cow, ffi::CString, string::String, vec::Vec};
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

pub(super) fn owned_string(value: &str) -> String {
    if let Ok(value) = allocation::try_string(value) {
        value
    } else {
        pending::mark_json_allocation_failed();
        String::new()
    }
}

pub(super) fn own_cow(value: Cow<'_, str>) -> String {
    match value {
        Cow::Owned(value) => value,
        Cow::Borrowed(value) => owned_string(value),
    }
}

pub(super) fn box_array<T, const N: usize>(values: [T; N]) -> Box<[T]> {
    collect_values(values).unwrap_or_default()
}

pub(super) fn collect_values<T>(values: impl IntoIterator<Item = T>) -> Option<Box<[T]>> {
    if let Ok(values) = allocation::try_collect(values) {
        Some(values)
    } else {
        pending::mark_json_allocation_failed();
        None
    }
}

pub(super) fn box_values<T>(values: Vec<T>) -> Box<[T]> {
    if let Ok(values) = allocation::try_into_boxed_slice(values) {
        values
    } else {
        pending::mark_json_allocation_failed();
        Box::default()
    }
}

pub(super) fn push_value<T>(values: &mut Vec<T>, value: T) {
    if allocation::try_push(values, value).is_err() {
        pending::mark_json_allocation_failed();
    }
}

pub(super) fn extend_values<T>(values: &mut Vec<T>, incoming: impl IntoIterator<Item = T>) {
    for value in incoming {
        if allocation::try_push(values, value).is_err() {
            pending::mark_json_allocation_failed();
            break;
        }
    }
}

pub(super) fn collect_optional_values<T>(
    values: impl IntoIterator<Item = Option<T>>,
) -> Option<Box<[T]>> {
    let mut valid = true;
    let collected = collect_values(values.into_iter().map_while(|value| {
        valid = value.is_some();
        value
    }))?;
    valid.then_some(collected)
}
