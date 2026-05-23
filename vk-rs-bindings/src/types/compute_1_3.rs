#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_3",
  feature = "VK_EXT_pipeline_creation_feedback"
))]
use crate::enums::VkPipelineCreationFeedbackFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkShaderStageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkComputePipelineCreateInfo;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineCreateInfoARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorPoolCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkExecutionGraphPipelineCreateInfoAMDX;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineShaderStageCreateInfo;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkRayTracingPipelineCreateInfoKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkRayTracingPipelineCreateInfoNV;
#[cfg(feature = "VK_EXT_shader_object")]
use crate::types::VkShaderCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkShaderStageFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkWriteDescriptorSet;
use core::ffi::c_void;
/// [VkPipelineCreationFeedbackFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
pub type VkPipelineCreationFeedbackFlags = VkPipelineCreationFeedbackFlagBits;
/// [VkPhysicalDeviceInlineUniformBlockFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceInlineUniformBlockFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceInlineUniformBlockFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub inlineUniformBlock: VkBool32,
  pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceInlineUniformBlockFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceInlineUniformBlockFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceInlineUniformBlockFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceInlineUniformBlockFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceInlineUniformBlockFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES,
    pNext: core::ptr::null_mut(),
    inlineUniformBlock: 0,
    descriptorBindingInlineUniformBlockUpdateAfterBind: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_inlineUniformBlock(mut self, val: VkBool32) -> Self {
    self.inlineUniformBlock = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingInlineUniformBlockUpdateAfterBind(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.descriptorBindingInlineUniformBlockUpdateAfterBind = val;
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
/// [VkPhysicalDeviceInlineUniformBlockProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceInlineUniformBlockProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceInlineUniformBlockProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxInlineUniformBlockSize: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorInlineUniformBlocks: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetInlineUniformBlocks: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceInlineUniformBlockProperties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceInlineUniformBlockProperties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceInlineUniformBlockProperties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceInlineUniformBlockProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES,
    pNext: core::ptr::null_mut(),
    maxInlineUniformBlockSize: 0,
    maxPerStageDescriptorInlineUniformBlocks: 0,
    maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: 0,
    maxDescriptorSetInlineUniformBlocks: 0,
    maxDescriptorSetUpdateAfterBindInlineUniformBlocks: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_maxInlineUniformBlockSize(mut self, val: u32) -> Self {
    self.maxInlineUniformBlockSize = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorInlineUniformBlocks(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorInlineUniformBlocks = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks(
    mut self,
    val: u32,
  ) -> Self {
    self.maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetInlineUniformBlocks(mut self, val: u32) -> Self {
    self.maxDescriptorSetInlineUniformBlocks = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUpdateAfterBindInlineUniformBlocks(mut self, val: u32) -> Self {
    self.maxDescriptorSetUpdateAfterBindInlineUniformBlocks = val;
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
/// [VkWriteDescriptorSetInlineUniformBlock](https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSetInlineUniformBlock.html)
///
/// **Extends:** VkWriteDescriptorSet.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkWriteDescriptorSetInlineUniformBlock<'a> {
  /// Values: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub dataSize: u32,
  /// Length: dataSize
  pub pData: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkWriteDescriptorSetInlineUniformBlock<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkWriteDescriptorSetInlineUniformBlock<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkWriteDescriptorSet<'root>>
  for VkWriteDescriptorSetInlineUniformBlock<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkWriteDescriptorSetInlineUniformBlock<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK,
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
  pub const fn with_dataSize(mut self, val: u32) -> Self {
    self.dataSize = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pData(mut self, val: &'a [u8]) -> Self {
    self.dataSize = val.len() as u32;
    self.pData = val.as_ptr().cast::<c_void>();
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
/// [VkDescriptorPoolInlineUniformBlockCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolInlineUniformBlockCreateInfo.html)
///
/// **Extends:** VkDescriptorPoolCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorPoolInlineUniformBlockCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub maxInlineUniformBlockBindings: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkDescriptorPoolInlineUniformBlockCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkDescriptorPoolInlineUniformBlockCreateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDescriptorPoolCreateInfo<'root>>
  for VkDescriptorPoolInlineUniformBlockCreateInfo<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkDescriptorPoolInlineUniformBlockCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO,
    pNext: core::ptr::null(),
    maxInlineUniformBlockBindings: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_maxInlineUniformBlockBindings(mut self, val: u32) -> Self {
    self.maxInlineUniformBlockBindings = val;
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
/// [VkPipelineCreationFeedback](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedback.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCreationFeedback {
  pub flags: VkPipelineCreationFeedbackFlags,
  pub duration: u64,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl Send for VkPipelineCreationFeedback {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl Sync for VkPipelineCreationFeedback {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl VkPipelineCreationFeedback {
  pub const DEFAULT: Self = Self {
    flags: VkPipelineCreationFeedbackFlagBits(0),
    duration: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_flags(mut self, val: VkPipelineCreationFeedbackFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_duration(mut self, val: u64) -> Self {
    self.duration = val;
    self
  }
}
/// [VkPipelineCreationFeedbackCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackCreateInfo.html)
///
/// **Extends:** VkGraphicsPipelineCreateInfo, VkComputePipelineCreateInfo, VkRayTracingPipelineCreateInfoNV, VkRayTracingPipelineCreateInfoKHR, VkExecutionGraphPipelineCreateInfoAMDX, VkDataGraphPipelineCreateInfoARM.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCreationFeedbackCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pPipelineCreationFeedback: *mut VkPipelineCreationFeedback,
  /// Optional: true
  pub pipelineStageCreationFeedbackCount: u32,
  /// Length: pipelineStageCreationFeedbackCount
  pub pPipelineStageCreationFeedbacks: *mut VkPipelineCreationFeedback,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPipelineCreationFeedbackCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPipelineCreationFeedbackCreateInfo<'a> {}
#[cfg(all(
  feature = "VK_COMPUTE_VERSION_1_3",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkPipelineCreationFeedbackCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkComputePipelineCreateInfo<'root>>
  for VkPipelineCreationFeedbackCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_NV_ray_tracing"))]
unsafe impl<'child, 'root> VkPNextExtends<VkRayTracingPipelineCreateInfoNV<'root>>
  for VkPipelineCreationFeedbackCreateInfo<'child>
{
}
#[cfg(all(
  feature = "VK_COMPUTE_VERSION_1_3",
  feature = "VK_KHR_ray_tracing_pipeline"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRayTracingPipelineCreateInfoKHR<'root>>
  for VkPipelineCreationFeedbackCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_AMDX_shader_enqueue"))]
unsafe impl<'child, 'root> VkPNextExtends<VkExecutionGraphPipelineCreateInfoAMDX<'root>>
  for VkPipelineCreationFeedbackCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_ARM_data_graph"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDataGraphPipelineCreateInfoARM<'root>>
  for VkPipelineCreationFeedbackCreateInfo<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPipelineCreationFeedbackCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO,
    pNext: core::ptr::null(),
    pPipelineCreationFeedback: core::ptr::null_mut(),
    pipelineStageCreationFeedbackCount: 0,
    pPipelineStageCreationFeedbacks: core::ptr::null_mut(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_pPipelineCreationFeedback(
    mut self,
    val: &'a mut VkPipelineCreationFeedback,
  ) -> Self {
    self.pPipelineCreationFeedback = val as *mut VkPipelineCreationFeedback;
    self
  }
  #[inline]
  pub const fn with_pipelineStageCreationFeedbackCount(mut self, val: u32) -> Self {
    self.pipelineStageCreationFeedbackCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPipelineStageCreationFeedbacks(
    mut self,
    val: &'a mut [VkPipelineCreationFeedback],
  ) -> Self {
    self.pipelineStageCreationFeedbackCount = val.len() as u32;
    self.pPipelineStageCreationFeedbacks = val.as_mut_ptr();
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
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_AMDX_shader_enqueue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkExecutionGraphPipelineCreateInfoAMDX<
    'root,
    T: VkPNextExtends<VkExecutionGraphPipelineCreateInfoAMDX<'root>>,
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
/// [VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderDemoteToHelperInvocation: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderDemoteToHelperInvocation: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderDemoteToHelperInvocation(mut self, val: VkBool32) -> Self {
    self.shaderDemoteToHelperInvocation = val;
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
/// [VkPhysicalDeviceTexelBufferAlignmentProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTexelBufferAlignmentProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceTexelBufferAlignmentProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Min, Pot]
  pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
  /// Limit Type: [Exact]
  pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
  /// Limit Type: [Min, Pot]
  pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
  /// Limit Type: [Exact]
  pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceTexelBufferAlignmentProperties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceTexelBufferAlignmentProperties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceTexelBufferAlignmentProperties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceTexelBufferAlignmentProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES,
    pNext: core::ptr::null_mut(),
    storageTexelBufferOffsetAlignmentBytes: 0,
    storageTexelBufferOffsetSingleTexelAlignment: 0,
    uniformTexelBufferOffsetAlignmentBytes: 0,
    uniformTexelBufferOffsetSingleTexelAlignment: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_storageTexelBufferOffsetAlignmentBytes(mut self, val: VkDeviceSize) -> Self {
    self.storageTexelBufferOffsetAlignmentBytes = val;
    self
  }
  #[inline]
  pub const fn with_storageTexelBufferOffsetSingleTexelAlignment(mut self, val: VkBool32) -> Self {
    self.storageTexelBufferOffsetSingleTexelAlignment = val;
    self
  }
  #[inline]
  pub const fn with_uniformTexelBufferOffsetAlignmentBytes(mut self, val: VkDeviceSize) -> Self {
    self.uniformTexelBufferOffsetAlignmentBytes = val;
    self
  }
  #[inline]
  pub const fn with_uniformTexelBufferOffsetSingleTexelAlignment(mut self, val: VkBool32) -> Self {
    self.uniformTexelBufferOffsetSingleTexelAlignment = val;
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
/// [VkPhysicalDeviceSubgroupSizeControlFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubgroupSizeControlFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSubgroupSizeControlFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub subgroupSizeControl: VkBool32,
  pub computeFullSubgroups: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceSubgroupSizeControlFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceSubgroupSizeControlFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceSubgroupSizeControlFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceSubgroupSizeControlFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceSubgroupSizeControlFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES,
    pNext: core::ptr::null_mut(),
    subgroupSizeControl: 0,
    computeFullSubgroups: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_subgroupSizeControl(mut self, val: VkBool32) -> Self {
    self.subgroupSizeControl = val;
    self
  }
  #[inline]
  pub const fn with_computeFullSubgroups(mut self, val: VkBool32) -> Self {
    self.computeFullSubgroups = val;
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
/// [VkPhysicalDeviceSubgroupSizeControlProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubgroupSizeControlProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSubgroupSizeControlProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Min, Pot],  No Auto-Validity
  pub minSubgroupSize: u32,
  /// Limit Type: [Max, Pot],  No Auto-Validity
  pub maxSubgroupSize: u32,
  /// Limit Type: [Max],  No Auto-Validity
  pub maxComputeWorkgroupSubgroups: u32,
  /// Limit Type: [Bitmask]
  pub requiredSubgroupSizeStages: VkShaderStageFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceSubgroupSizeControlProperties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceSubgroupSizeControlProperties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceSubgroupSizeControlProperties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceSubgroupSizeControlProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES,
    pNext: core::ptr::null_mut(),
    minSubgroupSize: 0,
    maxSubgroupSize: 0,
    maxComputeWorkgroupSubgroups: 0,
    requiredSubgroupSizeStages: VkShaderStageFlagBits(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_minSubgroupSize(mut self, val: u32) -> Self {
    self.minSubgroupSize = val;
    self
  }
  #[inline]
  pub const fn with_maxSubgroupSize(mut self, val: u32) -> Self {
    self.maxSubgroupSize = val;
    self
  }
  #[inline]
  pub const fn with_maxComputeWorkgroupSubgroups(mut self, val: u32) -> Self {
    self.maxComputeWorkgroupSubgroups = val;
    self
  }
  #[inline]
  pub const fn with_requiredSubgroupSizeStages(mut self, val: VkShaderStageFlags) -> Self {
    self.requiredSubgroupSizeStages = val;
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
/// [VkPipelineShaderStageRequiredSubgroupSizeCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageRequiredSubgroupSizeCreateInfo.html)
///
/// **Extends:** VkPipelineShaderStageCreateInfo, VkShaderCreateInfoEXT.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub requiredSubgroupSize: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPipelineShaderStageRequiredSubgroupSizeCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPipelineShaderStageRequiredSubgroupSizeCreateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineShaderStageCreateInfo<'root>>
  for VkPipelineShaderStageRequiredSubgroupSizeCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_EXT_shader_object"))]
unsafe impl<'child, 'root> VkPNextExtends<VkShaderCreateInfoEXT<'root>>
  for VkPipelineShaderStageRequiredSubgroupSizeCreateInfo<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPipelineShaderStageRequiredSubgroupSizeCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO,
    pNext: core::ptr::null(),
    requiredSubgroupSize: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_requiredSubgroupSize(mut self, val: u32) -> Self {
    self.requiredSubgroupSize = val;
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
  #[cfg(feature = "VK_EXT_shader_object")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkShaderCreateInfoEXT<
    'root,
    T: VkPNextExtends<VkShaderCreateInfoEXT<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDevicePipelineCreationCacheControlFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineCreationCacheControlFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePipelineCreationCacheControlFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub pipelineCreationCacheControl: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDevicePipelineCreationCacheControlFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDevicePipelineCreationCacheControlFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePipelineCreationCacheControlFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePipelineCreationCacheControlFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDevicePipelineCreationCacheControlFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES,
    pNext: core::ptr::null_mut(),
    pipelineCreationCacheControl: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_pipelineCreationCacheControl(mut self, val: VkBool32) -> Self {
    self.pipelineCreationCacheControl = val;
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
/// [VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderZeroInitializeWorkgroupMemory: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderZeroInitializeWorkgroupMemory: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderZeroInitializeWorkgroupMemory(mut self, val: VkBool32) -> Self {
    self.shaderZeroInitializeWorkgroupMemory = val;
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
/// [VkPhysicalDeviceImageRobustnessFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageRobustnessFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceImageRobustnessFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub robustImageAccess: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceImageRobustnessFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceImageRobustnessFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceImageRobustnessFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceImageRobustnessFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceImageRobustnessFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES,
    pNext: core::ptr::null_mut(),
    robustImageAccess: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_robustImageAccess(mut self, val: VkBool32) -> Self {
    self.robustImageAccess = val;
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
/// [VkPhysicalDeviceShaderTerminateInvocationFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderTerminateInvocationFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderTerminateInvocationFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub shaderTerminateInvocation: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderTerminateInvocationFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderTerminateInvocationFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderTerminateInvocationFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderTerminateInvocationFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceShaderTerminateInvocationFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderTerminateInvocation: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderTerminateInvocation(mut self, val: VkBool32) -> Self {
    self.shaderTerminateInvocation = val;
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
/// [VkPhysicalDeviceShaderIntegerDotProductFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderIntegerDotProductFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderIntegerDotProductFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderIntegerDotProduct: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderIntegerDotProductFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderIntegerDotProductFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderIntegerDotProductFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderIntegerDotProductFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceShaderIntegerDotProductFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderIntegerDotProduct: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderIntegerDotProduct(mut self, val: VkBool32) -> Self {
    self.shaderIntegerDotProduct = val;
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
/// [VkPhysicalDeviceShaderIntegerDotProductProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderIntegerDotProductProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderIntegerDotProductProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct8BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct16BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct32BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct64BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
  /// Limit Type: [Max]
  pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderIntegerDotProductProperties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderIntegerDotProductProperties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceShaderIntegerDotProductProperties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
impl<'a> VkPhysicalDeviceShaderIntegerDotProductProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES,
    pNext: core::ptr::null_mut(),
    integerDotProduct8BitUnsignedAccelerated: 0,
    integerDotProduct8BitSignedAccelerated: 0,
    integerDotProduct8BitMixedSignednessAccelerated: 0,
    integerDotProduct4x8BitPackedUnsignedAccelerated: 0,
    integerDotProduct4x8BitPackedSignedAccelerated: 0,
    integerDotProduct4x8BitPackedMixedSignednessAccelerated: 0,
    integerDotProduct16BitUnsignedAccelerated: 0,
    integerDotProduct16BitSignedAccelerated: 0,
    integerDotProduct16BitMixedSignednessAccelerated: 0,
    integerDotProduct32BitUnsignedAccelerated: 0,
    integerDotProduct32BitSignedAccelerated: 0,
    integerDotProduct32BitMixedSignednessAccelerated: 0,
    integerDotProduct64BitUnsignedAccelerated: 0,
    integerDotProduct64BitSignedAccelerated: 0,
    integerDotProduct64BitMixedSignednessAccelerated: 0,
    integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: 0,
    integerDotProductAccumulatingSaturating8BitSignedAccelerated: 0,
    integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: 0,
    integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: 0,
    integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: 0,
    integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: 0,
    integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: 0,
    integerDotProductAccumulatingSaturating16BitSignedAccelerated: 0,
    integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: 0,
    integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: 0,
    integerDotProductAccumulatingSaturating32BitSignedAccelerated: 0,
    integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: 0,
    integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: 0,
    integerDotProductAccumulatingSaturating64BitSignedAccelerated: 0,
    integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_integerDotProduct8BitUnsignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct8BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct8BitSignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct8BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct8BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct8BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct4x8BitPackedUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct4x8BitPackedUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct4x8BitPackedSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct4x8BitPackedSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct4x8BitPackedMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct4x8BitPackedMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct16BitUnsignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct16BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct16BitSignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct16BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct16BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct16BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct32BitUnsignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct32BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct32BitSignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct32BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct32BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct32BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct64BitUnsignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct64BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct64BitSignedAccelerated(mut self, val: VkBool32) -> Self {
    self.integerDotProduct64BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProduct64BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProduct64BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating8BitUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating8BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating8BitSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating8BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating16BitUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating16BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating16BitSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating16BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating32BitUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating32BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating32BitSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating32BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating64BitUnsignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating64BitUnsignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating64BitSignedAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating64BitSignedAccelerated = val;
    self
  }
  #[inline]
  pub const fn with_integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated = val;
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
