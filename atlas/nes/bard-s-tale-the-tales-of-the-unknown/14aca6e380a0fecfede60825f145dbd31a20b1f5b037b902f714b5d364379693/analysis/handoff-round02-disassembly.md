# Round02 disassembly handoff

Exactly one disassembly pass completed; ROM incomplete. Next: current-source commentary, then redirect. No terminal exhaustion assessment asserted.

ROM classification: 60.09% (78767 / 131088 bytes classified; 52321 bytes unknown).

Code26829, Data51938, Unknown52321 across164gaps. Gain50491 = Code8909 +Data41582. Header16bytes included in Data. Exact perbank counts in round02/counts.json and progress.md; exact complement round02/unknown-complement.json. PRGbanks2,3,7 now fully structurally classified. All704 inputregions preserved exactly (round02/preservation.json).

Authoritative finalcheckpoint: **maps/round02-disassembly-complete.json**, equal byte-for-byte to maps/current.json, semantically equal analysis/state.json. Generated source generated/round02-disassembly-complete; verification analysis/round02-disassembly-complete-verification.json confirms byte_identical=true. MapSHA25691d07d0cf3a37c0763c89b1bbdc304b58bf5ffe4155fa61093f64ef16acc3c90. ROMSHA25614aca6e380a0fecfede60825f145dbd31a20b1f5b037b902f714b5d364379693 unchanged. Configured binary /root/romatlas/target/release/romatlas used.

Historical checkpoint maps/round02-disassembly.json was created while failed in-memory annotation attempts had left saved state unchanged; it is an unchanged round01input checkpoint, not finalclassification. maps/round02-disassembly-final.json is a verified intermediate77217classified-byte map. Preserve both. All sources/evidence/history retained; no archive, publication, toolkit edits or cleanup/deletion.

## Independent acceptance and corrections

All48814unique proposedUnknownbytes from redirect round01-r02 accepted after independent review:8292Code +40522Data. Plus127repeated-nibbleData bytes at physical7F40..7FBF. Independent decode rechecked3682executionviews with zero opcode/length/reference/successor discrepancies; graph-review.json validates parent edges,64literal MMC1selector patterns and26explicit descriptor consumers. Summary-review.json independently recomputes60derived bank/stack helper summaries; initial helper source/body inspection saved helper-*.txt.

Correction: source seedE7E0 is KIL/midoperand, not a helper entry. ZeroJSRcallers in supplied fullgraph; zero dependent summaries/proposedbytes. Removed from all new work; old model/evidence preserved. This is a research-model mistake, not toolkit defect.

Correction: pair2 encounter container ends CPU BF30/physical7F40 beside repeated-nibbleData, not nextCode. Generic noncode acceptance does not rely on the incorrect nextCode premise. Exact127byte suffix has equal high/low nibbles at everybyte, ends at uniformzero boundary7FBF, no control/code conflict, descriptor/consumer meanings boundedly inspected (patterned-suffix-review.json). No fillpurpose claimed.

Data review independently rederives fixed map layers,16byte sparse masks/popcount literals,FFterminated3byte events,28creditrow strings/pointers, all D42Dreadspans and encounter tables. See data-review.json and local mixed-data-inspection.hex. Audio840byte overcopies and graphics cursoroverreads never prove serializedobject extents. Specific roles retained where supported; generic mixedData for unresolved individual audio/encounter/fixedrecord meanings. ROMtemplates are Data despite RAMgeneratedexecution.

Additional local933Data bytes: source-consumed fixed lookup/text/pointer/period tables and pair2 physical5553..55E3 map-adjacent noncode pointer/pattern records. Exact ranges, role/consumer inspection and contextual references in local-data-review.json. Local187StaticCodebytes: complete repeated savedselector wrapper fragments and understood resetcontinuations (local-code-review.json).

Fourteen additional savedselector wrappers have balanced localstack/allstandardreturn exits at explicit selectorrestore (saved-selector-wrappers.json). Ordinary descriptor/mapper connected batch followed beyond originalranges:348+69+13 =430additionalStaticCodebytes in local-connected-new.json,mapper-connected-new.json,final-connected-new.json; independent freshdecoderesults and parent/context graphs saved. Main48941 +local1120 +connected430 =50491gain. Final connected walkqueue empty within bounded budget; exact immediate mapperselection modeled, unknown lowerbanks remain null. Staticconditionalreturn preconditions are structuralclaims, not naturalreachability. Original326acceptedartificialfetchstarts remain Observed after verification against primary contextualroots; static extraaliases are explained separately.

## Commentary requirements

**Entire existing commentary tree untouched.** Previous commentary/source/game.asm SHA256069c4fb726bd1a71b5e64650094d65cf3047ed792a36599fc46620e098abac78 unchanged. Next commentary worker should freeze/preserve the entire prior reviewed tree before refreshing generatedsource; carry all stillvalid labels/comments/reviews to newly classified source and review newbytes. Do not overwrite or discard previous material. Prior current-source readability gate applies only oldmap; finaleditorial/archive gate remains pending.

New source labels exist for well-supported localData and mainData roles; most newCode needs routine/branch behavior labels. Newly explicit ending80BB..8249, eventhandler branches, menu/battle services, generatedRAMtemplate inputs and graphics/audio/map records should receive coherent commentary. Source/contextual corrections in previous uncertainties.md remain sound. No new whole-game fidelity claim.

## Residual redirect frontier and stop rationale

All ready leads marked verified with overlapping perlead counts in round02/lead-outcomes.json; previous histories preserved in leads.json. New needs_redirect lead R02-RESIDUAL-DISPATCH-CONTEXT has concrete evidence/recipe. No unsupportedbytes swept intoData by exportfallback;52321bytes remainUnknown, not dead/unused.

Final-connected-barriers.json has155unknownbank/nonROM observations,22indirectcontrol observations and11mapperwrites; repeatedcontexts are not uniquequestions. Trace writers/transformations for RAM0032/003A/audio06F6; prove missing lowerbank return summaries; inspect newly connected bank12B463 streamtableB8A6 and remaining source-visible lookup/resourcecontainers. saved-selector-wrappers.json unresolvedreasons localize omitted callee/writecontracts. Finalfixedbanks14/15 residual264/71bytes include mixed unrooted stackrestore/routine/data fragments; don't infer entiregaps from legaldecode or reject because no naturalcaller. Much bank8/9/10/11/12/13 content may yield genericData after bounded independent pattern/consumer inspection; no blanket wholebank claim.

Supported ready batch, practical adjacent easygains and ordinary connectedwrapper paths complete. Further useful control expansion needs those pointerwriter/calleebank contracts, so commentary/redirect is appropriate; this is not exhaustive recursiveclosure or a bytequota stop. No unchanged startup/progression replay. Existing scenarios already cover town,Cellar,finalarea movement,combat,Mangar,endingnarrative,credits and THEEND with artificialshortcut provenance; boss5/credits-finish snapshots reusable for a specific new question. Naturalcombatvictory/reachability/fullhardwarecompatibility not claimed.

No toolkit issues found; analysis/toolkit-issues.md. Failed annotationattempts were caller-side insertionid/labelsyntax mistakes, transactional and preserved in rpc.jsonl; no toolkit change. Diskcheck~6.1GiBfree, round02evidence~92MiB. Coordinator should print exact metric above before commentary.
