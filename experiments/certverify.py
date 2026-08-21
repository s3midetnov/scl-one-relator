#!/usr/bin/env python3
import os
BASE = os.environ.get("OM_DATA",
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "data"))
os.makedirs(BASE, exist_ok=True)
"""INDEPENDENT verification of the certificates produced by cert.rs.

Nothing here trusts the Rust code. For each claimed membership u in <<r>> we
re-run a Dehn reduction in Python, RECORD the conjugators, and then check the
resulting identity  u == prod_i (A_i c_i A_i^-1)  as an exact equality of
FREELY REDUCED WORDS in F(a,b). That identity is a complete proof of membership.
"""

import subprocess, sys
INV = {'a':'A','A':'a','b':'B','B':'b'}
def red(w):
    o=[]
    for c in w:
        if o and INV[o[-1]]==c: o.pop()
        else: o.append(c)
    return ''.join(o)
def iv(w): return ''.join(INV[c] for c in reversed(w))

def pieces(r):
    n=len(r); ri=iv(r); out=[]
    for src in (r,ri):
        for i in range(n): out.append(src[i:]+src[:i])
    return out

def dehn_with_witness(u, r, budget=4000):
    """Return list of (A,c) with u == prod A c A^-1 freely, or None."""
    pcs = pieces(r)
    w = red(u); wit=[]
    for _ in range(budget):
        if not w: return wit
        best=None
        for i in range(len(w)):
            for c in pcs:
                m=len(c); k=0
                while k<m and i+k<len(w) and w[i+k]==c[k]: k+=1
                if 2*k>m:
                    g=2*k-m
                    if best is None or g>best[0]: best=(g,i,c,k)
        if best is None: return None
        _,i,c,k = best
        A = w[:i]
        wit.append((A,c))
        w = red(w[:i] + iv(c[k:]) + w[i+k:])
    return None

def check_membership(u, r, name):
    wit = dehn_with_witness(u, r)
    if wit is None: return False, f"{name}: NO PROOF FOUND"
    prod = ''
    for A,c in wit: prod = red(prod + A + c + iv(A))
    ok = (prod == red(u))
    return ok, f"{name}: {len(wit)} conjugators, identity holds = {ok}"

def subst(word, p, q):
    m = {'a':p,'A':iv(p),'b':q,'B':iv(q)}
    return red(''.join(m[c] for c in word))

def verify_pair(r, rp, p, q, s, t):
    """Full check of the four conditions."""
    checks = [
        (subst(rp,p,q),               r,  "(1) r'(p,q) in <<r>>"),
        (subst(r,s,t),                rp, "(2) r(s,t) in <<r'>>"),
        (red(subst(s,p,q)+'A'),       r,  "(4a) s(p,q)=a mod <<r>>"),
        (red(subst(t,p,q)+'B'),       r,  "(4b) t(p,q)=b mod <<r>>"),
        (red(subst(p,s,t)+'A'),       rp, "(3a) p(s,t)=x mod <<r'>>"),
        (red(subst(q,s,t)+'B'),       rp, "(3b) q(s,t)=y mod <<r'>>"),
    ]
    allok=True; msgs=[]
    for u, rel, nm in checks:
        if not u:
            msgs.append(f"{nm}: freely trivial"); continue
        ok,msg = check_membership(u, rel, nm)
        allok &= ok; msgs.append(msg)
    # sanity: the tuple must be an H_1 isomorphism
    def ev(w):
        return (sum(1 if c=='a' else -1 if c=='A' else 0 for c in w),
                sum(1 if c=='b' else -1 if c=='B' else 0 for c in w))
    (pa,pb),(qa,qb)=ev(p),ev(q)
    det=pa*qb-pb*qa
    msgs.append(f"det on H_1 = {det}")
    allok &= abs(det)==1
    return allok, msgs

if __name__=="__main__":
    lines=[l.split() for l in open(os.path.join(BASE,"cert7.txt")) if 'CERTIFIED' in l]
    print(f"verifying {len(lines)} certificates\n")
    good=[]
    for f in lines:
        r, rp = f[0], f[1]
        p = f[3].split('->')[1]; q = f[4].split('->')[1]
        s = f[6].split('->')[1]; t = f[7].split('->')[1]
        ok,msgs = verify_pair(r,rp,p,q,s,t)
        print(f"{'PASS' if ok else 'FAIL'}  r={r} (L={len(r)})  r'={rp} (L={len(rp)})")
        for m in msgs: print("      ",m)
        if ok: good.append((r,rp))
        print()
    print("independently verified isomorphisms:",len(good))
    open(os.path.join(BASE,"verified_pairs.txt"),"w").write(
        "\n".join(f"{a} {b}" for a,b in good)+"\n")
