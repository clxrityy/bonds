#!/usr/bin/env python3
"""
scripts/versioner.py

Updates version(s) in:
- crates/core/Cargo.toml ([package].version)
- crates/cli/Cargo.toml ([package].version)

Also updates:
- crates/cli/Cargo.toml bonds-core dependency version (when target includes core)
- Makefile VERSION ?= vX.Y.Z (optional)

Usage examples:
  python3 scripts/versioner.py --version v0.1.4 --target all --update-makefile
  python3 scripts/versioner.py --version 0.1.4 --target core
  python3 scripts/versioner.py --version 0.1.4 --target cli --dry-run
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path

# Basic semver validator: MAJOR.MINOR.PATCH with optional pre-release/build.
SEMVER_RE = re.compile(
    r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)"
    r"(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$"
)

# Finds the first version line in [package] section.
PACKAGE_VERSION_RE = re.compile(
    r'(?ms)(^\[package\]\r?\n.*?^version\s*=\s*")([^"]+)(")'
)

# Finds inline bonds-core dependency version in cli Cargo.toml.
BONDS_CORE_DEP_RE = re.compile(
    r'(\bbonds-core\s*=\s*\{[^}\n]*\bversion\s*=\s*")[^"]+(")'
)

# Finds Makefile VERSION assignment.
MAKEFILE_VERSION_RE = re.compile(r"(?m)^VERSION\s*\?=\s*.*$")


def normalize_versions(raw_version: str) -> tuple[str, str]:
    """
    Cargo.toml versions should be plain semver (e.g., 0.1.4).
    Makefile VERSION convention here uses leading 'v' (e.g., v0.1.4).
    """
    cargo_version = raw_version[1:] if raw_version.startswith("v") else raw_version
    if not SEMVER_RE.fullmatch(cargo_version):
        raise ValueError(
            f"Invalid version '{raw_version}'. Expected semver like 0.1.4 or v0.1.4."
        )
    makefile_version = f"v{cargo_version}"
    return cargo_version, makefile_version


def write_if_changed(path: Path, original: str, updated: str, dry_run: bool) -> bool:
    changed = original != updated
    if changed and not dry_run:
        path.write_text(updated, encoding="utf-8")
    return changed


def update_package_version(path: Path, new_version: str, dry_run: bool) -> bool:
    original = path.read_text(encoding="utf-8")
    updated, count = PACKAGE_VERSION_RE.subn(
        lambda m: f'{m.group(1)}{new_version}{m.group(3)}',
        original,
        count=1,
    )
    if count == 0:
        raise RuntimeError(f"Could not find [package].version in {path}")
    return write_if_changed(path, original, updated, dry_run)


def update_cli_core_dependency(path: Path, new_version: str, dry_run: bool) -> bool:
    original = path.read_text(encoding="utf-8")
    updated, count = BONDS_CORE_DEP_RE.subn(
        lambda m: f'{m.group(1)}{new_version}{m.group(2)}',
        original,
        count=1,
    )
    if count == 0:
        raise RuntimeError(f"Could not find bonds-core dependency version in {path}")
    return write_if_changed(path, original, updated, dry_run)


def update_makefile_version(path: Path, makefile_version: str, dry_run: bool) -> bool:
    original = path.read_text(encoding="utf-8")
    updated, count = MAKEFILE_VERSION_RE.subn(
        f"VERSION ?= {makefile_version}",
        original,
        count=1,
    )
    if count == 0:
        raise RuntimeError(f"Could not find VERSION ?= ... in {path}")
    return write_if_changed(path, original, updated, dry_run)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Update crate Cargo.toml versions and optionally Makefile VERSION."
    )
    parser.add_argument(
        "--version",
        required=True,
        help='Target version, e.g. "0.1.4" or "v0.1.4".',
    )
    parser.add_argument(
        "--target",
        choices=("all", "core", "cli"),
        default="all",
        help="Which crate(s) to update.",
    )
    parser.add_argument(
        "--update-makefile",
        action="store_true",
        help="Also update Makefile VERSION ?= vX.Y.Z.",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Preview changes without writing files.",
    )
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[1]
    core_manifest = repo_root / "crates/core/Cargo.toml"
    cli_manifest = repo_root / "crates/cli/Cargo.toml"
    makefile = repo_root / "Makefile"

    cargo_version, makefile_version = normalize_versions(args.version)
    results: list[str] = []

    if args.target in ("all", "core"):
        changed = update_package_version(core_manifest, cargo_version, args.dry_run)
        results.append(
            f"{'UPDATED' if changed else 'UNCHANGED'} {core_manifest} [package].version -> {cargo_version}"
        )

        # Keep CLI dependency aligned when core version moves.
        dep_changed = update_cli_core_dependency(cli_manifest, cargo_version, args.dry_run)
        results.append(
            f"{'UPDATED' if dep_changed else 'UNCHANGED'} {cli_manifest} bonds-core dependency -> {cargo_version}"
        )

    if args.target in ("all", "cli"):
        changed = update_package_version(cli_manifest, cargo_version, args.dry_run)
        results.append(
            f"{'UPDATED' if changed else 'UNCHANGED'} {cli_manifest} [package].version -> {cargo_version}"
        )

    if args.update_makefile:
        changed = update_makefile_version(makefile, makefile_version, args.dry_run)
        results.append(
            f"{'UPDATED' if changed else 'UNCHANGED'} {makefile} VERSION -> {makefile_version}"
        )

    print("\n".join(results))
    if args.dry_run:
        print("DRY RUN: no files were changed.")


if __name__ == "__main__":
    try:
        main()
    except Exception as exc:
        raise SystemExit(f"Error: {exc}")
