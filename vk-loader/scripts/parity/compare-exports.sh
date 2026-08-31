#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
rust_library="$(rust_loader_library debug)"
upstream_library="$upstream_build_dir/loader/libvulkan.so"

if [[ "$(uname -s)" != "Linux" ]]; then
  echo "compare-exports.sh currently supports ELF/Linux builds" >&2
  exit 2
fi
ensure_upstream_tests test_regression "$upstream_library"
build_rust_loader debug >/dev/null

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
nm -D --defined-only "$upstream_library" | awk '$NF ~ /^vk/ { print $NF }' | sort -u > "$tmp_dir/upstream"
nm -D --defined-only "$rust_library" | awk '$NF ~ /^vk/ { print $NF }' | sort -u > "$tmp_dir/rust"

missing="$tmp_dir/missing"
additional="$tmp_dir/additional"
comm -23 "$tmp_dir/upstream" "$tmp_dir/rust" > "$missing"
comm -13 "$tmp_dir/upstream" "$tmp_dir/rust" > "$additional"

echo "Vulkan symbols missing from the Rust loader:"
cat "$missing"
echo
echo "Additional Vulkan symbols exported by the Rust loader:"
cat "$additional"

if [[ -s "$missing" || -s "$additional" ]]; then
  exit 1
fi

echo
echo "Vulkan ELF exports match the upstream loader"
