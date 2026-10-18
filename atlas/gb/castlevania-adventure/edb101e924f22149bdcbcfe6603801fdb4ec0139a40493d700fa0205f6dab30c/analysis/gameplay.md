# Gameplay research

No research performed yet. Follow workflow.md's Game familiarization section. Record source URLs, version uncertainty, actionable routes and local confirmation separately.

## Round 1 targeted familiarization (2026-10-17)
ROM title bytes are CASTLEVANIA ADVE; title screen confirms The Castlevania Adventure, TM and ©1989 Konami. Header destination byte 01 suggests overseas cartridge; exact commercial revision is not independently established. Platform: DMG Game Boy, MBC1, four 16-KiB banks.

Sources consulted for immediate stage-1 input hypotheses:
- https://strategywiki.org/wiki/Castlevania%3A_The_Adventure/Gameplay — A jumps, B whips, Up/Down climb ropes.
- https://gamefaqs.gamespot.com/gameboy/585660-castlevania-the-adventure/faqs/6614 — jump/whip timing and stage-1 secret-room route.
- https://castlevania.fandom.com/wiki/Walkthrough%3AThe_Adventure/Stage_1 — first rope after opening enemies, falling platforms later; no subweapons.
Search excerpts consulted, not full walkthroughs; applicability to this exact revision remains a hypothesis. These external statements are not ROM classification evidence.

Local confirmation: natural startup displays Konami logo, then title; Start on logo advances to title, another accepted Start begins the game transition. Holds and releases plus operation/frame budgets are preserved in analysis/runtime/rpc.jsonl and consolidated into analysis/scenarios.json at handoff. Initial gameplay experiment holds Right to expose player, enemies, and scrolling code. More precise jump/rope route remains under test.

Confirmed first-field route: import field753.snapshot.json (naturally reached from startup), explicitly set Right=true, pulse B 8/20 frames and A during every third segment to reach combat953; then import combat953, explicitly release Left/Up/Down/A/B/Start/Select and press Right, run12*20 frames to reach walkfixed1193. Local screenshot confirms scrolling, score50 and opening enemy adjacent to player. Importing a snapshot alone does NOT preserve desired held inputs with this toolkit build (T001); always reassert them. walk1193 without explicit Right did not move and is superseded. Next targeted hypothesis is opening enemy combat/damage/item paths via Right plus jump/whip pulses. No later level or rope has been reached yet.

## Round 4 static verification (2026-10-17)
No new gameplay research or runtime route. Bounded first-substage layout/threshold proof used local consumer evidence only; existing sourced controls/routes and T001 input workaround retained. No later-stage reachability claimed.

## Round 5 static verification (2026-10-17)
No new research, runtime or input route. Exact first-layout tile-index records verified using local consumer/bank evidence. Prior sourced routes and T001 workaround preserved; no new natural stage reachability claim.

## Round 5 resumed redirect (2026-10-17)
No new external gameplay research or natural route. Fourteen artificial stage/nested samples invoked0968 then262f under explicit bank1 and safe RAM return sentinel; this verifies local descriptor behavior only. It does not prove later stages, death/respawn routing, coordinate gate success or record selector reachability. Exact setup and bounded replay are in analysis/redirect/round-005-resumed-probe.py and analysis/redirect/round-005-resumed-api.json. No input or snapshot used; prior natural routes and T001 workaround preserved.

### Round 006 static structural pass
No new gameplay route or external claim added. Existing sourced controls/routes and tested scenarios retained. Respawn/transition record structure established statically for bounded descriptor cases; prior constructed function samples do not establish natural stage/substage reachability. Further natural progression remains a separate experiment if a useful route emerges.

## Round 006 redirect — artificial descriptor tests
No new natural route or external gameplay research. Nineteen stage/selector262f samples from verified transition fifth fields checked nested/width under constructed state only; this does not prove successful1e98 gates or natural selector reachability. [Replay](redirect/round-006/research.py), [cases](redirect/round-006/evidence.json), [RPC](redirect/round-006/rpc.json). Clean snapshot is session-local; explicitly release all8buttons after restoration(T001). Existing sourced controls/routes and natural input records preserved.

## Round 007 static structural pass
No new external research, natural input route or runtime test. Nineteen transition-sourced descriptor cases and exact downstream records verified statically; conditional list interpretation does not establish ordinary1e98 gate success or later-stage reachability. Prior sourced controls, tested routes and T001 workaround preserved. Evidence: [round007 verification](round-007evidence/verified.json).

## Round007 redirect — constructed tests only
No external claim or new natural input route.26 artificial descriptor calls and4 artificial continue-wrapper calls verify local behavior only; no ordinary gate success/later-stage reachability inferred. [Handoff and replay recipes](round-007-redirect-handoff.md). All8buttons explicitly released after each clean session-local snapshot restoration(T001).


## Round008 disassembly — artificial continue tests
No new external research or natural route. Four independent constructed calls PC06f1/0702 with c00d0/4/8,c6010 and safeRAMc100 return verify continue prompt initialization/input behavior only. Returned988/19/279/278operations, cap10000; fixed0/switch1 restored. All8buttons explicitly released after each clean session-local restore(T001). [Replay](round-008evidence/raw-probe.py), [results](round-008evidence/raw-probe-tests.json), scenarios.json. Stage/descriptor data proof remains static; no natural gates/later-stage reachability claimed. Existing sourced controls/routes preserved.

## Round008 redirect — bounded artificial loaders
No external research or new natural route.33descriptor calls and7selected audio commands verified exact local loader behavior with explicit all8button releases after clean session-local restores(T001). Exact patches,budgets and results in [redirect evidence](redirect/round-008/evidence.json), [audio evidence](redirect/round-008/audio-evidence.json), scripts and scenarios.json. No ordinary transition gates,later-stage reachability or natural sound-command execution inferred.


## Round009 static structural pass
No external research, runtime or natural input route added. Exact transition-derived layouts/lists and selected audio command headers verified statically; no natural gate success, later-stage reachability or sound-command execution inferred. Existing sourced routes/scenarios and T001 workaround preserved. [Independent proof](round-009evidence/verified.json).

## Round009 redirect — artificial consumer tests only

No new external research or natural route. Three artificial262f descriptor calls and31directaudio event-parser calls corroborate localconsumer behavior;no ordinarygate/laterstage/sound-commandexecution claimed. Clean snapshots session-local;all8buttons explicitlyreleased afterrestoration(T001). Exactpatches,budgets andresults in[redirectevidence](redirect/round-009/evidence.json),[replay](redirect/round-009/research.py),scenarios.json. Staticgrammarbounds selectedfiniteaudio streams;command6repetitions not replayed.

## Round 010 disassembly — static consumer verification

No new external research, natural input route or emulator execution. Independently verified transition-derived descriptor contexts and four finite audio streams using exact consumers; artificial tests from the preceding redirect remain historical and do not prove natural gates, later-stage reachability or natural audio execution. No snapshots used. Existing sourced controls/routes and T001 workaround are preserved. [Evidence](round-010evidence/verified.json), [handoff](round-010-disassembly-handoff.md).

## Round010 redirect — artificial consumers
No external research or natural route added. Two262f loader calls and43command49 parser eventcalls verify only local descriptor/grammar behavior. All8buttons explicitly released after session-local clean restoration(T001). Exact patches,budgets,bankcontexts and outcomes in[findings](redirect/round-010/findings.md),[replay](redirect/round-010/research.py),scenarios.json. No natural gates,laterstage or audio execution claim.

## Round011 — bounded artificial EA consumers
No new external research or natural route. Four42a5 calls with parameters16/21/2a/20 confirm exact nested dispatch/word reads, including EA20 eight-bit index wrap.22/31/31/32operations,cap100,stop41a2;clean session-local restores and explicit eight-button releases(T001). No natural audio/later-stage/gate reachability. [Replay and read proof](round-011evidence/word-read-verification.json). Existing sourced routes retained.

## Round011 redirect — artificial structural consumers
No new external research or natural route. One selector66 loader call and seven isolated FB/FC/FD/FE controls use explicit bank/state patches, clean session-local snapshots and all8button releases(T001). Exact caps/patches/results in [research](redirect/round-011/research.py) and [evidence](redirect/round-011/evidence.json). Static cyclic music graphs establish byte roles only; no natural audio execution or finite-duration claim.

## Round012 — artificial parser controls and static music graphs
No new external research or natural input route. Seven isolated parser controls replayed with clean session-local restores, explicit eight-button releases, bank2,c106/c11b..c11e patches,PC41a3/DEevent/SPdff0,cap100. Actual23/32/30/31/31/23/32operations match exact expected slots/counter includingFF-to00wrap. Full source-command40/48 graphs remain static structural cycles, with no natural playback or finite-duration claim. Existing sourced controls/routes and historical snapshots retained. [Tests](round-012evidence/control-tests.json),[replay](round-012evidence/verify.py).

## Round012 redirect — source-selected audio only
No new external research or natural route. Existing sourced gameplay routes and T001 input workaround retained. Exact additional stage music graphs are structural cycles, not playback/reachability evidence; stage05bc domain is explicitly conditional0..3, whereas0a1e has local&3. Constructed header/priority/EA tests use bank2,clean session-local restores and explicitall8button releases. [Research](redirect/round-012/findings.md), [replay](redirect/round-012/research.py).

## Round013 — static source-selected audio verification
No external research, natural route, runtime or input sequence added. Existing sourced gameplay routes and T001 workaround are preserved. Stage05bc music selection is conditional on0..3 because05aa has no local mask;0a0e independently masks&3. Sparse stage graphs prove structural cycles only. Prior artificial loader/priority/EA tests remain historical local-consumer evidence, not natural reachability or finite playback. [Independent proof](round-013evidence/verified.json), scenarios.json.

## Round013 redirect — constructed object consumers
No new natural input route or external research.40 bank1 directory calls,7 isolated spawn and5 terminal-scan calls corroborate conditional source contexts and placement schema only. Clean session-local snapshots explicitly release all8buttons(T001); exact registers/RAM/caps and equality-negative retry history in[findings](redirect/round-013/findings.md),[replay](redirect/round-013/probe.py),[tests](redirect/round-013/tests.json). No natural object selection/later-stage/gameplay claim.

## Round014 — static object placement verification
No new external research, natural route, emulator or snapshot used. Existing sourced controls/routes and T001 workaround retained. Forty descriptor-backed object contexts establish conditional structure only; coordinates0070,0038 in stage1/nested12 are nonmonotonic and do not establish natural second-row selection. Historical constructed consumers use coordinate+1; equality does not spawn. E000 is a comparison boundary only for nonnegative DE<2000. [Proof](round-014evidence/verified.json), scenarios.json; original artificial replay remains linked without rerun.

## Round014 redirect — artificial persistent-placement consumers
No external research or new natural route. Forty bank2 directory calls, six isolated persistent spawns and five unsigned-coordinate scans confirm local consumer behavior only. All8buttons explicitly released after clean session-local restoration(T001); bank2, exact source cases/patches/budgets/retries in [findings](redirect/round-014/findings.md), [replay](redirect/round-014/research.py), scenarios.json. No natural later-stage/placement selection implied; bank2 list1/15 is nonmonotonic. Prior sourced routes retained.

## Round015 — conditional persistent placements
Static-only verification; no new external research, natural route, emulator or snapshot. Existing sourced gameplay and T001 workaround preserved. Forty accepted descriptor contexts establish conditional bank2 structure, not natural selection. Stage1/nested15 coordinates0088,0058 are nonmonotonic. Four-slot c580 runtime type2 retains original type and CBxx persistence index; no global enemy/type domain claim. Historical artificial routes/budgets/setup retries remain in redirect/round-014 and scenarios.json. [Proof](round-015evidence/verified.json).

## Round015 redirect — constructed finite object and renderer consumers

No external research or new natural route.117 final bounded tests(51 dispatch/conversion/initializer,44 rendering,22 initial-state wrappers) verify local finite field/dispatch/sprite contracts only. Snapshots session-local; explicit eight-button releases after each restore(T001). Renderer tests disableLCD and use controlled positions/timers; no ordinary animation/later-stage timing or selection claim. Captured wrapper instructions were executed, reached firststate bodies were not. [Exact recipes](redirect/round-015/findings.md),[verification](redirect/round-015/verification.json),scenarios.json. Existing sourced gameplay/routes remain preserved.

## Round016 — finite dispatch and sprite structures
No new external research, natural route, emulator execution or snapshot. Existing sourced controls/routes and T001 workaround retained. Independent static grammar and historical constructed trace audit establish finite local roles only. First-state wrapper prefixes were fetched; selected target bodies were not. Converted persistent+1b and2f7e selector+9 remain distinct; no ordinary state selection, animation timing or later-stage reachability claim. [Proof](round-016evidence/verified.json), [historical recipes](redirect/round-015/findings.md).

## Round016 redirect — finite constructed first-state calls
No external research or new natural input route.18 correctly slotted artificial state0 calls return state1; actual fetched first-state bodies and local initialized fields only.52 isolated renderer calls verify26node grammar/OAM/pointers, not ordinary timing. Type3 temporarily calls bank2 audio then restoresbank1. All8buttons explicitly released after each clean session-local restore(T001); no persistent named snapshot assumed. [Exact actor/player/stack/bank patches, limits and recipes](redirect/round-016/findings.md),[replay](redirect/round-016/probe.py). Existing sourced gameplay/routes preserved; later state bodies/natural selection remain unproved.

## Round017 — finite historical first-state audit
No new external research, natural route, emulator execution or snapshots. Existing sourced controls/routes and T001 workaround retained. Independently audited18 historical constructed first-state calls and52 renderer results, not natural selection or state1-body execution. Exact artificial inputs/patches remain in redirect/round-016/probe.py and consumer.py; [proof](round-017evidence/verified.json).

## Round017 redirect — finite parent/child state consumers

No external research or new natural input route.56 constructed actor/parser/child/bounds calls and26 isolated renderer calls confirm source-selected local contracts only. Types25/33/45/46 use actual slot/frame structures; all8buttons explicitly released after every restore/import(T001). Exact bounded patches and persistent local init snapshots: [findings](redirect/round-017/findings.md),probe.py/parsers.py/children.py/bounds.py/consumer.py there. Natural scheduling, live allocator pressure, later scene/animation timing remain unproved. Existing sourced routes preserved.034d exchange helper has no RNG role in this parser; type33 delay copies row count directly.

## Round018 — historical connected-consumer audit
No new external research, natural route, emulator execution or snapshots. Reused56 artificial actor/26 renderer cases with original explicit eight-button releases(T001); exact finite input/patch/budget recipes remain in redirect/round-017. Natural timing/selection remains unproved. [Independent audit](round-018evidence/verified.json).


## Round018 commentary — finite semantics only
No external research, new gameplay route, input sequence, runtime or snapshot was
used. Existing sourced controls/routes and T001 workaround are preserved. Parent/
child names describe observed mechanics rather than enemy/artwork identities;
constructed calls do not prove natural scheduling or visibility. Existing saved
child37 consumer reads also cover speed1/2/3 words, contradicting prior prose;
[exact correction](round-018/speed-step-followup.json). Structural map unchanged.

## Round018 redirect — finite constructed companion/spawn consumers
No external research/new natural route.7constructed actor calls follow26/27/31
state1 and produced type7 states;20renderer cases inspect animation0/28;16audio
calls verify sound26 header and finite stream toFF. These establish local contracts,
not normal ownership/scheduling/stage/audio reachability. All restores explicitly
release8buttons(T001); existing natural routes remain unchanged. Exact patches,
operation budgets and inputs:[findings](redirect/round-018/findings.md),
[replay](redirect/round-018/probe.py),scenarios.json.

## Round019 — independent finite historical audit
No external research,new natural route,runtime or snapshot. Reused7actor/20renderer/16audio calls from redirect018 and historical child speed Reads. Exact constructed patches/budgets remain redirect/round-018/findings.md and scripts; eight-button releases retained(T001). Natural timing,selection,ownership and later-stage reachability remain unproved. [Proof](round-019evidence/verified.json).


## Round019 commentary — local contracts only
No new external research,natural route,runtime,input or snapshot. Historical finite
companion/spawn/type7/animation28/audio26 cases retain constructed-state limits;
source names describe mechanics, not natural ownership or artwork identities.
Existing sourced routes and explicit eight-button-release T001 workaround preserved.
[Commentary handoff](round-019-commentary-handoff.md).

## Round019 redirect — artificial later-state local contracts
No new external research or natural route.34constructed actor calls test accepted
types6/8 later states with explicit field/timer/FF-marker/512byte packed class cache
00/55 patches;22renderer and6audio calls check literal assets. These establish
local transition/asset mechanics,not natural enemy identity,timing,placement or
ordinary contact/audio reachability. Persistent snapshot import followed by explicit
all8button release(T001);exact inputs/patches/budgets preserved in scenarios.json
and[findings](redirect/round-019/findings.md). Prior sourced natural routes unchanged.

## Round020 commentary — finite mechanics retained
No external research, natural route, runtime, input or snapshot added. Existing
sourced routes and T001 explicit eight-button-release workaround preserved. Type6/
type8 names explain local terrain/marker/motion contracts without enemy/artwork or
ordinary timing claims. Exact instructions correct historical prose: type8 contact
leaves+07 untouched while writing+08/+09=0200. Animation36 timer5 and42 timer7 are
renderer counts, not calibrated time. [Commentary](round-020-commentary-handoff.md).

## Round020 redirect — constructed local state graphs (research checkpoint)
No external research/new natural route.60 constructed actor calls,152 renderer
cases and117 audio calls test accepted later states/direct converters, produced
9/12/15/17/18 and finite assets. All8buttons explicitly released after every
snapshot import/load(T001). Actor max1585/cap1800; renderermax475/cap1000;
audiomax465/cap1000. Exact persistent snapshot, inputs, RAM/register patches and
contexts in scenarios.json and [findings](redirect/round-020/findings.md), scripts
there. Local state/slot/contact/audio contracts do not prove natural timing,
ownership, visibility or later-stage reachability. Existing sourced controls and
natural routes retained. Redirect is still in progress at a same-phase checkpoint.

## Round020 redirect continuation — artificial mechanics only
No external research or natural late-stage/ending route added.36constructed alternate
setup/update/actor calls,198additional renderer cases,11parameter calls(excluding
operand-cursor5) and1EE call preserve exact patches/budgets in redirect/round-020.
All8buttons explicitly released after every snapshot import/load(T001). Static
stage-clear/ending/death graphs do not demonstrate ordinary gameplay reachability.
Persistent state2 CP7/conversion proves normalized contact0..6 as a local source
contract; no natural pickup/alternate-mode entry claimed. Final table audit corrects
historical direct test names:5d7d/5e86 are runtime18/19, while type16 children are
runtime17->61ac; that child's graph is the next same-phase research frontier.
See scenarios.json, findings.md and continuation-checkpoint.json.

## Round020 redirect runtime17/player/window continuation
No new external research or natural route.10runtime17,14fixedwindow/terrain,
12playerstate/aux/shape and1ending-animation-wrapper constructed calls pass
(max1434/cap1800);76new renderer cases bring supplemental total274(max905/cap1000).
All8buttons explicitly released after every import/restore(T001). RAM input-latch
patches are debugger states, not natural button routes. Exact patches/caps/results:
scenarios.json round020_redirect_type17_player and redirect/round-020 scripts.
No natural climbing/transition/death/contact/late-stage/ending/ownership claim.
New source placement38 stage2/nested0->C4051->nested1 is a static conditional research
frontier; no runtime or natural route yet. Existing sourced routes retained.

## Round020 redirect — type38 transition context closure
No external research/new natural route.16constructed calls verify conditional
stage2/nested0->1 increment, transition row seed47/decrement,10first/last-row tile
pointer consumers, phase2 placement and empty persistent lookup. All8buttons
explicitly released after each persisted round017/init25 snapshot import(T001);
cap1800,max489,zero trace drops. Exact patches/inputs/results:
[recipe](redirect/round-020/nested1.py),[tests](redirect/round-020/nested1-tests.json).
Transition has48rows*5cells independently of width53; no ordinary nested1scrolling
or natural late-stage/proximity/visibility claim. Existing sourced routes retained.

## Round021 — historical connected mechanics audit
No new external research, natural input route, emulator execution or snapshots.
Reused redirect020 finite actor/alternate/player/window/contact/ending/asset and
nested1 transition evidence with original explicit8button releases(T001). Conditional
major0..47/cross0..4 bounds do not establish ordinary nested1 scrolling or natural
late-stage/ending/visibility. Historical direct names17/18 actually mean runtime18/19;
actualruntime17->61ac is separately closed. Existing sourced routes and failed
route history remain unchanged. [Proof](round-021evidence/verified.json),scenarios.json.


## Round021 commentary partial checkpoint
No new research,natural route,runtime,input or snapshot. Prior sourced gameplay
and constructed-case/T001 limits retained. Runtime17 parent-window and nested1
transition mechanics do not prove ordinary ownership,visibility or later progression.
Literal nonlocal-return correction recorded separately without gameplay claims.

### Round021 commentary continuation — ending input correction
No research/runtime/natural route added. Literal final bank2 ending handler CPU6dff tests c00c AND10: held A(bit4), not Start as older redirect prose claimed. This is static input-mask interpretation, not a natural ending test. Existing constructed-case and T001 limitations remain.

## Round021 redirect — local input/contact mechanics
No new externalresearch/naturalroute. Two artificial climbing-A calls use sampled
heldC418 and pressedC41A; rawC00C/C00D alone failed and that negative log is retained.
CorrectedCALL1a54 enters AFTERPOP1a6e,returns into the jump/motion tail,whose finalPOP
skips the immediateinputcheckcaller. Nine artificial overlaps preserve actual prior
initializer occupancy/type/shape and align only coordinates; incomingcontactrequires
shape bit5,auxiliarycontact bit6. Results are conditional mechanics,not natural
climbing/damage/visibility. Exact eleven recipes/results and8buttonrelease workaround
in scenarios.json and redirect/round-021/. Existing source/route uncertainties retained.

## Round022 — reused local input/contact evidence
No new external research,natural route,runtime,input or snapshots. Independently
audited eleven saved redirect021 calls with original exact sampledC418/C41A inputs,
source-compatible contactfields,bank/stack patches/caps and explicit8buttonreleases
after imports(T001);no unchanged failed naturalroute replay. Constructed215/494operation
results establish local mechanics only,not natural climbing/contact/timing/visibility.
Existing sourced controls/routes and negative rawC00C/C00D probe preserved.
[Independent proof](round-022evidence/audit.json),scenarios.json,redirect/round-021/.

## Round022 commentary — static source explanations only
No new research, route, runtime, input or snapshot. Preserved prior source and
conditional constructed-case limits. Climbing-A's actual stack contract is now
explained through ordinary CALL1a54 and final caller-discard POP1a6c. Sprite payload
second byte05 is ignored, not an actor-shape writer. Incoming contact uses
post-conversion runtime7/15/17. No new natural gameplay/timing/visibility claim.

## Round022 redirect — no changed gameplay prerequisite
No new external research, natural route, runtime, input or snapshot. Fresh current
source reassessment found no materially improved route to unresolved selector/
producer contexts; prior failed/unconfirmed natural routes are preserved without
unchanged replay. Historical constructed climbing/contact tests remain conditional
local mechanics, not timing/visibility/natural reachability. Existing sourced controls
and T001 explicit all8button reassertion after restore remain. [Findings](redirect/round-022/findings.md).

## Final investigation FI022-20261018-v1 — static structures only
No external research, natural route, emulator, snapshots, debugger patches or input
sequence added. Existing sourced controls/routes and failed-route history preserved.
The new isolated serial/palette/arithmetic and closed animation29/37 structural proofs
make no active-feature, natural-selection or timing claim. Future snapshot restores
still require all8inputreassertions(T001). [Findings](final-investigation/findings.md).

Final investigation same-phase revision adds static transition/descriptor/class-child,
noise-header and pitch-loop objects only. The preliminary311scope was superseded by
969proposalbytes; no new natural selector, route, input, snapshot or timing claim.
[Final handoff](final-investigation/handoff.md) retains all consumer-domain caveats.

## Round023 disassembly
No new runtime/input route or external research. Exact969 static bytes accepted from independently reviewed FI022 contracts; no later-stage, animation29/37 or sound4/25 natural selection inferred. Prior sourced routes and all8input reassertion T001 workaround retained. [Evidence](round-023evidence/audit.json).

### Round023 commentary
No external research, runtime, input route or new natural selection claim. Static
source explanations preserve animation29/37,sound4/25,class-parent34/35 and descriptor
gate reachability uncertainty. Existing sourced routes and T001 workaround retained.

## Round023 redirect — static reassessment only

No external research, new gameplay claim, emulator/input/snapshot call or natural
route replay. New transition28-context, class-parent34/35 and noise4/25 objects have
static structural contracts; natural scheduling remains unproved. Serial flag loop
and pitch inheritedcount255 cannot supply a natural experiment route. Prior sourced
controls, saved scenarios and all8button-reassertion workaround remain unchanged.
[Findings](redirect/round-023/findings.md).

## FI023 residual static credit proof
No new external research, runtime, inputs or natural route. Static ending state6 recurrence establishes conditional index8 rendering before the phase17 exit, and state9 later erases it. This is producer/consumer structural evidence, not natural ending reachability or measured gameplay timing. Exact proof in final-investigation/FI023-20261018-residual-v2/ending-proof.json; existing sourced routes and T001 eight-button restoration workaround preserved.

## Round024 disassembly — static credit extent only
No new research, runtime/input/snapshot or natural route. Independently reused
FI023 producer/consumer contracts: state6 phase1 draws index8 before phase17 exit;
state9 phase36 later erases it. Exact count3 tiles9/2/6 are structural evidence,
not natural ending reachability or measured timing. Existing sourced routes and
T001 all8button reassertion workaround retained. [Audit](round-024evidence/audit.json).

## Round024 commentary — conditional credit mechanics

No research/runtime/input/snapshot or natural route added. Static source explains
state6 phase1 drawing index8 before phase17 exit and state9 phase36 later erasing it;
count3 runs9/2/6 and exactad35 boundary do not prove natural ending reachability or
artwork identity. Existing sourced routes and T001 input reassertion workaround
remain. [Assessment](round-024/readability-assessment.md).

## Round024 redirect — source-only residual triage
No external research,new natural route,runtime,input or snapshot. Ninth credit
source closes at addressed VRAM writes; source order retains conditional state6
index8 draw/state9 erase, not natural ending reachability or artwork meaning.
Existing sourced routes and T001 all8button reassertion workaround preserved;
unchanged failed natural routes were not replayed.
[Findings](redirect/round-024/findings.md).

## FI024-20261018-contract-domains-v1 — conditional screen-major structure
No research,runtime,input,snapshot or naturalroute added. Static ordinary nested1 structural contract uses265 sparse cells in275byte envelope;known transition still increments nested1 to2 before ordinary queue. No naturalordinarynested1/visibility/ending timing claim. Existing sourcedroutes and T001all8buttonrestoration workaround preserved. [Proof](final-investigation/FI024-20261018-contract-domains-v1/screen-major-layout-proof.json).

## Round025 disassembly — conditional static scrolling structure
No external research,runtime,input,snapshot or naturalroute. Ordinarynested1 width53
and265sparse screen-major cells are statically supported conditional data; known
transition still incrementsnested1to2 before ordinary work. No natural selection,
visibility or timing claim. Existing sourcedroutes and T001all8button restoration
workaround retained. [Proof](round-025evidence/audit.json).

## Round025 redirect — conditional source geometry only
No new research,naturalroute,runtime,input or snapshot. Corrected sparse addresses
select no Unknown source cell across45coindexed rooted layouts;this is a conditional
structural constraint,not natural ordinarynested1 entry or visibility proof. Known
transition increments1to2. Existing sourced routes,failedroute history and T001
all8buttonreassertion workaround retained. [Findings](redirect/round-025/findings.md).

## FI025 — terminal static-method assessment
No new research, natural route, runtime, input or snapshot. Evaluated structural
alternatives do not establish natural selection, timing or unused/dead status.
Existing sourced routes, failed-route history and T001 explicit all8button
reassertion after restore remain unchanged. [Assessment](final-investigation/FI025-20261018-exhaustion-v1/method-outcomes.md).
