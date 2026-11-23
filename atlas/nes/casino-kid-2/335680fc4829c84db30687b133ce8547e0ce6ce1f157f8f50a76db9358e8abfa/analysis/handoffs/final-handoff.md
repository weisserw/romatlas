# Final coordinator handoff (terminal)

Date 2026-11-23. Terminal state of the goal.md coordinator workflow after round 9.

## Result
- ROM classification: **100.00% (131088 / 131088 bytes classified; 0 bytes unknown).**
- Checkpoint **pass-009**: maps/pass-009.json == maps/current.json == analysis/state.json ==
  commentary/phase1-state.json (sha256 18763c395f6636b1f8d9c827708e98b059ebca665921b90ebc6835c154f95de1);
  analysis/pass-009-verification.json ok, byte_identical true. input/game.nes unchanged (sha256 335680fc...abfa).
- Regions 2,590 (1,670 code, 902 data, 18 padding). Observed code 26,839 B (part from artificial RAM-patched
  scenarios S16/S17, recorded as such; not natural-reachability claims). Remaining code is Static with structural
  evidence; reachability is recorded separately and not claimed.
- Commentary: final editorial acceptance done in round 9. `commentary.py audit` ok, byte_identical true,
  2,590/2,590 regions and 2,042/2,042 units reviewed, editorial_audit reviewed (commentary/audit.json; game.asm
  7efbc797...9651, symbols.asm 7db4abb2...5def). Reading copy commentary/reading-after.asm refreshed by the audit.
- Gameplay checklist analysis/scenarios.json S01-S17 (natural ending S13, game over S16, scripted final-match deal
  S17; S15 full credits scroll deferred, no structure depends on it).

## Why the workflow stopped
- Round 8 redirect (analysis/handoffs/round-08-redirect.md) produced a rejection review for structural research;
  the coordinator inspected it (decisive artifacts in analysis/redirect/r08-a1/: FE=4 sound re-walk identical, no
  $FE byte present; weak-region census with no new caller/reader/trace hit; padding review r06-a1 on unchanged
  premises) and found no corrected inference or discriminating test. Its two metadata/prose leads were applied in
  round 9 (R8-L043 map notes, R8-L044 commentary names). Round 9 changed no structure (metadata/prose only), so the
  sound review is reused on unchanged premises and no further redirect cycle was run.
- analysis/leads.json: 44 leads, none ready (38 verified, others superseded/partially addressed).

## Residual limitations (not classification gaps)
- Weakest structural claims: b4 $BB30-$BB31 Static code 0.5 (unreferenced leftover load); unreferenced code
  fragments with sibling-shape evidence; b2 $BB1F-$BB4A generic unpointed data; 18 Padding fill regions.
- Open meanings (commentary/uncertainties.md, Round 1-9 sections): e.g. plane flight-table step bytes, world-map
  info-panel marks, roulette wheel-strip order/board geometry, blackjack hand status 3, PwdVar057B (never read),
  PkrHandFlag values 2/3 never stored.
- Acceptable editorial residue: generic local branch labels inside explained routines, local numeric immediates,
  B6_Stream_XXXX sound-stream labels.
- Toolkit notes: analysis/toolkit-issues.md (TK-001..TK-006 plus performance note; no toolkit edits made).

## Archive preparation
archive-options.json lists additional final documents and omitted scratch/superseded copies (see that file).
Archive execution and workspace cleanup were NOT performed (not authorized).

archive-options.json rationale: documents = all 27 phase/final handoffs (analysis/handoffs/*.md) plus the 8
preserved per-round commentary progress logs (analysis/round-N/commentary/progress.md, referenced by
commentary/progress.md "Earlier rounds"). Omitted (33): per-round copies of toolkit-provided AGENTS/prompt/workflow
boilerplate, per-round uncertainties.md snapshots (superseded by the cumulative commentary/uncertainties.md, which
keeps every round's section), and analysis/redirect/r01-a1/unknown-ledger.md (pass-001 unknown ledger, superseded:
0 unknown bytes; its dispositions are summarized in round-01-redirect.md and investigation.md). Built-in final
documents (progress, investigation, gameplay, commentary progress/uncertainties) are selected by archive.py itself.
