// Reads words (aAbB) on stdin. For each: Whitehead-reduce to minimal length,
// compute the canonical Aut(F_2)-orbit representative, orbit size, and Alexander data.
use std::collections::HashSet;
use std::io::{BufRead, Write};

#[derive(Clone)]
struct Move { img: [Vec<u8>; 4] }

fn whitehead_moves() -> Vec<Move> {
    let mut mv = Vec::new();
    for perm in 0..2usize { for s0 in 0..2u8 { for s1 in 0..2u8 {
        if perm == 0 && s0 == 0 && s1 == 0 { continue; }
        let s = [s0, s1];
        let mut img: [Vec<u8>; 4] = [vec![], vec![], vec![], vec![]];
        for c in 0u8..4 {
            let g = (c >> 1) as usize; let ib = c & 1;
            let ng = if perm == 0 { g } else { 1 - g };
            img[c as usize] = vec![((ng as u8) << 1) | (ib ^ s[g])];
        }
        mv.push(Move { img });
    }}}
    for m in 0u8..4 {
        let mbar = m ^ 1; let xg = 1 - (m >> 1); let x = xg << 1; let xi = x | 1;
        for opt in 1..4u8 {
            let l = opt & 1; let r = (opt >> 1) & 1;
            let mut img: [Vec<u8>; 4] = [vec![], vec![], vec![], vec![]];
            img[m as usize] = vec![m]; img[mbar as usize] = vec![mbar];
            let mut wx = Vec::new();
            if l == 1 { wx.push(mbar); } wx.push(x); if r == 1 { wx.push(m); }
            let mut wxi = Vec::new();
            if r == 1 { wxi.push(mbar); } wxi.push(xi); if l == 1 { wxi.push(m); }
            img[x as usize] = wx; img[xi as usize] = wxi;
            mv.push(Move { img });
        }
    }
    mv
}

fn cyc_reduce(v: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(v.len());
    for &c in v {
        if let Some(&t) = out.last() { if t == (c ^ 1) { out.pop(); continue; } }
        out.push(c);
    }
    let mut i = 0usize; let mut j = out.len();
    while j - i >= 2 && out[i] == (out[j - 1] ^ 1) { i += 1; j -= 1; }
    out[i..j].to_vec()
}

fn apply_move(w: &[u8], m: &Move) -> Vec<u8> {
    let mut buf = Vec::with_capacity(3 * w.len());
    for &c in w { for &d in &m.img[c as usize] { buf.push(d); } }
    cyc_reduce(&buf)
}

fn inverse(w: &[u8]) -> Vec<u8> { w.iter().rev().map(|&c| c ^ 1).collect() }

fn canon(w: &[u8]) -> Vec<u8> {
    let n = w.len();
    if n == 0 { return vec![]; }
    let iw = inverse(w);
    let mut best: Option<Vec<u8>> = None;
    for src in [w, &iw[..]] {
        for i in 0..n {
            let cand: Vec<u8> = (0..n).map(|k| src[(i + k) % n]).collect();
            if best.is_none() || cand < *best.as_ref().unwrap() { best = Some(cand); }
        }
    }
    best.unwrap()
}

/// Whitehead's theorem: if w is not of minimal length in its Aut-orbit,
/// some single Whitehead automorphism strictly shortens it. Greedy descent is complete.
fn reduce_min(w: &[u8], mv: &[Move]) -> Vec<u8> {
    let mut cur = cyc_reduce(w);
    'outer: loop {
        for m in mv {
            let nx = apply_move(&cur, m);
            if nx.len() < cur.len() { cur = nx; continue 'outer; }
        }
        return cur;
    }
}

fn orbit(w: &[u8], mv: &[Move], cap: usize) -> (Vec<u8>, usize, bool) {
    let n = w.len();
    let start = canon(w);
    let mut seen: HashSet<Vec<u8>> = HashSet::new();
    seen.insert(start.clone());
    let mut stack = vec![start.clone()];
    let mut best = start;
    let mut truncated = false;
    while let Some(u) = stack.pop() {
        for m in mv {
            let v = apply_move(&u, m);
            if v.len() == n {
                let c = canon(&v);
                if !seen.contains(&c) {
                    if seen.len() >= cap { truncated = true; continue; }
                    if c < best { best = c.clone(); }
                    seen.insert(c.clone()); stack.push(c);
                }
            }
        }
    }
    (best, seen.len(), truncated)
}

fn gcd(a: i64, b: i64) -> i64 { let (mut a, mut b) = (a.abs(), b.abs()); while b != 0 { let t = a % b; a = b; b = t; } a }

fn alexander(w: &[u8]) -> Vec<(i32, i32, i64)> {
    let off = (w.len() / 2 + 2) as i32;
    let sz = (2 * off + 2) as usize;
    let mut d = vec![0i64; sz * sz];
    let idx = |a: i32, b: i32| -> usize { ((a + off) as usize) * sz + ((b + off) as usize) };
    let (mut al, mut be) = (0i32, 0i32);
    for &c in w {
        match c { 0 => { d[idx(al, be)] += 1; al += 1; }
                  1 => { al -= 1; d[idx(al, be)] -= 1; }
                  2 => { be += 1; } _ => { be -= 1; } }
    }
    let mut out = Vec::new();
    for a in -off..=off {
        let (mut hi, mut lo) = (i32::MIN, i32::MAX);
        for b in -off..=off { if d[idx(a, b)] != 0 { if b > hi { hi = b; } if b < lo { lo = b; } } }
        if hi == i32::MIN { continue; }
        let mut q = 0i64; let mut b = hi;
        while b >= lo { q = d[idx(a, b)] + q; if q != 0 { out.push((a, b - 1, q)); } b -= 1; }
        assert!(q == 0, "Fox syzygy violated");
    }
    out
}

fn alex_inv(pts: &[(i32, i32, i64)]) -> (usize, i64, u64, u64) {
    if pts.is_empty() { return (0, 0, 0, 0); }
    let mut best: Option<(usize, i64, u64, u64)> = None;
    for sgn in [1i64, -1i64] {
        let p: Vec<(i32, i32, i64)> = pts.iter().map(|&(a, b, c)| (a, b, sgn * c)).collect();
        let k = p.len();
        let mut coeffs: Vec<i64> = p.iter().map(|x| x.2).collect(); coeffs.sort();
        let p11: i64 = coeffs.iter().sum();
        let mut pairs: Vec<(i64, i64)> = Vec::new();
        for i in 0..k { for j in (i + 1)..k {
            pairs.push((p[i].2 * p[j].2, gcd((p[j].0 - p[i].0) as i64, (p[j].1 - p[i].1) as i64))); }}
        pairs.sort();
        let mut tris: Vec<(i64, i64)> = Vec::new();
        for i in 0..k { for j in (i + 1)..k { for l in (j + 1)..k {
            let d1 = ((p[j].0 - p[i].0) as i64, (p[j].1 - p[i].1) as i64);
            let d2 = ((p[l].0 - p[i].0) as i64, (p[l].1 - p[i].1) as i64);
            tris.push(((d1.0 * d2.1 - d1.1 * d2.0).abs(), p[i].2 * p[j].2 * p[l].2)); }}}
        tris.sort();
        let mut h1: u64 = 0xcbf29ce484222325;
        for &c in &coeffs { h1 = (h1 ^ (c as u64)).wrapping_mul(0x100000001b3); }
        for &(x, y) in &pairs { h1 = (h1 ^ (x as u64)).wrapping_mul(0x100000001b3); h1 = (h1 ^ (y as u64)).wrapping_mul(0x100000001b3); }
        let mut h2: u64 = 0xcbf29ce484222325;
        for &(x, y) in &tris { h2 = (h2 ^ (x as u64)).wrapping_mul(0x100000001b3); h2 = (h2 ^ (y as u64)).wrapping_mul(0x100000001b3); }
        let cand = (k, p11, h1, h2);
        if best.is_none() || cand < best.unwrap() { best = Some(cand); }
    }
    best.unwrap()
}

fn s2w(s: &str) -> Option<Vec<u8>> {
    let mut v = Vec::new();
    for c in s.bytes() { v.push(match c { b'a' => 0, b'A' => 1, b'b' => 2, b'B' => 3, _ => return None }); }
    Some(v)
}
fn w2s(w: &[u8]) -> String { w.iter().map(|&c| ['a', 'A', 'b', 'B'][c as usize]).collect() }

fn main() {
    let mv = whitehead_moves();
    let stdin = std::io::stdin();
    let so = std::io::stdout();
    let mut o = std::io::BufWriter::new(so.lock());
    writeln!(o, "#input\tminlen\tcanon\torbitsize\ttrunc\tsupp\tP11\th1\th2").unwrap();
    for line in stdin.lock().lines() {
        let l = line.unwrap(); let t = l.trim();
        if t.is_empty() || t.starts_with('#') { continue; }
        let w = match s2w(t) { Some(w) => w, None => continue };
        let ea: i32 = w.iter().map(|&c| if c==0 {1} else if c==1 {-1} else {0}).sum();
        let eb: i32 = w.iter().map(|&c| if c==2 {1} else if c==3 {-1} else {0}).sum();
        if ea != 0 || eb != 0 { writeln!(o, "{}\tNOT_IN_F'", t).unwrap(); continue; }
        let m = reduce_min(&w, &mv);
        if m.is_empty() { writeln!(o, "{}\t0\t<trivial>\t0\t0\t0\t0\t0\t0", t).unwrap(); continue; }
        let (rep, sz, tr) = orbit(&m, &mv, 400000);
        let inv = alex_inv(&alexander(&rep));
        writeln!(o, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:016x}\t{:016x}",
                 t, m.len(), w2s(&rep), sz, if tr { 1 } else { 0 }, inv.0, inv.1, inv.2, inv.3).unwrap();
    }
}
