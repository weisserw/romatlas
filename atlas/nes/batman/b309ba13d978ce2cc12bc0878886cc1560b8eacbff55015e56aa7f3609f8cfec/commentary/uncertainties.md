# Commentary uncertainties (rounds 1-5)

Investigated limitations; each is also noted in the source next to the code.

- Object kind identities: handlers are named ObjKindNN by ObjKind number. Only
  Batman's weapons (kinds 1-3: Speargun-like shot, Batarang, spread "Dirk"; inferred
  from FireWeapon's WeaponType/ammo mapping), the item drop (kind 4), explosion
  (kind 11) and the boss-defeat sequence (kind 29) are identified by role. Which
  enemy or boss each other kind is (e.g. kinds 20/24/26/32/33/42 are bosses by their
  structure) is not confirmed against gameplay.
- ObjKind33 zero-page state is now named (Boss33Speed/VSpeed/Mode/Timer, $70-$73)
  from its use in the approach/dash/leap/turn/attack behaviours; individual
  Boss33Mode bits are mechanical readings. Other bosses' ObjState/ObjCounter bit
  meanings are described mechanically per behaviour.
- Sound command semantics ($A0-$C3 / DMC $A0-$A8) are named from their effect on
  channel variables; SndTempoAdj ($40) use is unresolved; music/sound numbers are
  not mapped to tunes.
- Cutscene scenes 1-6 are described at scene level (sequence of shots); the screen
  content of each scene and the text scripts were not catalogued.
- Bank-4 background streaming (QueueMapColumn/QueueMapRow/QueueAttributes and the
  scroll-limit map) is described at routine level. Round 2: the area banks are now
  labelled at block level (MapNNMetatiles, 8-byte records; MapNNScreenCells, 120 bytes
  per screen) from the redirect L-014 model, reached through MetatileTablePtrs /
  ScreenCellTablePtrs. The model over-counts a few aliased screen ids (banks 9/11, see
  the label notes); the byte-level metatile/cell/attribute formats and the bank-10
  attribute data remain undecoded.
- $04F0-$04FF: WeaponObjState ($04F0), WeaponHitTimer ($04F1-$04F3) and ObjAux
  ($04F4+slot) overlap with accesses such as $04F2,x in kind 38; the exact array
  layout is uncertain.
- Branch labels: all 950 former mechanical <Routine>_<loop|exit|fwd>_<addr> labels
  now carry descriptive names (resumed session #2). Names inside the bank-4 streaming
  code (QueueAttributes, LookupAttributeCells, EdgeVramAddress, ApplyScroll) describe
  the mechanical step (row/column, nametable, wrap) rather than a verified data format.
- Round 2: bank 13 $BCB4 is now code (ObjKind42Defeated). Round 3 resolved the value 5
  it stores in ObjXLo+15 ($051F): that byte is ExitDelay, the delayed-exit countdown of
  bank-8 AreaExitCheck (boss areas, flag bit5); every boss-defeat path loads it (kind 13:
  1, BossDefeated: 128, kind 39: 16, kind 42: 5). Redirect r2-a1 followed the defeat
  through the area-17 exit to the ending (RAM-patched experiment, lead L-019). Round 4
  (L-020): the defeat clamp's non-indexed slot-0 X read is the boss's own X under the
  encoded set-up (SetupBossSlots/BossSlotTemplate0 puts kind 42 in slot 0; seen there in
  two area-17 loads). Not ruled out: an area spawn list placing kind 42 in another slot.
- Resolved in round 4 (L-020, redirect r3-a1): SkipArea16Flag bit7 means "area-16 boss
  (kind 33) beaten". Only TitleAndNewGame clears it; continue keeps it and restarts area 17
  at area 15, so AreaCleared then skips area 16. Artificial test: $76=$80 sends the area-15
  clear to area 17, $76=0 to area 16. The name SkipArea16Flag is kept (it describes the
  effect). SetupBossSlotsArea17 is not on that skip path (not fetched); its caller is
  still unknown.
- Round 3 provenance: code regions observed only in RAM-patched experiments (r1-a2, r2-a1)
  are headed "Observed, artificial scenario" in the source; this is execution evidence,
  not natural reachability. Round 4 adds the r3-a1 runs (continue restart $1DE45/$1DE56,
  area-15 skip test $1DE9C, $1DEFE, bank-8 boss-type-5 intro $11717-$11791).
- Round 2 orphan code (no caller known, structural classification only): bank 2
  CsMoveGroupUp, CsSetGroupPos1-3, CsReverseOamOrder, CsGetTextPos, CsQueuePokeAt,
  CsAnimateCommitFinish, CsReadJoypadsAlt (+ helpers); bank 4 ApplyScrollSameBanks;
  bank 5 Cs5_BlinkScreen, Cs5_ClearBothNametables; bank 8 SetupBossSlotsArea17;
  bank 12 ObjKind9Fall_placeAt; bank 14 QueuePpuBlock. Zero-page $38/$3E, used only by
  the bank-2 orphans, are named CsQueuePokeOffset/CsJoyStrobeBase (round 4). The reset
  zero-page clear is their only classified writer and spot reads were 0, so they are
  effectively 0. Run-time-pointer (zp),y writes were not excluded exhaustively.
  Computed-dispatch scans (r3-a1 orphref.py) found no callers for any orphan.
- Round 2 data with weak role evidence: D-SNDX sound bytes after endless loops (no
  entry found; probably unused), the bank-15 sound-format block $F4F1-$F7FF (syntax
  only), the bank-3 period-16 records after Cs3_SeqData2 (no consumer), and the
  duplicate tails of banks 4 and 8 (byte copies, never referenced).
- Bank 14 $C59E-$C59F (file $1C5AE-$1C5AF, 2 bytes, lead L-018): round 5 (pass-005) classifies
  them as generic Data of unknown meaning only as a user-authorized manual fallback (Manual
  provenance). This is not evidence: they were never observed being read or executed, and no
  static reference reaches them. They follow JumpTableDispatch's unconditional indirect JMP and
  would decode as a zero-page load whose result is dead on every path into HideUnusedSprites.
  Unused alternate entry, filler or something else all remain possible; no label or format is
  given. L-018 stays exhausted-unknown; its reopen conditions would supersede the Manual region.
- Some indexed operands are rendered with the neighbouring array's name because the
  16-byte object arrays are adjacent (e.g. ObjKind38 reads ObjSprite+14,x meaning
  ObjPosXHi-2,x; StartArea writes ObjCounter+15,x meaning PaletteBuf-1,x). Values are
  exact; inline comments give the intended array.
- RAM names in banks 2/3/5 use a cutscene override table (notes/ram_cutscene.txt)
  because the cutscene engine reuses gameplay RAM ($30-$3F, $0400-$04FF, $0500 page).
- Round 5 editorial audit: the 111 short notes added to formerly uncommented data tables
  (object-kind sprite/offset/step tables, boss set-up, death animation, terrain thresholds)
  state the consuming routine and the field each byte feeds. Per-entry meanings (which
  animation frame, which slot) were not checked against gameplay.
