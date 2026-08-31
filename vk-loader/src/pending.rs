//! Per-thread state for synchronous loader/layer create chains.
//!
//! `std::thread_local!` uses process TLS indices on Windows. Those indices are
//! not reclaimed when a Rust DLL is unloaded, so applications which repeatedly
//! load and unload the Vulkan loader can exhaust the process TLS table. Create
//! chains are cold paths; keying a small process map by the native thread ID
//! preserves nesting semantics without imposing work on dispatch trampolines.

use std::{ffi::CString, sync::LazyLock};

use crate::collections::HashMap;
use crate::sync::Mutex;

#[derive(Default)]
struct ThreadState {
    instance: usize,
    device_sentinels: Vec<usize>,
    device_extensions: Vec<(usize, usize)>,
    created_devices: Vec<usize>,
}

static THREADS: LazyLock<Mutex<HashMap<usize, ThreadState>>> =
    LazyLock::new(|| Mutex::new(HashMap::default()));

fn thread_key() -> usize {
    crate::platform::current_thread_key()
}

fn remove_if_empty(threads: &mut HashMap<usize, ThreadState>, key: usize) {
    if threads.get(&key).is_some_and(|state| {
        state.instance == 0
            && state.device_sentinels.is_empty()
            && state.device_extensions.is_empty()
            && state.created_devices.is_empty()
    }) {
        threads.remove(&key);
        if threads.is_empty() {
            *threads = HashMap::default();
        }
    }
}

pub(crate) fn instance() -> vk::VkInstance {
    let key = thread_key();
    vk::VkInstance(
        THREADS
            .lock()
            .get(&key)
            .map_or(core::ptr::null_mut(), |state| state.instance as *mut _),
    )
}

pub(crate) fn replace_instance(instance: vk::VkInstance) -> vk::VkInstance {
    let key = thread_key();
    let mut threads = THREADS.lock();
    let state = threads.entry(key).or_default();
    let previous = core::mem::replace(&mut state.instance, instance.0 as usize);
    remove_if_empty(&mut threads, key);
    vk::VkInstance(previous as *mut _)
}

pub(crate) fn push_device_sentinel(sentinel: usize) {
    THREADS
        .lock()
        .entry(thread_key())
        .or_default()
        .device_sentinels
        .push(sentinel);
}

pub(crate) fn pop_device_sentinel() -> Option<usize> {
    let key = thread_key();
    let mut threads = THREADS.lock();
    let popped = threads
        .get_mut(&key)
        .and_then(|state| state.device_sentinels.pop());
    remove_if_empty(&mut threads, key);
    popped
}

pub(crate) fn device_sentinel() -> Option<usize> {
    THREADS
        .lock()
        .get(&thread_key())
        .and_then(|state| state.device_sentinels.last().copied())
}

pub(crate) fn push_device_extensions(extensions: &[CString]) -> (usize, usize) {
    let value = (extensions.as_ptr() as usize, extensions.len());
    THREADS
        .lock()
        .entry(thread_key())
        .or_default()
        .device_extensions
        .push(value);
    value
}

pub(crate) fn pop_device_extensions() -> Option<(usize, usize)> {
    let key = thread_key();
    let mut threads = THREADS.lock();
    let popped = threads
        .get_mut(&key)
        .and_then(|state| state.device_extensions.pop());
    remove_if_empty(&mut threads, key);
    popped
}

pub(crate) fn device_extensions() -> Option<(*const CString, usize)> {
    THREADS
        .lock()
        .get(&thread_key())
        .and_then(|state| state.device_extensions.last().copied())
        .map(|(pointer, length)| (pointer as *const CString, length))
}

pub(crate) fn push_created_device_slot() {
    THREADS
        .lock()
        .entry(thread_key())
        .or_default()
        .created_devices
        .push(0);
}

pub(crate) fn set_created_device(dispatch_key: usize) {
    if let Some(slot) = THREADS
        .lock()
        .get_mut(&thread_key())
        .and_then(|state| state.created_devices.last_mut())
    {
        *slot = dispatch_key;
    }
}

pub(crate) fn pop_created_device() -> Option<usize> {
    let key = thread_key();
    let mut threads = THREADS.lock();
    let popped = threads
        .get_mut(&key)
        .and_then(|state| state.created_devices.pop())
        .filter(|dispatch| *dispatch != 0);
    remove_if_empty(&mut threads, key);
    popped
}
