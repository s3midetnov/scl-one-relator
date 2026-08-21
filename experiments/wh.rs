// Aut(F_2)-orbit enumeration for cyclically reduced words in F' = [F_2,F_2]
// letters: 0='a' 1='A' 2='b' 3='B' ; inverse = xor 1 ; gen = c>>1
use std::io::Write;

type W = u64;

#[inline(always)]
fn inverse(w: W, n: usize) -> W {
    let mut r = 0u64;
    let mut x = w;
    for _ in 0..n {
        let c = x & 3;
        x >>= 2;
        r = (r << 2) | (c ^ 1);
    }
    r
}

#[inline(always)]
fn canon(w: W, n: usize) -> W {
    let bits = 2 * n;
    let mask = (1u64 << bits) - 1;
    let mut best = w;
    let mut x = w;
    for _ in 1..n {
        x = ((x << 2) | (x >> (bits - 2))) & mask;
        if x < best { best = x; }
    }
    let mut y = inverse(w, n);
    if y < best { best = y; }
    for _ in 1..n {
        y = ((y << 2) | (y >> (bits - 2))) & mask;
        if y < best { best = y; }
    }
    best
}

fn unpack(w: W, n: usize) -> Vec<u8> {
    (0..n).map(|i| ((w >> (2 * (n - 1 - i))) & 3) as u8).collect()
}

fn to_str(w: W, n: usize) -> String {
    unpack(w, n).iter().map(|&c| ['a', 'A', 'b', 'B'][c as usize]).collect()
}

// ---------- Stage 1: enumerate canonical cyclic classes of length n in F' ----------
// canonical rep is lex-min over rotations+inversion; any cyclically reduced word in F'
// contains the letter 'a' (code 0), so the canonical rep starts with 'a'.
fn enumerate(n: usize) -> Vec<W> {
    let mut out: Vec<W> = Vec::new();
    let mut stack: Vec<(usize, W, i32, i32, u8)> = Vec::with_capacity(n + 2);
    // start: first letter 'a'
    stack.push((1usize, 0u64, 1i32, 0i32, 0u8));
    // iterative DFS
    fn rec(n: usize, depth: usize, packed: W, ea: i32, eb: i32, last: u8, out: &mut Vec<W>) {
        if depth == n {
            if last != 1 && ea == 0 && eb == 0 {   // cyclically reduced AND in F'
                if canon(packed, n) == packed { out.push(packed); }
            }
            return;
        }
        let rem = (n - depth) as i32;
        let d = ea.abs() + eb.abs();
        if d > rem || ((rem - d) & 1) != 0 { return; }
        let forb = last ^ 1;
        for c in 0u8..4 {
            if c == forb { continue; }
            let (na, nb) = match c {
                0 => (ea + 1, eb),
                1 => (ea - 1, eb),
                2 => (ea, eb + 1),
                _ => (ea, eb - 1),
            };
            rec(n, depth + 1, (packed << 2) | (c as u64), na, nb, c, out);
        }
    }
    stack.clear();
    rec(n, 1, 0u64, 1, 0, 0u8, &mut out);
    out
}

// ---------- Whitehead automorphisms of F_2 ----------
// a move is a substitution table: image of each of the 4 letters as a short word
#[derive(Clone)]
struct Move { img: [Vec<u8>; 4] }

fn whitehead_moves() -> Vec<Move> {
    let mut mv = Vec::new();
    // Type I: signed permutations of {a,b}  (8 total, identity included but harmless/skipped)
    for perm in 0..2usize {
        for s0 in 0..2u8 {
            for s1 in 0..2u8 {
                if perm == 0 && s0 == 0 && s1 == 0 { continue; } // identity
                let s = [s0, s1];
                let mut img: [Vec<u8>; 4] = [vec![], vec![], vec![], vec![]];
                for c in 0u8..4 {
                    let g = (c >> 1) as usize;
                    let ib = c & 1;
                    let ng = if perm == 0 { g } else { 1 - g };
                    img[c as usize] = vec![((ng as u8) << 1) | (ib ^ s[g])];
                }
                mv.push(Move { img });
            }
        }
    }
    // Type II: multiplier m in {a,A,b,B}; other generator x; x -> m^{-l} x m^{r}
    for m in 0u8..4 {
        let mbar = m ^ 1;
        let xg = 1 - (m >> 1);           // the other generator index
        let x = xg << 1;                 // positive letter
        let xi = x | 1;                  // its inverse
        for opt in 1..4u8 {              // skip opt=0 (identity)
            let l = opt & 1;
            let r = (opt >> 1) & 1;
            let mut img: [Vec<u8>; 4] = [vec![], vec![], vec![], vec![]];
            img[m as usize] = vec![m];
            img[mbar as usize] = vec![mbar];
            let mut wx = Vec::new();
            if l == 1 { wx.push(mbar); }
            wx.push(x);
            if r == 1 { wx.push(m); }
            let mut wxi = Vec::new();
            if r == 1 { wxi.push(mbar); }
            wxi.push(xi);
            if l == 1 { wxi.push(m); }
            img[x as usize] = wx;
            img[xi as usize] = wxi;
            mv.push(Move { img });
        }
    }
    mv
}

// apply move to cyclic word, return cyclically reduced result
fn apply_move(word: &[u8], m: &Move, buf: &mut Vec<u8>) {
    buf.clear();
    for &c in word {
        for &d in &m.img[c as usize] {
            if let Some(&t) = buf.last() {
                if t == (d ^ 1) { buf.pop(); continue; }
            }
            buf.push(d);
        }
    }
    // cyclic reduction
    while buf.len() >= 2 && buf[0] == (buf[buf.len() - 1] ^ 1) {
        buf.pop();
        buf.remove(0);
    }
}

fn pack(v: &[u8]) -> W {
    let mut w = 0u64;
    for &c in v { w = (w << 2) | (c as u64); }
    w
}

// ---------- Alexander polynomial via Fox calculus ----------
// D_a = (d r / d a)^ab ;  D_a = (s-1) * P  ; return support of P as (alpha,beta,coeff)
fn alexander(word: &[u8], n: usize) -> Vec<(i32, i32, i64)> {
    let off = (n / 2 + 2) as i32;
    let sz = (2 * off + 2) as usize;
    let mut d = vec![0i64; sz * sz];
    let idx = |a: i32, b: i32| -> usize { ((a + off) as usize) * sz + ((b + off) as usize) };
    let (mut al, mut be) = (0i32, 0i32);
    for &c in word {
        match c {
            0 => { d[idx(al, be)] += 1; al += 1; }
            1 => { al -= 1; d[idx(al, be)] -= 1; }
            2 => { be += 1; }
            _ => { be -= 1; }
        }
    }
    // divide each alpha-row by (s-1):  q_{b-1} = c_b + q_b
    let mut out = Vec::new();
    for a in -off..=off {
        // find beta range
        let mut hi = i32::MIN;
        let mut lo = i32::MAX;
        for b in -off..=off {
            if d[idx(a, b)] != 0 { if b > hi { hi = b; } if b < lo { lo = b; } }
        }
        if hi == i32::MIN { continue; }
        let mut q = 0i64;
        let mut b = hi;
        while b >= lo {
            q = d[idx(a, b)] + q;      // q_{b-1}
            if q != 0 { out.push((a, b - 1, q)); }
            b -= 1;
        }
        // syzygy D_a*(t-1) + D_b*(s-1) = 0 forces exact divisibility of D_a by (s-1)
        assert!(q == 0, "Fox syzygy violated: D_a not divisible by (s-1)");
    }
    out
}

fn gcd(a: i64, b: i64) -> i64 { let (mut a, mut b) = (a.abs(), b.abs()); while b != 0 { let t = a % b; a = b; b = t; } a }

// GL_2(Z)+translation+sign invariant fingerprint of the support
fn alex_invariant(pts: &[(i32, i32, i64)]) -> (usize, i64, u64, u64) {
    if pts.is_empty() { return (0, 0, 0, 0); }
    let mut best: Option<(usize, i64, u64, u64)> = None;
    for sgn in [1i64, -1i64] {
        let p: Vec<(i32, i32, i64)> = pts.iter().map(|&(a, b, c)| (a, b, sgn * c)).collect();
        let k = p.len();
        let mut coeffs: Vec<i64> = p.iter().map(|x| x.2).collect();
        coeffs.sort();
        let p11: i64 = coeffs.iter().sum();
        // pair invariants: (c_i*c_j, gcd of difference vector)
        let mut pairs: Vec<(i64, i64)> = Vec::new();
        for i in 0..k { for j in (i + 1)..k {
            let dv = gcd((p[j].0 - p[i].0) as i64, (p[j].1 - p[i].1) as i64);
            pairs.push((p[i].2 * p[j].2, dv));
        }}
        pairs.sort();
        // triple invariants: (|det|, product of coeffs)
        let mut tris: Vec<(i64, i64)> = Vec::new();
        for i in 0..k { for j in (i + 1)..k { for l in (j + 1)..k {
            let d1 = ((p[j].0 - p[i].0) as i64, (p[j].1 - p[i].1) as i64);
            let d2 = ((p[l].0 - p[i].0) as i64, (p[l].1 - p[i].1) as i64);
            let det = (d1.0 * d2.1 - d1.1 * d2.0).abs();
            tris.push((det, p[i].2 * p[j].2 * p[l].2));
        }}}
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

// is the word a proper power (cyclically)?  returns k>1 if w = u^k
fn proper_power(word: &[u8]) -> usize {
    let n = word.len();
    for d in 1..n {
        if n % d == 0 {
            if (0..n).all(|i| word[i] == word[i % d]) { return n / d; }
        }
    }
    1
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args[1].parse().unwrap();
    let outpath = args.get(2).cloned().unwrap_or_else(|| format!("orbits_{}.txt", n));

    let t0 = std::time::Instant::now();
    let all = enumerate(n);
    eprintln!("[L={}] cyclic classes in F': {}  ({:.1}s)", n, all.len(), t0.elapsed().as_secs_f64());

    let moves = whitehead_moves();
    // Stage 2a: Whitehead minimality
    let mut buf: Vec<u8> = Vec::with_capacity(3 * n + 4);
    let mut minimal: Vec<W> = Vec::new();
    for &w in &all {
        let word = unpack(w, n);
        let mut ok = true;
        for m in &moves {
            apply_move(&word, m, &mut buf);
            if buf.len() < n { ok = false; break; }
        }
        if ok { minimal.push(w); }
    }
    eprintln!("[L={}] Whitehead-minimal: {}  ({:.1}s)", n, minimal.len(), t0.elapsed().as_secs_f64());

    // Stage 2b: union-find over length-preserving Whitehead moves
    let sorted = minimal.clone(); // already ascending (DFS emits ascending? ensure)
    let mut sorted = sorted; sorted.sort_unstable();
    let idx_of = |x: W| -> Option<usize> { sorted.binary_search(&x).ok() };
    let m = sorted.len();
    let mut parent: Vec<u32> = (0..m as u32).collect();
    fn find(parent: &mut Vec<u32>, mut x: u32) -> u32 {
        while parent[x as usize] != x { parent[x as usize] = parent[parent[x as usize] as usize]; x = parent[x as usize]; }
        x
    }
    for i in 0..m {
        let word = unpack(sorted[i], n);
        for mo in &moves {
            apply_move(&word, mo, &mut buf);
            if buf.len() == n {
                let c = canon(pack(&buf), n);
                if let Some(j) = idx_of(c) {
                    let (ra, rb) = (find(&mut parent, i as u32), find(&mut parent, j as u32));
                    if ra != rb { parent[ra as usize] = rb; }
                }
            }
        }
    }
    let mut orbit_of: std::collections::HashMap<u32, Vec<usize>> = std::collections::HashMap::new();
    for i in 0..m { let r = find(&mut parent, i as u32); orbit_of.entry(r).or_default().push(i); }
    eprintln!("[L={}] Aut(F_2)-orbits: {}  ({:.1}s)", n, orbit_of.len(), t0.elapsed().as_secs_f64());

    // Stage 3: invariants, one line per orbit
    let mut f = std::io::BufWriter::new(std::fs::File::create(&outpath).unwrap());
    let mut orbs: Vec<(W, usize, (usize, i64, u64, u64), usize)> = Vec::new();
    for (_, members) in orbit_of.iter() {
        let rep = members.iter().map(|&i| sorted[i]).min().unwrap();
        let word = unpack(rep, n);
        let pts = alexander(&word, n);
        let inv = alex_invariant(&pts);
        orbs.push((rep, members.len(), inv, proper_power(&word)));
    }
    orbs.sort();
    writeln!(f, "# L={} orbits={} classes={} minimal={}", n, orbs.len(), all.len(), m).unwrap();
    for (rep, sz, inv, pp) in &orbs {
        writeln!(f, "{}\t{}\t{}\t{}\t{}\t{:016x}\t{:016x}", to_str(*rep, n), sz, pp, inv.0, inv.1, inv.2, inv.3).unwrap();
    }
    eprintln!("[L={}] done ({:.1}s) -> {}", n, t0.elapsed().as_secs_f64(), outpath);
}
