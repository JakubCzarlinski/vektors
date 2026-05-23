#[cfg(feature = "VK_NV_ray_tracing")]
use crate::enums::VkAccelerationStructureMemoryRequirementsTypeNV;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::enums::VkAccelerationStructureMotionInstanceTypeNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::enums::VkAccelerationStructureTypeNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkBufferUsageFlagBits;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing"
))]
use crate::enums::VkBuildAccelerationStructureFlagBitsKHR;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureAddressResolutionFlagBitsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureClusterFlagBitsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureGeometryFlagBitsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureIndexFormatFlagBitsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureOpModeNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureOpTypeNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureTypeNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::enums::VkCoarseSampleOrderTypeNV;
#[cfg(any(
  feature = "VK_NV_cooperative_vector",
  feature = "VK_KHR_cooperative_matrix",
  feature = "VK_NV_cooperative_matrix"
))]
use crate::enums::VkComponentTypeKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::enums::VkComponentTypeNV;
#[cfg(feature = "VK_NV_cooperative_vector")]
use crate::enums::VkCooperativeVectorMatrixLayoutNV;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
use crate::enums::VkCoverageModulationModeNV;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::enums::VkCoverageReductionModeNV;
#[cfg(feature = "VK_NV_device_diagnostics_config")]
use crate::enums::VkDeviceDiagnosticsConfigFlagBitsNV;
#[cfg(feature = "VK_NV_display_stereo")]
use crate::enums::VkDisplaySurfaceStereoTypeNV;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_fence_capabilities"
))]
use crate::enums::VkExternalFenceHandleTypeFlagBits;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::enums::VkExternalMemoryFeatureFlagBitsNV;
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
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::enums::VkExternalMemoryHandleTypeFlagBitsNV;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_semaphore_capabilities"
))]
use crate::enums::VkExternalSemaphoreHandleTypeFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::enums::VkFragmentShadingRateCombinerOpKHR;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::enums::VkFragmentShadingRateNV;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::enums::VkFragmentShadingRateTypeNV;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing"
))]
use crate::enums::VkGeometryFlagBitsKHR;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing"
))]
use crate::enums::VkGeometryInstanceFlagBitsKHR;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing"
))]
use crate::enums::VkGeometryTypeKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageUsageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkIndexType;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::enums::VkIndirectCommandsLayoutUsageFlagBitsNV;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::enums::VkIndirectCommandsTokenTypeNV;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::enums::VkIndirectStateFlagBitsNV;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::enums::VkLatencyMarkerNV;
#[cfg(feature = "VK_EXT_memory_decompression")]
use crate::enums::VkMemoryDecompressionMethodFlagBitsEXT;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::enums::VkOpticalFlowExecuteFlagBitsNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::enums::VkOpticalFlowGridSizeFlagBitsNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::enums::VkOpticalFlowPerformanceLevelNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::enums::VkOpticalFlowSessionCreateFlagBitsNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::enums::VkOpticalFlowUsageFlagBitsNV;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::enums::VkOutOfBandQueueTypeNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::enums::VkPartitionedAccelerationStructureInstanceFlagBitsNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::enums::VkPartitionedAccelerationStructureOpTypeNV;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_AMDX_shader_enqueue",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_ray_tracing",
  feature = "VK_HUAWEI_subpass_shading"
))]
use crate::enums::VkPipelineBindPoint;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_KHR_device_group",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_ray_tracing",
  all(
    feature = "VK_EXT_fragment_density_map",
    feature = "VK_KHR_dynamic_rendering"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_KHR_fragment_shading_rate"
  ),
  feature = "VK_KHR_pipeline_executable_properties",
  feature = "VK_NV_device_generated_commands",
  feature = "VK_KHR_pipeline_library",
  feature = "VK_EXT_pipeline_creation_cache_control",
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_EXT_attachment_feedback_loop_layout",
  feature = "VK_EXT_opacity_micromap",
  feature = "VK_EXT_pipeline_protected_access",
  feature = "VK_KHR_opacity_micromap"
))]
use crate::enums::VkPipelineCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkPipelineStageFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_3",
  feature = "VK_KHR_video_decode_queue",
  feature = "VK_KHR_video_encode_queue",
  feature = "VK_KHR_synchronization2",
  feature = "VK_HUAWEI_subpass_shading",
  feature = "VK_HUAWEI_invocation_mask",
  feature = "VK_EXT_opacity_micromap",
  feature = "VK_HUAWEI_cluster_culling_shader",
  feature = "VK_NV_optical_flow",
  feature = "VK_NV_cooperative_vector",
  feature = "VK_KHR_copy_memory_indirect",
  feature = "VK_EXT_memory_decompression"
))]
use crate::enums::VkPipelineStageFlagBits2;
#[cfg(feature = "VK_KHR_surface")]
use crate::enums::VkPresentModeKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueueFlagBits;
#[cfg(any(
  feature = "VK_EXT_ray_tracing_invocation_reorder",
  feature = "VK_NV_ray_tracing_invocation_reorder"
))]
use crate::enums::VkRayTracingInvocationReorderModeEXT;
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
use crate::enums::VkRayTracingLssIndexingModeNV;
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
use crate::enums::VkRayTracingLssPrimitiveEndCapsModeNV;
#[cfg(any(feature = "VK_KHR_ray_tracing_pipeline", feature = "VK_NV_ray_tracing"))]
use crate::enums::VkRayTracingShaderGroupTypeKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSampleCountFlagBits;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::enums::VkSciSyncClientTypeNV;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::enums::VkSciSyncPrimitiveTypeNV;
#[cfg(any(
  feature = "VK_KHR_cooperative_matrix",
  feature = "VK_NV_cooperative_matrix"
))]
use crate::enums::VkScopeKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::enums::VkScopeNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkShaderStageFlagBits;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::enums::VkShadingRatePaletteEntryNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_NV_viewport_swizzle")]
use crate::enums::VkViewportCoordinateSwizzleNV;
#[cfg(any(
  feature = "VK_NV_external_memory_win32",
  feature = "VK_KHR_external_memory_win32",
  feature = "VK_KHR_external_semaphore_win32",
  feature = "VK_KHR_external_fence_win32"
))]
use crate::types::DWORD;
#[cfg(any(
  feature = "VK_NV_external_memory_win32",
  feature = "VK_KHR_external_memory_win32",
  feature = "VK_KHR_external_semaphore_win32",
  feature = "VK_KHR_external_fence_win32"
))]
use crate::types::HANDLE;
#[cfg(any(
  feature = "VK_NV_external_memory_win32",
  feature = "VK_KHR_external_memory_win32",
  feature = "VK_KHR_external_semaphore_win32",
  feature = "VK_KHR_external_fence_win32"
))]
use crate::types::SECURITY_ATTRIBUTES;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureCreateInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureGeometryKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureGeometryTrianglesDataKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureInstanceKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBuffer;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferUsageFlags;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkBuildAccelerationStructureFlagsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferInheritanceInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkComputePipelineCreateInfo;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkDescriptorSetAndBindingMappingEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceAddress;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceMemory;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_cooperative_vector"
))]
use crate::types::VkDeviceOrHostAddressConstKHR;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_cooperative_vector"
))]
use crate::types::VkDeviceOrHostAddressKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::types::VkDisplayModeProperties2KHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplaySurfaceCreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtent2D;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtent3D;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFence;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFenceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(any(feature = "VK_BASE_VERSION_1_3", feature = "VK_KHR_synchronization2"))]
use crate::types::VkFlags64;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkGeometryFlagsKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkGeometryInstanceFlagsKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImage;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageSubresourceLayers;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageUsageFlags;
#[cfg(all(
  feature = "VK_EXT_descriptor_heap",
  feature = "VK_NV_device_generated_commands"
))]
use crate::types::VkIndirectCommandsLayoutPushDataTokenNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsLayoutTokenEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryAllocateInfo;
#[cfg(feature = "VK_EXT_memory_decompression")]
use crate::types::VkMemoryDecompressionMethodFlagsEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkMicromapEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkMicromapUsageEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkOffset3D;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkOpaqueCaptureDescriptorDataCreateInfoEXT;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceImageFormatInfo2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipeline;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPipelineCreateFlags2CreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPipelineCreationFeedbackCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineLayout;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineMultisampleStateCreateInfo;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPipelineOfflineCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineShaderStageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPipelineStageFlags;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPipelineStageFlags2;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineTessellationStateCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineVertexInputStateCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineViewportStateCreateInfo;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::types::VkPresentInfoKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPushConstantsInfo;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkPushDataInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueue;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkQueueFamilyProperties2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueueFlags;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkRayTracingPipelineCreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkRect2D;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSampleCountFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSemaphore;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSemaphoreCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkShaderStageFlags;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkStridedDeviceAddressRegionKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSubmitInfo;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkSubmitInfo2;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::types::VkSurfaceCapabilities2KHR;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::types::VkSwapchainCreateInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkTransformMatrixKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkViewport;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkWriteDescriptorSet;
use core::ffi::{c_char, c_void};
/// [VkViewportWScalingNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkViewportWScalingNV.html)
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkViewportWScalingNV {
  pub xcoeff: f32,
  pub ycoeff: f32,
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
unsafe impl Send for VkViewportWScalingNV {}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
unsafe impl Sync for VkViewportWScalingNV {}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
impl VkViewportWScalingNV {
  pub const DEFAULT: Self = Self {
    xcoeff: 0.0f32,
    ycoeff: 0.0f32,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_xcoeff(mut self, val: f32) -> Self {
    self.xcoeff = val;
    self
  }
  #[inline]
  pub const fn with_ycoeff(mut self, val: f32) -> Self {
    self.ycoeff = val;
    self
  }
}
/// [VkPipelineViewportWScalingStateCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportWScalingStateCreateInfoNV.html)
///
/// **Extends:** VkPipelineViewportStateCreateInfo.
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineViewportWScalingStateCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub viewportWScalingEnable: VkBool32,
  pub viewportCount: u32,
  /// Optional: true,  Length: viewportCount,  No Auto-Validity
  pub pViewportWScalings: *const VkViewportWScalingNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
unsafe impl<'a> Send for VkPipelineViewportWScalingStateCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
unsafe impl<'a> Sync for VkPipelineViewportWScalingStateCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_clip_space_w_scaling",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineViewportStateCreateInfo<'root>>
  for VkPipelineViewportWScalingStateCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
impl<'a> VkPipelineViewportWScalingStateCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    viewportWScalingEnable: 0,
    viewportCount: 0,
    pViewportWScalings: core::ptr::null(),
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
  pub const fn with_viewportWScalingEnable(mut self, val: VkBool32) -> Self {
    self.viewportWScalingEnable = val;
    self
  }
  #[inline]
  pub const fn with_viewportCount(mut self, val: u32) -> Self {
    self.viewportCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pViewportWScalings(mut self, val: &'a [VkViewportWScalingNV]) -> Self {
    self.viewportCount = val.len() as u32;
    self.pViewportWScalings = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineViewportStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineViewportStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkClusterAccelerationStructureGeometryFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGeometryFlagsNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
pub type VkClusterAccelerationStructureGeometryFlagsNV =
  VkClusterAccelerationStructureGeometryFlagBitsNV;
/// [VkClusterAccelerationStructureClusterFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureClusterFlagsNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
pub type VkClusterAccelerationStructureClusterFlagsNV =
  VkClusterAccelerationStructureClusterFlagBitsNV;
/// [VkClusterAccelerationStructureAddressResolutionFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureAddressResolutionFlagsNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
pub type VkClusterAccelerationStructureAddressResolutionFlagsNV =
  VkClusterAccelerationStructureAddressResolutionFlagBitsNV;
/// [VkClusterAccelerationStructureIndexFormatFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureIndexFormatFlagsNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
pub type VkClusterAccelerationStructureIndexFormatFlagsNV =
  VkClusterAccelerationStructureIndexFormatFlagBitsNV;
/// [VkPhysicalDeviceClusterAccelerationStructureFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceClusterAccelerationStructureFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub clusterAccelerationStructure: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Send for VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Sync for VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_cluster_acceleration_structure",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_cluster_acceleration_structure",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl<'a> VkPhysicalDeviceClusterAccelerationStructureFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    clusterAccelerationStructure: 0,
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
  pub const fn with_clusterAccelerationStructure(mut self, val: VkBool32) -> Self {
    self.clusterAccelerationStructure = val;
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
/// [VkPhysicalDeviceClusterAccelerationStructurePropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceClusterAccelerationStructurePropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceClusterAccelerationStructurePropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxVerticesPerCluster: u32,
  /// Limit Type: [Max]
  pub maxTrianglesPerCluster: u32,
  /// Limit Type: [Min]
  pub clusterScratchByteAlignment: u32,
  /// Limit Type: [Min]
  pub clusterByteAlignment: u32,
  /// Limit Type: [Min]
  pub clusterTemplateByteAlignment: u32,
  /// Limit Type: [Min]
  pub clusterBottomLevelByteAlignment: u32,
  /// Limit Type: [Min]
  pub clusterTemplateBoundsByteAlignment: u32,
  /// Limit Type: [Max]
  pub maxClusterGeometryIndex: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Send for VkPhysicalDeviceClusterAccelerationStructurePropertiesNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Sync for VkPhysicalDeviceClusterAccelerationStructurePropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_cluster_acceleration_structure",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceClusterAccelerationStructurePropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl<'a> VkPhysicalDeviceClusterAccelerationStructurePropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    maxVerticesPerCluster: 0,
    maxTrianglesPerCluster: 0,
    clusterScratchByteAlignment: 0,
    clusterByteAlignment: 0,
    clusterTemplateByteAlignment: 0,
    clusterBottomLevelByteAlignment: 0,
    clusterTemplateBoundsByteAlignment: 0,
    maxClusterGeometryIndex: 0,
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
  pub const fn with_maxVerticesPerCluster(mut self, val: u32) -> Self {
    self.maxVerticesPerCluster = val;
    self
  }
  #[inline]
  pub const fn with_maxTrianglesPerCluster(mut self, val: u32) -> Self {
    self.maxTrianglesPerCluster = val;
    self
  }
  #[inline]
  pub const fn with_clusterScratchByteAlignment(mut self, val: u32) -> Self {
    self.clusterScratchByteAlignment = val;
    self
  }
  #[inline]
  pub const fn with_clusterByteAlignment(mut self, val: u32) -> Self {
    self.clusterByteAlignment = val;
    self
  }
  #[inline]
  pub const fn with_clusterTemplateByteAlignment(mut self, val: u32) -> Self {
    self.clusterTemplateByteAlignment = val;
    self
  }
  #[inline]
  pub const fn with_clusterBottomLevelByteAlignment(mut self, val: u32) -> Self {
    self.clusterBottomLevelByteAlignment = val;
    self
  }
  #[inline]
  pub const fn with_clusterTemplateBoundsByteAlignment(mut self, val: u32) -> Self {
    self.clusterTemplateBoundsByteAlignment = val;
    self
  }
  #[inline]
  pub const fn with_maxClusterGeometryIndex(mut self, val: u32) -> Self {
    self.maxClusterGeometryIndex = val;
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
/// [VkStridedDeviceAddressNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkStridedDeviceAddressNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkStridedDeviceAddressNV {
  pub startAddress: VkDeviceAddress,
  pub strideInBytes: VkDeviceSize,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Send for VkStridedDeviceAddressNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Sync for VkStridedDeviceAddressNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl VkStridedDeviceAddressNV {
  pub const DEFAULT: Self = Self {
    startAddress: 0,
    strideInBytes: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_startAddress(mut self, val: VkDeviceAddress) -> Self {
    self.startAddress = val;
    self
  }
  #[inline]
  pub const fn with_strideInBytes(mut self, val: VkDeviceSize) -> Self {
    self.strideInBytes = val;
    self
  }
}
/// [VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV.html)
///
/// **Extends:** VkRayTracingPipelineCreateInfoKHR.
///
/// **Availability:** depends on `VK_KHR_ray_tracing_pipeline`.
#[cfg(all(
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_cluster_acceleration_structure"
))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub allowClusterAccelerationStructure: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_cluster_acceleration_structure"
))]
unsafe impl<'a> Send for VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_cluster_acceleration_structure"
))]
unsafe impl<'a> Sync for VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV<'a> {}
#[cfg(all(
  all(
    feature = "VK_KHR_ray_tracing_pipeline",
    feature = "VK_NV_cluster_acceleration_structure"
  ),
  feature = "VK_KHR_ray_tracing_pipeline"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRayTracingPipelineCreateInfoKHR<'root>>
  for VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV<'child>
{
}
#[cfg(all(
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_cluster_acceleration_structure"
))]
impl<'a> VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV,
    pNext: core::ptr::null_mut(),
    allowClusterAccelerationStructure: 0,
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
  pub const fn with_allowClusterAccelerationStructure(mut self, val: VkBool32) -> Self {
    self.allowClusterAccelerationStructure = val;
    self
  }
  #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRayTracingPipelineCreateInfoKHR<
    'root,
    T: VkPNextExtends<VkRayTracingPipelineCreateInfoKHR<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV {
  pub geometryIndex: u32,
  pub reserved: u32,
  pub geometryFlags: u32,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Send for VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Sync for VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV {
  pub const DEFAULT: Self = Self {
    geometryIndex: 0,
    reserved: 0,
    geometryFlags: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_geometryIndex(mut self, val: u32) -> Self {
    self.geometryIndex = val;
    self
  }
  #[inline]
  pub const fn with_reserved(mut self, val: u32) -> Self {
    self.reserved = val;
    self
  }
  #[inline]
  pub const fn with_geometryFlags(mut self, val: u32) -> Self {
    self.geometryFlags = val;
    self
  }
}
/// [VkClusterAccelerationStructureMoveObjectsInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureMoveObjectsInfoNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureMoveObjectsInfoNV {
  pub srcAccelerationStructure: VkDeviceAddress,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Send for VkClusterAccelerationStructureMoveObjectsInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Sync for VkClusterAccelerationStructureMoveObjectsInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl VkClusterAccelerationStructureMoveObjectsInfoNV {
  pub const DEFAULT: Self = Self {
    srcAccelerationStructure: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_srcAccelerationStructure(mut self, val: VkDeviceAddress) -> Self {
    self.srcAccelerationStructure = val;
    self
  }
}
/// [VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV {
  pub clusterReferencesCount: u32,
  pub clusterReferencesStride: u32,
  pub clusterReferences: VkDeviceAddress,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Send for VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Sync for VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV {
  pub const DEFAULT: Self = Self {
    clusterReferencesCount: 0,
    clusterReferencesStride: 0,
    clusterReferences: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_clusterReferencesCount(mut self, val: u32) -> Self {
    self.clusterReferencesCount = val;
    self
  }
  #[inline]
  pub const fn with_clusterReferencesStride(mut self, val: u32) -> Self {
    self.clusterReferencesStride = val;
    self
  }
  #[inline]
  pub const fn with_clusterReferences(mut self, val: VkDeviceAddress) -> Self {
    self.clusterReferences = val;
    self
  }
}
/// [VkClusterAccelerationStructureGetTemplateIndicesInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGetTemplateIndicesInfoNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureGetTemplateIndicesInfoNV {
  pub clusterTemplateAddress: VkDeviceAddress,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Send for VkClusterAccelerationStructureGetTemplateIndicesInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Sync for VkClusterAccelerationStructureGetTemplateIndicesInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl VkClusterAccelerationStructureGetTemplateIndicesInfoNV {
  pub const DEFAULT: Self = Self {
    clusterTemplateAddress: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_clusterTemplateAddress(mut self, val: VkDeviceAddress) -> Self {
    self.clusterTemplateAddress = val;
    self
  }
}
/// [VkClusterAccelerationStructureBuildTriangleClusterInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureBuildTriangleClusterInfoNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureBuildTriangleClusterInfoNV {
  pub clusterID: u32,
  /// Optional: true
  pub clusterFlags: VkClusterAccelerationStructureClusterFlagsNV,
  pub triangleCount: u32,
  pub vertexCount: u32,
  pub positionTruncateBitCount: u32,
  pub indexType: u32,
  pub opacityMicromapIndexType: u32,
  pub baseGeometryIndexAndGeometryFlags:
    VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
  pub indexBufferStride: u16,
  pub vertexBufferStride: u16,
  pub geometryIndexAndFlagsBufferStride: u16,
  pub opacityMicromapIndexBufferStride: u16,
  pub indexBuffer: VkDeviceAddress,
  pub vertexBuffer: VkDeviceAddress,
  /// Optional: true
  pub geometryIndexAndFlagsBuffer: VkDeviceAddress,
  /// Optional: true
  pub opacityMicromapArray: VkDeviceAddress,
  /// Optional: true
  pub opacityMicromapIndexBuffer: VkDeviceAddress,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Send for VkClusterAccelerationStructureBuildTriangleClusterInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Sync for VkClusterAccelerationStructureBuildTriangleClusterInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl VkClusterAccelerationStructureBuildTriangleClusterInfoNV {
  pub const DEFAULT: Self = Self {
    clusterID: 0,
    clusterFlags: VkClusterAccelerationStructureClusterFlagBitsNV(0),
    triangleCount: 0,
    vertexCount: 0,
    positionTruncateBitCount: 0,
    indexType: 0,
    opacityMicromapIndexType: 0,
    baseGeometryIndexAndGeometryFlags:
      VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV::DEFAULT,
    indexBufferStride: 0,
    vertexBufferStride: 0,
    geometryIndexAndFlagsBufferStride: 0,
    opacityMicromapIndexBufferStride: 0,
    indexBuffer: 0,
    vertexBuffer: 0,
    geometryIndexAndFlagsBuffer: 0,
    opacityMicromapArray: 0,
    opacityMicromapIndexBuffer: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_clusterID(mut self, val: u32) -> Self {
    self.clusterID = val;
    self
  }
  #[inline]
  pub const fn with_clusterFlags(
    mut self,
    val: VkClusterAccelerationStructureClusterFlagsNV,
  ) -> Self {
    self.clusterFlags = val;
    self
  }
  #[inline]
  pub const fn with_triangleCount(mut self, val: u32) -> Self {
    self.triangleCount = val;
    self
  }
  #[inline]
  pub const fn with_vertexCount(mut self, val: u32) -> Self {
    self.vertexCount = val;
    self
  }
  #[inline]
  pub const fn with_positionTruncateBitCount(mut self, val: u32) -> Self {
    self.positionTruncateBitCount = val;
    self
  }
  #[inline]
  pub const fn with_indexType(mut self, val: u32) -> Self {
    self.indexType = val;
    self
  }
  #[inline]
  pub const fn with_opacityMicromapIndexType(mut self, val: u32) -> Self {
    self.opacityMicromapIndexType = val;
    self
  }
  #[inline]
  pub const fn with_baseGeometryIndexAndGeometryFlags(
    mut self,
    val: VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
  ) -> Self {
    self.baseGeometryIndexAndGeometryFlags = val;
    self
  }
  #[inline]
  pub const fn with_indexBufferStride(mut self, val: u16) -> Self {
    self.indexBufferStride = val;
    self
  }
  #[inline]
  pub const fn with_vertexBufferStride(mut self, val: u16) -> Self {
    self.vertexBufferStride = val;
    self
  }
  #[inline]
  pub const fn with_geometryIndexAndFlagsBufferStride(mut self, val: u16) -> Self {
    self.geometryIndexAndFlagsBufferStride = val;
    self
  }
  #[inline]
  pub const fn with_opacityMicromapIndexBufferStride(mut self, val: u16) -> Self {
    self.opacityMicromapIndexBufferStride = val;
    self
  }
  #[inline]
  pub const fn with_indexBuffer(mut self, val: VkDeviceAddress) -> Self {
    self.indexBuffer = val;
    self
  }
  #[inline]
  pub const fn with_vertexBuffer(mut self, val: VkDeviceAddress) -> Self {
    self.vertexBuffer = val;
    self
  }
  #[inline]
  pub const fn with_geometryIndexAndFlagsBuffer(mut self, val: VkDeviceAddress) -> Self {
    self.geometryIndexAndFlagsBuffer = val;
    self
  }
  #[inline]
  pub const fn with_opacityMicromapArray(mut self, val: VkDeviceAddress) -> Self {
    self.opacityMicromapArray = val;
    self
  }
  #[inline]
  pub const fn with_opacityMicromapIndexBuffer(mut self, val: VkDeviceAddress) -> Self {
    self.opacityMicromapIndexBuffer = val;
    self
  }
}
/// [VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV {
  pub clusterID: u32,
  /// Optional: true
  pub clusterFlags: VkClusterAccelerationStructureClusterFlagsNV,
  pub triangleCount: u32,
  pub vertexCount: u32,
  pub positionTruncateBitCount: u32,
  pub indexType: u32,
  pub opacityMicromapIndexType: u32,
  pub baseGeometryIndexAndGeometryFlags:
    VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
  pub indexBufferStride: u16,
  pub vertexBufferStride: u16,
  pub geometryIndexAndFlagsBufferStride: u16,
  pub opacityMicromapIndexBufferStride: u16,
  pub indexBuffer: VkDeviceAddress,
  /// Optional: true
  pub vertexBuffer: VkDeviceAddress,
  /// Optional: true
  pub geometryIndexAndFlagsBuffer: VkDeviceAddress,
  /// Optional: true
  pub opacityMicromapArray: VkDeviceAddress,
  /// Optional: true
  pub opacityMicromapIndexBuffer: VkDeviceAddress,
  /// Optional: true
  pub instantiationBoundingBoxLimit: VkDeviceAddress,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Send for VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Sync for VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV {
  pub const DEFAULT: Self = Self {
    clusterID: 0,
    clusterFlags: VkClusterAccelerationStructureClusterFlagBitsNV(0),
    triangleCount: 0,
    vertexCount: 0,
    positionTruncateBitCount: 0,
    indexType: 0,
    opacityMicromapIndexType: 0,
    baseGeometryIndexAndGeometryFlags:
      VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV::DEFAULT,
    indexBufferStride: 0,
    vertexBufferStride: 0,
    geometryIndexAndFlagsBufferStride: 0,
    opacityMicromapIndexBufferStride: 0,
    indexBuffer: 0,
    vertexBuffer: 0,
    geometryIndexAndFlagsBuffer: 0,
    opacityMicromapArray: 0,
    opacityMicromapIndexBuffer: 0,
    instantiationBoundingBoxLimit: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_clusterID(mut self, val: u32) -> Self {
    self.clusterID = val;
    self
  }
  #[inline]
  pub const fn with_clusterFlags(
    mut self,
    val: VkClusterAccelerationStructureClusterFlagsNV,
  ) -> Self {
    self.clusterFlags = val;
    self
  }
  #[inline]
  pub const fn with_triangleCount(mut self, val: u32) -> Self {
    self.triangleCount = val;
    self
  }
  #[inline]
  pub const fn with_vertexCount(mut self, val: u32) -> Self {
    self.vertexCount = val;
    self
  }
  #[inline]
  pub const fn with_positionTruncateBitCount(mut self, val: u32) -> Self {
    self.positionTruncateBitCount = val;
    self
  }
  #[inline]
  pub const fn with_indexType(mut self, val: u32) -> Self {
    self.indexType = val;
    self
  }
  #[inline]
  pub const fn with_opacityMicromapIndexType(mut self, val: u32) -> Self {
    self.opacityMicromapIndexType = val;
    self
  }
  #[inline]
  pub const fn with_baseGeometryIndexAndGeometryFlags(
    mut self,
    val: VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
  ) -> Self {
    self.baseGeometryIndexAndGeometryFlags = val;
    self
  }
  #[inline]
  pub const fn with_indexBufferStride(mut self, val: u16) -> Self {
    self.indexBufferStride = val;
    self
  }
  #[inline]
  pub const fn with_vertexBufferStride(mut self, val: u16) -> Self {
    self.vertexBufferStride = val;
    self
  }
  #[inline]
  pub const fn with_geometryIndexAndFlagsBufferStride(mut self, val: u16) -> Self {
    self.geometryIndexAndFlagsBufferStride = val;
    self
  }
  #[inline]
  pub const fn with_opacityMicromapIndexBufferStride(mut self, val: u16) -> Self {
    self.opacityMicromapIndexBufferStride = val;
    self
  }
  #[inline]
  pub const fn with_indexBuffer(mut self, val: VkDeviceAddress) -> Self {
    self.indexBuffer = val;
    self
  }
  #[inline]
  pub const fn with_vertexBuffer(mut self, val: VkDeviceAddress) -> Self {
    self.vertexBuffer = val;
    self
  }
  #[inline]
  pub const fn with_geometryIndexAndFlagsBuffer(mut self, val: VkDeviceAddress) -> Self {
    self.geometryIndexAndFlagsBuffer = val;
    self
  }
  #[inline]
  pub const fn with_opacityMicromapArray(mut self, val: VkDeviceAddress) -> Self {
    self.opacityMicromapArray = val;
    self
  }
  #[inline]
  pub const fn with_opacityMicromapIndexBuffer(mut self, val: VkDeviceAddress) -> Self {
    self.opacityMicromapIndexBuffer = val;
    self
  }
  #[inline]
  pub const fn with_instantiationBoundingBoxLimit(mut self, val: VkDeviceAddress) -> Self {
    self.instantiationBoundingBoxLimit = val;
    self
  }
}
/// [VkClusterAccelerationStructureInstantiateClusterInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureInstantiateClusterInfoNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureInstantiateClusterInfoNV {
  pub clusterIdOffset: u32,
  pub geometryIndexOffset: u32,
  pub reserved: u32,
  pub clusterTemplateAddress: VkDeviceAddress,
  pub vertexBuffer: VkStridedDeviceAddressNV,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Send for VkClusterAccelerationStructureInstantiateClusterInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl Sync for VkClusterAccelerationStructureInstantiateClusterInfoNV {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl VkClusterAccelerationStructureInstantiateClusterInfoNV {
  pub const DEFAULT: Self = Self {
    clusterIdOffset: 0,
    geometryIndexOffset: 0,
    reserved: 0,
    clusterTemplateAddress: 0,
    vertexBuffer: VkStridedDeviceAddressNV::DEFAULT,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_clusterIdOffset(mut self, val: u32) -> Self {
    self.clusterIdOffset = val;
    self
  }
  #[inline]
  pub const fn with_geometryIndexOffset(mut self, val: u32) -> Self {
    self.geometryIndexOffset = val;
    self
  }
  #[inline]
  pub const fn with_reserved(mut self, val: u32) -> Self {
    self.reserved = val;
    self
  }
  #[inline]
  pub const fn with_clusterTemplateAddress(mut self, val: VkDeviceAddress) -> Self {
    self.clusterTemplateAddress = val;
    self
  }
  #[inline]
  pub const fn with_vertexBuffer(mut self, val: VkStridedDeviceAddressNV) -> Self {
    self.vertexBuffer = val;
    self
  }
}
/// [VkClusterAccelerationStructureClustersBottomLevelInputNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureClustersBottomLevelInputNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureClustersBottomLevelInputNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub maxTotalClusterCount: u32,
  pub maxClusterCountPerAccelerationStructure: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Send for VkClusterAccelerationStructureClustersBottomLevelInputNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Sync for VkClusterAccelerationStructureClustersBottomLevelInputNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl<'a> VkClusterAccelerationStructureClustersBottomLevelInputNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV,
    pNext: core::ptr::null_mut(),
    maxTotalClusterCount: 0,
    maxClusterCountPerAccelerationStructure: 0,
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
  pub const fn with_maxTotalClusterCount(mut self, val: u32) -> Self {
    self.maxTotalClusterCount = val;
    self
  }
  #[inline]
  pub const fn with_maxClusterCountPerAccelerationStructure(mut self, val: u32) -> Self {
    self.maxClusterCountPerAccelerationStructure = val;
    self
  }
  #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkClusterAccelerationStructureClustersBottomLevelInputNV<
    'root,
    T: VkPNextExtends<VkClusterAccelerationStructureClustersBottomLevelInputNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkClusterAccelerationStructureTriangleClusterInputNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureTriangleClusterInputNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureTriangleClusterInputNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub vertexFormat: VkFormat,
  pub maxGeometryIndexValue: u32,
  pub maxClusterUniqueGeometryCount: u32,
  pub maxClusterTriangleCount: u32,
  pub maxClusterVertexCount: u32,
  pub maxTotalTriangleCount: u32,
  pub maxTotalVertexCount: u32,
  pub minPositionTruncateBitCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Send for VkClusterAccelerationStructureTriangleClusterInputNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Sync for VkClusterAccelerationStructureTriangleClusterInputNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl<'a> VkClusterAccelerationStructureTriangleClusterInputNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV,
    pNext: core::ptr::null_mut(),
    vertexFormat: VkFormat(0),
    maxGeometryIndexValue: 0,
    maxClusterUniqueGeometryCount: 0,
    maxClusterTriangleCount: 0,
    maxClusterVertexCount: 0,
    maxTotalTriangleCount: 0,
    maxTotalVertexCount: 0,
    minPositionTruncateBitCount: 0,
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
  pub const fn with_vertexFormat(mut self, val: VkFormat) -> Self {
    self.vertexFormat = val;
    self
  }
  #[inline]
  pub const fn with_maxGeometryIndexValue(mut self, val: u32) -> Self {
    self.maxGeometryIndexValue = val;
    self
  }
  #[inline]
  pub const fn with_maxClusterUniqueGeometryCount(mut self, val: u32) -> Self {
    self.maxClusterUniqueGeometryCount = val;
    self
  }
  #[inline]
  pub const fn with_maxClusterTriangleCount(mut self, val: u32) -> Self {
    self.maxClusterTriangleCount = val;
    self
  }
  #[inline]
  pub const fn with_maxClusterVertexCount(mut self, val: u32) -> Self {
    self.maxClusterVertexCount = val;
    self
  }
  #[inline]
  pub const fn with_maxTotalTriangleCount(mut self, val: u32) -> Self {
    self.maxTotalTriangleCount = val;
    self
  }
  #[inline]
  pub const fn with_maxTotalVertexCount(mut self, val: u32) -> Self {
    self.maxTotalVertexCount = val;
    self
  }
  #[inline]
  pub const fn with_minPositionTruncateBitCount(mut self, val: u32) -> Self {
    self.minPositionTruncateBitCount = val;
    self
  }
  #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkClusterAccelerationStructureTriangleClusterInputNV<
    'root,
    T: VkPNextExtends<VkClusterAccelerationStructureTriangleClusterInputNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkClusterAccelerationStructureMoveObjectsInputNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureMoveObjectsInputNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureMoveObjectsInputNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub type_: VkClusterAccelerationStructureTypeNV,
  pub noMoveOverlap: VkBool32,
  pub maxMovedBytes: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Send for VkClusterAccelerationStructureMoveObjectsInputNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Sync for VkClusterAccelerationStructureMoveObjectsInputNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl<'a> VkClusterAccelerationStructureMoveObjectsInputNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV,
    pNext: core::ptr::null_mut(),
    type_: VkClusterAccelerationStructureTypeNV(0),
    noMoveOverlap: 0,
    maxMovedBytes: 0,
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
  pub const fn with_type(mut self, val: VkClusterAccelerationStructureTypeNV) -> Self {
    self.type_ = val;
    self
  }
  #[inline]
  pub const fn with_noMoveOverlap(mut self, val: VkBool32) -> Self {
    self.noMoveOverlap = val;
    self
  }
  #[inline]
  pub const fn with_maxMovedBytes(mut self, val: VkDeviceSize) -> Self {
    self.maxMovedBytes = val;
    self
  }
  #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkClusterAccelerationStructureMoveObjectsInputNV<
    'root,
    T: VkPNextExtends<VkClusterAccelerationStructureMoveObjectsInputNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkClusterAccelerationStructureOpInputNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureOpInputNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClusterAccelerationStructureOpInputNV<'a> {
  pub pClustersBottomLevel: *mut VkClusterAccelerationStructureClustersBottomLevelInputNV<'a>,
  pub pTriangleClusters: *mut VkClusterAccelerationStructureTriangleClusterInputNV<'a>,
  pub pMoveObjects: *mut VkClusterAccelerationStructureMoveObjectsInputNV<'a>,
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Send for VkClusterAccelerationStructureOpInputNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Sync for VkClusterAccelerationStructureOpInputNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl<'a> VkClusterAccelerationStructureOpInputNV<'a> {
  pub const DEFAULT: Self = unsafe {
    Self {
      pClustersBottomLevel: core::mem::zeroed::<
        *mut VkClusterAccelerationStructureClustersBottomLevelInputNV<'a>,
      >(),
    }
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl<'a> core::fmt::Debug for VkClusterAccelerationStructureOpInputNV<'a> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("VkClusterAccelerationStructureOpInputNV")
      .field("pClustersBottomLevel", unsafe {
        &self.pClustersBottomLevel
      })
      .finish()
  }
}
/// [VkClusterAccelerationStructureInputInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureInputInfoNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureInputInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub maxAccelerationStructureCount: u32,
  /// Optional: true
  pub flags: VkBuildAccelerationStructureFlagsKHR,
  pub opType: VkClusterAccelerationStructureOpTypeNV,
  pub opMode: VkClusterAccelerationStructureOpModeNV,
  pub opInput: VkClusterAccelerationStructureOpInputNV<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Send for VkClusterAccelerationStructureInputInfoNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Sync for VkClusterAccelerationStructureInputInfoNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl<'a> VkClusterAccelerationStructureInputInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV,
    pNext: core::ptr::null_mut(),
    maxAccelerationStructureCount: 0,
    flags: VkBuildAccelerationStructureFlagBitsKHR(0),
    opType: VkClusterAccelerationStructureOpTypeNV(0),
    opMode: VkClusterAccelerationStructureOpModeNV(0),
    opInput: VkClusterAccelerationStructureOpInputNV::DEFAULT,
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
  pub const fn with_maxAccelerationStructureCount(mut self, val: u32) -> Self {
    self.maxAccelerationStructureCount = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkBuildAccelerationStructureFlagsKHR) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_opType(mut self, val: VkClusterAccelerationStructureOpTypeNV) -> Self {
    self.opType = val;
    self
  }
  #[inline]
  pub const fn with_opMode(mut self, val: VkClusterAccelerationStructureOpModeNV) -> Self {
    self.opMode = val;
    self
  }
  #[inline]
  pub const fn with_opInput(mut self, val: VkClusterAccelerationStructureOpInputNV<'a>) -> Self {
    self.opInput = val;
    self
  }
  #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkClusterAccelerationStructureInputInfoNV<
    'root,
    T: VkPNextExtends<VkClusterAccelerationStructureInputInfoNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkClusterAccelerationStructureCommandsInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureCommandsInfoNV.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClusterAccelerationStructureCommandsInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub input: VkClusterAccelerationStructureInputInfoNV<'a>,
  /// Optional: true
  pub dstImplicitData: VkDeviceAddress,
  pub scratchData: VkDeviceAddress,
  pub dstAddressesArray: VkStridedDeviceAddressRegionKHR,
  pub dstSizesArray: VkStridedDeviceAddressRegionKHR,
  pub srcInfosArray: VkStridedDeviceAddressRegionKHR,
  /// Optional: true
  pub srcInfosCount: VkDeviceAddress,
  /// Optional: true
  pub addressResolutionFlags: VkClusterAccelerationStructureAddressResolutionFlagsNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Send for VkClusterAccelerationStructureCommandsInfoNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
unsafe impl<'a> Sync for VkClusterAccelerationStructureCommandsInfoNV<'a> {}
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
impl<'a> VkClusterAccelerationStructureCommandsInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV,
    pNext: core::ptr::null_mut(),
    input: VkClusterAccelerationStructureInputInfoNV::DEFAULT,
    dstImplicitData: 0,
    scratchData: 0,
    dstAddressesArray: VkStridedDeviceAddressRegionKHR::DEFAULT,
    dstSizesArray: VkStridedDeviceAddressRegionKHR::DEFAULT,
    srcInfosArray: VkStridedDeviceAddressRegionKHR::DEFAULT,
    srcInfosCount: 0,
    addressResolutionFlags: VkClusterAccelerationStructureAddressResolutionFlagBitsNV(0),
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
  pub const fn with_input(mut self, val: VkClusterAccelerationStructureInputInfoNV<'a>) -> Self {
    self.input = val;
    self
  }
  #[inline]
  pub const fn with_dstImplicitData(mut self, val: VkDeviceAddress) -> Self {
    self.dstImplicitData = val;
    self
  }
  #[inline]
  pub const fn with_scratchData(mut self, val: VkDeviceAddress) -> Self {
    self.scratchData = val;
    self
  }
  #[inline]
  pub const fn with_dstAddressesArray(mut self, val: VkStridedDeviceAddressRegionKHR) -> Self {
    self.dstAddressesArray = val;
    self
  }
  #[inline]
  pub const fn with_dstSizesArray(mut self, val: VkStridedDeviceAddressRegionKHR) -> Self {
    self.dstSizesArray = val;
    self
  }
  #[inline]
  pub const fn with_srcInfosArray(mut self, val: VkStridedDeviceAddressRegionKHR) -> Self {
    self.srcInfosArray = val;
    self
  }
  #[inline]
  pub const fn with_srcInfosCount(mut self, val: VkDeviceAddress) -> Self {
    self.srcInfosCount = val;
    self
  }
  #[inline]
  pub const fn with_addressResolutionFlags(
    mut self,
    val: VkClusterAccelerationStructureAddressResolutionFlagsNV,
  ) -> Self {
    self.addressResolutionFlags = val;
    self
  }
  #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkClusterAccelerationStructureCommandsInfoNV<
    'root,
    T: VkPNextExtends<VkClusterAccelerationStructureCommandsInfoNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceCommandBufferInheritanceFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCommandBufferInheritanceFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_command_buffer_inheritance")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub commandBufferInheritance: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_command_buffer_inheritance")]
unsafe impl<'a> Send for VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_command_buffer_inheritance")]
unsafe impl<'a> Sync for VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_command_buffer_inheritance",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_command_buffer_inheritance",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_command_buffer_inheritance")]
impl<'a> VkPhysicalDeviceCommandBufferInheritanceFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    commandBufferInheritance: 0,
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
  pub const fn with_commandBufferInheritance(mut self, val: VkBool32) -> Self {
    self.commandBufferInheritance = val;
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
/// [VkComputeOccupancyPriorityParametersNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkComputeOccupancyPriorityParametersNV.html)
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkComputeOccupancyPriorityParametersNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub occupancyPriority: f32,
  pub occupancyThrottling: f32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
unsafe impl<'a> Send for VkComputeOccupancyPriorityParametersNV<'a> {}
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
unsafe impl<'a> Sync for VkComputeOccupancyPriorityParametersNV<'a> {}
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
impl<'a> VkComputeOccupancyPriorityParametersNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV,
    pNext: core::ptr::null(),
    occupancyPriority: 0.0f32,
    occupancyThrottling: 0.0f32,
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
  pub const fn with_occupancyPriority(mut self, val: f32) -> Self {
    self.occupancyPriority = val;
    self
  }
  #[inline]
  pub const fn with_occupancyThrottling(mut self, val: f32) -> Self {
    self.occupancyThrottling = val;
    self
  }
  #[cfg(feature = "VK_NV_compute_occupancy_priority")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkComputeOccupancyPriorityParametersNV<
    'root,
    T: VkPNextExtends<VkComputeOccupancyPriorityParametersNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub computeOccupancyPriority: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
unsafe impl<'a> Send for VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
unsafe impl<'a> Sync for VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_compute_occupancy_priority",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_compute_occupancy_priority",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
impl<'a> VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    computeOccupancyPriority: 0,
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
  pub const fn with_computeOccupancyPriority(mut self, val: VkBool32) -> Self {
    self.computeOccupancyPriority = val;
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
/// [VkPhysicalDeviceComputeShaderDerivativesFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_compute_shader_derivatives")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub computeDerivativeGroupQuads: VkBool32,
  pub computeDerivativeGroupLinear: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_compute_shader_derivatives")]
unsafe impl<'a> Send for VkPhysicalDeviceComputeShaderDerivativesFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_compute_shader_derivatives")]
unsafe impl<'a> Sync for VkPhysicalDeviceComputeShaderDerivativesFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_compute_shader_derivatives",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceComputeShaderDerivativesFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_compute_shader_derivatives",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceComputeShaderDerivativesFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_compute_shader_derivatives")]
impl<'a> VkPhysicalDeviceComputeShaderDerivativesFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    computeDerivativeGroupQuads: 0,
    computeDerivativeGroupLinear: 0,
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
  pub const fn with_computeDerivativeGroupQuads(mut self, val: VkBool32) -> Self {
    self.computeDerivativeGroupQuads = val;
    self
  }
  #[inline]
  pub const fn with_computeDerivativeGroupLinear(mut self, val: VkBool32) -> Self {
    self.computeDerivativeGroupLinear = val;
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
/// [VkPhysicalDeviceCooperativeMatrixFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_cooperative_matrix")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub cooperativeMatrix: VkBool32,
  pub cooperativeMatrixRobustBufferAccess: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cooperative_matrix")]
unsafe impl<'a> Send for VkPhysicalDeviceCooperativeMatrixFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_matrix")]
unsafe impl<'a> Sync for VkPhysicalDeviceCooperativeMatrixFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_cooperative_matrix", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCooperativeMatrixFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_cooperative_matrix", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCooperativeMatrixFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_cooperative_matrix")]
impl<'a> VkPhysicalDeviceCooperativeMatrixFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    cooperativeMatrix: 0,
    cooperativeMatrixRobustBufferAccess: 0,
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
  pub const fn with_cooperativeMatrix(mut self, val: VkBool32) -> Self {
    self.cooperativeMatrix = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeMatrixRobustBufferAccess(mut self, val: VkBool32) -> Self {
    self.cooperativeMatrixRobustBufferAccess = val;
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
/// [VkPhysicalDeviceCooperativeMatrixPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_cooperative_matrix")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub cooperativeMatrixSupportedStages: VkShaderStageFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cooperative_matrix")]
unsafe impl<'a> Send for VkPhysicalDeviceCooperativeMatrixPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_matrix")]
unsafe impl<'a> Sync for VkPhysicalDeviceCooperativeMatrixPropertiesNV<'a> {}
#[cfg(all(feature = "VK_NV_cooperative_matrix", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceCooperativeMatrixPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_cooperative_matrix")]
impl<'a> VkPhysicalDeviceCooperativeMatrixPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    cooperativeMatrixSupportedStages: VkShaderStageFlagBits(0),
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
  pub const fn with_cooperativeMatrixSupportedStages(mut self, val: VkShaderStageFlags) -> Self {
    self.cooperativeMatrixSupportedStages = val;
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
/// [VkCooperativeMatrixPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_NV_cooperative_matrix")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCooperativeMatrixPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub MSize: u32,
  pub NSize: u32,
  pub KSize: u32,
  pub AType: VkComponentTypeNV,
  pub BType: VkComponentTypeNV,
  pub CType: VkComponentTypeNV,
  pub DType: VkComponentTypeNV,
  pub scope: VkScopeNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cooperative_matrix")]
unsafe impl<'a> Send for VkCooperativeMatrixPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_matrix")]
unsafe impl<'a> Sync for VkCooperativeMatrixPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_matrix")]
impl<'a> VkCooperativeMatrixPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COOPERATIVE_MATRIX_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    MSize: 0,
    NSize: 0,
    KSize: 0,
    AType: VkComponentTypeNV(0),
    BType: VkComponentTypeNV(0),
    CType: VkComponentTypeNV(0),
    DType: VkComponentTypeNV(0),
    scope: VkScopeNV(0),
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
  pub const fn with_MSize(mut self, val: u32) -> Self {
    self.MSize = val;
    self
  }
  #[inline]
  pub const fn with_NSize(mut self, val: u32) -> Self {
    self.NSize = val;
    self
  }
  #[inline]
  pub const fn with_KSize(mut self, val: u32) -> Self {
    self.KSize = val;
    self
  }
  #[inline]
  pub const fn with_AType(mut self, val: VkComponentTypeNV) -> Self {
    self.AType = val;
    self
  }
  #[inline]
  pub const fn with_BType(mut self, val: VkComponentTypeNV) -> Self {
    self.BType = val;
    self
  }
  #[inline]
  pub const fn with_CType(mut self, val: VkComponentTypeNV) -> Self {
    self.CType = val;
    self
  }
  #[inline]
  pub const fn with_DType(mut self, val: VkComponentTypeNV) -> Self {
    self.DType = val;
    self
  }
  #[inline]
  pub const fn with_scope(mut self, val: VkScopeNV) -> Self {
    self.scope = val;
    self
  }
  #[cfg(feature = "VK_NV_cooperative_matrix")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCooperativeMatrixPropertiesNV<
    'root,
    T: VkPNextExtends<VkCooperativeMatrixPropertiesNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceCooperativeMatrix2FeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrix2FeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_cooperative_matrix2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub cooperativeMatrixWorkgroupScope: VkBool32,
  pub cooperativeMatrixFlexibleDimensions: VkBool32,
  pub cooperativeMatrixReductions: VkBool32,
  pub cooperativeMatrixConversions: VkBool32,
  pub cooperativeMatrixPerElementOperations: VkBool32,
  pub cooperativeMatrixTensorAddressing: VkBool32,
  pub cooperativeMatrixBlockLoads: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cooperative_matrix2")]
unsafe impl<'a> Send for VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_matrix2")]
unsafe impl<'a> Sync for VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_cooperative_matrix2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_cooperative_matrix2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_cooperative_matrix2")]
impl<'a> VkPhysicalDeviceCooperativeMatrix2FeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    cooperativeMatrixWorkgroupScope: 0,
    cooperativeMatrixFlexibleDimensions: 0,
    cooperativeMatrixReductions: 0,
    cooperativeMatrixConversions: 0,
    cooperativeMatrixPerElementOperations: 0,
    cooperativeMatrixTensorAddressing: 0,
    cooperativeMatrixBlockLoads: 0,
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
  pub const fn with_cooperativeMatrixWorkgroupScope(mut self, val: VkBool32) -> Self {
    self.cooperativeMatrixWorkgroupScope = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeMatrixFlexibleDimensions(mut self, val: VkBool32) -> Self {
    self.cooperativeMatrixFlexibleDimensions = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeMatrixReductions(mut self, val: VkBool32) -> Self {
    self.cooperativeMatrixReductions = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeMatrixConversions(mut self, val: VkBool32) -> Self {
    self.cooperativeMatrixConversions = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeMatrixPerElementOperations(mut self, val: VkBool32) -> Self {
    self.cooperativeMatrixPerElementOperations = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeMatrixTensorAddressing(mut self, val: VkBool32) -> Self {
    self.cooperativeMatrixTensorAddressing = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeMatrixBlockLoads(mut self, val: VkBool32) -> Self {
    self.cooperativeMatrixBlockLoads = val;
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
/// [VkPhysicalDeviceCooperativeMatrix2PropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrix2PropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_cooperative_matrix2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCooperativeMatrix2PropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub cooperativeMatrixWorkgroupScopeMaxWorkgroupSize: u32,
  /// Limit Type: [Max]
  pub cooperativeMatrixFlexibleDimensionsMaxDimension: u32,
  /// Limit Type: [Max]
  pub cooperativeMatrixWorkgroupScopeReservedSharedMemory: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cooperative_matrix2")]
unsafe impl<'a> Send for VkPhysicalDeviceCooperativeMatrix2PropertiesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_matrix2")]
unsafe impl<'a> Sync for VkPhysicalDeviceCooperativeMatrix2PropertiesNV<'a> {}
#[cfg(all(feature = "VK_NV_cooperative_matrix2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceCooperativeMatrix2PropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_cooperative_matrix2")]
impl<'a> VkPhysicalDeviceCooperativeMatrix2PropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    cooperativeMatrixWorkgroupScopeMaxWorkgroupSize: 0,
    cooperativeMatrixFlexibleDimensionsMaxDimension: 0,
    cooperativeMatrixWorkgroupScopeReservedSharedMemory: 0,
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
  pub const fn with_cooperativeMatrixWorkgroupScopeMaxWorkgroupSize(mut self, val: u32) -> Self {
    self.cooperativeMatrixWorkgroupScopeMaxWorkgroupSize = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeMatrixFlexibleDimensionsMaxDimension(mut self, val: u32) -> Self {
    self.cooperativeMatrixFlexibleDimensionsMaxDimension = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeMatrixWorkgroupScopeReservedSharedMemory(
    mut self,
    val: u32,
  ) -> Self {
    self.cooperativeMatrixWorkgroupScopeReservedSharedMemory = val;
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
/// [VkCooperativeMatrixFlexibleDimensionsPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixFlexibleDimensionsPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_NV_cooperative_matrix2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCooperativeMatrixFlexibleDimensionsPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub MGranularity: u32,
  pub NGranularity: u32,
  pub KGranularity: u32,
  pub AType: VkComponentTypeKHR,
  pub BType: VkComponentTypeKHR,
  pub CType: VkComponentTypeKHR,
  pub ResultType: VkComponentTypeKHR,
  pub saturatingAccumulation: VkBool32,
  pub scope: VkScopeKHR,
  pub workgroupInvocations: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cooperative_matrix2")]
unsafe impl<'a> Send for VkCooperativeMatrixFlexibleDimensionsPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_matrix2")]
unsafe impl<'a> Sync for VkCooperativeMatrixFlexibleDimensionsPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_matrix2")]
impl<'a> VkCooperativeMatrixFlexibleDimensionsPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    MGranularity: 0,
    NGranularity: 0,
    KGranularity: 0,
    AType: VkComponentTypeKHR(0),
    BType: VkComponentTypeKHR(0),
    CType: VkComponentTypeKHR(0),
    ResultType: VkComponentTypeKHR(0),
    saturatingAccumulation: 0,
    scope: VkScopeKHR(0),
    workgroupInvocations: 0,
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
  pub const fn with_MGranularity(mut self, val: u32) -> Self {
    self.MGranularity = val;
    self
  }
  #[inline]
  pub const fn with_NGranularity(mut self, val: u32) -> Self {
    self.NGranularity = val;
    self
  }
  #[inline]
  pub const fn with_KGranularity(mut self, val: u32) -> Self {
    self.KGranularity = val;
    self
  }
  #[inline]
  pub const fn with_AType(mut self, val: VkComponentTypeKHR) -> Self {
    self.AType = val;
    self
  }
  #[inline]
  pub const fn with_BType(mut self, val: VkComponentTypeKHR) -> Self {
    self.BType = val;
    self
  }
  #[inline]
  pub const fn with_CType(mut self, val: VkComponentTypeKHR) -> Self {
    self.CType = val;
    self
  }
  #[inline]
  pub const fn with_ResultType(mut self, val: VkComponentTypeKHR) -> Self {
    self.ResultType = val;
    self
  }
  #[inline]
  pub const fn with_saturatingAccumulation(mut self, val: VkBool32) -> Self {
    self.saturatingAccumulation = val;
    self
  }
  #[inline]
  pub const fn with_scope(mut self, val: VkScopeKHR) -> Self {
    self.scope = val;
    self
  }
  #[inline]
  pub const fn with_workgroupInvocations(mut self, val: u32) -> Self {
    self.workgroupInvocations = val;
    self
  }
  #[cfg(feature = "VK_NV_cooperative_matrix2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCooperativeMatrixFlexibleDimensionsPropertiesNV<
    'root,
    T: VkPNextExtends<VkCooperativeMatrixFlexibleDimensionsPropertiesNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_cooperative_matrix_decode_vector")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_DECODE_VECTOR_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub cooperativeMatrixDecodeVector: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cooperative_matrix_decode_vector")]
unsafe impl<'a> Send for VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_matrix_decode_vector")]
unsafe impl<'a> Sync for VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_cooperative_matrix_decode_vector",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_cooperative_matrix_decode_vector",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_cooperative_matrix_decode_vector")]
impl<'a> VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_DECODE_VECTOR_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    cooperativeMatrixDecodeVector: 0,
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
  pub const fn with_cooperativeMatrixDecodeVector(mut self, val: VkBool32) -> Self {
    self.cooperativeMatrixDecodeVector = val;
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
/// [VkPhysicalDeviceCooperativeVectorFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeVectorFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_cooperative_vector")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCooperativeVectorFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub cooperativeVector: VkBool32,
  pub cooperativeVectorTraining: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cooperative_vector")]
unsafe impl<'a> Send for VkPhysicalDeviceCooperativeVectorFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_vector")]
unsafe impl<'a> Sync for VkPhysicalDeviceCooperativeVectorFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_cooperative_vector", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCooperativeVectorFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_cooperative_vector", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCooperativeVectorFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_cooperative_vector")]
impl<'a> VkPhysicalDeviceCooperativeVectorFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    cooperativeVector: 0,
    cooperativeVectorTraining: 0,
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
  pub const fn with_cooperativeVector(mut self, val: VkBool32) -> Self {
    self.cooperativeVector = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeVectorTraining(mut self, val: VkBool32) -> Self {
    self.cooperativeVectorTraining = val;
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
/// [VkCooperativeVectorPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeVectorPropertiesNV.html)
#[cfg(feature = "VK_NV_cooperative_vector")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCooperativeVectorPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_COOPERATIVE_VECTOR_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub inputType: VkComponentTypeKHR,
  pub inputInterpretation: VkComponentTypeKHR,
  pub matrixInterpretation: VkComponentTypeKHR,
  pub biasInterpretation: VkComponentTypeKHR,
  pub resultType: VkComponentTypeKHR,
  pub transpose: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cooperative_vector")]
unsafe impl<'a> Send for VkCooperativeVectorPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_vector")]
unsafe impl<'a> Sync for VkCooperativeVectorPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_vector")]
impl<'a> VkCooperativeVectorPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COOPERATIVE_VECTOR_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    inputType: VkComponentTypeKHR(0),
    inputInterpretation: VkComponentTypeKHR(0),
    matrixInterpretation: VkComponentTypeKHR(0),
    biasInterpretation: VkComponentTypeKHR(0),
    resultType: VkComponentTypeKHR(0),
    transpose: 0,
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
  pub const fn with_inputType(mut self, val: VkComponentTypeKHR) -> Self {
    self.inputType = val;
    self
  }
  #[inline]
  pub const fn with_inputInterpretation(mut self, val: VkComponentTypeKHR) -> Self {
    self.inputInterpretation = val;
    self
  }
  #[inline]
  pub const fn with_matrixInterpretation(mut self, val: VkComponentTypeKHR) -> Self {
    self.matrixInterpretation = val;
    self
  }
  #[inline]
  pub const fn with_biasInterpretation(mut self, val: VkComponentTypeKHR) -> Self {
    self.biasInterpretation = val;
    self
  }
  #[inline]
  pub const fn with_resultType(mut self, val: VkComponentTypeKHR) -> Self {
    self.resultType = val;
    self
  }
  #[inline]
  pub const fn with_transpose(mut self, val: VkBool32) -> Self {
    self.transpose = val;
    self
  }
  #[cfg(feature = "VK_NV_cooperative_vector")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCooperativeVectorPropertiesNV<
    'root,
    T: VkPNextExtends<VkCooperativeVectorPropertiesNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceCooperativeVectorPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCooperativeVectorPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_cooperative_vector")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCooperativeVectorPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub cooperativeVectorSupportedStages: VkShaderStageFlags,
  /// Limit Type: [Exact]
  pub cooperativeVectorTrainingFloat16Accumulation: VkBool32,
  /// Limit Type: [Exact]
  pub cooperativeVectorTrainingFloat32Accumulation: VkBool32,
  /// Limit Type: [Max]
  pub maxCooperativeVectorComponents: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cooperative_vector")]
unsafe impl<'a> Send for VkPhysicalDeviceCooperativeVectorPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_vector")]
unsafe impl<'a> Sync for VkPhysicalDeviceCooperativeVectorPropertiesNV<'a> {}
#[cfg(all(feature = "VK_NV_cooperative_vector", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceCooperativeVectorPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_cooperative_vector")]
impl<'a> VkPhysicalDeviceCooperativeVectorPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    cooperativeVectorSupportedStages: VkShaderStageFlagBits(0),
    cooperativeVectorTrainingFloat16Accumulation: 0,
    cooperativeVectorTrainingFloat32Accumulation: 0,
    maxCooperativeVectorComponents: 0,
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
  pub const fn with_cooperativeVectorSupportedStages(mut self, val: VkShaderStageFlags) -> Self {
    self.cooperativeVectorSupportedStages = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeVectorTrainingFloat16Accumulation(mut self, val: VkBool32) -> Self {
    self.cooperativeVectorTrainingFloat16Accumulation = val;
    self
  }
  #[inline]
  pub const fn with_cooperativeVectorTrainingFloat32Accumulation(mut self, val: VkBool32) -> Self {
    self.cooperativeVectorTrainingFloat32Accumulation = val;
    self
  }
  #[inline]
  pub const fn with_maxCooperativeVectorComponents(mut self, val: u32) -> Self {
    self.maxCooperativeVectorComponents = val;
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
/// [VkConvertCooperativeVectorMatrixInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkConvertCooperativeVectorMatrixInfoNV.html)
#[cfg(feature = "VK_NV_cooperative_vector")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkConvertCooperativeVectorMatrixInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcSize: usize,
  pub srcData: VkDeviceOrHostAddressConstKHR<'a>,
  /// Optional: pointer required, values optional if pointer not null
  pub pDstSize: *mut usize,
  pub dstData: VkDeviceOrHostAddressKHR<'a>,
  pub srcComponentType: VkComponentTypeKHR,
  pub dstComponentType: VkComponentTypeKHR,
  pub numRows: u32,
  pub numColumns: u32,
  pub srcLayout: VkCooperativeVectorMatrixLayoutNV,
  pub srcStride: usize,
  pub dstLayout: VkCooperativeVectorMatrixLayoutNV,
  pub dstStride: usize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cooperative_vector")]
unsafe impl<'a> Send for VkConvertCooperativeVectorMatrixInfoNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_vector")]
unsafe impl<'a> Sync for VkConvertCooperativeVectorMatrixInfoNV<'a> {}
#[cfg(feature = "VK_NV_cooperative_vector")]
impl<'a> VkConvertCooperativeVectorMatrixInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV,
    pNext: core::ptr::null(),
    srcSize: 0,
    srcData: VkDeviceOrHostAddressConstKHR::DEFAULT,
    pDstSize: core::ptr::null_mut(),
    dstData: VkDeviceOrHostAddressKHR::DEFAULT,
    srcComponentType: VkComponentTypeKHR(0),
    dstComponentType: VkComponentTypeKHR(0),
    numRows: 0,
    numColumns: 0,
    srcLayout: VkCooperativeVectorMatrixLayoutNV(0),
    srcStride: 0,
    dstLayout: VkCooperativeVectorMatrixLayoutNV(0),
    dstStride: 0,
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
  pub const fn with_srcSize(mut self, val: usize) -> Self {
    self.srcSize = val;
    self
  }
  #[inline]
  pub const fn with_srcData(mut self, val: VkDeviceOrHostAddressConstKHR<'a>) -> Self {
    self.srcData = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDstSize(mut self, val: *mut usize) -> Self {
    self.pDstSize = val;
    self
  }
  #[inline]
  pub const fn with_dstData(mut self, val: VkDeviceOrHostAddressKHR<'a>) -> Self {
    self.dstData = val;
    self
  }
  #[inline]
  pub const fn with_srcComponentType(mut self, val: VkComponentTypeKHR) -> Self {
    self.srcComponentType = val;
    self
  }
  #[inline]
  pub const fn with_dstComponentType(mut self, val: VkComponentTypeKHR) -> Self {
    self.dstComponentType = val;
    self
  }
  #[inline]
  pub const fn with_numRows(mut self, val: u32) -> Self {
    self.numRows = val;
    self
  }
  #[inline]
  pub const fn with_numColumns(mut self, val: u32) -> Self {
    self.numColumns = val;
    self
  }
  #[inline]
  pub const fn with_srcLayout(mut self, val: VkCooperativeVectorMatrixLayoutNV) -> Self {
    self.srcLayout = val;
    self
  }
  #[inline]
  pub const fn with_srcStride(mut self, val: usize) -> Self {
    self.srcStride = val;
    self
  }
  #[inline]
  pub const fn with_dstLayout(mut self, val: VkCooperativeVectorMatrixLayoutNV) -> Self {
    self.dstLayout = val;
    self
  }
  #[inline]
  pub const fn with_dstStride(mut self, val: usize) -> Self {
    self.dstStride = val;
    self
  }
  #[cfg(feature = "VK_NV_cooperative_vector")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkConvertCooperativeVectorMatrixInfoNV<
    'root,
    T: VkPNextExtends<VkConvertCooperativeVectorMatrixInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCopyMemoryIndirectCommandNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryIndirectCommandNV.html)
#[cfg(feature = "VK_NV_copy_memory_indirect")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyMemoryIndirectCommandNV {
  pub srcAddress: VkDeviceAddress,
  pub dstAddress: VkDeviceAddress,
  pub size: VkDeviceSize,
}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
unsafe impl Send for VkCopyMemoryIndirectCommandNV {}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
unsafe impl Sync for VkCopyMemoryIndirectCommandNV {}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
impl VkCopyMemoryIndirectCommandNV {
  pub const DEFAULT: Self = Self {
    srcAddress: 0,
    dstAddress: 0,
    size: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_srcAddress(mut self, val: VkDeviceAddress) -> Self {
    self.srcAddress = val;
    self
  }
  #[inline]
  pub const fn with_dstAddress(mut self, val: VkDeviceAddress) -> Self {
    self.dstAddress = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
}
/// [VkCopyMemoryToImageIndirectCommandNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryToImageIndirectCommandNV.html)
#[cfg(feature = "VK_NV_copy_memory_indirect")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyMemoryToImageIndirectCommandNV {
  pub srcAddress: VkDeviceAddress,
  pub bufferRowLength: u32,
  pub bufferImageHeight: u32,
  pub imageSubresource: VkImageSubresourceLayers,
  pub imageOffset: VkOffset3D,
  pub imageExtent: VkExtent3D,
}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
unsafe impl Send for VkCopyMemoryToImageIndirectCommandNV {}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
unsafe impl Sync for VkCopyMemoryToImageIndirectCommandNV {}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
impl VkCopyMemoryToImageIndirectCommandNV {
  pub const DEFAULT: Self = Self {
    srcAddress: 0,
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
  pub const fn with_srcAddress(mut self, val: VkDeviceAddress) -> Self {
    self.srcAddress = val;
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
/// [VkPhysicalDeviceCopyMemoryIndirectFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCopyMemoryIndirectFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_copy_memory_indirect")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub indirectCopy: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
unsafe impl<'a> Send for VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
unsafe impl<'a> Sync for VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_copy_memory_indirect",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_copy_memory_indirect",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
impl<'a> VkPhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    indirectCopy: 0,
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
  pub const fn with_indirectCopy(mut self, val: VkBool32) -> Self {
    self.indirectCopy = val;
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
/// [VkPhysicalDeviceCopyMemoryIndirectPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCopyMemoryIndirectPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_copy_memory_indirect")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCopyMemoryIndirectPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask],  No Auto-Validity
  pub supportedQueues: VkQueueFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
unsafe impl<'a> Send for VkPhysicalDeviceCopyMemoryIndirectPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
unsafe impl<'a> Sync for VkPhysicalDeviceCopyMemoryIndirectPropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_copy_memory_indirect",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceCopyMemoryIndirectPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_copy_memory_indirect")]
impl<'a> VkPhysicalDeviceCopyMemoryIndirectPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    supportedQueues: VkQueueFlagBits(0),
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
  pub const fn with_supportedQueues(mut self, val: VkQueueFlags) -> Self {
    self.supportedQueues = val;
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
/// [VkPhysicalDeviceCornerSampledImageFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCornerSampledImageFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_corner_sampled_image")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCornerSampledImageFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub cornerSampledImage: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_corner_sampled_image")]
unsafe impl<'a> Send for VkPhysicalDeviceCornerSampledImageFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_corner_sampled_image")]
unsafe impl<'a> Sync for VkPhysicalDeviceCornerSampledImageFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_corner_sampled_image",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCornerSampledImageFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_corner_sampled_image",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCornerSampledImageFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_corner_sampled_image")]
impl<'a> VkPhysicalDeviceCornerSampledImageFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    cornerSampledImage: 0,
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
  pub const fn with_cornerSampledImage(mut self, val: VkBool32) -> Self {
    self.cornerSampledImage = val;
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
/// [VkPipelineCoverageReductionStateCreateFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageReductionStateCreateFlagsNV.html)
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
pub type VkPipelineCoverageReductionStateCreateFlagsNV = VkFlags;
/// [VkPhysicalDeviceCoverageReductionModeFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCoverageReductionModeFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCoverageReductionModeFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub coverageReductionMode: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
unsafe impl<'a> Send for VkPhysicalDeviceCoverageReductionModeFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
unsafe impl<'a> Sync for VkPhysicalDeviceCoverageReductionModeFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_coverage_reduction_mode",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCoverageReductionModeFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_coverage_reduction_mode",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCoverageReductionModeFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
impl<'a> VkPhysicalDeviceCoverageReductionModeFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    coverageReductionMode: 0,
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
  pub const fn with_coverageReductionMode(mut self, val: VkBool32) -> Self {
    self.coverageReductionMode = val;
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
/// [VkPipelineCoverageReductionStateCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageReductionStateCreateInfoNV.html)
///
/// **Extends:** VkPipelineMultisampleStateCreateInfo.
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCoverageReductionStateCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineCoverageReductionStateCreateFlagsNV,
  pub coverageReductionMode: VkCoverageReductionModeNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
unsafe impl<'a> Send for VkPipelineCoverageReductionStateCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
unsafe impl<'a> Sync for VkPipelineCoverageReductionStateCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_coverage_reduction_mode",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineMultisampleStateCreateInfo<'root>>
  for VkPipelineCoverageReductionStateCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
impl<'a> VkPipelineCoverageReductionStateCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    flags: 0,
    coverageReductionMode: VkCoverageReductionModeNV(0),
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
  pub const fn with_flags(mut self, val: VkPipelineCoverageReductionStateCreateFlagsNV) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_coverageReductionMode(mut self, val: VkCoverageReductionModeNV) -> Self {
    self.coverageReductionMode = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineMultisampleStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineMultisampleStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkFramebufferMixedSamplesCombinationNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferMixedSamplesCombinationNV.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFramebufferMixedSamplesCombinationNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub coverageReductionMode: VkCoverageReductionModeNV,
  pub rasterizationSamples: VkSampleCountFlagBits,
  pub depthStencilSamples: VkSampleCountFlags,
  pub colorSamples: VkSampleCountFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
unsafe impl<'a> Send for VkFramebufferMixedSamplesCombinationNV<'a> {}
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
unsafe impl<'a> Sync for VkFramebufferMixedSamplesCombinationNV<'a> {}
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
impl<'a> VkFramebufferMixedSamplesCombinationNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV,
    pNext: core::ptr::null_mut(),
    coverageReductionMode: VkCoverageReductionModeNV(0),
    rasterizationSamples: VkSampleCountFlagBits(0),
    depthStencilSamples: VkSampleCountFlagBits(0),
    colorSamples: VkSampleCountFlagBits(0),
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
  pub const fn with_coverageReductionMode(mut self, val: VkCoverageReductionModeNV) -> Self {
    self.coverageReductionMode = val;
    self
  }
  #[inline]
  pub const fn with_rasterizationSamples(mut self, val: VkSampleCountFlagBits) -> Self {
    self.rasterizationSamples = val;
    self
  }
  #[inline]
  pub const fn with_depthStencilSamples(mut self, val: VkSampleCountFlags) -> Self {
    self.depthStencilSamples = val;
    self
  }
  #[inline]
  pub const fn with_colorSamples(mut self, val: VkSampleCountFlags) -> Self {
    self.colorSamples = val;
    self
  }
  #[cfg(feature = "VK_NV_coverage_reduction_mode")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkFramebufferMixedSamplesCombinationNV<
    'root,
    T: VkPNextExtends<VkFramebufferMixedSamplesCombinationNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkCudaModuleNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaModuleNV.html)
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkCudaModuleNV(pub *mut c_void);
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl VkCudaModuleNV {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl Default for VkCudaModuleNV {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl Send for VkCudaModuleNV {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl Sync for VkCudaModuleNV {}
/// [VkCudaFunctionNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaFunctionNV.html)
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkCudaFunctionNV(pub *mut c_void);
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl VkCudaFunctionNV {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl Default for VkCudaFunctionNV {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl Send for VkCudaFunctionNV {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl Sync for VkCudaFunctionNV {}
/// [VkCudaModuleCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaModuleCreateInfoNV.html)
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCudaModuleCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_CUDA_MODULE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub dataSize: usize,
  /// Length: dataSize
  pub pData: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'a> Send for VkCudaModuleCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'a> Sync for VkCudaModuleCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl<'a> VkCudaModuleCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CUDA_MODULE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    dataSize: 0,
    pData: core::ptr::null(),
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
  pub const fn with_dataSize(mut self, val: usize) -> Self {
    self.dataSize = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pData(mut self, val: &'a [u8]) -> Self {
    self.dataSize = val.len() as usize;
    self.pData = val.as_ptr().cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCudaModuleCreateInfoNV<
    'root,
    T: VkPNextExtends<VkCudaModuleCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCudaFunctionCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaFunctionCreateInfoNV.html)
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCudaFunctionCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_CUDA_FUNCTION_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub module: VkCudaModuleNV,
  /// Length: null-terminated
  pub pName: *const c_char,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'a> Send for VkCudaFunctionCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'a> Sync for VkCudaFunctionCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl<'a> VkCudaFunctionCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CUDA_FUNCTION_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    module: VkCudaModuleNV::DEFAULT,
    pName: core::ptr::null(),
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
  pub const fn with_module(mut self, val: VkCudaModuleNV) -> Self {
    self.module = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pName(mut self, val: *const c_char) -> Self {
    self.pName = val;
    self
  }
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCudaFunctionCreateInfoNV<
    'root,
    T: VkPNextExtends<VkCudaFunctionCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCudaLaunchInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaLaunchInfoNV.html)
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCudaLaunchInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_CUDA_LAUNCH_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub function: VkCudaFunctionNV,
  pub gridDimX: u32,
  pub gridDimY: u32,
  pub gridDimZ: u32,
  pub blockDimX: u32,
  pub blockDimY: u32,
  pub blockDimZ: u32,
  pub sharedMemBytes: u32,
  /// Optional: true
  pub paramCount: usize,
  /// Length: paramCount,  No Auto-Validity
  pub pParams: *const *const c_void,
  /// Optional: true
  pub extraCount: usize,
  /// Length: extraCount,  No Auto-Validity
  pub pExtras: *const *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'a> Send for VkCudaLaunchInfoNV<'a> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'a> Sync for VkCudaLaunchInfoNV<'a> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl<'a> VkCudaLaunchInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CUDA_LAUNCH_INFO_NV,
    pNext: core::ptr::null(),
    function: VkCudaFunctionNV::DEFAULT,
    gridDimX: 0,
    gridDimY: 0,
    gridDimZ: 0,
    blockDimX: 0,
    blockDimY: 0,
    blockDimZ: 0,
    sharedMemBytes: 0,
    paramCount: 0,
    pParams: core::ptr::null(),
    extraCount: 0,
    pExtras: core::ptr::null(),
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
  pub const fn with_function(mut self, val: VkCudaFunctionNV) -> Self {
    self.function = val;
    self
  }
  #[inline]
  pub const fn with_gridDimX(mut self, val: u32) -> Self {
    self.gridDimX = val;
    self
  }
  #[inline]
  pub const fn with_gridDimY(mut self, val: u32) -> Self {
    self.gridDimY = val;
    self
  }
  #[inline]
  pub const fn with_gridDimZ(mut self, val: u32) -> Self {
    self.gridDimZ = val;
    self
  }
  #[inline]
  pub const fn with_blockDimX(mut self, val: u32) -> Self {
    self.blockDimX = val;
    self
  }
  #[inline]
  pub const fn with_blockDimY(mut self, val: u32) -> Self {
    self.blockDimY = val;
    self
  }
  #[inline]
  pub const fn with_blockDimZ(mut self, val: u32) -> Self {
    self.blockDimZ = val;
    self
  }
  #[inline]
  pub const fn with_sharedMemBytes(mut self, val: u32) -> Self {
    self.sharedMemBytes = val;
    self
  }
  #[inline]
  pub const fn with_paramCount(mut self, val: usize) -> Self {
    self.paramCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pParams(mut self, val: &'a [*const c_void]) -> Self {
    self.paramCount = val.len() as usize;
    self.pParams = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_extraCount(mut self, val: usize) -> Self {
    self.extraCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pExtras(mut self, val: &'a [*const c_void]) -> Self {
    self.extraCount = val.len() as usize;
    self.pExtras = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCudaLaunchInfoNV<
    'root,
    T: VkPNextExtends<VkCudaLaunchInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceCudaKernelLaunchFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCudaKernelLaunchFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub cudaKernelLaunchFeatures: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'a> Send for VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'a> Sync for VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_cuda_kernel_launch", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_cuda_kernel_launch", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl<'a> VkPhysicalDeviceCudaKernelLaunchFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    cudaKernelLaunchFeatures: 0,
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
  pub const fn with_cudaKernelLaunchFeatures(mut self, val: VkBool32) -> Self {
    self.cudaKernelLaunchFeatures = val;
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
/// [VkPhysicalDeviceCudaKernelLaunchPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCudaKernelLaunchPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceCudaKernelLaunchPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub computeCapabilityMinor: u32,
  /// Limit Type: [Min]
  pub computeCapabilityMajor: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'a> Send for VkPhysicalDeviceCudaKernelLaunchPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'a> Sync for VkPhysicalDeviceCudaKernelLaunchPropertiesNV<'a> {}
#[cfg(all(feature = "VK_NV_cuda_kernel_launch", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceCudaKernelLaunchPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl<'a> VkPhysicalDeviceCudaKernelLaunchPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    computeCapabilityMinor: 0,
    computeCapabilityMajor: 0,
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
  pub const fn with_computeCapabilityMinor(mut self, val: u32) -> Self {
    self.computeCapabilityMinor = val;
    self
  }
  #[inline]
  pub const fn with_computeCapabilityMajor(mut self, val: u32) -> Self {
    self.computeCapabilityMajor = val;
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
/// [VkDedicatedAllocationImageCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkDedicatedAllocationImageCreateInfoNV.html)
///
/// **Extends:** VkImageCreateInfo.
#[cfg(feature = "VK_NV_dedicated_allocation")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDedicatedAllocationImageCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub dedicatedAllocation: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'a> Send for VkDedicatedAllocationImageCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'a> Sync for VkDedicatedAllocationImageCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_dedicated_allocation",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkDedicatedAllocationImageCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'a> VkDedicatedAllocationImageCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    dedicatedAllocation: 0,
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
  pub const fn with_dedicatedAllocation(mut self, val: VkBool32) -> Self {
    self.dedicatedAllocation = val;
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
/// [VkDedicatedAllocationBufferCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkDedicatedAllocationBufferCreateInfoNV.html)
///
/// **Extends:** VkBufferCreateInfo.
#[cfg(feature = "VK_NV_dedicated_allocation")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDedicatedAllocationBufferCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub dedicatedAllocation: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'a> Send for VkDedicatedAllocationBufferCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'a> Sync for VkDedicatedAllocationBufferCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_dedicated_allocation",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkBufferCreateInfo<'root>>
  for VkDedicatedAllocationBufferCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'a> VkDedicatedAllocationBufferCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    dedicatedAllocation: 0,
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
  pub const fn with_dedicatedAllocation(mut self, val: VkBool32) -> Self {
    self.dedicatedAllocation = val;
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
/// [VkDedicatedAllocationMemoryAllocateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkDedicatedAllocationMemoryAllocateInfoNV.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_NV_dedicated_allocation")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV
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
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'a> Send for VkDedicatedAllocationMemoryAllocateInfoNV<'a> {}
#[cfg(feature = "VK_NV_dedicated_allocation")]
unsafe impl<'a> Sync for VkDedicatedAllocationMemoryAllocateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_dedicated_allocation",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkDedicatedAllocationMemoryAllocateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_dedicated_allocation")]
impl<'a> VkDedicatedAllocationMemoryAllocateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
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
/// [VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub dedicatedAllocationImageAliasing: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
unsafe impl<'a> Send for VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
unsafe impl<'a> Sync for VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_dedicated_allocation_image_aliasing",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_dedicated_allocation_image_aliasing",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
impl<'a> VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    dedicatedAllocationImageAliasing: 0,
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
  pub const fn with_dedicatedAllocationImageAliasing(mut self, val: VkBool32) -> Self {
    self.dedicatedAllocationImageAliasing = val;
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
/// [VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_descriptor_pool_overallocation")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub descriptorPoolOverallocation: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_descriptor_pool_overallocation")]
unsafe impl<'a> Send for VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_descriptor_pool_overallocation")]
unsafe impl<'a> Sync for VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_descriptor_pool_overallocation",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_descriptor_pool_overallocation",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_descriptor_pool_overallocation")]
impl<'a> VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    descriptorPoolOverallocation: 0,
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
  pub const fn with_descriptorPoolOverallocation(mut self, val: VkBool32) -> Self {
    self.descriptorPoolOverallocation = val;
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
/// [VkQueueFamilyCheckpointPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyCheckpointPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkQueueFamilyProperties2.
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueueFamilyCheckpointPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub checkpointExecutionStageMask: VkPipelineStageFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
unsafe impl<'a> Send for VkQueueFamilyCheckpointPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
unsafe impl<'a> Sync for VkQueueFamilyCheckpointPropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_device_diagnostic_checkpoints",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkQueueFamilyProperties2<'root>>
  for VkQueueFamilyCheckpointPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
impl<'a> VkQueueFamilyCheckpointPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    checkpointExecutionStageMask: VkPipelineStageFlagBits(0),
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
  pub const fn with_checkpointExecutionStageMask(mut self, val: VkPipelineStageFlags) -> Self {
    self.checkpointExecutionStageMask = val;
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
/// [VkCheckpointDataNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCheckpointDataNV.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCheckpointDataNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub stage: VkPipelineStageFlagBits,
  /// No Auto-Validity
  pub pCheckpointMarker: *mut c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
unsafe impl<'a> Send for VkCheckpointDataNV<'a> {}
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
unsafe impl<'a> Sync for VkCheckpointDataNV<'a> {}
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
impl<'a> VkCheckpointDataNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CHECKPOINT_DATA_NV,
    pNext: core::ptr::null_mut(),
    stage: VkPipelineStageFlagBits(0),
    pCheckpointMarker: core::ptr::null_mut(),
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
  pub const fn with_stage(mut self, val: VkPipelineStageFlagBits) -> Self {
    self.stage = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCheckpointMarker(mut self, val: *mut c_void) -> Self {
    self.pCheckpointMarker = val;
    self
  }
  #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCheckpointDataNV<
    'root,
    T: VkPNextExtends<VkCheckpointDataNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkQueueFamilyCheckpointProperties2NV](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyCheckpointProperties2NV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkQueueFamilyProperties2.
///
/// **Availability:** depends on `VK_VERSION_1_3 + VK_KHR_synchronization2`.
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
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueueFamilyCheckpointProperties2NV<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub checkpointExecutionStageMask: VkPipelineStageFlags2,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
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
unsafe impl<'a> Send for VkQueueFamilyCheckpointProperties2NV<'a> {}
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
unsafe impl<'a> Sync for VkQueueFamilyCheckpointProperties2NV<'a> {}
#[cfg(all(
  any(
    all(
      feature = "VK_NV_device_diagnostic_checkpoints",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_KHR_synchronization2",
      feature = "VK_NV_device_diagnostic_checkpoints"
    )
  ),
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkQueueFamilyProperties2<'root>>
  for VkQueueFamilyCheckpointProperties2NV<'child>
{
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
impl<'a> VkQueueFamilyCheckpointProperties2NV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV,
    pNext: core::ptr::null_mut(),
    checkpointExecutionStageMask: VkPipelineStageFlagBits2(0),
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
  pub const fn with_checkpointExecutionStageMask(mut self, val: VkPipelineStageFlags2) -> Self {
    self.checkpointExecutionStageMask = val;
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
/// [VkCheckpointData2NV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCheckpointData2NV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Availability:** depends on `VK_VERSION_1_3 + VK_KHR_synchronization2`.
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
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCheckpointData2NV<'a> {
  /// Values: VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub stage: VkPipelineStageFlags2,
  /// No Auto-Validity
  pub pCheckpointMarker: *mut c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
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
unsafe impl<'a> Send for VkCheckpointData2NV<'a> {}
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
unsafe impl<'a> Sync for VkCheckpointData2NV<'a> {}
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
impl<'a> VkCheckpointData2NV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::CHECKPOINT_DATA_2_NV,
    pNext: core::ptr::null_mut(),
    stage: VkPipelineStageFlagBits2(0),
    pCheckpointMarker: core::ptr::null_mut(),
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
  pub const fn with_stage(mut self, val: VkPipelineStageFlags2) -> Self {
    self.stage = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCheckpointMarker(mut self, val: *mut c_void) -> Self {
    self.pCheckpointMarker = val;
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
  pub const fn with_pNext_chain_VkCheckpointData2NV<
    'root,
    T: VkPNextExtends<VkCheckpointData2NV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDeviceDiagnosticsConfigFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceDiagnosticsConfigFlagsNV.html)
#[cfg(feature = "VK_NV_device_diagnostics_config")]
pub type VkDeviceDiagnosticsConfigFlagsNV = VkDeviceDiagnosticsConfigFlagBitsNV;
/// [VkPhysicalDeviceDiagnosticsConfigFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_device_diagnostics_config")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub diagnosticsConfig: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_diagnostics_config")]
unsafe impl<'a> Send for VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_device_diagnostics_config")]
unsafe impl<'a> Sync for VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_device_diagnostics_config",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_device_diagnostics_config",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_device_diagnostics_config")]
impl<'a> VkPhysicalDeviceDiagnosticsConfigFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    diagnosticsConfig: 0,
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
  pub const fn with_diagnosticsConfig(mut self, val: VkBool32) -> Self {
    self.diagnosticsConfig = val;
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
/// [VkDeviceDiagnosticsConfigCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceDiagnosticsConfigCreateInfoNV.html)
///
/// **Extends:** VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_device_diagnostics_config")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceDiagnosticsConfigCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkDeviceDiagnosticsConfigFlagsNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_diagnostics_config")]
unsafe impl<'a> Send for VkDeviceDiagnosticsConfigCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_diagnostics_config")]
unsafe impl<'a> Sync for VkDeviceDiagnosticsConfigCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_device_diagnostics_config",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkDeviceDiagnosticsConfigCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_device_diagnostics_config")]
impl<'a> VkDeviceDiagnosticsConfigCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    flags: VkDeviceDiagnosticsConfigFlagBitsNV(0),
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
  pub const fn with_flags(mut self, val: VkDeviceDiagnosticsConfigFlagsNV) -> Self {
    self.flags = val;
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
/// [VkIndirectCommandsLayoutUsageFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagsNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
pub type VkIndirectCommandsLayoutUsageFlagsNV = VkIndirectCommandsLayoutUsageFlagBitsNV;
/// [VkIndirectStateFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectStateFlagsNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
pub type VkIndirectStateFlagsNV = VkIndirectStateFlagBitsNV;
/// [VkIndirectCommandsLayoutNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkIndirectCommandsLayoutNV(pub *mut c_void);
#[cfg(feature = "VK_NV_device_generated_commands")]
impl VkIndirectCommandsLayoutNV {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl Default for VkIndirectCommandsLayoutNV {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Send for VkIndirectCommandsLayoutNV {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Sync for VkIndirectCommandsLayoutNV {}
/// [VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub deviceGeneratedCommands: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Send for VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Sync for VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_device_generated_commands",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_device_generated_commands",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl<'a> VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    deviceGeneratedCommands: 0,
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
  pub const fn with_deviceGeneratedCommands(mut self, val: VkBool32) -> Self {
    self.deviceGeneratedCommands = val;
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
/// [VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxGraphicsShaderGroupCount: u32,
  /// Limit Type: [Max]
  pub maxIndirectSequenceCount: u32,
  /// Limit Type: [Max]
  pub maxIndirectCommandsTokenCount: u32,
  /// Limit Type: [Max]
  pub maxIndirectCommandsStreamCount: u32,
  /// Limit Type: [Max]
  pub maxIndirectCommandsTokenOffset: u32,
  /// Limit Type: [Max]
  pub maxIndirectCommandsStreamStride: u32,
  /// Limit Type: [Min]
  pub minSequencesCountBufferOffsetAlignment: u32,
  /// Limit Type: [Min]
  pub minSequencesIndexBufferOffsetAlignment: u32,
  /// Limit Type: [Min]
  pub minIndirectCommandsBufferOffsetAlignment: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Send for VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Sync for VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_device_generated_commands",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl<'a> VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    maxGraphicsShaderGroupCount: 0,
    maxIndirectSequenceCount: 0,
    maxIndirectCommandsTokenCount: 0,
    maxIndirectCommandsStreamCount: 0,
    maxIndirectCommandsTokenOffset: 0,
    maxIndirectCommandsStreamStride: 0,
    minSequencesCountBufferOffsetAlignment: 0,
    minSequencesIndexBufferOffsetAlignment: 0,
    minIndirectCommandsBufferOffsetAlignment: 0,
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
  pub const fn with_maxGraphicsShaderGroupCount(mut self, val: u32) -> Self {
    self.maxGraphicsShaderGroupCount = val;
    self
  }
  #[inline]
  pub const fn with_maxIndirectSequenceCount(mut self, val: u32) -> Self {
    self.maxIndirectSequenceCount = val;
    self
  }
  #[inline]
  pub const fn with_maxIndirectCommandsTokenCount(mut self, val: u32) -> Self {
    self.maxIndirectCommandsTokenCount = val;
    self
  }
  #[inline]
  pub const fn with_maxIndirectCommandsStreamCount(mut self, val: u32) -> Self {
    self.maxIndirectCommandsStreamCount = val;
    self
  }
  #[inline]
  pub const fn with_maxIndirectCommandsTokenOffset(mut self, val: u32) -> Self {
    self.maxIndirectCommandsTokenOffset = val;
    self
  }
  #[inline]
  pub const fn with_maxIndirectCommandsStreamStride(mut self, val: u32) -> Self {
    self.maxIndirectCommandsStreamStride = val;
    self
  }
  #[inline]
  pub const fn with_minSequencesCountBufferOffsetAlignment(mut self, val: u32) -> Self {
    self.minSequencesCountBufferOffsetAlignment = val;
    self
  }
  #[inline]
  pub const fn with_minSequencesIndexBufferOffsetAlignment(mut self, val: u32) -> Self {
    self.minSequencesIndexBufferOffsetAlignment = val;
    self
  }
  #[inline]
  pub const fn with_minIndirectCommandsBufferOffsetAlignment(mut self, val: u32) -> Self {
    self.minIndirectCommandsBufferOffsetAlignment = val;
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
/// [VkGraphicsShaderGroupCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsShaderGroupCreateInfoNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGraphicsShaderGroupCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub stageCount: u32,
  /// Length: stageCount
  pub pStages: *const VkPipelineShaderStageCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub pTessellationState: *const VkPipelineTessellationStateCreateInfo<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Send for VkGraphicsShaderGroupCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Sync for VkGraphicsShaderGroupCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl<'a> VkGraphicsShaderGroupCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    stageCount: 0,
    pStages: core::ptr::null(),
    pVertexInputState: core::ptr::null(),
    pTessellationState: core::ptr::null(),
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
  pub const fn with_stageCount(mut self, val: u32) -> Self {
    self.stageCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pStages(mut self, val: &'a [VkPipelineShaderStageCreateInfo<'a>]) -> Self {
    self.stageCount = val.len() as u32;
    self.pStages = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pVertexInputState(
    mut self,
    val: *const VkPipelineVertexInputStateCreateInfo<'a>,
  ) -> Self {
    self.pVertexInputState = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pTessellationState(
    mut self,
    val: *const VkPipelineTessellationStateCreateInfo<'a>,
  ) -> Self {
    self.pTessellationState = val;
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGraphicsShaderGroupCreateInfoNV<
    'root,
    T: VkPNextExtends<VkGraphicsShaderGroupCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkGraphicsPipelineShaderGroupsCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineShaderGroupsCreateInfoNV.html)
///
/// **Extends:** VkGraphicsPipelineCreateInfo.
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGraphicsPipelineShaderGroupsCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub groupCount: u32,
  /// Length: groupCount
  pub pGroups: *const VkGraphicsShaderGroupCreateInfoNV<'a>,
  /// Optional: true
  pub pipelineCount: u32,
  /// Length: pipelineCount
  pub pPipelines: *const VkPipeline,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Send for VkGraphicsPipelineShaderGroupsCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Sync for VkGraphicsPipelineShaderGroupsCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_device_generated_commands",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkGraphicsPipelineShaderGroupsCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl<'a> VkGraphicsPipelineShaderGroupsCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    groupCount: 0,
    pGroups: core::ptr::null(),
    pipelineCount: 0,
    pPipelines: core::ptr::null(),
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
  pub const fn with_groupCount(mut self, val: u32) -> Self {
    self.groupCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pGroups(mut self, val: &'a [VkGraphicsShaderGroupCreateInfoNV<'a>]) -> Self {
    self.groupCount = val.len() as u32;
    self.pGroups = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_pipelineCount(mut self, val: u32) -> Self {
    self.pipelineCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPipelines(mut self, val: &'a [VkPipeline]) -> Self {
    self.pipelineCount = val.len() as u32;
    self.pPipelines = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGraphicsPipelineCreateInfo<
    'root,
    T: VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBindShaderGroupIndirectCommandNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindShaderGroupIndirectCommandNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindShaderGroupIndirectCommandNV {
  pub groupIndex: u32,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Send for VkBindShaderGroupIndirectCommandNV {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Sync for VkBindShaderGroupIndirectCommandNV {}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl VkBindShaderGroupIndirectCommandNV {
  pub const DEFAULT: Self = Self { groupIndex: 0 };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_groupIndex(mut self, val: u32) -> Self {
    self.groupIndex = val;
    self
  }
}
/// [VkBindIndexBufferIndirectCommandNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindIndexBufferIndirectCommandNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindIndexBufferIndirectCommandNV {
  pub bufferAddress: VkDeviceAddress,
  pub size: u32,
  pub indexType: VkIndexType,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Send for VkBindIndexBufferIndirectCommandNV {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Sync for VkBindIndexBufferIndirectCommandNV {}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl VkBindIndexBufferIndirectCommandNV {
  pub const DEFAULT: Self = Self {
    bufferAddress: 0,
    size: 0,
    indexType: VkIndexType(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_bufferAddress(mut self, val: VkDeviceAddress) -> Self {
    self.bufferAddress = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: u32) -> Self {
    self.size = val;
    self
  }
  #[inline]
  pub const fn with_indexType(mut self, val: VkIndexType) -> Self {
    self.indexType = val;
    self
  }
}
/// [VkBindVertexBufferIndirectCommandNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindVertexBufferIndirectCommandNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindVertexBufferIndirectCommandNV {
  pub bufferAddress: VkDeviceAddress,
  pub size: u32,
  pub stride: u32,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Send for VkBindVertexBufferIndirectCommandNV {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Sync for VkBindVertexBufferIndirectCommandNV {}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl VkBindVertexBufferIndirectCommandNV {
  pub const DEFAULT: Self = Self {
    bufferAddress: 0,
    size: 0,
    stride: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_bufferAddress(mut self, val: VkDeviceAddress) -> Self {
    self.bufferAddress = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: u32) -> Self {
    self.size = val;
    self
  }
  #[inline]
  pub const fn with_stride(mut self, val: u32) -> Self {
    self.stride = val;
    self
  }
}
/// [VkSetStateFlagsIndirectCommandNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSetStateFlagsIndirectCommandNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSetStateFlagsIndirectCommandNV {
  pub data: u32,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Send for VkSetStateFlagsIndirectCommandNV {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Sync for VkSetStateFlagsIndirectCommandNV {}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl VkSetStateFlagsIndirectCommandNV {
  pub const DEFAULT: Self = Self { data: 0 };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_data(mut self, val: u32) -> Self {
    self.data = val;
    self
  }
}
/// [VkIndirectCommandsStreamNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsStreamNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkIndirectCommandsStreamNV {
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Send for VkIndirectCommandsStreamNV {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl Sync for VkIndirectCommandsStreamNV {}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl VkIndirectCommandsStreamNV {
  pub const DEFAULT: Self = Self {
    buffer: VkBuffer::DEFAULT,
    offset: 0,
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
  pub const fn with_offset(mut self, val: VkDeviceSize) -> Self {
    self.offset = val;
    self
  }
}
/// [VkIndirectCommandsLayoutTokenNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutTokenNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkIndirectCommandsLayoutTokenNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub tokenType: VkIndirectCommandsTokenTypeNV,
  pub stream: u32,
  pub offset: u32,
  pub vertexBindingUnit: u32,
  pub vertexDynamicStride: VkBool32,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// Optional: true
  pub pushconstantPipelineLayout: VkPipelineLayout,
  #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
  /// Optional: true
  pub pushconstantPipelineLayout: *mut c_void,
  /// Optional: true
  pub pushconstantShaderStageFlags: VkShaderStageFlags,
  pub pushconstantOffset: u32,
  pub pushconstantSize: u32,
  /// Optional: true
  pub indirectStateFlags: VkIndirectStateFlagsNV,
  /// Optional: true
  pub indexTypeCount: u32,
  /// Length: indexTypeCount
  pub pIndexTypes: *const VkIndexType,
  /// Length: indexTypeCount
  pub pIndexTypeValues: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Send for VkIndirectCommandsLayoutTokenNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Sync for VkIndirectCommandsLayoutTokenNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl<'a> VkIndirectCommandsLayoutTokenNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV,
    pNext: core::ptr::null(),
    tokenType: VkIndirectCommandsTokenTypeNV(0),
    stream: 0,
    offset: 0,
    vertexBindingUnit: 0,
    vertexDynamicStride: 0,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pushconstantPipelineLayout: VkPipelineLayout::DEFAULT,
    #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
    pushconstantPipelineLayout: core::ptr::null_mut(),
    pushconstantShaderStageFlags: VkShaderStageFlagBits(0),
    pushconstantOffset: 0,
    pushconstantSize: 0,
    indirectStateFlags: VkIndirectStateFlagBitsNV(0),
    indexTypeCount: 0,
    pIndexTypes: core::ptr::null(),
    pIndexTypeValues: core::ptr::null(),
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
  pub const fn with_tokenType(mut self, val: VkIndirectCommandsTokenTypeNV) -> Self {
    self.tokenType = val;
    self
  }
  #[inline]
  pub const fn with_stream(mut self, val: u32) -> Self {
    self.stream = val;
    self
  }
  #[inline]
  pub const fn with_offset(mut self, val: u32) -> Self {
    self.offset = val;
    self
  }
  #[inline]
  pub const fn with_vertexBindingUnit(mut self, val: u32) -> Self {
    self.vertexBindingUnit = val;
    self
  }
  #[inline]
  pub const fn with_vertexDynamicStride(mut self, val: VkBool32) -> Self {
    self.vertexDynamicStride = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub const fn with_pushconstantPipelineLayout(mut self, val: VkPipelineLayout) -> Self {
    self.pushconstantPipelineLayout = val;
    self
  }
  #[inline]
  pub const fn with_pushconstantShaderStageFlags(mut self, val: VkShaderStageFlags) -> Self {
    self.pushconstantShaderStageFlags = val;
    self
  }
  #[inline]
  pub const fn with_pushconstantOffset(mut self, val: u32) -> Self {
    self.pushconstantOffset = val;
    self
  }
  #[inline]
  pub const fn with_pushconstantSize(mut self, val: u32) -> Self {
    self.pushconstantSize = val;
    self
  }
  #[inline]
  pub const fn with_indirectStateFlags(mut self, val: VkIndirectStateFlagsNV) -> Self {
    self.indirectStateFlags = val;
    self
  }
  #[inline]
  pub const fn with_indexTypeCount(mut self, val: u32) -> Self {
    self.indexTypeCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pIndexTypes(mut self, val: &'a [VkIndexType]) -> Self {
    self.indexTypeCount = val.len() as u32;
    self.pIndexTypes = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pIndexTypeValues(mut self, val: &'a [u32]) -> Self {
    self.indexTypeCount = val.len() as u32;
    self.pIndexTypeValues = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure every provided array constrained by `indexTypeCount` has the same length. Optional pointer arguments may be null, but non-null pointers must be valid for that same length and outlive any use of this struct instance.
  #[inline]
  pub const fn with_indexTypeCount_slices(
    mut self,
    pIndexTypes: &'a [VkIndexType],
    pIndexTypeValues: &'a [u32],
  ) -> Self {
    let len = pIndexTypes.len();
    self.indexTypeCount = len as u32;
    self.pIndexTypes = pIndexTypes.as_ptr();
    self.pIndexTypeValues = pIndexTypeValues.as_ptr();
    self
  }
  #[cfg(all(
    feature = "VK_EXT_descriptor_heap",
    feature = "VK_NV_device_generated_commands"
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkIndirectCommandsLayoutPushDataTokenNV<'child>(
    mut self,
    val: &'a VkIndirectCommandsLayoutPushDataTokenNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkIndirectCommandsLayoutPushDataTokenNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkIndirectCommandsLayoutTokenNV<
    'root,
    T: VkPNextExtends<VkIndirectCommandsLayoutTokenNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkIndirectCommandsLayoutCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutCreateInfoNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkIndirectCommandsLayoutCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkIndirectCommandsLayoutUsageFlagsNV,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub tokenCount: u32,
  /// Length: tokenCount
  pub pTokens: *const VkIndirectCommandsLayoutTokenNV<'a>,
  pub streamCount: u32,
  /// Length: streamCount
  pub pStreamStrides: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Send for VkIndirectCommandsLayoutCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Sync for VkIndirectCommandsLayoutCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl<'a> VkIndirectCommandsLayoutCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    flags: VkIndirectCommandsLayoutUsageFlagBitsNV(0),
    pipelineBindPoint: VkPipelineBindPoint(0),
    tokenCount: 0,
    pTokens: core::ptr::null(),
    streamCount: 0,
    pStreamStrides: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkIndirectCommandsLayoutUsageFlagsNV) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_pipelineBindPoint(mut self, val: VkPipelineBindPoint) -> Self {
    self.pipelineBindPoint = val;
    self
  }
  #[inline]
  pub const fn with_tokenCount(mut self, val: u32) -> Self {
    self.tokenCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pTokens(mut self, val: &'a [VkIndirectCommandsLayoutTokenNV<'a>]) -> Self {
    self.tokenCount = val.len() as u32;
    self.pTokens = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_streamCount(mut self, val: u32) -> Self {
    self.streamCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pStreamStrides(mut self, val: &'a [u32]) -> Self {
    self.streamCount = val.len() as u32;
    self.pStreamStrides = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkIndirectCommandsLayoutCreateInfoNV<
    'root,
    T: VkPNextExtends<VkIndirectCommandsLayoutCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkGeneratedCommandsInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeneratedCommandsInfoNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGeneratedCommandsInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pipelineBindPoint: VkPipelineBindPoint,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// Optional: true
  pub pipeline: VkPipeline,
  #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
  /// Optional: true
  pub pipeline: *mut c_void,
  pub indirectCommandsLayout: VkIndirectCommandsLayoutNV,
  pub streamCount: u32,
  /// Length: streamCount
  pub pStreams: *const VkIndirectCommandsStreamNV,
  pub sequencesCount: u32,
  pub preprocessBuffer: VkBuffer,
  pub preprocessOffset: VkDeviceSize,
  pub preprocessSize: VkDeviceSize,
  /// Optional: true
  pub sequencesCountBuffer: VkBuffer,
  pub sequencesCountOffset: VkDeviceSize,
  /// Optional: true
  pub sequencesIndexBuffer: VkBuffer,
  pub sequencesIndexOffset: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Send for VkGeneratedCommandsInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Sync for VkGeneratedCommandsInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl<'a> VkGeneratedCommandsInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GENERATED_COMMANDS_INFO_NV,
    pNext: core::ptr::null(),
    pipelineBindPoint: VkPipelineBindPoint(0),
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pipeline: VkPipeline::DEFAULT,
    #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
    pipeline: core::ptr::null_mut(),
    indirectCommandsLayout: VkIndirectCommandsLayoutNV::DEFAULT,
    streamCount: 0,
    pStreams: core::ptr::null(),
    sequencesCount: 0,
    preprocessBuffer: VkBuffer::DEFAULT,
    preprocessOffset: 0,
    preprocessSize: 0,
    sequencesCountBuffer: VkBuffer::DEFAULT,
    sequencesCountOffset: 0,
    sequencesIndexBuffer: VkBuffer::DEFAULT,
    sequencesIndexOffset: 0,
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
  pub const fn with_pipelineBindPoint(mut self, val: VkPipelineBindPoint) -> Self {
    self.pipelineBindPoint = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub const fn with_pipeline(mut self, val: VkPipeline) -> Self {
    self.pipeline = val;
    self
  }
  #[inline]
  pub const fn with_indirectCommandsLayout(mut self, val: VkIndirectCommandsLayoutNV) -> Self {
    self.indirectCommandsLayout = val;
    self
  }
  #[inline]
  pub const fn with_streamCount(mut self, val: u32) -> Self {
    self.streamCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pStreams(mut self, val: &'a [VkIndirectCommandsStreamNV]) -> Self {
    self.streamCount = val.len() as u32;
    self.pStreams = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_sequencesCount(mut self, val: u32) -> Self {
    self.sequencesCount = val;
    self
  }
  #[inline]
  pub const fn with_preprocessBuffer(mut self, val: VkBuffer) -> Self {
    self.preprocessBuffer = val;
    self
  }
  #[inline]
  pub const fn with_preprocessOffset(mut self, val: VkDeviceSize) -> Self {
    self.preprocessOffset = val;
    self
  }
  #[inline]
  pub const fn with_preprocessSize(mut self, val: VkDeviceSize) -> Self {
    self.preprocessSize = val;
    self
  }
  #[inline]
  pub const fn with_sequencesCountBuffer(mut self, val: VkBuffer) -> Self {
    self.sequencesCountBuffer = val;
    self
  }
  #[inline]
  pub const fn with_sequencesCountOffset(mut self, val: VkDeviceSize) -> Self {
    self.sequencesCountOffset = val;
    self
  }
  #[inline]
  pub const fn with_sequencesIndexBuffer(mut self, val: VkBuffer) -> Self {
    self.sequencesIndexBuffer = val;
    self
  }
  #[inline]
  pub const fn with_sequencesIndexOffset(mut self, val: VkDeviceSize) -> Self {
    self.sequencesIndexOffset = val;
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGeneratedCommandsInfoNV<
    'root,
    T: VkPNextExtends<VkGeneratedCommandsInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkGeneratedCommandsMemoryRequirementsInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeneratedCommandsMemoryRequirementsInfoNV.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGeneratedCommandsMemoryRequirementsInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pipelineBindPoint: VkPipelineBindPoint,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// Optional: true
  pub pipeline: VkPipeline,
  #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
  /// Optional: true
  pub pipeline: *mut c_void,
  pub indirectCommandsLayout: VkIndirectCommandsLayoutNV,
  pub maxSequencesCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Send for VkGeneratedCommandsMemoryRequirementsInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'a> Sync for VkGeneratedCommandsMemoryRequirementsInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl<'a> VkGeneratedCommandsMemoryRequirementsInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV,
    pNext: core::ptr::null(),
    pipelineBindPoint: VkPipelineBindPoint(0),
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pipeline: VkPipeline::DEFAULT,
    #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
    pipeline: core::ptr::null_mut(),
    indirectCommandsLayout: VkIndirectCommandsLayoutNV::DEFAULT,
    maxSequencesCount: 0,
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
  pub const fn with_pipelineBindPoint(mut self, val: VkPipelineBindPoint) -> Self {
    self.pipelineBindPoint = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub const fn with_pipeline(mut self, val: VkPipeline) -> Self {
    self.pipeline = val;
    self
  }
  #[inline]
  pub const fn with_indirectCommandsLayout(mut self, val: VkIndirectCommandsLayoutNV) -> Self {
    self.indirectCommandsLayout = val;
    self
  }
  #[inline]
  pub const fn with_maxSequencesCount(mut self, val: u32) -> Self {
    self.maxSequencesCount = val;
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGeneratedCommandsMemoryRequirementsInfoNV<
    'root,
    T: VkPNextExtends<VkGeneratedCommandsMemoryRequirementsInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkComputePipelineIndirectBufferInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkComputePipelineIndirectBufferInfoNV.html)
///
/// **Extends:** VkComputePipelineCreateInfo.
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkComputePipelineIndirectBufferInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub deviceAddress: VkDeviceAddress,
  pub size: VkDeviceSize,
  /// Optional: true
  pub pipelineDeviceAddressCaptureReplay: VkDeviceAddress,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
unsafe impl<'a> Send for VkComputePipelineIndirectBufferInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
unsafe impl<'a> Sync for VkComputePipelineIndirectBufferInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_device_generated_commands_compute",
  feature = "VK_COMPUTE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkComputePipelineCreateInfo<'root>>
  for VkComputePipelineIndirectBufferInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
impl<'a> VkComputePipelineIndirectBufferInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV,
    pNext: core::ptr::null(),
    deviceAddress: 0,
    size: 0,
    pipelineDeviceAddressCaptureReplay: 0,
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
  pub const fn with_deviceAddress(mut self, val: VkDeviceAddress) -> Self {
    self.deviceAddress = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[inline]
  pub const fn with_pipelineDeviceAddressCaptureReplay(mut self, val: VkDeviceAddress) -> Self {
    self.pipelineDeviceAddressCaptureReplay = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkComputePipelineCreateInfo<
    'root,
    T: VkPNextExtends<VkComputePipelineCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub deviceGeneratedCompute: VkBool32,
  pub deviceGeneratedComputePipelines: VkBool32,
  pub deviceGeneratedComputeCaptureReplay: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
unsafe impl<'a> Send for VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
unsafe impl<'a> Sync for VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_device_generated_commands_compute",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_device_generated_commands_compute",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
impl<'a> VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    deviceGeneratedCompute: 0,
    deviceGeneratedComputePipelines: 0,
    deviceGeneratedComputeCaptureReplay: 0,
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
  pub const fn with_deviceGeneratedCompute(mut self, val: VkBool32) -> Self {
    self.deviceGeneratedCompute = val;
    self
  }
  #[inline]
  pub const fn with_deviceGeneratedComputePipelines(mut self, val: VkBool32) -> Self {
    self.deviceGeneratedComputePipelines = val;
    self
  }
  #[inline]
  pub const fn with_deviceGeneratedComputeCaptureReplay(mut self, val: VkBool32) -> Self {
    self.deviceGeneratedComputeCaptureReplay = val;
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
/// [VkPipelineIndirectDeviceAddressInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineIndirectDeviceAddressInfoNV.html)
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineIndirectDeviceAddressInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pipelineBindPoint: VkPipelineBindPoint,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub pipeline: VkPipeline,
  #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
  pub pipeline: *mut c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
unsafe impl<'a> Send for VkPipelineIndirectDeviceAddressInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
unsafe impl<'a> Sync for VkPipelineIndirectDeviceAddressInfoNV<'a> {}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
impl<'a> VkPipelineIndirectDeviceAddressInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV,
    pNext: core::ptr::null(),
    pipelineBindPoint: VkPipelineBindPoint(0),
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pipeline: VkPipeline::DEFAULT,
    #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
    pipeline: core::ptr::null_mut(),
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
  pub const fn with_pipelineBindPoint(mut self, val: VkPipelineBindPoint) -> Self {
    self.pipelineBindPoint = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub const fn with_pipeline(mut self, val: VkPipeline) -> Self {
    self.pipeline = val;
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands_compute")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineIndirectDeviceAddressInfoNV<
    'root,
    T: VkPNextExtends<VkPipelineIndirectDeviceAddressInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBindPipelineIndirectCommandNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindPipelineIndirectCommandNV.html)
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindPipelineIndirectCommandNV {
  pub pipelineAddress: VkDeviceAddress,
}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
unsafe impl Send for VkBindPipelineIndirectCommandNV {}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
unsafe impl Sync for VkBindPipelineIndirectCommandNV {}
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
impl VkBindPipelineIndirectCommandNV {
  pub const DEFAULT: Self = Self { pipelineAddress: 0 };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_pipelineAddress(mut self, val: VkDeviceAddress) -> Self {
    self.pipelineAddress = val;
    self
  }
}
/// [VkPhysicalDeviceDisplacementMicromapFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDisplacementMicromapFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_displacement_micromap")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDisplacementMicromapFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub displacementMicromap: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_displacement_micromap")]
unsafe impl<'a> Send for VkPhysicalDeviceDisplacementMicromapFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_displacement_micromap")]
unsafe impl<'a> Sync for VkPhysicalDeviceDisplacementMicromapFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_displacement_micromap",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDisplacementMicromapFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_displacement_micromap",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDisplacementMicromapFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_displacement_micromap")]
impl<'a> VkPhysicalDeviceDisplacementMicromapFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    displacementMicromap: 0,
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
  pub const fn with_displacementMicromap(mut self, val: VkBool32) -> Self {
    self.displacementMicromap = val;
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
/// [VkPhysicalDeviceDisplacementMicromapPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDisplacementMicromapPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_displacement_micromap")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDisplacementMicromapPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxDisplacementMicromapSubdivisionLevel: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_displacement_micromap")]
unsafe impl<'a> Send for VkPhysicalDeviceDisplacementMicromapPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_displacement_micromap")]
unsafe impl<'a> Sync for VkPhysicalDeviceDisplacementMicromapPropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_displacement_micromap",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceDisplacementMicromapPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_displacement_micromap")]
impl<'a> VkPhysicalDeviceDisplacementMicromapPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    maxDisplacementMicromapSubdivisionLevel: 0,
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
  pub const fn with_maxDisplacementMicromapSubdivisionLevel(mut self, val: u32) -> Self {
    self.maxDisplacementMicromapSubdivisionLevel = val;
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
/// [VkAccelerationStructureTrianglesDisplacementMicromapNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureTrianglesDisplacementMicromapNV.html)
///
/// **Extends:** VkAccelerationStructureGeometryTrianglesDataKHR.
#[cfg(feature = "VK_NV_displacement_micromap")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureTrianglesDisplacementMicromapNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub displacementBiasAndScaleFormat: VkFormat,
  pub displacementVectorFormat: VkFormat,
  /// No Auto-Validity
  pub displacementBiasAndScaleBuffer: VkDeviceOrHostAddressConstKHR<'a>,
  pub displacementBiasAndScaleStride: VkDeviceSize,
  /// No Auto-Validity
  pub displacementVectorBuffer: VkDeviceOrHostAddressConstKHR<'a>,
  pub displacementVectorStride: VkDeviceSize,
  /// No Auto-Validity
  pub displacedMicromapPrimitiveFlags: VkDeviceOrHostAddressConstKHR<'a>,
  pub displacedMicromapPrimitiveFlagsStride: VkDeviceSize,
  pub indexType: VkIndexType,
  /// No Auto-Validity
  pub indexBuffer: VkDeviceOrHostAddressConstKHR<'a>,
  pub indexStride: VkDeviceSize,
  pub baseTriangle: u32,
  /// Optional: true
  pub usageCountsCount: u32,
  /// Optional: true,  Length: usageCountsCount
  pub pUsageCounts: *const VkMicromapUsageEXT,
  /// Optional: pointer optional, values required if pointer not null,  Length: usageCountsCount,1
  pub ppUsageCounts: *const *const VkMicromapUsageEXT,
  /// Optional: true
  pub micromap: VkMicromapEXT,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_displacement_micromap")]
unsafe impl<'a> Send for VkAccelerationStructureTrianglesDisplacementMicromapNV<'a> {}
#[cfg(feature = "VK_NV_displacement_micromap")]
unsafe impl<'a> Sync for VkAccelerationStructureTrianglesDisplacementMicromapNV<'a> {}
#[cfg(all(
  feature = "VK_NV_displacement_micromap",
  feature = "VK_KHR_acceleration_structure"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAccelerationStructureGeometryTrianglesDataKHR<'root>>
  for VkAccelerationStructureTrianglesDisplacementMicromapNV<'child>
{
}
#[cfg(feature = "VK_NV_displacement_micromap")]
impl<'a> VkAccelerationStructureTrianglesDisplacementMicromapNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV,
    pNext: core::ptr::null_mut(),
    displacementBiasAndScaleFormat: VkFormat(0),
    displacementVectorFormat: VkFormat(0),
    displacementBiasAndScaleBuffer: VkDeviceOrHostAddressConstKHR::DEFAULT,
    displacementBiasAndScaleStride: 0,
    displacementVectorBuffer: VkDeviceOrHostAddressConstKHR::DEFAULT,
    displacementVectorStride: 0,
    displacedMicromapPrimitiveFlags: VkDeviceOrHostAddressConstKHR::DEFAULT,
    displacedMicromapPrimitiveFlagsStride: 0,
    indexType: VkIndexType(0),
    indexBuffer: VkDeviceOrHostAddressConstKHR::DEFAULT,
    indexStride: 0,
    baseTriangle: 0,
    usageCountsCount: 0,
    pUsageCounts: core::ptr::null(),
    ppUsageCounts: core::ptr::null(),
    micromap: VkMicromapEXT::DEFAULT,
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
  pub const fn with_displacementBiasAndScaleFormat(mut self, val: VkFormat) -> Self {
    self.displacementBiasAndScaleFormat = val;
    self
  }
  #[inline]
  pub const fn with_displacementVectorFormat(mut self, val: VkFormat) -> Self {
    self.displacementVectorFormat = val;
    self
  }
  #[inline]
  pub const fn with_displacementBiasAndScaleBuffer(
    mut self,
    val: VkDeviceOrHostAddressConstKHR<'a>,
  ) -> Self {
    self.displacementBiasAndScaleBuffer = val;
    self
  }
  #[inline]
  pub const fn with_displacementBiasAndScaleStride(mut self, val: VkDeviceSize) -> Self {
    self.displacementBiasAndScaleStride = val;
    self
  }
  #[inline]
  pub const fn with_displacementVectorBuffer(
    mut self,
    val: VkDeviceOrHostAddressConstKHR<'a>,
  ) -> Self {
    self.displacementVectorBuffer = val;
    self
  }
  #[inline]
  pub const fn with_displacementVectorStride(mut self, val: VkDeviceSize) -> Self {
    self.displacementVectorStride = val;
    self
  }
  #[inline]
  pub const fn with_displacedMicromapPrimitiveFlags(
    mut self,
    val: VkDeviceOrHostAddressConstKHR<'a>,
  ) -> Self {
    self.displacedMicromapPrimitiveFlags = val;
    self
  }
  #[inline]
  pub const fn with_displacedMicromapPrimitiveFlagsStride(mut self, val: VkDeviceSize) -> Self {
    self.displacedMicromapPrimitiveFlagsStride = val;
    self
  }
  #[inline]
  pub const fn with_indexType(mut self, val: VkIndexType) -> Self {
    self.indexType = val;
    self
  }
  #[inline]
  pub const fn with_indexBuffer(mut self, val: VkDeviceOrHostAddressConstKHR<'a>) -> Self {
    self.indexBuffer = val;
    self
  }
  #[inline]
  pub const fn with_indexStride(mut self, val: VkDeviceSize) -> Self {
    self.indexStride = val;
    self
  }
  #[inline]
  pub const fn with_baseTriangle(mut self, val: u32) -> Self {
    self.baseTriangle = val;
    self
  }
  #[inline]
  pub const fn with_usageCountsCount(mut self, val: u32) -> Self {
    self.usageCountsCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pUsageCounts(mut self, val: &'a [VkMicromapUsageEXT]) -> Self {
    self.usageCountsCount = val.len() as u32;
    self.pUsageCounts = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_ppUsageCounts(mut self, val: &'a [*const VkMicromapUsageEXT]) -> Self {
    self.usageCountsCount = val.len() as u32;
    self.ppUsageCounts = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_micromap(mut self, val: VkMicromapEXT) -> Self {
    self.micromap = val;
    self
  }
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAccelerationStructureGeometryTrianglesDataKHR<
    'root,
    T: VkPNextExtends<VkAccelerationStructureGeometryTrianglesDataKHR<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDisplaySurfaceStereoCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplaySurfaceStereoCreateInfoNV.html)
///
/// **Extends:** VkDisplaySurfaceCreateInfoKHR.
#[cfg(feature = "VK_NV_display_stereo")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDisplaySurfaceStereoCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_DISPLAY_SURFACE_STEREO_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub stereoType: VkDisplaySurfaceStereoTypeNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_display_stereo")]
unsafe impl<'a> Send for VkDisplaySurfaceStereoCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_display_stereo")]
unsafe impl<'a> Sync for VkDisplaySurfaceStereoCreateInfoNV<'a> {}
#[cfg(all(feature = "VK_NV_display_stereo", feature = "VK_KHR_display"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDisplaySurfaceCreateInfoKHR<'root>>
  for VkDisplaySurfaceStereoCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_display_stereo")]
impl<'a> VkDisplaySurfaceStereoCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DISPLAY_SURFACE_STEREO_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    stereoType: VkDisplaySurfaceStereoTypeNV(0),
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
  pub const fn with_stereoType(mut self, val: VkDisplaySurfaceStereoTypeNV) -> Self {
    self.stereoType = val;
    self
  }
  #[cfg(feature = "VK_KHR_display")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDisplaySurfaceCreateInfoKHR<
    'root,
    T: VkPNextExtends<VkDisplaySurfaceCreateInfoKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDisplayModeStereoPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeStereoPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkDisplayModeProperties2KHR.
#[cfg(feature = "VK_NV_display_stereo")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDisplayModeStereoPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_DISPLAY_MODE_STEREO_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub hdmi3DSupported: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_display_stereo")]
unsafe impl<'a> Send for VkDisplayModeStereoPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_display_stereo")]
unsafe impl<'a> Sync for VkDisplayModeStereoPropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_display_stereo",
  feature = "VK_KHR_get_display_properties2"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDisplayModeProperties2KHR<'root>>
  for VkDisplayModeStereoPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_display_stereo")]
impl<'a> VkDisplayModeStereoPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DISPLAY_MODE_STEREO_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    hdmi3DSupported: 0,
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
  pub const fn with_hdmi3DSupported(mut self, val: VkBool32) -> Self {
    self.hdmi3DSupported = val;
    self
  }
  #[cfg(feature = "VK_KHR_get_display_properties2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDisplayModeProperties2KHR<
    'root,
    T: VkPNextExtends<VkDisplayModeProperties2KHR<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub extendedSparseAddressSpace: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
unsafe impl<'a> Send for VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
unsafe impl<'a> Sync for VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_extended_sparse_address_space",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_extended_sparse_address_space",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
impl<'a> VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    extendedSparseAddressSpace: 0,
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
  pub const fn with_extendedSparseAddressSpace(mut self, val: VkBool32) -> Self {
    self.extendedSparseAddressSpace = val;
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
/// [VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub extendedSparseAddressSpaceSize: VkDeviceSize,
  /// Limit Type: [Bitmask]
  pub extendedSparseImageUsageFlags: VkImageUsageFlags,
  /// Limit Type: [Bitmask]
  pub extendedSparseBufferUsageFlags: VkBufferUsageFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
unsafe impl<'a> Send for VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'a> {}
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
unsafe impl<'a> Sync for VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_extended_sparse_address_space",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
impl<'a> VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    extendedSparseAddressSpaceSize: 0,
    extendedSparseImageUsageFlags: VkImageUsageFlagBits(0),
    extendedSparseBufferUsageFlags: VkBufferUsageFlagBits(0),
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
  pub const fn with_extendedSparseAddressSpaceSize(mut self, val: VkDeviceSize) -> Self {
    self.extendedSparseAddressSpaceSize = val;
    self
  }
  #[inline]
  pub const fn with_extendedSparseImageUsageFlags(mut self, val: VkImageUsageFlags) -> Self {
    self.extendedSparseImageUsageFlags = val;
    self
  }
  #[inline]
  pub const fn with_extendedSparseBufferUsageFlags(mut self, val: VkBufferUsageFlags) -> Self {
    self.extendedSparseBufferUsageFlags = val;
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
/// [VkExternalComputeQueueNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueNV.html)
#[cfg(feature = "VK_NV_external_compute_queue")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkExternalComputeQueueNV(pub *mut c_void);
#[cfg(feature = "VK_NV_external_compute_queue")]
impl VkExternalComputeQueueNV {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_NV_external_compute_queue")]
impl Default for VkExternalComputeQueueNV {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_NV_external_compute_queue")]
unsafe impl Send for VkExternalComputeQueueNV {}
#[cfg(feature = "VK_NV_external_compute_queue")]
unsafe impl Sync for VkExternalComputeQueueNV {}
/// [VkExternalComputeQueueDeviceCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueDeviceCreateInfoNV.html)
///
/// **Extends:** VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_external_compute_queue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalComputeQueueDeviceCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub reservedExternalQueues: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_compute_queue")]
unsafe impl<'a> Send for VkExternalComputeQueueDeviceCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_compute_queue")]
unsafe impl<'a> Sync for VkExternalComputeQueueDeviceCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_external_compute_queue",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkExternalComputeQueueDeviceCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_external_compute_queue")]
impl<'a> VkExternalComputeQueueDeviceCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    reservedExternalQueues: 0,
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
  pub const fn with_reservedExternalQueues(mut self, val: u32) -> Self {
    self.reservedExternalQueues = val;
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
/// [VkExternalComputeQueueCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueCreateInfoNV.html)
#[cfg(feature = "VK_NV_external_compute_queue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalComputeQueueCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub preferredQueue: VkQueue,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_compute_queue")]
unsafe impl<'a> Send for VkExternalComputeQueueCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_compute_queue")]
unsafe impl<'a> Sync for VkExternalComputeQueueCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_compute_queue")]
impl<'a> VkExternalComputeQueueCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    preferredQueue: VkQueue::DEFAULT,
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
  pub const fn with_preferredQueue(mut self, val: VkQueue) -> Self {
    self.preferredQueue = val;
    self
  }
  #[cfg(feature = "VK_NV_external_compute_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkExternalComputeQueueCreateInfoNV<
    'root,
    T: VkPNextExtends<VkExternalComputeQueueCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkExternalComputeQueueDataParamsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueDataParamsNV.html)
#[cfg(feature = "VK_NV_external_compute_queue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalComputeQueueDataParamsNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub deviceIndex: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_compute_queue")]
unsafe impl<'a> Send for VkExternalComputeQueueDataParamsNV<'a> {}
#[cfg(feature = "VK_NV_external_compute_queue")]
unsafe impl<'a> Sync for VkExternalComputeQueueDataParamsNV<'a> {}
#[cfg(feature = "VK_NV_external_compute_queue")]
impl<'a> VkExternalComputeQueueDataParamsNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV,
    pNext: core::ptr::null(),
    deviceIndex: 0,
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
  pub const fn with_deviceIndex(mut self, val: u32) -> Self {
    self.deviceIndex = val;
    self
  }
  #[cfg(feature = "VK_NV_external_compute_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkExternalComputeQueueDataParamsNV<
    'root,
    T: VkPNextExtends<VkExternalComputeQueueDataParamsNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceExternalComputeQueuePropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalComputeQueuePropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_external_compute_queue")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalComputeQueuePropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Noauto]
  pub externalDataSize: u32,
  /// Limit Type: [Noauto]
  pub maxExternalQueues: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_compute_queue")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalComputeQueuePropertiesNV<'a> {}
#[cfg(feature = "VK_NV_external_compute_queue")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalComputeQueuePropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_external_compute_queue",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceExternalComputeQueuePropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_external_compute_queue")]
impl<'a> VkPhysicalDeviceExternalComputeQueuePropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    externalDataSize: 0,
    maxExternalQueues: 0,
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
  pub const fn with_externalDataSize(mut self, val: u32) -> Self {
    self.externalDataSize = val;
    self
  }
  #[inline]
  pub const fn with_maxExternalQueues(mut self, val: u32) -> Self {
    self.maxExternalQueues = val;
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
/// [VkExternalMemoryImageCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryImageCreateInfoNV.html)
///
/// **Extends:** VkImageCreateInfo.
#[cfg(feature = "VK_NV_external_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalMemoryImageCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'a> Send for VkExternalMemoryImageCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'a> Sync for VkExternalMemoryImageCreateInfoNV<'a> {}
#[cfg(all(feature = "VK_NV_external_memory", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkExternalMemoryImageCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_external_memory")]
impl<'a> VkExternalMemoryImageCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    handleTypes: VkExternalMemoryHandleTypeFlagBitsNV(0),
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
  pub const fn with_handleTypes(mut self, val: VkExternalMemoryHandleTypeFlagsNV) -> Self {
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
/// [VkExportMemoryAllocateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMemoryAllocateInfoNV.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_NV_external_memory")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExportMemoryAllocateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'a> Send for VkExportMemoryAllocateInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_memory")]
unsafe impl<'a> Sync for VkExportMemoryAllocateInfoNV<'a> {}
#[cfg(all(feature = "VK_NV_external_memory", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkExportMemoryAllocateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_external_memory")]
impl<'a> VkExportMemoryAllocateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXPORT_MEMORY_ALLOCATE_INFO_NV,
    pNext: core::ptr::null(),
    handleTypes: VkExternalMemoryHandleTypeFlagBitsNV(0),
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
  pub const fn with_handleTypes(mut self, val: VkExternalMemoryHandleTypeFlagsNV) -> Self {
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
/// [VkExternalMemoryHandleTypeFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagsNV.html)
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub type VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagBitsNV;
/// [VkExternalMemoryFeatureFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagsNV.html)
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub type VkExternalMemoryFeatureFlagsNV = VkExternalMemoryFeatureFlagBitsNV;
/// [VkExternalImageFormatPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalImageFormatPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_NV_external_memory_capabilities")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalImageFormatPropertiesNV {
  pub imageFormatProperties: VkImageFormatProperties,
  /// Optional: true
  pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV,
  /// Optional: true
  pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
  /// Optional: true
  pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
}
#[cfg(feature = "VK_NV_external_memory_capabilities")]
unsafe impl Send for VkExternalImageFormatPropertiesNV {}
#[cfg(feature = "VK_NV_external_memory_capabilities")]
unsafe impl Sync for VkExternalImageFormatPropertiesNV {}
#[cfg(feature = "VK_NV_external_memory_capabilities")]
impl VkExternalImageFormatPropertiesNV {
  pub const DEFAULT: Self = Self {
    imageFormatProperties: VkImageFormatProperties::DEFAULT,
    externalMemoryFeatures: VkExternalMemoryFeatureFlagBitsNV(0),
    exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagBitsNV(0),
    compatibleHandleTypes: VkExternalMemoryHandleTypeFlagBitsNV(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_imageFormatProperties(mut self, val: VkImageFormatProperties) -> Self {
    self.imageFormatProperties = val;
    self
  }
  #[inline]
  pub const fn with_externalMemoryFeatures(mut self, val: VkExternalMemoryFeatureFlagsNV) -> Self {
    self.externalMemoryFeatures = val;
    self
  }
  #[inline]
  pub const fn with_exportFromImportedHandleTypes(
    mut self,
    val: VkExternalMemoryHandleTypeFlagsNV,
  ) -> Self {
    self.exportFromImportedHandleTypes = val;
    self
  }
  #[inline]
  pub const fn with_compatibleHandleTypes(
    mut self,
    val: VkExternalMemoryHandleTypeFlagsNV,
  ) -> Self {
    self.compatibleHandleTypes = val;
    self
  }
}
/// [VkRemoteAddressNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkRemoteAddressNV.html)
#[cfg(feature = "VK_NV_external_memory_rdma")]
pub type VkRemoteAddressNV = c_void;
/// [VkPhysicalDeviceExternalMemoryRDMAFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalMemoryRDMAFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_external_memory_rdma")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub externalMemoryRDMA: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory_rdma")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_rdma")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_external_memory_rdma",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_external_memory_rdma",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_external_memory_rdma")]
impl<'a> VkPhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    externalMemoryRDMA: 0,
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
  pub const fn with_externalMemoryRDMA(mut self, val: VkBool32) -> Self {
    self.externalMemoryRDMA = val;
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
/// [VkMemoryGetRemoteAddressInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetRemoteAddressInfoNV.html)
#[cfg(feature = "VK_NV_external_memory_rdma")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryGetRemoteAddressInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory_rdma")]
unsafe impl<'a> Send for VkMemoryGetRemoteAddressInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_rdma")]
unsafe impl<'a> Sync for VkMemoryGetRemoteAddressInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_rdma")]
impl<'a> VkMemoryGetRemoteAddressInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_GET_REMOTE_ADDRESS_INFO_NV,
    pNext: core::ptr::null(),
    memory: VkDeviceMemory::DEFAULT,
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
  pub const fn with_memory(mut self, val: VkDeviceMemory) -> Self {
    self.memory = val;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalMemoryHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  #[cfg(feature = "VK_NV_external_memory_rdma")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryGetRemoteAddressInfoNV<
    'root,
    T: VkPNextExtends<VkMemoryGetRemoteAddressInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [NvSciBufAttrList](https://docs.vulkan.org/refpages/latest/refpages/source/NvSciBufAttrList.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvSciBufAttrList(pub *mut c_void);
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
impl NvSciBufAttrList {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl Send for NvSciBufAttrList {}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl Sync for NvSciBufAttrList {}
/// [NvSciBufObj](https://docs.vulkan.org/refpages/latest/refpages/source/NvSciBufObj.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvSciBufObj(pub *mut c_void);
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
impl NvSciBufObj {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl Send for NvSciBufObj {}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl Sync for NvSciBufObj {}
/// [VkExportMemorySciBufInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMemorySciBufInfoNV.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExportMemorySciBufInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXPORT_MEMORY_SCI_BUF_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pAttributes: NvSciBufAttrList,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Send for VkExportMemorySciBufInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Sync for VkExportMemorySciBufInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_external_memory_sci_buf",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkExportMemorySciBufInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
impl<'a> VkExportMemorySciBufInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXPORT_MEMORY_SCI_BUF_INFO_NV,
    pNext: core::ptr::null(),
    pAttributes: NvSciBufAttrList::NULL,
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
  pub const fn with_pAttributes(mut self, val: NvSciBufAttrList) -> Self {
    self.pAttributes = val;
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
/// [VkImportMemorySciBufInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMemorySciBufInfoNV.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImportMemorySciBufInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMPORT_MEMORY_SCI_BUF_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
  pub handle: NvSciBufObj,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Send for VkImportMemorySciBufInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Sync for VkImportMemorySciBufInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_external_memory_sci_buf",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkImportMemorySciBufInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
impl<'a> VkImportMemorySciBufInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMPORT_MEMORY_SCI_BUF_INFO_NV,
    pNext: core::ptr::null(),
    handleType: VkExternalMemoryHandleTypeFlagBits(0),
    handle: NvSciBufObj::NULL,
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
  #[inline]
  pub const fn with_handle(mut self, val: NvSciBufObj) -> Self {
    self.handle = val;
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
/// [VkMemoryGetSciBufInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryGetSciBufInfoNV.html)
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryGetSciBufInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_GET_SCI_BUF_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Send for VkMemoryGetSciBufInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Sync for VkMemoryGetSciBufInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
impl<'a> VkMemoryGetSciBufInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_GET_SCI_BUF_INFO_NV,
    pNext: core::ptr::null(),
    memory: VkDeviceMemory::DEFAULT,
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
  pub const fn with_memory(mut self, val: VkDeviceMemory) -> Self {
    self.memory = val;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalMemoryHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryGetSciBufInfoNV<
    'root,
    T: VkPNextExtends<VkMemoryGetSciBufInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkMemorySciBufPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemorySciBufPropertiesNV.html)
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemorySciBufPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_SCI_BUF_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub memoryTypeBits: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Send for VkMemorySciBufPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Sync for VkMemorySciBufPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
impl<'a> VkMemorySciBufPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_SCI_BUF_PROPERTIES_NV,
    pNext: core::ptr::null(),
    memoryTypeBits: 0,
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
  pub const fn with_memoryTypeBits(mut self, val: u32) -> Self {
    self.memoryTypeBits = val;
    self
  }
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemorySciBufPropertiesNV<
    'root,
    T: VkPNextExtends<VkMemorySciBufPropertiesNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceExternalMemorySciBufFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalMemorySciBufFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub sciBufImport: VkBool32,
  pub sciBufExport: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_external_memory_sci_buf",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_external_memory_sci_buf",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
impl<'a> VkPhysicalDeviceExternalMemorySciBufFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    sciBufImport: 0,
    sciBufExport: 0,
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
  pub const fn with_sciBufImport(mut self, val: VkBool32) -> Self {
    self.sciBufImport = val;
    self
  }
  #[inline]
  pub const fn with_sciBufExport(mut self, val: VkBool32) -> Self {
    self.sciBufExport = val;
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
/// [VkPhysicalDeviceExternalSciBufFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalSciBufFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalSciBufFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub sciBufImport: VkBool32,
  pub sciBufExport: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalSciBufFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalSciBufFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_external_memory_sci_buf",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceExternalSciBufFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_external_memory_sci_buf",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceExternalSciBufFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
impl<'a> VkPhysicalDeviceExternalSciBufFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    sciBufImport: 0,
    sciBufExport: 0,
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
  pub const fn with_sciBufImport(mut self, val: VkBool32) -> Self {
    self.sciBufImport = val;
    self
  }
  #[inline]
  pub const fn with_sciBufExport(mut self, val: VkBool32) -> Self {
    self.sciBufExport = val;
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
/// [VkImportMemoryWin32HandleInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkImportMemoryWin32HandleInfoNV.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_NV_external_memory_win32")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImportMemoryWin32HandleInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub handleType: VkExternalMemoryHandleTypeFlagsNV,
  /// Optional: true
  pub handle: HANDLE,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory_win32")]
unsafe impl<'a> Send for VkImportMemoryWin32HandleInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_win32")]
unsafe impl<'a> Sync for VkImportMemoryWin32HandleInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_external_memory_win32",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkImportMemoryWin32HandleInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_external_memory_win32")]
impl<'a> VkImportMemoryWin32HandleInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
    pNext: core::ptr::null(),
    handleType: VkExternalMemoryHandleTypeFlagBitsNV(0),
    handle: HANDLE::NULL,
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
  pub const fn with_handleType(mut self, val: VkExternalMemoryHandleTypeFlagsNV) -> Self {
    self.handleType = val;
    self
  }
  #[inline]
  pub const fn with_handle(mut self, val: HANDLE) -> Self {
    self.handle = val;
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
/// [VkExportMemoryWin32HandleInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMemoryWin32HandleInfoNV.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_NV_external_memory_win32")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExportMemoryWin32HandleInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub pAttributes: *const SECURITY_ATTRIBUTES,
  /// Optional: true
  pub dwAccess: DWORD,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_memory_win32")]
unsafe impl<'a> Send for VkExportMemoryWin32HandleInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_memory_win32")]
unsafe impl<'a> Sync for VkExportMemoryWin32HandleInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_external_memory_win32",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkExportMemoryWin32HandleInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_external_memory_win32")]
impl<'a> VkExportMemoryWin32HandleInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
    pNext: core::ptr::null(),
    pAttributes: core::ptr::null(),
    dwAccess: DWORD::NULL,
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
  pub const fn with_pAttributes(mut self, val: *const SECURITY_ATTRIBUTES) -> Self {
    self.pAttributes = val;
    self
  }
  #[inline]
  pub const fn with_dwAccess(mut self, val: DWORD) -> Self {
    self.dwAccess = val;
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
/// [VkExportSemaphoreSciSyncInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExportSemaphoreSciSyncInfoNV.html)
///
/// **Extends:** VkSemaphoreCreateInfo.
#[cfg(feature = "VK_NV_external_sci_sync")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExportSemaphoreSciSyncInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_SCI_SYNC_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pAttributes: NvSciSyncAttrList,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_sci_sync")]
unsafe impl<'a> Send for VkExportSemaphoreSciSyncInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_sci_sync")]
unsafe impl<'a> Sync for VkExportSemaphoreSciSyncInfoNV<'a> {}
#[cfg(all(feature = "VK_NV_external_sci_sync", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSemaphoreCreateInfo<'root>>
  for VkExportSemaphoreSciSyncInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_external_sci_sync")]
impl<'a> VkExportSemaphoreSciSyncInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXPORT_SEMAPHORE_SCI_SYNC_INFO_NV,
    pNext: core::ptr::null(),
    pAttributes: NvSciSyncAttrList::NULL,
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
  pub const fn with_pAttributes(mut self, val: NvSciSyncAttrList) -> Self {
    self.pAttributes = val;
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
/// [VkImportSemaphoreSciSyncInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkImportSemaphoreSciSyncInfoNV.html)
#[cfg(feature = "VK_NV_external_sci_sync")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImportSemaphoreSciSyncInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
  pub handle: *mut c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_sci_sync")]
unsafe impl<'a> Send for VkImportSemaphoreSciSyncInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_sci_sync")]
unsafe impl<'a> Sync for VkImportSemaphoreSciSyncInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_sci_sync")]
impl<'a> VkImportSemaphoreSciSyncInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV,
    pNext: core::ptr::null(),
    semaphore: VkSemaphore::DEFAULT,
    handleType: VkExternalSemaphoreHandleTypeFlagBits(0),
    handle: core::ptr::null_mut(),
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
  pub const fn with_semaphore(mut self, val: VkSemaphore) -> Self {
    self.semaphore = val;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalSemaphoreHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_handle(mut self, val: *mut c_void) -> Self {
    self.handle = val;
    self
  }
  #[cfg(feature = "VK_NV_external_sci_sync")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImportSemaphoreSciSyncInfoNV<
    'root,
    T: VkPNextExtends<VkImportSemaphoreSciSyncInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSemaphoreGetSciSyncInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreGetSciSyncInfoNV.html)
#[cfg(feature = "VK_NV_external_sci_sync")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSemaphoreGetSciSyncInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_SEMAPHORE_GET_SCI_SYNC_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_sci_sync")]
unsafe impl<'a> Send for VkSemaphoreGetSciSyncInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_sci_sync")]
unsafe impl<'a> Sync for VkSemaphoreGetSciSyncInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_sci_sync")]
impl<'a> VkSemaphoreGetSciSyncInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SEMAPHORE_GET_SCI_SYNC_INFO_NV,
    pNext: core::ptr::null(),
    semaphore: VkSemaphore::DEFAULT,
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
  pub const fn with_semaphore(mut self, val: VkSemaphore) -> Self {
    self.semaphore = val;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalSemaphoreHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  #[cfg(feature = "VK_NV_external_sci_sync")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSemaphoreGetSciSyncInfoNV<
    'root,
    T: VkPNextExtends<VkSemaphoreGetSciSyncInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceExternalSciSyncFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalSciSyncFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_external_sci_sync")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalSciSyncFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub sciSyncFence: VkBool32,
  pub sciSyncSemaphore: VkBool32,
  pub sciSyncImport: VkBool32,
  pub sciSyncExport: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_sci_sync")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalSciSyncFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_external_sci_sync")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalSciSyncFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_external_sci_sync", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceExternalSciSyncFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_external_sci_sync", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceExternalSciSyncFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_external_sci_sync")]
impl<'a> VkPhysicalDeviceExternalSciSyncFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    sciSyncFence: 0,
    sciSyncSemaphore: 0,
    sciSyncImport: 0,
    sciSyncExport: 0,
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
  pub const fn with_sciSyncFence(mut self, val: VkBool32) -> Self {
    self.sciSyncFence = val;
    self
  }
  #[inline]
  pub const fn with_sciSyncSemaphore(mut self, val: VkBool32) -> Self {
    self.sciSyncSemaphore = val;
    self
  }
  #[inline]
  pub const fn with_sciSyncImport(mut self, val: VkBool32) -> Self {
    self.sciSyncImport = val;
    self
  }
  #[inline]
  pub const fn with_sciSyncExport(mut self, val: VkBool32) -> Self {
    self.sciSyncExport = val;
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
/// [NvSciSyncAttrList](https://docs.vulkan.org/refpages/latest/refpages/source/NvSciSyncAttrList.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvSciSyncAttrList(pub *mut c_void);
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
impl NvSciSyncAttrList {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
unsafe impl Send for NvSciSyncAttrList {}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
unsafe impl Sync for NvSciSyncAttrList {}
/// [VkExportFenceSciSyncInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkExportFenceSciSyncInfoNV.html)
///
/// **Extends:** VkFenceCreateInfo.
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExportFenceSciSyncInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXPORT_FENCE_SCI_SYNC_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pAttributes: NvSciSyncAttrList,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
unsafe impl<'a> Send for VkExportFenceSciSyncInfoNV<'a> {}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
unsafe impl<'a> Sync for VkExportFenceSciSyncInfoNV<'a> {}
#[cfg(all(
  any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ),
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkFenceCreateInfo<'root>>
  for VkExportFenceSciSyncInfoNV<'child>
{
}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
impl<'a> VkExportFenceSciSyncInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EXPORT_FENCE_SCI_SYNC_INFO_NV,
    pNext: core::ptr::null(),
    pAttributes: NvSciSyncAttrList::NULL,
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
  pub const fn with_pAttributes(mut self, val: NvSciSyncAttrList) -> Self {
    self.pAttributes = val;
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
/// [VkImportFenceSciSyncInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkImportFenceSciSyncInfoNV.html)
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImportFenceSciSyncInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMPORT_FENCE_SCI_SYNC_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBits,
  pub handle: *mut c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
unsafe impl<'a> Send for VkImportFenceSciSyncInfoNV<'a> {}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
unsafe impl<'a> Sync for VkImportFenceSciSyncInfoNV<'a> {}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
impl<'a> VkImportFenceSciSyncInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMPORT_FENCE_SCI_SYNC_INFO_NV,
    pNext: core::ptr::null(),
    fence: VkFence::DEFAULT,
    handleType: VkExternalFenceHandleTypeFlagBits(0),
    handle: core::ptr::null_mut(),
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
  pub const fn with_fence(mut self, val: VkFence) -> Self {
    self.fence = val;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalFenceHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_handle(mut self, val: *mut c_void) -> Self {
    self.handle = val;
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
  pub const fn with_pNext_chain_VkImportFenceSciSyncInfoNV<
    'root,
    T: VkPNextExtends<VkImportFenceSciSyncInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkFenceGetSciSyncInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceGetSciSyncInfoNV.html)
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFenceGetSciSyncInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_FENCE_GET_SCI_SYNC_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub fence: VkFence,
  pub handleType: VkExternalFenceHandleTypeFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
unsafe impl<'a> Send for VkFenceGetSciSyncInfoNV<'a> {}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
unsafe impl<'a> Sync for VkFenceGetSciSyncInfoNV<'a> {}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
impl<'a> VkFenceGetSciSyncInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::FENCE_GET_SCI_SYNC_INFO_NV,
    pNext: core::ptr::null(),
    fence: VkFence::DEFAULT,
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
  pub const fn with_fence(mut self, val: VkFence) -> Self {
    self.fence = val;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalFenceHandleTypeFlagBits) -> Self {
    self.handleType = val;
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
  pub const fn with_pNext_chain_VkFenceGetSciSyncInfoNV<
    'root,
    T: VkPNextExtends<VkFenceGetSciSyncInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSciSyncAttributesInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSciSyncAttributesInfoNV.html)
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSciSyncAttributesInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_SCI_SYNC_ATTRIBUTES_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub clientType: VkSciSyncClientTypeNV,
  pub primitiveType: VkSciSyncPrimitiveTypeNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
unsafe impl<'a> Send for VkSciSyncAttributesInfoNV<'a> {}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
unsafe impl<'a> Sync for VkSciSyncAttributesInfoNV<'a> {}
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
impl<'a> VkSciSyncAttributesInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SCI_SYNC_ATTRIBUTES_INFO_NV,
    pNext: core::ptr::null(),
    clientType: VkSciSyncClientTypeNV(0),
    primitiveType: VkSciSyncPrimitiveTypeNV(0),
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
  pub const fn with_clientType(mut self, val: VkSciSyncClientTypeNV) -> Self {
    self.clientType = val;
    self
  }
  #[inline]
  pub const fn with_primitiveType(mut self, val: VkSciSyncPrimitiveTypeNV) -> Self {
    self.primitiveType = val;
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
  pub const fn with_pNext_chain_VkSciSyncAttributesInfoNV<
    'root,
    T: VkPNextExtends<VkSciSyncAttributesInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [NvSciSyncObj](https://docs.vulkan.org/refpages/latest/refpages/source/NvSciSyncObj.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_NV_external_sci_sync2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvSciSyncObj(pub *mut c_void);
#[cfg(feature = "VK_NV_external_sci_sync2")]
impl NvSciSyncObj {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl Send for NvSciSyncObj {}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl Sync for NvSciSyncObj {}
/// [NvSciSyncFence](https://docs.vulkan.org/refpages/latest/refpages/source/NvSciSyncFence.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_NV_external_sci_sync2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvSciSyncFence(pub *mut c_void);
#[cfg(feature = "VK_NV_external_sci_sync2")]
impl NvSciSyncFence {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl Send for NvSciSyncFence {}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl Sync for NvSciSyncFence {}
/// [VkSemaphoreSciSyncPoolNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreSciSyncPoolNV.html)
#[cfg(feature = "VK_NV_external_sci_sync2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkSemaphoreSciSyncPoolNV(pub *mut c_void);
#[cfg(feature = "VK_NV_external_sci_sync2")]
impl VkSemaphoreSciSyncPoolNV {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
impl Default for VkSemaphoreSciSyncPoolNV {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl Send for VkSemaphoreSciSyncPoolNV {}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl Sync for VkSemaphoreSciSyncPoolNV {}
/// [VkPhysicalDeviceExternalSciSync2FeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalSciSync2FeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_external_sci_sync2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalSciSync2FeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub sciSyncFence: VkBool32,
  pub sciSyncSemaphore2: VkBool32,
  pub sciSyncImport: VkBool32,
  pub sciSyncExport: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalSciSync2FeaturesNV<'a> {}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalSciSync2FeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_external_sci_sync2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceExternalSciSync2FeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_external_sci_sync2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceExternalSciSync2FeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
impl<'a> VkPhysicalDeviceExternalSciSync2FeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    sciSyncFence: 0,
    sciSyncSemaphore2: 0,
    sciSyncImport: 0,
    sciSyncExport: 0,
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
  pub const fn with_sciSyncFence(mut self, val: VkBool32) -> Self {
    self.sciSyncFence = val;
    self
  }
  #[inline]
  pub const fn with_sciSyncSemaphore2(mut self, val: VkBool32) -> Self {
    self.sciSyncSemaphore2 = val;
    self
  }
  #[inline]
  pub const fn with_sciSyncImport(mut self, val: VkBool32) -> Self {
    self.sciSyncImport = val;
    self
  }
  #[inline]
  pub const fn with_sciSyncExport(mut self, val: VkBool32) -> Self {
    self.sciSyncExport = val;
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
/// [VkSemaphoreSciSyncPoolCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreSciSyncPoolCreateInfoNV.html)
#[cfg(feature = "VK_NV_external_sci_sync2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSemaphoreSciSyncPoolCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_SEMAPHORE_SCI_SYNC_POOL_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub handle: NvSciSyncObj,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl<'a> Send for VkSemaphoreSciSyncPoolCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl<'a> Sync for VkSemaphoreSciSyncPoolCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_sci_sync2")]
impl<'a> VkSemaphoreSciSyncPoolCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SEMAPHORE_SCI_SYNC_POOL_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    handle: NvSciSyncObj::NULL,
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
  pub const fn with_handle(mut self, val: NvSciSyncObj) -> Self {
    self.handle = val;
    self
  }
  #[cfg(feature = "VK_NV_external_sci_sync2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSemaphoreSciSyncPoolCreateInfoNV<
    'root,
    T: VkPNextExtends<VkSemaphoreSciSyncPoolCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSemaphoreSciSyncCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreSciSyncCreateInfoNV.html)
///
/// **Extends:** VkSemaphoreCreateInfo.
#[cfg(feature = "VK_NV_external_sci_sync2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSemaphoreSciSyncCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_SEMAPHORE_SCI_SYNC_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub semaphorePool: VkSemaphoreSciSyncPoolNV,
  pub pFence: *const NvSciSyncFence,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl<'a> Send for VkSemaphoreSciSyncCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_external_sci_sync2")]
unsafe impl<'a> Sync for VkSemaphoreSciSyncCreateInfoNV<'a> {}
#[cfg(all(feature = "VK_NV_external_sci_sync2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSemaphoreCreateInfo<'root>>
  for VkSemaphoreSciSyncCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
impl<'a> VkSemaphoreSciSyncCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SEMAPHORE_SCI_SYNC_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    semaphorePool: VkSemaphoreSciSyncPoolNV::DEFAULT,
    pFence: core::ptr::null(),
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
  pub const fn with_semaphorePool(mut self, val: VkSemaphoreSciSyncPoolNV) -> Self {
    self.semaphorePool = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pFence(mut self, val: *const NvSciSyncFence) -> Self {
    self.pFence = val;
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
/// [VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV.html)
///
/// **Extends:** VkDeviceCreateInfo.
///
/// **Availability:** depends on `VKSC_VERSION_1_0`.
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_NV_external_sci_sync2"))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_SEMAPHORE_SCI_SYNC_POOL_RESERVATION_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub semaphoreSciSyncPoolRequestCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_NV_external_sci_sync2"))]
unsafe impl<'a> Send for VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV<'a> {}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_NV_external_sci_sync2"))]
unsafe impl<'a> Sync for VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV<'a> {}
#[cfg(all(
  all(feature = "VKSC_VERSION_1_0", feature = "VK_NV_external_sci_sync2"),
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV<'child>
{
}
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_NV_external_sci_sync2"))]
impl<'a> VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_SEMAPHORE_SCI_SYNC_POOL_RESERVATION_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    semaphoreSciSyncPoolRequestCount: 0,
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
  pub const fn with_semaphoreSciSyncPoolRequestCount(mut self, val: u32) -> Self {
    self.semaphoreSciSyncPoolRequestCount = val;
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
/// [VkPipelineCoverageToColorStateCreateFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageToColorStateCreateFlagsNV.html)
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub type VkPipelineCoverageToColorStateCreateFlagsNV = VkFlags;
/// [VkPipelineCoverageToColorStateCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageToColorStateCreateInfoNV.html)
///
/// **Extends:** VkPipelineMultisampleStateCreateInfo.
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCoverageToColorStateCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
  pub coverageToColorEnable: VkBool32,
  /// Optional: true
  pub coverageToColorLocation: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
unsafe impl<'a> Send for VkPipelineCoverageToColorStateCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
unsafe impl<'a> Sync for VkPipelineCoverageToColorStateCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_fragment_coverage_to_color",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineMultisampleStateCreateInfo<'root>>
  for VkPipelineCoverageToColorStateCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
impl<'a> VkPipelineCoverageToColorStateCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    flags: 0,
    coverageToColorEnable: 0,
    coverageToColorLocation: 0,
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
  pub const fn with_flags(mut self, val: VkPipelineCoverageToColorStateCreateFlagsNV) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_coverageToColorEnable(mut self, val: VkBool32) -> Self {
    self.coverageToColorEnable = val;
    self
  }
  #[inline]
  pub const fn with_coverageToColorLocation(mut self, val: u32) -> Self {
    self.coverageToColorLocation = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineMultisampleStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineMultisampleStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_fragment_shader_barycentric")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub fragmentShaderBarycentric: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_fragment_shader_barycentric")]
unsafe impl<'a> Send for VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_fragment_shader_barycentric")]
unsafe impl<'a> Sync for VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_fragment_shader_barycentric",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_fragment_shader_barycentric",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_fragment_shader_barycentric")]
impl<'a> VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    fragmentShaderBarycentric: 0,
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
  pub const fn with_fragmentShaderBarycentric(mut self, val: VkBool32) -> Self {
    self.fragmentShaderBarycentric = val;
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
/// [VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub fragmentShadingRateEnums: VkBool32,
  pub supersampleFragmentShadingRates: VkBool32,
  pub noInvocationFragmentShadingRates: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
unsafe impl<'a> Send for VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
unsafe impl<'a> Sync for VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_fragment_shading_rate_enums",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_fragment_shading_rate_enums",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
impl<'a> VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    fragmentShadingRateEnums: 0,
    supersampleFragmentShadingRates: 0,
    noInvocationFragmentShadingRates: 0,
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
  pub const fn with_fragmentShadingRateEnums(mut self, val: VkBool32) -> Self {
    self.fragmentShadingRateEnums = val;
    self
  }
  #[inline]
  pub const fn with_supersampleFragmentShadingRates(mut self, val: VkBool32) -> Self {
    self.supersampleFragmentShadingRates = val;
    self
  }
  #[inline]
  pub const fn with_noInvocationFragmentShadingRates(mut self, val: VkBool32) -> Self {
    self.noInvocationFragmentShadingRates = val;
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
/// [VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxFragmentShadingRateInvocationCount: VkSampleCountFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
unsafe impl<'a> Send for VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
unsafe impl<'a> Sync for VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_fragment_shading_rate_enums",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
impl<'a> VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    maxFragmentShadingRateInvocationCount: VkSampleCountFlagBits(0),
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
  pub const fn with_maxFragmentShadingRateInvocationCount(
    mut self,
    val: VkSampleCountFlagBits,
  ) -> Self {
    self.maxFragmentShadingRateInvocationCount = val;
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
/// [VkPipelineFragmentShadingRateEnumStateCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.html)
///
/// **Extends:** VkGraphicsPipelineCreateInfo.
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// No Auto-Validity
  pub shadingRateType: VkFragmentShadingRateTypeNV,
  /// No Auto-Validity
  pub shadingRate: VkFragmentShadingRateNV,
  /// No Auto-Validity
  pub combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2],
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
unsafe impl<'a> Send for VkPipelineFragmentShadingRateEnumStateCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
unsafe impl<'a> Sync for VkPipelineFragmentShadingRateEnumStateCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_fragment_shading_rate_enums",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkPipelineFragmentShadingRateEnumStateCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
impl<'a> VkPipelineFragmentShadingRateEnumStateCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    shadingRateType: VkFragmentShadingRateTypeNV(0),
    shadingRate: VkFragmentShadingRateNV(0),
    combinerOps: [VkFragmentShadingRateCombinerOpKHR(0); 2],
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
  pub const fn with_shadingRateType(mut self, val: VkFragmentShadingRateTypeNV) -> Self {
    self.shadingRateType = val;
    self
  }
  #[inline]
  pub const fn with_shadingRate(mut self, val: VkFragmentShadingRateNV) -> Self {
    self.shadingRate = val;
    self
  }
  #[inline]
  pub const fn with_combinerOps(mut self, val: [VkFragmentShadingRateCombinerOpKHR; 2]) -> Self {
    self.combinerOps = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGraphicsPipelineCreateInfo<
    'root,
    T: VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPipelineCoverageModulationStateCreateFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageModulationStateCreateFlagsNV.html)
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub type VkPipelineCoverageModulationStateCreateFlagsNV = VkFlags;
/// [VkPipelineCoverageModulationStateCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCoverageModulationStateCreateInfoNV.html)
///
/// **Extends:** VkPipelineMultisampleStateCreateInfo.
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCoverageModulationStateCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
  pub coverageModulationMode: VkCoverageModulationModeNV,
  pub coverageModulationTableEnable: VkBool32,
  /// Optional: true
  pub coverageModulationTableCount: u32,
  /// Optional: true,  Length: coverageModulationTableCount,  No Auto-Validity
  pub pCoverageModulationTable: *const f32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
unsafe impl<'a> Send for VkPipelineCoverageModulationStateCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
unsafe impl<'a> Sync for VkPipelineCoverageModulationStateCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_framebuffer_mixed_samples",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineMultisampleStateCreateInfo<'root>>
  for VkPipelineCoverageModulationStateCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
impl<'a> VkPipelineCoverageModulationStateCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    flags: 0,
    coverageModulationMode: VkCoverageModulationModeNV(0),
    coverageModulationTableEnable: 0,
    coverageModulationTableCount: 0,
    pCoverageModulationTable: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkPipelineCoverageModulationStateCreateFlagsNV) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_coverageModulationMode(mut self, val: VkCoverageModulationModeNV) -> Self {
    self.coverageModulationMode = val;
    self
  }
  #[inline]
  pub const fn with_coverageModulationTableEnable(mut self, val: VkBool32) -> Self {
    self.coverageModulationTableEnable = val;
    self
  }
  #[inline]
  pub const fn with_coverageModulationTableCount(mut self, val: u32) -> Self {
    self.coverageModulationTableCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCoverageModulationTable(mut self, val: &'a [f32]) -> Self {
    self.coverageModulationTableCount = val.len() as u32;
    self.pCoverageModulationTable = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineMultisampleStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineMultisampleStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkAttachmentSampleCountInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentSampleCountInfoNV.html)
///
/// **Extends:** VkCommandBufferInheritanceInfo, VkGraphicsPipelineCreateInfo.
///
/// **Availability:** depends on `VK_VERSION_1_3 + VK_KHR_dynamic_rendering`.
#[cfg(any(
  all(
    feature = "VK_NV_framebuffer_mixed_samples",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NV_framebuffer_mixed_samples"
  )
))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAttachmentSampleCountInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub colorAttachmentCount: u32,
  /// Length: colorAttachmentCount,  No Auto-Validity
  pub pColorAttachmentSamples: *const VkSampleCountFlagBits,
  /// Optional: true,  No Auto-Validity
  pub depthStencilAttachmentSamples: VkSampleCountFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(any(
  all(
    feature = "VK_NV_framebuffer_mixed_samples",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NV_framebuffer_mixed_samples"
  )
))]
unsafe impl<'a> Send for VkAttachmentSampleCountInfoNV<'a> {}
#[cfg(any(
  all(
    feature = "VK_NV_framebuffer_mixed_samples",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NV_framebuffer_mixed_samples"
  )
))]
unsafe impl<'a> Sync for VkAttachmentSampleCountInfoNV<'a> {}
#[cfg(all(
  any(
    all(
      feature = "VK_NV_framebuffer_mixed_samples",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_KHR_dynamic_rendering",
      feature = "VK_NV_framebuffer_mixed_samples"
    )
  ),
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkAttachmentSampleCountInfoNV<'child>
{
}
#[cfg(all(
  any(
    all(
      feature = "VK_NV_framebuffer_mixed_samples",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_KHR_dynamic_rendering",
      feature = "VK_NV_framebuffer_mixed_samples"
    )
  ),
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkAttachmentSampleCountInfoNV<'child>
{
}
#[cfg(any(
  all(
    feature = "VK_NV_framebuffer_mixed_samples",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NV_framebuffer_mixed_samples"
  )
))]
impl<'a> VkAttachmentSampleCountInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ATTACHMENT_SAMPLE_COUNT_INFO_NV,
    pNext: core::ptr::null(),
    colorAttachmentCount: 0,
    pColorAttachmentSamples: core::ptr::null(),
    depthStencilAttachmentSamples: VkSampleCountFlagBits(0),
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
  pub const fn with_colorAttachmentCount(mut self, val: u32) -> Self {
    self.colorAttachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pColorAttachmentSamples(mut self, val: &'a [VkSampleCountFlagBits]) -> Self {
    self.colorAttachmentCount = val.len() as u32;
    self.pColorAttachmentSamples = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_depthStencilAttachmentSamples(mut self, val: VkSampleCountFlagBits) -> Self {
    self.depthStencilAttachmentSamples = val;
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
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGraphicsPipelineCreateInfo<
    'root,
    T: VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceInheritedViewportScissorFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceInheritedViewportScissorFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub inheritedViewportScissor2D: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
unsafe impl<'a> Send for VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
unsafe impl<'a> Sync for VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_inherited_viewport_scissor",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_inherited_viewport_scissor",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
impl<'a> VkPhysicalDeviceInheritedViewportScissorFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    inheritedViewportScissor2D: 0,
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
  pub const fn with_inheritedViewportScissor2D(mut self, val: VkBool32) -> Self {
    self.inheritedViewportScissor2D = val;
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
/// [VkCommandBufferInheritanceViewportScissorInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferInheritanceViewportScissorInfoNV.html)
///
/// **Extends:** VkCommandBufferInheritanceInfo.
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandBufferInheritanceViewportScissorInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub viewportScissor2D: VkBool32,
  pub viewportDepthCount: u32,
  /// No Auto-Validity
  pub pViewportDepths: *const VkViewport,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
unsafe impl<'a> Send for VkCommandBufferInheritanceViewportScissorInfoNV<'a> {}
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
unsafe impl<'a> Sync for VkCommandBufferInheritanceViewportScissorInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_inherited_viewport_scissor",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkCommandBufferInheritanceViewportScissorInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
impl<'a> VkCommandBufferInheritanceViewportScissorInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV,
    pNext: core::ptr::null(),
    viewportScissor2D: 0,
    viewportDepthCount: 0,
    pViewportDepths: core::ptr::null(),
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
  pub const fn with_viewportScissor2D(mut self, val: VkBool32) -> Self {
    self.viewportScissor2D = val;
    self
  }
  #[inline]
  pub const fn with_viewportDepthCount(mut self, val: u32) -> Self {
    self.viewportDepthCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pViewportDepths(mut self, val: &'a VkViewport) -> Self {
    self.pViewportDepths = val as *const VkViewport;
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
/// [VkPhysicalDeviceLinearColorAttachmentFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLinearColorAttachmentFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_linear_color_attachment")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub linearColorAttachment: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_linear_color_attachment")]
unsafe impl<'a> Send for VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_linear_color_attachment")]
unsafe impl<'a> Sync for VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_linear_color_attachment",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_linear_color_attachment",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_linear_color_attachment")]
impl<'a> VkPhysicalDeviceLinearColorAttachmentFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    linearColorAttachment: 0,
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
  pub const fn with_linearColorAttachment(mut self, val: VkBool32) -> Self {
    self.linearColorAttachment = val;
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
/// [VkQueryLowLatencySupportNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryLowLatencySupportNV.html)
///
/// **Extends:** VkSemaphoreCreateInfo.
#[cfg(feature = "VK_NV_low_latency")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueryLowLatencySupportNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUERY_LOW_LATENCY_SUPPORT_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  pub pQueriedLowLatencyData: *mut c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_low_latency")]
unsafe impl<'a> Send for VkQueryLowLatencySupportNV<'a> {}
#[cfg(feature = "VK_NV_low_latency")]
unsafe impl<'a> Sync for VkQueryLowLatencySupportNV<'a> {}
#[cfg(all(feature = "VK_NV_low_latency", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSemaphoreCreateInfo<'root>>
  for VkQueryLowLatencySupportNV<'child>
{
}
#[cfg(feature = "VK_NV_low_latency")]
impl<'a> VkQueryLowLatencySupportNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::QUERY_LOW_LATENCY_SUPPORT_NV,
    pNext: core::ptr::null(),
    pQueriedLowLatencyData: core::ptr::null_mut(),
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
  pub const fn with_pQueriedLowLatencyData(mut self, val: *mut c_void) -> Self {
    self.pQueriedLowLatencyData = val;
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
/// [VkLatencySleepModeInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencySleepModeInfoNV.html)
#[cfg(feature = "VK_NV_low_latency2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkLatencySleepModeInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_LATENCY_SLEEP_MODE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  pub lowLatencyMode: VkBool32,
  pub lowLatencyBoost: VkBool32,
  pub minimumIntervalUs: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Send for VkLatencySleepModeInfoNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Sync for VkLatencySleepModeInfoNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
impl<'a> VkLatencySleepModeInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::LATENCY_SLEEP_MODE_INFO_NV,
    pNext: core::ptr::null(),
    lowLatencyMode: 0,
    lowLatencyBoost: 0,
    minimumIntervalUs: 0,
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
  pub const fn with_lowLatencyMode(mut self, val: VkBool32) -> Self {
    self.lowLatencyMode = val;
    self
  }
  #[inline]
  pub const fn with_lowLatencyBoost(mut self, val: VkBool32) -> Self {
    self.lowLatencyBoost = val;
    self
  }
  #[inline]
  pub const fn with_minimumIntervalUs(mut self, val: u32) -> Self {
    self.minimumIntervalUs = val;
    self
  }
  #[cfg(feature = "VK_NV_low_latency2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkLatencySleepModeInfoNV<
    'root,
    T: VkPNextExtends<VkLatencySleepModeInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkLatencySleepInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencySleepInfoNV.html)
#[cfg(feature = "VK_NV_low_latency2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkLatencySleepInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_LATENCY_SLEEP_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  pub signalSemaphore: VkSemaphore,
  pub value: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Send for VkLatencySleepInfoNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Sync for VkLatencySleepInfoNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
impl<'a> VkLatencySleepInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::LATENCY_SLEEP_INFO_NV,
    pNext: core::ptr::null(),
    signalSemaphore: VkSemaphore::DEFAULT,
    value: 0,
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
  pub const fn with_signalSemaphore(mut self, val: VkSemaphore) -> Self {
    self.signalSemaphore = val;
    self
  }
  #[inline]
  pub const fn with_value(mut self, val: u64) -> Self {
    self.value = val;
    self
  }
  #[cfg(feature = "VK_NV_low_latency2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkLatencySleepInfoNV<
    'root,
    T: VkPNextExtends<VkLatencySleepInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSetLatencyMarkerInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSetLatencyMarkerInfoNV.html)
#[cfg(feature = "VK_NV_low_latency2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSetLatencyMarkerInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_SET_LATENCY_MARKER_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  pub presentID: u64,
  pub marker: VkLatencyMarkerNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Send for VkSetLatencyMarkerInfoNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Sync for VkSetLatencyMarkerInfoNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
impl<'a> VkSetLatencyMarkerInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SET_LATENCY_MARKER_INFO_NV,
    pNext: core::ptr::null(),
    presentID: 0,
    marker: VkLatencyMarkerNV(0),
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
  pub const fn with_presentID(mut self, val: u64) -> Self {
    self.presentID = val;
    self
  }
  #[inline]
  pub const fn with_marker(mut self, val: VkLatencyMarkerNV) -> Self {
    self.marker = val;
    self
  }
  #[cfg(feature = "VK_NV_low_latency2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSetLatencyMarkerInfoNV<
    'root,
    T: VkPNextExtends<VkSetLatencyMarkerInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkGetLatencyMarkerInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkGetLatencyMarkerInfoNV.html)
#[cfg(feature = "VK_NV_low_latency2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGetLatencyMarkerInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_GET_LATENCY_MARKER_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  /// Optional: true
  pub timingCount: u32,
  /// Optional: true,  Length: timingCount
  pub pTimings: *mut VkLatencyTimingsFrameReportNV<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Send for VkGetLatencyMarkerInfoNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Sync for VkGetLatencyMarkerInfoNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
impl<'a> VkGetLatencyMarkerInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GET_LATENCY_MARKER_INFO_NV,
    pNext: core::ptr::null(),
    timingCount: 0,
    pTimings: core::ptr::null_mut(),
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
  pub const fn with_timingCount(mut self, val: u32) -> Self {
    self.timingCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pTimings(mut self, val: &'a mut [VkLatencyTimingsFrameReportNV<'a>]) -> Self {
    self.timingCount = val.len() as u32;
    self.pTimings = val.as_mut_ptr();
    self
  }
  #[cfg(feature = "VK_NV_low_latency2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGetLatencyMarkerInfoNV<
    'root,
    T: VkPNextExtends<VkGetLatencyMarkerInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkLatencyTimingsFrameReportNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencyTimingsFrameReportNV.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_NV_low_latency2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkLatencyTimingsFrameReportNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_LATENCY_TIMINGS_FRAME_REPORT_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub presentID: u64,
  pub inputSampleTimeUs: u64,
  pub simStartTimeUs: u64,
  pub simEndTimeUs: u64,
  pub renderSubmitStartTimeUs: u64,
  pub renderSubmitEndTimeUs: u64,
  pub presentStartTimeUs: u64,
  pub presentEndTimeUs: u64,
  pub driverStartTimeUs: u64,
  pub driverEndTimeUs: u64,
  pub osRenderQueueStartTimeUs: u64,
  pub osRenderQueueEndTimeUs: u64,
  pub gpuRenderStartTimeUs: u64,
  pub gpuRenderEndTimeUs: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Send for VkLatencyTimingsFrameReportNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Sync for VkLatencyTimingsFrameReportNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
impl<'a> VkLatencyTimingsFrameReportNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::LATENCY_TIMINGS_FRAME_REPORT_NV,
    pNext: core::ptr::null_mut(),
    presentID: 0,
    inputSampleTimeUs: 0,
    simStartTimeUs: 0,
    simEndTimeUs: 0,
    renderSubmitStartTimeUs: 0,
    renderSubmitEndTimeUs: 0,
    presentStartTimeUs: 0,
    presentEndTimeUs: 0,
    driverStartTimeUs: 0,
    driverEndTimeUs: 0,
    osRenderQueueStartTimeUs: 0,
    osRenderQueueEndTimeUs: 0,
    gpuRenderStartTimeUs: 0,
    gpuRenderEndTimeUs: 0,
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
  pub const fn with_presentID(mut self, val: u64) -> Self {
    self.presentID = val;
    self
  }
  #[inline]
  pub const fn with_inputSampleTimeUs(mut self, val: u64) -> Self {
    self.inputSampleTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_simStartTimeUs(mut self, val: u64) -> Self {
    self.simStartTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_simEndTimeUs(mut self, val: u64) -> Self {
    self.simEndTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_renderSubmitStartTimeUs(mut self, val: u64) -> Self {
    self.renderSubmitStartTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_renderSubmitEndTimeUs(mut self, val: u64) -> Self {
    self.renderSubmitEndTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_presentStartTimeUs(mut self, val: u64) -> Self {
    self.presentStartTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_presentEndTimeUs(mut self, val: u64) -> Self {
    self.presentEndTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_driverStartTimeUs(mut self, val: u64) -> Self {
    self.driverStartTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_driverEndTimeUs(mut self, val: u64) -> Self {
    self.driverEndTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_osRenderQueueStartTimeUs(mut self, val: u64) -> Self {
    self.osRenderQueueStartTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_osRenderQueueEndTimeUs(mut self, val: u64) -> Self {
    self.osRenderQueueEndTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_gpuRenderStartTimeUs(mut self, val: u64) -> Self {
    self.gpuRenderStartTimeUs = val;
    self
  }
  #[inline]
  pub const fn with_gpuRenderEndTimeUs(mut self, val: u64) -> Self {
    self.gpuRenderEndTimeUs = val;
    self
  }
  #[cfg(feature = "VK_NV_low_latency2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkLatencyTimingsFrameReportNV<
    'root,
    T: VkPNextExtends<VkLatencyTimingsFrameReportNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkOutOfBandQueueTypeInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkOutOfBandQueueTypeInfoNV.html)
#[cfg(feature = "VK_NV_low_latency2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkOutOfBandQueueTypeInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_OUT_OF_BAND_QUEUE_TYPE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  pub queueType: VkOutOfBandQueueTypeNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Send for VkOutOfBandQueueTypeInfoNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Sync for VkOutOfBandQueueTypeInfoNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
impl<'a> VkOutOfBandQueueTypeInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::OUT_OF_BAND_QUEUE_TYPE_INFO_NV,
    pNext: core::ptr::null(),
    queueType: VkOutOfBandQueueTypeNV(0),
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
  pub const fn with_queueType(mut self, val: VkOutOfBandQueueTypeNV) -> Self {
    self.queueType = val;
    self
  }
  #[cfg(feature = "VK_NV_low_latency2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkOutOfBandQueueTypeInfoNV<
    'root,
    T: VkPNextExtends<VkOutOfBandQueueTypeInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkLatencySubmissionPresentIdNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencySubmissionPresentIdNV.html)
///
/// **Extends:** VkSubmitInfo, VkSubmitInfo2.
#[cfg(feature = "VK_NV_low_latency2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkLatencySubmissionPresentIdNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_LATENCY_SUBMISSION_PRESENT_ID_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub presentID: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Send for VkLatencySubmissionPresentIdNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Sync for VkLatencySubmissionPresentIdNV<'a> {}
#[cfg(all(feature = "VK_NV_low_latency2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubmitInfo<'root>>
  for VkLatencySubmissionPresentIdNV<'child>
{
}
#[cfg(all(feature = "VK_NV_low_latency2", feature = "VK_BASE_VERSION_1_3"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubmitInfo2<'root>>
  for VkLatencySubmissionPresentIdNV<'child>
{
}
#[cfg(feature = "VK_NV_low_latency2")]
impl<'a> VkLatencySubmissionPresentIdNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::LATENCY_SUBMISSION_PRESENT_ID_NV,
    pNext: core::ptr::null(),
    presentID: 0,
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
  pub const fn with_presentID(mut self, val: u64) -> Self {
    self.presentID = val;
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
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubmitInfo2<'root, T: VkPNextExtends<VkSubmitInfo2<'root>>>(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSwapchainLatencyCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainLatencyCreateInfoNV.html)
///
/// **Extends:** VkSwapchainCreateInfoKHR.
#[cfg(feature = "VK_NV_low_latency2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSwapchainLatencyCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_SWAPCHAIN_LATENCY_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub latencyModeEnable: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Send for VkSwapchainLatencyCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Sync for VkSwapchainLatencyCreateInfoNV<'a> {}
#[cfg(all(feature = "VK_NV_low_latency2", feature = "VK_KHR_swapchain"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSwapchainCreateInfoKHR<'root>>
  for VkSwapchainLatencyCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_low_latency2")]
impl<'a> VkSwapchainLatencyCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SWAPCHAIN_LATENCY_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    latencyModeEnable: 0,
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
  pub const fn with_latencyModeEnable(mut self, val: VkBool32) -> Self {
    self.latencyModeEnable = val;
    self
  }
  #[cfg(feature = "VK_KHR_swapchain")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSwapchainCreateInfoKHR<
    'root,
    T: VkPNextExtends<VkSwapchainCreateInfoKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkLatencySurfaceCapabilitiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencySurfaceCapabilitiesNV.html)
///
/// **Extends:** VkSurfaceCapabilities2KHR.
#[cfg(feature = "VK_NV_low_latency2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkLatencySurfaceCapabilitiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_LATENCY_SURFACE_CAPABILITIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub presentModeCount: u32,
  /// Optional: true,  Length: presentModeCount
  pub pPresentModes: *mut VkPresentModeKHR,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Send for VkLatencySurfaceCapabilitiesNV<'a> {}
#[cfg(feature = "VK_NV_low_latency2")]
unsafe impl<'a> Sync for VkLatencySurfaceCapabilitiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_low_latency2",
  feature = "VK_KHR_get_surface_capabilities2"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSurfaceCapabilities2KHR<'root>>
  for VkLatencySurfaceCapabilitiesNV<'child>
{
}
#[cfg(feature = "VK_NV_low_latency2")]
impl<'a> VkLatencySurfaceCapabilitiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::LATENCY_SURFACE_CAPABILITIES_NV,
    pNext: core::ptr::null(),
    presentModeCount: 0,
    pPresentModes: core::ptr::null_mut(),
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
  pub const fn with_presentModeCount(mut self, val: u32) -> Self {
    self.presentModeCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPresentModes(mut self, val: &'a mut [VkPresentModeKHR]) -> Self {
    self.presentModeCount = val.len() as u32;
    self.pPresentModes = val.as_mut_ptr();
    self
  }
  #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSurfaceCapabilities2KHR<
    'root,
    T: VkPNextExtends<VkSurfaceCapabilities2KHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkMemoryDecompressionMethodFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagsNV.html)
#[cfg(feature = "VK_NV_memory_decompression")]
pub type VkMemoryDecompressionMethodFlagsNV = VkFlags64;
/// [VkPhysicalDeviceMemoryDecompressionFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryDecompressionFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_memory_decompression")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMemoryDecompressionFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub memoryDecompression: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_memory_decompression")]
unsafe impl<'a> Send for VkPhysicalDeviceMemoryDecompressionFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_memory_decompression")]
unsafe impl<'a> Sync for VkPhysicalDeviceMemoryDecompressionFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_memory_decompression",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceMemoryDecompressionFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_memory_decompression",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceMemoryDecompressionFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_memory_decompression")]
impl<'a> VkPhysicalDeviceMemoryDecompressionFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    memoryDecompression: 0,
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
  pub const fn with_memoryDecompression(mut self, val: VkBool32) -> Self {
    self.memoryDecompression = val;
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
/// [VkPhysicalDeviceMemoryDecompressionPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryDecompressionPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_memory_decompression")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMemoryDecompressionPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub decompressionMethods: VkMemoryDecompressionMethodFlagsNV,
  /// Limit Type: [Max]
  pub maxDecompressionIndirectCount: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_memory_decompression")]
unsafe impl<'a> Send for VkPhysicalDeviceMemoryDecompressionPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_memory_decompression")]
unsafe impl<'a> Sync for VkPhysicalDeviceMemoryDecompressionPropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_memory_decompression",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceMemoryDecompressionPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_memory_decompression")]
impl<'a> VkPhysicalDeviceMemoryDecompressionPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    decompressionMethods: 0,
    maxDecompressionIndirectCount: 0,
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
  pub const fn with_decompressionMethods(
    mut self,
    val: VkMemoryDecompressionMethodFlagsNV,
  ) -> Self {
    self.decompressionMethods = val;
    self
  }
  #[inline]
  pub const fn with_maxDecompressionIndirectCount(mut self, val: u64) -> Self {
    self.maxDecompressionIndirectCount = val;
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
/// [VkDecompressMemoryRegionNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkDecompressMemoryRegionNV.html)
#[cfg(feature = "VK_NV_memory_decompression")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDecompressMemoryRegionNV {
  pub srcAddress: VkDeviceAddress,
  pub dstAddress: VkDeviceAddress,
  pub compressedSize: VkDeviceSize,
  pub decompressedSize: VkDeviceSize,
  pub decompressionMethod: VkMemoryDecompressionMethodFlagsEXT,
}
#[cfg(feature = "VK_NV_memory_decompression")]
unsafe impl Send for VkDecompressMemoryRegionNV {}
#[cfg(feature = "VK_NV_memory_decompression")]
unsafe impl Sync for VkDecompressMemoryRegionNV {}
#[cfg(feature = "VK_NV_memory_decompression")]
impl VkDecompressMemoryRegionNV {
  pub const DEFAULT: Self = Self {
    srcAddress: 0,
    dstAddress: 0,
    compressedSize: 0,
    decompressedSize: 0,
    decompressionMethod: VkMemoryDecompressionMethodFlagBitsEXT(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_srcAddress(mut self, val: VkDeviceAddress) -> Self {
    self.srcAddress = val;
    self
  }
  #[inline]
  pub const fn with_dstAddress(mut self, val: VkDeviceAddress) -> Self {
    self.dstAddress = val;
    self
  }
  #[inline]
  pub const fn with_compressedSize(mut self, val: VkDeviceSize) -> Self {
    self.compressedSize = val;
    self
  }
  #[inline]
  pub const fn with_decompressedSize(mut self, val: VkDeviceSize) -> Self {
    self.decompressedSize = val;
    self
  }
  #[inline]
  pub const fn with_decompressionMethod(
    mut self,
    val: VkMemoryDecompressionMethodFlagsEXT,
  ) -> Self {
    self.decompressionMethod = val;
    self
  }
}
/// [VkPhysicalDeviceMeshShaderFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMeshShaderFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_mesh_shader")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMeshShaderFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub taskShader: VkBool32,
  pub meshShader: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_mesh_shader")]
unsafe impl<'a> Send for VkPhysicalDeviceMeshShaderFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_mesh_shader")]
unsafe impl<'a> Sync for VkPhysicalDeviceMeshShaderFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_mesh_shader", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceMeshShaderFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_mesh_shader", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceMeshShaderFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_mesh_shader")]
impl<'a> VkPhysicalDeviceMeshShaderFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    taskShader: 0,
    meshShader: 0,
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
  pub const fn with_taskShader(mut self, val: VkBool32) -> Self {
    self.taskShader = val;
    self
  }
  #[inline]
  pub const fn with_meshShader(mut self, val: VkBool32) -> Self {
    self.meshShader = val;
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
/// [VkPhysicalDeviceMeshShaderPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMeshShaderPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_mesh_shader")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMeshShaderPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxDrawMeshTasksCount: u32,
  /// Limit Type: [Max]
  pub maxTaskWorkGroupInvocations: u32,
  /// Limit Type: [Max]
  pub maxTaskWorkGroupSize: [u32; 3],
  /// Limit Type: [Max]
  pub maxTaskTotalMemorySize: u32,
  /// Limit Type: [Max]
  pub maxTaskOutputCount: u32,
  /// Limit Type: [Max]
  pub maxMeshWorkGroupInvocations: u32,
  /// Limit Type: [Max]
  pub maxMeshWorkGroupSize: [u32; 3],
  /// Limit Type: [Max]
  pub maxMeshTotalMemorySize: u32,
  /// Limit Type: [Max]
  pub maxMeshOutputVertices: u32,
  /// Limit Type: [Max]
  pub maxMeshOutputPrimitives: u32,
  /// Limit Type: [Max]
  pub maxMeshMultiviewViewCount: u32,
  /// Limit Type: [Min, Mul]
  pub meshOutputPerVertexGranularity: u32,
  /// Limit Type: [Min, Mul]
  pub meshOutputPerPrimitiveGranularity: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_mesh_shader")]
unsafe impl<'a> Send for VkPhysicalDeviceMeshShaderPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_mesh_shader")]
unsafe impl<'a> Sync for VkPhysicalDeviceMeshShaderPropertiesNV<'a> {}
#[cfg(all(feature = "VK_NV_mesh_shader", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceMeshShaderPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_mesh_shader")]
impl<'a> VkPhysicalDeviceMeshShaderPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    maxDrawMeshTasksCount: 0,
    maxTaskWorkGroupInvocations: 0,
    maxTaskWorkGroupSize: [0u32; 3],
    maxTaskTotalMemorySize: 0,
    maxTaskOutputCount: 0,
    maxMeshWorkGroupInvocations: 0,
    maxMeshWorkGroupSize: [0u32; 3],
    maxMeshTotalMemorySize: 0,
    maxMeshOutputVertices: 0,
    maxMeshOutputPrimitives: 0,
    maxMeshMultiviewViewCount: 0,
    meshOutputPerVertexGranularity: 0,
    meshOutputPerPrimitiveGranularity: 0,
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
  pub const fn with_maxDrawMeshTasksCount(mut self, val: u32) -> Self {
    self.maxDrawMeshTasksCount = val;
    self
  }
  #[inline]
  pub const fn with_maxTaskWorkGroupInvocations(mut self, val: u32) -> Self {
    self.maxTaskWorkGroupInvocations = val;
    self
  }
  #[inline]
  pub const fn with_maxTaskWorkGroupSize(mut self, val: [u32; 3]) -> Self {
    self.maxTaskWorkGroupSize = val;
    self
  }
  #[inline]
  pub const fn with_maxTaskTotalMemorySize(mut self, val: u32) -> Self {
    self.maxTaskTotalMemorySize = val;
    self
  }
  #[inline]
  pub const fn with_maxTaskOutputCount(mut self, val: u32) -> Self {
    self.maxTaskOutputCount = val;
    self
  }
  #[inline]
  pub const fn with_maxMeshWorkGroupInvocations(mut self, val: u32) -> Self {
    self.maxMeshWorkGroupInvocations = val;
    self
  }
  #[inline]
  pub const fn with_maxMeshWorkGroupSize(mut self, val: [u32; 3]) -> Self {
    self.maxMeshWorkGroupSize = val;
    self
  }
  #[inline]
  pub const fn with_maxMeshTotalMemorySize(mut self, val: u32) -> Self {
    self.maxMeshTotalMemorySize = val;
    self
  }
  #[inline]
  pub const fn with_maxMeshOutputVertices(mut self, val: u32) -> Self {
    self.maxMeshOutputVertices = val;
    self
  }
  #[inline]
  pub const fn with_maxMeshOutputPrimitives(mut self, val: u32) -> Self {
    self.maxMeshOutputPrimitives = val;
    self
  }
  #[inline]
  pub const fn with_maxMeshMultiviewViewCount(mut self, val: u32) -> Self {
    self.maxMeshMultiviewViewCount = val;
    self
  }
  #[inline]
  pub const fn with_meshOutputPerVertexGranularity(mut self, val: u32) -> Self {
    self.meshOutputPerVertexGranularity = val;
    self
  }
  #[inline]
  pub const fn with_meshOutputPerPrimitiveGranularity(mut self, val: u32) -> Self {
    self.meshOutputPerPrimitiveGranularity = val;
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
/// [VkDrawMeshTasksIndirectCommandNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkDrawMeshTasksIndirectCommandNV.html)
#[cfg(feature = "VK_NV_mesh_shader")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDrawMeshTasksIndirectCommandNV {
  pub taskCount: u32,
  pub firstTask: u32,
}
#[cfg(feature = "VK_NV_mesh_shader")]
unsafe impl Send for VkDrawMeshTasksIndirectCommandNV {}
#[cfg(feature = "VK_NV_mesh_shader")]
unsafe impl Sync for VkDrawMeshTasksIndirectCommandNV {}
#[cfg(feature = "VK_NV_mesh_shader")]
impl VkDrawMeshTasksIndirectCommandNV {
  pub const DEFAULT: Self = Self {
    taskCount: 0,
    firstTask: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_taskCount(mut self, val: u32) -> Self {
    self.taskCount = val;
    self
  }
  #[inline]
  pub const fn with_firstTask(mut self, val: u32) -> Self {
    self.firstTask = val;
    self
  }
}
/// [VkOpticalFlowGridSizeFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowGridSizeFlagsNV.html)
#[cfg(feature = "VK_NV_optical_flow")]
pub type VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagBitsNV;
/// [VkOpticalFlowUsageFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowUsageFlagsNV.html)
#[cfg(feature = "VK_NV_optical_flow")]
pub type VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagBitsNV;
/// [VkOpticalFlowSessionCreateFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreateFlagsNV.html)
#[cfg(feature = "VK_NV_optical_flow")]
pub type VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagBitsNV;
/// [VkOpticalFlowExecuteFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowExecuteFlagsNV.html)
#[cfg(feature = "VK_NV_optical_flow")]
pub type VkOpticalFlowExecuteFlagsNV = VkOpticalFlowExecuteFlagBitsNV;
/// [VkOpticalFlowSessionNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionNV.html)
#[cfg(feature = "VK_NV_optical_flow")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkOpticalFlowSessionNV(pub *mut c_void);
#[cfg(feature = "VK_NV_optical_flow")]
impl VkOpticalFlowSessionNV {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_NV_optical_flow")]
impl Default for VkOpticalFlowSessionNV {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl Send for VkOpticalFlowSessionNV {}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl Sync for VkOpticalFlowSessionNV {}
/// [VkPhysicalDeviceOpticalFlowFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceOpticalFlowFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceOpticalFlowFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub opticalFlow: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Send for VkPhysicalDeviceOpticalFlowFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Sync for VkPhysicalDeviceOpticalFlowFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_optical_flow", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceOpticalFlowFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_optical_flow", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceOpticalFlowFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_optical_flow")]
impl<'a> VkPhysicalDeviceOpticalFlowFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    opticalFlow: 0,
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
  pub const fn with_opticalFlow(mut self, val: VkBool32) -> Self {
    self.opticalFlow = val;
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
/// [VkPhysicalDeviceOpticalFlowPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceOpticalFlowPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceOpticalFlowPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub supportedOutputGridSizes: VkOpticalFlowGridSizeFlagsNV,
  /// Limit Type: [Bitmask]
  pub supportedHintGridSizes: VkOpticalFlowGridSizeFlagsNV,
  /// Limit Type: [Max]
  pub hintSupported: VkBool32,
  /// Limit Type: [Max]
  pub costSupported: VkBool32,
  /// Limit Type: [Max]
  pub bidirectionalFlowSupported: VkBool32,
  /// Limit Type: [Max]
  pub globalFlowSupported: VkBool32,
  /// Limit Type: [Noauto]
  pub minWidth: u32,
  /// Limit Type: [Noauto]
  pub minHeight: u32,
  /// Limit Type: [Noauto]
  pub maxWidth: u32,
  /// Limit Type: [Noauto]
  pub maxHeight: u32,
  /// Limit Type: [Noauto]
  pub maxNumRegionsOfInterest: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Send for VkPhysicalDeviceOpticalFlowPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Sync for VkPhysicalDeviceOpticalFlowPropertiesNV<'a> {}
#[cfg(all(feature = "VK_NV_optical_flow", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceOpticalFlowPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_optical_flow")]
impl<'a> VkPhysicalDeviceOpticalFlowPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    supportedOutputGridSizes: VkOpticalFlowGridSizeFlagBitsNV(0),
    supportedHintGridSizes: VkOpticalFlowGridSizeFlagBitsNV(0),
    hintSupported: 0,
    costSupported: 0,
    bidirectionalFlowSupported: 0,
    globalFlowSupported: 0,
    minWidth: 0,
    minHeight: 0,
    maxWidth: 0,
    maxHeight: 0,
    maxNumRegionsOfInterest: 0,
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
  pub const fn with_supportedOutputGridSizes(mut self, val: VkOpticalFlowGridSizeFlagsNV) -> Self {
    self.supportedOutputGridSizes = val;
    self
  }
  #[inline]
  pub const fn with_supportedHintGridSizes(mut self, val: VkOpticalFlowGridSizeFlagsNV) -> Self {
    self.supportedHintGridSizes = val;
    self
  }
  #[inline]
  pub const fn with_hintSupported(mut self, val: VkBool32) -> Self {
    self.hintSupported = val;
    self
  }
  #[inline]
  pub const fn with_costSupported(mut self, val: VkBool32) -> Self {
    self.costSupported = val;
    self
  }
  #[inline]
  pub const fn with_bidirectionalFlowSupported(mut self, val: VkBool32) -> Self {
    self.bidirectionalFlowSupported = val;
    self
  }
  #[inline]
  pub const fn with_globalFlowSupported(mut self, val: VkBool32) -> Self {
    self.globalFlowSupported = val;
    self
  }
  #[inline]
  pub const fn with_minWidth(mut self, val: u32) -> Self {
    self.minWidth = val;
    self
  }
  #[inline]
  pub const fn with_minHeight(mut self, val: u32) -> Self {
    self.minHeight = val;
    self
  }
  #[inline]
  pub const fn with_maxWidth(mut self, val: u32) -> Self {
    self.maxWidth = val;
    self
  }
  #[inline]
  pub const fn with_maxHeight(mut self, val: u32) -> Self {
    self.maxHeight = val;
    self
  }
  #[inline]
  pub const fn with_maxNumRegionsOfInterest(mut self, val: u32) -> Self {
    self.maxNumRegionsOfInterest = val;
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
/// [VkOpticalFlowImageFormatInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowImageFormatInfoNV.html)
///
/// **Extends:** VkPhysicalDeviceImageFormatInfo2, VkImageCreateInfo.
#[cfg(feature = "VK_NV_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkOpticalFlowImageFormatInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub usage: VkOpticalFlowUsageFlagsNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Send for VkOpticalFlowImageFormatInfoNV<'a> {}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Sync for VkOpticalFlowImageFormatInfoNV<'a> {}
#[cfg(all(feature = "VK_NV_optical_flow", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceImageFormatInfo2<'root>>
  for VkOpticalFlowImageFormatInfoNV<'child>
{
}
#[cfg(all(feature = "VK_NV_optical_flow", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkOpticalFlowImageFormatInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_optical_flow")]
impl<'a> VkOpticalFlowImageFormatInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV,
    pNext: core::ptr::null(),
    usage: VkOpticalFlowUsageFlagBitsNV(0),
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
  pub const fn with_usage(mut self, val: VkOpticalFlowUsageFlagsNV) -> Self {
    self.usage = val;
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
/// [VkOpticalFlowImageFormatPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowImageFormatPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_NV_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkOpticalFlowImageFormatPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub format: VkFormat,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Send for VkOpticalFlowImageFormatPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Sync for VkOpticalFlowImageFormatPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_optical_flow")]
impl<'a> VkOpticalFlowImageFormatPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    format: VkFormat(0),
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
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[cfg(feature = "VK_NV_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkOpticalFlowImageFormatPropertiesNV<
    'root,
    T: VkPNextExtends<VkOpticalFlowImageFormatPropertiesNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkOpticalFlowSessionCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreateInfoNV.html)
#[cfg(feature = "VK_NV_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkOpticalFlowSessionCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub width: u32,
  pub height: u32,
  pub imageFormat: VkFormat,
  pub flowVectorFormat: VkFormat,
  /// Optional: true
  pub costFormat: VkFormat,
  pub outputGridSize: VkOpticalFlowGridSizeFlagsNV,
  /// Optional: true
  pub hintGridSize: VkOpticalFlowGridSizeFlagsNV,
  /// Optional: true
  pub performanceLevel: VkOpticalFlowPerformanceLevelNV,
  /// Optional: true
  pub flags: VkOpticalFlowSessionCreateFlagsNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Send for VkOpticalFlowSessionCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Sync for VkOpticalFlowSessionCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_optical_flow")]
impl<'a> VkOpticalFlowSessionCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::OPTICAL_FLOW_SESSION_CREATE_INFO_NV,
    pNext: core::ptr::null_mut(),
    width: 0,
    height: 0,
    imageFormat: VkFormat(0),
    flowVectorFormat: VkFormat(0),
    costFormat: VkFormat(0),
    outputGridSize: VkOpticalFlowGridSizeFlagBitsNV(0),
    hintGridSize: VkOpticalFlowGridSizeFlagBitsNV(0),
    performanceLevel: VkOpticalFlowPerformanceLevelNV(0),
    flags: VkOpticalFlowSessionCreateFlagBitsNV(0),
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
  pub const fn with_imageFormat(mut self, val: VkFormat) -> Self {
    self.imageFormat = val;
    self
  }
  #[inline]
  pub const fn with_flowVectorFormat(mut self, val: VkFormat) -> Self {
    self.flowVectorFormat = val;
    self
  }
  #[inline]
  pub const fn with_costFormat(mut self, val: VkFormat) -> Self {
    self.costFormat = val;
    self
  }
  #[inline]
  pub const fn with_outputGridSize(mut self, val: VkOpticalFlowGridSizeFlagsNV) -> Self {
    self.outputGridSize = val;
    self
  }
  #[inline]
  pub const fn with_hintGridSize(mut self, val: VkOpticalFlowGridSizeFlagsNV) -> Self {
    self.hintGridSize = val;
    self
  }
  #[inline]
  pub const fn with_performanceLevel(mut self, val: VkOpticalFlowPerformanceLevelNV) -> Self {
    self.performanceLevel = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkOpticalFlowSessionCreateFlagsNV) -> Self {
    self.flags = val;
    self
  }
  #[cfg(feature = "VK_NV_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkOpticalFlowSessionCreatePrivateDataInfoNV<'child>(
    mut self,
    val: &'a mut VkOpticalFlowSessionCreatePrivateDataInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkOpticalFlowSessionCreatePrivateDataInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkOpticalFlowSessionCreateInfoNV<
    'root,
    T: VkPNextExtends<VkOpticalFlowSessionCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkOpticalFlowSessionCreatePrivateDataInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreatePrivateDataInfoNV.html)
///
/// **Extends:** VkOpticalFlowSessionCreateInfoNV.
#[cfg(feature = "VK_NV_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkOpticalFlowSessionCreatePrivateDataInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub id: u32,
  pub size: u32,
  pub pPrivateData: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Send for VkOpticalFlowSessionCreatePrivateDataInfoNV<'a> {}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Sync for VkOpticalFlowSessionCreatePrivateDataInfoNV<'a> {}
#[cfg(all(feature = "VK_NV_optical_flow", feature = "VK_NV_optical_flow"))]
unsafe impl<'child, 'root> VkPNextExtends<VkOpticalFlowSessionCreateInfoNV<'root>>
  for VkOpticalFlowSessionCreatePrivateDataInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_optical_flow")]
impl<'a> VkOpticalFlowSessionCreatePrivateDataInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV,
    pNext: core::ptr::null_mut(),
    id: 0,
    size: 0,
    pPrivateData: core::ptr::null(),
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
  pub const fn with_id(mut self, val: u32) -> Self {
    self.id = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: u32) -> Self {
    self.size = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPrivateData(mut self, val: *const c_void) -> Self {
    self.pPrivateData = val;
    self
  }
  #[cfg(feature = "VK_NV_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkOpticalFlowSessionCreateInfoNV<
    'root,
    T: VkPNextExtends<VkOpticalFlowSessionCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkOpticalFlowExecuteInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowExecuteInfoNV.html)
#[cfg(feature = "VK_NV_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkOpticalFlowExecuteInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_OPTICAL_FLOW_EXECUTE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Optional: true
  pub flags: VkOpticalFlowExecuteFlagsNV,
  /// Optional: true
  pub regionCount: u32,
  /// Length: regionCount
  pub pRegions: *const VkRect2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Send for VkOpticalFlowExecuteInfoNV<'a> {}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'a> Sync for VkOpticalFlowExecuteInfoNV<'a> {}
#[cfg(feature = "VK_NV_optical_flow")]
impl<'a> VkOpticalFlowExecuteInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::OPTICAL_FLOW_EXECUTE_INFO_NV,
    pNext: core::ptr::null_mut(),
    flags: VkOpticalFlowExecuteFlagBitsNV(0),
    regionCount: 0,
    pRegions: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkOpticalFlowExecuteFlagsNV) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_regionCount(mut self, val: u32) -> Self {
    self.regionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pRegions(mut self, val: &'a [VkRect2D]) -> Self {
    self.regionCount = val.len() as u32;
    self.pRegions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_NV_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkOpticalFlowExecuteInfoNV<
    'root,
    T: VkPNextExtends<VkOpticalFlowExecuteInfoNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPartitionedAccelerationStructureInstanceFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureInstanceFlagsNV.html)
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
pub type VkPartitionedAccelerationStructureInstanceFlagsNV =
  VkPartitionedAccelerationStructureInstanceFlagBitsNV;
/// [VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub partitionedAccelerationStructure: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Send for VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Sync for VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_partitioned_acceleration_structure",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_partitioned_acceleration_structure",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
impl<'a> VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    partitionedAccelerationStructure: 0,
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
  pub const fn with_partitionedAccelerationStructure(mut self, val: VkBool32) -> Self {
    self.partitionedAccelerationStructure = val;
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
/// [VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxPartitionCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Send for VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV<'a> {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Sync for VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_partitioned_acceleration_structure",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
impl<'a> VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    maxPartitionCount: 0,
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
  pub const fn with_maxPartitionCount(mut self, val: u32) -> Self {
    self.maxPartitionCount = val;
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
/// [VkBuildPartitionedAccelerationStructureIndirectCommandNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildPartitionedAccelerationStructureIndirectCommandNV.html)
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBuildPartitionedAccelerationStructureIndirectCommandNV {
  pub opType: VkPartitionedAccelerationStructureOpTypeNV,
  pub argCount: u32,
  pub argData: VkStridedDeviceAddressNV,
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl Send for VkBuildPartitionedAccelerationStructureIndirectCommandNV {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl Sync for VkBuildPartitionedAccelerationStructureIndirectCommandNV {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
impl VkBuildPartitionedAccelerationStructureIndirectCommandNV {
  pub const DEFAULT: Self = Self {
    opType: VkPartitionedAccelerationStructureOpTypeNV(0),
    argCount: 0,
    argData: VkStridedDeviceAddressNV::DEFAULT,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_opType(mut self, val: VkPartitionedAccelerationStructureOpTypeNV) -> Self {
    self.opType = val;
    self
  }
  #[inline]
  pub const fn with_argCount(mut self, val: u32) -> Self {
    self.argCount = val;
    self
  }
  #[inline]
  pub const fn with_argData(mut self, val: VkStridedDeviceAddressNV) -> Self {
    self.argData = val;
    self
  }
}
/// [VkPartitionedAccelerationStructureFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureFlagsNV.html)
///
/// **Extends:** VkPartitionedAccelerationStructureInstancesInputNV.
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPartitionedAccelerationStructureFlagsNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub enablePartitionTranslation: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Send for VkPartitionedAccelerationStructureFlagsNV<'a> {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Sync for VkPartitionedAccelerationStructureFlagsNV<'a> {}
#[cfg(all(
  feature = "VK_NV_partitioned_acceleration_structure",
  feature = "VK_NV_partitioned_acceleration_structure"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPartitionedAccelerationStructureInstancesInputNV<'root>>
  for VkPartitionedAccelerationStructureFlagsNV<'child>
{
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
impl<'a> VkPartitionedAccelerationStructureFlagsNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV,
    pNext: core::ptr::null_mut(),
    enablePartitionTranslation: 0,
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
  pub const fn with_enablePartitionTranslation(mut self, val: VkBool32) -> Self {
    self.enablePartitionTranslation = val;
    self
  }
  #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPartitionedAccelerationStructureInstancesInputNV<
    'root,
    T: VkPNextExtends<VkPartitionedAccelerationStructureInstancesInputNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPartitionedAccelerationStructureWriteInstanceDataNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureWriteInstanceDataNV.html)
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPartitionedAccelerationStructureWriteInstanceDataNV {
  pub transform: VkTransformMatrixKHR,
  pub explicitAABB: [f32; 6],
  pub instanceID: u32,
  pub instanceMask: u32,
  pub instanceContributionToHitGroupIndex: u32,
  /// Optional: true
  pub instanceFlags: VkPartitionedAccelerationStructureInstanceFlagsNV,
  pub instanceIndex: u32,
  pub partitionIndex: u32,
  pub accelerationStructure: VkDeviceAddress,
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl Send for VkPartitionedAccelerationStructureWriteInstanceDataNV {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl Sync for VkPartitionedAccelerationStructureWriteInstanceDataNV {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
impl VkPartitionedAccelerationStructureWriteInstanceDataNV {
  pub const DEFAULT: Self = Self {
    transform: VkTransformMatrixKHR::DEFAULT,
    explicitAABB: [0.0f32; 6],
    instanceID: 0,
    instanceMask: 0,
    instanceContributionToHitGroupIndex: 0,
    instanceFlags: VkPartitionedAccelerationStructureInstanceFlagBitsNV(0),
    instanceIndex: 0,
    partitionIndex: 0,
    accelerationStructure: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_transform(mut self, val: VkTransformMatrixKHR) -> Self {
    self.transform = val;
    self
  }
  #[inline]
  pub const fn with_explicitAABB(mut self, val: [f32; 6]) -> Self {
    self.explicitAABB = val;
    self
  }
  #[inline]
  pub const fn with_instanceID(mut self, val: u32) -> Self {
    self.instanceID = val;
    self
  }
  #[inline]
  pub const fn with_instanceMask(mut self, val: u32) -> Self {
    self.instanceMask = val;
    self
  }
  #[inline]
  pub const fn with_instanceContributionToHitGroupIndex(mut self, val: u32) -> Self {
    self.instanceContributionToHitGroupIndex = val;
    self
  }
  #[inline]
  pub const fn with_instanceFlags(
    mut self,
    val: VkPartitionedAccelerationStructureInstanceFlagsNV,
  ) -> Self {
    self.instanceFlags = val;
    self
  }
  #[inline]
  pub const fn with_instanceIndex(mut self, val: u32) -> Self {
    self.instanceIndex = val;
    self
  }
  #[inline]
  pub const fn with_partitionIndex(mut self, val: u32) -> Self {
    self.partitionIndex = val;
    self
  }
  #[inline]
  pub const fn with_accelerationStructure(mut self, val: VkDeviceAddress) -> Self {
    self.accelerationStructure = val;
    self
  }
}
/// [VkPartitionedAccelerationStructureUpdateInstanceDataNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureUpdateInstanceDataNV.html)
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPartitionedAccelerationStructureUpdateInstanceDataNV {
  pub instanceIndex: u32,
  pub instanceContributionToHitGroupIndex: u32,
  pub accelerationStructure: VkDeviceAddress,
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl Send for VkPartitionedAccelerationStructureUpdateInstanceDataNV {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl Sync for VkPartitionedAccelerationStructureUpdateInstanceDataNV {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
impl VkPartitionedAccelerationStructureUpdateInstanceDataNV {
  pub const DEFAULT: Self = Self {
    instanceIndex: 0,
    instanceContributionToHitGroupIndex: 0,
    accelerationStructure: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_instanceIndex(mut self, val: u32) -> Self {
    self.instanceIndex = val;
    self
  }
  #[inline]
  pub const fn with_instanceContributionToHitGroupIndex(mut self, val: u32) -> Self {
    self.instanceContributionToHitGroupIndex = val;
    self
  }
  #[inline]
  pub const fn with_accelerationStructure(mut self, val: VkDeviceAddress) -> Self {
    self.accelerationStructure = val;
    self
  }
}
/// [VkPartitionedAccelerationStructureWritePartitionTranslationDataNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureWritePartitionTranslationDataNV.html)
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPartitionedAccelerationStructureWritePartitionTranslationDataNV {
  pub partitionIndex: u32,
  pub partitionTranslation: [f32; 3],
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl Send for VkPartitionedAccelerationStructureWritePartitionTranslationDataNV {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl Sync for VkPartitionedAccelerationStructureWritePartitionTranslationDataNV {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
impl VkPartitionedAccelerationStructureWritePartitionTranslationDataNV {
  pub const DEFAULT: Self = Self {
    partitionIndex: 0,
    partitionTranslation: [0.0f32; 3],
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_partitionIndex(mut self, val: u32) -> Self {
    self.partitionIndex = val;
    self
  }
  #[inline]
  pub const fn with_partitionTranslation(mut self, val: [f32; 3]) -> Self {
    self.partitionTranslation = val;
    self
  }
}
/// [VkWriteDescriptorSetPartitionedAccelerationStructureNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSetPartitionedAccelerationStructureNV.html)
///
/// **Extends:** VkWriteDescriptorSet.
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkWriteDescriptorSetPartitionedAccelerationStructureNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub accelerationStructureCount: u32,
  /// Optional: pointer required, values optional if pointer not null,  Length: accelerationStructureCount
  pub pAccelerationStructures: *const VkDeviceAddress,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Send for VkWriteDescriptorSetPartitionedAccelerationStructureNV<'a> {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Sync for VkWriteDescriptorSetPartitionedAccelerationStructureNV<'a> {}
#[cfg(all(
  feature = "VK_NV_partitioned_acceleration_structure",
  feature = "VK_COMPUTE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkWriteDescriptorSet<'root>>
  for VkWriteDescriptorSetPartitionedAccelerationStructureNV<'child>
{
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
impl<'a> VkWriteDescriptorSetPartitionedAccelerationStructureNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV,
    pNext: core::ptr::null_mut(),
    accelerationStructureCount: 0,
    pAccelerationStructures: core::ptr::null(),
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
  pub const fn with_accelerationStructureCount(mut self, val: u32) -> Self {
    self.accelerationStructureCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAccelerationStructures(mut self, val: &'a [VkDeviceAddress]) -> Self {
    self.accelerationStructureCount = val.len() as u32;
    self.pAccelerationStructures = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkWriteDescriptorSet<
    'root,
    T: VkPNextExtends<VkWriteDescriptorSet<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPartitionedAccelerationStructureInstancesInputNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureInstancesInputNV.html)
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPartitionedAccelerationStructureInstancesInputNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Optional: true
  pub flags: VkBuildAccelerationStructureFlagsKHR,
  pub instanceCount: u32,
  pub maxInstancePerPartitionCount: u32,
  pub partitionCount: u32,
  pub maxInstanceInGlobalPartitionCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Send for VkPartitionedAccelerationStructureInstancesInputNV<'a> {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Sync for VkPartitionedAccelerationStructureInstancesInputNV<'a> {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
impl<'a> VkPartitionedAccelerationStructureInstancesInputNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV,
    pNext: core::ptr::null_mut(),
    flags: VkBuildAccelerationStructureFlagBitsKHR(0),
    instanceCount: 0,
    maxInstancePerPartitionCount: 0,
    partitionCount: 0,
    maxInstanceInGlobalPartitionCount: 0,
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
  pub const fn with_flags(mut self, val: VkBuildAccelerationStructureFlagsKHR) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_instanceCount(mut self, val: u32) -> Self {
    self.instanceCount = val;
    self
  }
  #[inline]
  pub const fn with_maxInstancePerPartitionCount(mut self, val: u32) -> Self {
    self.maxInstancePerPartitionCount = val;
    self
  }
  #[inline]
  pub const fn with_partitionCount(mut self, val: u32) -> Self {
    self.partitionCount = val;
    self
  }
  #[inline]
  pub const fn with_maxInstanceInGlobalPartitionCount(mut self, val: u32) -> Self {
    self.maxInstanceInGlobalPartitionCount = val;
    self
  }
  #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPartitionedAccelerationStructureFlagsNV<'child>(
    mut self,
    val: &'a mut VkPartitionedAccelerationStructureFlagsNV<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkPartitionedAccelerationStructureFlagsNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPartitionedAccelerationStructureInstancesInputNV<
    'root,
    T: VkPNextExtends<VkPartitionedAccelerationStructureInstancesInputNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkBuildPartitionedAccelerationStructureInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildPartitionedAccelerationStructureInfoNV.html)
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBuildPartitionedAccelerationStructureInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub input: VkPartitionedAccelerationStructureInstancesInputNV<'a>,
  /// Optional: true
  pub srcAccelerationStructureData: VkDeviceAddress,
  pub dstAccelerationStructureData: VkDeviceAddress,
  /// No Auto-Validity
  pub scratchData: VkDeviceAddress,
  pub srcInfos: VkDeviceAddress,
  pub srcInfosCount: VkDeviceAddress,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Send for VkBuildPartitionedAccelerationStructureInfoNV<'a> {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
unsafe impl<'a> Sync for VkBuildPartitionedAccelerationStructureInfoNV<'a> {}
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
impl<'a> VkBuildPartitionedAccelerationStructureInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV,
    pNext: core::ptr::null_mut(),
    input: VkPartitionedAccelerationStructureInstancesInputNV::DEFAULT,
    srcAccelerationStructureData: 0,
    dstAccelerationStructureData: 0,
    scratchData: 0,
    srcInfos: 0,
    srcInfosCount: 0,
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
  pub const fn with_input(
    mut self,
    val: VkPartitionedAccelerationStructureInstancesInputNV<'a>,
  ) -> Self {
    self.input = val;
    self
  }
  #[inline]
  pub const fn with_srcAccelerationStructureData(mut self, val: VkDeviceAddress) -> Self {
    self.srcAccelerationStructureData = val;
    self
  }
  #[inline]
  pub const fn with_dstAccelerationStructureData(mut self, val: VkDeviceAddress) -> Self {
    self.dstAccelerationStructureData = val;
    self
  }
  #[inline]
  pub const fn with_scratchData(mut self, val: VkDeviceAddress) -> Self {
    self.scratchData = val;
    self
  }
  #[inline]
  pub const fn with_srcInfos(mut self, val: VkDeviceAddress) -> Self {
    self.srcInfos = val;
    self
  }
  #[inline]
  pub const fn with_srcInfosCount(mut self, val: VkDeviceAddress) -> Self {
    self.srcInfosCount = val;
    self
  }
  #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBuildPartitionedAccelerationStructureInfoNV<
    'root,
    T: VkPNextExtends<VkBuildPartitionedAccelerationStructureInfoNV<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDevicePerStageDescriptorSetFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePerStageDescriptorSetFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_per_stage_descriptor_set")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub perStageDescriptorSet: VkBool32,
  pub dynamicPipelineLayout: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_per_stage_descriptor_set")]
unsafe impl<'a> Send for VkPhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_per_stage_descriptor_set")]
unsafe impl<'a> Sync for VkPhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_per_stage_descriptor_set",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePerStageDescriptorSetFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_per_stage_descriptor_set",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePerStageDescriptorSetFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_per_stage_descriptor_set")]
impl<'a> VkPhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    perStageDescriptorSet: 0,
    dynamicPipelineLayout: 0,
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
  pub const fn with_perStageDescriptorSet(mut self, val: VkBool32) -> Self {
    self.perStageDescriptorSet = val;
    self
  }
  #[inline]
  pub const fn with_dynamicPipelineLayout(mut self, val: VkBool32) -> Self {
    self.dynamicPipelineLayout = val;
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
/// [VkPhysicalDevicePresentBarrierFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentBarrierFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_present_barrier")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePresentBarrierFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub presentBarrier: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_present_barrier")]
unsafe impl<'a> Send for VkPhysicalDevicePresentBarrierFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_present_barrier")]
unsafe impl<'a> Sync for VkPhysicalDevicePresentBarrierFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_present_barrier", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePresentBarrierFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_present_barrier", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePresentBarrierFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_present_barrier")]
impl<'a> VkPhysicalDevicePresentBarrierFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    presentBarrier: 0,
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
  pub const fn with_presentBarrier(mut self, val: VkBool32) -> Self {
    self.presentBarrier = val;
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
/// [VkSurfaceCapabilitiesPresentBarrierNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCapabilitiesPresentBarrierNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkSurfaceCapabilities2KHR.
#[cfg(feature = "VK_NV_present_barrier")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSurfaceCapabilitiesPresentBarrierNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_PRESENT_BARRIER_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub presentBarrierSupported: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_present_barrier")]
unsafe impl<'a> Send for VkSurfaceCapabilitiesPresentBarrierNV<'a> {}
#[cfg(feature = "VK_NV_present_barrier")]
unsafe impl<'a> Sync for VkSurfaceCapabilitiesPresentBarrierNV<'a> {}
#[cfg(all(
  feature = "VK_NV_present_barrier",
  feature = "VK_KHR_get_surface_capabilities2"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSurfaceCapabilities2KHR<'root>>
  for VkSurfaceCapabilitiesPresentBarrierNV<'child>
{
}
#[cfg(feature = "VK_NV_present_barrier")]
impl<'a> VkSurfaceCapabilitiesPresentBarrierNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SURFACE_CAPABILITIES_PRESENT_BARRIER_NV,
    pNext: core::ptr::null_mut(),
    presentBarrierSupported: 0,
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
  pub const fn with_presentBarrierSupported(mut self, val: VkBool32) -> Self {
    self.presentBarrierSupported = val;
    self
  }
  #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSurfaceCapabilities2KHR<
    'root,
    T: VkPNextExtends<VkSurfaceCapabilities2KHR<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkSwapchainPresentBarrierCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentBarrierCreateInfoNV.html)
///
/// **Extends:** VkSwapchainCreateInfoKHR.
#[cfg(feature = "VK_NV_present_barrier")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSwapchainPresentBarrierCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub presentBarrierEnable: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_present_barrier")]
unsafe impl<'a> Send for VkSwapchainPresentBarrierCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_present_barrier")]
unsafe impl<'a> Sync for VkSwapchainPresentBarrierCreateInfoNV<'a> {}
#[cfg(all(feature = "VK_NV_present_barrier", feature = "VK_KHR_swapchain"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSwapchainCreateInfoKHR<'root>>
  for VkSwapchainPresentBarrierCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_present_barrier")]
impl<'a> VkSwapchainPresentBarrierCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV,
    pNext: core::ptr::null_mut(),
    presentBarrierEnable: 0,
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
  pub const fn with_presentBarrierEnable(mut self, val: VkBool32) -> Self {
    self.presentBarrierEnable = val;
    self
  }
  #[cfg(feature = "VK_KHR_swapchain")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSwapchainCreateInfoKHR<
    'root,
    T: VkPNextExtends<VkSwapchainCreateInfoKHR<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkSetPresentConfigNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSetPresentConfigNV.html)
///
/// **Extends:** VkPresentInfoKHR.
#[cfg(feature = "VK_NV_present_metering")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSetPresentConfigNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_SET_PRESENT_CONFIG_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub numFramesPerBatch: u32,
  pub presentConfigFeedback: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_present_metering")]
unsafe impl<'a> Send for VkSetPresentConfigNV<'a> {}
#[cfg(feature = "VK_NV_present_metering")]
unsafe impl<'a> Sync for VkSetPresentConfigNV<'a> {}
#[cfg(all(feature = "VK_NV_present_metering", feature = "VK_KHR_swapchain"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPresentInfoKHR<'root>>
  for VkSetPresentConfigNV<'child>
{
}
#[cfg(feature = "VK_NV_present_metering")]
impl<'a> VkSetPresentConfigNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SET_PRESENT_CONFIG_NV,
    pNext: core::ptr::null(),
    numFramesPerBatch: 0,
    presentConfigFeedback: 0,
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
  pub const fn with_numFramesPerBatch(mut self, val: u32) -> Self {
    self.numFramesPerBatch = val;
    self
  }
  #[inline]
  pub const fn with_presentConfigFeedback(mut self, val: u32) -> Self {
    self.presentConfigFeedback = val;
    self
  }
  #[cfg(feature = "VK_KHR_swapchain")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPresentInfoKHR<
    'root,
    T: VkPNextExtends<VkPresentInfoKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDevicePresentMeteringFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentMeteringFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_present_metering")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePresentMeteringFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub presentMetering: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_present_metering")]
unsafe impl<'a> Send for VkPhysicalDevicePresentMeteringFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_present_metering")]
unsafe impl<'a> Sync for VkPhysicalDevicePresentMeteringFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_present_metering", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePresentMeteringFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_present_metering", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePresentMeteringFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_present_metering")]
impl<'a> VkPhysicalDevicePresentMeteringFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    presentMetering: 0,
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
  pub const fn with_presentMetering(mut self, val: VkBool32) -> Self {
    self.presentMetering = val;
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
/// [VkPushConstantBankInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPushConstantBankInfoNV.html)
///
/// **Extends:** VkDescriptorSetAndBindingMappingEXT, VkPushDataInfoEXT, VkPushConstantsInfo, VkIndirectCommandsLayoutTokenEXT.
#[cfg(feature = "VK_NV_push_constant_bank")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPushConstantBankInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PUSH_CONSTANT_BANK_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub bank: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_push_constant_bank")]
unsafe impl<'a> Send for VkPushConstantBankInfoNV<'a> {}
#[cfg(feature = "VK_NV_push_constant_bank")]
unsafe impl<'a> Sync for VkPushConstantBankInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_push_constant_bank",
  feature = "VK_EXT_descriptor_heap"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDescriptorSetAndBindingMappingEXT<'root>>
  for VkPushConstantBankInfoNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_push_constant_bank",
  feature = "VK_EXT_descriptor_heap"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPushDataInfoEXT<'root>>
  for VkPushConstantBankInfoNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_push_constant_bank",
  feature = "VK_COMPUTE_VERSION_1_4"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPushConstantsInfo<'root>>
  for VkPushConstantBankInfoNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_push_constant_bank",
  feature = "VK_EXT_device_generated_commands"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkIndirectCommandsLayoutTokenEXT<'root>>
  for VkPushConstantBankInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_push_constant_bank")]
impl<'a> VkPushConstantBankInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PUSH_CONSTANT_BANK_INFO_NV,
    pNext: core::ptr::null(),
    bank: 0,
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
  pub const fn with_bank(mut self, val: u32) -> Self {
    self.bank = val;
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDescriptorSetAndBindingMappingEXT<
    'root,
    T: VkPNextExtends<VkDescriptorSetAndBindingMappingEXT<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPushDataInfoEXT<
    'root,
    T: VkPNextExtends<VkPushDataInfoEXT<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPushConstantsInfo<
    'root,
    T: VkPNextExtends<VkPushConstantsInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkIndirectCommandsLayoutTokenEXT<
    'root,
    T: VkPNextExtends<VkIndirectCommandsLayoutTokenEXT<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDevicePushConstantBankFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePushConstantBankFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_push_constant_bank")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePushConstantBankFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub pushConstantBank: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_push_constant_bank")]
unsafe impl<'a> Send for VkPhysicalDevicePushConstantBankFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_push_constant_bank")]
unsafe impl<'a> Sync for VkPhysicalDevicePushConstantBankFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_push_constant_bank", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePushConstantBankFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_push_constant_bank", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePushConstantBankFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_push_constant_bank")]
impl<'a> VkPhysicalDevicePushConstantBankFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    pushConstantBank: 0,
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
  pub const fn with_pushConstantBank(mut self, val: VkBool32) -> Self {
    self.pushConstantBank = val;
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
/// [VkPhysicalDevicePushConstantBankPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePushConstantBankPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_push_constant_bank")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePushConstantBankPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxGraphicsPushConstantBanks: u32,
  /// Limit Type: [Max]
  pub maxComputePushConstantBanks: u32,
  /// Limit Type: [Max]
  pub maxGraphicsPushDataBanks: u32,
  /// Limit Type: [Max]
  pub maxComputePushDataBanks: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_push_constant_bank")]
unsafe impl<'a> Send for VkPhysicalDevicePushConstantBankPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_push_constant_bank")]
unsafe impl<'a> Sync for VkPhysicalDevicePushConstantBankPropertiesNV<'a> {}
#[cfg(all(feature = "VK_NV_push_constant_bank", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDevicePushConstantBankPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_push_constant_bank")]
impl<'a> VkPhysicalDevicePushConstantBankPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    maxGraphicsPushConstantBanks: 0,
    maxComputePushConstantBanks: 0,
    maxGraphicsPushDataBanks: 0,
    maxComputePushDataBanks: 0,
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
  pub const fn with_maxGraphicsPushConstantBanks(mut self, val: u32) -> Self {
    self.maxGraphicsPushConstantBanks = val;
    self
  }
  #[inline]
  pub const fn with_maxComputePushConstantBanks(mut self, val: u32) -> Self {
    self.maxComputePushConstantBanks = val;
    self
  }
  #[inline]
  pub const fn with_maxGraphicsPushDataBanks(mut self, val: u32) -> Self {
    self.maxGraphicsPushDataBanks = val;
    self
  }
  #[inline]
  pub const fn with_maxComputePushDataBanks(mut self, val: u32) -> Self {
    self.maxComputePushDataBanks = val;
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
/// [VkPhysicalDeviceRawAccessChainsFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRawAccessChainsFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_raw_access_chains")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRawAccessChainsFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderRawAccessChains: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_raw_access_chains")]
unsafe impl<'a> Send for VkPhysicalDeviceRawAccessChainsFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_raw_access_chains")]
unsafe impl<'a> Sync for VkPhysicalDeviceRawAccessChainsFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_raw_access_chains", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceRawAccessChainsFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_raw_access_chains", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceRawAccessChainsFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_raw_access_chains")]
impl<'a> VkPhysicalDeviceRawAccessChainsFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    shaderRawAccessChains: 0,
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
  pub const fn with_shaderRawAccessChains(mut self, val: VkBool32) -> Self {
    self.shaderRawAccessChains = val;
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
/// [VkGeometryFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagsNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
pub type VkGeometryFlagsNV = VkGeometryFlagBitsKHR;
/// [VkGeometryInstanceFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagsNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
pub type VkGeometryInstanceFlagsNV = VkGeometryInstanceFlagBitsKHR;
/// [VkBuildAccelerationStructureFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagsNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
pub type VkBuildAccelerationStructureFlagsNV = VkBuildAccelerationStructureFlagBitsKHR;
/// [VkAccelerationStructureNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkAccelerationStructureNV(pub *mut c_void);
#[cfg(feature = "VK_NV_ray_tracing")]
impl VkAccelerationStructureNV {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl Default for VkAccelerationStructureNV {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl Send for VkAccelerationStructureNV {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl Sync for VkAccelerationStructureNV {}
/// [VkRayTracingShaderGroupCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingShaderGroupCreateInfoNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRayTracingShaderGroupCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub type_: VkRayTracingShaderGroupTypeKHR,
  pub generalShader: u32,
  pub closestHitShader: u32,
  pub anyHitShader: u32,
  pub intersectionShader: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkRayTracingShaderGroupCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkRayTracingShaderGroupCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkRayTracingShaderGroupCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    type_: VkRayTracingShaderGroupTypeKHR(0),
    generalShader: 0,
    closestHitShader: 0,
    anyHitShader: 0,
    intersectionShader: 0,
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
  pub const fn with_type(mut self, val: VkRayTracingShaderGroupTypeKHR) -> Self {
    self.type_ = val;
    self
  }
  #[inline]
  pub const fn with_generalShader(mut self, val: u32) -> Self {
    self.generalShader = val;
    self
  }
  #[inline]
  pub const fn with_closestHitShader(mut self, val: u32) -> Self {
    self.closestHitShader = val;
    self
  }
  #[inline]
  pub const fn with_anyHitShader(mut self, val: u32) -> Self {
    self.anyHitShader = val;
    self
  }
  #[inline]
  pub const fn with_intersectionShader(mut self, val: u32) -> Self {
    self.intersectionShader = val;
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRayTracingShaderGroupCreateInfoNV<
    'root,
    T: VkPNextExtends<VkRayTracingShaderGroupCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkRayTracingPipelineCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingPipelineCreateInfoNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRayTracingPipelineCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true,  No Auto-Validity
  pub flags: VkPipelineCreateFlags,
  pub stageCount: u32,
  /// Length: stageCount
  pub pStages: *const VkPipelineShaderStageCreateInfo<'a>,
  pub groupCount: u32,
  /// Length: groupCount
  pub pGroups: *const VkRayTracingShaderGroupCreateInfoNV<'a>,
  pub maxRecursionDepth: u32,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// Optional: true
  pub layout: VkPipelineLayout,
  #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
  /// Optional: true
  pub layout: *mut c_void,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// Optional: true,  No Auto-Validity
  pub basePipelineHandle: VkPipeline,
  #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
  /// Optional: true,  No Auto-Validity
  pub basePipelineHandle: *mut c_void,
  pub basePipelineIndex: i32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkRayTracingPipelineCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkRayTracingPipelineCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkRayTracingPipelineCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RAY_TRACING_PIPELINE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    flags: VkPipelineCreateFlagBits(0),
    stageCount: 0,
    pStages: core::ptr::null(),
    groupCount: 0,
    pGroups: core::ptr::null(),
    maxRecursionDepth: 0,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    layout: VkPipelineLayout::DEFAULT,
    #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
    layout: core::ptr::null_mut(),
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    basePipelineHandle: VkPipeline::DEFAULT,
    #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
    basePipelineHandle: core::ptr::null_mut(),
    basePipelineIndex: 0,
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
  pub const fn with_flags(mut self, val: VkPipelineCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_stageCount(mut self, val: u32) -> Self {
    self.stageCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pStages(mut self, val: &'a [VkPipelineShaderStageCreateInfo<'a>]) -> Self {
    self.stageCount = val.len() as u32;
    self.pStages = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_groupCount(mut self, val: u32) -> Self {
    self.groupCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pGroups(mut self, val: &'a [VkRayTracingShaderGroupCreateInfoNV<'a>]) -> Self {
    self.groupCount = val.len() as u32;
    self.pGroups = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_maxRecursionDepth(mut self, val: u32) -> Self {
    self.maxRecursionDepth = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub const fn with_layout(mut self, val: VkPipelineLayout) -> Self {
    self.layout = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub const fn with_basePipelineHandle(mut self, val: VkPipeline) -> Self {
    self.basePipelineHandle = val;
    self
  }
  #[inline]
  pub const fn with_basePipelineIndex(mut self, val: i32) -> Self {
    self.basePipelineIndex = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineCreateFlags2CreateInfo<'child>(
    mut self,
    val: &'a VkPipelineCreateFlags2CreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineCreateFlags2CreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineCreationFeedbackCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineCreationFeedbackCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineCreationFeedbackCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineOfflineCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineOfflineCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineOfflineCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRayTracingPipelineCreateInfoNV<
    'root,
    T: VkPNextExtends<VkRayTracingPipelineCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkGeometryTrianglesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryTrianglesNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGeometryTrianglesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub vertexData: VkBuffer,
  pub vertexOffset: VkDeviceSize,
  pub vertexCount: u32,
  pub vertexStride: VkDeviceSize,
  pub vertexFormat: VkFormat,
  /// Optional: true
  pub indexData: VkBuffer,
  pub indexOffset: VkDeviceSize,
  pub indexCount: u32,
  pub indexType: VkIndexType,
  /// Optional: true
  pub transformData: VkBuffer,
  pub transformOffset: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkGeometryTrianglesNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkGeometryTrianglesNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkGeometryTrianglesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GEOMETRY_TRIANGLES_NV,
    pNext: core::ptr::null(),
    vertexData: VkBuffer::DEFAULT,
    vertexOffset: 0,
    vertexCount: 0,
    vertexStride: 0,
    vertexFormat: VkFormat(0),
    indexData: VkBuffer::DEFAULT,
    indexOffset: 0,
    indexCount: 0,
    indexType: VkIndexType(0),
    transformData: VkBuffer::DEFAULT,
    transformOffset: 0,
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
  pub const fn with_vertexData(mut self, val: VkBuffer) -> Self {
    self.vertexData = val;
    self
  }
  #[inline]
  pub const fn with_vertexOffset(mut self, val: VkDeviceSize) -> Self {
    self.vertexOffset = val;
    self
  }
  #[inline]
  pub const fn with_vertexCount(mut self, val: u32) -> Self {
    self.vertexCount = val;
    self
  }
  #[inline]
  pub const fn with_vertexStride(mut self, val: VkDeviceSize) -> Self {
    self.vertexStride = val;
    self
  }
  #[inline]
  pub const fn with_vertexFormat(mut self, val: VkFormat) -> Self {
    self.vertexFormat = val;
    self
  }
  #[inline]
  pub const fn with_indexData(mut self, val: VkBuffer) -> Self {
    self.indexData = val;
    self
  }
  #[inline]
  pub const fn with_indexOffset(mut self, val: VkDeviceSize) -> Self {
    self.indexOffset = val;
    self
  }
  #[inline]
  pub const fn with_indexCount(mut self, val: u32) -> Self {
    self.indexCount = val;
    self
  }
  #[inline]
  pub const fn with_indexType(mut self, val: VkIndexType) -> Self {
    self.indexType = val;
    self
  }
  #[inline]
  pub const fn with_transformData(mut self, val: VkBuffer) -> Self {
    self.transformData = val;
    self
  }
  #[inline]
  pub const fn with_transformOffset(mut self, val: VkDeviceSize) -> Self {
    self.transformOffset = val;
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGeometryTrianglesNV<
    'root,
    T: VkPNextExtends<VkGeometryTrianglesNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkGeometryAABBNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryAABBNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGeometryAABBNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub aabbData: VkBuffer,
  pub numAABBs: u32,
  pub stride: u32,
  pub offset: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkGeometryAABBNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkGeometryAABBNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkGeometryAABBNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GEOMETRY_AABB_NV,
    pNext: core::ptr::null(),
    aabbData: VkBuffer::DEFAULT,
    numAABBs: 0,
    stride: 0,
    offset: 0,
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
  pub const fn with_aabbData(mut self, val: VkBuffer) -> Self {
    self.aabbData = val;
    self
  }
  #[inline]
  pub const fn with_numAABBs(mut self, val: u32) -> Self {
    self.numAABBs = val;
    self
  }
  #[inline]
  pub const fn with_stride(mut self, val: u32) -> Self {
    self.stride = val;
    self
  }
  #[inline]
  pub const fn with_offset(mut self, val: VkDeviceSize) -> Self {
    self.offset = val;
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGeometryAABBNV<
    'root,
    T: VkPNextExtends<VkGeometryAABBNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkGeometryDataNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryDataNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGeometryDataNV<'a> {
  pub triangles: VkGeometryTrianglesNV<'a>,
  pub aabbs: VkGeometryAABBNV<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkGeometryDataNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkGeometryDataNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkGeometryDataNV<'a> {
  pub const DEFAULT: Self = Self {
    triangles: VkGeometryTrianglesNV::DEFAULT,
    aabbs: VkGeometryAABBNV::DEFAULT,
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_triangles(mut self, val: VkGeometryTrianglesNV<'a>) -> Self {
    self.triangles = val;
    self
  }
  #[inline]
  pub const fn with_aabbs(mut self, val: VkGeometryAABBNV<'a>) -> Self {
    self.aabbs = val;
    self
  }
}
/// [VkGeometryNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGeometryNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_GEOMETRY_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub geometryType: VkGeometryTypeKHR,
  pub geometry: VkGeometryDataNV<'a>,
  /// Optional: true
  pub flags: VkGeometryFlagsKHR,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkGeometryNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkGeometryNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkGeometryNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GEOMETRY_NV,
    pNext: core::ptr::null(),
    geometryType: VkGeometryTypeKHR(0),
    geometry: VkGeometryDataNV::DEFAULT,
    flags: VkGeometryFlagBitsKHR(0),
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
  pub const fn with_geometryType(mut self, val: VkGeometryTypeKHR) -> Self {
    self.geometryType = val;
    self
  }
  #[inline]
  pub const fn with_geometry(mut self, val: VkGeometryDataNV<'a>) -> Self {
    self.geometry = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkGeometryFlagsKHR) -> Self {
    self.flags = val;
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGeometryNV<'root, T: VkPNextExtends<VkGeometryNV<'root>>>(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkAccelerationStructureInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureInfoNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub type_: VkAccelerationStructureTypeNV,
  /// Optional: true
  pub flags: VkBuildAccelerationStructureFlagsKHR,
  /// Optional: true
  pub instanceCount: u32,
  /// Optional: true
  pub geometryCount: u32,
  /// Length: geometryCount
  pub pGeometries: *const VkGeometryNV<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkAccelerationStructureInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkAccelerationStructureInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkAccelerationStructureInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ACCELERATION_STRUCTURE_INFO_NV,
    pNext: core::ptr::null(),
    type_: VkAccelerationStructureTypeNV(0),
    flags: VkBuildAccelerationStructureFlagBitsKHR(0),
    instanceCount: 0,
    geometryCount: 0,
    pGeometries: core::ptr::null(),
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
  pub const fn with_type(mut self, val: VkAccelerationStructureTypeNV) -> Self {
    self.type_ = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkBuildAccelerationStructureFlagsKHR) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_instanceCount(mut self, val: u32) -> Self {
    self.instanceCount = val;
    self
  }
  #[inline]
  pub const fn with_geometryCount(mut self, val: u32) -> Self {
    self.geometryCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pGeometries(mut self, val: &'a [VkGeometryNV<'a>]) -> Self {
    self.geometryCount = val.len() as u32;
    self.pGeometries = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAccelerationStructureInfoNV<
    'root,
    T: VkPNextExtends<VkAccelerationStructureInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkAccelerationStructureCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateInfoNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub compactedSize: VkDeviceSize,
  pub info: VkAccelerationStructureInfoNV<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkAccelerationStructureCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkAccelerationStructureCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkAccelerationStructureCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ACCELERATION_STRUCTURE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    compactedSize: 0,
    info: VkAccelerationStructureInfoNV::DEFAULT,
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
  pub const fn with_compactedSize(mut self, val: VkDeviceSize) -> Self {
    self.compactedSize = val;
    self
  }
  #[inline]
  pub const fn with_info(mut self, val: VkAccelerationStructureInfoNV<'a>) -> Self {
    self.info = val;
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
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAccelerationStructureCreateInfoNV<
    'root,
    T: VkPNextExtends<VkAccelerationStructureCreateInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBindAccelerationStructureMemoryInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindAccelerationStructureMemoryInfoNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindAccelerationStructureMemoryInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub accelerationStructure: VkAccelerationStructureNV,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  /// Optional: true
  pub deviceIndexCount: u32,
  /// Length: deviceIndexCount
  pub pDeviceIndices: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkBindAccelerationStructureMemoryInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkBindAccelerationStructureMemoryInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkBindAccelerationStructureMemoryInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV,
    pNext: core::ptr::null(),
    accelerationStructure: VkAccelerationStructureNV::DEFAULT,
    memory: VkDeviceMemory::DEFAULT,
    memoryOffset: 0,
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
  pub const fn with_accelerationStructure(mut self, val: VkAccelerationStructureNV) -> Self {
    self.accelerationStructure = val;
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
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBindAccelerationStructureMemoryInfoNV<
    'root,
    T: VkPNextExtends<VkBindAccelerationStructureMemoryInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkWriteDescriptorSetAccelerationStructureNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSetAccelerationStructureNV.html)
///
/// **Extends:** VkWriteDescriptorSet.
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkWriteDescriptorSetAccelerationStructureNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub accelerationStructureCount: u32,
  /// Optional: pointer required, values optional if pointer not null,  Length: accelerationStructureCount
  pub pAccelerationStructures: *const VkAccelerationStructureNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkWriteDescriptorSetAccelerationStructureNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkWriteDescriptorSetAccelerationStructureNV<'a> {}
#[cfg(all(feature = "VK_NV_ray_tracing", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkWriteDescriptorSet<'root>>
  for VkWriteDescriptorSetAccelerationStructureNV<'child>
{
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkWriteDescriptorSetAccelerationStructureNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV,
    pNext: core::ptr::null(),
    accelerationStructureCount: 0,
    pAccelerationStructures: core::ptr::null(),
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
  pub const fn with_accelerationStructureCount(mut self, val: u32) -> Self {
    self.accelerationStructureCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAccelerationStructures(
    mut self,
    val: &'a [VkAccelerationStructureNV],
  ) -> Self {
    self.accelerationStructureCount = val.len() as u32;
    self.pAccelerationStructures = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkWriteDescriptorSet<
    'root,
    T: VkPNextExtends<VkWriteDescriptorSet<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkAccelerationStructureMemoryRequirementsInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMemoryRequirementsInfoNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureMemoryRequirementsInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub type_: VkAccelerationStructureMemoryRequirementsTypeNV,
  pub accelerationStructure: VkAccelerationStructureNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkAccelerationStructureMemoryRequirementsInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkAccelerationStructureMemoryRequirementsInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkAccelerationStructureMemoryRequirementsInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV,
    pNext: core::ptr::null(),
    type_: VkAccelerationStructureMemoryRequirementsTypeNV(0),
    accelerationStructure: VkAccelerationStructureNV::DEFAULT,
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
  pub const fn with_type(mut self, val: VkAccelerationStructureMemoryRequirementsTypeNV) -> Self {
    self.type_ = val;
    self
  }
  #[inline]
  pub const fn with_accelerationStructure(mut self, val: VkAccelerationStructureNV) -> Self {
    self.accelerationStructure = val;
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAccelerationStructureMemoryRequirementsInfoNV<
    'root,
    T: VkPNextExtends<VkAccelerationStructureMemoryRequirementsInfoNV<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceRayTracingPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRayTracingPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Exact]
  pub shaderGroupHandleSize: u32,
  /// Limit Type: [Max]
  pub maxRecursionDepth: u32,
  /// Limit Type: [Max]
  pub maxShaderGroupStride: u32,
  /// Limit Type: [Exact]
  pub shaderGroupBaseAlignment: u32,
  /// Limit Type: [Max]
  pub maxGeometryCount: u64,
  /// Limit Type: [Max]
  pub maxInstanceCount: u64,
  /// Limit Type: [Max]
  pub maxTriangleCount: u64,
  /// Limit Type: [Max]
  pub maxDescriptorSetAccelerationStructures: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Send for VkPhysicalDeviceRayTracingPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'a> Sync for VkPhysicalDeviceRayTracingPropertiesNV<'a> {}
#[cfg(all(feature = "VK_NV_ray_tracing", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceRayTracingPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'a> VkPhysicalDeviceRayTracingPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    shaderGroupHandleSize: 0,
    maxRecursionDepth: 0,
    maxShaderGroupStride: 0,
    shaderGroupBaseAlignment: 0,
    maxGeometryCount: 0,
    maxInstanceCount: 0,
    maxTriangleCount: 0,
    maxDescriptorSetAccelerationStructures: 0,
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
  pub const fn with_shaderGroupHandleSize(mut self, val: u32) -> Self {
    self.shaderGroupHandleSize = val;
    self
  }
  #[inline]
  pub const fn with_maxRecursionDepth(mut self, val: u32) -> Self {
    self.maxRecursionDepth = val;
    self
  }
  #[inline]
  pub const fn with_maxShaderGroupStride(mut self, val: u32) -> Self {
    self.maxShaderGroupStride = val;
    self
  }
  #[inline]
  pub const fn with_shaderGroupBaseAlignment(mut self, val: u32) -> Self {
    self.shaderGroupBaseAlignment = val;
    self
  }
  #[inline]
  pub const fn with_maxGeometryCount(mut self, val: u64) -> Self {
    self.maxGeometryCount = val;
    self
  }
  #[inline]
  pub const fn with_maxInstanceCount(mut self, val: u64) -> Self {
    self.maxInstanceCount = val;
    self
  }
  #[inline]
  pub const fn with_maxTriangleCount(mut self, val: u64) -> Self {
    self.maxTriangleCount = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetAccelerationStructures(mut self, val: u32) -> Self {
    self.maxDescriptorSetAccelerationStructures = val;
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
/// [VkAabbPositionsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAabbPositionsNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAabbPositionsNV {
  pub minX: f32,
  pub minY: f32,
  pub minZ: f32,
  pub maxX: f32,
  pub maxY: f32,
  pub maxZ: f32,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl Send for VkAabbPositionsNV {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl Sync for VkAabbPositionsNV {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl VkAabbPositionsNV {
  pub const DEFAULT: Self = Self {
    minX: 0.0f32,
    minY: 0.0f32,
    minZ: 0.0f32,
    maxX: 0.0f32,
    maxY: 0.0f32,
    maxZ: 0.0f32,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_minX(mut self, val: f32) -> Self {
    self.minX = val;
    self
  }
  #[inline]
  pub const fn with_minY(mut self, val: f32) -> Self {
    self.minY = val;
    self
  }
  #[inline]
  pub const fn with_minZ(mut self, val: f32) -> Self {
    self.minZ = val;
    self
  }
  #[inline]
  pub const fn with_maxX(mut self, val: f32) -> Self {
    self.maxX = val;
    self
  }
  #[inline]
  pub const fn with_maxY(mut self, val: f32) -> Self {
    self.maxY = val;
    self
  }
  #[inline]
  pub const fn with_maxZ(mut self, val: f32) -> Self {
    self.maxZ = val;
    self
  }
}
/// [VkTransformMatrixNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkTransformMatrixNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTransformMatrixNV {
  pub matrix: [f32; 3],
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl Send for VkTransformMatrixNV {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl Sync for VkTransformMatrixNV {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl VkTransformMatrixNV {
  pub const DEFAULT: Self = Self {
    matrix: [0.0f32; 3],
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_matrix(mut self, val: [f32; 3]) -> Self {
    self.matrix = val;
    self
  }
}
/// [VkAccelerationStructureInstanceNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureInstanceNV.html)
#[cfg(feature = "VK_NV_ray_tracing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureInstanceNV {
  pub transform: VkTransformMatrixNV,
  pub instanceCustomIndex: u32,
  pub mask: u32,
  pub instanceShaderBindingTableRecordOffset: u32,
  /// Optional: true
  pub flags: VkGeometryInstanceFlagsNV,
  pub accelerationStructureReference: u64,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl Send for VkAccelerationStructureInstanceNV {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl Sync for VkAccelerationStructureInstanceNV {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl VkAccelerationStructureInstanceNV {
  pub const DEFAULT: Self = Self {
    transform: VkTransformMatrixNV::DEFAULT,
    instanceCustomIndex: 0,
    mask: 0,
    instanceShaderBindingTableRecordOffset: 0,
    flags: VkGeometryInstanceFlagBitsKHR(0),
    accelerationStructureReference: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_transform(mut self, val: VkTransformMatrixNV) -> Self {
    self.transform = val;
    self
  }
  #[inline]
  pub const fn with_instanceCustomIndex(mut self, val: u32) -> Self {
    self.instanceCustomIndex = val;
    self
  }
  #[inline]
  pub const fn with_mask(mut self, val: u32) -> Self {
    self.mask = val;
    self
  }
  #[inline]
  pub const fn with_instanceShaderBindingTableRecordOffset(mut self, val: u32) -> Self {
    self.instanceShaderBindingTableRecordOffset = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkGeometryInstanceFlagsNV) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_accelerationStructureReference(mut self, val: u64) -> Self {
    self.accelerationStructureReference = val;
    self
  }
}
/// [VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub rayTracingInvocationReorder: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
unsafe impl<'a> Send for VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
unsafe impl<'a> Sync for VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_ray_tracing_invocation_reorder",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_ray_tracing_invocation_reorder",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
impl<'a> VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    rayTracingInvocationReorder: 0,
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
  pub const fn with_rayTracingInvocationReorder(mut self, val: VkBool32) -> Self {
    self.rayTracingInvocationReorder = val;
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
/// [VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  /// Limit Type: [Noauto]
  pub rayTracingInvocationReorderReorderingHint: VkRayTracingInvocationReorderModeEXT,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
unsafe impl<'a> Send for VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
unsafe impl<'a> Sync for VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_ray_tracing_invocation_reorder",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
impl<'a> VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    rayTracingInvocationReorderReorderingHint: VkRayTracingInvocationReorderModeEXT(0),
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
  pub const fn with_rayTracingInvocationReorderReorderingHint(
    mut self,
    val: VkRayTracingInvocationReorderModeEXT,
  ) -> Self {
    self.rayTracingInvocationReorderReorderingHint = val;
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
/// [VkAccelerationStructureGeometryLinearSweptSpheresDataNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometryLinearSweptSpheresDataNV.html)
///
/// **Extends:** VkAccelerationStructureGeometryKHR.
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureGeometryLinearSweptSpheresDataNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub vertexFormat: VkFormat,
  pub vertexData: VkDeviceOrHostAddressConstKHR<'a>,
  pub vertexStride: VkDeviceSize,
  pub radiusFormat: VkFormat,
  pub radiusData: VkDeviceOrHostAddressConstKHR<'a>,
  pub radiusStride: VkDeviceSize,
  pub indexType: VkIndexType,
  pub indexData: VkDeviceOrHostAddressConstKHR<'a>,
  pub indexStride: VkDeviceSize,
  pub indexingMode: VkRayTracingLssIndexingModeNV,
  pub endCapsMode: VkRayTracingLssPrimitiveEndCapsModeNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
unsafe impl<'a> Send for VkAccelerationStructureGeometryLinearSweptSpheresDataNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
unsafe impl<'a> Sync for VkAccelerationStructureGeometryLinearSweptSpheresDataNV<'a> {}
#[cfg(all(
  feature = "VK_NV_ray_tracing_linear_swept_spheres",
  feature = "VK_KHR_acceleration_structure"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAccelerationStructureGeometryKHR<'root>>
  for VkAccelerationStructureGeometryLinearSweptSpheresDataNV<'child>
{
}
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
impl<'a> VkAccelerationStructureGeometryLinearSweptSpheresDataNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV,
    pNext: core::ptr::null(),
    vertexFormat: VkFormat(0),
    vertexData: VkDeviceOrHostAddressConstKHR::DEFAULT,
    vertexStride: 0,
    radiusFormat: VkFormat(0),
    radiusData: VkDeviceOrHostAddressConstKHR::DEFAULT,
    radiusStride: 0,
    indexType: VkIndexType(0),
    indexData: VkDeviceOrHostAddressConstKHR::DEFAULT,
    indexStride: 0,
    indexingMode: VkRayTracingLssIndexingModeNV(0),
    endCapsMode: VkRayTracingLssPrimitiveEndCapsModeNV(0),
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
  pub const fn with_vertexFormat(mut self, val: VkFormat) -> Self {
    self.vertexFormat = val;
    self
  }
  #[inline]
  pub const fn with_vertexData(mut self, val: VkDeviceOrHostAddressConstKHR<'a>) -> Self {
    self.vertexData = val;
    self
  }
  #[inline]
  pub const fn with_vertexStride(mut self, val: VkDeviceSize) -> Self {
    self.vertexStride = val;
    self
  }
  #[inline]
  pub const fn with_radiusFormat(mut self, val: VkFormat) -> Self {
    self.radiusFormat = val;
    self
  }
  #[inline]
  pub const fn with_radiusData(mut self, val: VkDeviceOrHostAddressConstKHR<'a>) -> Self {
    self.radiusData = val;
    self
  }
  #[inline]
  pub const fn with_radiusStride(mut self, val: VkDeviceSize) -> Self {
    self.radiusStride = val;
    self
  }
  #[inline]
  pub const fn with_indexType(mut self, val: VkIndexType) -> Self {
    self.indexType = val;
    self
  }
  #[inline]
  pub const fn with_indexData(mut self, val: VkDeviceOrHostAddressConstKHR<'a>) -> Self {
    self.indexData = val;
    self
  }
  #[inline]
  pub const fn with_indexStride(mut self, val: VkDeviceSize) -> Self {
    self.indexStride = val;
    self
  }
  #[inline]
  pub const fn with_indexingMode(mut self, val: VkRayTracingLssIndexingModeNV) -> Self {
    self.indexingMode = val;
    self
  }
  #[inline]
  pub const fn with_endCapsMode(mut self, val: VkRayTracingLssPrimitiveEndCapsModeNV) -> Self {
    self.endCapsMode = val;
    self
  }
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAccelerationStructureGeometryKHR<
    'root,
    T: VkPNextExtends<VkAccelerationStructureGeometryKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkAccelerationStructureGeometrySpheresDataNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometrySpheresDataNV.html)
///
/// **Extends:** VkAccelerationStructureGeometryKHR.
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureGeometrySpheresDataNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub vertexFormat: VkFormat,
  pub vertexData: VkDeviceOrHostAddressConstKHR<'a>,
  pub vertexStride: VkDeviceSize,
  pub radiusFormat: VkFormat,
  pub radiusData: VkDeviceOrHostAddressConstKHR<'a>,
  pub radiusStride: VkDeviceSize,
  pub indexType: VkIndexType,
  pub indexData: VkDeviceOrHostAddressConstKHR<'a>,
  pub indexStride: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
unsafe impl<'a> Send for VkAccelerationStructureGeometrySpheresDataNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
unsafe impl<'a> Sync for VkAccelerationStructureGeometrySpheresDataNV<'a> {}
#[cfg(all(
  feature = "VK_NV_ray_tracing_linear_swept_spheres",
  feature = "VK_KHR_acceleration_structure"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAccelerationStructureGeometryKHR<'root>>
  for VkAccelerationStructureGeometrySpheresDataNV<'child>
{
}
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
impl<'a> VkAccelerationStructureGeometrySpheresDataNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV,
    pNext: core::ptr::null(),
    vertexFormat: VkFormat(0),
    vertexData: VkDeviceOrHostAddressConstKHR::DEFAULT,
    vertexStride: 0,
    radiusFormat: VkFormat(0),
    radiusData: VkDeviceOrHostAddressConstKHR::DEFAULT,
    radiusStride: 0,
    indexType: VkIndexType(0),
    indexData: VkDeviceOrHostAddressConstKHR::DEFAULT,
    indexStride: 0,
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
  pub const fn with_vertexFormat(mut self, val: VkFormat) -> Self {
    self.vertexFormat = val;
    self
  }
  #[inline]
  pub const fn with_vertexData(mut self, val: VkDeviceOrHostAddressConstKHR<'a>) -> Self {
    self.vertexData = val;
    self
  }
  #[inline]
  pub const fn with_vertexStride(mut self, val: VkDeviceSize) -> Self {
    self.vertexStride = val;
    self
  }
  #[inline]
  pub const fn with_radiusFormat(mut self, val: VkFormat) -> Self {
    self.radiusFormat = val;
    self
  }
  #[inline]
  pub const fn with_radiusData(mut self, val: VkDeviceOrHostAddressConstKHR<'a>) -> Self {
    self.radiusData = val;
    self
  }
  #[inline]
  pub const fn with_radiusStride(mut self, val: VkDeviceSize) -> Self {
    self.radiusStride = val;
    self
  }
  #[inline]
  pub const fn with_indexType(mut self, val: VkIndexType) -> Self {
    self.indexType = val;
    self
  }
  #[inline]
  pub const fn with_indexData(mut self, val: VkDeviceOrHostAddressConstKHR<'a>) -> Self {
    self.indexData = val;
    self
  }
  #[inline]
  pub const fn with_indexStride(mut self, val: VkDeviceSize) -> Self {
    self.indexStride = val;
    self
  }
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAccelerationStructureGeometryKHR<
    'root,
    T: VkPNextExtends<VkAccelerationStructureGeometryKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub spheres: VkBool32,
  pub linearSweptSpheres: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
unsafe impl<'a> Send for VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
unsafe impl<'a> Sync for VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_ray_tracing_linear_swept_spheres",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_ray_tracing_linear_swept_spheres",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
impl<'a> VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    spheres: 0,
    linearSweptSpheres: 0,
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
  pub const fn with_spheres(mut self, val: VkBool32) -> Self {
    self.spheres = val;
    self
  }
  #[inline]
  pub const fn with_linearSweptSpheres(mut self, val: VkBool32) -> Self {
    self.linearSweptSpheres = val;
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
/// [VkAccelerationStructureMotionInfoFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInfoFlagsNV.html)
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
pub type VkAccelerationStructureMotionInfoFlagsNV = VkFlags;
/// [VkAccelerationStructureMotionInstanceFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceFlagsNV.html)
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
pub type VkAccelerationStructureMotionInstanceFlagsNV = VkFlags;
/// [VkPhysicalDeviceRayTracingMotionBlurFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingMotionBlurFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub rayTracingMotionBlur: VkBool32,
  pub rayTracingMotionBlurPipelineTraceRaysIndirect: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl<'a> Send for VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl<'a> Sync for VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_ray_tracing_motion_blur",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_ray_tracing_motion_blur",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl<'a> VkPhysicalDeviceRayTracingMotionBlurFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    rayTracingMotionBlur: 0,
    rayTracingMotionBlurPipelineTraceRaysIndirect: 0,
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
  pub const fn with_rayTracingMotionBlur(mut self, val: VkBool32) -> Self {
    self.rayTracingMotionBlur = val;
    self
  }
  #[inline]
  pub const fn with_rayTracingMotionBlurPipelineTraceRaysIndirect(mut self, val: VkBool32) -> Self {
    self.rayTracingMotionBlurPipelineTraceRaysIndirect = val;
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
/// [VkAccelerationStructureGeometryMotionTrianglesDataNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometryMotionTrianglesDataNV.html)
///
/// **Extends:** VkAccelerationStructureGeometryTrianglesDataKHR.
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureGeometryMotionTrianglesDataNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// No Auto-Validity
  pub vertexData: VkDeviceOrHostAddressConstKHR<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl<'a> Send for VkAccelerationStructureGeometryMotionTrianglesDataNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl<'a> Sync for VkAccelerationStructureGeometryMotionTrianglesDataNV<'a> {}
#[cfg(all(
  feature = "VK_NV_ray_tracing_motion_blur",
  feature = "VK_KHR_acceleration_structure"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAccelerationStructureGeometryTrianglesDataKHR<'root>>
  for VkAccelerationStructureGeometryMotionTrianglesDataNV<'child>
{
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl<'a> VkAccelerationStructureGeometryMotionTrianglesDataNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV,
    pNext: core::ptr::null(),
    vertexData: VkDeviceOrHostAddressConstKHR::DEFAULT,
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
  pub const fn with_vertexData(mut self, val: VkDeviceOrHostAddressConstKHR<'a>) -> Self {
    self.vertexData = val;
    self
  }
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAccelerationStructureGeometryTrianglesDataKHR<
    'root,
    T: VkPNextExtends<VkAccelerationStructureGeometryTrianglesDataKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkAccelerationStructureMotionInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInfoNV.html)
///
/// **Extends:** VkAccelerationStructureCreateInfoKHR.
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureMotionInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub maxInstances: u32,
  /// Optional: true
  pub flags: VkAccelerationStructureMotionInfoFlagsNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl<'a> Send for VkAccelerationStructureMotionInfoNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl<'a> Sync for VkAccelerationStructureMotionInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_ray_tracing_motion_blur",
  feature = "VK_KHR_acceleration_structure"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAccelerationStructureCreateInfoKHR<'root>>
  for VkAccelerationStructureMotionInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl<'a> VkAccelerationStructureMotionInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ACCELERATION_STRUCTURE_MOTION_INFO_NV,
    pNext: core::ptr::null(),
    maxInstances: 0,
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
  pub const fn with_maxInstances(mut self, val: u32) -> Self {
    self.maxInstances = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkAccelerationStructureMotionInfoFlagsNV) -> Self {
    self.flags = val;
    self
  }
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAccelerationStructureCreateInfoKHR<
    'root,
    T: VkPNextExtends<VkAccelerationStructureCreateInfoKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSRTDataNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkSRTDataNV.html)
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSRTDataNV {
  pub sx: f32,
  pub a: f32,
  pub b: f32,
  pub pvx: f32,
  pub sy: f32,
  pub c: f32,
  pub pvy: f32,
  pub sz: f32,
  pub pvz: f32,
  pub qx: f32,
  pub qy: f32,
  pub qz: f32,
  pub qw: f32,
  pub tx: f32,
  pub ty: f32,
  pub tz: f32,
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl Send for VkSRTDataNV {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl Sync for VkSRTDataNV {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl VkSRTDataNV {
  pub const DEFAULT: Self = Self {
    sx: 0.0f32,
    a: 0.0f32,
    b: 0.0f32,
    pvx: 0.0f32,
    sy: 0.0f32,
    c: 0.0f32,
    pvy: 0.0f32,
    sz: 0.0f32,
    pvz: 0.0f32,
    qx: 0.0f32,
    qy: 0.0f32,
    qz: 0.0f32,
    qw: 0.0f32,
    tx: 0.0f32,
    ty: 0.0f32,
    tz: 0.0f32,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_sx(mut self, val: f32) -> Self {
    self.sx = val;
    self
  }
  #[inline]
  pub const fn with_a(mut self, val: f32) -> Self {
    self.a = val;
    self
  }
  #[inline]
  pub const fn with_b(mut self, val: f32) -> Self {
    self.b = val;
    self
  }
  #[inline]
  pub const fn with_pvx(mut self, val: f32) -> Self {
    self.pvx = val;
    self
  }
  #[inline]
  pub const fn with_sy(mut self, val: f32) -> Self {
    self.sy = val;
    self
  }
  #[inline]
  pub const fn with_c(mut self, val: f32) -> Self {
    self.c = val;
    self
  }
  #[inline]
  pub const fn with_pvy(mut self, val: f32) -> Self {
    self.pvy = val;
    self
  }
  #[inline]
  pub const fn with_sz(mut self, val: f32) -> Self {
    self.sz = val;
    self
  }
  #[inline]
  pub const fn with_pvz(mut self, val: f32) -> Self {
    self.pvz = val;
    self
  }
  #[inline]
  pub const fn with_qx(mut self, val: f32) -> Self {
    self.qx = val;
    self
  }
  #[inline]
  pub const fn with_qy(mut self, val: f32) -> Self {
    self.qy = val;
    self
  }
  #[inline]
  pub const fn with_qz(mut self, val: f32) -> Self {
    self.qz = val;
    self
  }
  #[inline]
  pub const fn with_qw(mut self, val: f32) -> Self {
    self.qw = val;
    self
  }
  #[inline]
  pub const fn with_tx(mut self, val: f32) -> Self {
    self.tx = val;
    self
  }
  #[inline]
  pub const fn with_ty(mut self, val: f32) -> Self {
    self.ty = val;
    self
  }
  #[inline]
  pub const fn with_tz(mut self, val: f32) -> Self {
    self.tz = val;
    self
  }
}
/// [VkAccelerationStructureSRTMotionInstanceNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureSRTMotionInstanceNV.html)
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureSRTMotionInstanceNV {
  pub transformT0: VkSRTDataNV,
  pub transformT1: VkSRTDataNV,
  pub instanceCustomIndex: u32,
  pub mask: u32,
  pub instanceShaderBindingTableRecordOffset: u32,
  /// Optional: true
  pub flags: VkGeometryInstanceFlagsKHR,
  pub accelerationStructureReference: u64,
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl Send for VkAccelerationStructureSRTMotionInstanceNV {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl Sync for VkAccelerationStructureSRTMotionInstanceNV {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl VkAccelerationStructureSRTMotionInstanceNV {
  pub const DEFAULT: Self = Self {
    transformT0: VkSRTDataNV::DEFAULT,
    transformT1: VkSRTDataNV::DEFAULT,
    instanceCustomIndex: 0,
    mask: 0,
    instanceShaderBindingTableRecordOffset: 0,
    flags: VkGeometryInstanceFlagBitsKHR(0),
    accelerationStructureReference: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_transformT0(mut self, val: VkSRTDataNV) -> Self {
    self.transformT0 = val;
    self
  }
  #[inline]
  pub const fn with_transformT1(mut self, val: VkSRTDataNV) -> Self {
    self.transformT1 = val;
    self
  }
  #[inline]
  pub const fn with_instanceCustomIndex(mut self, val: u32) -> Self {
    self.instanceCustomIndex = val;
    self
  }
  #[inline]
  pub const fn with_mask(mut self, val: u32) -> Self {
    self.mask = val;
    self
  }
  #[inline]
  pub const fn with_instanceShaderBindingTableRecordOffset(mut self, val: u32) -> Self {
    self.instanceShaderBindingTableRecordOffset = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkGeometryInstanceFlagsKHR) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_accelerationStructureReference(mut self, val: u64) -> Self {
    self.accelerationStructureReference = val;
    self
  }
}
/// [VkAccelerationStructureMatrixMotionInstanceNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMatrixMotionInstanceNV.html)
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureMatrixMotionInstanceNV {
  pub transformT0: VkTransformMatrixKHR,
  pub transformT1: VkTransformMatrixKHR,
  pub instanceCustomIndex: u32,
  pub mask: u32,
  pub instanceShaderBindingTableRecordOffset: u32,
  /// Optional: true
  pub flags: VkGeometryInstanceFlagsKHR,
  pub accelerationStructureReference: u64,
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl Send for VkAccelerationStructureMatrixMotionInstanceNV {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl Sync for VkAccelerationStructureMatrixMotionInstanceNV {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl VkAccelerationStructureMatrixMotionInstanceNV {
  pub const DEFAULT: Self = Self {
    transformT0: VkTransformMatrixKHR::DEFAULT,
    transformT1: VkTransformMatrixKHR::DEFAULT,
    instanceCustomIndex: 0,
    mask: 0,
    instanceShaderBindingTableRecordOffset: 0,
    flags: VkGeometryInstanceFlagBitsKHR(0),
    accelerationStructureReference: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_transformT0(mut self, val: VkTransformMatrixKHR) -> Self {
    self.transformT0 = val;
    self
  }
  #[inline]
  pub const fn with_transformT1(mut self, val: VkTransformMatrixKHR) -> Self {
    self.transformT1 = val;
    self
  }
  #[inline]
  pub const fn with_instanceCustomIndex(mut self, val: u32) -> Self {
    self.instanceCustomIndex = val;
    self
  }
  #[inline]
  pub const fn with_mask(mut self, val: u32) -> Self {
    self.mask = val;
    self
  }
  #[inline]
  pub const fn with_instanceShaderBindingTableRecordOffset(mut self, val: u32) -> Self {
    self.instanceShaderBindingTableRecordOffset = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkGeometryInstanceFlagsKHR) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_accelerationStructureReference(mut self, val: u64) -> Self {
    self.accelerationStructureReference = val;
    self
  }
}
/// [VkAccelerationStructureMotionInstanceDataNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceDataNV.html)
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkAccelerationStructureMotionInstanceDataNV {
  pub staticInstance: VkAccelerationStructureInstanceKHR,
  pub matrixMotionInstance: VkAccelerationStructureMatrixMotionInstanceNV,
  pub srtMotionInstance: VkAccelerationStructureSRTMotionInstanceNV,
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl Send for VkAccelerationStructureMotionInstanceDataNV {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl Sync for VkAccelerationStructureMotionInstanceDataNV {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl VkAccelerationStructureMotionInstanceDataNV {
  pub const DEFAULT: Self = unsafe {
    Self {
      staticInstance: core::mem::zeroed::<VkAccelerationStructureInstanceKHR>(),
    }
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl core::fmt::Debug for VkAccelerationStructureMotionInstanceDataNV {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("VkAccelerationStructureMotionInstanceDataNV")
      .field("staticInstance", unsafe { &self.staticInstance })
      .finish()
  }
}
/// [VkAccelerationStructureMotionInstanceNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceNV.html)
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAccelerationStructureMotionInstanceNV {
  pub type_: VkAccelerationStructureMotionInstanceTypeNV,
  /// Optional: true
  pub flags: VkAccelerationStructureMotionInstanceFlagsNV,
  pub data: VkAccelerationStructureMotionInstanceDataNV,
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl Send for VkAccelerationStructureMotionInstanceNV {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
unsafe impl Sync for VkAccelerationStructureMotionInstanceNV {}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl VkAccelerationStructureMotionInstanceNV {
  pub const DEFAULT: Self = Self {
    type_: VkAccelerationStructureMotionInstanceTypeNV(0),
    flags: 0,
    data: VkAccelerationStructureMotionInstanceDataNV::DEFAULT,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_type(mut self, val: VkAccelerationStructureMotionInstanceTypeNV) -> Self {
    self.type_ = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkAccelerationStructureMotionInstanceFlagsNV) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_data(mut self, val: VkAccelerationStructureMotionInstanceDataNV) -> Self {
    self.data = val;
    self
  }
}
/// [VkPhysicalDeviceRayTracingValidationFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRayTracingValidationFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_ray_tracing_validation")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRayTracingValidationFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub rayTracingValidation: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_ray_tracing_validation")]
unsafe impl<'a> Send for VkPhysicalDeviceRayTracingValidationFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_ray_tracing_validation")]
unsafe impl<'a> Sync for VkPhysicalDeviceRayTracingValidationFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_ray_tracing_validation",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceRayTracingValidationFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_ray_tracing_validation",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceRayTracingValidationFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_ray_tracing_validation")]
impl<'a> VkPhysicalDeviceRayTracingValidationFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    rayTracingValidation: 0,
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
  pub const fn with_rayTracingValidation(mut self, val: VkBool32) -> Self {
    self.rayTracingValidation = val;
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
/// [VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_representative_fragment_test")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub representativeFragmentTest: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_representative_fragment_test")]
unsafe impl<'a> Send for VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_representative_fragment_test")]
unsafe impl<'a> Sync for VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_representative_fragment_test",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_representative_fragment_test",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_representative_fragment_test")]
impl<'a> VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    representativeFragmentTest: 0,
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
  pub const fn with_representativeFragmentTest(mut self, val: VkBool32) -> Self {
    self.representativeFragmentTest = val;
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
/// [VkPipelineRepresentativeFragmentTestStateCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html)
///
/// **Extends:** VkGraphicsPipelineCreateInfo.
#[cfg(feature = "VK_NV_representative_fragment_test")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub representativeFragmentTestEnable: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_representative_fragment_test")]
unsafe impl<'a> Send for VkPipelineRepresentativeFragmentTestStateCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_representative_fragment_test")]
unsafe impl<'a> Sync for VkPipelineRepresentativeFragmentTestStateCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_representative_fragment_test",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkPipelineRepresentativeFragmentTestStateCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_representative_fragment_test")]
impl<'a> VkPipelineRepresentativeFragmentTestStateCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    representativeFragmentTestEnable: 0,
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
  pub const fn with_representativeFragmentTestEnable(mut self, val: VkBool32) -> Self {
    self.representativeFragmentTestEnable = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkGraphicsPipelineCreateInfo<
    'root,
    T: VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceExclusiveScissorFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExclusiveScissorFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_scissor_exclusive")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExclusiveScissorFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub exclusiveScissor: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_scissor_exclusive")]
unsafe impl<'a> Send for VkPhysicalDeviceExclusiveScissorFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_scissor_exclusive")]
unsafe impl<'a> Sync for VkPhysicalDeviceExclusiveScissorFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_scissor_exclusive", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceExclusiveScissorFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_scissor_exclusive", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceExclusiveScissorFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_scissor_exclusive")]
impl<'a> VkPhysicalDeviceExclusiveScissorFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    exclusiveScissor: 0,
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
  pub const fn with_exclusiveScissor(mut self, val: VkBool32) -> Self {
    self.exclusiveScissor = val;
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
/// [VkPipelineViewportExclusiveScissorStateCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html)
///
/// **Extends:** VkPipelineViewportStateCreateInfo.
#[cfg(feature = "VK_NV_scissor_exclusive")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineViewportExclusiveScissorStateCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub exclusiveScissorCount: u32,
  /// Length: exclusiveScissorCount,  No Auto-Validity
  pub pExclusiveScissors: *const VkRect2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_scissor_exclusive")]
unsafe impl<'a> Send for VkPipelineViewportExclusiveScissorStateCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_scissor_exclusive")]
unsafe impl<'a> Sync for VkPipelineViewportExclusiveScissorStateCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_scissor_exclusive",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineViewportStateCreateInfo<'root>>
  for VkPipelineViewportExclusiveScissorStateCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_scissor_exclusive")]
impl<'a> VkPipelineViewportExclusiveScissorStateCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    exclusiveScissorCount: 0,
    pExclusiveScissors: core::ptr::null(),
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
  pub const fn with_exclusiveScissorCount(mut self, val: u32) -> Self {
    self.exclusiveScissorCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pExclusiveScissors(mut self, val: &'a [VkRect2D]) -> Self {
    self.exclusiveScissorCount = val.len() as u32;
    self.pExclusiveScissors = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineViewportStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineViewportStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_shader_atomic_float16_vector")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderFloat16VectorAtomics: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_shader_atomic_float16_vector")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_shader_atomic_float16_vector")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_shader_atomic_float16_vector",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_shader_atomic_float16_vector",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_shader_atomic_float16_vector")]
impl<'a> VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    shaderFloat16VectorAtomics: 0,
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
  pub const fn with_shaderFloat16VectorAtomics(mut self, val: VkBool32) -> Self {
    self.shaderFloat16VectorAtomics = val;
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
/// [VkPhysicalDeviceShaderImageFootprintFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderImageFootprintFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_shader_image_footprint")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderImageFootprintFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub imageFootprint: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_shader_image_footprint")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderImageFootprintFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_shader_image_footprint")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderImageFootprintFeaturesNV<'a> {}
#[cfg(all(
  feature = "VK_NV_shader_image_footprint",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderImageFootprintFeaturesNV<'child>
{
}
#[cfg(all(
  feature = "VK_NV_shader_image_footprint",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderImageFootprintFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_shader_image_footprint")]
impl<'a> VkPhysicalDeviceShaderImageFootprintFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    imageFootprint: 0,
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
  pub const fn with_imageFootprint(mut self, val: VkBool32) -> Self {
    self.imageFootprint = val;
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
/// [VkPhysicalDeviceShaderSMBuiltinsPropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_shader_sm_builtins")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub shaderSMCount: u32,
  /// Limit Type: [Max]
  pub shaderWarpsPerSM: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_shader_sm_builtins")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderSMBuiltinsPropertiesNV<'a> {}
#[cfg(feature = "VK_NV_shader_sm_builtins")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderSMBuiltinsPropertiesNV<'a> {}
#[cfg(all(feature = "VK_NV_shader_sm_builtins", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceShaderSMBuiltinsPropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_shader_sm_builtins")]
impl<'a> VkPhysicalDeviceShaderSMBuiltinsPropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    shaderSMCount: 0,
    shaderWarpsPerSM: 0,
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
  pub const fn with_shaderSMCount(mut self, val: u32) -> Self {
    self.shaderSMCount = val;
    self
  }
  #[inline]
  pub const fn with_shaderWarpsPerSM(mut self, val: u32) -> Self {
    self.shaderWarpsPerSM = val;
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
/// [VkPhysicalDeviceShaderSMBuiltinsFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_shader_sm_builtins")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderSMBuiltins: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_shader_sm_builtins")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_shader_sm_builtins")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_shader_sm_builtins", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_shader_sm_builtins", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_shader_sm_builtins")]
impl<'a> VkPhysicalDeviceShaderSMBuiltinsFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    shaderSMBuiltins: 0,
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
  pub const fn with_shaderSMBuiltins(mut self, val: VkBool32) -> Self {
    self.shaderSMBuiltins = val;
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
/// [VkShadingRatePaletteNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkShadingRatePaletteNV.html)
#[cfg(feature = "VK_NV_shading_rate_image")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkShadingRatePaletteNV<'a> {
  pub shadingRatePaletteEntryCount: u32,
  /// Length: shadingRatePaletteEntryCount
  pub pShadingRatePaletteEntries: *const VkShadingRatePaletteEntryNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Send for VkShadingRatePaletteNV<'a> {}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Sync for VkShadingRatePaletteNV<'a> {}
#[cfg(feature = "VK_NV_shading_rate_image")]
impl<'a> VkShadingRatePaletteNV<'a> {
  pub const DEFAULT: Self = Self {
    shadingRatePaletteEntryCount: 0,
    pShadingRatePaletteEntries: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_shadingRatePaletteEntryCount(mut self, val: u32) -> Self {
    self.shadingRatePaletteEntryCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pShadingRatePaletteEntries(
    mut self,
    val: &'a [VkShadingRatePaletteEntryNV],
  ) -> Self {
    self.shadingRatePaletteEntryCount = val.len() as u32;
    self.pShadingRatePaletteEntries = val.as_ptr();
    self
  }
}
/// [VkPipelineViewportShadingRateImageStateCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportShadingRateImageStateCreateInfoNV.html)
///
/// **Extends:** VkPipelineViewportStateCreateInfo.
#[cfg(feature = "VK_NV_shading_rate_image")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineViewportShadingRateImageStateCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub shadingRateImageEnable: VkBool32,
  /// Optional: true
  pub viewportCount: u32,
  /// Length: viewportCount,  No Auto-Validity
  pub pShadingRatePalettes: *const VkShadingRatePaletteNV<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Send for VkPipelineViewportShadingRateImageStateCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Sync for VkPipelineViewportShadingRateImageStateCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_shading_rate_image",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineViewportStateCreateInfo<'root>>
  for VkPipelineViewportShadingRateImageStateCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_shading_rate_image")]
impl<'a> VkPipelineViewportShadingRateImageStateCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    shadingRateImageEnable: 0,
    viewportCount: 0,
    pShadingRatePalettes: core::ptr::null(),
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
  pub const fn with_shadingRateImageEnable(mut self, val: VkBool32) -> Self {
    self.shadingRateImageEnable = val;
    self
  }
  #[inline]
  pub const fn with_viewportCount(mut self, val: u32) -> Self {
    self.viewportCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pShadingRatePalettes(mut self, val: &'a [VkShadingRatePaletteNV<'a>]) -> Self {
    self.viewportCount = val.len() as u32;
    self.pShadingRatePalettes = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineViewportStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineViewportStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceShadingRateImageFeaturesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShadingRateImageFeaturesNV.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_NV_shading_rate_image")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShadingRateImageFeaturesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shadingRateImage: VkBool32,
  pub shadingRateCoarseSampleOrder: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Send for VkPhysicalDeviceShadingRateImageFeaturesNV<'a> {}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Sync for VkPhysicalDeviceShadingRateImageFeaturesNV<'a> {}
#[cfg(all(feature = "VK_NV_shading_rate_image", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShadingRateImageFeaturesNV<'child>
{
}
#[cfg(all(feature = "VK_NV_shading_rate_image", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShadingRateImageFeaturesNV<'child>
{
}
#[cfg(feature = "VK_NV_shading_rate_image")]
impl<'a> VkPhysicalDeviceShadingRateImageFeaturesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV,
    pNext: core::ptr::null_mut(),
    shadingRateImage: 0,
    shadingRateCoarseSampleOrder: 0,
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
  pub const fn with_shadingRateImage(mut self, val: VkBool32) -> Self {
    self.shadingRateImage = val;
    self
  }
  #[inline]
  pub const fn with_shadingRateCoarseSampleOrder(mut self, val: VkBool32) -> Self {
    self.shadingRateCoarseSampleOrder = val;
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
/// [VkPhysicalDeviceShadingRateImagePropertiesNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShadingRateImagePropertiesNV.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_NV_shading_rate_image")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShadingRateImagePropertiesNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Exact]
  pub shadingRateTexelSize: VkExtent2D,
  /// Limit Type: [Max]
  pub shadingRatePaletteSize: u32,
  /// Limit Type: [Max]
  pub shadingRateMaxCoarseSamples: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Send for VkPhysicalDeviceShadingRateImagePropertiesNV<'a> {}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Sync for VkPhysicalDeviceShadingRateImagePropertiesNV<'a> {}
#[cfg(all(feature = "VK_NV_shading_rate_image", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceShadingRateImagePropertiesNV<'child>
{
}
#[cfg(feature = "VK_NV_shading_rate_image")]
impl<'a> VkPhysicalDeviceShadingRateImagePropertiesNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV,
    pNext: core::ptr::null_mut(),
    shadingRateTexelSize: VkExtent2D::DEFAULT,
    shadingRatePaletteSize: 0,
    shadingRateMaxCoarseSamples: 0,
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
  pub const fn with_shadingRateTexelSize(mut self, val: VkExtent2D) -> Self {
    self.shadingRateTexelSize = val;
    self
  }
  #[inline]
  pub const fn with_shadingRatePaletteSize(mut self, val: u32) -> Self {
    self.shadingRatePaletteSize = val;
    self
  }
  #[inline]
  pub const fn with_shadingRateMaxCoarseSamples(mut self, val: u32) -> Self {
    self.shadingRateMaxCoarseSamples = val;
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
/// [VkCoarseSampleLocationNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCoarseSampleLocationNV.html)
#[cfg(feature = "VK_NV_shading_rate_image")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCoarseSampleLocationNV {
  pub pixelX: u32,
  pub pixelY: u32,
  pub sample: u32,
}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl Send for VkCoarseSampleLocationNV {}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl Sync for VkCoarseSampleLocationNV {}
#[cfg(feature = "VK_NV_shading_rate_image")]
impl VkCoarseSampleLocationNV {
  pub const DEFAULT: Self = Self {
    pixelX: 0,
    pixelY: 0,
    sample: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_pixelX(mut self, val: u32) -> Self {
    self.pixelX = val;
    self
  }
  #[inline]
  pub const fn with_pixelY(mut self, val: u32) -> Self {
    self.pixelY = val;
    self
  }
  #[inline]
  pub const fn with_sample(mut self, val: u32) -> Self {
    self.sample = val;
    self
  }
}
/// [VkCoarseSampleOrderCustomNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkCoarseSampleOrderCustomNV.html)
#[cfg(feature = "VK_NV_shading_rate_image")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCoarseSampleOrderCustomNV<'a> {
  pub shadingRate: VkShadingRatePaletteEntryNV,
  pub sampleCount: u32,
  pub sampleLocationCount: u32,
  /// Length: sampleLocationCount
  pub pSampleLocations: *const VkCoarseSampleLocationNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Send for VkCoarseSampleOrderCustomNV<'a> {}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Sync for VkCoarseSampleOrderCustomNV<'a> {}
#[cfg(feature = "VK_NV_shading_rate_image")]
impl<'a> VkCoarseSampleOrderCustomNV<'a> {
  pub const DEFAULT: Self = Self {
    shadingRate: VkShadingRatePaletteEntryNV(0),
    sampleCount: 0,
    sampleLocationCount: 0,
    pSampleLocations: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_shadingRate(mut self, val: VkShadingRatePaletteEntryNV) -> Self {
    self.shadingRate = val;
    self
  }
  #[inline]
  pub const fn with_sampleCount(mut self, val: u32) -> Self {
    self.sampleCount = val;
    self
  }
  #[inline]
  pub const fn with_sampleLocationCount(mut self, val: u32) -> Self {
    self.sampleLocationCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSampleLocations(mut self, val: &'a [VkCoarseSampleLocationNV]) -> Self {
    self.sampleLocationCount = val.len() as u32;
    self.pSampleLocations = val.as_ptr();
    self
  }
}
/// [VkPipelineViewportCoarseSampleOrderStateCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html)
///
/// **Extends:** VkPipelineViewportStateCreateInfo.
#[cfg(feature = "VK_NV_shading_rate_image")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub sampleOrderType: VkCoarseSampleOrderTypeNV,
  /// Optional: true
  pub customSampleOrderCount: u32,
  /// Length: customSampleOrderCount
  pub pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Send for VkPipelineViewportCoarseSampleOrderStateCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_shading_rate_image")]
unsafe impl<'a> Sync for VkPipelineViewportCoarseSampleOrderStateCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_shading_rate_image",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineViewportStateCreateInfo<'root>>
  for VkPipelineViewportCoarseSampleOrderStateCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_shading_rate_image")]
impl<'a> VkPipelineViewportCoarseSampleOrderStateCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    sampleOrderType: VkCoarseSampleOrderTypeNV(0),
    customSampleOrderCount: 0,
    pCustomSampleOrders: core::ptr::null(),
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
  pub const fn with_sampleOrderType(mut self, val: VkCoarseSampleOrderTypeNV) -> Self {
    self.sampleOrderType = val;
    self
  }
  #[inline]
  pub const fn with_customSampleOrderCount(mut self, val: u32) -> Self {
    self.customSampleOrderCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCustomSampleOrders(
    mut self,
    val: &'a [VkCoarseSampleOrderCustomNV<'a>],
  ) -> Self {
    self.customSampleOrderCount = val.len() as u32;
    self.pCustomSampleOrders = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineViewportStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineViewportStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPipelineViewportSwizzleStateCreateFlagsNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportSwizzleStateCreateFlagsNV.html)
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub type VkPipelineViewportSwizzleStateCreateFlagsNV = VkFlags;
/// [VkViewportSwizzleNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkViewportSwizzleNV.html)
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkViewportSwizzleNV {
  pub x: VkViewportCoordinateSwizzleNV,
  pub y: VkViewportCoordinateSwizzleNV,
  pub z: VkViewportCoordinateSwizzleNV,
  pub w: VkViewportCoordinateSwizzleNV,
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
unsafe impl Send for VkViewportSwizzleNV {}
#[cfg(feature = "VK_NV_viewport_swizzle")]
unsafe impl Sync for VkViewportSwizzleNV {}
#[cfg(feature = "VK_NV_viewport_swizzle")]
impl VkViewportSwizzleNV {
  pub const DEFAULT: Self = Self {
    x: VkViewportCoordinateSwizzleNV(0),
    y: VkViewportCoordinateSwizzleNV(0),
    z: VkViewportCoordinateSwizzleNV(0),
    w: VkViewportCoordinateSwizzleNV(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_x(mut self, val: VkViewportCoordinateSwizzleNV) -> Self {
    self.x = val;
    self
  }
  #[inline]
  pub const fn with_y(mut self, val: VkViewportCoordinateSwizzleNV) -> Self {
    self.y = val;
    self
  }
  #[inline]
  pub const fn with_z(mut self, val: VkViewportCoordinateSwizzleNV) -> Self {
    self.z = val;
    self
  }
  #[inline]
  pub const fn with_w(mut self, val: VkViewportCoordinateSwizzleNV) -> Self {
    self.w = val;
    self
  }
}
/// [VkPipelineViewportSwizzleStateCreateInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportSwizzleStateCreateInfoNV.html)
///
/// **Extends:** VkPipelineViewportStateCreateInfo.
#[cfg(feature = "VK_NV_viewport_swizzle")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
  pub viewportCount: u32,
  /// Length: viewportCount
  pub pViewportSwizzles: *const VkViewportSwizzleNV,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
unsafe impl<'a> Send for VkPipelineViewportSwizzleStateCreateInfoNV<'a> {}
#[cfg(feature = "VK_NV_viewport_swizzle")]
unsafe impl<'a> Sync for VkPipelineViewportSwizzleStateCreateInfoNV<'a> {}
#[cfg(all(
  feature = "VK_NV_viewport_swizzle",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineViewportStateCreateInfo<'root>>
  for VkPipelineViewportSwizzleStateCreateInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_viewport_swizzle")]
impl<'a> VkPipelineViewportSwizzleStateCreateInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
    pNext: core::ptr::null(),
    flags: 0,
    viewportCount: 0,
    pViewportSwizzles: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkPipelineViewportSwizzleStateCreateFlagsNV) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_viewportCount(mut self, val: u32) -> Self {
    self.viewportCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pViewportSwizzles(mut self, val: &'a [VkViewportSwizzleNV]) -> Self {
    self.viewportCount = val.len() as u32;
    self.pViewportSwizzles = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineViewportStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineViewportStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkWin32KeyedMutexAcquireReleaseInfoNV](https://docs.vulkan.org/refpages/latest/refpages/source/VkWin32KeyedMutexAcquireReleaseInfoNV.html)
///
/// **Extends:** VkSubmitInfo, VkSubmitInfo2.
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {
  /// Values: VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub acquireCount: u32,
  /// Length: acquireCount
  pub pAcquireSyncs: *const VkDeviceMemory,
  /// Length: acquireCount
  pub pAcquireKeys: *const u64,
  /// Length: acquireCount
  pub pAcquireTimeoutMilliseconds: *const u32,
  /// Optional: true
  pub releaseCount: u32,
  /// Length: releaseCount
  pub pReleaseSyncs: *const VkDeviceMemory,
  /// Length: releaseCount
  pub pReleaseKeys: *const u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
unsafe impl<'a> Send for VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
unsafe impl<'a> Sync for VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {}
#[cfg(all(feature = "VK_NV_win32_keyed_mutex", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubmitInfo<'root>>
  for VkWin32KeyedMutexAcquireReleaseInfoNV<'child>
{
}
#[cfg(all(feature = "VK_NV_win32_keyed_mutex", feature = "VK_BASE_VERSION_1_3"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubmitInfo2<'root>>
  for VkWin32KeyedMutexAcquireReleaseInfoNV<'child>
{
}
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
impl<'a> VkWin32KeyedMutexAcquireReleaseInfoNV<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV,
    pNext: core::ptr::null(),
    acquireCount: 0,
    pAcquireSyncs: core::ptr::null(),
    pAcquireKeys: core::ptr::null(),
    pAcquireTimeoutMilliseconds: core::ptr::null(),
    releaseCount: 0,
    pReleaseSyncs: core::ptr::null(),
    pReleaseKeys: core::ptr::null(),
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
  pub const fn with_acquireCount(mut self, val: u32) -> Self {
    self.acquireCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAcquireSyncs(mut self, val: &'a [VkDeviceMemory]) -> Self {
    self.acquireCount = val.len() as u32;
    self.pAcquireSyncs = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAcquireKeys(mut self, val: &'a [u64]) -> Self {
    self.acquireCount = val.len() as u32;
    self.pAcquireKeys = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAcquireTimeoutMilliseconds(mut self, val: &'a [u32]) -> Self {
    self.acquireCount = val.len() as u32;
    self.pAcquireTimeoutMilliseconds = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_releaseCount(mut self, val: u32) -> Self {
    self.releaseCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pReleaseSyncs(mut self, val: &'a [VkDeviceMemory]) -> Self {
    self.releaseCount = val.len() as u32;
    self.pReleaseSyncs = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pReleaseKeys(mut self, val: &'a [u64]) -> Self {
    self.releaseCount = val.len() as u32;
    self.pReleaseKeys = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure every provided array constrained by `acquireCount` has the same length. Optional pointer arguments may be null, but non-null pointers must be valid for that same length and outlive any use of this struct instance.
  #[inline]
  pub const fn with_acquireCount_slices(
    mut self,
    pAcquireSyncs: &'a [VkDeviceMemory],
    pAcquireKeys: &'a [u64],
    pAcquireTimeoutMilliseconds: &'a [u32],
  ) -> Self {
    let len = pAcquireSyncs.len();
    self.acquireCount = len as u32;
    self.pAcquireSyncs = pAcquireSyncs.as_ptr();
    self.pAcquireKeys = pAcquireKeys.as_ptr();
    self.pAcquireTimeoutMilliseconds = pAcquireTimeoutMilliseconds.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure every provided array constrained by `releaseCount` has the same length. Optional pointer arguments may be null, but non-null pointers must be valid for that same length and outlive any use of this struct instance.
  #[inline]
  pub const fn with_releaseCount_slices(
    mut self,
    pReleaseSyncs: &'a [VkDeviceMemory],
    pReleaseKeys: &'a [u64],
  ) -> Self {
    let len = pReleaseSyncs.len();
    self.releaseCount = len as u32;
    self.pReleaseSyncs = pReleaseSyncs.as_ptr();
    self.pReleaseKeys = pReleaseKeys.as_ptr();
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
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubmitInfo2<'root, T: VkPNextExtends<VkSubmitInfo2<'root>>>(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
