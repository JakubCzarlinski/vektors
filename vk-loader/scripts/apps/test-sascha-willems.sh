#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
wsi="${VK_LOADER_SASCHA_WSI:-}"
frames="${VK_LOADER_SASCHA_FRAMES:-1200}"
duration_seconds="${VK_LOADER_SASCHA_DURATION:-30}"
timeout_seconds="${VK_LOADER_SASCHA_TIMEOUT:-60}"
validation="${VK_LOADER_SASCHA_VALIDATION:-0}"

require_tools cargo timeout

if [[ -z "$wsi" ]]; then
  if [[ -n "${WAYLAND_DISPLAY:-}" ]]; then
    wsi="wayland"
  else
    wsi="xcb"
  fi
fi

case "$wsi" in
  wayland)
    [[ -n "${WAYLAND_DISPLAY:-}" ]] || {
      echo "WAYLAND_DISPLAY is required for the Wayland examples" >&2
      exit 2
    }
    ;;
  xcb)
    [[ -n "${DISPLAY:-}" ]] || {
      echo "DISPLAY is required for the XCB examples" >&2
      exit 2
    }
    ;;
  *)
    echo "VK_LOADER_SASCHA_WSI must be 'wayland' or 'xcb', got: $wsi" >&2
    exit 2
    ;;
esac

build_dir="$repo_root/target/sascha-willems-vulkan-$wsi"
if [[ ! -x "$build_dir/bin/triangle" ]]; then
  VK_LOADER_SASCHA_WSI="$wsi" \
    "$loader_scripts/apps/setup-sascha-willems-tests.sh"
fi

rust_loader="$(resolve_rust_loader "${VK_LOADER_LIBRARY:-}" release)"
require_files "$rust_loader"

samples=(
  triangle
  trianglevulkan13
  descriptorsets
  dynamicrendering
  computenbody
  indirectdraw
  multithreading
  raytracingbasic
)
benchmark_args=(-b -bw 0 -br "$duration_seconds" -bfs "$frames")
if [[ "$validation" == "1" ]]; then
  benchmark_args+=(-v)
fi

active_library_dir=""
cleanup_suite() {
  if [[ -n "$active_library_dir" ]]; then
    unlink "$active_library_dir/libvulkan.so.1" 2>/dev/null || true
    rmdir "$active_library_dir" 2>/dev/null || true
    active_library_dir=""
  fi
}
trap cleanup_suite EXIT

run_suite() {
  local label="$1"
  local loader="$2"
  local library_dir
  library_dir="$(mktemp -d)"
  active_library_dir="$library_dir"
  ln -s "$loader" "$library_dir/libvulkan.so.1"

  echo "$label ($wsi, $frames frames per example)"
  local sample
  for sample in "${samples[@]}"; do
    echo "  $sample"
    if ! LD_LIBRARY_PATH="$library_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
      timeout "${timeout_seconds}s" "$build_dir/bin/$sample" "${benchmark_args[@]}"; then
      return 1
    fi
  done
  cleanup_suite
}


run_suite "Rust loader" "$rust_loader"

if [[ "${VK_LOADER_SASCHA_COMPARE_UPSTREAM:-0}" == "1" ]]; then

  upstream_loader="${VK_LOADER_SASCHA_UPSTREAM_LIBRARY:-$repo_root/.upstream/vulkan-loader/build-rust-parity/loader/libvulkan.so.1.4.361}"
  [[ -f "$upstream_loader" ]] || {
    echo "upstream loader not found: $upstream_loader" >&2
    exit 2
  }
  run_suite "Upstream loader" "$upstream_loader"
fi

echo "Sascha Willems behavioural tests passed"
