# Commentary uncertainties

Investigated limitations carried into the source comments.  Round 1 entries are kept below
(as revised in rounds 2 and 3).  Structural classification is phase 1's (pass-003: 100%
classified).  These are meaning gaps, not unclassified bytes.

## Round 3: L-010 findings applied
- Resolved: the credits pages' character map and tile offset come from menu op 07 in their
  scripts (NameCharMap, offset $80).  The entry values do not matter.
- Resolved: NameCharMap ($CD43) is the character map of window frame style 0.  Its live body
  is NameCharMapChars ($CD63-$CDAD, codes $20-$6A, including the frame pieces $60-$6A).
  BigFontBottom is 26 bytes long.  Still open: FontTablesTail ($CDAE-$D0C3, 790 bytes) has no
  known reader, and the separate read runs inside AsciiTileTable (several maps?) are not
  decoded.
- Tentative: PortraitFrame 1 is set before messages where the opponent lost and 2 before
  messages where the opponent won (PORTRAIT_OPP_LOST / PORTRAIT_OPP_WON).  This is inferred
  from the message texts; the pictures were not captured, because the $DD=2 shortcut skips
  EndOfMatch.
- Correction to redirect r2-q1 finding 4: RecordGameWin (formerly PickRandomComment) does
  choose RandomMessageTable entry 0 when Random & $1F is 0, i.e. 1 time in 32.  This is a
  static reading of the code and was not observed.
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
- Bank 3 $8A76-$8AFF: generic data shaped like a tile rectangle, no reference.
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
