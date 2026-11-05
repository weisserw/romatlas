# Round 1 disassembly handoff

Status: incomplete; single substantive disassembly pass completed. Proceed to full current-source commentary, then redirect. This is not a terminal exhaustion assessment.

ROM classification: 60.31% (158102 / 262160 bytes classified; 104058 bytes unknown).

Code 8914; Data 18116; Image 131072. Exact PRG bank counts: [progress.md](progress.md). Evidence-backed regions total 364, no overlaps. Authoritative state [state.json](state.json), exported map ../maps/current.json; checkpoint pass-001. Map SHA256 fded0bf4e573c67d93860eaede594282b2640f885e4c126a298395b85bac2fda. Baseline and pass-001 validate and rebuild byte-identically; see pass-001-verification.json. Input SHA256 unchanged.

Primary evidence: round1/annotation-proposals.json, annotation-results.json, code-proposals.json, observed-decodes.json, discover-*.json, uniform-runs.json. Observed-decodes includes exploratory RAM-root decodes; only actual ROM-space roots (CPU >=$8000) were accepted. RAM execution does not establish immutable upload origin. Contexts retain effective 8 KiB banks and CPU aliases.

Gameplay: title/mode/trial setup, named Brazil course, car setup/music/grid, acceleration/turbo/steer/brake. Snapshots and recipes are local under round1/ and scenarios.json. Later-course experiment filenames mentioning Japan are hypotheses: course capture confirms Brazil, not Japan. Longer waits resolved cursor movement but course latch remains uncertain. No artificial RAM/register changes used. Later areas, championship, pit and ending milestones are explicitly attempted/deferred, not exhausted.

Concrete redirect questions: (1) Trace initialization writes into PRG RAM and identify upload/decompression origins for execution at $6600+; 1546 distinct RAM opcode locations were excluded from ROM count. (2) Inspect course-selection consumer/latching, especially candidate RAM $062e/$062f changes 15->0 from one right press, then select a later course and inspect named course + actual race. (3) Initialize Grand Prix with Al Unser preset; identify course/lap/time/progression fields, then use supported RAM transition shortcuts to qualifying/results/late races/ending. Full recipes/status are in leads.json.

No toolkit issues found; see toolkit-issues.md. Bounded timeouts retain completed execution; capacity-zero trace detail losses intentional, aggregates dropped zero. An exploratory file-request broker ended; direct bounded RPC scripts replaced it. No active emulator is required for handoff; durable snapshots plus recorded inputs restore scenarios. Do not reuse session-local named snapshots. No files deleted or toolkit edits performed.
