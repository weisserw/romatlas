# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 01 active frontier
Baseline verified; vector reset F9C7, NMI FAF3, IRQ C0C0 fixed banks14/15 validated and control-flow candidates accepted. Generic planar graphics banks0–3 and exact uniform runs accepted (50,829-byte current frontier). See round01/prg-planar-atlas.png and uniform-runs-review.json. Runtime intro opcode roots extend static coverage; bank6/7 context observed, RAM execution401/404 not treated as ROM origins. Root candidate lists remain hypotheses until exact decode and annotation. One indirect target at129665 unresolved.
Password UI established RAM C2..C9 letter encoding by UI experiment. Episode4 password PKKCNMPD accepted; distinct forest intro confirms transition. Episode state81=0 vs3 is a differential hypothesis only. Full episode1/4 dialogue/mechanics and episode2/3 accepted transition experiments remain underway. Runtime helper transport errors and missing id calls were rejected atomically; fixed strict helper and static backup preserve accepted work. Old background RPC bridges are superseded; only direct per-experiment subprocesses used. An earlier long dialogue helper exited143; preserve script and episode1.snapshot frontier without inferring toolkit fault.

Validated dispatch table at physical1FA8A..1FA93 inclusive. Exact extent: start129674 length10 (CPUFA7A..FA83), PRG selectors start129689 length5 (CPUFA89..FA8D), bounded scheduler0..4. Five handlers PCs9D18,A6A0,C4EF,E371,DACC. Effective bank8/9 for first2,12/13 for remaining fixed-bank handlers. Classified connected static graph after examining control/data flow; no pointer-only code inference. Handler4 deals with sound and RAM78 (music identifier), so do not misinterpret RAM78>=4 as ending trigger. Handler3 sets health7E=0, likely death sequence; also not a supported ending shortcut. Intro exit remains main-thread/coroutine/bytecode question, not scheduler task dispatch.

Correction to provisional task4 semantics: full DB03/DBC3 control flow and scene3 DCF4 show task4 is the scene controller, containing music initialization calls; RAM78 cannot be assigned sound-only semantics from F357 alone. Earlier sound-only interpretation was unsupported and is superseded. No RAM78 patch or classification based on it was made. Scene3 initializes episode/items/room then enters native per-episode intro AC89 in selector6 (physical12/13); shared cleanup ACEE has independent no-input contract: delay60, fade, zero85 actor bytes, RTS. Caller return DE13 remains on stack and post-intro code DE14 resets actor positions/room. A snapshot-backed artificial bypass now reconstructs the FEFB(bank=A=6)→ACEE return chain while retaining DE13 and upper scheduler frames; exact RAM/register edits logged in ep4-bypass-route.json. This tests mechanics only, not natural dialogue completion.

## Round 01 phase result

ROM classification: 48.19% (63165 / 131088 bytes classified; 67923 bytes unknown).

Exact counts/per-bank table and verification are authoritative in progress.md final section, round01/metrics.json and pass-001-verification.json. Natural versus artificial scenarios are separated in scenarios.json; four actual gameplay states visited with representative mechanics. Two ending tests did not activate scene4; neither establishes impossible/unused ending code. Preserve test scope: asynchronous request nonconsumption and synthetic-entry nonprogress do not reject scene4/6 structural contracts. Further testing requires identifying live coroutine return/activation state and observing accepted instruction fetches immediately after DB03 entry.

Easy-gain stopping rationale: vector/static call graphs and supported connected intro/scene structures accepted; obvious patterned non-code graphics/uniform data classified rather than left unknown for semantic reasons. Shared gameplay samples now show diminishing new code (episode1 +161 bytes, episode2/3 combined+394); final balanced fixed-helper continuation batch+104. Remaining dense banks4/5/10, bank8/9/12/13 data/code frontiers, post-call bank state and scene activation require consumer/source research. No terminal rejection review is claimed or needed for this first-pass handoff; redirect should investigate concrete leads thoroughly.

Transport correction: superseded file bridge was accidentally given overlapping clients; rejected missing-id annotations were corrected with strict direct RPC and static-state backup. Later runtime script's empty save was restored from backup before all accepted merges. Identical adjacent entries were compacted without crossing provenance/evidence/context/CPU continuity, with precompact-state.json retained. Final map validated and fresh checkpoint rebuilt byte-identically. Long helpers exiting143 retain partial route records; per-cycle snapshots and shorter samples preserve repeatable frontier.


## Round02 completed — disassembly pass, incomplete ROM

ROM classification: 84.54% (110826 / 131088 bytes classified; 20262 bytes unknown).

Accepted 42761 Static Data bytes and 4900 Static Code bytes after independent consumer/contract and exact bank/boundary review. Unique Code 17613, Data 93213; header16 Data included in physical total. 213 unknown intervals remain; exact complement in round02/unknown.json, metrics in round02/metrics.json.

| Physical PRG8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 1467 | 6199 | 7666 | 526 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 644 | 7448 | 8092 | 100 |
| 9 | 4262 | 653 | 4915 | 3277 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 283 | 7242 | 7525 | 667 |
| 12 | 1898 | 0 | 1898 | 6294 |
| 13 | 86 | 3979 | 4065 | 4127 |
| 14 | 3800 | 390 | 4190 | 4002 |
| 15 | 5173 | 1750 | 6923 | 1269 |

Output state analysis/state.json and maps/current.json validated; fresh output maps/pass-003.json, generated/pass-003 and analysis/pass-003-verification.json confirm ok=true, byte_identical=true. Input pass-001 retained. pass-002 is an additional byte-identical unchanged-input checkpoint accidentally generated after a rejected missing-id annotation; it is preserved as history, not claimed as round02 output. Annotation id and duplicate-label validation errors were atomic and corrected before accepted state save; no toolkit fault inferred.

Evidence: round02/review-notes.md, data-independent-review.json, code-independent-decodes.json and consumer-independent-decodes.json; primary redirect/round01-contracts-20261031 contracts preserved. All proposed Static bytes accepted; natural reachability remains separate. R1-DATA-CONSUMERS and R1-CALL-CONTEXT verified for the supported batch. Cached supported closures are already at their bounded fixed point; residual bank/return callbacks and mixed intervals need refreshed-source redirect research. No unchanged gameplay replay: four artificial episode gameplay routes and actual terminal THE END artifacts reused; distinct boss initialization/event contract remains unavailable. See scenarios.json and gameplay.md.

NES-DOC-001 workaround retained; NES-MAP-002 remains one-byte representation blocker at C0BF. See [toolkit-issues.md](toolkit-issues.md). No toolkit changes. Input ROM unchanged. Existing commentary/source/review history preserved; all-current commentary is now pending for newly accepted classifications, followed by redirect. No archive/publication/cleanup performed.


## Round03 completed — disassembly pass, incomplete ROM

ROM classification: 99.29% (130160 / 131088 bytes classified; 928 bytes unknown).

Independent primary contract/consumer review and222 fresh bounded decode groups accepted14237 new StaticCode bytes and5097 StaticData bytes. Corrected123 existingData bytes at93313..93436 toCode using explicit selector5/E514 FEFB/E519 AC71 native caller and62-instruction leaf endingRTS ACEB; seven-byte records/four actor slots. Split only formerData3910; surrounding resources and ACEC directory remainData with prior evidence. Zero classification gain from correction. UniqueCode31973, Data98187; header16 included. Exact44-interval complement round03/unknown.json; metrics round03/metrics.json.

| Physical PRG8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 1776 | 6263 | 8039 | 153 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 744 | 7448 | 8192 | 0 |
| 9 | 6906 | 1112 | 8018 | 174 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 1013 | 7179 | 8192 | 0 |
| 12 | 6594 | 1590 | 8184 | 8 |
| 13 | 3566 | 4626 | 8192 | 0 |
| 14 | 5287 | 2555 | 7842 | 350 |
| 15 | 6087 | 1862 | 7949 | 243 |

Fresh maps/pass-004.json, generated/pass-004 and analysis/pass-004-verification.json confirm valid and byte_identical=true; analysis/state.json and maps/current.json current. ROM unchanged. Input pass-003/history preserved. Independent evidence round03/review-notes.md, code-independent-decodes.json, consumer-independent-decodes.json, correction-caller.json and data-independent-review.json; primary redirect/round02-residual-20261031 contracts/ledger retained. Both ready R2 leads verified for supported batch; structuralStatic, no new natural-reachability/execution claim.

Remaining928 bytes need localized entry, matched stack/callback or bank-effect evidence. C0BF retains NES-MAP-002 one-byte overlap limitation/IRQ evidence, noData workaround. Reuse prior four artificial episode routes and native THE END; no unchanged replay or arbitrary bossRAM sweep. Natural boss initialization/victory remains unavailable. Strong ready connected batch exhausted; refreshed-source commentary/redirect may resolve precise residual conditions. This is an easy-gain handoff, not terminal exhaustion.

Toolkit report [toolkit-issues.md](toolkit-issues.md) retains NES-DOC-001, NES-MAP-002 and NES-DISC-003; no new issue or toolkit edit. Existing maintained commentary/source/review/history preserved. Full-current commentary must reconcile pass-004 additions and localized AC71 broadData correction; final editorial remains pending. No ROM mutation, archive, publication, cleanup or deletion.


## Round04 completed — disassembly pass, incomplete ROM

ROM classification: 99.97% (131048 / 131088 bytes classified; 40 bytes unknown).

Accepted832 StaticCode+56 StaticData bytes after independent primary per-interval structural/partial-return/stack/consumer review,386 fresh proposal instruction decodes,25 native context groups and5 supplemental boundary/interface groups. Code32805/Data98243 including16-byte header. Exact13-span complement round04/unknown.json; per-bank metrics round04/metrics.json.

| Physical PRG8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 1929 | 6263 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 744 | 7448 | 8192 | 0 |
| 9 | 7063 | 1128 | 8191 | 1 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 1013 | 7179 | 8192 | 0 |
| 12 | 6602 | 1590 | 8192 | 0 |
| 13 | 3566 | 4626 | 8192 | 0 |
| 14 | 5588 | 2595 | 8183 | 9 |
| 15 | 6300 | 1862 | 8162 | 30 |

Current state/map valid; fresh pass-006 rebuild byte-identical. Input pass-004 preserved. pass-005 is an additional byte-identical unchanged-input checkpoint accidentally generated after first annotation was atomically rejected for missing view cpu; corrected main CPU/uploaded fields applied before any accepted map save. Historical pass-005 retained, no toolkit bug inferred. Evidence round04/review-notes.md, code-independent-decodes.json, consumer-independent-decodes.json, supplemental-evidence.json and data-independent-review.json. Primary redirect/round03-local-contracts-20261031 retained. R3-LOCAL-STRUCTURAL-CONTRACTS verified; Static structure, no new observed/natural execution claims. Existing AC71 correction/source/history untouched.

Remaining40bytes/13spans need localized entry/return/field-interface or mixed-data discrimination; C0BF remains NES-MAP-002 representation issue, noData workaround. This is an easy-gain handoff, not terminal exhaustion. No obvious connected supported continuation reserved. Reused four artificial episode/native THE END evidence; no unchanged gameplay replay. Natural boss initialization/victory remains unresolved. Toolkit [toolkit-issues.md](toolkit-issues.md) preserves NES-DOC-001/NES-MAP-002/NES-DISC-003; no new issue or edits.

Full-current commentary pending for888 added bytes; prior maintained source/review/history preserved. Final editorial pending. No ROM/toolkit mutation, archive/publication, cleanup/deletion or next phase.


## Round05 completed — disassembly pass, incomplete ROM

ROM classification: 99.99% (131070 / 131088 bytes classified; 18 bytes unknown).

Accepted22 StaticCode/0Data bytes: D6E3 argument2, F6E9 timer5, DAF1 scene return1, DCF3 password return1, separately accepted E762 mechanical scratch-pair transform13. Independent review round05/review-notes.md,14 fresh instruction decodes and12 native context groups. E762 does not claim arbitrary multiply20 or original input relation. No new observed/natural-use claim. Code32827/Data98243 including16-byte header.

| Physical PRG8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 1929 | 6263 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 744 | 7448 | 8192 | 0 |
| 9 | 7063 | 1128 | 8191 | 1 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 1013 | 7179 | 8192 | 0 |
| 12 | 6602 | 1590 | 8192 | 0 |
| 13 | 3566 | 4626 | 8192 | 0 |
| 14 | 5592 | 2595 | 8187 | 5 |
| 15 | 6318 | 1862 | 8180 | 12 |

Exact complement round05/unknown.json: isolated RTS79830,117611,122893,128500,129494; C09C114860+2; overlapping C0BF114895+1; mixed pre-vector131072+10. No speculative tinyData. State/map validated, fresh pass-007 byte-identical, mapSHA256 92c25b1185f7cc9103a90fa5bcf8b8a47b604f12747dde478becf6c028d0e2b0. Current-source maintained commentary and historical evidence untouched; full-current reconciliation required for22 added bytes, final editorial pending. R4-RESIDUAL-INTERFACES verified; all supported connected additions accepted. No productive continuation deliberately reserved. Residual same-field/stack/format questions need materially new discrimination; not terminal exhaustion. Reused broad four artificial episode/native THE END evidence, no unchanged replay; natural victory unresolved.

Toolkit [toolkit-issues.md](toolkit-issues.md): existing confirmed documentation issue NES-DOC-001, overlap representation feature NES-MAP-002, conservative discovery limitation NES-DISC-003 preserved. No new issue or toolkit edit. C0BF remains representation blocker/noData workaround. No ROM mutation, archive/publication/cleanup/deletion or next phase.


## Round06 completed — disassembly pass, incomplete ROM

ROM classification: 99.99% (131072 / 131088 bytes classified; 16 bytes unknown).

Accepted2 StaticCode/0Data bytes, physical114860+2 CPUC09C A9BD, InstallSpriteZeroMarkerAt189, context[null,null,14,15]. R5-MARKER-CONSTANT-ENTRY verified by independent primary decode/native caller/complete body/bounds review: round06/review-notes.md and primary-decodes.json. Narrow mechanical constant189 supplier to native A-to-OAM Y interface; no natural caller, scene-default or timing claim. Prior exactBD default prerequisite corrected; preserved interim rejection superseded on that premise only. Existing unreferenced shared-body wrappers apply the same structural standard. Code32829/Data98243 including16-byte header.

| Physical PRG8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 1929 | 6263 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 744 | 7448 | 8192 | 0 |
| 9 | 7063 | 1128 | 8191 | 1 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 1013 | 7179 | 8192 | 0 |
| 12 | 6602 | 1590 | 8192 | 0 |
| 13 | 3566 | 4626 | 8192 | 0 |
| 14 | 5594 | 2595 | 8189 | 3 |
| 15 | 6318 | 1862 | 8180 | 12 |

Exact complement round06/unknown.json: isolated RTS79830,117611,122893,128500,129494; mixedFFF0 physical131072+10; C0BF114895+1.15research-ambiguous bytes plus1external representation blocker. No automatic terminal determination from this gain. Connected native body/callers already classified; no supported continuation withheld. Fresh pass-008 valid/byte-identical; mapSHA256 eadc61fc669a5a918f0479c7062b2c86858b279a25d6eeed77f17aae941890e3. Input ROM unchanged; maintained source/review/history preserved. Next phase must reconcile current commentary for one new instruction/two bytes and corrected stale C09C uncertainty; final editorial pending.

Reused prior four artificial episodes/native THE END evidence and existing scenarios; no unchanged gameplay replay or new reachability claim. Natural boss initializer/victory unresolved. Residual scoped review remains in redirect/round05-residual-rejection-20261031/rejection-review.md with marker-correction.md; no same-method retry. Toolkit [toolkit-issues.md](toolkit-issues.md): NES-DOC-001, NES-MAP-002 and NES-DISC-003 retained, no new issue or edits. Preserve C0BF ordinary LDA/IRQ SLO overlap, noData workaround. No ROM/toolkit mutation, archive/publication/cleanup/deletion or next phase.
