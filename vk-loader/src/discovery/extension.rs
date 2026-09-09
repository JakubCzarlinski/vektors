//! Internal extension identities; unknown vendor extensions retain their names.

use alloc::{ffi::CString, vec::Vec};
use core::{ffi::CStr, ops::Deref};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ExtensionName {
    Known(u16),
    Unknown(CString),
}

impl ExtensionName {
    pub(crate) fn as_c_str(&self) -> &CStr {
        match self {
            Self::Known(id) => crate::extension_name(*id),
            Self::Unknown(name) => name,
        }
    }

    pub(crate) fn matches(&self, id: Option<u16>, name: &CStr) -> bool {
        match self {
            Self::Known(known) => id == Some(*known),
            Self::Unknown(unknown) => id.is_none() && unknown.as_c_str() == name,
        }
    }
}

impl From<CString> for ExtensionName {
    fn from(name: CString) -> Self {
        match crate::extension_id(&name) {
            Some(id) => Self::Known(id),
            None => Self::Unknown(name),
        }
    }
}

impl Deref for ExtensionName {
    type Target = CStr;
    fn deref(&self) -> &CStr {
        self.as_c_str()
    }
}

/// Known extensions need no owned strings or search structure.
#[derive(Default)]
pub(crate) struct AvailableDeviceExtensions {
    known: crate::generated::ExtensionSet,
    unknown: Vec<CString>,
}

impl AvailableDeviceExtensions {
    pub(crate) fn insert(&mut self, name: &ExtensionName) -> Result<(), vk::VkResult> {
        match name {
            ExtensionName::Known(id) => {
                self.known.insert(*id);
                Ok(())
            }
            ExtensionName::Unknown(name) => self.insert_unknown(name),
        }
    }

    pub(crate) fn insert_name(&mut self, name: &CStr) -> Result<(), vk::VkResult> {
        match crate::extension_id(name) {
            Some(id) => {
                self.known.insert(id);
                Ok(())
            }
            None => self.insert_unknown(name),
        }
    }

    fn insert_unknown(&mut self, name: &CStr) -> Result<(), vk::VkResult> {
        if !self
            .unknown
            .iter()
            .any(|existing| existing.as_c_str() == name)
        {
            let name = crate::allocation::try_c_string(name)?;
            crate::allocation::try_push(&mut self.unknown, name)?;
        }
        Ok(())
    }

    pub(crate) fn contains(&self, id: Option<u16>, name: &CStr) -> bool {
        match id {
            Some(id) => self.known.contains(id),
            None => self
                .unknown
                .iter()
                .any(|unknown| unknown.as_c_str() == name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vendor_extension_insertion_reports_oom_and_can_be_retried() {
        crate::allocation::fault::sweep_operation(|| {
            let mut available = AvailableDeviceExtensions::default();
            let name = c"VK_VENDOR_allocation_test";
            let result = available.insert_name(name);
            if let Err(error) = result {
                assert!(!available.contains(None, name));
                // The injected failure happens once. A failed insertion must
                // leave the set usable without publishing a partial name.
                available.insert_name(name).unwrap();
                assert!(available.contains(None, name));
                return error;
            }
            assert!(available.contains(None, name));
            vk::VkResult::SUCCESS
        });
    }

    #[test]
    fn known_and_vendor_extension_names_preserve_exact_membership() {
        let known = vk::VK_EXT_DEBUG_UTILS_EXTENSION_NAME;
        let vendor = c"VK_VENDOR_test_\xff";
        let mut available = AvailableDeviceExtensions::default();
        for name in [known, vendor, known, vendor] {
            available.insert_name(name).unwrap();
        }
        for (name, expected) in [
            (known, true),
            (vendor, true),
            (c"VK_EXT_DEBUG_UTILS", false),
            (c"VK_VENDOR_test", false),
            (vk::VK_KHR_SURFACE_EXTENSION_NAME, false),
        ] {
            assert_eq!(
                available.contains(crate::extension_id(name), name),
                expected
            );
        }
        for name in [known, vendor] {
            let identity = ExtensionName::from(name.to_owned());
            assert_eq!(identity.as_c_str(), name);
            assert!(identity.matches(crate::extension_id(name), name));
            assert!(!identity.matches(crate::extension_id(c"absent"), c"absent"));
        }
    }
}
