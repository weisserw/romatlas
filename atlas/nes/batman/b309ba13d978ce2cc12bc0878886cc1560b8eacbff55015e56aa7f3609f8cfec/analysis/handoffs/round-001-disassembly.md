# Round 1 — disassembly handoff

- Scope: first full disassembly pass (baseline, static discovery, broad gameplay scenarios, whole-ROM classification).
- Input checkpoint: none (baseline created this pass: maps/baseline.json, analysis/baseline-verification.json).
- Output checkpoint: **pass-001** — maps/pass-001.json, generated/pass-001,
  verification [analysis/pass-001-verification.json](../pass-001-verification.json): ok=true, byte_identical=true,
  map_sha256 f914cdaf82bb6283c917747e58bfe5673028883769a396c0b324542eb38508a5.
- Internal store: analysis/state.json (identical to maps/current.json; 1,765 regions, map.validate ok).

## Completion

ROM classification: 98.19% (257410 / 262160 bytes classified; 4750 bytes unknown).

Status: incomplete (4,750 unknown bytes remain).

| Segment | Code | Data | Text | Image | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 | 0 |
| PRG 0 | 2241 | 5448 | 0 | 0 | 503 |
| PRG 1 | 0 | 8192 | 0 | 0 | 0 |
| PRG 2 | 2702 | 3848 | 1120 | 0 | 522 |
| PRG 3 | 2999 | 4468 | 0 | 0 | 725 |
| PRG 4 | 7400 | 702 | 0 | 0 | 90 |
| PRG 5 | 1477 | 6152 | 144 | 0 | 419 |
| PRG 6 | 245 | 7193 | 0 | 0 | 754 |
| PRG 7 | 1400 | 6500 | 0 | 0 | 292 |
| PRG 8 | 2036 | 5779 | 68 | 0 | 309 |
| PRG 9 | 0 | 8192 | 0 | 0 | 0 |
| PRG 10 | 0 | 8192 | 0 | 0 | 0 |
| PRG 11 | 0 | 8192 | 0 | 0 | 0 |
| PRG 12 | 6945 | 889 | 0 | 0 | 358 |
| PRG 13 | 6563 | 1347 | 0 | 0 | 282 |
| PRG 14 | 3104 | 4783 | 0 | 0 | 305 |
| PRG 15 | 3495 | 4506 | 0 | 0 | 191 |
| CHR | 0 | 0 | 0 | 131072 | 0 |

Code 40,607 B (Observed-provenance code regions 25,860 B; Static 14,747 B incl. 366 B unreferenced routines).
Execution coverage (separate metric): 12,662 observed opcode offset/CPU pairs (analysis/observed/contexts.json).

## Findings
- Game: Sunsoft *Batman: The Video Game* (NES, NA). MMC3, PRG mode 0 throughout: $C000=bank 14, $E000=bank 15.
  Bank switch routine $C440 (A->R6 $8000, X->R7 $A000; values masked mod 16, e.g. $1C/$1D -> banks 12/13).
- Engine layout: bank 0 sound driver (+bank 1 music data), bank 4 area engine (+bank 10 data), banks 12/13 object
  engine (46-entry object-type jump table at bank 12 $80BA via jump engine $C588; handlers in banks 12, 13 and 15),
  banks 2+3/5 cutscene/ending engine, banks 8/9/11/6/7 per-stage area banks ($E0CC table indexed by $88>>1).
- Helpers: $C4F3 takes a 2-byte inline argument; $C588 (and bank-2 copy $8792) is an inline-table jump engine.
- RAM contract: $88 area*2, $CF area result (bit6 clear -> next area, bit7 life lost), $B7 health, $BE lives.
- DPCM samples at $F000 (209 B), $F100 (497 B), $F300 (497 B) from DMC records at bank 0 $89DD.
- Crash trail in artificial scenarios executed bank 8 $8001/$A020-$A03B; excluded from code evidence.

Methods/evidence detail: [analysis/investigation.md](../investigation.md), [analysis/progress.md](../progress.md).
Tools: analysis/tools/ (sa.py, build_map.py, classify_gaps.py, orphans.py, apply_map.py, scenario scripts).

## Leads
- Consumed: none (no prior leads).
- Produced: L-001..L-006 in [analysis/leads.json](../leads.json) (cutscene banks 2/3/5, stage-4/5 banks 6/7,
  bosses, continue screen, classifier boundary review, fixed-bank residual tables).

## Scenarios visited ([analysis/scenarios.json](../scenarios.json))
- S1 boot/license/title/intro/stage 1 (natural). S2 areas 0-20 via death reload (artificial; idx>=18 broken).
- S3 stage transitions to stages 2-5 and area 17 via $CF=$40 clear path (artificial entry, natural transition code).
- S4 ending cutscene + credits via $88=$22,$CF=$40 (artificial entry).
- Deferred: bosses (no position/scroll shortcut), game-over/continue screen.

## Unresolved questions
- Code vs data for the 4,750 unknown bytes (mostly clean-decoding stretches in cutscene banks 2/3/5, area banks 6/7,
  fixed banks 14/15 tables, bank 0 sound data near code).
- Whether the 9 unreferenced routines (orphans) are reachable (e.g. sound pause/resume $8641/$866D).
- Exact domains of the object-type and sound command tables (extent bounded structurally, not by selector proof).

## Deferred work
- Boss/continue scenarios (L-003, L-004); cutscene sweep (L-001); boundary review of auto data (L-005).
- Labels/commentary: only structural labels added (INesHeader, ChrRom, CpuVectors, DPCM, sound tables).

## Toolkit issues ([analysis/toolkit-issues.md](../toolkit-issues.md))
- TK-001 missing feature: NES static discovery stops at every JSR/indexed store on MMC3 (69 instructions from vectors).
- TK-002 suspected bug: NES trace reports operand fetches as Read, no Fetch events.
- TK-003 missing feature: tracing slows emulation ~13x (no coverage-only mode).
No blockers.

## Residual unknown stretches >= 16 bytes
- bank 0: file 0x00c64-0x00c76 (18 B)
- bank 0: file 0x00cac-0x00cd0 (36 B)
- bank 0: file 0x00ea2-0x00eec (74 B)
- bank 0: file 0x014d7-0x0150c (53 B)
- bank 0: file 0x015f4-0x01613 (31 B)
- bank 0: file 0x01a78-0x01a91 (25 B)
- bank 2: file 0x042bf-0x042d8 (25 B)
- bank 2: file 0x04522-0x04582 (96 B)
- bank 2: file 0x0462e-0x04656 (40 B)
- bank 2: file 0x04905-0x04915 (16 B)
- bank 2: file 0x055f0-0x05619 (41 B)
- bank 2: file 0x05820-0x0583b (27 B)
- bank 2: file 0x05b9f-0x05bc9 (42 B)
- bank 2: file 0x05d79-0x05d95 (28 B)
- bank 2: file 0x05e91-0x05ed7 (70 B)
- bank 3: file 0x06040-0x06057 (23 B)
- bank 3: file 0x060bb-0x060cc (17 B)
- bank 3: file 0x062b5-0x062cd (24 B)
- bank 3: file 0x064de-0x064f2 (20 B)
- bank 3: file 0x06533-0x06546 (19 B)
- bank 3: file 0x06557-0x065b1 (90 B)
- bank 3: file 0x065c1-0x065dc (27 B)
- bank 3: file 0x0679d-0x067bd (32 B)
- bank 3: file 0x06966-0x06986 (32 B)
- bank 3: file 0x069c1-0x06a27 (102 B)
- bank 3: file 0x06a54-0x06a7b (39 B)
- bank 3: file 0x06a9a-0x06aad (19 B)
- bank 3: file 0x074fb-0x0758b (144 B)
- bank 4: file 0x09fcc-0x0a010 (68 B)
- bank 5: file 0x0a0e1-0x0a0fe (29 B)
- bank 5: file 0x0a65f-0x0a6a2 (67 B)
- bank 5: file 0x0a7b9-0x0a7d6 (29 B)
- bank 5: file 0x0a855-0x0a87b (38 B)
- bank 5: file 0x0a920-0x0a93c (28 B)
- bank 5: file 0x0aa3f-0x0aa6e (47 B)
- bank 6: file 0x0c028-0x0c03b (19 B)
- bank 6: file 0x0c0a8-0x0c0bd (21 B)
- bank 6: file 0x0c0ff-0x0c119 (26 B)
- bank 6: file 0x0c52e-0x0c540 (18 B)
- bank 6: file 0x0c594-0x0c5b0 (28 B)
- bank 6: file 0x0c9e7-0x0c9fb (20 B)
- bank 6: file 0x0ca18-0x0ca2d (21 B)
- bank 6: file 0x0ce27-0x0ce4b (36 B)
- bank 6: file 0x0d1ba-0x0d1d0 (22 B)
- bank 6: file 0x0d1e8-0x0d1fa (18 B)
- bank 6: file 0x0d259-0x0d275 (28 B)
- bank 6: file 0x0d342-0x0d370 (46 B)
- bank 6: file 0x0d3c0-0x0d412 (82 B)
- bank 6: file 0x0d4a8-0x0d4bb (19 B)
- bank 6: file 0x0d551-0x0d577 (38 B)
- bank 6: file 0x0d659-0x0d66d (20 B)
- bank 6: file 0x0d79d-0x0d7b6 (25 B)
- bank 7: file 0x0e078-0x0e096 (30 B)
- bank 7: file 0x0e289-0x0e2a0 (23 B)
- bank 7: file 0x0e4dc-0x0e4f7 (27 B)
- bank 7: file 0x0e557-0x0e578 (33 B)
- bank 7: file 0x0e595-0x0e5ae (25 B)
- bank 7: file 0x0e7f5-0x0e826 (49 B)
- bank 7: file 0x0fd1e-0x0fd35 (23 B)
- bank 7: file 0x0fe11-0x0fe23 (18 B)
- bank 8: file 0x108fc-0x10910 (20 B)
- bank 8: file 0x1142b-0x11441 (22 B)
- bank 8: file 0x116d4-0x11710 (60 B)
- bank 8: file 0x11fcc-0x12010 (68 B)
- bank 12: file 0x18379-0x1839d (36 B)
- bank 12: file 0x18969-0x1897e (21 B)
- bank 12: file 0x18e8d-0x18eaa (29 B)
- bank 12: file 0x18ecf-0x18f65 (150 B)
- bank 12: file 0x18ffc-0x19015 (25 B)
- bank 12: file 0x19a48-0x19a6b (35 B)
- bank 13: file 0x1a010-0x1a03e (46 B)
- bank 13: file 0x1a067-0x1a07b (20 B)
- bank 13: file 0x1a985-0x1a9a4 (31 B)
- bank 13: file 0x1adc4-0x1ade4 (32 B)
- bank 13: file 0x1b3b3-0x1b3dd (42 B)
- bank 13: file 0x1bcc4-0x1bcd9 (21 B)
- bank 14: file 0x1c2a9-0x1c2bd (20 B)
- bank 14: file 0x1c512-0x1c527 (21 B)
- bank 14: file 0x1d4b6-0x1d4db (37 B)
- bank 14: file 0x1dbbb-0x1dbd4 (25 B)
- bank 15: file 0x1e287-0x1e2a4 (29 B)
- bank 15: file 0x1f5bc-0x1f606 (74 B)
- bank 15: file 0x1f6bc-0x1f6dc (32 B)
(plus shorter stretches; total 306 stretches / 4,750 bytes)
