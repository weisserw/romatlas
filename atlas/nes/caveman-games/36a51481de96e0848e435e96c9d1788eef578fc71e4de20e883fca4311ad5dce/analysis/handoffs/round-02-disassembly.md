# Round 2 — disassembly handoff

- Input: pass-001b. Output checkpoint: **pass-002**.
  - Map: maps/pass-002.json, identical to maps/current.json.
  - Generated source: generated/pass-002.
  - Verification: analysis/pass-002-verification.json (ok, byte_identical true).
  - Store: analysis/state.json.
  - input/game.nes unchanged.

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

## Applied (all ready leads accepted after independent re-check; none rejected)
Script: tools/r02_apply.py. Evidence is in each region's evidence text and in analysis/leads.json.

| Lead | Bank and range | Change |
|---|---|---|
| L-001 | bank 3, file 0xE61C-0xEF08 ($A60C-$AEF8) | One Data region, DR_StaleForeignImage, conf 0.85. It is a stale, corrupted foreign 6502 image (+$1F21 relocation) with no references and cannot run here. |
| L-010 | bank 5, $9570-$9581 and $9582-$9587 | Data: the CL_Bracket6 pair table (18 B) and the 6-player placement list (6 B). |
| L-008 | bank 2, $95E6-$95E7 and $95E8-$9613 | $95E6-$95E7 Data (marker X offsets). $95E8-$9613 Static Code: a PpuBuf meter-row routine entered with a stack argument (PHA; JMP), unreferenced. |
| L-009 | bank 3, $9179-$917E | Static Code: preset-X entry that falls into $917F. |
| L-007 | fixed bank | 111 B Data->Code: IRQ split tail $C036, handlers $C080-$C0BD, $F26A QueueSfx copy, $C62F, $C61A, $C5F1. Words $C072-$C07F stay Data. $CEDB-$CEE7 stays Data. |
| L-011 | bank 5 $8CBD (11 B), bank 6 $AE0C (10 B) | Data->Code, Static, conf 0.7. |
| L-012 | Static Code regions with observed fetches | Provenance upgraded to Observed: 50 regions whole, 17 split; 1264 B. Observed code is now 32245 B. |

Static code is classified on structural evidence. Its reachability is recorded separately: unreferenced, or
static-only for the IRQ path. None of it is claimed dead.

## For commentary (changed ranges; commentary/ was not edited)
Carry the source forward from pass-002. Changed ranges:
- bank 2: $95E6-$9613
- bank 3: $9179-$917E and $A60C-$AEF8. The L-001 remark can document the decoded foreign routines.
- bank 5: $8CBD and $9570-$9587
- bank 6: $AE0C
- bank 7: $C036-$C047, $C072-$C0BD, $C5F1, $C61A, $C62F, $F26A

Some region boundaries moved because of the L-012 splits. Labels and comments live in the commentary notes, not
in the map.

## Open (no classification effect)
- L-004: 11 inferred roots, 717 B, Static conf 0.8. Reachability only.
- S15: 2-player mode is deferred.
- L-005: specific roles inside generic data are deferred.
- The ROM is not complete overall until full current-source commentary of the changed ranges and the final editorial audit are done.

Toolkit: no new issues found (analysis/toolkit-issues.md). No processes left running.
