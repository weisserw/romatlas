# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Pass 001 exploratory progression (ongoing)
Natural intro reached bank0 at frame4683, but player-only black screen persists; left input moved sprite without a confirmed ordinary scene. Saved ready.snapshot/game.snapshot permit deterministic branches. Source inspection identified $039F as screen/init-dispatch selector (written by PRG bank4 CPU$8000 graphics loader), not independently established level number. Artificial initializer tests selector1/4/8 with health$392=$80 and PC$8006 produced distinct sprites/encounters but visibly malformed backgrounds; these are attempted transitions and accepted artificial fetch evidence, not verified later-level visits. Further parent-state initialization is required. A direct PC$C0BB top-level route entered another story scene; narrowing to skip its CE9C story helper is a new contract test, not repetition. Exact requests/results/captures under pass-001.

## Pass 001 final supported batch
ROM classification: 55.49% (145482 / 262160 bytes classified; 116678 bytes unknown).
Verified map/current and pass-001-final checkpoint:8,118 code,131,072 CHR image,6,292 data. Exact per-bank counts below and analysis/pass-001/counts.json unknown complement. 74 uniform intervals have exact bytes/boundaries and neighboring context in uniform-runs.json; generic Data claim only after checking discovered instruction conflicts. ScreenInitDispatch physical83..111 is anchored by consumer atCPU$8030/$8035 and indirect call$8040 with independent continuation boundary$805F;14entries,one nullreserved, no natural selector-domain claim. Static control-flow discovery candidates inspected and bank views retained; no mere linear decode of pointer bytes classified as code. Source/rebuild validation passed in analysis/pass-001-final-verification.json.

Progression outcome: parent$C0BE still entered narrative events; narrower$C0DB bypassed those events and set$2B=1 but produced black output and unexpected stack state. This rejects those partial-state recipes as successful scene visits only. Do not infer unsupported emulator defect or inability to reach later scenes. New main-late fetch roots are saved but excluded from final promotion. Further direct-entry edits need scheduler/room/stack preconditions. R001/R002/R003 in leads.json and pass-001/handoff.md provide concrete continuations. No terminal exhaustion assertion; no unchanged failed retry recommended. No toolkit issues found (toolkit-issues.md).
ROM classification: 55.49% (145482 / 262160 bytes classified; 116678 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 2267 | 28 | 5897 |
| PRG01 | 2356 | 1240 | 4596 |
| PRG02 | 108 | 0 | 8084 |
| PRG03 | 4 | 1285 | 6903 |
| PRG04 | 362 | 1430 | 6400 |
| PRG05 | 4 | 281 | 7907 |
| PRG06 | 0 | 1434 | 6758 |
| PRG07 | 4 | 497 | 7691 |
| PRG08 | 1411 | 0 | 6781 |
| PRG09 | 4 | 0 | 8188 |
| PRG10 | 0 | 0 | 8192 |
| PRG11 | 4 | 75 | 8113 |
| PRG12 | 0 | 0 | 8192 |
| PRG13 | 4 | 0 | 8188 |
| PRG14 | 457 | 0 | 7735 |
| PRG15 | 1133 | 6 | 7053 |
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

## Round1 redirect combined batch
analysis/redirect/round1-20261126/findings.md and combined-proposals.json preserve corrected dispatch/graphics/parent/text/actor investigation. Allsixstages, oneboss, fairy narrative and THE END visitedartificially. Proposed 47927bytes;accepted0;currentunknown116678. Whole-bankgenericproposal rejected and narrowed; inlineF61D/FF86 barrierscorrected. No terminalexhaustionclaim.

Round1 redirect final manifest supersedes provisional gain counts:8882 code +41898 data =50780 proposed,0accepted;116678 current unknown;65898 prospective residual if all accepted. Five ready leads. Final artifact/lead/pass consistency checks passed.

## Round 2 disassembly complete — analysis incomplete
ROM classification: 76.13% (199588 / 262160 bytes classified; 62572 bytes unknown).

Accepted54106newbytes (8890code,45216data): all50780reviewed manifest bytes plus3326connected consumer/pattern/loop bytes. Totals17008code,51508data,131072CHRimage;8152/8152regions have evidence. All prior3648regions unchanged. Exact physical unknown complement and per-bank counts: analysis/pass-002/counts.json and final-checks.json. Allfive ready leads R001..R005 consumed/verified; exact per-lead outcomes in lead-outcomes.json. Structural evidence is separate from reachability;8562codebytes/3840instructionstarts have Observed provenance across passes, including artificial routes; whole-game natural execution coverage unclaimed.

Fresh checkpoint maps/pass-002-verified.json, generated/pass-002-verified/game.asm and analysis/pass-002-verified-verification.json passed mapvalidation and byte-identical rebuild (ok=true,byte_identical=true). State/map/checkpoint bytes identical, original inputSHAunchanged. Earlier pass-002-final verified checkpoint retained as intermediate history before graphics gap closure; pass-001-final remains immutable. Maintained commentary/source remains round1 and next phase must migrate/review all current classified source; final editorial audit pending.

Independent stack/bank/consumer/generic review in analysis/pass-002/review.md; primary fresh decodes and exact artificial-fetch matches in instruction-review.json. Consumer extras and graphics closures recorded separately. No new gameplay replay: prior sixstages/laterrooms/boss/THE END coverage was sufficient and reused. Remaining lower-window call-return bank effects, inline selector domains and mixed audio/text/actor fragments need new focused redirect evidence. No terminal exhaustion claim. No toolkit issues found; analysis/toolkit-issues.md. No toolkit/ROM edits, publishing, archive, deletion, cleanup or commentary phase performed.

ROM classification: 76.13% (199588 / 262160 bytes classified; 62572 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 2267 | 28 | 5897 |
| PRG01 | 2439 | 1260 | 4493 |
| PRG02 | 288 | 0 | 7904 |
| PRG03 | 456 | 1285 | 6451 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 4 | 2008 | 6180 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 36 | 979 | 7177 |
| PRG08 | 1939 | 338 | 5915 |
| PRG09 | 4 | 0 | 8188 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 296 | 4907 | 2989 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 4 | 7681 | 507 |
| PRG14 | 4416 | 288 | 3488 |
| PRG15 | 4489 | 320 | 3383 |
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


## Round2 redirect
Combined ready20761byte batch; see analysis/handoffs/round2-redirect.md and analysis/redirect/round2-20261126/contracts.md. Corrected lower-window stack/bank premises, bounded inline dispatch/record domains, mixed audio/sprite islands, fixed actor response handlers and PRG1 room transitions. All new evidence Static; no accepted map changes. Latest manifest, exactunknown ledger and actual deferral reasons preserved. Ready leadsR006..R009; prior outcomes retained. No terminal exhaustion claim.

## Round3 disassembly completed — ROM incomplete
ROM classification: 84.10% (220484 / 262160 bytes classified; 41676 bytes unknown).

Accepted20896newbytes (3944code/16952data):20761reviewed manifest bytes with corrected13E72sprite/transition boundary plus135connected bytes. Original redirect's data/code split was corrected by1byte after independent semantic review; fresh decode agreement alone had not detected the mistaken operand root. Primary contracts, complete pattern/neighbor inspection, bank/stack/inline semantics and consumer behavior supplied role evidence. See analysis/pass-003/review.md, instruction-review.json, helper-contract-review.json, corrected-audio-tail-graph.json, connected-lookups.json and connected-tail-review.json. Original redirect history remains unchanged.

Totals20952code/68460data/131072CHRimage. All10080regions have evidence; all8152baseline regions unchanged. Exact unique physical union and188range complement, per-bank breakdown and lead outcomes in analysis/pass-003/final-checks.json, counts.json and lead-outcomes.json. R006..R009 all verified. New annotations entirelyStatic; observed execution coverage unchanged8562bytes/3840instructionstarts, including saved artificial scenarios, no natural full-game completion claim. Sixstages/laterrooms/boss/fairy/THE END saved recipes reused; no gameplay replay without changed evidence.

Final fresh checkpoint maps/pass-003-final.json, generated/pass-003-final/game.asm and analysis/pass-003-final-verification.json validate and rebuild byte-identically. analysis/state.json/maps/current.json/finalcheckpoint are identical; ROMSHA unchanged. Earlier pass-003-verified checkpoint preserved as intermediate before connected tail closure. No new toolkit issues found; analysis/toolkit-issues.md. Commentary remains preceding round and requires full current-source migration/review next; final editorial audit pending. No toolkit/ROM/maintained commentary edits, archive, publishing, deletion or cleanup. No terminal exhaustion claim.

ROM classification: 84.10% (220484 / 262160 bytes classified; 41676 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 2267 | 28 | 5897 |
| PRG01 | 2439 | 1260 | 4493 |
| PRG02 | 288 | 0 | 7904 |
| PRG03 | 519 | 1321 | 6352 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 36 | 979 | 7177 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 296 | 4907 | 2989 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 4 | 7681 | 507 |
| PRG14 | 4416 | 456 | 3320 |
| PRG15 | 4567 | 593 | 3032 |
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


## Round4 disassembly complete — ROM incomplete

ROM classification: 99.53% (260918 / 262160 bytes classified; 1242 bytes unknown).

Accepted40434newbytes (18349Code/22085Data):40272 independently reviewed R010..R014 proposals plus162connected (112Code/50Data). All five leads verified. Primary consumer, writer, record, stack/inline/bank and pattern evidence reviewed independently; fresh8312instruction decode checks were supplemental. Additional independent loop startsF14B, not initialF149; twozero bytes retained as data. review.md, semantic-review.json, instruction-review.json, connected-review.json and lead-outcomes.json retain decisions and exact bounds. Original evidence/history preserved.

Totals 39301Code/90545Data/131072CHRimage. All18634regions evidence-backed; all10080baseline regions unchanged. Exact physical union, per-bank counts and 34range/1242unknown complement in analysis/pass-004/counts.json/final-checks.json. Execution coverage unchanged8562bytes/3840instructionstarts, including prior artificial scenarios; no new natural-selection or full-game completion claims. Saved sixstage/laterroom/boss/fairy/THE END routes reused; no unchanged replay.

Final fresh maps/pass-004-final.json matches analysis/state.json and maps/current.json exactly, validates and rebuilds byte-identically: analysis/pass-004-final-verification.json, generated/pass-004-final/game.asm. ROM and maintained commentary/source unchanged; full current-source migration/readability review next phase, final editorial audit pending/not run. No toolkit issues found; analysis/toolkit-issues.md. No toolkit/ROM edits, publication/archive/deletion/cleanup. This is not terminal exhaustion.

ROM classification: 99.53% (260918 / 262160 bytes classified; 1242 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6336 | 1855 | 1 |
| PRG01 | 4430 | 3717 | 45 |
| PRG02 | 2693 | 5498 | 1 |
| PRG03 | 2951 | 5228 | 13 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 2247 | 4809 | 1136 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1364 | 6814 | 14 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 322 | 7854 | 16 |
| PRG14 | 7242 | 945 | 5 |
| PRG15 | 5596 | 2590 | 6 |
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

## Round5 disassembly complete — ROM incomplete

ROM classification: 99.80% (261628 / 262160 bytes classified; 532 bytes unknown).

Accepted722reviewbytes:636Code/86Data, comprising710newlyclassifiedbytes (624Code/86Data) and12Data-to-Code correction. Netcodegain636/netdatagain74. R007/R010/R011/R013 independently verified with primary role/boundary/stack/mapper/bank/consumer evidence and301fresh configured-release instruction decodes; legaldecoding not solebasis. review.md/semantic-review.json preserve59directcontrol references,74branch edges, exact domains and oldpalette record. Originalpalette id13964 shrunk60->48with allmetadata/evidence preserved; allother18633priorregions unchanged.

Totals39937Code/90619Data/131072CHRimage; all18950regions evidence-backed. Exactphysicalunion and22interval/532bytecomplement plus perbankcounts saved in counts.json/final-checks.json. Residual502bytelegacy prefix and30tinybytes retain localized code/data/boundary alternatives. Nonterminal; no exhaustionclaim. Executioncoverage unchanged8562bytes/3840observedstarts; allnew/corrected annotationsStatic. Saved broadartificial sixstage/laterroom/boss/fairy/THE END evidence reused; no unchangedgameplay/reachabilitypromotion.

Fresh maps/pass-005-final.json matches state/current exactly, validates and rebuilds byte-identically: analysis/pass-005-final-verification.json; generated/pass-005-final/game.asm. ROM, configuredtoolkit/binary and maintainedcommentary/source unchanged. Fullcurrent-source migration/readability review next; finaleditorialaudit pending/notrun. Lead histories preserved/appended. No toolkitissuesfound; analysis/toolkit-issues.md. No archive/publication/deletion/cleanup.

ROM classification: 99.80% (261628 / 262160 bytes classified; 532 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6336 | 1855 | 1 |
| PRG01 | 4461 | 3724 | 7 |
| PRG02 | 2705 | 5486 | 1 |
| PRG03 | 2954 | 5235 | 3 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 2831 | 4858 | 503 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1367 | 6825 | 0 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 325 | 7865 | 2 |
| PRG14 | 7242 | 945 | 5 |
| PRG15 | 5596 | 2591 | 5 |
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

## Round5 redirect — changed local-prefix inference
Analysis/redirect/round5-20261126/contracts.md and combined-proposals.json: separate actual stack-safe B044RAMfill local subentries and patterned literal storage from invalid copied-inline wrappers. R013ready388Staticnewbytes(272Code/116Data), accepted0. Primary actual-bank callee/operand/stack counterexamples preserved with narrower scope. Exact532/22ledger/currentpass005 identity verified; hypothetical144/38 residual. Independent tiny-source/getter/parent/OAM boundary review retained ambiguity; no arbitrarycaller/semantic prerequisite or forcedclassification. Next independentdisassembly/currentcommentary; notterminalexhaustion. No toolkit issues.

## Round6 disassembly complete — ROM incomplete

ROM classification: 99.95% (262016 / 262160 bytes classified; 144 bytes unknown).

Accepted388newStaticbytes=272Code+116Data after independent actual-bank control/stack/dataflow/boundary/pattern review and138fresh configured-release instruction decodes/141closed edges. Review analysis/pass-006/review.md explicitly assesses F242RTS against the ten-pair family; actualB044 fills finite RAM suffix00,02..07, preserving stack01/bank and possibly overwriting scratch. No renderer/functioning-inline/natural-entry claim. All18950priorregions unchanged; exact union/perbank/complement and observed8562bytes/3840starts unchanged in counts.json/final-checks.json. Remaining144unknown/38intervals=114legacy wrapper/conflict bytes+30tiny ambiguous bytes. Connected/residual alternatives reviewed; no supported extension withheld. Nonterminal, not exhaustion.

Fresh maps/pass-006-final.json equals current/state, validates and rebuilds byte-identically: analysis/pass-006-final-verification.json; generated/pass-006-final/game.asm. ROM/toolkit/binary/maintainedcommentary/source unchanged; current-source migration/readability and finaleditorialaudit pending. R013history preserved/appended; no unchanged gameplay replay, archive/publication/deletion/cleanup. No toolkit issues found; analysis/toolkit-issues.md.

ROM classification: 99.95% (262016 / 262160 bytes classified; 144 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6336 | 1855 | 1 |
| PRG01 | 4461 | 3724 | 7 |
| PRG02 | 2705 | 5486 | 1 |
| PRG03 | 2954 | 5235 | 3 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 3103 | 4974 | 115 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1367 | 6825 | 0 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 325 | 7865 | 2 |
| PRG14 | 7242 | 945 | 5 |
| PRG15 | 5596 | 2591 | 5 |
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

## Round7 disassembly complete — ROM incomplete

ROM classification: 99.95% (262020 / 262160 bytes classified; 140 bytes unknown).

Accepted4newStaticData bytes:1B9F3..1B9F5 independently addressed80 empty-list cells and1F93E..1F940 selected stage6room4 C9/00 parent replacement pair. Independent bank/consumer/pointer/boundary/record/actual-read review:analysis/pass-007/review.md; semantic-review.json. Trailing60 and unrelated80 excluded. All18977priorregions unchanged; exact union/perbank/unknown complement in counts.json/final-checks.json. Remaining140unknown/37intervals=114legacy wrapper/conflict+26tiny ambiguous bytes. Observed instruction coverage unchanged8562bytes/3840starts; no replay. No supported connected extension withheld; nonterminal stopping point.

Fresh state/current/maps/pass-007-final.json identical, validated and byte-identically rebuilt:analysis/pass-007-final-verification.json; generated/pass-007-final/game.asm. ROM/configured binary/maintained source unchanged; R003/R014 outcome history appended. Full current-source commentary and final editorial/terminal rejection audits pending. No toolkit issues found; analysis/toolkit-issues.md. No archive/publication/cleanup.

ROM classification: 99.95% (262020 / 262160 bytes classified; 140 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6336 | 1855 | 1 |
| PRG01 | 4461 | 3724 | 7 |
| PRG02 | 2705 | 5486 | 1 |
| PRG03 | 2954 | 5235 | 3 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 3103 | 4974 | 115 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1367 | 6825 | 0 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 325 | 7867 | 0 |
| PRG14 | 7242 | 945 | 5 |
| PRG15 | 5596 | 2593 | 3 |
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

## Round7 redirect: encoded legacy instruction fragments
Attempt analysis/redirect/round7-20261126; input pass-007-final262020classified/140unknown37intervals. Reopened R013 by correcting the claimed role: actual callee failures reject functioning inline dispatch, not independently templated instruction fragments. Complete accepted bank5 prefix maps+$3FEE to target502bytes:192records,176opcode matches,16targetData barriers116bytes; remaining17intervals114bytes are38exact Code homologs. Fresh bank-aware decodes and exact independent target anchors support ready Static fragment proposal, with actual incompatible callee effects explicitly retained. Full direct-control audit3858edges/43longviews has zero unknown targets; tiny8/16flank counterpart scan finds none. No new emulator experiment or broad gameplay replay. No terminal assertion. Manifest/contracts/ledger/source windows and lead history are saved for independent disassembly. Prospective26unknown20intervals only if accepted; current140/37 unchanged.

## Round8 disassembly complete — ROM incomplete

ROM classification: 99.99% (262134 / 262160 bytes classified; 26 bytes unknown).

Accepted114newStaticCode bytes in17intervals/38instructions as encoded legacy fragments; independent complete192record/502byte homolog role/bounds and actual-bank decoding: analysis/pass-008/review.md; semantic-review.json; instruction-review.json. Actual incompatible A83D/A7F1/AB28/B0A1 behavior unchanged; no working copied wrappers, source callee substitution or natural entry inferred. TwoJMPB044 fragments separately supported by actual valid-stack finite tail. All18979priorregions unchanged. Remaining26unknown/20tinyintervals have no independent homolog/role discriminator; exact complement/perbank in counts.json and unknown-frontier.json. No productive connected extension withheld; no broad gameplay replay. Observed instruction coverage unchanged8562bytes/3840starts.

Fresh state/current/pass-008-final identical, validated, byte-identical rebuild: analysis/pass-008-final-verification.json; generated/pass-008-final/game.asm. ROM/configured binary/maintained source unchanged. R013 history preserved/appended. Full current-source commentary and final editorial/terminal rejection audits pending. No toolkit issues found; analysis/toolkit-issues.md. No archive/publication/cleanup.

ROM classification: 99.99% (262134 / 262160 bytes classified; 26 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6336 | 1855 | 1 |
| PRG01 | 4461 | 3724 | 7 |
| PRG02 | 2705 | 5486 | 1 |
| PRG03 | 2954 | 5235 | 3 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 3217 | 4974 | 1 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1367 | 6825 | 0 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 325 | 7867 | 0 |
| PRG14 | 7242 | 945 | 5 |
| PRG15 | 5596 | 2593 | 3 |
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
