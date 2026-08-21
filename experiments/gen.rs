// Given a relator r, find pairs (p,q) that GENERATE G_r, together with words (s,t)
// expressing a,b in terms of them:  s(p,q) = a  and  t(p,q) = b  mod <<r>>.
// Such a pair yields a new presentation of the SAME group, with relation kernel
//   <<psi(r), psi(p)x^-1, psi(q)y^-1>>,   psi: a->s, b->t.
use std::io::{BufRead, Write};

fn freered(v: &[u8]) -> Vec<u8> {
    let mut o: Vec<u8> = Vec::with_capacity(v.len());
    for &c in v { if let Some(&t) = o.last() { if t == (c ^ 1) { o.pop(); continue; } } o.push(c); }
    o
}
fn inv(w: &[u8]) -> Vec<u8> { w.iter().rev().map(|&c| c ^ 1).collect() }

fn pieces(r: &[u8]) -> Vec<Vec<u8>> {
    let n = r.len(); let ri = inv(r); let mut v = Vec::with_capacity(2 * n);
    for src in [r, &ri[..]] { for i in 0..n { v.push((0..n).map(|k| src[(i + k) % n]).collect()); } }
    v
}

fn dehn_trivial(u: &[u8], pcs: &[Vec<u8>], budget: usize) -> bool {
    let mut w = freered(u); let mut steps = 0usize;
    loop {
        if w.is_empty() { return true; }
        if steps > budget { return false; }
        steps += 1;
        let mut best: Option<(usize, usize, usize)> = None;
        for i in 0..w.len() { for (pi, c) in pcs.iter().enumerate() {
            let m = c.len(); let mut k = 0usize;
            while k < m && i + k < w.len() && w[i + k] == c[k] { k += 1; }
            if 2 * k > m { let g = 2 * k - m; if best.is_none() || g > best.unwrap().0 { best = Some((g, i, pi)); } }
        }}
        match best { None => return false, Some((_, i, pi)) => {
            let c = &pcs[pi]; let m = c.len(); let mut k = 0usize;
            while k < m && i + k < w.len() && w[i + k] == c[k] { k += 1; }
            let tail = inv(&c[k..]);
            let mut nw: Vec<u8> = Vec::with_capacity(w.len());
            nw.extend_from_slice(&w[..i]); nw.extend_from_slice(&tail); nw.extend_from_slice(&w[i + k..]);
            w = freered(&nw);
        }}
    }
}

fn subst(word: &[u8], p: &[u8], q: &[u8]) -> Vec<u8> {
    let pi = inv(p); let qi = inv(q);
    let mut out = Vec::new();
    for &c in word { let src: &[u8] = match c { 0 => p, 1 => &pi, 2 => q, _ => &qi }; for &d in src { out.push(d); } }
    freered(&out)
}

fn tuples(maxlen: usize, ea0: i32, eb0: i32) -> Vec<Vec<u8>> {
    let mut out = Vec::new(); let mut cur: Vec<u8> = Vec::new();
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
            rec(cur, maxlen, na, nb, ea0, eb0, out); cur.pop();
        }
    }
    rec(&mut cur, maxlen, 0, 0, ea0, eb0, &mut out); out
}

fn s2w(s: &str) -> Vec<u8> { s.bytes().map(|c| match c { b'a'|b'x' => 0, b'A'|b'X' => 1, b'b'|b'y' => 2, _ => 3 }).collect() }
fn w2s(w: &[u8]) -> String { w.iter().map(|&c| ['a','A','b','B'][c as usize]).collect() }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // separate bounds: p-bound, q-bound, and bound for the inverse words s,t
    let pb: usize = args.get(1).map(|x| x.parse().unwrap()).unwrap_or(7);
    let qb: usize = args.get(2).map(|x| x.parse().unwrap()).unwrap_or(7);
    let sb: usize = args.get(3).map(|x| x.parse().unwrap()).unwrap_or(7);
    let budget: usize = args.get(4).map(|x| x.parse().unwrap()).unwrap_or(400);
    let ps = tuples(pb, 1, 0);
    let qs = tuples(qb, 0, 1);
    let ss = tuples(sb, 1, 0);
    let ts = tuples(sb, 0, 1);
    eprintln!("pb={} qb={} sb={}: |P|={} |Q|={} |S|={} |T|={}", pb, qb, sb,
              ps.len(), qs.len(), ss.len(), ts.len());

    let stdin = std::io::stdin(); let so = std::io::stdout();
    let mut o = std::io::BufWriter::new(so.lock());
    for line in stdin.lock().lines() {
        let l = line.unwrap(); let t0 = l.trim();
        if t0.is_empty() || t0.starts_with('#') { continue; }
        let r = s2w(t0);
        let pcs = pieces(&r);
        let mut found = 0usize;
        for p in &ps { for q in &qs {
            // identity tuple and its inner conjugates are uninteresting
            if p.len() == 1 && q.len() == 1 { continue; }
            // s with s(p,q) = a
            let mut sfound: Option<Vec<u8>> = None;
            for s in &ss {
                let mut u = subst(s, p, q); u.push(1); u = freered(&u);
                if u.is_empty() || dehn_trivial(&u, &pcs, budget) { sfound = Some(s.clone()); break; }
            }
            if sfound.is_none() { continue; }
            let mut tfound: Option<Vec<u8>> = None;
            for t in &ts {
                let mut u = subst(t, p, q); u.push(3); u = freered(&u);
                if u.is_empty() || dehn_trivial(&u, &pcs, budget) { tfound = Some(t.clone()); break; }
            }
            if let Some(t) = tfound {
                let s = sfound.unwrap();
                writeln!(o, "{}\t{}\t{}\t{}\t{}", t0, w2s(p), w2s(q), w2s(&s), w2s(&t)).unwrap();
                found += 1;
            }
        }}
        o.flush().unwrap();
        eprintln!("  {} : {} generating pairs certified", t0, found);
    }
}
