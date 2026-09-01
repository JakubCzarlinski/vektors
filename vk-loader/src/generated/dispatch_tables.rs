// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.

use super::commands::DEVICE_DISPATCH_MASKS;
use crate::CStr;
use crate::DEVICE_DISPATCH_MAGIC;
use crate::load_typed;
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
impl InstanceDispatchTable {
    #[allow(clippy::too_many_lines)]
    pub(crate) unsafe fn load_into(
        table: *mut Self,
        gipa: vk::PFN_vkGetInstanceProcAddr,
        handle: vk::VkInstance,
    ) {
        unsafe {
            core::ptr::addr_of_mut!((*table).vkAcquireDrmDisplayEXT)
                .write(load_typed(gipa(handle, c"vkAcquireDrmDisplayEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkAcquireWinrtDisplayNV).write(load_typed(gipa(
                handle,
                c"vkAcquireWinrtDisplayNV".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkAcquireXlibDisplayEXT).write(load_typed(gipa(
                handle,
                c"vkAcquireXlibDisplayEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateAndroidSurfaceKHR).write(load_typed(gipa(
                handle,
                c"vkCreateAndroidSurfaceKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateDebugReportCallbackEXT).write(load_typed(
                gipa(handle, c"vkCreateDebugReportCallbackEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateDebugUtilsMessengerEXT).write(load_typed(
                gipa(handle, c"vkCreateDebugUtilsMessengerEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateDevice)
                .write(load_typed(gipa(handle, c"vkCreateDevice".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateDirectFBSurfaceEXT).write(load_typed(gipa(
                handle,
                c"vkCreateDirectFBSurfaceEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateDisplayModeKHR)
                .write(load_typed(gipa(handle, c"vkCreateDisplayModeKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateDisplayPlaneSurfaceKHR).write(load_typed(
                gipa(handle, c"vkCreateDisplayPlaneSurfaceKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateHeadlessSurfaceEXT).write(load_typed(gipa(
                handle,
                c"vkCreateHeadlessSurfaceEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateIOSSurfaceMVK)
                .write(load_typed(gipa(handle, c"vkCreateIOSSurfaceMVK".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateImagePipeSurfaceFUCHSIA).write(load_typed(
                gipa(handle, c"vkCreateImagePipeSurfaceFUCHSIA".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateMacOSSurfaceMVK).write(load_typed(gipa(
                handle,
                c"vkCreateMacOSSurfaceMVK".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateMetalSurfaceEXT).write(load_typed(gipa(
                handle,
                c"vkCreateMetalSurfaceEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateScreenSurfaceQNX).write(load_typed(gipa(
                handle,
                c"vkCreateScreenSurfaceQNX".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateStreamDescriptorSurfaceGGP).write(load_typed(
                gipa(handle, c"vkCreateStreamDescriptorSurfaceGGP".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateSurfaceOHOS)
                .write(load_typed(gipa(handle, c"vkCreateSurfaceOHOS".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateUbmSurfaceSEC)
                .write(load_typed(gipa(handle, c"vkCreateUbmSurfaceSEC".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateViSurfaceNN)
                .write(load_typed(gipa(handle, c"vkCreateViSurfaceNN".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateWaylandSurfaceKHR).write(load_typed(gipa(
                handle,
                c"vkCreateWaylandSurfaceKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateWin32SurfaceKHR).write(load_typed(gipa(
                handle,
                c"vkCreateWin32SurfaceKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateXcbSurfaceKHR)
                .write(load_typed(gipa(handle, c"vkCreateXcbSurfaceKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkCreateXlibSurfaceKHR)
                .write(load_typed(gipa(handle, c"vkCreateXlibSurfaceKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkDebugReportMessageEXT).write(load_typed(gipa(
                handle,
                c"vkDebugReportMessageEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkDestroyDebugReportCallbackEXT).write(load_typed(
                gipa(handle, c"vkDestroyDebugReportCallbackEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkDestroyDebugUtilsMessengerEXT).write(load_typed(
                gipa(handle, c"vkDestroyDebugUtilsMessengerEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkDestroyInstance)
                .write(load_typed(gipa(handle, c"vkDestroyInstance".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkDestroySurfaceKHR)
                .write(load_typed(gipa(handle, c"vkDestroySurfaceKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkEnumerateDeviceExtensionProperties).write(
                load_typed(gipa(
                    handle,
                    c"vkEnumerateDeviceExtensionProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkEnumerateDeviceLayerProperties).write(load_typed(
                gipa(handle, c"vkEnumerateDeviceLayerProperties".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkEnumeratePhysicalDeviceGroups).write(load_typed(
                gipa(handle, c"vkEnumeratePhysicalDeviceGroups".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkEnumeratePhysicalDeviceGroupsKHR).write(load_typed(
                gipa(handle, c"vkEnumeratePhysicalDeviceGroupsKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table).vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM
            )
            .write(load_typed(gipa(
                handle,
                c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table).vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR
            )
            .write(load_typed(gipa(
                handle,
                c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table).vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM
            )
            .write(load_typed(gipa(
                handle,
                c"vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkEnumeratePhysicalDevices).write(load_typed(gipa(
                handle,
                c"vkEnumeratePhysicalDevices".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetDeviceProcAddr)
                .write(load_typed(gipa(handle, c"vkGetDeviceProcAddr".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetDisplayModeProperties2KHR).write(load_typed(
                gipa(handle, c"vkGetDisplayModeProperties2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetDisplayModePropertiesKHR).write(load_typed(
                gipa(handle, c"vkGetDisplayModePropertiesKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetDisplayPlaneCapabilities2KHR).write(load_typed(
                gipa(handle, c"vkGetDisplayPlaneCapabilities2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetDisplayPlaneCapabilitiesKHR).write(load_typed(
                gipa(handle, c"vkGetDisplayPlaneCapabilitiesKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetDisplayPlaneSupportedDisplaysKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetDisplayPlaneSupportedDisplaysKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetDrmDisplayEXT)
                .write(load_typed(gipa(handle, c"vkGetDrmDisplayEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceCalibrateableTimeDomainsEXT).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceCalibrateableTimeDomainsKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table).vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV
            )
            .write(load_typed(gipa(
                handle,
                c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceCooperativeMatrixProperties2EXT)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceCooperativeMatrixProperties2EXT".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceCooperativeMatrixPropertiesNV)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceCooperativeVectorPropertiesNV)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceDescriptorSizeEXT).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceDescriptorSizeEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceDirectFBPresentationSupportEXT)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceDisplayPlaneProperties2KHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceDisplayPlanePropertiesKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceDisplayProperties2KHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceDisplayProperties2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceDisplayPropertiesKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceDisplayPropertiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceExternalBufferProperties).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceExternalBufferProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceExternalBufferPropertiesKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceExternalBufferPropertiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceExternalFenceProperties).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceExternalFenceProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceExternalFencePropertiesKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceExternalFencePropertiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceExternalImageFormatPropertiesNV)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceExternalSemaphoreProperties).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceExternalSemaphoreProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceExternalSemaphorePropertiesKHR)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceExternalTensorPropertiesARM).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceExternalTensorPropertiesARM".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceFeatures).write(load_typed(gipa(
                handle,
                c"vkGetPhysicalDeviceFeatures".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceFeatures2).write(load_typed(gipa(
                handle,
                c"vkGetPhysicalDeviceFeatures2".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceFeatures2KHR).write(load_typed(
                gipa(handle, c"vkGetPhysicalDeviceFeatures2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceFormatProperties).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceFormatProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceFormatProperties2).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceFormatProperties2".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceFormatProperties2KHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceFormatProperties2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceFragmentShadingRatesKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceFragmentShadingRatesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceImageFormatProperties).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceImageFormatProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceImageFormatProperties2).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceImageFormatProperties2".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceImageFormatProperties2KHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceImageFormatProperties2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceMemoryProperties).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceMemoryProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceMemoryProperties2).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceMemoryProperties2".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceMemoryProperties2KHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceMemoryProperties2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceMultisamplePropertiesEXT).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceMultisamplePropertiesEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceOpticalFlowImageFormatsNV).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDevicePresentRectanglesKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDevicePresentRectanglesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceProperties).write(load_typed(
                gipa(handle, c"vkGetPhysicalDeviceProperties".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceProperties2).write(load_typed(
                gipa(handle, c"vkGetPhysicalDeviceProperties2".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceProperties2KHR).write(load_typed(
                gipa(handle, c"vkGetPhysicalDeviceProperties2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table).vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM
            )
            .write(load_typed(gipa(
                handle,
                c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table).vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM
            )
            .write(load_typed(gipa(
                handle,
                c"vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table).vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM
            )
            .write(load_typed(gipa(
                handle,
                c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table).vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR
            )
            .write(load_typed(gipa(
                handle,
                c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceQueueFamilyProperties).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceQueueFamilyProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceQueueFamilyProperties2).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceQueueFamilyProperties2".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceQueueFamilyProperties2KHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceQueueFamilyProperties2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceScreenPresentationSupportQNX)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceScreenPresentationSupportQNX".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceSparseImageFormatProperties).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceSparseImageFormatProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceSparseImageFormatProperties2)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceSparseImageFormatProperties2".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceSparseImageFormatProperties2KHR)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table).vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV
            )
            .write(load_typed(gipa(
                handle,
                c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceSurfaceCapabilities2EXT).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceSurfaceCapabilities2EXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceSurfaceCapabilities2KHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceSurfaceCapabilities2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceSurfaceCapabilitiesKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceSurfaceFormats2KHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceSurfaceFormats2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceSurfaceFormatsKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceSurfaceFormatsKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceSurfacePresentModes2EXT).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceSurfacePresentModes2EXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceSurfacePresentModesKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceSurfacePresentModesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceSurfaceSupportKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceSurfaceSupportKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceToolProperties).write(load_typed(
                gipa(handle, c"vkGetPhysicalDeviceToolProperties".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceToolPropertiesEXT).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceToolPropertiesEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceUbmPresentationSupportSEC).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceUbmPresentationSupportSEC".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceVideoCapabilitiesKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceVideoCapabilitiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table).vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR
            )
            .write(load_typed(gipa(
                handle,
                c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceVideoFormatPropertiesKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceVideoFormatPropertiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceWaylandPresentationSupportKHR)
                .write(load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceWaylandPresentationSupportKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceWin32PresentationSupportKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceWin32PresentationSupportKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceXcbPresentationSupportKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceXcbPresentationSupportKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetPhysicalDeviceXlibPresentationSupportKHR).write(
                load_typed(gipa(
                    handle,
                    c"vkGetPhysicalDeviceXlibPresentationSupportKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetRandROutputDisplayEXT).write(load_typed(gipa(
                handle,
                c"vkGetRandROutputDisplayEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkGetWinrtDisplayNV)
                .write(load_typed(gipa(handle, c"vkGetWinrtDisplayNV".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkReleaseDisplayEXT)
                .write(load_typed(gipa(handle, c"vkReleaseDisplayEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table).vkSubmitDebugUtilsMessageEXT).write(load_typed(gipa(
                handle,
                c"vkSubmitDebugUtilsMessageEXT".as_ptr(),
            )));
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
impl LayerInstanceDispatchTable {
    #[allow(clippy::too_many_lines)]
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
            core::ptr::addr_of_mut!((*table_ptr).vkCreateInstance)
                .write(load_typed(gipa(instance, c"vkCreateInstance".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyInstance)
                .write(load_typed(gipa(instance, c"vkDestroyInstance".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkEnumeratePhysicalDevices).write(load_typed(
                gipa(instance, c"vkEnumeratePhysicalDevices".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceFeatures).write(load_typed(
                gipa(instance, c"vkGetPhysicalDeviceFeatures".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceFormatProperties).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceFormatProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceImageFormatProperties).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceImageFormatProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceProperties).write(load_typed(
                gipa(instance, c"vkGetPhysicalDeviceProperties".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceQueueFamilyProperties).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceQueueFamilyProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceMemoryProperties).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceMemoryProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetInstanceProcAddr).write(Some(gipa));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDevice)
                .write(load_typed(gipa(instance, c"vkCreateDevice".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkEnumerateInstanceExtensionProperties).write(
                load_typed(gipa(
                    instance,
                    c"vkEnumerateInstanceExtensionProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkEnumerateDeviceExtensionProperties).write(
                load_typed(gipa(
                    instance,
                    c"vkEnumerateDeviceExtensionProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkEnumerateInstanceLayerProperties).write(
                load_typed(gipa(
                    instance,
                    c"vkEnumerateInstanceLayerProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkEnumerateDeviceLayerProperties).write(
                load_typed(gipa(instance, c"vkEnumerateDeviceLayerProperties".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceSparseImageFormatProperties)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceSparseImageFormatProperties".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkEnumerateInstanceVersion).write(load_typed(
                gipa(instance, c"vkEnumerateInstanceVersion".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkEnumeratePhysicalDeviceGroups).write(
                load_typed(gipa(instance, c"vkEnumeratePhysicalDeviceGroups".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceFeatures2).write(load_typed(
                gipa(instance, c"vkGetPhysicalDeviceFeatures2".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceProperties2).write(load_typed(
                gipa(instance, c"vkGetPhysicalDeviceProperties2".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceFormatProperties2).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceFormatProperties2".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceImageFormatProperties2).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceImageFormatProperties2".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceQueueFamilyProperties2).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceQueueFamilyProperties2".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceMemoryProperties2).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceMemoryProperties2".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceSparseImageFormatProperties2)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceSparseImageFormatProperties2".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceExternalBufferProperties)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceExternalBufferProperties".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceExternalFenceProperties).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceExternalFenceProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceExternalSemaphoreProperties)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceExternalSemaphoreProperties".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceToolProperties).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceToolProperties".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroySurfaceKHR)
                .write(load_typed(gipa(instance, c"vkDestroySurfaceKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceSurfaceSupportKHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceSurfaceSupportKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceSurfaceCapabilitiesKHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceSurfaceFormatsKHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceSurfaceFormatsKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceSurfacePresentModesKHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceSurfacePresentModesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDevicePresentRectanglesKHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDevicePresentRectanglesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceDisplayPropertiesKHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceDisplayPropertiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceDisplayPlanePropertiesKHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDisplayPlaneSupportedDisplaysKHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetDisplayPlaneSupportedDisplaysKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDisplayModePropertiesKHR).write(load_typed(
                gipa(instance, c"vkGetDisplayModePropertiesKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDisplayModeKHR).write(load_typed(gipa(
                instance,
                c"vkCreateDisplayModeKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDisplayPlaneCapabilitiesKHR).write(
                load_typed(gipa(instance, c"vkGetDisplayPlaneCapabilitiesKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDisplayPlaneSurfaceKHR).write(load_typed(
                gipa(instance, c"vkCreateDisplayPlaneSurfaceKHR".as_ptr()),
            ));
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
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateXlibSurfaceKHR).write(load_typed(gipa(
                instance,
                c"vkCreateXlibSurfaceKHR".as_ptr(),
            )));
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
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceXlibPresentationSupportKHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceXlibPresentationSupportKHR".as_ptr(),
                )));
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
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateXcbSurfaceKHR).write(load_typed(gipa(
                instance,
                c"vkCreateXcbSurfaceKHR".as_ptr(),
            )));
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
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceXcbPresentationSupportKHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceXcbPresentationSupportKHR".as_ptr(),
                )));
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
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateWaylandSurfaceKHR).write(load_typed(
                gipa(instance, c"vkCreateWaylandSurfaceKHR".as_ptr()),
            ));
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
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceWaylandPresentationSupportKHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceWaylandPresentationSupportKHR".as_ptr(),
                )));
        }
        #[cfg(target_os = "android")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateAndroidSurfaceKHR).write(load_typed(
                gipa(instance, c"vkCreateAndroidSurfaceKHR".as_ptr()),
            ));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateWin32SurfaceKHR).write(load_typed(gipa(
                instance,
                c"vkCreateWin32SurfaceKHR".as_ptr(),
            )));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceWin32PresentationSupportKHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceWin32PresentationSupportKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceVideoCapabilitiesKHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceVideoCapabilitiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceVideoFormatPropertiesKHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceVideoFormatPropertiesKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceFeatures2KHR).write(
                load_typed(gipa(instance, c"vkGetPhysicalDeviceFeatures2KHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceProperties2KHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceProperties2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceFormatProperties2KHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceFormatProperties2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceImageFormatProperties2KHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceImageFormatProperties2KHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceQueueFamilyProperties2KHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceQueueFamilyProperties2KHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceMemoryProperties2KHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceMemoryProperties2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPhysicalDeviceSparseImageFormatProperties2KHR
            )
            .write(load_typed(gipa(
                instance,
                c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkEnumeratePhysicalDeviceGroupsKHR).write(
                load_typed(gipa(
                    instance,
                    c"vkEnumeratePhysicalDeviceGroupsKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceExternalBufferPropertiesKHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceExternalBufferPropertiesKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceExternalSemaphorePropertiesKHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceExternalFencePropertiesKHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceExternalFencePropertiesKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR
            )
            .write(load_typed(gipa(
                instance,
                c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR
            )
            .write(load_typed(gipa(
                instance,
                c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceSurfaceCapabilities2KHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceSurfaceCapabilities2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceSurfaceFormats2KHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceSurfaceFormats2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceDisplayProperties2KHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceDisplayProperties2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceDisplayPlaneProperties2KHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDisplayModeProperties2KHR).write(load_typed(
                gipa(instance, c"vkGetDisplayModeProperties2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDisplayPlaneCapabilities2KHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetDisplayPlaneCapabilities2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceFragmentShadingRatesKHR).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceFragmentShadingRatesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR
            )
            .write(load_typed(gipa(
                instance,
                c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceCalibrateableTimeDomainsKHR)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDebugReportCallbackEXT).write(load_typed(
                gipa(instance, c"vkCreateDebugReportCallbackEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyDebugReportCallbackEXT).write(
                load_typed(gipa(instance, c"vkDestroyDebugReportCallbackEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDebugReportMessageEXT).write(load_typed(gipa(
                instance,
                c"vkDebugReportMessageEXT".as_ptr(),
            )));
        }
        #[cfg(feature = "platform-ggp")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateStreamDescriptorSurfaceGGP).write(
                load_typed(gipa(
                    instance,
                    c"vkCreateStreamDescriptorSurfaceGGP".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPhysicalDeviceExternalImageFormatPropertiesNV
            )
            .write(load_typed(gipa(
                instance,
                c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV".as_ptr(),
            )));
        }
        #[cfg(feature = "platform-vi")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateViSurfaceNN)
                .write(load_typed(gipa(instance, c"vkCreateViSurfaceNN".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkReleaseDisplayEXT)
                .write(load_typed(gipa(instance, c"vkReleaseDisplayEXT".as_ptr())));
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
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAcquireXlibDisplayEXT).write(load_typed(gipa(
                instance,
                c"vkAcquireXlibDisplayEXT".as_ptr(),
            )));
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
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetRandROutputDisplayEXT).write(load_typed(
                gipa(instance, c"vkGetRandROutputDisplayEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceSurfaceCapabilities2EXT).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceSurfaceCapabilities2EXT".as_ptr(),
                )),
            );
        }
        #[cfg(target_os = "ios")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateIOSSurfaceMVK).write(load_typed(gipa(
                instance,
                c"vkCreateIOSSurfaceMVK".as_ptr(),
            )));
        }
        #[cfg(target_os = "macos")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateMacOSSurfaceMVK).write(load_typed(gipa(
                instance,
                c"vkCreateMacOSSurfaceMVK".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDebugUtilsMessengerEXT).write(load_typed(
                gipa(instance, c"vkCreateDebugUtilsMessengerEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyDebugUtilsMessengerEXT).write(
                load_typed(gipa(instance, c"vkDestroyDebugUtilsMessengerEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSubmitDebugUtilsMessageEXT).write(load_typed(
                gipa(instance, c"vkSubmitDebugUtilsMessageEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceDescriptorSizeEXT).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceDescriptorSizeEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceMultisamplePropertiesEXT)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceMultisamplePropertiesEXT".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceCalibrateableTimeDomainsEXT)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT".as_ptr(),
                )));
        }
        #[cfg(target_os = "fuchsia")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateImagePipeSurfaceFUCHSIA).write(
                load_typed(gipa(instance, c"vkCreateImagePipeSurfaceFUCHSIA".as_ptr())),
            );
        }
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateMetalSurfaceEXT).write(load_typed(gipa(
                instance,
                c"vkCreateMetalSurfaceEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceToolPropertiesEXT).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceToolPropertiesEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceCooperativeMatrixPropertiesNV)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV
            )
            .write(load_typed(gipa(
                instance,
                c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV".as_ptr(),
            )));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceSurfacePresentModes2EXT).write(
                load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceSurfacePresentModes2EXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateHeadlessSurfaceEXT).write(load_typed(
                gipa(instance, c"vkCreateHeadlessSurfaceEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAcquireDrmDisplayEXT).write(load_typed(gipa(
                instance,
                c"vkAcquireDrmDisplayEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDrmDisplayEXT)
                .write(load_typed(gipa(instance, c"vkGetDrmDisplayEXT".as_ptr())));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAcquireWinrtDisplayNV).write(load_typed(gipa(
                instance,
                c"vkAcquireWinrtDisplayNV".as_ptr(),
            )));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetWinrtDisplayNV)
                .write(load_typed(gipa(instance, c"vkGetWinrtDisplayNV".as_ptr())));
        }
        #[cfg(feature = "wsi-directfb")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDirectFBSurfaceEXT).write(load_typed(
                gipa(instance, c"vkCreateDirectFBSurfaceEXT".as_ptr()),
            ));
        }
        #[cfg(feature = "wsi-directfb")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceDirectFBPresentationSupportEXT)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT".as_ptr(),
                )));
        }
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateScreenSurfaceQNX).write(load_typed(gipa(
                instance,
                c"vkCreateScreenSurfaceQNX".as_ptr(),
            )));
        }
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceScreenPresentationSupportQNX)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceScreenPresentationSupportQNX".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceExternalTensorPropertiesARM)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceExternalTensorPropertiesARM".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceOpticalFlowImageFormatsNV)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceCooperativeVectorPropertiesNV)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM
            )
            .write(load_typed(gipa(
                instance,
                c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM
            )
            .write(load_typed(gipa(
                instance,
                c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM
            )
            .write(load_typed(gipa(
                instance,
                c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM".as_ptr(),
            )));
        }
        #[cfg(target_env = "ohos")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateSurfaceOHOS)
                .write(load_typed(gipa(instance, c"vkCreateSurfaceOHOS".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV
            )
            .write(load_typed(gipa(
                instance,
                c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM
            )
            .write(load_typed(gipa(
                instance,
                c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM
            )
            .write(load_typed(gipa(
                instance,
                c"vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM
            )
            .write(load_typed(gipa(
                instance,
                c"vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPhysicalDeviceCooperativeMatrixProperties2EXT
            )
            .write(load_typed(gipa(
                instance,
                c"vkGetPhysicalDeviceCooperativeMatrixProperties2EXT".as_ptr(),
            )));
        }
        #[cfg(feature = "platform-ubm")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateUbmSurfaceSEC).write(load_typed(gipa(
                instance,
                c"vkCreateUbmSurfaceSEC".as_ptr(),
            )));
        }
        #[cfg(feature = "platform-ubm")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPhysicalDeviceUbmPresentationSupportSEC)
                .write(load_typed(gipa(
                    instance,
                    c"vkGetPhysicalDeviceUbmPresentationSupportSEC".as_ptr(),
                )));
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
impl LayerDeviceDispatchTable {
    #[allow(clippy::too_many_lines)]
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
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyDevice)
                .write(load_typed(gdpa(device, c"vkDestroyDevice".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceQueue)
                .write(load_typed(gdpa(device, c"vkGetDeviceQueue".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueSubmit)
                .write(load_typed(gdpa(device, c"vkQueueSubmit".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueWaitIdle)
                .write(load_typed(gdpa(device, c"vkQueueWaitIdle".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDeviceWaitIdle)
                .write(load_typed(gdpa(device, c"vkDeviceWaitIdle".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAllocateMemory)
                .write(load_typed(gdpa(device, c"vkAllocateMemory".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkFreeMemory)
                .write(load_typed(gdpa(device, c"vkFreeMemory".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkMapMemory)
                .write(load_typed(gdpa(device, c"vkMapMemory".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkUnmapMemory)
                .write(load_typed(gdpa(device, c"vkUnmapMemory".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkFlushMappedMemoryRanges).write(load_typed(
                gdpa(device, c"vkFlushMappedMemoryRanges".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkInvalidateMappedMemoryRanges).write(load_typed(
                gdpa(device, c"vkInvalidateMappedMemoryRanges".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceMemoryCommitment).write(load_typed(
                gdpa(device, c"vkGetDeviceMemoryCommitment".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBindBufferMemory)
                .write(load_typed(gdpa(device, c"vkBindBufferMemory".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBindImageMemory)
                .write(load_typed(gdpa(device, c"vkBindImageMemory".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetBufferMemoryRequirements).write(load_typed(
                gdpa(device, c"vkGetBufferMemoryRequirements".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageMemoryRequirements).write(load_typed(
                gdpa(device, c"vkGetImageMemoryRequirements".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageSparseMemoryRequirements).write(
                load_typed(gdpa(device, c"vkGetImageSparseMemoryRequirements".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueBindSparse)
                .write(load_typed(gdpa(device, c"vkQueueBindSparse".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateFence)
                .write(load_typed(gdpa(device, c"vkCreateFence".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyFence)
                .write(load_typed(gdpa(device, c"vkDestroyFence".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkResetFences)
                .write(load_typed(gdpa(device, c"vkResetFences".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetFenceStatus)
                .write(load_typed(gdpa(device, c"vkGetFenceStatus".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkWaitForFences)
                .write(load_typed(gdpa(device, c"vkWaitForFences".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateSemaphore)
                .write(load_typed(gdpa(device, c"vkCreateSemaphore".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroySemaphore)
                .write(load_typed(gdpa(device, c"vkDestroySemaphore".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateQueryPool)
                .write(load_typed(gdpa(device, c"vkCreateQueryPool".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyQueryPool)
                .write(load_typed(gdpa(device, c"vkDestroyQueryPool".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetQueryPoolResults)
                .write(load_typed(gdpa(device, c"vkGetQueryPoolResults".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateBuffer)
                .write(load_typed(gdpa(device, c"vkCreateBuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyBuffer)
                .write(load_typed(gdpa(device, c"vkDestroyBuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateImage)
                .write(load_typed(gdpa(device, c"vkCreateImage".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyImage)
                .write(load_typed(gdpa(device, c"vkDestroyImage".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageSubresourceLayout).write(load_typed(
                gdpa(device, c"vkGetImageSubresourceLayout".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateImageView)
                .write(load_typed(gdpa(device, c"vkCreateImageView".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyImageView)
                .write(load_typed(gdpa(device, c"vkDestroyImageView".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateCommandPool)
                .write(load_typed(gdpa(device, c"vkCreateCommandPool".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyCommandPool)
                .write(load_typed(gdpa(device, c"vkDestroyCommandPool".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkResetCommandPool)
                .write(load_typed(gdpa(device, c"vkResetCommandPool".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAllocateCommandBuffers).write(load_typed(gdpa(
                device,
                c"vkAllocateCommandBuffers".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkFreeCommandBuffers)
                .write(load_typed(gdpa(device, c"vkFreeCommandBuffers".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBeginCommandBuffer)
                .write(load_typed(gdpa(device, c"vkBeginCommandBuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkEndCommandBuffer)
                .write(load_typed(gdpa(device, c"vkEndCommandBuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkResetCommandBuffer)
                .write(load_typed(gdpa(device, c"vkResetCommandBuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyBuffer)
                .write(load_typed(gdpa(device, c"vkCmdCopyBuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyImage)
                .write(load_typed(gdpa(device, c"vkCmdCopyImage".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyBufferToImage)
                .write(load_typed(gdpa(device, c"vkCmdCopyBufferToImage".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyImageToBuffer)
                .write(load_typed(gdpa(device, c"vkCmdCopyImageToBuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdUpdateBuffer)
                .write(load_typed(gdpa(device, c"vkCmdUpdateBuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdFillBuffer)
                .write(load_typed(gdpa(device, c"vkCmdFillBuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPipelineBarrier)
                .write(load_typed(gdpa(device, c"vkCmdPipelineBarrier".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginQuery)
                .write(load_typed(gdpa(device, c"vkCmdBeginQuery".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndQuery)
                .write(load_typed(gdpa(device, c"vkCmdEndQuery".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdResetQueryPool)
                .write(load_typed(gdpa(device, c"vkCmdResetQueryPool".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWriteTimestamp)
                .write(load_typed(gdpa(device, c"vkCmdWriteTimestamp".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyQueryPoolResults).write(load_typed(
                gdpa(device, c"vkCmdCopyQueryPoolResults".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdExecuteCommands)
                .write(load_typed(gdpa(device, c"vkCmdExecuteCommands".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateEvent)
                .write(load_typed(gdpa(device, c"vkCreateEvent".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyEvent)
                .write(load_typed(gdpa(device, c"vkDestroyEvent".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetEventStatus)
                .write(load_typed(gdpa(device, c"vkGetEventStatus".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetEvent)
                .write(load_typed(gdpa(device, c"vkSetEvent".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkResetEvent)
                .write(load_typed(gdpa(device, c"vkResetEvent".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateBufferView)
                .write(load_typed(gdpa(device, c"vkCreateBufferView".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyBufferView)
                .write(load_typed(gdpa(device, c"vkDestroyBufferView".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateShaderModule)
                .write(load_typed(gdpa(device, c"vkCreateShaderModule".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyShaderModule)
                .write(load_typed(gdpa(device, c"vkDestroyShaderModule".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreatePipelineCache)
                .write(load_typed(gdpa(device, c"vkCreatePipelineCache".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyPipelineCache)
                .write(load_typed(gdpa(device, c"vkDestroyPipelineCache".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPipelineCacheData)
                .write(load_typed(gdpa(device, c"vkGetPipelineCacheData".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkMergePipelineCaches)
                .write(load_typed(gdpa(device, c"vkMergePipelineCaches".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateComputePipelines).write(load_typed(gdpa(
                device,
                c"vkCreateComputePipelines".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyPipeline)
                .write(load_typed(gdpa(device, c"vkDestroyPipeline".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreatePipelineLayout)
                .write(load_typed(gdpa(device, c"vkCreatePipelineLayout".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyPipelineLayout).write(load_typed(gdpa(
                device,
                c"vkDestroyPipelineLayout".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateSampler)
                .write(load_typed(gdpa(device, c"vkCreateSampler".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroySampler)
                .write(load_typed(gdpa(device, c"vkDestroySampler".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDescriptorSetLayout).write(load_typed(
                gdpa(device, c"vkCreateDescriptorSetLayout".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyDescriptorSetLayout).write(load_typed(
                gdpa(device, c"vkDestroyDescriptorSetLayout".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDescriptorPool)
                .write(load_typed(gdpa(device, c"vkCreateDescriptorPool".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyDescriptorPool).write(load_typed(gdpa(
                device,
                c"vkDestroyDescriptorPool".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkResetDescriptorPool)
                .write(load_typed(gdpa(device, c"vkResetDescriptorPool".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAllocateDescriptorSets).write(load_typed(gdpa(
                device,
                c"vkAllocateDescriptorSets".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkFreeDescriptorSets)
                .write(load_typed(gdpa(device, c"vkFreeDescriptorSets".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkUpdateDescriptorSets)
                .write(load_typed(gdpa(device, c"vkUpdateDescriptorSets".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindPipeline)
                .write(load_typed(gdpa(device, c"vkCmdBindPipeline".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindDescriptorSets).write(load_typed(gdpa(
                device,
                c"vkCmdBindDescriptorSets".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdClearColorImage)
                .write(load_typed(gdpa(device, c"vkCmdClearColorImage".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDispatch)
                .write(load_typed(gdpa(device, c"vkCmdDispatch".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDispatchIndirect)
                .write(load_typed(gdpa(device, c"vkCmdDispatchIndirect".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetEvent)
                .write(load_typed(gdpa(device, c"vkCmdSetEvent".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdResetEvent)
                .write(load_typed(gdpa(device, c"vkCmdResetEvent".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWaitEvents)
                .write(load_typed(gdpa(device, c"vkCmdWaitEvents".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushConstants)
                .write(load_typed(gdpa(device, c"vkCmdPushConstants".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateGraphicsPipelines).write(load_typed(
                gdpa(device, c"vkCreateGraphicsPipelines".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateFramebuffer)
                .write(load_typed(gdpa(device, c"vkCreateFramebuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyFramebuffer)
                .write(load_typed(gdpa(device, c"vkDestroyFramebuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateRenderPass)
                .write(load_typed(gdpa(device, c"vkCreateRenderPass".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyRenderPass)
                .write(load_typed(gdpa(device, c"vkDestroyRenderPass".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetRenderAreaGranularity).write(load_typed(
                gdpa(device, c"vkGetRenderAreaGranularity".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetViewport)
                .write(load_typed(gdpa(device, c"vkCmdSetViewport".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetScissor)
                .write(load_typed(gdpa(device, c"vkCmdSetScissor".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetLineWidth)
                .write(load_typed(gdpa(device, c"vkCmdSetLineWidth".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthBias)
                .write(load_typed(gdpa(device, c"vkCmdSetDepthBias".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetBlendConstants)
                .write(load_typed(gdpa(device, c"vkCmdSetBlendConstants".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthBounds)
                .write(load_typed(gdpa(device, c"vkCmdSetDepthBounds".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetStencilCompareMask).write(load_typed(
                gdpa(device, c"vkCmdSetStencilCompareMask".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetStencilWriteMask).write(load_typed(gdpa(
                device,
                c"vkCmdSetStencilWriteMask".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetStencilReference).write(load_typed(gdpa(
                device,
                c"vkCmdSetStencilReference".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindIndexBuffer)
                .write(load_typed(gdpa(device, c"vkCmdBindIndexBuffer".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindVertexBuffers)
                .write(load_typed(gdpa(device, c"vkCmdBindVertexBuffers".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDraw)
                .write(load_typed(gdpa(device, c"vkCmdDraw".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndexed)
                .write(load_typed(gdpa(device, c"vkCmdDrawIndexed".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndirect)
                .write(load_typed(gdpa(device, c"vkCmdDrawIndirect".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndexedIndirect).write(load_typed(gdpa(
                device,
                c"vkCmdDrawIndexedIndirect".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBlitImage)
                .write(load_typed(gdpa(device, c"vkCmdBlitImage".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdClearDepthStencilImage).write(load_typed(
                gdpa(device, c"vkCmdClearDepthStencilImage".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdClearAttachments)
                .write(load_typed(gdpa(device, c"vkCmdClearAttachments".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdResolveImage)
                .write(load_typed(gdpa(device, c"vkCmdResolveImage".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginRenderPass)
                .write(load_typed(gdpa(device, c"vkCmdBeginRenderPass".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdNextSubpass)
                .write(load_typed(gdpa(device, c"vkCmdNextSubpass".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndRenderPass)
                .write(load_typed(gdpa(device, c"vkCmdEndRenderPass".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBindBufferMemory2)
                .write(load_typed(gdpa(device, c"vkBindBufferMemory2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBindImageMemory2)
                .write(load_typed(gdpa(device, c"vkBindImageMemory2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceGroupPeerMemoryFeatures).write(
                load_typed(gdpa(device, c"vkGetDeviceGroupPeerMemoryFeatures".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDeviceMask)
                .write(load_typed(gdpa(device, c"vkCmdSetDeviceMask".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageMemoryRequirements2).write(load_typed(
                gdpa(device, c"vkGetImageMemoryRequirements2".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetBufferMemoryRequirements2).write(load_typed(
                gdpa(device, c"vkGetBufferMemoryRequirements2".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageSparseMemoryRequirements2).write(
                load_typed(gdpa(
                    device,
                    c"vkGetImageSparseMemoryRequirements2".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkTrimCommandPool)
                .write(load_typed(gdpa(device, c"vkTrimCommandPool".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceQueue2)
                .write(load_typed(gdpa(device, c"vkGetDeviceQueue2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDispatchBase)
                .write(load_typed(gdpa(device, c"vkCmdDispatchBase".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDescriptorUpdateTemplate).write(
                load_typed(gdpa(device, c"vkCreateDescriptorUpdateTemplate".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyDescriptorUpdateTemplate).write(
                load_typed(gdpa(device, c"vkDestroyDescriptorUpdateTemplate".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkUpdateDescriptorSetWithTemplate).write(
                load_typed(gdpa(device, c"vkUpdateDescriptorSetWithTemplate".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDescriptorSetLayoutSupport).write(
                load_typed(gdpa(device, c"vkGetDescriptorSetLayoutSupport".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateSamplerYcbcrConversion).write(load_typed(
                gdpa(device, c"vkCreateSamplerYcbcrConversion".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroySamplerYcbcrConversion).write(
                load_typed(gdpa(device, c"vkDestroySamplerYcbcrConversion".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkResetQueryPool)
                .write(load_typed(gdpa(device, c"vkResetQueryPool".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSemaphoreCounterValue).write(load_typed(
                gdpa(device, c"vkGetSemaphoreCounterValue".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkWaitSemaphores)
                .write(load_typed(gdpa(device, c"vkWaitSemaphores".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSignalSemaphore)
                .write(load_typed(gdpa(device, c"vkSignalSemaphore".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetBufferDeviceAddress).write(load_typed(gdpa(
                device,
                c"vkGetBufferDeviceAddress".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetBufferOpaqueCaptureAddress).write(
                load_typed(gdpa(device, c"vkGetBufferOpaqueCaptureAddress".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceMemoryOpaqueCaptureAddress).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceMemoryOpaqueCaptureAddress".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndirectCount)
                .write(load_typed(gdpa(device, c"vkCmdDrawIndirectCount".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndexedIndirectCount).write(load_typed(
                gdpa(device, c"vkCmdDrawIndexedIndirectCount".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateRenderPass2)
                .write(load_typed(gdpa(device, c"vkCreateRenderPass2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginRenderPass2)
                .write(load_typed(gdpa(device, c"vkCmdBeginRenderPass2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdNextSubpass2)
                .write(load_typed(gdpa(device, c"vkCmdNextSubpass2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndRenderPass2)
                .write(load_typed(gdpa(device, c"vkCmdEndRenderPass2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreatePrivateDataSlot).write(load_typed(gdpa(
                device,
                c"vkCreatePrivateDataSlot".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyPrivateDataSlot).write(load_typed(gdpa(
                device,
                c"vkDestroyPrivateDataSlot".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetPrivateData)
                .write(load_typed(gdpa(device, c"vkSetPrivateData".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPrivateData)
                .write(load_typed(gdpa(device, c"vkGetPrivateData".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPipelineBarrier2)
                .write(load_typed(gdpa(device, c"vkCmdPipelineBarrier2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWriteTimestamp2)
                .write(load_typed(gdpa(device, c"vkCmdWriteTimestamp2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueSubmit2)
                .write(load_typed(gdpa(device, c"vkQueueSubmit2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyBuffer2)
                .write(load_typed(gdpa(device, c"vkCmdCopyBuffer2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyImage2)
                .write(load_typed(gdpa(device, c"vkCmdCopyImage2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyBufferToImage2).write(load_typed(gdpa(
                device,
                c"vkCmdCopyBufferToImage2".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyImageToBuffer2).write(load_typed(gdpa(
                device,
                c"vkCmdCopyImageToBuffer2".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceBufferMemoryRequirements).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceBufferMemoryRequirements".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceImageMemoryRequirements).write(
                load_typed(gdpa(device, c"vkGetDeviceImageMemoryRequirements".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceImageSparseMemoryRequirements).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceImageSparseMemoryRequirements".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetEvent2)
                .write(load_typed(gdpa(device, c"vkCmdSetEvent2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdResetEvent2)
                .write(load_typed(gdpa(device, c"vkCmdResetEvent2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWaitEvents2)
                .write(load_typed(gdpa(device, c"vkCmdWaitEvents2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBlitImage2)
                .write(load_typed(gdpa(device, c"vkCmdBlitImage2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdResolveImage2)
                .write(load_typed(gdpa(device, c"vkCmdResolveImage2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginRendering)
                .write(load_typed(gdpa(device, c"vkCmdBeginRendering".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndRendering)
                .write(load_typed(gdpa(device, c"vkCmdEndRendering".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetCullMode)
                .write(load_typed(gdpa(device, c"vkCmdSetCullMode".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetFrontFace)
                .write(load_typed(gdpa(device, c"vkCmdSetFrontFace".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetPrimitiveTopology).write(load_typed(
                gdpa(device, c"vkCmdSetPrimitiveTopology".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetViewportWithCount).write(load_typed(
                gdpa(device, c"vkCmdSetViewportWithCount".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetScissorWithCount).write(load_typed(gdpa(
                device,
                c"vkCmdSetScissorWithCount".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindVertexBuffers2).write(load_typed(gdpa(
                device,
                c"vkCmdBindVertexBuffers2".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthTestEnable).write(load_typed(gdpa(
                device,
                c"vkCmdSetDepthTestEnable".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthWriteEnable).write(load_typed(gdpa(
                device,
                c"vkCmdSetDepthWriteEnable".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthCompareOp)
                .write(load_typed(gdpa(device, c"vkCmdSetDepthCompareOp".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthBoundsTestEnable).write(load_typed(
                gdpa(device, c"vkCmdSetDepthBoundsTestEnable".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetStencilTestEnable).write(load_typed(
                gdpa(device, c"vkCmdSetStencilTestEnable".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetStencilOp)
                .write(load_typed(gdpa(device, c"vkCmdSetStencilOp".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetRasterizerDiscardEnable).write(
                load_typed(gdpa(device, c"vkCmdSetRasterizerDiscardEnable".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthBiasEnable).write(load_typed(gdpa(
                device,
                c"vkCmdSetDepthBiasEnable".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetPrimitiveRestartEnable).write(load_typed(
                gdpa(device, c"vkCmdSetPrimitiveRestartEnable".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkMapMemory2)
                .write(load_typed(gdpa(device, c"vkMapMemory2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkUnmapMemory2)
                .write(load_typed(gdpa(device, c"vkUnmapMemory2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceImageSubresourceLayout).write(
                load_typed(gdpa(device, c"vkGetDeviceImageSubresourceLayout".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageSubresourceLayout2).write(load_typed(
                gdpa(device, c"vkGetImageSubresourceLayout2".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyMemoryToImage)
                .write(load_typed(gdpa(device, c"vkCopyMemoryToImage".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyImageToMemory)
                .write(load_typed(gdpa(device, c"vkCopyImageToMemory".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyImageToImage)
                .write(load_typed(gdpa(device, c"vkCopyImageToImage".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkTransitionImageLayout).write(load_typed(gdpa(
                device,
                c"vkTransitionImageLayout".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushDescriptorSet)
                .write(load_typed(gdpa(device, c"vkCmdPushDescriptorSet".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushDescriptorSetWithTemplate).write(
                load_typed(gdpa(device, c"vkCmdPushDescriptorSetWithTemplate".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindDescriptorSets2).write(load_typed(gdpa(
                device,
                c"vkCmdBindDescriptorSets2".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushConstants2)
                .write(load_typed(gdpa(device, c"vkCmdPushConstants2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushDescriptorSet2).write(load_typed(gdpa(
                device,
                c"vkCmdPushDescriptorSet2".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushDescriptorSetWithTemplate2).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdPushDescriptorSetWithTemplate2".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetLineStipple)
                .write(load_typed(gdpa(device, c"vkCmdSetLineStipple".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindIndexBuffer2)
                .write(load_typed(gdpa(device, c"vkCmdBindIndexBuffer2".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetRenderingAreaGranularity).write(load_typed(
                gdpa(device, c"vkGetRenderingAreaGranularity".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetRenderingAttachmentLocations).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetRenderingAttachmentLocations".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetRenderingInputAttachmentIndices).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetRenderingInputAttachmentIndices".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateSwapchainKHR)
                .write(load_typed(gdpa(device, c"vkCreateSwapchainKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroySwapchainKHR)
                .write(load_typed(gdpa(device, c"vkDestroySwapchainKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSwapchainImagesKHR).write(load_typed(gdpa(
                device,
                c"vkGetSwapchainImagesKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAcquireNextImageKHR)
                .write(load_typed(gdpa(device, c"vkAcquireNextImageKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueuePresentKHR)
                .write(load_typed(gdpa(device, c"vkQueuePresentKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceGroupPresentCapabilitiesKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceGroupPresentCapabilitiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceGroupSurfacePresentModesKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceGroupSurfacePresentModesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAcquireNextImage2KHR)
                .write(load_typed(gdpa(device, c"vkAcquireNextImage2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateSharedSwapchainsKHR).write(load_typed(
                gdpa(device, c"vkCreateSharedSwapchainsKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateVideoSessionKHR).write(load_typed(gdpa(
                device,
                c"vkCreateVideoSessionKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyVideoSessionKHR).write(load_typed(gdpa(
                device,
                c"vkDestroyVideoSessionKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetVideoSessionMemoryRequirementsKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetVideoSessionMemoryRequirementsKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBindVideoSessionMemoryKHR).write(load_typed(
                gdpa(device, c"vkBindVideoSessionMemoryKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateVideoSessionParametersKHR).write(
                load_typed(gdpa(device, c"vkCreateVideoSessionParametersKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkUpdateVideoSessionParametersKHR).write(
                load_typed(gdpa(device, c"vkUpdateVideoSessionParametersKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyVideoSessionParametersKHR).write(
                load_typed(gdpa(device, c"vkDestroyVideoSessionParametersKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginVideoCodingKHR).write(load_typed(gdpa(
                device,
                c"vkCmdBeginVideoCodingKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndVideoCodingKHR)
                .write(load_typed(gdpa(device, c"vkCmdEndVideoCodingKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdControlVideoCodingKHR).write(load_typed(
                gdpa(device, c"vkCmdControlVideoCodingKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDecodeVideoKHR)
                .write(load_typed(gdpa(device, c"vkCmdDecodeVideoKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginRenderingKHR)
                .write(load_typed(gdpa(device, c"vkCmdBeginRenderingKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndRenderingKHR)
                .write(load_typed(gdpa(device, c"vkCmdEndRenderingKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceGroupPeerMemoryFeaturesKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceGroupPeerMemoryFeaturesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDeviceMaskKHR)
                .write(load_typed(gdpa(device, c"vkCmdSetDeviceMaskKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDispatchBaseKHR)
                .write(load_typed(gdpa(device, c"vkCmdDispatchBaseKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkTrimCommandPoolKHR)
                .write(load_typed(gdpa(device, c"vkTrimCommandPoolKHR".as_ptr())));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryWin32HandleKHR).write(load_typed(
                gdpa(device, c"vkGetMemoryWin32HandleKHR".as_ptr()),
            ));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryWin32HandlePropertiesKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetMemoryWin32HandlePropertiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryFdKHR)
                .write(load_typed(gdpa(device, c"vkGetMemoryFdKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryFdPropertiesKHR).write(load_typed(
                gdpa(device, c"vkGetMemoryFdPropertiesKHR".as_ptr()),
            ));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkImportSemaphoreWin32HandleKHR).write(
                load_typed(gdpa(device, c"vkImportSemaphoreWin32HandleKHR".as_ptr())),
            );
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSemaphoreWin32HandleKHR).write(load_typed(
                gdpa(device, c"vkGetSemaphoreWin32HandleKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkImportSemaphoreFdKHR)
                .write(load_typed(gdpa(device, c"vkImportSemaphoreFdKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSemaphoreFdKHR)
                .write(load_typed(gdpa(device, c"vkGetSemaphoreFdKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushDescriptorSetKHR).write(load_typed(
                gdpa(device, c"vkCmdPushDescriptorSetKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushDescriptorSetWithTemplateKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdPushDescriptorSetWithTemplateKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDescriptorUpdateTemplateKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCreateDescriptorUpdateTemplateKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyDescriptorUpdateTemplateKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkDestroyDescriptorUpdateTemplateKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkUpdateDescriptorSetWithTemplateKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkUpdateDescriptorSetWithTemplateKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateRenderPass2KHR)
                .write(load_typed(gdpa(device, c"vkCreateRenderPass2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginRenderPass2KHR).write(load_typed(gdpa(
                device,
                c"vkCmdBeginRenderPass2KHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdNextSubpass2KHR)
                .write(load_typed(gdpa(device, c"vkCmdNextSubpass2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndRenderPass2KHR)
                .write(load_typed(gdpa(device, c"vkCmdEndRenderPass2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSwapchainStatusKHR).write(load_typed(gdpa(
                device,
                c"vkGetSwapchainStatusKHR".as_ptr(),
            )));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkImportFenceWin32HandleKHR).write(load_typed(
                gdpa(device, c"vkImportFenceWin32HandleKHR".as_ptr()),
            ));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetFenceWin32HandleKHR).write(load_typed(gdpa(
                device,
                c"vkGetFenceWin32HandleKHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkImportFenceFdKHR)
                .write(load_typed(gdpa(device, c"vkImportFenceFdKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetFenceFdKHR)
                .write(load_typed(gdpa(device, c"vkGetFenceFdKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAcquireProfilingLockKHR).write(load_typed(
                gdpa(device, c"vkAcquireProfilingLockKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkReleaseProfilingLockKHR).write(load_typed(
                gdpa(device, c"vkReleaseProfilingLockKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageMemoryRequirements2KHR).write(
                load_typed(gdpa(device, c"vkGetImageMemoryRequirements2KHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetBufferMemoryRequirements2KHR).write(
                load_typed(gdpa(device, c"vkGetBufferMemoryRequirements2KHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageSparseMemoryRequirements2KHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetImageSparseMemoryRequirements2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateSamplerYcbcrConversionKHR).write(
                load_typed(gdpa(device, c"vkCreateSamplerYcbcrConversionKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroySamplerYcbcrConversionKHR).write(
                load_typed(gdpa(device, c"vkDestroySamplerYcbcrConversionKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBindBufferMemory2KHR)
                .write(load_typed(gdpa(device, c"vkBindBufferMemory2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBindImageMemory2KHR)
                .write(load_typed(gdpa(device, c"vkBindImageMemory2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDescriptorSetLayoutSupportKHR).write(
                load_typed(gdpa(device, c"vkGetDescriptorSetLayoutSupportKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndirectCountKHR).write(load_typed(
                gdpa(device, c"vkCmdDrawIndirectCountKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndexedIndirectCountKHR).write(
                load_typed(gdpa(device, c"vkCmdDrawIndexedIndirectCountKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSemaphoreCounterValueKHR).write(load_typed(
                gdpa(device, c"vkGetSemaphoreCounterValueKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkWaitSemaphoresKHR)
                .write(load_typed(gdpa(device, c"vkWaitSemaphoresKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSignalSemaphoreKHR)
                .write(load_typed(gdpa(device, c"vkSignalSemaphoreKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetFragmentShadingRateKHR).write(load_typed(
                gdpa(device, c"vkCmdSetFragmentShadingRateKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetRenderingAttachmentLocationsKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetRenderingAttachmentLocationsKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetRenderingInputAttachmentIndicesKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetRenderingInputAttachmentIndicesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkWaitForPresentKHR)
                .write(load_typed(gdpa(device, c"vkWaitForPresentKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetBufferDeviceAddressKHR).write(load_typed(
                gdpa(device, c"vkGetBufferDeviceAddressKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetBufferOpaqueCaptureAddressKHR).write(
                load_typed(gdpa(device, c"vkGetBufferOpaqueCaptureAddressKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceMemoryOpaqueCaptureAddressKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceMemoryOpaqueCaptureAddressKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDeferredOperationKHR).write(load_typed(
                gdpa(device, c"vkCreateDeferredOperationKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyDeferredOperationKHR).write(load_typed(
                gdpa(device, c"vkDestroyDeferredOperationKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeferredOperationMaxConcurrencyKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeferredOperationMaxConcurrencyKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeferredOperationResultKHR).write(
                load_typed(gdpa(device, c"vkGetDeferredOperationResultKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDeferredOperationJoinKHR).write(load_typed(
                gdpa(device, c"vkDeferredOperationJoinKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPipelineExecutablePropertiesKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetPipelineExecutablePropertiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPipelineExecutableStatisticsKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetPipelineExecutableStatisticsKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPipelineExecutableInternalRepresentationsKHR)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetPipelineExecutableInternalRepresentationsKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkMapMemory2KHR)
                .write(load_typed(gdpa(device, c"vkMapMemory2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkUnmapMemory2KHR)
                .write(load_typed(gdpa(device, c"vkUnmapMemory2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetEncodedVideoSessionParametersKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetEncodedVideoSessionParametersKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEncodeVideoKHR)
                .write(load_typed(gdpa(device, c"vkCmdEncodeVideoKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetEvent2KHR)
                .write(load_typed(gdpa(device, c"vkCmdSetEvent2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdResetEvent2KHR)
                .write(load_typed(gdpa(device, c"vkCmdResetEvent2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWaitEvents2KHR)
                .write(load_typed(gdpa(device, c"vkCmdWaitEvents2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPipelineBarrier2KHR).write(load_typed(gdpa(
                device,
                c"vkCmdPipelineBarrier2KHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWriteTimestamp2KHR).write(load_typed(gdpa(
                device,
                c"vkCmdWriteTimestamp2KHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueSubmit2KHR)
                .write(load_typed(gdpa(device, c"vkQueueSubmit2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindIndexBuffer3KHR).write(load_typed(gdpa(
                device,
                c"vkCmdBindIndexBuffer3KHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindVertexBuffers3KHR).write(load_typed(
                gdpa(device, c"vkCmdBindVertexBuffers3KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndirect2KHR)
                .write(load_typed(gdpa(device, c"vkCmdDrawIndirect2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndexedIndirect2KHR).write(load_typed(
                gdpa(device, c"vkCmdDrawIndexedIndirect2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDispatchIndirect2KHR).write(load_typed(
                gdpa(device, c"vkCmdDispatchIndirect2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyMemoryKHR)
                .write(load_typed(gdpa(device, c"vkCmdCopyMemoryKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyMemoryToImageKHR).write(load_typed(
                gdpa(device, c"vkCmdCopyMemoryToImageKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyImageToMemoryKHR).write(load_typed(
                gdpa(device, c"vkCmdCopyImageToMemoryKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdUpdateMemoryKHR)
                .write(load_typed(gdpa(device, c"vkCmdUpdateMemoryKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdFillMemoryKHR)
                .write(load_typed(gdpa(device, c"vkCmdFillMemoryKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyQueryPoolResultsToMemoryKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdCopyQueryPoolResultsToMemoryKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndirectCount2KHR).write(load_typed(
                gdpa(device, c"vkCmdDrawIndirectCount2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndexedIndirectCount2KHR).write(
                load_typed(gdpa(device, c"vkCmdDrawIndexedIndirectCount2KHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginConditionalRendering2EXT).write(
                load_typed(gdpa(device, c"vkCmdBeginConditionalRendering2EXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindTransformFeedbackBuffers2EXT).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdBindTransformFeedbackBuffers2EXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginTransformFeedback2EXT).write(
                load_typed(gdpa(device, c"vkCmdBeginTransformFeedback2EXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndTransformFeedback2EXT).write(load_typed(
                gdpa(device, c"vkCmdEndTransformFeedback2EXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndirectByteCount2EXT).write(load_typed(
                gdpa(device, c"vkCmdDrawIndirectByteCount2EXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawMeshTasksIndirect2EXT).write(load_typed(
                gdpa(device, c"vkCmdDrawMeshTasksIndirect2EXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawMeshTasksIndirectCount2EXT).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdDrawMeshTasksIndirectCount2EXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWriteMarkerToMemoryAMD).write(load_typed(
                gdpa(device, c"vkCmdWriteMarkerToMemoryAMD".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateAccelerationStructure2KHR).write(
                load_typed(gdpa(device, c"vkCreateAccelerationStructure2KHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyBuffer2KHR)
                .write(load_typed(gdpa(device, c"vkCmdCopyBuffer2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyImage2KHR)
                .write(load_typed(gdpa(device, c"vkCmdCopyImage2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyBufferToImage2KHR).write(load_typed(
                gdpa(device, c"vkCmdCopyBufferToImage2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyImageToBuffer2KHR).write(load_typed(
                gdpa(device, c"vkCmdCopyImageToBuffer2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBlitImage2KHR)
                .write(load_typed(gdpa(device, c"vkCmdBlitImage2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdResolveImage2KHR)
                .write(load_typed(gdpa(device, c"vkCmdResolveImage2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdTraceRaysIndirect2KHR).write(load_typed(
                gdpa(device, c"vkCmdTraceRaysIndirect2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceBufferMemoryRequirementsKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceBufferMemoryRequirementsKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceImageMemoryRequirementsKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceImageMemoryRequirementsKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceImageSparseMemoryRequirementsKHR)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetDeviceImageSparseMemoryRequirementsKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindIndexBuffer2KHR).write(load_typed(gdpa(
                device,
                c"vkCmdBindIndexBuffer2KHR".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetRenderingAreaGranularityKHR).write(
                load_typed(gdpa(device, c"vkGetRenderingAreaGranularityKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceImageSubresourceLayoutKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceImageSubresourceLayoutKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageSubresourceLayout2KHR).write(
                load_typed(gdpa(device, c"vkGetImageSubresourceLayout2KHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkWaitForPresent2KHR)
                .write(load_typed(gdpa(device, c"vkWaitForPresent2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreatePipelineBinariesKHR).write(load_typed(
                gdpa(device, c"vkCreatePipelineBinariesKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyPipelineBinaryKHR).write(load_typed(
                gdpa(device, c"vkDestroyPipelineBinaryKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPipelineKeyKHR)
                .write(load_typed(gdpa(device, c"vkGetPipelineKeyKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPipelineBinaryDataKHR).write(load_typed(
                gdpa(device, c"vkGetPipelineBinaryDataKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkReleaseCapturedPipelineDataKHR).write(
                load_typed(gdpa(device, c"vkReleaseCapturedPipelineDataKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkReleaseSwapchainImagesKHR).write(load_typed(
                gdpa(device, c"vkReleaseSwapchainImagesKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetLineStippleKHR)
                .write(load_typed(gdpa(device, c"vkCmdSetLineStippleKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetCalibratedTimestampsKHR).write(load_typed(
                gdpa(device, c"vkGetCalibratedTimestampsKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindDescriptorSets2KHR).write(load_typed(
                gdpa(device, c"vkCmdBindDescriptorSets2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushConstants2KHR)
                .write(load_typed(gdpa(device, c"vkCmdPushConstants2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushDescriptorSet2KHR).write(load_typed(
                gdpa(device, c"vkCmdPushDescriptorSet2KHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushDescriptorSetWithTemplate2KHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdPushDescriptorSetWithTemplate2KHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDescriptorBufferOffsets2EXT).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetDescriptorBufferOffsets2EXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindDescriptorBufferEmbeddedSamplers2EXT)
                .write(load_typed(gdpa(
                    device,
                    c"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyMemoryIndirectKHR).write(load_typed(
                gdpa(device, c"vkCmdCopyMemoryIndirectKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyMemoryToImageIndirectKHR).write(
                load_typed(gdpa(device, c"vkCmdCopyMemoryToImageIndirectKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceFaultReportsKHR).write(load_typed(
                gdpa(device, c"vkGetDeviceFaultReportsKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceFaultDebugInfoKHR).write(load_typed(
                gdpa(device, c"vkGetDeviceFaultDebugInfoKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndRendering2KHR)
                .write(load_typed(gdpa(device, c"vkCmdEndRendering2KHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDebugMarkerSetObjectTagEXT).write(load_typed(
                gdpa(device, c"vkDebugMarkerSetObjectTagEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDebugMarkerSetObjectNameEXT).write(load_typed(
                gdpa(device, c"vkDebugMarkerSetObjectNameEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDebugMarkerBeginEXT).write(load_typed(gdpa(
                device,
                c"vkCmdDebugMarkerBeginEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDebugMarkerEndEXT)
                .write(load_typed(gdpa(device, c"vkCmdDebugMarkerEndEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDebugMarkerInsertEXT).write(load_typed(
                gdpa(device, c"vkCmdDebugMarkerInsertEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindTransformFeedbackBuffersEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdBindTransformFeedbackBuffersEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginTransformFeedbackEXT).write(load_typed(
                gdpa(device, c"vkCmdBeginTransformFeedbackEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndTransformFeedbackEXT).write(load_typed(
                gdpa(device, c"vkCmdEndTransformFeedbackEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginQueryIndexedEXT).write(load_typed(
                gdpa(device, c"vkCmdBeginQueryIndexedEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndQueryIndexedEXT).write(load_typed(gdpa(
                device,
                c"vkCmdEndQueryIndexedEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndirectByteCountEXT).write(load_typed(
                gdpa(device, c"vkCmdDrawIndirectByteCountEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateCuModuleNVX)
                .write(load_typed(gdpa(device, c"vkCreateCuModuleNVX".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateCuFunctionNVX)
                .write(load_typed(gdpa(device, c"vkCreateCuFunctionNVX".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyCuModuleNVX)
                .write(load_typed(gdpa(device, c"vkDestroyCuModuleNVX".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyCuFunctionNVX)
                .write(load_typed(gdpa(device, c"vkDestroyCuFunctionNVX".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCuLaunchKernelNVX)
                .write(load_typed(gdpa(device, c"vkCmdCuLaunchKernelNVX".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageViewHandleNVX).write(load_typed(gdpa(
                device,
                c"vkGetImageViewHandleNVX".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageViewHandle64NVX).write(load_typed(
                gdpa(device, c"vkGetImageViewHandle64NVX".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageViewAddressNVX).write(load_typed(gdpa(
                device,
                c"vkGetImageViewAddressNVX".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceCombinedImageSamplerIndexNVX).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceCombinedImageSamplerIndexNVX".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndirectCountAMD).write(load_typed(
                gdpa(device, c"vkCmdDrawIndirectCountAMD".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawIndexedIndirectCountAMD).write(
                load_typed(gdpa(device, c"vkCmdDrawIndexedIndirectCountAMD".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetShaderInfoAMD)
                .write(load_typed(gdpa(device, c"vkGetShaderInfoAMD".as_ptr())));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryWin32HandleNV).write(load_typed(gdpa(
                device,
                c"vkGetMemoryWin32HandleNV".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginConditionalRenderingEXT).write(
                load_typed(gdpa(device, c"vkCmdBeginConditionalRenderingEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndConditionalRenderingEXT).write(
                load_typed(gdpa(device, c"vkCmdEndConditionalRenderingEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetViewportWScalingNV).write(load_typed(
                gdpa(device, c"vkCmdSetViewportWScalingNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDisplayPowerControlEXT).write(load_typed(gdpa(
                device,
                c"vkDisplayPowerControlEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkRegisterDeviceEventEXT).write(load_typed(gdpa(
                device,
                c"vkRegisterDeviceEventEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkRegisterDisplayEventEXT).write(load_typed(
                gdpa(device, c"vkRegisterDisplayEventEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSwapchainCounterEXT).write(load_typed(gdpa(
                device,
                c"vkGetSwapchainCounterEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetRefreshCycleDurationGOOGLE).write(
                load_typed(gdpa(device, c"vkGetRefreshCycleDurationGOOGLE".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPastPresentationTimingGOOGLE).write(
                load_typed(gdpa(device, c"vkGetPastPresentationTimingGOOGLE".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDiscardRectangleEXT).write(load_typed(
                gdpa(device, c"vkCmdSetDiscardRectangleEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDiscardRectangleEnableEXT).write(
                load_typed(gdpa(device, c"vkCmdSetDiscardRectangleEnableEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDiscardRectangleModeEXT).write(
                load_typed(gdpa(device, c"vkCmdSetDiscardRectangleModeEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetHdrMetadataEXT)
                .write(load_typed(gdpa(device, c"vkSetHdrMetadataEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetDebugUtilsObjectNameEXT).write(load_typed(
                gdpa(device, c"vkSetDebugUtilsObjectNameEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetDebugUtilsObjectTagEXT).write(load_typed(
                gdpa(device, c"vkSetDebugUtilsObjectTagEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueBeginDebugUtilsLabelEXT).write(load_typed(
                gdpa(device, c"vkQueueBeginDebugUtilsLabelEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueEndDebugUtilsLabelEXT).write(load_typed(
                gdpa(device, c"vkQueueEndDebugUtilsLabelEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueInsertDebugUtilsLabelEXT).write(
                load_typed(gdpa(device, c"vkQueueInsertDebugUtilsLabelEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginDebugUtilsLabelEXT).write(load_typed(
                gdpa(device, c"vkCmdBeginDebugUtilsLabelEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndDebugUtilsLabelEXT).write(load_typed(
                gdpa(device, c"vkCmdEndDebugUtilsLabelEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdInsertDebugUtilsLabelEXT).write(load_typed(
                gdpa(device, c"vkCmdInsertDebugUtilsLabelEXT".as_ptr()),
            ));
        }
        #[cfg(target_os = "android")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetAndroidHardwareBufferPropertiesANDROID)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetAndroidHardwareBufferPropertiesANDROID".as_ptr(),
                )));
        }
        #[cfg(target_os = "android")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryAndroidHardwareBufferANDROID).write(
                load_typed(gdpa(
                    device,
                    c"vkGetMemoryAndroidHardwareBufferANDROID".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateGpaSessionAMD)
                .write(load_typed(gdpa(device, c"vkCreateGpaSessionAMD".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyGpaSessionAMD)
                .write(load_typed(gdpa(device, c"vkDestroyGpaSessionAMD".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetGpaDeviceClockModeAMD).write(load_typed(
                gdpa(device, c"vkSetGpaDeviceClockModeAMD".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetGpaDeviceClockInfoAMD).write(load_typed(
                gdpa(device, c"vkGetGpaDeviceClockInfoAMD".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginGpaSessionAMD).write(load_typed(gdpa(
                device,
                c"vkCmdBeginGpaSessionAMD".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndGpaSessionAMD)
                .write(load_typed(gdpa(device, c"vkCmdEndGpaSessionAMD".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginGpaSampleAMD)
                .write(load_typed(gdpa(device, c"vkCmdBeginGpaSampleAMD".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndGpaSampleAMD)
                .write(load_typed(gdpa(device, c"vkCmdEndGpaSampleAMD".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetGpaSessionStatusAMD).write(load_typed(gdpa(
                device,
                c"vkGetGpaSessionStatusAMD".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetGpaSessionResultsAMD).write(load_typed(
                gdpa(device, c"vkGetGpaSessionResultsAMD".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkResetGpaSessionAMD)
                .write(load_typed(gdpa(device, c"vkResetGpaSessionAMD".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyGpaSessionResultsAMD).write(load_typed(
                gdpa(device, c"vkCmdCopyGpaSessionResultsAMD".as_ptr()),
            ));
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateExecutionGraphPipelinesAMDX).write(
                load_typed(gdpa(
                    device,
                    c"vkCreateExecutionGraphPipelinesAMDX".as_ptr(),
                )),
            );
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetExecutionGraphPipelineScratchSizeAMDX).write(
                load_typed(gdpa(
                    device,
                    c"vkGetExecutionGraphPipelineScratchSizeAMDX".as_ptr(),
                )),
            );
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetExecutionGraphPipelineNodeIndexAMDX).write(
                load_typed(gdpa(
                    device,
                    c"vkGetExecutionGraphPipelineNodeIndexAMDX".as_ptr(),
                )),
            );
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdInitializeGraphScratchMemoryAMDX).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdInitializeGraphScratchMemoryAMDX".as_ptr(),
                )),
            );
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDispatchGraphAMDX)
                .write(load_typed(gdpa(device, c"vkCmdDispatchGraphAMDX".as_ptr())));
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDispatchGraphIndirectAMDX).write(load_typed(
                gdpa(device, c"vkCmdDispatchGraphIndirectAMDX".as_ptr()),
            ));
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDispatchGraphIndirectCountAMDX).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdDispatchGraphIndirectCountAMDX".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkWriteSamplerDescriptorsEXT).write(load_typed(
                gdpa(device, c"vkWriteSamplerDescriptorsEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkWriteResourceDescriptorsEXT).write(load_typed(
                gdpa(device, c"vkWriteResourceDescriptorsEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindSamplerHeapEXT).write(load_typed(gdpa(
                device,
                c"vkCmdBindSamplerHeapEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindResourceHeapEXT).write(load_typed(gdpa(
                device,
                c"vkCmdBindResourceHeapEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPushDataEXT)
                .write(load_typed(gdpa(device, c"vkCmdPushDataEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageOpaqueCaptureDataEXT).write(load_typed(
                gdpa(device, c"vkGetImageOpaqueCaptureDataEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkRegisterCustomBorderColorEXT).write(load_typed(
                gdpa(device, c"vkRegisterCustomBorderColorEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkUnregisterCustomBorderColorEXT).write(
                load_typed(gdpa(device, c"vkUnregisterCustomBorderColorEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetTensorOpaqueCaptureDataARM).write(
                load_typed(gdpa(device, c"vkGetTensorOpaqueCaptureDataARM".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetSampleLocationsEXT).write(load_typed(
                gdpa(device, c"vkCmdSetSampleLocationsEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageDrmFormatModifierPropertiesEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkGetImageDrmFormatModifierPropertiesEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateValidationCacheEXT).write(load_typed(
                gdpa(device, c"vkCreateValidationCacheEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyValidationCacheEXT).write(load_typed(
                gdpa(device, c"vkDestroyValidationCacheEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkMergeValidationCachesEXT).write(load_typed(
                gdpa(device, c"vkMergeValidationCachesEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetValidationCacheDataEXT).write(load_typed(
                gdpa(device, c"vkGetValidationCacheDataEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindShadingRateImageNV).write(load_typed(
                gdpa(device, c"vkCmdBindShadingRateImageNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetViewportShadingRatePaletteNV).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetViewportShadingRatePaletteNV".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetCoarseSampleOrderNV).write(load_typed(
                gdpa(device, c"vkCmdSetCoarseSampleOrderNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateAccelerationStructureNV).write(
                load_typed(gdpa(device, c"vkCreateAccelerationStructureNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyAccelerationStructureNV).write(
                load_typed(gdpa(device, c"vkDestroyAccelerationStructureNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetAccelerationStructureMemoryRequirementsNV)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetAccelerationStructureMemoryRequirementsNV".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBindAccelerationStructureMemoryNV).write(
                load_typed(gdpa(
                    device,
                    c"vkBindAccelerationStructureMemoryNV".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBuildAccelerationStructureNV).write(
                load_typed(gdpa(device, c"vkCmdBuildAccelerationStructureNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyAccelerationStructureNV).write(
                load_typed(gdpa(device, c"vkCmdCopyAccelerationStructureNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdTraceRaysNV)
                .write(load_typed(gdpa(device, c"vkCmdTraceRaysNV".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateRayTracingPipelinesNV).write(load_typed(
                gdpa(device, c"vkCreateRayTracingPipelinesNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetRayTracingShaderGroupHandlesKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetRayTracingShaderGroupHandlesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetRayTracingShaderGroupHandlesNV).write(
                load_typed(gdpa(
                    device,
                    c"vkGetRayTracingShaderGroupHandlesNV".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetAccelerationStructureHandleNV).write(
                load_typed(gdpa(device, c"vkGetAccelerationStructureHandleNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWriteAccelerationStructuresPropertiesNV)
                .write(load_typed(gdpa(
                    device,
                    c"vkCmdWriteAccelerationStructuresPropertiesNV".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCompileDeferredNV)
                .write(load_typed(gdpa(device, c"vkCompileDeferredNV".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryHostPointerPropertiesEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkGetMemoryHostPointerPropertiesEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWriteBufferMarkerAMD).write(load_typed(
                gdpa(device, c"vkCmdWriteBufferMarkerAMD".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWriteBufferMarker2AMD).write(load_typed(
                gdpa(device, c"vkCmdWriteBufferMarker2AMD".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetCalibratedTimestampsEXT).write(load_typed(
                gdpa(device, c"vkGetCalibratedTimestampsEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawMeshTasksNV)
                .write(load_typed(gdpa(device, c"vkCmdDrawMeshTasksNV".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawMeshTasksIndirectNV).write(load_typed(
                gdpa(device, c"vkCmdDrawMeshTasksIndirectNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawMeshTasksIndirectCountNV).write(
                load_typed(gdpa(device, c"vkCmdDrawMeshTasksIndirectCountNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetExclusiveScissorEnableNV).write(
                load_typed(gdpa(device, c"vkCmdSetExclusiveScissorEnableNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetExclusiveScissorNV).write(load_typed(
                gdpa(device, c"vkCmdSetExclusiveScissorNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetCheckpointNV)
                .write(load_typed(gdpa(device, c"vkCmdSetCheckpointNV".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetQueueCheckpointDataNV).write(load_typed(
                gdpa(device, c"vkGetQueueCheckpointDataNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetQueueCheckpointData2NV).write(load_typed(
                gdpa(device, c"vkGetQueueCheckpointData2NV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetSwapchainPresentTimingQueueSizeEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkSetSwapchainPresentTimingQueueSizeEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSwapchainTimingPropertiesEXT).write(
                load_typed(gdpa(device, c"vkGetSwapchainTimingPropertiesEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSwapchainTimeDomainPropertiesEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkGetSwapchainTimeDomainPropertiesEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPastPresentationTimingEXT).write(load_typed(
                gdpa(device, c"vkGetPastPresentationTimingEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkInitializePerformanceApiINTEL).write(
                load_typed(gdpa(device, c"vkInitializePerformanceApiINTEL".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkUninitializePerformanceApiINTEL).write(
                load_typed(gdpa(device, c"vkUninitializePerformanceApiINTEL".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetPerformanceMarkerINTEL).write(load_typed(
                gdpa(device, c"vkCmdSetPerformanceMarkerINTEL".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetPerformanceStreamMarkerINTEL).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetPerformanceStreamMarkerINTEL".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetPerformanceOverrideINTEL).write(
                load_typed(gdpa(device, c"vkCmdSetPerformanceOverrideINTEL".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAcquirePerformanceConfigurationINTEL).write(
                load_typed(gdpa(
                    device,
                    c"vkAcquirePerformanceConfigurationINTEL".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkReleasePerformanceConfigurationINTEL).write(
                load_typed(gdpa(
                    device,
                    c"vkReleasePerformanceConfigurationINTEL".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueSetPerformanceConfigurationINTEL).write(
                load_typed(gdpa(
                    device,
                    c"vkQueueSetPerformanceConfigurationINTEL".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPerformanceParameterINTEL).write(load_typed(
                gdpa(device, c"vkGetPerformanceParameterINTEL".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetLocalDimmingAMD)
                .write(load_typed(gdpa(device, c"vkSetLocalDimmingAMD".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetBufferDeviceAddressEXT).write(load_typed(
                gdpa(device, c"vkGetBufferDeviceAddressEXT".as_ptr()),
            ));
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAcquireFullScreenExclusiveModeEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkAcquireFullScreenExclusiveModeEXT".as_ptr(),
                )),
            );
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkReleaseFullScreenExclusiveModeEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkReleaseFullScreenExclusiveModeEXT".as_ptr(),
                )),
            );
        }
        #[cfg(target_os = "windows")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceGroupSurfacePresentModes2EXT).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceGroupSurfacePresentModes2EXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetLineStippleEXT)
                .write(load_typed(gdpa(device, c"vkCmdSetLineStippleEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkResetQueryPoolEXT)
                .write(load_typed(gdpa(device, c"vkResetQueryPoolEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetCullModeEXT)
                .write(load_typed(gdpa(device, c"vkCmdSetCullModeEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetFrontFaceEXT)
                .write(load_typed(gdpa(device, c"vkCmdSetFrontFaceEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetPrimitiveTopologyEXT).write(load_typed(
                gdpa(device, c"vkCmdSetPrimitiveTopologyEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetViewportWithCountEXT).write(load_typed(
                gdpa(device, c"vkCmdSetViewportWithCountEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetScissorWithCountEXT).write(load_typed(
                gdpa(device, c"vkCmdSetScissorWithCountEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindVertexBuffers2EXT).write(load_typed(
                gdpa(device, c"vkCmdBindVertexBuffers2EXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthTestEnableEXT).write(load_typed(
                gdpa(device, c"vkCmdSetDepthTestEnableEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthWriteEnableEXT).write(load_typed(
                gdpa(device, c"vkCmdSetDepthWriteEnableEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthCompareOpEXT).write(load_typed(
                gdpa(device, c"vkCmdSetDepthCompareOpEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthBoundsTestEnableEXT).write(
                load_typed(gdpa(device, c"vkCmdSetDepthBoundsTestEnableEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetStencilTestEnableEXT).write(load_typed(
                gdpa(device, c"vkCmdSetStencilTestEnableEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetStencilOpEXT)
                .write(load_typed(gdpa(device, c"vkCmdSetStencilOpEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyMemoryToImageEXT)
                .write(load_typed(gdpa(device, c"vkCopyMemoryToImageEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyImageToMemoryEXT)
                .write(load_typed(gdpa(device, c"vkCopyImageToMemoryEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyImageToImageEXT)
                .write(load_typed(gdpa(device, c"vkCopyImageToImageEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkTransitionImageLayoutEXT).write(load_typed(
                gdpa(device, c"vkTransitionImageLayoutEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageSubresourceLayout2EXT).write(
                load_typed(gdpa(device, c"vkGetImageSubresourceLayout2EXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkReleaseSwapchainImagesEXT).write(load_typed(
                gdpa(device, c"vkReleaseSwapchainImagesEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetGeneratedCommandsMemoryRequirementsNV).write(
                load_typed(gdpa(
                    device,
                    c"vkGetGeneratedCommandsMemoryRequirementsNV".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPreprocessGeneratedCommandsNV).write(
                load_typed(gdpa(device, c"vkCmdPreprocessGeneratedCommandsNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdExecuteGeneratedCommandsNV).write(
                load_typed(gdpa(device, c"vkCmdExecuteGeneratedCommandsNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindPipelineShaderGroupNV).write(load_typed(
                gdpa(device, c"vkCmdBindPipelineShaderGroupNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateIndirectCommandsLayoutNV).write(
                load_typed(gdpa(device, c"vkCreateIndirectCommandsLayoutNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyIndirectCommandsLayoutNV).write(
                load_typed(gdpa(device, c"vkDestroyIndirectCommandsLayoutNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthBias2EXT)
                .write(load_typed(gdpa(device, c"vkCmdSetDepthBias2EXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreatePrivateDataSlotEXT).write(load_typed(
                gdpa(device, c"vkCreatePrivateDataSlotEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyPrivateDataSlotEXT).write(load_typed(
                gdpa(device, c"vkDestroyPrivateDataSlotEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetPrivateDataEXT)
                .write(load_typed(gdpa(device, c"vkSetPrivateDataEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPrivateDataEXT)
                .write(load_typed(gdpa(device, c"vkGetPrivateDataEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueSetPerfHintQCOM)
                .write(load_typed(gdpa(device, c"vkQueueSetPerfHintQCOM".as_ptr())));
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateCudaModuleNV)
                .write(load_typed(gdpa(device, c"vkCreateCudaModuleNV".as_ptr())));
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetCudaModuleCacheNV)
                .write(load_typed(gdpa(device, c"vkGetCudaModuleCacheNV".as_ptr())));
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateCudaFunctionNV)
                .write(load_typed(gdpa(device, c"vkCreateCudaFunctionNV".as_ptr())));
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyCudaModuleNV)
                .write(load_typed(gdpa(device, c"vkDestroyCudaModuleNV".as_ptr())));
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyCudaFunctionNV).write(load_typed(gdpa(
                device,
                c"vkDestroyCudaFunctionNV".as_ptr(),
            )));
        }
        #[cfg(feature = "beta-extensions")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCudaLaunchKernelNV).write(load_typed(gdpa(
                device,
                c"vkCmdCudaLaunchKernelNV".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDispatchTileQCOM)
                .write(load_typed(gdpa(device, c"vkCmdDispatchTileQCOM".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginPerTileExecutionQCOM).write(load_typed(
                gdpa(device, c"vkCmdBeginPerTileExecutionQCOM".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndPerTileExecutionQCOM).write(load_typed(
                gdpa(device, c"vkCmdEndPerTileExecutionQCOM".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetLatencySleepModeLegacyNV).write(load_typed(
                gdpa(device, c"vkSetLatencySleepModeLegacyNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkLatencySleepLegacyNV)
                .write(load_typed(gdpa(device, c"vkLatencySleepLegacyNV".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetLatencyMarkerLegacyNV).write(load_typed(
                gdpa(device, c"vkSetLatencyMarkerLegacyNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetLatencyTimingsLegacyNV).write(load_typed(
                gdpa(device, c"vkGetLatencyTimingsLegacyNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueNotifyOutOfBandLegacyNV).write(load_typed(
                gdpa(device, c"vkQueueNotifyOutOfBandLegacyNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSleepStatusLegacyNV).write(load_typed(gdpa(
                device,
                c"vkGetSleepStatusLegacyNV".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkShutdownLatencyDeviceLegacyNV).write(
                load_typed(gdpa(device, c"vkShutdownLatencyDeviceLegacyNV".as_ptr())),
            );
        }
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkExportMetalObjectsEXT).write(load_typed(gdpa(
                device,
                c"vkExportMetalObjectsEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDescriptorSetLayoutSizeEXT).write(
                load_typed(gdpa(device, c"vkGetDescriptorSetLayoutSizeEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDescriptorSetLayoutBindingOffsetEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDescriptorSetLayoutBindingOffsetEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDescriptorEXT)
                .write(load_typed(gdpa(device, c"vkGetDescriptorEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindDescriptorBuffersEXT).write(load_typed(
                gdpa(device, c"vkCmdBindDescriptorBuffersEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDescriptorBufferOffsetsEXT).write(
                load_typed(gdpa(device, c"vkCmdSetDescriptorBufferOffsetsEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindDescriptorBufferEmbeddedSamplersEXT)
                .write(load_typed(gdpa(
                    device,
                    c"vkCmdBindDescriptorBufferEmbeddedSamplersEXT".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetBufferOpaqueCaptureDescriptorDataEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkGetBufferOpaqueCaptureDescriptorDataEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageOpaqueCaptureDescriptorDataEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkGetImageOpaqueCaptureDescriptorDataEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetImageViewOpaqueCaptureDescriptorDataEXT)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetImageViewOpaqueCaptureDescriptorDataEXT".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSamplerOpaqueCaptureDescriptorDataEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkGetSamplerOpaqueCaptureDescriptorDataEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT
            )
            .write(load_typed(gdpa(
                device,
                c"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetFragmentShadingRateEnumNV).write(
                load_typed(gdpa(device, c"vkCmdSetFragmentShadingRateEnumNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceFaultInfoEXT).write(load_typed(gdpa(
                device,
                c"vkGetDeviceFaultInfoEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetVertexInputEXT)
                .write(load_typed(gdpa(device, c"vkCmdSetVertexInputEXT".as_ptr())));
        }
        #[cfg(target_os = "fuchsia")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryZirconHandleFUCHSIA).write(load_typed(
                gdpa(device, c"vkGetMemoryZirconHandleFUCHSIA".as_ptr()),
            ));
        }
        #[cfg(target_os = "fuchsia")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryZirconHandlePropertiesFUCHSIA).write(
                load_typed(gdpa(
                    device,
                    c"vkGetMemoryZirconHandlePropertiesFUCHSIA".as_ptr(),
                )),
            );
        }
        #[cfg(target_os = "fuchsia")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkImportSemaphoreZirconHandleFUCHSIA).write(
                load_typed(gdpa(
                    device,
                    c"vkImportSemaphoreZirconHandleFUCHSIA".as_ptr(),
                )),
            );
        }
        #[cfg(target_os = "fuchsia")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetSemaphoreZirconHandleFUCHSIA).write(
                load_typed(gdpa(device, c"vkGetSemaphoreZirconHandleFUCHSIA".as_ptr())),
            );
        }
        #[cfg(target_os = "fuchsia")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateBufferCollectionFUCHSIA).write(
                load_typed(gdpa(device, c"vkCreateBufferCollectionFUCHSIA".as_ptr())),
            );
        }
        #[cfg(target_os = "fuchsia")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetBufferCollectionImageConstraintsFUCHSIA)
                .write(load_typed(gdpa(
                    device,
                    c"vkSetBufferCollectionImageConstraintsFUCHSIA".as_ptr(),
                )));
        }
        #[cfg(target_os = "fuchsia")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetBufferCollectionBufferConstraintsFUCHSIA)
                .write(load_typed(gdpa(
                    device,
                    c"vkSetBufferCollectionBufferConstraintsFUCHSIA".as_ptr(),
                )));
        }
        #[cfg(target_os = "fuchsia")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyBufferCollectionFUCHSIA).write(
                load_typed(gdpa(device, c"vkDestroyBufferCollectionFUCHSIA".as_ptr())),
            );
        }
        #[cfg(target_os = "fuchsia")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetBufferCollectionPropertiesFUCHSIA).write(
                load_typed(gdpa(
                    device,
                    c"vkGetBufferCollectionPropertiesFUCHSIA".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSubpassShadingHUAWEI).write(load_typed(
                gdpa(device, c"vkCmdSubpassShadingHUAWEI".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindInvocationMaskHUAWEI).write(load_typed(
                gdpa(device, c"vkCmdBindInvocationMaskHUAWEI".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryRemoteAddressNV).write(load_typed(
                gdpa(device, c"vkGetMemoryRemoteAddressNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPipelinePropertiesEXT).write(load_typed(
                gdpa(device, c"vkGetPipelinePropertiesEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetPatchControlPointsEXT).write(load_typed(
                gdpa(device, c"vkCmdSetPatchControlPointsEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetRasterizerDiscardEnableEXT).write(
                load_typed(gdpa(device, c"vkCmdSetRasterizerDiscardEnableEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthBiasEnableEXT).write(load_typed(
                gdpa(device, c"vkCmdSetDepthBiasEnableEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetLogicOpEXT)
                .write(load_typed(gdpa(device, c"vkCmdSetLogicOpEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetPrimitiveRestartEnableEXT).write(
                load_typed(gdpa(device, c"vkCmdSetPrimitiveRestartEnableEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetColorWriteEnableEXT).write(load_typed(
                gdpa(device, c"vkCmdSetColorWriteEnableEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawMultiEXT)
                .write(load_typed(gdpa(device, c"vkCmdDrawMultiEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawMultiIndexedEXT).write(load_typed(gdpa(
                device,
                c"vkCmdDrawMultiIndexedEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateMicromapEXT)
                .write(load_typed(gdpa(device, c"vkCreateMicromapEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyMicromapEXT)
                .write(load_typed(gdpa(device, c"vkDestroyMicromapEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBuildMicromapsEXT)
                .write(load_typed(gdpa(device, c"vkCmdBuildMicromapsEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBuildMicromapsEXT)
                .write(load_typed(gdpa(device, c"vkBuildMicromapsEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyMicromapEXT)
                .write(load_typed(gdpa(device, c"vkCopyMicromapEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyMicromapToMemoryEXT).write(load_typed(
                gdpa(device, c"vkCopyMicromapToMemoryEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyMemoryToMicromapEXT).write(load_typed(
                gdpa(device, c"vkCopyMemoryToMicromapEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkWriteMicromapsPropertiesEXT).write(load_typed(
                gdpa(device, c"vkWriteMicromapsPropertiesEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyMicromapEXT)
                .write(load_typed(gdpa(device, c"vkCmdCopyMicromapEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyMicromapToMemoryEXT).write(load_typed(
                gdpa(device, c"vkCmdCopyMicromapToMemoryEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyMemoryToMicromapEXT).write(load_typed(
                gdpa(device, c"vkCmdCopyMemoryToMicromapEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWriteMicromapsPropertiesEXT).write(
                load_typed(gdpa(device, c"vkCmdWriteMicromapsPropertiesEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceMicromapCompatibilityEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceMicromapCompatibilityEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMicromapBuildSizesEXT).write(load_typed(
                gdpa(device, c"vkGetMicromapBuildSizesEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawClusterHUAWEI)
                .write(load_typed(gdpa(device, c"vkCmdDrawClusterHUAWEI".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawClusterIndirectHUAWEI).write(load_typed(
                gdpa(device, c"vkCmdDrawClusterIndirectHUAWEI".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetDeviceMemoryPriorityEXT).write(load_typed(
                gdpa(device, c"vkSetDeviceMemoryPriorityEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDispatchParametersARM).write(load_typed(
                gdpa(device, c"vkCmdSetDispatchParametersARM".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDescriptorSetLayoutHostMappingInfoVALVE)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetDescriptorSetLayoutHostMappingInfoVALVE".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDescriptorSetHostMappingVALVE).write(
                load_typed(gdpa(device, c"vkGetDescriptorSetHostMappingVALVE".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyMemoryIndirectNV).write(load_typed(
                gdpa(device, c"vkCmdCopyMemoryIndirectNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyMemoryToImageIndirectNV).write(
                load_typed(gdpa(device, c"vkCmdCopyMemoryToImageIndirectNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDecompressMemoryNV).write(load_typed(gdpa(
                device,
                c"vkCmdDecompressMemoryNV".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDecompressMemoryIndirectCountNV).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdDecompressMemoryIndirectCountNV".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPipelineIndirectMemoryRequirementsNV).write(
                load_typed(gdpa(
                    device,
                    c"vkGetPipelineIndirectMemoryRequirementsNV".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdUpdatePipelineIndirectBufferNV).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdUpdatePipelineIndirectBufferNV".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetPipelineIndirectDeviceAddressNV).write(
                load_typed(gdpa(
                    device,
                    c"vkGetPipelineIndirectDeviceAddressNV".as_ptr(),
                )),
            );
        }
        #[cfg(target_env = "ohos")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetNativeBufferPropertiesOHOS).write(
                load_typed(gdpa(device, c"vkGetNativeBufferPropertiesOHOS".as_ptr())),
            );
        }
        #[cfg(target_env = "ohos")]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryNativeBufferOHOS).write(load_typed(
                gdpa(device, c"vkGetMemoryNativeBufferOHOS".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthClampEnableEXT).write(load_typed(
                gdpa(device, c"vkCmdSetDepthClampEnableEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetPolygonModeEXT)
                .write(load_typed(gdpa(device, c"vkCmdSetPolygonModeEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetRasterizationSamplesEXT).write(
                load_typed(gdpa(device, c"vkCmdSetRasterizationSamplesEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetSampleMaskEXT)
                .write(load_typed(gdpa(device, c"vkCmdSetSampleMaskEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetAlphaToCoverageEnableEXT).write(
                load_typed(gdpa(device, c"vkCmdSetAlphaToCoverageEnableEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetAlphaToOneEnableEXT).write(load_typed(
                gdpa(device, c"vkCmdSetAlphaToOneEnableEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetLogicOpEnableEXT).write(load_typed(gdpa(
                device,
                c"vkCmdSetLogicOpEnableEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetColorBlendEnableEXT).write(load_typed(
                gdpa(device, c"vkCmdSetColorBlendEnableEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetColorBlendEquationEXT).write(load_typed(
                gdpa(device, c"vkCmdSetColorBlendEquationEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetColorWriteMaskEXT).write(load_typed(
                gdpa(device, c"vkCmdSetColorWriteMaskEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetTessellationDomainOriginEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetTessellationDomainOriginEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetRasterizationStreamEXT).write(load_typed(
                gdpa(device, c"vkCmdSetRasterizationStreamEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetConservativeRasterizationModeEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetConservativeRasterizationModeEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetExtraPrimitiveOverestimationSizeEXT)
                .write(load_typed(gdpa(
                    device,
                    c"vkCmdSetExtraPrimitiveOverestimationSizeEXT".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthClipEnableEXT).write(load_typed(
                gdpa(device, c"vkCmdSetDepthClipEnableEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetSampleLocationsEnableEXT).write(
                load_typed(gdpa(device, c"vkCmdSetSampleLocationsEnableEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetColorBlendAdvancedEXT).write(load_typed(
                gdpa(device, c"vkCmdSetColorBlendAdvancedEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetProvokingVertexModeEXT).write(load_typed(
                gdpa(device, c"vkCmdSetProvokingVertexModeEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetLineRasterizationModeEXT).write(
                load_typed(gdpa(device, c"vkCmdSetLineRasterizationModeEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetLineStippleEnableEXT).write(load_typed(
                gdpa(device, c"vkCmdSetLineStippleEnableEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthClipNegativeOneToOneEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetDepthClipNegativeOneToOneEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetViewportWScalingEnableNV).write(
                load_typed(gdpa(device, c"vkCmdSetViewportWScalingEnableNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetViewportSwizzleNV).write(load_typed(
                gdpa(device, c"vkCmdSetViewportSwizzleNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetCoverageToColorEnableNV).write(
                load_typed(gdpa(device, c"vkCmdSetCoverageToColorEnableNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetCoverageToColorLocationNV).write(
                load_typed(gdpa(device, c"vkCmdSetCoverageToColorLocationNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetCoverageModulationModeNV).write(
                load_typed(gdpa(device, c"vkCmdSetCoverageModulationModeNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetCoverageModulationTableEnableNV).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetCoverageModulationTableEnableNV".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetCoverageModulationTableNV).write(
                load_typed(gdpa(device, c"vkCmdSetCoverageModulationTableNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetShadingRateImageEnableNV).write(
                load_typed(gdpa(device, c"vkCmdSetShadingRateImageEnableNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetRepresentativeFragmentTestEnableNV).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetRepresentativeFragmentTestEnableNV".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetCoverageReductionModeNV).write(
                load_typed(gdpa(device, c"vkCmdSetCoverageReductionModeNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateTensorARM)
                .write(load_typed(gdpa(device, c"vkCreateTensorARM".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyTensorARM)
                .write(load_typed(gdpa(device, c"vkDestroyTensorARM".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateTensorViewARM)
                .write(load_typed(gdpa(device, c"vkCreateTensorViewARM".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyTensorViewARM)
                .write(load_typed(gdpa(device, c"vkDestroyTensorViewARM".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetTensorMemoryRequirementsARM).write(
                load_typed(gdpa(device, c"vkGetTensorMemoryRequirementsARM".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBindTensorMemoryARM)
                .write(load_typed(gdpa(device, c"vkBindTensorMemoryARM".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceTensorMemoryRequirementsARM).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDeviceTensorMemoryRequirementsARM".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyTensorARM)
                .write(load_typed(gdpa(device, c"vkCmdCopyTensorARM".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetTensorOpaqueCaptureDescriptorDataARM).write(
                load_typed(gdpa(
                    device,
                    c"vkGetTensorOpaqueCaptureDescriptorDataARM".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetTensorViewOpaqueCaptureDescriptorDataARM)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetTensorViewOpaqueCaptureDescriptorDataARM".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetShaderModuleIdentifierEXT).write(load_typed(
                gdpa(device, c"vkGetShaderModuleIdentifierEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetShaderModuleCreateInfoIdentifierEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkGetShaderModuleCreateInfoIdentifierEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateOpticalFlowSessionNV).write(load_typed(
                gdpa(device, c"vkCreateOpticalFlowSessionNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyOpticalFlowSessionNV).write(load_typed(
                gdpa(device, c"vkDestroyOpticalFlowSessionNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBindOpticalFlowSessionImageNV).write(
                load_typed(gdpa(device, c"vkBindOpticalFlowSessionImageNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdOpticalFlowExecuteNV).write(load_typed(
                gdpa(device, c"vkCmdOpticalFlowExecuteNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkAntiLagUpdateAMD)
                .write(load_typed(gdpa(device, c"vkAntiLagUpdateAMD".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateShadersEXT)
                .write(load_typed(gdpa(device, c"vkCreateShadersEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyShaderEXT)
                .write(load_typed(gdpa(device, c"vkDestroyShaderEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetShaderBinaryDataEXT).write(load_typed(gdpa(
                device,
                c"vkGetShaderBinaryDataEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindShadersEXT)
                .write(load_typed(gdpa(device, c"vkCmdBindShadersEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetDepthClampRangeEXT).write(load_typed(
                gdpa(device, c"vkCmdSetDepthClampRangeEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetFramebufferTilePropertiesQCOM).write(
                load_typed(gdpa(device, c"vkGetFramebufferTilePropertiesQCOM".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDynamicRenderingTilePropertiesQCOM).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDynamicRenderingTilePropertiesQCOM".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkConvertCooperativeVectorMatrixNV).write(
                load_typed(gdpa(device, c"vkConvertCooperativeVectorMatrixNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdConvertCooperativeVectorMatrixNV).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdConvertCooperativeVectorMatrixNV".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetLatencySleepModeNV).write(load_typed(gdpa(
                device,
                c"vkSetLatencySleepModeNV".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkLatencySleepNV)
                .write(load_typed(gdpa(device, c"vkLatencySleepNV".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkSetLatencyMarkerNV)
                .write(load_typed(gdpa(device, c"vkSetLatencyMarkerNV".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetLatencyTimingsNV)
                .write(load_typed(gdpa(device, c"vkGetLatencyTimingsNV".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkQueueNotifyOutOfBandNV).write(load_typed(gdpa(
                device,
                c"vkQueueNotifyOutOfBandNV".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDataGraphPipelinesARM).write(load_typed(
                gdpa(device, c"vkCreateDataGraphPipelinesARM".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateDataGraphPipelineSessionARM).write(
                load_typed(gdpa(
                    device,
                    c"vkCreateDataGraphPipelineSessionARM".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetDataGraphPipelineSessionBindPointRequirementsARM
            )
            .write(load_typed(gdpa(
                device,
                c"vkGetDataGraphPipelineSessionBindPointRequirementsARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetDataGraphPipelineSessionMemoryRequirementsARM
            )
            .write(load_typed(gdpa(
                device,
                c"vkGetDataGraphPipelineSessionMemoryRequirementsARM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBindDataGraphPipelineSessionMemoryARM).write(
                load_typed(gdpa(
                    device,
                    c"vkBindDataGraphPipelineSessionMemoryARM".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyDataGraphPipelineSessionARM).write(
                load_typed(gdpa(
                    device,
                    c"vkDestroyDataGraphPipelineSessionARM".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDispatchDataGraphARM).write(load_typed(
                gdpa(device, c"vkCmdDispatchDataGraphARM".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDataGraphPipelineAvailablePropertiesARM)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetDataGraphPipelineAvailablePropertiesARM".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDataGraphPipelinePropertiesARM).write(
                load_typed(gdpa(
                    device,
                    c"vkGetDataGraphPipelinePropertiesARM".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetAttachmentFeedbackLoopEnableEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetAttachmentFeedbackLoopEnableEXT".as_ptr(),
                )),
            );
        }
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetScreenBufferPropertiesQNX).write(load_typed(
                gdpa(device, c"vkGetScreenBufferPropertiesQNX".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBindTileMemoryQCOM).write(load_typed(gdpa(
                device,
                c"vkCmdBindTileMemoryQCOM".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDecompressMemoryEXT).write(load_typed(gdpa(
                device,
                c"vkCmdDecompressMemoryEXT".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDecompressMemoryIndirectCountEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdDecompressMemoryIndirectCountEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateExternalComputeQueueNV).write(load_typed(
                gdpa(device, c"vkCreateExternalComputeQueueNV".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyExternalComputeQueueNV).write(
                load_typed(gdpa(device, c"vkDestroyExternalComputeQueueNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetExternalComputeQueueDataNV).write(
                load_typed(gdpa(device, c"vkGetExternalComputeQueueDataNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetClusterAccelerationStructureBuildSizesNV)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetClusterAccelerationStructureBuildSizesNV".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBuildClusterAccelerationStructureIndirectNV)
                .write(load_typed(gdpa(
                    device,
                    c"vkCmdBuildClusterAccelerationStructureIndirectNV".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!(
                (*table_ptr).vkGetPartitionedAccelerationStructuresBuildSizesNV
            )
            .write(load_typed(gdpa(
                device,
                c"vkGetPartitionedAccelerationStructuresBuildSizesNV".as_ptr(),
            )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBuildPartitionedAccelerationStructuresNV)
                .write(load_typed(gdpa(
                    device,
                    c"vkCmdBuildPartitionedAccelerationStructuresNV".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetGeneratedCommandsMemoryRequirementsEXT)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetGeneratedCommandsMemoryRequirementsEXT".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdPreprocessGeneratedCommandsEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdPreprocessGeneratedCommandsEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdExecuteGeneratedCommandsEXT).write(
                load_typed(gdpa(device, c"vkCmdExecuteGeneratedCommandsEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateIndirectCommandsLayoutEXT).write(
                load_typed(gdpa(device, c"vkCreateIndirectCommandsLayoutEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyIndirectCommandsLayoutEXT).write(
                load_typed(gdpa(device, c"vkDestroyIndirectCommandsLayoutEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateIndirectExecutionSetEXT).write(
                load_typed(gdpa(device, c"vkCreateIndirectExecutionSetEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyIndirectExecutionSetEXT).write(
                load_typed(gdpa(device, c"vkDestroyIndirectExecutionSetEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkUpdateIndirectExecutionSetPipelineEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkUpdateIndirectExecutionSetPipelineEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkUpdateIndirectExecutionSetShaderEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkUpdateIndirectExecutionSetShaderEXT".as_ptr(),
                )),
            );
        }
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryMetalHandleEXT).write(load_typed(
                gdpa(device, c"vkGetMemoryMetalHandleEXT".as_ptr()),
            ));
        }
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetMemoryMetalHandlePropertiesEXT).write(
                load_typed(gdpa(
                    device,
                    c"vkGetMemoryMetalHandlePropertiesEXT".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateShaderInstrumentationARM).write(
                load_typed(gdpa(device, c"vkCreateShaderInstrumentationARM".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyShaderInstrumentationARM).write(
                load_typed(gdpa(device, c"vkDestroyShaderInstrumentationARM".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginShaderInstrumentationARM).write(
                load_typed(gdpa(device, c"vkCmdBeginShaderInstrumentationARM".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndShaderInstrumentationARM).write(
                load_typed(gdpa(device, c"vkCmdEndShaderInstrumentationARM".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetShaderInstrumentationValuesARM).write(
                load_typed(gdpa(
                    device,
                    c"vkGetShaderInstrumentationValuesARM".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkClearShaderInstrumentationMetricsARM).write(
                load_typed(gdpa(
                    device,
                    c"vkClearShaderInstrumentationMetricsARM".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdEndRendering2EXT)
                .write(load_typed(gdpa(device, c"vkCmdEndRendering2EXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBeginCustomResolveEXT).write(load_typed(
                gdpa(device, c"vkCmdBeginCustomResolveEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetComputeOccupancyPriorityNV).write(
                load_typed(gdpa(device, c"vkCmdSetComputeOccupancyPriorityNV".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetPrimitiveRestartIndexEXT).write(
                load_typed(gdpa(device, c"vkCmdSetPrimitiveRestartIndexEXT".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateAccelerationStructureKHR).write(
                load_typed(gdpa(device, c"vkCreateAccelerationStructureKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkDestroyAccelerationStructureKHR).write(
                load_typed(gdpa(device, c"vkDestroyAccelerationStructureKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBuildAccelerationStructuresKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdBuildAccelerationStructuresKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdBuildAccelerationStructuresIndirectKHR)
                .write(load_typed(gdpa(
                    device,
                    c"vkCmdBuildAccelerationStructuresIndirectKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkBuildAccelerationStructuresKHR).write(
                load_typed(gdpa(device, c"vkBuildAccelerationStructuresKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyAccelerationStructureKHR).write(load_typed(
                gdpa(device, c"vkCopyAccelerationStructureKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyAccelerationStructureToMemoryKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCopyAccelerationStructureToMemoryKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCopyMemoryToAccelerationStructureKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCopyMemoryToAccelerationStructureKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkWriteAccelerationStructuresPropertiesKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkWriteAccelerationStructuresPropertiesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyAccelerationStructureKHR).write(
                load_typed(gdpa(device, c"vkCmdCopyAccelerationStructureKHR".as_ptr())),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyAccelerationStructureToMemoryKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdCopyAccelerationStructureToMemoryKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdCopyMemoryToAccelerationStructureKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdCopyMemoryToAccelerationStructureKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetAccelerationStructureDeviceAddressKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetAccelerationStructureDeviceAddressKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdWriteAccelerationStructuresPropertiesKHR)
                .write(load_typed(gdpa(
                    device,
                    c"vkCmdWriteAccelerationStructuresPropertiesKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetDeviceAccelerationStructureCompatibilityKHR)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetDeviceAccelerationStructureCompatibilityKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetAccelerationStructureBuildSizesKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetAccelerationStructureBuildSizesKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdTraceRaysKHR)
                .write(load_typed(gdpa(device, c"vkCmdTraceRaysKHR".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCreateRayTracingPipelinesKHR).write(load_typed(
                gdpa(device, c"vkCreateRayTracingPipelinesKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetRayTracingCaptureReplayShaderGroupHandlesKHR)
                .write(load_typed(gdpa(
                    device,
                    c"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR".as_ptr(),
                )));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdTraceRaysIndirectKHR).write(load_typed(
                gdpa(device, c"vkCmdTraceRaysIndirectKHR".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkGetRayTracingShaderGroupStackSizeKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkGetRayTracingShaderGroupStackSizeKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdSetRayTracingPipelineStackSizeKHR).write(
                load_typed(gdpa(
                    device,
                    c"vkCmdSetRayTracingPipelineStackSizeKHR".as_ptr(),
                )),
            );
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawMeshTasksEXT)
                .write(load_typed(gdpa(device, c"vkCmdDrawMeshTasksEXT".as_ptr())));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawMeshTasksIndirectEXT).write(load_typed(
                gdpa(device, c"vkCmdDrawMeshTasksIndirectEXT".as_ptr()),
            ));
        }
        unsafe {
            core::ptr::addr_of_mut!((*table_ptr).vkCmdDrawMeshTasksIndirectCountEXT).write(
                load_typed(gdpa(device, c"vkCmdDrawMeshTasksIndirectCountEXT".as_ptr())),
            );
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
impl IcdDeviceTerminatorDispatchTable {
    pub(crate) unsafe fn load(
        gdpa: vk::PFN_vkGetDeviceProcAddr,
        device: vk::VkDevice,
        mut available: impl FnMut(&CStr) -> bool,
    ) -> Self {
        Self {
            vkDestroyDevice: if available(c"vkDestroyDevice") {
                unsafe { load_typed(gdpa(device, c"vkDestroyDevice".as_ptr())) }
            } else {
                None
            },
            vkCreateSwapchainKHR: if available(c"vkCreateSwapchainKHR") {
                unsafe { load_typed(gdpa(device, c"vkCreateSwapchainKHR".as_ptr())) }
            } else {
                None
            },
            vkGetDeviceGroupSurfacePresentModesKHR: if available(
                c"vkGetDeviceGroupSurfacePresentModesKHR",
            ) {
                unsafe {
                    load_typed(gdpa(
                        device,
                        c"vkGetDeviceGroupSurfacePresentModesKHR".as_ptr(),
                    ))
                }
            } else {
                None
            },
            vkCreateSharedSwapchainsKHR: if available(c"vkCreateSharedSwapchainsKHR") {
                unsafe { load_typed(gdpa(device, c"vkCreateSharedSwapchainsKHR".as_ptr())) }
            } else {
                None
            },
            vkDebugMarkerSetObjectTagEXT: if available(c"vkDebugMarkerSetObjectTagEXT") {
                unsafe { load_typed(gdpa(device, c"vkDebugMarkerSetObjectTagEXT".as_ptr())) }
            } else {
                None
            },
            vkDebugMarkerSetObjectNameEXT: if available(c"vkDebugMarkerSetObjectNameEXT") {
                unsafe { load_typed(gdpa(device, c"vkDebugMarkerSetObjectNameEXT".as_ptr())) }
            } else {
                None
            },
            vkSetDebugUtilsObjectNameEXT: if available(c"vkSetDebugUtilsObjectNameEXT") {
                unsafe { load_typed(gdpa(device, c"vkSetDebugUtilsObjectNameEXT".as_ptr())) }
            } else {
                None
            },
            vkSetDebugUtilsObjectTagEXT: if available(c"vkSetDebugUtilsObjectTagEXT") {
                unsafe { load_typed(gdpa(device, c"vkSetDebugUtilsObjectTagEXT".as_ptr())) }
            } else {
                None
            },
            vkQueueBeginDebugUtilsLabelEXT: if available(c"vkQueueBeginDebugUtilsLabelEXT") {
                unsafe { load_typed(gdpa(device, c"vkQueueBeginDebugUtilsLabelEXT".as_ptr())) }
            } else {
                None
            },
            vkQueueEndDebugUtilsLabelEXT: if available(c"vkQueueEndDebugUtilsLabelEXT") {
                unsafe { load_typed(gdpa(device, c"vkQueueEndDebugUtilsLabelEXT".as_ptr())) }
            } else {
                None
            },
            vkQueueInsertDebugUtilsLabelEXT: if available(c"vkQueueInsertDebugUtilsLabelEXT") {
                unsafe { load_typed(gdpa(device, c"vkQueueInsertDebugUtilsLabelEXT".as_ptr())) }
            } else {
                None
            },
            vkCmdBeginDebugUtilsLabelEXT: if available(c"vkCmdBeginDebugUtilsLabelEXT") {
                unsafe { load_typed(gdpa(device, c"vkCmdBeginDebugUtilsLabelEXT".as_ptr())) }
            } else {
                None
            },
            vkCmdEndDebugUtilsLabelEXT: if available(c"vkCmdEndDebugUtilsLabelEXT") {
                unsafe { load_typed(gdpa(device, c"vkCmdEndDebugUtilsLabelEXT".as_ptr())) }
            } else {
                None
            },
            vkCmdInsertDebugUtilsLabelEXT: if available(c"vkCmdInsertDebugUtilsLabelEXT") {
                unsafe { load_typed(gdpa(device, c"vkCmdInsertDebugUtilsLabelEXT".as_ptr())) }
            } else {
                None
            },
            #[cfg(target_os = "windows")]
            vkGetDeviceGroupSurfacePresentModes2EXT: if available(
                c"vkGetDeviceGroupSurfacePresentModes2EXT",
            ) {
                unsafe {
                    load_typed(gdpa(
                        device,
                        c"vkGetDeviceGroupSurfacePresentModes2EXT".as_ptr(),
                    ))
                }
            } else {
                None
            },
        }
    }
}
