# Round 1 disassembly handoff

Status: incomplete ROM; single easy-gain disassembly pass finished. Next phase is full current-source commentary, then redirect. This is not terminal investigation exhaustion.

ROM classification: 58.35% (382424 / 655376 bytes classified; 272952 bytes unknown).

Code22,128; generic/specific Data140,607; CHR Image131,072; Text88,617. 5,299 nonoverlapping regions all have structural evidence. Per-bank classified/unknown counts: [counts.md](counts.md). Exact unknown complement: [unknown-ranges.json](unknown-ranges.json). Summary: [summary.json](summary.json).

Verification: internal map validated; saved `analysis/state.json`, exported `maps/current.json`; fresh checkpoint `maps/pass-001.json`, `generated/pass-001`, [verification](../pass-001-verification.json) confirms byte-identical rebuild and original ROM SHA-256. Earlier baseline/pass-001a/b/c checkpoints preserved. No toolkit or ROM modifications.

Natural scenarios: reset/copyright; prebuilt Gwydion/Felgar/Goldleaf party; Phlan tour; character stats/equipment view; city movement into slums; orc encounter; tactical grid movement/target/menu; Goldleaf Cast menu and actual Sleep cast. Durable latest natural state is `analysis/round1/spell-resolve.snapshot.json`; useful earlier branches are `city.snapshot.json`, `battle2.snapshot.json`, `cast.snapshot.json`. [Scenarios](../scenarios.json) provides routes and statuses, [gameplay](../gameplay.md) provides manual source and tested applicability. Full RPC request/response logs and physical/contextual root files are local beside this handoff.

Execution is separate: 9,223 unique natural ROM instruction starts, 18,950 fetched instruction bytes (3.6144% of PRG bytes), 20,429 contextual roots across saved captures. Long waits sometimes ran with capture disabled; no execution claim for them. Enabled aggregate overflow was zero; detail drops with capacity0 are expected. All saved natural fetched bytes lie inside Code annotations (zero classification conflicts). Artificial shortcut roots excluded from these metrics.

Static easy gains: validated reset/NMI/IRQ; supported successor and direct-JSR closures from natural roots, explicit effective 8 KiB banks, data barriers and competing-boundary checks. Latest static batch259 bytes and direct-call closure41 bytes; repeated battle idle sample0 bytes, spell resolution256 bytes, showing diminishing returns from the current representative scenario. Further routine closure requires callee return/bank effects or indirect dispatch contracts: ordinary discovery conservatively stops calls/stores and cannot safely step through inline tables.

Data easy gains: CHR section/header/vectors, exact00/FF runs with adjacent bytes/reference inspection, finite repeated tile/flag-like pair domains (generic Data, no specific asset claim), and bounded readable English text strips. All adjacent byte evidence is in `uniform-runs.json`, `paired-data.json`, `text-boundaries.json`. One suspicious printable prefix was split off and remains unknown; no blanket rejection of neighboring prose.

Unresolved / redirect priorities:

1. Later areas, major bosses and ending remain deferred; do not mark gameplay globally exhausted. Native loader/save pair at physical$65642..$6568F copies PRG-RAM$75D7↔RAM$6C, $75D2/3/4↔$0600/1/2, $760C↔$6D, $760F↔$70. $0600/$0601 are packed map X/Y, $0602 facing. Bounded clean-snapshot patch $6D=5 and $0600..602=[8,8,0] did not reach a later area (still slums); clean restored. This falsifies that isolated shortcut recipe, not the existence of coherent area tuples or generic data. Resolve tuple/transition initialization before another patch. The API edits internal RAM; persistent PRG-RAM fields need a supported transition/copy routine contract.
2. Event/script banks interleave readable prose and short VM-looking controls. Fixed $F26C reads through $71/$72, distinguishes high-bit fields and B5 delimiters; bank59 consumes decoded values. Follow actual consumers and establish bounded command/operand slices, including straightforward generic non-code where justified. Current uncertainty is native code versus VM/data for residual strips and exact local boundaries, not missing game meaning. No complete-bank script claim was accepted or rejected.
3. Resolve inline dispatch/callee bank-return effects to expand ordinary static code safely. Do not feed pointer words directly to code classification; retain actual table/entry contracts.

Compact rejection review: only an isolated renderer-index stage patch failed, and only one printable prefix was excluded. Neither outcome rejects broader stage initialization or neighboring data slices. No historical rejection vetoes exist. This pass took broad representative gameplay and static gains; coherent stage contract and VM framing need redirect research. No quota or full-game compatibility claim applies.

Readability: commentary has not run. It must cover every currently classified region/current source, including older regions, before redirect; verified raw/data emission alone is not readable commentary completion.

Toolkit findings: no toolkit issues found (0 confirmed bugs,0 suspected,0 missing features; no toolkit blockers). [Report](../toolkit-issues.md). Environment socket denial/file-bridge SIGTERM was an interruption, not a toolkit defect or exhaustion reason; self-contained batches plus durable snapshots worked.
