# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 disassembly

ROM classification: 19.15% (25107 / 131088 bytes classified; 105981 bytes unknown).

| 8 KiB PRG bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 2217 | 20 | 5955 |
| 1 | 2295 | 10 | 5887 |
| 2 | 1109 | 47 | 7036 |
| 3 | 189 | 1062 | 6941 |
| 4 | 1571 | 18 | 6603 |
| 5 | 0 | 143 | 8049 |
| 6 | 0 | 176 | 8016 |
| 7 | 200 | 209 | 7783 |
| 8 | 438 | 61 | 7693 |
| 9 | 139 | 44 | 8009 |
| 10 | 76 | 2 | 8114 |
| 11 | 1654 | 261 | 6277 |
| 12 | 3459 | 56 | 4677 |
| 13 | 2738 | 20 | 5434 |
| 14 | 3959 | 52 | 4181 |
| 15 | 2603 | 263 | 5326 |

See round1-disassembly-handoff.md for exact structural basis, candidate rejection scope, gameplay routes, lead outcomes and verification. L-R1-ROOM: 3 targeted artificial entry variants failed to initialize overhead state; missing scheduler/transition contract, not evidence of dead code. L-R1-STATIC: validated dispatch domains and 3-wave callee traversal accepted; one impossible branch fallthrough rejected, generic data slice recovered. L-R1-GFX: concrete PPU loader/payload-boundary research remains. No terminal rejection review claimed because actionable redirect avenues remain.


## Round 2 disassembly completed — incomplete ROM

ROM classification: 62.87% (82416 / 131088 bytes classified; 48672 bytes unknown).

One pass accepted57,309newbytes:616Code and56,693Data, including63connected low-room table entries beyond the redirect batch. Total23,263Code/59,153Data,1,328regions. All new provenance Static; prior Observed bytes unaffected. Unique-byte counts and exact residual complement: analysis/round2-counts.json.

| 8 KiB PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 2217 | 20 | 2237 | 5955 |
| 1 | 2295 | 10 | 2305 | 5887 |
| 2 | 1109 | 47 | 1156 | 7036 |
| 3 | 189 | 7968 | 8157 | 35 |
| 4 | 1593 | 3700 | 5293 | 2899 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 200 | 6631 | 6831 | 1361 |
| 8 | 438 | 7180 | 7618 | 574 |
| 9 | 231 | 3041 | 3272 | 4920 |
| 10 | 316 | 7528 | 7844 | 348 |
| 11 | 1661 | 4566 | 6227 | 1965 |
| 12 | 3536 | 58 | 3594 | 4598 |
| 13 | 2846 | 501 | 3347 | 4845 |
| 14 | 4028 | 425 | 4453 | 3739 |
| 15 | 2604 | 1078 | 3682 | 4510 |

Header16Data is included in total and excluded from PRG table. Fresh map.validate/save/export completed and authoritative checkpoint **pass-002-reviewed** rebuilds byte-identically; analysis/pass-002-reviewed-verification.json. Map/state SHA256377fd647a853708853b65861cdd40264cc36df0932078f8bd711cc3330d59b78. Original ROM and prior commentary source unchanged (sourceSHA82974701ab5a7bef74fffdc36b920ea20caaba5a41b9274d9802a53b7342ee12). Superseded pass-002-final holds unchanged input due atomically rejected first request; corrected API schema before accepted batch, retained history.

Independent structural evidence: analysis/disassembly/r2-20261119-a/review.md, validation.json, decode-review.json, accepted-candidates.json and annotation-batch.json. Eleven independent validators plus primary code/control/stack/bank inspection support all303narrowed redirect candidates; no blanket decode promotion.63extra bytes are9finite low-room1..7source/attribute/metatile/property/marker table slices. All six ready leads verified in leads.json; detailed outcomes in disassembly/r2-20261119-a/lead-outcomes.json. Overlapping graphics/metatile/sprite read views do not assert original serialized objects. BRK2operand barriers retained; unresolved heterogeneous external graphics tails remain unknown.

Gameplay review reused supported rooms1/2/9/18with actual movement/actions and artificial victory dialogue completion; natural terminal/credits and named later-area identities remain unproven in scenarios.json. No useful changed-condition experiment exists for those missing gate/identity contracts in this pass; no repetitive startup or arbitrary room enumeration performed. Next evidence-informed redirect can investigate actual gate/consumer/entry contracts for residuals. This is easy-gain handoff, not terminal exhaustion or whole-ROM classification completion.

Toolkit: existing NES-001documentation/API mismatch remains in analysis/toolkit-issues.md; no new issues, no edits. Data-view client request corrected without state mutation, not a toolkit defect. Prior source review credit retained; next commentary covers new ranges and corrects spriteFFprefix/80terminator and text1return/2..7Fdictionary descriptions. All servers finished; no active writer. About4.4GBfree, no cleanup/delete/archive/publication. Compact handoff: analysis/round2-disassembly-handoff.md.

## Round2 redirect completed — r2-20261119-b supported batch
Inputpass-002-reviewed/mapstateSHA377fd647a853708853b65861cdd40264cc36df0932078f8bd711cc3330d59b78 unchanged. Acceptedclassification82416/131088=62.87%,unknown48672;566researchcandidates29394uniqueunknownbytes(17813Code/11581Data),accepted0. FourreadyIDsL-R2-AUDIO/ACTOR/PATTERN/FLOW appended withrecipes/history; prior6verifiedIDsretained. Exact352rangeledger,kind/perbankcounts,unchangedROM/sourceidentitiesandcandidatechecks inanalysis/redirect/r2-20261119-b/final-audit.jsonandunknown-ledger.json. See report.mdforpostcall/return/bank/SP/inlineproofs,typedbiased/nullselectorlimits,genericDatafallbacksandnegativepremisecorrections; graphnode/officialdecodechecksarenotacceptancealone. Fresh376Codespanboundarychecks pass;35connectedframegrammarszeroerrors. Actualartificialroom7gate->curtain/autowalk/dance->recognizedcyclecounter7->fade->actualscript24->FDC3/titleatframe1877 confirmed; exactpatches/snapshots/fetchesandsuccess/failedpose-onlytests preserved inreport/scenarios. No scriptpointer/gatePCedit; fullnaturalplay/creditsunclaimed. Otheropaqueconsumers/graphics tailsretainlocalizedprerequisites,no terminalexhaustionclaim. Currentfullsourcecommentarygatewasmetbeforephase; finaleditorialauditpending. Independentdisassemblynext,thenfullcommentary(withquarterconstantbranchanddamage-rootmechanicalcorrections). No ROM/map/state/source/toolkitedit,cleanup/deletion/archive/publication; NES-001existingonly. Allserversfinished/noactivewriter.


## Round 3 disassembly completed — incomplete ROM

ROM classification: 85.53% (112126 / 131088 bytes classified; 18962 bytes unknown).

One pass accepted29,710newunique bytes:17,813Code and11,897Data (29,394redirect union plus316connectedData). Internal map totals41,076Code/71,050Data across1,823regions. All new provenance Static; every previous accepted region record and prior full commentary source preserved byte-for-byte. Exact counts/complement163ranges: analysis/round3-counts.json.

| 8 KiB PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 3745 | 925 | 4670 | 3522 |
| 1 | 4235 | 3459 | 7694 | 498 |
| 2 | 1742 | 5596 | 7338 | 854 |
| 3 | 189 | 7976 | 8165 | 27 |
| 4 | 2166 | 4045 | 6211 | 1981 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 229 | 6639 | 6868 | 1324 |
| 8 | 697 | 7436 | 8133 | 59 |
| 9 | 343 | 3041 | 3384 | 4808 |
| 10 | 316 | 7528 | 7844 | 348 |
| 11 | 2791 | 4574 | 7365 | 827 |
| 12 | 6305 | 318 | 6623 | 1569 |
| 13 | 6625 | 649 | 7274 | 918 |
| 14 | 7094 | 618 | 7712 | 480 |
| 15 | 4599 | 1846 | 6445 | 1747 |

Header16Data is included in the total and excluded from the PRG table. Fresh map.validate/save/export and checkpoint **pass-003-reviewed** succeeded; analysis/pass-003-reviewed-verification.json confirms byte-identical rebuilding. Map/stateSHAe71e545e01a1437d4ffbff9ffaef3b80ea2e5e94fe30d1ce7a39f59dbf217ae7; ROM remains37a4fa8b9d4b2b715dcfb244c14e0479ed25385712e1123eb55a8b9719d09c01. Commentary source remainsaf64079029970ec793f30ffd8b7a7f452e7aba5996f4aec96bdac4a9e26e373a; next commentary must preserve/rebase full prior review while explaining new ranges.

Independent structural review: analysis/disassembly/r3-20261119-a/review.md; validation.json, contract-validation.json, decode-review.json, annotation-batch.json and lead-outcomes.json. All four ready IDs L-R2-AUDIO/ACTOR/PATTERN/FLOW verified with exact finite/entry/return/bank/inline/stack contracts.49source references/102roots/203BRKprofiles checked against source/ROM;376Code spans/8268freshinstructions; independent112x6actor partition,55descriptor views,34effects,9445music states/8030readbytes and20pattern records. Specific versus generic Data distinctions retained, including8750tail and888Dprefix; failed pure11bit or first-quarter premises are localized. Additional256byte nibble permutation/four8byte mapper lookup tables/28byte ASCII credit classified with bounded role/context review.

Gameplay checklist reviewed: existing artificial rooms1/2/9/18action and actual room7gate->curtain/autowalk/dance->fade->script24->FDC3/title evidence reused. Named later-area/boss identity and natural full-completion/separate credits need an actual source-backed transition/identity contract; no useful changed recipe was available in this batch. No replay of resolved routes or arbitrary room sweep. This is easy-gain handoff, not terminal exhaustion. Residuals include selected-handler wider domains, saved-PC coroutine context, unselected assets and heterogeneous graphics tails; upcoming readable-source review may expose concrete consumers.

Existing NES-001documentation/API mismatch remains in analysis/toolkit-issues.md; no new toolkit issues or edits. First annotation client attempt incorrectly included execution views on Data; rejected before save/export, transient session discarded, corrected Code-only views with Data read contexts in evidence. This was not a second pass or toolkit defect. No deletion/cleanup/archive/publication. All servers finished/no active writer. Compact handoff: analysis/round3-disassembly-handoff.md.


## Round 4 disassembly completed — incomplete ROM

ROM classification: 99.97% (131051 / 131088 bytes classified; 37 bytes unknown).

Single pass accepted18925newunique bytes=1107Code+17818Data from536research views,444unknown-only canonical annotations. Internal totals42183Code/88868Data across2267regions. All1823previous region records preserved exactly; allnew provenance Static. Exact13range/37byte complement, per-bank counts and identities: analysis/round4-counts.json.

| 8 KiB PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 3775 | 4417 | 8192 | 0 |
| 1 | 4240 | 3952 | 8192 | 0 |
| 2 | 1742 | 6450 | 8192 | 0 |
| 3 | 212 | 7976 | 8188 | 4 |
| 4 | 2166 | 6026 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 250 | 7939 | 8189 | 3 |
| 8 | 697 | 7495 | 8192 | 0 |
| 9 | 343 | 7849 | 8192 | 0 |
| 10 | 316 | 7876 | 8192 | 0 |
| 11 | 3366 | 4822 | 8188 | 4 |
| 12 | 6573 | 1603 | 8176 | 16 |
| 13 | 6638 | 1548 | 8186 | 6 |
| 14 | 7105 | 1086 | 8191 | 1 |
| 15 | 4760 | 3429 | 8189 | 3 |

Header16Data included in total, outside PRG table. Fresh map.validate/save/export and checkpoint pass-004-reviewed verified byte-identical (analysis/pass-004-reviewed-verification.json). Map/state SHAa054cad533065eba0fb67f797975f39b4f6ada358f60518161f9d5fa16f94baa; ROM unchanged. Source/commentary/history untouched, source SHA5215e633bf60f026a995bf7c10e39f8c341e16f12ee20115dd8a476be40333c4. Current full-source readability now needs rebase/review for new classified spans; previous1986reviews/1383Codeunits preserved.

Independent acceptance analysis/disassembly/r4-20261119-a/review.md plus contract-validation.json:45Code views/552fresh instructions,364exact source references,74full numeric/generic records,209sprite views,69roomobjectslots,6motion slots, finite stream/property/stack/vector/wrapper contracts. Allseven L-R3ready IDs verified with attempt histories; no proposed interval rejected. Earlier invalid sprite interpretation, backward-timer premise, global encoded-end rule, consumed-tail-payload inference and broad natural-domain claims remain rejected at their documented scope. Three graphics-tail bytes accepted narrowly as generic local container storage after primaryr1 records.

Remaining37:16heterogeneous852E..853E,sixisolatedRTS,nineoldphysicalFFEBshadowJMPbytes,twoRTI,fourBC11..BC15. Exactresidual primary reasoning remains analysis/redirect/r3-20261119-c/residual-assessment.json. Not terminal exhaustion; no forced classification. Existing broad scenarios including room29event→settledroom7andending-to-title reused; no unchanged gameplay replay/new natural-reachability claim.

Only existing NES-001 in analysis/toolkit-issues.md; no new toolkit issue/edit. No cleanup/deletion/archive/publication/sourceoverwrite. All RPCserversclosed and checkpointprocessfinished. Nextphase fullcurrent-source commentary, including inherited12vs13and20vs21copy-count prose corrections; disassembly did not edit thatsource. Compact handoff analysis/round4-disassembly-handoff.md.

## Round 5 disassembly completed — one pass

ROM classification: 99.98% (131067 / 131088 bytes classified; 21 bytes unknown).

Incomplete. Independently accepted L-R4-PPU:16 unique Static Data bytes, physical99646..99662, CPU852E..853E in effective banks12..15. Two bounded PPUDATA blocks at PPU starts1E00/1E50; exact immutable caller/helper/control-flow/boundaries in analysis/disassembly/r5-20261119-a/review.md and contract-validation.json. Corrected old no-consumer premise without rewriting historical records. All2267 prior region records remain exact; source SHA5a1c69fc6659bf7bceb2bd6956108bab3cfde15efb4ce232bb717fe05e1e8c5f and entire source/review history untouched.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 3775 | 4417 | 8192 | 0 |
| 1 | 4240 | 3952 | 8192 | 0 |
| 2 | 1742 | 6450 | 8192 | 0 |
| 3 | 212 | 7976 | 8188 | 4 |
| 4 | 2166 | 6026 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 250 | 7939 | 8189 | 3 |
| 8 | 697 | 7495 | 8192 | 0 |
| 9 | 343 | 7849 | 8192 | 0 |
| 10 | 316 | 7876 | 8192 | 0 |
| 11 | 3366 | 4822 | 8188 | 4 |
| 12 | 6573 | 1619 | 8192 | 0 |
| 13 | 6638 | 1548 | 8186 | 6 |
| 14 | 7105 | 1086 | 8191 | 1 |
| 15 | 4760 | 3429 | 8189 | 3 |

Header16Data included outside PRG table. Total42183Code/88884Data;2268regions. Exact21unknown/12ranges in analysis/round5-counts.json. No new runtime execution evidence; r4 broad gameplay, source triage, rejection-review and residual ledger reused without unchanged route replay. Structural classification and execution/readability are separate.

Fresh map.validate/save/export, pass-005-reviewed checkpoint and byte-identical rebuild passed: analysis/pass-005-reviewed-verification.json. Map/state SHA65c2cde5d46cf3c6230bbec1704dec5c30a84b5a5c2872e1394db23887b7a8c4. ROM unchanged. Ready lead verified with append-only attempt history; other residual outcomes retained. Only existing confirmed NES-001 in analysis/toolkit-issues.md, workaround buttons arrays; no new issue/blocker. No toolkit edits/deletion/cleanup/archive/publication. No active server/writer.

Next phase full current-source commentary: retain all prior source/history/reviews, rebase onto pass005 and explicitly correct old852Eunknown/no-consumer prose including adjacent initialization-field note; name/explain two bounded PPU pattern blocks. Source readability was current at pass004 but needs refresh for this accepted span. No terminal exhaustion, forced residual classification, natural completion/boss identity or full hardware/game compatibility claim.
