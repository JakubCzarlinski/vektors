#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_4",
  feature = "VK_EXT_line_rasterization",
  feature = "VK_KHR_line_rasterization"
))]
use crate::enums::VkLineRasterizationMode;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferInheritanceInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineRasterizationStateCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineVertexInputStateCreateInfo;
use core::ffi::c_void;
/// [VkRenderingAreaInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAreaInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderingAreaInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDERING_AREA_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub viewMask: u32,
  /// Optional: true
  pub colorAttachmentCount: u32,
  /// Length: colorAttachmentCount,  No Auto-Validity
  pub pColorAttachmentFormats: *const VkFormat,
  /// No Auto-Validity
  pub depthAttachmentFormat: VkFormat,
  /// No Auto-Validity
  pub stencilAttachmentFormat: VkFormat,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Send for VkRenderingAreaInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Sync for VkRenderingAreaInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
impl<'a> VkRenderingAreaInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RENDERING_AREA_INFO,
    pNext: core::ptr::null(),
    viewMask: 0,
    colorAttachmentCount: 0,
    pColorAttachmentFormats: core::ptr::null(),
    depthAttachmentFormat: VkFormat(0),
    stencilAttachmentFormat: VkFormat(0),
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
  pub const fn with_viewMask(mut self, val: u32) -> Self {
    self.viewMask = val;
    self
  }
  #[inline]
  pub const fn with_colorAttachmentCount(mut self, val: u32) -> Self {
    self.colorAttachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pColorAttachmentFormats(mut self, val: &'a [VkFormat]) -> Self {
    self.colorAttachmentCount = val.len() as u32;
    self.pColorAttachmentFormats = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_depthAttachmentFormat(mut self, val: VkFormat) -> Self {
    self.depthAttachmentFormat = val;
    self
  }
  #[inline]
  pub const fn with_stencilAttachmentFormat(mut self, val: VkFormat) -> Self {
    self.stencilAttachmentFormat = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderingAreaInfo<
    'root,
    T: VkPNextExtends<VkRenderingAreaInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkVertexInputBindingDivisorDescription](https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputBindingDivisorDescription.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkVertexInputBindingDivisorDescription {
  pub binding: u32,
  pub divisor: u32,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl Send for VkVertexInputBindingDivisorDescription {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl Sync for VkVertexInputBindingDivisorDescription {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
impl VkVertexInputBindingDivisorDescription {
  pub const DEFAULT: Self = Self {
    binding: 0,
    divisor: 0,
  };
  #[inline]
  pub const fn new() -> Self {
    Self::DEFAULT
  }
  #[inline]
  pub const fn with_binding(mut self, val: u32) -> Self {
    self.binding = val;
    self
  }
  #[inline]
  pub const fn with_divisor(mut self, val: u32) -> Self {
    self.divisor = val;
    self
  }
}
/// [VkPipelineVertexInputDivisorStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineVertexInputDivisorStateCreateInfo.html)
///
/// **Extends:** VkPipelineVertexInputStateCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineVertexInputDivisorStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub vertexBindingDivisorCount: u32,
  /// Length: vertexBindingDivisorCount
  pub pVertexBindingDivisors: *const VkVertexInputBindingDivisorDescription,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Send for VkPipelineVertexInputDivisorStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Sync for VkPipelineVertexInputDivisorStateCreateInfo<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_4",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineVertexInputStateCreateInfo<'root>>
  for VkPipelineVertexInputDivisorStateCreateInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
impl<'a> VkPipelineVertexInputDivisorStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    vertexBindingDivisorCount: 0,
    pVertexBindingDivisors: core::ptr::null(),
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
  pub const fn with_vertexBindingDivisorCount(mut self, val: u32) -> Self {
    self.vertexBindingDivisorCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pVertexBindingDivisors(
    mut self,
    val: &'a [VkVertexInputBindingDivisorDescription],
  ) -> Self {
    self.vertexBindingDivisorCount = val.len() as u32;
    self.pVertexBindingDivisors = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineVertexInputStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineVertexInputStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceVertexAttributeDivisorProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVertexAttributeDivisorProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVertexAttributeDivisorProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Max]
  pub maxVertexAttribDivisor: u32,
  /// Limit Type: [Max]
  pub supportsNonZeroFirstInstance: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceVertexAttributeDivisorProperties<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceVertexAttributeDivisorProperties<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceVertexAttributeDivisorProperties<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
impl<'a> VkPhysicalDeviceVertexAttributeDivisorProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES,
    pNext: core::ptr::null_mut(),
    maxVertexAttribDivisor: 0,
    supportsNonZeroFirstInstance: 0,
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
  pub const fn with_maxVertexAttribDivisor(mut self, val: u32) -> Self {
    self.maxVertexAttribDivisor = val;
    self
  }
  #[inline]
  pub const fn with_supportsNonZeroFirstInstance(mut self, val: VkBool32) -> Self {
    self.supportsNonZeroFirstInstance = val;
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
/// [VkPhysicalDeviceVertexAttributeDivisorFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVertexAttributeDivisorFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub vertexAttributeInstanceRateDivisor: VkBool32,
  pub vertexAttributeInstanceRateZeroDivisor: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceVertexAttributeDivisorFeatures<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceVertexAttributeDivisorFeatures<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceVertexAttributeDivisorFeatures<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceVertexAttributeDivisorFeatures<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
impl<'a> VkPhysicalDeviceVertexAttributeDivisorFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES,
    pNext: core::ptr::null_mut(),
    vertexAttributeInstanceRateDivisor: 0,
    vertexAttributeInstanceRateZeroDivisor: 0,
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
  pub const fn with_vertexAttributeInstanceRateDivisor(mut self, val: VkBool32) -> Self {
    self.vertexAttributeInstanceRateDivisor = val;
    self
  }
  #[inline]
  pub const fn with_vertexAttributeInstanceRateZeroDivisor(mut self, val: VkBool32) -> Self {
    self.vertexAttributeInstanceRateZeroDivisor = val;
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
/// [VkPhysicalDeviceLineRasterizationFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLineRasterizationFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceLineRasterizationFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub rectangularLines: VkBool32,
  pub bresenhamLines: VkBool32,
  pub smoothLines: VkBool32,
  pub stippledRectangularLines: VkBool32,
  pub stippledBresenhamLines: VkBool32,
  pub stippledSmoothLines: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceLineRasterizationFeatures<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceLineRasterizationFeatures<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceLineRasterizationFeatures<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceLineRasterizationFeatures<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
impl<'a> VkPhysicalDeviceLineRasterizationFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES,
    pNext: core::ptr::null_mut(),
    rectangularLines: 0,
    bresenhamLines: 0,
    smoothLines: 0,
    stippledRectangularLines: 0,
    stippledBresenhamLines: 0,
    stippledSmoothLines: 0,
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
/// [VkPhysicalDeviceLineRasterizationProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLineRasterizationProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceLineRasterizationProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bits]
  pub lineSubPixelPrecisionBits: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceLineRasterizationProperties<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceLineRasterizationProperties<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceLineRasterizationProperties<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
impl<'a> VkPhysicalDeviceLineRasterizationProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES,
    pNext: core::ptr::null_mut(),
    lineSubPixelPrecisionBits: 0,
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
/// [VkPipelineRasterizationLineStateCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationLineStateCreateInfo.html)
///
/// **Extends:** VkPipelineRasterizationStateCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineRasterizationLineStateCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub lineRasterizationMode: VkLineRasterizationMode,
  pub stippledLineEnable: VkBool32,
  pub lineStippleFactor: u32,
  pub lineStipplePattern: u16,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Send for VkPipelineRasterizationLineStateCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Sync for VkPipelineRasterizationLineStateCreateInfo<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_4",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPipelineRasterizationStateCreateInfo<'root>>
  for VkPipelineRasterizationLineStateCreateInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
impl<'a> VkPipelineRasterizationLineStateCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO,
    pNext: core::ptr::null(),
    lineRasterizationMode: VkLineRasterizationMode(0),
    stippledLineEnable: 0,
    lineStippleFactor: 0,
    lineStipplePattern: 0,
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
  pub const fn with_lineRasterizationMode(mut self, val: VkLineRasterizationMode) -> Self {
    self.lineRasterizationMode = val;
    self
  }
  #[inline]
  pub const fn with_stippledLineEnable(mut self, val: VkBool32) -> Self {
    self.stippledLineEnable = val;
    self
  }
  #[inline]
  pub const fn with_lineStippleFactor(mut self, val: u32) -> Self {
    self.lineStippleFactor = val;
    self
  }
  #[inline]
  pub const fn with_lineStipplePattern(mut self, val: u16) -> Self {
    self.lineStipplePattern = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkPipelineRasterizationStateCreateInfo<
    'root,
    T: VkPNextExtends<VkPipelineRasterizationStateCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceDynamicRenderingLocalReadFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDynamicRenderingLocalReadFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub dynamicRenderingLocalRead: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Send for VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Sync for VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_4", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
impl<'a> VkPhysicalDeviceDynamicRenderingLocalReadFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES,
    pNext: core::ptr::null_mut(),
    dynamicRenderingLocalRead: 0,
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
  pub const fn with_dynamicRenderingLocalRead(mut self, val: VkBool32) -> Self {
    self.dynamicRenderingLocalRead = val;
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
/// [VkRenderingAttachmentLocationInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentLocationInfo.html)
///
/// **Extends:** VkGraphicsPipelineCreateInfo, VkCommandBufferInheritanceInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderingAttachmentLocationInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_LOCATION_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub colorAttachmentCount: u32,
  /// Length: colorAttachmentCount,  No Auto-Validity
  pub pColorAttachmentLocations: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Send for VkRenderingAttachmentLocationInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Sync for VkRenderingAttachmentLocationInfo<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_4",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkRenderingAttachmentLocationInfo<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkRenderingAttachmentLocationInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
impl<'a> VkRenderingAttachmentLocationInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RENDERING_ATTACHMENT_LOCATION_INFO,
    pNext: core::ptr::null(),
    colorAttachmentCount: 0,
    pColorAttachmentLocations: core::ptr::null(),
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
  pub const fn with_colorAttachmentCount(mut self, val: u32) -> Self {
    self.colorAttachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pColorAttachmentLocations(mut self, val: &'a [u32]) -> Self {
    self.colorAttachmentCount = val.len() as u32;
    self.pColorAttachmentLocations = val.as_ptr();
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
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCommandBufferInheritanceInfo<
    'root,
    T: VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkRenderingInputAttachmentIndexInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingInputAttachmentIndexInfo.html)
///
/// **Extends:** VkGraphicsPipelineCreateInfo, VkCommandBufferInheritanceInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderingInputAttachmentIndexInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDERING_INPUT_ATTACHMENT_INDEX_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub colorAttachmentCount: u32,
  /// Optional: true,  Length: colorAttachmentCount
  pub pColorAttachmentInputIndices: *const u32,
  /// Optional: true
  pub pDepthInputAttachmentIndex: *const u32,
  /// Optional: true
  pub pStencilInputAttachmentIndex: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Send for VkRenderingInputAttachmentIndexInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
unsafe impl<'a> Sync for VkRenderingInputAttachmentIndexInfo<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_4",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkRenderingInputAttachmentIndexInfo<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_4", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkRenderingInputAttachmentIndexInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
impl<'a> VkRenderingInputAttachmentIndexInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RENDERING_INPUT_ATTACHMENT_INDEX_INFO,
    pNext: core::ptr::null(),
    colorAttachmentCount: 0,
    pColorAttachmentInputIndices: core::ptr::null(),
    pDepthInputAttachmentIndex: core::ptr::null(),
    pStencilInputAttachmentIndex: core::ptr::null(),
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
  pub const fn with_colorAttachmentCount(mut self, val: u32) -> Self {
    self.colorAttachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pColorAttachmentInputIndices(mut self, val: &'a [u32]) -> Self {
    self.colorAttachmentCount = val.len() as u32;
    self.pColorAttachmentInputIndices = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDepthInputAttachmentIndex(mut self, val: *const u32) -> Self {
    self.pDepthInputAttachmentIndex = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pStencilInputAttachmentIndex(mut self, val: *const u32) -> Self {
    self.pStencilInputAttachmentIndex = val;
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
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkCommandBufferInheritanceInfo<
    'root,
    T: VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
