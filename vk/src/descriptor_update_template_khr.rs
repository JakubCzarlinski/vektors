#![allow(
  non_snake_case,
  unused_imports,
  clippy::too_many_arguments,
  clippy::missing_safety_doc
)]
#[cfg(all(
  feature = "VK_KHR_descriptor_update_template",
  not(feature = "VKSC_VERSION_1_0")
))]
use crate::commands::PFN_vkDestroyDescriptorUpdateTemplateKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkResult;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkAllocationCallbacks;
#[cfg(all(
  feature = "VK_KHR_descriptor_update_template",
  not(feature = "VKSC_VERSION_1_0")
))]
use crate::types::VkDescriptorUpdateTemplateKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDevice;
use core::ffi::{c_char, c_void};
#[cfg(all(
  feature = "VK_KHR_descriptor_update_template",
  not(feature = "VKSC_VERSION_1_0")
))]
#[derive(Debug, Clone)]
pub struct DescriptorUpdateTemplateKHRDispatchTable {
  #[cfg(all(
    feature = "VK_KHR_descriptor_update_template",
    not(feature = "VKSC_VERSION_1_0")
  ))]
  pub vkDestroyDescriptorUpdateTemplateKHR: Option<PFN_vkDestroyDescriptorUpdateTemplateKHR>,
}
#[cfg(all(
  feature = "VK_KHR_descriptor_update_template",
  not(feature = "VKSC_VERSION_1_0")
))]
impl DescriptorUpdateTemplateKHRDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(all(
      feature = "VK_KHR_descriptor_update_template",
      not(feature = "VKSC_VERSION_1_0")
    ))]
    vkDestroyDescriptorUpdateTemplateKHR: None,
  };
  #[inline]
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(all(
        feature = "VK_KHR_descriptor_update_template",
        not(feature = "VKSC_VERSION_1_0")
      ))]
      vkDestroyDescriptorUpdateTemplateKHR: loader(
        c"vkDestroyDescriptorUpdateTemplateKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(all(
  feature = "VK_KHR_descriptor_update_template",
  not(feature = "VKSC_VERSION_1_0")
))]
pub struct DescriptorUpdateTemplateKHR<'dev> {
  pub(crate) raw: VkDescriptorUpdateTemplateKHR,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
}
#[cfg(all(
  feature = "VK_KHR_descriptor_update_template",
  not(feature = "VKSC_VERSION_1_0")
))]
unsafe impl<'dev> Send for DescriptorUpdateTemplateKHR<'dev> {}
#[cfg(all(
  feature = "VK_KHR_descriptor_update_template",
  not(feature = "VKSC_VERSION_1_0")
))]
unsafe impl<'dev> Sync for DescriptorUpdateTemplateKHR<'dev> {}
#[cfg(all(
  feature = "VK_KHR_descriptor_update_template",
  not(feature = "VKSC_VERSION_1_0")
))]
#[cfg(not(feature = "VKSC_VERSION_1_0"))]
impl<'dev> Drop for DescriptorUpdateTemplateKHR<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      ((&self.parent.descriptor_update_template_khr_table).vkDestroyDescriptorUpdateTemplateKHR)
        .unwrap_unchecked()(self.parent.raw(), self.raw, core::ptr::null())
    };
  }
}
#[cfg(all(
  feature = "VK_KHR_descriptor_update_template",
  not(feature = "VKSC_VERSION_1_0")
))]
impl<'dev> DescriptorUpdateTemplateKHR<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkDescriptorUpdateTemplateKHR {
    self.raw
  }
  #[inline(always)]
  pub const fn parent(&self) -> &'dev crate::device::Device<'dev> {
    self.parent
  }
  #[inline(always)]
  pub const fn device(&self) -> &'dev crate::device::Device<'dev> {
    self.parent
  }
  #[inline(always)]
  pub const fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
    self.parent.instance()
  }
  #[inline(always)]
  pub const fn table(&self) -> &DescriptorUpdateTemplateKHRDispatchTable {
    &self.parent.descriptor_update_template_khr_table
  }
  /// [`vkDestroyDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html)
  ///
  /// Provided by:
  /// - `VK_KHR_descriptor_update_template`
  ///
  /// - **Removed by:** `VKSC_VERSION_1_0`
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `descriptorUpdateTemplate`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(all(
    feature = "VK_KHR_descriptor_update_template",
    not(feature = "VKSC_VERSION_1_0")
  ))]
  #[inline(always)]
  pub fn vkDestroyDescriptorUpdateTemplateKHR(
    &mut self,
    pAllocator: *const VkAllocationCallbacks<'_>,
  ) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.parent.descriptor_update_template_khr_table)
        .vkDestroyDescriptorUpdateTemplateKHR
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkDescriptorUpdateTemplateKHR::NULL;
  }
}
