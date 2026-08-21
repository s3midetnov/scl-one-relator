#!/usr/bin/env python3
import os
BASE = os.environ.get("OM_DATA",
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "data"))
os.makedirs(BASE, exist_ok=True)

import sys, os
from collections import defaultdict

orbits=[]   # (word, L, orbitsize, properpower, invariant tuple)
for L in range(4,21,2):
    p=os.path.join(BASE,f"orbits_{L}.txt")
    if not os.path.exists(p): continue
    for line in open(p):
        if line.startswith('#'): continue
        f=line.rstrip('\n').split('\t')
        w,sz,pp,k,p11,h1,h2 = f[0],int(f[1]),int(f[2]),int(f[3]),int(f[4]),f[5],f[6]
        orbits.append((w,len(w),sz,pp,(k,p11,h1,h2)))

print(f"total Aut(F_2)-orbits pooled (L=4..20): {len(orbits)}")
by=defaultdict(list)
for o in orbits: by[o[4]].append(o)
sizes=defaultdict(int)
for k,v in by.items(): sizes[len(v)]+=1
print(f"distinct Alexander-invariant values: {len(by)}")
print("bucket size histogram (size: #buckets):", dict(sorted(sizes.items())[:12]), "...")
coll=[v for v in by.values() if len(v)>1]
ncand=sum(len(v)*(len(v)-1)//2 for v in coll)
print(f"buckets with >1 orbit: {len(coll)}   orbits inside them: {sum(len(v) for v in coll)}   candidate pairs: {ncand}")
biggest=max(by.values(), key=len)
print("largest bucket:", len(biggest), "e.g.", [x[0] for x in biggest[:3]])

with open(os.path.join(BASE,"collisions.txt"),'w') as f:
    for v in sorted(coll, key=lambda v:-len(v)):
        f.write("BUCKET %d %s\n" % (len(v), str(v[0][4])))
        for w,L,sz,pp,inv in v: f.write("  %s\t%d\t%d\t%d\n" % (w,L,sz,pp))
print("written", os.path.join(BASE,"collisions.txt"))
