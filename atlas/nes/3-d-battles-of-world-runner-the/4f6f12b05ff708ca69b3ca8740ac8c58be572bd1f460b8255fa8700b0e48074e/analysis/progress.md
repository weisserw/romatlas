# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 disassembly — active
Baseline established and byte-identical: [baseline-verification.json](baseline-verification.json), map maps/baseline.json, generated/baseline. Persistent configured binary session accessed through analysis/round-001/session.py (PTY 32237). Initial state loaded; baseline was not imported as classification. Static reset/NMI discovery artifacts and bounded startup trace saved in analysis/round-001. Identified title as The 3-D Battles of WorldRunner (1987 Square). Current startup at >450 frames, mapper2 [10,11,14,15], main wait loop $FEEC. No map edits yet. Investigating title progression, correct input contract is buttons array (initial button/pressed calls were ignored); corrected Start experiment active. Trace detail capacity bounded 0/300; aggregates no overflow. Frame-step timeouts retain progress. No toolkit modifications. Baseline count remains 0/131088 classified pending evidence review. Scratch negligible; 12 GiB free at start.

Natural Start succeeded after correcting the NES buttons-array API. First classification batch saved and validated: 5,129 code bytes plus 22 structural data bytes, 2,395 regions. Header and hardware vectors included; no unknown gaps counted. Discovery large has 2,432 context candidates / 2,393 unique physical instruction intervals, completed traversal; second gameplay discovery has 2,139 contextual candidates. Collecting connected additions. World4/world8 artificial initialization attempts ongoing; no unverified later-world milestone claimed. Evidence: round-001/game-correct-input.ppm, playing.snapshot, early-game.snapshot, discovery-large*.json, game-init-contract.json. Classified state now persisted to analysis/state.json and maps/current.json.

Second batch validated and pass-001 byte-identical checkpoint created (analysis/pass-001-verification.json): 37,414/131,088 classified (28.54%), 93,674 unknown; 6,353 code, 31,061 data. Generic data additions are exact uniform residual runs (uniform-runs.json), not asserted padding/fill or semantic tables. No conflicting discovered instruction. Continued direct-call callee closure after noting mapper2 discovery conservatively halts at JSR; rooted only callee entries in source bank context, without assuming bank-preserving returns.

Broad progression attempts: World4 via PC8905/A3 failed corroboration: RAM$EE returned0; do not call visited. World8 via $EE7,$E2=0,PC8015 retained$EE7 but image still WORLD1-1 and player absent after300+20 furtherframes: initialized world rendering contract incomplete, not a verified world8 visit. Later-world/ending gates remain unresolved; bounded unsuccessful shortcuts preserved in snapshots and RAM/frame artifacts. Natural replay restored title, actual Start, then release; new clean gameplay continuation pending. No toolkit issues found so far. Scratch4KiB/free12GiB.

## Round 001 completed — incomplete ROM

ROM classification: 29.93% (39239 / 131088 bytes classified; 91849 bytes unknown).

Code8,178 bytes; data31,061 bytes (header16, vectors6, uniform generic residual runs31,039). Header is outside PRG banks. Counts from internal state, unique nonoverlapping regions; unknown export fallbacks excluded.

| Physical 8KiB bank | Classified | Unknown |
|---|---:|---:|
| 0 | 0 | 8192 |
| 1 | 369 | 7823 |
| 2 | 7034 | 1158 |
| 3 | 8176 | 16 |
| 4 | 0 | 8192 |
| 5 | 0 | 8192 |
| 6 | 701 | 7491 |
| 7 | 216 | 7976 |
| 8 | 982 | 7210 |
| 9 | 528 | 7664 |
| 10 | 3213 | 4979 |
| 11 | 2718 | 5474 |
| 12 | 4285 | 3907 |
| 13 | 2440 | 5752 |
| 14 | 1434 | 6758 |
| 15 | 7127 | 1065 |

Verification: baseline, pass-001 and final pass-001-final all byte-identical. Authoritative final report [pass-001-final-verification.json](pass-001-final-verification.json), map maps/pass-001-final.json, source generated/pass-001-final. State analysis/state.json and maps/current.json. No ROM/toolkit changes. No archive/publishing/deletion performed.

Evidence: round-001/discovery-large*.json; discovery-gameplay*.json; call-closure-evidence.json; contexts-final-*.json; final-closure-evidence.json; uniform-runs.json. Runtime summary no aggregate overflow (aggregate_dropped0), bounded detail capacity0/300 intentionally dropped detail20,516,868events; preserved aggregates and selected frames/snapshots. Reported frame timeouts are partial execution, not completed frame budgets. Artificial edits tagged in emulator state; annotation provenance conservatively Static.

Natural title/start/running/action exercised; broad best-effort later-world shortcuts failed full state corroboration. Final EE8/PC8043 attempt reached ending-like SQUARE animation; no natural completion/complete credits claim. Recipes and milestone statuses in scenarios.json/gameplay.md. No preexisting leads; new outcomes in leads.json.

Toolkit: one confirmed bug NES-001 (observed ROM roots not globally seeded by discovery), workaround completed; no suspected bugs/missing features recorded; no blockers. See [toolkit-issues.md](toolkit-issues.md).

Stopping scope: this is a single substantive easy-gain pass, not terminal exhaustion. Broad nonuniform graphic/table regions and bank-return/inline-data analysis need focused redirect. Commentary should review full current source; labels currently header/vector only and readability incomplete. Strong next questions: graphics copy boundaries, proper later-stage initializer, RAM upload bounds, callee return bank preservation. Do not infer remaining data from baseline or mere nonexecution.

Final evidence audit: explicit-root closure has no non-code annotation barrier conflicts; remaining diagnostics are indirect targets at physical131030 and117391. Input hash unchanged. Final retention inspection: scratch4KiB, phase artifacts46MiB,11GiB free. Persistent live session reloaded authoritative final state before shutdown; snapshots already exported for replay. No active experiment remains.

## Round 001 commentary in progress

Prepared frozen commentary baseline; boundary notes and hardware symbols added throughout. Reviewed bank 2 return stub, bank 8 isolated setup prefixes and bank 10 presentation/graphics/palette/controller routines. Batch verification byte-identical (commentary/verification.json). Main classified code physical98320..131082 remains pending, exact scaffold fragments in round-001-commentary-pending.json. No classification/toolkit/ROM change.

## Round 001 commentary completed — current classified source readable

All current classified source reviewed: 8,178 code bytes plus 31,061 data bytes; 4,127 scaffold entries and 128 routine/structure units recorded in commentary/review.json. This reconciles source edits that were ahead of the earlier saved frontier; historical pending list preserved, current empty readability frontier in round-001-commentary-scope.json. Main-game, projection/raster, player/object/metasprite, display/counter, audio, fixed-bank graphics and NMI/reset behavior now have source explanations and descriptive symbols. Corrected reset's bank-dependent CPU $8000 naming, mapper-value/physical-bank confusion, and added projection/motion/effect/sound-field aliases.

Fresh commentary verification byte-identical, source game.asm SHA-256 d4eb75c818d6bcb603640715dee96814f9e2d751ed724d9f2c1e0f46b13ff2e4; reading copy refreshed. See commentary/verification.json and commentary/reading-after.asm. State/map hashes still match frozen manifest. Classification remains 39,239/131,088 (29.93%), 91,849 unknown; no classification or execution-coverage gain claimed. Final editorial audit remains pending.

Compact handoff: commentary/progress.md; concrete unresolved questions in commentary/uncertainties.md. Redirect should investigate connected graphics/source extents and return-bank contracts, later-stage initialization, RAM-helper origins, and sound/object handler continuations. Earlier $EF substage interpretation is not established; source names it AlternateDisplayState from consumers. No unread classified ranges deferred. Existing toolkit report: one confirmed NES-001, no new findings, no blockers; see toolkit-issues.md. No toolkit/ROM change, archive, publishing, redirect execution or workspace deletion. Disk 11 GiB free; scratch 144 KiB.

## Round001 redirect — active
Unique research root analysis/redirect/round001-a; checkpoint pass-001-final, unchanged 91849-byte unknown complement. Investigating newly readable consumer contracts, starting graphics, RAM upload, progression transition and D0 audio. No map/source mutations. Frontier in attempt plan; no terminal assessment.

## Round001 redirect completed — combined batch ready for independent disassembly

Authoritative handoff [round-001-redirect-handoff.json](round-001-redirect-handoff.json), findings [redirect/round001-a/findings.md](redirect/round001-a/findings.md), proposal [redirect/round001-a/proposal-batch.json](redirect/round001-a/proposal-batch.json). Seven ready leads in leads.json; stage-contract historical blocked status preserved in attempt history and now verified. Connected and independent graphics/object grids, bank-aware missing code, exact RAM-upload origin, progression/actions, audio, projection/motion/text/display, dormant bank2 initializer and bounded metasprite structures investigated together. Static closure reached stable rootset38iterations; actual artificial later-state fetches remain distinct from natural reachability.

Proposed, not accepted:3975Code+71338Data/Image/Text=75313unique unknown bytes,1909instruction candidates. Projected residual16536 only if independently accepted. Current map/state/classification unchanged:8178Code+31061Data=39239/131088classified,91849unknown. Exact154range complement and proposal-aware residual dispositions retained. verification-batch.json reports no instruction overlaps, existing/proposed Code/Data conflicts or ROM-bound violations; ROM/state/map/source SHA-256 match frozen input and accepted commentary. Current accepted-source readability complete; final editorial audit pending. Earlier proposal.json/proposal-final.json preserved and superseded by proposal-batch.json.

Artificial source-supported stages3/7 visited with distinct scenery, player respawn and A+right action; pending-NMI prerequisite corrected,248tick respawn gate respected. EE8 path visits bank8/9 hero/segmented-enemy animation;540additional frames withA did not show terminal text/complete credits. Recipes/evidence in scenarios.json/gameplay.md. No natural later-world or full ending-completion claim.

Remaining concrete fragment/selector/bank/stack barriers and localized BD50pointer-table counterexample are in findings; no terminal exhaustion assertion, no empty workflow cycle. Next: independently validate/classify combined batch, verify checkpoint, complete whole current-source commentary, then redirect. No active process or unfinished research represented as exhausted.

Toolkit: existing confirmedNES-001 and new documented missingfeatureNES-002 (provably non-mapper indexed stores stop discovery); no suspected bugs/blockers. Workarounds used with explicit address-domain contracts, toolkit unmodified. No ROM/source/classification changes, archive, cleanup, deletion, publication or sharing. All assets/snapshots/evidence local.

## Round 002 disassembly — active independent review

Authoritative round001-a proposal under independent review; accepted checkpoint pass-001-final preserved. Reviewing return and mapper contracts, indexed stack aliases, physical boundaries, atlas structure and bounded metasprite records before annotations. Review frontier in round-002/frontier.json; no new classified bytes yet.

Round002 executable batch accepted: 3,975 new Code bytes / 1,909 instructions, individually freshly decoded, rooted/certified under exact physical bank contexts. Review123return graphs and1763roots had0errors; no indexed/mirrored-stack alias in actual proofs. Saved/validated state and map; current43,214classified (32.97%),87,874unknown. Evidence round-002/review.json and review-notes.md.

Round002 data batch accepted: 72,685 new Data/Image/Text bytes (71,338 from independently reviewed redirect proposal plus1,347 from scoped BD50prefix/37record correction). Copy counts combined with coherent atlases/noncode layout, exact table masks/text terminators, generic fallback consumer/pattern review and independent metasprite parsing. Failed60entry BD50extrapolation excluded at index37; first37records form exact contiguous114090..115363 boundedchain. Saved/validated state/map; current115,899classified,15,189unknown (88.41%). Full current-source commentary pending; prior commentary source preserved.

## Round 002 completed — incomplete ROM

ROM classification: 88.47% (115977 / 131088 bytes classified; 15111 bytes unknown).

Code12,231; Data50,674; Image52,878; Text194. New gain76,738bytes:4,053Code and72,685noncode. Unique evidence-backed physical bytes from internal state; Unknown gaps and raw fallback excluded. Header16 is outside PRG banks.

| Physical8KiB bank | Classified | Unknown |
|---|---:|---:|
| 0 | 8192 | 0 |
| 1 | 8192 | 0 |
| 2 | 8187 | 5 |
| 3 | 8192 | 0 |
| 4 | 8192 | 0 |
| 5 | 8192 | 0 |
| 6 | 8192 | 0 |
| 7 | 8192 | 0 |
| 8 | 3112 | 5080 |
| 9 | 8192 | 0 |
| 10 | 8069 | 123 |
| 11 | 8192 | 0 |
| 12 | 6588 | 1604 |
| 13 | 4104 | 4088 |
| 14 | 4077 | 4115 |
| 15 | 8096 | 96 |

Independent review: [review-notes.md](round-002/review-notes.md), [review.json](round-002/review.json), [bd50-review.json](round-002/bd50-review.json), [parser-tail-review.json](round-002/parser-tail-review.json). Fresh1909candidate/4124cached instruction decodes,123return graphs,1763roots,59data exact intersections and168metasprite record parses had0errors. Additional BD50scope correction adds37ordered words and37contiguousrecords114016..115363 (1347bytes), preserving index37word0700 counterexample to60entry extrapolation. CBBB78byte tail independently decoded44instructions throughCC97RTS under reviewedRAM05xx/Y<=15contract; discovery indirect-store barrier explains prior omission.

Seven ready lead IDs all verified on supported structural slices; exact outcomes/history in leads.json and round-002/handoff.json. Existing stage-contract remains verified, no new natural progression claim. Saved artificial stages3/7 scene/respawn/actions and EE8animation evidence reused; full credits/boss completion unresolved. No startup or unchanged held-input replay repeated.

Verification: code-only pass-002, data pass-002-final and final [pass-002-complete-verification.json](pass-002-complete-verification.json) all byte-identical. Authoritative checkpoint maps/pass-002-complete.json, generated/pass-002-complete; analysis/state.json == maps/current.json == checkpoint map. ROM SHA256unchanged. First data annotation transactionally rejected Code-only execution-view metadata; corrected Data source contexts to comments/evidence, historical script/error retained. Expected schema enforcement, no toolkit defect.

Easy-gain stopping scope: accepted connected proposal plus scoped BD50andCBBB continuations, not terminal exhaustion. Remaining15,111bytes in77exact physical fragments are mainly bank8mixed program/literals, bank12/13object continuations and fixed14object code/data blocked on indirect destination/dispatch/stack contracts. Tiny bank2/bank10metadata remains materially ambiguous. Current fixed-point direct/return/indexed-store closure has no further established roots; broad repetitive gameplay offers no discriminating fact. Whole current-source commentary should now reveal specific local contracts for next redirect rather than classifying remaining raw gaps speculatively. Previous commentary/reading/source untouched, readable only for prior39,239byte checkpoint; carry its baseline forward and review all new source separately. Final editorial audit pending.

Toolkit: NES-001confirmed bug and NES-002documented missing precision feature unchanged; no suspected bugs, new findings or blockers. See [toolkit-issues.md](toolkit-issues.md). No toolkit/ROM changes, archive, cleanup, deletion, publication or sharing. All evidence/assets local. No live process remains.

Final retention: analysis/round-0021.2MiB, final generated checkpoint5.3MiB;11GiB free. No deletion/cleanup performed. All analysis server subprocesses have exited.

## Round 002 commentary completed — current classified source readable

Completed the whole current-source readability scope at pass-002-complete: 12,231 Code plus 103,746 Data/Image/Text bytes, 6,107 scaffold entries and 291 reviewed routine/structure units. Retained all 128 previous units, carried 3,980 exact physical source ranges and 4,021 scaffold reviews, reverified carry and reconciled 37 affected descriptions. Entire prior commentary is preserved at round-002/commentary-pass-001-preserved/ with the original source hash intact. No unread classified ranges remain; all 77 unknown fragments have source uncertainty notes.

Source now explains bank2 dormant PPU initialization, bank8 animation/segment/OAM/palette and separate sound driver, main/fixed-bank bank-aware continuations, projection/collision handling and compound object parser. Expanded text/pointer/lookup/metasprite formats make their records readable directly. Corrected reset marker selection ($35=>mapper6/main; $36=>mapper4/scene), the two-byte palette swap, and unsupported claims about ordinary parser tail fields. Exact unresolved meanings and next structural questions in commentary/uncertainties.md; no speculative classification.

Fresh final commentary verification byte-identical; source SHA317db909e3cc51dcff3cf40dd7e53a01e9af80159c08c984be4cc53418c9fcf8; reading-after.asm refreshed. Current/state/checkpoint/frozen map hashes all unchanged at3924f32566ee7e567a902af723f59d57721a7bbd465169eb5dfaf43448bcb7ca. [Scope](round-002-commentary-scope.json), [handoff](round-002-commentary-handoff.json), [empty readability frontier](round-002/commentary-frontier.json). ROM unchanged.

Classification remains 115,977/131,088 (88.47%); 15,111 unknown bytes in77fragments. Full current-source readability is complete; whole-ROM classification is incomplete and final editorial audit remains pending. Next phase redirect: bank8 mixed record/config/song backing, bank12/13/14 indirect handler/RAM/stack contracts, tiny ambiguous boundaries and scoped progression limits. Resolved prior RAM0770/D0/graphics extent questions are reconciled in the uncertainty record rather than repeated.

Toolkit report unchanged: NES-001confirmed bug and NES-002documented missing precision feature, no new findings/suspected bugs/blockers; see [toolkit-issues.md](toolkit-issues.md). No ROM/toolkit edits, archive, cleanup, deletion, publication or sharing. All source/assets local; about11GiBfree; no live analysis process.

## Round 002 redirect — combined batch ready for independent review

Attempt [round002-a](redirect/round002-a/handoff.json) started from pass-002-complete and the complete current-source commentary handoff. Investigated bank8 scene configuration, local sprite records, sound and dormant helper fragments; bank12/13/14 object RAM, finite dispatch and deliberate stack unwinds; independent display/motion/noise lookups, multipart sprites, fixed sound backing and small bank10/15 boundaries. Previous round001-a remains preserved. No unchanged gameplay replay.

Ready proposal: **2,506 Code + 12,490 Data = 14,996 unique currently unknown bytes**. Accepted new bytes: **0**. Current classification remains **115,977 / 131,088 (88.47%)**, with **15,111 unknown bytes in 77 fragments**. Only if every proposal is independently accepted would the residual become **115 bytes in 12 fragments**. Four new ready leads are recorded in leads.json, with source/evidence, physical ranges, bank contexts, bounded independent verification recipes and attempt histories. Prior verified/rejected histories are retained.

Key new constraints: VisitObjectRecordPage supplies RAM $05xx for pointer $63/$64; CBBB parser pointer $61/$62 has its separate RAM contract. A94D and AEEB can return normally or deliberately discard one caller frame, so ordinary-return failure was localized rather than rejecting their coherent handlers. Fifteen A44E pose targets and six 9C8E action targets close the remaining object dispatch; all 47 A07B handler targets have coherent conditional graphs. The ordinary CC29 tail writes fields 1/4/7 and toggles field $0C, preserving the commentary correction. Fixed graphics tail jumps reuse their established mapper-6 restoration contract. Complete scene/multipart/auxiliary pointer prefixes and bounded 4-byte/FF records are supported independently; shared suffixes and gaps stay explicit. Sound grammar parses all selected branches within physical bounds. Clear repeated noncode patterns receive generic Data claims where schema remains uncertain.

Fresh [verification-batch.json](redirect/round002-a/verification-batch.json) decoded **1,237 candidate instruction contexts**, with zero boundary/overlap/control-into-Data errors and zero Code/Data proposal conflicts. ROM, current map/state and commentary source hashes are unchanged. Exact current unknown ledger with neighbors and evidence-linked dispositions: [unknown-dispositions.json](redirect/round002-a/unknown-dispositions.json). Actual unclassified residue versus proposed slices is distinguished throughout. Primary reasoning and corrected narrow hypotheses are retained in [findings.md](redirect/round002-a/findings.md); proposal [proposal-batch.json](redirect/round002-a/proposal-batch.json).

Residual triage retains bank10 old callee/stack fragments (including a byte-identical main-code copy with conflicting local mapping), a six-byte main legacy call/self-loop fragment, a truncated bank8 byte, isolated return/zero opcodes and tiny metadata. No arbitrary execution or inferred missing caller was substituted for structural evidence. This is a ready independent-review batch, **not terminal exhaustion**. Next: disassembly review of the combined batch, full current-source commentary, then redirect. Final editorial audit remains pending.

Toolkit findings unchanged: NES-001 confirmed bug, NES-002 documented missing feature; no new findings, suspected bugs or blockers. See [toolkit-issues.md](toolkit-issues.md). No map/classification/source/ROM/toolkit edits, archive, cleanup, deletion, publication or sharing. No live process remains. All assets/evidence remain local.

## Round 003 disassembly — active independent review

Reviewing round002-a combined proposal under exact physical bank contexts and conditional routine contracts. No new bytes accepted yet. Durable review frontier analysis/round-003/frontier.json. Distinct RAM63/RAM61 domains, normal-or-abort stack paths, finite dispatch, sprite record boundaries and sound grammar are being checked independently.

## Round 003 completed — incomplete ROM

ROM classification: 99.91% (130973 / 131088 bytes classified; 115 bytes unknown).

Accepted 14,996 new bytes: 2,506 Code and 12,490 Data. Current totals: 14,737 Code, 63,164 Data, 52,878 Image and 194 Text. The remaining 115 bytes occupy 12 physical fragments. The 16-byte header is classified Data outside the PRG banks. Structural classification and execution coverage remain separate.

| Physical 8 KiB bank | Code | Data | Image | Text | Unknown |
|---|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 0 | 0 | 0 |
| 1 | 0 | 369 | 7823 | 0 | 0 |
| 2 | 135 | 7284 | 768 | 0 | 5 |
| 3 | 0 | 8192 | 0 | 0 | 0 |
| 4 | 0 | 0 | 8192 | 0 | 0 |
| 5 | 0 | 0 | 8192 | 0 | 0 |
| 6 | 0 | 701 | 7491 | 0 | 0 |
| 7 | 0 | 216 | 7976 | 0 | 0 |
| 8 | 2367 | 5824 | 0 | 0 | 1 |
| 9 | 0 | 4464 | 3728 | 0 | 0 |
| 10 | 385 | 4478 | 3234 | 0 | 95 |
| 11 | 0 | 2718 | 5474 | 0 | 0 |
| 12 | 5682 | 2304 | 0 | 194 | 12 |
| 13 | 4500 | 3691 | 0 | 0 | 1 |
| 14 | 1222 | 6970 | 0 | 0 | 0 |
| 15 | 446 | 7745 | 0 | 0 | 1 |

Independent evidence: [review notes](round-003/review-notes.md), [review](round-003/review.json), [accepted annotations](round-003/accepted.json). Fresh decoding covered 5,991 unique instruction contexts, including all 1,237 candidate contexts and instructions from 56 object graphs and 134 connected graphs. No encoding, alias, boundary, branch-into-data or proposal conflicts were found. Independently recomputed 49 scene, 60 multipart and four auxiliary sprite records, plus 3,047 main and 922 scene sound nodes. Consumer bounds and actual generic patterns were inspected separately.

The redirect generator's boolean ordinary-summary wording is overbroad for special callees. Acceptance explicitly uses normal-or-caller-abort structural contracts, never universal ordinary RTS. RAM $63/$64 object and $61/$62 parser contracts remain separate. Fifteen pose and six action dispatch preconditions stay explicit. CC29 toggles field $0C; the bank8 code seed copied from $8018 remains Code. All four R2 ready leads are verified on their proposed structural slices; exact outcomes and preserved histories are in leads.json and round-003/handoff.json. No natural caller or whole selector-domain claim is made.

The [pass-003-complete verification](pass-003-complete-verification.json) is byte-identical. Saved state, current map and maps/pass-003-complete.json are identical, with SHA256 662912acba2d7b605f0d0753db414b84b85ae0cd839c0f8a4ceb089b73d0b84d. Generated source is in generated/pass-003-complete. ROM SHA256 is unchanged. The entire current commentary tree remains untouched; source SHA256 is 317db909e3cc51dcff3cf40dd7e53a01e9af80159c08c984be4cc53418c9fcf8. Its readability is complete only for the prior 115,977-byte checkpoint. The next commentary phase must preserve/carry that tree separately before reviewing new source and affected descriptions. Final editorial audit remains pending.

Immediate easy-gain stopping reason: the coherent combined proposal is accepted; the twelve residual fragments need narrower local role, stack or origin evidence. The [exact unknown complement](round-003/unknown-complement.json) retains reasons and corrected CPU aliases. These include two bank10 fragments of 57 and 36 bytes, a six-byte main legacy call/self-loop, isolated return/zero/truncated bytes and four-byte metadata. This is not terminal exhaustion. Commentary followed by redirect should reevaluate the narrow structural barriers. Arbitrary execution and a missing natural caller were not used as substitutes for structural evidence.

Saved representative stage3/7 scene, respawn and action evidence and EE8 animation were reused. Unchanged startup and held-input replay were not repeated. Routes and patches remain in scenarios.json and gameplay.md. No new runtime or execution-coverage claim is made; complete credits, boss completion and natural late progression remain unconfirmed.

Toolkit findings are unchanged: NES-001 is a confirmed bug and NES-002 a missing precision feature. No new findings, suspected bugs or blockers. See [toolkit issues](toolkit-issues.md). No ROM/toolkit changes, commentary or redirect execution, archive, cleanup, deletion, publication or sharing. All assets and evidence remain local. No live analysis process remains.


## Round 003 commentary completed — current classified source readable

ROM classification remains99.91% (130973/131088bytes;115unknown in12fragments). Commentary now covers all current14,737Code and116,236noncode classified bytes:7443scaffold entries and559routine/structure units reviewed, with291prior units retained. Empty readability backlog. Complete prior commentary tree, sourceSHA317db909e3cc51dcff3cf40dd7e53a01e9af80159c08c984be4cc53418c9fcf8, preserved at round-003/commentary-pass-002-preserved with historical subtrees intact.

Carried6030exact physical source blocks and6039scaffold reviews; read1237newinstructions in39runs, explained104functional routines/continuations and164data/format units. Source includes conditional object/pose/action dispatch, separate RAM63/64 and61/62contracts, normal-or-caller-abort timer/contact paths, CC29field0Ctoggle,49scene/60multipart/4auxiliary pointer/tuple formats, bank8duplicate helpers split at864Apalette with8018seed remaining Code, sound command grammar, generic patterns and smallhelpers. Corrected stale prior prose and refreshed all source locations. Local failed carry variants retained; successful corrected carry and coherent batches verified. No toolkit defect inferred from local editing errors.

Fresh final commentary verification byte-identical, sourceSHA408586980be87e03bfda1d358372af93e26f014b557a102a84795eed1dda6618; reading-after.asm refreshed. State/current/checkpoint/frozenmap SHA662912acba2d7b605f0d0753db414b84b85ae0cd839c0f8a4ceb089b73d0b84d unchanged. ROM unchanged. [Scope](round-003-commentary-scope.json), [handoff](round-003-commentary-handoff.json), [emptyfrontier](round-003/commentary-frontier.json).

Whole-ROM classification remains incomplete; final editorial audit pending and not invoked. Next redirect follows the exact115-byte unknown complement, chiefly bank10oldcallee/stack/origin remnants and mainlegacycall/selfloop plus isolated opcodes/metadata. Missing natural callers are not the structural barrier. Actor identities, generic curve/pitch schemas and natural late progression/fullcredits remain uncertain; commentary/uncertainties.md records concrete limits. No new runtime claim or unchanged gameplay replay.

Toolkit NES-001confirmedbug/NES-002missingfeature unchanged; no new findings, suspected bugs or blockers. See toolkit-issues.md. No map/classification/ROM/toolkit changes, archive, cleanup, deletion, publication or sharing. All source/assets/evidence local; no live analysis process.

## Round003 redirect active

Worker round003-a resumed pass-003-complete (115 unknown bytes in12 fragments); current-source readability and byte-identical verification gates satisfied. Research frontier: analysis/redirect/round003-a/frontier.json. Inspect localized call-independent fragments, fourbyte pattern and independent tiny boundaries before any terminal assessment. No map/source/ROM changes.

## Round003 redirect complete-ready-batch

Input/output checkpoint pass-003-complete unchanged. Current99.9122726718% classified130973/131088;115unknown in12fragments. Produced R3-bank10-selection, R3-main-local-selfloop, R3-generic-phase-pattern:9proposed bytes (3Code6Data),0accepted. Combined proposal, exact dispositionledger, scopedprimaryreview and preservation hashes in analysis/redirect/round003-a/handoff.json. Whole acceptedCode absoluteoperand/context census corrected overlooked8003/8004selectionconsumers; localized8113selfloop and8584genericpattern separately proposed.57/36bank10remnants and tinyresiduals reviewed withactual missingdiscriminators. Savedlatergame/ending540frameevidence reviewed, no unchangedreplay. Current-sourcecommentarygate and byteidenticalverification inheritedvalid onunchangedsource. No new toolkitissues found; existingNES-001/NES-002, no blockers. No liveprocesses, map/source/ROM/toolkitedits, archive/deletion/cleanup/publication. Next independentdisassembly; no terminalexhaustion claim.


## Round004 disassembly completed — incomplete ROM

ROM classification: 99.92% (130982 / 131088 bytes classified; 106 bytes unknown).

Accepted9newbytes:3Code and6Data. Internal unique-byte totals:14,740Code,63,170Data,52,878Image,194Text;106unknown in10physical fragments. Header16bytes classifiedData outside PRG banks. Execution coverage is separate and unchanged.

| Physical8KiBbank | Code | Data | Image | Text | Unknown |
|---|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 0 | 0 | 0 |
| 1 | 0 | 369 | 7823 | 0 | 0 |
| 2 | 135 | 7284 | 768 | 0 | 5 |
| 3 | 0 | 8192 | 0 | 0 | 0 |
| 4 | 0 | 0 | 8192 | 0 | 0 |
| 5 | 0 | 0 | 8192 | 0 | 0 |
| 6 | 0 | 701 | 7491 | 0 | 0 |
| 7 | 0 | 216 | 7976 | 0 | 0 |
| 8 | 2367 | 5824 | 0 | 0 | 1 |
| 9 | 0 | 4464 | 3728 | 0 | 0 |
| 10 | 385 | 4480 | 3234 | 0 | 93 |
| 11 | 0 | 2718 | 5474 | 0 | 0 |
| 12 | 5685 | 2308 | 0 | 194 | 5 |
| 13 | 4500 | 3691 | 0 | 0 | 1 |
| 14 | 1222 | 6970 | 0 | 0 | 0 |
| 15 | 446 | 7745 | 0 | 0 | 1 |

Independent [review](round-004/review-notes.md), [freshdecoding](round-004/fresh-decode.json), [contextreferencecensus](round-004/fresh-reference-census.json), [acceptedannotations](round-004/accepted.json). Verified two bank10selection bytes from accepted consumers; independently closed8113selfloop separated from unresolved8110call; bounded00204060genericData separated from32byteupload/projection with no invented schema/label. AllthreeR3lead statuses/history updated. No broader supported new connected bytes: presentation ordinarypaths terminate at alreadyclassified8075RTS; selfloop closes; upload/projection/rowconsumer alreadyclassified.

[pass-004-completeverification](pass-004-complete-verification.json) is fresh and byte-identical; state/current/checkpoint maps identical SHA2565ad7004ff9397be12be1c88f1d721bdb066b7e5d5cb674285dfea9ff8c3cdd7f. [Exact106bytecomplement](round-004/unknown-complement.json) retains10localized fragments. ROM unchanged. Entire25filecommentarytree untouched ([hashmanifest](round-004/commentary-preservation.json)), sourceSHA408586980be87e03bfda1d358372af93e26f014b557a102a84795eed1dda6618. Current-source readability remains complete for input130,973classifiedbytes/7443entries/559units; next phase must preserve/carry whole tree then review all current source and affected explanations. Finaleditorialaudit pending.

Residual57/36bytebank10fragments retain code-like evidence but fixedcallee/stack/origin/truncatedendpoint ambiguity; tinyzero/RTS/truncated slices and8110call remain unresolved. This is ordinary easy-gain stopping, not terminal exhaustion. Saved world4/world8movement andfinalbank8animation/540framefollowup reviewed; no changed supported boss/credits shortcut, so unchanged gameplay replay omitted. Natural latecompletion andfullcredits unconfirmed. No new runtime/executioncoverage claim. Next fullcurrent-sourcecommentary, thenredirect. [Compacthandoff](round-004-disassembly-handoff.json).

Local scripting errors: annotation rejected missingid then nonzeronewid; correctedid0 before successful map mutation. Accidental unchanged-map checkpoint retained as pass-004-preannotation; only pass-004-complete above is authoritative. Not a toolkit defect. NES-001confirmedbug/NES-002missingfeature unchanged; no new findings, suspectedbugs orblockers; [toolkitreport](toolkit-issues.md). No toolkit/ROM/commentary changes, archive, cleanup, deletion, publication orsharing. All evidence/assets local; no liveprocess remains.


## Round 004 commentary completed

All current classified source is readable: 130982/131088 bytes (99.92%), with106 unknown bytes in10 fragments explained. Reviewed7444 scaffold entries and561 routine/structure units; reused all559 sound prior units and7440 unchanged records. Entire prior commentary tree (25 files), including historical subtrees, preserved at analysis/round-004/commentary-pass-003-preserved before preparing the new frozen baseline. No readability backlog remains.

Named bank10 selection values and CPU alias, tied timeout/button1/button5 paths to RAM$F9, and corrected stale entry/unknown explanations. Added independently closed main$8113stoploop, retaining unknown$8110call and fixed-data callee limitation. Documented generic$8584arithmetic data separately from exact32byte upload and projection; row-low resemblance does not claim a shared consumer/schema. Reused prior consumer/source reviews and reconciled every current unknown boundary and source location. See analysis/round-004-commentary-scope.json for full scope/limits.

Fresh verification passed byte-identically at source SHA10b0c8b09ec3c3144310e66c40850efcc96124c75a2657a89adb88a3a94d61fb; reading-after.asm refreshed. Initial local physical-label/CPU-operand alias error corrected before final successful verification. Final editorial audit remains pending and was not invoked. Map/state/checkpoint/frozen-map SHA5ad7004ff9397be12be1c88f1d721bdb066b7e5d5cb674285dfea9ff8c3cdd7f unchanged. ROM/toolkit unchanged. No cleanup, archive, deletion, publication or sharing. NES-001confirmed bug andNES-002missing feature unchanged; no new findings or blockers. Next phase redirect via analysis/round-004-commentary-handoff.json.


## Round004 redirect complete-ready-batch

Input/output pass-004-complete unchanged:99.92% (130982/131088classified;106unknown in10fragments). Reopened36byte bank10 sameCPU-origin truncatedCodefragment with corrected claimstrength; prior badcallee/openend counterexamples reject completecurrentroutine, not independently supported fragmentrole. Combined encodedorigin, fifteen coherentfieldinstructions and independentpalette/page/instructionboundaries support R4-bank10-truncated-code-origin for independentreview.36proposedCodebytes,0accepted. Related57wrapper,narrowtails,main8110,tinyboundaries andsavedlateprogression reviewed without forcedanalogy, arbitrarysweeps orunchangedreplay. Boundednewimmediate/originchecks andexactprimaryevidence at analysis/redirect/round004-a; scoped-review.md preserves priorrejection and competingliteralData interpretation. Compacthandoff analysis/round-004-redirect-handoff.json. Source/map/state/ROM hashes unchanged; inheritedcurrentbyteidenticalverification valid, currentreadabilitygate complete, finaleditorialaudit reserved. NES-001bug/NES-002feature unchanged,no newtoolkitissues orblockers. No liveprocess, ROM/toolkit/source/classification edits, cleanup/archive/deletion/publication. Next independentdisassembly thencommentary/redirect; no terminalexhaustion claim.

Round004redirect coordinator correction: initial57byte insufficiency required preservation/termination/sibling facts stronger than fragmentrole. Preserved then superseded in scoped-review.md. Three encodedinternalcontrolrelationships, twofunctionbounds andstack/fieldidioms support R4-bank10-two-function-fragment without claiming externalcallee effects, boundedloop or naturaluse. Combinedfinalproposal93StaticCodebytes(57+36),0accepted;106unknownunchanged until independentdisassembly. Compacthandoffs/leads/proposal refreshed.


## Round005 disassembly completed — incomplete ROM

ROM classification: 99.99% (131075 / 131088 bytes classified; 13 bytes unknown).

Accepted93StaticCode bytes in41instruction annotations:57byte two-function historical fragment and36byte sameCPU-origin truncated transition prefix, both explicit [10,11,14,15]. Independent [review](round-005/review-notes.md), [freshdecode](round-005/fresh-decode.json), [bytechecks](round-005/byte-checks.json), [acceptedannotations](round-005/accepted.json). Combined internal control identities/stack-field idioms/two-function bounds support57; same origin/encoded maincallee/fifteen semantic instructions/independent palette-page clip support36. Literal instructionData alternatives considered; working current routines, oldcallee effects, termination and reachability remain unclaimed. Prior history preserved and corrected at narrower scope. Fixed/localzeroData and all prior annotations unchanged. Connected continuations are already classified or Data barriers; no followthrough.

Unique totals:14,833Code;63,170Data;52,878Image;194Text. Header16Data bytes outside PRG banks. Annotation coverage: all131075classified bytes carry preserved or new structural evidence; 41new StaticCode regions have exact execution views. Execution coverage unchanged and separate.

| Physical8KiBbank | Code | Data | Image | Text | Unknown |
|---|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 0 | 0 | 0 |
| 1 | 0 | 369 | 7823 | 0 | 0 |
| 2 | 135 | 7284 | 768 | 0 | 5 |
| 3 | 0 | 8192 | 0 | 0 | 0 |
| 4 | 0 | 0 | 8192 | 0 | 0 |
| 5 | 0 | 0 | 8192 | 0 | 0 |
| 6 | 0 | 701 | 7491 | 0 | 0 |
| 7 | 0 | 216 | 7976 | 0 | 0 |
| 8 | 2367 | 5824 | 0 | 0 | 1 |
| 9 | 0 | 4464 | 3728 | 0 | 0 |
| 10 | 478 | 4480 | 3234 | 0 | 0 |
| 11 | 0 | 2718 | 5474 | 0 | 0 |
| 12 | 5685 | 2308 | 0 | 194 | 5 |
| 13 | 4500 | 3691 | 0 | 0 | 1 |
| 14 | 1222 | 6970 | 0 | 0 | 0 |
| 15 | 446 | 7745 | 0 | 0 | 1 |

[Exact13bytecomplement](round-005/unknown-complement.json) in8fragments retains individually scoped code/data ambiguities. [Freshverifiedcheckpoint](pass-005-complete-verification.json) byte-identical; state/current/checkpoint SHA256 ae516baf3dedb981da1d076b1098b3bf1f3317f65f02b4646c40a81998f61a93. ROM unchanged. [Preservation](round-005/preservation.json) verifies all preexisting regions and entire25filecommentarytree unchanged; sourceSHA 10b0c8b09ec3c3144310e66c40850efcc96124c75a2657a89adb88a3a94d61fb. Input readability7444entries/561units complete; full current-source commentary now pending, carrying entire tree and correcting old fragment explanations before redirect. Final editorial audit pending.

Saved world4/world8 actions andfinalanimation540frames/sourceidleloop reviewed; no changed supported gameplay prerequisite or terminalcallback, so no unchanged replay or arbitraryRAM sweep. Boss/fullcredits unconfirmed. Easy-gain pass complete, ROM incomplete; no terminal-exhaustion claim. [Compacthandoff](round-005-disassembly-handoff.json). NES-001confirmedbug andNES-002missingfeature unchanged, no new toolkitfindings orblockers; [report](toolkit-issues.md). No toolkit/ROM/commentary edits, archive, deletion, cleanup, publication orsharing; all artifacts local. No liveprocesses.


## Round 005 commentary completed

Current-source readability is complete for all 131075 classified bytes (99.99%). All 7483 scaffold entries and 564 meaningful units are reviewed. Reused all 561 valid prior units and 7442 unchanged entries; the entire prior 25-file commentary tree is preserved at analysis/round-005/commentary-pass-004-preserved. The 13 unknown bytes in eight fragments retain honest boundary notes. No readability backlog remains.

Explained the historical stack/call wrapper, conditional repeat helper and clipped main-transition prefix in source. Added scratch and CPU-address aliases and branch names, removed obsolete unknown/rejection prose, and preserved precise historical-only claims. The helper's external effects can change its repeat count or prevent return. Local $8200/$8229 and fixed stale callees stay Data; no current routine validity, preservation, termination, execution or natural reachability is claimed. Prior valid physical-range explanations and source names remain intact. See analysis/round-005-commentary-scope.json for full scope and limits.

Fresh byte-identical verification passed with source SHA256 07522ceead6ff9f4e7ceb4d137db21233d7a7346c41a1de640cfb89d8a3d6140; reading-after.asm is refreshed. Map/state/checkpoint and frozen baseline SHA256 ae516baf3dedb981da1d076b1098b3bf1f3317f65f02b4646c40a81998f61a93 agree. Final editorial audit remains pending and was not invoked. ROM and toolkit unchanged; NES-001 and NES-002 unchanged, no new findings or blockers. No archive, cleanup, deletion, publication or sharing. Next phase: redirect via analysis/round-005-commentary-handoff.json.


## Redirect round005-a — terminal incomplete recommendation

Input/output pass-005-complete unchanged:131075/131088 classified,13 unknown bytes in8 fragments. Current-source readability gate complete; editorial audit pending. Reassessed main8110/81DB under historical-fragment claims without currentreturn requirements and all tiny Data alternatives without semantic/schema prerequisites. New bounded bank8 exact-origin test found8601 wrapper9byte match at84FD but predecessor85 differs from unknown86, so no extended origin proof. All eight exact claims and combined independent constraints/alternatives/actual negative scopes are in analysis/redirect/round005-a/rejection-review.md with fresh primary-boundaries/source excerpts. No ready leads, proposed0/accepted0bytes; no remaining productive test/frontier identified, subject to coordinator primary review. Actual saved later-world actions/ending followup and current source graph reviewed; no changed practical boss/credits shortcut. No runtime replay or server. ROM/map/state/source/configured binary untouched; no archive/deletion/cleanup/publication. NES-001 bug and NES-002 feature unchanged, no new toolkit issue/blocker.


## Final editorial acceptance — structural ROM incomplete

Final targeted editorial review accepted. Reused 7483 reviewed scaffold entries and 564 reviewed physical-span unit identities (563 unique names); all are represented in 7692 nested audit attachments. Removed a stale complete-record reference from the nine-byte generic scene gap, renamed ten meaningful generated jump targets, clarified seven unknown-boundary notes and the historical wrapper stack-byte comment. Preserved valid earlier review credit and the overlapping contact-prefix/envelope scopes. No source readability backlog remains.

Reconciled the audit inventory schema and source-relative paths without changing scaffold identities, ranges, kinds or classification. The initially failed path audit and two intermediate successful audits are preserved in analysis/final-editorial-001. Final documented commentary.py audit passed: 7483 regions, 7692 nested reviewed units, editorial_audit=true; fresh source rebuild byte-identical and reading-after.asm refreshed. Source SHA256 917a2f87abe6bab9ac2a63236d544db913c3201c246874bedaf04be971638d55. Exact artifact hashes and checks: analysis/final-editorial-handoff.json; substantive review and remaining limits: commentary/editorial-audit.md.

Structural classification remains incomplete at 131075/131088 bytes, with 13 unknown bytes in eight fragments. Map/state/checkpoint/frozen-map SHA256 ae516baf3dedb981da1d076b1098b3bf1f3317f65f02b4646c40a81998f61a93 and original ROM unchanged. Unresolved mechanics and unconfirmed natural late progression/boss/full credits remain explicit. NES-001 confirmed bug and NES-002 missing feature unchanged; no new toolkit issue or blocker. No live processes from this phase. No archive, deletion, cleanup, sharing or publication; coordinator retains final requirements/archive-options accounting.

## Terminal coordinator acceptance

ROM classification: 99.99% (131075 / 131088 bytes classified; 13 bytes unknown).

Structural terminal incomplete accepted after primary rejection review; final editorial acceptance and fresh byte-identical audit passed. All7483entries/564unitspanidentities reviewed; zero readability backlog. Final checkpoint pass-005-complete; no ready leads or interrupted productive research. See [terminal handoff](terminal-handoff.md), [requirement audit](coordinator/final-completion-audit.md) and [current-state checks](coordinator/final-state-verification.json). Archive-options accounting prepared; no archive execution, deletion, cleanup or publication. ROM/toolkit unchanged. Orchestration complete; ROM classification remains incomplete.
