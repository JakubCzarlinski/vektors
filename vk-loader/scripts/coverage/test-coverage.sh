#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
upstream_loader="$upstream_build_dir/loader/libvulkan.so.1.4.361"
target_dir="${VK_LOADER_COVERAGE_DIR:-$repo_root/target/vk-loader-coverage}"
profile_dir="$target_dir/profiles"

require_tools llvm-cov llvm-profdata

ensure_upstream_tests test_regression "$upstream_loader"

mkdir -p "$profile_dir"
find "$profile_dir" -type f -name '*.profraw' -delete
find "$target_dir" -maxdepth 1 -type f -name 'test_*.log' -delete

RUSTFLAGS='-Cinstrument-coverage -Cforce-frame-pointers=yes' \
  build_rust_loader release "$target_dir"

echo "coverage: Rust unit tests"
unit_log="$target_dir/unit.log"
if ! LLVM_PROFILE_FILE="$profile_dir/unit-%p-%m.profraw" \
  CARGO_TARGET_DIR="$target_dir" \
  RUSTFLAGS='-Cinstrument-coverage -Cforce-frame-pointers=yes' \
  cargo test --quiet --manifest-path "$repo_root/Cargo.toml" -p vk-loader --lib --release \
    > "$unit_log" 2>&1; then
  tail -n 200 "$unit_log" >&2
  exit 1
fi
rm -f "$unit_log"

loader="$target_dir/release/libvulkan.so"
status_summary="$target_dir/status.tsv"
printf 'suite\ttest_count\tupstream_status\trust_status\n' >"$status_summary"
for suite in test_regression test_fuzzing test_threading; do
  upstream_log="$target_dir/$suite.upstream.log"
  rust_log="$target_dir/$suite.rust.log"
  echo "coverage parity: $suite (upstream)"
  if ! run_gtest_shards "$upstream_loader" "$suite" "$upstream_log" --gtest_brief=1; then
    tail -n 200 "$upstream_log" >&2
    exit 1
  fi
  echo "coverage parity: $suite (Rust)"
  if ! LLVM_PROFILE_FILE="$profile_dir/%p-%m.profraw" \
    run_gtest_shards "$loader" "$suite" "$rust_log" --gtest_brief=1; then
    tail -n 200 "$rust_log" >&2
    exit 1
  fi
  printf '%s\t%s\t0\t0\n' "$suite" "$(gtest_case_count "$suite")" >>"$status_summary"
  rm -f "$upstream_log" "$rust_log"
done

parity_status=0
run_coverage_probe() {
  local label="$1"
  local profile_name="$2"
  shift 2
  echo "coverage: $label"
  if ! env \
    VK_LOADER_PARITY_RUST_LIBRARY="$loader" \
    LLVM_PROFILE_FILE="$profile_dir/$profile_name-%p-%m.profraw" \
    "$@"; then
    parity_status=1
  fi
}

run_coverage_probe "differential pre-instance public-contract probe" pre-instance \
  VK_LOADER_CONTRACT_PARITY_DIR="$target_dir/pre-instance-contract-parity" \
  "$loader_scripts/parity/test-pre-instance-contract-parity.sh"
run_coverage_probe "differential valid-driver instance-extension contract probe" instance-extension \
  VK_LOADER_EXTENSION_CONTRACT_PARITY_DIR="$target_dir/instance-extension-contract-parity" \
  "$loader_scripts/parity/test-instance-extension-contract-parity.sh"
run_coverage_probe "differential two-layer chain contract probe" layer-chain \
  VK_LOADER_LAYER_CHAIN_PARITY_DIR="$target_dir/layer-chain-contract-parity" \
  "$loader_scripts/parity/test-layer-chain-contract-parity.sh"
run_coverage_probe "paired layer-modified create-terminator tests" create-terminator \
  VK_LOADER_CREATE_TERMINATOR_PARITY_DIR="$target_dir/create-terminator-parity" \
  "$loader_scripts/parity/test-create-terminator-parity.sh"
run_coverage_probe "differential debug-report ICD forwarding probe" debug-report \
  VK_LOADER_DEBUG_REPORT_PARITY_DIR="$target_dir/debug-report-forwarding-parity" \
  "$loader_scripts/parity/test-debug-report-forwarding-parity.sh"
run_coverage_probe "differential device-group and WSI lifecycle probe" device-group \
  "$loader_scripts/parity/test-device-group-parity.sh"

profiles=("$profile_dir"/*.profraw)
if [[ ! -e "${profiles[0]}" ]]; then
  echo "instrumented loader did not produce coverage profiles" >&2
  exit 1
fi

profile="$target_dir/merged.profdata"
report="$target_dir/report.txt"
llvm-profdata merge -sparse "${profiles[@]}" -o "$profile"
llvm-cov report "$loader" \
  -instr-profile="$profile" \
  --ignore-filename-regex='(/rustc/|/\.cargo/registry/|vk-loader/src/generated/)' \
  > "$report"

unit_binary="$({
  find "$target_dir/release/deps" -maxdepth 1 -type f -perm -u+x \
    -name 'vulkan-*' -printf '%T@\t%p\n'
} | sort -n | tail -n 1 | cut -f2-)"
if [[ -z "$unit_binary" ]]; then
  echo "instrumented Rust unit-test binary not found" >&2
  exit 1
fi
union_report="$target_dir/production-line-union.txt"
"$loader_scripts/coverage/coverage-line-union.sh" \
  "$loader" "$unit_binary" "$profile" "$report" "$union_report"

uncovered="$target_dir/uncovered-lines.txt"
mapfile -t coverage_sources < <(
  awk 'NR > 2 && $1 != "TOTAL" && $1 !~ /^-+$/ { print $1 }' "$report" |
    while IFS= read -r relative; do
      [[ -f "$repo_root/$relative" ]] && printf '%s\n' "$repo_root/$relative"
    done
)
{
  for source in "${coverage_sources[@]}"; do
    relative="${source#"$repo_root/"}"
    echo "== $relative =="
    llvm-cov show "$loader" -instr-profile="$profile" \
      --show-line-counts-or-regions --show-expansions=false "$source" |
      awk -F '|' '$2 ~ /^[[:space:]]*0[[:space:]]*$/ && $3 ~ /[^[:space:]]/ {
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", $1)
        print $1 ":" $3
      }'
  done
} > "$uncovered"

external_total="$(awk '$1 == "TOTAL" { print $10 }' "$report")"
union_total="$(awk '$1 == "TOTAL" { print $4 }' "$union_report")"
echo "External differential line coverage: $external_total"
echo "Production line coverage including unit tests: $union_total"
echo "Coverage report: $report"
echo "Production line-union report: $union_report"
echo "Uncovered source lines: $uncovered"

minimum_line_coverage="${VK_LOADER_MIN_LINE_COVERAGE:-90.0}"
line_coverage="$(awk '$1 == "TOTAL" { value = $10; gsub(/%/, "", value); print value }' "$report")"
if ! awk -v actual="$line_coverage" -v minimum="$minimum_line_coverage" \
  'BEGIN { exit !(actual + 0 >= minimum + 0) }'; then
  echo "Line coverage ${line_coverage}% is below required ${minimum_line_coverage}%" >&2
  exit 1
fi
minimum_union_coverage="${VK_LOADER_MIN_UNION_COVERAGE:-91.0}"
union_coverage="$(awk '$1 == "TOTAL" { value = $4; gsub(/%/, "", value); print value }' "$union_report")"
if ! awk -v actual="$union_coverage" -v minimum="$minimum_union_coverage" \
  'BEGIN { exit !(actual + 0 >= minimum + 0) }'; then
  echo "Production line-union coverage ${union_coverage}% is below required ${minimum_union_coverage}%" >&2
  exit 1
fi
if (( parity_status != 0 )); then
  echo "One or more differential coverage probes did not match upstream" >&2
  exit "$parity_status"
fi

# The merged profile is sufficient to reproduce reports. Raw process profiles
# are numerous and can include a root-level file from env-sanitizing death-test
# children, so discard them after a successful run.
find "$profile_dir" -type f -name '*.profraw' -delete
find "$repo_root" -maxdepth 1 -type f -name '*.profraw' -delete
