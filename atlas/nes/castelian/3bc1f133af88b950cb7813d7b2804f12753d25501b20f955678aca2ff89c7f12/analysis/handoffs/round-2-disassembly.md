# Round 2 disassembly handoff

- Status: pass COMPLETED; ROM incomplete (2 unknown bytes).
- ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown).
- Input: pass-001 (68.00%). Output checkpoint: pass-002 (maps/pass-002.json, generated/pass-002,
  analysis/pass-002-verification.json; ok, byte_identical; map sha256 9a7a6642...7f0f5).
  analysis/state.json (571 regions) and maps/current.json (validated) match pass-002.
- Applied all 75 r1-a1 ledger proposals (analysis/redirect/r1-a1/ledger.json) after re-check
  (analysis/evidence/round2/tools/verify_leads.py, apply_leads.py). Leads L1-01..L1-09 verified;
  none rejected. Lower-confidence items: $E4B3-$E4CC, $E585-$E590 (Static code 0.6, orphan,
  reachability unproven); $B692-$B6B7 (RAM routine storage image, Code view at $B692 because NES
  uploaded views are unsupported; executes at $06D5).
- New Observed: L1-05/L1-09 code (attract run S12), $F4ED-$F4FC (S13 PLAYER 1 WINS, artificial).
- Residual: L2-01 (blocked) bank 3 $B6B0 `$60`, bank 4 $B685 `$EA`; unreferenced 1-byte valid
  opcodes vs pad; no discriminating evidence. Needs a terminal rejection review in redirect.
- Commentary needed for newly classified regions (75 new + 1 split into 2) - most have no names yet;
  roles are in region evidence strings and ledger.json.
- Gameplay: all milestones visited except PLAYER 2 WINS branch (trivially reachable by swapping
  the S13 score patch) and high-score entry (does not exist). See analysis/scenarios.json.
- Toolkit: no new issues (analysis/toolkit-issues.md TK-001..TK-005).
