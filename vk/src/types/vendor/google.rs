#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::types::VkPresentInfoKHR;
use core::ffi::c_void;
/// [VkRefreshCycleDurationGOOGLE](https://docs.vulkan.org/refpages/latest/refpages/source/VkRefreshCycleDurationGOOGLE.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRefreshCycleDurationGOOGLE {
  pub refreshDuration: u64,
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl Send for VkRefreshCycleDurationGOOGLE {}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl Sync for VkRefreshCycleDurationGOOGLE {}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl VkRefreshCycleDurationGOOGLE {
  pub const DEFAULT: Self = Self { refreshDuration: 0 };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_refreshDuration(mut self, val: u64) -> Self {
    self.refreshDuration = val;
    self
  }
}
/// [VkPastPresentationTimingGOOGLE](https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingGOOGLE.html)
///
/// *Note: This is a **returned only** struct.*
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPastPresentationTimingGOOGLE {
  pub presentID: u32,
  pub desiredPresentTime: u64,
  pub actualPresentTime: u64,
  pub earliestPresentTime: u64,
  pub presentMargin: u64,
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl Send for VkPastPresentationTimingGOOGLE {}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl Sync for VkPastPresentationTimingGOOGLE {}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl VkPastPresentationTimingGOOGLE {
  pub const DEFAULT: Self = Self {
    presentID: 0,
    desiredPresentTime: 0,
    actualPresentTime: 0,
    earliestPresentTime: 0,
    presentMargin: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_presentID(mut self, val: u32) -> Self {
    self.presentID = val;
    self
  }
  #[inline]
  pub const fn with_desiredPresentTime(mut self, val: u64) -> Self {
    self.desiredPresentTime = val;
    self
  }
  #[inline]
  pub const fn with_actualPresentTime(mut self, val: u64) -> Self {
    self.actualPresentTime = val;
    self
  }
  #[inline]
  pub const fn with_earliestPresentTime(mut self, val: u64) -> Self {
    self.earliestPresentTime = val;
    self
  }
  #[inline]
  pub const fn with_presentMargin(mut self, val: u64) -> Self {
    self.presentMargin = val;
    self
  }
}
/// [VkPresentTimesInfoGOOGLE](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimesInfoGOOGLE.html)
///
/// **Extends:** VkPresentInfoKHR.
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPresentTimesInfoGOOGLE<'a> {
  /// Values: VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub swapchainCount: u32,
  /// Optional: true,  Length: swapchainCount
  pub pTimes: *const VkPresentTimeGOOGLE,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl<'a> Send for VkPresentTimesInfoGOOGLE<'a> {}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl<'a> Sync for VkPresentTimesInfoGOOGLE<'a> {}
#[cfg(all(feature = "VK_GOOGLE_display_timing", feature = "VK_KHR_swapchain"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPresentInfoKHR<'root>>
  for VkPresentTimesInfoGOOGLE<'child>
{
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl<'a> VkPresentTimesInfoGOOGLE<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PRESENT_TIMES_INFO_GOOGLE,
    pNext: core::ptr::null(),
    swapchainCount: 0,
    pTimes: core::ptr::null(),
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
  pub const fn with_swapchainCount(mut self, val: u32) -> Self {
    self.swapchainCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pTimes(mut self, val: &'a [VkPresentTimeGOOGLE]) -> Self {
    self.swapchainCount = val.len() as u32;
    self.pTimes = val.as_ptr();
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
}
/// [VkPresentTimeGOOGLE](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimeGOOGLE.html)
#[cfg(feature = "VK_GOOGLE_display_timing")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPresentTimeGOOGLE {
  pub presentID: u32,
  pub desiredPresentTime: u64,
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl Send for VkPresentTimeGOOGLE {}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl Sync for VkPresentTimeGOOGLE {}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl VkPresentTimeGOOGLE {
  pub const DEFAULT: Self = Self {
    presentID: 0,
    desiredPresentTime: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_presentID(mut self, val: u32) -> Self {
    self.presentID = val;
    self
  }
  #[inline]
  pub const fn with_desiredPresentTime(mut self, val: u64) -> Self {
    self.desiredPresentTime = val;
    self
  }
}
