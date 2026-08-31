//! Driver manifest discovery and parsing.

use std::{
    borrow::Cow,
    env,
    ffi::{CStr, CString, OsStr, OsString},
    fmt,
    marker::PhantomData,
    ops::Deref,
    path::{Path, PathBuf},
};

use crate::collections::HashSet;

use serde::{
    Deserialize, Deserializer,
    de::{IgnoredAny, MapAccess, SeqAccess, Visitor},
};
use serde_json::Value;
use serde_json::value::RawValue;
use vk::VK_MAKE_API_VERSION;

use crate::platform;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DriverManifest {
    pub(crate) manifest_path: PathBuf,
    pub(crate) library_path: PathBuf,
    pub(crate) manifest_version: u32,
    pub(crate) api_version: u32,
    pub(crate) architecture_supported: bool,
    pub(crate) portability_driver: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LayerExtension {
    pub(crate) name: CString,
    pub(crate) spec_version: u32,
    pub(crate) entrypoints: Box<[CString]>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LayerManifest {
    pub(crate) name: CString,
    pub(crate) manifest_path: PathBuf,
    pub(crate) library_path: Option<PathBuf>,
    pub(crate) manifest_version: u32,
    pub(crate) api_version: u32,
    pub(crate) architecture_supported: bool,
    pub(crate) implementation_version: u32,
    pub(crate) description: CString,
    pub(crate) instance_extensions: Box<[LayerExtension]>,
    pub(crate) device_extensions: Box<[LayerExtension]>,
    pub(crate) enable_environment: Option<(OsString, OsString)>,
    pub(crate) disable_environment: Option<(OsString, OsString)>,
    pub(crate) component_layers: Box<[CString]>,
    pub(crate) blacklisted_layers: Box<[CString]>,
    pub(crate) override_paths: Box<[PathBuf]>,
    pub(crate) app_keys: Box<[PathBuf]>,
    pub(crate) functions: LayerFunctions,
    pub(crate) pre_instance_functions: PreInstanceFunctions,
    pub(crate) implicit: bool,
    pub(crate) settings_control: Option<Box<str>>,
}

pub(crate) struct DiscoveredLayers {
    manifests: Box<[LayerManifest]>,
    searches: Box<[LayerSearch]>,
}

pub(crate) struct LayerSearch {
    pub(crate) implicit: bool,
    pub(crate) roots: Box<[PathBuf]>,
    pub(crate) files: Box<[PathBuf]>,
}

impl DiscoveredLayers {
    pub(crate) fn searches(&self) -> &[LayerSearch] {
        &self.searches
    }

    pub(crate) fn into_vec(self) -> Vec<LayerManifest> {
        self.manifests.into_vec()
    }
}

impl Deref for DiscoveredLayers {
    type Target = [LayerManifest];

    fn deref(&self) -> &Self::Target {
        &self.manifests
    }
}

impl<'a> IntoIterator for &'a DiscoveredLayers {
    type Item = &'a LayerManifest;
    type IntoIter = core::slice::Iter<'a, LayerManifest>;

    fn into_iter(self) -> Self::IntoIter {
        self.manifests.iter()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct LayerFunctions {
    pub(crate) negotiate: Option<CString>,
    pub(crate) get_instance_proc_addr: Option<CString>,
    pub(crate) get_device_proc_addr: Option<CString>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct PreInstanceFunctions {
    pub(crate) extension_properties: Option<CString>,
    pub(crate) layer_properties: Option<CString>,
    pub(crate) version: Option<CString>,
}

/// Marks manifests whose complete meta-layer dependency graph is present and acyclic.
pub(crate) fn valid_layer_mask(manifests: &[LayerManifest]) -> Box<[bool]> {
    fn visit(index: usize, manifests: &[LayerManifest], states: &mut [u8]) -> bool {
        match states[index] {
            1 | 3 => return false,
            2 => return true,
            _ => {}
        }
        if vk::VK_API_VERSION_VARIANT(manifests[index].api_version) != 0
            || !manifests[index].architecture_supported
        {
            states[index] = 3;
            return false;
        }
        states[index] = 1;
        let valid = manifests[index].component_layers.iter().all(|name| {
            manifests
                .iter()
                .position(|candidate| candidate.name == *name)
                .is_some_and(|component| {
                    let meta = manifests[index].api_version;
                    let component_version = manifests[component].api_version;
                    vk::VK_API_VERSION_MAJOR(component_version) >= vk::VK_API_VERSION_MAJOR(meta)
                        && (vk::VK_API_VERSION_MAJOR(component_version)
                            > vk::VK_API_VERSION_MAJOR(meta)
                            || vk::VK_API_VERSION_MINOR(component_version)
                                >= vk::VK_API_VERSION_MINOR(meta))
                        && visit(component, manifests, states)
                })
        });
        states[index] = if valid { 2 } else { 3 };
        valid
    }

    let mut states = vec![0; manifests.len()];
    for index in 0..manifests.len() {
        visit(index, manifests, &mut states);
    }
    states.into_iter().map(|state| state == 2).collect()
}

fn strtoul_prefix(value: &str) -> libc::c_ulong {
    let bytes = value.as_bytes();
    let mut index = bytes
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let negative = bytes.get(index) == Some(&b'-');
    if matches!(bytes.get(index), Some(b'+' | b'-')) {
        index += 1;
    }
    let mut parsed = 0_u128;
    let mut overflow = false;
    while let Some(digit) = bytes.get(index).and_then(|byte| byte.checked_sub(b'0'))
        && digit < 10
    {
        parsed = parsed
            .checked_mul(10)
            .and_then(|value| value.checked_add(u128::from(digit)))
            .unwrap_or_else(|| {
                overflow = true;
                u128::MAX
            });
        if parsed > u128::from(libc::c_ulong::MAX) {
            overflow = true;
        }
        index += 1;
    }
    if overflow {
        return libc::c_ulong::MAX;
    }
    let parsed = parsed as libc::c_ulong;
    if negative {
        parsed.wrapping_neg()
    } else {
        parsed
    }
}

fn parse_api_version(version: Option<&str>) -> Option<u32> {
    let mut components = version?.split(['.', '"', '\n', '\r']);
    let mut next_component = || {
        components
            .find(|component| !component.is_empty())
            .map_or(0, |component| u32::from(strtoul_prefix(component) as u16))
    };
    let first = next_component();
    let second = next_component();
    let third = next_component();
    let fourth = components
        .find(|component| !component.is_empty())
        .map(|component| u32::from(strtoul_prefix(component) as u16));
    Some(match fourth {
        Some(patch) => VK_MAKE_API_VERSION(first, second, third, patch),
        None => VK_MAKE_API_VERSION(0, first, second, third),
    })
}

fn parse_manifest_u32(value: &str) -> u32 {
    strtoul_prefix(value) as u32
}

fn split_paths(value: &OsStr) -> Box<[PathBuf]> {
    env::split_paths(value)
        .filter(|path| !path.as_os_str().is_empty())
        .collect()
}

#[cfg(unix)]
fn append_search_root(paths: &mut Vec<PathBuf>, root: impl AsRef<Path>, leaf: &str) {
    paths.push(root.as_ref().join("vulkan").join(leaf));
}

#[cfg(unix)]
pub(crate) fn default_search_paths(leaf: &str) -> Box<[PathBuf]> {
    let mut paths = Vec::new();
    let elevated = platform::has_elevated_privileges();

    #[cfg(target_vendor = "apple")]
    {
        if let Some(resources) = platform::bundle_resource_directory() {
            append_search_root(&mut paths, resources, leaf);
        }
        // Presence, rather than the environment variable's value, activates
        // the upstream Apple loader's bundle-exclusive mode.
        if !elevated && env::var_os("VK_LOADER_SEARCH_ONLY_IN_BUNDLE").is_some() {
            return paths.into_boxed_slice();
        }
    }

    if elevated {
        if !cfg!(any(
            target_os = "fuchsia",
            target_os = "nto",
            target_os = "qnx"
        )) {
            append_search_root(&mut paths, "/etc/xdg", leaf);
        }
    } else {
        if let Some(value) = env::var_os("XDG_CONFIG_HOME") {
            for root in split_paths(&value) {
                append_search_root(&mut paths, root, leaf);
            }
        } else if let Some(value) = env::var_os("HOME") {
            append_search_root(&mut paths, PathBuf::from(value).join(".config"), leaf);
        }
        if let Some(value) = env::var_os("XDG_CONFIG_DIRS") {
            for root in split_paths(&value) {
                append_search_root(&mut paths, root, leaf);
            }
        } else if !cfg!(any(
            target_os = "fuchsia",
            target_os = "nto",
            target_os = "qnx"
        )) {
            append_search_root(&mut paths, "/etc/xdg", leaf);
        }
    }

    if cfg!(target_os = "fuchsia") {
        append_search_root(&mut paths, "/config", leaf);
        append_search_root(&mut paths, "/pkg/data", leaf);
    } else if cfg!(any(target_os = "nto", target_os = "qnx")) {
        append_search_root(&mut paths, "/etc", leaf);
    } else {
        append_search_root(&mut paths, "/usr/local/etc", leaf);
        append_search_root(&mut paths, "/etc", leaf);
    }

    if !elevated {
        if let Some(value) = env::var_os("XDG_DATA_HOME") {
            for root in split_paths(&value) {
                append_search_root(&mut paths, root, leaf);
            }
        } else if let Some(value) = env::var_os("HOME") {
            append_search_root(&mut paths, PathBuf::from(value).join(".local/share"), leaf);
        }
        if let Some(value) = env::var_os("XDG_DATA_DIRS") {
            for root in split_paths(&value) {
                append_search_root(&mut paths, root, leaf);
            }
        } else if !cfg!(any(
            target_os = "fuchsia",
            target_os = "nto",
            target_os = "qnx"
        )) {
            append_search_root(&mut paths, "/usr/local/share", leaf);
            append_search_root(&mut paths, "/usr/share", leaf);
        }
    } else if !cfg!(any(
        target_os = "fuchsia",
        target_os = "nto",
        target_os = "qnx"
    )) {
        append_search_root(&mut paths, "/usr/local/share", leaf);
        append_search_root(&mut paths, "/usr/share", leaf);
    }
    paths.into_boxed_slice()
}

#[cfg(windows)]
pub(crate) fn default_search_paths(leaf: &str) -> Box<[PathBuf]> {
    platform::registry_manifest_files(leaf)
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn default_search_paths(_leaf: &str) -> Box<[PathBuf]> {
    Box::default()
}

fn resolve_library_path(manifest_path: &Path, library: PathBuf) -> Option<PathBuf> {
    if library.is_absolute() || library.parent() == Some(Path::new("")) {
        Some(library)
    } else {
        Some(manifest_path.parent()?.join(library))
    }
}

fn library_architecture_supported(value: Option<&str>) -> bool {
    !matches!(
        (value, core::mem::size_of::<usize>()),
        (Some(value), 8) if value.starts_with("32")
    ) && !matches!(
        (value, core::mem::size_of::<usize>()),
        (Some(value), 4) if value.starts_with("64")
    )
}

struct DriverManifestDocument<'a> {
    file_format_version: Option<BorrowedString<'a>>,
    icd: Option<RawDriverIcd<'a>>,
}

struct DriverManifestDocumentVisitor<'a>(PhantomData<&'a ()>);

impl<'de: 'a, 'a> Visitor<'de> for DriverManifestDocumentVisitor<'a> {
    type Value = DriverManifestDocument<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Vulkan ICD-manifest document")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut document = DriverManifestDocument {
            file_format_version: None,
            icd: None,
        };
        while let Some(key) = map.next_key::<BorrowedString<'a>>()? {
            if key.eq_ignore_ascii_case("file_format_version")
                && document.file_format_version.is_none()
            {
                document.file_format_version = Some(map.next_value()?);
            } else if key.eq_ignore_ascii_case("ICD") && document.icd.is_none() {
                document.icd = Some(map.next_value()?);
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }
        Ok(document)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for DriverManifestDocument<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DriverManifestDocumentVisitor(PhantomData))
    }
}

#[derive(Default)]
struct RawDriverIcdFields<'a> {
    library_path: Option<&'a RawValue>,
    api_version: Option<BorrowedString<'a>>,
    is_portability_driver: Option<&'a RawValue>,
    library_arch: Option<BorrowedString<'a>>,
}

struct RawDriverIcd<'a>(Box<RawDriverIcdFields<'a>>);

struct RawDriverIcdVisitor<'a>(PhantomData<&'a ()>);

impl<'de: 'a, 'a> Visitor<'de> for RawDriverIcdVisitor<'a> {
    type Value = RawDriverIcd<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Vulkan ICD object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut icd = Box::<RawDriverIcdFields<'a>>::default();
        while let Some(key) = map.next_key::<BorrowedString<'a>>()? {
            if key.eq_ignore_ascii_case("library_path") && icd.library_path.is_none() {
                icd.library_path = Some(map.next_value()?);
            } else if key.eq_ignore_ascii_case("api_version") && icd.api_version.is_none() {
                icd.api_version = Some(map.next_value()?);
            } else if key.eq_ignore_ascii_case("is_portability_driver")
                && icd.is_portability_driver.is_none()
            {
                icd.is_portability_driver = Some(map.next_value()?);
            } else if key.eq_ignore_ascii_case("library_arch") && icd.library_arch.is_none() {
                icd.library_arch = Some(map.next_value()?);
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }
        Ok(RawDriverIcd(icd))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for RawDriverIcd<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RawDriverIcdVisitor(PhantomData))
    }
}

fn raw_json_path(value: &RawValue) -> Option<PathBuf> {
    let raw = value.get();
    if raw.as_bytes().first() == Some(&b'"') {
        let value: BorrowedString<'_> = serde_json::from_str(raw).ok()?;
        Some(PathBuf::from(value.as_ref()))
    } else {
        Some(PathBuf::from(raw))
    }
}

pub(crate) fn parse_manifest(path: &Path) -> Option<DriverManifest> {
    let bytes = platform::read_file(path)?;
    let root: DriverManifestDocument<'_> = serde_json::from_slice(&bytes).ok()?;
    let manifest_version = parse_api_version(Some(root.file_format_version?.as_ref()))?;
    let icd = root.icd?.0;
    let library = raw_json_path(icd.library_path?)?;
    if library.as_os_str().is_empty() {
        return None;
    }
    let library_path = resolve_library_path(path, library)?;
    let api_version = parse_api_version(Some(icd.api_version?.as_ref()))?;
    Some(DriverManifest {
        manifest_path: path.to_owned(),
        library_path,
        manifest_version,
        api_version,
        architecture_supported: library_architecture_supported(icd.library_arch.as_deref()),
        portability_driver: icd
            .is_portability_driver
            .is_some_and(|value| value.get().trim() == "true"),
    })
}

struct LayerManifestDocument<'a> {
    file_format_version: Option<BorrowedString<'a>>,
    layer: Option<RawLayerBox<'a>>,
    layers: Option<Box<[RawLayerBox<'a>]>>,
}

struct LayerManifestDocumentVisitor<'a>(PhantomData<&'a ()>);

impl<'de: 'a, 'a> Visitor<'de> for LayerManifestDocumentVisitor<'a> {
    type Value = LayerManifestDocument<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Vulkan layer-manifest document")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut document = LayerManifestDocument {
            file_format_version: None,
            layer: None,
            layers: None,
        };
        while let Some(key) = map.next_key::<BorrowedString<'a>>()? {
            let destination = if key.eq_ignore_ascii_case("file_format_version") {
                Some(&mut document.file_format_version)
            } else {
                None
            };
            if let Some(destination) = destination {
                if destination.is_none() {
                    *destination = Some(map.next_value()?);
                } else {
                    map.next_value::<IgnoredAny>()?;
                }
            } else if key.eq_ignore_ascii_case("layer") {
                if document.layer.is_none() {
                    document.layer = Some(map.next_value()?);
                } else {
                    map.next_value::<IgnoredAny>()?;
                }
            } else if key.eq_ignore_ascii_case("layers") {
                if document.layers.is_none() {
                    document.layers = Some(map.next_value()?);
                } else {
                    map.next_value::<IgnoredAny>()?;
                }
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }
        Ok(document)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for LayerManifestDocument<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(LayerManifestDocumentVisitor(PhantomData))
    }
}

#[derive(Default)]
struct RawLayer<'a> {
    name: Option<LenientString<'a>>,
    layer_type: Option<LenientString<'a>>,
    library_path: Option<LenientString<'a>>,
    api_version: Option<LenientString<'a>>,
    library_arch: Option<LenientString<'a>>,
    implementation_version: Option<LenientString<'a>>,
    description: Option<LenientString<'a>>,
    instance_extensions: Option<&'a RawValue>,
    device_extensions: Option<&'a RawValue>,
    enable_environment: Option<&'a RawValue>,
    disable_environment: Option<&'a RawValue>,
    component_layers: Option<&'a RawValue>,
    blacklisted_layers: Option<&'a RawValue>,
    override_paths: Option<&'a RawValue>,
    app_keys: Option<&'a RawValue>,
    functions: Option<&'a RawValue>,
    pre_instance_functions: Option<&'a RawValue>,
}

struct RawLayerBox<'a>(Box<RawLayer<'a>>);

struct RawLayerBoxVisitor<'a>(PhantomData<&'a ()>);

impl<'de: 'a, 'a> Visitor<'de> for RawLayerBoxVisitor<'a> {
    type Value = RawLayerBox<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Vulkan layer-manifest object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut layer = Box::<RawLayer<'a>>::default();
        while let Some(key) = map.next_key::<BorrowedString<'a>>()? {
            macro_rules! parse_first {
                ($field:expr) => {
                    if $field.is_none() {
                        $field = Some(map.next_value()?);
                    } else {
                        map.next_value::<IgnoredAny>()?;
                    }
                };
            }
            if key.eq_ignore_ascii_case("name") {
                parse_first!(layer.name);
            } else if key.eq_ignore_ascii_case("type") {
                parse_first!(layer.layer_type);
            } else if key.eq_ignore_ascii_case("library_path") {
                parse_first!(layer.library_path);
            } else if key.eq_ignore_ascii_case("api_version") {
                parse_first!(layer.api_version);
            } else if key.eq_ignore_ascii_case("library_arch") {
                parse_first!(layer.library_arch);
            } else if key.eq_ignore_ascii_case("implementation_version") {
                parse_first!(layer.implementation_version);
            } else if key.eq_ignore_ascii_case("description") {
                parse_first!(layer.description);
            } else if key.eq_ignore_ascii_case("instance_extensions") {
                parse_first!(layer.instance_extensions);
            } else if key.eq_ignore_ascii_case("device_extensions") {
                parse_first!(layer.device_extensions);
            } else if key.eq_ignore_ascii_case("enable_environment") {
                parse_first!(layer.enable_environment);
            } else if key.eq_ignore_ascii_case("disable_environment") {
                parse_first!(layer.disable_environment);
            } else if key.eq_ignore_ascii_case("component_layers") {
                parse_first!(layer.component_layers);
            } else if key.eq_ignore_ascii_case("blacklisted_layers") {
                parse_first!(layer.blacklisted_layers);
            } else if key.eq_ignore_ascii_case("override_paths") {
                parse_first!(layer.override_paths);
            } else if key.eq_ignore_ascii_case("app_keys") {
                parse_first!(layer.app_keys);
            } else if key.eq_ignore_ascii_case("functions") {
                parse_first!(layer.functions);
            } else if key.eq_ignore_ascii_case("pre_instance_functions") {
                parse_first!(layer.pre_instance_functions);
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }
        Ok(RawLayerBox(layer))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for RawLayerBox<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RawLayerBoxVisitor(PhantomData))
    }
}

struct RawLayerExtension<'a> {
    name: Option<LenientString<'a>>,
    spec_version: Option<LenientString<'a>>,
    entrypoints: Option<&'a RawValue>,
}

struct RawLayerExtensionVisitor<'a>(PhantomData<&'a ()>);

impl<'de: 'a, 'a> Visitor<'de> for RawLayerExtensionVisitor<'a> {
    type Value = RawLayerExtension<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a layer-extension object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut extension = RawLayerExtension {
            name: None,
            spec_version: None,
            entrypoints: None,
        };
        while let Some(key) = map.next_key::<BorrowedString<'a>>()? {
            if key.eq_ignore_ascii_case("name") && extension.name.is_none() {
                extension.name = Some(map.next_value()?);
            } else if key.eq_ignore_ascii_case("spec_version") && extension.spec_version.is_none() {
                extension.spec_version = Some(map.next_value()?);
            } else if key.eq_ignore_ascii_case("entrypoints") && extension.entrypoints.is_none() {
                extension.entrypoints = Some(map.next_value()?);
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }
        Ok(extension)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for RawLayerExtension<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RawLayerExtensionVisitor(PhantomData))
    }
}

#[derive(Default)]
struct RawLayerFunctions<'a> {
    negotiate: Option<LenientString<'a>>,
    get_instance_proc_addr: Option<LenientString<'a>>,
    get_device_proc_addr: Option<LenientString<'a>>,
    enumerate_instance_extension_properties: Option<LenientString<'a>>,
    enumerate_instance_layer_properties: Option<LenientString<'a>>,
    enumerate_instance_version: Option<LenientString<'a>>,
}

struct RawLayerFunctionsVisitor<'a>(PhantomData<&'a ()>);

impl<'de: 'a, 'a> Visitor<'de> for RawLayerFunctionsVisitor<'a> {
    type Value = RawLayerFunctions<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a layer-functions object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut functions = RawLayerFunctions::default();
        while let Some(key) = map.next_key::<BorrowedString<'a>>()? {
            let destination = if key.eq_ignore_ascii_case("vkNegotiateLoaderLayerInterfaceVersion")
            {
                Some(&mut functions.negotiate)
            } else if key.eq_ignore_ascii_case("vkGetInstanceProcAddr") {
                Some(&mut functions.get_instance_proc_addr)
            } else if key.eq_ignore_ascii_case("vkGetDeviceProcAddr") {
                Some(&mut functions.get_device_proc_addr)
            } else if key.eq_ignore_ascii_case("vkEnumerateInstanceExtensionProperties") {
                Some(&mut functions.enumerate_instance_extension_properties)
            } else if key.eq_ignore_ascii_case("vkEnumerateInstanceLayerProperties") {
                Some(&mut functions.enumerate_instance_layer_properties)
            } else if key.eq_ignore_ascii_case("vkEnumerateInstanceVersion") {
                Some(&mut functions.enumerate_instance_version)
            } else {
                None
            };
            if let Some(destination) = destination {
                if destination.is_none() {
                    *destination = Some(map.next_value()?);
                } else {
                    map.next_value::<IgnoredAny>()?;
                }
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }
        Ok(functions)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for RawLayerFunctions<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RawLayerFunctionsVisitor(PhantomData))
    }
}

struct BorrowedString<'a>(Cow<'a, str>);

impl Deref for BorrowedString<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for BorrowedString<'_> {
    fn as_ref(&self) -> &str {
        self
    }
}

struct BorrowedStringVisitor<'a>(PhantomData<&'a str>);

impl<'de: 'a, 'a> Visitor<'de> for BorrowedStringVisitor<'a> {
    type Value = BorrowedString<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a UTF-8 string")
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BorrowedString(Cow::Borrowed(value)))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BorrowedString(Cow::Owned(value.to_owned())))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BorrowedString(Cow::Owned(value)))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for BorrowedString<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(BorrowedStringVisitor(PhantomData))
    }
}

struct LenientString<'a>(Option<BorrowedString<'a>>);

struct LenientStringVisitor<'a>(PhantomData<&'a str>);

impl<'de: 'a, 'a> Visitor<'de> for LenientStringVisitor<'a> {
    type Value = LenientString<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("any JSON value")
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
        Ok(LenientString(Some(BorrowedString(Cow::Borrowed(value)))))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(LenientString(Some(BorrowedString(Cow::Owned(
            value.to_owned(),
        )))))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(LenientString(Some(BorrowedString(Cow::Owned(value)))))
    }

    fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E> {
        Ok(LenientString(None))
    }

    fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E> {
        Ok(LenientString(None))
    }

    fn visit_u64<E>(self, _value: u64) -> Result<Self::Value, E> {
        Ok(LenientString(None))
    }

    fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E> {
        Ok(LenientString(None))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(LenientString(None))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(LenientString(None))
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        while sequence.next_element::<IgnoredAny>()?.is_some() {}
        Ok(LenientString(None))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
        Ok(LenientString(None))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for LenientString<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(LenientStringVisitor(PhantomData))
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
struct BorrowedStrings<'a>(#[serde(borrow)] Box<[BorrowedString<'a>]>);

struct BorrowedEnvironment<'a>(Option<(BorrowedString<'a>, BorrowedString<'a>)>);

struct BorrowedEnvironmentVisitor<'a>(PhantomData<&'a ()>);

impl<'de: 'a, 'a> Visitor<'de> for BorrowedEnvironmentVisitor<'a> {
    type Value = BorrowedEnvironment<'a>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an environment-variable object")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let first = map.next_entry::<BorrowedString<'a>, BorrowedString<'a>>()?;
        while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
        Ok(BorrowedEnvironment(first))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for BorrowedEnvironment<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(BorrowedEnvironmentVisitor(PhantomData))
    }
}

fn borrowed_c_string(value: Option<LenientString<'_>>) -> Option<CString> {
    CString::new(value?.0?.as_bytes()).ok()
}

fn borrowed_c_string_limited(value: Option<LenientString<'_>>, capacity: usize) -> Option<CString> {
    let value = value?.0?;
    (value.len() < capacity)
        .then(|| CString::new(value.as_bytes()).ok())
        .flatten()
}

fn raw_c_string_array(value: Option<&RawValue>) -> Box<[CString]> {
    value
        .and_then(|value| serde_json::from_str::<BorrowedStrings<'_>>(value.get()).ok())
        .map(|values| {
            values
                .0
                .into_iter()
                .filter_map(|value| CString::new(value.as_bytes()).ok())
                .collect()
        })
        .unwrap_or_default()
}

fn raw_path_array(value: Option<&RawValue>) -> Box<[PathBuf]> {
    value
        .and_then(|value| serde_json::from_str::<BorrowedStrings<'_>>(value.get()).ok())
        .map(|values| {
            values
                .0
                .into_iter()
                .map(|value| PathBuf::from(value.as_ref()))
                .collect()
        })
        .unwrap_or_default()
}

fn raw_environment(value: Option<&RawValue>) -> Option<(OsString, OsString)> {
    let environment = serde_json::from_str::<BorrowedEnvironment<'_>>(value?.get()).ok()?;
    let (name, value) = environment.0?;
    Some((
        OsString::from(name.as_ref()),
        OsString::from(value.as_ref()),
    ))
}

fn parse_raw_layer_extension(value: &RawValue) -> Option<LayerExtension> {
    let extension = serde_json::from_str::<RawLayerExtension<'_>>(value.get()).ok()?;
    Some(LayerExtension {
        name: borrowed_c_string_limited(extension.name, vk::VK_MAX_EXTENSION_NAME_SIZE as usize)?,
        spec_version: extension
            .spec_version
            .and_then(|value| value.0)
            .as_deref()
            .map_or(0, parse_manifest_u32),
        entrypoints: raw_c_string_array(extension.entrypoints),
    })
}

fn parse_raw_layer_extensions(value: Option<&RawValue>, instance: bool) -> Box<[LayerExtension]> {
    value
        .and_then(|value| serde_json::from_str::<Box<[&RawValue]>>(value.get()).ok())
        .map(|values| {
            values
                .into_iter()
                .filter_map(parse_raw_layer_extension)
                .filter(|extension| {
                    !instance || crate::wsi_instance_extension_supported(extension.name.as_c_str())
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_raw_layer(
    path: &Path,
    mut layer: RawLayerBox<'_>,
    implicit: bool,
    supports_pre_instance: bool,
    manifest_version: u32,
) -> Option<LayerManifest> {
    let layer = &mut layer.0;
    let has_component_layers = layer.component_layers.is_some();
    let component_layers = raw_c_string_array(layer.component_layers.take());
    let name =
        borrowed_c_string_limited(layer.name.take(), vk::VK_MAX_EXTENSION_NAME_SIZE as usize)?;
    match layer.layer_type.take()?.0?.as_ref() {
        "INSTANCE" | "GLOBAL" => {}
        _ => return None,
    }
    let is_override = name.as_c_str() == c"VK_LAYER_LUNARG_override";
    let disable_environment = implicit
        .then(|| raw_environment(layer.disable_environment.take()))
        .flatten();
    if implicit && disable_environment.is_none() {
        return None;
    }
    let library_path = layer
        .library_path
        .take()
        .and_then(|library| library.0)
        .map(|library| PathBuf::from(library.as_ref()))
        .and_then(|library| resolve_library_path(path, library));
    if (library_path.is_none() && !has_component_layers)
        || (library_path.is_some() && has_component_layers)
    {
        return None;
    }
    let functions = layer
        .functions
        .take()
        .and_then(|value| serde_json::from_str::<RawLayerFunctions<'_>>(value.get()).ok());
    let pre_instance = (implicit && supports_pre_instance)
        .then(|| layer.pre_instance_functions.take())
        .flatten()
        .and_then(|value| serde_json::from_str::<RawLayerFunctions<'_>>(value.get()).ok());
    let api_version = parse_api_version(Some(layer.api_version.take()?.0?.as_ref()))?;
    // Preserve rejected records until layer diagnostics are emitted. Upstream
    // logs these conditions inside `loader_read_layer_json`; our discovery
    // phase has no create-info callback chain, so `valid_layer_mask` removes
    // them immediately after `load_active_layers` reports the same messages.
    let library_arch = layer.library_arch.take().and_then(|value| value.0);
    let architecture_supported = library_architecture_supported(library_arch.as_deref());
    let functions = functions.unwrap_or_default();
    let functions = LayerFunctions {
        negotiate: (manifest_version >= VK_MAKE_API_VERSION(0, 1, 1, 0))
            .then(|| borrowed_c_string(functions.negotiate))
            .flatten(),
        get_instance_proc_addr: borrowed_c_string(functions.get_instance_proc_addr),
        get_device_proc_addr: borrowed_c_string(functions.get_device_proc_addr),
    };
    let pre_instance = pre_instance.unwrap_or_default();
    let pre_instance_functions = PreInstanceFunctions {
        extension_properties: borrowed_c_string(
            pre_instance.enumerate_instance_extension_properties,
        ),
        layer_properties: borrowed_c_string(pre_instance.enumerate_instance_layer_properties),
        version: borrowed_c_string(pre_instance.enumerate_instance_version),
    };
    Some(LayerManifest {
        name,
        manifest_path: path.to_owned(),
        library_path,
        manifest_version,
        api_version,
        architecture_supported,
        implementation_version: parse_manifest_u32(
            layer.implementation_version.take()?.0?.as_ref(),
        ),
        description: borrowed_c_string_limited(
            layer.description.take(),
            vk::VK_MAX_DESCRIPTION_SIZE as usize,
        )?,
        instance_extensions: parse_raw_layer_extensions(layer.instance_extensions.take(), true),
        device_extensions: parse_raw_layer_extensions(layer.device_extensions.take(), false),
        enable_environment: implicit
            .then(|| raw_environment(layer.enable_environment.take()))
            .flatten(),
        disable_environment,
        component_layers,
        blacklisted_layers: if is_override {
            raw_c_string_array(layer.blacklisted_layers.take())
        } else {
            Box::default()
        },
        override_paths: raw_path_array(layer.override_paths.take()),
        app_keys: raw_path_array(layer.app_keys.take()),
        functions,
        pre_instance_functions,
        implicit,
        settings_control: None,
    })
}

fn parse_layer_manifest(path: &Path, implicit: bool) -> Box<[LayerManifest]> {
    let Some(bytes) = platform::read_file(path) else {
        return Box::default();
    };
    let Ok(root) = serde_json::from_slice::<LayerManifestDocument<'_>>(&bytes) else {
        return Box::default();
    };
    let Some(manifest_version) = parse_api_version(root.file_format_version.as_deref()) else {
        return Box::default();
    };
    let supports_pre_instance = manifest_version >= VK_MAKE_API_VERSION(0, 1, 1, 2);
    if let Some(layers) = root.layers {
        return layers
            .into_iter()
            .filter_map(|layer| {
                parse_raw_layer(
                    path,
                    layer,
                    implicit,
                    supports_pre_instance,
                    manifest_version,
                )
            })
            .collect();
    }
    root.layer
        .and_then(|layer| {
            parse_raw_layer(
                path,
                layer,
                implicit,
                supports_pre_instance,
                manifest_version,
            )
        })
        .into_iter()
        .collect()
}

pub(crate) fn layer_search_roots(implicit: bool) -> Box<[PathBuf]> {
    let (override_name, add_name, leaf) = if implicit {
        (
            "VK_IMPLICIT_LAYER_PATH",
            "VK_ADD_IMPLICIT_LAYER_PATH",
            "implicit_layer.d",
        )
    } else {
        ("VK_LAYER_PATH", "VK_ADD_LAYER_PATH", "explicit_layer.d")
    };
    let elevated = platform::has_elevated_privileges();
    let override_paths = (!elevated).then(|| env::var_os(override_name)).flatten();
    let has_override = override_paths.is_some();
    let mut roots = override_paths.map_or_else(
        || default_search_paths(leaf).into_vec(),
        |value| split_paths(&value).into_vec(),
    );
    if !elevated
        && !has_override
        && let Some(value) = env::var_os(add_name)
    {
        let mut additional = split_paths(&value).into_vec();
        additional.append(&mut roots);
        roots = additional;
    }
    roots.into_boxed_slice()
}

#[cold]
#[inline(never)]
pub(crate) fn discover_layers() -> DiscoveredLayers {
    let settings = loader_settings();
    discover_layers_with_settings(settings.as_ref())
}

pub(crate) fn discover_layers_with_settings(settings: Option<&LoaderSettings>) -> DiscoveredLayers {
    if let Some(settings) = settings {
        let Some(configurations) = settings.layer_configurations.as_ref() else {
            let (manifests, searches) = discover_layers_from_search_paths_with_diagnostics();
            return DiscoveredLayers {
                manifests,
                searches,
            };
        };
        let mut layers = Vec::new();
        let configured_names: HashSet<&CStr> = configurations
            .iter()
            .filter(|configuration| configuration.control.as_ref() != "unordered_layer_location")
            .map(|configuration| configuration.name.as_c_str())
            .collect();
        for configuration in configurations {
            if configuration.control.as_ref() == "unordered_layer_location" {
                let mut regular = discover_layers_from_search_paths().into_vec();
                regular.retain(|manifest| !configured_names.contains(manifest.name.as_c_str()));
                layers.extend(regular);
                continue;
            }
            if configuration.control.as_ref() == "off" {
                continue;
            }
            let mut configured = parse_layer_manifest(
                &configuration.path,
                configuration.treat_as_implicit_manifest,
            )
            .into_vec();
            configured.retain(|layer| layer.name.as_c_str() == configuration.name.as_c_str());
            for layer in &mut configured {
                layer.settings_control = Some(configuration.control.clone());
            }
            for layer in configured {
                let duplicate = layers.iter().any(|existing: &LayerManifest| {
                    existing.name == layer.name
                        && (!existing.component_layers.is_empty()
                            || existing.manifest_path == layer.manifest_path)
                });
                if !duplicate {
                    layers.push(layer);
                }
            }
        }
        return DiscoveredLayers {
            manifests: layers.into_boxed_slice(),
            searches: Box::default(),
        };
    }
    let (manifests, searches) = discover_layers_from_search_paths_with_diagnostics();
    let mut manifests = manifests.into_vec();
    let mut names = HashSet::default();
    manifests.retain(|manifest| names.insert(manifest.name.clone()));
    DiscoveredLayers {
        manifests: manifests.into_boxed_slice(),
        searches,
    }
}

/// Discovers the subset used by pre-instance extension enumeration.
///
/// Upstream scans only implicit manifests on this path. Explicit manifests are
/// needed solely when an active override layer or an implicit meta-layer may
/// reference them.
#[cold]
#[inline(never)]
pub(crate) fn discover_implicit_layers() -> DiscoveredLayers {
    let settings = loader_settings();
    if settings.is_some() {
        let mut discovered = discover_layers_with_settings(settings.as_ref());
        let mut manifests = core::mem::take(&mut discovered.manifests).into_vec();
        retain_implicit_layers_and_components(&mut manifests);
        discovered.manifests = manifests.into_boxed_slice();
        return discovered;
    }

    let implicit_roots = layer_search_roots(true);
    let (mut manifests, implicit_files) =
        discover_layers_in_roots_with_files(&implicit_roots, true);
    select_override_layer(&mut manifests);
    let active_override = manifests.iter().find(|manifest| {
        manifest.name.as_c_str() == c"VK_LAYER_LUNARG_override"
            && crate::layer::implicit_manifest_is_active(manifest)
    });
    let has_implicit_meta_layer = manifests
        .iter()
        .any(|manifest| !manifest.component_layers.is_empty());
    let explicit_search = if active_override.is_some() || has_implicit_meta_layer {
        let explicit_roots = active_override
            .filter(|manifest| !manifest.override_paths.is_empty())
            .map_or_else(
                || layer_search_roots(false),
                |manifest| Box::from(manifest.override_paths.as_ref()),
            );
        let (explicit, explicit_files) =
            discover_layers_in_roots_with_files(&explicit_roots, false);
        manifests.extend(explicit);
        retain_implicit_layers_and_components(&mut manifests);
        Some(LayerSearch {
            implicit: false,
            roots: explicit_roots,
            files: explicit_files,
        })
    } else {
        None
    };
    let mut names = HashSet::default();
    manifests.retain(|manifest| names.insert(manifest.name.clone()));
    let implicit_search = LayerSearch {
        implicit: true,
        roots: implicit_roots,
        files: implicit_files,
    };
    let searches = match explicit_search {
        Some(explicit) => Box::from([implicit_search, explicit]),
        None => Box::from([implicit_search]),
    };
    DiscoveredLayers {
        manifests: manifests.into_boxed_slice(),
        searches,
    }
}

fn retain_implicit_layers_and_components(manifests: &mut Vec<LayerManifest>) {
    let mut keep = vec![false; manifests.len()].into_boxed_slice();
    for (index, manifest) in manifests.iter().enumerate() {
        keep[index] |= manifest.implicit;
        for component in &manifest.component_layers {
            if let Some(component_index) = manifests
                .iter()
                .position(|candidate| candidate.name == *component)
            {
                keep[component_index] = true;
            }
        }
    }
    let mut index = 0;
    manifests.retain(|_| {
        let retain = keep[index];
        index += 1;
        retain
    });
}

fn discover_layers_from_search_paths() -> Box<[LayerManifest]> {
    discover_layers_from_search_paths_with_diagnostics().0
}

fn discover_layers_from_search_paths_with_diagnostics() -> (Box<[LayerManifest]>, Box<[LayerSearch]>)
{
    let implicit_roots = layer_search_roots(true);
    let (mut layers, implicit_files) = discover_layers_in_roots_with_files(&implicit_roots, true);
    select_override_layer(&mut layers);
    let override_roots = layers
        .iter()
        .find(|layer| {
            layer.name.as_c_str() == c"VK_LAYER_LUNARG_override"
                && crate::layer::implicit_manifest_is_active(layer)
                && !layer.override_paths.is_empty()
        })
        .map(|layer| layer.override_paths.as_ref());
    let explicit_roots =
        override_roots.map_or_else(|| layer_search_roots(false), Box::<[PathBuf]>::from);
    let (explicit, explicit_files) = discover_layers_in_roots_with_files(&explicit_roots, false);
    layers.extend(explicit);
    let searches = Box::from([
        LayerSearch {
            implicit: true,
            roots: implicit_roots,
            files: implicit_files,
        },
        LayerSearch {
            implicit: false,
            roots: explicit_roots,
            files: explicit_files,
        },
    ]);
    (layers.into_boxed_slice(), searches)
}

fn discover_layers_in_roots_with_files(
    roots: &[PathBuf],
    implicit: bool,
) -> (Vec<LayerManifest>, Box<[PathBuf]>) {
    let mut layers = Vec::new();
    let mut files = Vec::new();
    for root in roots {
        for path in platform::manifest_files(root) {
            layers.extend(parse_layer_manifest(&path, implicit));
            files.push(path);
        }
    }
    (layers, files.into_boxed_slice())
}

fn select_override_layer(layers: &mut Vec<LayerManifest>) {
    let executable = platform::executable_path();
    select_override_layer_for_executable(layers, executable.as_deref());
}

fn select_override_layer_for_executable(
    layers: &mut Vec<LayerManifest>,
    executable: Option<&Path>,
) {
    let Some(executable) = executable else {
        // Upstream cannot validate app keys without an executable path and
        // consequently leaves the discovered override-layer list unchanged.
        return;
    };
    let matching = layers.iter().position(|layer| {
        layer.name.as_c_str() == c"VK_LAYER_LUNARG_override"
            && layer.app_keys.iter().any(|key| key == executable)
    });
    let global = layers.iter().position(|layer| {
        layer.name.as_c_str() == c"VK_LAYER_LUNARG_override" && layer.app_keys.is_empty()
    });
    let selected = matching.or(global);
    let mut index = 0;
    layers.retain(|layer| {
        let keep = layer.name.as_c_str() != c"VK_LAYER_LUNARG_override" || Some(index) == selected;
        index += 1;
        keep
    });
}

struct SettingsLayerConfiguration {
    name: CString,
    path: PathBuf,
    control: Box<str>,
    treat_as_implicit_manifest: bool,
}

pub(crate) struct LoaderSettings {
    settings_file_path: PathBuf,
    layer_configurations: Option<Box<[SettingsLayerConfiguration]>>,
    additional_drivers: Box<[PathBuf]>,
    additional_drivers_use_exclusively: bool,
    device_configurations: Option<Box<[DeviceConfiguration]>>,
}

impl LoaderSettings {
    pub(crate) fn settings_file_path(&self) -> &Path {
        &self.settings_file_path
    }

    pub(crate) fn into_device_configurations(self) -> Option<Box<[DeviceConfiguration]>> {
        self.device_configurations
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DeviceConfiguration {
    pub(crate) device_uuid: [u8; vk::VK_UUID_SIZE as usize],
    pub(crate) driver_uuid: [u8; vk::VK_UUID_SIZE as usize],
    pub(crate) driver_version: u32,
    pub(crate) device_name: Option<Box<str>>,
    pub(crate) driver_name: Option<Box<str>>,
}

fn format_uuid(uuid: &[u8; vk::VK_UUID_SIZE as usize]) -> String {
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        uuid[0],
        uuid[1],
        uuid[2],
        uuid[3],
        uuid[4],
        uuid[5],
        uuid[6],
        uuid[7],
        uuid[8],
        uuid[9],
        uuid[10],
        uuid[11],
        uuid[12],
        uuid[13],
        uuid[14],
        uuid[15]
    )
}

pub(crate) fn loader_settings() -> Option<LoaderSettings> {
    let (file_path, bytes) = find_loader_settings_file()?;
    let root: Value = serde_json::from_slice(&bytes).ok()?;
    root.get("file_format_version")?.as_str()?;
    let settings = select_settings(&root)?;
    let stderr_filters = settings.get("stderr_log").and_then(Value::as_array);
    let settings_filter_enabled = |name: &str| {
        stderr_filters.is_some_and(|filters| {
            filters.iter().any(|filter| {
                filter.as_str().is_some_and(|filter| {
                    filter.eq_ignore_ascii_case("all") || filter.eq_ignore_ascii_case(name)
                })
            })
        })
    };
    let settings_logging_active = [
        "error", "warn", "warning", "info", "debug", "perf", "driver", "layer",
    ]
    .into_iter()
    .any(settings_filter_enabled);
    let filter_enabled = |name: &str| {
        if settings_logging_active {
            settings_filter_enabled(name)
        } else {
            platform::loader_debug_filter_enabled(name)
        }
    };
    let stderr_logging_active = settings_logging_active
        || ["error", "warn", "info", "debug", "perf", "driver", "layer"]
            .into_iter()
            .any(platform::loader_debug_filter_enabled);
    let layer_configurations: Option<Box<[SettingsLayerConfiguration]>> =
        match settings.get("layers") {
            Some(layers) => Some(
                layers
                    .as_array()?
                    .iter()
                    .filter_map(|layer| {
                        let control = layer.get("control")?.as_str()?;
                        if control == "unordered_layer_location" {
                            return Some(SettingsLayerConfiguration {
                                name: CString::default(),
                                path: PathBuf::new(),
                                control: control.into(),
                                treat_as_implicit_manifest: false,
                            });
                        }
                        Some(SettingsLayerConfiguration {
                            name: CString::new(layer.get("name")?.as_str()?).ok()?,
                            path: PathBuf::from(layer.get("path")?.as_str()?),
                            control: control.into(),
                            treat_as_implicit_manifest: layer
                                .get("treat_as_implicit_manifest")
                                .and_then(Value::as_bool)
                                .unwrap_or(false),
                        })
                    })
                    .collect(),
            ),
            None => None,
        };
    let additional_drivers = settings
        .get("additional_drivers")
        .and_then(Value::as_array)
        .and_then(|drivers| {
            drivers
                .iter()
                .map(|driver| driver.as_object()?.get("path")?.as_str().map(PathBuf::from))
                .collect::<Option<Box<[_]>>>()
        })
        .unwrap_or_default();
    let device_configurations: Option<Box<[DeviceConfiguration]>> =
        settings.get("device_configurations").map(|configurations| {
            configurations
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(parse_device_configuration)
                .collect()
        });
    let settings_active = layer_configurations.is_some()
        || settings_logging_active
        || !additional_drivers.is_empty()
        || device_configurations.is_some();
    if !settings_active {
        return None;
    }
    if stderr_logging_active {
        if filter_enabled("info") {
            let display_path = file_path
                .to_string_lossy()
                .replace("/vulkan/loader_settings.d", "/vulkan//loader_settings.d");
            platform::write_stderr(&format!(
                "[Vulkan Loader] INFO:           Using layer configurations found in loader settings from {display_path}\n"
            ));
        }
        let mut enabled = Vec::new();
        if filter_enabled("error") {
            enabled.push("ERROR");
        }
        if filter_enabled("warn") || filter_enabled("warning") {
            enabled.push("WARNING");
        }
        for (filter, label) in [
            ("info", "INFO"),
            ("debug", "DEBUG"),
            ("perf", "PERF"),
            ("driver", "DRIVER"),
            ("layer", "LAYER"),
        ] {
            if filter_enabled(filter) {
                enabled.push(label);
            }
        }
        if settings_logging_active && filter_enabled("debug") {
            platform::write_stderr(&format!(
                "[Vulkan Loader] DEBUG:          Loader Settings Filters for Logging to Standard Error: {}\n",
                enabled.join(" | ")
            ));
        }
        if filter_enabled("debug")
            && let Some(configurations) = &layer_configurations
        {
            platform::write_stderr(&format!(
                "[Vulkan Loader] DEBUG:          Layer Configurations count = {}\n",
                configurations.len()
            ));
            for (index, configuration) in configurations.iter().enumerate() {
                platform::write_stderr(&format!(
                    "[Vulkan Loader] DEBUG:          ---- Layer Configuration [{index}] ----\n"
                ));
                if configuration.control.as_ref() != "unordered_layer_location" {
                    platform::write_stderr(&format!(
                        "[Vulkan Loader] DEBUG:          Name: {}\n",
                        configuration.name.to_string_lossy()
                    ));
                    platform::write_stderr(&format!(
                        "[Vulkan Loader] DEBUG:          Path: {}\n",
                        configuration.path.to_string_lossy()
                    ));
                    platform::write_stderr(&format!(
                        "[Vulkan Loader] DEBUG:          Layer Type: {}\n",
                        if configuration.treat_as_implicit_manifest {
                            "Implicit"
                        } else {
                            "Explicit"
                        }
                    ));
                }
                platform::write_stderr(&format!(
                    "[Vulkan Loader] DEBUG:          Control: {}\n",
                    configuration.control
                ));
            }
        }
        if filter_enabled("debug") && !additional_drivers.is_empty() {
            platform::write_stderr("[Vulkan Loader] DEBUG:          ----\n");
            platform::write_stderr(&format!(
                "[Vulkan Loader] DEBUG:          Use Additional Drivers Exclusively = {}\n",
                if settings
                    .get("additional_drivers_use_exclusively")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                {
                    "true"
                } else {
                    "false"
                }
            ));
            platform::write_stderr(&format!(
                "[Vulkan Loader] DEBUG:          Additional Driver Configurations count = {}\n",
                additional_drivers.len()
            ));
            for (index, path) in additional_drivers.iter().enumerate() {
                platform::write_stderr(&format!(
                    "[Vulkan Loader] DEBUG:          ---- Driver Configuration [{index}] ----\n"
                ));
                platform::write_stderr(&format!(
                    "[Vulkan Loader] DEBUG:          Path: {}\n",
                    path.to_string_lossy()
                ));
            }
        }
        if filter_enabled("debug")
            && let Some(configurations) = &device_configurations
        {
            platform::write_stderr("[Vulkan Loader] DEBUG:          ----\n");
            platform::write_stderr(&format!(
                "[Vulkan Loader] DEBUG:          Device Configurations count = {}\n",
                configurations.len()
            ));
            for (index, configuration) in configurations.iter().enumerate() {
                platform::write_stderr(&format!(
                    "[Vulkan Loader] DEBUG:          ---- Device Configuration [{index}] ----\n"
                ));
                platform::write_stderr(&format!(
                    "[Vulkan Loader] DEBUG:          deviceUUID: {}\n",
                    format_uuid(&configuration.device_uuid)
                ));
                platform::write_stderr(&format!(
                    "[Vulkan Loader] DEBUG:          driverUUID: {}\n",
                    format_uuid(&configuration.driver_uuid)
                ));
                platform::write_stderr(&format!(
                    "[Vulkan Loader] DEBUG:          driverVersion: {}\n",
                    configuration.driver_version
                ));
                if let Some(name) = &configuration.device_name {
                    platform::write_stderr(&format!(
                        "[Vulkan Loader] DEBUG:          deviceName: {name}\n"
                    ));
                }
                if let Some(name) = &configuration.driver_name {
                    platform::write_stderr(&format!(
                        "[Vulkan Loader] DEBUG:          driverName: {name}\n"
                    ));
                }
            }
        }
        if filter_enabled("debug") {
            platform::write_stderr(
                "[Vulkan Loader] DEBUG:          ---------------------------------\n",
            );
        }
        if let Some(configurations) = &layer_configurations {
            for configuration in configurations {
                if (filter_enabled("warn") || filter_enabled("warning"))
                    && configuration.control.as_ref() != "unordered_layer_location"
                    && !configuration.name.to_bytes().starts_with(b"VK_LAYER_")
                {
                    platform::write_stderr(&format!(
                        "[Vulkan Loader] WARNING:        Layer name {} does not conform to naming standard (Policy #LLP_LAYER_3)\n",
                        configuration.name.to_string_lossy()
                    ));
                }
                if filter_enabled("error")
                    && configuration.control.as_ref() != "unordered_layer_location"
                    && !platform::file_exists(&configuration.path)
                {
                    platform::write_stderr(&format!(
                        "[Vulkan Loader] ERROR:          loader_get_json: Failed to open JSON file {}\n",
                        configuration.path.to_string_lossy()
                    ));
                }
            }
        }
    }
    Some(LoaderSettings {
        settings_file_path: file_path,
        layer_configurations,
        additional_drivers,
        additional_drivers_use_exclusively: settings
            .get("additional_drivers_use_exclusively")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        device_configurations,
    })
}

fn parse_device_configuration(value: &Value) -> Option<DeviceConfiguration> {
    fn uuid(value: &Value) -> Option<[u8; vk::VK_UUID_SIZE as usize]> {
        let values = value.as_array()?;
        if values.len() != vk::VK_UUID_SIZE as usize {
            return None;
        }
        let mut uuid = [0; vk::VK_UUID_SIZE as usize];
        for (destination, value) in uuid.iter_mut().zip(values) {
            let value = value.as_u64()?;
            if value > u64::from(u8::MAX) {
                return None;
            }
            *destination = value as u8;
        }
        Some(uuid)
    }

    let driver_version = value.get("driverVersion")?.as_u64()?;
    if driver_version > u64::from(u32::MAX) {
        return None;
    }

    Some(DeviceConfiguration {
        device_uuid: uuid(value.get("deviceUUID")?)?,
        driver_uuid: uuid(value.get("driverUUID")?)?,
        driver_version: driver_version as u32,
        device_name: value
            .get("deviceName")
            .and_then(Value::as_str)
            .filter(|name| !name.is_empty())
            .map(Into::into),
        driver_name: value
            .get("driverName")
            .and_then(Value::as_str)
            .filter(|name| !name.is_empty())
            .map(Into::into),
    })
}

fn select_settings(root: &Value) -> Option<&Value> {
    if let Some(settings) = root.get("settings") {
        return settings.as_object().map(|_| settings);
    }
    let settings = root.get("settings_array")?.as_array()?;
    let executable = platform::executable_path();
    let global = settings
        .iter()
        .find(|settings| settings.get("app_keys").is_none());
    settings
        .iter()
        .find(|settings| {
            executable.as_ref().is_some_and(|executable| {
                settings
                    .get("app_keys")
                    .and_then(Value::as_array)
                    .is_some_and(|keys| {
                        keys.iter().any(|key| {
                            key.as_str()
                                .is_some_and(|key| Path::new(key) == executable.as_path())
                        })
                    })
            })
        })
        .or(global)
}

fn read_settings_file(path: &Path) -> Option<(PathBuf, Box<[u8]>)> {
    platform::file_exists(path)
        .then(|| platform::read_file(path).map(|bytes| (path.to_owned(), bytes)))?
}

#[cfg(unix)]
fn find_loader_settings_file() -> Option<(PathBuf, Box<[u8]>)> {
    const DIRECTORY_SUFFIX: &str = "vulkan/loader_settings.d";
    const FILE_NAME: &str = "vk_loader_settings.json";

    fn read_from_root(root: &Path) -> Option<(PathBuf, Box<[u8]>)> {
        read_settings_file(&root.join(DIRECTORY_SUFFIX).join(FILE_NAME))
    }

    let secure_environment = !platform::has_elevated_privileges();
    let environment = |name| secure_environment.then(|| env::var_os(name)).flatten();
    let config_home = environment("XDG_CONFIG_HOME");
    let data_home = environment("XDG_DATA_HOME");
    if let Some(path) = config_home.as_deref()
        && let Some(settings) = read_from_root(Path::new(path))
    {
        return Some(settings);
    }
    if let Some(path) = data_home.as_deref()
        && let Some(settings) = read_from_root(Path::new(path))
    {
        return Some(settings);
    }

    if let Some(home) = environment("HOME") {
        if config_home.is_none()
            && let Some(settings) = read_from_root(&PathBuf::from(&home).join(".config"))
        {
            return Some(settings);
        }
        if data_home.is_none()
            && let Some(settings) = read_from_root(&PathBuf::from(home).join(".local/share"))
        {
            return Some(settings);
        }
    }

    if let Some(config_dirs) = environment("XDG_CONFIG_DIRS").filter(|paths| !paths.is_empty()) {
        for path in env::split_paths(&config_dirs) {
            if let Some(settings) = read_from_root(&path) {
                return Some(settings);
            }
        }
    } else if !cfg!(any(
        target_os = "fuchsia",
        target_os = "nto",
        target_os = "qnx"
    )) && let Some(settings) = read_settings_file(Path::new(
        "/etc/xdg/vulkan/loader_settings.d/vk_loader_settings.json",
    )) {
        return Some(settings);
    }

    if cfg!(target_os = "fuchsia") {
        for root in [Path::new("/config"), Path::new("/pkg/data")] {
            if let Some(settings) = read_from_root(root) {
                return Some(settings);
            }
        }
    } else if cfg!(any(target_os = "nto", target_os = "qnx")) {
        if let Some(settings) = read_from_root(Path::new("/etc")) {
            return Some(settings);
        }
    } else {
        // These defaults match an upstream CMake build installed with its
        // default `/usr/local` prefix. `/etc` is EXTRASYSCONFDIR.
        for path in [
            Path::new("/usr/local/etc/vulkan/loader_settings.d/vk_loader_settings.json"),
            Path::new("/etc/vulkan/loader_settings.d/vk_loader_settings.json"),
        ] {
            if let Some(settings) = read_settings_file(path) {
                return Some(settings);
            }
        }
    }

    if let Some(data_dirs) = environment("XDG_DATA_DIRS").filter(|paths| !paths.is_empty()) {
        for path in env::split_paths(&data_dirs) {
            if let Some(settings) = read_from_root(&path) {
                return Some(settings);
            }
        }
    } else if !cfg!(any(
        target_os = "fuchsia",
        target_os = "nto",
        target_os = "qnx"
    )) {
        for path in [
            Path::new("/usr/local/share/vulkan/loader_settings.d/vk_loader_settings.json"),
            Path::new("/usr/share/vulkan/loader_settings.d/vk_loader_settings.json"),
        ] {
            if let Some(settings) = read_settings_file(path) {
                return Some(settings);
            }
        }
    }
    None
}

#[cfg(windows)]
fn find_loader_settings_file() -> Option<(PathBuf, Box<[u8]>)> {
    platform::settings_files()
        .into_iter()
        .find_map(|path| read_settings_file(&path))
}

#[cfg(not(any(unix, windows)))]
fn find_loader_settings_file() -> Option<(PathBuf, Box<[u8]>)> {
    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DriverDisposition {
    Accepted,
    NotSelected,
    Disabled,
}

pub(crate) struct DriverScan {
    pub(crate) manifests: Box<[DriverManifest]>,
    pub(crate) candidates: Box<[(PathBuf, DriverDisposition)]>,
    pub(crate) search_roots: Box<[PathBuf]>,
    #[cfg(windows)]
    pub(crate) registry_diagnostics: Option<platform::RegistryDiagnostics>,
}

#[cfg(windows)]
fn driver_search_roots_with_diagnostics() -> (Box<[PathBuf]>, Option<platform::RegistryDiagnostics>)
{
    let elevated = platform::has_elevated_privileges();
    let override_paths = (!elevated)
        .then(|| env::var_os("VK_DRIVER_FILES").or_else(|| env::var_os("VK_ICD_FILENAMES")))
        .flatten();
    let has_override = override_paths.is_some();
    let (mut roots, diagnostics) = override_paths.map_or_else(
        || {
            let (files, diagnostics) = platform::registry_manifest_files_with_diagnostics("icd.d");
            (files.into_vec(), Some(diagnostics))
        },
        |value| (split_paths(&value).into_vec(), None),
    );
    if !elevated
        && !has_override
        && let Some(value) = env::var_os("VK_ADD_DRIVER_FILES")
    {
        roots.extend(split_paths(&value));
    }
    (roots.into_boxed_slice(), diagnostics)
}

#[cold]
#[inline(never)]
pub(crate) fn scan_drivers() -> DriverScan {
    let settings = loader_settings();
    scan_drivers_with_settings(settings.as_ref())
}

pub(crate) fn scan_drivers_with_settings(settings: Option<&LoaderSettings>) -> DriverScan {
    let elevated = platform::has_elevated_privileges();
    let use_driver_environment =
        settings.is_none_or(|settings| settings.device_configurations.is_none());
    #[cfg(windows)]
    let (roots, registry_diagnostics) = if use_driver_environment {
        driver_search_roots_with_diagnostics()
    } else {
        (default_search_paths("icd.d"), None)
    };
    #[cfg(not(windows))]
    let roots = if use_driver_environment {
        driver_search_roots()
    } else {
        default_search_paths("icd.d")
    };
    let mut files = Vec::new();
    if let Some(settings) = settings {
        for path in settings.additional_drivers.iter().rev() {
            files.extend(platform::manifest_files(path));
        }
    }
    if settings.is_none_or(|settings| {
        !settings.additional_drivers_use_exclusively || settings.additional_drivers.is_empty()
    }) {
        for root in &roots {
            files.extend(platform::manifest_files(root));
        }
    }

    let mut seen = HashSet::default();
    files.retain(|path| seen.insert(path.clone()));
    let select = (!elevated && use_driver_environment)
        .then(|| env::var("VK_LOADER_DRIVERS_SELECT").ok())
        .flatten()
        .filter(|filters| !filters.is_empty());
    let disable = (!elevated && use_driver_environment)
        .then(|| env::var("VK_LOADER_DRIVERS_DISABLE").ok())
        .flatten()
        .filter(|filters| !filters.is_empty());
    let candidates: Box<[_]> = files
        .into_iter()
        .map(|path| {
            let selected = select
                .as_deref()
                .is_some_and(|filters| driver_filter_matches(filters, &path));
            let disposition = if selected {
                DriverDisposition::Accepted
            } else if disable
                .as_deref()
                .is_some_and(|filters| driver_filter_matches(filters, &path))
            {
                DriverDisposition::Disabled
            } else if select.is_some() {
                DriverDisposition::NotSelected
            } else {
                DriverDisposition::Accepted
            };
            (path, disposition)
        })
        .collect();
    let manifests = candidates
        .iter()
        .filter(|(_, disposition)| *disposition == DriverDisposition::Accepted)
        .filter_map(|(path, _)| parse_manifest(path))
        .collect();
    DriverScan {
        manifests,
        candidates,
        search_roots: roots,
        #[cfg(windows)]
        registry_diagnostics,
    }
}

#[cfg(not(windows))]
pub(crate) fn driver_search_roots() -> Box<[PathBuf]> {
    let elevated = platform::has_elevated_privileges();
    let override_paths = (!elevated)
        .then(|| env::var_os("VK_DRIVER_FILES").or_else(|| env::var_os("VK_ICD_FILENAMES")))
        .flatten();
    let has_override = override_paths.is_some();
    let mut roots = override_paths.map_or_else(
        || default_search_paths("icd.d").into_vec(),
        |value| split_paths(&value).into_vec(),
    );
    if !elevated
        && !has_override
        && let Some(value) = env::var_os("VK_ADD_DRIVER_FILES")
    {
        roots.extend(split_paths(&value));
    }
    roots.into_boxed_slice()
}

fn driver_filter_matches(filters: &str, path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    filters.split(',').any(|filter| {
        filter.eq_ignore_ascii_case("~all~")
            || wildcard_matches_ascii(filter.as_bytes(), name.as_bytes())
    })
}

fn wildcard_matches_ascii(pattern: &[u8], value: &[u8]) -> bool {
    let (mut pattern_index, mut value_index) = (0, 0);
    let (mut star, mut retry) = (None, 0);
    while value_index < value.len() {
        if pattern_index < pattern.len()
            && pattern[pattern_index] != b'*'
            && pattern[pattern_index].eq_ignore_ascii_case(&value[value_index])
        {
            pattern_index += 1;
            value_index += 1;
        } else if pattern_index < pattern.len() && pattern[pattern_index] == b'*' {
            star = Some(pattern_index);
            pattern_index += 1;
            retry = value_index;
        } else if let Some(star_index) = star {
            pattern_index = star_index + 1;
            retry += 1;
            value_index = retry;
        } else {
            return false;
        }
    }
    pattern[pattern_index..].iter().all(|byte| *byte == b'*')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_library_with_a_directory_is_manifest_relative() {
        let path = Path::new("/tmp/driver/icd.json");
        assert_eq!(
            resolve_library_path(path, PathBuf::from("./driver.so")),
            Some(PathBuf::from("/tmp/driver/driver.so"))
        );
    }

    #[test]
    fn bare_library_name_uses_the_platform_search_path() {
        let path = Path::new("/tmp/driver/icd.json");
        assert_eq!(
            resolve_library_path(path, PathBuf::from("driver.so")),
            Some(PathBuf::from("driver.so"))
        );
    }

    #[test]
    fn parses_standard_and_variant_api_versions() {
        assert_eq!(parse_api_version(Some("1.3.275")), Some(0x0040_3113));
        assert_eq!(parse_api_version(Some("1.1.3.275")), Some(0x2040_3113));
        assert_eq!(parse_api_version(Some("invalid")), Some(0));
        assert_eq!(parse_api_version(None), None);
    }

    #[test]
    fn manifest_integer_overflow_saturates_at_target_ulong_like_strtoul() {
        let overflow = (u128::from(libc::c_ulong::MAX) + 1).to_string();
        assert_eq!(strtoul_prefix(&overflow), libc::c_ulong::MAX);
        assert_eq!(strtoul_prefix(&format!("-{overflow}")), libc::c_ulong::MAX);
    }

    #[test]
    fn borrowed_layer_extension_array_preserves_entries() {
        let json = r#"[
            {
                "name": "VK_EXT_debug_utils",
                "spec_version": "1",
                "spec_version": "2",
                "entrypoints": ["vkCreateDebugUtilsMessengerEXT"]
            }
        ]"#;
        let raw: &RawValue = serde_json::from_str(json).unwrap();
        let extensions = parse_raw_layer_extensions(Some(raw), false);
        assert_eq!(extensions.len(), 1);
        assert_eq!(extensions[0].name.as_c_str(), c"VK_EXT_debug_utils");
        assert_eq!(extensions[0].spec_version, 1);
        assert_eq!(extensions[0].entrypoints.len(), 1);
    }

    #[test]
    fn borrowed_layer_parser_rejects_missing_api_version() {
        let json = r#"{
            "name": "VK_LAYER_missing_api_version",
            "type": "INSTANCE",
            "library_path": "layer.so",
            "implementation_version": "1",
            "description": "invalid"
        }"#;
        let raw: RawLayerBox<'_> = serde_json::from_str(json).unwrap();
        assert!(
            parse_raw_layer(
                Path::new("layer.json"),
                raw,
                false,
                false,
                vk::VK_API_VERSION_1_0,
            )
            .is_none()
        );
    }

    #[test]
    fn layer_keys_are_case_insensitive_like_upstream_cjson() {
        let json = r#"{
            "NAME": "VK_LAYER_case_insensitive",
            "TYPE": "INSTANCE",
            "LIBRARY_PATH": "layer.so",
            "API_VERSION": "1.0.0",
            "IMPLEMENTATION_VERSION": "1",
            "DESCRIPTION": "valid"
        }"#;
        let raw: RawLayerBox<'_> = serde_json::from_str(json).unwrap();
        let layer = parse_raw_layer(
            Path::new("layer.json"),
            raw,
            false,
            false,
            vk::VK_API_VERSION_1_0,
        )
        .unwrap();
        assert_eq!(layer.name.as_c_str(), c"VK_LAYER_case_insensitive");
    }

    #[test]
    fn device_layer_manifest_is_rejected_like_upstream() {
        let json = r#"{
            "name": "VK_LAYER_device",
            "type": "DEVICE",
            "library_path": "layer.so",
            "api_version": "1.0.0",
            "implementation_version": "1",
            "description": "deprecated"
        }"#;
        let raw: RawLayerBox<'_> = serde_json::from_str(json).unwrap();
        assert!(
            parse_raw_layer(
                Path::new("layer.json"),
                raw,
                false,
                false,
                vk::VK_API_VERSION_1_0,
            )
            .is_none()
        );
    }

    #[test]
    fn component_layers_presence_conflicts_with_library_path() {
        let json = r#"{
            "name": "VK_LAYER_not_a_meta_layer",
            "type": "INSTANCE",
            "library_path": "layer.so",
            "component_layers": [],
            "api_version": "1.0.0",
            "implementation_version": "1",
            "description": "invalid"
        }"#;
        let raw: RawLayerBox<'_> = serde_json::from_str(json).unwrap();
        assert!(
            parse_raw_layer(
                Path::new("layer.json"),
                raw,
                false,
                false,
                vk::VK_API_VERSION_1_0,
            )
            .is_none()
        );
    }

    #[test]
    fn wrong_typed_optional_objects_do_not_reject_explicit_layer() {
        let json = r#"{
            "name": "VK_LAYER_valid",
            "type": "INSTANCE",
            "library_path": "layer.so",
            "api_version": "1.0.0",
            "implementation_version": "1",
            "description": "valid",
            "functions": 7,
            "disable_environment": false,
            "instance_extensions": false
        }"#;
        let raw: RawLayerBox<'_> = serde_json::from_str(json).unwrap();
        let layer = parse_raw_layer(
            Path::new("layer.json"),
            raw,
            false,
            false,
            vk::VK_API_VERSION_1_0,
        )
        .unwrap();
        assert!(layer.instance_extensions.is_empty());
        assert_eq!(layer.disable_environment, None);
        assert_eq!(layer.functions, LayerFunctions::default());
    }

    fn override_manifest(app_keys: &[&str]) -> LayerManifest {
        LayerManifest {
            name: c"VK_LAYER_LUNARG_override".to_owned(),
            manifest_path: PathBuf::from("override.json"),
            library_path: None,
            manifest_version: vk::VK_API_VERSION_1_0,
            api_version: vk::VK_API_VERSION_1_0,
            architecture_supported: true,
            implementation_version: 0,
            description: CString::default(),
            instance_extensions: Box::default(),
            device_extensions: Box::default(),
            enable_environment: None,
            disable_environment: None,
            component_layers: Box::default(),
            blacklisted_layers: Box::default(),
            override_paths: Box::default(),
            app_keys: app_keys.iter().map(PathBuf::from).collect(),
            functions: LayerFunctions::default(),
            pre_instance_functions: PreInstanceFunctions::default(),
            implicit: true,
            settings_control: None,
        }
    }

    #[test]
    fn unavailable_executable_path_preserves_override_manifests() {
        let mut layers = vec![
            override_manifest(&["/pkg/bin/application"]),
            override_manifest(&[]),
        ];
        select_override_layer_for_executable(&mut layers, None);
        assert_eq!(layers.len(), 2);
    }

    #[test]
    fn executable_path_selects_matching_override_before_global() {
        let mut layers = vec![
            override_manifest(&["/pkg/bin/application"]),
            override_manifest(&[]),
        ];
        select_override_layer_for_executable(&mut layers, Some(Path::new("/pkg/bin/application")));
        assert_eq!(layers.len(), 1);
        assert_eq!(layers[0].app_keys[0], Path::new("/pkg/bin/application"));
    }

    #[test]
    fn implicit_discovery_retains_only_meta_layer_components() {
        let mut meta = override_manifest(&[]);
        meta.name = c"VK_LAYER_implicit_meta".to_owned();
        meta.component_layers = [c"VK_LAYER_explicit_component".to_owned()].into();

        let mut component = override_manifest(&[]);
        component.name = c"VK_LAYER_explicit_component".to_owned();
        component.implicit = false;

        let mut unrelated = override_manifest(&[]);
        unrelated.name = c"VK_LAYER_unrelated_explicit".to_owned();
        unrelated.implicit = false;

        let mut manifests = vec![meta, component, unrelated];
        retain_implicit_layers_and_components(&mut manifests);

        assert_eq!(manifests.len(), 2);
        assert_eq!(manifests[0].name.as_c_str(), c"VK_LAYER_implicit_meta");
        assert_eq!(manifests[1].name.as_c_str(), c"VK_LAYER_explicit_component");
    }
}
