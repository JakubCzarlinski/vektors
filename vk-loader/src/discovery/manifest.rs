//! Driver and layer manifest parsing.

use super::DriverManifest;
use super::DriverManifestError;
use super::LayerAllocationShadow;
use super::LayerExtension;
use super::LayerFunctions;
use super::LayerManifest;
use super::PreInstanceFunctions;
use super::collect_values;
use super::owned_c_string;
use super::owned_path;
use super::parse_json_value;
use super::shadow_layer_json_allocations;
use crate::platform;
use crate::{debug::diagnostics, json::Value, pending};
use alloc::{borrow::Cow, ffi::CString, string::String};
use core::fmt::Write as _;
#[cfg(unix)]
use std::{ffi::OsStr, os::unix::ffi::OsStrExt as _};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};
use vk::VK_MAKE_API_VERSION;

pub(super) fn strtoul_prefix(bytes: &[u8]) -> libc::c_ulong {
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

pub(super) fn parse_api_version(version: Option<&str>) -> Option<u32> {
    parse_api_version_bytes(version.map(str::as_bytes))
}

fn parse_api_version_bytes(version: Option<&[u8]>) -> Option<u32> {
    let version = version?.split(|byte| *byte == 0).next()?;
    let mut components = version.split(|byte| matches!(byte, b'.' | b'"' | b'\n' | b'\r'));
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

fn parse_manifest_u32(value: &[u8]) -> u32 {
    strtoul_prefix(value) as u32
}

pub(super) fn resolve_library_path(manifest_path: &Path, library: PathBuf) -> Option<PathBuf> {
    if library.is_absolute() || library.parent() == Some(Path::new("")) {
        Some(library)
    } else {
        let parent = manifest_path.parent()?;
        crate::allocation::try_join_path(parent, &library)
            .map_err(|_| {
                pending::mark_json_allocation_failed();
            })
            .ok()
    }
}

fn library_architecture_supported(value: Option<&[u8]>) -> bool {
    !matches!(
        (value, core::mem::size_of::<usize>()),
        (Some(value), 8) if value.starts_with(b"32")
    ) && !matches!(
        (value, core::mem::size_of::<usize>()),
        (Some(value), 4) if value.starts_with(b"64")
    )
}

#[cfg(test)]
mod byte_contract_tests {
    use super::{library_architecture_supported, parse_api_version_bytes};

    #[test]
    fn layer_fields_preserve_first_case_insensitive_and_nul_terminated_key() {
        let value = crate::json::parse(br#"{"NAME":null,"name":"later","TyPe\u0000ignored":"GLOBAL","type":"DEVICE","DESCRIPTION":"first","description":"later"}"#).unwrap();
        let raw = super::RawLayer::from_value(&value).unwrap();
        assert_eq!(raw.name, None);
        assert_eq!(raw.layer_type, Some(b"GLOBAL".as_slice()));
        assert_eq!(raw.description, Some(b"first".as_slice()));
    }

    #[test]
    fn control_byte_scan_preserves_word_and_tail_boundaries() {
        for byte in 0..=u8::MAX {
            for position in 0..24 {
                let mut bytes = [b'a'; 32];
                bytes[position] = byte;
                for length in [position + 1, bytes.len()] {
                    assert_eq!(super::contains_control_byte(&bytes[..length]), byte < 32);
                }
            }
        }
        assert!(!super::contains_control_byte(b""));
    }

    #[test]
    fn printed_string_capacity_includes_escapes_and_terminator() {
        for (value, printed_length) in [
            (b"".as_slice(), 0),
            (b"plain".as_slice(), 5),
            (b"\"\\\n".as_slice(), 6),
            (b"\x01\x1f".as_slice(), 12),
            (b"prefix\x00ignored".as_slice(), 6),
            (b"\xff".as_slice(), 1),
        ] {
            assert!(!super::printed_bytes_fit(value, printed_length));
            assert!(!super::printed_bytes_fit(value, printed_length + 1));
            assert!(super::printed_bytes_fit(value, printed_length + 2));
        }
        assert!(super::printed_bytes_fit(&[b'a'; 254], 256));
        assert!(!super::printed_bytes_fit(&[b'a'; 255], 256));
        assert!(!super::printed_bytes_fit(&[b'a'; 256], 256));
    }

    #[test]
    fn architecture_and_version_prefixes_do_not_require_utf8() {
        assert_eq!(
            library_architecture_supported(Some(b"32\xff")),
            core::mem::size_of::<usize>() != 8
        );
        assert_eq!(
            library_architecture_supported(Some(b"64\xff")),
            core::mem::size_of::<usize>() != 4
        );
        assert_eq!(
            parse_api_version_bytes(Some(b"1.3.127\xff")),
            Some(vk::VK_MAKE_API_VERSION(0, 1, 3, 127))
        );
    }
}

fn raw_json_path(value: &Value) -> Option<PathBuf> {
    match value {
        Value::String(value) => owned_byte_path(value),
        Value::Number(value) => {
            if let Ok(value) = diagnostics::try_format(format_args!("{value}")) {
                Some(PathBuf::from(value))
            } else {
                pending::mark_json_allocation_failed();
                None
            }
        }
        Value::Bool(value) => owned_path(Path::new(if *value { "true" } else { "false" })),
        Value::Null => owned_path(Path::new("null")),
        _ => None,
    }
}

pub(super) fn parse_manifest_result(path: &Path) -> Result<DriverManifest, DriverManifestError> {
    let bytes = platform::read_file(path).ok_or(DriverManifestError::FailedOpen)?;
    let root = parse_json_value(&bytes).ok_or(DriverManifestError::InvalidJson)?;
    let file_format_version = root
        .field("file_format_version")
        .and_then(Value::as_bytes)
        .ok_or(DriverManifestError::MissingFileFormatVersion)?;
    let manifest_version =
        parse_api_version_bytes(Some(file_format_version)).ok_or(DriverManifestError::Invalid)?;
    let icd = root.field("ICD").ok_or(DriverManifestError::Invalid)?;
    let library = raw_json_path(
        icd.field("library_path")
            .ok_or(DriverManifestError::Invalid)?,
    )
    .ok_or(DriverManifestError::Invalid)?;
    if library.as_os_str().is_empty() {
        return Err(DriverManifestError::EmptyLibraryPath { manifest_version });
    }
    let library_path = resolve_library_path(path, library).ok_or(DriverManifestError::Invalid)?;
    let api_version = parse_api_version_bytes(icd.field("api_version").and_then(Value::as_bytes))
        .ok_or(DriverManifestError::Invalid)?;
    Ok(DriverManifest {
        manifest_path: owned_path(path).ok_or(DriverManifestError::OutOfMemory)?,
        library_path,
        manifest_version,
        api_version,
        architecture_supported: library_architecture_supported(
            icd.field("library_arch").and_then(Value::as_bytes),
        ),
        portability_driver: icd.field("is_portability_driver").and_then(Value::as_bool)
            == Some(true),
    })
}

pub(crate) fn parse_manifest(path: &Path) -> Option<DriverManifest> {
    parse_manifest_result(path).ok()
}

#[derive(Default)]
pub(super) struct RawLayer<'a> {
    name: Option<&'a [u8]>,
    layer_type: Option<&'a [u8]>,
    library_path: Option<&'a [u8]>,
    api_version: Option<&'a [u8]>,
    library_arch: Option<&'a [u8]>,
    implementation_version: Option<&'a [u8]>,
    description: Option<&'a [u8]>,
    instance_extensions: Option<&'a Value<'a>>,
    device_extensions: Option<&'a Value<'a>>,
    enable_environment: Option<&'a Value<'a>>,
    disable_environment: Option<&'a Value<'a>>,
    component_layers: Option<&'a Value<'a>>,
    blacklisted_layers: Option<&'a Value<'a>>,
    override_paths: Option<&'a Value<'a>>,
    app_keys: Option<&'a Value<'a>>,
    functions: Option<&'a Value<'a>>,
    pre_instance_functions: Option<&'a Value<'a>>,
}
impl<'a> RawLayer<'a> {
    pub(super) fn from_value(value: &'a Value<'a>) -> Option<Self> {
        let object = value.as_object()?;
        let mut fields = [None; 17];
        for (name, value) in object.iter() {
            let name = name.split(|byte| *byte == 0).next().unwrap_or_default();
            let index = match name.len() {
                4 if name.eq_ignore_ascii_case(b"name") => 0,
                4 if name.eq_ignore_ascii_case(b"type") => 1,
                12 if name.eq_ignore_ascii_case(b"library_path") => 2,
                11 if name.eq_ignore_ascii_case(b"api_version") => 3,
                12 if name.eq_ignore_ascii_case(b"library_arch") => 4,
                22 if name.eq_ignore_ascii_case(b"implementation_version") => 5,
                11 if name.eq_ignore_ascii_case(b"description") => 6,
                19 if name.eq_ignore_ascii_case(b"instance_extensions") => 7,
                17 if name.eq_ignore_ascii_case(b"device_extensions") => 8,
                18 if name.eq_ignore_ascii_case(b"enable_environment") => 9,
                19 if name.eq_ignore_ascii_case(b"disable_environment") => 10,
                16 if name.eq_ignore_ascii_case(b"component_layers") => 11,
                18 if name.eq_ignore_ascii_case(b"blacklisted_layers") => 12,
                14 if name.eq_ignore_ascii_case(b"override_paths") => 13,
                8 if name.eq_ignore_ascii_case(b"app_keys") => 14,
                9 if name.eq_ignore_ascii_case(b"functions") => 15,
                22 if name.eq_ignore_ascii_case(b"pre_instance_functions") => 16,
                _ => continue,
            };
            fields[index].get_or_insert(value);
        }
        Some(Self {
            name: fields[0].and_then(Value::as_bytes),
            layer_type: fields[1].and_then(Value::as_bytes),
            library_path: fields[2].and_then(Value::as_bytes),
            api_version: fields[3].and_then(Value::as_bytes),
            library_arch: fields[4].and_then(Value::as_bytes),
            implementation_version: fields[5].and_then(Value::as_bytes),
            description: fields[6].and_then(Value::as_bytes),
            instance_extensions: fields[7],
            device_extensions: fields[8],
            enable_environment: fields[9],
            disable_environment: fields[10],
            component_layers: fields[11],
            blacklisted_layers: fields[12],
            override_paths: fields[13],
            app_keys: fields[14],
            functions: fields[15],
            pre_instance_functions: fields[16],
        })
    }
}

#[derive(Default)]
struct RawLayerFunctions<'a> {
    negotiate: Option<&'a [u8]>,
    get_instance_proc_addr: Option<&'a [u8]>,
    get_device_proc_addr: Option<&'a [u8]>,
    enumerate_instance_extension_properties: Option<&'a [u8]>,
    enumerate_instance_layer_properties: Option<&'a [u8]>,
    enumerate_instance_version: Option<&'a [u8]>,
}
impl<'a> RawLayerFunctions<'a> {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.as_object()?;
        Some(Self {
            negotiate: value
                .field("vkNegotiateLoaderLayerInterfaceVersion")
                .and_then(Value::as_bytes),
            get_instance_proc_addr: value
                .field("vkGetInstanceProcAddr")
                .and_then(Value::as_bytes),
            get_device_proc_addr: value.field("vkGetDeviceProcAddr").and_then(Value::as_bytes),
            enumerate_instance_extension_properties: value
                .field("vkEnumerateInstanceExtensionProperties")
                .and_then(Value::as_bytes),
            enumerate_instance_layer_properties: value
                .field("vkEnumerateInstanceLayerProperties")
                .and_then(Value::as_bytes),
            enumerate_instance_version: value
                .field("vkEnumerateInstanceVersion")
                .and_then(Value::as_bytes),
        })
    }
}

#[derive(Default)]
struct RawLayerExtension<'a> {
    name: Option<&'a [u8]>,
    spec_version: Option<&'a [u8]>,
    entrypoints: Option<&'a Value<'a>>,
}
impl<'a> RawLayerExtension<'a> {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.as_object()?;
        Some(Self {
            name: value.field("name").and_then(Value::as_bytes),
            spec_version: value.field("spec_version").and_then(Value::as_bytes),
            entrypoints: value.field("entrypoints"),
        })
    }
}

struct LayerManifestDocument<'a> {
    file_format_version: Option<&'a [u8]>,
    layer: Option<&'a Value<'a>>,
    layers: Option<&'a [Value<'a>]>,
}
impl<'a> LayerManifestDocument<'a> {
    fn from_value(value: &'a Value<'a>) -> Option<Self> {
        value.as_object()?;
        let layers = match value.field("layers") {
            Some(value) => {
                let values = value.as_array()?;
                if values.iter().any(|value| !value.is_object()) {
                    return None;
                }
                Some(values.as_slice())
            }
            None => None,
        };
        Some(Self {
            file_format_version: value.field("file_format_version").and_then(Value::as_bytes),
            layer: match value.field("layer") {
                Some(value) => {
                    value.as_object()?;
                    Some(value)
                }
                None => None,
            },
            layers,
        })
    }
}

fn borrowed_c_string(value: Option<&[u8]>) -> Option<CString> {
    let value = printed_bytes(value?);
    // SAFETY: printed_bytes truncates at NUL and never introduces a NUL.
    unsafe { copy_printed_c_string(&value) }
}

fn borrowed_c_string_limited(value: Option<&[u8]>, capacity: usize) -> Option<CString> {
    let value = value?;
    if !printed_bytes_fit(value, capacity) {
        return None;
    }
    let value = printed_bytes(value);
    // SAFETY: printed_bytes truncates at NUL and never introduces a NUL.
    unsafe { copy_printed_c_string(&value) }
}

/// Copies an already-printed string, propagating allocation failure to discovery.
///
/// # Safety
///
/// `value` must contain no NUL bytes, as guaranteed by `printed_bytes`.
unsafe fn copy_printed_c_string(value: &[u8]) -> Option<CString> {
    // SAFETY: The caller guarantees the bytes contain no NUL.
    if let Ok(value) = unsafe { crate::allocation::try_c_string_bytes_unchecked(value) } {
        Some(value)
    } else {
        pending::mark_json_allocation_failed();
        None
    }
}

pub(super) fn printed_bytes(value: &[u8]) -> Cow<'_, [u8]> {
    if !contains_control_byte(value) {
        return Cow::Borrowed(value);
    }
    printed_bytes_with_controls(value)
}

/// Tests eight bytes at a time; byte order does not affect an any-match test.
#[inline(never)]
fn contains_control_byte(value: &[u8]) -> bool {
    let (words, remainder) = value.as_chunks::<8>();
    for bytes in words {
        let high = u64::from_ne_bytes(*bytes) & 0xe0e0_e0e0_e0e0_e0e0;
        if high.wrapping_sub(0x0101_0101_0101_0101) & !high & 0x8080_8080_8080_8080 != 0 {
            return true;
        }
    }
    remainder.iter().any(|byte| *byte < 32)
}

#[cold]
fn printed_bytes_with_controls(value: &[u8]) -> Cow<'_, [u8]> {
    let value = value.split(|byte| *byte == 0).next().unwrap_or_default();
    if !value
        .iter()
        .any(|byte| matches!(byte, 0..=0x1f) && !matches!(byte, 8 | 9 | 10 | 12 | 13))
    {
        return Cow::Borrowed(value);
    }
    let mut output = Vec::new();
    let length = value.iter().try_fold(0_usize, |length, byte| {
        length.checked_add(if *byte < 32 && !matches!(byte, 8 | 9 | 10 | 12 | 13) {
            5
        } else {
            1
        })
    });
    if length.is_none_or(|length| output.try_reserve_exact(length).is_err()) {
        pending::mark_json_allocation_failed();
        return Cow::Borrowed(b"");
    }
    for &byte in value {
        let control = byte < 32 && !matches!(byte, 8 | 9 | 10 | 12 | 13);
        if control {
            const HEX: &[u8; 16] = b"0123456789abcdef";
            output.extend_from_slice(&[
                b'u',
                b'0',
                b'0',
                HEX[usize::from(byte >> 4)],
                HEX[usize::from(byte & 15)],
            ]);
        } else {
            output.push(byte);
        }
    }
    Cow::Owned(output)
}

pub(super) fn owned_byte_path(bytes: &[u8]) -> Option<PathBuf> {
    #[cfg(unix)]
    {
        owned_path(Path::new(OsStr::from_bytes(bytes)))
    }
    #[cfg(not(unix))]
    {
        match diagnostics::try_format(format_args!("{}", diagnostics::LossyBytes(bytes))) {
            Ok(path) => Some(PathBuf::from(path)),
            Err(_) => {
                pending::mark_json_allocation_failed();
                None
            }
        }
    }
}

pub(super) fn cjson_string(value: &str) -> Cow<'_, str> {
    let value = cjson_value_string(value);
    if !value
        .as_bytes()
        .iter()
        .any(|byte| matches!(byte, 0..=0x1f) && !matches!(byte, 8 | 9 | 10 | 12 | 13))
    {
        return value;
    }
    let length = value.chars().try_fold(0_usize, |length, character| {
        let extra = if character <= '\u{1f}'
            && !matches!(character, '\u{8}' | '\t' | '\n' | '\u{c}' | '\r')
        {
            5
        } else {
            character.len_utf8()
        };
        length.checked_add(extra)
    });
    let mut printed = String::new();
    if length.is_none_or(|length| printed.try_reserve_exact(length).is_err()) {
        pending::mark_json_allocation_failed();
        return Cow::Borrowed("");
    }
    for character in value.chars() {
        if character <= '\u{1f}' && !matches!(character, '\u{8}' | '\t' | '\n' | '\u{c}' | '\r') {
            let _ = write!(printed, "u{:04x}", u32::from(character));
        } else {
            printed.push(character);
        }
    }
    Cow::Owned(printed)
}

pub(super) fn cjson_value_string(value: &str) -> Cow<'_, str> {
    value
        .split_once('\0')
        .map_or(Cow::Borrowed(value), |(prefix, _)| Cow::Borrowed(prefix))
}

pub(super) fn printed_bytes_fit(value: &[u8], capacity: usize) -> bool {
    // cJSON's printer requests escaped length + 1; ensure adds another byte.
    // Size the original decoded bytes, before loader-specific control printing.
    let length = if contains_control_byte(value) {
        escaped_control_length(value)
    } else {
        let quotes = value
            .iter()
            .filter(|byte| matches!(byte, b'"' | b'\\'))
            .count();
        value.len().checked_add(quotes)
    };
    length
        .and_then(|length| length.checked_add(2))
        .is_some_and(|length| length <= capacity)
}

#[cold]
fn escaped_control_length(value: &[u8]) -> Option<usize> {
    value
        .iter()
        .take_while(|byte| **byte != 0)
        .try_fold(0_usize, |length, byte| {
            length.checked_add(match byte {
                b'"' | b'\\' | 8 | 9 | 10 | 12 | 13 => 2,
                0..=31 => 6,
                _ => 1,
            })
        })
}

fn raw_string_array<T: From<CString>>(value: Option<&Value>) -> Box<[T]> {
    let Some(values) = value.and_then(Value::as_array) else {
        return Box::default();
    };
    if values.iter().any(|value| value.as_bytes().is_none()) {
        return Box::default();
    }
    collect_values(
        values
            .iter()
            .filter_map(|value| borrowed_c_string(value.as_bytes()))
            .map(T::from),
    )
    .unwrap_or_default()
}
fn raw_path_array(value: Option<&Value>) -> Box<[PathBuf]> {
    let Some(values) = value.and_then(Value::as_array) else {
        return Box::default();
    };
    if values.iter().any(|value| value.as_bytes().is_none()) {
        return Box::default();
    }
    collect_values(
        values
            .iter()
            .filter_map(Value::as_bytes)
            .filter_map(|value| owned_byte_path(&printed_bytes(value))),
    )
    .unwrap_or_default()
}
fn raw_environment(value: Option<&Value>) -> Option<(OsString, OsString)> {
    let (name, value) = value?.as_object()?.iter().next()?;
    Some((
        owned_byte_path(name)?.into_os_string(),
        owned_byte_path(value.as_bytes()?)?.into_os_string(),
    ))
}
fn parse_raw_layer_extension(value: &Value) -> Option<LayerExtension> {
    let extension = RawLayerExtension::from_value(value)?;
    let name = extension.name?;
    if !printed_bytes_fit(name, vk::VK_MAX_EXTENSION_NAME_SIZE as usize) {
        return None;
    }
    let name = printed_bytes(name);
    let name = match crate::generated::extension_id_bytes(&name) {
        Some(id) => super::ExtensionName::Known(id),
        None => super::ExtensionName::Unknown(owned_c_string(&name)?),
    };
    Some(LayerExtension {
        name,
        spec_version: extension.spec_version.map_or(0, parse_manifest_u32),
        entrypoints: raw_string_array(extension.entrypoints),
    })
}
pub(super) fn parse_raw_layer_extensions(
    value: Option<&Value>,
    instance: bool,
) -> Box<[LayerExtension]> {
    value
        .and_then(Value::as_array)
        .map(|values| {
            collect_values(values.iter().filter_map(parse_raw_layer_extension).filter(
                |extension| {
                    !instance || crate::wsi_instance_extension_supported(extension.name.as_c_str())
                },
            ))
            .unwrap_or_default()
        })
        .unwrap_or_default()
}

pub(super) fn parse_raw_layer(
    path: &Path,
    layer: &RawLayer<'_>,
    source_index: usize,
    implicit: bool,
    supports_pre_instance: bool,
    manifest_version: u32,
) -> Option<LayerManifest> {
    let has_component_layers = layer.component_layers.is_some();
    let component_layers = raw_string_array(layer.component_layers);
    let name = borrowed_c_string_limited(layer.name, vk::VK_MAX_EXTENSION_NAME_SIZE as usize)?;
    match layer.layer_type? {
        b"INSTANCE" | b"GLOBAL" => {}
        _ => return None,
    }
    let is_override = name.as_c_str() == c"VK_LAYER_LUNARG_override";
    let disable_environment = implicit
        .then(|| raw_environment(layer.disable_environment))
        .flatten();
    if implicit && disable_environment.is_none() {
        return None;
    }
    let library_path = layer
        .library_path
        .and_then(owned_byte_path)
        .and_then(|library| resolve_library_path(path, library));
    let source = match (library_path, has_component_layers, is_override) {
        (Some(path), false, false) => super::LayerSource::Library(path),
        (None, true, false) => super::LayerSource::Meta(component_layers),
        (Some(path), false, true) => super::LayerSource::OverrideLibrary(path),
        (None, true, true) => super::LayerSource::OverrideMeta(component_layers),
        _ => return None,
    };
    let functions = layer.functions.and_then(RawLayerFunctions::from_value);
    let has_pre_instance_functions = layer.pre_instance_functions.is_some();
    let pre_instance = (implicit && supports_pre_instance)
        .then_some(layer.pre_instance_functions)
        .flatten()
        .and_then(RawLayerFunctions::from_value);
    let api_version = parse_api_version_bytes(layer.api_version)?;
    // Preserve rejected records until layer diagnostics are emitted. Upstream
    // logs these conditions inside `loader_read_layer_json`; our discovery
    // phase has no create-info callback chain, so `valid_layer_mask` removes
    // them immediately after `load_active_layers` reports the same messages.
    let library_arch = layer.library_arch;
    let architecture_supported = library_architecture_supported(library_arch);
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
        source_index,
        name_index: core::cell::Cell::new(0),
        name,
        manifest_path: owned_path(path)?,
        source,
        manifest_version,
        api_version,
        architecture_supported,
        implementation_version: parse_manifest_u32(layer.implementation_version?),
        description: borrowed_c_string_limited(
            layer.description,
            vk::VK_MAX_DESCRIPTION_SIZE as usize,
        )?,
        instance_extensions: parse_raw_layer_extensions(layer.instance_extensions, true),
        device_extensions: parse_raw_layer_extensions(layer.device_extensions, false),
        enable_environment: implicit
            .then(|| raw_environment(layer.enable_environment))
            .flatten(),
        disable_environment,
        blacklisted_layers: if is_override {
            raw_string_array(layer.blacklisted_layers)
        } else {
            Box::default()
        },
        override_paths: raw_path_array(layer.override_paths),
        app_keys: layer.app_keys.map(|value| raw_path_array(Some(value))),
        functions,
        pre_instance_functions,
        has_pre_instance_functions,
        implicit,
        settings_control: None,
    })
}

pub(super) fn parse_layer_manifest_inner(
    path: &Path,
    implicit: bool,
    callbacks: Option<*const vk::VkAllocationCallbacks<'static>>,
) -> (Box<[LayerManifest]>, bool, bool) {
    let Some(bytes) = platform::read_file(path) else {
        return (Box::default(), true, false);
    };
    if let Some(callbacks) = callbacks {
        match shadow_layer_json_allocations(path, &bytes, callbacks) {
            LayerAllocationShadow::Continue => {}
            LayerAllocationShadow::Abort => return (Box::default(), true, false),
            LayerAllocationShadow::ReparseForDiagnostics => {
                return (Box::default(), true, true);
            }
        }
    }
    let Some(value) = parse_json_value(&bytes) else {
        return (Box::default(), true, false);
    };
    let Some(root) = LayerManifestDocument::from_value(&value) else {
        return (Box::default(), true, false);
    };
    let Some(manifest_version) = parse_api_version_bytes(root.file_format_version) else {
        return (Box::default(), true, false);
    };
    let major = vk::VK_API_VERSION_MAJOR(manifest_version);
    let minor = vk::VK_API_VERSION_MINOR(manifest_version);
    let patch = vk::VK_API_VERSION_PATCH(manifest_version);
    let known_version = major == 1
        && ((minor == 0 && patch < 2) || (minor == 1 && patch < 3) || (minor == 2 && patch < 2));
    let supports_pre_instance = manifest_version >= VK_MAKE_API_VERSION(0, 1, 1, 2);
    if let Some(layers) = root.layers {
        let source_count = layers.len();
        let mut manifests = Vec::new();
        for (source_index, layer) in layers.iter().enumerate() {
            let raw = RawLayer::from_value(layer);
            let Some(raw) = raw.as_ref() else {
                continue;
            };
            let Some(manifest) = parse_raw_layer(
                path,
                raw,
                source_index,
                implicit,
                supports_pre_instance,
                manifest_version,
            ) else {
                continue;
            };
            if crate::allocation::try_push(&mut manifests, manifest).is_err() {
                pending::mark_json_allocation_failed();
                return (Box::default(), true, false);
            }
        }
        let manifests = super::box_values(manifests);
        let needs_diagnostics = !known_version
            || manifest_version < VK_MAKE_API_VERSION(0, 1, 0, 1)
            || manifests.len() != source_count;
        return (manifests, needs_diagnostics, false);
    }
    let had_layer = root.layer.is_some();
    let manifest = root.layer.and_then(|layer| {
        parse_raw_layer(
            path,
            RawLayer::from_value(layer).as_ref()?,
            0,
            implicit,
            supports_pre_instance,
            manifest_version,
        )
    });
    let manifests: Box<[LayerManifest]> = match manifest {
        Some(manifest) => {
            let Ok(storage) = crate::allocation::try_box_uninit::<[LayerManifest; 1]>() else {
                pending::mark_json_allocation_failed();
                return (Box::default(), true, false);
            };
            Box::write(storage, [manifest])
        }
        None => Box::default(),
    };
    let needs_diagnostics = !known_version || manifests.len() != usize::from(had_layer);
    (manifests, needs_diagnostics, false)
}

pub(crate) fn parse_layer_manifest(path: &Path, implicit: bool) -> Box<[LayerManifest]> {
    parse_layer_manifest_inner(path, implicit, pending::instance_allocator()).0
}

pub(crate) fn reparse_layer_manifest(path: &Path, implicit: bool) -> Box<[LayerManifest]> {
    parse_layer_manifest_inner(path, implicit, None).0
}
