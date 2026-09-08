#!/usr/bin/env python3
"""Generate deterministic JSON/manifest mutations without replacing regressions."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import tempfile


def mutations(data):
    yield "original", data
    # Exercise byte-prefix fields without converting the whole document to
    # Unicode: cJSON accepts an invalid UTF-8 suffix after an ASCII prefix.
    for field in (b"library_arch", b"api_version"):
        pattern = rb'("' + field + rb'"\s*:\s*"[^"\\]*)"'
        changed, count = re.subn(pattern, lambda match: match[1] + b'\xff"', data)
        if count:
            yield f"raw-byte-suffix-{field.decode('ascii')}", changed
    positions = sorted(set(range(min(len(data), 32))) | {
        len(data) // 4, len(data) // 2, 3 * len(data) // 4, max(0, len(data) - 1)
    })
    for position in positions:
        yield f"truncate-{position}", data[:position]
    for suffix in (b"\x00", b"\xff", b"\xc2", b"\xe2\x82", b"\xf0\x9f\x92", b"{}", b"[]"):
        yield f"suffix-{suffix.hex()}", data + suffix
    for token in (b'"', b"{", b"}", b"[", b"]", b":", b","):
        position = data.find(token)
        if position >= 0:
            yield f"delete-{token.hex()}", data[:position] + data[position + 1:]
            yield f"duplicate-{token.hex()}", data[:position] + token + data[position:]
    try:
        document = json.loads(data)
    except (ValueError, UnicodeError):
        return
    if isinstance(document, dict):
        replacements = (None, False, 0, -1, 4294967295, 4294967296, "", [], {}, "\ud800")
        for key in sorted(document):
            changed = dict(document)
            del changed[key]
            yield f"remove-key-{key}", json.dumps(changed, ensure_ascii=True).encode()
            for index, replacement in enumerate(replacements):
                changed = dict(document)
                changed[key] = replacement
                yield f"replace-key-{key}-{index}", json.dumps(changed, ensure_ascii=True).encode()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--seeds", type=Path, action="append", required=True)
    parser.add_argument("--output-parent", type=Path, required=True)
    args = parser.parse_args()
    sources = sorted({path for root in args.seeds for path in root.iterdir() if path.is_file()})
    if not sources:
        parser.error("no seed files found")
    args.output_parent.mkdir(parents=True, exist_ok=True)
    output = Path(tempfile.mkdtemp(prefix="generated-", dir=args.output_parent))
    cases = output / "cases"
    cases.mkdir()
    records = {}
    for source in sources:
        data = source.read_bytes()
        source_hash = hashlib.sha256(data).hexdigest()
        for mutation, content in mutations(data):
            digest = hashlib.sha256(content).hexdigest()
            if digest in records:
                continue
            (cases / digest).write_bytes(content)
            records[digest] = {
                "sha256": digest, "bytes": len(content), "seed": source.name,
                "seed_sha256": source_hash, "mutation": mutation,
            }
    with (output / "manifest.jsonl").open("w", encoding="utf-8") as manifest:
        for digest in sorted(records):
            manifest.write(json.dumps(records[digest], sort_keys=True) + "\n")
    print(json.dumps({"directory": str(output), "seeds": len(sources), "cases": len(records),
                      "bytes": sum(record["bytes"] for record in records.values()),  # ty: ignore[no-matching-overload]
                      "coverage_guided": False, "replayed": False}, sort_keys=True))


if __name__ == "__main__":
    main()
