//! Exhaustive fault injection for Rust allocations; never linked into the loader.

#[cfg(unix)]
use crate::platform;
use crate::{allocation, json, pending};
use core::{
    alloc::{GlobalAlloc, Layout},
    cell::Cell,
    ptr,
};
use std::alloc::System;

#[derive(Clone, Copy)]
struct State {
    armed: bool,
    remaining: usize,
    failed: bool,
    live_bytes: isize,
}

std::thread_local! {
    static STATE: Cell<State> = const { Cell::new(State {
        armed: false, remaining: 0, failed: false, live_bytes: 0,
    }) };
}

struct FaultAllocator;

// Tests using shared loader state must not overlap a thread-local allocation
// sweep: allocation and release of a shared map can occur on different threads.
pub(crate) static SWEEP_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub(crate) fn sweep_operation(mut operation: impl FnMut() -> vk::VkResult) {
    let guard = SWEEP_LOCK.lock().unwrap();
    for index in 0..1024 {
        STATE.set(State {
            armed: true,
            remaining: index,
            failed: false,
            live_bytes: 0,
        });
        let result = operation();
        let state = STATE.replace(State {
            armed: false,
            remaining: 0,
            failed: false,
            live_bytes: 0,
        });
        let expected = if state.failed {
            vk::VkResult::ERROR_OUT_OF_HOST_MEMORY
        } else {
            vk::VkResult::SUCCESS
        };
        assert_eq!(
            result, expected,
            "allocation {index}: incorrect failure propagation"
        );
        assert_eq!(
            state.live_bytes, 0,
            "allocation {index}: rollback leaked storage"
        );
        if !state.failed {
            drop(guard);
            return;
        }
    }
    panic!("allocation sweep did not reach success");
}

#[global_allocator]
static ALLOCATOR: FaultAllocator = FaultAllocator;

fn should_fail() -> bool {
    STATE
        .try_with(|state| {
            let mut value = state.get();
            if !value.armed {
                return false;
            }
            let fail = value.remaining == 0 && !value.failed;
            if fail {
                value.failed = true;
            } else {
                value.remaining = value.remaining.saturating_sub(1);
            }
            state.set(value);
            fail
        })
        .unwrap_or(false)
}

fn account(bytes: isize) {
    let _ = STATE.try_with(|state| {
        let mut value = state.get();
        if value.armed {
            value.live_bytes += bytes;
            state.set(value);
        }
    });
}

// SAFETY: All successful operations delegate to System with the original
// layout. Injected failures return null without changing the old allocation.
unsafe impl GlobalAlloc for FaultAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if should_fail() {
            return ptr::null_mut();
        }
        // SAFETY: The caller supplies a valid nonzero allocation layout.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            account(layout.size() as isize);
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if should_fail() {
            return ptr::null_mut();
        }
        // SAFETY: The caller supplies a valid nonzero allocation layout.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            account(layout.size() as isize);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        account(-(layout.size() as isize));
        // SAFETY: The pointer and layout are those returned by System.
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if should_fail() {
            return ptr::null_mut();
        }
        // SAFETY: The original pointer/layout and new size satisfy realloc's contract.
        let pointer = unsafe { System.realloc(pointer, layout, size) };
        if !pointer.is_null() {
            account(size as isize - layout.size() as isize);
        }
        pointer
    }
}

fn exercise_error_scope() -> vk::VkResult {
    pending::with_json_error_scope(|| {
        match json::parse(br#"{"name":"test"}"#) {
            Ok(value) => drop(value),
            Err(json::Error::OutOfMemory) => {
                pending::mark_json_allocation_failed();
            }
            Err(json::Error::Invalid) => {
                return vk::VkResult::ERROR_INITIALIZATION_FAILED;
            }
        }
        vk::VkResult::SUCCESS
    })
}

fn exercise_owned_storage(scenario: &str) -> vk::VkResult {
    let result = (|| -> Result<(), vk::VkResult> {
        match scenario {
            "owned-format" => {
                drop(crate::debug::diagnostics::try_format(format_args!(
                    "{}:{:2048}",
                    "prefix", "message"
                ))?);
            }
            "owned-path" => {
                drop(allocation::try_path(std::path::Path::new(
                    "manifest/path.json",
                ))?);
            }
            "boxed-string" => {
                drop(allocation::try_box_str("owned UTF-8: λ")?);
            }
            "byte-c-string" => {
                drop(allocation::try_c_string_bytes(b"function-name").map_err(
                    |error| match error {
                        allocation::CStringError::OutOfMemory => {
                            vk::VkResult::ERROR_OUT_OF_HOST_MEMORY
                        }
                        allocation::CStringError::InteriorNul => {
                            vk::VkResult::ERROR_INITIALIZATION_FAILED
                        }
                    },
                )?);
            }
            "nested-collection" => {
                drop(allocation::try_collect_results(
                    (0..9).map(|_| allocation::try_c_string(c"nested")),
                )?);
            }
            "filtered-collection" => {
                drop(allocation::try_collect(
                    (0..17).filter(|value| value % 2 == 0),
                )?);
            }
            _ => return Err(vk::VkResult::ERROR_INITIALIZATION_FAILED),
        }
        Ok(())
    })();
    result.map_or_else(|error| error, |()| vk::VkResult::SUCCESS)
}

#[cfg(unix)]
fn exercise_discovery(scenario: &str, long_path: &str, fixture: &std::ffi::OsStr) -> vk::VkResult {
    pending::with_json_error_scope(|| {
        let valid = match scenario {
            "file" | "long-path" => {
                let path = if scenario == "long-path" {
                    long_path
                } else {
                    concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")
                };
                let bytes = platform::read_file(std::path::Path::new(path));
                let found = bytes.is_some();
                drop(bytes);
                found
            }
            _ => {
                let path = match scenario {
                    "directory" => std::path::Path::new(fixture),
                    _ => std::path::Path::new("manifest.json"),
                };
                let files = platform::manifest_files(path);
                let expected = if scenario == "directory" { 12 } else { 1 };
                let valid = files.len() == expected;
                drop(files);
                valid
            }
        };
        if valid {
            vk::VkResult::SUCCESS
        } else {
            vk::VkResult::ERROR_INITIALIZATION_FAILED
        }
    })
}

fn exercise_allocation_failure(
    scenario: &str,
    index: usize,
    #[cfg(unix)] fixture: &std::ffi::OsStr,
) -> bool {
    #[cfg(unix)]
    let long_path = format!(
        "{}/{}Cargo.toml",
        env!("CARGO_MANIFEST_DIR"),
        "./".repeat(160)
    );
    STATE.set(State {
        armed: true,
        remaining: index,
        failed: false,
        live_bytes: 0,
    });
    let mut reported_failure = false;
    let recovered = match scenario {
        "json" | "json-lossy" => {
            let input: &[u8] = match scenario {
                "json-lossy" => b"\"invalid\xffUTF-8\" trailing",
                _ => br#"{"a":["escaped\ntext",1,2,3,4,5,6,7,8,9],"b":{"c":true},"unicode":"\ud83d\ude00"}"#,
            };
            let result = json::parse(input);
            let recovered = result
                .as_ref()
                .err()
                .is_none_or(|error| *error == json::Error::OutOfMemory);
            reported_failure = result.is_err();
            drop(result);
            recovered
        }
        "string" => {
            let result = allocation::try_c_string(c"allocation ownership");
            let recovered = result
                .as_ref()
                .err()
                .is_none_or(|error| *error == vk::VkResult::ERROR_OUT_OF_HOST_MEMORY);
            reported_failure = result.is_err();
            drop(result);
            recovered
        }
        "scratch" => {
            let result = crate::collections::ScratchArray::<u64, 8>::try_new(32);
            reported_failure = result.is_err();
            drop(result);
            true
        }
        "error-scope" => {
            let result = exercise_error_scope();
            reported_failure = result == vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
            result == vk::VkResult::SUCCESS || reported_failure
        }
        "owned-format"
        | "owned-path"
        | "boxed-string"
        | "byte-c-string"
        | "nested-collection"
        | "filtered-collection" => {
            let result = exercise_owned_storage(scenario);
            reported_failure = result == vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
            result == vk::VkResult::SUCCESS || reported_failure
        }
        #[cfg(unix)]
        "file" | "long-path" | "directory" | "manifest-path" => {
            let result = exercise_discovery(scenario, &long_path, fixture);
            reported_failure = result == vk::VkResult::ERROR_OUT_OF_HOST_MEMORY;
            result == vk::VkResult::SUCCESS || reported_failure
        }
        "message" => {
            let mut delivered = false;
            crate::debug::diagnostics::with_message(
                format_args!("{:2048}", "diagnostic"),
                |message| {
                    delivered = message.to_bytes().len() == 2048;
                },
            );
            reported_failure = !delivered;
            true
        }
        "reallocation" => {
            let mut bytes = Vec::<u8>::new();
            if bytes.try_reserve_exact(16).is_err() {
                reported_failure = true;
                true
            } else {
                bytes.extend_from_slice(b"preserve-content");
                reported_failure = bytes.try_reserve_exact(8192).is_err();
                let preserved = bytes.as_slice() == b"preserve-content";
                drop(bytes);
                preserved
            }
        }
        _ => false,
    };
    let state = STATE.replace(State {
        armed: false,
        remaining: 0,
        failed: false,
        live_bytes: 0,
    });
    assert!(
        recovered,
        "{scenario}, allocation {index}: failure was not reported as OOM"
    );
    assert_eq!(
        reported_failure, state.failed,
        "{scenario}, allocation {index}: injected failure was swallowed"
    );
    assert_eq!(
        state.live_bytes, 0,
        "{scenario}, allocation {index}: allocation leaked during rollback"
    );
    state.failed
}

#[test]
fn sweep_rust_allocation_failures() {
    let guard = SWEEP_LOCK.lock().unwrap();
    let fixture_root = std::path::Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../target/test/allocations/fixtures"
    ));
    std::fs::create_dir_all(fixture_root).unwrap();
    let fixture = fixture_root.join(format!("rust-allocation-{}", std::process::id()));
    std::fs::create_dir(&fixture).unwrap();
    for index in 0..12 {
        std::fs::write(fixture.join(format!("{index}.json")), b"{}").unwrap();
    }
    std::fs::write(fixture.join("ignored.txt"), b"ignored").unwrap();
    let scenarios = [
        "json",
        "json-lossy",
        "string",
        "scratch",
        "error-scope",
        "message",
        "reallocation",
        "owned-format",
        "owned-path",
        "boxed-string",
        "byte-c-string",
        "nested-collection",
        "filtered-collection",
        #[cfg(unix)]
        "file",
        #[cfg(unix)]
        "long-path",
        #[cfg(unix)]
        "directory",
        #[cfg(unix)]
        "manifest-path",
    ];
    for scenario in scenarios {
        let mut completed = false;
        for index in 0..1024 {
            if !exercise_allocation_failure(
                scenario,
                index,
                #[cfg(unix)]
                fixture.as_os_str(),
            ) {
                completed = true;
                break;
            }
        }
        assert!(
            completed,
            "{scenario}: allocation sweep did not reach success"
        );
    }
    for entry in std::fs::read_dir(&fixture).unwrap() {
        std::fs::remove_file(entry.unwrap().path()).unwrap();
    }
    std::fs::remove_dir(fixture).unwrap();
    drop(guard);
}
