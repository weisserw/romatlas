# Round 3 - commentary handoff

Status: **complete**. All currently classified source (pass-003) meets the naming and comment
standard. The final editorial audit was not run, because it is reserved for final acceptance.
review.json editorial_audit is still pending.

## Input and preservation
- maps/current.json and analysis/state.json (pass-003, sha256 2d5ac8c3...) differed from the
  frozen commentary baseline (pass-002-final, 265dcf1d...).
- Before preparing, the entire round-2 commentary directory was copied to
  **analysis/round-2/commentary/**. A recursive diff confirmed the copy was identical.
  analysis/round-1/commentary/ is untouched.
- A fresh `commentary.py prepare` on pass-003 gave ok and byte_identical, with 1129 scaffold
  regions.

## Carry-over result: clean
- before/game.asm is identical to round 2. Only before/mapping.json differs: the evidence and
  confidence text of 20 regions. No byte, kind or boundary changed.
- The round-2 tools/, progress.md and uncertainties.md were copied over, followed by
  `build.py --force` and review.py.
  - The rendered source was identical to round 2.
  - review.json matched round 2 in every scaffold status, summary, source location and unit
    (1129/1129 regions, 623 units).
  - No record was invalidated.
- All 20 changed regions are code, so no generated data-block "Phase-1 basis" note quotes them.
  Source comments that state the refreshed facts were updated:
  - DrawBigNumber $F9A9: its five callers are in DrawScoreboardRow, and it has been seen only
    under the artificial-state shortcut.
  - The $DE1C @unused_prologue note and the VramBufFlushNow header now give the stack contract:
    the routine pushes 2 bytes and pulls 3, so A must be pushed before entry.
  - AimAtCalledBall: the bank-5 $8A3D-$8A48 tail is reached by the normal return from
    SelectBank5_Temp, called every frame in GameCmd 5.
  - S14 "natural" wording: no source comment quoted it.

## L-010 applied (commentary/tools/annotations.txt, rendered to source)
1. **NameCharMap** ($CD43) is the character map of window frame style 0 (WindowFrameTable
   record 0, bytes 2-3).
   - The bank-7 font block is now split into labelled sub-blocks:
     - BigFontBottom: 26 bytes, $CD49-$CD62.
     - **NameCharMapChars**: $CD63-$CDAD, character codes $20-$6A. Codes $60-$6A are the
       window frame pieces.
     - **FontTablesTail**: $CDAE-$D0C3, 790 bytes. No reader is known, and the r2-q1 trace read
       nothing past $CDAD.
   - AsciiTileTable now notes its separate read runs.
   - RunMenu's four `#$43/#$CD` literals now render as `#</#>NameCharMap`. They are documented as
     dead stores, because both drawers reload DstPtr from MenuFrameTiles.
   - The MenuFrameTiles and WindowFrameTable comments now explain how these pieces link up.
2. **Credits.** The DrawCreditsPage1/2, DrawMenuBodyWithOffset and DrawMenuBody headers now say
   that menu op 07 sets NameCharMap and tile offset $80. The entry values do not matter. The
   DrawMenuBody opcode list has been checked against the code: 07, 13, 15, 16 and 17 were added,
   and the nonexistent 14 was removed.
3. **PortraitFrame.** New constants PORTRAIT_NEUTRAL, PORTRAIT_OPP_LOST (1) and PORTRAIT_OPP_WON
   (2) are used at all 8 writers.
   - These meanings remain **tentative**: they are inferred from the message texts, and the
     pictures were never captured.
   - New constant MSG_OPPONENT_WINS_MATCH ($9CE4) replaces a raw pair in EndOfMatch.
4. **Correction to finding 4.** The routine is **RecordGameWin** (formerly PickRandomComment).
   - It shows the comment, credits PlayerWins and runs EndOfMatch.
   - RandomMessageTable entry 0 *is* chosen, whenever Random & $1F = 0 (1 time in 32). The
     redirect note overlooked that path. This is a static reading only.
5. **Delay path renamed throughout:**

   | Old name | New name |
   |---|---|
   | DelayThenEnding | RunFramesBeforeMessage |
   | WaitFramesThenEnding | RunFramesLoop |
   | EndingScreenLoop | DelayReturn (bare rts), with @disabled_countdown at $B9AC |
   | EndDelay | FrameDelay |

   The ShowMessage header was also wrong: it does not close the menu, it holds the screen for
   120 frames with GameCmd $1B (scoreboard only).
6. The RunMenu $2000/$2001 note now says the reads return the PPU I/O latch, not open bus.
7. The $DE1C stack contract is documented (see above).

**Further defects fixed** in reviewed material near these routines:
- DrawFullScreenMenu was really the framed (windowed) menu drawer, so it is now
  **DrawFramedMenuBody**.
- MenuLineTopBorder builds an empty line with side edges, so it is now
  **ClearMenuLineBordered**.
- The RunMenu header was corrected.
- The EndOfMatch header now covers all four callers and both ways a match can end.

## Readability assessment
The new and changed material reads at the round-2 standard: routine headers, symbolic operands,
and tentative wording where meaning is inferred. Areas that remain shallower are unchanged
(rules details, flag bits, physics constants). See commentary/uncertainties.md, which has a new
round-3 section.

## Remaining generic-name counts
| Kind | Count |
|---|---|
| Address-only labels (sub_/loc_/unref_/tbl_) | 0 |
| Address-only RAM names (ram_/zp_) | 0 |
| @skipN | 0 |
| @join / @if_* | 0 |
| @loop / @return (structural) | 297 |

There are 684 regular labels, all named, over 21,478 instructions. A few names keep a hex
suffix because their role is only partly known (e.g. ShotStat0590, GameFlagsC3).
review.json has 1129/1129 regions and 625 units reviewed.

## Verification and reading copy
- `commentary.py verify`: ok true, **byte_identical true**. This was run after the last source
  build (commentary/verification.json).
- `commentary.py read`: refreshed **commentary/reading-after.asm**.
- input/game.nes, the classification map (maps/, analysis/state.json) and the toolkit were not
  modified.
- analysis/leads.json: L-010 is now status verified, with a round-3 history entry.

## Questions for redirect (optional, commentary-only; no structural change proposed)
1. **Portrait frames 1/2:** a visual capture would confirm the opponent-lost / opponent-won
   expressions. It needs a natural game loss or win that reaches EndOfMatch.
2. **FontTablesTail** ($CDAE-$D0C3) and the extra read runs inside AsciiTileTable: are any of
   them reached through another base? A bank-7 data-read trace during gameplay text, e.g. the
   status line or the big-font banners, would show this. Leaving them as generic data is
   acceptable.
3. **RecordGameWin entry-0 claim:** a quick runtime check could confirm it. For example, force
   RandomLo & $1F = 0 right after the `jsr Random` in RecordGameWin during a tournament win. This is low value.

## Disk and processes
- No processes were started.
- The scratch directory /tmp/ra-2707-c3 (including the temporary moved copy of the round-2
  commentary) was removed. The preserved copy is the one in analysis/round-2/commentary/.
- commentary/ and analysis/round-2/commentary/ are about 9.3 MB each. About 2.5 GB is free.
