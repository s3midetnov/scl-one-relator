# Build the Rust tools (no crates needed; rustc alone suffices).
RUSTC ?= rustc
FLAGS ?= -O
BINS = wh wcanon hom cert gen

all: $(BINS)

wh: wh.rs
	$(RUSTC) $(FLAGS) -o $@ $<
wcanon: wcanon.rs
	$(RUSTC) $(FLAGS) -o $@ $<
hom: hom.rs
	$(RUSTC) $(FLAGS) -o $@ $<
cert: cert.rs
	$(RUSTC) $(FLAGS) -o $@ $<
gen: gen.rs
	$(RUSTC) $(FLAGS) -o $@ $<

groups: mkgroups.py
	python3 mkgroups.py

# quick self-test: agreement with the independent brute force at small length
check: wh
	@for L in 4 6 8 10 12; do ./wh $$L data/orbits_$$L.txt 2>&1 | grep orbits; done
	@python3 verify.py

clean:
	rm -f $(BINS) *.o
.PHONY: all clean check groups
