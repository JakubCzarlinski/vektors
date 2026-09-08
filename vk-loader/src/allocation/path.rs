//! Fallible path joining, including Windows verbatim normalization.

#[cfg(windows)]
use alloc::vec::Vec;
use std::path::{Path, PathBuf};
#[cfg(windows)]
use std::{ffi::OsString, path::Component};
use vk::VkResult;

pub(crate) fn try_join_path(base: &Path, suffix: &Path) -> Result<PathBuf, VkResult> {
    #[cfg(windows)]
    if matches!(base.components().next(), Some(Component::Prefix(prefix)) if prefix.kind().is_verbatim())
        && !suffix.as_os_str().is_empty()
        && !matches!(suffix.components().next(), Some(Component::Prefix(_)))
        && !suffix.is_absolute()
    {
        return join_verbatim(base, suffix);
    }

    let capacity = base
        .as_os_str()
        .len()
        .checked_add(suffix.as_os_str().len())
        .and_then(|length| length.checked_add(1))
        .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut joined = PathBuf::new();
    joined
        .try_reserve_exact(capacity)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    joined.push(base);
    joined.push(suffix);
    Ok(joined)
}

#[cfg(windows)]
fn join_verbatim(base: &Path, suffix: &Path) -> Result<PathBuf, VkResult> {
    // PathBuf::push internally allocates both a component vector and a new
    // string for verbatim paths, regardless of the destination's capacity.
    let capacity = base
        .components()
        .count()
        .checked_add(suffix.components().count())
        .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut components = Vec::new();
    components
        .try_reserve_exact(capacity)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    components.extend(base.components());
    for component in suffix.components() {
        match component {
            Component::RootDir => {
                components.truncate(1);
                components.push(component);
            }
            Component::CurDir => (),
            Component::ParentDir => {
                if matches!(components.last(), Some(Component::Normal(_))) {
                    components.pop();
                }
            }
            _ => components.push(component),
        }
    }
    let capacity = components
        .iter()
        .try_fold(0_usize, |length, component| {
            length
                .checked_add(component.as_os_str().len())?
                .checked_add(1)
        })
        .ok_or(VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut output = OsString::new();
    output
        .try_reserve_exact(capacity)
        .map_err(|_| VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut separator = false;
    for component in components {
        if separator && component != Component::RootDir {
            output.push("\\");
        }
        output.push(component.as_os_str());
        separator = match component {
            Component::RootDir => false,
            Component::Prefix(prefix) => !matches!(prefix.kind(), std::path::Prefix::Disk(_)),
            _ => true,
        };
    }
    Ok(PathBuf::from(output))
}

#[cfg(test)]
mod tests {
    use super::try_join_path;
    use std::path::Path;
    use vk::VkResult;

    #[test]
    fn joining_matches_std_and_reports_allocation_failures() {
        for base in [
            "",
            ".",
            "/a/b",
            "C:",
            "C:\\a",
            r"\\?\C:\a\b",
            r"\\?\UNC\host\share\a",
        ] {
            for suffix in [
                "",
                ".",
                "..",
                "../..",
                "../c",
                "x/./y/../z",
                "/root",
                r"\root",
                "D:relative",
                r"D:\absolute",
            ] {
                let base = Path::new(base);
                let suffix = Path::new(suffix);
                let expected = base.join(suffix);
                crate::allocation::fault::sweep_operation(|| match try_join_path(base, suffix) {
                    Ok(actual) => {
                        assert_eq!(
                            actual.as_os_str(),
                            expected.as_os_str(),
                            "{base:?} + {suffix:?}"
                        );
                        VkResult::SUCCESS
                    }
                    Err(error) => error,
                });
            }
        }
    }
}
