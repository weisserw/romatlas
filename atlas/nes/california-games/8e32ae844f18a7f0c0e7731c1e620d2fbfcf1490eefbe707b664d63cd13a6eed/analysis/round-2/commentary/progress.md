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
