# Final coordinator handoff — TERMINAL INCOMPLETE (2026-11-20)

Workflow (goal.md) reached its documented terminal handoff after 4 rounds of
disassembly -> commentary -> redirect. This is workflow termination, NOT whole-ROM
completion: ROM classification is not complete.

- ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown)
  — rounded; exact 99.998%. Per-bank unknown: b0 1, b4 1, b5 1, b6 2.
- Final verified checkpoint: pass-004 (maps/pass-004.json == maps/current.json ==
  analysis/state.json; analysis/pass-004-verification.json ok, byte_identical).
- Residual unknown (lead L-027, blocked): lone $60 bytes at b0 $8D92, b4 $B979,
  b5 $ACFA, b6 $9745, b6 $EFE5. Competing roles: unreachable `rts` vs unread
  remnant/data byte. Rejection review: analysis/handoffs/round-04-redirect.md
  (supersedes round-03 table); survey artifacts analysis/redirect/r04-a1/.
  Coordinator reviewed it: Q-R4-1 (combined pattern test) was run and its
  premises proved false ($FF is the ROM's fill, incl. a lone $FF after rts at
  b5 $8093; orphans are not uniformly $60; no dead-rts habit). Reopen only with a
  new reference/consumer for one of the five addresses or a twin routine.
- Commentary: current-source readability COMPLETE on pass-004
  (analysis/handoffs/round-04-commentary.md; commentary/source/game.asm,
  reading copy commentary/reading-after.asm, commentary/verification.json ok,
  byte_identical). Final strict editorial audit NOT yet run — reserved for final
  acceptance/archive. Uncertainties: commentary/uncertainties.md.
- Gameplay coverage (analysis/scenarios.json): title/menus, all 7 levels (stage
  order $03,$05,$02,$04,$08,$09,$06 via RAM $DA=$FF stage-end chain; artificial),
  ending $0B (no credits roll), game over, 2P Game A and B.
- Toolkit issues: analysis/toolkit-issues.md (TK-001..TK-003; none blocking).
- Leads: analysis/leads.json (L-001..L-028). Phase history: analysis/progress.md.

---

# Final conclusions — round 5 batch acceptance with USER-DIRECTED MANUAL FALLBACK (2026-11-20)

The terminal-incomplete record above is preserved as history. By explicit user authorization
(batch-only exception for <= 10 residual bytes) the five L-027 bytes were marked generic Data
of UNKNOWN MEANING with Manual provenance. This is not an observed or statically proven data
role; their code-vs-data ambiguity (dead rts vs unread pad/table byte) remains unresolved.

- Exact fallback ranges (physical offset, length 1 each): 3490 (b0 $8D92), 145801 (b4 $B979),
  175370 (b5 $ACFA), 202581 (b6 $9745), 225269 (b6 $EFE5). Region ids 1657-1661, conf 0.5.
- Evidence-backed classification: 262,155 / 262,160 bytes (99.998%).
  Manual fallback: 5 bytes. Map coverage: 262,160 / 262,160 (0 unknown).
- Final checkpoint: pass-005 (maps/pass-005.json == maps/current.json == analysis/state.json;
  analysis/pass-005-verification.json ok, byte_identical).
- Commentary: re-prepared on pass-005; final strict editorial audit PASSED
  (commentary/audit.json: ok, byte_identical, 1661 regions, 1761 units, editorial_audit).
- Record: analysis/handoffs/round-05-manual-fallback.md. No other blockers; archive not run.
