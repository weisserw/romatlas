# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

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


## Redirect round005-a — terminal incomplete recommendation

Input/output pass-005-complete unchanged:131075/131088 classified,13 unknown bytes in8 fragments. Current-source readability gate complete; editorial audit pending. Reassessed main8110/81DB under historical-fragment claims without currentreturn requirements and all tiny Data alternatives without semantic/schema prerequisites. New bounded bank8 exact-origin test found8601 wrapper9byte match at84FD but predecessor85 differs from unknown86, so no extended origin proof. All eight exact claims and combined independent constraints/alternatives/actual negative scopes are in analysis/redirect/round005-a/rejection-review.md with fresh primary-boundaries/source excerpts. No ready leads, proposed0/accepted0bytes; no remaining productive test/frontier identified, subject to coordinator primary review. Actual saved later-world actions/ending followup and current source graph reviewed; no changed practical boss/credits shortcut. No runtime replay or server. ROM/map/state/source/configured binary untouched; no archive/deletion/cleanup/publication. NES-001 bug and NES-002 feature unchanged, no new toolkit issue/blocker.
