# Round 3 disassembly handoff

- Status: COMPLETE (pass). ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown) — NOT complete.
- Input pass-002 (571 regions) -> output checkpoint pass-003 (599 regions, map sha256 9baf6bef...e17b6,
  byte-identical, analysis/pass-003-verification.json). No classification (kind/bytes) changes; provenance/boundaries only.
- Lead L2-02 verified; L2-01 untouched (terminal). No ready leads remain.
- Full per-piece list (ids, ranges, phys, provenance): analysis/evidence/round3/l2-02-changes.json.

## Provenance-only changes (Static -> Observed, same id and bounds; comments carry over unchanged)
id 1937 $F4FD (S13b); 1384 bank3 $BD1E-$BD2A; 1397 bank4 $97D3-$9840; 1597 $C753-$C75F;
1617 $CDB1-$CDB3; 1697 $DDDB-$DDDD; 1703 $DE05-$DE07 (S14; $BD1E also S13b).

## Boundary changes (old region split; first piece keeps old id; O=Observed, s=Static)
- 1395 bank4 $9137-$956C -> s$9137-$9438 (1395), O$9439-$956C (1938)
- 1400 bank4 $98FE-$9D33 -> s$98FE-$9BFF (1400), O$9C00-$9D33 (1939)
- 1497 bank5 $A4DA-$A519 -> O$A4DA-$A513 (1497), s$A514-$A519 (1940)
- 1621 $CE34-$CEB3 -> O$CE34-$CE7E, s$CE7F-$CEB0 (1941), O$CEB1-$CEB3 (1942)
- 1640 $D1EC-$D2D6 -> O$D1EC-$D1FB, s$D1FC-$D1FE, O$D1FF-$D203, s$D204-$D2AB, O$D2AC-$D2CE, s$D2CF-$D2D6 (ids 1640,1943-1947)
- 1680 $D8A3-$D900 -> O$D8A3-$D8CE (1680), s$D8CF-$D900 (1948)
- 1729 $E27C-$E4B2 -> 14 runs, ids 1729,1949-1961: O$E27C-$E285 s$E286-$E287 O$E288-$E29C s$E29D-$E2AD
  O$E2AE s$E2AF-$E2E1 O$E2E2-$E2FC s$E2FD-$E2FF O$E300-$E307 s$E308-$E31B O$E31C-$E32C s$E32D-$E32F
  O$E330-$E338 s$E339-$E4B2
- 1735 $E665-$E684 -> O$E665-$E66B (1735), s$E66C-$E684 (1962)
- 1739 $E6E2-$E70C -> s$E6E2-$E6E3 (1739), O$E6E4-$E6F8 (1963), s$E6F9-$E709 (1964), O$E70A-$E70C (1965)
Commentary: carry names/comments by physical identity; splits fall on instruction starts inside former
regions, so only region-level notes need redistribution. Evidence text states the artificial-scenario
caveat (observed execution, not natural reachability).

## Next
No classification proposal remains (2 bytes terminal per r2-a1 review). Proceed to commentary carry-over
for the split regions, then final editorial audit/acceptance.
