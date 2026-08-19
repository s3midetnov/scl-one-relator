# Isomorphic one-relator groups need not have relators of equal scl

Companion repository to the paper *Isomorphic one-relator groups need not
have relators of equal stable commutator length* (`paper/`).

Heuer and Löh ask whether $\langle S \mid r\rangle \cong \langle S' \mid
r'\rangle$, for relators $r, r'$ in the respective commutator subgroups,
forces $\mathrm{scl}_S(r) = \mathrm{scl}_{S'}(r')$. The paper answers this
negatively with an explicit pair of length-20 words in $F(a,b)'$:

```
r  = aabABabABBAbaabABBAb    scl(r)  = 1
r' = aabABabABabABBAbaBAb    scl(r') = 1/2
```

with $\langle a,b \mid r\rangle \cong \langle a,b \mid r'\rangle$.

This repository backs both halves of that claim with independently checkable
computation:

| | claim | how it's checked |
|---|---|---|
| [`lean/`](lean) | $\langle a,b\mid r\rangle \cong \langle a,b\mid r'\rangle$ | Lean 4 / Mathlib proof, no `sorry` |
| [`scl/`](scl) | $\mathrm{scl}(r) = 1$, $\mathrm{scl}(r') = 1/2$ | wrapper around [`scallop`](https://github.com/aldenwalker/scallop) |

Neither depends on the other: the isomorphism is exhibited by an explicit
certificate (Lemma 1 of the paper) that is checked purely combinatorially in
the free group, with no reference to scl; the scl values are computed by a
general-purpose solver with no reference to the isomorphism.

## Contents

- `paper/scl-counterexample.tex` — the paper.
- `lean/` — formal proof that $G_r \cong G_{r'}$. See `lean/README.md`.
- `scl/` — recomputes $\mathrm{scl}(r)$, $\mathrm{scl}(r')$, and the other
  scl values quoted in the paper. See `scl/README.md`.

## Quick start

```sh
# formal proof of the isomorphism
cd lean && lake update && lake build

# recompute the scl values
cd scl && ./setup_scallop.sh && python3 compute_scl.py
```

## Acknowledgements

The Lean formalization was produced with the assistance of
[Aristotle](https://aristotle.harmonic.fun). scl values are computed with
[`scallop`](https://github.com/aldenwalker/scallop), Alden Walker's
implementation of Calegari's algorithm for stable commutator length in free
groups.
