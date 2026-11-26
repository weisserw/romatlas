# Round 2 - commentary handoff

Status: complete. All currently classified source (pass-002-final) meets the naming/comment
standard. The final editorial audit was not run because it is reserved for final acceptance;
review.json editorial_audit is still pending.

## Input and preservation
- The map changed since round 1 (commentary baseline was pass-001-final; maps/current.json and
  analysis/state.json are pass-002-final).
- Before preparing, the entire round-1 commentary directory was copied unchanged to
  **analysis/round-1/commentary/**. A recursive diff confirmed the copy was identical.
- `commentary.py prepare` then built a fresh byte-identical baseline: commentary/before, 1129
  scaffold regions, 100.00% classified.

## Carry-over method and invalidated records
- The round-1 machinery was reused. commentary/tools was copied over, and annotations.txt is
  still the edit record. Its keys are bank:CPU, so each key is a physical offset. build.py
  renders it onto the new before/game.asm, and review.py refreshes review.json.
- A per-byte comparison of the old and new region maps (kind and instruction starts) found
  40 changed ranges. All of them are formerly-unknown bytes. Every annotation key inside or
  next to them was checked:
  - **13 invalidated "UNKNOWN" headers were rewritten:** 2:B502, 3:830E, 3:832B, 3:88A8, 3:8A51,
    3:95F6, 7:D9B0, 7:DC06, 7:DE1C, 7:E3A3, 7:F885, 7:FB32, 7:FC65.
  - **Names contradicted by the new consumers were renamed:**
    - The round-1 "tournament chart" was wrong; this code draws opponent and interviewer
      portraits. Chart* tables and routines became PortraitTilePtrs/RectPtrs/Colors,
      DrawPortraitFrame, LoadPortraitGraphics, DrawPortraitCurrentFrame, and InterviewTalk
      (was WaitButtonWithChart).
    - MatchesPlayed became PortraitIndex; RackNumber became PortraitFrame.
    - PlayerStat1 became PlayerWins and PlayerGames became PlayerTotalScore. Both are
      scoreboard columns. AddGameWon became AddScoreToTotal.
    - TitleSkipWord became IntroCountdown.
    - PlayerLabelScripts became GameTypeNameScripts. Its round-1 note mixed up bank 2 and
      bank 3.
  - **About 20 headers were updated** because callers and installers are now known from the
    decoded scripts, for example the hooks, MultiplySigned, DrawBigNumber and ShowTitleSequence.
    ShowTitleSequence's claim about a Ptr04 return value was wrong and was removed.
  - All other records were carried unchanged, and the result was rebuilt and verified
    byte-identical.
- **Defect fixed in carried material:** 8 DstPtr pointer pairs had been rendered as
  `#<SplitResetAddr`, which is a code label. The value is really a character-to-tile map
  pointer, $C84D = AsciiTileTable-$20, and these pairs now render as `#<(AsciiTileTable-$20)`.
- **Changes to the workspace tools:**
  - build.py has a new `D` tag. It keeps an unreachable fragment as a local label, so the
    L-006 fragments no longer split routines into loc_/unref_ scopes.
  - In build.py, explicit operand overrides now take priority over pointer-pair detection.
    This removed the false label loc_3_9E03.
  - gensyms.py gained the credits and scoreboard MSG_* constants.

## Covered scope
- **Boot intro (L-002), bank 3:**
  - TitleIntro: a star field (40 sprites), a split slide-in, a palette fade and colour cycle,
    and four objects orbiting on ellipses with depth given by sprite priority. It loops until
    the 240-frame countdown runs out or a button is pressed.
  - Helper routines and tables: IntroOrbitInit/Step, IntroStarsStep/Move, IntroOrbitRecords,
    IntroStarTiles, and CosineTable. CosineTable is the "consumer not identified" curve table
    from round 1.
  - The overlay RAM has its own names.
- **Credits (L-001):** DrawCreditsPage1/2 and Far_DrawMenuBodyWithOffset.
- **Portrait screen (L-004, L-005):**
  - Routines: ShowOpponentPortrait, ShowPortraitExpression and PrintOpponentName, plus
    PrintString in bank 7 at $E3A3.
  - Tables: the 11 opponent names, the name pointers, the opponent-to-portrait map and the
    8 portrait records.
  - Trampolines: Far_ShowOpponentPortrait and Far_ShowPortraitExpression.
- **Intermission scoreboard (L-003):**
  - Routines: DrawIntermissionScoreboard (heading line: ROUND x OF n, SCORING GOAL or
    RACE TO), DrawScoreboardRow, DrawSmallNumber, and WaitForStartPress (uncalled).
  - Tables: the per-GameType layout tables, ScoreboardHeadingScripts (7 layouts) and
    ScoreboardLayout ($DE).
- **L-006:** 30 fragments are now local @dead_*/@unused_prologue labels with notes.
  bank 5 $AA9A is named Mul16By8.
- **L-008:** all 29 bank-6 screen scripts are labelled (ScrLogo ... ScrIntermission), with
  headers grouping each setup/open/close triple. All ScriptPtr loads in code are symbolic.

## Readability assessment
- The new code reads at the round-1 core standard: routine headers, algorithm-level block
  comments, symbolic operands, and explicit notes where something is unreachable or unknown.
- The carried material is improved:
  - The 54 mechanical @join/@if_* merge labels are renamed by role.
  - Stale address-based names in prose were replaced, for example sub_3_8038 is now
    InitMatchPlayers.
- The areas that are still shallower are the same as in round 1: rules details, flag bits and
  physics constants (see commentary/uncertainties.md).

## Remaining generic-name counts
| Kind | Count |
|---|---|
| Address-only labels (sub_/loc_/unref_/tbl_) | 0 |
| Address-only RAM names (ram_/zp_) | 0 |
| @skipN | 0 |
| @join / @if_* | 0 |

There are 682 regular labels, all named, over 21,478 instructions. review.json has
1129/1129 regions and 623 units reviewed.

## Verification and reading copy
- `commentary.py verify`: ok true, byte_identical true (commentary/verification.json). This
  was run after the last source build.
- Reading copy refreshed: **commentary/reading-after.asm**.
- input/game.nes, the classification map and the toolkit were not modified.

## Structural questions for redirect (see evidence-policy.md)

**Do the low-confidence code regions read plausibly as code?** Yes:
- Far_ZeroRollWheels ($FB32) matches the SelectBankN_Temp trampoline pattern.
- WaitForStartPress ($97B1) is a clean WaitForNmi/Start-button loop.
- The dead else-skip JMPs repeat one compiler pattern across the rules sections.
- The "lda #n / ora #n / rts" stubs match the live arms of the same bank-5 classifier.
- The title dead arms fit the PalCycleState logic: they would have let the colour cycle end
  the intro.
- The weakest are the one-byte prologues at $D9B0 (txa) and $DE1C (pha). They are plausible
  alternate entries, but too short to rule out data.

No change to the classification is proposed. The open questions are:
1. **PortraitFrame 1/2:** which expression does each value show? Bounded check: capture the
   portrait after EndOfMatch with CurrentPlayer 0 and 1, using the opponent-message route from
   redirect r1-a7.
2. **NameCharMap ($CD43):** what are the extent and layout of this character map inside the
   bank-7 font data? The base address lies inside BigFontTop ($CD2F-$CD48), so the entries
   for character codes $20 and up fall in the single 891-byte BigFontBottom block
   ($CD49-$D0C3). That block probably holds several tables. Bounded check: list the PrintString reads while names are
   printed.
3. **Credits pages:** what are Tmp08 and DstPtr when $830E/$831A run? One breakpoint on the
   boot route would settle it. This affects commentary only.
4. **Bank 3 $8A76-$8AFF** (generic, unreferenced tile-rectangle shape): check whether any
   bank-6 script or rectangle command uses it with bank 3. Leaving it as generic data is
   acceptable.
5. **Carried from round 1:** the RunMenu $2000/$2001 reads (open bus), and what the
   scoreboard rolling wheels count.

## Disk
- commentary/ is 9.3 MB; analysis/round-1/commentary/ is 8.7 MB.
- About 2.5 GB is free.
- The scratch directory /tmp/ra-2707-c2 has been removed.
