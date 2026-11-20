# User-directed fallback disassembly — complete, commentary pending

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

This is annotation coverage including exactly 7 user-directed Manual fallback bytes, not a claim that all bytes have evidence-backed structural proof. Prior evidence-backed classification remains 262153 / 262160 (99.997329875%); Code 25622, prior Data 236531. Current Data 236538; provenance totals Static 252971, Observed 9182, Manual 7. Execution coverage unchanged. Current readability/editorial acceptance requires the separately authorized following phase.

Explicit new user authorization, relayed by coordinator: verify the exact physical unclassified complement first; stop if it exceeds 10 bytes; classify only these residuals as generic Data, Manual provenance, meaning unknown, user-directed fallback rather than structural proof. Expected and measured complement is exactly 199683..199686 (3), 249693..249696 (3), 260427..260428 (1), half-open physical file offsets. No exception expansion occurred. See [pre-change complement](../user-fallback-01/complement-before.json) and [preserved pre-change state](../user-fallback-01/state-before.json).

The original ambiguity remains unresolved and retained in [primary rejection review](../redirect/round-02-a/rejection-review.md) and [residual ledger](../redirect/round-02-a/residual-ledger.json):

- Bank24 CPU $8BF3, context [24,25,30,31], `4C 5C 8A` JMP $8A5C: plausible menu-refresh stub versus retained instruction-shaped residue. Prior handler jumps away; fourteen-word menu dispatch supplies no entry here, unlike the accepted duplicate transfer at $8C46.
- Bank30 CPU $CF4D, context [unknown,unknown,30,31], `4C DC CD` JMP $CDDC: plausible reset veneer versus residue after returning OAM initializer. No independently bounded slot array; raw pointer coincidence lies inside established graphics payload.
- Bank31 CPU $F93B, context [unknown,unknown,30,31], `60` RTS: plausible standalone return versus retained byte. Adjacent helper already returns; no independent stack/control entry or repeated-terminal layout establishes this role.

Earlier reference/consumer, neighboring control and format/layout inspection is reused without research reopening. Manual annotations explain these limits individually. No code role, non-code proof, reachability, deadness, fill purpose or specific semantics is asserted by the fallback.

Supported RPC sequence: `api.describe`, `map.load {path:analysis/state.json}`, three `map.annotate {region:{offset,length,kind:Data,provenance:Manual,...}}`, `map.validate`, `map.save {path:analysis/state.json}`, `map.export {path:maps/current.json}`. Reproducible local script and complete bounded transport log: [apply.py](../user-fallback-01/apply.py), [rpc.jsonl](../user-fallback-01/rpc.jsonl). All preexisting regions are verified unchanged. [Counts](../user-fallback-01/counts.json), [per-bank counts](../user-fallback-01/bank-counts.md), [RPC results](../user-fallback-01/results.json).

Fresh unique command `python3 run.py checkpoint user-fallback-01` validated, generated and verified byte-identically: [report](../user-fallback-01-verification.json), maps/user-fallback-01.json, generated/user-fallback-01/game.asm. Current/checkpoint map SHA256 e870ff399453b69cc107cf3e5ac76dad6048e16e75910b144cc0a7b1dc0e7525. Original ROM SHA256 remains 2f9be326bec3c4ed127ec17d91dca4a395d2b05ad4581f68990a8b17af6cdd2f. Internal state and portable map validate. Rebuild fidelity proves preservation, not the fallback's structural truth.

Disk checked before bounded work: 4.3 GiB free, analysis142 MiB/generated7.8 MiB/commentary6.1 MiB. No deletion or cleanup. Existing evidence, checkpoints and handoffs preserved; coordinator separately preserved earlier terminal/completion/archive documents. No ROM/toolkit edits, research reopening, emulator execution, archive execution or publication. Toolkit report retains TK-001/TK-002: 2 confirmed nonblocking documentation/API bugs, 0 new findings, 0 suspected, 0 missing features, 0 blockers; [report](../toolkit-issues.md).

Next: separate commentary phase updates current-source coverage and fallback notes while retaining the explicit Manual distinction; coordinator handles subsequent gates. This phase performs disassembly fallback only.
