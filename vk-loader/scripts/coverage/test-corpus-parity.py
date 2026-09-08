#!/usr/bin/env python3
"""Compare every corpus replay observation; a crash or mismatch fails the run."""

import argparse
import difflib
import os
from pathlib import Path
import re
import resource
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--runner", type=Path, required=True)
    parser.add_argument("--upstream", type=Path, required=True)
    parser.add_argument("--rust", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--timeout", type=int, default=600)
    args = parser.parse_args()
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    args.output.mkdir(parents=True, exist_ok=True)
    observations = []
    failed = False
    summary = ["loader\texit_status\tpassed_tests\tscenarios\tcomplete\n"]
    for label, loader in (("upstream", args.upstream), ("rust", args.rust)):
        environment = {key: value for key, value in os.environ.items()
                       if not key.startswith("GTEST_")}
        environment["VK_LOADER_TEST_LOADER_PATH"] = str(loader.resolve(strict=True))
        log_path = args.output / f"{label}.log"
        with log_path.open("wb") as log, (args.output / f"{label}.stderr").open("wb") as errors:
            try:
                result = subprocess.run(
                    [str(args.runner.resolve(strict=True)), "--gtest_brief=1", "--gtest_color=no"],
                    env=environment, stdout=log, stderr=errors,
                    timeout=args.timeout, check=False)
                status = result.returncode
            except subprocess.TimeoutExpired:
                status = 124
        records = []
        passed = 0
        with log_path.open("rb") as log:
            for line in log:
                if line.startswith((b"RESULT ", b"LAYER_RESULT ", b"LAYER ",
                                    b"EXTENSION_RESULT ", b"EXTENSION ")):
                    records.append(line.decode("ascii"))
                match = re.match(rb"\[  PASSED  \] (\d+) tests?\.", line)
                if match:
                    passed = int(match[1])
        scenarios = [line.split()[:3] for line in records if line.startswith("RESULT ")]
        # Every test must emit one result in each of the four manifest roles.
        roles = {}
        for _, case, role in scenarios:
            roles.setdefault(case, []).append(role)
        complete = passed > 0 and len(roles) == passed and all(
            sorted(values) == ["0", "1", "2", "3"] for values in roles.values())
        failed |= status != 0 or not complete
        summary.append(f"{label}\t{status}\t{passed}\t{len(scenarios)}\t{complete}\n")
        (args.output / f"{label}.results").write_text("".join(records))
        observations.append(records)
        print(f"{label}: exit={status}, passed={passed}, scenarios={len(scenarios)}, complete={complete}")
    diff = list(difflib.unified_diff(*observations, fromfile="upstream", tofile="rust"))
    (args.output / "parity.diff").write_text("".join(diff))
    (args.output / "status.tsv").write_text("".join(summary))
    if diff:
        print(f"API parity mismatch: {args.output / 'parity.diff'}")
    elif not failed:
        for label in ("upstream", "rust"):
            for suffix in ("log", "stderr", "results"):
                (args.output / f"{label}.{suffix}").unlink()
        print(f"Corpus API parity passed: {args.output / 'status.tsv'}")
    raise SystemExit(1 if failed or diff else 0)


if __name__ == "__main__":
    main()
