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

## Round 2 (map changed: pass-001-final -> pass-002-final; complete readability pass)
Preservation: the entire round-1 commentary directory (source, tools, annotations, review,
reading copies, baseline) was copied unchanged to analysis/round-1/commentary/ before a fresh
`commentary.py prepare` on pass-002-final (byte-identical baseline, 1129 scaffold regions).

Carry-over method: tools/ (build.py, annotations.txt, review.py, gensyms.py, ...) copied from
round 1.  annotations.txt is keyed by bank:CPU address = physical offset, so names, comments,
operand overrides and RAM symbols re-render onto the new before/game.asm unchanged.  A diff of
the old and new region maps (code/data/padding/unknown per byte and instruction starts) gave
40 changed ranges, all formerly-unknown bytes; annotation keys inside or adjacent to them were
checked one by one:
- invalidated and rewritten: 13 "UNKNOWN ..." headers (2:B502, 3:830E, 3:832B, 3:88A8, 3:8A51,
  3:95F6, 7:D9B0, 7:DC06, 7:DE1C, 7:E3A3, 7:F885, 7:FB32, 7:FC65);
- renamed after the new consumers showed the round-1 reading was wrong: ChartTilePtrs/
  ChartRectPtrs/ChartPalettes, DrawTournamentChart, LoadChartTiles, DrawChartBlock,
  WaitButtonWithChart (opponent/interviewer portraits, not a tournament chart);
  MatchesPlayed -> PortraitIndex, RackNumber -> PortraitFrame; PlayerStat1 -> PlayerWins and
  PlayerGames -> PlayerTotalScore (scoreboard WINS / TOTAL columns), AddGameWon ->
  AddScoreToTotal; TitleSkipWord -> IntroCountdown; PlayerLabelScripts -> GameTypeNameScripts
  (its "points into the unclassified $88A8 area" note confused bank 2 with bank 3);
- headers updated with newly known installers/callers: ClearPressedHook, TitleSpritesHookInit,
  TitleAnimationHook, CoinSpriteHookInit, ResetSplitHook, SelectionScreenHook,
  RackPreviewHookInit/RackPreviewHook (game-mode menu), CoinTossHook, MultiplySigned,
  DrawBigNumber, DrawMenuBodyWithOffset, ZeroRollWheels, ShowIntermission,
  ShowTitleSequence (its Ptr04 return claim was wrong: no return value), TournamentComplete,
  DrawPlayerNames, EliminateShooter, EndOfMatch; MatchSetting1-3 comments;
- still valid and kept: data labels TitleSpriteTiles, PanelLayout, BigDigitTiles (now 12
  records), SplitSpriteTemplate, ResetSignature, and every record outside the changed ranges.
Reverification: rebuilt and verified byte-identical after each batch.

Concrete defect fixed in carried material: 8 DstPtr pointer pairs in bank 3 were rendered as
#<SplitResetAddr (a code label); DstPtr/MenuTileMap is a character-to-tile map indexed by the
character code, and $C84D = AsciiTileTable-$20, so they are now #<(AsciiTileTable-$20).
Three ASCII lookups in the new DrawSmallNumber got the same fix.

build.py: new tag `D key name` (unreachable fragment kept as a local label of the enclosing
routine) so the L-006 dead JMP/RTS fragments do not split routines into loc_/unref_ scopes;
immediate pointer-pair detection now yields to explicit operand overrides (a message pointer
pair had created a false code label loc_3_9E03).  gensyms.py: credits and scoreboard heading
MSG_* constants.

New commentary (all newly classified code and data):
- bank 3 boot intro: TitleIntro (star field, split slide-in, orbiting objects), IntroOrbitInit/
  IntroOrbitStep (CosineTable ellipses with depth via sprite priority), IntroStarsStep/
  IntroStarMove, IntroOrbitRecords, IntroStarTiles, CosineTable (round-1 "consumer not
  identified" curve table), overlay names IntroOrbit*/IntroStar* (K + per-routine overrides);
  credits pages DrawCreditsPage1/2.
- bank 3 opponent portrait screen: ShowOpponentPortrait, ShowPortraitExpression,
  DrawPortraitCurrentFrame, LoadPortraitGraphics, DrawPortraitFrame, PrintOpponentName;
  tables PortraitTilePtrs/RectPtrs/Colors, OpponentNamePtrs, OpponentNames (11 names),
  OpponentPortraitTable; NameCharMap constant.
- bank 3 intermission scoreboard: DrawIntermissionScoreboard, DrawScoreboardRow,
  DrawSmallNumber, WaitForStartPress (uncalled), ScoreboardLayoutByType/Challenge,
  ScoreboardHeadingScripts (7 layouts), ScoreboardLayout ($DE).
- bank 7: PrintString ($E3A3), trampolines Far_ShowOpponentPortrait,
  Far_ShowPortraitExpression, Far_DrawMenuBodyWithOffset, Far_ZeroRollWheels (uncalled).
- L-006 fragments: 30 fragments as local @dead_*/@unused_prologue labels with block notes; 5:AA9A named
  Mul16By8 (unreferenced helper).
- L-008: the 29 bank-6 screen scripts $BB48-$BE8D are labelled (ScrLogo ... ScrIntermission)
  with headers grouping setup/open/close triples; all ScriptPtr pointer loads in code are now
  symbolic (#<ScrXxx, via X bank overrides).
- 54 mechanical @join/@if_* merge labels renamed by their role (@set_item_disabled,
  @test_right_limit, @position_changed, ...).

Counts: 21,478 instructions, 682 regular labels (all named), 0 address-only labels
(sub_/loc_/unref_/tbl_), 0 address-only RAM symbols (ram_/zp_), 0 @skipN, 0 @join/@if_*.
review.json: 1129/1129 scaffold regions and 623 units reviewed; editorial_audit pending.
Verification: `commentary.py verify` byte_identical true; reading copy refreshed.
