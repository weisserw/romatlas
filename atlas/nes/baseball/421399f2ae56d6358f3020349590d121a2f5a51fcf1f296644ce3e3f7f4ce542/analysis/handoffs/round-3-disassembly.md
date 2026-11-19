# Round 3 — disassembly handoff (2026-11-18)

Status: INCOMPLETE — 1 unknown byte remains (0x2604 / $E5F4, L-009, terminal residual; not reopened, no new evidence).

## Checkpoint and verification
- Checkpoint pass-003: maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json
  (ok true, byte_identical true, map sha256 a60ef9e5ef1a4eed8f6de39ef26f41e6b7f3360e965fd7d8ae7401d2030ae2d0).
- analysis/state.json saved, maps/current.json exported (== pass-003), map.validate ok. input/game.nes unchanged.
- ROM classification: 100.00% (24591 / 24592 bytes classified; 1 bytes unknown). (exact 99.996%)
- Per bank: header 16/0; PRG bank0 $C000-$DFFF 8192/0; PRG bank1 $E000-$FFFF 8191/1; CHR 8192/0.

## Lead outcomes
| Lead | Outcome |
|---|---|
| L-011 | verified, applied (consumed): region 648 evidence corrected; script analysis/tools-r3/apply_l011.py |
| L-009 | not reopened (no new evidence); remains Unknown |
| L-012 | untouched — commentary only (camera naming inversion) |

## Changed regions
- Metadata only, region 648 (physical 0x1FA2-0x1FD3, $DF92-$DFC3): evidence string replaced
  (structural = sum-of-squares tail + SquareByte; reachability = no effective entry found, BEQ $DF90
  always taken, never executed in r1 coverage; not claimed dead). Kind Code, provenance Static,
  confidence, views/bank context unchanged.
- Physical boundaries unchanged; no region added, split or removed (471 regions). Commentary
  carry-forward by address is unaffected.

## Next
- Commentary: apply L-012 (see analysis/leads.json and analysis/handoffs/round-2-redirect.md SQ-R2-3).
- Toolkit issues: none new (analysis/toolkit-issues.md: TK-001, TK-002).
