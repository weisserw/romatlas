# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 1
- Enemy object states (ObjState 2-9) are named mechanically (walk, fall,
  stunned, appear/disappear, flyer, vertical mover); which enemy types use
  them is not identified.
- Map cell kinds: bit 7 solid, low bits 3 = door, $81/$A1/$B1 blocking,
  $82/$91 and bits 4-6 object codes are inferred from the collision and
  renderer code only; the visual meaning of each kind is unconfirmed.
- Sound driver commands $F4-$FE: only the control-flow role ($FF next
  pattern, $FE end, $F8 rest, $F9 tie) is reasonably clear; the parameter
  meaning of $F4/$F6/$F7/$FA/$FB/$FC/$FD is not decoded. The `cmp #$18`
  branch can never match a command byte (looks dead).
- TechniqueValue/ExtrasCounter are named from the tally captions and their
  update sites; the exact scoring formula is not checked against play.
- UpdateHiScore copies the high score back into the player score when the
  score is not larger; this looks odd and was not tested at runtime.
- ShowTimeBonusScreen, GameOverSecret, attract mode and HERO-specific paths
  are static-only (never executed in round 1). Round 2: attract mode was
  reached in the redirect run S12 (see the round-2 section).
- (Resolved in round 2) The stubs at $F11A/$F197 are the return points of
  the bank 5 lift/breakable drawers $A533-$A5E1 (lead L1-05 verified).
- ComputeWaterLevel/BottomRows naming ("water", "bottom rows") is tentative.

## Round 2 (checkpoint pass-002)
- Bank 3 $A500-$AEFF (40 x 64-byte 1bpp mask/image bitmap pairs) has no
  reader anywhere in the ROM; role unknown, possibly leftover data.
- Bank 3 $B6B0 ($60) and bank 4 $B685 ($EA) remain unknown: valid 1-byte
  opcodes or pad bytes, no reference either way (lead L2-01).
- RestartSongIfAllowed / SoundApi_RestartSong ($A0EC/$A019),
  PlayerState_Door_ReverseFrame ($E4B3) and IncExitFlagCapped ($E585) are
  static code with no known caller; purposes are inferred from shape only.
  The meaning of SoundMisc+6 ($0618: $FF after PlaySong, Y after command
  $FE) is not settled.
- BottomChain2400 entry 0 decodes to $6391 (not ROM); presumably BottomRows
  is never 0 on that path, unverified.
- BreakPixelRamp as a "crumble" animation and the lift drawer tile patterns
  are visual interpretations from the masks only.
- $DD4B-$DD58 and $DD7D-$DD82 inside the player frame lists are only read
  through base-minus-index addressing or not at all; the exact list
  boundaries are inferred.
- AttractDemoInput pair order (buttons, duration) follows AttractInput's
  code; BonusDemoInput is assumed to use the same format.
- TowerTextPalette has a ninth byte ($A297 = $04) with no known reader.
