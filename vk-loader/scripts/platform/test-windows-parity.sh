#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
target="${VK_LOADER_WINDOWS_TARGET:-x86_64-pc-windows-gnu}"
test_dir="$repo_root/.upstream/vulkan-loader/build-windows-rust-parity/tests"
regression="$test_dir/test_regression.exe"
unicode_icd="$test_dir/framework/icd/lib🌋.dll"

require_tools rustup winepath rg
if command -v wine64 >/dev/null; then
  wine_runner=wine64
elif command -v wine >/dev/null; then
  wine_runner=wine
else
  echo "required tool not found: wine64 or wine" >&2
  exit 2
fi
if [[ ! -x "$regression" || ! -f "$unicode_icd" ]]; then
  "$loader_scripts/platform/setup-upstream-windows-tests.sh"
fi

toolchain="$(rustup show active-toolchain | awk '{print $1}')"
toolchain_bin="$(dirname "$(rustup which --toolchain "$toolchain" rustc)")"
PATH="$toolchain_bin:$PATH" RUSTC="$toolchain_bin/rustc" "$toolchain_bin/cargo" build \
  --quiet \
  --manifest-path "$repo_root/Cargo.toml" \
  -p vk-loader \
  --target "$target"

shim_dir="$(winepath -w "$test_dir/framework/shim")"
loader_dll="$(winepath -w "$repo_root/target/$target/debug/vulkan.dll")"
wine_path="$shim_dir;Z:\\usr\\x86_64-w64-mingw32\\bin"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

run_case() {
  local executable="$1"
  local filter="$2"
  local output="$3"
  (
    cd "$test_dir"
    WINEDEBUG=-all WINEPATH="$wine_path" "$wine_runner" cmd /c \
      "set VK_LOADER_TEST_LOADER_PATH=$loader_dll&& $executable --gtest_brief=1 --gtest_filter=$filter"
  ) >"$output" 2>&1
}

failed_output() {
  rg -q '\[  FAILED  \]|fatal runtime error|Unhandled exception|Invalid physicalDevice' "$1"
}

if [[ "${1:-}" != "--full" ]]; then
  filter="${1:-GetProcAddr.GlobalFunctions}"
  status=0
  run_case test_regression.exe "$filter" "$tmp_dir/result" || status=$?
  cat "$tmp_dir/result"
  if ((status != 0)) || failed_output "$tmp_dir/result" || ! rg -q '\[  PASSED  \]' "$tmp_dir/result"; then
    exit 1
  fi
  exit
fi

(
  cd "$test_dir"
  WINEDEBUG=-all WINEPATH="$wine_path" "$wine_runner" test_regression.exe --gtest_list_tests
) | tr -d '\r' | awk '
  /^[^ ]/ { suite = $1; next }
  /^  [^ ]/ { name = $1; sub(/#.*/, "", name); print suite name }
' >"$tmp_dir/cases"

total="$(wc -l <"$tmp_dir/cases")"
index=0
failures=0
while IFS= read -r filter; do
  index=$((index + 1))
  status=0
  run_case test_regression.exe "$filter" "$tmp_dir/result" || status=$?
  if ((status != 0)) || failed_output "$tmp_dir/result" || ! rg -q '\[  PASSED  \]' "$tmp_dir/result"; then
    failures=$((failures + 1))
    echo "FAILED: $filter (status $status)" >&2
    rg -n -C5 'Failure|FAILED|fatal runtime error|Unhandled exception|Invalid physicalDevice' \
      "$tmp_dir/result" >&2 || true
  fi
  if ((index % 100 == 0)); then
    echo "$index/$total Windows regression cases"
  fi
done <"$tmp_dir/cases"

for executable in test_fuzzing.exe test_threading.exe; do
  status=0
  run_case "$executable" '*' "$tmp_dir/result" || status=$?
  if ((status != 0)) || failed_output "$tmp_dir/result" || ! rg -q '\[  PASSED  \]' "$tmp_dir/result"; then
    failures=$((failures + 1))
    echo "FAILED: $executable (status $status)" >&2
    tail -n 80 "$tmp_dir/result" >&2
  else
    rg '\[==========\]|\[  PASSED  \]' "$tmp_dir/result" | tail -n 2
  fi
done

if ((failures != 0)); then
  echo "$failures Windows parity failures" >&2
  exit 1
fi
echo "Windows parity passed: $total regression cases plus fuzzing and threading suites"
