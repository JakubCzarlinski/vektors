//! Vulkan core-promotion terminators requiring pre-promotion ICD emulation.

use alloc::vec::Vec;
use vk::{VkPhysicalDevice, VkStructureType};

use crate::emulation::{emulate_void_array, for_each_output_chain, optional_output_slice};
use crate::generated::{
    EmulatedCommand, PromotedDispatch, dispatch_promoted_external_buffer_properties,
    dispatch_promoted_external_fence_properties, dispatch_promoted_external_semaphore_properties,
    dispatch_promoted_features2, dispatch_promoted_format_properties2,
    dispatch_promoted_image_format_properties2, dispatch_promoted_memory_properties2,
    dispatch_promoted_properties2, dispatch_promoted_queue_family_properties2,
    dispatch_promoted_sparse_image_format_properties2,
};
use crate::instance::LoaderPhysicalDevice;

const STACK_PROPERTIES: usize = 8;

pub(crate) unsafe fn features2_impl(
    physical_device: VkPhysicalDevice,
    output: *mut vk::VkPhysicalDeviceFeatures2<'_>,
) {
    if output.is_null() {
        return;
    }
    let output = unsafe { &mut *output };
    // SAFETY: This is an ICD-boundary terminator handle.
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if let PromotedDispatch::Dispatched(()) = unsafe { dispatch_promoted_features2(device, output) }
    {
        return;
    }
    let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFeatures else {
        return;
    };
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceFeatures2);
    // Both core and KHR Features2 begin with sType, pNext, then features.
    // SAFETY: The entry-point contract makes the output writable.
    unsafe { command(device.native, &raw mut output.features) };
    // Upstream's only defined Features2 emulation node is multiview.
    unsafe {
        for_each_output_chain(output.pNext, |header| {
            if header.sType == VkStructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES {
                let multiview =
                    core::ptr::from_mut(header).cast::<vk::VkPhysicalDeviceMultiviewFeatures<'_>>();
                (*multiview).multiview = 0;
                (*multiview).multiviewGeometryShader = 0;
                (*multiview).multiviewTessellationShader = 0;
            }
        });
    }
}

pub(crate) unsafe fn properties2_impl(
    physical_device: VkPhysicalDevice,
    output: *mut vk::VkPhysicalDeviceProperties2<'_>,
) {
    if output.is_null() {
        return;
    }
    let output = unsafe { &mut *output };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if let PromotedDispatch::Dispatched(()) =
        unsafe { dispatch_promoted_properties2(device, output) }
    {
        return;
    }
    let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceProperties else {
        return;
    };
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceProperties2);
    unsafe { command(device.native, &raw mut output.properties) };
    unsafe {
        for_each_output_chain(output.pNext, |header| {
            if header.sType == VkStructureType::PHYSICAL_DEVICE_ID_PROPERTIES {
                let ids =
                    core::ptr::from_mut(header).cast::<vk::VkPhysicalDeviceIDProperties<'_>>();
                (*ids).deviceUUID = [0; vk::VK_UUID_SIZE as usize];
                (*ids).driverUUID = [0; vk::VK_UUID_SIZE as usize];
                (*ids).deviceLUIDValid = 0;
            }
        });
    }
}

pub(crate) unsafe fn format_properties2_impl(
    physical_device: VkPhysicalDevice,
    format: vk::VkFormat,
    output: *mut vk::VkFormatProperties2<'_>,
) {
    if output.is_null() {
        return;
    }
    let output = unsafe { &mut *output };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if let PromotedDispatch::Dispatched(()) =
        unsafe { dispatch_promoted_format_properties2(device, format, output) }
    {
        return;
    }
    let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFormatProperties else {
        return;
    };
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceFormatProperties2);
    unsafe { command(device.native, format, &raw mut output.formatProperties) };
}

pub(crate) unsafe fn memory_properties2_impl(
    physical_device: VkPhysicalDevice,
    output: *mut vk::VkPhysicalDeviceMemoryProperties2<'_>,
) {
    if output.is_null() {
        return;
    }
    let output = unsafe { &mut *output };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if let PromotedDispatch::Dispatched(()) =
        unsafe { dispatch_promoted_memory_properties2(device, output) }
    {
        return;
    }
    let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceMemoryProperties else {
        return;
    };
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceMemoryProperties2);
    unsafe { command(device.native, &raw mut output.memoryProperties) };
}

pub(crate) unsafe fn image_format_properties2_impl(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceImageFormatInfo2<'_>,
    output: *mut vk::VkImageFormatProperties2<'_>,
) -> vk::VkResult {
    if input.is_null() || output.is_null() {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let input = unsafe { &*input };
    let output = unsafe { &mut *output };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if let PromotedDispatch::Dispatched(result) =
        unsafe { dispatch_promoted_image_format_properties2(device, input, output) }
    {
        return result;
    }
    let Some(command) = device
        .icd()
        .dispatch
        .vkGetPhysicalDeviceImageFormatProperties
    else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceImageFormatProperties2);
    if !input.pNext.is_null() || !output.pNext.is_null() {
        return vk::VkResult::ERROR_FORMAT_NOT_SUPPORTED;
    }
    unsafe {
        command(
            device.native,
            input.format,
            input.type_,
            input.tiling,
            input.usage,
            input.flags,
            &raw mut output.imageFormatProperties,
        )
    }
}

pub(crate) unsafe fn external_buffer_properties_impl(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceExternalBufferInfo<'_>,
    output: *mut vk::VkExternalBufferProperties<'_>,
) {
    if input.is_null() || output.is_null() {
        return;
    }
    let input = unsafe { &*input };
    let output = unsafe { &mut *output };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if let PromotedDispatch::Dispatched(()) =
        unsafe { dispatch_promoted_external_buffer_properties(device, input, output) }
    {
        return;
    }
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceExternalBufferProperties);
    output.externalMemoryProperties = vk::VkExternalMemoryProperties::DEFAULT;
}

pub(crate) unsafe fn external_semaphore_properties_impl(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceExternalSemaphoreInfo<'_>,
    output: *mut vk::VkExternalSemaphoreProperties<'_>,
) {
    if input.is_null() || output.is_null() {
        return;
    }
    let input = unsafe { &*input };
    let output = unsafe { &mut *output };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if let PromotedDispatch::Dispatched(()) =
        unsafe { dispatch_promoted_external_semaphore_properties(device, input, output) }
    {
        return;
    }
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceExternalSemaphoreProperties);
    output.exportFromImportedHandleTypes = vk::VkExternalSemaphoreHandleTypeFlagBits::EMPTY;
    output.compatibleHandleTypes = vk::VkExternalSemaphoreHandleTypeFlagBits::EMPTY;
    output.externalSemaphoreFeatures = vk::VkExternalSemaphoreFeatureFlagBits::EMPTY;
}

pub(crate) unsafe fn external_fence_properties_impl(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceExternalFenceInfo<'_>,
    output: *mut vk::VkExternalFenceProperties<'_>,
) {
    if input.is_null() || output.is_null() {
        return;
    }
    let input = unsafe { &*input };
    let output = unsafe { &mut *output };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if let PromotedDispatch::Dispatched(()) =
        unsafe { dispatch_promoted_external_fence_properties(device, input, output) }
    {
        return;
    }
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceExternalFenceProperties);
    output.exportFromImportedHandleTypes = vk::VkExternalFenceHandleTypeFlagBits::EMPTY;
    output.compatibleHandleTypes = vk::VkExternalFenceHandleTypeFlagBits::EMPTY;
    output.externalFenceFeatures = vk::VkExternalFenceFeatureFlagBits::EMPTY;
}

pub(crate) unsafe fn queue_family_properties2_impl(
    physical_device: VkPhysicalDevice,
    count: *mut u32,
    output: *mut vk::VkQueueFamilyProperties2<'_>,
) {
    if count.is_null() {
        return;
    }
    let count = unsafe { &mut *count };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if let PromotedDispatch::Dispatched(()) =
        unsafe { dispatch_promoted_queue_family_properties2(device, count, output) }
    {
        return;
    }
    let Some(command) = device
        .icd()
        .dispatch
        .vkGetPhysicalDeviceQueueFamilyProperties
    else {
        *count = 0;
        return;
    };
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceQueueFamilyProperties2);
    let capacity = *count as usize;
    if output.is_null() || capacity == 0 {
        unsafe { command(device.native, count, core::ptr::null_mut()) };
        return;
    }
    // SAFETY: A non-null Vulkan enumeration output points to `count` writable elements.
    let output = unsafe { optional_output_slice(output, *count) };
    let result = unsafe {
        emulate_void_array::<_, _, STACK_PROPERTIES>(
            count,
            output,
            |count, temporary| command(device.native, count, temporary),
            |slot, property| slot.queueFamilyProperties = property,
        )
    };
    if result.is_err() {
        *count = 0;
    }
}

pub(crate) unsafe fn sparse_image_format_properties2_impl(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceSparseImageFormatInfo2<'_>,
    count: *mut u32,
    output: *mut vk::VkSparseImageFormatProperties2<'_>,
) {
    if input.is_null() || count.is_null() {
        return;
    }
    let input = unsafe { &*input };
    let count = unsafe { &mut *count };
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if let PromotedDispatch::Dispatched(()) =
        unsafe { dispatch_promoted_sparse_image_format_properties2(device, input, count, output) }
    {
        return;
    }
    let Some(command) = device
        .icd()
        .dispatch
        .vkGetPhysicalDeviceSparseImageFormatProperties
    else {
        *count = 0;
        return;
    };
    device.log_icd_emulation(EmulatedCommand::GetPhysicalDeviceSparseImageFormatProperties2);
    let capacity = *count as usize;
    if output.is_null() || capacity == 0 {
        unsafe {
            command(
                device.native,
                input.format,
                input.type_,
                input.samples,
                input.usage,
                input.tiling,
                count,
                core::ptr::null_mut(),
            );
        };
        return;
    }
    // SAFETY: A non-null Vulkan enumeration output points to `count` writable elements.
    let output = unsafe { optional_output_slice(output, *count) };
    let result = unsafe {
        emulate_void_array::<_, _, STACK_PROPERTIES>(
            count,
            output,
            |count, temporary| {
                command(
                    device.native,
                    input.format,
                    input.type_,
                    input.samples,
                    input.usage,
                    input.tiling,
                    count,
                    temporary,
                );
            },
            |slot, property| slot.properties = property,
        )
    };
    if result.is_err() {
        *count = 0;
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceToolProperties(
    physical_device: VkPhysicalDevice,
    count: *mut u32,
    properties: *mut vk::VkPhysicalDeviceToolProperties<'_>,
) -> vk::VkResult {
    if count.is_null() {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let mut physical_properties = vk::VkPhysicalDeviceProperties::DEFAULT;
    if let Some(get_properties) = device.icd().dispatch.vkGetPhysicalDeviceProperties {
        unsafe { get_properties(device.native, &raw mut physical_properties) };
        if physical_properties.apiVersion >= vk::VK_API_VERSION_1_3
            && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceToolProperties
        {
            return unsafe { command(device.native, count, properties) };
        }
    }
    unsafe { count.write(0) };
    vk::VkResult::SUCCESS
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceToolPropertiesEXT(
    physical_device: VkPhysicalDevice,
    count: *mut u32,
    properties: *mut vk::VkPhysicalDeviceToolPropertiesEXT<'_>,
) -> vk::VkResult {
    if count.is_null() {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    let Some(enumerate) = device.icd().dispatch.vkEnumerateDeviceExtensionProperties else {
        unsafe { count.write(0) };
        return vk::VkResult::SUCCESS;
    };
    let mut extension_count = 0;
    let result = unsafe {
        enumerate(
            device.native,
            core::ptr::null(),
            &raw mut extension_count,
            core::ptr::null_mut(),
        )
    };
    if result != vk::VkResult::SUCCESS {
        unsafe { count.write(0) };
        return result;
    }
    let capacity = extension_count as usize;
    let mut extensions = Vec::new();
    if extensions.try_reserve_exact(capacity).is_err() {
        unsafe { count.write(0) };
        return vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
    }
    extensions.resize(capacity, vk::VkExtensionProperties::DEFAULT);
    let result = unsafe {
        enumerate(
            device.native,
            core::ptr::null(),
            &raw mut extension_count,
            extensions.as_mut_ptr(),
        )
    };
    if result != vk::VkResult::SUCCESS {
        unsafe { count.write(0) };
        return result;
    }
    let supported = extensions
        .iter()
        .take((extension_count as usize).min(capacity))
        .any(|extension| unsafe {
            core::ffi::CStr::from_ptr(extension.extensionName.as_ptr())
                == vk::VK_EXT_TOOLING_INFO_EXTENSION_NAME
        });
    if supported && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceToolPropertiesEXT {
        return unsafe { command(device.native, count, properties) };
    }
    unsafe { count.write(0) };
    vk::VkResult::SUCCESS
}
