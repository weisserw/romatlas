# Round 1 disassembly handoff

Scope: first disassembly pass on a fresh workspace (baseline created, one pass, no
pre-existing leads). Status: pass completed; ROM classification incomplete.

- Checkpoint: `pass-001` — analysis/pass-001-verification.json: ok=true, byte_identical=true
  (maps/pass-001.json, generated/pass-001). State: analysis/state.json; export maps/current.json.
- ROM classification: 99.69% (261346 / 262160 bytes classified; 814 bytes unknown).
- Per bank: PRG 0-4, 6, 7, 8, 10, 12, 13 and CHR fully classified; unknown: PRG05 17,
  PRG09 13, PRG11 17, PRG14 12, PRG15 755. Code 40561 B (21041 observed-only regions),
  data 89713 B, CHR image 131072 B. Table: analysis/progress.md "Round 1 disassembly pass".

## Findings
- Game: Kemco Bugs Bunny ("Happy Birthday Bugs"), MMC3 PRG mode 0; $8000/$A000 always a
  bank pair (2p,2p+1) via $E964; banks 14/15 fixed.
- Code reached by: vectors; bank-call dispatchers $F780-$F7C0 (A=pair<<4|index) with
  tables $8000/$8020/$BE00/$A980/$9CF0; inline jump tables after JSR $ECC7/$FEBF; keyed
  table $B1BB (bank 5); inline-parameter calls $E9F7 (8 bytes) and $EAB6 (RLE VRAM stream,
  no return). All observed fetches lie inside the static set; no invalid opcodes on paths.
- RAM contract for progression: $0420 stage, $0421 round, $043F round state (8 = goal ->
  clear), $0429 lives. Recipes in analysis/gameplay.md and analysis/scenarios.json.
- Visited: title/intro, all six stages, bonus board, ending + The End (artificial shortcuts).

## Unresolved / deferred
- L-001 orphan code-like routines in banks 14/15 (746 B; $F411 block 456 B, $EA73/$EB56
  stream-decoder variant, $E938, $FFDE...): need reference or contract evidence.
- L-002 six small gaps (47 B); L-003 stale duplicate $E8AB (21 B).
- L-004 bonus game (carrot RAM unknown), L-005 bosses not visited, L-006 semantics of
  generic data banks (level/text/metatile tables) for commentary.
- Generic data classification relies on pattern/reference criteria documented in
  analysis/investigation.md; meanings mostly unresolved (not claimed unused).

## Artifacts
- analysis/progress.md, analysis/investigation.md, analysis/leads.json,
  analysis/scenarios.json, analysis/gameplay.md, analysis/toolkit-issues.md
- analysis/evidence/round1-observed-coverage.json, round1-unknown-ranges.json,
  round1-progression.log, screens/, round1-tools/ (analysis scripts; paths assume /tmp/ra)
- Toolkit: TK-001, TK-003 confirmed bugs; TK-002, TK-004 missing features; no blockers.
