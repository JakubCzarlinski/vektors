#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_4",
  feature = "VK_EXT_descriptor_heap",
  feature = "VK_KHR_ray_tracing_pipeline",
  all(
    feature = "VK_EXT_legacy_dithering",
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_KHR_extended_flags"
  ),
  all(
    feature = "VK_EXT_legacy_dithering",
    feature = "VK_KHR_extended_flags",
    feature = "VK_VERSION_1_3"
  ),
  feature = "VK_KHR_maintenance5",
  feature = "VK_KHR_pipeline_binary",
  feature = "VK_EXT_device_generated_commands",
  feature = "VK_VALVE_fragment_density_map_layered",
  all(feature = "VK_KHR_extended_flags", feature = "VK_KHR_opacity_micromap"),
  feature = "VK_EXT_shader_64bit_indexing"
))]
use crate::enums::VkPipelineCreateFlagBits2;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_4",
  feature = "VK_EXT_pipeline_robustness"
))]
use crate::enums::VkPipelineRobustnessBufferBehavior;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_4",
  feature = "VK_EXT_pipeline_robustness"
))]
use crate::enums::VkPipelineRobustnessImageBehavior;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkShaderStageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkComputePipelineCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSet;
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
use crate::types::VkDescriptorUpdateTemplate;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineLayout;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineLayoutCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineShaderStageCreateInfo;
#[cfg(feature = "VK_NV_push_constant_bank")]
use crate::types::VkPushConstantBankInfoNV;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkRayTracingPipelineCreateInfoKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkRayTracingPipelineCreateInfoNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkShaderStageFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkWriteDescriptorSet;
use core::ffi::c_void;
/// [VkPipelineCreateFlags2](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags2.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
pub type VkPipelineCreateFlags2 = VkPipelineCreateFlagBits2;
/// [VkPipelineCreateFlags2CreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags2CreateInfo.html)
///
/// **Extends:** VkComputePipelineCreateInfo, VkGraphicsPipelineCreateInfo, VkRayTracingPipelineCreateInfoNV, VkRayTracingPipelineCreateInfoKHR.
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineCreateFlags2CreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_CREATE_FLAGS_2_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkPipelineCreateFlags2,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkPipelineCreateFlags2CreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPipelineCreateFlags2CreateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkComputePipelineCreateInfo<'root>>
  for VkPipelineCreateFlags2CreateInfo<'child>
{
}
#[cfg(all(
  feature = "VK_COMPUTE_VERSION_1_4",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkPipelineCreateFlags2CreateInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_NV_ray_tracing"))]
unsafe impl<'child, 'root> VkPNextExtends<VkRayTracingPipelineCreateInfoNV<'root>>
  for VkPipelineCreateFlags2CreateInfo<'child>
{
}
#[cfg(all(
  feature = "VK_COMPUTE_VERSION_1_4",
  feature = "VK_KHR_ray_tracing_pipeline"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRayTracingPipelineCreateInfoKHR<'root>>
  for VkPipelineCreateFlags2CreateInfo<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkPipelineCreateFlags2CreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_CREATE_FLAGS_2_CREATE_INFO,
    pNext: core::ptr::null(),
    flags: VkPipelineCreateFlagBits2(0),
    _marker: core::marker::PhantomData,
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
}
/// [VkPhysicalDevicePushDescriptorProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePushDescriptorProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePushDescriptorProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxPushDescriptors: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDevicePushDescriptorProperties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDevicePushDescriptorProperties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDevicePushDescriptorProperties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkPhysicalDevicePushDescriptorProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES,
    pNext: core::ptr::null_mut(),
    maxPushDescriptors: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_maxPushDescriptors(mut self, val: u32) -> Self {
    self.maxPushDescriptors = val;
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
/// [VkPhysicalDevicePipelineProtectedAccessFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineProtectedAccessFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePipelineProtectedAccessFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub pipelineProtectedAccess: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDevicePipelineProtectedAccessFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDevicePipelineProtectedAccessFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePipelineProtectedAccessFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePipelineProtectedAccessFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkPhysicalDevicePipelineProtectedAccessFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES,
    pNext: core::ptr::null_mut(),
    pipelineProtectedAccess: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_pipelineProtectedAccess(mut self, val: VkBool32) -> Self {
    self.pipelineProtectedAccess = val;
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
/// [VkPhysicalDevicePipelineRobustnessFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineRobustnessFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePipelineRobustnessFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub pipelineRobustness: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDevicePipelineRobustnessFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDevicePipelineRobustnessFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevicePipelineRobustnessFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevicePipelineRobustnessFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkPhysicalDevicePipelineRobustnessFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES,
    pNext: core::ptr::null_mut(),
    pipelineRobustness: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_pipelineRobustness(mut self, val: VkBool32) -> Self {
    self.pipelineRobustness = val;
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
/// [VkPipelineRobustnessCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessCreateInfo.html)
///
/// **Extends:** VkGraphicsPipelineCreateInfo, VkComputePipelineCreateInfo, VkPipelineShaderStageCreateInfo, VkRayTracingPipelineCreateInfoKHR.
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineRobustnessCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *const c_void,
  pub storageBuffers: VkPipelineRobustnessBufferBehavior,
  pub uniformBuffers: VkPipelineRobustnessBufferBehavior,
  pub vertexInputs: VkPipelineRobustnessBufferBehavior,
  pub images: VkPipelineRobustnessImageBehavior,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkPipelineRobustnessCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPipelineRobustnessCreateInfo<'a> {}
#[cfg(all(
  feature = "VK_COMPUTE_VERSION_1_4",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkPipelineRobustnessCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkComputePipelineCreateInfo<'root>>
  for VkPipelineRobustnessCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineShaderStageCreateInfo<'root>>
  for VkPipelineRobustnessCreateInfo<'child>
{
}
#[cfg(all(
  feature = "VK_COMPUTE_VERSION_1_4",
  feature = "VK_KHR_ray_tracing_pipeline"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRayTracingPipelineCreateInfoKHR<'root>>
  for VkPipelineRobustnessCreateInfo<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkPipelineRobustnessCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_ROBUSTNESS_CREATE_INFO,
    pNext: core::ptr::null(),
    storageBuffers: VkPipelineRobustnessBufferBehavior(0),
    uniformBuffers: VkPipelineRobustnessBufferBehavior(0),
    vertexInputs: VkPipelineRobustnessBufferBehavior(0),
    images: VkPipelineRobustnessImageBehavior(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_storageBuffers(mut self, val: VkPipelineRobustnessBufferBehavior) -> Self {
    self.storageBuffers = val;
    self
  }
  #[inline]
  pub const fn with_uniformBuffers(mut self, val: VkPipelineRobustnessBufferBehavior) -> Self {
    self.uniformBuffers = val;
    self
  }
  #[inline]
  pub const fn with_vertexInputs(mut self, val: VkPipelineRobustnessBufferBehavior) -> Self {
    self.vertexInputs = val;
    self
  }
  #[inline]
  pub const fn with_images(mut self, val: VkPipelineRobustnessImageBehavior) -> Self {
    self.images = val;
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
}
/// [VkPhysicalDevicePipelineRobustnessProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineRobustnessProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePipelineRobustnessProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Exact]
  pub defaultRobustnessStorageBuffers: VkPipelineRobustnessBufferBehavior,
  /// Limit Type: [Exact]
  pub defaultRobustnessUniformBuffers: VkPipelineRobustnessBufferBehavior,
  /// Limit Type: [Exact]
  pub defaultRobustnessVertexInputs: VkPipelineRobustnessBufferBehavior,
  /// Limit Type: [Exact]
  pub defaultRobustnessImages: VkPipelineRobustnessImageBehavior,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDevicePipelineRobustnessProperties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDevicePipelineRobustnessProperties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDevicePipelineRobustnessProperties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkPhysicalDevicePipelineRobustnessProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES,
    pNext: core::ptr::null_mut(),
    defaultRobustnessStorageBuffers: VkPipelineRobustnessBufferBehavior(0),
    defaultRobustnessUniformBuffers: VkPipelineRobustnessBufferBehavior(0),
    defaultRobustnessVertexInputs: VkPipelineRobustnessBufferBehavior(0),
    defaultRobustnessImages: VkPipelineRobustnessImageBehavior(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_defaultRobustnessStorageBuffers(
    mut self,
    val: VkPipelineRobustnessBufferBehavior,
  ) -> Self {
    self.defaultRobustnessStorageBuffers = val;
    self
  }
  #[inline]
  pub const fn with_defaultRobustnessUniformBuffers(
    mut self,
    val: VkPipelineRobustnessBufferBehavior,
  ) -> Self {
    self.defaultRobustnessUniformBuffers = val;
    self
  }
  #[inline]
  pub const fn with_defaultRobustnessVertexInputs(
    mut self,
    val: VkPipelineRobustnessBufferBehavior,
  ) -> Self {
    self.defaultRobustnessVertexInputs = val;
    self
  }
  #[inline]
  pub const fn with_defaultRobustnessImages(
    mut self,
    val: VkPipelineRobustnessImageBehavior,
  ) -> Self {
    self.defaultRobustnessImages = val;
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
/// [VkBindDescriptorSetsInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindDescriptorSetsInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindDescriptorSetsInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BIND_DESCRIPTOR_SETS_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub stageFlags: VkShaderStageFlags,
  /// Optional: true
  pub layout: VkPipelineLayout,
  /// Optional: true
  pub firstSet: u32,
  pub descriptorSetCount: u32,
  /// Length: descriptorSetCount
  pub pDescriptorSets: *const VkDescriptorSet,
  /// Optional: true
  pub dynamicOffsetCount: u32,
  /// Optional: pointer, values optional,  Length: dynamicOffsetCount
  pub pDynamicOffsets: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkBindDescriptorSetsInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkBindDescriptorSetsInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkBindDescriptorSetsInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BIND_DESCRIPTOR_SETS_INFO,
    pNext: core::ptr::null(),
    stageFlags: VkShaderStageFlagBits(0),
    layout: VkPipelineLayout::DEFAULT,
    firstSet: 0,
    descriptorSetCount: 0,
    pDescriptorSets: core::ptr::null(),
    dynamicOffsetCount: 0,
    pDynamicOffsets: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_stageFlags(mut self, val: VkShaderStageFlags) -> Self {
    self.stageFlags = val;
    self
  }
  #[inline]
  pub const fn with_layout(mut self, val: VkPipelineLayout) -> Self {
    self.layout = val;
    self
  }
  #[inline]
  pub const fn with_firstSet(mut self, val: u32) -> Self {
    self.firstSet = val;
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
  pub const fn with_pDescriptorSets(mut self, val: &'a [VkDescriptorSet]) -> Self {
    self.descriptorSetCount = val.len() as u32;
    self.pDescriptorSets = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_dynamicOffsetCount(mut self, val: u32) -> Self {
    self.dynamicOffsetCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDynamicOffsets(mut self, val: &'a [u32]) -> Self {
    self.dynamicOffsetCount = val.len() as u32;
    self.pDynamicOffsets = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineLayoutCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineLayoutCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineLayoutCreateInfo<'child>).cast::<c_void>();
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
}
/// [VkPushConstantsInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPushConstantsInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPushConstantsInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PUSH_CONSTANTS_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub layout: VkPipelineLayout,
  pub stageFlags: VkShaderStageFlags,
  /// Optional: true
  pub offset: u32,
  pub size: u32,
  /// Length: size
  pub pValues: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkPushConstantsInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPushConstantsInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkPushConstantsInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PUSH_CONSTANTS_INFO,
    pNext: core::ptr::null(),
    layout: VkPipelineLayout::DEFAULT,
    stageFlags: VkShaderStageFlagBits(0),
    offset: 0,
    size: 0,
    pValues: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_layout(mut self, val: VkPipelineLayout) -> Self {
    self.layout = val;
    self
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
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pValues(mut self, val: &'a [u8]) -> Self {
    self.size = val.len() as u32;
    self.pValues = val.as_ptr().cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineLayoutCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineLayoutCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineLayoutCreateInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_NV_push_constant_bank")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPushConstantBankInfoNV<'child>(
    mut self,
    val: &'a VkPushConstantBankInfoNV<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPushConstantBankInfoNV<'child>).cast::<c_void>();
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
}
/// [VkPushDescriptorSetInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPushDescriptorSetInfo.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPushDescriptorSetInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub stageFlags: VkShaderStageFlags,
  /// Optional: true
  pub layout: VkPipelineLayout,
  /// Optional: true
  pub set: u32,
  pub descriptorWriteCount: u32,
  /// Length: descriptorWriteCount
  pub pDescriptorWrites: *const VkWriteDescriptorSet<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkPushDescriptorSetInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPushDescriptorSetInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkPushDescriptorSetInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PUSH_DESCRIPTOR_SET_INFO,
    pNext: core::ptr::null(),
    stageFlags: VkShaderStageFlagBits(0),
    layout: VkPipelineLayout::DEFAULT,
    set: 0,
    descriptorWriteCount: 0,
    pDescriptorWrites: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_stageFlags(mut self, val: VkShaderStageFlags) -> Self {
    self.stageFlags = val;
    self
  }
  #[inline]
  pub const fn with_layout(mut self, val: VkPipelineLayout) -> Self {
    self.layout = val;
    self
  }
  #[inline]
  pub const fn with_set(mut self, val: u32) -> Self {
    self.set = val;
    self
  }
  #[inline]
  pub const fn with_descriptorWriteCount(mut self, val: u32) -> Self {
    self.descriptorWriteCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDescriptorWrites(mut self, val: &'a [VkWriteDescriptorSet<'a>]) -> Self {
    self.descriptorWriteCount = val.len() as u32;
    self.pDescriptorWrites = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineLayoutCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineLayoutCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineLayoutCreateInfo<'child>).cast::<c_void>();
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
}
/// [VkPushDescriptorSetWithTemplateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPushDescriptorSetWithTemplateInfo.html)
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", not(feature = "VKSC_VERSION_1_0")))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPushDescriptorSetWithTemplateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
  /// Optional: true
  pub layout: VkPipelineLayout,
  /// Optional: true
  pub set: u32,
  pub pData: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Send for VkPushDescriptorSetWithTemplateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", not(feature = "VKSC_VERSION_1_0")))]
unsafe impl<'a> Sync for VkPushDescriptorSetWithTemplateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", not(feature = "VKSC_VERSION_1_0")))]
impl<'a> VkPushDescriptorSetWithTemplateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO,
    pNext: core::ptr::null(),
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate::DEFAULT,
    layout: VkPipelineLayout::DEFAULT,
    set: 0,
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
  pub const fn with_descriptorUpdateTemplate(mut self, val: VkDescriptorUpdateTemplate) -> Self {
    self.descriptorUpdateTemplate = val;
    self
  }
  #[inline]
  pub const fn with_layout(mut self, val: VkPipelineLayout) -> Self {
    self.layout = val;
    self
  }
  #[inline]
  pub const fn with_set(mut self, val: u32) -> Self {
    self.set = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pData(mut self, val: *const c_void) -> Self {
    self.pData = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkPipelineLayoutCreateInfo<'child>(
    mut self,
    val: &'a VkPipelineLayoutCreateInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkPipelineLayoutCreateInfo<'child>).cast::<c_void>();
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
}
/// [VkPhysicalDeviceShaderSubgroupRotateFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSubgroupRotateFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderSubgroupRotateFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderSubgroupRotate: VkBool32,
  pub shaderSubgroupRotateClustered: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderSubgroupRotateFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderSubgroupRotateFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderSubgroupRotateFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderSubgroupRotateFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceShaderSubgroupRotateFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderSubgroupRotate: 0,
    shaderSubgroupRotateClustered: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderSubgroupRotate(mut self, val: VkBool32) -> Self {
    self.shaderSubgroupRotate = val;
    self
  }
  #[inline]
  pub const fn with_shaderSubgroupRotateClustered(mut self, val: VkBool32) -> Self {
    self.shaderSubgroupRotateClustered = val;
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
/// [VkPhysicalDeviceShaderExpectAssumeFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderExpectAssumeFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderExpectAssumeFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub shaderExpectAssume: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderExpectAssumeFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderExpectAssumeFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderExpectAssumeFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderExpectAssumeFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceShaderExpectAssumeFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderExpectAssume: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderExpectAssume(mut self, val: VkBool32) -> Self {
    self.shaderExpectAssume = val;
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
/// [VkPhysicalDeviceShaderFloatControls2Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderFloatControls2Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderFloatControls2Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderFloatControls2: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderFloatControls2Features<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderFloatControls2Features<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderFloatControls2Features<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderFloatControls2Features<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceShaderFloatControls2Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderFloatControls2: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderFloatControls2(mut self, val: VkBool32) -> Self {
    self.shaderFloatControls2 = val;
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
