//! Vulkan core-promotion terminators requiring pre-promotion ICD emulation.

use vk::{VkBaseOutStructure, VkPhysicalDevice, VkStructureType};

use crate::collections::ScratchArray;
use crate::instance::LoaderPhysicalDevice;

const STACK_PROPERTIES: usize = 8;

#[inline]
fn properties2_extension_enabled(device: &LoaderPhysicalDevice) -> bool {
    device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
}

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
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFeatures2
    {
        // SAFETY: Core and erased output types match this entry point.
        unsafe { command(device.native, output) };
        return;
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFeatures2KHR
    {
        // SAFETY: The KHR and core structures have the same Vulkan ABI.
        unsafe { command(device.native, core::ptr::from_mut(output).cast()) };
        return;
    }
    let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFeatures else {
        return;
    };
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceFeatures2: Emulating call in ICD using vkGetPhysicalDeviceFeatures",
    );
    // Both core and KHR Features2 begin with sType, pNext, then features.
    // SAFETY: The entry-point contract makes the output writable.
    unsafe { command(device.native, &raw mut output.features) };
    // Upstream's only defined Features2 emulation node is multiview.
    let mut next = output.pNext.cast::<VkBaseOutStructure<'_>>();
    while !next.is_null() {
        let header = unsafe { next.read() };
        if header.sType == VkStructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES {
            let multiview = next.cast::<vk::VkPhysicalDeviceMultiviewFeatures<'_>>();
            unsafe {
                (*multiview).multiview = 0;
                (*multiview).multiviewGeometryShader = 0;
                (*multiview).multiviewTessellationShader = 0;
            }
        }
        next = header.pNext;
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
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceProperties2
    {
        unsafe { command(device.native, output) };
        return;
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceProperties2KHR
    {
        unsafe { command(device.native, core::ptr::from_mut(output).cast()) };
        return;
    }
    let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceProperties else {
        return;
    };
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceProperties2: Emulating call in ICD using vkGetPhysicalDeviceProperties",
    );
    unsafe { command(device.native, &raw mut output.properties) };
    let mut next = output.pNext.cast::<VkBaseOutStructure<'_>>();
    while !next.is_null() {
        let header = unsafe { next.read() };
        if header.sType == VkStructureType::PHYSICAL_DEVICE_ID_PROPERTIES {
            let ids = next.cast::<vk::VkPhysicalDeviceIDProperties<'_>>();
            unsafe {
                (*ids).deviceUUID = [0; vk::VK_UUID_SIZE as usize];
                (*ids).driverUUID = [0; vk::VK_UUID_SIZE as usize];
                (*ids).deviceLUIDValid = 0;
            }
        }
        next = header.pNext;
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
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFormatProperties2
    {
        unsafe { command(device.native, format, output) };
        return;
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceFormatProperties2KHR
    {
        unsafe { command(device.native, format, core::ptr::from_mut(output).cast()) };
        return;
    }
    let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFormatProperties else {
        return;
    };
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceFormatProperties2: Emulating call in ICD using vkGetPhysicalDeviceFormatProperties",
    );
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
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceMemoryProperties2
    {
        unsafe { command(device.native, output) };
        return;
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceMemoryProperties2KHR
    {
        unsafe { command(device.native, core::ptr::from_mut(output).cast()) };
        return;
    }
    let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceMemoryProperties else {
        return;
    };
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceMemoryProperties2: Emulating call in ICD using vkGetPhysicalDeviceMemoryProperties",
    );
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
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceImageFormatProperties2
    {
        return unsafe { command(device.native, input, output) };
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceImageFormatProperties2KHR
    {
        return unsafe {
            command(
                device.native,
                core::ptr::from_ref(input).cast(),
                core::ptr::from_mut(output).cast(),
            )
        };
    }
    let Some(command) = device
        .icd()
        .dispatch
        .vkGetPhysicalDeviceImageFormatProperties
    else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceImageFormatProperties2: Emulating call in ICD using vkGetPhysicalDeviceImageFormatProperties",
    );
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
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalBufferProperties
    {
        unsafe { command(device.native, input, output) };
        return;
    }
    if device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalBufferPropertiesKHR
    {
        unsafe {
            command(
                device.native,
                core::ptr::from_ref(input).cast(),
                core::ptr::from_mut(output).cast(),
            );
        };
        return;
    }
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceExternalBufferProperties: Emulating call in ICD",
    );
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
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalSemaphoreProperties
    {
        unsafe { command(device.native, input, output) };
        return;
    }
    if device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalSemaphorePropertiesKHR
    {
        unsafe {
            command(
                device.native,
                core::ptr::from_ref(input).cast(),
                core::ptr::from_mut(output).cast(),
            );
        };
        return;
    }
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceExternalSemaphoreProperties: Emulating call in ICD",
    );
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
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalFenceProperties
    {
        unsafe { command(device.native, input, output) };
        return;
    }
    if device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalFencePropertiesKHR
    {
        unsafe {
            command(
                device.native,
                core::ptr::from_ref(input).cast(),
                core::ptr::from_mut(output).cast(),
            );
        };
        return;
    }
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceExternalFenceProperties: Emulating call in ICD",
    );
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
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceQueueFamilyProperties2
    {
        unsafe { command(device.native, count, output) };
        return;
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceQueueFamilyProperties2KHR
    {
        unsafe { command(device.native, count, output.cast()) };
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
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceQueueFamilyProperties2: Emulating call in ICD using vkGetPhysicalDeviceQueueFamilyProperties",
    );
    let capacity = *count as usize;
    if output.is_null() || capacity == 0 {
        unsafe { command(device.native, count, core::ptr::null_mut()) };
        return;
    }
    let Ok(mut temporary) =
        ScratchArray::<vk::VkQueueFamilyProperties, STACK_PROPERTIES>::try_new(capacity)
    else {
        *count = 0;
        return;
    };
    unsafe { command(device.native, count, temporary.as_mut_ptr()) };
    let written = (*count as usize).min(capacity);
    let output = unsafe { core::slice::from_raw_parts_mut(output, capacity) };
    // SAFETY: The ICD initialized the prefix reported through `count`.
    for (slot, &property) in output
        .iter_mut()
        .zip(unsafe { temporary.initialized(written) })
    {
        slot.queueFamilyProperties = property;
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
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceSparseImageFormatProperties2
    {
        unsafe { command(device.native, input, count, output) };
        return;
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceSparseImageFormatProperties2KHR
    {
        unsafe {
            command(
                device.native,
                core::ptr::from_ref(input).cast(),
                count,
                output.cast(),
            );
        };
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
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceSparseImageFormatProperties2: Emulating call in ICD using vkGetPhysicalDeviceSparseImageFormatProperties",
    );
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
    let Ok(mut temporary) =
        ScratchArray::<vk::VkSparseImageFormatProperties, STACK_PROPERTIES>::try_new(capacity)
    else {
        *count = 0;
        return;
    };
    unsafe {
        command(
            device.native,
            input.format,
            input.type_,
            input.samples,
            input.usage,
            input.tiling,
            count,
            temporary.as_mut_ptr(),
        );
    };
    let written = (*count as usize).min(capacity);
    let output = unsafe { core::slice::from_raw_parts_mut(output, capacity) };
    // SAFETY: The ICD initialized the prefix reported through `count`.
    for (slot, &property) in output
        .iter_mut()
        .zip(unsafe { temporary.initialized(written) })
    {
        slot.properties = property;
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
