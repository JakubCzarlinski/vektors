//! Vulkan core-promotion terminators requiring pre-promotion ICD emulation.

use alloc::vec::Vec;
use core::ffi::c_void;

use vk::{VkBaseOutStructure, VkPhysicalDevice, VkStructureType};

use crate::instance::LoaderPhysicalDevice;

#[inline]
fn properties2_extension_enabled(device: &LoaderPhysicalDevice) -> bool {
    device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
}

unsafe fn features2_impl(physical_device: VkPhysicalDevice, output: *mut c_void) {
    if output.is_null() {
        return;
    }
    // SAFETY: This is an ICD-boundary terminator handle.
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFeatures2
    {
        // SAFETY: Core and erased output types match this entry point.
        unsafe { command(device.native, output.cast()) };
        return;
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFeatures2KHR
    {
        // SAFETY: The KHR and core structures have the same Vulkan ABI.
        unsafe { command(device.native, output.cast()) };
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
    let output = output.cast::<vk::VkPhysicalDeviceFeatures2<'_>>();
    // SAFETY: The entry-point contract makes the output writable.
    unsafe { command(device.native, &raw mut (*output).features) };
    // Upstream's only defined Features2 emulation node is multiview.
    let mut next = unsafe { (*output).pNext.cast::<VkBaseOutStructure<'_>>() };
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

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceFeatures2(
    physical_device: VkPhysicalDevice,
    output: *mut vk::VkPhysicalDeviceFeatures2<'_>,
) {
    unsafe { features2_impl(physical_device, output.cast()) };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceFeatures2KHR(
    physical_device: VkPhysicalDevice,
    output: *mut vk::VkPhysicalDeviceFeatures2KHR<'_>,
) {
    unsafe { features2_impl(physical_device, output.cast()) };
}

unsafe fn properties2_impl(physical_device: VkPhysicalDevice, output: *mut c_void) {
    if output.is_null() {
        return;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceProperties2
    {
        unsafe { command(device.native, output.cast()) };
        return;
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceProperties2KHR
    {
        unsafe { command(device.native, output.cast()) };
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
    let output = output.cast::<vk::VkPhysicalDeviceProperties2<'_>>();
    unsafe { command(device.native, &raw mut (*output).properties) };
    let mut next = unsafe { (*output).pNext.cast::<VkBaseOutStructure<'_>>() };
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

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceProperties2(
    physical_device: VkPhysicalDevice,
    output: *mut vk::VkPhysicalDeviceProperties2<'_>,
) {
    unsafe { properties2_impl(physical_device, output.cast()) };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceProperties2KHR(
    physical_device: VkPhysicalDevice,
    output: *mut vk::VkPhysicalDeviceProperties2KHR<'_>,
) {
    unsafe { properties2_impl(physical_device, output.cast()) };
}

unsafe fn format_properties2_impl(
    physical_device: VkPhysicalDevice,
    format: vk::VkFormat,
    output: *mut c_void,
) {
    if output.is_null() {
        return;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFormatProperties2
    {
        unsafe { command(device.native, format, output.cast()) };
        return;
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceFormatProperties2KHR
    {
        unsafe { command(device.native, format, output.cast()) };
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
    let output = output.cast::<vk::VkFormatProperties2<'_>>();
    unsafe { command(device.native, format, &raw mut (*output).formatProperties) };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceFormatProperties2(
    physical_device: VkPhysicalDevice,
    format: vk::VkFormat,
    output: *mut vk::VkFormatProperties2<'_>,
) {
    unsafe { format_properties2_impl(physical_device, format, output.cast()) };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceFormatProperties2KHR(
    physical_device: VkPhysicalDevice,
    format: vk::VkFormat,
    output: *mut vk::VkFormatProperties2KHR<'_>,
) {
    unsafe { format_properties2_impl(physical_device, format, output.cast()) };
}

unsafe fn memory_properties2_impl(physical_device: VkPhysicalDevice, output: *mut c_void) {
    if output.is_null() {
        return;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceMemoryProperties2
    {
        unsafe { command(device.native, output.cast()) };
        return;
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceMemoryProperties2KHR
    {
        unsafe { command(device.native, output.cast()) };
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
    let output = output.cast::<vk::VkPhysicalDeviceMemoryProperties2<'_>>();
    unsafe { command(device.native, &raw mut (*output).memoryProperties) };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceMemoryProperties2(
    physical_device: VkPhysicalDevice,
    output: *mut vk::VkPhysicalDeviceMemoryProperties2<'_>,
) {
    unsafe { memory_properties2_impl(physical_device, output.cast()) };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceMemoryProperties2KHR(
    physical_device: VkPhysicalDevice,
    output: *mut vk::VkPhysicalDeviceMemoryProperties2KHR<'_>,
) {
    unsafe { memory_properties2_impl(physical_device, output.cast()) };
}

unsafe fn image_format_properties2_impl(
    physical_device: VkPhysicalDevice,
    input: *const c_void,
    output: *mut c_void,
) -> vk::VkResult {
    if input.is_null() || output.is_null() {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return vk::VkResult::ERROR_INITIALIZATION_FAILED;
    };
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceImageFormatProperties2
    {
        return unsafe { command(device.native, input.cast(), output.cast()) };
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceImageFormatProperties2KHR
    {
        return unsafe { command(device.native, input.cast(), output.cast()) };
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
    let input = input.cast::<vk::VkPhysicalDeviceImageFormatInfo2<'_>>();
    let output = output.cast::<vk::VkImageFormatProperties2<'_>>();
    if unsafe { !(*input).pNext.is_null() || !(*output).pNext.is_null() } {
        return vk::VkResult::ERROR_FORMAT_NOT_SUPPORTED;
    }
    unsafe {
        command(
            device.native,
            (*input).format,
            (*input).type_,
            (*input).tiling,
            (*input).usage,
            (*input).flags,
            &raw mut (*output).imageFormatProperties,
        )
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceImageFormatProperties2(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceImageFormatInfo2<'_>,
    output: *mut vk::VkImageFormatProperties2<'_>,
) -> vk::VkResult {
    unsafe { image_format_properties2_impl(physical_device, input.cast(), output.cast()) }
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceImageFormatProperties2KHR(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceImageFormatInfo2KHR<'_>,
    output: *mut vk::VkImageFormatProperties2KHR<'_>,
) -> vk::VkResult {
    unsafe { image_format_properties2_impl(physical_device, input.cast(), output.cast()) }
}

unsafe fn external_buffer_properties_impl(
    physical_device: VkPhysicalDevice,
    input: *const c_void,
    output: *mut c_void,
) {
    if input.is_null() || output.is_null() {
        return;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalBufferProperties
    {
        unsafe { command(device.native, input.cast(), output.cast()) };
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
        unsafe { command(device.native, input.cast(), output.cast()) };
        return;
    }
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceExternalBufferProperties: Emulating call in ICD",
    );
    let output = output.cast::<vk::VkExternalBufferProperties<'_>>();
    unsafe { (*output).externalMemoryProperties = vk::VkExternalMemoryProperties::DEFAULT };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalBufferProperties(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceExternalBufferInfo<'_>,
    output: *mut vk::VkExternalBufferProperties<'_>,
) {
    unsafe { external_buffer_properties_impl(physical_device, input.cast(), output.cast()) };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalBufferPropertiesKHR(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceExternalBufferInfoKHR<'_>,
    output: *mut vk::VkExternalBufferPropertiesKHR<'_>,
) {
    unsafe { external_buffer_properties_impl(physical_device, input.cast(), output.cast()) };
}

unsafe fn external_semaphore_properties_impl(
    physical_device: VkPhysicalDevice,
    input: *const c_void,
    output: *mut c_void,
) {
    if input.is_null() || output.is_null() {
        return;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalSemaphoreProperties
    {
        unsafe { command(device.native, input.cast(), output.cast()) };
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
        unsafe { command(device.native, input.cast(), output.cast()) };
        return;
    }
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceExternalSemaphoreProperties: Emulating call in ICD",
    );
    let output = output.cast::<vk::VkExternalSemaphoreProperties<'_>>();
    unsafe {
        (*output).exportFromImportedHandleTypes = Default::default();
        (*output).compatibleHandleTypes = Default::default();
        (*output).externalSemaphoreFeatures = Default::default();
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalSemaphoreProperties(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceExternalSemaphoreInfo<'_>,
    output: *mut vk::VkExternalSemaphoreProperties<'_>,
) {
    unsafe { external_semaphore_properties_impl(physical_device, input.cast(), output.cast()) };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceExternalSemaphoreInfoKHR<'_>,
    output: *mut vk::VkExternalSemaphorePropertiesKHR<'_>,
) {
    unsafe { external_semaphore_properties_impl(physical_device, input.cast(), output.cast()) };
}

unsafe fn external_fence_properties_impl(
    physical_device: VkPhysicalDevice,
    input: *const c_void,
    output: *mut c_void,
) {
    if input.is_null() || output.is_null() {
        return;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalFenceProperties
    {
        unsafe { command(device.native, input.cast(), output.cast()) };
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
        unsafe { command(device.native, input.cast(), output.cast()) };
        return;
    }
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceExternalFenceProperties: Emulating call in ICD",
    );
    let output = output.cast::<vk::VkExternalFenceProperties<'_>>();
    unsafe {
        (*output).exportFromImportedHandleTypes = Default::default();
        (*output).compatibleHandleTypes = Default::default();
        (*output).externalFenceFeatures = Default::default();
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalFenceProperties(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceExternalFenceInfo<'_>,
    output: *mut vk::VkExternalFenceProperties<'_>,
) {
    unsafe { external_fence_properties_impl(physical_device, input.cast(), output.cast()) };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalFencePropertiesKHR(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceExternalFenceInfoKHR<'_>,
    output: *mut vk::VkExternalFencePropertiesKHR<'_>,
) {
    unsafe { external_fence_properties_impl(physical_device, input.cast(), output.cast()) };
}

unsafe fn queue_family_properties2_impl(
    physical_device: VkPhysicalDevice,
    count: *mut u32,
    output: *mut c_void,
) {
    if count.is_null() {
        return;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceQueueFamilyProperties2
    {
        unsafe { command(device.native, count, output.cast()) };
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
        unsafe { count.write(0) };
        return;
    };
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceQueueFamilyProperties2: Emulating call in ICD using vkGetPhysicalDeviceQueueFamilyProperties",
    );
    let capacity = unsafe { count.read() } as usize;
    if output.is_null() || capacity == 0 {
        unsafe { command(device.native, count, core::ptr::null_mut()) };
        return;
    }
    let mut temporary = Vec::new();
    if temporary.try_reserve_exact(capacity).is_err() {
        unsafe { count.write(0) };
        return;
    }
    temporary.resize(capacity, vk::VkQueueFamilyProperties::DEFAULT);
    unsafe { command(device.native, count, temporary.as_mut_ptr()) };
    let written = (unsafe { count.read() } as usize).min(capacity);
    let output = output.cast::<vk::VkQueueFamilyProperties2<'_>>();
    for (index, property) in temporary.into_iter().take(written).enumerate() {
        unsafe { (*output.add(index)).queueFamilyProperties = property };
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceQueueFamilyProperties2(
    physical_device: VkPhysicalDevice,
    count: *mut u32,
    output: *mut vk::VkQueueFamilyProperties2<'_>,
) {
    unsafe { queue_family_properties2_impl(physical_device, count, output.cast()) };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceQueueFamilyProperties2KHR(
    physical_device: VkPhysicalDevice,
    count: *mut u32,
    output: *mut vk::VkQueueFamilyProperties2KHR<'_>,
) {
    unsafe { queue_family_properties2_impl(physical_device, count, output.cast()) };
}

unsafe fn sparse_image_format_properties2_impl(
    physical_device: VkPhysicalDevice,
    input: *const c_void,
    count: *mut u32,
    output: *mut c_void,
) {
    if input.is_null() || count.is_null() {
        return;
    }
    let Some(device) = (unsafe { LoaderPhysicalDevice::from_handle(physical_device) }) else {
        return;
    };
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceSparseImageFormatProperties2
    {
        unsafe { command(device.native, input.cast(), count, output.cast()) };
        return;
    }
    if properties2_extension_enabled(device)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceSparseImageFormatProperties2KHR
    {
        unsafe { command(device.native, input.cast(), count, output.cast()) };
        return;
    }
    let Some(command) = device
        .icd()
        .dispatch
        .vkGetPhysicalDeviceSparseImageFormatProperties
    else {
        unsafe { count.write(0) };
        return;
    };
    device.instance().submit_loader_message(
        vk::VkDebugUtilsMessageSeverityFlagBitsEXT::INFO,
        vk::VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL,
        c"vkGetPhysicalDeviceSparseImageFormatProperties2: Emulating call in ICD using vkGetPhysicalDeviceSparseImageFormatProperties",
    );
    let input = input.cast::<vk::VkPhysicalDeviceSparseImageFormatInfo2<'_>>();
    let capacity = unsafe { count.read() } as usize;
    if output.is_null() || capacity == 0 {
        unsafe {
            command(
                device.native,
                (*input).format,
                (*input).type_,
                (*input).samples,
                (*input).usage,
                (*input).tiling,
                count,
                core::ptr::null_mut(),
            )
        };
        return;
    }
    let mut temporary = Vec::new();
    if temporary.try_reserve_exact(capacity).is_err() {
        unsafe { count.write(0) };
        return;
    }
    temporary.resize(capacity, vk::VkSparseImageFormatProperties::DEFAULT);
    unsafe {
        command(
            device.native,
            (*input).format,
            (*input).type_,
            (*input).samples,
            (*input).usage,
            (*input).tiling,
            count,
            temporary.as_mut_ptr(),
        )
    };
    let written = (unsafe { count.read() } as usize).min(capacity);
    let output = output.cast::<vk::VkSparseImageFormatProperties2<'_>>();
    for (index, property) in temporary.into_iter().take(written).enumerate() {
        unsafe { (*output.add(index)).properties = property };
    }
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSparseImageFormatProperties2(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceSparseImageFormatInfo2<'_>,
    count: *mut u32,
    output: *mut vk::VkSparseImageFormatProperties2<'_>,
) {
    unsafe {
        sparse_image_format_properties2_impl(physical_device, input.cast(), count, output.cast())
    };
}

pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
    physical_device: VkPhysicalDevice,
    input: *const vk::VkPhysicalDeviceSparseImageFormatInfo2KHR<'_>,
    count: *mut u32,
    output: *mut vk::VkSparseImageFormatProperties2KHR<'_>,
) {
    unsafe {
        sparse_image_format_properties2_impl(physical_device, input.cast(), count, output.cast())
    };
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
