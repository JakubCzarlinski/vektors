# vektors

Rust Vulkan bindings generated from Khronos registry XML, plus generator and
demos.

## Versioning

I plan to move versioning to be pinned to Vulkan. This will happen when I am
happy with the API in this repo.
<!-- generated registry metadata -->
As of 2026-08-29, bindings are generated from the Vulkan 1.4.359 registry.

## Installation

It is not currently possible to publish this crate to crates.io due to the
number of features exposed.

```bash
cargo add vk \
  --git https://github.com/JakubCzarlinski/vektors \
  --tag v0.1.16
```

See [https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html](https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html)
for more details.

## Typical Usage

Add the crate and only the Vulkan features/extensions you need. We recommend
only enabling the extension you actually use - this keeps the namespace cleaner
but also reduces the size of dispatch tables and the function pointers loaded at
runtime. Note that static linking is not planned.

```toml
[dependencies.vk]
package = "vk"
features = [
  "VK_VERSION_1_4",
  "VK_KHR_surface",
  "VK_KHR_swapchain",
]
```

Minimal startup pattern:

```rust
use vk::*;

let lib = VulkanLib::load().expect("load Vulkan loader");
let entry = Entry::new(&lib);

let app = VkApplicationInfo::DEFAULT
    .with_apiVersion(VK_API_VERSION_1_4)
    .with_pApplicationName(c"my-app".as_ptr())
    .with_pEngineName(c"my-engine".as_ptr());

let inst_info = VkInstanceCreateInfo::DEFAULT.with_pApplicationInfo(&app);
let instance = entry.vkCreateInstance(&inst_info, null()).expect("vkCreateInstance");
// Instance dropped here, vkDestroyInstance called automatically.
// Manual call to vkDestroyInstance is permitted, but not required.
// All descendants of instance are also dropped automatically due to lifetime
// tracking.
```

## Design Choices

### Safety

These bindings do not prevent you from calling Vulkan functions in an unsafe
way. For example, attempting to call functions that are not supported by the
current instance/device will result in a panic at runtime. The implementation of
these bindings stores Vulkan function pointers in tables as `Option` types.

In some applicaitons, the cost of a branch on every Vulkan call may be
unacceptable, we use `unwrap_unchecked` to call the function pointers. Some
applications may run with extensions conditionally enabled, in which case,
making the `Option` type useful. It is up to the user to decide how
wish to handle unsupported functions (`None` values in tables). Some strategies
include:

- Group code per supported extension, so that unsupported functions are never
  checked or called.
  - One branch per group, rather than one branch per call.
  - Might introduce some code duplication.
  - More planning is required to group functions by extension, although the docs
    generated should aid in this.

- Check for `None` for each call, such that unsupported functions are never
  called.
  - One branch per call, so potentially more overhead.
  - Less code duplication, but more verbose.
  - Easier to implement, as less consideration is required for enabled
    extensions.

TODO(czarlinski): discuss optionals and arrays.

### Naming

Type, function, member, and top-level constant names stay close to the original
C API so cross-referencing the Vulkan specification remains straightforward.

Associated constants on enum and bitmask newtypes intentionally drop redundant
type prefixes. For example, use `VkStructureType::BUFFER_CREATE_INFO` instead
of `VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO`, and
`VkResult::SUCCESS` instead of `VkResult::VK_SUCCESS`.

Bitmask constants also drop the trailing `_BIT` when it is redundant in Rust:
`VkBufferUsageFlagBits::STORAGE_BUFFER`,
`VkPipelineStageFlagBits2::COMPUTE_SHADER`, and
`VkImageCompressionFixedRateFlagBitsEXT::RATE_1BPC`. Vendor suffixes are kept
when they distinguish extension-provided values.

### Feature Gating

Note that unlike the C headers, and as already mentioned, this repository uses
very extensive use of feature flags. These include Vulkan versions, extensions,
but also internal APIs.

Imagine a case where you have an application for hardware with support for
Vulkan 1.2 + `VK_KHR_dynamic_rendering`. In your `Cargo.toml` you can specify:

```toml
features = [
  "VK_VERSION_1_2",
  "VK_KHR_dynamic_rendering",
]
```

In effect this prevents you from using features from later versions of Vulkan or
other extensions at compile time (as opposed to validation layers).
An added benefit is the instance and device dispatch tables will change to only
load/store functions relevant to the enabled features. You should not, however,
depend on these tables maintaining a stable ABI across feature flags being
changed.

### Hierarchy of Vulkan Objects and Lifetimes

TODO(czarlinski): document this.

## Regenerating bindings

From the workspace root:

```bash
./generate.sh
cargo fmt
```

Set `version` at the top of `generate.sh` before regenerating for a release.

`vk-codegen` emits all generated sources into `vk/`.

## Demos

- `cargo run -p vk-demo-compute`
- `cargo run -p vk-demo-compute-vulkan-1-0`
- `cargo run -p vk-demo-spinning-triangle`

## Workspace layout

- `vk-codegen/`: parses `vk.xml` + `video.xml` into IR and generates Rust bindings.
- `vk/`: generated low-level Vulkan FFI crate (raw handles, structs, commands, enums, consts).
- `vk-alloc/`: allocator utilities built on top of `vk`. This is in early stages of development.
- `vk-demo/`: example applications showing end-to-end Vulkan usage. Each demo is a separate crate.
