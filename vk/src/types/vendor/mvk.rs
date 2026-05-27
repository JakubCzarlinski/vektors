#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
use crate::types::VkPNextExtends;
use core::ffi::c_void;
/// [VkIOSSurfaceCreateFlagsMVK](https://docs.vulkan.org/refpages/latest/refpages/source/VkIOSSurfaceCreateFlagsMVK.html)
#[cfg(feature = "VK_MVK_ios_surface")]
pub type VkIOSSurfaceCreateFlagsMVK = VkFlags;
/// [VkIOSSurfaceCreateInfoMVK](https://docs.vulkan.org/refpages/latest/refpages/source/VkIOSSurfaceCreateInfoMVK.html)
#[cfg(feature = "VK_MVK_ios_surface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkIOSSurfaceCreateInfoMVK<'a> {
  /// Values: VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkIOSSurfaceCreateFlagsMVK,
  /// No Auto-Validity
  pub pView: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_MVK_ios_surface")]
unsafe impl<'a> Send for VkIOSSurfaceCreateInfoMVK<'a> {}
#[cfg(feature = "VK_MVK_ios_surface")]
unsafe impl<'a> Sync for VkIOSSurfaceCreateInfoMVK<'a> {}
#[cfg(feature = "VK_MVK_ios_surface")]
impl<'a> VkIOSSurfaceCreateInfoMVK<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IOS_SURFACE_CREATE_INFO_MVK,
    pNext: core::ptr::null(),
    flags: 0,
    pView: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkIOSSurfaceCreateFlagsMVK) -> Self {
    self.flags = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pView(mut self, val: *const c_void) -> Self {
    self.pView = val;
    self
  }
  #[cfg(feature = "VK_MVK_ios_surface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkIOSSurfaceCreateInfoMVK<
    'root,
    T: VkPNextExtends<VkIOSSurfaceCreateInfoMVK<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkMacOSSurfaceCreateFlagsMVK](https://docs.vulkan.org/refpages/latest/refpages/source/VkMacOSSurfaceCreateFlagsMVK.html)
#[cfg(feature = "VK_MVK_macos_surface")]
pub type VkMacOSSurfaceCreateFlagsMVK = VkFlags;
/// [VkMacOSSurfaceCreateInfoMVK](https://docs.vulkan.org/refpages/latest/refpages/source/VkMacOSSurfaceCreateInfoMVK.html)
#[cfg(feature = "VK_MVK_macos_surface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkMacOSSurfaceCreateInfoMVK<'a> {
  /// Values: VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkMacOSSurfaceCreateFlagsMVK,
  /// No Auto-Validity
  pub pView: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_MVK_macos_surface")]
unsafe impl<'a> Send for VkMacOSSurfaceCreateInfoMVK<'a> {}
#[cfg(feature = "VK_MVK_macos_surface")]
unsafe impl<'a> Sync for VkMacOSSurfaceCreateInfoMVK<'a> {}
#[cfg(feature = "VK_MVK_macos_surface")]
impl<'a> VkMacOSSurfaceCreateInfoMVK<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::MACOS_SURFACE_CREATE_INFO_MVK,
    pNext: core::ptr::null(),
    flags: 0,
    pView: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkMacOSSurfaceCreateFlagsMVK) -> Self {
    self.flags = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pView(mut self, val: *const c_void) -> Self {
    self.pView = val;
    self
  }
  #[cfg(feature = "VK_MVK_macos_surface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkMacOSSurfaceCreateInfoMVK<
    'root,
    T: VkPNextExtends<VkMacOSSurfaceCreateInfoMVK<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
