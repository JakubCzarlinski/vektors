#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageAspectFlagBits;
#[cfg(any(feature = "VK_BASE_VERSION_1_1", feature = "VK_KHR_maintenance2"))]
use crate::enums::VkPointClippingBehavior;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(any(feature = "VK_GRAPHICS_VERSION_1_1", feature = "VK_KHR_maintenance2"))]
use crate::enums::VkTessellationDomainOrigin;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageAspectFlags;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineTessellationStateCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkRect2D;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPassBeginInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPassCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkRenderingInfo;
use core::ffi::c_void;
/// [VkPhysicalDeviceMultiviewFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMultiviewFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub multiview: VkBool32,
  pub multiviewGeometryShader: VkBool32,
  pub multiviewTessellationShader: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceMultiviewFeatures<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceMultiviewFeatures<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceMultiviewFeatures<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceMultiviewFeatures<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
impl<'a> VkPhysicalDeviceMultiviewFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES,
    pNext: core::ptr::null_mut(),
    multiview: 0,
    multiviewGeometryShader: 0,
    multiviewTessellationShader: 0,
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
/// [VkPhysicalDeviceMultiviewProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceMultiviewProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxMultiviewViewCount: u32,
  /// Limit Type: [Max]
  pub maxMultiviewInstanceIndex: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceMultiviewProperties<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceMultiviewProperties<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceMultiviewProperties<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
impl<'a> VkPhysicalDeviceMultiviewProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES,
    pNext: core::ptr::null_mut(),
    maxMultiviewViewCount: 0,
    maxMultiviewInstanceIndex: 0,
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
  pub const fn with_maxMultiviewViewCount(mut self, val: u32) -> Self {
    self.maxMultiviewViewCount = val;
    self
  }
  #[inline]
  pub const fn with_maxMultiviewInstanceIndex(mut self, val: u32) -> Self {
    self.maxMultiviewInstanceIndex = val;
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
/// [VkRenderPassMultiviewCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassMultiviewCreateInfo.html)
///
/// **Extends:** VkRenderPassCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassMultiviewCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub subpassCount: u32,
  /// Length: subpassCount
  pub pViewMasks: *const u32,
  /// Optional: true
  pub dependencyCount: u32,
  /// Length: dependencyCount
  pub pViewOffsets: *const i32,
  /// Optional: true
  pub correlationMaskCount: u32,
  /// Length: correlationMaskCount
  pub pCorrelationMasks: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Send for VkRenderPassMultiviewCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Sync for VkRenderPassMultiviewCreateInfo<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_1",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassCreateInfo<'root>>
  for VkRenderPassMultiviewCreateInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
impl<'a> VkRenderPassMultiviewCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO,
    pNext: core::ptr::null(),
    subpassCount: 0,
    pViewMasks: core::ptr::null(),
    dependencyCount: 0,
    pViewOffsets: core::ptr::null(),
    correlationMaskCount: 0,
    pCorrelationMasks: core::ptr::null(),
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
  pub const fn with_subpassCount(mut self, val: u32) -> Self {
    self.subpassCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pViewMasks(mut self, val: &'a [u32]) -> Self {
    self.subpassCount = val.len() as u32;
    self.pViewMasks = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_dependencyCount(mut self, val: u32) -> Self {
    self.dependencyCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pViewOffsets(mut self, val: &'a [i32]) -> Self {
    self.dependencyCount = val.len() as u32;
    self.pViewOffsets = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_correlationMaskCount(mut self, val: u32) -> Self {
    self.correlationMaskCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCorrelationMasks(mut self, val: &'a [u32]) -> Self {
    self.correlationMaskCount = val.len() as u32;
    self.pCorrelationMasks = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassCreateInfo<
    'root,
    T: VkPNextExtends<VkRenderPassCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkDeviceGroupRenderPassBeginInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupRenderPassBeginInfo.html)
///
/// **Extends:** VkRenderPassBeginInfo, VkRenderingInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkDeviceGroupRenderPassBeginInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub deviceMask: u32,
  /// Optional: true
  pub deviceRenderAreaCount: u32,
  /// Length: deviceRenderAreaCount
  pub pDeviceRenderAreas: *const VkRect2D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Send for VkDeviceGroupRenderPassBeginInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Sync for VkDeviceGroupRenderPassBeginInfo<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_1",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassBeginInfo<'root>>
  for VkDeviceGroupRenderPassBeginInfo<'child>
{
}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_1",
  feature = "VK_GRAPHICS_VERSION_1_3"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderingInfo<'root>>
  for VkDeviceGroupRenderPassBeginInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
impl<'a> VkDeviceGroupRenderPassBeginInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO,
    pNext: core::ptr::null(),
    deviceMask: 0,
    deviceRenderAreaCount: 0,
    pDeviceRenderAreas: core::ptr::null(),
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
  pub const fn with_deviceMask(mut self, val: u32) -> Self {
    self.deviceMask = val;
    self
  }
  #[inline]
  pub const fn with_deviceRenderAreaCount(mut self, val: u32) -> Self {
    self.deviceRenderAreaCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDeviceRenderAreas(mut self, val: &'a [VkRect2D]) -> Self {
    self.deviceRenderAreaCount = val.len() as u32;
    self.pDeviceRenderAreas = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassBeginInfo<
    'root,
    T: VkPNextExtends<VkRenderPassBeginInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderingInfo<
    'root,
    T: VkPNextExtends<VkRenderingInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkInputAttachmentAspectReference](https://docs.vulkan.org/refpages/latest/refpages/source/VkInputAttachmentAspectReference.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkInputAttachmentAspectReference {
  pub subpass: u32,
  pub inputAttachmentIndex: u32,
  pub aspectMask: VkImageAspectFlags,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl Send for VkInputAttachmentAspectReference {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl Sync for VkInputAttachmentAspectReference {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
impl VkInputAttachmentAspectReference {
  pub const DEFAULT: Self = Self {
    subpass: 0,
    inputAttachmentIndex: 0,
    aspectMask: VkImageAspectFlagBits(0),
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_subpass(mut self, val: u32) -> Self {
    self.subpass = val;
    self
  }
  #[inline]
  pub const fn with_inputAttachmentIndex(mut self, val: u32) -> Self {
    self.inputAttachmentIndex = val;
    self
  }
  #[inline]
  pub const fn with_aspectMask(mut self, val: VkImageAspectFlags) -> Self {
    self.aspectMask = val;
    self
  }
}
/// [VkRenderPassInputAttachmentAspectCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassInputAttachmentAspectCreateInfo.html)
///
/// **Extends:** VkRenderPassCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub aspectReferenceCount: u32,
  /// Length: aspectReferenceCount
  pub pAspectReferences: *const VkInputAttachmentAspectReference,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Send for VkRenderPassInputAttachmentAspectCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Sync for VkRenderPassInputAttachmentAspectCreateInfo<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_1",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassCreateInfo<'root>>
  for VkRenderPassInputAttachmentAspectCreateInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
impl<'a> VkRenderPassInputAttachmentAspectCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO,
    pNext: core::ptr::null(),
    aspectReferenceCount: 0,
    pAspectReferences: core::ptr::null(),
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
  pub const fn with_aspectReferenceCount(mut self, val: u32) -> Self {
    self.aspectReferenceCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAspectReferences(
    mut self,
    val: &'a [VkInputAttachmentAspectReference],
  ) -> Self {
    self.aspectReferenceCount = val.len() as u32;
    self.pAspectReferences = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassCreateInfo<
    'root,
    T: VkPNextExtends<VkRenderPassCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDevicePointClippingProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePointClippingProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDevicePointClippingProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Exact]
  pub pointClippingBehavior: VkPointClippingBehavior,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDevicePointClippingProperties<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDevicePointClippingProperties<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDevicePointClippingProperties<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
impl<'a> VkPhysicalDevicePointClippingProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES,
    pNext: core::ptr::null_mut(),
    pointClippingBehavior: VkPointClippingBehavior(0),
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
  pub const fn with_pointClippingBehavior(mut self, val: VkPointClippingBehavior) -> Self {
    self.pointClippingBehavior = val;
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
/// [VkPipelineTessellationDomainOriginStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineTessellationDomainOriginStateCreateInfo.html)
///
/// **Extends:** VkPipelineTessellationStateCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub domainOrigin: VkTessellationDomainOrigin,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Send for VkPipelineTessellationDomainOriginStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Sync for VkPipelineTessellationDomainOriginStateCreateInfo<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_1",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineTessellationStateCreateInfo<'root>>
  for VkPipelineTessellationDomainOriginStateCreateInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
impl<'a> VkPipelineTessellationDomainOriginStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    domainOrigin: VkTessellationDomainOrigin(0),
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
  pub const fn with_domainOrigin(mut self, val: VkTessellationDomainOrigin) -> Self {
    self.domainOrigin = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineTessellationStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineTessellationStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceShaderDrawParametersFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderDrawParametersFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderDrawParametersFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderDrawParameters: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderDrawParametersFeatures<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderDrawParametersFeatures<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderDrawParametersFeatures<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderDrawParametersFeatures<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
impl<'a> VkPhysicalDeviceShaderDrawParametersFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES,
    pNext: core::ptr::null_mut(),
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
/// [VkPhysicalDeviceShaderDrawParameterFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderDrawParameterFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceShaderDrawParameterFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub shaderDrawParameters: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Send for VkPhysicalDeviceShaderDrawParameterFeatures<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
unsafe impl<'a> Sync for VkPhysicalDeviceShaderDrawParameterFeatures<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_1", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceShaderDrawParameterFeatures<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_1", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceShaderDrawParameterFeatures<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
impl<'a> VkPhysicalDeviceShaderDrawParameterFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES,
    pNext: core::ptr::null_mut(),
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
