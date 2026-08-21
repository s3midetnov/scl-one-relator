#!/usr/bin/env python3
import os
BASE = os.environ.get("OM_DATA",
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "data"))
os.makedirs(BASE, exist_ok=True)
"""SOUNDNESS CHECK: every invariant used must be constant on Aut(F_2)-orbits.
Independently reimplements the Alexander invariant and compares (a) to the Rust
output on orbit representatives, (b) across all members of each orbit."""

import subprocess, random, sys
from collections import deque
from math import gcd
from verify import MOVES, apply, canon, cyc_red, INV

# ---- independent Alexander invariant, written from the definition ----
def alexander_pts(w):
    """D_a = (dr/da)^ab as dict (alpha,beta)->coeff, then divide by (s-1)."""
    D = {}
    al = be = 0
    for c in w:
        if c == 'a':
            D[(al, be)] = D.get((al, be), 0) + 1; al += 1
        elif c == 'A':
            al -= 1; D[(al, be)] = D.get((al, be), 0) - 1
        elif c == 'b': be += 1
        else: be -= 1
    D = {k: v for k, v in D.items() if v != 0}
    # sanity: syzygy D_a*(t-1) + D_b*(s-1) = 0  =>  (s-1) | D_a
    P = {}
    alphas = sorted(set(k[0] for k in D))
    for a in alphas:
        row = {k[1]: v for k, v in D.items() if k[0] == a}
        hi, lo = max(row), min(row)
        q = 0
        for b in range(hi, lo - 1, -1):
            q = row.get(b, 0) + q
            if q: P[(a, b - 1)] = q
        assert q == 0, "not divisible by (s-1)"
    return sorted((a, b, c) for (a, b), c in P.items())

def alex_inv(pts):
    if not pts: return (0, 0, (), ())
    best = None
    for sgn in (1, -1):
        p = [(a, b, sgn * c) for a, b, c in pts]
        k = len(p)
        coeffs = tuple(sorted(x[2] for x in p))
        p11 = sum(coeffs)
        pairs = tuple(sorted((p[i][2] * p[j][2], gcd(p[j][0] - p[i][0], p[j][1] - p[i][1]))
                             for i in range(k) for j in range(i + 1, k)))
        tris = tuple(sorted((abs((p[j][0]-p[i][0])*(p[l][1]-p[i][1]) - (p[j][1]-p[i][1])*(p[l][0]-p[i][0])),
                             p[i][2]*p[j][2]*p[l][2])
                            for i in range(k) for j in range(i+1,k) for l in range(j+1,k)))
        cand = (k, p11, coeffs, pairs, tris)
        if best is None or cand < best: best = cand
    return best

def orbit(w):
    n = len(w); comp = {canon(w)}; dq = deque([canon(w)])
    while dq:
        u = dq.popleft()
        for d in MOVES:
            v = apply(u, d)
            if len(v) == n:
                v = canon(v)
                if v not in comp: comp.add(v); dq.append(v)
    return sorted(comp)

if __name__ == "__main__":
    random.seed(7)
    reps = [l.split()[0] for l in open(os.path.join(BASE,"surv_words.txt"))]
    sample = random.sample(reps, 60)
    allw, tag = [], []
    for r in sample:
        orb = orbit(r)
        for u in orb: allw.append(u); tag.append(r)
    print(f"{len(sample)} orbits, {len(allw)} total members")

    # (b) Alexander invariant constant on orbits?
    bad = 0
    for r in sample:
        vals = {alex_inv(alexander_pts(u)) for u in orbit(r)}
        if len(vals) != 1: bad += 1; print("  !! Alexander NOT invariant on orbit of", r)
    print("Alexander invariant constant on all sampled orbits:", bad == 0)

    # (b) Hom counts constant on orbits?
    out = subprocess.run([os.path.join(os.path.dirname(os.path.abspath(__file__)),"hom"), os.path.join(BASE,"groups.txt")], input="\n".join(allw), capture_output=True, text=True).stdout
    hv = {}
    for line in out.splitlines():
        if line.startswith('#'): continue
        f = line.split('\t'); hv[f[0]] = tuple(f[1:])
    bad = 0
    for r in sample:
        vals = {hv[u] for u in orbit(r)}
        if len(vals) != 1: bad += 1; print("  !! Hom NOT invariant on orbit of", r)
    print("Hom-count vector constant on all sampled orbits:", bad == 0)

    # (a) agreement with Rust on representatives (k and P(1,1) columns)
    rust = {}
    import os
    for L in range(4, 21, 2):
        p = os.path.join(BASE,f"orbits_{L}.txt")
        if not os.path.exists(p): continue
        for line in open(p):
            if line.startswith('#'): continue
            f = line.rstrip().split('\t'); rust[f[0]] = (int(f[3]), int(f[4]))
    mism = [r for r in sample if rust[r] != alex_inv(alexander_pts(r))[:2]]
    print("Rust vs Python Alexander (support size, P(1,1)) agree on all reps:", not mism)
