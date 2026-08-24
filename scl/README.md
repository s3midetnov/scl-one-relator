# scl computation

Recomputes every stable-commutator-length value quoted in the paper, using
[`scallop`](https://github.com/aldenwalker/scallop) (Alden Walker's
implementation of Calegari's rationality algorithm for scl in free groups) as
the solver. `scallop` is a dependency, not part of this repository: it is
fetched and built on demand into `scl/vendor/`, which is gitignored.

## Setup

Requires `git`, `make`, a C++11 compiler, and the GLPK and GMP development
libraries.

```sh
# macOS
brew install glpk gmp

# Debian / Ubuntu
sudo apt install libglpk-dev libgmp-dev
```

Then, from this directory:

```sh
./setup_scallop.sh
```

This clones `scallop` into `scl/vendor/scallop`, patches its Makefiles to add
`-std=c++11` (the upstream code predates C++11 and does not compile as-is
with a modern clang/gcc), and builds it. Safe to re-run.

## Usage

```sh
python3 compute_scl.py
```

For each word in `relators.py` this runs `scallop <word>`, parses the printed
`scl( word ) = p/q = decimal` line, and checks it against the value claimed
in `paper/scl-counterexample.tex`. Exits non-zero if anything disagrees.

```
using scl/vendor/scallop/scallop

calibration: abAB                   scl =  1/2  (paper:  1/2)  [ok]  ...
calibration: [a1,b1][a2,b2]         scl =  3/2  (paper:  3/2)  [ok]  ...
calibration: Heuer-Löh Example 1.1  scl =  5/8  (paper:  5/8)  [ok]  ...
theorem: r  = aabABabABBAbaabABBAb  scl =    1  (paper:    1)  [ok]  ...
theorem: r' = aabABabABabABBAbaBAb  scl =  1/2  (paper:  1/2)  [ok]  ...
remark: aabaBAAbABabAB              scl =  3/4  (paper:  3/4)  [ok]  ...
remark: aabaBAAbaBAbABaabAAB        scl =  2/3  (paper:  2/3)  [ok]  ...

All scl values match the paper.
```

If `scallop` is installed somewhere else, or was built without setting an
absolute `install_name` for `libglpk`/`libgmp` and so cannot find them at
runtime, use:

```sh
python3 compute_scl.py --scallop /path/to/scallop --lib-dir /path/to/libs
```

(or set the `SCALLOP` environment variable). On the reference setup above
(Homebrew on macOS), `--lib-dir` is not needed: the built binary already
records absolute paths to `libglpk`/`libgmp`.

## Files

- `relators.py` — the words and their claimed scl values, each tagged with
  where in the paper the claim appears.
- `compute_scl.py` — runs `scallop` on each word and checks the result.
- `setup_scallop.sh` — fetches and builds `scallop`.
