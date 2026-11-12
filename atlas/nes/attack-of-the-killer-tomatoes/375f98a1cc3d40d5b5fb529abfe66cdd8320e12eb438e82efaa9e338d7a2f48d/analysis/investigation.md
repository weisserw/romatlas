# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 completed — incomplete ROM

ROM classification: 59.93% (157105 / 262160 bytes classified; 105055 bytes unknown).

Code:15126; data/image:141979; unknown:105055. All 968 classified regions have evidence annotations. Observed ROM instruction bytes:11045; statically supported code bytes:4081. Execution coverage is separate from structural coverage; no natural reachability claim for artificial patches or static paths. RAM execution retains RAM identity.

Verified baseline and pass-001 retained. Final checkpoint: maps/pass-001-final.json, generated/pass-001-final, analysis/pass-001-final-verification.json. Validation succeeded and full262160-byte generated assembly rebuild is byte-identical. Final map SHA256 a10e5d449f1da16d2fcb15bb5c2ee2514d66c7caae0b953ac076906bf7de2b54. ROM SHA256 unchanged.

Easy-gain pass ends after broad representative states, source-guided shortcuts, bounded mechanic alternatives, vector/observed/static-call batches, tile-format and uniform-data checks. Further work requires indirect-dispatch/upload/actor-initialization research, not repeated early gameplay. This is ordinary easy-gain stopping, not terminal exhaustion. No commentary/redirect work started.

Lead outcomes: L001–L004 verified; L005–L008 blocked by specific contracts, with actionable redirect recipes in analysis/leads.json. Boss and full credits remain unconfirmed; explosion scene is an ending hypothesis only. See analysis/scenarios.json and analysis/gameplay.md for inputs and research uncertainties. Toolkit: no toolkit issues found; analysis/toolkit-issues.md.

Per-bank counts (data includes Image):

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 1034 | 0 | 7158 |
| PRG01 | 24 | 4245 | 3923 |
| PRG02 | 474 | 0 | 7718 |
| PRG03 | 212 | 0 | 7980 |
| PRG04 | 0 | 0 | 8192 |
| PRG05 | 1031 | 114 | 7047 |
| PRG06 | 1930 | 0 | 6262 |
| PRG07 | 2314 | 217 | 5661 |
| PRG08 | 2541 | 1529 | 4122 |
| PRG09 | 0 | 1646 | 6546 |
| PRG10 | 0 | 163 | 8029 |
| PRG11 | 0 | 5553 | 2639 |
| PRG12 | 0 | 960 | 7232 |
| PRG13 | 0 | 2836 | 5356 |
| PRG14 | 3396 | 0 | 4796 |
| PRG15 | 2170 | 1969 | 4053 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 2562 | 1534 |
| CHR05 | 0 | 2464 | 1632 |
| CHR06 | 0 | 2837 | 1259 |
| CHR07 | 0 | 180 | 3916 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |

Compact handoff: analysis/round001/handoff.md. Evidence/snapshots/source remain local. Scratch is small and no cited evidence pruned.

## Round002 disassembly accepted batch

ROM classification: 69.12% (181217 / 262160 bytes classified; 80943 bytes unknown).

Accepted24112 new bytes:6992 Code and17120 Data. All23995 proposed bytes passed independent review; ordinary bank-service continuations add42 Code and adjacent inspected resource data add75 Data. map.validate passed, state/current exported. Primary review: analysis/round002/review.md; exact accepted inventory: accepted.json; exact unknown complement and per-bank counts: counts.json. New annotations Static; prior provenance and commentary/source preserved. No runtime replay or natural-reachability expansion. Checkpoint verification next.

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 1637 | 1253 | 5302 |
| PRG01 | 1339 | 4624 | 2229 |
| PRG02 | 1890 | 1932 | 4370 |
| PRG03 | 1096 | 2384 | 4712 |
| PRG04 | 0 | 0 | 8192 |
| PRG05 | 1031 | 1014 | 6147 |
| PRG06 | 2341 | 36 | 5815 |
| PRG07 | 2527 | 289 | 5376 |
| PRG08 | 3430 | 3455 | 1307 |
| PRG09 | 0 | 8177 | 15 |
| PRG10 | 0 | 163 | 8029 |
| PRG11 | 0 | 5553 | 2639 |
| PRG12 | 0 | 960 | 7232 |
| PRG13 | 0 | 2836 | 5356 |
| PRG14 | 3961 | 75 | 4156 |
| PRG15 | 2866 | 1969 | 3357 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 2562 | 1534 |
| CHR05 | 0 | 4096 | 0 |
| CHR06 | 0 | 2837 | 1259 |
| CHR07 | 0 | 180 | 3916 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |

## Round002 disassembly complete — incomplete ROM

ROM classification: 69.12% (181217 / 262160 bytes classified; 80943 bytes unknown).

Fresh checkpoint pass-002-final validated and rebuilt byte-identical for262160 bytes; state/current/checkpoint SHA256 b87f350b989813988facedc2b41e1bc591cbeeedc0d0a5cab9d0e01f566e944f. Verification: analysis/pass-002-final-verification.json. Exact review/accepted views/counts/unknown complement: analysis/round002/. L005–L011 verified with preserved historical attempts and localized remaining questions. Static new annotations do not expand natural/observed execution claims. Existing commentary source hash37a739f50caa0fe9919aa891eed528de58e59bce6983fc2f0a37b048c2780af1 unchanged; its old-map readability credit needs carry-forward and full current-map review next. Final editorial audit pending.

Ordinary easy-gain stop: saved broad gameplay and new static contracts reused, no unchanged terminal waits. Boss/credits selection, complexA040 schema, unrelated graphics resource endpoints and other mixed CHR formats remain useful future redirect questions. This is not terminal exhaustion. Compact handoff: analysis/round002/handoff.md. No commentary/redirect started. No toolkit issues found; analysis/toolkit-issues.md.8.2GiB free, scratch108KiB. No ROM/toolkit changes, archive, publication or cleanup.

## Round002 redirect — round002-r01
Combined static batch ready L012-L014;40434 distinct proposed unknown bytes,0 accepted. Corrected graphics selector context, exact stream/copy/allocator helper graph and mixedCHR role-localized inference; old primary evidence reused without unchanged gameplay. Exact remaining accepted-map complement80943 bytes/320 intervals retained. Findings and concrete recipes: analysis/redirect/round002-r01/findings.md and handoff.md; proposals and validation alongside them. Narrative success/credits remains mechanically constrained without new discriminating route; no terminal exhaustion claimed.

## Round003 disassembly ACTIVE
Independent review of L012–L014 combined batch; input pass-002-final b87f350b989813988facedc2b41e1bc591cbeeedc0d0a5cab9d0e01f566e944f. Frontier analysis/round003/frontier.md. Source and prior evidence preserved.

## Round003 disassembly accepted batch

ROM classification: 84.55% (221651 / 262160 bytes classified; 40509 bytes unknown).

Accepted40434 new bytes:1065 Code,37081 Data,2288 Image. All510 exact intervals independently reviewed and annotated Static; existing classifications/source unchanged. Evidence: analysis/round003/review.md and independent-review.json; exact accepted views accepted.json; exact complement counts.json. Fresh pass-003-final validates and rebuilds byte-identical.

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 1637 | 1253 | 5302 |
| PRG01 | 1339 | 4624 | 2229 |
| PRG02 | 2338 | 2449 | 3405 |
| PRG03 | 1530 | 2524 | 4138 |
| PRG04 | 0 | 8186 | 6 |
| PRG05 | 1031 | 2393 | 4768 |
| PRG06 | 2341 | 36 | 5815 |
| PRG07 | 2527 | 289 | 5376 |
| PRG08 | 3459 | 3455 | 1278 |
| PRG09 | 0 | 8177 | 15 |
| PRG10 | 0 | 8186 | 6 |
| PRG11 | 0 | 8177 | 15 |
| PRG12 | 0 | 8186 | 6 |
| PRG13 | 0 | 6361 | 1831 |
| PRG14 | 4084 | 1099 | 3009 |
| PRG15 | 2897 | 1985 | 3310 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 4096 | 0 |
| CHR05 | 0 | 4096 | 0 |
| CHR06 | 0 | 4096 | 0 |
| CHR07 | 0 | 4096 | 0 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |

## Round003 disassembly complete — incomplete ROM

ROM classification: 84.55% (221651 / 262160 bytes classified; 40509 bytes unknown).

Fresh checkpoint pass-003-final validated and rebuilt byte-identical for262160 bytes; state/current/checkpoint SHA256 a5fd2a6856cc5964af3fe38872eacf33f3dbea93b29eb983e72f3e0d294477bc. Independent exact Code decoding and primary consumer/resource/CHR review passed; all40434 proposal bytes accepted Static. Source 9925f61269c76ed79ab4cf8b93fb91822fdb855859a8f60b48a14f5176fc9e58 unchanged; full current-map commentary pending for new1065Code+37081Data+2288Image. Lead histories preserved; broad gameplay artifacts reused without unchanged replay. Ordinary easy-gain stop, not terminal exhaustion. Compact handoff analysis/round003/handoff.md. No commentary/redirect started. No toolkit issues found; analysis/toolkit-issues.md. No toolkit/ROM/source edits, archive, publication or cleanup.

## Round003 redirect complete — ready combined batch

Input pass-003-final a5fd2a6856cc5964af3fe38872eacf33f3dbea93b29eb983e72f3e0d294477bc. Accepted classification84.55% (221651/262160);40509 unknown/313 ranges unchanged. Proposed36989 new Static bytes=19506Code+17483Data asreadyL015-L017; accepted new0. Fresh configureddecode8585 exactCodeentries/322 boundedcalls/zeroerrors. Exactledger andcompacthandoff analysis/redirect/round003-r01/handoff.md; contracts/mixed-gap corrections findings.md. Proposal leaves3520 bytes/121subintervals outsidebatch; not terminalexhaustion. Current-source gate andsourcehash d98197e634454628f70120490f5481c472e0505e7cd9ff66f2c629305096b368 unchanged. No runtime visit/map/source/ROM/toolkit edits,archive,publish,cleanup. Next independentdisassemblyreview required. No toolkit issues found.


## Round004 disassembly complete — incomplete ROM

ROM classification: 98.66% (258640 / 262160 bytes classified; 3520 bytes unknown).

Accepted36989 new Static bytes:19506 Code+17483 Data, all8712 independently reviewed exact intervals. Independent configured decoding8585 Code entries/322 calls/zero errors supplements primary scene/actor/stack/bank/geometry/queue/consumer evidence. Review: analysis/round004/review.md and independent-review.json; exact annotations accepted.json; exact complement counts.json. Fresh pass-004-final validates and rebuilds all262160 bytes byte-identical. ROM and entire prior readable source remain unchanged. Full current-map commentary is required for new classifications; final editorial audit remains pending. No new runtime execution or milestone visit; preserved broad progression evidence reused. Ordinary easy-gain stop, not terminal exhaustion; no supported connected continuation omitted at initial bounds. No toolkit issues found; analysis/toolkit-issues.md. No toolkit/ROM/source changes, archive, publication or cleanup.

| Bank | Code | Data/Image/Header | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 3886 | 4040 | 266 |
| PRG01 | 1940 | 5931 | 321 |
| PRG02 | 3809 | 4136 | 247 |
| PRG03 | 2479 | 5709 | 4 |
| PRG04 | 6 | 8186 | 0 |
| PRG05 | 3439 | 4573 | 180 |
| PRG06 | 5534 | 2451 | 207 |
| PRG07 | 6376 | 1207 | 609 |
| PRG08 | 4125 | 3816 | 251 |
| PRG09 | 9 | 8183 | 0 |
| PRG10 | 6 | 8186 | 0 |
| PRG11 | 9 | 8183 | 0 |
| PRG12 | 6 | 8186 | 0 |
| PRG13 | 9 | 8183 | 0 |
| PRG14 | 6174 | 1575 | 443 |
| PRG15 | 4882 | 2318 | 992 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 4096 | 0 |
| CHR05 | 0 | 4096 | 0 |
| CHR06 | 0 | 4096 | 0 |
| CHR07 | 0 | 4096 | 0 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |


Round005 disassembly ACTIVE: analysis/round005/frontier.md. Input pass-004-final; independent contract/context/extent review of L018–L020 underway.


## Round005 disassembly complete — incomplete ROM

ROM classification: 99.99% (262125 / 262160 bytes classified; 35 bytes unknown).

Accepted3485 new Static bytes:3306Code+179Data;1534 exact Code instruction annotations and27 Data intervals. Fresh configured1535-entry/89-call decode zeroerrors included one rejectedRTS; independent355 directROMtargets (threebankcalls resolved explicitly) and ordinary continuations zeroerrors. Review analysis/round005/review.md, reviewed-contracts.json, independent-review.json; exact acceptedrecords accepted.json; perbankcounts/exactcomplement counts.json. Claim109/E5E6 rejected: E5F9/E606 targets areE5E1. A55B..55E genericData claim narrowed; no complete selectedqueue-directory claim. Other mechanical index/stack/carry preconditions corrected. Original proposal/evidence preserved.

Fresh pass-005-final validates and rebuilds all262160 bytes byte-identical; analysis/pass-005-final-verification.json. All35unknown bytes remain60opcode fragments in29ranges; neither code nor padding inferred. Entire prior currentcommentarysource preserved (SHA256 52a3bc9c124b14652b9a19261aa4b7c71ed08f264bd5da77850dc69fd046f9f5), but new3306Code/179Data require full current-source commentary. Final editorial audit pending. Existing input unchanged. Structural classification is distinct from execution coverage: no new execution observations/route replay, historical ObservedCode structural bytes 11045. Annotation coverage: {'evidence_bytes': 262125, 'comment_bytes': 254060}. Prior broad progression/scenarios retained; no new concrete successful boss/credits condition. Ordinary easy-gain stop, not terminal exhaustion. No toolkit issues found; analysis/toolkit-issues.md. No toolkit/ROM/source changes, archive, publication, cleanup or nextphase work.

| Bank | Code | Data/Image/Header | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 4117 | 4074 | 1 |
| PRG01 | 2216 | 5969 | 7 |
| PRG02 | 4035 | 4148 | 9 |
| PRG03 | 2479 | 5709 | 4 |
| PRG04 | 6 | 8186 | 0 |
| PRG05 | 3592 | 4595 | 5 |
| PRG06 | 5729 | 2463 | 0 |
| PRG07 | 6949 | 1243 | 0 |
| PRG08 | 4371 | 3820 | 1 |
| PRG09 | 9 | 8183 | 0 |
| PRG10 | 6 | 8186 | 0 |
| PRG11 | 9 | 8183 | 0 |
| PRG12 | 6 | 8186 | 0 |
| PRG13 | 9 | 8183 | 0 |
| PRG14 | 6607 | 1584 | 1 |
| PRG15 | 5855 | 2330 | 7 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 4096 | 0 |
| CHR05 | 0 | 4096 | 0 |
| CHR06 | 0 | 4096 | 0 |
| CHR07 | 0 | 4096 | 0 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |

Round005 redirect round005-r01: ready L021 sound27-installed98EC one-byte callback and L022 staleB2FA-failure annotation correction. Constructed pointer producer9791 storage/9614 consumer discriminates isolatedRTS; prior trigger-only negative evidence had narrower scope. Independent immediate/count-delimited/reference/neighbor checks and scene writerSTX86C9 review preserved in analysis/redirect/round005-r01/findings.md. No other productive identified continuation deferred; accepted state unchanged35unknown/29ranges. Not terminal exhaustion. Next independent disassembly review combined batch.


## Round006 disassembly complete — incomplete ROM

ROM classification: 99.99% (262126 / 262160 bytes classified; 34 bytes unknown).

L021 verified one new Static Code byte, physical71932 CPU$98EC effective banks[8,9,14,15]. Independent configured decoding matched producer/setter/dispatch/RTS primary contracts; $98ED remains existing APU Data. Natural selection and observed execution unresolved. L022 verified metadata correction:61 existing Code comments/132bytes, only false failure-TAX wording replaced. Every old ID, bounds, kind, provenance, confidence, execution view, evidence and label preserved. Historical round005 review/contract records remain immutable; review.md appends the erratum. Audit annotation-corrections-applied.json and accepted.json. All connected sound continuations already classified; no additional supported interval.

Incomplete:34 Unknown bytes/28 ranges; exact complement/counts in counts.json, inherited scoped neighbors in unknown-ledger.json. Remaining standalone RTS ambiguity remains; no Data inference from absent calls, no Code inference from opcode alone. Prior direct-edge, immediate-pair, actual count-delimited entry-minus1 and unaligned-word negatives retain only their stated scope (analysis/redirect/round005-r01/findings.md and primaries). No unchanged runtime/waits or arbitrary RAM experiments. Prior broad gameplay checklist/negative ending attempts preserved; no changed source-supported successful scenario.

Configured validation/save/export and fresh unique pass-006-final byte-identical rebuild of262160bytes passed; analysis/pass-006-final-verification.json. Source unchanged df9559083a4d1fd1d42e789b60e7a4dc7a20e86bee4930370ecae49fe91233bc; no source editing occurred. Full current-source commentary must follow for new byte and metadata consistency, retaining complete current source/history; final editorial audit pending. No next phase, terminal exhaustion, toolkit modification, publication, archive or cleanup. No toolkit issues found; analysis/toolkit-issues.md.

Annotation coverage: {'evidence_bytes': 262126, 'comment_bytes': 254061}. Historical Observed Code bytes 11045, new execution observations0.

| Bank | Code | Data/Image/Header | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 4117 | 4074 | 1 |
| PRG01 | 2216 | 5969 | 7 |
| PRG02 | 4035 | 4148 | 9 |
| PRG03 | 2479 | 5709 | 4 |
| PRG04 | 6 | 8186 | 0 |
| PRG05 | 3592 | 4595 | 5 |
| PRG06 | 5729 | 2463 | 0 |
| PRG07 | 6949 | 1243 | 0 |
| PRG08 | 4372 | 3820 | 0 |
| PRG09 | 9 | 8183 | 0 |
| PRG10 | 6 | 8186 | 0 |
| PRG11 | 9 | 8183 | 0 |
| PRG12 | 6 | 8186 | 0 |
| PRG13 | 9 | 8183 | 0 |
| PRG14 | 6607 | 1584 | 1 |
| PRG15 | 5855 | 2330 | 7 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 4096 | 0 |
| CHR05 | 0 | 4096 | 0 |
| CHR06 | 0 | 4096 | 0 |
| CHR07 | 0 | 4096 | 0 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |

## Round006-r01 terminal review proposed

Exact current residual34bytes/28ranges refreshed from map; all short$60 fragments. Current-source boundaries and15 indirect consumer neighborhoods checked; prior pointer and gameplay negatives retrieved with scoped conclusions. Repetition and arbitrary returning ABI do not discriminate empty entries from stored separators; no independent residual-specific structural inference survives. No new actionable lead or runtime replay. See analysis/redirect/round006-r01/rejection-review.md for candidate claims, primary evidence, competitors and stopping reasons; coordinator acceptance required before terminal assessment is final. Broad stage0A/ending shortcut review appended to analysis/scenarios.json. Input pass-006-final unchanged;0proposed/accepted bytes. Future specific producers/endpoints/data evidence may reopen; no unchanged retries.
