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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::enums::VkVideoEncodeRgbChromaOffsetFlagBitsVALVE;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::enums::VkVideoEncodeRgbModelConversionFlagBitsVALVE;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::enums::VkVideoEncodeRgbRangeCompressionFlagBitsVALVE;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorPoolCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSetLayout;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSetLayoutCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoSessionCreateInfoKHR;
use core::ffi::c_void;
/// [VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub descriptorSetHostMapping: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
unsafe impl<'a> Send for VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'a> {}
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
unsafe impl<'a> Sync for VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'a> {}
#[cfg(all(
  feature = "VK_VALVE_descriptor_set_host_mapping",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'child>
{
}
#[cfg(all(
  feature = "VK_VALVE_descriptor_set_host_mapping",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'child>
{
}
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
impl<'a> VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE,
    pNext: core::ptr::null_mut(),
    descriptorSetHostMapping: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_descriptorSetHostMapping(mut self, val: VkBool32) -> Self {
    self.descriptorSetHostMapping = val;
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
/// [VkDescriptorSetBindingReferenceVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetBindingReferenceVALVE.html)
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorSetBindingReferenceVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub descriptorSetLayout: VkDescriptorSetLayout,
  #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
  pub descriptorSetLayout: *mut c_void,
  pub binding: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
unsafe impl<'a> Send for VkDescriptorSetBindingReferenceVALVE<'a> {}
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
unsafe impl<'a> Sync for VkDescriptorSetBindingReferenceVALVE<'a> {}
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
impl<'a> VkDescriptorSetBindingReferenceVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DESCRIPTOR_SET_BINDING_REFERENCE_VALVE,
    pNext: core::ptr::null(),
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    descriptorSetLayout: VkDescriptorSetLayout::DEFAULT,
    #[cfg(not(feature = "VK_COMPUTE_VERSION_1_0"))]
    descriptorSetLayout: core::ptr::null_mut(),
    binding: 0,
    _marker: core::marker::PhantomData,
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
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub const fn with_descriptorSetLayout(mut self, val: VkDescriptorSetLayout) -> Self {
    self.descriptorSetLayout = val;
    self
  }
  #[inline]
  pub const fn with_binding(mut self, val: u32) -> Self {
    self.binding = val;
    self
  }
  #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDescriptorSetBindingReferenceVALVE<
    'root,
    T: VkPNextExtends<VkDescriptorSetBindingReferenceVALVE<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDescriptorSetLayoutHostMappingInfoVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutHostMappingInfoVALVE.html)
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDescriptorSetLayoutHostMappingInfoVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub descriptorOffset: usize,
  pub descriptorSize: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
unsafe impl<'a> Send for VkDescriptorSetLayoutHostMappingInfoVALVE<'a> {}
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
unsafe impl<'a> Sync for VkDescriptorSetLayoutHostMappingInfoVALVE<'a> {}
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
impl<'a> VkDescriptorSetLayoutHostMappingInfoVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE,
    pNext: core::ptr::null_mut(),
    descriptorOffset: 0,
    descriptorSize: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_descriptorOffset(mut self, val: usize) -> Self {
    self.descriptorOffset = val;
    self
  }
  #[inline]
  pub const fn with_descriptorSize(mut self, val: u32) -> Self {
    self.descriptorSize = val;
    self
  }
  #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDescriptorSetLayoutHostMappingInfoVALVE<
    'root,
    T: VkPNextExtends<VkDescriptorSetLayoutHostMappingInfoVALVE<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxFragmentDensityMapLayers: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
unsafe impl<'a> Send for VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'a> {}
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
unsafe impl<'a> Sync for VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'a> {}
#[cfg(all(
  feature = "VK_VALVE_fragment_density_map_layered",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'child>
{
}
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
impl<'a> VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE,
    pNext: core::ptr::null_mut(),
    maxFragmentDensityMapLayers: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_maxFragmentDensityMapLayers(mut self, val: u32) -> Self {
    self.maxFragmentDensityMapLayers = val;
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
/// [VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub fragmentDensityMapLayered: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
unsafe impl<'a> Send for VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a> {}
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
unsafe impl<'a> Sync for VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a> {}
#[cfg(all(
  feature = "VK_VALVE_fragment_density_map_layered",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'child>
{
}
#[cfg(all(
  feature = "VK_VALVE_fragment_density_map_layered",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'child>
{
}
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
impl<'a> VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE,
    pNext: core::ptr::null_mut(),
    fragmentDensityMapLayered: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_fragmentDensityMapLayered(mut self, val: VkBool32) -> Self {
    self.fragmentDensityMapLayered = val;
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
/// [VkPipelineFragmentDensityMapLayeredCreateInfoVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineFragmentDensityMapLayeredCreateInfoVALVE.html)
///
/// **Extends:** VkGraphicsPipelineCreateInfo.
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineFragmentDensityMapLayeredCreateInfoVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub maxFragmentDensityMapLayers: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
unsafe impl<'a> Send for VkPipelineFragmentDensityMapLayeredCreateInfoVALVE<'a> {}
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
unsafe impl<'a> Sync for VkPipelineFragmentDensityMapLayeredCreateInfoVALVE<'a> {}
#[cfg(all(
  feature = "VK_VALVE_fragment_density_map_layered",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkPipelineFragmentDensityMapLayeredCreateInfoVALVE<'child>
{
}
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
impl<'a> VkPipelineFragmentDensityMapLayeredCreateInfoVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE,
    pNext: core::ptr::null(),
    maxFragmentDensityMapLayers: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_maxFragmentDensityMapLayers(mut self, val: u32) -> Self {
    self.maxFragmentDensityMapLayers = val;
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
/// [VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub mutableDescriptorType: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
unsafe impl<'a> Send for VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'a> {}
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
unsafe impl<'a> Sync for VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'a> {}
#[cfg(all(
  feature = "VK_VALVE_mutable_descriptor_type",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'child>
{
}
#[cfg(all(
  feature = "VK_VALVE_mutable_descriptor_type",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'child>
{
}
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
impl<'a> VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE,
    pNext: core::ptr::null_mut(),
    mutableDescriptorType: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_mutableDescriptorType(mut self, val: VkBool32) -> Self {
    self.mutableDescriptorType = val;
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
/// [VkMutableDescriptorTypeListVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkMutableDescriptorTypeListVALVE.html)
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMutableDescriptorTypeListVALVE<'a> {
  /// Optional: true
  pub descriptorTypeCount: u32,
  /// Length: descriptorTypeCount
  pub pDescriptorTypes: *const VkDescriptorType,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
unsafe impl<'a> Send for VkMutableDescriptorTypeListVALVE<'a> {}
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
unsafe impl<'a> Sync for VkMutableDescriptorTypeListVALVE<'a> {}
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
impl<'a> VkMutableDescriptorTypeListVALVE<'a> {
  pub const DEFAULT: Self = Self {
    descriptorTypeCount: 0,
    pDescriptorTypes: core::ptr::null(),
    _marker: core::marker::PhantomData,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_descriptorTypeCount(mut self, val: u32) -> Self {
    self.descriptorTypeCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDescriptorTypes(mut self, val: &'a [VkDescriptorType]) -> Self {
    self.descriptorTypeCount = val.len() as u32;
    self.pDescriptorTypes = val.as_ptr();
    self
  }
}
/// [VkMutableDescriptorTypeCreateInfoVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkMutableDescriptorTypeCreateInfoVALVE.html)
///
/// **Extends:** VkDescriptorSetLayoutCreateInfo, VkDescriptorPoolCreateInfo.
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMutableDescriptorTypeCreateInfoVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub mutableDescriptorTypeListCount: u32,
  /// Length: mutableDescriptorTypeListCount
  pub pMutableDescriptorTypeLists: *const VkMutableDescriptorTypeListVALVE<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
unsafe impl<'a> Send for VkMutableDescriptorTypeCreateInfoVALVE<'a> {}
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
unsafe impl<'a> Sync for VkMutableDescriptorTypeCreateInfoVALVE<'a> {}
#[cfg(all(
  feature = "VK_VALVE_mutable_descriptor_type",
  feature = "VK_COMPUTE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDescriptorSetLayoutCreateInfo<'root>>
  for VkMutableDescriptorTypeCreateInfoVALVE<'child>
{
}
#[cfg(all(
  feature = "VK_VALVE_mutable_descriptor_type",
  feature = "VK_COMPUTE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDescriptorPoolCreateInfo<'root>>
  for VkMutableDescriptorTypeCreateInfoVALVE<'child>
{
}
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
impl<'a> VkMutableDescriptorTypeCreateInfoVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE,
    pNext: core::ptr::null(),
    mutableDescriptorTypeListCount: 0,
    pMutableDescriptorTypeLists: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_mutableDescriptorTypeListCount(mut self, val: u32) -> Self {
    self.mutableDescriptorTypeListCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pMutableDescriptorTypeLists(
    mut self,
    val: &'a [VkMutableDescriptorTypeListVALVE<'a>],
  ) -> Self {
    self.mutableDescriptorTypeListCount = val.len() as u32;
    self.pMutableDescriptorTypeLists = val.as_ptr();
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
/// [VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_VALVE_shader_mixed_float_dot_product")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderMixedFloatDotProductFloat16AccFloat32: VkBool32,
  pub shaderMixedFloatDotProductFloat16AccFloat16: VkBool32,
  pub shaderMixedFloatDotProductBFloat16Acc: VkBool32,
  pub shaderMixedFloatDotProductFloat8AccFloat32: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_shader_mixed_float_dot_product")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'a> {}
#[cfg(feature = "VK_VALVE_shader_mixed_float_dot_product")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'a> {}
#[cfg(all(
  feature = "VK_VALVE_shader_mixed_float_dot_product",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'child>
{
}
#[cfg(all(
  feature = "VK_VALVE_shader_mixed_float_dot_product",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'child>
{
}
#[cfg(feature = "VK_VALVE_shader_mixed_float_dot_product")]
impl<'a> VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE,
    pNext: core::ptr::null_mut(),
    shaderMixedFloatDotProductFloat16AccFloat32: 0,
    shaderMixedFloatDotProductFloat16AccFloat16: 0,
    shaderMixedFloatDotProductBFloat16Acc: 0,
    shaderMixedFloatDotProductFloat8AccFloat32: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_shaderMixedFloatDotProductFloat16AccFloat32(mut self, val: VkBool32) -> Self {
    self.shaderMixedFloatDotProductFloat16AccFloat32 = val;
    self
  }
  #[inline]
  pub const fn with_shaderMixedFloatDotProductFloat16AccFloat16(mut self, val: VkBool32) -> Self {
    self.shaderMixedFloatDotProductFloat16AccFloat16 = val;
    self
  }
  #[inline]
  pub const fn with_shaderMixedFloatDotProductBFloat16Acc(mut self, val: VkBool32) -> Self {
    self.shaderMixedFloatDotProductBFloat16Acc = val;
    self
  }
  #[inline]
  pub const fn with_shaderMixedFloatDotProductFloat8AccFloat32(mut self, val: VkBool32) -> Self {
    self.shaderMixedFloatDotProductFloat8AccFloat32 = val;
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
/// [VkVideoEncodeRgbModelConversionFlagsVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbModelConversionFlagsVALVE.html)
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
pub type VkVideoEncodeRgbModelConversionFlagsVALVE = VkVideoEncodeRgbModelConversionFlagBitsVALVE;
/// [VkVideoEncodeRgbRangeCompressionFlagsVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbRangeCompressionFlagsVALVE.html)
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
pub type VkVideoEncodeRgbRangeCompressionFlagsVALVE = VkVideoEncodeRgbRangeCompressionFlagBitsVALVE;
/// [VkVideoEncodeRgbChromaOffsetFlagsVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbChromaOffsetFlagsVALVE.html)
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
pub type VkVideoEncodeRgbChromaOffsetFlagsVALVE = VkVideoEncodeRgbChromaOffsetFlagBitsVALVE;
/// [VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub videoEncodeRgbConversion: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
unsafe impl<'a> Send for VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'a> {}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
unsafe impl<'a> Sync for VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'a> {}
#[cfg(all(
  feature = "VK_VALVE_video_encode_rgb_conversion",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'child>
{
}
#[cfg(all(
  feature = "VK_VALVE_video_encode_rgb_conversion",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'child>
{
}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
impl<'a> VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE,
    pNext: core::ptr::null_mut(),
    videoEncodeRgbConversion: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_videoEncodeRgbConversion(mut self, val: VkBool32) -> Self {
    self.videoEncodeRgbConversion = val;
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
/// [VkVideoEncodeRgbConversionCapabilitiesVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbConversionCapabilitiesVALVE.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkVideoCapabilitiesKHR.
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkVideoEncodeRgbConversionCapabilitiesVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub rgbModels: VkVideoEncodeRgbModelConversionFlagsVALVE,
  /// Limit Type: [Bitmask]
  pub rgbRanges: VkVideoEncodeRgbRangeCompressionFlagsVALVE,
  /// Limit Type: [Bitmask]
  pub xChromaOffsets: VkVideoEncodeRgbChromaOffsetFlagsVALVE,
  /// Limit Type: [Bitmask]
  pub yChromaOffsets: VkVideoEncodeRgbChromaOffsetFlagsVALVE,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
unsafe impl<'a> Send for VkVideoEncodeRgbConversionCapabilitiesVALVE<'a> {}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
unsafe impl<'a> Sync for VkVideoEncodeRgbConversionCapabilitiesVALVE<'a> {}
#[cfg(all(
  feature = "VK_VALVE_video_encode_rgb_conversion",
  feature = "VK_KHR_video_queue"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkVideoCapabilitiesKHR<'root>>
  for VkVideoEncodeRgbConversionCapabilitiesVALVE<'child>
{
}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
impl<'a> VkVideoEncodeRgbConversionCapabilitiesVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE,
    pNext: core::ptr::null_mut(),
    rgbModels: VkVideoEncodeRgbModelConversionFlagBitsVALVE(0),
    rgbRanges: VkVideoEncodeRgbRangeCompressionFlagBitsVALVE(0),
    xChromaOffsets: VkVideoEncodeRgbChromaOffsetFlagBitsVALVE(0),
    yChromaOffsets: VkVideoEncodeRgbChromaOffsetFlagBitsVALVE(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_rgbModels(mut self, val: VkVideoEncodeRgbModelConversionFlagsVALVE) -> Self {
    self.rgbModels = val;
    self
  }
  #[inline]
  pub const fn with_rgbRanges(mut self, val: VkVideoEncodeRgbRangeCompressionFlagsVALVE) -> Self {
    self.rgbRanges = val;
    self
  }
  #[inline]
  pub const fn with_xChromaOffsets(mut self, val: VkVideoEncodeRgbChromaOffsetFlagsVALVE) -> Self {
    self.xChromaOffsets = val;
    self
  }
  #[inline]
  pub const fn with_yChromaOffsets(mut self, val: VkVideoEncodeRgbChromaOffsetFlagsVALVE) -> Self {
    self.yChromaOffsets = val;
    self
  }
  #[cfg(feature = "VK_KHR_video_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkVideoCapabilitiesKHR<
    'root,
    T: VkPNextExtends<VkVideoCapabilitiesKHR<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkVideoEncodeProfileRgbConversionInfoVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeProfileRgbConversionInfoVALVE.html)
///
/// **Extends:** VkVideoProfileInfoKHR.
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkVideoEncodeProfileRgbConversionInfoVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub performEncodeRgbConversion: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
unsafe impl<'a> Send for VkVideoEncodeProfileRgbConversionInfoVALVE<'a> {}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
unsafe impl<'a> Sync for VkVideoEncodeProfileRgbConversionInfoVALVE<'a> {}
#[cfg(all(
  feature = "VK_VALVE_video_encode_rgb_conversion",
  feature = "VK_KHR_video_queue"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkVideoProfileInfoKHR<'root>>
  for VkVideoEncodeProfileRgbConversionInfoVALVE<'child>
{
}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
impl<'a> VkVideoEncodeProfileRgbConversionInfoVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE,
    pNext: core::ptr::null(),
    performEncodeRgbConversion: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_performEncodeRgbConversion(mut self, val: VkBool32) -> Self {
    self.performEncodeRgbConversion = val;
    self
  }
  #[cfg(feature = "VK_KHR_video_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkVideoProfileInfoKHR<
    'root,
    T: VkPNextExtends<VkVideoProfileInfoKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkVideoEncodeSessionRgbConversionCreateInfoVALVE](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeSessionRgbConversionCreateInfoVALVE.html)
///
/// **Extends:** VkVideoSessionCreateInfoKHR.
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkVideoEncodeSessionRgbConversionCreateInfoVALVE<'a> {
  /// Values: VK_STRUCTURE_TYPE_VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub rgbModel: VkVideoEncodeRgbModelConversionFlagBitsVALVE,
  pub rgbRange: VkVideoEncodeRgbRangeCompressionFlagBitsVALVE,
  pub xChromaOffset: VkVideoEncodeRgbChromaOffsetFlagBitsVALVE,
  pub yChromaOffset: VkVideoEncodeRgbChromaOffsetFlagBitsVALVE,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
unsafe impl<'a> Send for VkVideoEncodeSessionRgbConversionCreateInfoVALVE<'a> {}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
unsafe impl<'a> Sync for VkVideoEncodeSessionRgbConversionCreateInfoVALVE<'a> {}
#[cfg(all(
  feature = "VK_VALVE_video_encode_rgb_conversion",
  feature = "VK_KHR_video_queue"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkVideoSessionCreateInfoKHR<'root>>
  for VkVideoEncodeSessionRgbConversionCreateInfoVALVE<'child>
{
}
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
impl<'a> VkVideoEncodeSessionRgbConversionCreateInfoVALVE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE,
    pNext: core::ptr::null(),
    rgbModel: VkVideoEncodeRgbModelConversionFlagBitsVALVE(0),
    rgbRange: VkVideoEncodeRgbRangeCompressionFlagBitsVALVE(0),
    xChromaOffset: VkVideoEncodeRgbChromaOffsetFlagBitsVALVE(0),
    yChromaOffset: VkVideoEncodeRgbChromaOffsetFlagBitsVALVE(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_rgbModel(mut self, val: VkVideoEncodeRgbModelConversionFlagBitsVALVE) -> Self {
    self.rgbModel = val;
    self
  }
  #[inline]
  pub const fn with_rgbRange(mut self, val: VkVideoEncodeRgbRangeCompressionFlagBitsVALVE) -> Self {
    self.rgbRange = val;
    self
  }
  #[inline]
  pub const fn with_xChromaOffset(
    mut self,
    val: VkVideoEncodeRgbChromaOffsetFlagBitsVALVE,
  ) -> Self {
    self.xChromaOffset = val;
    self
  }
  #[inline]
  pub const fn with_yChromaOffset(
    mut self,
    val: VkVideoEncodeRgbChromaOffsetFlagBitsVALVE,
  ) -> Self {
    self.yChromaOffset = val;
    self
  }
  #[cfg(feature = "VK_KHR_video_queue")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkVideoSessionCreateInfoKHR<
    'root,
    T: VkPNextExtends<VkVideoSessionCreateInfoKHR<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
