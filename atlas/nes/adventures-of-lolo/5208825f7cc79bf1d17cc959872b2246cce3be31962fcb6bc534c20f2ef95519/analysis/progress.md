# Disassembly progress

- Round: 1
- Phase: disassembly (active)
- Baseline: maps/baseline.json, generated/baseline; analysis/baseline-verification.json confirms byte-identical reconstruction.
- Active server: Python REPL session 74169, configured release binary via run.py; API transcript analysis/pass001-rpc.jsonl.
- No annotations committed yet. Initial traced startup has 816 distinct instruction starts at effective banks [0,1,2,3]; analysis/pass001-contexts.json captures roots.
- Runtime: intro completed, Start menu selected, castle-entry transition reached at frame 474. Advancing untraced 600 frames from frame 519 toward room entry. Initial file names level1 are transition captures, not proven room visits.
- Identity: likely Adventures of Lolo US (HAL 1988/1989 menu and blue protagonist); guide confirmation in progress. Input ROM unchanged.
- Resume frontier: inspect game1.ppm, collect room inputs and state snapshots; classify accepted observed instruction spans plus inspected static discoveries. Discover requires max_instructions, not budget. Do not rely on stale session snapshots; durable level1.snapshot.json captures castle transition.
- No toolkit issues established; initial 180-frame traced step timed out after 80 frames, retained valid partial state and aggregates. Use bounded shorter traced steps and disable tracing for long delays.

## Intermediate verified batch

- Checkpoint pass-001-a verified byte-identical; classification 36,047/65,552 (54.99%), 29,505 unknown.
- Header, validated NMI/reset/IRQ vectors, dedicated 32KiB CHR image backing and 1,574 inspected instruction regions classified. Reachability separate: accepted fetch roots only Observed; connected conservative control-flow discoveries Static.
- Floor1 actual room reached frame1541; durable room-ready.snapshot.json, room-ready.ppm/RAM. Traced Right+A24, Down24 frames from there (natural).
- Static $98E3 loads room index $55 into loader $B44A; $999F increments index on clear. Death flag $67 routes $998F->$B1F6/$9A2A then reload at $98CD. Testing supported reload patch $55=24,$57=99,$67=1 from natural frame1589, 240-frame untraced delay.
- Inline data: helpers $DA54/$DA5A/$DA60 pull JSR return address, consume exactly 2 inline pointer bytes, advance stacked return by2. Preserve barriers on further static expansion.

## Late-game checkpoint frontier

- Artificial room25/room50 visits confirmed with distinct rendered layouts and bounded movement/shoot inputs. Ending clear gate $61=1,$6F=0 at room49 accepted, branch at $99E4 compares incremented index50 and jumps ending $9F74. Captures show corridor reunion then close-up Lolo/Lala heart scene. After another600frames castle view appears; no claim this is a terminal credits screen.
- Additional static contract: $D8DE pops return address, consumes inline pointer+1/+2, pushes adjusted return+2, reads32 pointed bytes through ($0F),Y into palette staging $0569,Y with brightness lookup $D959,X. Extending this independently supported inline barrier/continuation in final batch; do not confuse inline pointers with opcodes.
- Generic fallback accepted1027 bytes in maximal uniform runs>=32 after neighbor/known-code inspection (pass001-uniform-inspection.json). This claims generic noncode only, not original fill/table boundaries.
- Active final frontier: REPL74169 running final observed merge2 and connected static extend3; next validate, save final checkpoint/counts, handoff. All previous inspection artifacts retained under unique suffixes.

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

## Round 1 commentary active

Frozen baseline prepared from verified pass-001; current scope all 7112 classified code bytes and 35199 data/image bytes. Exact pending frontier: all code/data scaffold entries in commentary/review.json, plus unknown-gap boundary notes. No readability review credit yet; phase remains commentary-active.

## Round 1 commentary complete — current classified source

- Phase: commentary-complete; next coordinator phase may be redirect. Full current-source readability covered: 7112 code bytes / 3478 instruction regions, 35199 data/image bytes, 101 honestly annotated unknown gaps. Classification remains 64.55% (42311/65552); no map/state edits.
- 230 functional contracts, descriptive branches and RAM/hardware/consumer-table aliases added to maintained commentary/source. review.json scaffold preserved and reviewed; editorial_audit deliberately pending. Functional inventory: commentary/functional-units.json.
- Corrected semantic details: room RLE has an extra common-exit emit; rectangle column path is selected by width<height; CHR backing also supplies compressed room records; legacy wave-register writes do not establish FDS hardware support.
- Fresh verification byte_identical=true; reading-after.asm refreshed. Exact source hashes in commentary/verification.json. Remaining current-source naming/comment frontier: none. Unknown semantic/structural questions: commentary/uncertainties.md and handoff-commentary-001.md.
- ROM, prior evidence, generated/pass-001, maps and state preserved. No toolkit edits, audit, archive or publication.

## Round 1 redirect active

Frozen checkpoint pass-001; 42311 classified,23241 unknown in101 ranges. Research only, no map/source/state mutation. Attempt analysis/redirect/round001-a. Frontier: per-callee ordinary return contracts (L005), room record consumers/bounds (L006), audio control/data graph (L007), remaining transfer modes (L008), then independent source leads.

Redirect checkpoint round001-a:7911 candidate code bytes from stack/bank-aware summarized returns, no annotations. Room50/50 records validate exact15x16 decode and exact lengths; metadata/tile table proposals in findings.md. Queued-column34byte body inspected;21 actor dispatch handlers and continuations under review. Active frontier audio tables/streams, mode2 inline records, actor producer domains, residual dispositions.

## Round 1 redirect complete — combined ready batch

Attempt analysis/redirect/round001-a: L005-L011 ready;8135code+15103data=23238proposed bytes,0accepted in redirect,0overlap conflicts. Current classification unchanged42311/65552;23241unknown/101ranges. Exact complement/dispositions and frozen hashes checked. Proposed residual3bytes: A2DA2ambiguous bytes and C3F9isolatedRTS. All initial and independent actor/sprite/table opportunities followed; methods/results/actual defer reasons in findings.md. Next independent disassembly review of coherent combined batch; full refreshed commentary and redirect afterward. No terminal whole-ROM acceptance/exhaustion, no final audit/archive. No toolkit issues found; ROM/map/state/source/configured binary untouched.

## Round 2 disassembly active

Frozen input pass-001 SHA9273216e51ffe10c18f028442698efc6393c8a105ab52f93843f555658105950; independently reviewing L005-L011 combined proposals against ROM bytes, stack/bank contracts and consumer bounds. No classifications changed yet. Durable frontier analysis/disassembly/round002/frontier.json. Prior commentary/source preserved; gameplay milestones reuse scenarios.json.

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


## Round 2 commentary active

Prior complete commentary preserved in analysis/round-2/previous-commentary before refreshed preparation. Current scope 15247 code / 50302 data-image / 3 unknown. Frontier: prepare fresh baseline; carry valid physical-range commentary then review all additions and affected dependencies. No completion claimed.

Round 2 commentary checkpoint: entire prior commentary preserved; fresh baseline prepared and carried 3573 stable classified records. 376 functional contracts now in maintained source (146 additions); coherent edited batches verify byte-identical. Remaining frontier: data-format notes, descriptive branch labels, context-sensitive RAM aliases and affected prior prose sweep. A1B5 indexed ending timer read is a concrete corrected residual premise reserved for next redirect; no map/state change.

## Round 2 commentary complete — current source

All15247code/50302data-image bytes readable,7230instruction regions;3unknown remain honest. Full prior commentary preserved in round-2/previous-commentary before refreshing pass-002 baseline. 376code contracts/blocks(146added),299new descriptive control labels;1004total compact code/data units and7541unchanged scaffold records reviewed with refreshed source locations. Typed audio/sprite/table source exposes complete accepted structures. Corrected stale unknown-tail prose/shared RAM aliases and concrete audio command/table interpretations. No readability frontier remains; final editorial_audit pending, no audit/archive acceptance.

Fresh commentary/verification.json byte_identical=true; reading-after.asm refreshed SHA2d078120041c94b15534db19aa72be3d2fb2d9a7c53bf0aeb7d74579beb961fd. ROM/map/state unchanged. Concrete corrected residual premise: A1B5 LDA A2D9,X with phase increment atA1AF selectsA2DA/A2DB during ending pose timer reload; next separate redirect must examine exact domain/consumer role, see commentary/uncertainties.md and handoff-commentary-002.md. This is not new classification or observed reachability. No toolkit issues found; no toolkit edits or blockers.

## Round 2 redirect active

Attempt analysis/redirect/round002-a; frozen pass-002,65549 classified/3unknown. Research only. Frontier: corrected A1B5 indexed timer consumer, all phase/timer writers and connected ending control; independently inspect C3F9 competing RTS/literal interpretation and concrete source opportunities. No map/state/source changes.

## Round 2 redirect complete — corrected timer-data batch ready

Attempt round002-a assessed connected ending state/consumer/call effects, independent C3F9 primary rejection and other concrete commentary opportunities. L012 ready proposes exactly physical8938length2 as Static countdown data; corrected A1B5 indexed access disproves prior no-reference premise. L013 blocked preserves lone C3F9 literal/redundantRTS ambiguity after direct/LE/indexed/dispatch review. Exact current complement3bytes unchanged;2proposed/0accepted,1residual if accepted. Durable findings, raw listing, read-only recipe, writer audit, dispositions and hashes in analysis/redirect/round002-a; handoff-redirect-002.md for independent disassembly. Current-source commentary complete; final audit pending. No ROM/map/state/source/binary edits, archive, publication, commits or cleanup. No toolkit issues found.

## Round 3 disassembly active

Frozen pass-002 SHA5681e75c92c80fe9258809fb2d01ebf7a5d3261a61debbb9f40c0bfd8cab6bab. Independent corrected L012 timer-data review in analysis/disassembly/round003; exact two-byte boundary, phase/writer/NMI constraints and L013 rejection scope under review. Saved gameplay reused; maintained commentary untouched.

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


## Round 3 commentary active

Prior commentary preserved intact at analysis/round-3/previous-commentary before fresh baseline preparation. Current-source scope 15247 code / 50304 data-image bytes plus one unknown at physical17417. Durable frontier: analysis/round-3/commentary-frontier.md. Reuse reviewed source by physical boundaries/dependencies; new timer data and affected prose under review. Final editorial audit deferred.

## Round 3 commentary complete

All current-source readability complete:15247code/50304data-image plus explained1unknown. Entire previous commentary preserved before fresh baseline;7540unchanged scaffold entries and typed includes carried after exact physical/generation/asset checks, new20/60-frame countdown data explained. Indexed-base/phase3 packet and C3F9 prose corrected consistently. All7541records/1005units reviewed;fresh verify byte_identical=true and reading refreshed. No remaining readability gaps; sole physical17417 C3F9 unknown remains for redirect/coordinator assessment. Final editorial audit pending, none run. Handoff: analysis/handoff-commentary-003.md; detailed report commentary/progress.md. No toolkit issues found; no map/ROM/toolkit edit, archive/publication/deletion/cleanup/commit.

## Round 3 redirect active

Current-source commentary remains complete. Research-only attempt analysis/redirect/round003-a at pass-003/current/state SHA076e6f678902d7c525009d6a75b0f145bc842951bee42f0751f9b3f26fd2ef33. Fresh exact complement65551classified/1unknown C3F9. Primary rejection and localized emitted-return convention review saved; no annotation proposed. Independently exploring unvisited special room/floor-clear transition using initialized room5 shortcut, bounded traced frames and exact patches in progression.py/rpc.jsonl; outcome pending. Active frontier.md preserves contract and stop condition; no server restart or map/state/source/toolkit edits.

## Round 3 redirect complete — INCOMPLETE, coordinator review required

Research-only round003-a retains65551classified/1unknown,0proposed/0accepted bytes. Fresh exact complementC3F9physical17417length1; compact primary rejection review includes new narrow JMP-followedRTS convention comparison and corrected timer-premise check. No useful discriminating inference/test or ready structural lead remains; coordinator must inspect rejection-review.md before accepting terminal incomplete. L013 history preserved/appended, L002 enhanced; leads pass added[],updated[L002,L013]. Current-source readability remains complete; final editorial audit pending.

Independently added broad gameplay milestone: initialized room5 special clear requires player6D/6F equal saved goals92/93, not ordinaryY<16. Corrected artificial recipe observed full88pose iterations, progression increment andA7B8floor entry with1526roots/zeroaggregate drops; inspected Floor2 congratulations frame2309 and new room6frame2789. Saved snapshots, captures, exact bounded scripts and client/runtime budget failures preserved; scenarios updated and stale ending Pending inspection phrase removed. Further water/bridge/specific enemy permutations remain honestly deferred for missing particular semantic target/action contract after representative layouts, movement/shoot attempts, special floor scene and ending. No terminal credits claim or exhaustive compatibility claim.

Handoff analysis/handoff-redirect-003.md; detailed evidence analysis/redirect/round003-a. ROM/map/state/source/configured binary unchanged. No toolkit defect identified; no archive/publication/commit/delete/cleanup. Disk9.2GiBfree,scratch180KiB. No live worker or productive interrupted frontier remains.

Round3 coordinator-requested follow-up complete: room6 Right+A16frames/release16 from saved checkpoint, four8-frame bounded calls,1100roots/zeroaggregate drops. No coordinate/shot change; initial-lock/freeze remain1. Recorded bounded interaction attempt and actual limitation, inspected frame2821 capture/snapshot; no success-motion claim. Same handoff/scenarios/results updated; no extra room, classification/source change or new lead.

Known room6 entry delay checked:9A4D..9A6F fade requires8frames before992D/992F initialize lock/freeze;9931..994C clears them on direction input, not time. No supported additional wait-to-unlock method, so preserved32frame attempted-with-lock result without arbitrary flag clearing or further run. Follow-up finished, no live worker remains; current-source readability and checkpoint identities unchanged.

## Final editorial acceptance active

Separate sequential final editor is reviewing changed ending/index/packet explanations, the C3F9 uncertainty boundary, representative routine/control/data explanations and full location integrity while retaining valid prior all-source review. Found and corrected concrete split-record prose: payload fragments repeated a full 32x32 header claim. Also removed two obsolete unclassified-continuation claims and identified metatile table slices explicitly. Line counts and bytes preserved; no classification changes. Active frontier analysis/final-editorial-frontier.md. Final audit/verification and exact hash handoff pending.


## Final editorial acceptance complete

Separate final editorial worker completed actual targeted review and concrete fragment/stale-prose repairs, retained valid all-source review credit and checked20375location records without line shifts. Supported commentary.py audit succeeds with fresh byte-identical rebuild/read refresh:7541regions,11829embedded unit occurrences (1005distinct compact units),editorial_audit=true. Readability/final editorial accepted; classification INCOMPLETE65551/65552,Unknown physical17417length1 C3F9 unchanged. Handoff analysis/handoff-final-editorial.md and scope commentary/editorial-audit.md. Maintained game.asm SHA00039a274255982f5e22a71117f56cdbac2591607b156f5fff45a663514b3c43; reading SHA62bbba5dcaee19301732474db000b3342c4e83ecf74c2ca76ced5236d0ff5441; verification SHAc98dd16fb983e0e6a136a904cc5bb8e02ac88720d3b443a31daa7588ddb0fdb4; audit SHA66cbff33f355320f3d26ffdd72ac60bf89b7530587b104de52e8ac92ddbb731f; map/state SHA076e6f678902d7c525009d6a75b0f145bc842951bee42f0751f9b3f26fd2ef33. No editorial frontier, toolkit issues or ROM/map/state/toolkit changes. Final coordinator inventory/archive-options/requirement audit remain separate.

## Coordinator terminal handoff — workflow complete, ROM INCOMPLETE

All three sequential disassembly/commentary/redirect rounds and separate final editorial acceptance finished. Coordinator inspected compact rejection review AND decisive current source/raw listing/search/convention artifacts under evidence-policy.md; no corrected useful inference/test remains for C3F9. Accepted terminal incomplete rationale, not a blocked unfinished phase or whole-ROM completion. No live productive worker/process remains.

ROM classification: 100.00% (65551 / 65552 bytes classified; 1 byte unknown).

Rounded100.00% is incomplete: physical17417length1, CPUC3F9=$60, PRG2 effective[0,1,2,3]. Final checkpoint pass-003, current/state SHA076e6f678902d7c525009d6a75b0f145bc842951bee42f0751f9b3f26fd2ef33. All15247code/50304data-image bytes readable; final editorial audit passed7541regions/11829embedded unit occurrences/1005distinct compact units, fresh byte-identical verify and refreshed reading SHA62bbba5dcaee19301732474db000b3342c4e83ecf74c2ca76ced5236d0ff5441. Final configured-binary store/map validation both ok. See terminal-handoff.md, completion-audit.md, completion-verification.json and coordinator-rejection-review.md.

Final archive-options explicitly preserves conclusions/handoffs/history and omits only duplicate prior-phase instruction scaffolding; document-selection-verification.json checks actual archive.py contract and all analysis/commentary Markdown. No archive execution/publication/cleanup/workspace deletion/toolkit modification/commit. Input ROM and binary unchanged. No toolkit issues found. Reopen classification only with supported exact entry/read/format/boundary evidence or materially justified new inference; no empty unchanged workflow cycle is required.
