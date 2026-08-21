// Positive-direction certificate search.
// Given r, r' in F(a,b), look for words p,q (images of x,y) and s,t (images of a,b) with
//   (1) r'(p,q) in <<r>>        -> phi: G' -> G
//   (2) r(s,t)  in <<r'>>       -> psi: G  -> G'
//   (3) p(s,t) = x, q(s,t) = y  mod <<r'>>   (psi o phi = id_{G'})
//   (4) s(p,q) = a, t(p,q) = b  mod <<r>>    (phi o psi = id_{G})
// All four are normal-closure membership tests, done by a SOUND one-sided Dehn-style
// prover (complete for C'(1/6); "not proved" never means "false").
use std::io::{BufRead, Write};

fn freered(v: &[u8]) -> Vec<u8> {
    let mut o: Vec<u8> = Vec::with_capacity(v.len());
    for &c in v { if let Some(&t) = o.last() { if t == (c ^ 1) { o.pop(); continue; } } o.push(c); }
    o
}
fn inv(w: &[u8]) -> Vec<u8> { w.iter().rev().map(|&c| c ^ 1).collect() }

/// all cyclic conjugates of r and r^{-1}
fn pieces(r: &[u8]) -> Vec<Vec<u8>> {
    let n = r.len();
    let ri = inv(r);
    let mut v = Vec::with_capacity(2 * n);
    for src in [r, &ri[..]] {
        for i in 0..n { v.push((0..n).map(|k| src[(i + k) % n]).collect()); }
    }
    v
}

/// Sound one-sided test for u in <<r>>: greedy Dehn reduction.
/// Each rewrite multiplies by a conjugate of r^{+-1}, so reaching the empty word is a PROOF.
fn dehn_trivial(u: &[u8], pcs: &[Vec<u8>], budget: usize) -> bool {
    let mut w = freered(u);
    let mut steps = 0usize;
    loop {
        if w.is_empty() { return true; }
        if steps > budget { return false; }
        steps += 1;
        let mut best: Option<(usize, usize, usize)> = None; // (gain, pos, piece)
        for i in 0..w.len() {
            for (pi, c) in pcs.iter().enumerate() {
                let m = c.len();
                let mut k = 0usize;
                while k < m && i + k < w.len() && w[i + k] == c[k] { k += 1; }
                if 2 * k > m {
                    let gain = 2 * k - m;
                    if best.is_none() || gain > best.unwrap().0 { best = Some((gain, i, pi)); }
                }
            }
        }
        match best {
            None => return false,
            Some((_, i, pi)) => {
                let c = &pcs[pi];
                let m = c.len();
                let mut k = 0usize;
                while k < m && i + k < w.len() && w[i + k] == c[k] { k += 1; }
                let tail = inv(&c[k..]);
                let mut nw: Vec<u8> = Vec::with_capacity(w.len());
                nw.extend_from_slice(&w[..i]);
                nw.extend_from_slice(&tail);
                nw.extend_from_slice(&w[i + k..]);
                w = freered(&nw);
            }
        }
    }
}

/// substitute: word over {x,y} (codes 0/1 = x/X, 2/3 = y/Y) -> word over {a,b} using p,q
fn subst(word: &[u8], p: &[u8], q: &[u8]) -> Vec<u8> {
    let pi = inv(p); let qi = inv(q);
    let mut out = Vec::new();
    for &c in word {
        let src: &[u8] = match c { 0 => p, 1 => &pi, 2 => q, _ => &qi };
        for &d in src { out.push(d); }
    }
    freered(&out)
}

/// all reduced words of length <= maxlen with given exponent vector
fn tuples(maxlen: usize, ea0: i32, eb0: i32) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut cur: Vec<u8> = Vec::new();
    fn rec(cur: &mut Vec<u8>, maxlen: usize, ea: i32, eb: i32, ea0: i32, eb0: i32, out: &mut Vec<Vec<u8>>) {
        if ea == ea0 && eb == eb0 { out.push(cur.clone()); }
        if cur.len() == maxlen { return; }
        let rem = (maxlen - cur.len()) as i32;
        if (ea - ea0).abs() + (eb - eb0).abs() > rem { return; }
        let forb = cur.last().map(|&c| c ^ 1).unwrap_or(255);
        for c in 0u8..4 {
            if c == forb { continue; }
            cur.push(c);
            let (na, nb) = match c { 0 => (ea + 1, eb), 1 => (ea - 1, eb), 2 => (ea, eb + 1), _ => (ea, eb - 1) };
            rec(cur, maxlen, na, nb, ea0, eb0, out);
            cur.pop();
        }
    }
    rec(&mut cur, maxlen, 0, 0, ea0, eb0, &mut out);
    out
}

fn expv(w: &[u8]) -> (i32, i32) {
    let a = w.iter().map(|&c| if c==0 {1} else if c==1 {-1} else {0}).sum();
    let b = w.iter().map(|&c| if c==2 {1} else if c==3 {-1} else {0}).sum();
    (a, b)
}
fn allwords(maxlen: usize) -> Vec<Vec<u8>> {
    let mut out = vec![]; let mut cur: Vec<Vec<u8>> = vec![vec![]];
    out.push(vec![]);
    for _ in 0..maxlen {
        let mut nx = vec![];
        for w in &cur { for c in 0u8..4 {
            if let Some(&t) = w.last() { if t == (c^1) { continue; } }
            let mut v = w.clone(); v.push(c); nx.push(v); }}
        out.extend(nx.iter().cloned()); cur = nx;
    }
    out.retain(|w| !w.is_empty());
    out
}
fn s2w(s: &str) -> Vec<u8> {
    s.bytes().map(|c| match c { b'a' | b'x' => 0, b'A' | b'X' => 1, b'b' | b'y' => 2, _ => 3 }).collect()
}
fn w2s(w: &[u8]) -> String { w.iter().map(|&c| ['a', 'A', 'b', 'B'][c as usize]).collect() }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bound: usize = args.get(1).map(|x| x.parse().unwrap()).unwrap_or(5);
    let budget: usize = args.get(2).map(|x| x.parse().unwrap()).unwrap_or(400);

    let mode: usize = args.get(3).map(|x| x.parse().unwrap()).unwrap_or(0);
    // mode 0 (production): H_1-normalized, p in aF', q in bF'.
    // mode 1 (control/broad): all reduced words, filtered later to det = +-1 on H_1.
    let (ps, qs) = if mode == 0 { (tuples(bound, 1, 0), tuples(bound, 0, 1)) }
                   else { (allwords(bound), allwords(bound)) };
    eprintln!("mode {} bound {}: |P|={} |Q|={} -> {} tuples/pair", mode, bound, ps.len(), qs.len(), ps.len()*qs.len());

    let stdin = std::io::stdin();
    let so = std::io::stdout();
    let mut o = std::io::BufWriter::new(so.lock());
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let t = l.trim();
        if t.is_empty() || t.starts_with('#') { continue; }
        let f: Vec<&str> = t.split_whitespace().collect();
        if f.len() < 2 { continue; }
        let r = s2w(f[0]);
        let rp = s2w(f[1]);
        let pcs_r = pieces(&r);
        let pcs_rp = pieces(&rp);
        // step (1): find (p,q) with r'(p,q) in <<r>>
        let mut hits: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        for p in &ps {
            for q in &qs {
                let (pa, pb) = expv(p); let (qa, qb) = expv(q);
                if (pa*qb - pb*qa).abs() != 1 { continue; }
                let img = subst(&rp, p, q);
                if img.is_empty() || dehn_trivial(&img, &pcs_r, budget) {
                    hits.push((p.clone(), q.clone()));
                }
            }
        }
        // trivial hits: p,q generating F freely with r'(p,q) freely trivial cannot happen
        let mut cert = None;
        'outer: for (p, q) in &hits {
            for s in &ps {
                for tq in &qs {
                    let (sa, sb) = expv(s); let (ta, tb) = expv(tq);
                    if (sa*tb - sb*ta).abs() != 1 { continue; }
                    if !dehn_trivial(&subst(&r, s, tq), &pcs_rp, budget) { continue; }
                    // (4) s(p,q) = a, t(p,q) = b  mod <<r>>
                    let mut sp = subst(s, p, q); sp.push(1); sp = freered(&sp);
                    if !dehn_trivial(&sp, &pcs_r, budget) { continue; }
                    let mut tp = subst(tq, p, q); tp.push(3); tp = freered(&tp);
                    if !dehn_trivial(&tp, &pcs_r, budget) { continue; }
                    // (3) p(s,t) = x, q(s,t) = y  mod <<r'>>
                    let mut ps2 = subst(p, s, tq); ps2.push(1); ps2 = freered(&ps2);
                    if !dehn_trivial(&ps2, &pcs_rp, budget) { continue; }
                    let mut qs2 = subst(q, s, tq); qs2.push(3); qs2 = freered(&qs2);
                    if !dehn_trivial(&qs2, &pcs_rp, budget) { continue; }
                    cert = Some((p.clone(), q.clone(), s.clone(), tq.clone()));
                    break 'outer;
                }
            }
        }
        match cert {
            Some((p, q, s, tq)) => writeln!(o, "{} {} CERTIFIED x->{} y->{} | a->{} b->{}",
                                            f[0], f[1], w2s(&p), w2s(&q), w2s(&s), w2s(&tq)).unwrap(),
            None => writeln!(o, "{} {} none  (partial hits: {})", f[0], f[1], hits.len()).unwrap(),
        }
    }
}
