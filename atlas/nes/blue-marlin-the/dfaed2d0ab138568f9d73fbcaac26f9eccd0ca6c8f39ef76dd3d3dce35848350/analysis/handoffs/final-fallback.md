# Final user-directed residual fallback (2026-11-20)

Status: **APPLIED**. This is a **user-directed manual fallback**, not evidence-backed classification.

## Authorization
The batch user explicitly authorized a one-time, batch-only exception: when at most 10 physical ROM bytes remain
unclassified after a terminal incomplete handoff, mark them as generic Data with *unknown meaning* using Manual
provenance. The user stated this is a fallback, not an observed or statically proven data role, and that it overrides
workspace rules requiring further structural proof for these residual bytes only. No further redirect research on them.

## Complement verified before applying
- Input: pass-004 (map sha256 4582999d..., = maps/current.json = analysis/state.json).
- Recomputed from analysis/state.json: exactly 3 uncovered bytes, no overlaps; identical to the user-supplied list.

| physical offset | bank / CPU | byte | prior status |
|---|---|---|---|
| 0x02DC6 (11718), len 1 | bank 1 $ADB6 (window $A000) | $60 | Unknown; follows tail `jmp` of FightFish_Update_fish_away |
| 0x032FA (13050), len 1 | bank 1 $B2EA | $60 | Unknown; follows tail `jmp` of FightFish_Passive |
| 0x03353 (13139), len 1 | bank 1 $B343 | $60 | Unknown; follows tail `jmp` of FightFish_DirectionTerms |

## Prior ambiguity (preserved, unchanged)
- L-023 (Static Code RTS) was rejected in round 4: an unreferenced RTS after an unconditional `jmp` has no
  control/stack contract; the twin/analogue test was negative; no reference in any tested encoding
  (analysis/handoffs/round-04-disassembly.md, analysis/redirect/r03-a/).
- r04-a runtime read/fetch/DMA watch: 0 accesses across 20 runs / 9,714 frames (analysis/handoffs/round-04-redirect.md).
- Code versus data therefore remains **materially undetermined by evidence**. The fallback does not resolve it.

## Applied
- Script: analysis/evidence/final-fallback/apply_manual_fallback.py (map.annotate).
- Regions 3315, 3316, 3317: kind Data, provenance **Manual**, confidence 0.0, no label, no views (data regions
  carry no execution view; bank/CPU recorded in the evidence text). Evidence text states the user-directed fallback,
  unknown meaning, prior L-023 rejection and that no role, consumer, format or reachability is claimed.
- map.validate ok; map.save -> analysis/state.json; map.export -> maps/current.json
  (both sha256 e401cd2f4322cb63e80ff5478fd165c85c4efdb0aacaf997db43e63f5b958b5e).
- `python3 run.py checkpoint pass-005`: maps/pass-005.json, generated/pass-005,
  analysis/pass-005-verification.json — ok true, byte_identical true. ROM unchanged (sha256 dfaed2d0...).

## Counts after fallback
- Uncovered bytes: 0 of 262,160.
- Evidence-backed classified (Observed/Static): 262,157 bytes. Manual user-directed fallback: 3 bytes.
- Code 30,903 (Observed 20,357 / Static 10,546), Data 78,810 (Static 78,807 / Manual 3), Text 13,377,
  Padding 7,998, Image 131,072, Unknown 0.

## Reporting rule
Report the 3 Manual bytes separately from proven classification. Do not describe them as a data table, padding,
dead code or unused. Their meaning is unknown and their code-versus-data role was not established by evidence.
