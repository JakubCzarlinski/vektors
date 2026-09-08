//! Process privilege snapshots used during discovery.

use core::cell::Cell;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt as _;
#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::CloseHandle,
    Security::{
        GetSidSubAuthority, GetSidSubAuthorityCount, GetTokenInformation, TOKEN_MANDATORY_LABEL,
        TokenIntegrityLevel,
    },
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

// SECURITY_MAX_SID_SIZE plus the attributes DWORD, aligned for TOKEN_MANDATORY_LABEL.
#[cfg(windows)]
#[repr(C, align(8))]
struct MandatoryLabelBuffer([u8; 72]);

thread_local! {
    static ELEVATED_PRIVILEGES: Cell<Option<bool>> = const { Cell::new(None) };
}

struct ElevatedPrivilegesGuard(Option<bool>);

/// Inspects an environment value without copying it or allowing it to escape.
///
/// # Safety
/// Environment mutation must not race this operation or occur in `inspect`.
#[cfg(unix)]
pub(crate) unsafe fn inspect_environment<T>(
    name: &core::ffi::CStr,
    inspect: impl FnOnce(Option<&core::ffi::CStr>) -> T,
) -> T {
    // SAFETY: The name is terminated, and the caller excludes mutation.
    let value = unsafe { libc::getenv(name.as_ptr()) };
    let value = if value.is_null() {
        None
    } else {
        // SAFETY: getenv returns a terminated value, live until mutation.
        Some(unsafe { core::ffi::CStr::from_ptr(value) })
    };
    inspect(value)
}

/// Copies an environment value without retaining libc-owned storage.
///
/// # Safety
/// On Unix, environment mutation must not race this operation, including
/// mutation from an allocator callback. This is the upstream getenv contract.
pub(crate) unsafe fn environment_value(
    name: &core::ffi::CStr,
) -> Result<Option<std::ffi::OsString>, vk::VkResult> {
    #[cfg(unix)]
    {
        // SAFETY: The caller excludes mutation, including allocator callbacks.
        unsafe {
            inspect_environment(name, |value| {
                value
                    .map(|value| {
                        crate::allocation::try_os_string(std::ffi::OsStr::from_bytes(
                            value.to_bytes(),
                        ))
                    })
                    .transpose()
            })
        }
    }
    #[cfg(windows)]
    {
        // Upstream loader_getenv converts UTF-16 with WideCharToMultiByte(CP_UTF8,
        // 0), replacing unpaired surrogates. Do the same before owning the path
        // or filter; OsString::from_wide would both allocate infallibly and
        // preserve values that upstream cannot expose.
        // SAFETY: The Windows implementation owns its native scratch buffer.
        unsafe {
            super::environment_text::inspect_environment_lossy(name, |value| {
                value
                    .map(|value| crate::allocation::try_os_string(std::ffi::OsStr::new(value)))
                    .transpose()
            })
        }?
    }
    #[cfg(not(any(unix, windows)))]
    {
        Ok(name.to_str().ok().and_then(std::env::var_os))
    }
}

impl Drop for ElevatedPrivilegesGuard {
    fn drop(&mut self) {
        ELEVATED_PRIVILEGES.set(self.0);
    }
}

pub(crate) fn with_elevated_privileges_snapshot<T>(operation: impl FnOnce() -> T) -> T {
    ELEVATED_PRIVILEGES.with(|cached| {
        if cached.get().is_some() {
            return operation();
        }
        let previous = cached.replace(Some(query_elevated_privileges()));
        let _guard = ElevatedPrivilegesGuard(previous);
        operation()
    })
}

#[cfg(unix)]
pub(crate) fn has_elevated_privileges() -> bool {
    ELEVATED_PRIVILEGES
        .get()
        .unwrap_or_else(query_elevated_privileges)
}

#[cfg(unix)]
fn query_elevated_privileges() -> bool {
    // SAFETY: These process identity queries have no pointer contracts. The
    // upstream parity harness interposes them to exercise secure discovery.
    unsafe { libc::geteuid() != libc::getuid() || libc::getegid() != libc::getgid() }
}

#[cfg(windows)]
pub(crate) fn has_elevated_privileges() -> bool {
    ELEVATED_PRIVILEGES
        .get()
        .unwrap_or_else(query_elevated_privileges)
}

#[cfg(windows)]
fn query_elevated_privileges() -> bool {
    const TOKEN_QUERY: u32 = 0x0008;
    const TOKEN_QUERY_SOURCE: u32 = 0x0010;
    const SECURITY_MANDATORY_HIGH_RID: u32 = 0x0000_3000;

    let mut token = core::ptr::null_mut();
    // SAFETY: The pseudo-handle is process-owned and `token` is writable.
    if unsafe {
        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_QUERY | TOKEN_QUERY_SOURCE,
            &mut token,
        )
    } == 0
    {
        return false;
    }

    let mut buffer = MandatoryLabelBuffer([0; 72]);
    let mut required = 0;
    // SAFETY: `buffer` is aligned, writable, and its exact size is supplied.
    let queried = unsafe {
        GetTokenInformation(
            token,
            TokenIntegrityLevel,
            buffer.0.as_mut_ptr().cast(),
            buffer.0.len() as u32,
            &mut required,
        )
    } != 0;
    let elevated = if queried {
        // SAFETY: A successful TokenIntegrityLevel query initialized this
        // prefix as TOKEN_MANDATORY_LABEL.
        let label = unsafe { &*buffer.0.as_ptr().cast::<TOKEN_MANDATORY_LABEL>() };
        // SAFETY: Windows supplied the SID in the successful token query.
        let count = unsafe { GetSidSubAuthorityCount(label.Label.Sid) };
        if count.is_null() || unsafe { *count } == 0 {
            false
        } else {
            // SAFETY: The SID reports at least one sub-authority, and the last
            // index is consequently in bounds.
            let level = unsafe { GetSidSubAuthority(label.Label.Sid, u32::from(*count) - 1) };
            !level.is_null() && unsafe { *level } >= SECURITY_MANDATORY_HIGH_RID
        }
    } else {
        false
    };
    // SAFETY: `token` was returned by OpenProcessToken and is owned here.
    unsafe { CloseHandle(token) };
    elevated
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn has_elevated_privileges() -> bool {
    false
}

#[cfg(not(any(unix, windows)))]
const fn query_elevated_privileges() -> bool {
    false
}
