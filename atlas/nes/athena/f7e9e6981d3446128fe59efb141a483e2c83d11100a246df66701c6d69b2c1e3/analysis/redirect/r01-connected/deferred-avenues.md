# Deferred questions after the combined ready batch

This is a normal ready-lead handoff, not terminal exhaustion. All proposals await independent disassembly. The current map still has 88,457 unknown bytes. If every proposal is accepted, the following nine intervals, totaling 27 bytes, remain structurally ambiguous:

| Physical interval [start,end) | Bytes | Actual remaining issue |
| --- | ---: | --- |
| 00F9C3..00F9C7 | 4 | Four EA bytes: executed NOP sequence versus non-code alignment/fill is not discriminated by the adjacent tile-data boundary. |
| 018232..018235 | 3 | Three EA bytes between a tail-jump and an explicitly addressed five-byte graphics record. Neither record membership nor an instruction entry is supported. |
| 01944E..019451 | 3 | Standalone JMP D17E trampoline. The extra dispatcher word at81B4 points here, but selectorB2 is excluded by CMPB2; pointer plausibility plus a lone transfer does not independently establish intended code. No unused/dead claim. |
| 01967F..019680 | 1 | Standalone RTS; no independent bounded routine role. |
| 01BE04..01BE0B | 7 | Mixed0D/7A/60/1A bytes. Short data-record and instruction interpretations remain possible; no established consumer or record boundary discriminates them. |
| 01C490..01C493 | 3 | Standalone JMP C454 between existing paths. Code versus three-byte embedded constant remains unresolved; no field/stack routine contract beyond the transfer itself. |
| 01D4AF..01D4B1 | 2 | A5 7D could be an alternate LDA prefix or a literal pair; exact entry/record interpretation remains unsupported. |
| 01E609..01E60A | 1 | Standalone RTS after the graphics-script loop; a return opcode alone does not establish an independent routine. |
| 01F2D1..01F2D4 | 3 | JSR E108 between a preceding unconditional jump and a supported convergence path. It mechanically fits a flag-clear prefix, but the fragment has no independently established entry or inline-data role. |

Missing natural execution is not a universal rejection reason. These narrow claims lack sufficient *structural* discrimination beyond a lone opcode/transfer, short literal pattern or ambiguous boundary. The independent phase should compare the ready routine contracts consistently; it may find a stronger local inference. Do not replay gameplay or fabricate entry points merely to execute these bytes. A new caller/consumer, a supported alternate-entry contract or a narrower format proof would change their disposition.

The other investigated questions are closed for this pass as follows:

- **Complete player/progression initialization:** resolved by task04 E460/stack47 with mode94=1. Representative stages0/2/4/6/13, composite boss37 and post-boss terminal throne scene were visited artificially. No separate credits transition was identified; F2C4..F2DD pins stage13/world0B in a scheduler yield loop. Further arbitrary flag or actor sweeps have no supported target.
- **Connected actor families:** main B2-bounded dispatch, eleven secondary tables, their handlers/shared helpers and the representative boss children were followed in the same batch. EC=FF and bank-restoring API contracts are recorded. No recursive gameplay enumeration is needed. Small actual helper/parameter gaps were inspected separately rather than deferred because they belonged to another actor.
- **Sprite/audio extent questions:** consumer grammar, encoded origins and independent next-container/code endpoints provide ready non-code proposals. The aux offset view overlaps extra-pointer bytes at indices194..203; this limits the original offset-table schema, not code-versus-data classification. No exact original generator or full global valid selector domain is claimed or needed for generic Data.
- **Generic data:** terrain, sprite/audio backing, stage/text/palette/animation/motion data and local patterned lookup bytes were inspected against consumers and supported code. Unknown semantics did not cause clear non-code to remain unknown. Generic Data proposals do not claim original source layout. Review large regions locally if new code evidence appears.
- **Graphics counterexample:** existing Data region314 must be split around the actual64-byte code at013E30..013E70. Adjacent patterned Data remains justified; do not invalidate the entire32KiB graphics region. The draft bank2 data endpoint was also corrected fromBF8D to the independently calledBF70 helper; that draft had never changed the map.

Primary evidence: proposals.json, unknown-ledger.json, proposal-checks.json, stack-control-audit.json, graphics-code-conflict.json, local-routine-contracts.json, sprite-format.json and the preserved failed sprite models. Research history and exact routes remain in frontier.md, scenarios.json and per-experiment runtime/context files.
