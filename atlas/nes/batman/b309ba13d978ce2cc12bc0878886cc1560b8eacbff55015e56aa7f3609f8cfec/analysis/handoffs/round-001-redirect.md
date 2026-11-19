# Round 1 — redirect handoff (attempt r1-a1)

**Recommendation: RECOMMEND NEXT CYCLE** — ready leads L-007..L-013, L-015 propose 3,372 of the
4,750 unknown bytes (plus a 142-byte data->code correction). L-014/L-016 (1,378 B residual) carry
concrete recipes for the next redirect.

## Scope and checkpoint
- Input checkpoint: **pass-001** (maps/pass-001.json == maps/current.json == analysis/state.json;
  analysis/pass-001-verification.json byte_identical; map_sha256 f914cdaf...a5). Classification
  unchanged by this phase (98.19%, 4,750 B unknown). No map/state edits were made.
- Research directory: `analysis/redirect/r1-a1/` (tools/, proposals.json, residual.txt,
  ledger-initial.txt, *_marked.json walk outputs, observed/contexts.json for this attempt's traces,
  experiment logs). Pass-001 evidence (analysis/observed/*) untouched.
- Binary: /root/romatlas/target/release/romatlas via `run.py serve` (relay analysis/tools/rpcd.py);
  server shut down at the end.

## Methods
1. Unknown ledger with neighbours (`tools/ledger.py`; initial copy `ledger-initial.txt`).
2. Consumer-anchored format walkers that establish whole-structure extents from the game's own
   reader code: sound driver (`sndwalk.py`), cutscene background streams (`csbg.py`), cutscene
   metasprites (`csspr.py`), object/boss metasprite tables (`msprite.py`), Batman pose lists
   (`batpose.py`). Every walk was checked for overlap with classified code (0 conflicts in all).
3. Bridge test for unknown stretches adjacent to code (`bridges.py`: exact instruction-boundary
   landing, terminator of previous region, control-flow refs), word-reference search (`findw.py`),
   indexed-table consumers (`consumers.py`), re-check of phase-1 "Observed reads" data regions
   (`obsreads.py`).
4. Gameplay (artificial, RAM patches, recipes `bossgo.py`, `bosskill.py`, `exp2.py`): boss areas via
   stage-start transition then `$88=2*(T-1), $CF=$40`; forced boss defeat by writing
   `ObjState|=$40, ObjVarE=0, ObjTimer=0` (the outcome CollideObjects produces); game over via
   `$BE=0,$B7=0`; weapons via `$A6`/`$B8`. Traced windows aggregated per tag in
   `analysis/redirect/r1-a1/observed/contexts.json`.
5. Combination/accounting: `tools/combine.py` -> `proposals.json` (per-group ranges, unknown
   bytes, conflicts) and `residual.txt`.

## Findings (proposal groups; bytes = currently-unknown bytes covered)
| Lead | Group | Bytes | Basis |
|---|---|---|---|
| L-007 | C-R06 bank 13 $BCB4-$BD56 code (file 0x1BCC4-0x1BD66) | 21 + 142 reclass | JMP $BCB4 from ObjKind42 ($E82C); one routine ending JMP $EA8E, reads tables $BD57/$BD5F that follow; phase-1 data claim rested only on 5 "reads" at 0x1BD16-0x1BD1A that are instruction bytes of this decode |
| L-008 | C-R01 $C502, C-K1X bank13 $A000-$A02D, C-JOY bank2 $9E81-$9EC6 (strong); C-B2/C-MISC orphans | 137 + 216 | same-contract entry falling into observed $C517; bank-12 code falling through into bank 13; $4016/$4017 joypad routine; orphan routines landing exactly on instruction boundaries |
| L-009 | D-SND / D-SNDX bank 0 sound data | 388 + 115 | all 503 bank-0 unknown bytes are headers/streams/envelopes of the bank-0 driver format (12,602 B walked, no undefined commands) |
| L-010 | D-CSBG / D-CSSPR cutscene streams + metasprites | 952 + 84 | gapless chained streams ending at code $AAE4 (bank 3) and at $9B82 (metasprites); covers all 573 bank-3 data/data gaps |
| L-011 | bank 14/15 tables (metasprites, maps, HUD, area-start records, DPCM pad, bank-15 sound block) | 424 | table counts 195/189 bounded by observed code at $D981/$DC42; 17 x 11-byte area-start records end at $E0CC |
| L-012 | R-02 33 consumer-read tables | 731 | indexed reads by classified code; decodes hit illegal opcodes/overrun |
| L-013 | bank-3 period-16 records, Batman pose lists, bank-4 tail duplicate of bank 12 | 236 | pattern/consumer; bank 4 $9FBC-$9FFF byte-identical to bank 12 code but non-functional in bank-4 context |
| L-015 | small tables/palette/credit text | 68 | see leads.json |

Total proposed: **3,372 B** of 4,750 (+142 B correction). Exact ranges per group: `analysis/redirect/r1-a1/proposals.json`.

Other findings:
- RAM/position contract for areas: `$DFED,x` (x=$88) points to 11-byte area-start records
  `$E011-$E0CB` loaded into $EA, $E0, $E1, $E2, ... (scroll/map cursor) at bank 14 $DFA7 —
  usable for future position shortcuts.
- Boss kinds per area: idx 2 kind 20 (Killer Moth, HP 16); idx 6 kinds 22-25; idx 10 kinds 26/27/28;
  idx 14 kinds 32/38/40; idx 16 boss visible but not in the active-slot mask in 400 frames.
- Unobserved object kinds after pass-001: 0-4, 6, 13-15, 18, 19, 28 (now observed via kill26), 29
  (now observed), 30, 34, 39, 41.
- Bank 8 $93E0 (`lda #$22 / sta $88`) is an alternate entry setting area 17 before observed code
  $93E4 (relevant to the area-16 skip path).

## Gameplay experiments (artificial, analysis/scenarios.json S5/S6 updated)
- Bosses (S5, partial): idx 2 kind 20 forced defeat -> explosion kind 29, BossDefeatFlag $CE=$80,
  area clear -> stage-2 cutscene; idx 10 kinds 26/27 -> clear -> stage 4; idx 14 kinds 32/38 -> clear
  -> stage 5; idx 16 kinds 33/35 (HP 96) behaviour traced, forced defeat sets $CE but no clear within
  900 frames; idx 6 kind-23 kill did not clear. **Kind 42 / idx 17 not reached**, so L-007 (R-06)
  remains static evidence only. Logs kill23.log, kill26_32.log, exp2_A.log, exp2_D.log.
- Game over / continue (S6): `$BE=0,$B7=0` -> GAME OVER / CONTINUE / END screen; Start continues
  (lives 2). New observed bank-14 code 0x1DE13-0x1DE43. exp2_B.log.
- Weapons (S7): `$A6`=$50/$C0/$60 with `$B8=30`, B presses -> weapon object kinds 1-3 executed
  (607 new opcode offsets, bank 12 $9BA9-$A004, bank 4 $9108-$9D5E). exp2_C.log.
- No byte of any data proposal was fetched as an instruction in these traces (counter-evidence check).
- New observed code from these traces is recorded per tag in
  analysis/redirect/r1-a1/observed/contexts.json for the disassembly pass to merge (it is
  artificial-scenario execution evidence).

## Rejected / negative results
- L-001 premise (unreached cutscene scenes): all scene vectors already observed; bank 3/5 residual
  is background data (L-010). Marked superseded.
- First sound walk over-covered by falling through count-0 (endless) loops; corrected rule; bytes
  only reachable after such loops are separated as D-SNDX (pattern-consistent, entry unknown).
- "jsr $BD05" hits from a raw byte scan were misaligned operand bytes, not calls.
- Kill of kind 23 (idx 6) did not clear the area (Batman died meanwhile); kind 22-25 core not identified.
- Bank 15 $F4F1-$F7FF sound block: no header/stream pointer from banks 0/1 reaches it (word search);
  classification rests on syntax consistency only (medium).

## Deferrals (with reasons)
- **L-014 area data banks 6/7/8 (1,231 B)**: needs a walker of the bank-4 area format
  (per-area tables Dat_14_C404 metatile base / Dat_14_C422 block base, map cells via AreaMapPtr,
  64-byte column-major blocks, 8-byte metatile records). Not attempted for time; no missing
  prerequisite. Frontier: commentary/source/bank04.asm QueueMapColumn/LookupMapCell (lines ~100-340)
  and lines 2107-2200; validate on all-data banks 9/11 first.
- **L-016 (147 B)**: CsSceneTable record count, $8FC0 animation/text script format, bank-5 ending
  tile lists. Low priority.
- $C59E (2 B) ambiguous (`lda $F5` immediately before HideUnusedSprites); left unknown.
- Natural reachability of orphan routines (C-B2/C-MISC) not established; structural proposal only.

## Toolkit
No new toolkit issues. Emulation with tracing was very slow under machine load (TK-003 applies).
