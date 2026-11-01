# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 01 disassembly handoff — incomplete
ROM classification: 57.74% (151364 / 262160 bytes classified; 110796 bytes unknown).

Code 15706; non-code 135658. Provenance bytes: {'Static': 140886, 'Observed': 10478}.
Header:16 data; CHR:131072 image,0 unknown. PRG per physical8KiB bank:

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| PRG 00 | 0 | 0 | 8192 |
| PRG 01 | 2090 | 0 | 6102 |
| PRG 02 | 546 | 0 | 7646 |
| PRG 03 | 0 | 517 | 7675 |
| PRG 04 | 0 | 0 | 8192 |
| PRG 05 | 0 | 102 | 8090 |
| PRG 06 | 2659 | 30 | 5503 |
| PRG 07 | 498 | 103 | 7591 |
| PRG 08 | 1015 | 0 | 7177 |
| PRG 09 | 641 | 712 | 6839 |
| PRG 10 | 2952 | 32 | 5208 |
| PRG 11 | 44 | 0 | 8148 |
| PRG 12 | 0 | 0 | 8192 |
| PRG 13 | 800 | 605 | 6787 |
| PRG 14 | 892 | 10 | 7290 |
| PRG 15 | 3569 | 2459 | 2164 |

Exact unknown complement: analysis/evidence/round01/unknown-final.json (273 gaps).
Every classified region has structural evidence; code execution views are explicit.
Execution coverage separately: 5091 distinct physical accepted ROM instruction
roots across saved scenarios (mixed natural/artificial); it is not byte classification.

Final validation/rebuild: analysis/pass-001-final-verification.json; maps/current.json
is byte-for-byte the checkpoint map maps/pass-001-final.json, SHA256
687e9f29ec9f58f16b94a00c952536b7f9464a679019f8d37fe13100509a097b.
Generated current classified assembly: generated/pass-001-final/game.asm.
Baseline and interim checkpoints retained. Input ROM SHA256 unchanged.

Inline correction: first pre-repair snapshot had13764 code bytes; first repair
removed inline fallthrough and rederived13753 code bytes from accepted roots.
Later handler expansion exposed3 more sites; second pre-repair snapshot had15356
code bytes. Final root rederivation14528 plus1178 supported static handler bytes
=15706 code bytes. These totals involve differing scenario/root sets; net
difference is not the number of rejected table bytes. Eleven E116 call sites
audited; their11 barriers total188 bytes (tables/partial supported slices).
No additional unchecked E116 sites remain in accepted code.

Gameplay checklist and recipes: analysis/scenarios.json. Natural startup and
stage1; artificial stage2 shooting,stage4 driving,stage6 city,stage9 mansion/boss
meter/combat and ending dialogue. No natural late-game or boss-defeat claim.
Credits continuation produced dialogue with zero gain; a second input trial was
interrupted after its bounded wait and is explicitly attempted, not visited.
Remaining similar-stage variants/practice/sound deferred to focused questions,
not a claim of gameplay exhaustion. No terminal exhaustion claimed; this is an
easy-gain handoff with concrete redirect leads.

Toolkit:1 missing feature NES-001, no confirmed/suspected bugs or blockers;
analysis/toolkit-issues.md. Current source needs full commentary; labels mostly
structural fallback names. No archive/publication/deletion/toolkit edits.

Lead outcomes and recipes in analysis/leads.json; bounded evidence inspection lists retained under analysis/evidence/round01.

## Round 02 disassembly complete — incomplete classification

ROM classification: 100.00% (262153 / 262160 bytes classified; 7 bytes unknown).

The displayed percentage rounds from 99.99732987%; seven unknown bytes remain, so this is incomplete. Code39602; non-code222551. New coverage:23909 Code +86880 Data =110789 previously unknown bytes; C01 additionally corrects13 existing Static Code bytes toData. All new claims are Static; previous Observed evidence preserved. Provenance bytes: {'Static': 251675, 'Observed': 10478}.

| Physical PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 00 | 0 | 8192 | 8192 | 0 |
| 01 | 2547 | 5645 | 8192 | 0 |
| 02 | 549 | 7643 | 8192 | 0 |
| 03 | 457 | 7735 | 8192 | 0 |
| 04 | 253 | 7939 | 8192 | 0 |
| 05 | 8 | 8184 | 8192 | 0 |
| 06 | 6505 | 1683 | 8188 | 4 |
| 07 | 6470 | 1722 | 8192 | 0 |
| 08 | 7198 | 994 | 8192 | 0 |
| 09 | 1146 | 7046 | 8192 | 0 |
| 10 | 6451 | 1740 | 8191 | 1 |
| 11 | 245 | 7947 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 1543 | 6648 | 8191 | 1 |
| 14 | 1204 | 6988 | 8192 | 0 |
| 15 | 5026 | 3165 | 8191 | 1 |

Header16Data andCHR131072Image remain classified. Unknown half-open physical intervals: $C1E2..$C1E5,$C7A6..$C7A7,$14379..$1437A,$1ACD0..$1ACD1,$1EE00..$1EE01. Exact machine-readable complement: /root/romatlas-batch/a6a548c4a27d3961821312a95e45ecda0eea36e247ee2d963079c336b23882c2/workspace/analysis/disassembly/round02-20261031-a/unknown-final.json.

Eleven ready leads verified: R01-L01, R01-L03, R01-L05, R01-L06, R01-L07, R01-L08, R01-L09, R01-L10, R01-L11, R01-L12, R01-L13. R01-L14 retains five role ambiguities, independently reviewed against changed premise of now accepted full neighboring graphs. Primary evidence and reasons: /root/romatlas-batch/a6a548c4a27d3961821312a95e45ecda0eea36e247ee2d963079c336b23882c2/workspace/analysis/disassembly/round02-20261031-a/review.md; fresh consumer decoding and independent full format checks in same directory. Counts/conflict audit alone were not acceptance evidence. All14E116 barriers preserved; shooting state0 overlaps excluded; four flag-proved branches retain constant boundaries; footer/raster template role separated from runtime fetch. Generic encoded/script/constant data accepted after bounded consumer/reference/format inspection without inventing schema/sample identities.

Saved state and map.validate successful; maps/current.json andanalysis/state.json match fresh maps/pass-002-disassembly-20261031-a.json, SHA256 ff89defe97828358320b8a0b360c796abc3e5924cd63b1894c98f36c6fd6c359. Fresh generated/pass-002-disassembly-20261031-a byte-identical rebuild verified inanalysis/pass-002-disassembly-20261031-a-verification.json. Original ROM hash unchanged. Entire28-file commentary copy preserved unchanged, including high-quality round01source/readability evidence; new classified source requires commentary carry-forward and extension.

Gameplay checklist reviewed: three mechanics, latercity/mansion/bosscombat, movingcredits,THEENDandStartexit already represented by saved natural/artificial recipes. No unchanged replay; similar-stage/practice variants have no concrete useful residual-entry hypothesis this pass. Natural late-game/bossdefeat remains unproved. Easy-gain single pass ends incomplete; full current-source commentary then redirect review are next, not terminal coordinator acceptance.

Toolkit: existingNES-001 missingfeature only; no new confirmed/suspected bugs or blockers. See analysis/toolkit-issues.md. No toolkit edit, archive, publication, deletion, cleanup or baseline reset. Local artifacts retained; free 9.63GiB, scratch 587.5KiB.


## Round 03 disassembly complete — incomplete classification

ROM classification: 100.00% (262156 / 262160 bytes classified; 4 bytes unknown).

Exact percentage 99.99847421%; rounded100.00% is not completion. Accepted only
R01-L14 narrowed physical C1E2..C1E5 as Static Code BrawlEnterGameOverState,
CPU81D2/[6,7,14,15]. Combined independent structural review and fresh immutable
configured-binary decoding: analysis/disassembly/round03-20261031-a/review.md and
independent-primary.json. Delegated state3/substate0/ordinary-return contract,
restart/completion family, exact neighbors and accepted matching1E32F/1EAF2 tails
support this inference. Actual entry/natural selection remain unresolved.
Gain3Code; Code39605/Data91479/Image131072. Observed coverage unchanged.

| PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 00 | 0 | 8192 | 8192 | 0 |
| 01 | 2547 | 5645 | 8192 | 0 |
| 02 | 549 | 7643 | 8192 | 0 |
| 03 | 457 | 7735 | 8192 | 0 |
| 04 | 253 | 7939 | 8192 | 0 |
| 05 | 8 | 8184 | 8192 | 0 |
| 06 | 6508 | 1683 | 8191 | 1 |
| 07 | 6470 | 1722 | 8192 | 0 |
| 08 | 7198 | 994 | 8192 | 0 |
| 09 | 1146 | 7046 | 8192 | 0 |
| 10 | 6451 | 1740 | 8191 | 1 |
| 11 | 245 | 7947 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 1543 | 6648 | 8191 | 1 |
| 14 | 1204 | 6988 | 8192 | 0 |
| 15 | 5026 | 3165 | 8191 | 1 |

Header16Data andCHR131072Image also classified. Exact half-open Unknown intervals:
$C7A6..$C7A7,$14379..$1437A,$1ACD0..$1ACD1,$1EE00..$1EE01.
Four isolated RTS retain code-versus-data uncertainty; no forced classification.
No connected downstream unknown: common setter and surrounding handlers classified.
Broad gameplay reviewed/reused, including three mechanics/later areas/boss combat/
credits/THE END/Start exit; no unchanged replay or unearned natural-reachability claim.

Fresh final checkpoint pass-003-disassembly-20261031-b, successful map validation,
byte-identical rebuild: analysis/pass-003-disassembly-20261031-b-verification.json.
Map/state SHA256 8fac0ec90e59abe29b84a73cee9875b8e89ff5d7c5841e626579807c6a56138c. Original ROM unchanged.
The earlier pass-003-disassembly-20261031-a preserved an unchanged-map checkpoint
after a rejected unsaved annotation lacking required id; final b is authoritative.
This was caller parameter error, not a toolkit bug. No classification was lost.
Final counts, hashes, bank ledger, commentary preservation: analysis/disassembly/round03-20261031-a/final-audit.json.
All39 existing commentary files unchanged; full current-source commentary must carry
forward prior prose, explain new3byte unit and repair stale ContinueRestart dependency
claim that following classified bytes are unknown. No historical source/evidence removed.

Toolkit: existing NES-001 missing feature only; no new confirmed/suspected bugs or
blockers. See analysis/toolkit-issues.md. No toolkit edits, baseline reset, archive,
publication, cleanup or deletion. One disassembly pass complete; subsequent commentary
and redirect/coordinator rejection review remain, not terminal acceptance.

Round03 redirect (round03-20261031-a) closes with no ready classification lead and
four separately reviewed isolated60 ambiguities. Primary rejection review/current
ledger and accepted-C1E2 comparison saved in attempt; handoff-round03-redirect.md
requests coordinator terminal-reasoning inspection. 24 concrete stale prose claims
found and recorded in editorial-carry.json for final editorial repair; source unchanged.
No useful current experimental frontier remains; no unchanged runtime replay.
