# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Pass 001 in progress
Baseline established via run.py inspect/baseline; analysis/baseline-verification.json confirms byte identity. Mapper 1, PRG 131072, CHR 131072, total 262160. No prior leads. Active phase evidence: analysis/pass-001/rpc.jsonl; bounded startup/gameplay probe (180 frame requests, 30-second timeouts and 30M operation limits). Static frontier: reset $FFD8, NMI/IRQ $C000 in fixed physical banks 14/15. Classification not yet committed. Next: inspect frames, discover validated roots and observed contexts, annotate supported intervals, broader progression and checkpoint.

Initial classification batch: 133426 / 262160 physical bytes (50.89%), 128734 unknown. Header 16, CHR image 131072, vector data 6, code 2332 bytes in 1178 instructions. Observed code from 951 contextual roots over three intro frames, static continuation from validated vectors/observed contracts. Observed aggregate dropped=0; detail capacity=0 intentionally drops detail. Checkpoint pass-001-initial byte-identical verified. Gameplay identified visually as Adventures in the Magic Kingdom; name entry and castle introduction visited, START bypassed dialogue into park, explored left/up paths. Broad progression continues; later attractions and ending unvisited. Evidence and scripts in analysis/pass-001; manual controls https://www.world-of-nintendo.com/manuals/nes/adventures_magic_kingdom.shtml .

Connected static direct-call expansion added 141 code bytes (expand.py and expand-*.json); map/state exported and validated. Current classified 133567, unknown 128593. Coordinate entrance shortcut successfully triggers five distinct attraction instruction screens (Pirates index1, Haunted2, Autopia3, index4 pending frame identification, Space7). START did not dismiss instruction screens; corrected input route to four distinct A press/release cycles and representative mechanic input, running next bounded batch. Exact patch sets and clean snapshot retained. Stage gameplay and ending not yet claimed visited.

Merged broad gameplay batch: 154835 / 262160 bytes classified (59.06%), 107325 unknown; code 9655 bytes, generic uniform data 14086, header/vectors22, CHR131072. Preserved per-fetch effective contexts and artificial state marker. All five distinct attractions exercised: Pirates1, Big Thunder train2, Autopia3, Haunted4, Space7. Screens in play-1/2/3/4.png and space-play.png show actual gameplay; 90-120 frame representative mechanics inputs. Ending shortcut (exact index6 entrance plus RAM $06D9=6) successfully reached parade dialogue (ending.png); natural six-key completion not asserted. Replay scripts and durable snapshots retained. Uniform zero maximal intervals classified as generic Data after pattern/boundary/code conflict inspection, without claiming fill purpose or semantic layout. map.validate passed; pass-001-gameplay checkpoint byte-identical verified. Next bounded gains: trace supported scene transitions to capture initialization code missed by untraced broad progression, terminal parade and SELECT menu; no exhaustive gameplay planned. No toolkit issues found.

Final pass consolidation in progress: targeted transition/menu/terminal additions plus exact structure/replica batch reached157308 classified. Additional maximal repeated generic data in PRG2..9 added3924; connected direct-call closure added860 =>162092 classified /100068 unknown. Checkpoint pass-001 verified byte-identical (3346e5b0999c421a4ee7d8b174c5bd99655927225c7c73b2ad4622478cc9d8a6 map hash). Final Autopia UP+A300 mechanic confirmed to advance past starting grid; its short trace is merging now. No further experiments planned; finalize connected call closure and fresh final checkpoint/counts/handoff. Terminal End screen confirmed. document.py contains pending durable scenarios/lead/per-bank/handoff consolidation.

## Pass 001 complete: incomplete ROM classification

ROM classification: 62.53% (163919 / 262160 bytes classified; 98241 bytes unknown).

Code 14429; Data 18418; CHR Image 131072. Header16 and all CHR131072 classified; remaining unknown complement is entirely PRG. Unique physical intervals, no raw fallback counted. Counts and exact unknown complement: pass-001/counts.json.

| Physical 8KiB PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 1805 | 35 | 1840 | 6352 |
| 1 | 7 | 2569 | 2576 | 5616 |
| 2 | 0 | 1573 | 1573 | 6619 |
| 3 | 7 | 1306 | 1313 | 6879 |
| 4 | 0 | 1218 | 1218 | 6974 |
| 5 | 7 | 1153 | 1160 | 7032 |
| 6 | 0 | 854 | 854 | 7338 |
| 7 | 7 | 2100 | 2107 | 6085 |
| 8 | 0 | 1233 | 1233 | 6959 |
| 9 | 7 | 1867 | 1874 | 6318 |
| 10 | 302 | 0 | 302 | 7890 |
| 11 | 317 | 2983 | 3300 | 4892 |
| 12 | 1670 | 0 | 1670 | 6522 |
| 13 | 2691 | 224 | 2915 | 5277 |
| 14 | 3998 | 32 | 4030 | 4162 |
| 15 | 3611 | 1255 | 4866 | 3326 |

All annotations include structural evidence and provenance; code includes explicit CPU execution views. Observed and Static remain separate. Execution coverage is a subset of code, not whole-ROM classification; runtime roots and artificial flags retained in *contexts.json. Bounded detail buffers intentionally capacity0 (detail drops), all cited trace summaries aggregate_dropped=0; no dropped aggregate evidence treated as observed.

Broad progression: normal startup/name/castle/hub/SELECT; supported artificial entrances into all five attractions, representative mechanics; artificial six-key parade and final End screen. Full objective wins, Haunted boss and trivia chain remain deferred with missing RAM/script contracts in scenarios.json/L004. Repetitive hub movement stopped at geometry barrier; corrected dialogue A route succeeded. Initial traced180frame request timed out and was replaced with untraced broad progression and1-3frame captures.

Static gains include validated vectors, accepted fetches, connected control flow/direct-call entries, exact six16-byte entrance arrays, inline scene dispatch table, replicated reset wrappers/trailers and explicit generic repeated-data slices after boundary/conflict inspection. Direct-call closure stopped naturally when no new entry roots remained, not a byte ceiling. Unknown unique interstices and inline/bank-return barriers need redirect investigation L003/L004; this is an ordinary easy-gain handoff, not terminal exhaustion.

Verification: map.validate passed; maps/current.json exported from analysis/state.json; run.py checkpoint pass-001-final validated/generated/rebuilt byte-identically, report analysis/pass-001-final-verification.json. Input SHA-256 remains workspace identity. No toolkit issues found; [toolkit issues](toolkit-issues.md). Evidence retained locally, no toolkit modifications or publication. Disk145MB phase evidence, scratch4KiB,9.8GB free. Final handoff: pass-001/handoff.md.

Final execution-evidence split: Observed code 7175 bytes in 3539 physical instruction annotations; Static code 7254 bytes in 3477 instruction annotations. These are map provenance counts, separate from contextual fetch/hit counts; artificial scenario evidence remains marked in saved contexts.

## Pass 001 commentary active

Prepared commentary frozen baseline from pass-001-final; preparation rebuild byte-identical. Current source in commentary/source/game.asm. Full readability scope 14429 code /18418 data /131072 CHR plus notes on98241 unknown; initial remaining frontier all current source. Durable frontier and review in commentary/progress.md and review.json. Final editorial audit remains reserved for terminal acceptance.

## Pass 001 commentary complete for current source

Full readability scope complete: all7016 instruction regions/14429 code bytes,168 data regions/18418 bytes,131072 CHR bytes and376 unknown intervals.262 functional code units plus four header/table/CHR structures; stable globals, CPU aliases, actor fields, sound fields, tables and branch paths named. Uncertain scene scratch/actor/tune identities and unknown gaps remain honest mechanical explanations. Current source review credit preserved in commentary/review.json; no pending readability range. Classification unchanged163919/262160 (62.53%),98241 unknown. Editorial audit pending for terminal acceptance and not run.

Fresh commentary verification byte-identical; source game.asm SHA2561774e416228a4cac5bca2337c7870c915975409f40c332017bca454a03ffe1b9 and symbols.inc6824b6da6ccd2a4d66ed21ce0e8121a20bdd067b3abbba4b7b7b3eeff84fed94; reading-after.asm refreshed afterward. Source/verification/coverage/uncertainties: commentary/. Compact handoff commentary/handoff.md. L004 inline helper mechanics narrowed: C317 skips a two-byte caller argument; D164 uploads that pointed stream; C3AF directly fills nametable from A/X/Y and consumes no inline bytes. L003/L004 remain redirect questions; no new classification performed. No toolkit issues found; no toolkit edits/archive/publication/deletion.

## Redirect r001 active

Attempt analysis/redirect/r001-inline-structure starts from frozen pass-001-final map e3cfeb964dfe48b53bd62f4f140a79b8021d3b79b646941fc9312cdfb9fd7102, current-source readability complete; no classification edits. 163919 classified /98241 unknown. L004 first batch scans all classified D164 callsites: five inline pointer words recovered; four point to RAM $0300, one points to physical95026 bounded190-byte PPU stream. Bank12 CPU8000 entry/inline calls remain a separate static entry-contract investigation. Next: inline-aware continuation closure, other C317 wrappers and data-bank format review; exact unknown ledger/dispositions being preserved. 9.8GB free, scratch4KiB. Research proposals only.

Redirect r001 batch checkpoint: compositional return-contract walk now proposes2429 code bytes (research only), ten inline words, three terminated PPU streams and two sprite stream pointers. Exact per-instruction contexts, summary decisions and blocked call frontiers saved under analysis/redirect/r001-inline-structure. Independent sound pass recovered bounded inline handler lists (8 control,5 channel fields,25 effect commands,5 envelope states),192-byte note-period read domain and64-entry request pointer table with explicit count. Invalid arbitrary command/state values remain outside the supported calling preconditions; no natural-use claim. Next independent promising avenue: D714 banked map layout/block consumers and paired pointer extents, then consolidate/conflict-check proposals and exact unknown ledger. Map/state/source unchanged.

Redirect r001 connected L003 result:227*72-byte map block container spans banks4/5 up to preserved reset wrapper;14006 currently unknown bytes supported as data. Paired227-entry low/high arrays and56-entry layout pointer arrays independently bounded;56*32 layout bytes all index the227-record domain (689 additional unknown bytes). Connected seven dictionary bases/consumers under review. C21F review recovered4-entry row-stream,4-entry map-stream and12-entry driving-state tables with independent endpoint/domain caveats. Bank5 explicit service interface and existing scene16-word table now supply supported roots for bounded continuation research. Exact proposals and primary decoder evidence in attempt directory. Counts remain frozen98241 unknown; proposals unaccepted.

Redirect r001 further connected findings: seven map dictionaries (23296 bytes total before overlap) constrained jointly by independent quadrant/bank/tile/property tables; all use2048-byte quadrant IDs,1024 tile-plane bytes and256 property bytes. Actor bank4 consumer review recovered281 valid frame records,182 animation records and106 initializer field records, associated pointer lists and coordinate data. Counterexamples localized (reserved9DA6 frame target, unflippable coordinate143, zero-count initializer AE6D); no overbroad full-table semantic assertion. Latest code walk tracks explicit MMC1 bank-return contracts, scene/service remembered-bank preconditions and all recovered dispatch handlers; current research artifacts remain unaccepted. Full qualification/frontier in attempt plan.md. Frozen98241 unknown unchanged.

Redirect r001 sound/palette checkpoint: explicit consumer grammars recover bounded8BFB..AECC sound bytecode collection (6287 records, no unresolved parse targets; generic small tails distinguished),47 eight-byte instruments and104-entry palette pointer pairs with87 complete bounded records. Localized counterexamples corrected the stop-byte prefetch and palette count112→104; reserved zero-run targets excluded from record claims. Exact primary evidence/recipes saved. Consolidating inline PPU/sprite assets, exact unknown ledger, code/data conflicts and lead histories next; still research only and map/source unchanged.

Redirect r001 latest: additional text consumer CBEB/C317 resolves logical1 text selector table;448 entries bounded by first text9DC0,155 terminated text scripts and9 escaped-code references (2 distinct bodies). Logical3 dictionary pointer pair128 entries plus56 terminated words recovered; reserved zero-tail targets excluded. All88 actor behavior entries now feed connected static research under synthetic return-frame contract. Exact NPC trivia state/answer/key contracts recovered and bounded contact experiment active. Haunted source-coordinate attempt with UP did not transition; corrected direction is DOWN=$04 per existing symbols and EFEB consumer, separate preserved retry active. No boss/question outcome claimed without frame/state evidence. Map/source still frozen.

Redirect r001 confirmed progression/consolidation: Haunted-specific ED95 gate (not prior EEE5 platform table) yielded initialized interior floor5 via exact source-supported UP coordinate shortcut;90-frame mechanic exercised and interior images saved. Master Spectre remains unidentified. Real trivia menu answered correctly at first quest stage; final-NPC state shortcut then answered questions30/32 correctly, quest progress38→39→3A and key0→1. Exact patches, screenshots, snapshots and accepted answer traces saved; artificial stages, not natural full chain. Current union81085 proposed unknown bytes (71653 data,9432 code); hypothetical residual17156, actualmap remains98241 unknown. Consolidation reports zero instruction/code/data conflicts. Next exact residual ledger/triage and combined independent disassembly recipe; no toolkit/source/map edits.

Redirect r001 Space/display batch: bounded display sequences/frame arrays/sprite streams partition A559..B0CD;30 terminated additional PPU programs partition8FEB..9A0D between independent consumed boundaries. Space four160-byte fixed-point planes, full305-record command script endingF8E5, finite target/rate/progress domains and prompt/reserved/feedback tables recovered with writer/consumer proofs. Consolidation refreshed next. Remaining-method review corrects universal callee-return requirement: retain positive balanced, bank-preserving return witnesses under explicit calling preconditions; unresolved alternative exits do not reject a valid structural continuation. Old conservative walker preserved, witness-producing revision active before final ledger/triage. No classifications changed.

Redirect r001 closure/remaining review: last consolidated93318 proposed unique bytes (80456 data,12862 code),4923 hypothetical residual, zero structural conflicts. Positive return witnesses retained; explicit FF8B/FF96 bank-save wrappers and accepted Haunted/trivia artificial fetch roots extend the batch. Partial Haunted context page marked partial; complete trivia pages/aggregate counters preserved. Remaining primary review recovered exact829-byte driving spawn container and finite actor velocity/name-grid/title pointer arrays, strengthening prior generic program claims with real selectors. ProbeActorMap's already documented return6 and scene reentry contracts now close the remaining connected code paths; final validator/ledger/lead packaging follows these concrete checks. Actual map/state/source unchanged.

Redirect r001 latest union95561 proposed bytes (80986 data,14575 code),2680 hypothetical residual; actual98241 unknown unchanged. Return summaries now carry remembered RAM43 alongside bank context; reviewed scheduler/render/frame-delay/map-upload contracts close matching continuations. Late526-byte table batch covers exact name-grid, title program pointers, actor arcs and six platform-door records. Zero alignment/kind conflicts. Final bounded residual scrutiny checks meaningful standalone C669/C26F/964A contracts without demanding live callers; then exact ledger/validator/combined lead handoff. No map/source edits.

## Redirect r001 ready for independent disassembly

Combined handoff: analysis/redirect/r001-inline-structure/handoff.md. Core research96381 unique previously unknown bytes:81427 Data +14954 Code candidates. Accepted0; actual map remains163919/262160 (62.53%) classified and98241 unknown. If every core claim accepted,1860 bytes remain in83 local pieces. Exact ledger recomputed from frozen map with physical8KiB identities, CPU aliases/contexts, classified neighbors, proposal links and residual dispositions. L008 provides up to286 additional conditional Data bytes; F3CA/BE36 conditional continuation recipes are included in the same batch, outside core counts. This is a ready handoff, not terminal exhaustion.

All given and newly connected promising graphs were investigated as one phase: map/actor/text/word containers, sound/palette, title/status/display/spawns, full Space script/curves/prompts, all16 scenes/88 actor targets and inline/bank/stack continuations. New D172 queued-Ppu skip2/count distinction and standalone C26E math/964A/9716 transition contracts preserved. No classification or source commentary edits performed.

Independent validator28 checks passes; zero core instruction-alignment/Code-Data conflicts. Two qualified warnings concern CBB5 writing text buffer0180..01DF: live stack exclusion required. CFG return-route sketches omit loop backedges and are not concrete executions or universal return proofs; branch/loop/stack/43/44 coherence gates explicit in manual-contracts.md. Some instructions validly span adjacent physical8KiB slots, checked byte-by-byte. Observed artificial fetch roots remain separate, with partial Haunted page limits and complete trivia capture counts recorded.

Haunted initialized floor5 interior and90-frame mechanics visited; named boss remains unproved. Real trivia menus and first/final quest stages exercised; final progress38→39→3A, key0→1 via documented artificial state. scenarios.json updated with prior outcomes retained. No claim of ordinary full quest/all53 questions or full hardware/game fidelity.

Persistent leads L003/L004 updated; L005/L006/L008 ready; L007 verified; L001/L002 histories retained. Exact proposals/results, compact rejection review, conditional residual recipes and final hashes are in attempt directory. ROM/map/state/source unchanged; map/state both hash e3cfeb964dfe48b53bd62f4f140a79b8021d3b79b646941fc9312cdfb9fd7102. Attempt≈98MiB, free≈9.6GiB, scratch0 file bytes. No toolkit edits, archive, publication, deletion or commit. Next coordinator action: inspect this handoff/qualifications, then independently validate/annotate combined batch and complete full current-source commentary afterward.

## Pass 002 disassembly active

Input pass-001-final/current SHA256 e3cfeb964dfe48b53bd62f4f140a79b8021d3b79b646941fc9312cdfb9fd7102; baseline preserved. Frontier: independent review of redirect r001 core data/code clusters, loop/stack/bank preconditions, L008 and localized F3CA/BE36 continuations. Initial163919 classified/98241 unknown. Review records and acceptance scripts in analysis/pass-002/. Prior gameplay/ending snapshots reused; no unchanged capture replay planned. Prior commentary preserved.

Pass002 incremental Data saved:81427 new Static Data bytes after independent cluster review;245346/262160 classified,16814 unknown. State/current exported and map validation passed; exact accepted slices/data groups in pass-002/data-accepted.json and data-group-counts.json. Core Code and conditional residual review remains active.

Pass002 core and conditional batches saved:260748/262160 classified,1412 unknown (99.46%).96381 core bytes plus448 conditional bytes accepted, all Static. L008 all286 bytes accepted with code prefixes excluded; F3CA44code/F3F6 directional40data/BE3652code/CE2A26code accepted. map.validate passed; checkpoint pass-002-reviewed rebuilt byte-identically (mapSHA256 ef2e9a03f900472f130139d5733274e4601c0d6ab0c22e2d95663188a1881caf). Current frontier: bounded positive-pattern residual tables and small companion helpers; preserve uncertain prefixes/call/stack states.

## Pass 002 disassembly result

Incomplete; ordinary easy-gain handoff, not terminal exhaustion. Accepted97584 new bytes:15318 Code and82266 Data, all Static. Existing classifications/provenance retained. Core96381 accepted, conditional448 accepted, tail698 net (one-byte correction), title57 accepted. Exact review, proposal/accepted slices and correction history: pass-002/review.md.

ROM classification: 99.75% (261503 / 262160 bytes classified; 657 bytes unknown).

| PRG8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 2186 | 6006 | 8192 | 0 |
| 1 | 7 | 8185 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 7 | 8185 | 8192 | 0 |
| 10 | 1384 | 6760 | 8144 | 48 |
| 11 | 1528 | 6652 | 8180 | 12 |
| 12 | 5679 | 2485 | 8164 | 28 |
| 13 | 6759 | 1325 | 8084 | 108 |
| 14 | 6720 | 1113 | 7833 | 359 |
| 15 | 5456 | 2634 | 8090 | 102 |

Header16/16 and CHR131072/131072 classified; remaining unknown is PRG only. Code provenance7175 Observed /22572 Static; no new runtime observations. Prior bounded gameplay/snapshots reused: all five systems/ending, Haunted interior and first/final trivia key verified artificially. Named boss activation/natural full quest unresolved; no unchanged replay.

Final state/current map agree; map.validate passed. Authoritative fresh checkpoint maps/pass-002-final-corrected.json and analysis/pass-002-final-corrected-verification.json rebuild byte-identically. Earlier pass-002-final checkpoint superseded only for C9D6 boundary correction. Input SHA256 unchanged. Final exact unknown complement and per-bank counts: pass-002/final-unknown-ranges.json and final-counts.json. Current-source commentary preserved;15318 newly classified code bytes/82266 data bytes await full commentary phase; no editorial completion credit inferred.

LeadsL003/L004/L005/L006/L008 consumed with prior rejected/blocked history preserved; newL009 carries residual657 bytes for localized bank/stack/entry review. Strongest useful questions: C9D9/CDE6 bounded buffer operations; F8 escape synthetic returnCD42 and map-upload continuations; actor query pop-two return contractC669; final status/title and text-return fragments. No broad missing-caller requirement. No terminal rejection review claim.

No toolkit issues found; confirmed0/suspected0/missing features0/blockersnone. See [toolkit issues](toolkit-issues.md). No toolkit edits, publication, commits, archive or deletion. Compact handoff pass-002/handoff.md.

## Pass 002 commentary active

Entire prior commentary preserved locally at analysis/round-2/commentary-pass-001-preserved; changed baseline prepared and byte-identical. Full current-source readability pending. Active frontier and incremental batches: commentary/progress.md. No successful handoff or final audit claimed.

## Pass 002 commentary complete for current source

Full readability complete for corrected baseline d39393ad520c8deeaf9c4a4502f451cb26f984edcea7e16818f349c0093f2557:14068 instructions/29747 code,100684 data,131072 CHR and explained657 unknown bytes in39 intervals.399 functional code units plus207 named data structures, all14931 scaffold records reviewed, no pending readability ranges. Preserved entire prior commentary at analysis/round-2/commentary-pass-001-preserved before preparation. Concrete older health/life, sprite/animation/initializer count, mapper-bank and obsolete-gap defects corrected; all newly classified systems integrated. Fresh byte-identical verification and reading refresh after final assembly edit. Final editorial audit pending, not run; ROM classification remains261503/262160 (99.75%) and incomplete. Compact handoff commentary/handoff.md; L009 localized questions in commentary/uncertainties.md. No map/state/ROM mutation, toolkit changes, archive, deletion, publication or commits.

## Active redirect r002-local-contracts

Frozen input pass-002-final-corrected d39393ad520c8deeaf9c4a4502f451cb26f984edcea7e16818f349c0093f2557;261503 classified/657 unknown in39 intervals. Research only, map/state/source unchanged. Primary bounded decode/source index saved in analysis/redirect/r002-local-contracts. Current frontier: numeric/row buffer C9D9 and eight-row CDE6, text synthetic CD42/CD99, actor C669/C6B3 stack contracts, F969 code/data alias, FAB9 split, other supported residual continuations.

Redirect r002 checkpoint: primary bounded decodes, source-index/primary-source, actual selected text witnesses and findings saved. Combined proposals now656 bytes (623 Code/33 Data) in39 slices; classification not applied. Exact657-byte/39-interval ledger includes neighbors/bank/dispositions, leaves isolated9741 as1-byte ambiguity if all proposals accepted. Connected map/train, actor stack, text callback/escape, completion and title bootstrap paths investigated together. Awaiting final package/lead update; frozen map/source/ROM hashes rechecked unchanged.

Redirect r002-local-contracts complete as ready batch, not exhaustion. L009 updated;39 slices propose656bytes (623Code/33Data),0 accepted; currentmap unchanged657unknown. Validation checked exact boundaries/nonoverlap/complement and unchangedROM/map/source hashes. Handoff and localized rejection/competing-role review saved. Isolated9741 remains1-byte unknown if all proposals accepted; EB58/96D0 flagged for independent narrow review. All concrete finite-text/startup/map/actor/alias avenues completed or disposed with actual reasons; no new supported boss route. Next phase independent disassembly, then complete current-source commentary and redirect.

## Pass 003 disassembly result

Incomplete; ordinary one-pass handoff, not terminal exhaustion. Independently accepted651 bytes:618 Code/33 generic Data, all Static, in38 slices. Fresh exact decodes matched saved objects, while structural acceptance used actual local loop/branch/stack/bank/consumer evidence. Prior classifications and histories unchanged. See [review](pass-003/review.md), [accepted slices](pass-003/accepted.json), [decisions](pass-003/decisions.json).

ROM classification: 100.00% (262154 / 262160 bytes classified; 6 bytes unknown).

| PRG8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 2186 | 6006 | 8192 | 0 |
| 1 | 7 | 8185 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 7 | 8185 | 8192 | 0 |
| 10 | 1432 | 6760 | 8192 | 0 |
| 11 | 1540 | 6652 | 8192 | 0 |
| 12 | 5701 | 2490 | 8191 | 1 |
| 13 | 6867 | 1325 | 8192 | 0 |
| 14 | 7079 | 1113 | 8192 | 0 |
| 15 | 5525 | 2662 | 8187 | 5 |

Header16/16 and CHR131072/131072 classified. Exact unknowns: physical104273 (bank12 CPU9741), length1; physical125800..125804 (bank15 CPUEB58..EB5C), length5. EB58 rejection is localized to insufficient independent structural constraints for a tiny zero-store-return body; no universal caller prerequisite. 96D0 five-byte constants accepted as generic Data after indexed palette reader/pattern/boundary check, without claiming exact table layout. LeadL009 partially verified; remaining6 bytes carry into later redirect. No additional concrete supported connected interval found.

Structural annotations cover262154 bytes; all651 newly accepted bytes carry evidence and Code execution views. Code provenance7175 Observed/23190 Static bytes is structural provenance, not full executed-byte coverage; no new runtime capture or execution-coverage gain asserted. Reused all-five-systems/ending/Haunted-floor5/trivia-key evidence; natural full quest/named boss activation unresolved. No unchanged gameplay replay or fabricated spawn contract.

Validated internal/current maps agree byte-for-byte. Fresh checkpoint [map](../maps/pass-003-final.json), [byte-identical verification](pass-003-final-verification.json), [audit](pass-003/verification.json); ROM hash unchanged. Current map SHA256 1a19aa8e9e018537acb0ab6a0029aad96e563a60389982f3e2f0a734ecf1c775. All14892 preexisting region records preserved exactly. Current commentary unchanged; full commentary refresh for newly accepted618 Code/33 Data and whole-source gate comes next.

No toolkit issues found: confirmed0/suspected0/missing-feature0/blockersnone; [persistent report](toolkit-issues.md). No toolkit edits, archive, cleanup/deletion, publication or commits. Compact [handoff](pass-003/handoff.md).

## Pass 003 full current-source commentary completed

Full readability gate passed for current map/state1a19aa8e9e018537acb0ab6a0029aad96e563a60389982f3e2f0a734ecf1c775: all14932 scaffold records,645 meaningful units,30365 Code/100717 Data/131072 CHR bytes and explanatory notes for6 Unknown bytes. Preserved entire previous commentary at round-3/commentary-pass-002-preserved; carried14894 exact physical fragments, inspected all38 accepted slices and affected stack/bank/text dependencies. Zero pending readability ranges. Fresh final byte-identical verify and refreshed reading copy; [handoff](../commentary/handoff.md), [coverage](../commentary/coverage.json), [verification](../commentary/verification.json). Classification262154/262160 remains incomplete despite rounded100.00%. Next ordinary redirect reviews isolated9741 and tinyEB58; terminal editorial audit remains pending. No classification edits, toolkit changes, archive, cleanup/deletion, publication or commits.

Redirect r003 started pass003-final,262154classified/6unknown; frontier analysis/redirect/r003-residual-constraints/checkpoint.json. Research only.

## Redirect r003 residual constraints

Exactly one redirect completed from pass003-final/current-source readability gate. Classification262154/262160,6Unknown unchanged;0proposed/0accepted. Reviewed connected A3 collision/slope and9742 motion consumers, actual indirect/split/stack target constructors, text bank transform, residual subentries/RTSencoded forms and bounded indexed/copy alternatives. Primary evidence and localized competing-interpretation review: [rejection review](redirect/r003-residual-constraints/rejection-review.md); [findings](redirect/r003-residual-constraints/findings.md). No supported additional structural claim or currently credible discriminating method remains; terminal-incomplete recommendation awaits coordinator inspection, not self-accepted exhaustion.

Residuals physical104273/bank12CPU9741 length1(60), physical125800/bank15CPUEB58 length5(A90085A360). Exact complement independently rederived; ROM/current/frozen/state/source hashes unchanged. L009 history preserved and marked blocked/no ready recipe; [checkpoint](redirect/r003-residual-constraints/checkpoint.json), [ledger](redirect/r003-residual-constraints/unknown-ledger.json), [handoff](redirect/r003-residual-constraints/handoff.md). Existing representative five-system/ending/Haunted-floor5/trivia-key evidence reused; no new named-boss activation contract or natural fullquest assertion. Final editorial audit remains pending. Research/lead records only; no toolkit changes, runtime replay, source/map mutations, cleanup/deletion, archive, publication or commits.

Coordinator inspected r003 decisive primary artifacts and accepted localized structurally incomplete stopping: [review](coordinator-terminal-review.md). Final redirect checkpoint/handoff now records coordinator-reviewed status;6Unknown retained, no new proposal. Final editorial audit remains separate.

## Final commentary editorial acceptance

Final [editorial assessment](../commentary/editorial-audit.md) reviewed; [toolkit audit](../commentary/audit.json) passed with14932 reviewed scaffold entries and14577 nested unit fragments for645 distinct semantic units. Repaired scaffold-local unit references and source-relative locations; no source bytes changed. Fresh byte-identical verification/read refresh passed, hashes and toolkit reading expansion independently agree. Zero readability backlog;30365 Code/100717 Data/131072 CHR and6 explained Unknown. Coordinator accepted terminal structural incompleteness after r003;262154/262160 remains incomplete. [Compact final handoff](../commentary/handoff.md). Archive document inventory and coordinator final requirements remain separate; no archive executed. No classification edits, toolkit changes, cleanup/deletion, publication or commits.

## Coordinator terminal handoff

Sequential workflow reached justified terminal structural incompleteness after primary-artifact rejection review. Final editorial audit and byte-identical rebuild passed; zero readability backlog. ROM classification: 100.00% (262154 / 262160 bytes classified; 6 bytes unknown). Rounded percentage is not complete classification. Final handoff: final-handoff.md; exact machine checks: coordinator-completion-audit.json. archive-options.json accounts for final and historical Markdown under inspected archive.py contract; no archive/cleanup/publication/toolkit change performed.
