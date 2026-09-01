//! Loader-owned dispatchable instance state.

use core::{ffi::c_void, mem::MaybeUninit};
use std::{ffi::CString, sync::LazyLock};

use crate::sync::Mutex;
use vk::{
    VK_API_VERSION_1_0, VkAllocationCallbacks, VkDebugReportFlagsEXT, VkDebugReportObjectTypeEXT,
    VkDebugUtilsMessageSeverityFlagBitsEXT, VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessengerCallbackDataEXT, VkDebugUtilsObjectNameInfoEXT, VkInstance, VkObjectType,
    VkPhysicalDevice,
};

use crate::{
    ExtensionSet, LayerInstanceDispatchTable,
    collections::HashMap,
    debug_messenger::{DebugCallback, DebugMessengerState},
    discovery::DeviceConfiguration,
    icd::IcdInstance,
    layer::{self, ActiveLayerProperty, LoadedLayer},
    surface::SurfaceState,
    unknown::{UnknownDeviceState, UnknownPhysicalDeviceState},
};

const INSTANCE_MAGIC: u64 = 0x10AD_ED01_0110_ADED;
const PHYSICAL_DEVICE_MAGIC: u64 = 0x10AD_ED02_0210_ADED;
const PHYSICAL_DEVICE_TRAMPOLINE_MAGIC: u64 = 0x10AD_ED03_0310_ADED;

static INSTANCES: LazyLock<Mutex<HashMap<usize, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::default()));

#[repr(C)]
pub(crate) struct LoaderInstance {
    dispatch: *const LayerInstanceDispatchTable,
    chain_instance: VkInstance,
    magic: u64,
    dispatch_table: Box<MaybeUninit<LayerInstanceDispatchTable>>,
    pub(crate) api_version: u32,
    pub(crate) enabled_extensions: ExtensionSet,
    pub(crate) pending_icds: Option<Box<[crate::ScannedIcdRecord]>>,
    pub(crate) icds: Box<[IcdInstance]>,
    pub(crate) layers: Box<[LoadedLayer]>,
    pub(crate) active_layer_properties: Box<[ActiveLayerProperty]>,
    pub(crate) enabled_layer_names: Box<[CString]>,
    pub(crate) device_configurations: Option<Box<[DeviceConfiguration]>>,
    allocator: Option<VkAllocationCallbacks<'static>>,
    pub(crate) physical_devices: Mutex<PhysicalDeviceState>,
    pub(crate) surfaces: Mutex<HashMap<usize, SurfaceState>>,
    pub(crate) debug_messengers: Mutex<DebugMessengerState>,
    pub(crate) unknown_physical_devices: Mutex<UnknownPhysicalDeviceState>,
    pub(crate) unknown_devices: Mutex<UnknownDeviceState>,
}

#[derive(Default)]
pub(crate) struct PhysicalDeviceState {
    pub(crate) owned: HashMap<(usize, usize), Box<LoaderPhysicalDevice>>,
    pub(crate) trampolines: HashMap<usize, Box<LoaderPhysicalDeviceTrampoline>>,
    pub(crate) active: Box<[VkPhysicalDevice]>,
}

#[repr(C)]
pub(crate) struct LoaderPhysicalDevice {
    dispatch: *const LayerInstanceDispatchTable,
    instance: *const LoaderInstance,
    magic: u64,
    pub(crate) icd_index: usize,
    icd: *const IcdInstance,
    pub(crate) app_api_version: u32,
    pub(crate) native: VkPhysicalDevice,
    pub(crate) unknown_dispatch: *const core::sync::atomic::AtomicPtr<c_void>,
}

#[repr(C)]
pub(crate) struct LoaderPhysicalDeviceTrampoline {
    dispatch: *const LayerInstanceDispatchTable,
    instance: *const LoaderInstance,
    magic: u64,
    pub(crate) chain: VkPhysicalDevice,
    pub(crate) terminator: VkPhysicalDevice,
    pub(crate) unknown_dispatch: *const core::sync::atomic::AtomicPtr<c_void>,
}

impl LoaderInstance {
    pub(crate) unsafe fn internal_magic(handle: VkInstance) -> Option<u64> {
        let instance = handle.0.cast::<Self>();
        (!instance.is_null()).then(|| unsafe { (*instance).magic })
    }

    pub(crate) fn new(
        api_version: u32,
        enabled_extensions: ExtensionSet,
        scanned_icds: Box<[crate::ScannedIcdRecord]>,
        active_layers: layer::ActiveLayers,
        device_configurations: Option<Box<[DeviceConfiguration]>>,
        allocator: *const VkAllocationCallbacks<'_>,
    ) -> Box<Self> {
        let has_layers = !active_layers.loaded.is_empty();
        let mut dispatch_table = Box::<LayerInstanceDispatchTable>::new_uninit();
        let dispatch = dispatch_table.as_mut_ptr().cast_const();
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
        let mut instance = Box::new(Self {
            dispatch,
            chain_instance: VkInstance::NULL,
            magic: INSTANCE_MAGIC,
            dispatch_table,
            api_version: api_version.max(VK_API_VERSION_1_0),
            enabled_extensions,
            pending_icds: Some(scanned_icds),
            icds: Box::default(),
            layers: active_layers.loaded,
            active_layer_properties: active_layers.reported,
            enabled_layer_names: active_layers.requested,
            device_configurations,
            allocator,
            physical_devices: Mutex::new(PhysicalDeviceState::default()),
            surfaces: Mutex::new(HashMap::default()),
            debug_messengers: Mutex::new(DebugMessengerState::new()),
            unknown_physical_devices: Mutex::new(UnknownPhysicalDeviceState::new()),
            unknown_devices: Mutex::new(UnknownDeviceState::new()),
        });
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
        instance
    }

    pub(crate) fn register(instance: Box<Self>) {
        let key = instance.dispatch() as usize;
        let pointer = Box::into_raw(instance) as usize;
        let previous = INSTANCES.lock().insert(key, pointer);
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
        self.dispatch
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
        let pointer = *INSTANCES.lock().get(&(dispatch as usize))?;
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
        let pointer = instances.remove(&(dispatch as usize))?;
        if instances.is_empty() {
            *instances = HashMap::default();
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
    pub(crate) fn new(
        icd_index: usize,
        icd: &IcdInstance,
        instance: &LoaderInstance,
        app_api_version: u32,
        native: VkPhysicalDevice,
    ) -> Self {
        Self {
            dispatch: instance.dispatch(),
            instance: core::ptr::from_ref(instance),
            magic: PHYSICAL_DEVICE_MAGIC,
            icd_index,
            icd: core::ptr::from_ref(icd),
            app_api_version,
            native,
            unknown_dispatch: icd.unknown_physical_device_dispatch.as_ptr(),
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
        debug_assert!(!self.icd.is_null());
        // SAFETY: The pointer targets the owning instance's boxed ICD slice,
        // which outlives every physical-device wrapper stored by that instance.
        unsafe { &*self.icd }
    }

    pub(crate) fn instance(&self) -> &LoaderInstance {
        debug_assert!(!self.instance.is_null());
        // SAFETY: Physical-device wrappers are owned by this loader instance.
        unsafe { &*self.instance }
    }

    #[cfg(test)]
    pub(crate) fn test_stub(
        native: VkPhysicalDevice,
        unknown_dispatch: *const core::sync::atomic::AtomicPtr<c_void>,
    ) -> Self {
        Self {
            dispatch: core::ptr::null(),
            instance: core::ptr::null(),
            magic: PHYSICAL_DEVICE_MAGIC,
            icd_index: 0,
            icd: core::ptr::null(),
            app_api_version: 0,
            native,
            unknown_dispatch,
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
            dispatch: instance.dispatch(),
            instance,
            magic: PHYSICAL_DEVICE_TRAMPOLINE_MAGIC,
            chain,
            terminator,
            unknown_dispatch,
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
            dispatch: core::ptr::null(),
            instance: core::ptr::null(),
            magic: PHYSICAL_DEVICE_TRAMPOLINE_MAGIC,
            chain,
            terminator: VkPhysicalDevice::NULL,
            unknown_dispatch,
        }
    }
}
