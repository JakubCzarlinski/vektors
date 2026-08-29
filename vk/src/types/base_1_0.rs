use crate::consts::VK_MAX_DESCRIPTION_SIZE;
use crate::consts::VK_MAX_EXTENSION_NAME_SIZE;
use crate::consts::VK_MAX_MEMORY_HEAPS;
use crate::consts::VK_MAX_MEMORY_TYPES;
use crate::consts::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE;
use crate::consts::VK_UUID_SIZE;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkAccessFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkBufferCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkBufferUsageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkCommandBufferLevel;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkCommandBufferResetFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkCommandBufferUsageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkCommandPoolCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkCommandPoolResetFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkComponentSwizzle;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkDependencyFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkDeviceQueueCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFenceCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormatFeatureFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageAspectFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageLayout;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageTiling;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageUsageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageViewCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageViewType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkInstanceCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkInternalAllocationType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkMemoryHeapFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkMemoryMapFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkMemoryPropertyFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkPhysicalDeviceType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkPipelineStageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueryControlFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueryPipelineStatisticFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueryPoolCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueryResultFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueryType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueueFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSampleCountFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkShaderStageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSharingMode;
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
use crate::enums::VkSparseImageFormatFlagBits;
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
use crate::enums::VkSparseMemoryBindFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSystemAllocationScope;
#[cfg(feature = "VK_SEC_amigo_profiling")]
use crate::types::VkAmigoProfilingSubmitInfoSEC;
#[cfg(feature = "VK_EXT_application_parameters")]
use crate::types::VkApplicationParametersEXT;
#[cfg(any(
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_KHR_dynamic_rendering"
  )
))]
use crate::types::VkAttachmentSampleCountInfoAMD;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkBufferCollectionBufferCreateInfoFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkBufferCollectionImageCreateInfoFUCHSIA;
#[cfg(feature = "VK_EXT_buffer_device_address")]
use crate::types::VkBufferDeviceAddressCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkBufferOpaqueCaptureAddressCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkBufferUsageFlags2CreateInfo;
#[cfg(feature = "VK_EXT_conditional_rendering")]
use crate::types::VkCommandBufferInheritanceConditionalRenderingInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkCommandBufferInheritanceDescriptorHeapInfoEXT;
#[cfg(feature = "VK_QCOM_render_pass_transform")]
use crate::types::VkCommandBufferInheritanceRenderPassTransformInfoQCOM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkCommandBufferInheritanceRenderingInfo;
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
use crate::types::VkCommandBufferInheritanceViewportScissorInfoNV;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkCommandPoolMemoryReservationCreateInfo;
#[cfg(any(
  all(
    feature = "VK_EXT_custom_resolve",
    feature = "VK_KHR_dynamic_rendering"
  ),
  all(feature = "VK_EXT_custom_resolve", feature = "VK_VERSION_1_3")
))]
use crate::types::VkCustomResolveCreateInfoEXT;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
use crate::types::VkD3D12FenceSubmitInfoKHR;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphOpticalFlowImageFormatInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphProcessingEngineCreateInfoARM;
#[cfg(feature = "VK_EXT_debug_report")]
use crate::types::VkDebugReportCallbackCreateInfoEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsMessengerCreateInfoEXT;
#[cfg(feature = "VK_NV_dedicated_allocation")]
use crate::types::VkDedicatedAllocationBufferCreateInfoNV;
#[cfg(feature = "VK_NV_dedicated_allocation")]
use crate::types::VkDedicatedAllocationImageCreateInfoNV;
#[cfg(feature = "VK_NV_dedicated_allocation")]
use crate::types::VkDedicatedAllocationMemoryAllocateInfoNV;
#[cfg(feature = "VK_EXT_device_memory_report")]
use crate::types::VkDeviceDeviceMemoryReportCreateInfoEXT;
#[cfg(feature = "VK_NV_device_diagnostics_config")]
use crate::types::VkDeviceDiagnosticsConfigCreateInfoNV;
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
use crate::types::VkDeviceGroupBindSparseInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkDeviceGroupCommandBufferBeginInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkDeviceGroupDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkDeviceGroupSubmitInfo;
#[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
use crate::types::VkDeviceMemoryOverallocationCreateInfoAMD;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkDeviceObjectReservationCreateInfo;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkDevicePipelineBinaryInternalCacheControlKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkDevicePrivateDataCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkDeviceQueueGlobalPriorityCreateInfo;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::types::VkDeviceQueueShaderCoreControlCreateInfoARM;
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_NV_external_sci_sync2"))]
use crate::types::VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV;
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
use crate::types::VkDirectDriverLoadingListLUNARG;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExportFenceCreateInfo;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::types::VkExportFenceSciSyncInfoNV;
#[cfg(feature = "VK_KHR_external_fence_win32")]
use crate::types::VkExportFenceWin32HandleInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExportMemoryAllocateInfo;
#[cfg(feature = "VK_NV_external_memory")]
use crate::types::VkExportMemoryAllocateInfoNV;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::VkExportMemorySciBufInfoNV;
#[cfg(feature = "VK_KHR_external_memory_win32")]
use crate::types::VkExportMemoryWin32HandleInfoKHR;
#[cfg(feature = "VK_NV_external_memory_win32")]
use crate::types::VkExportMemoryWin32HandleInfoNV;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkExportMetalObjectCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExportSemaphoreCreateInfo;
#[cfg(feature = "VK_NV_external_sci_sync")]
use crate::types::VkExportSemaphoreSciSyncInfoNV;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
use crate::types::VkExportSemaphoreWin32HandleInfoKHR;
#[cfg(feature = "VK_NV_external_compute_queue")]
use crate::types::VkExternalComputeQueueDeviceCreateInfoNV;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkExternalFormatANDROID;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkExternalFormatOHOS;
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
use crate::types::VkExternalFormatQNX;
#[cfg(feature = "VK_EXT_external_memory_acquire_unmodified")]
use crate::types::VkExternalMemoryAcquireUnmodifiedEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalMemoryBufferCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalMemoryImageCreateInfo;
#[cfg(feature = "VK_NV_external_memory")]
use crate::types::VkExternalMemoryImageCreateInfoNV;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkFaultCallbackInfo;
#[cfg(feature = "VK_EXT_frame_boundary")]
use crate::types::VkFrameBoundaryEXT;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"))]
use crate::types::VkFrameBoundaryTensorsARM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkFramebuffer;
#[cfg(feature = "VK_MESA_image_alignment_control")]
use crate::types::VkImageAlignmentControlCreateInfoMESA;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::types::VkImageCompressionControlEXT;
#[cfg(feature = "VK_KHR_extended_flags")]
use crate::types::VkImageCreateFlags2CreateInfoKHR;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::types::VkImageDrmFormatModifierExplicitCreateInfoEXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::types::VkImageDrmFormatModifierListCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkImageFormatListCreateInfo;
#[cfg(any(
  all(feature = "VK_KHR_extended_flags", feature = "VK_VERSION_1_2"),
  all(
    feature = "VK_EXT_separate_stencil_usage",
    feature = "VK_KHR_extended_flags"
  )
))]
use crate::types::VkImageStencilUsage2CreateInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkImageStencilUsageCreateInfo;
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain")
))]
use crate::types::VkImageSwapchainCreateInfoKHR;
#[cfg(feature = "VK_EXT_image_tiling_control")]
use crate::types::VkImageTilingControlCreateInfoEXT;
#[cfg(feature = "VK_KHR_extended_flags")]
use crate::types::VkImageUsageFlags2CreateInfoKHR;
#[cfg(feature = "VK_EXT_astc_decode_mode")]
use crate::types::VkImageViewASTCDecodeModeEXT;
#[cfg(feature = "VK_EXT_image_view_min_lod")]
use crate::types::VkImageViewMinLodCreateInfoEXT;
#[cfg(feature = "VK_QCOM_image_processing")]
use crate::types::VkImageViewSampleWeightCreateInfoQCOM;
#[cfg(feature = "VK_EXT_image_sliced_view_of_3d")]
use crate::types::VkImageViewSlicedCreateInfoEXT;
#[cfg(feature = "VK_KHR_extended_flags")]
use crate::types::VkImageViewUsage2CreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImageViewUsageCreateInfo;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkImportAndroidHardwareBufferInfoANDROID;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkImportMemoryBufferCollectionFUCHSIA;
#[cfg(feature = "VK_KHR_external_memory_fd")]
use crate::types::VkImportMemoryFdInfoKHR;
#[cfg(feature = "VK_EXT_external_memory_host")]
use crate::types::VkImportMemoryHostPointerInfoEXT;
#[cfg(feature = "VK_EXT_external_memory_metal")]
use crate::types::VkImportMemoryMetalHandleInfoEXT;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::VkImportMemorySciBufInfoNV;
#[cfg(feature = "VK_KHR_external_memory_win32")]
use crate::types::VkImportMemoryWin32HandleInfoKHR;
#[cfg(feature = "VK_NV_external_memory_win32")]
use crate::types::VkImportMemoryWin32HandleInfoNV;
#[cfg(feature = "VK_FUCHSIA_external_memory")]
use crate::types::VkImportMemoryZirconHandleInfoFUCHSIA;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkImportMetalBufferInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkImportMetalIOSurfaceInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkImportMetalSharedEventInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkImportMetalTextureInfoEXT;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkImportNativeBufferInfoOHOS;
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
use crate::types::VkImportScreenBufferInfoQNX;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::types::VkLatencySubmissionPresentIdNV;
#[cfg(feature = "VK_EXT_layer_settings")]
use crate::types::VkLayerSettingsCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkMemoryAllocateFlagsInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkMemoryDedicatedAllocateInfo;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkMemoryDedicatedAllocateInfoTensorARM;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkMemoryOpaqueCaptureAddressAllocateInfo;
#[cfg(feature = "VK_EXT_memory_priority")]
use crate::types::VkMemoryPriorityAllocateInfoEXT;
#[cfg(any(
  all(
    feature = "VK_NVX_multiview_per_view_attributes",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NVX_multiview_per_view_attributes"
  )
))]
use crate::types::VkMultiviewPerViewAttributesInfoNVX;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkOpaqueCaptureDataCreateInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkOpaqueCaptureDescriptorDataCreateInfoEXT;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowImageFormatInfoNV;
use crate::types::VkPNextExtends;
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_KHR_performance_query"))]
use crate::types::VkPerformanceQueryReservationInfoKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPerformanceQuerySubmitInfoKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDevice8BitStorageFeatures;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDevice16BitStorageFeatures;
#[cfg(feature = "VK_EXT_4444_formats")]
use crate::types::VkPhysicalDevice4444FormatsFeaturesEXT;
#[cfg(feature = "VK_EXT_astc_decode_mode")]
use crate::types::VkPhysicalDeviceASTCDecodeFeaturesEXT;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkPhysicalDeviceAccelerationStructureFeaturesKHR;
#[cfg(feature = "VK_EXT_device_address_binding_report")]
use crate::types::VkPhysicalDeviceAddressBindingReportFeaturesEXT;
#[cfg(feature = "VK_SEC_amigo_profiling")]
use crate::types::VkPhysicalDeviceAmigoProfilingFeaturesSEC;
#[cfg(feature = "VK_AMD_anti_lag")]
use crate::types::VkPhysicalDeviceAntiLagFeaturesAMD;
#[cfg(feature = "VK_EXT_attachment_feedback_loop_dynamic_state")]
use crate::types::VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT;
#[cfg(feature = "VK_EXT_attachment_feedback_loop_layout")]
use crate::types::VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
use crate::types::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT;
#[cfg(feature = "VK_EXT_border_color_swizzle")]
use crate::types::VkPhysicalDeviceBorderColorSwizzleFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceBufferDeviceAddressFeatures;
#[cfg(feature = "VK_EXT_buffer_device_address")]
use crate::types::VkPhysicalDeviceBufferDeviceAddressFeaturesEXT;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkPhysicalDeviceClusterAccelerationStructureFeaturesNV;
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
use crate::types::VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI;
#[cfg(feature = "VK_AMD_device_coherent_memory")]
use crate::types::VkPhysicalDeviceCoherentMemoryFeaturesAMD;
#[cfg(feature = "VK_EXT_color_write_enable")]
use crate::types::VkPhysicalDeviceColorWriteEnableFeaturesEXT;
#[cfg(feature = "VK_NV_command_buffer_inheritance")]
use crate::types::VkPhysicalDeviceCommandBufferInheritanceFeaturesNV;
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
use crate::types::VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV;
#[cfg(feature = "VK_KHR_compute_shader_derivatives")]
use crate::types::VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR;
#[cfg(feature = "VK_EXT_conditional_rendering")]
use crate::types::VkPhysicalDeviceConditionalRenderingFeaturesEXT;
#[cfg(feature = "VK_NV_cooperative_matrix2")]
use crate::types::VkPhysicalDeviceCooperativeMatrix2FeaturesNV;
#[cfg(feature = "VK_QCOM_cooperative_matrix_conversion")]
use crate::types::VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM;
#[cfg(feature = "VK_NV_cooperative_matrix_decode_vector")]
use crate::types::VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV;
#[cfg(feature = "VK_KHR_cooperative_matrix")]
use crate::types::VkPhysicalDeviceCooperativeMatrixFeaturesKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::types::VkPhysicalDeviceCooperativeMatrixFeaturesNV;
#[cfg(feature = "VK_NV_cooperative_vector")]
use crate::types::VkPhysicalDeviceCooperativeVectorFeaturesNV;
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
use crate::types::VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR;
#[cfg(feature = "VK_NV_copy_memory_indirect")]
use crate::types::VkPhysicalDeviceCopyMemoryIndirectFeaturesNV;
#[cfg(feature = "VK_NV_corner_sampled_image")]
use crate::types::VkPhysicalDeviceCornerSampledImageFeaturesNV;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::types::VkPhysicalDeviceCoverageReductionModeFeaturesNV;
#[cfg(feature = "VK_QCOM_filter_cubic_clamp")]
use crate::types::VkPhysicalDeviceCubicClampFeaturesQCOM;
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
use crate::types::VkPhysicalDeviceCubicWeightsFeaturesQCOM;
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
use crate::types::VkPhysicalDeviceCudaKernelLaunchFeaturesNV;
#[cfg(feature = "VK_EXT_custom_border_color")]
use crate::types::VkPhysicalDeviceCustomBorderColorFeaturesEXT;
#[cfg(feature = "VK_EXT_custom_resolve")]
use crate::types::VkPhysicalDeviceCustomResolveFeaturesEXT;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkPhysicalDeviceDataGraphFeaturesARM;
#[cfg(feature = "VK_QCOM_data_graph_model")]
use crate::types::VkPhysicalDeviceDataGraphModelFeaturesQCOM;
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
use crate::types::VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM;
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
use crate::types::VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
use crate::types::VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX;
#[cfg(feature = "VK_EXT_depth_bias_control")]
use crate::types::VkPhysicalDeviceDepthBiasControlFeaturesEXT;
#[cfg(feature = "VK_EXT_depth_clamp_control")]
use crate::types::VkPhysicalDeviceDepthClampControlFeaturesEXT;
#[cfg(feature = "VK_KHR_depth_clamp_zero_one")]
use crate::types::VkPhysicalDeviceDepthClampZeroOneFeaturesKHR;
#[cfg(feature = "VK_EXT_depth_clip_control")]
use crate::types::VkPhysicalDeviceDepthClipControlFeaturesEXT;
#[cfg(feature = "VK_EXT_depth_clip_enable")]
use crate::types::VkPhysicalDeviceDepthClipEnableFeaturesEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkPhysicalDeviceDescriptorBufferFeaturesEXT;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
use crate::types::VkPhysicalDeviceDescriptorBufferTensorFeaturesARM;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkPhysicalDeviceDescriptorHeapFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceDescriptorIndexingFeatures;
#[cfg(feature = "VK_NV_descriptor_pool_overallocation")]
use crate::types::VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV;
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
use crate::types::VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR;
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
use crate::types::VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
#[cfg(feature = "VK_EXT_device_memory_report")]
use crate::types::VkPhysicalDeviceDeviceMemoryReportFeaturesEXT;
#[cfg(feature = "VK_NV_device_diagnostics_config")]
use crate::types::VkPhysicalDeviceDiagnosticsConfigFeaturesNV;
#[cfg(feature = "VK_NV_displacement_micromap")]
use crate::types::VkPhysicalDeviceDisplacementMicromapFeaturesNV;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkPhysicalDeviceDynamicRenderingFeatures;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceDynamicRenderingLocalReadFeatures;
#[cfg(feature = "VK_EXT_dynamic_rendering_unused_attachments")]
use crate::types::VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT;
#[cfg(feature = "VK_QCOM_elapsed_timer_query")]
use crate::types::VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM;
#[cfg(feature = "VK_NV_scissor_exclusive")]
use crate::types::VkPhysicalDeviceExclusiveScissorFeaturesNV;
#[cfg(feature = "VK_EXT_extended_dynamic_state2")]
use crate::types::VkPhysicalDeviceExtendedDynamicState2FeaturesEXT;
#[cfg(feature = "VK_EXT_extended_dynamic_state3")]
use crate::types::VkPhysicalDeviceExtendedDynamicState3FeaturesEXT;
#[cfg(feature = "VK_EXT_extended_dynamic_state")]
use crate::types::VkPhysicalDeviceExtendedDynamicStateFeaturesEXT;
#[cfg(feature = "VK_KHR_extended_flags")]
use crate::types::VkPhysicalDeviceExtendedFlagsFeaturesKHR;
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
use crate::types::VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV;
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
use crate::types::VkPhysicalDeviceExternalFormatResolveFeaturesANDROID;
#[cfg(feature = "VK_NV_external_memory_rdma")]
use crate::types::VkPhysicalDeviceExternalMemoryRDMAFeaturesNV;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::VkPhysicalDeviceExternalMemorySciBufFeaturesNV;
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
use crate::types::VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX;
#[cfg(feature = "VK_NV_external_sci_sync2")]
use crate::types::VkPhysicalDeviceExternalSciSync2FeaturesNV;
#[cfg(feature = "VK_NV_external_sci_sync")]
use crate::types::VkPhysicalDeviceExternalSciSyncFeaturesNV;
#[cfg(feature = "VK_EXT_device_fault")]
use crate::types::VkPhysicalDeviceFaultFeaturesEXT;
#[cfg(feature = "VK_KHR_device_fault")]
use crate::types::VkPhysicalDeviceFaultFeaturesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_ARM_format_pack")]
use crate::types::VkPhysicalDeviceFormatPackFeaturesARM;
#[cfg(feature = "VK_EXT_fragment_density_map2")]
use crate::types::VkPhysicalDeviceFragmentDensityMap2FeaturesEXT;
#[cfg(feature = "VK_EXT_fragment_density_map")]
use crate::types::VkPhysicalDeviceFragmentDensityMapFeaturesEXT;
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
use crate::types::VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE;
#[cfg(feature = "VK_EXT_fragment_density_map_offset")]
use crate::types::VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT;
#[cfg(feature = "VK_KHR_fragment_shader_barycentric")]
use crate::types::VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR;
#[cfg(feature = "VK_EXT_fragment_shader_interlock")]
use crate::types::VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::types::VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::types::VkPhysicalDeviceFragmentShadingRateFeaturesKHR;
#[cfg(feature = "VK_EXT_frame_boundary")]
use crate::types::VkPhysicalDeviceFrameBoundaryFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceGlobalPriorityQueryFeatures;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkPhysicalDeviceGpaFeaturesAMD;
#[cfg(feature = "VK_EXT_graphics_pipeline_library")]
use crate::types::VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT;
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
use crate::types::VkPhysicalDeviceHdrVividFeaturesHUAWEI;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceHostImageCopyFeatures;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceHostQueryResetFeatures;
#[cfg(feature = "VK_EXT_image_2d_view_of_3d")]
use crate::types::VkPhysicalDeviceImage2DViewOf3DFeaturesEXT;
#[cfg(feature = "VK_MESA_image_alignment_control")]
use crate::types::VkPhysicalDeviceImageAlignmentControlFeaturesMESA;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::types::VkPhysicalDeviceImageCompressionControlFeaturesEXT;
#[cfg(feature = "VK_EXT_image_compression_control_swapchain")]
use crate::types::VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT;
#[cfg(feature = "VK_QCOM_image_processing2")]
use crate::types::VkPhysicalDeviceImageProcessing2FeaturesQCOM;
#[cfg(feature = "VK_QCOM_image_processing3")]
use crate::types::VkPhysicalDeviceImageProcessing3FeaturesQCOM;
#[cfg(feature = "VK_QCOM_image_processing")]
use crate::types::VkPhysicalDeviceImageProcessingFeaturesQCOM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceImageRobustnessFeatures;
#[cfg(feature = "VK_EXT_image_sliced_view_of_3d")]
use crate::types::VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT;
#[cfg(feature = "VK_EXT_image_tiling_control")]
use crate::types::VkPhysicalDeviceImageTilingControlFeaturesEXT;
#[cfg(feature = "VK_EXT_image_view_min_lod")]
use crate::types::VkPhysicalDeviceImageViewMinLodFeaturesEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkPhysicalDeviceImagelessFramebufferFeatures;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceIndexTypeUint8Features;
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
use crate::types::VkPhysicalDeviceInheritedViewportScissorFeaturesNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceInlineUniformBlockFeatures;
#[cfg(feature = "VK_KHR_internally_synchronized_queues")]
use crate::types::VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR;
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
use crate::types::VkPhysicalDeviceInvocationMaskFeaturesHUAWEI;
#[cfg(feature = "VK_EXT_legacy_dithering")]
use crate::types::VkPhysicalDeviceLegacyDitheringFeaturesEXT;
#[cfg(feature = "VK_EXT_legacy_vertex_attributes")]
use crate::types::VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceLineRasterizationFeatures;
#[cfg(feature = "VK_NV_linear_color_attachment")]
use crate::types::VkPhysicalDeviceLinearColorAttachmentFeaturesNV;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceMaintenance4Features;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceMaintenance5Features;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceMaintenance6Features;
#[cfg(feature = "VK_KHR_maintenance7")]
use crate::types::VkPhysicalDeviceMaintenance7FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance8")]
use crate::types::VkPhysicalDeviceMaintenance8FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance9")]
use crate::types::VkPhysicalDeviceMaintenance9FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkPhysicalDeviceMaintenance10FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance11")]
use crate::types::VkPhysicalDeviceMaintenance11FeaturesKHR;
#[cfg(feature = "VK_EXT_map_memory_placed")]
use crate::types::VkPhysicalDeviceMapMemoryPlacedFeaturesEXT;
#[cfg(feature = "VK_EXT_memory_decompression")]
use crate::types::VkPhysicalDeviceMemoryDecompressionFeaturesEXT;
#[cfg(feature = "VK_EXT_memory_priority")]
use crate::types::VkPhysicalDeviceMemoryPriorityFeaturesEXT;
#[cfg(feature = "VK_EXT_mesh_shader")]
use crate::types::VkPhysicalDeviceMeshShaderFeaturesEXT;
#[cfg(feature = "VK_NV_mesh_shader")]
use crate::types::VkPhysicalDeviceMeshShaderFeaturesNV;
#[cfg(feature = "VK_EXT_multi_draw")]
use crate::types::VkPhysicalDeviceMultiDrawFeaturesEXT;
#[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
use crate::types::VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT;
#[cfg(feature = "VK_EXT_multisampled_render_to_swapchain")]
use crate::types::VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPhysicalDeviceMultiviewFeatures;
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
use crate::types::VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM;
#[cfg(feature = "VK_QCOM_multiview_per_view_viewports")]
use crate::types::VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM;
#[cfg(feature = "VK_EXT_mutable_descriptor_type")]
use crate::types::VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT;
#[cfg(feature = "VK_EXT_nested_command_buffer")]
use crate::types::VkPhysicalDeviceNestedCommandBufferFeaturesEXT;
#[cfg(feature = "VK_EXT_non_seamless_cube_map")]
use crate::types::VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkPhysicalDeviceOpacityMicromapFeaturesEXT;
#[cfg(feature = "VK_KHR_opacity_micromap")]
use crate::types::VkPhysicalDeviceOpacityMicromapFeaturesKHR;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkPhysicalDeviceOpticalFlowFeaturesNV;
#[cfg(feature = "VK_EXT_pageable_device_local_memory")]
use crate::types::VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV;
#[cfg(feature = "VK_NV_per_stage_descriptor_set")]
use crate::types::VkPhysicalDevicePerStageDescriptorSetFeaturesNV;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkPhysicalDevicePerformanceCountersByRegionFeaturesARM;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPhysicalDevicePerformanceQueryFeaturesKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPhysicalDevicePipelineBinaryFeaturesKHR;
#[cfg(feature = "VK_SEC_pipeline_cache_incremental_mode")]
use crate::types::VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDevicePipelineCreationCacheControlFeatures;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::types::VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
#[cfg(feature = "VK_EXT_pipeline_library_group_handles")]
use crate::types::VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT;
#[cfg(feature = "VK_ARM_pipeline_opacity_micromap")]
use crate::types::VkPhysicalDevicePipelineOpacityMicromapFeaturesARM;
#[cfg(feature = "VK_EXT_pipeline_properties")]
use crate::types::VkPhysicalDevicePipelinePropertiesFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDevicePipelineProtectedAccessFeatures;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDevicePipelineRobustnessFeatures;
#[cfg(feature = "VK_KHR_portability_subset")]
use crate::types::VkPhysicalDevicePortabilitySubsetFeaturesKHR;
#[cfg(feature = "VK_NV_present_barrier")]
use crate::types::VkPhysicalDevicePresentBarrierFeaturesNV;
#[cfg(feature = "VK_KHR_present_id2")]
use crate::types::VkPhysicalDevicePresentId2FeaturesKHR;
#[cfg(feature = "VK_KHR_present_id")]
use crate::types::VkPhysicalDevicePresentIdFeaturesKHR;
#[cfg(feature = "VK_NV_present_metering")]
use crate::types::VkPhysicalDevicePresentMeteringFeaturesNV;
#[cfg(feature = "VK_KHR_present_mode_fifo_latest_ready")]
use crate::types::VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPhysicalDevicePresentTimingFeaturesEXT;
#[cfg(feature = "VK_KHR_present_wait2")]
use crate::types::VkPhysicalDevicePresentWait2FeaturesKHR;
#[cfg(feature = "VK_KHR_present_wait")]
use crate::types::VkPhysicalDevicePresentWaitFeaturesKHR;
#[cfg(feature = "VK_EXT_primitive_restart_index")]
use crate::types::VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT;
#[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
use crate::types::VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
#[cfg(feature = "VK_EXT_primitives_generated_query")]
use crate::types::VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDevicePrivateDataFeatures;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProtectedMemoryFeatures;
#[cfg(feature = "VK_EXT_provoking_vertex")]
use crate::types::VkPhysicalDeviceProvokingVertexFeaturesEXT;
#[cfg(feature = "VK_NV_push_constant_bank")]
use crate::types::VkPhysicalDevicePushConstantBankFeaturesNV;
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
use crate::types::VkPhysicalDeviceQueuePerfHintFeaturesQCOM;
#[cfg(feature = "VK_EXT_rgba10x6_formats")]
use crate::types::VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT;
#[cfg(feature = "VK_EXT_rasterization_order_attachment_access")]
use crate::types::VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT;
#[cfg(feature = "VK_NV_raw_access_chains")]
use crate::types::VkPhysicalDeviceRawAccessChainsFeaturesNV;
#[cfg(feature = "VK_KHR_ray_query")]
use crate::types::VkPhysicalDeviceRayQueryFeaturesKHR;
#[cfg(feature = "VK_EXT_ray_tracing_invocation_reorder")]
use crate::types::VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT;
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
use crate::types::VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV;
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
use crate::types::VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV;
#[cfg(feature = "VK_KHR_ray_tracing_maintenance1")]
use crate::types::VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkPhysicalDeviceRayTracingMotionBlurFeaturesNV;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkPhysicalDeviceRayTracingPipelineFeaturesKHR;
#[cfg(feature = "VK_KHR_ray_tracing_position_fetch")]
use crate::types::VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR;
#[cfg(feature = "VK_NV_ray_tracing_validation")]
use crate::types::VkPhysicalDeviceRayTracingValidationFeaturesNV;
#[cfg(feature = "VK_IMG_relaxed_line_rasterization")]
use crate::types::VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG;
#[cfg(feature = "VK_ARM_render_pass_striped")]
use crate::types::VkPhysicalDeviceRenderPassStripedFeaturesARM;
#[cfg(feature = "VK_NV_representative_fragment_test")]
use crate::types::VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV;
#[cfg(feature = "VK_KHR_robustness2")]
use crate::types::VkPhysicalDeviceRobustness2FeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceSamplerYcbcrConversionFeatures;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceScalarBlockLayoutFeatures;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::types::VkPhysicalDeviceSchedulingControlsFeaturesARM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures;
#[cfg(feature = "VK_EXT_shader_64bit_indexing")]
use crate::types::VkPhysicalDeviceShader64BitIndexingFeaturesEXT;
#[cfg(feature = "VK_KHR_shader_abort")]
use crate::types::VkPhysicalDeviceShaderAbortFeaturesKHR;
#[cfg(feature = "VK_EXT_shader_atomic_float2")]
use crate::types::VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT;
#[cfg(feature = "VK_NV_shader_atomic_float16_vector")]
use crate::types::VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV;
#[cfg(feature = "VK_EXT_shader_atomic_float")]
use crate::types::VkPhysicalDeviceShaderAtomicFloatFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceShaderAtomicInt64Features;
#[cfg(feature = "VK_KHR_shader_bfloat16")]
use crate::types::VkPhysicalDeviceShaderBfloat16FeaturesKHR;
#[cfg(feature = "VK_KHR_shader_clock")]
use crate::types::VkPhysicalDeviceShaderClockFeaturesKHR;
#[cfg(feature = "VK_KHR_shader_constant_data")]
use crate::types::VkPhysicalDeviceShaderConstantDataFeaturesKHR;
#[cfg(feature = "VK_ARM_shader_core_builtins")]
use crate::types::VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPhysicalDeviceShaderDrawParametersFeatures;
#[cfg(feature = "VK_AMD_shader_early_and_late_fragment_tests")]
use crate::types::VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkPhysicalDeviceShaderEnqueueFeaturesAMDX;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceShaderExpectAssumeFeatures;
#[cfg(feature = "VK_EXT_shader_float8")]
use crate::types::VkPhysicalDeviceShaderFloat8FeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceShaderFloat16Int8Features;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceShaderFloatControls2Features;
#[cfg(feature = "VK_KHR_shader_fma")]
use crate::types::VkPhysicalDeviceShaderFmaFeaturesKHR;
#[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
use crate::types::VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
#[cfg(feature = "VK_NV_shader_image_footprint")]
use crate::types::VkPhysicalDeviceShaderImageFootprintFeaturesNV;
#[cfg(feature = "VK_ARM_shader_instrumentation")]
use crate::types::VkPhysicalDeviceShaderInstrumentationFeaturesARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceShaderIntegerDotProductFeatures;
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
use crate::types::VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
#[cfg(feature = "VK_EXT_shader_long_vector")]
use crate::types::VkPhysicalDeviceShaderLongVectorFeaturesEXT;
#[cfg(feature = "VK_KHR_shader_maximal_reconvergence")]
use crate::types::VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR;
#[cfg(feature = "VK_VALVE_shader_mixed_float_dot_product")]
use crate::types::VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE;
#[cfg(feature = "VK_EXT_shader_module_identifier")]
use crate::types::VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT;
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
use crate::types::VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM;
#[cfg(feature = "VK_EXT_shader_ocp_microscaling_types")]
use crate::types::VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_object")]
use crate::types::VkPhysicalDeviceShaderObjectFeaturesEXT;
#[cfg(feature = "VK_KHR_shader_quad_control")]
use crate::types::VkPhysicalDeviceShaderQuadControlFeaturesKHR;
#[cfg(feature = "VK_KHR_shader_relaxed_extended_instruction")]
use crate::types::VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR;
#[cfg(feature = "VK_EXT_shader_replicated_composites")]
use crate::types::VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT;
#[cfg(feature = "VK_NV_shader_sm_builtins")]
use crate::types::VkPhysicalDeviceShaderSMBuiltinsFeaturesNV;
#[cfg(feature = "VK_EXT_shader_split_barrier")]
use crate::types::VkPhysicalDeviceShaderSplitBarrierFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures;
#[cfg(feature = "VK_EXT_shader_subgroup_partitioned")]
use crate::types::VkPhysicalDeviceShaderSubgroupPartitionedFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceShaderSubgroupRotateFeatures;
#[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
use crate::types::VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceShaderTerminateInvocationFeatures;
#[cfg(feature = "VK_EXT_shader_tile_image")]
use crate::types::VkPhysicalDeviceShaderTileImageFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_uniform_buffer_unsized_array")]
use crate::types::VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT;
#[cfg(feature = "VK_KHR_shader_untyped_pointers")]
use crate::types::VkPhysicalDeviceShaderUntypedPointersFeaturesKHR;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkPhysicalDeviceShadingRateImageFeaturesNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceSubgroupSizeControlFeatures;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::types::VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT;
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
use crate::types::VkPhysicalDeviceSubpassShadingFeaturesHUAWEI;
#[cfg(feature = "VK_KHR_swapchain_maintenance1")]
use crate::types::VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceSynchronization2Features;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkPhysicalDeviceTensorFeaturesARM;
#[cfg(feature = "VK_EXT_texel_buffer_alignment")]
use crate::types::VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT;
#[cfg(feature = "VK_EXT_texture_compression_astc_3d")]
use crate::types::VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceTextureCompressionASTCHDRFeatures;
#[cfg(feature = "VK_SEC_throttle_hint")]
use crate::types::VkPhysicalDeviceThrottleHintFeaturesSEC;
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
use crate::types::VkPhysicalDeviceTileMemoryHeapFeaturesQCOM;
#[cfg(feature = "VK_QCOM_tile_properties")]
use crate::types::VkPhysicalDeviceTilePropertiesFeaturesQCOM;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkPhysicalDeviceTileShadingFeaturesQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceTimelineSemaphoreFeatures;
#[cfg(feature = "VK_EXT_transform_feedback")]
use crate::types::VkPhysicalDeviceTransformFeedbackFeaturesEXT;
#[cfg(feature = "VK_KHR_unified_image_layouts")]
use crate::types::VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceUniformBufferStandardLayoutFeatures;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceVariablePointersFeatures;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceVertexAttributeDivisorFeatures;
#[cfg(feature = "VK_EXT_vertex_attribute_robustness")]
use crate::types::VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT;
#[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
use crate::types::VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT;
#[cfg(feature = "VK_KHR_video_decode_vp9")]
use crate::types::VkPhysicalDeviceVideoDecodeVP9FeaturesKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkPhysicalDeviceVideoEncodeAV1FeaturesKHR;
#[cfg(feature = "VK_KHR_video_encode_feedback2")]
use crate::types::VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR;
#[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
use crate::types::VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR;
#[cfg(feature = "VK_KHR_video_encode_quantization_map")]
use crate::types::VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::types::VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE;
#[cfg(feature = "VK_KHR_video_maintenance1")]
use crate::types::VkPhysicalDeviceVideoMaintenance1FeaturesKHR;
#[cfg(feature = "VK_KHR_video_maintenance2")]
use crate::types::VkPhysicalDeviceVideoMaintenance2FeaturesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceVulkan11Features;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceVulkan12Features;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceVulkan13Features;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceVulkan14Features;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceVulkanMemoryModelFeatures;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPhysicalDeviceVulkanSC10Features;
#[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
use crate::types::VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
#[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
use crate::types::VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
use crate::types::VkPhysicalDeviceYcbcrDegammaFeaturesQCOM;
#[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
use crate::types::VkPhysicalDeviceYcbcrImageArraysFeaturesEXT;
#[cfg(feature = "VK_EXT_zero_initialize_device_memory")]
use crate::types::VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkProtectedSubmitInfo;
#[cfg(feature = "VK_NV_low_latency")]
use crate::types::VkQueryLowLatencySupportNV;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkQueryPoolPerformanceCreateInfoKHR;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::types::VkQueryPoolPerformanceQueryCreateInfoINTEL;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkQueryPoolVideoEncodeFeedbackCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_feedback2")]
use crate::types::VkQueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPass;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkRenderPassTileShadingCreateInfoQCOM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkRenderingAttachmentLocationInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkRenderingInputAttachmentIndexInfo;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkSampleLocationsInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkSamplerYcbcrConversionInfo;
#[cfg(feature = "VK_NV_external_sci_sync2")]
use crate::types::VkSemaphoreSciSyncCreateInfoNV;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkSemaphoreTypeCreateInfo;
#[cfg(feature = "VK_SEC_throttle_hint")]
use crate::types::VkThrottleHintSubmitInfoSEC;
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
use crate::types::VkTileMemoryBindInfoQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkTimelineSemaphoreSubmitInfo;
#[cfg(feature = "VK_EXT_validation_features")]
use crate::types::VkValidationFeaturesEXT;
#[cfg(feature = "VK_EXT_validation_flags")]
use crate::types::VkValidationFlagsEXT;
#[cfg(feature = "VK_KHR_video_decode_av1")]
use crate::types::VkVideoDecodeAV1ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::VkVideoDecodeH264ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_h265")]
use crate::types::VkVideoDecodeH265ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::types::VkVideoDecodeUsageInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_vp9")]
use crate::types::VkVideoDecodeVP9ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeUsageInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoProfileListInfoKHR;
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
use crate::types::VkWin32KeyedMutexAcquireReleaseInfoKHR;
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
use crate::types::VkWin32KeyedMutexAcquireReleaseInfoNV;
use core::ffi::{c_char, c_void};
/// [VkBool32](https://docs.vulkan.org/refpages/latest/refpages/source/VkBool32.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkBool32 = u32;
/// [VkFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkFlags = u32;
/// [VkDeviceSize](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceSize.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkDeviceSize = u64;
/// [VkDeviceAddress](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddress.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkDeviceAddress = u64;
/// [VkQueryPoolCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkQueryPoolCreateFlags = VkQueryPoolCreateFlagBits;
/// [VkInstanceCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkInstanceCreateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkInstanceCreateFlags = VkInstanceCreateFlagBits;
/// [VkDeviceCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceCreateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkDeviceCreateFlags = VkFlags;
/// [VkDeviceQueueCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueCreateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkDeviceQueueCreateFlags = VkDeviceQueueCreateFlagBits;
/// [VkQueueFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkQueueFlags = VkQueueFlagBits;
/// [VkMemoryPropertyFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryPropertyFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkMemoryPropertyFlags = VkMemoryPropertyFlagBits;
/// [VkMemoryHeapFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHeapFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkMemoryHeapFlags = VkMemoryHeapFlagBits;
/// [VkAccessFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkAccessFlags = VkAccessFlagBits;
/// [VkBufferUsageFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkBufferUsageFlags = VkBufferUsageFlagBits;
/// [VkBufferCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCreateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkBufferCreateFlags = VkBufferCreateFlagBits;
/// [VkShaderStageFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderStageFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkShaderStageFlags = VkShaderStageFlagBits;
/// [VkImageUsageFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkImageUsageFlags = VkImageUsageFlagBits;
/// [VkImageCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkImageCreateFlags = VkImageCreateFlagBits;
/// [VkImageViewCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCreateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkImageViewCreateFlags = VkImageViewCreateFlagBits;
/// [VkFenceCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceCreateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkFenceCreateFlags = VkFenceCreateFlagBits;
/// [VkSemaphoreCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreCreateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkSemaphoreCreateFlags = VkFlags;
/// [VkFormatFeatureFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkFormatFeatureFlags = VkFormatFeatureFlagBits;
/// [VkQueryControlFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryControlFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkQueryControlFlags = VkQueryControlFlagBits;
/// [VkQueryResultFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryResultFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkQueryResultFlags = VkQueryResultFlagBits;
/// [VkCommandPoolCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolCreateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkCommandPoolCreateFlags = VkCommandPoolCreateFlagBits;
/// [VkCommandPoolResetFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolResetFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkCommandPoolResetFlags = VkCommandPoolResetFlagBits;
/// [VkCommandBufferResetFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferResetFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkCommandBufferResetFlags = VkCommandBufferResetFlagBits;
/// [VkCommandBufferUsageFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferUsageFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkCommandBufferUsageFlags = VkCommandBufferUsageFlagBits;
/// [VkQueryPipelineStatisticFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPipelineStatisticFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlagBits;
/// [VkMemoryMapFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkMemoryMapFlags = VkMemoryMapFlagBits;
/// [VkImageAspectFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAspectFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkImageAspectFlags = VkImageAspectFlagBits;
/// [VkSparseMemoryBindFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseMemoryBindFlags.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
pub type VkSparseMemoryBindFlags = VkSparseMemoryBindFlagBits;
/// [VkSparseImageFormatFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatFlags.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
pub type VkSparseImageFormatFlags = VkSparseImageFormatFlagBits;
/// [VkPipelineStageFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkPipelineStageFlags = VkPipelineStageFlagBits;
/// [VkSampleCountFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleCountFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkSampleCountFlags = VkSampleCountFlagBits;
/// [VkDependencyFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type VkDependencyFlags = VkDependencyFlagBits;
/// [VkInstance](https://docs.vulkan.org/refpages/latest/refpages/source/VkInstance.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkInstance(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkInstance {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkInstance {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkInstance {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkInstance {}
/// [VkPhysicalDevice](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkPhysicalDevice(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkPhysicalDevice {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkPhysicalDevice {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkPhysicalDevice {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkPhysicalDevice {}
/// [VkDevice](https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDevice(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkDevice {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkDevice {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkDevice {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkDevice {}
/// [VkQueue](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueue.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkQueue(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkQueue {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkQueue {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkQueue {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkQueue {}
/// [VkCommandBuffer](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkCommandBuffer(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkCommandBuffer {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkCommandBuffer {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkCommandBuffer {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkCommandBuffer {}
/// [VkDeviceMemory](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDeviceMemory(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkDeviceMemory {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkDeviceMemory {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkDeviceMemory {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkDeviceMemory {}
/// [VkCommandPool](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPool.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkCommandPool(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkCommandPool {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkCommandPool {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkCommandPool {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkCommandPool {}
/// [VkBuffer](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuffer.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkBuffer(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkBuffer {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkBuffer {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkBuffer {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkBuffer {}
/// [VkImage](https://docs.vulkan.org/refpages/latest/refpages/source/VkImage.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkImage(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkImage {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkImage {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkImage {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkImage {}
/// [VkImageView](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageView.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkImageView(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkImageView {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkImageView {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkImageView {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkImageView {}
/// [VkFence](https://docs.vulkan.org/refpages/latest/refpages/source/VkFence.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkFence(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkFence {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkFence {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkFence {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkFence {}
/// [VkSemaphore](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphore.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkSemaphore(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkSemaphore {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkSemaphore {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkSemaphore {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkSemaphore {}
/// [VkQueryPool](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPool.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkQueryPool(pub *mut c_void);
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkQueryPool {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl Default for VkQueryPool {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkQueryPool {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkQueryPool {}
/// [PFN_vkInternalAllocationNotification](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalAllocationNotification.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkInternalAllocationNotification = Option<
  unsafe extern "system" fn(
    pUserData: *mut c_void,
    size: usize,
    allocationType: VkInternalAllocationType,
    allocationScope: VkSystemAllocationScope,
  ),
>;
/// [PFN_vkInternalFreeNotification](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalFreeNotification.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkInternalFreeNotification = Option<
  unsafe extern "system" fn(
    pUserData: *mut c_void,
    size: usize,
    allocationType: VkInternalAllocationType,
    allocationScope: VkSystemAllocationScope,
  ),
>;
/// [PFN_vkReallocationFunction](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkReallocationFunction.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkReallocationFunction = Option<
  unsafe extern "system" fn(
    pUserData: *mut c_void,
    pOriginal: *mut c_void,
    size: usize,
    alignment: usize,
    allocationScope: VkSystemAllocationScope,
  ) -> *mut c_void,
>;
/// [PFN_vkAllocationFunction](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkAllocationFunction.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkAllocationFunction = Option<
  unsafe extern "system" fn(
    pUserData: *mut c_void,
    size: usize,
    alignment: usize,
    allocationScope: VkSystemAllocationScope,
  ) -> *mut c_void,
>;
/// [PFN_vkFreeFunction](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkFreeFunction.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkFreeFunction =
  Option<unsafe extern "system" fn(pUserData: *mut c_void, pMemory: *mut c_void)>;
/// [PFN_vkVoidFunction](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkVoidFunction.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkVoidFunction = Option<unsafe extern "system" fn()>;
/// [VkBaseOutStructure](https://docs.vulkan.org/refpages/latest/refpages/source/VkBaseOutStructure.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBaseOutStructure<'a> {
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut VkBaseOutStructure<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkBaseOutStructure<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkBaseOutStructure<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkBaseOutStructure<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType(0),
    pNext: core::ptr::null_mut(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *mut VkBaseOutStructure<'a>) -> Self {
    self.pNext = val;
    self
  }
}
/// [VkBaseInStructure](https://docs.vulkan.org/refpages/latest/refpages/source/VkBaseInStructure.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBaseInStructure<'a> {
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const VkBaseInStructure<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkBaseInStructure<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkBaseInStructure<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkBaseInStructure<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType(0),
    pNext: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const VkBaseInStructure<'a>) -> Self {
    self.pNext = val;
    self
  }
}
/// [VkOffset2D](https://docs.vulkan.org/refpages/latest/refpages/source/VkOffset2D.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkOffset2D {
  pub x: i32,
  pub y: i32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkOffset2D {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkOffset2D {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkOffset2D {
  pub const DEFAULT: Self = Self { x: 0, y: 0 };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_x(mut self, val: i32) -> Self {
    self.x = val;
    self
  }
  #[inline]
  pub const fn with_y(mut self, val: i32) -> Self {
    self.y = val;
    self
  }
}
/// [VkOffset3D](https://docs.vulkan.org/refpages/latest/refpages/source/VkOffset3D.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkOffset3D {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkOffset3D {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkOffset3D {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkOffset3D {
  pub const DEFAULT: Self = Self { x: 0, y: 0, z: 0 };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_x(mut self, val: i32) -> Self {
    self.x = val;
    self
  }
  #[inline]
  pub const fn with_y(mut self, val: i32) -> Self {
    self.y = val;
    self
  }
  #[inline]
  pub const fn with_z(mut self, val: i32) -> Self {
    self.z = val;
    self
  }
}
/// [VkExtent2D](https://docs.vulkan.org/refpages/latest/refpages/source/VkExtent2D.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExtent2D {
  pub width: u32,
  pub height: u32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkExtent2D {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkExtent2D {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkExtent2D {
  pub const DEFAULT: Self = Self {
    width: 0,
    height: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_width(mut self, val: u32) -> Self {
    self.width = val;
    self
  }
  #[inline]
  pub const fn with_height(mut self, val: u32) -> Self {
    self.height = val;
    self
  }
}
/// [VkExtent3D](https://docs.vulkan.org/refpages/latest/refpages/source/VkExtent3D.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExtent3D {
  pub width: u32,
  pub height: u32,
  pub depth: u32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkExtent3D {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkExtent3D {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkExtent3D {
  pub const DEFAULT: Self = Self {
    width: 0,
    height: 0,
    depth: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_width(mut self, val: u32) -> Self {
    self.width = val;
    self
  }
  #[inline]
  pub const fn with_height(mut self, val: u32) -> Self {
    self.height = val;
    self
  }
  #[inline]
  pub const fn with_depth(mut self, val: u32) -> Self {
    self.depth = val;
    self
  }
}
/// [VkRect2D](https://docs.vulkan.org/refpages/latest/refpages/source/VkRect2D.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRect2D {
  pub offset: VkOffset2D,
  pub extent: VkExtent2D,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkRect2D {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkRect2D {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkRect2D {
  pub const DEFAULT: Self = Self {
    offset: VkOffset2D::DEFAULT,
    extent: VkExtent2D::DEFAULT,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_offset(mut self, val: VkOffset2D) -> Self {
    self.offset = val;
    self
  }
  #[inline]
  pub const fn with_extent(mut self, val: VkExtent2D) -> Self {
    self.extent = val;
    self
  }
}
/// [VkComponentMapping](https://docs.vulkan.org/refpages/latest/refpages/source/VkComponentMapping.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkComponentMapping {
  pub r: VkComponentSwizzle,
  pub g: VkComponentSwizzle,
  pub b: VkComponentSwizzle,
  pub a: VkComponentSwizzle,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkComponentMapping {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkComponentMapping {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkComponentMapping {
  pub const DEFAULT: Self = Self {
    r: VkComponentSwizzle(0),
    g: VkComponentSwizzle(0),
    b: VkComponentSwizzle(0),
    a: VkComponentSwizzle(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_r(mut self, val: VkComponentSwizzle) -> Self {
    self.r = val;
    self
  }
  #[inline]
  pub const fn with_g(mut self, val: VkComponentSwizzle) -> Self {
    self.g = val;
    self
  }
  #[inline]
  pub const fn with_b(mut self, val: VkComponentSwizzle) -> Self {
    self.b = val;
    self
  }
  #[inline]
  pub const fn with_a(mut self, val: VkComponentSwizzle) -> Self {
    self.a = val;
    self
  }
}
/// [VkPhysicalDeviceProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceProperties {
  /// Limit Type: [Noauto]
  pub apiVersion: u32,
  /// Limit Type: [Noauto]
  pub driverVersion: u32,
  /// Limit Type: [Noauto]
  pub vendorID: u32,
  /// Limit Type: [Noauto]
  pub deviceID: u32,
  /// Limit Type: [Noauto]
  pub deviceType: VkPhysicalDeviceType,
  /// Length: null-terminated,  Limit Type: [Noauto]
  pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
  /// Limit Type: [Noauto]
  pub pipelineCacheUUID: [u8; VK_UUID_SIZE as usize],
  /// Limit Type: [Struct]
  pub limits: VkPhysicalDeviceLimits,
  /// Limit Type: [Struct]
  pub sparseProperties: VkPhysicalDeviceSparseProperties,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkPhysicalDeviceProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkPhysicalDeviceProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkPhysicalDeviceProperties {
  pub const DEFAULT: Self = Self {
    apiVersion: 0,
    driverVersion: 0,
    vendorID: 0,
    deviceID: 0,
    deviceType: VkPhysicalDeviceType(0),
    deviceName: [0 as c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    pipelineCacheUUID: [0u8; VK_UUID_SIZE as usize],
    limits: VkPhysicalDeviceLimits::DEFAULT,
    sparseProperties: VkPhysicalDeviceSparseProperties::DEFAULT,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_apiVersion(mut self, val: u32) -> Self {
    self.apiVersion = val;
    self
  }
  #[inline]
  pub const fn with_driverVersion(mut self, val: u32) -> Self {
    self.driverVersion = val;
    self
  }
  #[inline]
  pub const fn with_vendorID(mut self, val: u32) -> Self {
    self.vendorID = val;
    self
  }
  #[inline]
  pub const fn with_deviceID(mut self, val: u32) -> Self {
    self.deviceID = val;
    self
  }
  #[inline]
  pub const fn with_deviceType(mut self, val: VkPhysicalDeviceType) -> Self {
    self.deviceType = val;
    self
  }
  #[inline]
  pub const fn with_deviceName(
    mut self,
    val: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
  ) -> Self {
    self.deviceName = val;
    self
  }
  #[inline]
  pub const fn with_pipelineCacheUUID(mut self, val: [u8; VK_UUID_SIZE as usize]) -> Self {
    self.pipelineCacheUUID = val;
    self
  }
  #[inline]
  pub const fn with_limits(mut self, val: VkPhysicalDeviceLimits) -> Self {
    self.limits = val;
    self
  }
  #[inline]
  pub const fn with_sparseProperties(mut self, val: VkPhysicalDeviceSparseProperties) -> Self {
    self.sparseProperties = val;
    self
  }
}
/// [VkExtensionProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkExtensionProperties.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExtensionProperties {
  /// Length: null-terminated
  pub extensionName: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
  pub specVersion: u32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkExtensionProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkExtensionProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkExtensionProperties {
  pub const DEFAULT: Self = Self {
    extensionName: [0 as c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
    specVersion: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_extensionName(
    mut self,
    val: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
  ) -> Self {
    self.extensionName = val;
    self
  }
  #[inline]
  pub const fn with_specVersion(mut self, val: u32) -> Self {
    self.specVersion = val;
    self
  }
}
/// [VkLayerProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkLayerProperties.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkLayerProperties {
  /// Length: null-terminated
  pub layerName: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
  pub specVersion: u32,
  pub implementationVersion: u32,
  /// Length: null-terminated
  pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkLayerProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkLayerProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkLayerProperties {
  pub const DEFAULT: Self = Self {
    layerName: [0 as c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
    specVersion: 0,
    implementationVersion: 0,
    description: [0 as c_char; VK_MAX_DESCRIPTION_SIZE as usize],
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_layerName(
    mut self,
    val: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
  ) -> Self {
    self.layerName = val;
    self
  }
  #[inline]
  pub const fn with_specVersion(mut self, val: u32) -> Self {
    self.specVersion = val;
    self
  }
  #[inline]
  pub const fn with_implementationVersion(mut self, val: u32) -> Self {
    self.implementationVersion = val;
    self
  }
  #[inline]
  pub const fn with_description(mut self, val: [c_char; VK_MAX_DESCRIPTION_SIZE as usize]) -> Self {
    self.description = val;
    self
  }
}
/// [VkApplicationInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkApplicationInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkApplicationInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_APPLICATION_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true,  Length: null-terminated
  pub pApplicationName: *const c_char,
  pub applicationVersion: u32,
  /// Optional: true,  Length: null-terminated
  pub pEngineName: *const c_char,
  pub engineVersion: u32,
  pub apiVersion: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkApplicationInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkApplicationInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkApplicationInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::APPLICATION_INFO,
    pNext: core::ptr::null(),
    pApplicationName: core::ptr::null(),
    applicationVersion: 0,
    pEngineName: core::ptr::null(),
    engineVersion: 0,
    apiVersion: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pApplicationName(mut self, val: *const c_char) -> Self {
    self.pApplicationName = val;
    self
  }
  #[inline]
  pub const fn with_applicationVersion(mut self, val: u32) -> Self {
    self.applicationVersion = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pEngineName(mut self, val: *const c_char) -> Self {
    self.pEngineName = val;
    self
  }
  #[inline]
  pub const fn with_engineVersion(mut self, val: u32) -> Self {
    self.engineVersion = val;
    self
  }
  #[inline]
  pub const fn with_apiVersion(mut self, val: u32) -> Self {
    self.apiVersion = val;
    self
  }
  #[cfg(feature = "VK_EXT_application_parameters")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkApplicationParametersEXT<'child>(
    mut self,
    val: &'a VkApplicationParametersEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkApplicationParametersEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkApplicationInfo<
    'root,
    T: VkPNextExtends<VkApplicationInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkAllocationCallbacks](https://docs.vulkan.org/refpages/latest/refpages/source/VkAllocationCallbacks.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAllocationCallbacks<'a> {
  /// Optional: true
  pub pUserData: *mut c_void,
  /// No Auto-Validity
  pub pfnAllocation: PFN_vkAllocationFunction,
  /// No Auto-Validity
  pub pfnReallocation: PFN_vkReallocationFunction,
  /// No Auto-Validity
  pub pfnFree: PFN_vkFreeFunction,
  /// Optional: true,  No Auto-Validity
  pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
  /// Optional: true,  No Auto-Validity
  pub pfnInternalFree: PFN_vkInternalFreeNotification,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkAllocationCallbacks<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkAllocationCallbacks<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkAllocationCallbacks<'a> {
  pub const DEFAULT: Self = Self {
    pUserData: core::ptr::null_mut(),
    pfnAllocation: None,
    pfnReallocation: None,
    pfnFree: None,
    pfnInternalAllocation: None,
    pfnInternalFree: None,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pUserData(mut self, val: *mut c_void) -> Self {
    self.pUserData = val;
    self
  }
  #[inline]
  pub const fn with_pfnAllocation(mut self, val: PFN_vkAllocationFunction) -> Self {
    self.pfnAllocation = val;
    self
  }
  #[inline]
  pub const fn with_pfnReallocation(mut self, val: PFN_vkReallocationFunction) -> Self {
    self.pfnReallocation = val;
    self
  }
  #[inline]
  pub const fn with_pfnFree(mut self, val: PFN_vkFreeFunction) -> Self {
    self.pfnFree = val;
    self
  }
  #[inline]
  pub const fn with_pfnInternalAllocation(
    mut self,
    val: PFN_vkInternalAllocationNotification,
  ) -> Self {
    self.pfnInternalAllocation = val;
    self
  }
  #[inline]
  pub const fn with_pfnInternalFree(mut self, val: PFN_vkInternalFreeNotification) -> Self {
    self.pfnInternalFree = val;
    self
  }
}
/// [VkDeviceQueueCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueCreateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceQueueCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkDeviceQueueCreateFlags,
  pub queueFamilyIndex: u32,
  pub queueCount: u32,
  /// Length: queueCount
  pub pQueuePriorities: *const f32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkDeviceQueueCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkDeviceQueueCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkDeviceQueueCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_QUEUE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkDeviceQueueCreateFlagBits(0),
    queueFamilyIndex: 0,
    queueCount: 0,
    pQueuePriorities: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkDeviceQueueCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_queueFamilyIndex(mut self, val: u32) -> Self {
    self.queueFamilyIndex = val;
    self
  }
  #[inline]
  pub const fn with_queueCount(mut self, val: u32) -> Self {
    self.queueCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pQueuePriorities(mut self, val: &'a [f32]) -> Self {
    self.queueCount = val.len() as u32;
    self.pQueuePriorities = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceQueueGlobalPriorityCreateInfo<'child>(
    mut self,
    val: &'a VkDeviceQueueGlobalPriorityCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDeviceQueueGlobalPriorityCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_scheduling_controls")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceQueueShaderCoreControlCreateInfoARM<'child>(
    mut self,
    val: &'a VkDeviceQueueShaderCoreControlCreateInfoARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDeviceQueueShaderCoreControlCreateInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDeviceQueueCreateInfo<
    'root,
    T: VkPNextExtends<VkDeviceQueueCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDeviceCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceCreateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkDeviceCreateFlags,
  /// Optional: true
  pub queueCreateInfoCount: u32,
  /// Length: queueCreateInfoCount
  pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  #[deprecated(note = "unused")]
  pub enabledLayerCount: u32,
  /// Length: enabledLayerCount,null-terminated,  No Auto-Validity
  #[deprecated(note = "unused")]
  pub ppEnabledLayerNames: *const *const c_char,
  /// Optional: true
  pub enabledExtensionCount: u32,
  /// Length: enabledExtensionCount,null-terminated
  pub ppEnabledExtensionNames: *const *const c_char,
  /// Optional: true
  pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkDeviceCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkDeviceCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkDeviceCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    queueCreateInfoCount: 0,
    pQueueCreateInfos: core::ptr::null(),
    enabledLayerCount: 0,
    ppEnabledLayerNames: core::ptr::null(),
    enabledExtensionCount: 0,
    ppEnabledExtensionNames: core::ptr::null(),
    pEnabledFeatures: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkDeviceCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_queueCreateInfoCount(mut self, val: u32) -> Self {
    self.queueCreateInfoCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pQueueCreateInfos(mut self, val: &'a [VkDeviceQueueCreateInfo<'a>]) -> Self {
    self.queueCreateInfoCount = val.len() as u32;
    self.pQueueCreateInfos = val.as_ptr();
    self
  }
  #[deprecated(note = "unused")]
  #[inline]
  pub const fn with_enabledLayerCount(mut self, val: u32) -> Self {
    self.enabledLayerCount = val;
    self
  }
  #[deprecated(note = "unused")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_ppEnabledLayerNames(mut self, val: &'a [*const c_char]) -> Self {
    self.enabledLayerCount = val.len() as u32;
    self.ppEnabledLayerNames = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_enabledExtensionCount(mut self, val: u32) -> Self {
    self.enabledExtensionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_ppEnabledExtensionNames(mut self, val: &'a [*const c_char]) -> Self {
    self.enabledExtensionCount = val.len() as u32;
    self.ppEnabledExtensionNames = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pEnabledFeatures(mut self, val: *const VkPhysicalDeviceFeatures) -> Self {
    self.pEnabledFeatures = val;
    self
  }
  #[cfg(feature = "VK_EXT_application_parameters")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkApplicationParametersEXT<'child>(
    mut self,
    val: &'a VkApplicationParametersEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkApplicationParametersEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_device_memory_report")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceDeviceMemoryReportCreateInfoEXT<'child>(
    mut self,
    val: &'a VkDeviceDeviceMemoryReportCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDeviceDeviceMemoryReportCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_device_diagnostics_config")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceDiagnosticsConfigCreateInfoNV<'child>(
    mut self,
    val: &'a VkDeviceDiagnosticsConfigCreateInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDeviceDiagnosticsConfigCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceGroupDeviceCreateInfo<'child>(
    mut self,
    val: &'a VkDeviceGroupDeviceCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDeviceGroupDeviceCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceMemoryOverallocationCreateInfoAMD<'child>(
    mut self,
    val: &'a VkDeviceMemoryOverallocationCreateInfoAMD<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDeviceMemoryOverallocationCreateInfoAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceObjectReservationCreateInfo<'child>(
    mut self,
    val: &'a VkDeviceObjectReservationCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDeviceObjectReservationCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDevicePipelineBinaryInternalCacheControlKHR<'child>(
    mut self,
    val: &'a VkDevicePipelineBinaryInternalCacheControlKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDevicePipelineBinaryInternalCacheControlKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDevicePrivateDataCreateInfo<'child>(
    mut self,
    val: &'a VkDevicePrivateDataCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDevicePrivateDataCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_scheduling_controls")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceQueueShaderCoreControlCreateInfoARM<'child>(
    mut self,
    val: &'a VkDeviceQueueShaderCoreControlCreateInfoARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDeviceQueueShaderCoreControlCreateInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_NV_external_sci_sync2"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV<'child>(
    mut self,
    val: &'a VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_compute_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalComputeQueueDeviceCreateInfoNV<'child>(
    mut self,
    val: &'a VkExternalComputeQueueDeviceCreateInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalComputeQueueDeviceCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFaultCallbackInfo<'child>(
    mut self,
    val: &'a VkFaultCallbackInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkFaultCallbackInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_KHR_performance_query"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPerformanceQueryReservationInfoKHR<'child>(
    mut self,
    val: &'a VkPerformanceQueryReservationInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPerformanceQueryReservationInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevice16BitStorageFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDevice16BitStorageFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevice16BitStorageFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_4444_formats")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevice4444FormatsFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDevice4444FormatsFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevice4444FormatsFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevice8BitStorageFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDevice8BitStorageFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevice8BitStorageFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_astc_decode_mode")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceASTCDecodeFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceASTCDecodeFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceASTCDecodeFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAccelerationStructureFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceAccelerationStructureFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceAccelerationStructureFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_device_address_binding_report")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAddressBindingReportFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceAddressBindingReportFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceAddressBindingReportFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_SEC_amigo_profiling")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAmigoProfilingFeaturesSEC<'child>(
    mut self,
    val: &'a VkPhysicalDeviceAmigoProfilingFeaturesSEC<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceAmigoProfilingFeaturesSEC<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_anti_lag")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAntiLagFeaturesAMD<'child>(
    mut self,
    val: &'a VkPhysicalDeviceAntiLagFeaturesAMD<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceAntiLagFeaturesAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_attachment_feedback_loop_dynamic_state")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val
      as *const VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_attachment_feedback_loop_layout")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_blend_operation_advanced")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_border_color_swizzle")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceBorderColorSwizzleFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceBorderColorSwizzleFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceBorderColorSwizzleFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceBufferDeviceAddressFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceBufferDeviceAddressFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceBufferDeviceAddressFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_buffer_device_address")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceBufferDeviceAddressFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceBufferDeviceAddressFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceBufferDeviceAddressFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'child>(
    mut self,
    val: &'a VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_device_coherent_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCoherentMemoryFeaturesAMD<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCoherentMemoryFeaturesAMD<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceCoherentMemoryFeaturesAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_color_write_enable")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceColorWriteEnableFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceColorWriteEnableFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceColorWriteEnableFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_command_buffer_inheritance")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_compute_occupancy_priority")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_compute_shader_derivatives")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_conditional_rendering")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceConditionalRenderingFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceConditionalRenderingFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceConditionalRenderingFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cooperative_matrix2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_cooperative_matrix_conversion")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cooperative_matrix_decode_vector")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_cooperative_matrix")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrixFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCooperativeMatrixFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceCooperativeMatrixFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cooperative_matrix")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrixFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCooperativeMatrixFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceCooperativeMatrixFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cooperative_vector")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeVectorFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCooperativeVectorFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceCooperativeVectorFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_copy_memory_indirect")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_copy_memory_indirect")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_corner_sampled_image")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCornerSampledImageFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCornerSampledImageFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceCornerSampledImageFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_coverage_reduction_mode")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCoverageReductionModeFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCoverageReductionModeFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceCoverageReductionModeFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_filter_cubic_clamp")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCubicClampFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCubicClampFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceCubicClampFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_filter_cubic_weights")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCubicWeightsFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCubicWeightsFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceCubicWeightsFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_custom_border_color")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCustomBorderColorFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCustomBorderColorFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceCustomBorderColorFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_custom_resolve")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCustomResolveFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceCustomResolveFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceCustomResolveFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDataGraphFeaturesARM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDataGraphFeaturesARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceDataGraphFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_data_graph_model")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDataGraphModelFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDataGraphModelFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDataGraphModelFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<
    'child,
  >(
    mut self,
    val: &'a VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'child>,
  ) -> Self {
    self.pNext = (val
      as *const VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMDX_dense_geometry_format")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_depth_bias_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDepthBiasControlFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDepthBiasControlFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDepthBiasControlFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_depth_clamp_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDepthClampControlFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDepthClampControlFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDepthClampControlFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_depth_clamp_zero_one")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDepthClampZeroOneFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDepthClampZeroOneFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDepthClampZeroOneFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_depth_clip_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDepthClipControlFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDepthClipControlFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDepthClipControlFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_depth_clip_enable")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDepthClipEnableFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDepthClipEnableFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDepthClipEnableFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorBufferFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDescriptorBufferFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDescriptorBufferFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorHeapFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDescriptorHeapFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceDescriptorHeapFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorIndexingFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDescriptorIndexingFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDescriptorIndexingFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_descriptor_pool_overallocation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_device_address_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands_compute")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_device_memory_report")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDeviceMemoryReportFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDeviceMemoryReportFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDeviceMemoryReportFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_device_diagnostics_config")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_displacement_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDisplacementMicromapFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDisplacementMicromapFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDisplacementMicromapFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDynamicRenderingFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDynamicRenderingFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceDynamicRenderingFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_dynamic_rendering_unused_attachments")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val
      as *const VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_elapsed_timer_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_scissor_exclusive")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExclusiveScissorFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExclusiveScissorFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceExclusiveScissorFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_extended_dynamic_state2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedDynamicState2FeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExtendedDynamicState2FeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceExtendedDynamicState2FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_extended_dynamic_state3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedDynamicState3FeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExtendedDynamicState3FeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceExtendedDynamicState3FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_extended_dynamic_state")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedDynamicStateFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExtendedDynamicStateFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceExtendedDynamicStateFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_extended_flags")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedFlagsFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExtendedFlagsFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceExtendedFlagsFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_extended_sparse_address_space")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ANDROID_external_format_resolve")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_memory_rdma")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_sci_sync2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalSciSync2FeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExternalSciSync2FeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceExternalSciSync2FeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_sci_sync")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalSciSyncFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExternalSciSyncFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceExternalSciSyncFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_device_fault")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFaultFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFaultFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceFaultFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_device_fault")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFaultFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFaultFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceFaultFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFeatures2<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFeatures2<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceFeatures2<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_format_pack")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFormatPackFeaturesARM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFormatPackFeaturesARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceFormatPackFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_density_map2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMap2FeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFragmentDensityMap2FeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceFragmentDensityMap2FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_density_map")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMapFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFragmentDensityMapFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceFragmentDensityMapFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_density_map_offset")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_fragment_shader_barycentric")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_shader_interlock")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_fragment_shading_rate")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentShadingRateFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFragmentShadingRateFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceFragmentShadingRateFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_frame_boundary")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFrameBoundaryFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceFrameBoundaryFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceFrameBoundaryFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceGlobalPriorityQueryFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceGlobalPriorityQueryFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceGlobalPriorityQueryFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_gpa_interface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceGpaFeaturesAMD<'child>(
    mut self,
    val: &'a VkPhysicalDeviceGpaFeaturesAMD<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceGpaFeaturesAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_graphics_pipeline_library")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_HUAWEI_hdr_vivid")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceHdrVividFeaturesHUAWEI<'child>(
    mut self,
    val: &'a VkPhysicalDeviceHdrVividFeaturesHUAWEI<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceHdrVividFeaturesHUAWEI<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceHostImageCopyFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceHostImageCopyFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceHostImageCopyFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceHostQueryResetFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceHostQueryResetFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceHostQueryResetFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_2d_view_of_3d")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImage2DViewOf3DFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImage2DViewOf3DFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImage2DViewOf3DFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_MESA_image_alignment_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_compression_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageCompressionControlFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageCompressionControlFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImageCompressionControlFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_compression_control_swapchain")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val
      as *const VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_image_processing2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageProcessing2FeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageProcessing2FeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImageProcessing2FeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_image_processing3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageProcessing3FeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageProcessing3FeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImageProcessing3FeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_image_processing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageProcessingFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageProcessingFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImageProcessingFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageRobustnessFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageRobustnessFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceImageRobustnessFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_sliced_view_of_3d")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_tiling_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageTilingControlFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageTilingControlFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImageTilingControlFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_view_min_lod")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageViewMinLodFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageViewMinLodFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImageViewMinLodFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImagelessFramebufferFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImagelessFramebufferFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImagelessFramebufferFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceIndexTypeUint8Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceIndexTypeUint8Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceIndexTypeUint8Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_inherited_viewport_scissor")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceInlineUniformBlockFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceInlineUniformBlockFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceInlineUniformBlockFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_internally_synchronized_queues")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_HUAWEI_invocation_mask")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceInvocationMaskFeaturesHUAWEI<'child>(
    mut self,
    val: &'a VkPhysicalDeviceInvocationMaskFeaturesHUAWEI<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceInvocationMaskFeaturesHUAWEI<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_legacy_dithering")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLegacyDitheringFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceLegacyDitheringFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceLegacyDitheringFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_legacy_vertex_attributes")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLineRasterizationFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceLineRasterizationFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceLineRasterizationFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_linear_color_attachment")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance10")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance10FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMaintenance10FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMaintenance10FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance11")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance11FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMaintenance11FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMaintenance11FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance4Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMaintenance4Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMaintenance4Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance5Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMaintenance5Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMaintenance5Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance6Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMaintenance6Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMaintenance6Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance7")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance7FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMaintenance7FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMaintenance7FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance8")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance8FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMaintenance8FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMaintenance8FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance9")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance9FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMaintenance9FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMaintenance9FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_map_memory_placed")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMapMemoryPlacedFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMapMemoryPlacedFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceMapMemoryPlacedFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_memory_decompression")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMemoryDecompressionFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMemoryDecompressionFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceMemoryDecompressionFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_memory_priority")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMemoryPriorityFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMemoryPriorityFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMemoryPriorityFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_mesh_shader")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMeshShaderFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMeshShaderFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMeshShaderFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_mesh_shader")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMeshShaderFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMeshShaderFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMeshShaderFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_multi_draw")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultiDrawFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMultiDrawFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMultiDrawFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val
      as *const VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_multisampled_render_to_swapchain")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultiviewFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMultiviewFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMultiviewFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_multiview_per_view_viewports")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_mutable_descriptor_type")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_nested_command_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceNestedCommandBufferFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceNestedCommandBufferFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceNestedCommandBufferFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_non_seamless_cube_map")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceOpacityMicromapFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceOpacityMicromapFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceOpacityMicromapFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_opacity_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceOpacityMicromapFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceOpacityMicromapFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceOpacityMicromapFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceOpticalFlowFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceOpticalFlowFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceOpticalFlowFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_per_stage_descriptor_set")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePerStageDescriptorSetFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDevicePerStageDescriptorSetFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePerStageDescriptorSetFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_performance_counters_by_region")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'child>(
    mut self,
    val: &'a VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePerformanceQueryFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDevicePerformanceQueryFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePerformanceQueryFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineBinaryFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDevicePipelineBinaryFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePipelineBinaryFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_SEC_pipeline_cache_incremental_mode")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'child>(
    mut self,
    val: &'a VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineCreationCacheControlFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDevicePipelineCreationCacheControlFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePipelineCreationCacheControlFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_pipeline_library_group_handles")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_pipeline_opacity_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'child>(
    mut self,
    val: &'a VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_pipeline_properties")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelinePropertiesFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDevicePipelinePropertiesFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePipelinePropertiesFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineProtectedAccessFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDevicePipelineProtectedAccessFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePipelineProtectedAccessFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineRobustnessFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDevicePipelineRobustnessFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePipelineRobustnessFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_portability_subset")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePortabilitySubsetFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDevicePortabilitySubsetFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePortabilitySubsetFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_present_barrier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentBarrierFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDevicePresentBarrierFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePresentBarrierFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_present_id2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentId2FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDevicePresentId2FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePresentId2FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_present_id")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentIdFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDevicePresentIdFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePresentIdFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_present_metering")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentMeteringFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDevicePresentMeteringFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePresentMeteringFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_present_mode_fifo_latest_ready")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_present_timing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentTimingFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDevicePresentTimingFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePresentTimingFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_present_wait2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentWait2FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDevicePresentWait2FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePresentWait2FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_present_wait")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentWaitFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDevicePresentWaitFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePresentWaitFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_primitive_restart_index")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_primitives_generated_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePrivateDataFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDevicePrivateDataFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDevicePrivateDataFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceProtectedMemoryFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceProtectedMemoryFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceProtectedMemoryFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_provoking_vertex")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceProvokingVertexFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceProvokingVertexFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceProvokingVertexFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_push_constant_bank")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePushConstantBankFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDevicePushConstantBankFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDevicePushConstantBankFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_queue_perf_hint")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_rgba10x6_formats")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_rasterization_order_attachment_access")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val
      as *const VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_raw_access_chains")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRawAccessChainsFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRawAccessChainsFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceRawAccessChainsFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_ray_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayQueryFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRayQueryFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceRayQueryFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_ray_tracing_invocation_reorder")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_ray_tracing_maintenance1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingPipelineFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRayTracingPipelineFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceRayTracingPipelineFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_ray_tracing_position_fetch")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing_validation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingValidationFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRayTracingValidationFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceRayTracingValidationFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_IMG_relaxed_line_rasterization")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_render_pass_striped")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRenderPassStripedFeaturesARM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRenderPassStripedFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceRenderPassStripedFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_representative_fragment_test")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_robustness2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRobustness2FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceRobustness2FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceRobustness2FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSamplerYcbcrConversionFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceSamplerYcbcrConversionFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceSamplerYcbcrConversionFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceScalarBlockLayoutFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceScalarBlockLayoutFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceScalarBlockLayoutFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_scheduling_controls")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSchedulingControlsFeaturesARM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceSchedulingControlsFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceSchedulingControlsFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_64bit_indexing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShader64BitIndexingFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShader64BitIndexingFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShader64BitIndexingFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_abort")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderAbortFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderAbortFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderAbortFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_shader_atomic_float16_vector")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_atomic_float2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_atomic_float")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderAtomicFloatFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderAtomicFloatFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderAtomicFloatFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderAtomicInt64Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderAtomicInt64Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderAtomicInt64Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_bfloat16")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderBfloat16FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderBfloat16FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderBfloat16FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_clock")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderClockFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderClockFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderClockFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_constant_data")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderConstantDataFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderConstantDataFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderConstantDataFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_shader_core_builtins")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderDrawParametersFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderDrawParametersFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderDrawParametersFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_shader_early_and_late_fragment_tests")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMDX_shader_enqueue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderEnqueueFeaturesAMDX<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderEnqueueFeaturesAMDX<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderEnqueueFeaturesAMDX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderExpectAssumeFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderExpectAssumeFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderExpectAssumeFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderFloat16Int8Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderFloat16Int8Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderFloat16Int8Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_float8")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderFloat8FeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderFloat8FeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderFloat8FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderFloatControls2Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderFloatControls2Features<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderFloatControls2Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_fma")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderFmaFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderFmaFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderFmaFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_shader_image_footprint")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderImageFootprintFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderImageFootprintFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderImageFootprintFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderInstrumentationFeaturesARM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderInstrumentationFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderInstrumentationFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderIntegerDotProductFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderIntegerDotProductFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderIntegerDotProductFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_INTEL_shader_integer_functions2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_long_vector")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderLongVectorFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderLongVectorFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderLongVectorFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_maximal_reconvergence")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_VALVE_shader_mixed_float_dot_product")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_module_identifier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_ocp_microscaling_types")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_object")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderObjectFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderObjectFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderObjectFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_quad_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderQuadControlFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderQuadControlFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderQuadControlFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_relaxed_extended_instruction")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val
      as *const VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_replicated_composites")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_shader_sm_builtins")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_split_barrier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSplitBarrierFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderSplitBarrierFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderSplitBarrierFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_subgroup_partitioned")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSubgroupPartitionedFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderSubgroupPartitionedFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderSubgroupPartitionedFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSubgroupRotateFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderSubgroupRotateFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderSubgroupRotateFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val
      as *const VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderTerminateInvocationFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderTerminateInvocationFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderTerminateInvocationFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_tile_image")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderTileImageFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderTileImageFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderTileImageFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_uniform_buffer_unsized_array")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_untyped_pointers")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderUntypedPointersFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShaderUntypedPointersFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShaderUntypedPointersFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_shading_rate_image")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShadingRateImageFeaturesNV<'child>(
    mut self,
    val: &'a VkPhysicalDeviceShadingRateImageFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceShadingRateImageFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSubgroupSizeControlFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceSubgroupSizeControlFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceSubgroupSizeControlFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_subpass_merge_feedback")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_HUAWEI_subpass_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'child>(
    mut self,
    val: &'a VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_swapchain_maintenance1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSynchronization2Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceSynchronization2Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceSynchronization2Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTensorFeaturesARM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceTensorFeaturesARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceTensorFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_texel_buffer_alignment")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_texture_compression_astc_3d")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_SEC_throttle_hint")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceThrottleHintFeaturesSEC<'child>(
    mut self,
    val: &'a VkPhysicalDeviceThrottleHintFeaturesSEC<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceThrottleHintFeaturesSEC<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_memory_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_properties")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTilePropertiesFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceTilePropertiesFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceTilePropertiesFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTileShadingFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceTileShadingFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceTileShadingFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTimelineSemaphoreFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceTimelineSemaphoreFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceTimelineSemaphoreFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_transform_feedback")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTransformFeedbackFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceTransformFeedbackFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceTransformFeedbackFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_unified_image_layouts")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVariablePointersFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVariablePointersFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceVariablePointersFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVertexAttributeDivisorFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVertexAttributeDivisorFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceVertexAttributeDivisorFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_vertex_attribute_robustness")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_decode_vp9")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoDecodeVP9FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVideoDecodeVP9FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceVideoDecodeVP9FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_av1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoEncodeAV1FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVideoEncodeAV1FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceVideoEncodeAV1FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_feedback2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_quantization_map")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_maintenance1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoMaintenance1FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVideoMaintenance1FeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceVideoMaintenance1FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_maintenance2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoMaintenance2FeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVideoMaintenance2FeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceVideoMaintenance2FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan11Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVulkan11Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceVulkan11Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan12Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVulkan12Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceVulkan12Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan13Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVulkan13Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceVulkan13Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan14Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVulkan14Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceVulkan14Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkanMemoryModelFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVulkanMemoryModelFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceVulkanMemoryModelFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkanSC10Features<'child>(
    mut self,
    val: &'a VkPhysicalDeviceVulkanSC10Features<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceVulkanSC10Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'child>(
    mut self,
    val: &'a VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_ycbcr_degamma")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'child>(
    mut self,
    val: &'a VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceYcbcrImageArraysFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceYcbcrImageArraysFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceYcbcrImageArraysFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_zero_initialize_device_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'child>(
    mut self,
    val: &'a VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDeviceCreateInfo<
    'root,
    T: VkPNextExtends<VkDeviceCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkInstanceCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkInstanceCreateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkInstanceCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkInstanceCreateFlags,
  /// Optional: true
  pub pApplicationInfo: *const VkApplicationInfo<'a>,
  /// Optional: true
  pub enabledLayerCount: u32,
  /// Length: enabledLayerCount,null-terminated
  pub ppEnabledLayerNames: *const *const c_char,
  /// Optional: true
  pub enabledExtensionCount: u32,
  /// Length: enabledExtensionCount,null-terminated
  pub ppEnabledExtensionNames: *const *const c_char,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkInstanceCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkInstanceCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkInstanceCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::INSTANCE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkInstanceCreateFlagBits(0),
    pApplicationInfo: core::ptr::null(),
    enabledLayerCount: 0,
    ppEnabledLayerNames: core::ptr::null(),
    enabledExtensionCount: 0,
    ppEnabledExtensionNames: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkInstanceCreateFlags) -> Self {
    self.flags = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pApplicationInfo(mut self, val: *const VkApplicationInfo<'a>) -> Self {
    self.pApplicationInfo = val;
    self
  }
  #[inline]
  pub const fn with_enabledLayerCount(mut self, val: u32) -> Self {
    self.enabledLayerCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_ppEnabledLayerNames(mut self, val: &'a [*const c_char]) -> Self {
    self.enabledLayerCount = val.len() as u32;
    self.ppEnabledLayerNames = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_enabledExtensionCount(mut self, val: u32) -> Self {
    self.enabledExtensionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_ppEnabledExtensionNames(mut self, val: &'a [*const c_char]) -> Self {
    self.enabledExtensionCount = val.len() as u32;
    self.ppEnabledExtensionNames = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_EXT_debug_report")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDebugReportCallbackCreateInfoEXT<'child>(
    mut self,
    val: &'a VkDebugReportCallbackCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDebugReportCallbackCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_debug_utils")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDebugUtilsMessengerCreateInfoEXT<'child>(
    mut self,
    val: &'a VkDebugUtilsMessengerCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDebugUtilsMessengerCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_LUNARG_direct_driver_loading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDirectDriverLoadingListLUNARG<'child>(
    mut self,
    val: &'a VkDirectDriverLoadingListLUNARG<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDirectDriverLoadingListLUNARG<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_metal_objects")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportMetalObjectCreateInfoEXT<'child>(
    mut self,
    val: &'a VkExportMetalObjectCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportMetalObjectCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_layer_settings")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkLayerSettingsCreateInfoEXT<'child>(
    mut self,
    val: &'a VkLayerSettingsCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkLayerSettingsCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_validation_features")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkValidationFeaturesEXT<'child>(
    mut self,
    val: &'a VkValidationFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkValidationFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_validation_flags")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkValidationFlagsEXT<'child>(
    mut self,
    val: &'a VkValidationFlagsEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkValidationFlagsEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkInstanceCreateInfo<
    'root,
    T: VkPNextExtends<VkInstanceCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkQueueFamilyProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueueFamilyProperties {
  /// Optional: true,  Limit Type: [Bitmask]
  pub queueFlags: VkQueueFlags,
  /// Limit Type: [Max]
  pub queueCount: u32,
  /// Limit Type: [Bits]
  pub timestampValidBits: u32,
  /// Limit Type: [Min, Mul]
  pub minImageTransferGranularity: VkExtent3D,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkQueueFamilyProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkQueueFamilyProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkQueueFamilyProperties {
  pub const DEFAULT: Self = Self {
    queueFlags: VkQueueFlagBits(0),
    queueCount: 0,
    timestampValidBits: 0,
    minImageTransferGranularity: VkExtent3D::DEFAULT,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_queueFlags(mut self, val: VkQueueFlags) -> Self {
    self.queueFlags = val;
    self
  }
  #[inline]
  pub const fn with_queueCount(mut self, val: u32) -> Self {
    self.queueCount = val;
    self
  }
  #[inline]
  pub const fn with_timestampValidBits(mut self, val: u32) -> Self {
    self.timestampValidBits = val;
    self
  }
  #[inline]
  pub const fn with_minImageTransferGranularity(mut self, val: VkExtent3D) -> Self {
    self.minImageTransferGranularity = val;
    self
  }
}
/// [VkPhysicalDeviceMemoryProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryProperties.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMemoryProperties {
  pub memoryTypeCount: u32,
  /// Length: memoryTypeCount
  pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES as usize],
  pub memoryHeapCount: u32,
  /// Length: memoryHeapCount
  pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS as usize],
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkPhysicalDeviceMemoryProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkPhysicalDeviceMemoryProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkPhysicalDeviceMemoryProperties {
  pub const DEFAULT: Self = Self {
    memoryTypeCount: 0,
    memoryTypes: [VkMemoryType::DEFAULT; VK_MAX_MEMORY_TYPES as usize],
    memoryHeapCount: 0,
    memoryHeaps: [VkMemoryHeap::DEFAULT; VK_MAX_MEMORY_HEAPS as usize],
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_memoryTypeCount(mut self, val: u32) -> Self {
    self.memoryTypeCount = val;
    self
  }
  #[inline]
  pub const fn with_memoryTypes(
    mut self,
    val: [VkMemoryType; VK_MAX_MEMORY_TYPES as usize],
  ) -> Self {
    self.memoryTypes = val;
    self
  }
  #[inline]
  pub const fn with_memoryHeapCount(mut self, val: u32) -> Self {
    self.memoryHeapCount = val;
    self
  }
  #[inline]
  pub const fn with_memoryHeaps(
    mut self,
    val: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS as usize],
  ) -> Self {
    self.memoryHeaps = val;
    self
  }
}
/// [VkMemoryAllocateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryAllocateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub allocationSize: VkDeviceSize,
  pub memoryTypeIndex: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkMemoryAllocateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkMemoryAllocateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkMemoryAllocateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_ALLOCATE_INFO,
    pNext: core::ptr::null(),
    allocationSize: 0,
    memoryTypeIndex: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_allocationSize(mut self, val: VkDeviceSize) -> Self {
    self.allocationSize = val;
    self
  }
  #[inline]
  pub const fn with_memoryTypeIndex(mut self, val: u32) -> Self {
    self.memoryTypeIndex = val;
    self
  }
  #[cfg(feature = "VK_NV_dedicated_allocation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDedicatedAllocationMemoryAllocateInfoNV<'child>(
    mut self,
    val: &'a VkDedicatedAllocationMemoryAllocateInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDedicatedAllocationMemoryAllocateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportMemoryAllocateInfo<'child>(
    mut self,
    val: &'a VkExportMemoryAllocateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportMemoryAllocateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportMemoryAllocateInfoNV<'child>(
    mut self,
    val: &'a VkExportMemoryAllocateInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportMemoryAllocateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportMemorySciBufInfoNV<'child>(
    mut self,
    val: &'a VkExportMemorySciBufInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportMemorySciBufInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_external_memory_win32")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportMemoryWin32HandleInfoKHR<'child>(
    mut self,
    val: &'a VkExportMemoryWin32HandleInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportMemoryWin32HandleInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_memory_win32")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportMemoryWin32HandleInfoNV<'child>(
    mut self,
    val: &'a VkExportMemoryWin32HandleInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportMemoryWin32HandleInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_metal_objects")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportMetalObjectCreateInfoEXT<'child>(
    mut self,
    val: &'a VkExportMetalObjectCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportMetalObjectCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportAndroidHardwareBufferInfoANDROID<'child>(
    mut self,
    val: &'a VkImportAndroidHardwareBufferInfoANDROID<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportAndroidHardwareBufferInfoANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMemoryBufferCollectionFUCHSIA<'child>(
    mut self,
    val: &'a VkImportMemoryBufferCollectionFUCHSIA<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMemoryBufferCollectionFUCHSIA<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_external_memory_fd")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMemoryFdInfoKHR<'child>(
    mut self,
    val: &'a VkImportMemoryFdInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMemoryFdInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_external_memory_host")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMemoryHostPointerInfoEXT<'child>(
    mut self,
    val: &'a VkImportMemoryHostPointerInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMemoryHostPointerInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_external_memory_metal")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMemoryMetalHandleInfoEXT<'child>(
    mut self,
    val: &'a VkImportMemoryMetalHandleInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMemoryMetalHandleInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMemorySciBufInfoNV<'child>(
    mut self,
    val: &'a VkImportMemorySciBufInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMemorySciBufInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_external_memory_win32")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMemoryWin32HandleInfoKHR<'child>(
    mut self,
    val: &'a VkImportMemoryWin32HandleInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMemoryWin32HandleInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_memory_win32")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMemoryWin32HandleInfoNV<'child>(
    mut self,
    val: &'a VkImportMemoryWin32HandleInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMemoryWin32HandleInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_FUCHSIA_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMemoryZirconHandleInfoFUCHSIA<'child>(
    mut self,
    val: &'a VkImportMemoryZirconHandleInfoFUCHSIA<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMemoryZirconHandleInfoFUCHSIA<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_metal_objects")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMetalBufferInfoEXT<'child>(
    mut self,
    val: &'a VkImportMetalBufferInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMetalBufferInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_OHOS_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportNativeBufferInfoOHOS<'child>(
    mut self,
    val: &'a VkImportNativeBufferInfoOHOS<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportNativeBufferInfoOHOS<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportScreenBufferInfoQNX<'child>(
    mut self,
    val: &'a VkImportScreenBufferInfoQNX<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportScreenBufferInfoQNX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryAllocateFlagsInfo<'child>(
    mut self,
    val: &'a VkMemoryAllocateFlagsInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMemoryAllocateFlagsInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryDedicatedAllocateInfo<'child>(
    mut self,
    val: &'a VkMemoryDedicatedAllocateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMemoryDedicatedAllocateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryDedicatedAllocateInfoTensorARM<'child>(
    mut self,
    val: &'a VkMemoryDedicatedAllocateInfoTensorARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMemoryDedicatedAllocateInfoTensorARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryOpaqueCaptureAddressAllocateInfo<'child>(
    mut self,
    val: &'a VkMemoryOpaqueCaptureAddressAllocateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMemoryOpaqueCaptureAddressAllocateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_memory_priority")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryPriorityAllocateInfoEXT<'child>(
    mut self,
    val: &'a VkMemoryPriorityAllocateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMemoryPriorityAllocateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryAllocateInfo<
    'root,
    T: VkPNextExtends<VkMemoryAllocateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkMemoryRequirements](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryRequirements.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryRequirements {
  pub size: VkDeviceSize,
  pub alignment: VkDeviceSize,
  pub memoryTypeBits: u32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkMemoryRequirements {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkMemoryRequirements {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkMemoryRequirements {
  pub const DEFAULT: Self = Self {
    size: 0,
    alignment: 0,
    memoryTypeBits: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[inline]
  pub const fn with_alignment(mut self, val: VkDeviceSize) -> Self {
    self.alignment = val;
    self
  }
  #[inline]
  pub const fn with_memoryTypeBits(mut self, val: u32) -> Self {
    self.memoryTypeBits = val;
    self
  }
}
/// [VkSparseImageFormatProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSparseImageFormatProperties {
  /// Optional: true,  Limit Type: [Bitmask]
  pub aspectMask: VkImageAspectFlags,
  /// Limit Type: [Min, Mul]
  pub imageGranularity: VkExtent3D,
  /// Optional: true,  Limit Type: [Bitmask]
  pub flags: VkSparseImageFormatFlags,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Send for VkSparseImageFormatProperties {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Sync for VkSparseImageFormatProperties {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
impl VkSparseImageFormatProperties {
  pub const DEFAULT: Self = Self {
    aspectMask: VkImageAspectFlagBits(0),
    imageGranularity: VkExtent3D::DEFAULT,
    flags: VkSparseImageFormatFlagBits(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_aspectMask(mut self, val: VkImageAspectFlags) -> Self {
    self.aspectMask = val;
    self
  }
  #[inline]
  pub const fn with_imageGranularity(mut self, val: VkExtent3D) -> Self {
    self.imageGranularity = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkSparseImageFormatFlags) -> Self {
    self.flags = val;
    self
  }
}
/// [VkSparseImageMemoryRequirements](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageMemoryRequirements.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSparseImageMemoryRequirements {
  pub formatProperties: VkSparseImageFormatProperties,
  pub imageMipTailFirstLod: u32,
  pub imageMipTailSize: VkDeviceSize,
  pub imageMipTailOffset: VkDeviceSize,
  pub imageMipTailStride: VkDeviceSize,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Send for VkSparseImageMemoryRequirements {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Sync for VkSparseImageMemoryRequirements {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
impl VkSparseImageMemoryRequirements {
  pub const DEFAULT: Self = Self {
    formatProperties: VkSparseImageFormatProperties::DEFAULT,
    imageMipTailFirstLod: 0,
    imageMipTailSize: 0,
    imageMipTailOffset: 0,
    imageMipTailStride: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_formatProperties(mut self, val: VkSparseImageFormatProperties) -> Self {
    self.formatProperties = val;
    self
  }
  #[inline]
  pub const fn with_imageMipTailFirstLod(mut self, val: u32) -> Self {
    self.imageMipTailFirstLod = val;
    self
  }
  #[inline]
  pub const fn with_imageMipTailSize(mut self, val: VkDeviceSize) -> Self {
    self.imageMipTailSize = val;
    self
  }
  #[inline]
  pub const fn with_imageMipTailOffset(mut self, val: VkDeviceSize) -> Self {
    self.imageMipTailOffset = val;
    self
  }
  #[inline]
  pub const fn with_imageMipTailStride(mut self, val: VkDeviceSize) -> Self {
    self.imageMipTailStride = val;
    self
  }
}
/// [VkMemoryType](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryType.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryType {
  /// Optional: true
  pub propertyFlags: VkMemoryPropertyFlags,
  pub heapIndex: u32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkMemoryType {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkMemoryType {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkMemoryType {
  pub const DEFAULT: Self = Self {
    propertyFlags: VkMemoryPropertyFlagBits(0),
    heapIndex: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_propertyFlags(mut self, val: VkMemoryPropertyFlags) -> Self {
    self.propertyFlags = val;
    self
  }
  #[inline]
  pub const fn with_heapIndex(mut self, val: u32) -> Self {
    self.heapIndex = val;
    self
  }
}
/// [VkMemoryHeap](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHeap.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryHeap {
  pub size: VkDeviceSize,
  /// Optional: true
  pub flags: VkMemoryHeapFlags,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkMemoryHeap {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkMemoryHeap {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkMemoryHeap {
  pub const DEFAULT: Self = Self {
    size: 0,
    flags: VkMemoryHeapFlagBits(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkMemoryHeapFlags) -> Self {
    self.flags = val;
    self
  }
}
/// [VkMappedMemoryRange](https://docs.vulkan.org/refpages/latest/refpages/source/VkMappedMemoryRange.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMappedMemoryRange<'a> {
  /// Values: VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkMappedMemoryRange<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkMappedMemoryRange<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkMappedMemoryRange<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MAPPED_MEMORY_RANGE,
    pNext: core::ptr::null(),
    memory: VkDeviceMemory::DEFAULT,
    offset: 0,
    size: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_memory(mut self, val: VkDeviceMemory) -> Self {
    self.memory = val;
    self
  }
  #[inline]
  pub const fn with_offset(mut self, val: VkDeviceSize) -> Self {
    self.offset = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMappedMemoryRange<
    'root,
    T: VkPNextExtends<VkMappedMemoryRange<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkFormatProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFormatProperties {
  /// Optional: true,  Limit Type: [Bitmask]
  pub linearTilingFeatures: VkFormatFeatureFlags,
  /// Optional: true,  Limit Type: [Bitmask]
  pub optimalTilingFeatures: VkFormatFeatureFlags,
  /// Optional: true,  Limit Type: [Bitmask]
  pub bufferFeatures: VkFormatFeatureFlags,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkFormatProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkFormatProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkFormatProperties {
  pub const DEFAULT: Self = Self {
    linearTilingFeatures: VkFormatFeatureFlagBits(0),
    optimalTilingFeatures: VkFormatFeatureFlagBits(0),
    bufferFeatures: VkFormatFeatureFlagBits(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_linearTilingFeatures(mut self, val: VkFormatFeatureFlags) -> Self {
    self.linearTilingFeatures = val;
    self
  }
  #[inline]
  pub const fn with_optimalTilingFeatures(mut self, val: VkFormatFeatureFlags) -> Self {
    self.optimalTilingFeatures = val;
    self
  }
  #[inline]
  pub const fn with_bufferFeatures(mut self, val: VkFormatFeatureFlags) -> Self {
    self.bufferFeatures = val;
    self
  }
}
/// [VkImageFormatProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatProperties.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageFormatProperties {
  pub maxExtent: VkExtent3D,
  pub maxMipLevels: u32,
  pub maxArrayLayers: u32,
  /// Optional: true
  pub sampleCounts: VkSampleCountFlags,
  pub maxResourceSize: VkDeviceSize,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkImageFormatProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkImageFormatProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkImageFormatProperties {
  pub const DEFAULT: Self = Self {
    maxExtent: VkExtent3D::DEFAULT,
    maxMipLevels: 0,
    maxArrayLayers: 0,
    sampleCounts: VkSampleCountFlagBits(0),
    maxResourceSize: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_maxExtent(mut self, val: VkExtent3D) -> Self {
    self.maxExtent = val;
    self
  }
  #[inline]
  pub const fn with_maxMipLevels(mut self, val: u32) -> Self {
    self.maxMipLevels = val;
    self
  }
  #[inline]
  pub const fn with_maxArrayLayers(mut self, val: u32) -> Self {
    self.maxArrayLayers = val;
    self
  }
  #[inline]
  pub const fn with_sampleCounts(mut self, val: VkSampleCountFlags) -> Self {
    self.sampleCounts = val;
    self
  }
  #[inline]
  pub const fn with_maxResourceSize(mut self, val: VkDeviceSize) -> Self {
    self.maxResourceSize = val;
    self
  }
}
/// [VkBufferCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCreateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkBufferCreateFlags,
  pub size: VkDeviceSize,
  /// No Auto-Validity
  pub usage: VkBufferUsageFlags,
  pub sharingMode: VkSharingMode,
  /// Optional: true
  pub queueFamilyIndexCount: u32,
  /// Length: queueFamilyIndexCount,  No Auto-Validity
  pub pQueueFamilyIndices: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkBufferCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkBufferCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkBufferCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkBufferCreateFlagBits(0),
    size: 0,
    usage: VkBufferUsageFlagBits(0),
    sharingMode: VkSharingMode(0),
    queueFamilyIndexCount: 0,
    pQueueFamilyIndices: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkBufferCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[inline]
  pub const fn with_usage(mut self, val: VkBufferUsageFlags) -> Self {
    self.usage = val;
    self
  }
  #[inline]
  pub const fn with_sharingMode(mut self, val: VkSharingMode) -> Self {
    self.sharingMode = val;
    self
  }
  #[inline]
  pub const fn with_queueFamilyIndexCount(mut self, val: u32) -> Self {
    self.queueFamilyIndexCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pQueueFamilyIndices(mut self, val: &'a [u32]) -> Self {
    self.queueFamilyIndexCount = val.len() as u32;
    self.pQueueFamilyIndices = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkBufferCollectionBufferCreateInfoFUCHSIA<'child>(
    mut self,
    val: &'a VkBufferCollectionBufferCreateInfoFUCHSIA<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBufferCollectionBufferCreateInfoFUCHSIA<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_buffer_device_address")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkBufferDeviceAddressCreateInfoEXT<'child>(
    mut self,
    val: &'a VkBufferDeviceAddressCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBufferDeviceAddressCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkBufferOpaqueCaptureAddressCreateInfo<'child>(
    mut self,
    val: &'a VkBufferOpaqueCaptureAddressCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBufferOpaqueCaptureAddressCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkBufferUsageFlags2CreateInfo<'child>(
    mut self,
    val: &'a VkBufferUsageFlags2CreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBufferUsageFlags2CreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_dedicated_allocation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDedicatedAllocationBufferCreateInfoNV<'child>(
    mut self,
    val: &'a VkDedicatedAllocationBufferCreateInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDedicatedAllocationBufferCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalMemoryBufferCreateInfo<'child>(
    mut self,
    val: &'a VkExternalMemoryBufferCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalMemoryBufferCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkOpaqueCaptureDescriptorDataCreateInfoEXT<'child>(
    mut self,
    val: &'a VkOpaqueCaptureDescriptorDataCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkOpaqueCaptureDescriptorDataCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoProfileListInfoKHR<'child>(
    mut self,
    val: &'a VkVideoProfileListInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoProfileListInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferCreateInfo<
    'root,
    T: VkPNextExtends<VkBufferCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageSubresource](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSubresource.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageSubresource {
  pub aspectMask: VkImageAspectFlags,
  pub mipLevel: u32,
  pub arrayLayer: u32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkImageSubresource {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkImageSubresource {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkImageSubresource {
  pub const DEFAULT: Self = Self {
    aspectMask: VkImageAspectFlagBits(0),
    mipLevel: 0,
    arrayLayer: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_aspectMask(mut self, val: VkImageAspectFlags) -> Self {
    self.aspectMask = val;
    self
  }
  #[inline]
  pub const fn with_mipLevel(mut self, val: u32) -> Self {
    self.mipLevel = val;
    self
  }
  #[inline]
  pub const fn with_arrayLayer(mut self, val: u32) -> Self {
    self.arrayLayer = val;
    self
  }
}
/// [VkImageSubresourceLayers](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSubresourceLayers.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageSubresourceLayers {
  pub aspectMask: VkImageAspectFlags,
  pub mipLevel: u32,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkImageSubresourceLayers {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkImageSubresourceLayers {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkImageSubresourceLayers {
  pub const DEFAULT: Self = Self {
    aspectMask: VkImageAspectFlagBits(0),
    mipLevel: 0,
    baseArrayLayer: 0,
    layerCount: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_aspectMask(mut self, val: VkImageAspectFlags) -> Self {
    self.aspectMask = val;
    self
  }
  #[inline]
  pub const fn with_mipLevel(mut self, val: u32) -> Self {
    self.mipLevel = val;
    self
  }
  #[inline]
  pub const fn with_baseArrayLayer(mut self, val: u32) -> Self {
    self.baseArrayLayer = val;
    self
  }
  #[inline]
  pub const fn with_layerCount(mut self, val: u32) -> Self {
    self.layerCount = val;
    self
  }
}
/// [VkImageSubresourceRange](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSubresourceRange.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageSubresourceRange {
  pub aspectMask: VkImageAspectFlags,
  pub baseMipLevel: u32,
  pub levelCount: u32,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkImageSubresourceRange {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkImageSubresourceRange {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkImageSubresourceRange {
  pub const DEFAULT: Self = Self {
    aspectMask: VkImageAspectFlagBits(0),
    baseMipLevel: 0,
    levelCount: 0,
    baseArrayLayer: 0,
    layerCount: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_aspectMask(mut self, val: VkImageAspectFlags) -> Self {
    self.aspectMask = val;
    self
  }
  #[inline]
  pub const fn with_baseMipLevel(mut self, val: u32) -> Self {
    self.baseMipLevel = val;
    self
  }
  #[inline]
  pub const fn with_levelCount(mut self, val: u32) -> Self {
    self.levelCount = val;
    self
  }
  #[inline]
  pub const fn with_baseArrayLayer(mut self, val: u32) -> Self {
    self.baseArrayLayer = val;
    self
  }
  #[inline]
  pub const fn with_layerCount(mut self, val: u32) -> Self {
    self.layerCount = val;
    self
  }
}
/// [VkMemoryBarrier](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryBarrier.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `VkMemoryBarrier2`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryBarrier<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_BARRIER
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub srcAccessMask: VkAccessFlags,
  /// Optional: true
  pub dstAccessMask: VkAccessFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkMemoryBarrier<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkMemoryBarrier<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkMemoryBarrier<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_BARRIER,
    pNext: core::ptr::null(),
    srcAccessMask: VkAccessFlagBits(0),
    dstAccessMask: VkAccessFlagBits(0),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_srcAccessMask(mut self, val: VkAccessFlags) -> Self {
    self.srcAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_dstAccessMask(mut self, val: VkAccessFlags) -> Self {
    self.dstAccessMask = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryBarrier<
    'root,
    T: VkPNextExtends<VkMemoryBarrier<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBufferMemoryBarrier](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferMemoryBarrier.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `VkBufferMemoryBarrier2`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferMemoryBarrier<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// No Auto-Validity
  pub srcAccessMask: VkAccessFlags,
  /// No Auto-Validity
  pub dstAccessMask: VkAccessFlags,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkBufferMemoryBarrier<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkBufferMemoryBarrier<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkBufferMemoryBarrier<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_MEMORY_BARRIER,
    pNext: core::ptr::null(),
    srcAccessMask: VkAccessFlagBits(0),
    dstAccessMask: VkAccessFlagBits(0),
    srcQueueFamilyIndex: 0,
    dstQueueFamilyIndex: 0,
    buffer: VkBuffer::DEFAULT,
    offset: 0,
    size: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_srcAccessMask(mut self, val: VkAccessFlags) -> Self {
    self.srcAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_dstAccessMask(mut self, val: VkAccessFlags) -> Self {
    self.dstAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_srcQueueFamilyIndex(mut self, val: u32) -> Self {
    self.srcQueueFamilyIndex = val;
    self
  }
  #[inline]
  pub const fn with_dstQueueFamilyIndex(mut self, val: u32) -> Self {
    self.dstQueueFamilyIndex = val;
    self
  }
  #[inline]
  pub const fn with_buffer(mut self, val: VkBuffer) -> Self {
    self.buffer = val;
    self
  }
  #[inline]
  pub const fn with_offset(mut self, val: VkDeviceSize) -> Self {
    self.offset = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[cfg(feature = "VK_EXT_external_memory_acquire_unmodified")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalMemoryAcquireUnmodifiedEXT<'child>(
    mut self,
    val: &'a VkExternalMemoryAcquireUnmodifiedEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalMemoryAcquireUnmodifiedEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferMemoryBarrier<
    'root,
    T: VkPNextExtends<VkBufferMemoryBarrier<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageMemoryBarrier](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageMemoryBarrier.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `VkImageMemoryBarrier2`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageMemoryBarrier<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// No Auto-Validity
  pub srcAccessMask: VkAccessFlags,
  /// No Auto-Validity
  pub dstAccessMask: VkAccessFlags,
  pub oldLayout: VkImageLayout,
  pub newLayout: VkImageLayout,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub image: VkImage,
  pub subresourceRange: VkImageSubresourceRange,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkImageMemoryBarrier<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkImageMemoryBarrier<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkImageMemoryBarrier<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_MEMORY_BARRIER,
    pNext: core::ptr::null(),
    srcAccessMask: VkAccessFlagBits(0),
    dstAccessMask: VkAccessFlagBits(0),
    oldLayout: VkImageLayout(0),
    newLayout: VkImageLayout(0),
    srcQueueFamilyIndex: 0,
    dstQueueFamilyIndex: 0,
    image: VkImage::DEFAULT,
    subresourceRange: VkImageSubresourceRange::DEFAULT,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_srcAccessMask(mut self, val: VkAccessFlags) -> Self {
    self.srcAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_dstAccessMask(mut self, val: VkAccessFlags) -> Self {
    self.dstAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_oldLayout(mut self, val: VkImageLayout) -> Self {
    self.oldLayout = val;
    self
  }
  #[inline]
  pub const fn with_newLayout(mut self, val: VkImageLayout) -> Self {
    self.newLayout = val;
    self
  }
  #[inline]
  pub const fn with_srcQueueFamilyIndex(mut self, val: u32) -> Self {
    self.srcQueueFamilyIndex = val;
    self
  }
  #[inline]
  pub const fn with_dstQueueFamilyIndex(mut self, val: u32) -> Self {
    self.dstQueueFamilyIndex = val;
    self
  }
  #[inline]
  pub const fn with_image(mut self, val: VkImage) -> Self {
    self.image = val;
    self
  }
  #[inline]
  pub const fn with_subresourceRange(mut self, val: VkImageSubresourceRange) -> Self {
    self.subresourceRange = val;
    self
  }
  #[cfg(feature = "VK_EXT_external_memory_acquire_unmodified")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalMemoryAcquireUnmodifiedEXT<'child>(
    mut self,
    val: &'a VkExternalMemoryAcquireUnmodifiedEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalMemoryAcquireUnmodifiedEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_sample_locations")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSampleLocationsInfoEXT<'child>(
    mut self,
    val: &'a VkSampleLocationsInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkSampleLocationsInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageMemoryBarrier<
    'root,
    T: VkPNextExtends<VkImageMemoryBarrier<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkImageCreateFlags,
  pub imageType: VkImageType,
  pub format: VkFormat,
  pub extent: VkExtent3D,
  pub mipLevels: u32,
  pub arrayLayers: u32,
  pub samples: VkSampleCountFlagBits,
  pub tiling: VkImageTiling,
  pub usage: VkImageUsageFlags,
  pub sharingMode: VkSharingMode,
  /// Optional: true
  pub queueFamilyIndexCount: u32,
  /// Length: queueFamilyIndexCount,  No Auto-Validity
  pub pQueueFamilyIndices: *const u32,
  pub initialLayout: VkImageLayout,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkImageCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkImageCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkImageCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkImageCreateFlagBits(0),
    imageType: VkImageType(0),
    format: VkFormat(0),
    extent: VkExtent3D::DEFAULT,
    mipLevels: 0,
    arrayLayers: 0,
    samples: VkSampleCountFlagBits(0),
    tiling: VkImageTiling(0),
    usage: VkImageUsageFlagBits(0),
    sharingMode: VkSharingMode(0),
    queueFamilyIndexCount: 0,
    pQueueFamilyIndices: core::ptr::null(),
    initialLayout: VkImageLayout(0),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkImageCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_imageType(mut self, val: VkImageType) -> Self {
    self.imageType = val;
    self
  }
  #[inline]
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[inline]
  pub const fn with_extent(mut self, val: VkExtent3D) -> Self {
    self.extent = val;
    self
  }
  #[inline]
  pub const fn with_mipLevels(mut self, val: u32) -> Self {
    self.mipLevels = val;
    self
  }
  #[inline]
  pub const fn with_arrayLayers(mut self, val: u32) -> Self {
    self.arrayLayers = val;
    self
  }
  #[inline]
  pub const fn with_samples(mut self, val: VkSampleCountFlagBits) -> Self {
    self.samples = val;
    self
  }
  #[inline]
  pub const fn with_tiling(mut self, val: VkImageTiling) -> Self {
    self.tiling = val;
    self
  }
  #[inline]
  pub const fn with_usage(mut self, val: VkImageUsageFlags) -> Self {
    self.usage = val;
    self
  }
  #[inline]
  pub const fn with_sharingMode(mut self, val: VkSharingMode) -> Self {
    self.sharingMode = val;
    self
  }
  #[inline]
  pub const fn with_queueFamilyIndexCount(mut self, val: u32) -> Self {
    self.queueFamilyIndexCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pQueueFamilyIndices(mut self, val: &'a [u32]) -> Self {
    self.queueFamilyIndexCount = val.len() as u32;
    self.pQueueFamilyIndices = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_initialLayout(mut self, val: VkImageLayout) -> Self {
    self.initialLayout = val;
    self
  }
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkBufferCollectionImageCreateInfoFUCHSIA<'child>(
    mut self,
    val: &'a VkBufferCollectionImageCreateInfoFUCHSIA<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBufferCollectionImageCreateInfoFUCHSIA<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphOpticalFlowImageFormatInfoARM<'child>(
    mut self,
    val: &'a VkDataGraphOpticalFlowImageFormatInfoARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDataGraphOpticalFlowImageFormatInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_dedicated_allocation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDedicatedAllocationImageCreateInfoNV<'child>(
    mut self,
    val: &'a VkDedicatedAllocationImageCreateInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDedicatedAllocationImageCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_metal_objects")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportMetalObjectCreateInfoEXT<'child>(
    mut self,
    val: &'a VkExportMetalObjectCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportMetalObjectCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalFormatANDROID<'child>(
    mut self,
    val: &'a VkExternalFormatANDROID<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalFormatANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_OHOS_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalFormatOHOS<'child>(
    mut self,
    val: &'a VkExternalFormatOHOS<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalFormatOHOS<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalFormatQNX<'child>(
    mut self,
    val: &'a VkExternalFormatQNX<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalFormatQNX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalMemoryImageCreateInfo<'child>(
    mut self,
    val: &'a VkExternalMemoryImageCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalMemoryImageCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalMemoryImageCreateInfoNV<'child>(
    mut self,
    val: &'a VkExternalMemoryImageCreateInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalMemoryImageCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_MESA_image_alignment_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageAlignmentControlCreateInfoMESA<'child>(
    mut self,
    val: &'a VkImageAlignmentControlCreateInfoMESA<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageAlignmentControlCreateInfoMESA<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_compression_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageCompressionControlEXT<'child>(
    mut self,
    val: &'a VkImageCompressionControlEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageCompressionControlEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_extended_flags")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageCreateFlags2CreateInfoKHR<'child>(
    mut self,
    val: &'a VkImageCreateFlags2CreateInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageCreateFlags2CreateInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageDrmFormatModifierExplicitCreateInfoEXT<'child>(
    mut self,
    val: &'a VkImageDrmFormatModifierExplicitCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkImageDrmFormatModifierExplicitCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageDrmFormatModifierListCreateInfoEXT<'child>(
    mut self,
    val: &'a VkImageDrmFormatModifierListCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageDrmFormatModifierListCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageFormatListCreateInfo<'child>(
    mut self,
    val: &'a VkImageFormatListCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageFormatListCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(any(
    all(feature = "VK_KHR_extended_flags", feature = "VK_VERSION_1_2"),
    all(
      feature = "VK_EXT_separate_stencil_usage",
      feature = "VK_KHR_extended_flags"
    )
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageStencilUsage2CreateInfoKHR<'child>(
    mut self,
    val: &'a VkImageStencilUsage2CreateInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageStencilUsage2CreateInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageStencilUsageCreateInfo<'child>(
    mut self,
    val: &'a VkImageStencilUsageCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageStencilUsageCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(any(
    all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
    all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain")
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageSwapchainCreateInfoKHR<'child>(
    mut self,
    val: &'a VkImageSwapchainCreateInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageSwapchainCreateInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_tiling_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageTilingControlCreateInfoEXT<'child>(
    mut self,
    val: &'a VkImageTilingControlCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageTilingControlCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_extended_flags")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageUsageFlags2CreateInfoKHR<'child>(
    mut self,
    val: &'a VkImageUsageFlags2CreateInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageUsageFlags2CreateInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_metal_objects")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMetalIOSurfaceInfoEXT<'child>(
    mut self,
    val: &'a VkImportMetalIOSurfaceInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMetalIOSurfaceInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_metal_objects")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMetalTextureInfoEXT<'child>(
    mut self,
    val: &'a VkImportMetalTextureInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMetalTextureInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkOpaqueCaptureDataCreateInfoEXT<'child>(
    mut self,
    val: &'a VkOpaqueCaptureDataCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkOpaqueCaptureDataCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkOpaqueCaptureDescriptorDataCreateInfoEXT<'child>(
    mut self,
    val: &'a VkOpaqueCaptureDescriptorDataCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkOpaqueCaptureDescriptorDataCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkOpticalFlowImageFormatInfoNV<'child>(
    mut self,
    val: &'a VkOpticalFlowImageFormatInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkOpticalFlowImageFormatInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoProfileListInfoKHR<'child>(
    mut self,
    val: &'a VkVideoProfileListInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoProfileListInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageCreateInfo<
    'root,
    T: VkPNextExtends<VkImageCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSubresourceLayout](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubresourceLayout.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubresourceLayout {
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  pub rowPitch: VkDeviceSize,
  pub arrayPitch: VkDeviceSize,
  pub depthPitch: VkDeviceSize,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkSubresourceLayout {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkSubresourceLayout {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkSubresourceLayout {
  pub const DEFAULT: Self = Self {
    offset: 0,
    size: 0,
    rowPitch: 0,
    arrayPitch: 0,
    depthPitch: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_offset(mut self, val: VkDeviceSize) -> Self {
    self.offset = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[inline]
  pub const fn with_rowPitch(mut self, val: VkDeviceSize) -> Self {
    self.rowPitch = val;
    self
  }
  #[inline]
  pub const fn with_arrayPitch(mut self, val: VkDeviceSize) -> Self {
    self.arrayPitch = val;
    self
  }
  #[inline]
  pub const fn with_depthPitch(mut self, val: VkDeviceSize) -> Self {
    self.depthPitch = val;
    self
  }
}
/// [VkImageViewCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCreateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageViewCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkImageViewCreateFlags,
  pub image: VkImage,
  pub viewType: VkImageViewType,
  pub format: VkFormat,
  pub components: VkComponentMapping,
  pub subresourceRange: VkImageSubresourceRange,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkImageViewCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkImageViewCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkImageViewCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_VIEW_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkImageViewCreateFlagBits(0),
    image: VkImage::DEFAULT,
    viewType: VkImageViewType(0),
    format: VkFormat(0),
    components: VkComponentMapping::DEFAULT,
    subresourceRange: VkImageSubresourceRange::DEFAULT,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkImageViewCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_image(mut self, val: VkImage) -> Self {
    self.image = val;
    self
  }
  #[inline]
  pub const fn with_viewType(mut self, val: VkImageViewType) -> Self {
    self.viewType = val;
    self
  }
  #[inline]
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[inline]
  pub const fn with_components(mut self, val: VkComponentMapping) -> Self {
    self.components = val;
    self
  }
  #[inline]
  pub const fn with_subresourceRange(mut self, val: VkImageSubresourceRange) -> Self {
    self.subresourceRange = val;
    self
  }
  #[cfg(feature = "VK_EXT_metal_objects")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportMetalObjectCreateInfoEXT<'child>(
    mut self,
    val: &'a VkExportMetalObjectCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportMetalObjectCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_astc_decode_mode")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageViewASTCDecodeModeEXT<'child>(
    mut self,
    val: &'a VkImageViewASTCDecodeModeEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageViewASTCDecodeModeEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_view_min_lod")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageViewMinLodCreateInfoEXT<'child>(
    mut self,
    val: &'a VkImageViewMinLodCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageViewMinLodCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_image_processing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageViewSampleWeightCreateInfoQCOM<'child>(
    mut self,
    val: &'a VkImageViewSampleWeightCreateInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageViewSampleWeightCreateInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_sliced_view_of_3d")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageViewSlicedCreateInfoEXT<'child>(
    mut self,
    val: &'a VkImageViewSlicedCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageViewSlicedCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_extended_flags")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageViewUsage2CreateInfoKHR<'child>(
    mut self,
    val: &'a VkImageViewUsage2CreateInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageViewUsage2CreateInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageViewUsageCreateInfo<'child>(
    mut self,
    val: &'a VkImageViewUsageCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImageViewUsageCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkOpaqueCaptureDescriptorDataCreateInfoEXT<'child>(
    mut self,
    val: &'a VkOpaqueCaptureDescriptorDataCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkOpaqueCaptureDescriptorDataCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSamplerYcbcrConversionInfo<'child>(
    mut self,
    val: &'a VkSamplerYcbcrConversionInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkSamplerYcbcrConversionInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageViewCreateInfo<
    'root,
    T: VkPNextExtends<VkImageViewCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBufferCopy](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCopy.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferCopy {
  pub srcOffset: VkDeviceSize,
  pub dstOffset: VkDeviceSize,
  /// No Auto-Validity
  pub size: VkDeviceSize,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkBufferCopy {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkBufferCopy {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkBufferCopy {
  pub const DEFAULT: Self = Self {
    srcOffset: 0,
    dstOffset: 0,
    size: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_srcOffset(mut self, val: VkDeviceSize) -> Self {
    self.srcOffset = val;
    self
  }
  #[inline]
  pub const fn with_dstOffset(mut self, val: VkDeviceSize) -> Self {
    self.dstOffset = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
}
/// [VkSparseMemoryBind](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseMemoryBind.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSparseMemoryBind {
  pub resourceOffset: VkDeviceSize,
  pub size: VkDeviceSize,
  /// Optional: true
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  /// Optional: true
  pub flags: VkSparseMemoryBindFlags,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Send for VkSparseMemoryBind {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Sync for VkSparseMemoryBind {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
impl VkSparseMemoryBind {
  pub const DEFAULT: Self = Self {
    resourceOffset: 0,
    size: 0,
    memory: VkDeviceMemory::DEFAULT,
    memoryOffset: 0,
    flags: VkSparseMemoryBindFlagBits(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_resourceOffset(mut self, val: VkDeviceSize) -> Self {
    self.resourceOffset = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[inline]
  pub const fn with_memory(mut self, val: VkDeviceMemory) -> Self {
    self.memory = val;
    self
  }
  #[inline]
  pub const fn with_memoryOffset(mut self, val: VkDeviceSize) -> Self {
    self.memoryOffset = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkSparseMemoryBindFlags) -> Self {
    self.flags = val;
    self
  }
}
/// [VkSparseImageMemoryBind](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageMemoryBind.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSparseImageMemoryBind {
  pub subresource: VkImageSubresource,
  pub offset: VkOffset3D,
  pub extent: VkExtent3D,
  /// Optional: true
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  /// Optional: true
  pub flags: VkSparseMemoryBindFlags,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Send for VkSparseImageMemoryBind {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl Sync for VkSparseImageMemoryBind {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
impl VkSparseImageMemoryBind {
  pub const DEFAULT: Self = Self {
    subresource: VkImageSubresource::DEFAULT,
    offset: VkOffset3D::DEFAULT,
    extent: VkExtent3D::DEFAULT,
    memory: VkDeviceMemory::DEFAULT,
    memoryOffset: 0,
    flags: VkSparseMemoryBindFlagBits(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_subresource(mut self, val: VkImageSubresource) -> Self {
    self.subresource = val;
    self
  }
  #[inline]
  pub const fn with_offset(mut self, val: VkOffset3D) -> Self {
    self.offset = val;
    self
  }
  #[inline]
  pub const fn with_extent(mut self, val: VkExtent3D) -> Self {
    self.extent = val;
    self
  }
  #[inline]
  pub const fn with_memory(mut self, val: VkDeviceMemory) -> Self {
    self.memory = val;
    self
  }
  #[inline]
  pub const fn with_memoryOffset(mut self, val: VkDeviceSize) -> Self {
    self.memoryOffset = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkSparseMemoryBindFlags) -> Self {
    self.flags = val;
    self
  }
}
/// [VkSparseBufferMemoryBindInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseBufferMemoryBindInfo.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSparseBufferMemoryBindInfo<'a> {
  pub buffer: VkBuffer,
  pub bindCount: u32,
  /// Length: bindCount
  pub pBinds: *const VkSparseMemoryBind,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkSparseBufferMemoryBindInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkSparseBufferMemoryBindInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkSparseBufferMemoryBindInfo<'a> {
  pub const DEFAULT: Self = Self {
    buffer: VkBuffer::DEFAULT,
    bindCount: 0,
    pBinds: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_buffer(mut self, val: VkBuffer) -> Self {
    self.buffer = val;
    self
  }
  #[inline]
  pub const fn with_bindCount(mut self, val: u32) -> Self {
    self.bindCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pBinds(mut self, val: &'a [VkSparseMemoryBind]) -> Self {
    self.bindCount = val.len() as u32;
    self.pBinds = val.as_ptr();
    self
  }
}
/// [VkSparseImageOpaqueMemoryBindInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageOpaqueMemoryBindInfo.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSparseImageOpaqueMemoryBindInfo<'a> {
  pub image: VkImage,
  pub bindCount: u32,
  /// Length: bindCount
  pub pBinds: *const VkSparseMemoryBind,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkSparseImageOpaqueMemoryBindInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkSparseImageOpaqueMemoryBindInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkSparseImageOpaqueMemoryBindInfo<'a> {
  pub const DEFAULT: Self = Self {
    image: VkImage::DEFAULT,
    bindCount: 0,
    pBinds: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_image(mut self, val: VkImage) -> Self {
    self.image = val;
    self
  }
  #[inline]
  pub const fn with_bindCount(mut self, val: u32) -> Self {
    self.bindCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pBinds(mut self, val: &'a [VkSparseMemoryBind]) -> Self {
    self.bindCount = val.len() as u32;
    self.pBinds = val.as_ptr();
    self
  }
}
/// [VkSparseImageMemoryBindInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageMemoryBindInfo.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSparseImageMemoryBindInfo<'a> {
  pub image: VkImage,
  pub bindCount: u32,
  /// Length: bindCount
  pub pBinds: *const VkSparseImageMemoryBind,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkSparseImageMemoryBindInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkSparseImageMemoryBindInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkSparseImageMemoryBindInfo<'a> {
  pub const DEFAULT: Self = Self {
    image: VkImage::DEFAULT,
    bindCount: 0,
    pBinds: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_image(mut self, val: VkImage) -> Self {
    self.image = val;
    self
  }
  #[inline]
  pub const fn with_bindCount(mut self, val: u32) -> Self {
    self.bindCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pBinds(mut self, val: &'a [VkSparseImageMemoryBind]) -> Self {
    self.bindCount = val.len() as u32;
    self.pBinds = val.as_ptr();
    self
  }
}
/// [VkBindSparseInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindSparseInfo.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindSparseInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BIND_SPARSE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub waitSemaphoreCount: u32,
  /// Length: waitSemaphoreCount
  pub pWaitSemaphores: *const VkSemaphore,
  /// Optional: true
  pub bufferBindCount: u32,
  /// Length: bufferBindCount
  pub pBufferBinds: *const VkSparseBufferMemoryBindInfo<'a>,
  /// Optional: true
  pub imageOpaqueBindCount: u32,
  /// Length: imageOpaqueBindCount
  pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo<'a>,
  /// Optional: true
  pub imageBindCount: u32,
  /// Length: imageBindCount
  pub pImageBinds: *const VkSparseImageMemoryBindInfo<'a>,
  /// Optional: true
  pub signalSemaphoreCount: u32,
  /// Length: signalSemaphoreCount
  pub pSignalSemaphores: *const VkSemaphore,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkBindSparseInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkBindSparseInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkBindSparseInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BIND_SPARSE_INFO,
    pNext: core::ptr::null(),
    waitSemaphoreCount: 0,
    pWaitSemaphores: core::ptr::null(),
    bufferBindCount: 0,
    pBufferBinds: core::ptr::null(),
    imageOpaqueBindCount: 0,
    pImageOpaqueBinds: core::ptr::null(),
    imageBindCount: 0,
    pImageBinds: core::ptr::null(),
    signalSemaphoreCount: 0,
    pSignalSemaphores: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_waitSemaphoreCount(mut self, val: u32) -> Self {
    self.waitSemaphoreCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pWaitSemaphores(mut self, val: &'a [VkSemaphore]) -> Self {
    self.waitSemaphoreCount = val.len() as u32;
    self.pWaitSemaphores = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_bufferBindCount(mut self, val: u32) -> Self {
    self.bufferBindCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pBufferBinds(mut self, val: &'a [VkSparseBufferMemoryBindInfo<'a>]) -> Self {
    self.bufferBindCount = val.len() as u32;
    self.pBufferBinds = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_imageOpaqueBindCount(mut self, val: u32) -> Self {
    self.imageOpaqueBindCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pImageOpaqueBinds(
    mut self,
    val: &'a [VkSparseImageOpaqueMemoryBindInfo<'a>],
  ) -> Self {
    self.imageOpaqueBindCount = val.len() as u32;
    self.pImageOpaqueBinds = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_imageBindCount(mut self, val: u32) -> Self {
    self.imageBindCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pImageBinds(mut self, val: &'a [VkSparseImageMemoryBindInfo<'a>]) -> Self {
    self.imageBindCount = val.len() as u32;
    self.pImageBinds = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_signalSemaphoreCount(mut self, val: u32) -> Self {
    self.signalSemaphoreCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSignalSemaphores(mut self, val: &'a [VkSemaphore]) -> Self {
    self.signalSemaphoreCount = val.len() as u32;
    self.pSignalSemaphores = val.as_ptr();
    self
  }
  #[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceGroupBindSparseInfo<'child>(
    mut self,
    val: &'a VkDeviceGroupBindSparseInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDeviceGroupBindSparseInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_frame_boundary")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFrameBoundaryEXT<'child>(
    mut self,
    val: &'a VkFrameBoundaryEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkFrameBoundaryEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFrameBoundaryTensorsARM<'child>(
    mut self,
    val: &'a VkFrameBoundaryTensorsARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkFrameBoundaryTensorsARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTimelineSemaphoreSubmitInfo<'child>(
    mut self,
    val: &'a VkTimelineSemaphoreSubmitInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkTimelineSemaphoreSubmitInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBindSparseInfo<
    'root,
    T: VkPNextExtends<VkBindSparseInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageCopy](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCopy.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageCopy {
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffset: VkOffset3D,
  pub extent: VkExtent3D,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkImageCopy {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkImageCopy {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkImageCopy {
  pub const DEFAULT: Self = Self {
    srcSubresource: VkImageSubresourceLayers::DEFAULT,
    srcOffset: VkOffset3D::DEFAULT,
    dstSubresource: VkImageSubresourceLayers::DEFAULT,
    dstOffset: VkOffset3D::DEFAULT,
    extent: VkExtent3D::DEFAULT,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_srcSubresource(mut self, val: VkImageSubresourceLayers) -> Self {
    self.srcSubresource = val;
    self
  }
  #[inline]
  pub const fn with_srcOffset(mut self, val: VkOffset3D) -> Self {
    self.srcOffset = val;
    self
  }
  #[inline]
  pub const fn with_dstSubresource(mut self, val: VkImageSubresourceLayers) -> Self {
    self.dstSubresource = val;
    self
  }
  #[inline]
  pub const fn with_dstOffset(mut self, val: VkOffset3D) -> Self {
    self.dstOffset = val;
    self
  }
  #[inline]
  pub const fn with_extent(mut self, val: VkExtent3D) -> Self {
    self.extent = val;
    self
  }
}
/// [VkBufferImageCopy](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferImageCopy.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferImageCopy {
  pub bufferOffset: VkDeviceSize,
  pub bufferRowLength: u32,
  pub bufferImageHeight: u32,
  pub imageSubresource: VkImageSubresourceLayers,
  pub imageOffset: VkOffset3D,
  pub imageExtent: VkExtent3D,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkBufferImageCopy {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkBufferImageCopy {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkBufferImageCopy {
  pub const DEFAULT: Self = Self {
    bufferOffset: 0,
    bufferRowLength: 0,
    bufferImageHeight: 0,
    imageSubresource: VkImageSubresourceLayers::DEFAULT,
    imageOffset: VkOffset3D::DEFAULT,
    imageExtent: VkExtent3D::DEFAULT,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_bufferOffset(mut self, val: VkDeviceSize) -> Self {
    self.bufferOffset = val;
    self
  }
  #[inline]
  pub const fn with_bufferRowLength(mut self, val: u32) -> Self {
    self.bufferRowLength = val;
    self
  }
  #[inline]
  pub const fn with_bufferImageHeight(mut self, val: u32) -> Self {
    self.bufferImageHeight = val;
    self
  }
  #[inline]
  pub const fn with_imageSubresource(mut self, val: VkImageSubresourceLayers) -> Self {
    self.imageSubresource = val;
    self
  }
  #[inline]
  pub const fn with_imageOffset(mut self, val: VkOffset3D) -> Self {
    self.imageOffset = val;
    self
  }
  #[inline]
  pub const fn with_imageExtent(mut self, val: VkExtent3D) -> Self {
    self.imageExtent = val;
    self
  }
}
/// [VkCommandPoolCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolCreateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandPoolCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkCommandPoolCreateFlags,
  pub queueFamilyIndex: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkCommandPoolCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkCommandPoolCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkCommandPoolCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMMAND_POOL_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkCommandPoolCreateFlagBits(0),
    queueFamilyIndex: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkCommandPoolCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_queueFamilyIndex(mut self, val: u32) -> Self {
    self.queueFamilyIndex = val;
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkCommandPoolMemoryReservationCreateInfo<'child>(
    mut self,
    val: &'a VkCommandPoolMemoryReservationCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkCommandPoolMemoryReservationCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphProcessingEngineCreateInfoARM<'child>(
    mut self,
    val: &'a VkDataGraphProcessingEngineCreateInfoARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDataGraphProcessingEngineCreateInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCommandPoolCreateInfo<
    'root,
    T: VkPNextExtends<VkCommandPoolCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCommandBufferAllocateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferAllocateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandBufferAllocateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub commandPool: VkCommandPool,
  pub level: VkCommandBufferLevel,
  pub commandBufferCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkCommandBufferAllocateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkCommandBufferAllocateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkCommandBufferAllocateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMMAND_BUFFER_ALLOCATE_INFO,
    pNext: core::ptr::null(),
    commandPool: VkCommandPool::DEFAULT,
    level: VkCommandBufferLevel(0),
    commandBufferCount: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_commandPool(mut self, val: VkCommandPool) -> Self {
    self.commandPool = val;
    self
  }
  #[inline]
  pub const fn with_level(mut self, val: VkCommandBufferLevel) -> Self {
    self.level = val;
    self
  }
  #[inline]
  pub const fn with_commandBufferCount(mut self, val: u32) -> Self {
    self.commandBufferCount = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCommandBufferAllocateInfo<
    'root,
    T: VkPNextExtends<VkCommandBufferAllocateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCommandBufferInheritanceInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferInheritanceInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandBufferInheritanceInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// Optional: true,  No Auto-Validity
  pub renderPass: VkRenderPass,
  #[cfg(not(feature = "VK_GRAPHICS_VERSION_1_0"))]
  /// Optional: true,  No Auto-Validity
  pub renderPass: *mut c_void,
  pub subpass: u32,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// Optional: true,  No Auto-Validity
  pub framebuffer: VkFramebuffer,
  #[cfg(not(feature = "VK_GRAPHICS_VERSION_1_0"))]
  /// Optional: true,  No Auto-Validity
  pub framebuffer: *mut c_void,
  pub occlusionQueryEnable: VkBool32,
  /// Optional: true,  No Auto-Validity
  pub queryFlags: VkQueryControlFlags,
  /// Optional: true,  No Auto-Validity
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkCommandBufferInheritanceInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkCommandBufferInheritanceInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkCommandBufferInheritanceInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMMAND_BUFFER_INHERITANCE_INFO,
    pNext: core::ptr::null(),
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    renderPass: VkRenderPass::DEFAULT,
    #[cfg(not(feature = "VK_GRAPHICS_VERSION_1_0"))]
    renderPass: core::ptr::null_mut(),
    subpass: 0,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    framebuffer: VkFramebuffer::DEFAULT,
    #[cfg(not(feature = "VK_GRAPHICS_VERSION_1_0"))]
    framebuffer: core::ptr::null_mut(),
    occlusionQueryEnable: 0,
    queryFlags: VkQueryControlFlagBits(0),
    pipelineStatistics: VkQueryPipelineStatisticFlagBits(0),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  #[inline]
  pub const fn with_renderPass(mut self, val: VkRenderPass) -> Self {
    self.renderPass = val;
    self
  }
  #[inline]
  pub const fn with_subpass(mut self, val: u32) -> Self {
    self.subpass = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  #[inline]
  pub const fn with_framebuffer(mut self, val: VkFramebuffer) -> Self {
    self.framebuffer = val;
    self
  }
  #[inline]
  pub const fn with_occlusionQueryEnable(mut self, val: VkBool32) -> Self {
    self.occlusionQueryEnable = val;
    self
  }
  #[inline]
  pub const fn with_queryFlags(mut self, val: VkQueryControlFlags) -> Self {
    self.queryFlags = val;
    self
  }
  #[inline]
  pub const fn with_pipelineStatistics(mut self, val: VkQueryPipelineStatisticFlags) -> Self {
    self.pipelineStatistics = val;
    self
  }
  #[cfg(any(
    all(
      feature = "VK_AMD_mixed_attachment_samples",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_AMD_mixed_attachment_samples",
      feature = "VK_KHR_dynamic_rendering"
    )
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkAttachmentSampleCountInfoAMD<'child>(
    mut self,
    val: &'a VkAttachmentSampleCountInfoAMD<'child>,
  ) -> Self {
    self.pNext = (val as *const VkAttachmentSampleCountInfoAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_conditional_rendering")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkCommandBufferInheritanceConditionalRenderingInfoEXT<'child>(
    mut self,
    val: &'a VkCommandBufferInheritanceConditionalRenderingInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkCommandBufferInheritanceConditionalRenderingInfoEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkCommandBufferInheritanceDescriptorHeapInfoEXT<'child>(
    mut self,
    val: &'a VkCommandBufferInheritanceDescriptorHeapInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkCommandBufferInheritanceDescriptorHeapInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_render_pass_transform")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkCommandBufferInheritanceRenderPassTransformInfoQCOM<'child>(
    mut self,
    val: &'a VkCommandBufferInheritanceRenderPassTransformInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkCommandBufferInheritanceRenderPassTransformInfoQCOM<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkCommandBufferInheritanceRenderingInfo<'child>(
    mut self,
    val: &'a VkCommandBufferInheritanceRenderingInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkCommandBufferInheritanceRenderingInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_inherited_viewport_scissor")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkCommandBufferInheritanceViewportScissorInfoNV<'child>(
    mut self,
    val: &'a VkCommandBufferInheritanceViewportScissorInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkCommandBufferInheritanceViewportScissorInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(any(
    all(
      feature = "VK_EXT_custom_resolve",
      feature = "VK_KHR_dynamic_rendering"
    ),
    all(feature = "VK_EXT_custom_resolve", feature = "VK_VERSION_1_3")
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkCustomResolveCreateInfoEXT<'child>(
    mut self,
    val: &'a VkCustomResolveCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkCustomResolveCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalFormatANDROID<'child>(
    mut self,
    val: &'a VkExternalFormatANDROID<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalFormatANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_OHOS_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalFormatOHOS<'child>(
    mut self,
    val: &'a VkExternalFormatOHOS<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalFormatOHOS<'child>).cast::<c_void>();
    self
  }
  #[cfg(any(
    all(
      feature = "VK_NVX_multiview_per_view_attributes",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_KHR_dynamic_rendering",
      feature = "VK_NVX_multiview_per_view_attributes"
    )
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMultiviewPerViewAttributesInfoNVX<'child>(
    mut self,
    val: &'a VkMultiviewPerViewAttributesInfoNVX<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMultiviewPerViewAttributesInfoNVX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassTileShadingCreateInfoQCOM<'child>(
    mut self,
    val: &'a VkRenderPassTileShadingCreateInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassTileShadingCreateInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderingAttachmentLocationInfo<'child>(
    mut self,
    val: &'a VkRenderingAttachmentLocationInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderingAttachmentLocationInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderingInputAttachmentIndexInfo<'child>(
    mut self,
    val: &'a VkRenderingInputAttachmentIndexInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderingInputAttachmentIndexInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_memory_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTileMemoryBindInfoQCOM<'child>(
    mut self,
    val: &'a VkTileMemoryBindInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkTileMemoryBindInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCommandBufferInheritanceInfo<
    'root,
    T: VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCommandBufferBeginInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferBeginInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandBufferBeginInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkCommandBufferUsageFlags,
  /// Optional: true,  No Auto-Validity
  pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkCommandBufferBeginInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkCommandBufferBeginInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkCommandBufferBeginInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMMAND_BUFFER_BEGIN_INFO,
    pNext: core::ptr::null(),
    flags: VkCommandBufferUsageFlagBits(0),
    pInheritanceInfo: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkCommandBufferUsageFlags) -> Self {
    self.flags = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pInheritanceInfo(
    mut self,
    val: *const VkCommandBufferInheritanceInfo<'a>,
  ) -> Self {
    self.pInheritanceInfo = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceGroupCommandBufferBeginInfo<'child>(
    mut self,
    val: &'a VkDeviceGroupCommandBufferBeginInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDeviceGroupCommandBufferBeginInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCommandBufferBeginInfo<
    'root,
    T: VkPNextExtends<VkCommandBufferBeginInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkFenceCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceCreateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFenceCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkFenceCreateFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkFenceCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkFenceCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkFenceCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::FENCE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkFenceCreateFlagBits(0),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkFenceCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportFenceCreateInfo<'child>(
    mut self,
    val: &'a VkExportFenceCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportFenceCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportFenceSciSyncInfoNV<'child>(
    mut self,
    val: &'a VkExportFenceSciSyncInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportFenceSciSyncInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_external_fence_win32")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportFenceWin32HandleInfoKHR<'child>(
    mut self,
    val: &'a VkExportFenceWin32HandleInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportFenceWin32HandleInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkFenceCreateInfo<
    'root,
    T: VkPNextExtends<VkFenceCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFeatures.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceFeatures {
  /// **Removal note:** `VK_KHR_portability_subset` removes this from its mandatory feature set. The field remains queryable and may still be supported.
  pub robustBufferAccess: VkBool32,
  pub fullDrawIndexUint32: VkBool32,
  pub imageCubeArray: VkBool32,
  pub independentBlend: VkBool32,
  pub geometryShader: VkBool32,
  pub tessellationShader: VkBool32,
  pub sampleRateShading: VkBool32,
  pub dualSrcBlend: VkBool32,
  pub logicOp: VkBool32,
  pub multiDrawIndirect: VkBool32,
  pub drawIndirectFirstInstance: VkBool32,
  pub depthClamp: VkBool32,
  pub depthBiasClamp: VkBool32,
  pub fillModeNonSolid: VkBool32,
  pub depthBounds: VkBool32,
  pub wideLines: VkBool32,
  pub largePoints: VkBool32,
  pub alphaToOne: VkBool32,
  pub multiViewport: VkBool32,
  pub samplerAnisotropy: VkBool32,
  pub textureCompressionETC2: VkBool32,
  pub textureCompressionASTC_LDR: VkBool32,
  pub textureCompressionBC: VkBool32,
  pub occlusionQueryPrecise: VkBool32,
  pub pipelineStatisticsQuery: VkBool32,
  pub vertexPipelineStoresAndAtomics: VkBool32,
  pub fragmentStoresAndAtomics: VkBool32,
  pub shaderTessellationAndGeometryPointSize: VkBool32,
  pub shaderImageGatherExtended: VkBool32,
  pub shaderStorageImageExtendedFormats: VkBool32,
  pub shaderStorageImageMultisample: VkBool32,
  pub shaderStorageImageReadWithoutFormat: VkBool32,
  pub shaderStorageImageWriteWithoutFormat: VkBool32,
  pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
  pub shaderSampledImageArrayDynamicIndexing: VkBool32,
  pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
  pub shaderStorageImageArrayDynamicIndexing: VkBool32,
  pub shaderClipDistance: VkBool32,
  pub shaderCullDistance: VkBool32,
  pub shaderFloat64: VkBool32,
  pub shaderInt64: VkBool32,
  pub shaderInt16: VkBool32,
  pub shaderResourceResidency: VkBool32,
  pub shaderResourceMinLod: VkBool32,
  pub sparseBinding: VkBool32,
  pub sparseResidencyBuffer: VkBool32,
  pub sparseResidencyImage2D: VkBool32,
  pub sparseResidencyImage3D: VkBool32,
  pub sparseResidency2Samples: VkBool32,
  pub sparseResidency4Samples: VkBool32,
  pub sparseResidency8Samples: VkBool32,
  pub sparseResidency16Samples: VkBool32,
  pub sparseResidencyAliased: VkBool32,
  pub variableMultisampleRate: VkBool32,
  pub inheritedQueries: VkBool32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkPhysicalDeviceFeatures {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkPhysicalDeviceFeatures {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkPhysicalDeviceFeatures {
  pub const DEFAULT: Self = Self {
    robustBufferAccess: 0,
    fullDrawIndexUint32: 0,
    imageCubeArray: 0,
    independentBlend: 0,
    geometryShader: 0,
    tessellationShader: 0,
    sampleRateShading: 0,
    dualSrcBlend: 0,
    logicOp: 0,
    multiDrawIndirect: 0,
    drawIndirectFirstInstance: 0,
    depthClamp: 0,
    depthBiasClamp: 0,
    fillModeNonSolid: 0,
    depthBounds: 0,
    wideLines: 0,
    largePoints: 0,
    alphaToOne: 0,
    multiViewport: 0,
    samplerAnisotropy: 0,
    textureCompressionETC2: 0,
    textureCompressionASTC_LDR: 0,
    textureCompressionBC: 0,
    occlusionQueryPrecise: 0,
    pipelineStatisticsQuery: 0,
    vertexPipelineStoresAndAtomics: 0,
    fragmentStoresAndAtomics: 0,
    shaderTessellationAndGeometryPointSize: 0,
    shaderImageGatherExtended: 0,
    shaderStorageImageExtendedFormats: 0,
    shaderStorageImageMultisample: 0,
    shaderStorageImageReadWithoutFormat: 0,
    shaderStorageImageWriteWithoutFormat: 0,
    shaderUniformBufferArrayDynamicIndexing: 0,
    shaderSampledImageArrayDynamicIndexing: 0,
    shaderStorageBufferArrayDynamicIndexing: 0,
    shaderStorageImageArrayDynamicIndexing: 0,
    shaderClipDistance: 0,
    shaderCullDistance: 0,
    shaderFloat64: 0,
    shaderInt64: 0,
    shaderInt16: 0,
    shaderResourceResidency: 0,
    shaderResourceMinLod: 0,
    sparseBinding: 0,
    sparseResidencyBuffer: 0,
    sparseResidencyImage2D: 0,
    sparseResidencyImage3D: 0,
    sparseResidency2Samples: 0,
    sparseResidency4Samples: 0,
    sparseResidency8Samples: 0,
    sparseResidency16Samples: 0,
    sparseResidencyAliased: 0,
    variableMultisampleRate: 0,
    inheritedQueries: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_robustBufferAccess(mut self, val: VkBool32) -> Self {
    self.robustBufferAccess = val;
    self
  }
  #[inline]
  pub const fn with_fullDrawIndexUint32(mut self, val: VkBool32) -> Self {
    self.fullDrawIndexUint32 = val;
    self
  }
  #[inline]
  pub const fn with_imageCubeArray(mut self, val: VkBool32) -> Self {
    self.imageCubeArray = val;
    self
  }
  #[inline]
  pub const fn with_independentBlend(mut self, val: VkBool32) -> Self {
    self.independentBlend = val;
    self
  }
  #[inline]
  pub const fn with_geometryShader(mut self, val: VkBool32) -> Self {
    self.geometryShader = val;
    self
  }
  #[inline]
  pub const fn with_tessellationShader(mut self, val: VkBool32) -> Self {
    self.tessellationShader = val;
    self
  }
  #[inline]
  pub const fn with_sampleRateShading(mut self, val: VkBool32) -> Self {
    self.sampleRateShading = val;
    self
  }
  #[inline]
  pub const fn with_dualSrcBlend(mut self, val: VkBool32) -> Self {
    self.dualSrcBlend = val;
    self
  }
  #[inline]
  pub const fn with_logicOp(mut self, val: VkBool32) -> Self {
    self.logicOp = val;
    self
  }
  #[inline]
  pub const fn with_multiDrawIndirect(mut self, val: VkBool32) -> Self {
    self.multiDrawIndirect = val;
    self
  }
  #[inline]
  pub const fn with_drawIndirectFirstInstance(mut self, val: VkBool32) -> Self {
    self.drawIndirectFirstInstance = val;
    self
  }
  #[inline]
  pub const fn with_depthClamp(mut self, val: VkBool32) -> Self {
    self.depthClamp = val;
    self
  }
  #[inline]
  pub const fn with_depthBiasClamp(mut self, val: VkBool32) -> Self {
    self.depthBiasClamp = val;
    self
  }
  #[inline]
  pub const fn with_fillModeNonSolid(mut self, val: VkBool32) -> Self {
    self.fillModeNonSolid = val;
    self
  }
  #[inline]
  pub const fn with_depthBounds(mut self, val: VkBool32) -> Self {
    self.depthBounds = val;
    self
  }
  #[inline]
  pub const fn with_wideLines(mut self, val: VkBool32) -> Self {
    self.wideLines = val;
    self
  }
  #[inline]
  pub const fn with_largePoints(mut self, val: VkBool32) -> Self {
    self.largePoints = val;
    self
  }
  #[inline]
  pub const fn with_alphaToOne(mut self, val: VkBool32) -> Self {
    self.alphaToOne = val;
    self
  }
  #[inline]
  pub const fn with_multiViewport(mut self, val: VkBool32) -> Self {
    self.multiViewport = val;
    self
  }
  #[inline]
  pub const fn with_samplerAnisotropy(mut self, val: VkBool32) -> Self {
    self.samplerAnisotropy = val;
    self
  }
  #[inline]
  pub const fn with_textureCompressionETC2(mut self, val: VkBool32) -> Self {
    self.textureCompressionETC2 = val;
    self
  }
  #[inline]
  pub const fn with_textureCompressionASTC_LDR(mut self, val: VkBool32) -> Self {
    self.textureCompressionASTC_LDR = val;
    self
  }
  #[inline]
  pub const fn with_textureCompressionBC(mut self, val: VkBool32) -> Self {
    self.textureCompressionBC = val;
    self
  }
  #[inline]
  pub const fn with_occlusionQueryPrecise(mut self, val: VkBool32) -> Self {
    self.occlusionQueryPrecise = val;
    self
  }
  #[inline]
  pub const fn with_pipelineStatisticsQuery(mut self, val: VkBool32) -> Self {
    self.pipelineStatisticsQuery = val;
    self
  }
  #[inline]
  pub const fn with_vertexPipelineStoresAndAtomics(mut self, val: VkBool32) -> Self {
    self.vertexPipelineStoresAndAtomics = val;
    self
  }
  #[inline]
  pub const fn with_fragmentStoresAndAtomics(mut self, val: VkBool32) -> Self {
    self.fragmentStoresAndAtomics = val;
    self
  }
  #[inline]
  pub const fn with_shaderTessellationAndGeometryPointSize(mut self, val: VkBool32) -> Self {
    self.shaderTessellationAndGeometryPointSize = val;
    self
  }
  #[inline]
  pub const fn with_shaderImageGatherExtended(mut self, val: VkBool32) -> Self {
    self.shaderImageGatherExtended = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageImageExtendedFormats(mut self, val: VkBool32) -> Self {
    self.shaderStorageImageExtendedFormats = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageImageMultisample(mut self, val: VkBool32) -> Self {
    self.shaderStorageImageMultisample = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageImageReadWithoutFormat(mut self, val: VkBool32) -> Self {
    self.shaderStorageImageReadWithoutFormat = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageImageWriteWithoutFormat(mut self, val: VkBool32) -> Self {
    self.shaderStorageImageWriteWithoutFormat = val;
    self
  }
  #[inline]
  pub const fn with_shaderUniformBufferArrayDynamicIndexing(mut self, val: VkBool32) -> Self {
    self.shaderUniformBufferArrayDynamicIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderSampledImageArrayDynamicIndexing(mut self, val: VkBool32) -> Self {
    self.shaderSampledImageArrayDynamicIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageBufferArrayDynamicIndexing(mut self, val: VkBool32) -> Self {
    self.shaderStorageBufferArrayDynamicIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageImageArrayDynamicIndexing(mut self, val: VkBool32) -> Self {
    self.shaderStorageImageArrayDynamicIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderClipDistance(mut self, val: VkBool32) -> Self {
    self.shaderClipDistance = val;
    self
  }
  #[inline]
  pub const fn with_shaderCullDistance(mut self, val: VkBool32) -> Self {
    self.shaderCullDistance = val;
    self
  }
  #[inline]
  pub const fn with_shaderFloat64(mut self, val: VkBool32) -> Self {
    self.shaderFloat64 = val;
    self
  }
  #[inline]
  pub const fn with_shaderInt64(mut self, val: VkBool32) -> Self {
    self.shaderInt64 = val;
    self
  }
  #[inline]
  pub const fn with_shaderInt16(mut self, val: VkBool32) -> Self {
    self.shaderInt16 = val;
    self
  }
  #[inline]
  pub const fn with_shaderResourceResidency(mut self, val: VkBool32) -> Self {
    self.shaderResourceResidency = val;
    self
  }
  #[inline]
  pub const fn with_shaderResourceMinLod(mut self, val: VkBool32) -> Self {
    self.shaderResourceMinLod = val;
    self
  }
  #[inline]
  pub const fn with_sparseBinding(mut self, val: VkBool32) -> Self {
    self.sparseBinding = val;
    self
  }
  #[inline]
  pub const fn with_sparseResidencyBuffer(mut self, val: VkBool32) -> Self {
    self.sparseResidencyBuffer = val;
    self
  }
  #[inline]
  pub const fn with_sparseResidencyImage2D(mut self, val: VkBool32) -> Self {
    self.sparseResidencyImage2D = val;
    self
  }
  #[inline]
  pub const fn with_sparseResidencyImage3D(mut self, val: VkBool32) -> Self {
    self.sparseResidencyImage3D = val;
    self
  }
  #[inline]
  pub const fn with_sparseResidency2Samples(mut self, val: VkBool32) -> Self {
    self.sparseResidency2Samples = val;
    self
  }
  #[inline]
  pub const fn with_sparseResidency4Samples(mut self, val: VkBool32) -> Self {
    self.sparseResidency4Samples = val;
    self
  }
  #[inline]
  pub const fn with_sparseResidency8Samples(mut self, val: VkBool32) -> Self {
    self.sparseResidency8Samples = val;
    self
  }
  #[inline]
  pub const fn with_sparseResidency16Samples(mut self, val: VkBool32) -> Self {
    self.sparseResidency16Samples = val;
    self
  }
  #[inline]
  pub const fn with_sparseResidencyAliased(mut self, val: VkBool32) -> Self {
    self.sparseResidencyAliased = val;
    self
  }
  #[inline]
  pub const fn with_variableMultisampleRate(mut self, val: VkBool32) -> Self {
    self.variableMultisampleRate = val;
    self
  }
  #[inline]
  pub const fn with_inheritedQueries(mut self, val: VkBool32) -> Self {
    self.inheritedQueries = val;
    self
  }
}
/// [VkPhysicalDeviceSparseProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSparseProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSparseProperties {
  /// Limit Type: [Max]
  pub residencyStandard2DBlockShape: VkBool32,
  /// Limit Type: [Max]
  pub residencyStandard2DMultisampleBlockShape: VkBool32,
  /// Limit Type: [Max]
  pub residencyStandard3DBlockShape: VkBool32,
  /// Limit Type: [Min]
  pub residencyAlignedMipSize: VkBool32,
  /// Limit Type: [Max]
  pub residencyNonResidentStrict: VkBool32,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkPhysicalDeviceSparseProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkPhysicalDeviceSparseProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkPhysicalDeviceSparseProperties {
  pub const DEFAULT: Self = Self {
    residencyStandard2DBlockShape: 0,
    residencyStandard2DMultisampleBlockShape: 0,
    residencyStandard3DBlockShape: 0,
    residencyAlignedMipSize: 0,
    residencyNonResidentStrict: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_residencyStandard2DBlockShape(mut self, val: VkBool32) -> Self {
    self.residencyStandard2DBlockShape = val;
    self
  }
  #[inline]
  pub const fn with_residencyStandard2DMultisampleBlockShape(mut self, val: VkBool32) -> Self {
    self.residencyStandard2DMultisampleBlockShape = val;
    self
  }
  #[inline]
  pub const fn with_residencyStandard3DBlockShape(mut self, val: VkBool32) -> Self {
    self.residencyStandard3DBlockShape = val;
    self
  }
  #[inline]
  pub const fn with_residencyAlignedMipSize(mut self, val: VkBool32) -> Self {
    self.residencyAlignedMipSize = val;
    self
  }
  #[inline]
  pub const fn with_residencyNonResidentStrict(mut self, val: VkBool32) -> Self {
    self.residencyNonResidentStrict = val;
    self
  }
}
/// [VkPhysicalDeviceLimits](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLimits.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceLimits {
  /// Limit Type: [Max]
  pub maxImageDimension1D: u32,
  /// Limit Type: [Max]
  pub maxImageDimension2D: u32,
  /// Limit Type: [Max]
  pub maxImageDimension3D: u32,
  /// Limit Type: [Max]
  pub maxImageDimensionCube: u32,
  /// Limit Type: [Max]
  pub maxImageArrayLayers: u32,
  /// Limit Type: [Max]
  pub maxTexelBufferElements: u32,
  /// Limit Type: [Max]
  pub maxUniformBufferRange: u32,
  /// Limit Type: [Max]
  pub maxStorageBufferRange: u32,
  /// Limit Type: [Max]
  pub maxPushConstantsSize: u32,
  /// Limit Type: [Max]
  pub maxMemoryAllocationCount: u32,
  /// Limit Type: [Max]
  pub maxSamplerAllocationCount: u32,
  /// Limit Type: [Min, Mul]
  pub bufferImageGranularity: VkDeviceSize,
  /// Limit Type: [Max]
  pub sparseAddressSpaceSize: VkDeviceSize,
  /// Limit Type: [Max]
  pub maxBoundDescriptorSets: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorSamplers: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorUniformBuffers: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorStorageBuffers: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorSampledImages: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorStorageImages: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorInputAttachments: u32,
  /// Limit Type: [Max]
  pub maxPerStageResources: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetSamplers: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUniformBuffers: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUniformBuffersDynamic: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetStorageBuffers: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetStorageBuffersDynamic: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetSampledImages: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetStorageImages: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetInputAttachments: u32,
  /// Limit Type: [Max]
  pub maxVertexInputAttributes: u32,
  /// Limit Type: [Max]
  pub maxVertexInputBindings: u32,
  /// Limit Type: [Max]
  pub maxVertexInputAttributeOffset: u32,
  /// Limit Type: [Max]
  pub maxVertexInputBindingStride: u32,
  /// Limit Type: [Max]
  pub maxVertexOutputComponents: u32,
  /// Limit Type: [Max]
  pub maxTessellationGenerationLevel: u32,
  /// Limit Type: [Max]
  pub maxTessellationPatchSize: u32,
  /// Limit Type: [Max]
  pub maxTessellationControlPerVertexInputComponents: u32,
  /// Limit Type: [Max]
  pub maxTessellationControlPerVertexOutputComponents: u32,
  /// Limit Type: [Max]
  pub maxTessellationControlPerPatchOutputComponents: u32,
  /// Limit Type: [Max]
  pub maxTessellationControlTotalOutputComponents: u32,
  /// Limit Type: [Max]
  pub maxTessellationEvaluationInputComponents: u32,
  /// Limit Type: [Max]
  pub maxTessellationEvaluationOutputComponents: u32,
  /// Limit Type: [Max]
  pub maxGeometryShaderInvocations: u32,
  /// Limit Type: [Max]
  pub maxGeometryInputComponents: u32,
  /// Limit Type: [Max]
  pub maxGeometryOutputComponents: u32,
  /// Limit Type: [Max]
  pub maxGeometryOutputVertices: u32,
  /// Limit Type: [Max]
  pub maxGeometryTotalOutputComponents: u32,
  /// Limit Type: [Max]
  pub maxFragmentInputComponents: u32,
  /// Limit Type: [Max]
  pub maxFragmentOutputAttachments: u32,
  /// Limit Type: [Max]
  pub maxFragmentDualSrcAttachments: u32,
  /// Limit Type: [Max]
  pub maxFragmentCombinedOutputResources: u32,
  /// Limit Type: [Max]
  pub maxComputeSharedMemorySize: u32,
  /// Limit Type: [Max]
  pub maxComputeWorkGroupCount: [u32; 3],
  /// Limit Type: [Max]
  pub maxComputeWorkGroupInvocations: u32,
  /// Limit Type: [Max]
  pub maxComputeWorkGroupSize: [u32; 3],
  /// Limit Type: [Bits]
  pub subPixelPrecisionBits: u32,
  /// Limit Type: [Bits]
  pub subTexelPrecisionBits: u32,
  /// Limit Type: [Bits]
  pub mipmapPrecisionBits: u32,
  /// Limit Type: [Max]
  pub maxDrawIndexedIndexValue: u32,
  /// Limit Type: [Max]
  pub maxDrawIndirectCount: u32,
  /// Limit Type: [Max]
  pub maxSamplerLodBias: f32,
  /// Limit Type: [Max]
  pub maxSamplerAnisotropy: f32,
  /// Limit Type: [Max]
  pub maxViewports: u32,
  /// Limit Type: [Max]
  pub maxViewportDimensions: [u32; 2],
  /// Limit Type: [Range]
  pub viewportBoundsRange: [f32; 2],
  /// Limit Type: [Bits]
  pub viewportSubPixelBits: u32,
  /// Limit Type: [Max, Pot]
  pub minMemoryMapAlignment: usize,
  /// Limit Type: [Min, Pot]
  pub minTexelBufferOffsetAlignment: VkDeviceSize,
  /// Limit Type: [Min, Pot]
  pub minUniformBufferOffsetAlignment: VkDeviceSize,
  /// Limit Type: [Min, Pot]
  pub minStorageBufferOffsetAlignment: VkDeviceSize,
  /// Limit Type: [Min]
  pub minTexelOffset: i32,
  /// Limit Type: [Max]
  pub maxTexelOffset: u32,
  /// Limit Type: [Min]
  pub minTexelGatherOffset: i32,
  /// Limit Type: [Max]
  pub maxTexelGatherOffset: u32,
  /// Limit Type: [Min]
  pub minInterpolationOffset: f32,
  /// Limit Type: [Max]
  pub maxInterpolationOffset: f32,
  /// Limit Type: [Bits]
  pub subPixelInterpolationOffsetBits: u32,
  /// Limit Type: [Max]
  pub maxFramebufferWidth: u32,
  /// Limit Type: [Max]
  pub maxFramebufferHeight: u32,
  /// Limit Type: [Max]
  pub maxFramebufferLayers: u32,
  /// Optional: true,  Limit Type: [Bitmask]
  pub framebufferColorSampleCounts: VkSampleCountFlags,
  /// Optional: true,  Limit Type: [Bitmask]
  pub framebufferDepthSampleCounts: VkSampleCountFlags,
  /// Optional: true,  Limit Type: [Bitmask]
  pub framebufferStencilSampleCounts: VkSampleCountFlags,
  /// Optional: true,  Limit Type: [Bitmask]
  pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
  /// Limit Type: [Max]
  pub maxColorAttachments: u32,
  /// Optional: true,  Limit Type: [Bitmask]
  pub sampledImageColorSampleCounts: VkSampleCountFlags,
  /// Optional: true,  Limit Type: [Bitmask]
  pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
  /// Optional: true,  Limit Type: [Bitmask]
  pub sampledImageDepthSampleCounts: VkSampleCountFlags,
  /// Optional: true,  Limit Type: [Bitmask]
  pub sampledImageStencilSampleCounts: VkSampleCountFlags,
  /// Optional: true,  Limit Type: [Bitmask]
  pub storageImageSampleCounts: VkSampleCountFlags,
  /// Limit Type: [Max]
  pub maxSampleMaskWords: u32,
  /// Limit Type: [Max]
  pub timestampComputeAndGraphics: VkBool32,
  /// Limit Type: [Min, Mul]
  pub timestampPeriod: f32,
  /// Limit Type: [Max]
  pub maxClipDistances: u32,
  /// Limit Type: [Max]
  pub maxCullDistances: u32,
  /// Limit Type: [Max]
  pub maxCombinedClipAndCullDistances: u32,
  /// Limit Type: [Max]
  pub discreteQueuePriorities: u32,
  /// Limit Type: [Range]
  pub pointSizeRange: [f32; 2],
  /// Limit Type: [Range]
  pub lineWidthRange: [f32; 2],
  /// Limit Type: [Min, Mul]
  pub pointSizeGranularity: f32,
  /// Limit Type: [Min, Mul]
  pub lineWidthGranularity: f32,
  /// Limit Type: [Max]
  pub strictLines: VkBool32,
  /// Limit Type: [Max]
  pub standardSampleLocations: VkBool32,
  /// Limit Type: [Min, Pot]
  pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
  /// Limit Type: [Min, Pot]
  pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
  /// Limit Type: [Min, Pot]
  pub nonCoherentAtomSize: VkDeviceSize,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Send for VkPhysicalDeviceLimits {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl Sync for VkPhysicalDeviceLimits {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VkPhysicalDeviceLimits {
  pub const DEFAULT: Self = Self {
    maxImageDimension1D: 0,
    maxImageDimension2D: 0,
    maxImageDimension3D: 0,
    maxImageDimensionCube: 0,
    maxImageArrayLayers: 0,
    maxTexelBufferElements: 0,
    maxUniformBufferRange: 0,
    maxStorageBufferRange: 0,
    maxPushConstantsSize: 0,
    maxMemoryAllocationCount: 0,
    maxSamplerAllocationCount: 0,
    bufferImageGranularity: 0,
    sparseAddressSpaceSize: 0,
    maxBoundDescriptorSets: 0,
    maxPerStageDescriptorSamplers: 0,
    maxPerStageDescriptorUniformBuffers: 0,
    maxPerStageDescriptorStorageBuffers: 0,
    maxPerStageDescriptorSampledImages: 0,
    maxPerStageDescriptorStorageImages: 0,
    maxPerStageDescriptorInputAttachments: 0,
    maxPerStageResources: 0,
    maxDescriptorSetSamplers: 0,
    maxDescriptorSetUniformBuffers: 0,
    maxDescriptorSetUniformBuffersDynamic: 0,
    maxDescriptorSetStorageBuffers: 0,
    maxDescriptorSetStorageBuffersDynamic: 0,
    maxDescriptorSetSampledImages: 0,
    maxDescriptorSetStorageImages: 0,
    maxDescriptorSetInputAttachments: 0,
    maxVertexInputAttributes: 0,
    maxVertexInputBindings: 0,
    maxVertexInputAttributeOffset: 0,
    maxVertexInputBindingStride: 0,
    maxVertexOutputComponents: 0,
    maxTessellationGenerationLevel: 0,
    maxTessellationPatchSize: 0,
    maxTessellationControlPerVertexInputComponents: 0,
    maxTessellationControlPerVertexOutputComponents: 0,
    maxTessellationControlPerPatchOutputComponents: 0,
    maxTessellationControlTotalOutputComponents: 0,
    maxTessellationEvaluationInputComponents: 0,
    maxTessellationEvaluationOutputComponents: 0,
    maxGeometryShaderInvocations: 0,
    maxGeometryInputComponents: 0,
    maxGeometryOutputComponents: 0,
    maxGeometryOutputVertices: 0,
    maxGeometryTotalOutputComponents: 0,
    maxFragmentInputComponents: 0,
    maxFragmentOutputAttachments: 0,
    maxFragmentDualSrcAttachments: 0,
    maxFragmentCombinedOutputResources: 0,
    maxComputeSharedMemorySize: 0,
    maxComputeWorkGroupCount: [0u32; 3],
    maxComputeWorkGroupInvocations: 0,
    maxComputeWorkGroupSize: [0u32; 3],
    subPixelPrecisionBits: 0,
    subTexelPrecisionBits: 0,
    mipmapPrecisionBits: 0,
    maxDrawIndexedIndexValue: 0,
    maxDrawIndirectCount: 0,
    maxSamplerLodBias: 0.0f32,
    maxSamplerAnisotropy: 0.0f32,
    maxViewports: 0,
    maxViewportDimensions: [0u32; 2],
    viewportBoundsRange: [0.0f32; 2],
    viewportSubPixelBits: 0,
    minMemoryMapAlignment: 0,
    minTexelBufferOffsetAlignment: 0,
    minUniformBufferOffsetAlignment: 0,
    minStorageBufferOffsetAlignment: 0,
    minTexelOffset: 0,
    maxTexelOffset: 0,
    minTexelGatherOffset: 0,
    maxTexelGatherOffset: 0,
    minInterpolationOffset: 0.0f32,
    maxInterpolationOffset: 0.0f32,
    subPixelInterpolationOffsetBits: 0,
    maxFramebufferWidth: 0,
    maxFramebufferHeight: 0,
    maxFramebufferLayers: 0,
    framebufferColorSampleCounts: VkSampleCountFlagBits(0),
    framebufferDepthSampleCounts: VkSampleCountFlagBits(0),
    framebufferStencilSampleCounts: VkSampleCountFlagBits(0),
    framebufferNoAttachmentsSampleCounts: VkSampleCountFlagBits(0),
    maxColorAttachments: 0,
    sampledImageColorSampleCounts: VkSampleCountFlagBits(0),
    sampledImageIntegerSampleCounts: VkSampleCountFlagBits(0),
    sampledImageDepthSampleCounts: VkSampleCountFlagBits(0),
    sampledImageStencilSampleCounts: VkSampleCountFlagBits(0),
    storageImageSampleCounts: VkSampleCountFlagBits(0),
    maxSampleMaskWords: 0,
    timestampComputeAndGraphics: 0,
    timestampPeriod: 0.0f32,
    maxClipDistances: 0,
    maxCullDistances: 0,
    maxCombinedClipAndCullDistances: 0,
    discreteQueuePriorities: 0,
    pointSizeRange: [0.0f32; 2],
    lineWidthRange: [0.0f32; 2],
    pointSizeGranularity: 0.0f32,
    lineWidthGranularity: 0.0f32,
    strictLines: 0,
    standardSampleLocations: 0,
    optimalBufferCopyOffsetAlignment: 0,
    optimalBufferCopyRowPitchAlignment: 0,
    nonCoherentAtomSize: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_maxImageDimension1D(mut self, val: u32) -> Self {
    self.maxImageDimension1D = val;
    self
  }
  #[inline]
  pub const fn with_maxImageDimension2D(mut self, val: u32) -> Self {
    self.maxImageDimension2D = val;
    self
  }
  #[inline]
  pub const fn with_maxImageDimension3D(mut self, val: u32) -> Self {
    self.maxImageDimension3D = val;
    self
  }
  #[inline]
  pub const fn with_maxImageDimensionCube(mut self, val: u32) -> Self {
    self.maxImageDimensionCube = val;
    self
  }
  #[inline]
  pub const fn with_maxImageArrayLayers(mut self, val: u32) -> Self {
    self.maxImageArrayLayers = val;
    self
  }
  #[inline]
  pub const fn with_maxTexelBufferElements(mut self, val: u32) -> Self {
    self.maxTexelBufferElements = val;
    self
  }
  #[inline]
  pub const fn with_maxUniformBufferRange(mut self, val: u32) -> Self {
    self.maxUniformBufferRange = val;
    self
  }
  #[inline]
  pub const fn with_maxStorageBufferRange(mut self, val: u32) -> Self {
    self.maxStorageBufferRange = val;
    self
  }
  #[inline]
  pub const fn with_maxPushConstantsSize(mut self, val: u32) -> Self {
    self.maxPushConstantsSize = val;
    self
  }
  #[inline]
  pub const fn with_maxMemoryAllocationCount(mut self, val: u32) -> Self {
    self.maxMemoryAllocationCount = val;
    self
  }
  #[inline]
  pub const fn with_maxSamplerAllocationCount(mut self, val: u32) -> Self {
    self.maxSamplerAllocationCount = val;
    self
  }
  #[inline]
  pub const fn with_bufferImageGranularity(mut self, val: VkDeviceSize) -> Self {
    self.bufferImageGranularity = val;
    self
  }
  #[inline]
  pub const fn with_sparseAddressSpaceSize(mut self, val: VkDeviceSize) -> Self {
    self.sparseAddressSpaceSize = val;
    self
  }
  #[inline]
  pub const fn with_maxBoundDescriptorSets(mut self, val: u32) -> Self {
    self.maxBoundDescriptorSets = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorSamplers(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorSamplers = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorUniformBuffers(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorUniformBuffers = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorStorageBuffers(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorStorageBuffers = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorSampledImages(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorSampledImages = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorStorageImages(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorStorageImages = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorInputAttachments(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorInputAttachments = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageResources(mut self, val: u32) -> Self {
    self.maxPerStageResources = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetSamplers(mut self, val: u32) -> Self {
    self.maxDescriptorSetSamplers = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUniformBuffers(mut self, val: u32) -> Self {
    self.maxDescriptorSetUniformBuffers = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUniformBuffersDynamic(mut self, val: u32) -> Self {
    self.maxDescriptorSetUniformBuffersDynamic = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetStorageBuffers(mut self, val: u32) -> Self {
    self.maxDescriptorSetStorageBuffers = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetStorageBuffersDynamic(mut self, val: u32) -> Self {
    self.maxDescriptorSetStorageBuffersDynamic = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetSampledImages(mut self, val: u32) -> Self {
    self.maxDescriptorSetSampledImages = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetStorageImages(mut self, val: u32) -> Self {
    self.maxDescriptorSetStorageImages = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetInputAttachments(mut self, val: u32) -> Self {
    self.maxDescriptorSetInputAttachments = val;
    self
  }
  #[inline]
  pub const fn with_maxVertexInputAttributes(mut self, val: u32) -> Self {
    self.maxVertexInputAttributes = val;
    self
  }
  #[inline]
  pub const fn with_maxVertexInputBindings(mut self, val: u32) -> Self {
    self.maxVertexInputBindings = val;
    self
  }
  #[inline]
  pub const fn with_maxVertexInputAttributeOffset(mut self, val: u32) -> Self {
    self.maxVertexInputAttributeOffset = val;
    self
  }
  #[inline]
  pub const fn with_maxVertexInputBindingStride(mut self, val: u32) -> Self {
    self.maxVertexInputBindingStride = val;
    self
  }
  #[inline]
  pub const fn with_maxVertexOutputComponents(mut self, val: u32) -> Self {
    self.maxVertexOutputComponents = val;
    self
  }
  #[inline]
  pub const fn with_maxTessellationGenerationLevel(mut self, val: u32) -> Self {
    self.maxTessellationGenerationLevel = val;
    self
  }
  #[inline]
  pub const fn with_maxTessellationPatchSize(mut self, val: u32) -> Self {
    self.maxTessellationPatchSize = val;
    self
  }
  #[inline]
  pub const fn with_maxTessellationControlPerVertexInputComponents(mut self, val: u32) -> Self {
    self.maxTessellationControlPerVertexInputComponents = val;
    self
  }
  #[inline]
  pub const fn with_maxTessellationControlPerVertexOutputComponents(mut self, val: u32) -> Self {
    self.maxTessellationControlPerVertexOutputComponents = val;
    self
  }
  #[inline]
  pub const fn with_maxTessellationControlPerPatchOutputComponents(mut self, val: u32) -> Self {
    self.maxTessellationControlPerPatchOutputComponents = val;
    self
  }
  #[inline]
  pub const fn with_maxTessellationControlTotalOutputComponents(mut self, val: u32) -> Self {
    self.maxTessellationControlTotalOutputComponents = val;
    self
  }
  #[inline]
  pub const fn with_maxTessellationEvaluationInputComponents(mut self, val: u32) -> Self {
    self.maxTessellationEvaluationInputComponents = val;
    self
  }
  #[inline]
  pub const fn with_maxTessellationEvaluationOutputComponents(mut self, val: u32) -> Self {
    self.maxTessellationEvaluationOutputComponents = val;
    self
  }
  #[inline]
  pub const fn with_maxGeometryShaderInvocations(mut self, val: u32) -> Self {
    self.maxGeometryShaderInvocations = val;
    self
  }
  #[inline]
  pub const fn with_maxGeometryInputComponents(mut self, val: u32) -> Self {
    self.maxGeometryInputComponents = val;
    self
  }
  #[inline]
  pub const fn with_maxGeometryOutputComponents(mut self, val: u32) -> Self {
    self.maxGeometryOutputComponents = val;
    self
  }
  #[inline]
  pub const fn with_maxGeometryOutputVertices(mut self, val: u32) -> Self {
    self.maxGeometryOutputVertices = val;
    self
  }
  #[inline]
  pub const fn with_maxGeometryTotalOutputComponents(mut self, val: u32) -> Self {
    self.maxGeometryTotalOutputComponents = val;
    self
  }
  #[inline]
  pub const fn with_maxFragmentInputComponents(mut self, val: u32) -> Self {
    self.maxFragmentInputComponents = val;
    self
  }
  #[inline]
  pub const fn with_maxFragmentOutputAttachments(mut self, val: u32) -> Self {
    self.maxFragmentOutputAttachments = val;
    self
  }
  #[inline]
  pub const fn with_maxFragmentDualSrcAttachments(mut self, val: u32) -> Self {
    self.maxFragmentDualSrcAttachments = val;
    self
  }
  #[inline]
  pub const fn with_maxFragmentCombinedOutputResources(mut self, val: u32) -> Self {
    self.maxFragmentCombinedOutputResources = val;
    self
  }
  #[inline]
  pub const fn with_maxComputeSharedMemorySize(mut self, val: u32) -> Self {
    self.maxComputeSharedMemorySize = val;
    self
  }
  #[inline]
  pub const fn with_maxComputeWorkGroupCount(mut self, val: [u32; 3]) -> Self {
    self.maxComputeWorkGroupCount = val;
    self
  }
  #[inline]
  pub const fn with_maxComputeWorkGroupInvocations(mut self, val: u32) -> Self {
    self.maxComputeWorkGroupInvocations = val;
    self
  }
  #[inline]
  pub const fn with_maxComputeWorkGroupSize(mut self, val: [u32; 3]) -> Self {
    self.maxComputeWorkGroupSize = val;
    self
  }
  #[inline]
  pub const fn with_subPixelPrecisionBits(mut self, val: u32) -> Self {
    self.subPixelPrecisionBits = val;
    self
  }
  #[inline]
  pub const fn with_subTexelPrecisionBits(mut self, val: u32) -> Self {
    self.subTexelPrecisionBits = val;
    self
  }
  #[inline]
  pub const fn with_mipmapPrecisionBits(mut self, val: u32) -> Self {
    self.mipmapPrecisionBits = val;
    self
  }
  #[inline]
  pub const fn with_maxDrawIndexedIndexValue(mut self, val: u32) -> Self {
    self.maxDrawIndexedIndexValue = val;
    self
  }
  #[inline]
  pub const fn with_maxDrawIndirectCount(mut self, val: u32) -> Self {
    self.maxDrawIndirectCount = val;
    self
  }
  #[inline]
  pub const fn with_maxSamplerLodBias(mut self, val: f32) -> Self {
    self.maxSamplerLodBias = val;
    self
  }
  #[inline]
  pub const fn with_maxSamplerAnisotropy(mut self, val: f32) -> Self {
    self.maxSamplerAnisotropy = val;
    self
  }
  #[inline]
  pub const fn with_maxViewports(mut self, val: u32) -> Self {
    self.maxViewports = val;
    self
  }
  #[inline]
  pub const fn with_maxViewportDimensions(mut self, val: [u32; 2]) -> Self {
    self.maxViewportDimensions = val;
    self
  }
  #[inline]
  pub const fn with_viewportBoundsRange(mut self, val: [f32; 2]) -> Self {
    self.viewportBoundsRange = val;
    self
  }
  #[inline]
  pub const fn with_viewportSubPixelBits(mut self, val: u32) -> Self {
    self.viewportSubPixelBits = val;
    self
  }
  #[inline]
  pub const fn with_minMemoryMapAlignment(mut self, val: usize) -> Self {
    self.minMemoryMapAlignment = val;
    self
  }
  #[inline]
  pub const fn with_minTexelBufferOffsetAlignment(mut self, val: VkDeviceSize) -> Self {
    self.minTexelBufferOffsetAlignment = val;
    self
  }
  #[inline]
  pub const fn with_minUniformBufferOffsetAlignment(mut self, val: VkDeviceSize) -> Self {
    self.minUniformBufferOffsetAlignment = val;
    self
  }
  #[inline]
  pub const fn with_minStorageBufferOffsetAlignment(mut self, val: VkDeviceSize) -> Self {
    self.minStorageBufferOffsetAlignment = val;
    self
  }
  #[inline]
  pub const fn with_minTexelOffset(mut self, val: i32) -> Self {
    self.minTexelOffset = val;
    self
  }
  #[inline]
  pub const fn with_maxTexelOffset(mut self, val: u32) -> Self {
    self.maxTexelOffset = val;
    self
  }
  #[inline]
  pub const fn with_minTexelGatherOffset(mut self, val: i32) -> Self {
    self.minTexelGatherOffset = val;
    self
  }
  #[inline]
  pub const fn with_maxTexelGatherOffset(mut self, val: u32) -> Self {
    self.maxTexelGatherOffset = val;
    self
  }
  #[inline]
  pub const fn with_minInterpolationOffset(mut self, val: f32) -> Self {
    self.minInterpolationOffset = val;
    self
  }
  #[inline]
  pub const fn with_maxInterpolationOffset(mut self, val: f32) -> Self {
    self.maxInterpolationOffset = val;
    self
  }
  #[inline]
  pub const fn with_subPixelInterpolationOffsetBits(mut self, val: u32) -> Self {
    self.subPixelInterpolationOffsetBits = val;
    self
  }
  #[inline]
  pub const fn with_maxFramebufferWidth(mut self, val: u32) -> Self {
    self.maxFramebufferWidth = val;
    self
  }
  #[inline]
  pub const fn with_maxFramebufferHeight(mut self, val: u32) -> Self {
    self.maxFramebufferHeight = val;
    self
  }
  #[inline]
  pub const fn with_maxFramebufferLayers(mut self, val: u32) -> Self {
    self.maxFramebufferLayers = val;
    self
  }
  #[inline]
  pub const fn with_framebufferColorSampleCounts(mut self, val: VkSampleCountFlags) -> Self {
    self.framebufferColorSampleCounts = val;
    self
  }
  #[inline]
  pub const fn with_framebufferDepthSampleCounts(mut self, val: VkSampleCountFlags) -> Self {
    self.framebufferDepthSampleCounts = val;
    self
  }
  #[inline]
  pub const fn with_framebufferStencilSampleCounts(mut self, val: VkSampleCountFlags) -> Self {
    self.framebufferStencilSampleCounts = val;
    self
  }
  #[inline]
  pub const fn with_framebufferNoAttachmentsSampleCounts(
    mut self,
    val: VkSampleCountFlags,
  ) -> Self {
    self.framebufferNoAttachmentsSampleCounts = val;
    self
  }
  #[inline]
  pub const fn with_maxColorAttachments(mut self, val: u32) -> Self {
    self.maxColorAttachments = val;
    self
  }
  #[inline]
  pub const fn with_sampledImageColorSampleCounts(mut self, val: VkSampleCountFlags) -> Self {
    self.sampledImageColorSampleCounts = val;
    self
  }
  #[inline]
  pub const fn with_sampledImageIntegerSampleCounts(mut self, val: VkSampleCountFlags) -> Self {
    self.sampledImageIntegerSampleCounts = val;
    self
  }
  #[inline]
  pub const fn with_sampledImageDepthSampleCounts(mut self, val: VkSampleCountFlags) -> Self {
    self.sampledImageDepthSampleCounts = val;
    self
  }
  #[inline]
  pub const fn with_sampledImageStencilSampleCounts(mut self, val: VkSampleCountFlags) -> Self {
    self.sampledImageStencilSampleCounts = val;
    self
  }
  #[inline]
  pub const fn with_storageImageSampleCounts(mut self, val: VkSampleCountFlags) -> Self {
    self.storageImageSampleCounts = val;
    self
  }
  #[inline]
  pub const fn with_maxSampleMaskWords(mut self, val: u32) -> Self {
    self.maxSampleMaskWords = val;
    self
  }
  #[inline]
  pub const fn with_timestampComputeAndGraphics(mut self, val: VkBool32) -> Self {
    self.timestampComputeAndGraphics = val;
    self
  }
  #[inline]
  pub const fn with_timestampPeriod(mut self, val: f32) -> Self {
    self.timestampPeriod = val;
    self
  }
  #[inline]
  pub const fn with_maxClipDistances(mut self, val: u32) -> Self {
    self.maxClipDistances = val;
    self
  }
  #[inline]
  pub const fn with_maxCullDistances(mut self, val: u32) -> Self {
    self.maxCullDistances = val;
    self
  }
  #[inline]
  pub const fn with_maxCombinedClipAndCullDistances(mut self, val: u32) -> Self {
    self.maxCombinedClipAndCullDistances = val;
    self
  }
  #[inline]
  pub const fn with_discreteQueuePriorities(mut self, val: u32) -> Self {
    self.discreteQueuePriorities = val;
    self
  }
  #[inline]
  pub const fn with_pointSizeRange(mut self, val: [f32; 2]) -> Self {
    self.pointSizeRange = val;
    self
  }
  #[inline]
  pub const fn with_lineWidthRange(mut self, val: [f32; 2]) -> Self {
    self.lineWidthRange = val;
    self
  }
  #[inline]
  pub const fn with_pointSizeGranularity(mut self, val: f32) -> Self {
    self.pointSizeGranularity = val;
    self
  }
  #[inline]
  pub const fn with_lineWidthGranularity(mut self, val: f32) -> Self {
    self.lineWidthGranularity = val;
    self
  }
  #[inline]
  pub const fn with_strictLines(mut self, val: VkBool32) -> Self {
    self.strictLines = val;
    self
  }
  #[inline]
  pub const fn with_standardSampleLocations(mut self, val: VkBool32) -> Self {
    self.standardSampleLocations = val;
    self
  }
  #[inline]
  pub const fn with_optimalBufferCopyOffsetAlignment(mut self, val: VkDeviceSize) -> Self {
    self.optimalBufferCopyOffsetAlignment = val;
    self
  }
  #[inline]
  pub const fn with_optimalBufferCopyRowPitchAlignment(mut self, val: VkDeviceSize) -> Self {
    self.optimalBufferCopyRowPitchAlignment = val;
    self
  }
  #[inline]
  pub const fn with_nonCoherentAtomSize(mut self, val: VkDeviceSize) -> Self {
    self.nonCoherentAtomSize = val;
    self
  }
}
/// [VkSemaphoreCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreCreateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSemaphoreCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkSemaphoreCreateFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkSemaphoreCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkSemaphoreCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkSemaphoreCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SEMAPHORE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkSemaphoreCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[cfg(feature = "VK_EXT_metal_objects")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportMetalObjectCreateInfoEXT<'child>(
    mut self,
    val: &'a VkExportMetalObjectCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportMetalObjectCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportSemaphoreCreateInfo<'child>(
    mut self,
    val: &'a VkExportSemaphoreCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportSemaphoreCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_sci_sync")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportSemaphoreSciSyncInfoNV<'child>(
    mut self,
    val: &'a VkExportSemaphoreSciSyncInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportSemaphoreSciSyncInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_external_semaphore_win32")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExportSemaphoreWin32HandleInfoKHR<'child>(
    mut self,
    val: &'a VkExportSemaphoreWin32HandleInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExportSemaphoreWin32HandleInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_metal_objects")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImportMetalSharedEventInfoEXT<'child>(
    mut self,
    val: &'a VkImportMetalSharedEventInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImportMetalSharedEventInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_low_latency")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueryLowLatencySupportNV<'child>(
    mut self,
    val: &'a VkQueryLowLatencySupportNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkQueryLowLatencySupportNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_sci_sync2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSemaphoreSciSyncCreateInfoNV<'child>(
    mut self,
    val: &'a VkSemaphoreSciSyncCreateInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkSemaphoreSciSyncCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSemaphoreTypeCreateInfo<'child>(
    mut self,
    val: &'a VkSemaphoreTypeCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkSemaphoreTypeCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSemaphoreCreateInfo<
    'root,
    T: VkPNextExtends<VkSemaphoreCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkQueryPoolCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueryPoolCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkQueryPoolCreateFlags,
  pub queryType: VkQueryType,
  pub queryCount: u32,
  /// Optional: true,  No Auto-Validity
  pub pipelineStatistics: VkQueryPipelineStatisticFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkQueryPoolCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkQueryPoolCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkQueryPoolCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::QUERY_POOL_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkQueryPoolCreateFlagBits(0),
    queryType: VkQueryType(0),
    queryCount: 0,
    pipelineStatistics: VkQueryPipelineStatisticFlagBits(0),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkQueryPoolCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_queryType(mut self, val: VkQueryType) -> Self {
    self.queryType = val;
    self
  }
  #[inline]
  pub const fn with_queryCount(mut self, val: u32) -> Self {
    self.queryCount = val;
    self
  }
  #[inline]
  pub const fn with_pipelineStatistics(mut self, val: VkQueryPipelineStatisticFlags) -> Self {
    self.pipelineStatistics = val;
    self
  }
  #[cfg(feature = "VK_KHR_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueryPoolPerformanceCreateInfoKHR<'child>(
    mut self,
    val: &'a VkQueryPoolPerformanceCreateInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkQueryPoolPerformanceCreateInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_INTEL_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueryPoolPerformanceQueryCreateInfoINTEL<'child>(
    mut self,
    val: &'a VkQueryPoolPerformanceQueryCreateInfoINTEL<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkQueryPoolPerformanceQueryCreateInfoINTEL<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueryPoolVideoEncodeFeedbackCreateInfoKHR<'child>(
    mut self,
    val: &'a VkQueryPoolVideoEncodeFeedbackCreateInfoKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkQueryPoolVideoEncodeFeedbackCreateInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_feedback2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR<'child>(
    mut self,
    val: &'a VkQueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkQueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_decode_av1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoDecodeAV1ProfileInfoKHR<'child>(
    mut self,
    val: &'a VkVideoDecodeAV1ProfileInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoDecodeAV1ProfileInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_decode_h264")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoDecodeH264ProfileInfoKHR<'child>(
    mut self,
    val: &'a VkVideoDecodeH264ProfileInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoDecodeH264ProfileInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_decode_h265")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoDecodeH265ProfileInfoKHR<'child>(
    mut self,
    val: &'a VkVideoDecodeH265ProfileInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoDecodeH265ProfileInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_decode_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoDecodeUsageInfoKHR<'child>(
    mut self,
    val: &'a VkVideoDecodeUsageInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoDecodeUsageInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_decode_vp9")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoDecodeVP9ProfileInfoKHR<'child>(
    mut self,
    val: &'a VkVideoDecodeVP9ProfileInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoDecodeVP9ProfileInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_av1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoEncodeAV1ProfileInfoKHR<'child>(
    mut self,
    val: &'a VkVideoEncodeAV1ProfileInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoEncodeAV1ProfileInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_h264")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoEncodeH264ProfileInfoKHR<'child>(
    mut self,
    val: &'a VkVideoEncodeH264ProfileInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoEncodeH264ProfileInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_h265")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoEncodeH265ProfileInfoKHR<'child>(
    mut self,
    val: &'a VkVideoEncodeH265ProfileInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoEncodeH265ProfileInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoEncodeUsageInfoKHR<'child>(
    mut self,
    val: &'a VkVideoEncodeUsageInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoEncodeUsageInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkVideoProfileInfoKHR<'child>(
    mut self,
    val: &'a VkVideoProfileInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkVideoProfileInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkQueryPoolCreateInfo<
    'root,
    T: VkPNextExtends<VkQueryPoolCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSubmitInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `VkSubmitInfo2`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubmitInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SUBMIT_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub waitSemaphoreCount: u32,
  /// Length: waitSemaphoreCount
  pub pWaitSemaphores: *const VkSemaphore,
  /// Optional: pointer required, values optional if pointer not null,  Length: waitSemaphoreCount
  pub pWaitDstStageMask: *const VkPipelineStageFlags,
  /// Optional: true
  pub commandBufferCount: u32,
  /// Length: commandBufferCount
  pub pCommandBuffers: *const VkCommandBuffer,
  /// Optional: true
  pub signalSemaphoreCount: u32,
  /// Length: signalSemaphoreCount
  pub pSignalSemaphores: *const VkSemaphore,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Send for VkSubmitInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'a> Sync for VkSubmitInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'a> VkSubmitInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SUBMIT_INFO,
    pNext: core::ptr::null(),
    waitSemaphoreCount: 0,
    pWaitSemaphores: core::ptr::null(),
    pWaitDstStageMask: core::ptr::null(),
    commandBufferCount: 0,
    pCommandBuffers: core::ptr::null(),
    signalSemaphoreCount: 0,
    pSignalSemaphores: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_waitSemaphoreCount(mut self, val: u32) -> Self {
    self.waitSemaphoreCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pWaitSemaphores(mut self, val: &'a [VkSemaphore]) -> Self {
    self.waitSemaphoreCount = val.len() as u32;
    self.pWaitSemaphores = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pWaitDstStageMask(mut self, val: &'a [VkPipelineStageFlags]) -> Self {
    self.waitSemaphoreCount = val.len() as u32;
    self.pWaitDstStageMask = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_commandBufferCount(mut self, val: u32) -> Self {
    self.commandBufferCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCommandBuffers(mut self, val: &'a [VkCommandBuffer]) -> Self {
    self.commandBufferCount = val.len() as u32;
    self.pCommandBuffers = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_signalSemaphoreCount(mut self, val: u32) -> Self {
    self.signalSemaphoreCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSignalSemaphores(mut self, val: &'a [VkSemaphore]) -> Self {
    self.signalSemaphoreCount = val.len() as u32;
    self.pSignalSemaphores = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure every provided array constrained by `waitSemaphoreCount` has the same length. Optional pointer arguments may be null, but non-null pointers must be valid for that same length and outlive any use of this struct instance.
  #[inline]
  pub const fn with_waitSemaphoreCount_slices(
    mut self,
    pWaitSemaphores: &'a [VkSemaphore],
    pWaitDstStageMask: *const VkPipelineStageFlags,
  ) -> Self {
    let len = pWaitSemaphores.len();
    self.waitSemaphoreCount = len as u32;
    self.pWaitSemaphores = pWaitSemaphores.as_ptr();
    self.pWaitDstStageMask = pWaitDstStageMask;
    self
  }
  #[cfg(feature = "VK_SEC_amigo_profiling")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkAmigoProfilingSubmitInfoSEC<'child>(
    mut self,
    val: &'a VkAmigoProfilingSubmitInfoSEC<'child>,
  ) -> Self {
    self.pNext = (val as *const VkAmigoProfilingSubmitInfoSEC<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_external_semaphore_win32")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkD3D12FenceSubmitInfoKHR<'child>(
    mut self,
    val: &'a VkD3D12FenceSubmitInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkD3D12FenceSubmitInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceGroupSubmitInfo<'child>(
    mut self,
    val: &'a VkDeviceGroupSubmitInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDeviceGroupSubmitInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_frame_boundary")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFrameBoundaryEXT<'child>(
    mut self,
    val: &'a VkFrameBoundaryEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkFrameBoundaryEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFrameBoundaryTensorsARM<'child>(
    mut self,
    val: &'a VkFrameBoundaryTensorsARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkFrameBoundaryTensorsARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_low_latency2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkLatencySubmissionPresentIdNV<'child>(
    mut self,
    val: &'a VkLatencySubmissionPresentIdNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkLatencySubmissionPresentIdNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPerformanceQuerySubmitInfoKHR<'child>(
    mut self,
    val: &'a VkPerformanceQuerySubmitInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPerformanceQuerySubmitInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkProtectedSubmitInfo<'child>(
    mut self,
    val: &'a VkProtectedSubmitInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkProtectedSubmitInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_SEC_throttle_hint")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkThrottleHintSubmitInfoSEC<'child>(
    mut self,
    val: &'a VkThrottleHintSubmitInfoSEC<'child>,
  ) -> Self {
    self.pNext = (val as *const VkThrottleHintSubmitInfoSEC<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTimelineSemaphoreSubmitInfo<'child>(
    mut self,
    val: &'a VkTimelineSemaphoreSubmitInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkTimelineSemaphoreSubmitInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_win32_keyed_mutex")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkWin32KeyedMutexAcquireReleaseInfoKHR<'child>(
    mut self,
    val: &'a VkWin32KeyedMutexAcquireReleaseInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkWin32KeyedMutexAcquireReleaseInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_win32_keyed_mutex")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkWin32KeyedMutexAcquireReleaseInfoNV<'child>(
    mut self,
    val: &'a VkWin32KeyedMutexAcquireReleaseInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkWin32KeyedMutexAcquireReleaseInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubmitInfo<'root, T: VkPNextExtends<VkSubmitInfo<'root>>>(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
