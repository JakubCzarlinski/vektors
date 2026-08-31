#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
toolchain="$(rustup show active-toolchain | awk '{print $1}')"
toolchain_bin="$(dirname "$(rustup which --toolchain "$toolchain" rustc)")"
target_dir="${VK_LOADER_CROSS_TARGET_DIR:-$repo_root/target/vk-loader-cross}"

targets=(
  aarch64-apple-darwin
  aarch64-apple-ios
  aarch64-apple-ios-sim
  aarch64-apple-tvos
  aarch64-apple-visionos
  aarch64-linux-android
  aarch64-unknown-fuchsia
  armv7-unknown-linux-gnueabihf
  i686-pc-windows-gnu
  i686-unknown-linux-gnu
  x86_64-apple-ios
  x86_64-linux-android
  x86_64-pc-windows-gnu
  x86_64-unknown-freebsd
  x86_64-unknown-fuchsia
  x86_64-unknown-netbsd
)

installed="$(rustup target list --installed --toolchain "$toolchain")"
missing=()
for target in "${targets[@]}"; do
  if ! grep -Fqx "$target" <<< "$installed"; then
    missing+=("$target")
  fi
done

if ! rustup component list --installed --toolchain "$toolchain" | grep -Fqx rust-src; then
  echo "missing Rust component for source-built QNX check: rust-src" >&2
  echo "install it with: rustup component add --toolchain $toolchain rust-src" >&2
  exit 2
fi

# QNX standard libraries are not distributed by rustup. Build the legacy QNX
# 7.1 target's standard library from the pinned toolchain sources so the actual
# loader platform branch is type-checked rather than merely cfg-inspected.
echo "cross-check: aarch64-unknown-nto-qnx710 (source-built standard library)"
RUSTC_BOOTSTRAP=1 \
  RUSTFLAGS=-Dwarnings \
  RUSTC="$toolchain_bin/rustc" \
  RUSTDOC="$toolchain_bin/rustdoc" \
  CARGO_TARGET_DIR="$target_dir" \
  "$toolchain_bin/cargo" check --quiet -Z build-std=std,panic_abort \
    --manifest-path "$repo_root/Cargo.toml" \
    -p vk-loader \
    --target aarch64-unknown-nto-qnx710

# These upstream-supported desktop platforms are Rust tier-three targets
# without distributed standard-library artifacts. Build std so their loader
# and WSI branches are type-checked instead of being covered only by cfg tests.
source_built_targets=(
  x86_64-pc-cygwin
  x86_64-unknown-dragonfly
  x86_64-unknown-hurd-gnu
  x86_64-unknown-openbsd
)
for target in "${source_built_targets[@]}"; do
  echo "cross-check: $target (source-built standard library)"
  RUSTC_BOOTSTRAP=1 \
    RUSTFLAGS=-Dwarnings \
    RUSTC="$toolchain_bin/rustc" \
    RUSTDOC="$toolchain_bin/rustdoc" \
    CARGO_TARGET_DIR="$target_dir" \
    "$toolchain_bin/cargo" check --quiet -Z build-std=std,panic_abort \
      --manifest-path "$repo_root/Cargo.toml" \
      -p vk-loader \
      --target "$target"
done
if ((${#missing[@]} != 0)); then
  echo "missing Rust targets for $toolchain: ${missing[*]}" >&2
  echo "install them with: rustup target add --toolchain $toolchain ${missing[*]}" >&2
  exit 2
fi

for target in "${targets[@]}"; do
  echo "cross-check: $target"
  RUSTFLAGS=-Dwarnings \
    RUSTC="$toolchain_bin/rustc" \
    RUSTDOC="$toolchain_bin/rustdoc" \
    CARGO_TARGET_DIR="$target_dir" \
    "$toolchain_bin/cargo" check --quiet \
      --manifest-path "$repo_root/Cargo.toml" \
      -p vk-loader \
      --target "$target"
done

echo "cross-check: aarch64-apple-darwin (static loader)"
RUSTFLAGS=-Dwarnings \
  RUSTC="$toolchain_bin/rustc" \
  RUSTDOC="$toolchain_bin/rustdoc" \
  CARGO_TARGET_DIR="$target_dir" \
  "$toolchain_bin/cargo" check --quiet \
    --manifest-path "$repo_root/Cargo.toml" \
    -p vk-loader \
    --target aarch64-apple-darwin \
    --features apple-static-loader

echo "All ${#targets[@]} distributed, ${#source_built_targets[@]} tier-three, QNX, and the Apple static-loader checks passed"
