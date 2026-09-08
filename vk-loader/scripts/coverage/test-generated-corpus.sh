#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
require_tools python3 ninja
ensure_upstream_tests test_fuzzing_loader_neutral
output="$(test_scratch_dir coverage/corpus)"
loader="$(resolve_rust_loader "${VK_LOADER_PARITY_RUST_LIBRARY:-}" debug)"
python3 "$loader_scripts/coverage/generate-corpus.py" \
  --seeds "$repo_root/vk-loader/tests/corpus" \
  --seeds "$upstream_dir/tests/corpus" \
  --seeds "$upstream_dir/tests/framework/data/fuzz_test_minimized_test_cases" \
  --output-parent "$output" >"$output/generation.json"
corpus="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["directory"])' "$output/generation.json")"
python3 "$loader_scripts/coverage/build-corpus-replay.py" \
  --upstream-build "$upstream_build_dir" --corpus "$corpus/cases" --output "$output/replay"
python3 "$loader_scripts/coverage/test-corpus-parity.py" \
  --runner "$output/replay" --upstream "$upstream_build_dir/loader/libvulkan.so" \
  --rust "$loader" --output "$output/results"
