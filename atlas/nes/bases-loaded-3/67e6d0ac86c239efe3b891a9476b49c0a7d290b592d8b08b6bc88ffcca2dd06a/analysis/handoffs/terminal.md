# Terminal handoff — justified terminal incomplete (2026-11-18)

Workflow state: the coordinator workflow (goal.md) reached its documented terminal handoff after
3 rounds (disassembly → commentary → redirect) plus final editorial acceptance. This is a
**terminal incomplete** for ROM classification. The workflow ending does not make the ROM complete.

- ROM: Bases Loaded 3 (MMC3 / mapper 4, PRG 256 KiB, CHR 128 KiB), sha256 67e6d0ac…dd06a (unchanged).
- ROM classification: 100.00% (393230 / 393232 bytes classified; 2 bytes unknown).
  A rounded 100.00% is not completion.
- Unknown bytes: 28:$8620 (file 0x38630) and 28:$9883 (file 0x39893). Each is a lone RTS
  following another RTS. No static edge, reader, pointer, stack-dispatch or observed fetch reaches
  either one. Rejection review: L-015 in handoffs/round-002-redirect.md and round-002-redirect-b.md
  (evidence analysis/redirect/r002-a/, r002-b/). The coordinator re-checked it against the primary
  bytes and evidence-policy.md ("an unreferenced return opcode … lacks sufficient structure") and
  judged it sound. Its one premise error, at 23:$B402 (the static fall-through of a `bcc`), was
  corrected and classified as Code (L-017, pass-003). No corrected inference or discriminating test remains.
- Final checkpoint: pass-003 (maps/pass-003.json = maps/current.json, generated/pass-003,
  analysis/pass-003-verification.json: ok, byte_identical true). Internal store: analysis/state.json.
- Commentary: commentary/source/game.asm and the reading copy commentary/reading-after.asm.
  `commentary.py audit` returned ok, byte_identical true, 1232/1232 regions and 1937 units
  reviewed, editorial_audit true (commentary/audit.json, commentary/editorial-audit.md,
  handoffs/final-editorial.md). The remaining explained uncertainties are listed in
  commentary/uncertainties.md: object 10 = umpire is tentative, wall sections 2–4 come from
  code reading only, and the clock/BootABFlag/some counters are unknown. About 180 headers are
  terse but accurate, and branch labels are generic.
- Leads: analysis/leads.json. L-005, L-007 to L-014, L-016 and L-017 are verified; L-001 to L-004
  are superseded; L-015 is rejected (the 2 bytes); L-018 is commentary-only (applied). L-006 is
  still "ready" but optional: it would only add Observed provenance to the EDIT/2P code and
  classifies no bytes (deferral reason in round-002-redirect.md).
- Gameplay: analysis/scenarios.json S-001 to S-015. Covered: WATCH, 1P, 2P, EDIT, extra innings,
  win/loss flows, rating screen, ending/credits (by RAM shortcut, S-012), HBP/brawl, home-run
  scenes and the scoreboard.
- Toolkit findings: analysis/toolkit-issues.md (TK-001 confirmed bug; TK-002 to TK-005 missing
  features or doc mismatches). None block the work.
- Round history and per-phase handoffs: analysis/progress.md, analysis/handoffs/*.md.
- Resume: only new evidence or a justified corrected inference about the 2 bytes reopens
  classification. Don't relaunch on the unchanged assessment. Archiving was not run (not authorized).

## Archive document selection (archive-options.json)

Final-selected: terminal/final-editorial handoffs, all per-phase handoffs (incl. L-015 rejection review) and redirect attempt notes. Auto-included by archive.py FINAL: analysis/investigation.md, gameplay.md, progress.md, commentary/progress.md, uncertainties.md, editorial-audit.md. Implicitly known: analysis/toolkit-issues.md, commentary/AGENTS.md, prompt.md, workflow.md. Omitted: round-1/round-2 commentary snapshot copies (tool-generated instruction copies, plus progress/uncertainties superseded by and carried into current commentary records); they remain in the workspace.
Every Markdown file in analysis/ and commentary/ (outside before/ and source/) is accounted for.

## Update 2026-11-18: user-directed manual fallback and final acceptance on pass-004-fallback

The text above is kept as the pass-003 record. The current state is:

- **Proven classification (evidence-backed):** 393230 / 393232 bytes. This is unchanged.
- **Manual user-directed fallback (reported separately):** 2 bytes, 28:$8620 (phys 230960, region
  1252) and 28:$9883 (phys 235667, region 1253).
  - Each is generic Data with Manual provenance, confidence 0.5, no label and meaning unknown.
  - The user explicitly authorized this as a batch-only exception for at most 10 residual bytes.
  - It is not an observed or statically proven role. The code-vs-data question for these bytes is
    still open, and L-015 stays rejected as an evidence claim.
  - Record: analysis/handoffs/manual-fallback.md.
- Unknown bytes: 0. Whole-ROM *evidence-backed* classification is still not complete: 2 bytes are
  manual fallback.
- Final checkpoint: **pass-004-fallback**.
  - maps/pass-004-fallback.json = maps/current.json = analysis/state.json, sha256 c4dd4ae8…3dec.
  - Output in generated/pass-004-fallback.
  - analysis/pass-004-fallback-verification.json: ok, byte_identical true.
- Commentary was rebased onto the new map; the pass-003 commentary is preserved at
  analysis/round-4/commentary/. The final review and strict editorial audit were rerun by the
  coordinator: `commentary.py audit` gave ok, byte_identical true, 1232 regions and 1937 units
  reviewed, editorial_audit true, source game.asm sha256 dd77d23d…c482.
  - Handoff: analysis/handoffs/final-editorial-pass-004.md.
- archive-options.json selects manual-fallback.md and final-editorial-pass-004.md for preservation.
  The round-4 commentary snapshot copies are omitted, because the current records carry them
  forward; they remain in the workspace. Every Markdown file in analysis/ and commentary/ is
  accounted for. The archive was not executed.
- No open redirect research remains for these bytes; the user directed that it not be resumed.
  Toolkit findings: analysis/toolkit-issues.md (one new minor commentary-rebase feature gap). The
  toolkit was not modified.
