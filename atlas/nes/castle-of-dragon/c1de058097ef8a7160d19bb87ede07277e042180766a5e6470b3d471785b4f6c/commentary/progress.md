# Commentary progress

Prepared verified baseline (pass-005, 752 scaffold entries).

## Final editorial audit (2026-11-24) - PASSED

Baseline pass-005 unchanged. Strict editorial review with scripted scans plus targeted
reading (details: commentary/editorial-audit.md). Fixes via notes/*.txt + tools/apply.py
(new "N old new" source-only rename directive): PauseFlags -> BossScreenMode, LoadStage ->
LoadBossScreen, StageStartPageTbl -> StageBossPageTbl, Stage5AnimTiles/Stage6AnimTiles;
stale numeric aliases and SpriteTest*Tbl names replaced; BitMaskTbl note; $E78C note moved
to StageMusicTbl; CartridgeFooter and vectors notes corrected. $C415 wording unchanged
(Manual user-directed generic Data fallback, meaning unknown, L-001 open).
Result: audit ok, byte_identical true (game.asm 1c6efcc4..., symbols.inc 1adb9fbb...),
752/752 regions and 855/855 units reviewed, editorial_audit reviewed; reading-after.asm
refreshed by the audit.

## Round 4 commentary pass (2026-11-24) - complete

Baseline: pass-005 (maps/current.json = analysis/state.json; analysis/pass-005-verification.json
byte-identical). Only map change vs the round-3 baseline pass-003: new region-1245 (file $1C425,
CPU $C415, fixed bank 7, length 1, kind Data, provenance Manual, no label) replacing scaffold
entry unknown-01c425 - an explicit user-directed generic Data fallback, not a proven data role.
pass-004 is a no-op checkpoint identical to pass-003. Diff of before/game.asm vs round 3: only
the $C415 header lines; scaffold ids/offsets/lengths/kinds identical except that entry.

Preservation: whole round-3 commentary/ copied to analysis/round-3/commentary/ (diff -r clean,
43 files) before `commentary.py prepare`; rounds 1/2 untouched. notes/ and tools/ carried from
round 3; tools/apply.py regenerated source/ from the new before/game.asm with no dropped-comment
or lost-label warnings; first rebuild byte-identical and code/label lines identical to round 3.

Edits this pass:
- $C415: source note rewritten (generic Data by user-directed Manual fallback, meaning unknown,
  no role proven, orphan-RTS vs stray byte unresolved, L-001); no label. tools/review.py gives
  region-1245 that summary and one unit; ClearSpritesAndObjects' unit is clipped to end at
  $C414 so it no longer covers the fallback byte.
- Renames: ObjTypeE818Tbl -> ObjTypeExpTbl (experience awarded by UpdateGuardianPhysics),
  ObjTypeE846Tbl -> ObjTypeHpTbl (starting ObjHP in the guardian spawns); group note updated.
  UpdateGuardianPhysics header now names ObjSizeClassTbl/ObjHalfWidthTbl/ObjHalfHeightTbl and
  StageFinalPageTbl instead of raw bank-6 addresses.
- Branch labels: the 14 remaining generic @Next/@Next2/@Next3 got role names (@TestNextFlag,
  @AdvanceState, @RiseDone, @DescentDone, @NextSlot, @NextByte, @NextTrack, @NextFrame,
  @NextSprite). Short header notes added for RewardTypeTbl, GuardianType15, BossNone and
  BossHandlerLo/HiTbl.
- Full inventory rescan: no Sub_xxxx or address-named routine labels, no raw absolute operands
  left; remaining @Loop locals sit in short single-loop routines.

Result: 236 routine labels + 1221 named branch labels, 250 symbols; 752/752 scaffold entries
reviewed, 855 units (all reviewed); editorial_audit pending (final acceptance only).
Verification: byte_identical true (commentary/verification.json, game.asm bec35a06...,
symbols.inc 7ae0f295...); reading-after.asm refreshed. Deferred ranges: none.

## Round 3 commentary pass (2026-11-24) - complete

Baseline: pass-003 (maps/current.json = analysis/state.json, sha256 fc23d9a5...), changed
from the round-2 baseline pass-002b (643 -> 751 map regions; provenance splits of code
regions only, byte roles unchanged). The whole round-2 commentary/ tree was preserved at
analysis/round-2/commentary/ (diff-checked copy; round 1 stays at analysis/round-1/) before
`commentary.py prepare` made this fresh tree.

Carry-forward: notes/ and tools/ copied from round 2; tools/apply.py regenerated source/
from the new before/game.asm. Notes are keyed by CPU address + bank, so every name,
comment and operand override landed on the same physical instruction (no dropped-comment
or lost-label warnings); code/label lines are identical to round 2 apart from the edits
below and the first rebuild was byte-identical. "[static-only code ...]" markers are
regenerated from pass-003 (they now cover 2769 bytes, was 7078). Every comment that
mentions execution status was rechecked against pass-003 provenance (all consistent:
remaining "unreferenced"/"static-only"/"never seen" notes refer to still-Static runs).

Edits this pass (L-009, L-010 and review):
- Rename SpawnStage7BossBody -> SpawnLiftPlatform ($C5D4) and @CheckBossBody ->
  @CheckLiftGroup ($93A4); routine note rewritten (half-page type $24, pages 14/15/22/
  34/35/38, slot search from 12 with wrap, mirrored Y/VelY rule, observed S2-LIFT).
- Stage 5/6 boss content: SpawnPageObjects (boss door skipped for stages 5/6), LoadStage
  (callers; @SetupStage5/6 forced-entry only), BossStage5/6, BossAiStage5/6,
  BossHitBoxTbl (hit rule + stage 5/6 entries), UpdateHudAndStageEnd step 3 rewritten
  (boss-defeat clear for stages 3/4/7, end-page rule always for 5/6, guardian test
  skipped for 3/4; forced 5/6 fights never end) and pit-fall game over -> Reset.
- StageStart route select rules; MsprFrameData $8A1D-$8A40 / $8F89 note reworded
  (probably an unused frame grid, not code); guardian type $16 HP rule (player HP - $40
  each frame without armour); test-mode SPTEST/BGTEST noted as observed (S-TMSUB).
- Shared-header notes for GuardianType03/05/07; tools/review.py now gives units that
  share a group comment a summary pointing at that comment instead of "see source".

Result: 236 routine labels + 1221 named branch labels, 250 symbols; 752/752 scaffold
entries reviewed, 854 units; editorial_audit pending (final acceptance only).
Verification: byte_identical true (commentary/verification.json, game.asm 7de15cf9...,
symbols.inc 7ae0f295...); reading-after.asm refreshed. Deferred ranges: none.


## Round 2 commentary pass (2026-11-24) - complete

Baseline: pass-002b (maps/current.json = analysis/state.json, sha256 e30a89ef...), changed
from the round-1 baseline pass-001b. The whole round-1 commentary/ tree was preserved at
analysis/round-1/commentary/ (diff-checked copy) before `commentary.py prepare` made this
fresh tree (644 scaffold entries, was 497).

Carry-forward: notes/ and tools/ copied from round 1; tools/apply.py regenerated source/
from the new before/game.asm. All notes are keyed by CPU address + bank, so names and
comments carried by physical location; every label, comment and operand override landed
on an instruction start (no dropped-comment or lost-label warnings) and the first rebuild
was already byte-identical. "[static-only code ...]" markers are regenerated from the new
Observed/Static region split. review.json rebuilt by tools/review.py (scaffold intact).

Edits this pass (beyond the carry):
- Banks 4/5/6 data: new per-region notes for all 21 typed bank-4/5 regions, PaletteTable,
  MsprFramePtrTbl and MsprFrameData; CPU-addressed data labels (D b4:/b5:) so code uses
  SndSeqPtrTbl, SndEnvPtrTbl, SndLengthTbl, MsprWidthTbl, MsprHeightTbl, MsprFramePtrTbl,
  PaletteTable instead of the 7 removed address constants (SND_*_TBL, MSPR_*_TBL,
  PALETTE_TBL). New commentary label StaffRollFrames ($8F8B, frames $F6-$FF).
- Staff-roll consumer resolved in comments (UpdateEndingScene2 / DrawMetasprite);
  UpdateEndingScene2 steps explained; PlayEnding notes the observed (artificial) run.
- L-002: *_Unused labels renamed *_Unreferenced (GuardianType0F, EnemyType19,
  EnemyPadMove, BossPadMove); fragment prose no longer claims non-reachability;
  $CCE6 fragment confidence 0.85 and its exact gap ($F6/$F7 handlers) noted.
- Redirect findings: SetupStage stage-7 rule (StagesCleared < $7F -> boss screen,
  >= $7F -> palace walk); test mode STAGE row = MaxEnergy, WEAPON $80 + EXIT = armour;
  weapon cycling on Start (sword -> A -> B) at PlayerControl; CheckBossHit now seen running.
- Readability gaps fixed: purpose comments for continuation entry points (SpawnGuardian,
  SpawnReward, SpawnActivateSlot2, EnemyNext, EnemyIdle, EnemyNone, GuardianSetFrame12,
  DragonItemCheck2-4/All, BossAiStage6, EnvMarkActive, SchedAdvance); the
  WriteChannelRegs comment was attached one byte early (above the previous @Done) - fixed.

Result: 236 routine labels + 1221 named branch labels (unchanged), 250 symbols; 644/644
scaffold entries reviewed, 766 units; editorial_audit pending (final acceptance only).
Verification: byte_identical true (verification.json, game.asm 32bd7468...,
symbols.inc 7ae0f295...); reading-after.asm refreshed. Deferred ranges: none.
