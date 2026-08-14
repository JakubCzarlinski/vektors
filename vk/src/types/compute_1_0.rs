use crate::consts::VK_UUID_SIZE;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_custom_border_color"
))]
use crate::enums::VkBorderColor;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::enums::VkCompareOp;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_descriptor_indexing",
  feature = "VK_VALVE_mutable_descriptor_type",
  feature = "VK_EXT_mutable_descriptor_type"
))]
use crate::enums::VkDescriptorPoolCreateFlagBits;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_KHR_push_descriptor",
  feature = "VK_EXT_descriptor_indexing",
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_VALVE_mutable_descriptor_type",
  feature = "VK_NV_device_generated_commands_compute",
  feature = "VK_EXT_mutable_descriptor_type",
  feature = "VK_NV_per_stage_descriptor_set"
))]
use crate::enums::VkDescriptorSetLayoutCreateFlagBits;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_inline_uniform_block",
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing",
  feature = "VK_VALVE_mutable_descriptor_type",
  feature = "VK_QCOM_image_processing",
  feature = "VK_EXT_mutable_descriptor_type"
))]
use crate::enums::VkDescriptorType;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_KHR_synchronization2"
))]
use crate::enums::VkEventCreateFlagBits;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_IMG_filter_cubic",
  feature = "VK_EXT_filter_cubic"
))]
use crate::enums::VkFilter;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageLayout;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_pipeline_creation_cache_control"
))]
use crate::enums::VkPipelineCacheCreateFlagBits;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::enums::VkPipelineCacheHeaderVersion;
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
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_graphics_pipeline_library",
  all(
    feature = "VK_EXT_mesh_shader",
    feature = "VK_EXT_shader_object",
    feature = "VK_KHR_maintenance11"
  ),
  all(
    feature = "VK_EXT_shader_object",
    feature = "VK_KHR_maintenance11",
    feature = "VK_NV_mesh_shader"
  )
))]
use crate::enums::VkPipelineLayoutCreateFlagBits;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::enums::VkPipelineShaderStageCreateFlagBits;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_KHR_sampler_mirror_clamp_to_edge"
))]
use crate::enums::VkSamplerAddressMode;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_fragment_density_map",
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_EXT_non_seamless_cube_map",
  feature = "VK_QCOM_image_processing"
))]
use crate::enums::VkSamplerCreateFlagBits;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::enums::VkSamplerMipmapMode;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkShaderStageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(all(feature = "VK_EXT_descriptor_buffer", feature = "VK_KHR_maintenance6"))]
use crate::types::VkBindDescriptorBufferEmbeddedSamplersInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkBindDescriptorSetsInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBuffer;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkBufferUsageFlags2CreateInfo;
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
use crate::types::VkComputePipelineIndirectBufferInfoNV;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineCreateInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphProcessingEngineCreateInfoARM;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsObjectNameInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkDescriptorPoolInlineUniformBlockCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkDescriptorSetLayoutBindingFlagsCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkDescriptorSetVariableDescriptorCountAllocateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkExportMetalObjectCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageView;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkImportMetalSharedEventInfoEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsLayoutCreateInfoEXT;
#[cfg(feature = "VK_EXT_mutable_descriptor_type")]
use crate::types::VkMutableDescriptorTypeCreateInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkOpaqueCaptureDescriptorDataCreateInfoEXT;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPipelineBinaryInfoKHR;
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
use crate::types::VkPipelineCompilerControlCreateInfoAMD;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPipelineCreateFlags2CreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPipelineCreationFeedbackCreateInfo;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPipelineOfflineCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPipelineRobustnessCreateInfo;
#[cfg(feature = "VK_EXT_shader_module_identifier")]
use crate::types::VkPipelineShaderStageModuleIdentifierCreateInfoEXT;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkPipelineShaderStageNodeCreateInfoAMDX;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPipelineShaderStageRequiredSubgroupSizeCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPushConstantsInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPushDescriptorSetInfo;
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", not(feature = "VKSC_VERSION_1_0")))]
use crate::types::VkPushDescriptorSetWithTemplateInfo;
#[cfg(feature = "VK_QCOM_image_processing2")]
use crate::types::VkSamplerBlockMatchWindowCreateInfoQCOM;
#[cfg(feature = "VK_EXT_border_color_swizzle")]
use crate::types::VkSamplerBorderColorComponentMappingCreateInfoEXT;
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
use crate::types::VkSamplerCubicWeightsCreateInfoQCOM;
#[cfg(feature = "VK_EXT_custom_border_color")]
use crate::types::VkSamplerCustomBorderColorCreateInfoEXT;
#[cfg(all(
  feature = "VK_EXT_custom_border_color",
  feature = "VK_EXT_descriptor_heap"
))]
use crate::types::VkSamplerCustomBorderColorIndexCreateInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkSamplerReductionModeCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkSamplerYcbcrConversionInfo;
#[cfg(all(feature = "VK_EXT_descriptor_buffer", feature = "VK_KHR_maintenance6"))]
use crate::types::VkSetDescriptorBufferOffsetsInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkShaderDescriptorSetAndBindingMappingInfoEXT;
#[cfg(feature = "VK_EXT_validation_cache")]
use crate::types::VkShaderModuleValidationCacheCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkShaderStageFlags;
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
use crate::types::VkSubpassShadingPipelineCreateInfoHUAWEI;
#[cfg(feature = "VK_EXT_validation_features")]
use crate::types::VkValidationFeaturesEXT;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkWriteDescriptorSetAccelerationStructureKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkWriteDescriptorSetAccelerationStructureNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkWriteDescriptorSetInlineUniformBlock;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkWriteDescriptorSetPartitionedAccelerationStructureNV;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkWriteDescriptorSetTensorARM;
use core::ffi::{c_char, c_void};
/// [VkSamplerCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCreateFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type VkSamplerCreateFlags = VkSamplerCreateFlagBits;
/// [VkPipelineLayoutCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayoutCreateFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type VkPipelineLayoutCreateFlags = VkPipelineLayoutCreateFlagBits;
/// [VkPipelineCacheCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheCreateFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type VkPipelineCacheCreateFlags = VkPipelineCacheCreateFlagBits;
/// [VkPipelineShaderStageCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageCreateFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type VkPipelineShaderStageCreateFlags = VkPipelineShaderStageCreateFlagBits;
/// [VkDescriptorSetLayoutCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutCreateFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type VkDescriptorSetLayoutCreateFlags = VkDescriptorSetLayoutCreateFlagBits;
/// [VkBufferViewCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferViewCreateFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type VkBufferViewCreateFlags = VkFlags;
/// [VkPipelineCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type VkPipelineCreateFlags = VkPipelineCreateFlagBits;
/// [VkShaderModuleCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModuleCreateFlags.html)
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
pub type VkShaderModuleCreateFlags = VkFlags;
/// [VkEventCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkEventCreateFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type VkEventCreateFlags = VkEventCreateFlagBits;
/// [VkDescriptorPoolCreateFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolCreateFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type VkDescriptorPoolCreateFlags = VkDescriptorPoolCreateFlagBits;
/// [VkDescriptorPoolResetFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolResetFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type VkDescriptorPoolResetFlags = VkFlags;
/// [VkBufferView](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferView.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkBufferView(pub *mut c_void);
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkBufferView {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl Default for VkBufferView {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkBufferView {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkBufferView {}
/// [VkShaderModule](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModule.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkShaderModule(pub *mut c_void);
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkShaderModule {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl Default for VkShaderModule {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkShaderModule {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkShaderModule {}
/// [VkPipeline](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipeline.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkPipeline(pub *mut c_void);
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkPipeline {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl Default for VkPipeline {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkPipeline {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkPipeline {}
/// [VkPipelineLayout](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayout.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkPipelineLayout(pub *mut c_void);
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkPipelineLayout {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl Default for VkPipelineLayout {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkPipelineLayout {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkPipelineLayout {}
/// [VkSampler](https://docs.vulkan.org/refpages/latest/refpages/source/VkSampler.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkSampler(pub *mut c_void);
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkSampler {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl Default for VkSampler {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkSampler {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkSampler {}
/// [VkDescriptorSet](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSet.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDescriptorSet(pub *mut c_void);
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkDescriptorSet {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl Default for VkDescriptorSet {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkDescriptorSet {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkDescriptorSet {}
/// [VkDescriptorSetLayout](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayout.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDescriptorSetLayout(pub *mut c_void);
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkDescriptorSetLayout {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl Default for VkDescriptorSetLayout {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkDescriptorSetLayout {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkDescriptorSetLayout {}
/// [VkDescriptorPool](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPool.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDescriptorPool(pub *mut c_void);
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkDescriptorPool {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl Default for VkDescriptorPool {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkDescriptorPool {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkDescriptorPool {}
/// [VkEvent](https://docs.vulkan.org/refpages/latest/refpages/source/VkEvent.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkEvent(pub *mut c_void);
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkEvent {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl Default for VkEvent {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkEvent {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkEvent {}
/// [VkPipelineCache](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCache.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkPipelineCache(pub *mut c_void);
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkPipelineCache {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl Default for VkPipelineCache {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkPipelineCache {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkPipelineCache {}
/// [VkDescriptorBufferInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBufferInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorBufferInfo {
  /// Optional: true
  pub buffer: VkBuffer,
  pub offset: VkDeviceSize,
  pub range: VkDeviceSize,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkDescriptorBufferInfo {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkDescriptorBufferInfo {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkDescriptorBufferInfo {
  pub const DEFAULT: Self = Self {
    buffer: VkBuffer::DEFAULT,
    offset: 0,
    range: 0,
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
  #[inline]
  pub const fn with_range(mut self, val: VkDeviceSize) -> Self {
    self.range = val;
    self
  }
}
/// [VkDescriptorImageInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorImageInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorImageInfo {
  /// No Auto-Validity
  pub sampler: VkSampler,
  /// No Auto-Validity
  pub imageView: VkImageView,
  /// No Auto-Validity
  pub imageLayout: VkImageLayout,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkDescriptorImageInfo {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkDescriptorImageInfo {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkDescriptorImageInfo {
  pub const DEFAULT: Self = Self {
    sampler: VkSampler::DEFAULT,
    imageView: VkImageView::DEFAULT,
    imageLayout: VkImageLayout(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_sampler(mut self, val: VkSampler) -> Self {
    self.sampler = val;
    self
  }
  #[inline]
  pub const fn with_imageView(mut self, val: VkImageView) -> Self {
    self.imageView = val;
    self
  }
  #[inline]
  pub const fn with_imageLayout(mut self, val: VkImageLayout) -> Self {
    self.imageLayout = val;
    self
  }
}
/// [VkWriteDescriptorSet](https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSet.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkWriteDescriptorSet<'a> {
  /// Values: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// No Auto-Validity
  pub dstSet: VkDescriptorSet,
  pub dstBinding: u32,
  pub dstArrayElement: u32,
  pub descriptorCount: u32,
  pub descriptorType: VkDescriptorType,
  /// Length: descriptorCount,  No Auto-Validity
  pub pImageInfo: *const VkDescriptorImageInfo,
  /// Length: descriptorCount,  No Auto-Validity
  pub pBufferInfo: *const VkDescriptorBufferInfo,
  /// Length: descriptorCount,  No Auto-Validity
  pub pTexelBufferView: *const VkBufferView,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkWriteDescriptorSet<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkWriteDescriptorSet<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkWriteDescriptorSet<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::WRITE_DESCRIPTOR_SET,
    pNext: core::ptr::null(),
    dstSet: VkDescriptorSet::DEFAULT,
    dstBinding: 0,
    dstArrayElement: 0,
    descriptorCount: 0,
    descriptorType: VkDescriptorType(0),
    pImageInfo: core::ptr::null(),
    pBufferInfo: core::ptr::null(),
    pTexelBufferView: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_dstSet(mut self, val: VkDescriptorSet) -> Self {
    self.dstSet = val;
    self
  }
  #[inline]
  pub const fn with_dstBinding(mut self, val: u32) -> Self {
    self.dstBinding = val;
    self
  }
  #[inline]
  pub const fn with_dstArrayElement(mut self, val: u32) -> Self {
    self.dstArrayElement = val;
    self
  }
  #[inline]
  pub const fn with_descriptorCount(mut self, val: u32) -> Self {
    self.descriptorCount = val;
    self
  }
  #[inline]
  pub const fn with_descriptorType(mut self, val: VkDescriptorType) -> Self {
    self.descriptorType = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pImageInfo(mut self, val: &'a [VkDescriptorImageInfo]) -> Self {
    self.descriptorCount = val.len() as u32;
    self.pImageInfo = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pBufferInfo(mut self, val: &'a [VkDescriptorBufferInfo]) -> Self {
    self.descriptorCount = val.len() as u32;
    self.pBufferInfo = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pTexelBufferView(mut self, val: &'a [VkBufferView]) -> Self {
    self.descriptorCount = val.len() as u32;
    self.pTexelBufferView = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure every provided array constrained by `descriptorCount` has the same length. Optional pointer arguments may be null, but non-null pointers must be valid for that same length and outlive any use of this struct instance.
  #[inline]
  pub const fn with_descriptorCount_slices(
    mut self,
    pImageInfo: &'a [VkDescriptorImageInfo],
    pBufferInfo: &'a [VkDescriptorBufferInfo],
    pTexelBufferView: &'a [VkBufferView],
  ) -> Self {
    let len = pImageInfo.len();
    self.descriptorCount = len as u32;
    self.pImageInfo = pImageInfo.as_ptr();
    self.pBufferInfo = pBufferInfo.as_ptr();
    self.pTexelBufferView = pTexelBufferView.as_ptr();
    self
  }
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkWriteDescriptorSetAccelerationStructureKHR<'child>(
    mut self,
    val: &'a VkWriteDescriptorSetAccelerationStructureKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkWriteDescriptorSetAccelerationStructureKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_ray_tracing")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkWriteDescriptorSetAccelerationStructureNV<'child>(
    mut self,
    val: &'a VkWriteDescriptorSetAccelerationStructureNV<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkWriteDescriptorSetAccelerationStructureNV<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkWriteDescriptorSetInlineUniformBlock<'child>(
    mut self,
    val: &'a VkWriteDescriptorSetInlineUniformBlock<'child>,
  ) -> Self {
    self.pNext = (val as *const VkWriteDescriptorSetInlineUniformBlock<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkWriteDescriptorSetPartitionedAccelerationStructureNV<'child>(
    mut self,
    val: &'a VkWriteDescriptorSetPartitionedAccelerationStructureNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkWriteDescriptorSetPartitionedAccelerationStructureNV<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkWriteDescriptorSetTensorARM<'child>(
    mut self,
    val: &'a VkWriteDescriptorSetTensorARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkWriteDescriptorSetTensorARM<'child>).cast::<c_void>();
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
/// [VkCopyDescriptorSet](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyDescriptorSet.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyDescriptorSet<'a> {
  /// Values: VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcSet: VkDescriptorSet,
  pub srcBinding: u32,
  pub srcArrayElement: u32,
  pub dstSet: VkDescriptorSet,
  pub dstBinding: u32,
  pub dstArrayElement: u32,
  pub descriptorCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkCopyDescriptorSet<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkCopyDescriptorSet<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkCopyDescriptorSet<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COPY_DESCRIPTOR_SET,
    pNext: core::ptr::null(),
    srcSet: VkDescriptorSet::DEFAULT,
    srcBinding: 0,
    srcArrayElement: 0,
    dstSet: VkDescriptorSet::DEFAULT,
    dstBinding: 0,
    dstArrayElement: 0,
    descriptorCount: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_srcSet(mut self, val: VkDescriptorSet) -> Self {
    self.srcSet = val;
    self
  }
  #[inline]
  pub const fn with_srcBinding(mut self, val: u32) -> Self {
    self.srcBinding = val;
    self
  }
  #[inline]
  pub const fn with_srcArrayElement(mut self, val: u32) -> Self {
    self.srcArrayElement = val;
    self
  }
  #[inline]
  pub const fn with_dstSet(mut self, val: VkDescriptorSet) -> Self {
    self.dstSet = val;
    self
  }
  #[inline]
  pub const fn with_dstBinding(mut self, val: u32) -> Self {
    self.dstBinding = val;
    self
  }
  #[inline]
  pub const fn with_dstArrayElement(mut self, val: u32) -> Self {
    self.dstArrayElement = val;
    self
  }
  #[inline]
  pub const fn with_descriptorCount(mut self, val: u32) -> Self {
    self.descriptorCount = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCopyDescriptorSet<
    'root,
    T: VkPNextExtends<VkCopyDescriptorSet<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBufferViewCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferViewCreateInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferViewCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkBufferViewCreateFlags,
  pub buffer: VkBuffer,
  pub format: VkFormat,
  pub offset: VkDeviceSize,
  pub range: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkBufferViewCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkBufferViewCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkBufferViewCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_VIEW_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    buffer: VkBuffer::DEFAULT,
    format: VkFormat(0),
    offset: 0,
    range: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkBufferViewCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_buffer(mut self, val: VkBuffer) -> Self {
    self.buffer = val;
    self
  }
  #[inline]
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[inline]
  pub const fn with_offset(mut self, val: VkDeviceSize) -> Self {
    self.offset = val;
    self
  }
  #[inline]
  pub const fn with_range(mut self, val: VkDeviceSize) -> Self {
    self.range = val;
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
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferViewCreateInfo<
    'root,
    T: VkPNextExtends<VkBufferViewCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkShaderModuleCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModuleCreateInfo.html)
///
/// **Extends:** VkPipelineShaderStageCreateInfo, VkDataGraphPipelineCreateInfoARM.
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkShaderModuleCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkShaderModuleCreateFlags,
  pub codeSize: usize,
  /// Length: latexmath:[\textrm{codeSize} \over 4]
  pub pCode: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkShaderModuleCreateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkShaderModuleCreateInfo<'a> {}
#[cfg(all(
  all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")),
  feature = "VK_COMPUTE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineShaderStageCreateInfo<'root>>
  for VkShaderModuleCreateInfo<'child>
{
}
#[cfg(all(
  all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")),
  feature = "VK_ARM_data_graph"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>
  for VkShaderModuleCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkShaderModuleCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SHADER_MODULE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: 0,
    codeSize: 0,
    pCode: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkShaderModuleCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_codeSize(mut self, val: usize) -> Self {
    self.codeSize = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCode(mut self, val: &[u32]) -> Self {
    self.codeSize = val.len() as usize * 4;
    self.pCode = val.as_ptr().cast::<u32>();
    self
  }
  #[cfg(feature = "VK_EXT_validation_cache")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkShaderModuleValidationCacheCreateInfoEXT<'child>(
    mut self,
    val: &'a VkShaderModuleValidationCacheCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkShaderModuleValidationCacheCreateInfoEXT<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineShaderStageCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineShaderStageCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineCreateInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDescriptorSetLayoutBinding](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutBinding.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorSetLayoutBinding<'a> {
  pub binding: u32,
  pub descriptorType: VkDescriptorType,
  /// Optional: true
  pub descriptorCount: u32,
  /// No Auto-Validity
  pub stageFlags: VkShaderStageFlags,
  /// Optional: true,  Length: descriptorCount,  No Auto-Validity
  pub pImmutableSamplers: *const VkSampler,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkDescriptorSetLayoutBinding<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkDescriptorSetLayoutBinding<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkDescriptorSetLayoutBinding<'a> {
  pub const DEFAULT: Self = Self {
    binding: 0,
    descriptorType: VkDescriptorType(0),
    descriptorCount: 0,
    stageFlags: VkShaderStageFlagBits(0),
    pImmutableSamplers: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_descriptorType(mut self, val: VkDescriptorType) -> Self {
    self.descriptorType = val;
    self
  }
  #[inline]
  pub const fn with_descriptorCount(mut self, val: u32) -> Self {
    self.descriptorCount = val;
    self
  }
  #[inline]
  pub const fn with_stageFlags(mut self, val: VkShaderStageFlags) -> Self {
    self.stageFlags = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pImmutableSamplers(mut self, val: &'a [VkSampler]) -> Self {
    self.descriptorCount = val.len() as u32;
    self.pImmutableSamplers = val.as_ptr();
    self
  }
}
/// [VkDescriptorSetLayoutCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutCreateInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorSetLayoutCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkDescriptorSetLayoutCreateFlags,
  /// Optional: true
  pub bindingCount: u32,
  /// Length: bindingCount
  pub pBindings: *const VkDescriptorSetLayoutBinding<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkDescriptorSetLayoutCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkDescriptorSetLayoutCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkDescriptorSetLayoutCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkDescriptorSetLayoutCreateFlagBits(0),
    bindingCount: 0,
    pBindings: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkDescriptorSetLayoutCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_bindingCount(mut self, val: u32) -> Self {
    self.bindingCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pBindings(mut self, val: &'a [VkDescriptorSetLayoutBinding<'a>]) -> Self {
    self.bindingCount = val.len() as u32;
    self.pBindings = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDescriptorSetLayoutBindingFlagsCreateInfo<'child>(
    mut self,
    val: &'a VkDescriptorSetLayoutBindingFlagsCreateInfo<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDescriptorSetLayoutBindingFlagsCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_mutable_descriptor_type")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMutableDescriptorTypeCreateInfoEXT<'child>(
    mut self,
    val: &'a VkMutableDescriptorTypeCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMutableDescriptorTypeCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDescriptorSetLayoutCreateInfo<
    'root,
    T: VkPNextExtends<VkDescriptorSetLayoutCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDescriptorPoolSize](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolSize.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorPoolSize {
  pub type_: VkDescriptorType,
  pub descriptorCount: u32,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkDescriptorPoolSize {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkDescriptorPoolSize {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkDescriptorPoolSize {
  pub const DEFAULT: Self = Self {
    type_: VkDescriptorType(0),
    descriptorCount: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_type(mut self, val: VkDescriptorType) -> Self {
    self.type_ = val;
    self
  }
  #[inline]
  pub const fn with_descriptorCount(mut self, val: u32) -> Self {
    self.descriptorCount = val;
    self
  }
}
/// [VkDescriptorPoolCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolCreateInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorPoolCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkDescriptorPoolCreateFlags,
  pub maxSets: u32,
  /// Optional: true
  pub poolSizeCount: u32,
  /// Length: poolSizeCount
  pub pPoolSizes: *const VkDescriptorPoolSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkDescriptorPoolCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkDescriptorPoolCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkDescriptorPoolCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DESCRIPTOR_POOL_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkDescriptorPoolCreateFlagBits(0),
    maxSets: 0,
    poolSizeCount: 0,
    pPoolSizes: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkDescriptorPoolCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_maxSets(mut self, val: u32) -> Self {
    self.maxSets = val;
    self
  }
  #[inline]
  pub const fn with_poolSizeCount(mut self, val: u32) -> Self {
    self.poolSizeCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPoolSizes(mut self, val: &'a [VkDescriptorPoolSize]) -> Self {
    self.poolSizeCount = val.len() as u32;
    self.pPoolSizes = val.as_ptr();
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
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDescriptorPoolInlineUniformBlockCreateInfo<'child>(
    mut self,
    val: &'a VkDescriptorPoolInlineUniformBlockCreateInfo<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDescriptorPoolInlineUniformBlockCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_mutable_descriptor_type")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMutableDescriptorTypeCreateInfoEXT<'child>(
    mut self,
    val: &'a VkMutableDescriptorTypeCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMutableDescriptorTypeCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDescriptorPoolCreateInfo<
    'root,
    T: VkPNextExtends<VkDescriptorPoolCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDescriptorSetAllocateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetAllocateInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorSetAllocateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub descriptorPool: VkDescriptorPool,
  pub descriptorSetCount: u32,
  /// Length: descriptorSetCount
  pub pSetLayouts: *const VkDescriptorSetLayout,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkDescriptorSetAllocateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkDescriptorSetAllocateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkDescriptorSetAllocateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
    pNext: core::ptr::null(),
    descriptorPool: VkDescriptorPool::DEFAULT,
    descriptorSetCount: 0,
    pSetLayouts: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_descriptorPool(mut self, val: VkDescriptorPool) -> Self {
    self.descriptorPool = val;
    self
  }
  #[inline]
  pub const fn with_descriptorSetCount(mut self, val: u32) -> Self {
    self.descriptorSetCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSetLayouts(mut self, val: &'a [VkDescriptorSetLayout]) -> Self {
    self.descriptorSetCount = val.len() as u32;
    self.pSetLayouts = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDescriptorSetVariableDescriptorCountAllocateInfo<'child>(
    mut self,
    val: &'a VkDescriptorSetVariableDescriptorCountAllocateInfo<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDescriptorSetVariableDescriptorCountAllocateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDescriptorSetAllocateInfo<
    'root,
    T: VkPNextExtends<VkDescriptorSetAllocateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSpecializationMapEntry](https://docs.vulkan.org/refpages/latest/refpages/source/VkSpecializationMapEntry.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSpecializationMapEntry {
  pub constantID: u32,
  pub offset: u32,
  /// No Auto-Validity
  pub size: usize,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkSpecializationMapEntry {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkSpecializationMapEntry {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkSpecializationMapEntry {
  pub const DEFAULT: Self = Self {
    constantID: 0,
    offset: 0,
    size: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_constantID(mut self, val: u32) -> Self {
    self.constantID = val;
    self
  }
  #[inline]
  pub const fn with_offset(mut self, val: u32) -> Self {
    self.offset = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: usize) -> Self {
    self.size = val;
    self
  }
}
/// [VkSpecializationInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSpecializationInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSpecializationInfo<'a> {
  /// Optional: true
  pub mapEntryCount: u32,
  /// Length: mapEntryCount
  pub pMapEntries: *const VkSpecializationMapEntry,
  /// Optional: true
  pub dataSize: usize,
  /// Length: dataSize
  pub pData: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkSpecializationInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkSpecializationInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkSpecializationInfo<'a> {
  pub const DEFAULT: Self = Self {
    mapEntryCount: 0,
    pMapEntries: core::ptr::null(),
    dataSize: 0,
    pData: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_mapEntryCount(mut self, val: u32) -> Self {
    self.mapEntryCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pMapEntries(mut self, val: &'a [VkSpecializationMapEntry]) -> Self {
    self.mapEntryCount = val.len() as u32;
    self.pMapEntries = val.as_ptr();
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
}
/// [VkPipelineShaderStageCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageCreateInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineShaderStageCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineShaderStageCreateFlags,
  pub stage: VkShaderStageFlagBits,
  /// Optional: true
  pub module: VkShaderModule,
  #[cfg(not(feature = "VKSC_VERSION_1_0"))]
  /// Length: null-terminated
  pub pName: *const c_char,
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// Optional: true,  Length: null-terminated
  pub pName: *const c_char,
  /// Optional: true
  pub pSpecializationInfo: *const VkSpecializationInfo<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineShaderStageCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineShaderStageCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkPipelineShaderStageCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkPipelineShaderStageCreateFlagBits(0),
    stage: VkShaderStageFlagBits(0),
    module: VkShaderModule::DEFAULT,
    #[cfg(not(feature = "VKSC_VERSION_1_0"))]
    pName: core::ptr::null(),
    #[cfg(feature = "VKSC_VERSION_1_0")]
    pName: core::ptr::null(),
    pSpecializationInfo: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkPipelineShaderStageCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_stage(mut self, val: VkShaderStageFlagBits) -> Self {
    self.stage = val;
    self
  }
  #[inline]
  pub const fn with_module(mut self, val: VkShaderModule) -> Self {
    self.module = val;
    self
  }
  #[cfg(not(feature = "VKSC_VERSION_1_0"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pName(mut self, val: *const c_char) -> Self {
    self.pName = val;
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pName(mut self, val: *const c_char) -> Self {
    self.pName = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSpecializationInfo(mut self, val: *const VkSpecializationInfo<'a>) -> Self {
    self.pSpecializationInfo = val;
    self
  }
  #[cfg(feature = "VK_EXT_debug_utils")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDebugUtilsObjectNameInfoEXT<'child>(
    mut self,
    val: &'a VkDebugUtilsObjectNameInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDebugUtilsObjectNameInfoEXT<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_EXT_shader_module_identifier")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineShaderStageModuleIdentifierCreateInfoEXT<'child>(
    mut self,
    val: &'a VkPipelineShaderStageModuleIdentifierCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineShaderStageModuleIdentifierCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMDX_shader_enqueue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineShaderStageNodeCreateInfoAMDX<'child>(
    mut self,
    val: &'a VkPipelineShaderStageNodeCreateInfoAMDX<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineShaderStageNodeCreateInfoAMDX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineShaderStageRequiredSubgroupSizeCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineShaderStageRequiredSubgroupSizeCreateInfo<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkPipelineShaderStageRequiredSubgroupSizeCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkShaderDescriptorSetAndBindingMappingInfoEXT<'child>(
    mut self,
    val: &'a VkShaderDescriptorSetAndBindingMappingInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkShaderDescriptorSetAndBindingMappingInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkShaderModuleCreateInfo<'child>(
    mut self,
    val: &'a VkShaderModuleCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkShaderModuleCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_validation_cache")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkShaderModuleValidationCacheCreateInfoEXT<'child>(
    mut self,
    val: &'a VkShaderModuleValidationCacheCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkShaderModuleValidationCacheCreateInfoEXT<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineShaderStageCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineShaderStageCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkComputePipelineCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkComputePipelineCreateInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkComputePipelineCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true,  No Auto-Validity
  pub flags: VkPipelineCreateFlags,
  pub stage: VkPipelineShaderStageCreateInfo<'a>,
  /// Optional: true
  pub layout: VkPipelineLayout,
  /// Optional: true,  No Auto-Validity
  pub basePipelineHandle: VkPipeline,
  pub basePipelineIndex: i32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkComputePipelineCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkComputePipelineCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkComputePipelineCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMPUTE_PIPELINE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkPipelineCreateFlagBits(0),
    stage: VkPipelineShaderStageCreateInfo::DEFAULT,
    layout: VkPipelineLayout::DEFAULT,
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
  pub const fn with_stage(mut self, val: VkPipelineShaderStageCreateInfo<'a>) -> Self {
    self.stage = val;
    self
  }
  #[inline]
  pub const fn with_layout(mut self, val: VkPipelineLayout) -> Self {
    self.layout = val;
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
  #[cfg(feature = "VK_NV_device_generated_commands_compute")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkComputePipelineIndirectBufferInfoNV<'child>(
    mut self,
    val: &'a VkComputePipelineIndirectBufferInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkComputePipelineIndirectBufferInfoNV<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_HUAWEI_subpass_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSubpassShadingPipelineCreateInfoHUAWEI<'child>(
    mut self,
    val: &'a VkSubpassShadingPipelineCreateInfoHUAWEI<'child>,
  ) -> Self {
    self.pNext = (val as *const VkSubpassShadingPipelineCreateInfoHUAWEI<'child>).cast::<c_void>();
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
/// [VkPipelineCacheCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheCreateInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCacheCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineCacheCreateFlags,
  #[cfg(not(feature = "VKSC_VERSION_1_0"))]
  /// Optional: true
  pub initialDataSize: usize,
  #[cfg(feature = "VKSC_VERSION_1_0")]
  pub initialDataSize: usize,
  /// Length: initialDataSize
  pub pInitialData: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineCacheCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineCacheCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkPipelineCacheCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_CACHE_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkPipelineCacheCreateFlagBits(0),
    #[cfg(not(feature = "VKSC_VERSION_1_0"))]
    initialDataSize: 0,
    #[cfg(feature = "VKSC_VERSION_1_0")]
    initialDataSize: 0,
    pInitialData: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkPipelineCacheCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[cfg(not(feature = "VKSC_VERSION_1_0"))]
  #[inline]
  pub const fn with_initialDataSize(mut self, val: usize) -> Self {
    self.initialDataSize = val;
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
  #[inline]
  pub const fn with_initialDataSize(mut self, val: usize) -> Self {
    self.initialDataSize = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pInitialData(mut self, val: &'a [u8]) -> Self {
    self.initialDataSize = val.len() as usize;
    self.pInitialData = val.as_ptr().cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineCacheCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineCacheCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPipelineCacheHeaderVersionOne](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheHeaderVersionOne.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCacheHeaderVersionOne {
  pub headerSize: u32,
  pub headerVersion: VkPipelineCacheHeaderVersion,
  pub vendorID: u32,
  pub deviceID: u32,
  pub pipelineCacheUUID: [u8; VK_UUID_SIZE as usize],
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkPipelineCacheHeaderVersionOne {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkPipelineCacheHeaderVersionOne {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkPipelineCacheHeaderVersionOne {
  pub const DEFAULT: Self = Self {
    headerSize: 0,
    headerVersion: VkPipelineCacheHeaderVersion(0),
    vendorID: 0,
    deviceID: 0,
    pipelineCacheUUID: [0u8; VK_UUID_SIZE as usize],
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_headerSize(mut self, val: u32) -> Self {
    self.headerSize = val;
    self
  }
  #[inline]
  pub const fn with_headerVersion(mut self, val: VkPipelineCacheHeaderVersion) -> Self {
    self.headerVersion = val;
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
  pub const fn with_pipelineCacheUUID(mut self, val: [u8; VK_UUID_SIZE as usize]) -> Self {
    self.pipelineCacheUUID = val;
    self
  }
}
/// [VkPushConstantRange](https://docs.vulkan.org/refpages/latest/refpages/source/VkPushConstantRange.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPushConstantRange {
  pub stageFlags: VkShaderStageFlags,
  pub offset: u32,
  pub size: u32,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkPushConstantRange {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkPushConstantRange {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkPushConstantRange {
  pub const DEFAULT: Self = Self {
    stageFlags: VkShaderStageFlagBits(0),
    offset: 0,
    size: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_stageFlags(mut self, val: VkShaderStageFlags) -> Self {
    self.stageFlags = val;
    self
  }
  #[inline]
  pub const fn with_offset(mut self, val: u32) -> Self {
    self.offset = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: u32) -> Self {
    self.size = val;
    self
  }
}
/// [VkPipelineLayoutCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayoutCreateInfo.html)
///
/// **Extends:** VkBindDescriptorSetsInfo, VkPushConstantsInfo, VkPushDescriptorSetInfo, VkPushDescriptorSetWithTemplateInfo, VkSetDescriptorBufferOffsetsInfoEXT, VkBindDescriptorBufferEmbeddedSamplersInfoEXT, VkIndirectCommandsLayoutCreateInfoEXT.
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineLayoutCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineLayoutCreateFlags,
  /// Optional: true
  pub setLayoutCount: u32,
  /// Optional: pointer required, values optional if pointer not null,  Length: setLayoutCount
  pub pSetLayouts: *const VkDescriptorSetLayout,
  /// Optional: true
  pub pushConstantRangeCount: u32,
  /// Length: pushConstantRangeCount
  pub pPushConstantRanges: *const VkPushConstantRange,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkPipelineLayoutCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkPipelineLayoutCreateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", feature = "VK_COMPUTE_VERSION_1_4"))]
unsafe impl<'child, 'root> VkPNextExtends<VkBindDescriptorSetsInfo<'root>>
  for VkPipelineLayoutCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", feature = "VK_COMPUTE_VERSION_1_4"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPushConstantsInfo<'root>>
  for VkPipelineLayoutCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", feature = "VK_COMPUTE_VERSION_1_4"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPushDescriptorSetInfo<'root>>
  for VkPipelineLayoutCreateInfo<'child>
{
}
#[cfg(all(
  feature = "VK_COMPUTE_VERSION_1_0",
  all(feature = "VK_COMPUTE_VERSION_1_4", not(feature = "VKSC_VERSION_1_0"))
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPushDescriptorSetWithTemplateInfo<'root>>
  for VkPipelineLayoutCreateInfo<'child>
{
}
#[cfg(all(
  feature = "VK_COMPUTE_VERSION_1_0",
  all(feature = "VK_EXT_descriptor_buffer", feature = "VK_KHR_maintenance6")
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSetDescriptorBufferOffsetsInfoEXT<'root>>
  for VkPipelineLayoutCreateInfo<'child>
{
}
#[cfg(all(
  feature = "VK_COMPUTE_VERSION_1_0",
  all(feature = "VK_EXT_descriptor_buffer", feature = "VK_KHR_maintenance6")
))]
unsafe impl<'child, 'root> VkPNextExtends<VkBindDescriptorBufferEmbeddedSamplersInfoEXT<'root>>
  for VkPipelineLayoutCreateInfo<'child>
{
}
#[cfg(all(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_device_generated_commands"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkIndirectCommandsLayoutCreateInfoEXT<'root>>
  for VkPipelineLayoutCreateInfo<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkPipelineLayoutCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_LAYOUT_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkPipelineLayoutCreateFlagBits(0),
    setLayoutCount: 0,
    pSetLayouts: core::ptr::null(),
    pushConstantRangeCount: 0,
    pPushConstantRanges: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkPipelineLayoutCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_setLayoutCount(mut self, val: u32) -> Self {
    self.setLayoutCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSetLayouts(mut self, val: &'a [VkDescriptorSetLayout]) -> Self {
    self.setLayoutCount = val.len() as u32;
    self.pSetLayouts = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_pushConstantRangeCount(mut self, val: u32) -> Self {
    self.pushConstantRangeCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPushConstantRanges(mut self, val: &'a [VkPushConstantRange]) -> Self {
    self.pushConstantRangeCount = val.len() as u32;
    self.pPushConstantRanges = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBindDescriptorSetsInfo<
    'root,
    T: VkPNextExtends<VkBindDescriptorSetsInfo<'root>>,
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
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPushDescriptorSetInfo<
    'root,
    T: VkPNextExtends<VkPushDescriptorSetInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", not(feature = "VKSC_VERSION_1_0")))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPushDescriptorSetWithTemplateInfo<
    'root,
    T: VkPNextExtends<VkPushDescriptorSetWithTemplateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_EXT_descriptor_buffer", feature = "VK_KHR_maintenance6"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSetDescriptorBufferOffsetsInfoEXT<
    'root,
    T: VkPNextExtends<VkSetDescriptorBufferOffsetsInfoEXT<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(all(feature = "VK_EXT_descriptor_buffer", feature = "VK_KHR_maintenance6"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBindDescriptorBufferEmbeddedSamplersInfoEXT<
    'root,
    T: VkPNextExtends<VkBindDescriptorBufferEmbeddedSamplersInfoEXT<'root>>,
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
  pub const fn with_pNext_chain_VkIndirectCommandsLayoutCreateInfoEXT<
    'root,
    T: VkPNextExtends<VkIndirectCommandsLayoutCreateInfoEXT<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSamplerCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCreateInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSamplerCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkSamplerCreateFlags,
  pub magFilter: VkFilter,
  pub minFilter: VkFilter,
  pub mipmapMode: VkSamplerMipmapMode,
  pub addressModeU: VkSamplerAddressMode,
  pub addressModeV: VkSamplerAddressMode,
  pub addressModeW: VkSamplerAddressMode,
  pub mipLodBias: f32,
  pub anisotropyEnable: VkBool32,
  pub maxAnisotropy: f32,
  pub compareEnable: VkBool32,
  /// No Auto-Validity
  pub compareOp: VkCompareOp,
  pub minLod: f32,
  pub maxLod: f32,
  /// No Auto-Validity
  pub borderColor: VkBorderColor,
  pub unnormalizedCoordinates: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkSamplerCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkSamplerCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkSamplerCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SAMPLER_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkSamplerCreateFlagBits(0),
    magFilter: VkFilter(0),
    minFilter: VkFilter(0),
    mipmapMode: VkSamplerMipmapMode(0),
    addressModeU: VkSamplerAddressMode(0),
    addressModeV: VkSamplerAddressMode(0),
    addressModeW: VkSamplerAddressMode(0),
    mipLodBias: 0.0f32,
    anisotropyEnable: 0,
    maxAnisotropy: 0.0f32,
    compareEnable: 0,
    compareOp: VkCompareOp(0),
    minLod: 0.0f32,
    maxLod: 0.0f32,
    borderColor: VkBorderColor(0),
    unnormalizedCoordinates: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkSamplerCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_magFilter(mut self, val: VkFilter) -> Self {
    self.magFilter = val;
    self
  }
  #[inline]
  pub const fn with_minFilter(mut self, val: VkFilter) -> Self {
    self.minFilter = val;
    self
  }
  #[inline]
  pub const fn with_mipmapMode(mut self, val: VkSamplerMipmapMode) -> Self {
    self.mipmapMode = val;
    self
  }
  #[inline]
  pub const fn with_addressModeU(mut self, val: VkSamplerAddressMode) -> Self {
    self.addressModeU = val;
    self
  }
  #[inline]
  pub const fn with_addressModeV(mut self, val: VkSamplerAddressMode) -> Self {
    self.addressModeV = val;
    self
  }
  #[inline]
  pub const fn with_addressModeW(mut self, val: VkSamplerAddressMode) -> Self {
    self.addressModeW = val;
    self
  }
  #[inline]
  pub const fn with_mipLodBias(mut self, val: f32) -> Self {
    self.mipLodBias = val;
    self
  }
  #[inline]
  pub const fn with_anisotropyEnable(mut self, val: VkBool32) -> Self {
    self.anisotropyEnable = val;
    self
  }
  #[inline]
  pub const fn with_maxAnisotropy(mut self, val: f32) -> Self {
    self.maxAnisotropy = val;
    self
  }
  #[inline]
  pub const fn with_compareEnable(mut self, val: VkBool32) -> Self {
    self.compareEnable = val;
    self
  }
  #[inline]
  pub const fn with_compareOp(mut self, val: VkCompareOp) -> Self {
    self.compareOp = val;
    self
  }
  #[inline]
  pub const fn with_minLod(mut self, val: f32) -> Self {
    self.minLod = val;
    self
  }
  #[inline]
  pub const fn with_maxLod(mut self, val: f32) -> Self {
    self.maxLod = val;
    self
  }
  #[inline]
  pub const fn with_borderColor(mut self, val: VkBorderColor) -> Self {
    self.borderColor = val;
    self
  }
  #[inline]
  pub const fn with_unnormalizedCoordinates(mut self, val: VkBool32) -> Self {
    self.unnormalizedCoordinates = val;
    self
  }
  #[cfg(feature = "VK_EXT_debug_utils")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDebugUtilsObjectNameInfoEXT<'child>(
    mut self,
    val: &'a VkDebugUtilsObjectNameInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDebugUtilsObjectNameInfoEXT<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_QCOM_image_processing2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSamplerBlockMatchWindowCreateInfoQCOM<'child>(
    mut self,
    val: &'a VkSamplerBlockMatchWindowCreateInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkSamplerBlockMatchWindowCreateInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_border_color_swizzle")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSamplerBorderColorComponentMappingCreateInfoEXT<'child>(
    mut self,
    val: &'a VkSamplerBorderColorComponentMappingCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkSamplerBorderColorComponentMappingCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_filter_cubic_weights")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSamplerCubicWeightsCreateInfoQCOM<'child>(
    mut self,
    val: &'a VkSamplerCubicWeightsCreateInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkSamplerCubicWeightsCreateInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_custom_border_color")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSamplerCustomBorderColorCreateInfoEXT<'child>(
    mut self,
    val: &'a VkSamplerCustomBorderColorCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkSamplerCustomBorderColorCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(
    feature = "VK_EXT_custom_border_color",
    feature = "VK_EXT_descriptor_heap"
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSamplerCustomBorderColorIndexCreateInfoEXT<'child>(
    mut self,
    val: &'a VkSamplerCustomBorderColorIndexCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkSamplerCustomBorderColorIndexCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSamplerReductionModeCreateInfo<'child>(
    mut self,
    val: &'a VkSamplerReductionModeCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkSamplerReductionModeCreateInfo<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSamplerCreateInfo<
    'root,
    T: VkPNextExtends<VkSamplerCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkClearColorValue](https://docs.vulkan.org/refpages/latest/refpages/source/VkClearColorValue.html)
///
/// // Union allowing specification of floating-point, integer, or unsigned integer color data. Actual value selected is based on image/attachment being cleared.
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearColorValue {
  pub float32: [f32; 4],
  pub int32: [i32; 4],
  pub uint32: [u32; 4],
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkClearColorValue {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkClearColorValue {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkClearColorValue {
  pub const DEFAULT: Self = unsafe {
    Self {
      float32: core::mem::zeroed::<[f32; 4]>(),
    }
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl core::fmt::Debug for VkClearColorValue {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("VkClearColorValue")
      .field("float32", unsafe { &self.float32 })
      .finish()
  }
}
/// [VkEventCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkEventCreateInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkEventCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_EVENT_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkEventCreateFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Send for VkEventCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'a> Sync for VkEventCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'a> VkEventCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::EVENT_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkEventCreateFlagBits(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkEventCreateFlags) -> Self {
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
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkEventCreateInfo<
    'root,
    T: VkPNextExtends<VkEventCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDispatchIndirectCommand](https://docs.vulkan.org/refpages/latest/refpages/source/VkDispatchIndirectCommand.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDispatchIndirectCommand {
  /// No Auto-Validity
  pub x: u32,
  /// No Auto-Validity
  pub y: u32,
  /// No Auto-Validity
  pub z: u32,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Send for VkDispatchIndirectCommand {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl Sync for VkDispatchIndirectCommand {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl VkDispatchIndirectCommand {
  pub const DEFAULT: Self = Self { x: 0, y: 0, z: 0 };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_x(mut self, val: u32) -> Self {
    self.x = val;
    self
  }
  #[inline]
  pub const fn with_y(mut self, val: u32) -> Self {
    self.y = val;
    self
  }
  #[inline]
  pub const fn with_z(mut self, val: u32) -> Self {
    self.z = val;
    self
  }
}
