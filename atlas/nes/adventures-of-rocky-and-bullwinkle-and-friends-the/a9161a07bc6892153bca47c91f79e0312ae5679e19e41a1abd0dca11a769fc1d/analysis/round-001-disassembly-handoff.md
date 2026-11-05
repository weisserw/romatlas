# Round 001 disassembly handoff

Status: one complete first-pass disassembly; ROM incomplete. Next phase: full current-source commentary, then redirect. No commentary/redirect/archive work performed.

ROM classification: 67.21% (176201 / 262160 bytes classified; 85959 bytes unknown).

Code13090; non-code163111 (Data31878, Text161, CHR Image131072). All503 regions have structural evidence;4342 saved accepted opcode starts, natural and artificial, are a separate coverage metric. Zero saved data/fetch conflicts. Exact per-bank counts and unknown complement: [classification-counts.json](classification-counts.json), [unknown-ranges.json](unknown-ranges.json); tables also in progress/investigation.

Verified final checkpoint: [map](../maps/round-001-disassembly.json), [source](../generated/round-001-disassembly/game.asm), [verification](round-001-disassembly-verification.json), byte-identical true, empty generation diagnostics. Map SHA256 96563b18c7a8148113b94ee6122cc64360a8f5c3e4f4be32785fdea61627b8df. Durable current state analysis/state.json; portable maps/current.json. Baseline and startup/scene2 checkpoints retained. ROM input hash unchanged.

Scope/findings: validated reset/NMI/IRQ and explicit effective bank contexts; connected static/observed code; header/CHR; maximal uniform non-code tails/row slices; finite11-pointer table at physical$3D6C, eight-byte controller sequence$3D82 and11x23 scene descriptors$394F..$3A4C supported independently by loader$D117 readingY0..22. Caption text bounded separately. Generic data makes no invented semantic/fill claim. Key entry labels include reset/NMI/IRQ and scene descriptor loader; full routine commentary remains next-phase work.

Gameplay: title identification Rocky/Bullwinkle confirmed locally. Natural reset/menu/intro, early gameplay, jump/bomb and selection/special-input retries; artificial initialized railway/log, desert/plane, ship/water, island architecture and final indoor scenes. Exact patches: restore clean warp-1.snapshot, release ports, RAM$078F=$FF,$0790=(selector-1)mod11, controller2 B2/release, initialized transition and representative input budgets. Reuse [scenarios.json](scenarios.json), [gameplay.md](gameplay.md), [rpc log](evidence/rpc-direct.jsonl) and cited snapshots. Artificial roots prove execution in those edited states only.

Unresolved/deferred: boss/ending not verified. Final camera/player near-edge patch added565 code bytes but yielded stale/blank streamed graphics; no terminal screen. Do not retry unchanged. Need coupled map-column/object-stream initialization or supported boss/terminal-event transition. Exact patch and obstacle in ENDING-001/scenarios. Natural eight-button cheat entered during loading failed timing; source-supported flag/index route verified instead. Character/special sprite identity still needs semantic commentary, despite successful input execution.

Ready redirect leads in [leads.json](leads.json): SCENE-DATA-001 follows descriptor consumers through$D1E1/$D2D6 to data-bank streams and boundaries; STATIC-CALL-001 establishes bank-preserving fixed-helper contracts before following post-JSR continuations; ENDING-001 follows$22 interpreter and coupled streaming state. CHEAT-001 verified. No terminal rejection review claimed; these are concrete productive research frontiers.

Toolkit: [NES-001](toolkit-issues.md), confirmed generic-input schema silently ignored by NES adapter; use buttons arrays and explicit releases. One confirmed bug, no suspected/missing-feature findings, no blocker. Configured toolkit unchanged. No cleanup/deletion, archive, publication or ROM edits. Resume via configured serve, map.load and cited durable snapshot imports.
