# Commentary progress

## Round 1 (complete readability pass; final editorial audit still pending)
Source layout: source/game.asm includes ram.inc (symbols/constants) and bank0..7.asm.  The
tree is rendered by tools/build.py from the frozen before/game.asm plus the annotation database
tools/annotations.txt, which is the edit record for every name, symbol, operand override and
comment.  Edit annotations.txt and run `python3 tools/build.py`, then `python3 tools/review.py`
(refreshes review.json units/status).  build.py refuses to overwrite hand-edited source files
(hash guard tools/.last_build.json) unless --force.  view.py/summ.py/heads.py are reading aids.

Scope covered: all 20,698 classified instructions (banks 2, 3, 5, 6, 7) and all data/padding/
unknown ranges.
- 624 regular labels, all descriptively named (0 address-only routine labels left); every
  routine has a header; trampolines get generated "Far_<target>" names/headers.
- Local branch labels: descriptive names in core routines (NMI, VRAM buffer, reset, rules
  sections, ...); elsewhere derived names (@loop, @return, @not_XX / @is_XX from compare
  chains, @skipN).
- All branch/jump/call operands symbolic, incl. bank-aware fixed-bank calls (X overrides for
  8 calls/27 table reads whose bank is set by the caller); immediate pointer pairs (#</#>);
  flag/button constants (SYS_*, BTN_*, SCR_*, INPUT_*, BALL_*, MENU_*).
- ram.inc: 476 symbols (337 named RAM/register/constant/overlay names); 139 low-use
  addresses keep address-based zp_XX/ram_XXXX names (role unknown).
- About 1,180 forward-branch targets still carry generic @skipN names (plus @loop/@return);
  readable in context but a candidate for later editorial polish.
- Data: phase-1 basis printed at every data/padding block >= 32 bytes; referenced tables
  labelled with format notes; unknown ranges explained (see uncertainties.md).
- review.json: 1077 scaffold regions reviewed with summaries/source locations; 518 units
  (routines and labelled tables) reviewed.  editorial_audit left pending for final acceptance.

Depth limits (not blocking, documented in source as tentative): rules details of non-8-ball
sections, many GameCmd/flag meanings, physics constants.  See uncertainties.md.

Verification: `commentary.py verify` byte_identical true after the final build; reading copy
refreshed with `commentary.py read` (commentary/reading-after.asm).

## Round 1, part 2 (readability gaps from the coordinator; complete)
Workflow unchanged (annotations.txt -> build.py -> review.py).  New tool aids: tools/where.py
(source line -> annotation key), tools/remap.py (per-routine operand renames as O lines),
tools/msgscan.py (bank-2 message/menu script pointers + first text lines), tools/gensyms.py
(regenerates the GENERATED block of annotations.txt: MSG_*/MENU_* and GT_* constants and their
operand overrides), tools/cview.py (condensed reading view).
- Branch labels: 0 @skipN left (was ~1,180).  build.py now derives local names from the
  tested condition (value/compare/mask constant/routine result, e.g. @cue_not_pocketed,
  @ball_moving_clear, @not_eight_ball, @hit_lowest_first, @menu_parse_ptr_no_carry) and join
  points from the action at the label (@call_x / @store_x / @goto_x); EndOfShotRules labels
  carry their game section prefix (@nine_*, @rot_*, @c141_* ...).  54 merge points keep
  @join / @if_* (mechanical); 285 @loop/@return.
- RAM: 0 address-only symbols left (was 139): 112 named or aliased (OamBuf+n, PalTarget+n,
  BallState+$20 phantom ball, snapshot area Snap*, ReplayDistLeft, TargetScore,
  WorldChampUnlock, CoinToss*, ...); the 27 overlay-only defaults are no longer emitted;
  bank-5 physics now uses its own names for $0488-$0493 (ShotPeak, PhysStepCount, StepTime,
  EarliestTime) instead of the menu-window names; title hook uses TitlePhase/TitleScroll*.
- Semantics added: GameType numbers from the party menu (GT_* constants, rules sections
  renamed @eight_ball ... @one_pocket with per-branch block comments); 68 message/menu script
  pointers named from their text (MSG_*/MENU_*); 35 RuleCase_XXXX tails renamed and headed
  (RuleWinSank9, RuleFoulRespot10, RuleThreeFoulsMinus15 ...); consecutive-foul counter
  (IncFoulCount/GetFoulCount/ClearFoulCount, FoulCount), CalledBallNum/CalledPocket/
  CalledShotMade, LowestBallAtStart, RailHitCount/RailBallMask, EliminateShooter/PlayersLeft,
  PlayerFlags bits, GameOptions bits, ButtonsConsumed; scoreboard rolling wheels
  (RollWheel/RollState, ScoreboardUpdate algorithm); TitleAnimationHook phases,
  SelectionScreenHook (controller choice), RackPreviewHook; TableControlInput button map;
  28 secondary entry labels got headers.  Polarity fixes: AnyBallActive / CountGroupBalls /
  CountActiveBalls headers were inverted or vague and are now exact.
- Defects fixed: false immediate pointer pair (AddPoints -1 rendered as #<CoinTossData);
  two ASCII-table reads rendered relative to SplitResetAddr now use AsciiTileTable-$20;
  MenuScriptPtr comment said bank 3 (menu scripts are in bank 2).
- review.json: 577 units reviewed, scaffold unchanged, editorial_audit still pending.
- Verification: byte_identical true; reading copy refreshed.
