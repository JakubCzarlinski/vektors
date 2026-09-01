#!/usr/bin/env bash
set -euo pipefail

if (( $# != 5 )); then
  echo "usage: $0 LOADER UNIT_TEST_BINARY PROFILE EXTERNAL_REPORT OUTPUT" >&2
  exit 2
fi

loader="$1"
unit_binary="$2"
profile="$3"
external_report="$4"
output="$5"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../lib" && pwd)/common.sh"
scratch="$(mktemp -d)"
trap 'find "$scratch" -depth -delete' EXIT

total_lines=0
covered_lines=0
{
  printf '%-44s %10s %10s %10s\n' Filename Lines Covered Cover
  printf '%s\n' '-------------------------------------------------------------------------------'
  while IFS= read -r relative; do
    source="$(resolve_coverage_source "$relative")" || continue
    llvm-cov show "$loader" -instr-profile="$profile" \
      --show-line-counts-or-regions --show-expansions=false "$source" \
      > "$scratch/release.txt"
    llvm-cov show "$unit_binary" -instr-profile="$profile" \
      --show-line-counts-or-regions --show-expansions=false "$source" \
      > "$scratch/unit.txt"
    read -r lines covered < <(
      awk -F '|' '
        NR == FNR {
          line = $1
          gsub(/[[:space:]]/, "", line)
          count = $2
          gsub(/[[:space:]]/, "", count)
          if (line ~ /^[0-9]+$/ && count != "" && count != "0") unit_hit[line] = 1
          next
        }
        {
          line = $1
          gsub(/[[:space:]]/, "", line)
          count = $2
          gsub(/[[:space:]]/, "", count)
          if (line ~ /^[0-9]+$/ && count != "") {
            release_line[line] = 1
            if (count != "0") release_hit[line] = 1
          }
        }
        END {
          for (line in release_line) {
            lines++
            if (release_hit[line] || unit_hit[line]) covered++
          }
          print lines + 0, covered + 0
        }
      ' "$scratch/unit.txt" "$scratch/release.txt"
    )
    total_lines=$((total_lines + lines))
    covered_lines=$((covered_lines + covered))
    percentage="$(awk -v covered="$covered" -v lines="$lines" \
      'BEGIN { printf "%.2f%%", lines == 0 ? 100 : 100 * covered / lines }')"
    printf '%-44s %10d %10d %10s\n' "$relative" "$lines" "$covered" "$percentage"
  done < <(
    awk 'NR > 2 && $1 != "TOTAL" && $1 !~ /^-+$/ { print $1 }' "$external_report"
  )
  printf '%s\n' '-------------------------------------------------------------------------------'
  percentage="$(awk -v covered="$covered_lines" -v lines="$total_lines" \
    'BEGIN { printf "%.2f%%", lines == 0 ? 100 : 100 * covered / lines }')"
  printf '%-44s %10d %10d %10s\n' TOTAL "$total_lines" "$covered_lines" "$percentage"
} > "$output"
