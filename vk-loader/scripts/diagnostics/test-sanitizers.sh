#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
target_dir="$repo_root/target/vk-loader-asan"
log_dir="${VK_LOADER_ASAN_LOG_DIR:-$repo_root/target/asan}"

if [[ "$(uname -s)" != "Linux" ]]; then
  echo "AddressSanitizer parity testing currently supports Linux only" >&2
  exit 77
fi

require_tools clang

case "$(uname -m)" in
  x86_64) runtime_name=libclang_rt.asan-x86_64.so ;;
  aarch64) runtime_name=libclang_rt.asan-aarch64.so ;;
  i?86) runtime_name=libclang_rt.asan-i386.so ;;
  *)
    echo "unsupported AddressSanitizer architecture: $(uname -m)" >&2
    exit 77
    ;;
esac

runtime="$(find "$(clang --print-resource-dir)" -type f -name "$runtime_name" -print -quit)"
if [[ -z "$runtime" ]]; then
  echo "Clang AddressSanitizer runtime not found: $runtime_name" >&2
  exit 2
fi

ensure_upstream_tests

target="$(rustc -vV | sed -n 's/^host: //p')"
if [[ -z "$target" ]]; then
  echo "failed to determine the Rust host target" >&2
  exit 2
fi
# Target-scoped flags leave build scripts and proc macros as ordinary host
# binaries. Instrumenting those shared objects makes rustc try to load ASan
# symbols before the compiler process has an ASan runtime.
target_rustflags="CARGO_TARGET_${target^^}_RUSTFLAGS"
target_rustflags="${target_rustflags//-/_}"
target_rustflags="${target_rustflags//./_}"
export "$target_rustflags=-Zsanitizer=address -Cforce-frame-pointers=yes"

RUSTC_BOOTSTRAP=1 \
  CARGO_TARGET_DIR="$target_dir" \
  cargo build --quiet --manifest-path "$repo_root/Cargo.toml" -p vk-loader --release --target "$target"

loader="$target_dir/$target/release/libvulkan.so"
mkdir -p "$log_dir"

if [[ "${1:-}" == "--full" ]]; then
  suites=(test_regression test_fuzzing test_threading)
  filter=()
else
  suites=(test_regression)
  filter=(--gtest_filter='Allocation.*:CreateDevice.*:WsiTests.*')
fi

for suite in "${suites[@]}"; do
  log="$log_dir/$suite.log"
  echo "asan: $suite (log: $log)"
  ASAN_OPTIONS='abort_on_error=1:detect_leaks=0:strict_string_checks=1:check_initialization_order=1' \
    LD_PRELOAD="$runtime" \
    VK_LOADER_TEST_LOADER_PATH="$loader" \
    "$upstream_build_dir/tests/$suite" "${filter[@]}" > "$log" 2>&1
done

echo "AddressSanitizer parity tests passed"
