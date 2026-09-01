#!/usr/bin/env python3

import csv
import random
import statistics
import sys
from collections import defaultdict
from pathlib import Path


def reject_outliers(values: list[float]) -> tuple[list[float], int]:
    median = statistics.median(values)
    mad = statistics.median(abs(value - median) for value in values)
    if mad == 0:
        return values, 0
    retained = [value for value in values if abs(value - median) <= 5 * mad]
    return retained or values, len(values) - len(retained)


def bootstrap_ratio(rust: list[float], upstream: list[float]) -> tuple[float, float]:
    randomizer = random.Random(0x50455246)
    ratios = []
    for _ in range(10_000):
        rust_sample = randomizer.choices(rust, k=len(rust))
        upstream_sample = randomizer.choices(upstream, k=len(upstream))
        upstream_median = statistics.median(upstream_sample)
        if upstream_median != 0:
            ratios.append(statistics.median(rust_sample) / upstream_median)
    if not ratios:
        return float("nan"), float("nan")
    ratios.sort()
    return ratios[int(len(ratios) * 0.025)], ratios[int(len(ratios) * 0.975)]


def main() -> int:
    if len(sys.argv) != 3:
        print(f"usage: {sys.argv[0]} INPUT.csv OUTPUT.csv", file=sys.stderr)
        return 2
    groups: dict[tuple[str, str, str, str], dict[str, list[float]]] = defaultdict(
        lambda: defaultdict(list)
    )
    with Path(sys.argv[1]).open(newline="") as input_file:
        for row in csv.DictReader(input_file):
            groups[(row["layer"], row["mode"], row["group"], row["event"])][
                row["loader"]
            ].append(float(row["value"]))

    fields = [
        "layer",
        "mode",
        "group",
        "event",
        "rust_samples",
        "upstream_samples",
        "rust_outliers",
        "upstream_outliers",
        "rust_median",
        "upstream_median",
        "rust_mad",
        "upstream_mad",
        "rust_over_upstream",
        "ratio_ci95_low",
        "ratio_ci95_high",
    ]
    with Path(sys.argv[2]).open("w", newline="") as output_file:
        writer = csv.DictWriter(output_file, fieldnames=fields)
        writer.writeheader()
        for key, implementations in sorted(groups.items()):
            if "rust" not in implementations or "upstream" not in implementations:
                continue
            rust, rust_outliers = reject_outliers(implementations["rust"])
            upstream, upstream_outliers = reject_outliers(implementations["upstream"])
            rust_median = statistics.median(rust)
            upstream_median = statistics.median(upstream)
            if upstream_median == 0:
                ratio = ""
                low = ""
                high = ""
            else:
                ratio = f"{rust_median / upstream_median:.6f}"
                low_value, high_value = bootstrap_ratio(rust, upstream)
                low = f"{low_value:.6f}"
                high = f"{high_value:.6f}"
            writer.writerow(
                dict(
                    zip(fields[:4], key, strict=True),
                    rust_samples=len(rust),
                    upstream_samples=len(upstream),
                    rust_outliers=rust_outliers,
                    upstream_outliers=upstream_outliers,
                    rust_median=f"{rust_median:.3f}",
                    upstream_median=f"{upstream_median:.3f}",
                    rust_mad=f"{statistics.median(abs(value - rust_median) for value in rust):.3f}",
                    upstream_mad=f"{statistics.median(abs(value - upstream_median) for value in upstream):.3f}",
                    rust_over_upstream=ratio,
                    ratio_ci95_low=low,
                    ratio_ci95_high=high,
                )
            )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
