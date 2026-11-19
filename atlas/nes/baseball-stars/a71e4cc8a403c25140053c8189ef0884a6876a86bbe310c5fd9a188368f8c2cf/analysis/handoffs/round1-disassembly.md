# Round 1 — disassembly phase handoff

Game: Baseball Stars (SNK 1989), MMC1, PRG 128 KiB, CHR 128 KiB, battery. Status: **incomplete**
(easy gains taken; unknown bytes remain; no commentary yet).

## Scope
- Recovered from the orphaned previous worker: its serve process held 0 regions (map.get total 0);
  nothing to save; shut down with `__quit__`. Baseline untouched (not re-initialized).
- One disassembly pass: runtime coverage from scenarios S01-S11 (analysis/scenarios.json), bank-aware
  static descent from vectors + observed instruction starts, inline-argument/jump-table handling,
  generic-data classification of non-code gaps, CHR/header/vector structure.

## Checkpoint and verification
- Final: `maps/pass-003.json`, `generated/pass-003`, `analysis/pass-003-verification.json`
  (ok, byte_identical=true, map sha256 0408226ee1d1817111ec4762a98547c5fca2363ff10ebbc4a9b36674037134fe).
- Internal store `analysis/state.json` and `maps/current.json` identical to pass-003 content; map.validate ok.
- Earlier interim checkpoints pass-001 (contains a since-corrected bank-pair error) and pass-002 kept as history.

## Result
ROM classification: 96.38% (252677 / 262160 bytes classified; 9483 bytes unknown).
Per-bank table: analysis/progress.md (final entry). CHR 131,072 B Image; header 16 B; PRG code 54,599 B,
PRG data 67,006 B (generic 65.5k; inline args; $C178 jump tables; $FF80-$FFF9 tables; vectors).
Execution coverage (separate metric): 19,663 observed instruction starts.

## Key findings (details in analysis/investigation.md)
- MMC1 16K mode: PRG value n -> 8 KiB banks 2n/2n+1 at $8000; $C000-$FFFF fixed banks 14/15.
- Inline/stack-return helpers: $C53A, $C560 (2-byte pointers), $C791->$CBB7 (2-byte), $C0EE (PPU stream,
  $FE/$FF), $C178 (jump table after JSR), far-call $C18F (A = PRG value, Y = index into $FF80 table);
  far targets $C90A/$C87E consume 6 inline bytes.
- Generic-data classifier validated: 0 of 54.6k known code bytes misclassified; holdout coverage subsets
  contradict 0 data bytes (after two documented refinements; one 18-byte pass-002 error fixed in pass-003).
- Menu availability for TRADE/HIRE/FIRE PLAYER, REMOVE TEAM computed at bank 3 $AE81-$AF56 from WRAM
  $6790-$6795.

## Per-bank unknown (bytes)
PRG00 24, PRG01 102, PRG02 1891, PRG03 1855, PRG04 0, PRG05 2, PRG06 0, PRG07 2579, PRG08 0,
PRG09 67, PRG10 84, PRG11 622, PRG12 265, PRG13 86, PRG14 144, PRG15 1762; header/CHR 0.

## Leads produced (analysis/leads.json)
L-R1-01 conditional menus (WRAM-gated; biggest expected gain, banks 2/7/15) · L-R1-02 unbounded $C178
tables · L-R1-03 unknown code clusters list · L-R1-04 ~100 fixed->banked JSRs with unresolved bank ·
L-R1-05 indirect jumps (bank 3 $B71A, bank 0 $830F) · L-R1-06 season end (game end done) ·
L-R1-07 short fragments.

## Unresolved questions
- How to create two custom-team slots (MAKE TEAM -> MAKE A LEAGUE looped with 2 teams).
- Index domains for the unbounded jump tables; bank of fixed-bank calls that rely on $25.

## Deferred work
- Season end (S13), conditional menus (S12), relief pitcher / pinch runner, 2PLAYER mode.
- Commentary: code regions are fragmented by inline-argument data (871 regions); only vector/helper labels set.

## Artifacts
- Scripts (reproducible pipeline): scratch/an.py, build.py, regions.py, gaps.py, dataclass.py,
  allprops.py, commit.py, stats.py, play.py, h.py, rpc.py, broker*.py.
- Coverage dumps: scratch/ctx.json, ctx2.json, ctx2b.json (merged ctxall.json); proposals scratch/props.json.
- Same-build snapshot exports: scratch/{cpuplay_start,menu,league_menu,pregame,game1p,sub_menu}.snap.
- Toolkit issues: analysis/toolkit-issues.md (TK-001 trace slowdown ~11x, TK-002 no WRAM debug writes;
  both missing features, non-blocking).
- No servers left running.
