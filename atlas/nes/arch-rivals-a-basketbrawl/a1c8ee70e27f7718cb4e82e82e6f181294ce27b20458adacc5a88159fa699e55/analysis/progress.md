# Disassembly progress

Round 1 disassembly in progress; initialized baseline verified byte-identical.

ROM classification: 5.13% (6719 / 131088 bytes classified; 124369 bytes unknown).

Initial 60-frame startup trace retained in analysis/round-01; 649 contextual roots, 640 physical executed opcode starts (includes RAM execution). Static control-flow traversal from ROM opcode roots yielded 6,599 code bytes; hardware header/vectors and bounded ascending mapper-value sequences add 120 data bytes. Graph traversal stops at mapper-affecting writes/calls. Classification and execution coverage are separate.

Checkpoint: maps/pass-001-initial.json; generated/pass-001-initial; analysis/pass-001-initial-verification.json. All map validation/generation/rebuild checks passed.

Game identified from local copyright screen as Arch Rivals (NES, Rare). Gameplay frontier: reach title/menu/court, identify quarter clock and progression. Initial long traced replay lost process before final export; captured frames show title, not court. New bounded replay disables trace during waits and saves each milestone. No natural court/progression claim yet.

No toolkit issues found; see [toolkit report](toolkit-issues.md). Phase ongoing; no terminal assessment.

## Round 1 ongoing batch update

ROM classification: 63.58% (83343 / 131088 bytes classified; 47745 bytes unknown).

True input schema corrected to NES buttons arrays (NES-001); early shared-schema button requests were no-input attract playback. Proper Start/release route and clean court snapshot now verified: Tyrone vs Vinnie, first-quarter 03:49. Short A/B/movement execution captured. Source-backed ASCII clock/quarter patches reached coaches after quarter1, halftime, coaches after quarter3 and FINAL SCORE after quarter4; final/state/stat snapshots and frames retained in analysis/round-01. All patched-route execution is artificial, not natural reachability. Post-halftime and post-final progression continues in a bounded batch.

Static code gains derive from control-flow roots at accepted opcode fetches and direct JSR entry contracts (calls preserve banks on entry; return context remains unresolved where unobserved). No legal-decode-only blanket code classification. Image ranges derive from direct CHR-RAM upload loops, source pointers, neighboring code boundaries and independently inspected recognizable planar tile content. Uniform generic data uses maximal repeated-byte intervals after checking for conflicting code.

Initial checkpoint history retained above; latest classifications validated and state/map saved. Final checkpoint follows last milestone execution merge. Toolkit report: one confirmed documentation/API-validation bug, workaround active, no blocker.

## Round 1 disassembly pass complete — incomplete ROM

ROM classification: 63.91% (83773 / 131088 bytes classified; 47315 bytes unknown).

Structural totals: {'Code': 18680, 'Data': 1947, 'Image': 63146}. Header16 bytes classified separately from PRG banks. 720 regions; every classified region has evidence. Code execution views retain effective physical8KiB banks and CPU aliases. Distinct accepted ROM opcode starts across retained contexts: 3647; other-storage starts: 59 (RAM, no ROM-upload-origin claim). Natural and artificial routes are separated in scenarios.json. All retained trace summaries have aggregate_dropped=0; capacity0 intentionally discards detailed events while retaining aggregates.

| PRG8KiB bank | Code | Data/image | Unknown |
| --- | ---: | ---: | ---: |
| 0 | 5791 | 0 | 2401 |
| 1 | 2107 | 97 | 5988 |
| 2 | 160 | 0 | 8032 |
| 3 | 1074 | 93 | 7025 |
| 4 | 0 | 8005 | 187 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 39 | 8130 | 23 |
| 8 | 1235 | 0 | 6957 |
| 9 | 3170 | 0 | 5022 |
| 10 | 1188 | 330 | 6674 |
| 11 | 3396 | 170 | 4626 |
| 12 | 202 | 7918 | 72 |
| 13 | 318 | 7576 | 298 |
| 14 | 0 | 8192 | 0 |
| 15 | 0 | 8182 | 10 |

Validated/saved analysis/state.json; exported maps/current.json. Fresh verified checkpoint [pass-001](../maps/pass-001.json), source generated/pass-001, report [verification](pass-001-verification.json): byte_identical=true, ROM SHA unchanged. Initial checkpoint pass-001-initial retained.

Runtime scope: natural selection and first-quarter court; artificial clock/quarter patches reached all quarter-end systems, halftime and FINAL SCORE, and supported halftime continuation returned to third-quarter court. Post-final state17 remains blank after wait and A/Start/B tests; statistics/leaderboard unverified. Broad game milestones are tracked in [scenarios](scenarios.json); exact step/input/edit budgets are in cited scripts.

Static gains: actual accepted opcode roots plus bank-aware successors and direct JSR callee entry contracts; mapper-affecting writes/calls stop unproven return context. Two final supported direct-callee expansion batches added0 bytes. Remaining unknown complement: round-01/unknown-ranges.json. This is an easy-gain handoff, not terminal exhaustion or whole-ROM completion; unresolved data/dispatch/call-return questions go to commentary/redirect.

Toolkit: one confirmed documentation/API-validation issue NES-001, no blocker; [toolkit report](toolkit-issues.md). No toolkit modification, publishing, cleanup, or archive performed. Disk8.6GiB free, phase artifacts215MiB, reusable scratch16KiB; nothing deleted.

## Round 1 commentary active

Prepared commentary frozen baseline from pass-001; initial byte identity passed. Full source first reading complete; mechanical routine groups recorded in commentary/semantic-units.json. Applying whole-source naming and comments, then fresh verification and reading-copy refresh. Classification remains 63.91%; phase still ongoing.

## Round 1 commentary active

Prepared commentary frozen baseline from pass-001; initial byte identity passed. Full source first reading complete; mechanical routine groups recorded in commentary/semantic-units.json. Applying whole-source naming and comments, then fresh verification and reading-copy refresh. Classification remains 63.91%; phase still ongoing.

## Round 1 commentary complete for current source

ALL current classified source now has routine/control-flow/data explanations, descriptive physical-aware labels and shared RAM/hardware constants. 982 physical review partitions and 124 routine/structure groups reviewed; 1,355 code labels, 274 symbols/aliases. All 262 unknown spans have local explanatory notes; classification remains 83,773/131,088 bytes (63.91%), 47,315 unknown. No pending readability ranges.

Fresh commentary/verification.json reports byte_identical=true, input SHA unchanged; final source hashes checked against disk. reading-after.asm refreshed afterward. maps/current.json remains byte-for-byte equal to pass-001.json. Frozen before/baseline/phase1-state preserved. Commentary handoff: [round-01-commentary](handoffs/round-01-commentary.md); detailed progress and semantic uncertainties in commentary/progress.md and uncertainties.md. Final editorial_audit pending; whole-ROM classification remains separate. No new toolkit issues, no toolkit edits/archive/publish/cleanup.

## Round 1 redirect active — structural contracts batch

Input pass-001 (83773 classified, 47315 unknown); commentary gate verified. Research at analysis/redirect/r01-structural-contracts/plan.md; exact unknown ledger at unknown-ledger.json. No map/source changes. Consumer-grounded candidates: frame two-pointer table; screen-loader30 pointer records with independent next-handler endpoint; audio24 pointer records adjacent first command; HUD18 selected records. RAM upload template decoded in relocated RAM view: wrappers explicitly restore bank0, with nonreturn bank1 entry distinguished. Continuing target/control/stack and data boundaries; first ready finding is not phase completion.

Redirect active checkpoint: source-backed closures propose10676 code bytes (unaccepted), plus bounded data proposals and15 directly consumer-referenced patterned display intervals (11399 bytes). Inline table/template barriers and overlap review remain required. ENDSTATE decisive correction: former state17 was decimal$11; PC$D491 was zero-denominator percentage loop with$0722=0,$0724=1. Targeted$0722=1 repair actually displayed GAME STATISTICS and A returned title; artificial only. Durable findings and exact script/results under analysis/redirect/r01-structural-contracts. Clean highscore/entry continuation test underway. No map/commentary changes.

## Round 1 redirect complete — combined ready batch

Input/output structural checkpoint unchanged: pass-001; accepted83773/131088 (63.91%),47315 unknown. Research attempt analysis/redirect/r01-structural-contracts completed; handoff analysis/handoffs/round-01-redirect.md. Updated ready IDs:R01-DISPATCH,R01-RETURN,R01-DATA,R01-IMAGE-END,R01-PROGRESSION; R01-ENDSTATE verified with corrected premise. Next phase: round2 disassembly independent review of combined batch.

Proposal audit:11179 code bytes=11059 unknown+120 existing Image->Code;34540 unknown data;45599 new bytes if all accepted,1716 residual then. No code/data/fetch overlap;197 normal-return summaries/5119 proposed opcode starts, zero stopped calls under explicit bounded preconditions. Mandatory correction physical64155..64275 is120 bytes (FA8B..FB02 bank[4,5,6,7]), not earlier122 wording. Prior broad image suffix overran executed code; exact split/evidence/dual-use consumer audit in handoff. No map edits or double-counting.

Progression: blank saved state17 DECIMAL=$11 was zero-denominator stats loop, repaired with specific RAM contract. Source-supported qualification reached record/initials/statistics/AAA50-100% leaderboard/title, with actual RAM record commit. Tie shortcut reached SUDDEN DEATH OVERTIME and initialized quarter4court01:00 with representative inputs. All artificial routes separate from prior natural court. Historical failures and exact budgets preserved; scenarios updated.

Checkpoint/hash/readability verification retained: ROM SHA unchanged, maps/current=pass001, commentary/source hashes match existing byte-identical verification. Current accepted source untouched; next commentary must fix decimal/hex prose and comment new classifications/image correction. No new toolkit issues; no toolkit/archive/publish/cleanup. This is productive phase handoff, not terminal exhaustion. Exact current unknown ledger plus hypothetical proposal residuals retained.

## Round 2 disassembly active

Independently reviewing saved structural contracts, physical instruction boundaries and data consumers. Prior commentary preserved. No unchanged gameplay replay: representative repaired statistics, initials/leaderboard and overtime evidence reused. Batch checkpoint pass-001; accepted classification remains83773/131088.

## Round 2 disassembly complete — incomplete ROM

ROM classification: 99.85% (130891 / 131088 bytes classified; 197 bytes unknown).

Structural totals: {'Data': 37807, 'Code': 30058, 'Image': 63026}; header16 counted separately from PRG. 1101 evidence-backed regions. Added47118 unique bytes (11258 Code+35860 Data); mandatory120 Image->Code correction is zero coverage gain. Original combined45599 accepted after independent review;1519 connected/narrow-contract continuation bytes also classified. No proposal rejected; all five ready lead IDs verified.

| PRG8 bank | Code | Data | Image | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 0 | 7996 | 114 | 0 | 8110 | 82 |
| 1 | 4089 | 4096 | 0 | 8185 | 7 |
| 2 | 160 | 8032 | 0 | 8192 | 0 |
| 3 | 3301 | 4891 | 0 | 8192 | 0 |
| 4 | 116 | 71 | 8005 | 8192 | 0 |
| 5 | 0 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 0 | 8192 | 8192 | 0 |
| 7 | 181 | 1216 | 6795 | 8192 | 0 |
| 8 | 1936 | 6256 | 0 | 8192 | 0 |
| 9 | 4393 | 3799 | 0 | 8192 | 0 |
| 10 | 2710 | 5478 | 0 | 8188 | 4 |
| 11 | 4644 | 3444 | 0 | 8088 | 104 |
| 12 | 205 | 69 | 7918 | 8192 | 0 |
| 13 | 318 | 298 | 7576 | 8192 | 0 |
| 14 | 0 | 0 | 8192 | 8192 | 0 |
| 15 | 9 | 27 | 8156 | 8192 | 0 |

Fresh configured-toolkit exact decodes and independently rooted control graph checked5119 original proposed starts and197 conditional normal-return summaries. Explicit FD/row-count/character-index/EntryPlayerIndex preconditions retained, with inline barriers and bank force/return context. Original actual-fetch artifacts independently matched before Observed provenance; static descendants remainStatic. Details: [review](round-02/review.md), [summary](round-02/summary.json), [unknown ledger](round-02/unknown-ledger.json).

Current state saved and portable map exported/validated. Fresh [pass-002](../maps/pass-002.json), source generated/pass-002, [verification](pass-002-verification.json): byte_identical=true, input SHA unchanged. All prior commentary source hashes unchanged; commentary phase remains separate and must catch up new classifications plus decimal17=$11 historical prose correction. Prior completed redirect records unchanged.

Representative progression reused rather than replayed: coherent artificial statistics, record/initials/AAA50 leaderboard/title and initialized overtime court, plus original natural court route remain in scenarios.json. No new runtime/artificial route claimed. Credits stream17 remains statically classified, naturally unverified. Residual197 bytes in15 localized intervals have code/entry or small mixed-boundary ambiguity, not merely unknown data meaning. R02-RESIDUAL recipe directs actor-update and player-control contracts, isolated JMP triples and tiny data ambiguities to redirect. Easy-gain handoff, not terminal exhaustion.

Toolkit: no new issues; existing NES-001 documentation/input-schema issue remains with workaround in [toolkit report](toolkit-issues.md). No toolkit edits, publishing, archive, cleanup or input changes. Disk about8.3GiB free; scratch/review records retained.

## Round 2 commentary active — baseline prepared

Entire round1 commentary preserved at analysis/round-02/commentary-round-01/ before preparing current pass002 baseline. Fresh preparation byte-identical. Current scaffold1116 entries: exact physical/kind carry candidates640 Code spans18680 bytes,34 Data1947,45 Image31842,11 Unknown166; new/changed274 Code11378,107 Data35860,1 Image31184,4 Unknown31. All current classified source remains in scope; 120-byte Image->Code correction included. Carry will preserve physical identity and valid symbols/prose, reverify dependencies. New work11378 Code +35860 Data and corrected Image boundary pending; editorial audit reserved.

### Carry verified; new-code inspection complete, descriptions in progress

730 exact physical/kind spans carried with prior prose and symbols; carried source freshly verified byte-identical (verification.json). A temporary duplicate-gap assembly construction was caught by verification and fixed before this successful check. Entire current new-code listing inspected (274 spans,11378 bytes), including geometry/score, screen loaders/selection, bank-one return/leaderboard correction, audio commands, statistics/record insertion, computer-input and HUD patches. Written191+ mechanical routine/continuation descriptions in analysis/round-02/commentary_descriptions.txt; these are staged editorial input, not yet claimed source completion. Newly classified data evidence/consumer/format list inspected across all107 spans35860 bytes; bounded mixed containers retain explicit layout uncertainty.

Remaining source-edit frontier: apply staged descriptions and symbols to ALL274 new Code spans; annotate107 new Data spans; corrected Image boundary31184 bytes; local notes for15 unknown spans; refresh dependency comments/review identities and correct decimal17=$11 statistics confusion without changing legitimate stream/state$17. Review scaffold remains pending for these386 changed spans until source changes actually exist and reverify. Final editorial audit remains pending.

### Full source application verified; dependency refinement

Applied all274 new Code summaries and107 Data notes plus corrected Image endpoint and all new Unknown notes. New code includes descriptive entry/branch labels, shared RAM symbols, record fields, bank helper contracts and consumer-established data names. Review scaffold1116 entries retained and reviewed;511 compact physical units include124 carried groups and new continuations/structures. Full applied batch freshly verified byte-identical (game.asm aa2a825e1fb261f835c3edeee06f67d4d595a2b88195ac91a32fb4740ef85b37). Temporary symbolic .org/data-equate assembly errors were corrected before that check; no toolkit issue.

Concrete dependency refinement now removes stale claims that newly classified bodies are gaps, names numeric hardware/array/data-source operands while retaining widths, fixes frame/audio table bounds and the percentage denominator contract, and separates repaired statistics decimal17=$11 from legitimate credits$17. Corrected leaderboard grammar: zero terminates; $55 introduces its repeat count. Fresh refinement verification pending. Review-location/identity check, reading-after refresh and final phase report remain. No current source range intentionally deferred; editorial acceptance audit still reserved.

## Round 2 commentary complete — current-source scope

Complete readability coverage for pass002:30058 Code+37807 Data+63026 Image; all197 Unknown bytes in15 intervals have local notes. No current-source readability range remains. Whole-ROM classification remains99.85% and final editorial acceptance is separate.1116 scaffold entries retained/reviewed;511 unique units, all124 prior identities preserved. Entire prior commentary retained under round-02/commentary-round-01; frozen before/ retained. See round-02/commentary-scope-check.json and handoffs/round-02-commentary.md.

Final fresh commentary verify byte_identical=true; reading-after refreshed. Exact source hashes in commentary/verification.json and handoff. Maps/current.json and analysis/state.json equal frozen commentary baseline/state; input SHA unchanged. Concrete stale gap/table/statistics summaries corrected; decimal17=$11 artificial statistics is distinct from legitimate static credits$17. No current-source gaps; unresolved game meanings explicitly documented. R02-RESIDUAL still structural follow-up, not investigated here. Final editorial_audit remains pending. No new toolkit issues; NES-001 remains with workaround. No toolkit edits, audit, archive/publish/cleanup or ROM changes.

## Round 2 redirect active — r02-residual-contracts

Input pass002 remains130891/131088 classified;197 unknown bytes/15 intervals. Current-source commentary complete. Unique attempt analysis/redirect/r02-residual-contracts/plan.md preserves prior immutable r01 investigation. Actor/player branch mechanical contracts, independent tiny boundaries/JMP islands and small data fragments are actionable; inspecting primary exact bytes/neighbor contracts and earlier decoder/return evidence. Structural role and natural reachability separate; no classification/source edits or repeated broad gameplay.

### Redirect coherent batch ready —172-byte Static Code proposal

Saved analysis/redirect/r02-residual-contracts/findings.md, exact code-proposals.json, candidate-decodes.json, current-code-reference-check.json and hardware-vectors.json. Complete local contracts support EA39 actor steering78, EFE6 player gate26, two cursor prefixes4, menu fragments28, frame setup35 and hardware IRQ RTI1 =172 bytes in9 intervals. Natural selection unresolved; no observed provenance claimed. Connected waypoint/latch/cursor/menu continuations inspected and already classified. Other independent tiny boundaries inspected; five isolated JMPs15bytes and10 short scalar/code-ambiguous bytes remain provisional25/7, exact proposed-residual-ledger.json. Full current13741 instruction direct-reference check yielded no residual targets; banked encoded-entry coincidences rejected against actual instruction/OAM layouts. Current accepted197/15 remains exact, recomputed state complement and saved accepted-unknown-ledger.json. No map/source/state/ROM or prior attempt mutation; no new toolkit issue. R02-STATIC-CONTRACTS ready in leads.json for independent disassembly, not terminal exhaustion. Input hashes unchanged including existing successful byte-identical commentary verification; final editorial audit reserved.

## Round 3 disassembly active — independent static-contract review

Input pass002:130891/131088 classified,197 unknown. Reviewing R02-STATIC-CONTRACTS172 bytes/9 intervals against fresh configured-toolkit decodes, neighboring source roles and explicit bank/entry contracts. Structural and reachability claims separated; no repeated gameplay route. Remaining25-byte conditional residual is not auto-accepted. Commentary frozen for separate phase. Review artifacts: analysis/round-03/.

## Round 3 disassembly complete — incomplete ROM

ROM classification: 99.98% (131063 / 131088 bytes classified; 25 bytes unknown).

Accepted R02-STATIC-CONTRACTS172 Static Code bytes/9 intervals after independent role/bank/boundary review. Totals Code30230, Data37807, Image63026; header16 included in Data, separate from PRG. 1110 internal regions.

| PRG8 bank | Code | Data | Image | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 0 | 8060 | 114 | 0 | 8174 | 18 |
| 1 | 4089 | 4096 | 0 | 8185 | 7 |
| 2 | 160 | 8032 | 0 | 8192 | 0 |
| 3 | 3301 | 4891 | 0 | 8192 | 0 |
| 4 | 116 | 71 | 8005 | 8192 | 0 |
| 5 | 0 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 0 | 8192 | 8192 | 0 |
| 7 | 181 | 1216 | 6795 | 8192 | 0 |
| 8 | 1936 | 6256 | 0 | 8192 | 0 |
| 9 | 4393 | 3799 | 0 | 8192 | 0 |
| 10 | 2714 | 5478 | 0 | 8192 | 0 |
| 11 | 4748 | 3444 | 0 | 8192 | 0 |
| 12 | 205 | 69 | 7918 | 8192 | 0 |
| 13 | 318 | 298 | 7576 | 8192 | 0 |
| 14 | 0 | 0 | 8192 | 8192 | 0 |
| 15 | 9 | 27 | 8156 | 8192 | 0 |

Fresh configured-toolkit decode, structural review and exact residual ledger: round-03/review.md, fresh-decodes.json, neighbor-contracts.json, unknown-ledger.json, summary.json. Five isolated JMPs15bytes and two short code/data ambiguities10bytes remain unknown; no unsupported residual inference. Connected accepted continuations checked; representative gameplay evidence reused without repetition. R02-STATIC-CONTRACTS verified; no proposal rejected.

Saved state/export validated. Fresh maps/pass-003.json and generated/pass-003 rebuild byte_identical=true (pass-003-verification.json). Input SHA unchanged. Full commentary file-tree hashes unchanged; separate commentary must cover172 newly classified bytes and update dependency/unknown notes. Final editorial audit remains pending. No new toolkit issue; existing NES-001 workaround remains in [toolkit report](toolkit-issues.md). No toolkit edits/archive/publish/cleanup/input changes. Compact handoff: handoffs/round-03-disassembly.md.

## Round 3 commentary active — preservation and baseline preparation

Entire prior commentary preserved at analysis/round-03/commentary-round-02 before fresh map-changed preparation. Scope: carry all valid physical/kind/dependency review and symbols/prose, then explain172 new Static Code bytes/9 intervals, revise affected unknown/dependency notes, retain25 honest unknown bytes/7 intervals. Full current-source inspection, fresh verification and reading refresh pending; final editorial audit reserved.

## Round 3 current-source readability complete

All131063 classified bytes covered:30230 Code,37807 Data including16-byte header,63026 Image. Entire prior commentary is retained under analysis/round-03/commentary-round-02;1108 exact physical/kind spans carry valid review/prose/symbols. Nine new Static Code spans172bytes explain menu and frame/IRQ setup, two cursor prefixes, paired-computer steering and player-side interaction gates. Stale unknown/dependency notes at those spans removed; seven residual unknown intervals25bytes have exact local uncertainty notes. No current-source range deferred.

1117 scaffold entries and520 unique physical units reviewed; all511 prior unit identities retained. Full inventory/dependency assessment: analysis/round-03/commentary-scope-check.json. Final fresh verification byte_identical=true; source game.asm SHA7151eef698f68cf4181d1eb19d0acf2f0c795fe87146ebc1c5b00fd2040965db; symbols.inc SHA065221d257ff9f21098d73bd7dfd43c4b74a7ffffc736f6af700721964a19e30. reading-after.asm refreshed. Input, current map and analysis state identities unchanged.

Natural entry of new static routines remains unresolved; bank/selector/interrupt-frame contracts remain explicit, no new Observed claim. Residual isolated JMP source roles and two short code/data boundaries remain questions for redirect. No new toolkit issue; NES-001 workaround remains active. Final editorial audit remains pending; no redirect/toolkit edits/archive/publication/cleanup performed. Handoff: analysis/handoffs/round-03-commentary.md.

Round03 redirect active: r03-layout-review, checkpoint pass003,25 unknown bytes/7 intervals. Reassessing containing-layout motif for five isolated JMPs and generic-data patterns for two tiny fragments against primary r02 bank-aware decodes; no gameplay replay or classification mutation.

Round03 redirect r03-layout-review: fresh indexed-base audit13818 accepted instructions found3 apparent overlaps intoA951; boundedY0..46 excludes all(maxA930). Five-JMP joint-layout inference evaluated against exact primary local/target decodes; no fixed enclosing layout established. Both tiny generic-data alternatives reviewed without semantic/caller prerequisites. Compact rejection-review.md persisted for coordinator inspection; accepted complement unchanged25bytes/7intervals; no proposal.

## Final editorial acceptance active

Reusing1117 scaffold reviews and520 unique units; targeted generic-label, uncertainty, bank-contract and stale-claim checks. Found concrete numeric-alias gap in established ball trajectory mechanics: E9/EA signed axis-error pair, EB/EC retained horizontal span, ED depth magnitude. Replaced only zero-page operands with shared symbols; instruction widths/values preserved. Structural state and25 unknown bytes unchanged. Fresh supported audit pending.

## Final editorial acceptance passed

Targeted checks reused1117 scaffold reviews/520 unique units. Repaired36 trajectory numeric operands; supported audit additionally exposed1637 source-path prefixes and516 continuation-unit links, all corrected without new identities. Fresh audit byte_identical=true/editorial_audit=true;1036 unit links,520 unique units. Source hashes and post-audit input/map/state identities checked; reading refreshed. Detailed findings: commentary/editorial-audit.md. Compact handoff: analysis/handoffs/final-editorial-acceptance.md. Whole-ROM classification remains99.98%,25 unknown bytes/7 intervals, terminal INCOMPLETE; no source/review backlog. No archive/cleanup/publication/toolkit changes. Existing NES-001 nonblocking; no new toolkit issue.

## Coordinator terminal acceptance — justified incomplete

Workflow reached its documented terminal incomplete handoff after separate sequential workers for rounds1–3. Current structural checkpoint pass-003 remains131063/131088 classified: ROM classification: 99.98% (131063 / 131088 bytes classified; 25 bytes unknown). Whole-ROM classification remains incomplete. No blocked/interrupted work is recast as terminal success.

Coordinator inspected complete r03 rejection reasoning and decisive primary local bytes/neighbor/target/indexed-domain artifacts; sound scope and evidence standards confirmed in coordinator/rejection-review.md. No ready lead or credible corrected next test survives unchanged premises. Five isolated JMP triples and two tiny code/data ambiguities remain in seven exact ledger intervals. Research is resumable for materially changed evidence/inference or explicit reassessment.

Final separate editorial worker completed targeted acceptance and repaired concrete trajectory symbols, source-anchor paths and continuation-unit links. Supported commentary audit passed byte-identically with1117 reviewed regions,1036 unit links/520uniqueunits, current reading copy and hashes. Fresh coordinator store/map validation also passed. Final handoff analysis/final-handoff.md links checkpoint, residuals, current source/readability, lead/progression/toolkit history and archive-selection evidence. Archive-options explicitly preserve final/historical documents and omit only verified duplicate preparation instructions; final full Markdown accounting is recorded in coordinator/document-inventory.json. Input unchanged; no toolkit modification/archive/cleanup/publishing. Orchestration completion does not mark ROM classification complete.
