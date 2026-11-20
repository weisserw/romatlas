# Commentary uncertainties

Round-1 notes: analysis/round-1/commentary/uncertainties.md; round-2 copy: analysis/round-2/commentary/.
Open items are carried forward below.

## Carried from round 1 (status after pass-002)
- Q1/L-009 ($BA2A misaligned decode): resolved in pass-002; FightFish_SetThresholds starts at $BA2A.
- Q2/L-010 (bank-0 $8B81-$8D15 called from fixed-bank wrappers): resolved; now code (window routines).
- Q3/L-011 (Fight_BuildScreen bank context): resolved; banks 0/1 observed, $ADC0/$92A7 are code.
- L-012 (fight event script $E5DC): classified Data in pass-002; label FightEventScript.
- Still tentative: GloveHeat/ReelHeat/FightGauge names, fight-event meanings (from texts),
  fight fish model quantities, sound command semantics, FEV_ROD_BROKEN writer not found.

## Round 2
- HookedFishType (was TimeOfDayScene): 9 kinds; 0-4 are marlin (IsFishMarlin, MARLIN vs OTHER
  FISH records, "under 200 pounds ... too young" test), 6 is set by the shark event. Exact species
  names per index are not decoded (name tiles via FishNameTiles / PPU command 5).
- FishSize (was FishPullScale): weight = FishSize*8 + random 0..7 pounds (FightFish_ComputeWeight);
  its use as a scale factor in the fight model is unchanged.
- FishTypeAdjustSet ($05AC): read by FightFish_AdjustTable, but no code or script writer found
  (searched sta/stx/sty/Poke encodings); probably always 0, so only the first adjust table is used.
- FightFish_AddStaminaFar ($ECE4) sets only Ptr2Lo; Ptr2Hi is whatever the script/engine left,
  so the effective signed term may depend on earlier state (original-code behaviour, untested).
- PasswordEntry_PlaceCursors/NameEntry_PlaceCursors: the grid row drives ObjXLo and the column
  ObjYLo. Either the grid is drawn transposed or the round-1 ObjX*/ObjY* array names are swapped;
  not resolved (would need a frame capture of the name/password screen).
- Level-up rules (LevelUp_*): mechanics exact; the source of FightSkillScore points (script
  ScriptAccumulator values before ScriptNative_AddSkillScore) is per message.
- $CC/$CE/$D0-$D2 are shared script-scene work bytes (SceneCounter/SceneValue/EntryRow...);
  TitlePictureStep is a constant alias for $D0 in Title_ShowPicture.
- Unknown ranges (L-017): superseded in pass-003 (L-019/L-020 classified them); see round 3.

## Round 3 (pass-003)
- All round-3 code is Static with no known caller/table entry/execution. Names describe mechanics;
  any game role is tentative (e.g. FishShadow_TryBiteAlt "probably an earlier variant").
- Fish_CheckMapBounds sets FishState $A0 for leaving the map on Y; FishAI_Type0 handles $80/$40 only,
  so the meaning of $A0 is unknown.
- FishAheadHandlers: facing 8 uses the down+right handler (like 5-7) where "right" would be expected
  by symmetry; the facing-to-direction mapping itself was not checked against the observed movers.
- FatigueDrainFlag ($05A5): no writer found (absolute operand and script Poke encodings searched); BitArrayPtr ($84):
  no writer found either. Indirect writes cannot be excluded statically.
- OffsetPair16/18 helpers: what the $16/$17 and $18/$19 pairs hold for them is not established.
- ScriptSub_ShowAndWaitButton: command decoding is exact; no script calls it (parser did not reach it).
- 5 unknown bytes (L-021): lone RTS x3 in bank 1 and NOP x2 in bank 15; filler vs unreachable
  epilogue cannot be decided from the bytes alone. (Superseded in round 4: the two NOPs became Static code, L-022;
  the three RTS bytes remain unknown.)

## Round 4 (pass-004)
- The two bank-15 NOPs ($E0F6 NopBeforeWaitFrame, $E1B3 NopBeforeIrq) are now Static code (L-022):
  each falls into an observed routine entry. No reference is known, and whether they are deliberate
  padding or leftovers is still open. They are not claimed unused.
- 3 unknown bytes remain: the lone RTS opcodes at bank 1 $ADB6/$B2EA/$B343. L-023 was rejected in round 4
  (analysis/handoffs/round-04-disassembly.md): an unreferenced RTS after a tail `jmp` has no control-flow
  contract, so unreachable epilogue versus stray data byte stays open.

## Final editorial audit (pass-004)
- Address-style RAM names resolved: $04A0 TourWrapFlags (bit 0 set when TournamentRound wraps after round 4;
  setter and reader paths are static), $07AF EngineLevel (boat engine level, Boat_EngineSound), $0500 RamPage05
  (base of the page cleared at reset; no other reference), $05F6 HarbourReturnFlag. The $05F6 meaning is
  tentative: Fight_ReturnToSea goes to the harbour when it is nonzero, and bytes `$32 $F6 $05 $00/$80` at
  bank-12 $98C3/$98CA read as ScrCmd_Poke of $00/$80 if the data after ScriptInline_TimeUpTest ($98B5-$98D0) is
  script bytecode. That region stays Data in phase 1; its command boundaries were not verified.
- 482 data regions still carry only the generic phase-1 rule note ("consumer/format not identified"). They are
  explained as unidentified data, not decoded; this is a readability limitation, not a classification gap.
- ScriptArgC6/C8/CA keep their address-derived names for consistency with Script_ReadWordC6/C8/CA and
  Script_CallC6; their roles are now commented on the RAM definitions.
- The 3 bank-1 RTS bytes ($ADB6/$B2EA/$B343) remain Unknown; the whole-ROM classification is not complete.
