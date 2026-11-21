# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 disassembly — active

Verified baseline `analysis/baseline-verification.json` and `generated/baseline`;
input hash unchanged. NES mapper 1, physical size 163856 bytes (16 header,
131072 PRG, 32768 CHR). No annotations yet; raw baseline is not classification.
Validated vector entry instruction contracts: NMI $CE07 / offset 118295,
RESET $E882 / offset 125074, IRQ $CE61 / offset 118385. Startup effective
PRG banks [0,1,14,15]. Vector traversal saved in
`analysis/round1/initial-discovery.json` (53 static instruction candidates).
Live server is owned by `analysis/scratch/daemon.py`, tool session 96189;
RPC helper `analysis/scratch/rpc.py` uses bounded file requests. No socket
binding is allowed in this sandbox, so local file transport is used.
Runtime tracing enabled, detail capacity now zero; startup to frame 91 timed
out at 30 seconds but retained 860 physical instruction roots, no aggregate
drops. Initial frame `analysis/round1/title.ppm` shows colored bubbles.
A Start press plus three 60-frame requests is in progress (session 37517).
Next: identify game/title, productive gameplay and bank-aware observed/static
classification; save map/state and verified pass-001 before handoff.

### Corrected gameplay evidence

First file-RPC server ended with signal 143; durable exports retained. All new
batches now use one self-contained bounded server subprocess, with request and
response evidence in `analysis/round1/rpc-log.jsonl`. NES-001 records silent
single-button-schema mismatch. Correct input arrays confirmed menu selection.
Natural route: 300 untraced startup frames; Start 2/release;20 neutral;
Start2/release; requested100 (20s timeout at399 total); right+B35,
A+B35,left+B35 reached story at504. Snapshot mechanics.snapshot. Restore,
neutral request360 untraced (10M budget stopped at839); right+B30,A+B30,
left+B30 reached displayed Round01 by929. `round1-mechanics.ppm` confirms
stage, layout, player/enemy state. Accepted roots saved separately in contexts.json
and round1-contexts.json; snapshots are local same-build evidence only.

Later-state investigation: external RAM hypothesis $0401 current level confirmed
locally as 1 in Round01 and by static loads/stores. Restoring pre-level play.snapshot
and patching $0401 to30/80/112 was tested with300 untraced +60 traced frames;
all were still transitioning from story at759. RAM advanced selector by one.
Corrected continuation is now running: restore saved stage30/stage80 snapshot,
200 neutral untraced frames then representative right/B and A/B; final stage
uses111 before initialization because startup increments it. Do not count
written stage numbers as visits. Full classification/checkpoint pending these captures.


## Round 1 disassembly — complete phase, ROM incomplete

ROM classification: 81.25% (133132 / 163856 bytes classified; 30724 bytes unknown).

| PRG 8 KiB bank | Code | Data | Unknown |
| --- | ---: | ---: | ---: |
| 0 | 0 | 8192 | 0 |
| 1 | 0 | 8192 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 646 | 1805 | 5741 |
| 9 | 0 | 8192 | 0 |
| 10 | 832 | 822 | 6538 |
| 11 | 0 | 8192 | 0 |
| 12 | 1068 | 0 | 7124 |
| 13 | 152 | 6744 | 1296 |
| 14 | 4529 | 0 | 3663 |
| 15 | 1743 | 87 | 6362 |

Header:16 Data; CHR:32768 Image. Code/Data/Image totals: {'Code': 8970, 'Data': 91394, 'Image': 32768}.
Latest checkpoint `pass-003` validates and rebuilds byte-identically; see
[verification](pass-003-verification.json) and [handoff](round1-disassembly-handoff.md).
No live server. Final-boss artificial Continue route confirmed; ending unvisited.
One confirmed toolkit issue NES-001, no blockers; [report](toolkit-issues.md).

## Round 1 commentary — active

Prepared commentary/ using the configured toolkit helper; frozen current map and
phase1 state, before source and byte-identical initial verification retained.
Full source scope: 4549 emitted regions, including 4258 Code instruction regions;
133132 classified bytes and 30724 unknown unchanged. Mechanical review underway;
no readability completion claim yet. Working compact instruction inventory:
analysis/commentary-mechanics.txt. No server or toolkit edit. Disk ~3.5 GiB free;
scratch 756 KiB. Remaining: all source naming/explanations, review records, fresh
verification and refreshed reading copy, then compact commentary handoff.

## Round1 commentary — complete current-source readability

Reviewed the whole current source:4549 scaffold intervals,4258 code instructions,
24 data regions,1 CHR image and266 unknown gaps. All133132 classified bytes have
mechanical/format explanations;30724 unknown bytes have explicit uncertainty notes.
Added293 meaningful routine/fragment explanations,740 contextual labels and321
exact aliases. Full coverage and contexts:commentary/review.json. Local readable
source and refreshed reading copy:commentary/source/game.asm and
commentary/reading-after.asm. No remaining source-review ranges.

Fresh byte-identical verification:commentary/verification.json; source SHA256
 e0bbb595a126222574d2b35d1162db4edeba5f61190a0282744e107e7c067d6a.
Input unchanged; state/current map/frozen baseline identical SHA256
8c64f4b84194861555db57fa6dc98e0fc64b0349d3fe44e65cdfe8b7731be084.
Classification remains81.25%; final editorial/archive audit pending. See compact
handoff analysis/round1-commentary-handoff.md and commentary/uncertainties.md for
boss-defeat/ending, stage-record, sound-inline-word and actor-kind questions.
No terminal exhaustion claim or gameplay experiment in commentary.

Toolkit:existing confirmed NES-001, no new issues or blockers;
analysis/toolkit-issues.md. Disk checked3.5GiB free, scratch756KiB, commentary21MiB.
No toolkit edits, live server, publication, archive or file deletion.

### Round1 redirect active — attempt r1-20261121-a
Input pass-003; classification133132/163856=81.249389708036%,30724 unknown;
current-source commentary/verification byte identical. Research only, no map/source
changes. Durable active checkpoint: analysis/redirect/r1-20261121-a/checkpoint.md;
exact266-range unknown complement: unknown-ledger.json in same directory.
Finite RTS dispatch contracts recovered for36 sound commands and31 global effects.
Graph investigation combines these entry contracts, existing accepted instruction
successors, direct JSR entries and specifically audited inline-word return contracts.
Exploratory graph.json exposes6677 previously unknown code bytes and100 inline-word
sites (not yet independently reviewed/classified). All decoded official instructions,
no accepted Data conflict; call barriers remain explicit. Continuing normal-return
contract audit, handler bodies, data extents and boss-defeat/ending investigation.
Not a terminal handoff; preserve active frontier and continue same redirect.

Redirect active update: graph4.json / normal-return-contracts3.json now11035 unique
unknown code candidates,103 inline sites and199 conservative normal-return contracts.
Third finite palette dispatch recovered; sound generic sequence ranges being bounded.
Ending shortcut$6D=3 reaches defeat state4 after30frames; ending-b/run.py continues
from saved ending-step0.snapshot with bounded untraced/traced stepping (session4225).
Prior120 traced-frame timeout and capture-file-exists interruption preserved in
attempt scripts/RPC evidence. No classification changes or terminal assessment.

Redirect ending success: ending-b/frame2.png confirms artificial BAD END after
source-informed$6D=3 boss-defeat shortcut; ending-c variants confirm crystal flag
$049A=1 single-player friend-required BAD END and paired$42=3 rescue animation.
Both runtime handles completed. Paired final-text continuation prepared in ending-d;
representative Continue30/80 running in middle (session49721). Proposals.json bounded
batch currently17379 bytes (11192 Code/6187 Data), no map/source changes. Primary
edge, stack/inline/bank arguments and one excluded wrong-bank KIL target documented
in analysis/redirect/r1-20261121-a/inline-contracts.md. Still active research.

Redirect latest: HAPPY END! confirmed visually in ending-d/frame1.png, saved paired
endpoint step1.snapshot; no further ending run needed. Stage30 valid gameplay;
stage80 split part1 completed with Continue80 menu snapshot, part2 session26747
active. Long scenario processes exit143 were environmental interruptions with saved
endpoints, not ROM/toolkit failures. analysis/leads.json now contains five ready
structural proposal IDs with ranges/contexts/source/evidence/recipes/history.

### Round1 redirect complete — coherent ready batch for round2 disassembly
Input/output checkpoint pass-003 unchanged; current133132/163856=81.249389708036%
(81.25%),30724Unknown in266ranges. No new annotations. Proposed17625unique bytes:
11226Code+6399Data, filtered graph7/primary contracts, no overlaps/map conflicts;
101instruction/203byte actual artificial fetch subset recorded separately. Five ready
IDs: R1-residual-structure,R1-packed-records,R1-inline-contracts,
R1-finite-dispatch-data,R1-sound-noncode. Ending and middle Continue leads verified:
Round30/Round80 initialized gameplay, two BAD END requirements and HAPPY END saved.
All runtime handles closed; split stage80 route completed. Natural damage/pickup and
unresolved bank/stack propagation limits documented, no terminal exhaustion claimed.
Handoff: analysis/redirect/r1-20261121-a/handoff.md; exact Unknown ledger and proposal
manifest beside it; analysis/leads.json and analysis/scenarios.json updated with
preserved history. Commentary current-source readability/byte-identical verification
remain valid (source/map/ROM unchanged); final editorial/archive gates pending.
Toolkit: no new issues, existing NES-001 confirmed documentation/API validation bug,
explicit buttons-array workaround; no blockers (analysis/toolkit-issues.md). No
archive/publication/toolkit changes or cleanup/deletion performed. Coordinator may
independently review this batch and launch round2 disassembly worker.

### Round2 disassembly active
Input pass-003,133132 classified/30724 Unknown. Independently reviewing five ready
redirect leads; primary bank/stack/dispatch/data evidence recorded in
analysis/round2/review.md and reproducible review_apply.py. No gameplay replay or
commentary overwrite. Review then supported annotations with periodic state saves;
remaining frontier is redirect graph7 call/bank barriers and unknown-ledger.

### Round2 disassembly — verified pass-004, incomplete

ROM classification: 92.01% (150757 / 163856 bytes classified; 13099 bytes unknown).
Added17625bytes:11226Code+6399Data from all five ready R1 structural leads.
Totals:Code20196,Data97793,Image32768;155 remaining unknown intervals.
All9643 internal regions have structural evidence. Code provenance:7676Observed,
12520Static bytes; observed does not mean natural (includes prior artificial routes).
New observed subset101instructions/203bytes; two fixed-bank actual context additions
preserved. No new gameplay runs. Reused verified Round01/30/80/boss/twoBAD END/HAPPY END.

| PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
|0|0|8192|8192|0|
|1|0|8192|8192|0|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|0|8192|8192|0|
|6|0|8192|8192|0|
|7|0|8192|8192|0|
|8|934|7252|8186|6|
|9|0|8192|8192|0|
|10|1536|1176|2712|5480|
|11|0|8192|8192|0|
|12|5496|370|5866|2326|
|13|1096|6744|7840|352|
|14|6457|215|6672|1520|
|15|4677|100|4777|3415|

Header16Data and CHR32768Image fully classified, outside PRG bank counts.
Independent review: analysis/round2/review.md, review-decodes.json, data-boundaries.json;
full supported batch accepted after bank/stack/inline/finite-table consumer inspection.
Explicit wrong-bank98662 KIL operand excluded; boss-conditioned bank10 veneer accepted.
Local resume correction preserved additional actual fixed-bank contexts at118096/118099;
no toolkit defect. State saved, maps/current.json exported and map.validate passed.
Checkpoint pass-004 validated/generated/verified byte_identical=true,ok=true;
analysis/pass-004-verification.json, maps/pass-004.json, generated/pass-004/game.asm.
Map SHA256563b559e3c5d2a1a0d676f2f7734997365a4ddd6bd236f11454e305d2a5f2f41.
Input ROM unchanged; SHA256fc521e891d074369d67579fd56617460b9c9949dec67cc4a37d46b4e4bb09d29.

Consumed verified:R1-residual-structure,R1-packed-records,R1-inline-contracts,
R1-finite-dispatch-data,R1-sound-noncode. Produced research:R2-bank-shadow-contract,
R2-task-stack-bank-contract,R2-residual-consumers. Exact residual analysis/round2/unknown.json;
call/bank frontier analysis/round2/frontier.json. FA81 bank shadow, CEE3 nonlocal stack,
unresolved pointer domains and heterogeneous bank10 gaps need further redirect research;
no unsupported data blanket and no terminal exhaustion claim. Connected supported batch
is closed; remaining barriers require these missing invariants, not replayed gameplay.

Existing commentary/source preserved unchanged, still verified for input pass-003 only;
full current pass-004 source commentary is pending. No commentary/redirect phase executed,
no publication/archive/toolkit changes or cleanup/deletion. Toolkit:no new issues found;
existing NES-001 confirmed documentation/API validation bug, explicit buttons-array
workaround, zero blockers. See analysis/toolkit-issues.md.

## Round2 redirect started — r2-20261121-a
Current-source commentary is complete: [handoff](round2-commentary-handoff.md), source530c7d0404b310ffba3ec0271547890ed590187da93e12e1a6604679632c8047; byte-identical verification passed, final editorial audit pending. Map pass-004 unchanged:150757/163856classified;13099Unknown/155ranges. Research frontier/checkpoint:redirect/r2-20261121-a/checkpoint.md. FA81 shadow restoration, CEE3 stack/bank contract and residual consumers being investigated; no new observations/classification yet.

### Redirect r2-20261121-a active update
Graph10 conditional structural batch5099Codebytes; independent data/copy proofs2528Databytes; provisional proposals7627bytes, no overlaps or current-map conflicts. Exact evidence/contracts and active graph12 frontier:redirect/r2-20261121-a/checkpoint.md,contracts.md. All unacceptedStatic research; pass004 map/source/verification unchanged. No emulator or replay.

## Round 2 redirect complete — ready combined batch

Attempt: redirect/r2-20261121-a. Input pass-004 remains unchanged at 150,757 / 163,856 classified bytes, with 13,099 Unknown bytes across 155 ranges. Current-source readability and byte-identical verification remain as documented in round2-commentary-handoff.md; final editorial audit is pending.

The combined ready proposal is 12,957 Static bytes: 8,371 Code and 4,586 Data. No annotations were applied. Independent acceptance in full would leave exactly 142 Unknown bytes in 15 subintervals. Supported conditional bank/stack contracts, finite dispatch/task entry domains, connected ending/credits paths, bounded pointer/copy consumers and practical generic-data inspection are documented in redirect/r2-20261121-a/handoff.md. Exact recipes and three ready R2 lead outcomes are appended in leads.json; original r1 evidence is preserved.

handoff-checks.json confirms unchanged ROM/map/state/source hashes, exact Unknown complement, no proposal overlaps or map conflicts, and correct physical resolution for all Code views. No new toolkit issues found; existing NES-001 remains documented with a workaround and no blocker. No gameplay replay, emulator mutation, toolkit edit, cleanup, publication, archive or goal completion. This is a ready disassembly handoff, not a terminal exhaustion assessment.

## Round3 disassembly active
Independent primary contract/graph/data review accepts proposed12957Staticbytes (8371Code4586Data). Fresh configured-binary decoder checked4199Codeviews with zero discrepancies; physical mapping, overlap and exact residual checks pass. See round3/review.md and round3/review-checks.json. Supported annotation application in progress; final saved counts/checkpoint pending. Pass004 commentary preserved.

## Round3 disassembly — pass-005

ROM classification: 99.91% (163714 / 163856 bytes classified; 142 bytes unknown).

Accepted12957newStaticbytes:8371Code4586Data,4158regions. Fresh configured-binary decode4199views; zero decoder/physical/overlap discrepancies. Primary structural review: round3/review.md and review-decodes.json; exact counts round3/summary.json. Conditional FA81 mode3/shadow, eventual CEE3 task resumption, finite task0 initialization, bounded pointer/normal-branch contracts, independent bank10 collision routine and finite PPU copies reviewed. C5FD continuation includes all eight selected lower pairs and restored-pair C611. Reachability remains separate; no new observed execution.

Per physical8KiB PRG bank (classified/unknown): 0:8192/0, 1:8192/0, 2:8192/0, 3:8192/0, 4:8192/0, 5:8192/0, 6:8192/0, 7:8192/0, 8:8186/6, 9:8192/0, 10:8184/8, 11:8192/0, 12:8183/9, 13:8192/0, 14:8163/29, 15:8102/90. Header16/0; CHR32768/0. Provenance bytes: {'Observed': 7676, 'Static': 156038}. Structural kinds: {'Code': 28567, 'Data': 102379, 'Image': 32768}.

All three ready R2 leads verified and consumed; history appended in leads.json. Existing Round01/30/80/boss/BAD/HAPPY progression evidence reused; no unchanged replay. Remaining142Unknown in15intervals: round3/unknown.json and redirect/r2-20261121-a/residual.json. E902 has destructive reset-stack return barrier; other short instruction-like islands lack discriminating entry/boundary/control contracts. No blanket Data fallback. Connected queue exhausted; incomplete, no terminal rejection/completion claim.

map.validate passed; state saved and portable map exported. Fresh maps/pass-005.json and generated/pass-005 verified byte-identical by analysis/pass-005-verification.json. ROM unchanged. Existing pass004 commentary preserved; current pass005 commentary/readability and final editorial audit pending. No other phase/toolkit changes/publication/archive/cleanup. No new toolkit issues found; existing NES-001 and workaround retained in analysis/toolkit-issues.md.

## Round3 commentary complete; redirect active — r3-20261121-a
Current-source readability complete per round3-commentary-handoff.md: 775 Code and332 Data units; source ae4307f959517f34cc1a03b7ea1a3f251b0b90290ba155d31d28b46ef3ab1997, fresh byte-identical verification. Pass005 map/state unchanged at163714 classified/142Unknown in15ranges. Final editorial audit pending. Redirect starts from exact round3/unknown.json and prior immutable r2 evidence. Active frontier: reassess E902 safe reset stack against preceding E8DC TXSFF and bounded initialization calls; compare controller/display variants against accepted sibling semantics, rather than require actual callers universally. Research only.

## Round3 redirect complete — ready combined batch

Attempt redirect/r3-20261121-a retains pass005 map/state/source and142Unknown/15ranges. Proposed131Staticbytes (127Code+4Data), no accepted bytes: concrete resetSPFF producer and disjoint destructive-clear/task-seed frames, sibling controller/mask variants, duplicated36-byte sound-reset/disable neighborhood, bounded inline queue ABIs, OAM constant fill, result-mode gate and coordinate clamp. Detailed primary contract evidence and localized competing interpretations:redirect/r3-20261121-a/contracts.md; exact proposals/checks/ledger/handoff there. R3-reset-return andR3-static-variants ready in leads.json. If fully accepted11Unknown/5ranges; actual classification unchanged until independent disassembly.

Coherent connected and independent actionable avenues assessed. Remaining tinyphase-reset pair, isolatedRTS/JMP andFACE-like word stay ambiguous from insufficient independent structural discrimination; no universal actual-caller prerequisite and no forcedData. No terminal exhaustion assessment. Current-source readability remains complete at input pass005, fresh prior byte-identical verify; final editorial audit pending. No new toolkit issues found; existingNES-001 workaround/no blocker retained. No live server/replay/map/source/state change/toolkit/archive/publication/cleanup/deletion/goal completion.

## Round4 disassembly — pass-006

ROM classification: 99.99% (163845 / 163856 bytes classified; 11 bytes unknown).

Accepted131newStaticbytes (127Code+4caller-inlineData),65regions. Independent primary review in round4/review.md; fresh63proposal views plus743accepted-support decodes in round4/review-decodes.json/primary-decodes.json. Zero decoder/physical/overlap discrepancies. Concrete TXSFF reset return frame survives0110..01CF clear and012E/F task seeds; stable controller and mask siblings, duplicated36byte sound neighborhood, known FA04/F922 inline ABI, actual absolute-LDA F4 OAM constant fill, mode-consistent panel selector and paired coordinate clamp supported. Structural classification only, natural entry unresolved; no new observed execution. Prior destructive arbitrary-SP objection localized/corrected, history preserved.

Per physical8KiB PRG bank(classified/unknown): 0:8192/0, 1:8192/0, 2:8192/0, 3:8192/0, 4:8192/0, 5:8192/0, 6:8192/0, 7:8192/0, 8:8192/0, 9:8192/0, 10:8192/0, 11:8192/0, 12:8183/9, 13:8192/0, 14:8190/2, 15:8192/0. Header16/0; CHR32768/0. Provenance bytes: {'Observed': 7676, 'Static': 156169}. Structural kinds: {'Code': 28694, 'Data': 102383, 'Image': 32768}. Exact counts:round4/summary.json.

Both R3 ready leads verified/consumed with appended history. Remaining11Unknown in5ranges:round4/unknown.json. Bank12 8AE1+3/8B0E+2 candidate linked phase-reset,8E24+1 isolatedRTS,9525+3 isolatedJMP; fixedC1E3+2 FACE-like word. Independent entry/extent or code-versus-data discrimination remains absent; no speculative genericData. Connected supported structures already classified, no further easy gain identified. Reuse prior later-level/boss/BAD/HAPPY progression evidence; no unchanged replay. Incomplete ordinary handoff, no terminal-exhaustion or full-readability claim.

map.validate passed; saved analysis/state.json/exported maps/current.json. Fresh pass006 map/generated source verified ok=true,byte_identical=true in analysis/pass-006-verification.json. ROM unchanged. Entire pass005 commentary/source preserved at recordedSHA256; pass006 commentary expansion/final editorial audit pending. No other phase executed, toolkit change/publication/archive/cleanup/deletion/goal completion. No new toolkit issues found; existing NES-001 workaround retained in analysis/toolkit-issues.md.

## Round4 redirect complete — ready generic non-code boundary batch

Attempt analysis/redirect/r4-20261121-a starts from pass006/fullcurrent commentary source bf24d353c74443d753aec2b792ebe8c599ad902d1b62cfa815680d7dc90cebe7. Actual163845classified/11Unknown5ranges unchanged. Proposed2StaticData bytes physical115187..115189/fixedC1E3..C1E5, accepted0: priorJSR/RTS boundaries exclude operand continuation; CE length3 conflicts with independently consumerbounded paletteC1E5; FA1byteNOP mandatory successor also conflicts with sameData boundary. Genericnoncode only; specificFACEpointer role unresolved. R4-c1e3-boundary-data ready in leads.json; independent disassembly required. If accepted,9Unknown4bank12ranges.

All5actual residuals reviewed against primary source/map and fresh configuredrelease decodes; exact state complement ledger/checks and compact reasoned rejection review in attempt. Phasepair bounds now explicitly anchored; insufficiency localized to lack of independent resetrole discrimination: type7-gated40/80setters preserve lowbits, proposed0clear resets wholefield unconditionally. IsolatedRTS andJMP have coherent terminal contracts but no further discriminating operation; no actualcaller universal requirement. Known directcontrol references/rawhits checked contextually. No other identified promising actionable avenue remains in this redirect; no artificialexecution test discriminates intended role, no unchanged gameplay replay. This is readybatch handoff, not terminal/editorial acceptance.

ROM/map/state/current source unchanged per preservation/checks. No new toolkitissues; existingNES-001workaround retained. No liveprocess/toolkitedit/publication/archive/cleanup/deletion/goalcompletion.

## Round5 disassembly — pass-007

ROM classification: 99.99% (163847 / 163856 bytes classified; 9 bytes unknown).

Accepted2Static genericData bytes physical115187+2 /fixedC1E3. Independent review of actual precedingC1DFJSR/C1E2RTS boundaries, inlineE5C1 and descriptor00/count02 F992 consumer proves32byte paletteC1E5..C205. CE3byteDEC overlaps it; FA1byte nonterminalNOP requires successor in it. Strongest overlapping-code/detachedNOP alternatives assessed explicitly under accepted nonoverlap model; no hardware impossibility or FACE-pointer meaning asserted. Fresh91configured-release contextual decodes; no new execution/natural reachability. Evidence:analysis/round5/review.md, fresh-decodes.json, accepted-neighbors.json and annotation.json.

Per physical8KiB PRG bank(classified/unknown): 0:8192/0, 1:8192/0, 2:8192/0, 3:8192/0, 4:8192/0, 5:8192/0, 6:8192/0, 7:8192/0, 8:8192/0, 9:8192/0, 10:8192/0, 11:8192/0, 12:8183/9, 13:8192/0, 14:8192/0, 15:8192/0. Header16/0; CHR32768/0. Provenance:{'Observed': 7676, 'Static': 156171}. Structural kinds:{'Code': 28694, 'Data': 102385, 'Image': 32768}. Exact counts:analysis/round5/summary.json.

R4-c1e3-boundary-data verified with appended history. Remaining9Unknown4bank12ranges:101105+3,101150+2 linked potential wholefieldclear,101940+1 isolatedRTS,103733+3 JMP97C9. Accepted family/store/target bounds are acknowledged, but do not independently assign the candidate operations. Reused compact primary rejection review with scoped competitors; no caller mandate, unchanged gameplay replay or unsupported genericData. Connected supported structures already classified. Incomplete ordinary handoff; no terminal-exhaustion or editorial acceptance claim.

map.validate passed, state saved/map exported. Fresh pass007 verified ok=true,byte_identical=true:analysis/pass-007-verification.json. Entire pass006 commentary/source preserved SHA256bf24d353c74443d753aec2b792ebe8c599ad902d1b62cfa815680d7dc90cebe7; pass007 commentary update remains separate. ROM unchanged. No other phase, toolkit modification, live emulator/server, archive, publication, cleanup/deletion or goal completion. No new toolkit issues found; existing NES-001 workaround retained in analysis/toolkit-issues.md.

## Coordinator terminal handoff — justified incomplete, orchestration finished

ROM classification: 99.99% (163847 / 163856 bytes classified; 9 bytes unknown).

Final checkpoint pass-007 remains valid and byte-identical. Current source completed final editorial acceptance after targeted stale-boundary prose repairs; commentary/audit.json verifies13871 inventory entries and fresh byte-identical rebuild, and reading-after.asm exactly expands current source. Final source SHA256d0124e82f50ca0982fe19bedbfa680727a7814263550a2aad49f80a195ff9153. Configured release map/store validations passed. All1122 meaningful units are reviewed. The immutable source-read cache changed only repeated I/O cost of the original audit, with exact before/after SHA proof; failed/unobservable attempts receive no credit.

Coordinator inspected decisive primary bytes, decoded views, accepted field/setter semantics and neighboring control flow before accepting redirect/r5-20261121-a/review.md. Linked5bytes, isolatedRTS1byte and isolatedJMP3bytes remain structurally ambiguous. Bounds are acknowledged; no universal caller prerequisite or forced genericData is used. No useful corrected inference or discriminating test remains; no productive interrupted frontier or ready lead remains. This is documented terminal incomplete, not blocked work or whole-ROM completion.

Final conclusions:final-handoff.md. Coordinator primary-evidence critique:coordinator-rejection-review.md. Final editorial handoff:final-editorial-handoff.md. Requirement audit:coordinator-completion-audit.md and completion-audit.json (ok=true). Archive document-selection contract inspected without invoking archive; ../archive-options.json selects56 additional final/historical Markdown documents and omits15 duplicate instruction copies, with zero unrecognized Markdown paths. No final conclusion/handoff is omitted. Toolkit issueNES-001 and grounded audit-performance missing featureAUDIT-002 retained locally with workarounds; no blocker or toolkit edit. Input ROM unchanged, all derived artifacts local. No archive execution, publication, cleanup or workspace deletion occurred. All work/evidence remains resumable if new evidence or a justified corrected method arises. The orchestration goal is achieved; ROM classification remains incomplete.

## Manual fallback disassembly — pass-008

ROM classification: 100.00% (163856 / 163856 bytes classified; 0 bytes unknown), including 9 user-authorized Manual fallback bytes.
Evidence-backed classification remains 99.99% (163847 / 163856); the 9 Manual bytes are not structural proof. This phase does not claim whole-ROM evidence-backed completion or final editorial acceptance.

Input checkpoint: pass-007. Before any annotations, exact internal-map complement was verified as 9 bytes, within the user's <=10-byte exception: physical offsets 101105+3 ($8AE1), 101150+2 ($8B0E), 101940+1 ($8E24), 103733+3 ($9525), all physical PRG bank12 with CPU alias shown for lower mapped slot. These exact ranges alone were annotated generic Data through supported map.annotate with Manual provenance. The explicit user instruction authorized fallback despite unresolved code-versus-data ambiguity, required unknown meaning and preserved history, and prohibited redirect research. Manual confidence1.0 records certain authorization, not inferred role certainty.

Prior ambiguity remains unresolved: linked candidate whole-field phase clear (5 bytes), isolated RTS (1), isolated JMP (3). No semantic label, execution, natural reachability, unused/dead status or independent structural evidence was invented. See ../round5-disassembly-handoff.md and ../redirect/r4-20261121-a for primary historical reasoning; all records preserved. No leads consumed/produced and no research rerun.

Before/after checks prove all existing internal regions identical, exactly four new Manual Data regions totaling9 bytes, and unchanged ROM/commentary source. Supporting artifacts: complement-before.json, input-identities.json, annotations.json, map-validation.json, preservation-check.json and summary.json here. Totals: Code28694, Data102394, Image32768; provenance Observed7676, Static156171, Manual9. All16 PRG banks8192 classified/0 unknown; bank12 consists8183 evidence-backed +9 fallback bytes. Header16 and CHR32768 fully classified.

map.validate passed; analysis/state.json saved and maps/current.json exported. Fresh maps/pass-008.json and generated/pass-008/game.asm rebuild byte-identically: ../pass-008-verification.json (ok=true, byte_identical=true). ROM SHA256 fc521e891d074369d67579fd56617460b9c9949dec67cc4a37d46b4e4bb09d29. Map SHA256 9bf5919f780a425ad55d877612ea96407d65c4ddb1ce910b0d1f7141eab8a89e. All exact output hashes in summary.json.

Next: separate commentary phase must preserve and reconcile existing source with four fallback ranges, explain unknown meaning/manual policy explicitly, refresh reading copy and run fresh verification; then final editorial acceptance. Current source unchanged by this phase; prior commentary credit remains historical. No other phase, toolkit change, archive, publication, cleanup/deletion or goal completion. No new toolkit issues found; existing ../toolkit-issues.md retained (NES-001 and AUDIT-002 workarounds). All internal ROM, state, traces, assets and source stay local; supported ROM-free portable mapping is the only sharing deliverable.

## Pass008 final editorial acceptance

Final editorial acceptance passed. All13871 scaffold entries and1122 meaningful units are reviewed; full source inventory/map/store reconciliation found no pending naming/comment defect. Original strict commentary audit additionally validates13507 embedded code-unit records. Read-only original archive.editorial_status reports complete=true,13871/13871 regions,13507/13507 units,reasons=[].

Reused unchanged prior full review and483-alias inspection. Substantively checked all four changed Manual notes and adjacent phase-setter/shared-store, tracking-flag, award-arc and special-state dependencies. Only source comments changed since prior acceptance; executable body and line positions remain identical. Before-review records/source preserved in final-editorial/before-review; inspection.json records full inventory checks. Current editorial-audit.md and uncertainties reflect pass008 acceptance.

Fresh original strict audit passed ok=true,byte_identical=true,editorial_audit=true and refreshed reading-after.asm, which exactly equals current expansion. Immutable source-read caches affect I/O only; original checks unchanged; separate cache proofs retain equal before/after source hashes. Source SHA256 fb897e395e33489cc8df4e61cce915519ce17af9cdd208bcff14ce26facd3922. Configured release map validation and loaded-state map.validate both passed; current map/store exactly match frozen baseline and manifest. Audit result, editorial-status.json, reading-check.json and hashes.json are in final-editorial/.

Classification remains163856/163856 including9 user-authorized Manual fallback bytes;163847 evidence-backed,0 unclassified. Four fallback structural meanings remain unresolved; no semantics, execution or structural proof invented. No redirect/research, toolkit change, archive execution, deletion or publication. Existing NES-001/AUDIT-002 workarounds retained; no new toolkit issue or blocker. Coordinator final conclusions/archive-options remain separate.

## Coordinator final acceptance — explicit Manual fallback exception complete

ROM classification: 100.00% (163856 / 163856 bytes classified; 0 bytes unknown), including9 user-authorized Manual fallback bytes. Evidence-backed classification remains163847bytes; whole-ROM structural proof is not claimed. Exact fallback ranges101105+3,101150+2,101940+1,103733+3 were independently checked against the before-complement and unchanged pass007 regions. Prior ambiguity and explicit authorization remain in manual-fallback/handoff.md, annotations.json, current source and final-handoff.md. No unrelated exception or research restart was used.

Separate sequential workers completed pass008, full current-source commentary and final review/strict audit. Coordinator inspected original audit/status adapters and results, four source notes and final dependency/inventory assessment. Current source13871scaffold entries/1122meaningful units has no pending ranges;13507embedded Code-unit records passed the original unchanged audit. Fresh byte-identical checkpoint and final source rebuild, map/store validation, exact reading expansion and current map/baseline identity all passed. Source hashfb897e395e33489cc8df4e61cce915519ce17af9cdd208bcff14ce26facd3922; map/state9bf5919f780a425ad55d877612ea96407d65c4ddb1ce910b0d1f7141eab8a89e; original ROM hash unchanged.

Final conclusions and requirement assessment updated in final-handoff.md, final-editorial-handoff.md and coordinator-completion-audit.md, preserving original terminal docs in manual-fallback/prior-terminal and full prior commentary in round-6/commentary. Archive document-selection contract inspected without invocation; archive-options.json selects67additional final/historical Markdown documents including fallback record and final handoffs, omits18duplicate instruction copies, and has zero unrecognized Markdown. Final machine checks: completion-audit.json and manual-fallback/completion-audit.json, generated by manual-fallback/coordinator-audit.py.

No required blocker remains. Existing nonblocking NES-001/AUDIT-002 report/workarounds retained; no toolkit edit, archive, cleanup/deletion or publication. Final free disk2.4GiB; scratch756KiB. All ROM/source/assets/state/traces remain local; only supported ROM-free portable mapping is a sharing deliverable. The complete authorized batch workflow is achieved under the explicit Manual exception, with evidence-backed and fallback totals reported separately.
