# Disassembly progress

## Coordinator (goal.md orchestration)

- Round: 3
- Current phase: TERMINAL — whole-ROM classification complete; final audit passed (2026-11-24). Round 3 redirect not run: no unknowns/pending review/editorial issues remain. Ready for archive (not executed).
- Final checkpoint: pass-003 (== maps/current.json)
- Phase handoffs: analysis/handoffs/round-1-disassembly.md; analysis/handoffs/round-1-commentary.md (complete); analysis/handoffs/round-1-redirect.md (complete); analysis/handoffs/round-2-disassembly.md (complete); analysis/handoffs/round-2-commentary.md (complete); analysis/handoffs/round-2-redirect.md (complete); analysis/handoffs/round-3-disassembly.md (complete); analysis/handoffs/round-3-commentary.md (complete); final: analysis/handoffs/final-coordinator.md
- Coordinator note for redirect (resolved in round-1 redirect: confirmed, no contradicting code evidence): verify banks 1-9/11 whole-bank generic-data rationale (heuristic: no code-like run >=40 B) against cross-bank call targets / bank-switch contexts.
- Notes: disk free ~2.9G at start; keep scratch bounded.

## Phase log

| Round | Phase | Status | Input ckpt | Output ckpt | Verification | Leads consumed/produced | Classification |
|---|---|---|---|---|---|---|---|
| 1 | disassembly | incomplete | none (baseline created) | pass-001 | byte_identical true | consumed none / produced L-001..L-008 | 93.42% |
| 1 | commentary | complete | pass-001 | (source-only) commentary/source | byte_identical true (commentary/verification.json) | none / 7 redirect questions | 93.42% (unchanged) |
| 1 | redirect | complete (ready leads) | pass-001 | none (research only) | n/a (map unchanged) | updated L-001/L-002/L-003/L-007 / produced L-009..L-014 | 93.42% (unchanged; 8615 B proposed) |
| 2 | disassembly | incomplete (6 B unknown) | pass-001 | pass-002 | byte_identical true (analysis/pass-002-verification.json) | consumed L-001, L-002 (superseded), L-003, L-007, L-009..L-014 / produced none | 100.00% (131082 / 131088; 6 unknown) |
| 2 | commentary | complete | pass-002 | (source-only) commentary/source | byte_identical true (commentary/verification.json) | none / 3 redirect questions | 100.00% (unchanged; 6 unknown) |
| 2 | redirect | complete (ready lead) | pass-002 | none (research only) | n/a (map/state unchanged) | updated L-003, L-007 / produced L-015 | 100.00% (unchanged; 6 B proposed by L-015) |
| 3 | disassembly | complete (0 B unknown; commentary/editorial pending) | pass-002 | pass-003 | byte_identical true (analysis/pass-003-verification.json) | consumed L-015 / produced none | 100.00% (131088 / 131088; 0 unknown) |
| 3 | commentary (final acceptance) | complete; audit passed | pass-003 | (source-only) commentary/source | byte_identical true (commentary/audit.json, commentary/verification.json) | none / none | 100.00% (unchanged; 0 unknown) |

## Round 1 disassembly

ROM classification: 93.42% (122467 / 131088 bytes classified; 8621 bytes unknown).

- Checkpoint pass-001: verification ok, byte_identical true (analysis/pass-001-verification.json).
- Code 24148 (Observed 12483 / Static 11665), Data 82066, Padding 16253. Execution coverage
  (separate): 6296 observed instruction starts (4526 natural).
- Per-bank classified/unknown (8K banks): header 16/0; 0: 8185/7; 1-9: 8192/0 each; 10: 1280/6912;
  11: 8192/0; 12: 7498/694; 13: 8172/20; 14: 7690/502; 15: 7706/486.
- Evidence and rules: analysis/investigation.md. Scenarios: analysis/scenarios.json.
  Leads: analysis/leads.json (L-001..L-008). Toolkit: analysis/toolkit-issues.md (TK-NES-001..005).
- Handoff: [analysis/handoffs/round-1-disassembly.md](handoffs/round-1-disassembly.md).

## Round 1 commentary

- Status: COMPLETE (worker 1 interrupted, worker 2 resumed and finished). Input
  pass-001; classification unchanged at 93.42%.
- commentary/source/game.asm regenerated from the frozen before/ listing by
  commentary/tools/annotate.py + notes.txt: 692 described routines, 992 descriptive
  local branch labels, about 1070 inline comments, 171 RAM symbols (+ range aliases),
  116 data labels; bank 6 $9757-$BA4A fully read with name corrections; all
  address-named fields resolved or explained.
- Verification: commentary/verification.json ok/byte_identical true; reading copy
  commentary/reading-after.asm refreshed. review.json: all 685 entries and 615 units
  reviewed (editorial_audit pending for final acceptance).
- Details: commentary/progress.md, commentary/uncertainties.md. Handoff:
  [analysis/handoffs/round-1-commentary.md](handoffs/round-1-commentary.md)
  (7 structural questions for redirect). No toolkit issues found in this phase
  (analysis/toolkit-issues.md unchanged).

## Round 1 redirect

- Status: COMPLETE with ready leads (not terminal). Input pass-001; map/state/source unchanged.
- Attempt dir analysis/redirect/r1-a1/ (static scripts, closure_out.json, proposals.json).
- Ready: L-001 (7138 B: all of 8K bank 10 + 226 B fixed fragments), L-009 (420), L-010 (683),
  L-011 (244), L-012 (29), L-013 (14), L-014 (corrections, 0 B), L-003 (up to 87 B, low).
  6 B (lone RTS bytes, $9B8B) without proposal.
- Coordinator check of 8K banks 1-9/11 whole-bank data: confirmed, no contradicting code evidence.
- Recommendation: another disassembly round. Handoff:
  [analysis/handoffs/round-1-redirect.md](handoffs/round-1-redirect.md).

## Round 2 disassembly

ROM classification: 100.00% (131082 / 131088 bytes classified; 6 bytes unknown).

- Status: incomplete only because 6 bytes remain unknown (lone return opcodes); all ready leads consumed.
- Checkpoint pass-002: ok, byte_identical true (analysis/pass-002-verification.json, maps/pass-002.json,
  map sha256 b10f5901...). maps/current.json exported and validated.
- Code 30182 (Observed 12483 / Static 17699), Data 84627, Padding 16273; 767 regions.
- Per-bank classified/unknown (8K banks): header 16/0; 0-11: 8192/0 each; 12: 8190/2; 13: 8191/1;
  14: 8189/3; 15: 8192/0. Unknown: $9B8B-$9B8C (bank 6), $A1E4 (bank 6), $C405, $C916, $CA5B.
- Leads: L-001, L-003, L-009..L-014 verified; L-002 superseded; L-007 verified (Phantom Bat, Dracula,
  ending, credits visited via RAM shortcuts; scenarios SC-BOSS3/SC-WARP-CLEAR/SC-BOSS18/SC-ENDING2).
- Execution cross-check: 623 new observed starts, all inside existing Static code; no contradictions.
- Toolkit: no new issues this pass (analysis/toolkit-issues.md TK-NES-001..005 unchanged).
- Handoff: [analysis/handoffs/round-2-disassembly.md](handoffs/round-2-disassembly.md).

## Round 2 commentary

- Status: complete; handoff [analysis/handoffs/round-2-commentary.md](handoffs/round-2-commentary.md).
- Map changed since round 1: the whole round-1 commentary is preserved at
  analysis/round-1/commentary/; fresh baseline prepared from pass-002.
- Carried round-1 notes re-applied by physical offset (stale unknown/bank-context notes
  rewritten); new scope: bank-5 object handlers $80C7-$9A34 (bosses and stage enemies,
  demo scripts), fixed-bank helper fragments, bank-6 $90A1-$934B, item records, L-003 fragments.
- review.json 772/772 entries and 1150/1150 units reviewed; editorial audit pending.
- Verification byte_identical true (commentary/verification.json); reading copy refreshed.
- Redirect questions: 3 (unknown-byte references, low-confidence fragments, spawner/stage lists).


## Round 2 redirect

- Status: complete with one ready lead; attempt dir analysis/redirect/r2-a1/; handoff
  [analysis/handoffs/round-2-redirect.md](handoffs/round-2-redirect.md).
- L-015 ready: the 6 residual bytes as unreferenced return stubs (Code, Static, low confidence) -> 6 B.
- L-003 kept (no demotion); SC-BOSS6 (Medusa, stage 6) visited; bosses 9/12/15 deferred with recipe.
- Map, state, ROM, source unchanged; own emulator bridge stopped. Recommendation: another disassembly round for L-015.

## Round 3 disassembly

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

- Status: all bytes classified; commentary for the new stubs and the editorial audit remain.
- Checkpoint pass-003: ok, byte_identical true (analysis/pass-003-verification.json, maps/pass-003.json,
  map sha256 d9f5dde8...). maps/current.json exported; map.validate ok.
- Code 30188 (Observed 12483 / Static 17705), Data 84627, Padding 16273; 772 regions.
- Per-bank classified/unknown (8K banks): header 16/0; 0-15: 8192/0 each.
- L-015 verified after independent review: $C405, $C916, $CA5B (fixed), bank-6 $9B8B-$9B8C (sec;rts),
  bank-6 $A1E4 annotated Code/Static/0.5 as unreferenced return stubs; reachability unknown (not Observed).
- No gameplay (no new evidence available for these bytes). Toolkit: no new issues (analysis/toolkit-issues.md).
- Handoff: [analysis/handoffs/round-3-disassembly.md](handoffs/round-3-disassembly.md).

## Round 3 commentary (final acceptance)

- Status: complete; handoff [analysis/handoffs/round-3-commentary.md](handoffs/round-3-commentary.md).
- Map changed since round 2: whole round-2 commentary preserved at analysis/round-2/commentary/;
  fresh baseline prepared from pass-003; round-2 work carried by physical offset (only L-015 regions changed).
- L-015 stubs named/commented with low confidence and unknown reachability; editorial fixes
  (UnusedFillRomTarget, UnusedFillEntry, Temp4C/4D/4F, AddDamageToAux0514/ObjKind).
- review.json 772/772 entries, 1156/1156 units reviewed; editorial_audit reviewed.
- Final audit ok, byte_identical true (commentary/audit.json); reading copy refreshed.

## Coordinator terminal handoff

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
Final audit ok / byte_identical / editorial_audit true (commentary/audit.json). See [handoffs/final-coordinator.md](handoffs/final-coordinator.md).
