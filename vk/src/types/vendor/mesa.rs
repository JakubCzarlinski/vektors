#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
use crate::types::VkPNextExtends;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
use core::ffi::c_void;
/// [VkPhysicalDeviceImageAlignmentControlFeaturesMESA](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageAlignmentControlFeaturesMESA.html)
///
/// **Extends:** VkPhysicalDeviceFeatures2, VkDeviceCreateInfo.
#[cfg(feature = "VK_MESA_image_alignment_control")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  pub imageAlignmentControl: VkBool32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_MESA_image_alignment_control")]
unsafe impl<'a> Send for VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'a> {}
#[cfg(feature = "VK_MESA_image_alignment_control")]
unsafe impl<'a> Sync for VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'a> {}
#[cfg(all(
  feature = "VK_MESA_image_alignment_control",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceFeatures2<'root>>
  for VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'child>
{
}
#[cfg(all(
  feature = "VK_MESA_image_alignment_control",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkDeviceCreateInfo<'root>>
  for VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'child>
{
}
#[cfg(feature = "VK_MESA_image_alignment_control")]
impl<'a> VkPhysicalDeviceImageAlignmentControlFeaturesMESA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA,
    pNext: core::ptr::null_mut(),
    imageAlignmentControl: 0,
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
  pub const fn with_imageAlignmentControl(mut self, val: VkBool32) -> Self {
    self.imageAlignmentControl = val;
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
/// [VkPhysicalDeviceImageAlignmentControlPropertiesMESA](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageAlignmentControlPropertiesMESA.html)
///
/// *Note: This is a **returned only** struct.*
///
/// *Note: This struct has **required limit types**.*
///
/// **Extends:** VkPhysicalDeviceProperties2.
#[cfg(feature = "VK_MESA_image_alignment_control")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkPhysicalDeviceImageAlignmentControlPropertiesMESA<'a> {
  /// Values: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *mut c_void,
  /// Limit Type: [Bitmask]
  pub supportedImageAlignmentMask: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_MESA_image_alignment_control")]
unsafe impl<'a> Send for VkPhysicalDeviceImageAlignmentControlPropertiesMESA<'a> {}
#[cfg(feature = "VK_MESA_image_alignment_control")]
unsafe impl<'a> Sync for VkPhysicalDeviceImageAlignmentControlPropertiesMESA<'a> {}
#[cfg(all(
  feature = "VK_MESA_image_alignment_control",
  feature = "VK_BASE_VERSION_1_1"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkPhysicalDeviceProperties2<'root>>
  for VkPhysicalDeviceImageAlignmentControlPropertiesMESA<'child>
{
}
#[cfg(feature = "VK_MESA_image_alignment_control")]
impl<'a> VkPhysicalDeviceImageAlignmentControlPropertiesMESA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA,
    pNext: core::ptr::null_mut(),
    supportedImageAlignmentMask: 0,
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
  pub const fn with_supportedImageAlignmentMask(mut self, val: u32) -> Self {
    self.supportedImageAlignmentMask = val;
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
/// [VkImageAlignmentControlCreateInfoMESA](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAlignmentControlCreateInfoMESA.html)
///
/// **Extends:** VkImageCreateInfo.
#[cfg(feature = "VK_MESA_image_alignment_control")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkImageAlignmentControlCreateInfoMESA<'a> {
  /// Values: VK_STRUCTURE_TYPE_IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA
  pub sType: VkStructureType,
  /// Optional: true
  pub pNext: *const c_void,
  pub maximumRequestedAlignment: u32,
  #[doc(hidden)]
  pub _marker: core::marker::PhantomData<&'a ()>,
}
#[cfg(feature = "VK_MESA_image_alignment_control")]
unsafe impl<'a> Send for VkImageAlignmentControlCreateInfoMESA<'a> {}
#[cfg(feature = "VK_MESA_image_alignment_control")]
unsafe impl<'a> Sync for VkImageAlignmentControlCreateInfoMESA<'a> {}
#[cfg(all(
  feature = "VK_MESA_image_alignment_control",
  feature = "VK_BASE_VERSION_1_0"
))]
unsafe impl<'child, 'root> VkPNextExtends<VkImageCreateInfo<'root>>
  for VkImageAlignmentControlCreateInfoMESA<'child>
{
}
#[cfg(feature = "VK_MESA_image_alignment_control")]
impl<'a> VkImageAlignmentControlCreateInfoMESA<'a> {
  pub const DEFAULT: Self = Self {
    sType: VkStructureType::IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA,
    pNext: core::ptr::null(),
    maximumRequestedAlignment: 0,
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
  pub const fn with_maximumRequestedAlignment(mut self, val: u32) -> Self {
    self.maximumRequestedAlignment = val;
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
}
