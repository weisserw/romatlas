# Round 2 commentary handoff

- Status: **COMPLETE** (current-source readability scope done; ready for redirect).
- Input checkpoint: pass-002 (maps/pass-002.json = maps/current.json). Classification
  unchanged by this phase: 100.00% rounded (131082 / 131088; 6 bytes unknown).
- Map/state changed vs the round-1 frozen baseline, so the **entire round-1 commentary
  was preserved at [analysis/round-1/commentary/](../round-1/commentary/)** (moved intact,
  the only copy; not modified) and a fresh baseline was prepared in commentary/
  (before/ frozen, baseline-verification byte_identical true, 772 scaffold entries).
- Verification: [commentary/verification.json](../../commentary/verification.json) -
  ok true, **byte_identical true** (fresh run after the final edit batch), game.asm sha256
  86912353e2d15b79a0b7ad1b5115978bfb92bc94053edbb8193579198b00705b.
- Reading copy (refreshed): [commentary/reading-after.asm](../../commentary/reading-after.asm).
- Records: [commentary/progress.md](../../commentary/progress.md),
  [commentary/uncertainties.md](../../commentary/uncertainties.md),
  [commentary/review.json](../../commentary/review.json): 772 entries all reviewed,
  1150 units all reviewed, 0 pending; editorial_audit pending (final acceptance; audit not run).
- Tooling: local helpers commentary/tools/ (annotate.py + notes.txt, review_update.py,
  dump.py, remaining.sh), carried from round 1 with small extensions; scratch in
  scratch/c2/. No toolkit changes, no new toolkit issues.

## Carried vs new scope

- Carried (round-1 notes keyed by physical offset): all round-1 names, headers, 992+
  locals, inline notes and RAM symbols re-applied to the new baseline; carried build
  verified byte-identical before editing. Invalidated/rewritten only where evidence
  changed: unknown-range notes of every newly classified region, "unclassified bank-5"
  notes, CallStageSplashRoutine/CallStageAttackRoutine bank context (now bank 5),
  Spawn14_Chain head/base, Spawn01_EdgeFlyer -> Spawn01_EdgeWalker, boss hit/spawner
  headers, six local labels promoted to globals by new region splits.
- New: bank 5 $80C7-$9A34 (19 banked object handlers, 18 state tables, ~25 data tables,
  demo scripts), fixed-bank helper fragments ($EBF6, $EC59/$EC64, $ED38, $EE4B, $EF74,
  $DD1B, $FA90 + $FAAB, $FCCC-$FCF3, $CF61, $FF06, DmcSample), bank 6 $90A1-$934B,
  item records $D87B-$DA1E, and the low-confidence L-003 fragments (mechanical notes,
  "Unused..." names).

## Readability assessment

All classified source meets the standard used in round 1: 0 address-default routine
names, 0 structural locals, every routine/state with a header, symbolic operands for
RAM/fields/tables, .word jump tables with state names, explicit uncertainty in prose.
Bank-5 commentary is header-centred (one header per small state routine plus notes at
non-obvious instructions); identities are tentative except where scenarios visited them.

Examples: `ObjT1D_Dracula` (two 15/11-state tables, head sprite/hit box, transformation;
ties to Hit_Boss BossType04Hit), `ObjT19_PhantomBat` (target picking, dodge, swoop),
`Chain_FollowNextSegment` (segment link with clamped offset hand-off),
`ObjT27_Boomerang`, `EraseBossStatueTiles`, `DebugStageSkip`, `UnusedRecordInput`.

## Unresolved meanings

Tentative boss identities for stages 6/9/12/15 (not visited); regular bank-5 enemy
identities (ObjT07/08/0A/10/11/1E, the $14-$16 chain); Mummy state 5 and Dracula state
$0C setters; Medusa statue tile target; purpose of the unused fragments. See
commentary/uncertainties.md.

## Structural questions for redirect (3)

1. **Six unknown bytes** ($C405, $C916, $CA5B, bank-6 $A1E4 = lone $60; bank-6
   $9B8B-$9B8C = `sec; rts`): no reference found in commentary. Bounded check: scan all
   classified data words (and value-1 for pushed returns) and branch targets for these CPU
   addresses in the right bank; without a hit they should stay unknown.
2. **Low-confidence code fragments** (round-2 L-003, alignment only, confidence 0.6):
   $85BF, $C04F, $C0DB, $C0FD, $C312, $C31E, $C8C2, $CC3D, $DA61, $E9AC/$E9B2, bank-6
   $81F3, $A2E2, $AB07, $B8F2/$B8F5 (source notes give mechanics). Redirect may decide
   whether any should be demoted (no caller, pointer or table reference found while commenting).
3. **Spawner-to-stage association**: per-stage column lists overlap (StageItemListPtrs
   half lists share columns), so stage numbers attached to enemy types were read with a
   fixed 40-column scan (scratch/c2/stages.py). A bounded check of each half-list length
   (camera page limits StageRightLimitPages/StageLeftLimitPages) would firm up the boss and
   enemy identities used in names (no classification impact).

## Remaining gaps

None in the commentary readability scope. Whole-ROM classification (6 unknown bytes,
ranges above) and the final editorial audit remain separate gates.
