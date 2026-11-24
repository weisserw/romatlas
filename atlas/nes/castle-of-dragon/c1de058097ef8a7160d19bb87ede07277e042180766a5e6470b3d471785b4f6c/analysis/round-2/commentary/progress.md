# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

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
