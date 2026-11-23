# Commentary progress

## Round 1 commentary (2026-11-22) - input checkpoint maps/round01-final.json (sha256 b4a8fa61...)

Prepared with `scripts/commentary.py prepare` (byte-identical baseline, 2265 scaffold entries).

### How the source is produced
- `commentary/annotations.txt` is the durable hand-written commentary record (symbols, routine and
  branch labels, block/line comments, notes for data and unknown gaps).
- `analysis/tools/cmt_apply.py` builds `commentary/source/game.asm` from the frozen
  `commentary/before/game.asm` plus the annotations (bytes untouched; labels, exact-value symbol
  substitutions, comments, incbin splits only). It refuses to overwrite a hand-edited source
  (hash in `commentary/annotations.built`). Edit the annotations, then rebuild and verify.
- `analysis/tools/cmt_review.py` fills `review.json` (scaffold untouched) with status, summaries,
  source lines and routine units. Helpers: cmt_parse.py, cmt_xref.py, cmt_routines.py, cmt_dump.py.

### Scope covered (whole current source)
- All 579 code regions (24,686 bytes, 12,057 instructions): 458 routine/fragment entries labelled
  (all named; a few address-style names such as Disk_B1F8 where the role is unknown), 1,537 code labels total incl.
  generated branch labels (Loop/Skip/Return/Idle suffixes under the owning routine name).
- Every routine has a generated mechanical header (entry kind, fall-through, exit, calls, RAM
  reads/writes by symbol, hardware registers); 300 have hand-written prose blocks, the rest are
  named parts of larger routines described by label + header.
- 99 global RAM symbols, 38 event-scoped overlay symbols (skating, surfing, BMX, half pipe),
  hardware register and controller-mask constants, 27 table symbols, 8 BIT-skip alternate entries.
- All 902 data regions carry the phase-1 evidence class and the routines that reference them.
- All 784 unknown gaps carry a boundary line plus fill/reference notes; 14 gaps with concrete
  evidence (reset code, vector target, indirect-table targets) have explicit notes.
- review.json: all 2265 scaffold entries status=reviewed with summaries/sources; 876 code units.
  editorial_audit stays pending (final acceptance only).

### Key interpretations
Program architecture (NMI-driven frame handlers via FrameHandlerTable, IdleLoop), bank switching
(BankShadow, FarCall, CallBank1-7 BIT-skip chain, per-bank 00-07 bus-conflict tables and $8008 jump
tables), sound driver (8 slots, SongTable/SongTempoTable/SndCommandTable), text-screen/menu system
(BuildTextScreen, MenuFrame, MenuActionTable), registration and results, and the six events mapped to
code (half pipe/foot bag/flying disk/BMX mainly bank 0; skating/surfing mainly bank 7).

### Verification
`commentary.py verify`: ok, byte_identical true, source game.asm sha256 c0b3595f62bc45f1... (analysis/round01-commentary-verification.json).
Reading copy refreshed with `commentary.py read` -> commentary/reading-after.asm.

### Deferred
Event-internal variables outside the scoped sets remain numeric with mechanical descriptions;
branch labels are structural. See uncertainties.md and analysis/handoffs/round-01-commentary.md.

## Round 2 commentary - re-prepared on maps/round02-final.json (sha256 53e08b7b...); round-1 commentary preserved under analysis/round-1/commentary

### Round 2 pass (2026-11-22)
- Map changed (round01 b4a8fa61 -> round02 53e08b7b), so the whole round-1 commentary was moved to
  analysis/round-1/commentary/ (round-1 tool copies in analysis/round-1/tools/) and a fresh baseline was
  prepared (`commentary.py prepare`: byte-identical, 2291 scaffold entries). annotations.txt, progress.md
  and uncertainties.md were carried forward; annotations carry by bank/CPU address and re-applied cleanly.
- Invalidated/rewritten round-1 notes: the "unclassified gap" P-notes for reset, frame handler $18,
  surf states 5-7, sound commands, bank 1/2/3 jump-table targets and menu actions $8C15-$8C21 (now code);
  FrameHandlerTable, SurfStateTable, SndCommandTable and bank 1/2/3 jump-table descriptions; ShowEventScore
  (now known to be the surfing-only score screen); SurfStart split into SurfStart + SurfSetupScreen;
  surf overlay variable scope narrowed to $DBEC-$EBDC (BMX trick bonus code at $EBE8+ was mislabelled).
- New code commented (32 code regions, ~1.3 KB): PowerOnReset, ResetVector, ClearRamAtReset,
  InitHighScoresAndRamCode (warm-boot signature, default high scores, RAM copy routines), menu actions,
  SurfScoreFrame, AdvanceToNextPlayer, DrawLicenceText, DrawSurfScorePicture, BuildSurfScoreBackdrop and
  helpers, 12 course-script commands, sound commands 2/3/7/8/9 and driver split entries, SkateState4,
  SurfState5/6/7, SurfEndRideWait, BmxTrickBonus0-3. Descriptive branch labels for all new routines.
- Backlog: all 44 address-style routine names (HalfPipe_/FootBag_/Disk_/Bank*_) replaced by role names,
  most with prose; bank 1 copy-trampolines renamed. New RAM symbols: FarCallPtrHi, WarmBootSignature,
  RamCopyPlain, RamCopyRle, SndChanMuted. New data symbols: CourseCommandTable, BmxTrickBonusTable,
  LicenceTextStream, SurfPictureStream/Sprites, Backdrop*, BootSignatureText, SndPresetTable, SurfState5Facing.
- Data: round-2 data regions get their lead role text (first occurrence) and short "more of ..." notes;
  generic gap-screen data gets a mechanical note. The 18 residual unknown gaps have decode-for-reading notes.
- Tool fixes (workspace tools only): cmt_apply.py now decides "unknown" by region bounds (a Data region
  with a map label was wrongly printed as Unknown), better phase-1 evidence notes; cmt_routines.py no
  longer lists array bases like `sta $00,x` as BankShadow writes.
- Verification: `commentary.py verify` ok, byte_identical true, game.asm sha256 2ed461a9a774b743...
  (analysis/round02-commentary-verification.json). Reading copy refreshed (commentary/reading-after.asm).
- review.json: 2291/2291 entries reviewed, 981 units; editorial_audit pending (final acceptance only).

## Round 3 commentary - re-prepared on maps/round03-final.json (sha256 aab2fb6c...); round-2 commentary preserved under analysis/round-2/

### Round 3 pass (2026-11-23)
- Map changed (round02 53e08b7b -> round03 aab2fb6c), so the whole round-2 commentary directory was moved to
  analysis/round-2/commentary/ (round-2 tool copies in analysis/round-2/tools/) and a fresh baseline was prepared
  (`commentary.py prepare`: byte-identical, 2293 scaffold entries). annotations.txt, progress.md and
  uncertainties.md were carried forward; carried annotations re-applied cleanly except CallBank1, which is now
  inside a BIT operand (made a constant; CallBank0 labels the new chain start).
- New regions commented: Bank0Entry0, UploadStatusRow2360, StubJumpIdleLoop, SetStatusModeOneAndClear,
  PlaySfxSlot6, StubJumpSurfUpdate, FillTerrainTableForever, CallBank0, StubJumpReset; table tails
  (FrameHandlerTable $87CB-$87CF, BmxInitOam $BE8C-$BEA7, PlayerNameOffsetsB $EEAF-$EEB6, BinToBcdTable 94-99);
  generic data notes for $9B46, b2 $87DF, $F072, $F883, $FFDF. TK-04 notes now state the exact toolkit blocker.
- Corrections from the round-2 redirect: PlayerNames stride 7 (+ PlayerTeams = byte 6), FrameHandlerTable extent.
  Round-2 defect fixed: the $BEA3 and $BFFE gap notes were keyed to wrong physical offsets and silently dropped;
  cmt_apply.py now warns about P notes that match no region/gap start.
- Naming backlog: 28 global RAM names ($07-$09, $28/$29, $39, $3E-$40, $4D-$51, $5C-$5E, $61, $68, $6B-$73, $90),
  24 foot bag overlay names (Kicker*/Bag*), 5 flying disk names; new data symbols TextCursorOam, TeamNameEnds,
  TeamNameText, PlayerNameOffsetsB, BmxInitOam, SndCommandTableHi. Raw zero-page operands 1212 -> 922.
  Foot bag prose rewritten where it was wrong (FootBagPlayerMove -> FootBagStepPose, kick input, jump, bag physics).
- Tool changes (workspace only): cmt_apply.py (unplaced-P warning, no "fill-like" note on 1-byte gaps),
  cmt_review.py (a code region starting at its own label is credited to that label).
- Verification: `commentary.py verify` ok, byte_identical true, game.asm sha256 b70bdbb2f5fe38db...
  (analysis/round03-commentary-verification.json). Reading copy refreshed.
- review.json: 2293/2293 entries reviewed, 951 code units; editorial_audit pending (final acceptance only).
- Remaining naming gaps: see uncertainties.md "Round 3 additions" (event overlay bytes kept numeric).

## Round 4 commentary - resumed on the round-3 baseline (maps/current.json = analysis/state.json = commentary/phase1-state.json = aab2fb6c..., unchanged; no re-prepare)

### Round 4 pass (2026-11-23)
- Applied ready leads L-R3R-01..09 after re-reading each routine (all confirmed; corrections listed in
  uncertainties.md "Round 4 additions"). Renames: Disk* meter/catcher set (DiskThrowMeter, DiskMeterSetPower/
  SetLaunch, DiskReleaseAnim, DiskFlyAndDistance, DiskDrawDisk/DiskDrawPeople, DiskHoldPosition, DiskLeapStep,
  DiskCatcherInput, DiskCheckCatch, DiskUpdateCatcher, DiskFlightUpdate), UploadParallaxStrips /
  Bank6BuildParallaxStrips, FootBagSeagull/GullPickMode/GullFall/GullAdvance, FootBagShowMessageAt, BmxTakeoff/
  AirStep/AirRising/Touchdown/HazardCheck/FinishCourse, BmxBonus{Up,Down}Trick/{Left,Right}Spin, SkateSpawnBall/
  MoveBall/BounceBall/DrawBall/BallOff, SurfResetCarve, LoadBmxRiderTiles (was LoadSurfColumnSet), LoadGullTiles
  (was LoadTerrainStrip).
- Whole-source naming sweep: scoped overlay names 67 -> 299 (S lines), data/table symbols 46 -> 286, plus R spans
  so RAM operands inside named arrays print as Name+offset. Raw RAM operands 922 -> 62 (50 deliberate $00/$01/$02
  base-pair operands); raw ROM-table operands 415 -> ~120 (bank 1 CHR page copies, $8000 bank-select writes,
  a few unlabelled tables).
- Tool changes (workspace only, analysis/tools/cmt_apply.py): R directive (array + offset rendering); data operands
  resolve to a table symbol + 1..3 and, for fixed-bank code reading $8000-$BFFF, to a bank 0 table (with an
  "assumed" note); table symbols inside code are emitted as constants at the end of the file.
- review.json regenerated (cmt_review.py "round 4 commentary"): 2293/2293 reviewed, 951 code units; editorial_audit
  pending (final acceptance only).
- Verification: `commentary.py verify` ok, byte_identical true, game.asm sha256 e12e1cad...
  (analysis/round04-commentary-verification.json). Reading copy refreshed (commentary/reading-after.asm).

## Round 5 commentary - resumed on the round-3 baseline (aab2fb6c..., unchanged)
- Applied L-R4R-01/02: 20 index-offset aliases emitted as Name = Name_XX - $XX, with labels at the first
  real entry. Fixed 5 wrong comments, the CourseObjects Writes names and the BMX no-overlap note.
- 63 descriptive branch labels in HalfPipeFrame, BmxFrame, SurfFrame/SurfAdvanceWave, FootBagFrame,
  PipeBodyStoreAngle, CourseObjects and CourseObjectPalette.
- Verify ok/byte_identical (analysis/round05-commentary-verification.json), game.asm a2b9634c...; reading
  copy refreshed; review.json 2293/2293 reviewed, editorial_audit pending.

## Round 6 (final) commentary - fresh baseline on map 212cccb6... (maps/final-fallback.json)
- Map changed (user-directed fallback: 3 TK-04 bytes became Manual Data), so the whole round-5 commentary
  directory was moved to analysis/round-5/commentary/ (tools copied to analysis/round-5/tools/) and
  `commentary.py prepare` built a fresh byte-identical baseline (2293 scaffold entries). The only baseline
  differences are the two new region headers (unknown-00400e -> region-2509, unknown-01c010 -> region-2510).
- Carried by physical range: annotations.txt (bank/CPU keyed = physical), progress.md, uncertainties.md;
  review.json regenerated with cmt_review.py. Apart from the 2 changed entries, summaries differ only where
  renamed labels, rewritten prose or the newly included P notes changed them.
- Fallback bytes described as generic Data with UNKNOWN meaning (Manual, not a proven role), with the
  observed cross-window "lda $C149,x" and TK-04 explanation kept.
- Editorial: 84 branch labels renamed (SurfBuildWave, NameEntryFrame, FootBagUpdatePose, SkateFrame_Finish),
  stale raw-address prose replaced by symbols, DrawTeamPatterns description corrected, long lines
  reflowed, uncertainties.md brought current. editorial_audit = reviewed (findings in review.json).
- `commentary.py audit`: ok, byte_identical true, 2293 regions, 1237 units reviewed, editorial_audit true;
  game.asm e5b27876...; reading copy refreshed (analysis/round06-final-commentary-audit.json).
