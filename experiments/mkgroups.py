#!/usr/bin/env python3
import os
BASE = os.environ.get("OM_DATA",
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "data"))
os.makedirs(BASE, exist_ok=True)
"""Build Cayley tables of small groups by closure from generators."""

def close(gens, mul):
    ident = None
    elems = {}
    frontier = list(gens)
    seen = set(gens)
    # find identity by powering a generator
    g = gens[0]; x = g
    while True:
        y = mul(x, g)
        if y == g: ident = x; break
        x = y
    seen.add(ident); frontier.append(ident)
    allel = set(seen)
    while frontier:
        nf = []
        for a in frontier:
            for b in gens:
                c = mul(a, b)
                if c not in allel:
                    allel.add(c); nf.append(c)
        frontier = nf
    lst = sorted(allel)
    idx = {e: i for i, e in enumerate(lst)}
    n = len(lst)
    tab = [0]*(n*n)
    for i, a in enumerate(lst):
        for j, b in enumerate(lst):
            tab[i*n+j] = idx[mul(a, b)]
    return n, tab

def pmul(a, b):  # (a*b)(i) = a(b(i))
    return tuple(a[b[i]] for i in range(len(a)))

def perm(n, cycles):
    p = list(range(n))
    for cyc in cycles:
        for k in range(len(cyc)):
            p[cyc[k]] = cyc[(k+1) % len(cyc)]
    return tuple(p)

def matmul(p):
    def f(a, b):
        return ((a[0]*b[0]+a[1]*b[2]) % p, (a[0]*b[1]+a[1]*b[3]) % p,
                (a[2]*b[0]+a[3]*b[2]) % p, (a[2]*b[1]+a[3]*b[3]) % p)
    return f

GROUPS = []
def addp(name, npts, gens):
    GROUPS.append((name,) + close([perm(npts, g) for g in gens], pmul))
def addm(name, p, gens):
    GROUPS.append((name,) + close([tuple(g) for g in gens], matmul(p)))

addp("S3",      3, [[[0,1,2]], [[0,1]]])
addp("D4",      4, [[[0,1,2,3]], [[1,3]]])
addm("Q8",      3, [[0,2,1,0], [1,1,1,2]])
addp("D5",      5, [[[0,1,2,3,4]], [[1,4],[2,3]]])
addp("A4",      4, [[[0,1,2]], [[0,1],[2,3]]])
addp("D6",      6, [[[0,1,2,3,4,5]], [[1,5],[2,4]]])
addp("C3xS3",   6, [[[0,1,2]], [[3,4,5]], [[3,4]]])
addp("D8_16",   8, [[[0,1,2,3,4,5,6,7]], [[1,7],[2,6],[3,5]]])
addp("F20",     5, [[[0,1,2,3,4]], [[1,2,4,3]]])
addp("C7C3",    7, [[[0,1,2,3,4,5,6]], [[1,2,4],[3,6,5]]])
addm("SL23",    3, [[1,1,0,1], [1,0,1,1]])
addp("S4",      4, [[[0,1,2,3]], [[0,1]]])
addp("A5",      5, [[[0,1,2,3,4]], [[0,1,2]]])
addp("S3xS3",   6, [[[0,1,2]], [[0,1]], [[3,4,5]], [[3,4]]])

with open(os.path.join(BASE,"groups.txt"), "w") as f:
    f.write("%d\n" % len(GROUPS))
    for name, n, tab in GROUPS:
        f.write("%s %d\n" % (name, n))
        f.write(" ".join(map(str, tab)) + "\n")
        print(name, "order", n)

# ---- larger, non-nilpotent groups for the |m|=1 stratum ----
GROUPS2 = []
def addp2(name, npts, gens):
    GROUPS2.append((name,) + close([perm(npts, g) for g in gens], pmul))
def addm2(name, p, gens):
    GROUPS2.append((name,) + close([tuple(g) for g in gens], matmul(p)))
addp2("S5",     5, [[[0,1,2,3,4]], [[0,1]]])
addm2("SL25",   5, [[1,1,0,1], [1,0,1,1]])
addp2("A6",     6, [[[0,1,2,3,4]], [[3,4,5]]])
addm2("SL27",   7, [[1,1,0,1], [1,0,1,1]])
addp2("S6",     6, [[[0,1,2,3,4,5]], [[0,1]]])
with open(os.path.join(BASE,"groups2.txt"),"w") as f:
    f.write("%d\n" % len(GROUPS2))
    for name,n,tab in GROUPS2:
        f.write("%s %d\n"%(name,n)); f.write(" ".join(map(str,tab))+"\n")
        print("BIG:",name,"order",n)

