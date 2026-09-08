//! Borrowed Unicode environment inspection with fallible Windows scratch space.

#[cfg(windows)]
use super::environment_predicate::WideBuffer;
use core::ffi::CStr;
#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{
        ERROR_NOT_ENOUGH_MEMORY, ERROR_OUTOFMEMORY, ERROR_SUCCESS, GetLastError, SetLastError,
    },
    System::Environment::GetEnvironmentVariableW,
};

#[derive(Clone, Copy)]
enum TextConversion {
    Strict,
    Lossy,
}

/// Inspects a Unicode value; missing and non-Unicode values become `None`.
///
/// # Safety
/// On Unix, neither another thread nor `inspect` may mutate the environment
/// while its libc-owned value is borrowed.
pub(crate) unsafe fn inspect_environment_text<T>(
    name: &CStr,
    inspect: impl FnOnce(Option<&str>) -> T,
) -> Result<T, vk::VkResult> {
    // SAFETY: Forward the caller's environment lifetime guarantee.
    unsafe { inspect_text(name, TextConversion::Strict, inspect) }
}

/// Inspects a value with invalid Unicode replaced, without an owned OS string.
///
/// # Safety
/// On Unix, the caller and callback must exclude environment mutation.
pub(crate) unsafe fn inspect_environment_lossy<T>(
    name: &CStr,
    inspect: impl FnOnce(Option<&str>) -> T,
) -> Result<T, vk::VkResult> {
    // SAFETY: Forward the caller's environment lifetime guarantee.
    unsafe { inspect_text(name, TextConversion::Lossy, inspect) }
}

unsafe fn inspect_text<T>(
    name: &CStr,
    conversion: TextConversion,
    inspect: impl FnOnce(Option<&str>) -> T,
) -> Result<T, vk::VkResult> {
    #[cfg(unix)]
    {
        // SAFETY: The caller excludes mutation for the entire callback.
        unsafe {
            super::environment::inspect_environment(name, |value| {
                let Some(value) = value else {
                    return Ok(inspect(None));
                };
                match (value.to_str(), conversion) {
                    (Ok(value), _) => Ok(inspect(Some(value))),
                    (Err(_), TextConversion::Strict) => Ok(inspect(None)),
                    (Err(_), TextConversion::Lossy) => {
                        let value = crate::debug::diagnostics::try_format(format_args!(
                            "{}",
                            crate::debug::diagnostics::LossyBytes(value.to_bytes())
                        ))?;
                        Ok(inspect(Some(&value)))
                    }
                }
            })
        }
    }
    #[cfg(windows)]
    {
        windows_text(name, conversion, inspect)
    }
    #[cfg(not(any(unix, windows)))]
    {
        let value = name.to_str().ok().and_then(std::env::var_os);
        let value = value.as_deref().and_then(|value| match conversion {
            TextConversion::Strict => value.to_str().map(alloc::borrow::Cow::Borrowed),
            TextConversion::Lossy => Some(value.to_string_lossy()),
        });
        Ok(inspect(value.as_deref()))
    }
}

#[cfg(windows)]
fn windows_text<T>(
    name: &CStr,
    conversion: TextConversion,
    inspect: impl FnOnce(Option<&str>) -> T,
) -> Result<T, vk::VkResult> {
    let Ok(name) = name.to_str() else {
        return Ok(inspect(None));
    };
    let name_length = name
        .len()
        .checked_add(1)
        .ok_or(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    let mut name_buffer = WideBuffer::new(name_length)?;
    let name_buffer = name_buffer.as_mut_slice();
    for (slot, unit) in name_buffer.iter_mut().zip(name.encode_utf16()) {
        *slot = unit;
    }
    let mut capacity = 256_u32;
    let mut value = WideBuffer::new(capacity as usize)?;
    loop {
        let buffer = &mut value.as_mut_slice()[..capacity as usize];
        // SAFETY: The name is terminated and both buffers remain live for the
        // call. The writable output has exactly the supplied capacity.
        let (length, error) = unsafe {
            SetLastError(ERROR_SUCCESS);
            let length =
                GetEnvironmentVariableW(name_buffer.as_ptr(), buffer.as_mut_ptr(), capacity);
            (length, GetLastError())
        };
        if length == 0 && error != ERROR_SUCCESS {
            return match error {
                ERROR_NOT_ENOUGH_MEMORY | ERROR_OUTOFMEMORY => {
                    Err(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)
                }
                _ => Ok(inspect(None)),
            };
        }
        if length < capacity {
            return inspect_utf16(&buffer[..length as usize], conversion, inspect);
        }
        capacity = length;
        value = WideBuffer::new(capacity as usize)?;
    }
}

#[cfg(windows)]
fn inspect_utf16<T>(
    value: &[u16],
    conversion: TextConversion,
    inspect: impl FnOnce(Option<&str>) -> T,
) -> Result<T, vk::VkResult> {
    let mut length = 0_usize;
    for character in core::char::decode_utf16(value.iter().copied()) {
        let character = match (character, conversion) {
            (Ok(character), _) => character,
            (Err(_), TextConversion::Strict) => return Ok(inspect(None)),
            (Err(_), TextConversion::Lossy) => core::char::REPLACEMENT_CHARACTER,
        };
        length = length
            .checked_add(character.len_utf8())
            .ok_or(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    }
    let mut stack = [0_u8; 1024];
    let mut heap;
    let output = if length <= stack.len() {
        &mut stack[..length]
    } else {
        heap = crate::allocation::try_boxed_slice_filled(length, 0)?;
        &mut heap[..]
    };
    let mut offset = 0;
    // Strict mode rejected invalid units above; lossy mode replaces each one.
    for character in core::char::decode_utf16(value.iter().copied()) {
        let character = character.unwrap_or(core::char::REPLACEMENT_CHARACTER);
        offset += character.encode_utf8(&mut output[offset..]).len();
    }
    // SAFETY: Every byte was written by char::encode_utf8, and the first pass
    // computed the exact total length from this same immutable input.
    Ok(inspect(Some(unsafe {
        core::str::from_utf8_unchecked(output)
    })))
}
