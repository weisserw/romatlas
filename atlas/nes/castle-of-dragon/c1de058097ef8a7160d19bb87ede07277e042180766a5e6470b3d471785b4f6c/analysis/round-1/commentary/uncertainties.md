# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 1
- $C415 (file $1C425): unknown byte $60 after an RTS. No reference, never executed;
  dead code vs. padding undecided (lead L-001). Explained in source, kept unknown.
- Unreferenced static fragments (L-002) now individually explained: TaskHoldAndYield
  $C1D5, TaskExit $C1EC, SndCmdTransposeRel/SndCmdVolumeRel $CCE6-$CD06,
  YieldKeepY $D375, GuardianNone $9A27 (in GuardianHandlerTbl for type $2F but the
  dispatch filters types >= $2E), GuardianType0F_Unused $A38F, EnemyType19_Unused
  $A6E8, EnemyPadMove_Unused $A870-$A8DE, BossPadMove_Unused $B44B-$B4C1. All look
  like leftover/dead code; none has a caller.
- Game identities of object types (guardians $00-$18/$22, enemies $19-$24, items
  $25-$2D), sound numbers and quest items are not matched to in-game names.
- Staff-roll text at bank 6 $8F92-$90D7 has no direct reader; consumer unknown.
- Banks 4 and 5 are documented by consumer address ranges (map layout, block
  attributes/definitions, metasprite sizes, palettes; collision map, sound tables)
  but internal boundaries were not split into separate regions.
- ObjFieldD/ObjFieldF are only cleared; meaning unknown.
- TerrainFlags bit names (floor/wall) are inferred from use in the physics code.
- PlayerControl's "frame $A9 = defeated" and some guardian AI step names are
  inferred from behaviour, not runtime-confirmed.
- UpdateGuardianPhysics reads ObjSizeClassTbl with Y before loading ObjType
  ($FA3B) - apparent original bug, harmless in observed play.
- FlushVramQueue read entries discard the data they read (original behaviour).
