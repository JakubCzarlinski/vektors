// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.

#[cfg(feature = "wsi-directfb")]
use super::extensions::VK_EXT_DIRECTFB_SURFACE_EXTENSION_ID;
use super::extensions::VK_EXT_HEADLESS_SURFACE_EXTENSION_ID;
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
use super::extensions::VK_EXT_METAL_SURFACE_EXTENSION_ID;
#[cfg(target_os = "fuchsia")]
use super::extensions::VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_ID;
#[cfg(feature = "platform-ggp")]
use super::extensions::VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_ID;
#[cfg(target_os = "android")]
use super::extensions::VK_KHR_ANDROID_SURFACE_EXTENSION_ID;
use super::extensions::VK_KHR_DISPLAY_EXTENSION_ID;
#[cfg(all(
    feature = "wsi-wayland",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
use super::extensions::VK_KHR_WAYLAND_SURFACE_EXTENSION_ID;
#[cfg(target_os = "windows")]
use super::extensions::VK_KHR_WIN32_SURFACE_EXTENSION_ID;
#[cfg(all(
    feature = "wsi-xcb",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
use super::extensions::VK_KHR_XCB_SURFACE_EXTENSION_ID;
#[cfg(all(
    feature = "wsi-xlib",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
use super::extensions::VK_KHR_XLIB_SURFACE_EXTENSION_ID;
#[cfg(target_os = "ios")]
use super::extensions::VK_MVK_IOS_SURFACE_EXTENSION_ID;
#[cfg(target_os = "macos")]
use super::extensions::VK_MVK_MACOS_SURFACE_EXTENSION_ID;
#[cfg(feature = "platform-vi")]
use super::extensions::VK_NN_VI_SURFACE_EXTENSION_ID;
#[cfg(target_env = "ohos")]
use super::extensions::VK_OHOS_SURFACE_EXTENSION_ID;
#[cfg(any(target_os = "nto", target_os = "qnx"))]
use super::extensions::VK_QNX_SCREEN_SURFACE_EXTENSION_ID;
#[cfg(feature = "platform-ubm")]
use super::extensions::VK_SEC_UBM_SURFACE_EXTENSION_ID;
use crate::VkStructureType;
use crate::create_loader_surface;
use crate::fatal_loader_error;
use crate::promoted;
use crate::resolve_physical_device;
use crate::translate_physical_device_surface;
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceFeatures2(
    physicalDevice: vk::VkPhysicalDevice,
    pFeatures: *mut vk::VkPhysicalDeviceFeatures2<'_>,
) {
    unsafe {
        promoted::features2_impl(physicalDevice, pFeatures);
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceFeatures2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pFeatures: *mut vk::VkPhysicalDeviceFeatures2KHR<'_>,
) {
    unsafe {
        promoted::features2_impl(physicalDevice, pFeatures.cast());
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    pProperties: *mut vk::VkPhysicalDeviceProperties2<'_>,
) {
    unsafe {
        promoted::properties2_impl(physicalDevice, pProperties);
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pProperties: *mut vk::VkPhysicalDeviceProperties2KHR<'_>,
) {
    unsafe {
        promoted::properties2_impl(physicalDevice, pProperties.cast());
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceFormatProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    pFormatProperties: *mut vk::VkFormatProperties2<'_>,
) {
    unsafe {
        promoted::format_properties2_impl(physicalDevice, format, pFormatProperties);
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceFormatProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    pFormatProperties: *mut vk::VkFormatProperties2KHR<'_>,
) {
    unsafe {
        promoted::format_properties2_impl(physicalDevice, format, pFormatProperties.cast());
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceMemoryProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    pMemoryProperties: *mut vk::VkPhysicalDeviceMemoryProperties2<'_>,
) {
    unsafe {
        promoted::memory_properties2_impl(physicalDevice, pMemoryProperties);
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceMemoryProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pMemoryProperties: *mut vk::VkPhysicalDeviceMemoryProperties2KHR<'_>,
) {
    unsafe {
        promoted::memory_properties2_impl(physicalDevice, pMemoryProperties.cast());
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceImageFormatProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    pImageFormatInfo: *const vk::VkPhysicalDeviceImageFormatInfo2<'_>,
    pImageFormatProperties: *mut vk::VkImageFormatProperties2<'_>,
) -> vk::VkResult {
    unsafe {
        promoted::image_format_properties2_impl(
            physicalDevice,
            pImageFormatInfo,
            pImageFormatProperties,
        )
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceImageFormatProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pImageFormatInfo: *const vk::VkPhysicalDeviceImageFormatInfo2KHR<'_>,
    pImageFormatProperties: *mut vk::VkImageFormatProperties2KHR<'_>,
) -> vk::VkResult {
    unsafe {
        promoted::image_format_properties2_impl(
            physicalDevice,
            pImageFormatInfo.cast(),
            pImageFormatProperties.cast(),
        )
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalBufferProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalBufferInfo: *const vk::VkPhysicalDeviceExternalBufferInfo<'_>,
    pExternalBufferProperties: *mut vk::VkExternalBufferProperties<'_>,
) {
    unsafe {
        promoted::external_buffer_properties_impl(
            physicalDevice,
            pExternalBufferInfo,
            pExternalBufferProperties,
        );
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalBufferPropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalBufferInfo: *const vk::VkPhysicalDeviceExternalBufferInfoKHR<'_>,
    pExternalBufferProperties: *mut vk::VkExternalBufferPropertiesKHR<'_>,
) {
    unsafe {
        promoted::external_buffer_properties_impl(
            physicalDevice,
            pExternalBufferInfo.cast(),
            pExternalBufferProperties.cast(),
        );
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalSemaphoreProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalSemaphoreInfo: *const vk::VkPhysicalDeviceExternalSemaphoreInfo<'_>,
    pExternalSemaphoreProperties: *mut vk::VkExternalSemaphoreProperties<'_>,
) {
    unsafe {
        promoted::external_semaphore_properties_impl(
            physicalDevice,
            pExternalSemaphoreInfo,
            pExternalSemaphoreProperties,
        );
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalSemaphoreInfo: *const vk::VkPhysicalDeviceExternalSemaphoreInfoKHR<'_>,
    pExternalSemaphoreProperties: *mut vk::VkExternalSemaphorePropertiesKHR<'_>,
) {
    unsafe {
        promoted::external_semaphore_properties_impl(
            physicalDevice,
            pExternalSemaphoreInfo.cast(),
            pExternalSemaphoreProperties.cast(),
        );
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalFenceProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalFenceInfo: *const vk::VkPhysicalDeviceExternalFenceInfo<'_>,
    pExternalFenceProperties: *mut vk::VkExternalFenceProperties<'_>,
) {
    unsafe {
        promoted::external_fence_properties_impl(
            physicalDevice,
            pExternalFenceInfo,
            pExternalFenceProperties,
        );
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalFencePropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalFenceInfo: *const vk::VkPhysicalDeviceExternalFenceInfoKHR<'_>,
    pExternalFenceProperties: *mut vk::VkExternalFencePropertiesKHR<'_>,
) {
    unsafe {
        promoted::external_fence_properties_impl(
            physicalDevice,
            pExternalFenceInfo.cast(),
            pExternalFenceProperties.cast(),
        );
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceQueueFamilyProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut vk::VkQueueFamilyProperties2<'_>,
) {
    unsafe {
        promoted::queue_family_properties2_impl(
            physicalDevice,
            pQueueFamilyPropertyCount,
            pQueueFamilyProperties,
        );
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceQueueFamilyProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut vk::VkQueueFamilyProperties2KHR<'_>,
) {
    unsafe {
        promoted::queue_family_properties2_impl(
            physicalDevice,
            pQueueFamilyPropertyCount,
            pQueueFamilyProperties.cast(),
        );
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSparseImageFormatProperties2(
    physicalDevice: vk::VkPhysicalDevice,
    pFormatInfo: *const vk::VkPhysicalDeviceSparseImageFormatInfo2<'_>,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkSparseImageFormatProperties2<'_>,
) {
    unsafe {
        promoted::sparse_image_format_properties2_impl(
            physicalDevice,
            pFormatInfo,
            pPropertyCount,
            pProperties,
        );
    }
}
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
    physicalDevice: vk::VkPhysicalDevice,
    pFormatInfo: *const vk::VkPhysicalDeviceSparseImageFormatInfo2KHR<'_>,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkSparseImageFormatProperties2KHR<'_>,
) {
    unsafe {
        promoted::sparse_image_format_properties2_impl(
            physicalDevice,
            pFormatInfo.cast(),
            pPropertyCount,
            pProperties.cast(),
        );
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkAcquireDrmDisplayEXT(
    physicalDevice: vk::VkPhysicalDevice,
    drmFd: i32,
    display: vk::VkDisplayKHR,
) -> vk::VkResult {
    let command: Option<(vk::PFN_vkAcquireDrmDisplayEXT, vk::VkPhysicalDevice)> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkAcquireDrmDisplayEXT,
            c"vkAcquireDrmDisplayEXT",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_EXTENSION_NOT_PRESENT
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, drmFd, display) },
    )
}
#[cfg(target_os = "windows")]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkAcquireWinrtDisplayNV(
    physicalDevice: vk::VkPhysicalDevice,
    display: vk::VkDisplayKHR,
) -> vk::VkResult {
    let command: Option<(vk::PFN_vkAcquireWinrtDisplayNV, vk::VkPhysicalDevice)> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkAcquireWinrtDisplayNV,
            c"vkAcquireWinrtDisplayNV",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            fatal_loader_error(c"vkAcquireWinrtDisplayNV: Driver's function pointer was NULL")
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, display) },
    )
}
#[cfg(all(
    feature = "wsi-xlib-xrandr",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkAcquireXlibDisplayEXT(
    physicalDevice: vk::VkPhysicalDevice,
    dpy: *mut vk::Display,
    display: vk::VkDisplayKHR,
) -> vk::VkResult {
    let command: Option<(vk::PFN_vkAcquireXlibDisplayEXT, vk::VkPhysicalDevice)> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkAcquireXlibDisplayEXT,
            c"vkAcquireXlibDisplayEXT",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, dpy, display) },
    )
}
#[cfg(target_os = "android")]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateAndroidSurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkAndroidSurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::ANDROID_SURFACE_CREATE_INFO_KHR,
            pAllocator,
            pSurface,
            c"vkCreateAndroidSurfaceKHR",
            VK_KHR_ANDROID_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(feature = "wsi-directfb")]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateDirectFBSurfaceEXT(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkDirectFBSurfaceCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::DIRECTFB_SURFACE_CREATE_INFO_EXT,
            pAllocator,
            pSurface,
            c"vkCreateDirectFBSurfaceEXT",
            VK_EXT_DIRECTFB_SURFACE_EXTENSION_ID,
        )
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateDisplayModeKHR(
    physicalDevice: vk::VkPhysicalDevice,
    display: vk::VkDisplayKHR,
    pCreateInfo: *const vk::VkDisplayModeCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pMode: *mut vk::VkDisplayModeKHR,
) -> vk::VkResult {
    let command: Option<(vk::PFN_vkCreateDisplayModeKHR, vk::VkPhysicalDevice)> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkCreateDisplayModeKHR,
            c"vkCreateDisplayModeKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, display, pCreateInfo, pAllocator, pMode)
        },
    )
}
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateDisplayPlaneSurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkDisplaySurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::DISPLAY_SURFACE_CREATE_INFO_KHR,
            pAllocator,
            pSurface,
            c"vkCreateDisplayPlaneSurfaceKHR",
            VK_KHR_DISPLAY_EXTENSION_ID,
        )
    }
}
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateHeadlessSurfaceEXT(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkHeadlessSurfaceCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::HEADLESS_SURFACE_CREATE_INFO_EXT,
            pAllocator,
            pSurface,
            c"vkCreateHeadlessSurfaceEXT",
            VK_EXT_HEADLESS_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(target_os = "ios")]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateIOSSurfaceMVK(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkIOSSurfaceCreateInfoMVK<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::IOS_SURFACE_CREATE_INFO_MVK,
            pAllocator,
            pSurface,
            c"vkCreateIOSSurfaceMVK",
            VK_MVK_IOS_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(target_os = "fuchsia")]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateImagePipeSurfaceFUCHSIA(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkImagePipeSurfaceCreateInfoFUCHSIA<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA,
            pAllocator,
            pSurface,
            c"vkCreateImagePipeSurfaceFUCHSIA",
            VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(target_os = "macos")]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateMacOSSurfaceMVK(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkMacOSSurfaceCreateInfoMVK<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::MACOS_SURFACE_CREATE_INFO_MVK,
            pAllocator,
            pSurface,
            c"vkCreateMacOSSurfaceMVK",
            VK_MVK_MACOS_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateMetalSurfaceEXT(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkMetalSurfaceCreateInfoEXT<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::METAL_SURFACE_CREATE_INFO_EXT,
            pAllocator,
            pSurface,
            c"vkCreateMetalSurfaceEXT",
            VK_EXT_METAL_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(any(target_os = "nto", target_os = "qnx"))]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateScreenSurfaceQNX(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkScreenSurfaceCreateInfoQNX<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::SCREEN_SURFACE_CREATE_INFO_QNX,
            pAllocator,
            pSurface,
            c"vkCreateScreenSurfaceQNX",
            VK_QNX_SCREEN_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(feature = "platform-ggp")]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateStreamDescriptorSurfaceGGP(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkStreamDescriptorSurfaceCreateInfoGGP<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP,
            pAllocator,
            pSurface,
            c"vkCreateStreamDescriptorSurfaceGGP",
            VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(target_env = "ohos")]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateSurfaceOHOS(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkSurfaceCreateInfoOHOS<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::SURFACE_CREATE_INFO_OHOS,
            pAllocator,
            pSurface,
            c"vkCreateSurfaceOHOS",
            VK_OHOS_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(feature = "platform-ubm")]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateUbmSurfaceSEC(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkUbmSurfaceCreateInfoSEC<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::UBM_SURFACE_CREATE_INFO_SEC,
            pAllocator,
            pSurface,
            c"vkCreateUbmSurfaceSEC",
            VK_SEC_UBM_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(feature = "platform-vi")]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateViSurfaceNN(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkViSurfaceCreateInfoNN<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::VI_SURFACE_CREATE_INFO_NN,
            pAllocator,
            pSurface,
            c"vkCreateViSurfaceNN",
            VK_NN_VI_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(all(
    feature = "wsi-wayland",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateWaylandSurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkWaylandSurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::WAYLAND_SURFACE_CREATE_INFO_KHR,
            pAllocator,
            pSurface,
            c"vkCreateWaylandSurfaceKHR",
            VK_KHR_WAYLAND_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(target_os = "windows")]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateWin32SurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkWin32SurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            pAllocator,
            pSurface,
            c"vkCreateWin32SurfaceKHR",
            VK_KHR_WIN32_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(all(
    feature = "wsi-xcb",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateXcbSurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkXcbSurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::XCB_SURFACE_CREATE_INFO_KHR,
            pAllocator,
            pSurface,
            c"vkCreateXcbSurfaceKHR",
            VK_KHR_XCB_SURFACE_EXTENSION_ID,
        )
    }
}
#[cfg(all(
    feature = "wsi-xlib",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Creates loader-owned WSI state at the bottom of an instance layer chain.
///
/// # Safety
///
/// The instance must identify a live loader terminator and all pointers must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkCreateXlibSurfaceKHR(
    instance: vk::VkInstance,
    pCreateInfo: *const vk::VkXlibSurfaceCreateInfoKHR<'_>,
    pAllocator: *const vk::VkAllocationCallbacks<'_>,
    pSurface: *mut vk::VkSurfaceKHR,
) -> vk::VkResult {
    unsafe {
        create_loader_surface(
            instance,
            pCreateInfo,
            VkStructureType::XLIB_SURFACE_CREATE_INFO_KHR,
            pAllocator,
            pSurface,
            c"vkCreateXlibSurfaceKHR",
            VK_KHR_XLIB_SURFACE_EXTENSION_ID,
        )
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    pCounterCount: *mut u32,
    pCounters: *mut vk::VkPerformanceCounterARM<'_>,
    pCounterDescriptions: *mut vk::VkPerformanceCounterDescriptionARM<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM,
            c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(
                    physicalDevice,
                    queueFamilyIndex,
                    pCounterCount,
                    pCounters,
                    pCounterDescriptions,
                )
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    pCounterCount: *mut u32,
    pCounters: *mut vk::VkPerformanceCounterKHR<'_>,
    pCounterDescriptions: *mut vk::VkPerformanceCounterDescriptionKHR<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
            c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(
                    physicalDevice,
                    queueFamilyIndex,
                    pCounterCount,
                    pCounters,
                    pCounterDescriptions,
                )
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM(
    physicalDevice: vk::VkPhysicalDevice,
    pDescriptionCount: *mut u32,
    pDescriptions: *mut vk::VkShaderInstrumentationMetricDescriptionARM<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM,
            c"vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(physicalDevice, pDescriptionCount, pDescriptions)
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetDisplayModePropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    display: vk::VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkDisplayModePropertiesKHR,
) -> vk::VkResult {
    let command: Option<(vk::PFN_vkGetDisplayModePropertiesKHR, vk::VkPhysicalDevice)> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetDisplayModePropertiesKHR,
            c"vkGetDisplayModePropertiesKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            {
                if !pPropertyCount.is_null() {
                    unsafe {
                        pPropertyCount.write(0);
                    }
                }
                vk::VkResult::SUCCESS
            }
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, display, pPropertyCount, pProperties)
        },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetDisplayPlaneCapabilitiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    mode: vk::VkDisplayModeKHR,
    planeIndex: u32,
    pCapabilities: *mut vk::VkDisplayPlaneCapabilitiesKHR,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetDisplayPlaneCapabilitiesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetDisplayPlaneCapabilitiesKHR,
            c"vkGetDisplayPlaneCapabilitiesKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            {
                if !pCapabilities.is_null() {
                    unsafe {
                        pCapabilities.write(vk::VkDisplayPlaneCapabilitiesKHR::DEFAULT);
                    }
                }
                vk::VkResult::SUCCESS
            }
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, mode, planeIndex, pCapabilities)
        },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetDisplayPlaneSupportedDisplaysKHR(
    physicalDevice: vk::VkPhysicalDevice,
    planeIndex: u32,
    pDisplayCount: *mut u32,
    pDisplays: *mut vk::VkDisplayKHR,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetDisplayPlaneSupportedDisplaysKHR,
            c"vkGetDisplayPlaneSupportedDisplaysKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            {
                if !pDisplayCount.is_null() {
                    unsafe {
                        pDisplayCount.write(0);
                    }
                }
                vk::VkResult::SUCCESS
            }
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, planeIndex, pDisplayCount, pDisplays)
        },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetDrmDisplayEXT(
    physicalDevice: vk::VkPhysicalDevice,
    drmFd: i32,
    connectorId: u32,
    display: *mut vk::VkDisplayKHR,
) -> vk::VkResult {
    let command: Option<(vk::PFN_vkGetDrmDisplayEXT, vk::VkPhysicalDevice)> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetDrmDisplayEXT,
            c"vkGetDrmDisplayEXT",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_EXTENSION_NOT_PRESENT
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, drmFd, connectorId, display) },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(
    physicalDevice: vk::VkPhysicalDevice,
    pTimeDomainCount: *mut u32,
    pTimeDomains: *mut vk::VkTimeDomainEXT,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceCalibrateableTimeDomainsEXT,
            c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(physicalDevice, pTimeDomainCount, pTimeDomains)
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pTimeDomainCount: *mut u32,
    pTimeDomains: *mut vk::VkTimeDomainKHR,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
            c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(physicalDevice, pTimeDomainCount, pTimeDomains)
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkCooperativeMatrixFlexibleDimensionsPropertiesNV<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV,
            c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(physicalDevice, pPropertyCount, pProperties)
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT(
    physicalDevice: vk::VkPhysicalDevice,
    pCooperativeMatrixInfo: *const vk::VkPhysicalDeviceCooperativeMatrixInfo2EXT<'_>,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkCooperativeMatrixProperties2EXT<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceCooperativeMatrixProperties2EXT,
            c"vkGetPhysicalDeviceCooperativeMatrixProperties2EXT",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceCooperativeMatrixProperties2EXT: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(
                    physicalDevice,
                    pCooperativeMatrixInfo,
                    pPropertyCount,
                    pProperties,
                )
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkCooperativeMatrixPropertiesKHR<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR,
            c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(physicalDevice, pPropertyCount, pProperties)
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkCooperativeMatrixPropertiesNV<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
            c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(physicalDevice, pPropertyCount, pProperties)
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceCooperativeVectorPropertiesNV(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkCooperativeVectorPropertiesNV<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceCooperativeVectorPropertiesNV,
            c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(physicalDevice, pPropertyCount, pProperties)
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceDescriptorSizeEXT(
    physicalDevice: vk::VkPhysicalDevice,
    descriptorType: vk::VkDescriptorType,
) -> vk::VkDeviceSize {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceDescriptorSizeEXT,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceDescriptorSizeEXT,
            c"vkGetPhysicalDeviceDescriptorSizeEXT",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            fatal_loader_error(
                c"vkGetPhysicalDeviceDescriptorSizeEXT: Driver's function pointer was NULL",
            )
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, descriptorType) },
    )
}
#[cfg(feature = "wsi-directfb")]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceDirectFBPresentationSupportEXT(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    dfb: *mut vk::IDirectFB,
) -> vk::VkBool32 {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceDirectFBPresentationSupportEXT,
            c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, queueFamilyIndex, dfb) },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkDisplayPlanePropertiesKHR,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
            c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            {
                if !pPropertyCount.is_null() {
                    unsafe {
                        pPropertyCount.write(0);
                    }
                }
                vk::VkResult::SUCCESS
            }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pPropertyCount, pProperties) },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceDisplayPropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkDisplayPropertiesKHR<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceDisplayPropertiesKHR,
            c"vkGetPhysicalDeviceDisplayPropertiesKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            {
                if !pPropertyCount.is_null() {
                    unsafe {
                        pPropertyCount.write(0);
                    }
                }
                vk::VkResult::SUCCESS
            }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, pPropertyCount, pProperties) },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    type_: vk::VkImageType,
    tiling: vk::VkImageTiling,
    usage: vk::VkImageUsageFlags,
    flags: vk::VkImageCreateFlags,
    externalHandleType: vk::VkExternalMemoryHandleTypeFlagsNV,
    pExternalImageFormatProperties: *mut vk::VkExternalImageFormatPropertiesNV,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
            c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                format,
                type_,
                tiling,
                usage,
                flags,
                externalHandleType,
                pExternalImageFormatProperties,
            )
        },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceExternalTensorPropertiesARM(
    physicalDevice: vk::VkPhysicalDevice,
    pExternalTensorInfo: *const vk::VkPhysicalDeviceExternalTensorInfoARM<'_>,
    pExternalTensorProperties: *mut vk::VkExternalTensorPropertiesARM<'_>,
) {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceExternalTensorPropertiesARM,
            c"vkGetPhysicalDeviceExternalTensorPropertiesARM",
        )
    };
    let Some((command, physicalDevice)) = command else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceExternalTensorPropertiesARM: Driver's function pointer was NULL",
        )
    };
    unsafe {
        command(
            physicalDevice,
            pExternalTensorInfo,
            pExternalTensorProperties,
        );
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceFeatures(
    physicalDevice: vk::VkPhysicalDevice,
    pFeatures: *mut vk::VkPhysicalDeviceFeatures,
) {
    let command: Option<(vk::PFN_vkGetPhysicalDeviceFeatures, vk::VkPhysicalDevice)> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceFeatures,
            c"vkGetPhysicalDeviceFeatures",
        )
    };
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pFeatures);
        }
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceFormatProperties(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    pFormatProperties: *mut vk::VkFormatProperties,
) {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceFormatProperties,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceFormatProperties,
            c"vkGetPhysicalDeviceFormatProperties",
        )
    };
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, format, pFormatProperties);
        }
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceFragmentShadingRatesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pFragmentShadingRateCount: *mut u32,
    pFragmentShadingRates: *mut vk::VkPhysicalDeviceFragmentShadingRateKHR<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceFragmentShadingRatesKHR,
            c"vkGetPhysicalDeviceFragmentShadingRatesKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            fatal_loader_error(
                c"vkGetPhysicalDeviceFragmentShadingRatesKHR: Driver's function pointer was NULL",
            )
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                pFragmentShadingRateCount,
                pFragmentShadingRates,
            )
        },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceImageFormatProperties(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    type_: vk::VkImageType,
    tiling: vk::VkImageTiling,
    usage: vk::VkImageUsageFlags,
    flags: vk::VkImageCreateFlags,
    pImageFormatProperties: *mut vk::VkImageFormatProperties,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceImageFormatProperties,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceImageFormatProperties,
            c"vkGetPhysicalDeviceImageFormatProperties",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                format,
                type_,
                tiling,
                usage,
                flags,
                pImageFormatProperties,
            )
        },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceMemoryProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pMemoryProperties: *mut vk::VkPhysicalDeviceMemoryProperties,
) {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceMemoryProperties,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceMemoryProperties,
            c"vkGetPhysicalDeviceMemoryProperties",
        )
    };
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pMemoryProperties);
        }
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceMultisamplePropertiesEXT(
    physicalDevice: vk::VkPhysicalDevice,
    samples: vk::VkSampleCountFlagBits,
    pMultisampleProperties: *mut vk::VkMultisamplePropertiesEXT<'_>,
) {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceMultisamplePropertiesEXT,
            c"vkGetPhysicalDeviceMultisamplePropertiesEXT",
        )
    };
    let Some((command, physicalDevice)) = command else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceMultisamplePropertiesEXT: Driver's function pointer was NULL",
        )
    };
    unsafe {
        command(physicalDevice, samples, pMultisampleProperties);
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceOpticalFlowImageFormatsNV(
    physicalDevice: vk::VkPhysicalDevice,
    pOpticalFlowImageFormatInfo: *const vk::VkOpticalFlowImageFormatInfoNV<'_>,
    pFormatCount: *mut u32,
    pImageFormatProperties: *mut vk::VkOpticalFlowImageFormatPropertiesNV<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceOpticalFlowImageFormatsNV,
            c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            fatal_loader_error(
                c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV: Driver's function pointer was NULL",
            )
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                pOpticalFlowImageFormatInfo,
                pFormatCount,
                pImageFormatProperties,
            )
        },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDevicePresentRectanglesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    surface: vk::VkSurfaceKHR,
    pRectCount: *mut u32,
    pRects: *mut vk::VkRect2D,
) -> vk::VkResult {
    let surface = match unsafe { translate_physical_device_surface(physicalDevice, surface) } {
        Ok(surface) => surface,
        Err(result) => {
            core::hint::cold_path();
            return result;
        }
    };
    let command: Option<(
        vk::PFN_vkGetPhysicalDevicePresentRectanglesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDevicePresentRectanglesKHR,
            c"vkGetPhysicalDevicePresentRectanglesKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            fatal_loader_error(
                c"vkGetPhysicalDevicePresentRectanglesKHR: Driver's function pointer was NULL",
            )
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, surface, pRectCount, pRects) },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pProperties: *mut vk::VkPhysicalDeviceProperties,
) {
    let command: Option<(vk::PFN_vkGetPhysicalDeviceProperties, vk::VkPhysicalDevice)> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceProperties,
            c"vkGetPhysicalDeviceProperties",
        )
    };
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(physicalDevice, pProperties);
        }
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphProperties: *const vk::VkQueueFamilyDataGraphPropertiesARM<'_>,
    pProperties: *mut vk::VkBaseOutStructure<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM,
            c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(
                    physicalDevice,
                    queueFamilyIndex,
                    pQueueFamilyDataGraphProperties,
                    pProperties,
                )
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphProperties: *const vk::VkQueueFamilyDataGraphPropertiesARM<'_>,
    pOpticalFlowImageFormatInfo: *const vk::VkDataGraphOpticalFlowImageFormatInfoARM<'_>,
    pFormatCount: *mut u32,
    pImageFormatProperties: *mut vk::VkDataGraphOpticalFlowImageFormatPropertiesARM<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM,
            c"vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(
                    physicalDevice,
                    queueFamilyIndex,
                    pQueueFamilyDataGraphProperties,
                    pOpticalFlowImageFormatInfo,
                    pFormatCount,
                    pImageFormatProperties,
                )
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM(
    physicalDevice: vk::VkPhysicalDevice,
    pQueueFamilyDataGraphProcessingEngineInfo: *const vk::VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<
        '_,
    >,
    pQueueFamilyDataGraphProcessingEngineProperties: *mut vk::VkQueueFamilyDataGraphProcessingEnginePropertiesARM<
        '_,
    >,
) {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| {
                dispatch.vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM
            },
            c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM",
        )
    };
    let Some((command, physicalDevice)) = command else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM: Driver's function pointer was NULL",
        )
    };
    unsafe {
        command(
            physicalDevice,
            pQueueFamilyDataGraphProcessingEngineInfo,
            pQueueFamilyDataGraphProcessingEngineProperties,
        );
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphPropertyCount: *mut u32,
    pQueueFamilyDataGraphProperties: *mut vk::VkQueueFamilyDataGraphPropertiesARM<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM,
            c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(
                    physicalDevice,
                    queueFamilyIndex,
                    pQueueFamilyDataGraphPropertyCount,
                    pQueueFamilyDataGraphProperties,
                )
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pPerformanceQueryCreateInfo: *const vk::VkQueryPoolPerformanceCreateInfoKHR<'_>,
    pNumPasses: *mut u32,
) {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
            c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR",
        )
    };
    let Some((command, physicalDevice)) = command else {
        fatal_loader_error(
            c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR: Driver's function pointer was NULL",
        )
    };
    unsafe {
        command(physicalDevice, pPerformanceQueryCreateInfo, pNumPasses);
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceQueueFamilyProperties(
    physicalDevice: vk::VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut vk::VkQueueFamilyProperties,
) {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceQueueFamilyProperties,
            c"vkGetPhysicalDeviceQueueFamilyProperties",
        )
    };
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                pQueueFamilyPropertyCount,
                pQueueFamilyProperties,
            );
        }
    }
}
#[cfg(any(target_os = "nto", target_os = "qnx"))]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceScreenPresentationSupportQNX(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    window: *mut vk::_screen_window,
) -> vk::VkBool32 {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceScreenPresentationSupportQNX,
            c"vkGetPhysicalDeviceScreenPresentationSupportQNX",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, queueFamilyIndex, window) },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSparseImageFormatProperties(
    physicalDevice: vk::VkPhysicalDevice,
    format: vk::VkFormat,
    type_: vk::VkImageType,
    samples: vk::VkSampleCountFlagBits,
    usage: vk::VkImageUsageFlags,
    tiling: vk::VkImageTiling,
    pPropertyCount: *mut u32,
    pProperties: *mut vk::VkSparseImageFormatProperties,
) {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceSparseImageFormatProperties,
            c"vkGetPhysicalDeviceSparseImageFormatProperties",
        )
    };
    if let Some((command, physicalDevice)) = command {
        unsafe {
            command(
                physicalDevice,
                format,
                type_,
                samples,
                usage,
                tiling,
                pPropertyCount,
                pProperties,
            );
        }
    }
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(
    physicalDevice: vk::VkPhysicalDevice,
    pCombinationCount: *mut u32,
    pCombinations: *mut vk::VkFramebufferMixedSamplesCombinationNV<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
            c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(physicalDevice, pCombinationCount, pCombinations)
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    surface: vk::VkSurfaceKHR,
    pSurfaceCapabilities: *mut vk::VkSurfaceCapabilitiesKHR,
) -> vk::VkResult {
    let surface = match unsafe { translate_physical_device_surface(physicalDevice, surface) } {
        Ok(surface) => surface,
        Err(result) => {
            core::hint::cold_path();
            return result;
        }
    };
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
            c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, surface, pSurfaceCapabilities)
        },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSurfaceFormatsKHR(
    physicalDevice: vk::VkPhysicalDevice,
    surface: vk::VkSurfaceKHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut vk::VkSurfaceFormatKHR,
) -> vk::VkResult {
    let surface = match unsafe { translate_physical_device_surface(physicalDevice, surface) } {
        Ok(surface) => surface,
        Err(result) => {
            core::hint::cold_path();
            return result;
        }
    };
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceSurfaceFormatsKHR,
            c"vkGetPhysicalDeviceSurfaceFormatsKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                surface,
                pSurfaceFormatCount,
                pSurfaceFormats,
            )
        },
    )
}
#[cfg(target_os = "windows")]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSurfacePresentModes2EXT(
    physicalDevice: vk::VkPhysicalDevice,
    pSurfaceInfo: *const vk::VkPhysicalDeviceSurfaceInfo2KHR<'_>,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut vk::VkPresentModeKHR,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceSurfacePresentModes2EXT,
            c"vkGetPhysicalDeviceSurfacePresentModes2EXT",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            fatal_loader_error(
                c"vkGetPhysicalDeviceSurfacePresentModes2EXT: Driver's function pointer was NULL",
            )
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                pSurfaceInfo,
                pPresentModeCount,
                pPresentModes,
            )
        },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceSurfacePresentModesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    surface: vk::VkSurfaceKHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut vk::VkPresentModeKHR,
) -> vk::VkResult {
    let surface = match unsafe { translate_physical_device_surface(physicalDevice, surface) } {
        Ok(surface) => surface,
        Err(result) => {
            core::hint::cold_path();
            return result;
        }
    };
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceSurfacePresentModesKHR,
            c"vkGetPhysicalDeviceSurfacePresentModesKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, surface, pPresentModeCount, pPresentModes)
        },
    )
}
#[cfg(feature = "platform-ubm")]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceUbmPresentationSupportSEC(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    device: *mut vk::ubm_device,
) -> vk::VkBool32 {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceUbmPresentationSupportSEC,
            c"vkGetPhysicalDeviceUbmPresentationSupportSEC",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, queueFamilyIndex, device) },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceVideoCapabilitiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pVideoProfile: *const vk::VkVideoProfileInfoKHR<'_>,
    pCapabilities: *mut vk::VkVideoCapabilitiesKHR<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceVideoCapabilitiesKHR,
            c"vkGetPhysicalDeviceVideoCapabilitiesKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            fatal_loader_error(
                c"vkGetPhysicalDeviceVideoCapabilitiesKHR: Driver's function pointer was NULL",
            )
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, pVideoProfile, pCapabilities)
        },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pQualityLevelInfo: *const vk::VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
    pQualityLevelProperties: *mut vk::VkVideoEncodeQualityLevelPropertiesKHR<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR,
            c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR",
        )
    };
    command
        .map_or_else(
            || {
                core::hint::cold_path();
                fatal_loader_error(
                    c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR: Driver's function pointer was NULL",
                )
            },
            |(command, physicalDevice)| unsafe {
                command(physicalDevice, pQualityLevelInfo, pQualityLevelProperties)
            },
        )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceVideoFormatPropertiesKHR(
    physicalDevice: vk::VkPhysicalDevice,
    pVideoFormatInfo: *const vk::VkPhysicalDeviceVideoFormatInfoKHR<'_>,
    pVideoFormatPropertyCount: *mut u32,
    pVideoFormatProperties: *mut vk::VkVideoFormatPropertiesKHR<'_>,
) -> vk::VkResult {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceVideoFormatPropertiesKHR,
            c"vkGetPhysicalDeviceVideoFormatPropertiesKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            fatal_loader_error(
                c"vkGetPhysicalDeviceVideoFormatPropertiesKHR: Driver's function pointer was NULL",
            )
        },
        |(command, physicalDevice)| unsafe {
            command(
                physicalDevice,
                pVideoFormatInfo,
                pVideoFormatPropertyCount,
                pVideoFormatProperties,
            )
        },
    )
}
#[cfg(all(
    feature = "wsi-wayland",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceWaylandPresentationSupportKHR(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    display: *mut vk::wl_display,
) -> vk::VkBool32 {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceWaylandPresentationSupportKHR,
            c"vkGetPhysicalDeviceWaylandPresentationSupportKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, queueFamilyIndex, display) },
    )
}
#[cfg(target_os = "windows")]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceWin32PresentationSupportKHR(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
) -> vk::VkBool32 {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceWin32PresentationSupportKHR,
            c"vkGetPhysicalDeviceWin32PresentationSupportKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, queueFamilyIndex) },
    )
}
#[cfg(all(
    feature = "wsi-xcb",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceXcbPresentationSupportKHR(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    connection: *mut vk::xcb_connection_t,
    visual_id: vk::xcb_visualid_t,
) -> vk::VkBool32 {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceXcbPresentationSupportKHR,
            c"vkGetPhysicalDeviceXcbPresentationSupportKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, queueFamilyIndex, connection, visual_id)
        },
    )
}
#[cfg(all(
    feature = "wsi-xlib",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetPhysicalDeviceXlibPresentationSupportKHR(
    physicalDevice: vk::VkPhysicalDevice,
    queueFamilyIndex: u32,
    dpy: *mut vk::Display,
    visualID: vk::VisualID,
) -> vk::VkBool32 {
    let command: Option<(
        vk::PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
        vk::VkPhysicalDevice,
    )> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetPhysicalDeviceXlibPresentationSupportKHR,
            c"vkGetPhysicalDeviceXlibPresentationSupportKHR",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            unsafe { core::mem::zeroed::<vk::VkBool32>() }
        },
        |(command, physicalDevice)| unsafe {
            command(physicalDevice, queueFamilyIndex, dpy, visualID)
        },
    )
}
#[cfg(all(
    feature = "wsi-xlib-xrandr",
    any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "hurd",
        target_os = "cygwin"
    )
))]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetRandROutputDisplayEXT(
    physicalDevice: vk::VkPhysicalDevice,
    dpy: *mut vk::Display,
    rrOutput: vk::RROutput,
    pDisplay: *mut vk::VkDisplayKHR,
) -> vk::VkResult {
    let command: Option<(vk::PFN_vkGetRandROutputDisplayEXT, vk::VkPhysicalDevice)> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetRandROutputDisplayEXT,
            c"vkGetRandROutputDisplayEXT",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, dpy, rrOutput, pDisplay) },
    )
}
#[cfg(target_os = "windows")]
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkGetWinrtDisplayNV(
    physicalDevice: vk::VkPhysicalDevice,
    deviceRelativeId: u32,
    pDisplay: *mut vk::VkDisplayKHR,
) -> vk::VkResult {
    let command: Option<(vk::PFN_vkGetWinrtDisplayNV, vk::VkPhysicalDevice)> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkGetWinrtDisplayNV,
            c"vkGetWinrtDisplayNV",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            fatal_loader_error(c"vkGetWinrtDisplayNV: Driver's function pointer was NULL")
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, deviceRelativeId, pDisplay) },
    )
}
/// Forwards a loader terminator command to the owning ICD.
///
/// # Safety
///
/// The physical device must be a live loader terminator handle and all other arguments must satisfy Vulkan's contracts.
pub(crate) unsafe extern "system" fn terminator_vkReleaseDisplayEXT(
    physicalDevice: vk::VkPhysicalDevice,
    display: vk::VkDisplayKHR,
) -> vk::VkResult {
    let command: Option<(vk::PFN_vkReleaseDisplayEXT, vk::VkPhysicalDevice)> = unsafe {
        resolve_physical_device(
            physicalDevice,
            |dispatch| dispatch.vkReleaseDisplayEXT,
            c"vkReleaseDisplayEXT",
        )
    };
    command.map_or_else(
        || {
            core::hint::cold_path();
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        },
        |(command, physicalDevice)| unsafe { command(physicalDevice, display) },
    )
}
