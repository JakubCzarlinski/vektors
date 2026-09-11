//! physical device implementation.

use crate::LoaderPathExt;
use crate::{
    CStr, IcdInstance, LayerInstanceDispatchTable, LoaderInstance, LoaderPhysicalDevice,
    LoaderPhysicalDeviceTrampoline, MaybeUninit, Ordering, PFN_vkEnumeratePhysicalDevices,
    VkExtensionProperties, VkInstance, VkLayerProperties, VkPhysicalDevice,
    VkPhysicalDeviceGroupProperties, VkPhysicalDeviceGroupPropertiesKHR, VkResult, allocation,
    collections, debug, destroy_icd_surfaces, discovery, fatal_loader_error, generated, icd, layer,
    platform, resolve_trampoline_physical_device,
};
use core::ffi::c_char;
use core::ptr;
use std::ffi::OsStr;

/// Enumerates physical devices through the active instance layer chain.
///
/// # Safety
///
/// The count and optional output array must satisfy Vulkan's enumeration
/// contract, and `instance` must be a live loader instance.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumeratePhysicalDevices(
    instance: VkInstance,
    physical_device_count: *mut u32,
    physical_devices: *mut VkPhysicalDevice,
) -> VkResult {
    let _loader_guard = platform::lock_loader();
    // SAFETY: A Vulkan instance entry point requires a live instance wrapper.
    let loader = unsafe { LoaderInstance::from_handle(instance) }.unwrap_or_else(|| {
        fatal_loader_error(
            c"vkEnumeratePhysicalDevices: Invalid instance [VUID-vkEnumeratePhysicalDevices-instance-parameter]",
        )
    });
    if physical_device_count.is_null() {
        loader.log_loader_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL
                | vk::VkDebugUtilsMessageTypeFlagBitsEXT::VALIDATION,
            c"vkEnumeratePhysicalDevices: Received NULL pointer for physical device count return value. [VUID-vkEnumeratePhysicalDevices-pPhysicalDeviceCount-parameter]",
        );
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The immutable dispatch allocation lives with the instance.
    let dispatch = unsafe { &*loader.dispatch() };
    debug_assert!(dispatch.vkEnumeratePhysicalDevices.is_some());
    // SAFETY: Dispatch loading guarantees this core Vulkan 1.0 command exists.
    let enumerate = unsafe { dispatch.vkEnumeratePhysicalDevices.unwrap_unchecked() };
    let filters = match IdFilters::from_environment() {
        Ok(filters) => filters,
        Err(result) => return result,
    };
    // SAFETY: The caller's Vulkan contracts are forwarded unchanged.
    let result = if let Some(filters) = filters.as_ref() {
        unsafe {
            enumerate_filtered_physical_devices(
                loader,
                dispatch,
                enumerate,
                instance,
                physical_device_count,
                physical_devices,
                filters,
            )
        }
    } else {
        unsafe { enumerate(instance, physical_device_count, physical_devices) }
    };
    if !physical_devices.is_null() && matches!(result, VkResult::SUCCESS | VkResult::INCOMPLETE) {
        let count = unsafe { physical_device_count.read() } as usize;
        if let Err(error) =
            unsafe { setup_trampoline_physical_devices(loader, physical_devices, count) }
        {
            return error;
        }
        retire_icds_without_physical_devices(loader);
    }
    result
}

#[cold]
#[inline(never)]
pub(crate) fn retire_icds_without_physical_devices(instance: &LoaderInstance) {
    for (icd_index, icd) in instance.active_icds().rev() {
        let has_physical_devices = {
            let devices = instance.physical_devices.lock();
            devices.active.iter().any(|handle| {
                // SAFETY: The terminator owns every handle in its active list.
                unsafe { LoaderPhysicalDevice::from_handle(*handle) }
                    .is_some_and(|device| device.icd_index == icd_index)
            })
        };
        if has_physical_devices || !icd.begin_retire() {
            continue;
        }

        if instance.device_configurations.is_none()
            && let Some(path) = icd.icd.library_path()
        {
            emit_instance_loader_category_message(
                instance,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                platform::LogFilter::Driver,
                format_args!(
                    "Removing driver {} due to not having any physical devices",
                    path.loader_display()
                ),
            );
        }
        destroy_icd_surfaces(instance, icd_index);
        debug::messenger::destroy_icd_objects(instance, icd_index);
        if let Some(destroy) = icd.dispatch.vkDestroyInstance {
            // SAFETY: This call owns the one-time retirement claim and all ICD
            // children were destroyed immediately above.
            unsafe { destroy(icd.handle, instance.forced_destroy_allocator()) };
        }
        if let Some(path) = icd.icd.library_path() {
            icd::unload_preloaded_icd(path);
        }
        icd.icd.unload_library();
    }
}

pub(crate) unsafe fn setup_trampoline_physical_devices(
    instance: &LoaderInstance,
    physical_devices: *mut VkPhysicalDevice,
    count: usize,
) -> Result<(), VkResult> {
    let mut state = instance.physical_devices.lock();
    state
        .trampolines
        .try_reserve(count)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    for index in 0..count {
        let chain = unsafe { physical_devices.add(index).read() };
        let terminator = state.active.get(index).copied().unwrap_or(chain);
        let key = chain.0 as usize;
        let trampoline = match state.trampolines.entry(key) {
            collections::HashMapEntry::Occupied(entry) => *entry.into_mut(),
            collections::HashMapEntry::Vacant(entry) => {
                let trampoline = allocation::try_box(LoaderPhysicalDeviceTrampoline::new(
                    instance, chain, terminator,
                ))
                .map_err(|(result, _trampoline)| result)?;
                // SAFETY: The instance registry serializes ownership; Vulkan
                // governs external synchronization of the pointed-to handle.
                *entry.insert(unsafe { collections::ErasedPointer::from_box(trampoline) })
            }
        };
        // SAFETY: The registry retains the trampoline's Box owner.
        let trampoline = unsafe { &*trampoline.as_ptr::<LoaderPhysicalDeviceTrampoline>() };
        unsafe { physical_devices.add(index).write(trampoline.handle()) };
    }
    Ok(())
}

pub(crate) unsafe fn setup_trampoline_physical_device_groups(
    instance: &LoaderInstance,
    groups: *mut VkPhysicalDeviceGroupProperties<'_>,
    count: usize,
) -> Result<(), VkResult> {
    for group_index in 0..count {
        let group = unsafe { &mut *groups.add(group_index) };
        let device_count =
            (group.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        unsafe {
            setup_trampoline_physical_devices(
                instance,
                group.physicalDevices.as_mut_ptr(),
                device_count,
            )
        }?;
    }
    Ok(())
}

pub(crate) const MAX_ID_FILTERS: usize = 16;

#[derive(Clone, Copy, Default)]
pub(crate) struct IdRange {
    begin: u32,
    end: u32,
}

#[derive(Default)]
pub(crate) struct IdFilter {
    ranges: [IdRange; MAX_ID_FILTERS],
    len: usize,
}

impl IdFilter {
    fn from_environment(name: &CStr, filter: &mut Self) -> Result<bool, VkResult> {
        // SAFETY: Loader configuration excludes concurrent environment mutation;
        // the callback parses the borrowed value and retains no reference.
        unsafe {
            platform::inspect_environment_lossy(name, |value| {
                let nonempty = value.is_some_and(|value| !value.is_empty());
                if let Some(value) = value {
                    Self::parse_into(OsStr::new(value), filter)?;
                }
                Ok(nonempty)
            })
        }?
    }

    fn parse_into(value: &OsStr, filter: &mut Self) -> Result<(), VkResult> {
        let owned;
        let value = if let Some(value) = value.to_str() {
            value
        } else {
            owned = debug::diagnostics::try_format(format_args!(
                "{}",
                std::path::Path::new(value).loader_display()
            ))?;
            &owned
        };
        filter.len = 0;
        for (index, token) in value.split(',').take(MAX_ID_FILTERS).enumerate() {
            let (begin, consumed) = parse_c_u32(token.as_bytes());
            let end = token
                .as_bytes()
                .get(consumed.saturating_add(1)..)
                .map_or(begin, |tail| parse_c_u32(tail).0);
            filter.ranges[index] = IdRange { begin, end };
            filter.len += 1;
        }
        Ok(())
    }

    fn matches(&self, value: u32) -> bool {
        self.ranges[..self.len]
            .iter()
            .any(|range| range.begin <= value && value <= range.end)
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

pub(crate) fn parse_c_u32(bytes: &[u8]) -> (u32, usize) {
    let whitespace = bytes
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let mut index = whitespace;
    let negative = bytes.get(index) == Some(&b'-');
    if matches!(bytes.get(index), Some(b'-' | b'+')) {
        index += 1;
    }
    let (radix, prefix) =
        if bytes.get(index) == Some(&b'0') && matches!(bytes.get(index + 1), Some(b'x' | b'X')) {
            (16_u32, 2_usize)
        } else if bytes.get(index) == Some(&b'0') {
            (8, 0)
        } else {
            (10, 0)
        };
    index += prefix;
    let digit_start = index;
    let mut value = 0_u64;
    while let Some(digit) = bytes.get(index).and_then(|byte| match byte {
        b'0'..=b'9' => Some(u32::from(byte - b'0')),
        b'a'..=b'f' => Some(u32::from(byte - b'a') + 10),
        b'A'..=b'F' => Some(u32::from(byte - b'A') + 10),
        _ => None,
    }) {
        if digit >= radix {
            break;
        }
        value = value
            .saturating_mul(u64::from(radix))
            .saturating_add(u64::from(digit));
        index += 1;
    }
    if index == digit_start {
        return (0, whitespace);
    }
    let value = if negative {
        0_u64.wrapping_sub(value)
    } else {
        value
    };
    ((value & u64::from(u32::MAX)) as u32, index)
}

#[derive(Default)]
pub(crate) struct IdFilters {
    device: IdFilter,
    vendor: IdFilter,
    driver: IdFilter,
}

impl IdFilters {
    fn from_environment() -> Result<Option<Self>, VkResult> {
        if platform::has_elevated_privileges() {
            return Ok(None);
        }
        let mut filters = Self::default();
        let has_device =
            IdFilter::from_environment(c"VK_LOADER_DEVICE_ID_FILTER", &mut filters.device)?;
        let has_vendor =
            IdFilter::from_environment(c"VK_LOADER_VENDOR_ID_FILTER", &mut filters.vendor)?;
        let has_driver =
            IdFilter::from_environment(c"VK_LOADER_DRIVER_ID_FILTER", &mut filters.driver)?;
        if !has_device && !has_vendor && !has_driver {
            return Ok(None);
        }
        Ok(Some(filters))
    }
}

pub(crate) struct IdFilterPropertyStorage {
    basic: vk::VkPhysicalDeviceProperties,
    properties2: vk::VkPhysicalDeviceProperties2<'static>,
    driver: vk::VkPhysicalDeviceDriverProperties<'static>,
}

pub(crate) unsafe fn physical_device_matches_id_filters(
    instance: &LoaderInstance,
    dispatch: &LayerInstanceDispatchTable,
    physical_device: VkPhysicalDevice,
    filters: &IdFilters,
    storage: *mut IdFilterPropertyStorage,
) -> bool {
    let Some(get_properties) = dispatch.vkGetPhysicalDeviceProperties else {
        return false;
    };
    unsafe { get_properties(physical_device, ptr::addr_of_mut!((*storage).basic)) };
    let device_id = unsafe { ptr::addr_of!((*storage).basic.deviceID).read() };
    let vendor_id = unsafe { ptr::addr_of!((*storage).basic.vendorID).read() };
    if (!filters.device.is_empty() && !filters.device.matches(device_id))
        || (!filters.vendor.is_empty() && !filters.vendor.matches(vendor_id))
    {
        return false;
    }
    if filters.driver.is_empty() {
        return true;
    }

    unsafe {
        ptr::addr_of_mut!((*storage).properties2.sType)
            .write(vk::VkStructureType::PHYSICAL_DEVICE_PROPERTIES_2);
        ptr::addr_of_mut!((*storage).properties2.pNext)
            .write(ptr::addr_of_mut!((*storage).driver).cast());
        ptr::addr_of_mut!((*storage).driver.sType)
            .write(vk::VkStructureType::PHYSICAL_DEVICE_DRIVER_PROPERTIES);
        ptr::addr_of_mut!((*storage).driver.pNext).write(ptr::null_mut());
    }
    if instance.api_version >= vk::VK_API_VERSION_1_1 {
        let Some(get_properties2) = dispatch.vkGetPhysicalDeviceProperties2 else {
            return false;
        };
        unsafe {
            get_properties2(physical_device, ptr::addr_of_mut!((*storage).properties2));
        };
    } else {
        let extension_enabled = instance
            .enabled_extensions
            .contains(generated::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION_ID);
        let Some(get_properties2) = extension_enabled
            .then_some(dispatch.vkGetPhysicalDeviceProperties2KHR)
            .flatten()
        else {
            return false;
        };
        debug_assert_eq!(
            core::mem::size_of::<vk::VkPhysicalDeviceProperties2KHR<'_>>(),
            core::mem::size_of::<vk::VkPhysicalDeviceProperties2<'_>>()
        );
        debug_assert_eq!(
            core::mem::align_of::<vk::VkPhysicalDeviceProperties2KHR<'_>>(),
            core::mem::align_of::<vk::VkPhysicalDeviceProperties2<'_>>()
        );
        unsafe {
            get_properties2(
                physical_device,
                ptr::addr_of_mut!((*storage).properties2).cast(),
            );
        };
    }
    let driver_id = unsafe { ptr::addr_of!((*storage).driver.driverID).read() };
    filters.driver.matches(driver_id.0.cast_unsigned())
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn enumerate_filtered_physical_devices(
    loader: &LoaderInstance,
    dispatch: &LayerInstanceDispatchTable,
    enumerate: PFN_vkEnumeratePhysicalDevices,
    instance: VkInstance,
    physical_device_count: *mut u32,
    physical_devices: *mut VkPhysicalDevice,
    filters: &IdFilters,
) -> VkResult {
    let mut available = 0;
    let result = unsafe { enumerate(instance, &raw mut available, ptr::null_mut()) };
    if result != VkResult::SUCCESS {
        return result;
    }
    let available = available as usize;
    let mut chain_devices = Vec::new();
    if chain_devices.try_reserve_exact(available).is_err() {
        return VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    chain_devices.resize(available, VkPhysicalDevice::NULL);
    let mut returned = available.min(u32::MAX as usize) as u32;
    let result = unsafe { enumerate(instance, &raw mut returned, chain_devices.as_mut_ptr()) };
    if result != VkResult::SUCCESS {
        return result;
    }
    chain_devices.truncate((returned as usize).min(available));

    let capacity = if physical_devices.is_null() {
        usize::MAX
    } else {
        (unsafe { physical_device_count.read() }) as usize
    };
    let mut storage = match allocation::try_box_uninit::<IdFilterPropertyStorage>() {
        Ok(storage) => storage,
        Err(result) => return result,
    };
    let storage = storage.as_mut_ptr();
    let mut matched = 0_usize;
    for physical_device in chain_devices {
        if !unsafe {
            physical_device_matches_id_filters(loader, dispatch, physical_device, filters, storage)
        } {
            continue;
        }
        if !physical_devices.is_null() && matched < capacity {
            unsafe { physical_devices.add(matched).write(physical_device) };
        }
        matched += 1;
    }
    let written = capacity.min(matched);
    unsafe {
        physical_device_count.write(written as u32);
    }
    if written < matched {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn enumerate_filtered_physical_device_groups(
    loader: &LoaderInstance,
    dispatch: &LayerInstanceDispatchTable,
    enumerate: vk::PFN_vkEnumeratePhysicalDeviceGroups,
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
    filters: &IdFilters,
) -> VkResult {
    let mut available = 0;
    let result = unsafe { enumerate(instance, &raw mut available, ptr::null_mut()) };
    if result != VkResult::SUCCESS {
        return result;
    }
    let available = available as usize;
    let mut chain_groups =
        match allocation::try_box_uninit_slice::<VkPhysicalDeviceGroupProperties<'_>>(available) {
            Ok(groups) => groups,
            Err(result) => return result,
        };
    unsafe { chain_groups.as_mut_ptr().write_bytes(0, available) };
    let mut returned = available.min(u32::MAX as usize) as u32;
    let result = unsafe {
        enumerate(
            instance,
            &raw mut returned,
            chain_groups.as_mut_ptr().cast(),
        )
    };
    if result != VkResult::SUCCESS {
        return result;
    }
    let returned = (returned as usize).min(available);
    let chain_groups = unsafe { chain_groups.assume_init() };

    let capacity = if group_properties.is_null() {
        usize::MAX
    } else {
        (unsafe { group_count.read() }) as usize
    };
    let mut storage = match allocation::try_box_uninit::<IdFilterPropertyStorage>() {
        Ok(storage) => storage,
        Err(result) => return result,
    };
    let storage = storage.as_mut_ptr();
    let mut matched = 0_usize;
    'groups: for group in &chain_groups[..returned] {
        let device_count =
            (group.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        for &physical_device in &group.physicalDevices[..device_count] {
            if !unsafe {
                physical_device_matches_id_filters(
                    loader,
                    dispatch,
                    physical_device,
                    filters,
                    storage,
                )
            } {
                continue 'groups;
            }
        }
        if !group_properties.is_null() && matched < capacity {
            unsafe { group_properties.add(matched).write(*group) };
        }
        matched += 1;
    }
    let written = capacity.min(matched);
    unsafe {
        group_count.write(written as u32);
    }
    if written < matched {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

#[derive(Clone, Copy)]
pub(crate) enum IcdGroupEnumerator {
    Core(vk::PFN_vkEnumeratePhysicalDeviceGroups),
    Khr(vk::PFN_vkEnumeratePhysicalDeviceGroupsKHR),
    PhysicalDevices(PFN_vkEnumeratePhysicalDevices),
}

pub(crate) fn icd_group_enumerator(
    loader: &LoaderInstance,
    icd: &IcdInstance,
) -> Option<IcdGroupEnumerator> {
    let use_khr = loader
        .enabled_extensions
        .contains(generated::VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_ID);
    if use_khr {
        icd.dispatch
            .vkEnumeratePhysicalDeviceGroupsKHR
            .map(IcdGroupEnumerator::Khr)
    } else {
        icd.dispatch
            .vkEnumeratePhysicalDeviceGroups
            .map(IcdGroupEnumerator::Core)
    }
    .or_else(|| {
        icd.dispatch
            .vkEnumeratePhysicalDevices
            .map(IcdGroupEnumerator::PhysicalDevices)
    })
}

pub(crate) unsafe fn query_icd_group_count(
    icd: &IcdInstance,
    enumerate: IcdGroupEnumerator,
) -> Result<u32, VkResult> {
    let mut count = 0;
    let result = match enumerate {
        IcdGroupEnumerator::Core(enumerate) => unsafe {
            enumerate(icd.handle, &raw mut count, ptr::null_mut())
        },
        IcdGroupEnumerator::Khr(enumerate) => unsafe {
            enumerate(icd.handle, &raw mut count, ptr::null_mut())
        },
        IcdGroupEnumerator::PhysicalDevices(enumerate) => unsafe {
            enumerate(icd.handle, &raw mut count, ptr::null_mut())
        },
    };
    (result == VkResult::SUCCESS).then_some(count).ok_or(result)
}

const _: () = {
    assert!(
        core::mem::size_of::<VkPhysicalDeviceGroupProperties<'_>>()
            == core::mem::size_of::<VkPhysicalDeviceGroupPropertiesKHR<'_>>()
    );
    assert!(
        core::mem::align_of::<VkPhysicalDeviceGroupProperties<'_>>()
            == core::mem::align_of::<VkPhysicalDeviceGroupPropertiesKHR<'_>>()
    );
    assert!(
        core::mem::offset_of!(VkPhysicalDeviceGroupProperties<'_>, sType)
            == core::mem::offset_of!(VkPhysicalDeviceGroupPropertiesKHR<'_>, sType)
    );
    assert!(
        core::mem::offset_of!(VkPhysicalDeviceGroupProperties<'_>, pNext)
            == core::mem::offset_of!(VkPhysicalDeviceGroupPropertiesKHR<'_>, pNext)
    );
    assert!(
        core::mem::offset_of!(VkPhysicalDeviceGroupProperties<'_>, physicalDeviceCount)
            == core::mem::offset_of!(VkPhysicalDeviceGroupPropertiesKHR<'_>, physicalDeviceCount)
    );
    assert!(
        core::mem::offset_of!(VkPhysicalDeviceGroupProperties<'_>, physicalDevices)
            == core::mem::offset_of!(VkPhysicalDeviceGroupPropertiesKHR<'_>, physicalDevices)
    );
    assert!(
        core::mem::offset_of!(VkPhysicalDeviceGroupProperties<'_>, subsetAllocation)
            == core::mem::offset_of!(VkPhysicalDeviceGroupPropertiesKHR<'_>, subsetAllocation)
    );
};

type NativeGroup = (usize, VkPhysicalDeviceGroupProperties<'static>);

/// Keeps the common single group on the stack, growing fallibly for more groups.
#[derive(Default)]
struct NativeGroups {
    single: Option<NativeGroup>,
    heap: Vec<NativeGroup>,
}

impl NativeGroups {
    fn try_reserve(&mut self, additional: usize) -> Result<(), VkResult> {
        if self.heap.capacity() != 0 {
            return self
                .heap
                .try_reserve(additional)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY);
        }
        let required = usize::from(self.single.is_some())
            .checked_add(additional)
            .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        if required > 1 {
            self.heap
                .try_reserve_exact(required)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            if let Some(single) = self.single.take() {
                self.heap.push(single);
            }
        }
        Ok(())
    }

    fn push(&mut self, icd_index: usize, properties: &VkPhysicalDeviceGroupProperties<'static>) {
        if self.heap.capacity() == 0 {
            debug_assert!(self.single.is_none());
            self.single = Some((icd_index, *properties));
        } else {
            self.heap.push((icd_index, *properties));
        }
    }
}

impl core::ops::Deref for NativeGroups {
    type Target = [NativeGroup];

    fn deref(&self) -> &Self::Target {
        if self.heap.capacity() == 0 {
            self.single.as_slice()
        } else {
            &self.heap
        }
    }
}

impl core::ops::DerefMut for NativeGroups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.heap.capacity() == 0 {
            self.single.as_mut_slice()
        } else {
            &mut self.heap
        }
    }
}

unsafe fn enumerate_icd_groups(
    icd: &IcdInstance,
    enumerate: IcdGroupEnumerator,
    output: *mut VkPhysicalDeviceGroupProperties<'_>,
    output_capacity: usize,
    output_offset: usize,
    icd_index: usize,
    native_groups: &mut NativeGroups,
) -> Result<(), VkResult> {
    let count = unsafe { query_icd_group_count(icd, enumerate) }? as usize;
    match enumerate {
        IcdGroupEnumerator::Core(_) | IcdGroupEnumerator::Khr(_) => {
            // Vulkan promotion preserves every field and the C layout. Use
            // one core buffer for either ABI instead of allocating a copy.
            // One group is common; keep that bounded temporary on the stack.
            // Larger driver-reported counts retain fallible heap storage.
            let mut single = [VkPhysicalDeviceGroupProperties::DEFAULT];
            let mut heap = Vec::new();
            let groups = if count <= single.len() {
                &mut single[..count]
            } else {
                heap.try_reserve_exact(count)
                    .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
                heap.resize(count, VkPhysicalDeviceGroupProperties::DEFAULT);
                heap.as_mut_slice()
            };
            for (index, group) in groups.iter_mut().enumerate() {
                if output_offset + index < output_capacity {
                    group.pNext = unsafe { (*output.add(output_offset + index)).pNext };
                }
            }
            let mut returned = count.min(u32::MAX as usize) as u32;
            // SAFETY: The initialized buffer has count entries. The KHR
            // structure has identical fields, validity rules and layout.
            let result = unsafe {
                match enumerate {
                    IcdGroupEnumerator::Core(enumerate) => {
                        enumerate(icd.handle, &raw mut returned, groups.as_mut_ptr())
                    }
                    IcdGroupEnumerator::Khr(enumerate) => {
                        enumerate(icd.handle, &raw mut returned, groups.as_mut_ptr().cast())
                    }
                    IcdGroupEnumerator::PhysicalDevices(_) => unreachable!(),
                }
            };
            if result != VkResult::SUCCESS && result != VkResult::INCOMPLETE {
                return Err(result);
            }
            let groups = &groups[..(returned as usize).min(count)];
            native_groups
                .try_reserve(groups.len())
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            for properties in groups {
                native_groups.push(icd_index, properties);
            }
            Ok(())
        }
        IcdGroupEnumerator::PhysicalDevices(_) => {
            let devices = unsafe { enumerate_icd_physical_devices(icd) }
                .map_err(PhysicalDeviceEnumerationError::result)?;
            native_groups
                .try_reserve(devices.len)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            for (index, device) in devices.iter().enumerate() {
                let mut group = VkPhysicalDeviceGroupProperties {
                    physicalDeviceCount: 1,
                    ..VkPhysicalDeviceGroupProperties::DEFAULT
                };
                group.physicalDevices[0] = device;
                if output_offset + index < output_capacity {
                    group.pNext = unsafe { (*output.add(output_offset + index)).pNext };
                }
                native_groups.push(icd_index, &group);
            }
            Ok(())
        }
    }
}

pub(crate) unsafe fn discover_all_physical_devices(
    instance: &LoaderInstance,
    sort_linux: bool,
) -> Result<Vec<NativePhysicalDevice>, VkResult> {
    #[cfg(windows)]
    let mut devices = unsafe { windows_sorted_physical_devices(instance) }?;
    #[cfg(not(windows))]
    let mut devices = Vec::new();
    let mut successful_icds = usize::from(!devices.is_empty());
    for (icd_index, icd) in instance.active_icds().rev() {
        let native = match unsafe { enumerate_icd_physical_devices(icd) } {
            Ok(native) => native,
            Err(PhysicalDeviceEnumerationError::LoaderAllocation) => {
                return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
            }
            Err(PhysicalDeviceEnumerationError::Driver(result)) => {
                let path = icd
                    .icd
                    .library_path()
                    .unwrap_or_else(|| std::path::Path::new(""))
                    .loader_display();
                emit_instance_loader_message(
                    instance,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                    format_args!(
                        "setup_loader_term_phys_devs: Call to 'vkEnumeratePhysicalDevices' in ICD {path} failed with error code {}",
                        result.0
                    ),
                );
                continue;
            }
        };
        successful_icds += 1;
        devices
            .try_reserve(native.len)
            .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
        for handle in native.iter() {
            let device = NativePhysicalDevice { icd_index, handle };
            if !devices.contains(&device) {
                devices.push(device);
            }
        }
    }
    if successful_icds == 0 || devices.is_empty() {
        Err(VkResult::ERROR_INITIALIZATION_FAILED)
    } else {
        if sort_linux && linux_sort_enabled(instance)? {
            unsafe { linux_sort_physical_devices(instance, &mut devices) }?;
        }
        Ok(devices)
    }
}

#[cfg(windows)]
#[cold]
#[inline(never)]
pub(crate) unsafe fn windows_sorted_physical_devices(
    instance: &LoaderInstance,
) -> Result<Vec<NativePhysicalDevice>, VkResult> {
    struct AdapterDevices {
        luid: platform::AdapterLuid,
        devices: Vec<NativePhysicalDevice>,
    }

    unsafe fn is_d3d12_layered(instance: &LoaderInstance, group: &AdapterDevices) -> bool {
        let icd = &instance.icds[group.devices[0].icd_index];
        let Some(get_properties) = icd.dispatch.vkGetPhysicalDeviceProperties else {
            return false;
        };
        let Ok(mut basic) = allocation::try_box_uninit::<vk::VkPhysicalDeviceProperties>() else {
            return false;
        };
        for device in &group.devices {
            // SAFETY: `basic` points to writable storage of the exact Vulkan
            // output type, and the native handle belongs to this ICD.
            unsafe { get_properties(device.handle, basic.as_mut_ptr()) };
            // SAFETY: The ICD initialized the output structure before return.
            let api_version = unsafe { ptr::addr_of!((*basic.as_ptr()).apiVersion).read() };
            let get_properties2 = if instance.api_version >= vk::VK_API_VERSION_1_1
                && api_version >= vk::VK_API_VERSION_1_1
            {
                icd.dispatch.vkGetPhysicalDeviceProperties2
            } else {
                icd.dispatch
                    .vkGetPhysicalDeviceProperties2KHR
                    .map(|command| {
                        // The promoted and KHR signatures and structures are ABI
                        // aliases by the Vulkan specification.
                        unsafe {
                            core::mem::transmute::<
                                vk::PFN_vkGetPhysicalDeviceProperties2KHR,
                                vk::PFN_vkGetPhysicalDeviceProperties2,
                            >(command)
                        }
                    })
            };
            let Some(get_properties2) = get_properties2 else {
                continue;
            };
            let mut layered = vk::VkPhysicalDeviceLayeredDriverPropertiesMSFT::DEFAULT;
            let mut properties = vk::VkPhysicalDeviceProperties2 {
                pNext: ptr::from_mut(&mut layered).cast(),
                ..vk::VkPhysicalDeviceProperties2::DEFAULT
            };
            // SAFETY: Both output structures are initialized, correctly
            // chained, and live for the call.
            unsafe { get_properties2(device.handle, &raw mut properties) };
            if layered.underlyingAPI == vk::VkLayeredDriverUnderlyingApiMSFT::D3D12 {
                return true;
            }
        }
        false
    }

    let mut groups: Vec<AdapterDevices> = Vec::new();
    for luid in platform::adapter_luids()? {
        for (icd_index, icd) in instance.active_icds().rev() {
            let Some(enumerate) = icd.icd.enumerate_adapter_physical_devices else {
                continue;
            };
            let mut count = 0;
            // SAFETY: Count is writable; a null output performs the required
            // loader-driver interface sizing query.
            let result = unsafe { enumerate(icd.handle, luid, &raw mut count, ptr::null_mut()) };
            if result == VkResult::ERROR_OUT_OF_HOST_MEMORY {
                return Err(result);
            }
            if result != VkResult::SUCCESS || count == 0 {
                continue;
            }

            let mut group = None;
            loop {
                let capacity = count as usize;
                let mut storage = allocation::try_box_uninit_slice::<VkPhysicalDevice>(capacity)?;
                let mut returned = count;
                // SAFETY: Storage contains `capacity` writable handles and
                // `returned` supplies that capacity to the ICD.
                let result = unsafe {
                    enumerate(
                        icd.handle,
                        luid,
                        &raw mut returned,
                        storage.as_mut_ptr().cast(),
                    )
                };
                if result == VkResult::INCOMPLETE {
                    count = returned.max(count.saturating_add(1));
                    continue;
                }
                if result == VkResult::ERROR_OUT_OF_HOST_MEMORY {
                    return Err(result);
                }
                if result != VkResult::SUCCESS {
                    break;
                }
                let initialized = (returned as usize).min(capacity);
                let mut devices = Vec::new();
                devices
                    .try_reserve_exact(initialized)
                    .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
                devices.extend(storage[..initialized].iter().map(|device| {
                    // SAFETY: The successful ICD call initialized the reported
                    // prefix, capped to the allocated capacity.
                    NativePhysicalDevice {
                        icd_index,
                        handle: unsafe { device.assume_init() },
                    }
                }));
                group = Some(devices);
                break;
            }
            let Some(group) = group else { continue };
            if !groups.iter().any(|existing| existing.devices == group) {
                groups
                    .try_reserve(1)
                    .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
                groups.push(AdapterDevices {
                    luid,
                    devices: group,
                });
            }
        }
    }

    // Match `sort_physical_devices_with_same_luid`: when two ICDs expose the
    // same adapter, a Vulkan-on-D3D12 implementation follows the native ICD.
    for index in 0..groups.len().saturating_sub(1) {
        for candidate in index + 1..groups.len() {
            if groups[index].luid == groups[candidate].luid
                && unsafe { is_d3d12_layered(instance, &groups[index]) }
            {
                groups.swap(index, candidate);
            }
        }
    }

    let count = groups.iter().try_fold(0_usize, |count, group| {
        count
            .checked_add(group.devices.len())
            .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)
    })?;
    let mut devices = Vec::new();
    devices
        .try_reserve_exact(count)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    devices.extend(groups.into_iter().flat_map(|group| group.devices));
    Ok(devices)
}

unsafe fn enumerate_physical_device_groups_impl(
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
) -> VkResult {
    // SAFETY: Only the core/KHR terminators call this helper. Layers pass
    // the loader-owned handle returned by the instance terminator down-chain.
    let Some(instance) = (unsafe { LoaderInstance::from_internal_handle(instance) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if group_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let group_count = unsafe { &mut *group_count };

    let mut upper_bound = 0_u32;
    for (_, icd) in instance.active_icds() {
        let Some(enumerate) = icd_group_enumerator(instance, icd) else {
            continue;
        };
        if let Ok(count) = unsafe { query_icd_group_count(icd, enumerate) } {
            upper_bound = upper_bound.saturating_add(count);
        }
    }
    if group_properties.is_null() {
        *group_count = upper_bound;
        return if upper_bound == 0 {
            emit_instance_loader_message(
                instance,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
                format_args!(
                    "setup_loader_term_phys_devs:  Failed to detect any valid GPUs in the current config"
                ),
            );
            VkResult::ERROR_INITIALIZATION_FAILED
        } else {
            VkResult::SUCCESS
        };
    }
    unsafe {
        enumerate_physical_device_group_properties(
            instance,
            group_count,
            group_properties,
            upper_bound,
        )
    }
}

#[cold]
fn emit_physical_device_discovery_error(instance: &LoaderInstance, result: VkResult) {
    if result == VkResult::ERROR_INITIALIZATION_FAILED {
        emit_instance_loader_message(
            instance,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR,
            format_args!(
                "setup_loader_term_phys_devs:  Failed to detect any valid GPUs in the current config"
            ),
        );
    }
}

fn physical_devices_need_refresh(
    instance: &LoaderInstance,
    all_devices: &[NativePhysicalDevice],
) -> bool {
    let state = instance.physical_devices.lock();
    state.active.len() != all_devices.len()
        || all_devices.iter().any(|native| {
            !state.active.iter().any(|handle| {
                // SAFETY: Active entries are loader-owned terminator handles.
                unsafe { LoaderPhysicalDevice::from_handle(*handle) }.is_some_and(|device| {
                    device.icd_index == native.icd_index && device.native == native.handle
                })
            })
        })
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn enumerate_physical_device_group_properties(
    instance: &LoaderInstance,
    group_count: &mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
    upper_bound: u32,
) -> VkResult {
    let capacity = *group_count as usize;
    // Like upstream, reuse the physical-device snapshot once populated.
    // Zero groups still trigger discovery, preserving its failure handling.
    let have_physical_devices =
        upper_bound != 0 && !instance.physical_devices.lock().active.is_empty();
    let mut all_devices = if have_physical_devices {
        Vec::new()
    } else {
        match unsafe { discover_all_physical_devices(instance, false) } {
            Ok(devices) => devices,
            Err(result) => {
                emit_physical_device_discovery_error(instance, result);
                *group_count = 0;
                return result;
            }
        }
    };
    let refresh_physical_devices =
        !have_physical_devices && physical_devices_need_refresh(instance, &all_devices);
    let sort_linux = match linux_sort_enabled(instance) {
        Ok(enabled) => enabled,
        Err(result) => {
            *group_count = 0;
            return result;
        }
    };
    if refresh_physical_devices
        && sort_linux
        && let Err(result) = unsafe { linux_sort_physical_devices(instance, &mut all_devices) }
    {
        *group_count = 0;
        return result;
    }
    #[cfg(windows)]
    let windows_sorted_devices = match unsafe { windows_sorted_physical_devices(instance) } {
        Ok(devices) => devices,
        Err(result) => {
            *group_count = 0;
            return result;
        }
    };
    let visible_devices = if instance.device_configurations.is_some() {
        match unsafe { discover_active_physical_devices(instance) } {
            Ok(devices) => Some(devices),
            Err(result) => {
                *group_count = 0;
                return result;
            }
        }
    } else {
        None
    };

    let mut native_groups = match unsafe {
        collect_native_device_groups(instance, group_properties, capacity, upper_bound)
    } {
        Ok(groups) => groups,
        Err(result) => {
            *group_count = 0;
            return result;
        }
    };
    if sort_linux
        && let Err(result) =
            unsafe { linux_sort_physical_device_groups(instance, &mut native_groups) }
    {
        *group_count = 0;
        return result;
    }
    #[cfg(windows)]
    if !windows_sorted_devices.is_empty() {
        windows_sort_physical_device_groups(&mut native_groups, &windows_sorted_devices);
    }

    let result = unsafe {
        write_visible_device_groups(
            instance,
            &all_devices,
            refresh_physical_devices,
            &mut native_groups,
            visible_devices.as_deref(),
            group_count,
            group_properties,
        )
    };
    match result {
        Ok(result) => result,
        Err(error) => {
            *group_count = 0;
            error
        }
    }
}

#[cfg(windows)]
pub(crate) fn windows_sort_physical_device_groups(
    groups: &mut [(usize, VkPhysicalDeviceGroupProperties<'static>)],
    sorted_devices: &[NativePhysicalDevice],
) {
    let device_order = |icd_index: usize, handle: VkPhysicalDevice| {
        sorted_devices
            .iter()
            .position(|device| device.icd_index == icd_index && device.handle == handle)
            .unwrap_or(usize::MAX)
    };

    // Device groups are bounded by VK_MAX_DEVICE_GROUP_SIZE. An insertion
    // sort avoids allocation on this cold path and is equivalent to upstream's
    // repeated search-and-swap against the DXGI-prioritized device sequence.
    for (icd_index, properties) in groups.iter_mut() {
        let count =
            (properties.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        for index in 1..count {
            let mut current = index;
            while current != 0
                && device_order(*icd_index, properties.physicalDevices[current])
                    < device_order(*icd_index, properties.physicalDevices[current - 1])
            {
                properties.physicalDevices.swap(current, current - 1);
                current -= 1;
            }
        }
    }

    let group_order = |group: &(usize, VkPhysicalDeviceGroupProperties<'static>)| {
        let count =
            (group.1.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        group.1.physicalDevices[..count]
            .iter()
            .map(|&handle| device_order(group.0, handle))
            .min()
            .unwrap_or(usize::MAX)
    };
    for index in 1..groups.len() {
        let mut current = index;
        while current != 0 && group_order(&groups[current]) < group_order(&groups[current - 1]) {
            groups.swap(current, current - 1);
            current -= 1;
        }
    }
}

pub(crate) unsafe extern "system" fn terminator_enumerate_physical_device_groups(
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
) -> VkResult {
    unsafe { enumerate_physical_device_groups_impl(instance, group_count, group_properties) }
}

pub(crate) unsafe extern "system" fn terminator_enumerate_physical_device_groups_khr(
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupPropertiesKHR<'_>,
) -> VkResult {
    debug_assert_eq!(
        core::mem::size_of::<VkPhysicalDeviceGroupPropertiesKHR<'_>>(),
        core::mem::size_of::<VkPhysicalDeviceGroupProperties<'_>>()
    );
    debug_assert_eq!(
        core::mem::align_of::<VkPhysicalDeviceGroupPropertiesKHR<'_>>(),
        core::mem::align_of::<VkPhysicalDeviceGroupProperties<'_>>()
    );
    unsafe { enumerate_physical_device_groups_impl(instance, group_count, group_properties.cast()) }
}

/// Enumerates physical-device groups through the active instance chain.
///
/// # Safety
///
/// `instance` must be live, `group_count` must be writable, and a non-null
/// `group_properties` must provide the capacity supplied through `group_count`.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumeratePhysicalDeviceGroups(
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
) -> VkResult {
    if group_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let Some(loader) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let dispatch = unsafe { &*loader.dispatch() };
    let Some(enumerate) = dispatch.vkEnumeratePhysicalDeviceGroups else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    unsafe {
        enumerate_physical_device_groups_entry(
            loader,
            dispatch,
            enumerate,
            instance,
            group_count,
            group_properties,
        )
    }
}

#[inline(never)]
unsafe fn enumerate_physical_device_groups_entry(
    loader: &LoaderInstance,
    dispatch: &LayerInstanceDispatchTable,
    enumerate: vk::PFN_vkEnumeratePhysicalDeviceGroups,
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
) -> VkResult {
    let filters = match IdFilters::from_environment() {
        Ok(filters) => filters,
        Err(result) => return result,
    };
    let result = if let Some(filters) = filters.as_ref() {
        unsafe {
            enumerate_filtered_physical_device_groups(
                loader,
                dispatch,
                enumerate,
                instance,
                group_count,
                group_properties,
                filters,
            )
        }
    } else {
        unsafe { enumerate(instance, group_count, group_properties) }
    };
    if !group_properties.is_null() && matches!(result, VkResult::SUCCESS | VkResult::INCOMPLETE) {
        let count = unsafe { group_count.read() } as usize;
        if let Err(error) =
            unsafe { setup_trampoline_physical_device_groups(loader, group_properties, count) }
        {
            return error;
        }
    }
    result
}

/// Enumerates physical-device groups through the KHR instance chain.
///
/// # Safety
///
/// `instance` must be live, `group_count` must be writable, and a non-null
/// `group_properties` must provide the capacity supplied through `group_count`.
pub unsafe extern "system" fn vkEnumeratePhysicalDeviceGroupsKHR(
    instance: VkInstance,
    group_count: *mut u32,
    group_properties: *mut VkPhysicalDeviceGroupPropertiesKHR<'_>,
) -> VkResult {
    if group_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let Some(loader) = (unsafe { LoaderInstance::from_handle(instance) }) else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let dispatch = unsafe { &*loader.dispatch() };
    let Some(enumerate) = dispatch.vkEnumeratePhysicalDeviceGroupsKHR else {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    };
    debug_assert_eq!(
        core::mem::size_of::<VkPhysicalDeviceGroupPropertiesKHR<'_>>(),
        core::mem::size_of::<VkPhysicalDeviceGroupProperties<'_>>()
    );
    debug_assert_eq!(
        core::mem::align_of::<VkPhysicalDeviceGroupPropertiesKHR<'_>>(),
        core::mem::align_of::<VkPhysicalDeviceGroupProperties<'_>>()
    );
    // SAFETY: The promoted KHR command and property structure have the same ABI
    // as their core aliases, as asserted above.
    let enumerate: vk::PFN_vkEnumeratePhysicalDeviceGroups = unsafe {
        core::mem::transmute::<
            vk::PFN_vkEnumeratePhysicalDeviceGroupsKHR,
            vk::PFN_vkEnumeratePhysicalDeviceGroups,
        >(enumerate)
    };
    unsafe {
        enumerate_physical_device_groups_entry(
            loader,
            dispatch,
            enumerate,
            instance,
            group_count,
            group_properties.cast(),
        )
    }
}

/// Reports the device layers active on the physical device's instance.
///
/// # Safety
///
/// Arguments must satisfy `vkEnumerateDeviceLayerProperties`' Vulkan contract.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumerateDeviceLayerProperties(
    physical_device: VkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut VkLayerProperties,
) -> VkResult {
    let _loader_guard = platform::lock_loader();
    if property_count.is_null() {
        return VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: Every live physical-device wrapper carries the owning instance dispatch key.
    let instance = unsafe { LoaderInstance::from_dispatchable(physical_device.0.cast()) }
        .unwrap_or_else(|| {
            fatal_loader_error(
                c"vkEnumerateDeviceLayerProperties: Invalid physicalDevice [VUID-vkEnumerateDeviceLayerProperties-physicalDevice-parameter]",
            )
        });
    // SAFETY: Forward the caller's enumeration storage contract.
    unsafe {
        layer::enumerate_active_device_layers(
            &instance.active_layer_properties,
            &mut *property_count,
            properties,
        )
    }
}

/// Enumerates ICD extensions or the extensions declared by a named layer.
///
/// # Safety
///
/// Arguments must satisfy `vkEnumerateDeviceExtensionProperties`' Vulkan contract.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vkEnumerateDeviceExtensionProperties(
    physical_device: VkPhysicalDevice,
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut VkExtensionProperties,
) -> VkResult {
    let _loader_guard = platform::lock_loader();
    let (dispatch, physical_device) = unsafe {
        resolve_trampoline_physical_device(physical_device)
    }
    .unwrap_or_else(|| {
        fatal_loader_error(
            c"vkEnumerateDeviceExtensionProperties: Invalid physicalDevice [VUID-vkEnumerateDeviceExtensionProperties-physicalDevice-parameter]",
        )
    });
    let command = dispatch.vkEnumerateDeviceExtensionProperties;
    command.map_or_else(
        || unsafe {
            layer::terminator_enumerate_device_extension_properties(
                physical_device,
                layer_name,
                property_count,
                properties,
            )
        },
        |command| unsafe { command(physical_device, layer_name, property_count, properties) },
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct NativePhysicalDevice {
    pub(super) icd_index: usize,
    pub(super) handle: VkPhysicalDevice,
}

pub(crate) struct DeviceConfigurationProperties {
    properties: vk::VkPhysicalDeviceProperties2<'static>,
    identifiers: vk::VkPhysicalDeviceIDProperties<'static>,
    driver: vk::VkPhysicalDeviceDriverProperties<'static>,
}

#[derive(Clone, Copy)]
pub(crate) struct LinuxSortedDeviceInfo {
    device: NativePhysicalDevice,
    device_name: [c_char; vk::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    device_type: vk::VkPhysicalDeviceType,
    vendor_id: u32,
    device_id: u32,
    pci: Option<(u32, u32, u32, u32)>,
    default_device: bool,
    original_order: usize,
}

pub(crate) struct LinuxSortPropertyStorage {
    basic: vk::VkPhysicalDeviceProperties,
    properties2: vk::VkPhysicalDeviceProperties2<'static>,
    pci: vk::VkPhysicalDevicePCIBusInfoPropertiesEXT<'static>,
}

pub(crate) const LINUX_SORT_PLATFORM_ENABLED: bool = cfg!(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "hurd"
));

pub(crate) const fn linux_sort_requires_properties_extension(
    application_api_version: u32,
    driver_api_version: u32,
) -> bool {
    application_api_version < vk::VK_API_VERSION_1_1 || driver_api_version < vk::VK_API_VERSION_1_1
}

pub(crate) fn decimal_prefix_nonzero(bytes: &[u8]) -> bool {
    let mut index = bytes
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let negative = bytes.get(index) == Some(&b'-');
    if matches!(bytes.get(index), Some(b'-' | b'+')) {
        index += 1;
    }

    let limit = if negative {
        libc::c_long::MAX as u64 + 1
    } else {
        libc::c_long::MAX as u64
    };
    let mut magnitude = 0_u64;
    while let Some(digit) = bytes.get(index).and_then(|byte| match byte {
        b'0'..=b'9' => Some(u64::from(byte - b'0')),
        _ => None,
    }) {
        magnitude = magnitude
            .saturating_mul(10)
            .saturating_add(digit)
            .min(limit);
        index += 1;
    }

    let value = if negative {
        if magnitude == limit {
            libc::c_long::MIN
        } else {
            -(magnitude as libc::c_long)
        }
    } else {
        magnitude as libc::c_long
    };
    let low = value.cast_unsigned() & libc::c_ulong::from(u32::MAX);
    low != 0
}

pub(crate) fn scanf_hex(bytes: &[u8], mut index: usize) -> Option<(u32, usize)> {
    const C_ULONG_MAX: u128 = if core::mem::size_of::<libc::c_ulong>() == 4 {
        0xffff_ffff
    } else {
        0xffff_ffff_ffff_ffff
    };

    while bytes.get(index).is_some_and(u8::is_ascii_whitespace) {
        index += 1;
    }
    let negative = bytes.get(index) == Some(&b'-');
    if matches!(bytes.get(index), Some(b'-' | b'+')) {
        index += 1;
    }
    if bytes.get(index) == Some(&b'0')
        && matches!(bytes.get(index + 1), Some(b'x' | b'X'))
        && bytes.get(index + 2).is_some_and(u8::is_ascii_hexdigit)
    {
        index += 2;
    }
    let digit_start = index;
    let mut magnitude = 0_u128;
    let mut overflow = false;
    while let Some(digit) = bytes.get(index).and_then(|byte| match byte {
        b'0'..=b'9' => Some(u128::from(byte - b'0')),
        b'a'..=b'f' => Some(u128::from(byte - b'a') + 10),
        b'A'..=b'F' => Some(u128::from(byte - b'A') + 10),
        _ => None,
    }) {
        match magnitude
            .checked_mul(16)
            .and_then(|value| value.checked_add(digit))
            .filter(|&value| value <= C_ULONG_MAX)
        {
            Some(value) if !overflow => magnitude = value,
            _ => {
                magnitude = C_ULONG_MAX;
                overflow = true;
            }
        }
        index += 1;
    }
    let magnitude = magnitude as libc::c_ulong;
    let value = if negative && !overflow {
        magnitude.wrapping_neg()
    } else {
        magnitude
    };
    let value = (value & libc::c_ulong::from(u32::MAX)) as u32;
    (index != digit_start).then_some((value, index))
}

pub(crate) fn parse_linux_device_selection(bytes: &[u8]) -> Option<(u32, u32)> {
    let (vendor, index) = scanf_hex(bytes, 0)?;
    if bytes.get(index) != Some(&b':') {
        return None;
    }
    let (device, _) = scanf_hex(bytes, index + 1)?;
    Some((vendor, device))
}

pub(crate) fn linux_sort_enabled(instance: &LoaderInstance) -> Result<bool, VkResult> {
    if !LINUX_SORT_PLATFORM_ENABLED {
        return Ok(false);
    }
    // SAFETY: Loader configuration reads follow upstream's exclusion of
    // concurrent environment mutation, including from allocation callbacks.
    // Unicode replacement cannot change this ASCII numeric prefix: either
    // representation stops parsing at the first invalid/non-ASCII unit.
    if unsafe {
        platform::inspect_environment_lossy(c"VK_LOADER_DISABLE_SELECT", |value| {
            value.is_some_and(|value| decimal_prefix_nonzero(value.as_bytes()))
        })
    }? {
        return Ok(false);
    }
    Ok(instance.api_version >= vk::VK_API_VERSION_1_1
        || instance
            .enabled_extensions
            .contains(generated::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION_ID)
        || instance.active_icds().any(|(_, icd)| {
            icd.enabled_extensions
                .contains(generated::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION_ID)
        }))
}

pub(crate) fn linux_device_type_priority(device_type: vk::VkPhysicalDeviceType) -> u8 {
    match device_type {
        vk::VkPhysicalDeviceType::DISCRETE_GPU => 10,
        vk::VkPhysicalDeviceType::INTEGRATED_GPU => 5,
        vk::VkPhysicalDeviceType::VIRTUAL_GPU => 3,
        vk::VkPhysicalDeviceType::OTHER => 2,
        vk::VkPhysicalDeviceType::CPU => 1,
        _ => 0,
    }
}

#[inline(never)]
pub(crate) fn compare_linux_devices(
    left: &LinuxSortedDeviceInfo,
    right: &LinuxSortedDeviceInfo,
) -> core::cmp::Ordering {
    match (left.default_device, right.default_device) {
        (true, false) => return Ordering::Less,
        (false, true) => return Ordering::Greater,
        _ => {}
    }
    let type_order = linux_device_type_priority(right.device_type)
        .cmp(&linux_device_type_priority(left.device_type));
    if type_order != Ordering::Equal {
        return type_order;
    }
    match (left.pci, right.pci) {
        (Some(left), Some(right)) => {
            let order = left.cmp(&right);
            if order != Ordering::Equal {
                return order;
            }
        }
        (Some(_), None) => return Ordering::Less,
        (None, Some(_)) => return Ordering::Greater,
        (None, None) => {}
    }
    (left.device_id ^ left.vendor_id)
        .cmp(&(right.device_id ^ right.vendor_id))
        .then_with(|| left.original_order.cmp(&right.original_order))
}

pub(crate) fn heap_sort_by<T>(values: &mut [T], compare: impl Fn(&T, &T) -> core::cmp::Ordering) {
    fn sift_down<T>(
        values: &mut [T],
        mut root: usize,
        end: usize,
        compare: &impl Fn(&T, &T) -> core::cmp::Ordering,
    ) {
        loop {
            let left = root.saturating_mul(2).saturating_add(1);
            if left >= end {
                return;
            }
            let right = left + 1;
            let child = if right < end
                && compare(&values[left], &values[right]) == core::cmp::Ordering::Less
            {
                right
            } else {
                left
            };
            if compare(&values[root], &values[child]) != core::cmp::Ordering::Less {
                return;
            }
            values.swap(root, child);
            root = child;
        }
    }

    for root in (0..values.len() / 2).rev() {
        sift_down(values, root, values.len(), &compare);
    }
    for end in (1..values.len()).rev() {
        values.swap(0, end);
        sift_down(values, 0, end, &compare);
    }
}

pub(crate) unsafe fn icd_supports_device_extension(
    icd: &IcdInstance,
    physical_device: VkPhysicalDevice,
    name: &CStr,
) -> Result<bool, VkResult> {
    let Some(enumerate) = icd.dispatch.vkEnumerateDeviceExtensionProperties else {
        return Ok(false);
    };
    let mut count = 0;
    // Upstream intentionally ignores the driver's result here and uses the
    // returned count. GPU sorting is auxiliary and must not turn a device
    // extension enumeration error into a physical-device enumeration error.
    let _ = unsafe {
        enumerate(
            physical_device,
            ptr::null(),
            &raw mut count,
            ptr::null_mut(),
        )
    };
    let capacity = count as usize;
    if capacity == 0 {
        return Ok(false);
    }
    let mut properties = Vec::new();
    properties
        .try_reserve_exact(capacity)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    properties.resize(capacity, VkExtensionProperties::DEFAULT);
    // Match loader_linux.c: bound reads by the allocated count regardless of
    // the result or a driver's over-reported second-call count.
    let _ = unsafe {
        enumerate(
            physical_device,
            ptr::null(),
            &raw mut count,
            properties.as_mut_ptr(),
        )
    };
    Ok(properties[..(count as usize).min(capacity)]
        .iter()
        .any(|property| unsafe { CStr::from_ptr(property.extensionName.as_ptr()) == name }))
}

pub(crate) unsafe fn linux_sorted_device_info(
    instance: &LoaderInstance,
    device: NativePhysicalDevice,
    storage: *mut LinuxSortPropertyStorage,
    needs_pci_order: bool,
) -> Result<LinuxSortedDeviceInfo, VkResult> {
    let icd = &instance.icds[device.icd_index];
    let Some(get_properties) = icd.dispatch.vkGetPhysicalDeviceProperties else {
        return Err(VkResult::ERROR_INITIALIZATION_FAILED);
    };
    unsafe { get_properties(device.handle, ptr::addr_of_mut!((*storage).basic)) };
    let device_type = unsafe { ptr::addr_of!((*storage).basic.deviceType).read() };
    let device_name = unsafe { ptr::addr_of!((*storage).basic.deviceName).read() };
    let api_version = unsafe { ptr::addr_of!((*storage).basic.apiVersion).read() };
    let vendor_id = unsafe { ptr::addr_of!((*storage).basic.vendorID).read() };
    let device_id = unsafe { ptr::addr_of!((*storage).basic.deviceID).read() };
    // PCI addresses only break ordering ties between multiple devices.
    let has_pci = needs_pci_order
        && unsafe {
            icd_supports_device_extension(
                icd,
                device.handle,
                vk::VK_EXT_PCI_BUS_INFO_EXTENSION_NAME,
            )
        }?;
    let pci = if has_pci {
        unsafe {
            ptr::addr_of_mut!((*storage).properties2.sType)
                .write(vk::VkStructureType::PHYSICAL_DEVICE_PROPERTIES_2);
            ptr::addr_of_mut!((*storage).properties2.pNext)
                .write(ptr::addr_of_mut!((*storage).pci).cast());
            ptr::addr_of_mut!((*storage).pci)
                .write(vk::VkPhysicalDevicePCIBusInfoPropertiesEXT::DEFAULT);
        }
        let queried = if instance.api_version >= vk::VK_API_VERSION_1_1
            && api_version >= vk::VK_API_VERSION_1_1
        {
            icd.dispatch
                .vkGetPhysicalDeviceProperties2
                .map(|query| unsafe {
                    query(device.handle, ptr::addr_of_mut!((*storage).properties2));
                })
        } else {
            debug_assert_eq!(
                core::mem::size_of::<vk::VkPhysicalDeviceProperties2KHR<'_>>(),
                core::mem::size_of::<vk::VkPhysicalDeviceProperties2<'_>>()
            );
            debug_assert_eq!(
                core::mem::align_of::<vk::VkPhysicalDeviceProperties2KHR<'_>>(),
                core::mem::align_of::<vk::VkPhysicalDeviceProperties2<'_>>()
            );
            icd.dispatch
                .vkGetPhysicalDeviceProperties2KHR
                .map(|query| unsafe {
                    query(
                        device.handle,
                        ptr::addr_of_mut!((*storage).properties2).cast(),
                    );
                })
        };
        queried.map(|()| unsafe {
            (
                ptr::addr_of!((*storage).pci.pciDomain).read(),
                ptr::addr_of!((*storage).pci.pciBus).read(),
                ptr::addr_of!((*storage).pci.pciDevice).read(),
                ptr::addr_of!((*storage).pci.pciFunction).read(),
            )
        })
    } else {
        None
    };
    Ok(LinuxSortedDeviceInfo {
        device,
        device_name,
        device_type,
        vendor_id,
        device_id,
        pci,
        default_device: false,
        original_order: 0,
    })
}

pub(crate) fn selected_linux_device() -> Result<Option<(u32, u32)>, VkResult> {
    // SAFETY: Loader configuration reads follow upstream's exclusion of
    // concurrent environment mutation, including from allocation callbacks.
    unsafe {
        platform::inspect_environment_lossy(c"VK_LOADER_DEVICE_SELECT", |selection| {
            selection.and_then(|selection| parse_linux_device_selection(selection.as_bytes()))
        })
    }
}

#[cold]
#[inline(never)]
pub(crate) unsafe fn linux_sort_physical_devices(
    instance: &LoaderInstance,
    devices: &mut [NativePhysicalDevice],
) -> Result<(), VkResult> {
    let mut storage = allocation::try_box_uninit::<LinuxSortPropertyStorage>()?;
    let storage = storage.as_mut_ptr();
    let mut sorted = Vec::new();
    sorted
        .try_reserve_exact(devices.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    emit_instance_loader_category_message(
        instance,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        platform::LogFilter::Driver,
        format_args!("linux_read_sorted_physical_devices:"),
    );
    emit_instance_loader_category_message(
        instance,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        platform::LogFilter::Driver,
        format_args!("     Original order:"),
    );
    for (original_order, &device) in devices.iter().enumerate() {
        let mut info =
            unsafe { linux_sorted_device_info(instance, device, storage, devices.len() > 1) }?;
        info.original_order = original_order;
        let name = unsafe { CStr::from_ptr(info.device_name.as_ptr()) };
        let name = debug::diagnostics::LossyBytes(name.to_bytes());
        emit_instance_loader_category_message(
            instance,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            platform::LogFilter::Driver,
            format_args!("           [{original_order}] {name}"),
        );
        sorted.push(info);
    }
    if let Some((vendor_id, device_id)) = selected_linux_device()?
        && let Some(selected) = sorted
            .iter_mut()
            .find(|device| device.vendor_id == vendor_id && device.device_id == device_id)
    {
        selected.default_device = true;
    }
    heap_sort_by(&mut sorted, compare_linux_devices);
    emit_instance_loader_category_message(
        instance,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        platform::LogFilter::Driver,
        format_args!("     Sorted order:"),
    );
    for (index, (output, sorted)) in devices.iter_mut().zip(sorted).enumerate() {
        let name = unsafe { CStr::from_ptr(sorted.device_name.as_ptr()) };
        let name = debug::diagnostics::LossyBytes(name.to_bytes());
        let default = if sorted.default_device {
            "[default]"
        } else {
            ""
        };
        emit_instance_loader_category_message(
            instance,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            platform::LogFilter::Driver,
            format_args!(
                "           [{index}] {name}  {}",
                debug::diagnostics::Text(default)
            ),
        );
        *output = sorted.device;
    }
    Ok(())
}

pub(crate) struct LinuxSortableGroup {
    group_index: usize,
    devices: core::ops::Range<usize>,
    original_order: usize,
}

#[inline(never)]
fn compare_linux_groups(
    left: &LinuxSortableGroup,
    right: &LinuxSortableGroup,
    devices: &[LinuxSortedDeviceInfo],
) -> Ordering {
    match (
        devices[left.devices.clone()].first(),
        devices[right.devices.clone()].first(),
    ) {
        (Some(left), Some(right)) => compare_linux_devices(left, right),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
    .then_with(|| left.original_order.cmp(&right.original_order))
}

unsafe fn linux_sort_physical_device_groups(
    instance: &LoaderInstance,
    groups: &mut NativeGroups,
) -> Result<(), VkResult> {
    let selected = selected_linux_device()?;
    if groups.len() <= 1
        && groups
            .first()
            .is_none_or(|(_, group)| group.physicalDeviceCount <= 1)
        && !instance.wants_loader_category_message(
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            platform::LogFilter::Driver,
        )
    {
        // Neither device nor group order can change. Property queries are
        // only needed here to produce diagnostics for an interested recipient.
        return Ok(());
    }
    unsafe { sort_physical_device_groups_with_diagnostics(instance, groups, selected) }
}

#[cold]
unsafe fn sort_physical_device_groups_with_diagnostics(
    instance: &LoaderInstance,
    groups: &mut NativeGroups,
    selected: Option<(u32, u32)>,
) -> Result<(), VkResult> {
    let mut storage = allocation::try_box_uninit::<LinuxSortPropertyStorage>()?;
    let storage = storage.as_mut_ptr();
    let mut sortable = Vec::new();
    sortable
        .try_reserve_exact(groups.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let device_count = groups
        .iter()
        .try_fold(0_usize, |count, (_, group)| {
            count.checked_add(
                (group.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize),
            )
        })
        .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let needs_pci_order = device_count > 1;
    let mut device_storage = Vec::new();
    device_storage
        .try_reserve_exact(device_count)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    emit_instance_loader_category_message(
        instance,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        platform::LogFilter::Driver,
        format_args!("linux_sort_physical_device_groups:  Original order:"),
    );
    for (group_index, (icd_index, properties)) in groups.iter_mut().enumerate() {
        emit_instance_loader_category_message(
            instance,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            platform::LogFilter::Driver,
            format_args!("           Group {group_index}"),
        );
        let device_count =
            (properties.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        properties.physicalDeviceCount = device_count as u32;
        let device_start = device_storage.len();
        for (device_order, &handle) in properties.physicalDevices[..device_count]
            .iter()
            .enumerate()
        {
            let mut info = unsafe {
                linux_sorted_device_info(
                    instance,
                    NativePhysicalDevice {
                        icd_index: *icd_index,
                        handle,
                    },
                    storage,
                    needs_pci_order,
                )
            }?;
            info.original_order = device_order;
            let name = unsafe { CStr::from_ptr(info.device_name.as_ptr()) };
            let name = debug::diagnostics::LossyBytes(name.to_bytes());
            emit_instance_loader_category_message(
                instance,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                platform::LogFilter::Driver,
                format_args!("               [{device_order}] {name}"),
            );
            device_storage.push(info);
        }
        let devices = &mut device_storage[device_start..];
        if let Some((vendor_id, device_id)) = selected
            && let Some(selected) = devices
                .iter_mut()
                .find(|device| device.vendor_id == vendor_id && device.device_id == device_id)
        {
            selected.default_device = true;
        }
        heap_sort_by(devices, compare_linux_devices);
        for (output, sorted) in properties.physicalDevices[..device_count]
            .iter_mut()
            .zip(devices.iter())
        {
            *output = sorted.device.handle;
        }
        sortable.push(LinuxSortableGroup {
            group_index,
            devices: device_start..device_storage.len(),
            original_order: group_index,
        });
    }
    heap_sort_by(&mut sortable, |left, right| {
        compare_linux_groups(left, right, &device_storage)
    });
    emit_sorted_physical_device_groups(instance, &sortable, &device_storage);
    reorder_groups(groups, &mut sortable);
    Ok(())
}

/// Applies the destination-to-source permutation while consuming its indices.
fn reorder_groups<T>(groups: &mut [T], order: &mut [LinuxSortableGroup]) {
    for start in 0..order.len() {
        let mut current = start;
        loop {
            let next = core::mem::replace(&mut order[current].group_index, current);
            if next == start {
                break;
            }
            groups.swap(current, next);
            current = next;
        }
    }
}

#[inline(never)]
pub(crate) unsafe fn discover_active_physical_devices(
    instance: &LoaderInstance,
) -> Result<Vec<NativePhysicalDevice>, VkResult> {
    unsafe { discover_active_physical_devices_with_diagnostics(instance, true) }
}

pub(crate) unsafe fn discover_active_physical_devices_with_diagnostics(
    instance: &LoaderInstance,
    emit_diagnostics: bool,
) -> Result<Vec<NativePhysicalDevice>, VkResult> {
    let devices = unsafe { discover_all_physical_devices(instance, true) }?;
    if emit_diagnostics && linux_sort_enabled(instance)? {
        let state = instance.physical_devices.lock();
        for (new_index, native) in devices.iter().enumerate() {
            let old_index = state.active.iter().position(|handle| {
                unsafe { LoaderPhysicalDevice::from_handle(*handle) }.is_some_and(|device| {
                    device.icd_index == native.icd_index && device.native == native.handle
                })
            });
            if let Some(old_index) = old_index {
                emit_instance_loader_category_message(
                    instance,
                    vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                    platform::LogFilter::Driver,
                    format_args!("Copying old device {old_index} into new device {new_index}"),
                );
            }
        }
    }
    let Some(configurations) = instance.device_configurations.as_deref() else {
        return Ok(devices);
    };

    if emit_diagnostics {
        emit_instance_loader_message(
            instance,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!(
                "Selecting and ordering VkPhysicalDevices to match the loader settings device configurations list"
            ),
        );
    }

    let mut ordered = Vec::new();
    ordered
        .try_reserve_exact(configurations.len())
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut matched = allocation::try_boxed_slice_filled(devices.len(), false)?;
    let mut query = allocation::try_box_uninit::<DeviceConfigurationProperties>()?;
    let query_pointer = query.as_mut_ptr();
    for configuration in configurations {
        let mut configuration_found = false;
        for (device_index, device) in devices.iter().enumerate() {
            if matched[device_index] {
                continue;
            }
            if let Some((driver_version, supports_driver_properties)) = unsafe {
                match_device_configuration(instance, device, query_pointer, configuration)
            }? {
                unsafe {
                    emit_configured_physical_device(
                        instance,
                        query_pointer,
                        ordered.len(),
                        supports_driver_properties,
                        driver_version,
                        emit_diagnostics,
                    );
                };
                ordered.push(*device);
                matched[device_index] = true;
                configuration_found = true;
                break;
            }
        }
        if !configuration_found && emit_diagnostics {
            emit_missing_device_configuration(instance, configuration);
        }
    }
    unsafe { emit_excluded_physical_devices(instance, &devices, &matched, emit_diagnostics) };
    if ordered.is_empty() {
        if emit_diagnostics {
            emit_instance_loader_message(
                instance,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
                format_args!(
                    "loader_apply_settings_device_configurations: None of the settings file device configurations had deviceUUID's that corresponded to enumerated VkPhysicalDevices. Returning VK_ERROR_INITIALIZATION_FAILED"
                ),
            );
        }
        Err(VkResult::ERROR_INITIALIZATION_FAILED)
    } else {
        Ok(ordered)
    }
}

#[cold]
#[inline(never)]
pub(crate) fn emit_instance_loader_message(
    instance: &LoaderInstance,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    message: core::fmt::Arguments<'_>,
) {
    let filter = platform::LogFilter::from_severity(severity);
    platform::write_loader_log(filter, message);
    instance.submit_loader_message_text(
        severity,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        message,
    );
}

#[cold]
#[inline(never)]
pub(crate) fn emit_instance_category_message(
    instance: &LoaderInstance,
    category_filters: &[platform::LogFilter],
    category_label: &str,
    message: core::fmt::Arguments<'_>,
) {
    platform::write_loader_category_log_any(category_filters, category_label, message);
    instance.submit_loader_message_text(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        message,
    );
}

#[cold]
#[inline(never)]
pub(crate) fn emit_instance_loader_category_message(
    instance: &LoaderInstance,
    severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    category: platform::LogFilter,
    message: core::fmt::Arguments<'_>,
) {
    platform::write_loader_log_with_category(
        platform::LogFilter::from_severity(severity),
        category,
        message,
    );
    instance.submit_loader_message_text(
        severity,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        message,
    );
}

pub(crate) struct IcdPhysicalDevices {
    storage: Box<[MaybeUninit<VkPhysicalDevice>]>,
    len: usize,
}

enum PhysicalDeviceEnumerationError {
    LoaderAllocation,
    Driver(VkResult),
}

impl PhysicalDeviceEnumerationError {
    const fn result(self) -> VkResult {
        match self {
            Self::LoaderAllocation => VkResult::ERROR_OUT_OF_HOST_MEMORY,
            Self::Driver(result) => result,
        }
    }
}

impl IcdPhysicalDevices {
    fn iter(&self) -> impl Iterator<Item = VkPhysicalDevice> + '_ {
        self.storage[..self.len].iter().map(|device| {
            // SAFETY: The ICD reported these leading elements as written.
            unsafe { device.assume_init() }
        })
    }
}

#[cold]
#[inline(never)]
unsafe fn enumerate_icd_physical_devices(
    instance: &IcdInstance,
) -> Result<IcdPhysicalDevices, PhysicalDeviceEnumerationError> {
    debug_assert!(instance.dispatch.vkEnumeratePhysicalDevices.is_some());
    let enumerate: PFN_vkEnumeratePhysicalDevices =
        instance.dispatch.vkEnumeratePhysicalDevices.ok_or(
            PhysicalDeviceEnumerationError::Driver(VkResult::ERROR_INITIALIZATION_FAILED),
        )?;
    let mut count = 0;
    // SAFETY: Count points to writable local storage.
    let result = unsafe { enumerate(instance.handle, &raw mut count, ptr::null_mut()) };
    if result != VkResult::SUCCESS {
        return Err(PhysicalDeviceEnumerationError::Driver(result));
    }
    let count = count as usize;
    let mut storage = allocation::try_box_uninit_slice::<VkPhysicalDevice>(count)
        .map_err(|_| PhysicalDeviceEnumerationError::LoaderAllocation)?;
    let mut returned_count = count.min(u32::MAX as usize) as u32;
    // SAFETY: `storage` has `count` writable elements.
    let result = unsafe {
        enumerate(
            instance.handle,
            &raw mut returned_count,
            storage.as_mut_ptr().cast(),
        )
    };
    if result != VkResult::SUCCESS && result != VkResult::INCOMPLETE {
        return Err(PhysicalDeviceEnumerationError::Driver(result));
    }
    Ok(IcdPhysicalDevices {
        storage,
        len: (returned_count as usize).min(count),
    })
}

#[cold]
fn emit_sorted_physical_device_groups(
    instance: &LoaderInstance,
    sortable: &[LinuxSortableGroup],
    devices: &[LinuxSortedDeviceInfo],
) {
    emit_instance_loader_category_message(
        instance,
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        platform::LogFilter::Driver,
        format_args!("linux_sort_physical_device_groups:  Sorted order:"),
    );
    for (group_index, group) in sortable.iter().enumerate() {
        emit_instance_loader_category_message(
            instance,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            platform::LogFilter::Driver,
            format_args!("           Group {group_index}"),
        );
        for (device_index, device) in devices[group.devices.clone()].iter().enumerate() {
            let name = unsafe { CStr::from_ptr(device.device_name.as_ptr()) };
            let name = debug::diagnostics::LossyBytes(name.to_bytes());
            let default = if device.default_device {
                "[default]"
            } else {
                ""
            };
            emit_instance_loader_category_message(
                instance,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                platform::LogFilter::Driver,
                format_args!(
                    "               [{device_index}] {name} {:p} {}",
                    device.device.handle.0,
                    debug::diagnostics::Text(default),
                ),
            );
        }
    }
}

#[cold]
fn emit_missing_device_configuration(
    instance: &LoaderInstance,
    configuration: &discovery::DeviceConfiguration,
) {
    let device_uuid = discovery::format_uuid(&configuration.device_uuid);
    let driver_uuid = discovery::format_uuid(&configuration.driver_uuid);
    let emit_identity = |identity: core::fmt::Arguments<'_>| {
        emit_instance_loader_message(
            instance,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING,
            format_args!(
                "loader_apply_settings_device_configurations: settings file contained device_configuration which does not appear in the enumerated VkPhysicalDevices. Missing VkPhysicalDevice with {identity}"
            ),
        );
    };
    match (
        configuration.device_name.as_deref(),
        configuration.driver_name.as_deref(),
    ) {
        (Some(device_name), Some(driver_name)) => emit_identity(format_args!(
            "deviceName: \"{}\", deviceUUID: {device_uuid}, driverName: {}, driverUUID: {driver_uuid}, driverVersion: {}",
            debug::diagnostics::Text(device_name),
            debug::diagnostics::Text(driver_name),
            configuration.driver_version
        )),
        (Some(device_name), None) => emit_identity(format_args!(
            "deviceName: \"{}\", deviceUUID: {device_uuid}, driverUUID: {driver_uuid}, driverVersion: {}",
            debug::diagnostics::Text(device_name),
            configuration.driver_version
        )),
        (None, _) => emit_identity(format_args!(
            "deviceUUID: {device_uuid}, driverUUID: {driver_uuid}, driverVersion: {}",
            configuration.driver_version
        )),
    }
}

#[cold]
unsafe fn emit_configured_physical_device(
    instance: &LoaderInstance,
    query_pointer: *const DeviceConfigurationProperties,
    index: usize,
    supports_driver_properties: bool,
    driver_version: u32,
    emit_diagnostics: bool,
) {
    let properties = unsafe { ptr::addr_of!((*query_pointer).properties.properties).read() };
    let device_name = unsafe { CStr::from_ptr(properties.deviceName.as_ptr()) };
    let device_name = debug::diagnostics::LossyBytes(device_name.to_bytes());
    let emit_detail = |detail: core::fmt::Arguments<'_>| {
        if emit_diagnostics {
            emit_instance_loader_message(
                instance,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::VERBOSE,
                detail,
            );
        }
    };
    if supports_driver_properties {
        let driver_name = unsafe {
            CStr::from_ptr(ptr::addr_of!((*query_pointer).driver.driverName).cast::<c_char>())
        };
        let driver_name = debug::diagnostics::LossyBytes(driver_name.to_bytes());
        emit_detail(format_args!(
            "pPhysicalDevices array index {index} is set to \"{device_name}\" ({driver_name}, version {driver_version}) "
        ));
    } else {
        emit_detail(format_args!(
            "pPhysicalDevices array index {index} is set to \"{device_name}\" (driver version {driver_version}) "
        ));
    }
}

#[cold]
unsafe fn emit_excluded_physical_devices(
    instance: &LoaderInstance,
    devices: &[NativePhysicalDevice],
    matched: &[bool],
    emit_diagnostics: bool,
) {
    for (device, matched) in devices.iter().zip(matched) {
        if *matched {
            continue;
        }
        let icd = &instance.icds[device.icd_index];
        let mut properties = vk::VkPhysicalDeviceProperties::DEFAULT;
        if let Some(get_properties) = icd.dispatch.vkGetPhysicalDeviceProperties {
            unsafe { get_properties(device.handle, &raw mut properties) };
        }
        let device_name = unsafe { CStr::from_ptr(properties.deviceName.as_ptr()) };
        let device_name = debug::diagnostics::LossyBytes(device_name.to_bytes());
        if emit_diagnostics {
            emit_instance_loader_message(
                instance,
                vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                format_args!(
                    "VkPhysicalDevice \"{device_name}\" did not appear in the settings file device configurations list, so was not added to the pPhysicalDevices array"
                ),
            );
        }
    }
}

unsafe fn collect_native_device_groups(
    instance: &LoaderInstance,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
    capacity: usize,
    upper_bound: u32,
) -> Result<NativeGroups, VkResult> {
    let mut native_groups = NativeGroups::default();
    if native_groups.try_reserve(upper_bound as usize).is_err() {
        return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
    }
    for (icd_index, icd) in instance.active_icds().rev() {
        let Some(enumerate) = icd_group_enumerator(instance, icd) else {
            continue;
        };
        // The ICD count can grow after the upper-bound query; the append
        // helper reserves additional storage before publishing any new groups.
        unsafe {
            enumerate_icd_groups(
                icd,
                enumerate,
                group_properties,
                capacity,
                native_groups.len(),
                icd_index,
                &mut native_groups,
            )?;
        }
    }
    Ok(native_groups)
}

unsafe fn match_device_configuration(
    instance: &LoaderInstance,
    device: &NativePhysicalDevice,
    query_pointer: *mut DeviceConfigurationProperties,
    configuration: &discovery::DeviceConfiguration,
) -> Result<Option<(u32, bool)>, VkResult> {
    let icd = &instance.icds[device.icd_index];
    let Some(get_properties2) = icd.dispatch.vkGetPhysicalDeviceProperties2 else {
        return Ok(None);
    };
    unsafe {
        ptr::addr_of_mut!((*query_pointer).properties.sType)
            .write(vk::VkStructureType::PHYSICAL_DEVICE_PROPERTIES_2);
        ptr::addr_of_mut!((*query_pointer).properties.pNext)
            .write(ptr::addr_of_mut!((*query_pointer).identifiers).cast());
        ptr::addr_of_mut!((*query_pointer).identifiers.sType)
            .write(vk::VkStructureType::PHYSICAL_DEVICE_ID_PROPERTIES);
        ptr::addr_of_mut!((*query_pointer).identifiers.pNext).write(ptr::null_mut());
        ptr::addr_of_mut!((*query_pointer).driver)
            .write(vk::VkPhysicalDeviceDriverProperties::DEFAULT);
        get_properties2(
            device.handle,
            ptr::addr_of_mut!((*query_pointer).properties),
        );
    }
    let api_version =
        unsafe { ptr::addr_of!((*query_pointer).properties.properties.apiVersion).read() };
    let driver_version =
        unsafe { ptr::addr_of!((*query_pointer).properties.properties.driverVersion).read() };
    let device_uuid = unsafe { ptr::addr_of!((*query_pointer).identifiers.deviceUUID).read() };
    let driver_uuid = unsafe { ptr::addr_of!((*query_pointer).identifiers.driverUUID).read() };
    let supports_driver_properties = api_version >= vk::VK_API_VERSION_1_2
        || unsafe {
            icd_supports_device_extension(
                icd,
                device.handle,
                vk::VK_KHR_DRIVER_PROPERTIES_EXTENSION_NAME,
            )
        }?;
    if supports_driver_properties {
        unsafe {
            ptr::addr_of_mut!((*query_pointer).identifiers.pNext)
                .write(ptr::addr_of_mut!((*query_pointer).driver).cast());
            get_properties2(
                device.handle,
                ptr::addr_of_mut!((*query_pointer).properties),
            );
        }
    }
    if api_version >= vk::VK_API_VERSION_1_1
        && driver_version == configuration.driver_version
        && device_uuid == configuration.device_uuid
        && driver_uuid == configuration.driver_uuid
    {
        Ok(Some((driver_version, supports_driver_properties)))
    } else {
        Ok(None)
    }
}

unsafe fn write_visible_device_groups(
    instance: &LoaderInstance,
    all_devices: &[NativePhysicalDevice],
    refresh_physical_devices: bool,
    native_groups: &mut NativeGroups,
    visible_devices: Option<&[NativePhysicalDevice]>,
    group_count: &mut u32,
    group_properties: *mut VkPhysicalDeviceGroupProperties<'_>,
) -> Result<VkResult, VkResult> {
    let mut state = instance.physical_devices.lock();
    if state.owned.try_reserve(all_devices.len()).is_err() {
        return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
    }
    for device in all_devices {
        let key = (device.icd_index, device.handle.0 as usize);
        if let collections::HashMapEntry::Vacant(entry) = state.owned.entry(key) {
            let physical_device = match allocation::try_box(LoaderPhysicalDevice::new(
                device.icd_index,
                &instance.icds[device.icd_index],
                instance,
                instance.api_version,
                device.handle,
            )) {
                Ok(device) => device,
                Err((result, _device)) => {
                    return Err(result);
                }
            };
            entry.insert(physical_device);
        }
    }
    if refresh_physical_devices {
        state.active.clear();
        if state.active.try_reserve_exact(all_devices.len()).is_err() {
            return Err(VkResult::ERROR_OUT_OF_HOST_MEMORY);
        }
        for device in all_devices {
            let key = (device.icd_index, device.handle.0 as usize);
            let handle = if let Some(device) = state.owned.get(&key) {
                device.handle()
            } else {
                return Err(VkResult::ERROR_INITIALIZATION_FAILED);
            };
            state.active.push(handle);
        }
    }

    let mut visible_count = 0;
    'groups: for group_index in 0..native_groups.len() {
        let (icd_index, mut properties) = native_groups[group_index];
        let device_count =
            (properties.physicalDeviceCount as usize).min(vk::VK_MAX_DEVICE_GROUP_SIZE as usize);
        properties.physicalDeviceCount = device_count as u32;
        for native in &mut properties.physicalDevices[..device_count] {
            if let Some(visible) = visible_devices
                && !visible
                    .iter()
                    .any(|device| device.icd_index == icd_index && device.handle == *native)
            {
                if !group_properties.is_null() {
                    emit_instance_loader_message(
                        instance,
                        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
                        format_args!(
                            "terminator_EnumeratePhysicalDeviceGroups: Physical device group {group_index} contains a VkPhysicalDevice which the settings file device configurations exclude, so the group was not reported."
                        ),
                    );
                }
                continue 'groups;
            }
            let key = (icd_index, native.0 as usize);
            let Some(wrapped) = state.owned.get(&key) else {
                return Err(VkResult::ERROR_INITIALIZATION_FAILED);
            };
            *native = wrapped.handle();
        }
        native_groups[visible_count] = (icd_index, properties);
        visible_count += 1;
    }

    let written = (*group_count as usize).min(visible_count);
    for (index, (_, properties)) in native_groups.iter().take(written).enumerate() {
        unsafe { group_properties.add(index).write(*properties) };
    }
    *group_count = written as u32;
    if written < visible_count {
        emit_instance_loader_message(
            instance,
            vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
            format_args!(
                "terminator_EnumeratePhysicalDeviceGroups : Trimming device count from {visible_count} to {written}."
            ),
        );
        Ok(VkResult::INCOMPLETE)
    } else {
        Ok(VkResult::SUCCESS)
    }
}

#[cfg(test)]
mod allocation_tests {
    #[cfg(unix)]
    use std::os::unix::ffi::OsStrExt as _;
    #[cfg(windows)]
    use std::os::windows::ffi::OsStringExt as _;

    #[cfg(any(unix, windows))]
    use crate::allocation::fault;

    #[test]
    fn native_group_growth_preserves_entries_on_allocation_failure() {
        for count in [0, 1, 2, 8] {
            crate::allocation::fault::sweep_operation(|| {
                let mut groups = super::NativeGroups::default();
                for index in 0..count {
                    if let Err(result) = groups.try_reserve(1) {
                        assert_eq!(groups.len(), index);
                        for (expected, (actual, _)) in groups.iter().enumerate() {
                            assert_eq!(*actual, expected);
                        }
                        return result;
                    }
                    groups.push(index, &vk::VkPhysicalDeviceGroupProperties::DEFAULT);
                }
                assert_eq!(groups.len(), count);
                for (expected, (actual, _)) in groups.iter().enumerate() {
                    assert_eq!(*actual, expected);
                }
                vk::VkResult::SUCCESS
            });
        }
    }

    #[test]
    fn group_permutation_preserves_cycles_and_fixed_points() {
        for indices in [
            vec![],
            vec![0],
            vec![2, 0, 1],
            vec![1, 0, 3, 2, 4],
            vec![4, 3, 2, 1, 0],
        ] {
            let mut groups: Vec<_> = (0..indices.len()).collect();
            let mut order: Vec<_> = indices
                .iter()
                .map(|&index| super::LinuxSortableGroup {
                    group_index: index,
                    devices: 0..0,
                    original_order: index,
                })
                .collect();
            super::reorder_groups(&mut groups, &mut order);
            assert_eq!(groups, indices);
        }
    }

    #[cfg(unix)]
    #[test]
    fn id_filter_lossy_conversion_is_fallible_and_preserves_ranges() {
        for (bytes, begin, end) in [
            (b"1-2".as_slice(), 1, 2),
            (b"1\xff2".as_slice(), 1, 0),
            (b"\xff".as_slice(), 0, 0),
        ] {
            fault::sweep_operation(|| {
                let mut filter = super::IdFilter::default();
                match super::IdFilter::parse_into(std::ffi::OsStr::from_bytes(bytes), &mut filter) {
                    Ok(()) => {
                        assert_eq!(filter.len, 1);
                        assert_eq!(filter.ranges[0].begin, begin);
                        assert_eq!(filter.ranges[0].end, end);
                        vk::VkResult::SUCCESS
                    }
                    Err(result) => result,
                }
            });
        }
    }

    #[cfg(windows)]
    #[test]
    fn id_filter_unpaired_surrogate_conversion_is_fallible() {
        let value = std::ffi::OsString::from_wide(&[0x31, 0xd800, 0x32]);
        fault::sweep_operation(|| {
            let mut filter = super::IdFilter::default();
            match super::IdFilter::parse_into(&value, &mut filter) {
                Ok(()) => {
                    assert_eq!(filter.len, 1);
                    assert_eq!(filter.ranges[0].begin, 1);
                    assert_eq!(filter.ranges[0].end, 0);
                    vk::VkResult::SUCCESS
                }
                Err(result) => result,
            }
        });
    }
}
