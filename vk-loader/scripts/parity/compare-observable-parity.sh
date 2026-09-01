#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
suite="${VK_LOADER_PARITY_SUITE:-test_regression}"
filter="${1:-DirectDriverLoading.Individual}"
output_dir="${VK_LOADER_PARITY_DIFF_DIR:-$repo_root/target/observable-parity}"
upstream_loader="${VK_LOADER_PARITY_UPSTREAM_LIBRARY:-$upstream_build_dir/loader/libvulkan.so}"

case "$suite" in
  test_regression|test_fuzzing|test_threading) ;;
  *)
    echo "unsupported upstream suite: $suite" >&2
    exit 2
    ;;
esac

ensure_upstream_tests "$suite" "$upstream_loader"
rust_loader="$(resolve_rust_loader "${VK_LOADER_PARITY_RUST_LIBRARY:-}" release)"

mkdir -p "$output_dir"
upstream_raw="$output_dir/upstream.raw.log"
rust_raw="$output_dir/rust.raw.log"
upstream_normalized="$output_dir/upstream.normalized.log"
rust_normalized="$output_dir/rust.normalized.log"
comparison="$output_dir/observable.diff"
status_file="$output_dir/status.tsv"

run_suite() {
  local loader="$1"
  local output="$2"
  VK_LOADER_TEST_LOADER_PATH="$loader" \
    "$upstream_build_dir/tests/$suite" --gtest_color=no "--gtest_filter=$filter" \
    >"$output" 2>&1
}

normalize() {
  local input="$1"
  local output="$2"
  sed -E \
    -e 's/\r$//' \
    -e 's/0x[[:xdigit:]]{8,}/<pointer>/g' \
    -e 's/[0-9]+ ms/<time>/g' \
    -e 's/\[Vulkan Loader Git - Tag: [^,]*, Branch\/Commit: [^]]*\]/[Vulkan Loader Git - Tag: <branch>, Branch\/Commit: <commit>]/g' \
    "$input" >"$output"
}

if [[ "${VK_LOADER_PARITY_QUIET:-0}" != 1 ]]; then
  echo "observable parity: $suite --gtest_filter=$filter"
fi
set +e
run_suite "$upstream_loader" "$upstream_raw"
upstream_status=$?
run_suite "$rust_loader" "$rust_raw"
rust_status=$?
set -e
normalize "$upstream_raw" "$upstream_normalized"
normalize "$rust_raw" "$rust_normalized"

printf 'implementation\texit_status\nupstream\t%d\nrust\t%d\n' \
  "$upstream_status" "$rust_status" >"$status_file"

if (( upstream_status != 0 || rust_status != 0 )); then
  echo "At least one test execution failed (status: $status_file)" >&2
  diff -u "$upstream_normalized" "$rust_normalized" >"$comparison" || true
  exit 1
fi

if diff -u "$upstream_normalized" "$rust_normalized" >"$comparison"; then
  if [[ "${VK_LOADER_KEEP_MATCH_LOGS:-0}" != 1 ]]; then
    rm -f "$upstream_raw" "$rust_raw" "$upstream_normalized" "$rust_normalized" "$comparison"
  fi
  if [[ "${VK_LOADER_PARITY_QUIET:-0}" != 1 ]]; then
    echo "Observable output matches after pointer, timing and build-ID normalization"
  fi
  exit 0
fi

if [[ "${VK_LOADER_PARITY_QUIET:-0}" != 1 ]]; then
  echo "Observable output differs (diff: $comparison)" >&2
  echo "Upstream raw log: $upstream_raw" >&2
  echo "Rust raw log: $rust_raw" >&2
fi
exit 1
