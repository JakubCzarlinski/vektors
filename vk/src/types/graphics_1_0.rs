#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkAccessFlagBits;
#[cfg(any(feature = "VK_GRAPHICS_VERSION_1_0", feature = "VK_KHR_maintenance10"))]
use crate::enums::VkAttachmentDescriptionFlagBits;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_EXT_load_store_op_none",
  feature = "VK_KHR_load_store_op_none"
))]
use crate::enums::VkAttachmentLoadOp;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_KHR_dynamic_rendering",
  feature = "VK_QCOM_render_pass_store_ops",
  feature = "VK_EXT_load_store_op_none"
))]
use crate::enums::VkAttachmentStoreOp;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkBlendFactor;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_EXT_blend_operation_advanced"
))]
use crate::enums::VkBlendOp;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkColorComponentFlagBits;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::enums::VkCompareOp;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkCullModeFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkDependencyFlagBits;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_NV_clip_space_w_scaling",
  feature = "VK_EXT_discard_rectangles",
  feature = "VK_EXT_sample_locations",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_shading_rate_image",
  feature = "VK_NV_scissor_exclusive",
  feature = "VK_KHR_fragment_shading_rate",
  feature = "VK_EXT_line_rasterization",
  feature = "VK_EXT_extended_dynamic_state",
  feature = "VK_EXT_vertex_input_dynamic_state",
  feature = "VK_EXT_extended_dynamic_state2",
  feature = "VK_EXT_color_write_enable",
  feature = "VK_EXT_extended_dynamic_state3",
  feature = "VK_EXT_attachment_feedback_loop_dynamic_state",
  feature = "VK_KHR_line_rasterization",
  feature = "VK_EXT_depth_clamp_control"
))]
use crate::enums::VkDynamicState;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_KHR_imageless_framebuffer"
))]
use crate::enums::VkFramebufferCreateFlagBits;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkFrontFace;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageAspectFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageLayout;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkLogicOp;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_AMDX_shader_enqueue",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_ray_tracing",
  feature = "VK_HUAWEI_subpass_shading"
))]
use crate::enums::VkPipelineBindPoint;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_EXT_rasterization_order_attachment_access",
  feature = "VK_ARM_rasterization_order_attachment_access"
))]
use crate::enums::VkPipelineColorBlendStateCreateFlagBits;
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
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_EXT_rasterization_order_attachment_access",
  feature = "VK_ARM_rasterization_order_attachment_access"
))]
use crate::enums::VkPipelineDepthStencilStateCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkPipelineStageFlagBits;
#[cfg(any(feature = "VK_GRAPHICS_VERSION_1_0", feature = "VK_NV_fill_rectangle"))]
use crate::enums::VkPolygonMode;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkPrimitiveTopology;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_QCOM_render_pass_transform",
  feature = "VK_VALVE_fragment_density_map_layered"
))]
use crate::enums::VkRenderPassCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSampleCountFlagBits;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkStencilFaceFlagBits;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkStencilOp;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_NVX_multiview_per_view_attributes",
  feature = "VK_QCOM_render_pass_shader_resolve",
  feature = "VK_QCOM_tile_shading",
  feature = "VK_ARM_rasterization_order_attachment_access",
  feature = "VK_EXT_rasterization_order_attachment_access",
  feature = "VK_EXT_legacy_dithering",
  feature = "VK_EXT_custom_resolve"
))]
use crate::enums::VkSubpassDescriptionFlagBits;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkVertexInputRate;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkAccessFlags;
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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkClearColorValue;
#[cfg(any(
  all(
    feature = "VK_EXT_custom_resolve",
    feature = "VK_KHR_dynamic_rendering"
  ),
  all(feature = "VK_EXT_custom_resolve", feature = "VK_VERSION_1_3")
))]
use crate::types::VkCustomResolveCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDependencyFlags;
#[cfg(feature = "VK_EXT_depth_bias_control")]
use crate::types::VkDepthBiasRepresentationInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkDeviceGroupRenderPassBeginInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtent3D;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkExternalFormatANDROID;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkExternalFormatOHOS;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkFramebufferAttachmentsCreateInfo;
#[cfg(feature = "VK_EXT_graphics_pipeline_library")]
use crate::types::VkGraphicsPipelineLibraryCreateInfoEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkGraphicsPipelineShaderGroupsCreateInfoNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageAspectFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageSubresourceLayers;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageView;
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
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
use crate::types::VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkOffset3D;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipeline;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPipelineBinaryInfoKHR;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
use crate::types::VkPipelineColorBlendAdvancedStateCreateInfoEXT;
#[cfg(feature = "VK_EXT_color_write_enable")]
use crate::types::VkPipelineColorWriteCreateInfoEXT;
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
use crate::types::VkPipelineCompilerControlCreateInfoAMD;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
use crate::types::VkPipelineCoverageModulationStateCreateInfoNV;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::types::VkPipelineCoverageReductionStateCreateInfoNV;
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
use crate::types::VkPipelineCoverageToColorStateCreateInfoNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPipelineCreateFlags2CreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPipelineCreationFeedbackCreateInfo;
#[cfg(feature = "VK_EXT_discard_rectangles")]
use crate::types::VkPipelineDiscardRectangleStateCreateInfoEXT;
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
use crate::types::VkPipelineFragmentDensityMapLayeredCreateInfoVALVE;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::types::VkPipelineFragmentShadingRateEnumStateCreateInfoNV;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::types::VkPipelineFragmentShadingRateStateCreateInfoKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineLayout;
#[cfg(feature = "VK_KHR_pipeline_library")]
use crate::types::VkPipelineLibraryCreateInfoKHR;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPipelineOfflineCreateInfo;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
use crate::types::VkPipelineRasterizationConservativeStateCreateInfoEXT;
#[cfg(feature = "VK_EXT_depth_clip_enable")]
use crate::types::VkPipelineRasterizationDepthClipStateCreateInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPipelineRasterizationLineStateCreateInfo;
#[cfg(feature = "VK_EXT_provoking_vertex")]
use crate::types::VkPipelineRasterizationProvokingVertexStateCreateInfoEXT;
#[cfg(feature = "VK_AMD_rasterization_order")]
use crate::types::VkPipelineRasterizationStateRasterizationOrderAMD;
#[cfg(feature = "VK_EXT_transform_feedback")]
use crate::types::VkPipelineRasterizationStateStreamCreateInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkPipelineRenderingCreateInfo;
#[cfg(feature = "VK_NV_representative_fragment_test")]
use crate::types::VkPipelineRepresentativeFragmentTestStateCreateInfoNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPipelineRobustnessCreateInfo;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkPipelineSampleLocationsStateCreateInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineShaderStageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPipelineStageFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPipelineTessellationDomainOriginStateCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPipelineVertexInputDivisorStateCreateInfo;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkPipelineViewportCoarseSampleOrderStateCreateInfoNV;
#[cfg(feature = "VK_EXT_depth_clamp_control")]
use crate::types::VkPipelineViewportDepthClampControlCreateInfoEXT;
#[cfg(feature = "VK_EXT_depth_clip_control")]
use crate::types::VkPipelineViewportDepthClipControlCreateInfoEXT;
#[cfg(feature = "VK_NV_scissor_exclusive")]
use crate::types::VkPipelineViewportExclusiveScissorStateCreateInfoNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkPipelineViewportShadingRateImageStateCreateInfoNV;
#[cfg(feature = "VK_NV_viewport_swizzle")]
use crate::types::VkPipelineViewportSwizzleStateCreateInfoNV;
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
use crate::types::VkPipelineViewportWScalingStateCreateInfoNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkRect2D;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkRenderPassAttachmentBeginInfo;
#[cfg(feature = "VK_EXT_fragment_density_map")]
use crate::types::VkRenderPassFragmentDensityMapCreateInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkRenderPassInputAttachmentAspectCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkRenderPassMultiviewCreateInfo;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkRenderPassPerformanceCountersByRegionBeginInfoARM;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkRenderPassSampleLocationsBeginInfoEXT;
#[cfg(feature = "VK_ARM_render_pass_striped")]
use crate::types::VkRenderPassStripeBeginInfoARM;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkRenderPassTileShadingCreateInfoQCOM;
#[cfg(feature = "VK_QCOM_render_pass_transform")]
use crate::types::VkRenderPassTransformBeginInfoQCOM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkRenderingAttachmentLocationInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkRenderingInputAttachmentIndexInfo;
#[cfg(all(
  feature = "VK_QCOM_tile_memory_heap",
  feature = "VK_QCOM_tile_properties"
))]
use crate::types::VkTileMemorySizeInfoQCOM;
#[cfg(feature = "VK_EXT_validation_features")]
use crate::types::VkValidationFeaturesEXT;
use core::ffi::c_void;
/// [VkSampleMask](https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleMask.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkSampleMask = u32;
/// [VkFramebufferCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferCreateFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkFramebufferCreateFlags = VkFramebufferCreateFlagBits;
/// [VkRenderPassCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkRenderPassCreateFlags = VkRenderPassCreateFlagBits;
/// [VkPipelineDepthStencilStateCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDepthStencilStateCreateFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkPipelineDepthStencilStateCreateFlags = VkPipelineDepthStencilStateCreateFlagBits;
/// [VkPipelineDynamicStateCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDynamicStateCreateFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkPipelineDynamicStateCreateFlags = VkFlags;
/// [VkPipelineColorBlendStateCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendStateCreateFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkPipelineColorBlendStateCreateFlags = VkPipelineColorBlendStateCreateFlagBits;
/// [VkPipelineMultisampleStateCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineMultisampleStateCreateFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;
/// [VkPipelineRasterizationStateCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateCreateFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;
/// [VkPipelineViewportStateCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportStateCreateFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkPipelineViewportStateCreateFlags = VkFlags;
/// [VkPipelineTessellationStateCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineTessellationStateCreateFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkPipelineTessellationStateCreateFlags = VkFlags;
/// [VkPipelineInputAssemblyStateCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineInputAssemblyStateCreateFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
/// [VkPipelineVertexInputStateCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineVertexInputStateCreateFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;
/// [VkColorComponentFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkColorComponentFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkColorComponentFlags = VkColorComponentFlagBits;
/// [VkSubpassDescriptionFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkSubpassDescriptionFlags = VkSubpassDescriptionFlagBits;
/// [VkAttachmentDescriptionFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkAttachmentDescriptionFlags = VkAttachmentDescriptionFlagBits;
/// [VkStencilFaceFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilFaceFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkStencilFaceFlags = VkStencilFaceFlagBits;
/// [VkCullModeFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkCullModeFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type VkCullModeFlags = VkCullModeFlagBits;
/// [VkFramebuffer](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebuffer.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkFramebuffer(pub u64);
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkFramebuffer {
  pub const NULL: Self = Self(0);
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl Default for VkFramebuffer {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkFramebuffer {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkFramebuffer {}
/// [VkRenderPass](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPass.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkRenderPass(pub u64);
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkRenderPass {
  pub const NULL: Self = Self(0);
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl Default for VkRenderPass {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkRenderPass {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkRenderPass {}
/// [VkViewport](https://docs.vulkan.org/refpages/latest/refpages/source/VkViewport.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkViewport {
  /// No Auto-Validity
  pub x: f32,
  /// No Auto-Validity
  pub y: f32,
  /// No Auto-Validity
  pub width: f32,
  /// No Auto-Validity
  pub height: f32,
  pub minDepth: f32,
  pub maxDepth: f32,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkViewport {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkViewport {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkViewport {
  pub const DEFAULT: Self = Self {
    x: 0.0f32,
    y: 0.0f32,
    width: 0.0f32,
    height: 0.0f32,
    minDepth: 0.0f32,
    maxDepth: 0.0f32,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_x(mut self, val: f32) -> Self {
    self.x = val;
    self
  }
  #[inline]
  pub const fn with_y(mut self, val: f32) -> Self {
    self.y = val;
    self
  }
  #[inline]
  pub const fn with_width(mut self, val: f32) -> Self {
    self.width = val;
    self
  }
  #[inline]
  pub const fn with_height(mut self, val: f32) -> Self {
    self.height = val;
    self
  }
  #[inline]
  pub const fn with_minDepth(mut self, val: f32) -> Self {
    self.minDepth = val;
    self
  }
  #[inline]
  pub const fn with_maxDepth(mut self, val: f32) -> Self {
    self.maxDepth = val;
    self
  }
}
/// [VkClearRect](https://docs.vulkan.org/refpages/latest/refpages/source/VkClearRect.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClearRect {
  pub rect: VkRect2D,
  pub baseArrayLayer: u32,
  pub layerCount: u32,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkClearRect {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkClearRect {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkClearRect {
  pub const DEFAULT: Self = Self {
    rect: VkRect2D::DEFAULT,
    baseArrayLayer: 0,
    layerCount: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_rect(mut self, val: VkRect2D) -> Self {
    self.rect = val;
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
/// [VkImageBlit](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageBlit.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageBlit {
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffsets: [VkOffset3D; 2],
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffsets: [VkOffset3D; 2],
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkImageBlit {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkImageBlit {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkImageBlit {
  pub const DEFAULT: Self = Self {
    srcSubresource: VkImageSubresourceLayers::DEFAULT,
    srcOffsets: [VkOffset3D::DEFAULT; 2],
    dstSubresource: VkImageSubresourceLayers::DEFAULT,
    dstOffsets: [VkOffset3D::DEFAULT; 2],
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
  pub const fn with_srcOffsets(mut self, val: [VkOffset3D; 2]) -> Self {
    self.srcOffsets = val;
    self
  }
  #[inline]
  pub const fn with_dstSubresource(mut self, val: VkImageSubresourceLayers) -> Self {
    self.dstSubresource = val;
    self
  }
  #[inline]
  pub const fn with_dstOffsets(mut self, val: [VkOffset3D; 2]) -> Self {
    self.dstOffsets = val;
    self
  }
}
/// [VkImageResolve](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageResolve.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageResolve {
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffset: VkOffset3D,
  pub extent: VkExtent3D,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkImageResolve {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkImageResolve {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkImageResolve {
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
/// [VkVertexInputBindingDescription](https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputBindingDescription.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkVertexInputBindingDescription {
  pub binding: u32,
  pub stride: u32,
  pub inputRate: VkVertexInputRate,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkVertexInputBindingDescription {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkVertexInputBindingDescription {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkVertexInputBindingDescription {
  pub const DEFAULT: Self = Self {
    binding: 0,
    stride: 0,
    inputRate: VkVertexInputRate(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_binding(mut self, val: u32) -> Self {
    self.binding = val;
    self
  }
  #[inline]
  pub const fn with_stride(mut self, val: u32) -> Self {
    self.stride = val;
    self
  }
  #[inline]
  pub const fn with_inputRate(mut self, val: VkVertexInputRate) -> Self {
    self.inputRate = val;
    self
  }
}
/// [VkVertexInputAttributeDescription](https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputAttributeDescription.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkVertexInputAttributeDescription {
  pub location: u32,
  pub binding: u32,
  pub format: VkFormat,
  pub offset: u32,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkVertexInputAttributeDescription {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkVertexInputAttributeDescription {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkVertexInputAttributeDescription {
  pub const DEFAULT: Self = Self {
    location: 0,
    binding: 0,
    format: VkFormat(0),
    offset: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_location(mut self, val: u32) -> Self {
    self.location = val;
    self
  }
  #[inline]
  pub const fn with_binding(mut self, val: u32) -> Self {
    self.binding = val;
    self
  }
  #[inline]
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[inline]
  pub const fn with_offset(mut self, val: u32) -> Self {
    self.offset = val;
    self
  }
}
/// [VkPipelineVertexInputStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineVertexInputStateCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineVertexInputStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineVertexInputStateCreateFlags,
  /// Optional: true
  pub vertexBindingDescriptionCount: u32,
  /// Length: vertexBindingDescriptionCount
  pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
  /// Optional: true
  pub vertexAttributeDescriptionCount: u32,
  /// Length: vertexAttributeDescriptionCount
  pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineVertexInputStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineVertexInputStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkPipelineVertexInputStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    vertexBindingDescriptionCount: 0,
    pVertexBindingDescriptions: core::ptr::null(),
    vertexAttributeDescriptionCount: 0,
    pVertexAttributeDescriptions: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkPipelineVertexInputStateCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_vertexBindingDescriptionCount(mut self, val: u32) -> Self {
    self.vertexBindingDescriptionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pVertexBindingDescriptions(
    mut self,
    val: &'a [VkVertexInputBindingDescription],
  ) -> Self {
    self.vertexBindingDescriptionCount = val.len() as u32;
    self.pVertexBindingDescriptions = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_vertexAttributeDescriptionCount(mut self, val: u32) -> Self {
    self.vertexAttributeDescriptionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pVertexAttributeDescriptions(
    mut self,
    val: &'a [VkVertexInputAttributeDescription],
  ) -> Self {
    self.vertexAttributeDescriptionCount = val.len() as u32;
    self.pVertexAttributeDescriptions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineVertexInputDivisorStateCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineVertexInputDivisorStateCreateInfo<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineVertexInputDivisorStateCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineVertexInputStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineVertexInputStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPipelineInputAssemblyStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineInputAssemblyStateCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineInputAssemblyStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineInputAssemblyStateCreateFlags,
  pub topology: VkPrimitiveTopology,
  pub primitiveRestartEnable: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineInputAssemblyStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineInputAssemblyStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkPipelineInputAssemblyStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    topology: VkPrimitiveTopology(0),
    primitiveRestartEnable: 0,
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
  pub const fn with_flags(mut self, val: VkPipelineInputAssemblyStateCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_topology(mut self, val: VkPrimitiveTopology) -> Self {
    self.topology = val;
    self
  }
  #[inline]
  pub const fn with_primitiveRestartEnable(mut self, val: VkBool32) -> Self {
    self.primitiveRestartEnable = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineInputAssemblyStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineInputAssemblyStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPipelineTessellationStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineTessellationStateCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineTessellationStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineTessellationStateCreateFlags,
  pub patchControlPoints: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineTessellationStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineTessellationStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkPipelineTessellationStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    patchControlPoints: 0,
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
  pub const fn with_flags(mut self, val: VkPipelineTessellationStateCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_patchControlPoints(mut self, val: u32) -> Self {
    self.patchControlPoints = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineTessellationDomainOriginStateCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineTessellationDomainOriginStateCreateInfo<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineTessellationDomainOriginStateCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineTessellationStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineTessellationStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPipelineViewportStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineViewportStateCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineViewportStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineViewportStateCreateFlags,
  /// Optional: true
  pub viewportCount: u32,
  /// Optional: true,  Length: viewportCount,  No Auto-Validity
  pub pViewports: *const VkViewport,
  /// Optional: true
  pub scissorCount: u32,
  /// Optional: true,  Length: scissorCount,  No Auto-Validity
  pub pScissors: *const VkRect2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineViewportStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineViewportStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkPipelineViewportStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    viewportCount: 0,
    pViewports: core::ptr::null(),
    scissorCount: 0,
    pScissors: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkPipelineViewportStateCreateFlags) -> Self {
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
  pub const fn with_pViewports(mut self, val: &'a [VkViewport]) -> Self {
    self.viewportCount = val.len() as u32;
    self.pViewports = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_scissorCount(mut self, val: u32) -> Self {
    self.scissorCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pScissors(mut self, val: &'a [VkRect2D]) -> Self {
    self.scissorCount = val.len() as u32;
    self.pScissors = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_NV_shading_rate_image")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineViewportCoarseSampleOrderStateCreateInfoNV<'child>(
    mut self,
    val: &'a VkPipelineViewportCoarseSampleOrderStateCreateInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineViewportCoarseSampleOrderStateCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_depth_clamp_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineViewportDepthClampControlCreateInfoEXT<'child>(
    mut self,
    val: &'a VkPipelineViewportDepthClampControlCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineViewportDepthClampControlCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_depth_clip_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineViewportDepthClipControlCreateInfoEXT<'child>(
    mut self,
    val: &'a VkPipelineViewportDepthClipControlCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineViewportDepthClipControlCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_scissor_exclusive")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineViewportExclusiveScissorStateCreateInfoNV<'child>(
    mut self,
    val: &'a VkPipelineViewportExclusiveScissorStateCreateInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineViewportExclusiveScissorStateCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_shading_rate_image")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineViewportShadingRateImageStateCreateInfoNV<'child>(
    mut self,
    val: &'a VkPipelineViewportShadingRateImageStateCreateInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineViewportShadingRateImageStateCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_viewport_swizzle")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineViewportSwizzleStateCreateInfoNV<'child>(
    mut self,
    val: &'a VkPipelineViewportSwizzleStateCreateInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineViewportSwizzleStateCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_clip_space_w_scaling")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineViewportWScalingStateCreateInfoNV<'child>(
    mut self,
    val: &'a VkPipelineViewportWScalingStateCreateInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineViewportWScalingStateCreateInfoNV<'child>).cast::<c_void>();
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
/// [VkPipelineRasterizationStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationStateCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineRasterizationStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineRasterizationStateCreateFlags,
  pub depthClampEnable: VkBool32,
  pub rasterizerDiscardEnable: VkBool32,
  pub polygonMode: VkPolygonMode,
  /// Optional: true
  pub cullMode: VkCullModeFlags,
  pub frontFace: VkFrontFace,
  pub depthBiasEnable: VkBool32,
  pub depthBiasConstantFactor: f32,
  pub depthBiasClamp: f32,
  pub depthBiasSlopeFactor: f32,
  pub lineWidth: f32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineRasterizationStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineRasterizationStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkPipelineRasterizationStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    depthClampEnable: 0,
    rasterizerDiscardEnable: 0,
    polygonMode: VkPolygonMode(0),
    cullMode: VkCullModeFlagBits(0),
    frontFace: VkFrontFace(0),
    depthBiasEnable: 0,
    depthBiasConstantFactor: 0.0f32,
    depthBiasClamp: 0.0f32,
    depthBiasSlopeFactor: 0.0f32,
    lineWidth: 0.0f32,
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
  pub const fn with_flags(mut self, val: VkPipelineRasterizationStateCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_depthClampEnable(mut self, val: VkBool32) -> Self {
    self.depthClampEnable = val;
    self
  }
  #[inline]
  pub const fn with_rasterizerDiscardEnable(mut self, val: VkBool32) -> Self {
    self.rasterizerDiscardEnable = val;
    self
  }
  #[inline]
  pub const fn with_polygonMode(mut self, val: VkPolygonMode) -> Self {
    self.polygonMode = val;
    self
  }
  #[inline]
  pub const fn with_cullMode(mut self, val: VkCullModeFlags) -> Self {
    self.cullMode = val;
    self
  }
  #[inline]
  pub const fn with_frontFace(mut self, val: VkFrontFace) -> Self {
    self.frontFace = val;
    self
  }
  #[inline]
  pub const fn with_depthBiasEnable(mut self, val: VkBool32) -> Self {
    self.depthBiasEnable = val;
    self
  }
  #[inline]
  pub const fn with_depthBiasConstantFactor(mut self, val: f32) -> Self {
    self.depthBiasConstantFactor = val;
    self
  }
  #[inline]
  pub const fn with_depthBiasClamp(mut self, val: f32) -> Self {
    self.depthBiasClamp = val;
    self
  }
  #[inline]
  pub const fn with_depthBiasSlopeFactor(mut self, val: f32) -> Self {
    self.depthBiasSlopeFactor = val;
    self
  }
  #[inline]
  pub const fn with_lineWidth(mut self, val: f32) -> Self {
    self.lineWidth = val;
    self
  }
  #[cfg(feature = "VK_EXT_depth_bias_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDepthBiasRepresentationInfoEXT<'child>(
    mut self,
    val: &'a VkDepthBiasRepresentationInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDepthBiasRepresentationInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_conservative_rasterization")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineRasterizationConservativeStateCreateInfoEXT<'child>(
    mut self,
    val: &'a VkPipelineRasterizationConservativeStateCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineRasterizationConservativeStateCreateInfoEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_depth_clip_enable")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineRasterizationDepthClipStateCreateInfoEXT<'child>(
    mut self,
    val: &'a VkPipelineRasterizationDepthClipStateCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineRasterizationDepthClipStateCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineRasterizationLineStateCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineRasterizationLineStateCreateInfo<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineRasterizationLineStateCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_provoking_vertex")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineRasterizationProvokingVertexStateCreateInfoEXT<'child>(
    mut self,
    val: &'a VkPipelineRasterizationProvokingVertexStateCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineRasterizationProvokingVertexStateCreateInfoEXT<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_rasterization_order")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineRasterizationStateRasterizationOrderAMD<'child>(
    mut self,
    val: &'a VkPipelineRasterizationStateRasterizationOrderAMD<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineRasterizationStateRasterizationOrderAMD<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_transform_feedback")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineRasterizationStateStreamCreateInfoEXT<'child>(
    mut self,
    val: &'a VkPipelineRasterizationStateStreamCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineRasterizationStateStreamCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineRasterizationStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineRasterizationStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPipelineMultisampleStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineMultisampleStateCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineMultisampleStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineMultisampleStateCreateFlags,
  pub rasterizationSamples: VkSampleCountFlagBits,
  pub sampleShadingEnable: VkBool32,
  pub minSampleShading: f32,
  /// Optional: true,  Length: latexmath:[\lceil{\mathit{rasterizationSamples} \over 32}\rceil],  No Auto-Validity
  pub pSampleMask: *const VkSampleMask,
  pub alphaToCoverageEnable: VkBool32,
  pub alphaToOneEnable: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineMultisampleStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineMultisampleStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkPipelineMultisampleStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    rasterizationSamples: VkSampleCountFlagBits(0),
    sampleShadingEnable: 0,
    minSampleShading: 0.0f32,
    pSampleMask: core::ptr::null(),
    alphaToCoverageEnable: 0,
    alphaToOneEnable: 0,
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
  pub const fn with_flags(mut self, val: VkPipelineMultisampleStateCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_rasterizationSamples(mut self, val: VkSampleCountFlagBits) -> Self {
    self.rasterizationSamples = val;
    self
  }
  #[inline]
  pub const fn with_sampleShadingEnable(mut self, val: VkBool32) -> Self {
    self.sampleShadingEnable = val;
    self
  }
  #[inline]
  pub const fn with_minSampleShading(mut self, val: f32) -> Self {
    self.minSampleShading = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSampleMask(mut self, val: *const VkSampleMask) -> Self {
    self.pSampleMask = val;
    self
  }
  #[inline]
  pub const fn with_alphaToCoverageEnable(mut self, val: VkBool32) -> Self {
    self.alphaToCoverageEnable = val;
    self
  }
  #[inline]
  pub const fn with_alphaToOneEnable(mut self, val: VkBool32) -> Self {
    self.alphaToOneEnable = val;
    self
  }
  #[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineCoverageModulationStateCreateInfoNV<'child>(
    mut self,
    val: &'a VkPipelineCoverageModulationStateCreateInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineCoverageModulationStateCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_coverage_reduction_mode")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineCoverageReductionStateCreateInfoNV<'child>(
    mut self,
    val: &'a VkPipelineCoverageReductionStateCreateInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineCoverageReductionStateCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_fragment_coverage_to_color")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineCoverageToColorStateCreateInfoNV<'child>(
    mut self,
    val: &'a VkPipelineCoverageToColorStateCreateInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineCoverageToColorStateCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_sample_locations")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineSampleLocationsStateCreateInfoEXT<'child>(
    mut self,
    val: &'a VkPipelineSampleLocationsStateCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineSampleLocationsStateCreateInfoEXT<'child>).cast::<c_void>();
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
/// [VkPipelineColorBlendAttachmentState](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendAttachmentState.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineColorBlendAttachmentState {
  pub blendEnable: VkBool32,
  pub srcColorBlendFactor: VkBlendFactor,
  pub dstColorBlendFactor: VkBlendFactor,
  pub colorBlendOp: VkBlendOp,
  pub srcAlphaBlendFactor: VkBlendFactor,
  pub dstAlphaBlendFactor: VkBlendFactor,
  pub alphaBlendOp: VkBlendOp,
  /// Optional: true
  pub colorWriteMask: VkColorComponentFlags,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkPipelineColorBlendAttachmentState {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkPipelineColorBlendAttachmentState {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkPipelineColorBlendAttachmentState {
  pub const DEFAULT: Self = Self {
    blendEnable: 0,
    srcColorBlendFactor: VkBlendFactor(0),
    dstColorBlendFactor: VkBlendFactor(0),
    colorBlendOp: VkBlendOp(0),
    srcAlphaBlendFactor: VkBlendFactor(0),
    dstAlphaBlendFactor: VkBlendFactor(0),
    alphaBlendOp: VkBlendOp(0),
    colorWriteMask: VkColorComponentFlagBits(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_blendEnable(mut self, val: VkBool32) -> Self {
    self.blendEnable = val;
    self
  }
  #[inline]
  pub const fn with_srcColorBlendFactor(mut self, val: VkBlendFactor) -> Self {
    self.srcColorBlendFactor = val;
    self
  }
  #[inline]
  pub const fn with_dstColorBlendFactor(mut self, val: VkBlendFactor) -> Self {
    self.dstColorBlendFactor = val;
    self
  }
  #[inline]
  pub const fn with_colorBlendOp(mut self, val: VkBlendOp) -> Self {
    self.colorBlendOp = val;
    self
  }
  #[inline]
  pub const fn with_srcAlphaBlendFactor(mut self, val: VkBlendFactor) -> Self {
    self.srcAlphaBlendFactor = val;
    self
  }
  #[inline]
  pub const fn with_dstAlphaBlendFactor(mut self, val: VkBlendFactor) -> Self {
    self.dstAlphaBlendFactor = val;
    self
  }
  #[inline]
  pub const fn with_alphaBlendOp(mut self, val: VkBlendOp) -> Self {
    self.alphaBlendOp = val;
    self
  }
  #[inline]
  pub const fn with_colorWriteMask(mut self, val: VkColorComponentFlags) -> Self {
    self.colorWriteMask = val;
    self
  }
}
/// [VkPipelineColorBlendStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendStateCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineColorBlendStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineColorBlendStateCreateFlags,
  pub logicOpEnable: VkBool32,
  /// No Auto-Validity
  pub logicOp: VkLogicOp,
  /// Optional: true
  pub attachmentCount: u32,
  /// Optional: true,  Length: attachmentCount
  pub pAttachments: *const VkPipelineColorBlendAttachmentState,
  pub blendConstants: [f32; 4],
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineColorBlendStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineColorBlendStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkPipelineColorBlendStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkPipelineColorBlendStateCreateFlagBits(0),
    logicOpEnable: 0,
    logicOp: VkLogicOp(0),
    attachmentCount: 0,
    pAttachments: core::ptr::null(),
    blendConstants: [0.0f32; 4],
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
  pub const fn with_flags(mut self, val: VkPipelineColorBlendStateCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_logicOpEnable(mut self, val: VkBool32) -> Self {
    self.logicOpEnable = val;
    self
  }
  #[inline]
  pub const fn with_logicOp(mut self, val: VkLogicOp) -> Self {
    self.logicOp = val;
    self
  }
  #[inline]
  pub const fn with_attachmentCount(mut self, val: u32) -> Self {
    self.attachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAttachments(mut self, val: &'a [VkPipelineColorBlendAttachmentState]) -> Self {
    self.attachmentCount = val.len() as u32;
    self.pAttachments = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_blendConstants(mut self, val: [f32; 4]) -> Self {
    self.blendConstants = val;
    self
  }
  #[cfg(feature = "VK_EXT_blend_operation_advanced")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineColorBlendAdvancedStateCreateInfoEXT<'child>(
    mut self,
    val: &'a VkPipelineColorBlendAdvancedStateCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineColorBlendAdvancedStateCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_color_write_enable")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineColorWriteCreateInfoEXT<'child>(
    mut self,
    val: &'a VkPipelineColorWriteCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineColorWriteCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineColorBlendStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineColorBlendStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPipelineDynamicStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDynamicStateCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineDynamicStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineDynamicStateCreateFlags,
  /// Optional: true
  pub dynamicStateCount: u32,
  /// Length: dynamicStateCount
  pub pDynamicStates: *const VkDynamicState,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineDynamicStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineDynamicStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkPipelineDynamicStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    dynamicStateCount: 0,
    pDynamicStates: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkPipelineDynamicStateCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_dynamicStateCount(mut self, val: u32) -> Self {
    self.dynamicStateCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDynamicStates(mut self, val: &'a [VkDynamicState]) -> Self {
    self.dynamicStateCount = val.len() as u32;
    self.pDynamicStates = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineDynamicStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineDynamicStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkStencilOpState](https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilOpState.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkStencilOpState {
  pub failOp: VkStencilOp,
  pub passOp: VkStencilOp,
  pub depthFailOp: VkStencilOp,
  pub compareOp: VkCompareOp,
  pub compareMask: u32,
  pub writeMask: u32,
  pub reference: u32,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkStencilOpState {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkStencilOpState {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkStencilOpState {
  pub const DEFAULT: Self = Self {
    failOp: VkStencilOp(0),
    passOp: VkStencilOp(0),
    depthFailOp: VkStencilOp(0),
    compareOp: VkCompareOp(0),
    compareMask: 0,
    writeMask: 0,
    reference: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_failOp(mut self, val: VkStencilOp) -> Self {
    self.failOp = val;
    self
  }
  #[inline]
  pub const fn with_passOp(mut self, val: VkStencilOp) -> Self {
    self.passOp = val;
    self
  }
  #[inline]
  pub const fn with_depthFailOp(mut self, val: VkStencilOp) -> Self {
    self.depthFailOp = val;
    self
  }
  #[inline]
  pub const fn with_compareOp(mut self, val: VkCompareOp) -> Self {
    self.compareOp = val;
    self
  }
  #[inline]
  pub const fn with_compareMask(mut self, val: u32) -> Self {
    self.compareMask = val;
    self
  }
  #[inline]
  pub const fn with_writeMask(mut self, val: u32) -> Self {
    self.writeMask = val;
    self
  }
  #[inline]
  pub const fn with_reference(mut self, val: u32) -> Self {
    self.reference = val;
    self
  }
}
/// [VkPipelineDepthStencilStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDepthStencilStateCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineDepthStencilStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineDepthStencilStateCreateFlags,
  pub depthTestEnable: VkBool32,
  pub depthWriteEnable: VkBool32,
  pub depthCompareOp: VkCompareOp,
  pub depthBoundsTestEnable: VkBool32,
  pub stencilTestEnable: VkBool32,
  pub front: VkStencilOpState,
  pub back: VkStencilOpState,
  pub minDepthBounds: f32,
  pub maxDepthBounds: f32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineDepthStencilStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineDepthStencilStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkPipelineDepthStencilStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkPipelineDepthStencilStateCreateFlagBits(0),
    depthTestEnable: 0,
    depthWriteEnable: 0,
    depthCompareOp: VkCompareOp(0),
    depthBoundsTestEnable: 0,
    stencilTestEnable: 0,
    front: VkStencilOpState::DEFAULT,
    back: VkStencilOpState::DEFAULT,
    minDepthBounds: 0.0f32,
    maxDepthBounds: 0.0f32,
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
  pub const fn with_flags(mut self, val: VkPipelineDepthStencilStateCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_depthTestEnable(mut self, val: VkBool32) -> Self {
    self.depthTestEnable = val;
    self
  }
  #[inline]
  pub const fn with_depthWriteEnable(mut self, val: VkBool32) -> Self {
    self.depthWriteEnable = val;
    self
  }
  #[inline]
  pub const fn with_depthCompareOp(mut self, val: VkCompareOp) -> Self {
    self.depthCompareOp = val;
    self
  }
  #[inline]
  pub const fn with_depthBoundsTestEnable(mut self, val: VkBool32) -> Self {
    self.depthBoundsTestEnable = val;
    self
  }
  #[inline]
  pub const fn with_stencilTestEnable(mut self, val: VkBool32) -> Self {
    self.stencilTestEnable = val;
    self
  }
  #[inline]
  pub const fn with_front(mut self, val: VkStencilOpState) -> Self {
    self.front = val;
    self
  }
  #[inline]
  pub const fn with_back(mut self, val: VkStencilOpState) -> Self {
    self.back = val;
    self
  }
  #[inline]
  pub const fn with_minDepthBounds(mut self, val: f32) -> Self {
    self.minDepthBounds = val;
    self
  }
  #[inline]
  pub const fn with_maxDepthBounds(mut self, val: f32) -> Self {
    self.maxDepthBounds = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineDepthStencilStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineDepthStencilStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkGraphicsPipelineCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkGraphicsPipelineCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true,  No Auto-Validity
  pub flags: VkPipelineCreateFlags,
  /// Optional: true,  No Auto-Validity
  pub stageCount: u32,
  #[cfg(not(feature = "VKSC_VERSION_1_0"))]
  /// Optional: true,  Length: stageCount,  No Auto-Validity
  pub pStages: *const VkPipelineShaderStageCreateInfo<'a>,
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// Length: stageCount,  No Auto-Validity
  pub pStages: *const VkPipelineShaderStageCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub pTessellationState: *const VkPipelineTessellationStateCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub pViewportState: *const VkPipelineViewportStateCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo<'a>,
  /// Optional: true
  pub pDynamicState: *const VkPipelineDynamicStateCreateInfo<'a>,
  /// Optional: true,  No Auto-Validity
  pub layout: VkPipelineLayout,
  /// Optional: true,  No Auto-Validity
  pub renderPass: VkRenderPass,
  /// No Auto-Validity
  pub subpass: u32,
  /// Optional: true,  No Auto-Validity
  pub basePipelineHandle: VkPipeline,
  pub basePipelineIndex: i32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkGraphicsPipelineCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkGraphicsPipelineCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkGraphicsPipelineCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::GRAPHICS_PIPELINE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkPipelineCreateFlagBits(0),
    stageCount: 0,
    #[cfg(not(feature = "VKSC_VERSION_1_0"))]
    pStages: core::ptr::null(),
    #[cfg(feature = "VKSC_VERSION_1_0")]
    pStages: core::ptr::null(),
    pVertexInputState: core::ptr::null(),
    pInputAssemblyState: core::ptr::null(),
    pTessellationState: core::ptr::null(),
    pViewportState: core::ptr::null(),
    pRasterizationState: core::ptr::null(),
    pMultisampleState: core::ptr::null(),
    pDepthStencilState: core::ptr::null(),
    pColorBlendState: core::ptr::null(),
    pDynamicState: core::ptr::null(),
    layout: VkPipelineLayout::DEFAULT,
    renderPass: VkRenderPass::DEFAULT,
    subpass: 0,
    basePipelineHandle: VkPipeline::DEFAULT,
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
  #[cfg(not(feature = "VKSC_VERSION_1_0"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pStages(mut self, val: &'a [VkPipelineShaderStageCreateInfo<'a>]) -> Self {
    self.stageCount = val.len() as u32;
    self.pStages = val.as_ptr();
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
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
  pub const fn with_pInputAssemblyState(
    mut self,
    val: *const VkPipelineInputAssemblyStateCreateInfo<'a>,
  ) -> Self {
    self.pInputAssemblyState = val;
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
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pViewportState(
    mut self,
    val: *const VkPipelineViewportStateCreateInfo<'a>,
  ) -> Self {
    self.pViewportState = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pRasterizationState(
    mut self,
    val: *const VkPipelineRasterizationStateCreateInfo<'a>,
  ) -> Self {
    self.pRasterizationState = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pMultisampleState(
    mut self,
    val: *const VkPipelineMultisampleStateCreateInfo<'a>,
  ) -> Self {
    self.pMultisampleState = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDepthStencilState(
    mut self,
    val: *const VkPipelineDepthStencilStateCreateInfo<'a>,
  ) -> Self {
    self.pDepthStencilState = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pColorBlendState(
    mut self,
    val: *const VkPipelineColorBlendStateCreateInfo<'a>,
  ) -> Self {
    self.pColorBlendState = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDynamicState(
    mut self,
    val: *const VkPipelineDynamicStateCreateInfo<'a>,
  ) -> Self {
    self.pDynamicState = val;
    self
  }
  #[inline]
  pub const fn with_layout(mut self, val: VkPipelineLayout) -> Self {
    self.layout = val;
    self
  }
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
  #[cfg(feature = "VK_EXT_graphics_pipeline_library")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkGraphicsPipelineLibraryCreateInfoEXT<'child>(
    mut self,
    val: &'a VkGraphicsPipelineLibraryCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkGraphicsPipelineLibraryCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_device_generated_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkGraphicsPipelineShaderGroupsCreateInfoNV<'child>(
    mut self,
    val: &'a VkGraphicsPipelineShaderGroupsCreateInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkGraphicsPipelineShaderGroupsCreateInfoNV<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineBinaryInfoKHR<'child>(
    mut self,
    val: &'a VkPipelineBinaryInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineBinaryInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMD_pipeline_compiler_control")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineCompilerControlCreateInfoAMD<'child>(
    mut self,
    val: &'a VkPipelineCompilerControlCreateInfoAMD<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineCompilerControlCreateInfoAMD<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_EXT_discard_rectangles")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineDiscardRectangleStateCreateInfoEXT<'child>(
    mut self,
    val: &'a VkPipelineDiscardRectangleStateCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineDiscardRectangleStateCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineFragmentDensityMapLayeredCreateInfoVALVE<'child>(
    mut self,
    val: &'a VkPipelineFragmentDensityMapLayeredCreateInfoVALVE<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineFragmentDensityMapLayeredCreateInfoVALVE<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineFragmentShadingRateEnumStateCreateInfoNV<'child>(
    mut self,
    val: &'a VkPipelineFragmentShadingRateEnumStateCreateInfoNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineFragmentShadingRateEnumStateCreateInfoNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_fragment_shading_rate")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineFragmentShadingRateStateCreateInfoKHR<'child>(
    mut self,
    val: &'a VkPipelineFragmentShadingRateStateCreateInfoKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineFragmentShadingRateStateCreateInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_pipeline_library")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineLibraryCreateInfoKHR<'child>(
    mut self,
    val: &'a VkPipelineLibraryCreateInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineLibraryCreateInfoKHR<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineRenderingCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineRenderingCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineRenderingCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_representative_fragment_test")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineRepresentativeFragmentTestStateCreateInfoNV<'child>(
    mut self,
    val: &'a VkPipelineRepresentativeFragmentTestStateCreateInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineRepresentativeFragmentTestStateCreateInfoNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineRobustnessCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineRobustnessCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineRobustnessCreateInfo<'child>).cast::<c_void>();
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
/// [VkRenderPassBeginInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassBeginInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassBeginInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub renderPass: VkRenderPass,
  pub framebuffer: VkFramebuffer,
  pub renderArea: VkRect2D,
  /// Optional: true
  pub clearValueCount: u32,
  /// Length: clearValueCount,  No Auto-Validity
  pub pClearValues: *const VkClearValue,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkRenderPassBeginInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkRenderPassBeginInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkRenderPassBeginInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RENDER_PASS_BEGIN_INFO,
    pNext: core::ptr::null(),
    renderPass: VkRenderPass::DEFAULT,
    framebuffer: VkFramebuffer::DEFAULT,
    renderArea: VkRect2D::DEFAULT,
    clearValueCount: 0,
    pClearValues: core::ptr::null(),
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
  pub const fn with_renderPass(mut self, val: VkRenderPass) -> Self {
    self.renderPass = val;
    self
  }
  #[inline]
  pub const fn with_framebuffer(mut self, val: VkFramebuffer) -> Self {
    self.framebuffer = val;
    self
  }
  #[inline]
  pub const fn with_renderArea(mut self, val: VkRect2D) -> Self {
    self.renderArea = val;
    self
  }
  #[inline]
  pub const fn with_clearValueCount(mut self, val: u32) -> Self {
    self.clearValueCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pClearValues(mut self, val: &'a [VkClearValue]) -> Self {
    self.clearValueCount = val.len() as u32;
    self.pClearValues = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceGroupRenderPassBeginInfo<'child>(
    mut self,
    val: &'a VkDeviceGroupRenderPassBeginInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDeviceGroupRenderPassBeginInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'child>(
    mut self,
    val: &'a VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassAttachmentBeginInfo<'child>(
    mut self,
    val: &'a VkRenderPassAttachmentBeginInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassAttachmentBeginInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_performance_counters_by_region")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassPerformanceCountersByRegionBeginInfoARM<'child>(
    mut self,
    val: &'a VkRenderPassPerformanceCountersByRegionBeginInfoARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkRenderPassPerformanceCountersByRegionBeginInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_sample_locations")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassSampleLocationsBeginInfoEXT<'child>(
    mut self,
    val: &'a VkRenderPassSampleLocationsBeginInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassSampleLocationsBeginInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_render_pass_striped")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassStripeBeginInfoARM<'child>(
    mut self,
    val: &'a VkRenderPassStripeBeginInfoARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassStripeBeginInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_render_pass_transform")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassTransformBeginInfoQCOM<'child>(
    mut self,
    val: &'a VkRenderPassTransformBeginInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassTransformBeginInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassBeginInfo<
    'root,
    T: VkPNextExtends<VkRenderPassBeginInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkClearDepthStencilValue](https://docs.vulkan.org/refpages/latest/refpages/source/VkClearDepthStencilValue.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClearDepthStencilValue {
  pub depth: f32,
  pub stencil: u32,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkClearDepthStencilValue {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkClearDepthStencilValue {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkClearDepthStencilValue {
  pub const DEFAULT: Self = Self {
    depth: 0.0f32,
    stencil: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_depth(mut self, val: f32) -> Self {
    self.depth = val;
    self
  }
  #[inline]
  pub const fn with_stencil(mut self, val: u32) -> Self {
    self.stencil = val;
    self
  }
}
/// [VkClearValue](https://docs.vulkan.org/refpages/latest/refpages/source/VkClearValue.html)
///
/// // Union allowing specification of color or depth and stencil values. Actual value selected is based on attachment being cleared.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearValue {
  /// No Auto-Validity
  pub color: VkClearColorValue,
  pub depthStencil: VkClearDepthStencilValue,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkClearValue {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkClearValue {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkClearValue {
  pub const DEFAULT: Self = unsafe {
    Self {
      color: core::mem::zeroed::<VkClearColorValue>(),
    }
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl core::fmt::Debug for VkClearValue {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("VkClearValue")
      .field("color", unsafe { &self.color })
      .finish()
  }
}
/// [VkClearAttachment](https://docs.vulkan.org/refpages/latest/refpages/source/VkClearAttachment.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkClearAttachment {
  pub aspectMask: VkImageAspectFlags,
  pub colorAttachment: u32,
  /// No Auto-Validity
  pub clearValue: VkClearValue,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkClearAttachment {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkClearAttachment {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkClearAttachment {
  pub const DEFAULT: Self = Self {
    aspectMask: VkImageAspectFlagBits(0),
    colorAttachment: 0,
    clearValue: VkClearValue::DEFAULT,
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
  pub const fn with_colorAttachment(mut self, val: u32) -> Self {
    self.colorAttachment = val;
    self
  }
  #[inline]
  pub const fn with_clearValue(mut self, val: VkClearValue) -> Self {
    self.clearValue = val;
    self
  }
}
/// [VkAttachmentDescription](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescription.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `VkAttachmentDescription2`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAttachmentDescription {
  /// Optional: true
  pub flags: VkAttachmentDescriptionFlags,
  pub format: VkFormat,
  pub samples: VkSampleCountFlagBits,
  pub loadOp: VkAttachmentLoadOp,
  pub storeOp: VkAttachmentStoreOp,
  pub stencilLoadOp: VkAttachmentLoadOp,
  pub stencilStoreOp: VkAttachmentStoreOp,
  pub initialLayout: VkImageLayout,
  pub finalLayout: VkImageLayout,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkAttachmentDescription {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkAttachmentDescription {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkAttachmentDescription {
  pub const DEFAULT: Self = Self {
    flags: VkAttachmentDescriptionFlagBits(0),
    format: VkFormat(0),
    samples: VkSampleCountFlagBits(0),
    loadOp: VkAttachmentLoadOp(0),
    storeOp: VkAttachmentStoreOp(0),
    stencilLoadOp: VkAttachmentLoadOp(0),
    stencilStoreOp: VkAttachmentStoreOp(0),
    initialLayout: VkImageLayout(0),
    finalLayout: VkImageLayout(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkAttachmentDescriptionFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[inline]
  pub const fn with_samples(mut self, val: VkSampleCountFlagBits) -> Self {
    self.samples = val;
    self
  }
  #[inline]
  pub const fn with_loadOp(mut self, val: VkAttachmentLoadOp) -> Self {
    self.loadOp = val;
    self
  }
  #[inline]
  pub const fn with_storeOp(mut self, val: VkAttachmentStoreOp) -> Self {
    self.storeOp = val;
    self
  }
  #[inline]
  pub const fn with_stencilLoadOp(mut self, val: VkAttachmentLoadOp) -> Self {
    self.stencilLoadOp = val;
    self
  }
  #[inline]
  pub const fn with_stencilStoreOp(mut self, val: VkAttachmentStoreOp) -> Self {
    self.stencilStoreOp = val;
    self
  }
  #[inline]
  pub const fn with_initialLayout(mut self, val: VkImageLayout) -> Self {
    self.initialLayout = val;
    self
  }
  #[inline]
  pub const fn with_finalLayout(mut self, val: VkImageLayout) -> Self {
    self.finalLayout = val;
    self
  }
}
/// [VkAttachmentReference](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentReference.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `VkAttachmentReference2`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAttachmentReference {
  pub attachment: u32,
  pub layout: VkImageLayout,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkAttachmentReference {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkAttachmentReference {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkAttachmentReference {
  pub const DEFAULT: Self = Self {
    attachment: 0,
    layout: VkImageLayout(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_attachment(mut self, val: u32) -> Self {
    self.attachment = val;
    self
  }
  #[inline]
  pub const fn with_layout(mut self, val: VkImageLayout) -> Self {
    self.layout = val;
    self
  }
}
/// [VkSubpassDescription](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescription.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `VkSubpassDescription2`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubpassDescription<'a> {
  /// Optional: true
  pub flags: VkSubpassDescriptionFlags,
  pub pipelineBindPoint: VkPipelineBindPoint,
  /// Optional: true
  pub inputAttachmentCount: u32,
  /// Length: inputAttachmentCount
  pub pInputAttachments: *const VkAttachmentReference,
  /// Optional: true
  pub colorAttachmentCount: u32,
  /// Length: colorAttachmentCount
  pub pColorAttachments: *const VkAttachmentReference,
  /// Optional: true,  Length: colorAttachmentCount
  pub pResolveAttachments: *const VkAttachmentReference,
  /// Optional: true
  pub pDepthStencilAttachment: *const VkAttachmentReference,
  /// Optional: true
  pub preserveAttachmentCount: u32,
  /// Length: preserveAttachmentCount
  pub pPreserveAttachments: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkSubpassDescription<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkSubpassDescription<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkSubpassDescription<'a> {
  pub const DEFAULT: Self = Self {
    flags: VkSubpassDescriptionFlagBits(0),
    pipelineBindPoint: VkPipelineBindPoint(0),
    inputAttachmentCount: 0,
    pInputAttachments: core::ptr::null(),
    colorAttachmentCount: 0,
    pColorAttachments: core::ptr::null(),
    pResolveAttachments: core::ptr::null(),
    pDepthStencilAttachment: core::ptr::null(),
    preserveAttachmentCount: 0,
    pPreserveAttachments: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkSubpassDescriptionFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_pipelineBindPoint(mut self, val: VkPipelineBindPoint) -> Self {
    self.pipelineBindPoint = val;
    self
  }
  #[inline]
  pub const fn with_inputAttachmentCount(mut self, val: u32) -> Self {
    self.inputAttachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pInputAttachments(mut self, val: &'a [VkAttachmentReference]) -> Self {
    self.inputAttachmentCount = val.len() as u32;
    self.pInputAttachments = val.as_ptr();
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
  pub const fn with_pColorAttachments(mut self, val: &'a [VkAttachmentReference]) -> Self {
    self.colorAttachmentCount = val.len() as u32;
    self.pColorAttachments = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pResolveAttachments(mut self, val: &'a [VkAttachmentReference]) -> Self {
    self.colorAttachmentCount = val.len() as u32;
    self.pResolveAttachments = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDepthStencilAttachment(mut self, val: *const VkAttachmentReference) -> Self {
    self.pDepthStencilAttachment = val;
    self
  }
  #[inline]
  pub const fn with_preserveAttachmentCount(mut self, val: u32) -> Self {
    self.preserveAttachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPreserveAttachments(mut self, val: &'a [u32]) -> Self {
    self.preserveAttachmentCount = val.len() as u32;
    self.pPreserveAttachments = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure every provided array constrained by `colorAttachmentCount` has the same length. Optional pointer arguments may be null, but non-null pointers must be valid for that same length and outlive any use of this struct instance.
  #[inline]
  pub const fn with_colorAttachmentCount_slices(
    mut self,
    pColorAttachments: &'a [VkAttachmentReference],
    pResolveAttachments: *const VkAttachmentReference,
  ) -> Self {
    let len = pColorAttachments.len();
    self.colorAttachmentCount = len as u32;
    self.pColorAttachments = pColorAttachments.as_ptr();
    self.pResolveAttachments = pResolveAttachments;
    self
  }
}
/// [VkSubpassDependency](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDependency.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `VkSubpassDependency2`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubpassDependency {
  pub srcSubpass: u32,
  pub dstSubpass: u32,
  /// Optional: true
  pub srcStageMask: VkPipelineStageFlags,
  /// Optional: true
  pub dstStageMask: VkPipelineStageFlags,
  /// Optional: true
  pub srcAccessMask: VkAccessFlags,
  /// Optional: true
  pub dstAccessMask: VkAccessFlags,
  /// Optional: true
  pub dependencyFlags: VkDependencyFlags,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkSubpassDependency {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkSubpassDependency {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkSubpassDependency {
  pub const DEFAULT: Self = Self {
    srcSubpass: 0,
    dstSubpass: 0,
    srcStageMask: VkPipelineStageFlagBits(0),
    dstStageMask: VkPipelineStageFlagBits(0),
    srcAccessMask: VkAccessFlagBits(0),
    dstAccessMask: VkAccessFlagBits(0),
    dependencyFlags: VkDependencyFlagBits(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_srcSubpass(mut self, val: u32) -> Self {
    self.srcSubpass = val;
    self
  }
  #[inline]
  pub const fn with_dstSubpass(mut self, val: u32) -> Self {
    self.dstSubpass = val;
    self
  }
  #[inline]
  pub const fn with_srcStageMask(mut self, val: VkPipelineStageFlags) -> Self {
    self.srcStageMask = val;
    self
  }
  #[inline]
  pub const fn with_dstStageMask(mut self, val: VkPipelineStageFlags) -> Self {
    self.dstStageMask = val;
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
  pub const fn with_dependencyFlags(mut self, val: VkDependencyFlags) -> Self {
    self.dependencyFlags = val;
    self
  }
}
/// [VkRenderPassCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `VkRenderPassCreateInfo2`")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkRenderPassCreateFlags,
  /// Optional: true
  pub attachmentCount: u32,
  /// Length: attachmentCount
  pub pAttachments: *const VkAttachmentDescription,
  pub subpassCount: u32,
  /// Length: subpassCount
  pub pSubpasses: *const VkSubpassDescription<'a>,
  /// Optional: true
  pub dependencyCount: u32,
  /// Length: dependencyCount
  pub pDependencies: *const VkSubpassDependency,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkRenderPassCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkRenderPassCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkRenderPassCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RENDER_PASS_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkRenderPassCreateFlagBits(0),
    attachmentCount: 0,
    pAttachments: core::ptr::null(),
    subpassCount: 0,
    pSubpasses: core::ptr::null(),
    dependencyCount: 0,
    pDependencies: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkRenderPassCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_attachmentCount(mut self, val: u32) -> Self {
    self.attachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAttachments(mut self, val: &'a [VkAttachmentDescription]) -> Self {
    self.attachmentCount = val.len() as u32;
    self.pAttachments = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_subpassCount(mut self, val: u32) -> Self {
    self.subpassCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSubpasses(mut self, val: &'a [VkSubpassDescription<'a>]) -> Self {
    self.subpassCount = val.len() as u32;
    self.pSubpasses = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_dependencyCount(mut self, val: u32) -> Self {
    self.dependencyCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDependencies(mut self, val: &'a [VkSubpassDependency]) -> Self {
    self.dependencyCount = val.len() as u32;
    self.pDependencies = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_density_map")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassFragmentDensityMapCreateInfoEXT<'child>(
    mut self,
    val: &'a VkRenderPassFragmentDensityMapCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkRenderPassFragmentDensityMapCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassInputAttachmentAspectCreateInfo<'child>(
    mut self,
    val: &'a VkRenderPassInputAttachmentAspectCreateInfo<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkRenderPassInputAttachmentAspectCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassMultiviewCreateInfo<'child>(
    mut self,
    val: &'a VkRenderPassMultiviewCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassMultiviewCreateInfo<'child>).cast::<c_void>();
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
  #[cfg(all(
    feature = "VK_QCOM_tile_memory_heap",
    feature = "VK_QCOM_tile_properties"
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTileMemorySizeInfoQCOM<'child>(
    mut self,
    val: &'a VkTileMemorySizeInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkTileMemorySizeInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassCreateInfo<
    'root,
    T: VkPNextExtends<VkRenderPassCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkFramebufferCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferCreateInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFramebufferCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkFramebufferCreateFlags,
  pub renderPass: VkRenderPass,
  /// Optional: true
  pub attachmentCount: u32,
  /// Length: attachmentCount,  No Auto-Validity
  pub pAttachments: *const VkImageView,
  pub width: u32,
  pub height: u32,
  pub layers: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Send for VkFramebufferCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'a> Sync for VkFramebufferCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'a> VkFramebufferCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::FRAMEBUFFER_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkFramebufferCreateFlagBits(0),
    renderPass: VkRenderPass::DEFAULT,
    attachmentCount: 0,
    pAttachments: core::ptr::null(),
    width: 0,
    height: 0,
    layers: 0,
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
  pub const fn with_flags(mut self, val: VkFramebufferCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_renderPass(mut self, val: VkRenderPass) -> Self {
    self.renderPass = val;
    self
  }
  #[inline]
  pub const fn with_attachmentCount(mut self, val: u32) -> Self {
    self.attachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAttachments(mut self, val: &'a [VkImageView]) -> Self {
    self.attachmentCount = val.len() as u32;
    self.pAttachments = val.as_ptr();
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
  pub const fn with_layers(mut self, val: u32) -> Self {
    self.layers = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFramebufferAttachmentsCreateInfo<'child>(
    mut self,
    val: &'a VkFramebufferAttachmentsCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkFramebufferAttachmentsCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkFramebufferCreateInfo<
    'root,
    T: VkPNextExtends<VkFramebufferCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDrawIndirectCommand](https://docs.vulkan.org/refpages/latest/refpages/source/VkDrawIndirectCommand.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDrawIndirectCommand {
  pub vertexCount: u32,
  pub instanceCount: u32,
  pub firstVertex: u32,
  /// No Auto-Validity
  pub firstInstance: u32,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkDrawIndirectCommand {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkDrawIndirectCommand {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkDrawIndirectCommand {
  pub const DEFAULT: Self = Self {
    vertexCount: 0,
    instanceCount: 0,
    firstVertex: 0,
    firstInstance: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_vertexCount(mut self, val: u32) -> Self {
    self.vertexCount = val;
    self
  }
  #[inline]
  pub const fn with_instanceCount(mut self, val: u32) -> Self {
    self.instanceCount = val;
    self
  }
  #[inline]
  pub const fn with_firstVertex(mut self, val: u32) -> Self {
    self.firstVertex = val;
    self
  }
  #[inline]
  pub const fn with_firstInstance(mut self, val: u32) -> Self {
    self.firstInstance = val;
    self
  }
}
/// [VkDrawIndexedIndirectCommand](https://docs.vulkan.org/refpages/latest/refpages/source/VkDrawIndexedIndirectCommand.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDrawIndexedIndirectCommand {
  pub indexCount: u32,
  pub instanceCount: u32,
  pub firstIndex: u32,
  pub vertexOffset: i32,
  /// No Auto-Validity
  pub firstInstance: u32,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Send for VkDrawIndexedIndirectCommand {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl Sync for VkDrawIndexedIndirectCommand {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl VkDrawIndexedIndirectCommand {
  pub const DEFAULT: Self = Self {
    indexCount: 0,
    instanceCount: 0,
    firstIndex: 0,
    vertexOffset: 0,
    firstInstance: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_indexCount(mut self, val: u32) -> Self {
    self.indexCount = val;
    self
  }
  #[inline]
  pub const fn with_instanceCount(mut self, val: u32) -> Self {
    self.instanceCount = val;
    self
  }
  #[inline]
  pub const fn with_firstIndex(mut self, val: u32) -> Self {
    self.firstIndex = val;
    self
  }
  #[inline]
  pub const fn with_vertexOffset(mut self, val: i32) -> Self {
    self.vertexOffset = val;
    self
  }
  #[inline]
  pub const fn with_firstInstance(mut self, val: u32) -> Self {
    self.firstInstance = val;
    self
  }
}
