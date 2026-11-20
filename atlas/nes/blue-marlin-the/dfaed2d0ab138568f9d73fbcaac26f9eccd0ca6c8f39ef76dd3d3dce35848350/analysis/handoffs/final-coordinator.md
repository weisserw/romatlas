# Final coordinator handoff — terminal INCOMPLETE (workflow terminated; ROM classification NOT complete)

Date: 2026-11-20. Game: The Blue Marlin (USA), NES mapper 4 (MMC3), ROM sha256 dfaed2d0…8350 (input unchanged).

## Result
- ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown). The rounded 100.00% is NOT completion.
- Final checkpoint: pass-004 — maps/pass-004.json (= maps/current.json = analysis/state.json export, map sha256 4582999d…),
  generated/pass-004, analysis/pass-004-verification.json (ok, byte_identical true).
- Residual unknown (exact): bank 1 file 0x02DC6 ($ADB6), 0x032FA ($B2EA), 0x03353 ($B343) — single $60 (RTS) bytes after tail JMPs.
- Commentary: commentary/source (built from commentary/notes/annotations.txt via analysis/tools/commentary_build.py);
  commentary/audit.json ok + byte_identical + editorial_audit true (3272 regions, 1081 units reviewed);
  reading copy commentary/reading-after.asm; editorial audit record commentary/editorial-audit.md;
  uncertainties commentary/uncertainties.md. Known non-blocking limits: 482 generic data regions carry only
  "format not identified" notes (permitted generic data); 343 named stubs/continuations lack separate headers.

## Rounds (sequential phase workers)
| Round | Disassembly | Commentary | Redirect |
|---|---|---|---|
| 1 | pass-001 93.23% (17738 unk) | done (+continuation: numbered generic labels removed) | r01-a: ready L-004/005/009/010/012/013/015/016 |
| 2 | pass-002 99.60% (1044 unk) | done (round-1 preserved analysis/round-1/) | r02-a: ready L-019/L-020; dummy-read method correction |
| 3 | pass-003 (5 unk) | done (analysis/round-2/) | r03-a: ready L-022 (NOPs), L-023 (RTS) after coordinator corrected-test questions |
| 4 | pass-004 (3 unk): L-022 accepted, L-023 rejected | done (analysis/round-3/) | r04-a: runtime physical read-watch, no ready lead; final rejection review |
Final acceptance: analysis/handoffs/final-acceptance.md (editorial audit passed).

## Coordinator rejection-review assessment
Reviewed analysis/handoffs/round-04-redirect.md and decisive artifacts (analysis/redirect/r04-a/readwatch.txt: 0 Read/
Instruction/DMA events at the 3 physical offsets over 9,714 traced frames incl. fight runs where the preceding tail JMPs
executed thousands of times; r03-a refcheck/twins/rtsdisp/splitcons/bankrel: no reference in any encoding, twin test
negative; analysis/evidence/round-04/xref.txt; round-04-disassembly rejection). evidence-policy.md: "An unreferenced
return opcode … still lacks sufficient structure on its own." Dead-RTS vs stray byte remains materially ambiguous; no
corrected inference or discriminating test remains. Accepted as justified terminal incomplete; do not relaunch on
unchanged premises. Reopen only on new evidence (a reference/consumer, or an accepted evidence convention).

## Gameplay
All milestones S01–S16 in analysis/scenarios.json visited (later ones via documented RAM patches: artificial scenarios,
not natural reachability), incl. landing/weigh-in, end of day/results/password, Hawaii, ending (FIN).

## Toolkit
analysis/toolkit-issues.md: TK-001 (input API param mismatch, confirmed bug), TK-002..TK-004 (missing features); none blocking.

## Archive readiness
archive-options.json written (documents / omitted_documents). No archive, cleanup or publication performed.

---

## Addendum (2026-11-20): user-directed residual fallback — SUPERSEDES the residual/checkpoint lines above
The text above is retained as the historical terminal-incomplete record. Afterwards the batch user explicitly authorized a
batch-only exception (at most 10 residual bytes) to mark the remaining bytes as generic Data of unknown meaning with Manual
provenance. Full record: analysis/handoffs/final-fallback.md.

- Complement re-verified first: exactly the 3 bytes above (0x02DC6, 0x032FA, 0x03353; matches the user list).
- Regions 3315-3317: Data / Manual / confidence 0.0, no label. **Not** an observed or statically proven data role; the
  code-versus-data question (L-023 rejected; no reference; 0 runtime accesses) remains undetermined by evidence.
- Final checkpoint: **pass-005** — maps/pass-005.json (= maps/current.json = analysis/state.json, sha256 e401cd2f…),
  generated/pass-005, analysis/pass-005-verification.json: ok, byte_identical true. ROM unchanged.
- Classification: 0 unknown bytes. **Evidence-backed: 262,157 bytes** (Observed/Static). **Manual user fallback: 3 bytes**,
  reported separately. Code 30,903 (Obs 20,357 / Static 10,546), Data 78,810 (Static 78,807 / Manual 3), Text 13,377,
  Padding 7,998, Image 131,072.
- Commentary round 5 (analysis/handoffs/round-05-commentary.md): round-4 commentary preserved in analysis/round-4/;
  carried to pass-005 (only the 3 unknown notes replaced by honest Manual-fallback notes, no invented semantics);
  commentary.py verify ok/byte_identical (game.asm sha256 e06730f4…); strict editorial audit passed
  (commentary/audit.json: ok, byte_identical, editorial_audit true; 3,272 regions / 1,081 units reviewed).
- Not produced (no defined workspace command): analysis/annotations.json, analysis/current-audit.json, maps/current.sym,
  commentary/baseline.sym (NES uses JSON maps); archive.py copies FINAL entries only if present.
- archive-options.json updated: final-fallback.md, round-05 commentary handoff and the round-4 commentary archive are
  selected; round-4 copied AGENTS/prompt/workflow omitted. All analysis/ and commentary/ Markdown accounted for.
  No archive, cleanup, publication or toolkit change performed.
