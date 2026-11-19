# Round 2 disassembly handoff

- Input checkpoint pass-001; output checkpoint **pass-002** (maps/pass-002.json, generated/pass-002,
  analysis/pass-002-verification.json): ok, byte_identical. Store analysis/state.json; maps/current.json == pass-002.
- ROM classification: 100.00% (393223 / 393232 bytes classified; 9 bytes unknown). INCOMPLETE (9 B).
- Code 61,659 B (21,584 Observed, 40,075 Static); Data 69,420 B; Image 262,144 B; 1902 regions.

## What was done
- Applied L010 first, then reran the r01-a bank-aware descent (batch ALL) on the refreshed map: 25.6 KB code + tables.
- Extended descent (analysis/tools-round2/desc2.py) to JSR fall-through after dispatcher callees / $D156: +400 B.
- Data leads L003/L004/L013-L016 applied (banks 0, 12, 13 now fully classified).
- Found two Round-1 truncated JumpEngine tables (bank 7 $B649, $B763) and extended them (tblext.py); handler $B70D.
- Residual-hole pass: consumer-read data, coherent unreferenced routines (Static 0.7), non-instruction holes.
- Verification: 0 observed fetches (Round 1, r01-a, Round 2 probes) outside Code instruction starts.
- Gameplay: final boss ($3E=$15) reached; forced defeat flags -> StageState 4 -> GameMode 7, $B7B9 fetched (artificial).

## Remaining / next
- 9 unknown bytes (L017): RTS at phys 0x08743, 0x0B1BE, 0x0B7A2, 0x14CCA, 0x17BC6, 0x1FD5A; JMP $FBCC at 0x1C1E2-0x1C1E4.
  Need a reference/fetch or a supported convention; do not classify on decoding alone.
- L006 natural ending: find player X / attack hitbox RAM so player attacks hit the stage-$15 boss (ObjType $34, slot 0,
  EnemyHitPoints[0] $0145); snapshot traces/round2/final_boss.snap (same build).
- L018: reachability of unreferenced Round-2 routines (batch4-regions.json Code entries).
- Commentary: new regions (~780) need names/comments; commentary/ not edited in this phase.

## Files
- Tools/region lists: analysis/tools-round2/ (rt.py, attrib.py, build_code.py, desc2.py, batch2/3.py, build2/3/4.py,
  build_data.py, gapscan.py, cat_gaps.py, coherent.py, tblext.py, lowwin2.py, obscheck.py, covcheck.py, probe_final*.py).
- Evidence: analysis/evidence-round2/; leads analysis/leads.json; scenarios analysis/scenarios.json.
- Toolkit: analysis/toolkit-issues.md (TK-005 new, minor). Scratch /root/rascratch-r02 (disposable).
