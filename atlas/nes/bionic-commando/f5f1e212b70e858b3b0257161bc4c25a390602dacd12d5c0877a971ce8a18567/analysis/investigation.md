# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round001 initial disassembly batches

Baseline exact rebuild established; observed-root contextual startup plus inspected
direct fixed-bank call targets classified3600 code bytes. Decode artifacts under
analysis/evidence/round-001: startup-decoded.json and expanded-new-decoded.json.
Call barriers preserve unknown inline tables after $D6A0 stack dispatcher. Direct
JSR targets are supported entry contracts independent of natural reachability.
12 full8KiB banks of aligned NES2bpp tiles (banks8..19) inspected end-to-end as
contact sheet banks08-19-tiles.png; consumer graphics-transfer-decode.json copies
16-byte blocks to PPUDATA and derives source bank from record bits. Classified
98304 Image bytes. Header16 and NMI/reset/IRQ vectors6 classifiedData.
Total101926; unknown160234. New static code includes reset $FFE0, sprite rendering,
room structure consumers, bank serializers and graphics/text paths.

No ready input leads existed (analysis/leads.json initially empty). Broad gameplay
identified English Bionic Commando; slow communications opening, overworld prompt
reached. $CB/$CC/$CD are nested state selectors through inline stack dispatch;
$4C indexes room screen pointer table, $75 another room-dependent graphics selector.
Need establish safe stage transition before changing arbitrary selectors.
Current map prompt tests B-newpress ($F8 & $40) in bank27 $BF3F; freshB pending.
Do not infer reachability of pointer targets or classify inline table bounds solely
from observed selector. Toolkit docs budget mismatch recorded, no blocker.

### Room/resource and late-state review

Screen-cell data0x610..0x6010 and tile structures0x8810..0xEC10: every256-byte
block reviewed with saved samples/statistics(data-pattern-review.json); repeated
low-domain grids/four-tile attribute-like records, clear exact data boundaries,
no conflicting code. Existing uniform classifications preserved, newly classified
unknown complements41972bytes. Specific original schema is not asserted. Adjacent
0xEC10 onward contains code and stays separated; banks20/21 also contain code.

Broad artificial progression proved initialized area1, area12 exterior, neutral13,
special room 96 interior, room98 exterior and room 101 mechanical helicopter boss.
Each probe restores pre-shortcut-snapshot, patchesarea$4D,room$4C,$3F=0 and
$CC..CF=[3,0,0,0], waits100 frames untraced then exercisesright+B for 8 frames,right+A for 8 frames traced.
Accepted roots/contexts, candidate sets and snapshots saved per milestone. Normal
map selection remains unresolved; no natural late-game reachability claim.

Connected direct-call static expansion:150 explicitJSR targets from validated
Code regions under their execution views; preserve only resolved effective bank
identity, no presumed return after calls, no pointer target promotion. Inspected
new connected and late-runtime batch1118Codebytes in54groups(late-static-new-decoded.json).

CB4 main dispatcher experiment: initialized wounded-person communications screen;
fiveA180/release2 pulses reached black scene,[CB,CC,CD,CE,CF]=[4,0,1,1,2]. Its
identity as ending/intro remains unresolved. Bounded longwait pending; no speculative
classification or ending claim. Room101 gunshot attempted fromsource-supported
player X$056F=128,Y $05A9=64,health $0779=32:player fell andboss remained. Revised
weapon $04CA=5 test usesthe manual’s sixth weapon, Hyper Bazooka and needsvisual confirmation.
Do not repeat the unchanged normal gun attempt; investigate weapon/entrance/cockpitcontract.

## Round 001 disassembly phase closed — ROM incomplete

ROM classification: 61.42% (161031 / 262160 bytes classified; 101129 bytes unknown).

Final internal state: analysis/state.json; supported portable map: maps/current.json.
Fresh verified checkpoint: maps/round-001-final-tables.json,
generated/round-001-final-tables/game.asm,
analysis/round-001-final-tables-verification.json. Map validation passed and rebuild
is byte-identical. Input ROM SHA-256 remains f5f1e212b70e858b3b0257161bc4c25a390602dacd12d5c0877a971ce8a18567.
All 547 classified regions (161031 bytes) have evidence annotations.
Kinds: Code 8474, Data 53229, Image 99328. Static provenance is
intentional: structural classification is separate from observed reachability.
Saved execution evidence union: 2234 unique physical opcode starts,
2831 contextual starts. Natural startup/menu observations and
artificial later-game observations are separated by their scenario records; untraced
waits are excluded. This is not a whole-ROM execution percentage.

Final small batch added 125 cinematic code bytes and 32 inline dispatcher-pointer
bytes. Dispatcher tables have independently anchored adjacent code boundaries; no
claim all selectors occur naturally. Detailed exact unknown complement (203 prior
gaps, now updated) and per-bank metrics: evidence/round-001/unknown-ranges.json and
final-counts.json. The remaining code/data distinction in mixed banks and indirect
handler contracts need redirect research; no terminal exhaustion claim is made.

Latest cinematic snapshot cinematic-late-snapshot.json at frame6957 has selectors
[4,0,1,4,3], wait byte$74=0, bank context[24,25,30,31]. Base explosion and helicopter
rescue were visited; credits/terminal screen remain unverified after bounded waits.
Further long waiting or repeated cockpit shots is deferred until redirect resolves
the specific late-cinematic/player-entry contract. Representative area1, area12,
neutral13, special rooms96/98/101 and their mechanics have reproducible local routes.

Toolkit issues: two confirmed bugs/documentation defects, no blockers; see
analysis/toolkit-issues.md (NES-DOC-001 request budget naming; NES-OBS-002 observed
provenance fields). No toolkit edits, publishing, cleanup or deletion performed.
Hand off to commentary for all currently classified source, then redirect.

| Effective 8 KiB bank | Code | Data/Image | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 6656 | 6656 | 1536 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 111 | 300 | 411 | 7781 |
| 4 | 0 | 6177 | 6177 | 2015 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 29 | 3137 | 3166 | 5026 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 0 | 0 | 8192 |
| 21 | 303 | 64 | 367 | 7825 |
| 22 | 0 | 102 | 102 | 8090 |
| 23 | 0 | 3068 | 3068 | 5124 |
| 24 | 1968 | 0 | 1968 | 6224 |
| 25 | 0 | 719 | 719 | 7473 |
| 26 | 25 | 729 | 754 | 7438 |
| 27 | 1265 | 97 | 1362 | 6830 |
| 28 | 45 | 0 | 45 | 8147 |
| 29 | 0 | 192 | 192 | 8000 |
| 30 | 4003 | 32 | 4035 | 4157 |
| 31 | 725 | 196 | 921 | 7271 |

### Round001 redirect / round-001-a

Revisited caller/selector-only barriers using structural contracts, independently
bounded records and clear generic non-code evidence. Followed all connected actor,
player, audio, cinematic and helper graphs and newly revealed bank3/11 consumers.
Primary review corrected D7E1 inline barrier, rejected A9B5 operand-entry hypothesis,
split actual bank23 code out of generic resource proposal, and localized E000 trap.
Combined ready proposal95126 bytes; accepted0, actual unknown101129 unchanged.
See analysis/redirect/round-001-a/research.md and proposal.json; exact residual ledger
preserved. Artificial terminal signature visited through documented timer/gate edits;
room101 kind70 health39 premise corrected. No terminal exhaustion assessment.

## Round 002 disassembly complete — substantial reviewed batch

ROM classification: 97.85% (256535 / 262160 bytes classified; 5625 bytes unknown).

Gain95504 bytes:35277 Code and60227 Data, including375 additional bounded
residual-resource bytes. Total Code43751, Data113456, Image99328; header included
in Data. All previous regions preserved unchanged. The main95129-byte batch
corrected the B07A area-transfer island (241 bytes Data->Code) and added C6AF's
three-byte fixed vector. Generic audio data remains supported while eleven unused
cinematic-constant pointer-table claims were discarded. Evidence/consumer details:
analysis/evidence/round-002/review.md, independent-review.json, transfer-graph.json,
annotation-results.json and residual-data-results.json. R001-REDIRECT-CODE and
R001-REDIRECT-DATA verified with these localized corrections.

Both fresh checkpoints round-002-disassembly and round-002-final validated and
rebuilt byte-identically. Final report analysis/round-002-final-verification.json;
map/state SHA256 8b9f176e93f6c9b137dee1440e49653ab5916fe0e1d2380cc255db2385825b65.
ROM SHA256 unchanged f5f1e212b70e858b3b0257161bc4c25a390602dacd12d5c0877a971ce8a18567.

Current-source commentary remains the preserved round001 work; this new95504-byte
classification requires the next commentary phase to complete current readability.
Whole-ROM classification/editorial completion remains incomplete. No terminal
exhaustion claim: refreshed exact residual ledger has201 intervals/5625 bytes,
including unrooted code-shaped bodies, consumer/table gaps and mixed footer guards.
See analysis/handoffs/round-002-disassembly.md for concrete redirect questions.
Broad artificial terminal visits/routes retained; no unchanged replay performed.
Toolkit issues NES-DOC-001/NES-OBS-002 remain in analysis/toolkit-issues.md; no new
toolkit issues found this pass. No toolkit/ROM edits, deletion, archive or publish.

| Physical 8KiB bank | Code | Data/Image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1368 | 6792 | 8160 | 32 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 421 | 7211 | 7632 | 560 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1186 | 6914 | 8100 | 92 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 594 | 7310 | 7904 | 288 |
| 24 | 2329 | 5805 | 8134 | 58 |
| 25 | 0 | 8160 | 8160 | 32 |
| 26 | 6144 | 1270 | 7414 | 778 |
| 27 | 6362 | 702 | 7064 | 1128 |
| 28 | 6361 | 904 | 7265 | 927 |
| 29 | 7096 | 797 | 7893 | 299 |
| 30 | 7362 | 248 | 7610 | 582 |
| 31 | 4528 | 2815 | 7343 | 849 |


Round002 redirect-a: configured static inspection of201 residual intervals/5625 bytes; coherent scene/grapple/fixed continuation batch and independent resource review propose916 Code+4426 Data=5342 bytes, accepted0. Conditional residual283. Primary analysis/redirect/round-002-a/research.md, proposal.json and exact residual-ledger.json. Ready R002-REDIRECT-CODE/DATA for independent disassembly. No classification/source/ROM/toolkit changes; not terminal exhaustion.


## Round003 disassembly completed — round-003-classified

ROM classification: 99.89% (261877 / 262160 bytes classified; 283 bytes unknown).

Accepted R002-REDIRECT-CODE916 bytes and R002-REDIRECT-DATA4426 bytes Static after independent configured decode/stack/mapper/inline barriers and every generic-data boundary review. Fresh412 instructions, all external edges on accepted instruction grids. Full connected scene/grapple/initializer/helper batch retained; no proposal rejection.24 exact Unknown intervals remain; counts.json and unknown-ranges.json in analysis/evidence/round-003. Header16 classified separately; data/other includes image/pointer/palette roles. No new execution coverage or natural reachability claim. Reused prior artificial broad gameplay and terminal evidence; no unchanged replay.

Evidence/rejection scope/next frontier: analysis/evidence/round-003/review.md; handoff analysis/handoffs/round-003-disassembly.md. map.validate passed; fresh save/export and byte-identical checkpoint verification analysis/round-003-classified-verification.json. Current map/stateSHA29e4403b8b27d1c3ae75a0aec4c7214f1191a434339eca88ecdb9a1fb51359c9. round-003-final is preserved unchanged preacceptance verification, not current. All prior commentary preserved; full current-source commentary next. No ROM/toolkit/source edits, publication/archive/deletion. No new toolkit issues found; analysis/toolkit-issues.md retains two confirmed documentation bugs and one provenance bug, none blocking.

| Bank | Code | Data/other | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1368 | 6820 | 8188 | 4 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 421 | 7767 | 8188 | 4 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1186 | 7002 | 8188 | 4 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 594 | 7585 | 8179 | 13 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 0 | 8188 | 8188 | 4 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6587 | 1601 | 8188 | 4 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7107 | 1077 | 8184 | 8 |
| 30 | 7527 | 531 | 8058 | 134 |
| 31 | 4622 | 3470 | 8092 | 100 |


## Round004 disassembly completed — round-004-classified

ROM classification: 99.92% (261947 / 262160 bytes classified; 213 bytes unknown).

Accepted R003-RESET-ALIASES28, R003-HEADER-PUBLISHER21 and R003-TYPE-FILTER-SUFFIX21 Static Code bytes after independent fresh decoder, mapper/read/write/next-fetch and consumer/branch boundary review. Natural entry remains unresolved; no new execution coverage. Exact213 Unknown/17 intervals. Evidence: analysis/evidence/round-004/review.md, fresh-decode.json, target-grids.json, counts.json and unknown-ranges.json. map.validate passed; saved/exported map SHA ef40b6c7a98f2ad5f37da8693564ee177472e03a8869dde0d795f711d54331c7; fresh checkpoint byte-identical (analysis/round-004-classified-verification.json). Header16 classified separately. Full current-source commentary remains next; previous commentary unchanged. Broad prior artificial progression/terminal evidence reused, no changed replay needed. Incomplete; unresolved frontier remains renderer59, counter50, arithmetic64, wrapper/vector14, trap2 and isolated fragments24. No concrete connected extension omitted; local contradictions and ambiguity documented in review. No new toolkit issues found; analysis/toolkit-issues.md preserves two documentation bugs/one provenance bug, none blocking. No ROM/toolkit/source edits, archive/deletion/publication.

| Bank | Code | Data/other | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7548 | 531 | 8079 | 113 |
| 31 | 4643 | 3470 | 8113 | 79 |


## Round005 disassembly completed — round-005-classified

ROM classification: 99.94% (262012 / 262160 bytes classified; 148 bytes unknown).

Accepted R004-PRESERVED-TILE-ATTRIBUTE-FRAGMENT48 and R004-PRESERVED-SHIFT48-LOOP17 as Static Code after independent fresh decode, exact branch and closed-loop review; natural reachability unresolved. Full evidence: analysis/evidence/round-005/review.md, fresh-decode.json, target-grids.json, counts.json and unknown-ranges.json. Header16 separately classified. map.validate passed; saved/exported map SHA92909a271b71405418235187a087b923aa6930b0c5ffd9260d7d70f9f7d2df85. Fresh checkpoint byte-identical: analysis/round-005-classified-verification.json. Exact complement148/19intervals. Prior commentary preserved unchanged; full current-source commentary next. Reused broad artificial terminal milestone, no changed replay. Remaining renderer11,counter50,arithmetic47,wrapper/vector14,trap2,isolated24. No supported connected extension omitted. Incomplete; structural reachability and final editorial gates remain separate. No new toolkit issues; analysis/toolkit-issues.md preserves three prior nonblocking findings. No toolkit/ROM/commentary source edits, archive, deletion or publication.

| Bank | Code | Data/other | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7596 | 531 | 8127 | 65 |
| 31 | 4660 | 3470 | 8130 | 62 |


## Round006 — independent arithmetic disassembly pass

ROM classification: 99.96% (262055 / 262160 bytes classified; 105 bytes unknown).

Accepted43 Static Code bytes: physical3E9C2+21 trial24bit subtraction and3E9D9+22 inverse register commit/carry addition, fixed slots30/31. R005-TRIAL-SUB24 and R005-COMMIT-ADD24 verified. Evidence: analysis/evidence/round-006/review.md and fresh-decode.json. Combined accepted shift byte order, borrow chain and inverse stores discriminate intact arithmetic operations from numeric data; damaged enclosing routine does not refute local role. Natural reachability unresolved; no new execution claim. BCC3E9D7+2→3E9F3 and danglingSTA3E9EF+1 remain unknown. Other prior candidate-specific residual review reused without new supported continuation.

map.validate passed; internal state saved, maps/current.json exported. Checkpoint round-006-classified byte-identical, mapSHA256 d85801c366f073fc9b23ff7c701138408e723560eb47795c6525eb68718b5312; analysis/round-006-classified-verification.json. ROM, prior regions and commentary source preserved (preservation.json). No toolkit edits/new issues; existing analysis/toolkit-issues.md remains applicable. Broad gameplay evidence/checklist unchanged; no changed RAM/terminal contract warranted replay. Natural final cockpit/credits unresolved.

Incomplete:105 Unknown bytes in20intervals. Exact physical residuals: 2E810+9, 34231+3, 34DFC+1, 359D3+1, 38AF8+1, 39319+1, 39EDE+1, 3B612+4, 3D62A+4, 3D65E+7, 3D67E+50, 3D7E5+3, 3DFD3+1, 3E010+2, 3E99A+11, 3E9B0+1, 3E9D7+2, 3E9EF+1, 3F448+1, 3F9E5+1. Counts include16classified header bytes. Full current-source commentary and redirect next; prior commentary unchanged and new code not yet editorially reviewed.

| Bank | Code | Data/other | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7596 | 531 | 8127 | 65 |
| 31 | 4703 | 3470 | 8173 | 19 |


## Round007 — independent fragment/template disassembly pass

ROM classification: 99.98% (262113 / 262160 bytes classified; 47 bytes unknown).

Accepted58 Static Code bytes across all four R006 leads. Independently decoded proposed intervals and accepted anchors, recomputed branches and reviewed primary source templates. Evidence: analysis/evidence/round-007/review.md, fresh-decode.json, anchors.json. Exact arithmetic/renderer targets are accepted JMP9103/INX, correcting prior overbroad operand rejections. Counter actual operand conflict/truncated load excluded. BC00 callee remains Data; acceptance establishes preserved wrapper/vector template only. Natural reachability unresolved, no new execution coverage. Prior regions, ROM and commentary source preserved (preservation.json).

map.validate passed; state saved and current map exported. Checkpoint round-007-classified byte-identical, mapSHA256 37154c863b7df03e73aa39ca73aad64dd18f36fe61f0fef6c30aa5cfb39f8bbc; analysis/round-007-classified-verification.json. Incomplete:47Unknown bytes/18intervals: 2E810+9, 34231+3, 34DFC+1, 359D3+1, 38AF8+1, 39319+1, 39EDE+1, 3B612+4, 3D62A+4, 3D67E+12, 3D6A5+2, 3D6AF+1, 3DFD3+1, 3E010+2, 3E9B0+1, 3E9EF+1, 3F448+1, 3F9E5+1. Counts include16classified header bytes. New code awaits full current-source commentary, prior commentary unchanged. Broad progression/artifical terminal evidence unchanged; no new gameplay RAM contract warrants replay; natural final cockpit/credits unproved. Existing toolkit report has three confirmed issues (two documentation bugs, one provenance bug), all worked around, no blockers/new findings; see analysis/toolkit-issues.md. No toolkit edits, publishing, archive, deletion or cleanup.

| Bank | Code | Data/other | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7641 | 531 | 8172 | 20 |
| 31 | 4716 | 3470 | 8186 | 6 |


## Round008 disassembly

ROM classification: 99.98% (262115 / 262160 bytes classified; 45 bytes unknown).

Accepted 2 Static Code bytes, physical $3D6A5+2, R007-COUNTER-CONDITIONAL-FRAGMENT. Independent exact decode, accepted field/flag boundaries, 256-value wrap arithmetic and signed target review support preserved conditional structure only. Actual taken physical $3D6B5 remains accepted JMP operand; no functioning helper/target code/natural caller asserted. Evidence: analysis/evidence/round-008/review.md, fresh-decode.json, anchors.json, finite-byte-check.json. Connected scope yields no further supported extension; prior residual-specific review retained. Broad artificial progression and terminal visits retained; natural cockpit defeat/credits unproved, no changed runtime contract or replay.

Internal-store unique-byte complement: 45 unknown bytes in 17 intervals, analysis/evidence/round-008/unknown-ranges.json. Counts including all 32 physical banks: analysis/evidence/round-008/counts.json. Header 16 classified bytes. Checkpoint round-008-classified validates and rebuilds byte-identically: analysis/round-008-classified-verification.json, map SHA256 e4bde7adb3fbc8ffc808ca84fa771aa2a1c073f2164d1e2ba68fca20268313ed. Input ROM and prior commentary source/symbol hashes preserved, preservation.json. No toolkit edits or new toolkit findings; existing issues remain in [toolkit-issues.md](toolkit-issues.md). Incomplete; next full current-source commentary then redirect.

| Physical 8 KiB bank | Code | Other classified | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7643 | 531 | 8174 | 18 |
| 31 | 4716 | 3470 | 8186 | 6 |


## Round009 disassembly

ROM classification: 99.99% (262127 / 262160 bytes classified; 33 bytes unknown).

Accepted R008-RELOCATED-SCROLL-DISPATCH:6 Static Code bytes physical $3D67E+6, CPU D66E fixed slots[unknown,unknown,30,31], and6 Static Data bytes $3D684+6. Independently verified all three inline word relocations and all four downstream call relocations ($61E), exact accepted template/AF contract and packed-field operation boundaries. Fresh evidence: analysis/evidence/round-009/review.md, fresh-decode.json, anchors.json, relocation-check.json. Actual E000 BRK0035 and overwritten third target remain broken functionality; no natural-use or repaired callee claim. Connected scope already classified; unsupported residuals remain localized. Broad artificial later-game and terminal visits preserved; natural cockpit victory/full credits unproved, no changed contract or replay.

Fresh internal-store unique-byte complement:33 bytes/16 intervals; all32 per-bank counts and exact residuals in analysis/evidence/round-009/counts.json and unknown-ranges.json. Header16 classified. Fresh map.validate passed, analysis/evidence/round-009/validation.json; checkpoint round-009-classified rebuilds byte-identically, analysis/round-009-classified-verification.json. Map SHA256 16e473e723b5f85e8a8243e6dfbf27befa49aca77819a1f3f6107c0df1c1a18d. ROM and prior commentary source/symbol hashes unchanged (preservation.json). No new toolkit findings; existing [toolkit-issues.md](toolkit-issues.md) preserved. Incomplete; next full current-source commentary then redirect. No toolkit/source edits, archive, cleanup/deletion, publish or subagents.

| Physical 8 KiB bank | Code | Other classified | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7649 | 537 | 8186 | 6 |
| 31 | 4716 | 3470 | 8186 | 6 |


## Round010 disassembly

ROM classification: 99.99% (262130 / 262160 bytes classified; 30 bytes unknown).

Accepted R009-RELOCATED-RENDERER-TILE-BASE:3 Static Code bytes physical $3D62B+3, CPU $D61B fixed slots[unknown,unknown,30,31]. Independently verified accepted original instruction alignment, exact88-byte copy, three table and four helper relocation anchors (+$61E), fresh decode and dataflow into accepted paired STA $034C,X tile producer. Orphan $3D62A remains Unknown; no missing B1 repair, natural caller or whole-function claim. Connected continuation already classified. Evidence: analysis/evidence/round-010/review.md, fresh-decode.json, anchors.json, independent-check.json. Broad gameplay/scenarios preserved; no changed progression contract supports replay.

Fresh internal-store complement:30 bytes/16 intervals; exact residuals and all32 per-bank counts in analysis/evidence/round-010/counts.json and unknown-ranges.json. Header16 classified. map.validate passed, validation.json; checkpoint round-010-classified byte-identical, analysis/round-010-classified-verification.json. Map SHA256 8587a03323dec7a53bd828f783f452d69e83d16170550b999fa55a88dffc3480. ROM and commentary source/symbols unchanged (preservation.json). No new toolkit issues; existing [toolkit-issues.md](toolkit-issues.md) preserved. Incomplete; next full current-source commentary then redirect. No toolkit/source edits, archive, cleanup/deletion, publication or subagents.

| Physical 8 KiB bank | Code | Other classified | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7652 | 537 | 8189 | 3 |
| 31 | 4716 | 3470 | 8186 | 6 |


## Round011 disassembly

ROM classification: 99.99% (262132 / 262160 bytes classified; 28 bytes unknown).

Accepted R010-BRK-RTI-SKIP: Static Code1 physical $3E010 (CPU $E000), Static Data1 BRK-discarded padding $3E011. Independently verified actual FFFE vector B2C0 -> accepted physical $3C0C2 RTI40 with fixed slots[unknown,unknown,30,31]. Hardware pushes E002/P and RTI restores PC/SP, architectural comparison flags and leaves A/X/Y unchanged; three residual stack writes and stacked B marker are explicit. Contract excludes NMI hijack/intervening interrupts, with I=1 sufficient for maskable IRQ exclusion; no natural caller, execution or timing-transparency claim. Decoder BRK length1 verified. Connected E002 filter/deactivation/scan and RTI already classified. Evidence: analysis/evidence/round-011/review.md, independent-check.json and fresh-decode.json.

Fresh internal-store complement:28 bytes/15 intervals; all32 per-bank counts and exact residuals in analysis/evidence/round-011/counts.json and unknown-ranges.json. Header16 classified. map.validate passed (validation.json); fresh checkpoint round-011-classified byte-identical (analysis/round-011-classified-verification.json). Map/state SHA256 094b219910335a9fe0bdcc2d53f8e4fc87edbf0ea7fd660c4c3529fba993b7d8. All existing regions, input ROM and prior commentary source/symbols preserved (preservation.json). Broad gameplay/scenarios unchanged: no changed RAM/entry/progression contract supports useful replay; prior artificial terminal signature is not natural cockpit victory/full timed credits. No new toolkit issues; existing [toolkit-issues.md](toolkit-issues.md) retained. Incomplete; next full current-source commentary then redirect. No toolkit/ROM/source edits, archive, deletion/cleanup, publication or subagents.

| Physical 8 KiB bank | Code | Other classified | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7652 | 537 | 8189 | 3 |
| 31 | 4717 | 3471 | 8188 | 4 |
