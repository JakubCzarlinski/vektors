//! Loader-owned dispatchable device state.

mod snapshot;
use crate::LoaderPathExt;
use crate::{
    ExtensionSet, IcdDeviceTerminatorDispatchTable, LayerDeviceDispatchTable,
    allocation::{try_box, try_box_uninit},
    collections::{ErasedPointer, HashMap},
    emulation::find_input_chain,
    erase_function,
    icd::IcdInstance,
    instance::LoaderInstance,
    vkGetDeviceProcAddr,
};
use crate::{allocation, sync::GlobalLazyMutex, unknown};
use core::{
    ffi::{CStr, c_void},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::atomic::{AtomicPtr, Ordering},
};
use vk::{
    PFN_vkEnumerateDeviceExtensionProperties, PFN_vkGetDeviceProcAddr, PFN_vkVoidFunction,
    VK_KHR_MAINTENANCE_5_EXTENSION_NAME, VkDevice, VkDeviceCreateInfo, VkExtensionProperties,
    VkPhysicalDevice, VkPhysicalDeviceMaintenance5FeaturesKHR, VkResult, VkStructureType,
};

#[derive(Default)]
struct DeviceRegistry {
    owned: HashMap<usize, ErasedPointer>,
    aliases: HashMap<usize, ErasedPointer>,
    alias_reservations: usize,
}

impl Drop for DeviceRegistry {
    fn drop(&mut self) {
        for pointer in self.owned.values().copied() {
            // SAFETY: `owned` contains one raw Box owner for every registered device.
            drop(unsafe { pointer.into_box::<LoaderDevice>() });
        }
    }
}

impl DeviceRegistry {
    #[cold]
    #[inline(never)]
    fn reset_if_unused(&mut self) {
        if self.owned.is_empty() && self.aliases.is_empty() && self.alias_reservations == 0 {
            self.owned = HashMap::default();
            self.aliases = HashMap::default();
        }
    }
}

static DEVICE_SNAPSHOT: snapshot::Cache = snapshot::Cache::new();

static DEVICES: GlobalLazyMutex<DeviceRegistry> = GlobalLazyMutex::new(DeviceRegistry::default);

/// Zero-sized proof retained by a successfully constructed alias reservation.
struct DeviceRegistryReady;

impl DeviceRegistryReady {
    fn lock(_ready: &Self) -> impl DerefMut<Target = DeviceRegistry> + use<> {
        // SAFETY: Only a successful try_lock constructs this marker, and a live
        // reservation excludes library termination.
        unsafe { DEVICES.lock_initialized() }
    }
}

#[cfg(not(all(target_vendor = "apple", feature = "apple-static-loader")))]
pub(crate) unsafe fn destroy_device_registry_lock() {
    // SAFETY: The caller excludes loader entry points and live registry guards.
    unsafe { DEVICES.destroy() };
}

struct DeviceAliasReservation {
    active: bool,
    ready: DeviceRegistryReady,
}

impl DeviceAliasReservation {
    #[inline(never)]
    fn new() -> Result<Self, VkResult> {
        let mut devices = DEVICES.try_lock()?;
        let additional = devices
            .alias_reservations
            .checked_add(1)
            .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        devices
            .aliases
            .try_reserve(additional)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        devices.alias_reservations = additional;
        Ok(Self {
            active: true,
            ready: DeviceRegistryReady,
        })
    }

    fn insert(mut self, key: usize, canonical: usize) -> Option<ErasedPointer> {
        let mut devices = DeviceRegistryReady::lock(&self.ready);
        debug_assert!(devices.alias_reservations != 0);
        devices.alias_reservations -= 1;
        self.active = false;
        let alias = devices.aliases[&canonical];
        let previous = devices.aliases.insert(key, alias);
        DEVICE_SNAPSHOT.publish(canonical, key, alias.as_ptr());
        previous
    }
}

impl Drop for DeviceAliasReservation {
    fn drop(&mut self) {
        if self.active {
            let mut devices = DeviceRegistryReady::lock(&self.ready);
            debug_assert!(devices.alias_reservations != 0);
            devices.alias_reservations -= 1;
            devices.reset_if_unused();
        }
    }
}

/// Internal state for a device returned by a layer or ICD dispatch chain.
pub(crate) struct LoaderDevice {
    dispatch_table: LoaderDeviceDispatch,
    icd_terminator_dispatch: IcdDeviceTerminatorDispatchTable,
    pub(crate) chain_device: VkDevice,
    pub(crate) icd_device: VkDevice,
    get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    chain_get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    chain_dispatch_key: usize,
    instance: NonNull<LoaderInstance>,
    icd_index: usize,
    app_core_level: u16,
    ignore_newer_core_commands: bool,
    enabled_extensions: ExtensionSet,
}

#[repr(C)]
struct LoaderDeviceDispatchLayout {
    layer: LayerDeviceDispatchTable,
    unknown: [AtomicPtr<c_void>; unknown::MAX_UNKNOWN_COMMANDS],
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
    fn try_new() -> Result<Self, VkResult> {
        let mut storage = try_box_uninit::<LoaderDeviceDispatchLayout>()?;
        // SAFETY: Null is the valid initial value for every optional command
        // pointer and AtomicPtr slot. The non-optional magic field and all
        // generated fields are populated before the device becomes externally
        // visible.
        unsafe { storage.as_mut_ptr().write_bytes(0, 1) };
        // Publish the loader magic before installing this stable table in the
        // ICD dispatchable.
        unsafe {
            core::ptr::addr_of_mut!((*storage.as_mut_ptr()).layer.magic)
                .write(crate::DEVICE_DISPATCH_MAGIC);
        };
        Ok(Self { storage })
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

#[inline(never)]
fn device_terminator_command_available(
    id: u16,
    instance_extensions: &ExtensionSet,
    device_extensions: &ExtensionSet,
) -> bool {
    crate::command_core_level(id) != 0
        || crate::command_has_enabled_instance_extension(id, instance_extensions)
        || crate::command_has_enabled_device_extension(id, device_extensions)
}

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
        instance: &LoaderInstance,
        icd_index: usize,
        app_api_version: u32,
        ignore_newer_core_commands: bool,
        enabled_extensions: ExtensionSet,
    ) -> Result<Box<Self>, VkResult> {
        let dispatch_table = LoaderDeviceDispatch::try_new()?;
        let instance_extensions = instance.enabled_extensions;
        let icd_terminator_dispatch = unsafe {
            IcdDeviceTerminatorDispatchTable::load(get_device_proc_addr, native, |name| {
                device_terminator_command_available(name, &instance_extensions, &enabled_extensions)
            })
        };
        let device = try_box(Self {
            dispatch_table,
            icd_terminator_dispatch,
            chain_device: native,
            icd_device: native,
            get_device_proc_addr,
            chain_get_device_proc_addr: get_device_proc_addr,
            chain_dispatch_key: 0,
            instance: NonNull::from(instance),
            icd_index,
            app_core_level: api_core_level(app_api_version),
            ignore_newer_core_commands,
            enabled_extensions,
        })
        .map_err(|(result, _device)| result)?;
        // Upstream consumes the ICD loader-magic word immediately after
        // vkCreateDevice returns, before making any GDPA calls for the device.
        // SAFETY: A successfully-created ICD dispatchable has writable loader
        // data in its first word, as required by the loader/driver interface.
        unsafe { Self::set_dispatch(native, device.dispatch()) };
        Ok(device)
    }

    pub(crate) fn try_register(device: Box<Self>) -> Result<VkDevice, (VkResult, Box<Self>)> {
        let handle = device.chain_device;
        let dispatch_key = device.dispatch() as usize;
        let mut devices = match DEVICES.try_lock() {
            Ok(devices) => devices,
            Err(error) => return Err((error, device)),
        };
        let Some(alias_capacity) = devices.alias_reservations.checked_add(1) else {
            return Err((VkResult::ERROR_OUT_OF_HOST_MEMORY, device));
        };
        if devices.owned.try_reserve(1).is_err()
            || devices.aliases.try_reserve(alias_capacity).is_err()
        {
            return Err((VkResult::ERROR_OUT_OF_HOST_MEMORY, device));
        }
        // SAFETY: LoaderDevice is Send and registry ownership is synchronized.
        let pointer = unsafe { ErasedPointer::from_box(device) };
        let previous = devices.owned.insert(dispatch_key, pointer);
        let previous_alias = devices.aliases.insert(dispatch_key, pointer);
        DEVICE_SNAPSHOT.publish(dispatch_key, 0, pointer.as_ptr());
        debug_assert!(previous.is_none());
        debug_assert!(previous_alias.is_none());
        if let Some(previous) = previous {
            // Preserve the old typed-map replacement semantics on an impossible key collision.
            drop(unsafe { previous.into_box::<LoaderDevice>() });
        }
        drop(devices);
        crate::pending::set_created_device(dispatch_key);
        Ok(handle)
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
                .write(self.dispatch());
        };
    }

    unsafe fn set_dispatch(handle: VkDevice, dispatch: *const LayerDeviceDispatchTable) {
        // SAFETY: The caller guarantees that `handle` is writable loader data.
        unsafe {
            handle
                .0
                .cast::<*const LayerDeviceDispatchTable>()
                .write(dispatch);
        };
    }

    /// Resolves a command directly from the owning ICD.
    ///
    pub(crate) fn resolve(&self, name: &CStr) -> PFN_vkVoidFunction {
        if let Some(lookup) = crate::command_lookup(name.to_bytes())
            && let Some(command) =
                crate::icd_device_terminator_proc_addr(&self.icd_terminator_dispatch, lookup.id)
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
        let devices = DEVICES.lock_if_initialized()?;
        let device = devices.aliases.get(&key)?.as_ptr::<LoaderDevice>();
        drop(devices);
        // SAFETY: The caller guarantees creation-time exclusive access and the
        // boxed allocation remains stable after releasing the registry lock.
        Some(unsafe { &mut *device })
    }

    #[inline(never)]
    pub(crate) unsafe fn set_chain(
        &mut self,
        handle: VkDevice,
        resolver: PFN_vkGetDeviceProcAddr,
    ) -> Result<(), VkResult> {
        let chain_dispatch_key = unsafe { Self::dispatch_key(handle) }.unwrap_or(0);
        let alias_reservation = (chain_dispatch_key != 0)
            .then(DeviceAliasReservation::new)
            .transpose()?;
        // SAFETY: Creation is not yet externally visible, so the stable table
        // may be populated in place for the completed top-level chain.
        unsafe {
            LayerDeviceDispatchTable::load_into(self.dispatch_table.layer_mut(), resolver, handle);
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
        self.chain_dispatch_key = chain_dispatch_key;
        if self.chain_dispatch_key != 0 {
            let own_key = self.dispatch() as usize;
            let Some(alias_reservation) = alias_reservation else {
                // The non-zero key created a reservation immediately above.
                unsafe { core::hint::unreachable_unchecked() }
            };
            let previous = alias_reservation.insert(self.chain_dispatch_key, own_key);
            debug_assert!(
                previous
                    .is_none_or(|alias| alias.as_ptr::<LoaderDevice>() == core::ptr::from_mut(self))
            );
        }
        // Replace direct-ICD unknown slots with top-of-layer-chain targets.
        unknown::initialize_device_dispatch(self);
        Ok(())
    }

    #[inline(never)]
    pub(crate) unsafe fn from_handle<'a>(handle: VkDevice) -> Option<&'a Self> {
        let key = unsafe { Self::dispatch_key(handle) }?;
        if let Some(device) = DEVICE_SNAPSHOT.get(key) {
            // SAFETY: The coherent mapping belongs to the caller's live device;
            // its destruction cannot race this call under Vulkan's contract.
            return Some(unsafe { &*device });
        }
        // SAFETY: The registry fallback uses the same live-device contract.
        unsafe { Self::from_registry(key) }
    }

    // A collision can be frequent with many devices, so this is outlined without
    // a cold hint. Keeping its guard and hash lookup separate avoids saving
    // their registers on every successful cache lookup.
    #[inline(never)]
    unsafe fn from_registry<'a>(key: usize) -> Option<&'a Self> {
        let devices = DEVICES.lock_if_initialized()?;
        let device = devices.aliases.get(&key)?.as_ptr::<LoaderDevice>();
        if DEVICE_SNAPSHOT.is_empty(key) {
            // SAFETY: Registry ownership keeps the box live while refreshing
            // the snapshot after the previously published device was removed.
            let registered = unsafe { &*device };
            DEVICE_SNAPSHOT.publish(
                registered.dispatch() as usize,
                registered.chain_dispatch_key,
                device,
            );
        }
        drop(devices);
        // SAFETY: A live Vulkan handle cannot be destroyed concurrently under
        // the API's external-synchronization contract; the box is stable.
        Some(unsafe { &*device })
    }

    pub(crate) fn take_dispatch(dispatch: *const LayerDeviceDispatchTable) -> Option<Box<Self>> {
        let mut devices = DEVICES.lock_if_initialized()?;
        let alias = devices.aliases.remove(&(dispatch as usize))?;
        // SAFETY: The registry lock keeps the owning box live until removal below.
        let pointer = alias.as_ptr::<LoaderDevice>();
        let canonical = unsafe { (&*pointer).dispatch() as usize };
        let device = devices.owned.remove(&canonical)?;
        DEVICE_SNAPSHOT.remove(pointer);
        devices.aliases.remove(&canonical);
        if unsafe { (*device.as_ptr::<LoaderDevice>()).chain_dispatch_key } != 0 {
            devices
                .aliases
                .remove(&unsafe { (*device.as_ptr::<LoaderDevice>()).chain_dispatch_key });
        }
        devices.reset_if_unused();
        // SAFETY: The owning raw pointer was removed and all aliases were invalidated.
        Some(unsafe { device.into_box::<LoaderDevice>() })
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

    pub(crate) fn loader_proc_addr() -> unsafe extern "system" fn() {
        erase_function(vkGetDeviceProcAddr as PFN_vkGetDeviceProcAddr)
    }

    pub(crate) fn instance(&self) -> &LoaderInstance {
        // SAFETY: Vulkan requires the parent instance to outlive this device.
        unsafe { self.instance.as_ref() }
    }

    pub(crate) const fn icd_index(&self) -> usize {
        self.icd_index
    }

    pub(crate) const fn enabled_extensions(&self) -> &ExtensionSet {
        &self.enabled_extensions
    }

    pub(crate) fn chain_set_debug_utils_object_name(
        &self,
    ) -> Option<vk::PFN_vkSetDebugUtilsObjectNameEXT> {
        self.dispatch_table.vkSetDebugUtilsObjectNameEXT
    }

    pub(crate) const fn icd_set_debug_utils_object_name(
        &self,
    ) -> Option<vk::PFN_vkSetDebugUtilsObjectNameEXT> {
        self.icd_terminator_dispatch.vkSetDebugUtilsObjectNameEXT
    }

    pub(crate) fn icd_destroy_device(&self) -> Option<vk::PFN_vkDestroyDevice> {
        self.icd_terminator_dispatch.vkDestroyDevice
    }
}

pub(crate) fn initialize_unknown_dispatches(
    _guard: &crate::platform::LoaderLockGuard<'_>,
    instance: &LoaderInstance,
    name: &CStr,
    reserve_index: impl FnOnce() -> Option<usize>,
) -> Option<usize> {
    let Some(devices) = DEVICES.lock_if_initialized() else {
        // Unknown names must still be registered before the first device:
        // later device creation initializes its slots from that name table.
        return reserve_index();
    };
    let mut matching = Vec::new();
    matching.try_reserve_exact(devices.owned.len()).ok()?;
    matching.extend(
        devices
            .owned
            .values()
            .filter(|pointer| unsafe {
                core::ptr::eq((&*pointer.as_ptr::<LoaderDevice>()).instance(), instance)
            })
            .map(|pointer| unsafe { (&*pointer.as_ptr::<LoaderDevice>()).dispatch() as usize }),
    );
    drop(devices);
    // Finish fallible snapshot allocation before interning a command name.
    // Otherwise a retry can observe the name without initialized device slots.
    let index = reserve_index()?;
    for key in matching {
        // A resolver may re-enter the loader and destroy another device in
        // this snapshot. Revalidate each key instead of retaining raw pointers.
        let device = {
            let devices = DEVICES.lock_if_initialized()?;
            let Some(device) = devices.owned.get(&key) else {
                continue;
            };
            device.as_ptr::<LoaderDevice>()
        };
        // SAFETY: The loader lock excludes concurrent device destruction;
        // the resolver must keep its own device live for the call.
        let device = unsafe { &*device };
        // SAFETY: The stored resolver and chain handle are live together.
        let function = device.resolve_chain(name);
        device.store_unknown_dispatch(index, function);
    }
    Some(index)
}

const fn api_core_level(version: u32) -> u16 {
    let major = vk::VK_API_VERSION_MAJOR(version);
    let minor = vk::VK_API_VERSION_MINOR(version);
    debug_assert!(major < 64 && minor < 1024);
    ((major << 10) | minor) as u16
}

#[cfg(test)]
#[test]
fn unknown_dispatch_snapshot_revalidates_reentrantly_removed_devices() {
    let _test_guard = crate::allocation::fault::SWEEP_LOCK.lock().unwrap();
    let loader_guard = crate::platform::lock_loader();
    let instance = LoaderInstance::new(
        vk::VK_API_VERSION_1_0,
        ExtensionSet::default(),
        Vec::new(),
        crate::layer::ActiveLayers {
            loaded: Box::default(),
            reported: Box::default(),
            requested: Box::default(),
        },
        None,
        core::ptr::null(),
    )
    .unwrap();
    let mut native_dispatch: *const LayerDeviceDispatchTable = core::ptr::null();
    let handle = VkDevice(core::ptr::from_mut(&mut native_dispatch).cast());
    // SAFETY: The fake dispatchable is writable and remains live through the
    // test. Its resolver owns no foreign resources; instance outlives the device.
    let device = unsafe {
        LoaderDevice::new(
            handle,
            empty_test_resolver,
            &instance,
            0,
            vk::VK_API_VERSION_1_0,
            false,
            ExtensionSet::default(),
        )
    }
    .unwrap();
    let dispatch = device.dispatch();
    assert!(LoaderDevice::try_register(device).is_ok());
    // Exercise the published alias after ownership moved into the registry,
    // then after the exclusive creation-time borrow used to finalize the chain.
    unsafe {
        assert_eq!(
            LoaderDevice::from_handle(handle).unwrap().icd_device,
            handle
        );
        LoaderDevice::from_dispatch_key_mut(dispatch as usize)
            .unwrap()
            .set_chain(handle, empty_test_resolver)
            .unwrap();
        assert_eq!(
            LoaderDevice::from_handle(handle).unwrap().chain_device,
            handle
        );
    }
    assert_eq!(
        initialize_unknown_dispatches(&loader_guard, &instance, c"vkTestUnknown", || {
            // Exercise reentrant removal after snapshot construction, before it
            // is consumed. No snapshot pointer may be dereferenced after this drop.
            assert!(LoaderDevice::take_dispatch(dispatch).is_some());
            Some(0)
        }),
        Some(0)
    );
    // The fake native dispatch word remains readable after removal. Neither
    // the cache nor registry may return the freed loader record.
    assert!(unsafe { LoaderDevice::from_handle(handle) }.is_none());
}

#[cfg(test)]
unsafe extern "system" fn empty_test_resolver(
    _device: VkDevice,
    _name: *const core::ffi::c_char,
) -> PFN_vkVoidFunction {
    None
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
    // SAFETY: The caller guarantees a valid pNext chain.
    let Some(features) = (unsafe {
        find_input_chain(
            create_info.pNext,
            VkStructureType::PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR,
        )
    }) else {
        return false;
    };
    // SAFETY: `sType` identifies the concrete structure layout.
    let features = unsafe {
        &*core::ptr::from_ref(features).cast::<VkPhysicalDeviceMaintenance5FeaturesKHR<'_>>()
    };
    features.maintenance5 != 0
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
    instance: &LoaderInstance,
    icd: &IcdInstance,
    physical_device: VkPhysicalDevice,
    create_info: &VkDeviceCreateInfo<'_>,
    supported_by_layer: impl Fn(Option<u16>, &CStr) -> bool,
    validated: impl FnOnce(),
) -> Result<Vec<*const core::ffi::c_char>, VkResult> {
    if create_info.enabledExtensionCount == 0 {
        validated();
        return Ok(Vec::new());
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
    let capacity = count as usize;
    let mut properties = allocation::try_box_uninit_slice::<VkExtensionProperties>(capacity)?;
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
    let initialized = (returned_count as usize).min(capacity);
    let mut icd_names = Vec::new();
    icd_names
        .try_reserve_exact(create_info.enabledExtensionCount as usize)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let support = |requested: &CStr| {
        let requested_bytes = requested.to_bytes();
        let supported_by_layer = supported_by_layer(crate::extension_id(requested), requested);
        let supported_by_icd = properties[..initialized].iter().any(|property| {
            // SAFETY: The ICD reported these leading entries as initialized.
            let property = unsafe { property.assume_init_ref() };
            let chars = property.extensionName.as_slice();
            // SAFETY: `c_char` is exactly one byte on every supported C ABI.
            let bytes =
                unsafe { core::slice::from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len()) };
            let end = bytes
                .iter()
                .position(|byte| *byte == 0)
                .unwrap_or(bytes.len());
            requested_bytes.len() == end && requested_bytes == &bytes[..end]
        });
        (supported_by_layer, supported_by_icd)
    };
    // SAFETY: The non-null array contains `enabledExtensionCount` pointers
    // readable for this synchronous call, as required by the Vulkan contract.
    let requested_names = unsafe {
        core::slice::from_raw_parts(
            create_info.ppEnabledExtensionNames,
            create_info.enabledExtensionCount as usize,
        )
    };
    for &requested in requested_names {
        if requested.is_null() {
            return Err(VkResult::ERROR_INITIALIZATION_FAILED);
        }
        // SAFETY: Each requested extension is NUL-terminated by contract.
        let requested = unsafe { CStr::from_ptr(requested) };
        let (supported_by_layer, supported_by_icd) = support(requested);
        if !supported_by_layer && !supported_by_icd {
            emit_unsupported_device_extension(instance, requested);
            return Err(VkResult::ERROR_EXTENSION_NOT_PRESENT);
        }
        // Preserve each resolved result in the already-reserved output storage.
        icd_names.push(if supported_by_icd {
            requested.as_ptr()
        } else {
            core::ptr::null()
        });
    }
    validated();
    for (&resolved, &requested_pointer) in icd_names.iter().zip(requested_names) {
        if resolved.is_null() {
            // SAFETY: The validation pass established a live NUL-terminated string.
            let requested = unsafe { CStr::from_ptr(requested_pointer) };
            emit_unavailable_icd_extension(instance, icd, requested);
        }
    }
    icd_names.retain(|name| !name.is_null());
    // Retain the reserved storage: shrinking to a boxed slice can reallocate
    // through the infallible allocator after all fallible work has succeeded.
    Ok(icd_names)
}

#[cold]
fn emit_unsupported_device_extension(instance: &LoaderInstance, requested: &CStr) {
    instance.log_loader_message_text(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        format_args!(
            "loader_validate_device_extensions: Device extension {} not supported by selected physical device or enabled layers.",
            crate::debug::diagnostics::LossyBytes(requested.to_bytes())
        ),
    );
    instance.log_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkCreateDevice: Failed to validate extensions in list",
    );
}

#[cold]
fn emit_unavailable_icd_extension(instance: &LoaderInstance, icd: &IcdInstance, requested: &CStr) {
    instance.log_loader_category_message_text(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        crate::platform::LogFilter::Driver,
        format_args!(
            "vkCreateDevice extension {} not available for devices associated with ICD {}",
            crate::debug::diagnostics::LossyBytes(requested.to_bytes()),
            icd.icd
                .library_path()
                .unwrap_or_else(|| std::path::Path::new(""))
                .loader_display()
        ),
    );
}
