# Round 003 disassembly — verified localized batch, incomplete

Input round-002-commentary-final / structural round-002-disassembly-final. Output [phase checkpoint](../round-003-disassembly-final.json), [fresh byte-identical verification](../round-003-disassembly-final-verification.json), map maps/round-003-disassembly-final.json, source generated/round-003-disassembly-final/game.asm. Exactly one pass. No final audit or terminal exhaustion decision.

ROM classification: 100.00% (163848 / 163856 bytes classified; 8 bytes unknown).

Rounded100.00% is incomplete. Exact residual is8bytes in7spans, correcting the redirect's hypothetical8spans shorthand. [Counts/per-bank](../round-003/disassembly/classification.json), [exact physical unknown complement](../round-003/disassembly/unknown-ranges.json). Totals23097Code,9679Data,131072Image. PRG0:8192classified/0unknown; PRG1:8191/1; PRG2:8188/4; PRG3:8189/3. Header16/0 and each of16CHRbanks8192/0.

R004/R005/R006 verified structurally:8StaticCode +1genericStaticData bytes, four annotations. All531prior region objects preserved exactly, with labels/comments/evidence/views. [Independent review](../round-003/disassembly/review.md), [fresh decode](../round-003/disassembly/fresh-contextual-decodes.json), [primary boundary checks](../round-003/disassembly/boundary-check.json). No observed/natural-reachability upgrade.

- R004 physical31885..31887 CPUFC7D..FC7F: completeLDA06 after finite960-store loop. Homologous19byte loop ends inLDA47, not identical21bytes. No inferredattribute/fullroutine meaning. FC7Funknown,FC80Data preserved.
- R005 physical16634..16637 CPUC0EA..C0ED and20162..20165 CPUCEB2..CEB5: two origin-specific closedJMP sinks at independently bounded terminal/object junctions. Combined primaryconstraints support exactcode beyond legaldecode, no actualingress claim.
- R006 physical31922..31923 CPUFCA2: genericData within34zero/20/9zero neighborhood. Source musicconsumers/initpointers checked; no specificschema supported or claimed.

Connected continuation checked: selfjumps close, neighboringspawn/palette/music alreadyclassified; LDA continuationA2 would consume acceptedFC80Data, unsupported. No unchangedruntimeplay needed; broadflight/late mission,rescue/base,realcallerreturn,cleanartificialending alreadyrepresented.

Remaining: AD7D,DAFE,DB21,DB2E,DB44,FC68..FC6A,FC7F (CPUhalf-open). Fourstack-tailRTS followterminalJMP whosehelpersRTSdirectlytocaller, not thosebytes. AD7DextraRTS vs residue; FA60undocumentedhelper vs residue; loneFC7FA2 vs operandcrossing remain materialcode/dataambiguity. No missingcaller universalprerequisite used. Redirect may review strongestremainingrejectionpremises underpolicy; no terminalexhaustion claimed.

Maintained commentary/source/game.asm unchanged SHA25659d7c4719acfa1905c4ac26898d5f24a211b5cbb4478019f9cf57b50088dfef4. Commentary must merge four new blocks and revise obsolete boundary prose (especially oldthree-byteFC7Dfragment andsparse20/selfjumps) preserving priorlabels/comments. Full current-source review then redirect; finaleditorialacceptance pending. No sourcebaseline reinit.

[Toolkit issues](../toolkit-issues.md): no toolkit issues found; no toolkit changes. Configured releasebinary used. ROM identity unchanged; state=currentmap=checkpointmap. Disk approximately9GiBfree, boundedlocal evidence, no deletion/archive/publication or liveprocess.
