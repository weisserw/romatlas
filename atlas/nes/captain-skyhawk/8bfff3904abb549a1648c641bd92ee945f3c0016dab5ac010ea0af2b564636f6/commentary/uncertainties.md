# Commentary uncertainties (round 3, map pass-003)

Investigated limitations and tentative interpretations. Meaning-level questions only;
the pass-003 map classifies every byte. Earlier versions: analysis/round-1/commentary/,
analysis/round-2/commentary/.

1. Phase sequence - RESOLVED in round 2 by redirect scenarios S07-S09: ground mission
   objective (MissionStage = MissionObjectiveStage[Mission]) -> MissionEndMode=2 climb-out
   -> Y < $14 -> code $88 space battle -> PauseState=2 -> code $44 docking approach
   ("DOCK WITH SPACE STATION") -> B inside the window (Flight_DockApproach) -> shop ->
   NEXT MISSION -> next briefing. Final mission: base attack, MissionStage $40 ->
   congratulations -> game over / name entry. Former Landing ($83) renamed
   MissionEndMode; the CrashState $75-$7E "explosion" labels were the docking sequence
   (renamed Flight_Dock*). Carrier landing observed in round-2 redirect S11:
   missions 4 and 8 (index 3/7) are the carrier missions; the objective reached in
   section $44 (approach) or $2C (deck) runs the Flight_AutoLand autopilot, the plane
   lands (OnDeck=1, speed 0), UpdateGroundObjects' marker reaches GroundObjs_Landed,
   MissionEndMode=1, climb-out, briefing. No open phase-sequence question remains.
2. Joy2Held ($1C) is both controller-2 state and the phase code passed through the
   results screen. B+Up ($48) is a developer debug input: confirmed ending the space
   battle (S10); statically it also forces the ground climb-out and skips docking to
   the shop. RESOLVED for B+Down ($44): no ground handler tests it (S10 visual, S12
   static/runtime check). Controller 2 cannot choose the starting phase either:
   Briefing_NextPhase overwrites Joy2Held with the saved code before MissionStart (S12).
   Only the A/B star-speed bits in space remain unverified (low value).
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
   Regions without any identified consumer (with notes): 3:$D90A (DataB3_D90A, the only
   address-named data label left), 2:$FE65 (UnrefMetaspriteSetB2, metasprite-shaped),
   3:$F1C3 (UnrefHeightMapsB3, probably spare height maps),
   1:$FE16-$FE21 and $FE8x tail of PictureDataB1. (3:$91B9 and 3:$9E4A are now code,
   item 7.) Round 3 (pass-003, L009): the former "no consumer" 1:$D649 and the
   $D630/$D670 fills are records 3-7 of the 128-byte BlockSubstTables (reads observed
   for all 8 mission indices); bank 3 $D76F-$D7EE (BlockSubstTablesCopyB3) is a
   byte-identical copy with no reader found (named, but unread). Bank 1 RLE stream $BA ($80C9, 61
   tiles) decodes past the start of streams $96/$AE (overlap; intent unknown).
   Bank 0 shop RLE stream at $CA01 (29 tiles) decodes through $CB07, overlapping the
   16 BootChrAndTexts bytes at $CAF8 (shared bytes or a different count; unverified).
7. No-caller code (pass-002, L006-L008): 0:$844C, $9843, $98C6, $9A1A-$9A53, $9E30,
   $B091, $E0B7; 2:$A1EB; 3:$91B9, $9DBD, $9E4A-$9E8D (MulSignedB3), $A067-$A0A1, and
   (pass-003, L009) 3:$D76E, a lone RTS after GroundUnder_Height's RTS. Redirect r03-a:
   3:$D76E-$D7EE is byte-identical to 1:$D5FF-$D67F (DrawMetaspriteB1's final RTS +
   BlockSubstTables), so it is a copied RTS line; a padding reading predicts the same
   byte and is not discriminating (Code role unchanged). All are
   commented as static code with unresolved reachability (NoCaller_*/Bypassed* labels).
   Purposes are inferred (bypassed palette tweak, older hit test, leftover RTS bytes).
8. Resume pass: all 117 former Ln_XXXX branch labels now have role names (mechanical
   ones such as MulSigned_NoAdd0-7 where only the control-flow role is known). RAM
   $54, $78, $A1 and $06FF are written but have no direct reader (WriteOnly*/
   LastHeightBlock); $400D writes go to the unused APU register (NOISE_UNUSED).
9. Round 3: ObjectHit_ObjectiveHit writes block $6B into BlockSubst[7]/[8] when
   objective object $42/$43 is hit, remapping map blocks $F7/$F8. Observed mechanically
   (redirect r03-a, S13, mission 2, forced write): block $69, a dark recessed opening in a
   raised platform, is replaced by $6B, a closed flat panel, and the ground-height lookup
   also reads the swapped entry. The game meaning (e.g. a destroyed or sealed objective)
   is interpretive; a natural missile hit was not reproduced (E6).
10. Final editorial pass: unreferenced uniform $EA/$FF runs between structures are named
   FillBn_XXXX and described as probable padding (no reader found; not proven unused);
   uniform runs inside identified tables are named <Table>_FillXXXX after their owner.
