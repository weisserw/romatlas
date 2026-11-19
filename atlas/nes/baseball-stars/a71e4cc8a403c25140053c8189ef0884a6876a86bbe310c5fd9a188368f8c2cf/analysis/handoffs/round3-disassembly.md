# Round 3 — disassembly phase handoff (pass-005)

Status: INCOMPLETE (5 Unknown bytes). One pass; the single ready lead L-R4-01 verified and applied.

## Checkpoint and verification
- Input maps/pass-004.json (c1ceb883...). Output maps/pass-005.json
  (sha256 407f32c4ca37db1bc813060b933f91c3ea6001be92a2eddd4a4959b8eea92ee1), generated/pass-005,
  analysis/pass-005-verification.json: ok=true, byte_identical=true. state.json == current.json == pass-005.
- ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown) — exact 99.998%, NOT complete.
- Per-bank table: analysis/progress.md (pass-005 entry). Code 62,289 (Observed 10,804), Data 68,794, Image 131,072.

## L-R4-01 (240 B) — all seven fragments accepted, Static, reachability unresolved/no known caller
b3 $AC36-$AC46 (dup SetupSprite0Split3), b3 $AC73-$AC97 (dup TileXYToPpuAddr), b3 $B496-$B4C4 (menu erase
handler; $B4A9-$B4AA inline .addr Data), b7 $B820-$B83A, b9 $ACC2-$ACD4, b12 $8DE7-$8DFD, b14 $C126-$C16B.
Edits: scratch/r3/batch.json (applied by scratch/r3/apply.py through scratch/broker.py + scratch/rpc.py).
Details: analysis/investigation.md (Round 3 section).

## Remaining Unknown (5 B)
- b10 $9B08 (phys $15B18, 1) and b12 $9026-$9028 (phys $19036, 3): L-R3-01 deferred — rejection review in
  analysis/handoffs/round2-redirect.md still applies (no new reference evidence).
- b13 $A07A (phys $1A08A, 1): L-R2-04 blocked by TK-003 (overlapping instruction streams not representable).

## Next
- Commentary: new Static code in the 7 fragments needs labels/comments (b3/b7/b9/b12/b14).
- Redirect: nothing actionable unless a reference for L-R3-01 appears or TK-003 is addressed (toolkit change
  requires authorization). No gameplay run this pass (S01-S18 exhausted).
- Toolkit: TK-001..TK-003 unchanged (analysis/toolkit-issues.md). Servers started this pass were stopped.
