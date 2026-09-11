//! Loader-owned `VK_EXT_debug_utils` messenger state and entry points.

use alloc::{boxed::Box, vec::Vec};
use core::{ffi::c_void, mem, ptr::NonNull};

use vk::{
    PFN_vkDebugReportCallbackEXT, PFN_vkDebugUtilsMessengerCallbackEXT, VkAllocationCallbacks,
    VkDebugReportCallbackCreateInfoEXT, VkDebugReportCallbackEXT, VkDebugReportFlagBitsEXT,
    VkDebugReportFlagsEXT, VkDebugReportObjectTypeEXT, VkDebugUtilsMessageSeverityFlagBitsEXT,
    VkDebugUtilsMessageTypeFlagBitsEXT, VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessengerCallbackDataEXT, VkDebugUtilsMessengerCreateInfoEXT,
    VkDebugUtilsMessengerEXT, VkInstance, VkInstanceCreateInfo, VkObjectType, VkResult,
    VkStructureType, VkSystemAllocationScope,
};

use crate::{
    allocation::{self, LoaderBox, try_boxed_slice_filled},
    emulation::for_each_input_chain,
    instance::LoaderInstance,
    platform,
};

/// Converts debug-utils severity/type bits to the legacy debug-report flag,
/// matching Vulkan-Loader's `debug_utils_AnnotFlagsToReportFlags` priority.
pub(crate) const fn debug_report_flags(
    severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message_types: VkDebugUtilsMessageTypeFlagsEXT,
) -> VkDebugReportFlagsEXT {
    if severity.intersects(VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR) {
        VkDebugReportFlagBitsEXT::ERROR
    } else if severity.intersects(VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING) {
        if message_types.intersects(VkDebugUtilsMessageTypeFlagBitsEXT::PERFORMANCE) {
            VkDebugReportFlagBitsEXT::PERFORMANCE_WARNING
        } else {
            VkDebugReportFlagBitsEXT::WARNING
        }
    } else if severity.intersects(VkDebugUtilsMessageSeverityFlagBitsEXT::INFO) {
        VkDebugReportFlagBitsEXT::INFORMATION
    } else if severity.intersects(VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE) {
        VkDebugReportFlagBitsEXT::DEBUG
    } else {
        VkDebugReportFlagBitsEXT::EMPTY
    }
}

/// Converts legacy debug-report flags to debug-utils severity/type bits,
/// matching upstream's `debug_utils_ReportFlagsToAnnotFlags(..., false, ...)`.
pub(crate) const fn debug_utils_flags(
    flags: VkDebugReportFlagsEXT,
) -> (
    VkDebugUtilsMessageSeverityFlagBitsEXT,
    VkDebugUtilsMessageTypeFlagsEXT,
) {
    let severity = if flags.intersects(VkDebugReportFlagBitsEXT::INFORMATION) {
        VkDebugUtilsMessageSeverityFlagBitsEXT::INFO
    } else if flags.intersects(VkDebugReportFlagBitsEXT::WARNING)
        || flags.intersects(VkDebugReportFlagBitsEXT::PERFORMANCE_WARNING)
    {
        VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING
    } else if flags.intersects(VkDebugReportFlagBitsEXT::ERROR) {
        VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR
    } else if flags.intersects(VkDebugReportFlagBitsEXT::DEBUG) {
        VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE
    } else {
        VkDebugUtilsMessageSeverityFlagBitsEXT::EMPTY
    };
    let message_types = if flags.intersects(VkDebugReportFlagBitsEXT::PERFORMANCE_WARNING) {
        VkDebugUtilsMessageTypeFlagBitsEXT::PERFORMANCE
    } else {
        VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL
    };
    (severity, message_types)
}

pub(crate) struct DebugMessengerState {
    pub(crate) callbacks: Vec<DebugCallback>,
    messenger_slots: Vec<bool>,
    messenger_slot_allocation: CallbackBuffer,
    messenger_icd_allocations: Vec<CallbackBuffer>,
}

pub(crate) enum DebugCallback {
    Messenger(LoaderBox<DebugMessenger>),
    Report(LoaderBox<DebugReport>),
}

impl DebugMessengerState {
    pub(crate) const fn new() -> Self {
        Self {
            callbacks: Vec::new(),
            messenger_slots: Vec::new(),
            messenger_slot_allocation: CallbackBuffer::new(),
            messenger_icd_allocations: Vec::new(),
        }
    }

    fn reserve_messenger(&mut self, instance: &LoaderInstance) -> Result<usize, VkResult> {
        self.reserve_messenger_storage(instance.icds.len(), instance.allocator())
    }

    fn reserve_messenger_storage(
        &mut self,
        icd_count: usize,
        callbacks: Option<&VkAllocationCallbacks<'static>>,
    ) -> Result<usize, VkResult> {
        let index = self
            .messenger_slots
            .iter()
            .position(|used| !*used)
            .unwrap_or(self.messenger_slots.len());
        let grow_slots = index == self.messenger_slots.len();
        let required = if grow_slots {
            self.messenger_slot_allocation.next_entries()?
        } else {
            self.messenger_slots.len()
        };
        self.messenger_slots
            .try_reserve(required - self.messenger_slots.len())
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        self.messenger_icd_allocations
            .try_reserve(icd_count.saturating_sub(self.messenger_icd_allocations.len()))
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        if grow_slots
            && let Err(result) = self
                .messenger_slot_allocation
                .grow(callbacks, mem::size_of::<UsedObjectStatus>())
        {
            platform::write_loader_log(
                platform::LogFilter::Error,
                format_args!(
                    "loader_resize_generic_list: Failed to allocate space for generic list"
                ),
            );
            return Err(result);
        }
        self.messenger_slots
            .resize(self.messenger_slot_allocation.entries, false);
        if self.messenger_icd_allocations.len() < icd_count {
            self.messenger_icd_allocations
                .resize_with(icd_count, CallbackBuffer::new);
        }
        for allocation in &mut self.messenger_icd_allocations {
            if allocation.entries <= index
                && let Err(result) =
                    allocation.grow(callbacks, mem::size_of::<VkDebugUtilsMessengerEXT>())
            {
                platform::write_loader_log(
                    platform::LogFilter::Error,
                    format_args!(
                        "loader_resize_generic_list: Failed to allocate space for generic list"
                    ),
                );
                return Err(result);
            }
        }
        self.messenger_slots[index] = true;
        Ok(index)
    }

    fn release_messenger(&mut self, index: usize) {
        if let Some(slot) = self.messenger_slots.get_mut(index) {
            *slot = false;
        }
    }

    fn release_allocations(&mut self, instance: &LoaderInstance) {
        self.messenger_slot_allocation.release(instance.allocator());
        for allocation in &mut self.messenger_icd_allocations {
            allocation.release(instance.allocator());
        }
    }
}

#[repr(C)]
struct UsedObjectStatus {
    status: u32,
    callbacks: VkAllocationCallbacks<'static>,
}

struct CallbackBuffer {
    pointer: Option<NonNull<c_void>>,
    entries: usize,
}

// Access is serialized by the instance debug-state mutex.
unsafe impl Send for CallbackBuffer {}

impl CallbackBuffer {
    const fn new() -> Self {
        Self {
            pointer: None,
            entries: 0,
        }
    }

    fn next_entries(&self) -> Result<usize, VkResult> {
        if self.entries == 0 {
            Ok(32)
        } else {
            self.entries
                .checked_mul(2)
                .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)
        }
    }

    fn grow(
        &mut self,
        callbacks: Option<&VkAllocationCallbacks<'static>>,
        entry_size: usize,
    ) -> Result<(), VkResult> {
        let new_entries = self.next_entries()?;
        let size = new_entries
            .checked_mul(entry_size)
            .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        if let Some(callbacks) = callbacks {
            let pointer = if let Some(pointer) = self.pointer {
                let Some(reallocate) = callbacks.pfnReallocation else {
                    return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
                };
                // SAFETY: `pointer` came from these same callbacks.
                unsafe {
                    reallocate(
                        callbacks.pUserData,
                        pointer.as_ptr(),
                        size,
                        mem::align_of::<UsedObjectStatus>().max(mem::align_of::<usize>()),
                        VkSystemAllocationScope::OBJECT,
                    )
                }
            } else {
                let Some(allocate) = callbacks.pfnAllocation else {
                    return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
                };
                // SAFETY: The application allocator remains live for the instance.
                unsafe {
                    allocate(
                        callbacks.pUserData,
                        size,
                        mem::align_of::<UsedObjectStatus>().max(mem::align_of::<usize>()),
                        VkSystemAllocationScope::OBJECT,
                    )
                }
            };
            let Some(pointer) = NonNull::new(pointer) else {
                return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
            };
            self.pointer = Some(pointer);
        }
        self.entries = new_entries;
        Ok(())
    }

    fn release(&mut self, callbacks: Option<&VkAllocationCallbacks<'static>>) {
        if let Some(pointer) = self.pointer
            && let Some(callbacks) = callbacks
            && let Some(free) = callbacks.pfnFree
        {
            // SAFETY: This block was allocated by the matching instance callbacks.
            unsafe { free(callbacks.pUserData, pointer.as_ptr()) };
        }
        self.pointer = None;
        self.entries = 0;
    }
}

enum IndexAllocation {
    Callback {
        pointer: NonNull<u32>,
        callbacks: VkAllocationCallbacks<'static>,
    },
    Rust(Box<u32>),
}

unsafe impl Send for IndexAllocation {}
unsafe impl Sync for IndexAllocation {}

impl IndexAllocation {
    unsafe fn new(
        instance: &LoaderInstance,
        allocator: *const VkAllocationCallbacks<'_>,
    ) -> Result<Self, VkResult> {
        let callbacks = if allocator.is_null() {
            instance.allocator().copied()
        } else {
            // SAFETY: Vulkan retains application callbacks through object lifetime.
            Some(unsafe {
                mem::transmute::<VkAllocationCallbacks<'_>, VkAllocationCallbacks<'static>>(
                    allocator.read(),
                )
            })
        };
        let Some(callbacks) = callbacks else {
            return allocation::try_box(0)
                .map(Self::Rust)
                .map_err(|(result, _index)| result);
        };
        let Some(allocate) = callbacks.pfnAllocation else {
            return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
        };
        // SAFETY: The selected callbacks are live for the object lifetime.
        let pointer = unsafe {
            allocate(
                callbacks.pUserData,
                mem::size_of::<u32>(),
                mem::align_of::<u32>(),
                VkSystemAllocationScope::OBJECT,
            )
        }
        .cast::<u32>();
        let Some(pointer) = NonNull::new(pointer) else {
            return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
        };
        Ok(Self::Callback { pointer, callbacks })
    }

    fn pointer(&self) -> *mut u32 {
        match self {
            Self::Callback { pointer, .. } => pointer.as_ptr(),
            Self::Rust(index) => core::ptr::from_ref(index.as_ref()).cast_mut(),
        }
    }
}

impl Drop for IndexAllocation {
    fn drop(&mut self) {
        if let Self::Callback { pointer, callbacks } = self
            && let Some(free) = callbacks.pfnFree
        {
            // SAFETY: This pointer was allocated by these matching callbacks.
            unsafe { free(callbacks.pUserData, pointer.as_ptr().cast()) };
        }
    }
}

pub(crate) struct DebugReport {
    pub(crate) callback: PFN_vkDebugReportCallbackEXT,
    flags: VkDebugReportFlagsEXT,
    pub(crate) user_data: *mut c_void,
    icd_handles: Box<[VkDebugReportCallbackEXT]>,
    allocator: Option<VkAllocationCallbacks<'static>>,
}

// The application owns callback user data and its synchronization contract.
unsafe impl Send for DebugReport {}
unsafe impl Sync for DebugReport {}

impl DebugReport {
    pub(crate) const fn accepts(&self, flags: VkDebugReportFlagsEXT) -> bool {
        self.flags.0 & flags.0 != 0
    }
}

pub(crate) struct DebugMessenger {
    pub(crate) callback: PFN_vkDebugUtilsMessengerCallbackEXT,
    severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message_types: VkDebugUtilsMessageTypeFlagsEXT,
    pub(crate) user_data: *mut c_void,
    icd_handles: Box<[VkDebugUtilsMessengerEXT]>,
    allocator: Option<VkAllocationCallbacks<'static>>,
    slot: usize,
    index_allocation: IndexAllocation,
}

// Vulkan permits callbacks to be registered and invoked across threads. The
// application owns `user_data` and is responsible for synchronizing its use.
unsafe impl Send for DebugMessenger {}
unsafe impl Sync for DebugMessenger {}

impl DebugMessenger {
    #[inline]
    pub(crate) const fn accepts(
        &self,
        severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
        message_types: VkDebugUtilsMessageTypeFlagsEXT,
    ) -> bool {
        self.severity.0 & severity.0 != 0 && self.message_types.0 & message_types.0 != 0
    }
}

#[inline(never)]
unsafe fn retain_allocator(
    allocator: *const VkAllocationCallbacks<'_>,
) -> Option<VkAllocationCallbacks<'static>> {
    if allocator.is_null() {
        None
    } else {
        // SAFETY: Vulkan requires allocation callbacks to remain valid until
        // the matching object destruction operation.
        Some(unsafe {
            mem::transmute::<VkAllocationCallbacks<'_>, VkAllocationCallbacks<'static>>(
                allocator.read(),
            )
        })
    }
}

#[derive(Clone, Copy)]
enum NativeDebugKind {
    Messenger,
    Report,
}

#[cold]
#[inline(never)]
unsafe fn create_native_debug_callbacks(
    instance: &LoaderInstance,
    create_info: *const c_void,
    allocator: *const VkAllocationCallbacks<'_>,
    handles: *mut u64,
    kind: NativeDebugKind,
) -> VkResult {
    const {
        assert!(mem::size_of::<VkDebugUtilsMessengerEXT>() == mem::size_of::<u64>());
        assert!(mem::size_of::<VkDebugReportCallbackEXT>() == mem::size_of::<u64>());
    }
    for (index, icd) in instance.active_icds() {
        let output = unsafe { handles.add(index) };
        let result = match kind {
            NativeDebugKind::Messenger => {
                let Some(create) = icd.dispatch.vkCreateDebugUtilsMessengerEXT else {
                    continue;
                };
                unsafe { create(icd.handle, create_info.cast(), allocator, output.cast()) }
            }
            NativeDebugKind::Report => {
                let Some(create) = icd.dispatch.vkCreateDebugReportCallbackEXT else {
                    continue;
                };
                unsafe { create(icd.handle, create_info.cast(), allocator, output.cast()) }
            }
        };
        if result == VkResult::SUCCESS {
            continue;
        }
        for created_index in 0..index {
            let handle = unsafe { handles.add(created_index).read() };
            if handle == 0 {
                continue;
            }
            let created_icd = &instance.icds[created_index];
            match kind {
                NativeDebugKind::Messenger => {
                    if let Some(destroy) = created_icd.dispatch.vkDestroyDebugUtilsMessengerEXT {
                        unsafe {
                            destroy(
                                created_icd.handle,
                                VkDebugUtilsMessengerEXT(handle),
                                allocator,
                            );
                        }
                    }
                }
                NativeDebugKind::Report => {
                    if let Some(destroy) = created_icd.dispatch.vkDestroyDebugReportCallbackEXT {
                        unsafe {
                            destroy(
                                created_icd.handle,
                                VkDebugReportCallbackEXT(handle),
                                allocator,
                            );
                        }
                    }
                }
            }
        }
        return result;
    }
    VkResult::SUCCESS
}

fn forced_destroy_allocator(
    allocator: Option<&VkAllocationCallbacks<'static>>,
) -> *const VkAllocationCallbacks<'static> {
    let Some(callbacks) = allocator else {
        return core::ptr::null();
    };
    if callbacks.pfnAllocation.is_some()
        && callbacks.pfnReallocation.is_some()
        && callbacks.pfnFree.is_some()
        && callbacks.pfnInternalAllocation.is_some()
        && callbacks.pfnInternalFree.is_some()
    {
        callbacks
    } else {
        core::ptr::null()
    }
}

/// Delivers a loader warning to callbacks embedded in instance creation.
///
/// # Safety
///
/// The complete instance-create `pNext` chain must be live and well formed.
#[cold]
#[inline(never)]
pub(crate) unsafe fn submit_instance_create_message(
    create_info: &VkInstanceCreateInfo<'_>,
    severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: &core::ffi::CStr,
) {
    let object = vk::VkDebugUtilsObjectNameInfoEXT {
        objectType: VkObjectType::INSTANCE,
        ..vk::VkDebugUtilsObjectNameInfoEXT::DEFAULT
    };
    let callback_data = VkDebugUtilsMessengerCallbackDataEXT {
        pMessageIdName: c"Loader Message".as_ptr(),
        pMessage: message.as_ptr(),
        objectCount: 1,
        pObjects: core::ptr::from_ref(&object),
        ..VkDebugUtilsMessengerCallbackDataEXT::DEFAULT
    };
    // SAFETY: The caller guarantees a valid input structure chain.
    unsafe {
        for_each_input_chain(create_info.pNext, |structure| {
            match structure.sType {
                VkStructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT => {
                    // SAFETY: `sType` identifies the concrete structure layout.
                    let create = &*core::ptr::from_ref(structure)
                        .cast::<VkDebugUtilsMessengerCreateInfoEXT<'_>>();
                    if create.messageSeverity.intersects(severity)
                        && create
                            .messageType
                            .intersects(VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL)
                        && let Some(callback) = create.pfnUserCallback
                    {
                        // SAFETY: The application provided this callback and live user data.
                        callback(
                            severity,
                            VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
                            core::ptr::from_ref(&callback_data),
                            create.pUserData,
                        );
                    }
                }
                VkStructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT => {
                    // SAFETY: `sType` identifies the concrete structure layout.
                    let create = &*core::ptr::from_ref(structure)
                        .cast::<VkDebugReportCallbackCreateInfoEXT<'_>>();
                    let report_flags =
                        debug_report_flags(severity, VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL);
                    if create.flags.intersects(report_flags)
                        && let Some(callback) = create.pfnCallback
                    {
                        // SAFETY: The application provided this callback and live user data.
                        callback(
                            report_flags,
                            VkDebugReportObjectTypeEXT::INSTANCE,
                            0,
                            0,
                            0,
                            c"Loader Message".as_ptr(),
                            message.as_ptr(),
                            create.pUserData,
                        );
                    }
                }
                _ => {}
            }
        });
    }
}

/// Creates one loader messenger backed by zero or one native messenger per ICD.
///
/// # Safety
///
/// Arguments must satisfy `vkCreateDebugUtilsMessengerEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn terminator_create_debug_utils_messenger(
    instance: VkInstance,
    create_info: *const VkDebugUtilsMessengerCreateInfoEXT<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    messenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult {
    if create_info.is_null() || messenger.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The terminator receives the live loader instance at the end of the chain.
    let Some(instance) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    // SAFETY: The Vulkan entry-point contract guarantees readable create info.
    let create_info = unsafe { &*create_info };
    if create_info.pfnUserCallback.is_none() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }

    // Upstream represents the non-dispatchable handle as an allocator-backed
    // pointer to its reserved used-object index.
    let index_allocation = match unsafe { IndexAllocation::new(instance, allocator) } {
        Ok(allocation) => allocation,
        Err(result) => return result,
    };
    let slot = {
        let mut state = instance.debug_messengers.lock();
        match state.reserve_messenger(instance) {
            Ok(slot) => slot,
            Err(result) => return result,
        }
    };
    // SAFETY: The index allocation contains one writable `u32`.
    unsafe {
        index_allocation.pointer().write(slot as u32);
    };

    let mut native =
        match try_boxed_slice_filled(instance.icds.len(), VkDebugUtilsMessengerEXT::NULL) {
            Ok(native) => native,
            Err(result) => {
                instance.debug_messengers.lock().release_messenger(slot);
                return result;
            }
        };
    let result = unsafe {
        create_native_debug_callbacks(
            instance,
            core::ptr::from_ref(create_info).cast(),
            allocator,
            native.as_mut_ptr().cast(),
            NativeDebugKind::Messenger,
        )
    };
    if result != VkResult::SUCCESS {
        instance.debug_messengers.lock().release_messenger(slot);
        return result;
    }

    let object_allocator = unsafe { retain_allocator(allocator) };
    let storage_allocator = object_allocator
        .as_ref()
        .or_else(|| instance.allocator())
        .copied();
    let mut owned = match LoaderBox::<DebugMessenger>::try_new_uninit(
        storage_allocator.as_ref(),
        VkSystemAllocationScope::OBJECT,
    ) {
        Ok(owned) => owned,
        Err(result) => {
            instance.debug_messengers.lock().release_messenger(slot);
            destroy_native(instance, &native, allocator);
            return result;
        }
    };
    let pointer = owned.as_mut_ptr();
    // SAFETY: Allocation succeeded and every field is written exactly once;
    // no fallible operation can interrupt initialization.
    unsafe {
        core::ptr::addr_of_mut!((*pointer).callback).write(create_info.pfnUserCallback);
        core::ptr::addr_of_mut!((*pointer).severity).write(create_info.messageSeverity);
        core::ptr::addr_of_mut!((*pointer).message_types).write(create_info.messageType);
        core::ptr::addr_of_mut!((*pointer).user_data).write(create_info.pUserData);
        core::ptr::addr_of_mut!((*pointer).icd_handles).write(native);
        core::ptr::addr_of_mut!((*pointer).allocator).write(object_allocator);
        core::ptr::addr_of_mut!((*pointer).slot).write(slot);
        core::ptr::addr_of_mut!((*pointer).index_allocation).write(index_allocation);
    }
    // SAFETY: Every `DebugMessenger` field was initialized above.
    let owned = unsafe { owned.assume_init() };
    let address = owned.index_allocation.pointer() as usize;
    let mut state = instance.debug_messengers.lock();
    if state.callbacks.try_reserve(1).is_err() {
        state.release_messenger(slot);
        drop(state);
        destroy_native(instance, &owned.icd_handles, allocator);
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    instance.set_has_debug_callbacks(true);
    state.callbacks.push(DebugCallback::Messenger(owned));
    drop(state);
    // SAFETY: The caller supplies writable output storage.
    unsafe { messenger.write(VkDebugUtilsMessengerEXT(address as u64)) };
    VkResult::SUCCESS
}

/// Destroys a loader messenger and all native messenger objects it represents.
///
/// # Safety
///
/// Arguments must satisfy `vkDestroyDebugUtilsMessengerEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn terminator_destroy_debug_utils_messenger(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    allocator: *const VkAllocationCallbacks<'_>,
) {
    if messenger == VkDebugUtilsMessengerEXT::NULL {
        return;
    }
    // SAFETY: The terminator receives a live loader instance.
    let Some(instance) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return;
    };
    let key = messenger.0 as usize;
    let owned = {
        let mut state = instance.debug_messengers.lock();
        let Some(index) = state.callbacks.iter().position(|entry| {
            matches!(entry, DebugCallback::Messenger(messenger) if messenger.index_allocation.pointer() as usize == key)
        }) else {
            return;
        };
        let DebugCallback::Messenger(owned) = state.callbacks.remove(index) else {
            unreachable!();
        };
        state.release_messenger(owned.slot);
        instance.set_has_debug_callbacks(!state.callbacks.is_empty());
        owned
    };
    destroy_native(instance, &owned.icd_handles, allocator);
}

/// Delivers an application-submitted message exactly once to loader callbacks.
///
/// # Safety
///
/// Arguments must satisfy `vkSubmitDebugUtilsMessageEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn terminator_submit_debug_utils_message(
    instance: VkInstance,
    severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message_types: VkDebugUtilsMessageTypeFlagsEXT,
    callback_data: *const VkDebugUtilsMessengerCallbackDataEXT<'_>,
) {
    if callback_data.is_null() {
        return;
    }
    // SAFETY: Instance creation returns the loader-owned pending handle to
    // the bottom of the layer chain; this terminator never receives a wrapper.
    let Some(instance) = (unsafe { LoaderInstance::from_internal_handle(instance) }) else {
        return;
    };
    // SAFETY: The callback-data pointer was validated above and remains live
    // for this synchronous callback delivery.
    let callback_data = unsafe { &*callback_data };
    instance.submit_debug_message(severity, message_types, callback_data);
}

/// Creates one loader debug-report callback backed by per-ICD callbacks.
///
/// # Safety
///
/// Arguments must satisfy `vkCreateDebugReportCallbackEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn terminator_create_debug_report_callback(
    instance: VkInstance,
    create_info: *const VkDebugReportCallbackCreateInfoEXT<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    callback: *mut VkDebugReportCallbackEXT,
) -> VkResult {
    if create_info.is_null() || callback.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The terminator receives the live loader instance.
    let Some(instance) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    // SAFETY: The command contract guarantees readable create info.
    let create_info = unsafe { &*create_info };
    let mut native =
        match try_boxed_slice_filled(instance.icds.len(), VkDebugReportCallbackEXT::NULL) {
            Ok(native) => native,
            Err(result) => return result,
        };
    let result = unsafe {
        create_native_debug_callbacks(
            instance,
            core::ptr::from_ref(create_info).cast(),
            allocator,
            native.as_mut_ptr().cast(),
            NativeDebugKind::Report,
        )
    };
    if result != VkResult::SUCCESS {
        return result;
    }
    let object_allocator = unsafe { retain_allocator(allocator) };
    let storage_allocator = object_allocator
        .as_ref()
        .or_else(|| instance.allocator())
        .copied();
    let mut owned = match LoaderBox::<DebugReport>::try_new_uninit(
        storage_allocator.as_ref(),
        VkSystemAllocationScope::OBJECT,
    ) {
        Ok(owned) => owned,
        Err(result) => {
            destroy_native_reports(instance, &native, allocator);
            return result;
        }
    };
    let pointer = owned.as_mut_ptr();
    // SAFETY: Allocation succeeded and every field is written exactly once;
    // no fallible operation can interrupt initialization.
    unsafe {
        core::ptr::addr_of_mut!((*pointer).callback).write(create_info.pfnCallback);
        core::ptr::addr_of_mut!((*pointer).flags).write(create_info.flags);
        core::ptr::addr_of_mut!((*pointer).user_data).write(create_info.pUserData);
        core::ptr::addr_of_mut!((*pointer).icd_handles).write(native);
        core::ptr::addr_of_mut!((*pointer).allocator).write(object_allocator);
    }
    // SAFETY: Every `DebugReport` field was initialized above.
    let owned = unsafe { owned.assume_init() };
    let address = owned.as_ptr() as usize;
    let mut state = instance.debug_messengers.lock();
    if state.callbacks.try_reserve(1).is_err() {
        drop(state);
        destroy_native_reports(instance, &owned.icd_handles, allocator);
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    instance.set_has_debug_callbacks(true);
    state.callbacks.push(DebugCallback::Report(owned));
    drop(state);
    // SAFETY: The caller supplies writable output storage.
    unsafe { callback.write(VkDebugReportCallbackEXT(address as u64)) };
    VkResult::SUCCESS
}

/// Destroys a loader debug-report callback and its native objects.
///
/// # Safety
///
/// Arguments must satisfy `vkDestroyDebugReportCallbackEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn terminator_destroy_debug_report_callback(
    instance: VkInstance,
    callback: VkDebugReportCallbackEXT,
    allocator: *const VkAllocationCallbacks<'_>,
) {
    if callback == VkDebugReportCallbackEXT::NULL {
        return;
    }
    // SAFETY: The terminator receives a live loader instance.
    let Some(instance) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return;
    };
    let key = callback.0 as usize;
    let owned = {
        let mut state = instance.debug_messengers.lock();
        let Some(index) = state.callbacks.iter().position(|entry| {
            matches!(entry, DebugCallback::Report(report) if report.as_ptr() as usize == key)
        }) else {
            return;
        };
        let DebugCallback::Report(owned) = state.callbacks.remove(index) else {
            unreachable!();
        };
        instance.set_has_debug_callbacks(!state.callbacks.is_empty());
        owned
    };
    destroy_native_reports(instance, &owned.icd_handles, allocator);
}

/// Delivers a manual debug-report message once through loader callbacks.
///
/// # Safety
///
/// Arguments must satisfy `vkDebugReportMessageEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn terminator_debug_report_message(
    instance: VkInstance,
    flags: VkDebugReportFlagsEXT,
    object_type: VkDebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    layer_prefix: *const core::ffi::c_char,
    message: *const core::ffi::c_char,
) {
    // SAFETY: The terminator receives a live loader instance.
    let Some(instance) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return;
    };
    // Upstream first forwards manual debug-report messages to every active ICD,
    // then invokes the loader-owned report and debug-utils callbacks once.
    for (_, icd) in instance.active_icds() {
        if let Some(report) = icd.dispatch.vkDebugReportMessageEXT {
            // SAFETY: The native instance and remaining arguments retain the
            // public entry point's Vulkan contracts.
            unsafe {
                report(
                    icd.handle,
                    flags,
                    object_type,
                    object,
                    location,
                    message_code,
                    layer_prefix,
                    message,
                );
            };
        }
    }
    instance.submit_debug_report(
        flags,
        object_type,
        object,
        location,
        message_code,
        layer_prefix,
        message,
    );
}

pub(crate) fn destroy_all(instance: &LoaderInstance, _allocator: *const VkAllocationCallbacks<'_>) {
    let callbacks = {
        let mut state = instance.debug_messengers.lock();
        let callbacks = core::mem::take(&mut state.callbacks);
        instance.set_has_debug_callbacks(false);
        callbacks
    };
    for callback in callbacks {
        match callback {
            DebugCallback::Messenger(messenger) => destroy_native(
                instance,
                &messenger.icd_handles,
                forced_destroy_allocator(messenger.allocator.as_ref()),
            ),
            DebugCallback::Report(report) => destroy_native_reports(
                instance,
                &report.icd_handles,
                forced_destroy_allocator(report.allocator.as_ref()),
            ),
        }
    }
    instance
        .debug_messengers
        .lock()
        .release_allocations(instance);
}

/// Destroys the native debug objects owned by one ICD before that ICD is
/// retired, while retaining the loader-visible objects for the application.
pub(crate) fn destroy_icd_objects(instance: &LoaderInstance, icd_index: usize) {
    let Some(icd) = instance.icds.get(icd_index) else {
        return;
    };
    let mut state = instance.debug_messengers.lock();
    for callback in &mut state.callbacks {
        match callback {
            DebugCallback::Messenger(messenger) => {
                let allocator = forced_destroy_allocator(messenger.allocator.as_ref());
                let Some(handle) = messenger.icd_handles.get_mut(icd_index) else {
                    continue;
                };
                if *handle != VkDebugUtilsMessengerEXT::NULL
                    && let Some(destroy) = icd.dispatch.vkDestroyDebugUtilsMessengerEXT
                {
                    // SAFETY: This native object belongs to the retiring ICD.
                    unsafe { destroy(icd.handle, *handle, allocator) };
                    *handle = VkDebugUtilsMessengerEXT::NULL;
                }
            }
            DebugCallback::Report(report) => {
                let allocator = forced_destroy_allocator(report.allocator.as_ref());
                let Some(handle) = report.icd_handles.get_mut(icd_index) else {
                    continue;
                };
                if *handle != VkDebugReportCallbackEXT::NULL
                    && let Some(destroy) = icd.dispatch.vkDestroyDebugReportCallbackEXT
                {
                    // SAFETY: This native object belongs to the retiring ICD.
                    unsafe { destroy(icd.handle, *handle, allocator) };
                    *handle = VkDebugReportCallbackEXT::NULL;
                }
            }
        }
    }
    if let Some(allocation) = state.messenger_icd_allocations.get_mut(icd_index) {
        allocation.release(instance.allocator());
    }
}

#[cold]
#[inline(never)]
fn destroy_native_reports(
    instance: &LoaderInstance,
    handles: &[VkDebugReportCallbackEXT],
    allocator: *const VkAllocationCallbacks<'_>,
) {
    debug_assert_eq!(instance.icds.len(), handles.len());
    for (icd, handle) in instance.icds.iter().zip(handles) {
        if *handle != VkDebugReportCallbackEXT::NULL
            && let Some(destroy) = icd.dispatch.vkDestroyDebugReportCallbackEXT
        {
            // SAFETY: Each native handle belongs to this ICD and allocator.
            unsafe { destroy(icd.handle, *handle, allocator) };
        }
    }
}

#[cold]
#[inline(never)]
fn destroy_native(
    instance: &LoaderInstance,
    handles: &[VkDebugUtilsMessengerEXT],
    allocator: *const VkAllocationCallbacks<'_>,
) {
    debug_assert_eq!(instance.icds.len(), handles.len());
    for (icd, handle) in instance.icds.iter().zip(handles) {
        if *handle != VkDebugUtilsMessengerEXT::NULL
            && let Some(destroy) = icd.dispatch.vkDestroyDebugUtilsMessengerEXT
        {
            // SAFETY: Each native handle belongs to this ICD and the allocator
            // must match the corresponding public destroy call.
            unsafe { destroy(icd.handle, *handle, allocator) };
        }
    }
}

/// Dispatches debug-report callback creation through active layers.
///
/// # Safety
///
/// Arguments must satisfy `vkCreateDebugReportCallbackEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn vkCreateDebugReportCallbackEXT(
    instance: VkInstance,
    create_info: *const VkDebugReportCallbackCreateInfoEXT<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    callback: *mut VkDebugReportCallbackEXT,
) -> VkResult {
    // SAFETY: A non-null instance must identify a live loader instance.
    let Some(loader) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if loader.layers.is_empty() {
        // SAFETY: Forwarded from this entry point's contract.
        return unsafe {
            terminator_create_debug_report_callback(instance, create_info, allocator, callback)
        };
    }
    // SAFETY: The stable table was populated from the active top layer.
    let dispatch = unsafe { &*loader.dispatch() };
    let Some(create) = dispatch.vkCreateDebugReportCallbackEXT else {
        return VkResult::ERROR_EXTENSION_NOT_PRESENT;
    };
    // SAFETY: The arguments retain their application-provided contracts.
    unsafe { create(instance, create_info, allocator, callback) }
}

/// Dispatches debug-report callback destruction through active layers.
///
/// # Safety
///
/// Arguments must satisfy `vkDestroyDebugReportCallbackEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn vkDestroyDebugReportCallbackEXT(
    instance: VkInstance,
    callback: VkDebugReportCallbackEXT,
    allocator: *const VkAllocationCallbacks<'_>,
) {
    // SAFETY: A non-null instance must identify a live loader instance.
    let Some(loader) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return;
    };
    if loader.layers.is_empty() {
        // SAFETY: Forwarded from this entry point's contract.
        unsafe { terminator_destroy_debug_report_callback(instance, callback, allocator) };
        return;
    }
    // SAFETY: The stable table was populated from the active top layer.
    let dispatch = unsafe { &*loader.dispatch() };
    if let Some(destroy) = dispatch.vkDestroyDebugReportCallbackEXT {
        // SAFETY: The arguments retain their application-provided contracts.
        unsafe { destroy(instance, callback, allocator) };
    }
}

/// Dispatches a manual debug-report message through active layers.
///
/// # Safety
///
/// Arguments must satisfy `vkDebugReportMessageEXT`'s Vulkan contract.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "system" fn vkDebugReportMessageEXT(
    instance: VkInstance,
    flags: VkDebugReportFlagsEXT,
    object_type: VkDebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    layer_prefix: *const core::ffi::c_char,
    message: *const core::ffi::c_char,
) {
    // SAFETY: A non-null instance must identify a live loader instance.
    let Some(loader) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return;
    };
    if loader.layers.is_empty() {
        // SAFETY: Forwarded from this entry point's contract.
        unsafe {
            terminator_debug_report_message(
                instance,
                flags,
                object_type,
                object,
                location,
                message_code,
                layer_prefix,
                message,
            );
        };
        return;
    }
    // SAFETY: The stable table was populated from the active top layer.
    let dispatch = unsafe { &*loader.dispatch() };
    if let Some(report) = dispatch.vkDebugReportMessageEXT {
        // SAFETY: The arguments retain their application-provided contracts.
        unsafe {
            report(
                instance,
                flags,
                object_type,
                object,
                location,
                message_code,
                layer_prefix,
                message,
            );
        };
    }
}

/// Dispatches creation through active layers and then the loader terminator.
///
/// # Safety
///
/// Arguments must satisfy `vkCreateDebugUtilsMessengerEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn vkCreateDebugUtilsMessengerEXT(
    instance: VkInstance,
    create_info: *const VkDebugUtilsMessengerCreateInfoEXT<'_>,
    allocator: *const VkAllocationCallbacks<'_>,
    messenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult {
    // SAFETY: A non-null instance argument must identify a live loader instance.
    let Some(loader) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if loader.layers.is_empty() {
        // SAFETY: Forwarded from this entry point's contract.
        return unsafe {
            terminator_create_debug_utils_messenger(instance, create_info, allocator, messenger)
        };
    }
    // SAFETY: The stable table was populated from the active top layer.
    let dispatch = unsafe { &*loader.dispatch() };
    let Some(create) = dispatch.vkCreateDebugUtilsMessengerEXT else {
        return VkResult::ERROR_EXTENSION_NOT_PRESENT;
    };
    // SAFETY: The arguments retain their application-provided contracts.
    unsafe { create(instance, create_info, allocator, messenger) }
}

/// Dispatches destruction through active layers and then the loader terminator.
///
/// # Safety
///
/// Arguments must satisfy `vkDestroyDebugUtilsMessengerEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn vkDestroyDebugUtilsMessengerEXT(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    allocator: *const VkAllocationCallbacks<'_>,
) {
    // SAFETY: A non-null instance argument must identify a live loader instance.
    let Some(loader) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return;
    };
    if loader.layers.is_empty() {
        // SAFETY: Forwarded from this entry point's contract.
        unsafe { terminator_destroy_debug_utils_messenger(instance, messenger, allocator) };
        return;
    }
    // SAFETY: The stable table was populated from the active top layer.
    let dispatch = unsafe { &*loader.dispatch() };
    if let Some(destroy) = dispatch.vkDestroyDebugUtilsMessengerEXT {
        // SAFETY: The arguments retain their application-provided contracts.
        unsafe { destroy(instance, messenger, allocator) };
    }
}

/// Dispatches an application message through layers and then the terminator.
///
/// # Safety
///
/// Arguments must satisfy `vkSubmitDebugUtilsMessageEXT`'s Vulkan contract.
pub(crate) unsafe extern "system" fn vkSubmitDebugUtilsMessageEXT(
    instance: VkInstance,
    severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message_types: VkDebugUtilsMessageTypeFlagsEXT,
    callback_data: *const VkDebugUtilsMessengerCallbackDataEXT<'_>,
) {
    // SAFETY: A non-null instance argument must identify a live loader instance.
    let Some(loader) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return;
    };
    if loader.layers.is_empty() {
        // SAFETY: Forwarded from this entry point's contract.
        unsafe {
            terminator_submit_debug_utils_message(instance, severity, message_types, callback_data);
        };
        return;
    }
    // SAFETY: The stable table was populated from the active top layer.
    let dispatch = unsafe { &*loader.dispatch() };
    if let Some(submit) = dispatch.vkSubmitDebugUtilsMessageEXT {
        // SAFETY: The arguments retain their application-provided contracts.
        unsafe { submit(instance, severity, message_types, callback_data) };
    }
}

#[cfg(test)]
mod tests {
    use super::{debug_report_flags, debug_utils_flags};
    use vk::{
        VkDebugReportFlagBitsEXT as Report, VkDebugUtilsMessageSeverityFlagBitsEXT as Severity,
        VkDebugUtilsMessageTypeFlagBitsEXT as Type,
    };

    #[test]
    fn messenger_storage_growth_recovers_from_every_rust_allocation_failure() {
        crate::allocation::fault::sweep_operation(|| {
            let mut state = super::DebugMessengerState::new();
            let mut outcome = vk::VkResult::SUCCESS;
            for expected in 0..65 {
                let index = match state.reserve_messenger_storage(3, None) {
                    Ok(index) => index,
                    Err(error) => {
                        outcome = error;
                        // The failed attempt must leave the slot free and
                        // allow retrying the same object without reconstruction.
                        match state.reserve_messenger_storage(3, None) {
                            Ok(index) => index,
                            Err(_) => return vk::VkResult::ERROR_INITIALIZATION_FAILED,
                        }
                    }
                };
                if index != expected
                    || state
                        .messenger_icd_allocations
                        .iter()
                        .any(|allocation| allocation.entries <= index)
                {
                    return vk::VkResult::ERROR_INITIALIZATION_FAILED;
                }
            }
            outcome
        });
    }

    #[test]
    fn debug_utils_flags_convert_to_debug_report_flags_like_upstream() {
        assert_eq!(
            debug_report_flags(Severity::EMPTY, Type::GENERAL),
            Report::EMPTY
        );
        assert_eq!(
            debug_report_flags(Severity::VERBOSE, Type::GENERAL),
            Report::DEBUG
        );
        assert_eq!(
            debug_report_flags(Severity::INFO, Type::GENERAL),
            Report::INFORMATION
        );
        assert_eq!(
            debug_report_flags(Severity::WARNING, Type::GENERAL),
            Report::WARNING
        );
        assert_eq!(
            debug_report_flags(Severity::WARNING, Type::PERFORMANCE),
            Report::PERFORMANCE_WARNING
        );
        assert_eq!(
            debug_report_flags(Severity::ERROR, Type::GENERAL),
            Report::ERROR
        );

        // The C loader uses an error > warning > info > verbose priority chain.
        assert_eq!(
            debug_report_flags(Severity::INFO | Severity::ERROR, Type::GENERAL),
            Report::ERROR
        );
    }

    #[test]
    fn debug_report_flags_convert_to_debug_utils_flags_like_upstream() {
        assert_eq!(
            debug_utils_flags(Report::INFORMATION),
            (Severity::INFO, Type::GENERAL)
        );
        assert_eq!(
            debug_utils_flags(Report::WARNING),
            (Severity::WARNING, Type::GENERAL)
        );
        assert_eq!(
            debug_utils_flags(Report::PERFORMANCE_WARNING),
            (Severity::WARNING, Type::PERFORMANCE)
        );
        assert_eq!(
            debug_utils_flags(Report::ERROR),
            (Severity::ERROR, Type::GENERAL)
        );
        assert_eq!(
            debug_utils_flags(Report::DEBUG),
            (Severity::VERBOSE, Type::GENERAL)
        );
        assert_eq!(
            debug_utils_flags(Report::EMPTY),
            (Severity::EMPTY, Type::GENERAL)
        );

        // The legacy conversion uses info > warning > error > debug priority.
        assert_eq!(
            debug_utils_flags(Report::ERROR | Report::INFORMATION),
            (Severity::INFO, Type::GENERAL)
        );
    }
}
