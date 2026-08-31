//! Loader emulation for `VK_KHR_get_display_properties2` commands.

use alloc::vec::Vec;
use core::{ffi::c_void, mem::MaybeUninit};

use crate::instance::LoaderPhysicalDevice;

unsafe fn emulate_array<T: Copy>(
    count: *mut u32,
    output: *mut c_void,
    call: impl FnOnce(*mut u32, *mut T) -> vk::VkResult,
    mut write: impl FnMut(usize, T),
) -> vk::VkResult {
    if count.is_null() {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let capacity = unsafe { count.read() } as usize;
    if output.is_null() || capacity == 0 {
        return call(count, core::ptr::null_mut());
    }
    let mut temporary = Vec::<MaybeUninit<T>>::new();
    if temporary.try_reserve_exact(capacity).is_err() {
        return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    // SAFETY: The ICD receives all elements as writable output storage, and we
    // only read the number it reports as written after a non-error result.
    unsafe { temporary.set_len(capacity) };
    let result = call(count, temporary.as_mut_ptr().cast());
    if result.0 < 0 {
        return result;
    }
    let written = (unsafe { count.read() } as usize).min(capacity);
    for (index, property) in temporary.into_iter().take(written).enumerate() {
        write(index, unsafe { property.assume_init() });
    }
    result
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceDisplayProperties2KHR(
    physical_device: vk::VkPhysicalDevice,
    count: *mut u32,
    output: *mut vk::VkDisplayProperties2KHR<'_>,
) -> vk::VkResult {
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if let Some(command) = device
        .icd()
        .dispatch
        .vkGetPhysicalDeviceDisplayProperties2KHR
    {
        return unsafe { command(device.native, count, output) };
    }
    let Some(command) = device
        .icd()
        .dispatch
        .vkGetPhysicalDeviceDisplayPropertiesKHR
    else {
        if !count.is_null() {
            unsafe { count.write(0) };
        }
        return vk::VkResult::SUCCESS;
    };
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceDisplayProperties2KHR: Emulating call in ICD",
    );
    unsafe {
        emulate_array(
            count,
            output.cast(),
            |count, temporary| command(device.native, count, temporary),
            |index, property| (*output.add(index)).displayProperties = property,
        )
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceDisplayPlaneProperties2KHR(
    physical_device: vk::VkPhysicalDevice,
    count: *mut u32,
    output: *mut vk::VkDisplayPlaneProperties2KHR<'_>,
) -> vk::VkResult {
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if let Some(command) = device
        .icd()
        .dispatch
        .vkGetPhysicalDeviceDisplayPlaneProperties2KHR
    {
        return unsafe { command(device.native, count, output) };
    }
    let Some(command) = device
        .icd()
        .dispatch
        .vkGetPhysicalDeviceDisplayPlanePropertiesKHR
    else {
        if !count.is_null() {
            unsafe { count.write(0) };
        }
        return vk::VkResult::SUCCESS;
    };
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR: Emulating call in ICD",
    );
    unsafe {
        emulate_array(
            count,
            output.cast(),
            |count, temporary| command(device.native, count, temporary),
            |index, property| (*output.add(index)).displayPlaneProperties = property,
        )
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetDisplayModeProperties2KHR(
    physical_device: vk::VkPhysicalDevice,
    display: vk::VkDisplayKHR,
    count: *mut u32,
    output: *mut vk::VkDisplayModeProperties2KHR<'_>,
) -> vk::VkResult {
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if let Some(command) = device.icd().dispatch.vkGetDisplayModeProperties2KHR {
        return unsafe { command(device.native, display, count, output) };
    }
    let Some(command) = device.icd().dispatch.vkGetDisplayModePropertiesKHR else {
        if !count.is_null() {
            unsafe { count.write(0) };
        }
        return vk::VkResult::SUCCESS;
    };
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetDisplayModeProperties2KHR: Emulating call in ICD",
    );
    unsafe {
        emulate_array(
            count,
            output.cast(),
            |count, temporary| command(device.native, display, count, temporary),
            |index, property| (*output.add(index)).displayModeProperties = property,
        )
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetDisplayPlaneCapabilities2KHR(
    physical_device: vk::VkPhysicalDevice,
    info: *const vk::VkDisplayPlaneInfo2KHR<'_>,
    output: *mut vk::VkDisplayPlaneCapabilities2KHR<'_>,
) -> vk::VkResult {
    if info.is_null() || output.is_null() {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if let Some(command) = device.icd().dispatch.vkGetDisplayPlaneCapabilities2KHR {
        return unsafe { command(device.native, info, output) };
    }
    let Some(command) = device.icd().dispatch.vkGetDisplayPlaneCapabilitiesKHR else {
        unsafe { (*output).capabilities = vk::VkDisplayPlaneCapabilitiesKHR::DEFAULT };
        return vk::VkResult::SUCCESS;
    };
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetDisplayPlaneCapabilities2KHR: Emulating call in ICD",
    );
    unsafe {
        command(
            device.native,
            (*info).mode,
            (*info).planeIndex,
            &raw mut (*output).capabilities,
        )
    }
}
