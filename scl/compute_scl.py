#!/usr/bin/env python3
"""Recompute the scl values quoted in the paper, using scallop as the solver.

This script does no scl computation itself: scallop (Calegari's rationality
algorithm, implemented by Alden Walker, https://github.com/aldenwalker/scallop)
is an external dependency, not vendored here. Run ``./setup_scallop.sh`` once to
fetch and build it locally, then::

    python3 compute_scl.py

Each row of relators.py is passed to scallop; the returned scl is checked
against the value claimed in the paper.
"""

from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
from fractions import Fraction
from pathlib import Path

from relators import RELATORS

HERE = Path(__file__).resolve().parent

# scallop prints lines of the form "scl( word ) = 1/2 = 0.5" (or, for integers,
# "scl( word ) = 1 = 1"); the middle field is always an exact fraction.
SCL_LINE = re.compile(r"scl\(\s*\S+\s*\)\s*=\s*(?P<frac>\S+)\s*=")


def find_scallop(explicit: str | None) -> Path:
    candidates = []
    if explicit:
        candidates.append(Path(explicit))
    if os.environ.get("SCALLOP"):
        candidates.append(Path(os.environ["SCALLOP"]))
    candidates.append(HERE / "vendor" / "scallop" / "scallop")
    on_path = shutil.which("scallop")
    if on_path:
        candidates.append(Path(on_path))

    for candidate in candidates:
        if candidate.is_file() and os.access(candidate, os.X_OK):
            return candidate

    raise SystemExit(
        "scallop executable not found.\n"
        "Run ./setup_scallop.sh to fetch and build it, or pass --scallop PATH, "
        "or set the SCALLOP environment variable."
    )


def run_scallop(binary: Path, word: str, lib_dir: str | None) -> Fraction:
    env = dict(os.environ)
    if lib_dir:
        for var in ("DYLD_LIBRARY_PATH", "LD_LIBRARY_PATH"):
            env[var] = lib_dir + (":" + env[var] if env.get(var) else "")

    result = subprocess.run(
        [str(binary), word], capture_output=True, text=True, env=env, timeout=300
    )
    if result.returncode != 0:
        raise RuntimeError(f"scallop exited {result.returncode} on {word!r}:\n{result.stderr}")

    match = SCL_LINE.search(result.stdout)
    if not match:
        raise RuntimeError(f"could not parse scallop output for {word!r}:\n{result.stdout}")
    return Fraction(match.group("frac"))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--scallop", help="path to the scallop executable")
    parser.add_argument(
        "--lib-dir",
        help="directory containing libglpk/libgmp, if scallop cannot find them at runtime",
    )
    args = parser.parse_args()

    binary = find_scallop(args.scallop)
    print(f"using {binary}\n")

    width = max(len(r.name) for r in RELATORS)
    all_ok = True
    for relator in RELATORS:
        got = run_scallop(binary, relator.word, args.lib_dir)
        ok = got == relator.expected
        all_ok &= ok
        status = "ok" if ok else "MISMATCH"
        print(
            f"{relator.name:<{width}}  scl = {str(got):>4}  "
            f"(paper: {relator.expected!s:>4})  [{status}]  {relator.source}"
        )

    print()
    if all_ok:
        print("All scl values match the paper.")
    else:
        print("Some scl values did NOT match the paper -- see MISMATCH rows above.")
    sys.exit(0 if all_ok else 1)


if __name__ == "__main__":
    main()
