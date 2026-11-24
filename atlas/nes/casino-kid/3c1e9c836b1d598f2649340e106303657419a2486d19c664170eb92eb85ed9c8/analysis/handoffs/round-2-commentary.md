# Round 2 commentary handoff

## Scope

This round was a full current-source readability pass on checkpoint pass-003 (Casino Kid,
UxROM, 8 x 16 KiB PRG). It covered:
- all 1582 regions: 999 code regions in banks 0-7 and 583 data regions;
- the region changes from round-2 disassembly;
- lead corrections L9-L13 and the L14 labels;
- the full backlog of mechanical branch labels;
- raw operands.

The map and classification are unchanged: still 100.00% classified, with no structural edits.

## Checkpoint and baseline

- maps/current.json and analysis/state.json both equal maps/pass-003.json (sha256 9e2a7bca...22f2).
- The previous baseline was pass-002. The whole round-1 commentary is preserved in
  analysis/round-1/commentary/, and its tools in analysis/round-1/cmt.py and cmtreview.py.
- `commentary.py prepare` rebuilt commentary/ from pass-003, giving fresh before/, baseline.json,
  phase1-state.json and review.json scaffold (1582 entries).
- The source is generated as before: `python3 tools/cmt.py build` reads commentary/before/game.asm
  plus commentary/notes/*.ann. The notes were carried by bank + CPU address or physical offset.
- Since round 1, tools/cmt.py gained:
  - DL: labels inside a data region;
  - VR: routine-local RAM aliases;
  - DN: a default note per bank;
  - data-label symbols in inline parameter words;
  - a check for duplicate local labels in one scope.
- tools/cmtreview.py now also adds review units for labelled data tables.
- Edit the .ann files, not source/.

## Verification

- `python3 /root/romatlas/scripts/commentary.py verify <ws>` returned ok, byte_identical=true
  (commentary/verification.json). It was rerun after the final build.
- `commentary.py read` refreshed the reading copy commentary/reading-after.asm (21.8k lines).
- commentary/review.json:
  - all 1582 scaffold entries have status=reviewed, a summary and source lines;
  - there are 1419 units: routines plus labelled data tables;
  - editorial_audit is left pending. It is reserved for final acceptance, and the audit was not
    run.

## Readability assessment

The readability gate is met for all currently classified source.

**Routine comments.** Every code span is covered by a routine block comment (cmt.py todo: 0
uncommented spans). The source has 722 global labels, including data labels, and 1214
descriptive local labels.

**Branch labels.** All 966 mechanical @loop_/@skip_/@case_ labels were renamed. The names
describe the role, for example @dealer_pays, @check_full_house, @wait_vblank and
@remnant_push_char. Branch targets that cross a data label became globals, for example
KingEffectSingleFlash.

**RAM operands.** No raw RAM operands are left.
- 326 V symbols, 10 routine-local VR aliases and 10 constants.
- New names include OpponentProgressBits, PokerOppMoneySaved, BjDealerMoneySaved,
  PasswordBytes and PasswordNibbles.
- The music driver per-channel arrays are named (MusicAttackTimer .. MusicOctaveShift).
- Also new: the ending and password-sprite buffers, the FDS_* registers, StackPage-relative
  return-address access, and the fixed-bank FxTmp0-5.
- The debug monitor's reuse of zero-page bytes now reads as Dbg* variables.

**ROM table operands.** 76 D and 124 DL data labels now cover the casino, status, encounter,
debug, ending, King, music and SFX step tables, the poker and blackjack UI strings, and
others. The only raw ROM operands left are the commented cross-bank entry slots
($8010-$801C) and the `inc $8001` reset idiom.

**Region changes absorbed:**
- the 21 pass-003 data labels;
- $CFF6 is now code: it is commented as part of the unreachable text-printer remnant;
- the three "unlabelled" text spans now have consumers: BjLabelArgs pointers, and the
  inline PrintText / SayInline operands.

**Lead corrections:**
- L9: category 8 is ROYAL STRAIGHT, which is intentional.
- L10:
  - blackjack result codes 1-9 are documented;
  - the payout routines were renamed by status and factor: BjPaySurrender, BjDealerWins,
    BjPayWin, BjPayBlackjack, BjPayout and BjPayPush;
  - BjCheatActive was renamed BjHitAssist; it only helps the player's HIT;
  - BjQuitMenu is the BET ALL MONEY / FINAL HAND menu.
- L11: on the debug monitor's music and SFX pages, A selects the next number and B the
  previous one.
- L12: bank 6 $8013 shows the GAME OVER screen.
- L13: the cheat strings are 30 bytes each and 28 are compared; they are at PresetPw* and
  CheatPw*.

**Round-1 errors fixed:**
- StatusCursorInit and PokerCursorInit were called "unused". They are really cursor tables,
  copied by a PushLocals/PopLocals pair.
- The $8917 note said "no reference". It is PasswordMoneyLimit: money must be at most
  $1,000,000.00.
- BjDealerName is 9 bytes, not 13; the 4-byte BjLastBet follows it.
- The data notes for the graphics blocks in banks 0-2 and for the bank 3 screens now describe
  their role instead of saying "generic".

## Unresolved questions for redirect

All are low priority and do not affect classification. They are in lead L15 and
commentary/uncertainties.md.
- Q1: Blackjack result codes 1 (surrender), 3 (lost to a dealer blackjack) and 8 (doubled
  win) are each confirmed only from the code. The role of dealer record +10 (value 2 in every
  record) is open.
  - Bounded check: one bjsession.sh run with a surrender, a double and a dealer blackjack,
    reading BjPlayerHand+15 at BjSettleHand.
- Q2: PkOpponentTriples +0 was never seen read.
- Q3: the exact bit merge of BjDealerMoneySaved into the password (PackMoneyAndProgress,
  bank 5 $8684), compared with UnpackPasswordFields in bank 6.
- Carried from earlier rounds:
  - L4 (far-call trampoline) and L7 (unreferenced fragments): reachability is still open.
  - $CFE6-$CFF6 remnant: reachability is still open.

## Deferred

- Per-record decoding of the graphics banks, screens, map and metatile data, music patterns
  and credit text (L5, L6).
- The final editorial audit (`commentary.py audit`), which is reserved for final acceptance.

## Remaining ranges

None for the round-2 readability gate.

## Artifacts

- Annotation specs: commentary/notes/*.ann.
- Tools: tools/cmt.py, tools/cmtreview.py.
- Commentary records: commentary/progress.md, commentary/uncertainties.md.
- Analysis records: analysis/leads.json (L9-L14 applied, L15 produced), analysis/progress.md.
- Preserved round-1 commentary: analysis/round-1/.
