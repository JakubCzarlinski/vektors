#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
comparison_script="$loader_scripts/parity/compare-observable-parity.sh"
output_root="${VK_LOADER_PARITY_AUDIT_DIR:-$repo_root/target/observable-parity-audit}"
summary="$output_root/summary.tsv"
rust_loader="$(resolve_rust_loader "${VK_LOADER_PARITY_RUST_LIBRARY:-}" release)"
shard_count="${VK_LOADER_PARITY_SHARD_COUNT:-1}"
shard_index="${VK_LOADER_PARITY_SHARD_INDEX:-0}"

filters=(
  'DirectDriverLoading.Individual'
  'GetProcAddr.GlobalFunctions'
  'UnknownFunction.PhysicalDeviceFunctionTwoLayerInterception'
  'SettingsFile.DeviceConfigurationWithSameDriver'
  'WsiTests.SwapchainFunctional'
)
suites=(test_regression)

ensure_upstream_tests

if [[ "${1:-}" == "--full" ]]; then
  suites=(test_regression test_fuzzing test_threading)
  filters=()
  for suite in "${suites[@]}"; do
    while IFS= read -r filter; do
      filters+=("$suite:$filter")
    done < <(
      "$upstream_build_dir/tests/$suite" --gtest_list_tests |
        awk '
          /^[^[:space:]].*\.$/ {
            suite = $1
            next
          }
          /^  [^[:space:]]/ {
            test = $1
            if (suite !~ /(^|\/)DISABLED_/ && test !~ /^DISABLED_/) {
              print suite test
            }
          }
        '
    )
  done
elif (( $# > 0 )); then
  filters=("$@")
fi

if ! [[ "$shard_count" =~ ^[1-9][0-9]*$ && "$shard_index" =~ ^[0-9]+$ ]] ||
    (( shard_index >= shard_count )); then
  echo "invalid observable-parity shard: index=$shard_index count=$shard_count" >&2
  exit 2
fi

require_files "$rust_loader"

mkdir -p "$output_root"
printf 'suite\tfilter\tresult\tupstream_status\trust_status\tdiff\n' >"$summary"

differences=0
executed=0
for index in "${!filters[@]}"; do
  (( index % shard_count == shard_index )) || continue
  record="${filters[$index]}"
  if [[ "$record" == test_*:* ]]; then
    suite="${record%%:*}"
    filter="${record#*:}"
  else
    suite=test_regression
    filter="$record"
  fi
  case_dir="$(printf '%s_%s' "$suite" "$filter" | tr -c '[:alnum:]_-' '_')"
  result_dir="$output_root/$case_dir"
  rm -rf "$result_dir"

  if VK_LOADER_PARITY_SUITE="$suite" \
    VK_LOADER_PARITY_RUST_LIBRARY="$rust_loader" \
    VK_LOADER_PARITY_DIFF_DIR="$result_dir" \
    "$comparison_script" "$filter"; then
    result=match
  else
    differences=$((differences + 1))
    result=difference
  fi

  upstream_status=missing
  rust_status=missing
  if [[ -f "$result_dir/status.tsv" ]]; then
    upstream_status="$(awk -F '\t' '$1 == "upstream" { print $2 }' "$result_dir/status.tsv")"
    rust_status="$(awk -F '\t' '$1 == "rust" { print $2 }' "$result_dir/status.tsv")"
  fi
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$suite" "$filter" "$result" \
    "$upstream_status" "$rust_status" "$result_dir/observable.diff" >>"$summary"
  executed=$((executed + 1))
done

echo "Observable parity audit: $executed cases, $differences differences (shard $shard_index/$shard_count)"
echo "Summary: $summary"
(( differences == 0 ))
