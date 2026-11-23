# Round 2 commentary handoff - COMPLETE (current-source readability, map pass-002)

- Map changed pass-001 -> pass-002 (maps/current.json = analysis/state.json sha256 6fd86fe7...c0df).
  Entire round-1 commentary moved intact to analysis/round-1/commentary/ (no prior copy existed).
- Fresh baseline: `commentary.py prepare` from pass-002 (byte-identical, 487 scaffold entries,
  frozen before/, baseline.json, phase1-state.json).
- Carried by physical address: round-1 annotations (commentary/annotations/*.txt) + renderer
  (commentary/tools/render.py, review.py) re-rendered against the new listing; render reports no
  annotation at a non-instruction address, so all carried names/comments still land on the same
  physical instructions/regions. Only affected records were invalidated (below).

## New / changed review (this round)
- annotations/nocaller.txt (replaces unknown.txt): routine headers for all 12 no-caller code
  ranges, each describing mechanics and stating "no caller/pointer/fetch found, static only":
  0:$844C NoCaller_RtsAfterLoseLife, 0:$9843 NoCaller_JmpGameOverFade, 0:$98C6
  NoCaller_LaunchActiveCheck, 0:$9A1A-$9A53 BypassedTerrainPaletteBlock (+BypassedPal_Outside/
  _Store), 0:$9E30 NoCaller_StoreFramePhase, 0:$B091 BypassedStarResetZero, 0:$E0B7
  NoCaller_RtsAfterIncScoreDigit, 2:$A1EB NoCaller_FrameLeftClamp, 3:$91B9 NoCaller_RtsAfterSpawnRow,
  3:$9DBD NoCaller_ShadowDrawB3, 3:$9E4A-$9E8D MulSignedB3 (+NoAdd0-7/_Return), 3:$A067-$A0A1
  NoCaller_BulletHitCheckB3 (+BulletHitB3_Loop/Next/Hit/Rts). Stale data notes DataB3_91B9 and
  MulB3_Image removed; neighbour notes that said "after the unknown bytes" rewritten.
- Redirect progression facts (S07-S10) folded in: RAM $83 Landing -> MissionEndMode (climb-out
  mode 1/2); CrashState $75-$7E = docking sequence; mis-named "explode/crash" labels in the docking
  approach renamed Flight_DockApproach/DockState/DockIdle/DockButton/DockSequence/DockCheckWindow/
  DockFailed/DockedWait, Flight_Docking -> Flight_DockSparks; B+Up debug notes corrected per context;
  DockingSetup confirmed (S07), mission-8 base attack noted (S09). uncertainties.md items 1, 2, 6, 7
  updated (item 1 resolved).

## Readability assessment
- All classified source (100% of ROM; 0 unknown) meets the round-1 standard: every code region has
  a named routine/continuation with prose; no Ln_XXXX labels and no raw non-immediate RAM/ROM operands
  remain. review.json: 487/487 entries reviewed, 544 units; editorial_audit pending (final gate).
- Remaining address-named data labels (112) are fill runs and generic regions with notes, as in
  round 1.

## Verification / reading copy
- `commentary.py verify`: ok true, byte_identical true (game.asm sha256 25138423...0586, 24,284 lines).
- `commentary.py read`: commentary/reading-after.asm refreshed.

## Remaining gaps (meaning only, documented in source + commentary/uncertainties.md)
- Reachability of the 12 no-caller ranges above (purposes inferred, never observed).
- Landing autopilot variant in terrain sections $2C/$44 (0:$92DC Flight_AutoLand, 3:$D7EF
  UpdateGroundObjects) read statically only.
- Data without consumer: 1:$D649 = 3:$D7B8, 3:$D76E, 3:$D90A, 2:$FE65-$FFB6, 3:$F1C3-$F28C,
  1:$FE16-$FE21; undecoded internals (metasprites, spawn lists, maps, music); RLE overlaps
  (bank 1 stream $BA at $80C9; bank 0 $CA01 through $CAF8).
- RAM routine images 0:$F796-$F805 / 2:$C730-$C794 remain Data (L002, toolkit-blocked).

## Questions for redirect
1. Optional: observe the $2C/$44 landing ending (MissionEndMode=1 via UpdateGroundObjects) -
   e.g. a mission starting in section $2C with MissionStage patched to its objective.
2. Controller-2 B+Down ($44) on the ground: any effect beyond the phase code? (low value)
3. L005 data semantics only if finer data classification is wanted; no classification work pending.

## Toolkit
No new issues (prepare/verify/read worked); TK-001/TK-002 unchanged.
