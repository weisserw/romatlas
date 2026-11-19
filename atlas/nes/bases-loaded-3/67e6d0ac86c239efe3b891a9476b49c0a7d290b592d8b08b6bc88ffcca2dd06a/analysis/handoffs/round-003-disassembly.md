# Round 3 — disassembly phase handoff (pass-003)

Status: **incomplete** (2 bytes remain unknown; L-015 rejection stands).

- Input checkpoint: pass-002 (byte_identical). Output checkpoint: **pass-003**.
- Verification: `python3 run.py checkpoint pass-003` -> analysis/pass-003-verification.json:
  ok true, byte_identical true, map maps/pass-003.json (sha256 b810b6fa...8c0d), generated/pass-003.
  map.validate ok; analysis/state.json saved; maps/current.json exported. input/game.nes unchanged
  (sha256 67e6d0ac...).

ROM classification: 100.00% (393230 / 393232 bytes classified; 2 bytes unknown).

## Per-bank changes
- Bank 23 (PRG 8 KiB bank 23, $A000 slot): +1 byte -> 8192/8192 classified, 0 unknown.
  New region id 1251: phys 193554, len 1, Code, Static, conf 0.8, view $B402 ctx [22,23,30,31],
  comment "dead fall-through RTS (unreachable: carry clear here)".
- Bank 28: unchanged, 8190 classified / 2 unknown (phys 230960 = $8620, 235667 = $9883).
- All other PRG banks, CHR (131072) and header (16): unchanged, fully classified.
- State diff vs pass-002: exactly one added region (id 1251), next_id 1251 -> 1252; no other change.

## Lead outcomes
- L-017 **verified/applied**. Re-checked with the configured toolkit this pass: analysis.decode at
  193552 `bcc $b403` successors [46082, 46083]; 193554 decodes `rts` (terminal); static-only
  analysis.discover from 193529 ($B3E9) emits {193554, len 1, Static, ctx [22,23,30,31]}, no
  diagnostics, not budget-exhausted. Reachability recorded separately: statically dead fall-through
  (only predecessor path has C=0), never fetched. Provenance Static (not Observed).
- L-015 (28:$8620, 28:$9883) **not retried** — no new evidence; rejection review in
  analysis/handoffs/round-002-redirect-b.md remains the basis. Kept Unknown.
- L-016 **deferred** (optional runtime team-banner confirmation; readability only, no byte gain).
- L-006 **deferred** (EDIT/2P observed provenance; no classification gain).
- No gameplay run this pass; milestones incl. ending already visited (scenarios S-011/S-012);
  analysis/scenarios.json unchanged.

## Toolkit summary
- New TK-005 (minor doc mismatch: NES map.annotate needs `id` (0 = new) and capitalized
  kind/provenance). Existing TK-001..TK-004 unchanged. See analysis/toolkit-issues.md.

## Next
- Commentary: replace the '; Unknown: 1 byte, physical 193554' note before SetLevel0 in the reading
  copy with the dead fall-through RTS comment (already in region 1251 comment).
- Redirect/coordinator: the 2 residual bytes are expected to stay unknown under the existing review;
  terminal incomplete at 393230/393232 unless a new reference source appears.
Scratch: /tmp/r3s removed; no serve processes left (one-shot stdin sessions only).
