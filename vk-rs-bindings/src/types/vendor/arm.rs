use crate::consts::VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM;
use crate::consts::VK_MAX_DESCRIPTION_SIZE;
use crate::consts::VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_3",
  feature = "VK_KHR_video_decode_queue",
  feature = "VK_EXT_descriptor_heap",
  feature = "VK_KHR_video_encode_queue",
  feature = "VK_QCOM_tile_shading",
  feature = "VK_KHR_synchronization2",
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_HUAWEI_invocation_mask",
  feature = "VK_EXT_opacity_micromap",
  feature = "VK_NV_optical_flow",
  feature = "VK_EXT_memory_decompression"
))]
use crate::enums::VkAccessFlagBits2;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphOpticalFlowCreateFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphOpticalFlowExecuteFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphOpticalFlowGridSizeFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphOpticalFlowImageUsageFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphOpticalFlowPerformanceLevelARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::enums::VkDataGraphPipelineDispatchFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphPipelineNodeConnectionTypeARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphPipelineNodeTypeARM;
#[cfg(any(
  feature = "VK_ARM_data_graph",
  feature = "VK_ARM_data_graph_neural_accelerator_statistics"
))]
use crate::enums::VkDataGraphPipelinePropertyARM;
#[cfg(any(
  feature = "VK_ARM_data_graph",
  feature = "VK_ARM_data_graph_neural_accelerator_statistics"
))]
use crate::enums::VkDataGraphPipelineSessionBindPointARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::enums::VkDataGraphPipelineSessionBindPointTypeARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::enums::VkDataGraphPipelineSessionCreateFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
use crate::enums::VkDataGraphTOSALevelARM;
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
use crate::enums::VkDataGraphTOSAQualityFlagBitsARM;
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
use crate::enums::VkExternalSemaphoreHandleTypeFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_3",
  feature = "VK_EXT_host_image_copy",
  all(
    feature = "VK_QCOM_image_processing",
    feature = "VK_QCOM_image_processing3"
  ),
  feature = "VK_KHR_format_feature_flags2",
  feature = "VK_NV_ray_tracing_linear_swept_spheres",
  feature = "VK_NV_optical_flow",
  feature = "VK_KHR_copy_memory_indirect",
  feature = "VK_KHR_video_encode_quantization_map"
))]
use crate::enums::VkFormatFeatureFlagBits2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageLayout;
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
use crate::enums::VkNeuralAcceleratorStatisticsModeARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::enums::VkPhysicalDeviceDataGraphOperationTypeARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::enums::VkPhysicalDeviceDataGraphProcessingEngineTypeARM;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::enums::VkPhysicalDeviceSchedulingControlsFlagBitsARM;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_4",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_KHR_maintenance5",
  feature = "VK_KHR_pipeline_binary",
  feature = "VK_EXT_device_generated_commands",
  feature = "VK_VALVE_fragment_density_map_layered",
  feature = "VK_EXT_shader_64bit_indexing"
))]
use crate::enums::VkPipelineCreateFlagBits2;
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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkShaderStageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSharingMode;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_ARM_tensors")]
use crate::enums::VkTensorCreateFlagBitsARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::enums::VkTensorTilingARM;
#[cfg(any(feature = "VK_ARM_tensors", feature = "VK_ARM_data_graph"))]
use crate::enums::VkTensorUsageFlagBitsARM;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkAccessFlags2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBindSparseInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkCommandBufferSubmitInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandPoolCreateInfo;
#[cfg(feature = "VK_QCOM_data_graph_model")]
use crate::types::VkDataGraphPipelineBuiltinModelCreateInfoQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkDependencyInfo;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkDescriptorGetInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorPoolCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceAddress;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceMemory;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceQueueCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtent2D;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalMemoryHandleTypeFlags;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalMemoryProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalSemaphoreHandleTypeFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkFormatFeatureFlags2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkFormatProperties2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryAllocateInfo;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkOpaqueCaptureDataCreateInfoEXT;
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
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPipelineCreateFlags2;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPipelineCreationFeedbackCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineLayout;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPipelineStageFlags2;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::types::VkPresentInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkRect2D;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPassBeginInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkRenderingInfo;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkSemaphoreSubmitInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkShaderModule;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkShaderModuleCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkShaderStageFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkSpecializationInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSubmitInfo;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkSubmitInfo2;
#[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
use crate::types::VkTensorARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkWriteDescriptorSet;
use core::ffi::{c_char, c_void};
/// [VkDataGraphPipelineSessionCreateFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionCreateFlagsARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
pub type VkDataGraphPipelineSessionCreateFlagsARM = VkDataGraphPipelineSessionCreateFlagBitsARM;
/// [VkDataGraphPipelineDispatchFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineDispatchFlagsARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
pub type VkDataGraphPipelineDispatchFlagsARM = VkDataGraphPipelineDispatchFlagBitsARM;
/// [VkDataGraphPipelineSessionARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDataGraphPipelineSessionARM(pub *mut c_void);
#[cfg(feature = "VK_ARM_data_graph")]
impl VkDataGraphPipelineSessionARM {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_ARM_data_graph")]
impl Default for VkDataGraphPipelineSessionARM {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl Send for VkDataGraphPipelineSessionARM {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl Sync for VkDataGraphPipelineSessionARM {}
/// [VkPhysicalDeviceDataGraphFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDataGraphFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub dataGraph: VkBool32,
  pub dataGraphUpdateAfterBind: VkBool32,
  pub dataGraphSpecializationConstants: VkBool32,
  pub dataGraphDescriptorBuffer: VkBool32,
  pub dataGraphShaderModule: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkPhysicalDeviceDataGraphFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkPhysicalDeviceDataGraphFeaturesARM<'a> {}
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDataGraphFeaturesARM<'child>
{
}
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDataGraphFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkPhysicalDeviceDataGraphFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM,
    pNext: core::ptr::null_mut(),
    dataGraph: 0,
    dataGraphUpdateAfterBind: 0,
    dataGraphSpecializationConstants: 0,
    dataGraphDescriptorBuffer: 0,
    dataGraphShaderModule: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_dataGraph(mut self, val: VkBool32) -> Self {
    self.dataGraph = val;
    self
  }
  #[inline]
  pub const fn with_dataGraphUpdateAfterBind(mut self, val: VkBool32) -> Self {
    self.dataGraphUpdateAfterBind = val;
    self
  }
  #[inline]
  pub const fn with_dataGraphSpecializationConstants(mut self, val: VkBool32) -> Self {
    self.dataGraphSpecializationConstants = val;
    self
  }
  #[inline]
  pub const fn with_dataGraphDescriptorBuffer(mut self, val: VkBool32) -> Self {
    self.dataGraphDescriptorBuffer = val;
    self
  }
  #[inline]
  pub const fn with_dataGraphShaderModule(mut self, val: VkBool32) -> Self {
    self.dataGraphShaderModule = val;
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
/// [VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM.html)
///
/// **Extends:** VkDataGraphPipelineConstantARM.
///
/// **Availability:** depends on `VK_ARM_tensors`.
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_ARM_tensors"))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub dimension: u32,
  pub zeroCount: u32,
  pub groupSize: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_ARM_tensors"))]
unsafe impl<'a> Send for VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_ARM_tensors"))]
unsafe impl<'a> Sync for VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'a> {}
#[cfg(all(
  all(feature = "VK_ARM_data_graph", feature = "VK_ARM_tensors"),
  feature = "VK_ARM_data_graph"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineConstantARM<'root>>
  for VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'child>
{
}
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_ARM_tensors"))]
impl<'a> VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'a> {
  pub const DEFAULT: Self = Self {
        sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM,
        pNext: core::ptr::null(),
        dimension: 0,
        zeroCount: 0,
        groupSize: 0,
        _marker: core::marker::PhantomData,
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
  pub const fn with_dimension(mut self, val: u32) -> Self {
    self.dimension = val;
    self
  }
  #[inline]
  pub const fn with_zeroCount(mut self, val: u32) -> Self {
    self.zeroCount = val;
    self
  }
  #[inline]
  pub const fn with_groupSize(mut self, val: u32) -> Self {
    self.groupSize = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineConstantARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineConstantARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineConstantARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineConstantARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineConstantARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_CONSTANT_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub id: u32,
  pub pConstantData: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineConstantARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineConstantARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineConstantARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_CONSTANT_ARM,
    pNext: core::ptr::null(),
    id: 0,
    pConstantData: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_id(mut self, val: u32) -> Self {
    self.id = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pConstantData(mut self, val: *const c_void) -> Self {
    self.pConstantData = val;
    self
  }
  #[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_ARM_tensors"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'child>(
    mut self,
    val: &'a VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'child>,
  ) -> Self {
    self.pNext = (val
      as *const VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTensorDescriptionARM<'child>(
    mut self,
    val: &'a VkTensorDescriptionARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkTensorDescriptionARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineConstantARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineConstantARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineResourceInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineResourceInfoARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineResourceInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub descriptorSet: u32,
  pub binding: u32,
  /// Optional: true
  pub arrayElement: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineResourceInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineResourceInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineResourceInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM,
    pNext: core::ptr::null(),
    descriptorSet: 0,
    binding: 0,
    arrayElement: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_descriptorSet(mut self, val: u32) -> Self {
    self.descriptorSet = val;
    self
  }
  #[inline]
  pub const fn with_binding(mut self, val: u32) -> Self {
    self.binding = val;
    self
  }
  #[inline]
  pub const fn with_arrayElement(mut self, val: u32) -> Self {
    self.arrayElement = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphPipelineResourceInfoImageLayoutARM<'child>(
    mut self,
    val: &'a VkDataGraphPipelineResourceInfoImageLayoutARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDataGraphPipelineResourceInfoImageLayoutARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTensorDescriptionARM<'child>(
    mut self,
    val: &'a VkTensorDescriptionARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkTensorDescriptionARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineResourceInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineResourceInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineCompilerControlCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineCompilerControlCreateInfoARM.html)
///
/// **Extends:** VkDataGraphPipelineCreateInfoARM.
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineCompilerControlCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Length: null-terminated
  pub pVendorOptions: *const c_char,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineCompilerControlCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineCompilerControlCreateInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_ARM_data_graph"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>
  for VkDataGraphPipelineCompilerControlCreateInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineCompilerControlCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM,
    pNext: core::ptr::null(),
    pVendorOptions: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_pVendorOptions(mut self, val: *const c_char) -> Self {
    self.pVendorOptions = val;
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
/// [VkDataGraphPipelineCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineCreateInfoARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineCreateFlags2,
  pub layout: VkPipelineLayout,
  /// Optional: true
  pub resourceInfoCount: u32,
  /// Length: resourceInfoCount
  pub pResourceInfos: *const VkDataGraphPipelineResourceInfoARM<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_CREATE_INFO_ARM,
    pNext: core::ptr::null(),
    flags: VkPipelineCreateFlagBits2(0),
    layout: VkPipelineLayout::DEFAULT,
    resourceInfoCount: 0,
    pResourceInfos: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkPipelineCreateFlags2) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_layout(mut self, val: VkPipelineLayout) -> Self {
    self.layout = val;
    self
  }
  #[inline]
  pub const fn with_resourceInfoCount(mut self, val: u32) -> Self {
    self.resourceInfoCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pResourceInfos(
    mut self,
    val: &'a [VkDataGraphPipelineResourceInfoARM<'a>],
  ) -> Self {
    self.resourceInfoCount = val.len() as u32;
    self.pResourceInfos = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_QCOM_data_graph_model")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphPipelineBuiltinModelCreateInfoQCOM<'child>(
    mut self,
    val: &'a VkDataGraphPipelineBuiltinModelCreateInfoQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDataGraphPipelineBuiltinModelCreateInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphPipelineCompilerControlCreateInfoARM<'child>(
    mut self,
    val: &'a VkDataGraphPipelineCompilerControlCreateInfoARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDataGraphPipelineCompilerControlCreateInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphPipelineIdentifierCreateInfoARM<'child>(
    mut self,
    val: &'a VkDataGraphPipelineIdentifierCreateInfoARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDataGraphPipelineIdentifierCreateInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphPipelineNeuralStatisticsCreateInfoARM<'child>(
    mut self,
    val: &'a VkDataGraphPipelineNeuralStatisticsCreateInfoARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDataGraphPipelineNeuralStatisticsCreateInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphPipelineOpticalFlowCreateInfoARM<'child>(
    mut self,
    val: &'a VkDataGraphPipelineOpticalFlowCreateInfoARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDataGraphPipelineOpticalFlowCreateInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphPipelineShaderModuleCreateInfoARM<'child>(
    mut self,
    val: &'a VkDataGraphPipelineShaderModuleCreateInfoARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDataGraphPipelineShaderModuleCreateInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphPipelineSingleNodeCreateInfoARM<'child>(
    mut self,
    val: &'a VkDataGraphPipelineSingleNodeCreateInfoARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkDataGraphPipelineSingleNodeCreateInfoARM<'child>).cast::<c_void>();
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
  pub const fn with_pNext_VkPipelineCreationFeedbackCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineCreationFeedbackCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineCreationFeedbackCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
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
/// [VkDataGraphPipelineShaderModuleCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineShaderModuleCreateInfoARM.html)
///
/// **Extends:** VkDataGraphPipelineCreateInfoARM.
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineShaderModuleCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub module: VkShaderModule,
  /// Length: null-terminated
  pub pName: *const c_char,
  /// Optional: true
  pub pSpecializationInfo: *const VkSpecializationInfo<'a>,
  /// Optional: true
  pub constantCount: u32,
  /// Optional: true,  Length: constantCount
  pub pConstants: *const VkDataGraphPipelineConstantARM<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineShaderModuleCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineShaderModuleCreateInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_ARM_data_graph"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>
  for VkDataGraphPipelineShaderModuleCreateInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineShaderModuleCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM,
    pNext: core::ptr::null(),
    module: VkShaderModule::DEFAULT,
    pName: core::ptr::null(),
    pSpecializationInfo: core::ptr::null(),
    constantCount: 0,
    pConstants: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_module(mut self, val: VkShaderModule) -> Self {
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
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSpecializationInfo(mut self, val: *const VkSpecializationInfo<'a>) -> Self {
    self.pSpecializationInfo = val;
    self
  }
  #[inline]
  pub const fn with_constantCount(mut self, val: u32) -> Self {
    self.constantCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pConstants(mut self, val: &'a [VkDataGraphPipelineConstantARM<'a>]) -> Self {
    self.constantCount = val.len() as u32;
    self.pConstants = val.as_ptr();
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
/// [VkDataGraphPipelineSessionCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionCreateInfoARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineSessionCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkDataGraphPipelineSessionCreateFlagsARM,
  pub dataGraphPipeline: VkPipeline,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineSessionCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineSessionCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineSessionCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM,
    pNext: core::ptr::null(),
    flags: VkDataGraphPipelineSessionCreateFlagBitsARM(0),
    dataGraphPipeline: VkPipeline::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkDataGraphPipelineSessionCreateFlagsARM) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_dataGraphPipeline(mut self, val: VkPipeline) -> Self {
    self.dataGraphPipeline = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'child>(
    mut self,
    val: &'a VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'child>)
      .cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineSessionCreateInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineSessionCreateInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineSessionBindPointRequirementsInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionBindPointRequirementsInfoARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineSessionBindPointRequirementsInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub session: VkDataGraphPipelineSessionARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineSessionBindPointRequirementsInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineSessionBindPointRequirementsInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineSessionBindPointRequirementsInfoARM<'a> {
  pub const DEFAULT: Self = Self {
        sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM,
        pNext: core::ptr::null(),
        session: VkDataGraphPipelineSessionARM::DEFAULT,
        _marker: core::marker::PhantomData,
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
  pub const fn with_session(mut self, val: VkDataGraphPipelineSessionARM) -> Self {
    self.session = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineSessionBindPointRequirementsInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineSessionBindPointRequirementsInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineSessionBindPointRequirementARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionBindPointRequirementARM.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineSessionBindPointRequirementARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub bindPoint: VkDataGraphPipelineSessionBindPointARM,
  pub bindPointType: VkDataGraphPipelineSessionBindPointTypeARM,
  pub numObjects: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineSessionBindPointRequirementARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineSessionBindPointRequirementARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineSessionBindPointRequirementARM<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM,
    pNext: core::ptr::null_mut(),
    bindPoint: VkDataGraphPipelineSessionBindPointARM(0),
    bindPointType: VkDataGraphPipelineSessionBindPointTypeARM(0),
    numObjects: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_bindPoint(mut self, val: VkDataGraphPipelineSessionBindPointARM) -> Self {
    self.bindPoint = val;
    self
  }
  #[inline]
  pub const fn with_bindPointType(
    mut self,
    val: VkDataGraphPipelineSessionBindPointTypeARM,
  ) -> Self {
    self.bindPointType = val;
    self
  }
  #[inline]
  pub const fn with_numObjects(mut self, val: u32) -> Self {
    self.numObjects = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineSessionBindPointRequirementARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineSessionBindPointRequirementARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineSessionMemoryRequirementsInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionMemoryRequirementsInfoARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineSessionMemoryRequirementsInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub session: VkDataGraphPipelineSessionARM,
  pub bindPoint: VkDataGraphPipelineSessionBindPointARM,
  pub objectIndex: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineSessionMemoryRequirementsInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineSessionMemoryRequirementsInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineSessionMemoryRequirementsInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM,
    pNext: core::ptr::null(),
    session: VkDataGraphPipelineSessionARM::DEFAULT,
    bindPoint: VkDataGraphPipelineSessionBindPointARM(0),
    objectIndex: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_session(mut self, val: VkDataGraphPipelineSessionARM) -> Self {
    self.session = val;
    self
  }
  #[inline]
  pub const fn with_bindPoint(mut self, val: VkDataGraphPipelineSessionBindPointARM) -> Self {
    self.bindPoint = val;
    self
  }
  #[inline]
  pub const fn with_objectIndex(mut self, val: u32) -> Self {
    self.objectIndex = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineSessionMemoryRequirementsInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineSessionMemoryRequirementsInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBindDataGraphPipelineSessionMemoryInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindDataGraphPipelineSessionMemoryInfoARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindDataGraphPipelineSessionMemoryInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub session: VkDataGraphPipelineSessionARM,
  pub bindPoint: VkDataGraphPipelineSessionBindPointARM,
  pub objectIndex: u32,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkBindDataGraphPipelineSessionMemoryInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkBindDataGraphPipelineSessionMemoryInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkBindDataGraphPipelineSessionMemoryInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM,
    pNext: core::ptr::null(),
    session: VkDataGraphPipelineSessionARM::DEFAULT,
    bindPoint: VkDataGraphPipelineSessionBindPointARM(0),
    objectIndex: 0,
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
  pub const fn with_session(mut self, val: VkDataGraphPipelineSessionARM) -> Self {
    self.session = val;
    self
  }
  #[inline]
  pub const fn with_bindPoint(mut self, val: VkDataGraphPipelineSessionBindPointARM) -> Self {
    self.bindPoint = val;
    self
  }
  #[inline]
  pub const fn with_objectIndex(mut self, val: u32) -> Self {
    self.objectIndex = val;
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
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBindDataGraphPipelineSessionMemoryInfoARM<
    'root,
    T: VkPNextExtends<VkBindDataGraphPipelineSessionMemoryInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineInfoARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub dataGraphPipeline: VkPipeline,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_INFO_ARM,
    pNext: core::ptr::null(),
    dataGraphPipeline: VkPipeline::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_dataGraphPipeline(mut self, val: VkPipeline) -> Self {
    self.dataGraphPipeline = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelinePropertyQueryResultARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelinePropertyQueryResultARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelinePropertyQueryResultARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub property: VkDataGraphPipelinePropertyARM,
  pub isText: VkBool32,
  /// Optional: true
  pub dataSize: usize,
  /// Optional: true,  Length: dataSize
  pub pData: *mut c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelinePropertyQueryResultARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelinePropertyQueryResultARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelinePropertyQueryResultARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM,
    pNext: core::ptr::null_mut(),
    property: VkDataGraphPipelinePropertyARM(0),
    isText: 0,
    dataSize: 0,
    pData: core::ptr::null_mut(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_property(mut self, val: VkDataGraphPipelinePropertyARM) -> Self {
    self.property = val;
    self
  }
  #[inline]
  pub const fn with_isText(mut self, val: VkBool32) -> Self {
    self.isText = val;
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
  pub const fn with_pData(mut self, val: &'a mut [u8]) -> Self {
    self.dataSize = val.len() as usize;
    self.pData = val.as_mut_ptr().cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelinePropertyQueryResultARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelinePropertyQueryResultARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineIdentifierCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineIdentifierCreateInfoARM.html)
///
/// **Extends:** VkDataGraphPipelineCreateInfoARM.
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineIdentifierCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub identifierSize: u32,
  /// Length: identifierSize
  pub pIdentifier: *const u8,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineIdentifierCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineIdentifierCreateInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_ARM_data_graph"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>
  for VkDataGraphPipelineIdentifierCreateInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineIdentifierCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM,
    pNext: core::ptr::null(),
    identifierSize: 0,
    pIdentifier: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_identifierSize(mut self, val: u32) -> Self {
    self.identifierSize = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pIdentifier(mut self, val: &'a [u8]) -> Self {
    self.identifierSize = val.len() as u32;
    self.pIdentifier = val.as_ptr();
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
/// [VkDataGraphPipelineDispatchInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineDispatchInfoARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineDispatchInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Optional: true
  pub flags: VkDataGraphPipelineDispatchFlagsARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphPipelineDispatchInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphPipelineDispatchInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphPipelineDispatchInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM,
    pNext: core::ptr::null_mut(),
    flags: VkDataGraphPipelineDispatchFlagBitsARM(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkDataGraphPipelineDispatchFlagsARM) -> Self {
    self.flags = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDataGraphPipelineOpticalFlowDispatchInfoARM<'child>(
    mut self,
    val: &'a mut VkDataGraphPipelineOpticalFlowDispatchInfoARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *mut VkDataGraphPipelineOpticalFlowDispatchInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineDispatchInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineDispatchInfoARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceDataGraphProcessingEngineARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphProcessingEngineARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDataGraphProcessingEngineARM {
  /// No Auto-Validity
  pub type_: VkPhysicalDeviceDataGraphProcessingEngineTypeARM,
  pub isForeign: VkBool32,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl Send for VkPhysicalDeviceDataGraphProcessingEngineARM {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl Sync for VkPhysicalDeviceDataGraphProcessingEngineARM {}
#[cfg(feature = "VK_ARM_data_graph")]
impl VkPhysicalDeviceDataGraphProcessingEngineARM {
  pub const DEFAULT: Self = Self {
    type_: VkPhysicalDeviceDataGraphProcessingEngineTypeARM(0),
    isForeign: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_type(mut self, val: VkPhysicalDeviceDataGraphProcessingEngineTypeARM) -> Self {
    self.type_ = val;
    self
  }
  #[inline]
  pub const fn with_isForeign(mut self, val: VkBool32) -> Self {
    self.isForeign = val;
    self
  }
}
/// [VkPhysicalDeviceDataGraphOperationSupportARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphOperationSupportARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDataGraphOperationSupportARM {
  /// No Auto-Validity
  pub operationType: VkPhysicalDeviceDataGraphOperationTypeARM,
  /// Length: null-terminated,  No Auto-Validity
  pub name: [c_char; VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM as usize],
  pub version: u32,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl Send for VkPhysicalDeviceDataGraphOperationSupportARM {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl Sync for VkPhysicalDeviceDataGraphOperationSupportARM {}
#[cfg(feature = "VK_ARM_data_graph")]
impl VkPhysicalDeviceDataGraphOperationSupportARM {
  pub const DEFAULT: Self = Self {
    operationType: VkPhysicalDeviceDataGraphOperationTypeARM(0),
    name: [0i8; VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM as usize],
    version: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_operationType(
    mut self,
    val: VkPhysicalDeviceDataGraphOperationTypeARM,
  ) -> Self {
    self.operationType = val;
    self
  }
  #[inline]
  pub const fn with_name(
    mut self,
    val: [c_char; VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM as usize],
  ) -> Self {
    self.name = val;
    self
  }
  #[inline]
  pub const fn with_version(mut self, val: u32) -> Self {
    self.version = val;
    self
  }
}
/// [VkQueueFamilyDataGraphPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyDataGraphPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueueFamilyDataGraphPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub engine: VkPhysicalDeviceDataGraphProcessingEngineARM,
  pub operation: VkPhysicalDeviceDataGraphOperationSupportARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkQueueFamilyDataGraphPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkQueueFamilyDataGraphPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkQueueFamilyDataGraphPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    engine: VkPhysicalDeviceDataGraphProcessingEngineARM::DEFAULT,
    operation: VkPhysicalDeviceDataGraphOperationSupportARM::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_engine(mut self, val: VkPhysicalDeviceDataGraphProcessingEngineARM) -> Self {
    self.engine = val;
    self
  }
  #[inline]
  pub const fn with_operation(mut self, val: VkPhysicalDeviceDataGraphOperationSupportARM) -> Self {
    self.operation = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkQueueFamilyDataGraphPropertiesARM<
    'root,
    T: VkPNextExtends<VkQueueFamilyDataGraphPropertiesARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM.html)
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub queueFamilyIndex: u32,
  pub engineType: VkPhysicalDeviceDataGraphProcessingEngineTypeARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'a> {
  pub const DEFAULT: Self = Self {
        sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM,
        pNext: core::ptr::null(),
        queueFamilyIndex: 0,
        engineType: VkPhysicalDeviceDataGraphProcessingEngineTypeARM(0),
        _marker: core::marker::PhantomData,
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
  pub const fn with_queueFamilyIndex(mut self, val: u32) -> Self {
    self.queueFamilyIndex = val;
    self
  }
  #[inline]
  pub const fn with_engineType(
    mut self,
    val: VkPhysicalDeviceDataGraphProcessingEngineTypeARM,
  ) -> Self {
    self.engineType = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkQueueFamilyDataGraphProcessingEnginePropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyDataGraphProcessingEnginePropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueueFamilyDataGraphProcessingEnginePropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub foreignSemaphoreHandleTypes: VkExternalSemaphoreHandleTypeFlags,
  pub foreignMemoryHandleTypes: VkExternalMemoryHandleTypeFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkQueueFamilyDataGraphProcessingEnginePropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkQueueFamilyDataGraphProcessingEnginePropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkQueueFamilyDataGraphProcessingEnginePropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    foreignSemaphoreHandleTypes: VkExternalSemaphoreHandleTypeFlagBits(0),
    foreignMemoryHandleTypes: VkExternalMemoryHandleTypeFlagBits(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_foreignSemaphoreHandleTypes(
    mut self,
    val: VkExternalSemaphoreHandleTypeFlags,
  ) -> Self {
    self.foreignSemaphoreHandleTypes = val;
    self
  }
  #[inline]
  pub const fn with_foreignMemoryHandleTypes(
    mut self,
    val: VkExternalMemoryHandleTypeFlags,
  ) -> Self {
    self.foreignMemoryHandleTypes = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkQueueFamilyDataGraphProcessingEnginePropertiesARM<
    'root,
    T: VkPNextExtends<VkQueueFamilyDataGraphProcessingEnginePropertiesARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphProcessingEngineCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphProcessingEngineCreateInfoARM.html)
///
/// **Extends:** VkDataGraphPipelineCreateInfoARM, VkDescriptorPoolCreateInfo, VkCommandPoolCreateInfo.
#[cfg(feature = "VK_ARM_data_graph")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphProcessingEngineCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub processingEngineCount: u32,
  /// Length: processingEngineCount
  pub pProcessingEngines: *mut VkPhysicalDeviceDataGraphProcessingEngineARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Send for VkDataGraphProcessingEngineCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'a> Sync for VkDataGraphProcessingEngineCreateInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_ARM_data_graph"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>
  for VkDataGraphProcessingEngineCreateInfoARM<'child>
{
}
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDescriptorPoolCreateInfo<'root>>
  for VkDataGraphProcessingEngineCreateInfoARM<'child>
{
}
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandPoolCreateInfo<'root>>
  for VkDataGraphProcessingEngineCreateInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'a> VkDataGraphProcessingEngineCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM,
    pNext: core::ptr::null(),
    processingEngineCount: 0,
    pProcessingEngines: core::ptr::null_mut(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_processingEngineCount(mut self, val: u32) -> Self {
    self.processingEngineCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pProcessingEngines(
    mut self,
    val: &'a mut [VkPhysicalDeviceDataGraphProcessingEngineARM],
  ) -> Self {
    self.processingEngineCount = val.len() as u32;
    self.pProcessingEngines = val.as_mut_ptr();
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
/// [VkDataGraphTOSAQualityFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphTOSAQualityFlagsARM.html)
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
pub type VkDataGraphTOSAQualityFlagsARM = VkDataGraphTOSAQualityFlagBitsARM;
/// [VkDataGraphTOSANameQualityARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphTOSANameQualityARM.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphTOSANameQualityARM {
  /// Length: null-terminated,  No Auto-Validity
  pub name: [c_char; VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM as usize],
  pub qualityFlags: VkDataGraphTOSAQualityFlagsARM,
}
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
unsafe impl Send for VkDataGraphTOSANameQualityARM {}
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
unsafe impl Sync for VkDataGraphTOSANameQualityARM {}
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
impl VkDataGraphTOSANameQualityARM {
  pub const DEFAULT: Self = Self {
    name: [0i8; VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM as usize],
    qualityFlags: VkDataGraphTOSAQualityFlagBitsARM(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_name(
    mut self,
    val: [c_char; VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM as usize],
  ) -> Self {
    self.name = val;
    self
  }
  #[inline]
  pub const fn with_qualityFlags(mut self, val: VkDataGraphTOSAQualityFlagsARM) -> Self {
    self.qualityFlags = val;
    self
  }
}
/// [VkQueueFamilyDataGraphTOSAPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyDataGraphTOSAPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueueFamilyDataGraphTOSAPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub profileCount: u32,
  /// Length: profileCount
  pub pProfiles: *const VkDataGraphTOSANameQualityARM,
  pub extensionCount: u32,
  /// Length: extensionCount
  pub pExtensions: *const VkDataGraphTOSANameQualityARM,
  pub level: VkDataGraphTOSALevelARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
unsafe impl<'a> Send for VkQueueFamilyDataGraphTOSAPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
unsafe impl<'a> Sync for VkQueueFamilyDataGraphTOSAPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
impl<'a> VkQueueFamilyDataGraphTOSAPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    profileCount: 0,
    pProfiles: core::ptr::null(),
    extensionCount: 0,
    pExtensions: core::ptr::null(),
    level: VkDataGraphTOSALevelARM(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_profileCount(mut self, val: u32) -> Self {
    self.profileCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pProfiles(mut self, val: &'a [VkDataGraphTOSANameQualityARM]) -> Self {
    self.profileCount = val.len() as u32;
    self.pProfiles = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_extensionCount(mut self, val: u32) -> Self {
    self.extensionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pExtensions(mut self, val: &'a [VkDataGraphTOSANameQualityARM]) -> Self {
    self.extensionCount = val.len() as u32;
    self.pExtensions = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_level(mut self, val: VkDataGraphTOSALevelARM) -> Self {
    self.level = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkQueueFamilyDataGraphTOSAPropertiesARM<
    'root,
    T: VkPNextExtends<VkQueueFamilyDataGraphTOSAPropertiesARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub dataGraphNeuralAcceleratorStatistics: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
unsafe impl<'a> Send for VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
unsafe impl<'a> Sync for VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_data_graph_neural_accelerator_statistics",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_data_graph_neural_accelerator_statistics",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
impl<'a> VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
        sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_FEATURES_ARM,
        pNext: core::ptr::null_mut(),
        dataGraphNeuralAcceleratorStatistics: 0,
        _marker: core::marker::PhantomData,
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
  pub const fn with_dataGraphNeuralAcceleratorStatistics(mut self, val: VkBool32) -> Self {
    self.dataGraphNeuralAcceleratorStatistics = val;
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
/// [VkDataGraphPipelineNeuralStatisticsCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineNeuralStatisticsCreateInfoARM.html)
///
/// **Extends:** VkDataGraphPipelineCreateInfoARM.
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineNeuralStatisticsCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_NEURAL_STATISTICS_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub allowNeuralStatistics: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
unsafe impl<'a> Send for VkDataGraphPipelineNeuralStatisticsCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
unsafe impl<'a> Sync for VkDataGraphPipelineNeuralStatisticsCreateInfoARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_data_graph_neural_accelerator_statistics",
  feature = "VK_ARM_data_graph"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>
  for VkDataGraphPipelineNeuralStatisticsCreateInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
impl<'a> VkDataGraphPipelineNeuralStatisticsCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_NEURAL_STATISTICS_CREATE_INFO_ARM,
    pNext: core::ptr::null(),
    allowNeuralStatistics: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_allowNeuralStatistics(mut self, val: VkBool32) -> Self {
    self.allowNeuralStatistics = val;
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
/// [VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM.html)
///
/// **Extends:** VkDataGraphPipelineSessionCreateInfoARM.
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SESSION_NEURAL_STATISTICS_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub mode: VkNeuralAcceleratorStatisticsModeARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
unsafe impl<'a> Send for VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
unsafe impl<'a> Sync for VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_data_graph_neural_accelerator_statistics",
  feature = "VK_ARM_data_graph"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineSessionCreateInfoARM<'root>>
  for VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
impl<'a> VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
        sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SESSION_NEURAL_STATISTICS_CREATE_INFO_ARM,
        pNext: core::ptr::null(),
        mode: VkNeuralAcceleratorStatisticsModeARM(0),
        _marker: core::marker::PhantomData,
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
  pub const fn with_mode(mut self, val: VkNeuralAcceleratorStatisticsModeARM) -> Self {
    self.mode = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineSessionCreateInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineSessionCreateInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphOpticalFlowGridSizeFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowGridSizeFlagsARM.html)
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
pub type VkDataGraphOpticalFlowGridSizeFlagsARM = VkDataGraphOpticalFlowGridSizeFlagBitsARM;
/// [VkDataGraphOpticalFlowImageUsageFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowImageUsageFlagsARM.html)
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
pub type VkDataGraphOpticalFlowImageUsageFlagsARM = VkDataGraphOpticalFlowImageUsageFlagBitsARM;
/// [VkDataGraphOpticalFlowCreateFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowCreateFlagsARM.html)
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
pub type VkDataGraphOpticalFlowCreateFlagsARM = VkDataGraphOpticalFlowCreateFlagBitsARM;
/// [VkDataGraphOpticalFlowExecuteFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowExecuteFlagsARM.html)
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
pub type VkDataGraphOpticalFlowExecuteFlagsARM = VkDataGraphOpticalFlowExecuteFlagBitsARM;
/// [VkDataGraphPipelineResourceInfoImageLayoutARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineResourceInfoImageLayoutARM.html)
///
/// **Extends:** VkDataGraphPipelineResourceInfoARM.
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineResourceInfoImageLayoutARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub layout: VkImageLayout,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Send for VkDataGraphPipelineResourceInfoImageLayoutARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Sync for VkDataGraphPipelineResourceInfoImageLayoutARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_data_graph_optical_flow",
  feature = "VK_ARM_data_graph"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineResourceInfoARM<'root>>
  for VkDataGraphPipelineResourceInfoImageLayoutARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
impl<'a> VkDataGraphPipelineResourceInfoImageLayoutARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM,
    pNext: core::ptr::null(),
    layout: VkImageLayout(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_layout(mut self, val: VkImageLayout) -> Self {
    self.layout = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineResourceInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineResourceInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineSingleNodeConnectionARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSingleNodeConnectionARM.html)
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineSingleNodeConnectionARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub set: u32,
  pub binding: u32,
  pub connection: VkDataGraphPipelineNodeConnectionTypeARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Send for VkDataGraphPipelineSingleNodeConnectionARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Sync for VkDataGraphPipelineSingleNodeConnectionARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
impl<'a> VkDataGraphPipelineSingleNodeConnectionARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM,
    pNext: core::ptr::null_mut(),
    set: 0,
    binding: 0,
    connection: VkDataGraphPipelineNodeConnectionTypeARM(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_set(mut self, val: u32) -> Self {
    self.set = val;
    self
  }
  #[inline]
  pub const fn with_binding(mut self, val: u32) -> Self {
    self.binding = val;
    self
  }
  #[inline]
  pub const fn with_connection(mut self, val: VkDataGraphPipelineNodeConnectionTypeARM) -> Self {
    self.connection = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineSingleNodeConnectionARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineSingleNodeConnectionARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub dataGraphOpticalFlow: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Send for VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Sync for VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_data_graph_optical_flow",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_data_graph_optical_flow",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
impl<'a> VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM,
    pNext: core::ptr::null_mut(),
    dataGraphOpticalFlow: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_dataGraphOpticalFlow(mut self, val: VkBool32) -> Self {
    self.dataGraphOpticalFlow = val;
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
/// [VkQueueFamilyDataGraphOpticalFlowPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyDataGraphOpticalFlowPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueueFamilyDataGraphOpticalFlowPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub supportedOutputGridSizes: VkDataGraphOpticalFlowGridSizeFlagsARM,
  /// Limit Type: [Bitmask]
  pub supportedHintGridSizes: VkDataGraphOpticalFlowGridSizeFlagsARM,
  /// Limit Type: [Exact]
  pub hintSupported: VkBool32,
  /// Limit Type: [Exact]
  pub costSupported: VkBool32,
  /// Limit Type: [Noauto]
  pub minWidth: u32,
  /// Limit Type: [Noauto]
  pub minHeight: u32,
  /// Limit Type: [Noauto]
  pub maxWidth: u32,
  /// Limit Type: [Noauto]
  pub maxHeight: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Send for VkQueueFamilyDataGraphOpticalFlowPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Sync for VkQueueFamilyDataGraphOpticalFlowPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
impl<'a> VkQueueFamilyDataGraphOpticalFlowPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    supportedOutputGridSizes: VkDataGraphOpticalFlowGridSizeFlagBitsARM(0),
    supportedHintGridSizes: VkDataGraphOpticalFlowGridSizeFlagBitsARM(0),
    hintSupported: 0,
    costSupported: 0,
    minWidth: 0,
    minHeight: 0,
    maxWidth: 0,
    maxHeight: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_supportedOutputGridSizes(
    mut self,
    val: VkDataGraphOpticalFlowGridSizeFlagsARM,
  ) -> Self {
    self.supportedOutputGridSizes = val;
    self
  }
  #[inline]
  pub const fn with_supportedHintGridSizes(
    mut self,
    val: VkDataGraphOpticalFlowGridSizeFlagsARM,
  ) -> Self {
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
  #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkQueueFamilyDataGraphOpticalFlowPropertiesARM<
    'root,
    T: VkPNextExtends<VkQueueFamilyDataGraphOpticalFlowPropertiesARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphOpticalFlowImageFormatInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowImageFormatInfoARM.html)
///
/// **Extends:** VkPhysicalDeviceImageFormatInfo2, VkImageCreateInfo.
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphOpticalFlowImageFormatInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub usage: VkDataGraphOpticalFlowImageUsageFlagsARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Send for VkDataGraphOpticalFlowImageFormatInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Sync for VkDataGraphOpticalFlowImageFormatInfoARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_data_graph_optical_flow",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceImageFormatInfo2<'root>>
  for VkDataGraphOpticalFlowImageFormatInfoARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_data_graph_optical_flow",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkDataGraphOpticalFlowImageFormatInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
impl<'a> VkDataGraphOpticalFlowImageFormatInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM,
    pNext: core::ptr::null(),
    usage: VkDataGraphOpticalFlowImageUsageFlagBitsARM(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_usage(mut self, val: VkDataGraphOpticalFlowImageUsageFlagsARM) -> Self {
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
/// [VkDataGraphOpticalFlowImageFormatPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphOpticalFlowImageFormatPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphOpticalFlowImageFormatPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub format: VkFormat,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Send for VkDataGraphOpticalFlowImageFormatPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Sync for VkDataGraphOpticalFlowImageFormatPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
impl<'a> VkDataGraphOpticalFlowImageFormatPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM,
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
  #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphOpticalFlowImageFormatPropertiesARM<
    'root,
    T: VkPNextExtends<VkDataGraphOpticalFlowImageFormatPropertiesARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineSingleNodeCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSingleNodeCreateInfoARM.html)
///
/// **Extends:** VkDataGraphPipelineCreateInfoARM.
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineSingleNodeCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub nodeType: VkDataGraphPipelineNodeTypeARM,
  pub connectionCount: u32,
  /// Length: connectionCount
  pub pConnections: *const VkDataGraphPipelineSingleNodeConnectionARM<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Send for VkDataGraphPipelineSingleNodeCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Sync for VkDataGraphPipelineSingleNodeCreateInfoARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_data_graph_optical_flow",
  feature = "VK_ARM_data_graph"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>
  for VkDataGraphPipelineSingleNodeCreateInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
impl<'a> VkDataGraphPipelineSingleNodeCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM,
    pNext: core::ptr::null_mut(),
    nodeType: VkDataGraphPipelineNodeTypeARM(0),
    connectionCount: 0,
    pConnections: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_nodeType(mut self, val: VkDataGraphPipelineNodeTypeARM) -> Self {
    self.nodeType = val;
    self
  }
  #[inline]
  pub const fn with_connectionCount(mut self, val: u32) -> Self {
    self.connectionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pConnections(
    mut self,
    val: &'a [VkDataGraphPipelineSingleNodeConnectionARM<'a>],
  ) -> Self {
    self.connectionCount = val.len() as u32;
    self.pConnections = val.as_ptr();
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
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineOpticalFlowCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineOpticalFlowCreateInfoARM.html)
///
/// **Extends:** VkDataGraphPipelineCreateInfoARM.
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineOpticalFlowCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub width: u32,
  pub height: u32,
  pub imageFormat: VkFormat,
  pub flowVectorFormat: VkFormat,
  /// Optional: true
  pub costFormat: VkFormat,
  /// No Auto-Validity
  pub outputGridSize: VkDataGraphOpticalFlowGridSizeFlagsARM,
  /// No Auto-Validity
  pub hintGridSize: VkDataGraphOpticalFlowGridSizeFlagsARM,
  /// Optional: true
  pub performanceLevel: VkDataGraphOpticalFlowPerformanceLevelARM,
  /// Optional: true
  pub flags: VkDataGraphOpticalFlowCreateFlagsARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Send for VkDataGraphPipelineOpticalFlowCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Sync for VkDataGraphPipelineOpticalFlowCreateInfoARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_data_graph_optical_flow",
  feature = "VK_ARM_data_graph"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>
  for VkDataGraphPipelineOpticalFlowCreateInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
impl<'a> VkDataGraphPipelineOpticalFlowCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM,
    pNext: core::ptr::null_mut(),
    width: 0,
    height: 0,
    imageFormat: VkFormat(0),
    flowVectorFormat: VkFormat(0),
    costFormat: VkFormat(0),
    outputGridSize: VkDataGraphOpticalFlowGridSizeFlagBitsARM(0),
    hintGridSize: VkDataGraphOpticalFlowGridSizeFlagBitsARM(0),
    performanceLevel: VkDataGraphOpticalFlowPerformanceLevelARM(0),
    flags: VkDataGraphOpticalFlowCreateFlagBitsARM(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_outputGridSize(mut self, val: VkDataGraphOpticalFlowGridSizeFlagsARM) -> Self {
    self.outputGridSize = val;
    self
  }
  #[inline]
  pub const fn with_hintGridSize(mut self, val: VkDataGraphOpticalFlowGridSizeFlagsARM) -> Self {
    self.hintGridSize = val;
    self
  }
  #[inline]
  pub const fn with_performanceLevel(
    mut self,
    val: VkDataGraphOpticalFlowPerformanceLevelARM,
  ) -> Self {
    self.performanceLevel = val;
    self
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkDataGraphOpticalFlowCreateFlagsARM) -> Self {
    self.flags = val;
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
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDataGraphPipelineOpticalFlowDispatchInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineOpticalFlowDispatchInfoARM.html)
///
/// **Extends:** VkDataGraphPipelineDispatchInfoARM.
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDataGraphPipelineOpticalFlowDispatchInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Optional: true
  pub flags: VkDataGraphOpticalFlowExecuteFlagsARM,
  /// Optional: true
  pub meanFlowL1NormHint: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Send for VkDataGraphPipelineOpticalFlowDispatchInfoARM<'a> {}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
unsafe impl<'a> Sync for VkDataGraphPipelineOpticalFlowDispatchInfoARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_data_graph_optical_flow",
  feature = "VK_ARM_data_graph"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineDispatchInfoARM<'root>>
  for VkDataGraphPipelineOpticalFlowDispatchInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
impl<'a> VkDataGraphPipelineOpticalFlowDispatchInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM,
    pNext: core::ptr::null_mut(),
    flags: VkDataGraphOpticalFlowExecuteFlagBitsARM(0),
    meanFlowL1NormHint: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkDataGraphOpticalFlowExecuteFlagsARM) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_meanFlowL1NormHint(mut self, val: u32) -> Self {
    self.meanFlowL1NormHint = val;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineDispatchInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineDispatchInfoARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceFormatPackFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFormatPackFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_format_pack")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceFormatPackFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub formatPack: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_format_pack")]
unsafe impl<'a> Send for VkPhysicalDeviceFormatPackFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_format_pack")]
unsafe impl<'a> Sync for VkPhysicalDeviceFormatPackFeaturesARM<'a> {}
#[cfg(all(feature = "VK_ARM_format_pack", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceFormatPackFeaturesARM<'child>
{
}
#[cfg(all(feature = "VK_ARM_format_pack", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceFormatPackFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_format_pack")]
impl<'a> VkPhysicalDeviceFormatPackFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM,
    pNext: core::ptr::null_mut(),
    formatPack: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_formatPack(mut self, val: VkBool32) -> Self {
    self.formatPack = val;
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
/// [VkPerformanceCounterDescriptionFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagsARM.html)
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
pub type VkPerformanceCounterDescriptionFlagsARM = VkFlags;
/// [VkPhysicalDevicePerformanceCountersByRegionFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePerformanceCountersByRegionFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub performanceCountersByRegion: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
unsafe impl<'a> Send for VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
unsafe impl<'a> Sync for VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_performance_counters_by_region",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_performance_counters_by_region",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
impl<'a> VkPhysicalDevicePerformanceCountersByRegionFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM,
    pNext: core::ptr::null_mut(),
    performanceCountersByRegion: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_performanceCountersByRegion(mut self, val: VkBool32) -> Self {
    self.performanceCountersByRegion = val;
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
/// [VkPhysicalDevicePerformanceCountersByRegionPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePerformanceCountersByRegionPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePerformanceCountersByRegionPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxPerRegionPerformanceCounters: u32,
  /// Limit Type: [Exact]
  pub performanceCounterRegionSize: VkExtent2D,
  /// Limit Type: [Min]
  pub rowStrideAlignment: u32,
  /// Limit Type: [Exact]
  pub regionAlignment: u32,
  /// Limit Type: [Exact]
  pub identityTransformOrder: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
unsafe impl<'a> Send for VkPhysicalDevicePerformanceCountersByRegionPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
unsafe impl<'a> Sync for VkPhysicalDevicePerformanceCountersByRegionPropertiesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_performance_counters_by_region",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDevicePerformanceCountersByRegionPropertiesARM<'child>
{
}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
impl<'a> VkPhysicalDevicePerformanceCountersByRegionPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
        sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM,
        pNext: core::ptr::null_mut(),
        maxPerRegionPerformanceCounters: 0,
        performanceCounterRegionSize: VkExtent2D::DEFAULT,
        rowStrideAlignment: 0,
        regionAlignment: 0,
        identityTransformOrder: 0,
        _marker: core::marker::PhantomData,
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
  pub const fn with_maxPerRegionPerformanceCounters(mut self, val: u32) -> Self {
    self.maxPerRegionPerformanceCounters = val;
    self
  }
  #[inline]
  pub const fn with_performanceCounterRegionSize(mut self, val: VkExtent2D) -> Self {
    self.performanceCounterRegionSize = val;
    self
  }
  #[inline]
  pub const fn with_rowStrideAlignment(mut self, val: u32) -> Self {
    self.rowStrideAlignment = val;
    self
  }
  #[inline]
  pub const fn with_regionAlignment(mut self, val: u32) -> Self {
    self.regionAlignment = val;
    self
  }
  #[inline]
  pub const fn with_identityTransformOrder(mut self, val: VkBool32) -> Self {
    self.identityTransformOrder = val;
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
/// [VkPerformanceCounterARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterARM.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPerformanceCounterARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub counterID: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
unsafe impl<'a> Send for VkPerformanceCounterARM<'a> {}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
unsafe impl<'a> Sync for VkPerformanceCounterARM<'a> {}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
impl<'a> VkPerformanceCounterARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_ARM,
    pNext: core::ptr::null_mut(),
    counterID: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_counterID(mut self, val: u32) -> Self {
    self.counterID = val;
    self
  }
  #[cfg(feature = "VK_ARM_performance_counters_by_region")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPerformanceCounterARM<
    'root,
    T: VkPNextExtends<VkPerformanceCounterARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPerformanceCounterDescriptionARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionARM.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPerformanceCounterDescriptionARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Optional: true
  pub flags: VkPerformanceCounterDescriptionFlagsARM,
  /// Length: null-terminated
  pub name: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
unsafe impl<'a> Send for VkPerformanceCounterDescriptionARM<'a> {}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
unsafe impl<'a> Sync for VkPerformanceCounterDescriptionARM<'a> {}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
impl<'a> VkPerformanceCounterDescriptionARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_ARM,
    pNext: core::ptr::null_mut(),
    flags: 0,
    name: [0i8; VK_MAX_DESCRIPTION_SIZE as usize],
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkPerformanceCounterDescriptionFlagsARM) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_name(mut self, val: [c_char; VK_MAX_DESCRIPTION_SIZE as usize]) -> Self {
    self.name = val;
    self
  }
  #[cfg(feature = "VK_ARM_performance_counters_by_region")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPerformanceCounterDescriptionARM<
    'root,
    T: VkPNextExtends<VkPerformanceCounterDescriptionARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkRenderPassPerformanceCountersByRegionBeginInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassPerformanceCountersByRegionBeginInfoARM.html)
///
/// **Extends:** VkRenderPassBeginInfo, VkRenderingInfo.
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassPerformanceCountersByRegionBeginInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub counterAddressCount: u32,
  /// Length: counterAddressCount
  pub pCounterAddresses: *const VkDeviceAddress,
  pub serializeRegions: VkBool32,
  pub counterIndexCount: u32,
  /// Length: counterIndexCount
  pub pCounterIndices: *mut u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
unsafe impl<'a> Send for VkRenderPassPerformanceCountersByRegionBeginInfoARM<'a> {}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
unsafe impl<'a> Sync for VkRenderPassPerformanceCountersByRegionBeginInfoARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_performance_counters_by_region",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassBeginInfo<'root>>
  for VkRenderPassPerformanceCountersByRegionBeginInfoARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_performance_counters_by_region",
  feature = "VK_GRAPHICS_VERSION_1_3"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderingInfo<'root>>
  for VkRenderPassPerformanceCountersByRegionBeginInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
impl<'a> VkRenderPassPerformanceCountersByRegionBeginInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM,
    pNext: core::ptr::null_mut(),
    counterAddressCount: 0,
    pCounterAddresses: core::ptr::null(),
    serializeRegions: 0,
    counterIndexCount: 0,
    pCounterIndices: core::ptr::null_mut(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_counterAddressCount(mut self, val: u32) -> Self {
    self.counterAddressCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCounterAddresses(mut self, val: &'a [VkDeviceAddress]) -> Self {
    self.counterAddressCount = val.len() as u32;
    self.pCounterAddresses = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_serializeRegions(mut self, val: VkBool32) -> Self {
    self.serializeRegions = val;
    self
  }
  #[inline]
  pub const fn with_counterIndexCount(mut self, val: u32) -> Self {
    self.counterIndexCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCounterIndices(mut self, val: &'a mut [u32]) -> Self {
    self.counterIndexCount = val.len() as u32;
    self.pCounterIndices = val.as_mut_ptr();
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
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderingInfo<
    'root,
    T: VkPNextExtends<VkRenderingInfo<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDevicePipelineOpacityMicromapFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineOpacityMicromapFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_pipeline_opacity_micromap")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub pipelineOpacityMicromap: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_pipeline_opacity_micromap")]
unsafe impl<'a> Send for VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_pipeline_opacity_micromap")]
unsafe impl<'a> Sync for VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_pipeline_opacity_micromap",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_pipeline_opacity_micromap",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_pipeline_opacity_micromap")]
impl<'a> VkPhysicalDevicePipelineOpacityMicromapFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM,
    pNext: core::ptr::null_mut(),
    pipelineOpacityMicromap: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_pipelineOpacityMicromap(mut self, val: VkBool32) -> Self {
    self.pipelineOpacityMicromap = val;
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
/// [VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub rasterizationOrderColorAttachmentAccess: VkBool32,
  pub rasterizationOrderDepthAttachmentAccess: VkBool32,
  pub rasterizationOrderStencilAttachmentAccess: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
unsafe impl<'a> Send for VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
unsafe impl<'a> Sync for VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_rasterization_order_attachment_access",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_rasterization_order_attachment_access",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
impl<'a> VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
        sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM,
        pNext: core::ptr::null_mut(),
        rasterizationOrderColorAttachmentAccess: 0,
        rasterizationOrderDepthAttachmentAccess: 0,
        rasterizationOrderStencilAttachmentAccess: 0,
        _marker: core::marker::PhantomData,
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
  pub const fn with_rasterizationOrderColorAttachmentAccess(mut self, val: VkBool32) -> Self {
    self.rasterizationOrderColorAttachmentAccess = val;
    self
  }
  #[inline]
  pub const fn with_rasterizationOrderDepthAttachmentAccess(mut self, val: VkBool32) -> Self {
    self.rasterizationOrderDepthAttachmentAccess = val;
    self
  }
  #[inline]
  pub const fn with_rasterizationOrderStencilAttachmentAccess(mut self, val: VkBool32) -> Self {
    self.rasterizationOrderStencilAttachmentAccess = val;
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
/// [VkPhysicalDeviceRenderPassStripedFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRenderPassStripedFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_render_pass_striped")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRenderPassStripedFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub renderPassStriped: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_render_pass_striped")]
unsafe impl<'a> Send for VkPhysicalDeviceRenderPassStripedFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_render_pass_striped")]
unsafe impl<'a> Sync for VkPhysicalDeviceRenderPassStripedFeaturesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_render_pass_striped",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceRenderPassStripedFeaturesARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_render_pass_striped",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceRenderPassStripedFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_render_pass_striped")]
impl<'a> VkPhysicalDeviceRenderPassStripedFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM,
    pNext: core::ptr::null_mut(),
    renderPassStriped: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_renderPassStriped(mut self, val: VkBool32) -> Self {
    self.renderPassStriped = val;
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
/// [VkPhysicalDeviceRenderPassStripedPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRenderPassStripedPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_ARM_render_pass_striped")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRenderPassStripedPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Min, Mul]
  pub renderPassStripeGranularity: VkExtent2D,
  /// Limit Type: [Max]
  pub maxRenderPassStripes: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_render_pass_striped")]
unsafe impl<'a> Send for VkPhysicalDeviceRenderPassStripedPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_render_pass_striped")]
unsafe impl<'a> Sync for VkPhysicalDeviceRenderPassStripedPropertiesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_render_pass_striped",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceRenderPassStripedPropertiesARM<'child>
{
}
#[cfg(feature = "VK_ARM_render_pass_striped")]
impl<'a> VkPhysicalDeviceRenderPassStripedPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    renderPassStripeGranularity: VkExtent2D::DEFAULT,
    maxRenderPassStripes: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_renderPassStripeGranularity(mut self, val: VkExtent2D) -> Self {
    self.renderPassStripeGranularity = val;
    self
  }
  #[inline]
  pub const fn with_maxRenderPassStripes(mut self, val: u32) -> Self {
    self.maxRenderPassStripes = val;
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
/// [VkRenderPassStripeInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassStripeInfoARM.html)
#[cfg(feature = "VK_ARM_render_pass_striped")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassStripeInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_STRIPE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub stripeArea: VkRect2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_render_pass_striped")]
unsafe impl<'a> Send for VkRenderPassStripeInfoARM<'a> {}
#[cfg(feature = "VK_ARM_render_pass_striped")]
unsafe impl<'a> Sync for VkRenderPassStripeInfoARM<'a> {}
#[cfg(feature = "VK_ARM_render_pass_striped")]
impl<'a> VkRenderPassStripeInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_STRIPE_INFO_ARM,
    pNext: core::ptr::null(),
    stripeArea: VkRect2D::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_stripeArea(mut self, val: VkRect2D) -> Self {
    self.stripeArea = val;
    self
  }
  #[cfg(feature = "VK_ARM_render_pass_striped")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassStripeInfoARM<
    'root,
    T: VkPNextExtends<VkRenderPassStripeInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkRenderPassStripeBeginInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassStripeBeginInfoARM.html)
///
/// **Extends:** VkRenderingInfo, VkRenderPassBeginInfo.
#[cfg(feature = "VK_ARM_render_pass_striped")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassStripeBeginInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_STRIPE_BEGIN_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub stripeInfoCount: u32,
  /// Length: stripeInfoCount
  pub pStripeInfos: *const VkRenderPassStripeInfoARM<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_render_pass_striped")]
unsafe impl<'a> Send for VkRenderPassStripeBeginInfoARM<'a> {}
#[cfg(feature = "VK_ARM_render_pass_striped")]
unsafe impl<'a> Sync for VkRenderPassStripeBeginInfoARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_render_pass_striped",
  feature = "VK_GRAPHICS_VERSION_1_3"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderingInfo<'root>>
  for VkRenderPassStripeBeginInfoARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_render_pass_striped",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassBeginInfo<'root>>
  for VkRenderPassStripeBeginInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_render_pass_striped")]
impl<'a> VkRenderPassStripeBeginInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_STRIPE_BEGIN_INFO_ARM,
    pNext: core::ptr::null(),
    stripeInfoCount: 0,
    pStripeInfos: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_stripeInfoCount(mut self, val: u32) -> Self {
    self.stripeInfoCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pStripeInfos(mut self, val: &'a [VkRenderPassStripeInfoARM<'a>]) -> Self {
    self.stripeInfoCount = val.len() as u32;
    self.pStripeInfos = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderingInfo<
    'root,
    T: VkPNextExtends<VkRenderingInfo<'root>>,
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
/// [VkRenderPassStripeSubmitInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassStripeSubmitInfoARM.html)
///
/// **Extends:** VkCommandBufferSubmitInfo.
#[cfg(feature = "VK_ARM_render_pass_striped")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassStripeSubmitInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_STRIPE_SUBMIT_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub stripeSemaphoreInfoCount: u32,
  /// Length: stripeSemaphoreInfoCount
  pub pStripeSemaphoreInfos: *const VkSemaphoreSubmitInfo<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_render_pass_striped")]
unsafe impl<'a> Send for VkRenderPassStripeSubmitInfoARM<'a> {}
#[cfg(feature = "VK_ARM_render_pass_striped")]
unsafe impl<'a> Sync for VkRenderPassStripeSubmitInfoARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_render_pass_striped",
  feature = "VK_BASE_VERSION_1_3"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferSubmitInfo<'root>>
  for VkRenderPassStripeSubmitInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_render_pass_striped")]
impl<'a> VkRenderPassStripeSubmitInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_STRIPE_SUBMIT_INFO_ARM,
    pNext: core::ptr::null(),
    stripeSemaphoreInfoCount: 0,
    pStripeSemaphoreInfos: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_stripeSemaphoreInfoCount(mut self, val: u32) -> Self {
    self.stripeSemaphoreInfoCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pStripeSemaphoreInfos(mut self, val: &'a [VkSemaphoreSubmitInfo<'a>]) -> Self {
    self.stripeSemaphoreInfoCount = val.len() as u32;
    self.pStripeSemaphoreInfos = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCommandBufferSubmitInfo<
    'root,
    T: VkPNextExtends<VkCommandBufferSubmitInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceSchedulingControlsFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsFlagsARM.html)
#[cfg(feature = "VK_ARM_scheduling_controls")]
pub type VkPhysicalDeviceSchedulingControlsFlagsARM = VkPhysicalDeviceSchedulingControlsFlagBitsARM;
/// [VkDeviceQueueShaderCoreControlCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueShaderCoreControlCreateInfoARM.html)
///
/// **Extends:** VkDeviceQueueCreateInfo, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_scheduling_controls")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceQueueShaderCoreControlCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderCoreCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_scheduling_controls")]
unsafe impl<'a> Send for VkDeviceQueueShaderCoreControlCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_scheduling_controls")]
unsafe impl<'a> Sync for VkDeviceQueueShaderCoreControlCreateInfoARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_scheduling_controls",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceQueueCreateInfo<'root>>
  for VkDeviceQueueShaderCoreControlCreateInfoARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_scheduling_controls",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkDeviceQueueShaderCoreControlCreateInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_scheduling_controls")]
impl<'a> VkDeviceQueueShaderCoreControlCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM,
    pNext: core::ptr::null_mut(),
    shaderCoreCount: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderCoreCount(mut self, val: u32) -> Self {
    self.shaderCoreCount = val;
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
/// [VkPhysicalDeviceSchedulingControlsFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_scheduling_controls")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSchedulingControlsFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub schedulingControls: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_scheduling_controls")]
unsafe impl<'a> Send for VkPhysicalDeviceSchedulingControlsFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_scheduling_controls")]
unsafe impl<'a> Sync for VkPhysicalDeviceSchedulingControlsFeaturesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_scheduling_controls",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceSchedulingControlsFeaturesARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_scheduling_controls",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceSchedulingControlsFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_scheduling_controls")]
impl<'a> VkPhysicalDeviceSchedulingControlsFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM,
    pNext: core::ptr::null_mut(),
    schedulingControls: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_schedulingControls(mut self, val: VkBool32) -> Self {
    self.schedulingControls = val;
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
/// [VkPhysicalDeviceSchedulingControlsPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_ARM_scheduling_controls")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSchedulingControlsPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub schedulingControlsFlags: VkPhysicalDeviceSchedulingControlsFlagsARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_scheduling_controls")]
unsafe impl<'a> Send for VkPhysicalDeviceSchedulingControlsPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_scheduling_controls")]
unsafe impl<'a> Sync for VkPhysicalDeviceSchedulingControlsPropertiesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_scheduling_controls",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceSchedulingControlsPropertiesARM<'child>
{
}
#[cfg(feature = "VK_ARM_scheduling_controls")]
impl<'a> VkPhysicalDeviceSchedulingControlsPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    schedulingControlsFlags: VkPhysicalDeviceSchedulingControlsFlagBitsARM(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_schedulingControlsFlags(
    mut self,
    val: VkPhysicalDeviceSchedulingControlsFlagsARM,
  ) -> Self {
    self.schedulingControlsFlags = val;
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
/// [VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_ARM_scheduling_controls")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub schedulingControlsMaxWarpsCount: u32,
  /// Limit Type: [Max]
  pub schedulingControlsMaxQueuedBatchesCount: u32,
  /// Limit Type: [Max]
  pub schedulingControlsMaxWorkGroupBatchSize: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_scheduling_controls")]
unsafe impl<'a> Send for VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_scheduling_controls")]
unsafe impl<'a> Sync for VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_scheduling_controls",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'child>
{
}
#[cfg(feature = "VK_ARM_scheduling_controls")]
impl<'a> VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
        sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM,
        pNext: core::ptr::null_mut(),
        schedulingControlsMaxWarpsCount: 0,
        schedulingControlsMaxQueuedBatchesCount: 0,
        schedulingControlsMaxWorkGroupBatchSize: 0,
        _marker: core::marker::PhantomData,
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
  pub const fn with_schedulingControlsMaxWarpsCount(mut self, val: u32) -> Self {
    self.schedulingControlsMaxWarpsCount = val;
    self
  }
  #[inline]
  pub const fn with_schedulingControlsMaxQueuedBatchesCount(mut self, val: u32) -> Self {
    self.schedulingControlsMaxQueuedBatchesCount = val;
    self
  }
  #[inline]
  pub const fn with_schedulingControlsMaxWorkGroupBatchSize(mut self, val: u32) -> Self {
    self.schedulingControlsMaxWorkGroupBatchSize = val;
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
/// [VkDispatchParametersARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDispatchParametersARM.html)
#[cfg(feature = "VK_ARM_scheduling_controls")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDispatchParametersARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DISPATCH_PARAMETERS_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Optional: true
  pub workGroupBatchSize: u32,
  /// Optional: true
  pub maxQueuedWorkGroupBatches: u32,
  /// Optional: true
  pub maxWarpsPerShaderCore: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_scheduling_controls")]
unsafe impl<'a> Send for VkDispatchParametersARM<'a> {}
#[cfg(feature = "VK_ARM_scheduling_controls")]
unsafe impl<'a> Sync for VkDispatchParametersARM<'a> {}
#[cfg(feature = "VK_ARM_scheduling_controls")]
impl<'a> VkDispatchParametersARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DISPATCH_PARAMETERS_ARM,
    pNext: core::ptr::null_mut(),
    workGroupBatchSize: 0,
    maxQueuedWorkGroupBatches: 0,
    maxWarpsPerShaderCore: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_workGroupBatchSize(mut self, val: u32) -> Self {
    self.workGroupBatchSize = val;
    self
  }
  #[inline]
  pub const fn with_maxQueuedWorkGroupBatches(mut self, val: u32) -> Self {
    self.maxQueuedWorkGroupBatches = val;
    self
  }
  #[inline]
  pub const fn with_maxWarpsPerShaderCore(mut self, val: u32) -> Self {
    self.maxWarpsPerShaderCore = val;
    self
  }
  #[cfg(feature = "VK_ARM_scheduling_controls")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDispatchParametersARM<
    'root,
    T: VkPNextExtends<VkDispatchParametersARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_ARM_shader_core_builtins")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub shaderCoreMask: u64,
  /// Limit Type: [Max]
  pub shaderCoreCount: u32,
  /// Limit Type: [Max]
  pub shaderWarpsPerCore: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_shader_core_builtins")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_shader_core_builtins")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_shader_core_builtins",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM<'child>
{
}
#[cfg(feature = "VK_ARM_shader_core_builtins")]
impl<'a> VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    shaderCoreMask: 0,
    shaderCoreCount: 0,
    shaderWarpsPerCore: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderCoreMask(mut self, val: u64) -> Self {
    self.shaderCoreMask = val;
    self
  }
  #[inline]
  pub const fn with_shaderCoreCount(mut self, val: u32) -> Self {
    self.shaderCoreCount = val;
    self
  }
  #[inline]
  pub const fn with_shaderWarpsPerCore(mut self, val: u32) -> Self {
    self.shaderWarpsPerCore = val;
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
/// [VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_shader_core_builtins")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderCoreBuiltins: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_shader_core_builtins")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_shader_core_builtins")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_shader_core_builtins",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_shader_core_builtins",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_shader_core_builtins")]
impl<'a> VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM,
    pNext: core::ptr::null_mut(),
    shaderCoreBuiltins: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderCoreBuiltins(mut self, val: VkBool32) -> Self {
    self.shaderCoreBuiltins = val;
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
/// [VkPhysicalDeviceShaderCorePropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderCorePropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_ARM_shader_core_properties")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderCorePropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Exact]
  pub pixelRate: u32,
  /// Limit Type: [Exact]
  pub texelRate: u32,
  /// Limit Type: [Exact]
  pub fmaRate: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_shader_core_properties")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderCorePropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_shader_core_properties")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderCorePropertiesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_shader_core_properties",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceShaderCorePropertiesARM<'child>
{
}
#[cfg(feature = "VK_ARM_shader_core_properties")]
impl<'a> VkPhysicalDeviceShaderCorePropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    pixelRate: 0,
    texelRate: 0,
    fmaRate: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_pixelRate(mut self, val: u32) -> Self {
    self.pixelRate = val;
    self
  }
  #[inline]
  pub const fn with_texelRate(mut self, val: u32) -> Self {
    self.texelRate = val;
    self
  }
  #[inline]
  pub const fn with_fmaRate(mut self, val: u32) -> Self {
    self.fmaRate = val;
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
/// [VkShaderInstrumentationValuesFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationValuesFlagsARM.html)
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub type VkShaderInstrumentationValuesFlagsARM = VkFlags;
/// [VkShaderInstrumentationARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationARM.html)
#[cfg(feature = "VK_ARM_shader_instrumentation")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkShaderInstrumentationARM(pub *mut c_void);
#[cfg(feature = "VK_ARM_shader_instrumentation")]
impl VkShaderInstrumentationARM {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
impl Default for VkShaderInstrumentationARM {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl Send for VkShaderInstrumentationARM {}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl Sync for VkShaderInstrumentationARM {}
/// [VkPhysicalDeviceShaderInstrumentationFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderInstrumentationFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_shader_instrumentation")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderInstrumentationFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderInstrumentation: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderInstrumentationFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderInstrumentationFeaturesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_shader_instrumentation",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderInstrumentationFeaturesARM<'child>
{
}
#[cfg(all(
  feature = "VK_ARM_shader_instrumentation",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderInstrumentationFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
impl<'a> VkPhysicalDeviceShaderInstrumentationFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM,
    pNext: core::ptr::null_mut(),
    shaderInstrumentation: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderInstrumentation(mut self, val: VkBool32) -> Self {
    self.shaderInstrumentation = val;
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
/// [VkPhysicalDeviceShaderInstrumentationPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderInstrumentationPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_ARM_shader_instrumentation")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderInstrumentationPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Min]
  pub numMetrics: u32,
  /// Limit Type: [Max]
  pub perBasicBlockGranularity: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderInstrumentationPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderInstrumentationPropertiesARM<'a> {}
#[cfg(all(
  feature = "VK_ARM_shader_instrumentation",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceShaderInstrumentationPropertiesARM<'child>
{
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
impl<'a> VkPhysicalDeviceShaderInstrumentationPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    numMetrics: 0,
    perBasicBlockGranularity: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_numMetrics(mut self, val: u32) -> Self {
    self.numMetrics = val;
    self
  }
  #[inline]
  pub const fn with_perBasicBlockGranularity(mut self, val: VkBool32) -> Self {
    self.perBasicBlockGranularity = val;
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
/// [VkShaderInstrumentationCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationCreateInfoARM.html)
#[cfg(feature = "VK_ARM_shader_instrumentation")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkShaderInstrumentationCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_SHADER_INSTRUMENTATION_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl<'a> Send for VkShaderInstrumentationCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl<'a> Sync for VkShaderInstrumentationCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
impl<'a> VkShaderInstrumentationCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_SHADER_INSTRUMENTATION_CREATE_INFO_ARM,
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkShaderInstrumentationCreateInfoARM<
    'root,
    T: VkPNextExtends<VkShaderInstrumentationCreateInfoARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkShaderInstrumentationMetricDescriptionARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationMetricDescriptionARM.html)
#[cfg(feature = "VK_ARM_shader_instrumentation")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkShaderInstrumentationMetricDescriptionARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Length: null-terminated
  pub name: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
  /// Length: null-terminated
  pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl<'a> Send for VkShaderInstrumentationMetricDescriptionARM<'a> {}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl<'a> Sync for VkShaderInstrumentationMetricDescriptionARM<'a> {}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
impl<'a> VkShaderInstrumentationMetricDescriptionARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM,
    pNext: core::ptr::null_mut(),
    name: [0i8; VK_MAX_DESCRIPTION_SIZE as usize],
    description: [0i8; VK_MAX_DESCRIPTION_SIZE as usize],
    _marker: core::marker::PhantomData,
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
  pub const fn with_name(mut self, val: [c_char; VK_MAX_DESCRIPTION_SIZE as usize]) -> Self {
    self.name = val;
    self
  }
  #[inline]
  pub const fn with_description(mut self, val: [c_char; VK_MAX_DESCRIPTION_SIZE as usize]) -> Self {
    self.description = val;
    self
  }
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkShaderInstrumentationMetricDescriptionARM<
    'root,
    T: VkPNextExtends<VkShaderInstrumentationMetricDescriptionARM<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkShaderInstrumentationMetricDataHeaderARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationMetricDataHeaderARM.html)
#[cfg(feature = "VK_ARM_shader_instrumentation")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkShaderInstrumentationMetricDataHeaderARM {
  pub resultIndex: u32,
  pub resultSubIndex: u32,
  pub stages: VkShaderStageFlags,
  pub basicBlockIndex: u32,
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl Send for VkShaderInstrumentationMetricDataHeaderARM {}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl Sync for VkShaderInstrumentationMetricDataHeaderARM {}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
impl VkShaderInstrumentationMetricDataHeaderARM {
  pub const DEFAULT: Self = Self {
    resultIndex: 0,
    resultSubIndex: 0,
    stages: VkShaderStageFlagBits(0),
    basicBlockIndex: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_resultIndex(mut self, val: u32) -> Self {
    self.resultIndex = val;
    self
  }
  #[inline]
  pub const fn with_resultSubIndex(mut self, val: u32) -> Self {
    self.resultSubIndex = val;
    self
  }
  #[inline]
  pub const fn with_stages(mut self, val: VkShaderStageFlags) -> Self {
    self.stages = val;
    self
  }
  #[inline]
  pub const fn with_basicBlockIndex(mut self, val: u32) -> Self {
    self.basicBlockIndex = val;
    self
  }
}
/// [VkTensorCreateFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCreateFlagsARM.html)
#[cfg(feature = "VK_ARM_tensors")]
pub type VkTensorCreateFlagsARM = VkTensorCreateFlagBitsARM;
/// [VkTensorUsageFlagsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorUsageFlagsARM.html)
#[cfg(feature = "VK_ARM_tensors")]
pub type VkTensorUsageFlagsARM = VkTensorUsageFlagBitsARM;
/// [VkTensorViewARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewARM.html)
#[cfg(feature = "VK_ARM_tensors")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkTensorViewARM(pub *mut c_void);
#[cfg(feature = "VK_ARM_tensors")]
impl VkTensorViewARM {
  pub const NULL: Self = Self(core::ptr::null_mut());
  pub const DEFAULT: Self = Self::NULL;
}
#[cfg(feature = "VK_ARM_tensors")]
impl Default for VkTensorViewARM {
  fn default() -> Self {
    Self::NULL
  }
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl Send for VkTensorViewARM {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl Sync for VkTensorViewARM {}
/// [VkTensorDescriptionARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorDescriptionARM.html)
///
/// **Extends:** VkDataGraphPipelineResourceInfoARM, VkDataGraphPipelineConstantARM.
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTensorDescriptionARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TENSOR_DESCRIPTION_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub tiling: VkTensorTilingARM,
  pub format: VkFormat,
  pub dimensionCount: u32,
  /// Length: dimensionCount
  pub pDimensions: *const i64,
  /// Optional: true,  Length: dimensionCount
  pub pStrides: *const i64,
  pub usage: VkTensorUsageFlagsARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkTensorDescriptionARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkTensorDescriptionARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_ARM_data_graph"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineResourceInfoARM<'root>>
  for VkTensorDescriptionARM<'child>
{
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_ARM_data_graph"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineConstantARM<'root>>
  for VkTensorDescriptionARM<'child>
{
}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkTensorDescriptionARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_TENSOR_DESCRIPTION_ARM,
    pNext: core::ptr::null(),
    tiling: VkTensorTilingARM(0),
    format: VkFormat(0),
    dimensionCount: 0,
    pDimensions: core::ptr::null(),
    pStrides: core::ptr::null(),
    usage: VkTensorUsageFlagBitsARM(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_tiling(mut self, val: VkTensorTilingARM) -> Self {
    self.tiling = val;
    self
  }
  #[inline]
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[inline]
  pub const fn with_dimensionCount(mut self, val: u32) -> Self {
    self.dimensionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDimensions(mut self, val: &'a [i64]) -> Self {
    self.dimensionCount = val.len() as u32;
    self.pDimensions = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pStrides(mut self, val: &'a [i64]) -> Self {
    self.dimensionCount = val.len() as u32;
    self.pStrides = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_usage(mut self, val: VkTensorUsageFlagsARM) -> Self {
    self.usage = val;
    self
  }
  /// # Safety
  /// The caller must ensure every provided array constrained by `dimensionCount` has the same length. Optional pointer arguments may be null, but non-null pointers must be valid for that same length and outlive any use of this struct instance.
  #[inline]
  pub const fn with_dimensionCount_slices(
    mut self,
    pDimensions: &'a [i64],
    pStrides: *const i64,
  ) -> Self {
    let len = pDimensions.len();
    self.dimensionCount = len as u32;
    self.pDimensions = pDimensions.as_ptr();
    self.pStrides = pStrides;
    self
  }
  #[cfg(feature = "VK_ARM_data_graph")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDataGraphPipelineResourceInfoARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineResourceInfoARM<'root>>,
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
  pub const fn with_pNext_chain_VkDataGraphPipelineConstantARM<
    'root,
    T: VkPNextExtends<VkDataGraphPipelineConstantARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkTensorCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCreateInfoARM.html)
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTensorCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TENSOR_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkTensorCreateFlagsARM,
  pub pDescription: *const VkTensorDescriptionARM<'a>,
  pub sharingMode: VkSharingMode,
  /// Optional: true
  pub queueFamilyIndexCount: u32,
  /// Length: queueFamilyIndexCount,  No Auto-Validity
  pub pQueueFamilyIndices: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkTensorCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkTensorCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkTensorCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_TENSOR_CREATE_INFO_ARM,
    pNext: core::ptr::null(),
    flags: VkTensorCreateFlagBitsARM(0),
    pDescription: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkTensorCreateFlagsARM) -> Self {
    self.flags = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDescription(mut self, val: &'a VkTensorDescriptionARM<'a>) -> Self {
    self.pDescription = val as *const VkTensorDescriptionARM<'a>;
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
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalMemoryTensorCreateInfoARM<'child>(
    mut self,
    val: &'a VkExternalMemoryTensorCreateInfoARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalMemoryTensorCreateInfoARM<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkTensorCreateInfoARM<
    'root,
    T: VkPNextExtends<VkTensorCreateInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkTensorMemoryRequirementsInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorMemoryRequirementsInfoARM.html)
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTensorMemoryRequirementsInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TENSOR_MEMORY_REQUIREMENTS_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub tensor: VkTensorARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkTensorMemoryRequirementsInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkTensorMemoryRequirementsInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkTensorMemoryRequirementsInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_TENSOR_MEMORY_REQUIREMENTS_INFO_ARM,
    pNext: core::ptr::null(),
    tensor: VkTensorARM::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_tensor(mut self, val: VkTensorARM) -> Self {
    self.tensor = val;
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkTensorMemoryRequirementsInfoARM<
    'root,
    T: VkPNextExtends<VkTensorMemoryRequirementsInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBindTensorMemoryInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindTensorMemoryInfoARM.html)
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindTensorMemoryInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_BIND_TENSOR_MEMORY_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub tensor: VkTensorARM,
  pub memory: VkDeviceMemory,
  pub memoryOffset: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkBindTensorMemoryInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkBindTensorMemoryInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkBindTensorMemoryInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_BIND_TENSOR_MEMORY_INFO_ARM,
    pNext: core::ptr::null(),
    tensor: VkTensorARM::DEFAULT,
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
  pub const fn with_tensor(mut self, val: VkTensorARM) -> Self {
    self.tensor = val;
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
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBindTensorMemoryInfoARM<
    'root,
    T: VkPNextExtends<VkBindTensorMemoryInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkWriteDescriptorSetTensorARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSetTensorARM.html)
///
/// **Extends:** VkWriteDescriptorSet.
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkWriteDescriptorSetTensorARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_TENSOR_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub tensorViewCount: u32,
  /// Optional: pointer required, values optional if pointer not null,  Length: tensorViewCount
  pub pTensorViews: *const VkTensorViewARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkWriteDescriptorSetTensorARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkWriteDescriptorSetTensorARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkWriteDescriptorSet<'root>>
  for VkWriteDescriptorSetTensorARM<'child>
{
}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkWriteDescriptorSetTensorARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_TENSOR_ARM,
    pNext: core::ptr::null(),
    tensorViewCount: 0,
    pTensorViews: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_tensorViewCount(mut self, val: u32) -> Self {
    self.tensorViewCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pTensorViews(mut self, val: &'a [VkTensorViewARM]) -> Self {
    self.tensorViewCount = val.len() as u32;
    self.pTensorViews = val.as_ptr();
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
/// [VkTensorFormatPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorFormatPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkFormatProperties2.
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTensorFormatPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TENSOR_FORMAT_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub optimalTilingTensorFeatures: VkFormatFeatureFlags2,
  /// Limit Type: [Bitmask]
  pub linearTilingTensorFeatures: VkFormatFeatureFlags2,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkTensorFormatPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkTensorFormatPropertiesARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkFormatProperties2<'root>>
  for VkTensorFormatPropertiesARM<'child>
{
}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkTensorFormatPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_TENSOR_FORMAT_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    optimalTilingTensorFeatures: VkFormatFeatureFlagBits2(0),
    linearTilingTensorFeatures: VkFormatFeatureFlagBits2(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_optimalTilingTensorFeatures(mut self, val: VkFormatFeatureFlags2) -> Self {
    self.optimalTilingTensorFeatures = val;
    self
  }
  #[inline]
  pub const fn with_linearTilingTensorFeatures(mut self, val: VkFormatFeatureFlags2) -> Self {
    self.linearTilingTensorFeatures = val;
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
/// [VkPhysicalDeviceTensorPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTensorPropertiesARM.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceTensorPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxTensorDimensionCount: u32,
  /// Limit Type: [Max]
  pub maxTensorElements: u64,
  /// Limit Type: [Max]
  pub maxPerDimensionTensorElements: u64,
  /// Limit Type: [Max]
  pub maxTensorStride: i64,
  /// Limit Type: [Max]
  pub maxTensorSize: u64,
  /// Limit Type: [Max]
  pub maxTensorShaderAccessArrayLength: u32,
  /// Limit Type: [Max]
  pub maxTensorShaderAccessSize: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetStorageTensors: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorSetStorageTensors: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUpdateAfterBindStorageTensors: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorUpdateAfterBindStorageTensors: u32,
  /// Limit Type: [Max]
  pub shaderStorageTensorArrayNonUniformIndexingNative: VkBool32,
  /// Limit Type: [Bitmask]
  pub shaderTensorSupportedStages: VkShaderStageFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkPhysicalDeviceTensorPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkPhysicalDeviceTensorPropertiesARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceTensorPropertiesARM<'child>
{
}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkPhysicalDeviceTensorPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    maxTensorDimensionCount: 0,
    maxTensorElements: 0,
    maxPerDimensionTensorElements: 0,
    maxTensorStride: 0,
    maxTensorSize: 0,
    maxTensorShaderAccessArrayLength: 0,
    maxTensorShaderAccessSize: 0,
    maxDescriptorSetStorageTensors: 0,
    maxPerStageDescriptorSetStorageTensors: 0,
    maxDescriptorSetUpdateAfterBindStorageTensors: 0,
    maxPerStageDescriptorUpdateAfterBindStorageTensors: 0,
    shaderStorageTensorArrayNonUniformIndexingNative: 0,
    shaderTensorSupportedStages: VkShaderStageFlagBits(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_maxTensorDimensionCount(mut self, val: u32) -> Self {
    self.maxTensorDimensionCount = val;
    self
  }
  #[inline]
  pub const fn with_maxTensorElements(mut self, val: u64) -> Self {
    self.maxTensorElements = val;
    self
  }
  #[inline]
  pub const fn with_maxPerDimensionTensorElements(mut self, val: u64) -> Self {
    self.maxPerDimensionTensorElements = val;
    self
  }
  #[inline]
  pub const fn with_maxTensorStride(mut self, val: i64) -> Self {
    self.maxTensorStride = val;
    self
  }
  #[inline]
  pub const fn with_maxTensorSize(mut self, val: u64) -> Self {
    self.maxTensorSize = val;
    self
  }
  #[inline]
  pub const fn with_maxTensorShaderAccessArrayLength(mut self, val: u32) -> Self {
    self.maxTensorShaderAccessArrayLength = val;
    self
  }
  #[inline]
  pub const fn with_maxTensorShaderAccessSize(mut self, val: u32) -> Self {
    self.maxTensorShaderAccessSize = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetStorageTensors(mut self, val: u32) -> Self {
    self.maxDescriptorSetStorageTensors = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorSetStorageTensors(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorSetStorageTensors = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUpdateAfterBindStorageTensors(mut self, val: u32) -> Self {
    self.maxDescriptorSetUpdateAfterBindStorageTensors = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorUpdateAfterBindStorageTensors(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorUpdateAfterBindStorageTensors = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageTensorArrayNonUniformIndexingNative(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.shaderStorageTensorArrayNonUniformIndexingNative = val;
    self
  }
  #[inline]
  pub const fn with_shaderTensorSupportedStages(mut self, val: VkShaderStageFlags) -> Self {
    self.shaderTensorSupportedStages = val;
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
/// [VkTensorMemoryBarrierARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorMemoryBarrierARM.html)
///
/// **Extends:** VkDependencyInfo.
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTensorMemoryBarrierARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TENSOR_MEMORY_BARRIER_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub srcStageMask: VkPipelineStageFlags2,
  /// Optional: true
  pub srcAccessMask: VkAccessFlags2,
  /// Optional: true
  pub dstStageMask: VkPipelineStageFlags2,
  /// Optional: true
  pub dstAccessMask: VkAccessFlags2,
  pub srcQueueFamilyIndex: u32,
  pub dstQueueFamilyIndex: u32,
  pub tensor: VkTensorARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkTensorMemoryBarrierARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkTensorMemoryBarrierARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_BASE_VERSION_1_3"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDependencyInfo<'root>>
  for VkTensorMemoryBarrierARM<'child>
{
}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkTensorMemoryBarrierARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_TENSOR_MEMORY_BARRIER_ARM,
    pNext: core::ptr::null(),
    srcStageMask: VkPipelineStageFlagBits2(0),
    srcAccessMask: VkAccessFlagBits2(0),
    dstStageMask: VkPipelineStageFlagBits2(0),
    dstAccessMask: VkAccessFlagBits2(0),
    srcQueueFamilyIndex: 0,
    dstQueueFamilyIndex: 0,
    tensor: VkTensorARM::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_srcStageMask(mut self, val: VkPipelineStageFlags2) -> Self {
    self.srcStageMask = val;
    self
  }
  #[inline]
  pub const fn with_srcAccessMask(mut self, val: VkAccessFlags2) -> Self {
    self.srcAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_dstStageMask(mut self, val: VkPipelineStageFlags2) -> Self {
    self.dstStageMask = val;
    self
  }
  #[inline]
  pub const fn with_dstAccessMask(mut self, val: VkAccessFlags2) -> Self {
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
  pub const fn with_tensor(mut self, val: VkTensorARM) -> Self {
    self.tensor = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDependencyInfo<
    'root,
    T: VkPNextExtends<VkDependencyInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkTensorDependencyInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorDependencyInfoARM.html)
///
/// **Extends:** VkDependencyInfo.
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTensorDependencyInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TENSOR_DEPENDENCY_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub tensorMemoryBarrierCount: u32,
  /// Length: tensorMemoryBarrierCount
  pub pTensorMemoryBarriers: *const VkTensorMemoryBarrierARM<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkTensorDependencyInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkTensorDependencyInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_BASE_VERSION_1_3"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDependencyInfo<'root>>
  for VkTensorDependencyInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkTensorDependencyInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_TENSOR_DEPENDENCY_INFO_ARM,
    pNext: core::ptr::null(),
    tensorMemoryBarrierCount: 0,
    pTensorMemoryBarriers: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_tensorMemoryBarrierCount(mut self, val: u32) -> Self {
    self.tensorMemoryBarrierCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pTensorMemoryBarriers(
    mut self,
    val: &'a [VkTensorMemoryBarrierARM<'a>],
  ) -> Self {
    self.tensorMemoryBarrierCount = val.len() as u32;
    self.pTensorMemoryBarriers = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDependencyInfo<
    'root,
    T: VkPNextExtends<VkDependencyInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceTensorFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTensorFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceTensorFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TENSOR_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub tensorNonPacked: VkBool32,
  pub shaderTensorAccess: VkBool32,
  pub shaderStorageTensorArrayDynamicIndexing: VkBool32,
  pub shaderStorageTensorArrayNonUniformIndexing: VkBool32,
  pub descriptorBindingStorageTensorUpdateAfterBind: VkBool32,
  pub tensors: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkPhysicalDeviceTensorFeaturesARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkPhysicalDeviceTensorFeaturesARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceTensorFeaturesARM<'child>
{
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceTensorFeaturesARM<'child>
{
}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkPhysicalDeviceTensorFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TENSOR_FEATURES_ARM,
    pNext: core::ptr::null_mut(),
    tensorNonPacked: 0,
    shaderTensorAccess: 0,
    shaderStorageTensorArrayDynamicIndexing: 0,
    shaderStorageTensorArrayNonUniformIndexing: 0,
    descriptorBindingStorageTensorUpdateAfterBind: 0,
    tensors: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_tensorNonPacked(mut self, val: VkBool32) -> Self {
    self.tensorNonPacked = val;
    self
  }
  #[inline]
  pub const fn with_shaderTensorAccess(mut self, val: VkBool32) -> Self {
    self.shaderTensorAccess = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageTensorArrayDynamicIndexing(mut self, val: VkBool32) -> Self {
    self.shaderStorageTensorArrayDynamicIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageTensorArrayNonUniformIndexing(mut self, val: VkBool32) -> Self {
    self.shaderStorageTensorArrayNonUniformIndexing = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingStorageTensorUpdateAfterBind(mut self, val: VkBool32) -> Self {
    self.descriptorBindingStorageTensorUpdateAfterBind = val;
    self
  }
  #[inline]
  pub const fn with_tensors(mut self, val: VkBool32) -> Self {
    self.tensors = val;
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
/// [VkDeviceTensorMemoryRequirementsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceTensorMemoryRequirementsARM.html)
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceTensorMemoryRequirementsARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pCreateInfo: *const VkTensorCreateInfoARM<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkDeviceTensorMemoryRequirementsARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkDeviceTensorMemoryRequirementsARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkDeviceTensorMemoryRequirementsARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM,
    pNext: core::ptr::null(),
    pCreateInfo: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_pCreateInfo(mut self, val: &'a VkTensorCreateInfoARM<'a>) -> Self {
    self.pCreateInfo = val as *const VkTensorCreateInfoARM<'a>;
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDeviceTensorMemoryRequirementsARM<
    'root,
    T: VkPNextExtends<VkDeviceTensorMemoryRequirementsARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCopyTensorInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyTensorInfoARM.html)
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyTensorInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_COPY_TENSOR_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcTensor: VkTensorARM,
  pub dstTensor: VkTensorARM,
  pub regionCount: u32,
  /// Length: regionCount
  pub pRegions: *const VkTensorCopyARM<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkCopyTensorInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkCopyTensorInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkCopyTensorInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_COPY_TENSOR_INFO_ARM,
    pNext: core::ptr::null(),
    srcTensor: VkTensorARM::DEFAULT,
    dstTensor: VkTensorARM::DEFAULT,
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
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_srcTensor(mut self, val: VkTensorARM) -> Self {
    self.srcTensor = val;
    self
  }
  #[inline]
  pub const fn with_dstTensor(mut self, val: VkTensorARM) -> Self {
    self.dstTensor = val;
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
  pub const fn with_pRegions(mut self, val: &'a [VkTensorCopyARM<'a>]) -> Self {
    self.regionCount = val.len() as u32;
    self.pRegions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCopyTensorInfoARM<
    'root,
    T: VkPNextExtends<VkCopyTensorInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkTensorCopyARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCopyARM.html)
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTensorCopyARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TENSOR_COPY_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub dimensionCount: u32,
  /// Optional: true,  Length: dimensionCount
  pub pSrcOffset: *const u64,
  /// Optional: true,  Length: dimensionCount
  pub pDstOffset: *const u64,
  /// Optional: true,  Length: dimensionCount
  pub pExtent: *const u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkTensorCopyARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkTensorCopyARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkTensorCopyARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_TENSOR_COPY_ARM,
    pNext: core::ptr::null(),
    dimensionCount: 0,
    pSrcOffset: core::ptr::null(),
    pDstOffset: core::ptr::null(),
    pExtent: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_dimensionCount(mut self, val: u32) -> Self {
    self.dimensionCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSrcOffset(mut self, val: &'a [u64]) -> Self {
    self.dimensionCount = val.len() as u32;
    self.pSrcOffset = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDstOffset(mut self, val: &'a [u64]) -> Self {
    self.dimensionCount = val.len() as u32;
    self.pDstOffset = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pExtent(mut self, val: &'a [u64]) -> Self {
    self.dimensionCount = val.len() as u32;
    self.pExtent = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkTensorCopyARM<
    'root,
    T: VkPNextExtends<VkTensorCopyARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkMemoryDedicatedAllocateInfoTensorARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDedicatedAllocateInfoTensorARM.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryDedicatedAllocateInfoTensorARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub tensor: VkTensorARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkMemoryDedicatedAllocateInfoTensorARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkMemoryDedicatedAllocateInfoTensorARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkMemoryDedicatedAllocateInfoTensorARM<'child>
{
}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkMemoryDedicatedAllocateInfoTensorARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM,
    pNext: core::ptr::null(),
    tensor: VkTensorARM::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_tensor(mut self, val: VkTensorARM) -> Self {
    self.tensor = val;
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
/// [VkPhysicalDeviceDescriptorBufferTensorPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorBufferTensorPropertiesARM.html)
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
///
/// **Availability:** depends on `VK_EXT_descriptor_buffer`.
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDescriptorBufferTensorPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Noauto]
  pub tensorCaptureReplayDescriptorDataSize: usize,
  /// Limit Type: [Noauto]
  pub tensorViewCaptureReplayDescriptorDataSize: usize,
  /// Limit Type: [Max]
  pub tensorDescriptorSize: usize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
unsafe impl<'a> Send for VkPhysicalDeviceDescriptorBufferTensorPropertiesARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
unsafe impl<'a> Sync for VkPhysicalDeviceDescriptorBufferTensorPropertiesARM<'a> {}
#[cfg(all(
  all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"),
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceDescriptorBufferTensorPropertiesARM<'child>
{
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
impl<'a> VkPhysicalDeviceDescriptorBufferTensorPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM,
    pNext: core::ptr::null_mut(),
    tensorCaptureReplayDescriptorDataSize: 0,
    tensorViewCaptureReplayDescriptorDataSize: 0,
    tensorDescriptorSize: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_tensorCaptureReplayDescriptorDataSize(mut self, val: usize) -> Self {
    self.tensorCaptureReplayDescriptorDataSize = val;
    self
  }
  #[inline]
  pub const fn with_tensorViewCaptureReplayDescriptorDataSize(mut self, val: usize) -> Self {
    self.tensorViewCaptureReplayDescriptorDataSize = val;
    self
  }
  #[inline]
  pub const fn with_tensorDescriptorSize(mut self, val: usize) -> Self {
    self.tensorDescriptorSize = val;
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
/// [VkPhysicalDeviceDescriptorBufferTensorFeaturesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorBufferTensorFeaturesARM.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
///
/// **Availability:** depends on `VK_EXT_descriptor_buffer`.
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub descriptorBufferTensorDescriptors: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
unsafe impl<'a> Send for VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
unsafe impl<'a> Sync for VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'a> {}
#[cfg(all(
  all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"),
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'child>
{
}
#[cfg(all(
  all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"),
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'child>
{
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
impl<'a> VkPhysicalDeviceDescriptorBufferTensorFeaturesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM,
    pNext: core::ptr::null_mut(),
    descriptorBufferTensorDescriptors: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_descriptorBufferTensorDescriptors(mut self, val: VkBool32) -> Self {
    self.descriptorBufferTensorDescriptors = val;
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
/// [VkTensorCaptureDescriptorDataInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCaptureDescriptorDataInfoARM.html)
///
/// **Availability:** depends on `VK_EXT_descriptor_buffer`.
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTensorCaptureDescriptorDataInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub tensor: VkTensorARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
unsafe impl<'a> Send for VkTensorCaptureDescriptorDataInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
unsafe impl<'a> Sync for VkTensorCaptureDescriptorDataInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
impl<'a> VkTensorCaptureDescriptorDataInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM,
    pNext: core::ptr::null(),
    tensor: VkTensorARM::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_tensor(mut self, val: VkTensorARM) -> Self {
    self.tensor = val;
    self
  }
  #[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkTensorCaptureDescriptorDataInfoARM<
    'root,
    T: VkPNextExtends<VkTensorCaptureDescriptorDataInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkTensorViewCaptureDescriptorDataInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewCaptureDescriptorDataInfoARM.html)
///
/// **Availability:** depends on `VK_EXT_descriptor_buffer`.
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTensorViewCaptureDescriptorDataInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub tensorView: VkTensorViewARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
unsafe impl<'a> Send for VkTensorViewCaptureDescriptorDataInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
unsafe impl<'a> Sync for VkTensorViewCaptureDescriptorDataInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
impl<'a> VkTensorViewCaptureDescriptorDataInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM,
    pNext: core::ptr::null(),
    tensorView: VkTensorViewARM::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_tensorView(mut self, val: VkTensorViewARM) -> Self {
    self.tensorView = val;
    self
  }
  #[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkTensorViewCaptureDescriptorDataInfoARM<
    'root,
    T: VkPNextExtends<VkTensorViewCaptureDescriptorDataInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDescriptorGetTensorInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorGetTensorInfoARM.html)
///
/// **Extends:** VkDescriptorGetInfoEXT.
///
/// **Availability:** depends on `VK_EXT_descriptor_buffer`.
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorGetTensorInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_GET_TENSOR_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub tensorView: VkTensorViewARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
unsafe impl<'a> Send for VkDescriptorGetTensorInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
unsafe impl<'a> Sync for VkDescriptorGetTensorInfoARM<'a> {}
#[cfg(all(
  all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"),
  feature = "VK_EXT_descriptor_buffer"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDescriptorGetInfoEXT<'root>>
  for VkDescriptorGetTensorInfoARM<'child>
{
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
impl<'a> VkDescriptorGetTensorInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_GET_TENSOR_INFO_ARM,
    pNext: core::ptr::null(),
    tensorView: VkTensorViewARM::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_tensorView(mut self, val: VkTensorViewARM) -> Self {
    self.tensorView = val;
    self
  }
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDescriptorGetInfoEXT<
    'root,
    T: VkPNextExtends<VkDescriptorGetInfoEXT<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkFrameBoundaryTensorsARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkFrameBoundaryTensorsARM.html)
///
/// **Extends:** VkSubmitInfo, VkSubmitInfo2, VkPresentInfoKHR, VkBindSparseInfo.
///
/// **Availability:** depends on `VK_EXT_frame_boundary`.
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFrameBoundaryTensorsARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_FRAME_BOUNDARY_TENSORS_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub tensorCount: u32,
  /// Length: tensorCount
  pub pTensors: *const VkTensorARM,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"))]
unsafe impl<'a> Send for VkFrameBoundaryTensorsARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"))]
unsafe impl<'a> Sync for VkFrameBoundaryTensorsARM<'a> {}
#[cfg(all(
  all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"),
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubmitInfo<'root>>
  for VkFrameBoundaryTensorsARM<'child>
{
}
#[cfg(all(
  all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"),
  feature = "VK_BASE_VERSION_1_3"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubmitInfo2<'root>>
  for VkFrameBoundaryTensorsARM<'child>
{
}
#[cfg(all(
  all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"),
  feature = "VK_KHR_swapchain"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPresentInfoKHR<'root>>
  for VkFrameBoundaryTensorsARM<'child>
{
}
#[cfg(all(
  all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"),
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkBindSparseInfo<'root>>
  for VkFrameBoundaryTensorsARM<'child>
{
}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"))]
impl<'a> VkFrameBoundaryTensorsARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_FRAME_BOUNDARY_TENSORS_ARM,
    pNext: core::ptr::null(),
    tensorCount: 0,
    pTensors: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_tensorCount(mut self, val: u32) -> Self {
    self.tensorCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pTensors(mut self, val: &'a [VkTensorARM]) -> Self {
    self.tensorCount = val.len() as u32;
    self.pTensors = val.as_ptr();
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
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
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
/// [VkPhysicalDeviceExternalTensorInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalTensorInfoARM.html)
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceExternalTensorInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkTensorCreateFlagsARM,
  pub pDescription: *const VkTensorDescriptionARM<'a>,
  pub handleType: VkExternalMemoryHandleTypeFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkPhysicalDeviceExternalTensorInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkPhysicalDeviceExternalTensorInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkPhysicalDeviceExternalTensorInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM,
    pNext: core::ptr::null(),
    flags: VkTensorCreateFlagBitsARM(0),
    pDescription: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkTensorCreateFlagsARM) -> Self {
    self.flags = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDescription(mut self, val: &'a VkTensorDescriptionARM<'a>) -> Self {
    self.pDescription = val as *const VkTensorDescriptionARM<'a>;
    self
  }
  #[inline]
  pub const fn with_handleType(mut self, val: VkExternalMemoryHandleTypeFlagBits) -> Self {
    self.handleType = val;
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPhysicalDeviceExternalTensorInfoARM<
    'root,
    T: VkPNextExtends<VkPhysicalDeviceExternalTensorInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkExternalTensorPropertiesARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalTensorPropertiesARM.html)
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalTensorPropertiesARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_TENSOR_PROPERTIES_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub externalMemoryProperties: VkExternalMemoryProperties,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkExternalTensorPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkExternalTensorPropertiesARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkExternalTensorPropertiesARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_TENSOR_PROPERTIES_ARM,
    pNext: core::ptr::null(),
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
  pub const fn with_pNext(mut self, val: *const c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_externalMemoryProperties(mut self, val: VkExternalMemoryProperties) -> Self {
    self.externalMemoryProperties = val;
    self
  }
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkExternalTensorPropertiesARM<
    'root,
    T: VkPNextExtends<VkExternalTensorPropertiesARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkExternalMemoryTensorCreateInfoARM](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryTensorCreateInfoARM.html)
///
/// **Extends:** VkTensorCreateInfoARM.
#[cfg(feature = "VK_ARM_tensors")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkExternalMemoryTensorCreateInfoARM<'a> {
  /// Values: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub handleTypes: VkExternalMemoryHandleTypeFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Send for VkExternalMemoryTensorCreateInfoARM<'a> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'a> Sync for VkExternalMemoryTensorCreateInfoARM<'a> {}
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_ARM_tensors"))]
unsafe impl<'child, 'root> VkPNextExtends<VkTensorCreateInfoARM<'root>>
  for VkExternalMemoryTensorCreateInfoARM<'child>
{
}
#[cfg(feature = "VK_ARM_tensors")]
impl<'a> VkExternalMemoryTensorCreateInfoARM<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM,
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
  #[cfg(feature = "VK_ARM_tensors")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkTensorCreateInfoARM<
    'root,
    T: VkPNextExtends<VkTensorCreateInfoARM<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
