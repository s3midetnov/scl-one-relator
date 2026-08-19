#!/usr/bin/env bash
# Fetches and builds aldenwalker/scallop as a local build dependency.
#
# scallop is NOT vendored into this repository: this script clones it into
# scl/vendor/ (gitignored) and compiles it there. Re-run it any time; it is
# safe to call repeatedly.
#
# Requires: git, make, a C++11 compiler, and the GLPK and GMP development
# libraries (macOS: `brew install glpk gmp`; Debian/Ubuntu:
# `apt install libglpk-dev libgmp-dev`).
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VENDOR_DIR="$HERE/vendor"
SRC_DIR="$VENDOR_DIR/scallop"

mkdir -p "$VENDOR_DIR"

if [ -d "$SRC_DIR" ]; then
  echo "scl/vendor/scallop already present, skipping clone."
else
  git clone --depth 1 "${SCALLOP_REPO:-https://github.com/aldenwalker/scallop.git}" "$SRC_DIR"
fi

# The upstream Makefiles predate C++11 and, as of 2026, fail to compile with
# modern clang/gcc: scylla.cc uses brace-initialization, which needs
# -std=c++11 or later. Patch every subdirectory's CFLAGS; idempotent.
python3 - "$SRC_DIR" <<'PY'
import re
import sys
from pathlib import Path

src = Path(sys.argv[1])
makefiles = [src / "makefile"]
makefiles += [src / d / "Makefile" for d in ("scylla", "gallop", "trollop", "scabble", "hallop")]

for mk in makefiles:
    text = mk.read_text()
    if "-std=c++11" in text:
        continue
    patched = re.sub(r"(?m)^CFLAGS=", "CFLAGS=-std=c++11 ", text, count=1)
    mk.write_text(patched)
    print(f"patched {mk.relative_to(src)}")
PY

# On macOS, Homebrew's glpk/gmp headers and libraries live outside the
# compiler's default search path.
if command -v brew >/dev/null 2>&1; then
  BREW_PREFIX="$(brew --prefix)"
  export CPATH="$BREW_PREFIX/include${CPATH:+:$CPATH}"
  export LIBRARY_PATH="$BREW_PREFIX/lib${LIBRARY_PATH:+:$LIBRARY_PATH}"
fi

make -C "$SRC_DIR" -j"$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 2)"

echo
echo "Built $SRC_DIR/scallop"
echo "Verify with: python3 $HERE/compute_scl.py"
