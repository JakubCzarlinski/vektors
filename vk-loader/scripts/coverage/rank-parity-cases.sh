#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"

audit_dir="${VK_LOADER_PARITY_AUDIT_DIR:-$repo_root/target/coverage-parity-audit}"
profile_root="${VK_LOADER_PARITY_PROFILE_DIR:-$audit_dir/profiles}"
output="${VK_LOADER_PARITY_COVERAGE_RANKING:-$audit_dir/coverage.tsv}"
summary="$audit_dir/summary.tsv"
upstream_loader="${VK_LOADER_PARITY_UPSTREAM_LIBRARY:-$upstream_build_dir/loader/libvulkan.so}"
rust_loader="$(resolve_rust_loader "${VK_LOADER_PARITY_RUST_LIBRARY:-}" release)"

require_tools llvm-cov llvm-profdata
require_files "$summary" "$upstream_loader" "$rust_loader"

scratch="$(mktemp -d "$audit_dir/.coverage.XXXXXX")"
trap 'find "$scratch" -depth -delete' EXIT

coverage_counts() {
  local binary="$1"
  local profile_directory="$2"
  local merged="$3"
  local -a profiles=("$profile_directory"/*.profraw)
  if [[ ! -e "${profiles[0]}" ]]; then
    printf '0\t0\n'
    return
  fi
  llvm-profdata merge -sparse "${profiles[@]}" -o "$merged"
  llvm-cov report "$binary" -instr-profile="$merged" |
    awk '$1 == "TOTAL" { print ($2 - $3) "\t" ($8 - $9); found = 1 }
         END { if (!found) print "0\t0" }'
}

printf 'suite\tfilter\tobservable\tupstream_regions\tupstream_lines\trust_regions\trust_lines\tupstream_only_lines\trust_only_lines\tdiff_lines\n' \
  > "$output"
mapfile -t cases < <(tail -n +2 "$summary")
rows="$scratch/rows"
mkdir -p "$rows"
rank_case() {
    local index="$1"
    local suite filter result _upstream_status _rust_status _diff
    IFS=$'\t' read -r suite filter result _upstream_status _rust_status _diff \
      <<< "${cases[$index]}"
    case_dir="$(printf '%s_%s' "$suite" "$filter" | tr -c '[:alnum:]_-' '_')"
    read -r upstream_regions upstream_lines < <(
      coverage_counts "$upstream_loader" "$profile_root/$case_dir/upstream" \
        "$scratch/$index-upstream.profdata"
    )
    read -r rust_regions rust_lines < <(
      coverage_counts "$rust_loader" "$profile_root/$case_dir/rust" \
        "$scratch/$index-rust.profdata"
    )
    upstream_only=0
    rust_only=0
    if [[ -f "$_diff" ]]; then
      read -r upstream_only rust_only < <(
        awk '/^-[^-]/ { upstream++ } /^\+[^+]/ { rust++ }
             END { print upstream + 0, rust + 0 }' "$_diff"
      )
    fi
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$suite" "$filter" "$result" "$upstream_regions" "$upstream_lines" \
      "$rust_regions" "$rust_lines" "$upstream_only" "$rust_only" \
      "$((upstream_only + rust_only))" > "$rows/$index.tsv"
}

jobs="$(loader_test_jobs)"
pids=()
for index in "${!cases[@]}"; do
  rank_case "$index" &
  pids+=("$!")
  if ((${#pids[@]} == jobs)); then
    for pid in "${pids[@]}"; do
      wait "$pid"
    done
    pids=()
  fi
done
for pid in "${pids[@]}"; do
  wait "$pid"
done
for index in "${!cases[@]}"; do
  cat "$rows/$index.tsv" >> "$output"
done

ranking="$audit_dir/coverage-ranked.tsv"
{
  head -n 1 "$output"
  tail -n +2 "$output" | sort -t $'\t' -k3,3 -k7,7nr -k5,5nr
} > "$ranking"

echo "Per-case coverage: $output"
echo "Coverage-ranked TDD cases: $ranking"

tdd_ranking="$audit_dir/tdd-ranked.tsv"
{
  head -n 1 "$output"
  tail -n +2 "$output" | sort -t $'\t' -k3,3 -k10,10n -k7,7nr
} > "$tdd_ranking"
echo "Smallest failing TDD cases: $tdd_ranking"
