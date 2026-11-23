# Commentary uncertainties (round 2, map pass-002)

Investigated limitations and tentative interpretations. Meaning-level questions only;
the pass-002 map classifies every byte. Round-1 version: analysis/round-1/commentary/.

1. Phase sequence - RESOLVED in round 2 by redirect scenarios S07-S09: ground mission
   objective (MissionStage = MissionObjectiveStage[Mission]) -> MissionEndMode=2 climb-out
   -> Y < $14 -> code $88 space battle -> PauseState=2 -> code $44 docking approach
   ("DOCK WITH SPACE STATION") -> B inside the window (Flight_DockApproach) -> shop ->
   NEXT MISSION -> next briefing. Final mission: base attack, MissionStage $40 ->
   congratulations -> game over / name entry. Former Landing ($83) renamed
   MissionEndMode; the CrashState $75-$7E "explosion" labels were the docking sequence
   (renamed Flight_Dock*). Still static only: the landing autopilot in terrain
   sections $2C/$44 (Flight_AutoLand, UpdateGroundObjects) - not observed.
2. Joy2Held ($1C) is both controller-2 state and the phase code passed through the
   results screen. B+Up ($48) is a developer debug input: confirmed ending the space
   battle (S10); statically it also forces the ground climb-out and skips docking to
   the shop. B+Down ($44) showed no visible effect on the ground (S10); A/B star speed
   unverified.
3. SpecialEnemy ($E0-$E9) is used with different layouts in bank 2 (space diving
   fighter) and bank 3 (ground enemy jet); field names are therefore generic.
4. LandingMarkY ($55), CarrierX ($7A), DockState ($0641), EndingFlag ($90) and
   StallFlag ($95) names are tentative (few uses read).
5. Sound command names (SndCmd_SlideA/B, envelope variants, decay) describe the
   mechanics read from the handlers; musical meaning was not verified by playback.
6. Data tables are named from their consumers. Resume pass: the former 96 generic
   regions were traced through pointer tables and decoded stream lengths (RAM RLE
   decoder, DecodeVramBlock markers, GfxDescriptorTable, SongChannelPtrs, PlaneTileSrc,
   TerrainHeight/Metatile/TilePtr chains); boundaries are now exact for the RLE/VRAM
   streams. Still not decoded internally: individual metasprite layouts (PlaneLayouts,
   ObjectFrames, *Frames tables), spawn lists, map/tile contents, music note data.
   Regions without any identified consumer (kept address-labelled, with notes):
   1:$D649 and 3:$D7B8 (identical 39-byte tile-number-like runs), 3:$D76E, 3:$D90A,
   2:$FE65 (unreferenced metasprite-shaped set), 3:$F1C3 (probably spare height maps),
   1:$FE16-$FE21 and $FE8x tail of PictureDataB1. (3:$91B9 and 3:$9E4A are now code,
   item 7.) Bank 1 RLE stream $BA ($80C9, 61
   tiles) decodes past the start of streams $96/$AE (overlap; intent unknown).
   Bank 0 shop RLE stream at $CA01 (29 tiles) decodes through $CB07, overlapping the
   16 BootChrAndTexts bytes at $CAF8 (shared bytes or a different count; unverified).
7. No-caller code (pass-002, L006-L008): 0:$844C, $9843, $98C6, $9A1A-$9A53, $9E30,
   $B091, $E0B7; 2:$A1EB; 3:$91B9, $9DBD, $9E4A-$9E8D (MulSignedB3), $A067-$A0A1. All are
   commented as static code with unresolved reachability (NoCaller_*/Bypassed* labels).
   Purposes are inferred (bypassed palette tweak, older hit test, leftover RTS bytes).
8. Resume pass: all 117 former Ln_XXXX branch labels now have role names (mechanical
   ones such as MulSigned_NoAdd0-7 where only the control-flow role is known). RAM
   $54, $78, $A1 and $06FF are written but have no direct reader (WriteOnly*/
   LastHeightBlock); $400D writes go to the unused APU register (NOISE_UNUSED).
