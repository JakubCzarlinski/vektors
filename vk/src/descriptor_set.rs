#![allow(
  non_snake_case,
  unused_imports,
  clippy::too_many_arguments,
  clippy::missing_safety_doc
)]
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::commands::PFN_vkFreeDescriptorSets;
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
use crate::commands::PFN_vkGetDescriptorSetHostMappingVALVE;
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
use crate::commands::PFN_vkUpdateDescriptorSetWithTemplate;
#[cfg(all(
  feature = "VK_KHR_descriptor_update_template",
  not(feature = "VKSC_VERSION_1_0")
))]
use crate::commands::PFN_vkUpdateDescriptorSetWithTemplateKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkResult;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorPool;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSet;
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
use crate::types::VkDescriptorUpdateTemplate;
#[cfg(all(
  feature = "VK_KHR_descriptor_update_template",
  not(feature = "VKSC_VERSION_1_0")
))]
use crate::types::VkDescriptorUpdateTemplateKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDevice;
use core::ffi::{c_char, c_void};
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct DescriptorSetDispatchTable {
  #[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
  pub vkUpdateDescriptorSetWithTemplate: Option<PFN_vkUpdateDescriptorSetWithTemplate>,
  #[cfg(all(
    feature = "VK_KHR_descriptor_update_template",
    not(feature = "VKSC_VERSION_1_0")
  ))]
  pub vkUpdateDescriptorSetWithTemplateKHR: Option<PFN_vkUpdateDescriptorSetWithTemplateKHR>,
  #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
  pub vkGetDescriptorSetHostMappingVALVE: Option<PFN_vkGetDescriptorSetHostMappingVALVE>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl DescriptorSetDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
    vkUpdateDescriptorSetWithTemplate: None,
    #[cfg(all(
      feature = "VK_KHR_descriptor_update_template",
      not(feature = "VKSC_VERSION_1_0")
    ))]
    vkUpdateDescriptorSetWithTemplateKHR: None,
    #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
    vkGetDescriptorSetHostMappingVALVE: None,
  };
  #[inline]
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
      vkUpdateDescriptorSetWithTemplate: loader(c"vkUpdateDescriptorSetWithTemplate".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(all(
        feature = "VK_KHR_descriptor_update_template",
        not(feature = "VKSC_VERSION_1_0")
      ))]
      vkUpdateDescriptorSetWithTemplateKHR: loader(
        c"vkUpdateDescriptorSetWithTemplateKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
      vkGetDescriptorSetHostMappingVALVE: loader(c"vkGetDescriptorSetHostMappingVALVE".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct DescriptorSet<'dev> {
  pub(crate) raw: VkDescriptorSet,
  pub(crate) parent: &'dev crate::descriptor_pool::DescriptorPool<'dev>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Send for DescriptorSet<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Sync for DescriptorSet<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[cfg(not(feature = "VKSC_VERSION_1_0"))]
impl<'dev> Drop for DescriptorSet<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    if !self.parent.free_descriptor_sets {
      return;
    }
    unsafe {
      (self.parent.table().vkFreeDescriptorSets).unwrap_unchecked()(
        self.device().raw,
        self.parent.raw,
        1,
        &self.raw,
      )
    };
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> DescriptorSet<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkDescriptorSet {
    self.raw
  }
  #[inline(always)]
  pub const fn parent(&self) -> &'dev crate::descriptor_pool::DescriptorPool<'dev> {
    self.parent
  }
  #[inline(always)]
  pub const fn device(&self) -> &'dev crate::device::Device<'dev> {
    self.parent.device()
  }
  #[inline(always)]
  pub const fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
    self.parent.instance()
  }
  #[inline(always)]
  pub const fn table(&self) -> &DescriptorSetDispatchTable {
    &self.device().descriptor_set_table
  }
  /// [`vkUpdateDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_1`
  ///
  /// - **Removed by:** `VKSC_VERSION_1_0`
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `descriptorSet`
  /// - `descriptorUpdateTemplate`
  /// - `pData`
  #[cfg(all(feature = "VK_COMPUTE_VERSION_1_1", not(feature = "VKSC_VERSION_1_0")))]
  #[inline(always)]
  pub fn vkUpdateDescriptorSetWithTemplate(
    &self,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pData: *const c_void,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.device().descriptor_set_table)
        .vkUpdateDescriptorSetWithTemplate
        .unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        descriptorUpdateTemplate,
        pData,
      )
    }
  }
  /// [`vkUpdateDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html)
  ///
  /// Provided by:
  /// - `VK_KHR_descriptor_update_template`
  ///
  /// - **Removed by:** `VKSC_VERSION_1_0`
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `descriptorSet`
  /// - `descriptorUpdateTemplate`
  /// - `pData`
  #[cfg(all(
    feature = "VK_KHR_descriptor_update_template",
    not(feature = "VKSC_VERSION_1_0")
  ))]
  #[inline(always)]
  pub fn vkUpdateDescriptorSetWithTemplateKHR(
    &self,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR,
    pData: *const c_void,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.device().descriptor_set_table)
        .vkUpdateDescriptorSetWithTemplateKHR
        .unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        descriptorUpdateTemplate,
        pData,
      )
    }
  }
  /// [`vkGetDescriptorSetHostMappingVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html)
  ///
  /// Provided by:
  /// - `VK_VALVE_descriptor_set_host_mapping`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `descriptorSet`
  /// - `ppData`
  #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
  #[inline(always)]
  pub fn vkGetDescriptorSetHostMappingVALVE(&self, ppData: &mut *mut c_void) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.device().descriptor_set_table)
        .vkGetDescriptorSetHostMappingVALVE
        .unwrap_unchecked()(self.device().raw(), self.raw, ppData)
    }
  }
}
