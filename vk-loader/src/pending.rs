//! Per-thread state for synchronous loader/layer create chains.
//!
//! `std::thread_local!` uses process TLS indices on Windows. Those indices are
//! not reclaimed when a Rust DLL is unloaded, so applications which repeatedly
//! load and unload the Vulkan loader can exhaust the process TLS table. Create
//! chains are cold paths; keying a small process map by the native thread ID
//! preserves nesting semantics without imposing work on dispatch trampolines.
use crate::discovery::AvailableDeviceExtensions;
use alloc::rc::Rc;
use core::marker::PhantomData;

#[cfg(windows)]
use crate::collections::HashMap;
#[cfg(windows)]
use crate::platform;
#[cfg(windows)]
use crate::sync::GlobalLazyMutex;

#[derive(Default)]
struct ThreadState {
    instance: usize,
    instance_allocators: Vec<usize>,
    json_allocation_failed: bool,
    device_sentinels: Vec<usize>,
    device_extensions: Vec<usize>,
    created_devices: Vec<usize>,
    device_layer_starts: Vec<usize>,
}

impl ThreadState {
    fn is_empty(&self) -> bool {
        self.instance == 0
            && self.device_sentinels.is_empty()
            && self.instance_allocators.is_empty()
            && !self.json_allocation_failed
            && self.device_extensions.is_empty()
            && self.created_devices.is_empty()
            && self.device_layer_starts.is_empty()
    }
}

#[cfg(windows)]
static THREADS: GlobalLazyMutex<HashMap<usize, ThreadState>> =
    GlobalLazyMutex::new(HashMap::default);

#[cfg(not(windows))]
std::thread_local! {
    // ManuallyDrop prevents registration of a TLS destructor into this shared
    // library. Empty outermost scopes replace the state and release all heaps.
    static THREAD_STATE: core::mem::ManuallyDrop<core::cell::UnsafeCell<ThreadState>> =
        const {
            core::mem::ManuallyDrop::new(core::cell::UnsafeCell::new(ThreadState {
                instance: 0,
                instance_allocators: Vec::new(),
                json_allocation_failed: false,
                device_sentinels: Vec::new(),
                device_extensions: Vec::new(),
                created_devices: Vec::new(),
                device_layer_starts: Vec::new(),
            }))
        };
}

#[cfg(not(windows))]
#[inline(never)]
fn thread_state() -> *mut ThreadState {
    THREAD_STATE.with(|state| state.get())
}

#[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
pub(crate) unsafe fn destroy_thread_state_lock() {
    #[cfg(windows)]
    // SAFETY: The caller excludes loader entry points and pending-state guards.
    unsafe {
        THREADS.destroy()
    };
}

#[cfg(windows)]
fn with_thread_state<R>(operation: impl FnOnce(&ThreadState) -> R) -> Option<R> {
    let threads = THREADS.lock_if_initialized()?;
    threads.get(&platform::current_thread_key()).map(operation)
}

#[cfg(not(windows))]
fn with_thread_state<R>(operation: impl FnOnce(&ThreadState) -> R) -> Option<R> {
    // SAFETY: The pointer is current-thread storage, the reference does not
    // escape this call, and pending-state operations are non-reentrant.
    let state = core::ptr::NonNull::new(thread_state())?;
    Some(operation(unsafe { state.as_ref() }))
}

#[cfg(windows)]
fn with_thread_state_mut<R>(operation: impl FnOnce(&mut ThreadState) -> R) -> Option<R> {
    let key = platform::current_thread_key();
    let mut threads = THREADS.lock_if_initialized()?;
    let result = threads.get_mut(&key).map(operation);
    remove_if_empty(&mut threads, key);
    result
}

#[cfg(not(windows))]
fn with_thread_state_mut<R>(operation: impl FnOnce(&mut ThreadState) -> R) -> Option<R> {
    // SAFETY: The pointer is current-thread storage, the reference does not
    // escape this call, and pending-state operations are non-reentrant.
    let mut state = core::ptr::NonNull::new(thread_state())?;
    let state = unsafe { state.as_mut() };
    let result = operation(state);
    if state.is_empty() {
        *state = ThreadState::default();
    }
    Some(result)
}

#[cfg(windows)]
fn remove_if_empty(threads: &mut HashMap<usize, ThreadState>, key: usize) {
    if threads.get(&key).is_some_and(ThreadState::is_empty) {
        threads.remove(&key);
        if threads.is_empty() {
            *threads = HashMap::default();
        }
    }
}

#[cfg(windows)]
fn try_with_thread_state_mut<R>(
    operation: impl FnOnce(&mut ThreadState) -> Result<R, vk::VkResult>,
) -> Result<R, vk::VkResult> {
    let key = platform::current_thread_key();
    let mut threads = THREADS.try_lock()?;
    if !threads.contains_key(&key) {
        threads
            .try_reserve(1)
            .map_err(|_| vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    }
    let result = operation(threads.entry(key).or_default());
    remove_if_empty(&mut threads, key);
    result
}

#[cfg(not(windows))]
fn try_with_thread_state_mut<R>(
    operation: impl FnOnce(&mut ThreadState) -> Result<R, vk::VkResult>,
) -> Result<R, vk::VkResult> {
    // SAFETY: The pointer is current-thread storage, the reference does not
    // escape this call, and pending-state operations are non-reentrant.
    let state = unsafe { &mut *thread_state() };
    let result = operation(state);
    if state.is_empty() {
        *state = ThreadState::default();
    }
    result
}

pub(crate) struct InstanceAllocatorGuard {
    pointer: usize,
    // A synchronous create-chain guard must be dropped on its originating
    // thread. This marker prevents Send/Sync without retaining an Rc.
    thread_bound: PhantomData<Rc<()>>,
}

impl Drop for InstanceAllocatorGuard {
    fn drop(&mut self) {
        let popped = with_thread_state_mut(|state| {
            let popped = state.instance_allocators.pop();
            if state.instance_allocators.is_empty() {
                state.json_allocation_failed = false;
            }
            popped
        })
        .flatten();
        debug_assert_eq!(popped, Some(self.pointer));
    }
}

pub(crate) fn push_instance_allocator(
    callbacks: *const vk::VkAllocationCallbacks<'_>,
) -> Result<InstanceAllocatorGuard, vk::VkResult> {
    let pointer = callbacks as usize;
    try_with_thread_state_mut(|state| {
        state
            .instance_allocators
            .try_reserve(1)
            .map_err(|_| vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        state.instance_allocators.push(pointer);
        Ok(())
    })?;
    Ok(InstanceAllocatorGuard {
        pointer,
        thread_bound: PhantomData,
    })
}

pub(crate) fn instance_allocator() -> Option<*const vk::VkAllocationCallbacks<'static>> {
    with_thread_state(|state| state.instance_allocators.last().copied())
        .flatten()
        .filter(|pointer| *pointer != 0)
        .map(|pointer| pointer as *const vk::VkAllocationCallbacks<'static>)
}

pub(crate) fn mark_json_allocation_failed() {
    // API entry reserves this state before discovery. Recording an allocation
    // failure must never attempt another allocation.
    with_thread_state_mut(|state| {
        state.json_allocation_failed = true;
    });
}

/// Gives global enumeration the same reserved error state as instance creation.
/// Preserve an enclosing create chain's callbacks and pending failure status.
pub(crate) fn with_json_error_scope(operation: impl FnOnce() -> vk::VkResult) -> vk::VkResult {
    let callbacks = instance_allocator().unwrap_or(core::ptr::null());
    let _guard = match push_instance_allocator(callbacks) {
        Ok(guard) => guard,
        Err(error) => return error,
    };
    let previous_failure = take_json_allocation_failed();
    let result = operation();
    let failed = take_json_allocation_failed();
    if previous_failure {
        mark_json_allocation_failed();
    }
    if failed {
        vk::VkResult::ERROR_OUT_OF_HOST_MEMORY
    } else {
        result
    }
}

pub(crate) fn take_json_allocation_failed() -> bool {
    with_thread_state_mut(|state| core::mem::replace(&mut state.json_allocation_failed, false))
        .unwrap_or(false)
}

pub(crate) fn json_allocation_failed() -> bool {
    with_thread_state(|state| state.json_allocation_failed).unwrap_or(false)
}

pub(crate) fn instance() -> vk::VkInstance {
    vk::VkInstance(with_thread_state(|state| state.instance).unwrap_or(0) as *mut _)
}

pub(crate) fn replace_instance(instance: vk::VkInstance) -> vk::VkInstance {
    // Create entry points reserve this thread's state before calling a chain.
    // Never create a map entry while installing/restoring a pending handle.
    let previous =
        with_thread_state_mut(|state| core::mem::replace(&mut state.instance, instance.0 as usize));
    debug_assert!(previous.is_some() || instance == vk::VkInstance::NULL);
    vk::VkInstance(previous.unwrap_or(0) as *mut _)
}

pub(crate) fn pop_device_sentinel() -> Option<usize> {
    with_thread_state_mut(|state| state.device_sentinels.pop()).flatten()
}

pub(crate) fn device_sentinel() -> Option<usize> {
    with_thread_state(|state| state.device_sentinels.last().copied()).flatten()
}

pub(crate) struct DeviceExtensionsGuard {
    pointer: usize,
    thread_bound: PhantomData<Rc<()>>,
}

impl Drop for DeviceExtensionsGuard {
    fn drop(&mut self) {
        let popped = with_thread_state_mut(|state| state.device_extensions.pop()).flatten();
        debug_assert_eq!(popped, Some(self.pointer));
    }
}

pub(crate) fn push_device_extensions(
    extensions: &AvailableDeviceExtensions,
) -> Result<DeviceExtensionsGuard, vk::VkResult> {
    let value = core::ptr::from_ref(extensions) as usize;
    try_with_thread_state_mut(|state| {
        state
            .device_extensions
            .try_reserve(1)
            .map_err(|_| vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        state.device_extensions.push(value);
        Ok(())
    })?;
    Ok(DeviceExtensionsGuard {
        pointer: value,
        thread_bound: PhantomData,
    })
}

/// Reserves every stack entry before entering a device chain or creating any
/// native device. Subsequent pushes cannot allocate, including nested chains.
pub(crate) fn start_device_chain(
    sentinel: usize,
) -> Result<DeviceLayerStartReservation, vk::VkResult> {
    try_with_thread_state_mut(|state| {
        state
            .device_sentinels
            .try_reserve(1)
            .and_then(|()| state.created_devices.try_reserve(1))
            .and_then(|()| state.device_layer_starts.try_reserve(1))
            .map_err(|_| vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        state.device_sentinels.push(sentinel);
        state.created_devices.push(0);
        Ok(())
    })?;
    Ok(DeviceLayerStartReservation {
        thread_bound: PhantomData,
    })
}

/// A single reserved layer-start slot on the originating thread. The active
/// sentinel keeps its `ThreadState` alive while layer negotiation takes place.
pub(crate) struct DeviceLayerStartReservation {
    thread_bound: PhantomData<Rc<()>>,
}

impl DeviceLayerStartReservation {
    pub(crate) fn push(_reservation: Self, first: usize) {
        let inserted = with_thread_state_mut(|state| state.device_layer_starts.push(first));
        debug_assert!(inserted.is_some());
    }
}

pub(crate) fn device_extensions() -> Option<*const AvailableDeviceExtensions> {
    with_thread_state(|state| state.device_extensions.last().copied())
        .flatten()
        .map(|pointer| pointer as *const AvailableDeviceExtensions)
}

pub(crate) fn device_layer_start() -> usize {
    with_thread_state(|state| state.device_layer_starts.last().copied())
        .flatten()
        .unwrap_or(0)
}

pub(crate) fn pop_device_layer_start() -> Option<usize> {
    with_thread_state_mut(|state| state.device_layer_starts.pop()).flatten()
}

pub(crate) fn set_created_device(dispatch_key: usize) {
    with_thread_state_mut(|state| {
        if let Some(slot) = state.created_devices.last_mut() {
            *slot = dispatch_key;
        }
    });
}

pub(crate) fn pop_created_device() -> Option<usize> {
    with_thread_state_mut(|state| state.created_devices.pop())
        .flatten()
        .filter(|dispatch| *dispatch != 0)
}

#[cfg(test)]
mod tests {
    use super::{
        device_layer_start, device_sentinel, pop_created_device, pop_device_layer_start,
        pop_device_sentinel, set_created_device, start_device_chain, with_json_error_scope,
    };
    use vk::VkResult;

    #[test]
    fn nested_device_slots_preserve_outer_state_and_rollback_on_oom() {
        crate::allocation::fault::sweep_operation(|| {
            with_json_error_scope(|| {
                let outer = match start_device_chain(11) {
                    Ok(reservation) => reservation,
                    Err(error) => return error,
                };
                super::DeviceLayerStartReservation::push(outer, 3);
                set_created_device(101);
                let result = match start_device_chain(22) {
                    Ok(inner) => {
                        // Negotiation sees the parent's layer index until the
                        // reserved inner slot is published, as in the layer chain.
                        assert_eq!(device_layer_start(), 3);
                        super::DeviceLayerStartReservation::push(inner, 7);
                        set_created_device(202);
                        assert_eq!(device_sentinel(), Some(22));
                        assert_eq!(pop_device_layer_start(), Some(7));
                        assert_eq!(pop_created_device(), Some(202));
                        assert_eq!(pop_device_sentinel(), Some(22));
                        VkResult::SUCCESS
                    }
                    Err(error) => error,
                };
                assert_eq!(device_sentinel(), Some(11));
                assert_eq!(device_layer_start(), 3);
                assert_eq!(pop_device_layer_start(), Some(3));
                assert_eq!(pop_created_device(), Some(101));
                assert_eq!(pop_device_sentinel(), Some(11));
                result
            })
        });
    }
}
