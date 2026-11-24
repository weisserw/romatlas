# Commentary progress

Round 1 commentary (checkpoint pass-001) is preserved in full under
analysis/round-1/commentary/ (source, review.json, notes, tools, reading
copies, verification). Its progress record is
analysis/round-1/commentary/progress.md.

## Round 2 commentary pass (2026-11-24) - COMPLETE for current source

Baseline. maps/current.json and analysis/state.json (sha256 9a7a6642...7f0f5,
= pass-002) differed from the round-1 frozen baseline (21a4a6bf...a946), so
the whole round-1 commentary/ was copied to analysis/round-1/commentary/
(diff -r identical) and `scripts/commentary.py prepare` produced a fresh
verified baseline: before/, baseline.json, phase1-state.json, 573 scaffold
entries (571 regions + 2 unknown bytes).

Carry-over method. Round-1 annotations are keyed by 16K bank + CPU address
(physical identity) in tools/notes.txt, so they were re-applied by
tools/overlay.py to the new before/game.asm. All round-1 labels, symbols,
docs and data notes landed on the same physical instructions (0 mismatches;
the only boundary change, $F4ED split into $F4ED-$F4FC / $F4FD, keeps
the labels). Invalidated and rewritten: the 67 round-1 unknown-range notes
(now data/code), stale statements (unclassified $909D/$9864, $B692, L1-05
bank, RAM routine origin, title dnote, "never observed" attract mode), the
TowerParamTables blob, OccludeMaskTableC size (27 -> 13: the player frame
lists start at $DD4B). tools/r2/ holds the round-2 patch scripts
(patch_notes.py, round2-src.txt -> build_round2.py, names_in_docs.py).
Maintenance rule unchanged: edit tools/notes.txt, run tools/overlay.py,
tools/review.py, then verify.

Scope covered: every scaffold entry (573) incl. 79 new/changed regions.
- New code named and documented: CopyRowsBottom2000_Row21 / 2400_Row21
  ($909D/$9864, entry 11 of the bottom-row RTS tables), SoundApi_RestartSong
  + RestartSongIfAllowed ($A019/$A0EC, no caller), RamTileCopyImage ($B692,
  operand placeholders explained per instruction), BreakDrawShape0-3 /
  LiftDrawShape0-3 (bank 5 $A533-$A5E1), RenderTowerSlice_LiftReturn /
  BreakReturn ($F11A/$F197), PlayerState_Door_ReverseFrame ($E4B3, orphan),
  IncExitFlagCapped ($E585, orphan), the P1/P2 result branch $F4ED/$F4FD.
- New data: a boundary/format/consumer note for every new Data region (65)
  and 130 new data labels, e.g. PlayerSpriteFrames, TextFontTiles,
  TowerPalettes, Option* tables, SlotOrderTable, AttractDemoInput, Title*
  blocks, per-tower tables TowerStartTimer..BreakableListPtrs (replacing
  TowerParamTables+N), BreakPixelRamp, BonusMapTower1-7, Bonus* graphics/
  palette/path tables, bottom-row dispatch and VRAM tables, RamCopy* pass
  and destination tables, door/turn/fire/knockback frame lists, edge masks,
  shared projection tables ColumnLayoutA/B, EdgeBufferOffset/PatternIndex.
- Backlog fixes in older material: ~240 raw ROM operands replaced by labels
  (only the 8 intentional $8000 placeholders in RamTileCopyImage remain),
  ~130 raw table addresses in older block comments replaced by the names,
  pointer immediates for title tiles and the demo script symbolised,
  attract-demo zero-page reuse named (DemoStepIndex/DemoStepTimer),
  misleading operands fixed (DrawObjects_next,y -> ObjSpriteOamOffset-4,y;
  OccludeMaskTableC+N -> DoorTurnOutFrames*-$24,x), Ch3/Ch4_Command labels,
  sound API doc lists the $A019 vector.
- Unknown bytes: notes for bank 3 $B6B0 ($60) and bank 4 $B685 ($EA).

Numbers: 233 data labels, 101 data notes, 305 block docs, 2 unknown notes,
134 automatic @skip_/@loop_ branch labels (all inside documented routines),
review.json 573 entries reviewed, 831 units; editorial_audit pending.

Verification (handoff): `python3 /root/romatlas/scripts/commentary.py
verify` -> ok, byte_identical (commentary/verification.json, game.asm sha256
ffddb9fe...). Reading copy refreshed (commentary/reading-after.asm).

Unresolved / deferred: see uncertainties.md (round 2 section). No
current-source readability backlog remains; editorial audit is reserved for
final acceptance.

## Round 3 commentary pass (2026-11-24) - COMPLETE for current source

Baseline. maps/current.json / analysis/state.json (9baf6bef..., pass-003)
differed from the round-2 baseline (9a7a6642...), so the whole round-2
commentary/ was copied to analysis/round-2/commentary/ (diff -r identical)
and `commentary.py prepare` made a fresh verified baseline (601 scaffold
entries = 599 regions + 2 unknown bytes). tools/, progress.md and
uncertainties.md were carried; tools/overlay.py re-applied notes.txt by
bank + CPU address: every round-2 label/doc/symbol landed on the same
instruction (diff vs round-2 source = only the 37 new provenance region
headers/.org/.reloc). No classification changed (L2-02 provenance only).

Affected records (7 Static->Observed regions, 9 split regions):
- Stale statement fixed: $F4FD inline note (was "static; not seen in
  traces", now PLAYER 2 WINS executed in RAM-patched scenario S13b).
- CheckExtraLifeHero documented (10000-point counting, static-only award
  block $CE7F-$CEB0 explained; 5000/10000 operands as #</#> constants).
- Split points checked: all fall on instruction starts inside documented
  routines; labels/local scopes unchanged. Notes added where the split is
  informative (bottom-row chains $9439/$9C00 = row 15, AwardClimbPoints
  $A514 high-byte carry). The listing preamble now explains that a region
  header inside a routine is a provenance split, not a routine boundary.
- Bottom-row chain docs corrected: rows 20 down to 11 (was "to 12");
  CopyRowsBottom2400_Chain got its own doc.

Whole-source readability sweep (older backlog / concrete defects):
- ~70 raw ROM addresses in prose replaced by existing names
  (tools/r3/addr2name.py, addr2name2.py); this exposed and fixed wrong
  round-1 descriptions: game-logic phase (ScoreToDigits/UpdateLiftReturns
  were described as "object updates"/"player movement"), gameplay NMI
  ("sound" was TickTowerTimer/DrawScoreHud), PlayerState_Walk (slope
  handling was described as door input), SetupTower, ShowGameOverScreen,
  sound API table, bonus-door tables.
- Address-named trampolines renamed: FarCall_B4_PlaySong/Silence/
  EffectSq1/EffectSq2/EffectNoise, FarCall_B5_DrawTimerHud/TickTowerTimer/
  DrawScoreHud/ScoreToDigits, FarCall_B3_DrawTowerNumberHud_PrevTower,
  FarCall_B3_UpdateCamera; PlayerState_Door_setEnterFrame.
- Raw operands named: SoundSq1VolShadow ($0661), APU_UNUSED_400D,
  TowerUnused01F0 (write-only); $0214/$0215 title clears annotated. Only
  the 8 intentional $8000 placeholders and jmp $0200 remain numeric.

Numbers: review.json 601/601 reviewed, 857 units; editorial_audit pending.
Verification: `commentary.py verify` -> ok, byte_identical (game.asm sha256
063b8aa6...); reading copy refreshed (reading-after.asm).
Deferred: none in current-source readability. Unresolved meanings:
uncertainties.md (round 2 + round 3 sections).

## Final commentary pass and editorial audit (2026-11-24) - COMPLETE, AUDIT PASSED

Baseline. maps/current.json / analysis/state.json (c6868c2f..., pass-004)
differed from the round-3 baseline (9baf6bef...). The whole round-3
commentary/ was copied to analysis/round-3/commentary/ (diff -r identical);
the working copy was moved aside to scratch/commentary-r3-working (also
identical) so `commentary.py prepare` could build a fresh verified baseline
(601 entries; the only scaffold change is unknown-00f6c0 / unknown-013695 ->
region-1966 / region-1967, kind Data). tools/, progress.md and
uncertainties.md were carried; before/game.asm differs from round 3 only by
the two new "Data; Manual" region headers.

Affected records: the two former unk notes became dnotes stating generic
data, meaning unknown, manual user-directed fallback (not evidence-backed),
with the L2-01 code-vs-pad ambiguity; the listing preamble explains Manual
provenance (and no longer says round 2 / pass-002). Their review.json
summaries are set by tools/r4/fallback_summaries.py (run after review.py).

Final editorial review (concrete inconsistencies only; prior review credit
reused): $0614 = SoundTempo (was LowTimeCounter / SoundChannelBusy+3;
TickTowerTimer doc fixed), SoundChannelBusy 3 bytes, SoundMisc replaced by
SoundTempoCopy ($0615) and SoundRestartFlag ($0618); ObjFrame 5 bytes so
ObjFrame+5,y -> ObjDrawFrame,y; ObjTimer 4 bytes so ObjTimer+4 -> ShotTimer;
BoxAngle (lsym) in the map box test instead of OcclScreenX; removed unused
DemoIndex/DemoTimer/ShotHitSlot; Ch1/Ch2_Command and @unhandledCommand.

Numbers: review.json 601/601 reviewed, 857 units; editorial_audit reviewed.
`commentary.py audit` -> ok, byte_identical, 601 regions, 857 units
(commentary/audit.json; game.asm sha256 65b0b106...c451c); reading copy
refreshed. No current-source readability gaps remain.
