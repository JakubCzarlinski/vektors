#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkAccessFlagBits;
#[cfg(any(feature = "VK_GRAPHICS_VERSION_1_0", feature = "VK_KHR_maintenance10"))]
use crate::enums::VkAttachmentDescriptionFlagBits;
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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkDependencyFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageAspectFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageLayout;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageUsageFlagBits;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_AMDX_shader_enqueue",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_ray_tracing",
  feature = "VK_HUAWEI_subpass_shading"
))]
use crate::enums::VkPipelineBindPoint;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkPipelineStageFlagBits;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_QCOM_render_pass_transform",
  feature = "VK_VALVE_fragment_density_map_layered"
))]
use crate::enums::VkRenderPassCreateFlagBits;
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
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_EXT_nested_command_buffer"
))]
use crate::enums::VkSubpassContents;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_NVX_multiview_per_view_attributes",
  feature = "VK_QCOM_render_pass_shader_resolve",
  feature = "VK_QCOM_tile_shading",
  feature = "VK_ARM_rasterization_order_attachment_access",
  feature = "VK_EXT_rasterization_order_attachment_access",
  feature = "VK_EXT_legacy_dithering",
  feature = "VK_EXT_custom_resolve"
))]
use crate::enums::VkSubpassDescriptionFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkAccessFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkAttachmentDescriptionFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDependencyFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkExternalFormatANDROID;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkExternalFormatOHOS;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::types::VkFragmentShadingRateAttachmentInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkFramebufferCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageAspectFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageUsageFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageView;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkMemoryBarrier2;
#[cfg(feature = "VK_KHR_maintenance8")]
use crate::types::VkMemoryBarrierAccessFlags3KHR;
#[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
use crate::types::VkMultisampledRenderToSingleSampledInfoEXT;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceImageFormatInfo2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPipelineStageFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPassBeginInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPassCreateFlags;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::types::VkRenderPassCreationControlEXT;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::types::VkRenderPassCreationFeedbackCreateInfoEXT;
#[cfg(feature = "VK_EXT_fragment_density_map")]
use crate::types::VkRenderPassFragmentDensityMapCreateInfoEXT;
#[cfg(feature = "VK_EXT_fragment_density_map_offset")]
use crate::types::VkRenderPassFragmentDensityMapOffsetEndInfoEXT;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::types::VkRenderPassSubpassFeedbackCreateInfoEXT;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkRenderPassTileShadingCreateInfoQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkResolveModeFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkSubpassDescriptionFlags;
#[cfg(all(
  feature = "VK_QCOM_tile_memory_heap",
  feature = "VK_QCOM_tile_properties"
))]
use crate::types::VkTileMemorySizeInfoQCOM;
use core::ffi::c_void;
/// [VkAttachmentDescription2](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescription2.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAttachmentDescription2<'a> {
  /// Values: VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkAttachmentDescriptionFlags,
  pub format: VkFormat,
  pub samples: VkSampleCountFlagBits,
  pub loadOp: VkAttachmentLoadOp,
  pub storeOp: VkAttachmentStoreOp,
  pub stencilLoadOp: VkAttachmentLoadOp,
  pub stencilStoreOp: VkAttachmentStoreOp,
  pub initialLayout: VkImageLayout,
  pub finalLayout: VkImageLayout,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkAttachmentDescription2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkAttachmentDescription2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkAttachmentDescription2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ATTACHMENT_DESCRIPTION_2,
    pNext: core::ptr::null(),
    flags: VkAttachmentDescriptionFlagBits(0),
    format: VkFormat(0),
    samples: VkSampleCountFlagBits(0),
    loadOp: VkAttachmentLoadOp(0),
    storeOp: VkAttachmentStoreOp(0),
    stencilLoadOp: VkAttachmentLoadOp(0),
    stencilStoreOp: VkAttachmentStoreOp(0),
    initialLayout: VkImageLayout(0),
    finalLayout: VkImageLayout(0),
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
  pub const fn with_flags(mut self, val: VkAttachmentDescriptionFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_format(mut self, val: VkFormat) -> Self {
    self.format = val;
    self
  }
  #[inline]
  pub const fn with_samples(mut self, val: VkSampleCountFlagBits) -> Self {
    self.samples = val;
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
  pub const fn with_stencilLoadOp(mut self, val: VkAttachmentLoadOp) -> Self {
    self.stencilLoadOp = val;
    self
  }
  #[inline]
  pub const fn with_stencilStoreOp(mut self, val: VkAttachmentStoreOp) -> Self {
    self.stencilStoreOp = val;
    self
  }
  #[inline]
  pub const fn with_initialLayout(mut self, val: VkImageLayout) -> Self {
    self.initialLayout = val;
    self
  }
  #[inline]
  pub const fn with_finalLayout(mut self, val: VkImageLayout) -> Self {
    self.finalLayout = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkAttachmentDescriptionStencilLayout<'child>(
    mut self,
    val: &'a VkAttachmentDescriptionStencilLayout<'child>,
  ) -> Self {
    self.pNext = (val as *const VkAttachmentDescriptionStencilLayout<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalFormatANDROID<'child>(
    mut self,
    val: &'a VkExternalFormatANDROID<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalFormatANDROID<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_OHOS_external_memory")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkExternalFormatOHOS<'child>(
    mut self,
    val: &'a VkExternalFormatOHOS<'child>,
  ) -> Self {
    self.pNext = (val as *const VkExternalFormatOHOS<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAttachmentDescription2<
    'root,
    T: VkPNextExtends<VkAttachmentDescription2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkAttachmentReference2](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentReference2.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAttachmentReference2<'a> {
  /// Values: VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub attachment: u32,
  pub layout: VkImageLayout,
  /// No Auto-Validity
  pub aspectMask: VkImageAspectFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkAttachmentReference2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkAttachmentReference2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkAttachmentReference2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ATTACHMENT_REFERENCE_2,
    pNext: core::ptr::null(),
    attachment: 0,
    layout: VkImageLayout(0),
    aspectMask: VkImageAspectFlagBits(0),
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
  pub const fn with_attachment(mut self, val: u32) -> Self {
    self.attachment = val;
    self
  }
  #[inline]
  pub const fn with_layout(mut self, val: VkImageLayout) -> Self {
    self.layout = val;
    self
  }
  #[inline]
  pub const fn with_aspectMask(mut self, val: VkImageAspectFlags) -> Self {
    self.aspectMask = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkAttachmentReferenceStencilLayout<'child>(
    mut self,
    val: &'a VkAttachmentReferenceStencilLayout<'child>,
  ) -> Self {
    self.pNext = (val as *const VkAttachmentReferenceStencilLayout<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAttachmentReference2<
    'root,
    T: VkPNextExtends<VkAttachmentReference2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSubpassDescription2](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescription2.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubpassDescription2<'a> {
  /// Values: VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkSubpassDescriptionFlags,
  pub pipelineBindPoint: VkPipelineBindPoint,
  pub viewMask: u32,
  /// Optional: true
  pub inputAttachmentCount: u32,
  /// Length: inputAttachmentCount
  pub pInputAttachments: *const VkAttachmentReference2<'a>,
  /// Optional: true
  pub colorAttachmentCount: u32,
  /// Length: colorAttachmentCount
  pub pColorAttachments: *const VkAttachmentReference2<'a>,
  /// Optional: true,  Length: colorAttachmentCount
  pub pResolveAttachments: *const VkAttachmentReference2<'a>,
  /// Optional: true
  pub pDepthStencilAttachment: *const VkAttachmentReference2<'a>,
  /// Optional: true
  pub preserveAttachmentCount: u32,
  /// Length: preserveAttachmentCount
  pub pPreserveAttachments: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkSubpassDescription2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkSubpassDescription2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkSubpassDescription2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SUBPASS_DESCRIPTION_2,
    pNext: core::ptr::null(),
    flags: VkSubpassDescriptionFlagBits(0),
    pipelineBindPoint: VkPipelineBindPoint(0),
    viewMask: 0,
    inputAttachmentCount: 0,
    pInputAttachments: core::ptr::null(),
    colorAttachmentCount: 0,
    pColorAttachments: core::ptr::null(),
    pResolveAttachments: core::ptr::null(),
    pDepthStencilAttachment: core::ptr::null(),
    preserveAttachmentCount: 0,
    pPreserveAttachments: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkSubpassDescriptionFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_pipelineBindPoint(mut self, val: VkPipelineBindPoint) -> Self {
    self.pipelineBindPoint = val;
    self
  }
  #[inline]
  pub const fn with_viewMask(mut self, val: u32) -> Self {
    self.viewMask = val;
    self
  }
  #[inline]
  pub const fn with_inputAttachmentCount(mut self, val: u32) -> Self {
    self.inputAttachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pInputAttachments(mut self, val: &'a [VkAttachmentReference2<'a>]) -> Self {
    self.inputAttachmentCount = val.len() as u32;
    self.pInputAttachments = val.as_ptr();
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
  pub const fn with_pColorAttachments(mut self, val: &'a [VkAttachmentReference2<'a>]) -> Self {
    self.colorAttachmentCount = val.len() as u32;
    self.pColorAttachments = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pResolveAttachments(mut self, val: &'a [VkAttachmentReference2<'a>]) -> Self {
    self.colorAttachmentCount = val.len() as u32;
    self.pResolveAttachments = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDepthStencilAttachment(
    mut self,
    val: *const VkAttachmentReference2<'a>,
  ) -> Self {
    self.pDepthStencilAttachment = val;
    self
  }
  #[inline]
  pub const fn with_preserveAttachmentCount(mut self, val: u32) -> Self {
    self.preserveAttachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pPreserveAttachments(mut self, val: &'a [u32]) -> Self {
    self.preserveAttachmentCount = val.len() as u32;
    self.pPreserveAttachments = val.as_ptr();
    self
  }
  /// # Safety
  /// The caller must ensure every provided array constrained by `colorAttachmentCount` has the same length. Optional pointer arguments may be null, but non-null pointers must be valid for that same length and outlive any use of this struct instance.
  #[inline]
  pub const fn with_colorAttachmentCount_slices(
    mut self,
    pColorAttachments: &'a [VkAttachmentReference2<'a>],
    pResolveAttachments: *const VkAttachmentReference2<'a>,
  ) -> Self {
    let len = pColorAttachments.len();
    self.colorAttachmentCount = len as u32;
    self.pColorAttachments = pColorAttachments.as_ptr();
    self.pResolveAttachments = pResolveAttachments;
    self
  }
  #[cfg(feature = "VK_KHR_fragment_shading_rate")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkFragmentShadingRateAttachmentInfoKHR<'child>(
    mut self,
    val: &'a VkFragmentShadingRateAttachmentInfoKHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkFragmentShadingRateAttachmentInfoKHR<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_EXT_subpass_merge_feedback")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassCreationControlEXT<'child>(
    mut self,
    val: &'a VkRenderPassCreationControlEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassCreationControlEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_subpass_merge_feedback")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassSubpassFeedbackCreateInfoEXT<'child>(
    mut self,
    val: &'a VkRenderPassSubpassFeedbackCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassSubpassFeedbackCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkSubpassDescriptionDepthStencilResolve<'child>(
    mut self,
    val: &'a VkSubpassDescriptionDepthStencilResolve<'child>,
  ) -> Self {
    self.pNext = (val as *const VkSubpassDescriptionDepthStencilResolve<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubpassDescription2<
    'root,
    T: VkPNextExtends<VkSubpassDescription2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSubpassDependency2](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDependency2.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubpassDependency2<'a> {
  /// Values: VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub srcSubpass: u32,
  pub dstSubpass: u32,
  /// Optional: true
  pub srcStageMask: VkPipelineStageFlags,
  /// Optional: true
  pub dstStageMask: VkPipelineStageFlags,
  /// Optional: true
  pub srcAccessMask: VkAccessFlags,
  /// Optional: true
  pub dstAccessMask: VkAccessFlags,
  /// Optional: true
  pub dependencyFlags: VkDependencyFlags,
  pub viewOffset: i32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkSubpassDependency2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkSubpassDependency2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkSubpassDependency2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SUBPASS_DEPENDENCY_2,
    pNext: core::ptr::null(),
    srcSubpass: 0,
    dstSubpass: 0,
    srcStageMask: VkPipelineStageFlagBits(0),
    dstStageMask: VkPipelineStageFlagBits(0),
    srcAccessMask: VkAccessFlagBits(0),
    dstAccessMask: VkAccessFlagBits(0),
    dependencyFlags: VkDependencyFlagBits(0),
    viewOffset: 0,
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
  pub const fn with_srcSubpass(mut self, val: u32) -> Self {
    self.srcSubpass = val;
    self
  }
  #[inline]
  pub const fn with_dstSubpass(mut self, val: u32) -> Self {
    self.dstSubpass = val;
    self
  }
  #[inline]
  pub const fn with_srcStageMask(mut self, val: VkPipelineStageFlags) -> Self {
    self.srcStageMask = val;
    self
  }
  #[inline]
  pub const fn with_dstStageMask(mut self, val: VkPipelineStageFlags) -> Self {
    self.dstStageMask = val;
    self
  }
  #[inline]
  pub const fn with_srcAccessMask(mut self, val: VkAccessFlags) -> Self {
    self.srcAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_dstAccessMask(mut self, val: VkAccessFlags) -> Self {
    self.dstAccessMask = val;
    self
  }
  #[inline]
  pub const fn with_dependencyFlags(mut self, val: VkDependencyFlags) -> Self {
    self.dependencyFlags = val;
    self
  }
  #[inline]
  pub const fn with_viewOffset(mut self, val: i32) -> Self {
    self.viewOffset = val;
    self
  }
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryBarrier2<'child>(
    mut self,
    val: &'a VkMemoryBarrier2<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMemoryBarrier2<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_KHR_maintenance8")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkMemoryBarrierAccessFlags3KHR<'child>(
    mut self,
    val: &'a VkMemoryBarrierAccessFlags3KHR<'child>,
  ) -> Self {
    self.pNext = (val as *const VkMemoryBarrierAccessFlags3KHR<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubpassDependency2<
    'root,
    T: VkPNextExtends<VkSubpassDependency2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkRenderPassCreateInfo2](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateInfo2.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassCreateInfo2<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkRenderPassCreateFlags,
  /// Optional: true
  pub attachmentCount: u32,
  /// Length: attachmentCount
  pub pAttachments: *const VkAttachmentDescription2<'a>,
  pub subpassCount: u32,
  /// Length: subpassCount
  pub pSubpasses: *const VkSubpassDescription2<'a>,
  /// Optional: true
  pub dependencyCount: u32,
  /// Length: dependencyCount
  pub pDependencies: *const VkSubpassDependency2<'a>,
  /// Optional: true
  pub correlatedViewMaskCount: u32,
  /// Length: correlatedViewMaskCount
  pub pCorrelatedViewMasks: *const u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkRenderPassCreateInfo2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkRenderPassCreateInfo2<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkRenderPassCreateInfo2<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RENDER_PASS_CREATE_INFO_2,
    pNext: core::ptr::null(),
    flags: VkRenderPassCreateFlagBits(0),
    attachmentCount: 0,
    pAttachments: core::ptr::null(),
    subpassCount: 0,
    pSubpasses: core::ptr::null(),
    dependencyCount: 0,
    pDependencies: core::ptr::null(),
    correlatedViewMaskCount: 0,
    pCorrelatedViewMasks: core::ptr::null(),
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
  pub const fn with_flags(mut self, val: VkRenderPassCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_attachmentCount(mut self, val: u32) -> Self {
    self.attachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAttachments(mut self, val: &'a [VkAttachmentDescription2<'a>]) -> Self {
    self.attachmentCount = val.len() as u32;
    self.pAttachments = val.as_ptr();
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
  pub const fn with_pSubpasses(mut self, val: &'a [VkSubpassDescription2<'a>]) -> Self {
    self.subpassCount = val.len() as u32;
    self.pSubpasses = val.as_ptr();
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
  pub const fn with_pDependencies(mut self, val: &'a [VkSubpassDependency2<'a>]) -> Self {
    self.dependencyCount = val.len() as u32;
    self.pDependencies = val.as_ptr();
    self
  }
  #[inline]
  pub const fn with_correlatedViewMaskCount(mut self, val: u32) -> Self {
    self.correlatedViewMaskCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pCorrelatedViewMasks(mut self, val: &'a [u32]) -> Self {
    self.correlatedViewMaskCount = val.len() as u32;
    self.pCorrelatedViewMasks = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_EXT_subpass_merge_feedback")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassCreationControlEXT<'child>(
    mut self,
    val: &'a VkRenderPassCreationControlEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassCreationControlEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_subpass_merge_feedback")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassCreationFeedbackCreateInfoEXT<'child>(
    mut self,
    val: &'a VkRenderPassCreationFeedbackCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext = (val as *const VkRenderPassCreationFeedbackCreateInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_EXT_fragment_density_map")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassFragmentDensityMapCreateInfoEXT<'child>(
    mut self,
    val: &'a VkRenderPassFragmentDensityMapCreateInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkRenderPassFragmentDensityMapCreateInfoEXT<'child>).cast::<c_void>();
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
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkRenderPassCreateInfo2<
    'root,
    T: VkPNextExtends<VkRenderPassCreateInfo2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSubpassBeginInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassBeginInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubpassBeginInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub contents: VkSubpassContents,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkSubpassBeginInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkSubpassBeginInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkSubpassBeginInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SUBPASS_BEGIN_INFO,
    pNext: core::ptr::null(),
    contents: VkSubpassContents(0),
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
  pub const fn with_contents(mut self, val: VkSubpassContents) -> Self {
    self.contents = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubpassBeginInfo<
    'root,
    T: VkPNextExtends<VkSubpassBeginInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkSubpassEndInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassEndInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubpassEndInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_SUBPASS_END_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkSubpassEndInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkSubpassEndInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkSubpassEndInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SUBPASS_END_INFO,
    pNext: core::ptr::null(),
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
  #[cfg(feature = "VK_EXT_fragment_density_map_offset")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_VkRenderPassFragmentDensityMapOffsetEndInfoEXT<'child>(
    mut self,
    val: &'a VkRenderPassFragmentDensityMapOffsetEndInfoEXT<'child>,
  ) -> Self {
    self.pNext =
      (val as *const VkRenderPassFragmentDensityMapOffsetEndInfoEXT<'child>).cast::<c_void>();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubpassEndInfo<
    'root,
    T: VkPNextExtends<VkSubpassEndInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkPhysicalDeviceDepthStencilResolveProperties](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDepthStencilResolveProperties.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub supportedDepthResolveModes: VkResolveModeFlags,
  /// Limit Type: [Bitmask]
  pub supportedStencilResolveModes: VkResolveModeFlags,
  /// Limit Type: [Max]
  pub independentResolveNone: VkBool32,
  /// Limit Type: [Max]
  pub independentResolve: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceDepthStencilResolveProperties<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceDepthStencilResolveProperties<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceDepthStencilResolveProperties<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkPhysicalDeviceDepthStencilResolveProperties<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES,
    pNext: core::ptr::null_mut(),
    supportedDepthResolveModes: VkResolveModeFlagBits(0),
    supportedStencilResolveModes: VkResolveModeFlagBits(0),
    independentResolveNone: 0,
    independentResolve: 0,
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
/// [VkSubpassDescriptionDepthStencilResolve](https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionDepthStencilResolve.html)
///
/// **Extends:** VkSubpassDescription2.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkSubpassDescriptionDepthStencilResolve<'a> {
  /// Values: VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// No Auto-Validity
  pub depthResolveMode: VkResolveModeFlagBits,
  /// No Auto-Validity
  pub stencilResolveMode: VkResolveModeFlagBits,
  /// Optional: true
  pub pDepthStencilResolveAttachment: *const VkAttachmentReference2<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkSubpassDescriptionDepthStencilResolve<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkSubpassDescriptionDepthStencilResolve<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_2",
  feature = "VK_GRAPHICS_VERSION_1_2"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkSubpassDescription2<'root>>
  for VkSubpassDescriptionDepthStencilResolve<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkSubpassDescriptionDepthStencilResolve<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE,
    pNext: core::ptr::null(),
    depthResolveMode: VkResolveModeFlagBits(0),
    stencilResolveMode: VkResolveModeFlagBits(0),
    pDepthStencilResolveAttachment: core::ptr::null(),
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
  pub const fn with_depthResolveMode(mut self, val: VkResolveModeFlagBits) -> Self {
    self.depthResolveMode = val;
    self
  }
  #[inline]
  pub const fn with_stencilResolveMode(mut self, val: VkResolveModeFlagBits) -> Self {
    self.stencilResolveMode = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pDepthStencilResolveAttachment(
    mut self,
    val: *const VkAttachmentReference2<'a>,
  ) -> Self {
    self.pDepthStencilResolveAttachment = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkSubpassDescription2<
    'root,
    T: VkPNextExtends<VkSubpassDescription2<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkImageStencilUsageCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageStencilUsageCreateInfo.html)
///
/// **Extends:** VkImageCreateInfo, VkPhysicalDeviceImageFormatInfo2.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageStencilUsageCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub stencilUsage: VkImageUsageFlags,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkImageStencilUsageCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkImageStencilUsageCreateInfo<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkImageStencilUsageCreateInfo<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceImageFormatInfo2<'root>>
  for VkImageStencilUsageCreateInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkImageStencilUsageCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_STENCIL_USAGE_CREATE_INFO,
    pNext: core::ptr::null(),
    stencilUsage: VkImageUsageFlagBits(0),
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
  pub const fn with_stencilUsage(mut self, val: VkImageUsageFlags) -> Self {
    self.stencilUsage = val;
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
/// [VkPhysicalDeviceImagelessFramebufferFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImagelessFramebufferFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub imagelessFramebuffer: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceImagelessFramebufferFeatures<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceImagelessFramebufferFeatures<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceImagelessFramebufferFeatures<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceImagelessFramebufferFeatures<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkPhysicalDeviceImagelessFramebufferFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES,
    pNext: core::ptr::null_mut(),
    imagelessFramebuffer: 0,
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
  pub const fn with_imagelessFramebuffer(mut self, val: VkBool32) -> Self {
    self.imagelessFramebuffer = val;
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
/// [VkFramebufferAttachmentsCreateInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferAttachmentsCreateInfo.html)
///
/// **Extends:** VkFramebufferCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFramebufferAttachmentsCreateInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub attachmentImageInfoCount: u32,
  /// Length: attachmentImageInfoCount
  pub pAttachmentImageInfos: *const VkFramebufferAttachmentImageInfo<'a>,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkFramebufferAttachmentsCreateInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkFramebufferAttachmentsCreateInfo<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_2",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkFramebufferCreateInfo<'root>>
  for VkFramebufferAttachmentsCreateInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkFramebufferAttachmentsCreateInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO,
    pNext: core::ptr::null(),
    attachmentImageInfoCount: 0,
    pAttachmentImageInfos: core::ptr::null(),
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
  pub const fn with_attachmentImageInfoCount(mut self, val: u32) -> Self {
    self.attachmentImageInfoCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAttachmentImageInfos(
    mut self,
    val: &'a [VkFramebufferAttachmentImageInfo<'a>],
  ) -> Self {
    self.attachmentImageInfoCount = val.len() as u32;
    self.pAttachmentImageInfos = val.as_ptr();
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkFramebufferCreateInfo<
    'root,
    T: VkPNextExtends<VkFramebufferCreateInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkFramebufferAttachmentImageInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferAttachmentImageInfo.html)
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkFramebufferAttachmentImageInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub flags: VkImageCreateFlags,
  pub usage: VkImageUsageFlags,
  pub width: u32,
  pub height: u32,
  pub layerCount: u32,
  /// Optional: true
  pub viewFormatCount: u32,
  /// Length: viewFormatCount
  pub pViewFormats: *const VkFormat,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkFramebufferAttachmentImageInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkFramebufferAttachmentImageInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkFramebufferAttachmentImageInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO,
    pNext: core::ptr::null(),
    flags: VkImageCreateFlagBits(0),
    usage: VkImageUsageFlagBits(0),
    width: 0,
    height: 0,
    layerCount: 0,
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
  pub const fn with_flags(mut self, val: VkImageCreateFlags) -> Self {
    self.flags = val;
    self
  }
  #[inline]
  pub const fn with_usage(mut self, val: VkImageUsageFlags) -> Self {
    self.usage = val;
    self
  }
  #[inline]
  pub const fn with_width(mut self, val: u32) -> Self {
    self.width = val;
    self
  }
  #[inline]
  pub const fn with_height(mut self, val: u32) -> Self {
    self.height = val;
    self
  }
  #[inline]
  pub const fn with_layerCount(mut self, val: u32) -> Self {
    self.layerCount = val;
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
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkFramebufferAttachmentImageInfo<
    'root,
    T: VkPNextExtends<VkFramebufferAttachmentImageInfo<'root>>,
  >(
    mut self,
    val: &'a T,
  ) -> Self {
    self.pNext = (val as *const T).cast::<c_void>();
    self
  }
}
/// [VkRenderPassAttachmentBeginInfo](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassAttachmentBeginInfo.html)
///
/// **Extends:** VkRenderPassBeginInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkRenderPassAttachmentBeginInfo<'a> {
  /// Values: VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  /// Optional: true
  pub attachmentCount: u32,
  /// Length: attachmentCount
  pub pAttachments: *const VkImageView,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkRenderPassAttachmentBeginInfo<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkRenderPassAttachmentBeginInfo<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_2",
  feature = "VK_GRAPHICS_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkRenderPassBeginInfo<'root>>
  for VkRenderPassAttachmentBeginInfo<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkRenderPassAttachmentBeginInfo<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::RENDER_PASS_ATTACHMENT_BEGIN_INFO,
    pNext: core::ptr::null(),
    attachmentCount: 0,
    pAttachments: core::ptr::null(),
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
  pub const fn with_attachmentCount(mut self, val: u32) -> Self {
    self.attachmentCount = val;
    self
  }
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pAttachments(mut self, val: &'a [VkImageView]) -> Self {
    self.attachmentCount = val.len() as u32;
    self.pAttachments = val.as_ptr();
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
}
/// [VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub separateDepthStencilLayouts: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a> {}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_2", feature = "VK_BASE_VERSION_1_1"))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'child>
{
}
#[cfg(all(feature = "VK_GRAPHICS_VERSION_1_2", feature = "VK_BASE_VERSION_1_0"))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES,
    pNext: core::ptr::null_mut(),
    separateDepthStencilLayouts: 0,
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
  pub const fn with_separateDepthStencilLayouts(mut self, val: VkBool32) -> Self {
    self.separateDepthStencilLayouts = val;
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
/// [VkAttachmentReferenceStencilLayout](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentReferenceStencilLayout.html)
///
/// **Extends:** VkAttachmentReference2.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAttachmentReferenceStencilLayout<'a> {
  /// Values: VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub stencilLayout: VkImageLayout,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkAttachmentReferenceStencilLayout<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkAttachmentReferenceStencilLayout<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_2",
  feature = "VK_GRAPHICS_VERSION_1_2"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAttachmentReference2<'root>>
  for VkAttachmentReferenceStencilLayout<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkAttachmentReferenceStencilLayout<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ATTACHMENT_REFERENCE_STENCIL_LAYOUT,
    pNext: core::ptr::null_mut(),
    stencilLayout: VkImageLayout(0),
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
  pub const fn with_stencilLayout(mut self, val: VkImageLayout) -> Self {
    self.stencilLayout = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAttachmentReference2<
    'root,
    T: VkPNextExtends<VkAttachmentReference2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
/// [VkAttachmentDescriptionStencilLayout](https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionStencilLayout.html)
///
/// **Extends:** VkAttachmentDescription2.
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAttachmentDescriptionStencilLayout<'a> {
  /// Values: VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub stencilInitialLayout: VkImageLayout,
  pub stencilFinalLayout: VkImageLayout,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Send for VkAttachmentDescriptionStencilLayout<'a> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
unsafe impl<'a> Sync for VkAttachmentDescriptionStencilLayout<'a> {}
#[cfg(all(
  feature = "VK_GRAPHICS_VERSION_1_2",
  feature = "VK_GRAPHICS_VERSION_1_2"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkAttachmentDescription2<'root>>
  for VkAttachmentDescriptionStencilLayout<'child>
{
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
impl<'a> VkAttachmentDescriptionStencilLayout<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT,
    pNext: core::ptr::null_mut(),
    stencilInitialLayout: VkImageLayout(0),
    stencilFinalLayout: VkImageLayout(0),
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
  pub const fn with_stencilInitialLayout(mut self, val: VkImageLayout) -> Self {
    self.stencilInitialLayout = val;
    self
  }
  #[inline]
  pub const fn with_stencilFinalLayout(mut self, val: VkImageLayout) -> Self {
    self.stencilFinalLayout = val;
    self
  }
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  /// # Safety
  /// The caller must ensure `val` remains valid and outlives any use of this struct
  /// instance. The pointer is stored as-is without any lifetime tracking.
  #[inline]
  pub const fn with_pNext_chain_VkAttachmentDescription2<
    'root,
    T: VkPNextExtends<VkAttachmentDescription2<'root>>,
  >(
    mut self,
    val: &'a mut T,
  ) -> Self {
    self.pNext = (val as *mut T).cast::<c_void>();
    self
  }
}
