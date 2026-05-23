#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::types::VkPresentInfoKHR;
use core::ffi::c_void;
/// [GgpFrameToken](https://docs.vulkan.org/refpages/latest/refpages/source/GgpFrameToken.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_GGP_frame_token")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GgpFrameToken(pub *mut c_void);
#[cfg(feature = "VK_GGP_frame_token")]
impl GgpFrameToken {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_GGP_frame_token")]
unsafe impl Send for GgpFrameToken {}
#[cfg(feature = "VK_GGP_frame_token")]
unsafe impl Sync for GgpFrameToken {}
/// [VkPresentFrameTokenGGP](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentFrameTokenGGP.html)
///
/// **Extends:** VkPresentInfoKHR.
#[cfg(feature = "VK_GGP_frame_token")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPresentFrameTokenGGP<'a> {
  /// Values: VK_STRUCTURE_TYPE_PRESENT_FRAME_TOKEN_GGP
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub frameToken: GgpFrameToken,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GGP_frame_token")]
unsafe impl<'a> Send for VkPresentFrameTokenGGP<'a> {}
#[cfg(feature = "VK_GGP_frame_token")]
unsafe impl<'a> Sync for VkPresentFrameTokenGGP<'a> {}
#[cfg(all(feature = "VK_GGP_frame_token", feature = "VK_KHR_swapchain"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPresentInfoKHR<'root>>
  for VkPresentFrameTokenGGP<'child>
{
}
#[cfg(feature = "VK_GGP_frame_token")]
impl<'a> VkPresentFrameTokenGGP<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PRESENT_FRAME_TOKEN_GGP,
    pNext: core::ptr::null(),
    frameToken: GgpFrameToken::NULL,
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
  pub const fn with_frameToken(mut self, val: GgpFrameToken) -> Self {
    self.frameToken = val;
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
/// [GgpStreamDescriptor](https://docs.vulkan.org/refpages/latest/refpages/source/GgpStreamDescriptor.html)
/// Opaque platform handle - always used as a raw pointer.
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GgpStreamDescriptor(pub *mut c_void);
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
impl GgpStreamDescriptor {
  pub const NULL: Self = Self(core::ptr::null_mut());
}
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
unsafe impl Send for GgpStreamDescriptor {}
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
unsafe impl Sync for GgpStreamDescriptor {}
/// [VkStreamDescriptorSurfaceCreateFlagsGGP](https://docs.vulkan.org/refpages/latest/refpages/source/VkStreamDescriptorSurfaceCreateFlagsGGP.html)
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
pub type VkStreamDescriptorSurfaceCreateFlagsGGP = VkFlags;
/// [VkStreamDescriptorSurfaceCreateInfoGGP](https://docs.vulkan.org/refpages/latest/refpages/source/VkStreamDescriptorSurfaceCreateInfoGGP.html)
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkStreamDescriptorSurfaceCreateInfoGGP<'a> {
  /// Values: VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkStreamDescriptorSurfaceCreateFlagsGGP,
  pub streamDescriptor: GgpStreamDescriptor,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
unsafe impl<'a> Send for VkStreamDescriptorSurfaceCreateInfoGGP<'a> {}
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
unsafe impl<'a> Sync for VkStreamDescriptorSurfaceCreateInfoGGP<'a> {}
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
impl<'a> VkStreamDescriptorSurfaceCreateInfoGGP<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP,
    pNext: core::ptr::null(),
    flags: 0,
    streamDescriptor: GgpStreamDescriptor::NULL,
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
  pub const fn with_flags(mut self, val: VkStreamDescriptorSurfaceCreateFlagsGGP) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_streamDescriptor(mut self, val: GgpStreamDescriptor) -> Self {
    self.streamDescriptor = val;
    self
  }
  #[cfg(feature = "VK_GGP_stream_descriptor_surface")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkStreamDescriptorSurfaceCreateInfoGGP<
    'root,
    T: VkPNextExtends<VkStreamDescriptorSurfaceCreateInfoGGP<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
