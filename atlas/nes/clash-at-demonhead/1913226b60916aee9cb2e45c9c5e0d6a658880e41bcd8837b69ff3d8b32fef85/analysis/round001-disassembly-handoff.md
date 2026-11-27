# Round001 disassembly handoff

Status: pass finished; ROM incomplete.

ROM classification: 56.79% (148886 / 262160 bytes classified; 113274 bytes unknown).

Baseline and pass-001 validated, generated, byte-identical verified. Load analysis/state.json; maps/current.json is supported ROM-free portable map; maps/pass-001.json immutable checkpoint. Readability awaits commentary. Counts/unknown complement and all classifications: analysis/round001/counts.json, unknown-ranges.json; progress.md holds per-bank totals.

Evidence: CHR/hardware sections, accepted bank-aware fetch pages, initial validated vector paths, seven inspected uniform non-code runs. Source skip trick physical74688 BIT versus alternate LDA74689 is documented, not a conflict.

Gameplay: title, route1 map/action, inventory and Guycot dialogue visited. Genuine later routes/bosses/ending remain unvisited; shortcut $0342=10/42 at map resume failed to initialize them. Exact recipes/snapshots in scenarios.json and analysis/scratch/*.py. Latest natural frontier dialog-next.snapshot.json, clean route-map frontier map-clean.snapshot.json. Snapshot import requires emulator.start; restore does not restore trace configuration, and reassert input.

Redirect priorities: L001 recover stage initialization/junction contract; L002 dialogue input/text engine then ordinary progression; L003 inspect dispatch/static closure from observed contracts. Retain role vs natural reachability distinction. No ready lead was rejected wholesale, no terminal exhaustion claimed.

No toolkit issues found (analysis/toolkit-issues.md). No live persistent process required. Disk initially2.6GiB free; final storage report below. Preserve durable maps/source/evidence/snapshots; scratch is helper code and transient broker files, not needed for delivery. No archival/publication performed.

Final storage:2.5GiB free; selected round001 evidence63MiB, scratch92KiB. No cleanup. Execution metric detail in round001/execution-metrics.json.
