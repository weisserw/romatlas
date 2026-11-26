# Commentary uncertainties

Investigated limitations carried into the source comments.  Round 1 entries are kept below
(as revised in rounds 2-4).  Structural classification is phase 1's (pass-004: 100%
classified).  These are meaning gaps, not unclassified bytes.

## Final acceptance: L-013 applied and editorial audit (pass-004, unchanged map)
- Resolved (L-013, natural trace r4-m1): the SET SPIN window reads exactly $9043-$909A, so
  SpinPanelAttrs is 6 bytes.  Still open: the 32-byte uniform run $909B-$90BA is never read
  by the window reader and has no other known reader; its purpose is unknown.
- Resolved (L-013): the old "BallMarkerSprite" (bank 4 $B209) is CalledBallMarkerSet, a
  one-entry set pointer to CalledBallMarkerSprite ($B5D8, 4 pieces, 13 bytes), read in bank-4
  context.  The rest of that block holds other metasprite records and ContactSqrtTable
  ($B7E1, LookupBank4Word); the index range and end of that table are not established, so
  the remainder stays one generic block.
- Likely original-game bug, intent unprovable (L-013): the Speed Pool total is elapsed seconds
  (60-frame clock, TIME heading, World Championship unlock needs < 141), yet ChallengeResult
  inverts only THREE BALL, so a slower Speed Pool clear ranks higher.
- New static readings in the audit: bank 6 $8000-$944F holds BreakLayouts10 (80 x 25-byte
  10-ball layouts) and BreakLayouts16 (80 x 40-byte 8-ball layouts), decoded by
  LoadRackPositions.  All 160 records decode to table-range coordinates.  Which layout the
  game picks follows the index formulas in LoadBreakLayout/LoadBreakLayout8Ball; whether every
  record is reachable was not checked.  Bank 3 $9300-$936B is PreviewFrameRects (six 4 x 4
  rectangles); the 9 bytes after it have no known reader.
- Corrected meaning (audit): the bank-3 cluster formerly named StartNewRack / SetupGameType /
  RackForChallenge / LoadChallengeLayout is the tournament computer opponent's simulated turn
  and the stored post-break layouts (see SimulateOpponentTurn and LoadBreakLayout).  The
  opponent pocket counts and run-out limits per TournamentRound are mechanical facts.  The
  OppSunkTotal counter has no known reader, and the forced-rebreak path's carry result was not
  traced.
- GameCmd values other than $12 stay numeric with comments (unchanged accepted limitation).

## Round 4: L-012 findings applied (pass-004)
- Resolved: the old FontTablesTail ($CDAE-$D0C3) is two things.  $CDC3-$D0C3 is
  PowerPanelTiles, window tile set 3 (48 tiles, read in full by a natural SET POWER trace).
  $CDAE-$CDC2 is NameCharMapTail, the entries for codes $6B-$7F.  Still open: that tail is a
  pattern-only reading (seven ascending tile numbers, then zeros); no text is known to use these
  codes, so they may be unused padding.
- Resolved: WindowTileSets holds 4-byte records (source, bank, unused 0), and WinTilesBank
  (formerly WinTilesCount) holds the bank.  All seven window tile sets are labelled at their
  start.  Only set 3 has been traced; the extents of the others come from their count bytes.
- Resolved: $8A76-$8AFF is PowerPanelRect + PowerPanelAttrs (natural trace).  SpinPanelRect
  ($9043, 10 x 8) and SpinPanelAttrs ($9095) are static readings.  How many attribute bytes the
  spin window uses (probably 6) and what the uniform run $909B-$90BA is for were not traced.
  (Final acceptance: 6 bytes confirmed by trace; the run is unread.  See above.)
- Resolved: PortraitFrame 1 is a scowl (opponent lost) and 2 a grin (opponent won), seen on
  screen in redirect r3-p1 (entered through the INSTA-WIN game command).
- Confirmed: RandomMessageTable entry 0 is chosen when Random & $1F = 0 (redirect r3-p1,
  forced RandomLo).
- Open: ChallengeResult ranks every challenge except THREE BALL by "higher total = better".
  A Speed Pool total of 7 ranked NOVICE in r3-p1.  Whether the Speed Pool total is a time (where
  lower should be better) and the ranking is a game quirk was not established.  (Final
  acceptance: the total is a time; the missing inversion is a likely original bug.  See above.)
- New in this pass (static readings, consumers documented in source): the bank-7 font block
  also holds OverviewTableMap ($C8AF, 32 x 30 nametable image), OverviewTableAttrs ($CC6F) and
  ZoomedTableAttrs ($CCAF, two attribute tables), so AsciiTileTable itself only covers codes
  $20-$61.  ZoomedTableMap (bank 4 $9DAD, 35 rows x 50 tiles) is labelled.  BallMarkerSprite
  (bank 4 $B209) assumes bank 4 is mapped while DrawBallMarker runs (BuildSprites context).
  (Final acceptance: bank 4 confirmed by trace; renamed CalledBallMarkerSet.)
  In 8 ball, GameOptions bit2 probably means "groups not yet assigned" and bit3 "player 1 has
  solids" (DrawScoreLabels); not traced.
- GameCmd: only $12 has a constant (GAMECMD_INSTA_WIN).  The other values stay numeric with
  comments, because several meanings are inferred (see below).

## Round 3: L-010 findings applied
- Resolved: the credits pages' character map and tile offset come from menu op 07 in their
  scripts (NameCharMap, offset $80).  The entry values do not matter.
- Resolved: NameCharMap ($CD43) is the character map of window frame style 0.  Its live body
  is NameCharMapChars ($CD63-$CDAD, codes $20-$6A, including the frame pieces $60-$6A).
  BigFontBottom is 26 bytes long.  (FontTablesTail and the read runs inside AsciiTileTable were
  resolved in round 4, see above.)
- PortraitFrame 1 is set before messages where the opponent lost and 2 before messages where
  the opponent won (PORTRAIT_OPP_LOST / PORTRAIT_OPP_WON).  Round 3 inferred this from the
  message texts; round 4 confirmed it visually (see above).
- Correction to redirect r2-q1 finding 4: RecordGameWin (formerly PickRandomComment) does
  choose RandomMessageTable entry 0 when Random & $1F is 0, i.e. 1 time in 32 (confirmed at
  runtime in round 4).
- Renamed because they were misleading: DelayThenEnding / WaitFramesThenEnding /
  EndingScreenLoop / EndDelay became RunFramesBeforeMessage / RunFramesLoop / DelayReturn /
  FrameDelay.  The bytes after DelayReturn ($B9AC-$B9C0) are a disabled countdown.
  DrawFullScreenMenu became DrawFramedMenuBody and MenuLineTopBorder became
  ClearMenuLineBordered: the call site and border tiles show that this is the windowed path.

## Round 2: newly classified code and data
- Low-confidence code (classification by structure, no caller): Far_ZeroRollWheels ($FB32),
  WaitForStartPress (bank 3 $97B1), Mul16By8 (bank 5 $AA9A) and the 30 L-006 fragments
  (dead else-skip JMPs, second RTSs, "lda #n / ora #n / rts" stubs, one-byte prologues
  $D9B0/$DE1C, title dead arms $8469/$8476).  All read plausibly as code, consistent with
  neighbouring live patterns.  The one-byte prologues are the weakest.  $DE1C does fit
  VramBufFlushNow's stack contract, which needs one pushed byte (pass-003).  Their
  reachability is not claimed.  (In pass-003, DrawBigNumber has five static callers, and the
  bank-5 $8A3D-$8A48 tail is reached statically through AimAtCalledBall.)
- (Credits entry values and the NameCharMap extent: resolved in round 3, see above.)
- PortraitFrame values: 0 neutral, cycled 0-2 for the talking interviewer.  For the meaning
  of 1 and 2, see round 3 above.  PortraitIndex is also incremented after each won tournament match, a value
  the next portrait screen overwrites (no visible effect found).
- IntroWriteOnlyA/B ($04D0/$04D1): written by TitleIntro, no reader anywhere.
- TitleIntro's orbiting objects use overlay sprite sets 3 and 4; what they depict (probably
  balls) is not checked.
- Bank 3 $8A76-$8AFF: was "generic data, no reference"; now PowerPanelRect/PowerPanelAttrs
  (round 4).
- MatchSetting2/MatchSetting3: scoreboard use is exact ("ROUND <3> OF <2>"); their meaning in
  other modes (e.g. Challenge, where MatchSetting3 = 1 marks Speed Pool) is partly inferred.

## Semantic gaps (round 1, revised)
- GameType numbering (resolved statically in round 1 part 2; party NINE BALL -> 1 confirmed at
  runtime by redirect r1-a7): item values of the party game menu (MENU_PARTY_GAMES): 0 EIGHT
  BALL, 1 NINE BALL, 2 14.1 CONTINUOUS, 3 TEN BALL, 4 ROTATION, 6 ONE POCKET, 7 STRAIGHT POOL,
  $0B EQUAL OFFENSE, $0C FIFTEEN BALL, $11 THREE BALL, $12 SPEED POOL; $13 = Freestyle.
- GameCmd values: dispatch structure is exact (GameFrame); 1 aiming, 2 aim confirmed (second A
  shoots), 3 balls rolling, 5 ball placement, $10 ball in hand, $19 intermission scoreboard
  are supported by their handlers; most other numbers are inferred.
- ShotFlags ($C7), GameFlagsC3: roles partly inferred.  PlayerWins (formerly PlayerStat1) and
  PlayerTotalScore (formerly PlayerGames) are now named from the scoreboard columns.
  CalledBallNum/CalledPocket ($D8/$D9) bit6/bit7 flags are not decoded.  PlayerCount ($E8)
  is also written by placement/replay code with values that do not look like a player count.
- GameOptions: bit1 break pending, bit4 push shot requested, bits2-3 setup variant; other
  bits unverified.  PlayerFlags: bits0-1 consecutive fouls (FOULS column), bit2 out of the
  game, bit3 uses controller 2.
- RailHitCount/RailBallMask: CountRuleEvent is called from cushion and ball contact resolvers;
  the ball-contact caller is not explained.
- Scoreboard rolling wheels (RollWheel/RollState): animation exact; what the four wheels count
  was not traced.  ShotStat0590 role not traced.
- Physics: contact prediction and ball-ball response explained at the algorithm level; fixed-
  point scales, friction/spin constants and bank-4 table contents not decoded.
- Shared RAM: $052E-$05AD and $0488-$0578 are reused by menus, physics, title/intro hooks;
  bank-specific RB names and per-routine overrides document the overlays; lifetimes not traced.
  IntroOrbitInit also clears TitlePhase and the title scroll bytes (one byte per orbiter).
- Frame hooks and far-call entries: all installers are now known from the decoded screen
  scripts (round 2, L-008); see the Scr* headers in bank 6.
- RunMenu reads $2000/$2001 (write-only PPU registers) on the full-screen path.  This is kept
  and commented as a probable original bug.  On hardware the reads return the PPU I/O latch,
  not CPU open bus; the value was not checked at runtime.
- Data banks 0, 1, 4 and bank 6 $8000-$B152, bank 2 text: only phase-1 generic notes plus the
  table labels that code references; menu-script data blocks are not split (their format is
  documented at DrawMenuBody).  Bank-6 screen scripts are labelled per record.
