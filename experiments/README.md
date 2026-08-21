# orbit-mining

Tools for enumerating `Aut(F_2)`-orbits of relators in the commutator subgroup
`F' = [F_2, F_2]`, filtering pairs of orbits by isomorphism invariants of the
associated one-relator groups, and searching for explicit isomorphism certificates.

These were written to search for pairs `r, r' in F'` lying in **different**
`Aut(F_2)`-orbits with `<a,b | r>` isomorphic to `<a,b | r'>`. Such a pair
necessarily has `scl(r)` and `scl(r')` unconstrained by each other, since `scl` is
an `Aut(F_2)`-invariant; this is how the counterexample to Question 1.3(1) of
Heuer–Löh was found.

Everything runs on one core. The full search to length 20 takes about 40 minutes.

## Conventions

Words are strings over `a A b B`, with `A = a^-1` and `B = b^-1`. A relator is
always a **cyclically reduced** word, considered up to cyclic rotation and
inversion (Magnus: the normal closure `<<r>>` depends on exactly this data).
`r in F'` means both exponent sums vanish.

Two relators are *obviously isomorphic* if some `f in Aut(F_2)` carries `r` to a
conjugate of `r'^{±1}`. All invariants below are constant on `Aut(F_2)`-orbits.

## Requirements

- `rustc` (no cargo, no crates)
- `python3`
- GAP, for the low-index subgroup stage — invoke by absolute path, see note below
- optional: [`scallop`](https://github.com/aldenwalker/scallop) for `scl`

```
make          # builds wh, wcanon, hom, cert, gen
make groups   # writes data/groups.txt, data/groups2.txt (Cayley tables)
make check    # small-length agreement against the independent brute force
```

Data files are written to `./data`, or to `$OM_DATA` if set.

## The tools

### Enumeration

**`wh.rs`** — for a given even length `L`: enumerates cyclically reduced words in
`F'` by DFS with exponent-sum pruning, reduces modulo rotation and inversion
(canonical form via O(1) bit-rotations on a packed `u64`), filters for
Whitehead-minimality, and partitions into `Aut(F_2)`-orbits by union-find over the
19 nontrivial Whitehead automorphisms of `F_2` (7 of type I, 12 of type II). Also
emits the Alexander invariant of each orbit representative.

```
./wh 20 data/orbits_20.txt
# [L=20] cyclic classes in F': 1264945
# [L=20] Whitehead-minimal: 1067151
# [L=20] Aut(F_2)-orbits: 94401          (8.6 s)
```

Output columns: `word  orbitsize  properpower  supp(Delta)  Delta(1,1)  h1  h2`,
where `h1,h2` hash `GL_2(Z)`-invariants of the Alexander polynomial's support.

**`wcanon.rs`** — same Whitehead machinery but for words of **arbitrary** length:
greedy Whitehead descent to minimal length, then the canonical orbit
representative, orbit size, and Alexander data. Use this to test whether two
words of any length are `Aut(F_2)`-equivalent.

```
echo abababABABAB | ./wcanon     # -> minlen 8, canon aaabAAAB : (ab)^3(ba)^-3 ~ [a^3,b]
```

### Invariants

The Alexander polynomial comes from Fox calculus. Since `r in F'`, the syzygy
`D_a(t-1) + D_b(s-1) = 0` forces `D_a = (s-1)P`, and `P` is well defined up to
units and up to the `GL_2(Z)` change of variables induced by `Aut(H_1)`. The code
asserts the syzygy at runtime. Invariants used are the coefficient multiset plus
translation- and `GL_2(Z)`-invariant functions of the support.

**`hom.rs`** — computes `|Hom(G_r, Q)|` for each `Q` in a Cayley-table file.

```
./hom data/groups.txt < words.txt
```

**`mkgroups.py`** — builds Cayley tables by closure from generators.
`groups.txt` has 14 groups up to order 60; `groups2.txt` has
`S5, SL(2,5), A6, SL(2,7), S6` for the harder stratum. Note that when
`|Delta(1,1)| = 1` every nilpotent quotient of `G_r` is a quotient of `Z^2`, so
p-quotients are useless there and only non-solvable `Q` discriminate.

**`drive.py` / `li_batch.g`** — homology of low-index subgroups via GAP. This is
by far the strongest filter in the pipeline. GAP is OOM-killed after a few
thousand finitely presented groups in one session, so `drive.py` runs it in fresh
chunks with resume and per-chunk timeouts.

```
python3 drive.py 5     # index <= 5, reads data/surv_words.txt, writes data/li.out
```

### Filtering

**`bucket.py`** — pools all orbits and buckets them by Alexander invariant.
**`combine.py`** — applies Hom counts and low-index homology, reports the residue
stratified by the Hopf invariant `|Delta(1,1)|` and by length.

Every filter is a genuine isomorphism invariant, so **no filter can separate
isomorphic groups**: the surviving pairs are a complete superset of the
counterexample pairs in the range searched.

### Certificates

**`cert.rs`** — searches for a two-sided isomorphism certificate for a pair
`r, r'`. It looks for `p, q, s, t` with

```
(1) r'(p,q) in <<r>>              (2) r(s,t) in <<r'>>
(3) p(s,t)=x, q(s,t)=y mod <<r'>> (4) s(p,q)=a, t(p,q)=b mod <<r>>
```

which make the induced maps mutually inverse, so `G_r ≅ G_{r'}`. No Hopficity is
assumed. Membership in a normal closure is decided by a Dehn-style rewriting
search: **sound always** (each rewrite multiplies by a conjugate of `r^{±1}`),
complete only for `C'(1/6)`. Failure to certify is therefore never a proof of
non-isomorphism.

Search space is cut by two reductions. First, one may assume the tuple is
`H_1`-normalised, `p in aF'` and `q in bF'`. Second, `ker(Aut(F_2) -> GL_2(Z)) =
Inn(F_2)`, so such a tuple is either inner (nothing new) or not an automorphism of
`F_2` at all.

```
echo "r r'" | ./cert 7 400 0      # tuple bound 7, Dehn budget 400, H_1-normalised mode
```

Mode `1` drops the normalisation and filters on `det = ±1` instead; useful for
controls.

**`gen.rs`** — the same machinery but for a *single* relator: finds pairs `(p,q)`
that generate `G_r`, together with words expressing `a` and `b` back in terms of
them. This yields new presentations of the same group without needing a second
relator in advance: the new relation kernel is
`<<psi(r), psi(p)x^-1, psi(q)y^-1>>` with `psi: a->s, b->t`, which GAP's Tietze
routines will usually reduce to a single relator.

```
echo r | ./gen 5 11 7 400      # bounds for p, for q, for s and t
```

### Verification

These exist because a bug in an "invariant" that is not actually
`Aut(F_2)`-invariant would silently split genuine counterexamples apart and hide
them forever.

- **`verify.py`** — an independent brute-force reimplementation of the
  enumeration and orbit computation. Agrees with `wh.rs` exactly on counts of
  classes, Whitehead-minimal words and orbits for `L = 4,6,8,10,12`, and on
  orbit-size multisets at `L = 8`.
- **`invcheck.py`** — expands sampled orbits to all their members and checks the
  Alexander invariant and the Hom-count vector are constant on each.
- **`certverify.py`** — re-derives certificates independently, extracting the
  conjugators and checking each membership as an exact identity of freely reduced
  words `u = prod_i A_i c_i A_i^-1`, with every `c_i` verified to be a cyclic
  conjugate of the relator. This is a finite, elementary, hand-checkable proof;
  it does not depend on any of the search code being correct.
- **`families.py`** — generates structured relator families (`[a^m,b^n]`,
  general commutators, `[a,w]`, proper powers, products of commutators) as an
  independent cross-check on the exhaustive enumeration.

## A worked run

```
for L in 4 6 8 10 12 14 16 18 20; do ./wh $L data/orbits_$L.txt; done
make groups
python3 bucket.py                       # Alexander buckets -> data/collisions.txt
./hom data/groups.txt < words > data/homs.txt
python3 drive.py 5                      # low-index homology
python3 combine.py                      # residue -> data/FINAL_candidates.txt
./cert 7 400 0 < pairs.txt              # certificates
```

For `L <= 20` this funnel goes from `1.07e5` orbits (about `5.8e9` pairs) to 235
pairs, of which 8 were certified isomorphic at tuple bound 7.

## Notes and caveats

- On some systems `gap` is shadowed by another command; call GAP by absolute
  path (`/usr/bin/gap`). `drive.py` does this.
- The Dehn prover is one-sided. Negative results mean "not found", not "false".
- `make check` runs the small-length agreement test and takes a few minutes.

## Licence

Do as you like. If it is useful in published work, a mention is welcome.
