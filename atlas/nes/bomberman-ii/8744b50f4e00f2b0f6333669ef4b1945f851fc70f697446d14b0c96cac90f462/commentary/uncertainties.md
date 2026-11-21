# Commentary uncertainties

Round 2 (checkpoint pass-002, whole ROM classified).  Round-1 notes are preserved in
analysis/round-1/commentary/uncertainties.md.

## Resolved since round 1 (now classified data, named in source)
- Bank 5 $9508 PasswordCursorSprite, $A0B1 SoundTestCursorSprite, $B8E9/$B8EF VsMatchPointTiles9/VsMatchPointTiles,
  $B9C7 IntroScoreText; bank 0 BossActionTable (13 entries, incl. BossActionBlink and the new code
  BossActionMoveSpeed2/3); fixed $E000-$FF60 DMC samples (DmcSampleTable records); StageItemChoices 144 B,
  ItemChrIndex 13 B, StageTimeTable 96 B.

## Generic data kept as generic (bounded inspection done, no consumer)
- Bank 6 $B027 (6 B) after the ChrFontMenu chain: no reader; residue/padding role not claimed.
- Fixed DMC alignment gaps $EBA1, $EE71, $F071, $F581, $FF61: never fetched by any sample record.
- Fixed $D2F9-$D7FF, $DDA9-$DFFF, $FF68-$FFDF; bank tails ($B4CB-$BFBB bank 0, $BDB1 bank 1, $B02D bank 6,
  $BE96 bank 5); bank 0 $967E (3 unused BossActionTable-style words beyond the cmp #13 bound).
- CHR chain chunks in banks 1/6 and music data in banks 2-4 are generic pieces of documented structures
  (bank header notes); per-chunk names were not attempted.

## Semantic uncertainties (tentative wording in source)
- Enemy kinds 12-14 behaviour partly analysed; kind 22 (no known spawner, L-011) and kinds 23/24 roles tentative.
- Sound: $020B and $0210 are written by byte-codes (SndCmdSet020B/SndCmdSet0210) but never read by the driver;
  $0209/$0211/$0212 only cleared at init.  Channel-array names (Ch*) follow the driver code; musical intent of
  ChAttenuation units and envelope rates is mechanical.
- BossActionMoveSpeed2/3 (actions 6/7) are unused by the six known scripts.
- InvincibleFrames/Seconds, BlinkTimer/Phase, EnemySpeedFast, BonusEnemyScore names are tentative (marked).
- Reachability (L-006): SaveAndSelectPrgBank $C149, RestoreSavedPrgBank $C21B, ResetRandom $C8DA,
  QuerySoundState $C92A, HiddenP2WorldSelect bank 5 $A94A have no caller; their effects are static readings.

## Final editorial acceptance (round-2 checkpoint, no classification change)
- Raw RAM operands now carry address-based mechanical names: PowerOnStubRam ($0200, power-on stub page),
  SndInitClear0209/0211/0212 (only cleared by SoundDriverInit), SndCmdParam020B/0210 (written by byte-codes, never read),
  ZeroPageFF ($FF, read only by CtlFadeMusic with no song running; no direct writer, possibly an intended `#$FF`),
  IntroAnimVar/IntroAnimVar2 and BlinkPhase (cleared only; no direct reader).  Meanings remain unknown.
- Bank 5 $95F9 (16 B, a permutation of 0-15 before PasswordCharTiles): no operand or pointer addresses it; kept generic.
- Ending reachability: StageCleared falls through to RunEndingAndReset after stage 8 of world 6; the round-2 redirect
  (G4) exercised this with an artificial World/Stage poke.  A natural full play-through was not traced.
- Round-2 G5 observed kinds 12-15 active in artificial stage windows; their behaviour notes stay static readings.
