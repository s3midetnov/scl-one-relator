"""Words whose stable commutator length is quoted in the paper.

Every ``expected`` value below is a claim made in ``paper/scl-counterexample.tex``
(the calibration data in the main text, and the two theorem relators r, r'), plus
the bonus pair from the closing remark. ``compute_scl.py`` recomputes each with
scallop and checks it against this table.

r and r' themselves are not hardcoded: they are parsed out of ``words.tex`` at
the repository root, the same file the paper \\input{}s and lean/ embeds, so
the three cannot silently drift apart.
"""

import re
from dataclasses import dataclass
from fractions import Fraction
from pathlib import Path

WORDS_TEX = Path(__file__).resolve().parent.parent / "words.tex"


def _load_macro(name: str) -> str:
    """Extract \\newcommand{\\<name>}{<word>} from words.tex."""
    text = WORDS_TEX.read_text()
    match = re.search(rf"\\newcommand\{{\\{name}\}}\{{([a-zA-Z]+)\}}", text)
    if not match:
        raise ValueError(f"macro \\{name} not found in {WORDS_TEX}")
    return match.group(1)


R = _load_macro("rWord")
R_PRIME = _load_macro("rPrimeWord")


@dataclass(frozen=True)
class Relator:
    name: str
    word: str
    expected: Fraction
    source: str


RELATORS = [
    Relator(
        "calibration: abAB",
        "abAB",
        Fraction(1, 2),
        "Duncan-Howie sharp lower bound scl >= 1/2 on F'",
    ),
    Relator(
        "calibration: [a1,b1][a2,b2]",
        "abABcdCD",
        Fraction(3, 2),
        "genus-2 surface relator, rank 4",
    ),
    Relator(
        "calibration: Heuer-Loh Example 1.1",
        "aaaabABAbaBAAbAB",
        Fraction(5, 8),
        "paper, calibration sentence",
    ),
    Relator(
        f"theorem: r  = {R}",
        R,
        Fraction(1, 1),
        "paper, Theorem 1 (also formalized in lean/)",
    ),
    Relator(
        f"theorem: r' = {R_PRIME}",
        R_PRIME,
        Fraction(1, 2),
        "paper, Theorem 1: G_r isomorphic to G_r' but scl differs",
    ),
    Relator(
        "remark: aabaBAAbABabAB",
        "aabaBAAbABabAB",
        Fraction(3, 4),
        "paper, closing remark (second example pair)",
    ),
    Relator(
        "remark: aabaBAAbaBAbABaabAAB",
        "aabaBAAbaBAbABaabAAB",
        Fraction(2, 3),
        "paper, closing remark (second example pair)",
    ),
]
