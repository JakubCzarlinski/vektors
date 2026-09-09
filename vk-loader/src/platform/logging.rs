//! Loader log filtering and platform output.

use super::LogFilter;
#[cfg(unix)]
use super::environment::inspect_environment;
#[cfg(unix)]
use super::{fputs, stderr_stream};
use crate::debug::diagnostics::LogBuffer;
use core::fmt::Write as _;
#[cfg(not(any(unix, windows)))]
use std::io::Write as _;
#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{
        ERROR_NOT_ENOUGH_MEMORY, ERROR_OUTOFMEMORY, ERROR_SUCCESS, GetLastError,
        INVALID_HANDLE_VALUE, SetLastError,
    },
    Storage::FileSystem::WriteFile,
    System::{
        Console::{GetStdHandle, STD_ERROR_HANDLE},
        Diagnostics::Debug::OutputDebugStringA,
        Environment::GetEnvironmentVariableW,
    },
};

#[cfg(unix)]
pub(crate) fn write_stderr(message: &str) {
    write_stderr_fmt(format_args!("{message}"));
}

#[cfg(unix)]
pub(crate) fn write_stderr_fmt(message: core::fmt::Arguments<'_>) {
    let stream = stderr_stream();
    if stream.is_null() {
        return;
    }
    crate::debug::diagnostics::with_message(message, |message| {
        // SAFETY: `message` is NUL-terminated and the C runtime owns `stream`.
        unsafe { fputs()(message.as_ptr(), stream) };
    });
}

#[cfg(not(unix))]
pub(crate) fn write_stderr_fmt(message: core::fmt::Arguments<'_>) {
    crate::debug::diagnostics::with_text(message, write_stderr);
}

pub(super) fn loader_debug_filter_matches(filters: &str, filter_kind: LogFilter) -> bool {
    filters.split(',').any(|filter| {
        if filter.is_empty() {
            return false;
        }
        // Upstream compares the option length rather than the keyword length.
        // Consequently non-empty, case-sensitive prefixes are accepted.
        if "all".starts_with(filter) {
            return true;
        }
        filter_kind.name().starts_with(filter)
            || (filter_kind == LogFilter::Warning && "warning".starts_with(filter))
            || (filter_kind == LogFilter::Driver
                && ["driver", "implem", "icd"]
                    .into_iter()
                    .any(|keyword| keyword.starts_with(filter)))
    })
}

#[inline(never)]
pub(crate) fn loader_debug_filter_enabled(filter_kind: LogFilter) -> bool {
    let settings_filter = SETTINGS_LOG_FILTER.load(core::sync::atomic::Ordering::Relaxed);
    if settings_filter != NO_SETTINGS_LOG_FILTER {
        return settings_filter & u16::from(filter_kind.bit()) != 0;
    }
    // SAFETY: Loader configuration excludes concurrent environment mutation;
    // the callback only examines the borrowed string.
    unsafe {
        super::inspect_environment_text(c"VK_LOADER_DEBUG", |filters| {
            filters.is_some_and(|filters| loader_debug_filter_matches(filters, filter_kind))
        })
    }
    .unwrap_or_else(|_| {
        crate::pending::mark_json_allocation_failed();
        false
    })
}

pub(crate) fn loader_debug_logging_enabled() -> bool {
    let settings_filter = SETTINGS_LOG_FILTER.load(core::sync::atomic::Ordering::Relaxed);
    if settings_filter != NO_SETTINGS_LOG_FILTER {
        return settings_filter != 0;
    }
    #[cfg(unix)]
    {
        // SAFETY: Loader configuration excludes concurrent environment mutation;
        // the callback only reads the value's length and retains no pointer.
        unsafe {
            inspect_environment(c"VK_LOADER_DEBUG", |filters| {
                filters.is_some_and(|filters| !filters.is_empty())
            })
        }
    }
    #[cfg(windows)]
    {
        let mut value = [0_u16; 1];
        // SAFETY: The static name is terminated and the one-unit output buffer
        // can hold an empty value's terminator. Nonempty values report a larger
        // required length; their contents are not needed.
        let (length, error) = unsafe {
            SetLastError(ERROR_SUCCESS);
            let length = GetEnvironmentVariableW(
                windows_sys::core::w!("VK_LOADER_DEBUG"),
                value.as_mut_ptr(),
                1,
            );
            (length, GetLastError())
        };
        if length == 0 && matches!(error, ERROR_NOT_ENOUGH_MEMORY | ERROR_OUTOFMEMORY) {
            crate::pending::mark_json_allocation_failed();
        }
        length != 0
    }
    #[cfg(not(any(unix, windows)))]
    {
        std::env::var_os("VK_LOADER_DEBUG").is_some_and(|filters| !filters.is_empty())
    }
}

const NO_SETTINGS_LOG_FILTER: u16 = u16::MAX;
static SETTINGS_LOG_FILTER: core::sync::atomic::AtomicU16 =
    core::sync::atomic::AtomicU16::new(NO_SETTINGS_LOG_FILTER);

pub(crate) fn set_loader_settings_log_filter(filter: Option<u8>) {
    SETTINGS_LOG_FILTER.store(
        filter.map_or(NO_SETTINGS_LOG_FILTER, u16::from),
        core::sync::atomic::Ordering::Relaxed,
    );
}

pub(crate) struct SettingsLogFilterReset;

impl Drop for SettingsLogFilterReset {
    fn drop(&mut self) {
        set_loader_settings_log_filter(None);
    }
}

pub(crate) fn reset_loader_settings_log_filter() -> SettingsLogFilterReset {
    set_loader_settings_log_filter(None);
    SettingsLogFilterReset
}

#[inline(never)]
pub(crate) fn write_loader_log(filter_kind: LogFilter, message: core::fmt::Arguments<'_>) {
    if !loader_debug_filter_enabled(filter_kind) {
        return;
    }
    write_loader_log_enabled(filter_kind.label(), message);
}

#[inline(never)]
pub(crate) fn write_loader_log_with_category(
    severity_filter: LogFilter,
    category_filter: LogFilter,
    message: core::fmt::Arguments<'_>,
) {
    if !loader_debug_filter_enabled(severity_filter)
        && !loader_debug_filter_enabled(category_filter)
    {
        return;
    }
    let mut label = LogBuffer::<64>::new();
    let _ = write!(
        label,
        "{} | {}",
        severity_filter.label(),
        category_filter.label()
    );
    write_loader_log_enabled(label.as_str(), message);
}

#[inline(never)]
pub(crate) fn write_loader_category_log(
    category_filter: LogFilter,
    message: core::fmt::Arguments<'_>,
) {
    if loader_debug_filter_enabled(category_filter) {
        write_loader_log_enabled(category_filter.label(), message);
    }
}

#[inline(never)]
pub(crate) fn write_loader_category_log_any(
    category_filters: &[LogFilter],
    category_label: &str,
    message: core::fmt::Arguments<'_>,
) {
    if category_filters
        .iter()
        .copied()
        .any(loader_debug_filter_enabled)
    {
        write_loader_log_enabled(category_label, message);
    }
}

fn write_loader_log_enabled(label: &str, message: core::fmt::Arguments<'_>) {
    let mut text = LogBuffer::<511>::new();
    let _ = text.write_fmt(message);
    // All labels are fixed loader categories, at most two seven-byte names.
    // Prefix (16), label (17), separator (2), message (511), newline (1).
    debug_assert!(label.len() <= 17);
    let mut line = LogBuffer::<547>::new();
    let _ = write!(line, "[Vulkan Loader] {label}: ");
    let padding = 32_usize.saturating_sub(line.len());
    let _ = writeln!(line, "{:padding$}{}", "", text.as_str());
    write_stderr(line.as_str());
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn write_stderr(message: &str) {
    let _ = std::io::stderr().lock().write_all(message.as_bytes());
}

#[cfg(windows)]
pub(crate) fn write_stderr(message: &str) {
    // Upstream writes every loader log message to stderr before mirroring it
    // to the debugger. Death tests and console applications rely on the first
    // channel; GUI debuggers and the Windows parity shim rely on the second.
    // Avoid `std::io::Stderr`, whose reentrant lock uses a Windows TLS index
    // that is not reclaimed when a Rust DLL is unloaded.
    let handle = unsafe { GetStdHandle(STD_ERROR_HANDLE) };
    if !handle.is_null() && handle != INVALID_HANDLE_VALUE {
        let mut remaining = message.as_bytes();
        while !remaining.is_empty() {
            let request = remaining.len().min(u32::MAX as usize) as u32;
            let mut written = 0;
            // SAFETY: The standard-error handle is borrowed, and `remaining`
            // supplies at least `request` readable bytes.
            let succeeded = unsafe {
                WriteFile(
                    handle,
                    remaining.as_ptr().cast(),
                    request,
                    &raw mut written,
                    core::ptr::null_mut(),
                )
            };
            if succeeded == 0 || written == 0 {
                break;
            }
            remaining = &remaining[written as usize..];
        }
    }
    crate::debug::diagnostics::with_message(format_args!("{message}"), |message| {
        // SAFETY: The message is NUL-terminated and remains live for the call.
        unsafe { OutputDebugStringA(message.as_ptr().cast()) };
    });
}
