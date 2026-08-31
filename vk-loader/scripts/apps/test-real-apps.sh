#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
frames="${VK_LOADER_SMOKE_FRAMES:-120}"
timeout_seconds="${VK_LOADER_SMOKE_TIMEOUT:-30}"

require_tools cargo vulkaninfo vkcube timeout

loader="$(resolve_rust_loader "${VK_LOADER_LIBRARY:-}" release)"
require_files "$loader"

smoke_dir="$(mktemp -d)"
ln -s "$loader" "$smoke_dir/libvulkan.so.1"
cleanup() {
  unlink "$smoke_dir/libvulkan.so.1"
  rmdir "$smoke_dir"
}
trap cleanup EXIT

export LD_LIBRARY_PATH="$smoke_dir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

echo "vulkaninfo"
vulkaninfo --summary >/dev/null
vulkaninfo >/dev/null 2>&1

backends=()
if [[ -n "${WAYLAND_DISPLAY:-}" ]]; then
  backends+=(wayland)
fi
if [[ -n "${DISPLAY:-}" ]]; then
  backends+=(xcb xlib)
fi
if ((${#backends[@]} == 0)); then
  echo "no Wayland or X11 display is available; presentation demos skipped"
  exit 0
fi

for backend in "${backends[@]}"; do
  echo "vkcube ($backend, $frames frames)"
  timeout "${timeout_seconds}s" \
    vkcube --wsi "$backend" --c "$frames" --suppress_popups
done

if [[ -n "${WAYLAND_DISPLAY:-}" ]] && command -v vkcubepp >/dev/null; then
  echo "vkcubepp (wayland, $frames frames)"
  timeout "${timeout_seconds}s" \
    vkcubepp --wsi wayland --c "$frames" --suppress_popups
fi

if find /usr/share/vulkan/explicit_layer.d /etc/vulkan/explicit_layer.d \
  -maxdepth 1 -type f -name '*validation*.json' -print -quit 2>/dev/null | grep -q .; then
  echo "vkcube + validation (${backends[0]}, $frames frames)"
  timeout "${timeout_seconds}s" \
    vkcube --wsi "${backends[0]}" --validate --c "$frames" --suppress_popups
fi

if [[ -n "${DISPLAY:-}" ]] && command -v vkgears >/dev/null; then
  echo "vkgears (5 second liveness bound)"
  set +e
  timeout 5s vkgears >/dev/null 2>&1
  gears_status=$?
  set -e
  if [[ "$gears_status" -ne 124 ]]; then
    echo "vkgears exited unexpectedly with status $gears_status" >&2
    exit "$gears_status"
  fi
fi

echo "real-application smoke tests passed"
