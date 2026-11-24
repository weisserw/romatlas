# Round 1 disassembly handoff

- Scope: first disassembly pass on a fresh workspace (Castle of Dragon, SETA 1989 US; UxROM mapper 2, 128 KiB PRG, CHR-RAM). Baseline created and verified.
- Checkpoint: **pass-001b** (maps/pass-001b.json, generated/pass-001b, analysis/pass-001b-verification.json). pass-001 = earlier map of this pass (superseded, kept).
- Verification: ok, byte_identical true. State saved to analysis/state.json; maps/current.json exported and validated.
- ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown). Incomplete: $C415 (phys 0x1C425).

## Findings
- Code only in bank 7 (fixed, 14241 B) and bank 6 (10490 B, $90D8-$BFE9 with tables). Banks 0-3 = CHR tile graphics proven by LoadTileSet ($C51B) descriptor extents; bank 4 = data + PaletteTable $BC00; bank 5 = data.
- Core runtime: SetPrgBank $C50D, cooperative tasks (CreateTask $C200, RunTasks $C17E, TaskYield $C1DC; entries $D002, $F712).
- Stage index RAM $052A (0..7) — shortcut used to visit all 8 stages; hidden TEST MODE via A,A,Left,Left x8 at title ($D0CA).
- Details: analysis/investigation.md, analysis/gameplay.md, analysis/scenarios.json, map evidence strings.

## Unresolved / deferred (analysis/leads.json)
- L-001 one byte $C415 (lone RTS after RTS; code vs padding).
- L-002 review of 306 bytes of unreferenced fragments accepted as Static code at confidence 0.75.
- L-003 ending/credits route not reached (dragon defeat RAM contract unknown).
- L-004 bank 4/5 data semantics (optional; already classified generic data).
- L-005 bank context of JSR $BEFE at $F161 (unobserved caller).
- L-006 per-option test-mode exercise (STAGE option may replace the $052A patch).

## Artifacts
- analysis/evidence/round-1-disassembly/tools (local decoder/merge/annotate scripts; they expect a scratch dir analysis/scratch)
- analysis/evidence/round-1-disassembly/traces (gzipped coverage/context/edge aggregates and bank 0-5 read sets)
- analysis/evidence/round-1-disassembly/snapshots (map-screen, test-mode, stage-1 snapshots, gzipped)
- analysis/evidence/round-1-disassembly/frames, regions.json
- analysis/toolkit-issues.md: TI-001 missing feature (discovery stops at JSR on UxROM), TI-002 suspected bug (Read events include fetches), TI-003 trace throughput. None blocking.
