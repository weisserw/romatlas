# Round 2 commentary handoff

Status: COMPLETE (current-source readability on checkpoint pass-002).
Editorial audit pending (final acceptance only). Classification unchanged:
100.00% classified (131086/131088; 2 unknown bytes) - not complete.

## Scope
- Map/state changed vs the round-1 baseline, so the entire round-1
  commentary was preserved at analysis/round-1/commentary/ (identical copy)
  and a fresh baseline was prepared (commentary/before, baseline.json,
  phase1-state.json, 573 scaffold entries).
- Round-1 names/comments/review carried by physical identity (bank + CPU
  address in commentary/tools/notes.txt, re-applied by tools/overlay.py);
  only affected records were rewritten (unknown-range notes, stale
  "unclassified"/L1-05/RAM-origin statements, TowerParamTables blob,
  OccludeMaskTableC bounds). Patch scripts: commentary/tools/r2/.
- All 573 entries reviewed (831 units), incl. the 79 new/changed regions:
  12 code regions named/documented, 65 data regions with format/consumer
  notes and 130 new data labels, 2 unknown-byte notes.

## Readability assessment
Every routine has a descriptive name and block comment; data tables are
named by consumer and their formats explained once at the block. Examples:
CopyRowsBottom2000_Row21, RamTileCopyImage (placeholder operands annotated),
BreakDrawShape0-3/LiftDrawShape0-3 (2-bit column-strip row buffer
explained), RenderTowerSlice_BreakReturn, TowerStartTimer..BreakableListPtrs,
BonusMapTower1-7, PlayerSpriteFrames (column-major frame layout),
TowerPalettes (4 colours x 6 cycle phases). Older backlog fixed: ~240 raw
ROM operands and ~130 raw addresses in comments now use names; misleading
operands corrected (DrawObjects_next,y; OccludeMaskTableC+N). Remaining
numeric operands: only the 8 self-modified $8000 placeholders. 134
automatic @skip_/@loop_ labels remain inside documented routines.

## Verification / reading copy
- `python3 /root/romatlas/scripts/commentary.py verify .` -> ok,
  byte_identical (commentary/verification.json; game.asm sha256 ffddb9fe...).
- Reading copy refreshed: commentary/reading-after.asm.
- commentary/progress.md and commentary/uncertainties.md updated.

## Unresolved meanings
Bank 3 $A500-$AEFF bitmap pairs (no reader); SoundMisc+6 ($0618) role and
the orphan RestartSongIfAllowed/IncExitFlagCapped/PlayerState_Door_Reverse-
Frame; BottomChain2400 entry 0 ($6391); frame-list boundaries $DD4B-$DD58,
$DD7D-$DD82; spare TowerTextPalette byte $A297; plus round-1 items (enemy
states, cell kind visuals, sound command parameters).

## Structural questions for redirect
1. L2-01: bank 3 $B6B0 ($60) / bank 4 $B685 ($EA) - terminal rejection
   review needed; no reference found either way (notes in source).
2. Bank 3 $A500-$AEFF: classified Data from pattern only; any reader (e.g.
   via computed pointer in an unvisited mode) would change its role. A
   bounded watch on reads of $A500-$AEFF during attract/options/text
   screens could confirm "unused".
3. Static-only code $A019/$A0EC, $E4B3, $E585, $F4FD: reachability unproven
   (conf 0.6 for $E4B3/$E585); $F4FD is reachable by the PLAYER 2 WINS
   variant of scenario S13.
4. Bank 4 $9FAC entry 0 ($6391): check whether BottomRows can be 0 when
   CopyRowsBottom2400 runs (bounded trace of BottomRows writes).

## Remaining gaps
None in current-source readability. Classification gap: 2 unknown bytes
(L2-01). Toolkit: no new issues (TK-001..TK-005).
