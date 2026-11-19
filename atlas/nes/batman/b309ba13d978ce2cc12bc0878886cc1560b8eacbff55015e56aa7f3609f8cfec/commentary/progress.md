# Commentary progress

## Method (round 1)
source/ is generated ONLY by `python3 commentary/tools/annotate.py` from the frozen
before/game.asm plus the commentary database in commentary/notes/:
bankNN.txt (labels, block/inline comments, unknown-stretch notes), bankNN.head
(bank file headers), ram.txt (global RAM names), ram_cutscene.txt (RAM names used in
banks 2/3/5), shallow.txt (routines still needing branch-level review; empty now).
Edit the notes, rerun annotate.py, then
`python3 /root/romatlas/scripts/commentary.py verify <workspace>` and `... read`.
`python3 commentary/tools/review.py` refreshes review.json units/status (scaffold
intact). Reading aid: `python3 commentary/tools/compact.py BANK` (scratch listings).

annotate.py: bank-organized files (bank00-15.asm, chr.asm, symbols.asm), labels for
every resolved branch/call/data target, symbolic RAM/hardware/MMC3 operands (values
unchanged), jump/handler tables and inline arguments as `.word Label`, per-routine
headers (summary + callers/calls/RAM read/written/sounds), notes on every unknown
stretch (33 stretches flagged as read-as-data by known code), UNPLACED-label check.

## Round 1 pass (2026-11-19)
- Scope: all classified source, banks 0-15 + header/CHR/vectors.
- Every routine (437 subs) has a descriptive name and a header comment; 1,555 named
  labels in notes; 811 remaining branch labels are routine-scoped mechanical names.
- RAM: ~200 named variables/arrays (gameplay, object arrays, sound driver arrays,
  cutscene overrides); hardware/MMC3 registers symbolic.
- review.json: 2071/2071 scaffold entries reviewed (routine level), 493 units.
  editorial_audit left pending (final acceptance task).
- Verification: commentary/verification.json byte_identical=true (see handoff).
- Deferred / remaining: branch-level descriptive names for the 811 mechanical
  labels (largest groups: bank 4 player/streaming 335, bank 13 bosses 253, bank 15
  boss handlers 122); data formats of area banks; object kind identities.

## Round 1, resumed session #2 (2026-11-19)
- Scope: the 950 mechanical branch labels in 183 routines (banks 0, 2, 3, 4, 5, 8,
  12, 13, 14, 15) all renamed descriptively via staged notes merged with
  tools/merge_notes.py; short block comments at decision points. 0 remain
  (tools/remaining.py regenerates remaining-branch-labels.txt).
- Renames: ObjKind33Jump->ObjKind33Dash, ObjKind33Dive->ObjKind33Leap,
  Cs5_EndingPart->Cs5_EndingFigureFrame, Cs5_EndingScroll->Cs5_EndingFigureSlide,
  CsSplitFlag->CsSplitChrA; new Cs3_SceneEnd, scene-part locs (CsStage2Intro_Part2/3,
  CsStage5Intro_Part2, CsEnding_Part2, CsEnding_Final), JmpPlayerJumpCheck.
- New RAM names: Boss33* ($70-$73), CsEndingFlag, DmcStreamHold, AttrColumnSave and
  cutscene $0433/$0450-$0452/$045A/$045C/$04D3-$04D7. Stale Sub_ references fixed;
  inline comments for raw cross-bank operands and adjacent-array aliases.
- New redirect questions R-06 (bank 13 $BCB4 jump target left unknown), R-07
  ($04F0-$04FF layout).
- review.json 2071/2071 reviewed, 496 units reviewed; editorial_audit still pending.
- Verification: commentary/verification.json ok=true, byte_identical=true; reading
  copy refreshed (31,810 lines).

## Round 2 (2026-11-19) — fresh baseline from pass-002
- Map changed (pass-001 -> pass-002), so the whole round-1 commentary/ was copied to
  analysis/round-1/commentary/ (plus scratch listings in analysis/round-1/scratch-commentary/)
  and `commentary.py prepare` built a fresh baseline (before/, baseline.json, review.json
  scaffold of 2,183 entries). tools/ and notes/ were carried over; because notes are keyed
  by bank + CPU address (physical identity), names/comments carried by physical range.
  Carry check: every round-1 label keeps its name except deliberate renames (below).
- Tooling: annotate.py also loads notes/bankNN_*.txt and maps the area-map blocks of banks
  6/7/8/9/11 through the $A000 window (AREA_BLOCKS); tools/region_notes.py writes
  bankNN_regions.txt (role notes for the 294 data/text regions newly classified in pass-002,
  one note per run of same-kind pieces); tools/area_labels.py writes bankNN_areamaps.txt
  (MapNNMetatiles / MapNNScreenCells labels from redirect L-014); review.py takes data/unknown
  summaries from the region notes.
- New code named and explained: ObjKind42Defeated (bank 13 $BCB4, with branch labels and
  ObjKind42DefeatStepTimes/Sprites), ObjKind1ImpactAnim (bank 13 $A000), QueuePpuBlock
  ($C502), orphan routines CsMoveGroupUp, CsSetGroupPos1-3, CsReverseOamOrder, CsGetTextPos,
  CsQueuePokeAt, CsAnimateCommitFinish, CsReadJoypadsAlt/CsJoyStrobeRead/CsJoyMergeBits,
  ApplyScrollSameBanks, Cs5_BlinkScreen/Cs5_ToggleDisplay/Cs5_ClearBothNametables,
  SetupBossSlotsArea17, ObjKind9Fall_placeAt.
- All 85 remaining address-based data labels (Dat_NN_XXXX) replaced by role names from their
  consumers (kind 20/21/23/30/32/37/42-45/33-35 tables, boss set-up and boss-type-5 script
  tables in bank 8, MetaspriteTilePtrs/Sizes/Attrs in bank 11, ScrollMapPtrs,
  AttrCellTablePtrs, MetatileTablePtrs, ScreenCellTablePtrs, AreaStartPtrs/AreaStartRecords,
  SpritePalette3Fixed, MapCursorMoves, PoseSpriteOffsetX ...). 0 Sub_/L_/Dat_ labels, 0
  mechanical branch labels remain.
- Backlog fixed: 160 routines that had only the mechanical Callers/Calls header now have a
  purpose line (sound command handlers, cutscene helpers, bank-4 streaming helpers, boss
  behaviour states, fixed-bank vectors). AreaIntro's boss set-up branches were scoped
  under AddStepToY/BossType4Rows (renamed AreaIntro_bossType*). JumpToBank6_9BB0 ->
  JumpToDrawBatman (its caller CommitBatmanPosition was missed in round 1; R-03 answered).
  Stale comment symbols fixed (ObjType/ObjVar5C0, MulResult/Mul16, EnvHi, PathLens,
  ItemDropTables, JumpVelocityTables, Tmp0F, Dat_13_BDE5) and "phase-1 unknown" wording
  removed. Format notes added for CsPaletteSets, CsColourTriples, CsAnimScripts,
  CsMetaspriteTable, Cs5_BackgroundTable, Cs5_EndingData, Cs5_CreditsData. Bank heads
  6-11 describe the area-map block structure. RAM: SkipArea16Flag bit6 (controller
  freeze), CsSpriteRecs layout corrected (+2 Y, +3 Y wrap bit).
- review.json: 2,183/2,183 entries reviewed, 608 units reviewed; editorial_audit pending.
- Verification: commentary/verification.json ok=true, byte_identical=true; reading copy
  refreshed (32,679 lines).

## Round 3 (2026-11-19) — fresh baseline from pass-003
- Map changed (pass-002 -> pass-003, provenance only: 17 Static code regions split into
  Observed-artificial / Static runs, lead L-019). The whole round-2 commentary/ was copied to
  analysis/round-2/commentary/ (scratch listings to analysis/round-2/scratch-commentary/);
  `commentary.py prepare` built a fresh baseline (2,192 scaffold entries); tools/, notes/,
  progress.md, uncertainties.md were carried. Carry check before edits: source differed from
  round 2 only in phase-1 region/provenance lines (banks 8, 13, 15); all 2,653 labels and
  459 routine headers carried 1:1.
- annotate.py: regions whose evidence starts "Observed (artificial" (r1-a2 and r2-a1) are now
  headed "Observed, artificial scenario" and their inline provenance notes say natural
  reachability is not claimed (banks 0, 4, 8, 12-15).
- L-019 commentary part: $051F named ExitDelay (ram.txt); AreaExitCheck header rewritten
  (wrong $050F removed; exit tables, boss-flag requirement, delayed exit, area 12/13 special
  cases, 16-frame wait), .checkYBelow renamed .checkAboveLine (it requires Y above the line);
  AreaExitFlags note lists boss/left-exit areas and the respawn-at-exit observation;
  ObjKind42Defeated, BossDefeated, ObjKind39, ObjKind13, ObjKind42Decide and ObjKind42Patterns
  now explain their ExitDelay loads/use. ObjKind33 note corrected (its SkipArea16Flag bit has
  no effect when met in area 16; open question in uncertainties.md). Kind 25/42 notes name the
  areas where r2-a1 met them.
- review.json: 2,192/2,192 entries reviewed, 608 units reviewed; editorial_audit pending.
- Verification: commentary/verification.json ok=true, byte_identical=true; reading copy
  refreshed (32,792 lines).

## Round 4 (2026-11-19) — fresh baseline from pass-004
- Map changed (pass-003 -> pass-004, provenance only: 4 Static code regions split into
  Observed-artificial / Static runs at 0x11717, 0x1DE45, 0x1DE9C, 0x1DEFE; lead L-020).
  The whole round-3 commentary/ was copied to analysis/round-3/commentary/ (scratch listings
  to analysis/round-3/scratch-commentary/); `commentary.py prepare` built a fresh baseline
  (2,195 scaffold entries); tools/, notes/, progress.md, uncertainties.md carried. Carry
  check before edits: source differed from round 3 only in the inline provenance notes of
  the split runs (banks 8, 14); all 2,653 labels and 459 routine headers carried 1:1.
- L-020 commentary part: SkipArea16Flag bit7 documented as "area-16 boss beaten" (ram.txt,
  ObjKind33 note rewritten, AreaCleared / GameFrameLoop_continueGame cross-linked, inline
  note at the skip test); SetupBossSlotsArea17 no longer linked to the skip path;
  ObjKind42Defeated slot-0 X read explained (BossSlotTemplate0); zero page $38/$3E named
  CsQueuePokeOffset / CsJoyStrobeBase (ram_cutscene.txt) with the reset-clear-only note;
  AreaIntro_bossType5Script described (seen at the area-16 start). Uncertainties updated.
- review.json: 2,195/2,195 entries reviewed, 608 units reviewed; editorial_audit pending.
- Verification: commentary/verification.json ok=true, byte_identical=true; reading copy
  refreshed (32,813 lines).

## Round 5 (2026-11-19) — fresh baseline from pass-005, final editorial audit
- Map changed (pass-004 -> pass-005): exactly one new region. id 2265 is Data with Manual
  provenance at file $1C5AE-$1C5AF (bank 14 $C59E-$C59F), formerly Unknown, classified as a
  user-authorized fallback. The whole round-4 commentary/ was moved to
  analysis/round-4/commentary/ and scratch/commentary to analysis/round-4/scratch-commentary/.
  `commentary.py prepare` built a fresh baseline with 2,195 scaffold entries. tools/, notes/,
  progress.md, uncertainties.md and remaining-branch-labels.txt were carried over.
- Carry check: the regenerated source differed from round 4 only at the new region. All 2,653
  labels and 459 routine headers were carried 1:1. A review.json regenerated by tools/review.py
  matched round 4 for every entry except the new region; 114 source locations in bank 14 shifted.
- New region: the ?C59E unknown note was replaced by a !C59E note: generic data of unknown
  meaning, user-authorized manual fallback, not observed and not proven, no label. The header
  reads "data (Manual, user-authorized fallback; meaning unknown)". review.py gives it a
  dedicated summary. uncertainties.md was updated.
- Final editorial audit: see editorial-audit.md. Fixes: the $DE14 game-over call (now
  AreaSvc_GameOver), the GameFrameLoop header and labels .areaHook/.skipSpawn, the $9BA9 address,
  BossBlastSlotBits, CsJoyExpBits1/2, 14 stale numeric aliases, notes for 111 data tables, the
  iNES header note, certainty wording, and review summaries for unlabelled data.
- review.json: 2,195/2,195 entries reviewed. There are 720 distinct units, and 1,322 unit records
  after continuation code regions get clipped records of their spanning routine (the audit needs
  units on every Code entry). editorial_audit is reviewed.
- Verification: commentary/verification.json ok=true, byte_identical=true. `commentary.py audit`
  PASSED (commentary/audit.json: reviewed_regions 2195, reviewed_units 1322,
  editorial_audit true, byte_identical true). Reading copy refreshed (32,967 lines).
