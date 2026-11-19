# Final coordinator handoff — terminal incomplete (2 bytes unknown)

> **Superseded status (2026-11-19, round 5):** the user-authorized manual fallback and the passing final audit are recorded in
> "Round 5 final update" at the end of this file. The round-4 text below is kept as the historical record.


Date: 2026-11-19. Workflow: goal.md (disassembly -> commentary -> redirect), rounds 1-4.
Status: **workflow terminal; ROM classification INCOMPLETE** (not whole-ROM complete).

## Classification
ROM classification: 100.00% (262158 / 262160 bytes classified; 2 bytes unknown).
Exact value is 99.9992%; the rounded 100.00% does not mean completion.

- Only unknown range: file 0x1C5AE-0x1C5AF (PRG bank 14, fixed at CPU $C59E-$C59F),
  lead L-018, status `exhausted-unknown`.
- Code 41,123 B (28,697 Observed, of which part is artificial RAM-patched scenario
  evidence; 12,426 Static). Per-bank counts: analysis/handoffs/round-002-disassembly.md
  (unchanged since round 2; rounds 3-4 changed provenance only).

## Final checkpoint and verification
- Map checkpoint pass-004: maps/pass-004.json = maps/current.json = analysis/state.json;
  analysis/pass-004-verification.json ok, byte_identical (map sha256 d6e11e81...44b9).
- Commentary (round 4): commentary/source/ generated from commentary/notes/ by
  commentary/tools/annotate.py; commentary/verification.json ok, byte_identical;
  reading copy commentary/reading-after.asm refreshed. No mechanical labels remain;
  review.json 2,195/2,195 entries reviewed. Handoff: analysis/handoffs/round-004-commentary.md.
- Prior commentary snapshots preserved: analysis/round-1/, round-2/, round-3/.
- **Final editorial audit is still pending** (reserved for final acceptance/archive);
  commentary/editorial-audit.md does not exist yet.

## Terminal rationale (rejection review)
- L-018 reviews: analysis/redirect/r2-a1/l018-review.json and
  analysis/redirect/r3-a1/rejection-review.json. No static references (word, address-1,
  immediate-pair, branch) in any bank; never fetched in any trace (pass-001, r1-a1, r1-a2,
  r2-a1, r3-a1); the bytes follow an unconditional indirect jump and decode as a zero-page
  load whose value is dead on every path into the following routine, so no entry contract
  supports code; content is a plausible instruction rather than fill/pattern, so generic
  data is unsupported. Under evidence-policy.md the bytes stay unknown.
- Coordinator review: premises checked against primary artifacts; round 4 changed only
  provenance and comments, so the round-3 review applies unchanged and redirect was not
  relaunched. Reopen only with an observed fetch/edge to $C59E, a computed target
  producing $C59E/$C59D, or another build of the same engine showing the layout.
- Other open items are semantic or about reachability and cannot change classification
  (commentary/uncertainties.md): reachability of orphan routines, weak-role data blocks
  (bank-15 sound-like block, sound bytes after endless loops, bank-3 records, duplicate
  bank tails), approximate area-map byte formats, SetupBossSlotsArea17 caller.

## Gameplay coverage
analysis/scenarios.json milestones S1-S8 visited, including the ending/credits, the game-over
continue screen, weapons, and boss defeats and area clears. Boss-area clears were
reached through RAM patches (artificial), not by normal play.

## Leads
analysis/leads.json: L-001..L-020 consumed/superseded/verified; L-018 exhausted-unknown.
No ready leads remain.

## Handoff chain
round-001-{disassembly,commentary,redirect,redirect-a2}.md, round-002-{disassembly,
commentary,redirect}.md, round-003-{disassembly,commentary,redirect}.md,
round-004-{disassembly,commentary}.md (all in analysis/handoffs/).

## Toolkit
analysis/toolkit-issues.md: TK-001..TK-003 (unchanged, none blocking).

## Archive preparation
archive-options.json selects the handoffs and this file as final documents. It omits
the superseded round-N commentary snapshot Markdown (copies of instruction files, and
progress/uncertainty notes superseded by the current commentary/ versions). The
snapshots themselves stay in the workspace. No archive was run.


## Round 5 final update (2026-11-19) — user-authorized manual fallback and final acceptance

Status: **workflow terminal.** All 262,160 bytes now have a map classification. That total includes a 2-byte manual fallback
that the user authorized. The bytes are **not** all classified from evidence, so this is not an evidence-backed whole-ROM
completion under goal.md.

### Map classification
- 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
- **Classified from evidence: 262,158 B.** This covers code, data, text and graphics, backed by Observed (including artificial)
  or Static evidence. The counts are unchanged since round 2.
- **Manual fallback, reported separately: 2 B** at file 0x1C5AE-0x1C5AF (offset 116142, length 2; PRG bank 14, CPU $C59E-$C59F).
  - The map marks these bytes as generic **Data** with **Manual** provenance, confidence 0.5, no label and unknown meaning (region 2265).
  - This follows the user's explicit batch-only instruction. That instruction applies only when at most 10 bytes remain
    unclassified, and it overrides the structural-proof requirement for these residual bytes only.
  - The bytes were neither observed nor statically proven to be data.
  - The earlier ambiguity remains. The L-018 reviews (analysis/redirect/r2-a1/l018-review.json,
    analysis/redirect/r3-a1/rejection-review.json) found no static references and no fetches. The bytes follow an unconditional
    indirect jump and would decode as a dead zero-page load. The evidence supported neither code nor fill-pattern data.
  - L-018 keeps the evidence status exhausted-unknown, with round-5 history appended. No redirect research was reopened.
- No ROM bytes were modified, and the input/game.nes sha256 is unchanged.

### Checkpoint and verification
- The map checkpoint **pass-005** is saved as maps/pass-005.json, which matches maps/current.json and analysis/state.json.
  - Map sha256: a320e4a3…2dba.
  - analysis/pass-005-verification.json reports ok and byte_identical.
  - `romatlas store validate` and `romatlas map validate` both reported ok on 2026-11-19.
- Commentary round 5: the round-4 commentary is preserved in analysis/round-4/. A fresh baseline was built from pass-005, with
  carried names, comments and review 1:1. The final editorial review is in commentary/editorial-audit.md.
- **Strict editorial audit PASSED.** The coordinator re-ran `commentary.py audit` and got ok, byte_identical, 2,195/2,195
  regions reviewed, 1,322 unit records reviewed and editorial_audit=true. The reading copy was refreshed.
- Handoffs: analysis/handoffs/round-005-disassembly.md and analysis/handoffs/round-005-commentary.md.

### Still open
- Semantic uncertainties are listed in commentary/uncertainties.md.
- Toolkit issues TK-001..TK-003 are unchanged and none are blocking.
- No archive was run. archive-options.json is prepared for retention.
