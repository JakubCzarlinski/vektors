// Generated from registry/vk.xml by vk-loader-codegen. Do not edit.

use super::commands::COMMAND_DEVICE_DISPATCH_OFFSETS;
use super::dispatch_tables::IcdDeviceTerminatorDispatchTable;
use super::dispatch_tables::LayerDeviceDispatchTable;
use super::terminators::terminator_vkAcquireDrmDisplayEXT;
#[cfg(target_os = "windows")]
use super::terminators::terminator_vkAcquireWinrtDisplayNV;
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
use super::terminators::terminator_vkAcquireXlibDisplayEXT;
#[cfg(target_os = "android")]
use super::terminators::terminator_vkCreateAndroidSurfaceKHR;
#[cfg(feature = "wsi-directfb")]
use super::terminators::terminator_vkCreateDirectFBSurfaceEXT;
use super::terminators::terminator_vkCreateDisplayModeKHR;
use super::terminators::terminator_vkCreateDisplayPlaneSurfaceKHR;
use super::terminators::terminator_vkCreateHeadlessSurfaceEXT;
#[cfg(target_os = "ios")]
use super::terminators::terminator_vkCreateIOSSurfaceMVK;
#[cfg(target_os = "fuchsia")]
use super::terminators::terminator_vkCreateImagePipeSurfaceFUCHSIA;
#[cfg(target_os = "macos")]
use super::terminators::terminator_vkCreateMacOSSurfaceMVK;
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
use super::terminators::terminator_vkCreateMetalSurfaceEXT;
#[cfg(any(target_os = "nto", target_os = "qnx"))]
use super::terminators::terminator_vkCreateScreenSurfaceQNX;
#[cfg(feature = "platform-ggp")]
use super::terminators::terminator_vkCreateStreamDescriptorSurfaceGGP;
#[cfg(target_env = "ohos")]
use super::terminators::terminator_vkCreateSurfaceOHOS;
#[cfg(feature = "platform-ubm")]
use super::terminators::terminator_vkCreateUbmSurfaceSEC;
#[cfg(feature = "platform-vi")]
use super::terminators::terminator_vkCreateViSurfaceNN;
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
use super::terminators::terminator_vkCreateWaylandSurfaceKHR;
#[cfg(target_os = "windows")]
use super::terminators::terminator_vkCreateWin32SurfaceKHR;
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
use super::terminators::terminator_vkCreateXcbSurfaceKHR;
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
use super::terminators::terminator_vkCreateXlibSurfaceKHR;
use super::terminators::terminator_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM;
use super::terminators::terminator_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR;
use super::terminators::terminator_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM;
use super::terminators::terminator_vkGetDisplayModePropertiesKHR;
use super::terminators::terminator_vkGetDisplayPlaneCapabilitiesKHR;
use super::terminators::terminator_vkGetDisplayPlaneSupportedDisplaysKHR;
use super::terminators::terminator_vkGetDrmDisplayEXT;
use super::terminators::terminator_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT;
use super::terminators::terminator_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR;
use super::terminators::terminator_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV;
use super::terminators::terminator_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT;
use super::terminators::terminator_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV;
use super::terminators::terminator_vkGetPhysicalDeviceCooperativeVectorPropertiesNV;
use super::terminators::terminator_vkGetPhysicalDeviceDescriptorSizeEXT;
#[cfg(feature = "wsi-directfb")]
use super::terminators::terminator_vkGetPhysicalDeviceDirectFBPresentationSupportEXT;
use super::terminators::terminator_vkGetPhysicalDeviceDisplayPlanePropertiesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceDisplayPropertiesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceExternalBufferProperties;
use super::terminators::terminator_vkGetPhysicalDeviceExternalBufferPropertiesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceExternalFenceProperties;
use super::terminators::terminator_vkGetPhysicalDeviceExternalFencePropertiesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceExternalImageFormatPropertiesNV;
use super::terminators::terminator_vkGetPhysicalDeviceExternalSemaphoreProperties;
use super::terminators::terminator_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceExternalTensorPropertiesARM;
use super::terminators::terminator_vkGetPhysicalDeviceFeatures;
use super::terminators::terminator_vkGetPhysicalDeviceFeatures2;
use super::terminators::terminator_vkGetPhysicalDeviceFeatures2KHR;
use super::terminators::terminator_vkGetPhysicalDeviceFormatProperties;
use super::terminators::terminator_vkGetPhysicalDeviceFormatProperties2;
use super::terminators::terminator_vkGetPhysicalDeviceFormatProperties2KHR;
use super::terminators::terminator_vkGetPhysicalDeviceFragmentShadingRatesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceImageFormatProperties;
use super::terminators::terminator_vkGetPhysicalDeviceImageFormatProperties2;
use super::terminators::terminator_vkGetPhysicalDeviceImageFormatProperties2KHR;
use super::terminators::terminator_vkGetPhysicalDeviceMemoryProperties;
use super::terminators::terminator_vkGetPhysicalDeviceMemoryProperties2;
use super::terminators::terminator_vkGetPhysicalDeviceMemoryProperties2KHR;
use super::terminators::terminator_vkGetPhysicalDeviceMultisamplePropertiesEXT;
use super::terminators::terminator_vkGetPhysicalDeviceOpticalFlowImageFormatsNV;
use super::terminators::terminator_vkGetPhysicalDevicePresentRectanglesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceProperties;
use super::terminators::terminator_vkGetPhysicalDeviceProperties2;
use super::terminators::terminator_vkGetPhysicalDeviceProperties2KHR;
use super::terminators::terminator_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM;
use super::terminators::terminator_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM;
use super::terminators::terminator_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM;
use super::terminators::terminator_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM;
use super::terminators::terminator_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceQueueFamilyProperties;
use super::terminators::terminator_vkGetPhysicalDeviceQueueFamilyProperties2;
use super::terminators::terminator_vkGetPhysicalDeviceQueueFamilyProperties2KHR;
#[cfg(any(target_os = "nto", target_os = "qnx"))]
use super::terminators::terminator_vkGetPhysicalDeviceScreenPresentationSupportQNX;
use super::terminators::terminator_vkGetPhysicalDeviceSparseImageFormatProperties;
use super::terminators::terminator_vkGetPhysicalDeviceSparseImageFormatProperties2;
use super::terminators::terminator_vkGetPhysicalDeviceSparseImageFormatProperties2KHR;
use super::terminators::terminator_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV;
use super::terminators::terminator_vkGetPhysicalDeviceSurfaceCapabilitiesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceSurfaceFormatsKHR;
#[cfg(target_os = "windows")]
use super::terminators::terminator_vkGetPhysicalDeviceSurfacePresentModes2EXT;
use super::terminators::terminator_vkGetPhysicalDeviceSurfacePresentModesKHR;
#[cfg(feature = "platform-ubm")]
use super::terminators::terminator_vkGetPhysicalDeviceUbmPresentationSupportSEC;
use super::terminators::terminator_vkGetPhysicalDeviceVideoCapabilitiesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR;
use super::terminators::terminator_vkGetPhysicalDeviceVideoFormatPropertiesKHR;
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
use super::terminators::terminator_vkGetPhysicalDeviceWaylandPresentationSupportKHR;
#[cfg(target_os = "windows")]
use super::terminators::terminator_vkGetPhysicalDeviceWin32PresentationSupportKHR;
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
use super::terminators::terminator_vkGetPhysicalDeviceXcbPresentationSupportKHR;
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
use super::terminators::terminator_vkGetPhysicalDeviceXlibPresentationSupportKHR;
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
use super::terminators::terminator_vkGetRandROutputDisplayEXT;
#[cfg(target_os = "windows")]
use super::terminators::terminator_vkGetWinrtDisplayNV;
use super::terminators::terminator_vkReleaseDisplayEXT;
use super::trampolines::vkAcquireDrmDisplayEXT;
#[cfg(target_os = "windows")]
use super::trampolines::vkAcquireFullScreenExclusiveModeEXT;
use super::trampolines::vkAcquireNextImage2KHR;
use super::trampolines::vkAcquireNextImageKHR;
use super::trampolines::vkAcquirePerformanceConfigurationINTEL;
use super::trampolines::vkAcquireProfilingLockKHR;
#[cfg(target_os = "windows")]
use super::trampolines::vkAcquireWinrtDisplayNV;
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
use super::trampolines::vkAcquireXlibDisplayEXT;
use super::trampolines::vkAllocateCommandBuffers;
use super::trampolines::vkAllocateDescriptorSets;
use super::trampolines::vkAllocateMemory;
use super::trampolines::vkAntiLagUpdateAMD;
use super::trampolines::vkBeginCommandBuffer;
use super::trampolines::vkBindAccelerationStructureMemoryNV;
use super::trampolines::vkBindBufferMemory;
use super::trampolines::vkBindBufferMemory2;
use super::trampolines::vkBindBufferMemory2KHR;
use super::trampolines::vkBindDataGraphPipelineSessionMemoryARM;
use super::trampolines::vkBindImageMemory;
use super::trampolines::vkBindImageMemory2;
use super::trampolines::vkBindImageMemory2KHR;
use super::trampolines::vkBindOpticalFlowSessionImageNV;
use super::trampolines::vkBindTensorMemoryARM;
use super::trampolines::vkBindVideoSessionMemoryKHR;
use super::trampolines::vkBuildAccelerationStructuresKHR;
use super::trampolines::vkBuildMicromapsEXT;
use super::trampolines::vkClearShaderInstrumentationMetricsARM;
use super::trampolines::vkCmdBeginConditionalRendering2EXT;
use super::trampolines::vkCmdBeginConditionalRenderingEXT;
use super::trampolines::vkCmdBeginCustomResolveEXT;
use super::trampolines::vkCmdBeginDebugUtilsLabelEXT;
use super::trampolines::vkCmdBeginGpaSampleAMD;
use super::trampolines::vkCmdBeginGpaSessionAMD;
use super::trampolines::vkCmdBeginPerTileExecutionQCOM;
use super::trampolines::vkCmdBeginQuery;
use super::trampolines::vkCmdBeginQueryIndexedEXT;
use super::trampolines::vkCmdBeginRenderPass;
use super::trampolines::vkCmdBeginRenderPass2;
use super::trampolines::vkCmdBeginRenderPass2KHR;
use super::trampolines::vkCmdBeginRendering;
use super::trampolines::vkCmdBeginRenderingKHR;
use super::trampolines::vkCmdBeginShaderInstrumentationARM;
use super::trampolines::vkCmdBeginTransformFeedback2EXT;
use super::trampolines::vkCmdBeginTransformFeedbackEXT;
use super::trampolines::vkCmdBeginVideoCodingKHR;
use super::trampolines::vkCmdBindDescriptorBufferEmbeddedSamplers2EXT;
use super::trampolines::vkCmdBindDescriptorBufferEmbeddedSamplersEXT;
use super::trampolines::vkCmdBindDescriptorBuffersEXT;
use super::trampolines::vkCmdBindDescriptorSets;
use super::trampolines::vkCmdBindDescriptorSets2;
use super::trampolines::vkCmdBindDescriptorSets2KHR;
use super::trampolines::vkCmdBindIndexBuffer;
use super::trampolines::vkCmdBindIndexBuffer2;
use super::trampolines::vkCmdBindIndexBuffer2KHR;
use super::trampolines::vkCmdBindIndexBuffer3KHR;
use super::trampolines::vkCmdBindInvocationMaskHUAWEI;
use super::trampolines::vkCmdBindPipeline;
use super::trampolines::vkCmdBindPipelineShaderGroupNV;
use super::trampolines::vkCmdBindResourceHeapEXT;
use super::trampolines::vkCmdBindSamplerHeapEXT;
use super::trampolines::vkCmdBindShadersEXT;
use super::trampolines::vkCmdBindShadingRateImageNV;
use super::trampolines::vkCmdBindTileMemoryQCOM;
use super::trampolines::vkCmdBindTransformFeedbackBuffers2EXT;
use super::trampolines::vkCmdBindTransformFeedbackBuffersEXT;
use super::trampolines::vkCmdBindVertexBuffers;
use super::trampolines::vkCmdBindVertexBuffers2;
use super::trampolines::vkCmdBindVertexBuffers2EXT;
use super::trampolines::vkCmdBindVertexBuffers3KHR;
use super::trampolines::vkCmdBlitImage;
use super::trampolines::vkCmdBlitImage2;
use super::trampolines::vkCmdBlitImage2KHR;
use super::trampolines::vkCmdBuildAccelerationStructureNV;
use super::trampolines::vkCmdBuildAccelerationStructuresIndirectKHR;
use super::trampolines::vkCmdBuildAccelerationStructuresKHR;
use super::trampolines::vkCmdBuildClusterAccelerationStructureIndirectNV;
use super::trampolines::vkCmdBuildMicromapsEXT;
use super::trampolines::vkCmdBuildPartitionedAccelerationStructuresNV;
use super::trampolines::vkCmdClearAttachments;
use super::trampolines::vkCmdClearColorImage;
use super::trampolines::vkCmdClearDepthStencilImage;
use super::trampolines::vkCmdControlVideoCodingKHR;
use super::trampolines::vkCmdConvertCooperativeVectorMatrixNV;
use super::trampolines::vkCmdCopyAccelerationStructureKHR;
use super::trampolines::vkCmdCopyAccelerationStructureNV;
use super::trampolines::vkCmdCopyAccelerationStructureToMemoryKHR;
use super::trampolines::vkCmdCopyBuffer;
use super::trampolines::vkCmdCopyBuffer2;
use super::trampolines::vkCmdCopyBuffer2KHR;
use super::trampolines::vkCmdCopyBufferToImage;
use super::trampolines::vkCmdCopyBufferToImage2;
use super::trampolines::vkCmdCopyBufferToImage2KHR;
use super::trampolines::vkCmdCopyGpaSessionResultsAMD;
use super::trampolines::vkCmdCopyImage;
use super::trampolines::vkCmdCopyImage2;
use super::trampolines::vkCmdCopyImage2KHR;
use super::trampolines::vkCmdCopyImageToBuffer;
use super::trampolines::vkCmdCopyImageToBuffer2;
use super::trampolines::vkCmdCopyImageToBuffer2KHR;
use super::trampolines::vkCmdCopyImageToMemoryKHR;
use super::trampolines::vkCmdCopyMemoryIndirectKHR;
use super::trampolines::vkCmdCopyMemoryIndirectNV;
use super::trampolines::vkCmdCopyMemoryKHR;
use super::trampolines::vkCmdCopyMemoryToAccelerationStructureKHR;
use super::trampolines::vkCmdCopyMemoryToImageIndirectKHR;
use super::trampolines::vkCmdCopyMemoryToImageIndirectNV;
use super::trampolines::vkCmdCopyMemoryToImageKHR;
use super::trampolines::vkCmdCopyMemoryToMicromapEXT;
use super::trampolines::vkCmdCopyMicromapEXT;
use super::trampolines::vkCmdCopyMicromapToMemoryEXT;
use super::trampolines::vkCmdCopyQueryPoolResults;
use super::trampolines::vkCmdCopyQueryPoolResultsToMemoryKHR;
use super::trampolines::vkCmdCopyTensorARM;
use super::trampolines::vkCmdCuLaunchKernelNVX;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkCmdCudaLaunchKernelNV;
use super::trampolines::vkCmdDebugMarkerBeginEXT;
use super::trampolines::vkCmdDebugMarkerEndEXT;
use super::trampolines::vkCmdDebugMarkerInsertEXT;
use super::trampolines::vkCmdDecodeVideoKHR;
use super::trampolines::vkCmdDecompressMemoryEXT;
use super::trampolines::vkCmdDecompressMemoryIndirectCountEXT;
use super::trampolines::vkCmdDecompressMemoryIndirectCountNV;
use super::trampolines::vkCmdDecompressMemoryNV;
use super::trampolines::vkCmdDispatch;
use super::trampolines::vkCmdDispatchBase;
use super::trampolines::vkCmdDispatchBaseKHR;
use super::trampolines::vkCmdDispatchDataGraphARM;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkCmdDispatchGraphAMDX;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkCmdDispatchGraphIndirectAMDX;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkCmdDispatchGraphIndirectCountAMDX;
use super::trampolines::vkCmdDispatchIndirect;
use super::trampolines::vkCmdDispatchIndirect2KHR;
use super::trampolines::vkCmdDispatchTileQCOM;
use super::trampolines::vkCmdDraw;
use super::trampolines::vkCmdDrawClusterHUAWEI;
use super::trampolines::vkCmdDrawClusterIndirectHUAWEI;
use super::trampolines::vkCmdDrawIndexed;
use super::trampolines::vkCmdDrawIndexedIndirect;
use super::trampolines::vkCmdDrawIndexedIndirect2KHR;
use super::trampolines::vkCmdDrawIndexedIndirectCount;
use super::trampolines::vkCmdDrawIndexedIndirectCount2KHR;
use super::trampolines::vkCmdDrawIndexedIndirectCountAMD;
use super::trampolines::vkCmdDrawIndexedIndirectCountKHR;
use super::trampolines::vkCmdDrawIndirect;
use super::trampolines::vkCmdDrawIndirect2KHR;
use super::trampolines::vkCmdDrawIndirectByteCount2EXT;
use super::trampolines::vkCmdDrawIndirectByteCountEXT;
use super::trampolines::vkCmdDrawIndirectCount;
use super::trampolines::vkCmdDrawIndirectCount2KHR;
use super::trampolines::vkCmdDrawIndirectCountAMD;
use super::trampolines::vkCmdDrawIndirectCountKHR;
use super::trampolines::vkCmdDrawMeshTasksEXT;
use super::trampolines::vkCmdDrawMeshTasksIndirect2EXT;
use super::trampolines::vkCmdDrawMeshTasksIndirectCount2EXT;
use super::trampolines::vkCmdDrawMeshTasksIndirectCountEXT;
use super::trampolines::vkCmdDrawMeshTasksIndirectCountNV;
use super::trampolines::vkCmdDrawMeshTasksIndirectEXT;
use super::trampolines::vkCmdDrawMeshTasksIndirectNV;
use super::trampolines::vkCmdDrawMeshTasksNV;
use super::trampolines::vkCmdDrawMultiEXT;
use super::trampolines::vkCmdDrawMultiIndexedEXT;
use super::trampolines::vkCmdEncodeVideoKHR;
use super::trampolines::vkCmdEndConditionalRenderingEXT;
use super::trampolines::vkCmdEndDebugUtilsLabelEXT;
use super::trampolines::vkCmdEndGpaSampleAMD;
use super::trampolines::vkCmdEndGpaSessionAMD;
use super::trampolines::vkCmdEndPerTileExecutionQCOM;
use super::trampolines::vkCmdEndQuery;
use super::trampolines::vkCmdEndQueryIndexedEXT;
use super::trampolines::vkCmdEndRenderPass;
use super::trampolines::vkCmdEndRenderPass2;
use super::trampolines::vkCmdEndRenderPass2KHR;
use super::trampolines::vkCmdEndRendering;
use super::trampolines::vkCmdEndRendering2EXT;
use super::trampolines::vkCmdEndRendering2KHR;
use super::trampolines::vkCmdEndRenderingKHR;
use super::trampolines::vkCmdEndShaderInstrumentationARM;
use super::trampolines::vkCmdEndTransformFeedback2EXT;
use super::trampolines::vkCmdEndTransformFeedbackEXT;
use super::trampolines::vkCmdEndVideoCodingKHR;
use super::trampolines::vkCmdExecuteCommands;
use super::trampolines::vkCmdExecuteGeneratedCommandsEXT;
use super::trampolines::vkCmdExecuteGeneratedCommandsNV;
use super::trampolines::vkCmdFillBuffer;
use super::trampolines::vkCmdFillMemoryKHR;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkCmdInitializeGraphScratchMemoryAMDX;
use super::trampolines::vkCmdInsertDebugUtilsLabelEXT;
use super::trampolines::vkCmdNextSubpass;
use super::trampolines::vkCmdNextSubpass2;
use super::trampolines::vkCmdNextSubpass2KHR;
use super::trampolines::vkCmdOpticalFlowExecuteNV;
use super::trampolines::vkCmdPipelineBarrier;
use super::trampolines::vkCmdPipelineBarrier2;
use super::trampolines::vkCmdPipelineBarrier2KHR;
use super::trampolines::vkCmdPreprocessGeneratedCommandsEXT;
use super::trampolines::vkCmdPreprocessGeneratedCommandsNV;
use super::trampolines::vkCmdPushConstants;
use super::trampolines::vkCmdPushConstants2;
use super::trampolines::vkCmdPushConstants2KHR;
use super::trampolines::vkCmdPushDataEXT;
use super::trampolines::vkCmdPushDescriptorSet;
use super::trampolines::vkCmdPushDescriptorSet2;
use super::trampolines::vkCmdPushDescriptorSet2KHR;
use super::trampolines::vkCmdPushDescriptorSetKHR;
use super::trampolines::vkCmdPushDescriptorSetWithTemplate;
use super::trampolines::vkCmdPushDescriptorSetWithTemplate2;
use super::trampolines::vkCmdPushDescriptorSetWithTemplate2KHR;
use super::trampolines::vkCmdPushDescriptorSetWithTemplateKHR;
use super::trampolines::vkCmdResetEvent;
use super::trampolines::vkCmdResetEvent2;
use super::trampolines::vkCmdResetEvent2KHR;
use super::trampolines::vkCmdResetQueryPool;
use super::trampolines::vkCmdResolveImage;
use super::trampolines::vkCmdResolveImage2;
use super::trampolines::vkCmdResolveImage2KHR;
use super::trampolines::vkCmdSetAlphaToCoverageEnableEXT;
use super::trampolines::vkCmdSetAlphaToOneEnableEXT;
use super::trampolines::vkCmdSetAttachmentFeedbackLoopEnableEXT;
use super::trampolines::vkCmdSetBlendConstants;
use super::trampolines::vkCmdSetCheckpointNV;
use super::trampolines::vkCmdSetCoarseSampleOrderNV;
use super::trampolines::vkCmdSetColorBlendAdvancedEXT;
use super::trampolines::vkCmdSetColorBlendEnableEXT;
use super::trampolines::vkCmdSetColorBlendEquationEXT;
use super::trampolines::vkCmdSetColorWriteEnableEXT;
use super::trampolines::vkCmdSetColorWriteMaskEXT;
use super::trampolines::vkCmdSetComputeOccupancyPriorityNV;
use super::trampolines::vkCmdSetConservativeRasterizationModeEXT;
use super::trampolines::vkCmdSetCoverageModulationModeNV;
use super::trampolines::vkCmdSetCoverageModulationTableEnableNV;
use super::trampolines::vkCmdSetCoverageModulationTableNV;
use super::trampolines::vkCmdSetCoverageReductionModeNV;
use super::trampolines::vkCmdSetCoverageToColorEnableNV;
use super::trampolines::vkCmdSetCoverageToColorLocationNV;
use super::trampolines::vkCmdSetCullMode;
use super::trampolines::vkCmdSetCullModeEXT;
use super::trampolines::vkCmdSetDepthBias;
use super::trampolines::vkCmdSetDepthBias2EXT;
use super::trampolines::vkCmdSetDepthBiasEnable;
use super::trampolines::vkCmdSetDepthBiasEnableEXT;
use super::trampolines::vkCmdSetDepthBounds;
use super::trampolines::vkCmdSetDepthBoundsTestEnable;
use super::trampolines::vkCmdSetDepthBoundsTestEnableEXT;
use super::trampolines::vkCmdSetDepthClampEnableEXT;
use super::trampolines::vkCmdSetDepthClampRangeEXT;
use super::trampolines::vkCmdSetDepthClipEnableEXT;
use super::trampolines::vkCmdSetDepthClipNegativeOneToOneEXT;
use super::trampolines::vkCmdSetDepthCompareOp;
use super::trampolines::vkCmdSetDepthCompareOpEXT;
use super::trampolines::vkCmdSetDepthTestEnable;
use super::trampolines::vkCmdSetDepthTestEnableEXT;
use super::trampolines::vkCmdSetDepthWriteEnable;
use super::trampolines::vkCmdSetDepthWriteEnableEXT;
use super::trampolines::vkCmdSetDescriptorBufferOffsets2EXT;
use super::trampolines::vkCmdSetDescriptorBufferOffsetsEXT;
use super::trampolines::vkCmdSetDeviceMask;
use super::trampolines::vkCmdSetDeviceMaskKHR;
use super::trampolines::vkCmdSetDiscardRectangleEXT;
use super::trampolines::vkCmdSetDiscardRectangleEnableEXT;
use super::trampolines::vkCmdSetDiscardRectangleModeEXT;
use super::trampolines::vkCmdSetDispatchParametersARM;
use super::trampolines::vkCmdSetEvent;
use super::trampolines::vkCmdSetEvent2;
use super::trampolines::vkCmdSetEvent2KHR;
use super::trampolines::vkCmdSetExclusiveScissorEnableNV;
use super::trampolines::vkCmdSetExclusiveScissorNV;
use super::trampolines::vkCmdSetExtraPrimitiveOverestimationSizeEXT;
use super::trampolines::vkCmdSetFragmentShadingRateEnumNV;
use super::trampolines::vkCmdSetFragmentShadingRateKHR;
use super::trampolines::vkCmdSetFrontFace;
use super::trampolines::vkCmdSetFrontFaceEXT;
use super::trampolines::vkCmdSetLineRasterizationModeEXT;
use super::trampolines::vkCmdSetLineStipple;
use super::trampolines::vkCmdSetLineStippleEXT;
use super::trampolines::vkCmdSetLineStippleEnableEXT;
use super::trampolines::vkCmdSetLineStippleKHR;
use super::trampolines::vkCmdSetLineWidth;
use super::trampolines::vkCmdSetLogicOpEXT;
use super::trampolines::vkCmdSetLogicOpEnableEXT;
use super::trampolines::vkCmdSetPatchControlPointsEXT;
use super::trampolines::vkCmdSetPerformanceMarkerINTEL;
use super::trampolines::vkCmdSetPerformanceOverrideINTEL;
use super::trampolines::vkCmdSetPerformanceStreamMarkerINTEL;
use super::trampolines::vkCmdSetPolygonModeEXT;
use super::trampolines::vkCmdSetPrimitiveRestartEnable;
use super::trampolines::vkCmdSetPrimitiveRestartEnableEXT;
use super::trampolines::vkCmdSetPrimitiveRestartIndexEXT;
use super::trampolines::vkCmdSetPrimitiveTopology;
use super::trampolines::vkCmdSetPrimitiveTopologyEXT;
use super::trampolines::vkCmdSetProvokingVertexModeEXT;
use super::trampolines::vkCmdSetRasterizationSamplesEXT;
use super::trampolines::vkCmdSetRasterizationStreamEXT;
use super::trampolines::vkCmdSetRasterizerDiscardEnable;
use super::trampolines::vkCmdSetRasterizerDiscardEnableEXT;
use super::trampolines::vkCmdSetRayTracingPipelineStackSizeKHR;
use super::trampolines::vkCmdSetRenderingAttachmentLocations;
use super::trampolines::vkCmdSetRenderingAttachmentLocationsKHR;
use super::trampolines::vkCmdSetRenderingInputAttachmentIndices;
use super::trampolines::vkCmdSetRenderingInputAttachmentIndicesKHR;
use super::trampolines::vkCmdSetRepresentativeFragmentTestEnableNV;
use super::trampolines::vkCmdSetSampleLocationsEXT;
use super::trampolines::vkCmdSetSampleLocationsEnableEXT;
use super::trampolines::vkCmdSetSampleMaskEXT;
use super::trampolines::vkCmdSetScissor;
use super::trampolines::vkCmdSetScissorWithCount;
use super::trampolines::vkCmdSetScissorWithCountEXT;
use super::trampolines::vkCmdSetShadingRateImageEnableNV;
use super::trampolines::vkCmdSetStencilCompareMask;
use super::trampolines::vkCmdSetStencilOp;
use super::trampolines::vkCmdSetStencilOpEXT;
use super::trampolines::vkCmdSetStencilReference;
use super::trampolines::vkCmdSetStencilTestEnable;
use super::trampolines::vkCmdSetStencilTestEnableEXT;
use super::trampolines::vkCmdSetStencilWriteMask;
use super::trampolines::vkCmdSetTessellationDomainOriginEXT;
use super::trampolines::vkCmdSetVertexInputEXT;
use super::trampolines::vkCmdSetViewport;
use super::trampolines::vkCmdSetViewportShadingRatePaletteNV;
use super::trampolines::vkCmdSetViewportSwizzleNV;
use super::trampolines::vkCmdSetViewportWScalingEnableNV;
use super::trampolines::vkCmdSetViewportWScalingNV;
use super::trampolines::vkCmdSetViewportWithCount;
use super::trampolines::vkCmdSetViewportWithCountEXT;
use super::trampolines::vkCmdSubpassShadingHUAWEI;
use super::trampolines::vkCmdTraceRaysIndirect2KHR;
use super::trampolines::vkCmdTraceRaysIndirectKHR;
use super::trampolines::vkCmdTraceRaysKHR;
use super::trampolines::vkCmdTraceRaysNV;
use super::trampolines::vkCmdUpdateBuffer;
use super::trampolines::vkCmdUpdateMemoryKHR;
use super::trampolines::vkCmdUpdatePipelineIndirectBufferNV;
use super::trampolines::vkCmdWaitEvents;
use super::trampolines::vkCmdWaitEvents2;
use super::trampolines::vkCmdWaitEvents2KHR;
use super::trampolines::vkCmdWriteAccelerationStructuresPropertiesKHR;
use super::trampolines::vkCmdWriteAccelerationStructuresPropertiesNV;
use super::trampolines::vkCmdWriteBufferMarker2AMD;
use super::trampolines::vkCmdWriteBufferMarkerAMD;
use super::trampolines::vkCmdWriteMarkerToMemoryAMD;
use super::trampolines::vkCmdWriteMicromapsPropertiesEXT;
use super::trampolines::vkCmdWriteTimestamp;
use super::trampolines::vkCmdWriteTimestamp2;
use super::trampolines::vkCmdWriteTimestamp2KHR;
use super::trampolines::vkCompileDeferredNV;
use super::trampolines::vkConvertCooperativeVectorMatrixNV;
use super::trampolines::vkCopyAccelerationStructureKHR;
use super::trampolines::vkCopyAccelerationStructureToMemoryKHR;
use super::trampolines::vkCopyImageToImage;
use super::trampolines::vkCopyImageToImageEXT;
use super::trampolines::vkCopyImageToMemory;
use super::trampolines::vkCopyImageToMemoryEXT;
use super::trampolines::vkCopyMemoryToAccelerationStructureKHR;
use super::trampolines::vkCopyMemoryToImage;
use super::trampolines::vkCopyMemoryToImageEXT;
use super::trampolines::vkCopyMemoryToMicromapEXT;
use super::trampolines::vkCopyMicromapEXT;
use super::trampolines::vkCopyMicromapToMemoryEXT;
use super::trampolines::vkCreateAccelerationStructure2KHR;
use super::trampolines::vkCreateAccelerationStructureKHR;
use super::trampolines::vkCreateAccelerationStructureNV;
#[cfg(target_os = "android")]
use super::trampolines::vkCreateAndroidSurfaceKHR;
use super::trampolines::vkCreateBuffer;
#[cfg(target_os = "fuchsia")]
use super::trampolines::vkCreateBufferCollectionFUCHSIA;
use super::trampolines::vkCreateBufferView;
use super::trampolines::vkCreateCommandPool;
use super::trampolines::vkCreateComputePipelines;
use super::trampolines::vkCreateCuFunctionNVX;
use super::trampolines::vkCreateCuModuleNVX;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkCreateCudaFunctionNV;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkCreateCudaModuleNV;
use super::trampolines::vkCreateDataGraphPipelineSessionARM;
use super::trampolines::vkCreateDataGraphPipelinesARM;
use super::trampolines::vkCreateDeferredOperationKHR;
use super::trampolines::vkCreateDescriptorPool;
use super::trampolines::vkCreateDescriptorSetLayout;
use super::trampolines::vkCreateDescriptorUpdateTemplate;
use super::trampolines::vkCreateDescriptorUpdateTemplateKHR;
#[cfg(feature = "wsi-directfb")]
use super::trampolines::vkCreateDirectFBSurfaceEXT;
use super::trampolines::vkCreateDisplayModeKHR;
use super::trampolines::vkCreateDisplayPlaneSurfaceKHR;
use super::trampolines::vkCreateEvent;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkCreateExecutionGraphPipelinesAMDX;
use super::trampolines::vkCreateExternalComputeQueueNV;
use super::trampolines::vkCreateFence;
use super::trampolines::vkCreateFramebuffer;
use super::trampolines::vkCreateGpaSessionAMD;
use super::trampolines::vkCreateGraphicsPipelines;
use super::trampolines::vkCreateHeadlessSurfaceEXT;
#[cfg(target_os = "ios")]
use super::trampolines::vkCreateIOSSurfaceMVK;
use super::trampolines::vkCreateImage;
#[cfg(target_os = "fuchsia")]
use super::trampolines::vkCreateImagePipeSurfaceFUCHSIA;
use super::trampolines::vkCreateImageView;
use super::trampolines::vkCreateIndirectCommandsLayoutEXT;
use super::trampolines::vkCreateIndirectCommandsLayoutNV;
use super::trampolines::vkCreateIndirectExecutionSetEXT;
#[cfg(target_os = "macos")]
use super::trampolines::vkCreateMacOSSurfaceMVK;
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
use super::trampolines::vkCreateMetalSurfaceEXT;
use super::trampolines::vkCreateMicromapEXT;
use super::trampolines::vkCreateOpticalFlowSessionNV;
use super::trampolines::vkCreatePipelineBinariesKHR;
use super::trampolines::vkCreatePipelineCache;
use super::trampolines::vkCreatePipelineLayout;
use super::trampolines::vkCreatePrivateDataSlot;
use super::trampolines::vkCreatePrivateDataSlotEXT;
use super::trampolines::vkCreateQueryPool;
use super::trampolines::vkCreateRayTracingPipelinesKHR;
use super::trampolines::vkCreateRayTracingPipelinesNV;
use super::trampolines::vkCreateRenderPass;
use super::trampolines::vkCreateRenderPass2;
use super::trampolines::vkCreateRenderPass2KHR;
use super::trampolines::vkCreateSampler;
use super::trampolines::vkCreateSamplerYcbcrConversion;
use super::trampolines::vkCreateSamplerYcbcrConversionKHR;
#[cfg(any(target_os = "nto", target_os = "qnx"))]
use super::trampolines::vkCreateScreenSurfaceQNX;
use super::trampolines::vkCreateSemaphore;
use super::trampolines::vkCreateShaderInstrumentationARM;
use super::trampolines::vkCreateShaderModule;
use super::trampolines::vkCreateShadersEXT;
#[cfg(feature = "platform-ggp")]
use super::trampolines::vkCreateStreamDescriptorSurfaceGGP;
#[cfg(target_env = "ohos")]
use super::trampolines::vkCreateSurfaceOHOS;
use super::trampolines::vkCreateTensorARM;
use super::trampolines::vkCreateTensorViewARM;
#[cfg(feature = "platform-ubm")]
use super::trampolines::vkCreateUbmSurfaceSEC;
use super::trampolines::vkCreateValidationCacheEXT;
#[cfg(feature = "platform-vi")]
use super::trampolines::vkCreateViSurfaceNN;
use super::trampolines::vkCreateVideoSessionKHR;
use super::trampolines::vkCreateVideoSessionParametersKHR;
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
use super::trampolines::vkCreateWaylandSurfaceKHR;
#[cfg(target_os = "windows")]
use super::trampolines::vkCreateWin32SurfaceKHR;
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
use super::trampolines::vkCreateXcbSurfaceKHR;
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
use super::trampolines::vkCreateXlibSurfaceKHR;
use super::trampolines::vkDeferredOperationJoinKHR;
use super::trampolines::vkDestroyAccelerationStructureKHR;
use super::trampolines::vkDestroyAccelerationStructureNV;
use super::trampolines::vkDestroyBuffer;
#[cfg(target_os = "fuchsia")]
use super::trampolines::vkDestroyBufferCollectionFUCHSIA;
use super::trampolines::vkDestroyBufferView;
use super::trampolines::vkDestroyCommandPool;
use super::trampolines::vkDestroyCuFunctionNVX;
use super::trampolines::vkDestroyCuModuleNVX;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkDestroyCudaFunctionNV;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkDestroyCudaModuleNV;
use super::trampolines::vkDestroyDataGraphPipelineSessionARM;
use super::trampolines::vkDestroyDeferredOperationKHR;
use super::trampolines::vkDestroyDescriptorPool;
use super::trampolines::vkDestroyDescriptorSetLayout;
use super::trampolines::vkDestroyDescriptorUpdateTemplate;
use super::trampolines::vkDestroyDescriptorUpdateTemplateKHR;
use super::trampolines::vkDestroyEvent;
use super::trampolines::vkDestroyExternalComputeQueueNV;
use super::trampolines::vkDestroyFence;
use super::trampolines::vkDestroyFramebuffer;
use super::trampolines::vkDestroyGpaSessionAMD;
use super::trampolines::vkDestroyImage;
use super::trampolines::vkDestroyImageView;
use super::trampolines::vkDestroyIndirectCommandsLayoutEXT;
use super::trampolines::vkDestroyIndirectCommandsLayoutNV;
use super::trampolines::vkDestroyIndirectExecutionSetEXT;
use super::trampolines::vkDestroyMicromapEXT;
use super::trampolines::vkDestroyOpticalFlowSessionNV;
use super::trampolines::vkDestroyPipeline;
use super::trampolines::vkDestroyPipelineBinaryKHR;
use super::trampolines::vkDestroyPipelineCache;
use super::trampolines::vkDestroyPipelineLayout;
use super::trampolines::vkDestroyPrivateDataSlot;
use super::trampolines::vkDestroyPrivateDataSlotEXT;
use super::trampolines::vkDestroyQueryPool;
use super::trampolines::vkDestroyRenderPass;
use super::trampolines::vkDestroySampler;
use super::trampolines::vkDestroySamplerYcbcrConversion;
use super::trampolines::vkDestroySamplerYcbcrConversionKHR;
use super::trampolines::vkDestroySemaphore;
use super::trampolines::vkDestroyShaderEXT;
use super::trampolines::vkDestroyShaderInstrumentationARM;
use super::trampolines::vkDestroyShaderModule;
use super::trampolines::vkDestroySwapchainKHR;
use super::trampolines::vkDestroyTensorARM;
use super::trampolines::vkDestroyTensorViewARM;
use super::trampolines::vkDestroyValidationCacheEXT;
use super::trampolines::vkDestroyVideoSessionKHR;
use super::trampolines::vkDestroyVideoSessionParametersKHR;
use super::trampolines::vkDeviceWaitIdle;
use super::trampolines::vkDisplayPowerControlEXT;
use super::trampolines::vkEndCommandBuffer;
use super::trampolines::vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM;
use super::trampolines::vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR;
use super::trampolines::vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM;
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
use super::trampolines::vkExportMetalObjectsEXT;
use super::trampolines::vkFlushMappedMemoryRanges;
use super::trampolines::vkFreeCommandBuffers;
use super::trampolines::vkFreeDescriptorSets;
use super::trampolines::vkFreeMemory;
use super::trampolines::vkGetAccelerationStructureBuildSizesKHR;
use super::trampolines::vkGetAccelerationStructureDeviceAddressKHR;
use super::trampolines::vkGetAccelerationStructureHandleNV;
use super::trampolines::vkGetAccelerationStructureMemoryRequirementsNV;
use super::trampolines::vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT;
#[cfg(target_os = "android")]
use super::trampolines::vkGetAndroidHardwareBufferPropertiesANDROID;
#[cfg(target_os = "fuchsia")]
use super::trampolines::vkGetBufferCollectionPropertiesFUCHSIA;
use super::trampolines::vkGetBufferDeviceAddress;
use super::trampolines::vkGetBufferDeviceAddressEXT;
use super::trampolines::vkGetBufferDeviceAddressKHR;
use super::trampolines::vkGetBufferMemoryRequirements;
use super::trampolines::vkGetBufferMemoryRequirements2;
use super::trampolines::vkGetBufferMemoryRequirements2KHR;
use super::trampolines::vkGetBufferOpaqueCaptureAddress;
use super::trampolines::vkGetBufferOpaqueCaptureAddressKHR;
use super::trampolines::vkGetBufferOpaqueCaptureDescriptorDataEXT;
use super::trampolines::vkGetCalibratedTimestampsEXT;
use super::trampolines::vkGetCalibratedTimestampsKHR;
use super::trampolines::vkGetClusterAccelerationStructureBuildSizesNV;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkGetCudaModuleCacheNV;
use super::trampolines::vkGetDataGraphPipelineAvailablePropertiesARM;
use super::trampolines::vkGetDataGraphPipelinePropertiesARM;
use super::trampolines::vkGetDataGraphPipelineSessionBindPointRequirementsARM;
use super::trampolines::vkGetDataGraphPipelineSessionMemoryRequirementsARM;
use super::trampolines::vkGetDeferredOperationMaxConcurrencyKHR;
use super::trampolines::vkGetDeferredOperationResultKHR;
use super::trampolines::vkGetDescriptorEXT;
use super::trampolines::vkGetDescriptorSetHostMappingVALVE;
use super::trampolines::vkGetDescriptorSetLayoutBindingOffsetEXT;
use super::trampolines::vkGetDescriptorSetLayoutHostMappingInfoVALVE;
use super::trampolines::vkGetDescriptorSetLayoutSizeEXT;
use super::trampolines::vkGetDescriptorSetLayoutSupport;
use super::trampolines::vkGetDescriptorSetLayoutSupportKHR;
use super::trampolines::vkGetDeviceAccelerationStructureCompatibilityKHR;
use super::trampolines::vkGetDeviceBufferMemoryRequirements;
use super::trampolines::vkGetDeviceBufferMemoryRequirementsKHR;
use super::trampolines::vkGetDeviceCombinedImageSamplerIndexNVX;
use super::trampolines::vkGetDeviceFaultDebugInfoKHR;
use super::trampolines::vkGetDeviceFaultInfoEXT;
use super::trampolines::vkGetDeviceFaultReportsKHR;
use super::trampolines::vkGetDeviceGroupPeerMemoryFeatures;
use super::trampolines::vkGetDeviceGroupPeerMemoryFeaturesKHR;
use super::trampolines::vkGetDeviceGroupPresentCapabilitiesKHR;
#[cfg(target_os = "windows")]
use super::trampolines::vkGetDeviceGroupSurfacePresentModes2EXT;
use super::trampolines::vkGetDeviceImageMemoryRequirements;
use super::trampolines::vkGetDeviceImageMemoryRequirementsKHR;
use super::trampolines::vkGetDeviceImageSparseMemoryRequirements;
use super::trampolines::vkGetDeviceImageSparseMemoryRequirementsKHR;
use super::trampolines::vkGetDeviceImageSubresourceLayout;
use super::trampolines::vkGetDeviceImageSubresourceLayoutKHR;
use super::trampolines::vkGetDeviceMemoryCommitment;
use super::trampolines::vkGetDeviceMemoryOpaqueCaptureAddress;
use super::trampolines::vkGetDeviceMemoryOpaqueCaptureAddressKHR;
use super::trampolines::vkGetDeviceMicromapCompatibilityEXT;
use super::trampolines::vkGetDeviceQueue;
use super::trampolines::vkGetDeviceQueue2;
use super::trampolines::vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI;
use super::trampolines::vkGetDeviceTensorMemoryRequirementsARM;
use super::trampolines::vkGetDisplayModeProperties2KHR;
use super::trampolines::vkGetDisplayModePropertiesKHR;
use super::trampolines::vkGetDisplayPlaneCapabilities2KHR;
use super::trampolines::vkGetDisplayPlaneCapabilitiesKHR;
use super::trampolines::vkGetDisplayPlaneSupportedDisplaysKHR;
use super::trampolines::vkGetDrmDisplayEXT;
use super::trampolines::vkGetDynamicRenderingTilePropertiesQCOM;
use super::trampolines::vkGetEncodedVideoSessionParametersKHR;
use super::trampolines::vkGetEventStatus;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkGetExecutionGraphPipelineNodeIndexAMDX;
#[cfg(feature = "beta-extensions")]
use super::trampolines::vkGetExecutionGraphPipelineScratchSizeAMDX;
use super::trampolines::vkGetExternalComputeQueueDataNV;
use super::trampolines::vkGetFenceFdKHR;
use super::trampolines::vkGetFenceStatus;
#[cfg(target_os = "windows")]
use super::trampolines::vkGetFenceWin32HandleKHR;
use super::trampolines::vkGetFramebufferTilePropertiesQCOM;
use super::trampolines::vkGetGeneratedCommandsMemoryRequirementsEXT;
use super::trampolines::vkGetGeneratedCommandsMemoryRequirementsNV;
use super::trampolines::vkGetGpaDeviceClockInfoAMD;
use super::trampolines::vkGetGpaSessionResultsAMD;
use super::trampolines::vkGetGpaSessionStatusAMD;
use super::trampolines::vkGetImageDrmFormatModifierPropertiesEXT;
use super::trampolines::vkGetImageMemoryRequirements;
use super::trampolines::vkGetImageMemoryRequirements2;
use super::trampolines::vkGetImageMemoryRequirements2KHR;
use super::trampolines::vkGetImageOpaqueCaptureDataEXT;
use super::trampolines::vkGetImageOpaqueCaptureDescriptorDataEXT;
use super::trampolines::vkGetImageSparseMemoryRequirements;
use super::trampolines::vkGetImageSparseMemoryRequirements2;
use super::trampolines::vkGetImageSparseMemoryRequirements2KHR;
use super::trampolines::vkGetImageSubresourceLayout;
use super::trampolines::vkGetImageSubresourceLayout2;
use super::trampolines::vkGetImageSubresourceLayout2EXT;
use super::trampolines::vkGetImageSubresourceLayout2KHR;
use super::trampolines::vkGetImageViewAddressNVX;
use super::trampolines::vkGetImageViewHandle64NVX;
use super::trampolines::vkGetImageViewHandleNVX;
use super::trampolines::vkGetImageViewOpaqueCaptureDescriptorDataEXT;
use super::trampolines::vkGetLatencyTimingsLegacyNV;
use super::trampolines::vkGetLatencyTimingsNV;
#[cfg(target_os = "android")]
use super::trampolines::vkGetMemoryAndroidHardwareBufferANDROID;
use super::trampolines::vkGetMemoryFdKHR;
use super::trampolines::vkGetMemoryFdPropertiesKHR;
use super::trampolines::vkGetMemoryHostPointerPropertiesEXT;
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
use super::trampolines::vkGetMemoryMetalHandleEXT;
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
use super::trampolines::vkGetMemoryMetalHandlePropertiesEXT;
#[cfg(target_env = "ohos")]
use super::trampolines::vkGetMemoryNativeBufferOHOS;
use super::trampolines::vkGetMemoryRemoteAddressNV;
#[cfg(target_os = "windows")]
use super::trampolines::vkGetMemoryWin32HandleKHR;
#[cfg(target_os = "windows")]
use super::trampolines::vkGetMemoryWin32HandleNV;
#[cfg(target_os = "windows")]
use super::trampolines::vkGetMemoryWin32HandlePropertiesKHR;
#[cfg(target_os = "fuchsia")]
use super::trampolines::vkGetMemoryZirconHandleFUCHSIA;
#[cfg(target_os = "fuchsia")]
use super::trampolines::vkGetMemoryZirconHandlePropertiesFUCHSIA;
use super::trampolines::vkGetMicromapBuildSizesEXT;
#[cfg(target_env = "ohos")]
use super::trampolines::vkGetNativeBufferPropertiesOHOS;
use super::trampolines::vkGetPartitionedAccelerationStructuresBuildSizesNV;
use super::trampolines::vkGetPastPresentationTimingEXT;
use super::trampolines::vkGetPastPresentationTimingGOOGLE;
use super::trampolines::vkGetPerformanceParameterINTEL;
use super::trampolines::vkGetPhysicalDeviceCalibrateableTimeDomainsEXT;
use super::trampolines::vkGetPhysicalDeviceCalibrateableTimeDomainsKHR;
use super::trampolines::vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV;
use super::trampolines::vkGetPhysicalDeviceCooperativeMatrixProperties2EXT;
use super::trampolines::vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR;
use super::trampolines::vkGetPhysicalDeviceCooperativeMatrixPropertiesNV;
use super::trampolines::vkGetPhysicalDeviceCooperativeVectorPropertiesNV;
use super::trampolines::vkGetPhysicalDeviceDescriptorSizeEXT;
#[cfg(feature = "wsi-directfb")]
use super::trampolines::vkGetPhysicalDeviceDirectFBPresentationSupportEXT;
use super::trampolines::vkGetPhysicalDeviceDisplayPlaneProperties2KHR;
use super::trampolines::vkGetPhysicalDeviceDisplayPlanePropertiesKHR;
use super::trampolines::vkGetPhysicalDeviceDisplayProperties2KHR;
use super::trampolines::vkGetPhysicalDeviceDisplayPropertiesKHR;
use super::trampolines::vkGetPhysicalDeviceExternalBufferProperties;
use super::trampolines::vkGetPhysicalDeviceExternalBufferPropertiesKHR;
use super::trampolines::vkGetPhysicalDeviceExternalFenceProperties;
use super::trampolines::vkGetPhysicalDeviceExternalFencePropertiesKHR;
use super::trampolines::vkGetPhysicalDeviceExternalImageFormatPropertiesNV;
use super::trampolines::vkGetPhysicalDeviceExternalSemaphoreProperties;
use super::trampolines::vkGetPhysicalDeviceExternalSemaphorePropertiesKHR;
use super::trampolines::vkGetPhysicalDeviceExternalTensorPropertiesARM;
use super::trampolines::vkGetPhysicalDeviceFeatures;
use super::trampolines::vkGetPhysicalDeviceFeatures2;
use super::trampolines::vkGetPhysicalDeviceFeatures2KHR;
use super::trampolines::vkGetPhysicalDeviceFormatProperties;
use super::trampolines::vkGetPhysicalDeviceFormatProperties2;
use super::trampolines::vkGetPhysicalDeviceFormatProperties2KHR;
use super::trampolines::vkGetPhysicalDeviceFragmentShadingRatesKHR;
use super::trampolines::vkGetPhysicalDeviceImageFormatProperties;
use super::trampolines::vkGetPhysicalDeviceImageFormatProperties2;
use super::trampolines::vkGetPhysicalDeviceImageFormatProperties2KHR;
use super::trampolines::vkGetPhysicalDeviceMemoryProperties;
use super::trampolines::vkGetPhysicalDeviceMemoryProperties2;
use super::trampolines::vkGetPhysicalDeviceMemoryProperties2KHR;
use super::trampolines::vkGetPhysicalDeviceMultisamplePropertiesEXT;
use super::trampolines::vkGetPhysicalDeviceOpticalFlowImageFormatsNV;
use super::trampolines::vkGetPhysicalDevicePresentRectanglesKHR;
use super::trampolines::vkGetPhysicalDeviceProperties;
use super::trampolines::vkGetPhysicalDeviceProperties2;
use super::trampolines::vkGetPhysicalDeviceProperties2KHR;
use super::trampolines::vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM;
use super::trampolines::vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM;
use super::trampolines::vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM;
use super::trampolines::vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM;
use super::trampolines::vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR;
use super::trampolines::vkGetPhysicalDeviceQueueFamilyProperties;
use super::trampolines::vkGetPhysicalDeviceQueueFamilyProperties2;
use super::trampolines::vkGetPhysicalDeviceQueueFamilyProperties2KHR;
#[cfg(any(target_os = "nto", target_os = "qnx"))]
use super::trampolines::vkGetPhysicalDeviceScreenPresentationSupportQNX;
use super::trampolines::vkGetPhysicalDeviceSparseImageFormatProperties;
use super::trampolines::vkGetPhysicalDeviceSparseImageFormatProperties2;
use super::trampolines::vkGetPhysicalDeviceSparseImageFormatProperties2KHR;
use super::trampolines::vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV;
use super::trampolines::vkGetPhysicalDeviceSurfaceCapabilities2EXT;
use super::trampolines::vkGetPhysicalDeviceSurfaceCapabilities2KHR;
use super::trampolines::vkGetPhysicalDeviceSurfaceCapabilitiesKHR;
use super::trampolines::vkGetPhysicalDeviceSurfaceFormats2KHR;
use super::trampolines::vkGetPhysicalDeviceSurfaceFormatsKHR;
#[cfg(target_os = "windows")]
use super::trampolines::vkGetPhysicalDeviceSurfacePresentModes2EXT;
use super::trampolines::vkGetPhysicalDeviceSurfacePresentModesKHR;
use super::trampolines::vkGetPhysicalDeviceSurfaceSupportKHR;
use super::trampolines::vkGetPhysicalDeviceToolProperties;
use super::trampolines::vkGetPhysicalDeviceToolPropertiesEXT;
#[cfg(feature = "platform-ubm")]
use super::trampolines::vkGetPhysicalDeviceUbmPresentationSupportSEC;
use super::trampolines::vkGetPhysicalDeviceVideoCapabilitiesKHR;
use super::trampolines::vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR;
use super::trampolines::vkGetPhysicalDeviceVideoFormatPropertiesKHR;
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
use super::trampolines::vkGetPhysicalDeviceWaylandPresentationSupportKHR;
#[cfg(target_os = "windows")]
use super::trampolines::vkGetPhysicalDeviceWin32PresentationSupportKHR;
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
use super::trampolines::vkGetPhysicalDeviceXcbPresentationSupportKHR;
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
use super::trampolines::vkGetPhysicalDeviceXlibPresentationSupportKHR;
use super::trampolines::vkGetPipelineBinaryDataKHR;
use super::trampolines::vkGetPipelineCacheData;
use super::trampolines::vkGetPipelineExecutableInternalRepresentationsKHR;
use super::trampolines::vkGetPipelineExecutablePropertiesKHR;
use super::trampolines::vkGetPipelineExecutableStatisticsKHR;
use super::trampolines::vkGetPipelineIndirectDeviceAddressNV;
use super::trampolines::vkGetPipelineIndirectMemoryRequirementsNV;
use super::trampolines::vkGetPipelineKeyKHR;
use super::trampolines::vkGetPipelinePropertiesEXT;
use super::trampolines::vkGetPrivateData;
use super::trampolines::vkGetPrivateDataEXT;
use super::trampolines::vkGetQueryPoolResults;
use super::trampolines::vkGetQueueCheckpointData2NV;
use super::trampolines::vkGetQueueCheckpointDataNV;
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
use super::trampolines::vkGetRandROutputDisplayEXT;
use super::trampolines::vkGetRayTracingCaptureReplayShaderGroupHandlesKHR;
use super::trampolines::vkGetRayTracingShaderGroupHandlesKHR;
use super::trampolines::vkGetRayTracingShaderGroupHandlesNV;
use super::trampolines::vkGetRayTracingShaderGroupStackSizeKHR;
use super::trampolines::vkGetRefreshCycleDurationGOOGLE;
use super::trampolines::vkGetRenderAreaGranularity;
use super::trampolines::vkGetRenderingAreaGranularity;
use super::trampolines::vkGetRenderingAreaGranularityKHR;
use super::trampolines::vkGetSamplerOpaqueCaptureDescriptorDataEXT;
#[cfg(any(target_os = "nto", target_os = "qnx"))]
use super::trampolines::vkGetScreenBufferPropertiesQNX;
use super::trampolines::vkGetSemaphoreCounterValue;
use super::trampolines::vkGetSemaphoreCounterValueKHR;
use super::trampolines::vkGetSemaphoreFdKHR;
#[cfg(target_os = "windows")]
use super::trampolines::vkGetSemaphoreWin32HandleKHR;
#[cfg(target_os = "fuchsia")]
use super::trampolines::vkGetSemaphoreZirconHandleFUCHSIA;
use super::trampolines::vkGetShaderBinaryDataEXT;
use super::trampolines::vkGetShaderInfoAMD;
use super::trampolines::vkGetShaderInstrumentationValuesARM;
use super::trampolines::vkGetShaderModuleCreateInfoIdentifierEXT;
use super::trampolines::vkGetShaderModuleIdentifierEXT;
use super::trampolines::vkGetSleepStatusLegacyNV;
use super::trampolines::vkGetSwapchainCounterEXT;
use super::trampolines::vkGetSwapchainImagesKHR;
use super::trampolines::vkGetSwapchainStatusKHR;
use super::trampolines::vkGetSwapchainTimeDomainPropertiesEXT;
use super::trampolines::vkGetSwapchainTimingPropertiesEXT;
use super::trampolines::vkGetTensorMemoryRequirementsARM;
use super::trampolines::vkGetTensorOpaqueCaptureDataARM;
use super::trampolines::vkGetTensorOpaqueCaptureDescriptorDataARM;
use super::trampolines::vkGetTensorViewOpaqueCaptureDescriptorDataARM;
use super::trampolines::vkGetValidationCacheDataEXT;
use super::trampolines::vkGetVideoSessionMemoryRequirementsKHR;
#[cfg(target_os = "windows")]
use super::trampolines::vkGetWinrtDisplayNV;
use super::trampolines::vkImportFenceFdKHR;
#[cfg(target_os = "windows")]
use super::trampolines::vkImportFenceWin32HandleKHR;
use super::trampolines::vkImportSemaphoreFdKHR;
#[cfg(target_os = "windows")]
use super::trampolines::vkImportSemaphoreWin32HandleKHR;
#[cfg(target_os = "fuchsia")]
use super::trampolines::vkImportSemaphoreZirconHandleFUCHSIA;
use super::trampolines::vkInitializePerformanceApiINTEL;
use super::trampolines::vkInvalidateMappedMemoryRanges;
use super::trampolines::vkLatencySleepLegacyNV;
use super::trampolines::vkLatencySleepNV;
use super::trampolines::vkMapMemory;
use super::trampolines::vkMapMemory2;
use super::trampolines::vkMapMemory2KHR;
use super::trampolines::vkMergePipelineCaches;
use super::trampolines::vkMergeValidationCachesEXT;
use super::trampolines::vkQueueBeginDebugUtilsLabelEXT;
use super::trampolines::vkQueueBindSparse;
use super::trampolines::vkQueueEndDebugUtilsLabelEXT;
use super::trampolines::vkQueueInsertDebugUtilsLabelEXT;
use super::trampolines::vkQueueNotifyOutOfBandLegacyNV;
use super::trampolines::vkQueueNotifyOutOfBandNV;
use super::trampolines::vkQueuePresentKHR;
use super::trampolines::vkQueueSetPerfHintQCOM;
use super::trampolines::vkQueueSetPerformanceConfigurationINTEL;
use super::trampolines::vkQueueSubmit;
use super::trampolines::vkQueueSubmit2;
use super::trampolines::vkQueueSubmit2KHR;
use super::trampolines::vkQueueWaitIdle;
use super::trampolines::vkRegisterCustomBorderColorEXT;
use super::trampolines::vkRegisterDeviceEventEXT;
use super::trampolines::vkRegisterDisplayEventEXT;
use super::trampolines::vkReleaseCapturedPipelineDataKHR;
use super::trampolines::vkReleaseDisplayEXT;
#[cfg(target_os = "windows")]
use super::trampolines::vkReleaseFullScreenExclusiveModeEXT;
use super::trampolines::vkReleasePerformanceConfigurationINTEL;
use super::trampolines::vkReleaseProfilingLockKHR;
use super::trampolines::vkReleaseSwapchainImagesEXT;
use super::trampolines::vkReleaseSwapchainImagesKHR;
use super::trampolines::vkResetCommandBuffer;
use super::trampolines::vkResetCommandPool;
use super::trampolines::vkResetDescriptorPool;
use super::trampolines::vkResetEvent;
use super::trampolines::vkResetFences;
use super::trampolines::vkResetGpaSessionAMD;
use super::trampolines::vkResetQueryPool;
use super::trampolines::vkResetQueryPoolEXT;
#[cfg(target_os = "fuchsia")]
use super::trampolines::vkSetBufferCollectionBufferConstraintsFUCHSIA;
#[cfg(target_os = "fuchsia")]
use super::trampolines::vkSetBufferCollectionImageConstraintsFUCHSIA;
use super::trampolines::vkSetDeviceMemoryPriorityEXT;
use super::trampolines::vkSetEvent;
use super::trampolines::vkSetGpaDeviceClockModeAMD;
use super::trampolines::vkSetHdrMetadataEXT;
use super::trampolines::vkSetLatencyMarkerLegacyNV;
use super::trampolines::vkSetLatencyMarkerNV;
use super::trampolines::vkSetLatencySleepModeLegacyNV;
use super::trampolines::vkSetLatencySleepModeNV;
use super::trampolines::vkSetLocalDimmingAMD;
use super::trampolines::vkSetPrivateData;
use super::trampolines::vkSetPrivateDataEXT;
use super::trampolines::vkSetSwapchainPresentTimingQueueSizeEXT;
use super::trampolines::vkShutdownLatencyDeviceLegacyNV;
use super::trampolines::vkSignalSemaphore;
use super::trampolines::vkSignalSemaphoreKHR;
use super::trampolines::vkTransitionImageLayout;
use super::trampolines::vkTransitionImageLayoutEXT;
use super::trampolines::vkTrimCommandPool;
use super::trampolines::vkTrimCommandPoolKHR;
use super::trampolines::vkUninitializePerformanceApiINTEL;
use super::trampolines::vkUnmapMemory;
use super::trampolines::vkUnmapMemory2;
use super::trampolines::vkUnmapMemory2KHR;
use super::trampolines::vkUnregisterCustomBorderColorEXT;
use super::trampolines::vkUpdateDescriptorSetWithTemplate;
use super::trampolines::vkUpdateDescriptorSetWithTemplateKHR;
use super::trampolines::vkUpdateDescriptorSets;
use super::trampolines::vkUpdateIndirectExecutionSetPipelineEXT;
use super::trampolines::vkUpdateIndirectExecutionSetShaderEXT;
use super::trampolines::vkUpdateVideoSessionParametersKHR;
use super::trampolines::vkWaitForFences;
use super::trampolines::vkWaitForPresent2KHR;
use super::trampolines::vkWaitForPresentKHR;
use super::trampolines::vkWaitSemaphores;
use super::trampolines::vkWaitSemaphoresKHR;
use super::trampolines::vkWriteAccelerationStructuresPropertiesKHR;
use super::trampolines::vkWriteMicromapsPropertiesEXT;
use super::trampolines::vkWriteResourceDescriptorsEXT;
use super::trampolines::vkWriteSamplerDescriptorsEXT;
use crate::CStr;
use crate::PFN_vkVoidFunction;
use crate::erase_function;
use crate::terminator_vkDestroySurfaceKHR;
use crate::terminator_vkGetDisplayModeProperties2KHR;
use crate::terminator_vkGetDisplayPlaneCapabilities2KHR;
use crate::terminator_vkGetPhysicalDeviceDisplayPlaneProperties2KHR;
use crate::terminator_vkGetPhysicalDeviceDisplayProperties2KHR;
use crate::terminator_vkGetPhysicalDeviceSurfaceCapabilities2EXT;
use crate::terminator_vkGetPhysicalDeviceSurfaceCapabilities2KHR;
use crate::terminator_vkGetPhysicalDeviceSurfaceFormats2KHR;
use crate::terminator_vkGetPhysicalDeviceSurfaceSupportKHR;
use crate::terminator_vkGetPhysicalDeviceToolProperties;
use crate::terminator_vkGetPhysicalDeviceToolPropertiesEXT;
use crate::vkCreateDebugReportCallbackEXT;
use crate::vkCreateDebugUtilsMessengerEXT;
use crate::vkCreateDevice;
use crate::vkCreateInstance;
use crate::vkCreateSharedSwapchainsKHR;
use crate::vkCreateSwapchainKHR;
use crate::vkDebugMarkerSetObjectNameEXT;
use crate::vkDebugMarkerSetObjectTagEXT;
use crate::vkDebugReportMessageEXT;
use crate::vkDestroyDebugReportCallbackEXT;
use crate::vkDestroyDebugUtilsMessengerEXT;
use crate::vkDestroyDevice;
use crate::vkDestroyInstance;
use crate::vkDestroySurfaceKHR;
use crate::vkEnumerateDeviceExtensionProperties;
use crate::vkEnumerateDeviceLayerProperties;
use crate::vkEnumerateInstanceExtensionProperties;
use crate::vkEnumerateInstanceLayerProperties;
use crate::vkEnumerateInstanceVersion;
use crate::vkEnumeratePhysicalDeviceGroups;
use crate::vkEnumeratePhysicalDeviceGroupsKHR;
use crate::vkEnumeratePhysicalDevices;
use crate::vkGetDeviceGroupSurfacePresentModesKHR;
use crate::vkGetDeviceProcAddr;
use crate::vkGetInstanceProcAddr;
use crate::vkSetDebugUtilsObjectNameEXT;
use crate::vkSetDebugUtilsObjectTagEXT;
use crate::vkSubmitDebugUtilsMessageEXT;
pub(crate) fn global_proc_addr(name: &CStr) -> PFN_vkVoidFunction {
    match name.to_bytes() {
        b"vkCreateInstance" => Some(erase_function(vkCreateInstance as *const ())),
        b"vkEnumerateInstanceExtensionProperties" => Some(erase_function(
            vkEnumerateInstanceExtensionProperties as *const (),
        )),
        b"vkEnumerateInstanceLayerProperties" => Some(erase_function(
            vkEnumerateInstanceLayerProperties as *const (),
        )),
        b"vkEnumerateInstanceVersion" => {
            Some(erase_function(vkEnumerateInstanceVersion as *const ()))
        }
        b"vkGetInstanceProcAddr" => Some(erase_function(vkGetInstanceProcAddr as *const ())),
        _ => None,
    }
}
#[inline]
pub(crate) unsafe fn layer_device_dispatch_proc_addr(
    table: &LayerDeviceDispatchTable,
    id: u16,
) -> PFN_vkVoidFunction {
    let index = usize::from(id);
    debug_assert!(index < COMMAND_DEVICE_DISPATCH_OFFSETS.len());
    let offset = unsafe { *COMMAND_DEVICE_DISPATCH_OFFSETS.get_unchecked(index) };
    if offset == u16::MAX {
        return None;
    }
    unsafe {
        core::ptr::from_ref(table)
            .cast::<u8>()
            .add(usize::from(offset))
            .cast::<PFN_vkVoidFunction>()
            .read()
    }
}
#[inline(never)]
#[allow(clippy::too_many_lines)]
pub(crate) fn exported_proc_addr(id: u16) -> PFN_vkVoidFunction {
    match id {
        0 => Some(erase_function(vkAcquireDrmDisplayEXT as *const ())),
        #[cfg(target_os = "windows")]
        1 => Some(erase_function(
            vkAcquireFullScreenExclusiveModeEXT as *const (),
        )),
        2 => Some(erase_function(vkAcquireNextImage2KHR as *const ())),
        3 => Some(erase_function(vkAcquireNextImageKHR as *const ())),
        4 => Some(erase_function(
            vkAcquirePerformanceConfigurationINTEL as *const (),
        )),
        5 => Some(erase_function(vkAcquireProfilingLockKHR as *const ())),
        #[cfg(target_os = "windows")]
        6 => Some(erase_function(vkAcquireWinrtDisplayNV as *const ())),
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
        7 => Some(erase_function(vkAcquireXlibDisplayEXT as *const ())),
        8 => Some(erase_function(vkAllocateCommandBuffers as *const ())),
        9 => Some(erase_function(vkAllocateDescriptorSets as *const ())),
        10 => Some(erase_function(vkAllocateMemory as *const ())),
        11 => Some(erase_function(vkAntiLagUpdateAMD as *const ())),
        12 => Some(erase_function(vkBeginCommandBuffer as *const ())),
        13 => Some(erase_function(
            vkBindAccelerationStructureMemoryNV as *const (),
        )),
        14 => Some(erase_function(vkBindBufferMemory as *const ())),
        15 => Some(erase_function(vkBindBufferMemory2 as *const ())),
        16 => Some(erase_function(vkBindBufferMemory2KHR as *const ())),
        17 => Some(erase_function(
            vkBindDataGraphPipelineSessionMemoryARM as *const (),
        )),
        18 => Some(erase_function(vkBindImageMemory as *const ())),
        19 => Some(erase_function(vkBindImageMemory2 as *const ())),
        20 => Some(erase_function(vkBindImageMemory2KHR as *const ())),
        21 => Some(erase_function(vkBindOpticalFlowSessionImageNV as *const ())),
        22 => Some(erase_function(vkBindTensorMemoryARM as *const ())),
        23 => Some(erase_function(vkBindVideoSessionMemoryKHR as *const ())),
        24 => Some(erase_function(
            vkBuildAccelerationStructuresKHR as *const (),
        )),
        25 => Some(erase_function(vkBuildMicromapsEXT as *const ())),
        26 => Some(erase_function(
            vkClearShaderInstrumentationMetricsARM as *const (),
        )),
        27 => Some(erase_function(
            vkCmdBeginConditionalRendering2EXT as *const (),
        )),
        28 => Some(erase_function(
            vkCmdBeginConditionalRenderingEXT as *const (),
        )),
        29 => Some(erase_function(vkCmdBeginCustomResolveEXT as *const ())),
        30 => Some(erase_function(vkCmdBeginDebugUtilsLabelEXT as *const ())),
        31 => Some(erase_function(vkCmdBeginGpaSampleAMD as *const ())),
        32 => Some(erase_function(vkCmdBeginGpaSessionAMD as *const ())),
        33 => Some(erase_function(vkCmdBeginPerTileExecutionQCOM as *const ())),
        34 => Some(erase_function(vkCmdBeginQuery as *const ())),
        35 => Some(erase_function(vkCmdBeginQueryIndexedEXT as *const ())),
        36 => Some(erase_function(vkCmdBeginRenderPass as *const ())),
        37 => Some(erase_function(vkCmdBeginRenderPass2 as *const ())),
        38 => Some(erase_function(vkCmdBeginRenderPass2KHR as *const ())),
        39 => Some(erase_function(vkCmdBeginRendering as *const ())),
        40 => Some(erase_function(vkCmdBeginRenderingKHR as *const ())),
        41 => Some(erase_function(
            vkCmdBeginShaderInstrumentationARM as *const (),
        )),
        42 => Some(erase_function(vkCmdBeginTransformFeedback2EXT as *const ())),
        43 => Some(erase_function(vkCmdBeginTransformFeedbackEXT as *const ())),
        44 => Some(erase_function(vkCmdBeginVideoCodingKHR as *const ())),
        45 => Some(erase_function(
            vkCmdBindDescriptorBufferEmbeddedSamplers2EXT as *const (),
        )),
        46 => Some(erase_function(
            vkCmdBindDescriptorBufferEmbeddedSamplersEXT as *const (),
        )),
        47 => Some(erase_function(vkCmdBindDescriptorBuffersEXT as *const ())),
        48 => Some(erase_function(vkCmdBindDescriptorSets as *const ())),
        49 => Some(erase_function(vkCmdBindDescriptorSets2 as *const ())),
        50 => Some(erase_function(vkCmdBindDescriptorSets2KHR as *const ())),
        51 => Some(erase_function(vkCmdBindIndexBuffer as *const ())),
        52 => Some(erase_function(vkCmdBindIndexBuffer2 as *const ())),
        53 => Some(erase_function(vkCmdBindIndexBuffer2KHR as *const ())),
        54 => Some(erase_function(vkCmdBindIndexBuffer3KHR as *const ())),
        55 => Some(erase_function(vkCmdBindInvocationMaskHUAWEI as *const ())),
        56 => Some(erase_function(vkCmdBindPipeline as *const ())),
        57 => Some(erase_function(vkCmdBindPipelineShaderGroupNV as *const ())),
        58 => Some(erase_function(vkCmdBindResourceHeapEXT as *const ())),
        59 => Some(erase_function(vkCmdBindSamplerHeapEXT as *const ())),
        60 => Some(erase_function(vkCmdBindShadersEXT as *const ())),
        61 => Some(erase_function(vkCmdBindShadingRateImageNV as *const ())),
        62 => Some(erase_function(vkCmdBindTileMemoryQCOM as *const ())),
        63 => Some(erase_function(
            vkCmdBindTransformFeedbackBuffers2EXT as *const (),
        )),
        64 => Some(erase_function(
            vkCmdBindTransformFeedbackBuffersEXT as *const (),
        )),
        65 => Some(erase_function(vkCmdBindVertexBuffers as *const ())),
        66 => Some(erase_function(vkCmdBindVertexBuffers2 as *const ())),
        67 => Some(erase_function(vkCmdBindVertexBuffers2EXT as *const ())),
        68 => Some(erase_function(vkCmdBindVertexBuffers3KHR as *const ())),
        69 => Some(erase_function(vkCmdBlitImage as *const ())),
        70 => Some(erase_function(vkCmdBlitImage2 as *const ())),
        71 => Some(erase_function(vkCmdBlitImage2KHR as *const ())),
        72 => Some(erase_function(
            vkCmdBuildAccelerationStructureNV as *const (),
        )),
        73 => Some(erase_function(
            vkCmdBuildAccelerationStructuresIndirectKHR as *const (),
        )),
        74 => Some(erase_function(
            vkCmdBuildAccelerationStructuresKHR as *const (),
        )),
        75 => Some(erase_function(
            vkCmdBuildClusterAccelerationStructureIndirectNV as *const (),
        )),
        76 => Some(erase_function(vkCmdBuildMicromapsEXT as *const ())),
        77 => Some(erase_function(
            vkCmdBuildPartitionedAccelerationStructuresNV as *const (),
        )),
        78 => Some(erase_function(vkCmdClearAttachments as *const ())),
        79 => Some(erase_function(vkCmdClearColorImage as *const ())),
        80 => Some(erase_function(vkCmdClearDepthStencilImage as *const ())),
        81 => Some(erase_function(vkCmdControlVideoCodingKHR as *const ())),
        82 => Some(erase_function(
            vkCmdConvertCooperativeVectorMatrixNV as *const (),
        )),
        83 => Some(erase_function(
            vkCmdCopyAccelerationStructureKHR as *const (),
        )),
        84 => Some(erase_function(
            vkCmdCopyAccelerationStructureNV as *const (),
        )),
        85 => Some(erase_function(
            vkCmdCopyAccelerationStructureToMemoryKHR as *const (),
        )),
        86 => Some(erase_function(vkCmdCopyBuffer as *const ())),
        87 => Some(erase_function(vkCmdCopyBuffer2 as *const ())),
        88 => Some(erase_function(vkCmdCopyBuffer2KHR as *const ())),
        89 => Some(erase_function(vkCmdCopyBufferToImage as *const ())),
        90 => Some(erase_function(vkCmdCopyBufferToImage2 as *const ())),
        91 => Some(erase_function(vkCmdCopyBufferToImage2KHR as *const ())),
        92 => Some(erase_function(vkCmdCopyGpaSessionResultsAMD as *const ())),
        93 => Some(erase_function(vkCmdCopyImage as *const ())),
        94 => Some(erase_function(vkCmdCopyImage2 as *const ())),
        95 => Some(erase_function(vkCmdCopyImage2KHR as *const ())),
        96 => Some(erase_function(vkCmdCopyImageToBuffer as *const ())),
        97 => Some(erase_function(vkCmdCopyImageToBuffer2 as *const ())),
        98 => Some(erase_function(vkCmdCopyImageToBuffer2KHR as *const ())),
        99 => Some(erase_function(vkCmdCopyImageToMemoryKHR as *const ())),
        100 => Some(erase_function(vkCmdCopyMemoryIndirectKHR as *const ())),
        101 => Some(erase_function(vkCmdCopyMemoryIndirectNV as *const ())),
        102 => Some(erase_function(vkCmdCopyMemoryKHR as *const ())),
        103 => Some(erase_function(
            vkCmdCopyMemoryToAccelerationStructureKHR as *const (),
        )),
        104 => Some(erase_function(
            vkCmdCopyMemoryToImageIndirectKHR as *const (),
        )),
        105 => Some(erase_function(
            vkCmdCopyMemoryToImageIndirectNV as *const (),
        )),
        106 => Some(erase_function(vkCmdCopyMemoryToImageKHR as *const ())),
        107 => Some(erase_function(vkCmdCopyMemoryToMicromapEXT as *const ())),
        108 => Some(erase_function(vkCmdCopyMicromapEXT as *const ())),
        109 => Some(erase_function(vkCmdCopyMicromapToMemoryEXT as *const ())),
        110 => Some(erase_function(vkCmdCopyQueryPoolResults as *const ())),
        111 => Some(erase_function(
            vkCmdCopyQueryPoolResultsToMemoryKHR as *const (),
        )),
        112 => Some(erase_function(vkCmdCopyTensorARM as *const ())),
        113 => Some(erase_function(vkCmdCuLaunchKernelNVX as *const ())),
        #[cfg(feature = "beta-extensions")]
        114 => Some(erase_function(vkCmdCudaLaunchKernelNV as *const ())),
        115 => Some(erase_function(vkCmdDebugMarkerBeginEXT as *const ())),
        116 => Some(erase_function(vkCmdDebugMarkerEndEXT as *const ())),
        117 => Some(erase_function(vkCmdDebugMarkerInsertEXT as *const ())),
        118 => Some(erase_function(vkCmdDecodeVideoKHR as *const ())),
        119 => Some(erase_function(vkCmdDecompressMemoryEXT as *const ())),
        120 => Some(erase_function(
            vkCmdDecompressMemoryIndirectCountEXT as *const (),
        )),
        121 => Some(erase_function(
            vkCmdDecompressMemoryIndirectCountNV as *const (),
        )),
        122 => Some(erase_function(vkCmdDecompressMemoryNV as *const ())),
        123 => Some(erase_function(vkCmdDispatch as *const ())),
        124 => Some(erase_function(vkCmdDispatchBase as *const ())),
        125 => Some(erase_function(vkCmdDispatchBaseKHR as *const ())),
        126 => Some(erase_function(vkCmdDispatchDataGraphARM as *const ())),
        #[cfg(feature = "beta-extensions")]
        127 => Some(erase_function(vkCmdDispatchGraphAMDX as *const ())),
        #[cfg(feature = "beta-extensions")]
        128 => Some(erase_function(vkCmdDispatchGraphIndirectAMDX as *const ())),
        #[cfg(feature = "beta-extensions")]
        129 => Some(erase_function(
            vkCmdDispatchGraphIndirectCountAMDX as *const (),
        )),
        130 => Some(erase_function(vkCmdDispatchIndirect as *const ())),
        131 => Some(erase_function(vkCmdDispatchIndirect2KHR as *const ())),
        132 => Some(erase_function(vkCmdDispatchTileQCOM as *const ())),
        133 => Some(erase_function(vkCmdDraw as *const ())),
        134 => Some(erase_function(vkCmdDrawClusterHUAWEI as *const ())),
        135 => Some(erase_function(vkCmdDrawClusterIndirectHUAWEI as *const ())),
        136 => Some(erase_function(vkCmdDrawIndexed as *const ())),
        137 => Some(erase_function(vkCmdDrawIndexedIndirect as *const ())),
        138 => Some(erase_function(vkCmdDrawIndexedIndirect2KHR as *const ())),
        139 => Some(erase_function(vkCmdDrawIndexedIndirectCount as *const ())),
        140 => Some(erase_function(
            vkCmdDrawIndexedIndirectCount2KHR as *const (),
        )),
        141 => Some(erase_function(
            vkCmdDrawIndexedIndirectCountAMD as *const (),
        )),
        142 => Some(erase_function(
            vkCmdDrawIndexedIndirectCountKHR as *const (),
        )),
        143 => Some(erase_function(vkCmdDrawIndirect as *const ())),
        144 => Some(erase_function(vkCmdDrawIndirect2KHR as *const ())),
        145 => Some(erase_function(vkCmdDrawIndirectByteCount2EXT as *const ())),
        146 => Some(erase_function(vkCmdDrawIndirectByteCountEXT as *const ())),
        147 => Some(erase_function(vkCmdDrawIndirectCount as *const ())),
        148 => Some(erase_function(vkCmdDrawIndirectCount2KHR as *const ())),
        149 => Some(erase_function(vkCmdDrawIndirectCountAMD as *const ())),
        150 => Some(erase_function(vkCmdDrawIndirectCountKHR as *const ())),
        151 => Some(erase_function(vkCmdDrawMeshTasksEXT as *const ())),
        152 => Some(erase_function(vkCmdDrawMeshTasksIndirect2EXT as *const ())),
        153 => Some(erase_function(
            vkCmdDrawMeshTasksIndirectCount2EXT as *const (),
        )),
        154 => Some(erase_function(
            vkCmdDrawMeshTasksIndirectCountEXT as *const (),
        )),
        155 => Some(erase_function(
            vkCmdDrawMeshTasksIndirectCountNV as *const (),
        )),
        156 => Some(erase_function(vkCmdDrawMeshTasksIndirectEXT as *const ())),
        157 => Some(erase_function(vkCmdDrawMeshTasksIndirectNV as *const ())),
        158 => Some(erase_function(vkCmdDrawMeshTasksNV as *const ())),
        159 => Some(erase_function(vkCmdDrawMultiEXT as *const ())),
        160 => Some(erase_function(vkCmdDrawMultiIndexedEXT as *const ())),
        161 => Some(erase_function(vkCmdEncodeVideoKHR as *const ())),
        162 => Some(erase_function(vkCmdEndConditionalRenderingEXT as *const ())),
        163 => Some(erase_function(vkCmdEndDebugUtilsLabelEXT as *const ())),
        164 => Some(erase_function(vkCmdEndGpaSampleAMD as *const ())),
        165 => Some(erase_function(vkCmdEndGpaSessionAMD as *const ())),
        166 => Some(erase_function(vkCmdEndPerTileExecutionQCOM as *const ())),
        167 => Some(erase_function(vkCmdEndQuery as *const ())),
        168 => Some(erase_function(vkCmdEndQueryIndexedEXT as *const ())),
        169 => Some(erase_function(vkCmdEndRenderPass as *const ())),
        170 => Some(erase_function(vkCmdEndRenderPass2 as *const ())),
        171 => Some(erase_function(vkCmdEndRenderPass2KHR as *const ())),
        172 => Some(erase_function(vkCmdEndRendering as *const ())),
        173 => Some(erase_function(vkCmdEndRendering2EXT as *const ())),
        174 => Some(erase_function(vkCmdEndRendering2KHR as *const ())),
        175 => Some(erase_function(vkCmdEndRenderingKHR as *const ())),
        176 => Some(erase_function(
            vkCmdEndShaderInstrumentationARM as *const (),
        )),
        177 => Some(erase_function(vkCmdEndTransformFeedback2EXT as *const ())),
        178 => Some(erase_function(vkCmdEndTransformFeedbackEXT as *const ())),
        179 => Some(erase_function(vkCmdEndVideoCodingKHR as *const ())),
        180 => Some(erase_function(vkCmdExecuteCommands as *const ())),
        181 => Some(erase_function(
            vkCmdExecuteGeneratedCommandsEXT as *const (),
        )),
        182 => Some(erase_function(vkCmdExecuteGeneratedCommandsNV as *const ())),
        183 => Some(erase_function(vkCmdFillBuffer as *const ())),
        184 => Some(erase_function(vkCmdFillMemoryKHR as *const ())),
        #[cfg(feature = "beta-extensions")]
        185 => Some(erase_function(
            vkCmdInitializeGraphScratchMemoryAMDX as *const (),
        )),
        186 => Some(erase_function(vkCmdInsertDebugUtilsLabelEXT as *const ())),
        187 => Some(erase_function(vkCmdNextSubpass as *const ())),
        188 => Some(erase_function(vkCmdNextSubpass2 as *const ())),
        189 => Some(erase_function(vkCmdNextSubpass2KHR as *const ())),
        190 => Some(erase_function(vkCmdOpticalFlowExecuteNV as *const ())),
        191 => Some(erase_function(vkCmdPipelineBarrier as *const ())),
        192 => Some(erase_function(vkCmdPipelineBarrier2 as *const ())),
        193 => Some(erase_function(vkCmdPipelineBarrier2KHR as *const ())),
        194 => Some(erase_function(
            vkCmdPreprocessGeneratedCommandsEXT as *const (),
        )),
        195 => Some(erase_function(
            vkCmdPreprocessGeneratedCommandsNV as *const (),
        )),
        196 => Some(erase_function(vkCmdPushConstants as *const ())),
        197 => Some(erase_function(vkCmdPushConstants2 as *const ())),
        198 => Some(erase_function(vkCmdPushConstants2KHR as *const ())),
        199 => Some(erase_function(vkCmdPushDataEXT as *const ())),
        200 => Some(erase_function(vkCmdPushDescriptorSet as *const ())),
        201 => Some(erase_function(vkCmdPushDescriptorSet2 as *const ())),
        202 => Some(erase_function(vkCmdPushDescriptorSet2KHR as *const ())),
        203 => Some(erase_function(vkCmdPushDescriptorSetKHR as *const ())),
        204 => Some(erase_function(
            vkCmdPushDescriptorSetWithTemplate as *const (),
        )),
        205 => Some(erase_function(
            vkCmdPushDescriptorSetWithTemplate2 as *const (),
        )),
        206 => Some(erase_function(
            vkCmdPushDescriptorSetWithTemplate2KHR as *const (),
        )),
        207 => Some(erase_function(
            vkCmdPushDescriptorSetWithTemplateKHR as *const (),
        )),
        208 => Some(erase_function(vkCmdResetEvent as *const ())),
        209 => Some(erase_function(vkCmdResetEvent2 as *const ())),
        210 => Some(erase_function(vkCmdResetEvent2KHR as *const ())),
        211 => Some(erase_function(vkCmdResetQueryPool as *const ())),
        212 => Some(erase_function(vkCmdResolveImage as *const ())),
        213 => Some(erase_function(vkCmdResolveImage2 as *const ())),
        214 => Some(erase_function(vkCmdResolveImage2KHR as *const ())),
        215 => Some(erase_function(
            vkCmdSetAlphaToCoverageEnableEXT as *const (),
        )),
        216 => Some(erase_function(vkCmdSetAlphaToOneEnableEXT as *const ())),
        217 => Some(erase_function(
            vkCmdSetAttachmentFeedbackLoopEnableEXT as *const (),
        )),
        218 => Some(erase_function(vkCmdSetBlendConstants as *const ())),
        219 => Some(erase_function(vkCmdSetCheckpointNV as *const ())),
        220 => Some(erase_function(vkCmdSetCoarseSampleOrderNV as *const ())),
        221 => Some(erase_function(vkCmdSetColorBlendAdvancedEXT as *const ())),
        222 => Some(erase_function(vkCmdSetColorBlendEnableEXT as *const ())),
        223 => Some(erase_function(vkCmdSetColorBlendEquationEXT as *const ())),
        224 => Some(erase_function(vkCmdSetColorWriteEnableEXT as *const ())),
        225 => Some(erase_function(vkCmdSetColorWriteMaskEXT as *const ())),
        226 => Some(erase_function(
            vkCmdSetComputeOccupancyPriorityNV as *const (),
        )),
        227 => Some(erase_function(
            vkCmdSetConservativeRasterizationModeEXT as *const (),
        )),
        228 => Some(erase_function(
            vkCmdSetCoverageModulationModeNV as *const (),
        )),
        229 => Some(erase_function(
            vkCmdSetCoverageModulationTableEnableNV as *const (),
        )),
        230 => Some(erase_function(
            vkCmdSetCoverageModulationTableNV as *const (),
        )),
        231 => Some(erase_function(vkCmdSetCoverageReductionModeNV as *const ())),
        232 => Some(erase_function(vkCmdSetCoverageToColorEnableNV as *const ())),
        233 => Some(erase_function(
            vkCmdSetCoverageToColorLocationNV as *const (),
        )),
        234 => Some(erase_function(vkCmdSetCullMode as *const ())),
        235 => Some(erase_function(vkCmdSetCullModeEXT as *const ())),
        236 => Some(erase_function(vkCmdSetDepthBias as *const ())),
        237 => Some(erase_function(vkCmdSetDepthBias2EXT as *const ())),
        238 => Some(erase_function(vkCmdSetDepthBiasEnable as *const ())),
        239 => Some(erase_function(vkCmdSetDepthBiasEnableEXT as *const ())),
        240 => Some(erase_function(vkCmdSetDepthBounds as *const ())),
        241 => Some(erase_function(vkCmdSetDepthBoundsTestEnable as *const ())),
        242 => Some(erase_function(
            vkCmdSetDepthBoundsTestEnableEXT as *const (),
        )),
        243 => Some(erase_function(vkCmdSetDepthClampEnableEXT as *const ())),
        244 => Some(erase_function(vkCmdSetDepthClampRangeEXT as *const ())),
        245 => Some(erase_function(vkCmdSetDepthClipEnableEXT as *const ())),
        246 => Some(erase_function(
            vkCmdSetDepthClipNegativeOneToOneEXT as *const (),
        )),
        247 => Some(erase_function(vkCmdSetDepthCompareOp as *const ())),
        248 => Some(erase_function(vkCmdSetDepthCompareOpEXT as *const ())),
        249 => Some(erase_function(vkCmdSetDepthTestEnable as *const ())),
        250 => Some(erase_function(vkCmdSetDepthTestEnableEXT as *const ())),
        251 => Some(erase_function(vkCmdSetDepthWriteEnable as *const ())),
        252 => Some(erase_function(vkCmdSetDepthWriteEnableEXT as *const ())),
        253 => Some(erase_function(
            vkCmdSetDescriptorBufferOffsets2EXT as *const (),
        )),
        254 => Some(erase_function(
            vkCmdSetDescriptorBufferOffsetsEXT as *const (),
        )),
        255 => Some(erase_function(vkCmdSetDeviceMask as *const ())),
        256 => Some(erase_function(vkCmdSetDeviceMaskKHR as *const ())),
        257 => Some(erase_function(vkCmdSetDiscardRectangleEXT as *const ())),
        258 => Some(erase_function(
            vkCmdSetDiscardRectangleEnableEXT as *const (),
        )),
        259 => Some(erase_function(vkCmdSetDiscardRectangleModeEXT as *const ())),
        260 => Some(erase_function(vkCmdSetDispatchParametersARM as *const ())),
        261 => Some(erase_function(vkCmdSetEvent as *const ())),
        262 => Some(erase_function(vkCmdSetEvent2 as *const ())),
        263 => Some(erase_function(vkCmdSetEvent2KHR as *const ())),
        264 => Some(erase_function(
            vkCmdSetExclusiveScissorEnableNV as *const (),
        )),
        265 => Some(erase_function(vkCmdSetExclusiveScissorNV as *const ())),
        266 => Some(erase_function(
            vkCmdSetExtraPrimitiveOverestimationSizeEXT as *const (),
        )),
        267 => Some(erase_function(
            vkCmdSetFragmentShadingRateEnumNV as *const (),
        )),
        268 => Some(erase_function(vkCmdSetFragmentShadingRateKHR as *const ())),
        269 => Some(erase_function(vkCmdSetFrontFace as *const ())),
        270 => Some(erase_function(vkCmdSetFrontFaceEXT as *const ())),
        271 => Some(erase_function(
            vkCmdSetLineRasterizationModeEXT as *const (),
        )),
        272 => Some(erase_function(vkCmdSetLineStipple as *const ())),
        273 => Some(erase_function(vkCmdSetLineStippleEXT as *const ())),
        274 => Some(erase_function(vkCmdSetLineStippleEnableEXT as *const ())),
        275 => Some(erase_function(vkCmdSetLineStippleKHR as *const ())),
        276 => Some(erase_function(vkCmdSetLineWidth as *const ())),
        277 => Some(erase_function(vkCmdSetLogicOpEXT as *const ())),
        278 => Some(erase_function(vkCmdSetLogicOpEnableEXT as *const ())),
        279 => Some(erase_function(vkCmdSetPatchControlPointsEXT as *const ())),
        280 => Some(erase_function(vkCmdSetPerformanceMarkerINTEL as *const ())),
        281 => Some(erase_function(
            vkCmdSetPerformanceOverrideINTEL as *const (),
        )),
        282 => Some(erase_function(
            vkCmdSetPerformanceStreamMarkerINTEL as *const (),
        )),
        283 => Some(erase_function(vkCmdSetPolygonModeEXT as *const ())),
        284 => Some(erase_function(vkCmdSetPrimitiveRestartEnable as *const ())),
        285 => Some(erase_function(
            vkCmdSetPrimitiveRestartEnableEXT as *const (),
        )),
        286 => Some(erase_function(
            vkCmdSetPrimitiveRestartIndexEXT as *const (),
        )),
        287 => Some(erase_function(vkCmdSetPrimitiveTopology as *const ())),
        288 => Some(erase_function(vkCmdSetPrimitiveTopologyEXT as *const ())),
        289 => Some(erase_function(vkCmdSetProvokingVertexModeEXT as *const ())),
        290 => Some(erase_function(vkCmdSetRasterizationSamplesEXT as *const ())),
        291 => Some(erase_function(vkCmdSetRasterizationStreamEXT as *const ())),
        292 => Some(erase_function(vkCmdSetRasterizerDiscardEnable as *const ())),
        293 => Some(erase_function(
            vkCmdSetRasterizerDiscardEnableEXT as *const (),
        )),
        294 => Some(erase_function(
            vkCmdSetRayTracingPipelineStackSizeKHR as *const (),
        )),
        295 => Some(erase_function(
            vkCmdSetRenderingAttachmentLocations as *const (),
        )),
        296 => Some(erase_function(
            vkCmdSetRenderingAttachmentLocationsKHR as *const (),
        )),
        297 => Some(erase_function(
            vkCmdSetRenderingInputAttachmentIndices as *const (),
        )),
        298 => Some(erase_function(
            vkCmdSetRenderingInputAttachmentIndicesKHR as *const (),
        )),
        299 => Some(erase_function(
            vkCmdSetRepresentativeFragmentTestEnableNV as *const (),
        )),
        300 => Some(erase_function(vkCmdSetSampleLocationsEXT as *const ())),
        301 => Some(erase_function(
            vkCmdSetSampleLocationsEnableEXT as *const (),
        )),
        302 => Some(erase_function(vkCmdSetSampleMaskEXT as *const ())),
        303 => Some(erase_function(vkCmdSetScissor as *const ())),
        304 => Some(erase_function(vkCmdSetScissorWithCount as *const ())),
        305 => Some(erase_function(vkCmdSetScissorWithCountEXT as *const ())),
        306 => Some(erase_function(
            vkCmdSetShadingRateImageEnableNV as *const (),
        )),
        307 => Some(erase_function(vkCmdSetStencilCompareMask as *const ())),
        308 => Some(erase_function(vkCmdSetStencilOp as *const ())),
        309 => Some(erase_function(vkCmdSetStencilOpEXT as *const ())),
        310 => Some(erase_function(vkCmdSetStencilReference as *const ())),
        311 => Some(erase_function(vkCmdSetStencilTestEnable as *const ())),
        312 => Some(erase_function(vkCmdSetStencilTestEnableEXT as *const ())),
        313 => Some(erase_function(vkCmdSetStencilWriteMask as *const ())),
        314 => Some(erase_function(
            vkCmdSetTessellationDomainOriginEXT as *const (),
        )),
        315 => Some(erase_function(vkCmdSetVertexInputEXT as *const ())),
        316 => Some(erase_function(vkCmdSetViewport as *const ())),
        317 => Some(erase_function(
            vkCmdSetViewportShadingRatePaletteNV as *const (),
        )),
        318 => Some(erase_function(vkCmdSetViewportSwizzleNV as *const ())),
        319 => Some(erase_function(
            vkCmdSetViewportWScalingEnableNV as *const (),
        )),
        320 => Some(erase_function(vkCmdSetViewportWScalingNV as *const ())),
        321 => Some(erase_function(vkCmdSetViewportWithCount as *const ())),
        322 => Some(erase_function(vkCmdSetViewportWithCountEXT as *const ())),
        323 => Some(erase_function(vkCmdSubpassShadingHUAWEI as *const ())),
        324 => Some(erase_function(vkCmdTraceRaysIndirect2KHR as *const ())),
        325 => Some(erase_function(vkCmdTraceRaysIndirectKHR as *const ())),
        326 => Some(erase_function(vkCmdTraceRaysKHR as *const ())),
        327 => Some(erase_function(vkCmdTraceRaysNV as *const ())),
        328 => Some(erase_function(vkCmdUpdateBuffer as *const ())),
        329 => Some(erase_function(vkCmdUpdateMemoryKHR as *const ())),
        330 => Some(erase_function(
            vkCmdUpdatePipelineIndirectBufferNV as *const (),
        )),
        331 => Some(erase_function(vkCmdWaitEvents as *const ())),
        332 => Some(erase_function(vkCmdWaitEvents2 as *const ())),
        333 => Some(erase_function(vkCmdWaitEvents2KHR as *const ())),
        334 => Some(erase_function(
            vkCmdWriteAccelerationStructuresPropertiesKHR as *const (),
        )),
        335 => Some(erase_function(
            vkCmdWriteAccelerationStructuresPropertiesNV as *const (),
        )),
        336 => Some(erase_function(vkCmdWriteBufferMarker2AMD as *const ())),
        337 => Some(erase_function(vkCmdWriteBufferMarkerAMD as *const ())),
        338 => Some(erase_function(vkCmdWriteMarkerToMemoryAMD as *const ())),
        339 => Some(erase_function(
            vkCmdWriteMicromapsPropertiesEXT as *const (),
        )),
        340 => Some(erase_function(vkCmdWriteTimestamp as *const ())),
        341 => Some(erase_function(vkCmdWriteTimestamp2 as *const ())),
        342 => Some(erase_function(vkCmdWriteTimestamp2KHR as *const ())),
        343 => Some(erase_function(vkCompileDeferredNV as *const ())),
        344 => Some(erase_function(
            vkConvertCooperativeVectorMatrixNV as *const (),
        )),
        345 => Some(erase_function(vkCopyAccelerationStructureKHR as *const ())),
        346 => Some(erase_function(
            vkCopyAccelerationStructureToMemoryKHR as *const (),
        )),
        347 => Some(erase_function(vkCopyImageToImage as *const ())),
        348 => Some(erase_function(vkCopyImageToImageEXT as *const ())),
        349 => Some(erase_function(vkCopyImageToMemory as *const ())),
        350 => Some(erase_function(vkCopyImageToMemoryEXT as *const ())),
        351 => Some(erase_function(
            vkCopyMemoryToAccelerationStructureKHR as *const (),
        )),
        352 => Some(erase_function(vkCopyMemoryToImage as *const ())),
        353 => Some(erase_function(vkCopyMemoryToImageEXT as *const ())),
        354 => Some(erase_function(vkCopyMemoryToMicromapEXT as *const ())),
        355 => Some(erase_function(vkCopyMicromapEXT as *const ())),
        356 => Some(erase_function(vkCopyMicromapToMemoryEXT as *const ())),
        357 => Some(erase_function(
            vkCreateAccelerationStructure2KHR as *const (),
        )),
        358 => Some(erase_function(
            vkCreateAccelerationStructureKHR as *const (),
        )),
        359 => Some(erase_function(vkCreateAccelerationStructureNV as *const ())),
        #[cfg(target_os = "android")]
        360 => Some(erase_function(vkCreateAndroidSurfaceKHR as *const ())),
        361 => Some(erase_function(vkCreateBuffer as *const ())),
        #[cfg(target_os = "fuchsia")]
        362 => Some(erase_function(vkCreateBufferCollectionFUCHSIA as *const ())),
        363 => Some(erase_function(vkCreateBufferView as *const ())),
        364 => Some(erase_function(vkCreateCommandPool as *const ())),
        365 => Some(erase_function(vkCreateComputePipelines as *const ())),
        366 => Some(erase_function(vkCreateCuFunctionNVX as *const ())),
        367 => Some(erase_function(vkCreateCuModuleNVX as *const ())),
        #[cfg(feature = "beta-extensions")]
        368 => Some(erase_function(vkCreateCudaFunctionNV as *const ())),
        #[cfg(feature = "beta-extensions")]
        369 => Some(erase_function(vkCreateCudaModuleNV as *const ())),
        370 => Some(erase_function(
            vkCreateDataGraphPipelineSessionARM as *const (),
        )),
        371 => Some(erase_function(vkCreateDataGraphPipelinesARM as *const ())),
        372 => Some(erase_function(vkCreateDebugReportCallbackEXT as *const ())),
        373 => Some(erase_function(vkCreateDebugUtilsMessengerEXT as *const ())),
        374 => Some(erase_function(vkCreateDeferredOperationKHR as *const ())),
        375 => Some(erase_function(vkCreateDescriptorPool as *const ())),
        376 => Some(erase_function(vkCreateDescriptorSetLayout as *const ())),
        377 => Some(erase_function(
            vkCreateDescriptorUpdateTemplate as *const (),
        )),
        378 => Some(erase_function(
            vkCreateDescriptorUpdateTemplateKHR as *const (),
        )),
        379 => Some(erase_function(vkCreateDevice as *const ())),
        #[cfg(feature = "wsi-directfb")]
        380 => Some(erase_function(vkCreateDirectFBSurfaceEXT as *const ())),
        381 => Some(erase_function(vkCreateDisplayModeKHR as *const ())),
        382 => Some(erase_function(vkCreateDisplayPlaneSurfaceKHR as *const ())),
        383 => Some(erase_function(vkCreateEvent as *const ())),
        #[cfg(feature = "beta-extensions")]
        384 => Some(erase_function(
            vkCreateExecutionGraphPipelinesAMDX as *const (),
        )),
        385 => Some(erase_function(vkCreateExternalComputeQueueNV as *const ())),
        386 => Some(erase_function(vkCreateFence as *const ())),
        387 => Some(erase_function(vkCreateFramebuffer as *const ())),
        388 => Some(erase_function(vkCreateGpaSessionAMD as *const ())),
        389 => Some(erase_function(vkCreateGraphicsPipelines as *const ())),
        390 => Some(erase_function(vkCreateHeadlessSurfaceEXT as *const ())),
        #[cfg(target_os = "ios")]
        391 => Some(erase_function(vkCreateIOSSurfaceMVK as *const ())),
        392 => Some(erase_function(vkCreateImage as *const ())),
        #[cfg(target_os = "fuchsia")]
        393 => Some(erase_function(vkCreateImagePipeSurfaceFUCHSIA as *const ())),
        394 => Some(erase_function(vkCreateImageView as *const ())),
        395 => Some(erase_function(
            vkCreateIndirectCommandsLayoutEXT as *const (),
        )),
        396 => Some(erase_function(
            vkCreateIndirectCommandsLayoutNV as *const (),
        )),
        397 => Some(erase_function(vkCreateIndirectExecutionSetEXT as *const ())),
        #[cfg(target_os = "macos")]
        399 => Some(erase_function(vkCreateMacOSSurfaceMVK as *const ())),
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        400 => Some(erase_function(vkCreateMetalSurfaceEXT as *const ())),
        401 => Some(erase_function(vkCreateMicromapEXT as *const ())),
        402 => Some(erase_function(vkCreateOpticalFlowSessionNV as *const ())),
        403 => Some(erase_function(vkCreatePipelineBinariesKHR as *const ())),
        404 => Some(erase_function(vkCreatePipelineCache as *const ())),
        405 => Some(erase_function(vkCreatePipelineLayout as *const ())),
        406 => Some(erase_function(vkCreatePrivateDataSlot as *const ())),
        407 => Some(erase_function(vkCreatePrivateDataSlotEXT as *const ())),
        408 => Some(erase_function(vkCreateQueryPool as *const ())),
        409 => Some(erase_function(vkCreateRayTracingPipelinesKHR as *const ())),
        410 => Some(erase_function(vkCreateRayTracingPipelinesNV as *const ())),
        411 => Some(erase_function(vkCreateRenderPass as *const ())),
        412 => Some(erase_function(vkCreateRenderPass2 as *const ())),
        413 => Some(erase_function(vkCreateRenderPass2KHR as *const ())),
        414 => Some(erase_function(vkCreateSampler as *const ())),
        415 => Some(erase_function(vkCreateSamplerYcbcrConversion as *const ())),
        416 => Some(erase_function(
            vkCreateSamplerYcbcrConversionKHR as *const (),
        )),
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        417 => Some(erase_function(vkCreateScreenSurfaceQNX as *const ())),
        418 => Some(erase_function(vkCreateSemaphore as *const ())),
        419 => Some(erase_function(
            vkCreateShaderInstrumentationARM as *const (),
        )),
        420 => Some(erase_function(vkCreateShaderModule as *const ())),
        421 => Some(erase_function(vkCreateShadersEXT as *const ())),
        422 => Some(erase_function(vkCreateSharedSwapchainsKHR as *const ())),
        #[cfg(feature = "platform-ggp")]
        423 => Some(erase_function(
            vkCreateStreamDescriptorSurfaceGGP as *const (),
        )),
        #[cfg(target_env = "ohos")]
        424 => Some(erase_function(vkCreateSurfaceOHOS as *const ())),
        425 => Some(erase_function(vkCreateSwapchainKHR as *const ())),
        426 => Some(erase_function(vkCreateTensorARM as *const ())),
        427 => Some(erase_function(vkCreateTensorViewARM as *const ())),
        #[cfg(feature = "platform-ubm")]
        428 => Some(erase_function(vkCreateUbmSurfaceSEC as *const ())),
        429 => Some(erase_function(vkCreateValidationCacheEXT as *const ())),
        #[cfg(feature = "platform-vi")]
        430 => Some(erase_function(vkCreateViSurfaceNN as *const ())),
        431 => Some(erase_function(vkCreateVideoSessionKHR as *const ())),
        432 => Some(erase_function(
            vkCreateVideoSessionParametersKHR as *const (),
        )),
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
        433 => Some(erase_function(vkCreateWaylandSurfaceKHR as *const ())),
        #[cfg(target_os = "windows")]
        434 => Some(erase_function(vkCreateWin32SurfaceKHR as *const ())),
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
        435 => Some(erase_function(vkCreateXcbSurfaceKHR as *const ())),
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
        436 => Some(erase_function(vkCreateXlibSurfaceKHR as *const ())),
        437 => Some(erase_function(vkDebugMarkerSetObjectNameEXT as *const ())),
        438 => Some(erase_function(vkDebugMarkerSetObjectTagEXT as *const ())),
        439 => Some(erase_function(vkDebugReportMessageEXT as *const ())),
        440 => Some(erase_function(vkDeferredOperationJoinKHR as *const ())),
        441 => Some(erase_function(
            vkDestroyAccelerationStructureKHR as *const (),
        )),
        442 => Some(erase_function(
            vkDestroyAccelerationStructureNV as *const (),
        )),
        443 => Some(erase_function(vkDestroyBuffer as *const ())),
        #[cfg(target_os = "fuchsia")]
        444 => Some(erase_function(
            vkDestroyBufferCollectionFUCHSIA as *const (),
        )),
        445 => Some(erase_function(vkDestroyBufferView as *const ())),
        446 => Some(erase_function(vkDestroyCommandPool as *const ())),
        447 => Some(erase_function(vkDestroyCuFunctionNVX as *const ())),
        448 => Some(erase_function(vkDestroyCuModuleNVX as *const ())),
        #[cfg(feature = "beta-extensions")]
        449 => Some(erase_function(vkDestroyCudaFunctionNV as *const ())),
        #[cfg(feature = "beta-extensions")]
        450 => Some(erase_function(vkDestroyCudaModuleNV as *const ())),
        451 => Some(erase_function(
            vkDestroyDataGraphPipelineSessionARM as *const (),
        )),
        452 => Some(erase_function(vkDestroyDebugReportCallbackEXT as *const ())),
        453 => Some(erase_function(vkDestroyDebugUtilsMessengerEXT as *const ())),
        454 => Some(erase_function(vkDestroyDeferredOperationKHR as *const ())),
        455 => Some(erase_function(vkDestroyDescriptorPool as *const ())),
        456 => Some(erase_function(vkDestroyDescriptorSetLayout as *const ())),
        457 => Some(erase_function(
            vkDestroyDescriptorUpdateTemplate as *const (),
        )),
        458 => Some(erase_function(
            vkDestroyDescriptorUpdateTemplateKHR as *const (),
        )),
        459 => Some(erase_function(vkDestroyDevice as *const ())),
        460 => Some(erase_function(vkDestroyEvent as *const ())),
        461 => Some(erase_function(vkDestroyExternalComputeQueueNV as *const ())),
        462 => Some(erase_function(vkDestroyFence as *const ())),
        463 => Some(erase_function(vkDestroyFramebuffer as *const ())),
        464 => Some(erase_function(vkDestroyGpaSessionAMD as *const ())),
        465 => Some(erase_function(vkDestroyImage as *const ())),
        466 => Some(erase_function(vkDestroyImageView as *const ())),
        467 => Some(erase_function(
            vkDestroyIndirectCommandsLayoutEXT as *const (),
        )),
        468 => Some(erase_function(
            vkDestroyIndirectCommandsLayoutNV as *const (),
        )),
        469 => Some(erase_function(
            vkDestroyIndirectExecutionSetEXT as *const (),
        )),
        470 => Some(erase_function(vkDestroyInstance as *const ())),
        471 => Some(erase_function(vkDestroyMicromapEXT as *const ())),
        472 => Some(erase_function(vkDestroyOpticalFlowSessionNV as *const ())),
        473 => Some(erase_function(vkDestroyPipeline as *const ())),
        474 => Some(erase_function(vkDestroyPipelineBinaryKHR as *const ())),
        475 => Some(erase_function(vkDestroyPipelineCache as *const ())),
        476 => Some(erase_function(vkDestroyPipelineLayout as *const ())),
        477 => Some(erase_function(vkDestroyPrivateDataSlot as *const ())),
        478 => Some(erase_function(vkDestroyPrivateDataSlotEXT as *const ())),
        479 => Some(erase_function(vkDestroyQueryPool as *const ())),
        480 => Some(erase_function(vkDestroyRenderPass as *const ())),
        481 => Some(erase_function(vkDestroySampler as *const ())),
        482 => Some(erase_function(vkDestroySamplerYcbcrConversion as *const ())),
        483 => Some(erase_function(
            vkDestroySamplerYcbcrConversionKHR as *const (),
        )),
        484 => Some(erase_function(vkDestroySemaphore as *const ())),
        485 => Some(erase_function(vkDestroyShaderEXT as *const ())),
        486 => Some(erase_function(
            vkDestroyShaderInstrumentationARM as *const (),
        )),
        487 => Some(erase_function(vkDestroyShaderModule as *const ())),
        488 => Some(erase_function(vkDestroySurfaceKHR as *const ())),
        489 => Some(erase_function(vkDestroySwapchainKHR as *const ())),
        490 => Some(erase_function(vkDestroyTensorARM as *const ())),
        491 => Some(erase_function(vkDestroyTensorViewARM as *const ())),
        492 => Some(erase_function(vkDestroyValidationCacheEXT as *const ())),
        493 => Some(erase_function(vkDestroyVideoSessionKHR as *const ())),
        494 => Some(erase_function(
            vkDestroyVideoSessionParametersKHR as *const (),
        )),
        495 => Some(erase_function(vkDeviceWaitIdle as *const ())),
        496 => Some(erase_function(vkDisplayPowerControlEXT as *const ())),
        497 => Some(erase_function(vkEndCommandBuffer as *const ())),
        498 => Some(erase_function(
            vkEnumerateDeviceExtensionProperties as *const (),
        )),
        499 => Some(erase_function(
            vkEnumerateDeviceLayerProperties as *const (),
        )),
        503 => Some(erase_function(vkEnumeratePhysicalDeviceGroups as *const ())),
        504 => Some(erase_function(
            vkEnumeratePhysicalDeviceGroupsKHR as *const (),
        )),
        505 => Some(erase_function(
            vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM as *const (),
        )),
        506 => Some(erase_function(
            vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR as *const (),
        )),
        507 => Some(erase_function(
            vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM as *const (),
        )),
        508 => Some(erase_function(vkEnumeratePhysicalDevices as *const ())),
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        509 => Some(erase_function(vkExportMetalObjectsEXT as *const ())),
        510 => Some(erase_function(vkFlushMappedMemoryRanges as *const ())),
        511 => Some(erase_function(vkFreeCommandBuffers as *const ())),
        512 => Some(erase_function(vkFreeDescriptorSets as *const ())),
        513 => Some(erase_function(vkFreeMemory as *const ())),
        514 => Some(erase_function(
            vkGetAccelerationStructureBuildSizesKHR as *const (),
        )),
        515 => Some(erase_function(
            vkGetAccelerationStructureDeviceAddressKHR as *const (),
        )),
        516 => Some(erase_function(
            vkGetAccelerationStructureHandleNV as *const (),
        )),
        517 => Some(erase_function(
            vkGetAccelerationStructureMemoryRequirementsNV as *const (),
        )),
        518 => Some(erase_function(
            vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT as *const (),
        )),
        #[cfg(target_os = "android")]
        519 => Some(erase_function(
            vkGetAndroidHardwareBufferPropertiesANDROID as *const (),
        )),
        #[cfg(target_os = "fuchsia")]
        520 => Some(erase_function(
            vkGetBufferCollectionPropertiesFUCHSIA as *const (),
        )),
        521 => Some(erase_function(vkGetBufferDeviceAddress as *const ())),
        522 => Some(erase_function(vkGetBufferDeviceAddressEXT as *const ())),
        523 => Some(erase_function(vkGetBufferDeviceAddressKHR as *const ())),
        524 => Some(erase_function(vkGetBufferMemoryRequirements as *const ())),
        525 => Some(erase_function(vkGetBufferMemoryRequirements2 as *const ())),
        526 => Some(erase_function(
            vkGetBufferMemoryRequirements2KHR as *const (),
        )),
        527 => Some(erase_function(vkGetBufferOpaqueCaptureAddress as *const ())),
        528 => Some(erase_function(
            vkGetBufferOpaqueCaptureAddressKHR as *const (),
        )),
        529 => Some(erase_function(
            vkGetBufferOpaqueCaptureDescriptorDataEXT as *const (),
        )),
        530 => Some(erase_function(vkGetCalibratedTimestampsEXT as *const ())),
        531 => Some(erase_function(vkGetCalibratedTimestampsKHR as *const ())),
        532 => Some(erase_function(
            vkGetClusterAccelerationStructureBuildSizesNV as *const (),
        )),
        #[cfg(feature = "beta-extensions")]
        533 => Some(erase_function(vkGetCudaModuleCacheNV as *const ())),
        534 => Some(erase_function(
            vkGetDataGraphPipelineAvailablePropertiesARM as *const (),
        )),
        535 => Some(erase_function(
            vkGetDataGraphPipelinePropertiesARM as *const (),
        )),
        536 => Some(erase_function(
            vkGetDataGraphPipelineSessionBindPointRequirementsARM as *const (),
        )),
        537 => Some(erase_function(
            vkGetDataGraphPipelineSessionMemoryRequirementsARM as *const (),
        )),
        538 => Some(erase_function(
            vkGetDeferredOperationMaxConcurrencyKHR as *const (),
        )),
        539 => Some(erase_function(vkGetDeferredOperationResultKHR as *const ())),
        540 => Some(erase_function(vkGetDescriptorEXT as *const ())),
        541 => Some(erase_function(
            vkGetDescriptorSetHostMappingVALVE as *const (),
        )),
        542 => Some(erase_function(
            vkGetDescriptorSetLayoutBindingOffsetEXT as *const (),
        )),
        543 => Some(erase_function(
            vkGetDescriptorSetLayoutHostMappingInfoVALVE as *const (),
        )),
        544 => Some(erase_function(vkGetDescriptorSetLayoutSizeEXT as *const ())),
        545 => Some(erase_function(vkGetDescriptorSetLayoutSupport as *const ())),
        546 => Some(erase_function(
            vkGetDescriptorSetLayoutSupportKHR as *const (),
        )),
        547 => Some(erase_function(
            vkGetDeviceAccelerationStructureCompatibilityKHR as *const (),
        )),
        548 => Some(erase_function(
            vkGetDeviceBufferMemoryRequirements as *const (),
        )),
        549 => Some(erase_function(
            vkGetDeviceBufferMemoryRequirementsKHR as *const (),
        )),
        550 => Some(erase_function(
            vkGetDeviceCombinedImageSamplerIndexNVX as *const (),
        )),
        551 => Some(erase_function(vkGetDeviceFaultDebugInfoKHR as *const ())),
        552 => Some(erase_function(vkGetDeviceFaultInfoEXT as *const ())),
        553 => Some(erase_function(vkGetDeviceFaultReportsKHR as *const ())),
        554 => Some(erase_function(
            vkGetDeviceGroupPeerMemoryFeatures as *const (),
        )),
        555 => Some(erase_function(
            vkGetDeviceGroupPeerMemoryFeaturesKHR as *const (),
        )),
        556 => Some(erase_function(
            vkGetDeviceGroupPresentCapabilitiesKHR as *const (),
        )),
        #[cfg(target_os = "windows")]
        557 => Some(erase_function(
            vkGetDeviceGroupSurfacePresentModes2EXT as *const (),
        )),
        558 => Some(erase_function(
            vkGetDeviceGroupSurfacePresentModesKHR as *const (),
        )),
        559 => Some(erase_function(
            vkGetDeviceImageMemoryRequirements as *const (),
        )),
        560 => Some(erase_function(
            vkGetDeviceImageMemoryRequirementsKHR as *const (),
        )),
        561 => Some(erase_function(
            vkGetDeviceImageSparseMemoryRequirements as *const (),
        )),
        562 => Some(erase_function(
            vkGetDeviceImageSparseMemoryRequirementsKHR as *const (),
        )),
        563 => Some(erase_function(
            vkGetDeviceImageSubresourceLayout as *const (),
        )),
        564 => Some(erase_function(
            vkGetDeviceImageSubresourceLayoutKHR as *const (),
        )),
        565 => Some(erase_function(vkGetDeviceMemoryCommitment as *const ())),
        566 => Some(erase_function(
            vkGetDeviceMemoryOpaqueCaptureAddress as *const (),
        )),
        567 => Some(erase_function(
            vkGetDeviceMemoryOpaqueCaptureAddressKHR as *const (),
        )),
        568 => Some(erase_function(
            vkGetDeviceMicromapCompatibilityEXT as *const (),
        )),
        569 => Some(erase_function(vkGetDeviceProcAddr as *const ())),
        570 => Some(erase_function(vkGetDeviceQueue as *const ())),
        571 => Some(erase_function(vkGetDeviceQueue2 as *const ())),
        572 => Some(erase_function(
            vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI as *const (),
        )),
        573 => Some(erase_function(
            vkGetDeviceTensorMemoryRequirementsARM as *const (),
        )),
        574 => Some(erase_function(vkGetDisplayModeProperties2KHR as *const ())),
        575 => Some(erase_function(vkGetDisplayModePropertiesKHR as *const ())),
        576 => Some(erase_function(
            vkGetDisplayPlaneCapabilities2KHR as *const (),
        )),
        577 => Some(erase_function(
            vkGetDisplayPlaneCapabilitiesKHR as *const (),
        )),
        578 => Some(erase_function(
            vkGetDisplayPlaneSupportedDisplaysKHR as *const (),
        )),
        579 => Some(erase_function(vkGetDrmDisplayEXT as *const ())),
        580 => Some(erase_function(
            vkGetDynamicRenderingTilePropertiesQCOM as *const (),
        )),
        581 => Some(erase_function(
            vkGetEncodedVideoSessionParametersKHR as *const (),
        )),
        582 => Some(erase_function(vkGetEventStatus as *const ())),
        #[cfg(feature = "beta-extensions")]
        583 => Some(erase_function(
            vkGetExecutionGraphPipelineNodeIndexAMDX as *const (),
        )),
        #[cfg(feature = "beta-extensions")]
        584 => Some(erase_function(
            vkGetExecutionGraphPipelineScratchSizeAMDX as *const (),
        )),
        585 => Some(erase_function(vkGetExternalComputeQueueDataNV as *const ())),
        586 => Some(erase_function(vkGetFenceFdKHR as *const ())),
        587 => Some(erase_function(vkGetFenceStatus as *const ())),
        #[cfg(target_os = "windows")]
        588 => Some(erase_function(vkGetFenceWin32HandleKHR as *const ())),
        589 => Some(erase_function(
            vkGetFramebufferTilePropertiesQCOM as *const (),
        )),
        590 => Some(erase_function(
            vkGetGeneratedCommandsMemoryRequirementsEXT as *const (),
        )),
        591 => Some(erase_function(
            vkGetGeneratedCommandsMemoryRequirementsNV as *const (),
        )),
        592 => Some(erase_function(vkGetGpaDeviceClockInfoAMD as *const ())),
        593 => Some(erase_function(vkGetGpaSessionResultsAMD as *const ())),
        594 => Some(erase_function(vkGetGpaSessionStatusAMD as *const ())),
        595 => Some(erase_function(
            vkGetImageDrmFormatModifierPropertiesEXT as *const (),
        )),
        596 => Some(erase_function(vkGetImageMemoryRequirements as *const ())),
        597 => Some(erase_function(vkGetImageMemoryRequirements2 as *const ())),
        598 => Some(erase_function(
            vkGetImageMemoryRequirements2KHR as *const (),
        )),
        599 => Some(erase_function(vkGetImageOpaqueCaptureDataEXT as *const ())),
        600 => Some(erase_function(
            vkGetImageOpaqueCaptureDescriptorDataEXT as *const (),
        )),
        601 => Some(erase_function(
            vkGetImageSparseMemoryRequirements as *const (),
        )),
        602 => Some(erase_function(
            vkGetImageSparseMemoryRequirements2 as *const (),
        )),
        603 => Some(erase_function(
            vkGetImageSparseMemoryRequirements2KHR as *const (),
        )),
        604 => Some(erase_function(vkGetImageSubresourceLayout as *const ())),
        605 => Some(erase_function(vkGetImageSubresourceLayout2 as *const ())),
        606 => Some(erase_function(vkGetImageSubresourceLayout2EXT as *const ())),
        607 => Some(erase_function(vkGetImageSubresourceLayout2KHR as *const ())),
        608 => Some(erase_function(vkGetImageViewAddressNVX as *const ())),
        609 => Some(erase_function(vkGetImageViewHandle64NVX as *const ())),
        610 => Some(erase_function(vkGetImageViewHandleNVX as *const ())),
        611 => Some(erase_function(
            vkGetImageViewOpaqueCaptureDescriptorDataEXT as *const (),
        )),
        613 => Some(erase_function(vkGetLatencyTimingsLegacyNV as *const ())),
        614 => Some(erase_function(vkGetLatencyTimingsNV as *const ())),
        #[cfg(target_os = "android")]
        615 => Some(erase_function(
            vkGetMemoryAndroidHardwareBufferANDROID as *const (),
        )),
        616 => Some(erase_function(vkGetMemoryFdKHR as *const ())),
        617 => Some(erase_function(vkGetMemoryFdPropertiesKHR as *const ())),
        618 => Some(erase_function(
            vkGetMemoryHostPointerPropertiesEXT as *const (),
        )),
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        619 => Some(erase_function(vkGetMemoryMetalHandleEXT as *const ())),
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        620 => Some(erase_function(
            vkGetMemoryMetalHandlePropertiesEXT as *const (),
        )),
        #[cfg(target_env = "ohos")]
        621 => Some(erase_function(vkGetMemoryNativeBufferOHOS as *const ())),
        622 => Some(erase_function(vkGetMemoryRemoteAddressNV as *const ())),
        #[cfg(target_os = "windows")]
        623 => Some(erase_function(vkGetMemoryWin32HandleKHR as *const ())),
        #[cfg(target_os = "windows")]
        624 => Some(erase_function(vkGetMemoryWin32HandleNV as *const ())),
        #[cfg(target_os = "windows")]
        625 => Some(erase_function(
            vkGetMemoryWin32HandlePropertiesKHR as *const (),
        )),
        #[cfg(target_os = "fuchsia")]
        626 => Some(erase_function(vkGetMemoryZirconHandleFUCHSIA as *const ())),
        #[cfg(target_os = "fuchsia")]
        627 => Some(erase_function(
            vkGetMemoryZirconHandlePropertiesFUCHSIA as *const (),
        )),
        628 => Some(erase_function(vkGetMicromapBuildSizesEXT as *const ())),
        #[cfg(target_env = "ohos")]
        629 => Some(erase_function(vkGetNativeBufferPropertiesOHOS as *const ())),
        630 => Some(erase_function(
            vkGetPartitionedAccelerationStructuresBuildSizesNV as *const (),
        )),
        631 => Some(erase_function(vkGetPastPresentationTimingEXT as *const ())),
        632 => Some(erase_function(
            vkGetPastPresentationTimingGOOGLE as *const (),
        )),
        633 => Some(erase_function(vkGetPerformanceParameterINTEL as *const ())),
        634 => Some(erase_function(
            vkGetPhysicalDeviceCalibrateableTimeDomainsEXT as *const (),
        )),
        635 => Some(erase_function(
            vkGetPhysicalDeviceCalibrateableTimeDomainsKHR as *const (),
        )),
        636 => Some(erase_function(
            vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV as *const (),
        )),
        637 => Some(erase_function(
            vkGetPhysicalDeviceCooperativeMatrixProperties2EXT as *const (),
        )),
        638 => Some(erase_function(
            vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR as *const (),
        )),
        639 => Some(erase_function(
            vkGetPhysicalDeviceCooperativeMatrixPropertiesNV as *const (),
        )),
        640 => Some(erase_function(
            vkGetPhysicalDeviceCooperativeVectorPropertiesNV as *const (),
        )),
        641 => Some(erase_function(
            vkGetPhysicalDeviceDescriptorSizeEXT as *const (),
        )),
        #[cfg(feature = "wsi-directfb")]
        642 => Some(erase_function(
            vkGetPhysicalDeviceDirectFBPresentationSupportEXT as *const (),
        )),
        643 => Some(erase_function(
            vkGetPhysicalDeviceDisplayPlaneProperties2KHR as *const (),
        )),
        644 => Some(erase_function(
            vkGetPhysicalDeviceDisplayPlanePropertiesKHR as *const (),
        )),
        645 => Some(erase_function(
            vkGetPhysicalDeviceDisplayProperties2KHR as *const (),
        )),
        646 => Some(erase_function(
            vkGetPhysicalDeviceDisplayPropertiesKHR as *const (),
        )),
        647 => Some(erase_function(
            vkGetPhysicalDeviceExternalBufferProperties as *const (),
        )),
        648 => Some(erase_function(
            vkGetPhysicalDeviceExternalBufferPropertiesKHR as *const (),
        )),
        649 => Some(erase_function(
            vkGetPhysicalDeviceExternalFenceProperties as *const (),
        )),
        650 => Some(erase_function(
            vkGetPhysicalDeviceExternalFencePropertiesKHR as *const (),
        )),
        651 => Some(erase_function(
            vkGetPhysicalDeviceExternalImageFormatPropertiesNV as *const (),
        )),
        652 => Some(erase_function(
            vkGetPhysicalDeviceExternalSemaphoreProperties as *const (),
        )),
        653 => Some(erase_function(
            vkGetPhysicalDeviceExternalSemaphorePropertiesKHR as *const (),
        )),
        654 => Some(erase_function(
            vkGetPhysicalDeviceExternalTensorPropertiesARM as *const (),
        )),
        655 => Some(erase_function(vkGetPhysicalDeviceFeatures as *const ())),
        656 => Some(erase_function(vkGetPhysicalDeviceFeatures2 as *const ())),
        657 => Some(erase_function(vkGetPhysicalDeviceFeatures2KHR as *const ())),
        658 => Some(erase_function(
            vkGetPhysicalDeviceFormatProperties as *const (),
        )),
        659 => Some(erase_function(
            vkGetPhysicalDeviceFormatProperties2 as *const (),
        )),
        660 => Some(erase_function(
            vkGetPhysicalDeviceFormatProperties2KHR as *const (),
        )),
        661 => Some(erase_function(
            vkGetPhysicalDeviceFragmentShadingRatesKHR as *const (),
        )),
        662 => Some(erase_function(
            vkGetPhysicalDeviceImageFormatProperties as *const (),
        )),
        663 => Some(erase_function(
            vkGetPhysicalDeviceImageFormatProperties2 as *const (),
        )),
        664 => Some(erase_function(
            vkGetPhysicalDeviceImageFormatProperties2KHR as *const (),
        )),
        665 => Some(erase_function(
            vkGetPhysicalDeviceMemoryProperties as *const (),
        )),
        666 => Some(erase_function(
            vkGetPhysicalDeviceMemoryProperties2 as *const (),
        )),
        667 => Some(erase_function(
            vkGetPhysicalDeviceMemoryProperties2KHR as *const (),
        )),
        668 => Some(erase_function(
            vkGetPhysicalDeviceMultisamplePropertiesEXT as *const (),
        )),
        669 => Some(erase_function(
            vkGetPhysicalDeviceOpticalFlowImageFormatsNV as *const (),
        )),
        670 => Some(erase_function(
            vkGetPhysicalDevicePresentRectanglesKHR as *const (),
        )),
        671 => Some(erase_function(vkGetPhysicalDeviceProperties as *const ())),
        672 => Some(erase_function(vkGetPhysicalDeviceProperties2 as *const ())),
        673 => Some(erase_function(
            vkGetPhysicalDeviceProperties2KHR as *const (),
        )),
        674 => Some(erase_function(
            vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM as *const (),
        )),
        675 => Some(erase_function(
            vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM as *const (),
        )),
        676 => Some(erase_function(
            vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM as *const (),
        )),
        677 => Some(erase_function(
            vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM as *const (),
        )),
        678 => Some(erase_function(
            vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR as *const (),
        )),
        679 => Some(erase_function(
            vkGetPhysicalDeviceQueueFamilyProperties as *const (),
        )),
        680 => Some(erase_function(
            vkGetPhysicalDeviceQueueFamilyProperties2 as *const (),
        )),
        681 => Some(erase_function(
            vkGetPhysicalDeviceQueueFamilyProperties2KHR as *const (),
        )),
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        682 => Some(erase_function(
            vkGetPhysicalDeviceScreenPresentationSupportQNX as *const (),
        )),
        683 => Some(erase_function(
            vkGetPhysicalDeviceSparseImageFormatProperties as *const (),
        )),
        684 => Some(erase_function(
            vkGetPhysicalDeviceSparseImageFormatProperties2 as *const (),
        )),
        685 => Some(erase_function(
            vkGetPhysicalDeviceSparseImageFormatProperties2KHR as *const (),
        )),
        686 => Some(erase_function(
            vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV as *const (),
        )),
        687 => Some(erase_function(
            vkGetPhysicalDeviceSurfaceCapabilities2EXT as *const (),
        )),
        688 => Some(erase_function(
            vkGetPhysicalDeviceSurfaceCapabilities2KHR as *const (),
        )),
        689 => Some(erase_function(
            vkGetPhysicalDeviceSurfaceCapabilitiesKHR as *const (),
        )),
        690 => Some(erase_function(
            vkGetPhysicalDeviceSurfaceFormats2KHR as *const (),
        )),
        691 => Some(erase_function(
            vkGetPhysicalDeviceSurfaceFormatsKHR as *const (),
        )),
        #[cfg(target_os = "windows")]
        692 => Some(erase_function(
            vkGetPhysicalDeviceSurfacePresentModes2EXT as *const (),
        )),
        693 => Some(erase_function(
            vkGetPhysicalDeviceSurfacePresentModesKHR as *const (),
        )),
        694 => Some(erase_function(
            vkGetPhysicalDeviceSurfaceSupportKHR as *const (),
        )),
        695 => Some(erase_function(
            vkGetPhysicalDeviceToolProperties as *const (),
        )),
        696 => Some(erase_function(
            vkGetPhysicalDeviceToolPropertiesEXT as *const (),
        )),
        #[cfg(feature = "platform-ubm")]
        697 => Some(erase_function(
            vkGetPhysicalDeviceUbmPresentationSupportSEC as *const (),
        )),
        698 => Some(erase_function(
            vkGetPhysicalDeviceVideoCapabilitiesKHR as *const (),
        )),
        699 => Some(erase_function(
            vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR as *const (),
        )),
        700 => Some(erase_function(
            vkGetPhysicalDeviceVideoFormatPropertiesKHR as *const (),
        )),
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
        701 => Some(erase_function(
            vkGetPhysicalDeviceWaylandPresentationSupportKHR as *const (),
        )),
        #[cfg(target_os = "windows")]
        702 => Some(erase_function(
            vkGetPhysicalDeviceWin32PresentationSupportKHR as *const (),
        )),
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
        703 => Some(erase_function(
            vkGetPhysicalDeviceXcbPresentationSupportKHR as *const (),
        )),
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
        704 => Some(erase_function(
            vkGetPhysicalDeviceXlibPresentationSupportKHR as *const (),
        )),
        705 => Some(erase_function(vkGetPipelineBinaryDataKHR as *const ())),
        706 => Some(erase_function(vkGetPipelineCacheData as *const ())),
        707 => Some(erase_function(
            vkGetPipelineExecutableInternalRepresentationsKHR as *const (),
        )),
        708 => Some(erase_function(
            vkGetPipelineExecutablePropertiesKHR as *const (),
        )),
        709 => Some(erase_function(
            vkGetPipelineExecutableStatisticsKHR as *const (),
        )),
        710 => Some(erase_function(
            vkGetPipelineIndirectDeviceAddressNV as *const (),
        )),
        711 => Some(erase_function(
            vkGetPipelineIndirectMemoryRequirementsNV as *const (),
        )),
        712 => Some(erase_function(vkGetPipelineKeyKHR as *const ())),
        713 => Some(erase_function(vkGetPipelinePropertiesEXT as *const ())),
        714 => Some(erase_function(vkGetPrivateData as *const ())),
        715 => Some(erase_function(vkGetPrivateDataEXT as *const ())),
        716 => Some(erase_function(vkGetQueryPoolResults as *const ())),
        717 => Some(erase_function(vkGetQueueCheckpointData2NV as *const ())),
        718 => Some(erase_function(vkGetQueueCheckpointDataNV as *const ())),
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
        719 => Some(erase_function(vkGetRandROutputDisplayEXT as *const ())),
        720 => Some(erase_function(
            vkGetRayTracingCaptureReplayShaderGroupHandlesKHR as *const (),
        )),
        721 => Some(erase_function(
            vkGetRayTracingShaderGroupHandlesKHR as *const (),
        )),
        722 => Some(erase_function(
            vkGetRayTracingShaderGroupHandlesNV as *const (),
        )),
        723 => Some(erase_function(
            vkGetRayTracingShaderGroupStackSizeKHR as *const (),
        )),
        724 => Some(erase_function(vkGetRefreshCycleDurationGOOGLE as *const ())),
        725 => Some(erase_function(vkGetRenderAreaGranularity as *const ())),
        726 => Some(erase_function(vkGetRenderingAreaGranularity as *const ())),
        727 => Some(erase_function(
            vkGetRenderingAreaGranularityKHR as *const (),
        )),
        728 => Some(erase_function(
            vkGetSamplerOpaqueCaptureDescriptorDataEXT as *const (),
        )),
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        729 => Some(erase_function(vkGetScreenBufferPropertiesQNX as *const ())),
        730 => Some(erase_function(vkGetSemaphoreCounterValue as *const ())),
        731 => Some(erase_function(vkGetSemaphoreCounterValueKHR as *const ())),
        732 => Some(erase_function(vkGetSemaphoreFdKHR as *const ())),
        #[cfg(target_os = "windows")]
        733 => Some(erase_function(vkGetSemaphoreWin32HandleKHR as *const ())),
        #[cfg(target_os = "fuchsia")]
        734 => Some(erase_function(
            vkGetSemaphoreZirconHandleFUCHSIA as *const (),
        )),
        735 => Some(erase_function(vkGetShaderBinaryDataEXT as *const ())),
        736 => Some(erase_function(vkGetShaderInfoAMD as *const ())),
        737 => Some(erase_function(
            vkGetShaderInstrumentationValuesARM as *const (),
        )),
        738 => Some(erase_function(
            vkGetShaderModuleCreateInfoIdentifierEXT as *const (),
        )),
        739 => Some(erase_function(vkGetShaderModuleIdentifierEXT as *const ())),
        740 => Some(erase_function(vkGetSleepStatusLegacyNV as *const ())),
        741 => Some(erase_function(vkGetSwapchainCounterEXT as *const ())),
        742 => Some(erase_function(vkGetSwapchainImagesKHR as *const ())),
        743 => Some(erase_function(vkGetSwapchainStatusKHR as *const ())),
        744 => Some(erase_function(
            vkGetSwapchainTimeDomainPropertiesEXT as *const (),
        )),
        745 => Some(erase_function(
            vkGetSwapchainTimingPropertiesEXT as *const (),
        )),
        746 => Some(erase_function(
            vkGetTensorMemoryRequirementsARM as *const (),
        )),
        747 => Some(erase_function(vkGetTensorOpaqueCaptureDataARM as *const ())),
        748 => Some(erase_function(
            vkGetTensorOpaqueCaptureDescriptorDataARM as *const (),
        )),
        749 => Some(erase_function(
            vkGetTensorViewOpaqueCaptureDescriptorDataARM as *const (),
        )),
        750 => Some(erase_function(vkGetValidationCacheDataEXT as *const ())),
        751 => Some(erase_function(
            vkGetVideoSessionMemoryRequirementsKHR as *const (),
        )),
        #[cfg(target_os = "windows")]
        752 => Some(erase_function(vkGetWinrtDisplayNV as *const ())),
        753 => Some(erase_function(vkImportFenceFdKHR as *const ())),
        #[cfg(target_os = "windows")]
        754 => Some(erase_function(vkImportFenceWin32HandleKHR as *const ())),
        755 => Some(erase_function(vkImportSemaphoreFdKHR as *const ())),
        #[cfg(target_os = "windows")]
        756 => Some(erase_function(vkImportSemaphoreWin32HandleKHR as *const ())),
        #[cfg(target_os = "fuchsia")]
        757 => Some(erase_function(
            vkImportSemaphoreZirconHandleFUCHSIA as *const (),
        )),
        758 => Some(erase_function(vkInitializePerformanceApiINTEL as *const ())),
        759 => Some(erase_function(vkInvalidateMappedMemoryRanges as *const ())),
        760 => Some(erase_function(vkLatencySleepLegacyNV as *const ())),
        761 => Some(erase_function(vkLatencySleepNV as *const ())),
        762 => Some(erase_function(vkMapMemory as *const ())),
        763 => Some(erase_function(vkMapMemory2 as *const ())),
        764 => Some(erase_function(vkMapMemory2KHR as *const ())),
        765 => Some(erase_function(vkMergePipelineCaches as *const ())),
        766 => Some(erase_function(vkMergeValidationCachesEXT as *const ())),
        767 => Some(erase_function(vkQueueBeginDebugUtilsLabelEXT as *const ())),
        768 => Some(erase_function(vkQueueBindSparse as *const ())),
        769 => Some(erase_function(vkQueueEndDebugUtilsLabelEXT as *const ())),
        770 => Some(erase_function(vkQueueInsertDebugUtilsLabelEXT as *const ())),
        771 => Some(erase_function(vkQueueNotifyOutOfBandLegacyNV as *const ())),
        772 => Some(erase_function(vkQueueNotifyOutOfBandNV as *const ())),
        773 => Some(erase_function(vkQueuePresentKHR as *const ())),
        774 => Some(erase_function(vkQueueSetPerfHintQCOM as *const ())),
        775 => Some(erase_function(
            vkQueueSetPerformanceConfigurationINTEL as *const (),
        )),
        776 => Some(erase_function(vkQueueSubmit as *const ())),
        777 => Some(erase_function(vkQueueSubmit2 as *const ())),
        778 => Some(erase_function(vkQueueSubmit2KHR as *const ())),
        779 => Some(erase_function(vkQueueWaitIdle as *const ())),
        780 => Some(erase_function(vkRegisterCustomBorderColorEXT as *const ())),
        781 => Some(erase_function(vkRegisterDeviceEventEXT as *const ())),
        782 => Some(erase_function(vkRegisterDisplayEventEXT as *const ())),
        783 => Some(erase_function(
            vkReleaseCapturedPipelineDataKHR as *const (),
        )),
        784 => Some(erase_function(vkReleaseDisplayEXT as *const ())),
        #[cfg(target_os = "windows")]
        785 => Some(erase_function(
            vkReleaseFullScreenExclusiveModeEXT as *const (),
        )),
        786 => Some(erase_function(
            vkReleasePerformanceConfigurationINTEL as *const (),
        )),
        787 => Some(erase_function(vkReleaseProfilingLockKHR as *const ())),
        788 => Some(erase_function(vkReleaseSwapchainImagesEXT as *const ())),
        789 => Some(erase_function(vkReleaseSwapchainImagesKHR as *const ())),
        790 => Some(erase_function(vkResetCommandBuffer as *const ())),
        791 => Some(erase_function(vkResetCommandPool as *const ())),
        792 => Some(erase_function(vkResetDescriptorPool as *const ())),
        793 => Some(erase_function(vkResetEvent as *const ())),
        794 => Some(erase_function(vkResetFences as *const ())),
        795 => Some(erase_function(vkResetGpaSessionAMD as *const ())),
        796 => Some(erase_function(vkResetQueryPool as *const ())),
        797 => Some(erase_function(vkResetQueryPoolEXT as *const ())),
        #[cfg(target_os = "fuchsia")]
        798 => Some(erase_function(
            vkSetBufferCollectionBufferConstraintsFUCHSIA as *const (),
        )),
        #[cfg(target_os = "fuchsia")]
        799 => Some(erase_function(
            vkSetBufferCollectionImageConstraintsFUCHSIA as *const (),
        )),
        800 => Some(erase_function(vkSetDebugUtilsObjectNameEXT as *const ())),
        801 => Some(erase_function(vkSetDebugUtilsObjectTagEXT as *const ())),
        802 => Some(erase_function(vkSetDeviceMemoryPriorityEXT as *const ())),
        803 => Some(erase_function(vkSetEvent as *const ())),
        804 => Some(erase_function(vkSetGpaDeviceClockModeAMD as *const ())),
        805 => Some(erase_function(vkSetHdrMetadataEXT as *const ())),
        806 => Some(erase_function(vkSetLatencyMarkerLegacyNV as *const ())),
        807 => Some(erase_function(vkSetLatencyMarkerNV as *const ())),
        808 => Some(erase_function(vkSetLatencySleepModeLegacyNV as *const ())),
        809 => Some(erase_function(vkSetLatencySleepModeNV as *const ())),
        810 => Some(erase_function(vkSetLocalDimmingAMD as *const ())),
        811 => Some(erase_function(vkSetPrivateData as *const ())),
        812 => Some(erase_function(vkSetPrivateDataEXT as *const ())),
        813 => Some(erase_function(
            vkSetSwapchainPresentTimingQueueSizeEXT as *const (),
        )),
        814 => Some(erase_function(vkShutdownLatencyDeviceLegacyNV as *const ())),
        815 => Some(erase_function(vkSignalSemaphore as *const ())),
        816 => Some(erase_function(vkSignalSemaphoreKHR as *const ())),
        817 => Some(erase_function(vkSubmitDebugUtilsMessageEXT as *const ())),
        818 => Some(erase_function(vkTransitionImageLayout as *const ())),
        819 => Some(erase_function(vkTransitionImageLayoutEXT as *const ())),
        820 => Some(erase_function(vkTrimCommandPool as *const ())),
        821 => Some(erase_function(vkTrimCommandPoolKHR as *const ())),
        822 => Some(erase_function(
            vkUninitializePerformanceApiINTEL as *const (),
        )),
        823 => Some(erase_function(vkUnmapMemory as *const ())),
        824 => Some(erase_function(vkUnmapMemory2 as *const ())),
        825 => Some(erase_function(vkUnmapMemory2KHR as *const ())),
        826 => Some(erase_function(
            vkUnregisterCustomBorderColorEXT as *const (),
        )),
        827 => Some(erase_function(
            vkUpdateDescriptorSetWithTemplate as *const (),
        )),
        828 => Some(erase_function(
            vkUpdateDescriptorSetWithTemplateKHR as *const (),
        )),
        829 => Some(erase_function(vkUpdateDescriptorSets as *const ())),
        830 => Some(erase_function(
            vkUpdateIndirectExecutionSetPipelineEXT as *const (),
        )),
        831 => Some(erase_function(
            vkUpdateIndirectExecutionSetShaderEXT as *const (),
        )),
        832 => Some(erase_function(
            vkUpdateVideoSessionParametersKHR as *const (),
        )),
        833 => Some(erase_function(vkWaitForFences as *const ())),
        834 => Some(erase_function(vkWaitForPresent2KHR as *const ())),
        835 => Some(erase_function(vkWaitForPresentKHR as *const ())),
        836 => Some(erase_function(vkWaitSemaphores as *const ())),
        837 => Some(erase_function(vkWaitSemaphoresKHR as *const ())),
        838 => Some(erase_function(
            vkWriteAccelerationStructuresPropertiesKHR as *const (),
        )),
        839 => Some(erase_function(vkWriteMicromapsPropertiesEXT as *const ())),
        840 => Some(erase_function(vkWriteResourceDescriptorsEXT as *const ())),
        841 => Some(erase_function(vkWriteSamplerDescriptorsEXT as *const ())),
        _ => None,
    }
}
#[inline(never)]
pub(crate) fn instance_terminator_proc_addr(id: u16) -> PFN_vkVoidFunction {
    match id {
        #[cfg(target_os = "android")]
        360 => Some(erase_function(
            terminator_vkCreateAndroidSurfaceKHR as *const (),
        )),
        #[cfg(feature = "wsi-directfb")]
        380 => Some(erase_function(
            terminator_vkCreateDirectFBSurfaceEXT as *const (),
        )),
        382 => Some(erase_function(
            terminator_vkCreateDisplayPlaneSurfaceKHR as *const (),
        )),
        390 => Some(erase_function(
            terminator_vkCreateHeadlessSurfaceEXT as *const (),
        )),
        #[cfg(target_os = "ios")]
        391 => Some(erase_function(
            terminator_vkCreateIOSSurfaceMVK as *const (),
        )),
        #[cfg(target_os = "fuchsia")]
        393 => Some(erase_function(
            terminator_vkCreateImagePipeSurfaceFUCHSIA as *const (),
        )),
        #[cfg(target_os = "macos")]
        399 => Some(erase_function(
            terminator_vkCreateMacOSSurfaceMVK as *const (),
        )),
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        400 => Some(erase_function(
            terminator_vkCreateMetalSurfaceEXT as *const (),
        )),
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        417 => Some(erase_function(
            terminator_vkCreateScreenSurfaceQNX as *const (),
        )),
        #[cfg(feature = "platform-ggp")]
        423 => Some(erase_function(
            terminator_vkCreateStreamDescriptorSurfaceGGP as *const (),
        )),
        #[cfg(target_env = "ohos")]
        424 => Some(erase_function(terminator_vkCreateSurfaceOHOS as *const ())),
        #[cfg(feature = "platform-ubm")]
        428 => Some(erase_function(
            terminator_vkCreateUbmSurfaceSEC as *const (),
        )),
        #[cfg(feature = "platform-vi")]
        430 => Some(erase_function(terminator_vkCreateViSurfaceNN as *const ())),
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
        433 => Some(erase_function(
            terminator_vkCreateWaylandSurfaceKHR as *const (),
        )),
        #[cfg(target_os = "windows")]
        434 => Some(erase_function(
            terminator_vkCreateWin32SurfaceKHR as *const (),
        )),
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
        435 => Some(erase_function(
            terminator_vkCreateXcbSurfaceKHR as *const (),
        )),
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
        436 => Some(erase_function(
            terminator_vkCreateXlibSurfaceKHR as *const (),
        )),
        488 => Some(erase_function(terminator_vkDestroySurfaceKHR as *const ())),
        _ => None,
    }
}
#[inline(never)]
#[allow(clippy::too_many_lines)]
pub(crate) fn physical_device_terminator_proc_addr(id: u16) -> PFN_vkVoidFunction {
    match id {
        0 => Some(erase_function(
            terminator_vkAcquireDrmDisplayEXT as *const (),
        )),
        #[cfg(target_os = "windows")]
        6 => Some(erase_function(
            terminator_vkAcquireWinrtDisplayNV as *const (),
        )),
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
        7 => Some(erase_function(
            terminator_vkAcquireXlibDisplayEXT as *const (),
        )),
        381 => Some(erase_function(
            terminator_vkCreateDisplayModeKHR as *const (),
        )),
        505 => Some(erase_function(
            terminator_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM
                as *const (),
        )),
        506 => Some(erase_function(
            terminator_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR as *const (),
        )),
        507 => Some(erase_function(
            terminator_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM as *const (),
        )),
        574 => Some(erase_function(
            terminator_vkGetDisplayModeProperties2KHR as *const (),
        )),
        575 => Some(erase_function(
            terminator_vkGetDisplayModePropertiesKHR as *const (),
        )),
        576 => Some(erase_function(
            terminator_vkGetDisplayPlaneCapabilities2KHR as *const (),
        )),
        577 => Some(erase_function(
            terminator_vkGetDisplayPlaneCapabilitiesKHR as *const (),
        )),
        578 => Some(erase_function(
            terminator_vkGetDisplayPlaneSupportedDisplaysKHR as *const (),
        )),
        579 => Some(erase_function(terminator_vkGetDrmDisplayEXT as *const ())),
        634 => Some(erase_function(
            terminator_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT as *const (),
        )),
        635 => Some(erase_function(
            terminator_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR as *const (),
        )),
        636 => Some(erase_function(
            terminator_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV
                as *const (),
        )),
        637 => Some(erase_function(
            terminator_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT as *const (),
        )),
        638 => Some(erase_function(
            terminator_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR as *const (),
        )),
        639 => Some(erase_function(
            terminator_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV as *const (),
        )),
        640 => Some(erase_function(
            terminator_vkGetPhysicalDeviceCooperativeVectorPropertiesNV as *const (),
        )),
        641 => Some(erase_function(
            terminator_vkGetPhysicalDeviceDescriptorSizeEXT as *const (),
        )),
        #[cfg(feature = "wsi-directfb")]
        642 => Some(erase_function(
            terminator_vkGetPhysicalDeviceDirectFBPresentationSupportEXT as *const (),
        )),
        643 => Some(erase_function(
            terminator_vkGetPhysicalDeviceDisplayPlaneProperties2KHR as *const (),
        )),
        644 => Some(erase_function(
            terminator_vkGetPhysicalDeviceDisplayPlanePropertiesKHR as *const (),
        )),
        645 => Some(erase_function(
            terminator_vkGetPhysicalDeviceDisplayProperties2KHR as *const (),
        )),
        646 => Some(erase_function(
            terminator_vkGetPhysicalDeviceDisplayPropertiesKHR as *const (),
        )),
        647 => Some(erase_function(
            terminator_vkGetPhysicalDeviceExternalBufferProperties as *const (),
        )),
        648 => Some(erase_function(
            terminator_vkGetPhysicalDeviceExternalBufferPropertiesKHR as *const (),
        )),
        649 => Some(erase_function(
            terminator_vkGetPhysicalDeviceExternalFenceProperties as *const (),
        )),
        650 => Some(erase_function(
            terminator_vkGetPhysicalDeviceExternalFencePropertiesKHR as *const (),
        )),
        651 => Some(erase_function(
            terminator_vkGetPhysicalDeviceExternalImageFormatPropertiesNV as *const (),
        )),
        652 => Some(erase_function(
            terminator_vkGetPhysicalDeviceExternalSemaphoreProperties as *const (),
        )),
        653 => Some(erase_function(
            terminator_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR as *const (),
        )),
        654 => Some(erase_function(
            terminator_vkGetPhysicalDeviceExternalTensorPropertiesARM as *const (),
        )),
        655 => Some(erase_function(
            terminator_vkGetPhysicalDeviceFeatures as *const (),
        )),
        656 => Some(erase_function(
            terminator_vkGetPhysicalDeviceFeatures2 as *const (),
        )),
        657 => Some(erase_function(
            terminator_vkGetPhysicalDeviceFeatures2KHR as *const (),
        )),
        658 => Some(erase_function(
            terminator_vkGetPhysicalDeviceFormatProperties as *const (),
        )),
        659 => Some(erase_function(
            terminator_vkGetPhysicalDeviceFormatProperties2 as *const (),
        )),
        660 => Some(erase_function(
            terminator_vkGetPhysicalDeviceFormatProperties2KHR as *const (),
        )),
        661 => Some(erase_function(
            terminator_vkGetPhysicalDeviceFragmentShadingRatesKHR as *const (),
        )),
        662 => Some(erase_function(
            terminator_vkGetPhysicalDeviceImageFormatProperties as *const (),
        )),
        663 => Some(erase_function(
            terminator_vkGetPhysicalDeviceImageFormatProperties2 as *const (),
        )),
        664 => Some(erase_function(
            terminator_vkGetPhysicalDeviceImageFormatProperties2KHR as *const (),
        )),
        665 => Some(erase_function(
            terminator_vkGetPhysicalDeviceMemoryProperties as *const (),
        )),
        666 => Some(erase_function(
            terminator_vkGetPhysicalDeviceMemoryProperties2 as *const (),
        )),
        667 => Some(erase_function(
            terminator_vkGetPhysicalDeviceMemoryProperties2KHR as *const (),
        )),
        668 => Some(erase_function(
            terminator_vkGetPhysicalDeviceMultisamplePropertiesEXT as *const (),
        )),
        669 => Some(erase_function(
            terminator_vkGetPhysicalDeviceOpticalFlowImageFormatsNV as *const (),
        )),
        670 => Some(erase_function(
            terminator_vkGetPhysicalDevicePresentRectanglesKHR as *const (),
        )),
        671 => Some(erase_function(
            terminator_vkGetPhysicalDeviceProperties as *const (),
        )),
        672 => Some(erase_function(
            terminator_vkGetPhysicalDeviceProperties2 as *const (),
        )),
        673 => Some(erase_function(
            terminator_vkGetPhysicalDeviceProperties2KHR as *const (),
        )),
        674 => Some(erase_function(
            terminator_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM
                as *const (),
        )),
        675 => Some(erase_function(
            terminator_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM
                as *const (),
        )),
        676 => Some(erase_function(
            terminator_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM
                as *const (),
        )),
        677 => Some(erase_function(
            terminator_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM as *const (),
        )),
        678 => Some(erase_function(
            terminator_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR as *const (),
        )),
        679 => Some(erase_function(
            terminator_vkGetPhysicalDeviceQueueFamilyProperties as *const (),
        )),
        680 => Some(erase_function(
            terminator_vkGetPhysicalDeviceQueueFamilyProperties2 as *const (),
        )),
        681 => Some(erase_function(
            terminator_vkGetPhysicalDeviceQueueFamilyProperties2KHR as *const (),
        )),
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        682 => Some(erase_function(
            terminator_vkGetPhysicalDeviceScreenPresentationSupportQNX as *const (),
        )),
        683 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSparseImageFormatProperties as *const (),
        )),
        684 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSparseImageFormatProperties2 as *const (),
        )),
        685 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSparseImageFormatProperties2KHR as *const (),
        )),
        686 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV
                as *const (),
        )),
        687 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSurfaceCapabilities2EXT as *const (),
        )),
        688 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSurfaceCapabilities2KHR as *const (),
        )),
        689 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSurfaceCapabilitiesKHR as *const (),
        )),
        690 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSurfaceFormats2KHR as *const (),
        )),
        691 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSurfaceFormatsKHR as *const (),
        )),
        #[cfg(target_os = "windows")]
        692 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSurfacePresentModes2EXT as *const (),
        )),
        693 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSurfacePresentModesKHR as *const (),
        )),
        694 => Some(erase_function(
            terminator_vkGetPhysicalDeviceSurfaceSupportKHR as *const (),
        )),
        695 => Some(erase_function(
            terminator_vkGetPhysicalDeviceToolProperties as *const (),
        )),
        696 => Some(erase_function(
            terminator_vkGetPhysicalDeviceToolPropertiesEXT as *const (),
        )),
        #[cfg(feature = "platform-ubm")]
        697 => Some(erase_function(
            terminator_vkGetPhysicalDeviceUbmPresentationSupportSEC as *const (),
        )),
        698 => Some(erase_function(
            terminator_vkGetPhysicalDeviceVideoCapabilitiesKHR as *const (),
        )),
        699 => Some(erase_function(
            terminator_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR as *const (),
        )),
        700 => Some(erase_function(
            terminator_vkGetPhysicalDeviceVideoFormatPropertiesKHR as *const (),
        )),
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
        701 => Some(erase_function(
            terminator_vkGetPhysicalDeviceWaylandPresentationSupportKHR as *const (),
        )),
        #[cfg(target_os = "windows")]
        702 => Some(erase_function(
            terminator_vkGetPhysicalDeviceWin32PresentationSupportKHR as *const (),
        )),
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
        703 => Some(erase_function(
            terminator_vkGetPhysicalDeviceXcbPresentationSupportKHR as *const (),
        )),
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
        704 => Some(erase_function(
            terminator_vkGetPhysicalDeviceXlibPresentationSupportKHR as *const (),
        )),
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
        719 => Some(erase_function(
            terminator_vkGetRandROutputDisplayEXT as *const (),
        )),
        #[cfg(target_os = "windows")]
        752 => Some(erase_function(terminator_vkGetWinrtDisplayNV as *const ())),
        784 => Some(erase_function(terminator_vkReleaseDisplayEXT as *const ())),
        _ => None,
    }
}
#[inline(never)]
pub(crate) fn icd_device_terminator_proc_addr(
    table: &IcdDeviceTerminatorDispatchTable,
    id: u16,
) -> PFN_vkVoidFunction {
    match id {
        459 => table.vkDestroyDevice.map(erase_function),
        425 => table.vkCreateSwapchainKHR.map(erase_function),
        558 => table
            .vkGetDeviceGroupSurfacePresentModesKHR
            .map(erase_function),
        422 => table.vkCreateSharedSwapchainsKHR.map(erase_function),
        438 => table.vkDebugMarkerSetObjectTagEXT.map(erase_function),
        437 => table.vkDebugMarkerSetObjectNameEXT.map(erase_function),
        800 => table.vkSetDebugUtilsObjectNameEXT.map(erase_function),
        801 => table.vkSetDebugUtilsObjectTagEXT.map(erase_function),
        767 => table.vkQueueBeginDebugUtilsLabelEXT.map(erase_function),
        769 => table.vkQueueEndDebugUtilsLabelEXT.map(erase_function),
        770 => table.vkQueueInsertDebugUtilsLabelEXT.map(erase_function),
        30 => table.vkCmdBeginDebugUtilsLabelEXT.map(erase_function),
        163 => table.vkCmdEndDebugUtilsLabelEXT.map(erase_function),
        186 => table.vkCmdInsertDebugUtilsLabelEXT.map(erase_function),
        #[cfg(target_os = "windows")]
        557 => table
            .vkGetDeviceGroupSurfacePresentModes2EXT
            .map(erase_function),
        _ => None,
    }
}
