# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round001 intermediate findings
- Baseline verified; mapper1 initial PRG slots[0,1,14,15]. Reset$C000 enters PPU-vblank wait and MMC1 setup; mapper-affecting store stops static path after nine instructions without new context.
- Legal splash is not interactive game title. Wait through frame1011 to PRESS START title; corrected buttons-array input starts narrative intro. Ten bounded Start/release cycles advance to street gameplay. Movement/jump/up/down/select/B trace sampled; frame capture shows moved player and changed HUD.
- B+Select at true title opens scrambled password. RAM$0250 contains displayed permutation, RAM$0270 contains expected ASCII password; separate PPU upload copy$0104. Exact clean snapshot password-ready.snapshot retained. Testing buffer patch to target, Select/Start marks evidence artificial. No arbitrary RAM search or ROM changes.
- Static connected flow and observed opcode starts classified incrementally; raw baseline not imported. Evidence under round-001. Current-source commentary deferred to designated phase.

Part III shortcut succeeded: frame capture part3-play.ppm.png shows distinct outdoor canyon terrain and player movement/jump after exact RAM password buffer patch. This is an artificial scenario with actual opcode fetch evidence, not proof of natural password solution route. Gameplay controls right30/right+a20/right20/up5/empty5 exercised after bounded narrative skips. New later-game trace has3219 contextual roots; connected static expansion is the final batch. Ending remains unvisited: expected inventory/object-return progression contract is not understood; no arbitrary flags were patched.

Concrete redirect questions:
1. Recover time-travel/area transition RAM contract from Select/B consumers and stage initialization; use clean street and PartIII snapshots to distinguish time-period/current-area state before patching. Needed to visit additional periods/rooms without repeated manual traversal.
2. Inspect $0250/$0270 password comparison and screen exit consumer to explain accepted password contract; exact byte patches are proven sufficient, natural shuffle controls unresolved.
3. Follow unresolved indirect CPU-window targets at physical127941 and127944 (saved discovery diagnostics), likely dispatch structures. Determine selector domain/bank context before pointer-table extent or static target claims.
4. PRG banks4-7 contain many repeated runs/record patterns (uniform-data-inspection.json); adjacent non-code spans can be bounded by consumers. Generic data requires no recovered semantics, but code-versus-data ambiguity outside sampled runs remains unresolved.

## Round001 disassembly complete, ROM incomplete

ROM classification: 55.07% (144372 / 262160 bytes classified; 117788 bytes unknown).


| Physical section | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 274 | 0 | 7918 |
| PRG01 | 0 | 32 | 8160 |
| PRG02 | 0 | 0 | 8192 |
| PRG03 | 148 | 34 | 8010 |
| PRG04 | 0 | 429 | 7763 |
| PRG05 | 223 | 0 | 7969 |
| PRG06 | 0 | 320 | 7872 |
| PRG07 | 0 | 370 | 7822 |
| PRG08 | 0 | 0 | 8192 |
| PRG09 | 249 | 246 | 7697 |
| PRG10 | 0 | 35 | 8157 |
| PRG11 | 1090 | 837 | 6265 |
| PRG12 | 1902 | 0 | 6290 |
| PRG13 | 529 | 0 | 7663 |
| PRG14 | 3648 | 0 | 4544 |
| PRG15 | 2793 | 125 | 5274 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

Internal store independently checked for nonoverlap, unique physical-byte count, and equality with exported current map. 5164 regions,10856 Code bytes and133516 Data/Image bytes. All regions have evidence; code has explicit CPU views. Reachability differs: ordinary startup/title/intro/street observed; PartIII artificial password-buffer scenario observed; connected static successors not claimed reached. Execution coverage is retained per-scenario in round-001/*summary.json and *contexts.json, not this classification percentage.

Final checkpoint [pass-001-final](pass-001-final-verification.json) validates, generates and rebuilds byte-identically; map SHA25632ffeb8013e4cd14cbba45d0f914858256ca73eaacee6be15d1f9fbbc726a538. Input SHA256 unchanged. Baseline and superseded checkpoints retained. Current phase: disassembly finished; coordinator should run commentary then redirect. Current-source readability is not yet completed.

Easy connected observed/static batch complete; practical street and PartIII shortcut tested. Further gameplay requires area/time/inventory transition contract recovery, not repeated basic traversal. Remaining indirect dispatch and patterned data questions are ready for redirect, not a terminal exhaustion claim. Two confirmed API/documentation discrepancies in [toolkit report](toolkit-issues.md), no blockers, no toolkit edits. No publishing/archive done.

## Round001 redirect — concrete saved frontiers investigated
Resolved camera and terrain false static-fallthrough claims, recovered writable-hook setters/three callback contracts plus thirteen bank2 handlers, requested/current-area initializer and doorway records, paired password strings/pointers, returned-item bitset gate and ending sequence. Artificial area1F/2F gameplay and visible THE END now confirmed; prior unresolved transition/ending prerequisite has been materially resolved. Generic-data role review proposes27repeated low-value subintervals5826bytes beyond initial uniform runs. Seven ready leads and reusable artifacts in redirect/round-001-a, compact handoff handoffs/round-001-redirect.md. Proposal/acceptance remains separate; map still144372classified,117788unknown. See findings.md for exact boundaries, failed setup trials, context/alias safeguards and actual unresolved prerequisites. This is productive phase completion, not terminal exhaustion.

## Round002 disassembly complete — ROM incomplete
Input pass-001-final; output pass-002-disassembly. ROM classification: 60.47% (158530 / 262160 bytes classified; 103630 bytes unknown).

Accepted14158new unique physical bytes:9717core(2666artificial Observed Code+7051Static Data),4441connected Static Code; corrected22Code bytes to Data. Internal map/export equal, nonoverlap and exact unique-byte counts independently checked; 8566regions. Map SHA256 40ea782f16e1af1e4512f2388dc161d07561ab70976924ec3ed75c6422d275e5. ROM SHA unchanged.

Evidence: [review](round-002/review-notes.md), [core](round-002/core-applied.json), [observed checks](round-002/observed-review.json), [static accepted batch1](round-002/static-review-accepted-batch1.json), [static final batch](round-002/static-review.json), [counts](round-002/counts.json). Historical preliminary static reviews retained but superseded by stricter indirect-store audit; final static accepted records total4441bytes. All seven R001 lead outcomes verified in leads.json; structural semantics and natural reachability kept separate.

Fresh checkpoint validates/generates/rebuilds byte-identically; verification analysis/pass-002-disassembly-verification.json. Commentary files preserved; they cover previous map and require reconciliation for new code/data next phase, final editorial audit pending. Saved broad street/PartIII/area1F/2F/ending THE END scenarios reviewed, no unchanged replay or new runtime patches. Concrete unproved return-bank and indirect selector contracts retained for redirect, not terminal exhaustion. Existing [toolkit issues](toolkit-issues.md):two nonblocking API/documentation discrepancies; no new toolkit issues found. No toolkit/ROM edits, archive, publish or cleanup.

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 367 | 353 | 7472 |
| PRG01 | 0 | 118 | 8074 |
| PRG02 | 0 | 440 | 7752 |
| PRG03 | 148 | 34 | 8010 |
| PRG04 | 0 | 2296 | 5896 |
| PRG05 | 2168 | 94 | 5930 |
| PRG06 | 0 | 617 | 7575 |
| PRG07 | 0 | 438 | 7754 |
| PRG08 | 0 | 84 | 8108 |
| PRG09 | 1634 | 2459 | 4099 |
| PRG10 | 0 | 619 | 7573 |
| PRG11 | 1173 | 1057 | 5962 |
| PRG12 | 2067 | 0 | 6125 |
| PRG13 | 1305 | 96 | 6791 |
| PRG14 | 5143 | 0 | 3049 |
| PRG15 | 3936 | 796 | 3460 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

## Round002 redirect complete — combined structural proposal
Input pass-002-disassembly unchanged;6146unknown bytes proposed (4301Code/1845Data),accepted0. Corrected broad mapper-store barrier with CHR-specific/explicit restore wrappers,16input and conditional87object/17terrain dispatch handlers; followed connected scene/score/bank5helper graph. Independent222byte resource-copy and bounded HUD/row-table evidence included. Valid packed/map coordinate preconditions remain distinct from natural writer proof. Read analysis/redirect/round-002-a/findings.md and proposal-preconditions.json; exact views/data/254interval complement retained. Three readyR002leads inleads.json; prior outcomes preserved. No terminal-exhaustion claim. Current-source readability complete at unchanged input; next disassembly independently validates proposals, then commentary.

## Round003 disassembly complete — ROM incomplete
Input pass-002-disassembly; output pass-003-disassembly. ROM classification: 62.82% (164676 / 262160 bytes classified; 97484 bytes unknown).

Accepted6146new unique physical bytes:4301Static Code at2048starts,1845Static Data. ThreeR002ready leads verified; full fresh candidate/context decodes and independent physical data/consumer/wrapper audits in [review](round-003/review-notes.md), [fresh decodes](round-003/fresh-candidate-decodes.json), [primary](round-003/primary-audit.json), [supplement](round-003/supplemental-primary.json), [accepted](round-003/accepted.json), [counts](round-003/counts.json). Generic Data used after bounded role inspection where full schema/domain unproved. Static calling contracts separate from natural reachability. Internal map/export equal and validated; exact unknown complement [here](round-003/unknown-complement.json). Map SHA256291758555315a83340815774c6461b664d55eabcb52f36c9fb692deda6b80d72; ROM hash unchanged.

Fresh checkpoint validates/generates/rebuilds byte-identically:analysis/pass-003-disassembly-verification.json. Existing commentary/source/history preserved; next commentary must reconcile full current source and correct bank0F1E1->811A six-digit score-adder semantics. Broad saved progression/ending milestones reviewed, no unchanged replay. Remaining unrestricted selector/map-coordinate invariants do not invalidate accepted conditional slices. Residual compressed maps/scripts/audio/resource extents need concrete structural research; ordinary easy-gain handoff, no terminal whole-ROM exhaustion claim. Existing [toolkit issues](toolkit-issues.md)NES-001/NES-002 nonblocking;no new toolkit issues found. No ROM/toolkit edits,archive,publish or cleanup.

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 436 | 353 | 7403 |
| PRG01 | 0 | 118 | 8074 |
| PRG02 | 0 | 440 | 7752 |
| PRG03 | 148 | 34 | 8010 |
| PRG04 | 0 | 2296 | 5896 |
| PRG05 | 3017 | 94 | 5081 |
| PRG06 | 0 | 617 | 7575 |
| PRG07 | 0 | 1134 | 7058 |
| PRG08 | 0 | 84 | 8108 |
| PRG09 | 2148 | 2459 | 3585 |
| PRG10 | 0 | 619 | 7573 |
| PRG11 | 1231 | 1838 | 5123 |
| PRG12 | 2084 | 0 | 6108 |
| PRG13 | 3519 | 304 | 4369 |
| PRG14 | 5143 | 0 | 3049 |
| PRG15 | 4516 | 956 | 2720 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |


## Round004 disassembly complete — ROM incomplete
Input pass-003-disassembly; output pass-004-disassembly. ROM classification: 93.17% (244253 / 262160 bytes classified; 17907 bytes unknown).

Accepted79577new unique physical bytes:74798Static Data in751bank-split runs and4779Static Code at2210starts. BothR003ready leads verified, prior histories preserved. Complete independent fresh views, primary consumer/wrapper/context/physical audits and exact bound reparses in [review](round-004/review-notes.md), [Code decodes](round-004/fresh-candidate-decodes.json), [primary](round-004/primary-audit.json), [supplement](round-004/supplemental-primary.json), [Data audit](round-004/data-audit.json), [physical audit](round-004/physical-audit.json), [accepted](round-004/accepted.json). Generic fallback follows practical role/reference/neighbor/pattern review and asserts noncode, not a full original schema. Normal-return/fixed-only views and local selector/predicate/object/queue contracts remain separate from natural reachability.

Internal map equals current export; validation passed. Counts independently use unique classified physical bytes, ignoring raw export fallback:Code27021,Data86160,Image131072. Exact17907unknown bytes in793bank-split intervals with structure-group/frontier dispositions in [ledger](round-004/unknown-complement.json), [counts](round-004/counts.json). Map/state/checkpoint SHA25662ceac56bdceca6f4f957fcf6a979a2fa18333bbb96d94a8ce9538a7d30a451f. Fresh checkpoint validation/generation/rebuild byte-identical: [verification](pass-004-disassembly-verification.json). ROM hash unchanged; current commentary source SHA256adfe90d4e9d9d200af346b71ae6af854a4a2b8bdc2560f95de26081326d9bfb2 unchanged/preserved. Full new-current-source commentary remains next phase.

Saved street/PartIII/area1F/2F/THE END observed capture/fetch routes reviewed, no unchanged replay. Supported connected command/subcommand/predicate, initializer/effect, map/sprite/audio/script/placement/resource families fully reviewed; no original lead-range ceiling. Remaining mixed code/data, lower-window return contexts and statefulscriptreturn/loop extents require specific local bank/state/boundary evidence in next redirect, not another unchanged broad gameplay run. This is an ordinary finite-batch handoff, not whole-ROM exhaustion. Toolkit report [NES-001/NES-002](toolkit-issues.md) preserved, both nonblocking;no new toolkit issues found. No toolkit/ROM/current-commentary-source edits,archive,publish orcleanup. Disk7.4GiBfree, no scratch batch generated. Compact next-phase handoff: [round-004-disassembly](handoffs/round-004-disassembly.md).

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 646 | 5937 | 1609 |
| PRG01 | 0 | 6474 | 1718 |
| PRG02 | 0 | 7724 | 468 |
| PRG03 | 148 | 7575 | 469 |
| PRG04 | 0 | 8192 | 0 |
| PRG05 | 3017 | 94 | 5081 |
| PRG06 | 0 | 7914 | 278 |
| PRG07 | 0 | 7867 | 325 |
| PRG08 | 0 | 7540 | 652 |
| PRG09 | 2611 | 4432 | 1149 |
| PRG10 | 0 | 7584 | 608 |
| PRG11 | 1231 | 6244 | 717 |
| PRG12 | 3479 | 2070 | 2643 |
| PRG13 | 3828 | 3997 | 367 |
| PRG14 | 6846 | 154 | 1192 |
| PRG15 | 5215 | 2346 | 631 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |


## Round005 disassembly complete — ROM incomplete
Input pass-004-disassembly; output pass-005-disassembly. ROM classification: 99.94% (261993 / 262160 bytes classified; 167 bytes unknown).

Accepted17740new unique physical bytes:5443StaticCode at2575starts/views and12297StaticData (10084resource/generic +2213script) in738bank-split runs. All three R004 ready leads verified; earlier outcomes/history preserved. Independent exact freshCode/physicalData/primarycontract audits and own complete101908state/2437token traversal,22wholefamilies withthreeprefix-onlyhigharrays: [review](round-005/review-notes.md), [freshCode](round-005/fresh-candidate-decodes.json), [primary](round-005/primary-audit.json), [supplement](round-005/supplemental-primary.json), [physical](round-005/physical-audit.json), [structures](round-005/structure-audit.json), [accepted](round-005/accepted.json). Exact local finite bank/selector/stack/RAM-grid/normalreturn contracts retained; naturalreachabilityunclaimed. GenericData follows bounded role/reference/neighbor/pattern review, no originalgenerator/schema asserted.

Internalmap equals currentexport;mapvalidationpassed. Independent unique classified totals:Code32464,Data98457,Image131072. Exact167unknownbytes/21bank-splitintervals in [ledger](round-005/unknown-complement.json), [counts](round-005/counts.json). Currentmap/state/checkpoint SHA256 504f8246c54888d097dec9bc54bce35a53575b00f2518c8d443e76d339adc34f. Freshcheckpoint validation/generation/rebuild byteidentical: [verification](pass-005-disassembly-verification.json). ROM hash unchanged. Full existing commentarysource/artifacts retained byte-for-byte, [preservation](round-005/commentary-preservation.json); resulting-source reconciliation/readability pending next commentary phase. No finaleditorialaudit claim.

Saved street/PartIII/areas1F/2F/THE END actual fetch/capture/routes reviewed; no changed discriminating question warrants unchangedreplay. Supported connected batch accepted together. Remaining short code/stack/return, record/script fragments retain material local structural/extent/bank uncertainty; exact redirectfrontier explanations remain [frontier](redirect/round-004-a/frontier-review.json), nowcorroboratedbyactualledger. Full new-current-sourcecommentary should inspect newlyacceptedpredecessors/consumers before redirect. This is ordinaryproductivebatchhandoff, not terminalexhaustion; no arbitraryquotaor speculativecompletion. [Toolkitissues](toolkit-issues.md) NES-001/NES-002 remainnonblocking;no new toolkit issues found. Configuredreleasebinary/toolkit/ROM unchanged. Noarchive,publish,cleanup. Compacthandoff: [round005](handoffs/round-005-disassembly.md).

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 852 | 7340 | 0 |
| PRG01 | 0 | 8192 | 0 |
| PRG02 | 0 | 8192 | 0 |
| PRG03 | 148 | 8044 | 0 |
| PRG04 | 0 | 8192 | 0 |
| PRG05 | 6487 | 1705 | 0 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 |
| PRG09 | 2753 | 5403 | 36 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1231 | 6961 | 0 |
| PRG12 | 3922 | 4191 | 79 |
| PRG13 | 3855 | 4314 | 23 |
| PRG14 | 7658 | 511 | 23 |
| PRG15 | 5558 | 2628 | 6 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

Round005 handoff diskcheck:7.2GiBfree,analysis/scratch32KiB,round0054.1MiB;no cleanup. All26commentaryartifacts match preservation hashes after checkpoint.

## Round006 disassembly complete — ROM incomplete

ROM classification: 100.00% (262153 / 262160 bytes classified; 7 bytes unknown).

Rounded100.00% does not establish completion. Accepted160new unique physical bytes in17Static intervals:111Code49Data. Both R005 ready leads verified, prior history retained. Independent all17 immutable byte/bank/boundary checks,58 exact new instruction decodes,14 fresh helper windows and primary local stack/leaf/explicit bank-return/finite-consumer/repeated-pattern contracts: [review](round-006/review-notes.md), [claims](round-006/fresh-claim-audit.json), [helpers](round-006/fresh-helper-decodes.json), [accepted](round-006/accepted.json). Correct BD76 BNEBD83 sharedLDY1/upload16 retained. GenericData follows bounded role/reference/neighbor/pattern inspection; no original schema/selector or new natural/observed reachability claim.

Internal state equals currentexport. Independent unique totals: Code32575,Data98506,Image131072; [counts/per-bank](round-006/counts.json). Exact residual7bytes/fourintervals:78036..78037 RTS(PRG09),105403..10540580FF(PRG12),118249..118252JMPCCCB(PRG14),130858..130859RTS(PRG15), [complement](round-006/unknown-complement.json). Accepted batch supplies no fallthrough or separate entry/stack/pattern envelope for these; legal decoding alone insufficient. Concrete remaining discriminator is separate entry/control or non-code-envelope evidence; preserve unknown. Productive pass handoff, not terminal exhaustion; commentary should reconcile17new regions before redirect.

Validation passed; fresh pass-006-disassembly source generation/rebuild byte-identical: [verification](pass-006-disassembly-verification.json). Map/state/checkpoint SHA2513dedafed5398b7ab7d62b63f7ca347d721c6958d1f2137be98aa35c6f8084. ROM hash unchanged. Prior full commentary and pass005 source retained byte-for-byte: [preservation](round-006/preservation-verified.json). New-current-source readability and final editorial audit remain separate pending phases.

Saved broad street/PartIII/areas1F/2F/THE END routes/captures/fetches reused; no changed discriminating gameplay experiment warrants unchanged replay. [Toolkit issues](toolkit-issues.md) NES-001/NES-002 remain nonblocking; no new toolkit issues found. Configuredreleasebinary/toolkit unchanged. No archive,publish,cleanup or source edits. Disk7.1GiBfree, scratch32KiB; no bulk evidence. Handoff [round006](handoffs/round-006-disassembly.md).

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 852 | 7340 | 0 |
| PRG01 | 0 | 8192 | 0 |
| PRG02 | 0 | 8192 | 0 |
| PRG03 | 148 | 8044 | 0 |
| PRG04 | 0 | 8192 | 0 |
| PRG05 | 6487 | 1705 | 0 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 |
| PRG09 | 2762 | 5429 | 1 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1231 | 6961 | 0 |
| PRG12 | 3999 | 4191 | 2 |
| PRG13 | 3864 | 4328 | 0 |
| PRG14 | 7669 | 520 | 3 |
| PRG15 | 5563 | 2628 | 1 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

## Round006 redirect: narrowed80FF genericData proposal

Input pass-006-disassembly,7unknownbytes/fourintervals. R006-generic-script-prefix ready2bytes at105403..105405, accepted0. Corrected token-origin prerequisite only for genericData; independent fixed80/89 descriptor semantics and13accepted repetition instances plus adjacent27-byte script favor non-code. Full original start/domain and natural reachability remain unclaimed. Exact evidence and independent review recipe: redirect/round-006-a/proposal.json,script-pattern-evidence.json,findings.md. Three strongest remaining candidates have primary combined-constraint rejectionreview at redirect/round-006-a/rejection-review.md; no concrete current revised experiment identified. No unchanged runtime replay; all local state/maps/source unchanged. Productive proposal handoff, not terminal whole-ROM exhaustion. See handoffs/round-006-redirect.md.

## Round007 disassembly complete — incomplete ROM

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).

Accepted R006-generic-script-prefix:2bytes Static genericData [105403,105405). Independent physical/repeated-pattern/descriptor/RAM-handler/full29-byte bounded representation and competing-code review: analysis/round-007/review-notes.md; fresh-audit.json. Lead verified; original stream origin, global schema and natural reachability unclaimed. Complete connected supported batch already classified downstream. Saved broad progression/ending evidence reused; no unchanged gameplay replay. Exact residuals:78036..78037 RTS,118249..118252 JMPCCCB,130858..130859 RTS; rationale preserved in redirect/round-006-a/rejection-review.md for coordinator inspection.

map.validate ok; state/current equal; new maps/pass-007-disassembly.json SHA256 56271f635aa2e50716e6ebb1c1b3f33ae4a0e9eca4233b9b446dd56984493ea4; generated/pass-007-disassembly verifies byte_identical true. ROM SHA unchanged. Existing commentary/source and history preserved untouched; updated classification needs next commentary reconciliation. No new toolkit issues; existing [NES-001/NES-002](toolkit-issues.md) nonblocking. No toolkit edits, archive, publish or cleanup. Independent unique-byte/per-bank counts in round-007/counts.json; exact complement in round-007/unknown-complement.json.

| Bank | Classified | Unknown |
|---|---:|---:|
| Header | 16 | 0 |
| PRG00 | 8192 | 0 |
| PRG01 | 8192 | 0 |
| PRG02 | 8192 | 0 |
| PRG03 | 8192 | 0 |
| PRG04 | 8192 | 0 |
| PRG05 | 8192 | 0 |
| PRG06 | 8192 | 0 |
| PRG07 | 8192 | 0 |
| PRG08 | 8192 | 0 |
| PRG09 | 8191 | 1 |
| PRG10 | 8192 | 0 |
| PRG11 | 8192 | 0 |
| PRG12 | 8192 | 0 |
| PRG13 | 8192 | 0 |
| PRG14 | 8189 | 3 |
| PRG15 | 8191 | 1 |
| CHR00 | 8192 | 0 |
| CHR01 | 8192 | 0 |
| CHR02 | 8192 | 0 |
| CHR03 | 8192 | 0 |
| CHR04 | 8192 | 0 |
| CHR05 | 8192 | 0 |
| CHR06 | 8192 | 0 |
| CHR07 | 8192 | 0 |
| CHR08 | 8192 | 0 |
| CHR09 | 8192 | 0 |
| CHR10 | 8192 | 0 |
| CHR11 | 8192 | 0 |
| CHR12 | 8192 | 0 |
| CHR13 | 8192 | 0 |
| CHR14 | 8192 | 0 |
| CHR15 | 8192 | 0 |

## Round007 redirect

Reviewed pass007's new80FF Data acceptance and current readability repair against prior primary three-fragment rejection reasoning; no transferable new constraint or corrected premise remains. Fresh exact complement5bytes/3intervals; no ready lead or discriminating runtime method. Reuse primary rejection credit with changed-premise audit at redirect/round-007-a/rejection-review.md. Terminal-incomplete recommendation only; coordinator final inspection/editorial acceptance pending. No productive frontier interrupted; unchanged broad ending routes not replayed. No map/source/ROM/toolkit changes.
