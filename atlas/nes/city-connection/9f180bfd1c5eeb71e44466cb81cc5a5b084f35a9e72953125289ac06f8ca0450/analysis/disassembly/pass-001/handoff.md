# Round 1 disassembly handoff

Scope: exactly one disassembly pass; no commentary/redirect/archive/publication. Incomplete, nonterminal. ROM and toolkit unchanged.

ROM classification: 99.19% (65022 / 65552 bytes classified; 530 bytes unknown).

Checkpoint: maps/pass-001-final.json; generated/pass-001-final; analysis/pass-001-final-verification.json (ok and byte_identical true). Current internal state analysis/state.json and portable maps/current.json match final checkpoint. Baseline independently verified analysis/baseline-verification.json. Full per-bank counts counts.json and analysis/progress.md. Code9181/Data23073/Image32768. All annotations carry evidence; full readable-source commentary remains undone.

Accepted graph: fixed CNROM effective PRG slots[0,1,2,3], NMI$838D,reset$D000,IRQ$842E. OAM inline data at2637..2657 removed false constant-branch traversal. Return-address dispatch620..641, inline table21406..21418 and six targets added connected code. Evidence reviewed-static.json, dispatch-accepted.json, generic-data-review.json, uniform-runs.json. Generic noncode semantics intentionally unresolved where patterns and consumers support classification.

Runtime: title120frames; natural Start/NewYork driving/jump/oil, artificial level-clear bonus and Tokyo. Reproducible scripts/snapshots and exact patches in analysis/scenarios.json and gameplay.md. natural1697context roots, artificial late1796roots, aggregate_dropped0, no residual fetch. Detail intentionally capacity0; no full-game hardware/compatibility/execution claim. Invalid documented input attempts retained and superseded; actual input buttons array required.

Next phase should review current source, then redirect D001[379,421),D002[1976,2325),D003[7648,7662),D004[18032,18157). All are actionable unresolved leads, not rejected/blocked/exhausted. Specific recipes in analysis/leads.json. D004 PRG-writing candidate needs CNROM-aware structural interpretation; do not promote legal decoding alone. More city/terminal progression possible using current RAM transition contract; other backgrounds deferred, Tokyo wrap not runtime-confirmed. No terminal rejection audit warranted at this ordinary easy-gain handoff.

Toolkit: analysis/toolkit-issues.md NES-001 confirmed documentation/API mismatch (button/pressed and budget silently ignored); use buttons array/max_instructions. No blocker, no toolkit changes. Earlier worker paging bug corrected by next_offset>=total; no toolkit defect claimed. Servers closed. Files local only.
