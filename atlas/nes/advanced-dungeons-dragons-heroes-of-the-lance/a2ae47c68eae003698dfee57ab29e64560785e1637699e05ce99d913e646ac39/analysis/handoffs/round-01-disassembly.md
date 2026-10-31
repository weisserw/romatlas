# Round 1 disassembly handoff

Phase complete; ROM incomplete. Next phase: full current-source commentary, then redirect. No commentary/redirect executed here. This is ordinary pass handoff, not terminal exhaustion.

ROM classification: 65.50% (171722 / 262160 bytes classified; 90438 bytes unknown).

Code23118, Data17516, Image131072, Text16. Header16 classified; each CHR8KiB bank8192 classified/0unknown. Exact PRG8KiB counts and physical residuals: [current-counts](../evidence/current-counts.json); table also in [progress](../progress.md) and [investigation](../investigation.md).

Authoritative [state](../state.json), [portable map](../../maps/current.json), [pass-004 map](../../maps/pass-004.json), [generated source](../../generated/pass-004/game.asm), [verification](../pass-004-verification.json): map valid, ok=true, byte_identical=true. Map SHA256 d8c5f7e719b59cb825ed5679edfa957367d48bd91ecf98947d097bf4b26e95c7. Input hash unchanged. Baseline/pass001–003 retained; no deletion/cleanup/publishing/toolkit edits.

Natural milestones: title, all eight portraits, first doorway0->1, Main Menu, Goldmoon status19/19HP, mage spells and Charm selection. Artificial supported transition visits:20/40/52,0F entries1/3,2B entry2; combat/pits and waterfall cutscene, no dragon or ending. Exact clean shortcut: transition-contract.snapshot, PCDEA7 A1 X2 SP247 banks[0,1,14,15]; edit A/X only, preserve caller stack, then normal initialization. Do not claim a room byte write alone as visit. [Scenarios](../scenarios.json), [gameplay](../gameplay.md), durable [replays](../replays).

Static review accepted direct control/callee paths, literal MMC1 bank entry contracts, header/CHR, exact uniform non-code, bank1 sparse records, eight-entry initialization tables, debug flag/title fragment, debug hexadecimal tiles and11four-byte transition hooks. Source annotations retain structural evidence; commentary must still supply meaningful routine/branch/RAM labels and behavior explanations. Generic record meaning remains unresolved.

All cited aggregate_dropped=0; capacity0 discarded details intentionally, initial capacity2000 overflowed. Untraced waits do not prove execution. Bounded timeouts retained partial state. External subprocess exits143 were handled with durable snapshots and short direct-RPC batches; no toolkit defect inferred. Clerical follow-up interrupted before durable result, not claimed successful.

Next questions [R001–R004](../leads.json): infer boss/ending contract before further map patches; inspect large banked table/data consumers; prove selected bank-preserving return continuations; targeted clerical/inventory follow-up if useful. Prior map0F/2B results reject only immediate boss/ending at tested entries. No broad terminal rejection asserted.

**No toolkit issues found**:0confirmed,0suspected,0missing-feature,0blockers. [Report](../toolkit-issues.md). Disk11GiB free, scratch344KiB, selected evidence~126MiB at last check. Local input/state/assets remain local; only supported ROM-free map is portable.
