#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_2",
  feature = "VK_EXT_descriptor_indexing"
))]
use crate::enums::VkDescriptorBindingFlagBits;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_2",
  feature = "VK_EXT_sampler_filter_minmax",
  feature = "VK_QCOM_filter_cubic_clamp"
))]
use crate::enums::VkSamplerReductionMode;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_2",
  feature = "VK_KHR_shader_float_controls"
))]
use crate::enums::VkShaderFloatControlsIndependence;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSetAllocateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSetLayoutCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkDescriptorSetLayoutSupport;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkSamplerCreateInfo;
use core::ffi::c_void;
/// [VkDescriptorBindingFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlags.html)
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
pub type VkDescriptorBindingFlags = VkDescriptorBindingFlagBits;
/// [VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// No Auto-Validity
  pub shaderSubgroupExtendedTypes: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderSubgroupExtendedTypes: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderSubgroupExtendedTypes(mut self, val: VkBool32) -> Self {
    self.shaderSubgroupExtendedTypes = val;
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
/// [VkPhysicalDeviceSamplerFilterMinmaxProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSamplerFilterMinmaxProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub filterMinmaxSingleComponentFormats: VkBool32,
  /// Limit Type: [Max]
  pub filterMinmaxImageComponentMapping: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceSamplerFilterMinmaxProperties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceSamplerFilterMinmaxProperties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceSamplerFilterMinmaxProperties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceSamplerFilterMinmaxProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES,
    pNext: core::ptr::null_mut(),
    filterMinmaxSingleComponentFormats: 0,
    filterMinmaxImageComponentMapping: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_filterMinmaxSingleComponentFormats(mut self, val: VkBool32) -> Self {
    self.filterMinmaxSingleComponentFormats = val;
    self
  }
  #[inline]
  pub const fn with_filterMinmaxImageComponentMapping(mut self, val: VkBool32) -> Self {
    self.filterMinmaxImageComponentMapping = val;
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
/// [VkSamplerReductionModeCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerReductionModeCreateInfo.html)
///
/// **Extends:** VkSamplerCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSamplerReductionModeCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub reductionMode: VkSamplerReductionMode,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkSamplerReductionModeCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkSamplerReductionModeCreateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSamplerCreateInfo<'root>>
  for VkSamplerReductionModeCreateInfo<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkSamplerReductionModeCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO,
    pNext: core::ptr::null(),
    reductionMode: VkSamplerReductionMode(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_reductionMode(mut self, val: VkSamplerReductionMode) -> Self {
    self.reductionMode = val;
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
/// [VkPhysicalDeviceShaderFloat16Int8Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderFloat16Int8Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderFloat16: VkBool32,
  pub shaderInt8: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderFloat16Int8Features<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderFloat16Int8Features<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderFloat16Int8Features<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderFloat16Int8Features<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceShaderFloat16Int8Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderFloat16: 0,
    shaderInt8: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderFloat16(mut self, val: VkBool32) -> Self {
    self.shaderFloat16 = val;
    self
  }
  #[inline]
  pub const fn with_shaderInt8(mut self, val: VkBool32) -> Self {
    self.shaderInt8 = val;
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
/// [VkPhysicalDeviceFloatControlsProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFloatControlsProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceFloatControlsProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Exact]
  pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
  /// Limit Type: [Exact]
  pub roundingModeIndependence: VkShaderFloatControlsIndependence,
  /// Limit Type: [Max]
  pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
  /// Limit Type: [Max]
  pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
  /// Limit Type: [Max]
  pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
  /// Limit Type: [Max]
  pub shaderDenormPreserveFloat16: VkBool32,
  /// Limit Type: [Max]
  pub shaderDenormPreserveFloat32: VkBool32,
  /// Limit Type: [Max]
  pub shaderDenormPreserveFloat64: VkBool32,
  /// Limit Type: [Max]
  pub shaderDenormFlushToZeroFloat16: VkBool32,
  /// Limit Type: [Max]
  pub shaderDenormFlushToZeroFloat32: VkBool32,
  /// Limit Type: [Max]
  pub shaderDenormFlushToZeroFloat64: VkBool32,
  /// Limit Type: [Max]
  pub shaderRoundingModeRTEFloat16: VkBool32,
  /// Limit Type: [Max]
  pub shaderRoundingModeRTEFloat32: VkBool32,
  /// Limit Type: [Max]
  pub shaderRoundingModeRTEFloat64: VkBool32,
  /// Limit Type: [Max]
  pub shaderRoundingModeRTZFloat16: VkBool32,
  /// Limit Type: [Max]
  pub shaderRoundingModeRTZFloat32: VkBool32,
  /// Limit Type: [Max]
  pub shaderRoundingModeRTZFloat64: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceFloatControlsProperties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceFloatControlsProperties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceFloatControlsProperties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceFloatControlsProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES,
    pNext: core::ptr::null_mut(),
    denormBehaviorIndependence: VkShaderFloatControlsIndependence(0),
    roundingModeIndependence: VkShaderFloatControlsIndependence(0),
    shaderSignedZeroInfNanPreserveFloat16: 0,
    shaderSignedZeroInfNanPreserveFloat32: 0,
    shaderSignedZeroInfNanPreserveFloat64: 0,
    shaderDenormPreserveFloat16: 0,
    shaderDenormPreserveFloat32: 0,
    shaderDenormPreserveFloat64: 0,
    shaderDenormFlushToZeroFloat16: 0,
    shaderDenormFlushToZeroFloat32: 0,
    shaderDenormFlushToZeroFloat64: 0,
    shaderRoundingModeRTEFloat16: 0,
    shaderRoundingModeRTEFloat32: 0,
    shaderRoundingModeRTEFloat64: 0,
    shaderRoundingModeRTZFloat16: 0,
    shaderRoundingModeRTZFloat32: 0,
    shaderRoundingModeRTZFloat64: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_denormBehaviorIndependence(
    mut self,
    val: VkShaderFloatControlsIndependence,
  ) -> Self {
    self.denormBehaviorIndependence = val;
    self
  }
  #[inline]
  pub const fn with_roundingModeIndependence(
    mut self,
    val: VkShaderFloatControlsIndependence,
  ) -> Self {
    self.roundingModeIndependence = val;
    self
  }
  #[inline]
  pub const fn with_shaderSignedZeroInfNanPreserveFloat16(mut self, val: VkBool32) -> Self {
    self.shaderSignedZeroInfNanPreserveFloat16 = val;
    self
  }
  #[inline]
  pub const fn with_shaderSignedZeroInfNanPreserveFloat32(mut self, val: VkBool32) -> Self {
    self.shaderSignedZeroInfNanPreserveFloat32 = val;
    self
  }
  #[inline]
  pub const fn with_shaderSignedZeroInfNanPreserveFloat64(mut self, val: VkBool32) -> Self {
    self.shaderSignedZeroInfNanPreserveFloat64 = val;
    self
  }
  #[inline]
  pub const fn with_shaderDenormPreserveFloat16(mut self, val: VkBool32) -> Self {
    self.shaderDenormPreserveFloat16 = val;
    self
  }
  #[inline]
  pub const fn with_shaderDenormPreserveFloat32(mut self, val: VkBool32) -> Self {
    self.shaderDenormPreserveFloat32 = val;
    self
  }
  #[inline]
  pub const fn with_shaderDenormPreserveFloat64(mut self, val: VkBool32) -> Self {
    self.shaderDenormPreserveFloat64 = val;
    self
  }
  #[inline]
  pub const fn with_shaderDenormFlushToZeroFloat16(mut self, val: VkBool32) -> Self {
    self.shaderDenormFlushToZeroFloat16 = val;
    self
  }
  #[inline]
  pub const fn with_shaderDenormFlushToZeroFloat32(mut self, val: VkBool32) -> Self {
    self.shaderDenormFlushToZeroFloat32 = val;
    self
  }
  #[inline]
  pub const fn with_shaderDenormFlushToZeroFloat64(mut self, val: VkBool32) -> Self {
    self.shaderDenormFlushToZeroFloat64 = val;
    self
  }
  #[inline]
  pub const fn with_shaderRoundingModeRTEFloat16(mut self, val: VkBool32) -> Self {
    self.shaderRoundingModeRTEFloat16 = val;
    self
  }
  #[inline]
  pub const fn with_shaderRoundingModeRTEFloat32(mut self, val: VkBool32) -> Self {
    self.shaderRoundingModeRTEFloat32 = val;
    self
  }
  #[inline]
  pub const fn with_shaderRoundingModeRTEFloat64(mut self, val: VkBool32) -> Self {
    self.shaderRoundingModeRTEFloat64 = val;
    self
  }
  #[inline]
  pub const fn with_shaderRoundingModeRTZFloat16(mut self, val: VkBool32) -> Self {
    self.shaderRoundingModeRTZFloat16 = val;
    self
  }
  #[inline]
  pub const fn with_shaderRoundingModeRTZFloat32(mut self, val: VkBool32) -> Self {
    self.shaderRoundingModeRTZFloat32 = val;
    self
  }
  #[inline]
  pub const fn with_shaderRoundingModeRTZFloat64(mut self, val: VkBool32) -> Self {
    self.shaderRoundingModeRTZFloat64 = val;
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
/// [VkPhysicalDeviceDescriptorIndexingFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorIndexingFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
  pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
  pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
  pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
  pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
  pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
  pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
  pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
  pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
  pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
  pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
  pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
  pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
  pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
  pub descriptorBindingPartiallyBound: VkBool32,
  pub descriptorBindingVariableDescriptorCount: VkBool32,
  pub runtimeDescriptorArray: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceDescriptorIndexingFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceDescriptorIndexingFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDescriptorIndexingFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDescriptorIndexingFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceDescriptorIndexingFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderInputAttachmentArrayDynamicIndexing: 0,
    shaderUniformTexelBufferArrayDynamicIndexing: 0,
    shaderStorageTexelBufferArrayDynamicIndexing: 0,
    shaderUniformBufferArrayNonUniformIndexing: 0,
    shaderSampledImageArrayNonUniformIndexing: 0,
    shaderStorageBufferArrayNonUniformIndexing: 0,
    shaderStorageImageArrayNonUniformIndexing: 0,
    shaderInputAttachmentArrayNonUniformIndexing: 0,
    shaderUniformTexelBufferArrayNonUniformIndexing: 0,
    shaderStorageTexelBufferArrayNonUniformIndexing: 0,
    descriptorBindingUniformBufferUpdateAfterBind: 0,
    descriptorBindingSampledImageUpdateAfterBind: 0,
    descriptorBindingStorageImageUpdateAfterBind: 0,
    descriptorBindingStorageBufferUpdateAfterBind: 0,
    descriptorBindingUniformTexelBufferUpdateAfterBind: 0,
    descriptorBindingStorageTexelBufferUpdateAfterBind: 0,
    descriptorBindingUpdateUnusedWhilePending: 0,
    descriptorBindingPartiallyBound: 0,
    descriptorBindingVariableDescriptorCount: 0,
    runtimeDescriptorArray: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderInputAttachmentArrayDynamicIndexing(mut self, val: VkBool32) -> Self {
    self.shaderInputAttachmentArrayDynamicIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderUniformTexelBufferArrayDynamicIndexing(mut self, val: VkBool32) -> Self {
    self.shaderUniformTexelBufferArrayDynamicIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageTexelBufferArrayDynamicIndexing(mut self, val: VkBool32) -> Self {
    self.shaderStorageTexelBufferArrayDynamicIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderUniformBufferArrayNonUniformIndexing(mut self, val: VkBool32) -> Self {
    self.shaderUniformBufferArrayNonUniformIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderSampledImageArrayNonUniformIndexing(mut self, val: VkBool32) -> Self {
    self.shaderSampledImageArrayNonUniformIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageBufferArrayNonUniformIndexing(mut self, val: VkBool32) -> Self {
    self.shaderStorageBufferArrayNonUniformIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageImageArrayNonUniformIndexing(mut self, val: VkBool32) -> Self {
    self.shaderStorageImageArrayNonUniformIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderInputAttachmentArrayNonUniformIndexing(mut self, val: VkBool32) -> Self {
    self.shaderInputAttachmentArrayNonUniformIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderUniformTexelBufferArrayNonUniformIndexing(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.shaderUniformTexelBufferArrayNonUniformIndexing = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageTexelBufferArrayNonUniformIndexing(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.shaderStorageTexelBufferArrayNonUniformIndexing = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingUniformBufferUpdateAfterBind(mut self, val: VkBool32) -> Self {
    self.descriptorBindingUniformBufferUpdateAfterBind = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingSampledImageUpdateAfterBind(mut self, val: VkBool32) -> Self {
    self.descriptorBindingSampledImageUpdateAfterBind = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingStorageImageUpdateAfterBind(mut self, val: VkBool32) -> Self {
    self.descriptorBindingStorageImageUpdateAfterBind = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingStorageBufferUpdateAfterBind(mut self, val: VkBool32) -> Self {
    self.descriptorBindingStorageBufferUpdateAfterBind = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingUniformTexelBufferUpdateAfterBind(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.descriptorBindingUniformTexelBufferUpdateAfterBind = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingStorageTexelBufferUpdateAfterBind(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.descriptorBindingStorageTexelBufferUpdateAfterBind = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingUpdateUnusedWhilePending(mut self, val: VkBool32) -> Self {
    self.descriptorBindingUpdateUnusedWhilePending = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingPartiallyBound(mut self, val: VkBool32) -> Self {
    self.descriptorBindingPartiallyBound = val;
    self
  }
  #[inline]
  pub const fn with_descriptorBindingVariableDescriptorCount(mut self, val: VkBool32) -> Self {
    self.descriptorBindingVariableDescriptorCount = val;
    self
  }
  #[inline]
  pub const fn with_runtimeDescriptorArray(mut self, val: VkBool32) -> Self {
    self.runtimeDescriptorArray = val;
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
/// [VkPhysicalDeviceDescriptorIndexingProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorIndexingProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxUpdateAfterBindDescriptorsInAllPools: u32,
  /// Limit Type: [Max]
  pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
  /// Limit Type: [Max]
  pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
  /// Limit Type: [Max]
  pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
  /// Limit Type: [Max]
  pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
  /// Limit Type: [Max]
  pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
  /// Limit Type: [Max]
  pub robustBufferAccessUpdateAfterBind: VkBool32,
  /// Limit Type: [Max]
  pub quadDivergentImplicitLod: VkBool32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
  /// Limit Type: [Max]
  pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
  /// Limit Type: [Max]
  pub maxPerStageUpdateAfterBindResources: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUpdateAfterBindSamplers: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
  /// Limit Type: [Max]
  pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceDescriptorIndexingProperties<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceDescriptorIndexingProperties<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceDescriptorIndexingProperties<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceDescriptorIndexingProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES,
    pNext: core::ptr::null_mut(),
    maxUpdateAfterBindDescriptorsInAllPools: 0,
    shaderUniformBufferArrayNonUniformIndexingNative: 0,
    shaderSampledImageArrayNonUniformIndexingNative: 0,
    shaderStorageBufferArrayNonUniformIndexingNative: 0,
    shaderStorageImageArrayNonUniformIndexingNative: 0,
    shaderInputAttachmentArrayNonUniformIndexingNative: 0,
    robustBufferAccessUpdateAfterBind: 0,
    quadDivergentImplicitLod: 0,
    maxPerStageDescriptorUpdateAfterBindSamplers: 0,
    maxPerStageDescriptorUpdateAfterBindUniformBuffers: 0,
    maxPerStageDescriptorUpdateAfterBindStorageBuffers: 0,
    maxPerStageDescriptorUpdateAfterBindSampledImages: 0,
    maxPerStageDescriptorUpdateAfterBindStorageImages: 0,
    maxPerStageDescriptorUpdateAfterBindInputAttachments: 0,
    maxPerStageUpdateAfterBindResources: 0,
    maxDescriptorSetUpdateAfterBindSamplers: 0,
    maxDescriptorSetUpdateAfterBindUniformBuffers: 0,
    maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: 0,
    maxDescriptorSetUpdateAfterBindStorageBuffers: 0,
    maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: 0,
    maxDescriptorSetUpdateAfterBindSampledImages: 0,
    maxDescriptorSetUpdateAfterBindStorageImages: 0,
    maxDescriptorSetUpdateAfterBindInputAttachments: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_maxUpdateAfterBindDescriptorsInAllPools(mut self, val: u32) -> Self {
    self.maxUpdateAfterBindDescriptorsInAllPools = val;
    self
  }
  #[inline]
  pub const fn with_shaderUniformBufferArrayNonUniformIndexingNative(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.shaderUniformBufferArrayNonUniformIndexingNative = val;
    self
  }
  #[inline]
  pub const fn with_shaderSampledImageArrayNonUniformIndexingNative(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.shaderSampledImageArrayNonUniformIndexingNative = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageBufferArrayNonUniformIndexingNative(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.shaderStorageBufferArrayNonUniformIndexingNative = val;
    self
  }
  #[inline]
  pub const fn with_shaderStorageImageArrayNonUniformIndexingNative(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.shaderStorageImageArrayNonUniformIndexingNative = val;
    self
  }
  #[inline]
  pub const fn with_shaderInputAttachmentArrayNonUniformIndexingNative(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.shaderInputAttachmentArrayNonUniformIndexingNative = val;
    self
  }
  #[inline]
  pub const fn with_robustBufferAccessUpdateAfterBind(mut self, val: VkBool32) -> Self {
    self.robustBufferAccessUpdateAfterBind = val;
    self
  }
  #[inline]
  pub const fn with_quadDivergentImplicitLod(mut self, val: VkBool32) -> Self {
    self.quadDivergentImplicitLod = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorUpdateAfterBindSamplers(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorUpdateAfterBindSamplers = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorUpdateAfterBindUniformBuffers(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorUpdateAfterBindUniformBuffers = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorUpdateAfterBindStorageBuffers(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorUpdateAfterBindStorageBuffers = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorUpdateAfterBindSampledImages(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorUpdateAfterBindSampledImages = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorUpdateAfterBindStorageImages(mut self, val: u32) -> Self {
    self.maxPerStageDescriptorUpdateAfterBindStorageImages = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageDescriptorUpdateAfterBindInputAttachments(
    mut self,
    val: u32,
  ) -> Self {
    self.maxPerStageDescriptorUpdateAfterBindInputAttachments = val;
    self
  }
  #[inline]
  pub const fn with_maxPerStageUpdateAfterBindResources(mut self, val: u32) -> Self {
    self.maxPerStageUpdateAfterBindResources = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUpdateAfterBindSamplers(mut self, val: u32) -> Self {
    self.maxDescriptorSetUpdateAfterBindSamplers = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUpdateAfterBindUniformBuffers(mut self, val: u32) -> Self {
    self.maxDescriptorSetUpdateAfterBindUniformBuffers = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUpdateAfterBindUniformBuffersDynamic(
    mut self,
    val: u32,
  ) -> Self {
    self.maxDescriptorSetUpdateAfterBindUniformBuffersDynamic = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUpdateAfterBindStorageBuffers(mut self, val: u32) -> Self {
    self.maxDescriptorSetUpdateAfterBindStorageBuffers = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUpdateAfterBindStorageBuffersDynamic(
    mut self,
    val: u32,
  ) -> Self {
    self.maxDescriptorSetUpdateAfterBindStorageBuffersDynamic = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUpdateAfterBindSampledImages(mut self, val: u32) -> Self {
    self.maxDescriptorSetUpdateAfterBindSampledImages = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUpdateAfterBindStorageImages(mut self, val: u32) -> Self {
    self.maxDescriptorSetUpdateAfterBindStorageImages = val;
    self
  }
  #[inline]
  pub const fn with_maxDescriptorSetUpdateAfterBindInputAttachments(mut self, val: u32) -> Self {
    self.maxDescriptorSetUpdateAfterBindInputAttachments = val;
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
/// [VkDescriptorSetLayoutBindingFlagsCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutBindingFlagsCreateInfo.html)
///
/// **Extends:** VkDescriptorSetLayoutCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub bindingCount: u32,
  /// Optional: pointer required, values optional if pointer not null,  Length: bindingCount
  pub pBindingFlags: *const VkDescriptorBindingFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkDescriptorSetLayoutBindingFlagsCreateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkDescriptorSetLayoutBindingFlagsCreateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDescriptorSetLayoutCreateInfo<'root>>
  for VkDescriptorSetLayoutBindingFlagsCreateInfo<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkDescriptorSetLayoutBindingFlagsCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO,
    pNext: core::ptr::null(),
    bindingCount: 0,
    pBindingFlags: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_bindingCount(mut self, val: u32) -> Self {
    self.bindingCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pBindingFlags(mut self, val: &'a [VkDescriptorBindingFlags]) -> Self {
    self.bindingCount = val.len() as u32;
    self.pBindingFlags = val.as_ptr();
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
/// [VkDescriptorSetVariableDescriptorCountAllocateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetVariableDescriptorCountAllocateInfo.html)
///
/// **Extends:** VkDescriptorSetAllocateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub descriptorSetCount: u32,
  /// Length: descriptorSetCount
  pub pDescriptorCounts: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkDescriptorSetVariableDescriptorCountAllocateInfo<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkDescriptorSetVariableDescriptorCountAllocateInfo<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDescriptorSetAllocateInfo<'root>>
  for VkDescriptorSetVariableDescriptorCountAllocateInfo<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkDescriptorSetVariableDescriptorCountAllocateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO,
    pNext: core::ptr::null(),
    descriptorSetCount: 0,
    pDescriptorCounts: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_descriptorSetCount(mut self, val: u32) -> Self {
    self.descriptorSetCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDescriptorCounts(mut self, val: &'a [u32]) -> Self {
    self.descriptorSetCount = val.len() as u32;
    self.pDescriptorCounts = val.as_ptr();
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
/// [VkDescriptorSetVariableDescriptorCountLayoutSupport](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetVariableDescriptorCountLayoutSupport.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkDescriptorSetLayoutSupport.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub maxVariableDescriptorCount: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkDescriptorSetVariableDescriptorCountLayoutSupport<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkDescriptorSetVariableDescriptorCountLayoutSupport<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_COMPUTE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDescriptorSetLayoutSupport<'root>>
  for VkDescriptorSetVariableDescriptorCountLayoutSupport<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkDescriptorSetVariableDescriptorCountLayoutSupport<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT,
    pNext: core::ptr::null_mut(),
    maxVariableDescriptorCount: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_maxVariableDescriptorCount(mut self, val: u32) -> Self {
    self.maxVariableDescriptorCount = val;
    self
  }
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDescriptorSetLayoutSupport<
    'root,
    T: VkPNextExtends<VkDescriptorSetLayoutSupport<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDevice8BitStorageFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice8BitStorageFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevice8BitStorageFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub storageBuffer8BitAccess: VkBool32,
  pub uniformAndStorageBuffer8BitAccess: VkBool32,
  pub storagePushConstant8: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDevice8BitStorageFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDevice8BitStorageFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDevice8BitStorageFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDevice8BitStorageFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkPhysicalDevice8BitStorageFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES,
    pNext: core::ptr::null_mut(),
    storageBuffer8BitAccess: 0,
    uniformAndStorageBuffer8BitAccess: 0,
    storagePushConstant8: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_storageBuffer8BitAccess(mut self, val: VkBool32) -> Self {
    self.storageBuffer8BitAccess = val;
    self
  }
  #[inline]
  pub const fn with_uniformAndStorageBuffer8BitAccess(mut self, val: VkBool32) -> Self {
    self.uniformAndStorageBuffer8BitAccess = val;
    self
  }
  #[inline]
  pub const fn with_storagePushConstant8(mut self, val: VkBool32) -> Self {
    self.storagePushConstant8 = val;
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
/// [VkPhysicalDeviceShaderAtomicInt64Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderAtomicInt64Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderAtomicInt64Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderBufferInt64Atomics: VkBool32,
  pub shaderSharedInt64Atomics: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderAtomicInt64Features<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderAtomicInt64Features<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderAtomicInt64Features<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderAtomicInt64Features<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceShaderAtomicInt64Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES,
    pNext: core::ptr::null_mut(),
    shaderBufferInt64Atomics: 0,
    shaderSharedInt64Atomics: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderBufferInt64Atomics(mut self, val: VkBool32) -> Self {
    self.shaderBufferInt64Atomics = val;
    self
  }
  #[inline]
  pub const fn with_shaderSharedInt64Atomics(mut self, val: VkBool32) -> Self {
    self.shaderSharedInt64Atomics = val;
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
/// [VkPhysicalDeviceScalarBlockLayoutFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceScalarBlockLayoutFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub scalarBlockLayout: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceScalarBlockLayoutFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceScalarBlockLayoutFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceScalarBlockLayoutFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceScalarBlockLayoutFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceScalarBlockLayoutFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES,
    pNext: core::ptr::null_mut(),
    scalarBlockLayout: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_scalarBlockLayout(mut self, val: VkBool32) -> Self {
    self.scalarBlockLayout = val;
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
/// [VkPhysicalDeviceUniformBufferStandardLayoutFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceUniformBufferStandardLayoutFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub uniformBufferStandardLayout: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'a> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'a> {}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'child>
{
}
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'child>
{
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceUniformBufferStandardLayoutFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES,
    pNext: core::ptr::null_mut(),
    uniformBufferStandardLayout: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_uniformBufferStandardLayout(mut self, val: VkBool32) -> Self {
    self.uniformBufferStandardLayout = val;
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
