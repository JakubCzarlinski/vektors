#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
target=x86_64-unknown-linux-gnu
target_dir="${VK_LOADER_PORTABLE_TARGET_DIR:-$repo_root/target/vk-loader-glibc217}"
linker="$loader_scripts/platform/zig-cc-glibc-2.17.sh"

require_tools readelf sort zig

CARGO_TARGET_DIR="$target_dir" \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="$linker" \
  cargo build --quiet \
    --manifest-path "$repo_root/Cargo.toml" \
    -p vk-loader \
    --release \
    --target "$target"

loader="$target_dir/$target/release/libvulkan.so"
latest_glibc="$({
  readelf --version-info --wide "$loader" |
    awk '/Name: GLIBC_[0-9]/{ print $3 }'
  echo GLIBC_2.2.5
} | sort -V | tail -n 1)"
maximum_glibc=GLIBC_2.17
if [[ "$(printf '%s\n' "$latest_glibc" "$maximum_glibc" | sort -V | tail -n 1)" != "$maximum_glibc" ]]; then
  echo "portable loader requires unexpected libc version: $latest_glibc" >&2
  exit 1
fi

echo "Portable Linux loader: $loader"
echo "Maximum required libc version: $latest_glibc"
