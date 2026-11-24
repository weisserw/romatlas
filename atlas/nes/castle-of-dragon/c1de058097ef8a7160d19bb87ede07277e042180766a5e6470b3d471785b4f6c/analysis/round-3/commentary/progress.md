# Commentary progress

Prepared verified baseline (pass-003, 752 scaffold entries).

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
