//! Bounded stderr formatting and fallible callback message storage.

use alloc::{borrow::Cow, string::String, vec::Vec};
use core::{
    ffi::CStr,
    fmt::{self, Write as _},
};
use std::path::Path;

/// Keeps the first N UTF-8 bytes of a diagnostic without allocating. Once a
/// fragment is truncated, later fragments must not fill its unused tail.
pub(crate) struct LogBuffer<const N: usize> {
    bytes: [u8; N],
    len: usize,
    truncated: bool,
}

impl<const N: usize> LogBuffer<N> {
    pub(crate) const fn new() -> Self {
        Self {
            bytes: [0; N],
            len: 0,
            truncated: false,
        }
    }

    pub(crate) const fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }
}

impl<const N: usize> core::fmt::Write for LogBuffer<N> {
    fn write_str(&mut self, value: &str) -> core::fmt::Result {
        if self.truncated {
            return Ok(());
        }
        let mut len = value.len().min(N - self.len);
        self.truncated = len != value.len();
        while !value.is_char_boundary(len) {
            len -= 1;
        }
        self.bytes[self.len..self.len + len].copy_from_slice(&value.as_bytes()[..len]);
        self.len += len;
        Ok(())
    }
}

struct MessageBuffer {
    stack: [u8; 1024],
    len: usize,
    heap: Vec<u8>,
}

struct OwnedMessage(String);

struct SettingsPath<'a>(Cow<'a, str>);

impl fmt::Display for SettingsPath<'_> {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pieces = self.0.split("/vulkan/loader_settings.d");
        if let Some(first) = pieces.next() {
            output.write_str(first)?;
        }
        for piece in pieces {
            output.write_str("/vulkan//loader_settings.d")?;
            output.write_str(piece)?;
        }
        Ok(())
    }
}

/// Preserves upstream's diagnostic double slash without an owned replacement string.
/// Non-Unicode paths need a fallible UTF-8 copy; ordinary paths remain borrowed.
pub(crate) fn settings_path(path: &Path) -> Result<impl fmt::Display + '_, vk::VkResult> {
    let text = match path.to_str() {
        Some(text) => Cow::Borrowed(text),
        None => Cow::Owned(try_format(format_args!("{}", path.display()))?),
    };
    Ok(SettingsPath(text))
}

pub(crate) struct LossyBytes<'a>(pub(crate) &'a [u8]);

impl fmt::Display for LossyBytes<'_> {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut remaining = self.0;
        loop {
            match core::str::from_utf8(remaining) {
                Ok(text) => return output.write_str(text),
                Err(error) => {
                    let valid = error.valid_up_to();
                    // The validator identified this prefix as valid UTF-8.
                    if let Ok(prefix) = core::str::from_utf8(&remaining[..valid]) {
                        output.write_str(prefix)?;
                    }
                    output.write_str("\u{fffd}")?;
                    let Some(invalid) = error.error_len() else {
                        return Ok(());
                    };
                    remaining = &remaining[valid + invalid..];
                }
            }
        }
    }
}

impl fmt::Write for OwnedMessage {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        self.0.try_reserve(value.len()).map_err(|_| fmt::Error)?;
        self.0.push_str(value);
        Ok(())
    }
}

pub(crate) fn try_format(message: fmt::Arguments<'_>) -> Result<String, vk::VkResult> {
    let mut output = OwnedMessage(String::new());
    output
        .write_fmt(message)
        .map_err(|_| vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)?;
    Ok(output.0)
}

pub(crate) fn with_text(message: fmt::Arguments<'_>, submit: impl FnOnce(&str)) {
    let mut buffer = MessageBuffer {
        stack: [0; 1024],
        len: 0,
        heap: Vec::new(),
    };
    if buffer.write_fmt(message).is_err() {
        return;
    }
    let bytes = if buffer.heap.is_empty() {
        &buffer.stack[..buffer.len]
    } else {
        &buffer.heap
    };
    if let Ok(message) = core::str::from_utf8(bytes) {
        submit(message);
    }
}
impl fmt::Write for MessageBuffer {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        if self.heap.is_empty() {
            let len = self.len.checked_add(value.len()).ok_or(fmt::Error)?;
            if len < self.stack.len() {
                self.stack[self.len..len].copy_from_slice(value.as_bytes());
                self.len = len;
                return Ok(());
            }
            self.heap
                .try_reserve(len.checked_add(1).ok_or(fmt::Error)?)
                .map_err(|_| fmt::Error)?;
            self.heap.extend_from_slice(&self.stack[..self.len]);
        } else {
            self.heap
                .try_reserve(value.len().checked_add(1).ok_or(fmt::Error)?)
                .map_err(|_| fmt::Error)?;
        }
        self.heap.extend_from_slice(value.as_bytes());
        Ok(())
    }
}
/// Formats a callback message without invoking the allocation-error handler.
/// The callback borrows the terminated bytes, avoiding a boxed-slice shrink.
pub(crate) fn with_message(message: fmt::Arguments<'_>, submit: impl FnOnce(&CStr)) {
    let mut buffer = MessageBuffer {
        stack: [0; 1024],
        len: 0,
        heap: Vec::new(),
    };
    if buffer.write_fmt(message).is_err() {
        return;
    }
    let bytes = if buffer.heap.is_empty() {
        &buffer.stack[..=buffer.len]
    } else {
        // Every growth reserved the terminator along with the message bytes.
        buffer.heap.push(0);
        &buffer.heap
    };
    if let Ok(message) = CStr::from_bytes_with_nul(bytes) {
        submit(message);
    }
}

#[cfg(test)]
mod tests {
    use super::{LogBuffer, LossyBytes, settings_path, try_format, with_message};
    use core::fmt::Write as _;
    use std::path::Path;

    fn check_settings_path(path: &Path) {
        let expected = path
            .to_string_lossy()
            .replace("/vulkan/loader_settings.d", "/vulkan//loader_settings.d");
        crate::allocation::fault::sweep_operation(|| {
            let display = match settings_path(path) {
                Ok(display) => display,
                Err(error) => return error,
            };
            let mut result = vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
            with_message(format_args!("{display}"), |message| {
                assert_eq!(message.to_bytes(), expected.as_bytes());
                result = vk::VkResult::SUCCESS;
            });
            result
        });
    }

    #[test]
    fn settings_paths_stream_the_exact_upstream_replacement() {
        for path in [
            "",
            "/plain/路径",
            "/vulkan/loader_settings.d/file.json",
            "/vulkan/vulkan/loader_settings.d",
            "/vulkan/loader_settings.d/vulkan/loader_settings.d",
        ] {
            check_settings_path(Path::new(path));
        }
    }

    #[cfg(unix)]
    #[test]
    fn non_unicode_settings_paths_propagate_allocation_failures() {
        use std::{ffi::OsStr, os::unix::ffi::OsStrExt as _};
        check_settings_path(Path::new(OsStr::from_bytes(
            b"/invalid-\xff/vulkan/loader_settings.d/file.json",
        )));
    }

    #[cfg(windows)]
    #[test]
    fn unpaired_surrogate_settings_paths_propagate_allocation_failures() {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt as _};
        let mut units = vec![0xd800];
        units.extend("/vulkan/loader_settings.d/file.json".encode_utf16());
        check_settings_path(Path::new(&OsString::from_wide(&units)));
    }

    #[test]
    fn streamed_lossy_bytes_match_standard_utf8_replacement() {
        for first in u8::MIN..=u8::MAX {
            for second in u8::MIN..=u8::MAX {
                let bytes = [first, second];
                let mut output = LogBuffer::<16>::new();
                write!(&mut output, "{}", LossyBytes(&bytes)).unwrap();
                assert_eq!(output.as_str(), String::from_utf8_lossy(&bytes));
            }
        }
        for bytes in [
            b"\xf0\x9f\x98\x80".as_slice(),
            b"\xed\xa0\x80",
            b"\xf0\x9f\x98",
            b"\xe2\x82",
            b"\xf4\x90\x80\x80",
        ] {
            let mut output = LogBuffer::<16>::new();
            write!(&mut output, "{}", LossyBytes(bytes)).unwrap();
            assert_eq!(output.as_str(), String::from_utf8_lossy(bytes));
        }
    }

    #[test]
    fn callback_message_allocation_failures_drop_the_message_without_leaking() {
        for len in [0, 1023, 1024, 4096] {
            let text = "x".repeat(len);
            crate::allocation::fault::sweep_operation(|| {
                let mut result = vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
                with_message(format_args!("{text}"), |message| {
                    assert_eq!(message.to_bytes(), text.as_bytes());
                    result = vk::VkResult::SUCCESS;
                });
                result
            });
        }
    }

    #[test]
    fn owned_formatting_propagates_each_allocation_failure() {
        let text = "x".repeat(4096);
        crate::allocation::fault::sweep_operation(|| {
            match try_format(format_args!("prefix{text}suffix")) {
                Ok(message) => {
                    assert_eq!(message.len(), text.len() + 12);
                    assert!(message.starts_with("prefix"));
                    assert!(message.ends_with("suffix"));
                    vk::VkResult::SUCCESS
                }
                Err(error) => error,
            }
        });
    }

    #[test]
    fn callback_messages_preserve_contents_across_stack_and_heap_storage() {
        for len in [0, 511, 512, 1022, 1023, 1024, 2048] {
            let text = "x".repeat(len);
            let mut called = false;
            with_message(format_args!("{text}é"), |message| {
                assert_eq!(message.to_bytes(), format!("{text}é").as_bytes());
                called = true;
            });
            assert!(called);
        }
        with_message(format_args!("invalid\0message"), |_| {
            panic!("embedded NUL accepted")
        });
    }
}
