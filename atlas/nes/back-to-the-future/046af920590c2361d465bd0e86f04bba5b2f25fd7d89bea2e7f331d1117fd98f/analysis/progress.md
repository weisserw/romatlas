# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 disassembly — active
Input: initialized placeholder analysis/state.json; no prior classifications/leads.
Verified raw baseline: maps/baseline.json, generated/baseline,
analysis/baseline-verification.json (byte-identical).
Game identity: embedded ASCII Back to the Future V1.21, Beam Software 3/5/1989;
mapper 3, fixed physical PRG banks [0,1,2,3]. Reset $8033, NMI $8461,
IRQ $8464 validated from physical vectors 0x800a..0x800f.
Static discovery reached 3,177 instruction candidates; observation supplement
currently 3,309; raw candidate evidence in analysis/round-001. Classification
not yet applied. Active server is held by Python PTY session 44573. Runtime trace
has zero detail capacity, Instruction kind, bounded aggregates; stepping uses
30-second timeout and <=10M operations. Runtime advances slowly (~5–8 frames/sec
with tracing), so early title intervals timed out but retained inspectable state.
Current frontier: title/start sequence, source inspection for stage modifier $D2,
then RAM shortcuts to representative systems. No ROM edits.
Startup progressed through copyright/title, Hill Valley map and natural street
play (frame 1229); representative up/A/B inputs accepted and traced. Durable clean
snapshot analysis/round-001/active.snapshot at frame 1471 before artificial edits.
Local control-flow establishes $04f1 as street-active latch checked at $8162,
$05c8 as post-street mode selector ($81 cafe etc) checked at $817b. Practical
shortcut testing now uses those RAM values to enter initialized bonus systems.
Accepted first annotation batch: 7,670 physical PRG bytes of code in 36 regions,
with explicit fixed PRG execution views; saved/exported analysis/state.json and
maps/current.json. Late-system shortcuts currently reach street completion/bonus
countdown, not yet cafe/school/dance; screenshots must not be treated as visits
until initialized minigame output and actual execution are confirmed.

## Round 001 disassembly — complete phase, ROM incomplete

Input checkpoint: initialized placeholder state; no consumed leads. Output:
analysis/state.json and maps/current.json. Final verified checkpoint:
`round-001-disassembly-final` (maps/round-001-disassembly-final.json,
generated/round-001-disassembly-final,
analysis/round-001-disassembly-final-verification.json). Earlier checkpoint
round-001-disassembly preserved; final adds a localized non-code record suffix.

ROM classification: 90.76% (59496 / 65552 bytes classified; 6056 bytes unknown).

| Physical section / 8 KiB bank | Code | Data/text/image | Unknown |
| --- | ---: | ---: | ---: |
| Header | 0 | 16 | 0 |
| PRG 0 | 6258 | 759 | 1175 |
| PRG 1 | 2740 | 3047 | 2405 |
| PRG 2 | 5997 | 306 | 1889 |
| PRG 3 | 1453 | 6152 | 587 |
| CHR 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 0 |

All accepted regions carry structural evidence and provenance. Code has explicit
CPU execution views for physical PRG slots [0,1,2,3]; CHR has no CPU view.
Vector-rooted discovery plus observed branches produced 7,258 unique instruction
candidates, 16,448 code bytes. Expanded/final discoveries preserved under
analysis/round-001. The final merge yielded zero additional instructions, showing
local static closure under the current roots, not proof that all code is reached.

Broad exploration: natural startup, map selection and street inputs; artificial
initialized school and dance visits, cafe visit, later street initialization
at stage $0F, driving attempt, and explicit terminal success-screen entry.
See scenarios.json and gameplay.md for exact routes, budgets, distinctions and
obstacles. Driving frames remained black; speed patch did not establish a visit
or success. A direct terminal-screen entry displayed congratulations, but does
not prove the full driving completion transition. Street-completion RAM recipe
exposed bonus-count underflow/long countdown; superseded by understood main-loop
entries, not endlessly repeated. Later street stage initialization attempted,
with distinct-frame evidence retained but full gameplay visit not claimed.

Execution is separate: trace-summary.json records 5,425 context/provenance-qualified ROM instruction
records (3,397 distinct physical starts) and 5,911 edges; zero aggregate drops. Trace detail capacity was zero
after startup; 7,664,122 detail events were intentionally dropped. Some waits
ran without tracing for speed, so this is partial execution coverage. Snapshots
are local, build-specific and contain ROM backing; never portable maps.

No input leads existed. New concrete redirect questions: interpreter handler
contracts/extent at $8B7C and indirect sites in final-discovery.json; split mixed
unknown code/data intervals (notably file $32FD..$39D3 and $5A0A..$625B);
resolve driving initialization prerequisites/black output, and natural ending
transition. Exact complement is in round-001/unknown-ranges.json.

Verification: map.validate passed; configured release binary generated ca65
source and rebuilt byte-identically. Input SHA-256 remains workspace identity.
Toolkit findings: one confirmed documentation/API inconsistency (NES-001),
zero suspected bugs or analysis blockers; see toolkit-issues.md. Workaround uses
max_instructions. Toolkit was not changed. No publish/archive performed.

## Round 001 commentary — active
Prepared commentary/ from round-001-disassembly-final; initial byte-identical verification passed. Frozen maps/state SHA256 5176915eb58cb7ad441494c0df2f162baffc5b3e64b05100545c82f4c300b94c. Current-source readability remains incomplete. Full inventory 112 regions, source/game.asm initially 7705 lines. Frontier: reviewing fixed PRG $8033..$9305; no source edits or review credit yet. Remaining all current source, including CHR and unknown-boundary explanations. Final editorial audit reserved.

Applied first full-inventory readability draft: 226 semantic routine/control-block notes, 778 control-flow labels and 378 hardware/RAM/table symbols. All 7705 original source lines inspected. Fresh verify passed byte-identically (source game.asm f49137b1775f6145d60d86e6c4696b022fd38703a4165c1993b7a5f119a80b88). Scope spans reset/main mode dispatch, route/history, player collision/animation, strip/attribute rendering, ten-object interpreter, cafe/school/dance/driving and seven-voice audio. Still active: semantic accuracy and symbol-offset pass, data/unknown notes and inventory review records; do not redirect yet.

## Round 001 commentary — complete current-source phase

Checkpoint `round-001-commentary-final`, based on unchanged `round-001-disassembly-final`. Read and explained the entire current classified listing, all accepted data/CHR boundaries and all nineteen unknown intervals. All 112 full-ROM scaffold records reviewed; 226 meaningful routine units recorded. Source contains 778 named control-flow destinations (721 semantic starts plus local return labels), 440 declared hardware/RAM/table/encoding symbols, and additional base-plus-offset substitutions for understood structure fields. Explicit widths, instruction order and raster timing remain intact.

Covered reset/main modes, route history, player motion/collision/script dispatch, nametable strip/attribute construction, timer/score/meter/HUD, ten-slot object physics/interpreter, projectile systems, cafe/school/dance/driving, CHR-backed RLE/text, fades and seven-voice audio. Corrected draft interpretations of per-sprite attributes and countdown zero handling during the accuracy pass. Unknown handler extents, mixed tables, artistic meanings and driving prerequisites are documented in commentary/uncertainties.md; no classified-source readability remains deferred.

Fresh handoff verification passed byte-identically with configured release binary; report analysis/round-001-commentary-final-verification.json. Source hashes: game.asm 0ba36906e5252fb5bfe7928e22fcd728d214afd93b9a745037904ae194073005; symbols.inc d27533cf7ed145918b0ab40044c65b171339a06f9e908de9fbe168086fca68b8. Reading copy refreshed at commentary/reading-after.asm. Structural state/map/frozen baseline hash remains 5176915eb58cb7ad441494c0df2f162baffc5b3e64b05100545c82f4c300b94c; input ROM hash unchanged. Classification remains 59496/65552 (90.76%), 6056 unknown.

Final editorial audit remains pending and was not invoked. Toolkit issue NES-001 is unchanged; no additional findings, toolkit changes, publishing or archive work. Exact remaining commentary ranges: none for current source. Next phase: redirect through analysis/handoffs/round-001-commentary.md and its concrete questions.

## Round 001 redirect — active, attempt 001
Input round-001-commentary-final; structural baseline round-001-disassembly-final.
Research directory analysis/redirect/round-001-attempt-001 contains exact nineteen-range
6056-byte unknown ledger, protected hashes and bounded configured-binary linear probes.
These probes are inspection aids, not classification. All maps/source unchanged.
Consumer review establishes title $82CA interrupt save/restore/RTI contract;
player/car cursor repeat/loop handlers; bonus handler implementations $D9FA..$DC9C
with actor/shape scratch, payload and redispatch/return contracts; street object
script, behavior and collision response continuations plus helper $BDC5.
Active frontier: audit handler-table extents and downstream non-code regions,
then targeted driving initialization prerequisite investigation. No ready batch yet;
no terminal/exhaustion claim. Existing deferred lead histories retained.

## Round 001 redirect — combined ready batch

Research completed in analysis/redirect/round-001-attempt-001; handoff
analysis/handoffs/round-001-redirect.md. Five ready leads propose 6056 residual
bytes (1991 Code, 4065 Data), zero accepted in redirect. Complete handler families,
shared helpers, title interrupt contract and adjacent consumer/pattern-supported
non-code containers researched together; weaker 45-byte standalone contracts kept
separate. Every exact unknown interval has an evidence-linked proposal disposition.
No classification or complete-ROM claim is made; accounting stays 59496/65552,
6056 unknown. leads.json preserves prior deferred records and new recipes/history.
Driving now visibly visited artificially via $D1FC; up/right steering/speed and
patched route-end latch observed. Direct entry lacks a caller return frame, so
post-RTS state cannot prove main outcome. Future caller-correct entry $8200 and
speed >= 88 type12 interaction are documented; natural ending remains unobserved.
Scenarios/gameplay preserve the distinction and old attempt history.
All runtime processes stopped. Protected ROM/map/state/source hashes match before
research. Map/state SHA256 remains 5176915eb58cb7ad441494c0df2f162baffc5b3e64b05100545c82f4c300b94c.
Disk 7.8 GiB free; research 12 MiB. No toolkit changes/new issue; existing NES-001 only.
No map changes, commentary reprepare, cleanup, archive or publication. Ready batch
returns to independent disassembly review; this is not terminal exhaustion.

## Round 002 disassembly — active independent review

Input structural checkpoint round-001-disassembly-final; current readable source
round-001-commentary-final preserved. Input state/map SHA256
5176915eb58cb7ad441494c0df2f162baffc5b3e64b05100545c82f4c300b94c.
Independent exact decoding/consumer review complete in analysis/round-002/review.md
and code-review-decode.json: 18 code intervals / 875 instructions, explicit $3E04
lookup barrier and restart $3E0A. All five ready lead IDs structurally supported;
static annotations pending. Corrected helper $9C44: unsaturated sum returned in A,
not stored; saturation stores 3 and clears fractions. No reachability inferred.

## Round 002 disassembly — phase complete, structural classification complete

Input round-001-disassembly-final / readable round-001-commentary-final. Output
analysis/state.json, maps/current.json; final fresh verified checkpoint
`round-002-disassembly-final-verified`, map SHA256
676ee71971b543a3a3f7122047bb699b5d2042e48116f72fe358c12a1e2b6849.

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

| Physical section / 8 KiB bank | Code | Data/text/image | Unknown |
| --- | ---: | ---: | ---: |
| Header | 0 | 16 | 0 |
| PRG 0 | 6612 | 1580 | 0 |
| PRG 1 | 3617 | 4575 | 0 |
| PRG 2 | 6757 | 1435 | 0 |
| PRG 3 | 1453 | 6739 | 0 |
| CHR 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 0 |

All five ready IDs verified structurally: R001-TITLE-NMI +52;
R001-INTERPRETERS +1386; R001-MIXED-GAPS +2405;
R001-BONUS-CONTRACTS +2168; R001-STANDALONE-HELPERS +45.
Total +6056 (1991 Code / 4065 Data), all Static. Independent rationale,
consumer/stack contracts and exact immutable decode: analysis/round-002/review.md,
code-review-decode.json. No rejection/blocked bytes. Exact current unknown
complement is [] in analysis/round-002/unknown-ranges.json. Specific data layout
not overclaimed where generic Data suffices. No known caller claimed for two
standalone helpers. $9C44 returns unsaturated sum rather than storing it.

Execution coverage unchanged by this static pass. Existing natural street and
artificial bonus/driving/terminal scenarios reused; natural ending transition
remains unresolved. No new Observed annotations or hardware compatibility claim.
Full current-source readability/editorial acceptance remains commentary work;
commentary/source and review records preserved byte-identically, not regenerated.

map.validate passed after each lead; each incremental batch saved/exported.
Final configured-release ca65 generation verified byte-identical; ROM hash stays
workspace identity. protected-verification.json confirms unchanged ROM/readable
source. Initial map.annotate request missing required id was rejected without
state mutation; corrected to id=0. An inadvertently created preliminary checkpoint
round-002-disassembly-final retains unchanged old map (90.76%); preserve history
and use only round-002-disassembly-final-verified as final output.
Existing toolkit documentation issue NES-001 remains; no new toolkit issue found.
See analysis/toolkit-issues.md. No toolkit edits, runtime servers, archive,
cleanup or publication. Disk 7.7 GiB free; no scratch cleanup required.

## Round 002 commentary — active
Previous commentary preserved in full at analysis/round-002/commentary-history/round-001-complete before fresh baseline preparation. Current map/state 676ee71971b543a3a3f7122047bb699b5d2042e48116f72fe358c12a1e2b6849. Carried 93 unchanged classified physical regions and prior 226 unit reviews; 24 newly classified regions pending fresh source explanation and review. Carried assembly byte-identical verified. Active frontier: title/player/object/car/bonus handler explanations, new data boundary notes, stale table/dependency wording; final verification and reading refresh still pending. Final editorial audit reserved.

## Round 002 commentary — completed current-source gate
All current classified source readable, no deferred ranges. Whole previous commentary tree preserved before new baseline in analysis/round-002/commentary-history/round-001-complete. Reused 93 classified regions/226 units; independently explained and reviewed 24 new regions, including 79 code units and six data containers. Current inventory 117 reviewed scaffold records/311 reviewed units. Dependency/semantic corrections and honest limits: commentary/progress.md, uncertainties.md and analysis/round-002/commentary-carry.json. All new provenance remains Static.

Fresh byte-identical verification passed and reading-after.asm refreshed. Checkpoint round-002-commentary-final records hashes; handoff analysis/handoffs/round-002-commentary.md. Source readability is complete; final editorial acceptance remains pending. No audit/redirect/archive/runtime/toolkit edit/publication. Structural map/state retain verified round2 SHA256 676ee71971b543a3a3f7122047bb699b5d2042e48116f72fe358c12a1e2b6849. Classification is 65552/65552, 0 unknown, separately established.

## Round 002 redirect — active
Input round-002-commentary-final; unique attempt analysis/redirect/round-002-attempt-001. Protected hashes saved. Exact map complement freshly computed: 65552 classified, zero unknown/overlap. Current frontier: source target completeness and residual standalone/data/driving consistency assessment; no map/source/toolkit edits or runtime started. Final editorial audit remains pending.

## Round 002 redirect — complete; final editorial pending
Required post-commentary scope/consistency pass complete. No ready leads, proposed/accepted bytes this pass 0, exact unknown complement empty (65552/65552 classified, no overlap). All 762 named direct control references resolve; standalone/data/driving evidence and concrete competing interpretations checked. No new structural contradiction or materially improved actionable recipe. Interpretation/natural-use limits retained separately. Evidence and protected hash equality: analysis/redirect/round-002-attempt-001. Leads history preserved and pass appended. Handoff analysis/handoffs/round-002-redirect.md to final editorial acceptance; no final audit performed. No owned processes, source/map/state/ROM/toolkit changes, archive, cleanup or publication.

## Final editorial acceptance — active
Reusing 117 reviewed regions/311 units. Targeted older mainline/meter/audio and new object/bonus/driving assessment found no structural contradiction. Corrected misleading five-bit meter lookup wording (five shifts produce a three-bit index), named palette table/scratch operands and added three bounded data units. Final audit/verification and durable findings/checkpoint pending; no runtime, archive, cleanup or toolkit changes.

## Final editorial acceptance — accepted
Substantive review: commentary/editorial-audit.md. Fixed older meter lookup index-width wording and named proven palette tables/scratch; inventory 117 reviewed regions/314 units, no deferred ranges. commentary.py audit passed with fresh byte-identical rebuild and reading refresh. Configured release store validate and map validate both returned {"ok":true}; final-editorial-verified checkpoint generated/rebuilt byte-identically. Final checkpoint analysis/final-editorial-checkpoint.json records exact hashes/reports. Classification 65552/65552 = 100.00%, zero unknown. Map/state unchanged 676ee71971b543a3a3f7122047bb699b5d2042e48116f72fe358c12a1e2b6849 and ROM unchanged workspace identity. No structural contradictions; uncertainties remain interpretation/reachability only, no compatibility/natural-ending claim. NES-001 unchanged. No archive/options changes, cleanup, publication, runtime or toolkit edit. Coordinator final selection/completion checks remain.

## Coordinator terminal acceptance — whole-ROM path

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).
Separate sequential workers completed both phase sequences and final editorial acceptance.
Final source gate: 117 reviewed regions/314 units, no deferred ranges. Final audit,
store/map validation and fresh byte-identical rebuild passed. Coordinator independently
verified contiguous evidence-bearing map coverage and all final checkpoint hashes.
Final checkpoint `final-editorial-verified`; final source/editorial checkpoint
analysis/final-editorial-checkpoint.json. No ready leads or unknown residuals.
Terminal handoff and requirement audit: analysis/handoffs/terminal.md and
analysis/current-audit.json. Archive selection is explicitly recorded in
archive-options.json and rechecked against scripts/archive.py without archive execution.
Natural-ending/uncalled-helper/semantic limits remain explicit. ROM/toolkit unchanged;
no archive, cleanup or publication. This is the documented whole-ROM terminal path,
not a blocked or interrupted stop.
