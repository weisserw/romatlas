# Disassembly progress

Workspace initialized 2026-11-14; baseline and round-1 disassembly ran 2026-11-19 (see below).

## Coordinator state (goal.md orchestration)

- Round: 3 (rounds 1–2 complete; redirect r2-a1 recommended round 3 with L-028, L-029, L-031)
- Current phase: COMPLETE. Final acceptance passed on 2026-11-19: editorial audit reviewed, `commentary.py audit` ok, byte_identical true, map validate ok, maps/current.json == maps/pass-003.json. The round 3 redirect was skipped because there were 0 unknown bytes and no open structural leads.
- Output (final acceptance): analysis/handoffs/final-acceptance.md; commentary/audit.json; pre-audit commentary in analysis/round-3/commentary/; leads L-005 verified, L-007 resolved, L-003/L-004/L-006/L-008/L-013/L-014 deferred-semantic
- Output (round 3 commentary): commentary/ (fresh baseline from pass-003; round-2 copy in analysis/round-2/commentary/); handoff analysis/handoffs/round-3-commentary.md; L-031 verified (done); verify byte_identical true; reading copy refreshed
- Output (round 3 disassembly): pass-003 (maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json, byte_identical true, map sha256 4b3c1d85...); handoff analysis/handoffs/round-3-disassembly.md; verified L-028, L-029; L-031 left ready for commentary
- Output (round 2 redirect): analysis/redirect/r2-a1/, handoff analysis/handoffs/round-2-redirect.md; ready L-028 (last unknown byte -> Code), L-029 (boundary), L-031 (commentary corrections); resolved L-012, L-026, L-030
- Output (round 2 commentary): commentary/ (fresh baseline from pass-002; round-1 copy in analysis/round-1/commentary/); handoff analysis/handoffs/round-2-commentary.md; leads closed L-025, L-027; new L-029, L-030
- Output (round 2 disassembly): pass-002 (maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json, byte_identical true); handoff analysis/handoffs/round-2-disassembly.md
- Input checkpoint: pass-001 (byte_identical true); redirect r1-a1 ready leads L-009, L-015..L-023 + reopened L-024
- Leads consumed/produced (round 2 disassembly): verified L-009, L-015..L-024; produced L-027 (commentary for new code), L-028 (1 unknown byte); L-025 (commentary label swap) and L-026 (ending stall) carried over
- Earlier outputs: round 1 disassembly pass-001; commentary commentary/source (round 1); redirect analysis/handoffs/round-1-redirect.md
- ROM classification: 100.00% (327696 / 327696 bytes classified; 0 bytes unknown).

## Blocker — 2026-11-14 coordinator session

Every Bash command fails before execution, both in the coordinator and in a
spawned subagent, with:

    apply-seccomp: write /proc/self/setgroups (nested userns is capability-restricted; caller must provide CAP_SYS_ADMIN): Permission denied

Reproduced with `echo hi`, `true` and `python3 --version`. This is a host
sandbox (bubblewrap/seccomp) configuration problem, not a toolkit bug. All
phases require command execution (`python3 run.py inspect|baseline|serve|checkpoint`,
the configured romatlas binary, emulator JSON-RPC), so no phase worker can do work.

No workspace files other than this progress note were changed. input/game.nes is
untouched. archive-options.json was not written because no final analysis or
commentary documents exist yet.

Resume: once shell execution works, start round 1 phase 1 (disassembly worker)
from workflow.md step 1 (`python3 run.py inspect`, then `python3 run.py baseline`).

## Blocker resolved — 2026-11-19 coordinator session

Shell execution works again (verified with ls/cat/python3). Free disk ~4.1 GB.
Round 1 phase 1 (disassembly) worker started from workflow.md step 1.

## Round 1 — disassembly pass (2026-11-19)

ROM classification: 95.56% (313139 / 327696 bytes classified; 14557 bytes unknown).

Result: incomplete (14,557 unknown bytes remain). Checkpoint pass-001 verified byte_identical true
(analysis/pass-001-verification.json, map sha256 6a0d90d1...). Internal store analysis/state.json
(1,280 regions), export maps/current.json.

By kind/provenance: Code Observed 31,968; Code Static 19,644; Data Static 129,487 (incl. 74,756
heuristic generic data at confidence 0.75, 14 DPCM samples, 35 jump tables, vectors, header);
Padding 66,492; Image (CHR) 65,536; Text 12. Execution coverage (separate metric): 15,593 observed
instruction starts. Commentary review: not started.

Per-bank classified/unknown (bytes):

| Area | Code | Data | Padding | Image | Text | Unknown |
|---|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 | 0 | 0 |
| PRG 8K bank 0 (32K bank 0, $8000) | 6933 | 1238 | 0 | 0 | 0 | 21 |
| PRG 8K bank 1 (32K bank 0, $A000) | 6437 | 157 | 184 | 0 | 0 | 1414 |
| PRG 8K bank 2 (32K bank 0, $C000) | 904 | 6881 | 0 | 0 | 0 | 407 |
| PRG 8K bank 3 (32K bank 0, $E000) | 5954 | 1280 | 804 | 0 | 0 | 154 |
| PRG 8K bank 4 (32K bank 1, $8000) | 2592 | 5357 | 43 | 0 | 0 | 200 |
| PRG 8K bank 5 (32K bank 1, $A000) | 45 | 8081 | 66 | 0 | 0 | 0 |
| PRG 8K bank 6 (32K bank 1, $C000) | 2926 | 5256 | 0 | 0 | 0 | 10 |
| PRG 8K bank 7 (32K bank 1, $E000) | 2670 | 235 | 3670 | 0 | 0 | 1617 |
| PRG 8K bank 8 (32K bank 2, $8000) | 1760 | 3914 | 106 | 0 | 0 | 2412 |
| PRG 8K bank 9 (32K bank 2, $A000) | 1115 | 2000 | 4629 | 0 | 0 | 448 |
| PRG 8K bank 10 (32K bank 2, $C000) | 0 | 8192 | 0 | 0 | 0 | 0 |
| PRG 8K bank 11 (32K bank 2, $E000) | 60 | 4047 | 4082 | 0 | 0 | 3 |
| PRG 8K bank 12 (32K bank 3, $8000) | 4568 | 1198 | 0 | 0 | 0 | 2426 |
| PRG 8K bank 13 (32K bank 3, $A000) | 4950 | 373 | 372 | 0 | 0 | 2497 |
| PRG 8K bank 14 (32K bank 3, $C000) | 2267 | 4964 | 0 | 0 | 0 | 961 |
| PRG 8K bank 15 (32K bank 3, $E000) | 75 | 7886 | 216 | 0 | 0 | 15 |
| PRG 8K bank 16 (32K bank 4, $8000) | 914 | 3523 | 3745 | 0 | 0 | 10 |
| PRG 8K bank 17 (32K bank 4, $A000) | 1130 | 128 | 6451 | 0 | 0 | 483 |
| PRG 8K bank 18 (32K bank 4, $C000) | 0 | 7682 | 510 | 0 | 0 | 0 |
| PRG 8K bank 19 (32K bank 4, $E000) | 45 | 7688 | 459 | 0 | 0 | 0 |
| PRG 8K bank 20 (32K bank 5, $8000) | 914 | 3523 | 3745 | 0 | 0 | 10 |
| PRG 8K bank 21 (32K bank 5, $A000) | 1130 | 128 | 6451 | 0 | 0 | 483 |
| PRG 8K bank 22 (32K bank 5, $C000) | 0 | 7682 | 510 | 0 | 0 | 0 |
| PRG 8K bank 23 (32K bank 5, $E000) | 45 | 7688 | 459 | 0 | 0 | 0 |
| PRG 8K bank 24 (32K bank 6, $8000) | 914 | 3523 | 3745 | 0 | 0 | 10 |
| PRG 8K bank 25 (32K bank 6, $A000) | 1130 | 128 | 6451 | 0 | 0 | 483 |
| PRG 8K bank 26 (32K bank 6, $C000) | 0 | 7682 | 510 | 0 | 0 | 0 |
| PRG 8K bank 27 (32K bank 6, $E000) | 45 | 7688 | 459 | 0 | 0 | 0 |
| PRG 8K bank 28 (32K bank 7, $8000) | 914 | 3523 | 3745 | 0 | 0 | 10 |
| PRG 8K bank 29 (32K bank 7, $A000) | 1130 | 128 | 6451 | 0 | 0 | 483 |
| PRG 8K bank 30 (32K bank 7, $C000) | 0 | 7682 | 510 | 0 | 0 | 0 |
| PRG 8K bank 31 (32K bank 7, $E000) | 45 | 16 | 8119 | 0 | 12 | 0 |
| CHR 64K | 0 | 0 | 0 | 65536 | 0 | 0 |

Method summary, evidence and residuals: analysis/investigation.md. Scenarios: analysis/scenarios.json
(S01-S09 visited; end-of-game, pennant progression and ending deferred). Handoff:
analysis/handoffs/round-1-disassembly.md. Toolkit findings: analysis/toolkit-issues.md (TK-001
confirmed bug, TK-002 missing feature, TK-003 suspected bug + missing feature; none blocking).

## Round 1 — commentary pass (2026-11-19)

Scope: all classified source (8 x 32 KiB PRG banks, header, vectors, CHR) brought to the naming/
comment standard: descriptive names for every routine/jump target (no Sub_/Lxxxx left), ~300 RAM
symbols with Runner/Fielder/Marker record layouts, hardware/MMC1 symbols, `.word` jump tables,
condition/loop-derived branch labels, hand-written explanations for the engine and mechanical
summaries elsewhere, boundary notes for data/padding/unknown ranges. review.json: 1,465 entries
reviewed, 1,596 units; editorial audit pending (final acceptance). Verification: commentary.py verify
ok, byte_identical true; reading copy refreshed. Leads added L-009..L-014. No new toolkit issues.
Details: analysis/handoffs/round-1-commentary.md, commentary/progress.md, commentary/uncertainties.md.

## Round 1 — redirect pass (2026-11-19, attempt r1-a1)

Input checkpoint pass-001 (14,557 unknown). Research only (no map/source changes). Findings: FarCall
fall-through correction of the static traversal (+5,088 code bytes, no conflicts), AtBatStateTable
bounded to 128 words with new handlers, bank-2 $9551 screen loader + images (L-002 correction), bank-1
CPU pitch tables, sound tables in banks 2/4-7, heuristic-data counterexample ($B7B2 stubs), score
label swap ($0418 = home). Ready leads: L-009, L-015..L-023 (12,096 bytes) and L-025 (commentary);
projected unknown after acceptance 2,461 (L-024 blocked: code-like routines without entry).
Gameplay: end of game (S10), season end and ending scene 12 (S11) visited with RAM patches
(artificial); ending needed an extra DpcmRequest unblock (L-026). Toolkit: TK-004 added.
Handoff: analysis/handoffs/round-1-redirect.md. Recommendation: start round 2 disassembly.

## Round 2 — disassembly pass (2026-11-19)

ROM classification: 100.00% (327695 / 327696 bytes classified; 1 bytes unknown).

Result: incomplete (1 unknown byte: phys $19CBC, bank 3 $9CAC lone unreferenced RTS, lead L-028).
Checkpoint pass-002 verified byte_identical true (analysis/pass-002-verification.json, map sha256 05fd28c2...).
Internal store analysis/state.json (1,496 regions), export maps/current.json.

Leads (all re-verified before annotation; analysis/investigation.md): L-015 +5,226 code (incl. 138 Data->Code),
L-016 +250, L-017 +2,252, L-018 +1,472, L-019 +2,240, L-023 +541 data; L-020 +140, L-009 +70, L-021 +28,
L-022 +15 code; L-024 reopened on corrected premise (structural evidence, entry not required) +2,427 code
(confidence 0.75, reachability unresolved) +33 data. Total +14,556 bytes.

By kind/provenance: Code Observed 31,968; Code Static 27,550; Data Static 136,137; Padding 66,492;
Image 65,536; Text 12. Execution coverage unchanged this pass (no new runs; r1-a1 artificial-state coverage
of game end/season end/ending cited in evidence only). Commentary review: round-1 source predates these regions.

Per-bank classified/unknown (bytes):

| Area | Code | Data | Padding | Image | Text | Unknown | New this pass |
|---|---|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 | 0 | 0 | 0 |
| PRG 8K bank 0 | 6950 | 1242 | 0 | 0 | 0 | 0 | 21 |
| PRG 8K bank 1 | 7430 | 578 | 184 | 0 | 0 | 0 | 1414 |
| PRG 8K bank 2 | 1311 | 6881 | 0 | 0 | 0 | 0 | 407 |
| PRG 8K bank 3 | 6092 | 1296 | 804 | 0 | 0 | 0 | 154 |
| PRG 8K bank 4 | 2792 | 5357 | 43 | 0 | 0 | 0 | 200 |
| PRG 8K bank 5 | 45 | 8081 | 66 | 0 | 0 | 0 | 0 |
| PRG 8K bank 6 | 2936 | 5256 | 0 | 0 | 0 | 0 | 10 |
| PRG 8K bank 7 | 2811 | 1711 | 3670 | 0 | 0 | 0 | 1617 |
| PRG 8K bank 8 | 1920 | 6166 | 106 | 0 | 0 | 0 | 2412 |
| PRG 8K bank 9 | 1115 | 2448 | 4629 | 0 | 0 | 0 | 448 |
| PRG 8K bank 10 | 0 | 8192 | 0 | 0 | 0 | 0 | 0 |
| PRG 8K bank 11 | 63 | 4047 | 4082 | 0 | 0 | 0 | 3 |
| PRG 8K bank 12 | 6901 | 1290 | 0 | 0 | 0 | 1 | 2425 |
| PRG 8K bank 13 | 7303 | 517 | 372 | 0 | 0 | 0 | 2497 |
| PRG 8K bank 14 | 3223 | 4969 | 0 | 0 | 0 | 0 | 961 |
| PRG 8K bank 15 | 90 | 7886 | 216 | 0 | 0 | 0 | 15 |
| PRG 8K bank 16 | 924 | 3523 | 3745 | 0 | 0 | 0 | 10 |
| PRG 8K bank 17 | 1165 | 576 | 6451 | 0 | 0 | 0 | 483 |
| PRG 8K bank 18 | 0 | 7682 | 510 | 0 | 0 | 0 | 0 |
| PRG 8K bank 19 | 45 | 7688 | 459 | 0 | 0 | 0 | 0 |
| PRG 8K bank 20 | 924 | 3523 | 3745 | 0 | 0 | 0 | 10 |
| PRG 8K bank 21 | 1165 | 576 | 6451 | 0 | 0 | 0 | 483 |
| PRG 8K bank 22 | 0 | 7682 | 510 | 0 | 0 | 0 | 0 |
| PRG 8K bank 23 | 45 | 7688 | 459 | 0 | 0 | 0 | 0 |
| PRG 8K bank 24 | 924 | 3523 | 3745 | 0 | 0 | 0 | 10 |
| PRG 8K bank 25 | 1165 | 576 | 6451 | 0 | 0 | 0 | 483 |
| PRG 8K bank 26 | 0 | 7682 | 510 | 0 | 0 | 0 | 0 |
| PRG 8K bank 27 | 45 | 7688 | 459 | 0 | 0 | 0 | 0 |
| PRG 8K bank 28 | 924 | 3523 | 3745 | 0 | 0 | 0 | 10 |
| PRG 8K bank 29 | 1165 | 576 | 6451 | 0 | 0 | 0 | 483 |
| PRG 8K bank 30 | 0 | 7682 | 510 | 0 | 0 | 0 | 0 |
| PRG 8K bank 31 | 45 | 16 | 8119 | 0 | 12 | 0 | 0 |
| CHR 64K | 0 | 0 | 0 | 65536 | 0 | 0 | 0 |

Gameplay: checklist reviewed (S01-S11, S13 visited; S12 attempted); no new runs (see analysis/scenarios.json
notes_round2_disassembly). Toolkit: no new toolkit issues found this pass (analysis/toolkit-issues.md, TK-001..TK-004
unchanged). Handoff: analysis/handoffs/round-2-disassembly.md.

## Round 2 — commentary pass (2026-11-19)

- Baseline: pass-002 (commentary.py prepare after preserving round-1 commentary at
  analysis/round-1/commentary/). Notes/tools carried by physical range; first rebuild
  byte-identical; ~20 never-rendered round-1 notes re-targeted.
- Scope: all classified source in banks 0-7 (new L-009/L-015..L-024 code and data plus the
  round-1 backlog of ~540 mechanical summaries) now has hand-written routine comments and
  descriptive names; L-024 regions carry "reachability unresolved" notes.
- Key changes: L-025 score swap; hits/errors counters; pennant games/wins; at-bat states
  named; password layout decoded; developer shortcuts identified; several swapped round-1
  names corrected (batter lines, password cursor routines, team A = home).
- review.json 1,497/1,497 reviewed (1,278 carried, 219 new); editorial audit pending.
- Verification: commentary.py verify byte_identical true; reading-after.asm refreshed.
- Leads: closed L-025, L-027; updated L-012; new L-029 (bank 3 $A39C boundary), L-030
  (bank 1 $85BE multiply contract). Handoff: analysis/handoffs/round-2-commentary.md.

## Round 2 — redirect r2-a1 (2026-11-19)

Input pass-002 (100.00%, 1 unknown byte phys $19CBC). Research only (no map/state/commentary change);
details analysis/redirect/r2-a1/notes.md, handoff analysis/handoffs/round-2-redirect.md.
- L-028 ready: $9CAC RTS proposed Code/Static (exact 1-byte gap JMP-barrier -> live entry; 26 matching
  standalone-RTS bytes all Code incl. 7 unreferenced accepted in round 2; no data consumer) -> projected 0 unknown.
- L-029 ready: AnimationStepTimes 5 bytes, DrawStepFigure starts $A39D (byte-identical).
- L-030 resolved: static caller bank 0 $9B6C; LaunchSpeed forced to $0400 when home leads (X1 runtime).
- L-026 resolved: both season-end exits stall (SceneInit17 music before SaveGameState clear; X2 runtime).
- L-012 resolved: EDCBAAK decode verified (X4). New L-031 commentary corrections (A button, above facts).
- Gameplay S14-S16 added (artificial). Server stopped; scratch unchanged in size; disk ~4.8 GB free.
Recommendation: (a) round 3 disassembly with L-028, L-029; commentary L-031; then final editorial audit.

## Round 3 — disassembly pass (2026-11-19)

ROM classification: 100.00% (327696 / 327696 bytes classified; 0 bytes unknown).

Input pass-002 (1 unknown byte). Checkpoint pass-003 verified byte_identical true
(analysis/pass-003-verification.json, map sha256 4b3c1d85...). Internal store analysis/state.json
(1,497 regions), export maps/current.json. input/game.nes sha256 re-checked unchanged.

Leads (independently re-verified; analysis/investigation.md "Round 3 disassembly"):
- L-028 verified: phys $19CBC (bank 3 $9CAC, prg_banks [12,13,14,15]) lone RTS -> Code/Static, confidence 0.6,
  reachability unresolved. +1 byte; unknown complement now empty.
- L-029 verified: AnimationStepTimes Data $1A3A8+5 (was +4); DrawStepFigure Code/Static 0.75 $1A3AD+31 at $A39D
  (was $1A3AC+32). Boundary only, 0 net byte change.
- L-031 left ready for commentary.

By kind/provenance: Code Observed 31,968; Code Static 27,550 (+1 -1); Data Static 136,138 (+1); Padding 66,492;
Image 65,536; Text 12. Lower-confidence classifications: heuristic generic data 74,961 bytes (0.75), L-024 code
2,426 bytes (0.75), L-024 data 33 bytes (0.8), L-028 byte (0.6).

Per-bank: identical to the round-2 table above except PRG 8K bank 12 Code 6902 / Data 1290 / Unknown 0
(+1 code) and PRG 8K bank 13 Code 7302 / Data 518 / Padding 372 (1 byte moved Code->Data). All banks 0 unknown.

Gameplay: no new runs (checklist essentially fully visited; no lead needed play). Toolkit: no new toolkit
issues found this pass (analysis/toolkit-issues.md, TK-001..TK-004 unchanged). Server started for this pass
was stopped. Handoff: analysis/handoffs/round-3-disassembly.md.
Status: classification complete (0 unknown); commentary (L-031, new region notes) and final editorial audit remain.

## Round 3 — commentary pass (2026-11-19)

- Baseline: pass-003 (round-2 commentary preserved and diff-verified at analysis/round-2/commentary/,
  old directory removed, `commentary.py prepare` re-run; TK-005 workaround). Notes/tools carried by
  physical range; first rebuild byte-identical, no dropped notes.
- Map changes handled: NullHook9CAC (L-028, reachability unresolved), AnimationStepTimes 5-byte note
  and DrawStepFigure at $A39D (L-029).
- L-031 done: A-button comments (SceneFrame17/18, also SceneFrame15; EndingPressAToTitle renamed);
  ScaleLaunchByHomeLead caller/effect (L-030); season-end stall as probable game bug (L-026);
  PennantCounterA = game number (ram.inc and all dependent texts; NextScheduleEntry explained).
- review.json 1,497/1,497 reviewed (1,475 carried with identical range and rendered text, 19
  re-reviewed after comment changes, 3 new); 1,887 units; editorial_audit pending.
- Verification: commentary.py verify ok, byte_identical true; reading-after.asm refreshed.
- ROM classification: 100.00% (327696 / 327696 bytes classified; 0 bytes unknown).
  Handoff: analysis/handoffs/round-3-commentary.md.

## Final acceptance (2026-11-19)

Verdict: **complete.** Every byte has an evidence-backed classification. The required
annotations are in place, the map is valid, the final commentary audit passed and the
rebuild is byte-identical.

- Editorial audit: commentary/review.json editorial_audit is reviewed.
  - `commentary.py audit` passed: 1497 regions, 1889 units, byte_identical true (commentary/audit.json).
  - The reading copy is refreshed.
- Edits:
  - 545 condition-accurate branch-label renames.
  - Process references removed from the source.
  - Module-copy wording is now "no known entry" instead of "dead" (L-007).
  - Every lower-confidence region carries an explicit confidence/unresolved note.
  - ModeFlags and Port7000 documentation is updated.
- Map: `romatlas map validate maps/current.json --rom input/game.nes` returned ok.
  - maps/current.json is byte-equal to maps/pass-003.json (sha256 4b3c1d85...).
  - input/game.nes is unchanged.
- Leads:
  - L-005 verified: the dispatch-table extents match the static index bounds.
  - L-007 resolved.
  - L-003, L-004, L-006, L-008, L-013 and L-014 are deferred-semantic and do not affect classification.
- Lower-confidence classifications:
  - 2,426 B of code with no known entry (0.75).
  - NullHook9CAC (0.6).
  - About 74,961 B of pattern-based generic data (0.75).
  - AnimationStepTimes entries 2-4: no known reader.
- Handoff: analysis/handoffs/final-acceptance.md.
- ROM classification: 100.00% (327696 / 327696 bytes classified; 0 bytes unknown).

## Coordinator terminal handoff — 2026-11-19

- Workflow reached its documented terminal handoff: whole-ROM classification complete
  on its own evidence (pass-003 map; 0 unknown bytes), not because orchestration ended.
- ROM classification: 100.00% (327696 / 327696 bytes classified; 0 bytes unknown).
- Coordinator independent checks: maps/current.json byte-equal to maps/pass-003.json;
  `romatlas map validate` ok; `commentary.py verify` ok, byte_identical true;
  commentary/audit.json ok, editorial_audit true (1497 regions, 1889 units);
  input/game.nes sha256 unchanged; region recount covers all 327696 bytes
  (Code 59518, Data 136138, Image 65536, Padding 66492, Text 12).
- Lower-confidence (but classified) areas are documented in source and in
  analysis/handoffs/final-acceptance.md: ~2,426 B statically classified code with no
  known caller (0.75), ~74,961 B pattern-based generic data (0.75), bank 3 $9CAC lone
  RTS (0.6). Semantic leads L-003/L-004/L-006/L-008/L-013/L-014 deferred (no
  classification impact).
- Archive document selection: archive-options.json (contract from
  /root/romatlas/scripts/archive.py). Selected: all analysis/handoffs/*.md, redirect
  notes r1-a1/r2-a1, analysis/round-1/commentary/progress.md (referenced by current
  commentary/progress.md). Omitted as superseded snapshots/tool-generated instructions:
  analysis/round-{1,2,3}/commentary/{AGENTS,prompt,workflow,uncertainties}.md and
  round-{2,3} progress.md (their content is carried in current commentary/progress.md
  and uncertainties.md). Every Markdown file under analysis/ and commentary/ is
  accounted for (checked against archive.py's unrecognized-document rule).
- Not done (not authorized): archive execution, cleanup, publishing. scratch/ (47 MB)
  left for the archive/cleanup step.
