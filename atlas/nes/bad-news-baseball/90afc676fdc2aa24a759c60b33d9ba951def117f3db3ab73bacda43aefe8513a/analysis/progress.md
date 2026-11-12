# Round 1 disassembly checkpoint

Status: incomplete. Exactly one substantive disassembly pass completed. No persistent server remains; replay scripts start the configured release binary and import durable local snapshots.

ROM classification: 61.45% (161110 / 262160 bytes classified; 101050 bytes unknown).

Code 16321; Data 13717; Image 131072. 520 regions all carry evidence; 173 unknown intervals. Observed instruction bytes 12715, unique starts 6199; natural 5144 starts, artificial 5449 starts (overlapping sets). Static code3606 bytes is not execution coverage.

Checkpoint: maps/pass-001-final.json; generated/pass-001-final; analysis/pass-001-final-verification.json confirms byte-identical ca65/ld65 rebuild. map/state exported and validated. ROM SHA256 90afc676fdc2aa24a759c60b33d9ba951def117f3db3ab73bacda43aefe8513a.

| Physical bank | Code | Data | Image | Classified | Unknown |
|---|---:|---:|---:|---:|---:|
| header | 0 | 16 | 0 | 16 | 0 |
| PRG00 | 503 | 0 | 0 | 503 | 7689 |
| PRG01 | 0 | 1508 | 0 | 1508 | 6684 |
| PRG02 | 0 | 0 | 0 | 0 | 8192 |
| PRG03 | 0 | 3023 | 0 | 3023 | 5169 |
| PRG04 | 1940 | 0 | 0 | 1940 | 6252 |
| PRG05 | 0 | 1167 | 0 | 1167 | 7025 |
| PRG06 | 562 | 408 | 0 | 970 | 7222 |
| PRG07 | 0 | 1050 | 0 | 1050 | 7142 |
| PRG08 | 1102 | 0 | 0 | 1102 | 7090 |
| PRG09 | 0 | 252 | 0 | 252 | 7940 |
| PRG10 | 4575 | 0 | 0 | 4575 | 3617 |
| PRG11 | 1367 | 5223 | 0 | 6590 | 1602 |
| PRG12 | 3287 | 18 | 0 | 3305 | 4887 |
| PRG13 | 11 | 1046 | 0 | 1057 | 7135 |
| PRG14 | 2962 | 0 | 0 | 2962 | 5230 |
| PRG15 | 12 | 6 | 0 | 18 | 8174 |
| CHR00 | 0 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 0 | 8192 | 8192 | 0 |

Evidence: round001-counts.json and round001-unknown.json record exact physical complement. Startup/menu/match/mechanics/password runtime fetches retain CPU aliases and effective PRG-slot contexts in *-contexts.json/*-decoded.json. Generic data rationale and exact boundary neighbors: uniform-data.json. Specific tables: PasswordAlphabetGlyphs 0x16ADF..0x16B1F, MatchPhaseDispatchWords 0x18033..0x18045. Static continuations: static-full-new.json/static-final-new.json with bounded discovery reports. Final narrow audit corrected initial withholding of BRK at0x159DD: rooted LDA$AD/CMP#3/BCC-over-BRK supplies a valid exception contract independently of fetch. It and six-byte IRQ reset entry0x1FFE8 are Static; natural trap/IRQ entry unresolved. Earlier proposal/checkpoint preserved. See final_audit.py and round001-checkpoint-notes.md. No raw-export fallback bytes count as classified.

Progression: natural introduction/title/team/lineup/pitcher selection and batting/pitching/pause/batter-substitution visited. Artificial supported half-inning entry reaches results and password; accepted final-game password reaches Chicago vs Toronto. Ninth-inning top display visited. Final-game win shows GAME11 and subsequent engine7 victory parade after1200 untraced empty frames+30 traced frames (ending-wait.ppm.png). Ending visited; credits completion not claimed. Ninth count/outs patch did not establish an actual side transition; retain as attempted. Alternate all-star/spectator/girls modes and sustained fielding remain deferred, not declared unreachable. See scenarios.json and gameplay.md.

No initial ready leads existed. Verified phase leads D001 startup, D002 natural match, D003 half-inning results, D004 password, D005 ending, D006 ninth; exact recipes/outcomes in leads.json. New redirect avenues: engine7 script/ending consumers, code/data in untouched bank2 and paired data banks, all-star/girls state routes, nonuniform pointer/roster/audio structures adjacent to existing code. Further work needs consumer/bank-bound proof rather than longer repeated initial play. This is an easy-gain handoff, not terminal exhaustion or a completion claim.

Toolkit: no toolkit issues found; see analysis/toolkit-issues.md. Initial mailbox process termination and rejected socket creation were local harness/infrastructure events, not reproduced toolkit defects. Bounded direct subprocess/snapshot replay worked. Trace detail capacity0 intentionally drops detail while aggregate_dropped=0 in retained summaries. Disk at handoff ~6.7GiB free; scratch112KiB before final records, evidence~119MiB; cited evidence retained.

Historical checkpoints pass-001 and baseline remain intact. Final map SHA256 ef26a4ca85d00c0d61f4c49b5f912bfc38ad473426182b902d1652e3e0dec96d. Full current-source readability is assigned to subsequent commentary; this phase does not claim editorial completion. Ending parade is confirmed in ending-wait.ppm.png; terminal credits were not exhaustively played.

### Round 001 commentary milestone

Commentary prepared from pass-001-final with frozen map/state and verified before source. Full source inspection completed; all 693 scaffold intervals have local explanations and review locations, including classified code/data/CHR and unknown gaps. Coherent annotation batch rebuild passed byte-identically. Active work: inspect naming/prose consistency, then run fresh verification and refresh reading-after.asm before the commentary handoff. See commentary/progress.md and commentary/scope-notes.json. Whole-ROM classification and final editorial acceptance remain incomplete; redirect is not yet authorized by this milestone.

### Round 001 commentary complete

Full current-source readability gate passed: all 693 physical scaffold entries reviewed (502 Code,17 Data,1 Image,173 Unknown). Source includes routine/control-flow/data explanations, symbolic hardware and understood RAM operands, lookup-base identities and explicit uncertainty for all unknown intervals. Final byte-identical commentary verification and refreshed reading copy completed; source hash ec5bcb5c5d31a2b7d07ea1b255e2f055f5d95dffb4414be908cb5e3a7814a49d. Compact handoff: analysis/handoffs/round-001-commentary.md. Classification remains61.45% and final editorial/archive acceptance remains pending. No toolkit issues found. Coordinator can proceed to redirect; no toolkit edits, input changes, publication, archive or cleanup occurred.

### Round 001 redirect active

Attempt analysis/redirect/round001-a; input map ef26a4ca85d00c0d61f4c49b5f912bfc38ad473426182b902d1652e3e0dec96d. Presentation/audio consumer and continuation contracts first, then alternate-mode/side transition and remaining concrete residual avenues. Classification and commentary remain unchanged.

### Round 001 redirect complete — positive batch

Attempt analysis/redirect/round001-a complete; independent disassembly next. Accepted map unchanged161110classified/101050unknown(173intervals), map ef26a4ca85d00c0d61f4c49b5f912bfc38ad473426182b902d1652e3e0dec96d. Combined structurally supported proposals83061bytes(71416Data,11645Code;3747Observed/7898Static), no overlap conflicts. Conditional residual17989 if all accepted. Exact proposal/unknown-ledger/contract review in attempt; compact handoff analysis/handoffs/round-001-redirect.md. LeadsR001-R003 updated,R004-R007 added.

Side transition, AllStar/Special CPU fielding, alternative roster and complete ending credits-to-introduction observed with recorded ordinary/artificial routes. Current-source commentary remains complete and unchanged; all existing prose/review credit preserved. No toolkit issues found. No live server, toolkit/ROM/map/state/source edits, archive/publication or cleanup. Positive proposal handoff, not terminal exhaustion.

### Round 002 disassembly active

Exactly one substantive pass active. Input checkpoint pass-001-final, current map SHA256 ef26a4ca85d00c0d61f4c49b5f912bfc38ad473426182b902d1652e3e0dec96d. Independent review of analysis/redirect/round001-a/proposal.json and primary consumer/CFG/fetch evidence underway. Existing commentary/source and review credit preserved.

### Round 002 disassembly complete — incomplete classification

Exactly one substantive pass completed. Saved redirect evidence was reviewed independently; unchanged gameplay routes were not replayed.

ROM classification: 93.14% (244184 / 262160 bytes classified; 17976 bytes unknown).

Added 83,074 bytes: 71,432 Data and 11,642 Code (3,747 Observed, 7,895 Static). Totals: 27,963 Code, 85,149 Data and 131,072 Image; 148 unknown intervals; 1,332 evidence-backed map regions. Raw export fallback does not count.

| Physical bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG00 | 1959 | 6052 | 8011 | 181 |
| PRG01 | 6 | 8186 | 8192 | 0 |
| PRG02 | 0 | 8192 | 8192 | 0 |
| PRG03 | 190 | 3705 | 3895 | 4297 |
| PRG04 | 2119 | 5974 | 8093 | 99 |
| PRG05 | 6 | 8186 | 8192 | 0 |
| PRG06 | 566 | 7626 | 8192 | 0 |
| PRG07 | 6 | 8186 | 8192 | 0 |
| PRG08 | 1172 | 6956 | 8128 | 64 |
| PRG09 | 6 | 8186 | 8192 | 0 |
| PRG10 | 5368 | 0 | 5368 | 2824 |
| PRG11 | 2517 | 5281 | 7798 | 394 |
| PRG12 | 6282 | 18 | 6300 | 1892 |
| PRG13 | 3667 | 1088 | 4755 | 3437 |
| PRG14 | 4087 | 0 | 4087 | 4105 |
| PRG15 | 12 | 7497 | 7509 | 683 |
| CHR00..15 (each) | 0 | 0 | 8192 Image | 0 |

Independent review: analysis/disassembly/round002/review.md. checks.json validates 8,529 fresh instruction/context decodes, 125 callee proofs containing 5,561 steps, actual fetch witnesses and saved parsed records. closure-checks.json records 14,515 context nodes, 5,688 new ordinary starts and 14 conditional mirrored starts. R001..R007 are verified with scoped rejections and exact, overlapping lead-range counts in leads.json.

Correction: the proposed Static BMI/BRK at physical 0x16253..0x16256 is false code. BCS fallthrough leaves carry clear; DEC/LDX preserve it; the following BCC always jumps away. Consumer offsets, four LE16 lower/upper pairs and the independent routine at CPU A253 establish Data at 0x16253..0x16263. This replaces three proposed Code bytes and adds thirteen previously unproposed bytes. The original 83,061-byte proposal becomes 83,074 accepted new bytes. BRK at 0x15999 retains its supported terminal exception contract. The whole-envelope FF terminator and alternate 49th screen schemas remain rejected within their stated scope; surrounding supported Data remains accepted.

Fresh checkpoint: maps/pass-002-final.json, generated/pass-002-final and analysis/pass-002-final-verification.json confirm a byte-identical ca65/ld65 rebuild. Map SHA256: 24ef23895954dee3a4262e48055c86620455cd656c94853167e745e879f4c5d1. ROM SHA256 remains 90afc676fdc2aa24a759c60b33d9ba951def117f3db3ab73bacda43aefe8513a. Map validation, save and export passed.

All prior region IDs and metadata are preserved. commentary/source/game.asm SHA256 remains ec5bcb5c5d31a2b7d07ea1b255e2f055f5d95dffb4414be908cb5e3a7814a49d; prior prose and review credit are intact. Full commentary for the expanded current map is next and is not claimed complete. Saved scenarios cover the ninth side switch, AllStar, Special fielding, girls and credits-to-introduction; their ordinary/artificial origins remain distinguished.

Residual frontier: 17,976 bytes in 148 intervals, recorded exactly in unknown.json and bank-counts.json. Remaining 167 call-return and 272 write barriers need concrete pointer bounds, dispatcher domains or return/bank contracts. This is an incomplete positive handoff, not terminal exhaustion or an unused/dead claim. Commentary then redirect can reuse the saved barriers and successful routes.

No toolkit issues found; see analysis/toolkit-issues.md. Two rejected annotation requests were local harness schema mistakes (required id=0 allocation), corrected before save; no toolkit defect or persisted partial map. Disk: 6.5 GiB free; scratch: 208 KiB. No live server, cleanup, toolkit edits, input changes, publication or archive. Handoff: analysis/handoffs/round-002-disassembly.md.

## Round 002 redirect active
Current-source commentary gate complete. Attempt analysis/redirect/round002-a; input pass-002-final, map SHA24ef23895954dee3a4262e48055c86620455cd656c94853167e745e879f4c5d1;244184 classified /17976 unknown. Reusing prior decisive evidence without replay. Initial frontier: filter covered call/write bookkeeping; prove ordinary PRG-preserving writes and inspect C024/C330 inline dispatch plus pointer contracts.

## Round 002 redirect ready handoff

Attempt round002-a complete as a positive combined proposal, not terminal exhaustion. R008/R009 ready:17429 proposed bytes (8074StaticCode,9355StaticData), accepted new0. Input/current classification unchanged244184/262160=93.14%,17976unknown/148intervals, pass-002-final map SHA24ef23895954dee3a4262e48055c86620455cd656c94853167e745e879f4c5d1. Conditional residual if all accepted:547bytes/10intervals. Exact full current unknown ledger and per-slice proposal/deferred reasons saved.

Investigated current roots/callee/write closure, every inline dispatcher and associated handlers, service-JMP arrays, immediate bank contexts, pointer destinations, finite text/audio dispatch, bank-aware record returns, complete numeric/text containers and bounded generic residuals. Fresh3967 contextual instruction checks;113callee/16wrapper/5bank summaries; no proposed known-byte/duplicate overlaps. ROM/map/commentary unchanged; false16253BMI/BRK remains rejected Data. Further field return-stack/domain and tiny-fragment ambiguity localized in round002-a/frontier.md. Existing broad gameplay routes reused, no runtime replay. Next independent disassembly then full current-source commentary preserving prose/review credit. See analysis/handoffs/round-002-redirect.md.


### Round 003 disassembly active

Exactly one substantive pass active from pass-002-final (map SHA256 24ef23895954dee3a4262e48055c86620455cd656c94853167e745e879f4c5d1). Independent primary consumer/CFG/context/bounds review of R008/R009 underway; 17429-byte proposal pending. Current classification unchanged244184/262160;17976 unknown. Preserve current source SHA256862952e794ae020694d09b1e815afe1275602e6dd0035682e63193efb061fc52 and prior review credit. No competing writer or emulator replay.


### Round 003 disassembly complete — incomplete classification

Exactly one substantive pass completed. R008/R009 accepted17429 bytes; bounded independent/connected continuation adds361 bytes. New17790 bytes =8423 Static Code +9367 Static Data. No new Observed execution or natural invocation claim.

ROM classification: 99.93% (261974 / 262160 bytes classified; 186 bytes unknown).

Totals36386 Code/94516 Data/131072 Image;1532 evidence-backed regions. Ten residual intervals, exact complement in analysis/disassembly/round003/unknown.json. All prior IDs/metadata and commentary/source/game.asm SHA256862952e794ae020694d09b1e815afe1275602e6dd0035682e63193efb061fc52 preserved. Full expanded-current-source commentary remains next; prior review credit intact.

| Physical bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| header |0|16|16|0|
| PRG00 |2138|6054|8192|0|
| PRG01 |6|8186|8192|0|
| PRG02 |0|8192|8192|0|
| PRG03 |901|7291|8192|0|
| PRG04 |2218|5974|8192|0|
| PRG05 |6|8186|8192|0|
| PRG06 |566|7626|8192|0|
| PRG07 |6|8186|8192|0|
| PRG08 |1232|6956|8188|4|
| PRG09 |6|8186|8192|0|
| PRG10 |7403|789|8192|0|
| PRG11 |2603|5589|8192|0|
| PRG12 |7759|259|8018|174|
| PRG13 |5939|2247|8186|6|
| PRG14 |5591|2599|8190|2|
| PRG15 |12|8180|8192|0|
| CHR00..15 (each) |0|0|8192 Image|0|


Independent review analysis/disassembly/round003/review.md:8046 fresh contextual decodes,6725 callee proof steps/113 summaries,107 explicit call dependency contracts,43 balanced tails;5 bank-aware CPU/software stack proofs and16 wrappers separately checked. Additional finite RAM routine/lookup and connected proof decodes in extra-proofs.json/connected-proofs.json. Special PLA/PLA/RTS exits remain distinguished from balanced return contracts; corrected16253..16263 arithmetic Data preserved. No assumed universal selector domain or live caller prerequisite.

Map validate/save/export passed. Fresh checkpoint maps/pass-003-final-connected.json and generated/pass-003-final-connected, verification analysis/pass-003-final-connected-verification.json: byte-identical true. Map SHA256118c926b99e13dddf0dfae69066f2084c12a9ca8ce3c502441d741f3e487a14c; input SHA25690afc676fdc2aa24a759c60b33d9ba951def117f3db3ab73bacda43aefe8513a unchanged. Earlier same-pass checkpoints retained; final adds an explicit valid-position input precondition and connected code.

Broad saved gameplay/ending scenarios reused; no unchanged replay. Residual presentation bodies/prologues need localized stack/entry review; tiny scalar/return fragments remain ambiguous. This is a positive incomplete handoff, not terminal exhaustion or editorial completion. Handoff analysis/handoffs/round-003-disassembly.md supplies exact frontier.

No toolkit issues found; see analysis/toolkit-issues.md. Disk6.5GiB free; scratch292KiB. No live server or competing writer at handoff; no toolkit/input/source edits, publication/archive or cleanup.


### Round 004 disassembly active
Exactly one substantive pass from pass-003-final-connected. Independent review of R010/R011 local stack, consumer and sequential-return claims underway; current261974/262160 classified,186 unknown. Preserve source SHA2562a68c97f32366331429ff8ab04ba251a52d8627829d8ed61fcd5f67052a95430 and existing prose/review credit. Reuse saved gameplay, no competing writer.


### Round 004 disassembly complete — incomplete classification

Exactly one substantive pass. R010/R011 verified with independent local control/stack/consumer review; accepted180bytes=171 Static Code+9 Static Data. Connected downstream routines already classified; no further supported unknown neighbor. No new Observed or natural invocation claim.

ROM classification: 100.00% (262154 / 262160 bytes classified; 6 bytes unknown).

Rounding does not establish completion. Exact complement: physical0x10530..0x10534 (4bytes, PRG08, CPU8520 slots[8,9,14,15]);0x19E99..0x19E9B (2bytes, PRG12, CPU9E89 slots[12,13,14,15]). Code/data alternatives remain materially ambiguous; independent review at analysis/disassembly/round004/review.md. No terminal exhaustion claim.

| Physical bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| header |0|16|16|0|
| PRG00 |2138|6054|8192|0|
| PRG01 |6|8186|8192|0|
| PRG02 |0|8192|8192|0|
| PRG03 |901|7291|8192|0|
| PRG04 |2218|5974|8192|0|
| PRG05 |6|8186|8192|0|
| PRG06 |566|7626|8192|0|
| PRG07 |6|8186|8192|0|
| PRG08 |1232|6956|8188|4|
| PRG09 |6|8186|8192|0|
| PRG10 |7403|789|8192|0|
| PRG11 |2603|5589|8192|0|
| PRG12 |7928|262|8190|2|
| PRG13 |5939|2253|8192|0|
| PRG14 |5593|2599|8192|0|
| PRG15 |12|8180|8192|0|
| CHR00..15 each |0|0|8192 Image|0|

Totals36557Code/94525Data/131072Image;1540 evidence-backed regions. Preserve all existing region IDs/metadata and commentary/source/game.asm SHA2562a68c97f32366331429ff8ab04ba251a52d8627829d8ed61fcd5f67052a95430; prior prose/review credit intact. Current expanded-source commentary is next, not claimed complete. Fresh map validate/save/export and pass-004-final checkpoint byte-identical true; maps/current.json SHA25620b933e17d8f3032291cb4ca229fd6da613cee1a1975e18d4d2fff4df73f6dc6. Verification analysis/pass-004-final-verification.json; generated/pass-004-final. ROM hash unchanged.

Fresh explicit stack audit47/60/81/1007 states, no stops. Actual CDA4 removes C042 return then uses retainedA378; savedY survives. Actor0..11 and menu/field inputs remain conditional. D4DB is exact nonstandard sequential transfer, not a balanced wrapper. Direct9A45/A8D9/B5B2 consumers reviewed; sharedB5B2 RTS Code preserved. Broad saved gameplay including credits/return reused without unchanged replay.

No toolkit issues found; see analysis/toolkit-issues.md. Disk6.4GiB free, scratch376KiB; no cleanup. No live server/competing writer, toolkit/input/source edits, archive or publication. Handoff analysis/handoffs/round-004-disassembly.md.


### Round004 redirect complete — positive independent proposal

Input pass-004-final unchanged:262154/262160 classified,6unknown. R012 proposes2 Static Data bytes19E99..19E9B via accepted LDA9E86,X consumer and explicit Work04EB3/4 writers; prior exact-origin rejection corrected. No classification/source changes. Audio10530..10534 remains ambiguous after localized dispatch/period/helper review. Full review and primary artifacts analysis/redirect/round004-a; exact handoff analysis/handoffs/round-004-redirect.md. No toolkit issues found, no server, no ROM/toolkit edits, no cleanup/publication. Independent disassembly next, no terminal exhaustion claim.


### Round 005 disassembly active
Exactly one pass from pass-004-final; independent primary R012 consumer/selector/bank/boundary review active. Current262154/262160 classified,6unknown. Preserve source SHA256976b55a104e228767cb35b3633c9b18c98951bcda749474b1fc5e9790dc65123 and all prior map metadata/prose/review credit; no gameplay replay or competing writer.


### Round005 disassembly complete — incomplete classification

Exactly one pass; R012 independently verified and accepted2 Static Data bytes19E99..19E9B. Connected writer/trajectory/direction/multiplier inspection adds0: all supported continuation already classified. No natural invocation or new execution claimed; execution coverage unchanged. Saved broad progression scenarios reused.

ROM classification: 100.00% (262156 / 262160 bytes classified; 4 bytes unknown).

Totals36557Code/94527Data/131072Image;1541 evidence-backed regions, all prior metadata preserved. Exact residual10530..10534C8C8C860, CPU8520 slots[8,9,14,15], remains genuinely code/data ambiguous; no unchanged artificial entry/replay. Review analysis/disassembly/round005/review.md; counts bank-counts.json, complement unknown.json. All commentary/source prose and prior review credit preserved, source SHA256976b55a104e228767cb35b3633c9b18c98951bcda749474b1fc5e9790dc65123. Expanded current-source commentary is next, not completed here.

| Physical bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| header |0|16|16|0|
| PRG00 |2138|6054|8192|0|
| PRG01 |6|8186|8192|0|
| PRG02 |0|8192|8192|0|
| PRG03 |901|7291|8192|0|
| PRG04 |2218|5974|8192|0|
| PRG05 |6|8186|8192|0|
| PRG06 |566|7626|8192|0|
| PRG07 |6|8186|8192|0|
| PRG08 |1232|6956|8188|4|
| PRG09 |6|8186|8192|0|
| PRG10 |7403|789|8192|0|
| PRG11 |2603|5589|8192|0|
| PRG12 |7928|264|8192|0|
| PRG13 |5939|2253|8192|0|
| PRG14 |5593|2599|8192|0|
| PRG15 |12|8180|8192|0|
| CHR00..15 each |0|0|8192 Image|0|

Map validate/save/export passed. Authoritative checkpoint maps/pass-005-final-reviewed.json; generated/pass-005-final-reviewed; analysis/pass-005-final-reviewed-verification.json byte-identical true. Map SHA256a839d376085c7bc579e260056f5bf4c81fceb3316622389bfcfa63d9f3cb4b02. ROM hash unchanged. Earlier pass-005-final checkpoint preserved: annotation schema rejected unsupported Data context fields before mutation, so that checkpoint reflects input map. Corrected local harness omits those fields; CPU/bank evidence retained in checks/review. This was a local schema error, not a toolkit defect.

No toolkit issues found; analysis/toolkit-issues.md preserved. No live server/competing writer, toolkit/input/source changes, archive, publication or cleanup. Worker stopped writing after handoff analysis/handoffs/round-005-disassembly.md. Next commentary then redirect; no terminal exhaustion claim.

### Round005 redirect complete — incomplete, coordinator inspection required

Input pass-005-final-reviewed and fully commented current source unchanged. Exact262156/262160classified,4unknown(10530..10534,C8C8C860;CPU8520,[8,9,14,15]). Changed unsigned-indexed coverage/period-wrap method returned0 covering references; all1232 current audio Code bytes covered in preserved decode cache. Primary-linked compact rejection review evaluates helperCode versus sparseData, proper subintervals, period prefix and screen-bank alias without universal caller/natural-reachability prerequisites. R013 added blocked; prior history preserved; no ready/proposed bytes and0accepted changes. Broad10-milestone progression/ending checklist reviewed with superseded failures retained; terminal return primary capture and progress inspected. analysis/redirect/round005-a/review.md and analysis/handoffs/round-005-redirect.md provide terminal incomplete candidate for coordinator independent inspection. No toolkit issues found. No live server, toolkit/input/source/map/state changes, cleanup, archive or publication.


## Coordinator terminal handoff — justified incomplete

Round005 sequential workers completed disassembly, full current-source commentary and redirect; coordinator inspected rejection reasoning and decisive primary artifacts, reran the bounded indexed-domain reproduction and independently validated store/map. No corrected useful next inference/test survived. Targeted final editorial acceptance then passed, preserving valid prior credit. This is terminal workflow completion only, not ROM classification completion and not an infrastructure block. R013 remains evidence-dependent/resumable; no worker/server remains active.

ROM classification: 100.00% (262156 / 262160 bytes classified; 4 bytes unknown).

Exact residual10530..10534 C8C8C860, CPU8520 slots[8,9,14,15]; code-versus-data ambiguous. Totals36557Code/94527Data/131072Image;1541 classified regions. Final checkpoint pass-005-final-reviewed map/current state SHAa839d376085c7bc579e260056f5bf4c81fceb3316622389bfcfa63d9f3cb4b02, fresh checkpoint and commented-source rebuilds byte-identical. Final source SHAea8947206254f1a3596a253aa370d67b18e924249794dc33aeff0d1c6a1de01c; all1542 scaffold entries/1449 units reviewed, editorial audit passed, reading copy refreshed and compared. Input hash unchanged. See analysis/current-audit.json and analysis/handoffs/terminal-incomplete.md for exact artifacts and resume boundary.

Archive.py document-selection contract inspected; archive-options.json selects final conclusions, compact handoffs and durable Markdown history, omitting duplicate procedural scaffolding only. analysis/archive-selection-audit.json accounts for all current analysis/commentary Markdown. No archive execution, cleanup/deletion, publication, toolkit or ROM modification. No toolkit issues found. Historical evidence/checkpoints retained.
