# Round 4 - commentary handoff

Status: **complete**. All currently classified source (pass-004) meets the naming and comment
standard. The final editorial audit was not run, because it is reserved for final acceptance.
review.json editorial_audit is still pending.

## Input and preservation
- maps/current.json and analysis/state.json (pass-004, sha256 35ed6454...) differed from the
  frozen commentary baseline (pass-003).
- Before preparing, the entire round-3 commentary directory was copied to
  **analysis/round-3/commentary/**. A recursive diff confirmed the copy was identical.
  analysis/round-1/ and analysis/round-2/ are untouched.
- A fresh `commentary.py prepare` on pass-004 gave ok and byte_identical, with 1130 scaffold
  regions.

## Carry-over result
- before/game.asm differs from round 3 only where region 1016 was split (bank 7 $C86D-$CDC2,
  plus new region 1131 at $CDC3-$D0C3). before/mapping.json also has new evidence text for
  regions 991, 998, 1016 and 1064.
- tools/ (annotations.txt, build.py, review.py, ...), progress.md and uncertainties.md were
  copied over, followed by `build.py --force` and review.py.
  - The rendered source equalled round 3 except for the phase-1 notes of those regions.
  - review.json matched round 3 in every other region's status, summary and units.
- **Invalidated and rewritten:**
  - the FontTablesTail record, which crossed the new boundary;
  - the generic notes for regions 1064 and 998, which turn out to be panel window rectangles;
  - every comment that depended on these: WindowFrameTable, WindowTileSets, OpenWindow*,
    UploadWindowTiles, PowerMeterPanel, SpinPanel, the NameCharMap constant and
    AsciiTileTable.

## L-012 applied (all seven items, plus the SpinPanel addendum)
1. **FontTablesTail split** into:
   - **NameCharMapTail** ($CDAE-$CDC2): NameCharMap codes $6B-$7F. This is a pattern-only
     reading and is stated as such.
   - **PowerPanelTiles** ($CDC3-$D0C3): window tile set 3, 48 tiles.
2. **Window tables corrected:**
   - The WindowFrameTable and WindowTileSets headers now give the real formats: 7 x 4 bytes
     each, with tile-set records of (source, bank, unused 0).
   - **WinTilesCount -> WinTilesBank.**
   - The headers of OpenWindowBanked (A = request bits, X = frame record), SetWindowTileFlagA/B,
     UploadWindowTiles, OpenWindow, OpenWindowAndDraw and OpenFramedWindow(Draw) are
     rewritten. The RAM comments for WinParams, WinTilesPtr and WinTileFlagA/B are updated.
   - All seven tile sets are labelled at their start:
     - WindowTiles0 (bank 1)
     - WindowTiles1 and PowerPanelTiles2 (bank 4)
     - SpinPanelTiles and WindowTiles2 (bank 6)
     - SpinPanelTiles2 (bank 5 $8000)
     - PowerPanelTiles (bank 7)
3. **PowerPanelRect/PowerPanelAttrs** ($8A76/$8AF8) are labelled, as are **SpinPanelRect/
   SpinPanelAttrs** ($9043/$9095, static). In both panel routines, the two immediate byte loads
   that build each address now render as `#<`/`#>` pairs.
4. **PORTRAIT_OPP_LOST** (scowl) and **PORTRAIT_OPP_WON** (grin) are no longer tentative. The
   ShowPortraitExpression header and the PortraitFrame RAM comment were updated to match.
5. **RandomMessageTable entry 0:** RecordGameWin and RandomMessageTable now record the runtime
   confirmation.
6. **ChallengeResult and ChallengeResultTable** are fully documented:
   - six groups of 5 thresholds;
   - THREE BALL is inverted;
   - a Speed Pool ranking note says the meaning is uncertain.

   Its raw GameType compares now use GT_* constants.
7. **GameCmd $12:** the constant **GAMECMD_INSTA_WIN** is used in the GameFrame dispatch and in
   the GAME CONTROL menu's INSTA-WIN confirmation. Each site has a block comment.

## Other defects fixed in previously reviewed material
These were found by scanning the whole source.
- **Script pointers:** 25 raw bank-2 script pointer pairs are now symbolic, using 21 new MSG_*
  constants such as MSG_HEAD_GAMES, MSG_GROUP_SOLIDS, MSG_TWO_FOULS_WARNING,
  MSG_BALL_IN_HAND_KITCHEN, MSG_COIN_WAS_HEADS and MSG_BANNER_TOURNAMENT.
- **GameMode / GameType compares:** new GM_TOURNAMENT/CHALLENGE/PARTY/FREESTYLE constants are
  used at all 27 GameMode compares. 15 more chained GameType compares, which gensyms.py had
  missed, now use GT_*.
- **Misnamed routine:** SetMenuScriptAX actually loads a fixed message, so it is now
  **SelectOpponentMissedMsg**.
- **Headers made exact:** DrawPlayerNames, DrawPlayerLabel and DrawScoreLabels (the 8-ball
  group label). The address-derived local label `@goto_goto_99bf` was replaced.
- **Table graphics found inside the bank-7 "font" block:**
  - **OverviewTableMap** ($C8AF, 32 x 30 nametable image)
  - **OverviewTableAttrs** ($CC6F)
  - **ZoomedTableAttrs** ($CCAF)

  Their consumers are DrawTableScreen, LoadZoomedTableNametables, RestoreTableTiles and
  RestoreAttrRow. This also explains the r2-q1 read runs. Also labelled: **ZoomedTableMap**
  (bank 4 $9DAD), **BallMarkerSprite** (bank 4 $B209, static) and **BannerRightCap** ($EA73).
- **Stale references:** two headers still named Far_sub_5_AEAD (now Far_UpdateHud). Raw
  routine addresses in comments are replaced by PhysicsStep, AnimateBallsStep and LaunchShot.
- **Fill blocks:** three small unexplained fill blocks now have notes.

## Readability assessment
The new and changed material reads at the round-3 standard: routine headers, symbolic operands,
and tentative wording where meaning is inferred. Areas that remain shallower are unchanged:
rules details, flag bits, physics constants, and most GameCmd values, which stay numeric with
comments because their meanings are partly inferred. See commentary/uncertainties.md, which has
a new round-4 section.

## Remaining generic-name counts
| Kind | Count |
|---|---|
| Address-only labels (sub_/loc_/unref_/tbl_) | 0 |
| Address-only RAM names (ram_/zp_) | 0 |
| @skipN | 0 |
| @join / @if_* | 0 |
| Address-suffixed local labels | 0 |
| @loop / @return (structural) | 297 |

There are 701 regular labels, all named, over 21,478 instructions. review.json has 1130/1130
regions and 642 units reviewed.

## Verification and reading copy
- `commentary.py verify`: ok true, **byte_identical true**, run after the last source build
  (commentary/verification.json).
- `commentary.py read`: refreshed **commentary/reading-after.asm**.
- input/game.nes, the classification map (maps/, analysis/state.json) and the toolkit were not
  modified.
- analysis/leads.json: L-012 is now status verified, with a round-4 history entry.
- Progress is in commentary/progress.md (round 4 section).

## Questions for redirect (optional, low value, commentary-only)
1. **Speed Pool ranking:** is the Speed Pool total a time? ChallengeResult ranks a higher total
   as better, so a total of 7 gives NOVICE. A natural Speed Pool clear with a known total would
   settle it.
2. **Spin panel attributes:** a read trace of the SET SPIN panel (aim, open GAME CONTROL > SET
   SPIN) would confirm SpinPanelAttrs' length and whether the $55 run at $909B-$90BA is read.
3. **BallMarkerSprite bank:** a short read trace while the ball marker is drawn would confirm
   bank 4.

## Disk and processes
- No emulator or server processes were started.
- The scratch directory /tmp/ra-2707-c4 (including the temporary moved copy of the round-3
  commentary) was removed. The preserved copy is the one in analysis/round-3/commentary/
  (9.2 MB). About 2.5 GB is free.
