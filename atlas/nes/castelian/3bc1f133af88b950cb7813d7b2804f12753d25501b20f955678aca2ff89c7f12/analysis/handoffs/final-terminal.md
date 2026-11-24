# Final coordinator handoff — terminal INCOMPLETE (2026-11-24)

Workflow (goal.md) reached its documented terminal stop after 3 rounds of
disassembly -> commentary -> redirect. This is workflow termination, NOT whole-ROM
completion; ROM classification is not complete.

- ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown) — rounded.
- Residual unknown bytes (lead L2-01): bank 3 CPU $B6B0 (phys 63168, `$60`) and
  bank 4 CPU $B685 (phys 79509, `$EA`). Rejection review: analysis/redirect/r2-a1/notes.md
  (§L2-01), reused on unchanged premises in analysis/redirect/r3-a1/notes.md. Coordinator
  inspected it: no reference (incl. split/RTS/vector tables), no format membership, no
  ROM-wide pad or jmp+rts convention; code and padding readings both predict non-execution.
  Reopen only with a new reference, original listing or ROM-wide convention.
- Final checkpoint: pass-003 (maps/pass-003.json == maps/current.json == commentary/baseline.json;
  analysis/pass-003-verification.json ok, byte_identical). analysis/state.json saved.
- Commentary: current-source readability COMPLETE on pass-003 (review.json 601/601 reviewed);
  commentary/verification.json ok + byte_identical; reading copy commentary/reading-after.asm.
  Final strict editorial audit (commentary/editorial-audit.md / audit.json) NOT yet run — reserved
  for final acceptance/archive. Prior commentary rounds preserved at analysis/round-1/, analysis/round-2/.
- Gameplay: all checklist milestones visited (analysis/scenarios.json), incl. 8 towers, bonus stage,
  ending, demo mode, PLAYER 1/2 WINS, HERO difficulty (some via RAM patches; recorded as artificial).
- Leads: L1-01..L1-09, L2-02 consumed/accepted; L2-01 terminal (see above). No ready leads.
- Static-only code with no known caller: $A019/$A0EC, $E4B3, $E585 (conf 0.6). Bank 3
  $A500-$AEFF data by pattern only (no reader). Open meanings: commentary/uncertainties.md.
- Toolkit issues: analysis/toolkit-issues.md (TK-001..TK-005; none blocking).
- Round handoffs: analysis/handoffs/round-{1,2,3}-{disassembly,commentary,redirect}.md.
- Archive selection: archive-options.json (no archive executed; no cleanup done).

---

## Addendum — final user-directed manual fallback and audit (2026-11-24)

The terminal-incomplete record above is kept unchanged as history. Afterward, the user gave an
explicit batch-only authorization covering at most 10 residual unclassified bytes:

- Before any edit, the exact unknown complement was confirmed on pass-003. It was 2 bytes: phys 63168
  (16K bank 3 CPU $B6B0) and phys 79509 (16K bank 4 CPU $B685), which is lead L2-01.
- Both bytes were marked as generic **Data, meaning unknown, provenance Manual** (region ids
  1966 and 1967) by analysis/evidence/final-fallback/tools/apply_fallback.py. The change log is in
  analysis/evidence/final-fallback/changes.json.
- This marking is a **user-directed fallback, not evidence-backed classification**. No observed or
  statically proven data role exists. The prior ambiguity is unchanged: the L2-01 review in
  analysis/redirect/r2-a1/notes.md found no reference, no format membership and no pad/jmp+rts
  convention, so the code vs padding question is still open.
- New checkpoint **pass-004**: maps/pass-004.json == maps/current.json, sha256 c6868c2f...2f43.
  analysis/pass-004-verification.json reports ok and byte_identical. ROM sha256 is unchanged (3bc1f133...).
- Classification is counted in separate groups:
  - evidence-backed (Observed/Static): 131086 / 131088 bytes
  - manual fallback: 2 bytes
  - unknown: 0
- Commentary: the round-3 commentary is preserved at analysis/round-3/commentary/. A fresh baseline was
  built from pass-004 with carry-over, and the final review fixes are listed in
  analysis/handoffs/final-commentary-audit.md. The **strict editorial audit PASSED**: ok,
  byte_identical, 601/601 regions and 857/857 units reviewed, editorial_audit true. Source
  game.asm sha256 is 65b0b106...451c.
- Whole-ROM *evidence-backed* classification is still not complete: 2 bytes rest only on the manual
  fallback. Open meanings are listed in commentary/uncertainties.md.
- Disposable scratch: scratch/commentary-r3-working is an identical duplicate of
  analysis/round-3/commentary/. It is not deleted, because no cleanup was authorized.
- Archive selection: archive-options.json. No archive was executed.
