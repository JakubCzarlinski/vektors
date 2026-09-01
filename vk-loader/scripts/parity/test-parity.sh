#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
output_dir="${VK_LOADER_TEST_PARITY_DIR:-$repo_root/target/vk-loader-parity}"
upstream_loader="${VK_LOADER_PARITY_UPSTREAM_LIBRARY:-$upstream_build_dir/loader/libvulkan.so}"

"$loader_scripts/parity/check-dispatch-abi.sh" "$upstream_dir"

ensure_upstream_tests test_regression "$upstream_loader"
loader_library="$(resolve_rust_loader "${VK_LOADER_PARITY_RUST_LIBRARY:-}" debug)"

mkdir -p "$output_dir"
summary="$output_dir/status.tsv"
printf 'suite\tselection\tavailable_test_count\tupstream_status\trust_status\n' > "$summary"

run_pair() {
  local suite="$1"
  local selection="$2"
  shift 2
  local upstream_log="$output_dir/$suite.upstream.log"
  local rust_log="$output_dir/$suite.rust.log"
  local test_count
  test_count="$(gtest_case_count "$suite")"

  echo "parity: $suite $selection (upstream, then Rust)"
  set +e
  run_gtest_shards "$upstream_loader" "$suite" "$upstream_log" \
    --gtest_color=no --gtest_brief=1 "$@"
  local upstream_status=$?
  run_gtest_shards "$loader_library" "$suite" "$rust_log" \
    --gtest_color=no --gtest_brief=1 "$@"
  local rust_status=$?
  set -e
  printf '%s\t%s\t%d\t%d\t%d\n' "$suite" "$selection" "$test_count" \
    "$upstream_status" "$rust_status" >> "$summary"

  if (( upstream_status != 0 || rust_status != 0 )); then
    echo "parity failure: upstream=$upstream_status Rust=$rust_status" >&2
    echo "logs: $upstream_log $rust_log" >&2
    return 1
  fi
  rm -f "$upstream_log" "$rust_log"
}

if [[ "${1:-}" == "--full" ]]; then
  parity_status=0
  for suite in test_regression test_fuzzing test_threading; do
    run_pair "$suite" all || parity_status=1
  done
  (( parity_status == 0 ))
else
  filter="${1:-GetProcAddr.GlobalFunctions}"
  run_pair test_regression "$filter" "--gtest_filter=$filter"
fi

echo "paired parity passed (summary: $summary)"
