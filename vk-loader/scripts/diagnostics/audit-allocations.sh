#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
output_dir="$loader_test_root/allocations/audit"
mkdir -p "$output_dir"
CARGO_TARGET_DIR="$loader_test_root/allocations/tools" \
  cargo run --quiet --manifest-path "$repo_root/Cargo.toml" \
    -p vk-codegen --bin vk-loader-allocation-audit -- "$repo_root/vk-loader" "$@" \
    >"$output_dir/inventory.tsv" 2>"$output_dir/summary.txt"
cat "$output_dir/summary.txt"
echo "Allocation review inventory: $output_dir/inventory.tsv"
