#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
output_dir="${VK_LOADER_LAYER_CHAIN_PARITY_DIR:-$repo_root/target/layer-chain-contract-parity}"

# This is deliberately the unchanged upstream regression test. Its two-layer
# unknown-command path observes manifest discovery, reverse library loading,
# top-down callstack construction, nested GPDPA routing, and unload ordering.
VK_LOADER_PARITY_DIFF_DIR="$output_dir" \
  "$loader_scripts/parity/compare-observable-parity.sh" \
  'UnknownFunction.PhysicalDeviceFunctionTwoLayerInterception'

echo "two-layer chain behavior matches upstream"
