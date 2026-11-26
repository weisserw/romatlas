# Round 4 - disassembly handoff (checkpoint pass-004)

Status: **complete** (metadata-only pass). Ready lead L-011 is verified and applied. No bytes or kinds changed.

## Scope
- I restored analysis/state.json from pass-003 (byte_identical) and kept every existing classification.
- **L-011**, applied by analysis/tools/annotate_r4.py, which re-asserts each fact from the ROM and the saved r3-p1
  traces before it edits:
  - refresh the evidence for regions 1016, 1064 and 998;
  - split region 1016 at phys $1CDD3, keeping Data on both sides.
- **Bounded follow-up scan for split-immediate addresses** (analysis/tools/splitimm_r4.py ->
  analysis/round-4/splitimm.json).
  - It covered every region whose evidence says "no reference / no caller / unreferenced", plus 998, 1016 and 1064:
    33 regions, 28 of them Code.
  - Patterns searched: stored `ld? #lo; st? M ... ld? #hi; st? M+1` pairs and loose #imm pairs. For Code regions the
    targets were the region start and start-1. Hits were filtered for bank relevance.
- There was no gameplay work. Every byte is already classified, and the only leads are metadata and commentary.
- Server: analysis/tools/rpcd.py with ROMATLAS_SCRATCH=/tmp/ra-2707-r4. Only its own PIDs were killed, and the
  scratch directory was removed.

## Checkpoint and verification
- map.validate: ok. map.save went to analysis/state.json, and map.export to maps/current.json, which is identical to
  maps/pass-004.json.
- `python3 run.py checkpoint pass-004`: ok, byte_identical (analysis/pass-004-verification.json, map_sha256
  35ed6454...ae26).
- input/game.nes is unchanged (sha256 2707303d...1c3e).

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Bank | Code | Data | Padding | Unknown |
|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 |
| PRG 0 | 0 | 16165 | 219 | 0 |
| PRG 1 | 0 | 16384 | 0 | 0 |
| PRG 2 | 3495 | 12755 | 134 | 0 |
| PRG 3 | 15023 | 1361 | 0 | 0 |
| PRG 4 | 0 | 16369 | 15 | 0 |
| PRG 5 | 13298 | 3045 | 41 | 0 |
| PRG 6 | 2309 | 13705 | 370 | 0 |
| PRG 7 | 11619 | 4676 | 89 | 0 |

## Changed regions
| id | Phys offset | CPU (bank) | Length | Change |
|---|---|---|---|---|
| 1016 | $1C87D | $C86D-$CDC2 (7) | 2135 -> 1366 | **Split.** Data, Static 0.9. Evidence rewritten: font/char tables. The NameCharMap base $CD43 now comes from the WindowFrameTable record-0 word and from split #imm stores at bank 2 $B23B/$B255 and bank 3 $8A5D/$A1A1. The tail $CDAE-$CDC2 is NameCharMap codes $6B-$7F; this is a pattern-only reading and was never observed read. |
| **1131 (new)** | $1CDD3 | $CDC3-$D0C3 (7) | 769 | Data, Static 0.95. SET POWER window CHR tile set: WindowTileSets record 3 (4-byte records at bank-3 $801C) = ($CDC3, bank 7). Its extent is fixed three ways: count 48 -> 1+48*16 bytes, it ends at the PlayMatch code at $D0C4, and a natural trace read all 769 bytes. |
| 1064 | $CA86 | $8A76-$8AFF (3) | 138 | Evidence rewritten as the SET POWER window rectangle: PowerMeterPanel split #imm WinParams stores at $8B21/$8B30, DrawWindowRect header (16, 8), attributes at $8AF8, and a natural read of all 138 bytes. The "no reference found" claim is removed; confidence 0.9 -> 0.95. |
| 998 | $D053 | $9043-$90D2 (3) | 144 | Evidence appended: the region starts with the SET SPIN rectangle (SpinPanel $8E44/$8E4E, header (10, 8), attributes at $9095). Static only. |
| 991 | $10 | $8000-$BF24 (0) | 16165 | "no code reference enters it" narrowed to "no code flow or direct operand". Fixed-bank code stores 44 split-#imm $8000-$BFFF pairs, but their bank is unresolved, so none is attributed to bank 0. The existing S6 runtime reads are cited. |

On every changed region, kind (Data) and provenance (Static) are unchanged. The region count goes from 1129 to 1130.

## Split-immediate scan outcome
- The 28 low-confidence/dead Code regions (L-006 family, 1081, 1129, 1120-1122, 974) have no stored or loose hit at
  start or start-1. This agrees with the r2-q1 immpair check, and their reachability wording stands.
- Region 992 (padding): no hit.
- Data regions with hits: 1064 and 998 are the WinParams rectangles above. 1016 has the NameCharMap and AsciiTileTable
  bases. For 991 the hits are bank-ambiguous; see above.

## Leads
- L-011: **verified** (history recorded in analysis/leads.json; pass entry added).
- L-012 (commentary) is still ready. Note for it: WindowTileSets starts at bank-3 $801C (phys $C02C) and has seven
  4-byte records. The bank-3 header comment says 3-byte records.

## Toolkit
TK-004 was added to analysis/toolkit-issues.md. It is a minor documentation mismatch: docs/protocol.md describes a
flat map.annotate form, but the NES map requires `{region:{...}}`, and id 0 creates a new region. It is not
blocking. TK-001 to TK-003 are unchanged.

## Recommendation
No unknown bytes remain, and no code/data conflict was found. Proceed to final acceptance and apply L-012 in
commentary. Report status: the classification contract is met at 100.00%. Remaining open items are meaning only:
whether NameCharMap codes $6B-$7F are ever printed, and the natural reachability of the win/loss screens.
