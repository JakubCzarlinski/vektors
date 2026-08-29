#![allow(
  non_snake_case,
  unused_imports,
  clippy::too_many_arguments,
  clippy::missing_safety_doc
)]
#[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
use crate::commands::PFN_vkDestroyShaderModule;
#[cfg(feature = "VK_EXT_shader_module_identifier")]
use crate::commands::PFN_vkGetShaderModuleIdentifierEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkResult;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkAllocationCallbacks;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDevice;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkShaderModule;
#[cfg(feature = "VK_EXT_shader_module_identifier")]
use crate::types::VkShaderModuleIdentifierEXT;
use core::ffi::{c_char, c_void};
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct ShaderModuleDispatchTable {
  #[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
  pub vkDestroyShaderModule: Option<PFN_vkDestroyShaderModule>,
  #[cfg(feature = "VK_EXT_shader_module_identifier")]
  pub vkGetShaderModuleIdentifierEXT: Option<PFN_vkGetShaderModuleIdentifierEXT>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl ShaderModuleDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
    vkDestroyShaderModule: None,
    #[cfg(feature = "VK_EXT_shader_module_identifier")]
    vkGetShaderModuleIdentifierEXT: None,
  };
  #[inline]
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
      vkDestroyShaderModule: loader(c"vkDestroyShaderModule".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_shader_module_identifier")]
      vkGetShaderModuleIdentifierEXT: loader(c"vkGetShaderModuleIdentifierEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct ShaderModule<'dev> {
  pub(crate) raw: VkShaderModule,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Send for ShaderModule<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Sync for ShaderModule<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[cfg(not(feature = "VKSC_VERSION_1_0"))]
impl<'dev> Drop for ShaderModule<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      ((&self.parent.shader_module_table).vkDestroyShaderModule).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> ShaderModule<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkShaderModule {
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
  pub const fn table(&self) -> &ShaderModuleDispatchTable {
    &self.parent.shader_module_table
  }
  /// [`vkDestroyShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Removed by:** `VKSC_VERSION_1_0`
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `shaderModule`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
  #[inline(always)]
  pub fn vkDestroyShaderModule(&mut self, pAllocator: *const VkAllocationCallbacks<'_>) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.parent.shader_module_table)
        .vkDestroyShaderModule
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkShaderModule::NULL;
  }
  /// [`vkGetShaderModuleIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_shader_module_identifier`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `shaderModule`
  /// - `pIdentifier`
  #[cfg(feature = "VK_EXT_shader_module_identifier")]
  #[inline(always)]
  pub fn vkGetShaderModuleIdentifierEXT(&self, pIdentifier: &mut VkShaderModuleIdentifierEXT<'_>) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.parent.shader_module_table)
        .vkGetShaderModuleIdentifierEXT
        .unwrap_unchecked()(self.device().raw(), self.raw, pIdentifier)
    }
  }
}
