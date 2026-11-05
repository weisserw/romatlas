# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 disassembly, intermediate

Baseline verified. Startup to actual Floor1 room took frame1541; intermediate captures are castle/briefing. All exact inputs, trace toggles, timeouts and states logged in pass001-rpc.jsonl. $55 room-index and $67 death flag supported by inspected $98CD-$99A1 main loop. Artificial $55=24,$57=99,$67=1 from natural room1 produced a visibly different room25 layout with lives98 after 240 frames (room25.ppm), proving reload contract. Traced 24-frame Left+A /24 Down exercise that room. Durable room25 snapshot, RAM and screenshot retained. Now testing last room index49 by same supported reload, with 24 frames traced then216 untraced. Ending transition to be inspected before shortcut.

Static initial batch at pass-001-a: 1574 instructions, header, vectors, dedicated CHR image section. 36047 bytes classified; vector targets DC88/C000/DD04 and PRG banks[0,1,2,3]. Zero aggregate drops; detail capacity0 intentionally drops detail but keeps coverage/context aggregates. See pass001-contexts.json, observed-inspection.json, discovery.json. Direct-call target extension had no extra code candidates on initial roots; specific DA54/DA5A/DA60 inline pointer bytes classified (helper pops return and pushes return+2). No generic unknown sweeping or invented semantics applied.

D8DE palette contract verified statically from actual code: saves A, pops JSR return to $11/$12, pushes return+2, consumes two inline pointer bytes into $0F/$10, iterates Y=0..31, transforms using brightness lookup $D959,X and stores staging $0569,Y. Pointer values and consumer count jointly establish 32-byte palette seed slices, independent of natural selection. Connected continuation extension now supports D8DE inline barriers too. References at $8063,$9156,$91C0,$9F60,$C081,$9A39,$9A41 point to $809C,$91DE,$9B33,$9A70,$9A90; each slice is32 valid palette values. Do not extrapolate beyond those bounded slices.

Final bounded reload replay: restore room1-play.snapshot.json atframe1589, release inputs, clear trace to forward timeline, enable zero-detail tracing, patch$55=49,$57=99,$67=1. Replay5x48frames with each max_operations2,000,000/timeout15,000ms; retain combined snapshots and classification from earlier timeline. New replay context/inspection files use -replay suffix. This targets initialization previously traversed with tracing disabled; no arbitrary RAM search.

## Round 1 disassembly complete — incomplete ROM

- Round:1; phase:disassembly-complete. Next phase:full current-source commentary, then redirect.
- ROM classification: 64.55% (42311 / 65552 bytes classified; 23241 bytes unknown).
- Final checkpoint maps/pass-001.json + generated/pass-001, analysis/pass-001-verification.json: valid, byte_identical=true. Internal analysis/state.json and maps/current.json saved. Input ROM SHA256 unchanged.
- 7112 code bytes,35199 data/image;3573 regions;101 unknown ranges (exact complement pass001-counts.json). All42311 classified bytes have evidence. Provenance5474 Observed/36837 Static bytes; execution and readability separate.
- 2820 unique physical ROM fetch starts across final and replay saved contexts;4410 contextual roots. Original timeline3566 roots/replay2091;zero aggregate drops, intentionally omitted detailed events.
- Natural room1, artificial room25/finalroom50, ending reunion and heart scene visited. No terminal credits claim. Reproducible snapshots/scenarios and external guide links retained; stage/clear flags statically supported.
- Call-target/inline continuation, five palette seeds, positive video rectangles and generic uniform islands classified with separate evidence. No raw fallback counted.
- Whole current-source readability NOT complete; commentary must cover7112 code bytes plus data roles. See handoff-disassembly-001.md.
- Leads L001-L004 verified;L005-L008 open research avenues, not failed/rejected. Pass stopped after bounded coherent easy gains; no terminal investigation-exhaustion claim.
- Toolkit: no toolkit issues found;0 bugs/suspected bugs/missing features, no blockers. See toolkit-issues.md. Toolkit unmodified. Server closed cleanly after durable replay snapshot.

|Section|Code|Data/image|Unknown|
|---|---:|---:|---:|
|Header|0|16|0|
|PRG0|843|2322|5027|
|PRG1|2390|67|5735|
|PRG2|3040|20|5132|
|PRG3|839|6|7347|
|CHR0|0|8192|0|
|CHR1|0|8192|0|
|CHR2|0|8192|0|
|CHR3|0|8192|0|

Detailed evidence and localized unresolved hypotheses: [handoff](handoff-disassembly-001.md). No structural lead was rejected merely for missing natural caller/selection. The generic uniform claims do not establish original fill/table semantics; the positive rectangle proofs do not extend to high-bit/mode2 records. Ordinary JSR return effects remain the practical static barrier, with concrete consumer-led L005-L008 research deferred to redirect, not terminally exhausted.

## Round 2 disassembly complete — incomplete ROM

- Phase: disassembly-complete. Exactly one pass; next full current-source commentary then redirect. No terminal exhaustion or editorial audit claim.
- ROM classification: 100.00% (65549 / 65552 bytes classified; 3 bytes unknown).
- Accepted connected L005-L011 union23238bytes:8135code+15103data. Freshly decoded3752proposed instructions; independent stack/PRG/inline contracts, raw format/consumer/boundary review in analysis/disassembly/round002/review.md, decode-review.json and format-review.json. All prior3573regions preserved unchanged.
- Current15247code,50302data/image;7539regions; all65549classified bytes have evidence. Provenance5474Observed/60075Static. No new execution observations: saved2820unique physical fetch starts/4410contextual roots retained as separate execution metric. No unsupported reachability claims.
- Remaining physical8938length2 (A2DA14 3C) and17417length1 (C3F9=60), exact complement in pass002-counts.json. Local alternatives and rejection scope in review.md. Rounded100.00% is not completion.
- Fresh maps/current.json, analysis/state.json and maps/pass-002.json SHA5681e75c92c80fe9258809fb2d01ebf7a5d3261a61debbb9f40c0bfd8cab6bab; map validation successful, generated/pass-002 verified byte_identical=true via analysis/pass-002-verification.json. ROM hash unchanged. Previous commentary/source hashes unchanged; its7112code-byte coverage predates new15247code baseline, so whole current-source readability awaits refresh. No source edits in this phase.
- Gameplay reuse: natural room1, artificial room25/50 and ending scenes documented in scenarios.json; no new concrete structural question justified replay, no new exhaustion/terminal credits assertion.
- An initial annotation request omitted required id and was transactionally rejected before mutation. A prematurely launched checkpoint captured unchanged baseline; own new outputs retained as maps/pass-002-preannotation.json, generated/pass-002-preannotation and analysis/pass-002-preannotation-verification.json. Its report original paths refer to the original creation locations. Corrected client id0 annotation succeeded. This is client error, not toolkit issue.
- No toolkit issues found:0confirmed bugs,0suspected bugs,0missing features; blockers:none. See toolkit-issues.md. No toolkit edit, archive, publication, commit, deletion/cleanup or ROM modification. Coordinator-owned archive-options preserved.

|Section|Code|Data/image|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG0|3484|4708|8192|0|
|PRG1|4621|3569|8190|2|
|PRG2|6179|2012|8191|1|
|PRG3|963|7229|8192|0|
|CHR0|0|8192|8192|0|
|CHR1|0|8192|8192|0|
|CHR2|0|8192|8192|0|
|CHR3|0|8192|8192|0|


## Round 3 disassembly complete — incomplete ROM

ROM classification: 100.00% (65551 / 65552 bytes classified; 1 bytes unknown).

Exactly one pass. Accepted L012 physical8938 length2 as Static Data `ending_embrace_pose_durations` after independent93-instruction fresh decode, phase-domain/consumer/boundary and writer/call/NMI/bank review. Review: analysis/disassembly/round003/review.md and decode-review.json. Phase1/2 values20/60 reload countdown; phase3 reads preexisting packet then exits. Prior no-reference premise corrected byA1B5 indexed read, not opcode shape. All7539 prior regions unchanged;7540 current regions,15247code/50304data-image bytes, all65551classified bytes evidence-backed. Provenance5474Observed/60077Static. No new runtime observation; saved2820unique physical fetch starts/4410contextual roots remain separate execution metrics.

Exact residual physical17417 length1 CPUC3F9=60, PRG2 under[0,1,2,3]. L013 blocked: literal versus isolated/redundantRTS; no independent target/consumer/boundary convention. Primary localized rejection review retained in review.md and redirect/round002-a/findings.md. No code/data claim follows from tail-JMP/no-reference negatives. No unsupported connected proposal withheld. Saved room1/25/50 and ending progression evidence reused; no new exhaustion or terminal credits claim.

Saved/exported analysis/state.json/maps/current.json and fresh maps/pass-003.json SHA076e6f678902d7c525009d6a75b0f145bc842951bee42f0751f9b3f26fd2ef33. Validation successful; generated/pass-003 verification byte_identical=true (analysis/pass-003-verification.json). Exact counts/complement/banks/integrity in analysis/pass003-counts.json. Input ROM, configured binary, maintained commentary/source and archive-options unchanged. Data execution views are code-only: client annotation carrying a view was rejected before mutation; corrected annotation omits views while evidence documents bank context. An inadvertently launched unchanged checkpoint retained as maps/pass-003-preannotation.json/generated/pass-003-preannotation/analysis/pass-003-preannotation-verification.json (report creation paths historical). Client errors, no toolkit defect.

Next full current-source commentary refresh, preserving reviewed baseline/history, then redirect. Previous pass-002 readability remains valid for its scope; new2data bytes and changed unknown frontier require refresh. Final editorial audit pending. No audit/archive/publication/commit/cleanup/toolkit edit. No toolkit issues found:0confirmed bugs/0suspected/0missing features; blockers:none. Persistent report analysis/toolkit-issues.md.

|Section|Code|Data/image|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG0|3484|4708|8192|0|
|PRG1|4621|3571|8192|0|
|PRG2|6179|2012|8191|1|
|PRG3|963|7229|8192|0|
|CHR0|0|8192|8192|0|
|CHR1|0|8192|8192|0|
|CHR2|0|8192|8192|0|
|CHR3|0|8192|8192|0|

