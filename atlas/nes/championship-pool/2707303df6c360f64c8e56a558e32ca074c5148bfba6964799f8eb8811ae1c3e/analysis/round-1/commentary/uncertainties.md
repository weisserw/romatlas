# Commentary uncertainties

Investigated limitations carried into the source comments (round 1).  Structural
classification is unchanged from phase 1; the items below are meaning gaps.

## Unclassified ranges (explained in source, not classified)
- Bank 3 $830E-$8325 (L-001), $832B-$866B (L-002, probably an unexercised title/attract
  frame hook plus tables; $8665 labelled as it is read), $88A8-$89D6 and $8A51-$8B13 (L-004,
  table starts $88A8/$88B8/$88C9 labelled), $95F6-$9880 (L-003, tables $9820/$9880 labelled).
- Fixed bank $E3A3-$E3E6 (L-005; $E3E2 signature table labelled), $F885/$FB32/$FC65
  trampoline-shaped stubs (L-001).
- ~35 gaps of 1-8 bytes after unconditional jumps/returns (L-006), each with a source note.

## Semantic gaps
- GameType numbering (resolved statically in round 1 part 2): the values are the item values
  of the party game menu script (bank 2 $A137, MENU_PARTY_GAMES): 0 EIGHT BALL, 1 NINE BALL,
  2 14.1 CONTINUOUS, 3 TEN BALL, 4 ROTATION, 6 ONE POCKET, 7 STRAIGHT POOL, $0B EQUAL OFFENSE,
  $0C FIFTEEN BALL, $11 THREE BALL, $12 SPEED POOL; $13 = Freestyle.  The rules sections and
  their foul/win messages (MSG_* constants, texts read from the bank-2 menu scripts) agree.
  Not runtime-verified that every menu writes its result to GameType unchanged.
- GameCmd values: dispatch structure is exact (GameFrame); 1 aiming, 2 aim confirmed (second A
  shoots), 3 balls rolling, 5 ball placement, $10 ball in hand are supported by
  TableControlInput; most other numbers are inferred from their handlers.
- ShotFlags ($C7), GameFlagsC3, MatchSetting1-3 ($E1-$E3), PlayerStat1: roles partly inferred.
  CalledBallNum/CalledPocket ($D8/$D9, formerly ShooterInfo/OpponentInfo) are taken to be the
  called ball and pocket (CalledShotMade increments when that ball drops into that pocket;
  CalledPocket 7 = safety); their bit6/bit7 flags are not decoded.  PlayerCount ($E8) is also
  written by placement/replay code with values that do not look like a player count - its
  name is probably too narrow.
- GameOptions: bit1 break pending, bit4 push shot requested, bits2-3 setup variant; other
  bits unverified.  PlayerFlags: bits0-1 consecutive fouls, bit2 out of the game, bit3 uses
  controller 2.
- RailHitCount/RailBallMask: CountRuleEvent is called from cushion and ball contact resolvers;
  the 'no ball hit a rail' message supports the cushion reading, the ball-contact caller is
  not explained.
- Scoreboard rolling wheels (RollWheel/RollState): the 0-11 wheel animation is exact; what the
  four wheels count was not traced.  ShotStat0590 role not traced.
- Physics: contact prediction and ball-ball response are explained at the algorithm level
  (projection onto the relative-velocity direction, sqrt table in bank 4, normal-component
  exchange).  Exact fixed-point scales, friction/spin constants and the bank-4 table contents
  are not decoded.  PocketPointX/Y are taken to be cushion-nose/jaw corner points (consumer
  geometry); not verified at runtime.
- Shared RAM: $052E-$05AD and $0488-$04B7 are reused by the menu system, physics and title
  hooks; bank-specific names (RB entries in tools/annotations.txt, listed at the end of
  ram.inc) document the overlay, but lifetimes were not traced.
- Frame hooks reached only through FrameHookAddr/FarCallViaZp04 (title/selection/preview
  hooks in bank 3, CoinTossHook in bank 6): named from their behaviour (TitleAnimationHook
  phase machine, controller-choice SelectionScreenHook, cursor-driven RackPreviewHook); the
  installing screen script records were not decoded, so the menus using RackPreviewHook are
  unknown.  TitleSkipWord ($04D4) is read by ShowTitleSequence but its writer was not found.
- Local branch labels are derived mechanically from the tested condition (build.py
  describe()); about 54 merge points keep generic @join / @if_* names, and dispatch chains on
  a caller-supplied value keep @not_XX / @is_XX.
- RunMenu reads $2000/$2001 (write-only PPU registers) on the full-screen path; kept and
  commented as a probable original bug.
- Data banks 0, 1, 4 and bank 6 $8000-$B152, bank 2 text: only phase-1 generic notes plus the
  table labels that code references; screen-script and menu-script byte formats are documented
  at their interpreters (RunScreenScript, DrawMenuBody) but the data blocks are not split.
