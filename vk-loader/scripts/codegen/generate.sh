#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"

cargo run \
  --manifest-path "$repo_root/Cargo.toml" \
  -p vk-codegen \
  --bin vk-loader-codegen \
  -- \
  "$repo_root/registry/vk.xml" \
  "$repo_root/vk-loader/src/generated/global_proc_addr.rs" \
  "$repo_root/vk-loader/Cargo.toml"
cargo fmt --manifest-path "$repo_root/Cargo.toml" --all -- --check
