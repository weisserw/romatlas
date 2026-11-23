# Commentary uncertainties (round 1)

Investigated limitations and tentative interpretations. Classification is unchanged
(phase-1 map pass-001); these are meaning-level questions only.

1. Phase sequence naming. Code: ground mission ends by climbing above Y $14 (code $88)
   -> results -> MissionStart($88) runs bank 2 SpaceBattleInit; battle end (PauseState=2)
   gives code $44 -> results -> MissionStart($44) -> DockingSetup (station pictures,
   DrawDockingStation). Landing completion at Flight_EnterShop leads to ShopInit.
   "Docking" and "landing" are inferred from graphics routines and briefing texts, not
   observed in round-1 traces (L003 progression deferred). Check with a $0669/$1C patch.
2. Joy2Held ($1C) is both controller-2 state and the phase code passed through the
   results screen to MissionStart. Controller-2 combinations B+Up ($48), B+Down ($44)
   and A/B (star speed) look like developer debug inputs.
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
   3:$91B9 (lone RTS), 1:$FE16-$FE21 and $FE8x tail of PictureDataB1, 3:$9E4A
   (MulB3_Image: unreferenced copy of MulSigned). Bank 1 RLE stream $BA ($80C9, 61
   tiles) decodes past the start of streams $96/$AE (overlap; intent unknown).
   Bank 0 shop RLE stream at $CA01 (29 tiles) decodes through $CB07, overlapping the
   16 BootChrAndTexts bytes at $CAF8 (shared bytes or a different count; unverified).
7. The ten L001 unknown fragments are explained locally (decoded mnemonics given in
   prose only); none has a known entry. 0:$9A1A-$9A53 is bypassed by an unconditional
   jump (disabled palette feature?) and 3:$A067-$A0A1 is a complete bullet-hit routine
   with no caller - both candidates for an evidence-policy decision in redirect.
8. Resume pass: all 117 former Ln_XXXX branch labels now have role names (mechanical
   ones such as MulSigned_NoAdd0-7 where only the control-flow role is known). RAM
   $54, $78, $A1 and $06FF are written but have no direct reader (WriteOnly*/
   LastHeightBlock); $400D writes go to the unused APU register (NOISE_UNUSED).
