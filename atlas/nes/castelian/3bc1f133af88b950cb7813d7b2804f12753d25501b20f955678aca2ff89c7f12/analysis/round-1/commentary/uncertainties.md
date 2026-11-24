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
  are static-only (never executed in round 1).
- The 7-byte stubs at $F11A/$F197 and the jmp ($01EB)/($0063) targets keep
  their bank unresolved (lead L1-05).
- ComputeWaterLevel/BottomRows naming ("water", "bottom rows") is tentative.
