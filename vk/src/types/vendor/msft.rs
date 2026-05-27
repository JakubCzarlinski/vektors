#[cfg(feature = "VK_MSFT_layered_driver")]
use crate::enums::VkLayeredDriverUnderlyingApiMSFT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
use core::ffi::c_void;
/// [VkPhysicalDeviceLayeredDriverPropertiesMSFT](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLayeredDriverPropertiesMSFT.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_MSFT_layered_driver")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceLayeredDriverPropertiesMSFT<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Exact]
  pub underlyingAPI: VkLayeredDriverUnderlyingApiMSFT,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_MSFT_layered_driver")]
unsafe impl<'a> Send for VkPhysicalDeviceLayeredDriverPropertiesMSFT<'a> {}
#[cfg(feature = "VK_MSFT_layered_driver")]
unsafe impl<'a> Sync for VkPhysicalDeviceLayeredDriverPropertiesMSFT<'a> {}
#[cfg(all(feature = "VK_MSFT_layered_driver", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceLayeredDriverPropertiesMSFT<'child>
{
}
#[cfg(feature = "VK_MSFT_layered_driver")]
impl<'a> VkPhysicalDeviceLayeredDriverPropertiesMSFT<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT,
    pNext: core::ptr::null_mut(),
    underlyingAPI: VkLayeredDriverUnderlyingApiMSFT(0),
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
  pub const fn with_underlyingAPI(mut self, val: VkLayeredDriverUnderlyingApiMSFT) -> Self {
    self.underlyingAPI = val;
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
