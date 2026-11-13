# Round02 independent disassembly review (active)

Input maps/round01-disassembly.json, 28276 classified /102812 unknown. Previous commentary source retained unchanged. Configured binary used throughout; no toolkit edits.

Independent analysis.decode rechecked3682 unique proposed execution views: zero opcode/length/reference/successor mismatches (independent-decode.json, decode-discrepancies.json). graph-review.json checks all supplied parent edges and physical mappings,64 exact LDA#selector/STA59/JSRC533 contracts and26 explicit descriptor consumers. No discrepancies. Code roots are source/descriptor/conditional control-flow contracts, not pointer-only claims; actual artificial fetch provenance remains separate.

60 derived helper summaries independently re-evaluated path/stack/write invariants (summary-review.json). Initial helper bodies saved helper-*.txt and source reviewed. Correction: seed E7E0 decodes KIL in an operand, not a routine. It is absent from every JSR target in connected-walk.json, hence zero dependent summaries, proposed starts or byte ranges. The seed is removed in all new work; previous evidence retained. Research-model defect, no toolkit issue.

Fourteen additional saved-selector wrappers prove complete save/select/descriptor/restore idioms, target-local stack balance and all standard-return exits at explicit selector restoration (saved-selector-wrappers.json). Connected continuations under these effective bank contracts are being decoded; unresolved summaries retain localized reasons.

Independent data_review.py re-derives228 claims /40522 unique bytes: six16-word map tables, fixed map layers, sparse popcount literals, terminated3-byte events, 28 credit strings and pointer tables, all graphics read spans, finite encounter pointer tables and zero tails. Existing/newCode conflict0. Source D42D/EB1E/D4FF/ED3A and retained primary runtime/template observations reviewed. Mixed graphics/audio/fixed containers use Data with bounded mechanical comments; no specific serialization/object extent is inferred from read overrun. Audio840-byte overcopy is not a blanket Data contract.

Correction: bankpair2 encounter container ends CPU BF30/physical7F40 beside127 bytes of visibly repeated-nibble data, not nextCode. This changes the endpoint rationale, not the supported noncode claim. Independent boundary inspection 7EE0..8006 shows graphics masks before7F40; 7F40..7FBF bytes have identical high/low nibbles (00,11,..FF), then279-byte aggregate zero tails includes local7FBF..8006. This local127-byte region is being inspected as generic patterned Data; role/reference unresolved, no fill claim. Other five containers end at known Code/reset bridge.

Gameplay boss/ending/credits/THE END primary snapshots and accepted artificial traces reused. No reason to replay unchanged progression. Terminal exhaustion not claimed; next phase commentary then redirect of localized unknown control/bank/data questions.

## Final pass closure

All48814proposedbytes accepted, plus127patternedData,933source-consumed adjacentData,187localwrapper/resetCode and430connectedCode. Totalgain50491; Code26829,Data51938,Unknown52321/164gaps. Finalmapmaps/round02-disassembly-complete.json freshlybyte-identical; preservation.json confirms all704originalregions and previouscommentarysourceunchanged. Finalexactmappercontext walk exhaustedqueue; no additionalinvocation/reachabilityassumptions. Leadoutcomes/counts/perbank/nextquestions in handoff-round02-disassembly.md and counts.json. No toolkitissuesfound.
