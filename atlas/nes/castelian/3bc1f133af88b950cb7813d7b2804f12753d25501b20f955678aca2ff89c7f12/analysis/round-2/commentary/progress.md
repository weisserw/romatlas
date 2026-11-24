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
