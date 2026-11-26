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

## Round 3 (map changed: pass-002-final -> pass-003, metadata only; complete readability pass)
Preservation: the entire round-2 commentary directory was copied to analysis/round-2/commentary/
(recursive diff identical) before a fresh `commentary.py prepare` on pass-003 (byte-identical
baseline, 1129 scaffold regions).  analysis/round-1/commentary/ is untouched.

Carry-over: tools/ (annotations.txt, build.py, review.py, gensyms.py, ...), progress.md and
uncertainties.md copied from round 2.  before/game.asm is identical to round 2 (only
before/mapping.json differs: 20 regions' evidence/confidence text), so `build.py --force`
re-rendered a source tree identical to round 2 and review.py reproduced review.json identically
(1129/1129 regions, 623 units, all statuses/summaries/source locations equal).  No record was
invalidated by the map change.  The 20 changed regions are all Code, so no data-block "Phase-1
basis" note quotes them.
Source comments quoting the refreshed evidence were updated: DrawBigNumber (five DrawScoreboardRow
callers; artificial-state observation only), @unused_prologue $DE1C (stack contract) and the
VramBufFlushNow header (entry needs A pushed), AimAtCalledBall (bank-5 tail reached by normal
return; GameCmd 5 caller).

L-010 applied (annotations.txt; round-3 section at the end for new constants/overrides):
- NameCharMap = character map of window frame style 0 (WindowFrameTable record 0 bytes 2-3);
  new labelled sub-blocks of the bank-7 font block: BigFontBottom now 26 bytes,
  NameCharMapChars $CD63-$CDAD (codes $20-$6A, frame pieces $60-$6A), FontTablesTail
  $CDAE-$D0C3 (no known reader); AsciiTileTable notes its separate read runs.  RunMenu's four
  #$43/#$CD literals now render as #</#>NameCharMap (noted as dead stores: both drawers reload
  DstPtr from MenuFrameTiles).  MenuFrameTiles and WindowFrameTable comments explain the link.
- Credits: DrawCreditsPage1/2, DrawMenuBodyWithOffset and DrawMenuBody (opcode list now
  complete: 07 set map/offset, 13, 15, 16, 17 added, 14 removed) state that menu op 07 sets
  NameCharMap / offset $80.
- PortraitFrame: constants PORTRAIT_NEUTRAL/PORTRAIT_OPP_LOST/PORTRAIT_OPP_WON (tentative) on
  all 8 writers; EndOfMatch, ShowPortraitExpression and the RAM comment updated.
  MSG_OPPONENT_WINS_MATCH ($9CE4) added (gensyms.py table + manual override: the pair is
  followed by jmp, which msgscan does not detect).
- Delay path renamed: DelayThenEnding -> RunFramesBeforeMessage, WaitFramesThenEnding ->
  RunFramesLoop, EndingScreenLoop -> DelayReturn (+ @disabled_countdown), EndDelay ->
  FrameDelay; ShowMessage header corrected (it does not close the menu; it holds 120 frames).
- RunMenu $2000/$2001 note: PPU I/O latch, not open bus.
- Finding 4 corrected: the routine is RecordGameWin (was PickRandomComment: it also credits
  PlayerWins and runs EndOfMatch); RandomMessageTable entry 0 is chosen 1/32 of the time.
Other concrete defects found in reviewed material near L-010: DrawFullScreenMenu was the framed
(windowed) menu drawer -> DrawFramedMenuBody; MenuLineTopBorder builds an empty line with side
borders -> ClearMenuLineBordered; RunMenu header corrected; EndOfMatch header now describes
all four callers and both match-end cases.

Counts: 21,478 instructions, 684 regular labels (all named; +NameCharMapChars,
+FontTablesTail), 0 address-only labels, 0 ram_/zp_ symbols, 0 @skipN, 0 @join/@if_*,
297 @loop/@return.  review.json: 1129/1129 regions and 625 units reviewed; editorial_audit
pending (final acceptance).
Verification: `commentary.py verify` ok, byte_identical true; `commentary.py read` refreshed
reading-after.asm.

## Round 4 (map changed: pass-003 -> pass-004, metadata + one data split; complete readability pass)
Preservation: the entire round-3 commentary directory was copied to analysis/round-3/commentary/
(recursive diff identical) before a fresh `commentary.py prepare` on pass-004 (ok,
byte-identical baseline, 1130 scaffold regions).  analysis/round-1/ and round-2/ untouched.

Carry-over: tools/, progress.md and uncertainties.md copied from round 3, then
`build.py --force` and review.py.  before/game.asm differs from round 3 only by the split of
region 1016 (bank 7 $C86D-$CDC2 + new region 1131 $CDC3-$D0C3); before/mapping.json also has new
evidence text for regions 991, 998, 1016 and 1064.  The re-rendered source equalled round 3
except for those regions' phase-1 notes, and review.json matched round 3 in every other
region's status, summary and units.  Invalidated and rewritten: the FontTablesTail record
(spanned the new boundary), the region 1064 and 998 notes (generic -> panel rectangles) and
every comment that relied on them (WindowFrameTable, WindowTileSets, OpenWindow*,
UploadWindowTiles, PowerMeterPanel, SpinPanel, the NameCharMap constant, AsciiTileTable).

L-012 applied (annotations.txt, "round 4" sections at the end, plus in-place header edits):
- FontTablesTail split: NameCharMapTail ($CDAE-$CDC2, pattern-only) and PowerPanelTiles
  ($CDC3-$D0C3, window tile set 3).
- WindowFrameTable / WindowTileSets headers give the real record formats (7 x 4 bytes each).
  WinTilesCount -> WinTilesBank.  OpenWindowBanked (A = request bits, X = frame record),
  SetWindowTileFlagA/B, UploadWindowTiles, OpenWindow, OpenWindowAndDraw, OpenFramedWindow(Draw)
  headers rewritten; WinTileFlagA/B and WinParams RAM comments describe their fields; the
  OpenWindowBanked branch labels are named by role.
- All seven window tile sets are labelled: WindowTiles0 (bank 1), WindowTiles1 and
  PowerPanelTiles2 (bank 4), SpinPanelTiles and WindowTiles2 (bank 6), SpinPanelTiles2
  (bank 5 $8000, which replaces that region's generic note), PowerPanelTiles (bank 7).
- PowerPanelRect/PowerPanelAttrs ($8A76/$8AF8) and SpinPanelRect/SpinPanelAttrs ($9043/$9095)
  are labelled; the panel routines load them as #</#> pairs.
- PORTRAIT_OPP_LOST/WON are no longer tentative (scowl / grin), and the ShowPortraitExpression
  header and PortraitFrame RAM comment were updated.  RecordGameWin and RandomMessageTable note
  the runtime confirmation of entry 0.
- ChallengeResult / ChallengeResultTable fully documented (six 5-threshold groups, THREE BALL
  inversion, Speed Pool ranking note).  The chained GameType compares now use GT_* constants.
- GameCmd $12: GAMECMD_INSTA_WIN constant (GameFrame dispatch and the GAME CONTROL menu's
  INSTA-WIN confirmation), with block comments at both sites.

Other concrete defects fixed in previously reviewed material (found by whole-source scans):
- 25 raw bank-2 script pointer pairs are now symbolic (21 new MSG_* constants, e.g.
  MSG_HEAD_GAMES, MSG_GROUP_SOLIDS, MSG_TWO_FOULS_WARNING, MSG_BALL_IN_HAND_KITCHEN,
  MSG_COIN_WAS_HEADS, MSG_BANNER_TOURNAMENT; MENU_COIN_CALL reused in bank 6).
- GameMode constants GM_TOURNAMENT/CHALLENGE/PARTY/FREESTYLE added and used at all 27 GameMode
  compares; 15 more chained GameType compares that gensyms.py missed now use GT_*.
- SetMenuScriptAX was wrong (it loads a fixed message): renamed SelectOpponentMissedMsg.
- DrawPlayerNames, DrawPlayerLabel and DrawScoreLabels headers are now exact (heading choice,
  party PLAYER label, 8-ball OPEN TABLE/SOLIDS/STRIPES group label).  The address-derived
  label @goto_goto_99bf is replaced.
- The bank-7 font block holds the table graphics: OverviewTableMap ($C8AF, 960 bytes),
  OverviewTableAttrs ($CC6F) and ZoomedTableAttrs ($CCAF), whose consumers are
  DrawTableScreen, LoadZoomedTableNametables, RestoreTableTiles and RestoreAttrRow.
  ZoomedTableMap (bank 4 $9DAD) and BallMarkerSprite (bank 4 $B209) are labelled, and
  BannerRightCap ($EA73) is named.  The TableTileSrc/TableAttrSrc comments and the
  DrawTableScreen/RestoreTableTiles headers were updated.
- Stale names in comments: Far_sub_5_AEAD -> Far_UpdateHud (two headers); raw $90B9/$AF0E/$864D
  routine addresses -> PhysicsStep/AnimateBallsStep/LaunchShot; MainMenuScripts header.
- The three unlabelled small fill blocks (bank 3 $BFFE, bank 4 $BFF1, bank 7 $FFF8) have notes.

Counts: 21,478 instructions, 701 regular labels (all named), 0 address-only labels, 0 ram_/zp_
symbols, 0 @skipN, 0 @join/@if_*, 0 address-suffixed locals, 297 @loop/@return.
review.json: 1130/1130 regions and 642 units reviewed; editorial_audit pending (final
acceptance).
Verification: `commentary.py verify` ok, byte_identical true after the last build;
`commentary.py read` refreshed reading-after.asm.

## Final acceptance (editorial audit; map unchanged at pass-004)
Scope: L-013 applied, whole-source mechanical checks, and targeted rereads only where a check
or lead found a concrete inconsistency.  Round-4 review credit was reused; there was no full
reread.  Details: commentary/editorial-audit.md.
- L-013:
  - SpinPanelRect/SpinPanelAttrs give the traced extent $9043-$909A (6 attribute bytes).
    $909B-$90BA is an unread run of unknown purpose.
  - BallMarkerSprite -> CalledBallMarkerSet (bank 4 confirmed), plus a CalledBallMarkerSprite
    record label ($B5D8).
  - ChallengeResult: the Speed Pool total is elapsed seconds, and the missing inversion is a
    likely original bug (intent unprovable).
- Fixed:
  - EmitMetasprite piece order (dx, dy, tile).
  - PendingSfx* -> RowCopyPending/RowCopyRow.
  - Ball slot $10 is the phantom: DrawBallShadowSprite -> DrawPhantomSprite, and
    TableControlOff/On use BALL_VISIBLE.
  - The tournament computer-opponent cluster in bank 3 is renamed and re-headed
    (SimulateOpponentTurn, SimulateOpponentShots(8Ball), LoadBreakLayout(8Ball),
    AssignOpponentGroup, PocketOpponentGroupBall, ClearBallPositions, CountGroupsOnTable,
    PickRandomBall, AnyObjectBallActive, RandomBank3, OppSink*/OppRunOutLimit* tables,
    OppSinkCount/OppSunkTotal/OppGroup).
  - New data labels: BreakLayouts10/16 (bank 6), ContactSqrtTable (bank 4), PreviewFrameRects
    and ModeMenuSpriteTile (bank 3).
  - 6 stale comment names, the stale ShotStat0590 symbol, and three entries that lacked
    headers.
- Counts:
  - 21,478 instructions and 706 regular labels, all named.
  - Address-only labels, ram_/zp_ names, address-suffixed locals and @skipN/@join/@if_*
    labels: 0 each.
  - @loop/@return labels: 297.
  - review.json: 1130/1130 regions and 651 discovered units reviewed (1514 unit records);
    editorial_audit reviewed.
- Verification: `commentary.py verify` ok, byte_identical true.  `commentary.py audit` and
  `read` were run at handoff (see analysis/handoffs/final-acceptance.md).
