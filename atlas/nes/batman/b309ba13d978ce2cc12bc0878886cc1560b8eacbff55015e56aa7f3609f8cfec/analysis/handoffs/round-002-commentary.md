# Round 2 — commentary handoff

Status: **COMPLETE** for current-source readability (all classified source of checkpoint
pass-002). Final editorial audit is reserved for final acceptance.

## Input / checkpoint
- Input: pass-002 (maps/pass-002.json == maps/current.json == analysis/state.json,
  map_sha256 50c836be…c6e3; analysis/pass-002-verification.json byte_identical).
- The map changed since round 1, so the complete round-1 commentary was preserved at
  analysis/round-1/commentary/ (byte-for-byte copy, plus analysis/round-1/scratch-commentary/
  listings) before `commentary.py prepare` built a fresh baseline from pass-002.
- Fresh verification: [commentary/verification.json](../../commentary/verification.json) ok=true,
  byte_identical=true. Reading copy refreshed: commentary/reading-after.asm (32,679 lines).
- Classification is unchanged by commentary: 262,158 / 262,160 bytes classified; 2 bytes
  unknown (file 0x1C5AE-0x1C5AF, lead L-018), explained in the source.

## How carried over
Notes are keyed by bank + CPU address (physical identity), so round-1 names, comments and
review were carried by physical range onto the fresh baseline; source/ is regenerated only by
commentary/tools/annotate.py from before/ + notes/. All round-1 labels keep their names except
the deliberate renames below. Affected records (the R-06 data->code change, the 294 newly
classified data/text regions, the new code ranges, and comments that described now-classified
bytes as unknown) were rewritten; everything else was reused. Resume instructions:
commentary/progress.md (Method + Round 2).

## Scope covered / readability assessment
- All 16 PRG banks, header, CHR, vectors. 459 routines, every one with a purpose header (160
  round-1 routines that only had mechanical Callers/Calls lines got purpose text this round);
  2,653 labels, 0 address-based Sub_/L_/Dat_ labels, 0 mechanical branch labels.
- New code: ObjKind42Defeated (bank 13 $BCB4; defeat animation, controller release via
  SkipArea16Flag bit6, proximity check, BossDefeatFlag), ObjKind1ImpactAnim (bank 13 $A000
  continuation), QueuePpuBlock ($C502), and the orphan entries/routines from C-B2/C-JOY/C-MISC,
  each marked "no caller known; structural code only".
- New data: every pass-002 data/text region has a role note (sound streams and post-loop
  leftovers, cutscene palettes/scripts/metasprites/backgrounds, boss metasprite lists, pose
  lists, hitbox/spawn tables, area-start records, HUD tiles, DPCM padding, duplicate bank
  tails, credits text). Area banks 6/7/8/9/11 now carry MapNNMetatiles / MapNNScreenCells
  labels (redirect L-014 model) with honest notes where the model is approximate.
- review.json: scaffold intact, 2,183/2,183 entries reviewed, 608 units reviewed.

## Findings
- R-03 is answered: JumpToDrawBatman ($C8C9, formerly JumpToBank6_9BB0) is entered by the JMP
  at the end of CommitBatmanPosition (bank 4); the round-1 "no static caller" note was wrong.
- SkipArea16Flag bit6 freezes PlayerUpdate; ObjKind42 sets it on defeat and ObjKind42Defeated
  clears it on the first animation step change.
- AreaIntro boss set-up branches had been named under AddStepToY/BossType4Rows (concrete
  round-1 naming defect), now AreaIntro_bossType*.

## Unresolved questions (also in commentary/uncertainties.md)
- ObjKind42Defeated: why it stores 5 in ObjXLo+15 and why the X clamp reads slot 0
  non-indexed; post-defeat area clear for areas 6/16/17 still not observed (L-003).
- Orphan code reachability (bank 2 cutscene helpers, CsReadJoypadsAlt, ApplyScrollSameBanks,
  Cs5_BlinkScreen, SetupBossSlotsArea17, ObjKind9Fall_placeAt, QueuePpuBlock); zero-page
  $38/$3E roles (only used by the bank-2 orphans).
- Weak-role data: D-SNDX (post-loop sound bytes), bank-15 sound-format block, bank-3
  period-16 records after Cs3_SeqData2 (no consumer), duplicate tails of banks 4/8.
- Byte-level metatile/cell/attribute formats of the area banks and bank 10; map 4/5/7 screen
  aliasing in the L-014 model.
- Possible bounded redirect checks (low priority, no classification at stake except L-018):
  trace a kind-42 defeat through the area clear; search for callers of $C59E only if new
  evidence appears.

## Deferred
- Final editorial audit (review.json editorial_audit pending) — reserved for final acceptance.
- Whole-ROM classification remains incomplete by 2 bytes (L-018); separate from readability.

## Toolkit
No toolkit issues found (commentary.py prepare/verify/read worked as documented);
analysis/toolkit-issues.md unchanged.
