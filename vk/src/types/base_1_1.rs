use crate::consts::VK_LUID_SIZE;
use crate::consts::VK_MAX_DEVICE_GROUP_SIZE;
use crate::consts::VK_UUID_SIZE;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkBufferCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkBufferUsageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkDeviceQueueCreateFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_fence_capabilities"
))]
use crate::enums::VkExternalFenceFeatureFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_fence_capabilities"
))]
use crate::enums::VkExternalFenceHandleTypeFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_memory_capabilities"
))]
use crate::enums::VkExternalMemoryFeatureFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_memory_capabilities",
  feature = "VK_EXT_external_memory_dma_buf",
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  feature = "VK_EXT_external_memory_host",
  feature = "VK_FUCHSIA_external_memory",
  feature = "VK_NV_external_memory_rdma",
  feature = "VK_OHOS_external_memory",
  feature = "VK_QNX_external_memory_screen_buffer",
  feature = "VK_EXT_external_memory_metal"
))]
use crate::enums::VkExternalMemoryHandleTypeFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_semaphore_capabilities"
))]
use crate::enums::VkExternalSemaphoreFeatureFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_semaphore_capabilities"
))]
use crate::enums::VkExternalSemaphoreHandleTypeFlagBits;
#[cfg(any(feature = "VK_BASE_VERSION_1_1", feature = "VK_KHR_external_fence"))]
use crate::enums::VkFenceImportFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageAspectFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageTiling;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageUsageFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_device_group",
  feature = "VK_KHR_buffer_device_address",
  feature = "VK_EXT_zero_initialize_device_memory"
))]
use crate::enums::VkMemoryAllocateFlagBits;
#[cfg(any(feature = "VK_BASE_VERSION_1_1", feature = "VK_KHR_device_group"))]
use crate::enums::VkPeerMemoryFeatureFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSampleCountFlagBits;
#[cfg(any(feature = "VK_BASE_VERSION_1_1", feature = "VK_KHR_external_semaphore"))]
use crate::enums::VkSemaphoreImportFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_shader_subgroup_rotate",
  feature = "VK_EXT_shader_subgroup_partitioned"
))]
use crate::enums::VkSubgroupFeatureFlagBits;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkAndroidHardwareBufferUsageANDROID;
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain")
))]
use crate::types::VkBindImageMemorySwapchainInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkBindMemoryStatus;
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
use crate::types::VkBindSparseInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBuffer;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferUsageFlags;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkBufferUsageFlags2CreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferBeginInfo;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphOpticalFlowImageFormatInfoARM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceMemory;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceQueueCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(any(
  all(
    feature = "VK_EXT_image_drm_format_modifier",
    feature = "VK_KHR_format_feature_flags2"
  ),
  all(
    feature = "VK_EXT_image_drm_format_modifier",
    feature = "VK_VERSION_1_3"
  )
))]
use crate::types::VkDrmFormatModifierPropertiesList2EXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::types::VkDrmFormatModifierPropertiesListEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFenceCreateInfo;
#[cfg(feature = "VK_EXT_filter_cubic")]
use crate::types::VkFilterCubicImageViewImageFormatPropertiesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkFormatProperties3;
#[cfg(feature = "VK_KHR_extended_flags")]
use crate::types::VkFormatProperties4KHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkHostImageCopyDevicePerformanceQuery;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImage;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::types::VkImageCompressionControlEXT;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::types::VkImageCompressionPropertiesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateFlags;
#[cfg(feature = "VK_KHR_extended_flags")]
use crate::types::VkImageCreateFlags2CreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkImageFormatListCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageFormatProperties;
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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageUsageFlags;
#[cfg(feature = "VK_KHR_extended_flags")]
use crate::types::VkImageUsageFlags2CreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageViewCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryAllocateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryRequirements;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkNativeBufferUsageOHOS;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowImageFormatInfoNV;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDevice;
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
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkPhysicalDeviceAccelerationStructurePropertiesKHR;
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
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
use crate::types::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT;
#[cfg(feature = "VK_EXT_border_color_swizzle")]
use crate::types::VkPhysicalDeviceBorderColorSwizzleFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceBufferDeviceAddressFeatures;
#[cfg(feature = "VK_EXT_buffer_device_address")]
use crate::types::VkPhysicalDeviceBufferDeviceAddressFeaturesEXT;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkPhysicalDeviceClusterAccelerationStructureFeaturesNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkPhysicalDeviceClusterAccelerationStructurePropertiesNV;
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
use crate::types::VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI;
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
use crate::types::VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI;
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
#[cfg(feature = "VK_KHR_compute_shader_derivatives")]
use crate::types::VkPhysicalDeviceComputeShaderDerivativesPropertiesKHR;
#[cfg(feature = "VK_EXT_conditional_rendering")]
use crate::types::VkPhysicalDeviceConditionalRenderingFeaturesEXT;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
use crate::types::VkPhysicalDeviceConservativeRasterizationPropertiesEXT;
#[cfg(feature = "VK_NV_cooperative_matrix2")]
use crate::types::VkPhysicalDeviceCooperativeMatrix2FeaturesNV;
#[cfg(feature = "VK_NV_cooperative_matrix2")]
use crate::types::VkPhysicalDeviceCooperativeMatrix2PropertiesNV;
#[cfg(feature = "VK_QCOM_cooperative_matrix_conversion")]
use crate::types::VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM;
#[cfg(feature = "VK_NV_cooperative_matrix_decode_vector")]
use crate::types::VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV;
#[cfg(feature = "VK_KHR_cooperative_matrix")]
use crate::types::VkPhysicalDeviceCooperativeMatrixFeaturesKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::types::VkPhysicalDeviceCooperativeMatrixFeaturesNV;
#[cfg(feature = "VK_EXT_cooperative_matrix_maintenance1")]
use crate::types::VkPhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT;
#[cfg(feature = "VK_KHR_cooperative_matrix")]
use crate::types::VkPhysicalDeviceCooperativeMatrixPropertiesKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::types::VkPhysicalDeviceCooperativeMatrixPropertiesNV;
#[cfg(feature = "VK_NV_cooperative_vector")]
use crate::types::VkPhysicalDeviceCooperativeVectorFeaturesNV;
#[cfg(feature = "VK_NV_cooperative_vector")]
use crate::types::VkPhysicalDeviceCooperativeVectorPropertiesNV;
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
use crate::types::VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR;
#[cfg(feature = "VK_NV_copy_memory_indirect")]
use crate::types::VkPhysicalDeviceCopyMemoryIndirectFeaturesNV;
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
use crate::types::VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR;
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
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
use crate::types::VkPhysicalDeviceCudaKernelLaunchPropertiesNV;
#[cfg(feature = "VK_EXT_custom_border_color")]
use crate::types::VkPhysicalDeviceCustomBorderColorFeaturesEXT;
#[cfg(feature = "VK_EXT_custom_border_color")]
use crate::types::VkPhysicalDeviceCustomBorderColorPropertiesEXT;
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
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkPhysicalDeviceDepthStencilResolveProperties;
#[cfg(all(
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_EXT_fragment_density_map"
))]
use crate::types::VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkPhysicalDeviceDescriptorBufferFeaturesEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkPhysicalDeviceDescriptorBufferPropertiesEXT;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
use crate::types::VkPhysicalDeviceDescriptorBufferTensorFeaturesARM;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
use crate::types::VkPhysicalDeviceDescriptorBufferTensorPropertiesARM;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkPhysicalDeviceDescriptorHeapFeaturesEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkPhysicalDeviceDescriptorHeapPropertiesEXT;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_heap"))]
use crate::types::VkPhysicalDeviceDescriptorHeapTensorPropertiesARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceDescriptorIndexingFeatures;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceDescriptorIndexingProperties;
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
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkPhysicalDeviceDeviceGeneratedCommandsPropertiesEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV;
#[cfg(feature = "VK_EXT_device_memory_report")]
use crate::types::VkPhysicalDeviceDeviceMemoryReportFeaturesEXT;
#[cfg(feature = "VK_NV_device_diagnostics_config")]
use crate::types::VkPhysicalDeviceDiagnosticsConfigFeaturesNV;
#[cfg(feature = "VK_EXT_discard_rectangles")]
use crate::types::VkPhysicalDeviceDiscardRectanglePropertiesEXT;
#[cfg(feature = "VK_NV_displacement_micromap")]
use crate::types::VkPhysicalDeviceDisplacementMicromapFeaturesNV;
#[cfg(feature = "VK_NV_displacement_micromap")]
use crate::types::VkPhysicalDeviceDisplacementMicromapPropertiesNV;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceDriverProperties;
#[cfg(feature = "VK_EXT_physical_device_drm")]
use crate::types::VkPhysicalDeviceDrmPropertiesEXT;
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
#[cfg(feature = "VK_EXT_extended_dynamic_state3")]
use crate::types::VkPhysicalDeviceExtendedDynamicState3PropertiesEXT;
#[cfg(feature = "VK_EXT_extended_dynamic_state")]
use crate::types::VkPhysicalDeviceExtendedDynamicStateFeaturesEXT;
#[cfg(feature = "VK_KHR_extended_flags")]
use crate::types::VkPhysicalDeviceExtendedFlagsFeaturesKHR;
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
use crate::types::VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV;
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
use crate::types::VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV;
#[cfg(feature = "VK_NV_external_compute_queue")]
use crate::types::VkPhysicalDeviceExternalComputeQueuePropertiesNV;
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
use crate::types::VkPhysicalDeviceExternalFormatResolveFeaturesANDROID;
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
use crate::types::VkPhysicalDeviceExternalFormatResolvePropertiesANDROID;
#[cfg(feature = "VK_EXT_external_memory_host")]
use crate::types::VkPhysicalDeviceExternalMemoryHostPropertiesEXT;
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
#[cfg(feature = "VK_KHR_device_fault")]
use crate::types::VkPhysicalDeviceFaultPropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDeviceFeatures;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceFloatControlsProperties;
#[cfg(feature = "VK_ARM_format_pack")]
use crate::types::VkPhysicalDeviceFormatPackFeaturesARM;
#[cfg(feature = "VK_EXT_fragment_density_map2")]
use crate::types::VkPhysicalDeviceFragmentDensityMap2FeaturesEXT;
#[cfg(feature = "VK_EXT_fragment_density_map2")]
use crate::types::VkPhysicalDeviceFragmentDensityMap2PropertiesEXT;
#[cfg(feature = "VK_EXT_fragment_density_map")]
use crate::types::VkPhysicalDeviceFragmentDensityMapFeaturesEXT;
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
use crate::types::VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE;
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
use crate::types::VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE;
#[cfg(feature = "VK_EXT_fragment_density_map_offset")]
use crate::types::VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT;
#[cfg(feature = "VK_EXT_fragment_density_map_offset")]
use crate::types::VkPhysicalDeviceFragmentDensityMapOffsetPropertiesEXT;
#[cfg(feature = "VK_EXT_fragment_density_map")]
use crate::types::VkPhysicalDeviceFragmentDensityMapPropertiesEXT;
#[cfg(feature = "VK_KHR_fragment_shader_barycentric")]
use crate::types::VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR;
#[cfg(all(
  feature = "VK_EXT_provoking_vertex",
  feature = "VK_KHR_fragment_shader_barycentric"
))]
use crate::types::VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR;
#[cfg(feature = "VK_EXT_fragment_shader_interlock")]
use crate::types::VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::types::VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::types::VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::types::VkPhysicalDeviceFragmentShadingRateFeaturesKHR;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::types::VkPhysicalDeviceFragmentShadingRatePropertiesKHR;
#[cfg(feature = "VK_EXT_frame_boundary")]
use crate::types::VkPhysicalDeviceFrameBoundaryFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceGlobalPriorityQueryFeatures;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkPhysicalDeviceGpaFeaturesAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkPhysicalDeviceGpaProperties2AMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkPhysicalDeviceGpaPropertiesAMD;
#[cfg(feature = "VK_EXT_graphics_pipeline_library")]
use crate::types::VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT;
#[cfg(feature = "VK_EXT_graphics_pipeline_library")]
use crate::types::VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT;
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
use crate::types::VkPhysicalDeviceHdrVividFeaturesHUAWEI;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceHostImageCopyFeatures;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceHostImageCopyProperties;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceHostQueryResetFeatures;
#[cfg(feature = "VK_EXT_image_2d_view_of_3d")]
use crate::types::VkPhysicalDeviceImage2DViewOf3DFeaturesEXT;
#[cfg(feature = "VK_MESA_image_alignment_control")]
use crate::types::VkPhysicalDeviceImageAlignmentControlFeaturesMESA;
#[cfg(feature = "VK_MESA_image_alignment_control")]
use crate::types::VkPhysicalDeviceImageAlignmentControlPropertiesMESA;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::types::VkPhysicalDeviceImageCompressionControlFeaturesEXT;
#[cfg(feature = "VK_EXT_image_compression_control_swapchain")]
use crate::types::VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::types::VkPhysicalDeviceImageDrmFormatModifierInfoEXT;
#[cfg(feature = "VK_QCOM_image_processing2")]
use crate::types::VkPhysicalDeviceImageProcessing2FeaturesQCOM;
#[cfg(feature = "VK_QCOM_image_processing2")]
use crate::types::VkPhysicalDeviceImageProcessing2PropertiesQCOM;
#[cfg(feature = "VK_QCOM_image_processing3")]
use crate::types::VkPhysicalDeviceImageProcessing3FeaturesQCOM;
#[cfg(feature = "VK_QCOM_image_processing")]
use crate::types::VkPhysicalDeviceImageProcessingFeaturesQCOM;
#[cfg(feature = "VK_QCOM_image_processing")]
use crate::types::VkPhysicalDeviceImageProcessingPropertiesQCOM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceImageRobustnessFeatures;
#[cfg(feature = "VK_EXT_image_sliced_view_of_3d")]
use crate::types::VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT;
#[cfg(feature = "VK_EXT_image_tiling_control")]
use crate::types::VkPhysicalDeviceImageTilingControlFeaturesEXT;
#[cfg(feature = "VK_EXT_filter_cubic")]
use crate::types::VkPhysicalDeviceImageViewImageFormatInfoEXT;
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
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceInlineUniformBlockProperties;
#[cfg(feature = "VK_KHR_internally_synchronized_queues")]
use crate::types::VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR;
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
use crate::types::VkPhysicalDeviceInvocationMaskFeaturesHUAWEI;
#[cfg(feature = "VK_KHR_maintenance7")]
use crate::types::VkPhysicalDeviceLayeredApiPropertiesListKHR;
#[cfg(feature = "VK_MSFT_layered_driver")]
use crate::types::VkPhysicalDeviceLayeredDriverPropertiesMSFT;
#[cfg(feature = "VK_EXT_legacy_dithering")]
use crate::types::VkPhysicalDeviceLegacyDitheringFeaturesEXT;
#[cfg(feature = "VK_EXT_legacy_vertex_attributes")]
use crate::types::VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT;
#[cfg(feature = "VK_EXT_legacy_vertex_attributes")]
use crate::types::VkPhysicalDeviceLegacyVertexAttributesPropertiesEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceLineRasterizationFeatures;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceLineRasterizationProperties;
#[cfg(feature = "VK_NV_linear_color_attachment")]
use crate::types::VkPhysicalDeviceLinearColorAttachmentFeaturesNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceMaintenance3Properties;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceMaintenance4Features;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceMaintenance4Properties;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceMaintenance5Features;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceMaintenance5Properties;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceMaintenance6Features;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceMaintenance6Properties;
#[cfg(feature = "VK_KHR_maintenance7")]
use crate::types::VkPhysicalDeviceMaintenance7FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance7")]
use crate::types::VkPhysicalDeviceMaintenance7PropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance8")]
use crate::types::VkPhysicalDeviceMaintenance8FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance9")]
use crate::types::VkPhysicalDeviceMaintenance9FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance9")]
use crate::types::VkPhysicalDeviceMaintenance9PropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkPhysicalDeviceMaintenance10FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkPhysicalDeviceMaintenance10PropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance11")]
use crate::types::VkPhysicalDeviceMaintenance11FeaturesKHR;
#[cfg(feature = "VK_EXT_map_memory_placed")]
use crate::types::VkPhysicalDeviceMapMemoryPlacedFeaturesEXT;
#[cfg(feature = "VK_EXT_map_memory_placed")]
use crate::types::VkPhysicalDeviceMapMemoryPlacedPropertiesEXT;
#[cfg(feature = "VK_EXT_memory_budget")]
use crate::types::VkPhysicalDeviceMemoryBudgetPropertiesEXT;
#[cfg(feature = "VK_EXT_memory_decompression")]
use crate::types::VkPhysicalDeviceMemoryDecompressionFeaturesEXT;
#[cfg(feature = "VK_EXT_memory_decompression")]
use crate::types::VkPhysicalDeviceMemoryDecompressionPropertiesEXT;
#[cfg(feature = "VK_EXT_memory_priority")]
use crate::types::VkPhysicalDeviceMemoryPriorityFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDeviceMemoryProperties;
#[cfg(feature = "VK_EXT_mesh_shader")]
use crate::types::VkPhysicalDeviceMeshShaderFeaturesEXT;
#[cfg(feature = "VK_NV_mesh_shader")]
use crate::types::VkPhysicalDeviceMeshShaderFeaturesNV;
#[cfg(feature = "VK_EXT_mesh_shader")]
use crate::types::VkPhysicalDeviceMeshShaderPropertiesEXT;
#[cfg(feature = "VK_NV_mesh_shader")]
use crate::types::VkPhysicalDeviceMeshShaderPropertiesNV;
#[cfg(feature = "VK_EXT_multi_draw")]
use crate::types::VkPhysicalDeviceMultiDrawFeaturesEXT;
#[cfg(feature = "VK_EXT_multi_draw")]
use crate::types::VkPhysicalDeviceMultiDrawPropertiesEXT;
#[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
use crate::types::VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT;
#[cfg(feature = "VK_EXT_multisampled_render_to_swapchain")]
use crate::types::VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPhysicalDeviceMultiviewFeatures;
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
use crate::types::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX;
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
use crate::types::VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM;
#[cfg(feature = "VK_QCOM_multiview_per_view_viewports")]
use crate::types::VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPhysicalDeviceMultiviewProperties;
#[cfg(feature = "VK_EXT_mutable_descriptor_type")]
use crate::types::VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT;
#[cfg(feature = "VK_EXT_nested_command_buffer")]
use crate::types::VkPhysicalDeviceNestedCommandBufferFeaturesEXT;
#[cfg(feature = "VK_EXT_nested_command_buffer")]
use crate::types::VkPhysicalDeviceNestedCommandBufferPropertiesEXT;
#[cfg(feature = "VK_EXT_non_seamless_cube_map")]
use crate::types::VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkPhysicalDeviceOpacityMicromapFeaturesEXT;
#[cfg(feature = "VK_KHR_opacity_micromap")]
use crate::types::VkPhysicalDeviceOpacityMicromapFeaturesKHR;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkPhysicalDeviceOpacityMicromapPropertiesEXT;
#[cfg(feature = "VK_KHR_opacity_micromap")]
use crate::types::VkPhysicalDeviceOpacityMicromapPropertiesKHR;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkPhysicalDeviceOpticalFlowFeaturesNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkPhysicalDeviceOpticalFlowPropertiesNV;
#[cfg(feature = "VK_EXT_pci_bus_info")]
use crate::types::VkPhysicalDevicePCIBusInfoPropertiesEXT;
#[cfg(feature = "VK_EXT_pageable_device_local_memory")]
use crate::types::VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV;
#[cfg(feature = "VK_NV_per_stage_descriptor_set")]
use crate::types::VkPhysicalDevicePerStageDescriptorSetFeaturesNV;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkPhysicalDevicePerformanceCountersByRegionFeaturesARM;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkPhysicalDevicePerformanceCountersByRegionPropertiesARM;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPhysicalDevicePerformanceQueryFeaturesKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPhysicalDevicePerformanceQueryPropertiesKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPhysicalDevicePipelineBinaryFeaturesKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPhysicalDevicePipelineBinaryPropertiesKHR;
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
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDevicePipelineRobustnessProperties;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPhysicalDevicePointClippingProperties;
#[cfg(feature = "VK_KHR_portability_subset")]
use crate::types::VkPhysicalDevicePortabilitySubsetFeaturesKHR;
#[cfg(feature = "VK_KHR_portability_subset")]
use crate::types::VkPhysicalDevicePortabilitySubsetPropertiesKHR;
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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDeviceProperties;
#[cfg(feature = "VK_EXT_provoking_vertex")]
use crate::types::VkPhysicalDeviceProvokingVertexFeaturesEXT;
#[cfg(feature = "VK_EXT_provoking_vertex")]
use crate::types::VkPhysicalDeviceProvokingVertexPropertiesEXT;
#[cfg(feature = "VK_NV_push_constant_bank")]
use crate::types::VkPhysicalDevicePushConstantBankFeaturesNV;
#[cfg(feature = "VK_NV_push_constant_bank")]
use crate::types::VkPhysicalDevicePushConstantBankPropertiesNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDevicePushDescriptorProperties;
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
use crate::types::VkPhysicalDeviceQueuePerfHintFeaturesQCOM;
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
use crate::types::VkPhysicalDeviceQueuePerfHintPropertiesQCOM;
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
#[cfg(feature = "VK_EXT_ray_tracing_invocation_reorder")]
use crate::types::VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT;
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
use crate::types::VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV;
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
use crate::types::VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV;
#[cfg(feature = "VK_KHR_ray_tracing_maintenance1")]
use crate::types::VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkPhysicalDeviceRayTracingMotionBlurFeaturesNV;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkPhysicalDeviceRayTracingPipelineFeaturesKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkPhysicalDeviceRayTracingPipelinePropertiesKHR;
#[cfg(feature = "VK_KHR_ray_tracing_position_fetch")]
use crate::types::VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkPhysicalDeviceRayTracingPropertiesNV;
#[cfg(feature = "VK_NV_ray_tracing_validation")]
use crate::types::VkPhysicalDeviceRayTracingValidationFeaturesNV;
#[cfg(feature = "VK_IMG_relaxed_line_rasterization")]
use crate::types::VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG;
#[cfg(feature = "VK_ARM_render_pass_striped")]
use crate::types::VkPhysicalDeviceRenderPassStripedFeaturesARM;
#[cfg(feature = "VK_ARM_render_pass_striped")]
use crate::types::VkPhysicalDeviceRenderPassStripedPropertiesARM;
#[cfg(feature = "VK_NV_representative_fragment_test")]
use crate::types::VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV;
#[cfg(feature = "VK_KHR_robustness2")]
use crate::types::VkPhysicalDeviceRobustness2FeaturesKHR;
#[cfg(feature = "VK_KHR_robustness2")]
use crate::types::VkPhysicalDeviceRobustness2PropertiesKHR;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkPhysicalDeviceSampleLocationsPropertiesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceSamplerFilterMinmaxProperties;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceSamplerYcbcrConversionFeatures;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceScalarBlockLayoutFeatures;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::types::VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::types::VkPhysicalDeviceSchedulingControlsFeaturesARM;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::types::VkPhysicalDeviceSchedulingControlsPropertiesARM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures;
#[cfg(feature = "VK_EXT_shader_64bit_indexing")]
use crate::types::VkPhysicalDeviceShader64BitIndexingFeaturesEXT;
#[cfg(feature = "VK_KHR_shader_abort")]
use crate::types::VkPhysicalDeviceShaderAbortFeaturesKHR;
#[cfg(feature = "VK_KHR_shader_abort")]
use crate::types::VkPhysicalDeviceShaderAbortPropertiesKHR;
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
#[cfg(feature = "VK_ARM_shader_core_builtins")]
use crate::types::VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM;
#[cfg(feature = "VK_AMD_shader_core_properties2")]
use crate::types::VkPhysicalDeviceShaderCoreProperties2AMD;
#[cfg(feature = "VK_AMD_shader_core_properties")]
use crate::types::VkPhysicalDeviceShaderCorePropertiesAMD;
#[cfg(feature = "VK_ARM_shader_core_properties")]
use crate::types::VkPhysicalDeviceShaderCorePropertiesARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPhysicalDeviceShaderDrawParametersFeatures;
#[cfg(feature = "VK_AMD_shader_early_and_late_fragment_tests")]
use crate::types::VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkPhysicalDeviceShaderEnqueueFeaturesAMDX;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkPhysicalDeviceShaderEnqueuePropertiesAMDX;
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
#[cfg(feature = "VK_ARM_shader_instrumentation")]
use crate::types::VkPhysicalDeviceShaderInstrumentationPropertiesARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceShaderIntegerDotProductFeatures;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceShaderIntegerDotProductProperties;
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
use crate::types::VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
#[cfg(feature = "VK_EXT_shader_long_vector")]
use crate::types::VkPhysicalDeviceShaderLongVectorFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_long_vector")]
use crate::types::VkPhysicalDeviceShaderLongVectorPropertiesEXT;
#[cfg(feature = "VK_KHR_shader_maximal_reconvergence")]
use crate::types::VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR;
#[cfg(feature = "VK_VALVE_shader_mixed_float_dot_product")]
use crate::types::VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE;
#[cfg(feature = "VK_EXT_shader_module_identifier")]
use crate::types::VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_module_identifier")]
use crate::types::VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT;
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
use crate::types::VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM;
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
use crate::types::VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM;
#[cfg(feature = "VK_EXT_shader_ocp_microscaling_types")]
use crate::types::VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_object")]
use crate::types::VkPhysicalDeviceShaderObjectFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_object")]
use crate::types::VkPhysicalDeviceShaderObjectPropertiesEXT;
#[cfg(feature = "VK_KHR_shader_quad_control")]
use crate::types::VkPhysicalDeviceShaderQuadControlFeaturesKHR;
#[cfg(feature = "VK_KHR_shader_relaxed_extended_instruction")]
use crate::types::VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR;
#[cfg(feature = "VK_EXT_shader_replicated_composites")]
use crate::types::VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT;
#[cfg(feature = "VK_NV_shader_sm_builtins")]
use crate::types::VkPhysicalDeviceShaderSMBuiltinsFeaturesNV;
#[cfg(feature = "VK_NV_shader_sm_builtins")]
use crate::types::VkPhysicalDeviceShaderSMBuiltinsPropertiesNV;
#[cfg(feature = "VK_EXT_shader_split_barrier")]
use crate::types::VkPhysicalDeviceShaderSplitBarrierFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_split_barrier")]
use crate::types::VkPhysicalDeviceShaderSplitBarrierPropertiesEXT;
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
#[cfg(feature = "VK_EXT_shader_tile_image")]
use crate::types::VkPhysicalDeviceShaderTileImagePropertiesEXT;
#[cfg(feature = "VK_EXT_shader_uniform_buffer_unsized_array")]
use crate::types::VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT;
#[cfg(feature = "VK_KHR_shader_untyped_pointers")]
use crate::types::VkPhysicalDeviceShaderUntypedPointersFeaturesKHR;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkPhysicalDeviceShadingRateImageFeaturesNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkPhysicalDeviceShadingRateImagePropertiesNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceSubgroupProperties;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceSubgroupSizeControlFeatures;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceSubgroupSizeControlProperties;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::types::VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT;
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
use crate::types::VkPhysicalDeviceSubpassShadingFeaturesHUAWEI;
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
use crate::types::VkPhysicalDeviceSubpassShadingPropertiesHUAWEI;
#[cfg(feature = "VK_KHR_swapchain_maintenance1")]
use crate::types::VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceSynchronization2Features;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkPhysicalDeviceTensorFeaturesARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkPhysicalDeviceTensorPropertiesARM;
#[cfg(feature = "VK_EXT_texel_buffer_alignment")]
use crate::types::VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceTexelBufferAlignmentProperties;
#[cfg(feature = "VK_EXT_texture_compression_astc_3d")]
use crate::types::VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceTextureCompressionASTCHDRFeatures;
#[cfg(feature = "VK_SEC_throttle_hint")]
use crate::types::VkPhysicalDeviceThrottleHintFeaturesSEC;
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
use crate::types::VkPhysicalDeviceTileMemoryHeapFeaturesQCOM;
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
use crate::types::VkPhysicalDeviceTileMemoryHeapPropertiesQCOM;
#[cfg(feature = "VK_QCOM_tile_properties")]
use crate::types::VkPhysicalDeviceTilePropertiesFeaturesQCOM;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkPhysicalDeviceTileShadingFeaturesQCOM;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkPhysicalDeviceTileShadingPropertiesQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceTimelineSemaphoreFeatures;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceTimelineSemaphoreProperties;
#[cfg(feature = "VK_EXT_transform_feedback")]
use crate::types::VkPhysicalDeviceTransformFeedbackFeaturesEXT;
#[cfg(feature = "VK_EXT_transform_feedback")]
use crate::types::VkPhysicalDeviceTransformFeedbackPropertiesEXT;
#[cfg(feature = "VK_KHR_unified_image_layouts")]
use crate::types::VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceUniformBufferStandardLayoutFeatures;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceVariablePointersFeatures;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceVertexAttributeDivisorFeatures;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceVertexAttributeDivisorProperties;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
use crate::types::VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT;
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
use crate::types::VkPhysicalDeviceVulkan11Properties;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceVulkan12Features;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceVulkan12Properties;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceVulkan13Features;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceVulkan13Properties;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceVulkan14Features;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceVulkan14Properties;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceVulkanMemoryModelFeatures;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPhysicalDeviceVulkanSC10Features;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPhysicalDeviceVulkanSC10Properties;
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
#[cfg(any(
  all(
    feature = "VK_NV_device_diagnostic_checkpoints",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_synchronization2",
    feature = "VK_NV_device_diagnostic_checkpoints"
  )
))]
use crate::types::VkQueueFamilyCheckpointProperties2NV;
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
use crate::types::VkQueueFamilyCheckpointPropertiesNV;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkQueueFamilyGlobalPriorityProperties;
#[cfg(feature = "VK_KHR_maintenance11")]
use crate::types::VkQueueFamilyOptimalImageTransferGranularityPropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance9")]
use crate::types::VkQueueFamilyOwnershipTransferPropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueueFamilyProperties;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkQueueFamilyQueryResultStatusPropertiesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkQueueFamilyVideoPropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkRect2D;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkSamplerYcbcrConversionImageFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSemaphoreCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkSemaphoreTypeCreateInfo;
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
use crate::types::VkSparseImageFormatProperties;
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
use crate::types::VkSparseImageMemoryRequirements;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSubmitInfo;
#[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
use crate::types::VkSubpassResolvePerformanceQueryEXT;
#[cfg(all(
  feature = "VK_EXT_descriptor_heap",
  feature = "VK_EXT_fragment_density_map"
))]
use crate::types::VkSubsampledImageFormatPropertiesEXT;
#[cfg(feature = "VK_ARM_tensor_controls")]
use crate::types::VkTensorExplicitTilingFormatPropertiesARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorFormatPropertiesARM;
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
use crate::types::VkTextureLODGatherFormatPropertiesAMD;
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
use crate::types::VkTileMemoryRequirementsQCOM;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoProfileListInfoKHR;
use core::ffi::c_void;
/// [VkSubgroupFeatureFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubgroupFeatureFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type VkSubgroupFeatureFlags = VkSubgroupFeatureFlagBits;
/// [VkPeerMemoryFeatureFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlagBits;
/// [VkMemoryAllocateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type VkMemoryAllocateFlags = VkMemoryAllocateFlagBits;
/// [VkCommandPoolTrimFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolTrimFlags.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
pub type VkCommandPoolTrimFlags = VkFlags;
/// [VkExternalMemoryHandleTypeFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlagBits;
/// [VkExternalMemoryFeatureFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type VkExternalMemoryFeatureFlags = VkExternalMemoryFeatureFlagBits;
/// [VkExternalSemaphoreHandleTypeFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlagBits;
/// [VkExternalSemaphoreFeatureFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type VkExternalSemaphoreFeatureFlags = VkExternalSemaphoreFeatureFlagBits;
/// [VkSemaphoreImportFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type VkSemaphoreImportFlags = VkSemaphoreImportFlagBits;
/// [VkExternalFenceHandleTypeFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlagBits;
/// [VkExternalFenceFeatureFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type VkExternalFenceFeatureFlags = VkExternalFenceFeatureFlagBits;
/// [VkFenceImportFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type VkFenceImportFlags = VkFenceImportFlagBits;
/// [VkPhysicalDeviceFeatures2](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFeatures2.html)
///
/// **Extends:** VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceFeatures2<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub features: VkPhysicalDeviceFeatures,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceFeatures2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceFeatures2<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceFeatures2<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceFeatures2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_FEATURES_2,
    pNext: core::ptr::null_mut(),
    features: VkPhysicalDeviceFeatures::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_features(mut self, val: VkPhysicalDeviceFeatures) -> Self {
    self.features = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevice16BitStorageFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDevice16BitStorageFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevice16BitStorageFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_4444_formats")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevice4444FormatsFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDevice4444FormatsFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevice4444FormatsFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevice8BitStorageFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDevice8BitStorageFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevice8BitStorageFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_astc_decode_mode")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceASTCDecodeFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceASTCDecodeFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceASTCDecodeFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAccelerationStructureFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceAccelerationStructureFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceAccelerationStructureFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_device_address_binding_report")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAddressBindingReportFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceAddressBindingReportFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceAddressBindingReportFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_SEC_amigo_profiling")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAmigoProfilingFeaturesSEC<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceAmigoProfilingFeaturesSEC<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceAmigoProfilingFeaturesSEC<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_anti_lag")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAntiLagFeaturesAMD<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceAntiLagFeaturesAMD<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceAntiLagFeaturesAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_attachment_feedback_loop_dynamic_state")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val
      as *mut VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'child>)
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
    val: &'a mut VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT<'child>)
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
    val: &'a mut VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_border_color_swizzle")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceBorderColorSwizzleFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceBorderColorSwizzleFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceBorderColorSwizzleFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceBufferDeviceAddressFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceBufferDeviceAddressFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceBufferDeviceAddressFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_buffer_device_address")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceBufferDeviceAddressFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceBufferDeviceAddressFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceBufferDeviceAddressFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_device_coherent_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCoherentMemoryFeaturesAMD<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCoherentMemoryFeaturesAMD<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceCoherentMemoryFeaturesAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_color_write_enable")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceColorWriteEnableFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceColorWriteEnableFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceColorWriteEnableFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_command_buffer_inheritance")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_compute_occupancy_priority")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_compute_shader_derivatives")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_conditional_rendering")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceConditionalRenderingFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceConditionalRenderingFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceConditionalRenderingFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cooperative_matrix2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_cooperative_matrix_conversion")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM<'child>)
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
    val: &'a mut VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'child>)
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
    val: &'a mut VkPhysicalDeviceCooperativeMatrixFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCooperativeMatrixFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cooperative_matrix")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrixFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCooperativeMatrixFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceCooperativeMatrixFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_cooperative_matrix_maintenance1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cooperative_vector")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeVectorFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCooperativeVectorFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceCooperativeVectorFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_copy_memory_indirect")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_copy_memory_indirect")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_corner_sampled_image")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCornerSampledImageFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCornerSampledImageFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCornerSampledImageFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_coverage_reduction_mode")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCoverageReductionModeFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCoverageReductionModeFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCoverageReductionModeFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_filter_cubic_clamp")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCubicClampFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCubicClampFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceCubicClampFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_filter_cubic_weights")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCubicWeightsFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCubicWeightsFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceCubicWeightsFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_custom_border_color")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCustomBorderColorFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCustomBorderColorFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCustomBorderColorFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_custom_resolve")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCustomResolveFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCustomResolveFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceCustomResolveFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDataGraphFeaturesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDataGraphFeaturesARM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDataGraphFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_data_graph_model")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDataGraphModelFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDataGraphModelFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDataGraphModelFeaturesQCOM<'child>).cast::<c_void>();
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
    val: &'a mut VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'child>,
  ) -> Self {
    self.pNext = (val
      as *mut VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'child>)
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
    val: &'a mut VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'child>)
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
    val: &'a mut VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_depth_bias_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDepthBiasControlFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDepthBiasControlFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDepthBiasControlFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_depth_clamp_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDepthClampControlFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDepthClampControlFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDepthClampControlFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_depth_clamp_zero_one")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDepthClampZeroOneFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDepthClampZeroOneFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDepthClampZeroOneFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_depth_clip_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDepthClipControlFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDepthClipControlFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDepthClipControlFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_depth_clip_enable")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDepthClipEnableFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDepthClipEnableFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDepthClipEnableFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorBufferFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorBufferFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDescriptorBufferFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorHeapFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorHeapFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDescriptorHeapFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorIndexingFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorIndexingFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDescriptorIndexingFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_descriptor_pool_overallocation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_device_address_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands_compute")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'child>)
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
    val: &'a mut VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_device_memory_report")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDeviceMemoryReportFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDeviceMemoryReportFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDeviceMemoryReportFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_device_diagnostics_config")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_displacement_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDisplacementMicromapFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDisplacementMicromapFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDisplacementMicromapFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDynamicRenderingFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDynamicRenderingFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDynamicRenderingFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_dynamic_rendering_unused_attachments")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'child>)
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
    val: &'a mut VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_scissor_exclusive")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExclusiveScissorFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExclusiveScissorFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceExclusiveScissorFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_extended_dynamic_state2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedDynamicState2FeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExtendedDynamicState2FeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExtendedDynamicState2FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_extended_dynamic_state3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedDynamicState3FeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExtendedDynamicState3FeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExtendedDynamicState3FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_extended_dynamic_state")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedDynamicStateFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExtendedDynamicStateFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExtendedDynamicStateFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_extended_flags")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedFlagsFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExtendedFlagsFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceExtendedFlagsFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_extended_sparse_address_space")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ANDROID_external_format_resolve")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExternalFormatResolveFeaturesANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_memory_rdma")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_sci_sync2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalSciSync2FeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExternalSciSync2FeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceExternalSciSync2FeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_sci_sync")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalSciSyncFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExternalSciSyncFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceExternalSciSyncFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_device_fault")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFaultFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFaultFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceFaultFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_device_fault")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFaultFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFaultFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceFaultFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_format_pack")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFormatPackFeaturesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFormatPackFeaturesARM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceFormatPackFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_density_map2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMap2FeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentDensityMap2FeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentDensityMap2FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_density_map")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMapFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentDensityMapFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentDensityMapFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_density_map_offset")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_fragment_shader_barycentric")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_shader_interlock")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_fragment_shading_rate")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentShadingRateFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentShadingRateFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentShadingRateFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_frame_boundary")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFrameBoundaryFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFrameBoundaryFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceFrameBoundaryFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceGlobalPriorityQueryFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceGlobalPriorityQueryFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceGlobalPriorityQueryFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_gpa_interface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceGpaFeaturesAMD<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceGpaFeaturesAMD<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceGpaFeaturesAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_graphics_pipeline_library")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_HUAWEI_hdr_vivid")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceHdrVividFeaturesHUAWEI<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceHdrVividFeaturesHUAWEI<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceHdrVividFeaturesHUAWEI<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceHostImageCopyFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceHostImageCopyFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceHostImageCopyFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceHostQueryResetFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceHostQueryResetFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceHostQueryResetFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_2d_view_of_3d")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImage2DViewOf3DFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImage2DViewOf3DFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceImage2DViewOf3DFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_MESA_image_alignment_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_compression_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageCompressionControlFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageCompressionControlFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceImageCompressionControlFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_compression_control_swapchain")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'child>)
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
    val: &'a mut VkPhysicalDeviceImageProcessing2FeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceImageProcessing2FeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_image_processing3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageProcessing3FeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageProcessing3FeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceImageProcessing3FeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_image_processing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageProcessingFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageProcessingFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceImageProcessingFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageRobustnessFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageRobustnessFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceImageRobustnessFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_sliced_view_of_3d")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_tiling_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageTilingControlFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageTilingControlFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceImageTilingControlFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_view_min_lod")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageViewMinLodFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageViewMinLodFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceImageViewMinLodFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImagelessFramebufferFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImagelessFramebufferFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceImagelessFramebufferFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceIndexTypeUint8Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceIndexTypeUint8Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceIndexTypeUint8Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_inherited_viewport_scissor")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceInlineUniformBlockFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceInlineUniformBlockFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceInlineUniformBlockFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_internally_synchronized_queues")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'child>)
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
    val: &'a mut VkPhysicalDeviceInvocationMaskFeaturesHUAWEI<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceInvocationMaskFeaturesHUAWEI<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_legacy_dithering")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLegacyDitheringFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceLegacyDitheringFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceLegacyDitheringFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_legacy_vertex_attributes")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLineRasterizationFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceLineRasterizationFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceLineRasterizationFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_linear_color_attachment")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance10")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance10FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance10FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance10FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance11")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance11FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance11FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance11FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance4Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance4Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance4Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance5Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance5Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance5Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance6Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance6Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance6Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance7")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance7FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance7FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance7FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance8")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance8FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance8FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance8FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance9")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance9FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance9FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance9FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_map_memory_placed")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMapMemoryPlacedFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMapMemoryPlacedFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMapMemoryPlacedFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_memory_decompression")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMemoryDecompressionFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMemoryDecompressionFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceMemoryDecompressionFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_memory_priority")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMemoryPriorityFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMemoryPriorityFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMemoryPriorityFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_mesh_shader")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMeshShaderFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMeshShaderFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMeshShaderFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_mesh_shader")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMeshShaderFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMeshShaderFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMeshShaderFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_multi_draw")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultiDrawFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMultiDrawFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMultiDrawFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'child>)
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
    val: &'a mut VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'child>)
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
    val: &'a mut VkPhysicalDeviceMultiviewFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMultiviewFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'child>)
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
    val: &'a mut VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_mutable_descriptor_type")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_nested_command_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceNestedCommandBufferFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceNestedCommandBufferFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceNestedCommandBufferFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_non_seamless_cube_map")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceOpacityMicromapFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceOpacityMicromapFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceOpacityMicromapFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_opacity_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceOpacityMicromapFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceOpacityMicromapFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceOpacityMicromapFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceOpticalFlowFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceOpticalFlowFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceOpticalFlowFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'child>)
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
    val: &'a mut VkPhysicalDevicePerStageDescriptorSetFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePerStageDescriptorSetFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_performance_counters_by_region")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePerformanceQueryFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePerformanceQueryFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePerformanceQueryFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineBinaryFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePipelineBinaryFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePipelineBinaryFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_SEC_pipeline_cache_incremental_mode")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'child>)
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
    val: &'a mut VkPhysicalDevicePipelineCreationCacheControlFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePipelineCreationCacheControlFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'child>)
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
    val: &'a mut VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_pipeline_opacity_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_pipeline_properties")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelinePropertiesFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePipelinePropertiesFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePipelinePropertiesFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineProtectedAccessFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePipelineProtectedAccessFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePipelineProtectedAccessFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineRobustnessFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePipelineRobustnessFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePipelineRobustnessFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_portability_subset")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePortabilitySubsetFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePortabilitySubsetFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePortabilitySubsetFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_present_barrier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentBarrierFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePresentBarrierFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePresentBarrierFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_present_id2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentId2FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePresentId2FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePresentId2FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_present_id")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentIdFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePresentIdFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePresentIdFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_present_metering")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentMeteringFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePresentMeteringFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePresentMeteringFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_present_mode_fifo_latest_ready")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_present_timing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentTimingFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePresentTimingFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePresentTimingFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_present_wait2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentWait2FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePresentWait2FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePresentWait2FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_present_wait")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePresentWaitFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePresentWaitFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePresentWaitFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_primitive_restart_index")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'child>)
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
    val: &'a mut VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePrivateDataFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePrivateDataFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePrivateDataFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceProtectedMemoryFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceProtectedMemoryFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceProtectedMemoryFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_provoking_vertex")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceProvokingVertexFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceProvokingVertexFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceProvokingVertexFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_push_constant_bank")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePushConstantBankFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePushConstantBankFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePushConstantBankFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_queue_perf_hint")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceQueuePerfHintFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_rgba10x6_formats")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_rasterization_order_attachment_access")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val
      as *mut VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'child>)
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
    val: &'a mut VkPhysicalDeviceRawAccessChainsFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceRawAccessChainsFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_ray_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayQueryFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayQueryFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceRayQueryFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_ray_tracing_invocation_reorder")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_ray_tracing_maintenance1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingPipelineFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingPipelineFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRayTracingPipelineFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_ray_tracing_position_fetch")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing_validation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingValidationFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingValidationFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRayTracingValidationFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_IMG_relaxed_line_rasterization")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_render_pass_striped")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRenderPassStripedFeaturesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRenderPassStripedFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRenderPassStripedFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_representative_fragment_test")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_robustness2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRobustness2FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRobustness2FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceRobustness2FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSamplerYcbcrConversionFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSamplerYcbcrConversionFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceSamplerYcbcrConversionFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceScalarBlockLayoutFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceScalarBlockLayoutFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceScalarBlockLayoutFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_scheduling_controls")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSchedulingControlsFeaturesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSchedulingControlsFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceSchedulingControlsFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_64bit_indexing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShader64BitIndexingFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShader64BitIndexingFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShader64BitIndexingFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_abort")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderAbortFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderAbortFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderAbortFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_shader_atomic_float16_vector")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_atomic_float2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_atomic_float")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderAtomicFloatFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderAtomicFloatFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderAtomicFloatFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderAtomicInt64Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderAtomicInt64Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderAtomicInt64Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_bfloat16")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderBfloat16FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderBfloat16FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderBfloat16FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_clock")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderClockFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderClockFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderClockFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_constant_data")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderConstantDataFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderConstantDataFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderConstantDataFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_shader_core_builtins")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderDrawParametersFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderDrawParametersFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderDrawParametersFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_shader_early_and_late_fragment_tests")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'child>)
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
    val: &'a mut VkPhysicalDeviceShaderEnqueueFeaturesAMDX<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderEnqueueFeaturesAMDX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderExpectAssumeFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderExpectAssumeFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderExpectAssumeFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderFloat16Int8Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderFloat16Int8Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderFloat16Int8Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_float8")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderFloat8FeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderFloat8FeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderFloat8FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderFloatControls2Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderFloatControls2Features<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderFloatControls2Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_fma")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderFmaFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderFmaFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderFmaFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_shader_image_footprint")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderImageFootprintFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderImageFootprintFeaturesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderImageFootprintFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderInstrumentationFeaturesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderInstrumentationFeaturesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderInstrumentationFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderIntegerDotProductFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderIntegerDotProductFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderIntegerDotProductFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_INTEL_shader_integer_functions2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_long_vector")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderLongVectorFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderLongVectorFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderLongVectorFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_maximal_reconvergence")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_VALVE_shader_mixed_float_dot_product")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'child>)
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
    val: &'a mut VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_ocp_microscaling_types")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_object")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderObjectFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderObjectFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderObjectFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_quad_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderQuadControlFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderQuadControlFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderQuadControlFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_relaxed_extended_instruction")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR<'child>)
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
    val: &'a mut VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_shader_sm_builtins")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_split_barrier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSplitBarrierFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderSplitBarrierFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderSplitBarrierFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_subgroup_partitioned")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSubgroupPartitionedFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderSubgroupPartitionedFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderSubgroupPartitionedFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSubgroupRotateFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderSubgroupRotateFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderSubgroupRotateFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'child>)
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
    val: &'a mut VkPhysicalDeviceShaderTerminateInvocationFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderTerminateInvocationFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_tile_image")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderTileImageFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderTileImageFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderTileImageFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_uniform_buffer_unsized_array")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT<'child>)
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
    val: &'a mut VkPhysicalDeviceShaderUntypedPointersFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderUntypedPointersFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_shading_rate_image")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShadingRateImageFeaturesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShadingRateImageFeaturesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShadingRateImageFeaturesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSubgroupSizeControlFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSubgroupSizeControlFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceSubgroupSizeControlFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_subpass_merge_feedback")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_HUAWEI_subpass_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceSubpassShadingFeaturesHUAWEI<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_swapchain_maintenance1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSynchronization2Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSynchronization2Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceSynchronization2Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTensorFeaturesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTensorFeaturesARM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceTensorFeaturesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_texel_buffer_alignment")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_texture_compression_astc_3d")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceTextureCompressionASTCHDRFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_SEC_throttle_hint")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceThrottleHintFeaturesSEC<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceThrottleHintFeaturesSEC<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceThrottleHintFeaturesSEC<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_memory_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceTileMemoryHeapFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_properties")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTilePropertiesFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTilePropertiesFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceTilePropertiesFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTileShadingFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTileShadingFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceTileShadingFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTimelineSemaphoreFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTimelineSemaphoreFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceTimelineSemaphoreFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_transform_feedback")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTransformFeedbackFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTransformFeedbackFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceTransformFeedbackFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_unified_image_layouts")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVariablePointersFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVariablePointersFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVariablePointersFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVertexAttributeDivisorFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVertexAttributeDivisorFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceVertexAttributeDivisorFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_vertex_attribute_robustness")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_decode_vp9")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoDecodeVP9FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVideoDecodeVP9FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVideoDecodeVP9FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_av1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoEncodeAV1FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVideoEncodeAV1FeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVideoEncodeAV1FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_feedback2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_encode_quantization_map")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_maintenance1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoMaintenance1FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVideoMaintenance1FeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceVideoMaintenance1FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_maintenance2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVideoMaintenance2FeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVideoMaintenance2FeaturesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceVideoMaintenance2FeaturesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan11Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVulkan11Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVulkan11Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan12Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVulkan12Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVulkan12Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan13Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVulkan13Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVulkan13Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan14Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVulkan14Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVulkan14Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkanMemoryModelFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVulkanMemoryModelFeatures<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVulkanMemoryModelFeatures<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkanSC10Features<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVulkanSC10Features<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVulkanSC10Features<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'child>)
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
    val: &'a mut VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_ycbcr_degamma")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceYcbcrDegammaFeaturesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceYcbcrImageArraysFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceYcbcrImageArraysFeaturesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceYcbcrImageArraysFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_zero_initialize_device_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'child>).cast::<c_void>();
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
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceProperties2](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceProperties2.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceProperties2<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Struct]
  pub properties: VkPhysicalDeviceProperties,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceProperties2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceProperties2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceProperties2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PROPERTIES_2,
    pNext: core::ptr::null_mut(),
    properties: VkPhysicalDeviceProperties::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_properties(mut self, val: VkPhysicalDeviceProperties) -> Self {
    self.properties = val;
    self
  }
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceAccelerationStructurePropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceAccelerationStructurePropertiesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceAccelerationStructurePropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_blend_operation_advanced")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceClusterAccelerationStructurePropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceClusterAccelerationStructurePropertiesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceClusterAccelerationStructurePropertiesNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_compute_shader_derivatives")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceComputeShaderDerivativesPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceComputeShaderDerivativesPropertiesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceComputeShaderDerivativesPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_conservative_rasterization")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceConservativeRasterizationPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceConservativeRasterizationPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceConservativeRasterizationPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cooperative_matrix2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrix2PropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCooperativeMatrix2PropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCooperativeMatrix2PropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_cooperative_matrix")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrixPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCooperativeMatrixPropertiesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCooperativeMatrixPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cooperative_matrix")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeMatrixPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCooperativeMatrixPropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCooperativeMatrixPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cooperative_vector")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCooperativeVectorPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCooperativeVectorPropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCooperativeVectorPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_copy_memory_indirect")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCudaKernelLaunchPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCudaKernelLaunchPropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCudaKernelLaunchPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_custom_border_color")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceCustomBorderColorPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceCustomBorderColorPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceCustomBorderColorPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDepthStencilResolveProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDepthStencilResolveProperties<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDepthStencilResolveProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(
    feature = "VK_EXT_descriptor_buffer",
    feature = "VK_EXT_fragment_density_map"
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorBufferPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorBufferPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDescriptorBufferPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorBufferTensorPropertiesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorBufferTensorPropertiesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDescriptorBufferTensorPropertiesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorHeapPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorHeapPropertiesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDescriptorHeapPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_heap"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorHeapTensorPropertiesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorHeapTensorPropertiesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDescriptorHeapTensorPropertiesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDescriptorIndexingProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDescriptorIndexingProperties<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDescriptorIndexingProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDeviceGeneratedCommandsPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDeviceGeneratedCommandsPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDeviceGeneratedCommandsPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_discard_rectangles")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDiscardRectanglePropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDiscardRectanglePropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDiscardRectanglePropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_displacement_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDisplacementMicromapPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDisplacementMicromapPropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceDisplacementMicromapPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDriverProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDriverProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDriverProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_physical_device_drm")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceDrmPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceDrmPropertiesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceDrmPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_extended_dynamic_state3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedDynamicState3PropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExtendedDynamicState3PropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExtendedDynamicState3PropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_extended_sparse_address_space")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_external_compute_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalComputeQueuePropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExternalComputeQueuePropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExternalComputeQueuePropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ANDROID_external_format_resolve")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalFormatResolvePropertiesANDROID<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExternalFormatResolvePropertiesANDROID<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExternalFormatResolvePropertiesANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_external_memory_host")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalMemoryHostPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceExternalMemoryHostPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceExternalMemoryHostPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_device_fault")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFaultPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFaultPropertiesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceFaultPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFloatControlsProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFloatControlsProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceFloatControlsProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_density_map2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMap2PropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentDensityMap2PropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentDensityMap2PropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_density_map_offset")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_density_map")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentDensityMapPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentDensityMapPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentDensityMapPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(
    feature = "VK_EXT_provoking_vertex",
    feature = "VK_KHR_fragment_shader_barycentric"
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_fragment_shading_rate")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceFragmentShadingRatePropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceFragmentShadingRatePropertiesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceFragmentShadingRatePropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_gpa_interface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceGpaProperties2AMD<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceGpaProperties2AMD<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceGpaProperties2AMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_gpa_interface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceGpaPropertiesAMD<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceGpaPropertiesAMD<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceGpaPropertiesAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_graphics_pipeline_library")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceHostImageCopyProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceHostImageCopyProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceHostImageCopyProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceIDProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceIDProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceIDProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_MESA_image_alignment_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageAlignmentControlPropertiesMESA<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageAlignmentControlPropertiesMESA<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceImageAlignmentControlPropertiesMESA<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_image_processing2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageProcessing2PropertiesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageProcessing2PropertiesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceImageProcessing2PropertiesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_image_processing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageProcessingPropertiesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceImageProcessingPropertiesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceImageProcessingPropertiesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceInlineUniformBlockProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceInlineUniformBlockProperties<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceInlineUniformBlockProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance7")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLayeredApiPropertiesListKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceLayeredApiPropertiesListKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceLayeredApiPropertiesListKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_MSFT_layered_driver")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLayeredDriverPropertiesMSFT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceLayeredDriverPropertiesMSFT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceLayeredDriverPropertiesMSFT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_legacy_vertex_attributes")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLegacyVertexAttributesPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceLegacyVertexAttributesPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceLegacyVertexAttributesPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceLineRasterizationProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceLineRasterizationProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceLineRasterizationProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance10")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance10PropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance10PropertiesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance10PropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance3Properties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance3Properties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance3Properties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance4Properties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance4Properties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance4Properties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance5Properties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance5Properties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance5Properties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance6Properties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance6Properties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance6Properties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance7")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance7PropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance7PropertiesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance7PropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance9")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMaintenance9PropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMaintenance9PropertiesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMaintenance9PropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_map_memory_placed")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMapMemoryPlacedPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMapMemoryPlacedPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceMapMemoryPlacedPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_memory_decompression")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMemoryDecompressionPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMemoryDecompressionPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceMemoryDecompressionPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_mesh_shader")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMeshShaderPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMeshShaderPropertiesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMeshShaderPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_mesh_shader")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMeshShaderPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMeshShaderPropertiesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMeshShaderPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_multi_draw")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultiDrawPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMultiDrawPropertiesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMultiDrawPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMultiviewProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMultiviewProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMultiviewProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_nested_command_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceNestedCommandBufferPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceNestedCommandBufferPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceNestedCommandBufferPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceOpacityMicromapPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceOpacityMicromapPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceOpacityMicromapPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_opacity_micromap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceOpacityMicromapPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceOpacityMicromapPropertiesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceOpacityMicromapPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceOpticalFlowPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceOpticalFlowPropertiesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceOpticalFlowPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_pci_bus_info")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePCIBusInfoPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePCIBusInfoPropertiesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePCIBusInfoPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_performance_counters_by_region")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePerformanceCountersByRegionPropertiesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePerformanceCountersByRegionPropertiesARM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePerformanceCountersByRegionPropertiesARM<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_performance_query")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePerformanceQueryPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePerformanceQueryPropertiesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePerformanceQueryPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineBinaryPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePipelineBinaryPropertiesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePipelineBinaryPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePipelineRobustnessProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePipelineRobustnessProperties<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePipelineRobustnessProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePointClippingProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePointClippingProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePointClippingProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_portability_subset")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePortabilitySubsetPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePortabilitySubsetPropertiesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePortabilitySubsetPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceProtectedMemoryProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceProtectedMemoryProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceProtectedMemoryProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_provoking_vertex")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceProvokingVertexPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceProvokingVertexPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceProvokingVertexPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_push_constant_bank")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePushConstantBankPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePushConstantBankPropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDevicePushConstantBankPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDevicePushDescriptorProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDevicePushDescriptorProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDevicePushDescriptorProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_queue_perf_hint")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceQueuePerfHintPropertiesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceQueuePerfHintPropertiesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceQueuePerfHintPropertiesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_ray_tracing_invocation_reorder")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingPipelinePropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingPipelinePropertiesKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRayTracingPipelinePropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRayTracingPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRayTracingPropertiesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceRayTracingPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_render_pass_striped")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRenderPassStripedPropertiesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRenderPassStripedPropertiesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceRenderPassStripedPropertiesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_robustness2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceRobustness2PropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceRobustness2PropertiesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceRobustness2PropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_sample_locations")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSampleLocationsPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSampleLocationsPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceSampleLocationsPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSamplerFilterMinmaxProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSamplerFilterMinmaxProperties<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceSamplerFilterMinmaxProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_scheduling_controls")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<
    'child,
  >(
    mut self,
    val: &'a mut VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'child>,
  ) -> Self {
    self.pNext = (val
      as *mut VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_scheduling_controls")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSchedulingControlsPropertiesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSchedulingControlsPropertiesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceSchedulingControlsPropertiesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_shader_abort")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderAbortPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderAbortPropertiesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderAbortPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_shader_core_builtins")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_shader_core_properties2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderCoreProperties2AMD<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderCoreProperties2AMD<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderCoreProperties2AMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_shader_core_properties")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderCorePropertiesAMD<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderCorePropertiesAMD<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderCorePropertiesAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_shader_core_properties")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderCorePropertiesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderCorePropertiesARM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderCorePropertiesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMDX_shader_enqueue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderEnqueuePropertiesAMDX<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderEnqueuePropertiesAMDX<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderEnqueuePropertiesAMDX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderInstrumentationPropertiesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderInstrumentationPropertiesARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderInstrumentationPropertiesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderIntegerDotProductProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderIntegerDotProductProperties<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderIntegerDotProductProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_long_vector")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderLongVectorPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderLongVectorPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderLongVectorPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_module_identifier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_object")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderObjectPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderObjectPropertiesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceShaderObjectPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_shader_sm_builtins")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSMBuiltinsPropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderSMBuiltinsPropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderSMBuiltinsPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_split_barrier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderSplitBarrierPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderSplitBarrierPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderSplitBarrierPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_shader_tile_image")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShaderTileImagePropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShaderTileImagePropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShaderTileImagePropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_shading_rate_image")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceShadingRateImagePropertiesNV<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceShadingRateImagePropertiesNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceShadingRateImagePropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSubgroupProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSubgroupProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceSubgroupProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSubgroupSizeControlProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSubgroupSizeControlProperties<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceSubgroupSizeControlProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_HUAWEI_subpass_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceSubpassShadingPropertiesHUAWEI<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceSubpassShadingPropertiesHUAWEI<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceSubpassShadingPropertiesHUAWEI<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTensorPropertiesARM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTensorPropertiesARM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceTensorPropertiesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTexelBufferAlignmentProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTexelBufferAlignmentProperties<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceTexelBufferAlignmentProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_memory_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTileMemoryHeapPropertiesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTileMemoryHeapPropertiesQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceTileMemoryHeapPropertiesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTileShadingPropertiesQCOM<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTileShadingPropertiesQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceTileShadingPropertiesQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTimelineSemaphoreProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTimelineSemaphoreProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceTimelineSemaphoreProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_transform_feedback")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceTransformFeedbackPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceTransformFeedbackPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceTransformFeedbackPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVertexAttributeDivisorProperties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVertexAttributeDivisorProperties<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceVertexAttributeDivisorProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan11Properties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVulkan11Properties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVulkan11Properties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan12Properties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVulkan12Properties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVulkan12Properties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan13Properties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVulkan13Properties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVulkan13Properties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkan14Properties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVulkan14Properties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVulkan14Properties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceVulkanSC10Properties<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceVulkanSC10Properties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceVulkanSC10Properties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceProperties2<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkFormatProperties2](https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatProperties2.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFormatProperties2<'a> {
  /// Values: VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Struct]
  pub formatProperties: VkFormatProperties,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkFormatProperties2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkFormatProperties2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkFormatProperties2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::FORMAT_PROPERTIES_2,
    pNext: core::ptr::null_mut(),
    formatProperties: VkFormatProperties::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_formatProperties(mut self, val: VkFormatProperties) -> Self {
    self.formatProperties = val;
    self
  }
  #[cfg(any(
    all(
      feature = "VK_EXT_image_drm_format_modifier",
      feature = "VK_KHR_format_feature_flags2"
    ),
    all(
      feature = "VK_EXT_image_drm_format_modifier",
      feature = "VK_VERSION_1_3"
    )
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDrmFormatModifierPropertiesList2EXT<'child>(
    mut self,
    val: &'a mut VkDrmFormatModifierPropertiesList2EXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkDrmFormatModifierPropertiesList2EXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDrmFormatModifierPropertiesListEXT<'child>(
    mut self,
    val: &'a mut VkDrmFormatModifierPropertiesListEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkDrmFormatModifierPropertiesListEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFormatProperties3<'child>(
    mut self,
    val: &'a mut VkFormatProperties3<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkFormatProperties3<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_extended_flags")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFormatProperties4KHR<'child>(
    mut self,
    val: &'a mut VkFormatProperties4KHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkFormatProperties4KHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSubpassResolvePerformanceQueryEXT<'child>(
    mut self,
    val: &'a mut VkSubpassResolvePerformanceQueryEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkSubpassResolvePerformanceQueryEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_tensor_controls")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTensorExplicitTilingFormatPropertiesARM<'child>(
    mut self,
    val: &'a mut VkTensorExplicitTilingFormatPropertiesARM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkTensorExplicitTilingFormatPropertiesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTensorFormatPropertiesARM<'child>(
    mut self,
    val: &'a mut VkTensorFormatPropertiesARM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkTensorFormatPropertiesARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkFormatProperties2<
    'root,
    T: VkPNextExtends<VkFormatProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkImageFormatProperties2](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatProperties2.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageFormatProperties2<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub imageFormatProperties: VkImageFormatProperties,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkImageFormatProperties2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkImageFormatProperties2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkImageFormatProperties2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_FORMAT_PROPERTIES_2,
    pNext: core::ptr::null_mut(),
    imageFormatProperties: VkImageFormatProperties::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_imageFormatProperties(mut self, val: VkImageFormatProperties) -> Self {
    self.imageFormatProperties = val;
    self
  }
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkAndroidHardwareBufferUsageANDROID<'child>(
    mut self,
    val: &'a mut VkAndroidHardwareBufferUsageANDROID<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkAndroidHardwareBufferUsageANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalImageFormatProperties<'child>(
    mut self,
    val: &'a mut VkExternalImageFormatProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkExternalImageFormatProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_filter_cubic")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFilterCubicImageViewImageFormatPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkFilterCubicImageViewImageFormatPropertiesEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkFilterCubicImageViewImageFormatPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkHostImageCopyDevicePerformanceQuery<'child>(
    mut self,
    val: &'a mut VkHostImageCopyDevicePerformanceQuery<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkHostImageCopyDevicePerformanceQuery<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_compression_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImageCompressionPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkImageCompressionPropertiesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkImageCompressionPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_OHOS_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkNativeBufferUsageOHOS<'child>(
    mut self,
    val: &'a mut VkNativeBufferUsageOHOS<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkNativeBufferUsageOHOS<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSamplerYcbcrConversionImageFormatProperties<'child>(
    mut self,
    val: &'a mut VkSamplerYcbcrConversionImageFormatProperties<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkSamplerYcbcrConversionImageFormatProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(
    feature = "VK_EXT_descriptor_heap",
    feature = "VK_EXT_fragment_density_map"
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSubsampledImageFormatPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkSubsampledImageFormatPropertiesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkSubsampledImageFormatPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTextureLODGatherFormatPropertiesAMD<'child>(
    mut self,
    val: &'a mut VkTextureLODGatherFormatPropertiesAMD<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkTextureLODGatherFormatPropertiesAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageFormatProperties2<
    'root,
    T: VkPNextExtends<VkImageFormatProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceImageFormatInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageFormatInfo2.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceImageFormatInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub format: VkFormat,
  pub type_: VkImageType,
  pub tiling: VkImageTiling,
  pub usage: VkImageUsageFlags,
  /// Optional: true
  pub flags: VkImageCreateFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceImageFormatInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceImageFormatInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceImageFormatInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2,
    pNext: core::ptr::null(),
    format: VkFormat(0),
    type_: VkImageType(0),
    tiling: VkImageTiling(0),
    usage: VkImageUsageFlagBits(0),
    flags: VkImageCreateFlagBits(0),
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
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[inline]
  pub const fn with_type(mut self, val: VkImageType) -> Self {
    self.type_ = val;
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
  pub const fn with_flags(mut self, val: VkImageCreateFlags) -> Self {
    self.flags = val;
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
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceExternalImageFormatInfo<'child>(
    mut self,
    val: &'a VkPhysicalDeviceExternalImageFormatInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPhysicalDeviceExternalImageFormatInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageDrmFormatModifierInfoEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageDrmFormatModifierInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImageDrmFormatModifierInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_filter_cubic")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceImageViewImageFormatInfoEXT<'child>(
    mut self,
    val: &'a VkPhysicalDeviceImageViewImageFormatInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPhysicalDeviceImageViewImageFormatInfoEXT<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceImageFormatInfo2<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceImageFormatInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkQueueFamilyProperties2](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyProperties2.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueueFamilyProperties2<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Struct]
  pub queueFamilyProperties: VkQueueFamilyProperties,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkQueueFamilyProperties2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkQueueFamilyProperties2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkQueueFamilyProperties2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::QUEUE_FAMILY_PROPERTIES_2,
    pNext: core::ptr::null_mut(),
    queueFamilyProperties: VkQueueFamilyProperties::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_queueFamilyProperties(mut self, val: VkQueueFamilyProperties) -> Self {
    self.queueFamilyProperties = val;
    self
  }
  #[cfg(any(
    all(
      feature = "VK_NV_device_diagnostic_checkpoints",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_KHR_synchronization2",
      feature = "VK_NV_device_diagnostic_checkpoints"
    )
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueueFamilyCheckpointProperties2NV<'child>(
    mut self,
    val: &'a mut VkQueueFamilyCheckpointProperties2NV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkQueueFamilyCheckpointProperties2NV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueueFamilyCheckpointPropertiesNV<'child>(
    mut self,
    val: &'a mut VkQueueFamilyCheckpointPropertiesNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkQueueFamilyCheckpointPropertiesNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueueFamilyGlobalPriorityProperties<'child>(
    mut self,
    val: &'a mut VkQueueFamilyGlobalPriorityProperties<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkQueueFamilyGlobalPriorityProperties<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance11")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueueFamilyOptimalImageTransferGranularityPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkQueueFamilyOptimalImageTransferGranularityPropertiesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkQueueFamilyOptimalImageTransferGranularityPropertiesKHR<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance9")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueueFamilyOwnershipTransferPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkQueueFamilyOwnershipTransferPropertiesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkQueueFamilyOwnershipTransferPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueueFamilyQueryResultStatusPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkQueueFamilyQueryResultStatusPropertiesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkQueueFamilyQueryResultStatusPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_video_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkQueueFamilyVideoPropertiesKHR<'child>(
    mut self,
    val: &'a mut VkQueueFamilyVideoPropertiesKHR<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkQueueFamilyVideoPropertiesKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkQueueFamilyProperties2<
    'root,
    T: VkPNextExtends<VkQueueFamilyProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceMemoryProperties2](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryProperties2.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMemoryProperties2<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub memoryProperties: VkPhysicalDeviceMemoryProperties,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceMemoryProperties2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceMemoryProperties2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceMemoryProperties2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2,
    pNext: core::ptr::null_mut(),
    memoryProperties: VkPhysicalDeviceMemoryProperties::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_memoryProperties(mut self, val: VkPhysicalDeviceMemoryProperties) -> Self {
    self.memoryProperties = val;
    self
  }
  #[cfg(feature = "VK_EXT_memory_budget")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPhysicalDeviceMemoryBudgetPropertiesEXT<'child>(
    mut self,
    val: &'a mut VkPhysicalDeviceMemoryBudgetPropertiesEXT<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPhysicalDeviceMemoryBudgetPropertiesEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceMemoryProperties2<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceMemoryProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkSparseImageFormatProperties2](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatProperties2.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSparseImageFormatProperties2<'a> {
  /// Values: VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Struct]
  pub properties: VkSparseImageFormatProperties,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkSparseImageFormatProperties2<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkSparseImageFormatProperties2<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkSparseImageFormatProperties2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2,
    pNext: core::ptr::null_mut(),
    properties: VkSparseImageFormatProperties::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_properties(mut self, val: VkSparseImageFormatProperties) -> Self {
    self.properties = val;
    self
  }
  #[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSparseImageFormatProperties2<
    'root,
    T: VkPNextExtends<VkSparseImageFormatProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceSparseImageFormatInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSparseImageFormatInfo2.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub format: VkFormat,
  pub type_: VkImageType,
  pub samples: VkSampleCountFlagBits,
  pub usage: VkImageUsageFlags,
  pub tiling: VkImageTiling,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkPhysicalDeviceSparseImageFormatInfo2<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkPhysicalDeviceSparseImageFormatInfo2<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkPhysicalDeviceSparseImageFormatInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2,
    pNext: core::ptr::null(),
    format: VkFormat(0),
    type_: VkImageType(0),
    samples: VkSampleCountFlagBits(0),
    usage: VkImageUsageFlagBits(0),
    tiling: VkImageTiling(0),
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
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[inline]
  pub const fn with_type(mut self, val: VkImageType) -> Self {
    self.type_ = val;
    self
  }
  #[inline]
  pub const fn with_samples(mut self, val: VkSampleCountFlagBits) -> Self {
    self.samples = val;
    self
  }
  #[inline]
  pub const fn with_usage(mut self, val: VkImageUsageFlags) -> Self {
    self.usage = val;
    self
  }
  #[inline]
  pub const fn with_tiling(mut self, val: VkImageTiling) -> Self {
    self.tiling = val;
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
  #[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceSparseImageFormatInfo2<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceSparseImageFormatInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkExternalMemoryProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryProperties.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalMemoryProperties {
  pub externalMemoryFeatures: VkExternalMemoryFeatureFlags,
  /// Optional: true
  pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlags,
  pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlags,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl Send for VkExternalMemoryProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl Sync for VkExternalMemoryProperties {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl VkExternalMemoryProperties {
  pub const DEFAULT: Self = Self {
    externalMemoryFeatures: VkExternalMemoryFeatureFlagBits(0),
    exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagBits(0),
    compatibleHandleTypes: VkExternalMemoryHandleTypeFlagBits(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_externalMemoryFeatures(mut self, val: VkExternalMemoryFeatureFlags) -> Self {
    self.externalMemoryFeatures = val;
    self
  }
  #[inline]
  pub const fn with_exportFromImportedHandleTypes(
    mut self,
    val: VkExternalMemoryHandleTypeFlags,
  ) -> Self {
    self.exportFromImportedHandleTypes = val;
    self
  }
  #[inline]
  pub const fn with_compatibleHandleTypes(mut self, val: VkExternalMemoryHandleTypeFlags) -> Self {
    self.compatibleHandleTypes = val;
    self
  }
}
/// [VkPhysicalDeviceExternalImageFormatInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalImageFormatInfo.html)
///
/// **Extends:** VkPhysicalDeviceImageFormatInfo2.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalImageFormatInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalImageFormatInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalImageFormatInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceImageFormatInfo2<'root>>
  for VkPhysicalDeviceExternalImageFormatInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceExternalImageFormatInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO,
    pNext: core::ptr::null(),
    handleType: VkExternalMemoryHandleTypeFlagBits(0),
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
  pub const fn with_handleType(mut self, val: VkExternalMemoryHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceImageFormatInfo2<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceImageFormatInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkExternalImageFormatProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalImageFormatProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkImageFormatProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalImageFormatProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub externalMemoryProperties: VkExternalMemoryProperties,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkExternalImageFormatProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkExternalImageFormatProperties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageFormatProperties2<'root>>
  for VkExternalImageFormatProperties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkExternalImageFormatProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_IMAGE_FORMAT_PROPERTIES,
    pNext: core::ptr::null_mut(),
    externalMemoryProperties: VkExternalMemoryProperties::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_externalMemoryProperties(mut self, val: VkExternalMemoryProperties) -> Self {
    self.externalMemoryProperties = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageFormatProperties2<
    'root,
    T: VkPNextExtends<VkImageFormatProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceExternalBufferInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalBufferInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalBufferInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkBufferCreateFlags,
  /// Optional: true,  No Auto-Validity
  pub usage: VkBufferUsageFlags,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalBufferInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalBufferInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceExternalBufferInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO,
    pNext: core::ptr::null(),
    flags: VkBufferCreateFlagBits(0),
    usage: VkBufferUsageFlagBits(0),
    handleType: VkExternalMemoryHandleTypeFlagBits(0),
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
  pub const fn with_usage(mut self, val: VkBufferUsageFlags) -> Self {
    self.usage = val;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalMemoryHandleTypeFlagBits) -> Self {
    self.handleType = val;
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
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceExternalBufferInfo<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceExternalBufferInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkExternalBufferProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalBufferProperties.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalBufferProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub externalMemoryProperties: VkExternalMemoryProperties,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkExternalBufferProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkExternalBufferProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkExternalBufferProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_BUFFER_PROPERTIES,
    pNext: core::ptr::null_mut(),
    externalMemoryProperties: VkExternalMemoryProperties::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_externalMemoryProperties(mut self, val: VkExternalMemoryProperties) -> Self {
    self.externalMemoryProperties = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkExternalBufferProperties<
    'root,
    T: VkPNextExtends<VkExternalBufferProperties<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceIDProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceIDProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceIDProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Noauto]
  pub deviceUUID: [u8; VK_UUID_SIZE as usize],
  /// Limit Type: [Noauto]
  pub driverUUID: [u8; VK_UUID_SIZE as usize],
  /// Limit Type: [Noauto]
  pub deviceLUID: [u8; VK_LUID_SIZE as usize],
  /// Limit Type: [Noauto]
  pub deviceNodeMask: u32,
  /// Limit Type: [Max]
  pub deviceLUIDValid: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceIDProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceIDProperties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceIDProperties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceIDProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_ID_PROPERTIES,
    pNext: core::ptr::null_mut(),
    deviceUUID: [0u8; VK_UUID_SIZE as usize],
    driverUUID: [0u8; VK_UUID_SIZE as usize],
    deviceLUID: [0u8; VK_LUID_SIZE as usize],
    deviceNodeMask: 0,
    deviceLUIDValid: 0,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_deviceUUID(mut self, val: [u8; VK_UUID_SIZE as usize]) -> Self {
    self.deviceUUID = val;
    self
  }
  #[inline]
  pub const fn with_driverUUID(mut self, val: [u8; VK_UUID_SIZE as usize]) -> Self {
    self.driverUUID = val;
    self
  }
  #[inline]
  pub const fn with_deviceLUID(mut self, val: [u8; VK_LUID_SIZE as usize]) -> Self {
    self.deviceLUID = val;
    self
  }
  #[inline]
  pub const fn with_deviceNodeMask(mut self, val: u32) -> Self {
    self.deviceNodeMask = val;
    self
  }
  #[inline]
  pub const fn with_deviceLUIDValid(mut self, val: VkBool32) -> Self {
    self.deviceLUIDValid = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceProperties2<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkExternalMemoryImageCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryImageCreateInfo.html)
///
/// **Extends:** VkImageCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalMemoryImageCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub handleTypes: VkExternalMemoryHandleTypeFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkExternalMemoryImageCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkExternalMemoryImageCreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkExternalMemoryImageCreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkExternalMemoryImageCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO,
    pNext: core::ptr::null(),
    handleTypes: VkExternalMemoryHandleTypeFlagBits(0),
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
  pub const fn with_handleTypes(mut self, val: VkExternalMemoryHandleTypeFlags) -> Self {
    self.handleTypes = val;
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
/// [VkExternalMemoryBufferCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryBufferCreateInfo.html)
///
/// **Extends:** VkBufferCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalMemoryBufferCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub handleTypes: VkExternalMemoryHandleTypeFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkExternalMemoryBufferCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkExternalMemoryBufferCreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkBufferCreateInfo<'root>>
  for VkExternalMemoryBufferCreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkExternalMemoryBufferCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO,
    pNext: core::ptr::null(),
    handleTypes: VkExternalMemoryHandleTypeFlagBits(0),
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
  pub const fn with_handleTypes(mut self, val: VkExternalMemoryHandleTypeFlags) -> Self {
    self.handleTypes = val;
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
/// [VkExportMemoryAllocateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMemoryAllocateInfo.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExportMemoryAllocateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub handleTypes: VkExternalMemoryHandleTypeFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkExportMemoryAllocateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkExportMemoryAllocateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkExportMemoryAllocateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkExportMemoryAllocateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXPORT_MEMORY_ALLOCATE_INFO,
    pNext: core::ptr::null(),
    handleTypes: VkExternalMemoryHandleTypeFlagBits(0),
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
  pub const fn with_handleTypes(mut self, val: VkExternalMemoryHandleTypeFlags) -> Self {
    self.handleTypes = val;
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
/// [VkPhysicalDeviceExternalSemaphoreInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalSemaphoreInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalSemaphoreInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalSemaphoreInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceExternalSemaphoreInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO,
    pNext: core::ptr::null(),
    handleType: VkExternalSemaphoreHandleTypeFlagBits(0),
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
  pub const fn with_handleType(mut self, val: VkExternalSemaphoreHandleTypeFlagBits) -> Self {
    self.handleType = val;
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
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceExternalSemaphoreInfo<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceExternalSemaphoreInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkExternalSemaphoreProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreProperties.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalSemaphoreProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlags,
  pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlags,
  /// Optional: true
  pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkExternalSemaphoreProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkExternalSemaphoreProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkExternalSemaphoreProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_SEMAPHORE_PROPERTIES,
    pNext: core::ptr::null_mut(),
    exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlagBits(0),
    compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlagBits(0),
    externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlagBits(0),
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_exportFromImportedHandleTypes(
    mut self,
    val: VkExternalSemaphoreHandleTypeFlags,
  ) -> Self {
    self.exportFromImportedHandleTypes = val;
    self
  }
  #[inline]
  pub const fn with_compatibleHandleTypes(
    mut self,
    val: VkExternalSemaphoreHandleTypeFlags,
  ) -> Self {
    self.compatibleHandleTypes = val;
    self
  }
  #[inline]
  pub const fn with_externalSemaphoreFeatures(
    mut self,
    val: VkExternalSemaphoreFeatureFlags,
  ) -> Self {
    self.externalSemaphoreFeatures = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkExternalSemaphoreProperties<
    'root,
    T: VkPNextExtends<VkExternalSemaphoreProperties<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkExportSemaphoreCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkExportSemaphoreCreateInfo.html)
///
/// **Extends:** VkSemaphoreCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExportSemaphoreCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub handleTypes: VkExternalSemaphoreHandleTypeFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkExportSemaphoreCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkExportSemaphoreCreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSemaphoreCreateInfo<'root>>
  for VkExportSemaphoreCreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkExportSemaphoreCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXPORT_SEMAPHORE_CREATE_INFO,
    pNext: core::ptr::null(),
    handleTypes: VkExternalSemaphoreHandleTypeFlagBits(0),
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
  pub const fn with_handleTypes(mut self, val: VkExternalSemaphoreHandleTypeFlags) -> Self {
    self.handleTypes = val;
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
/// [VkPhysicalDeviceExternalFenceInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalFenceInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalFenceInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub handleType: VkExternalFenceHandleTypeFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalFenceInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalFenceInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceExternalFenceInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO,
    pNext: core::ptr::null(),
    handleType: VkExternalFenceHandleTypeFlagBits(0),
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
  pub const fn with_handleType(mut self, val: VkExternalFenceHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceExternalFenceInfo<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceExternalFenceInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkExternalFenceProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceProperties.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalFenceProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlags,
  pub compatibleHandleTypes: VkExternalFenceHandleTypeFlags,
  /// Optional: true
  pub externalFenceFeatures: VkExternalFenceFeatureFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkExternalFenceProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkExternalFenceProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkExternalFenceProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_FENCE_PROPERTIES,
    pNext: core::ptr::null_mut(),
    exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlagBits(0),
    compatibleHandleTypes: VkExternalFenceHandleTypeFlagBits(0),
    externalFenceFeatures: VkExternalFenceFeatureFlagBits(0),
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_exportFromImportedHandleTypes(
    mut self,
    val: VkExternalFenceHandleTypeFlags,
  ) -> Self {
    self.exportFromImportedHandleTypes = val;
    self
  }
  #[inline]
  pub const fn with_compatibleHandleTypes(mut self, val: VkExternalFenceHandleTypeFlags) -> Self {
    self.compatibleHandleTypes = val;
    self
  }
  #[inline]
  pub const fn with_externalFenceFeatures(mut self, val: VkExternalFenceFeatureFlags) -> Self {
    self.externalFenceFeatures = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkExternalFenceProperties<
    'root,
    T: VkPNextExtends<VkExternalFenceProperties<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkExportFenceCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkExportFenceCreateInfo.html)
///
/// **Extends:** VkFenceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExportFenceCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub handleTypes: VkExternalFenceHandleTypeFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkExportFenceCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkExportFenceCreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkFenceCreateInfo<'root>>
  for VkExportFenceCreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkExportFenceCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXPORT_FENCE_CREATE_INFO,
    pNext: core::ptr::null(),
    handleTypes: VkExternalFenceHandleTypeFlagBits(0),
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
  pub const fn with_handleTypes(mut self, val: VkExternalFenceHandleTypeFlags) -> Self {
    self.handleTypes = val;
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
/// [VkPhysicalDeviceGroupProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGroupProperties.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceGroupProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub physicalDeviceCount: u32,
  /// Length: physicalDeviceCount
  pub physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE as usize],
  pub subsetAllocation: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceGroupProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceGroupProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceGroupProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES,
    pNext: core::ptr::null_mut(),
    physicalDeviceCount: 0,
    physicalDevices: [VkPhysicalDevice::DEFAULT; VK_MAX_DEVICE_GROUP_SIZE as usize],
    subsetAllocation: 0,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_physicalDeviceCount(mut self, val: u32) -> Self {
    self.physicalDeviceCount = val;
    self
  }
  #[inline]
  pub const fn with_physicalDevices(
    mut self,
    val: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE as usize],
  ) -> Self {
    self.physicalDevices = val;
    self
  }
  #[inline]
  pub const fn with_subsetAllocation(mut self, val: VkBool32) -> Self {
    self.subsetAllocation = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceGroupProperties<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceGroupProperties<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkMemoryAllocateFlagsInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagsInfo.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryAllocateFlagsInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkMemoryAllocateFlags,
  pub deviceMask: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkMemoryAllocateFlagsInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkMemoryAllocateFlagsInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkMemoryAllocateFlagsInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkMemoryAllocateFlagsInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_ALLOCATE_FLAGS_INFO,
    pNext: core::ptr::null(),
    flags: VkMemoryAllocateFlagBits(0),
    deviceMask: 0,
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
  pub const fn with_flags(mut self, val: VkMemoryAllocateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_deviceMask(mut self, val: u32) -> Self {
    self.deviceMask = val;
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
/// [VkBindBufferMemoryInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindBufferMemoryInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindBufferMemoryInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub buffer: VkBuffer,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkBindBufferMemoryInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkBindBufferMemoryInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkBindBufferMemoryInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BIND_BUFFER_MEMORY_INFO,
    pNext: core::ptr::null(),
    buffer: VkBuffer::DEFAULT,
    memory: VkDeviceMemory::DEFAULT,
    memoryOffset: 0,
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
  pub const fn with_buffer(mut self, val: VkBuffer) -> Self {
    self.buffer = val;
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
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkBindBufferMemoryDeviceGroupInfo<'child>(
    mut self,
    val: &'a VkBindBufferMemoryDeviceGroupInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBindBufferMemoryDeviceGroupInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkBindMemoryStatus<'child>(
    mut self,
    val: &'a VkBindMemoryStatus<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBindMemoryStatus<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBindBufferMemoryInfo<
    'root,
    T: VkPNextExtends<VkBindBufferMemoryInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBindBufferMemoryDeviceGroupInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindBufferMemoryDeviceGroupInfo.html)
///
/// **Extends:** VkBindBufferMemoryInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindBufferMemoryDeviceGroupInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub deviceIndexCount: u32,
  /// Length: deviceIndexCount
  pub pDeviceIndices: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkBindBufferMemoryDeviceGroupInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkBindBufferMemoryDeviceGroupInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkBindBufferMemoryInfo<'root>>
  for VkBindBufferMemoryDeviceGroupInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkBindBufferMemoryDeviceGroupInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO,
    pNext: core::ptr::null(),
    deviceIndexCount: 0,
    pDeviceIndices: core::ptr::null(),
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
  pub const fn with_deviceIndexCount(mut self, val: u32) -> Self {
    self.deviceIndexCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDeviceIndices(mut self, val: &'a [u32]) -> Self {
    self.deviceIndexCount = val.len() as u32;
    self.pDeviceIndices = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBindBufferMemoryInfo<
    'root,
    T: VkPNextExtends<VkBindBufferMemoryInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBindImageMemoryInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindImageMemoryInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindImageMemoryInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub image: VkImage,
  /// No Auto-Validity
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkBindImageMemoryInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkBindImageMemoryInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkBindImageMemoryInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BIND_IMAGE_MEMORY_INFO,
    pNext: core::ptr::null(),
    image: VkImage::DEFAULT,
    memory: VkDeviceMemory::DEFAULT,
    memoryOffset: 0,
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
  pub const fn with_image(mut self, val: VkImage) -> Self {
    self.image = val;
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
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkBindImageMemoryDeviceGroupInfo<'child>(
    mut self,
    val: &'a VkBindImageMemoryDeviceGroupInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBindImageMemoryDeviceGroupInfo<'child>).cast::<c_void>();
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
  pub const fn with_pNext_VkBindImageMemorySwapchainInfoKHR<'child>(
    mut self,
    val: &'a VkBindImageMemorySwapchainInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBindImageMemorySwapchainInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkBindImagePlaneMemoryInfo<'child>(
    mut self,
    val: &'a VkBindImagePlaneMemoryInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBindImagePlaneMemoryInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkBindMemoryStatus<'child>(
    mut self,
    val: &'a VkBindMemoryStatus<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBindMemoryStatus<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBindImageMemoryInfo<
    'root,
    T: VkPNextExtends<VkBindImageMemoryInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBindImageMemoryDeviceGroupInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindImageMemoryDeviceGroupInfo.html)
///
/// **Extends:** VkBindImageMemoryInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindImageMemoryDeviceGroupInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub deviceIndexCount: u32,
  /// Length: deviceIndexCount
  pub pDeviceIndices: *const u32,
  /// Optional: true
  pub splitInstanceBindRegionCount: u32,
  /// Length: splitInstanceBindRegionCount
  pub pSplitInstanceBindRegions: *const VkRect2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkBindImageMemoryDeviceGroupInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkBindImageMemoryDeviceGroupInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkBindImageMemoryInfo<'root>>
  for VkBindImageMemoryDeviceGroupInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkBindImageMemoryDeviceGroupInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO,
    pNext: core::ptr::null(),
    deviceIndexCount: 0,
    pDeviceIndices: core::ptr::null(),
    splitInstanceBindRegionCount: 0,
    pSplitInstanceBindRegions: core::ptr::null(),
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
  pub const fn with_deviceIndexCount(mut self, val: u32) -> Self {
    self.deviceIndexCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDeviceIndices(mut self, val: &'a [u32]) -> Self {
    self.deviceIndexCount = val.len() as u32;
    self.pDeviceIndices = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_splitInstanceBindRegionCount(mut self, val: u32) -> Self {
    self.splitInstanceBindRegionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSplitInstanceBindRegions(mut self, val: &'a [VkRect2D]) -> Self {
    self.splitInstanceBindRegionCount = val.len() as u32;
    self.pSplitInstanceBindRegions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBindImageMemoryInfo<
    'root,
    T: VkPNextExtends<VkBindImageMemoryInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDeviceGroupCommandBufferBeginInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupCommandBufferBeginInfo.html)
///
/// **Extends:** VkCommandBufferBeginInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceGroupCommandBufferBeginInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub deviceMask: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkDeviceGroupCommandBufferBeginInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkDeviceGroupCommandBufferBeginInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferBeginInfo<'root>>
  for VkDeviceGroupCommandBufferBeginInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkDeviceGroupCommandBufferBeginInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
    pNext: core::ptr::null(),
    deviceMask: 0,
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
  pub const fn with_deviceMask(mut self, val: u32) -> Self {
    self.deviceMask = val;
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
/// [VkDeviceGroupSubmitInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupSubmitInfo.html)
///
/// **Extends:** VkSubmitInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceGroupSubmitInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub waitSemaphoreCount: u32,
  /// Length: waitSemaphoreCount
  pub pWaitSemaphoreDeviceIndices: *const u32,
  /// Optional: true
  pub commandBufferCount: u32,
  /// Length: commandBufferCount
  pub pCommandBufferDeviceMasks: *const u32,
  /// Optional: true
  pub signalSemaphoreCount: u32,
  /// Length: signalSemaphoreCount
  pub pSignalSemaphoreDeviceIndices: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkDeviceGroupSubmitInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkDeviceGroupSubmitInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubmitInfo<'root>> for VkDeviceGroupSubmitInfo<'child> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkDeviceGroupSubmitInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_GROUP_SUBMIT_INFO,
    pNext: core::ptr::null(),
    waitSemaphoreCount: 0,
    pWaitSemaphoreDeviceIndices: core::ptr::null(),
    commandBufferCount: 0,
    pCommandBufferDeviceMasks: core::ptr::null(),
    signalSemaphoreCount: 0,
    pSignalSemaphoreDeviceIndices: core::ptr::null(),
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
  pub const fn with_pWaitSemaphoreDeviceIndices(mut self, val: &'a [u32]) -> Self {
    self.waitSemaphoreCount = val.len() as u32;
    self.pWaitSemaphoreDeviceIndices = val.as_ptr();
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
  pub const fn with_pCommandBufferDeviceMasks(mut self, val: &'a [u32]) -> Self {
    self.commandBufferCount = val.len() as u32;
    self.pCommandBufferDeviceMasks = val.as_ptr();
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
  pub const fn with_pSignalSemaphoreDeviceIndices(mut self, val: &'a [u32]) -> Self {
    self.signalSemaphoreCount = val.len() as u32;
    self.pSignalSemaphoreDeviceIndices = val.as_ptr();
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
/// [VkDeviceGroupBindSparseInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupBindSparseInfo.html)
///
/// **Extends:** VkBindSparseInfo.
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceGroupBindSparseInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub resourceDeviceIndex: u32,
  pub memoryDeviceIndex: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkDeviceGroupBindSparseInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkDeviceGroupBindSparseInfo<'a> {}
#[cfg(all(
  all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")),
  all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0"))
))]
unsafe impl<'child, 'root> VkPNextExtends<VkBindSparseInfo<'root>>
  for VkDeviceGroupBindSparseInfo<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkDeviceGroupBindSparseInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_GROUP_BIND_SPARSE_INFO,
    pNext: core::ptr::null(),
    resourceDeviceIndex: 0,
    memoryDeviceIndex: 0,
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
  pub const fn with_resourceDeviceIndex(mut self, val: u32) -> Self {
    self.resourceDeviceIndex = val;
    self
  }
  #[inline]
  pub const fn with_memoryDeviceIndex(mut self, val: u32) -> Self {
    self.memoryDeviceIndex = val;
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
/// [VkDeviceGroupDeviceCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupDeviceCreateInfo.html)
///
/// **Extends:** VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceGroupDeviceCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub physicalDeviceCount: u32,
  /// Length: physicalDeviceCount
  pub pPhysicalDevices: *const VkPhysicalDevice,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkDeviceGroupDeviceCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkDeviceGroupDeviceCreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkDeviceGroupDeviceCreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkDeviceGroupDeviceCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_GROUP_DEVICE_CREATE_INFO,
    pNext: core::ptr::null(),
    physicalDeviceCount: 0,
    pPhysicalDevices: core::ptr::null(),
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
  pub const fn with_physicalDeviceCount(mut self, val: u32) -> Self {
    self.physicalDeviceCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPhysicalDevices(mut self, val: &'a [VkPhysicalDevice]) -> Self {
    self.physicalDeviceCount = val.len() as u32;
    self.pPhysicalDevices = val.as_ptr();
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
/// [VkBufferMemoryRequirementsInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferMemoryRequirementsInfo2.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferMemoryRequirementsInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub buffer: VkBuffer,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkBufferMemoryRequirementsInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkBufferMemoryRequirementsInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkBufferMemoryRequirementsInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2,
    pNext: core::ptr::null(),
    buffer: VkBuffer::DEFAULT,
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
  pub const fn with_buffer(mut self, val: VkBuffer) -> Self {
    self.buffer = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferMemoryRequirementsInfo2<
    'root,
    T: VkPNextExtends<VkBufferMemoryRequirementsInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageMemoryRequirementsInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageMemoryRequirementsInfo2.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageMemoryRequirementsInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub image: VkImage,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkImageMemoryRequirementsInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkImageMemoryRequirementsInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkImageMemoryRequirementsInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2,
    pNext: core::ptr::null(),
    image: VkImage::DEFAULT,
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
  pub const fn with_image(mut self, val: VkImage) -> Self {
    self.image = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkImagePlaneMemoryRequirementsInfo<'child>(
    mut self,
    val: &'a VkImagePlaneMemoryRequirementsInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkImagePlaneMemoryRequirementsInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageMemoryRequirementsInfo2<
    'root,
    T: VkPNextExtends<VkImageMemoryRequirementsInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageSparseMemoryRequirementsInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSparseMemoryRequirementsInfo2.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageSparseMemoryRequirementsInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub image: VkImage,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkImageSparseMemoryRequirementsInfo2<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkImageSparseMemoryRequirementsInfo2<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkImageSparseMemoryRequirementsInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2,
    pNext: core::ptr::null(),
    image: VkImage::DEFAULT,
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
  pub const fn with_image(mut self, val: VkImage) -> Self {
    self.image = val;
    self
  }
  #[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageSparseMemoryRequirementsInfo2<
    'root,
    T: VkPNextExtends<VkImageSparseMemoryRequirementsInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkMemoryRequirements2](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryRequirements2.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryRequirements2<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub memoryRequirements: VkMemoryRequirements,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkMemoryRequirements2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkMemoryRequirements2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkMemoryRequirements2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_REQUIREMENTS_2,
    pNext: core::ptr::null_mut(),
    memoryRequirements: VkMemoryRequirements::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_memoryRequirements(mut self, val: VkMemoryRequirements) -> Self {
    self.memoryRequirements = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryDedicatedRequirements<'child>(
    mut self,
    val: &'a mut VkMemoryDedicatedRequirements<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkMemoryDedicatedRequirements<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_memory_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTileMemoryRequirementsQCOM<'child>(
    mut self,
    val: &'a mut VkTileMemoryRequirementsQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkTileMemoryRequirementsQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryRequirements2<
    'root,
    T: VkPNextExtends<VkMemoryRequirements2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkSparseImageMemoryRequirements2](https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageMemoryRequirements2.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSparseImageMemoryRequirements2<'a> {
  /// Values: VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub memoryRequirements: VkSparseImageMemoryRequirements,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkSparseImageMemoryRequirements2<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkSparseImageMemoryRequirements2<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkSparseImageMemoryRequirements2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2,
    pNext: core::ptr::null_mut(),
    memoryRequirements: VkSparseImageMemoryRequirements::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_memoryRequirements(mut self, val: VkSparseImageMemoryRequirements) -> Self {
    self.memoryRequirements = val;
    self
  }
  #[cfg(all(feature = "VK_BASE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSparseImageMemoryRequirements2<
    'root,
    T: VkPNextExtends<VkSparseImageMemoryRequirements2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkMemoryDedicatedRequirements](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDedicatedRequirements.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkMemoryRequirements2.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryDedicatedRequirements<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub prefersDedicatedAllocation: VkBool32,
  pub requiresDedicatedAllocation: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkMemoryDedicatedRequirements<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkMemoryDedicatedRequirements<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryRequirements2<'root>>
  for VkMemoryDedicatedRequirements<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkMemoryDedicatedRequirements<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_DEDICATED_REQUIREMENTS,
    pNext: core::ptr::null_mut(),
    prefersDedicatedAllocation: 0,
    requiresDedicatedAllocation: 0,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_prefersDedicatedAllocation(mut self, val: VkBool32) -> Self {
    self.prefersDedicatedAllocation = val;
    self
  }
  #[inline]
  pub const fn with_requiresDedicatedAllocation(mut self, val: VkBool32) -> Self {
    self.requiresDedicatedAllocation = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryRequirements2<
    'root,
    T: VkPNextExtends<VkMemoryRequirements2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkMemoryDedicatedAllocateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDedicatedAllocateInfo.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryDedicatedAllocateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub image: VkImage,
  /// Optional: true
  pub buffer: VkBuffer,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkMemoryDedicatedAllocateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkMemoryDedicatedAllocateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkMemoryDedicatedAllocateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkMemoryDedicatedAllocateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_DEDICATED_ALLOCATE_INFO,
    pNext: core::ptr::null(),
    image: VkImage::DEFAULT,
    buffer: VkBuffer::DEFAULT,
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
  pub const fn with_image(mut self, val: VkImage) -> Self {
    self.image = val;
    self
  }
  #[inline]
  pub const fn with_buffer(mut self, val: VkBuffer) -> Self {
    self.buffer = val;
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
/// [VkImageViewUsageCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewUsageCreateInfo.html)
///
/// **Extends:** VkImageViewCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageViewUsageCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub usage: VkImageUsageFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkImageViewUsageCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkImageViewUsageCreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageViewCreateInfo<'root>>
  for VkImageViewUsageCreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkImageViewUsageCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_VIEW_USAGE_CREATE_INFO,
    pNext: core::ptr::null(),
    usage: VkImageUsageFlagBits(0),
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
  pub const fn with_usage(mut self, val: VkImageUsageFlags) -> Self {
    self.usage = val;
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
/// [VkBindImagePlaneMemoryInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindImagePlaneMemoryInfo.html)
///
/// **Extends:** VkBindImageMemoryInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindImagePlaneMemoryInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub planeAspect: VkImageAspectFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkBindImagePlaneMemoryInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkBindImagePlaneMemoryInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkBindImageMemoryInfo<'root>>
  for VkBindImagePlaneMemoryInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkBindImagePlaneMemoryInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BIND_IMAGE_PLANE_MEMORY_INFO,
    pNext: core::ptr::null(),
    planeAspect: VkImageAspectFlagBits(0),
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
  pub const fn with_planeAspect(mut self, val: VkImageAspectFlagBits) -> Self {
    self.planeAspect = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBindImageMemoryInfo<
    'root,
    T: VkPNextExtends<VkBindImageMemoryInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImagePlaneMemoryRequirementsInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkImagePlaneMemoryRequirementsInfo.html)
///
/// **Extends:** VkImageMemoryRequirementsInfo2.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImagePlaneMemoryRequirementsInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub planeAspect: VkImageAspectFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkImagePlaneMemoryRequirementsInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkImagePlaneMemoryRequirementsInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageMemoryRequirementsInfo2<'root>>
  for VkImagePlaneMemoryRequirementsInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkImagePlaneMemoryRequirementsInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO,
    pNext: core::ptr::null(),
    planeAspect: VkImageAspectFlagBits(0),
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
  pub const fn with_planeAspect(mut self, val: VkImageAspectFlagBits) -> Self {
    self.planeAspect = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageMemoryRequirementsInfo2<
    'root,
    T: VkPNextExtends<VkImageMemoryRequirementsInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkProtectedSubmitInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkProtectedSubmitInfo.html)
///
/// **Extends:** VkSubmitInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkProtectedSubmitInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub protectedSubmit: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkProtectedSubmitInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkProtectedSubmitInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubmitInfo<'root>> for VkProtectedSubmitInfo<'child> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkProtectedSubmitInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PROTECTED_SUBMIT_INFO,
    pNext: core::ptr::null(),
    protectedSubmit: 0,
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
  pub const fn with_protectedSubmit(mut self, val: VkBool32) -> Self {
    self.protectedSubmit = val;
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
/// [VkPhysicalDeviceProtectedMemoryFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceProtectedMemoryFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub protectedMemory: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceProtectedMemoryFeatures<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceProtectedMemoryFeatures<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceProtectedMemoryFeatures<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceProtectedMemoryFeatures<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceProtectedMemoryFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES,
    pNext: core::ptr::null_mut(),
    protectedMemory: 0,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_protectedMemory(mut self, val: VkBool32) -> Self {
    self.protectedMemory = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceFeatures2<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
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
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceProtectedMemoryProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceProtectedMemoryProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceProtectedMemoryProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Exact]
  pub protectedNoFault: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceProtectedMemoryProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceProtectedMemoryProperties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceProtectedMemoryProperties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkPhysicalDeviceProtectedMemoryProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES,
    pNext: core::ptr::null_mut(),
    protectedNoFault: 0,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_protectedNoFault(mut self, val: VkBool32) -> Self {
    self.protectedNoFault = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceProperties2<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceProperties2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDeviceQueueInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueInfo2.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceQueueInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkDeviceQueueCreateFlags,
  pub queueFamilyIndex: u32,
  pub queueIndex: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Send for VkDeviceQueueInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
unsafe impl<'a> Sync for VkDeviceQueueInfo2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_1")]
impl<'a> VkDeviceQueueInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_QUEUE_INFO_2,
    pNext: core::ptr::null(),
    flags: VkDeviceQueueCreateFlagBits(0),
    queueFamilyIndex: 0,
    queueIndex: 0,
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
  pub const fn with_queueIndex(mut self, val: u32) -> Self {
    self.queueIndex = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDeviceQueueInfo2<
    'root,
    T: VkPNextExtends<VkDeviceQueueInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
