#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
toolchain="$(rustup show active-toolchain | awk '{print $1}')"
toolchain_bin="$(dirname "$(rustup which --toolchain "$toolchain" rustc)")"
host="$($toolchain_bin/rustc -vV | awk '/^host:/ { print $2 }')"

if [[ "$host" == *-apple-darwin ]]; then
  default_target="$host"
else
  default_target=aarch64-apple-darwin
fi
target="${VK_LOADER_APPLE_TARGET:-$default_target}"
case "$target" in
  *-apple-*) ;;
  *)
    echo "APPLE static loader requires an Apple target, got: $target" >&2
    exit 2
    ;;
esac

installed="$(rustup target list --installed --toolchain "$toolchain")"
if ! grep -Fqx "$target" <<< "$installed"; then
  echo "missing Rust target for $toolchain: $target" >&2
  echo "install it with: rustup target add --toolchain $toolchain $target" >&2
  exit 2
fi

target_dir="${VK_LOADER_APPLE_STATIC_TARGET_DIR:-$repo_root/target/vk-loader-apple-static}"
RUSTFLAGS="${RUSTFLAGS:--Dwarnings}" \
  RUSTC="$toolchain_bin/rustc" \
  RUSTDOC="$toolchain_bin/rustdoc" \
  CARGO_TARGET_DIR="$target_dir" \
  "$toolchain_bin/cargo" rustc --quiet --release \
    --manifest-path "$repo_root/Cargo.toml" \
    -p vk-loader \
    --target "$target" \
    --features apple-static-loader \
    --lib \
    --crate-type staticlib

archive="$target_dir/$target/release/libvulkan.a"
if [[ ! -f "$archive" ]]; then
  echo "Cargo did not produce the expected static loader: $archive" >&2
  exit 1
fi

echo "Apple static Vulkan loader: $archive"
echo "Link consumers with the macOS system libraries required by Vulkan-Loader, including CoreFoundation."
