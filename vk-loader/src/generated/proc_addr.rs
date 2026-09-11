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
pub(crate) fn global_proc_addr(id: u16) -> PFN_vkVoidFunction {
    let address = match id {
        398 => vkCreateInstance as *const (),
        500 => vkEnumerateInstanceExtensionProperties as *const (),
        501 => vkEnumerateInstanceLayerProperties as *const (),
        502 => vkEnumerateInstanceVersion as *const (),
        612 => vkGetInstanceProcAddr as *const (),
        _ => return None,
    };
    Some(erase_function(address))
}
#[inline(never)]
#[allow(clippy::too_many_lines)]
pub(crate) fn exported_proc_addr(id: u16) -> PFN_vkVoidFunction {
    let address = match id {
        0 => vkAcquireDrmDisplayEXT as *const (),
        #[cfg(target_os = "windows")]
        1 => vkAcquireFullScreenExclusiveModeEXT as *const (),
        2 => vkAcquireNextImage2KHR as *const (),
        3 => vkAcquireNextImageKHR as *const (),
        4 => vkAcquirePerformanceConfigurationINTEL as *const (),
        5 => vkAcquireProfilingLockKHR as *const (),
        #[cfg(target_os = "windows")]
        6 => vkAcquireWinrtDisplayNV as *const (),
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
        7 => vkAcquireXlibDisplayEXT as *const (),
        8 => vkAllocateCommandBuffers as *const (),
        9 => vkAllocateDescriptorSets as *const (),
        10 => vkAllocateMemory as *const (),
        11 => vkAntiLagUpdateAMD as *const (),
        12 => vkBeginCommandBuffer as *const (),
        13 => vkBindAccelerationStructureMemoryNV as *const (),
        14 => vkBindBufferMemory as *const (),
        15 => vkBindBufferMemory2 as *const (),
        16 => vkBindBufferMemory2KHR as *const (),
        17 => vkBindDataGraphPipelineSessionMemoryARM as *const (),
        18 => vkBindImageMemory as *const (),
        19 => vkBindImageMemory2 as *const (),
        20 => vkBindImageMemory2KHR as *const (),
        21 => vkBindOpticalFlowSessionImageNV as *const (),
        22 => vkBindTensorMemoryARM as *const (),
        23 => vkBindVideoSessionMemoryKHR as *const (),
        24 => vkBuildAccelerationStructuresKHR as *const (),
        25 => vkBuildMicromapsEXT as *const (),
        26 => vkClearShaderInstrumentationMetricsARM as *const (),
        27 => vkCmdBeginConditionalRendering2EXT as *const (),
        28 => vkCmdBeginConditionalRenderingEXT as *const (),
        29 => vkCmdBeginCustomResolveEXT as *const (),
        30 => vkCmdBeginDebugUtilsLabelEXT as *const (),
        31 => vkCmdBeginGpaSampleAMD as *const (),
        32 => vkCmdBeginGpaSessionAMD as *const (),
        33 => vkCmdBeginPerTileExecutionQCOM as *const (),
        34 => vkCmdBeginQuery as *const (),
        35 => vkCmdBeginQueryIndexedEXT as *const (),
        36 => vkCmdBeginRenderPass as *const (),
        37 => vkCmdBeginRenderPass2 as *const (),
        38 => vkCmdBeginRenderPass2KHR as *const (),
        39 => vkCmdBeginRendering as *const (),
        40 => vkCmdBeginRenderingKHR as *const (),
        41 => vkCmdBeginShaderInstrumentationARM as *const (),
        42 => vkCmdBeginTransformFeedback2EXT as *const (),
        43 => vkCmdBeginTransformFeedbackEXT as *const (),
        44 => vkCmdBeginVideoCodingKHR as *const (),
        45 => vkCmdBindDescriptorBufferEmbeddedSamplers2EXT as *const (),
        46 => vkCmdBindDescriptorBufferEmbeddedSamplersEXT as *const (),
        47 => vkCmdBindDescriptorBuffersEXT as *const (),
        48 => vkCmdBindDescriptorSets as *const (),
        49 => vkCmdBindDescriptorSets2 as *const (),
        50 => vkCmdBindDescriptorSets2KHR as *const (),
        51 => vkCmdBindIndexBuffer as *const (),
        52 => vkCmdBindIndexBuffer2 as *const (),
        53 => vkCmdBindIndexBuffer2KHR as *const (),
        54 => vkCmdBindIndexBuffer3KHR as *const (),
        55 => vkCmdBindInvocationMaskHUAWEI as *const (),
        56 => vkCmdBindPipeline as *const (),
        57 => vkCmdBindPipelineShaderGroupNV as *const (),
        58 => vkCmdBindResourceHeapEXT as *const (),
        59 => vkCmdBindSamplerHeapEXT as *const (),
        60 => vkCmdBindShadersEXT as *const (),
        61 => vkCmdBindShadingRateImageNV as *const (),
        62 => vkCmdBindTileMemoryQCOM as *const (),
        63 => vkCmdBindTransformFeedbackBuffers2EXT as *const (),
        64 => vkCmdBindTransformFeedbackBuffersEXT as *const (),
        65 => vkCmdBindVertexBuffers as *const (),
        66 => vkCmdBindVertexBuffers2 as *const (),
        67 => vkCmdBindVertexBuffers2EXT as *const (),
        68 => vkCmdBindVertexBuffers3KHR as *const (),
        69 => vkCmdBlitImage as *const (),
        70 => vkCmdBlitImage2 as *const (),
        71 => vkCmdBlitImage2KHR as *const (),
        72 => vkCmdBuildAccelerationStructureNV as *const (),
        73 => vkCmdBuildAccelerationStructuresIndirectKHR as *const (),
        74 => vkCmdBuildAccelerationStructuresKHR as *const (),
        75 => vkCmdBuildClusterAccelerationStructureIndirectNV as *const (),
        76 => vkCmdBuildMicromapsEXT as *const (),
        77 => vkCmdBuildPartitionedAccelerationStructuresNV as *const (),
        78 => vkCmdClearAttachments as *const (),
        79 => vkCmdClearColorImage as *const (),
        80 => vkCmdClearDepthStencilImage as *const (),
        81 => vkCmdControlVideoCodingKHR as *const (),
        82 => vkCmdConvertCooperativeVectorMatrixNV as *const (),
        83 => vkCmdCopyAccelerationStructureKHR as *const (),
        84 => vkCmdCopyAccelerationStructureNV as *const (),
        85 => vkCmdCopyAccelerationStructureToMemoryKHR as *const (),
        86 => vkCmdCopyBuffer as *const (),
        87 => vkCmdCopyBuffer2 as *const (),
        88 => vkCmdCopyBuffer2KHR as *const (),
        89 => vkCmdCopyBufferToImage as *const (),
        90 => vkCmdCopyBufferToImage2 as *const (),
        91 => vkCmdCopyBufferToImage2KHR as *const (),
        92 => vkCmdCopyGpaSessionResultsAMD as *const (),
        93 => vkCmdCopyImage as *const (),
        94 => vkCmdCopyImage2 as *const (),
        95 => vkCmdCopyImage2KHR as *const (),
        96 => vkCmdCopyImageToBuffer as *const (),
        97 => vkCmdCopyImageToBuffer2 as *const (),
        98 => vkCmdCopyImageToBuffer2KHR as *const (),
        99 => vkCmdCopyImageToMemoryKHR as *const (),
        100 => vkCmdCopyMemoryIndirectKHR as *const (),
        101 => vkCmdCopyMemoryIndirectNV as *const (),
        102 => vkCmdCopyMemoryKHR as *const (),
        103 => vkCmdCopyMemoryToAccelerationStructureKHR as *const (),
        104 => vkCmdCopyMemoryToImageIndirectKHR as *const (),
        105 => vkCmdCopyMemoryToImageIndirectNV as *const (),
        106 => vkCmdCopyMemoryToImageKHR as *const (),
        107 => vkCmdCopyMemoryToMicromapEXT as *const (),
        108 => vkCmdCopyMicromapEXT as *const (),
        109 => vkCmdCopyMicromapToMemoryEXT as *const (),
        110 => vkCmdCopyQueryPoolResults as *const (),
        111 => vkCmdCopyQueryPoolResultsToMemoryKHR as *const (),
        112 => vkCmdCopyTensorARM as *const (),
        113 => vkCmdCuLaunchKernelNVX as *const (),
        #[cfg(feature = "beta-extensions")]
        114 => vkCmdCudaLaunchKernelNV as *const (),
        115 => vkCmdDebugMarkerBeginEXT as *const (),
        116 => vkCmdDebugMarkerEndEXT as *const (),
        117 => vkCmdDebugMarkerInsertEXT as *const (),
        118 => vkCmdDecodeVideoKHR as *const (),
        119 => vkCmdDecompressMemoryEXT as *const (),
        120 => vkCmdDecompressMemoryIndirectCountEXT as *const (),
        121 => vkCmdDecompressMemoryIndirectCountNV as *const (),
        122 => vkCmdDecompressMemoryNV as *const (),
        123 => vkCmdDispatch as *const (),
        124 => vkCmdDispatchBase as *const (),
        125 => vkCmdDispatchBaseKHR as *const (),
        126 => vkCmdDispatchDataGraphARM as *const (),
        #[cfg(feature = "beta-extensions")]
        127 => vkCmdDispatchGraphAMDX as *const (),
        #[cfg(feature = "beta-extensions")]
        128 => vkCmdDispatchGraphIndirectAMDX as *const (),
        #[cfg(feature = "beta-extensions")]
        129 => vkCmdDispatchGraphIndirectCountAMDX as *const (),
        130 => vkCmdDispatchIndirect as *const (),
        131 => vkCmdDispatchIndirect2KHR as *const (),
        132 => vkCmdDispatchTileQCOM as *const (),
        133 => vkCmdDraw as *const (),
        134 => vkCmdDrawClusterHUAWEI as *const (),
        135 => vkCmdDrawClusterIndirectHUAWEI as *const (),
        136 => vkCmdDrawIndexed as *const (),
        137 => vkCmdDrawIndexedIndirect as *const (),
        138 => vkCmdDrawIndexedIndirect2KHR as *const (),
        139 => vkCmdDrawIndexedIndirectCount as *const (),
        140 => vkCmdDrawIndexedIndirectCount2KHR as *const (),
        141 => vkCmdDrawIndexedIndirectCountAMD as *const (),
        142 => vkCmdDrawIndexedIndirectCountKHR as *const (),
        143 => vkCmdDrawIndirect as *const (),
        144 => vkCmdDrawIndirect2KHR as *const (),
        145 => vkCmdDrawIndirectByteCount2EXT as *const (),
        146 => vkCmdDrawIndirectByteCountEXT as *const (),
        147 => vkCmdDrawIndirectCount as *const (),
        148 => vkCmdDrawIndirectCount2KHR as *const (),
        149 => vkCmdDrawIndirectCountAMD as *const (),
        150 => vkCmdDrawIndirectCountKHR as *const (),
        151 => vkCmdDrawMeshTasksEXT as *const (),
        152 => vkCmdDrawMeshTasksIndirect2EXT as *const (),
        153 => vkCmdDrawMeshTasksIndirectCount2EXT as *const (),
        154 => vkCmdDrawMeshTasksIndirectCountEXT as *const (),
        155 => vkCmdDrawMeshTasksIndirectCountNV as *const (),
        156 => vkCmdDrawMeshTasksIndirectEXT as *const (),
        157 => vkCmdDrawMeshTasksIndirectNV as *const (),
        158 => vkCmdDrawMeshTasksNV as *const (),
        159 => vkCmdDrawMultiEXT as *const (),
        160 => vkCmdDrawMultiIndexedEXT as *const (),
        161 => vkCmdEncodeVideoKHR as *const (),
        162 => vkCmdEndConditionalRenderingEXT as *const (),
        163 => vkCmdEndDebugUtilsLabelEXT as *const (),
        164 => vkCmdEndGpaSampleAMD as *const (),
        165 => vkCmdEndGpaSessionAMD as *const (),
        166 => vkCmdEndPerTileExecutionQCOM as *const (),
        167 => vkCmdEndQuery as *const (),
        168 => vkCmdEndQueryIndexedEXT as *const (),
        169 => vkCmdEndRenderPass as *const (),
        170 => vkCmdEndRenderPass2 as *const (),
        171 => vkCmdEndRenderPass2KHR as *const (),
        172 => vkCmdEndRendering as *const (),
        173 => vkCmdEndRendering2EXT as *const (),
        174 => vkCmdEndRendering2KHR as *const (),
        175 => vkCmdEndRenderingKHR as *const (),
        176 => vkCmdEndShaderInstrumentationARM as *const (),
        177 => vkCmdEndTransformFeedback2EXT as *const (),
        178 => vkCmdEndTransformFeedbackEXT as *const (),
        179 => vkCmdEndVideoCodingKHR as *const (),
        180 => vkCmdExecuteCommands as *const (),
        181 => vkCmdExecuteGeneratedCommandsEXT as *const (),
        182 => vkCmdExecuteGeneratedCommandsNV as *const (),
        183 => vkCmdFillBuffer as *const (),
        184 => vkCmdFillMemoryKHR as *const (),
        #[cfg(feature = "beta-extensions")]
        185 => vkCmdInitializeGraphScratchMemoryAMDX as *const (),
        186 => vkCmdInsertDebugUtilsLabelEXT as *const (),
        187 => vkCmdNextSubpass as *const (),
        188 => vkCmdNextSubpass2 as *const (),
        189 => vkCmdNextSubpass2KHR as *const (),
        190 => vkCmdOpticalFlowExecuteNV as *const (),
        191 => vkCmdPipelineBarrier as *const (),
        192 => vkCmdPipelineBarrier2 as *const (),
        193 => vkCmdPipelineBarrier2KHR as *const (),
        194 => vkCmdPreprocessGeneratedCommandsEXT as *const (),
        195 => vkCmdPreprocessGeneratedCommandsNV as *const (),
        196 => vkCmdPushConstants as *const (),
        197 => vkCmdPushConstants2 as *const (),
        198 => vkCmdPushConstants2KHR as *const (),
        199 => vkCmdPushDataEXT as *const (),
        200 => vkCmdPushDescriptorSet as *const (),
        201 => vkCmdPushDescriptorSet2 as *const (),
        202 => vkCmdPushDescriptorSet2KHR as *const (),
        203 => vkCmdPushDescriptorSetKHR as *const (),
        204 => vkCmdPushDescriptorSetWithTemplate as *const (),
        205 => vkCmdPushDescriptorSetWithTemplate2 as *const (),
        206 => vkCmdPushDescriptorSetWithTemplate2KHR as *const (),
        207 => vkCmdPushDescriptorSetWithTemplateKHR as *const (),
        208 => vkCmdResetEvent as *const (),
        209 => vkCmdResetEvent2 as *const (),
        210 => vkCmdResetEvent2KHR as *const (),
        211 => vkCmdResetQueryPool as *const (),
        212 => vkCmdResolveImage as *const (),
        213 => vkCmdResolveImage2 as *const (),
        214 => vkCmdResolveImage2KHR as *const (),
        215 => vkCmdSetAlphaToCoverageEnableEXT as *const (),
        216 => vkCmdSetAlphaToOneEnableEXT as *const (),
        217 => vkCmdSetAttachmentFeedbackLoopEnableEXT as *const (),
        218 => vkCmdSetBlendConstants as *const (),
        219 => vkCmdSetCheckpointNV as *const (),
        220 => vkCmdSetCoarseSampleOrderNV as *const (),
        221 => vkCmdSetColorBlendAdvancedEXT as *const (),
        222 => vkCmdSetColorBlendEnableEXT as *const (),
        223 => vkCmdSetColorBlendEquationEXT as *const (),
        224 => vkCmdSetColorWriteEnableEXT as *const (),
        225 => vkCmdSetColorWriteMaskEXT as *const (),
        226 => vkCmdSetComputeOccupancyPriorityNV as *const (),
        227 => vkCmdSetConservativeRasterizationModeEXT as *const (),
        228 => vkCmdSetCoverageModulationModeNV as *const (),
        229 => vkCmdSetCoverageModulationTableEnableNV as *const (),
        230 => vkCmdSetCoverageModulationTableNV as *const (),
        231 => vkCmdSetCoverageReductionModeNV as *const (),
        232 => vkCmdSetCoverageToColorEnableNV as *const (),
        233 => vkCmdSetCoverageToColorLocationNV as *const (),
        234 => vkCmdSetCullMode as *const (),
        235 => vkCmdSetCullModeEXT as *const (),
        236 => vkCmdSetDepthBias as *const (),
        237 => vkCmdSetDepthBias2EXT as *const (),
        238 => vkCmdSetDepthBiasEnable as *const (),
        239 => vkCmdSetDepthBiasEnableEXT as *const (),
        240 => vkCmdSetDepthBounds as *const (),
        241 => vkCmdSetDepthBoundsTestEnable as *const (),
        242 => vkCmdSetDepthBoundsTestEnableEXT as *const (),
        243 => vkCmdSetDepthClampEnableEXT as *const (),
        244 => vkCmdSetDepthClampRangeEXT as *const (),
        245 => vkCmdSetDepthClipEnableEXT as *const (),
        246 => vkCmdSetDepthClipNegativeOneToOneEXT as *const (),
        247 => vkCmdSetDepthCompareOp as *const (),
        248 => vkCmdSetDepthCompareOpEXT as *const (),
        249 => vkCmdSetDepthTestEnable as *const (),
        250 => vkCmdSetDepthTestEnableEXT as *const (),
        251 => vkCmdSetDepthWriteEnable as *const (),
        252 => vkCmdSetDepthWriteEnableEXT as *const (),
        253 => vkCmdSetDescriptorBufferOffsets2EXT as *const (),
        254 => vkCmdSetDescriptorBufferOffsetsEXT as *const (),
        255 => vkCmdSetDeviceMask as *const (),
        256 => vkCmdSetDeviceMaskKHR as *const (),
        257 => vkCmdSetDiscardRectangleEXT as *const (),
        258 => vkCmdSetDiscardRectangleEnableEXT as *const (),
        259 => vkCmdSetDiscardRectangleModeEXT as *const (),
        260 => vkCmdSetDispatchParametersARM as *const (),
        261 => vkCmdSetEvent as *const (),
        262 => vkCmdSetEvent2 as *const (),
        263 => vkCmdSetEvent2KHR as *const (),
        264 => vkCmdSetExclusiveScissorEnableNV as *const (),
        265 => vkCmdSetExclusiveScissorNV as *const (),
        266 => vkCmdSetExtraPrimitiveOverestimationSizeEXT as *const (),
        267 => vkCmdSetFragmentShadingRateEnumNV as *const (),
        268 => vkCmdSetFragmentShadingRateKHR as *const (),
        269 => vkCmdSetFrontFace as *const (),
        270 => vkCmdSetFrontFaceEXT as *const (),
        271 => vkCmdSetLineRasterizationModeEXT as *const (),
        272 => vkCmdSetLineStipple as *const (),
        273 => vkCmdSetLineStippleEXT as *const (),
        274 => vkCmdSetLineStippleEnableEXT as *const (),
        275 => vkCmdSetLineStippleKHR as *const (),
        276 => vkCmdSetLineWidth as *const (),
        277 => vkCmdSetLogicOpEXT as *const (),
        278 => vkCmdSetLogicOpEnableEXT as *const (),
        279 => vkCmdSetPatchControlPointsEXT as *const (),
        280 => vkCmdSetPerformanceMarkerINTEL as *const (),
        281 => vkCmdSetPerformanceOverrideINTEL as *const (),
        282 => vkCmdSetPerformanceStreamMarkerINTEL as *const (),
        283 => vkCmdSetPolygonModeEXT as *const (),
        284 => vkCmdSetPrimitiveRestartEnable as *const (),
        285 => vkCmdSetPrimitiveRestartEnableEXT as *const (),
        286 => vkCmdSetPrimitiveRestartIndexEXT as *const (),
        287 => vkCmdSetPrimitiveTopology as *const (),
        288 => vkCmdSetPrimitiveTopologyEXT as *const (),
        289 => vkCmdSetProvokingVertexModeEXT as *const (),
        290 => vkCmdSetRasterizationSamplesEXT as *const (),
        291 => vkCmdSetRasterizationStreamEXT as *const (),
        292 => vkCmdSetRasterizerDiscardEnable as *const (),
        293 => vkCmdSetRasterizerDiscardEnableEXT as *const (),
        294 => vkCmdSetRayTracingPipelineStackSizeKHR as *const (),
        295 => vkCmdSetRenderingAttachmentLocations as *const (),
        296 => vkCmdSetRenderingAttachmentLocationsKHR as *const (),
        297 => vkCmdSetRenderingInputAttachmentIndices as *const (),
        298 => vkCmdSetRenderingInputAttachmentIndicesKHR as *const (),
        299 => vkCmdSetRepresentativeFragmentTestEnableNV as *const (),
        300 => vkCmdSetSampleLocationsEXT as *const (),
        301 => vkCmdSetSampleLocationsEnableEXT as *const (),
        302 => vkCmdSetSampleMaskEXT as *const (),
        303 => vkCmdSetScissor as *const (),
        304 => vkCmdSetScissorWithCount as *const (),
        305 => vkCmdSetScissorWithCountEXT as *const (),
        306 => vkCmdSetShadingRateImageEnableNV as *const (),
        307 => vkCmdSetStencilCompareMask as *const (),
        308 => vkCmdSetStencilOp as *const (),
        309 => vkCmdSetStencilOpEXT as *const (),
        310 => vkCmdSetStencilReference as *const (),
        311 => vkCmdSetStencilTestEnable as *const (),
        312 => vkCmdSetStencilTestEnableEXT as *const (),
        313 => vkCmdSetStencilWriteMask as *const (),
        314 => vkCmdSetTessellationDomainOriginEXT as *const (),
        315 => vkCmdSetVertexInputEXT as *const (),
        316 => vkCmdSetViewport as *const (),
        317 => vkCmdSetViewportShadingRatePaletteNV as *const (),
        318 => vkCmdSetViewportSwizzleNV as *const (),
        319 => vkCmdSetViewportWScalingEnableNV as *const (),
        320 => vkCmdSetViewportWScalingNV as *const (),
        321 => vkCmdSetViewportWithCount as *const (),
        322 => vkCmdSetViewportWithCountEXT as *const (),
        323 => vkCmdSubpassShadingHUAWEI as *const (),
        324 => vkCmdTraceRaysIndirect2KHR as *const (),
        325 => vkCmdTraceRaysIndirectKHR as *const (),
        326 => vkCmdTraceRaysKHR as *const (),
        327 => vkCmdTraceRaysNV as *const (),
        328 => vkCmdUpdateBuffer as *const (),
        329 => vkCmdUpdateMemoryKHR as *const (),
        330 => vkCmdUpdatePipelineIndirectBufferNV as *const (),
        331 => vkCmdWaitEvents as *const (),
        332 => vkCmdWaitEvents2 as *const (),
        333 => vkCmdWaitEvents2KHR as *const (),
        334 => vkCmdWriteAccelerationStructuresPropertiesKHR as *const (),
        335 => vkCmdWriteAccelerationStructuresPropertiesNV as *const (),
        336 => vkCmdWriteBufferMarker2AMD as *const (),
        337 => vkCmdWriteBufferMarkerAMD as *const (),
        338 => vkCmdWriteMarkerToMemoryAMD as *const (),
        339 => vkCmdWriteMicromapsPropertiesEXT as *const (),
        340 => vkCmdWriteTimestamp as *const (),
        341 => vkCmdWriteTimestamp2 as *const (),
        342 => vkCmdWriteTimestamp2KHR as *const (),
        343 => vkCompileDeferredNV as *const (),
        344 => vkConvertCooperativeVectorMatrixNV as *const (),
        345 => vkCopyAccelerationStructureKHR as *const (),
        346 => vkCopyAccelerationStructureToMemoryKHR as *const (),
        347 => vkCopyImageToImage as *const (),
        348 => vkCopyImageToImageEXT as *const (),
        349 => vkCopyImageToMemory as *const (),
        350 => vkCopyImageToMemoryEXT as *const (),
        351 => vkCopyMemoryToAccelerationStructureKHR as *const (),
        352 => vkCopyMemoryToImage as *const (),
        353 => vkCopyMemoryToImageEXT as *const (),
        354 => vkCopyMemoryToMicromapEXT as *const (),
        355 => vkCopyMicromapEXT as *const (),
        356 => vkCopyMicromapToMemoryEXT as *const (),
        357 => vkCreateAccelerationStructure2KHR as *const (),
        358 => vkCreateAccelerationStructureKHR as *const (),
        359 => vkCreateAccelerationStructureNV as *const (),
        #[cfg(target_os = "android")]
        360 => vkCreateAndroidSurfaceKHR as *const (),
        361 => vkCreateBuffer as *const (),
        #[cfg(target_os = "fuchsia")]
        362 => vkCreateBufferCollectionFUCHSIA as *const (),
        363 => vkCreateBufferView as *const (),
        364 => vkCreateCommandPool as *const (),
        365 => vkCreateComputePipelines as *const (),
        366 => vkCreateCuFunctionNVX as *const (),
        367 => vkCreateCuModuleNVX as *const (),
        #[cfg(feature = "beta-extensions")]
        368 => vkCreateCudaFunctionNV as *const (),
        #[cfg(feature = "beta-extensions")]
        369 => vkCreateCudaModuleNV as *const (),
        370 => vkCreateDataGraphPipelineSessionARM as *const (),
        371 => vkCreateDataGraphPipelinesARM as *const (),
        372 => vkCreateDebugReportCallbackEXT as *const (),
        373 => vkCreateDebugUtilsMessengerEXT as *const (),
        374 => vkCreateDeferredOperationKHR as *const (),
        375 => vkCreateDescriptorPool as *const (),
        376 => vkCreateDescriptorSetLayout as *const (),
        377 => vkCreateDescriptorUpdateTemplate as *const (),
        378 => vkCreateDescriptorUpdateTemplateKHR as *const (),
        379 => vkCreateDevice as *const (),
        #[cfg(feature = "wsi-directfb")]
        380 => vkCreateDirectFBSurfaceEXT as *const (),
        381 => vkCreateDisplayModeKHR as *const (),
        382 => vkCreateDisplayPlaneSurfaceKHR as *const (),
        383 => vkCreateEvent as *const (),
        #[cfg(feature = "beta-extensions")]
        384 => vkCreateExecutionGraphPipelinesAMDX as *const (),
        385 => vkCreateExternalComputeQueueNV as *const (),
        386 => vkCreateFence as *const (),
        387 => vkCreateFramebuffer as *const (),
        388 => vkCreateGpaSessionAMD as *const (),
        389 => vkCreateGraphicsPipelines as *const (),
        390 => vkCreateHeadlessSurfaceEXT as *const (),
        #[cfg(target_os = "ios")]
        391 => vkCreateIOSSurfaceMVK as *const (),
        392 => vkCreateImage as *const (),
        #[cfg(target_os = "fuchsia")]
        393 => vkCreateImagePipeSurfaceFUCHSIA as *const (),
        394 => vkCreateImageView as *const (),
        395 => vkCreateIndirectCommandsLayoutEXT as *const (),
        396 => vkCreateIndirectCommandsLayoutNV as *const (),
        397 => vkCreateIndirectExecutionSetEXT as *const (),
        #[cfg(target_os = "macos")]
        399 => vkCreateMacOSSurfaceMVK as *const (),
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        400 => vkCreateMetalSurfaceEXT as *const (),
        401 => vkCreateMicromapEXT as *const (),
        402 => vkCreateOpticalFlowSessionNV as *const (),
        403 => vkCreatePipelineBinariesKHR as *const (),
        404 => vkCreatePipelineCache as *const (),
        405 => vkCreatePipelineLayout as *const (),
        406 => vkCreatePrivateDataSlot as *const (),
        407 => vkCreatePrivateDataSlotEXT as *const (),
        408 => vkCreateQueryPool as *const (),
        409 => vkCreateRayTracingPipelinesKHR as *const (),
        410 => vkCreateRayTracingPipelinesNV as *const (),
        411 => vkCreateRenderPass as *const (),
        412 => vkCreateRenderPass2 as *const (),
        413 => vkCreateRenderPass2KHR as *const (),
        414 => vkCreateSampler as *const (),
        415 => vkCreateSamplerYcbcrConversion as *const (),
        416 => vkCreateSamplerYcbcrConversionKHR as *const (),
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        417 => vkCreateScreenSurfaceQNX as *const (),
        418 => vkCreateSemaphore as *const (),
        419 => vkCreateShaderInstrumentationARM as *const (),
        420 => vkCreateShaderModule as *const (),
        421 => vkCreateShadersEXT as *const (),
        422 => vkCreateSharedSwapchainsKHR as *const (),
        #[cfg(feature = "platform-ggp")]
        423 => vkCreateStreamDescriptorSurfaceGGP as *const (),
        #[cfg(target_env = "ohos")]
        424 => vkCreateSurfaceOHOS as *const (),
        425 => vkCreateSwapchainKHR as *const (),
        426 => vkCreateTensorARM as *const (),
        427 => vkCreateTensorViewARM as *const (),
        #[cfg(feature = "platform-ubm")]
        428 => vkCreateUbmSurfaceSEC as *const (),
        429 => vkCreateValidationCacheEXT as *const (),
        #[cfg(feature = "platform-vi")]
        430 => vkCreateViSurfaceNN as *const (),
        431 => vkCreateVideoSessionKHR as *const (),
        432 => vkCreateVideoSessionParametersKHR as *const (),
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
        433 => vkCreateWaylandSurfaceKHR as *const (),
        #[cfg(target_os = "windows")]
        434 => vkCreateWin32SurfaceKHR as *const (),
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
        435 => vkCreateXcbSurfaceKHR as *const (),
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
        436 => vkCreateXlibSurfaceKHR as *const (),
        437 => vkDebugMarkerSetObjectNameEXT as *const (),
        438 => vkDebugMarkerSetObjectTagEXT as *const (),
        439 => vkDebugReportMessageEXT as *const (),
        440 => vkDeferredOperationJoinKHR as *const (),
        441 => vkDestroyAccelerationStructureKHR as *const (),
        442 => vkDestroyAccelerationStructureNV as *const (),
        443 => vkDestroyBuffer as *const (),
        #[cfg(target_os = "fuchsia")]
        444 => vkDestroyBufferCollectionFUCHSIA as *const (),
        445 => vkDestroyBufferView as *const (),
        446 => vkDestroyCommandPool as *const (),
        447 => vkDestroyCuFunctionNVX as *const (),
        448 => vkDestroyCuModuleNVX as *const (),
        #[cfg(feature = "beta-extensions")]
        449 => vkDestroyCudaFunctionNV as *const (),
        #[cfg(feature = "beta-extensions")]
        450 => vkDestroyCudaModuleNV as *const (),
        451 => vkDestroyDataGraphPipelineSessionARM as *const (),
        452 => vkDestroyDebugReportCallbackEXT as *const (),
        453 => vkDestroyDebugUtilsMessengerEXT as *const (),
        454 => vkDestroyDeferredOperationKHR as *const (),
        455 => vkDestroyDescriptorPool as *const (),
        456 => vkDestroyDescriptorSetLayout as *const (),
        457 => vkDestroyDescriptorUpdateTemplate as *const (),
        458 => vkDestroyDescriptorUpdateTemplateKHR as *const (),
        459 => vkDestroyDevice as *const (),
        460 => vkDestroyEvent as *const (),
        461 => vkDestroyExternalComputeQueueNV as *const (),
        462 => vkDestroyFence as *const (),
        463 => vkDestroyFramebuffer as *const (),
        464 => vkDestroyGpaSessionAMD as *const (),
        465 => vkDestroyImage as *const (),
        466 => vkDestroyImageView as *const (),
        467 => vkDestroyIndirectCommandsLayoutEXT as *const (),
        468 => vkDestroyIndirectCommandsLayoutNV as *const (),
        469 => vkDestroyIndirectExecutionSetEXT as *const (),
        470 => vkDestroyInstance as *const (),
        471 => vkDestroyMicromapEXT as *const (),
        472 => vkDestroyOpticalFlowSessionNV as *const (),
        473 => vkDestroyPipeline as *const (),
        474 => vkDestroyPipelineBinaryKHR as *const (),
        475 => vkDestroyPipelineCache as *const (),
        476 => vkDestroyPipelineLayout as *const (),
        477 => vkDestroyPrivateDataSlot as *const (),
        478 => vkDestroyPrivateDataSlotEXT as *const (),
        479 => vkDestroyQueryPool as *const (),
        480 => vkDestroyRenderPass as *const (),
        481 => vkDestroySampler as *const (),
        482 => vkDestroySamplerYcbcrConversion as *const (),
        483 => vkDestroySamplerYcbcrConversionKHR as *const (),
        484 => vkDestroySemaphore as *const (),
        485 => vkDestroyShaderEXT as *const (),
        486 => vkDestroyShaderInstrumentationARM as *const (),
        487 => vkDestroyShaderModule as *const (),
        488 => vkDestroySurfaceKHR as *const (),
        489 => vkDestroySwapchainKHR as *const (),
        490 => vkDestroyTensorARM as *const (),
        491 => vkDestroyTensorViewARM as *const (),
        492 => vkDestroyValidationCacheEXT as *const (),
        493 => vkDestroyVideoSessionKHR as *const (),
        494 => vkDestroyVideoSessionParametersKHR as *const (),
        495 => vkDeviceWaitIdle as *const (),
        496 => vkDisplayPowerControlEXT as *const (),
        497 => vkEndCommandBuffer as *const (),
        498 => vkEnumerateDeviceExtensionProperties as *const (),
        499 => vkEnumerateDeviceLayerProperties as *const (),
        503 => vkEnumeratePhysicalDeviceGroups as *const (),
        504 => vkEnumeratePhysicalDeviceGroupsKHR as *const (),
        505 => vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM as *const (),
        506 => vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR as *const (),
        507 => vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM as *const (),
        508 => vkEnumeratePhysicalDevices as *const (),
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        509 => vkExportMetalObjectsEXT as *const (),
        510 => vkFlushMappedMemoryRanges as *const (),
        511 => vkFreeCommandBuffers as *const (),
        512 => vkFreeDescriptorSets as *const (),
        513 => vkFreeMemory as *const (),
        514 => vkGetAccelerationStructureBuildSizesKHR as *const (),
        515 => vkGetAccelerationStructureDeviceAddressKHR as *const (),
        516 => vkGetAccelerationStructureHandleNV as *const (),
        517 => vkGetAccelerationStructureMemoryRequirementsNV as *const (),
        518 => vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT as *const (),
        #[cfg(target_os = "android")]
        519 => vkGetAndroidHardwareBufferPropertiesANDROID as *const (),
        #[cfg(target_os = "fuchsia")]
        520 => vkGetBufferCollectionPropertiesFUCHSIA as *const (),
        521 => vkGetBufferDeviceAddress as *const (),
        522 => vkGetBufferDeviceAddressEXT as *const (),
        523 => vkGetBufferDeviceAddressKHR as *const (),
        524 => vkGetBufferMemoryRequirements as *const (),
        525 => vkGetBufferMemoryRequirements2 as *const (),
        526 => vkGetBufferMemoryRequirements2KHR as *const (),
        527 => vkGetBufferOpaqueCaptureAddress as *const (),
        528 => vkGetBufferOpaqueCaptureAddressKHR as *const (),
        529 => vkGetBufferOpaqueCaptureDescriptorDataEXT as *const (),
        530 => vkGetCalibratedTimestampsEXT as *const (),
        531 => vkGetCalibratedTimestampsKHR as *const (),
        532 => vkGetClusterAccelerationStructureBuildSizesNV as *const (),
        #[cfg(feature = "beta-extensions")]
        533 => vkGetCudaModuleCacheNV as *const (),
        534 => vkGetDataGraphPipelineAvailablePropertiesARM as *const (),
        535 => vkGetDataGraphPipelinePropertiesARM as *const (),
        536 => vkGetDataGraphPipelineSessionBindPointRequirementsARM as *const (),
        537 => vkGetDataGraphPipelineSessionMemoryRequirementsARM as *const (),
        538 => vkGetDeferredOperationMaxConcurrencyKHR as *const (),
        539 => vkGetDeferredOperationResultKHR as *const (),
        540 => vkGetDescriptorEXT as *const (),
        541 => vkGetDescriptorSetHostMappingVALVE as *const (),
        542 => vkGetDescriptorSetLayoutBindingOffsetEXT as *const (),
        543 => vkGetDescriptorSetLayoutHostMappingInfoVALVE as *const (),
        544 => vkGetDescriptorSetLayoutSizeEXT as *const (),
        545 => vkGetDescriptorSetLayoutSupport as *const (),
        546 => vkGetDescriptorSetLayoutSupportKHR as *const (),
        547 => vkGetDeviceAccelerationStructureCompatibilityKHR as *const (),
        548 => vkGetDeviceBufferMemoryRequirements as *const (),
        549 => vkGetDeviceBufferMemoryRequirementsKHR as *const (),
        550 => vkGetDeviceCombinedImageSamplerIndexNVX as *const (),
        551 => vkGetDeviceFaultDebugInfoKHR as *const (),
        552 => vkGetDeviceFaultInfoEXT as *const (),
        553 => vkGetDeviceFaultReportsKHR as *const (),
        554 => vkGetDeviceGroupPeerMemoryFeatures as *const (),
        555 => vkGetDeviceGroupPeerMemoryFeaturesKHR as *const (),
        556 => vkGetDeviceGroupPresentCapabilitiesKHR as *const (),
        #[cfg(target_os = "windows")]
        557 => vkGetDeviceGroupSurfacePresentModes2EXT as *const (),
        558 => vkGetDeviceGroupSurfacePresentModesKHR as *const (),
        559 => vkGetDeviceImageMemoryRequirements as *const (),
        560 => vkGetDeviceImageMemoryRequirementsKHR as *const (),
        561 => vkGetDeviceImageSparseMemoryRequirements as *const (),
        562 => vkGetDeviceImageSparseMemoryRequirementsKHR as *const (),
        563 => vkGetDeviceImageSubresourceLayout as *const (),
        564 => vkGetDeviceImageSubresourceLayoutKHR as *const (),
        565 => vkGetDeviceMemoryCommitment as *const (),
        566 => vkGetDeviceMemoryOpaqueCaptureAddress as *const (),
        567 => vkGetDeviceMemoryOpaqueCaptureAddressKHR as *const (),
        568 => vkGetDeviceMicromapCompatibilityEXT as *const (),
        569 => vkGetDeviceProcAddr as *const (),
        570 => vkGetDeviceQueue as *const (),
        571 => vkGetDeviceQueue2 as *const (),
        572 => vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI as *const (),
        573 => vkGetDeviceTensorMemoryRequirementsARM as *const (),
        574 => vkGetDisplayModeProperties2KHR as *const (),
        575 => vkGetDisplayModePropertiesKHR as *const (),
        576 => vkGetDisplayPlaneCapabilities2KHR as *const (),
        577 => vkGetDisplayPlaneCapabilitiesKHR as *const (),
        578 => vkGetDisplayPlaneSupportedDisplaysKHR as *const (),
        579 => vkGetDrmDisplayEXT as *const (),
        580 => vkGetDynamicRenderingTilePropertiesQCOM as *const (),
        581 => vkGetEncodedVideoSessionParametersKHR as *const (),
        582 => vkGetEventStatus as *const (),
        #[cfg(feature = "beta-extensions")]
        583 => vkGetExecutionGraphPipelineNodeIndexAMDX as *const (),
        #[cfg(feature = "beta-extensions")]
        584 => vkGetExecutionGraphPipelineScratchSizeAMDX as *const (),
        585 => vkGetExternalComputeQueueDataNV as *const (),
        586 => vkGetFenceFdKHR as *const (),
        587 => vkGetFenceStatus as *const (),
        #[cfg(target_os = "windows")]
        588 => vkGetFenceWin32HandleKHR as *const (),
        589 => vkGetFramebufferTilePropertiesQCOM as *const (),
        590 => vkGetGeneratedCommandsMemoryRequirementsEXT as *const (),
        591 => vkGetGeneratedCommandsMemoryRequirementsNV as *const (),
        592 => vkGetGpaDeviceClockInfoAMD as *const (),
        593 => vkGetGpaSessionResultsAMD as *const (),
        594 => vkGetGpaSessionStatusAMD as *const (),
        595 => vkGetImageDrmFormatModifierPropertiesEXT as *const (),
        596 => vkGetImageMemoryRequirements as *const (),
        597 => vkGetImageMemoryRequirements2 as *const (),
        598 => vkGetImageMemoryRequirements2KHR as *const (),
        599 => vkGetImageOpaqueCaptureDataEXT as *const (),
        600 => vkGetImageOpaqueCaptureDescriptorDataEXT as *const (),
        601 => vkGetImageSparseMemoryRequirements as *const (),
        602 => vkGetImageSparseMemoryRequirements2 as *const (),
        603 => vkGetImageSparseMemoryRequirements2KHR as *const (),
        604 => vkGetImageSubresourceLayout as *const (),
        605 => vkGetImageSubresourceLayout2 as *const (),
        606 => vkGetImageSubresourceLayout2EXT as *const (),
        607 => vkGetImageSubresourceLayout2KHR as *const (),
        608 => vkGetImageViewAddressNVX as *const (),
        609 => vkGetImageViewHandle64NVX as *const (),
        610 => vkGetImageViewHandleNVX as *const (),
        611 => vkGetImageViewOpaqueCaptureDescriptorDataEXT as *const (),
        613 => vkGetLatencyTimingsLegacyNV as *const (),
        614 => vkGetLatencyTimingsNV as *const (),
        #[cfg(target_os = "android")]
        615 => vkGetMemoryAndroidHardwareBufferANDROID as *const (),
        616 => vkGetMemoryFdKHR as *const (),
        617 => vkGetMemoryFdPropertiesKHR as *const (),
        618 => vkGetMemoryHostPointerPropertiesEXT as *const (),
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        619 => vkGetMemoryMetalHandleEXT as *const (),
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        620 => vkGetMemoryMetalHandlePropertiesEXT as *const (),
        #[cfg(target_env = "ohos")]
        621 => vkGetMemoryNativeBufferOHOS as *const (),
        622 => vkGetMemoryRemoteAddressNV as *const (),
        #[cfg(target_os = "windows")]
        623 => vkGetMemoryWin32HandleKHR as *const (),
        #[cfg(target_os = "windows")]
        624 => vkGetMemoryWin32HandleNV as *const (),
        #[cfg(target_os = "windows")]
        625 => vkGetMemoryWin32HandlePropertiesKHR as *const (),
        #[cfg(target_os = "fuchsia")]
        626 => vkGetMemoryZirconHandleFUCHSIA as *const (),
        #[cfg(target_os = "fuchsia")]
        627 => vkGetMemoryZirconHandlePropertiesFUCHSIA as *const (),
        628 => vkGetMicromapBuildSizesEXT as *const (),
        #[cfg(target_env = "ohos")]
        629 => vkGetNativeBufferPropertiesOHOS as *const (),
        630 => vkGetPartitionedAccelerationStructuresBuildSizesNV as *const (),
        631 => vkGetPastPresentationTimingEXT as *const (),
        632 => vkGetPastPresentationTimingGOOGLE as *const (),
        633 => vkGetPerformanceParameterINTEL as *const (),
        634 => vkGetPhysicalDeviceCalibrateableTimeDomainsEXT as *const (),
        635 => vkGetPhysicalDeviceCalibrateableTimeDomainsKHR as *const (),
        636 => vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV as *const (),
        637 => vkGetPhysicalDeviceCooperativeMatrixProperties2EXT as *const (),
        638 => vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR as *const (),
        639 => vkGetPhysicalDeviceCooperativeMatrixPropertiesNV as *const (),
        640 => vkGetPhysicalDeviceCooperativeVectorPropertiesNV as *const (),
        641 => vkGetPhysicalDeviceDescriptorSizeEXT as *const (),
        #[cfg(feature = "wsi-directfb")]
        642 => vkGetPhysicalDeviceDirectFBPresentationSupportEXT as *const (),
        643 => vkGetPhysicalDeviceDisplayPlaneProperties2KHR as *const (),
        644 => vkGetPhysicalDeviceDisplayPlanePropertiesKHR as *const (),
        645 => vkGetPhysicalDeviceDisplayProperties2KHR as *const (),
        646 => vkGetPhysicalDeviceDisplayPropertiesKHR as *const (),
        647 => vkGetPhysicalDeviceExternalBufferProperties as *const (),
        648 => vkGetPhysicalDeviceExternalBufferPropertiesKHR as *const (),
        649 => vkGetPhysicalDeviceExternalFenceProperties as *const (),
        650 => vkGetPhysicalDeviceExternalFencePropertiesKHR as *const (),
        651 => vkGetPhysicalDeviceExternalImageFormatPropertiesNV as *const (),
        652 => vkGetPhysicalDeviceExternalSemaphoreProperties as *const (),
        653 => vkGetPhysicalDeviceExternalSemaphorePropertiesKHR as *const (),
        654 => vkGetPhysicalDeviceExternalTensorPropertiesARM as *const (),
        655 => vkGetPhysicalDeviceFeatures as *const (),
        656 => vkGetPhysicalDeviceFeatures2 as *const (),
        657 => vkGetPhysicalDeviceFeatures2KHR as *const (),
        658 => vkGetPhysicalDeviceFormatProperties as *const (),
        659 => vkGetPhysicalDeviceFormatProperties2 as *const (),
        660 => vkGetPhysicalDeviceFormatProperties2KHR as *const (),
        661 => vkGetPhysicalDeviceFragmentShadingRatesKHR as *const (),
        662 => vkGetPhysicalDeviceImageFormatProperties as *const (),
        663 => vkGetPhysicalDeviceImageFormatProperties2 as *const (),
        664 => vkGetPhysicalDeviceImageFormatProperties2KHR as *const (),
        665 => vkGetPhysicalDeviceMemoryProperties as *const (),
        666 => vkGetPhysicalDeviceMemoryProperties2 as *const (),
        667 => vkGetPhysicalDeviceMemoryProperties2KHR as *const (),
        668 => vkGetPhysicalDeviceMultisamplePropertiesEXT as *const (),
        669 => vkGetPhysicalDeviceOpticalFlowImageFormatsNV as *const (),
        670 => vkGetPhysicalDevicePresentRectanglesKHR as *const (),
        671 => vkGetPhysicalDeviceProperties as *const (),
        672 => vkGetPhysicalDeviceProperties2 as *const (),
        673 => vkGetPhysicalDeviceProperties2KHR as *const (),
        674 => vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM as *const (),
        675 => vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM as *const (),
        676 => vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM as *const (),
        677 => vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM as *const (),
        678 => vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR as *const (),
        679 => vkGetPhysicalDeviceQueueFamilyProperties as *const (),
        680 => vkGetPhysicalDeviceQueueFamilyProperties2 as *const (),
        681 => vkGetPhysicalDeviceQueueFamilyProperties2KHR as *const (),
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        682 => vkGetPhysicalDeviceScreenPresentationSupportQNX as *const (),
        683 => vkGetPhysicalDeviceSparseImageFormatProperties as *const (),
        684 => vkGetPhysicalDeviceSparseImageFormatProperties2 as *const (),
        685 => vkGetPhysicalDeviceSparseImageFormatProperties2KHR as *const (),
        686 => vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV as *const (),
        687 => vkGetPhysicalDeviceSurfaceCapabilities2EXT as *const (),
        688 => vkGetPhysicalDeviceSurfaceCapabilities2KHR as *const (),
        689 => vkGetPhysicalDeviceSurfaceCapabilitiesKHR as *const (),
        690 => vkGetPhysicalDeviceSurfaceFormats2KHR as *const (),
        691 => vkGetPhysicalDeviceSurfaceFormatsKHR as *const (),
        #[cfg(target_os = "windows")]
        692 => vkGetPhysicalDeviceSurfacePresentModes2EXT as *const (),
        693 => vkGetPhysicalDeviceSurfacePresentModesKHR as *const (),
        694 => vkGetPhysicalDeviceSurfaceSupportKHR as *const (),
        695 => vkGetPhysicalDeviceToolProperties as *const (),
        696 => vkGetPhysicalDeviceToolPropertiesEXT as *const (),
        #[cfg(feature = "platform-ubm")]
        697 => vkGetPhysicalDeviceUbmPresentationSupportSEC as *const (),
        698 => vkGetPhysicalDeviceVideoCapabilitiesKHR as *const (),
        699 => vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR as *const (),
        700 => vkGetPhysicalDeviceVideoFormatPropertiesKHR as *const (),
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
        701 => vkGetPhysicalDeviceWaylandPresentationSupportKHR as *const (),
        #[cfg(target_os = "windows")]
        702 => vkGetPhysicalDeviceWin32PresentationSupportKHR as *const (),
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
        703 => vkGetPhysicalDeviceXcbPresentationSupportKHR as *const (),
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
        704 => vkGetPhysicalDeviceXlibPresentationSupportKHR as *const (),
        705 => vkGetPipelineBinaryDataKHR as *const (),
        706 => vkGetPipelineCacheData as *const (),
        707 => vkGetPipelineExecutableInternalRepresentationsKHR as *const (),
        708 => vkGetPipelineExecutablePropertiesKHR as *const (),
        709 => vkGetPipelineExecutableStatisticsKHR as *const (),
        710 => vkGetPipelineIndirectDeviceAddressNV as *const (),
        711 => vkGetPipelineIndirectMemoryRequirementsNV as *const (),
        712 => vkGetPipelineKeyKHR as *const (),
        713 => vkGetPipelinePropertiesEXT as *const (),
        714 => vkGetPrivateData as *const (),
        715 => vkGetPrivateDataEXT as *const (),
        716 => vkGetQueryPoolResults as *const (),
        717 => vkGetQueueCheckpointData2NV as *const (),
        718 => vkGetQueueCheckpointDataNV as *const (),
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
        719 => vkGetRandROutputDisplayEXT as *const (),
        720 => vkGetRayTracingCaptureReplayShaderGroupHandlesKHR as *const (),
        721 => vkGetRayTracingShaderGroupHandlesKHR as *const (),
        722 => vkGetRayTracingShaderGroupHandlesNV as *const (),
        723 => vkGetRayTracingShaderGroupStackSizeKHR as *const (),
        724 => vkGetRefreshCycleDurationGOOGLE as *const (),
        725 => vkGetRenderAreaGranularity as *const (),
        726 => vkGetRenderingAreaGranularity as *const (),
        727 => vkGetRenderingAreaGranularityKHR as *const (),
        728 => vkGetSamplerOpaqueCaptureDescriptorDataEXT as *const (),
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        729 => vkGetScreenBufferPropertiesQNX as *const (),
        730 => vkGetSemaphoreCounterValue as *const (),
        731 => vkGetSemaphoreCounterValueKHR as *const (),
        732 => vkGetSemaphoreFdKHR as *const (),
        #[cfg(target_os = "windows")]
        733 => vkGetSemaphoreWin32HandleKHR as *const (),
        #[cfg(target_os = "fuchsia")]
        734 => vkGetSemaphoreZirconHandleFUCHSIA as *const (),
        735 => vkGetShaderBinaryDataEXT as *const (),
        736 => vkGetShaderInfoAMD as *const (),
        737 => vkGetShaderInstrumentationValuesARM as *const (),
        738 => vkGetShaderModuleCreateInfoIdentifierEXT as *const (),
        739 => vkGetShaderModuleIdentifierEXT as *const (),
        740 => vkGetSleepStatusLegacyNV as *const (),
        741 => vkGetSwapchainCounterEXT as *const (),
        742 => vkGetSwapchainImagesKHR as *const (),
        743 => vkGetSwapchainStatusKHR as *const (),
        744 => vkGetSwapchainTimeDomainPropertiesEXT as *const (),
        745 => vkGetSwapchainTimingPropertiesEXT as *const (),
        746 => vkGetTensorMemoryRequirementsARM as *const (),
        747 => vkGetTensorOpaqueCaptureDataARM as *const (),
        748 => vkGetTensorOpaqueCaptureDescriptorDataARM as *const (),
        749 => vkGetTensorViewOpaqueCaptureDescriptorDataARM as *const (),
        750 => vkGetValidationCacheDataEXT as *const (),
        751 => vkGetVideoSessionMemoryRequirementsKHR as *const (),
        #[cfg(target_os = "windows")]
        752 => vkGetWinrtDisplayNV as *const (),
        753 => vkImportFenceFdKHR as *const (),
        #[cfg(target_os = "windows")]
        754 => vkImportFenceWin32HandleKHR as *const (),
        755 => vkImportSemaphoreFdKHR as *const (),
        #[cfg(target_os = "windows")]
        756 => vkImportSemaphoreWin32HandleKHR as *const (),
        #[cfg(target_os = "fuchsia")]
        757 => vkImportSemaphoreZirconHandleFUCHSIA as *const (),
        758 => vkInitializePerformanceApiINTEL as *const (),
        759 => vkInvalidateMappedMemoryRanges as *const (),
        760 => vkLatencySleepLegacyNV as *const (),
        761 => vkLatencySleepNV as *const (),
        762 => vkMapMemory as *const (),
        763 => vkMapMemory2 as *const (),
        764 => vkMapMemory2KHR as *const (),
        765 => vkMergePipelineCaches as *const (),
        766 => vkMergeValidationCachesEXT as *const (),
        767 => vkQueueBeginDebugUtilsLabelEXT as *const (),
        768 => vkQueueBindSparse as *const (),
        769 => vkQueueEndDebugUtilsLabelEXT as *const (),
        770 => vkQueueInsertDebugUtilsLabelEXT as *const (),
        771 => vkQueueNotifyOutOfBandLegacyNV as *const (),
        772 => vkQueueNotifyOutOfBandNV as *const (),
        773 => vkQueuePresentKHR as *const (),
        774 => vkQueueSetPerfHintQCOM as *const (),
        775 => vkQueueSetPerformanceConfigurationINTEL as *const (),
        776 => vkQueueSubmit as *const (),
        777 => vkQueueSubmit2 as *const (),
        778 => vkQueueSubmit2KHR as *const (),
        779 => vkQueueWaitIdle as *const (),
        780 => vkRegisterCustomBorderColorEXT as *const (),
        781 => vkRegisterDeviceEventEXT as *const (),
        782 => vkRegisterDisplayEventEXT as *const (),
        783 => vkReleaseCapturedPipelineDataKHR as *const (),
        784 => vkReleaseDisplayEXT as *const (),
        #[cfg(target_os = "windows")]
        785 => vkReleaseFullScreenExclusiveModeEXT as *const (),
        786 => vkReleasePerformanceConfigurationINTEL as *const (),
        787 => vkReleaseProfilingLockKHR as *const (),
        788 => vkReleaseSwapchainImagesEXT as *const (),
        789 => vkReleaseSwapchainImagesKHR as *const (),
        790 => vkResetCommandBuffer as *const (),
        791 => vkResetCommandPool as *const (),
        792 => vkResetDescriptorPool as *const (),
        793 => vkResetEvent as *const (),
        794 => vkResetFences as *const (),
        795 => vkResetGpaSessionAMD as *const (),
        796 => vkResetQueryPool as *const (),
        797 => vkResetQueryPoolEXT as *const (),
        #[cfg(target_os = "fuchsia")]
        798 => vkSetBufferCollectionBufferConstraintsFUCHSIA as *const (),
        #[cfg(target_os = "fuchsia")]
        799 => vkSetBufferCollectionImageConstraintsFUCHSIA as *const (),
        800 => vkSetDebugUtilsObjectNameEXT as *const (),
        801 => vkSetDebugUtilsObjectTagEXT as *const (),
        802 => vkSetDeviceMemoryPriorityEXT as *const (),
        803 => vkSetEvent as *const (),
        804 => vkSetGpaDeviceClockModeAMD as *const (),
        805 => vkSetHdrMetadataEXT as *const (),
        806 => vkSetLatencyMarkerLegacyNV as *const (),
        807 => vkSetLatencyMarkerNV as *const (),
        808 => vkSetLatencySleepModeLegacyNV as *const (),
        809 => vkSetLatencySleepModeNV as *const (),
        810 => vkSetLocalDimmingAMD as *const (),
        811 => vkSetPrivateData as *const (),
        812 => vkSetPrivateDataEXT as *const (),
        813 => vkSetSwapchainPresentTimingQueueSizeEXT as *const (),
        814 => vkShutdownLatencyDeviceLegacyNV as *const (),
        815 => vkSignalSemaphore as *const (),
        816 => vkSignalSemaphoreKHR as *const (),
        817 => vkSubmitDebugUtilsMessageEXT as *const (),
        818 => vkTransitionImageLayout as *const (),
        819 => vkTransitionImageLayoutEXT as *const (),
        820 => vkTrimCommandPool as *const (),
        821 => vkTrimCommandPoolKHR as *const (),
        822 => vkUninitializePerformanceApiINTEL as *const (),
        823 => vkUnmapMemory as *const (),
        824 => vkUnmapMemory2 as *const (),
        825 => vkUnmapMemory2KHR as *const (),
        826 => vkUnregisterCustomBorderColorEXT as *const (),
        827 => vkUpdateDescriptorSetWithTemplate as *const (),
        828 => vkUpdateDescriptorSetWithTemplateKHR as *const (),
        829 => vkUpdateDescriptorSets as *const (),
        830 => vkUpdateIndirectExecutionSetPipelineEXT as *const (),
        831 => vkUpdateIndirectExecutionSetShaderEXT as *const (),
        832 => vkUpdateVideoSessionParametersKHR as *const (),
        833 => vkWaitForFences as *const (),
        834 => vkWaitForPresent2KHR as *const (),
        835 => vkWaitForPresentKHR as *const (),
        836 => vkWaitSemaphores as *const (),
        837 => vkWaitSemaphoresKHR as *const (),
        838 => vkWriteAccelerationStructuresPropertiesKHR as *const (),
        839 => vkWriteMicromapsPropertiesEXT as *const (),
        840 => vkWriteResourceDescriptorsEXT as *const (),
        841 => vkWriteSamplerDescriptorsEXT as *const (),
        _ => return None,
    };
    Some(erase_function(address))
}
#[inline(never)]
pub(crate) fn instance_terminator_proc_addr(id: u16) -> PFN_vkVoidFunction {
    let address = match id {
        #[cfg(target_os = "android")]
        360 => terminator_vkCreateAndroidSurfaceKHR as *const (),
        #[cfg(feature = "wsi-directfb")]
        380 => terminator_vkCreateDirectFBSurfaceEXT as *const (),
        382 => terminator_vkCreateDisplayPlaneSurfaceKHR as *const (),
        390 => terminator_vkCreateHeadlessSurfaceEXT as *const (),
        #[cfg(target_os = "ios")]
        391 => terminator_vkCreateIOSSurfaceMVK as *const (),
        #[cfg(target_os = "fuchsia")]
        393 => terminator_vkCreateImagePipeSurfaceFUCHSIA as *const (),
        #[cfg(target_os = "macos")]
        399 => terminator_vkCreateMacOSSurfaceMVK as *const (),
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        400 => terminator_vkCreateMetalSurfaceEXT as *const (),
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        417 => terminator_vkCreateScreenSurfaceQNX as *const (),
        #[cfg(feature = "platform-ggp")]
        423 => terminator_vkCreateStreamDescriptorSurfaceGGP as *const (),
        #[cfg(target_env = "ohos")]
        424 => terminator_vkCreateSurfaceOHOS as *const (),
        #[cfg(feature = "platform-ubm")]
        428 => terminator_vkCreateUbmSurfaceSEC as *const (),
        #[cfg(feature = "platform-vi")]
        430 => terminator_vkCreateViSurfaceNN as *const (),
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
        433 => terminator_vkCreateWaylandSurfaceKHR as *const (),
        #[cfg(target_os = "windows")]
        434 => terminator_vkCreateWin32SurfaceKHR as *const (),
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
        435 => terminator_vkCreateXcbSurfaceKHR as *const (),
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
        436 => terminator_vkCreateXlibSurfaceKHR as *const (),
        488 => terminator_vkDestroySurfaceKHR as *const (),
        _ => return None,
    };
    Some(erase_function(address))
}
#[inline(never)]
#[allow(clippy::too_many_lines)]
pub(crate) fn physical_device_terminator_proc_addr(id: u16) -> PFN_vkVoidFunction {
    let address = match id {
        0 => terminator_vkAcquireDrmDisplayEXT as *const (),
        #[cfg(target_os = "windows")]
        6 => terminator_vkAcquireWinrtDisplayNV as *const (),
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
        7 => terminator_vkAcquireXlibDisplayEXT as *const (),
        381 => terminator_vkCreateDisplayModeKHR as *const (),
        505 => {
            terminator_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM
                as *const ()
        }
        506 => {
            terminator_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR as *const ()
        }
        507 => terminator_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM as *const (),
        574 => terminator_vkGetDisplayModeProperties2KHR as *const (),
        575 => terminator_vkGetDisplayModePropertiesKHR as *const (),
        576 => terminator_vkGetDisplayPlaneCapabilities2KHR as *const (),
        577 => terminator_vkGetDisplayPlaneCapabilitiesKHR as *const (),
        578 => terminator_vkGetDisplayPlaneSupportedDisplaysKHR as *const (),
        579 => terminator_vkGetDrmDisplayEXT as *const (),
        634 => terminator_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT as *const (),
        635 => terminator_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR as *const (),
        636 => {
            terminator_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV
                as *const ()
        }
        637 => terminator_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT as *const (),
        638 => terminator_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR as *const (),
        639 => terminator_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV as *const (),
        640 => terminator_vkGetPhysicalDeviceCooperativeVectorPropertiesNV as *const (),
        641 => terminator_vkGetPhysicalDeviceDescriptorSizeEXT as *const (),
        #[cfg(feature = "wsi-directfb")]
        642 => terminator_vkGetPhysicalDeviceDirectFBPresentationSupportEXT as *const (),
        643 => terminator_vkGetPhysicalDeviceDisplayPlaneProperties2KHR as *const (),
        644 => terminator_vkGetPhysicalDeviceDisplayPlanePropertiesKHR as *const (),
        645 => terminator_vkGetPhysicalDeviceDisplayProperties2KHR as *const (),
        646 => terminator_vkGetPhysicalDeviceDisplayPropertiesKHR as *const (),
        647 => terminator_vkGetPhysicalDeviceExternalBufferProperties as *const (),
        648 => terminator_vkGetPhysicalDeviceExternalBufferPropertiesKHR as *const (),
        649 => terminator_vkGetPhysicalDeviceExternalFenceProperties as *const (),
        650 => terminator_vkGetPhysicalDeviceExternalFencePropertiesKHR as *const (),
        651 => terminator_vkGetPhysicalDeviceExternalImageFormatPropertiesNV as *const (),
        652 => terminator_vkGetPhysicalDeviceExternalSemaphoreProperties as *const (),
        653 => terminator_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR as *const (),
        654 => terminator_vkGetPhysicalDeviceExternalTensorPropertiesARM as *const (),
        655 => terminator_vkGetPhysicalDeviceFeatures as *const (),
        656 => terminator_vkGetPhysicalDeviceFeatures2 as *const (),
        657 => terminator_vkGetPhysicalDeviceFeatures2KHR as *const (),
        658 => terminator_vkGetPhysicalDeviceFormatProperties as *const (),
        659 => terminator_vkGetPhysicalDeviceFormatProperties2 as *const (),
        660 => terminator_vkGetPhysicalDeviceFormatProperties2KHR as *const (),
        661 => terminator_vkGetPhysicalDeviceFragmentShadingRatesKHR as *const (),
        662 => terminator_vkGetPhysicalDeviceImageFormatProperties as *const (),
        663 => terminator_vkGetPhysicalDeviceImageFormatProperties2 as *const (),
        664 => terminator_vkGetPhysicalDeviceImageFormatProperties2KHR as *const (),
        665 => terminator_vkGetPhysicalDeviceMemoryProperties as *const (),
        666 => terminator_vkGetPhysicalDeviceMemoryProperties2 as *const (),
        667 => terminator_vkGetPhysicalDeviceMemoryProperties2KHR as *const (),
        668 => terminator_vkGetPhysicalDeviceMultisamplePropertiesEXT as *const (),
        669 => terminator_vkGetPhysicalDeviceOpticalFlowImageFormatsNV as *const (),
        670 => terminator_vkGetPhysicalDevicePresentRectanglesKHR as *const (),
        671 => terminator_vkGetPhysicalDeviceProperties as *const (),
        672 => terminator_vkGetPhysicalDeviceProperties2 as *const (),
        673 => terminator_vkGetPhysicalDeviceProperties2KHR as *const (),
        674 => {
            terminator_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM
                as *const ()
        }
        675 => {
            terminator_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM
                as *const ()
        }
        676 => {
            terminator_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM
                as *const ()
        }
        677 => terminator_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM as *const (),
        678 => terminator_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR as *const (),
        679 => terminator_vkGetPhysicalDeviceQueueFamilyProperties as *const (),
        680 => terminator_vkGetPhysicalDeviceQueueFamilyProperties2 as *const (),
        681 => terminator_vkGetPhysicalDeviceQueueFamilyProperties2KHR as *const (),
        #[cfg(any(target_os = "nto", target_os = "qnx"))]
        682 => terminator_vkGetPhysicalDeviceScreenPresentationSupportQNX as *const (),
        683 => terminator_vkGetPhysicalDeviceSparseImageFormatProperties as *const (),
        684 => terminator_vkGetPhysicalDeviceSparseImageFormatProperties2 as *const (),
        685 => terminator_vkGetPhysicalDeviceSparseImageFormatProperties2KHR as *const (),
        686 => {
            terminator_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV
                as *const ()
        }
        687 => terminator_vkGetPhysicalDeviceSurfaceCapabilities2EXT as *const (),
        688 => terminator_vkGetPhysicalDeviceSurfaceCapabilities2KHR as *const (),
        689 => terminator_vkGetPhysicalDeviceSurfaceCapabilitiesKHR as *const (),
        690 => terminator_vkGetPhysicalDeviceSurfaceFormats2KHR as *const (),
        691 => terminator_vkGetPhysicalDeviceSurfaceFormatsKHR as *const (),
        #[cfg(target_os = "windows")]
        692 => terminator_vkGetPhysicalDeviceSurfacePresentModes2EXT as *const (),
        693 => terminator_vkGetPhysicalDeviceSurfacePresentModesKHR as *const (),
        694 => terminator_vkGetPhysicalDeviceSurfaceSupportKHR as *const (),
        695 => terminator_vkGetPhysicalDeviceToolProperties as *const (),
        696 => terminator_vkGetPhysicalDeviceToolPropertiesEXT as *const (),
        #[cfg(feature = "platform-ubm")]
        697 => terminator_vkGetPhysicalDeviceUbmPresentationSupportSEC as *const (),
        698 => terminator_vkGetPhysicalDeviceVideoCapabilitiesKHR as *const (),
        699 => terminator_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR as *const (),
        700 => terminator_vkGetPhysicalDeviceVideoFormatPropertiesKHR as *const (),
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
        701 => terminator_vkGetPhysicalDeviceWaylandPresentationSupportKHR as *const (),
        #[cfg(target_os = "windows")]
        702 => terminator_vkGetPhysicalDeviceWin32PresentationSupportKHR as *const (),
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
        703 => terminator_vkGetPhysicalDeviceXcbPresentationSupportKHR as *const (),
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
        704 => terminator_vkGetPhysicalDeviceXlibPresentationSupportKHR as *const (),
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
        719 => terminator_vkGetRandROutputDisplayEXT as *const (),
        #[cfg(target_os = "windows")]
        752 => terminator_vkGetWinrtDisplayNV as *const (),
        784 => terminator_vkReleaseDisplayEXT as *const (),
        _ => return None,
    };
    Some(erase_function(address))
}
#[inline(never)]
pub(crate) fn icd_device_terminator_proc_addr(
    table: &IcdDeviceTerminatorDispatchTable,
    id: u16,
) -> PFN_vkVoidFunction {
    let address = match id {
        459 => table.vkDestroyDevice? as *const (),
        425 => table.vkCreateSwapchainKHR? as *const (),
        558 => table.vkGetDeviceGroupSurfacePresentModesKHR? as *const (),
        422 => table.vkCreateSharedSwapchainsKHR? as *const (),
        438 => table.vkDebugMarkerSetObjectTagEXT? as *const (),
        437 => table.vkDebugMarkerSetObjectNameEXT? as *const (),
        800 => table.vkSetDebugUtilsObjectNameEXT? as *const (),
        801 => table.vkSetDebugUtilsObjectTagEXT? as *const (),
        767 => table.vkQueueBeginDebugUtilsLabelEXT? as *const (),
        769 => table.vkQueueEndDebugUtilsLabelEXT? as *const (),
        770 => table.vkQueueInsertDebugUtilsLabelEXT? as *const (),
        30 => table.vkCmdBeginDebugUtilsLabelEXT? as *const (),
        163 => table.vkCmdEndDebugUtilsLabelEXT? as *const (),
        186 => table.vkCmdInsertDebugUtilsLabelEXT? as *const (),
        #[cfg(target_os = "windows")]
        557 => table.vkGetDeviceGroupSurfacePresentModes2EXT? as *const (),
        _ => return None,
    };
    Some(erase_function(address))
}
pub(crate) fn layer_instance_special_proc_addr(id: u16) -> PFN_vkVoidFunction {
    let address = match id {
        398 => crate::layer::create_instance_terminator as *const (),
        612 => crate::layer::terminator_get_instance_proc_addr as *const (),
        379 => crate::create_device_terminator as *const (),
        470 => crate::destroy_instance_terminator as *const (),
        508 => crate::terminator_enumerate_physical_devices as *const (),
        503 => crate::terminator_enumerate_physical_device_groups as *const (),
        504 => crate::terminator_enumerate_physical_device_groups_khr as *const (),
        499 => crate::layer::terminator_enumerate_device_layer_properties as *const (),
        498 => crate::layer::terminator_enumerate_device_extension_properties as *const (),
        373 => crate::debug::messenger::terminator_create_debug_utils_messenger as *const (),
        372 => crate::debug::messenger::terminator_create_debug_report_callback as *const (),
        453 => crate::debug::messenger::terminator_destroy_debug_utils_messenger as *const (),
        452 => crate::debug::messenger::terminator_destroy_debug_report_callback as *const (),
        817 => crate::debug::messenger::terminator_submit_debug_utils_message as *const (),
        439 => crate::debug::messenger::terminator_debug_report_message as *const (),
        _ => return None,
    };
    Some(erase_function(address))
}
pub(crate) fn layer_device_special_proc_addr(id: u16) -> PFN_vkVoidFunction {
    let address = match id {
        569 => crate::layer::terminator_get_device_proc_addr as *const (),
        459 => crate::destroy_device_terminator as *const (),
        425 => crate::surface::terminator_create_swapchain as *const (),
        422 => crate::surface::terminator_create_shared_swapchains as *const (),
        558 => crate::surface::terminator_get_device_group_surface_present_modes as *const (),
        437 => crate::debug::terminator_vkDebugMarkerSetObjectNameEXT as *const (),
        438 => crate::debug::terminator_vkDebugMarkerSetObjectTagEXT as *const (),
        800 => crate::debug::terminator_vkSetDebugUtilsObjectNameEXT as *const (),
        801 => crate::debug::terminator_vkSetDebugUtilsObjectTagEXT as *const (),
        _ => return None,
    };
    Some(erase_function(address))
}
