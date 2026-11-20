# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 — disassembly in progress

Actual state inspected: fresh workspace, no prior state. Baseline created once at
maps/baseline.json; generated/baseline verified byte-identical, report
analysis/baseline-verification.json. Configured binary is unchanged.
Validated vectors physical 0x2000A/0x2000C/0x2000E -> CPU C204/C1C5/C266,
initial banks [0,1,14,15]. Conservative context discovery and runtime edges yielded
1,476 physical starts; 3,061 code bytes saved to analysis/state.json so far.
Map checkpoint pending further gameplay/static work; do not treat baseline as classification.

Title confirmed locally as Blades of Steel (Konami 1988). RPC replay/evidence at
analysis/round01/rpc.jsonl; session helper analysis/round01/session.py; live PTY
46898. Initial traced 180-frame runs timed out with retained partial state; use
short traced transitions and bounded untraced waits. Gameplay menu progression in
progress, broad later-period/tournament/fight/ending scenarios still pending.
No leads existed at start. New concrete frontier: fixed bank dispatch and mapper
wrappers plus data boundaries. Internal annotations remain Static unless a physical
instruction fetch has been individually matched. No toolkit issues identified yet.

### Incremental checkpoint (round 1 continuing)

Saved internal map now has 12,765 code + 9,666 data bytes = 22,431 classified.
Direct JSR callee closure adds supported source-bank targets only; it never
assumes a normal return continuation over inline parameters. Exact FF-run data
boundary evidence is analysis/round01/data-boundaries.json.

Gameplay correction: earlier hockey was attract/demo, not confirmed player mode;
old period2/third file names do not prove milestones. Timer shortcut was real but
applied to demo. Reset was sampled every 10 frames until scene18=1 at frame240;
Start then transitioned to scene18=3 (confirmed setup), durable title-ready.snapshot.
Actual menus now being exercised. This correction preserves all earlier fetch
and static classification evidence while narrowing reachability claims.

Specific data batch: four speech bitstreams (+7,424 bytes), four descriptors and
four selector bytes (+28), static consumer proof in speech_gains.py. Map saved;
29,883 bytes classified (12,765 code,17,118 data). C34B maps selector1..4 to effective
16KiB banks2,6,6,5; EC72 computes six-byte descriptor offset then emits each packed
bit to DAC4011 for exactly encoded source-byte count. This is structure, not a claim
that each phrase has naturally played. Actual College League Exhibition setup was
visually confirmed; scene5 match intro and competitive hockey reached. Timer patch
before the intro initialized was superseded by a patch from confirmed live play.

### Verification checkpoint and final connected closure

Provisional round01-disassembly checkpoint succeeded: internal/export maps valid,
byte-identical ca65 rebuild, verification report analysis/round01-disassembly-verification.json.
At that point 48,159 /131,088 classified (15,139 Code,33,020 Data),36.74%.
Additional generic bank3 payload after tag and before FF suffix classified from
repeated count/literal/mask/text patterns after nearby PPU-stream consumer inspection;
no specific compressed-format layout claimed. Final runtime merge found eight new
direct callee starts beyond the earlier closure. Following these supported static
continuations before fresh final checkpoint; provisional checkpoint preserved.

## Round 1 — disassembly pass finished, commentary pending

ROM classification: 36.80% (48238 / 131088 bytes classified; 82850 bytes unknown).

Code:15,218;Data:33,020;unknown:82,850;regions:7,380. Physical header16 data plus
16 physical8KiB banks. Unique execution fetch metric is separate:5,179 instruction
starts /10,819 decoded bytes union across natural and artificial timelines.
All7,380 classifications carry evidence and provenance; full readable-source
commentary review is pending and is not represented by this coverage number.

| PRG bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 1935 | 1 | 6256 |
| 1 | 2223 | 2599 | 3370 |
| 2 | 2200 | 1 | 5991 |
| 3 | 0 | 127 | 8065 |
| 4 | 148 | 225 | 7819 |
| 5 | 0 | 5721 | 2471 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 61 | 1 | 8130 |
| 9 | 105 | 1534 | 6553 |
| 10 | 203 | 1 | 7988 |
| 11 | 0 | 2167 | 6025 |
| 12 | 1795 | 1 | 6396 |
| 13 | 0 | 3518 | 4674 |
| 14 | 4173 | 5 | 4014 |
| 15 | 2375 | 719 | 5098 |

Final map validation and ca65 byte-identical reconstruction passed:
analysis/round01-disassembly-final-verification.json; generated/round01-disassembly-final;
maps/round01-disassembly-final.json. Internal state and maps/current.json saved;
checkpoint map SHA256 ecde055d22cf42971290adbfd33853bdcb7abe0d42907ed8ad330a9693f68c62.
Input SHA256 unchanged. Provisional checkpoint and all history retained.

Lead outcomes: vectors/call closure/speech/uniform generic data/ending verified.
Static direct callee closure from final runtime merge added79 bytes after provisional
checkpoint; no supported concrete callee target deferred. Countdown gate blocked
for this bounded experiment, not disproved; static return/bank contracts need redirect
research. Actual Exhibition hockey visited, broad best-effort later-period timer
shortcut attempted, scene7 terminal congratulations visited and A exercised via
recorded artificial RAM transition. Attract/demo correction and exact recipes are
in analysis/gameplay.md and analysis/scenarios.json. Later intermission, fights,
penalties, overtime and tournament rounds remain unvisited for documented missing
state contracts; do not replay unchanged failed waits. No terminal exhaustion claim.

Handoff: analysis/handoffs/round01-disassembly.md. Evidence/unknown complement/per-bank
counts: analysis/round01/counts.json; live PTY46898. No toolkit issues found;
see analysis/toolkit-issues.md. ROM incomplete; next phase is current-source commentary.

## Round 1 — commentary active

Prepared once using supported toolkit commentary.py prepare. Frozen baseline and
phase1-state retained in commentary; initial source reconstruction byte-identical.
Current phase is commentary, not redirect. Readability frontier at preparation:
all classified code/data and unknown boundary notes pending; no source edits or
semantic review credit yet. Full scope is 15,218 code +33,020 data bytes and
82,850 unknown bytes, 7,626 review scaffold entries. Source game.asm has44,458
lines because each accepted instruction has its own physical region. Work will
review meaningful contiguous routines/blocks, beginning fixed-bank initialization,
interrupts and mapper/dispatch helpers, then every switchable-bank code range.
Exact remaining scope is all review.json entries with status pending.

### Commentary readability batch 1

80 hand-inspected units persisted;2,762 physical bytes reviewed. Fixed-bank startup,
NMI, PPU, mapper, input, scene and arithmetic mechanisms and initial bank-zero sprite
routines now explained; hardware/RAM symbols applied consistently in operands.
Fresh commentary verify passed byte-identical. Commentary remains active/incomplete.
Exact remaining source is review.json pending entries; see commentary/progress.md.
Structural question C01 in commentary/uncertainties.md flags bank0 $000328..$0003F1
as pointer-like Code overlapping the known $8316 table consumer, without modifying
maps or claiming an executable contract. Final audit remains pending.

### Readability batch 2

170 explicit hand-inspected units now persisted. Reviewed bytes by kind: {'Code': 6089, 'Data': 33020, 'Unknown': 82850}.
All bank-zero accepted Code through physical $0035E9 is explained, including
actor contact windows, control dispatch, motion scripts, team input paths and
animation mechanics. Generic data notes reuse the retained boundary/consumer
inspection, and every unknown interval now has an honest exact-boundary note.
Fresh verification again passed byte-identical. Current phase remains commentary.
Exact remaining classified Code intervals: analysis/round01/commentary/pending-ranges.json
(also status=pending in review.json), beginning bank1 $004011 and bank2 $008011,
then banks4/5/6 and fixed-bank code after $01CC9F. No phase handoff yet.

### Readability batches 3 and 4

276 hand-inspected units persisted; reviewed bytes {'Code': 10601, 'Data': 33020, 'Unknown': 82850}. Bank1 camera/strip,
menu choice/highlight and presentation mechanics, smaller bank2/4/5 PPU queues,
and all classified bank6 sound-engine code are now explained. Two further frozen
code/data conflicts are recorded in uncertainties.md as C02. Fresh verification
passed byte-identical after each coherent batch.

The earlier bank-zero blanket statement omitted the three-byte DEC at physical
$002DD8; it remains pending and will be covered with the next fixed-code batch.
Exact remaining scope is pending-ranges.json: that instruction plus fixed-bank
code from $01CC9F onward. Current phase remains commentary; no handoff yet.

### Round 1 commentary phase complete

All current-source readability work is complete:15,218 classified Code bytes,
33,020 classified Data bytes and exact uncertainty/boundary notes for82,850 Unknown
bytes. All7,626 scaffold entries reviewed;381 meaningful hand-inspected units with
source line locations and493 descriptive routine/branch entry labels. No pending
readability ranges remain (pending-ranges.json is empty). Symbols now consistently
cover hardware, shared scratch, actor fields, sound-channel arrays and known targets;
reused scene bytes60/61 have neutral global aliases to avoid false permanent roles.

Examples: OAM metasprite expansion/sorting, PPU token decoding and queue formats,
reset/NMI bank protection, double-sampled controller edges, actor motion/drag and
packed collision probes, angle quantization and scaled squared distance, plus
sound arbitration/envelopes/DMC setup. Uncertain game purposes use mechanical names.
C01/C02/C03 in uncertainties.md preserve concrete frozen code-versus-inline-data
conflicts for redirect. No map/classification or ROM change was made.

Final fresh commentary verify returned ok=true,byte_identical=true. Source hashes:
game.asm68b8914ded989b70c0debd79399f6c01de17bf159616bab9ea69cdb346d6efd4;
symbols.incb5cc634f03a7bdb8b9b58cbaa3292a4af57cbf9f775e34bbfb22c787414110c9.
reading-after.asm refreshed successfully afterward. Scope/fidelity assessment is
commentary/scope-assessment.json; exact verification is commentary/verification.json.
Frozen/current map hashes remain ecde055d22cf42971290adbfd33853bdcb7abe0d42907ed8ad330a9693f68c62.
No final audit was run; editorial_audit remains pending for final acceptance.
ROM classification remains incomplete at36.80%; readability completion does not
resolve the unknown byte complement or establish full game/hardware compatibility.
No toolkit issues found; report analysis/toolkit-issues.md preserved.
Current worker phase is commentary, completed; coordinator may now select redirect.
Handoff: analysis/handoffs/round01-commentary.md.

## Round 1 — redirect active

Input checkpoint round01-disassembly-final; map SHA256 ecde055d22cf42971290adbfd33853bdcb7abe0d42907ed8ad330a9693f68c62. Current-source commentary accepted as complete; map remains unchanged. Research directory analysis/redirect/r01-structural-contracts. Initial frontier: C01 metasprite pointer overlap (note source's table comment may have wrong base), C02 inline PPU bytes, C03 animation lookup; then clock/faceoff progression, static return/bank contracts, graphic boundaries. No new lead ready yet. Disk 4GiB available; no cleanup performed.

### Redirect structural and progression checkpoint

C01: two tables ($8316..$83F5, $83F6..$8469) resolve to142 unique count+4N metasprite records, exactly adjacent physical $00047A..$00115C (exclusive). $8316/$8317 remain genuine branch code/dummy selector0 slot; reclassification proposal begins $000328. C02 payload $0040BF..$0040CF includes first FF terminator immediately before real handler; direction deltas $0043EB..$0043ED are separate two-byte payload. C03 eight-byte direction records anchored by fixed animation consumer; bound inspection ongoing. Evidence metasprites.py/json and decode artifacts in attempt directory.
Graphics: consumer C818/C89D pointer and C8BB bank tables yield15 bounded terminated token streams including bank4 physical $010011..$0112CF and $0121F6..$012328, $0128E4..$012F63; fixed stream $01C8DA..$01C904 and prior generic bank3 payload. All15 parse without degenerate tokens or bank crossing; independent neighboring boundaries being reviewed before final proposal.
Clock: expiry-frontier restored, controls released; explicit timer [0,0,0,1], divider0 and6C=0 led state30 6->A with zero clock in60frames; next untraced progression reached statesB andC. Changed evidence resolves the prior failed-wait premise. Selected captures/checkpoints saved; connected later-state progression remains active. Map untouched.

### Redirect continued batch

Counter34 now independently identified: D1C5 increments it after period-summary completion, D171 tests>=2 for final-score comparison and D1C9 tests>=3 for terminal/overtime route. Initial expiry route observed34=0->1, establishing second period (not merely goal reset). Live screenshot progress-11.png and timer19:48 retained. Second expiry route now active; if stateB waits on bank1 $9CC4 dispatcher66, inspect its actual substate before further progression.
New connected graphic avenue: fixed PPU-template pointer table $C961 has36 entries ending exactly first payload $C9A9.35 distinct template starts (one suffix alias), FE orFF terminators via C93E/C95E, FD internal continuation; union contiguous to $CC8F next accepted routine.814-byte region $01C971..$01CC9F supports specific pointer/template data. Initial FF-only scan falsified the narrower termination premise; corrected from consumer branch and endpoints, not threshold relaxation. templates.py/json preserves exact entries.
Return proof generator now172 summaries,1283 proposed instructions2776 distinct-context bytes. Bank-wrapper safe-save-slot condition remains conservative; C3CB queue copy may write0700+X and requires caller queue bound before asserting saved07E0 preservation. No inferred generic wrapper permission. Entire map unchanged.

### Redirect summary and intermission graph expanded

Actual second-period expiry reached stateB/31=2 and arcade screenshot period2-step8.png. Static trace identifies66=8->bank1 A2D8, then6-entry80 dispatcher and two84 dispatcher variants (19/13 entries), all handlers and supported ordinary helpers investigated together. First minigame timeout completes naturally with no gameplay shortcut and period counter34=2, third-period live state7 with19:31. Historical later_game.py batch3 timer edit was while summary, so is not a third expiry; exact correction in notes.md. Mask table physical00640F..006414 separated from impossible fallthrough. static-contracts.json/dispatch-tables.json retain combined candidates; map unchanged. Frontier: actual third expiry/tie route, representative fight-state initialization, final C03/data boundary and proposal review, then combined leads.

### Redirect broad progression and structural batch nearly consolidated

C03 complete non-code interval physical01DF49..01E063 (282bytes), with first64bytes eight direction records and remaining mixed frame/attribute/descriptors (no unsupported exact schema). Specific nine-state descriptor domains overlap apparent bases; generic fallback for mixed remainder is honest. Existing wrong Code there is2bytes, not3 (primary map/source bounds verified).
17 explicit PPU chunk specs at bank2 8097/80A8/80CA/80EC yield contiguous raw payload00811E..00A99E; tables+payload0080A7..00A99E, chunks max80 and cursor<=128 yields lastqueue07D4 below saved07E0. Conditional C3CB contract recorded in read-only proof. Current static batch2439 instruction contexts,5288 context bytes /5261 unique physical bytes,210 summaries; actual accepted gains stillzero. Data proposals37862 unique bytes:219 currentCode correction,15888 alreadyData,21755 Unknown. No code/data proposal overlap. Exact unchanged unknown complement82850 split dispositions in unknown-ledger.json (ready data/code versus materially unresolved); not terminal.
Terminal_probe established third-period live34=2, timer shortcut and tiedscore07E5/07E9=0 -> summary then34=3 state17 shootout, screenshot third-expiry-6.png. Separate initialized state16/80=0/6D=0 reached visible fight screen state16-2.png. Prior state17 shortcut was penalty/shootout arena, not fight; correction retained. Fight, shootout and arcade controls now being exercised with bounded inputs, actual fetch contexts recorded. New selected bank4 four-handler dispatcher investigated alongside intermission graphs. Map unchanged; no toolkit issues found.

### Redirect connected/source-wide dispatch closure completed

Controls completed and captured: fight power display changed afterA/B/right; arcade moved to NICE SHOT afterA/right; shootoutA/right/B exercised but no completed shot claimed. Actual artificial fetch contexts retained in contexts-exercised.json and observed-roots.json.
Source-wide C79A triage found10 (not preliminary estimate12) tables with independently accepted next-handler boundaries.13 pointer-pattern prefixes plus two initially ambiguous tables were then inspected using full local pointers and coherent handlers; nested continuations added6 and3 further tables. All newly exposed inline dispatchers in this connected source graph now have inspected tables/handlers; nested-frontier.json is empty. No arbitrary dispatcher/actor quota or recursive gameplay sweep was used.
Correction to first8-pointer heuristic: source002E4F is13 entries ending handlerAE5C (11th target provides the earlier endpoint), not a55-entry table containing nonpointers. Source01C487 scene table is8 entries before independently accepted C48A JMP, while first forward targetC48D lies3bytes later. Preserve original triage as method history; these counterexamples reject first8-only endpoint inference, not those bounded data claims.
Current proposal manifest:30318 Unknown->Data,14500 Unknown->Code,219 currentCode->Data,15888 alreadyData role refinements. Unknown complement remains82850 in unchanged map; hypothetical residual38032 only if all proposals independently accepted. Code/data proposal overlapzero.314 stack/bank summaries; selected PPU C816/C818 manual finite-stream return contract documented and each opened call selector reviewed (literal evenX0..28 or04/08/0A lookup). Full instruction/context/evidence manifests code-proposals.json, data-proposals.json, validation.json and unknown-ledger.json. No classification performed.
Actual accepted checkpoint remains round01-disassembly-final/map ecde055d22cf42971290adbfd33853bdcb7abe0d42907ed8ad330a9693f68c62. Frontier now final combined lead/handoff publication locally; ready disassembly review, no terminal exhaustion claimed. Remaining conservative barriers are sound page1 stack reservation/dynamic writes, bank save-slot wrappers lacking nonclobber proof, and indirect state/script domains without bounded pointer layout (not missing natural callers). Details in final deferrals record.

## Round 1 — redirect complete, disassembly review pending

Combined ready batch saved in analysis/handoffs/round01-redirect.md; analysis/leads.json preserves all historical entries/outcomes and adds7 IDs plus materially improves R1-STATIC-RETURN/R1-PERIOD-GATE (9 ready). Exact accepted state unchanged:48,238 classified,82,850 Unknown,36.80%; checkpoint round01-disassembly-final map SHA256 ecde055d22cf42971290adbfd33853bdcb7abe0d42907ed8ad330a9693f68c62. Proposed30,318 UnknownData +14,500 unique UnknownCode,219 currentCode corrections and15,888 existingData refinements; hypothetical residual38,032 only after independent acceptance.314 summaries and762 matched new artificial fetch starts. These counts are final and did not change in validation; earlier smaller figures were incremental expansion, not accepted progress.

scenarios.json now records real counter-based second/third periods, arcade NICE SHOT, initialized fight power response and tied-game shootout controls, with old attempts preserved. Actual deferrals/method corrections in analysis/redirect/r01-structural-contracts/deferrals.md; no identified selected-dispatch frontier remains (nested-frontier.json empty). No terminal exhaustion claim; next phase is independent disassembly of combinedbatch, then full current-source commentary. Source readability/fidelity unchanged from accepted commentary handoff; no editorial final acceptance run. No toolkit issues found. ROM/map hashes and exact ledger/proposal counts independently checked; no ROM/toolkit/map/source mutation, cleanup, archive or publication.

## Round 2 — disassembly review in progress

Phase: disassembly, one pass; input checkpoint round01-disassembly-final (map SHA256
 ecde055d22cf42971290adbfd33853bdcb7abe0d42907ed8ad330a9693f68c62).
Reading the compact redirect handoff and independently reviewing nine ready leads,
exact code corrections, bank views, consumer bounds and caller-return summaries.
Accepted baseline remains48238 classified /82850 Unknown until annotations are saved.
Existing commentary and all prior phase evidence are preserved. Review frontier:
consumer-anchored data batch first, then code graph/conditional return contracts and
exact matched artificial fetch provenance. No toolkit/ROM changes authorized.

### Round2 data acceptance checkpoint (continuing)

Accepted independently checked30318 Unknown->Data and219 prior false Code->Data;
refined15888 existingData bytes. Internal state/maps/current saved and map.validate
passed. Current classified78556 /52532 Unknown (Code14999, Data63557). Data history
including removed erroneous instruction comments retained in analysis/round02/data-changes.json;
original commentary untouched. BCC physical000326..328 preserved as90 F2. Every
proposed instruction/context was separately decoded by configured toolkit:6674
unique starts /6789 contexts /14500 bytes;762 exact artificial fetched starts.
Independent data verification:142 adjacent sprite records,15 finite PPU streams,
36 template pointers35 distinct starts with suffix alias,17+6 copy-spec unions.
See round02/independent-validation.json and review-notes.md. No checkpoint generated
yet: code graph/conditional return audit continues, then remaining easy gains.

### Round2 code acceptance checkpoint (continuing)

Saved and validated6674 new instruction regions,14500 unique bytes across6789
explicit bank views. Classification now93056 /38032 Unknown (Code29499, Data63557).
762 starts have exact artificial fetch identity; Observed provenance is assigned
only when every attached view matched, otherwise Static with exact matched views
listed in evidence.304 ordinary/10 conditional graphs independently reviewed;
647 ordinary-return edges audited. See round02/accepted-code.json, graph-audit.json,
review-notes.md. Sound page1 and other unproved wrapper families remain barriers.
Fresh connected easy-gain inspection found bank5's existing8002 consumer describes
10 eight-byte graphics records, two terminated script sets/coordinate arrays and
six23-tile plus four50-tile payloads exactly to9759; independently validating this
new5773-byte data container. Bank1 PPU cell/block/command data also being inspected
from B778/B7F8 consumers. No additional gameplay replay: prior milestone evidence
already covers representative later periods/intermission/fight/shootout/ending,
unchanged waits/inputs have no new discrimination. No terminal exhaustion claimed.

### Round2 connected acceptance checkpoint (continuing)

Saved and validated103849 classified /27239 Unknown after connected gains:
bank5 descriptors/scripts/destinations/16-byte tiles5773 newData; bank1 94 counted
PPU/text command records1898 newData (+1 existingData refined); bank1 paired block
pointers/cellmaps/tile data3010 newData; typed block-readerB778..B7E7 112 newCode
(60 starts). Precise manifests round02/connected-data-validation.json,
connected-changes.json and block-reader-decode.json; review-notes.md preserves
parser correction and local entry/record contracts. Natural caller/selection for
new block reader unresolved, structural code/data contracts independently coherent.
Final bounded residual inspection narrowed low-valued pattern candidate by6bytes:
47 denseLE pointer words019F56..019FB4 plus34 repeated65-byte low-valued records
019FB4..01A856. Proposed2304 genericData bytes: no exact waveform generator/full
selector schema claimed; consumer semantics unresolved. Different packed content
afterA846 retainedUnknown. Annotation/validation in progress; final count/immutable
checkpoint and byte-identical verification next. No terminal exhaustion decision.

## Round 2 — disassembly pass complete, ROM incomplete

Current phase: disassembly complete; next full current-source commentary, then redirect.
ROM classification: 81.00% (106178 / 131088 bytes classified; 24910 bytes unknown).

Code:29636;Data:76542;Unknown:24910;all14078 regions
carry evidence/provenance. Physical header16 Data plus16 physical8KiB PRG banks.
Net57940 Unknown bytes classified this pass;219 erroneous Code bytes corrected toData.
Existing commentary/source and prior accepted maps/evidence remain untouched.

| PRG bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 3707 | 3677 | 808 |
| 1 | 3601 | 2713 | 1878 |
| 2 | 6291 | 167 | 1734 |
| 3 | 1795 | 5116 | 1281 |
| 4 | 148 | 8042 | 2 |
| 5 | 25 | 8167 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 1817 | 4887 | 1488 |
| 9 | 1367 | 3556 | 3269 |
| 10 | 351 | 7839 | 2 |
| 11 | 0 | 8192 | 0 |
| 12 | 2141 | 187 | 5864 |
| 13 | 0 | 5636 | 2556 |
| 14 | 5485 | 1161 | 1546 |
| 15 | 2908 | 802 | 4482 |

Internalstate andmaps/current saved, map.validate passed. Final accepted immutable
checkpoint:round02-disassembly-closure; maps/round02-disassembly-closure.json,
generated/round02-disassembly-closure, analysis/round02-disassembly-closure-verification.json.
Map/state SHA256e734fb13fbb234220432d10fd198d4336bdf369b1830ce8dbcb594b25b0fbac2. Whole-ROM ca65 rebuild byte-identical;
ROM SHA256 unchangedc0637229cffcc7eab627423ebfc47e561ef8c64df06a1a6b22e2b26ed8fb37f7. Earlier same-pass
round02-disassembly-final checkpoint preserved (106153 classified/24935Unknown);
closure adds25-byte initializer after local C68C proof. This does not supersede or
rewrite prior checkpoint evidence. All hashes/complement/bank counts inround02/counts.json.

Nine ready leads verified with preserved historical outcomes: C01/C02/C03 code
corrections/data, PPUstreams/templates/chunks, dispatch, staticreturn andperiodgate.
Combined initial batch30318 newData+14500 newCode accepted;219 corrections and15888
existingData refinements. Additional connected gains:10681 newData frombank5 graphics
container/bank1commands/cellmaps/tiles,2304 generic pointer/low-valued patternedData;
112-byte B778 blockreader and25-byte B78E initializer StaticCode. All boundaries,
parser/proof corrections and local calling limits inround02/review-notes.md,
independent-validation.json, graph-audit.json, connected-data-validation.json,
pattern-validation.json andclear-closure-decode.json.762 newly matched exact
physical+PC+bank artificial fetched starts;762 regionsObserved,13316Static.
This is separate from executioncoverage and from natural gameplay reachability;
no new emulation needed because saved representative scenarios remain valid.

C68C bounded indirect0300..06FF clear proof corrects genericchecker rejection;
global existing-root closure added0bytes. B78E initializer independently supported
by complete PHA/PLA-preservation/init/knowncallee contract and data boundaries, not
an invented naturalcaller. Initial scratch A98E root was a decimal-offset reading
error, blocked byexistingData before annotations; correctedphysical00B79E..B7B7 is
authoritative. Two isolated CLC/RTS tails remainUnknown. Soundpage1 versus maximum
service/NMI stack reservation, unresolved saved-bank nonclobber and indirect selector
domains remain concrete redirect frontiers; no universal wrapper certification.

Easy-gain pass stops after full combined batch, connected consumers and one bounded
residual pattern/clear-loop review. Reused broader laterperiod/arcade/fight/shootout/
ending evidence rather than unchanged waits. Natural tournament wins/cup route and
completedfight/shootout remain unclaimed. No terminal exhaustion claim/rejectionreview;
remaining useful research goes toredirect after commentary. No toolkit issues found;
analysis/toolkit-issues.md preserved. ROM/toolkit/archive unchanged;no cleanup/publication.

## Round 2 — commentary active: preserved baseline, preparation in progress

Current phase is commentary on round02-disassembly-closure (map e734fb13fbb234220432d10fd198d4336bdf369b1830ce8dbcb594b25b0fbac2). Entire prior commentary moved intact to analysis/round-2/commentary-round01-preserved before fresh supported preparation. File hashes are in analysis/round-2/commentary-preservation.json. Fresh preparation is running; no source edits or new review credit yet. Frontier: carry valid prior physical units and symbols, invalidate 219 prior Code-to-Data bytes and affected source dependencies, then inspect all newly classified routines/data and unknown interval notes. Readability incomplete; no handoff.

### Round 2 commentary data milestone

Fresh supported baseline and carried-source byte-identical verification passed. All current76,542 Data bytes and24,910 Unknown boundary notes reviewed, including80 explicit new data structures. Code reviewed14,164/pending15,472 bytes;7,146 Code records remain, including27 old incomplete-code dependency claims. Exact frontier and source locations are persisted under analysis/round-2/commentary. Commentary remains active and incomplete; no redirect handoff.

### Round 2 commentary fixed-bank milestone

Small bank2/5/6 and fixed-bank code review saved, including dependency repairs and the twelve-byte template correction. Exact remaining readability is {'Code': 12722, 'Data': 0, 'Unknown': 0}; structured frontier under analysis/round-2/commentary. Large banks0/1/4 remain in the same active phase. No handoff, redirect or final editorial audit.

### Round 2 commentary bank-zero milestone

Current-source bank0/2/5/6/7Code explained; bank1/4 remaining. Code reviewed20,703/pending8,933bytes, 4,121pendingCode records. AllData76,542 andUnknown24,910 notes complete. Concrete carried-dependency prose revised, unknown physical gaps retained. Fresh verification correcting a commentary symbol retarget; no structural map or input changes. Commentary active.

### Round 2 commentary inventory complete, consistency active

All29,636Code/76,542Data/24,910Unknown-note bytes covered;14,274scaffold entries reviewed and820meaningful units. No pending ranges. Final consistency work still active: context-scoped symbols/branch/data names, remaining old missing-code prose and27carry-dependency dispositions. C04 bank4delta/JSRconflict saved for redirect with exact source note. Fresh batch verify passed including symbol-target repair; final matching verify/read/scope assessment/handoff still required before commentary completion. No final editorial audit.

### Round 2 commentary complete

All current-source readability is complete:29,636Code and76,542Data bytes, with exact honest boundary/uncertainty notes for24,910Unknown bytes. All14,274 scaffold records reviewed;820 meaningful units and1,002 descriptive source labels. Exact pending-ranges.json is empty. All27 carried dependency questions rechecked; four contradicted prior units invalidated. Entire prior commentary remains intact under analysis/round-2/commentary-round01-preserved with its preservation hash manifest.

Coverage includes camera tile/attribute queues, counted record/token/template formats, metasprites, actor contact/target/control mechanics, formations, staged scripts, tournament bracket traversal, shootout scoring/physics, fighter input/AI/animation/contact/effects, intermission activity, sound parsing/envelopes and fixed-bank state/PPU/math/speech helpers. Context-scoped fight/shootout RAM aliases avoid false permanent meanings for reused storage. Direct control-transfer symbols derive from the frozen physical instruction destinations. Corrected twelve-byte presentation-template geometry and repeated-byte PPU helper descriptions; literal and repeat formats are explicitly distinct.

C01/C02/C03 old instruction interpretations are superseded by exact consumer-backed data formats. New C04 remains an explicit structural question:physical012853..012857/CPUA843[8,9,14,15] is consumed as signed delta words but its firstthree bytes remain frozenStaticCode JSR$E000. Source explains the conflict without inventing a routine; exact evidence/bounded redirect check is conflict-C04.json. No map/ROM edit.

Final fresh verify passed ok=true,byte_identical=true. game.asmSHA25657445c04e4506fac3ff000c3f3565b702c99c7a563fe2d49d6c6e4141dce4ee8;symbols.incSHA25679cb86a997a62b1ceff9ba4369f2904c5f5e983458d39a61bb11b49948962c58. reading-after.asm refreshed after verification. Scope/hashes/examples:commentary/scope-assessment.json; report:commentary/verification.json. Frozen/currentmap/state remain e734fb13fbb234220432d10fd198d4336bdf369b1830ce8dbcb594b25b0fbac2. ROMhash unchanged.

Current phase commentary, complete; coordinator may select redirect. Whole-ROM classification remains81.00%, incomplete. Final editorial acceptance remains separately required, including terminal incomplete outcomes; editorial_audit pending and no final audit run now. No toolkit edits, disassembly/redirect, archive, deletion, cleanup, publication or sharing. Handoff:analysis/handoffs/round02-commentary.md.

## Round 2 — redirect in progress

Worker resumed from completed full-source commentary and round02-disassembly-closure (map/state e734fb13fbb234220432d10fd198d4336bdf369b1830ce8dbcb594b25b0fbac2). Research directory analysis/redirect/r02-structural-frontier; active frontier in frontier.md. Accepted classified106178, Unknown24910 (81.00%); proposals separate. Reviewing C04 exact evidence then connected and independent remaining promising avenues. No toolkit issues found so far.

### Redirect durable checkpoint — C04, bank1, sound

C04 primary review complete (4Data proposal,3Code correction+1Unknown); bank1 five-state graph335new starts655UnknownCode,174UnknownData; sound descriptor/pattern/command proposal7834UnknownData. Evidence under analysis/redirect/r02-structural-frontier/{C04-review,bank1-graph,sound-data,data-conflicting-edges}.json. LocalC340 reset stack/bank contract now proved under NMIoff,S=FF; arbitrary sound service remains locally unresolved. Current frontier: explicitC459 initializer wrapper and CC8F template6 queue capacity/nonclobber, then remaining concrete wrapper domains. Findings provisional pending disassembly; accepted counts unchanged.

### Redirect checkpoint — finite sound startup and bank wrappers

Bounded non-speechC31F startup contract (request<=51,NMIoff,S>=B0) yielded229candidateviews504view-bytes after Z-pruning; exact uniquephysical count pending. Corrected DE1E rejection: knownzeroDF32/DF37 branch cannot fall intoData1DF49; DDF1/DE1E now ordinary under per-sound-call contracts. wrapper-review.json adds96UnknownCodebytes by complete select/call/restore structure, including explicitC459/initializer conditionalqueue<=B4 proof. Currentfrontier: canonicalize combinedproposals, inspect newdownstreamtables and sevenremaininglocalized blockers, then combinedhandoff. Accepted map/source/checkpoint/counts unchanged.

## Round 2 — redirect complete; round 3 disassembly pending

Combinedfive ready leads: newR2-C04,R2-SOUND-DATA,R2-DMC-SAMPLES; reopenedR1-STATIC-RETURN,R2-CONNECTED-CONSUMERS withhistorypreserved. Finalmanifest analysis/redirect/r02-structural-frontier/proposals.json:604physicalstarts1262UnknownCode+12481UnknownData,3priorCodecorrections,13743proposedUnknownbytes,0accepted. DMCexacthardwareconsumerfouruniquespans/shared2/3alias plusFF/mapperbanktable/texttail added4106; softwarevoicetailinspectionadded352. Earliercounts superseded; acceptedmap/state/source/ROMunchanged. Unknownledger198ranges24910 acceptedUnknown refreshed; projected11167residualonlyifallaccepted.

ROMclassification remains81.00%(106178/131088,24910Unknown), checkpoint round02-disassembly-closure,e734fb13fbb234220432d10fd198d4336bdf369b1830ce8dbcb594b25b0fbac2. Currentreadability820units1002labels complete; commentary/verification.json freshbyte-identical inputverification reused, finaleditorialpending. PerC31Fstartupcontract keepsrequest<=51,NMIoff/nonreentrant,S>=B0; max7additionalstackbytes correctedfromprovisional6. C459conditionalqueue<=B4 nonclobber andknownZDF37 correction retained.

Connected/independentpromisingavenues followed throughnewreaders/tables/sampletail. RemainingC3F0/intermissiongeneralreturn hasactualqueue07E0/nestedscriptdomain prerequisite andalreadyclassifiedtargets; mapperC2F7/C304contextchange notsamebankreturn. No uninspected supportedunknownhandlerfrontier. This isproductivebatchclosure, notterminalexhaustion. Handoff analysis/handoffs/round02-redirect.md; validation.json hashes/counts. Savedgameplaybest-effortscenarios reused. No toolkitissuesfound; noarchive/cleanup/publication/toolkitedits.

## Round 3 — disassembly active

Resumed accepted round02-disassembly-closure, map/state e734fb13fbb234220432d10fd198d4336bdf369b1830ce8dbcb594b25b0fbac2; current accepted classification106178/131088,24910Unknown(81.00%). Five ready leads independently under review; round02 redirect manifests immutable. Mechanical physical/context decoding confirms604starts1262UnknownCode,12481UnknownData and3Code corrections; independent DMC specification arithmetic confirms four unique spans and shared2/3alias.87sound descriptors independently recomputed. No annotations accepted yet. Evidence analysis/round03/independent-validation.json,decode-review.json,data-review.json. Current frontier: conditional sound/mapper contracts, direct graph conflicts and changed-neighbor closure before acceptance. Existing commentary820units1002labels preserved on prior map; no commentary work here.

## Round 3 — disassembly complete, classification incomplete

ROM classification: 91.51% (119955 / 131088 bytes classified; 11133 bytes unknown).

Five ready leads verified. Accepted1280newCode bytes(611starts),12497newData bytes,3priorCode->Data corrections; net13777Unknown classified. Original604starts1262Code+12481Data proposal accepted with19effective fixed-wrapper view corrections after C2F1. Connected explicit bank1 C3B5/8663 parameter helper added18Code+16Data beyond manifest. AllnewclassificationStatic; no new execution/naturalreachability credit. Exact sound request<=51,NMIoff/nonreentrant,S>=B0 and per-call queue/bank conditions retained; actualcallerconditions unproved. ResetC340 actual initialization proof separately confirmed. C04 exact4Data; no falseBEQ fallthrough. DMC fourunique spans/shared2/3alias andoneexistingData overlap independently checked;24packed residual separate genericData, no enlarged hardwareextent.

Internalmap/state saved; map.validate passed. Fresh checkpoint round03-disassembly-closure, map/state SHA256605b2a81e96a8a392a7f654cbd9b9fcbf3d009103e2d91b38c947f1e25b0ca0f. Full ca65 rebuild ok=true,byte_identical=true; analysis/round03-disassembly-closure-verification.json. InputROM SHA256unchanged. Exactbankcounts/complement analysis/round03/counts.json; primary review/changes inanalysis/round03. Header16classified independently of16PRGbanks.

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 3735 | 3677 | 780 |
| 1 | 3809 | 2715 | 1668 |
| 2 | 6391 | 183 | 1618 |
| 3 | 2450 | 5290 | 452 |
| 4 | 148 | 8042 | 2 |
| 5 | 25 | 8167 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 1817 | 4887 | 1488 |
| 9 | 1428 | 3572 | 3192 |
| 10 | 351 | 7839 | 2 |
| 11 | 0 | 8192 | 0 |
| 12 | 2141 | 5817 | 234 |
| 13 | 0 | 8192 | 0 |
| 14 | 5710 | 1161 | 1321 |
| 15 | 2908 | 4908 | 376 |

Easy-gain pass complete after allfive leads, correctedcontext graph andconnectedhelper/parameter closure plus3changedneighborfragments reviewed.108directtransfers resolve toaccepted/newstarts inactualcontext. RemainingC745 prefix needs alternateentrycontract; bank1 52C5 boundary/initialization origin unresolved; D0C9 needsA882 bank/return/queuecontract. GeneralC3F0 queue07E0/nestedscriptdomains andactualsoundcallerinterrupt/stackfacts remain redirectresearch, not broad rejection. Exact proposals/residual ledger staylocal; no terminalexhaustionclaim. Savedlaterperiod/arcade/fight/shootout/artificialending scenarios reused; no new shortcut or unchanged replay. scenarios.json round3reuse recordslimits.

Existing fullcommentary820units1002labels preservedonoldmap; newmap requiresfreshcurrent-sourcecommentary andcarryvalidation. No commentary/redirect workperformed. Finaleditorialaudit remainsseparatepending. No toolkit issues found; analysis/toolkit-issues.md preserved. No ROM/toolkitedits, archive, deletion/cleanup, publication/sharing. Handoff analysis/handoffs/round03-disassembly.md.

## Round 3 commentary active

Entire prior commentary preserved at analysis/round-3/commentary-round02-preserved with per-file hash manifest commentary-preservation.json. Fresh supported preparation completed on checkpoint round03-disassembly-closure; current phase commentary, carry validation underway. Remaining: changed code/data, all affected dependencies, source inventory and unknown notes, fresh verification and reading copy. Final editorial acceptance remains pending.

Round3 commentary milestone: 14,234 records carried; C04 unit invalidated and 19 changed-view bodies regenerated. Current source has coverage across all 30,913 Code/89,042 Data/11,133 Unknown bytes; pending frontier empty. Coherent current-source verification byte-identical (source 7f3dba06..., symbols b5abf2e0...). Final consistency/dependency/name screening remains active before fresh final verify/read and compact handoff; no final audit.

## Round 3 commentary complete

Current-source readability complete: all 14,873 records, 896 units, 1176 descriptive source labels; pending ranges empty. Fresh final byte-identical verification matches final source c13f72c09d0b026c8e515a4efa16da182a88642f7d1ad00baa9b8140d81fdc15 and symbols b5abf2e063ce09e8d7d8f5ae15f2744f1bc0728299da66d5b0a71d8a41eb158a; reading copy refreshed. Scope assessment commentary/scope-assessment.json and compact handoff analysis/handoffs/round03-commentary.md. Classification remains 91.51%, 11,133 Unknown; concrete structural frontier retained. Final editorial acceptance pending, no final audit. Source/evidence local, ROM/map/state/toolkit unchanged.

## Round 3 redirect active
Unique research directory analysis/redirect/r03-residual-contracts, frontier.md. Input round03-disassembly-closure map/state605b2a81e96a8a392a7f654cbd9b9fcbf3d009103e2d91b38c947f1e25b0ca0f,119955classified11133Unknown(91.51%). Fullcurrent-source readability complete. Revisiting precise static-entry versus actualcaller premises at52C5/C745, then connected D0C9/A882/queue and independent residual opportunities. No accepted map/source changes, no terminal assessment; no toolkit issues found so far.

Round3redirect checkpoint: primary threepairedshootoutdata52C5..52CB corrected priorentryambiguity; cohesive92BB..9376staticgraph under review. IndependentB002/B502fivepagegraphics1280each, packedcollision104bytes and33foldedangleentries found; narrower genericdata classification does not require originaltableextent. Exactcomplement+allabsoluteconsumers durable in analysis/redirect/r03-residual-contracts/unknown-ledger.json,absolute-references.json,consumer-review.txt. Frontier now includes A6DF10stateactorgraph (dummyindexedanimationbyte overlaps precedingJMP operand) and directEC72speechentry/C362wrapper localcontracts distinct from nestedC31F exclusion. Allfindingsprovisional; map/sourcefrozen, accepted119955classified11133Unknown. Continue samephase.

Round3redirect provisionalcombinedmanifest695starts1508UnknownCode+9039UnknownData=10547proposed(0accepted);586projectedresidual. Allproposeddirecttargetsaccepted/newstarts, noacceptedcontrolintoData;validation.json hashesfrozen. Connectedshootoutcallers, actor10state+D24C/D4E2graphs anddirectspeechselectedcontextscombined. Currentproductivefrontier explicitmusiccommandtargetsfrom832F..8359/876D/878D localhandlers andremaining shortcallercontinuations; localhandlercontractsseparatefromuniversalinterpreterreturn. Detailedfrontier.mdupdated; samephasecontinues.

## Round 3 redirect complete — round 4 disassembly pending
Combined8readynewIDs: R3-GRAPHICS-BOUNDS,R3-ARITHMETIC-ENTRIES,R3-ACTOR-PARITY,R3-SHOOTOUT-CLOSURE,R3-DIRECT-SPEECH,R3-SOUND-COMMANDS,R3-STATE-CONTINUATIONS,R3-RESIDUAL-DATA. Final951physicalstarts2081UnknownCode+9041UnknownData=11122proposedUnknown,0accepted. Manifest analysis/redirect/r03-residual-contracts/proposals.json;primaryfindings.md,independent-constraints.json,data-consumer-review.json,validation.json. Allproposeddirecttargets alignaccepted/newstarts;noacceptedcontrolintoData;knownpositiveED0BbranchpreservesdescriptorData. Eightleadrecipes/historyandphasepasspersistedinanalysis/leads.json.

ROMclassification remains91.51%(119955/131088,11133Unknown),checkpoint round03-disassembly-closure,map/state605b2a81e96a8a392a7f654cbd9b9fcbf3d009103e2d91b38c947f1e25b0ca0f;sourcec13f72c09d0b026c8e515a4efa16da182a88642f7d1ad00baa9b8140d81fdc15unchanged. Commentary896units1176labelscomplete/currentverificationreused;finaleditorialseparatepending. Proposedonlyprojection11Unknownafteracceptance:1C7F..1C81,80A5..80A7,11B4F..11B50,157FD..157FF,1D01F..1D023. Exactinputcomplement/proposalsplitsunknown-dispositions.json andprimarytinycandidatealternativesresidual-review.json. No terminalexhaustionclaim.

Correctedactualentry/fullselectordomainprerequisitesbyprimaryroleevidence;threepaired52C5parametersandcoherentrow/callergraph;registerarithmeticprefixes;A6DF10stategraphthroughD24C/D4E2chains;D0C9/CE64localC3F0terminalselector7nonclobber;directC34B/EC72speechselectedphysicalcontexts4/5,12/13,10/11distinctfromunsafeC31Fnested07E1save;explicit876D/878Dlocalmusicparameter/loophandlerswithoutuniversalinterpreterreturnclaim. Genericpatterneddataaftermeaningfulreader/boundaryinspection;strongindependent2x1280graphics,104collisionbytes,33foldedcomponents,6presentationtemplates constraints. Dummyindex0A788/A862/C36EoperandaliasesretainCode. Connectedandindependentconcreteavenuesclosed;nootherproductiveunknowncontinuationdeferred. Savedgameplayroutesreused/scenariosreviewappended;no newexecution orshortcutcontract. No toolkitissuesfound;ROM/map/source/toolkitunchanged,noarchive/deletion/cleanup/publication/sharing. Compacthandoff analysis/handoffs/round03-redirect.md. Completedresearchattemptimmutable.

## Round 4 disassembly active
Input checkpoint round03-disassembly-closure; accepted map/state605b2a81e96a8a392a7f654cbd9b9fcbf3d009103e2d91b38c947f1e25b0ca0f,119955classified11133Unknown. Eight ready leads under independent physical decode/context/consumer/conditional-contract review. No annotations accepted yet. Current frontier: direct speech selected bank views, known ED0B branch, local music/state contracts and all105 data boundaries; then changed-neighbor residual review and fresh validation/checkpoint. Commentary896units1176labels and historical attempts preserved.

## Round 4 — disassembly complete, classification incomplete

ROM classification: 99.99% (131077 / 131088 bytes classified; 11 bytes unknown).

All eight ready leads independently verified and accepted:951 physical starts/1027 views,2081 newCode+9041 newData=11122Unknown classified. All Static; no new execution/natural reachability credit. Every prior map region and commentary source unchanged.82 supported data labels added; remaining generic data carries meaningful role/boundary evidence without invented schemas. Primary review analysis/round04/review-notes.md; exact per-instruction/data review, API decodes,320 direct transfers and preservation checks in same directory. No accepted/proposed direct control enters newData. DummyA788/A862/C36E remainCode. Positive speech descriptorstep3 provesED0B branch; selected speech contexts4/5,12/13,10/11 and all local stack/NMI/channel/state/queue preconditions retained. No universal music return, nested speech safety or actual caller-domain claim.

map.validate passed; analysis/state.json saved and maps/current.json exported. Fresh checkpoint round04-disassembly-closure; wholeROMca65 verify ok=true,byte_identical=true. Verification analysis/round04-disassembly-closure-verification.json. Map/state SHA256c41bbd28e42a66f0f1a155af77e681aa13fa9d6776583a8fcd95c185f337f4ac. ROMSHA256unchanged. Exact counts/complement analysis/round04/counts.json.32994Code/98083Data including header16; all15768regions evidence-backed.

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 3752 | 4438 | 2 |
| 1 | 4087 | 4105 | 0 |
| 2 | 6777 | 1415 | 0 |
| 3 | 2450 | 5742 | 0 |
| 4 | 148 | 8042 | 2 |
| 5 | 25 | 8167 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 1967 | 6224 | 1 |
| 9 | 1452 | 6740 | 0 |
| 10 | 351 | 7839 | 2 |
| 11 | 0 | 8192 | 0 |
| 12 | 2295 | 5897 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 6688 | 1500 | 4 |
| 15 | 3002 | 5190 | 0 |

Exact Unknown half-open physical intervals:1C7F..1C81(2),80A5..80A7(2),11B4F..11B50(1),157FD..157FF(2),1D01F..1D023(4). Changed-neighbor continuation review found no new supported entry or consumer; tiny scalar/unofficial-prefix/unrooted-return alternatives remain materially ambiguous. Ready batch and connected easy gains closed; deeper independent-origin review belongs next redirect after full current-source commentary. No terminal exhaustion claim. Saved broad gameplay milestones reused; no changed credible shortcut/no unchanged replay, no execution coverage change.

Prior commentary896units1176labels/sourcec13f72c09d0b026c8e515a4efa16da182a88642f7d1ad00baa9b8140d81fdc15 preserved; newmap requires fresh full current-source commentary carry review. No commentary/redirect work performed; final editorial acceptance pending. No toolkit issues found; analysis/toolkit-issues.md retained. No ROM/toolkit edits, archive, cleanup/deletion, publication/sharing. Current phase: disassembly complete, commentary pending. Compact handoff analysis/handoffs/round04-disassembly.md.


## Round 4 commentary active

Current phase: commentary. Entire prior commentary preserved and hash-checked at analysis/round-4/commentary-round03-preserved; fresh supported preparation passed byte-identical. Current frozen map/state c41bbd28e42a66f0f1a155af77e681aa13fa9d6776583a8fcd95c185f337f4ac. Carried 14717 exact physical/view records, 896 prior units; 2105 Code bytes (2081 new plus 24 dependency bytes) and 9041 Data bytes pending. Exact machine frontier analysis/round-4/commentary/pending-ranges.json. Source explanations for new routines being applied; all new data and contextual names/dependency review remain. No handoff or final audit.


### Round 4 commentary saved scope

All 32994 Code and 98083 Data bytes explained/reused; exact pending-ranges inventory empty. Added 109 code/control-flow and 105 data units. Concrete affected old gap claims repaired; code batch byte-identical verification passed. Naming pass in progress: symbolic reader bases, physical data labels, contextual music/voice/side-upload RAM, actual transfer targets. Then check names, fresh final verification/read/scope hashes and handoff. Current phase remains commentary active; final editorial audit pending.

## Round 4 commentary complete

Current-source readability complete: 15773 reviewed records, 1111 meaningful units, 2616 descriptive labels; pending ranges empty. All 32994 Code/98083 Data and exact 11 Unknown covered. Final byte-identical verification matches source efdd2b36270ec5ef2b843183b088a677914045441e7bc794aa5e155c870aae99 and symbols e6282311a0951faf6485507a35354cb5b00ea293a3334667120ea4da48148d77; reading refreshed. Scope commentary/scope-assessment.json; handoff analysis/handoffs/round04-commentary.md. Map/state/ROM unchanged. Classification 99.99%, incomplete; next redirect independent residual-origin review. Final editorial acceptance separately pending, no audit.

## Round 4 redirect active

Attempt analysis/redirect/r04-residual-origin; frozen checkpoint round04-disassembly-closure, map/state c41bbd28e42a66f0f1a155af77e681aa13fa9d6776583a8fcd95c185f337f4ac. Current phase redirect; commentary complete. Frontier: independent combined-constraint residual review of 1C7F..1C81,80A5..80A7,11B4F..11B50,157FD..157FF,1D01F..1D023; primary-neighbor and origin inspection active. No map/source mutation.

## Round 4 redirect complete: ready cadence-mask proposal

Attempt r04-residual-origin corrects prior no-consumer rejection: physical1C1A LDA9C6E,Y reads Unknown1C7F/1C80 under guarded difficulty1/2; menu group1 bounds and same-selector neighboring masks independently anchor exact extent. R4-ACTOR-TARGET-CADENCE ready, Data2 proposed/0accepted, projectedUnknown9 only if accepted. Current accepted131077classified/11Unknown unchanged. Current2059absolute references/nearby indexed bases and residual neighbors reviewed; remaining CLC/RTS twins, isolated RTS and four scalar/opcode bytes lack structural discriminator, not a universal caller/schema prerequisite. findings.md records actual alternatives and primary evidence. Saved gameplay milestones reused, no changed shortcut/no replay. Next disassembly independent review then full current-source commentary and redirect; final editorial separatepending. No toolkit issues found; no ROM/map/source/toolkit changes, archive/cleanup/publication/sharing. Handoff analysis/handoffs/round04-redirect.md.

## Round 5 disassembly active

Input round04-disassembly-closure. Independent API decode and raw-byte/context review accepted R4-ACTOR-TARGET-CADENCE: two Static Data bytes 1C7F..1C81, preserving dummy-index-zero RTS and following Code. Prior regions and entire commentary preserved by exact comparison. Current classification 131079 / 131088; nine Unknown remain. Frontier: supported changed-neighbor closure and fresh validation/byte-identical checkpoint; no unchanged gameplay replay. Commentary/redirect remain pending.

## Round 5 disassembly complete, classification incomplete

ROM classification: 99.99% (131079 / 131088 bytes classified; 9 bytes unknown).

R4-ACTOR-TARGET-CADENCE independently verified; two Static Data bytes accepted, no new Code/execution credit. 32994 Code/98085 Data including header16;15769 regions. Review analysis/round05/review-notes.md, fresh API decodes/preservation/counts in same directory. Entire prior regions and commentary1111units2616labels preserved. Connected actor cadence/window graph already classified; no further changed-neighbor gain. Saved broad gameplay reused, no changed shortcut or unchanged replay.

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 3752 | 4440 | 0 |
| 1 | 4087 | 4105 | 0 |
| 2 | 6777 | 1415 | 0 |
| 3 | 2450 | 5742 | 0 |
| 4 | 148 | 8042 | 2 |
| 5 | 25 | 8167 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 1967 | 6224 | 1 |
| 9 | 1452 | 6740 | 0 |
| 10 | 351 | 7839 | 2 |
| 11 | 0 | 8192 | 0 |
| 12 | 2295 | 5897 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 6688 | 1500 | 4 |
| 15 | 3002 | 5190 | 0 |

Exact Unknown half-open physical intervals:80A5..80A7(2),11B4F..11B50(1),157FD..157FF(2),1D01F..1D023(4). Separate competing structural roles retained; no forced100 or terminal exhaustion claim.

map.validate/save/export passed; fresh checkpoint round05-disassembly-cadence verified ok=true,byte_identical=true. Map/state SHA256 52a6c8c849dfcc0feb583e4137794178228d17f42272b534a4adfe941bec430a. ROM unchanged. Verification analysis/round05-disassembly-cadence-verification.json. No toolkit issues found; analysis/toolkit-issues.md preserved. No ROM/toolkit/commentary edits, archive, deletion/cleanup, publication/sharing. Disassembly complete; full current-source commentary then redirect pending; final editorial acceptance separate. Compact handoff analysis/handoffs/round05-disassembly.md.

## Round 5 commentary

Current phase: commentary complete; next redirect. Prior full commentary preserved at analysis/round-5/commentary-round04-preserved before supported fresh preparation. Exact physical/kind/view carry: 15772 records, plus newly reviewed two-byte ActorTargetCheckCadenceMasks record; 1111 meaningful units, no pending readability ranges. Corrected guarded reader numeric CPU alias and stale Unknown/no-consumer prose. Fresh verification byte-identical; reading refreshed. Source scope and hashes: commentary/scope-assessment.json; compact handoff analysis/handoffs/round05-commentary.md. ROM/map/state unchanged. Classification remains 131079/131088 (nine Unknown); natural read remains unproved. Final editorial acceptance remains separately pending after redirect terminal review; no audit run.

## Round 5 redirect active

Unique attempt analysis/redirect/r05-terminal-constraints. Input round05-disassembly-cadence, map/state 52a6c8c849dfcc0feb583e4137794178228d17f42272b534a4adfe941bec430a; current-source commentary complete. Frontier: inspect decisive r04 primary combined-constraint reasoning for exact four residual intervals and assess other concrete avenues. Reuse unchanged sound evidence; no inherited universal caller requirement, no unchanged replay. Classification remains 131079/131088, nine Unknown. Frozen ROM/map/source/toolkit.

## Round 5 redirect complete for coordinator terminal review

TERMINAL INCOMPLETE proposed, not goal completion or final editorial acceptance. Unique attempt r05-terminal-constraints/rejection-review.md evaluates exact claims, primary evidence scope, independent constraints, strongest competitors and specific insufficiency/no useful revised tests; other-avenue assessment included. Sound r04 four-range assessment reused on unchanged premises; obsolete cadence rejection excluded. No new/updated leads; pass appended, all existing verified history preserved. Classification99.99% (131079/131088), nine Unknown; proposal/acceptance0/0. Independent exact-complement and frozen-input hashes saved. Saved gameplay milestones reused with actual prospect assessment, no unchanged replay. Current-source readability and fresh byte-identical verification retained. Frontier complete pending coordinator reasoning inspection; finaleditorial separate pending. Handoff analysis/handoffs/round05-redirect.md. Disk3.3GiB free; no cleanup/deletion/ROM/map/source/toolkit edits/archive/publication/sharing.

Coordinator personally inspected primary rejection review/listings/neighbors/audit and accepted terminal incomplete; no useful corrected inference/test. Redirect closed. Final editorial acceptance remains next separate gate; ROM remains nine bytes structurally incomplete.

## Final editorial acceptance in progress

Coordinator accepted terminal-incomplete structural rejection review; see analysis/coordinator-rejection-review.md. Final editorial worker preserved source/review/scope, repaired concrete stale comments and cadence branch semantics, and normalized review paths/unit attachments. Supported audit remains live in session 67986; wait for its terminal result and hash-matched verification/reading artifacts. No archive, cleanup or toolkit change. Structural classification stays 131079/131088, nine Unknown; orchestration goal remains active until this gate and final completion audit pass.

## Final editorial acceptance complete

Phase: final acceptance; input/output checkpoint round05-disassembly-cadence unchanged (map/state52a6c8c849dfcc0feb583e4137794178228d17f42272b534a4adfe941bec430a). Reused valid current-source review; targeted semantic checks corrected 45 comment lines across28 units, chiefly stale Unknown-continuation claims and cadence-test scope. No instruction/operand or map edit. Review metadata source paths and containing-unit attachments repaired to supported audit contract;1111 unique units,15750 attachments,15773records. Source snapshots preserved under analysis/final-acceptance/editorial-20261120/. No leads consumed/produced; no classification change.

Supported audit session67986 exited0: full inventory/editorial gate passed, fresh byte_identical=true, reading refreshed; actual final source hashes match verification. Scope commentary/scope-assessment.json; semantic findings commentary/editorial-audit.md; handoff analysis/handoffs/final-editorial-acceptance.md. Performance issue ROMATLAS-PERF-001 recorded without toolkit modification. Editorial acceptance/readability passed; structural classification remains99.99% (131079/131088), exact9Unknown. Coordinator final requirement review remains; no goal completion. No archive, cleanup/deletion, publication or sharing.

## Coordinator terminal handoff — orchestration complete, ROM incomplete

ROM classification: 99.99% (131079 / 131088 bytes classified; 9 bytes unknown). Exact residual half-open physical intervals:80A5..80A7,11B4F..11B50,157FD..157FF,1D01F..1D023. Final checkpoint round05-disassembly-cadence/map-state52a6c8c849dfcc0feb583e4137794178228d17f42272b534a4adfe941bec430a. Terminal-incomplete rejection review accepted after coordinator primary-artifact inspection; no ready/interrupted lead or useful revised method remains on unchanged premises. See analysis/coordinator-rejection-review.md.

Final editorial acceptance PASSED: supported audit session67986 exited0, fresh whole-ROMbyte-identical verification, refreshed reading. All15773regions/1111uniqueunits/15750attachments reviewed. Sourcefd7a81f718141a42796130941c25eba304364c1f00602f20bd702dbfe8aeb47c; symbols732513d1e188b9f7f4d16f3dad803d39b876671f453307f567a291f38caedb56; readingc67c2540fa6cb54d1ced06f05fd29d8d5638f82b0d9b21819630c6a669d5d203. Map/storevalidation and15726physicalview checks passed. All final hashes and frozen/current map correspondence verified.

Final handoff analysis/terminal-handoff.md; complete requirements audit analysis/coordinator-audit.json; exactverification analysis/coordinator-verification.json. archive-options.json and analysis/archive-selection-verification.json account for all61Markdownfiles, preservefinalconclusions/handoffs, omitonly12duplicateinstructioncopies. No archive execution or cleanup performed. One documented toolkit performance missingfeature ROMATLAS-PERF-001, no remaining blocker and no toolkit edit. All source/state/ROM/evidence stay local; no publication/sharing. Workflow completion does not mark ROM classification complete. Reopen structural research only for newindependentrole/boundary evidence or a justified concrete revised method, using retained handoffs.

## User-directed residual fallback — disassembly phase

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Exact preflight complement matched physical32933+2,72527+1,88061+2,118815+4 (nine bytes; cap ten). Four generic Data regions added through supported map.annotate with Manual provenance solely under explicit user authorization despite retained structural ambiguity. Proven structural bytes remain131079; manual fallback9 is distinct, not newly proven noncode. Prior competing roles and exact ranges preserved in analysis/user-fallback/decision.md and historical primary rejection review. Code32994/Data98094;15773regions. All15769priorregions and29commentaryfiles unchanged; ROM/toolkit unchanged.

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 3752 | 4440 | 0 |
| 1 | 4087 | 4105 | 0 |
| 2 | 6777 | 1415 | 0 |
| 3 | 2450 | 5742 | 0 |
| 4 | 148 | 8044 | 0 |
| 5 | 25 | 8167 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 1967 | 6225 | 0 |
| 9 | 1452 | 6740 | 0 |
| 10 | 351 | 7841 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 2295 | 5897 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 6688 | 1504 | 0 |
| 15 | 3002 | 5190 | 0 |

Counts/hashes/API record: analysis/user-fallback/. Supported validate/save/export passed. Fresh checkpoint user-directed-fallback-20261120 pending verification result below. Existing toolkit missing feature ROMATLAS-PERF-001 retained without blocker/new issue; analysis/toolkit-issues.md. No research/replay/archive/deletion/publication. Next phase full current-source commentary; then renewed final audit, neither claimed complete by this phase.

Fresh checkpoint user-directed-fallback-20261120 PASSED: ok=true,byte_identical=true; analysis/user-directed-fallback-20261120-verification.json; maps/user-directed-fallback-20261120.json; generated/user-directed-fallback-20261120. Map/stateSHA256 dca19f52fea5e9730bf21c9d8525b09dfbaab9cdd739c172092bfbae93873ff7. Disassembly fallback phase complete; next full current-source commentary, then renewed final audit.

## User-directed fallback commentary — complete current-source scope

Preserved the entire prior commentary before supported preparation at analysis/round-6/commentary-before-user-fallback. Fresh baseline matches user-directed-fallback-20261120 and map/state dca19f52fea5e9730bf21c9d8525b09dfbaab9cdd739c172092bfbae93873ff7. Compared every physical partition and phase1 dependency: 15769 unchanged records retain valid review, descriptive names, explanations, normalized source locations and containing-unit attachments. Four formerly Unknown records now reconcile to the exact current Manual Data regions; no unrelated region changed. All 15773 records and 1111 unique meaningful units retain reviewed status; no pending readability ranges.

Source ambiguity notes at physical $0080A5..$0080A7, $011B4F..$011B50, $0157FD..$0157FF and $01D01F..$01D023 now explain the explicit Manual exception without claiming non-code proof, a format, meaning or reachability. Source line count remains 101197; no instruction, operand, symbol value or asset changed. Historical uncertainty/editorial records remain preserved and are explicitly superseded for current classification/acceptance. Original sound/phrase/artwork names, generic schemas, some selector domains and natural caller prerequisites remain unresolved, with established mechanics explained in source. No new research or replay.

Fresh supported verify passed ok=true, byte_identical=true; reading copy refreshed. Scope/hashes: commentary/scope-assessment.json; carry comparison: analysis/round-6/commentary-carry-report.json. Readability complete; classification 131088/131088 (100%), comprising 131079 structurally supported bytes and nine ambiguous Manual fallback bytes. Renewed final editorial audit is pending and was not run. Existing ROMATLAS-PERF-001 missing feature has no blocker; no new toolkit issue or toolkit edit. No archive, cleanup, deletion, publication or sharing.

## User-directed fallback renewed final acceptance

Supported strict final audit session 32596 PASSED (exit 0, ok/byte_identical/editorial_audit true); fresh verification and reading refresh passed. All 15773 regions/1111 distinct meaningful units/15750 attachments reviewed, no readability gaps. Reused prior sound whole-source review, checked four changed Manual fallback notes and dependencies; prior editorial history preserved. Classification remains 100% annotated, distinguishing 131079 structurally supported bytes from nine structurally ambiguous user-authorized Manual Data bytes. Exact evidence/hashes and interrupted-precondition invocation record: analysis/handoffs/user-directed-fallback-final-acceptance.md; commentary/editorial-audit.md. Existing ROMATLAS-PERF-001 is nonblocking; no new issue or toolkit edit. No research/replay/archive/deletion/publication.

## Coordinator final handoff after authorized nine-byte fallback

ROM classification: **100.00% (131088 / 131088 bytes classified; 0 bytes unknown)**. Separate accounting:131079 structurally supported bytes plus9 structurally ambiguous user-directed Manual generic Data bytes of unknown meaning. Exact complement was verified before annotation at physical offsets32933 length2,72527 length1,88061 length2,118815 length4. Explicit user exception, prior ambiguity and supported API record: analysis/user-fallback/decision.md. This supersedes earlier terminal-incomplete coverage only under that exception; no new empirical proof or invented semantics. Historical evidence retained.

Separate sequential disassembly/commentary/final-acceptance workers completed. All15769 prior regions preserved; all15773 current review records reconcile,1111distinct units/15750attachments, no pending ranges. Renewed final semantic review and supported strict audit32596 PASSED exit0, fresh byte-identical rebuild and reading refresh. Coordinator independently validated exact physical coverage, metadata/source paths, fullCode unit coverage, ROM identity and current source/report hashes. Configured store/map validation and fresh checkpoint user-directed-fallback-20261120 passed. Current map/state/frozen/checkpoint SHA256 dca19f52fea5e9730bf21c9d8525b09dfbaab9cdd739c172092bfbae93873ff7.

Final conclusions: analysis/user-fallback/final-handoff.md. Requirement audit: analysis/user-fallback/coordinator-audit.json. Fresh hashes: analysis/user-fallback/coordinator-verification.json. archive-options.json accounts for all72Markdown files under inspected archive.py contract,47additional selected final/history documents and15duplicateinstruction omissions; no unrecognized Markdown, final fallback decision selected. No archive execution/deletion/cleanup/publication/toolkit edit. No remaining blocker; existing ROMATLAS-PERF-001 nonblocking. All ROM/source/evidence remain local. No further redirect research undertaken. Workflow complete under explicit exception, not a claim of complete evidence-backed structural proof.
