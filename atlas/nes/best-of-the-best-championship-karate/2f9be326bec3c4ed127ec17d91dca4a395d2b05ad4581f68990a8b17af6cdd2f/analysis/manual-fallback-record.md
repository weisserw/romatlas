# Final manual fallback record

The user explicitly authorized this batch-only exception after the previous justified terminal-incomplete handoff: verify the exact remaining complement first; if at most ten physical ROM bytes remain unclassified, annotate the listed residuals as generic Data with Manual provenance and unknown meaning. This authorization overrides further structural-proof requirements for these bytes only. More than ten would have stopped this exception; other blockers remain blocked. No broader exception is inferred.

The independently measured pre-change complement was exactly seven bytes, matching the supplied ranges. Half-open physical file offsets, including the iNES header:

| Offset | Length | End | Prior ambiguity |
|---|---:|---:|---|
| 199683 | 3 | 199686 | JMP $8A5C at bank24 CPU $8BF3, context [24,25,30,31]; independent entry/layout unresolved |
| 249693 | 3 | 249696 | JMP $CDDC at bank30 CPU $CF4D, fixed banks30/31; veneer versus residual storage unresolved |
| 260427 | 1 | 260428 | RTS at bank31 CPU $F93B, fixed banks30/31; independent control/stack entry unresolved |

All three intervals now have kind Data and provenance Manual through the supported map.annotate API. Meaning remains unknown. The API confidence value is not evidence of structural truth: the evidence fields explicitly identify user authorization and preserve code-versus-data ambiguity. No code, non-code proof, original format, reachability, deadness or game semantics is established by this fallback.

[Exact pre-change complement](user-fallback-01/complement-before.json), [preserved state](user-fallback-01/state-before.json), [supported API transport](user-fallback-01/rpc.jsonl), [disassembly handoff](handoffs/user-fallback-01-disassembly.md), [prior decisive rejection reasoning](redirect/round-02-a/rejection-review.md). No new redirect research was needed or performed. Lead IDs R02-L01..03 retain blocked structural-inference status, with the new annotation disposition recorded separately in leads.json.

Annotation coverage is 262160/262160 bytes, zero unclassified; 262153 prior evidence-backed bytes plus seven user-directed Manual bytes. Code is 25622 bytes; Data is 236538 bytes including the Manual seven. This is completion under the explicit exception, not evidence-backed structural proof for the entire ROM. Unknown meaning is not an Unknown region kind.

State/map validation and the fresh user-fallback-01 checkpoint passed byte identity; map SHA256 e870ff399453b69cc107cf3e5ac76dad6048e16e75910b144cc0a7b1dc0e7525. ROM SHA256 remains 2f9be326bec3c4ed127ec17d91dca4a395d2b05ad4581f68990a8b17af6cdd2f. [Checkpoint verification](user-fallback-01-verification.json). Current-source reconciliation and final strict audit are separate requirements recorded in the final handoff and completion audit.

This record is selected explicitly in archive-options.json for retention. Prior conclusions, evidence and checkpoints are preserved. No archive execution, deletion, toolkit change or publication is authorized or performed.
