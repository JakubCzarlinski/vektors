#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
use core::ffi::c_void;
/// [VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_IMG_relaxed_line_rasterization")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub relaxedLineRasterization: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_IMG_relaxed_line_rasterization")]
unsafe impl<'a> Send for VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'a> {}
#[cfg(feature = "VK_IMG_relaxed_line_rasterization")]
unsafe impl<'a> Sync for VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'a> {}
#[cfg(all(
  feature = "VK_IMG_relaxed_line_rasterization",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'child>
{
}
#[cfg(all(
  feature = "VK_IMG_relaxed_line_rasterization",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'child>
{
}
#[cfg(feature = "VK_IMG_relaxed_line_rasterization")]
impl<'a> VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'a> {
  pub const DEFAULT: Self = Self {
    sType:
      VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG,
    pNext: core::ptr::null_mut(),
    relaxedLineRasterization: 0,
    _marker: core::marker::PhantomData,
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
  pub const fn with_relaxedLineRasterization(mut self, val: VkBool32) -> Self {
    self.relaxedLineRasterization = val;
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
