#!/usr/bin/env python3
import os
BASE = os.environ.get("OM_DATA",
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "data"))
os.makedirs(BASE, exist_ok=True)
"""Run GAP low-index-subgroup homology in fresh chunks (GAP leaks memory across
thousands of fp-groups and gets OOM-killed). Resumable; skips chunks that fail."""

import subprocess, os, sys, time

WORDS = os.path.join(BASE,"surv_words.txt")
OUT   = os.path.join(BASE,"li.out")
MX    = int(sys.argv[1]) if len(sys.argv) > 1 else 5
CH    = 1000

GTPL = '''
F:=FreeGroup("a","b");; a:=F.1;; b:=F.2;;
w2g:=function(s) local r,c; r:=One(F);
  for c in s do if c='a' then r:=r*a; elif c='A' then r:=r*a^-1;
    elif c='b' then r:=r*b; else r:=r*b^-1; fi; od; return r; end;;
inp := InputTextFile("%s");;
out := OutputTextFile("%s", false);; SetPrintFormattingStatus(out, false);
line := ReadLine(inp);
while line <> fail do
  line := Chomp(line);
  if Length(line) > 0 then
    G := F/[w2g(line)];
    L := LowIndexSubgroupsFpGroup(G, %d);
    res := List(L, H -> [IndexInWholeGroup(H), AbelianInvariants(H)]);
    Sort(res);
    WriteAll(out, Concatenation(line, " ", String(res), "\\n"));
  fi;
  line := ReadLine(inp);
od;
CloseStream(out); CloseStream(inp); QUIT;
'''

words = [l.strip() for l in open(WORDS) if l.strip()]
done = set()
if os.path.exists(OUT):
    for l in open(OUT):
        done.add(l.split(' ', 1)[0])
todo = [w for w in words if w not in done]
print(f"total {len(words)}, already done {len(done)}, todo {len(todo)}", flush=True)

fout = open(OUT, "a")
t0 = time.time()
failed = 0
for i in range(0, len(todo), CH):
    chunk = todo[i:i+CH]
    open("/tmp/ch.txt", "w").write("\n".join(chunk) + "\n")
    open("/tmp/ch.g", "w").write(GTPL % ("/tmp/ch.txt", "/tmp/ch.out", MX))
    if os.path.exists("/tmp/ch.out"): os.remove("/tmp/ch.out")
    try:
        subprocess.run(["/usr/bin/gap", "-q", "-b", "-A", "-o", "1200m", "/tmp/ch.g"],
                       timeout=600, capture_output=True)
    except subprocess.TimeoutExpired:
        pass
    got = 0
    if os.path.exists("/tmp/ch.out"):
        for l in open("/tmp/ch.out"):
            fout.write(l); got += 1
    fout.flush()
    failed += len(chunk) - got
    el = time.time() - t0
    print(f"chunk {i//CH+1}/{(len(todo)+CH-1)//CH}: {got}/{len(chunk)}  "
          f"elapsed {el:.0f}s  unresolved so far {failed}", flush=True)
fout.close()
print("DONE. unresolved:", failed)
