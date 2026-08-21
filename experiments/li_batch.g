F:=FreeGroup("a","b");; a:=F.1;; b:=F.2;;
w2g:=function(s) local r,c; r:=One(F);
  for c in s do
    if c='a' then r:=r*a; elif c='A' then r:=r*a^-1;
    elif c='b' then r:=r*b; else r:=r*b^-1; fi; od; return r; end;;
MX := 5;
inp := InputTextFile(ARG_FILE);;
out := OutputTextFile(OUT_FILE, false);; SetPrintFormattingStatus(out, false);
line := ReadLine(inp);
while line <> fail do
  line := Chomp(line);
  if Length(line) > 0 then
    G := F/[w2g(line)];
    L := LowIndexSubgroupsFpGroup(G, MX);
    res := List(L, H -> [IndexInWholeGroup(H), AbelianInvariants(H)]);
    Sort(res);
    WriteAll(out, Concatenation(line, " ", String(res), "\n"));
  fi;
  line := ReadLine(inp);
od;
CloseStream(out); CloseStream(inp);
QUIT;
