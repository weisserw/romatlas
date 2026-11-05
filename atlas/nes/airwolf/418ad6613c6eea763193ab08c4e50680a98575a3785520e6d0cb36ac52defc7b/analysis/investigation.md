# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

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

Round003 redirect r003-residual-contracts: corrected completeFC7F..FC81 LDX proposal R007 across genericzeroData boundary. Prior one-byte/fullreturn rejection didnotresolvecomplete2byteclaim; record134 has noindependentFC80endpoint. Finiteacceptedprefixfallthrough plus homologousLDXplacement supportproposal; exactData split required. Other sevenbytes assessed withlocalcode/residuecompetitors; targetedencoding scan negativeonlyforitsliteralroutes. Actualmap8unknownunchanged; gain1unknown+reclassify1Data proposed, accepted0. Evidence analysis/redirect/r003-residual-contracts/review.md, handoff analysis/handoffs/round-003-redirect.md. Return coherentbatch todisassembly; notterminalexhaustion.

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
