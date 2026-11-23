# Disassembly progress

Workspace initialized. Baseline and classification have not run (initial state).

## Coordinator log

| Round | Phase | Status | Input checkpoint | Output checkpoint | Handoff | Verification | Leads consumed / produced | Outcome |
|---|---|---|---|---|---|---|---|---|
| 1 | disassembly | done (incomplete pass, as expected) | baseline | pass-001 | analysis/handoffs/round-01-disassembly.md | pass-001 byte-identical ok | produced L001-L005 | 99.88% (130936/131088; 152 unknown) |
| 1 | commentary | done (complete readability) | pass-001 | commentary/source (no map change) | analysis/handoffs/round-01-commentary.md | commentary verify byte-identical ok | consumed L005 (partial: data names); questions for L001/L003 | all 487 review entries reviewed, 401 units |

## Round 1 disassembly pass (2026-11-23) - incomplete pass, checkpoint pass-001

- Baseline created (maps/baseline.json, generated/baseline, analysis/baseline-verification.json: byte_identical true).
- Checkpoint pass-001: maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json
  (ok true, byte_identical true, map_sha256 b253e7dee5a06357e741383e7e130f4175fc047dd674b7ffd63edb97f406ee41).
- Internal map analysis/state.json = maps/current.json: 477 regions (55 Code, 422 Data).
- Per bank (classified / unknown bytes): header 16/0; bank0 32694/74 (Code 21125, Data 11569);
  bank1 32768/0 (Code 1365, Data 31403); bank2 32762/6 (Code 3891, Data 28871);
  bank3 32696/72 (Code 8704, Data 23992).
- Execution coverage (separate metric): 9478 observed ROM instruction starts, 90 RAM PCs; see analysis/investigation.md.
- Toolkit findings: TK-001 confirmed bug (NES emulator.input ignores button/pressed), TK-002 missing feature
  (AxROM static discovery stops at JSR/indexed stores; doc parameter mismatch). See analysis/toolkit-issues.md.
- Handoff: analysis/handoffs/round-01-disassembly.md

ROM classification: 99.88% (130936 / 131088 bytes classified; 152 bytes unknown).

## Round 1 commentary (2026-11-23) - complete current-source readability
- Prepared commentary/ from pass-001; source rendered from annotations (commentary/tools/render.py).
- Fresh verify: byte_identical true (commentary/verification.json); reading copy commentary/reading-after.asm.
- Classification unchanged: 99.88% (130936 / 131088; 152 unknown in L001 fragments).
- Handoff: analysis/handoffs/round-01-commentary.md

## Round 2 disassembly pass (2026-11-23) - checkpoint pass-002

- Scope: ready leads L006/L007/L008 (static verification + annotation); no gameplay (ending already visited, S01-S10).
- Checkpoint pass-002: maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json
  (ok true, byte_identical true, map_sha256 6fd86fe7c626951ce617cc477e33545cc661968b58358c9e0db5ccaa8c59c0df).
  map.validate ok; analysis/state.json saved; maps/current.json exported and validated.
- Per bank (classified / unknown): header 16/0; bank0 32768/0 (Code 21199, Data 11569); bank1 32768/0
  (Code 1365, Data 31403); bank2 32768/0 (Code 3897, Data 28871); bank3 32768/0 (Code 8845, Data 23923).
- Leads: L006 verified (150 B -> Code, Static, no known caller); L007 verified (3 lone RTS -> Code, conf 0.6,
  incl. 3:$91B9 Data->Code); L008 verified (3:$9E4A-$9E8D Data->Code, MulSignedB3). L002 blocked (not retried), L005 open.
- Toolkit issues unchanged: TK-001, TK-002 (analysis/toolkit-issues.md); no new toolkit issues found this pass.
- Handoff: analysis/handoffs/round-02-disassembly.md

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

## Round 3 disassembly pass (pass-003)
- Applied L009: 1:$D600-$D67F one 128-byte BlockSubstTables (8 missions x 16; merged 4 regions);
  3:$D76F-$D7EE BlockSubstTablesCopyB3 (unread byte-identical copy, merged 4 regions); 3:$D76E 1-byte
  lone rts -> Code (Static 0.55, consistent with L007). Net: 8 regions -> 3; 1 byte Data->Code.
- Per bank (code/data/unknown): header 0/16/0; bank0 21199/11569/0; bank1 1365/31403/0;
  bank2 3897/28871/0; bank3 8846/23922/0.
- Verification: map.validate ok; checkpoint pass-003 ok, byte_identical true (analysis/pass-003-verification.json).
- L002 blocked (not retried), L005 semantics-only. Toolkit issues unchanged (TK-001, TK-002; analysis/toolkit-issues.md); no new toolkit issues found.
- Handoff: analysis/handoffs/round-03-disassembly.md

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

## Terminal handoff (coordinator, 2026-11-23)

- Workflow state: TERMINAL. All phases done (rounds 1-3 disassembly/commentary/redirect + final acceptance).
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown) — map pass-003 (byte-identical,
  analysis/pass-003-verification.json). Whole-ROM byte classification is evidence-backed; 13 small no-caller code
  ranges are statically inferred (reachability unknown); several data blocks have no identified reader.
- Commentary: final editorial audit passed (commentary/audit.json, commentary/editorial-audit.md); reading copy
  commentary/reading-after.asm; residual meanings in commentary/uncertainties.md.
- Redirect terminal assessment: analysis/handoffs/round-03-redirect.md (rejection review inspected by coordinator: sound).
- Deferred/blocked: L002 RAM-executed routine views (toolkit feature missing; TK issues in analysis/toolkit-issues.md),
  L005 data semantics, unvisited milestones (natural objective hit, star-speed bits, unpatched full playthrough).
- Archive document selection: archive-options.json (no archive run, per instructions).
- Separate claims: this orchestration's terminal state does not by itself certify ROM classification completeness
  beyond the recorded evidence above; no full hardware/game compatibility is claimed.
