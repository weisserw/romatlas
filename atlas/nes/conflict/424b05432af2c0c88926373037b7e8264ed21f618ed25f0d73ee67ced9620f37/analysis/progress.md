# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 disassembly — in progress
Baseline established and verified byte-identical in analysis/baseline-verification.json.
Configured release binary and unchanged input SHA-256 confirmed. No classification yet.
Persistent server launched through file bridge (exec session 61314); requests and responses
in analysis/scratch are bounded to one file each. Unix sockets unavailable in sandbox;
this is an environment limitation. Current frontier: inspect vectors, identify startup/title,
then bounded runtime/static discovery. Disk 3.5 GiB free; no cleanup performed.
Reset vector $C000 and NMI/IRQ $C01B map to physical final 16 KiB PRG bank
(context [0,1,14,15]). Initial context-aware traversal found 346 instruction candidates;
these remain hypotheses pending review. Startup trace requested 180 frames but timed
out at 32; partial execution preserved, 435 unique instructions, aggregate drops zero.
Detail capacity zero intentionally discarded 1,509,630 events. Includes PRG-RAM $6000
execution; no ROM upload-source claim. Subsequent bounded 30-frame steps reached frame
122 with PRG context [4,5,14,15]. Initial frame at 32 black; identification pending.

### First structural batch checkpoint
Saved and validated analysis/state.json and maps/current.json; verified checkpoint
pass-001 at analysis/pass-001-verification.json (byte-identical). 970 code bytes,
131094 data/image/header/vector bytes, 130096 unknown of 262160 physical bytes.
ROM classification: 50.38% (132064 / 262160 bytes classified; 130096 bytes unknown).
416 observed physical ROM opcode roots and 59 additional static instruction starts;
PRG-RAM roots retained separately, never mapped as uploaded ROM. CHR classified from
hardware-defined planar-pattern storage, not CPU reachability. Full tile usage unknown.
Frontier snapshot frontier2.snapshot at frame342, natural state/context [4,5,14,15].
Further advancement shows partial title logo; game identity provisionally Conflict.
Read-only manual search found matching tactical controls, but full PDF fetch exceeds
web service size; external claims remain hypotheses. Start/A menu batch underway.
Server session 61314 exited143; discarded session-local state was replayed explicitly.
Subsequent self-contained batches export snapshots before process exit.
### Title and corrected input frontier
Frame1019 screenshot title-finish.png confirms Conflict ©1989 Vic Tokai Inc,
licensed Nintendo of America. Natural title-finish.snapshot retained. First Start/A
attempts used ignored generic button/pressed schema, and are corrected in scenarios;
NES requires buttons array and port (NES-DOC-001 in toolkit-issues.md).
Corrected Start2/release20 reaches1041 but remains title; a longer deliberate hold
is the next test. No RAM patches yet. Safe indexed RAM-write continuation/direct
JSR callee entry static extension added634 code bytes beyond first checkpoint;
validated current state now1604 code bytes. Return bank preservation remains unknown.
### Menu/static batch checkpoint
Natural progression reached mode/unit menu at1206 (scenario-menu.png), fame menu
at1240 (mode-select.png), then map1-8 selection at1268 (fame-select.png).
Snapshots/routes and corrected input schemas retained; map1 entry now underway.
Static bank-preserving balanced-stack contract traversal added370 bytes. Current
checkpoint pass-001b verified byte-identical (analysis/pass-001b-verification.json).
Grouped pattern review and exact uniform neighborhoods in round1/pattern-review.md
and uniform-neighbors.txt, including explicit data rationale for$CA/$AA runs.
Disk3.5GiB free; analysis75MiB, no cleanup performed. Later scenarios available
through natural map menu; primary manual confirms selectable1-15, final16 password.
External final-map password lead HEXASOURYOKUSEN remains untested.

### First field / later and final map batch
Natural map1 visibly reached frame1741; map1-ready.png shows hex terrain/units and
Blue turn. Map15 and final16 field construction confirmed separately in
map15-ready.png and final-ready.png, with distinct terrain/unit layouts. Their
wait400 requests timed out partially; snapshot state retained. They are artificial:
clean fame-select.snapshot; patch RAM34=14/15,3E=1,41=191. Final16 additionally
aligns operation then enters inspected epilogue PC8F38 on original menu stack251,
which sets05=FF/copies34->0B/RTS and thus runs ordinary field initialization while
bypassing password authorization. Exact source, edits/context and results in
map15-shortcut-route.json/final-shortcut-route.json. No natural final reachability claim.
Representative field input interactions now executing under trace for all three maps.
Connected static contract batch from field/map-menu roots added5006 code bytes;
proof has50 accepted of99 bank-preserving balanced-stack contracts,2921 decoded
contextual instructions. Checkpoint pass-001c byte-identical, see verification report.
ROM classification: 58.21% (152601 / 262160 bytes classified; 109559 bytes unknown).
Kinds: {'Code': 6980, 'Data': 13895, 'Image': 131072, 'Text': 654}. Preserve prior checkpoints and all evidence; no cleanup.

### Field interactions and static extension
Map1 progressed through turn introduction to CHOOSE THE UNIT TO BE MOVED command
(map1-unit.png), after explicit controller presses/releases and additional message
printing wait. Cursor selection still unresolved; investigating bounded left movement.
Later map15/final16 right/A/B interactions executed and final map scrolled, with
actual accepted fetch context records; no battle or terminal state claim. One final
A8 step timed out partially and release was explicitly applied afterward.
Interaction contract extension added1971 code bytes (2247 decoded contexts,44 of100
callee contracts accepted). Current code8951 bytes; maps saved/validated.
ROM classification: 58.96% (154572 / 262160 bytes classified; 107588 bytes unknown).
Next: locate cursor/unit RAM contract from bounded input differences and consumers;
try tactical/production transition and a practical terminal shortcut if supported.

### Cursor/unit, side transition, map-expansion data
Artificial cursor04/05=[6,20] selected infantryID6 exactly; unit mirror03B7..03BE
[6,6,20,87,248,0,16,129] equals observed PRG-RAM record. Soldier portrait/MOVE
menu and role text confirmed in cursor-unit-menu.png; exact contract/failed earlier
movement premise corrections in cursor-contract.md. MOVE/up/A attempts entered
warning text THEY CANNOT... (unit-orders-action.png), so no successful move claim.
Red side state03FA=1 shortcut invokes ordinary main0314=2 AI route; red-ai-active
ran after220-frame wait and scrolled to red units/new terrain. Its25-frame request
timed out partially; saved snapshot/state and fetch aggregates retained.
Static unit/AI batch added3084 code bytes (12035 total);48 of115 callee contracts
accepted. Map-expansion data batch added4525 bytes; connected4569-byte span uses
16 selector arrays, regular8-byte records/count streams; stage14 overlap and
stage15 third-group exception localized in stage-data-review/proof. Final shortcut
field is artificial/distinct but full natural final initialization not established.
Checkpoint pass-001e generation/verification in progress; prior pass-001d verified.
Current classified162181/262160, unknown99979; all mutations saved/validated.
Latest finished handles61441/21052, current verification handle59152. Next bounded
step: static closure of latest AI/orders fetch roots, then targeted battle setup if
its entry/stack contract is adequately supported; remaining unit/terminal contracts
become concrete redirect questions, not speculative classifications.
### Easy-gain frontier and targeted battle shortcut
Late AI/orders static closure decoded2852 contextual instructions but added only59
bytes (12094 code total). Prior interaction methods now have diminishing gains.
Battle shortcut now executing from clean cursor-unit-menu.snapshot: cursor04/05 and
loaded-unit coords03B8/9 set22,8; target direction0414=2 means(+1,-1) -> observed
red unit23,7; neighbor property03AD=$83 from translated tile table; menu index03B4=0.
After operation alignment, PC=$CA8B (accepted attack branch), stack=$FB preserves
existing return$C2AD at01FC/FD and trims active helper frames. This calls verified
bank6$B327 staging into$7000/$7008, then ordinary combat initialization. No observed
combat claim until staging/frame result reviewed. Live process53297; exact recipe
in battle_setup.py, durable battle-setup-route.json when finished. Terminal/production
still unvisited and require identified contracts; no arbitrary health/flag brute force.

### Tactical shortcut confirmed
Battle setup staged exact7000/7008 records and properties85/83; retained records
artifact and original stack in battle-setup-route.json. Ordinary combat constructor
switched to[0,1,14,15]; battle-ready.png at2506 shows BLUE AH-1S vs RED INFANTRYMEN,
strength15 both, terrain/characters/command windows. Earlier ID6 infantry guess
corrected in gameplay.md/cursor-contract.md; structural record contract unchanged.
A6/release15 tested (release request timed out partially). Select8/release12 settings
attempt current71013 (now polling); primary manual confirms Select opens speed/
normal-auto settings. First tactical static closure added1681 code bytes.
Current code13775, classified163921, unknown98239. No combat-win or ending claim.
Battle/auto/terminal exploration remains bounded; next test settings once initial
message cycle finishes, then inspect strength/flag-unit contract for terminal route.
### Tactical settings and connected text data
Select successfully opened message-speed/battle-mode UI (battle-settings-ready.png).
Right moved speed highlight NORMAL->FAST; two Down/A choices closed settings.
After600 no-input frames combat dialogue and actors changed, but exact Blue/Red auto
flag acceptance remains unconfirmed; do not claim verified auto mode. A6/release15
then progressed tactical messages/actors further to frame3338 (battle-command.png).
Final bounded600-frame no-input observation underway to distinguish autonomous
progress from a pending command. Terminal victory/defeat still unvisited: direct
PRG-RAM health editing unsupported; loaded health/result commit and flag-tank death
transition lack a supported contract, so no flag guessing or arbitrary PC bypass.
New data:41-pointer/26 counted field message records added1018 bytes;33 exact
length/FF equipment PPU records and86-alias/76 control02 battle-dialogue records
added2203 bytes. Boundary proofs and dialogue-consumer decode retained. Existing
ASCII annotations preserved, code at32ED is explicit next boundary. Latest connected
static batch19933; runtime600-frame observation separate isolated server.
### Resolved tactical encounter / final direct static check
Atframe3938 battle-later-result.png shows red strength0 and RED LOST100 POINTS /
BATTLE completion message, AH-1S flight scene. This is a resolved tactical encounter,
not a campaign ending. The600-frame no-input continuation was untraced; a final
10-frame trace captures accepted execution in the result state before handoff.
Settings/battle closure19933 added501 code bytes; ASCII record batch added2203 data.
Final direct easy check:6 already-classified fixed caller patterns JSRF7EA/JSRtarget/
JSRF7F6 provide independent Static effective-bank4 callee-entry contracts. Decoded
F7EA saves A/X/Y, FB09 updates software bank stack, FAF5 performs five serial writes
only toFFFF; runtime MMC1 fixed-upper mode context retained. No Static root is marked
Observed. Source/read proof bank-wrapper/stack/write-decode.json and wrapper-entry-
contracts.json; bounded final static process1152. Next save exact complement, validate,
verify new checkpoint, report full compact pass handoff. Remaining production,
password/natural-final equivalence and campaign terminal contracts deferred to redirect.
### Supported terminal shortcut attempt result
Primary loader CPU93B1 reads7004,Y, shifts4 to strength38,X and masks0F to ammo58,X.
Resolved prior encounter changed red byteF0->00 and ID101->229, blue F8->F7;
these are retained primary observations rather than guessed health flags.
New terminal attempt selects observed special-coordinate blue tank ID17 record
[17,72,20,134,254,0,32,129], relocates preserving coordinate bit40 and changes
mirror03BB strength/ammo byte to0, then same accepted-attack PC/stack producer path.
At2926 flag-defeat-result.png directly shows BLUE M1A1 strength0 vs red infantry15,
but a command menu remains: zero initial strength alone does not trigger campaign
terminal. Normal default A choice now tested once before a final bounded wait; if
still unresolved, result/commit timing is redirect prerequisite, not flag guessing.
Terminal test continuation wait600 timed out partially but saved state/frame: weapon
selection confirmation (BLUE CHOOSE THEIR ACTION / IS SELECTION CORRECT?,7.7mmMG),
not a campaign defeat. This is an explicit missing normal confirmation rather than
failed health contract; two normal A confirmations with releases are now tested,
then one bounded continuation. Static closure96989 finished1762 decodes/341 new
code bytes. No new state edits or speculative terminal PC/flags.
Terminal normal confirmations completed, then at4017 red machinegun-firing scene
against blue M1A1 strength0. Following600-frame continuation60429 timed out partially,
retaining firing/message scene with downward message-continue marker. This is not
campaign terminal and not a failed tool call that lost state. Ordinary A continuation
now dismisses that message; no new state edits. Static confirm closure8315 finished
1644 contextual decodes,828 new code bytes. Both handles closed normally/timeout as
recorded. A zero-strength actor requires combat result path, not constructor alone.

Pre-final preservation checkpoint pass-001f executing while terminal message/input sequence runs in isolated server15452. Current classified178560/262160, code16600, unknown83600; no competing map writer. Disk3.2GiB free at last check; all evidence retained.
### Commit counterexample and localized corrected terminal experiment
Comparative settings route98799 completed5867; black frame atPC$F3D2 is a real
world-record commit search loop, not campaign terminal/loading. Primary decode
flag-result-commit-decode.json: F3B7/F3D5 compare result7001/2 against world6002/3,
then copy7000..7 to matching6001..8. Relocated selected mirror(86,8) could not match
original blue flag-tank world(72,20), explaining failed commit. AH-1S tactical screen
and encounter result remain observed, but its relocated mirror also lacks valid
world commit; no successful return-to-field claim for that original shortcut.
Zero starting strength also did not mark blue ID dead: result blue17 unchanged,
redinfantry101->229 instead. These counterexamples localize the failed premises.

Corrected fresh producer experiment live34324 from clean cursor-unit-menu.snapshot:
selected blue flag-tank17 record retains world coords72,20; only strength/ammo byte
FE->1E gives strength1/ammo14 via proved93B1 loader. Cursor alone becomes22,8;
direction0414=0 delta(0,-2) targets observed red flag-tank104 at86(masked22),6,
property43 from translated tile28 (104&3F). Same accepted-attack PC$CA8B/stackFB,
originalC2AD caller retained, context[12,13,14,15]. Exact patches/source/records in
flag-consistent-setup-route.json when completed. No world-record writes, arbitrary
terminal flags or ROM edits. Expected discriminant: ordinary post-hit death sets
actual deadbit and coordinate-consistent result commit can now return/trigger defeat.

Corrected setup34324 completed and staged ordinary combat context[0,1,14,15].
Current settings/auto continuation41819 replaying original successful route; initial
flag-consistent-battle-ready.png directly shows BLUE M1A1 strength1/shell14 vs
RED T80 strength15/shell14. Selected mirror retains world72,20, target cursor22,8
independent, target world86,6. No initial-zero or relocated-world prerequisite remains.
Route results currently through settings closed2697; two bounded600-frame waits
and final10-frame trace queued. R1-A09 rejection scope and R1-A10 corrected proposal
persisted in leads.json. No restart or additional state edits.
Correction to live frontier:41819 exited143 after first600 wait reached3297; second
wait/final export never completed. This is process termination, not toolkit timeout.
Latest durable snapshot flag-consistent-auto-settings.snapshot at2646 precedes options;
printed3297 state is retained in process output but not an importable snapshot. Resume
from2646, replay exact recorded settings inputs in a short self-contained action batch,
then separate durable600-frame waits. No new patches/reset/baseline replay. Incomplete
flag-consistent-auto result/route files do not exist and must not be cited as completed.
Corrected options resume47835 completed2697; short wait36290 timed out partially,
durable flag-consistent-mid.snapshot at3168. Blue M1A1 remained strength1 but shell
14->13, redT80 still strength15/shell14, so actual blue shot occurred. Red command
prompt now visible. Readback input selector0314=0; two explicit ordinary A presses
with releases on standard ports1/2 (preserving each port independently) executed
82790 to3200. Red menu then visibly points to Ret (retreat), showing a concrete
nonlethal AI/action continuation rather than guaranteed attack/death. No fresh edits.
A bounded600-frame result continuation now runs, seeking actual coordinate-consistent
world commit/retreat outcome. Low starting health alone does not guarantee terminal
if opponent chooses retreat; manual-red attack/mode contract is a concrete redirect
question if no terminal occurs. Last static75592 added22 bytes only; further sampled
CHR planar preview across all16 banks confirms glyph/unit/terrain fragments and is
cross-linked in existing image annotations (no new classified bytes).
Corrected continuation63521 ended partial timeout with durable retreat-result frame:
BLUE M1A1 strength1, RED T80 strength5 and text describing retreat with cloud of dust.
Thus blue shot reduced target15->5, then target retreated; no campaign terminal.
This is useful actual combat/retreat evidence, not merely a menu selection. Ordinary
A message continuation4/release12 now dismisses result text before brief world-return
observation. No repeated fight, changed health or additional object edits; missing
manual-red attack/death control remains a redirect question. All completed prior
snapshots and negative commit proof preserved.
World-return observation51172 completed4106 but remains tactical: red retreat failed/
was caught, red strength5 vs blue1; prompt BLUE AMMUNITION / CHOOSE WEAPON. Thus
retreat animation was not a completed world return. No commit claim from filename.
Straight weapon confirmations now continue the same encounter. Red flag tank is
already damaged to5, so a second accepted shot may yield flag-tank death/mission
terminal rather than blue defeat; either direction uses ordinary gameplay result
logic. No new state edit, new fight or arbitrary terminal branch. Latest durable
frontier flag-consistent-world-return.snapshot at4106, context[0,1,14,15].
Second-shot observation33337 timed out partially, retaining hit/smoke scene with
BLUE HAS... result text; still tactical, not mission terminal. Supported bounded
A4/release56 continuation batch now advances the same encounter through message
prompts, stops if lower PRG context leaves combat[0,1], and records each budget/
actual frame. Context change alone will not be claimed as terminal. No new fights,
no new memory edits. Latest durable source flag-consistent-second-shot-result.snapshot.
Resolution batch82805 finished4961 (five A4/release56 cycles plus final4 traced).
flag-consistent-resolution.png now explicitly shows BLUE strength0 and BLUE HAS
BEEN DESTROYED; redT80 strength1 remains. Thus actual post-hit blue flag-tank death
occurred, unlike initial-zero shortcut. Result record7000 is not yet written back
at this message phase (artifact retains original strength1 record), so no world/
campaign defeat claim yet. Same supported message continuation now runs after
unit-destruction text, stops on PRG context leaving combat; no new state edits/fights.
The corrected health1+original coordinate premise has produced its discriminating
result. Latest durable source flag-consistent-resolution.snapshot, context[0,1,14,15].
Post-destruction continuation45299 stopped on actual lower PRG context change at5201
(final traced frame5205). Result record now blueID145=17|80 and strength/ammo0D;
red104 remains with5D. Coordinates retain original72,20 and86,6. PC at ordinary
F8E8 wait under[12,13,14,15], not invalid F3D2 loop. Black transitional frame alone
is not terminal; brief180-frame no-input observation now allows world/mission result
rendering. This is positive post-hit death plus coordinate-consistent commit path.
### Corrected world commit verified; campaign terminal remains open
Render wait9527 completed5385. flag-campaign-result.png is ordinary map1 CHOOSE THE
UNIT TO BE MOVED field, not campaign defeat/victory. Immutable world readback from
post-destruction snapshot shows unit count16->15, blue M1A1 ID17 absent, redT80 now
index12 record[104,86,6,134,93,2,32,129] matching battle result. Result commit/cleanup
therefore succeeded, unlike prior relocated-mirror F3D2 loops.

Identity correction: blueID17/M1A1 and redID104/T80 are *candidate* flag/objective
tanks based on special coordinate bit40 and initial uniqueness; independent objective
eligibility/turn-loss checks were not proved. Tactical unit destruction and world
removal do not prove campaign flag loss. Earlier unqualified flag-tank wording in
historical plans is superseded by this narrower claim. Missing objective metadata/
turn eligibility is a concrete R1-L01 redirect question; no further fight/state edits.
Broad terminal best effort has a supported attempt and localized negative, not a
blocked whole pass or terminal research-exhaustion claim. Final post-return10-frame
capture now records accepted field execution; remaining fetched-code merge and final
checkpoint/complement/handoff follow. Latest reusable state flag-campaign-result.snapshot.

## Round1 disassembly pass finished — ROM incomplete

ROM classification: 68.23% (178867 / 262160 bytes classified; 83293 bytes unknown).

Unique physical interval union from internal analysis/state.json; Unknown gaps and
raw export fallback excluded. Code16908; Data30233; Text654; Image131072.
All7525 classified regions have evidence;7507 comments,18 labels. Full current-source
readability/semantic commentary remains the next separate phase, not a passed gate.

Final checkpoint [pass-001g verification](pass-001g-verification.json),
[frozen map](../maps/pass-001g.json), [generated source](../generated/pass-001g/game.asm).
map.validate passed; ca65/ld65 rebuild byte-identical. Internal state/current export/
frozen map SHA256 all `7c2b49e7c4417c10fe6a7b5a6f107cb5b48a40b0c46b734bf80c3d139c5c4fbd`; ROM SHA unchanged
`424b05432af2c0c88926373037b7e8264ed21f618ed25f0d73ee67ced9620f37`. No toolkit source/binary or input edits; no publication/cleanup.

Exact residual: [275 unknown ranges](round1/unknown-complement.json),
[bank CSV](round1/bank-counts.csv), [full metrics](round1/final-stats.json).
Data below includes Text/Image. Physical PRG/CHR bank sizes8192; header16.

| Physical bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 16 | 0 |
| PRG00 | 1369 | 630 | 1999 | 6193 |
| PRG01 | 1459 | 2852 | 4311 | 3881 |
| PRG02 | 1086 | 89 | 1175 | 7017 |
| PRG03 | 561 | 1533 | 2094 | 6098 |
| PRG04 | 2434 | 208 | 2642 | 5550 |
| PRG05 | 0 | 3883 | 3883 | 4309 |
| PRG06 | 789 | 0 | 789 | 7403 |
| PRG07 | 0 | 3199 | 3199 | 4993 |
| PRG08 | 6 | 4628 | 4634 | 3558 |
| PRG09 | 790 | 2483 | 3273 | 4919 |
| PRG10 | 0 | 0 | 0 | 8192 |
| PRG11 | 0 | 206 | 206 | 7986 |
| PRG12 | 54 | 8039 | 8093 | 99 |
| PRG13 | 1936 | 3098 | 5034 | 3158 |
| PRG14 | 1695 | 0 | 1695 | 6497 |
| PRG15 | 4729 | 23 | 4752 | 3440 |
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

Execution separately:6020 distinct physical ROM opcode starts,41 PRG-RAM starts,
8260 contextual starts, union of natural/artificial sessions. All48 saved aggregate
drop checks zero; detail capacity0 deliberately drops event detail. This is not
execution-byte coverage, exhaustive gameplay or full hardware compatibility.

Final all-capture merge added285 code bytes and found one local Data conflict:
42DD observed RTS split from42DE23-byte parameter row, independently anchored by
preceding82CE,Y consumer. Original annotation/rejection and final reconciliation
retained in round1/fetch-conflict-review.json and fetch-reconciliation.json; no
outstanding conflicts. Do not propagate that prefix rejection to the numeric row.

Gameplay reached title/setup/map selector, natural map1 field; artificial later
selector14 and selector15 fields, red AI, selected AH-1S command menu, tactical
settings and red-infantry defeat screen. Original relocated-mirror battles failed
world commit atF3D2; their tactical evidence remains narrower. Corrected coordinate-
consistent M1A1/T80 battle produced real M1A1 death and world cleanup16->15, with
T80 remaining strength5/ammo13. Ordinary field returned and accepted post-return
fetch captured at5395 in corrected-world-active.snapshot. Candidate objective-tank
eligibility was not proved; no campaign terminal/credits claim. Terminal best-effort
attempt is documented, with actual objective predicate/turn eligibility deferred.
Production/password validation/natural final equivalence also remain unvisited or
uncertain, explicitly split in scenarios.json.

Last usable runtime: corrected-world-active.snapshot (artificial), frame5395,
context[12,13,14,15], field unit selection. No server left live; new server should
map.load state, emulator.start, import snapshot and reassert desired buttons on both
ports. Do not rerun baseline. Clean natural map1-idle-ready.snapshot and selector
fame-select.snapshot retained for independent branching.

Lead outcomes/history in leads.json (A01-A11 verified/rejected as scoped); ready
R1-L01 objective/terminal predicate, L02 final-stream/natural-final comparison,
L03 remaining supported bank wrappers, L04 factory production, L05 commentary.
This completes one easy-gain pass, not terminal research exhaustion. No repeated
low-health fights without a materially changed objective/action contract.

Toolkit: one confirmed NES input documentation/schema mismatch NES-DOC-001,
workaround buttons-array/port. [Issue report](toolkit-issues.md). No toolkit edits.
Process143 interruptions resumed from durable snapshots; no completed outcome
assumed from lost runtime. Disk3.0GiB free, analysis435MiB/generated37MiB; no cleanup.

Compact next-phase handoff: [disassembly-round1-handoff.md](disassembly-round1-handoff.md).

## Commentary round1 — in progress, batch01

Prepared commentary from frozen pass-001g and confirmed byte identity. Added14
reviewed mechanical routine contracts and bank-local routine/branch symbols;
unknown275 interval notes and CHR16 physical pattern-bank explanations complete.
490/7800 scaffold entries reviewed;7310 pending. Current-source readability remains
incomplete and this is not a redirect handoff. Exact frontier
[commentary-round1/remaining.json](commentary-round1/remaining.json); source-review
scope and batch hash in [commentary progress](../commentary/progress.md).
Fresh commentary verify passed byte-identical with unchanged input SHA256.
No audit or publication; reading copy will refresh when full phase scope is complete.

Commentary batch02:1015/7800 entries reviewed,6785 pending. Frame/controller/PPU/
dialogue/queue contracts and matching bank-scoped RAM/hardware aliases added.
Byte-identical verification passed after both semantic edits and symbols. Full
current-source readability remains pending; exact remaining.json refreshed.

Commentary batch03:1621/7800 scaffold entries reviewed,6179 pending. All classified
bank0 code contracts covered, with raw gaps explicitly unresolved. Added PPU stream
format, modal setting/cursor and battle-side action/commit mechanics. Corrected
nested service ladder count to16 via fall-through doubling. Batch verify passed;
next verify includes that final comment/symbol correction. Data and other code banks
remain pending; no current-source gate or redirect handoff claimed.

Commentary batch04: all classified Data/Text now reviewed, with source format/
consumer notes and honest generic-data limits.1844/7800 reviewed;5956 pending code
entries. Byte-identical verification passed including prior modal ladder correction;
unknown notes restored after data-comment replacement. Full code scope remains.

Commentary batch05: tactical animation bank classified Code fully covered with
actor-field/command identities, phase contracts, renderer algorithms and split
polling explained.2608/7800 reviewed;5192 pending Code. Byte-identical verify passed;
next batch includes8 final operand-name corrections. Exact remaining.json refreshed.

Commentary batch06: setup-bank860 Code regions reviewed, mode-scoped symbols and
state/menu/stage/PPU/CHR mechanics explained.3468/7800 reviewed;4332 pending Code
in physical16KiB banks3/4/6/7. Fresh byte-identical verification passed including
prior final aliases; exact remaining.json refreshed. Full readability remains open.

Commentary batch07: audio bank305 Code regions reviewed with envelope/sequencer/
track-overlay contracts and named record fields.3773/7800 reviewed;4027 pending
Code in banks4/6/7. Fresh byte-identical verify passed; exact frontier refreshed.

Commentary batch08: field-command bank359 Code regions fully reviewed; target
routing/variable-record bounds/neighborhood mask/turn flags explained with field
symbols.4132/7800 reviewed;3668 pending Code in banks6/7. Fresh byte-identical verify
passed; exact frontier refreshed. Current-source gate still pending.

Commentary batch09: field display/unit bank886 Code regions fully reviewed and
field-mode aliases installed.5018/7800 reviewed;2782 pending Code, all fixed bank7.
Fresh byte-identical verify passed; exact frontier refreshed. Concrete staged
comparison uncertainty appended to commentary/uncertainties.md for existing R1-L01.

Commentary batch10: fixed lower8KiB662 Code regions reviewed with reset/DMA, field
input/action routing, panel animation, factory/encounter prefixes and stage tables.
5680/7800 reviewed;2120 pending Code in upper fixed8KiB. Fresh byte-identical verify
passed; exact frontier refreshed. Fixed-field operand consistency still scheduled.

Commentary batch11:960 upper-fixed tile/scroll/expansion/overlay Code regions
reviewed.6640/7800 reviewed;1160 pending Code. Fresh byte-identical verify passed;
exact frontier refreshed. Remaining fixed input/attributes/neighborhood/movement
contracts and operand consistency still required before phase handoff.

Commentary batch12: inventory-derived7505/7800 reviewed;295 pending Code.
Attributes/cursor/neighborhood/movement/count/multiply contracts complete. Corrected
neighbor selector to movement after its consumer was read. Byte-identical verification
passed; fixed operands and final scope checks remain.

Commentary batch13:7800/7800 scaffold contracts reviewed, including all7525 classified
regions and275 explained unknowns;pending0. Verify passed byte-identical. Current
phase frontier is final fixed-field/cross-bank operand consistency and resolved role
corrections, then semantic source references/full-scope check, final verify/reading
refresh. No commentary handoff or final editorial acceptance claimed yet.

## Commentary round1 complete — current-source readability gate met

All7800 scaffold entries reviewed, including all7525 classified regions and275 exact
unknown notes.387 meaningful units,1245 named labels and358 equates. Fixed aliases,
bank-aware call targets, branch naming, resolved-role corrections and current source
references reconciled. No classified-source readability gap remains; final editorial
acceptance is pending and no audit was run. ROM classification remains68.23% with
83293 unknown bytes; pass-001g/map/input identities unchanged.

Final fresh byte-identical verification: [report](../commentary/verification.json).
Source SHA2567fefa470f60fbfb9b9f3c25176fd57ceb5161ae6adffb1740bed743a24b9f661.
Reading copy refreshed afterward, SHA256
e7de6aebf85a2c04b2d6b67f97ad8ee214b976d997e7469bdedc9f4ff5a45a18.
[Scope assessment](commentary-round1/scope-assessment.json),
[unique units](commentary-round1/units.json),
[compact handoff](commentary-round1-handoff.md).

No live server/map writer/tool handle. No runtime needed, map/ROM/toolkit changes,
publication/archive or cleanup. No new toolkit issue; existing confirmed NES-DOC-001
is worked around/unblocked: [issue report](toolkit-issues.md). Disk3.0GiB free.
Next phase is coordinator-reviewed redirect; no terminal exhaustion claim.

## Redirect round1 active — static continuation research

Attempt `analysis/redirect/r1-static-continuations/`; checkpoint pass-001g/map7c2b49e7 unchanged, classified178867 and unknown83293. Read-only factory graph currently contains2234 previously unknown candidate bytes (968 instructions), pending independent disassembly review; no map/source/classification edits. `graph-decode.json`, `graph-unknown.json` and `graph-contracts.json` retain primary decoding/contracts. Exact field-mode3 precondition: MMC1 control shadow17 values1C/1E and split1D retain mode3; paired audio selection restores prior PRG selector. Supported selected pointer A5/A6 is6001+8n, Y0..7 for writeback. Toolkit conservative barriers alone do not reject these continuations. Comparison03FA counters increment on action/reset per-turn, so objective interpretation remains unsupported. No unknown direct JSR/JMP target from accepted saved views; gaps stem from return/mapper/indirect barriers. Next: independent final password state5/schema and counter writers, then remaining static wrappers. No live server/handle. No toolkit edits/new issue, cleanup or publication.

Redirect checkpoint: supported selector entered MAP16 password UI (password-ui.png). Artificial buffer46..54 and latch3C patch accepted by actual9732 comparator/ordinary initializer;960 bytes6101..64C0 exactly match retained bypass final-ready snapshot, so third-group exception is not a bypass artifact. Equivalence restricted to this expansion, no natural-input claim. Field rendering/interaction continues from saved accepted snapshot. Independent bank5 data-container investigation now has consumer-anchored16-stage row directories8010 and encoded-map directoryB418, regular10-byte rows with localized stage2/14 overlaps; preparing bounded noncode proposal rather than requiring exclusive schema. See attempt frontier/artifacts. Map/source/ROM unchanged.

Redirect checkpoint: password validation route now has visible final field and bounded right/A interaction (password-field-interact.png). Scope of bypass equivalence stays exact960-byte expansion only. Counter producerB1C6 counts strength/ammo-nonzero records per owner;03F3 quota table8C77 holds3/16, and end-turn handlerC40B copies quota into active action counter before updating phase. No campaign-objective predicate found by this path. Bank5 exact directory/stream/pattern boundaries corrected: finalBBC4..BC2D=105=1+2*52,47six-byte patternsBC8B..BDA5; generic limited-alphabet tailBDA5..BFFA and vector mirrorBFFA..C000. Stage2 reuses earlier row831A; stage14 windows overlap with5stride. Static graph carry tracking removes impossible SEC/BCS fallthrough PHP atA900: candidate count2965 bytes before next scratch-list continuation, not2966. New bank4 clear has proven6666+33*k(k0..15) indirect stores, next constructor tail under review. Map/source/ROM unchanged; no persistent live server.

Redirect checkpoint: bank4 scratch-list constructor closes after bounding pointer62=6666+33*k(k0..15), max8bit indexed destination6954. Static graph3297 unknown bytes before audio root. Audio independent container proof:58sound-request pointer slots and58terminated records,185stream roots parse11661bytes throughB5E4; only one unreached inter-stream byteA8FD, final2582FF bytes already uniform evidence. Inline tables/period/track/duration/envelope bounds independently supported. Artifacts audio-requests.json/audio-stream-proof.json/audio-data-boundaries.json. Next battle-return static path CA96..CAAC restoring bank6/control1E then E042/E045/CD39; may resolve actual terminal condition, no repeat combat. Map/source/ROM unchanged.

Redirect checkpoint: factory coordinate route now independently grounded: world records46/47 at(7,19)/(7,21) use bank4unit patterns whose firsttilesF3/FC map to factory code20/21. Basegrid scan negative was correctly narrowed to basebank5layer, not factory absence. Trying only supported cursor7,19 from corrected-world-active(count15), preserving snapshot and ordinary A input. CD39 corrected from candidate terminal to field redraw; F389 factory-slot clearing and F4B7 dead-record compaction now decoded. Distinct handler continuations followed; generic dispatch pointer bytes proposed separately from unresolved exact directory partition. Map/source/ROM unchanged.

Redirect checkpoint: cursor-only factory retry selected record46 but sampled center85 (terrain), leaving choose-unit prompt. This identifies stale tile address as missing synchronization: MoveFieldCursor returns early on zero direction, before recomputing038D..0390. Improved recipe adds logicalcolumn/row7/19 and exact computedPPU2116 (A887 routine) alongsidecursor4/5; bounded retry active. No conclusion that factory coordinate/unit graphics hypothesis failed. Wrapper saved effective views corrected for post-F7EA middlecalls to[8,9,14,15] and final restoredRTS[12,13,14,15]; source/map untouched.

Redirect checkpoint: corrected cache order[HI21,LO16] preserved exactPPU2116, but sampledcenter03A7=06/code85 while physicalVRAM0116=F3. Ordinary factory selection remains unconfirmed due specific PPU/mirroring sampling context. No more blind coordinate retries. Supported artificial CB39 entry now tests mechanics from selected actualfactory46/stage0/player0/available03FB80, explicit03AE20 and preserved caller stackFB; stack primary evidence has[AD,C2] at1FC/FD and[1B,C1] at1FE/FF. Saved snapshot precedes mutation. Live35856 bounded runtime; no map/source/ROM edits. Static codeproposal4766bytes before observed-union reconciliation.

Redirect correction: saved roots show NO F19F/EF3A/C55D fetch in cursor/cache trials. They resumed insideexistingchooser, so stale03AE was never recomputed; no PPU/mirroring rejection or bug claim follows. DirectCB39 entry visited actual infantry production panel (factory-entry.png); ordinary A commit in progress15358. SourceC5D8 direction0307 exitschooser viaC5D1 RTS, enabling parentfieldrefreshC155. Preparing one-frame direction/release route fromclean state, verify refreshedcoordinates/codes beforeA.

Redirect checkpoint:2232 actualA commit reachedCC17 and manufacturing message20, then visible palette fade (factory-commit-confirm.png);03FBstill80/count15 at5749 is intermediate, continuation pending. Earlier15358 input was beforepoll and not commit-negative. Ordinary16120 direction input likewise did notfetchEF3A; savedstack containsCC96 delay returnC652, so it was duringblockingwarning. New method must wait forknownchooserpoll beforeinput. Both handles completed; no live server. Map/source unchanged.

Redirect decisiveL01 correction: C446/AC1E/AF13 tests savedbattle tilecodes6FFE/6FFF AND42==40/42 and correspondingdescriptor deadbit80; sets0420=1/2 thenfade/trampolineFFD7(SelectPRG2,JSR8033). terminal-entry-contract.json proves M1A1firsttile11=>dictionary41 and T80tile28=>43. Old artificialcenter83 masked02, so no-terminal outcome did not test objective eligibility. New bounded artificialwriter/consumer shortcut41596 updatesonlytwo savedcodes using first5 gameB327 instructions, verifies unchanged16descriptorbytes, thenC446/stackFB; renderedoutcome pending. Productionfinish observed stagedrecord[14,135,19,164,240,0,32,243] at03FB whileworldcount15. Naturalroute eventgate timedout duringC386panelanimation, no bugclaim; resumedouterC10B gate. Staticgraph5562candidatebytes anddata28864proposalbytes(noannotation/conflicts). Map/source unchanged.

Redirect checkpoint: actualBLUE HAS BEEN DEFEATED rendered afterboundedterminalcodecorrection,0420=1; all16resultbytes unchanged (terminal-route.json/terminal-outcome.png). Ordinaryfactoryparent gateC10B matched after84396operations,coords7/19/code20 naturallysampled,ordinaryA reachedproductionpanel (factory-parent.png), noPC/centerpatch onthatroute. Previoustiming/cachefailures localized byfetch evidence, no PPUbugclaim. Connectedfinaloutcome dispatch97A7 with0420=2/singleplayer0/stage15 leadsA088. Newexplicitartificial result-state shortcut63260 fromcleanloss snapshot patchesonly38/0420 andPC8033; actualfinalending pending. IndependentA/release lossrestart28302 active. Map/source/ROM unchanged; no publication/cleanup/toolkit edits.

Redirect checkpoint: finaloutcomeentry63260 actually fetchedA088 and renderedBLUE HAS WON intro. State0 intentionallywaits60*14frames via34/35; no failedfinaldispatch. SourcedsixstateRTStableA0FB..A107 targetsA113,A317,A4DA,A5ED,B013,B0F2. Timer shortcut[59,13] at34/35 frompreservedintro snapshot nowadvancesordinarysceneinitializer; nootherresult orselectorpatch. Connectedstaticgraph93109 usesbank2-scoped40byteRAMhelpercontract(select3/audio/restores2), boundedOAMcopy andCHR/controlserializers; noRAMopcodeattributedtoROMexecution. DefeatA/release restart28302 completed, evidence retained. Map/source/ROM unchanged.

Redirect checkpoint:17100 timeradvanceinitializedendingstate1; THE BATTLE IS OVER rendered. Counter01 counts112->0 every2frames after96framelead-in; sourcegatesA/Start when01=0/33=0(default00bit2=0), finite90framecontinuationthenAexit pending. All6primaryendinghandlersplus7secondarytargetsA5F7..A605 typedthroughinitializerA4DA/A683 andbounded34indices0..6; source03Bbit2variantnaturaleligibility remainsseparate. Staticgraph46274 extendedconnectedhelpers; no ROM/map/source edits.

Redirect correction: twoLSRs inA113/A317 testmask02(originalbit1), not04. Erroneous04variant78141 preserved and stayeddefaultstate1; no alternate-ending rejection follows. Correctwriterchain0A(option0..2)->3B->00 identifies thirdFameoption2 as supportedmask02producer. Newcleanstate0initializermode02 test active; dependentinit notbypassed, result/timerartificialprovenance retained. Defaultending84550 completedA/reset withactualC000fetch. Staticgraph9099candidatecodebytes/4078instructions,10validtypedPPUpacketroots. Map/source/ROM unchanged.

Redirect checkpoint:correctmode0282106 actualstates1->2->3; properstate0initialization01=F0/02=1, nowstate3 afterbounded3000frameobservation, continuationfrontier retained. Source-fieldinvalidations savedcommentary-invalidations.json(25sourcehits,9dependentunits):03BA/recordbyte3 isnotcombatHP/ammo; byte4/03BBchanges1E->0D whilebyte3=86 unchangedinactualtankdeath. Movementlowbyte3cost1/2/3independentlyprovestravelbudgetmechanics; priorcounterstrength/ammowordingcorrectedmechanically. Staticphysicaldedup8909candidatebytes vsgraphview-sum9102(193duplicatedbyviews); data30258physicalproposalbytes, no overlaps. Noacceptedclassificationchange.

Redirect checkpoint: hardcreditsactualAssistantProgrammerT.ISHIHARA atstate3/page2; normalAF8Eincrements35 andA663tests7->B0A0finalscene. Page35=6shortcut34950preservescurrentfadeandproperfinalinitializer. ProductionB7CB/8090consumerdecodedthroughactualINC6000; supportedturnresetC13E->E036sets040Cfrompendingpositive03FB. Bounded48616 assertsplayer0/count15/pendingID14, thenvalidatesexactinserted8bytes+15oldrecordsshiftedif16; noarbitrarygateflagpatch. Connectedproductionvectors8093/8096understaticcontractinspection. No map/source/ROMmutation.

Redirect checkpoint:credits34950 reachedproperfinalsceneB0A0/state4. Timer34notresetat60;35incrementsat60onceper256wrap, last34=1/35=3need59frames forstate5. Production48616provedturnreset040C=1,but0415=1selectionlatchpreventedC2D2/8090call(noactualfetch). SourceC40BEndTurnclears0415; correctedentrycontractrunsactualC40BwriterthroughRTSC40AthenmainnewturnC13E. Notfailedbirthmechanics. Staticconnectedproductiongraph10126view-sumunknownbytes,newphysicaldedup pending. Map/source/ROM unchanged.

Redirect checkpoint: credits3030reachedstate5andA/releaserestart. Production59314cleared0415throughactualC40BbutstalledC161/E087beforeC10B; eightpartialtimeouts=214actualframes, no8090/B7CBexecutionclaim. RenderentrymissedpairedE093upperrestore/CHR/audio. NewboundednormalizeE093bodythroughactualF962RTSthenmainC10B/SPFF usesverifiedplayer0/latch0/gate1/pending14/count15; source/OAM/stackevidence production-turn-state.json. Anothermissinghardwareprerequisitewillbedocumentedratherthanendlessnormalization. Map/source/ROM unchanged.

Redirect finalresearchoutcome:56731properE093->F962normalizationmatched309ops,mainC10Bthenactualbirth count15->16; exactpending8byteinsertandall15old8byterecordsshiftunchangedassertionspassedfromalready-resetbaseline. Creditsstate5andA/resetcomplete. Allruntimehandlesclosed. Finalpackaging/observedunion/alignment/complement/leadsinprogress, no furtherexploration. Bank3footervalueverifiedFFFF/C000/FFFF, so genericfooterdatareplacesunverifiedC01Bvector-mirrorinterpretation; bank5mirrorC01B/C000/C01Bconfirmed. Source/map/ROM unchanged.

## Redirect round1 COMPLETE — combined ready batch

[Compacthandoff](redirect-round1-handoff.md),[summary](redirect/r1-static-continuations/summary.json),[codeproposals](redirect/r1-static-continuations/combined-code-proposals.json),[dataproposals](redirect/r1-static-continuations/data-proposals.json),[exactcomplement/dispositions](redirect/r1-static-continuations/unknown-ledger.json).40517uniquephysicalcandidatebytes=10094Code+30423Data; zeroalignment/code-data/existing-map/viewidentityconflicts.1220instructionswithactualartificialfetchprovenance; staticviewskeptdistinct. Ifallaccepted,Unknownwould42776. Acceptedmapstays178867classified/83293unknownin275intervals; map/ROM/sourcehashesunchangedandoldbyte-identicalverificationremainsapplicable.

Actualdefeat,ordinary-inputfactorypanel,pendingproduction,exactworld15->16birthwithINC6000fetch,staffcredits/ENDandAresetverifiedunderexplicitartificialcontracts. Failed/wrongpremiseattemptsretainedandlocalized; noearnedcampaignclaim. Leads/history/scenarios/gameplay/investigationupdated. Nineoldcommentaryunits/25sourcehitsrequirebyte3/byte4dependencyreconciliationplusnewsource; mask02andfootersemanticscorrected. No newtoolkitissue;NES-DOC-001workaroundunchanged. No liveemulator/server/writer/handle, no publication/archive/cleanup/toolkitbuild. Nextindependentdisassemblythenfullcommentary/redirect; notterminalresearch-exhaustionorcompleteROM.

## Round2 disassembly — in progress

Input verified checkpoint pass-001g; map SHA2567c2b49e7c4417c10fe6a7b5a6f107cb5b48a40b0c46b734bf80c3d139c5c4fbd;178867 classified/83293 unknown. Independent combined-ready-batch review underway in analysis/round2. Configured release binary freshly re-decodes every proposed instruction and checks exact physical identity/operand alignment. Primary contracts separately reviewed for F7EA/F7F6 bank4 wrappers, F2E1/FB17 split, bounded world/attribute/OAM writes, mode3 mapper serializers and RAM helper select3/audio/select2 restoration. No source/commentary edits. Audio footer vector-mirror wording will be narrowed to generic FFFF/C000/FFFF words. Existing printable footer firstbyte1FFEF remains ambiguous and will be withdrawn locally from Data rather than silently promoted to Code. Current live audit shell91414; no emulator started. Disk2.8GiB free; no deletion.

Round2 audit91414 complete: all4531 instructions freshly matched,10094Code+30423Data bytes structurally accepted subject to local audio-footer wording correction. Apply22113 still live, no emulator; persisted map remains original178867 until the atomic save at batch end. API id-shape retries never saved and did not alter persisted state. Exact previous footer annotation retained in round2/withdrawn-footer-annotation.json; unsupported firstbyte will be Unknown. Follow-on review independently found197additional instructions/427bytes from residual valid branch paths and bank-restored conditional wrapper returns, plus122byte A900 offset/sentinel slab. A900 cannot be fallthrough code because A8FD SEC forces BCS; its actual indexed consumer and bounded sentinel records establish Data. Nine selectors share/overlap records, so no disjoint directory claim. Additional traversal stops at A5F0's record-loaded62/63 indirect store: its destination range needs a producer proof before preserving post-write bank context. A5FD matched-record return contract is local; the unmatched RTS leaves its saved registers on stack, so no unconditional safe-return claim. These small connected continuations are being completed before pass handoff; source/commentary untouched.

Round2 initial apply22113 COMPLETE; validated/saved/exported219383 classified/42777unknown (40517 accepted candidatebytes,1 prior unsupportedfooterbyte withdrawn). No live writer/emulator. Follow-on fixed-row primary proof saved round2/fixed-row-proof.json:16stage records,159rows,954destinationtriples, all6112..642B withshifts0/2/4/6;1+21*count<=253. Each FFseparator and nextpointer independently bounds its record; whole3403byte95F0..A33B slab previously Unknown. Scope is loader-populated/unmodified pointer fields. This supplies A5F0's boundedstore contract; continuation now203instructions/438bytes withno unresolvedlocalstops, plus122A900Data bytes. Final follow-on annotation/checkpoint pending; investigating independently referenced adjacentA33B directory before ending easy-gain pass. Source/commentary unchanged.

Adjacent A33B review complete:16stage pointers/123counted coordinate/value triples occupy417bytes throughindependentA4DCcodeentry; all16recordendpoints equalnextpointer. AE61loads4beforeE06F, so callerbank4 remainsbank4; E072restoresprior4. Narrowcallercontract adds63Codebytes without globally claimingE06Fpreservation. Follow-on finalcandidate501Code+3942Data=4443bytes; sole livewriter8724 nowapplying viaAPI. No emulator/sourceedit. Primaryproofs and stopped/continued reasoning inround2/structural-review.md; localcontinuationstops nowzero. Finalvalidation/checkpoint/complement pending. No further obvious productive normalgameplay or connectedreadybranch beyondthis supportedbatch; savedending/production evidence reused.

## Round2 disassembly COMPLETE — ROM INCOMPLETE

ROM classification: 85.38% (223826 / 262160 bytes classified; 38334 bytes unknown).

Fresh checkpoint pass-002 verified byte-identical: analysis/pass-002-verification.json,
maps/pass-002.json and generated/pass-002. Current map SHA256 38b30eb21a0c8e9c601f7d1b4d065e37185fec9a4855e2dc331582b1e632f959.
Unique classified counts: Code27503,Data64597,Image131072,Text654. Netgain44959:
readybatch40517 accepted, one unsupported old footerbyte withdrawn, connectedfollow-on
501Code+3942Data. Exact244interval complement and perbankcounts saved in
round2/unknown-complement.json and final-stats.json. Header16 andCHR131072 entirely
classified; PRGcounts below.

Independent primary contract/boundary review: round2/structural-review.md,
independent-decode.json/alignment-review.json; all4531readyinstructions freshly
matched but legaldecode alone was not acceptanceproof. Genericbank3footerwords
replace stalevectormirror wording; previousfooter1FFEFannotation retained andonly
thatbyte returnedUnknown. Follow-onA900lists, stagefixedrows159/954destinations,
and coordinate/value123triples supplied4443furtherbytes, ratherthan firstbatchstop.
Primaryproofs and leadoutcomes inround2/; R1-L01..04,L06..08 consumed. 981Dreturn
unsupported byendingresetcontract;1FFEFstillstructurallyambiguous.

No new gameplayreplay: saved actual artificialfetches establishObservations while
staticviews remaininferences; no naturalcampaign/earnedvictory claim. Existing
broad milestones include later/finalfields/password, combat/defeat, factorypanel/
pendingproduction/exactworldbirth andstaffcredits/END/restart. Connected obvious
staticcontinuations completed; no remaining productive readyrecipe inthisbatch.
This is single-pass easygain exhaustion, not terminal research orwholeROMcompletion.

Currentcommentary/source unchanged(hash7fefa470f60fbfb9b9f3c25176fd57ceb5161ae6adffb1740bed743a24b9f661).
Next fullcommentary mustpreserve priorwholecommentary/reviewhistory andcarryvalid
physicaldependencies, while correcting03BAbyte3movement vs03BBbyte4combatHP/ammo,
CB39/E048activeplayerunitcount/16comparison,turnquota andendingmask02; see
redirect/r1-static-continuations/commentary-invalidations.json. Also reconcile new
1FFEFUnknown boundary. Newsource501+ready10094Codebytes needs fullreview.

Mapvalidated/saved/exported. ROMunchanged. No toolkitissue added; NES-DOC-001
workaround remains. See toolkit-issues.md. No toolkitedit/build/publication/archive/
cleanup/deletion. Disk2.8GiBfree; round2evidence4.1MiB,scratch12KiB. Allhandlesclosed.

| Physical8KiB bank | Classified | Unknown |
|---:|---:|---:|
| 0 | 1999 | 6193 |
| 1 | 4311 | 3881 |
| 2 | 1175 | 7017 |
| 3 | 2094 | 6098 |
| 4 | 5598 | 2594 |
| 5 | 6140 | 2052 |
| 6 | 8176 | 16 |
| 7 | 8192 | 0 |
| 8 | 7222 | 970 |
| 9 | 6249 | 1943 |
| 10 | 8176 | 16 |
| 11 | 8192 | 0 |
| 12 | 8148 | 44 |
| 13 | 7805 | 387 |
| 14 | 3271 | 4921 |
| 15 | 5990 | 2202 |

## Round2 commentary in progress — first semantic batch

Previous whole commentary preserved at analysis/round-2/commentary-pass001g before fresh pass-002 preparation; preparation byte-identical. 7,524 classified entries carried by exact physical range/kind/view, with prior387-unit credit retained. Newly classified scope4,830 entries/44,960 bytes; one withdrawn footer byte is now Unknown. Current semantic work starts with password/outcome/ending/credits and setup/audio continuations. Exact pending frontier analysis/round-2/commentary-frontier.json; specification analysis/round-2/semantics.tsv; commentary/progress.md tracks dependency corrections. Readability remains in progress; no handoff to redirect or final audit authorized by this checkpoint. Configured-binary semantic-batch verify running via exec13915; no live server or map writer.

Round2 commentary batch update: initial semantic verification13915 PASSED byte-identical, source85ee26d5. Second application16502 complete:12414reviewed/184pending (121Code entries+63Data,34688bytes). Exact tails isolated to13runs; their semantic specs and mode-safe alias corrections now applying in52808. No server/map writer. Data consumer/boundary evidence inspected; data edit plan saved. Nine historical invalidated units still require prose/review reconciliation and are not counted repaired. Phase remains commentary until fullsource gate/finalverify/read; no audit.

Round2 commentary fullscope checkpoint:12598reviewed/0pending; all4767newinstruction entries and63newdata ranges covered.641distinctmeaningfulunits;29historicaldependency/boundaryrepairs includeall9explicit invalidations. Verification22419 passed59c40f1b; consistency45765 passed56e95187 andread refreshed. Finalexact interior-helper naming prompted one further coherentverify/read/scope54825. Source/comment/review locations current; oldpass001g wholecommentary/sourcehash retainedunchanged. Currentuncertainties rewritten with supersededR1-L01/04 premises corrected; exactUnknownfrontier unchanged244intervals/38334bytes. No finalaudit. Await54825 then save completedphasehandoff; no server/map writer.

## Round2 commentary COMPLETE — ROM INCOMPLETE

ROM85.38% (223826/262160classified;38334Unknown/244intervals). Fullcurrent-source readability complete:12598reviewedentries/641distinctunits/0pending;7524exactrange-kind-view carriedentries;all4830newclassifiedentries reviewed;29historicaldependency/boundaryrepairs includeall9explicitinvalidations. Fulloldcommentary preservedround-2/commentary-pass001g, source7fefa470...f661 unchanged. Sourcecorrected03BA lowtravel/highpersistentresource vs03BBcombat; turnquotas/eligibilitycounts; factorycount equality16; endingmask02; boundary1FFEFUnknown; explicitbank4/restoredbank6views. Newentrylabels/interiorcalls reconciled;12numericcontroltargets allUnknown.

Final54825 configured verificationPASSED byte-identical, source9e2e25c7d996ddf316c0a6351ee5d1b7278f52437a55a6a96a5acd83b8e2fcb0; reading-after refreshed. Currentsemanticreferences/fullscopeassessment savedround-2/commentary-assessment.json,current-scope-check.json,commentary-frontier.json; currentuncertainties refreshed. Compacthandoff analysis/commentary-round2-handoff.md. Finaleditorialaudit pending/notrun. No map/state/ROM/toolkitedits, archive/publish/cleanup/delete. No newtoolkitissues; NES-DOC-001remains. Disk2.7GiBfree;round-2preservation/evidence48MiB,scratch12KiB. Noactivehandles/server/writer. Commentarygate complete; nextphase redirect, ROMstillincomplete.

### Round2 redirect — active r2-tactical-frontiers
Input pass-002/current map38b30eb21a0c8e9c601f7d1b4d065e37185fec9a4855e2dc331582b1e632f959;
classified223826/262160,38334Unknown in244intervals. Current-source commentary gate reused.
Read-only current-source branch/continuation and explicit-question inventory saved in
analysis/redirect/r2-tactical-frontiers/. Command dispatcher A3E8 directly consumes
75word candidates A3EB..A481; accepted A481 routine supplies independent endpoint.
Corrected static graph treats24nested inline phase tables as data barriers, rather
than naively decoding the post-JSR words (initial exploratory result retained).
2954decoded graph instructions,5712Unknown candidate Code bytes, one localized
conflict atCPU9826/physical5836: accepted Data starts with RTS reached by981B BNE
and9823 JSR A48A returning contract. No map/source/state changes; conflict not resolved
by redirect annotation. Primary phase graph and old pattern-review retrieved.
RenderTacticalActorsForward/Reverse's A69B directory independently fits176words,
171unique count+4n sprite records with no gaps, fromA7FB throughBA1E, exactly next
accepted uniformFF suffix physical7A2E. This supports4995Data bytes, some already
classified; semantics and exact proposal intersections still being reviewed.
Independent source branches/return continuations remain actionable; redirect continues.
Both read-only server handles16015/94039 completed, no live server. Disk2.7GiBfree;
no toolkit edit/build, cleanup, archive/publication or ROM changes. NES-DOC-001 reused.

Round2 redirect continued independent source frontiers: initial graph6550instructions/
3341candidateUnknownCodebytes; expanded graph8262/5299 provisionalbytes. These are
exploratory counts, not accepted/proposal totals. Exact bank4 C547 wrapper and successful
A5FD row-match precondition identified; DA94 bank4 stage-table and D52A bank0 battle
crossings require explicit changed/restoredviews. D196 exposes66byte nested pointer-like
material andfour coherent handlers. Apparent fifthD71A pointer instead decodes recipe-
like low-valued data; provisionalfive-handler claim withdrawn locally, semantics/domain
and66byte non-code role separated. Primary bank-barriers.json and fullmethod history
in analysis/redirect/r2-tactical-frontiers/research.md. Handles96269/69250/94593 complete;
no livewriter/server, no classification/source edits. Redirect remains active.

Tactical batch packaged into persistent readyR2-L02/L03/L04:5712UnknownCode,
416inline wordsData,4964spriteData,1984numericData =13076Unknownbytes proposed;
oneacceptedData byte5836 proposedCode retype. Noacceptedclassificationchanges.
Sprite piececount1..32 fits byte-index loops. Configureddecode/immutable proofs and
exactunknownintersections in tactical-{code,data}-proposals.json; no instruction/table
or acceptedCode/Data-candidate overlap. Independent scopedgraph36383 completed8545
instructions/6030provisionalbytes;98603 completed8987/6671, includingDA94 bank4/
D52A bank0 continuation views. Scoped bank4samebank4 selection sitesAE63/AE8D and
B0E8/B12F/B146 identified; theyrestore4, not6. Bank0patternB68D resetsMMC1shift then
CHRonly serialwrites; mode3 precondition retained. Missingchooser8134 inline table
has producerAND0F, exactly16words8137..8157 withknownnextentry8157. Improvedgraph67524
is currentread-onlyresult; no newruntime, noROM/map/source/state mutation.

Independent graph58072 finished10533instructions/8831provisionalviewbytes; allopcode
starts official and no operand-overlap found in prioruniquephysicalaudit. Newbank0
matrix98CE(9words)/9725(4words) and chooser8137(16words) have finiteproducer contracts.
Battleassets20directories/20descriptors/101PPU roots prove3525totalData bytes exactly
9B1C..A8E1. Reset-template proof proposes28Code/89genericData with explicitmode2->3
views,7sourceEAbytes retainedUnknown. Exact56byteRAMcopy overlaps949EfirstCodebyte;
only55 usefulsourcebytes proposedData. MissingAIlookup unbalancedreturnlocalized,
not universalCode veto. Research/frontier updated; currentread-onlygraph finishes
AF13tailFFD7 sink and alreadyidentifiedsetup8686/controller8178/AC18 continuations.
No map/source/state mutation, no new runtime or toolkit issue.

32154 completed11636/9215provisionalviewbytes;73701 fixed55JMP-service check completed
2330instructions/133candidateviewbytes. Bothhandlesclosed, noactiveprocess. BC0C
03-stringunion/BD9Fcodesplit retained. connected-data-proposals.json adds9240Unknown
Data candidates, with no accepted/graphCode overlap. Two failedextentpremises were
localized:91E1 copy68 =PPUpacket36 +targetpalette32; queuedtarget stage0count4 leaves
46bytesfitting8extrarecords, genericData only. Other15stagequeuedlists meet nextroot,
lastendsE2A0. Fullnewresearch/proofs preserved, map/source/state unchanged. Next:
uniquephysicalaudit, practicalremaining setupasset roots, exactcomplementledger and
combinedleadshandoff afteridentifiedactionableavenues reasonablyclosed.

### Round2 redirect READY combined batch — accepted ROM still incomplete
Completedr2-tactical-frontiers research afterconnected+independent handler/consumer
closure. ReadyR2-L02..R2-L08 propose38310uniqueUnknownbytes (15344Code/22966Data),
plus5836's1byteData->Code repair preserving16byteData suffix. Acceptedmap/state remain
pass-002:223826classified/38334Unknown/244intervals, hash38b30eb21a0c8e9c601f7d1b4d065e37185fec9a4855e2dc331582b1e632f959.
Ifallaccepted,24Unknownbytes/11intervals remain, not a complete-ROM claim. Exact
244range ledger includesphysicalbanks/CPUaliases/neighbors/proposaldispositions;
summary,code/data proposals,audit,viewcorrections,residualreview andhandoff areat
analysis/redirect/r2-tactical-frontiers/. Candidateinstructionstarts crossnoaccepted
Code andhave0operand/Code-Data conflicts. E006/E009/E00C views correctedfreshlyto
bank5[10,11,14,15]; exploratoryfield6low-datareferences excludedfromprimaryproof.
Allother explicitchanged/restoredbankcontracts andnegativepremises retained.
ROM/source/currentmap hashes independentlyrechecked unchanged (424b0543...,9e2e25c7...,
38b30eb2...). Current-source commentary641units/12598entries remainscompleteforold
scope; newbytes requireindependentdisassembly/fullcommentary. R2-L01/R2-R01 kept
localizedambiguity history. No newruntime/patches or toolkit issues; no livehandles,
map/source/state/ROMmutations, toolkitedit/build, cleanup/delete/archive/publication.
Disk2.6GiBfree. Readyforcoordinator's independentdisassembly, notterminalexhaustion.
Finalpackagingverified all38310proposal bytes/24residuals andfreshbank5veneer view
corrections. Data package/ledger includeexactphysicalCPUaliases and8KiB contexts.
No live run.pyserver confirmed. Sevennewready leads andR2-L01/R2-R01localizedhistory
persisted; handoff completeat analysis/redirect/r2-tactical-frontiers/handoff.md.

### Round3 disassembly phase — independent review and application

Running exactly one disassembly pass from accepted pass-002. Read only this phase's
prompt plus required policies/workflow and r2-tactical-frontiers handoff/primary proofs.
Fresh configured-release decode independently matched all6702proposed instruction
views in223boundedgroups, exactphysical bank mapping, operand/accepted-region audit,
and counted sprite/battle/setup/credits/unit dictionaries. Structural role contracts,
partial-return constraints, inline-data barriers, genericData fallback and bank5
E006/E009/E00C overrides reviewed in analysis/round3/structural-review.md; machine
results in independent-review.json. All readyR2-L02..08 structurally supported;
localized5836Data->Code repair preserves16byte suffix. No newruntime/replay/patches;
reviewed saved broad progression including later/finalfield, battlecommit, production,
defeat/defaultending/staffcredits/END. No naturalcampaign/earnedvictory claim.
Prephaseinternalstate retained in analysis/round3/input-state.json. Applying through
map.annotate and preserving acceptedData/Text islands. Reviewserverhandle78864 closed;
currentapplyhandle56963 owns one configuredrun.pyserver; checkpoint pending.
Commentary/source remains untouched for its next fullcurrent-source review.

### Round3 disassembly complete — pass-003b, ROM remains incomplete

ROM classification: 99.99% (262136 / 262160 bytes classified; 24 bytes unknown).

Accepted38310new uniqueUnknownbytes:15344Code/22966Data, allStatic; plus one
existingData byte$5836->Code retaining16byte suffix. UniqueCode/Data intersections,
physical/CPU aliases, effective8KiB banks, instruction operands and exact record
bounds independently verified (6702instructionviews/223decodegroups); no conflicts.
Primary review analysis/round3/structural-review.md and independent-review.json;
applied.json retains exact annotation IDs. ReadyR2-L02..08 now verified, histories
preserved in leads.json. Header16 andCHR131072remain classified. Internal total kinds:
{'Code': 42848, 'Data': 87562, 'Image': 131072, 'Text': 654}. Exact24Unknownbytes/11intervals retained in
analysis/round3/unknown-complement.json; seven source-templateEAs,$982D..$9830,
$1E00D..$1E010,$1FFEF,$20000..$2000A. Local materialambiguity review reused, no
terminal-exhaustion claim. Connected supported handlers/consumer data accepted;
no obvious readyrecipe deferred. Next fullcommentary mustreview freshsource and
preserve priorreview/history; commentary/source untouched hash9e2e25c7d996ddf316c0a6351ee5d1b7278f52437a55a6a96a5acd83b8e2fcb0.

Saved scenarios reviewed and reused: later/finalfield/password, battle/worldcommit,
production/birth, defeat/defaultending/credits/END/reset. No newruntime/patches,
no earned/naturalwinning orallpages claim. Scenario phase-review appended.
Mapvalidated/saved/exported and freshpass-003b byte-identical rebuild passed:
analysis/pass-003b-verification.json, maps/pass-003b.json, generated/pass-003b.
MapSHA256 d3932aa1bbf71f8200a2af01bac65ca0665f4f545b40d864d59dbb1805aba9e3.
InputROMSHA256 unchanged424b05432af2c0c88926373037b7e8264ed21f618ed25f0d73ee67ced9620f37.
NES-MAP-002 size-limit mismatch recorded; NES-DOC-001 remains in analysis/toolkit-issues.md.
Failed pass-003 map retained; lossless JSON whitespace compaction keeps all parsed values/evidence and permits file load. No toolkit edit/build, ROM/commentary rewrite, cleanup/delete/archive/publication,
orwhole-workspace commit. Review/apply/checkpoint processes closed; no live server.

| Physical 8KiB bank | Classified | Unknown |
|---:|---:|---:|
| 0 | 8191 | 1 |
| 1 | 8192 | 0 |
| 2 | 8191 | 1 |
| 3 | 8192 | 0 |
| 4 | 8188 | 4 |
| 5 | 8192 | 0 |
| 6 | 8191 | 1 |
| 7 | 8192 | 0 |
| 8 | 8191 | 1 |
| 9 | 8192 | 0 |
| 10 | 8191 | 1 |
| 11 | 8192 | 0 |
| 12 | 8191 | 1 |
| 13 | 8192 | 0 |
| 14 | 8189 | 3 |
| 15 | 8181 | 11 |

### Round 3 commentary accepted; redirect active

Separate commentary workers completed the same phase through a preserved continuation.
All 19,334 entries / 1,360 meaningful units reviewed, zero backlog. Final source
SHA256 d3c8aa9a6605a91465fd47e7b00407fb01dad0e57bc1988c815615e18b5db4ce passed fresh
byte-identical batch009 verification; reading and review references refreshed.
Coordinator independently matched source/report/scope and exact reading rendering.
Handoff: analysis/commentary-round3-handoff.md; assessment and scope check under
analysis/round-3/. Preserved prior source/review in commentary-pass002 and working-prior.
Twenty-seven superseded continuation claims and contextual aliases/veneer names repaired.
Accepted input/output classification checkpoint remains pass-003b, map
 d3932aa1bbf71f8200a2af01bac65ca0665f4f545b40d864d59dbb1805aba9e3,
262136 classified / 24 Unknown bytes in 11 intervals (99.99%). No classification
or ROM changes during commentary. Final editorial audit remains pending.
All commentary handles closed before separate round3_redirect launched. Redirect
is active on residual evidence and concrete current-source questions; no terminal
exhaustion decision or final handoff has been accepted. Lead outcomes remain
R2-L02..08 verified/consumed in disassembly; new redirect leads pending investigation.

### Round3 redirect complete — five-byte supported batch, accepted map unchanged

Attempt analysis/redirect/r3-residual-contracts/ completed connected signature/reset
startup and dormant ending-arm investigation plus independent current-source refs,
password index producer, reset templates, footer and third-tag-byte review. Exact
prior errors corrected by primary evidence: C048/C04F read first2 signaturebytes
DFFD/DFFE; 4C2E98 is JMP982E, not operand981E. Ready R3-L01 Data2 at1E00D,
R2-R01 reopened Code3 at982D; no new execution/return/naturalreachability assertion.
Proposed5 bytes independentreviewpending; acceptedpass003b remains262136classified,
24Unknown/11intervals,99.99%, mapd3932aa1bbf71f8200a2af01bac65ca0665f4f545b40d864d59dbb1805aba9e3.
Ifaccepted:19Unknown/10intervals. Source/currentstate/ROMhashes independentlyunchanged.
Exactprimarydecodes/peerproof, source-triage, complement/dispositions and localized
rejectionreview saved; no terminalexhaustionclaim. Source scan flags17 stale rawprefix
comments for nextfullcommentary and eventualeditorialaudit, sourceuntouched.
Saved broadgameplay milestones reviewed/reused and scenariosphase-review appended;
no runtimepatch/replay or earnednaturalcampaignclaim. No newtoolkitissues; existing
NES-DOC-001/NES-MAP-002 retained. Configuredreleasebinary only; allserversclosed,
no livehandles. Disk2.4GiBfree/scratch12KiB, no cleanup/delete/publication/commit.
Handoff analysis/redirect-round3-handoff.md; leads.json histories and pass appended.
Next: independentdisassembly bothreadyproposals, fullcurrent-sourcecommentary, redirect.

### Round4 disassembly — supported five-byte gain; incomplete

ROM classification: 99.99% (262141 / 262160 bytes classified; 19 bytes unknown).

Accepted R3-L01 Static Data2 at physical1E00D and reopened R2-R01 Static Code3 at982D, independently verified bank/consumer/peer/bounds. Source unchanged. Full review: analysis/round4/structural-review.md; exact summary/residuals: analysis/round4/summary.json; independent configured decode: analysis/round4/independent-review.json. Both leads verified; no unchanged retry warranted. Connected supported continuations were already classified;19 residualbytes/10intervals require discriminating new structure evidence in next redirect. Saved broad gameplay milestones reviewed/reused; no missing practical representative milestone or new runtime scenario. No terminal-exhaustion claim.

Fresh pass-004 validation/generation/byte-identical verification: analysis/pass-004-verification.json, maps/pass-004.json, generated/pass-004. Map SHA256 4adf3367de16326a97e0f83cd6ea6bd5cf6a6188257d341f0332c03008339d13. Fresh compact state reload/validation in analysis/round4/fresh-state-validation.json. Lossless JSON compaction preserves parsed objects under NES-MAP-002. ROM SHA256 unchanged; commentary source d3c8aa9a6605a91465fd47e7b00407fb01dad0e57bc1988c815615e18b5db4ce preserved. No new toolkit issues found; existing analysis/toolkit-issues.md NES-DOC-001/NES-MAP-002 retained. All processes closed; no live handles.

|Physical8KiB PRG bank|Classified|Unknown|
|---:|---:|---:|
|0|8191|1|
|1|8192|0|
|2|8191|1|
|3|8192|0|
|4|8191|1|
|5|8192|0|
|6|8191|1|
|7|8192|0|
|8|8191|1|
|9|8192|0|
|10|8191|1|
|11|8192|0|
|12|8191|1|
|13|8192|0|
|14|8191|1|
|15|8181|11|

Header16 and CHR131072 bytes fully classified; no trainer/trailing bytes.

Handoff storage:2.3GiB available; reusable analysis/scratch12KiB. Compact map/state 13265196 bytes each. No deletion performed.

### Round4 commentary accepted; redirect active

Current source a9278c94fc310866da79d2ba72815c21f5068cd8db1e325a32122100ef658323 independently accepted: 19,335 entries / 1,363 meaningful units, zero backlog; fresh byte-identical verification and reading-copy identity. See analysis/commentary-round4-handoff.md and analysis/round-4/commentary-readability-assessment.md. Final editorial audit remains pending.

Redirect starts from unchanged pass-004 (262141 classified /19 Unknown/10 intervals); active research analysis/redirect/r4-role-review/plan.md. Reviewing narrow structural roles using primary evidence, including dormant copied reset templates, independently from actual remapped fetch. No live runtime/server handles; no map/state/source edits.

### Round4 redirect — supported seven-byte proposal; incomplete

Research completed in analysis/redirect/r4-role-review/; compact handoff analysis/redirect-round4-handoff.md. Ready R4-L01 proposes seven copied reset-template NOP bytes as Static Code, combining all eight vector origins, fixed-bank accepted Code homolog, exact five-byte identity and immediate suffix divergence. Correct prior remap proof is preserved as execution-only negative: no source-bank EA fetch through reset. Accepted map/state/source remain pass-004 unchanged;262141classified/19Unknown/10intervals. Proposed7 is not accepted; if accepted262148classified/12Unknown/3intervals.

Independent current-source/control/generic/uncertainty triage and tag/footer/trailer role checks completed; specific competing-role insufficiencies and exact complement recorded in primary-role-proof.json, unknown-ledger.json and claim-review.md. Footer R2-L01 history extended; no blanket old-rejection veto. Saved broad gameplay milestones reviewed, no practical missing representative milestone; no unchanged replay. No terminal-exhaustion claim. Next independent disassembly, full current-source commentary, redirect. No new toolkit issues found; NES-DOC-001/NES-MAP-002 retained. No ROM/toolkit/map/state/source edits/build, cleanup/delete, publication/archive or commit; no live server handles. Free2.16GiB, scratch12KiB.

### Round5 disassembly — seven Static Code bytes accepted; incomplete

ROM classification: 100.00% (262148 / 262160 bytes classified; 12 bytes unknown).

R4-L01 verified; exact independent review analysis/round5/structural-review.md, immutable-review.json and api-log.json. No observed fetch/natural-use claim. Connected supported continuation already classified; remaining three ranges physical1E00F length1,1FFEF length1,20000 length10 retain localized role ambiguity. Saved broad gameplay evidence reviewed/reused with no new runtime. No terminal exhaustion decision.

Fresh pass-005 byte-identical checkpoint: analysis/pass-005-verification.json, maps/pass-005.json, generated/pass-005. Map SHA256 e7d6e4a1b6e7fa1bf3e42d5cd6ecfdf37d94df7758bf4f52b83129957346bc23. Fresh compact state reload analysis/round5/fresh-state-validation.json; exact counts analysis/round5/summary.json. All prior region objects preserved. Commentary source a9278c94fc310866da79d2ba72815c21f5068cd8db1e325a32122100ef658323 unchanged whole; next full commentary must incorporate seven new Code bytes and reconcile old prefix wording. No new toolkit issues found; analysis/toolkit-issues.md NES-DOC-001/NES-MAP-002 retained. All servers/process handles closed.

|Physical8KiB PRG bank|Classified|Unknown|
|---:|---:|---:|
|0|8192|0|
|1|8192|0|
|2|8192|0|
|3|8192|0|
|4|8192|0|
|5|8192|0|
|6|8192|0|
|7|8192|0|
|8|8192|0|
|9|8192|0|
|10|8192|0|
|11|8192|0|
|12|8192|0|
|13|8192|0|
|14|8191|1|
|15|8181|11|

Header16 and CHR131072 fully classified.

### Round5 commentary accepted; redirect active

Current-source commentary independently accepted: 19335 entries/1370 units, zero backlog; source c48035c080d7be3f851ea3f8b41376bed27d1c7c60a8bf7e9a9abc87775726d6. Pass-005 map e7d6e4a1b6e7fa1bf3e42d5cd6ecfdf37d94df7758bf4f52b83129957346bc23 remains unchanged, 262148 classified/12 Unknown. Active attempt analysis/redirect/r5-terminal-review/ reviews exact complement, primary residual evidence and independent source/gameplay opportunities. Correct r4 claim-review path retrieved (not nonexistent handoff subdirectory). No runtime/server handles.

### Round5 redirect — terminal incomplete assessment submitted

Completed analysis/redirect/r5-terminal-review/; handoff analysis/redirect-round5-handoff.md. Fresh immutable/current-source proof confirms exact12byte complement in3intervals,2928resolved simple direct controls and only10DFFF mapper-write hits. Strongest residual claims reviewed against retrieved primary artifacts and accepted copied-template standard; no supported proposal, new ready lead, useful revised inference/test or deferred actionable avenue. Broad saved milestones reviewed; provenance retained. Required rejection-review.md awaits coordinator primary-evidence inspection. Map/state/source/ROM unchanged; final editorial audit pending. No live handles, no new toolkit issues found, no build/cleanup/delete/archive/publication/commit.

### TERMINAL coordinator handoff — workflow complete, ROM classification incomplete

Final checkpoint pass-005; ROM classification: 100.00% (262148 / 262160 bytes
classified; 12 bytes unknown). Rounded percentage is not whole-ROM completion.
Exact remaining half-open physical intervals1E00F..1E010,1FFEF..1FFF0,20000..2000A.
Coordinator accepted primary-evidence terminal rejection review, no ready lead or
concrete useful revised test remains. See analysis/terminal-rejection-review.md and
analysis/redirect/r5-terminal-review/rejection-review.md. No blocked work is hidden.

Final editorial acceptance PASSED actual required audit, fresh byte-identical rebuild
and reading refresh.19335reviewedentries,1375distinctunits,18943unitreferences; all Code
has valid reviewed-unit links after source-inspected metadata repair. Source unchanged
c48035c080d7be3f851ea3f8b41376bed27d1c7c60a8bf7e9a9abc87775726d6; map unchanged
e7d6e4a1b6e7fa1bf3e42d5cd6ecfdf37d94df7758bf4f52b83129957346bc23. Input ROM identity
unchanged; fresh map/store validation passed. All worker/tool handles closed.

Terminal deliverable analysis/terminal-handoff.md; objective completion evidence
analysis/completion-audit.json and analysis/current-audit.json. archive-options.json
selects final conclusions/handoffs and accounts for all79scannedMarkdownfiles,
54additionaldocuments/15verifiedduplicate-scaffoldomissions. No archive executed,
no cleanup/deletion/toolkit edit/build/publication/wholeworkspacecommit. All source,
assets,state,traces and history remain local. Resume only for new material evidence
or a genuinely different justified method; preserve this sound terminal review.
