//! dispatch implementation.

use crate::{
    CStr, InstanceDispatchTable, LayerDeviceDispatchTable, LayerInstanceDispatchTable,
    LoaderPhysicalDevice, LoaderPhysicalDeviceTrampoline, PFN_vkVoidFunction, VkDeviceCreateInfo,
    VkDeviceGroupDeviceCreateInfo, VkPhysicalDevice, VkResult, VkStructureType, allocation, c_void,
    platform,
};

pub(crate) const DEVICE_DISPATCH_MAGIC: u64 = 0x10AD_ED04_0410_ADED;

pub(crate) struct DeviceGroupChainPatch<'a> {
    _group: Box<VkDeviceGroupDeviceCreateInfo<'a>>,
    _physical_devices: Vec<VkPhysicalDevice>,
    restore: Option<(*mut *const c_void, *const c_void)>,
}

impl Drop for DeviceGroupChainPatch<'_> {
    fn drop(&mut self) {
        if let Some((field, original)) = self.restore {
            // SAFETY: The patched predecessor belongs to the synchronous,
            // caller-owned create chain and remains live until this guard drops.
            unsafe { field.write(original) };
        }
    }
}

pub(crate) unsafe fn translate_device_group_chain<'a>(
    create_info: &mut VkDeviceCreateInfo<'a>,
    mut translate: impl FnMut(VkPhysicalDevice) -> Option<VkPhysicalDevice>,
) -> Result<Option<DeviceGroupChainPatch<'a>>, VkResult> {
    let root_next = &raw mut create_info.pNext;
    let mut predecessor = root_next;
    let mut current = create_info.pNext.cast::<vk::VkBaseInStructure<'a>>();
    while !current.is_null() {
        // SAFETY: The Vulkan input-chain contract makes the common header readable.
        let header = unsafe { &*current };
        if header.sType == VkStructureType::DEVICE_GROUP_DEVICE_CREATE_INFO {
            // SAFETY: The matching sType identifies this concrete structure.
            let source = unsafe { &*current.cast::<VkDeviceGroupDeviceCreateInfo<'a>>() };
            if source.physicalDeviceCount == 0 || source.pPhysicalDevices.is_null() {
                return Ok(None);
            }
            let count = source.physicalDeviceCount as usize;
            let mut devices = Vec::new();
            devices
                .try_reserve_exact(count)
                .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
            for index in 0..count {
                // SAFETY: The source array contains physicalDeviceCount handles.
                let handle = unsafe { source.pPhysicalDevices.add(index).read() };
                devices.push(translate(handle).ok_or(VkResult::ERROR_INITIALIZATION_FAILED)?);
            }
            let mut group = allocation::try_box(*source).map_err(|(result, _group)| result)?;
            group.pPhysicalDevices = devices.as_ptr();
            let replacement = core::ptr::from_ref(group.as_ref()).cast::<c_void>();
            let original = current.cast::<c_void>();
            // SAFETY: predecessor is either the local root pNext field or the
            // writable pNext field of a live chain node, matching upstream.
            unsafe { predecessor.write(replacement) };
            let restore = (predecessor != root_next).then_some((predecessor, original));
            return Ok(Some(DeviceGroupChainPatch {
                _group: group,
                _physical_devices: devices,
                restore,
            }));
        }
        predecessor = unsafe { core::ptr::addr_of!((*current).pNext) }
            .cast::<*const c_void>()
            .cast_mut();
        current = header.pNext;
    }
    Ok(None)
}

pub(crate) union FunctionPointer<T: Copy> {
    erased: unsafe extern "system" fn(),
    typed: T,
}

pub(crate) const fn erase_function<T: Copy>(typed: T) -> unsafe extern "system" fn() {
    // SAFETY: Vulkan function pointers use a common representation.
    unsafe { FunctionPointer { typed }.erased }
}

pub(crate) const unsafe fn load_typed<T: Copy>(function: PFN_vkVoidFunction) -> Option<T> {
    // SAFETY: Vulkan requires compatible representations for all command
    // pointers returned by its proc-address functions.
    match function {
        Some(function) => Some(unsafe { FunctionPointer { erased: function }.typed }),
        None => None,
    }
}

#[inline]
pub(crate) unsafe fn instance_dispatch<'a>(
    dispatchable: *mut c_void,
) -> Option<&'a LayerInstanceDispatchTable> {
    if dispatchable.is_null() {
        return None;
    }
    // SAFETY: Every live Vulkan instance-scope dispatchable begins with its
    // loader dispatch-table pointer.
    let dispatch = unsafe {
        dispatchable
            .cast::<*const LayerInstanceDispatchTable>()
            .read()
    };
    // SAFETY: The dispatch allocation is retained by the owning instance.
    unsafe { dispatch.as_ref() }
}

#[derive(Clone, Copy)]
pub(crate) enum MissingPhysicalDeviceCommand {
    Error,
    DisplayWarning,
    AcquireDrmDisplay,
    GetDrmDisplay,
}

#[inline]
pub(crate) unsafe fn resolve_physical_device<T: Copy>(
    physical_device: VkPhysicalDevice,
    resolve: impl FnOnce(&InstanceDispatchTable) -> Option<T>,
    name: &CStr,
    missing: MissingPhysicalDeviceCommand,
) -> Option<(T, VkPhysicalDevice)> {
    // SAFETY: Generated terminators receive the live internal handle installed
    // by this loader in the physical-device layer chain.
    let physical_device = unsafe { LoaderPhysicalDevice::from_terminator_handle(physical_device) };
    let icd = physical_device.icd();
    let command = resolve(&icd.dispatch);
    let Some(command) = command else {
        report_missing_physical_device_command(physical_device, name, missing);
        return None;
    };
    Some((command, physical_device.native))
}

#[cold]
#[inline(never)]
pub(crate) fn report_missing_physical_device_command(
    physical_device: &LoaderPhysicalDevice,
    name: &CStr,
    missing: MissingPhysicalDeviceCommand,
) {
    use vk::VkDebugUtilsMessageSeverityFlagBitsEXT as Severity;

    let instance = physical_device.instance();
    match missing {
        MissingPhysicalDeviceCommand::DisplayWarning => instance.log_loader_message_text(
            Severity::WARNING,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
            format_args!(
                "ICD for selected physical device does not export {}!",
                crate::debug::diagnostics::LossyBytes(name.to_bytes())
            ),
        ),
        MissingPhysicalDeviceCommand::AcquireDrmDisplay => instance.log_loader_message_text(
            Severity::ERROR,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
            format_args!(
                "ICD associated with VkPhysicalDevice does not support AcquireDrmDisplayEXT"
            ),
        ),
        MissingPhysicalDeviceCommand::GetDrmDisplay => instance.log_loader_message_text(
            Severity::ERROR,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
            format_args!("ICD associated with VkPhysicalDevice does not support GetDrmDisplayEXT"),
        ),
        MissingPhysicalDeviceCommand::Error => instance.log_loader_message_text(
            Severity::ERROR,
            vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
            format_args!(
                "ICD for selected physical device does not export {}!",
                crate::debug::diagnostics::LossyBytes(name.to_bytes())
            ),
        ),
    }
}

pub(crate) unsafe fn resolve_trampoline_physical_device(
    physical_device: VkPhysicalDevice,
) -> Option<(&'static LayerInstanceDispatchTable, VkPhysicalDevice)> {
    let trampoline = unsafe { LoaderPhysicalDeviceTrampoline::from_handle(physical_device) }?;
    let dispatch = unsafe { instance_dispatch(physical_device.0.cast()) }?;
    Some((dispatch, trampoline.chain))
}

#[inline]
pub(crate) unsafe fn device_dispatch(
    handle: *mut c_void,
) -> Option<&'static LayerDeviceDispatchTable> {
    if handle.is_null() {
        return None;
    }
    // SAFETY: Every live Vulkan device dispatchable stores a dispatch-table
    // pointer in its first machine word.
    let dispatch = unsafe { handle.cast::<*const LayerDeviceDispatchTable>().read() };
    if dispatch.is_null() {
        return None;
    }
    // SAFETY: A live dispatchable's first word points to readable dispatch data.
    (unsafe { (*dispatch).magic } == DEVICE_DISPATCH_MAGIC).then(|| unsafe { &*dispatch })
}

#[cold]
#[inline(never)]
pub(crate) fn invalid_device_dispatch() -> ! {
    core::hint::cold_path();
    // SAFETY: A corrupted dispatchable handle is an unrecoverable loader ABI
    // violation and upstream terminates the process on this path.
    unsafe { libc::abort() }
}

#[cold]
#[inline(never)]
pub(crate) fn fatal_loader_error(message: &CStr) -> ! {
    core::hint::cold_path();
    platform::write_stderr_fmt(format_args!(
        "{}\n",
        crate::debug::diagnostics::LossyBytes(message.to_bytes())
    ));
    // SAFETY: A missing required driver entry point is fatal by loader ABI
    // contract; no Rust cleanup can make continued execution valid.
    unsafe { libc::abort() }
}

pub(crate) unsafe fn set_device_dispatchable(
    object: *mut c_void,
    dispatch: *const LayerDeviceDispatchTable,
) {
    if !object.is_null() {
        // SAFETY: The caller passes a live writable dispatchable object.
        unsafe {
            object
                .cast::<*const LayerDeviceDispatchTable>()
                .write(dispatch);
        };
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CommandScope {
    Global,
    Instance,
    Device,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CommandLookup {
    pub(crate) id: u16,
    pub(crate) scope: CommandScope,
}

#[derive(Clone, Copy)]
pub(crate) struct CommandRecord {
    pub(crate) name_offset: u16,
    pub(crate) id: u16,
    pub(crate) name_len: u8,
    pub(crate) scope: CommandScope,
}

#[derive(Clone, Copy)]
pub(crate) struct CommandProviderRange {
    pub(crate) offset: u16,
    pub(crate) len: u8,
}

pub(crate) fn command_hash(name: &[u8]) -> u64 {
    if name.len() < 8 {
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        for byte in name {
            hash = (hash ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3);
        }
        return hash;
    }
    let word = |start| {
        // SAFETY: Every caller-selected start leaves eight bytes in `name`.
        u64::from_le(unsafe { name.as_ptr().add(start).cast::<u64>().read_unaligned() })
    };
    let middle = (name.len() - 8) / 2;
    word(0)
        ^ word(middle).rotate_left(21)
        ^ word(name.len() - 8).rotate_left(42)
        // SAFETY: This branch requires `name.len() >= 8`, and 5/8 of a
        // positive length is strictly inside the slice.
        ^ u64::from(unsafe { *name.get_unchecked(name.len() * 5 / 8) }).rotate_left(7)
        ^ name.len() as u64
}

pub(crate) const fn command_slot_hash(mut hash: u64) -> u64 {
    hash ^= hash >> 32;
    hash = hash.wrapping_mul(0xd6e8_feb8_6659_fd93);
    hash ^ (hash >> 32)
}

pub(crate) const fn dispatch_offset(value: usize) -> u16 {
    assert!(value <= 65_535);
    value as u16
}
