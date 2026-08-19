#!/usr/bin/env python3
"""Check that lean/SclCounterexample/Isomorphism.lean's r, r' literals match
words.tex.

words.tex is the canonical source: the paper \\input{}s it and
scl/relators.py parses it directly, so those two can't drift. The Lean file
hardcodes the same two words instead of parsing words.tex at compile time --
embedding the file via `include_str` and extracting the macro bodies inside
the kernel made every `decide` in that file reduce a nontrivial search
through the file's characters, and that blew up kernel reduction time badly
enough to be untenable. This script is the substitute: a cheap, no-Lean-
toolchain-required check that the two copies still agree, meant to be run
after editing either file (and safe to wire into CI).
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WORDS_TEX = ROOT / "words.tex"
ISOMORPHISM_LEAN = ROOT / "lean" / "SclCounterexample" / "Isomorphism.lean"


def macro_body(text: str, name: str) -> str:
    match = re.search(rf"\\newcommand\{{\\{name}\}}\{{([a-zA-Z]+)\}}", text)
    if not match:
        raise ValueError(f"macro \\{name} not found in {WORDS_TEX}")
    return match.group(1)


def lean_literal(text: str, def_name: str) -> str:
    match = re.search(rf'def {re.escape(def_name)} : F := word "([a-zA-Z]+)"', text)
    if not match:
        raise ValueError(f"`def {def_name} : F := word \"...\"` not found in {ISOMORPHISM_LEAN}")
    return match.group(1)


def main() -> None:
    words_text = WORDS_TEX.read_text()
    lean_text = ISOMORPHISM_LEAN.read_text()

    pairs = [
        ("r", macro_body(words_text, "rWord"), lean_literal(lean_text, "r")),
        ("r'", macro_body(words_text, "rPrimeWord"), lean_literal(lean_text, "r'")),
    ]

    ok = True
    for name, from_tex, from_lean in pairs:
        match = from_tex == from_lean
        ok &= match
        status = "ok" if match else "MISMATCH"
        print(f"{name}: words.tex={from_tex}  lean={from_lean}  [{status}]")

    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
