# Commentary uncertainties

Investigated limitations carried into the source comments.  Round 1 entries are kept below
(as revised in round 2); structural classification is phase 1's (pass-002-final: 100%
classified).  These are meaning gaps, not unclassified bytes.

## Round 2: newly classified code and data
- Low-confidence code (classification by structure, no caller): Far_ZeroRollWheels ($FB32),
  WaitForStartPress (bank 3 $97B1), Mul16By8 (bank 5 $AA9A) and the 30 L-006 fragments
  (dead else-skip JMPs, second RTSs, "lda #n / ora #n / rts" stubs, one-byte prologues
  $D9B0/$DE1C, title dead arms $8469/$8476).  All read plausibly as code (consistent with
  neighbouring live patterns); the one-byte prologues are the weakest.  Their reachability is
  not claimed.
- Credits pages: DrawMenuBodyWithOffset uses Tmp08 (tile offset) and DstPtr (character map)
  as left by the screen-script commands; the actual values were not traced.
- NameCharMap ($CD43): a character-indexed tile map inside the bank-7 font data block
  (BigFontTop/BigFontBottom area); its exact extent/layout is not decoded.
- PortraitFrame values: 0 neutral, 1/2 set by EndOfMatch and other message paths before
  opponent messages, cycled 0-2 for the talking interviewer; which picture each value shows
  was not checked.  PortraitIndex is also incremented after each won tournament match, a value
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
- RunMenu reads $2000/$2001 (write-only PPU registers) on the full-screen path; kept and
  commented as a probable original bug (open-bus value not checked at runtime).
- Data banks 0, 1, 4 and bank 6 $8000-$B152, bank 2 text: only phase-1 generic notes plus the
  table labels that code references; menu-script data blocks are not split (their format is
  documented at DrawMenuBody).  Bank-6 screen scripts are labelled per record.
