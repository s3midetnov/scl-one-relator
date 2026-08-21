// For each relator word on stdin, print |Hom(<a,b|r>, Q)| for each group Q in groups.txt
use std::io::{BufRead, Write};

struct Grp { name: String, n: usize, tab: Vec<u16>, inv: Vec<u16>, e: u16 }

fn load(path: &str) -> Vec<Grp> {
    let s = std::fs::read_to_string(path).unwrap();
    let mut it = s.lines();
    let k: usize = it.next().unwrap().trim().parse().unwrap();
    let mut out = Vec::new();
    for _ in 0..k {
        let hdr: Vec<&str> = it.next().unwrap().split_whitespace().collect();
        let name = hdr[0].to_string();
        let n: usize = hdr[1].parse().unwrap();
        let tab: Vec<u16> = it.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap() as u16).collect();
        assert_eq!(tab.len(), n * n);
        // identity
        let mut e = 0u16;
        for i in 0..n { if (0..n).all(|j| tab[i * n + j] as usize == j) { e = i as u16; } }
        let mut inv = vec![0u16; n];
        for i in 0..n { for j in 0..n { if tab[i * n + j] == e { inv[i] = j as u16; } } }
        out.push(Grp { name, n, tab, inv, e });
    }
    out
}

fn main() {
    let groups = load(&std::env::args().nth(1).unwrap());
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut w = std::io::BufWriter::new(stdout.lock());
    write!(w, "#word").unwrap();
    for g in &groups { write!(w, "\t{}", g.name).unwrap(); }
    writeln!(w).unwrap();
    let mut word: Vec<u8> = Vec::with_capacity(32);
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') { continue; }
        word.clear();
        for c in t.bytes() {
            word.push(match c { b'a' => 0, b'A' => 1, b'b' => 2, b'B' => 3, _ => 255 });
        }
        if word.iter().any(|&c| c == 255) { continue; }
        write!(w, "{}", t).unwrap();
        for g in &groups {
            let (n, tab, e) = (g.n, &g.tab, g.e);
            let mut cnt: u64 = 0;
            for x in 0..n {
                let xi = g.inv[x];
                for y in 0..n {
                    let img = [x as u16, xi, y as u16, g.inv[y]];
                    let mut cur = e;
                    for &c in word.iter() {
                        cur = tab[(cur as usize) * n + (img[c as usize] as usize)];
                    }
                    if cur == e { cnt += 1; }
                }
            }
            write!(w, "\t{}", cnt).unwrap();
        }
        writeln!(w).unwrap();
    }
}
