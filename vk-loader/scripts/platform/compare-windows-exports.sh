#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
target="${1:-x86_64-pc-windows-gnu}"
tool_prefix="${target%-pc-windows-gnu}-w64-mingw32"
objdump="${OBJDUMP:-$tool_prefix-objdump}"
definition="$repo_root/.upstream/vulkan-loader/loader/vulkan-1.def"
library="$repo_root/target/$target/release/vulkan.dll"

require_tools rustup
toolchain="$(rustup show active-toolchain | awk '{print $1}')"
toolchain_bin="$(dirname "$(rustup which --toolchain "$toolchain" rustc)")"

if [[ ! -f "$definition" ]]; then
  "$loader_scripts/parity/setup-upstream-tests.sh"
fi

PATH="$toolchain_bin:$PATH" RUSTC="$toolchain_bin/rustc" "$toolchain_bin/cargo" build \
  --quiet \
  --manifest-path "$repo_root/Cargo.toml" \
  -p vk-loader \
  --target "$target" \
  --release

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

awk '
  found && NF && $1 !~ /^;/ { print $1 }
  /^EXPORTS/ { found = 1 }
' "$definition" | sort -u > "$tmp_dir/upstream"

"$objdump" -p "$library" | awk '
  /\[Ordinal\/Name Pointer\] Table/ { found = 1; next }
  found && /^\t\[/ { print $NF }
' | sort -u > "$tmp_dir/rust"

diff -u "$tmp_dir/upstream" "$tmp_dir/rust"
echo "Windows exports match Vulkan-Loader ($(wc -l < "$tmp_dir/rust") symbols)"
