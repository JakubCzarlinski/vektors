//! Loader-owned dispatchable instance state.

use alloc::{ffi::CString, vec::Vec};
use core::{
    ffi::c_void,
    mem::MaybeUninit,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering as AtomicOrdering},
};
use std::sync::LazyLock;

use crate::sync::Mutex;
use vk::{
    VK_API_VERSION_1_0, VkAllocationCallbacks, VkDebugReportFlagsEXT, VkDebugReportObjectTypeEXT,
    VkDebugUtilsMessageSeverityFlagBitsEXT, VkDebugUtilsMessageTypeFlagBitsEXT,
    VkDebugUtilsMessageTypeFlagsEXT, VkDebugUtilsMessengerCallbackDataEXT,
    VkDebugUtilsObjectNameInfoEXT, VkInstance, VkObjectType, VkPhysicalDevice, VkResult,
};

use crate::{
    ExtensionSet, LayerInstanceDispatchTable,
    allocation::{try_box, try_box_uninit},
    collections::HashMap,
    debug_messenger::{DebugCallback, DebugMessengerState},
    discovery::DeviceConfiguration,
    generated::EmulatedCommand,
    icd::IcdInstance,
    layer::{self, ActiveLayerProperty, LoadedLayer},
    surface::SurfaceState,
    unknown::{UnknownDeviceState, UnknownPhysicalDeviceState},
};

const INSTANCE_MAGIC: u64 = 0x10AD_ED01_0110_ADED;
const PHYSICAL_DEVICE_MAGIC: u64 = 0x10AD_ED02_0210_ADED;
const PHYSICAL_DEVICE_TRAMPOLINE_MAGIC: u64 = 0x10AD_ED03_0310_ADED;

#[derive(Default)]
struct InstanceRegistry {
    owned: HashMap<usize, usize>,
    reservations: usize,
}

static INSTANCES: LazyLock<Mutex<InstanceRegistry>> =
    LazyLock::new(|| Mutex::new(InstanceRegistry::default()));

struct InstanceRegistrationReservation {
    active: bool,
}

impl InstanceRegistrationReservation {
    fn new() -> Result<Self, VkResult> {
        let mut registry = INSTANCES.lock();
        let additional = registry
            .reservations
            .checked_add(1)
            .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        registry
            .owned
            .try_reserve(additional)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        registry.reservations = additional;
        Ok(Self { active: true })
    }
}

impl Drop for InstanceRegistrationReservation {
    fn drop(&mut self) {
        if self.active {
            let mut registry = INSTANCES.lock();
            debug_assert!(registry.reservations != 0);
            registry.reservations -= 1;
        }
    }
}

#[repr(C)]
pub(crate) struct LoaderInstance {
    // Vulkan's dispatchable-handle ABI requires this to remain the first field.
    dispatch: NonNull<LayerInstanceDispatchTable>,
    // Hot handle-validation and dispatch state.
    magic: u64,
    chain_instance: VkInstance,
    pub(crate) api_version: u32,
    pub(crate) enabled_extensions: ExtensionSet,
    pub(crate) icds: Vec<IcdInstance>,
    pub(crate) layers: Box<[LoadedLayer]>,
    pub(crate) physical_devices: Mutex<PhysicalDeviceState>,
    pub(crate) unknown_physical_devices: Mutex<UnknownPhysicalDeviceState>,
    pub(crate) unknown_devices: Mutex<UnknownDeviceState>,
    // Less frequently accessed metadata and object registries.
    dispatch_table: Box<MaybeUninit<LayerInstanceDispatchTable>>,
    pub(crate) pending_icds: Option<Vec<crate::ScannedIcdRecord>>,
    pub(crate) active_layer_properties: Box<[ActiveLayerProperty]>,
    pub(crate) enabled_layer_names: Box<[CString]>,
    pub(crate) device_configurations: Option<Box<[DeviceConfiguration]>>,
    allocator: Option<VkAllocationCallbacks<'static>>,
    pub(crate) surfaces: Mutex<HashMap<usize, SurfaceState>>,
    pub(crate) debug_messengers: Mutex<DebugMessengerState>,
    has_debug_callbacks: AtomicBool,
    registration: InstanceRegistrationReservation,
}

#[derive(Default)]
pub(crate) struct PhysicalDeviceState {
    pub(crate) owned: HashMap<(usize, usize), Box<LoaderPhysicalDevice>>,
    pub(crate) trampolines: HashMap<usize, Box<LoaderPhysicalDeviceTrampoline>>,
    pub(crate) active: Vec<VkPhysicalDevice>,
}

#[repr(C)]
pub(crate) struct LoaderPhysicalDevice {
    dispatch: NonNull<LayerInstanceDispatchTable>,
    instance: NonNull<LoaderInstance>,
    magic: u64,
    pub(crate) native: VkPhysicalDevice,
    pub(crate) unknown_dispatch: NonNull<core::sync::atomic::AtomicPtr<c_void>>,
    icd: NonNull<IcdInstance>,
    pub(crate) icd_index: usize,
    pub(crate) app_api_version: u32,
}

#[repr(C)]
pub(crate) struct LoaderPhysicalDeviceTrampoline {
    dispatch: NonNull<LayerInstanceDispatchTable>,
    instance: NonNull<LoaderInstance>,
    magic: u64,
    pub(crate) chain: VkPhysicalDevice,
    pub(crate) terminator: VkPhysicalDevice,
    pub(crate) unknown_dispatch: NonNull<core::sync::atomic::AtomicPtr<c_void>>,
}

const _: () = assert!(core::mem::offset_of!(LoaderInstance, dispatch) == 0);
const _: () = assert!(core::mem::offset_of!(LoaderPhysicalDevice, dispatch) == 0);
const _: () = assert!(core::mem::offset_of!(LoaderPhysicalDeviceTrampoline, dispatch) == 0);

impl LoaderInstance {
    pub(crate) unsafe fn internal_magic(handle: VkInstance) -> Option<u64> {
        let instance = handle.0.cast::<Self>();
        (!instance.is_null()).then(|| unsafe { (*instance).magic })
    }

    pub(crate) fn new(
        api_version: u32,
        enabled_extensions: ExtensionSet,
        scanned_icds: Vec<crate::ScannedIcdRecord>,
        active_layers: layer::ActiveLayers,
        device_configurations: Option<Box<[DeviceConfiguration]>>,
        allocator: *const VkAllocationCallbacks<'_>,
    ) -> Result<Box<Self>, VkResult> {
        let has_layers = !active_layers.loaded.is_empty();
        let mut dispatch_table = try_box_uninit::<LayerInstanceDispatchTable>()?;
        // SAFETY: The boxed table has stable, non-null storage.
        let dispatch = unsafe { NonNull::new_unchecked(dispatch_table.as_mut_ptr()) };
        let allocator = if allocator.is_null() {
            None
        } else {
            // SAFETY: Vulkan requires the allocation callbacks to remain valid
            // until the matching instance destruction call.
            Some(unsafe {
                core::mem::transmute::<VkAllocationCallbacks<'_>, VkAllocationCallbacks<'static>>(
                    allocator.read(),
                )
            })
        };
        let unknown_physical_devices = UnknownPhysicalDeviceState::try_new()?;
        let registration = InstanceRegistrationReservation::new()?;
        let mut instance = try_box(Self {
            dispatch,
            magic: INSTANCE_MAGIC,
            chain_instance: VkInstance::NULL,
            api_version: api_version.max(VK_API_VERSION_1_0),
            enabled_extensions,
            icds: Vec::new(),
            layers: active_layers.loaded,
            physical_devices: Mutex::new(PhysicalDeviceState::default()),
            unknown_physical_devices: Mutex::new(unknown_physical_devices),
            unknown_devices: Mutex::new(UnknownDeviceState::new()),
            dispatch_table,
            pending_icds: Some(scanned_icds),
            active_layer_properties: active_layers.reported,
            enabled_layer_names: active_layers.requested,
            device_configurations,
            allocator,
            surfaces: Mutex::new(HashMap::default()),
            debug_messengers: Mutex::new(DebugMessengerState::new()),
            has_debug_callbacks: AtomicBool::new(false),
            registration,
        })
        .map_err(|(result, _instance)| result)?;
        let handle = instance.handle();
        instance.chain_instance = handle;
        if has_layers {
            let table = instance.dispatch_table.as_mut_ptr();
            // SAFETY: Every field except the manually-added GPDPA is an
            // `Option<fn>`, for which the all-zero representation is `None`.
            // GPDPA is written before the table can be observed by a layer.
            unsafe {
                table.write_bytes(0, 1);
                core::ptr::addr_of_mut!((*table).vk_layerGetPhysicalDeviceProcAddr)
                    .write(layer::terminator_get_physical_device_proc_addr);
            }
        } else {
            // SAFETY: The boxed table has stable, writable storage and all proc
            // address functions remain loaded for the instance lifetime.
            unsafe {
                LayerInstanceDispatchTable::load_into(
                    instance.dispatch_table.as_mut_ptr(),
                    layer::terminator_get_instance_proc_addr,
                    layer::terminator_get_physical_device_proc_addr,
                    handle,
                );
            };
        }
        Ok(instance)
    }

    pub(crate) fn register(mut instance: Box<Self>) {
        let key = instance.dispatch() as usize;
        let mut registry = INSTANCES.lock();
        debug_assert!(instance.registration.active);
        debug_assert!(registry.reservations != 0);
        registry.reservations -= 1;
        instance.registration.active = false;
        let pointer = Box::into_raw(instance) as usize;
        let previous = registry.owned.insert(key, pointer);
        debug_assert!(previous.is_none());
        let _ = previous;
    }

    pub(crate) fn handle(&self) -> VkInstance {
        VkInstance(core::ptr::from_ref(self).cast_mut().cast())
    }

    pub(crate) const fn chain_handle(&self) -> VkInstance {
        self.chain_instance
    }

    pub(crate) const fn dispatch(&self) -> *const LayerInstanceDispatchTable {
        self.dispatch.as_ptr()
    }

    pub(crate) fn allocator(&self) -> Option<&VkAllocationCallbacks<'static>> {
        self.allocator.as_ref()
    }

    pub(crate) fn forced_destroy_allocator(&self) -> *const VkAllocationCallbacks<'static> {
        let Some(callbacks) = self.allocator.as_ref() else {
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

    pub(crate) fn active_icds(&self) -> impl DoubleEndedIterator<Item = (usize, &IcdInstance)> {
        self.icds
            .iter()
            .enumerate()
            .filter(|(_, icd)| icd.is_active())
    }

    /// Reloads the stable dispatch table for the top of an instance chain.
    ///
    /// # Safety
    ///
    /// The proc-address functions and `handle` must belong to the live chain
    /// represented by this instance.
    pub(crate) unsafe fn load_dispatch(
        &mut self,
        gipa: vk::PFN_vkGetInstanceProcAddr,
        gpdpa: layer::GetPhysicalDeviceProcAddr,
        handle: VkInstance,
    ) {
        // SAFETY: Exclusive instance access prevents calls through the table
        // while its stable allocation is initialized in place.
        unsafe {
            LayerInstanceDispatchTable::load_into(
                self.dispatch_table.as_mut_ptr(),
                gipa,
                gpdpa,
                handle,
            );
        };
        self.chain_instance = handle;
    }

    pub(crate) unsafe fn from_handle<'a>(handle: VkInstance) -> Option<&'a Self> {
        if handle == VkInstance::NULL {
            return None;
        }
        // SAFETY: Vulkan instance handles are dispatchable handles.
        unsafe { Self::from_dispatchable(handle.0.cast()) }
    }

    pub(crate) unsafe fn from_dispatchable<'a>(handle: *mut c_void) -> Option<&'a Self> {
        if handle.is_null() {
            return None;
        }
        // SAFETY: Every live instance dispatchable stores its dispatch table in
        // the first word, including handles wrapped by layers.
        let dispatch = unsafe { handle.cast::<*const LayerInstanceDispatchTable>().read() };
        let pointer = *INSTANCES.lock().owned.get(&(dispatch as usize))?;
        // SAFETY: Registration retains this boxed allocation until destruction.
        let instance = unsafe { &*(pointer as *const Self) };
        (instance.magic == INSTANCE_MAGIC).then_some(instance)
    }

    pub(crate) unsafe fn from_internal_handle<'a>(handle: VkInstance) -> Option<&'a Self> {
        let instance = handle.0.cast::<Self>();
        if instance.is_null() {
            return None;
        }
        // SAFETY: This helper is only used for the loader's own pending handle.
        let instance = unsafe { &*instance };
        (instance.magic == INSTANCE_MAGIC).then_some(instance)
    }

    pub(crate) unsafe fn from_internal_handle_mut<'a>(handle: VkInstance) -> Option<&'a mut Self> {
        let instance = handle.0.cast::<Self>();
        if instance.is_null() {
            return None;
        }
        // SAFETY: This is used only while the pending instance is exclusively
        // owned by the synchronous layer-create chain.
        let instance = unsafe { &mut *instance };
        (instance.magic == INSTANCE_MAGIC).then_some(instance)
    }

    pub(crate) fn take_dispatch(dispatch: *const LayerInstanceDispatchTable) -> Option<Box<Self>> {
        let mut instances = INSTANCES.lock();
        let pointer = instances.owned.remove(&(dispatch as usize))?;
        if instances.owned.is_empty() && instances.reservations == 0 {
            instances.owned = HashMap::default();
        }
        drop(instances);
        // SAFETY: Registration stored the unique Box allocation and removal
        // makes this the sole reconstruction during destruction.
        Some(unsafe { Box::from_raw(pointer as *mut Self) })
    }

    pub(crate) fn submit_debug_message(
        &self,
        severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
        message_types: VkDebugUtilsMessageTypeFlagsEXT,
        callback_data: &VkDebugUtilsMessengerCallbackDataEXT<'_>,
    ) {
        let report_flags = crate::debug_messenger::debug_report_flags(severity, message_types);
        let (object_type, object) = if callback_data.objectCount == 0 {
            (VkDebugReportObjectTypeEXT::UNKNOWN, 0)
        } else {
            // SAFETY: A non-zero object count requires a readable object array.
            let object = unsafe { &*callback_data.pObjects };
            (
                crate::convert_core_object_to_debug_report_object(object.objectType),
                object.objectHandle,
            )
        };
        let state = self.debug_messengers.lock();
        for entry in state.callbacks.iter().rev() {
            match entry {
                DebugCallback::Messenger(messenger) => {
                    if !messenger.accepts(severity, message_types) {
                        continue;
                    }
                    let Some(callback) = messenger.callback else {
                        continue;
                    };
                    // SAFETY: Messenger creation retained the callback and
                    // user data; callback data remains live for this call.
                    unsafe {
                        callback(severity, message_types, callback_data, messenger.user_data)
                    };
                }
                DebugCallback::Report(report) => {
                    if !report.accepts(report_flags) {
                        continue;
                    }
                    let Some(callback) = report.callback else {
                        continue;
                    };
                    // SAFETY: Report creation retained the callback and user
                    // data; callback-data strings remain live for this call.
                    unsafe {
                        callback(
                            report_flags,
                            object_type,
                            object,
                            0,
                            callback_data.messageIdNumber,
                            callback_data.pMessageIdName,
                            callback_data.pMessage,
                            report.user_data,
                        )
                    };
                }
            }
        }
    }

    pub(crate) fn submit_loader_message(
        &self,
        severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
        message_types: VkDebugUtilsMessageTypeFlagsEXT,
        message: &core::ffi::CStr,
    ) {
        let object = VkDebugUtilsObjectNameInfoEXT {
            objectType: VkObjectType::INSTANCE,
            objectHandle: self.handle().0 as usize as u64,
            ..VkDebugUtilsObjectNameInfoEXT::DEFAULT
        };
        let callback_data = VkDebugUtilsMessengerCallbackDataEXT {
            pMessageIdName: c"Loader Message".as_ptr(),
            pMessage: message.as_ptr(),
            objectCount: 1,
            pObjects: core::ptr::from_ref(&object),
            ..VkDebugUtilsMessengerCallbackDataEXT::DEFAULT
        };
        self.submit_debug_message(severity, message_types, &callback_data);
    }

    /// Writes an ordinary loader diagnostic and mirrors it to debug callbacks.
    pub(crate) fn log_loader_message(
        &self,
        severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
        message_types: VkDebugUtilsMessageTypeFlagsEXT,
        message: &core::ffi::CStr,
    ) {
        crate::platform::write_loader_log(
            crate::platform::LogFilter::from_severity(severity),
            format_args!("{}", message.to_string_lossy()),
        );
        self.submit_loader_message(severity, message_types, message);
    }

    pub(crate) fn log_loader_message_text(
        &self,
        severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
        message_types: VkDebugUtilsMessageTypeFlagsEXT,
        message: core::fmt::Arguments<'_>,
    ) {
        let message = message.to_string();
        crate::platform::write_loader_log(
            crate::platform::LogFilter::from_severity(severity),
            format_args!("{message}"),
        );
        if let Ok(message) = CString::new(message) {
            self.submit_loader_message(severity, message_types, &message);
        }
    }

    pub(crate) fn set_has_debug_callbacks(&self, has_callbacks: bool) {
        self.has_debug_callbacks
            .store(has_callbacks, AtomicOrdering::Release);
    }

    #[inline]
    fn wants_loader_message(&self, severity: VkDebugUtilsMessageSeverityFlagBitsEXT) -> bool {
        crate::platform::loader_debug_filter_enabled(crate::platform::LogFilter::from_severity(
            severity,
        )) || self.has_debug_callbacks.load(AtomicOrdering::Acquire)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn submit_debug_report(
        &self,
        flags: VkDebugReportFlagsEXT,
        object_type: VkDebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        layer_prefix: *const core::ffi::c_char,
        message: *const core::ffi::c_char,
    ) {
        let (severity, message_types) = crate::debug_messenger::debug_utils_flags(flags);
        let object_info = VkDebugUtilsObjectNameInfoEXT {
            objectType: crate::convert_debug_report_object_to_core_object(object_type),
            objectHandle: object,
            ..VkDebugUtilsObjectNameInfoEXT::DEFAULT
        };
        let callback_data = VkDebugUtilsMessengerCallbackDataEXT {
            pMessageIdName: layer_prefix,
            messageIdNumber: message_code,
            pMessage: message,
            objectCount: 1,
            pObjects: core::ptr::from_ref(&object_info),
            ..VkDebugUtilsMessengerCallbackDataEXT::DEFAULT
        };
        let state = self.debug_messengers.lock();
        for entry in state.callbacks.iter().rev() {
            match entry {
                DebugCallback::Report(report) => {
                    if !report.accepts(flags) {
                        continue;
                    }
                    let Some(callback) = report.callback else {
                        continue;
                    };
                    // SAFETY: Report creation retained the callback and user
                    // data; strings remain live for this synchronous call.
                    unsafe {
                        callback(
                            flags,
                            object_type,
                            object,
                            location,
                            message_code,
                            layer_prefix,
                            message,
                            report.user_data,
                        )
                    };
                }
                DebugCallback::Messenger(messenger) => {
                    if !messenger.accepts(severity, message_types) {
                        continue;
                    }
                    let Some(callback) = messenger.callback else {
                        continue;
                    };
                    // SAFETY: Messenger creation retained the callback and
                    // user data; converted data remains live for this call.
                    unsafe {
                        callback(
                            severity,
                            message_types,
                            core::ptr::from_ref(&callback_data),
                            messenger.user_data,
                        )
                    };
                }
            }
        }
    }
}

impl LoaderPhysicalDevice {
    #[inline]
    pub(crate) fn log_icd_emulation(&self, command: EmulatedCommand) {
        if !self
            .instance()
            .wants_loader_message(VkDebugUtilsMessageSeverityFlagBitsEXT::INFO)
        {
            return;
        }
        self.log_icd_emulation_enabled(command);
    }

    #[cold]
    #[inline(never)]
    fn log_icd_emulation_enabled(&self, command: EmulatedCommand) {
        let library = self
            .icd()
            .library_path()
            .map_or_else(|| "".into(), |path| path.to_string_lossy());
        match command.diagnostic_legacy_name() {
            Some(legacy) => self.instance().log_loader_message_text(
                VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
                format_args!(
                    "{}: Emulating call in ICD \"{library}\" using {legacy}",
                    command.name(),
                ),
            ),
            None => self.instance().log_loader_message_text(
                VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
                format_args!("{}: Emulating call in ICD \"{library}\"", command.name()),
            ),
        }
    }

    pub(crate) fn new(
        icd_index: usize,
        icd: &IcdInstance,
        instance: &LoaderInstance,
        app_api_version: u32,
        native: VkPhysicalDevice,
    ) -> Self {
        Self {
            dispatch: instance.dispatch,
            instance: NonNull::from(instance),
            magic: PHYSICAL_DEVICE_MAGIC,
            native,
            // SAFETY: The ICD owns a stable, non-null unknown-command table.
            unknown_dispatch: unsafe {
                NonNull::new_unchecked(icd.unknown_physical_device_dispatch.as_ptr().cast_mut())
            },
            icd: NonNull::from(icd),
            icd_index,
            app_api_version,
        }
    }

    pub(crate) fn handle(&self) -> VkPhysicalDevice {
        VkPhysicalDevice(core::ptr::from_ref(self).cast_mut().cast())
    }

    pub(crate) unsafe fn from_handle<'a>(handle: VkPhysicalDevice) -> Option<&'a Self> {
        let device = handle.0.cast::<Self>();
        if device.is_null() {
            return None;
        }
        // SAFETY: Vulkan requires a non-null physical-device argument to be a
        // live dispatchable handle. Magic distinguishes loader object kinds.
        let device = unsafe { &*device };
        (device.magic == PHYSICAL_DEVICE_MAGIC).then_some(device)
    }

    /// Borrows the loader-owned handle passed through an internal terminator chain.
    ///
    /// # Safety
    ///
    /// `handle` must be the live terminator handle supplied by this loader to
    /// the layer chain for a physical device.
    pub(crate) unsafe fn from_terminator_handle<'a>(handle: VkPhysicalDevice) -> &'a Self {
        // SAFETY: The caller guarantees this is the loader-owned terminator object.
        unsafe { &*handle.0.cast::<Self>() }
    }

    pub(crate) fn icd(&self) -> &IcdInstance {
        // SAFETY: The pointer targets the owning instance's boxed ICD slice,
        // which outlives every physical-device wrapper stored by that instance.
        unsafe { self.icd.as_ref() }
    }

    pub(crate) fn instance(&self) -> &LoaderInstance {
        // SAFETY: Physical-device wrappers are owned by this loader instance.
        unsafe { self.instance.as_ref() }
    }

    #[cfg(test)]
    pub(crate) fn test_stub(
        native: VkPhysicalDevice,
        unknown_dispatch: *const core::sync::atomic::AtomicPtr<c_void>,
    ) -> Self {
        Self {
            dispatch: NonNull::dangling(),
            instance: NonNull::dangling(),
            magic: PHYSICAL_DEVICE_MAGIC,
            native,
            // SAFETY: Test callers pass a live dispatch-table allocation.
            unknown_dispatch: unsafe { NonNull::new_unchecked(unknown_dispatch.cast_mut()) },
            icd: NonNull::dangling(),
            icd_index: 0,
            app_api_version: 0,
        }
    }
}

impl LoaderPhysicalDeviceTrampoline {
    pub(crate) fn new(
        instance: &LoaderInstance,
        chain: VkPhysicalDevice,
        terminator: VkPhysicalDevice,
    ) -> Self {
        let unknown_dispatch = instance.unknown_physical_devices.lock().dispatch().as_ptr();
        Self {
            dispatch: instance.dispatch,
            instance: NonNull::from(instance),
            magic: PHYSICAL_DEVICE_TRAMPOLINE_MAGIC,
            chain,
            terminator,
            // SAFETY: `dispatch` returns the stable, non-null table backing the
            // instance's unknown-command state.
            unknown_dispatch: unsafe { NonNull::new_unchecked(unknown_dispatch.cast_mut()) },
        }
    }

    pub(crate) fn handle(&self) -> VkPhysicalDevice {
        VkPhysicalDevice(core::ptr::from_ref(self).cast_mut().cast())
    }

    pub(crate) unsafe fn from_handle<'a>(handle: VkPhysicalDevice) -> Option<&'a Self> {
        let device = handle.0.cast::<Self>();
        if device.is_null() {
            return None;
        }
        let device = unsafe { &*device };
        (device.magic == PHYSICAL_DEVICE_TRAMPOLINE_MAGIC).then_some(device)
    }

    #[cfg(test)]
    pub(crate) fn test_stub(
        chain: VkPhysicalDevice,
        unknown_dispatch: *const core::sync::atomic::AtomicPtr<c_void>,
    ) -> Self {
        Self {
            dispatch: NonNull::dangling(),
            instance: NonNull::dangling(),
            magic: PHYSICAL_DEVICE_TRAMPOLINE_MAGIC,
            chain,
            terminator: VkPhysicalDevice::NULL,
            // SAFETY: Test callers pass a live dispatch-table allocation.
            unknown_dispatch: unsafe { NonNull::new_unchecked(unknown_dispatch.cast_mut()) },
        }
    }
}
