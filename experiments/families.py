#!/usr/bin/env python3
import os
BASE = os.environ.get("OM_DATA",
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "data"))
os.makedirs(BASE, exist_ok=True)
"""Structured families with r in F' -- these are where the known mechanism for
Nielsen-inequivalent generating tuples (splittings, amalgams, centres) lives."""

import itertools
INV={'a':'A','A':'a','b':'B','B':'b'}
def red(w):
    o=[]
    for c in w:
        if o and INV[o[-1]]==c: o.pop()
        else: o.append(c)
    return ''.join(o)
def inv(w): return ''.join(INV[c] for c in reversed(w))
def comm(u,v): return red(u+v+inv(u)+inv(v))
def words(maxlen):
    out=['']
    cur=['']
    for _ in range(maxlen):
        nxt=[]
        for w in cur:
            for c in 'aAbB':
                if w and INV[w[-1]]==c: continue
                nxt.append(w+c)
        out+=nxt; cur=nxt
    return out

fam={}
def add(tag,w):
    w=red(w)
    if not w: return
    if sum(1 if c=='a' else -1 if c=='A' else 0 for c in w)!=0: return
    if sum(1 if c=='b' else -1 if c=='B' else 0 for c in w)!=0: return
    fam.setdefault(w,tag)

# (A) [a^m, b^n]  -- amalgams Z *_Z Z^2 *_Z Z, and [a,b^n] which has a centre
for m in range(1,7):
    for n in range(1,7):
        add(f"A:[a^{m},b^{n}]", comm('a'*m,'b'*n))
# (B) general commutators [u,v] with |u|,|v| <= 4
W=[w for w in words(4) if w]
for u in W:
    for v in W:
        add("B:[u,v]", comm(u,v))
# (C) link-like  [a, w],  |w| <= 9   (covers 2-bridge link group presentations)
for w in words(9):
    if w: add("C:[a,w]", comm('a',w))
# (D) proper powers u^k, u in F', |u| <= 6, k=2,3
for u in words(6):
    if not u: continue
    if sum(1 if c=='a' else -1 if c=='A' else 0 for c in u)!=0: continue
    if sum(1 if c=='b' else -1 if c=='B' else 0 for c in u)!=0: continue
    for k in (2,3): add(f"D:u^{k}", u*k)
# (E) products of two commutators [u,v][x,y], short
S=[w for w in words(2) if w]
for u in S:
    for v in S:
        for x in S:
            for y in S:
                add("E:[u,v][x,y]", comm(u,v)+comm(x,y))
# (F) conjugated/twisted: [a^m, w b^n w^-1]
for m in range(1,4):
    for n in range(1,4):
        for w in words(3):
            add(f"F:[a^m,wb^nw^-1]", comm('a'*m, red(w+'b'*n+inv(w))))

with open(os.path.join(BASE,"families.txt"),"w") as f:
    for w in fam: f.write(w+"\n")
with open(os.path.join(BASE,"families_tag.txt"),"w") as f:
    for w,t in fam.items(): f.write(w+"\t"+t+"\n")
from collections import Counter
print("distinct relators generated:",len(fam))
print(Counter(t.split(':')[0] for t in fam.values()))
print("max length:",max(len(w) for w in fam))
