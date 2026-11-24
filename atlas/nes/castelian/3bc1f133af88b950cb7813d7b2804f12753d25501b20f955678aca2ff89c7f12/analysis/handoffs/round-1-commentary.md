# Round 1 commentary handoff

Status: COMPLETE (current-source readability). Editorial audit pending (final
acceptance only). Classification unchanged: 68.00% (89140/131088; 41948 unknown).

## Scope
- Prepared commentary/ via `scripts/commentary.py prepare` from pass-001
  (maps/current.json == maps/pass-001.json). Frozen: commentary/before/,
  baseline.json, phase1-state.json, reading-before.asm.
- All 459 Code regions (banks 2-7), 36 Data regions and all 67 unknown ranges
  covered. review.json: 562 scaffold entries status=reviewed with summaries and
  source locations, 689 units; editorial_audit pending.
- Method: commentary/tools/notes.txt (master annotation record) applied to
  before/game.asm by commentary/tools/overlay.py -> commentary/source/game.asm;
  tools/review.py fills review.json. See commentary/progress.md for the
  maintenance rule in later rounds.

## Verification / artifacts
- `python3 /root/romatlas/scripts/commentary.py verify .` -> ok, byte_identical
  (commentary/verification.json; game.asm sha256 17f96805...e326).
- Reading copy refreshed: commentary/reading-after.asm.
- commentary/progress.md, commentary/uncertainties.md updated.

## Readability assessment
Every routine/entry point has a descriptive name (414) and a block comment
(293 docs; the rest are covered by group docs, e.g. far-call trampolines and the
four sound channels); 768 named branch labels, 137 automatic @loop_/@skip_
labels remain inside documented routines. 279 RAM/register symbols and 64
bank-local zero-page names (bonus stage reuses zero page) replace raw operands;
103 data tables named by consumer. Examples: SwitchPrgBank, RunTowerFrames,
FrameRasterSplit, LagFrameSplit/StartDmcTimer, UpdatePlayer + 15 PlayerState_*
handlers, UpdateObjects, ComputeObjectOcclusion/UploadObjectTiles (RAM tile
copy), MapBoxTest, RenderTowerSlice, PlaySong/Ch1ReadNote, BonusStageEntry.

## Unresolved meanings (documented in source / uncertainties.md)
Enemy state kinds, map cell kind visuals, sound command parameters, bonus fade
purpose, odd UpdateHiScore branch, static-only paths (attract mode, time-bonus
screen, game-over secret, HERO paths).

## Structural questions for redirect (consumer evidence; see unknown notes)
1. Bank 3 $8000-$982F (6192 B): UploadPlayerFrame reads $8000+43*n (n=0..143)
   indexed by frame 0-42 -> player sprite frames; bounded exactly. (L1-01)
2. Bank 3 $9BAE-$A4AD (2304 B) copied by SetupTextScreen as font tiles;
   $AF00-$AF2F lift sprite tiles; $A4AE-$AEFF consumer unknown. (L1-01)
3. Bank 5 $8063-$9FDF: TitleScreen reads nametable/tile/attribute blocks
   covering the whole range ($8000-$817F, $8180-$8DBF, $8DC0-$8F3F, $8F40-$9D5F,
   $9D60-$9D8F, $9D90-$9FDF). (L1-02)
4. Bank 4 $A8BC-$A949 = NotePeriods; $AD38-$B685 sound data (headers $B096,
   pointers $AD38/$AEE8, envelopes $AF6D/$AF7F, modulation $B050/$B058). (L1-04)
5. Bank 4 $B692-$B6B7 is the image of the RAM routine at $06D5 (copied by
   CopyRamTileRoutine $B686): gives the RAM-execution upload origin;
   $B6B8-$B6D7 sprite palettes. $B7A6-$B7E5 packed tiles. (L1-06)
6. Fixed-bank tables with exact consumers: $C9D7-$CA2E, $CC16-$CC5D,
   $CD3F-$CD92, $D90F-$D9D8, $DD08-$DD89, $E261-$E27B, $EA5B-$EA7A,
   $EC9D-$ECB4, $EE4E-$EE5D, $EFB3-$EFD2, $F221-$F27C, $FB15-$FB9D,
   $FEBD-$FEC4 (DMC rate table, not code), $FF12-$FF15. (L1-08/L1-06)
7. Bank 6 $9536-$B98B: palettes $9536/$953D, metatiles $969A/$9713, bonus
   graphics $98F7 (16 pages), fade tiles $A5B7/$A7B7, bonus map pointers
   $A8F7/$A8FE/$A905. (L1-03)
8. Bank 5 $A51B-$A5E1 may hold the bank 5 cell drawers targeted by
   jmp ($01EB)/($0063) ($A533..$A5CB), supporting bank 5 for L1-05.
9. Tower maps are read from ROM: bank 5 $A298/$A2A0 give banks 0,0,0,1,1,1,2,2
   and pages $80/$90/$A0 (16 pages each), i.e. banks 0-2 $8000-$AFFF (already
   data). So the bank 5 $A9E5-$BFFF and bank 6 unknowns are NOT tower maps
   (L1-02 hypothesis weakened); their consumers remain to be found.
10. AttractMode is set by NmiTitle after $0A6E idle title frames (static path
    $FC0E): idling ~2700 frames on the title should exercise L1-07.

## Remaining gaps
None in current-source readability. Classification gaps unchanged (41,948
unknown bytes, leads L1-01..L1-08). Toolkit: TK-005 added (NES data labels
emitted in .reloc mode); no blockers.
