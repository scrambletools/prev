#!/usr/bin/env python3
"""Checks that packaging/flatpak/cargo-sources.json lists every crate in
Cargo.lock at the same version, so the Flatpak's offline build finds them.

Regenerate the list with flatpak-builder-tools' flatpak-cargo-generator.py
when this fails (see docs/RELEASING.md)."""
import json
import sys
import tomllib
from pathlib import Path

root = Path(__file__).resolve().parent.parent
lock = tomllib.loads((root / "Cargo.lock").read_text())
sources = json.loads((root / "packaging/flatpak/cargo-sources.json").read_text())
listed = {entry.get("url", "").rsplit("/", 1)[-1] for entry in sources}

missing = [
    f"{package['name']} {package['version']}"
    for package in lock["package"]
    if package.get("source", "").startswith("registry+")
    and f"{package['name']}-{package['version']}.crate" not in listed
]
if missing:
    print("cargo-sources.json is missing crates in Cargo.lock:", file=sys.stderr)
    for crate in missing:
        print(f"  {crate}", file=sys.stderr)
    print("Regenerate it with flatpak-cargo-generator.py (docs/RELEASING.md).", file=sys.stderr)
    sys.exit(1)
print(f"cargo-sources.json lists all {len(listed)} crates the Flatpak build needs")
