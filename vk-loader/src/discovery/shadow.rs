//! Upstream-compatible allocation probes.

use crate::json::{self, Value};
use crate::{allocation, pending, platform};
use std::path::Path;

fn cjson_allocation_count(value: &Value) -> usize {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) => 0,
        Value::String(_) => 1,
        Value::Array(values) => values
            .iter()
            .map(|value| 1usize.saturating_add(cjson_allocation_count(value)))
            .sum(),
        Value::Object(values) => values
            .values()
            .map(|value| 2usize.saturating_add(cjson_allocation_count(value)))
            .sum(),
    }
}

pub(super) fn parse_json_value(bytes: &[u8]) -> Option<Value> {
    match json::parse(bytes) {
        Ok(value) => Some(value),
        Err(json::Error::Invalid) => None,
        Err(json::Error::OutOfMemory) => {
            pending::mark_json_allocation_failed();
            None
        }
    }
}

struct CjsonAllocationParser<'a> {
    bytes: &'a [u8],
    index: usize,
    allocations: usize,
}

impl CjsonAllocationParser<'_> {
    fn skip_whitespace(&mut self) {
        while self
            .bytes
            .get(self.index)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.index += 1;
        }
    }

    fn consume(&mut self, expected: u8) -> bool {
        self.skip_whitespace();
        if self.bytes.get(self.index) != Some(&expected) {
            return false;
        }
        self.index += 1;
        true
    }

    fn string(&mut self) -> bool {
        if !self.consume(b'"') {
            return false;
        }
        let mut escaped = false;
        while let Some(&byte) = self.bytes.get(self.index) {
            self.index += 1;
            match (escaped, byte) {
                (false, b'"') => {
                    self.allocations = self.allocations.saturating_add(1);
                    return true;
                }
                (false, b'\\') => escaped = true,
                (false, 0..=0x1f) => return false,
                (true, _) => escaped = false,
                _ => {}
            }
        }
        false
    }

    fn object(&mut self) -> bool {
        self.index += 1;
        self.skip_whitespace();
        if self.bytes.get(self.index) == Some(&b'}') {
            self.index += 1;
            return true;
        }
        loop {
            self.allocations = self.allocations.saturating_add(1);
            if !self.string() || !self.consume(b':') || !self.value() {
                return false;
            }
            self.skip_whitespace();
            match self.bytes.get(self.index) {
                Some(b'}') => {
                    self.index += 1;
                    return true;
                }
                Some(b',') => self.index += 1,
                _ => return false,
            }
        }
    }

    fn array(&mut self) -> bool {
        self.index += 1;
        self.skip_whitespace();
        if self.bytes.get(self.index) == Some(&b']') {
            self.index += 1;
            return true;
        }
        loop {
            self.allocations = self.allocations.saturating_add(1);
            if !self.value() {
                return false;
            }
            self.skip_whitespace();
            match self.bytes.get(self.index) {
                Some(b']') => {
                    self.index += 1;
                    return true;
                }
                Some(b',') => self.index += 1,
                _ => return false,
            }
        }
    }

    fn primitive(&mut self) -> bool {
        let start = self.index;
        while self
            .bytes
            .get(self.index)
            .is_some_and(|byte| !byte.is_ascii_whitespace() && !matches!(byte, b',' | b']' | b'}'))
        {
            self.index += 1;
        }
        self.index != start
    }

    fn value(&mut self) -> bool {
        self.skip_whitespace();
        match self.bytes.get(self.index) {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => self.string(),
            Some(_) => self.primitive(),
            None => false,
        }
    }
}

fn cjson_partial_allocation_count(bytes: &[u8]) -> usize {
    let mut parser = CjsonAllocationParser {
        bytes,
        index: 0,
        allocations: 1,
    };
    parser.value();
    parser.allocations
}

pub(crate) fn probe_instance_allocation(size: usize) -> bool {
    let Some(callbacks) = pending::instance_allocator() else {
        return true;
    };
    // SAFETY: The create entrypoint retains this callback set until its guard drops.
    unsafe { probe_callback_allocation(callbacks, size) }
}

#[cold]
#[inline(never)]
unsafe fn probe_callback_allocation(
    callbacks: *const vk::VkAllocationCallbacks<'_>,
    size: usize,
) -> bool {
    // SAFETY: The instance-create entry point retains the application callback
    // structure for the complete synchronous discovery operation.
    let callbacks = unsafe { &*callbacks };
    let Some(allocate) = callbacks.pfnAllocation else {
        return true;
    };
    // SAFETY: Vulkan defines the callback contract and the returned allocation
    // is released through the matching callback before this function returns.
    let pointer = unsafe {
        allocate(
            callbacks.pUserData,
            size.max(1),
            allocation::LOADER_ALIGNMENT,
            vk::VkSystemAllocationScope::COMMAND,
        )
    };
    if pointer.is_null() {
        return false;
    }
    if let Some(free) = callbacks.pfnFree {
        // SAFETY: `pointer` came from this exact callback set above.
        unsafe { free(callbacks.pUserData, pointer) };
    }
    true
}

pub(super) fn probe_instance_shrinking_reallocation(
    initial_size: usize,
    final_size: usize,
) -> bool {
    let Some(callbacks) = pending::instance_allocator() else {
        return true;
    };
    // SAFETY: The instance-create entry point retains this callback structure
    // for the complete synchronous settings parse.
    let callbacks = unsafe { &*callbacks };
    let (Some(allocate), Some(reallocate)) = (callbacks.pfnAllocation, callbacks.pfnReallocation)
    else {
        return true;
    };
    // SAFETY: These callbacks and their user data satisfy Vulkan's allocator
    // contract for the duration of the entry point.
    let original = unsafe {
        allocate(
            callbacks.pUserData,
            initial_size,
            allocation::LOADER_ALIGNMENT,
            vk::VkSystemAllocationScope::INSTANCE,
        )
    };
    if original.is_null() {
        return false;
    }
    // SAFETY: `original` was returned by this callback set and remains live.
    let resized = unsafe {
        reallocate(
            callbacks.pUserData,
            original,
            final_size,
            allocation::LOADER_ALIGNMENT,
            vk::VkSystemAllocationScope::INSTANCE,
        )
    };
    let pointer = if resized.is_null() { original } else { resized };
    if let Some(free) = callbacks.pfnFree {
        // SAFETY: A failed reallocation leaves `original` live; a successful
        // one transfers ownership to `resized`.
        unsafe { free(callbacks.pUserData, pointer) };
    }
    !resized.is_null()
}

pub(super) fn shadow_json_allocations(
    display_path: impl core::fmt::Display,
    bytes: &[u8],
) -> Result<Option<Value>, ()> {
    if pending::instance_allocator().is_none() {
        return Ok(None);
    }
    shadow_json_with_callbacks(format_args!("{display_path}"), bytes)
}

// Keep callback-allocation emulation out of ordinary manifest/settings reads.
#[cold]
#[inline(never)]
fn shadow_json_with_callbacks(
    display_path: core::fmt::Arguments<'_>,
    bytes: &[u8],
) -> Result<Option<Value>, ()> {
    if !probe_instance_allocation(bytes.len().saturating_add(1)) {
        platform::write_loader_log(
            platform::LogFilter::Error,
            format_args!(
                "loader_get_json: Failed to allocate memory to read JSON file {display_path}"
            ),
        );
        pending::mark_json_allocation_failed();
        return Err(());
    }
    let value = parse_json_value(bytes);
    let parse_allocations = value.as_ref().map_or_else(
        || cjson_partial_allocation_count(bytes),
        |value| 1usize.saturating_add(cjson_allocation_count(value)),
    );
    for _ in 0..parse_allocations {
        if !probe_instance_allocation(core::mem::size_of::<usize>() * 8) {
            platform::write_loader_log(
                platform::LogFilter::Error,
                format_args!(
                    "loader_get_json: Out of Memory error occurred while parsing JSON file {display_path}."
                ),
            );
            pending::mark_json_allocation_failed();
            return Err(());
        }
    }
    Ok(value)
}

pub(super) enum LayerAllocationShadow {
    Continue,
    Abort,
    ReparseForDiagnostics,
}

pub(super) fn shadow_layer_json_allocations(path: &Path, bytes: &[u8]) -> LayerAllocationShadow {
    let display_path = path.display();
    let value = match shadow_json_allocations(&display_path, bytes) {
        Ok(Some(value)) => value,
        Ok(None) => return LayerAllocationShadow::Continue,
        Err(()) => return LayerAllocationShadow::Abort,
    };
    let layer = value.get("layer").or_else(|| {
        value
            .get("layers")
            .and_then(Value::as_array)
            .and_then(|layers| layers.first())
    });
    let Some(layer) = layer else {
        return LayerAllocationShadow::Continue;
    };
    // Upstream duplicates the manifest filename after locating a layer object.
    // Failure here is silent but aborts this discovery pass.
    if !probe_instance_allocation(1) {
        pending::mark_json_allocation_failed();
        return LayerAllocationShadow::Abort;
    }
    let Some(name) = layer.get("name").and_then(Value::as_str) else {
        return LayerAllocationShadow::Continue;
    };
    if let Some(library_path) = layer.get("library_path").and_then(Value::as_str)
        && !probe_instance_shrinking_reallocation(256, library_path.len().saturating_add(3))
    {
        let nonconforming_name = !name.as_bytes().starts_with(b"VK_LAYER_");
        if nonconforming_name {
            platform::write_loader_log(
                platform::LogFilter::Warning,
                format_args!(
                    "Layer name {name} does not conform to naming standard (Policy #LLP_LAYER_3)"
                ),
            );
        }
        platform::write_loader_log(
            platform::LogFilter::Warning,
            format_args!(
                "Skipping layer \"{name}\" due to problem accessing the library_path value in the manifest JSON file"
            ),
        );
        if nonconforming_name {
            return LayerAllocationShadow::ReparseForDiagnostics;
        }
        pending::mark_json_allocation_failed();
        return LayerAllocationShadow::Abort;
    }
    LayerAllocationShadow::Continue
}
