# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 3 (baseline pass-003)
Status changes since round 2:
- S-PROG3 title drop RESOLVED (round-2 redirect): a pit fall in stage 3 runs the
  game-over sequence (frame $D0 figure) and JMP Reset. Noted at UpdateHudAndStageEnd.
- MsprFrameData $8A1D-$8A40 note REWORDED: the 36 bytes are unique in the ROM and look
  like a tile grid (values $00-$40, empty cells, ascending runs), probably an unused
  frame; no frame pointer covers them. $8F89 is a single $00 after frame $D0 (pointer
  target of the empty frames $D1-$E5). Usage still unknown.
- $C5D4 renamed SpawnLiftPlatform (was SpawnStage7BossBody): HalfPageSpawnTbl type $24
  on pages 14/15/22/34/35/38. Exact meaning of the mirrored Y/VelY rule and of palette 7
  value $35 is inferred from code only (one observed spawn).
- Stage 5/6 boss screens (BossStage5/6, BossAiStage5/6, LoadStage @SetupStage5/6, the
  stage 5/6 BossHitBoxTbl entries) have no natural entry; they executed only under
  forced entry (S-BOSS5F/S-BOSS6F). Whether they are cut content or reached by a path
  not modelled (e.g. a different StageIndex during a stage) is not proven, but no such
  path was found statically.
- Provenance: Static code fell from 7078 to 2769 bytes (L-008, L-010); the
  "[static-only code ...]" markers were regenerated from pass-003. Several observed
  runs (incl. the stage 5/6 boss code) were seen only in artificial or forced contexts.
- Guardian type $16 HP: reset to player HP - $40 each frame without armour (code);
  its exact defeat condition in play was not observed.

## Round 2 (baseline pass-002b)
Status changes since round 1:
- Staff-roll consumer RESOLVED: bank-6 MsprFrameData frames $F6-$FF (StaffRollFrames
  $8F8B-$90D7) drawn by DrawMetasprite, stepped by UpdateEndingScene2 (round-1 redirect
  ending run). Frames $E6-$EF also point at $8F8A and overlap the text; whether they are
  ever used is unknown.
- Banks 4/5 internal boundaries RESOLVED by the round-2 typed split (L-004); the
  assignment of map page groups (0-17/18-33/34-49/50-56) to block sets 0-3 is inferred
  from matching block-id maxima, not from a traced ScreenKind per page.
- L-002 labels renamed to *_Unreferenced and prose reworded: no caller or reference was
  found, but non-reachability is not proven. $CCE6-$CD06 confidence now 0.85.
- SndSequenceData end ($BC67) is the last non-zero byte; the exact end of the final track
  stream is not proven. The zero runs in banks 4/5 have no reader (probably fill).
- (superseded in round 3) MsprFrameData $8A1D-$8A40 / $8F89: referenced by no frame.
- The ending's @FadeAway part of PlayEnding and UpdateEndingScene2 step 2 are not reached
  in the observed flow (scene 2 ends in an endless yield).

## Round 1 (still applicable unless noted above)
- $C415 (file $1C425): unknown byte $60 after an RTS. No reference, never executed;
  dead code vs. padding undecided (lead L-001). Explained in source, kept unknown.
- Unreferenced static fragments (L-002) individually explained: TaskHoldAndYield
  $C1D5, TaskExit $C1EC, SndCmdTransposeRel/SndCmdVolumeRel $CCE6-$CD06,
  YieldKeepY $D375, GuardianNone $9A27 (in GuardianHandlerTbl for type $2F but the
  dispatch filters types >= $2E), GuardianType0F_Unreferenced $A38F,
  EnemyType19_Unreferenced $A6E8, EnemyPadMove_Unreferenced $A870-$A8DE,
  BossPadMove_Unreferenced $B44B-$B4C1. None has a known caller; reachability unresolved.
- Game identities of object types (guardians $00-$18/$22, enemies $19-$24, items
  $25-$2D), sound numbers and quest items are not matched to in-game names.
- ObjFieldD/ObjFieldF are only cleared; meaning unknown.
- TerrainFlags bit names (floor/wall) are inferred from use in the physics code.
- PlayerControl's "frame $A9 = defeated" and some guardian AI step names are
  inferred from behaviour, not runtime-confirmed.
- UpdateGuardianPhysics reads ObjSizeClassTbl with Y before loading ObjType
  ($FA3B) - apparent original bug, harmless in observed play.
- FlushVramQueue read entries discard the data they read (original behaviour).
- (resolved in round 3) Stage-3 drop to the title screen = pit fall -> game over -> Reset.
