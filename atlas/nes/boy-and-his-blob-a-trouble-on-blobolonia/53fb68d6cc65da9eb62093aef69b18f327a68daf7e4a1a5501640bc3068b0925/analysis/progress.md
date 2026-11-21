# Disassembly progress

## Coordinator state

- Round: 3
- Current phase: TERMINAL handoff (whole-ROM classification + final editorial audit gates satisfied) — 2026-11-21
- Final checkpoints: map pass-004 (maps/current.json identical); commentary-003 (commentary/source/game.asm sha256 3cdcb947…7131)
- Phase handoffs: analysis/handoffs/ (one file per round/phase)

## Final coordinator handoff

- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown) — pass-004
  (Code 25172, Data 217410 incl. header+CHR, Text 116, Padding 19462).
- Verification: analysis/pass-004-verification.json (ok, byte_identical);
  commentary/audit.json (ok, byte_identical, 1365 regions / 1453 units reviewed, editorial_audit true).
- Lowest-confidence claims (documented, not reachability claims): b0 $9AB8 orphan RTS Code (0.6),
  b0 $BA42–$BA43 generic data (0.7), b1 $85E0 AddAToZpWord Code (0.75, no caller).
- Gameplay: title→play, Blobolonia, caves, underwater, pharmacy, treasure, rocket, room $CE key
  sequence, ending + post-ending restart reached (several via RAM edits). Not reached naturally:
  bean delivery to caged blob in room $CF (natural ending trigger). Semantic uncertainties in
  commentary/uncertainties.md. Toolkit issues TK-001..TK-003 in analysis/toolkit-issues.md (non-blocking).
- Coordinator inspected round-3 redirect rejection review: residuals are confidence/reachability/semantic
  only, no corrected inference or discriminating test survives; no further cycle.
- archive-options.json written (handoffs, redirect ledgers/reviews, prior-round commentary progress and
  uncertainties selected; regenerated toolkit instruction copies under analysis/round-*/commentary omitted).
  No archive run, cleanup or publication performed.

## Coordinator log

| Round | Phase | Input checkpoint | Output checkpoint | Verification | Leads consumed/produced | Outcome |
|---|---|---|---|---|---|---|
| 1 | disassembly | none | pass-002 (pass-001 superseded) | byte_identical true | produced L-001..L-009 | done; 98.48% (258166/262160; 3994 unknown); handoff analysis/handoffs/round-01-disassembly.md |
| 1 | commentary | pass-002 | commentary-001 (commentary/source/game.asm) | commentary.py verify byte_identical true | consumed L-002,L-005,(L-009 partial); produced L-010..L-016 | COMPLETE; all classified source covered; handoff analysis/handoffs/round-01-commentary.md |
| 1 | redirect | pass-002 / commentary-001 | (no map change) | n/a | produced ready L-010..L-021 (3993 B) | CYCLE; residual $9AB8 (1 B); handoff analysis/handoffs/round-01-redirect.md |
| 2 | disassembly | pass-002 | pass-003 | byte_identical true | consumed L-007,L-010..L-021; produced L-022,L-023 | done; 100.00% (262159/262160; 1 unknown: bank0 $9AB8); handoff analysis/handoffs/round-02-disassembly.md |
| 2 | commentary | pass-003 | commentary-002 | byte_identical true | consumed redirect-r1 fixes; produced L-024 | COMPLETE; all classified source covered; handoff analysis/handoffs/round-02-commentary.md |
| 2 | redirect | pass-003 / commentary-002 | (no map change) | n/a | L-023 ready (conf 0.6), L-024 ready (203 B padding->data), L-022 reachability unknown | CYCLE; post-ending + rocket reached; natural ending trigger not reached; handoff analysis/handoffs/round-02-redirect.md |
| 3 | disassembly | pass-003 | pass-004 | byte_identical true | consumed L-023 (accepted, conf 0.6, no reachability claim), L-024 | done; 100.00% (262160/262160; 0 unknown); handoff analysis/handoffs/round-03-disassembly.md |
| 3 | commentary | pass-004 | commentary-003 | commentary.py audit ok, byte_identical true, 1365 regions/1453 units reviewed, editorial_audit true | consumed L-023,L-024 notes; none produced | COMPLETE + final editorial audit; handoff analysis/handoffs/round-03-commentary.md |
| 3 | redirect | pass-004 / commentary-003 | (no map change) | boundary sweep + coverage cross-check, no contradictions | none produced; L-006,L-008 deferred | TERMINAL; rejection review in analysis/handoffs/round-03-redirect.md (coordinator-inspected, sound) |

## Worker notes

Workspace initialized. Baseline and classification have not run.

## Worker notes — round 1 disassembly

- Baseline created and verified (maps/baseline.json, generated/baseline, byte_identical).
- Output checkpoint: pass-002 (byte_identical: true; analysis/pass-002-verification.json). pass-001
  (same pass, superseded) retained.
- ROM classification: 98.48% (258166 / 262160 bytes classified; 3994 bytes unknown).
- Code 25146 / Data 213355 (incl. header 16 + CHR 131072) / Padding 19665 / Unknown 3994 bytes.
  Per-bank table and evidence: analysis/handoffs/round-01-disassembly.md; method: analysis/investigation.md.
- Execution coverage: 9704 unique instruction addresses (natural + artificial warp scenarios).
- Verification: generated/pass-002 rebuilds byte-identical.
- Leads produced L-001..L-009; none consumed. Toolkit findings: TK-001..TK-003 (analysis/toolkit-issues.md), no blockers.
- Note: generated/pass-001 was briefly deleted during cleanup and regenerated from maps/pass-001.json
  with the configured binary (verify: byte_identical true).
- Status: incomplete (3994 unknown bytes remain).

## Worker notes — round 1 commentary

- Status: COMPLETE (current-source readability for all classified source; classification unchanged,
  98.48% / 3994 unknown bytes).
- Prepared commentary/ with toolkit commentary.py from pass-002; output checkpoint commentary-001
  (commentary/source/game.asm, sha256 3ed7f088...be2), generated reproducibly by
  analysis/commentary-tools/cmt.py from commentary/before + notes/*.txt.
- Verification: byte_identical true (commentary/verification.json; analysis/commentary-001-verification.json);
  reading copy commentary/reading-after.asm refreshed. review.json: 1359/1359 entries reviewed with
  summary/source, 372 units; editorial_audit pending (final gate).
- Leads consumed L-002, L-005 (L-009 partial); produced L-010..L-016 (classification of table,
  DPCM and PCM ranges identified during commentary).
- No toolkit issues found in this phase (see analysis/toolkit-issues.md).
- Handoff: analysis/handoffs/round-01-commentary.md.

## Worker notes — round 1 redirect

- Status: COMPLETE; recommendation CYCLE. Attempt analysis/redirect/r01-a/; handoff analysis/handoffs/round-01-redirect.md.
- Input pass-002 / commentary-001; no map/source/state/toolkit changes (classification still 98.48%, 3994 unknown).
- Every unknown range has a disposition (analysis/redirect/r01-a/ledger.md): 3993 bytes proposed (3991
  consumer/structure-backed, 2 pattern-only), 1 byte residual (bank 0 $9AB8), plus Data->Code correction b7 $C088-$C08F.
- Leads: updated L-010..L-016 (verified, split into subleads), new L-017..L-021, consumed L-003/L-004,
  superseded L-001; previous file kept as analysis/leads.pre-redirect-r1.json.
- Milestones (artificial RAM shortcuts): pharmacy S10, ending S11 (Emperor scene, bank-5 PCM ran), treasure S12 visited.
- Toolkit: no new toolkit issues (TK-001..TK-003 stand; TK-001 buttons-array workaround used).
- Scratch directory removed; durable evidence (coverage gz, logs, 3 PNGs) in r01-a/evidence (~0.6 MB).

## Worker notes — round 2 disassembly

- Status: COMPLETE pass; ROM classification incomplete (1 unknown byte). Handoff: analysis/handoffs/round-02-disassembly.md.
- Input pass-002 (analysis/state.json); output checkpoint **pass-003** (maps/pass-003.json, generated/pass-003),
  verification ok/byte_identical true (analysis/pass-003-verification.json, map_sha256 b5c79502...b7d01a).
- ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown).
- Code 25171 / Data 217207 (incl. header 16 + CHR 131072) / Text 116 / Padding 19665 / Unknown 1.
  Per bank unknown: bank 0 = 1 ($9AB8), all other banks/CHR/header 0. Per-bank table in the handoff.
- Leads consumed: L-007, L-010..L-021 (all verified by analysis/pass-003/verify_leads.py, 28/28 PASS), L-022 (new).
  Produced: L-022 (b1 $85E0 Data->Code AddAToZpWord), L-023 (residual $9AB8, terminal review). Rejected: none;
  L-007 candidates $8D22/$883D rejected as code. Corrections: b7 $C088-$C08F and b1 $85E0-$85E7 Data->Code.
- Pattern-only decision: b0 $BA42-$BA43 generic data (conf 0.7).
- No gameplay/emulator runs this pass (no remaining unknowns needed runtime evidence). No new toolkit issues
  (TK-001..TK-003 stand; analysis/toolkit-issues.md). Evidence details: analysis/investigation.md (Round 2 section).

## Worker notes — round 2 commentary

- Status: COMPLETE (current-source readability for all classified source; classification unchanged, 262159/262160,
  1 unknown byte b0 $9AB8). Handoff: analysis/handoffs/round-02-commentary.md.
- Map changed since commentary-001, so round-1 commentary preserved under analysis/round-1/ (commentary/ and
  commentary-tools/) and commentary/ re-prepared from pass-003; notes carried by bank + CPU address via
  analysis/commentary-tools/cmt.py (now Text-aware, emits table-continuation pointers, splits units per scaffold entry).
- Output checkpoint commentary-002 (commentary/source/game.asm sha256 ebbee43d...e5f2); verify byte_identical true
  (analysis/commentary-002-verification.json); reading copy refreshed; review.json 1365/1365 reviewed, 396 units,
  editorial_audit pending.
- All new pass-003 regions documented; redirect r1 fixes applied; 69 generic branch labels renamed; misleading names
  and a '&BridgeAtAnchor' prose artefact fixed. Leads: L-005 correction noted, L-023 attempt appended, new L-024
  (Padding->Data role precision, bank 6). No toolkit issues found in this phase.

## Worker notes — round 2 redirect
- Phase: redirect, attempt analysis/redirect/r02-a; input checkpoint pass-003 (map sha256 b5c79502...b7d01a) /
  commentary-002. No map, state, source, ROM or toolkit edits; leads.json backed up to analysis/leads.pre-redirect-r2.json.
- Method change vs round 1/disassembly r2 for L-023: combined independent constraints instead of reference search
  alone (evidence-policy "newly combined independent evidence"); outcome: ready proposal Code/Static conf 0.6.
- L-024 recipe verified (l024_check.log); L-022 reachability negative (static, l022_check.log).
- Gameplay: S11b post-ending restart visited; S09 root beer rocket visited; S13 jack ending trigger attempted
  (blocked by the caged-blob room $CF setup). First post-ending run (traced throughout) stopped at its 900 s
  wall-clock limit; revised run traces only the final chunk. One rocket run was interrupted at t20 before its
  coverage dump and rerun (deterministic, identical log prefix).
- Scratch directory removed after promoting gzip coverage to r02-a/evidence; ~3.5 GB free.
- Recommendation: CYCLE (L-023 + L-024 to disassembly). Handoff: analysis/handoffs/round-02-redirect.md.

## Worker notes — round 3 disassembly

- Status: COMPLETE pass; all bytes classified. Input pass-003 (analysis/state.json); output checkpoint **pass-004**
  (maps/pass-004.json, generated/pass-004), verification ok/byte_identical true (analysis/pass-004-verification.json,
  map_sha256 119004fa...d55690 = analysis/state.json = maps/current.json). Handoff: analysis/handoffs/round-03-disassembly.md.
- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
- Code 25172 / Data 217410 (incl. header 16 + CHR 131072) / Text 116 / Padding 19462 / Unknown 0. Per bank unknown: 0 everywhere
  (per-bank table in analysis/pass-004/stats.log and the handoff).
- Leads consumed: L-023 (accepted by reviewer: b0 $9AB8 Code/Static 0.6, UpdateBoyStop_OrphanRts, no reachability claim),
  L-024 (25 bank-6 regions / 203 B Padding -> Data). Rejected: none. Evidence: analysis/pass-004/, investigation.md Round 3.
- No emulator/gameplay runs this pass (no unknown bytes required runtime evidence; scenarios.json unchanged).
- No new toolkit issues (TK-001..TK-003 stand; analysis/toolkit-issues.md).
- Completion gates still open: commentary must document the changed regions; editorial_audit pending. Not declared complete.

## Worker notes — round 3 commentary
- Status: COMPLETE (current-source readability + final editorial acceptance gate run). Input pass-004 (map sha256
  119004fa...d55690); output checkpoint **commentary-003** = commentary/source/game.asm (sha256 3cdcb947...7131).
- Round-2 commentary preserved in analysis/round-2/ (commentary/ + commentary-tools/) before re-preparing from pass-004.
- `commentary.py audit`: ok, byte_identical true, reviewed_regions 1365, reviewed_units 1453, editorial_audit true
  (commentary/audit.json, analysis/commentary-003-audit.json, analysis/commentary-003-verification.json); reading copy refreshed.
- Changed regions documented ($9AB8 orphan RTS; 25 bank-6 Data runs); 28 prose fixes incl. 7 old numeric-value->RAM-name
  artefacts. No map/state/ROM/toolkit edits. No toolkit issues found. Handoff: analysis/handoffs/round-03-commentary.md.

## Worker notes — round 3 redirect
- Phase: redirect, attempt analysis/redirect/r03-a; input pass-004 (map sha256 119004fa...d55690) / commentary-003
  (3cdcb947...7131). No map, state, source, ROM or toolkit edits; backups analysis/leads.pre-redirect-r3.json,
  analysis/redirect/r03-a/scenarios.pre-r03.json.
- Sweep (sweep/sweep2/covcheck/covdiff logs): no code/data boundary contradiction; low-confidence items unchanged.
- Gameplay: S14 room $CE key transition visited (artificial start, natural play); S13 ending trigger visited with an
  artificial eat-state injection (natural jack delivery into the cage unknown). E2 run hit its wall-clock limit before
  coverage export (log kept). Scratch removed; ~3.5 GB free.
- Leads: none added; L-006/L-008 ready -> deferred. Recommendation: TERMINAL. Handoff: analysis/handoffs/round-03-redirect.md.
