//! Layer library loading and interface negotiation.

use crate::{allocation, platform};

use crate::discovery::LayerSource;

use super::{
    CURRENT_LAYER_INTERFACE_VERSION, LayerLoadError, LayerManifest, LoadedLayer, LoaderLibrary,
    NEGOTIATE_INTERFACE_STRUCT, NegotiateLayerInterface, NegotiateLoaderLayerInterfaceVersion,
    OsStr, PFN_vkGetDeviceProcAddr, PFN_vkGetInstanceProcAddr, Path, VkResult, c_void, ptr,
};

impl LoadedLayer {
    pub(crate) fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    pub(crate) fn disable_environment(&self) -> Option<&OsStr> {
        self.disable_environment.as_deref()
    }

    pub(crate) fn enable_environment(&self) -> Option<(&OsStr, &OsStr)> {
        self.enable_environment
            .as_ref()
            .map(|(name, value)| (name.as_os_str(), value.as_os_str()))
    }

    pub(crate) const fn enabled_by(&self) -> &'static str {
        self.enabled_by
    }

    pub(super) fn load(
        manifest: &mut LayerManifest,
        manifest_index: usize,
        enabled_by: &'static str,
    ) -> Result<Self, LayerLoadError> {
        let path = manifest.library_path().ok_or(LayerLoadError::Failed)?;
        // SAFETY: The library is retained for the lifetime of every copied symbol.
        let library = unsafe { LoaderLibrary::open(path) }.map_err(|error| {
            let wrong_bit_type = error.is_wrong_bit_type();
            match error.into_message(path) {
                Ok(message) => LayerLoadError::OpenLibrary {
                    wrong_bit_type,
                    message,
                },
                Err(error) => LayerLoadError::from(error),
            }
        })?;
        let (has_negotiate, negotiated) = negotiate_layer_interface(&library, manifest)?;
        let negotiated_functions = has_negotiate
            && negotiated.loader_layer_interface_version >= CURRENT_LAYER_INTERFACE_VERSION;

        let get_instance_proc_addr = negotiated_functions
            .then_some(negotiated.get_instance_proc_addr)
            .flatten()
            .or_else(|| {
                let name = manifest
                    .functions
                    .get_instance_proc_addr
                    .as_deref()
                    .unwrap_or(c"vkGetInstanceProcAddr");
                // SAFETY: The layer ABI defines this symbol's signature.
                unsafe {
                    library
                        .get::<PFN_vkGetInstanceProcAddr>(name.to_bytes_with_nul())
                        .ok()
                        .map(|symbol| *symbol)
                }
            })
            .ok_or(LayerLoadError::Failed)?;
        let get_device_proc_addr = negotiated_functions
            .then_some(negotiated.get_device_proc_addr)
            .flatten()
            .or_else(|| {
                let name = manifest
                    .functions
                    .get_device_proc_addr
                    .as_deref()
                    .unwrap_or(c"vkGetDeviceProcAddr");
                // SAFETY: The layer ABI defines this symbol's signature.
                unsafe {
                    library
                        .get::<PFN_vkGetDeviceProcAddr>(name.to_bytes_with_nul())
                        .ok()
                        .map(|symbol| *symbol)
                }
            })
            .ok_or(LayerLoadError::Failed)?;
        let get_physical_device_proc_addr = negotiated_functions
            .then_some(negotiated.get_physical_device_proc_addr)
            .flatten();
        let loaded_path =
            platform::loaded_library_path((get_instance_proc_addr as *const ()).cast::<c_void>())?
                .map_or_else(|| allocation::try_path(path), Ok)?;

        // Finish fallible work before moving fields: load-error diagnostics
        // still need the complete manifest if opening or negotiation fails.
        let name = allocation::try_c_string(&manifest.name)?;
        let load_path = match &mut manifest.source {
            LayerSource::Library(path) | LayerSource::OverrideLibrary(path) => {
                core::mem::take(path)
            }
            LayerSource::Meta(_) | LayerSource::OverrideMeta(_) => {
                return Err(LayerLoadError::Failed);
            }
        };
        Ok(Self {
            library: Some(library),
            manifest_index,
            name,
            load_path,
            library_path: loaded_path,
            manifest_path: core::mem::take(&mut manifest.manifest_path),
            enable_environment: manifest.enable_environment.take(),
            disable_environment: manifest.disable_environment.take().map(|(name, _)| name),
            enabled_by,
            implicit: manifest.implicit,
            get_instance_proc_addr,
            get_device_proc_addr,
            get_physical_device_proc_addr,
            device_extensions: core::mem::take(&mut manifest.device_extensions),
        })
    }

    pub(super) fn unload(&mut self) {
        let Some(library) = self.library.take() else {
            return;
        };
        drop(library);
        platform::write_loader_log_with_category(
            platform::LogFilter::Debug,
            platform::LogFilter::Layer,
            format_args!("Unloading layer library {}", self.library_path.display()),
        );
    }
}

impl Drop for LoadedLayer {
    fn drop(&mut self) {
        self.unload();
    }
}

#[cold]
fn negotiate_layer_interface(
    library: &LoaderLibrary,
    manifest: &LayerManifest,
) -> Result<(bool, NegotiateLayerInterface), LayerLoadError> {
    let negotiate_name = manifest
        .functions
        .negotiate
        .as_deref()
        .unwrap_or(c"vkNegotiateLoaderLayerInterfaceVersion");
    // SAFETY: The manifest or layer ABI defines the symbol's signature.
    let negotiate = unsafe {
        library
            .get::<NegotiateLoaderLayerInterfaceVersion>(negotiate_name.to_bytes_with_nul())
            .ok()
            .map(|symbol| *symbol)
    };

    let mut negotiated = NegotiateLayerInterface {
        s_type: NEGOTIATE_INTERFACE_STRUCT,
        p_next: ptr::null_mut(),
        loader_layer_interface_version: CURRENT_LAYER_INTERFACE_VERSION,
        get_instance_proc_addr: None,
        get_device_proc_addr: None,
        get_physical_device_proc_addr: None,
    };
    if let Some(negotiate) = negotiate {
        // SAFETY: `negotiated` has the C layout required by `vk_layer.h`.
        if unsafe { negotiate(&raw mut negotiated) } != VkResult::SUCCESS
            || negotiated.loader_layer_interface_version == 0
        {
            return Err(LayerLoadError::Failed);
        }
    }
    Ok((negotiate.is_some(), negotiated))
}
