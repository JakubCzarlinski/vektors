#!/usr/bin/env bash
set -uo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/lib" && pwd)/common.sh"

mode="${1:---quick}"
case "$mode" in
  --quick|--full) ;;
  *) echo "usage: $0 [--quick | --full]" >&2; exit 2 ;;
esac

output_dir="${VK_LOADER_TEST_ALL_DIR:-$repo_root/target/vk-loader-test-all}"
mkdir -p "$output_dir/logs"
summary="$output_dir/status.tsv"
results_dir="$output_dir/results"
rm -rf "$results_dir"
mkdir -p "$results_dir"

run_test() {
  local name="$1"
  local required="$2"
  shift 2
  local log="$output_dir/logs/$name.log"
  local started finished status result retained_log
  started="$(date +%s)"
  echo "test-all: $name"
  "$@" >"$log" 2>&1
  status=$?
  finished="$(date +%s)"
  if (( status == 0 )); then
    result=pass
    if [[ "$name" != loader-clippy ]] || [[ ! -s "$log" ]]; then
      rm -f "$log"
    fi
  elif (( status == 77 )) && [[ "$required" == optional ]]; then
    result=skip
  else
    result=fail
    tail -n 80 "$log" >&2
  fi
  if [[ -f "$log" ]]; then
    retained_log="$log"
  else
    retained_log=-
  fi
  printf '%s\t%s\t%d\t%d\t%s\n' "$name" "$result" "$status" \
    "$((finished - started))" "$retained_log" >"$results_dir/$name.tsv"
}

run_test workspace-units required cargo test --workspace --lib --bins
run_test formatting required cargo fmt --all --check
run_test loader-clippy required cargo clippy -p vk-loader --all-targets
run_test generated-sources required "$loader_scripts/codegen/generate.sh" --check
run_test dispatch-abi required "$loader_scripts/parity/check-dispatch-abi.sh"
run_test elf-exports required "$loader_scripts/parity/compare-exports.sh"

if [[ "$mode" == --quick ]]; then
  run_test paired-smoke required env \
    VK_LOADER_TEST_PARITY_DIR="$output_dir/paired-smoke" \
    "$loader_scripts/parity/test-parity.sh"
  run_test observable-smoke required env \
    VK_LOADER_PARITY_AUDIT_DIR="$output_dir/observable-smoke" \
    "$loader_scripts/parity/audit-observable-parity.sh"
  run_test cross-targets required "$loader_scripts/platform/check-cross-targets.sh"
else
  run_test valgrind required \
    "$loader_scripts/diagnostics/test-valgrind.sh" --full &
  valgrind_lane_pid=$!

  {
    run_test cross-targets required "$loader_scripts/platform/check-cross-targets.sh"
    run_test portable-linux optional "$loader_scripts/platform/build-portable-linux.sh"
    run_test windows-exports optional "$loader_scripts/platform/compare-windows-exports.sh"
  } &
  build_lane_pid=$!

  wait "$valgrind_lane_pid"
  wait "$build_lane_pid"

  # These stages either modify or consume Vulkan's process-external manifest
  # discovery state, so they must not overlap with one another or Valgrind.
  # Coverage already executes all unchanged upstream suites against both
  # loaders, the Rust unit tests, and every focused differential probe.
  run_test coverage-and-paired-suites required "$loader_scripts/coverage/test-coverage.sh"
  run_test observable-parity required env \
    VK_LOADER_PARITY_AUDIT_DIR="$output_dir/observable-full" \
    "$loader_scripts/parity/audit-observable-parity.sh" --full
  run_test address-sanitizer required "$loader_scripts/diagnostics/test-sanitizers.sh" --full
  run_test real-apps optional "$loader_scripts/apps/test-real-apps.sh"
  for wsi in wayland xcb; do
    for validation in 0 1; do
      run_test "sascha-$wsi-validation-$validation" optional env \
        VK_LOADER_SASCHA_WSI="$wsi" \
        VK_LOADER_SASCHA_VALIDATION="$validation" \
        VK_LOADER_SASCHA_COMPARE_UPSTREAM=1 \
        VK_LOADER_SASCHA_NO_SETUP=1 \
        VK_LOADER_SASCHA_OUTPUT_DIR="$output_dir/sascha" \
        "$loader_scripts/apps/test-sascha-willems.sh"
    done
  done
fi

printf 'test\tresult\texit_status\tduration_seconds\tlog\n' >"$summary"
find "$results_dir" -maxdepth 1 -type f -name '*.tsv' -print0 \
  | sort -z \
  | xargs -0 cat >>"$summary"
passes="$(awk -F '\t' '$2 == "pass" { count++ } END { print count + 0 }' "$summary")"
failures="$(awk -F '\t' '$2 == "fail" { count++ } END { print count + 0 }' "$summary")"
skips="$(awk -F '\t' '$2 == "skip" { count++ } END { print count + 0 }' "$summary")"
echo "test-all: $passes passed, $failures failed, $skips skipped"
echo "test-all summary: $summary"
(( failures == 0 ))
