use crate::consts::VK_MAX_GLOBAL_PRIORITY_SIZE;
use crate::consts::VK_UUID_SIZE;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_4",
  feature = "VK_KHR_maintenance5",
  feature = "VK_AMDX_dense_geometry_format",
  feature = "VK_QCOM_tile_memory_heap",
  feature = "VK_EXT_memory_decompression",
  feature = "VK_EXT_device_generated_commands"
))]
use crate::enums::VkBufferUsageFlagBits2;
#[cfg(any(feature = "VK_BASE_VERSION_1_4", feature = "VK_EXT_host_image_copy"))]
use crate::enums::VkHostImageCopyFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageLayout;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkMemoryMapFlagBits;
#[cfg(any(feature = "VK_BASE_VERSION_1_4", feature = "VK_EXT_map_memory_placed"))]
use crate::enums::VkMemoryUnmapFlagBits;
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
#[cfg(any(
  feature = "VK_BASE_VERSION_1_4",
  feature = "VK_EXT_global_priority",
  feature = "VK_KHR_global_priority"
))]
use crate::enums::VkQueueGlobalPriority;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkResult;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkBindBufferMemoryInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkBindImageMemoryInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkBufferViewCreateInfo;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkDescriptorBufferBindingInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceMemory;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceQueueCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtent3D;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImage;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::types::VkImageCompressionPropertiesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkImageCopy2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImageFormatProperties2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageSubresource;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageSubresourceLayers;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageSubresourceRange;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryMapFlags;
#[cfg(feature = "VK_EXT_map_memory_placed")]
use crate::types::VkMemoryMapPlacedInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkOffset3D;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceExternalBufferInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkQueueFamilyProperties2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSubresourceLayout;
use core::ffi::c_void;
/// [VkMemoryUnmapFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type VkMemoryUnmapFlags = VkMemoryUnmapFlagBits;
/// [VkBufferUsageFlags2](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlags2.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type VkBufferUsageFlags2 = VkBufferUsageFlagBits2;
/// [VkHostImageCopyFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type VkHostImageCopyFlags = VkHostImageCopyFlagBits;
/// [VkBufferUsageFlags2CreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlags2CreateInfo.html)
///
/// **Extends:** VkBufferViewCreateInfo, VkBufferCreateInfo, VkPhysicalDeviceExternalBufferInfo, VkDescriptorBufferBindingInfoEXT.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferUsageFlags2CreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_USAGE_FLAGS_2_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub usage: VkBufferUsageFlags2,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkBufferUsageFlags2CreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkBufferUsageFlags2CreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_COMPUTE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkBufferViewCreateInfo<'root>>
  for VkBufferUsageFlags2CreateInfo<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkBufferCreateInfo<'root>>
  for VkBufferUsageFlags2CreateInfo<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceExternalBufferInfo<'root>>
  for VkBufferUsageFlags2CreateInfo<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_EXT_descriptor_buffer"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDescriptorBufferBindingInfoEXT<'root>>
  for VkBufferUsageFlags2CreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkBufferUsageFlags2CreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_BUFFER_USAGE_FLAGS_2_CREATE_INFO,
    pNext: core::ptr::null(),
    usage: VkBufferUsageFlagBits2(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_usage(mut self, val: VkBufferUsageFlags2) -> Self {
    self.usage = val;
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
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDescriptorBufferBindingInfoEXT<
    'root,
    T: VkPNextExtends<VkDescriptorBufferBindingInfoEXT<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceMaintenance5Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance5Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMaintenance5Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub maintenance5: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceMaintenance5Features<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceMaintenance5Features<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceMaintenance5Features<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceMaintenance5Features<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceMaintenance5Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES,
    pNext: core::ptr::null_mut(),
    maintenance5: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_maintenance5(mut self, val: VkBool32) -> Self {
    self.maintenance5 = val;
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
/// [VkPhysicalDeviceMaintenance5Properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance5Properties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMaintenance5Properties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub earlyFragmentMultisampleCoverageAfterSampleCounting: VkBool32,
  /// Limit Type: [Max]
  pub earlyFragmentSampleMaskTestBeforeSampleCounting: VkBool32,
  /// Limit Type: [Max]
  pub depthStencilSwizzleOneSupport: VkBool32,
  /// Limit Type: [Exact]
  pub polygonModePointSize: VkBool32,
  /// Limit Type: [Max]
  pub nonStrictSinglePixelWideLinesUseParallelogram: VkBool32,
  /// Limit Type: [Max]
  pub nonStrictWideLinesUseParallelogram: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceMaintenance5Properties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceMaintenance5Properties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceMaintenance5Properties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceMaintenance5Properties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES,
    pNext: core::ptr::null_mut(),
    earlyFragmentMultisampleCoverageAfterSampleCounting: 0,
    earlyFragmentSampleMaskTestBeforeSampleCounting: 0,
    depthStencilSwizzleOneSupport: 0,
    polygonModePointSize: 0,
    nonStrictSinglePixelWideLinesUseParallelogram: 0,
    nonStrictWideLinesUseParallelogram: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_earlyFragmentMultisampleCoverageAfterSampleCounting(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.earlyFragmentMultisampleCoverageAfterSampleCounting = val;
    self
  }
  #[inline]
  pub const fn with_earlyFragmentSampleMaskTestBeforeSampleCounting(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.earlyFragmentSampleMaskTestBeforeSampleCounting = val;
    self
  }
  #[inline]
  pub const fn with_depthStencilSwizzleOneSupport(mut self, val: VkBool32) -> Self {
    self.depthStencilSwizzleOneSupport = val;
    self
  }
  #[inline]
  pub const fn with_polygonModePointSize(mut self, val: VkBool32) -> Self {
    self.polygonModePointSize = val;
    self
  }
  #[inline]
  pub const fn with_nonStrictSinglePixelWideLinesUseParallelogram(mut self, val: VkBool32) -> Self {
    self.nonStrictSinglePixelWideLinesUseParallelogram = val;
    self
  }
  #[inline]
  pub const fn with_nonStrictWideLinesUseParallelogram(mut self, val: VkBool32) -> Self {
    self.nonStrictWideLinesUseParallelogram = val;
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
/// [VkPhysicalDeviceMaintenance6Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance6Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMaintenance6Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub maintenance6: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceMaintenance6Features<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceMaintenance6Features<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceMaintenance6Features<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceMaintenance6Features<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceMaintenance6Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES,
    pNext: core::ptr::null_mut(),
    maintenance6: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_maintenance6(mut self, val: VkBool32) -> Self {
    self.maintenance6 = val;
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
/// [VkPhysicalDeviceMaintenance6Properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance6Properties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMaintenance6Properties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub blockTexelViewCompatibleMultipleLayers: VkBool32,
  /// Limit Type: [Max]
  pub maxCombinedImageSamplerDescriptorCount: u32,
  /// Limit Type: [Max]
  pub fragmentShadingRateClampCombinerInputs: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceMaintenance6Properties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceMaintenance6Properties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceMaintenance6Properties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceMaintenance6Properties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES,
    pNext: core::ptr::null_mut(),
    blockTexelViewCompatibleMultipleLayers: 0,
    maxCombinedImageSamplerDescriptorCount: 0,
    fragmentShadingRateClampCombinerInputs: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_blockTexelViewCompatibleMultipleLayers(mut self, val: VkBool32) -> Self {
    self.blockTexelViewCompatibleMultipleLayers = val;
    self
  }
  #[inline]
  pub const fn with_maxCombinedImageSamplerDescriptorCount(mut self, val: u32) -> Self {
    self.maxCombinedImageSamplerDescriptorCount = val;
    self
  }
  #[inline]
  pub const fn with_fragmentShadingRateClampCombinerInputs(mut self, val: VkBool32) -> Self {
    self.fragmentShadingRateClampCombinerInputs = val;
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
/// [VkDeviceQueueGlobalPriorityCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueGlobalPriorityCreateInfo.html)
///
/// **Extends:** VkDeviceQueueCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceQueueGlobalPriorityCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub globalPriority: VkQueueGlobalPriority,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkDeviceQueueGlobalPriorityCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkDeviceQueueGlobalPriorityCreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceQueueCreateInfo<'root>>
  for VkDeviceQueueGlobalPriorityCreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkDeviceQueueGlobalPriorityCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO,
    pNext: core::ptr::null(),
    globalPriority: VkQueueGlobalPriority(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_globalPriority(mut self, val: VkQueueGlobalPriority) -> Self {
    self.globalPriority = val;
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
/// [VkPhysicalDeviceGlobalPriorityQueryFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGlobalPriorityQueryFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub globalPriorityQuery: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceGlobalPriorityQueryFeatures<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceGlobalPriorityQueryFeatures<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceGlobalPriorityQueryFeatures<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceGlobalPriorityQueryFeatures<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceGlobalPriorityQueryFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES,
    pNext: core::ptr::null_mut(),
    globalPriorityQuery: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_globalPriorityQuery(mut self, val: VkBool32) -> Self {
    self.globalPriorityQuery = val;
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
/// [VkQueueFamilyGlobalPriorityProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyGlobalPriorityProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkQueueFamilyProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkQueueFamilyGlobalPriorityProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub priorityCount: u32,
  /// Length: priorityCount,  Limit Type: [Bitmask]
  pub priorities: [VkQueueGlobalPriority; VK_MAX_GLOBAL_PRIORITY_SIZE as usize],
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkQueueFamilyGlobalPriorityProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkQueueFamilyGlobalPriorityProperties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkQueueFamilyProperties2<'root>>
  for VkQueueFamilyGlobalPriorityProperties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkQueueFamilyGlobalPriorityProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES,
    pNext: core::ptr::null_mut(),
    priorityCount: 0,
    priorities: [VkQueueGlobalPriority(0); VK_MAX_GLOBAL_PRIORITY_SIZE as usize],
    _marker: core::marker::PhantomData,
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
  pub const fn with_priorityCount(mut self, val: u32) -> Self {
    self.priorityCount = val;
    self
  }
  #[inline]
  pub const fn with_priorities(
    mut self,
    val: [VkQueueGlobalPriority; VK_MAX_GLOBAL_PRIORITY_SIZE as usize],
  ) -> Self {
    self.priorities = val;
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
/// [VkPhysicalDeviceIndexTypeUint8Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceIndexTypeUint8Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceIndexTypeUint8Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub indexTypeUint8: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceIndexTypeUint8Features<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceIndexTypeUint8Features<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceIndexTypeUint8Features<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceIndexTypeUint8Features<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceIndexTypeUint8Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES,
    pNext: core::ptr::null_mut(),
    indexTypeUint8: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_indexTypeUint8(mut self, val: VkBool32) -> Self {
    self.indexTypeUint8 = val;
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
/// [VkPhysicalDeviceVulkan14Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan14Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVulkan14Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_4_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub globalPriorityQuery: VkBool32,
  pub shaderSubgroupRotate: VkBool32,
  pub shaderSubgroupRotateClustered: VkBool32,
  pub shaderFloatControls2: VkBool32,
  pub shaderExpectAssume: VkBool32,
  pub rectangularLines: VkBool32,
  pub bresenhamLines: VkBool32,
  pub smoothLines: VkBool32,
  pub stippledRectangularLines: VkBool32,
  pub stippledBresenhamLines: VkBool32,
  pub stippledSmoothLines: VkBool32,
  pub vertexAttributeInstanceRateDivisor: VkBool32,
  pub vertexAttributeInstanceRateZeroDivisor: VkBool32,
  pub indexTypeUint8: VkBool32,
  pub dynamicRenderingLocalRead: VkBool32,
  pub maintenance5: VkBool32,
  pub maintenance6: VkBool32,
  pub pipelineProtectedAccess: VkBool32,
  pub pipelineRobustness: VkBool32,
  pub hostImageCopy: VkBool32,
  pub pushDescriptor: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceVulkan14Features<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceVulkan14Features<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceVulkan14Features<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceVulkan14Features<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceVulkan14Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_4_FEATURES,
    pNext: core::ptr::null_mut(),
    globalPriorityQuery: 0,
    shaderSubgroupRotate: 0,
    shaderSubgroupRotateClustered: 0,
    shaderFloatControls2: 0,
    shaderExpectAssume: 0,
    rectangularLines: 0,
    bresenhamLines: 0,
    smoothLines: 0,
    stippledRectangularLines: 0,
    stippledBresenhamLines: 0,
    stippledSmoothLines: 0,
    vertexAttributeInstanceRateDivisor: 0,
    vertexAttributeInstanceRateZeroDivisor: 0,
    indexTypeUint8: 0,
    dynamicRenderingLocalRead: 0,
    maintenance5: 0,
    maintenance6: 0,
    pipelineProtectedAccess: 0,
    pipelineRobustness: 0,
    hostImageCopy: 0,
    pushDescriptor: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_globalPriorityQuery(mut self, val: VkBool32) -> Self {
    self.globalPriorityQuery = val;
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
  #[inline]
  pub const fn with_shaderFloatControls2(mut self, val: VkBool32) -> Self {
    self.shaderFloatControls2 = val;
    self
  }
  #[inline]
  pub const fn with_shaderExpectAssume(mut self, val: VkBool32) -> Self {
    self.shaderExpectAssume = val;
    self
  }
  #[inline]
  pub const fn with_rectangularLines(mut self, val: VkBool32) -> Self {
    self.rectangularLines = val;
    self
  }
  #[inline]
  pub const fn with_bresenhamLines(mut self, val: VkBool32) -> Self {
    self.bresenhamLines = val;
    self
  }
  #[inline]
  pub const fn with_smoothLines(mut self, val: VkBool32) -> Self {
    self.smoothLines = val;
    self
  }
  #[inline]
  pub const fn with_stippledRectangularLines(mut self, val: VkBool32) -> Self {
    self.stippledRectangularLines = val;
    self
  }
  #[inline]
  pub const fn with_stippledBresenhamLines(mut self, val: VkBool32) -> Self {
    self.stippledBresenhamLines = val;
    self
  }
  #[inline]
  pub const fn with_stippledSmoothLines(mut self, val: VkBool32) -> Self {
    self.stippledSmoothLines = val;
    self
  }
  #[inline]
  pub const fn with_vertexAttributeInstanceRateDivisor(mut self, val: VkBool32) -> Self {
    self.vertexAttributeInstanceRateDivisor = val;
    self
  }
  #[inline]
  pub const fn with_vertexAttributeInstanceRateZeroDivisor(mut self, val: VkBool32) -> Self {
    self.vertexAttributeInstanceRateZeroDivisor = val;
    self
  }
  #[inline]
  pub const fn with_indexTypeUint8(mut self, val: VkBool32) -> Self {
    self.indexTypeUint8 = val;
    self
  }
  #[inline]
  pub const fn with_dynamicRenderingLocalRead(mut self, val: VkBool32) -> Self {
    self.dynamicRenderingLocalRead = val;
    self
  }
  #[inline]
  pub const fn with_maintenance5(mut self, val: VkBool32) -> Self {
    self.maintenance5 = val;
    self
  }
  #[inline]
  pub const fn with_maintenance6(mut self, val: VkBool32) -> Self {
    self.maintenance6 = val;
    self
  }
  #[inline]
  pub const fn with_pipelineProtectedAccess(mut self, val: VkBool32) -> Self {
    self.pipelineProtectedAccess = val;
    self
  }
  #[inline]
  pub const fn with_pipelineRobustness(mut self, val: VkBool32) -> Self {
    self.pipelineRobustness = val;
    self
  }
  #[inline]
  pub const fn with_hostImageCopy(mut self, val: VkBool32) -> Self {
    self.hostImageCopy = val;
    self
  }
  #[inline]
  pub const fn with_pushDescriptor(mut self, val: VkBool32) -> Self {
    self.pushDescriptor = val;
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
/// [VkPhysicalDeviceVulkan14Properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan14Properties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVulkan14Properties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bits]
  pub lineSubPixelPrecisionBits: u32,
  /// Limit Type: [Max]
  pub maxVertexAttribDivisor: u32,
  /// Limit Type: [Max]
  pub supportsNonZeroFirstInstance: VkBool32,
  /// Limit Type: [Max]
  pub maxPushDescriptors: u32,
  /// Limit Type: [Max]
  pub dynamicRenderingLocalReadDepthStencilAttachments: VkBool32,
  /// Limit Type: [Max]
  pub dynamicRenderingLocalReadMultisampledAttachments: VkBool32,
  /// Limit Type: [Max]
  pub earlyFragmentMultisampleCoverageAfterSampleCounting: VkBool32,
  /// Limit Type: [Max]
  pub earlyFragmentSampleMaskTestBeforeSampleCounting: VkBool32,
  /// Limit Type: [Max]
  pub depthStencilSwizzleOneSupport: VkBool32,
  /// Limit Type: [Exact]
  pub polygonModePointSize: VkBool32,
  /// Limit Type: [Max]
  pub nonStrictSinglePixelWideLinesUseParallelogram: VkBool32,
  /// Limit Type: [Max]
  pub nonStrictWideLinesUseParallelogram: VkBool32,
  /// Limit Type: [Max]
  pub blockTexelViewCompatibleMultipleLayers: VkBool32,
  /// Limit Type: [Max]
  pub maxCombinedImageSamplerDescriptorCount: u32,
  /// Limit Type: [Max]
  pub fragmentShadingRateClampCombinerInputs: VkBool32,
  /// Limit Type: [Exact]
  pub defaultRobustnessStorageBuffers: VkPipelineRobustnessBufferBehavior,
  /// Limit Type: [Exact]
  pub defaultRobustnessUniformBuffers: VkPipelineRobustnessBufferBehavior,
  /// Limit Type: [Exact]
  pub defaultRobustnessVertexInputs: VkPipelineRobustnessBufferBehavior,
  /// Limit Type: [Exact]
  pub defaultRobustnessImages: VkPipelineRobustnessImageBehavior,
  /// Optional: true,  Limit Type: [Noauto]
  pub copySrcLayoutCount: u32,
  /// Optional: true,  Length: copySrcLayoutCount,  Limit Type: [Noauto]
  pub pCopySrcLayouts: *mut VkImageLayout,
  /// Optional: true,  Limit Type: [Noauto]
  pub copyDstLayoutCount: u32,
  /// Optional: true,  Length: copyDstLayoutCount,  Limit Type: [Noauto]
  pub pCopyDstLayouts: *mut VkImageLayout,
  /// Optional: true,  Limit Type: [Noauto]
  pub optimalTilingLayoutUUID: [u8; VK_UUID_SIZE as usize],
  /// Limit Type: [Max]
  pub identicalMemoryTypeRequirements: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceVulkan14Properties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceVulkan14Properties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceVulkan14Properties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceVulkan14Properties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES,
    pNext: core::ptr::null_mut(),
    lineSubPixelPrecisionBits: 0,
    maxVertexAttribDivisor: 0,
    supportsNonZeroFirstInstance: 0,
    maxPushDescriptors: 0,
    dynamicRenderingLocalReadDepthStencilAttachments: 0,
    dynamicRenderingLocalReadMultisampledAttachments: 0,
    earlyFragmentMultisampleCoverageAfterSampleCounting: 0,
    earlyFragmentSampleMaskTestBeforeSampleCounting: 0,
    depthStencilSwizzleOneSupport: 0,
    polygonModePointSize: 0,
    nonStrictSinglePixelWideLinesUseParallelogram: 0,
    nonStrictWideLinesUseParallelogram: 0,
    blockTexelViewCompatibleMultipleLayers: 0,
    maxCombinedImageSamplerDescriptorCount: 0,
    fragmentShadingRateClampCombinerInputs: 0,
    defaultRobustnessStorageBuffers: VkPipelineRobustnessBufferBehavior(0),
    defaultRobustnessUniformBuffers: VkPipelineRobustnessBufferBehavior(0),
    defaultRobustnessVertexInputs: VkPipelineRobustnessBufferBehavior(0),
    defaultRobustnessImages: VkPipelineRobustnessImageBehavior(0),
    copySrcLayoutCount: 0,
    pCopySrcLayouts: core::ptr::null_mut(),
    copyDstLayoutCount: 0,
    pCopyDstLayouts: core::ptr::null_mut(),
    optimalTilingLayoutUUID: [0u8; VK_UUID_SIZE as usize],
    identicalMemoryTypeRequirements: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_lineSubPixelPrecisionBits(mut self, val: u32) -> Self {
    self.lineSubPixelPrecisionBits = val;
    self
  }
  #[inline]
  pub const fn with_maxVertexAttribDivisor(mut self, val: u32) -> Self {
    self.maxVertexAttribDivisor = val;
    self
  }
  #[inline]
  pub const fn with_supportsNonZeroFirstInstance(mut self, val: VkBool32) -> Self {
    self.supportsNonZeroFirstInstance = val;
    self
  }
  #[inline]
  pub const fn with_maxPushDescriptors(mut self, val: u32) -> Self {
    self.maxPushDescriptors = val;
    self
  }
  #[inline]
  pub const fn with_dynamicRenderingLocalReadDepthStencilAttachments(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.dynamicRenderingLocalReadDepthStencilAttachments = val;
    self
  }
  #[inline]
  pub const fn with_dynamicRenderingLocalReadMultisampledAttachments(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.dynamicRenderingLocalReadMultisampledAttachments = val;
    self
  }
  #[inline]
  pub const fn with_earlyFragmentMultisampleCoverageAfterSampleCounting(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.earlyFragmentMultisampleCoverageAfterSampleCounting = val;
    self
  }
  #[inline]
  pub const fn with_earlyFragmentSampleMaskTestBeforeSampleCounting(
    mut self,
    val: VkBool32,
  ) -> Self {
    self.earlyFragmentSampleMaskTestBeforeSampleCounting = val;
    self
  }
  #[inline]
  pub const fn with_depthStencilSwizzleOneSupport(mut self, val: VkBool32) -> Self {
    self.depthStencilSwizzleOneSupport = val;
    self
  }
  #[inline]
  pub const fn with_polygonModePointSize(mut self, val: VkBool32) -> Self {
    self.polygonModePointSize = val;
    self
  }
  #[inline]
  pub const fn with_nonStrictSinglePixelWideLinesUseParallelogram(mut self, val: VkBool32) -> Self {
    self.nonStrictSinglePixelWideLinesUseParallelogram = val;
    self
  }
  #[inline]
  pub const fn with_nonStrictWideLinesUseParallelogram(mut self, val: VkBool32) -> Self {
    self.nonStrictWideLinesUseParallelogram = val;
    self
  }
  #[inline]
  pub const fn with_blockTexelViewCompatibleMultipleLayers(mut self, val: VkBool32) -> Self {
    self.blockTexelViewCompatibleMultipleLayers = val;
    self
  }
  #[inline]
  pub const fn with_maxCombinedImageSamplerDescriptorCount(mut self, val: u32) -> Self {
    self.maxCombinedImageSamplerDescriptorCount = val;
    self
  }
  #[inline]
  pub const fn with_fragmentShadingRateClampCombinerInputs(mut self, val: VkBool32) -> Self {
    self.fragmentShadingRateClampCombinerInputs = val;
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
  #[inline]
  pub const fn with_copySrcLayoutCount(mut self, val: u32) -> Self {
    self.copySrcLayoutCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCopySrcLayouts(mut self, val: &'a mut [VkImageLayout]) -> Self {
    self.copySrcLayoutCount = val.len() as u32;
    self.pCopySrcLayouts = val.as_mut_ptr();
    self
  }
  #[inline]
  pub const fn with_copyDstLayoutCount(mut self, val: u32) -> Self {
    self.copyDstLayoutCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCopyDstLayouts(mut self, val: &'a mut [VkImageLayout]) -> Self {
    self.copyDstLayoutCount = val.len() as u32;
    self.pCopyDstLayouts = val.as_mut_ptr();
    self
  }
  #[inline]
  pub const fn with_optimalTilingLayoutUUID(mut self, val: [u8; VK_UUID_SIZE as usize]) -> Self {
    self.optimalTilingLayoutUUID = val;
    self
  }
  #[inline]
  pub const fn with_identicalMemoryTypeRequirements(mut self, val: VkBool32) -> Self {
    self.identicalMemoryTypeRequirements = val;
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
/// [VkPhysicalDeviceHostImageCopyFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceHostImageCopyFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceHostImageCopyFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub hostImageCopy: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceHostImageCopyFeatures<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceHostImageCopyFeatures<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceHostImageCopyFeatures<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceHostImageCopyFeatures<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceHostImageCopyFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES,
    pNext: core::ptr::null_mut(),
    hostImageCopy: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_hostImageCopy(mut self, val: VkBool32) -> Self {
    self.hostImageCopy = val;
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
/// [VkPhysicalDeviceHostImageCopyProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceHostImageCopyProperties.html)
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceHostImageCopyProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Optional: true,  Limit Type: [Noauto]
  pub copySrcLayoutCount: u32,
  /// Optional: true,  Length: copySrcLayoutCount,  Limit Type: [Noauto]
  pub pCopySrcLayouts: *mut VkImageLayout,
  /// Optional: true,  Limit Type: [Noauto]
  pub copyDstLayoutCount: u32,
  /// Optional: true,  Length: copyDstLayoutCount,  Limit Type: [Noauto]
  pub pCopyDstLayouts: *mut VkImageLayout,
  /// Optional: true,  Limit Type: [Noauto]
  pub optimalTilingLayoutUUID: [u8; VK_UUID_SIZE as usize],
  /// Limit Type: [Max]
  pub identicalMemoryTypeRequirements: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceHostImageCopyProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceHostImageCopyProperties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceHostImageCopyProperties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkPhysicalDeviceHostImageCopyProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES,
    pNext: core::ptr::null_mut(),
    copySrcLayoutCount: 0,
    pCopySrcLayouts: core::ptr::null_mut(),
    copyDstLayoutCount: 0,
    pCopyDstLayouts: core::ptr::null_mut(),
    optimalTilingLayoutUUID: [0u8; VK_UUID_SIZE as usize],
    identicalMemoryTypeRequirements: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_copySrcLayoutCount(mut self, val: u32) -> Self {
    self.copySrcLayoutCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCopySrcLayouts(mut self, val: &'a mut [VkImageLayout]) -> Self {
    self.copySrcLayoutCount = val.len() as u32;
    self.pCopySrcLayouts = val.as_mut_ptr();
    self
  }
  #[inline]
  pub const fn with_copyDstLayoutCount(mut self, val: u32) -> Self {
    self.copyDstLayoutCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCopyDstLayouts(mut self, val: &'a mut [VkImageLayout]) -> Self {
    self.copyDstLayoutCount = val.len() as u32;
    self.pCopyDstLayouts = val.as_mut_ptr();
    self
  }
  #[inline]
  pub const fn with_optimalTilingLayoutUUID(mut self, val: [u8; VK_UUID_SIZE as usize]) -> Self {
    self.optimalTilingLayoutUUID = val;
    self
  }
  #[inline]
  pub const fn with_identicalMemoryTypeRequirements(mut self, val: VkBool32) -> Self {
    self.identicalMemoryTypeRequirements = val;
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
/// [VkMemoryToImageCopy](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryToImageCopy.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryToImageCopy<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_TO_IMAGE_COPY
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pHostPointer: *const c_void,
  pub memoryRowLength: u32,
  pub memoryImageHeight: u32,
  pub imageSubresource: VkImageSubresourceLayers,
  pub imageOffset: VkOffset3D,
  pub imageExtent: VkExtent3D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkMemoryToImageCopy<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkMemoryToImageCopy<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkMemoryToImageCopy<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_MEMORY_TO_IMAGE_COPY,
    pNext: core::ptr::null(),
    pHostPointer: core::ptr::null(),
    memoryRowLength: 0,
    memoryImageHeight: 0,
    imageSubresource: VkImageSubresourceLayers::DEFAULT,
    imageOffset: VkOffset3D::DEFAULT,
    imageExtent: VkExtent3D::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_pHostPointer(mut self, val: *const c_void) -> Self {
    self.pHostPointer = val;
    self
  }
  #[inline]
  pub const fn with_memoryRowLength(mut self, val: u32) -> Self {
    self.memoryRowLength = val;
    self
  }
  #[inline]
  pub const fn with_memoryImageHeight(mut self, val: u32) -> Self {
    self.memoryImageHeight = val;
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
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryToImageCopy<
    'root,
    T: VkPNextExtends<VkMemoryToImageCopy<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageToMemoryCopy](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageToMemoryCopy.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageToMemoryCopy<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_TO_MEMORY_COPY
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pHostPointer: *mut c_void,
  pub memoryRowLength: u32,
  pub memoryImageHeight: u32,
  pub imageSubresource: VkImageSubresourceLayers,
  pub imageOffset: VkOffset3D,
  pub imageExtent: VkExtent3D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkImageToMemoryCopy<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkImageToMemoryCopy<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkImageToMemoryCopy<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_TO_MEMORY_COPY,
    pNext: core::ptr::null(),
    pHostPointer: core::ptr::null_mut(),
    memoryRowLength: 0,
    memoryImageHeight: 0,
    imageSubresource: VkImageSubresourceLayers::DEFAULT,
    imageOffset: VkOffset3D::DEFAULT,
    imageExtent: VkExtent3D::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_pHostPointer(mut self, val: *mut c_void) -> Self {
    self.pHostPointer = val;
    self
  }
  #[inline]
  pub const fn with_memoryRowLength(mut self, val: u32) -> Self {
    self.memoryRowLength = val;
    self
  }
  #[inline]
  pub const fn with_memoryImageHeight(mut self, val: u32) -> Self {
    self.memoryImageHeight = val;
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
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageToMemoryCopy<
    'root,
    T: VkPNextExtends<VkImageToMemoryCopy<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCopyMemoryToImageInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryToImageInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyMemoryToImageInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_COPY_MEMORY_TO_IMAGE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkHostImageCopyFlags,
  pub dstImage: VkImage,
  pub dstImageLayout: VkImageLayout,
  pub regionCount: u32,
  /// Length: regionCount
  pub pRegions: *const VkMemoryToImageCopy<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkCopyMemoryToImageInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkCopyMemoryToImageInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkCopyMemoryToImageInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_COPY_MEMORY_TO_IMAGE_INFO,
    pNext: core::ptr::null(),
    flags: VkHostImageCopyFlagBits(0),
    dstImage: VkImage::DEFAULT,
    dstImageLayout: VkImageLayout(0),
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
  pub const fn with_flags(mut self, val: VkHostImageCopyFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_dstImage(mut self, val: VkImage) -> Self {
    self.dstImage = val;
    self
  }
  #[inline]
  pub const fn with_dstImageLayout(mut self, val: VkImageLayout) -> Self {
    self.dstImageLayout = val;
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
  pub const fn with_pRegions(mut self, val: &'a [VkMemoryToImageCopy<'a>]) -> Self {
    self.regionCount = val.len() as u32;
    self.pRegions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCopyMemoryToImageInfo<
    'root,
    T: VkPNextExtends<VkCopyMemoryToImageInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCopyImageToMemoryInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageToMemoryInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyImageToMemoryInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_COPY_IMAGE_TO_MEMORY_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkHostImageCopyFlags,
  pub srcImage: VkImage,
  pub srcImageLayout: VkImageLayout,
  pub regionCount: u32,
  /// Length: regionCount
  pub pRegions: *const VkImageToMemoryCopy<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkCopyImageToMemoryInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkCopyImageToMemoryInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkCopyImageToMemoryInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_COPY_IMAGE_TO_MEMORY_INFO,
    pNext: core::ptr::null(),
    flags: VkHostImageCopyFlagBits(0),
    srcImage: VkImage::DEFAULT,
    srcImageLayout: VkImageLayout(0),
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
  pub const fn with_flags(mut self, val: VkHostImageCopyFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_srcImage(mut self, val: VkImage) -> Self {
    self.srcImage = val;
    self
  }
  #[inline]
  pub const fn with_srcImageLayout(mut self, val: VkImageLayout) -> Self {
    self.srcImageLayout = val;
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
  pub const fn with_pRegions(mut self, val: &'a [VkImageToMemoryCopy<'a>]) -> Self {
    self.regionCount = val.len() as u32;
    self.pRegions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCopyImageToMemoryInfo<
    'root,
    T: VkPNextExtends<VkCopyImageToMemoryInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkCopyImageToImageInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageToImageInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCopyImageToImageInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_COPY_IMAGE_TO_IMAGE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkHostImageCopyFlags,
  pub srcImage: VkImage,
  pub srcImageLayout: VkImageLayout,
  pub dstImage: VkImage,
  pub dstImageLayout: VkImageLayout,
  pub regionCount: u32,
  /// Length: regionCount
  pub pRegions: *const VkImageCopy2<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkCopyImageToImageInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkCopyImageToImageInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkCopyImageToImageInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_COPY_IMAGE_TO_IMAGE_INFO,
    pNext: core::ptr::null(),
    flags: VkHostImageCopyFlagBits(0),
    srcImage: VkImage::DEFAULT,
    srcImageLayout: VkImageLayout(0),
    dstImage: VkImage::DEFAULT,
    dstImageLayout: VkImageLayout(0),
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
  pub const fn with_flags(mut self, val: VkHostImageCopyFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_srcImage(mut self, val: VkImage) -> Self {
    self.srcImage = val;
    self
  }
  #[inline]
  pub const fn with_srcImageLayout(mut self, val: VkImageLayout) -> Self {
    self.srcImageLayout = val;
    self
  }
  #[inline]
  pub const fn with_dstImage(mut self, val: VkImage) -> Self {
    self.dstImage = val;
    self
  }
  #[inline]
  pub const fn with_dstImageLayout(mut self, val: VkImageLayout) -> Self {
    self.dstImageLayout = val;
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
  pub const fn with_pRegions(mut self, val: &'a [VkImageCopy2<'a>]) -> Self {
    self.regionCount = val.len() as u32;
    self.pRegions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCopyImageToImageInfo<
    'root,
    T: VkPNextExtends<VkCopyImageToImageInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkHostImageLayoutTransitionInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageLayoutTransitionInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkHostImageLayoutTransitionInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_HOST_IMAGE_LAYOUT_TRANSITION_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub image: VkImage,
  pub oldLayout: VkImageLayout,
  pub newLayout: VkImageLayout,
  pub subresourceRange: VkImageSubresourceRange,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkHostImageLayoutTransitionInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkHostImageLayoutTransitionInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkHostImageLayoutTransitionInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_HOST_IMAGE_LAYOUT_TRANSITION_INFO,
    pNext: core::ptr::null(),
    image: VkImage::DEFAULT,
    oldLayout: VkImageLayout(0),
    newLayout: VkImageLayout(0),
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
  pub const fn with_image(mut self, val: VkImage) -> Self {
    self.image = val;
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
  pub const fn with_subresourceRange(mut self, val: VkImageSubresourceRange) -> Self {
    self.subresourceRange = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkHostImageLayoutTransitionInfo<
    'root,
    T: VkPNextExtends<VkHostImageLayoutTransitionInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSubresourceHostMemcpySize](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubresourceHostMemcpySize.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkSubresourceLayout2.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubresourceHostMemcpySize<'a> {
  /// Values: VK_STRUCTURE_TYPE_SUBRESOURCE_HOST_MEMCPY_SIZE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub size: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkSubresourceHostMemcpySize<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkSubresourceHostMemcpySize<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_4"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubresourceLayout2<'root>>
  for VkSubresourceHostMemcpySize<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkSubresourceHostMemcpySize<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_SUBRESOURCE_HOST_MEMCPY_SIZE,
    pNext: core::ptr::null_mut(),
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
  pub const fn with_pNext(mut self, val: *mut c_void) -> Self {
    self.pNext = val;
    self
  }
  #[inline]
  pub const fn with_size(mut self, val: VkDeviceSize) -> Self {
    self.size = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubresourceLayout2<
    'root,
    T: VkPNextExtends<VkSubresourceLayout2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkHostImageCopyDevicePerformanceQuery](https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyDevicePerformanceQuery.html)
///
/// *Note: This is a **returned only** struct.*
///
/// **Extends:** VkImageFormatProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkHostImageCopyDevicePerformanceQuery<'a> {
  /// Values: VK_STRUCTURE_TYPE_HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub optimalDeviceAccess: VkBool32,
  pub identicalMemoryLayout: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkHostImageCopyDevicePerformanceQuery<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkHostImageCopyDevicePerformanceQuery<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageFormatProperties2<'root>>
  for VkHostImageCopyDevicePerformanceQuery<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkHostImageCopyDevicePerformanceQuery<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY,
    pNext: core::ptr::null_mut(),
    optimalDeviceAccess: 0,
    identicalMemoryLayout: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_optimalDeviceAccess(mut self, val: VkBool32) -> Self {
    self.optimalDeviceAccess = val;
    self
  }
  #[inline]
  pub const fn with_identicalMemoryLayout(mut self, val: VkBool32) -> Self {
    self.identicalMemoryLayout = val;
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
/// [VkImageSubresource2](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSubresource2.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageSubresource2<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub imageSubresource: VkImageSubresource,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkImageSubresource2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkImageSubresource2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkImageSubresource2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2,
    pNext: core::ptr::null_mut(),
    imageSubresource: VkImageSubresource::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_imageSubresource(mut self, val: VkImageSubresource) -> Self {
    self.imageSubresource = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageSubresource2<
    'root,
    T: VkPNextExtends<VkImageSubresource2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkSubresourceLayout2](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubresourceLayout2.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubresourceLayout2<'a> {
  /// Values: VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub subresourceLayout: VkSubresourceLayout,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkSubresourceLayout2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkSubresourceLayout2<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkSubresourceLayout2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2,
    pNext: core::ptr::null_mut(),
    subresourceLayout: VkSubresourceLayout::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_subresourceLayout(mut self, val: VkSubresourceLayout) -> Self {
    self.subresourceLayout = val;
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
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSubresourceHostMemcpySize<'child>(
    mut self,
    val: &'a mut VkSubresourceHostMemcpySize<'child>,
  ) -> Self {
    self.pNext = (val as *mut VkSubresourceHostMemcpySize<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubresourceLayout2<
    'root,
    T: VkPNextExtends<VkSubresourceLayout2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkDeviceImageSubresourceInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceImageSubresourceInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceImageSubresourceInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_IMAGE_SUBRESOURCE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pCreateInfo: *const VkImageCreateInfo<'a>,
  pub pSubresource: *const VkImageSubresource2<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkDeviceImageSubresourceInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkDeviceImageSubresourceInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkDeviceImageSubresourceInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_IMAGE_SUBRESOURCE_INFO,
    pNext: core::ptr::null(),
    pCreateInfo: core::ptr::null(),
    pSubresource: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_pCreateInfo(mut self, val: &'a VkImageCreateInfo<'a>) -> Self {
    self.pCreateInfo = val as *const VkImageCreateInfo<'a>;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSubresource(mut self, val: &'a VkImageSubresource2<'a>) -> Self {
    self.pSubresource = val as *const VkImageSubresource2<'a>;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDeviceImageSubresourceInfo<
    'root,
    T: VkPNextExtends<VkDeviceImageSubresourceInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkMemoryMapInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryMapInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_MAP_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkMemoryMapFlags,
  pub memory: VkDeviceMemory,
  pub offset: VkDeviceSize,
  pub size: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkMemoryMapInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkMemoryMapInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkMemoryMapInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_MEMORY_MAP_INFO,
    pNext: core::ptr::null(),
    flags: VkMemoryMapFlagBits(0),
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
  pub const fn with_flags(mut self, val: VkMemoryMapFlags) -> Self {
    self.flags = val;
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
  #[cfg(feature = "VK_EXT_map_memory_placed")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryMapPlacedInfoEXT<'child>(
    mut self,
    val: &'a VkMemoryMapPlacedInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMemoryMapPlacedInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryMapInfo<
    'root,
    T: VkPNextExtends<VkMemoryMapInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkMemoryUnmapInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryUnmapInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkMemoryUnmapFlags,
  pub memory: VkDeviceMemory,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkMemoryUnmapInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkMemoryUnmapInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkMemoryUnmapInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO,
    pNext: core::ptr::null(),
    flags: VkMemoryUnmapFlagBits(0),
    memory: VkDeviceMemory::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_flags(mut self, val: VkMemoryUnmapFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_memory(mut self, val: VkDeviceMemory) -> Self {
    self.memory = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMemoryUnmapInfo<
    'root,
    T: VkPNextExtends<VkMemoryUnmapInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBindMemoryStatus](https://docs.vulkan.org/refpages/latest/refpages/source/VkBindMemoryStatus.html)
///
/// **Extends:** VkBindBufferMemoryInfo, VkBindImageMemoryInfo.
#[cfg(feature = "VK_BASE_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBindMemoryStatus<'a> {
  /// Values: VK_STRUCTURE_TYPE_BIND_MEMORY_STATUS
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub pResult: *mut VkResult,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Send for VkBindMemoryStatus<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
unsafe impl<'a> Sync for VkBindMemoryStatus<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkBindBufferMemoryInfo<'root>>
  for VkBindMemoryStatus<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkBindImageMemoryInfo<'root>>
  for VkBindMemoryStatus<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_4")]
impl<'a> VkBindMemoryStatus<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_BIND_MEMORY_STATUS,
    pNext: core::ptr::null(),
    pResult: core::ptr::null_mut(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_pResult(mut self, val: &'a mut VkResult) -> Self {
    self.pResult = val as *mut VkResult;
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
