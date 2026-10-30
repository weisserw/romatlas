# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## R001 static and runtime batch
Validated baseline and vectors; reset/NMI/IRQ entries in fixed 8 KiB banks 14/15. Connected code discovery from accepted ROM fetches preserves exact execution views. Mapper and return-bank barriers remain. Additional static closure follows direct JSR *callee* entries under pre-call context, because entry cannot itself change banks before its first instruction; it does not assume a bank-preserving return. See round-001/staticcalls.py and call-expansion-*.json. New 4,022-byte static batch contains official decoded instructions only, no BRK/unofficial anomalies. Natural reachability remains distinct.
Banks 2–11 reviewed as continuous non-code map/graphic payloads, mixed/encoded portions classified generic Data. Local tile overview and boundary/block review retained. Specific subformats left for semantics rather than blocking classification.
TI-001 invalidated claimed button routes before realstart: those were no-input natural attract execution, not input experiments. realstart and parkrecovery use full buttons arrays. Natural Class 1/Main Park visited; interrupted skate process SIGTERM after 330 frames preserves frames/RAM but not its final aggregate store. Replay recovery to parked state uses realstart snapshot and 120 neutral frames. No unchanged failed input experiments retried after API correction.
Ready next experiments: artificial event entry $D93A under Y=$0565=1..4, tickets $0578>0, class $020E initialized, injected ordinary return stack to paused game; see event.py. Exact patches and artificial evidence recorded per branch. This is understood consumer mechanics and broad progression, not natural event reachability.


## Round 001 disassembly finished — incomplete ROM
ROM classification: 91.64% (120134 / 131088 bytes classified; 10954 bytes unknown).

Fresh `pass-001` checkpoint verifies byte-identical assembly, map SHA256 44848ce7a8323b7fcffbc8cb9542913b0d6a37253c6c8ef2f4bf7e9e48890e4b; internal state, current export and checkpoint map match. See analysis/pass-001-verification.json and generated/pass-001/game.asm. Input hash remains unchanged. 14,761 Code bytes, 105,373 Data bytes, 643 evidence-bearing regions; header16 Data bytes outside PRG banks. Unknown complement126 intervals in analysis/round-001/unknown-final.json.

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 751 | 7043 | 398 |
| 1 | 3059 | 714 | 4419 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 710 | 7482 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 5910 | 0 | 2282 |
| 15 | 4331 | 6 | 3855 |

Natural Class1/Main Park visited with corrected full buttons array. All four event modes and Class4 Ramp initialized and sampled under recorded artificial entry contracts; TOP TEN failure-path terminal screen visited artificially. Event results/medals, shops and distinct ending remain deferred with concrete missing contracts in scenarios.json and leads L005–L007. L001–L004 verified. Detailed handoff: analysis/handoffs/round-001-disassembly.md. Static/data evidence and exact replay artifacts remain local in round-001. Natural fetch starts4,868, artificial5,180, union5,868 (overlapping); execution coverage is separate from structural classification. No aggregate drops in retained scenario summaries; detail capacity0 intentionally drops event detail. Several SIGTERM experiments preserve partial evidence and resumable per-action snapshots; no running experiment required at handoff.

Toolkit summary: one confirmed documentation/API-validation mismatch TI-001 (emulator.input old schema silently ignored); corrected workaround in driver, no blocker, no toolkit edits. See analysis/toolkit-issues.md. Static easy gains and representative broad progression complete for this pass; mixed code/inline data and progression contracts go to redirect after commentary. No terminal exhaustion claim; no full-ROM/editorial/hardware compatibility claim.

## Round001 redirect r01 — combined ready batch
From pass-001/current-source readability-complete handoff, investigated L005 fixed-bank mixed objectscript/native structure, L006 results/class progression and L007 world interaction together, then actionable direct-consumer and native-continuation avenues L008. Evidence and exact frontier/history: analysis/redirect/round-001-r01/status.md, grammar.md, source-scan.json, native-cfg.json, native-continuations.json, independent-native.json, bounds.md. No structural map/source changes.

Combined proposals.json has10942 distinct currentlyunknown bytes:4078Code,6864Data. Exact unchanged acceptedunknown complement10954 in unknown-ledger.json; all12 unproposed bytes are RTS-only residuals without incoming control/consumed-data evidence, not proof of unused code/data. Independent disassembly must validate and accept candidates; predicted12unknown is not currentclassification. Initial7017byteproposal preserved separately. No terminal exhaustion/completeROM claim.

Runtime artificialresult reachesGoldMedal/CashPrize55/NoBonus/medalmap; overlay map shows fourevents and fourequipment shops then exits toREADY (parkreappearanceunproved). BE90 purchaseconsumer changescash100->75/equipment0->1/flagFF->0 under exact callcontract. Unacceleratedclass4shortcut showsClassFinished; processSIGTERM after1740savedframes leaves no finalaggregate/snapshot, preserved as partial observation. No replay needed for structure. OldD980shop hypothesis corrected toinformationmap; actualpurchase player script14->BE90. Existingcash account was namedTicketDigits/CompareTicketCost incommentary and needs corrected mechanicalnaming during nextfullreadabilitypass.

All known771scriptrecords/62callbackroots/34commands and concrete sourcecontrol/table references followed to useful bounds. RemainingRTS bytes have no newdiscriminating method under currentevidence; classificationreview ofreadybatch next, not emptycycle or terminalstop. No newtoolkitissuesfound; TI001inputschema mismatch workaroundremains, no blocker.


## Round 002 disassembly finished — incomplete ROM
ROM classification: 99.99% (131076 / 131088 bytes classified; 12 bytes unknown).

Independently accepted L005–L008 combined batch: 4,078 Code + 6,864 Data = 10,942 formerly unknown bytes, in 174 role/context-preserving annotations. Total Code18,839/Data112,237, 817/817 evidence-bearing regions (header16 Data outside PRG). Fresh native decoding1,900 instructions, independent771 script widths/bytes, no script/native overlap or unsupported direct successor. Primary interpreter/stack/consumer/bank review: analysis/round-002/review.md; native-review.json, pattern-review.json, accepted.json, accounting.json preserve checks. Static structural acceptance makes no natural-use claim.

pass-002 fresh checkpoint map validates and generated assembly rebuilds byte-identically; analysis/pass-002-verification.json. State/current/checkpoint map SHA256 c87510db28caf769ac3674f40e7628477fd591c3a21ac9e5c6ee3a8c66d642e4; input SHA unchanged. Exact physical accounting corrects redirect handoff bank totals: first three residual RTS are physical bank0 at7886..7888 (CPU9EBE..9EC0), not bank1. A native instruction crossing9FFF/A000 contributes bytes to both physical banks. Use per-byte counts below.

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 1071 | 7118 | 3 |
| 1 | 4577 | 3615 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 710 | 7482 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 7168 | 1018 | 6 |
| 15 | 5313 | 2876 | 3 |

Twelve Unknown bytes in eight intervals: bank0 CPU9EBE..9EC0; bank14 C2DD,CE88,CEAB,CEB8,CECE,D0F7; bank15 EFBE..EFC0. They are bare unreferenced RTS values; no forced code/data inference. Neighboring classifications do not require them. Exact offsets/lengths in accounting.json. This is an ordinary incomplete pass handoff, not terminal exhaustion.

Broad gameplay review reused existing four-event/class4/result/medal/information-map/purchase/ClassFinished artifacts and scenarios.json. No changed progression contract or boundary question justified unchanged replay. Park reappearance and a distinct ending remain unproved. Execution coverage is unchanged by this pass: no new emulator fetches; prior disassembly natural starts4868/artificial5180/union5868 remain historical, redirect artificial fetches separately retained in its contexts. Structural classification is independent of execution coverage.

Current-source commentary now requires full pass-002 review, including 10,942 added bytes and corrected cash/TicketDigits/CompareTicketCost names. Prior commentary/source/history preserved. No final editorial audit or ROM-complete claim. After commentary, redirect reassesses exact residuals and performs evidence-policy rejection review before any terminal decision. No new toolkit issues found; existing TI-001 confirmed input-schema mismatch/workaround known/no blocker (analysis/toolkit-issues.md). No toolkit edits, archive execution, deletion, publication, or ROM changes. Disk11GiB free; scratch4KiB before bounded review.
