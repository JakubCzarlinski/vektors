#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
audit_target="$repo_root/target/vk-loader-codegen-audit"
artifact_dir="$audit_target/release/deps"

# `emit-stack-sizes` is a rustc diagnostic flag. RUSTC_BOOTSTRAP is confined
# to this non-shipping audit build; ordinary builds remain stable-only.
RUSTC_BOOTSTRAP=1 \
  CARGO_TARGET_DIR="$audit_target" \
  RUSTFLAGS="-Zemit-stack-sizes" \
  cargo rustc \
    --manifest-path "$repo_root/Cargo.toml" \
    --release \
    -p vk-loader \
    --lib \
    -- \
    --emit=obj,llvm-ir

object="$artifact_dir/vulkan.o"
llvm_ir="$artifact_dir/vulkan.ll"
stack_report="$audit_target/stack-sizes.txt"
assembly_report="$audit_target/hot-paths.asm"

llvm-readelf --stack-sizes "$object" >"$stack_report"
command_lookup_symbol="$(
  llvm-nm --defined-only "$object" |
    awk '$3 ~ /command_lookup$/ && symbol == "" { symbol = $3 } END { print symbol }'
)"
focused_symbols="vkGetInstanceProcAddr,vkGetDeviceProcAddr,vkCreateInstance,vkDestroyInstance,vkEnumeratePhysicalDevices,vkGetPhysicalDeviceProperties,vkCreateDevice,vkDestroyDevice"
if [[ -n "$command_lookup_symbol" ]]; then
  focused_symbols+=",$command_lookup_symbol"
fi
objdump_args=(--disassemble-symbols="$focused_symbols")
case "$(uname -m)" in
  x86_64 | i?86) objdump_args+=(--x86-asm-syntax=intel) ;;
esac
llvm-objdump "${objdump_args[@]}" "$object" >"$assembly_report"

echo "Focused frame sizes:"
rg 'vk(GetInstanceProcAddr|GetDeviceProcAddr|CreateInstance|DestroyInstance|EnumeratePhysicalDevices|GetPhysicalDeviceProperties|CreateDevice|DestroyDevice)$|command_lookup$' "$stack_report"
echo
echo "Generated lookup and availability symbol sizes:"
llvm-nm -S --size-sort "$object" | \
  rg 'EXTENSION_NAMES|COMMAND_(NAMES|TABLE|DISPLACEMENTS|CORE_LEVELS|(INSTANCE|DEVICE)_EXTENSION_(IDS|RANGES))|command_lookup$'
echo
echo "Largest frames in vk-loader object:"
sed -n '/^Stack Sizes:/,$p' "$stack_report" | tail -n +3 | sort -nr -k1,1 | sed -n '1,20p'
echo
echo "LLVM IR: $llvm_ir"
echo "Focused assembly: $assembly_report"
echo "All frame sizes: $stack_report"
