#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
source_dir="$repo_root/.upstream/sascha-willems-vulkan"
expected_revision="e56bc4f10d5b86b792dbda750e65cd1f9657b053"
wsi="${VK_LOADER_SASCHA_WSI:-}"

require_tools cmake git mold ninja

if [[ ! -d "$source_dir/.git" ]]; then
  git clone --recursive https://github.com/SaschaWillems/Vulkan.git "$source_dir"
fi

actual_revision="$(git -C "$source_dir" rev-parse HEAD)"
if [[ "$actual_revision" != "$expected_revision" ]]; then
  git -C "$source_dir" fetch origin "$expected_revision"
  git -C "$source_dir" checkout --detach "$expected_revision"
fi
git -C "$source_dir" submodule update --init --recursive

if [[ -z "$wsi" ]]; then
  if [[ -n "${WAYLAND_DISPLAY:-}" ]]; then
    wsi="wayland"
  else
    wsi="xcb"
  fi
fi

case "$wsi" in
  wayland|xcb|all) ;;
  *)
    echo "VK_LOADER_SASCHA_WSI must be 'wayland', 'xcb', or 'all', got: $wsi" >&2
    exit 2
    ;;
esac

build_jobs="${VK_LOADER_SASCHA_BUILD_JOBS:-$(loader_test_jobs)}"
[[ "$build_jobs" =~ ^[1-9][0-9]*$ ]] || {
  echo "VK_LOADER_SASCHA_BUILD_JOBS must be a positive integer" >&2
  exit 2
}

build_wsi() {
  local build_wsi="$1"
  local jobs="$2"
  local build_dir="$repo_root/target/sascha-willems-vulkan-$build_wsi"
  local wayland=OFF
  [[ "$build_wsi" == wayland ]] && wayland=ON

  cmake -S "$source_dir" -B "$build_dir" -G Ninja \
    -D CMAKE_BUILD_TYPE=Release \
    -D CMAKE_EXE_LINKER_FLAGS=-fuse-ld=mold \
    -D CMAKE_MODULE_LINKER_FLAGS=-fuse-ld=mold \
    -D CMAKE_SHARED_LINKER_FLAGS=-fuse-ld=mold \
    -D USE_WAYLAND_WSI="$wayland"
  cmake --build "$build_dir" --parallel "$jobs"
  echo "All Sascha Willems examples from $expected_revision built for $build_wsi"
}

if [[ "$wsi" == all ]]; then
  wayland_jobs=$(((build_jobs + 1) / 2))
  xcb_jobs=$((build_jobs / 2))
  (( xcb_jobs == 0 )) && xcb_jobs=1
  build_wsi wayland "$wayland_jobs" &
  wayland_pid=$!
  build_wsi xcb "$xcb_jobs" &
  xcb_pid=$!
  wait "$wayland_pid" "$xcb_pid"
else
  build_wsi "$wsi" "$build_jobs"
fi
