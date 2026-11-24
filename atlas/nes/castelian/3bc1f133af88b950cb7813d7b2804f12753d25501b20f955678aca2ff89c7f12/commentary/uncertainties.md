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

## Round 3 (checkpoint pass-003)
- Observed provenance from S13b/S14 comes from artificial RAM-patched
  states: it shows the code executes, not that normal play reaches it.
- Still static-only: HERO extra-life award ($CE7F-$CEB0), BestHeight high
  byte carry ($A514), door transit/StartDoorWalk ($E339-$E4B2), jump-short
  and parts of the fall/jump states, rows 20-16 of the bottom-row chains.
- $01F0 (TowerUnused01F0) and $0214/$0215 are only written; roles unknown.
- InitTowerVars06B3 ($06B3-$06B6 init values) meaning still open.

## Final commentary (checkpoint pass-004)
- Bank 3 $B6B0 ($60, phys 63168) and bank 4 $B685 ($EA, phys 79509) are now
  map regions of kind Data with Manual provenance (ids 1966/1967). This is a
  user-directed fallback for the last residual bytes, not an evidence-backed
  classification: their meaning is unknown and the round-2 ambiguity above
  (unreachable RTS/NOP vs pad byte, no reference, lead L2-01) is unchanged.
  The source notes and review summaries say so; no label or role is implied.
- Sound globals renamed after rechecking every use: $0614 is the song tempo
  (SoundTempo; TickTowerTimer raises it below 30 time units, it was
  previously misnamed LowTimeCounter), $0615 is SoundTempoCopy (written with
  the tempo, no reader found) and $0618 is SoundRestartFlag (the former
  "SoundMisc+6" above; its purpose is still only inferred from the uncalled
  RestartSongIfAllowed).
