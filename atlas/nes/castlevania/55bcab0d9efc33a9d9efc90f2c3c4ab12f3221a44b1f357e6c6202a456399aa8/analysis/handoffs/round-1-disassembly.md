# Round 1 disassembly handoff

- Scope: first disassembly pass on a fresh workspace (Castlevania US, mapper 2 UxROM, 128 KiB PRG,
  CHR-RAM). Created the verified baseline (generated/baseline, byte_identical true).
- Output checkpoint: **pass-001** (maps/pass-001.json, generated/pass-001,
  analysis/pass-001-verification.json): ok true, **byte_identical true**.
- Status: **incomplete** (8621 bytes unknown).

ROM classification: 93.42% (122467 / 131088 bytes classified; 8621 bytes unknown).

Kinds: Code 24148 (Observed 12483, Static 11665), Data 82066, Padding 16253; 646 regions.

| Range | Code | Data | Padding | Classified | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 16 | 0 |
| 8K bank 0 (16K 0) | 1909 | 6276 | 0 | 8185 | 7 |
| 8K bank 1 (16K 0) | 0 | 6913 | 1279 | 8192 | 0 |
| 8K bank 2 (16K 1) | 0 | 8192 | 0 | 8192 | 0 |
| 8K bank 3 (16K 1) | 0 | 7432 | 760 | 8192 | 0 |
| 8K bank 4 (16K 2) | 0 | 8192 | 0 | 8192 | 0 |
| 8K bank 5 (16K 2) | 0 | 7400 | 792 | 8192 | 0 |
| 8K bank 6 (16K 3) | 0 | 8192 | 0 | 8192 | 0 |
| 8K bank 7 (16K 3) | 0 | 7944 | 248 | 8192 | 0 |
| 8K bank 8 (16K 4) | 0 | 8192 | 0 | 8192 | 0 |
| 8K bank 9 (16K 4) | 0 | 5128 | 3064 | 8192 | 0 |
| 8K bank 10 (16K 5) | 773 | 66 | 441 | 1280 | 6912 |
| 8K bank 11 (16K 5) | 0 | 0 | 8192 | 8192 | 0 |
| 8K bank 12 (16K 6) | 6042 | 1456 | 0 | 7498 | 694 |
| 8K bank 13 (16K 6) | 4055 | 2640 | 1477 | 8172 | 20 |
| 8K bank 14 (16K 7) | 5139 | 2551 | 0 | 7690 | 502 |
| 8K bank 15 (16K 7) | 6230 | 1476 | 0 | 7706 | 486 |

## Findings

- Bank model: fixed 16K bank 7; BankSelectY $C195 (Y = 16K bank, bus-conflict table $C000);
  BankRestoreSaved $C193 maps bank $24, written only at Reset (=6). Gameplay logic in banks 6/7,
  sound driver + music data in bank 0, object/enemy handlers partly in bank 5 ($8000-$9FFF).
- 8K banks 1-9 contain no code (graphics/level/music data); 8K bank 11 is all $FF.
- JumpTableDispatch $CA1E (inline word table after JSR) drives game states ($18, GameStateTable
  $C1B1) and object handlers (ObjectHandlerTable $E938, index $4B); two RTS-trick return words
  ($E9A4, $F3F9) are pushed before dispatch.
- Method details, rejected methods (R1-R3) and rules: analysis/investigation.md.
- Execution coverage (separate metric): 6296 observed ROM instruction starts, 4526 from natural play.

## Leads

- Consumed: none (no prior leads).
- Produced: L-001 (bank-5 object handlers/islands, ~6.9 KB), L-002 (bank-6 gaps), L-003 (fixed-bank
  unreferenced fragments), L-004 (jump-table extents), L-005 (sound data format), L-006 (data-bank
  loader lists $FD10), L-007 (boss/ending RAM contract), L-008 (toolkit static discovery, blocked).
  See analysis/leads.json.

## Unresolved questions

- Bank mapped at the $E935 ObjectHandlerTable dispatch for every arrival (observed only bank 5; a
  static path reaches it with bank 6).
- Whether state $18=16 ($FAFE word) exists; position/checkpoint RAM for boss rooms.

## Deferred work

- Stages 15/17/18 warps (per-stage gains had dropped to ~10 instruction starts), boss fights,
  natural ending (SC-BOSS, SC-ENDING). Semantic decoding of data banks and music data.

## Scenarios visited

analysis/scenarios.json: SC-001 title/attract, SC-002 game start, SC-003 stage-1 play (natural);
stage warps 0,1,2,3,6,9,12 and game-state probes $18=7,8,12,14,15 (artificial; ending code ran with
garbled CHR). Snapshots: scratch/snap_title.json, snap_stage1.json, snap_nat1end.json (local).

## Toolkit issues

analysis/toolkit-issues.md: TK-NES-001 confirmed bug (emulator.input ignores button/pressed),
TK-NES-002 performance limitation (tracing ~10x slower), TK-NES-003 confirmed doc mismatch (Read
events include instruction fetches), TK-NES-004 missing feature (static discovery stops at every
JSR on UxROM), TK-NES-005 suspected bug (discover budget param name). No blockers.

## Artifacts

- Internal map analysis/state.json; export maps/current.json; checkpoint maps/pass-001.json.
- Local helpers (not deliverables): scratch/sa2.py (static closure), scratch/islands.py,
  scratch/gapdata.py, scratch/datareg.py, scratch/regions.py, scratch/annotate.py, scratch/covF.json.
