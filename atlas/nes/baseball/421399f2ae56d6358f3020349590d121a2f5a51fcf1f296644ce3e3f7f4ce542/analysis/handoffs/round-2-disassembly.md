# Round 2 — disassembly handoff (2026-11-18)

Status: INCOMPLETE — 1 unknown byte remains ($E5F4, L-009). Easy gains and ready leads exhausted.

## Checkpoint and verification
- Checkpoint pass-002: maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json
  (ok true, byte_identical true, map sha256 9728112890a1cbfc86bd35b78461aa61daa6f023e13bd14e069188c183e02ef9).
- analysis/state.json saved, maps/current.json exported (== pass-002), map.validate ok. ROM unchanged.
- ROM classification: 100.00% (24591 / 24592 bytes classified; 1 bytes unknown). (exact 99.996%)
- Per bank: PRG bank0 $C000-$DFFF 8192/0 unknown; PRG bank1 $E000-$FFFF 8191/1 unknown; header, CHR complete.

## Lead outcomes
| Lead | Outcome |
|---|---|
| L-008 | verified; region split + EA05 added to traversal inline contracts (analysis/tools-r1, scratch inline.py/trav.py) |
| L-001..L-005 | verified, applied as proposed (L-002 tables and L-003 tables annotated as separate blocks) |
| L-009 | not reopened (no new evidence); remains Unknown |
| L-010 | not touched — for commentary |

All new regions: Static, confidence 0.9, prg_banks [0,1,0,1]; evidence says
"reachability unknown (not claimed unused)". Script: analysis/tools-r2/apply_leads.py.

## Changed physical ranges (commentary must carry names/comments forward)
Region split (previously one Code region 0x2AEE+44, id 696):
- 0x2AEE-0x2B07 Code ($EADE-$EAF7, id 696 shrunk; old metadata replaced)
- 0x2B08-0x2B0E Data inline args of WritePpuInline ($EAF8-$EAFE, count 6 + "CALLED" tiles)
- 0x2B0F-0x2B19 Code ($EAFF RTS, then $EB00-$EB09 routine)

Former Unknown, now classified (Code = C, Data = D):
- 0x0969-0x096B C, 0x096C-0x0971 D (C963 inline args), 0x0972 C  ($C959-$C962)
- 0x1820-0x1846 C ($D810); 0x1847-0x1852 D, 0x1853-0x185E D, 0x185F-0x186A D ($D837/$D843/$D84F)
- 0x1C06-0x1C0D D, 0x1C0E-0x1C15 D ($DBF6 RunFrameFirst / $DBFE RunFrameLast);
  0x1C16-0x1C38 C ($DC06); 0x1C39-0x1C40 D ($DC29)
- 0x1CA1-0x1CBB C ($DC91), 0x1CBC-0x1CBE D (E29B inline args -> $DC97), 0x1CBF-0x1CC9 C ($DCAF)
- 0x1FFB-0x1FFE D ($DFEB FielderReadyFrames), 0x1FFF-0x202C C ($DFEF), 0x202D-0x2046 D ($E01D)
- 0x22B5-0x22E2 C ($E2A5); 0x24C8-0x24D1 C ($E4B8); 0x2751-0x2755 C ($E741);
  0x27CC-0x27DA C ($E7BC); 0x2832-0x2837 C ($E822); 0x28C7-0x28E9 C ($E8B7);
  0x29D2-0x29E1 C ($E9C2); 0x29E2-0x29EA C ($E9D2); 0x2A0C-0x2A11 C ($E9FC); 0x2A12-0x2A14 C ($EA02);
  0x2B4C-0x2B50 C ($EB3C); 0x2BC4-0x2BCB C ($EBB4); 0x3ABE-0x3ACE C ($FAAE)

Commentary: these replace the 17 Unknown blocks in commentary/source; reuse the
round-1 commentary names (e.g. PrintCalled, RunFrameFirst/Last, FielderReadyFrames,
SetBallPositionInline) and apply L-010 corrections. Reachability wording: unknown, not dead.

## Gameplay
None this pass: remaining unknown byte cannot gain evidence from play (no ROM-encoded
reference can exist). scenarios.json unchanged; L-006 (other teams/substitution menus)
remains deferred and does not affect classification.

## Remaining / next
- 0x2604 ($E5F4): lone $60 between DrawTitleCursor RTS and Random; L-009 rejection review
  stands. Reopen only with a new reference/consumer or padding convention.
- Next: commentary (L-010 + new regions), then short redirect to confirm the residual.
- Toolkit: no new toolkit issues found (TK-001, TK-002 in analysis/toolkit-issues.md).
