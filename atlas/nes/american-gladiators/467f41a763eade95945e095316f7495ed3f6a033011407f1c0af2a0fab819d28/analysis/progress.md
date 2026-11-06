# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 01 disassembly — active checkpoint
Baseline established and byte-identical: [verification](baseline-verification.json).
ROM unchanged: American Gladiators, GameTek 1991, mapper 1, 128 KiB PRG / 128 KiB CHR.
Startup trace captured 2,143 contextual roots (2,113 physical instruction starts);
observed decodes and conservative connected static discovery (2,360 unique candidate starts)
are durable under round-01/. No classifications have yet been committed to the internal map;
baseline map remains only a fidelity baseline. Current work is progressing menus into events.
A late Start hold advanced title to player selection; early Start attempts were ignored.
Durable startup/event/menu snapshots are local under round-01/; scripts and complete JSON-RPC
request/reply log retain routes, exact budgets, timeouts, and effective bank contexts.
Menu navigation process is running via analysis/round-01/menus.py (PTY session 65783).
Next: inspect captured menu frames, visit representative selectable events, annotate supported
code and CHR/header structures, save internal state/export and verify round-01 checkpoint.
Not a terminal handoff; gameplay progression and classification remain active.

Update: menu traversal and first platform event succeeded; see scenarios.json,
traces/round-01/action.ppm and round-01/events-contexts.json. Current durable snapshot
round-01/events.snapshot.json is at frame 1114, effective PRG banks [2,3,14,15].
There are 3,066 observed physical starts before adding latest event evidence. A first
annotation request was rejected atomically for a missing mandatory id field; corrected
to id=0. No toolkit bug claimed (documented explicit region schema). Preparing final
bounded shortcut check and committing supported classifications/checkpoint next.

Sourced password shortcut now confirmed: round-01/shortcut.py entered ABBAB after
highlighting PASSWORD; traces/round-01/later-2.ppm displays LEVEL03 and LIVES10 and
five-event selection. Durable replay start: round-01/later-menu.snapshot.json.
Current classified internal batch: 141,048 / 262,160 = 53.80%; 7,656 code bytes,
131,072 CHR image bytes, 2,320 generic/header/vector data bytes. Additional shortcut
and representative-event evidence will be merged before the final checkpoint.
Final broad event experiment is running via representative.py/session26865; no RAM
edits or speculative register calls used. Ending requires event completion flags
whose contract is not yet known; follow up through menu/transition dispatch rather
than arbitrary RAM brute force.

## Round 01 disassembly — completed pass, ROM incomplete

ROM classification: 54.94% (144024 / 262160 bytes classified; 118136 bytes unknown).

One disassembly pass completed. Baseline and latest checkpoint both validate and rebuild
byte-identically with configured release binary; ROM SHA-256 unchanged. Latest verification:
[round-01-disassembly-verification.json](round-01-disassembly-verification.json);
[map](../maps/round-01-disassembly.json), [current source](../generated/round-01-disassembly/game.asm),
[internal state](state.json). Earlier batch checkpoint preserved.

Classification: 10,632 Code; 2,320 Data (16-byte iNES header, 6-byte vectors and 2,298
uniform non-code bytes); 131,072 Image (complete iNES CHR tile allocation). Exact unique
interval union counted from internal map; raw export fallback is excluded.
Provenance bytes: {'Observed': 9995, 'Static': 134029}. Header/CHR allocation, decoded instructions, contextual
fetches, direct control-flow and uniform boundaries supply separate structural evidence.
Uniform role remains generic data: specific fill purpose/consumer not asserted.

Execution coverage is separate: 4739 unique accepted physical opcode starts,
6148 effective contextual roots across retained natural/password routes.
Trace summaries report aggregate_dropped=0; capacity-zero captures intentionally dropped
detailed events. These are coverage aggregates with replay/snapshot evidence, not exhaustive
instruction traces or all-ROM execution coverage. Artificial terminal roots excluded from
classification and these execution counts.

| Physical bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 16 | 0 |
| PRG00 | 1753 | 1328 | 3081 | 5111 |
| PRG01 | 568 | 0 | 568 | 7624 |
| PRG02 | 1092 | 0 | 1092 | 7100 |
| PRG03 | 216 | 0 | 216 | 7976 |
| PRG04 | 1491 | 0 | 1491 | 6701 |
| PRG05 | 0 | 0 | 0 | 8192 |
| PRG06 | 2845 | 0 | 2845 | 5347 |
| PRG07 | 0 | 0 | 0 | 8192 |
| PRG08 | 0 | 0 | 0 | 8192 |
| PRG09 | 0 | 48 | 48 | 8144 |
| PRG10 | 0 | 0 | 0 | 8192 |
| PRG11 | 0 | 0 | 0 | 8192 |
| PRG12 | 0 | 0 | 0 | 8192 |
| PRG13 | 0 | 0 | 0 | 8192 |
| PRG14 | 0 | 87 | 87 | 8105 |
| PRG15 | 2667 | 841 | 3508 | 4684 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

Gameplay: title/player/start-password menus, default first event, sourced and confirmed
Level03/Lives10 password, and all five selectable events with representative input.
See [scenarios.json](scenarios.json), [gameplay.md](gameplay.md), captures follow-0..4.ppm
and matching durable snapshots under round-01/. Early and short-transition failures retained.
The first final/ending artificial test patched $0591/$0592, $0590, $0595 and entered
$89F2/$8B89 with stack $FF. Both froze; final screen remained menu, ending PC reached $0002.
The partial ending run ended before final snapshot; request/reply evidence is preserved.
Missing dependent entry/dispatch state prevents claiming successful final/ending visits.
No terminal artificial fetches justify intended-code annotations. Do not repeat unchanged.

Ready handoff: indirect bank-aware sound/engine dispatch, inline-parameter call formats,
completion/Eliminator initialization and residual PRG data/code boundaries. Details in
[leads.json](leads.json), [investigation.md](investigation.md), and
[unknown complement](round-01/unknown-complement.json). Other level permutations and full
ending are deferred to concrete state-contract research; this is an easy-gain handoff,
not a terminal exhaustion assertion. Full source commentary/editorial audit remains.

Toolkit: **no toolkit issues found**. [Persistent report](toolkit-issues.md).

## Round 01 commentary — active
Prepared commentary/ with immutable before/ and frozen classified map/store hashes;
initial preparation rebuild is byte-identical. Read all 278 contiguous classified code
fragments (10,632 bytes) and authored their mechanical summaries and descriptive entry
names in round-01-commentary/units.txt. Consolidated instruction wrappers in the editable
source copy. Now applying RAM/constants and internal routine/branch explanations across
the full scope, then refreshing reading copy and verifying. This is active commentary,
not a completed readability handoff. Classification remains 54.94%; original map/store
unchanged. No toolkit issues found.

## Round 01 commentary — completed current-source readability
All currently classified source is readable: 278 contiguous code fragments / 10,632
code bytes, 2,320 data bytes and 131,072 CHR image bytes. Added 689 functional routine/
branch names with explanations and 268 shared RAM aliases; named hardware/button/flag
constants are used in operands. Unknown fallback ranges retain explicit physical
boundary/uncertainty notes. No classified readability backlog remains. Full review
scaffold preserved; final editorial acceptance audit remains pending.

Fresh rebuild [verification](round-01-commentary-verification.json) is byte-identical;
[reading copy](../commentary/reading-after.asm) refreshed. Original map/store and frozen
copies retain hash 4c6a44876fb0b171e856cb54a96e453676de8b6ec8feb5da8b25957ddbc42cdf;
ROM identity unchanged. [Commentary checkpoint](round-01-commentary/checkpoint.json)
records source/reading hashes and scope assertions. Classification remains 54.94%
(144,024/262,160; 118,136 unknown). Execution coverage unchanged; no new runtime runs.

[Handoff](handoffs/round-01-commentary.md) and
[consumer contracts](round-01-commentary/redirect-contracts.md) provide exact inline
argument/animation/sprite/text/palette contracts, actor/audio dispatch prerequisites,
and supported completion-path questions for redirect. Unknown game meanings were
localized in source and commentary/uncertainties.md; none justify omitted explanations.
No toolkit issues found; [report](toolkit-issues.md). Disk has 8.8 GiB free; commentary
research records are under 1 MiB, with source/reading copies retained separately.

## Round 01 redirect — active consumer investigation (2026-11-05)

Input checkpoint round-01-disassembly; immutable map/store SHA256
4c6a44876fb0b171e856cb54a96e453676de8b6ec8feb5da8b25957ddbc42cdf.
Classification unchanged: 144024 classified / 262160; 118136 unknown.
Current-source readability gate complete in round-01-commentary.
Attempt: analysis/redirect/r01-consumers-20261105/ (active, not phase handoff).
Read evidence-policy and prior terminal.py/rpc failure; old direct-PC/stack recipe
is rejected only as an initialization recipe. New supported method preserves actor
scheduler and stack: follow-0 snapshot has parent slot11 continuation $8B17, savedY10,
bank0 and event actor10 in bank1. Clear that specific child active flag after setting
completion mask/level, allowing the naturally waiting parent to perform fade/transition.
Static consumer graph supplies 317 candidate records and connected callbacks, including
final event selector4 => physical banks8/9. Mapper-changing high sound wrappers need
explicit bank6/7 contracts; corrected graph now uses those instead of inherited caller
bank. Raw graph output remains candidate evidence pending conflicts/bounds review.
Sound directory pattern occupies $9925..$99E5 before an independently consumed envelope
directory; one cell points to RAM $0088. Physical pattern bounds do not prove natural
selector domain. Specific handler contracts and generic data claims will be separated.
Disk available 8.8GiB; no cleanup performed. No toolkit issues found so far.

Active redirect update: strict mapper barrier review excludes inherited lower-bank
continuations after unresolved WriteMMC1PRG calls. Final actor explicitly stores
selector5 in actorPRGSelector and enters $F595, which stores mainPRGSelector5,
sets PRG then JMP $8000 => physical banks10/11, exposing ending code rather than
re-decoding bank8. Current bounded graph: 17669 unique proposed code bytes;
360 inline/directory records (1394 gross bytes); finite animation/metasprite/
rectangle consumer graphs 5774 candidate data bytes. No new graph instruction is
BRK or an unofficial opcode; no proposed-record/code overlap seen in direct graph.
Runtime supported child termination preserved parent $8B17 but did not advance;
primary runtime RAM proves pauseMode $0576=$FE, so scheduler never ran. An initial
unpause retry mistakenly wrote adjacent $0577 (already1), producing unchanged
outcome; retain completion-unpaused.py/results and correct only exact $0576 to1.
New discriminant: parent callback must move beyond $8B17 and select final/ending
state while keeping natural stack and scheduler. Active scripts/results are in
r01-consumers-20261105; map/store/source remain unchanged. No toolkit issues found.

### Active redirect: supported progression and media results

Corrected pauseMode address $0576=1 made the preserved slot11 parent run: the
level3+mask$1F patch advances to level4/event5, shows Blaze's portrait, then confirm
and settling enter the final Eliminator arena in banks8/9. Final-arena input includes
30 frames Right+A+B; framebuffer shows contender moving among vertical obstacles.
Separate level4+mask$1F patch advances to level5 and renders the terminal congratulations
screen ('AMERICAN GLADIATORS GRAND CHAMPION') with password; this is an artificially
shortened supported transition, not ordinary-gameplay reachability. Source/stack were
not edited; active flags of child10 and pause mode were the necessary dependencies.
Primary artifacts: scheduler-eliminator.png, scheduler-ending.png, final-arena.png,
completion-scheduler.py, final-arena.py and runtime.jsonl under the attempt directory.
All bounded runtime jobs have now ended; no live server is needed for current frontier.

Independent media analysis: 18 supported bank12/13 graphics streams each decode exactly
1024 output bytes; consumed source intervals are individually recorded, never whole raw
gaps. Event graphics arrays have 26-byte neighbor boundaries; indices20..24 are zero,
index25 final screen, ordinary0..19 repeat five resource pointers. DMC fixed handlers
immediately select indices2..6, proving those descriptor cells and hardware sample
intervals with $C000 slot14; alignment/truncated eight-bit length is modeled separately
from nominal endpoints. Full descriptor selector domain remains unresolved. Media
proposal union12519 bytes before overlap/classified-byte subtraction.

Sound directory inspection proves two independent128-cell pointer arrays ($9A1D/$9B1D),
each selected with eight-bit ASL. All unique stream headers have slots0..9. Conservative
static sound parsing stops at nonlinear/unproven operand contracts; 62 command lengths
supported by explicit cursor paths, 984 header/command/table records totaling2489 union
bytes. Sound handler prefix96 cells remains physically bounded by envelope table,
with natural command selector domain unresolved. Next: conflict/extent review, envelope
and dynamic-RAM asset origins, exact unknown ledger dispositions, combined leads/handoff.
No classification changes. No toolkit issues found.

### Active redirect: audio closure and mapper-independent fixed code

Sound header/command graphs now recover12113 candidate bytes with95 proven handler
length contracts and zero parse barriers. Calls are followed through substreams and
only return to their following bytes when a selected target has a supported return.
This corrected an overbroad continuation premise: random-call$B056 is followed by
its pointer cells$B05A/$B05C, and selected streams tail-jump instead of returning.
Prior provisional parse produced opcode$E5 from the pointer's high byte; the exact
counterexample is retained in sound.py comments and current sound-call-frontier.json.
It rejects that inferred continuation, not the directory or selected stream roles.
All fourteen envelope pointer cells and finite attack/release streams are supported
by parsed8F selector operands0,2,..12 and the handler's adjacent-pointer reads.
Bounded descending RAM template copies identify original ROM templates separately
from runtime digit edits. Those combined add234 candidate data bytes.
Parsed D1/D2 operands establish every DMC index0..8; hardware sample ranges now cover
through CPU$E3F0 under fixed physical slots14/15. Media union grows to15960 bytes.
Current code/data candidates have zero physical overlap; before latest fixed-code
continuation work, expected unique new classification53204 bytes, not accepted.

A concrete remaining avenue appeared in boundary review: large residuals begin with
repeated tile records and end in a shared reset stub, so they must not be classified
as whole generic gaps. Inspecting scene-stream fixed high code$F234 is actionable.
Prior conservative mapper barrier unnecessarily withheld fixed-high instructions;
NES context supports [null,null,14,15]. Continue their bank-independent code while
leaving lower pointers unresolved. This preserves sound bank overrides and the
explicit final selector5 transition; no invented lower-bank context is supplied.
Active bounded static graph job then reruns dependent record extractors; next inspect
scene-stream consumer/containers, reset-tail boundaries and only then finalize ledger.
No toolkit issue: partial context is supported and documented. ROM/map/source unchanged.

### Active redirect — scene, suffix and description batch consolidated

Scene parser/checks now establish19 unique containers from8 supported roots:
35606 bytes, exact encoded source endpoint and whole30/32-byte output chunks for
every container. Source endpoints terminate each claim; invalid following headers
are not inherited into the container. Independent attribute pointers, repeated
packed rows and separated mirrored reset stubs support generic non-code proposals
for inspected suffixes without claiming fill meaning or an original generator.
Repeated finite metasprite runs add119 record/pattern proposals after conservative
format checks; partial-run rejections are retained and will be narrowed to supported
prefixes where appropriate. Seven identical15-byte bank-end bootstrap/vector copies
have explicit storage identities and context notes: mapper-reset STA changes actual
next fetch to canonical bank15, so mirrored JMP bytes are static copied code, not
claimed observed successors. Canonical IRQ vector supports one-byte RTI at$E46E.

Followed remaining concrete pointer consumers: split palette arrays38 cells each
lead to repeated JSRE784/terminated-inline/RTS dual-use palette thunks. Event directory
six entries points to eight variants each (RNG&7), recovering finite tile rectangles.
All prefix/cell/record bounds are retained in palette-descriptions.py/checks/artifacts.
Consolidation currently proposes113258 unique new bytes (18649 code,94609 data),
zero code/data/map conflicts, leaving4878 bytes if all proposals are independently
accepted. Accepted changes remain0; actual classification144024/262160=54.94%,
118136 unknown. candidate-batch.json and exact unknown-complement.json preserve
all input-current unknown ranges and bank-split dispositions; map/store unchanged.

Current frontier: inspect nine partially decoded repeated-record prefixes and the
few residual non-code-looking islands (tile/string data, motion records) under the
same role/extent criteria; do not let an unresolved suffix reject a supported prefix.
After bounded assessment, write residual reasons, leads and combined phase handoff.
No live runtime/static server; last consolidation command finished. No toolkit issues.

### Active redirect — narrowed follow-ups before final packaging

Narrowing partial record evidence identified three complete4x5 sprites before eight
complete palette thunks in bank4; only the original whole-gap inference failed.
Rigid matching of the established JSRE784/terminated color/RTS contract recovered
additional static palette variants. Four pointer-selected five-byte FFFF-terminated
event lists and a25-row motion/OAM directory now have independent boundaries; its
lowest pointed packed path equals directory end, thirteen payload slots have stride11,
and the consumer reads10 packed coordinate deltas per path. Meaning of the eleventh
storage byte is not invented. Generic small-value/tile-string islands were inspected
as non-code independently of their full semantics. Consolidated proposal115059 bytes,
zero conflicts, residual3077 (all still unaccepted).

Remaining small islands include genuine utility code. Supported alternate-entry
contracts now being decoded: actor velocity negation (X0..11), sibling sound-register
writer preserving Y, PPU/NMI waits, randomized counter wrapper and working-pointer
rectangle entry. Final-event byte-reader helpers have a decisive classified analogue
in bank2 ($9E45..$9E8D), including shared pointer loader/RTS structure. The final copy
omits dynamic-selector reload/sequence increment; record those differences literally,
without fixing bytes, demanding natural selection or inventing useful final behavior.
All remaining short music/animation/table islands have been inspected; preserve
code-looking helpers separately from generic non-code patterns. The isolated$60 at
physical$CBDF remains materially ambiguous (unreferenced RTS alone is insufficient).
Current bounded static graph job only; then consolidate final generic dispositions,
record actual residual reasons, lead batch and compact phase handoff. No toolkit issues.

## Round 01 redirect — phase complete, independent review next

Combined proposal118135 unique new bytes:18895 code,99240 data;8234 instruction and
6994 data-record candidates; zero candidate code/data or existing-map conflicts.
Actual accepted new bytes0. Current classification144024/262160=54.94%, unknown118136.
Conditional residual if all candidates accepted:1 byte at physicalCBDF (bank6 CPU8BCF).
The isolatedRTS has no discriminating code/data role contract; no arbitrary PC test
can supply one. This is a ready batch, not a terminal whole-ROM conclusion.

Completed research and residual assessment under redirect/r01-consumers-20261105/;
exact candidate-batch.json, unknown-complement.json and immutability-check.json link
primary records. Current lead entries preserved and updated; scene-container and
palette-description leads added. Final/ending progression verified artificially in
scenarios.json, with all prior failed recipes/outcomes retained. Broad event coverage
and selected final/congratulations captures are recorded in gameplay.md.

Final arithmetic audit corrected unaligned DMC sample7: the hardware start rounds down,
but encoder reversals restore original low byte before end-minus-original-start length.
Actual hardware endpointE0D1;47-byte gap toE100 remains separately inspected generic
non-code. Independent disassembly must preserve that distinction.

All supported connected and independent avenues were reasonably investigated before
this combined handoff. Helpers/data lacking natural callers were assessed structurally;
clear non-code was not withheld for semantic/schema uncertainty. No productive frontier
is interrupted. All owned research jobs/servers stopped. ROM, map/store and commentary
source/symbol hashes match round01 commentary checkpoint exactly. Existing byte-identical
verification remains applicable because those artifacts were untouched; no new source
checkpoint created in redirect. Per-bank actual counts remain round-01/classification-counts.json.
No toolkit issues found (0bugs/0suspicions/0missing features; no blockers).

## Round 02 disassembly — active independent review

Input round-01-disassembly map/store unchanged at SHA256 4c6a44876fb0b171e856cb54a96e453676de8b6ec8feb5da8b25957ddbc42cdf. Active frontier: round-02/ACTIVE.md. Reviewing combined proposals against primary bytes and consumers; no annotations accepted yet. Commentary source preserved. A bounded all-view decode/format review is running; role/stack/context review continues independently. Current classification remains 144024/262160 (54.94%), unknown 118136. No toolkit issues found.

Round02 annotation recovery: first live command ended exit143 after5656 requests with canonical map/store still unchanged. Independent review passed and is retained. Resumable bounded chunks now write round-02/working-state.json/cursor.json from the deterministic15213-region plan; final canonical save/export follows complete validation. No toolkit defect inferred.


## Round 02 disassembly — one pass complete, ROM incomplete

ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown).

Accepted118135 unique new bytes:18895 code and99240 data, all Static. Internal store has29527 Code,101560 Data,131072 Image bytes. Unique physical union, not instruction fetch coverage. Old classifications, IDs, annotations, Observed provenance and commentary source/symbols are preserved exactly. Header and all16 CHR banks remain independently classified. Residual physicalCBDF..CBE0 (PRG06 CPU8BCF,60) remains unknown; rounded100.00% does not establish ROM completion.

Independent role/stack/bank/extent assessment: [review](round-02/review-notes.md), [all-view and finite-format checks](round-02/independent-review.json), [accepted edits](round-02/acceptance.json), [exact unknown](round-02/unknown-complement.json), [per-bank counts](round-02/classification-counts.json).8501 instruction views for8234 candidates match decoding and bytewise physical contexts. Specific data roles rechecked against primary consumers and exact finite constraints; generic fallback follows bounded pattern/reference inspection. Existing region equality and ROM/commentary preservation passed. All five ready leads verified and histories appended in leads.json.

Reused previous broad five-event level3 gameplay, final Eliminator inputs and GrandChampion screen; inspected retained final-arena/ending PNGs. No unchanged runtime experiments repeated, no new execution coverage or natural winning route claimed. Artificial progression remains separately marked in scenarios.json. Isolated RTS has no discriminating consumer/family contract; do not classify from legal decoding or absence of fetch. No promising easy continuation remains identified for this pass; residual reasoning goes through commentary/redirect, without terminal exhaustion claim.

Internal map validated and saved/exported. Fresh [checkpoint verification](round-02-disassembly-verification.json):ok=true,byte_identical=true; [map](../maps/round-02-disassembly.json), [new generated source](../generated/round-02-disassembly/game.asm). Checkpoint/current/store SHA256 c2f9133cdb39a2b157df5db1758ea669e25f441c8e97d127cee34af3b47d819d. Full current-source commentary for the enlarged scope is pending; prior readable commentary source remains untouched for preservation/carry by next phase. No editorial completion claim.

No toolkit issues found:0 confirmed bugs,0 suspected bugs,0 missing features, no blockers; [report](toolkit-issues.md). No toolkit changes, archive, cleanup, publishing or further agents.

| Bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 16 | 0 |
| PRG00 | 3803 | 4389 | 8192 | 0 |
| PRG01 | 2125 | 6067 | 8192 | 0 |
| PRG02 | 5774 | 2418 | 8192 | 0 |
| PRG03 | 490 | 7702 | 8192 | 0 |
| PRG04 | 4764 | 3428 | 8192 | 0 |
| PRG05 | 9 | 8183 | 8192 | 0 |
| PRG06 | 3894 | 4297 | 8191 | 1 |
| PRG07 | 9 | 8183 | 8192 | 0 |
| PRG08 | 3432 | 4760 | 8192 | 0 |
| PRG09 | 9 | 8183 | 8192 | 0 |
| PRG10 | 1434 | 6758 | 8192 | 0 |
| PRG11 | 9 | 8183 | 8192 | 0 |
| PRG12 | 0 | 8192 | 8192 | 0 |
| PRG13 | 9 | 8183 | 8192 | 0 |
| PRG14 | 0 | 8192 | 8192 | 0 |
| PRG15 | 3766 | 4426 | 8192 | 0 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

Retained Observed code annotation subset: 4711 instruction regions / 9995 physical bytes; new Observed regions0, new runtime experiments0. This retained subset is distinct from structural classification and is not an all-game execution-coverage claim. Every newly accepted region has structural evidence; data has role comments and code explicit effective execution views. Full current-source commentary remains pending. Details: round-02/coverage-separation.json.

## Round 02 commentary ACTIVE

Complete round01 commentary preserved under round-02/commentary-round-01-preserved.
Prepared fresh c2f9133 enlarged baseline, verified byte-identical. Exact physical
carry validation succeeded for 325 prior source fragments/structures; prior RAM and
hardware definitions retained. New-source inventory contains 591 code continuations
(after splitting at carried boundaries), 18895 new code bytes. Working through
accepted consumer records and bank-local code families. No gameplay replay or map
mutation. Full new-source readability remains active; no completion claimed.

## Current source batch

Manual semantic catalog covers all 591 new code fragments: title/briefing/Joust,
bank2 events, bank4 timed/target events, sound dispatch/modulation/DMC, final
obstacle event and bank10 ending, plus fixed graphics/scene/PPU helpers.
Contract-derived notes cover short inline allocations, finite rectangle calls and
palette thunks; no PendingCode fragment remains. Data annotations use the accepted
finite-format consumer records and local generic uncertainty after prior inspection.
Current action: emit full current source, verify byte identity, inspect symbol and
boundary consistency, then refresh review/source locations and reading copy.
Remaining frontier: verification and source-consistency checks; branch/loop names
need a semantic pass for large multi-entry helper fragments before handoff.

## Round 02 commentary complete

Full current-source readability complete across29527 code,101560 data,131072 CHR
bytes and the explicit unknown note at physical$CBDF. Preserved the entire old
commentary, carried325 validated physical sections and reviewed all591 new code
fragments plus accepted data; updated stale classification prose and meaningful
internal helper/loop symbols. All20257 scaffold entries and2258 source units are
reviewed for current readability; final editorial_audit remains pending. Fresh
byte-identical verification and reading refresh succeeded. Canonical input/map/
state are unchanged. [Handoff](handoffs/round-02-commentary.md) records hashes,
coverage, uncertainties and the residual redirect question. No gameplay replay,
classification changes or toolkit issues found. Next: redirect residual review;
classification remains262159/262160,one unknown byte. No archive/cleanup/publishing.

### Round 02 commentary metadata correction

Before redirect, coordinator caught string unit references where the review schema
requires nested unit objects. Resumed the same phase and fixed all20257 region unit
lists, retaining the2258-unit top-level index and full scaffold. Also corrected
source locations to be relative to commentary/source. Targeted schema checks
validated every required field/range/status and source path/line, with no pending
units or Code regions without units. Source/reading/ROM/map/state unchanged; existing
final byte-identical verification applies. Handoff/scope check updated; no final
editorial audit, toolkit change or archive action. No toolkit issues found.

## Round02 redirect — completed rejection review, 2026-11-06

Checkpoint round-02-disassembly c2f9133cdb39a2b157df5db1758ea669e25f441c8e97d127cee34af3b47d819d unchanged. Current-source readability complete; byte-identical round-02-commentary-verification.json retained. Research analysis/redirect/r02-residual-review-20261106/ retrieves primary loop/dispatch/callback facts, exact one-byte unknown complement and per-bank ledger. Combined rejection review distinguishes unsupported isolated RTS role from natural reachability; static return precondition proves only opcode semantics. No supported new discriminator or ready leads.262159/262160 classified; physicalCBDF..CBE0 bank6 CPU8BCF byte60 remains unknown. No runtime replay or changes to map/store/source/ROM/toolkit. Full handoff analysis/handoffs/round-02-redirect.md; no toolkit issues found, no blockers, no active owned writer/server. Terminal assessment awaits coordinator inspection.

## Coordinator final acceptance — active

Accepted terminal-incomplete rejection review after inspecting actual ROM bytes, source and finite sound callback/directory constraints; no corrected inference or useful next test remains on current premises. See final-acceptance/coordinator-rejection-review.md. ROM classification remains 262159/262160, one unknown at physical CBDF.

Final editorial prose/readability review recorded in commentary/editorial-audit.md and review.json editorial_audit. Direct configured-binary store/map validation passed; identity and unique physical evidence-union checks passed. Completion requirements audit is in final-acceptance/completion-audit.json.

Final toolkit `commentary.py audit .` launched in unified execution session 78520, confirmed still running by polling; no output/completion yet. Resume by polling this exact handle, not restarting from a status file. It validates the full review inventory, then verifies and refreshes the reading copy. If the handle completes, inspect exit/result and commentary/audit.json against current source hashes. Final fidelity acceptance is not yet proven.

archive-options.json currently selects 15 additional final documents and explicitly omits six superseded scratch/instruction documents. Current inventory has zero unrecognized Markdown; add final terminal handoff and refresh contract accounting after the audit succeeds. No archive/cleanup/publishing/toolkit modifications performed. Goal remains active until final audit, terminal handoff and final document accounting are proven.

## Coordinator terminal handoff — justified incomplete

Final audit session78520 completed exit0: ok, byte_identical, editorial_audit true;20257 regions/nested reviewed-unit occurrences passed. Reading copy refreshed. Source/definition hashes match final audit and prior coherent verification. Direct map/store validation and ROM/frozen/current identities passed. See final-acceptance/final-verification.json.

[Terminal handoff](handoffs/terminal.md) records final checkpoints/source, primary-evidence rejection review, exact one-byte residual, gameplay/evidence limits and reopening conditions. Both sequential rounds and all six phase workers completed; no ready leads or productive interrupted frontier remains. Workflow termination is justified incomplete, not whole-ROM classification or a blocked-work success.

ROM classification: 100.00% (262159 / 262160 bytes classified; 1 byte unknown).
Exact99.99961855355508%; physicalCBDF remains unknown. Readability/editorial/fidelity acceptance is complete, independently of structural classification. Archive document accounting is refreshed after this handoff is written; no archive/cleanup/publishing/toolkit modifications are authorized or performed. No toolkit issues found.
