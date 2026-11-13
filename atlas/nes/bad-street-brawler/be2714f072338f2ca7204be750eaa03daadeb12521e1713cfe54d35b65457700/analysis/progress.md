# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 disassembly active
- Baseline established once: `maps/baseline.json`, `analysis/baseline-verification.json`, byte identical.
- Configured release binary used unchanged. Mapper 1, 131088 physical bytes, no CHR ROM.
- Runtime reset PC $CF4C, effective PRG banks [0,1,14,15]. Preparing bounded startup/static and gameplay exploration; classification not yet committed.
- First classification saved and verified at `pass-001-startup`: 768 regions from accepted natural fetches plus bounded vector static paths/header/vector data. Exact coverage/count report will be refreshed after gameplay. `analysis/round-001/reviewed-observed-decodes.json` preserves decoded instruction views.
- Game identified at title: Bad Street Brawler. Manual research recorded in `analysis/gameplay.md`; title START was pressed during animation and did not yet open selection. Broad gameplay remains active.
- Input-contract correction (NES-001): initial shared-protocol button/pressed routes were ignored by NES. Accepted routes use `buttons` full held-set arrays. Actual game selection and stage1 practice/fight now visited naturally; `accepted-start.py`, `practice.py`, `fight.py` preserve replay and exact timeouts.
- Practical progression shortcut established: restore `fight-action.snapshot.json`, release buttons, align one operation, write RAM $22 = zero-based stage, set PC $D014 / stack $FF. Effective PRG context [12,13,14,15]. D014 initialization refreshes dependent stage state, D03E copies $22 to $A1; D881 indexes records; ACDB selects quote. Tested artificially: stage5 FIFTH AVENUE and stage10 TENTH STREET practice; stage5 gameplay/action screen confirmed. Stage15 branch remains active in `round-001/later-stages.py`.
- Static text easy gains: bounded character alphabets, ending speech, menu strings, stage quote/name block classified using renderer/lookup consumers and inspected exact physical delimiters; `text-structure-review.json`, `fix-text-boundaries.py` document boundary refinements. Reachability of unvisited strings remains unresolved.
- Later-stage action refinement finished: after240f untraced settling, 50f right+A then30f up+B traced, stages5/10/15 have active gameplay and enemies; stage10/15 selected frames visually verified. Artificial-state fetch provenance retained separately from natural reachability.
- Static connected closure complete (+808 code bytes) and text-pointer/digit display structures classified. Ending dispatch test remains active; next step final classification refresh and checkpoint.

## Round 001 disassembly completed — incomplete ROM

ROM classification: 12.98% (17021 / 131088 bytes classified; 114067 bytes unknown).

Current authority: `analysis/state.json`, `maps/current.json`, verified checkpoint `maps/pass-001.json`, `generated/pass-001`, `analysis/pass-001-verification.json`. All prior active/frontier notes above are historical; no disassembly process remains active.

Classified code 10829, data 6192 (includes16-byte iNES header), unknown 114067. Observed code classification 9692 bytes, static code 1137 bytes. Observed includes explicitly artificial later-stage/ending experiments and does not imply natural reachability. Exact counts and unknown complement: `round-001/classification-counts.json`, `round-001/unknown-complement.json`. 5071 regions all carry evidence; semantic labels/readable routine commentary remain for commentary phase. Execution coverage is partial and distinct from classification.

| PRG 8KiB bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 861 | 0 | 7331 |
| 1 | 0 | 96 | 8096 |
| 2 | 0 | 0 | 8192 |
| 3 | 0 | 125 | 8067 |
| 4 | 0 | 0 | 8192 |
| 5 | 0 | 812 | 7380 |
| 6 | 0 | 0 | 8192 |
| 7 | 0 | 133 | 8059 |
| 8 | 0 | 0 | 8192 |
| 9 | 0 | 390 | 7802 |
| 10 | 0 | 0 | 8192 |
| 11 | 0 | 1119 | 7073 |
| 12 | 657 | 90 | 7445 |
| 13 | 1240 | 2447 | 4505 |
| 14 | 5765 | 149 | 2278 |
| 15 | 2306 | 815 | 5071 |


Verified checkpoint: byte_identical=true, ok=true, ROM SHA256 be2714f072338f2ca7204be750eaa03daadeb12521e1713cfe54d35b65457700; map SHA256 de7e76a6020c20ccef358832942b43f6196edf82499e8ab77dcbddcd0bc83945.

Gameplay: natural menu/stage1 practice/fighting; artificial stage5/10/15 practice and active enemies/attacks; artificial ending car/money animation and START exit. See `scenarios.json`, `gameplay.md`, and `round-001-disassembly-handoff.md`. Boss-specific triggers and natural stage-clear scoring were deferred with a concrete missing contract; repeated title/action play no longer yields substantial easy gains. Static connected closure gained808 then112 code bytes.

Toolkit findings: one confirmed protocol documentation/validation bug NES-001, no suspected bugs/missing features, no blocker. See `toolkit-issues.md`. Workaround buttons full-array validated. Configured toolkit commit a942a38 used unchanged. Disk handoff:6.6GiB free, scratch24KiB; cited snapshots remain local. No archive/publish/cleanup/toolkit edits.

## Coordinator boundary: round 001 commentary

Disassembly handoff accepted after inspecting exact current state/map intervals, evidence presence, checkpoint map identity and byte-identical report. Classified17021 / total131088; unknown114067. Percentage printed to user before commentary. Current phase: commentary; input checkpoint pass-001. Consumed disassembly handoff; open R001-STREAMS, R001-ASSETS and R001-BOSS-CLEAR remain for redirect after full current-source readability. No competing phase writer active.

## Round 001 commentary completed — readability only

Full current-source readability completed for10829 code +6192 data bytes, with unknown114067 bytes explicitly unresolved.235 meaningful semantic units and383 bank-scoped branch aliases; all5157 scaffold entries linked to reviewed source. Source/reading copy and fresh byte-identical verification retained in commentary/. Input/current maps/state and frozen evidence unchanged. Whole ROM remains12.98% classified and final editorial audit remains pending. Details: round-001-commentary-handoff.md, commentary/progress.md and commentary/uncertainties.md. New R001-BRK-CONTEXT lead preserves observed BRK/table-use ambiguity at physical1D19B for redirect. Existing NES-001 remains the sole confirmed toolkit finding, no blocker, no new issue. No toolkit edit, cleanup, archive or publication.

## Coordinator boundary: round 001 redirect

Accepted full current-source readability after substantive source spot checks, exact scaffold identity/source-location checks, all10829 classified code bytes covered by semantic-unit ranges, frozen map/state identity, independently repeated byte-identical rebuild, and refreshed reading-copy equality. Evidence: round-001-commentary-coordinator-check.json. Current phase redirect; input pass-001 / commentary source c5b3798e82718eed19827dab9d47efd3ab9eed533b59a395f3a6ea9a3927ecc6. Final editorial audit remains pending. Open STREAMS/ASSETS/BOSS-CLEAR and BRK-CONTEXT must be investigated as one supported batch before deciding another round or terminal exhaustion.

## Round001 redirect research checkpoint (active)
Same pass-001 map/source preserved. Research in `redirect/r001-connected-20261112/`; detailed recoverable frontier there. Fifteen exact stage streams/12handlers, sound sequences/patterns/directories, raw sprite bitplanes, graphics metadata and artificial stage6/12 boss/clear scoring progression investigated. Corrected BRK premise against primary map: Static erroneous fallthrough, not Observed; proposed correction pending independent disassembly. Resource mapped-window crossings/overlaps are scoped, not blindly classified. No live process at checkpoint; proposals/unknown ledger and combined handoff still being prepared. Useful research continues; no exhaustion claim or map/source/toolkit edit.

## Round001 redirect completed — ready combined proposal, not classification
Research `redirect/r001-connected-20261112/` now has coherent stage/assets/background/render/property/sound/objectVM/native/scoring/UI/peripheral proposals.2971entries,114060newbytes proposed (3158code/110902data), oneStaticCode->Data correction atphysical1D19B. Hypothetical residual7; accepted map still17021classified/114067unknown, unchanged SHA de7e76a6020c20ccef358832942b43f6196edf82499e8ab77dcbddcd0bc83945. No map/source/checkpoint/toolkit mutation. Caption boundary corrected to15x16bytesF349..F439 after independent audit; all Data/Data and code/data conflicts absent. Exact ledger86inputunknownintervals and per-bank accepted/proposed counts retained. Ready leads and gameplay scenarios updated with histories. Handoff `round-001-redirect-handoff.md`; independent disassembly next, then fullcurrent-source commentary/redirect. No terminal exhaustion claim or rejectionreview requirement. Current-source readability gate/final editorial pending status unchanged. ExistingNES-001 soleconfirmedtoolkitissue; no blocker/newfinding. No live process; ~6.5GiB free, scratch24KiB, local research~47MiB; no cleanup/archive/publication.

## Coordinator boundary: round 002 disassembly

Round001 redirect handoff accepted as coherent ready batch after inspecting scope, productive connected/independent avenues, exact proposal mechanics and corrected caption-boundary evidence. All2971 proposal intervals nonoverlapping;114060 unique new proposed bytes,7 projected residuals; single intentional staticCode->Data correction. Evidence: round-001-redirect-package-check.json. No terminal exhaustion accepted. Current phase round002 disassembly; input pass-001 and round-001-redirect-handoff.md. Accepted classification remains17021/131088,114067unknown until independent review/annotation. All ready lead IDs/recipes reside in leads.json; current readable source/frozen history must be preserved for later commentary update.

## Round002 disassembly completed — incomplete ROM

ROM classification: 99.99% (131081 / 131088 bytes classified; 7 bytes unknown).

Accepted114060new bytes (3158code/110902data), plus one prior StaticCode byte corrected toData. Total code13986/data117095 (including16header bytes); Observed code9692, Static code4294. Structural coverage and execution are separate; all new classifications Static. Exact primary review: `round-002/review.md`, `round-002/primary-review.json`, `round-002/mixed-source-review.json`; annotation journal and precise count/complement files in same directory.

| PRG 8KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 888 | 7304 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 1454 | 6737 | 8191 | 1 |
| 13 | 1901 | 6290 | 8191 | 1 |
| 14 | 7002 | 1188 | 8190 | 2 |
| 15 | 2741 | 5448 | 8189 | 3 |

State/map SHA256 `d2310ef19eec8c395e3be0f6ba1377a1511f35e9223a5cc0e7792067765808df`. `map.validate` passed; new `maps/pass-002.json`, `generated/pass-002`, `pass-002-verification.json` verify byte-identical ROM rebuild. Existing commentary/source SHA256 remains `c5b3798e82718eed19827dab9d47efd3ab9eed533b59a395f3a6ea9a3927ecc6`; current-source commentary refresh belongs to next phase. No new runtime replay; prior representative progression/boss/score/ending evidence reused. Seven unknown bytes at CPU805A,AC1F,DBA3,DDB5,F822..F824 retained; no whole-ROM completion/terminal exhaustion claim. All8ready lead outcomes verified with structural/reachability limits, new R002-RESIDUAL-7 records next question.

Toolkit: no new findings; existing confirmed NES-001 only, zero suspected/missing-feature findings, no blocker. See [toolkit-issues.md](toolkit-issues.md). Configured binary unchanged; no toolkit edits. ROM and historical evidence preserved; no publication/archive/cleanup.

## Coordinator boundary: round 002 commentary

Accepted round002 disassembly handoff after checking exact nonoverlapping state coverage/evidence, current/checkpoint map identity and byte-identical report, plus independent primary-review/preservation summaries. ROM classification:99.99% (131081/131088;7unknown) printed to user before commentary. Input pass-002 SHA d2310ef19eec8c395e3be0f6ba1377a1511f35e9223a5cc0e7792067765808df. Current phase commentary: prior pass001 review credit must be preserved/reused where valid and expanded current source fully explained before redirect. No whole-ROM or terminal completion accepted.

## Coordinator boundary: round002 redirect

Accepted full current-source commentary gate: checked substantive new code/data/VM examples, exact6724-entry scaffold and source locations, all13986code bytes covered by semantic units and all classified data regions carrying source-linked reviews (old6192data bytes retain prior region-level credit), reading/source equality, unchanged map/state, preserved prior source hash, independently repeated byte-identical rebuild. Evidence: round-002-commentary-coordinator-check.json. Current phase redirect; input pass-002 and round-002-commentary-handoff.md. Exact seven residual bytes and corrected inferences must be investigated under evidence-policy.md before terminal assessment. Final editorial/verification acceptance pending.

## Coordinator boundary: round003 disassembly

Accepted round002 redirect as coherent four-byte ready batch, not terminal exhaustion. Inspected localized generic-data rationale, primary dispatch/boundary extracts, exact two unknown intervals and independent scope-correct threeRTS rejection review. Input pass-002 unchanged; new ready IDs R002-STAGE-SEPARATOR-DATA and R002-LOCAL-VM-DATA. Current phase disassembly; seven stale scope comments in redirect/r002-residual-20261113/editorial-backlog.json reserved for following commentary. No broad retry of unchanged investigation or final completion accepted.

## Round003 disassembly completed — incomplete ROM

ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown).

Rounded100.00% does not establish completion: unknown3 bytes remain. Accepted4Static genericData bytes atphysical1806A length1 and1F832 length3. Totals13986code/117099data (includes16header), Observedcode9692/Staticcode4294 unchanged. No new execution coverage or reachability claim. Independent evidence and alternatives: round-003/review.md and apply.py; all prior regions/metadata preserved.

| PRG 8KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 888 | 7304 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 1454 | 6738 | 8192 | 0 |
| 13 | 1901 | 6290 | 8191 | 1 |
| 14 | 7002 | 1188 | 8190 | 2 |
| 15 | 2741 | 5451 | 8192 | 0 |

Unknown exactphysical1AC2F/1DBB3/1DDC5, CPUAC1F/DBA3/DDB5, each60 under[12,13,14,15]. Counts/complement: round-003/classification-counts.json and unknown-complement.json. State/current/pass-003 mapSHA `67db4c98e95d432b6caf57006f381944c440f7c1a726f9fda64957ee07acd308`; validation passed, fresh generated/pass-003 and pass-003-verification.json confirm byte-identical rebuild. Source/commentarySHA f6689b39eaf8419ac688c9df372283aa250160349091cb3e616a2bcbc725df4b preserved; next commentary must integrate4bytes and seven stale-scope editorial notes before later redirect/editorial assessment. Prior gameplay/static research reused without unchanged replay. Both ready lead outcomes verified; residual lead partially verified with3unknown remaining. No terminal exhaustion claim.

Toolkit: existing confirmedNES-001 only, zero suspected/missingfeatures, no blocker/new finding; [toolkit-issues.md](toolkit-issues.md). Configured binary/ROM unchanged; no toolkit edits/publication/archive/cleanup. Disk6.4GiBfree, scratch24KiB; durable historical evidence preserved.

## Coordinator boundary: round003 commentary

Accepted independent four-byte review/annotations after exact current counts/evidence/nonoverlap and checkpoint map/byte-identical verification checks. Printed ROM classification100.00% (131085/131088;3unknown) and explicitly incomplete despite rounding. Current phase commentary; input pass-003 SHA67db4c98e95d432b6caf57006f381944c440f7c1a726f9fda64957ee07acd308. Preserve/reuse prior readability, integrate4data bytes and resolve seven stale source claims in redirect/r002-residual-20261113/editorial-backlog.json before next redirect. Final editorial acceptance still pending.

## Coordinator boundary: round003 redirect

Current-source readability accepted after checking scope-correction/new-data records, full6724-entry source-linked scaffold, all13986code-byte unit coverage/412units, reading equality, frozen current map/state and preserved pass002source, plus independently repeated byte-identical rebuild. Evidence: round-003-commentary-coordinator-check.json. Current phase redirect, pass-003; exact three unknownRTS remain. Final editorial note: replace conditional unused-return wording with potential callable/no-op return so absence of references cannot imply unusedness. Final editorial audit and terminal rejection-review acceptance still pending.

## Round003 redirect rejection review — coordinator acceptance pending

No ready structural proposal; exact residual3 unchanged. Primary-backed current singleton contracts and scope-correct reuse of prior reference evidence: redirect/r003-terminal-review-20261113/results.md and primary-check.json. All viable code/data alternatives remain materially ambiguous; no concrete improved discriminator or productive frontier identified. Terminal incomplete proposed, subject to coordinator independent reasoning inspection. Full handoff: round-003-redirect-handoff.md. Final editorial audit remains pending; conditional unused-return phrasing requires potential callable/no-op wording. No classification/source/toolkit/runtime changes or unchanged gameplay replay.

## Coordinator terminal structural acceptance — incomplete

Inspected exact claims/alternatives/negative scope and independently checked decisive primary ROM intervals for all three residualRTS bytes; no corrected discriminator or concrete productive next test found. Accepted terminal structural incomplete under evidence-policy.md; not blocked, not whole-ROM complete. Evidence: terminal-rejection-review.md and round-003-redirect-handoff.md. Current stage final editorial acceptance/verification; classification131085/131088,3unknown remains. No archive/cleanup/publication authorized.

## Final editorial acceptance — pass003

Targeted final review accepted; reused full readability credit, corrected residual wording, stale nested summaries/links and obsolete camera alias, normalized data units and canonical ranges. Exact411 named anchor proofs and6696 nested canonical matches: final-unit-anchor-check.json. Supported final commentary audit PASSED ok/byte_identical/editorial true,6724regions/6696nested units; independently checked refreshed reading equality. SourceSHA4387fdb27c6a55e603c41dc9baf16cf1a7a57bebfe774a8707dbbf6bf818243e; pass003 map/state unchanged. Structural131085/131088 (rounded100.00%),3unknown remains INCOMPLETE. See final-acceptance-handoff.md and commentary/editorial-audit.md. Historical412unit records contained one duplicate obsolete camera alias; current411distinct units. No toolkit/map changes, archive, publication or cleanup; NES-001 soleissue/no blocker. Coordinator final inventory audit remains separate.

## Final coordinator ledger and terminal handoff

Sequential separate worker contexts were used in this order; each retrieved only its phase prompt/checkpoint/compact handoff. Baseline ran once; all earlier evidence remains preserved.

| Round / phase | Input -> output | Lead IDs consumed / produced and outcome | Verification |
|---|---|---|---|
|001disassembly|initialized -> pass-001|Produced verified R001-INPUT,R001-PROGRESSION,R001-ENDING; open R001-STREAMS,R001-ASSETS,R001-BOSS-CLEAR|byte-identical pass-001|
|001commentary|pass-001 -> readable sourcec5b3798e|Produced R001-BRK-CONTEXT question; current-source readability complete|fresh verification + coordinator check|
|001redirect|pass-001 -> combined ready batch|Consumed/improved R001-STREAMS,R001-ASSETS,R001-BOSS-CLEAR,R001-BRK-CONTEXT; produced R001-SOUND,R001-FRAME-PROPERTIES,R001-OBJECT-VM,R001-RESIDUAL-CONTINUATIONS ready|primary research and independent proposal mechanics|
|002disassembly|pass-001 -> pass-002|Consumed all eight ready IDs above, verified; produced R002-RESIDUAL-7 open|byte-identical pass-002|
|002commentary|pass-002 -> readable sourcef6689b39|No new ready IDs; preserved residual question|fresh verify + coordinator check|
|002redirect|pass-002 -> four-byte ready batch|Consumed R002-RESIDUAL-7; produced R002-STAGE-SEPARATOR-DATA,R002-LOCAL-VM-DATA ready; three RTS deferred|primary role/boundary audit and localized rejection review|
|003disassembly|pass-002 -> pass-003|Consumed/verified both narrow ready IDs; residual parent partially verified|byte-identical pass-003|
|003commentary|pass-003 -> readable sourced2ed49b2|No new ready IDs; resolved seven stale editorial claims plus concrete consistency gaps|fresh verify + coordinator check|
|003redirect|pass-003 -> proposed terminal incomplete|Consumed remaining R002-RESIDUAL-7; no produced IDs/ready proposals|primary rejection review inspected independently by coordinator|
|finalacceptance|pass-003 -> source4387fdb2|Residual lead deferred, no ready investigation; exact reopening condition preserved|supported audit + independent byte-identical rebuild, exact anchors/inventory/reading checks|

ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown).

Rounded100.00% is INCOMPLETE. Final structural terminal-incomplete outcome accepted after primary reasoning review; final editorial/readability acceptance passes. No blocked condition or required unfinished investigation remains. Final handoff: final-handoff.md; source/map/document/requirement evidence: current-audit.json. Archive preparation selects all final conclusions and handoffs; only duplicate instruction documents omitted. No archive execution, cleanup, deletion, publication or toolkit change. Original ROM unchanged; source/assets/internal state local.
