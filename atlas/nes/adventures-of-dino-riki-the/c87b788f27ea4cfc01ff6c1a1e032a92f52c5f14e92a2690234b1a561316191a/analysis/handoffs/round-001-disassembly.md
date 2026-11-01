# Round001 disassembly handoff

Phase complete; ROM incomplete.

ROM classification: 100.00% (65551 / 65552 bytes classified; 1 bytes unknown).
Exact 99.99847449353186%; rounded 100.00% leaves 1 unknown byte.
Checkpoint [map](../../maps/round-001-disassembly.json),
[source](../../generated/round-001-disassembly/game.asm),
[verification](../round-001-disassembly-verification.json): all validation passed,
fresh rebuild byte-identical. State: analysis/state.json; current: maps/current.json.

11837 code +53714 data/image/header bytes. Per-bank details in
[progress](../progress.md) and [counts](../round-001/counts.json). All150 regions
have labels and evidence; code reachability is separate.3353 unique physical
opcode starts fetched;2421 natural,3066 artificial,932 artificial-only.

L001 vectors/startup, L002 broad progression, L003 data barriers and L004 connected
object/audio handler contracts verified. Natural stage1; shortcuts visited desert,
swamp, all stage 4 variants, four boss types, final defeated transition back to stage 1
($48=0,$C0=1). [Scenarios](../scenarios.json) preserves exact patches, budgets,
inputs and durable snapshots. Generic data roles accepted despite partial semantics.

L005 ready: sole physical$39C7/CPU$B9B7 byte60 between tail JMP$BA53 and next routine.
[Residual review](../round-001/residual-review.json) and leads.json give focused
redirect recipe. Do not infer intended code from an isolated RTS or data from no
execution. No terminal exhaustion assessment; commentary then redirect should
review this localized ambiguity.

Readability scope: entire current source, all 11837 code bytes and 53714 data bytes,
including broad code labels and numeric RAM operands. Structural descriptions exist;
commentary/full editorial review has not run. Preserve accepted boundaries and
separate reachability from structural roles.

[Toolkit issues](../toolkit-issues.md):1 confirmed docs/parameter bug NES-DOC-001,
0 suspected,0 separate missing features,0 blockers. Use buttons array and
max_instructions; legacy button/pressed and budget fields silently ignored by NES.
Disk ample, scratch4 KiB, evidence retained locally. Input unchanged.
