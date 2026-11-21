# Round002 disassembly handoff — incomplete

Exactly one pass prompts/disassembly.txt completed. Next **full current-source commentary**, then redirect. No global completion or terminal exhaustion decision.

ROM classification: 99.99% (262141 / 262160 bytes classified; 19 bytes unknown).

Input pass-001-final256690/262160; accepted all138 reviewed slices/5451 bytes=4832Code+619Data. Lead R0013407, R0021901, R003143 accepted; histories retained in [leads.json](leads.json). [Independent review](round-002/review.md), [decisions](round-002/decisions.json), [fresh exact decodes](round-002/independent-decode.json). Critical contracts checked independently rather than relying on CFG closure: objectPtr six-byte RAM records and synthetic A089 return, ordinary render/allocator restores, three ending dispatchers, audio table/stream handler bodies and explicit stack unwinds. Static structural provenance; no new execution or natural entry claims.

Saved internal state/current/fresh maps/pass-002-final.json all SHA256 e88d90dd5992842c20317e59619cee3148dd84ce23395d5f01bb40f9f63e0eff. [Verification](pass-002-final-verification.json): byte_identical=true. [Identities](round-002/checkpoint-identities.json).737 evidence-backed regions;20669Code+110400Data+131072Image. [Per-bank counts](round-002/counts.json).

Use [current unknown complement](round-002/unknown-ranges.json): physical8714:1,13913:3,105610:1,130820:4,131072:10. Historical analysis/unknown-ranges.json is preserved pass001 evidence, **not the current complement**. Previous [residual review](redirect/round001-static-continuations/residual-review.json) remains narrow, not terminal. No invented separation/sentinel/table claims for these ambiguous fragments.

Physical103810..103813 is three-byte selector **Data**, CPU$9572..$9575; next code starts$9575. Actual pause helper opcode$9E atCPU$D8C4 preserved as `.byte $9e,$00,$07 ; shx a:$0700,y` in generated/pass-002-final/game.asm:10685. Commentary must retain uncertainty instead of silently treating this as STA abs,X. Audio pointer extent49 words is physical storage evidence, not complete selector-domain proof. MMC1 duplicate writer classification is static; no post-write mapping propagation or actual entry claim.

Commentary source remains unchanged8292800b9b7ca4fad4f5e65d9f3d8b2e8098c225ace6c07ad771ac34b55448fb with frozen phase1 baseline. **Full current-source readability now pending** because new spans were classified. Preserve and integrate existing notes/symbols into the current checkpoint through supported commentary workflow; do not overwrite historical commentary evidence. This disassembly did not edit commentary.

Broad representative gameplay previously completed; no unchanged replay. Natural Mountain; artificial allother4areas/late enemy line/congratulations/aircraft snapshots in scenarios.json. Next-loop post-aircraft remains unobserved and no concrete useful changed method linking it to residuals identified. Static connected batch included allocators/ending/audio downstream paths and independently supported data; no productive continuation deferred solely for lead-range scope.

No toolkit issues found:zero confirmed/suspected bugs, missing features, blockers; [report](toolkit-issues.md). Local client pagination error stopped before mutations;97MiB repetitive log retained, bounded final log separate. Annotation-shape errors were corrected before final successful save. Disk3.4GiB available; round002111MiB retained. No cleanup/archive/delete/publish/toolkit edits; ROM unchanged. After commentary, redirect should assess strongest residual inferences under evidence-policy.md before any terminal recommendation.
