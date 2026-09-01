// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.

use crate::LoaderPhysicalDevice;
#[derive(Clone, Copy)]
pub(crate) enum EmulatedCommand {
    GetPhysicalDeviceFeatures2,
    GetPhysicalDeviceProperties2,
    GetPhysicalDeviceFormatProperties2,
    GetPhysicalDeviceMemoryProperties2,
    GetPhysicalDeviceImageFormatProperties2,
    GetPhysicalDeviceExternalBufferProperties,
    GetPhysicalDeviceExternalSemaphoreProperties,
    GetPhysicalDeviceExternalFenceProperties,
    GetPhysicalDeviceQueueFamilyProperties2,
    GetPhysicalDeviceSparseImageFormatProperties2,
    GetPhysicalDeviceDisplayProperties2KHR,
    GetPhysicalDeviceDisplayPlaneProperties2KHR,
    GetDisplayModeProperties2KHR,
    GetDisplayPlaneCapabilities2KHR,
    GetPhysicalDeviceSurfaceCapabilities2KHR,
    GetPhysicalDeviceSurfaceFormats2KHR,
    GetPhysicalDeviceSurfaceCapabilities2EXT,
}
pub(crate) enum PromotedDispatch<T> {
    Dispatched(T),
    Unavailable,
}
impl EmulatedCommand {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::GetPhysicalDeviceFeatures2 => "vkGetPhysicalDeviceFeatures2",
            Self::GetPhysicalDeviceProperties2 => "vkGetPhysicalDeviceProperties2",
            Self::GetPhysicalDeviceFormatProperties2 => "vkGetPhysicalDeviceFormatProperties2",
            Self::GetPhysicalDeviceMemoryProperties2 => "vkGetPhysicalDeviceMemoryProperties2",
            Self::GetPhysicalDeviceImageFormatProperties2 => {
                "vkGetPhysicalDeviceImageFormatProperties2"
            }
            Self::GetPhysicalDeviceExternalBufferProperties => {
                "vkGetPhysicalDeviceExternalBufferProperties"
            }
            Self::GetPhysicalDeviceExternalSemaphoreProperties => {
                "vkGetPhysicalDeviceExternalSemaphoreProperties"
            }
            Self::GetPhysicalDeviceExternalFenceProperties => {
                "vkGetPhysicalDeviceExternalFenceProperties"
            }
            Self::GetPhysicalDeviceQueueFamilyProperties2 => {
                "vkGetPhysicalDeviceQueueFamilyProperties2"
            }
            Self::GetPhysicalDeviceSparseImageFormatProperties2 => {
                "vkGetPhysicalDeviceSparseImageFormatProperties2"
            }
            Self::GetPhysicalDeviceDisplayProperties2KHR => {
                "vkGetPhysicalDeviceDisplayProperties2KHR"
            }
            Self::GetPhysicalDeviceDisplayPlaneProperties2KHR => {
                "vkGetPhysicalDeviceDisplayPlaneProperties2KHR"
            }
            Self::GetDisplayModeProperties2KHR => "vkGetDisplayModeProperties2KHR",
            Self::GetDisplayPlaneCapabilities2KHR => "vkGetDisplayPlaneCapabilities2KHR",
            Self::GetPhysicalDeviceSurfaceCapabilities2KHR => {
                "vkGetPhysicalDeviceSurfaceCapabilities2KHR"
            }
            Self::GetPhysicalDeviceSurfaceFormats2KHR => "vkGetPhysicalDeviceSurfaceFormats2KHR",
            Self::GetPhysicalDeviceSurfaceCapabilities2EXT => {
                "vkGetPhysicalDeviceSurfaceCapabilities2EXT"
            }
        }
    }
    pub(crate) const fn diagnostic_legacy_name(self) -> Option<&'static str> {
        match self {
            Self::GetPhysicalDeviceExternalBufferProperties
            | Self::GetPhysicalDeviceExternalSemaphoreProperties
            | Self::GetPhysicalDeviceExternalFenceProperties
            | Self::GetPhysicalDeviceDisplayProperties2KHR
            | Self::GetPhysicalDeviceDisplayPlaneProperties2KHR
            | Self::GetDisplayModeProperties2KHR
            | Self::GetDisplayPlaneCapabilities2KHR => None,
            Self::GetPhysicalDeviceFeatures2 => Some("vkGetPhysicalDeviceFeatures"),
            Self::GetPhysicalDeviceFormatProperties2 => Some("vkGetPhysicalDeviceFormatProperties"),
            Self::GetPhysicalDeviceImageFormatProperties2 => {
                Some("vkGetPhysicalDeviceImageFormatProperties")
            }
            Self::GetPhysicalDeviceMemoryProperties2 => Some("vkGetPhysicalDeviceMemoryProperties"),
            Self::GetPhysicalDeviceProperties2 => Some("vkGetPhysicalDeviceProperties"),
            Self::GetPhysicalDeviceQueueFamilyProperties2 => {
                Some("vkGetPhysicalDeviceQueueFamilyProperties")
            }
            Self::GetPhysicalDeviceSparseImageFormatProperties2 => {
                Some("vkGetPhysicalDeviceSparseImageFormatProperties")
            }
            Self::GetPhysicalDeviceSurfaceCapabilities2KHR
            | Self::GetPhysicalDeviceSurfaceCapabilities2EXT => {
                Some("vkGetPhysicalDeviceSurfaceCapabilitiesKHR")
            }
            Self::GetPhysicalDeviceSurfaceFormats2KHR => {
                Some("vkGetPhysicalDeviceSurfaceFormatsKHR")
            }
        }
    }
}
pub(crate) unsafe fn dispatch_promoted_features2(
    device: &LoaderPhysicalDevice,
    pFeatures: *mut vk::VkPhysicalDeviceFeatures2<'_>,
) -> PromotedDispatch<()> {
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFeatures2
    {
        return {
            unsafe {
                command(device.native, pFeatures);
            }
            PromotedDispatch::Dispatched(())
        };
    }
    if device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFeatures2KHR
    {
        return {
            unsafe {
                command(device.native, pFeatures.cast());
            }
            PromotedDispatch::Dispatched(())
        };
    }
    PromotedDispatch::Unavailable
}
pub(crate) unsafe fn dispatch_promoted_properties2(
    device: &LoaderPhysicalDevice,
    pProperties: *mut vk::VkPhysicalDeviceProperties2<'_>,
) -> PromotedDispatch<()> {
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceProperties2
    {
        return {
            unsafe {
                command(device.native, pProperties);
            }
            PromotedDispatch::Dispatched(())
        };
    }
    if device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceProperties2KHR
    {
        return {
            unsafe {
                command(device.native, pProperties.cast());
            }
            PromotedDispatch::Dispatched(())
        };
    }
    PromotedDispatch::Unavailable
}
pub(crate) unsafe fn dispatch_promoted_format_properties2(
    device: &LoaderPhysicalDevice,
    format: vk::VkFormat,
    pFormatProperties: *mut vk::VkFormatProperties2<'_>,
) -> PromotedDispatch<()> {
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceFormatProperties2
    {
        return {
            unsafe {
                command(device.native, format, pFormatProperties);
            }
            PromotedDispatch::Dispatched(())
        };
    }
    if device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceFormatProperties2KHR
    {
        return {
            unsafe {
                command(device.native, format, pFormatProperties.cast());
            }
            PromotedDispatch::Dispatched(())
        };
    }
    PromotedDispatch::Unavailable
}
pub(crate) unsafe fn dispatch_promoted_memory_properties2(
    device: &LoaderPhysicalDevice,
    pMemoryProperties: *mut vk::VkPhysicalDeviceMemoryProperties2<'_>,
) -> PromotedDispatch<()> {
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device.icd().dispatch.vkGetPhysicalDeviceMemoryProperties2
    {
        return {
            unsafe {
                command(device.native, pMemoryProperties);
            }
            PromotedDispatch::Dispatched(())
        };
    }
    if device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceMemoryProperties2KHR
    {
        return {
            unsafe {
                command(device.native, pMemoryProperties.cast());
            }
            PromotedDispatch::Dispatched(())
        };
    }
    PromotedDispatch::Unavailable
}
pub(crate) unsafe fn dispatch_promoted_image_format_properties2(
    device: &LoaderPhysicalDevice,
    pImageFormatInfo: *const vk::VkPhysicalDeviceImageFormatInfo2<'_>,
    pImageFormatProperties: *mut vk::VkImageFormatProperties2<'_>,
) -> PromotedDispatch<vk::VkResult> {
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceImageFormatProperties2
    {
        return {
            PromotedDispatch::Dispatched(unsafe {
                command(device.native, pImageFormatInfo, pImageFormatProperties)
            })
        };
    }
    if device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceImageFormatProperties2KHR
    {
        return {
            PromotedDispatch::Dispatched(unsafe {
                command(
                    device.native,
                    pImageFormatInfo.cast(),
                    pImageFormatProperties.cast(),
                )
            })
        };
    }
    PromotedDispatch::Unavailable
}
pub(crate) unsafe fn dispatch_promoted_external_buffer_properties(
    device: &LoaderPhysicalDevice,
    pExternalBufferInfo: *const vk::VkPhysicalDeviceExternalBufferInfo<'_>,
    pExternalBufferProperties: *mut vk::VkExternalBufferProperties<'_>,
) -> PromotedDispatch<()> {
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalBufferProperties
    {
        return {
            unsafe {
                command(
                    device.native,
                    pExternalBufferInfo,
                    pExternalBufferProperties,
                );
            }
            PromotedDispatch::Dispatched(())
        };
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
        return {
            unsafe {
                command(
                    device.native,
                    pExternalBufferInfo.cast(),
                    pExternalBufferProperties.cast(),
                );
            }
            PromotedDispatch::Dispatched(())
        };
    }
    PromotedDispatch::Unavailable
}
pub(crate) unsafe fn dispatch_promoted_external_semaphore_properties(
    device: &LoaderPhysicalDevice,
    pExternalSemaphoreInfo: *const vk::VkPhysicalDeviceExternalSemaphoreInfo<'_>,
    pExternalSemaphoreProperties: *mut vk::VkExternalSemaphoreProperties<'_>,
) -> PromotedDispatch<()> {
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalSemaphoreProperties
    {
        return {
            unsafe {
                command(
                    device.native,
                    pExternalSemaphoreInfo,
                    pExternalSemaphoreProperties,
                );
            }
            PromotedDispatch::Dispatched(())
        };
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
        return {
            unsafe {
                command(
                    device.native,
                    pExternalSemaphoreInfo.cast(),
                    pExternalSemaphoreProperties.cast(),
                );
            }
            PromotedDispatch::Dispatched(())
        };
    }
    PromotedDispatch::Unavailable
}
pub(crate) unsafe fn dispatch_promoted_external_fence_properties(
    device: &LoaderPhysicalDevice,
    pExternalFenceInfo: *const vk::VkPhysicalDeviceExternalFenceInfo<'_>,
    pExternalFenceProperties: *mut vk::VkExternalFenceProperties<'_>,
) -> PromotedDispatch<()> {
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceExternalFenceProperties
    {
        return {
            unsafe {
                command(device.native, pExternalFenceInfo, pExternalFenceProperties);
            }
            PromotedDispatch::Dispatched(())
        };
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
        return {
            unsafe {
                command(
                    device.native,
                    pExternalFenceInfo.cast(),
                    pExternalFenceProperties.cast(),
                );
            }
            PromotedDispatch::Dispatched(())
        };
    }
    PromotedDispatch::Unavailable
}
pub(crate) unsafe fn dispatch_promoted_queue_family_properties2(
    device: &LoaderPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut vk::VkQueueFamilyProperties2<'_>,
) -> PromotedDispatch<()> {
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceQueueFamilyProperties2
    {
        return {
            unsafe {
                command(
                    device.native,
                    pQueueFamilyPropertyCount,
                    pQueueFamilyProperties,
                );
            }
            PromotedDispatch::Dispatched(())
        };
    }
    if device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceQueueFamilyProperties2KHR
    {
        return {
            unsafe {
                command(
                    device.native,
                    pQueueFamilyPropertyCount,
                    pQueueFamilyProperties.cast(),
                );
            }
            PromotedDispatch::Dispatched(())
        };
    }
    PromotedDispatch::Unavailable
}
pub(crate) unsafe fn dispatch_promoted_sparse_image_format_properties2(
    device: &LoaderPhysicalDevice,
    pFormatInfo: *const vk::VkPhysicalDeviceSparseImageFormatInfo2<'_>,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkSparseImageFormatProperties2<'_>,
) -> PromotedDispatch<()> {
    if device.app_api_version >= vk::VK_API_VERSION_1_1
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceSparseImageFormatProperties2
    {
        return {
            unsafe {
                command(device.native, pFormatInfo, pPropertyCount, pProperties);
            }
            PromotedDispatch::Dispatched(())
        };
    }
    if device
        .instance()
        .enabled_extensions
        .contains_name(vk::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME)
        && let Some(command) = device
            .icd()
            .dispatch
            .vkGetPhysicalDeviceSparseImageFormatProperties2KHR
    {
        return {
            unsafe {
                command(
                    device.native,
                    pFormatInfo.cast(),
                    pPropertyCount,
                    pProperties.cast(),
                );
            }
            PromotedDispatch::Dispatched(())
        };
    }
    PromotedDispatch::Unavailable
}
