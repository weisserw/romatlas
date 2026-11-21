# Commentary uncertainties

Carried forward from round 1 (analysis/round-1/commentary/uncertainties.md) and
revised for checkpoint pass-002.

## Still open from round 1
- CPU shot-type routines in bank 6 (ShotType*, TryCushions*, TryCombination*): the
  mechanics (paths, cushion mirroring, trial simulation) are clear; the exact game
  meaning of each variant (bank vs kick vs combination onto the 9 ball) is tentative.
- MatchGames ($9F): bits 0-1 games played, bits 2-3 target, bits 4-6 per-game winner
  bits (from GameResult/ShowMatchScore) - inferred from code, not observed in a match.
- RoundState ($A3) values 1/2/3 (foul, break/first shot, lag) are inferred
  from their tests; only $80 (rack over) is certain.
- WatchCpu1 ($CC) doubles as the title timeout flag set by a sound-script command, and
  bank 0's ending uses $CC as SparkleIndex (spelled separately where it is used so).
- Bank 7 $A41B reads ROM $C054 with LDA where JSR Random was probably intended; the
  branch it controls is never taken.

## Resolved or changed in round 2
- MetaStringStep is started only by MetaStringStart (library slot $C048), which has no
  caller in classified code; both stay "idle in practice".
- The former unknown entries $82E8, $83C2, $8E39, $9CD1, bank 6 $95FF and bank 14
  $C33E are classified code now and are named (MainMenuWithMusic, TourMenu_Password,
  ShowPasswordScreen, DrawScoreboardScreen, PathKind4Setup, MainLoop tour-lost answer).
- Bank 8 FarCall slots: round 1 had the entries from $8024 on shifted by one slot.
  $8024 is ShowGameSetBanner, $8027 the "BREAK SHOT" banner (renamed from
  AnimateHudBars), $802A ScatterBannerLetters, $802D FlashScoreMarkers.
- VoiceTable1/2 now carry their $C009 CPU value again (the bank 2/4 reset stubs at
  $8000 had shifted the generated label base).

## New in round 2
- Password system (bank 0): the encode/decode arithmetic is read statically; it has
  not been exercised at runtime (lead L-018).  The letters of SpecialPasswords are
  grid codes; their on-screen letters were not decoded.
- PasswordIconGrid (2x2 sprites on the password screen): picture not identified.
- Unreferenced code with plausible but unproven roles: ShotTypePowerUnused,
  ApplySpinCurveUnused, ProbeBehindBall, DirectionToSineCos, DebugScrollWithPad2,
  DebugShowBallPosition, AnimateGroupChoiceOld, RotateOamBy8, ScrollToOtherPage,
  RowColToNtOffset (its loop would hang for a nonzero column), the extra Load/Store
  ball-record helpers in banks 6 and 10, DrawPocketCursor_DeadPrologue.
- CollideHeadOnA subtracts the high byte of the transferred speed twice; probably a
  slip in the original, noted at the instruction.
- Bank 2 VoiceTable words are not 64-byte aligned; the player rounds them down, so
  sample 0's tail overlaps sample 1's first 65 bytes.  Harmless, but the exact sample
  extents follow the hardware rule, not the table words.
- Phys 88856..88888: 16 (X, Y) byte pairs with no reader found (lead L-019).
- Single unexplained byte at bank 1 $AD28 inside the screen-object chain.
- Data regions classified from consumer reads (lead L-016 tables): their labels and
  notes name the reading routines; record layouts beyond those reads are not claimed.
- The 23 unknown bytes (lead L-017) keep per-fragment notes; none is claimed as code.
