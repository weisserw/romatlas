# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 — disassembly (in progress)
Input checkpoint: initialized state, no classifications/leads. Read required policy/workflow/NES/protocol docs. Baseline generated and byte-identical verified: analysis/baseline-verification.json, maps/baseline.json.
Initial 180-frame traced request hit the documented execution timeout; no output snapshot survived that exploratory session. Retry with smaller 10-frame bounded chunks, persisting snapshots and evidence per chunk. No classification claim made from failed run. Remaining: startup/gameplay/static batch, validation, current-map export and fresh checkpoint.

Static checkpoint `round-001-disassembly-classified`: validated map and byte-identical generated source (analysis/round-001-disassembly-classified-verification.json). Classified 3292 PRG instruction bytes from vector/observed-root bounded successor discovery, 16 header bytes, 6 vector bytes and 131072 CHR image bytes. Earlier interim/static checkpoints retain empty-map fidelity outputs after rejected API annotation attempts; they are not classification milestones. API requires id=0 for new NES regions, corrected without toolkit edits.
Gameplay currently at Airwolf story dialogue (attack.png); the initial Start inputs entered dialogue, firing/turning inputs have not yet reached cockpit. Remaining this pass: bypass/wait through dialogue, actual flight/landing/progression route and practical RAM-contract exploration if available; ingest resulting fetch evidence; final checkpoint and handoff. Startup snapshots and play-log preserved. No terminal exhaustion claim.

Cockpit reached after bounded no-trace dialogue advancement: advance-4.png / advance-5.snapshot.json. Actual traced weapons/speed/turn/altitude routes saved in cockpit-log.json, cockpit-contexts.json and cockpit-* captures. Broad gameplay is not exhausted. Static progression inspection identifies $80 mission index, increment at CPU $CE65 and stage initialization at $CD39; $14 terminates first mission cycle at $CE7D and calls ending sequence $CE85. Saved progression-decode.json. Next bounded experiments restore clean cockpit and patch mission index with understood top-level stage entry; artificial outcomes will be distinguished from natural reachability.

Bounded static direct-call batch added 432 instruction bytes (13 supported JSR target roots; static-call-proofs.json). Artificial stage19 initialization executed and displayed distinct late mission map; ending entry displayed readable completion message with transient graphics corruption (late-mission.png, ending.png). Both are artificial observations, not natural mission completion. One follow-up advances each saved state and samples representative late cockpit mechanics / stable ending. Current state saved/exported after shortcut fetch batch (+651 code bytes), awaiting final follow-up ingestion and verification. RAM writes are only internal $80; PC/stack edits are documented in shortcut.py and shortcut-log.json; original ROM unchanged.

## Round 1 — disassembly completed (incomplete ROM)

ROM classification: 87.14% (142780 / 163856 bytes classified; 21076 bytes unknown).

Output checkpoint: `round-001-disassembly-final`, [verification](round-001-disassembly-final-verification.json), [map](../maps/round-001-disassembly-final.json), generated source `generated/round-001-disassembly-final/game.asm`. map.validate succeeded; generation and independent verify report byte_identical=true. Input SHA256 unchanged. Internal state `analysis/state.json` and `maps/current.json` match final saved map. Exact counts and unknown complement: [classification.json](round-001/classification.json), [unknown-ranges.json](round-001/unknown-ranges.json). All 393 classified regions have evidence. Execution coverage is separate: saved actual contextual opcode fetch sets under round-001/*contexts.json; structural successor candidates remain Static. Editorial readability is deferred to commentary, not complete.

| Physical bank | Code bytes | Data/image bytes | Unknown bytes |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG 0 | 2704 | 40 | 5448 |
| PRG 1 | 3873 | 70 | 4249 |
| PRG 2 | 2491 | 355 | 5346 |
| PRG 3 | 2022 | 137 | 6033 |
| CHR 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 0 |
| CHR 4 | 0 | 8192 | 0 |
| CHR 5 | 0 | 8192 | 0 |
| CHR 6 | 0 | 8192 | 0 |
| CHR 7 | 0 | 8192 | 0 |
| CHR 8 | 0 | 8192 | 0 |
| CHR 9 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

Lead outcomes: D001 verified cockpit weapons/speed/heading/altitude naturally; D002 verified late mission initialization and speed/weapons artificially (late-cockpit.png shows speed752 and changed stage map), D003 verified artificial completion-message branch and subsequent intelligence briefing; transient corrupt ending graphics prevents claiming faithful natural ending presentation. D004 verified static direct-call batch (432 bytes). No inherited leads existed. No rejected structural lead is being treated as permanently exhausted.

Easy-gain boundary: substantive static and representative ordinary flight gains and concrete stage/ending experiments are complete for this pass. Gameplay is not exhausted: prisoner landing/rescue/base service and natural mission-exit remain unvisited; concrete missing facts are their coordinate/trigger/return contracts, identified for redirect. Current artificial late-stage and ending snapshots retain resumable routes. No further arbitrary RAM guessing or repeated early-flight loop is justified before those contracts are inspected.

Toolkit: [toolkit-issues.md](toolkit-issues.md): no toolkit issues found, no blockers; no toolkit modifications. Disk checked before pass and at handoff; evidence retained, no cleanup/deletion/publication.

## Round 001 commentary — active
Input checkpoint round-001-disassembly-final; frozen commentary baseline prepared from maps/current.json and analysis/state.json. Preparation rebuild byte-identical. Scope: all 511 physical inventory regions, including 11090 classified code bytes and all data/image/unknown boundaries. Current frontier: initial full-source read; no readability review credit yet. Output checkpoint pending; no lead IDs resolved yet. Final acceptance audit deliberately deferred.

Round 001 commentary edit batch: complete source read and 130 mechanical editorial units persisted in analysis/round-001/commentary-work/edit.py and units.json; 711 code/control labels plus 354 RAM/hardware/table aliases applied throughout commentary/source/game.asm. All 511 scaffold entries now have source locations and review summaries. First edited-source verification passed byte-identical (source SHA256 fd8c4dab21b8ee05559b73b7aa812cbe9edf61555d91f7b1bdbfeea5c6e96b02). Current frontier: targeted semantic consistency check on mixed entrypoints, aliases and claims before phase handoff; editorial acceptance remains pending.

Round 001 commentary completed. Input round-001-disassembly-final; output round-001-commentary-final, recorded in analysis/round-001-commentary-final.json. ALL current classified source readability completed: 379 code islands / 11090 code bytes, 13 data regions, 1 CHR region; all 118 unknown spans explained honestly. Full-ROM 511-entry scaffold reviewed with 130 mechanical units; 716 code/control labels, 397 aliases plus constants. Targeted checks corrected mixed entrypoint boundaries and confirmed vectors CC2C/CBC3/CC39, actor fixed-point/carry conventions, software-stack versus CPU-stack distinction and music field offsets. No naming/comment gaps remain for this baseline. Structural map/state unchanged; classification remains 87.14%, 21076 unknown.

Fresh final source verification byte-identical, original ROM SHA256 unchanged; source SHA256 1f453816d02cafd9bb645e0872c6c7a1ef77d88e940ee259437b3a3ed09acf26. Reading copy refreshed after final verify. Immutable round source/review/report copies: analysis/round-001/commentary-final/. Lead outcomes: R001/R002/R003 refined with mechanical source contracts; C001 added for F17F request/code overlap, unresolved and no reclassification. Disassembly D001-D004 credit preserved. Final editorial audit reserved for terminal acceptance. No toolkit issues found; no toolkit edits/archive/deletion/publication. Compact handoff: analysis/handoffs/round-001-commentary.md.

## Round 001 redirect active — r001-contracts
Input round-001-commentary-final (round-001-disassembly-final structural baseline). Attempt analysis/redirect/r001-contracts/. Exact current residual21076 bytes /118 spans; source readability complete. Methods: source-backed static service/return continuation inspection, exact complement dispositions, bounded consumer/record analysis and sound overlap identity review. First frontier persisted in frontier.md. No classification proposals accepted or map changes; connected R001/R002 and independent R003/C001 still active.

Redirect r001-contracts method boundary: service and base side-view visits confirmed artificially; safe landing yields AE/CD and CB counter increments respectively.455 actor script records parse exactly F611..FB1F with68 non-null script pointers and all script branches aligned;36 machine-call targets support coherent follow-up. C001 request read identity resolved for selector13. Generic data inspection localized mixed code tails (AD1A, AEF2, EB56, FC68), preserving ambiguity instead of labeling entire gaps. Frontier and primary artifacts under attempt directory; no map changes.

## Round 001 redirect completed — ready combined batch
Input round-001-commentary-final; structural checkpoint round-001-disassembly-final remains unchanged. Attempt analysis/redirect/r001-contracts/. Authoritative proposal-revised.json proposes11999 Code +9060 Data =21059 currently unknown bytes, overlap0; no new accepted bytes. Exact current complement118spans/21076bytes refreshed in unknown-ledger-reviewed.json from internal state. Hypothetical residual17 bytes only if all proposals accepted; actual classification stays142780/163856 (87.14%).

Methods/results: source-supported connected service/scene/mission-return control contracts; artificial rescue/base landing success with counter evidence; RAM-only barrier return toactualCE12; mission19->20 cleanending throughsupportedreturn. Seven drawing callbacks share879Breturn andindependentendpoints.455scriptrecords parse exactly,68nonnullscriptpointers/allbranches align, all36machinecalltargets/25handlers followed. Independent sprite/screen/numeric/map/text/music data formats reviewed; genericData where semantics unresolved. Residual2436 ordinary tails inspected andpackaged, mixeddata/code boundaries split. Final reasoning review corrected a missing-caller overreach: resetveneer, DB95wrappers and19byteFC6Acopyloop have independent role/stack/hardwareconstraints; added29bytes. Earlier proposals preserved. Remaining17byte boundaries andacceptedFC80 conflict localized in remaining-boundaries.md; nonterminal batch, noexhaustionclaim.

Ready leads R001/R002/R003/C001 updated with preserved histories andrecipes; scenarios andgameplay updated. Fullcurrent-sourcecommentary wascompleteforunchangedbaseline, but newclassifications willcreate readabilitywork; finaleditorialacceptance remains deferred. Integrity.json confirms unchangedROM, state/map andsource hashes; previousbyteidenticalverification unchanged, freshverification requiredafterdisassemblyacceptance. No toolkitissues found, no toolkitmodifications/archive/deletion/publication. Disk~9.1GiBfree, attempt~66MiB, scratch0; no liveexperiments. Compacthandoff analysis/handoffs/round-001-redirect.md.

## Round 002 disassembly active
Input round-001-commentary-final / structural round-001-disassembly-final. Independent connected-batch review in analysis/round-002/disassembly/frontier.md. Existing classifications and commentary preserved. No new annotations yet.

## Round 002 disassembly completed — incomplete ROM

ROM classification: 99.99% (163839 / 163856 bytes classified; 17 bytes unknown).

Input round-001-commentary-final; structural baseline round-001-disassembly-final. Output `round-002-disassembly-final`: [verification](round-002-disassembly-final-verification.json), [portable map](../maps/round-002-disassembly-final.json), generated source `generated/round-002-disassembly-final/game.asm`. map.validate succeeded, state saved/exported, independent fresh rebuild reports byte_identical=true. Internal state and maps/current.json match; unchanged input SHA256 418ad661…f7b. 393 original annotations preserved exactly, plus 138 Static annotations: 11999 Code + 9060 Data. New total 23089 Code, 9678 Data, 131072 Image. No whole-island Observed/natural-reachability upgrade.

| Physical bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG 0 | 8049 | 143 | 8192 | 0 |
| PRG 1 | 5830 | 2361 | 8191 | 1 |
| PRG 2 | 4944 | 3238 | 8182 | 10 |
| PRG 3 | 4266 | 3920 | 8186 | 6 |
| CHR 0 | 0 | 8192 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 8192 | 0 |
| CHR 4 | 0 | 8192 | 8192 | 0 |
| CHR 5 | 0 | 8192 | 8192 | 0 |
| CHR 6 | 0 | 8192 | 8192 | 0 |
| CHR 7 | 0 | 8192 | 8192 | 0 |
| CHR 8 | 0 | 8192 | 8192 | 0 |
| CHR 9 | 0 | 8192 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 8192 | 0 |

Evidence: [independent review](round-002/disassembly/review.md), fresh configured-binary decodes of 5482 instructions with exact interval endpoints and no direct controls into non-Code or mapper stores, [boundary checks](round-002/disassembly/boundary-check.json), [exact counts](round-002/disassembly/classification.json), [unknown complement](round-002/disassembly/unknown-ranges.json). Both structural proof scripts rerun locally with identical JSON results; 455 script records, 68 non-null pointers / all branches aligned; callback,screen,sprite,sound consumer/extent proofs verified. Classification accepted supported connected service/script/callback graph plus independent scene/score/data avenues.

Lead outcomes R001/R002/R003/C001 verified structurally; append-only attempt histories retained. Saved natural cockpit and artificial service/base/late-stage/RAM-only mission-return / clean ending scenarios reused, no repeated gameplay needed for unchanged evidence. No full natural progression or hardware fidelity claim.

Residual 17 bytes / 10 spans: AD7D, C0EA..C0ED, CEB2..CEB5, DAFE, DB21, DB2E, DB44, FC68..FC6A, FC7D..FC80, FCA2. Half-open intervals. Existing Data region134 FC80..FCA2 explicitly inspected and preserved: possible LDX operand conflicts with zero block, insufficient supported continuation beyond the independent FC6A fill-loop prefix. Extra returns/self-jumps/FA60/$20 lack discriminating code-versus-data roles, not rejected merely for absent callers. Pass easy gains handled; no terminal exhaustion decision. Further localized review belongs after current-source commentary.

Readability: previous commentary/source/game.asm SHA256 1f453816…cf26 unchanged; immutable previous commentary copies preserved. New generated source needs full current classified-source commentary/merge, with prior labels/comments retained. Editorial acceptance is deferred.

Toolkit: [toolkit-issues.md](toolkit-issues.md): no toolkit issues found; no toolkit changes. Disk approximately 9.0 GiB free, round2 evidence 3.2 MiB, reusable redirect scratch 0. No deletion/archive/publication; no live server remains. Handoff: [round-002-disassembly](handoffs/round-002-disassembly.md).

## Round 002 commentary — in progress
Input: round-002-disassembly-final, 531 classified map regions. Preserved entire prior commentary at analysis/round-002/commentary-input before merging. Carry unchanged classified physical blocks and valid review credit; replace only previously unknown intervals from the current generated source, preserve previous symbol prefix and resolve new operands through existing aliases. Fresh round-2 baseline is staged separately; no blind prepare/regeneration over maintained source. New 11999 Code / 9060 Data bytes require complete review; old summaries whose unknown-gap dependencies changed require revision. Frontier and reproducible edits: analysis/round-002/commentary-work/. Final editorial acceptance remains pending.

Round 002 commentary completed current-source readability. Input round-002-disassembly-final; output round-002-commentary-final, metadata analysis/round-002-commentary-final.json. Entire previous phase preserved in analysis/round-002/commentary-input; 393 physical classified blocks/reviews and all prior labels carried, affected prose revised. New 138 regions fully reviewed (11999 Code +9060 Data), no deferred ranges. All 474 code regions /23089 bytes,56 data regions /9678 bytes,1 image region /131072 bytes and10 unknown boundaries /17 bytes covered.541-entry scaffold reviewed;249 new functional entries,1204 routine/control labels,628 unit records with370 distinct names. Current scaffold/map/source consistency checks saved in analysis/round-002/commentary-work/consistency.json. Final editorial acceptance is pending, no audit run.

Resolved source dependencies include rescue/base counters/objectives and mission-return/retry, all callback bodies, score/name UI, departure/service zero-page overlays, separate dispatch tables, signed motion, bitmap and segment collision with explicit one-return stack unwind, and data/sound/music formats. Prior generic physical slices within pointer/frame containers now have appropriate consumer-backed comments without changing classification. Corrected F1ED as audio timer-record continuation and AD1A as distinct single-bit/reentry LFSR mechanics. R001/R002/R003/C001 outcomes retained, C001 biased base explanation resolved for supported selectors; AD1A intent and RAM04F1 upload origin remain semantic limitations, not unreadable source.

Fresh final verify byte_identical=true, source SHA25659d7c4719acfa1905c4ac26898d5f24a211b5cbb4478019f9cf57b50088dfef4; reading copy refreshed. Original ROM and structural map/state unchanged. Classification remains99.99%(163839/163856;17unknown). Final snapshot analysis/round-002/commentary-final; handoff analysis/handoffs/round-002-commentary.md. No toolkit issue/change, runtime replay, cleanup/deletion/archive/publication; disk9.1GiB free. Redirect should review the exact residual boundaries and existing rejection evidence under evidence-policy.md; no terminal exhaustion acceptance claimed.

## Round002 redirect — r002-boundaries ready
Read input round-002-commentary-final and exact residual primary evidence. New localized/combined constraints produce3ready leadsR004/R005/R006: FC7D completeLDA2Codebytes excludingFC7F/FC80 overlap; two exactself-originJMP sinks6Codebytes; sparse-zero FCA2genericData1byte. Proposed9, accepted0; actual17unknown unchanged, hypothetical8unknown. All10residual spans plus source uncertainties/progression milestones assessed; remaining competing interpretations recorded. Evidence analysis/redirect/r002-boundaries/, handoff analysis/handoffs/round-002-redirect.md. No terminal exhaustion/finalaudit, map/source/ROMunchanged; current-source readability remains complete. Configured binary used with explicitbankcontext; no toolkit issues found, no new runtime replay or toolkit changes.

## Round 003 disassembly active
Input round-002-commentary-final / structural round-002-disassembly-final. Independent localized review recorded in analysis/round-003/disassembly/frontier.md and input.json before mutations. Prior classifications and commentary preserved; R004/R005/R006 under review, accepted0.

## Round 003 disassembly completed — incomplete ROM

ROM classification: 100.00% (163848 / 163856 bytes classified; 8 bytes unknown).

Rounded 100.00% does not establish completion: eight physical bytes remain unknown. Input round-002-commentary-final / structural round-002-disassembly-final; output round-003-disassembly-final. Accepted R004/R005/R006: 8 Static Code +1 generic Static Data bytes, four new annotations; all531 prior region objects (including labels/comments/evidence/views) preserved exactly. No observed/natural-reachability upgrade.

| Physical bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG 0 | 8049 | 143 | 8192 | 0 |
| PRG 1 | 5830 | 2361 | 8191 | 1 |
| PRG 2 | 4950 | 3238 | 8188 | 4 |
| PRG 3 | 4268 | 3921 | 8189 | 3 |
| CHR 0 | 0 | 8192 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 8192 | 0 |
| CHR 4 | 0 | 8192 | 8192 | 0 |
| CHR 5 | 0 | 8192 | 8192 | 0 |
| CHR 6 | 0 | 8192 | 8192 | 0 |
| CHR 7 | 0 | 8192 | 8192 | 0 |
| CHR 8 | 0 | 8192 | 8192 | 0 |
| CHR 9 | 0 | 8192 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 8192 | 0 |

Independent evidence: [review](round-003/disassembly/review.md), [fresh configured-binary decodes](round-003/disassembly/fresh-contextual-decodes.json), [boundary checks](round-003/disassembly/boundary-check.json), [exact counts](round-003/disassembly/classification.json), [unknown complement](round-003/disassembly/unknown-ranges.json). Finite30x32loop supports complete FC7D LDA06 only; homologous LDA47 differs. Paired self-origin JMPs at C0EA/CEB2 are exact bounded closed sinks. FCA2 sparse-zero byte is generic non-code without a music/header label. Connected continuations checked; FC7F remains unknown and FC80 Data unchanged.

Residual8bytes/7spans: AD7D; DAFE; DB21; DB2E; DB44; FC68..FC6A; FC7F (CPU half-open). Competing residue/code roles remain localized in review; no terminal exhaustion claim. Existing broad-flight, rescue/base, actual caller return and clean artificial-ending evidence reused, no unchanged gameplay replay.

map.validate succeeded; state saved/exported. Fresh [verification](round-003-disassembly-final-verification.json) reports byte_identical=true; state=current map=checkpoint map, input SHA256 unchanged. Prior maintained commentary/source/game.asm SHA25659d7c4719acfa1905c4ac26898d5f24a211b5cbb4478019f9cf57b50088dfef4 unchanged. New generated source requires current-source commentary merge and revised affected boundary prose; final editorial acceptance pending.

[Toolkit issues](toolkit-issues.md): no toolkit issues found; no toolkit changes. Disk approximately9GiBfree; evidence bounded/local. No deletion/archive/publication, no live subprocess remains. Handoff [round-003-disassembly](handoffs/round-003-disassembly.md).


Round 003 current-source readability is complete (round-003-disassembly-final to round-003-commentary-final). Preserved the entire incoming phase in analysis/round-003/commentary-input and staged the new generated baseline in analysis/round-003/commentary-work/before without overwriting before/ or regenerating source/. All 531 previous classified objects and all 1207 prior labels retain valid review credit; reviewed four new objects (8 Code bytes, 1 Data byte) and affected boundary notes. The full current 542-entry scaffold and 631 unit records are reviewed; no current-source gaps or deferred ranges remain. Final editorial acceptance is pending; no audit run.

Source explains the two closed self-loops without an ingress claim, the finite fill-loop's $06 load (distinct from the homologous $47 load), lone $a2 uncertainty and sparse $20 generic-data boundary. Replaced obsolete three-byte-prefix and self-jump/data ambiguity notes. Seven unknown spans total eight bytes. Classification remains incomplete at 163848/163856 despite rounded 100.00%. No full returning helper or natural reachability inferred. Remaining meaning questions and residual structural frontier are in commentary/uncertainties.md; primary structural evidence is analysis/round-003/disassembly/review.md. Reused existing consumer inspection; no runtime experiments or new structural annotation.

Fresh verification passed byte-identically; source SHA256 b91fc5537571ccfe531296bc9d32c50daa0da8370c444a16216475e80e232f3a. Reading copy refreshed. Full inventory/label preservation: analysis/round-003/commentary-work/inventory.json. Checkpoint: analysis/round-003-commentary-final.json. No toolkit issues found; no toolkit edits, archive, cleanup, deletion or publication.

Round003 redirect input round-003-commentary-final: new attempt analysis/redirect/r003-residual-contracts preserves prior research. R007 ready for independent review: complete A2 00 LDX atFC7F..FC81, gaining1unknown andconverting1genericDataoperand. Corrected old annotation-as-boundary premise; accepted finiteprefix/LDAfallthrough andhomologousLDX placement, nofullreturn/reachability claim. Explicit split/reclassification proposal andrivalreview saved. Other supported avenues assessed, broadS001..S007 reused; no arbitraryRAM orunchangedgameplay. Actual163848/163856 classified,8unknown7spans unchanged. Readability complete542scaffold/631units; finaleditorialpending. No toolkit issues found; no toolkit/map/source edits. No terminal exhaustion orfinalaudit. Handoff analysis/handoffs/round-003-redirect.md.

## Round004 disassembly completed — incomplete ROM

ROM classification: 100.00% (163849 / 163856 bytes classified; 7 bytes unknown).

Accepted R007: complete Static LDX #$00 physical[31887,31889), CPU[FC7F,FC81), effective banks[0,1,2,3]. Gain1 unknown byte, convert1 former generic Data operand: Code23099, Data9678, Image131072. Explicitly shortened region134 to Data[31889,31922), retaining its original evidence and preserving original full record in input-state.json and prior checkpoints. All534 other region objects preserved exactly. This is structural inference, no actual-entry/natural-reachability or full-return claim.

| Physical bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG 0 | 8049 | 143 | 8192 | 0 |
| PRG 1 | 5830 | 2361 | 8191 | 1 |
| PRG 2 | 4950 | 3238 | 8188 | 4 |
| PRG 3 | 4270 | 3920 | 8190 | 2 |
| CHR 0 | 0 | 8192 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 8192 | 0 |
| CHR 4 | 0 | 8192 | 8192 | 0 |
| CHR 5 | 0 | 8192 | 8192 | 0 |
| CHR 6 | 0 | 8192 | 8192 | 0 |
| CHR 7 | 0 | 8192 | 8192 | 0 |
| CHR 8 | 0 | 8192 | 8192 | 0 |
| CHR 9 | 0 | 8192 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 8192 | 0 |

Independent [review](round-004/disassembly/review.md) and [fresh decodes](round-004/disassembly/fresh-contextual-decodes.json) establish finite30x32 loop, accepted LDA zero-page $06 fallthrough and homologous complete LDX placement. Prior maximal-zero annotation has no independent FC80 endpoint. FC81 onward remains generic Data; legal BRK decode does not justify expansion. Scoped rival review preserved. Residual7 bytes/6 spans: AD7D; DAFE; DB21; DB2E; DB44; FC68..FC6A (half-open). Exact [classification](round-004/disassembly/classification.json) and [complement](round-004/disassembly/unknown-ranges.json). Rounded100.00% remains incomplete.

Existing broad S001..S007 gameplay/lateflight/rescue/base/actualreturn/clean artificialending evidence reused; no unchanged replay or arbitraryRAM sweep. No further supported actionable easy gain in this pass; no terminal exhaustion claim. Fresh map.validate succeeded, map saved/exported and checkpoint [round-004-disassembly-final](round-004-disassembly-final.json) rebuilt byte-identically: [verification](round-004-disassembly-final-verification.json). OriginalROM SHA256 unchanged. All maintained commentary/source unchanged (SHA256 b91fc5537571ccfe531296bc9d32c50daa0da8370c444a16216475e80e232f3a); current-source commentary must merge the newly classified complete instruction, revise region134 boundary and remove obsolete isolated-A2 wording. Final editorial audit reserved for terminal acceptance.

[Toolkit issues](toolkit-issues.md): no toolkit issues found; no toolkit edits. Approximately9GiBfree, bounded artifacts, no cleanup/archive/publication and no live server. Handoff [round-004-disassembly](handoffs/round-004-disassembly.md).


Round 004 current-source readability is complete (round-004-disassembly-final to round-004-commentary-final). ROM classification: 100.00% (163849 / 163856 bytes classified; 7 bytes unknown); exact percentage 99.99572795625427%, incomplete. Commentary changes no structural classification.

Preserved the entire incoming commentary in analysis/round-004/commentary-input; staged a fresh baseline in analysis/round-004/commentary-work/before without overwriting frozen before/ or generating over source/. Compared the full physical inventory: all 534 unaffected prior region objects and 1210 prior source labels retained; reviewed complete LDX physical31887..31889 and shortened generic Data31889..31922, with dependent sparse-boundary and uncertainty prose corrected. Full 542-entry scaffold / 632 units reviewed, no deferred source ranges. Existing sound full-source review credit retained by physical identity/dependencies. Final editorial acceptance remains pending; no audit run.

FillNametableClearIndex clears X after the zero-page load. Source explains finite-loop fallthrough and homologous complete instruction support, with later use/complete return unresolved. Thirty-three subsequent zeros remain generic data, with no fabricated BRK expansion or format. Seven unknown bytes in six spans remain honestly explained. Unknown complement: AD7D; DAFE; DB21; DB2E; DB44; FC68..FC6A (CPU half-open). Reused prior consumer inspection and broad gameplay evidence; no runtime replay or new structural question.

Fresh verification passed byte-identically; source SHA256 7c52826def1cc7f2c2193b75d13861faaa4ac946c11b0a8cc920a99235e5767b. Reading-after.asm refreshed. Original ROM unchanged; state=current map=current baseline. Inventory and reproducible update: analysis/round-004/commentary-work/. Checkpoint: analysis/round-004-commentary-final.json. No toolkit issues found; no toolkit edits, cleanup/archive/publication. Proceed to redirect; no terminal exhaustion claim.

Round004 redirect input round-004-commentary-final: unique analysis/redirect/r004-terminal-review, no structural mutation. New map-backed all-PRG template comparison plus actual instruction context exposed R008: region451 startsC27F although original explicit source_claim and supported palette contract beginC280. C269 returnsC27E;89E4/8A58 callC280; isolatedC27FRTS has no independent role discrimination. Ready independent correction: retain Codephysical[17040,17176), leave17039unknown, preserving originalrecord/history; noData claim. Expected -1Code,+1unknown, hypothetical163848/1638568unknown7spans; actual163849/1638567unknown6spans unchanged. Primary source_claim mismatch, freshconfigureddecodes and scoped residual/rival/progression review saved in attempt/review.md. Other sevenunknowns remain materiallyambiguous; no unsupportedcallerprerequisite orspeculativeclassification. BroadS001..S007 primaryevidence reviewed, no unchangedruntime replay ornewpracticalshortcut deferred. R008 requiresdisassemblyreview/current-sourcecommentary/redirect; notterminalexhaustion orfinalaudit. Checkpoint analysis/round-004-redirect-final.json; handoff analysis/handoffs/round-004-redirect.md. No toolkit issues found; no toolkit/map/source edits, cleanup/archive/publication. Approximately9GiBfree, boundedlocalartifacts, noactiveserver.


## Round 005 — disassembly complete, ROM incomplete

ROM classification: 100.00% (163848 / 163856 bytes classified; 8 bytes unknown).

R008 independently verified: shortened Code451 from physical[17039,17176) to[17040,17176), viewC280, leaving17039 unknown. Original explicit source_claim and fresh terminal/entry/call decodes justify scope correction; standalone RTS versus residue unresolved, no Data claim or universal caller prerequisite. All other objects preserved exactly; full prior record in [input-state](round-005/disassembly/input-state.json). [Review](round-005/disassembly/review.md), [fresh decodes](round-005/disassembly/fresh-contextual-decodes.json), [classification](round-005/disassembly/classification.json), [exact complement](round-005/disassembly/unknown-ranges.json).

| Physical bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG 0 | 8049 | 143 | 8192 | 0 |
| PRG 1 | 5830 | 2361 | 8191 | 1 |
| PRG 2 | 4949 | 3238 | 8187 | 5 |
| PRG 3 | 4270 | 3920 | 8190 | 2 |
| CHR 0 | 0 | 8192 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 8192 | 0 |
| CHR 4 | 0 | 8192 | 8192 | 0 |
| CHR 5 | 0 | 8192 | 8192 | 0 |
| CHR 6 | 0 | 8192 | 8192 | 0 |
| CHR 7 | 0 | 8192 | 8192 | 0 |
| CHR 8 | 0 | 8192 | 8192 | 0 |
| CHR 9 | 0 | 8192 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 8192 | 0 |

Residual CPU half-open spans: AD7D..AD7E; C27F..C280; DAFE..DAFF; DB21..DB22; DB2E..DB2F; DB44..DB45; FC68..FC6A. Reused sound broad scenariosS001..S007 and localized primary residual review; no unchanged replay or new natural-reachability claim. Single pass easy gains exhausted; next full current-source commentary then redirect, no terminal decision or final editorial audit.

Fresh map.validate/save/export and [checkpoint verification](round-005-disassembly-final-verification.json) passed byte_identical=true; state=current=checkpoint map, ROM SHA unchanged. Maintained commentary/source/game.asm SHA unchanged (checked against input.json); generated checkpoint awaits merge correcting misleading ReturnFromSpawnHelper. No toolkit issues found; [report](toolkit-issues.md), no blockers or toolkit edits. Approximately9GiB free, no scratch generated; no cleanup/archive/publication.


## Round005 commentary — current-source readability complete

ROM classification: 100.00% (163848 / 163856 bytes classified; 8 bytes unknown); incomplete. Current-source readability is complete, separately from structural completion and final editorial acceptance.

Preserved the entire previous commentary in analysis/round-005/commentary-input. Compared the full physical inventory: all 535 unaffected region objects and all 1210 sound labels retained. Removed only the unsupported ReturnFromSpawnHelper name and return claim. Physical17039/CPU C27F is raw unknown, with an adjacent explanation of intentional stub versus residue uncertainty. The timer helper already returns at C27E; the supported palette contract starts C280. Palette behavior, symbols and reviewed units remain intact. No new structural proposal or execution/reachability claim.

All 543 scaffold entries and 631 retained meaningful units reviewed; no deferred classified-source ranges. Valid prior review carried by physical identity and dependencies; changed entry, unknown byte, source references, baseline metadata and uncertainty summary updated. Frozen before/ preserved; fresh generated checkpoint staged under analysis/round-005/commentary-work/before, never over maintained source. Eight unknown bytes in seven spans remain honestly explained: AD7D; C27F; DAFE; DB21; DB2E; DB44; FC68..FC6A (CPU half-open).

Fresh byte-identical verification passed; reading-after.asm refreshed. Source SHA256 e69285337f4cf16711846d0785466161cd9c9893096217b94194f71cf58f03f1. State=current map=current baseline; original ROM unchanged. Checkpoint analysis/round-005-commentary-final.json; verification analysis/round-005-commentary-final-verification.json; inventory analysis/round-005/commentary-work/inventory.json. Reused sound broad gameplay and consumer review; no unchanged runtime replay. Final editorial audit remains pending and reserved for terminal acceptance; no audit run.

No toolkit issues found; [report](../toolkit-issues.md). No toolkit edits, redirect, map changes, cleanup, archive or publication. All source/assets remain local. Ready for redirect; no terminal exhaustion claim.


Round005 redirect: terminal incomplete recommendation pending coordinator inspection. Input round-005-commentary-final and current-source readability complete. Unique attempt analysis/redirect/r005-terminal-consistency verifies all checkpoint hashes and region451 C280 correction/source consistency; exact complement163848/163856 classified,8unknown7spans. Reused sound primary localized residual and coordinator reasoning review on unchanged premises; no repeated scan/runtime replay. Compact rejection review distinguishes six isolated RTS code/residue ambiguities and independent FC68FA/FC6960 subclaims, scopes stack/descriptor/ingress negatives and validates genericData fallback where noncodeclear. No new justified inference or discriminating method, no actionable connected/independent avenue or ready lead; proposed0/accepted0. S001..S007 broad best-effort progression retained, S006 natural completion attempted only; no new useful shortcut. Checkpoint analysis/round-005-redirect-final.json; handoff analysis/handoffs/round-005-redirect.md. Coordinator must inspect rationale before terminal acceptance. No final editorial audit, map/source/ROM/toolkit edits, cleanup/archive/publication. No toolkit issues found. About9GiB free; no scratch generated or live server.

Coordinator terminal review: personally inspected round005 rejection review and exact ledger against r004 primary control/stack/descriptor/homolog artifacts and corrected C27F source-claim mismatch. Accepted sound terminal incomplete, no ready useful next step; no empty cycle. Classification163848/163856,8unknown7spans. Final editorial acceptance worker active; no archive/cleanup/publication. Rationale: analysis/coordinator/residual-review.md.


Final editorial acceptance complete: all543 scaffold entries/631 units reviewed; valid whole-current-source review reused, R007/R008 and all seven unknown notes checked. Corrected AD7D return prose and1174 review source locator prefixes. Supported commentary audit passed with fresh byte-identical rebuild and refreshed reading copy. Configured map/store validation passed. Classification remains INCOMPLETE163848/163856,8unknown7spans. Checkpoint analysis/final-editorial-acceptance.json; editorial report commentary/editorial-audit.md; structural audit analysis/current-audit.json. No toolkit issues found; no toolkit/source-structure/map/input changes, archive, cleanup or publication.

Final coordinator handoff: analysis/handoffs/final-coordinator.md. Orchestration complete at justified terminal INCOMPLETE classification163848/163856,8unknown7spans, no useful next step after personally inspected rejection review and R008 correction. Final editorial audit543regions/631units passed; fresh byte-identical rebuild/map-store validation and independent checkpoint hash/archive editorial-status checks pass. Archive-options complete Markdown accounting; no archive/cleanup/publication/toolkit edits. Final acceptance checkpoint analysis/final-editorial-acceptance.json; requirement audit analysis/coordinator/completion-audit.md.
