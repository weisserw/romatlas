# Commentary uncertainties

Carried from round-2 commentary (analysis/round-2/commentary/uncertainties.md) and revised for the round-3 map
(maps/round3-disasm.json, sha 24099db1...). Items whose evidence changed in round 3 (L-007, L-008, L-009, L-010)
were rewritten; the rest is unchanged. Round 4 (map unchanged) applied L-011 (redirect r3-a1 captures and call-site
enumeration): clock temperature, slot symbol pictures and pay-line rows, sound ids $0D/$10, the 7:$C0D3 figure.

## Structural (phase 1)
- Every byte is classified. File 0x18010 (bank 6 $8000, value $EA) is Data, the bank-6 leader byte (Static 0.8, by
  analogy with the $FF leaders of banks 0-5). The competing reading, an unentered NOP prologue, is noted in source.
- Unreferenced code: the L-004 residue (675 bytes) and, new in round 3, 7:$CFCF-$D024 (86 bytes,
  AdvanceTextCursor_Unref / PrintCharAtCursor_Unref) are "statically unreferenced, no known entry" (Static 0.8),
  not claimed dead. The round-3 routines use $0308-$0312, which the shipped game uses as AttrShadow, so they look
  like leftovers of an earlier windowed text system; their 12-window tables TextCursorColBase/RowBase/Width are read
  by nothing else.
- Generic data with no reader (operand, immediate-pair, data-word scans; some also runtime read-watch):
  6:$B554 (20, looks like two extra SlotWinTextFrames rows), 6:$85CE (32), 7:$C0D3-$C286 (436, a 14-frame
  metasprite-style block with mirrored second half; no SprDef or fixed-count writer can draw it; r3-a1 renders show
  a knobbed rod swinging upright -> horizontal -> upright, so source calls it probably an unused sprite slot-handle
  pull animation -- inference only, row layout of the 13/17-piece frames guessed), 7:$D0E4 (12),
  7:$D0F0 (60, ascending nametable-0 row addresses), 7:$F398 (8), 7:$F3DC (60, reversed nametable-1 row addresses),
  7:$E195 (15) and the bank-7 $E000 head (5). Roles in source are worded as inferences only.
- 7:$E195: ShowStartupScreen computes X = StartupParam >> 1 and discards it, then loads fixed sprite-0 values twice;
  the 15-byte step-16 table right after it may be a vestigial sprite-0 height table indexed by that X (inference;
  runtime watch saw only the RTS dummy read).
- 7:$DCA3-$DCEF stays one generic region in phase 1; its internal tables (BigDigitTiles, ClockTextTiles, digit
  tables, label tiles) are named by equates from their readers.

## Data-record interpretation
- Record sizes now come from consumer index domains (round-3 map evidence). Eight round-2 equate sizes were wrong
  and are corrected (RouletteWheelFrames 196, SlotMessageFrames 210, ReelSymbolTilesA 48 + CoinMarkerTilesA,
  ReelSymbolTilesB 64, HoldCardTiles 24 (was HoldLabelTiles 6), NametableRowAddr 60, ClockHourTable 144 (was
  ClockSecondsTable 30; renamed TemperatureTable in round 4), FourReelDenomText 12). Records whose end is anchored only by the next table or a stride
  (PokerPayLineText, HandResultTexts, BigSixWheelTiles, BigSixSymbolText, SlotDenomTextA) may include trailing bytes
  that are never read.
- Metasprite definitions store, per frame, N attribute bytes followed by N tile numbers (the round-2 comment had the
  order reversed). Frame strides of DrawSprite3x2/4x2/5x2 (6/8/8) are smaller than their records, so only frame 0 is
  meaningful; all callers pass frame 0. Frame 2 of MsprWheelPointer is never selected.
- TemperatureTable (phase-1 label ClockHourTable; the source renames the record label. The phase-1 portable map
  maps/current.json and its generator copy commentary/source/mapping.json deliberately keep ClockHourTable: they
  are the frozen checkpoint the source was generated from, and the name still describes the hour-indexed layout): values 70-99 by hour and
  10-minute slot are the temperature the clock screen shows every other 4 in-game seconds, with a degree sign
  (r3-a1 capture: 85 degrees at 8:00 = entry 48). Degrees F is an inference from the value range. The index uses ClockMinutes mod 10 (probably meant /10), so rows overlap
  and hour 23 reads up to 4 bytes past the table.
- Screens: $10-$15 are casino entrance pictures with a leaving vehicle (bus, taxi, green car, orange sedan,
  limousine, red sports car; r1-a1 captures), chosen by EndingTally by total; $11 is also the intro after the title.
  Screen $02 is a developer placeholder text page (artificial decode only); no selector is known.
- Songs $81/$82 are the floor tunes, chosen at random by RebuildFloorWithMusic. MusicRestartId is only ever written
  with 0, so SoundFrameTick's restart path never runs.
- Sound effects $04-$08 are coin-insert sounds: slots play $04 + CoinsInserted after the increment ($05-$07), video
  poker plays $04 + PokerCoinsBet before the increment ($04-$08). (The redirect note said $05-$09; the code read in
  source gives $04-$08. $09 is the literal hold-marker sound.) Effects $0D and $10 are statically
  unreferenced: no literal or computed id at any of the 54 PlaySound call sites produces them (r3-a1 enumeration);
  their intended use is unknown.
- FloorMapRows orientation (8 x 9 blocks) follows MapWidth/MapStride; row vs column order was not re-checked.
- FloorAux (10 zero bytes) is reached only through the unreferenced GetFloorAuxPtr_Unref.
- SpriteSetData's four animation tables are consumed only by the unreferenced BuildAnimatedSprite_Unref.

## Semantic (tentative wording used in source; carried from earlier rounds)
- Sound driver command handlers SndCmdNN: named by command index; instrument meaning not decoded.
- Slot symbol pictures (r3-a1 framebuffer comparison; branch labels now use them): 4-reel 0 blank, 1 cherries,
  2 red 7, 3 green 7, 4 black 7; kind 10 0 blank, 1 black 7, 2 red 7, 3 BAR, 4 double BAR, 5 triple BAR; kind 11
  0 watermelon, 1 pale oval, 2 BAR, 3 seven, 4 cherries, 5 orange, 6 plum, 7 bell. Kind-11 symbol 1 "probably a
  lemon" rests on a tile render only (it never pays and was not on screen in the capture). Reel stop tables are
  weighted random-stop lists.
- Pay lines: window offsets 0/2/4 are the top/middle/bottom rows (r3-a1 screen check for kinds 10/11); 1 coin pays
  the middle line, 2 add the top, 3 add the bottom. The 4-reel machine reads only the middle row; its coins select
  the FourReelPayTable column. "Reel 1" in source comments means ReelSymbol+0 (assumed leftmost).
- Blackjack hotspot 4 interpreted as surrender (returns half the bet); the halving uses LSR/ROL (possible original
  bug for bets >= 256). Insurance payout ratio not verified.
- Roulette bet-spot areas are inferred from the RouletteFindBetSpot geometry and bet ids.
- HotspotTest copies the player's map Y into NumValue while B is held -- apparently a debug leftover.
- Day/night palette on the clock screen: outside 7:30-19:29 three entries are set to $3E (dark); "night" is an
  interpretation. (Round 3's header said the darkening applied between those times; the code shows the reverse.)
- Clock screen AM marker: ClockTextTiles+6/+7 are shown before noon; afternoon hours read +8/+9 = ClockBlankTiles,
  so no PM marker appears (+4/+5 are never read; they may be an unused PM marker -- not checked visually).
- InfoScreenAtTopEdge screens ($18-$1D) are per-game info pages; content not confirmed visually.
