# Commentary uncertainties (rounds 1-4)

- Event overlay RAM $91-$E2: only clearly understood variables are named (scoped per event).
  Foot bag and flying disk variables remain numeric. Round 2 replaced all address-based routine names
  (FootBag_xxxx, Disk_xxxx, HalfPipe_xxxx, Bank6_xxxx) with mechanical/tentative role names; the
  flying-disk actor roles (runner vs catcher) and foot bag pose meanings remain tentative.
- $07-$09 are read only by RandomByte. Round 2: the reset code (PowerOnReset/ClearRamAtReset) clears
  only $21-$FF and $0200-$07FF, so $07-$09 are never written - uninitialised RAM used as entropy.
- PauseOrHoldFlag ($0D), StatusMode ($55) and EventTries ($87) have event-dependent uses; the
  names describe the dominant mechanical use only.
- SprFlags/SprAttr/SprBankFlag: the bank 5 renderer uses SprFlags bit 0 for horizontal mirroring and
  SprBankFlag to select vertical placement direction; other bits not decoded.
- Bank 4 entry 0 (PickRandomMessage) is called by BMX through BmxCallBank4 with X = comment category; the message texts
  themselves are in still-generic data.
- Skating SkateFrame_Finish compares the player's name with two reversed 6-character strings
  (SkateEggNames: "PAUL", "RACHEL" - the same names as the boot signature) before a palette flash;
  an easter egg, not tested at runtime.
- Fixed-bank code reading $8000-$BFFF tables (e.g. PrintPlayerScore -> $A5FF) assumes bank 0 is
  mapped, which holds for every traced context; other contexts are not excluded.
- Phase-1 structural doubts carried forward: cross-window instruction $BFFE/$C000 (L-R1-01, TK-04).
  Reset code and the indirect-table targets were classified in round 2 and are now commented.
- Round 2 residual unknown bytes (L-R2-01, 149 bytes in 18 gaps) carry decode-for-reading notes in
  the source; none has a known caller, so code/data stays undecided.

## Round 2 additions
- ShowEventScore (name kept from round 1) is only reached from SurfRideEnd, so it is the surfing final
  score screen; the five rating digits $D1-$D5 are probably the judges' cards (not confirmed on screen).
- (Corrected in round 3: $61 and $4D are read - see below.) Menu kind 4 rows are read as yes/no on
  the confirm-players screen and kind 5 as an event list; the screen texts for those kinds were not
  matched to the menu kinds.
- Course script commands (bank 6 CourseCommandTable): $0A/$0C adjust $93/$94 and $10-$16 set $BE;
  the game meaning of these values (and of flag byte $95) is not established.
- Sound command 2/3 semantics (envelope mode / yield) are mechanical readings; command 9 preset
  records were not listened to.
- SurfState5/6/7 interpretations (manoeuvre, airborne, sinking after wipe-out) are tentative.
- BuildSurfScoreBackdrop: the picture content (judges' stand) is inferred; the two pattern copies at
  CHR $0310 and $0810 and the $FF-filled plane for tiles < $64 are exact mechanics.

## Round 3 additions
- Corrections: RegistrationNeeded ($61) is read by StartCompete (0 = skip registration and only ask
  for confirmation); TextGlyphMode ($4D) is read by the bank 2 text printer (glyph-pattern vs tile
  strip output). PlayerNames records are 7 bytes (name + team byte = PlayerTeams), not 6.
  FootBagPlayerMove was renamed FootBagStepPose (it sequences pose animation, not d-pad movement).
- Round-3 code with no caller (Bank0Entry0, CallBank0, UploadStatusRow2360, SetStatusModeOneAndClear,
  PlaySfxSlot6, FillTerrainTableForever, three StubJump* stubs) is described mechanically; whether
  any of it is ever executed is not established. Bank0Entry0 would return into the copyright text.
- TK-04: bank 0 $BFFE-$BFFF + bank 7 $C000 stay Unknown (one observed instruction across the window
  boundary that a Code region cannot represent); explained in the source at both spots.
- Event overlay naming (round 3 added Kicker*/Bag* for foot bag and five Disk* names). Still numeric:
  half pipe body-segment arrays ($91-$C1 except PipeTrick/PipeTrickStep; indexed per body part, roles
  not separated), flying disk $A1/$A2/$A5-$BB actors (thrower/catcher/runner roles conflict between
  routines), foot bag side object $DE-$E2 and counters $D6/$D7/$D9/$DB-$DD, $A7/$A8, BMX trick/landing
  bytes $C2-$D4, skating $CA-$D6 and surfing $C0-$D8 extras. The routine prose describes these by
  address; naming them needs per-event study rather than guessing.
- AwardFirst/Second/Third ($5C-$5E) add 5/3/1 to PresetScores, read here as medal points; AwardLayout
  ($68) layout meaning is tentative. Bank 6 course object slots ($73-$7E) are named only at the base.
- Text-screen tables $EE7D/$EE86 (per-kind first line / cursor X) are left numeric; TeamNameEnds,
  TextCursorOam and PlayerNameOffsetsB were added.

## Round 4 additions
- Round-3 naming gaps closed (leads L-R3R-01..09 applied after re-reading the code): flying disk
  (throw meter / disk / catcher, Parallax1-4 layers), half pipe board and rider records, foot bag
  seagull and trick-pattern cursors, BMX air/trick/course state, skating ball, surfing aerial /
  shark fin / carve / sea-row animation, bank 6 course builders. Raw RAM operands 922 -> 62 (50 of
  them are the deliberate zero-page base-pair operands `$00,x`/`$01,x`/`$02,x` of the 16/24-bit
  helpers). Raw ROM-table operands 415 -> about 120 (mostly bank 1 CHR page copies and bank-select
  writes to $8000).
- Corrections in round 4: the disk "catcher" routines are the thrower's throw meter; BmxLand was the
  takeoff, BmxScoreTrick the course finish, BmxUpdateTrick the hazard check; BmxTrickBonusTable is in
  reverse order to the old labels; LoadSurfColumnSet loads BMX rider patterns (renamed
  LoadBmxRiderTiles); LoadTerrainStrip loads the seagull patterns (LoadGullTiles); BmxCallBank4 shows
  a random comment, not a sound; HalfPipeSaveState copies while no fall is pending (L-R3R-09).
- Still tentative: half pipe "rider follows the board until a fall" reading of the two records (record
  8 diverging at a trick was not observed); PipeReact* sprite (spectator/flag?) meaning; BMX BmxLift
  (Up/Down rider offset bounded by the course clearance) and BmxWheelA/B (two bike parts, probably the
  wheels); SurfRideFlag ($CA) meaning; DiskUncaught's double use as the slow gravity step; the
  re-launch in DiskFlightUpdate (caught disk bobbing) is a static reading.
- Same zero-page byte, different names per event: names are scoped to code ranges (S lines in
  annotations.txt). Shared bank 6 code is the exception: CourseObjects/CourseObjectPalette write
  CourseMarker*/CoursePal* ($A9-$C1) for both course modes, which overlap BMX variables (BmxOnGround
  $A9, CoursePatternMode $BE). Presumably the BMX course has no type-1 or palette objects; not checked.
- Table symbols whose base lies inside code (PipeFrameFlags, PipeFallFramesA, SurfFaceTileIndex,
  SkateLeanFrames, FootBagDigitTiles, TeamLogoAddrHi/Lo, FrameBoxAddrs, AnimTileIndex,
  TextSpecialSlot) are index-offset aliases; the real index ranges were not enumerated, so their
  extents are not established.
- SkateBallSpawnMasks is 4 bytes but read with index 0-15 (static reading; indexes 4-15 read SurfStart
  code, so balls appear only while CameraX low byte is $00-$3F). BmxAirScoreLo/Hi are read with index
  21 (one past the 21-entry tables). Both are kept as described original-game behaviour.
- Unnamed RAM left raw: $0167 and $0172 (read by ShowAwards; role not established), page clears at
  $0400/$0500, RAM-routine jump targets $0310/$042A (commented), BIT-operand bytes inside CallBank0 /
  Bank2Entry2 ($01A9/$04A9, code overlap tricks), OamBuffer-1 and PpuQueue-1 index bases (commented).
- Branch labels inside routines remain structural (Owner_Skip/Loop/Return); key BMX landing targets
  got names (BmxTouchdown, BmxScoreLanding, BmxLandingCrash, BmxRollOffCrash).

## Round 5 additions (resolutions of Round 4 items; leads L-R4R-01/02)
- RESOLVED (L-R4R-02): shared bank 6 course code vs BMX variables. CourseObjects arms markers only for
  CourseObjType 1, and CourseObjectPalette claims palette slots only for types 1-$7F. Every one of the
  642 BMX object records (stream $97C2) has type 0 or $FF, so in BMX $A9-$AB/$AE-$B1/$B6-$B9/$BE-$C1
  are never written by this code. A BMX watch found 0 executions of the marker/palette stores; in a
  skating control run the marker store executed once. The overlap is a skating-only reuse of the
  bytes. The CourseObjects "Writes" list now shows CourseMarkerXHi/CourseMarkerX, not
  CoursePatternVramHi/Mode; the scope boundary was off by one.
- RESOLVED (L-R4R-01): index-offset table aliases. All 20 are listed below, defined as
  Name = Name_XX - $XX, with Name_XX at the first real byte. Every real extent lies inside
  classified Data.
  - Index domains from static gates or loop bounds:
    - FrameBoxAddrs: 1..$25.
    - TextSpecialSlot: $3E-$4D.
    - PipeFallFramesA/B, PipeRiderAngleFrame and PipeBackwardsBoard: 4..12.
    - SurfFaceTileIndex: 2..7.
    - SkateLeanFrames: $D0-$E5.
    - TeamCursorOam: 1..4.
    - SpriteListStart: $0B-$13.
    - FootBag letters/digits: 'A'-'Z' / '0'-'9'.
    - PipeWallX: $5E-$84.
  - Observed index sets: TeamLogoAddrHi/Lo 10..18; the half pipe rider frames 13..78.
  - The rider-frame maximum $5F is from table packing, not observation. Five parallel 83-entry tables
    have identical $FF sentinels and pack end-to-start. AnimTilePageGroup ends at code $830B, and the
    bank 0 pipe tables pack gaplessly from $96DC to $980D.
  - Five wrong comments were corrected:
    - FootBagLetterTiles "$41+ = $A256";
    - PipeFallFramesA "angles past 8 read B";
    - PipeFallFramesB "13 bytes" (it is 9);
    - TextSpecialSlot "lines $28+";
    - TeamLogo "team number" (it is the graphics block $0A-$12).
  - The static-only domains (TextSpecialSlot, FrameBoxAddrs, SkateLeanFrames, PipeFallFramesA) did
    not run in the bounded watches. Their gates are exact.
- Branch labels: descriptive names now cover HalfPipeFrame, BmxFrame, SurfFrame (+ SurfAdvanceWave),
  FootBagFrame, PipeBodyStoreAngle, CourseObjects and CourseObjectPalette (63 labels). The remaining
  ~840 branch labels stay generated Owner_Skip/Loop/Return; they are mechanical but consistent.
- Still open (unchanged, naming-only):
  - the tentative half pipe board/rider record reading;
  - PipeReact*;
  - BmxLift/BmxWheelA/B;
  - SurfRideFlag;
  - the DiskUncaught double use;
  - AwardLayout and the PresetScores medal reading;
  - the round-3 no-caller code;
  - TK-04 (3 bytes Unknown).
