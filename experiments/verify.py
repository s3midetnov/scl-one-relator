#!/usr/bin/env python3
import os
BASE = os.environ.get("OM_DATA",
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "data"))
os.makedirs(BASE, exist_ok=True)
"""Independent brute-force check of the Rust engine (small L only).
Words are tuples of chars from 'aAbB'. Everything recomputed from scratch."""
import itertools, sys
from collections import deque

INV = {'a':'A','A':'a','b':'B','B':'b'}

def cyc_red(w):
    # free reduction
    out=[]
    for c in w:
        if out and INV[out[-1]]==c: out.pop()
        else: out.append(c)
    # cyclic reduction
    while len(out)>=2 and INV[out[0]]==out[-1]:
        out.pop(); out.pop(0)
    return ''.join(out)

def inv(w): return ''.join(INV[c] for c in reversed(w))

def canon(w):
    n=len(w)
    cands=[w[i:]+w[:i] for i in range(n)]+[inv(w)[i:]+inv(w)[:i] for i in range(n)]
    return min(cands)

def in_commutator(w):
    return sum(1 if c=='a' else -1 if c=='A' else 0 for c in w)==0 and \
           sum(1 if c=='b' else -1 if c=='B' else 0 for c in w)==0

def all_classes(n):
    """all canonical cyclic/inversion classes of cyclically reduced words of length n in F'"""
    seen=set()
    for t in itertools.product('aAbB', repeat=n):
        w=''.join(t)
        if any(INV[w[i]]==w[i+1] for i in range(n-1)): continue
        if INV[w[-1]]==w[0]: continue
        if not in_commutator(w): continue
        seen.add(canon(w))
    return seen

def whitehead_moves():
    """substitution dicts. Type I: signed permutations of {a,b}. Type II: multiplier moves."""
    mvs=[]
    gens=['a','b']
    for perm in [(0,1),(1,0)]:
        for s0 in [0,1]:
            for s1 in [0,1]:
                s=[s0,s1]
                d={}
                for gi,g in enumerate(gens):
                    tgt=gens[perm[gi]]
                    img = tgt if s[gi]==0 else INV[tgt]
                    d[g]=img; d[INV[g]]=INV[img]
                if all(d[c]==c for c in 'aAbB'): continue
                mvs.append(d)
    for m in 'aAbB':
        mb=INV[m]
        x = 'b' if m in 'aA' else 'a'
        xi=INV[x]
        for l in [0,1]:
            for r in [0,1]:
                if l==0 and r==0: continue
                d={m:m, mb:mb}
                d[x]=(mb if l else '')+x+(m if r else '')
                d[xi]=(mb if r else '')+xi+(m if l else '')
                mvs.append(d)
    return mvs

MOVES=whitehead_moves()

def apply(w,d):
    return cyc_red(''.join(d[c] for c in w))

def orbits(n):
    classes=all_classes(n)
    minimal=set()
    for w in classes:
        if all(len(apply(w,d))>=n for d in MOVES): minimal.add(w)
    seen=set(); orbs=[]
    for w in sorted(minimal):
        if w in seen: continue
        comp=set([w]); dq=deque([w])
        while dq:
            u=dq.popleft()
            for d in MOVES:
                v=apply(u,d)
                if len(v)==n:
                    v=canon(v)
                    if v not in comp:
                        comp.add(v); dq.append(v)
        seen|=comp
        orbs.append((min(comp),len(comp)))
    return len(classes), len(minimal), sorted(orbs)

if __name__=="__main__":
    for n in [4,6,8,10,12]:
        c,m,o=orbits(n)
        print(f"L={n}: classes={c} minimal={m} orbits={len(o)}")
        if n<=8:
            for rep,sz in o: print("   ",rep,sz)
