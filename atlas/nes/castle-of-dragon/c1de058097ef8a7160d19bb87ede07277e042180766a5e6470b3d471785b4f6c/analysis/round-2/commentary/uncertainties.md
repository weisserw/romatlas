# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

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
- MsprFrameData bytes $8A1D-$8A40 (36 bytes, look like a cut-off copy of bank-6 code) and
  $8F89 are referenced by no frame.
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
- Why stage 3 (S-PROG3, round 2) dropped from gameplay to an emblem screen and the title
  is not determined (possibly a side effect of RAM assists).
