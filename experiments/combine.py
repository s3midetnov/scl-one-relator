#!/usr/bin/env python3
import os
BASE = os.environ.get("OM_DATA",
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "data"))
os.makedirs(BASE, exist_ok=True)

import os, pickle
from collections import defaultdict

hom = {}
for line in open(os.path.join(BASE,"homs.txt")):
    if line.startswith('#'): continue
    f = line.rstrip().split('\t'); hom[f[0]] = tuple(f[1:])

li = {}
for line in open(os.path.join(BASE,"li.out")):
    w, prof = line.split(' ', 1)
    li[w] = prof.strip()

# Alexander invariant + Hopf invariant m = |P(1,1)| for every orbit rep
alex, hopf, Ldict = {}, {}, {}
for L in range(4, 21, 2):
    p = os.path.join(BASE,f"orbits_{L}.txt")
    if not os.path.exists(p): continue
    for line in open(p):
        if line.startswith('#'): continue
        f = line.rstrip().split('\t')
        alex[f[0]] = (int(f[3]), int(f[4]), f[5], f[6])
        hopf[f[0]] = abs(int(f[4])); Ldict[f[0]] = len(f[0])

surv = pickle.load(open(os.path.join(BASE,"surv.pkl"),"rb"))
words = [w for v in surv for (w, L, sz, pp) in v]
miss = [w for w in words if w not in li]
print(f"orbits entering stage 3: {len(words)}   unresolved by GAP: {len(miss)}")

buckets = defaultdict(list)
for v in surv:
    for (w, L, sz, pp) in v:
        key = (alex[w], hom[w], li.get(w, 'UNRESOLVED:' + w))
        buckets[key].append((w, L, sz, pp))

final = [b for b in buckets.values() if len(b) > 1]
npairs = sum(len(b) * (len(b) - 1) // 2 for b in final)
print(f"\nAFTER Alexander + Hom(14) + LowIndexHomology(index<=5):")
print(f"  buckets with >1 orbit : {len(final)}")
print(f"  orbits inside them    : {sum(len(b) for b in final)}")
print(f"  candidate pairs       : {npairs}")
h = defaultdict(int)
for b in final: h[len(b)] += 1
print(f"  bucket-size histogram : {dict(sorted(h.items()))}")

print("\nStratification of surviving orbits by Hopf invariant m = |Delta(1,1)|:")
st = defaultdict(int)
for b in final:
    for (w, L, sz, pp) in b: st[hopf[w]] += 1
tot = sum(st.values())
for k in sorted(st):
    print(f"   |m| = {k:<3}  {st[k]:>6}  ({100*st[k]/tot:.1f}%)")

print("\nLength profile of surviving pairs (L,L'):")
lp = defaultdict(int)
for b in final:
    ls = sorted(Ldict[w] for (w, L, sz, pp) in b)
    for i in range(len(ls)):
        for j in range(i+1, len(ls)): lp[(ls[i], ls[j])] += 1
for k in sorted(lp): print(f"   {k}: {lp[k]}")

with open(os.path.join(BASE,"FINAL_candidates.txt"),"w") as f:
    for b in sorted(final, key=lambda b: (-len(b), b[0][0])):
        f.write("BUCKET size=%d  |m|=%d\n" % (len(b), hopf[b[0][0]]))
        for (w, L, sz, pp) in sorted(b): f.write("   %-22s L=%d orbit=%d\n" % (w, L, sz))
print("\nwrote FINAL_candidates.txt")
