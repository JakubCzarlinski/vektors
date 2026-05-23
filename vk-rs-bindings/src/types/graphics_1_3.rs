#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_EXT_load_store_op_none",
  feature = "VK_KHR_load_store_op_none"
))]
use crate::enums::VkAttachmentLoadOp;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_KHR_dynamic_rendering",
  feature = "VK_QCOM_render_pass_store_ops",
  feature = "VK_EXT_load_store_op_none"
))]
use crate::enums::VkAttachmentStoreOp;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_IMG_filter_cubic",
  feature = "VK_EXT_filter_cubic"
))]
use crate::enums::VkFilter;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageLayout;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_3",
  feature = "VK_KHR_dynamic_rendering",
  feature = "VK_EXT_nested_command_buffer",
  feature = "VK_KHR_maintenance7",
  feature = "VK_VALVE_fragment_density_map_layered"
))]
use crate::enums::VkRenderingFlagBits;
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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(any(
  all(
    feature = "VK_EXT_attachment_feedback_loop_layout",
    feature = "VK_KHR_unified_image_layouts",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_EXT_attachment_feedback_loop_layout",
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_KHR_unified_image_layouts"
  )
))]
use crate::types::VkAttachmentFeedbackLoopInfoEXT;
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
use crate::types::VkBlitImageCubicWeightsInfoQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkClearValue;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferInheritanceInfo;
#[cfg(feature = "VK_QCOM_rotated_copy_commands")]
use crate::types::VkCopyCommandTransformInfoQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkDeviceGroupRenderPassBeginInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtent3D;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImage;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageSubresourceLayers;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageView;
#[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
use crate::types::VkMultisampledRenderToSingleSampledInfoEXT;
#[cfg(any(
  all(
    feature = "VK_NVX_multiview_per_view_attributes",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NVX_multiview_per_view_attributes"
  )
))]
use crate::types::VkMultiviewPerViewAttributesInfoNVX;
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
use crate::types::VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkOffset3D;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkRect2D;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkRenderPassPerformanceCountersByRegionBeginInfoARM;
#[cfg(feature = "VK_ARM_render_pass_striped")]
use crate::types::VkRenderPassStripeBeginInfoARM;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkRenderPassTileShadingCreateInfoQCOM;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkRenderingAttachmentFlagsInfoKHR;
#[cfg(any(
  all(feature = "VK_EXT_fragment_density_map", feature = "VK_VERSION_1_3"),
  all(
    feature = "VK_EXT_fragment_density_map",
    feature = "VK_KHR_dynamic_rendering"
  )
))]
use crate::types::VkRenderingFragmentDensityMapAttachmentInfoEXT;
#[cfg(any(
  all(feature = "VK_KHR_fragment_shading_rate", feature = "VK_VERSION_1_3"),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_KHR_fragment_shading_rate"
  )
))]
use crate::types::VkRenderingFragmentShadingRateAttachmentInfoKHR;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkResolveImageModeInfoKHR;
#[cfg(all(
  feature = "VK_QCOM_tile_memory_heap",
  feature = "VK_QCOM_tile_properties"
))]
use crate::types::VkTileMemorySizeInfoQCOM;
use core::ffi::c_void;
/// [VkRenderingFlags](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlags.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type VkRenderingFlags = VkRenderingFlagBits;
/// [VkImageBlit2](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageBlit2.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageBlit2<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_BLIT_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffsets: [VkOffset3D; 2],
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffsets: [VkOffset3D; 2],
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Send for VkImageBlit2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Sync for VkImageBlit2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
impl<'a> VkImageBlit2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_BLIT_2,
    pNext: core::ptr::null(),
    srcSubresource: VkImageSubresourceLayers::DEFAULT,
    srcOffsets: [VkOffset3D::DEFAULT; 2],
    dstSubresource: VkImageSubresourceLayers::DEFAULT,
    dstOffsets: [VkOffset3D::DEFAULT; 2],
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
  pub const fn with_srcSubresource(mut self, val: VkImageSubresourceLayers) -> Self {
    self.srcSubresource = val;
    self
  }
  #[inline]
  pub const fn with_srcOffsets(mut self, val: [VkOffset3D; 2]) -> Self {
    self.srcOffsets = val;
    self
  }
  #[inline]
  pub const fn with_dstSubresource(mut self, val: VkImageSubresourceLayers) -> Self {
    self.dstSubresource = val;
    self
  }
  #[inline]
  pub const fn with_dstOffsets(mut self, val: [VkOffset3D; 2]) -> Self {
    self.dstOffsets = val;
    self
  }
  #[cfg(feature = "VK_QCOM_rotated_copy_commands")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkCopyCommandTransformInfoQCOM<'child>(
    mut self,
    val: &'a VkCopyCommandTransformInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkCopyCommandTransformInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageBlit2<'root, T: VkPNextExtends<VkImageBlit2<'root>>>(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageResolve2](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageResolve2.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageResolve2<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcSubresource: VkImageSubresourceLayers,
  pub srcOffset: VkOffset3D,
  pub dstSubresource: VkImageSubresourceLayers,
  pub dstOffset: VkOffset3D,
  pub extent: VkExtent3D,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Send for VkImageResolve2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Sync for VkImageResolve2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
impl<'a> VkImageResolve2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_RESOLVE_2,
    pNext: core::ptr::null(),
    srcSubresource: VkImageSubresourceLayers::DEFAULT,
    srcOffset: VkOffset3D::DEFAULT,
    dstSubresource: VkImageSubresourceLayers::DEFAULT,
    dstOffset: VkOffset3D::DEFAULT,
    extent: VkExtent3D::DEFAULT,
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
  pub const fn with_srcSubresource(mut self, val: VkImageSubresourceLayers) -> Self {
    self.srcSubresource = val;
    self
  }
  #[inline]
  pub const fn with_srcOffset(mut self, val: VkOffset3D) -> Self {
    self.srcOffset = val;
    self
  }
  #[inline]
  pub const fn with_dstSubresource(mut self, val: VkImageSubresourceLayers) -> Self {
    self.dstSubresource = val;
    self
  }
  #[inline]
  pub const fn with_dstOffset(mut self, val: VkOffset3D) -> Self {
    self.dstOffset = val;
    self
  }
  #[inline]
  pub const fn with_extent(mut self, val: VkExtent3D) -> Self {
    self.extent = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkImageResolve2<
    'root,
    T: VkPNextExtends<VkImageResolve2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkBlitImageInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkBlitImageInfo2.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkBlitImageInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcImage: VkImage,
  pub srcImageLayout: VkImageLayout,
  pub dstImage: VkImage,
  pub dstImageLayout: VkImageLayout,
  pub regionCount: u32,
  /// Length: regionCount
  pub pRegions: *const VkImageBlit2<'a>,
  pub filter: VkFilter,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Send for VkBlitImageInfo2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Sync for VkBlitImageInfo2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
impl<'a> VkBlitImageInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::BLIT_IMAGE_INFO_2,
    pNext: core::ptr::null(),
    srcImage: VkImage::DEFAULT,
    srcImageLayout: VkImageLayout(0),
    dstImage: VkImage::DEFAULT,
    dstImageLayout: VkImageLayout(0),
    regionCount: 0,
    pRegions: core::ptr::null(),
    filter: VkFilter(0),
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
  pub const fn with_pRegions(mut self, val: &'a [VkImageBlit2<'a>]) -> Self {
    self.regionCount = val.len() as u32;
    self.pRegions = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_filter(mut self, val: VkFilter) -> Self {
    self.filter = val;
    self
  }
  #[cfg(feature = "VK_QCOM_filter_cubic_weights")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkBlitImageCubicWeightsInfoQCOM<'child>(
    mut self,
    val: &'a VkBlitImageCubicWeightsInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkBlitImageCubicWeightsInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkBlitImageInfo2<
    'root,
    T: VkPNextExtends<VkBlitImageInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkResolveImageInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveImageInfo2.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkResolveImageInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcImage: VkImage,
  pub srcImageLayout: VkImageLayout,
  pub dstImage: VkImage,
  pub dstImageLayout: VkImageLayout,
  pub regionCount: u32,
  /// Length: regionCount
  pub pRegions: *const VkImageResolve2<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Send for VkResolveImageInfo2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Sync for VkResolveImageInfo2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
impl<'a> VkResolveImageInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RESOLVE_IMAGE_INFO_2,
    pNext: core::ptr::null(),
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
  pub const fn with_pRegions(mut self, val: &'a [VkImageResolve2<'a>]) -> Self {
    self.regionCount = val.len() as u32;
    self.pRegions = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance10")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkResolveImageModeInfoKHR<'child>(
    mut self,
    val: &'a VkResolveImageModeInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkResolveImageModeInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkResolveImageInfo2<
    'root,
    T: VkPNextExtends<VkResolveImageInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPipelineRenderingCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRenderingCreateInfo.html)
///
/// **Extends:** VkGraphicsPipelineCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPipelineRenderingCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO
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
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Send for VkPipelineRenderingCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Sync for VkPipelineRenderingCreateInfo<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_3",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkGraphicsPipelineCreateInfo<'root>>
  for VkPipelineRenderingCreateInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
impl<'a> VkPipelineRenderingCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PIPELINE_RENDERING_CREATE_INFO,
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
}
/// [VkRenderingInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderingInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDERING_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkRenderingFlags,
  pub renderArea: VkRect2D,
  pub layerCount: u32,
  pub viewMask: u32,
  /// Optional: true
  pub colorAttachmentCount: u32,
  /// Length: colorAttachmentCount
  pub pColorAttachments: *const VkRenderingAttachmentInfo<'a>,
  /// Optional: true
  pub pDepthAttachment: *const VkRenderingAttachmentInfo<'a>,
  /// Optional: true
  pub pStencilAttachment: *const VkRenderingAttachmentInfo<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Send for VkRenderingInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Sync for VkRenderingInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
impl<'a> VkRenderingInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RENDERING_INFO,
    pNext: core::ptr::null(),
    flags: VkRenderingFlagBits(0),
    renderArea: VkRect2D::DEFAULT,
    layerCount: 0,
    viewMask: 0,
    colorAttachmentCount: 0,
    pColorAttachments: core::ptr::null(),
    pDepthAttachment: core::ptr::null(),
    pStencilAttachment: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkRenderingFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_renderArea(mut self, val: VkRect2D) -> Self {
    self.renderArea = val;
    self
  }
  #[inline]
  pub const fn with_layerCount(mut self, val: u32) -> Self {
    self.layerCount = val;
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
  pub const fn with_pColorAttachments(mut self, val: &'a [VkRenderingAttachmentInfo<'a>]) -> Self {
    self.colorAttachmentCount = val.len() as u32;
    self.pColorAttachments = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDepthAttachment(mut self, val: *const VkRenderingAttachmentInfo<'a>) -> Self {
    self.pDepthAttachment = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pStencilAttachment(
    mut self,
    val: *const VkRenderingAttachmentInfo<'a>,
  ) -> Self {
    self.pStencilAttachment = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkDeviceGroupRenderPassBeginInfo<'child>(
    mut self,
    val: &'a VkDeviceGroupRenderPassBeginInfo<'child>,
  ) -> Self {
    self.pNext = (val as *const VkDeviceGroupRenderPassBeginInfo<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMultisampledRenderToSingleSampledInfoEXT<'child>(
    mut self,
    val: &'a VkMultisampledRenderToSingleSampledInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkMultisampledRenderToSingleSampledInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(any(
    all(
      feature = "VK_NVX_multiview_per_view_attributes",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_KHR_dynamic_rendering",
      feature = "VK_NVX_multiview_per_view_attributes"
    )
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMultiviewPerViewAttributesInfoNVX<'child>(
    mut self,
    val: &'a VkMultiviewPerViewAttributesInfoNVX<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMultiviewPerViewAttributesInfoNVX<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'child>(
    mut self,
    val: &'a VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_performance_counters_by_region")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassPerformanceCountersByRegionBeginInfoARM<'child>(
    mut self,
    val: &'a VkRenderPassPerformanceCountersByRegionBeginInfoARM<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkRenderPassPerformanceCountersByRegionBeginInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ARM_render_pass_striped")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassStripeBeginInfoARM<'child>(
    mut self,
    val: &'a VkRenderPassStripeBeginInfoARM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassStripeBeginInfoARM<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_QCOM_tile_shading")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassTileShadingCreateInfoQCOM<'child>(
    mut self,
    val: &'a VkRenderPassTileShadingCreateInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassTileShadingCreateInfoQCOM<'child>).cast::<c_void>();
    self
  }
  #[cfg(any(
    all(feature = "VK_EXT_fragment_density_map", feature = "VK_VERSION_1_3"),
    all(
      feature = "VK_EXT_fragment_density_map",
      feature = "VK_KHR_dynamic_rendering"
    )
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderingFragmentDensityMapAttachmentInfoEXT<'child>(
    mut self,
    val: &'a VkRenderingFragmentDensityMapAttachmentInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkRenderingFragmentDensityMapAttachmentInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(any(
    all(feature = "VK_KHR_fragment_shading_rate", feature = "VK_VERSION_1_3"),
    all(
      feature = "VK_KHR_dynamic_rendering",
      feature = "VK_KHR_fragment_shading_rate"
    )
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderingFragmentShadingRateAttachmentInfoKHR<'child>(
    mut self,
    val: &'a VkRenderingFragmentShadingRateAttachmentInfoKHR<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkRenderingFragmentShadingRateAttachmentInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(all(
    feature = "VK_QCOM_tile_memory_heap",
    feature = "VK_QCOM_tile_properties"
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkTileMemorySizeInfoQCOM<'child>(
    mut self,
    val: &'a VkTileMemorySizeInfoQCOM<'child>,
  ) -> Self {
    self.pNext = (val as *const VkTileMemorySizeInfoQCOM<'child>).cast::<c_void>();
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
/// [VkRenderingAttachmentInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderingAttachmentInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub imageView: VkImageView,
  pub imageLayout: VkImageLayout,
  /// Optional: true
  pub resolveMode: VkResolveModeFlagBits,
  /// Optional: true
  pub resolveImageView: VkImageView,
  pub resolveImageLayout: VkImageLayout,
  pub loadOp: VkAttachmentLoadOp,
  pub storeOp: VkAttachmentStoreOp,
  pub clearValue: VkClearValue,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Send for VkRenderingAttachmentInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Sync for VkRenderingAttachmentInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
impl<'a> VkRenderingAttachmentInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RENDERING_ATTACHMENT_INFO,
    pNext: core::ptr::null(),
    imageView: VkImageView::DEFAULT,
    imageLayout: VkImageLayout(0),
    resolveMode: VkResolveModeFlagBits(0),
    resolveImageView: VkImageView::DEFAULT,
    resolveImageLayout: VkImageLayout(0),
    loadOp: VkAttachmentLoadOp(0),
    storeOp: VkAttachmentStoreOp(0),
    clearValue: VkClearValue::DEFAULT,
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
  pub const fn with_imageView(mut self, val: VkImageView) -> Self {
    self.imageView = val;
    self
  }
  #[inline]
  pub const fn with_imageLayout(mut self, val: VkImageLayout) -> Self {
    self.imageLayout = val;
    self
  }
  #[inline]
  pub const fn with_resolveMode(mut self, val: VkResolveModeFlagBits) -> Self {
    self.resolveMode = val;
    self
  }
  #[inline]
  pub const fn with_resolveImageView(mut self, val: VkImageView) -> Self {
    self.resolveImageView = val;
    self
  }
  #[inline]
  pub const fn with_resolveImageLayout(mut self, val: VkImageLayout) -> Self {
    self.resolveImageLayout = val;
    self
  }
  #[inline]
  pub const fn with_loadOp(mut self, val: VkAttachmentLoadOp) -> Self {
    self.loadOp = val;
    self
  }
  #[inline]
  pub const fn with_storeOp(mut self, val: VkAttachmentStoreOp) -> Self {
    self.storeOp = val;
    self
  }
  #[inline]
  pub const fn with_clearValue(mut self, val: VkClearValue) -> Self {
    self.clearValue = val;
    self
  }
  #[cfg(any(
    all(
      feature = "VK_EXT_attachment_feedback_loop_layout",
      feature = "VK_KHR_unified_image_layouts",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_EXT_attachment_feedback_loop_layout",
      feature = "VK_KHR_dynamic_rendering",
      feature = "VK_KHR_unified_image_layouts"
    )
  ))]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkAttachmentFeedbackLoopInfoEXT<'child>(
    mut self,
    val: &'a VkAttachmentFeedbackLoopInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkAttachmentFeedbackLoopInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance10")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderingAttachmentFlagsInfoKHR<'child>(
    mut self,
    val: &'a VkRenderingAttachmentFlagsInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderingAttachmentFlagsInfoKHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderingAttachmentInfo<
    'root,
    T: VkPNextExtends<VkRenderingAttachmentInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceDynamicRenderingFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDynamicRenderingFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDynamicRenderingFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES
  pub sType: VkStructureType,
  /// Optional: true,  No Auto-Validity
  pub pNext: *mut c_void,
  pub dynamicRendering: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Send for VkPhysicalDeviceDynamicRenderingFeatures<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Sync for VkPhysicalDeviceDynamicRenderingFeatures<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_3", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceDynamicRenderingFeatures<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceDynamicRenderingFeatures<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
impl<'a> VkPhysicalDeviceDynamicRenderingFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES,
    pNext: core::ptr::null_mut(),
    dynamicRendering: 0,
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
  pub const fn with_dynamicRendering(mut self, val: VkBool32) -> Self {
    self.dynamicRendering = val;
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
/// [VkCommandBufferInheritanceRenderingInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferInheritanceRenderingInfo.html)
///
/// **Extends:** VkCommandBufferInheritanceInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkCommandBufferInheritanceRenderingInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkRenderingFlags,
  pub viewMask: u32,
  #[cfg(not(feature = "VKSC_VERSION_1_0"))]
  /// Optional: true
  pub colorAttachmentCount: u32,
  #[cfg(feature = "VKSC_VERSION_1_0")]
  pub colorAttachmentCount: u32,
  /// Length: colorAttachmentCount
  pub pColorAttachmentFormats: *const VkFormat,
  pub depthAttachmentFormat: VkFormat,
  pub stencilAttachmentFormat: VkFormat,
  /// Optional: true
  pub rasterizationSamples: VkSampleCountFlagBits,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Send for VkCommandBufferInheritanceRenderingInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
unsafe impl<'a> Sync for VkCommandBufferInheritanceRenderingInfo<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_3", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkCommandBufferInheritanceInfo<'root>>
  for VkCommandBufferInheritanceRenderingInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
impl<'a> VkCommandBufferInheritanceRenderingInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO,
    pNext: core::ptr::null(),
    flags: VkRenderingFlagBits(0),
    viewMask: 0,
    #[cfg(not(feature = "VKSC_VERSION_1_0"))]
    colorAttachmentCount: 0,
    #[cfg(feature = "VKSC_VERSION_1_0")]
    colorAttachmentCount: 0,
    pColorAttachmentFormats: core::ptr::null(),
    depthAttachmentFormat: VkFormat(0),
    stencilAttachmentFormat: VkFormat(0),
    rasterizationSamples: VkSampleCountFlagBits(0),
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
  pub const fn with_flags(mut self, val: VkRenderingFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_viewMask(mut self, val: u32) -> Self {
    self.viewMask = val;
    self
  }
  #[cfg(not(feature = "VKSC_VERSION_1_0"))]
  #[inline]
  pub const fn with_colorAttachmentCount(mut self, val: u32) -> Self {
    self.colorAttachmentCount = val;
    self
  }
  #[cfg(feature = "VKSC_VERSION_1_0")]
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
  #[inline]
  pub const fn with_rasterizationSamples(mut self, val: VkSampleCountFlagBits) -> Self {
    self.rasterizationSamples = val;
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
