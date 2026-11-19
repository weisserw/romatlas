# Round 1 — disassembly phase handoff (2026-11-19)

## Scope
First pass on Bases Loaded (NES, MMC1, 256 KiB PRG / 64 KiB CHR). Created the verified baseline,
established the bank architecture, classified code (observed + bank-aware static), structural
data (header, CHR, vectors, stubs, DPCM samples, jump tables), $FF padding and heuristic generic
data. No input leads.

## Checkpoint
- pass-001: maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json —
  verification ok, byte_identical true. Baseline: maps/baseline.json (byte_identical true).
- Internal store: analysis/state.json (1,280 regions); export maps/current.json.

ROM classification: 95.56% (313139 / 327696 bytes classified; 14557 bytes unknown).

Result: incomplete.

## Per-bank summary (classified / unknown bytes)
Full table with kinds in analysis/progress.md. Unknowns by 8 KiB PRG bank: 0: 21, 1: 1414,
2: 407, 3: 154, 4: 200, 6: 10, 7: 1617, 8: 2412, 9: 448, 11: 3, 12: 2426, 13: 2497, 14: 961,
15: 15, 16/20/24/28: 10 each, 17/21/25/29: 483 each. Banks 5, 10, 18, 19, 22, 23, 26, 27, 30, 31,
the header and all CHR have no unknown bytes.

## Findings
- 32 KiB PRG mode (reset stub in every 16 KiB bank: PRG=0, control=$02); context per 32 KiB bank n
  is [4n..4n+3]. Common $8000-$8353 module per bank (dead relocated copies in banks 3-7).
- PRG switch helpers $81AF/$8242/$817D; NMI $8354 per bank with $F1-indexed dispatch tables.
- Banks 4-7: shared lower 16 KiB engine; upper halves hold 14 DPCM samples of exactly 3841 bytes
  at $1000-aligned slots (player routine $BD00, $4013=$F0).
- Inline dispatcher JSR $DB7E (bank 1); overlapping-instruction skip trick at bank 0 $9E25/$9E3D.
- Details/evidence: analysis/investigation.md. Toolkit: analysis/toolkit-issues.md (TK-001 input
  schema bug, TK-002 MMC1 discovery limitation, TK-003 Read-trace/overhead) — workarounds used,
  nothing blocking.

## Gameplay scenarios reached
Title, pennant password screen and editing, team select, pitcher select, pitching, fielding
(outfield/infield), batting/half-inning changes, pinch-hitter menu, VS-mode team select
(analysis/scenarios.json S01-S09; snapshots scratch/snapshots/, screenshots scratch/shots/).
Not reached: end of game, standings/pennant progression, ending (S10/S11 deferred: RAM contract
for innings/score and the password format are unknown). Gains when stopping: the last traced
scenarios (VS mode, password entry, ~30k more frames of play) added 751 instruction starts
(+5%), i.e. diminishing; substantial new code likely needs end-of-game/progression states via RAM.

## Leads produced
L-001 (code islands behind unresolved indirect jumps), L-002 (data gaps with spurious islands),
L-003 (semantic roles of generic data), L-004 (end-of-game / pennant / ending via RAM),
L-005 (JSR $DB7E table extents), L-006 (bank 31 signature bytes), L-007 (dead module copies),
L-008 (DPCM sample selection). See analysis/leads.json.

## Unresolved questions
- Extents of non-adjacent, unmasked jump tables ($84F2, $87C1, $B199, $F43A; bank 3 $83E8/$8710/$9017).
- Whether 3-byte gaps between code are unreached instructions or data.
- Semantic roles of ~75 KB generic data (only non-code is established).

## Deferred work / how to resume
- Tools: scratch/rebuild.sh regenerates the whole map from scratch/obs.json (observed contexts) via
  run_static.py -> annotate_code.py -> tables_ann.py -> structural.py -> dpcm.py -> ffpad.py ->
  gapdata.py (needs a running bridge: `python3 scratch/bridge.py &`, socket /tmp/bl-rpc.sock).
  To add coverage: start the server, load a snapshot, enable trace (capacity 0), play, then
  `python3 scratch/dumpobs.py` (unions the session's coverage into scratch/obs.json), then rebuild.sh.
- No servers or emulator processes are left running.
