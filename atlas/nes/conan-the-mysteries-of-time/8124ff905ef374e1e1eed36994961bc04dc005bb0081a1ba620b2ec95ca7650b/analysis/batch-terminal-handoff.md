# Authorized fallback final handoff

This record supersedes the prior terminal-incomplete outcome for the explicit batch exception only. Historical [terminal handoff](terminal-handoff.md) and its rejection review remain preserved. The original evidence-backed classification gate is still unmet for one byte; this batch accepts its user-directed Manual Data annotation.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Evidence-backed classification covers 131087 bytes: Code 26849 and Data 104238. The additional one-byte generic Data region at physical [117489,117490), hexadecimal [$1CAF1,$1CAF2), CPU $CAE1, fixed physical 8 KiB PRG bank 14, has Manual provenance. Its meaning and original code-versus-data role remain unresolved. Redundant RTS/no-op callback and spare data remain compatible. This annotation asserts no padding, unused-byte, execution or reachability semantics. The user explicitly authorized this fallback only after verifying at most ten residual bytes; the exact precheck found one. No other bytes use this exception.

The [authorization and exact precheck](fallback/authorization-and-precheck.json), [supported annotation transcript](fallback/rpc.json), [counts and verification](fallback/counts-and-verification.json) and [phase handoff](handoffs/manual-fallback-disassembly.md) retain the distinction. Existing 906 classified regions remain unchanged. No further redirect research was required or performed.

Current [map](../maps/current.json), [state](state.json) and fresh [checkpoint](../maps/manual-fallback-disassembly.json) agree. The [checkpoint rebuild](manual-fallback-disassembly-verification.json) is byte-identical. Final commentary and acceptance are recorded separately in the commentary handoff and batch final verification; those gates must pass before this handoff is accepted.

All prior evidence, investigations and phase handoffs remain local. No toolkit modification, ROM modification, archive, deletion or publication is authorized or performed. Only the supported ROM-free portable mapping is a sharing deliverable. No hardware/game compatibility claim follows.

Final [commentary handoff](handoffs/manual-fallback-commentary.md) confirms all 907 scaffold entries and 1016 units reviewed with zero pending readability ranges. [Supported audit](../commentary/audit.json), [strict editorial checks](fallback/commentary-strict-audit.json) and fresh verification pass. The toolkit offers no strict CLI flag; explicit inventory/history/editorial checks supplement the supported audit. [Independent final verification](batch-final-verification.json) and [requirement audit](batch-completion-audit.md) record acceptance. Archive options select this fallback record, both new phase handoffs and all additional final/history documents; no archive execution occurred. No toolkit issues found.
