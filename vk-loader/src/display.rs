//! Loader emulation for `VK_KHR_get_display_properties2` commands.

use crate::emulation::{emulate_result_array, optional_output_slice};
use crate::generated::EmulatedCommand;
use crate::instance::LoaderPhysicalDevice;

const STACK_PROPERTIES: usize = 8;

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceDisplayProperties2KHR(
    physical_device: vk::VkPhysicalDevice,
    count: *mut u32,
    output: *mut vk::VkDisplayProperties2KHR<'_>,
) -> vk::VkResult {
    if count.is_null() {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The Vulkan contract requires writable count storage.
    let count = unsafe { &mut *count };
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
        *count = 0;
        return vk::VkResult::SUCCESS;
    };
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceDisplayProperties2KHR);
    // SAFETY: A non-null Vulkan enumeration output points to `count` writable elements.
    let output = unsafe { optional_output_slice(output, *count) };
    unsafe {
        emulate_result_array::<_, _, STACK_PROPERTIES>(
            count,
            output,
            |count, temporary| command(device.native, count, temporary),
            |output, property| output.displayProperties = property,
        )
        .unwrap_or(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceDisplayPlaneProperties2KHR(
    physical_device: vk::VkPhysicalDevice,
    count: *mut u32,
    output: *mut vk::VkDisplayPlaneProperties2KHR<'_>,
) -> vk::VkResult {
    if count.is_null() {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The Vulkan contract requires writable count storage.
    let count = unsafe { &mut *count };
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
        *count = 0;
        return vk::VkResult::SUCCESS;
    };
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceDisplayPlaneProperties2KHR);
    // SAFETY: A non-null Vulkan enumeration output points to `count` writable elements.
    let output = unsafe { optional_output_slice(output, *count) };
    unsafe {
        emulate_result_array::<_, _, STACK_PROPERTIES>(
            count,
            output,
            |count, temporary| command(device.native, count, temporary),
            |output, property| output.displayPlaneProperties = property,
        )
        .unwrap_or(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetDisplayModeProperties2KHR(
    physical_device: vk::VkPhysicalDevice,
    display: vk::VkDisplayKHR,
    count: *mut u32,
    output: *mut vk::VkDisplayModeProperties2KHR<'_>,
) -> vk::VkResult {
    if count.is_null() {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    }
    // SAFETY: The Vulkan contract requires writable count storage.
    let count = unsafe { &mut *count };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if let Some(command) = device.icd().dispatch.vkGetDisplayModeProperties2KHR {
        return unsafe { command(device.native, display, count, output) };
    }
    let Some(command) = device.icd().dispatch.vkGetDisplayModePropertiesKHR else {
        *count = 0;
        return vk::VkResult::SUCCESS;
    };
    device.log_icd_emulation(EmulatedCommand::GetDisplayModeProperties2KHR);
    // SAFETY: A non-null Vulkan enumeration output points to `count` writable elements.
    let output = unsafe { optional_output_slice(output, *count) };
    unsafe {
        emulate_result_array::<_, _, STACK_PROPERTIES>(
            count,
            output,
            |count, temporary| command(device.native, display, count, temporary),
            |output, property| output.displayModeProperties = property,
        )
        .unwrap_or(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)
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
    // SAFETY: Both pointers were validated and remain live for this call.
    let info = unsafe { &*info };
    let output = unsafe { &mut *output };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if let Some(command) = device.icd().dispatch.vkGetDisplayPlaneCapabilities2KHR {
        return unsafe { command(device.native, info, output) };
    }
    let Some(command) = device.icd().dispatch.vkGetDisplayPlaneCapabilitiesKHR else {
        output.capabilities = vk::VkDisplayPlaneCapabilitiesKHR::DEFAULT;
        return vk::VkResult::SUCCESS;
    };
    device.log_icd_emulation(EmulatedCommand::GetDisplayPlaneCapabilities2KHR);
    unsafe {
        command(
            device.native,
            info.mode,
            info.planeIndex,
            &raw mut output.capabilities,
        )
    }
}
