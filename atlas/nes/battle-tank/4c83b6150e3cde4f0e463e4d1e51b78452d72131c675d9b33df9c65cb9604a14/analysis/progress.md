# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Disassembly 001 in progress

Baseline verified in analysis/baseline-verification.json. CNROM mapper 3, fixed PRG slots [0,1,2,3]; physical file 65,552 bytes. Vector roots $8000/$E00B/$F567 inspected. State/map saved after vector-led and title task closure. Natural Start reached mission briefing; gameplay and later-mission RAM contracts still being explored. Durable artifacts and replay scripts in analysis/disassembly-001/. This is a resumable frontier, not a completed pass.

Trace title window: 60 frames; 1,211 unique fetched opcodes, 1,307 contextual edges; 5,192,848 raw detail events dropped at capacity 0 by design, aggregate_dropped=0. Raw details unavailable; bounded aggregate evidence remains.

## Disassembly 001 complete — structural coverage, editorial pending

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

| Physical section | Code | Data | Image | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| Header, file $0000 | 0 | 16 | 0 | 16 | 0 |
| PRG0, file $0010 | 5098 | 3094 | 0 | 8192 | 0 |
| PRG1, file $2010 | 6240 | 1952 | 0 | 8192 | 0 |
| PRG2, file $4010 | 1710 | 6482 | 0 | 8192 | 0 |
| PRG3, file $6010 | 7530 | 662 | 0 | 8192 | 0 |
| CHR0, file $8010 | 0 | 0 | 8192 | 8192 | 0 |
| CHR1, file $A010 | 0 | 0 | 8192 | 8192 | 0 |
| CHR2, file $C010 | 0 | 0 | 8192 | 8192 | 0 |
| CHR3, file $E010 | 0 | 0 | 8192 | 8192 | 0 |
| Total | 20578 | 12206 | 32768 | 65552 | 0 |

Classification evidence: vector/direct-flow closure, encoded task/frame contracts, callback/stack contracts, bounded format and generic pattern inspections, header-fixed CHR storage. 147/147 regions have structural evidence; all provenance conservatively Static. Runtime evidence is separately retained and does not imply every statically classified path naturally executes. See [investigation ledger](investigation.md) and [final counts](disassembly-001/final-counts.json).

Gameplay: natural mission1 cockpit/map; supported RAM initialization sampled mission4/7/10 and final result→ending transition. Exact recipes, artificial status and captures in [scenarios.json](scenarios.json). Bounded coverage audit retained 2,293 unique physical opcode offsets (3,889 origin/context keys), none inside Data/Image. No raw trace detail retained at capacity0; aggregate_dropped0. Historical title/mission summaries and final explicit addresses remain local.

Verification: [checkpoint report](disassembly-001-verification.json), [frozen map](../maps/disassembly-001.json), [generated source](../generated/disassembly-001/game.asm). Internal map validated, state saved, current export fresh, generated rebuild byte-identical; original input SHA-256 unchanged. Baseline remains preserved.

Toolkit findings: one confirmed nonblocking candidate-reporting bug NES-001, no suspected or missing-feature-only findings, toolkit unchanged; see [toolkit-issues.md](toolkit-issues.md). Discovery Static/false candidate markers cannot measure execution; explicit accepted coverage provides the workaround.

Phase handoff: full current-source commentary and final editorial audit remain. Code uses 102 generated region labels and lacks descriptive routine/branch/RAM commentary; do not declare whole workflow complete from classification or rebuilding alone. The disassembly pass is complete, not interrupted. No unknown ranges remain. Exact table subformats, sound effect meanings, all natural callers and exhaustive gameplay are outside the structural claim. See [handoff](handoff-disassembly-001.md).

## Final acceptance 001 complete — coordinator orchestration pending

ROM classification: **100.00% (65552 / 65552 bytes classified; 0 bytes unknown)**. Final editorial review: 147 regions and 365 units reviewed; zero editorial gaps. Corrected R001-SCREEN-EDITORIAL consistently in current map/state/source/review and fixed inherited audit source-location bases. Valid prior review credit and frozen baselines retained. Fresh map.validate/save/export, unique final-acceptance-001 checkpoint and final commentary.py audit all pass; generated checkpoint and maintained source both rebuild byte-identically. Reading copy refreshed. See commentary/editorial-audit.md and analysis/current-audit.json for evidence/current hashes. Lead R001-SCREEN-EDITORIAL verified; no structural leads produced. Coordinator archive-options/orchestration acceptance remains pending; no archive executed.

## Coordinator terminal acceptance — round 1

Current phase: terminal handoff complete; no worker or research frontier pending. Prior in-progress sections above are historical.

| Sequential phase | Input checkpoint | Output / evidence | Lead outcomes |
| --- | --- | --- | --- |
| Disassembly 001 | initialized workspace; baseline once | disassembly-001; handoff-disassembly-001.md; fresh byte-identical verify | D001 vector/task/AI/sound/data/progression/coverage batch verified; boundary correction preserved |
| Commentary 001 | disassembly-001 | handoff-commentary-001.md; all classified source readable; fresh verify/read | semantic questions carried to redirect; no structural change |
| Redirect 001 | commentary-001 + disassembly-001 | handoff-redirect-001.md; r001-20261119 plan/findings/rejection-review | R001 controls verified; screen editorial repair produced; no structural lead |
| Final acceptance 001 | redirect handoff + current commentary | final-acceptance-001; handoff-final-acceptance.md; final audit/verify/read | R001-SCREEN-EDITORIAL verified; no pending/ready classification leads |

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

Final map/state hash fb1d32dfe6307b11c9b92dae3d0e77395a830a64a0e78c0f21901b76eb04e81d. Structural totals unchanged by the final label/evidence correction. All 147 regions / 365 units reviewed; zero readability/editorial gaps. Coordinator verified current hashes, complete interval partitions and refreshed reading copy, inspected redirect primary reasoning, and confirmed read-only archive editorial status complete. See orchestration-audit.json, current-audit.json and handoff-terminal.md. Whole-ROM classification acceptance rests on those gates, separately from orchestration completion.

Toolkit: NES-001 confirmed nonblocking reporting bug; no other confirmed/suspected/missing-feature findings. Input and toolkit unchanged; no archive/publication/cleanup executed. Archive document contract inspected; archive-options.json selects all additional final/historical Markdown records, with no disposable Markdown scratch to omit. All analysis/commentary Markdown is accounted for in archive-document-selection.md. Remaining uncertainty is semantic/reachability as detailed in terminal handoff; no active blocker.
