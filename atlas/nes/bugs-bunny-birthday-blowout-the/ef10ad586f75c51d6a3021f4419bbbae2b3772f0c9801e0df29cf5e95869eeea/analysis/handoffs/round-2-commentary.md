# Round 2 commentary handoff

STATUS: COMPLETE. The current-source readability pass is finished, a fresh byte-identical
verification passed, and the reading copy has been refreshed. ROM classification is
unchanged at 100.00% rounded: 262157/262160 bytes, with 3 bytes still unknown, so the ROM
is not fully classified. The final editorial audit has not been run; it is kept for
archive acceptance.

Input checkpoint: pass-002 (maps/current.json = analysis/state.json, sha256 88ab22c6...).

## Baseline handling
- The map changed from pass-001, so the whole round-1 commentary directory was copied to
  analysis/round-1/commentary/ (checked identical with `diff -r`, 7.8 MB). After that,
  commentary/ was re-prepared with `commentary.py prepare` from pass-002: new frozen
  before/, baseline.json, phase1-state.json and a 993-entry review scaffold.
- Carried over: tools/, notes/, progress.md, uncertainties.md. Notes are keyed by bank
  and CPU address, which fixes the physical range, so each name and comment re-applies
  to the same bytes. annotate.py reports 0 misplaced directives. The only records
  invalidated were the round-1 "unclassified block" notes for the ranges that pass 2
  newly classified (L-001/L-002/L-003 in banks 9, 14 and 15) and the $A024 code-as-data
  note. All of these were rewritten.

## Scope
- New regions, all commented: L-007 Type12HitMotion / Type00MotionScript; L-008
  BlinkArrowSprite, BlinkArrowSprite3, ClearPanelsRect; L-009 WaitButtonBlinkingReset;
  L-010 ScrollTransitionAndFadeIn, InitMirroringVertical, ApplySplitScroll,
  ClearNametable0/1AndShow, FillAttrBuffer0/1Only, RequestAttr0/1Upload,
  EnableWramWriteProtected, DisableWram, $FFF6 vector padding; L-011 DrawInlineScreen2400,
  DecodeRleStream2, PutScreenByte2; L-012/L-003 the stale duplicate of
  StageMusicTable + ClearObjectSlots0500 and the $FF fill; L-013 the vertical
  tile-column writer and attribute-rectangle painter ($F411-$F5D8, 12 routines) plus
  AttrPaletteBits and AttrQuadrantClearMasks; L-014 StageSelectLimits; L-015 notes on the
  3 unknown bytes.
- Backlog in carried source:
  - Round 1's annotate.py never applied the WRAM symbols. This is now fixed, and 85 RAM
    symbols were added or resized. Raw RAM operands went from 788 to 8 (the 8 left are
    Reset's deliberate page addresses).
  - 172 routines of 20 bytes or more that had only a name now have header comments.
  - 62 pointer pairs now read `#<Label/#>Label`. PPUFLAG_* and ROUND_STATE_* constants
    are now used in operands.
  - 20 data blocks that pointers aim at have been given names.
  - Corrections: the ApplyMotionScript record format; ShowMessageAt renamed
    AwardLives; SpecialObjectProbe/Reset renamed SpecialObjectWriteTiles/DrawFrame;
    MediumToSmallA/B (removed the misplaced Tbl_09_AF61); the metasprite size wording;
    stale "unknown" notes.
- Still name-only: 278 short helpers or secondary entries (under 24 code bytes) rely on
  their descriptive names. The prompt allows this ("need not paraphrase every
  instruction").

## Verification
- `python3 /root/romatlas/scripts/commentary.py verify .` returned ok=true and
  byte_identical=true. The game.asm sha256 is 4dc10a2d..., recorded in
  commentary/verification.json.
- The reading copy was refreshed with `commentary.py read`: commentary/reading-after.asm
  (27553 lines).
- input/game.nes is unchanged (sha256 ef10ad58...).
- review.json has 993/993 scaffold entries reviewed and 1360 units. editorial_audit is
  still pending, by design.
- Workflow: edit commentary/notes/*.txt, then run `python3 commentary/tools/annotate.py`,
  then `python3 commentary/tools/review.py`, then verify and read.

## Examples
- `PaintAttrRect`: sets the palette for a rectangle of 16x16 areas in AttrBuffer0/1, using
  VramAddrToAttrCell plus the AttrQuadrantClearMasks and AttrPaletteBits tables.
- `StageSelectLoop` ... `cmp a:StageSelectLimits,y`. `DrawBlinkingArrow` now uses
  `lda #<BlinkArrowSprite` and the `BlinkCounter` alias.
- `GameInputStep` uses CursorPos, TargetHeight, HitPending and InputDelay where it
  previously had raw $7A8x addresses.

## Unresolved meanings
Details are in commentary/uncertainties.md:
- Which character or enemy each object type is; boss behaviours A-F.
- What the map-drawn "special object" ($0510-$051B) and the diagonal tile sweep look like
  in the game.
- The bonus-game prize families (MediumToSmallA/B is a tentative reading).
- Several write-only variables.
- Whether the pass-2 code variants (unused vertical mirroring, $2400 RLE screen, column
  writer, WRAM protect/disable, split scroll) are ever reached. No caller is known.
- WrapColumnToTop EORs the low address byte. This looks like an original bug in unused
  code.

## Structural questions for redirect (evidence-policy review needed)
1. L-015: the 3 unknown bytes $C43F-$C440 (`01 00`) and $C587 (`60`). Commentary found no
   new consumer. Reopen only if a caller or consumer turns up, for example from a trace
   of the PutMetatileInColumn neighbourhood.
2. Reachability of the pass-2 orphan code (L-009..L-013) and the hidden stage select
   (pair 6 entry 8). A bounded trace or watch on those entry points during play could
   confirm whether they are dead. They are useful as artificial-call test targets.
3. Boss coverage (L-005/S8) remains deferred. Names like BehaviourA-F would improve if a
   boss fight could be recorded.

## Remaining gaps
None for current-source readability. Two gates are separate and not done here: the final
editorial audit, and whole-ROM classification (3 bytes unknown).

## Toolkit
No new toolkit issues. The WRAM-symbol and pointer-immediate fixes were made in the
workspace-local annotate.py, not in the toolkit.
