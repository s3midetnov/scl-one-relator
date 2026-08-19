# Formal verification

`SclCounterexample/Isomorphism.lean` formalizes, in Lean 4 / Mathlib,
Theorem 1 of the paper: an explicit isomorphism

```
⟨a, b | aabABabABBAbaabABBAb⟩ ≅ ⟨a, b | aabABabABabABBAbaBAb⟩
```

between the two one-relator groups, built from Lemma 1's certificate
(two mutually-inverse free-group homomorphisms, checked on generators via
six explicit free-group identities). The file contains no `sorry` and
depends only on the standard axioms `propext`, `Classical.choice`,
`Quot.sound`.

Only the *isomorphism* is formalized. The scl values themselves (`scl r = 1`,
`scl r' = 1/2`, the actual content that makes the pair a counterexample) are
computed by an external solver — see `scl/` at the repository root.

## Building

Requires [`elan`](https://github.com/leanprover/elan) (the Lean toolchain
manager). The pinned toolchain and Mathlib revision are fixed by
`lean-toolchain` / `lake-manifest.json`.

```sh
lake update   # fetch Mathlib (first run only; downloads several GB)
lake build
```

`lake build` type-checks the file; a clean exit with no `sorry`/`axiom`
warnings is the proof. To inspect the axioms directly:

```sh
lake env lean --run - <<'EOF'
#print axioms SclCounterexample.groupIso
EOF
```

should print exactly `propext`, `Classical.choice`, `Quot.sound`.

## Reading guide

- `word` reads a string over `{a, A, b, B}` into `F := FreeGroup (Fin 2)`.
- `r`, `r'` are the two relators; `phi`, `psi` are the maps `φ`, `ψ` of
  Lemma 1, given directly on generators.
- `cert_one` … `cert_fourB` are the six word identities; each is closed by
  `decide`, i.e. by the kernel actually reducing both sides of a free-group
  equation and checking they match.
- `Phi`, `Psi` are the induced homomorphisms `G' →* G` and `G →* G'`, built
  from `cert_one`/`cert_two` via `PresentedGroup.toGroup`.
- `Phi_comp_Psi`, `Psi_comp_Phi` (from `cert_threeA/B`, `cert_fourA/B`) say
  the two homomorphisms are mutually inverse; `groupIso` packages them as
  the `MulEquiv`.
