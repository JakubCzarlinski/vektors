#!/usr/bin/env python3

import csv
import random
import statistics
import sys
from collections import defaultdict
from pathlib import Path


def reject_outliers(values: list[float]) -> tuple[list[float], int]:
    median = statistics.median(values)
    deviations = [abs(value - median) for value in values]
    mad = statistics.median(deviations)
    if mad == 0:
        retained = [value for value in values if value == median]
    else:
        retained = [value for value in values if abs(value - median) <= 5 * mad]
    return retained or values, len(values) - len(retained)


def bootstrap_ratio(
    rust: list[float], upstream: list[float], iterations: int = 10_000
) -> tuple[float, float]:
    randomizer = random.Random(0x564B4C44)
    ratios = []
    for _ in range(iterations):
        rust_sample = randomizer.choices(rust, k=len(rust))
        upstream_sample = randomizer.choices(upstream, k=len(upstream))
        ratios.append(
            statistics.median(rust_sample) / statistics.median(upstream_sample)
        )
    ratios.sort()
    return ratios[int(iterations * 0.025)], ratios[int(iterations * 0.975)]


def main() -> int:
    if len(sys.argv) != 3:
        print(f"usage: {sys.argv[0]} INPUT.csv OUTPUT.csv", file=sys.stderr)
        return 2

    input_path = Path(sys.argv[1])
    output_path = Path(sys.argv[2])
    groups: dict[tuple[str, str, str], dict[str, list[float]]] = defaultdict(
        lambda: defaultdict(list)
    )
    with input_path.open(newline="") as input_file:
        for row in csv.DictReader(input_file):
            groups[(row["layer"], row["log_level"], row["mode"])][row["loader"]].append(
                float(row["ns_per_operation"])
            )

    fieldnames = [
        "layer",
        "log_level",
        "mode",
        "rust_samples",
        "upstream_samples",
        "rust_outliers",
        "upstream_outliers",
        "rust_median_ns",
        "upstream_median_ns",
        "rust_mad_ns",
        "upstream_mad_ns",
        "rust_over_upstream",
        "ratio_ci95_low",
        "ratio_ci95_high",
    ]
    with output_path.open("w", newline="") as output_file:
        writer = csv.DictWriter(output_file, fieldnames=fieldnames)
        writer.writeheader()
        for (layer, log_level, mode), implementations in sorted(groups.items()):
            if "rust" not in implementations or "upstream" not in implementations:
                continue
            rust, rust_outliers = reject_outliers(implementations["rust"])
            upstream, upstream_outliers = reject_outliers(implementations["upstream"])
            rust_median = statistics.median(rust)
            upstream_median = statistics.median(upstream)
            low, high = bootstrap_ratio(rust, upstream)
            writer.writerow(
                {
                    "layer": layer,
                    "log_level": log_level,
                    "mode": mode,
                    "rust_samples": len(rust),
                    "upstream_samples": len(upstream),
                    "rust_outliers": rust_outliers,
                    "upstream_outliers": upstream_outliers,
                    "rust_median_ns": f"{rust_median:.3f}",
                    "upstream_median_ns": f"{upstream_median:.3f}",
                    "rust_mad_ns": f"{statistics.median(abs(value - rust_median) for value in rust):.3f}",
                    "upstream_mad_ns": f"{statistics.median(abs(value - upstream_median) for value in upstream):.3f}",
                    "rust_over_upstream": f"{rust_median / upstream_median:.6f}",
                    "ratio_ci95_low": f"{low:.6f}",
                    "ratio_ci95_high": f"{high:.6f}",
                }
            )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
