#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
upstream_loader="${VK_LOADER_PARITY_UPSTREAM_LIBRARY:-$upstream_build_dir/loader/libvulkan.so.1.4.361}"

require_tools cargo cc mold pkg-config timeout
ensure_upstream_tests test_regression "$upstream_loader"
rust_loader="$(resolve_rust_loader "${VK_LOADER_PARITY_RUST_LIBRARY:-}" release)"
require_files "$upstream_loader" "$rust_loader"

probe="$(mktemp)"
rust_dir="$(mktemp -d)"
upstream_dir="$(mktemp -d)"
cleanup() {
  unlink "$rust_dir/libvulkan.so.1" 2>/dev/null || true
  unlink "$upstream_dir/libvulkan.so.1" 2>/dev/null || true
  unlink "$probe" 2>/dev/null || true
  rmdir "$rust_dir" "$upstream_dir" 2>/dev/null || true
}
trap cleanup EXIT

cc -fuse-ld=mold -std=c11 -Wall -Wextra -Werror \
  "$repo_root/vk-loader/tests/two_device_smoke.c" \
  $(pkg-config --cflags --libs xcb wayland-client) -lvulkan -o "$probe"
ln -s "$rust_loader" "$rust_dir/libvulkan.so.1"
ln -s "$upstream_loader" "$upstream_dir/libvulkan.so.1"

echo "Rust loader device-group/WSI lifecycle"
LD_LIBRARY_PATH="$rust_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
  timeout 180s "$probe"

echo "Upstream loader device-group/WSI lifecycle"
LD_LIBRARY_PATH="$upstream_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
  timeout 180s "$probe"

echo "device-group/WSI lifecycle parity passed"
