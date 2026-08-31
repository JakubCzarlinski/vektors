#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
source_dir="$repo_root/.upstream/sascha-willems-vulkan"
expected_revision="e56bc4f10d5b86b792dbda750e65cd1f9657b053"
wsi="${VK_LOADER_SASCHA_WSI:-}"

require_tools cmake git ninja

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
  wayland) wsi_options=(-DUSE_WAYLAND_WSI=ON) ;;
  xcb) wsi_options=(-DUSE_WAYLAND_WSI=OFF) ;;
  *)
    echo "VK_LOADER_SASCHA_WSI must be 'wayland' or 'xcb', got: $wsi" >&2
    exit 2
    ;;
esac

build_dir="$repo_root/target/sascha-willems-vulkan-$wsi"
targets=(
  triangle
  trianglevulkan13
  descriptorsets
  dynamicrendering
  computenbody
  indirectdraw
  multithreading
  raytracingbasic
)

cmake -S "$source_dir" -B "$build_dir" -G Ninja \
  -D CMAKE_BUILD_TYPE=Release "${wsi_options[@]}"
cmake --build "$build_dir" --parallel --target "${targets[@]}"

echo "Sascha Willems examples $expected_revision built for $wsi"
