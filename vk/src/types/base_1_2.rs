use crate::consts::VK_LUID_SIZE;
use crate::consts::VK_MAX_DRIVER_INFO_SIZE;
use crate::consts::VK_MAX_DRIVER_NAME_SIZE;
use crate::consts::VK_UUID_SIZE;
#[cfg(any(feature = "VK_BASE_VERSION_1_2", feature = "VK_KHR_driver_properties"))]
use crate::enums::VkDriverId;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(any(feature = "VK_BASE_VERSION_1_1", feature = "VK_KHR_maintenance2"))]
use crate::enums::VkPointClippingBehavior;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_2",
  feature = "VK_KHR_depth_stencil_resolve",
  all(
    feature = "VK_ANDROID_external_format_resolve",
    feature = "VK_KHR_dynamic_rendering"
  ),
  all(
    feature = "VK_EXT_custom_resolve",
    feature = "VK_KHR_dynamic_rendering"
  )
))]
use crate::enums::VkResolveModeFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSampleCountFlagBits;
#[cfg(any(feature = "VK_BASE_VERSION_1_2", feature = "VK_KHR_timeline_semaphore"))]
use crate::enums::VkSemaphoreType;
#[cfg(any(feature = "VK_BASE_VERSION_1_2", feature = "VK_KHR_timeline_semaphore"))]
use crate::enums::VkSemaphoreWaitFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_2",
  feature = "VK_KHR_shader_float_controls"
))]
use crate::enums::VkShaderFloatControlsIndependence;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkShaderStageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_shader_subgroup_rotate",
  feature = "VK_EXT_shader_subgroup_partitioned"
))]
use crate::enums::VkSubgroupFeatureFlagBits;
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
use crate::types::VkBindSparseInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBuffer;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceMemory;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryAllocateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceExternalSemaphoreInfo;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceImageFormatInfo2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSampleCountFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSemaphore;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSemaphoreCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkShaderStageFlags;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkSubgroupFeatureFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSubmitInfo;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::types::VkSwapchainCreateInfoKHR;
use core::ffi::{c_char, c_void};
/// [VkSemaphoreWaitFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_2")]
pub type VkSemaphoreWaitFlags = VkSemaphoreWaitFlagBits;
/// [VkResolveModeFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlags.html)
#[cfg(feature = "VK_BASE_VERSION_1_2")]
pub type VkResolveModeFlags = VkResolveModeFlagBits;
/// [VkConformanceVersion](https://docs.vulkan.org/refpages/latest/refpages/source/VkConformanceVersion.html)
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkConformanceVersion {
  pub major: u8,
  pub minor: u8,
  pub subminor: u8,
  pub patch: u8,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl Send for VkConformanceVersion {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl Sync for VkConformanceVersion {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl VkConformanceVersion {
  pub const DEFAULT: Self = Self {
    major: 0,
    minor: 0,
    subminor: 0,
    patch: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_major(mut self, val: u8) -> Self {
    self.major = val;
    self
  }
  #[inline]
  pub const fn with_minor(mut self, val: u8) -> Self {
    self.minor = val;
    self
  }
  #[inline]
  pub const fn with_subminor(mut self, val: u8) -> Self {
    self.subminor = val;
    self
  }
  #[inline]
  pub const fn with_patch(mut self, val: u8) -> Self {
    self.patch = val;
    self
  }
}
/// [VkPhysicalDeviceDriverProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDriverProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDriverProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Exact]
  pub driverID: VkDriverId,
  /// Length: null-terminated,  Limit Type: [Exact]
  pub driverName: [c_char; VK_MAX_DRIVER_NAME_SIZE as usize],
  /// Length: null-terminated,  Limit Type: [Exact]
  pub driverInfo: [c_char; VK_MAX_DRIVER_INFO_SIZE as usize],
  /// Limit Type: [Exact]
  pub conformanceVersion: VkConformanceVersion,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceDriverProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceDriverProperties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceDriverProperties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceDriverProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DRIVER_PROPERTIES,
    pNext: core::ptr::null_mut(),
    driverID: VkDriverId(0),
    driverName: [0 as c_char; VK_MAX_DRIVER_NAME_SIZE as usize],
    driverInfo: [0 as c_char; VK_MAX_DRIVER_INFO_SIZE as usize],
    conformanceVersion: VkConformanceVersion::DEFAULT,
    _marker: core::marker::PhantomData,
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
  pub const fn with_driverID(mut self, val: VkDriverId) -> Self {
    self.driverID = val;
    self
  }
  #[inline]
  pub const fn with_driverName(mut self, val: [c_char; VK_MAX_DRIVER_NAME_SIZE as usize]) -> Self {
    self.driverName = val;
    self
  }
  #[inline]
  pub const fn with_driverInfo(mut self, val: [c_char; VK_MAX_DRIVER_INFO_SIZE as usize]) -> Self {
    self.driverInfo = val;
    self
  }
  #[inline]
  pub const fn with_conformanceVersion(mut self, val: VkConformanceVersion) -> Self {
    self.conformanceVersion = val;
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
/// [VkImageFormatListCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatListCreateInfo.html)
///
/// **Extends:** VkImageCreateInfo, VkSwapchainCreateInfoKHR, VkPhysicalDeviceImageFormatInfo2.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageFormatListCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub viewFormatCount: u32,
  /// Length: viewFormatCount
  pub pViewFormats: *const VkFormat,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkImageFormatListCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkImageFormatListCreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkImageFormatListCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_KHR_swapchain"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSwapchainCreateInfoKHR<'root>>
  for VkImageFormatListCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceImageFormatInfo2<'root>>
  for VkImageFormatListCreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkImageFormatListCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_FORMAT_LIST_CREATE_INFO,
    pNext: core::ptr::null(),
    viewFormatCount: 0,
    pViewFormats: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_viewFormatCount(mut self, val: u32) -> Self {
    self.viewFormatCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pViewFormats(mut self, val: &'a [VkFormat]) -> Self {
    self.viewFormatCount = val.len() as u32;
    self.pViewFormats = val.as_ptr();
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
/// [VkPhysicalDeviceHostQueryResetFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceHostQueryResetFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceHostQueryResetFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub hostQueryReset: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceHostQueryResetFeatures<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceHostQueryResetFeatures<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceHostQueryResetFeatures<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceHostQueryResetFeatures<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceHostQueryResetFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES,
    pNext: core::ptr::null_mut(),
    hostQueryReset: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_hostQueryReset(mut self, val: VkBool32) -> Self {
    self.hostQueryReset = val;
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
/// [VkPhysicalDeviceTimelineSemaphoreFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTimelineSemaphoreFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  #[cfg_attr(
    feature = "VKSC_VERSION_1_0",
    deprecated(
      note = "`timelineSemaphore` is removed by `VKSC_VERSION_1_0`; the field remains present only for ABI compatibility"
    )
  )]
  pub timelineSemaphore: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceTimelineSemaphoreFeatures<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceTimelineSemaphoreFeatures<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceTimelineSemaphoreFeatures<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceTimelineSemaphoreFeatures<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceTimelineSemaphoreFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES,
    pNext: core::ptr::null_mut(),
    timelineSemaphore: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_timelineSemaphore(mut self, val: VkBool32) -> Self {
    self.timelineSemaphore = val;
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
/// [VkPhysicalDeviceTimelineSemaphoreProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTimelineSemaphoreProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceTimelineSemaphoreProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxTimelineSemaphoreValueDifference: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceTimelineSemaphoreProperties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceTimelineSemaphoreProperties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceTimelineSemaphoreProperties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceTimelineSemaphoreProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES,
    pNext: core::ptr::null_mut(),
    maxTimelineSemaphoreValueDifference: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_maxTimelineSemaphoreValueDifference(mut self, val: u64) -> Self {
    self.maxTimelineSemaphoreValueDifference = val;
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
/// [VkSemaphoreTypeCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreTypeCreateInfo.html)
///
/// **Extends:** VkSemaphoreCreateInfo, VkPhysicalDeviceExternalSemaphoreInfo.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSemaphoreTypeCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub semaphoreType: VkSemaphoreType,
  pub initialValue: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkSemaphoreTypeCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkSemaphoreTypeCreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSemaphoreCreateInfo<'root>>
  for VkSemaphoreTypeCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceExternalSemaphoreInfo<'root>>
  for VkSemaphoreTypeCreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkSemaphoreTypeCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SEMAPHORE_TYPE_CREATE_INFO,
    pNext: core::ptr::null(),
    semaphoreType: VkSemaphoreType(0),
    initialValue: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_semaphoreType(mut self, val: VkSemaphoreType) -> Self {
    self.semaphoreType = val;
    self
  }
  #[inline]
  pub const fn with_initialValue(mut self, val: u64) -> Self {
    self.initialValue = val;
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
/// [VkTimelineSemaphoreSubmitInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkTimelineSemaphoreSubmitInfo.html)
///
/// **Extends:** VkSubmitInfo, VkBindSparseInfo.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkTimelineSemaphoreSubmitInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub waitSemaphoreValueCount: u32,
  /// Optional: true,  Length: waitSemaphoreValueCount
  pub pWaitSemaphoreValues: *const u64,
  /// Optional: true
  pub signalSemaphoreValueCount: u32,
  /// Optional: true,  Length: signalSemaphoreValueCount
  pub pSignalSemaphoreValues: *const u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkTimelineSemaphoreSubmitInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkTimelineSemaphoreSubmitInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubmitInfo<'root>>
  for VkTimelineSemaphoreSubmitInfo<'child>
{
}
#[cfg(all(
  feature = "VK_BASE_VERSION_1_2",
  all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0"))
))]
unsafe impl<'child, 'root> VkPNextExtends<VkBindSparseInfo<'root>>
  for VkTimelineSemaphoreSubmitInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkTimelineSemaphoreSubmitInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::TIMELINE_SEMAPHORE_SUBMIT_INFO,
    pNext: core::ptr::null(),
    waitSemaphoreValueCount: 0,
    pWaitSemaphoreValues: core::ptr::null(),
    signalSemaphoreValueCount: 0,
    pSignalSemaphoreValues: core::ptr::null(),
    _marker: core::marker::PhantomData,
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
  pub const fn with_waitSemaphoreValueCount(mut self, val: u32) -> Self {
    self.waitSemaphoreValueCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pWaitSemaphoreValues(mut self, val: &'a [u64]) -> Self {
    self.waitSemaphoreValueCount = val.len() as u32;
    self.pWaitSemaphoreValues = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_signalSemaphoreValueCount(mut self, val: u32) -> Self {
    self.signalSemaphoreValueCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSignalSemaphoreValues(mut self, val: &'a [u64]) -> Self {
    self.signalSemaphoreValueCount = val.len() as u32;
    self.pSignalSemaphoreValues = val.as_ptr();
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
/// [VkSemaphoreWaitInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSemaphoreWaitInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkSemaphoreWaitFlags,
  pub semaphoreCount: u32,
  /// Length: semaphoreCount
  pub pSemaphores: *const VkSemaphore,
  /// Length: semaphoreCount
  pub pValues: *const u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkSemaphoreWaitInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkSemaphoreWaitInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkSemaphoreWaitInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SEMAPHORE_WAIT_INFO,
    pNext: core::ptr::null(),
    flags: VkSemaphoreWaitFlagBits(0),
    semaphoreCount: 0,
    pSemaphores: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkSemaphoreWaitFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_semaphoreCount(mut self, val: u32) -> Self {
    self.semaphoreCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pSemaphores(mut self, val: &'a [VkSemaphore]) -> Self {
    self.semaphoreCount = val.len() as u32;
    self.pSemaphores = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pValues(mut self, val: &'a [u64]) -> Self {
    self.semaphoreCount = val.len() as u32;
    self.pValues = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure every provided array constrained by `semaphoreCount` has the same length. Optional pointer arguments may be null, but non-null pointers must be valid for that same length and outlive any use of this struct instance.
  #[inline]
  pub const fn with_semaphoreCount_slices(
    mut self,
    pSemaphores: &'a [VkSemaphore],
    pValues: &'a [u64],
  ) -> Self {
    let len = pSemaphores.len();
    self.semaphoreCount = len as u32;
    self.pSemaphores = pSemaphores.as_ptr();
    self.pValues = pValues.as_ptr();
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSemaphoreWaitInfo<
    'root,
    T: VkPNextExtends<VkSemaphoreWaitInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSemaphoreSignalInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreSignalInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSemaphoreSignalInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub semaphore: VkSemaphore,
  pub value: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkSemaphoreSignalInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkSemaphoreSignalInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkSemaphoreSignalInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SEMAPHORE_SIGNAL_INFO,
    pNext: core::ptr::null(),
    semaphore: VkSemaphore::DEFAULT,
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
  pub const fn with_semaphore(mut self, val: VkSemaphore) -> Self {
    self.semaphore = val;
    self
  }
  #[inline]
  pub const fn with_value(mut self, val: u64) -> Self {
    self.value = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSemaphoreSignalInfo<
    'root,
    T: VkPNextExtends<VkSemaphoreSignalInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceVulkanMemoryModelFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkanMemoryModelFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub vulkanMemoryModel: VkBool32,
  pub vulkanMemoryModelDeviceScope: VkBool32,
  pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceVulkanMemoryModelFeatures<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceVulkanMemoryModelFeatures<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceVulkanMemoryModelFeatures<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceVulkanMemoryModelFeatures<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceVulkanMemoryModelFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES,
    pNext: core::ptr::null_mut(),
    vulkanMemoryModel: 0,
    vulkanMemoryModelDeviceScope: 0,
    vulkanMemoryModelAvailabilityVisibilityChains: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_vulkanMemoryModel(mut self, val: VkBool32) -> Self {
    self.vulkanMemoryModel = val;
    self
  }
  #[inline]
  pub const fn with_vulkanMemoryModelDeviceScope(mut self, val: VkBool32) -> Self {
    self.vulkanMemoryModelDeviceScope = val;
    self
  }
  #[inline]
  pub const fn with_vulkanMemoryModelAvailabilityVisibilityChains(mut self, val: VkBool32) -> Self {
    self.vulkanMemoryModelAvailabilityVisibilityChains = val;
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
/// [VkPhysicalDeviceBufferDeviceAddressFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceBufferDeviceAddressFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub bufferDeviceAddress: VkBool32,
  pub bufferDeviceAddressCaptureReplay: VkBool32,
  pub bufferDeviceAddressMultiDevice: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceBufferDeviceAddressFeatures<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceBufferDeviceAddressFeatures<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceBufferDeviceAddressFeatures<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceBufferDeviceAddressFeatures<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceBufferDeviceAddressFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES,
    pNext: core::ptr::null_mut(),
    bufferDeviceAddress: 0,
    bufferDeviceAddressCaptureReplay: 0,
    bufferDeviceAddressMultiDevice: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_bufferDeviceAddress(mut self, val: VkBool32) -> Self {
    self.bufferDeviceAddress = val;
    self
  }
  #[inline]
  pub const fn with_bufferDeviceAddressCaptureReplay(mut self, val: VkBool32) -> Self {
    self.bufferDeviceAddressCaptureReplay = val;
    self
  }
  #[inline]
  pub const fn with_bufferDeviceAddressMultiDevice(mut self, val: VkBool32) -> Self {
    self.bufferDeviceAddressMultiDevice = val;
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
/// [VkBufferDeviceAddressInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferDeviceAddressInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferDeviceAddressInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub buffer: VkBuffer,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkBufferDeviceAddressInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkBufferDeviceAddressInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkBufferDeviceAddressInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_DEVICE_ADDRESS_INFO,
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
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBufferDeviceAddressInfo<
    'root,
    T: VkPNextExtends<VkBufferDeviceAddressInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBufferOpaqueCaptureAddressCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferOpaqueCaptureAddressCreateInfo.html)
///
/// **Extends:** VkBufferCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBufferOpaqueCaptureAddressCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub opaqueCaptureAddress: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkBufferOpaqueCaptureAddressCreateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkBufferOpaqueCaptureAddressCreateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkBufferCreateInfo<'root>>
  for VkBufferOpaqueCaptureAddressCreateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkBufferOpaqueCaptureAddressCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO,
    pNext: core::ptr::null(),
    opaqueCaptureAddress: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_opaqueCaptureAddress(mut self, val: u64) -> Self {
    self.opaqueCaptureAddress = val;
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
/// [VkMemoryOpaqueCaptureAddressAllocateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryOpaqueCaptureAddressAllocateInfo.html)
///
/// **Extends:** VkMemoryAllocateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub opaqueCaptureAddress: u64,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkMemoryOpaqueCaptureAddressAllocateInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkMemoryOpaqueCaptureAddressAllocateInfo<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkMemoryAllocateInfo<'root>>
  for VkMemoryOpaqueCaptureAddressAllocateInfo<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkMemoryOpaqueCaptureAddressAllocateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO,
    pNext: core::ptr::null(),
    opaqueCaptureAddress: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_opaqueCaptureAddress(mut self, val: u64) -> Self {
    self.opaqueCaptureAddress = val;
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
/// [VkDeviceMemoryOpaqueCaptureAddressInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryOpaqueCaptureAddressInfo.html)
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub memory: VkDeviceMemory,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkDeviceMemoryOpaqueCaptureAddressInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkDeviceMemoryOpaqueCaptureAddressInfo<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkDeviceMemoryOpaqueCaptureAddressInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO,
    pNext: core::ptr::null(),
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
  pub const fn with_memory(mut self, val: VkDeviceMemory) -> Self {
    self.memory = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkDeviceMemoryOpaqueCaptureAddressInfo<
    'root,
    T: VkPNextExtends<VkDeviceMemoryOpaqueCaptureAddressInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceVulkan11Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan11Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVulkan11Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub storageBuffer16BitAccess: VkBool32,
  pub uniformAndStorageBuffer16BitAccess: VkBool32,
  pub storagePushConstant16: VkBool32,
  pub storageInputOutput16: VkBool32,
  #[cfg_attr(
    feature = "VKSC_VERSION_1_0",
    deprecated(
      note = "`multiview` is removed by `VKSC_VERSION_1_0`; the field remains present only for ABI compatibility"
    )
  )]
  pub multiview: VkBool32,
  pub multiviewGeometryShader: VkBool32,
  pub multiviewTessellationShader: VkBool32,
  pub variablePointersStorageBuffer: VkBool32,
  pub variablePointers: VkBool32,
  pub protectedMemory: VkBool32,
  pub samplerYcbcrConversion: VkBool32,
  pub shaderDrawParameters: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceVulkan11Features<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceVulkan11Features<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceVulkan11Features<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceVulkan11Features<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceVulkan11Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES,
    pNext: core::ptr::null_mut(),
    storageBuffer16BitAccess: 0,
    uniformAndStorageBuffer16BitAccess: 0,
    storagePushConstant16: 0,
    storageInputOutput16: 0,
    multiview: 0,
    multiviewGeometryShader: 0,
    multiviewTessellationShader: 0,
    variablePointersStorageBuffer: 0,
    variablePointers: 0,
    protectedMemory: 0,
    samplerYcbcrConversion: 0,
    shaderDrawParameters: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_storageBuffer16BitAccess(mut self, val: VkBool32) -> Self {
    self.storageBuffer16BitAccess = val;
    self
  }
  #[inline]
  pub const fn with_uniformAndStorageBuffer16BitAccess(mut self, val: VkBool32) -> Self {
    self.uniformAndStorageBuffer16BitAccess = val;
    self
  }
  #[inline]
  pub const fn with_storagePushConstant16(mut self, val: VkBool32) -> Self {
    self.storagePushConstant16 = val;
    self
  }
  #[inline]
  pub const fn with_storageInputOutput16(mut self, val: VkBool32) -> Self {
    self.storageInputOutput16 = val;
    self
  }
  #[inline]
  pub const fn with_multiview(mut self, val: VkBool32) -> Self {
    self.multiview = val;
    self
  }
  #[inline]
  pub const fn with_multiviewGeometryShader(mut self, val: VkBool32) -> Self {
    self.multiviewGeometryShader = val;
    self
  }
  #[inline]
  pub const fn with_multiviewTessellationShader(mut self, val: VkBool32) -> Self {
    self.multiviewTessellationShader = val;
    self
  }
  #[inline]
  pub const fn with_variablePointersStorageBuffer(mut self, val: VkBool32) -> Self {
    self.variablePointersStorageBuffer = val;
    self
  }
  #[inline]
  pub const fn with_variablePointers(mut self, val: VkBool32) -> Self {
    self.variablePointers = val;
    self
  }
  #[inline]
  pub const fn with_protectedMemory(mut self, val: VkBool32) -> Self {
    self.protectedMemory = val;
    self
  }
  #[inline]
  pub const fn with_samplerYcbcrConversion(mut self, val: VkBool32) -> Self {
    self.samplerYcbcrConversion = val;
    self
  }
  #[inline]
  pub const fn with_shaderDrawParameters(mut self, val: VkBool32) -> Self {
    self.shaderDrawParameters = val;
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
/// [VkPhysicalDeviceVulkan11Properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan11Properties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVulkan11Properties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Exact]
  pub deviceUUID: [u8; VK_UUID_SIZE as usize],
  /// Limit Type: [Exact]
  pub driverUUID: [u8; VK_UUID_SIZE as usize],
  /// Limit Type: [Exact]
  pub deviceLUID: [u8; VK_LUID_SIZE as usize],
  /// Limit Type: [Exact]
  pub deviceNodeMask: u32,
  /// Limit Type: [Exact]
  pub deviceLUIDValid: VkBool32,
  /// Limit Type: [Max, Pot],  No Auto-Validity
  pub subgroupSize: u32,
  /// Limit Type: [Bitmask],  No Auto-Validity
  pub subgroupSupportedStages: VkShaderStageFlags,
  /// Limit Type: [Bitmask],  No Auto-Validity
  pub subgroupSupportedOperations: VkSubgroupFeatureFlags,
  /// Limit Type: [Max],  No Auto-Validity
  pub subgroupQuadOperationsInAllStages: VkBool32,
  /// Limit Type: [Exact]
  pub pointClippingBehavior: VkPointClippingBehavior,
  /// Limit Type: [Max]
  pub maxMultiviewViewCount: u32,
  /// Limit Type: [Max]
  pub maxMultiviewInstanceIndex: u32,
  /// Limit Type: [Exact]
  pub protectedNoFault: VkBool32,
  /// Limit Type: [Max]
  pub maxPerSetDescriptors: u32,
  /// Limit Type: [Max]
  pub maxMemoryAllocationSize: VkDeviceSize,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceVulkan11Properties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceVulkan11Properties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceVulkan11Properties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceVulkan11Properties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES,
    pNext: core::ptr::null_mut(),
    deviceUUID: [0u8; VK_UUID_SIZE as usize],
    driverUUID: [0u8; VK_UUID_SIZE as usize],
    deviceLUID: [0u8; VK_LUID_SIZE as usize],
    deviceNodeMask: 0,
    deviceLUIDValid: 0,
    subgroupSize: 0,
    subgroupSupportedStages: VkShaderStageFlagBits(0),
    subgroupSupportedOperations: VkSubgroupFeatureFlagBits(0),
    subgroupQuadOperationsInAllStages: 0,
    pointClippingBehavior: VkPointClippingBehavior(0),
    maxMultiviewViewCount: 0,
    maxMultiviewInstanceIndex: 0,
    protectedNoFault: 0,
    maxPerSetDescriptors: 0,
    maxMemoryAllocationSize: 0,
    _marker: core::marker::PhantomData,
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
  #[inline]
  pub const fn with_subgroupSize(mut self, val: u32) -> Self {
    self.subgroupSize = val;
    self
  }
  #[inline]
  pub const fn with_subgroupSupportedStages(mut self, val: VkShaderStageFlags) -> Self {
    self.subgroupSupportedStages = val;
    self
  }
  #[inline]
  pub const fn with_subgroupSupportedOperations(mut self, val: VkSubgroupFeatureFlags) -> Self {
    self.subgroupSupportedOperations = val;
    self
  }
  #[inline]
  pub const fn with_subgroupQuadOperationsInAllStages(mut self, val: VkBool32) -> Self {
    self.subgroupQuadOperationsInAllStages = val;
    self
  }
  #[inline]
  pub const fn with_pointClippingBehavior(mut self, val: VkPointClippingBehavior) -> Self {
    self.pointClippingBehavior = val;
    self
  }
  #[inline]
  pub const fn with_maxMultiviewViewCount(mut self, val: u32) -> Self {
    self.maxMultiviewViewCount = val;
    self
  }
  #[inline]
  pub const fn with_maxMultiviewInstanceIndex(mut self, val: u32) -> Self {
    self.maxMultiviewInstanceIndex = val;
    self
  }
  #[inline]
  pub const fn with_protectedNoFault(mut self, val: VkBool32) -> Self {
    self.protectedNoFault = val;
    self
  }
  #[inline]
  pub const fn with_maxPerSetDescriptors(mut self, val: u32) -> Self {
    self.maxPerSetDescriptors = val;
    self
  }
  #[inline]
  pub const fn with_maxMemoryAllocationSize(mut self, val: VkDeviceSize) -> Self {
    self.maxMemoryAllocationSize = val;
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
/// [VkPhysicalDeviceVulkan12Features](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan12Features.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVulkan12Features<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub samplerMirrorClampToEdge: VkBool32,
  pub drawIndirectCount: VkBool32,
  pub storageBuffer8BitAccess: VkBool32,
  pub uniformAndStorageBuffer8BitAccess: VkBool32,
  pub storagePushConstant8: VkBool32,
  pub shaderBufferInt64Atomics: VkBool32,
  pub shaderSharedInt64Atomics: VkBool32,
  pub shaderFloat16: VkBool32,
  pub shaderInt8: VkBool32,
  pub descriptorIndexing: VkBool32,
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
  pub samplerFilterMinmax: VkBool32,
  pub scalarBlockLayout: VkBool32,
  pub imagelessFramebuffer: VkBool32,
  pub uniformBufferStandardLayout: VkBool32,
  pub shaderSubgroupExtendedTypes: VkBool32,
  pub separateDepthStencilLayouts: VkBool32,
  pub hostQueryReset: VkBool32,
  #[cfg_attr(
    feature = "VKSC_VERSION_1_0",
    deprecated(
      note = "`timelineSemaphore` is removed by `VKSC_VERSION_1_0`; the field remains present only for ABI compatibility"
    )
  )]
  pub timelineSemaphore: VkBool32,
  pub bufferDeviceAddress: VkBool32,
  pub bufferDeviceAddressCaptureReplay: VkBool32,
  pub bufferDeviceAddressMultiDevice: VkBool32,
  pub vulkanMemoryModel: VkBool32,
  pub vulkanMemoryModelDeviceScope: VkBool32,
  pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
  pub shaderOutputViewportIndex: VkBool32,
  pub shaderOutputLayer: VkBool32,
  pub subgroupBroadcastDynamicId: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceVulkan12Features<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceVulkan12Features<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceVulkan12Features<'child>
{
}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceVulkan12Features<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceVulkan12Features<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
    pNext: core::ptr::null_mut(),
    samplerMirrorClampToEdge: 0,
    drawIndirectCount: 0,
    storageBuffer8BitAccess: 0,
    uniformAndStorageBuffer8BitAccess: 0,
    storagePushConstant8: 0,
    shaderBufferInt64Atomics: 0,
    shaderSharedInt64Atomics: 0,
    shaderFloat16: 0,
    shaderInt8: 0,
    descriptorIndexing: 0,
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
    samplerFilterMinmax: 0,
    scalarBlockLayout: 0,
    imagelessFramebuffer: 0,
    uniformBufferStandardLayout: 0,
    shaderSubgroupExtendedTypes: 0,
    separateDepthStencilLayouts: 0,
    hostQueryReset: 0,
    timelineSemaphore: 0,
    bufferDeviceAddress: 0,
    bufferDeviceAddressCaptureReplay: 0,
    bufferDeviceAddressMultiDevice: 0,
    vulkanMemoryModel: 0,
    vulkanMemoryModelDeviceScope: 0,
    vulkanMemoryModelAvailabilityVisibilityChains: 0,
    shaderOutputViewportIndex: 0,
    shaderOutputLayer: 0,
    subgroupBroadcastDynamicId: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_samplerMirrorClampToEdge(mut self, val: VkBool32) -> Self {
    self.samplerMirrorClampToEdge = val;
    self
  }
  #[inline]
  pub const fn with_drawIndirectCount(mut self, val: VkBool32) -> Self {
    self.drawIndirectCount = val;
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
  #[inline]
  pub const fn with_descriptorIndexing(mut self, val: VkBool32) -> Self {
    self.descriptorIndexing = val;
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
  #[inline]
  pub const fn with_samplerFilterMinmax(mut self, val: VkBool32) -> Self {
    self.samplerFilterMinmax = val;
    self
  }
  #[inline]
  pub const fn with_scalarBlockLayout(mut self, val: VkBool32) -> Self {
    self.scalarBlockLayout = val;
    self
  }
  #[inline]
  pub const fn with_imagelessFramebuffer(mut self, val: VkBool32) -> Self {
    self.imagelessFramebuffer = val;
    self
  }
  #[inline]
  pub const fn with_uniformBufferStandardLayout(mut self, val: VkBool32) -> Self {
    self.uniformBufferStandardLayout = val;
    self
  }
  #[inline]
  pub const fn with_shaderSubgroupExtendedTypes(mut self, val: VkBool32) -> Self {
    self.shaderSubgroupExtendedTypes = val;
    self
  }
  #[inline]
  pub const fn with_separateDepthStencilLayouts(mut self, val: VkBool32) -> Self {
    self.separateDepthStencilLayouts = val;
    self
  }
  #[inline]
  pub const fn with_hostQueryReset(mut self, val: VkBool32) -> Self {
    self.hostQueryReset = val;
    self
  }
  #[inline]
  pub const fn with_timelineSemaphore(mut self, val: VkBool32) -> Self {
    self.timelineSemaphore = val;
    self
  }
  #[inline]
  pub const fn with_bufferDeviceAddress(mut self, val: VkBool32) -> Self {
    self.bufferDeviceAddress = val;
    self
  }
  #[inline]
  pub const fn with_bufferDeviceAddressCaptureReplay(mut self, val: VkBool32) -> Self {
    self.bufferDeviceAddressCaptureReplay = val;
    self
  }
  #[inline]
  pub const fn with_bufferDeviceAddressMultiDevice(mut self, val: VkBool32) -> Self {
    self.bufferDeviceAddressMultiDevice = val;
    self
  }
  #[inline]
  pub const fn with_vulkanMemoryModel(mut self, val: VkBool32) -> Self {
    self.vulkanMemoryModel = val;
    self
  }
  #[inline]
  pub const fn with_vulkanMemoryModelDeviceScope(mut self, val: VkBool32) -> Self {
    self.vulkanMemoryModelDeviceScope = val;
    self
  }
  #[inline]
  pub const fn with_vulkanMemoryModelAvailabilityVisibilityChains(mut self, val: VkBool32) -> Self {
    self.vulkanMemoryModelAvailabilityVisibilityChains = val;
    self
  }
  #[inline]
  pub const fn with_shaderOutputViewportIndex(mut self, val: VkBool32) -> Self {
    self.shaderOutputViewportIndex = val;
    self
  }
  #[inline]
  pub const fn with_shaderOutputLayer(mut self, val: VkBool32) -> Self {
    self.shaderOutputLayer = val;
    self
  }
  #[inline]
  pub const fn with_subgroupBroadcastDynamicId(mut self, val: VkBool32) -> Self {
    self.subgroupBroadcastDynamicId = val;
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
/// [VkPhysicalDeviceVulkan12Properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan12Properties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_BASE_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVulkan12Properties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Noauto]
  pub driverID: VkDriverId,
  /// Length: null-terminated,  Limit Type: [Noauto]
  pub driverName: [c_char; VK_MAX_DRIVER_NAME_SIZE as usize],
  /// Length: null-terminated,  Limit Type: [Noauto]
  pub driverInfo: [c_char; VK_MAX_DRIVER_INFO_SIZE as usize],
  /// Limit Type: [Noauto]
  pub conformanceVersion: VkConformanceVersion,
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
  /// Limit Type: [Bitmask]
  pub supportedDepthResolveModes: VkResolveModeFlags,
  /// Limit Type: [Bitmask]
  pub supportedStencilResolveModes: VkResolveModeFlags,
  /// Limit Type: [Max]
  pub independentResolveNone: VkBool32,
  /// Limit Type: [Max]
  pub independentResolve: VkBool32,
  /// Limit Type: [Max]
  pub filterMinmaxSingleComponentFormats: VkBool32,
  /// Limit Type: [Max]
  pub filterMinmaxImageComponentMapping: VkBool32,
  /// Limit Type: [Max]
  pub maxTimelineSemaphoreValueDifference: u64,
  /// Optional: true,  Limit Type: [Bitmask]
  pub framebufferIntegerColorSampleCounts: VkSampleCountFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceVulkan12Properties<'a> {}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceVulkan12Properties<'a> {}
#[cfg(all(feature = "VK_BASE_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceVulkan12Properties<'child>
{
}
#[cfg(feature = "VK_BASE_VERSION_1_2")]
impl<'a> VkPhysicalDeviceVulkan12Properties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES,
    pNext: core::ptr::null_mut(),
    driverID: VkDriverId(0),
    driverName: [0 as c_char; VK_MAX_DRIVER_NAME_SIZE as usize],
    driverInfo: [0 as c_char; VK_MAX_DRIVER_INFO_SIZE as usize],
    conformanceVersion: VkConformanceVersion::DEFAULT,
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
    supportedDepthResolveModes: VkResolveModeFlagBits(0),
    supportedStencilResolveModes: VkResolveModeFlagBits(0),
    independentResolveNone: 0,
    independentResolve: 0,
    filterMinmaxSingleComponentFormats: 0,
    filterMinmaxImageComponentMapping: 0,
    maxTimelineSemaphoreValueDifference: 0,
    framebufferIntegerColorSampleCounts: VkSampleCountFlagBits(0),
    _marker: core::marker::PhantomData,
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
  pub const fn with_driverID(mut self, val: VkDriverId) -> Self {
    self.driverID = val;
    self
  }
  #[inline]
  pub const fn with_driverName(mut self, val: [c_char; VK_MAX_DRIVER_NAME_SIZE as usize]) -> Self {
    self.driverName = val;
    self
  }
  #[inline]
  pub const fn with_driverInfo(mut self, val: [c_char; VK_MAX_DRIVER_INFO_SIZE as usize]) -> Self {
    self.driverInfo = val;
    self
  }
  #[inline]
  pub const fn with_conformanceVersion(mut self, val: VkConformanceVersion) -> Self {
    self.conformanceVersion = val;
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
  #[inline]
  pub const fn with_supportedDepthResolveModes(mut self, val: VkResolveModeFlags) -> Self {
    self.supportedDepthResolveModes = val;
    self
  }
  #[inline]
  pub const fn with_supportedStencilResolveModes(mut self, val: VkResolveModeFlags) -> Self {
    self.supportedStencilResolveModes = val;
    self
  }
  #[inline]
  pub const fn with_independentResolveNone(mut self, val: VkBool32) -> Self {
    self.independentResolveNone = val;
    self
  }
  #[inline]
  pub const fn with_independentResolve(mut self, val: VkBool32) -> Self {
    self.independentResolve = val;
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
  #[inline]
  pub const fn with_maxTimelineSemaphoreValueDifference(mut self, val: u64) -> Self {
    self.maxTimelineSemaphoreValueDifference = val;
    self
  }
  #[inline]
  pub const fn with_framebufferIntegerColorSampleCounts(mut self, val: VkSampleCountFlags) -> Self {
    self.framebufferIntegerColorSampleCounts = val;
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
