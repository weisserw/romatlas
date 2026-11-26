# Round 1 - commentary handoff, part 2 (resumed phase)

Status: complete (coordinator's three readability gaps closed; final editorial audit
intentionally not run - review.json editorial_audit stays pending for final acceptance).

## Verification
- `python3 /root/romatlas/scripts/commentary.py verify <workspace>`: ok true, byte_identical true
  (commentary/verification.json), after the last build.
- Reading copy refreshed: commentary/reading-after.asm (`commentary.py read`).
- Baseline unchanged (no phase-1 map change); input/game.nes untouched; toolkit untouched.

## Scope
All classified source (banks 2, 3, 5, 6, 7 code; data/unknown notes unchanged).  Edit record:
commentary/tools/annotations.txt rendered by tools/build.py (local-label naming logic
extended); generated block maintained by tools/gensyms.py; review.json by tools/review.py
(577 units reviewed, 1077 scaffold regions unchanged).  Details: commentary/progress.md.

## Remaining generic names
- @skipN branch labels: 0 (was ~1,180).  Derived names describe the condition or the action
  at the label; 54 merge points keep @join / @if_* (mechanical names); 285 @loop/@return;
  dispatch chains on caller-supplied values keep @not_XX / @is_XX.
- Address-only RAM symbols (zp_XX / ram_XXXX): 0 (was 139).  Uncertain roles carry tentative
  comments in ram.inc.

## Readability assessment
- Rules engine: every GameType section is named after its game (GT_* from the party menu
  script) and has a block comment walking its checks in order; the 35 shared outcome tails are
  named by outcome (RuleWinSank9, RuleScratchMinus3, RuleThreeFoulsMinus15, ...); foul/win
  messages appear as MSG_* constants named from their texts.
- Bank-3 hooks: TitleAnimationHook phase machine documented (TitlePhase/TitleScroll*),
  SelectionScreenHook identified as the "press Start on the controller you will use" screen,
  RackPreviewHook mechanics; scoreboard rolling-wheel animation explained; TableControlInput
  button map and ButtonsConsumed combo logic explained.
- Still shallower: GameCmd values beyond aiming/shot/placement, some flag bits (ShotFlags,
  GameFlagsC3, CalledBallNum bits 6-7), physics constants, data-bank internals (see
  commentary/uncertainties.md).

## Structural questions for redirect (see evidence-policy.md)
1. Bank 3 $832B-$866B (L-002): probably a frame hook; search bank-6 screen scripts for
   FrameHookAddr writes ($0435/$0436) to find installers of it and of the other unreferenced
   bank-3 hook entries ($8326/$8699/$86C5/$9385/$93AE/$93BD/$94A4/$94D2).
2. $E3A3-$E3E6 (L-005) and bank 3 $8A51-$8B13 (L-004): caller/callee pair; a trace of the
   tournament chart screen (DrawTournamentChart, trampoline $F885) could confirm both.
3. GameType mapping: now answered statically from the party menu item values; a bounded
   runtime check (pick a party game, read $DA) would confirm it (low priority).
4. RunMenu full-screen path reads $2000/$2001 (write-only PPU registers): confirm at runtime
   what results (open bus); commentary only.
5. New: TitleSkipWord ($04D4/$04D5) is read by ShowTitleSequence after the logo scripts but no
   code writer was found; a screen-script command probably writes it - decoding the bank-6
   title script records would settle it (and question 1).
6. New: which menus install RackPreviewHook / which counts the four scoreboard rolling wheels
   show; a short runtime look at the party game menu and the in-game status bar would name
   them.

## Toolkit issues
None new (analysis/toolkit-issues.md unchanged).

## Disk
commentary/ about 8.7 MB; no scratch left outside the workspace; ~2.5 GB free.
