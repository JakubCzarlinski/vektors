#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"

output="$repo_root/vk-loader/src/generated/mod.rs"
manifest="$repo_root/vk-loader/Cargo.toml"
check=0
if [[ "${1:-}" == "--check" ]]; then
  check=1
  temporary="$(mktemp -d)"
  trap 'rm -rf "$temporary"' EXIT
  mkdir -p "$temporary/generated"
  cp "$repo_root/vk-loader/src/generated/"*.rs "$temporary/generated/"
  cp "$manifest" "$temporary/Cargo.toml"
  output="$temporary/generated/mod.rs"
  manifest="$temporary/Cargo.toml"
elif (( $# != 0 )); then
  echo "usage: $0 [--check]" >&2
  exit 2
fi

cargo run \
  --quiet \
  --manifest-path "$repo_root/Cargo.toml" \
  -p vk-codegen \
  --bin vk-loader-codegen \
  -- \
  "$repo_root/registry/vk.xml" \
  "$output" \
  "$manifest"

if (( check )); then
  rustfmt --edition 2024 "$temporary/generated/"*.rs
  diff -ru "$repo_root/vk-loader/src/generated" "$temporary/generated"
  diff -u "$repo_root/vk-loader/Cargo.toml" "$temporary/Cargo.toml"
  echo "Generated loader sources are reproducible"
else
  cargo fmt --manifest-path "$repo_root/Cargo.toml" --all
fi
