// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.

use super::commands::DEVICE_DISPATCH_LOADS;
use super::commands::DEVICE_DISPATCH_MASKS;
use super::commands::DeviceDispatchLoad;
use super::commands::IcdDeviceDispatchLoad;
use super::commands::LAYER_INSTANCE_DISPATCH_LOADS;
use super::commands::LayerInstanceDispatchLoad;
use super::commands::VK_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT_COMMAND_ID;
use super::commands::VK_CMD_END_DEBUG_UTILS_LABEL_EXT_COMMAND_ID;
use super::commands::VK_CMD_INSERT_DEBUG_UTILS_LABEL_EXT_COMMAND_ID;
use super::commands::VK_CREATE_SHARED_SWAPCHAINS_KHR_COMMAND_ID;
use super::commands::VK_CREATE_SWAPCHAIN_KHR_COMMAND_ID;
use super::commands::VK_DEBUG_MARKER_SET_OBJECT_NAME_EXT_COMMAND_ID;
use super::commands::VK_DEBUG_MARKER_SET_OBJECT_TAG_EXT_COMMAND_ID;
use super::commands::VK_DESTROY_DEVICE_COMMAND_ID;
use super::commands::VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR_COMMAND_ID;
#[cfg(target_os = "windows")]
use super::commands::VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES2EXT_COMMAND_ID;
use super::commands::VK_QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT_COMMAND_ID;
use super::commands::VK_QUEUE_END_DEBUG_UTILS_LABEL_EXT_COMMAND_ID;
use super::commands::VK_QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT_COMMAND_ID;
use super::commands::VK_SET_DEBUG_UTILS_OBJECT_NAME_EXT_COMMAND_ID;
use super::commands::VK_SET_DEBUG_UTILS_OBJECT_TAG_EXT_COMMAND_ID;
use crate::DEVICE_DISPATCH_MAGIC;
use crate::dispatch_offset;
#[repr(C)]
#[derive(Clone, Default)]
pub(crate) struct InstanceDispatchTable {
    pub(crate) vkAcquireDrmDisplayEXT: Option<vk::PFN_vkAcquireDrmDisplayEXT>,
    pub(crate) vkAcquireWinrtDisplayNV: Option<vk::PFN_vkAcquireWinrtDisplayNV>,
    pub(crate) vkAcquireXlibDisplayEXT: Option<vk::PFN_vkAcquireXlibDisplayEXT>,
    pub(crate) vkCreateAndroidSurfaceKHR: Option<vk::PFN_vkCreateAndroidSurfaceKHR>,
    pub(crate) vkCreateDebugReportCallbackEXT: Option<vk::PFN_vkCreateDebugReportCallbackEXT>,
    pub(crate) vkCreateDebugUtilsMessengerEXT: Option<vk::PFN_vkCreateDebugUtilsMessengerEXT>,
    pub(crate) vkCreateDevice: Option<vk::PFN_vkCreateDevice>,
    pub(crate) vkCreateDirectFBSurfaceEXT: Option<vk::PFN_vkCreateDirectFBSurfaceEXT>,
    pub(crate) vkCreateDisplayModeKHR: Option<vk::PFN_vkCreateDisplayModeKHR>,
    pub(crate) vkCreateDisplayPlaneSurfaceKHR: Option<vk::PFN_vkCreateDisplayPlaneSurfaceKHR>,
    pub(crate) vkCreateHeadlessSurfaceEXT: Option<vk::PFN_vkCreateHeadlessSurfaceEXT>,
    pub(crate) vkCreateIOSSurfaceMVK: Option<vk::PFN_vkCreateIOSSurfaceMVK>,
    pub(crate) vkCreateImagePipeSurfaceFUCHSIA: Option<vk::PFN_vkCreateImagePipeSurfaceFUCHSIA>,
    pub(crate) vkCreateMacOSSurfaceMVK: Option<vk::PFN_vkCreateMacOSSurfaceMVK>,
    pub(crate) vkCreateMetalSurfaceEXT: Option<vk::PFN_vkCreateMetalSurfaceEXT>,
    pub(crate) vkCreateScreenSurfaceQNX: Option<vk::PFN_vkCreateScreenSurfaceQNX>,
    pub(crate) vkCreateStreamDescriptorSurfaceGGP:
        Option<vk::PFN_vkCreateStreamDescriptorSurfaceGGP>,
    pub(crate) vkCreateSurfaceOHOS: Option<vk::PFN_vkCreateSurfaceOHOS>,
    pub(crate) vkCreateUbmSurfaceSEC: Option<vk::PFN_vkCreateUbmSurfaceSEC>,
    pub(crate) vkCreateViSurfaceNN: Option<vk::PFN_vkCreateViSurfaceNN>,
    pub(crate) vkCreateWaylandSurfaceKHR: Option<vk::PFN_vkCreateWaylandSurfaceKHR>,
    pub(crate) vkCreateWin32SurfaceKHR: Option<vk::PFN_vkCreateWin32SurfaceKHR>,
    pub(crate) vkCreateXcbSurfaceKHR: Option<vk::PFN_vkCreateXcbSurfaceKHR>,
    pub(crate) vkCreateXlibSurfaceKHR: Option<vk::PFN_vkCreateXlibSurfaceKHR>,
    pub(crate) vkDebugReportMessageEXT: Option<vk::PFN_vkDebugReportMessageEXT>,
    pub(crate) vkDestroyDebugReportCallbackEXT: Option<vk::PFN_vkDestroyDebugReportCallbackEXT>,
    pub(crate) vkDestroyDebugUtilsMessengerEXT: Option<vk::PFN_vkDestroyDebugUtilsMessengerEXT>,
    pub(crate) vkDestroyInstance: Option<vk::PFN_vkDestroyInstance>,
    pub(crate) vkDestroySurfaceKHR: Option<vk::PFN_vkDestroySurfaceKHR>,
    pub(crate) vkEnumerateDeviceExtensionProperties:
        Option<vk::PFN_vkEnumerateDeviceExtensionProperties>,
    pub(crate) vkEnumerateDeviceLayerProperties: Option<vk::PFN_vkEnumerateDeviceLayerProperties>,
    pub(crate) vkEnumeratePhysicalDeviceGroups: Option<vk::PFN_vkEnumeratePhysicalDeviceGroups>,
    pub(crate) vkEnumeratePhysicalDeviceGroupsKHR:
        Option<vk::PFN_vkEnumeratePhysicalDeviceGroupsKHR>,
    pub(crate) vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM:
        Option<vk::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM>,
    pub(crate) vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR:
        Option<vk::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR>,
    pub(crate) vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM:
        Option<vk::PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM>,
    pub(crate) vkEnumeratePhysicalDevices: Option<vk::PFN_vkEnumeratePhysicalDevices>,
    pub(crate) vkGetDeviceProcAddr: Option<vk::PFN_vkGetDeviceProcAddr>,
    pub(crate) vkGetDisplayModeProperties2KHR: Option<vk::PFN_vkGetDisplayModeProperties2KHR>,
    pub(crate) vkGetDisplayModePropertiesKHR: Option<vk::PFN_vkGetDisplayModePropertiesKHR>,
    pub(crate) vkGetDisplayPlaneCapabilities2KHR: Option<vk::PFN_vkGetDisplayPlaneCapabilities2KHR>,
    pub(crate) vkGetDisplayPlaneCapabilitiesKHR: Option<vk::PFN_vkGetDisplayPlaneCapabilitiesKHR>,
    pub(crate) vkGetDisplayPlaneSupportedDisplaysKHR:
        Option<vk::PFN_vkGetDisplayPlaneSupportedDisplaysKHR>,
    pub(crate) vkGetDrmDisplayEXT: Option<vk::PFN_vkGetDrmDisplayEXT>,
    pub(crate) vkGetPhysicalDeviceCalibrateableTimeDomainsEXT:
        Option<vk::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT>,
    pub(crate) vkGetPhysicalDeviceCalibrateableTimeDomainsKHR:
        Option<vk::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR>,
    pub(crate) vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV:
        Option<vk::PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV>,
    pub(crate) vkGetPhysicalDeviceCooperativeMatrixProperties2EXT:
        Option<vk::PFN_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT>,
    pub(crate) vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceCooperativeMatrixPropertiesNV:
        Option<vk::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV>,
    pub(crate) vkGetPhysicalDeviceCooperativeVectorPropertiesNV:
        Option<vk::PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV>,
    pub(crate) vkGetPhysicalDeviceDescriptorSizeEXT:
        Option<vk::PFN_vkGetPhysicalDeviceDescriptorSizeEXT>,
    pub(crate) vkGetPhysicalDeviceDirectFBPresentationSupportEXT:
        Option<vk::PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT>,
    pub(crate) vkGetPhysicalDeviceDisplayPlaneProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceDisplayPlanePropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceDisplayProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceDisplayProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceDisplayPropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceDisplayPropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceExternalBufferProperties:
        Option<vk::PFN_vkGetPhysicalDeviceExternalBufferProperties>,
    pub(crate) vkGetPhysicalDeviceExternalBufferPropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceExternalFenceProperties:
        Option<vk::PFN_vkGetPhysicalDeviceExternalFenceProperties>,
    pub(crate) vkGetPhysicalDeviceExternalFencePropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceExternalImageFormatPropertiesNV:
        Option<vk::PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV>,
    pub(crate) vkGetPhysicalDeviceExternalSemaphoreProperties:
        Option<vk::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties>,
    pub(crate) vkGetPhysicalDeviceExternalSemaphorePropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceExternalTensorPropertiesARM:
        Option<vk::PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM>,
    pub(crate) vkGetPhysicalDeviceFeatures: Option<vk::PFN_vkGetPhysicalDeviceFeatures>,
    pub(crate) vkGetPhysicalDeviceFeatures2: Option<vk::PFN_vkGetPhysicalDeviceFeatures2>,
    pub(crate) vkGetPhysicalDeviceFeatures2KHR: Option<vk::PFN_vkGetPhysicalDeviceFeatures2KHR>,
    pub(crate) vkGetPhysicalDeviceFormatProperties:
        Option<vk::PFN_vkGetPhysicalDeviceFormatProperties>,
    pub(crate) vkGetPhysicalDeviceFormatProperties2:
        Option<vk::PFN_vkGetPhysicalDeviceFormatProperties2>,
    pub(crate) vkGetPhysicalDeviceFormatProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceFormatProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceFragmentShadingRatesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR>,
    pub(crate) vkGetPhysicalDeviceImageFormatProperties:
        Option<vk::PFN_vkGetPhysicalDeviceImageFormatProperties>,
    pub(crate) vkGetPhysicalDeviceImageFormatProperties2:
        Option<vk::PFN_vkGetPhysicalDeviceImageFormatProperties2>,
    pub(crate) vkGetPhysicalDeviceImageFormatProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceImageFormatProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceMemoryProperties:
        Option<vk::PFN_vkGetPhysicalDeviceMemoryProperties>,
    pub(crate) vkGetPhysicalDeviceMemoryProperties2:
        Option<vk::PFN_vkGetPhysicalDeviceMemoryProperties2>,
    pub(crate) vkGetPhysicalDeviceMemoryProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceMemoryProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceMultisamplePropertiesEXT:
        Option<vk::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT>,
    pub(crate) vkGetPhysicalDeviceOpticalFlowImageFormatsNV:
        Option<vk::PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV>,
    pub(crate) vkGetPhysicalDevicePresentRectanglesKHR:
        Option<vk::PFN_vkGetPhysicalDevicePresentRectanglesKHR>,
    pub(crate) vkGetPhysicalDeviceProperties: Option<vk::PFN_vkGetPhysicalDeviceProperties>,
    pub(crate) vkGetPhysicalDeviceProperties2: Option<vk::PFN_vkGetPhysicalDeviceProperties2>,
    pub(crate) vkGetPhysicalDeviceProperties2KHR: Option<vk::PFN_vkGetPhysicalDeviceProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyProperties:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyProperties2:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceScreenPresentationSupportQNX:
        Option<vk::PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX>,
    pub(crate) vkGetPhysicalDeviceSparseImageFormatProperties:
        Option<vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties>,
    pub(crate) vkGetPhysicalDeviceSparseImageFormatProperties2:
        Option<vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2>,
    pub(crate) vkGetPhysicalDeviceSparseImageFormatProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV:
        Option<vk::PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV>,
    pub(crate) vkGetPhysicalDeviceSurfaceCapabilities2EXT:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT>,
    pub(crate) vkGetPhysicalDeviceSurfaceCapabilities2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR>,
    pub(crate) vkGetPhysicalDeviceSurfaceCapabilitiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>,
    pub(crate) vkGetPhysicalDeviceSurfaceFormats2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceFormats2KHR>,
    pub(crate) vkGetPhysicalDeviceSurfaceFormatsKHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>,
    pub(crate) vkGetPhysicalDeviceSurfacePresentModes2EXT:
        Option<vk::PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT>,
    pub(crate) vkGetPhysicalDeviceSurfacePresentModesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>,
    pub(crate) vkGetPhysicalDeviceSurfaceSupportKHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceSupportKHR>,
    pub(crate) vkGetPhysicalDeviceToolProperties: Option<vk::PFN_vkGetPhysicalDeviceToolProperties>,
    pub(crate) vkGetPhysicalDeviceToolPropertiesEXT:
        Option<vk::PFN_vkGetPhysicalDeviceToolPropertiesEXT>,
    pub(crate) vkGetPhysicalDeviceUbmPresentationSupportSEC:
        Option<vk::PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC>,
    pub(crate) vkGetPhysicalDeviceVideoCapabilitiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR>,
    pub(crate) vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceVideoFormatPropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceWaylandPresentationSupportKHR:
        Option<vk::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR>,
    pub(crate) vkGetPhysicalDeviceWin32PresentationSupportKHR:
        Option<vk::PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR>,
    pub(crate) vkGetPhysicalDeviceXcbPresentationSupportKHR:
        Option<vk::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR>,
    pub(crate) vkGetPhysicalDeviceXlibPresentationSupportKHR:
        Option<vk::PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR>,
    pub(crate) vkGetRandROutputDisplayEXT: Option<vk::PFN_vkGetRandROutputDisplayEXT>,
    pub(crate) vkGetWinrtDisplayNV: Option<vk::PFN_vkGetWinrtDisplayNV>,
    pub(crate) vkReleaseDisplayEXT: Option<vk::PFN_vkReleaseDisplayEXT>,
    pub(crate) vkSubmitDebugUtilsMessageEXT: Option<vk::PFN_vkSubmitDebugUtilsMessageEXT>,
}
const _: () = assert!(core::mem::size_of::<InstanceDispatchTable>() <= 65_535);
pub(super) static ICD_INSTANCE_DISPATCH_LOADS: &[LayerInstanceDispatchLoad] = &[
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkAcquireDrmDisplayEXT
        )),
        name: c"vkAcquireDrmDisplayEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkAcquireWinrtDisplayNV
        )),
        name: c"vkAcquireWinrtDisplayNV",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkAcquireXlibDisplayEXT
        )),
        name: c"vkAcquireXlibDisplayEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateAndroidSurfaceKHR
        )),
        name: c"vkCreateAndroidSurfaceKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateDebugReportCallbackEXT
        )),
        name: c"vkCreateDebugReportCallbackEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateDebugUtilsMessengerEXT
        )),
        name: c"vkCreateDebugUtilsMessengerEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(InstanceDispatchTable, vkCreateDevice)),
        name: c"vkCreateDevice",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateDirectFBSurfaceEXT
        )),
        name: c"vkCreateDirectFBSurfaceEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateDisplayModeKHR
        )),
        name: c"vkCreateDisplayModeKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateDisplayPlaneSurfaceKHR
        )),
        name: c"vkCreateDisplayPlaneSurfaceKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateHeadlessSurfaceEXT
        )),
        name: c"vkCreateHeadlessSurfaceEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateIOSSurfaceMVK
        )),
        name: c"vkCreateIOSSurfaceMVK",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateImagePipeSurfaceFUCHSIA
        )),
        name: c"vkCreateImagePipeSurfaceFUCHSIA",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateMacOSSurfaceMVK
        )),
        name: c"vkCreateMacOSSurfaceMVK",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateMetalSurfaceEXT
        )),
        name: c"vkCreateMetalSurfaceEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateScreenSurfaceQNX
        )),
        name: c"vkCreateScreenSurfaceQNX",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateStreamDescriptorSurfaceGGP
        )),
        name: c"vkCreateStreamDescriptorSurfaceGGP",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateSurfaceOHOS
        )),
        name: c"vkCreateSurfaceOHOS",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateUbmSurfaceSEC
        )),
        name: c"vkCreateUbmSurfaceSEC",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateViSurfaceNN
        )),
        name: c"vkCreateViSurfaceNN",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateWaylandSurfaceKHR
        )),
        name: c"vkCreateWaylandSurfaceKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateWin32SurfaceKHR
        )),
        name: c"vkCreateWin32SurfaceKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateXcbSurfaceKHR
        )),
        name: c"vkCreateXcbSurfaceKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkCreateXlibSurfaceKHR
        )),
        name: c"vkCreateXlibSurfaceKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkDebugReportMessageEXT
        )),
        name: c"vkDebugReportMessageEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkDestroyDebugReportCallbackEXT
        )),
        name: c"vkDestroyDebugReportCallbackEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkDestroyDebugUtilsMessengerEXT
        )),
        name: c"vkDestroyDebugUtilsMessengerEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkDestroyInstance
        )),
        name: c"vkDestroyInstance",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkDestroySurfaceKHR
        )),
        name: c"vkDestroySurfaceKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkEnumerateDeviceExtensionProperties
        )),
        name: c"vkEnumerateDeviceExtensionProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkEnumerateDeviceLayerProperties
        )),
        name: c"vkEnumerateDeviceLayerProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkEnumeratePhysicalDeviceGroups
        )),
        name: c"vkEnumeratePhysicalDeviceGroups",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkEnumeratePhysicalDeviceGroupsKHR
        )),
        name: c"vkEnumeratePhysicalDeviceGroupsKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM
        )),
        name: c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR
        )),
        name: c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM
        )),
        name: c"vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkEnumeratePhysicalDevices
        )),
        name: c"vkEnumeratePhysicalDevices",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetDeviceProcAddr
        )),
        name: c"vkGetDeviceProcAddr",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetDisplayModeProperties2KHR
        )),
        name: c"vkGetDisplayModeProperties2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetDisplayModePropertiesKHR
        )),
        name: c"vkGetDisplayModePropertiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetDisplayPlaneCapabilities2KHR
        )),
        name: c"vkGetDisplayPlaneCapabilities2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetDisplayPlaneCapabilitiesKHR
        )),
        name: c"vkGetDisplayPlaneCapabilitiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetDisplayPlaneSupportedDisplaysKHR
        )),
        name: c"vkGetDisplayPlaneSupportedDisplaysKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetDrmDisplayEXT
        )),
        name: c"vkGetDrmDisplayEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceCalibrateableTimeDomainsEXT
        )),
        name: c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceCalibrateableTimeDomainsKHR
        )),
        name: c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV
        )),
        name: c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceCooperativeMatrixProperties2EXT
        )),
        name: c"vkGetPhysicalDeviceCooperativeMatrixProperties2EXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR
        )),
        name: c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceCooperativeMatrixPropertiesNV
        )),
        name: c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceCooperativeVectorPropertiesNV
        )),
        name: c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceDescriptorSizeEXT
        )),
        name: c"vkGetPhysicalDeviceDescriptorSizeEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceDirectFBPresentationSupportEXT
        )),
        name: c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceDisplayPlaneProperties2KHR
        )),
        name: c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceDisplayPlanePropertiesKHR
        )),
        name: c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceDisplayProperties2KHR
        )),
        name: c"vkGetPhysicalDeviceDisplayProperties2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceDisplayPropertiesKHR
        )),
        name: c"vkGetPhysicalDeviceDisplayPropertiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceExternalBufferProperties
        )),
        name: c"vkGetPhysicalDeviceExternalBufferProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceExternalBufferPropertiesKHR
        )),
        name: c"vkGetPhysicalDeviceExternalBufferPropertiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceExternalFenceProperties
        )),
        name: c"vkGetPhysicalDeviceExternalFenceProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceExternalFencePropertiesKHR
        )),
        name: c"vkGetPhysicalDeviceExternalFencePropertiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceExternalImageFormatPropertiesNV
        )),
        name: c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceExternalSemaphoreProperties
        )),
        name: c"vkGetPhysicalDeviceExternalSemaphoreProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceExternalSemaphorePropertiesKHR
        )),
        name: c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceExternalTensorPropertiesARM
        )),
        name: c"vkGetPhysicalDeviceExternalTensorPropertiesARM",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceFeatures
        )),
        name: c"vkGetPhysicalDeviceFeatures",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceFeatures2
        )),
        name: c"vkGetPhysicalDeviceFeatures2",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceFeatures2KHR
        )),
        name: c"vkGetPhysicalDeviceFeatures2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceFormatProperties
        )),
        name: c"vkGetPhysicalDeviceFormatProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceFormatProperties2
        )),
        name: c"vkGetPhysicalDeviceFormatProperties2",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceFormatProperties2KHR
        )),
        name: c"vkGetPhysicalDeviceFormatProperties2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceFragmentShadingRatesKHR
        )),
        name: c"vkGetPhysicalDeviceFragmentShadingRatesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceImageFormatProperties
        )),
        name: c"vkGetPhysicalDeviceImageFormatProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceImageFormatProperties2
        )),
        name: c"vkGetPhysicalDeviceImageFormatProperties2",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceImageFormatProperties2KHR
        )),
        name: c"vkGetPhysicalDeviceImageFormatProperties2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceMemoryProperties
        )),
        name: c"vkGetPhysicalDeviceMemoryProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceMemoryProperties2
        )),
        name: c"vkGetPhysicalDeviceMemoryProperties2",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceMemoryProperties2KHR
        )),
        name: c"vkGetPhysicalDeviceMemoryProperties2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceMultisamplePropertiesEXT
        )),
        name: c"vkGetPhysicalDeviceMultisamplePropertiesEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceOpticalFlowImageFormatsNV
        )),
        name: c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDevicePresentRectanglesKHR
        )),
        name: c"vkGetPhysicalDevicePresentRectanglesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceProperties
        )),
        name: c"vkGetPhysicalDeviceProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceProperties2
        )),
        name: c"vkGetPhysicalDeviceProperties2",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceProperties2KHR
        )),
        name: c"vkGetPhysicalDeviceProperties2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM
        )),
        name: c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM
        )),
        name: c"vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM
        )),
        name: c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM
        )),
        name: c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR
        )),
        name: c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceQueueFamilyProperties
        )),
        name: c"vkGetPhysicalDeviceQueueFamilyProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceQueueFamilyProperties2
        )),
        name: c"vkGetPhysicalDeviceQueueFamilyProperties2",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceQueueFamilyProperties2KHR
        )),
        name: c"vkGetPhysicalDeviceQueueFamilyProperties2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceScreenPresentationSupportQNX
        )),
        name: c"vkGetPhysicalDeviceScreenPresentationSupportQNX",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSparseImageFormatProperties
        )),
        name: c"vkGetPhysicalDeviceSparseImageFormatProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSparseImageFormatProperties2
        )),
        name: c"vkGetPhysicalDeviceSparseImageFormatProperties2",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSparseImageFormatProperties2KHR
        )),
        name: c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV
        )),
        name: c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSurfaceCapabilities2EXT
        )),
        name: c"vkGetPhysicalDeviceSurfaceCapabilities2EXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSurfaceCapabilities2KHR
        )),
        name: c"vkGetPhysicalDeviceSurfaceCapabilities2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSurfaceCapabilitiesKHR
        )),
        name: c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSurfaceFormats2KHR
        )),
        name: c"vkGetPhysicalDeviceSurfaceFormats2KHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSurfaceFormatsKHR
        )),
        name: c"vkGetPhysicalDeviceSurfaceFormatsKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSurfacePresentModes2EXT
        )),
        name: c"vkGetPhysicalDeviceSurfacePresentModes2EXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSurfacePresentModesKHR
        )),
        name: c"vkGetPhysicalDeviceSurfacePresentModesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceSurfaceSupportKHR
        )),
        name: c"vkGetPhysicalDeviceSurfaceSupportKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceToolProperties
        )),
        name: c"vkGetPhysicalDeviceToolProperties",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceToolPropertiesEXT
        )),
        name: c"vkGetPhysicalDeviceToolPropertiesEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceUbmPresentationSupportSEC
        )),
        name: c"vkGetPhysicalDeviceUbmPresentationSupportSEC",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceVideoCapabilitiesKHR
        )),
        name: c"vkGetPhysicalDeviceVideoCapabilitiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR
        )),
        name: c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceVideoFormatPropertiesKHR
        )),
        name: c"vkGetPhysicalDeviceVideoFormatPropertiesKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceWaylandPresentationSupportKHR
        )),
        name: c"vkGetPhysicalDeviceWaylandPresentationSupportKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceWin32PresentationSupportKHR
        )),
        name: c"vkGetPhysicalDeviceWin32PresentationSupportKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceXcbPresentationSupportKHR
        )),
        name: c"vkGetPhysicalDeviceXcbPresentationSupportKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetPhysicalDeviceXlibPresentationSupportKHR
        )),
        name: c"vkGetPhysicalDeviceXlibPresentationSupportKHR",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetRandROutputDisplayEXT
        )),
        name: c"vkGetRandROutputDisplayEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkGetWinrtDisplayNV
        )),
        name: c"vkGetWinrtDisplayNV",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkReleaseDisplayEXT
        )),
        name: c"vkReleaseDisplayEXT",
    },
    LayerInstanceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            InstanceDispatchTable,
            vkSubmitDebugUtilsMessageEXT
        )),
        name: c"vkSubmitDebugUtilsMessageEXT",
    },
];
impl InstanceDispatchTable {
    pub(crate) unsafe fn load_into(
        table: *mut Self,
        gipa: vk::PFN_vkGetInstanceProcAddr,
        handle: vk::VkInstance,
    ) {
        unsafe {
            load_instance_dispatch_fields(table.cast(), gipa, handle, ICD_INSTANCE_DISPATCH_LOADS);
        }
    }
}
impl InstanceDispatchTable {
    pub(crate) const fn has_required_core_1_0(&self) -> bool {
        self.vkCreateDevice.is_some()
            && self.vkDestroyInstance.is_some()
            && self.vkEnumerateDeviceExtensionProperties.is_some()
            && self.vkEnumeratePhysicalDevices.is_some()
            && self.vkGetDeviceProcAddr.is_some()
            && self.vkGetPhysicalDeviceFeatures.is_some()
            && self.vkGetPhysicalDeviceFormatProperties.is_some()
            && self.vkGetPhysicalDeviceImageFormatProperties.is_some()
            && self.vkGetPhysicalDeviceMemoryProperties.is_some()
            && self.vkGetPhysicalDeviceProperties.is_some()
            && self.vkGetPhysicalDeviceQueueFamilyProperties.is_some()
            && self
                .vkGetPhysicalDeviceSparseImageFormatProperties
                .is_some()
    }
}
#[repr(C)]
pub(crate) struct LayerInstanceDispatchTable {
    pub(crate) vk_layerGetPhysicalDeviceProcAddr: crate::layer::GetPhysicalDeviceProcAddr,
    pub(crate) vkCreateInstance: Option<vk::PFN_vkCreateInstance>,
    pub(crate) vkDestroyInstance: Option<vk::PFN_vkDestroyInstance>,
    pub(crate) vkEnumeratePhysicalDevices: Option<vk::PFN_vkEnumeratePhysicalDevices>,
    pub(crate) vkGetPhysicalDeviceFeatures: Option<vk::PFN_vkGetPhysicalDeviceFeatures>,
    pub(crate) vkGetPhysicalDeviceFormatProperties:
        Option<vk::PFN_vkGetPhysicalDeviceFormatProperties>,
    pub(crate) vkGetPhysicalDeviceImageFormatProperties:
        Option<vk::PFN_vkGetPhysicalDeviceImageFormatProperties>,
    pub(crate) vkGetPhysicalDeviceProperties: Option<vk::PFN_vkGetPhysicalDeviceProperties>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyProperties:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties>,
    pub(crate) vkGetPhysicalDeviceMemoryProperties:
        Option<vk::PFN_vkGetPhysicalDeviceMemoryProperties>,
    pub(crate) vkGetInstanceProcAddr: Option<vk::PFN_vkGetInstanceProcAddr>,
    pub(crate) vkCreateDevice: Option<vk::PFN_vkCreateDevice>,
    pub(crate) vkEnumerateInstanceExtensionProperties:
        Option<vk::PFN_vkEnumerateInstanceExtensionProperties>,
    pub(crate) vkEnumerateDeviceExtensionProperties:
        Option<vk::PFN_vkEnumerateDeviceExtensionProperties>,
    pub(crate) vkEnumerateInstanceLayerProperties:
        Option<vk::PFN_vkEnumerateInstanceLayerProperties>,
    pub(crate) vkEnumerateDeviceLayerProperties: Option<vk::PFN_vkEnumerateDeviceLayerProperties>,
    pub(crate) vkGetPhysicalDeviceSparseImageFormatProperties:
        Option<vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties>,
    pub(crate) vkEnumerateInstanceVersion: Option<vk::PFN_vkEnumerateInstanceVersion>,
    pub(crate) vkEnumeratePhysicalDeviceGroups: Option<vk::PFN_vkEnumeratePhysicalDeviceGroups>,
    pub(crate) vkGetPhysicalDeviceFeatures2: Option<vk::PFN_vkGetPhysicalDeviceFeatures2>,
    pub(crate) vkGetPhysicalDeviceProperties2: Option<vk::PFN_vkGetPhysicalDeviceProperties2>,
    pub(crate) vkGetPhysicalDeviceFormatProperties2:
        Option<vk::PFN_vkGetPhysicalDeviceFormatProperties2>,
    pub(crate) vkGetPhysicalDeviceImageFormatProperties2:
        Option<vk::PFN_vkGetPhysicalDeviceImageFormatProperties2>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyProperties2:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2>,
    pub(crate) vkGetPhysicalDeviceMemoryProperties2:
        Option<vk::PFN_vkGetPhysicalDeviceMemoryProperties2>,
    pub(crate) vkGetPhysicalDeviceSparseImageFormatProperties2:
        Option<vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2>,
    pub(crate) vkGetPhysicalDeviceExternalBufferProperties:
        Option<vk::PFN_vkGetPhysicalDeviceExternalBufferProperties>,
    pub(crate) vkGetPhysicalDeviceExternalFenceProperties:
        Option<vk::PFN_vkGetPhysicalDeviceExternalFenceProperties>,
    pub(crate) vkGetPhysicalDeviceExternalSemaphoreProperties:
        Option<vk::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties>,
    pub(crate) vkGetPhysicalDeviceToolProperties: Option<vk::PFN_vkGetPhysicalDeviceToolProperties>,
    pub(crate) vkDestroySurfaceKHR: Option<vk::PFN_vkDestroySurfaceKHR>,
    pub(crate) vkGetPhysicalDeviceSurfaceSupportKHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceSupportKHR>,
    pub(crate) vkGetPhysicalDeviceSurfaceCapabilitiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>,
    pub(crate) vkGetPhysicalDeviceSurfaceFormatsKHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>,
    pub(crate) vkGetPhysicalDeviceSurfacePresentModesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>,
    pub(crate) vkGetPhysicalDevicePresentRectanglesKHR:
        Option<vk::PFN_vkGetPhysicalDevicePresentRectanglesKHR>,
    pub(crate) vkGetPhysicalDeviceDisplayPropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceDisplayPropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceDisplayPlanePropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR>,
    pub(crate) vkGetDisplayPlaneSupportedDisplaysKHR:
        Option<vk::PFN_vkGetDisplayPlaneSupportedDisplaysKHR>,
    pub(crate) vkGetDisplayModePropertiesKHR: Option<vk::PFN_vkGetDisplayModePropertiesKHR>,
    pub(crate) vkCreateDisplayModeKHR: Option<vk::PFN_vkCreateDisplayModeKHR>,
    pub(crate) vkGetDisplayPlaneCapabilitiesKHR: Option<vk::PFN_vkGetDisplayPlaneCapabilitiesKHR>,
    pub(crate) vkCreateDisplayPlaneSurfaceKHR: Option<vk::PFN_vkCreateDisplayPlaneSurfaceKHR>,
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
    pub(crate) vkCreateXlibSurfaceKHR: Option<vk::PFN_vkCreateXlibSurfaceKHR>,
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
    pub(crate) vkGetPhysicalDeviceXlibPresentationSupportKHR:
        Option<vk::PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR>,
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
    pub(crate) vkCreateXcbSurfaceKHR: Option<vk::PFN_vkCreateXcbSurfaceKHR>,
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
    pub(crate) vkGetPhysicalDeviceXcbPresentationSupportKHR:
        Option<vk::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR>,
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
    pub(crate) vkCreateWaylandSurfaceKHR: Option<vk::PFN_vkCreateWaylandSurfaceKHR>,
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
    pub(crate) vkGetPhysicalDeviceWaylandPresentationSupportKHR:
        Option<vk::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR>,
    #[cfg(target_os = "android")]
    pub(crate) vkCreateAndroidSurfaceKHR: Option<vk::PFN_vkCreateAndroidSurfaceKHR>,
    #[cfg(target_os = "windows")]
    pub(crate) vkCreateWin32SurfaceKHR: Option<vk::PFN_vkCreateWin32SurfaceKHR>,
    #[cfg(target_os = "windows")]
    pub(crate) vkGetPhysicalDeviceWin32PresentationSupportKHR:
        Option<vk::PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR>,
    pub(crate) vkGetPhysicalDeviceVideoCapabilitiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR>,
    pub(crate) vkGetPhysicalDeviceVideoFormatPropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceFeatures2KHR: Option<vk::PFN_vkGetPhysicalDeviceFeatures2KHR>,
    pub(crate) vkGetPhysicalDeviceProperties2KHR: Option<vk::PFN_vkGetPhysicalDeviceProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceFormatProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceFormatProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceImageFormatProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceImageFormatProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceMemoryProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceMemoryProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceSparseImageFormatProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR>,
    pub(crate) vkEnumeratePhysicalDeviceGroupsKHR:
        Option<vk::PFN_vkEnumeratePhysicalDeviceGroupsKHR>,
    pub(crate) vkGetPhysicalDeviceExternalBufferPropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceExternalSemaphorePropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceExternalFencePropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR>,
    pub(crate) vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR:
        Option<vk::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR>,
    pub(crate) vkGetPhysicalDeviceSurfaceCapabilities2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR>,
    pub(crate) vkGetPhysicalDeviceSurfaceFormats2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceFormats2KHR>,
    pub(crate) vkGetPhysicalDeviceDisplayProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceDisplayProperties2KHR>,
    pub(crate) vkGetPhysicalDeviceDisplayPlaneProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR>,
    pub(crate) vkGetDisplayModeProperties2KHR: Option<vk::PFN_vkGetDisplayModeProperties2KHR>,
    pub(crate) vkGetDisplayPlaneCapabilities2KHR: Option<vk::PFN_vkGetDisplayPlaneCapabilities2KHR>,
    pub(crate) vkGetPhysicalDeviceFragmentShadingRatesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR>,
    pub(crate) vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR:
        Option<vk::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR>,
    pub(crate) vkGetPhysicalDeviceCalibrateableTimeDomainsKHR:
        Option<vk::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR>,
    pub(crate) vkCreateDebugReportCallbackEXT: Option<vk::PFN_vkCreateDebugReportCallbackEXT>,
    pub(crate) vkDestroyDebugReportCallbackEXT: Option<vk::PFN_vkDestroyDebugReportCallbackEXT>,
    pub(crate) vkDebugReportMessageEXT: Option<vk::PFN_vkDebugReportMessageEXT>,
    #[cfg(feature = "platform-ggp")]
    pub(crate) vkCreateStreamDescriptorSurfaceGGP:
        Option<vk::PFN_vkCreateStreamDescriptorSurfaceGGP>,
    pub(crate) vkGetPhysicalDeviceExternalImageFormatPropertiesNV:
        Option<vk::PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV>,
    #[cfg(feature = "platform-vi")]
    pub(crate) vkCreateViSurfaceNN: Option<vk::PFN_vkCreateViSurfaceNN>,
    pub(crate) vkReleaseDisplayEXT: Option<vk::PFN_vkReleaseDisplayEXT>,
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
    pub(crate) vkAcquireXlibDisplayEXT: Option<vk::PFN_vkAcquireXlibDisplayEXT>,
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
    pub(crate) vkGetRandROutputDisplayEXT: Option<vk::PFN_vkGetRandROutputDisplayEXT>,
    pub(crate) vkGetPhysicalDeviceSurfaceCapabilities2EXT:
        Option<vk::PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT>,
    #[cfg(target_os = "ios")]
    pub(crate) vkCreateIOSSurfaceMVK: Option<vk::PFN_vkCreateIOSSurfaceMVK>,
    #[cfg(target_os = "macos")]
    pub(crate) vkCreateMacOSSurfaceMVK: Option<vk::PFN_vkCreateMacOSSurfaceMVK>,
    pub(crate) vkCreateDebugUtilsMessengerEXT: Option<vk::PFN_vkCreateDebugUtilsMessengerEXT>,
    pub(crate) vkDestroyDebugUtilsMessengerEXT: Option<vk::PFN_vkDestroyDebugUtilsMessengerEXT>,
    pub(crate) vkSubmitDebugUtilsMessageEXT: Option<vk::PFN_vkSubmitDebugUtilsMessageEXT>,
    pub(crate) vkGetPhysicalDeviceDescriptorSizeEXT:
        Option<vk::PFN_vkGetPhysicalDeviceDescriptorSizeEXT>,
    pub(crate) vkGetPhysicalDeviceMultisamplePropertiesEXT:
        Option<vk::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT>,
    pub(crate) vkGetPhysicalDeviceCalibrateableTimeDomainsEXT:
        Option<vk::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT>,
    #[cfg(target_os = "fuchsia")]
    pub(crate) vkCreateImagePipeSurfaceFUCHSIA: Option<vk::PFN_vkCreateImagePipeSurfaceFUCHSIA>,
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "visionos"
    ))]
    pub(crate) vkCreateMetalSurfaceEXT: Option<vk::PFN_vkCreateMetalSurfaceEXT>,
    pub(crate) vkGetPhysicalDeviceToolPropertiesEXT:
        Option<vk::PFN_vkGetPhysicalDeviceToolPropertiesEXT>,
    pub(crate) vkGetPhysicalDeviceCooperativeMatrixPropertiesNV:
        Option<vk::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV>,
    pub(crate) vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV:
        Option<vk::PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV>,
    #[cfg(target_os = "windows")]
    pub(crate) vkGetPhysicalDeviceSurfacePresentModes2EXT:
        Option<vk::PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT>,
    pub(crate) vkCreateHeadlessSurfaceEXT: Option<vk::PFN_vkCreateHeadlessSurfaceEXT>,
    pub(crate) vkAcquireDrmDisplayEXT: Option<vk::PFN_vkAcquireDrmDisplayEXT>,
    pub(crate) vkGetDrmDisplayEXT: Option<vk::PFN_vkGetDrmDisplayEXT>,
    #[cfg(target_os = "windows")]
    pub(crate) vkAcquireWinrtDisplayNV: Option<vk::PFN_vkAcquireWinrtDisplayNV>,
    #[cfg(target_os = "windows")]
    pub(crate) vkGetWinrtDisplayNV: Option<vk::PFN_vkGetWinrtDisplayNV>,
    #[cfg(feature = "wsi-directfb")]
    pub(crate) vkCreateDirectFBSurfaceEXT: Option<vk::PFN_vkCreateDirectFBSurfaceEXT>,
    #[cfg(feature = "wsi-directfb")]
    pub(crate) vkGetPhysicalDeviceDirectFBPresentationSupportEXT:
        Option<vk::PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT>,
    #[cfg(any(target_os = "nto", target_os = "qnx"))]
    pub(crate) vkCreateScreenSurfaceQNX: Option<vk::PFN_vkCreateScreenSurfaceQNX>,
    #[cfg(any(target_os = "nto", target_os = "qnx"))]
    pub(crate) vkGetPhysicalDeviceScreenPresentationSupportQNX:
        Option<vk::PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX>,
    pub(crate) vkGetPhysicalDeviceExternalTensorPropertiesARM:
        Option<vk::PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM>,
    pub(crate) vkGetPhysicalDeviceOpticalFlowImageFormatsNV:
        Option<vk::PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV>,
    pub(crate) vkGetPhysicalDeviceCooperativeVectorPropertiesNV:
        Option<vk::PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM>,
    #[cfg(target_env = "ohos")]
    pub(crate) vkCreateSurfaceOHOS: Option<vk::PFN_vkCreateSurfaceOHOS>,
    pub(crate) vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV:
        Option<vk::PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV>,
    pub(crate) vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM:
        Option<vk::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM>,
    pub(crate) vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM:
        Option<vk::PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM>,
    pub(crate) vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM:
        Option<vk::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM>,
    pub(crate) vkGetPhysicalDeviceCooperativeMatrixProperties2EXT:
        Option<vk::PFN_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT>,
    #[cfg(feature = "platform-ubm")]
    pub(crate) vkCreateUbmSurfaceSEC: Option<vk::PFN_vkCreateUbmSurfaceSEC>,
    #[cfg(feature = "platform-ubm")]
    pub(crate) vkGetPhysicalDeviceUbmPresentationSupportSEC:
        Option<vk::PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC>,
}
#[inline(never)]
pub(super) unsafe fn load_instance_dispatch_fields(
    table: *mut u8,
    gipa: vk::PFN_vkGetInstanceProcAddr,
    instance: vk::VkInstance,
    loads: &[LayerInstanceDispatchLoad],
) {
    for load in loads {
        let function = unsafe { gipa(instance, load.name.as_ptr()) };
        unsafe {
            table
                .add(usize::from(load.offset))
                .cast::<vk::PFN_vkVoidFunction>()
                .write(function);
        }
    }
}
impl LayerInstanceDispatchTable {
    pub(crate) unsafe fn load_into(
        table_ptr: *mut Self,
        gipa: vk::PFN_vkGetInstanceProcAddr,
        gpdpa: crate::layer::GetPhysicalDeviceProcAddr,
        instance: vk::VkInstance,
    ) {
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vk_layerGetPhysicalDeviceProcAddr).write(gpdpa);
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetInstanceProcAddr).write(Some(gipa));
        }
        unsafe {
            load_instance_dispatch_fields(
                table_ptr.cast(),
                gipa,
                instance,
                LAYER_INSTANCE_DISPATCH_LOADS,
            );
        }
    }
}
#[repr(C)]
pub(crate) struct LayerDeviceDispatchTable {
    pub(crate) magic: u64,
    pub(crate) vkGetDeviceProcAddr: Option<vk::PFN_vkGetDeviceProcAddr>,
    pub(crate) vkDestroyDevice: Option<vk::PFN_vkDestroyDevice>,
    pub(crate) vkGetDeviceQueue: Option<vk::PFN_vkGetDeviceQueue>,
    pub(crate) vkQueueSubmit: Option<vk::PFN_vkQueueSubmit>,
    pub(crate) vkQueueWaitIdle: Option<vk::PFN_vkQueueWaitIdle>,
    pub(crate) vkDeviceWaitIdle: Option<vk::PFN_vkDeviceWaitIdle>,
    pub(crate) vkAllocateMemory: Option<vk::PFN_vkAllocateMemory>,
    pub(crate) vkFreeMemory: Option<vk::PFN_vkFreeMemory>,
    pub(crate) vkMapMemory: Option<vk::PFN_vkMapMemory>,
    pub(crate) vkUnmapMemory: Option<vk::PFN_vkUnmapMemory>,
    pub(crate) vkFlushMappedMemoryRanges: Option<vk::PFN_vkFlushMappedMemoryRanges>,
    pub(crate) vkInvalidateMappedMemoryRanges: Option<vk::PFN_vkInvalidateMappedMemoryRanges>,
    pub(crate) vkGetDeviceMemoryCommitment: Option<vk::PFN_vkGetDeviceMemoryCommitment>,
    pub(crate) vkBindBufferMemory: Option<vk::PFN_vkBindBufferMemory>,
    pub(crate) vkBindImageMemory: Option<vk::PFN_vkBindImageMemory>,
    pub(crate) vkGetBufferMemoryRequirements: Option<vk::PFN_vkGetBufferMemoryRequirements>,
    pub(crate) vkGetImageMemoryRequirements: Option<vk::PFN_vkGetImageMemoryRequirements>,
    pub(crate) vkGetImageSparseMemoryRequirements:
        Option<vk::PFN_vkGetImageSparseMemoryRequirements>,
    pub(crate) vkQueueBindSparse: Option<vk::PFN_vkQueueBindSparse>,
    pub(crate) vkCreateFence: Option<vk::PFN_vkCreateFence>,
    pub(crate) vkDestroyFence: Option<vk::PFN_vkDestroyFence>,
    pub(crate) vkResetFences: Option<vk::PFN_vkResetFences>,
    pub(crate) vkGetFenceStatus: Option<vk::PFN_vkGetFenceStatus>,
    pub(crate) vkWaitForFences: Option<vk::PFN_vkWaitForFences>,
    pub(crate) vkCreateSemaphore: Option<vk::PFN_vkCreateSemaphore>,
    pub(crate) vkDestroySemaphore: Option<vk::PFN_vkDestroySemaphore>,
    pub(crate) vkCreateQueryPool: Option<vk::PFN_vkCreateQueryPool>,
    pub(crate) vkDestroyQueryPool: Option<vk::PFN_vkDestroyQueryPool>,
    pub(crate) vkGetQueryPoolResults: Option<vk::PFN_vkGetQueryPoolResults>,
    pub(crate) vkCreateBuffer: Option<vk::PFN_vkCreateBuffer>,
    pub(crate) vkDestroyBuffer: Option<vk::PFN_vkDestroyBuffer>,
    pub(crate) vkCreateImage: Option<vk::PFN_vkCreateImage>,
    pub(crate) vkDestroyImage: Option<vk::PFN_vkDestroyImage>,
    pub(crate) vkGetImageSubresourceLayout: Option<vk::PFN_vkGetImageSubresourceLayout>,
    pub(crate) vkCreateImageView: Option<vk::PFN_vkCreateImageView>,
    pub(crate) vkDestroyImageView: Option<vk::PFN_vkDestroyImageView>,
    pub(crate) vkCreateCommandPool: Option<vk::PFN_vkCreateCommandPool>,
    pub(crate) vkDestroyCommandPool: Option<vk::PFN_vkDestroyCommandPool>,
    pub(crate) vkResetCommandPool: Option<vk::PFN_vkResetCommandPool>,
    pub(crate) vkAllocateCommandBuffers: Option<vk::PFN_vkAllocateCommandBuffers>,
    pub(crate) vkFreeCommandBuffers: Option<vk::PFN_vkFreeCommandBuffers>,
    pub(crate) vkBeginCommandBuffer: Option<vk::PFN_vkBeginCommandBuffer>,
    pub(crate) vkEndCommandBuffer: Option<vk::PFN_vkEndCommandBuffer>,
    pub(crate) vkResetCommandBuffer: Option<vk::PFN_vkResetCommandBuffer>,
    pub(crate) vkCmdCopyBuffer: Option<vk::PFN_vkCmdCopyBuffer>,
    pub(crate) vkCmdCopyImage: Option<vk::PFN_vkCmdCopyImage>,
    pub(crate) vkCmdCopyBufferToImage: Option<vk::PFN_vkCmdCopyBufferToImage>,
    pub(crate) vkCmdCopyImageToBuffer: Option<vk::PFN_vkCmdCopyImageToBuffer>,
    pub(crate) vkCmdUpdateBuffer: Option<vk::PFN_vkCmdUpdateBuffer>,
    pub(crate) vkCmdFillBuffer: Option<vk::PFN_vkCmdFillBuffer>,
    pub(crate) vkCmdPipelineBarrier: Option<vk::PFN_vkCmdPipelineBarrier>,
    pub(crate) vkCmdBeginQuery: Option<vk::PFN_vkCmdBeginQuery>,
    pub(crate) vkCmdEndQuery: Option<vk::PFN_vkCmdEndQuery>,
    pub(crate) vkCmdResetQueryPool: Option<vk::PFN_vkCmdResetQueryPool>,
    pub(crate) vkCmdWriteTimestamp: Option<vk::PFN_vkCmdWriteTimestamp>,
    pub(crate) vkCmdCopyQueryPoolResults: Option<vk::PFN_vkCmdCopyQueryPoolResults>,
    pub(crate) vkCmdExecuteCommands: Option<vk::PFN_vkCmdExecuteCommands>,
    pub(crate) vkCreateEvent: Option<vk::PFN_vkCreateEvent>,
    pub(crate) vkDestroyEvent: Option<vk::PFN_vkDestroyEvent>,
    pub(crate) vkGetEventStatus: Option<vk::PFN_vkGetEventStatus>,
    pub(crate) vkSetEvent: Option<vk::PFN_vkSetEvent>,
    pub(crate) vkResetEvent: Option<vk::PFN_vkResetEvent>,
    pub(crate) vkCreateBufferView: Option<vk::PFN_vkCreateBufferView>,
    pub(crate) vkDestroyBufferView: Option<vk::PFN_vkDestroyBufferView>,
    pub(crate) vkCreateShaderModule: Option<vk::PFN_vkCreateShaderModule>,
    pub(crate) vkDestroyShaderModule: Option<vk::PFN_vkDestroyShaderModule>,
    pub(crate) vkCreatePipelineCache: Option<vk::PFN_vkCreatePipelineCache>,
    pub(crate) vkDestroyPipelineCache: Option<vk::PFN_vkDestroyPipelineCache>,
    pub(crate) vkGetPipelineCacheData: Option<vk::PFN_vkGetPipelineCacheData>,
    pub(crate) vkMergePipelineCaches: Option<vk::PFN_vkMergePipelineCaches>,
    pub(crate) vkCreateComputePipelines: Option<vk::PFN_vkCreateComputePipelines>,
    pub(crate) vkDestroyPipeline: Option<vk::PFN_vkDestroyPipeline>,
    pub(crate) vkCreatePipelineLayout: Option<vk::PFN_vkCreatePipelineLayout>,
    pub(crate) vkDestroyPipelineLayout: Option<vk::PFN_vkDestroyPipelineLayout>,
    pub(crate) vkCreateSampler: Option<vk::PFN_vkCreateSampler>,
    pub(crate) vkDestroySampler: Option<vk::PFN_vkDestroySampler>,
    pub(crate) vkCreateDescriptorSetLayout: Option<vk::PFN_vkCreateDescriptorSetLayout>,
    pub(crate) vkDestroyDescriptorSetLayout: Option<vk::PFN_vkDestroyDescriptorSetLayout>,
    pub(crate) vkCreateDescriptorPool: Option<vk::PFN_vkCreateDescriptorPool>,
    pub(crate) vkDestroyDescriptorPool: Option<vk::PFN_vkDestroyDescriptorPool>,
    pub(crate) vkResetDescriptorPool: Option<vk::PFN_vkResetDescriptorPool>,
    pub(crate) vkAllocateDescriptorSets: Option<vk::PFN_vkAllocateDescriptorSets>,
    pub(crate) vkFreeDescriptorSets: Option<vk::PFN_vkFreeDescriptorSets>,
    pub(crate) vkUpdateDescriptorSets: Option<vk::PFN_vkUpdateDescriptorSets>,
    pub(crate) vkCmdBindPipeline: Option<vk::PFN_vkCmdBindPipeline>,
    pub(crate) vkCmdBindDescriptorSets: Option<vk::PFN_vkCmdBindDescriptorSets>,
    pub(crate) vkCmdClearColorImage: Option<vk::PFN_vkCmdClearColorImage>,
    pub(crate) vkCmdDispatch: Option<vk::PFN_vkCmdDispatch>,
    pub(crate) vkCmdDispatchIndirect: Option<vk::PFN_vkCmdDispatchIndirect>,
    pub(crate) vkCmdSetEvent: Option<vk::PFN_vkCmdSetEvent>,
    pub(crate) vkCmdResetEvent: Option<vk::PFN_vkCmdResetEvent>,
    pub(crate) vkCmdWaitEvents: Option<vk::PFN_vkCmdWaitEvents>,
    pub(crate) vkCmdPushConstants: Option<vk::PFN_vkCmdPushConstants>,
    pub(crate) vkCreateGraphicsPipelines: Option<vk::PFN_vkCreateGraphicsPipelines>,
    pub(crate) vkCreateFramebuffer: Option<vk::PFN_vkCreateFramebuffer>,
    pub(crate) vkDestroyFramebuffer: Option<vk::PFN_vkDestroyFramebuffer>,
    pub(crate) vkCreateRenderPass: Option<vk::PFN_vkCreateRenderPass>,
    pub(crate) vkDestroyRenderPass: Option<vk::PFN_vkDestroyRenderPass>,
    pub(crate) vkGetRenderAreaGranularity: Option<vk::PFN_vkGetRenderAreaGranularity>,
    pub(crate) vkCmdSetViewport: Option<vk::PFN_vkCmdSetViewport>,
    pub(crate) vkCmdSetScissor: Option<vk::PFN_vkCmdSetScissor>,
    pub(crate) vkCmdSetLineWidth: Option<vk::PFN_vkCmdSetLineWidth>,
    pub(crate) vkCmdSetDepthBias: Option<vk::PFN_vkCmdSetDepthBias>,
    pub(crate) vkCmdSetBlendConstants: Option<vk::PFN_vkCmdSetBlendConstants>,
    pub(crate) vkCmdSetDepthBounds: Option<vk::PFN_vkCmdSetDepthBounds>,
    pub(crate) vkCmdSetStencilCompareMask: Option<vk::PFN_vkCmdSetStencilCompareMask>,
    pub(crate) vkCmdSetStencilWriteMask: Option<vk::PFN_vkCmdSetStencilWriteMask>,
    pub(crate) vkCmdSetStencilReference: Option<vk::PFN_vkCmdSetStencilReference>,
    pub(crate) vkCmdBindIndexBuffer: Option<vk::PFN_vkCmdBindIndexBuffer>,
    pub(crate) vkCmdBindVertexBuffers: Option<vk::PFN_vkCmdBindVertexBuffers>,
    pub(crate) vkCmdDraw: Option<vk::PFN_vkCmdDraw>,
    pub(crate) vkCmdDrawIndexed: Option<vk::PFN_vkCmdDrawIndexed>,
    pub(crate) vkCmdDrawIndirect: Option<vk::PFN_vkCmdDrawIndirect>,
    pub(crate) vkCmdDrawIndexedIndirect: Option<vk::PFN_vkCmdDrawIndexedIndirect>,
    pub(crate) vkCmdBlitImage: Option<vk::PFN_vkCmdBlitImage>,
    pub(crate) vkCmdClearDepthStencilImage: Option<vk::PFN_vkCmdClearDepthStencilImage>,
    pub(crate) vkCmdClearAttachments: Option<vk::PFN_vkCmdClearAttachments>,
    pub(crate) vkCmdResolveImage: Option<vk::PFN_vkCmdResolveImage>,
    pub(crate) vkCmdBeginRenderPass: Option<vk::PFN_vkCmdBeginRenderPass>,
    pub(crate) vkCmdNextSubpass: Option<vk::PFN_vkCmdNextSubpass>,
    pub(crate) vkCmdEndRenderPass: Option<vk::PFN_vkCmdEndRenderPass>,
    pub(crate) vkBindBufferMemory2: Option<vk::PFN_vkBindBufferMemory2>,
    pub(crate) vkBindImageMemory2: Option<vk::PFN_vkBindImageMemory2>,
    pub(crate) vkGetDeviceGroupPeerMemoryFeatures:
        Option<vk::PFN_vkGetDeviceGroupPeerMemoryFeatures>,
    pub(crate) vkCmdSetDeviceMask: Option<vk::PFN_vkCmdSetDeviceMask>,
    pub(crate) vkGetImageMemoryRequirements2: Option<vk::PFN_vkGetImageMemoryRequirements2>,
    pub(crate) vkGetBufferMemoryRequirements2: Option<vk::PFN_vkGetBufferMemoryRequirements2>,
    pub(crate) vkGetImageSparseMemoryRequirements2:
        Option<vk::PFN_vkGetImageSparseMemoryRequirements2>,
    pub(crate) vkTrimCommandPool: Option<vk::PFN_vkTrimCommandPool>,
    pub(crate) vkGetDeviceQueue2: Option<vk::PFN_vkGetDeviceQueue2>,
    pub(crate) vkCmdDispatchBase: Option<vk::PFN_vkCmdDispatchBase>,
    pub(crate) vkCreateDescriptorUpdateTemplate: Option<vk::PFN_vkCreateDescriptorUpdateTemplate>,
    pub(crate) vkDestroyDescriptorUpdateTemplate: Option<vk::PFN_vkDestroyDescriptorUpdateTemplate>,
    pub(crate) vkUpdateDescriptorSetWithTemplate: Option<vk::PFN_vkUpdateDescriptorSetWithTemplate>,
    pub(crate) vkGetDescriptorSetLayoutSupport: Option<vk::PFN_vkGetDescriptorSetLayoutSupport>,
    pub(crate) vkCreateSamplerYcbcrConversion: Option<vk::PFN_vkCreateSamplerYcbcrConversion>,
    pub(crate) vkDestroySamplerYcbcrConversion: Option<vk::PFN_vkDestroySamplerYcbcrConversion>,
    pub(crate) vkResetQueryPool: Option<vk::PFN_vkResetQueryPool>,
    pub(crate) vkGetSemaphoreCounterValue: Option<vk::PFN_vkGetSemaphoreCounterValue>,
    pub(crate) vkWaitSemaphores: Option<vk::PFN_vkWaitSemaphores>,
    pub(crate) vkSignalSemaphore: Option<vk::PFN_vkSignalSemaphore>,
    pub(crate) vkGetBufferDeviceAddress: Option<vk::PFN_vkGetBufferDeviceAddress>,
    pub(crate) vkGetBufferOpaqueCaptureAddress: Option<vk::PFN_vkGetBufferOpaqueCaptureAddress>,
    pub(crate) vkGetDeviceMemoryOpaqueCaptureAddress:
        Option<vk::PFN_vkGetDeviceMemoryOpaqueCaptureAddress>,
    pub(crate) vkCmdDrawIndirectCount: Option<vk::PFN_vkCmdDrawIndirectCount>,
    pub(crate) vkCmdDrawIndexedIndirectCount: Option<vk::PFN_vkCmdDrawIndexedIndirectCount>,
    pub(crate) vkCreateRenderPass2: Option<vk::PFN_vkCreateRenderPass2>,
    pub(crate) vkCmdBeginRenderPass2: Option<vk::PFN_vkCmdBeginRenderPass2>,
    pub(crate) vkCmdNextSubpass2: Option<vk::PFN_vkCmdNextSubpass2>,
    pub(crate) vkCmdEndRenderPass2: Option<vk::PFN_vkCmdEndRenderPass2>,
    pub(crate) vkCreatePrivateDataSlot: Option<vk::PFN_vkCreatePrivateDataSlot>,
    pub(crate) vkDestroyPrivateDataSlot: Option<vk::PFN_vkDestroyPrivateDataSlot>,
    pub(crate) vkSetPrivateData: Option<vk::PFN_vkSetPrivateData>,
    pub(crate) vkGetPrivateData: Option<vk::PFN_vkGetPrivateData>,
    pub(crate) vkCmdPipelineBarrier2: Option<vk::PFN_vkCmdPipelineBarrier2>,
    pub(crate) vkCmdWriteTimestamp2: Option<vk::PFN_vkCmdWriteTimestamp2>,
    pub(crate) vkQueueSubmit2: Option<vk::PFN_vkQueueSubmit2>,
    pub(crate) vkCmdCopyBuffer2: Option<vk::PFN_vkCmdCopyBuffer2>,
    pub(crate) vkCmdCopyImage2: Option<vk::PFN_vkCmdCopyImage2>,
    pub(crate) vkCmdCopyBufferToImage2: Option<vk::PFN_vkCmdCopyBufferToImage2>,
    pub(crate) vkCmdCopyImageToBuffer2: Option<vk::PFN_vkCmdCopyImageToBuffer2>,
    pub(crate) vkGetDeviceBufferMemoryRequirements:
        Option<vk::PFN_vkGetDeviceBufferMemoryRequirements>,
    pub(crate) vkGetDeviceImageMemoryRequirements:
        Option<vk::PFN_vkGetDeviceImageMemoryRequirements>,
    pub(crate) vkGetDeviceImageSparseMemoryRequirements:
        Option<vk::PFN_vkGetDeviceImageSparseMemoryRequirements>,
    pub(crate) vkCmdSetEvent2: Option<vk::PFN_vkCmdSetEvent2>,
    pub(crate) vkCmdResetEvent2: Option<vk::PFN_vkCmdResetEvent2>,
    pub(crate) vkCmdWaitEvents2: Option<vk::PFN_vkCmdWaitEvents2>,
    pub(crate) vkCmdBlitImage2: Option<vk::PFN_vkCmdBlitImage2>,
    pub(crate) vkCmdResolveImage2: Option<vk::PFN_vkCmdResolveImage2>,
    pub(crate) vkCmdBeginRendering: Option<vk::PFN_vkCmdBeginRendering>,
    pub(crate) vkCmdEndRendering: Option<vk::PFN_vkCmdEndRendering>,
    pub(crate) vkCmdSetCullMode: Option<vk::PFN_vkCmdSetCullMode>,
    pub(crate) vkCmdSetFrontFace: Option<vk::PFN_vkCmdSetFrontFace>,
    pub(crate) vkCmdSetPrimitiveTopology: Option<vk::PFN_vkCmdSetPrimitiveTopology>,
    pub(crate) vkCmdSetViewportWithCount: Option<vk::PFN_vkCmdSetViewportWithCount>,
    pub(crate) vkCmdSetScissorWithCount: Option<vk::PFN_vkCmdSetScissorWithCount>,
    pub(crate) vkCmdBindVertexBuffers2: Option<vk::PFN_vkCmdBindVertexBuffers2>,
    pub(crate) vkCmdSetDepthTestEnable: Option<vk::PFN_vkCmdSetDepthTestEnable>,
    pub(crate) vkCmdSetDepthWriteEnable: Option<vk::PFN_vkCmdSetDepthWriteEnable>,
    pub(crate) vkCmdSetDepthCompareOp: Option<vk::PFN_vkCmdSetDepthCompareOp>,
    pub(crate) vkCmdSetDepthBoundsTestEnable: Option<vk::PFN_vkCmdSetDepthBoundsTestEnable>,
    pub(crate) vkCmdSetStencilTestEnable: Option<vk::PFN_vkCmdSetStencilTestEnable>,
    pub(crate) vkCmdSetStencilOp: Option<vk::PFN_vkCmdSetStencilOp>,
    pub(crate) vkCmdSetRasterizerDiscardEnable: Option<vk::PFN_vkCmdSetRasterizerDiscardEnable>,
    pub(crate) vkCmdSetDepthBiasEnable: Option<vk::PFN_vkCmdSetDepthBiasEnable>,
    pub(crate) vkCmdSetPrimitiveRestartEnable: Option<vk::PFN_vkCmdSetPrimitiveRestartEnable>,
    pub(crate) vkMapMemory2: Option<vk::PFN_vkMapMemory2>,
    pub(crate) vkUnmapMemory2: Option<vk::PFN_vkUnmapMemory2>,
    pub(crate) vkGetDeviceImageSubresourceLayout: Option<vk::PFN_vkGetDeviceImageSubresourceLayout>,
    pub(crate) vkGetImageSubresourceLayout2: Option<vk::PFN_vkGetImageSubresourceLayout2>,
    pub(crate) vkCopyMemoryToImage: Option<vk::PFN_vkCopyMemoryToImage>,
    pub(crate) vkCopyImageToMemory: Option<vk::PFN_vkCopyImageToMemory>,
    pub(crate) vkCopyImageToImage: Option<vk::PFN_vkCopyImageToImage>,
    pub(crate) vkTransitionImageLayout: Option<vk::PFN_vkTransitionImageLayout>,
    pub(crate) vkCmdPushDescriptorSet: Option<vk::PFN_vkCmdPushDescriptorSet>,
    pub(crate) vkCmdPushDescriptorSetWithTemplate:
        Option<vk::PFN_vkCmdPushDescriptorSetWithTemplate>,
    pub(crate) vkCmdBindDescriptorSets2: Option<vk::PFN_vkCmdBindDescriptorSets2>,
    pub(crate) vkCmdPushConstants2: Option<vk::PFN_vkCmdPushConstants2>,
    pub(crate) vkCmdPushDescriptorSet2: Option<vk::PFN_vkCmdPushDescriptorSet2>,
    pub(crate) vkCmdPushDescriptorSetWithTemplate2:
        Option<vk::PFN_vkCmdPushDescriptorSetWithTemplate2>,
    pub(crate) vkCmdSetLineStipple: Option<vk::PFN_vkCmdSetLineStipple>,
    pub(crate) vkCmdBindIndexBuffer2: Option<vk::PFN_vkCmdBindIndexBuffer2>,
    pub(crate) vkGetRenderingAreaGranularity: Option<vk::PFN_vkGetRenderingAreaGranularity>,
    pub(crate) vkCmdSetRenderingAttachmentLocations:
        Option<vk::PFN_vkCmdSetRenderingAttachmentLocations>,
    pub(crate) vkCmdSetRenderingInputAttachmentIndices:
        Option<vk::PFN_vkCmdSetRenderingInputAttachmentIndices>,
    pub(crate) vkCreateSwapchainKHR: Option<vk::PFN_vkCreateSwapchainKHR>,
    pub(crate) vkDestroySwapchainKHR: Option<vk::PFN_vkDestroySwapchainKHR>,
    pub(crate) vkGetSwapchainImagesKHR: Option<vk::PFN_vkGetSwapchainImagesKHR>,
    pub(crate) vkAcquireNextImageKHR: Option<vk::PFN_vkAcquireNextImageKHR>,
    pub(crate) vkQueuePresentKHR: Option<vk::PFN_vkQueuePresentKHR>,
    pub(crate) vkGetDeviceGroupPresentCapabilitiesKHR:
        Option<vk::PFN_vkGetDeviceGroupPresentCapabilitiesKHR>,
    pub(crate) vkGetDeviceGroupSurfacePresentModesKHR:
        Option<vk::PFN_vkGetDeviceGroupSurfacePresentModesKHR>,
    pub(crate) vkAcquireNextImage2KHR: Option<vk::PFN_vkAcquireNextImage2KHR>,
    pub(crate) vkCreateSharedSwapchainsKHR: Option<vk::PFN_vkCreateSharedSwapchainsKHR>,
    pub(crate) vkCreateVideoSessionKHR: Option<vk::PFN_vkCreateVideoSessionKHR>,
    pub(crate) vkDestroyVideoSessionKHR: Option<vk::PFN_vkDestroyVideoSessionKHR>,
    pub(crate) vkGetVideoSessionMemoryRequirementsKHR:
        Option<vk::PFN_vkGetVideoSessionMemoryRequirementsKHR>,
    pub(crate) vkBindVideoSessionMemoryKHR: Option<vk::PFN_vkBindVideoSessionMemoryKHR>,
    pub(crate) vkCreateVideoSessionParametersKHR: Option<vk::PFN_vkCreateVideoSessionParametersKHR>,
    pub(crate) vkUpdateVideoSessionParametersKHR: Option<vk::PFN_vkUpdateVideoSessionParametersKHR>,
    pub(crate) vkDestroyVideoSessionParametersKHR:
        Option<vk::PFN_vkDestroyVideoSessionParametersKHR>,
    pub(crate) vkCmdBeginVideoCodingKHR: Option<vk::PFN_vkCmdBeginVideoCodingKHR>,
    pub(crate) vkCmdEndVideoCodingKHR: Option<vk::PFN_vkCmdEndVideoCodingKHR>,
    pub(crate) vkCmdControlVideoCodingKHR: Option<vk::PFN_vkCmdControlVideoCodingKHR>,
    pub(crate) vkCmdDecodeVideoKHR: Option<vk::PFN_vkCmdDecodeVideoKHR>,
    pub(crate) vkCmdBeginRenderingKHR: Option<vk::PFN_vkCmdBeginRenderingKHR>,
    pub(crate) vkCmdEndRenderingKHR: Option<vk::PFN_vkCmdEndRenderingKHR>,
    pub(crate) vkGetDeviceGroupPeerMemoryFeaturesKHR:
        Option<vk::PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR>,
    pub(crate) vkCmdSetDeviceMaskKHR: Option<vk::PFN_vkCmdSetDeviceMaskKHR>,
    pub(crate) vkCmdDispatchBaseKHR: Option<vk::PFN_vkCmdDispatchBaseKHR>,
    pub(crate) vkTrimCommandPoolKHR: Option<vk::PFN_vkTrimCommandPoolKHR>,
    #[cfg(target_os = "windows")]
    pub(crate) vkGetMemoryWin32HandleKHR: Option<vk::PFN_vkGetMemoryWin32HandleKHR>,
    #[cfg(target_os = "windows")]
    pub(crate) vkGetMemoryWin32HandlePropertiesKHR:
        Option<vk::PFN_vkGetMemoryWin32HandlePropertiesKHR>,
    pub(crate) vkGetMemoryFdKHR: Option<vk::PFN_vkGetMemoryFdKHR>,
    pub(crate) vkGetMemoryFdPropertiesKHR: Option<vk::PFN_vkGetMemoryFdPropertiesKHR>,
    #[cfg(target_os = "windows")]
    pub(crate) vkImportSemaphoreWin32HandleKHR: Option<vk::PFN_vkImportSemaphoreWin32HandleKHR>,
    #[cfg(target_os = "windows")]
    pub(crate) vkGetSemaphoreWin32HandleKHR: Option<vk::PFN_vkGetSemaphoreWin32HandleKHR>,
    pub(crate) vkImportSemaphoreFdKHR: Option<vk::PFN_vkImportSemaphoreFdKHR>,
    pub(crate) vkGetSemaphoreFdKHR: Option<vk::PFN_vkGetSemaphoreFdKHR>,
    pub(crate) vkCmdPushDescriptorSetKHR: Option<vk::PFN_vkCmdPushDescriptorSetKHR>,
    pub(crate) vkCmdPushDescriptorSetWithTemplateKHR:
        Option<vk::PFN_vkCmdPushDescriptorSetWithTemplateKHR>,
    pub(crate) vkCreateDescriptorUpdateTemplateKHR:
        Option<vk::PFN_vkCreateDescriptorUpdateTemplateKHR>,
    pub(crate) vkDestroyDescriptorUpdateTemplateKHR:
        Option<vk::PFN_vkDestroyDescriptorUpdateTemplateKHR>,
    pub(crate) vkUpdateDescriptorSetWithTemplateKHR:
        Option<vk::PFN_vkUpdateDescriptorSetWithTemplateKHR>,
    pub(crate) vkCreateRenderPass2KHR: Option<vk::PFN_vkCreateRenderPass2KHR>,
    pub(crate) vkCmdBeginRenderPass2KHR: Option<vk::PFN_vkCmdBeginRenderPass2KHR>,
    pub(crate) vkCmdNextSubpass2KHR: Option<vk::PFN_vkCmdNextSubpass2KHR>,
    pub(crate) vkCmdEndRenderPass2KHR: Option<vk::PFN_vkCmdEndRenderPass2KHR>,
    pub(crate) vkGetSwapchainStatusKHR: Option<vk::PFN_vkGetSwapchainStatusKHR>,
    #[cfg(target_os = "windows")]
    pub(crate) vkImportFenceWin32HandleKHR: Option<vk::PFN_vkImportFenceWin32HandleKHR>,
    #[cfg(target_os = "windows")]
    pub(crate) vkGetFenceWin32HandleKHR: Option<vk::PFN_vkGetFenceWin32HandleKHR>,
    pub(crate) vkImportFenceFdKHR: Option<vk::PFN_vkImportFenceFdKHR>,
    pub(crate) vkGetFenceFdKHR: Option<vk::PFN_vkGetFenceFdKHR>,
    pub(crate) vkAcquireProfilingLockKHR: Option<vk::PFN_vkAcquireProfilingLockKHR>,
    pub(crate) vkReleaseProfilingLockKHR: Option<vk::PFN_vkReleaseProfilingLockKHR>,
    pub(crate) vkGetImageMemoryRequirements2KHR: Option<vk::PFN_vkGetImageMemoryRequirements2KHR>,
    pub(crate) vkGetBufferMemoryRequirements2KHR: Option<vk::PFN_vkGetBufferMemoryRequirements2KHR>,
    pub(crate) vkGetImageSparseMemoryRequirements2KHR:
        Option<vk::PFN_vkGetImageSparseMemoryRequirements2KHR>,
    pub(crate) vkCreateSamplerYcbcrConversionKHR: Option<vk::PFN_vkCreateSamplerYcbcrConversionKHR>,
    pub(crate) vkDestroySamplerYcbcrConversionKHR:
        Option<vk::PFN_vkDestroySamplerYcbcrConversionKHR>,
    pub(crate) vkBindBufferMemory2KHR: Option<vk::PFN_vkBindBufferMemory2KHR>,
    pub(crate) vkBindImageMemory2KHR: Option<vk::PFN_vkBindImageMemory2KHR>,
    pub(crate) vkGetDescriptorSetLayoutSupportKHR:
        Option<vk::PFN_vkGetDescriptorSetLayoutSupportKHR>,
    pub(crate) vkCmdDrawIndirectCountKHR: Option<vk::PFN_vkCmdDrawIndirectCountKHR>,
    pub(crate) vkCmdDrawIndexedIndirectCountKHR: Option<vk::PFN_vkCmdDrawIndexedIndirectCountKHR>,
    pub(crate) vkGetSemaphoreCounterValueKHR: Option<vk::PFN_vkGetSemaphoreCounterValueKHR>,
    pub(crate) vkWaitSemaphoresKHR: Option<vk::PFN_vkWaitSemaphoresKHR>,
    pub(crate) vkSignalSemaphoreKHR: Option<vk::PFN_vkSignalSemaphoreKHR>,
    pub(crate) vkCmdSetFragmentShadingRateKHR: Option<vk::PFN_vkCmdSetFragmentShadingRateKHR>,
    pub(crate) vkCmdSetRenderingAttachmentLocationsKHR:
        Option<vk::PFN_vkCmdSetRenderingAttachmentLocationsKHR>,
    pub(crate) vkCmdSetRenderingInputAttachmentIndicesKHR:
        Option<vk::PFN_vkCmdSetRenderingInputAttachmentIndicesKHR>,
    pub(crate) vkWaitForPresentKHR: Option<vk::PFN_vkWaitForPresentKHR>,
    pub(crate) vkGetBufferDeviceAddressKHR: Option<vk::PFN_vkGetBufferDeviceAddressKHR>,
    pub(crate) vkGetBufferOpaqueCaptureAddressKHR:
        Option<vk::PFN_vkGetBufferOpaqueCaptureAddressKHR>,
    pub(crate) vkGetDeviceMemoryOpaqueCaptureAddressKHR:
        Option<vk::PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR>,
    pub(crate) vkCreateDeferredOperationKHR: Option<vk::PFN_vkCreateDeferredOperationKHR>,
    pub(crate) vkDestroyDeferredOperationKHR: Option<vk::PFN_vkDestroyDeferredOperationKHR>,
    pub(crate) vkGetDeferredOperationMaxConcurrencyKHR:
        Option<vk::PFN_vkGetDeferredOperationMaxConcurrencyKHR>,
    pub(crate) vkGetDeferredOperationResultKHR: Option<vk::PFN_vkGetDeferredOperationResultKHR>,
    pub(crate) vkDeferredOperationJoinKHR: Option<vk::PFN_vkDeferredOperationJoinKHR>,
    pub(crate) vkGetPipelineExecutablePropertiesKHR:
        Option<vk::PFN_vkGetPipelineExecutablePropertiesKHR>,
    pub(crate) vkGetPipelineExecutableStatisticsKHR:
        Option<vk::PFN_vkGetPipelineExecutableStatisticsKHR>,
    pub(crate) vkGetPipelineExecutableInternalRepresentationsKHR:
        Option<vk::PFN_vkGetPipelineExecutableInternalRepresentationsKHR>,
    pub(crate) vkMapMemory2KHR: Option<vk::PFN_vkMapMemory2KHR>,
    pub(crate) vkUnmapMemory2KHR: Option<vk::PFN_vkUnmapMemory2KHR>,
    pub(crate) vkGetEncodedVideoSessionParametersKHR:
        Option<vk::PFN_vkGetEncodedVideoSessionParametersKHR>,
    pub(crate) vkCmdEncodeVideoKHR: Option<vk::PFN_vkCmdEncodeVideoKHR>,
    pub(crate) vkCmdSetEvent2KHR: Option<vk::PFN_vkCmdSetEvent2KHR>,
    pub(crate) vkCmdResetEvent2KHR: Option<vk::PFN_vkCmdResetEvent2KHR>,
    pub(crate) vkCmdWaitEvents2KHR: Option<vk::PFN_vkCmdWaitEvents2KHR>,
    pub(crate) vkCmdPipelineBarrier2KHR: Option<vk::PFN_vkCmdPipelineBarrier2KHR>,
    pub(crate) vkCmdWriteTimestamp2KHR: Option<vk::PFN_vkCmdWriteTimestamp2KHR>,
    pub(crate) vkQueueSubmit2KHR: Option<vk::PFN_vkQueueSubmit2KHR>,
    pub(crate) vkCmdBindIndexBuffer3KHR: Option<vk::PFN_vkCmdBindIndexBuffer3KHR>,
    pub(crate) vkCmdBindVertexBuffers3KHR: Option<vk::PFN_vkCmdBindVertexBuffers3KHR>,
    pub(crate) vkCmdDrawIndirect2KHR: Option<vk::PFN_vkCmdDrawIndirect2KHR>,
    pub(crate) vkCmdDrawIndexedIndirect2KHR: Option<vk::PFN_vkCmdDrawIndexedIndirect2KHR>,
    pub(crate) vkCmdDispatchIndirect2KHR: Option<vk::PFN_vkCmdDispatchIndirect2KHR>,
    pub(crate) vkCmdCopyMemoryKHR: Option<vk::PFN_vkCmdCopyMemoryKHR>,
    pub(crate) vkCmdCopyMemoryToImageKHR: Option<vk::PFN_vkCmdCopyMemoryToImageKHR>,
    pub(crate) vkCmdCopyImageToMemoryKHR: Option<vk::PFN_vkCmdCopyImageToMemoryKHR>,
    pub(crate) vkCmdUpdateMemoryKHR: Option<vk::PFN_vkCmdUpdateMemoryKHR>,
    pub(crate) vkCmdFillMemoryKHR: Option<vk::PFN_vkCmdFillMemoryKHR>,
    pub(crate) vkCmdCopyQueryPoolResultsToMemoryKHR:
        Option<vk::PFN_vkCmdCopyQueryPoolResultsToMemoryKHR>,
    pub(crate) vkCmdDrawIndirectCount2KHR: Option<vk::PFN_vkCmdDrawIndirectCount2KHR>,
    pub(crate) vkCmdDrawIndexedIndirectCount2KHR: Option<vk::PFN_vkCmdDrawIndexedIndirectCount2KHR>,
    pub(crate) vkCmdBeginConditionalRendering2EXT:
        Option<vk::PFN_vkCmdBeginConditionalRendering2EXT>,
    pub(crate) vkCmdBindTransformFeedbackBuffers2EXT:
        Option<vk::PFN_vkCmdBindTransformFeedbackBuffers2EXT>,
    pub(crate) vkCmdBeginTransformFeedback2EXT: Option<vk::PFN_vkCmdBeginTransformFeedback2EXT>,
    pub(crate) vkCmdEndTransformFeedback2EXT: Option<vk::PFN_vkCmdEndTransformFeedback2EXT>,
    pub(crate) vkCmdDrawIndirectByteCount2EXT: Option<vk::PFN_vkCmdDrawIndirectByteCount2EXT>,
    pub(crate) vkCmdDrawMeshTasksIndirect2EXT: Option<vk::PFN_vkCmdDrawMeshTasksIndirect2EXT>,
    pub(crate) vkCmdDrawMeshTasksIndirectCount2EXT:
        Option<vk::PFN_vkCmdDrawMeshTasksIndirectCount2EXT>,
    pub(crate) vkCmdWriteMarkerToMemoryAMD: Option<vk::PFN_vkCmdWriteMarkerToMemoryAMD>,
    pub(crate) vkCreateAccelerationStructure2KHR: Option<vk::PFN_vkCreateAccelerationStructure2KHR>,
    pub(crate) vkCmdCopyBuffer2KHR: Option<vk::PFN_vkCmdCopyBuffer2KHR>,
    pub(crate) vkCmdCopyImage2KHR: Option<vk::PFN_vkCmdCopyImage2KHR>,
    pub(crate) vkCmdCopyBufferToImage2KHR: Option<vk::PFN_vkCmdCopyBufferToImage2KHR>,
    pub(crate) vkCmdCopyImageToBuffer2KHR: Option<vk::PFN_vkCmdCopyImageToBuffer2KHR>,
    pub(crate) vkCmdBlitImage2KHR: Option<vk::PFN_vkCmdBlitImage2KHR>,
    pub(crate) vkCmdResolveImage2KHR: Option<vk::PFN_vkCmdResolveImage2KHR>,
    pub(crate) vkCmdTraceRaysIndirect2KHR: Option<vk::PFN_vkCmdTraceRaysIndirect2KHR>,
    pub(crate) vkGetDeviceBufferMemoryRequirementsKHR:
        Option<vk::PFN_vkGetDeviceBufferMemoryRequirementsKHR>,
    pub(crate) vkGetDeviceImageMemoryRequirementsKHR:
        Option<vk::PFN_vkGetDeviceImageMemoryRequirementsKHR>,
    pub(crate) vkGetDeviceImageSparseMemoryRequirementsKHR:
        Option<vk::PFN_vkGetDeviceImageSparseMemoryRequirementsKHR>,
    pub(crate) vkCmdBindIndexBuffer2KHR: Option<vk::PFN_vkCmdBindIndexBuffer2KHR>,
    pub(crate) vkGetRenderingAreaGranularityKHR: Option<vk::PFN_vkGetRenderingAreaGranularityKHR>,
    pub(crate) vkGetDeviceImageSubresourceLayoutKHR:
        Option<vk::PFN_vkGetDeviceImageSubresourceLayoutKHR>,
    pub(crate) vkGetImageSubresourceLayout2KHR: Option<vk::PFN_vkGetImageSubresourceLayout2KHR>,
    pub(crate) vkWaitForPresent2KHR: Option<vk::PFN_vkWaitForPresent2KHR>,
    pub(crate) vkCreatePipelineBinariesKHR: Option<vk::PFN_vkCreatePipelineBinariesKHR>,
    pub(crate) vkDestroyPipelineBinaryKHR: Option<vk::PFN_vkDestroyPipelineBinaryKHR>,
    pub(crate) vkGetPipelineKeyKHR: Option<vk::PFN_vkGetPipelineKeyKHR>,
    pub(crate) vkGetPipelineBinaryDataKHR: Option<vk::PFN_vkGetPipelineBinaryDataKHR>,
    pub(crate) vkReleaseCapturedPipelineDataKHR: Option<vk::PFN_vkReleaseCapturedPipelineDataKHR>,
    pub(crate) vkReleaseSwapchainImagesKHR: Option<vk::PFN_vkReleaseSwapchainImagesKHR>,
    pub(crate) vkCmdSetLineStippleKHR: Option<vk::PFN_vkCmdSetLineStippleKHR>,
    pub(crate) vkGetCalibratedTimestampsKHR: Option<vk::PFN_vkGetCalibratedTimestampsKHR>,
    pub(crate) vkCmdBindDescriptorSets2KHR: Option<vk::PFN_vkCmdBindDescriptorSets2KHR>,
    pub(crate) vkCmdPushConstants2KHR: Option<vk::PFN_vkCmdPushConstants2KHR>,
    pub(crate) vkCmdPushDescriptorSet2KHR: Option<vk::PFN_vkCmdPushDescriptorSet2KHR>,
    pub(crate) vkCmdPushDescriptorSetWithTemplate2KHR:
        Option<vk::PFN_vkCmdPushDescriptorSetWithTemplate2KHR>,
    pub(crate) vkCmdSetDescriptorBufferOffsets2EXT:
        Option<vk::PFN_vkCmdSetDescriptorBufferOffsets2EXT>,
    pub(crate) vkCmdBindDescriptorBufferEmbeddedSamplers2EXT:
        Option<vk::PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT>,
    pub(crate) vkCmdCopyMemoryIndirectKHR: Option<vk::PFN_vkCmdCopyMemoryIndirectKHR>,
    pub(crate) vkCmdCopyMemoryToImageIndirectKHR: Option<vk::PFN_vkCmdCopyMemoryToImageIndirectKHR>,
    pub(crate) vkGetDeviceFaultReportsKHR: Option<vk::PFN_vkGetDeviceFaultReportsKHR>,
    pub(crate) vkGetDeviceFaultDebugInfoKHR: Option<vk::PFN_vkGetDeviceFaultDebugInfoKHR>,
    pub(crate) vkCmdEndRendering2KHR: Option<vk::PFN_vkCmdEndRendering2KHR>,
    pub(crate) vkDebugMarkerSetObjectTagEXT: Option<vk::PFN_vkDebugMarkerSetObjectTagEXT>,
    pub(crate) vkDebugMarkerSetObjectNameEXT: Option<vk::PFN_vkDebugMarkerSetObjectNameEXT>,
    pub(crate) vkCmdDebugMarkerBeginEXT: Option<vk::PFN_vkCmdDebugMarkerBeginEXT>,
    pub(crate) vkCmdDebugMarkerEndEXT: Option<vk::PFN_vkCmdDebugMarkerEndEXT>,
    pub(crate) vkCmdDebugMarkerInsertEXT: Option<vk::PFN_vkCmdDebugMarkerInsertEXT>,
    pub(crate) vkCmdBindTransformFeedbackBuffersEXT:
        Option<vk::PFN_vkCmdBindTransformFeedbackBuffersEXT>,
    pub(crate) vkCmdBeginTransformFeedbackEXT: Option<vk::PFN_vkCmdBeginTransformFeedbackEXT>,
    pub(crate) vkCmdEndTransformFeedbackEXT: Option<vk::PFN_vkCmdEndTransformFeedbackEXT>,
    pub(crate) vkCmdBeginQueryIndexedEXT: Option<vk::PFN_vkCmdBeginQueryIndexedEXT>,
    pub(crate) vkCmdEndQueryIndexedEXT: Option<vk::PFN_vkCmdEndQueryIndexedEXT>,
    pub(crate) vkCmdDrawIndirectByteCountEXT: Option<vk::PFN_vkCmdDrawIndirectByteCountEXT>,
    pub(crate) vkCreateCuModuleNVX: Option<vk::PFN_vkCreateCuModuleNVX>,
    pub(crate) vkCreateCuFunctionNVX: Option<vk::PFN_vkCreateCuFunctionNVX>,
    pub(crate) vkDestroyCuModuleNVX: Option<vk::PFN_vkDestroyCuModuleNVX>,
    pub(crate) vkDestroyCuFunctionNVX: Option<vk::PFN_vkDestroyCuFunctionNVX>,
    pub(crate) vkCmdCuLaunchKernelNVX: Option<vk::PFN_vkCmdCuLaunchKernelNVX>,
    pub(crate) vkGetImageViewHandleNVX: Option<vk::PFN_vkGetImageViewHandleNVX>,
    pub(crate) vkGetImageViewHandle64NVX: Option<vk::PFN_vkGetImageViewHandle64NVX>,
    pub(crate) vkGetImageViewAddressNVX: Option<vk::PFN_vkGetImageViewAddressNVX>,
    pub(crate) vkGetDeviceCombinedImageSamplerIndexNVX:
        Option<vk::PFN_vkGetDeviceCombinedImageSamplerIndexNVX>,
    pub(crate) vkCmdDrawIndirectCountAMD: Option<vk::PFN_vkCmdDrawIndirectCountAMD>,
    pub(crate) vkCmdDrawIndexedIndirectCountAMD: Option<vk::PFN_vkCmdDrawIndexedIndirectCountAMD>,
    pub(crate) vkGetShaderInfoAMD: Option<vk::PFN_vkGetShaderInfoAMD>,
    #[cfg(target_os = "windows")]
    pub(crate) vkGetMemoryWin32HandleNV: Option<vk::PFN_vkGetMemoryWin32HandleNV>,
    pub(crate) vkCmdBeginConditionalRenderingEXT: Option<vk::PFN_vkCmdBeginConditionalRenderingEXT>,
    pub(crate) vkCmdEndConditionalRenderingEXT: Option<vk::PFN_vkCmdEndConditionalRenderingEXT>,
    pub(crate) vkCmdSetViewportWScalingNV: Option<vk::PFN_vkCmdSetViewportWScalingNV>,
    pub(crate) vkDisplayPowerControlEXT: Option<vk::PFN_vkDisplayPowerControlEXT>,
    pub(crate) vkRegisterDeviceEventEXT: Option<vk::PFN_vkRegisterDeviceEventEXT>,
    pub(crate) vkRegisterDisplayEventEXT: Option<vk::PFN_vkRegisterDisplayEventEXT>,
    pub(crate) vkGetSwapchainCounterEXT: Option<vk::PFN_vkGetSwapchainCounterEXT>,
    pub(crate) vkGetRefreshCycleDurationGOOGLE: Option<vk::PFN_vkGetRefreshCycleDurationGOOGLE>,
    pub(crate) vkGetPastPresentationTimingGOOGLE: Option<vk::PFN_vkGetPastPresentationTimingGOOGLE>,
    pub(crate) vkCmdSetDiscardRectangleEXT: Option<vk::PFN_vkCmdSetDiscardRectangleEXT>,
    pub(crate) vkCmdSetDiscardRectangleEnableEXT: Option<vk::PFN_vkCmdSetDiscardRectangleEnableEXT>,
    pub(crate) vkCmdSetDiscardRectangleModeEXT: Option<vk::PFN_vkCmdSetDiscardRectangleModeEXT>,
    pub(crate) vkSetHdrMetadataEXT: Option<vk::PFN_vkSetHdrMetadataEXT>,
    pub(crate) vkSetDebugUtilsObjectNameEXT: Option<vk::PFN_vkSetDebugUtilsObjectNameEXT>,
    pub(crate) vkSetDebugUtilsObjectTagEXT: Option<vk::PFN_vkSetDebugUtilsObjectTagEXT>,
    pub(crate) vkQueueBeginDebugUtilsLabelEXT: Option<vk::PFN_vkQueueBeginDebugUtilsLabelEXT>,
    pub(crate) vkQueueEndDebugUtilsLabelEXT: Option<vk::PFN_vkQueueEndDebugUtilsLabelEXT>,
    pub(crate) vkQueueInsertDebugUtilsLabelEXT: Option<vk::PFN_vkQueueInsertDebugUtilsLabelEXT>,
    pub(crate) vkCmdBeginDebugUtilsLabelEXT: Option<vk::PFN_vkCmdBeginDebugUtilsLabelEXT>,
    pub(crate) vkCmdEndDebugUtilsLabelEXT: Option<vk::PFN_vkCmdEndDebugUtilsLabelEXT>,
    pub(crate) vkCmdInsertDebugUtilsLabelEXT: Option<vk::PFN_vkCmdInsertDebugUtilsLabelEXT>,
    #[cfg(target_os = "android")]
    pub(crate) vkGetAndroidHardwareBufferPropertiesANDROID:
        Option<vk::PFN_vkGetAndroidHardwareBufferPropertiesANDROID>,
    #[cfg(target_os = "android")]
    pub(crate) vkGetMemoryAndroidHardwareBufferANDROID:
        Option<vk::PFN_vkGetMemoryAndroidHardwareBufferANDROID>,
    pub(crate) vkCreateGpaSessionAMD: Option<vk::PFN_vkCreateGpaSessionAMD>,
    pub(crate) vkDestroyGpaSessionAMD: Option<vk::PFN_vkDestroyGpaSessionAMD>,
    pub(crate) vkSetGpaDeviceClockModeAMD: Option<vk::PFN_vkSetGpaDeviceClockModeAMD>,
    pub(crate) vkGetGpaDeviceClockInfoAMD: Option<vk::PFN_vkGetGpaDeviceClockInfoAMD>,
    pub(crate) vkCmdBeginGpaSessionAMD: Option<vk::PFN_vkCmdBeginGpaSessionAMD>,
    pub(crate) vkCmdEndGpaSessionAMD: Option<vk::PFN_vkCmdEndGpaSessionAMD>,
    pub(crate) vkCmdBeginGpaSampleAMD: Option<vk::PFN_vkCmdBeginGpaSampleAMD>,
    pub(crate) vkCmdEndGpaSampleAMD: Option<vk::PFN_vkCmdEndGpaSampleAMD>,
    pub(crate) vkGetGpaSessionStatusAMD: Option<vk::PFN_vkGetGpaSessionStatusAMD>,
    pub(crate) vkGetGpaSessionResultsAMD: Option<vk::PFN_vkGetGpaSessionResultsAMD>,
    pub(crate) vkResetGpaSessionAMD: Option<vk::PFN_vkResetGpaSessionAMD>,
    pub(crate) vkCmdCopyGpaSessionResultsAMD: Option<vk::PFN_vkCmdCopyGpaSessionResultsAMD>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkCreateExecutionGraphPipelinesAMDX:
        Option<vk::PFN_vkCreateExecutionGraphPipelinesAMDX>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkGetExecutionGraphPipelineScratchSizeAMDX:
        Option<vk::PFN_vkGetExecutionGraphPipelineScratchSizeAMDX>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkGetExecutionGraphPipelineNodeIndexAMDX:
        Option<vk::PFN_vkGetExecutionGraphPipelineNodeIndexAMDX>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkCmdInitializeGraphScratchMemoryAMDX:
        Option<vk::PFN_vkCmdInitializeGraphScratchMemoryAMDX>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkCmdDispatchGraphAMDX: Option<vk::PFN_vkCmdDispatchGraphAMDX>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkCmdDispatchGraphIndirectAMDX: Option<vk::PFN_vkCmdDispatchGraphIndirectAMDX>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkCmdDispatchGraphIndirectCountAMDX:
        Option<vk::PFN_vkCmdDispatchGraphIndirectCountAMDX>,
    pub(crate) vkWriteSamplerDescriptorsEXT: Option<vk::PFN_vkWriteSamplerDescriptorsEXT>,
    pub(crate) vkWriteResourceDescriptorsEXT: Option<vk::PFN_vkWriteResourceDescriptorsEXT>,
    pub(crate) vkCmdBindSamplerHeapEXT: Option<vk::PFN_vkCmdBindSamplerHeapEXT>,
    pub(crate) vkCmdBindResourceHeapEXT: Option<vk::PFN_vkCmdBindResourceHeapEXT>,
    pub(crate) vkCmdPushDataEXT: Option<vk::PFN_vkCmdPushDataEXT>,
    pub(crate) vkGetImageOpaqueCaptureDataEXT: Option<vk::PFN_vkGetImageOpaqueCaptureDataEXT>,
    pub(crate) vkRegisterCustomBorderColorEXT: Option<vk::PFN_vkRegisterCustomBorderColorEXT>,
    pub(crate) vkUnregisterCustomBorderColorEXT: Option<vk::PFN_vkUnregisterCustomBorderColorEXT>,
    pub(crate) vkGetTensorOpaqueCaptureDataARM: Option<vk::PFN_vkGetTensorOpaqueCaptureDataARM>,
    pub(crate) vkCmdSetSampleLocationsEXT: Option<vk::PFN_vkCmdSetSampleLocationsEXT>,
    pub(crate) vkGetImageDrmFormatModifierPropertiesEXT:
        Option<vk::PFN_vkGetImageDrmFormatModifierPropertiesEXT>,
    pub(crate) vkCreateValidationCacheEXT: Option<vk::PFN_vkCreateValidationCacheEXT>,
    pub(crate) vkDestroyValidationCacheEXT: Option<vk::PFN_vkDestroyValidationCacheEXT>,
    pub(crate) vkMergeValidationCachesEXT: Option<vk::PFN_vkMergeValidationCachesEXT>,
    pub(crate) vkGetValidationCacheDataEXT: Option<vk::PFN_vkGetValidationCacheDataEXT>,
    pub(crate) vkCmdBindShadingRateImageNV: Option<vk::PFN_vkCmdBindShadingRateImageNV>,
    pub(crate) vkCmdSetViewportShadingRatePaletteNV:
        Option<vk::PFN_vkCmdSetViewportShadingRatePaletteNV>,
    pub(crate) vkCmdSetCoarseSampleOrderNV: Option<vk::PFN_vkCmdSetCoarseSampleOrderNV>,
    pub(crate) vkCreateAccelerationStructureNV: Option<vk::PFN_vkCreateAccelerationStructureNV>,
    pub(crate) vkDestroyAccelerationStructureNV: Option<vk::PFN_vkDestroyAccelerationStructureNV>,
    pub(crate) vkGetAccelerationStructureMemoryRequirementsNV:
        Option<vk::PFN_vkGetAccelerationStructureMemoryRequirementsNV>,
    pub(crate) vkBindAccelerationStructureMemoryNV:
        Option<vk::PFN_vkBindAccelerationStructureMemoryNV>,
    pub(crate) vkCmdBuildAccelerationStructureNV: Option<vk::PFN_vkCmdBuildAccelerationStructureNV>,
    pub(crate) vkCmdCopyAccelerationStructureNV: Option<vk::PFN_vkCmdCopyAccelerationStructureNV>,
    pub(crate) vkCmdTraceRaysNV: Option<vk::PFN_vkCmdTraceRaysNV>,
    pub(crate) vkCreateRayTracingPipelinesNV: Option<vk::PFN_vkCreateRayTracingPipelinesNV>,
    pub(crate) vkGetRayTracingShaderGroupHandlesKHR:
        Option<vk::PFN_vkGetRayTracingShaderGroupHandlesKHR>,
    pub(crate) vkGetRayTracingShaderGroupHandlesNV:
        Option<vk::PFN_vkGetRayTracingShaderGroupHandlesNV>,
    pub(crate) vkGetAccelerationStructureHandleNV:
        Option<vk::PFN_vkGetAccelerationStructureHandleNV>,
    pub(crate) vkCmdWriteAccelerationStructuresPropertiesNV:
        Option<vk::PFN_vkCmdWriteAccelerationStructuresPropertiesNV>,
    pub(crate) vkCompileDeferredNV: Option<vk::PFN_vkCompileDeferredNV>,
    pub(crate) vkGetMemoryHostPointerPropertiesEXT:
        Option<vk::PFN_vkGetMemoryHostPointerPropertiesEXT>,
    pub(crate) vkCmdWriteBufferMarkerAMD: Option<vk::PFN_vkCmdWriteBufferMarkerAMD>,
    pub(crate) vkCmdWriteBufferMarker2AMD: Option<vk::PFN_vkCmdWriteBufferMarker2AMD>,
    pub(crate) vkGetCalibratedTimestampsEXT: Option<vk::PFN_vkGetCalibratedTimestampsEXT>,
    pub(crate) vkCmdDrawMeshTasksNV: Option<vk::PFN_vkCmdDrawMeshTasksNV>,
    pub(crate) vkCmdDrawMeshTasksIndirectNV: Option<vk::PFN_vkCmdDrawMeshTasksIndirectNV>,
    pub(crate) vkCmdDrawMeshTasksIndirectCountNV: Option<vk::PFN_vkCmdDrawMeshTasksIndirectCountNV>,
    pub(crate) vkCmdSetExclusiveScissorEnableNV: Option<vk::PFN_vkCmdSetExclusiveScissorEnableNV>,
    pub(crate) vkCmdSetExclusiveScissorNV: Option<vk::PFN_vkCmdSetExclusiveScissorNV>,
    pub(crate) vkCmdSetCheckpointNV: Option<vk::PFN_vkCmdSetCheckpointNV>,
    pub(crate) vkGetQueueCheckpointDataNV: Option<vk::PFN_vkGetQueueCheckpointDataNV>,
    pub(crate) vkGetQueueCheckpointData2NV: Option<vk::PFN_vkGetQueueCheckpointData2NV>,
    pub(crate) vkSetSwapchainPresentTimingQueueSizeEXT:
        Option<vk::PFN_vkSetSwapchainPresentTimingQueueSizeEXT>,
    pub(crate) vkGetSwapchainTimingPropertiesEXT: Option<vk::PFN_vkGetSwapchainTimingPropertiesEXT>,
    pub(crate) vkGetSwapchainTimeDomainPropertiesEXT:
        Option<vk::PFN_vkGetSwapchainTimeDomainPropertiesEXT>,
    pub(crate) vkGetPastPresentationTimingEXT: Option<vk::PFN_vkGetPastPresentationTimingEXT>,
    pub(crate) vkInitializePerformanceApiINTEL: Option<vk::PFN_vkInitializePerformanceApiINTEL>,
    pub(crate) vkUninitializePerformanceApiINTEL: Option<vk::PFN_vkUninitializePerformanceApiINTEL>,
    pub(crate) vkCmdSetPerformanceMarkerINTEL: Option<vk::PFN_vkCmdSetPerformanceMarkerINTEL>,
    pub(crate) vkCmdSetPerformanceStreamMarkerINTEL:
        Option<vk::PFN_vkCmdSetPerformanceStreamMarkerINTEL>,
    pub(crate) vkCmdSetPerformanceOverrideINTEL: Option<vk::PFN_vkCmdSetPerformanceOverrideINTEL>,
    pub(crate) vkAcquirePerformanceConfigurationINTEL:
        Option<vk::PFN_vkAcquirePerformanceConfigurationINTEL>,
    pub(crate) vkReleasePerformanceConfigurationINTEL:
        Option<vk::PFN_vkReleasePerformanceConfigurationINTEL>,
    pub(crate) vkQueueSetPerformanceConfigurationINTEL:
        Option<vk::PFN_vkQueueSetPerformanceConfigurationINTEL>,
    pub(crate) vkGetPerformanceParameterINTEL: Option<vk::PFN_vkGetPerformanceParameterINTEL>,
    pub(crate) vkSetLocalDimmingAMD: Option<vk::PFN_vkSetLocalDimmingAMD>,
    pub(crate) vkGetBufferDeviceAddressEXT: Option<vk::PFN_vkGetBufferDeviceAddressEXT>,
    #[cfg(target_os = "windows")]
    pub(crate) vkAcquireFullScreenExclusiveModeEXT:
        Option<vk::PFN_vkAcquireFullScreenExclusiveModeEXT>,
    #[cfg(target_os = "windows")]
    pub(crate) vkReleaseFullScreenExclusiveModeEXT:
        Option<vk::PFN_vkReleaseFullScreenExclusiveModeEXT>,
    #[cfg(target_os = "windows")]
    pub(crate) vkGetDeviceGroupSurfacePresentModes2EXT:
        Option<vk::PFN_vkGetDeviceGroupSurfacePresentModes2EXT>,
    pub(crate) vkCmdSetLineStippleEXT: Option<vk::PFN_vkCmdSetLineStippleEXT>,
    pub(crate) vkResetQueryPoolEXT: Option<vk::PFN_vkResetQueryPoolEXT>,
    pub(crate) vkCmdSetCullModeEXT: Option<vk::PFN_vkCmdSetCullModeEXT>,
    pub(crate) vkCmdSetFrontFaceEXT: Option<vk::PFN_vkCmdSetFrontFaceEXT>,
    pub(crate) vkCmdSetPrimitiveTopologyEXT: Option<vk::PFN_vkCmdSetPrimitiveTopologyEXT>,
    pub(crate) vkCmdSetViewportWithCountEXT: Option<vk::PFN_vkCmdSetViewportWithCountEXT>,
    pub(crate) vkCmdSetScissorWithCountEXT: Option<vk::PFN_vkCmdSetScissorWithCountEXT>,
    pub(crate) vkCmdBindVertexBuffers2EXT: Option<vk::PFN_vkCmdBindVertexBuffers2EXT>,
    pub(crate) vkCmdSetDepthTestEnableEXT: Option<vk::PFN_vkCmdSetDepthTestEnableEXT>,
    pub(crate) vkCmdSetDepthWriteEnableEXT: Option<vk::PFN_vkCmdSetDepthWriteEnableEXT>,
    pub(crate) vkCmdSetDepthCompareOpEXT: Option<vk::PFN_vkCmdSetDepthCompareOpEXT>,
    pub(crate) vkCmdSetDepthBoundsTestEnableEXT: Option<vk::PFN_vkCmdSetDepthBoundsTestEnableEXT>,
    pub(crate) vkCmdSetStencilTestEnableEXT: Option<vk::PFN_vkCmdSetStencilTestEnableEXT>,
    pub(crate) vkCmdSetStencilOpEXT: Option<vk::PFN_vkCmdSetStencilOpEXT>,
    pub(crate) vkCopyMemoryToImageEXT: Option<vk::PFN_vkCopyMemoryToImageEXT>,
    pub(crate) vkCopyImageToMemoryEXT: Option<vk::PFN_vkCopyImageToMemoryEXT>,
    pub(crate) vkCopyImageToImageEXT: Option<vk::PFN_vkCopyImageToImageEXT>,
    pub(crate) vkTransitionImageLayoutEXT: Option<vk::PFN_vkTransitionImageLayoutEXT>,
    pub(crate) vkGetImageSubresourceLayout2EXT: Option<vk::PFN_vkGetImageSubresourceLayout2EXT>,
    pub(crate) vkReleaseSwapchainImagesEXT: Option<vk::PFN_vkReleaseSwapchainImagesEXT>,
    pub(crate) vkGetGeneratedCommandsMemoryRequirementsNV:
        Option<vk::PFN_vkGetGeneratedCommandsMemoryRequirementsNV>,
    pub(crate) vkCmdPreprocessGeneratedCommandsNV:
        Option<vk::PFN_vkCmdPreprocessGeneratedCommandsNV>,
    pub(crate) vkCmdExecuteGeneratedCommandsNV: Option<vk::PFN_vkCmdExecuteGeneratedCommandsNV>,
    pub(crate) vkCmdBindPipelineShaderGroupNV: Option<vk::PFN_vkCmdBindPipelineShaderGroupNV>,
    pub(crate) vkCreateIndirectCommandsLayoutNV: Option<vk::PFN_vkCreateIndirectCommandsLayoutNV>,
    pub(crate) vkDestroyIndirectCommandsLayoutNV: Option<vk::PFN_vkDestroyIndirectCommandsLayoutNV>,
    pub(crate) vkCmdSetDepthBias2EXT: Option<vk::PFN_vkCmdSetDepthBias2EXT>,
    pub(crate) vkCreatePrivateDataSlotEXT: Option<vk::PFN_vkCreatePrivateDataSlotEXT>,
    pub(crate) vkDestroyPrivateDataSlotEXT: Option<vk::PFN_vkDestroyPrivateDataSlotEXT>,
    pub(crate) vkSetPrivateDataEXT: Option<vk::PFN_vkSetPrivateDataEXT>,
    pub(crate) vkGetPrivateDataEXT: Option<vk::PFN_vkGetPrivateDataEXT>,
    pub(crate) vkQueueSetPerfHintQCOM: Option<vk::PFN_vkQueueSetPerfHintQCOM>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkCreateCudaModuleNV: Option<vk::PFN_vkCreateCudaModuleNV>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkGetCudaModuleCacheNV: Option<vk::PFN_vkGetCudaModuleCacheNV>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkCreateCudaFunctionNV: Option<vk::PFN_vkCreateCudaFunctionNV>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkDestroyCudaModuleNV: Option<vk::PFN_vkDestroyCudaModuleNV>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkDestroyCudaFunctionNV: Option<vk::PFN_vkDestroyCudaFunctionNV>,
    #[cfg(feature = "beta-extensions")]
    pub(crate) vkCmdCudaLaunchKernelNV: Option<vk::PFN_vkCmdCudaLaunchKernelNV>,
    pub(crate) vkCmdDispatchTileQCOM: Option<vk::PFN_vkCmdDispatchTileQCOM>,
    pub(crate) vkCmdBeginPerTileExecutionQCOM: Option<vk::PFN_vkCmdBeginPerTileExecutionQCOM>,
    pub(crate) vkCmdEndPerTileExecutionQCOM: Option<vk::PFN_vkCmdEndPerTileExecutionQCOM>,
    pub(crate) vkSetLatencySleepModeLegacyNV: Option<vk::PFN_vkSetLatencySleepModeLegacyNV>,
    pub(crate) vkLatencySleepLegacyNV: Option<vk::PFN_vkLatencySleepLegacyNV>,
    pub(crate) vkSetLatencyMarkerLegacyNV: Option<vk::PFN_vkSetLatencyMarkerLegacyNV>,
    pub(crate) vkGetLatencyTimingsLegacyNV: Option<vk::PFN_vkGetLatencyTimingsLegacyNV>,
    pub(crate) vkQueueNotifyOutOfBandLegacyNV: Option<vk::PFN_vkQueueNotifyOutOfBandLegacyNV>,
    pub(crate) vkGetSleepStatusLegacyNV: Option<vk::PFN_vkGetSleepStatusLegacyNV>,
    pub(crate) vkShutdownLatencyDeviceLegacyNV: Option<vk::PFN_vkShutdownLatencyDeviceLegacyNV>,
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "visionos"
    ))]
    pub(crate) vkExportMetalObjectsEXT: Option<vk::PFN_vkExportMetalObjectsEXT>,
    pub(crate) vkGetDescriptorSetLayoutSizeEXT: Option<vk::PFN_vkGetDescriptorSetLayoutSizeEXT>,
    pub(crate) vkGetDescriptorSetLayoutBindingOffsetEXT:
        Option<vk::PFN_vkGetDescriptorSetLayoutBindingOffsetEXT>,
    pub(crate) vkGetDescriptorEXT: Option<vk::PFN_vkGetDescriptorEXT>,
    pub(crate) vkCmdBindDescriptorBuffersEXT: Option<vk::PFN_vkCmdBindDescriptorBuffersEXT>,
    pub(crate) vkCmdSetDescriptorBufferOffsetsEXT:
        Option<vk::PFN_vkCmdSetDescriptorBufferOffsetsEXT>,
    pub(crate) vkCmdBindDescriptorBufferEmbeddedSamplersEXT:
        Option<vk::PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT>,
    pub(crate) vkGetBufferOpaqueCaptureDescriptorDataEXT:
        Option<vk::PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT>,
    pub(crate) vkGetImageOpaqueCaptureDescriptorDataEXT:
        Option<vk::PFN_vkGetImageOpaqueCaptureDescriptorDataEXT>,
    pub(crate) vkGetImageViewOpaqueCaptureDescriptorDataEXT:
        Option<vk::PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT>,
    pub(crate) vkGetSamplerOpaqueCaptureDescriptorDataEXT:
        Option<vk::PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT>,
    pub(crate) vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT:
        Option<vk::PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT>,
    pub(crate) vkCmdSetFragmentShadingRateEnumNV: Option<vk::PFN_vkCmdSetFragmentShadingRateEnumNV>,
    pub(crate) vkGetDeviceFaultInfoEXT: Option<vk::PFN_vkGetDeviceFaultInfoEXT>,
    pub(crate) vkCmdSetVertexInputEXT: Option<vk::PFN_vkCmdSetVertexInputEXT>,
    #[cfg(target_os = "fuchsia")]
    pub(crate) vkGetMemoryZirconHandleFUCHSIA: Option<vk::PFN_vkGetMemoryZirconHandleFUCHSIA>,
    #[cfg(target_os = "fuchsia")]
    pub(crate) vkGetMemoryZirconHandlePropertiesFUCHSIA:
        Option<vk::PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA>,
    #[cfg(target_os = "fuchsia")]
    pub(crate) vkImportSemaphoreZirconHandleFUCHSIA:
        Option<vk::PFN_vkImportSemaphoreZirconHandleFUCHSIA>,
    #[cfg(target_os = "fuchsia")]
    pub(crate) vkGetSemaphoreZirconHandleFUCHSIA: Option<vk::PFN_vkGetSemaphoreZirconHandleFUCHSIA>,
    #[cfg(target_os = "fuchsia")]
    pub(crate) vkCreateBufferCollectionFUCHSIA: Option<vk::PFN_vkCreateBufferCollectionFUCHSIA>,
    #[cfg(target_os = "fuchsia")]
    pub(crate) vkSetBufferCollectionImageConstraintsFUCHSIA:
        Option<vk::PFN_vkSetBufferCollectionImageConstraintsFUCHSIA>,
    #[cfg(target_os = "fuchsia")]
    pub(crate) vkSetBufferCollectionBufferConstraintsFUCHSIA:
        Option<vk::PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA>,
    #[cfg(target_os = "fuchsia")]
    pub(crate) vkDestroyBufferCollectionFUCHSIA: Option<vk::PFN_vkDestroyBufferCollectionFUCHSIA>,
    #[cfg(target_os = "fuchsia")]
    pub(crate) vkGetBufferCollectionPropertiesFUCHSIA:
        Option<vk::PFN_vkGetBufferCollectionPropertiesFUCHSIA>,
    pub(crate) vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI:
        Option<vk::PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI>,
    pub(crate) vkCmdSubpassShadingHUAWEI: Option<vk::PFN_vkCmdSubpassShadingHUAWEI>,
    pub(crate) vkCmdBindInvocationMaskHUAWEI: Option<vk::PFN_vkCmdBindInvocationMaskHUAWEI>,
    pub(crate) vkGetMemoryRemoteAddressNV: Option<vk::PFN_vkGetMemoryRemoteAddressNV>,
    pub(crate) vkGetPipelinePropertiesEXT: Option<vk::PFN_vkGetPipelinePropertiesEXT>,
    pub(crate) vkCmdSetPatchControlPointsEXT: Option<vk::PFN_vkCmdSetPatchControlPointsEXT>,
    pub(crate) vkCmdSetRasterizerDiscardEnableEXT:
        Option<vk::PFN_vkCmdSetRasterizerDiscardEnableEXT>,
    pub(crate) vkCmdSetDepthBiasEnableEXT: Option<vk::PFN_vkCmdSetDepthBiasEnableEXT>,
    pub(crate) vkCmdSetLogicOpEXT: Option<vk::PFN_vkCmdSetLogicOpEXT>,
    pub(crate) vkCmdSetPrimitiveRestartEnableEXT: Option<vk::PFN_vkCmdSetPrimitiveRestartEnableEXT>,
    pub(crate) vkCmdSetColorWriteEnableEXT: Option<vk::PFN_vkCmdSetColorWriteEnableEXT>,
    pub(crate) vkCmdDrawMultiEXT: Option<vk::PFN_vkCmdDrawMultiEXT>,
    pub(crate) vkCmdDrawMultiIndexedEXT: Option<vk::PFN_vkCmdDrawMultiIndexedEXT>,
    pub(crate) vkCreateMicromapEXT: Option<vk::PFN_vkCreateMicromapEXT>,
    pub(crate) vkDestroyMicromapEXT: Option<vk::PFN_vkDestroyMicromapEXT>,
    pub(crate) vkCmdBuildMicromapsEXT: Option<vk::PFN_vkCmdBuildMicromapsEXT>,
    pub(crate) vkBuildMicromapsEXT: Option<vk::PFN_vkBuildMicromapsEXT>,
    pub(crate) vkCopyMicromapEXT: Option<vk::PFN_vkCopyMicromapEXT>,
    pub(crate) vkCopyMicromapToMemoryEXT: Option<vk::PFN_vkCopyMicromapToMemoryEXT>,
    pub(crate) vkCopyMemoryToMicromapEXT: Option<vk::PFN_vkCopyMemoryToMicromapEXT>,
    pub(crate) vkWriteMicromapsPropertiesEXT: Option<vk::PFN_vkWriteMicromapsPropertiesEXT>,
    pub(crate) vkCmdCopyMicromapEXT: Option<vk::PFN_vkCmdCopyMicromapEXT>,
    pub(crate) vkCmdCopyMicromapToMemoryEXT: Option<vk::PFN_vkCmdCopyMicromapToMemoryEXT>,
    pub(crate) vkCmdCopyMemoryToMicromapEXT: Option<vk::PFN_vkCmdCopyMemoryToMicromapEXT>,
    pub(crate) vkCmdWriteMicromapsPropertiesEXT: Option<vk::PFN_vkCmdWriteMicromapsPropertiesEXT>,
    pub(crate) vkGetDeviceMicromapCompatibilityEXT:
        Option<vk::PFN_vkGetDeviceMicromapCompatibilityEXT>,
    pub(crate) vkGetMicromapBuildSizesEXT: Option<vk::PFN_vkGetMicromapBuildSizesEXT>,
    pub(crate) vkCmdDrawClusterHUAWEI: Option<vk::PFN_vkCmdDrawClusterHUAWEI>,
    pub(crate) vkCmdDrawClusterIndirectHUAWEI: Option<vk::PFN_vkCmdDrawClusterIndirectHUAWEI>,
    pub(crate) vkSetDeviceMemoryPriorityEXT: Option<vk::PFN_vkSetDeviceMemoryPriorityEXT>,
    pub(crate) vkCmdSetDispatchParametersARM: Option<vk::PFN_vkCmdSetDispatchParametersARM>,
    pub(crate) vkGetDescriptorSetLayoutHostMappingInfoVALVE:
        Option<vk::PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE>,
    pub(crate) vkGetDescriptorSetHostMappingVALVE:
        Option<vk::PFN_vkGetDescriptorSetHostMappingVALVE>,
    pub(crate) vkCmdCopyMemoryIndirectNV: Option<vk::PFN_vkCmdCopyMemoryIndirectNV>,
    pub(crate) vkCmdCopyMemoryToImageIndirectNV: Option<vk::PFN_vkCmdCopyMemoryToImageIndirectNV>,
    pub(crate) vkCmdDecompressMemoryNV: Option<vk::PFN_vkCmdDecompressMemoryNV>,
    pub(crate) vkCmdDecompressMemoryIndirectCountNV:
        Option<vk::PFN_vkCmdDecompressMemoryIndirectCountNV>,
    pub(crate) vkGetPipelineIndirectMemoryRequirementsNV:
        Option<vk::PFN_vkGetPipelineIndirectMemoryRequirementsNV>,
    pub(crate) vkCmdUpdatePipelineIndirectBufferNV:
        Option<vk::PFN_vkCmdUpdatePipelineIndirectBufferNV>,
    pub(crate) vkGetPipelineIndirectDeviceAddressNV:
        Option<vk::PFN_vkGetPipelineIndirectDeviceAddressNV>,
    #[cfg(target_env = "ohos")]
    pub(crate) vkGetNativeBufferPropertiesOHOS: Option<vk::PFN_vkGetNativeBufferPropertiesOHOS>,
    #[cfg(target_env = "ohos")]
    pub(crate) vkGetMemoryNativeBufferOHOS: Option<vk::PFN_vkGetMemoryNativeBufferOHOS>,
    pub(crate) vkCmdSetDepthClampEnableEXT: Option<vk::PFN_vkCmdSetDepthClampEnableEXT>,
    pub(crate) vkCmdSetPolygonModeEXT: Option<vk::PFN_vkCmdSetPolygonModeEXT>,
    pub(crate) vkCmdSetRasterizationSamplesEXT: Option<vk::PFN_vkCmdSetRasterizationSamplesEXT>,
    pub(crate) vkCmdSetSampleMaskEXT: Option<vk::PFN_vkCmdSetSampleMaskEXT>,
    pub(crate) vkCmdSetAlphaToCoverageEnableEXT: Option<vk::PFN_vkCmdSetAlphaToCoverageEnableEXT>,
    pub(crate) vkCmdSetAlphaToOneEnableEXT: Option<vk::PFN_vkCmdSetAlphaToOneEnableEXT>,
    pub(crate) vkCmdSetLogicOpEnableEXT: Option<vk::PFN_vkCmdSetLogicOpEnableEXT>,
    pub(crate) vkCmdSetColorBlendEnableEXT: Option<vk::PFN_vkCmdSetColorBlendEnableEXT>,
    pub(crate) vkCmdSetColorBlendEquationEXT: Option<vk::PFN_vkCmdSetColorBlendEquationEXT>,
    pub(crate) vkCmdSetColorWriteMaskEXT: Option<vk::PFN_vkCmdSetColorWriteMaskEXT>,
    pub(crate) vkCmdSetTessellationDomainOriginEXT:
        Option<vk::PFN_vkCmdSetTessellationDomainOriginEXT>,
    pub(crate) vkCmdSetRasterizationStreamEXT: Option<vk::PFN_vkCmdSetRasterizationStreamEXT>,
    pub(crate) vkCmdSetConservativeRasterizationModeEXT:
        Option<vk::PFN_vkCmdSetConservativeRasterizationModeEXT>,
    pub(crate) vkCmdSetExtraPrimitiveOverestimationSizeEXT:
        Option<vk::PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT>,
    pub(crate) vkCmdSetDepthClipEnableEXT: Option<vk::PFN_vkCmdSetDepthClipEnableEXT>,
    pub(crate) vkCmdSetSampleLocationsEnableEXT: Option<vk::PFN_vkCmdSetSampleLocationsEnableEXT>,
    pub(crate) vkCmdSetColorBlendAdvancedEXT: Option<vk::PFN_vkCmdSetColorBlendAdvancedEXT>,
    pub(crate) vkCmdSetProvokingVertexModeEXT: Option<vk::PFN_vkCmdSetProvokingVertexModeEXT>,
    pub(crate) vkCmdSetLineRasterizationModeEXT: Option<vk::PFN_vkCmdSetLineRasterizationModeEXT>,
    pub(crate) vkCmdSetLineStippleEnableEXT: Option<vk::PFN_vkCmdSetLineStippleEnableEXT>,
    pub(crate) vkCmdSetDepthClipNegativeOneToOneEXT:
        Option<vk::PFN_vkCmdSetDepthClipNegativeOneToOneEXT>,
    pub(crate) vkCmdSetViewportWScalingEnableNV: Option<vk::PFN_vkCmdSetViewportWScalingEnableNV>,
    pub(crate) vkCmdSetViewportSwizzleNV: Option<vk::PFN_vkCmdSetViewportSwizzleNV>,
    pub(crate) vkCmdSetCoverageToColorEnableNV: Option<vk::PFN_vkCmdSetCoverageToColorEnableNV>,
    pub(crate) vkCmdSetCoverageToColorLocationNV: Option<vk::PFN_vkCmdSetCoverageToColorLocationNV>,
    pub(crate) vkCmdSetCoverageModulationModeNV: Option<vk::PFN_vkCmdSetCoverageModulationModeNV>,
    pub(crate) vkCmdSetCoverageModulationTableEnableNV:
        Option<vk::PFN_vkCmdSetCoverageModulationTableEnableNV>,
    pub(crate) vkCmdSetCoverageModulationTableNV: Option<vk::PFN_vkCmdSetCoverageModulationTableNV>,
    pub(crate) vkCmdSetShadingRateImageEnableNV: Option<vk::PFN_vkCmdSetShadingRateImageEnableNV>,
    pub(crate) vkCmdSetRepresentativeFragmentTestEnableNV:
        Option<vk::PFN_vkCmdSetRepresentativeFragmentTestEnableNV>,
    pub(crate) vkCmdSetCoverageReductionModeNV: Option<vk::PFN_vkCmdSetCoverageReductionModeNV>,
    pub(crate) vkCreateTensorARM: Option<vk::PFN_vkCreateTensorARM>,
    pub(crate) vkDestroyTensorARM: Option<vk::PFN_vkDestroyTensorARM>,
    pub(crate) vkCreateTensorViewARM: Option<vk::PFN_vkCreateTensorViewARM>,
    pub(crate) vkDestroyTensorViewARM: Option<vk::PFN_vkDestroyTensorViewARM>,
    pub(crate) vkGetTensorMemoryRequirementsARM: Option<vk::PFN_vkGetTensorMemoryRequirementsARM>,
    pub(crate) vkBindTensorMemoryARM: Option<vk::PFN_vkBindTensorMemoryARM>,
    pub(crate) vkGetDeviceTensorMemoryRequirementsARM:
        Option<vk::PFN_vkGetDeviceTensorMemoryRequirementsARM>,
    pub(crate) vkCmdCopyTensorARM: Option<vk::PFN_vkCmdCopyTensorARM>,
    pub(crate) vkGetTensorOpaqueCaptureDescriptorDataARM:
        Option<vk::PFN_vkGetTensorOpaqueCaptureDescriptorDataARM>,
    pub(crate) vkGetTensorViewOpaqueCaptureDescriptorDataARM:
        Option<vk::PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM>,
    pub(crate) vkGetShaderModuleIdentifierEXT: Option<vk::PFN_vkGetShaderModuleIdentifierEXT>,
    pub(crate) vkGetShaderModuleCreateInfoIdentifierEXT:
        Option<vk::PFN_vkGetShaderModuleCreateInfoIdentifierEXT>,
    pub(crate) vkCreateOpticalFlowSessionNV: Option<vk::PFN_vkCreateOpticalFlowSessionNV>,
    pub(crate) vkDestroyOpticalFlowSessionNV: Option<vk::PFN_vkDestroyOpticalFlowSessionNV>,
    pub(crate) vkBindOpticalFlowSessionImageNV: Option<vk::PFN_vkBindOpticalFlowSessionImageNV>,
    pub(crate) vkCmdOpticalFlowExecuteNV: Option<vk::PFN_vkCmdOpticalFlowExecuteNV>,
    pub(crate) vkAntiLagUpdateAMD: Option<vk::PFN_vkAntiLagUpdateAMD>,
    pub(crate) vkCreateShadersEXT: Option<vk::PFN_vkCreateShadersEXT>,
    pub(crate) vkDestroyShaderEXT: Option<vk::PFN_vkDestroyShaderEXT>,
    pub(crate) vkGetShaderBinaryDataEXT: Option<vk::PFN_vkGetShaderBinaryDataEXT>,
    pub(crate) vkCmdBindShadersEXT: Option<vk::PFN_vkCmdBindShadersEXT>,
    pub(crate) vkCmdSetDepthClampRangeEXT: Option<vk::PFN_vkCmdSetDepthClampRangeEXT>,
    pub(crate) vkGetFramebufferTilePropertiesQCOM:
        Option<vk::PFN_vkGetFramebufferTilePropertiesQCOM>,
    pub(crate) vkGetDynamicRenderingTilePropertiesQCOM:
        Option<vk::PFN_vkGetDynamicRenderingTilePropertiesQCOM>,
    pub(crate) vkConvertCooperativeVectorMatrixNV:
        Option<vk::PFN_vkConvertCooperativeVectorMatrixNV>,
    pub(crate) vkCmdConvertCooperativeVectorMatrixNV:
        Option<vk::PFN_vkCmdConvertCooperativeVectorMatrixNV>,
    pub(crate) vkSetLatencySleepModeNV: Option<vk::PFN_vkSetLatencySleepModeNV>,
    pub(crate) vkLatencySleepNV: Option<vk::PFN_vkLatencySleepNV>,
    pub(crate) vkSetLatencyMarkerNV: Option<vk::PFN_vkSetLatencyMarkerNV>,
    pub(crate) vkGetLatencyTimingsNV: Option<vk::PFN_vkGetLatencyTimingsNV>,
    pub(crate) vkQueueNotifyOutOfBandNV: Option<vk::PFN_vkQueueNotifyOutOfBandNV>,
    pub(crate) vkCreateDataGraphPipelinesARM: Option<vk::PFN_vkCreateDataGraphPipelinesARM>,
    pub(crate) vkCreateDataGraphPipelineSessionARM:
        Option<vk::PFN_vkCreateDataGraphPipelineSessionARM>,
    pub(crate) vkGetDataGraphPipelineSessionBindPointRequirementsARM:
        Option<vk::PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM>,
    pub(crate) vkGetDataGraphPipelineSessionMemoryRequirementsARM:
        Option<vk::PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM>,
    pub(crate) vkBindDataGraphPipelineSessionMemoryARM:
        Option<vk::PFN_vkBindDataGraphPipelineSessionMemoryARM>,
    pub(crate) vkDestroyDataGraphPipelineSessionARM:
        Option<vk::PFN_vkDestroyDataGraphPipelineSessionARM>,
    pub(crate) vkCmdDispatchDataGraphARM: Option<vk::PFN_vkCmdDispatchDataGraphARM>,
    pub(crate) vkGetDataGraphPipelineAvailablePropertiesARM:
        Option<vk::PFN_vkGetDataGraphPipelineAvailablePropertiesARM>,
    pub(crate) vkGetDataGraphPipelinePropertiesARM:
        Option<vk::PFN_vkGetDataGraphPipelinePropertiesARM>,
    pub(crate) vkCmdSetAttachmentFeedbackLoopEnableEXT:
        Option<vk::PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT>,
    #[cfg(any(target_os = "nto", target_os = "qnx"))]
    pub(crate) vkGetScreenBufferPropertiesQNX: Option<vk::PFN_vkGetScreenBufferPropertiesQNX>,
    pub(crate) vkCmdBindTileMemoryQCOM: Option<vk::PFN_vkCmdBindTileMemoryQCOM>,
    pub(crate) vkCmdDecompressMemoryEXT: Option<vk::PFN_vkCmdDecompressMemoryEXT>,
    pub(crate) vkCmdDecompressMemoryIndirectCountEXT:
        Option<vk::PFN_vkCmdDecompressMemoryIndirectCountEXT>,
    pub(crate) vkCreateExternalComputeQueueNV: Option<vk::PFN_vkCreateExternalComputeQueueNV>,
    pub(crate) vkDestroyExternalComputeQueueNV: Option<vk::PFN_vkDestroyExternalComputeQueueNV>,
    pub(crate) vkGetExternalComputeQueueDataNV: Option<vk::PFN_vkGetExternalComputeQueueDataNV>,
    pub(crate) vkGetClusterAccelerationStructureBuildSizesNV:
        Option<vk::PFN_vkGetClusterAccelerationStructureBuildSizesNV>,
    pub(crate) vkCmdBuildClusterAccelerationStructureIndirectNV:
        Option<vk::PFN_vkCmdBuildClusterAccelerationStructureIndirectNV>,
    pub(crate) vkGetPartitionedAccelerationStructuresBuildSizesNV:
        Option<vk::PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV>,
    pub(crate) vkCmdBuildPartitionedAccelerationStructuresNV:
        Option<vk::PFN_vkCmdBuildPartitionedAccelerationStructuresNV>,
    pub(crate) vkGetGeneratedCommandsMemoryRequirementsEXT:
        Option<vk::PFN_vkGetGeneratedCommandsMemoryRequirementsEXT>,
    pub(crate) vkCmdPreprocessGeneratedCommandsEXT:
        Option<vk::PFN_vkCmdPreprocessGeneratedCommandsEXT>,
    pub(crate) vkCmdExecuteGeneratedCommandsEXT: Option<vk::PFN_vkCmdExecuteGeneratedCommandsEXT>,
    pub(crate) vkCreateIndirectCommandsLayoutEXT: Option<vk::PFN_vkCreateIndirectCommandsLayoutEXT>,
    pub(crate) vkDestroyIndirectCommandsLayoutEXT:
        Option<vk::PFN_vkDestroyIndirectCommandsLayoutEXT>,
    pub(crate) vkCreateIndirectExecutionSetEXT: Option<vk::PFN_vkCreateIndirectExecutionSetEXT>,
    pub(crate) vkDestroyIndirectExecutionSetEXT: Option<vk::PFN_vkDestroyIndirectExecutionSetEXT>,
    pub(crate) vkUpdateIndirectExecutionSetPipelineEXT:
        Option<vk::PFN_vkUpdateIndirectExecutionSetPipelineEXT>,
    pub(crate) vkUpdateIndirectExecutionSetShaderEXT:
        Option<vk::PFN_vkUpdateIndirectExecutionSetShaderEXT>,
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "visionos"
    ))]
    pub(crate) vkGetMemoryMetalHandleEXT: Option<vk::PFN_vkGetMemoryMetalHandleEXT>,
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "visionos"
    ))]
    pub(crate) vkGetMemoryMetalHandlePropertiesEXT:
        Option<vk::PFN_vkGetMemoryMetalHandlePropertiesEXT>,
    pub(crate) vkCreateShaderInstrumentationARM: Option<vk::PFN_vkCreateShaderInstrumentationARM>,
    pub(crate) vkDestroyShaderInstrumentationARM: Option<vk::PFN_vkDestroyShaderInstrumentationARM>,
    pub(crate) vkCmdBeginShaderInstrumentationARM:
        Option<vk::PFN_vkCmdBeginShaderInstrumentationARM>,
    pub(crate) vkCmdEndShaderInstrumentationARM: Option<vk::PFN_vkCmdEndShaderInstrumentationARM>,
    pub(crate) vkGetShaderInstrumentationValuesARM:
        Option<vk::PFN_vkGetShaderInstrumentationValuesARM>,
    pub(crate) vkClearShaderInstrumentationMetricsARM:
        Option<vk::PFN_vkClearShaderInstrumentationMetricsARM>,
    pub(crate) vkCmdEndRendering2EXT: Option<vk::PFN_vkCmdEndRendering2EXT>,
    pub(crate) vkCmdBeginCustomResolveEXT: Option<vk::PFN_vkCmdBeginCustomResolveEXT>,
    pub(crate) vkCmdSetComputeOccupancyPriorityNV:
        Option<vk::PFN_vkCmdSetComputeOccupancyPriorityNV>,
    pub(crate) vkCmdSetPrimitiveRestartIndexEXT: Option<vk::PFN_vkCmdSetPrimitiveRestartIndexEXT>,
    pub(crate) vkCreateAccelerationStructureKHR: Option<vk::PFN_vkCreateAccelerationStructureKHR>,
    pub(crate) vkDestroyAccelerationStructureKHR: Option<vk::PFN_vkDestroyAccelerationStructureKHR>,
    pub(crate) vkCmdBuildAccelerationStructuresKHR:
        Option<vk::PFN_vkCmdBuildAccelerationStructuresKHR>,
    pub(crate) vkCmdBuildAccelerationStructuresIndirectKHR:
        Option<vk::PFN_vkCmdBuildAccelerationStructuresIndirectKHR>,
    pub(crate) vkBuildAccelerationStructuresKHR: Option<vk::PFN_vkBuildAccelerationStructuresKHR>,
    pub(crate) vkCopyAccelerationStructureKHR: Option<vk::PFN_vkCopyAccelerationStructureKHR>,
    pub(crate) vkCopyAccelerationStructureToMemoryKHR:
        Option<vk::PFN_vkCopyAccelerationStructureToMemoryKHR>,
    pub(crate) vkCopyMemoryToAccelerationStructureKHR:
        Option<vk::PFN_vkCopyMemoryToAccelerationStructureKHR>,
    pub(crate) vkWriteAccelerationStructuresPropertiesKHR:
        Option<vk::PFN_vkWriteAccelerationStructuresPropertiesKHR>,
    pub(crate) vkCmdCopyAccelerationStructureKHR: Option<vk::PFN_vkCmdCopyAccelerationStructureKHR>,
    pub(crate) vkCmdCopyAccelerationStructureToMemoryKHR:
        Option<vk::PFN_vkCmdCopyAccelerationStructureToMemoryKHR>,
    pub(crate) vkCmdCopyMemoryToAccelerationStructureKHR:
        Option<vk::PFN_vkCmdCopyMemoryToAccelerationStructureKHR>,
    pub(crate) vkGetAccelerationStructureDeviceAddressKHR:
        Option<vk::PFN_vkGetAccelerationStructureDeviceAddressKHR>,
    pub(crate) vkCmdWriteAccelerationStructuresPropertiesKHR:
        Option<vk::PFN_vkCmdWriteAccelerationStructuresPropertiesKHR>,
    pub(crate) vkGetDeviceAccelerationStructureCompatibilityKHR:
        Option<vk::PFN_vkGetDeviceAccelerationStructureCompatibilityKHR>,
    pub(crate) vkGetAccelerationStructureBuildSizesKHR:
        Option<vk::PFN_vkGetAccelerationStructureBuildSizesKHR>,
    pub(crate) vkCmdTraceRaysKHR: Option<vk::PFN_vkCmdTraceRaysKHR>,
    pub(crate) vkCreateRayTracingPipelinesKHR: Option<vk::PFN_vkCreateRayTracingPipelinesKHR>,
    pub(crate) vkGetRayTracingCaptureReplayShaderGroupHandlesKHR:
        Option<vk::PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR>,
    pub(crate) vkCmdTraceRaysIndirectKHR: Option<vk::PFN_vkCmdTraceRaysIndirectKHR>,
    pub(crate) vkGetRayTracingShaderGroupStackSizeKHR:
        Option<vk::PFN_vkGetRayTracingShaderGroupStackSizeKHR>,
    pub(crate) vkCmdSetRayTracingPipelineStackSizeKHR:
        Option<vk::PFN_vkCmdSetRayTracingPipelineStackSizeKHR>,
    pub(crate) vkCmdDrawMeshTasksEXT: Option<vk::PFN_vkCmdDrawMeshTasksEXT>,
    pub(crate) vkCmdDrawMeshTasksIndirectEXT: Option<vk::PFN_vkCmdDrawMeshTasksIndirectEXT>,
    pub(crate) vkCmdDrawMeshTasksIndirectCountEXT:
        Option<vk::PFN_vkCmdDrawMeshTasksIndirectCountEXT>,
}
const _: () = assert!(core::mem::size_of::<LayerDeviceDispatchTable>() <= 65_535);
#[inline(never)]
pub(super) unsafe fn load_device_dispatch_fields(
    table: *mut u8,
    gdpa: vk::PFN_vkGetDeviceProcAddr,
    device: vk::VkDevice,
    loads: &[DeviceDispatchLoad],
) {
    for load in loads {
        let function = unsafe { gdpa(device, load.name.as_ptr()) };
        unsafe {
            table
                .add(usize::from(load.offset))
                .cast::<vk::PFN_vkVoidFunction>()
                .write(function);
        }
    }
}
impl LayerDeviceDispatchTable {
    pub(crate) unsafe fn load_into(
        table_ptr: *mut Self,
        gdpa: vk::PFN_vkGetDeviceProcAddr,
        device: vk::VkDevice,
    ) {
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).magic).write(DEVICE_DISPATCH_MAGIC);
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceProcAddr).write(Some(gdpa));
        }
        unsafe {
            load_device_dispatch_fields(table_ptr.cast(), gdpa, device, DEVICE_DISPATCH_LOADS);
        }
    }
    pub(crate) fn mask_unavailable(&mut self, mut available: impl FnMut(u16) -> bool) {
        let table = core::ptr::from_mut(self).cast::<u8>();
        for mask in DEVICE_DISPATCH_MASKS {
            if !available(mask.command_id) {
                unsafe {
                    table
                        .add(usize::from(mask.offset))
                        .write_bytes(0, core::mem::size_of::<vk::PFN_vkVoidFunction>());
                }
            }
        }
    }
}
#[repr(C)]
pub(crate) struct IcdDeviceTerminatorDispatchTable {
    pub(crate) vkDestroyDevice: Option<vk::PFN_vkDestroyDevice>,
    pub(crate) vkCreateSwapchainKHR: Option<vk::PFN_vkCreateSwapchainKHR>,
    pub(crate) vkGetDeviceGroupSurfacePresentModesKHR:
        Option<vk::PFN_vkGetDeviceGroupSurfacePresentModesKHR>,
    pub(crate) vkCreateSharedSwapchainsKHR: Option<vk::PFN_vkCreateSharedSwapchainsKHR>,
    pub(crate) vkDebugMarkerSetObjectTagEXT: Option<vk::PFN_vkDebugMarkerSetObjectTagEXT>,
    pub(crate) vkDebugMarkerSetObjectNameEXT: Option<vk::PFN_vkDebugMarkerSetObjectNameEXT>,
    pub(crate) vkSetDebugUtilsObjectNameEXT: Option<vk::PFN_vkSetDebugUtilsObjectNameEXT>,
    pub(crate) vkSetDebugUtilsObjectTagEXT: Option<vk::PFN_vkSetDebugUtilsObjectTagEXT>,
    pub(crate) vkQueueBeginDebugUtilsLabelEXT: Option<vk::PFN_vkQueueBeginDebugUtilsLabelEXT>,
    pub(crate) vkQueueEndDebugUtilsLabelEXT: Option<vk::PFN_vkQueueEndDebugUtilsLabelEXT>,
    pub(crate) vkQueueInsertDebugUtilsLabelEXT: Option<vk::PFN_vkQueueInsertDebugUtilsLabelEXT>,
    pub(crate) vkCmdBeginDebugUtilsLabelEXT: Option<vk::PFN_vkCmdBeginDebugUtilsLabelEXT>,
    pub(crate) vkCmdEndDebugUtilsLabelEXT: Option<vk::PFN_vkCmdEndDebugUtilsLabelEXT>,
    pub(crate) vkCmdInsertDebugUtilsLabelEXT: Option<vk::PFN_vkCmdInsertDebugUtilsLabelEXT>,
    #[cfg(target_os = "windows")]
    pub(crate) vkGetDeviceGroupSurfacePresentModes2EXT:
        Option<vk::PFN_vkGetDeviceGroupSurfacePresentModes2EXT>,
}
const _: () = assert!(core::mem::size_of::<IcdDeviceTerminatorDispatchTable>() <= 65_535);
pub(super) static ICD_DEVICE_DISPATCH_LOADS: &[IcdDeviceDispatchLoad] = &[
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkDestroyDevice
        )),
        command_id: VK_DESTROY_DEVICE_COMMAND_ID,
        name: c"vkDestroyDevice",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkCreateSwapchainKHR
        )),
        command_id: VK_CREATE_SWAPCHAIN_KHR_COMMAND_ID,
        name: c"vkCreateSwapchainKHR",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkGetDeviceGroupSurfacePresentModesKHR
        )),
        command_id: VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR_COMMAND_ID,
        name: c"vkGetDeviceGroupSurfacePresentModesKHR",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkCreateSharedSwapchainsKHR
        )),
        command_id: VK_CREATE_SHARED_SWAPCHAINS_KHR_COMMAND_ID,
        name: c"vkCreateSharedSwapchainsKHR",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkDebugMarkerSetObjectTagEXT
        )),
        command_id: VK_DEBUG_MARKER_SET_OBJECT_TAG_EXT_COMMAND_ID,
        name: c"vkDebugMarkerSetObjectTagEXT",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkDebugMarkerSetObjectNameEXT
        )),
        command_id: VK_DEBUG_MARKER_SET_OBJECT_NAME_EXT_COMMAND_ID,
        name: c"vkDebugMarkerSetObjectNameEXT",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkSetDebugUtilsObjectNameEXT
        )),
        command_id: VK_SET_DEBUG_UTILS_OBJECT_NAME_EXT_COMMAND_ID,
        name: c"vkSetDebugUtilsObjectNameEXT",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkSetDebugUtilsObjectTagEXT
        )),
        command_id: VK_SET_DEBUG_UTILS_OBJECT_TAG_EXT_COMMAND_ID,
        name: c"vkSetDebugUtilsObjectTagEXT",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkQueueBeginDebugUtilsLabelEXT
        )),
        command_id: VK_QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT_COMMAND_ID,
        name: c"vkQueueBeginDebugUtilsLabelEXT",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkQueueEndDebugUtilsLabelEXT
        )),
        command_id: VK_QUEUE_END_DEBUG_UTILS_LABEL_EXT_COMMAND_ID,
        name: c"vkQueueEndDebugUtilsLabelEXT",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkQueueInsertDebugUtilsLabelEXT
        )),
        command_id: VK_QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT_COMMAND_ID,
        name: c"vkQueueInsertDebugUtilsLabelEXT",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkCmdBeginDebugUtilsLabelEXT
        )),
        command_id: VK_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT_COMMAND_ID,
        name: c"vkCmdBeginDebugUtilsLabelEXT",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkCmdEndDebugUtilsLabelEXT
        )),
        command_id: VK_CMD_END_DEBUG_UTILS_LABEL_EXT_COMMAND_ID,
        name: c"vkCmdEndDebugUtilsLabelEXT",
    },
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkCmdInsertDebugUtilsLabelEXT
        )),
        command_id: VK_CMD_INSERT_DEBUG_UTILS_LABEL_EXT_COMMAND_ID,
        name: c"vkCmdInsertDebugUtilsLabelEXT",
    },
    #[cfg(target_os = "windows")]
    IcdDeviceDispatchLoad {
        offset: dispatch_offset(core::mem::offset_of!(
            IcdDeviceTerminatorDispatchTable,
            vkGetDeviceGroupSurfacePresentModes2EXT
        )),
        command_id: VK_GET_DEVICE_GROUP_SURFACE_PRESENT_MODES2EXT_COMMAND_ID,
        name: c"vkGetDeviceGroupSurfacePresentModes2EXT",
    },
];
#[inline(never)]
pub(super) unsafe fn load_icd_device_dispatch_fields(
    table: *mut u8,
    gdpa: vk::PFN_vkGetDeviceProcAddr,
    device: vk::VkDevice,
    loads: &[IcdDeviceDispatchLoad],
    available: &mut impl FnMut(u16) -> bool,
) {
    for load in loads {
        let function = if available(load.command_id) {
            unsafe { gdpa(device, load.name.as_ptr()) }
        } else {
            None
        };
        unsafe {
            table
                .add(usize::from(load.offset))
                .cast::<vk::PFN_vkVoidFunction>()
                .write(function);
        }
    }
}
impl IcdDeviceTerminatorDispatchTable {
    pub(crate) unsafe fn load(
        gdpa: vk::PFN_vkGetDeviceProcAddr,
        device: vk::VkDevice,
        mut available: impl FnMut(u16) -> bool,
    ) -> Self {
        let mut table = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            load_icd_device_dispatch_fields(
                table.as_mut_ptr().cast(),
                gdpa,
                device,
                ICD_DEVICE_DISPATCH_LOADS,
                &mut available,
            );
            table.assume_init()
        }
    }
}
