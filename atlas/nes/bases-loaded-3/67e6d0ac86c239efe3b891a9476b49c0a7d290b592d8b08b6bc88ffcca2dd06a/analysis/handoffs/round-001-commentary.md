# Round 1 — commentary phase handoff

Status: complete for the current-source readability scope, with a stated depth limit (see
assessment). ROM classification is unchanged: 61.07% (pass-001); commentary does not change it.

- Input checkpoint: pass-001 (maps/current.json and analysis/state.json frozen as
  commentary/baseline.json and commentary/phase1-state.json; manifest.json has their hashes).
- Buildable source: commentary/source/game.asm (ca65; 26,089 lines).
- Reading copy (refreshed): commentary/reading-after.asm. Frozen baseline: commentary/reading-before.asm.
- Verification: `python3 /root/romatlas/scripts/commentary.py verify <ws>` -> ok, byte_identical
  true; commentary/verification.json (game.asm sha256 322d5f4e…d4e5).
- Records: commentary/progress.md (scope + rebuild recipe), commentary/uncertainties.md
  (tentative meanings + redirect questions), commentary/review.json (1,020 scaffold entries
  reviewed with source lines, 1,279 units; editorial_audit pending for final acceptance).

## Method (reusable for later rounds)
source/game.asm is regenerated from before/game.asm by analysis/tools/round-001-commentary/emit.py
using a database keyed by bank:CPU address (names_core.py and names_bNN.py). Run parse.py, emit.py,
review.py, then commentary.py verify/read. On a new checkpoint, preserve this commentary under
analysis/round-N/ first, prepare a fresh baseline, and re-run the same scripts: names carry by
bank:CPU address (re-check any key whose instruction boundary moved; emit.py reports how many
routines are named). Scratch db lives in /tmp/ra67c (recreated by parse.py).

## Scope covered
All classified source: 763 code regions (33,004 bytes) in banks 2, 8, 14, 16, 18, 20-28, 31;
76 data/padding regions; 181 unknown gaps.
- Routines: 932 global labels; 923 hand-named with header comments; 9 automatic shared-tail names.
- Branches: 1,122 local labels (core/key ones hand-named; the rest mechanical @loop/@join/@skip/
  @done/@return chosen by flow shape).
- RAM: ~150 named variables/arrays with comments; remaining referenced RAM as Var_xxxx with the
  banks that use them. Hardware registers, MODE_* GameMode values, OBJ_* object fields,
  ObjPtr set-ups, FarCall table reads symbolised. Every FarCall site names its resolved target.
- Data: pass-001 jump tables as .addr handler lists; data/padding/CHR/DPCM notes; ROM tables
  read by code have equates (named or Tbl_BB_AAAA); unknown gaps carry boundary notes, table hints
  and specific "probably code/data" notes.

## Readability assessment (honest)
- Strong: bank 31 (boot, main loop, NMI/IRQ, FarCall and restart, PPU upload, camera, metasprites,
  controllers, math, DMC), module dispatch, bank 2 streaming, bank 8 driver structure, bank 14/16
  screens, field-play/pitch-view result flow (FieldPlayUpdate, ApplyPitchOutcome, EndHalfInning).
- Adequate but shallow: most banked routines in 20-28 have a one-line purpose derived from static
  digests and targeted reading; many names are tentative and say so. Mechanical branch labels in
  those banks carry no semantic meaning.
- Not done: per-state meanings of handlers reached via unclassified inline jump tables; symbolic
  sound/music ids; screen ids.

## Examples
- `FarCall` (bank 31 $EF38) with block comments on the guarded MMC3 sequence and restart points.
- `jsr a:FarCall ; bank 26 entry 1 -> FieldPlayUpdate` in RunModeUpdate's handlers.
- `lda #MODE_MENUS|MODE_INIT / sta z:GameMode` in FieldPlayUpdate's @threeOuts.
- `ldy #OBJ_PROGRESS / lda (ObjPtr),y` in RunnerAddSpeed.
- Unknown note at $EAC7: probable RunModeInit jump table whose first word points at an RTS byte.

## Unresolved meanings
Var_xxxx RAM (counted in source equates), scoreboard values $0356/$0357/$0363/$03DA, clock
$03E5/$03E6, BootABFlag effect, CPU AI internals, runner/throw/pitch-type state identities,
extra objects in bank 23 (A6CF), sound command set in SndReadCommands. See commentary/uncertainties.md.

## Questions for redirect (details in commentary/uncertainties.md)
1. FarCall pointer tables at $8001 of each even bank (L-002) — bounded by used indices.
2. RunModeInit/Update tables $EAC7/$EAFB and RTS bytes $EACF/$EB03 (bank 31).
3. Bank-31 data with exact bounds: $E9D5 (4), $ED89 (8), $F444 (100), $FA72/$FA77, $FAE6 (6).
4. Calls into unclassified bytes: bank 25 $B93D (cold boot), bank 23 $A1DB/$A1F1/$A58B/$A66B,
   bank 22 $994D (FarCall entry 3).
5. Banks 0 and 4 likely stadium field banks sharing bank 2's code header: confirm via ground select.
6. Remaining inline jump tables (L-001); 7. RAM watches for count/outs/results/stadium.

## Remaining gaps
- Readability depth limit above (tentative banked names, mechanical branch labels); final
  editorial audit not run (reserved for final acceptance).
- Whole-ROM classification incomplete (153,078 unknown bytes); unchanged by this phase.
- Toolkit: no toolkit issues found in this phase (commentary.py prepare/verify/read worked).
