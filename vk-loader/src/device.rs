//! Loader-owned dispatchable device state.

use alloc::ffi::CString;
use core::{
    ffi::{CStr, c_void},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicPtr, Ordering},
};
use std::sync::LazyLock;

use crate::sync::Mutex;
use vk::{
    PFN_vkEnumerateDeviceExtensionProperties, PFN_vkGetDeviceProcAddr, PFN_vkVoidFunction,
    VK_KHR_MAINTENANCE_5_EXTENSION_NAME, VkBaseInStructure, VkDevice, VkDeviceCreateInfo,
    VkExtensionProperties, VkPhysicalDevice, VkPhysicalDeviceMaintenance5FeaturesKHR, VkResult,
    VkStructureType,
};

use crate::{
    ExtensionSet, IcdDeviceTerminatorDispatchTable, LayerDeviceDispatchTable, collections::HashMap,
    erase_function, icd::IcdInstance, instance::LoaderInstance, vkGetDeviceProcAddr,
};

#[derive(Default)]
struct DeviceRegistry {
    owned: HashMap<usize, Box<LoaderDevice>>,
    aliases: HashMap<usize, usize>,
}

static DEVICES: LazyLock<Mutex<DeviceRegistry>> =
    LazyLock::new(|| Mutex::new(DeviceRegistry::default()));

/// Internal state for a device returned by a layer or ICD dispatch chain.
pub(crate) struct LoaderDevice {
    dispatch_table: LoaderDeviceDispatch,
    icd_terminator_dispatch: Option<Box<IcdDeviceTerminatorDispatchTable>>,
    pub(crate) chain_device: VkDevice,
    pub(crate) icd_device: VkDevice,
    get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    chain_get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    chain_dispatch_key: usize,
    instance: *const LoaderInstance,
    icd_index: usize,
    app_core_level: u16,
    ignore_newer_core_commands: bool,
    enabled_extensions: ExtensionSet,
}

#[repr(C)]
struct LoaderDeviceDispatchLayout {
    layer: LayerDeviceDispatchTable,
    unknown: [AtomicPtr<c_void>; crate::unknown::MAX_UNKNOWN_COMMANDS],
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "x86_64"
))]
pub(crate) const UNKNOWN_DEVICE_DISPATCH_OFFSET: usize =
    core::mem::offset_of!(LoaderDeviceDispatchLayout, unknown);

struct LoaderDeviceDispatch {
    storage: Box<MaybeUninit<LoaderDeviceDispatchLayout>>,
}

impl LoaderDeviceDispatch {
    fn new() -> Self {
        let mut storage = Box::<LoaderDeviceDispatchLayout>::new_uninit();
        // SAFETY: Null is the valid initial value for every optional command
        // pointer and AtomicPtr slot. The non-optional magic field and all
        // generated fields are populated before the device becomes externally
        // visible.
        unsafe { storage.as_mut_ptr().write_bytes(0, 1) };
        // Publish the loader magic before installing this stable table in the
        // ICD dispatchable.
        unsafe {
            core::ptr::addr_of_mut!((*storage.as_mut_ptr()).layer.magic)
                .write(crate::DEVICE_DISPATCH_MAGIC)
        };
        Self { storage }
    }

    fn layer(&self) -> &LayerDeviceDispatchTable {
        // SAFETY: `new` initializes the complete generated table before return.
        unsafe { &(*self.storage.as_ptr()).layer }
    }

    fn layer_mut(&mut self) -> &mut LayerDeviceDispatchTable {
        // SAFETY: Exclusive access permits mutation of the initialized prefix.
        unsafe { &mut (*self.storage.as_mut_ptr()).layer }
    }

    pub(crate) fn store_unknown(&self, index: usize, function: PFN_vkVoidFunction) {
        debug_assert!(index < crate::unknown::MAX_UNKNOWN_COMMANDS);
        let address = function.map_or(core::ptr::null_mut(), |function| {
            function as *const () as *mut c_void
        });
        // SAFETY: `new` zero-initialized every AtomicPtr slot and the allocation
        // remains stable for the device lifetime.
        unsafe { (*self.storage.as_ptr()).unknown[index].store(address, Ordering::Release) };
    }
}

impl Deref for LoaderDeviceDispatch {
    type Target = LayerDeviceDispatchTable;

    fn deref(&self) -> &Self::Target {
        self.layer()
    }
}

impl DerefMut for LoaderDeviceDispatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.layer_mut()
    }
}

// Vulkan permits device and child-dispatchable handles to be used from
// multiple threads. Mutation of this internal object is limited to creation
// and destruction, and registry ownership is synchronized by `DEVICES`.
unsafe impl Send for LoaderDevice {}
unsafe impl Sync for LoaderDevice {}

impl LoaderDevice {
    /// Creates loader-owned state for a device returned by an ICD.
    ///
    /// # Safety
    ///
    /// `native` must be a live, writable ICD dispatchable, `get_device_proc_addr`
    /// must belong to that ICD, and `instance` must outlive the device.
    pub(crate) unsafe fn new(
        native: VkDevice,
        get_device_proc_addr: PFN_vkGetDeviceProcAddr,
        instance: *const LoaderInstance,
        icd_index: usize,
        app_api_version: u32,
        ignore_newer_core_commands: bool,
        enabled_extensions: ExtensionSet,
    ) -> Box<Self> {
        let dispatch_table = LoaderDeviceDispatch::new();
        let mut device = Box::new(Self {
            dispatch_table,
            icd_terminator_dispatch: None,
            chain_device: native,
            icd_device: native,
            get_device_proc_addr,
            chain_get_device_proc_addr: get_device_proc_addr,
            chain_dispatch_key: 0,
            instance,
            icd_index,
            app_core_level: api_core_level(app_api_version),
            ignore_newer_core_commands,
            enabled_extensions,
        });
        // Upstream consumes the ICD loader-magic word immediately after
        // vkCreateDevice returns, before making any GDPA calls for the device.
        // SAFETY: A successfully-created ICD dispatchable has writable loader
        // data in its first word, as required by the loader/driver interface.
        unsafe { Self::set_dispatch(native, device.dispatch()) };
        // Retain the direct ICD functions used by loader terminators after the
        // native dispatchable has received its loader data, matching upstream.
        let instance_extensions = device.instance().enabled_extensions;
        device.icd_terminator_dispatch = Some(unsafe {
            IcdDeviceTerminatorDispatchTable::load_boxed(get_device_proc_addr, native, |name| {
                let Some(lookup) = crate::command_lookup(name) else {
                    return false;
                };
                crate::command_core_level(lookup.id) != 0
                    || crate::command_has_enabled_instance_extension(
                        lookup.id,
                        &instance_extensions,
                    )
                    || crate::command_has_enabled_device_extension(lookup.id, &enabled_extensions)
            })
        });
        device
    }

    pub(crate) fn register(device: Box<Self>) -> VkDevice {
        let handle = device.chain_device;
        let dispatch_key = device.dispatch() as usize;
        let mut devices = DEVICES.lock();
        let previous = devices.owned.insert(dispatch_key, device);
        let previous_alias = devices.aliases.insert(dispatch_key, dispatch_key);
        debug_assert!(previous.is_none());
        debug_assert!(previous_alias.is_none());
        drop(previous);
        drop(devices);
        crate::pending::set_created_device(dispatch_key);
        handle
    }

    pub(crate) fn dispatch(&self) -> *const LayerDeviceDispatchTable {
        core::ptr::from_ref(self.dispatch_table.layer())
    }

    pub(crate) fn store_unknown_dispatch(&self, index: usize, function: PFN_vkVoidFunction) {
        self.dispatch_table.store_unknown(index, function);
    }

    /// Installs this device's loader dispatch table in a dispatchable object.
    ///
    /// # Safety
    ///
    /// `object` must be a live, writable Vulkan dispatchable object associated
    /// with this device.
    pub(crate) unsafe fn set_object_dispatch(&self, object: *mut core::ffi::c_void) {
        // SAFETY: Forwarded from this function's object-storage contract.
        unsafe {
            object
                .cast::<*const LayerDeviceDispatchTable>()
                .write(self.dispatch())
        };
    }

    unsafe fn set_dispatch(handle: VkDevice, dispatch: *const LayerDeviceDispatchTable) {
        // SAFETY: The caller guarantees that `handle` is writable loader data.
        unsafe {
            handle
                .0
                .cast::<*const LayerDeviceDispatchTable>()
                .write(dispatch)
        };
    }

    /// Resolves a command directly from the owning ICD.
    ///
    pub(crate) fn resolve(&self, name: &CStr) -> PFN_vkVoidFunction {
        if let Some(lookup) = crate::command_lookup(name)
            && let Some(command) = self
                .icd_terminator_dispatch
                .as_deref()
                .and_then(|dispatch| crate::icd_device_terminator_proc_addr(dispatch, lookup.id))
        {
            return Some(command);
        }
        // SAFETY: The function and native device were obtained from the same ICD.
        unsafe { (self.get_device_proc_addr)(self.icd_device, name.as_ptr()) }
    }

    /// Resolves a command through the first active layer, or directly from the ICD.
    ///
    pub(crate) fn resolve_chain(&self, name: &CStr) -> PFN_vkVoidFunction {
        // SAFETY: The layer/ICD that created `chain_device` remains loaded.
        unsafe { (self.chain_get_device_proc_addr)(self.chain_device, name.as_ptr()) }
    }

    pub(crate) unsafe fn from_dispatch_key_mut<'a>(key: usize) -> Option<&'a mut Self> {
        let mut devices = DEVICES.lock();
        let canonical = devices.aliases.get(&key).copied()?;
        let device = devices
            .owned
            .get_mut(&canonical)
            .map(|device| core::ptr::from_mut(device.as_mut()))?;
        drop(devices);
        // SAFETY: The caller guarantees creation-time exclusive access and the
        // boxed allocation remains stable after releasing the registry lock.
        Some(unsafe { &mut *device })
    }

    pub(crate) unsafe fn set_chain(&mut self, handle: VkDevice, resolver: PFN_vkGetDeviceProcAddr) {
        // SAFETY: Creation is not yet externally visible, so the stable table
        // may be populated in place for the completed top-level chain.
        unsafe {
            LayerDeviceDispatchTable::load_into(self.dispatch_table.layer_mut(), resolver, handle)
        };
        let app_core_level = self.app_core_level;
        let ignore_newer_core_commands = self.ignore_newer_core_commands;
        let instance_extensions = self.instance().enabled_extensions;
        let device_extensions = self.enabled_extensions;
        self.dispatch_table.mask_unavailable(|id| {
            let core_level = crate::command_core_level(id);
            let extension_enabled =
                crate::command_has_enabled_instance_extension(id, &instance_extensions)
                    || crate::command_has_enabled_device_extension(id, &device_extensions);
            (core_level != 0 && (!ignore_newer_core_commands || core_level <= app_core_level))
                || extension_enabled
        });
        self.chain_device = handle;
        self.chain_get_device_proc_addr = resolver;
        self.chain_dispatch_key = unsafe { Self::dispatch_key(handle) }.unwrap_or(0);
        if self.chain_dispatch_key != 0 {
            let own_key = self.dispatch() as usize;
            let previous = DEVICES
                .lock()
                .aliases
                .insert(self.chain_dispatch_key, own_key);
            debug_assert!(previous.is_none() || previous == Some(own_key));
        }
        // Replace direct-ICD unknown slots with top-of-layer-chain targets.
        crate::unknown::initialize_device_dispatch(self);
    }

    pub(crate) unsafe fn from_handle<'a>(handle: VkDevice) -> Option<&'a Self> {
        let key = unsafe { Self::dispatch_key(handle) }?;
        let devices = DEVICES.lock();
        let canonical = devices.aliases.get(&key).copied()?;
        let device = devices
            .owned
            .get(&canonical)
            .map(|device| core::ptr::from_ref(device.as_ref()));
        let device = device?;
        drop(devices);
        // SAFETY: A live Vulkan handle cannot be destroyed concurrently under
        // the API's external-synchronization contract; the box is stable.
        Some(unsafe { &*device })
    }

    pub(crate) fn take_dispatch(dispatch: *const LayerDeviceDispatchTable) -> Option<Box<Self>> {
        let mut devices = DEVICES.lock();
        let canonical = devices.aliases.remove(&(dispatch as usize))?;
        let device = devices.owned.remove(&canonical)?;
        devices.aliases.remove(&canonical);
        if device.chain_dispatch_key != 0 {
            devices.aliases.remove(&device.chain_dispatch_key);
        }
        if devices.owned.is_empty() && devices.aliases.is_empty() {
            *devices = DeviceRegistry::default();
        }
        Some(device)
    }

    unsafe fn dispatch_key(handle: VkDevice) -> Option<usize> {
        if handle == VkDevice::NULL {
            return None;
        }
        // SAFETY: A live Vulkan dispatchable stores its dispatch pointer in the
        // first machine word by loader ABI contract.
        let dispatch = unsafe { handle.0.cast::<*const LayerDeviceDispatchTable>().read() };
        if dispatch.is_null() {
            return None;
        }
        Some(dispatch as usize)
    }

    pub(crate) fn loader_proc_addr() -> PFN_vkVoidFunction {
        erase_function(vkGetDeviceProcAddr as PFN_vkGetDeviceProcAddr)
    }

    pub(crate) fn instance(&self) -> &LoaderInstance {
        debug_assert!(!self.instance.is_null());
        // SAFETY: Vulkan requires the parent instance to outlive this device.
        unsafe { &*self.instance }
    }

    pub(crate) const fn icd_index(&self) -> usize {
        self.icd_index
    }

    pub(crate) const fn enabled_extensions(&self) -> &ExtensionSet {
        &self.enabled_extensions
    }

    pub(crate) fn icd_destroy_device(&self) -> Option<vk::PFN_vkDestroyDevice> {
        match self.icd_terminator_dispatch.as_deref() {
            Some(dispatch) => dispatch.vkDestroyDevice,
            None => None,
        }
    }
}

pub(crate) fn initialize_unknown_dispatches(instance: &LoaderInstance, index: usize, name: &CStr) {
    let devices = DEVICES.lock();
    let mut matching = Vec::new();
    if matching.try_reserve_exact(devices.owned.len()).is_err() {
        return;
    }
    matching.extend(
        devices
            .owned
            .values()
            .filter(|device| core::ptr::eq(device.instance(), instance))
            .map(|device| core::ptr::from_ref(device.as_ref())),
    );
    drop(devices);
    for device in matching {
        // SAFETY: Registry-owned device boxes are stable, and instance
        // destruction requires all child devices to have been destroyed.
        let device = unsafe { &*device };
        // SAFETY: The stored resolver and chain handle are live together.
        let function = device.resolve_chain(name);
        device.store_unknown_dispatch(index, function);
    }
}

const fn api_core_level(version: u32) -> u16 {
    let major = vk::VK_API_VERSION_MAJOR(version) as u16;
    let minor = vk::VK_API_VERSION_MINOR(version) as u16;
    debug_assert!(major < 64 && minor < 1024);
    (major << 10) | minor
}

/// Returns whether maintenance5 requests strict device-command version checks.
///
/// # Safety
///
/// Every pointer reachable from `create_info` must satisfy `VkDeviceCreateInfo`'s
/// Vulkan validity requirements.
pub(crate) unsafe fn maintenance5_version_checks(create_info: &VkDeviceCreateInfo<'_>) -> bool {
    // Both the extension and feature must be enabled, matching the reference loader.
    if !unsafe { extension_enabled(create_info, VK_KHR_MAINTENANCE_5_EXTENSION_NAME) } {
        return false;
    }
    let mut next = create_info.pNext.cast::<VkBaseInStructure<'_>>();
    while !next.is_null() {
        // SAFETY: The caller guarantees a valid pNext chain.
        let structure = unsafe { &*next };
        if structure.sType == VkStructureType::PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR {
            // SAFETY: `sType` identifies the concrete structure layout.
            let features = unsafe { &*next.cast::<VkPhysicalDeviceMaintenance5FeaturesKHR<'_>>() };
            return features.maintenance5 != 0;
        }
        next = structure.pNext;
    }
    false
}

unsafe fn extension_enabled(create_info: &VkDeviceCreateInfo<'_>, name: &CStr) -> bool {
    if create_info.ppEnabledExtensionNames.is_null() {
        return false;
    }
    for index in 0..create_info.enabledExtensionCount as usize {
        // SAFETY: Vulkan requires an array of `enabledExtensionCount` live pointers.
        let extension = unsafe { create_info.ppEnabledExtensionNames.add(index).read() };
        if !extension.is_null()
            // SAFETY: Each enabled extension name is NUL-terminated by contract.
            && unsafe { CStr::from_ptr(extension) } == name
        {
            return true;
        }
    }
    false
}

/// Validates requested device extensions against an ICD's advertised set.
///
/// # Safety
///
/// `physical_device` must belong to `icd`, and every enabled-extension name in
/// `create_info` must satisfy Vulkan's string-array contract.
pub(crate) unsafe fn validate_and_filter_device_extensions(
    icd: &IcdInstance,
    physical_device: VkPhysicalDevice,
    create_info: &VkDeviceCreateInfo<'_>,
    layer_extensions: &[CString],
) -> Result<Box<[*const core::ffi::c_char]>, VkResult> {
    if create_info.enabledExtensionCount == 0 {
        return Ok(Box::default());
    }
    if create_info.ppEnabledExtensionNames.is_null() {
        return Err(VkResult::ERROR_INITIALIZATION_FAILED);
    }
    let enumerate: PFN_vkEnumerateDeviceExtensionProperties = icd
        .dispatch
        .vkEnumerateDeviceExtensionProperties
        .ok_or(VkResult::ERROR_INITIALIZATION_FAILED)?;
    let mut count = 0;
    // SAFETY: The physical device belongs to this ICD and count is writable.
    let result = unsafe {
        enumerate(
            physical_device,
            core::ptr::null(),
            &raw mut count,
            core::ptr::null_mut(),
        )
    };
    if result != VkResult::SUCCESS {
        return Err(result);
    }
    let capacity = usize::try_from(count).map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut properties = Box::<[VkExtensionProperties]>::new_uninit_slice(capacity);
    let mut returned_count = count;
    // SAFETY: The storage contains `capacity` writable entries.
    let result = unsafe {
        enumerate(
            physical_device,
            core::ptr::null(),
            &raw mut returned_count,
            properties.as_mut_ptr().cast(),
        )
    };
    if result != VkResult::SUCCESS && result != VkResult::INCOMPLETE {
        return Err(result);
    }
    let initialized = usize::try_from(returned_count)
        .unwrap_or(usize::MAX)
        .min(capacity);
    let mut icd_names = Vec::new();
    icd_names
        .try_reserve_exact(create_info.enabledExtensionCount as usize)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for index in 0..create_info.enabledExtensionCount as usize {
        // SAFETY: The caller provides `enabledExtensionCount` live pointers.
        let requested = unsafe { create_info.ppEnabledExtensionNames.add(index).read() };
        if requested.is_null() {
            return Err(VkResult::ERROR_INITIALIZATION_FAILED);
        }
        // SAFETY: Each requested extension is NUL-terminated by contract.
        let requested = unsafe { CStr::from_ptr(requested) }.to_bytes();
        let supported_by_layer = layer_extensions
            .iter()
            .any(|extension| extension.as_bytes() == requested);
        let supported_by_icd = properties[..initialized].iter().any(|property| {
            // SAFETY: The ICD reported these leading entries as initialized.
            let property = unsafe { property.assume_init_ref() };
            let bytes = property.extensionName.as_slice();
            let end = bytes
                .iter()
                .position(|byte| *byte == 0)
                .unwrap_or(bytes.len());
            requested.len() == end
                && requested
                    .iter()
                    .zip(&bytes[..end])
                    .all(|(left, right)| *left == *right as u8)
        });
        if !supported_by_layer && !supported_by_icd {
            return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
        }
        if supported_by_icd {
            icd_names.push(unsafe { create_info.ppEnabledExtensionNames.add(index).read() });
        }
    }
    Ok(icd_names.into_boxed_slice())
}
