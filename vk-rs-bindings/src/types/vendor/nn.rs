#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
use crate::types::VkPNextExtends;
use core::ffi::c_void;
/// [VkViSurfaceCreateFlagsNN](https://docs.vulkan.org/refpages/latest/refpages/source/VkViSurfaceCreateFlagsNN.html)
#[cfg(feature = "VK_NN_vi_surface")]
pub type VkViSurfaceCreateFlagsNN = VkFlags;
/// [VkViSurfaceCreateInfoNN](https://docs.vulkan.org/refpages/latest/refpages/source/VkViSurfaceCreateInfoNN.html)
#[cfg(feature = "VK_NN_vi_surface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkViSurfaceCreateInfoNN<'a> {
  /// Values: VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkViSurfaceCreateFlagsNN,
  /// No Auto-Validity
  pub window: *mut c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_NN_vi_surface")]
unsafe impl<'a> Send for VkViSurfaceCreateInfoNN<'a> {}
#[cfg(feature = "VK_NN_vi_surface")]
unsafe impl<'a> Sync for VkViSurfaceCreateInfoNN<'a> {}
#[cfg(feature = "VK_NN_vi_surface")]
impl<'a> VkViSurfaceCreateInfoNN<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN,
    pNext: core::ptr::null(),
    flags: 0,
    window: core::ptr::null_mut(),
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
  pub const fn with_flags(mut self, val: VkViSurfaceCreateFlagsNN) -> Self {
    self.flags = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_window(mut self, val: *mut c_void) -> Self {
    self.window = val;
    self
  }
  #[cfg(feature = "VK_NN_vi_surface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkViSurfaceCreateInfoNN<
    'root,
    T: VkPNextExtends<VkViSurfaceCreateInfoNN<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
