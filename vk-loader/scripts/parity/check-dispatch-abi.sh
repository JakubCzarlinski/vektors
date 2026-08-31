#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
upstream_root="${1:-$repo_root/.upstream/vulkan-loader}"
upstream_table="$upstream_root/loader/generated/vk_layer_dispatch_table.h"
upstream_extensions="$upstream_root/loader/generated/vk_loader_extensions.h"
upstream_header="$upstream_root/external/Debug/64/Vulkan-Headers/include/vulkan/vulkan_core.h"
rust_table="$repo_root/vk-loader/src/generated/global_proc_addr.rs"

require_files "$upstream_table" "$upstream_extensions" "$upstream_header" "$rust_table"

registry_version="$(sed -n 's@.*#define <name>VK_HEADER_VERSION</name> \([0-9][0-9]*\)</type>@\1@p' "$repo_root/registry/vk.xml" | head -n 1)"
upstream_version="$(sed -n 's/^#define VK_HEADER_VERSION  *\([0-9][0-9]*\)$/\1/p' "$upstream_header" | head -n 1)"
if [[ "$registry_version" != "$upstream_version" ]]; then
  echo "Vulkan-Headers version mismatch: registry=$registry_version upstream=$upstream_version" >&2
  exit 1
fi

audit_dir="$(mktemp -d)"
trap 'rm -f "$audit_dir"/*; rmdir "$audit_dir"' EXIT

awk '
  /typedef struct VkLayerInstanceDispatchTable_/ { inside = 1; next }
  inside && /^}/ { exit }
  inside && /PFN_/ {
    field = $NF
    sub(/;.*/, "", field)
    print field
  }
' "$upstream_table" > "$audit_dir/c-instance"

awk '
  /pub\(crate\) struct LayerInstanceDispatchTable/ { inside = 1; next }
  inside && /^}/ { exit }
  inside && /pub\(crate\)/ {
    field = $0
    sub(/.*pub\(crate\) /, "", field)
    sub(/:.*/, "", field)
    if (field == "vk_layerGetPhysicalDeviceProcAddr") {
      field = "GetPhysicalDeviceProcAddr"
    } else {
      sub(/^vk/, "", field)
    }
    print field
  }
' "$rust_table" > "$audit_dir/rust-instance"

awk '
  /typedef struct VkLayerDispatchTable_/ { inside = 1; next }
  inside && /^}/ { exit }
  inside && /uint64_t magic/ { print "magic" }
  inside && /PFN_/ {
    field = $NF
    sub(/;.*/, "", field)
    print field
  }
' "$upstream_table" > "$audit_dir/c-device"

awk '
  /pub\(crate\) struct LayerDeviceDispatchTable/ { inside = 1; next }
  inside && /^}/ { exit }
  inside && /pub\(crate\)/ {
    field = $0
    sub(/.*pub\(crate\) /, "", field)
    sub(/:.*/, "", field)
    sub(/^vk/, "", field)
    print field
  }
' "$rust_table" > "$audit_dir/rust-device"

diff -u "$audit_dir/c-instance" "$audit_dir/rust-instance"
diff -u "$audit_dir/c-device" "$audit_dir/rust-device"

awk '
  /struct loader_device_terminator_dispatch/ { inside = 1; next }
  inside && /^}/ { exit }
  inside && /PFN_/ {
    field = $NF
    sub(/;.*/, "", field)
    print field
  }
' "$upstream_extensions" > "$audit_dir/c-terminator"

awk '
  /pub\(crate\) struct IcdDeviceTerminatorDispatchTable/ { inside = 1; next }
  inside && /^}/ { exit }
  inside && /pub\(crate\)/ {
    field = $0
    sub(/.*pub\(crate\) /, "", field)
    sub(/:.*/, "", field)
    sub(/^vk/, "", field)
    if (field != "DestroyDevice") print field
  }
' "$rust_table" > "$audit_dir/rust-terminator"

diff -u "$audit_dir/c-terminator" "$audit_dir/rust-terminator"

echo "dispatch ABI and ICD terminator fields match Vulkan-Loader (Vulkan-Headers $registry_version)"
