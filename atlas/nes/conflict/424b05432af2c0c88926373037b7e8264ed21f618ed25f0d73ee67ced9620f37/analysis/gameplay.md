# Gameplay research and observations

ROM provisionally identified as **Conflict (NES, Vic Tokai)**: military equipment,
tactical action strings and partial CONFLICT logo in analysis/round1/after-wait.png.
Region/revision not yet independently confirmed; immutable ROM SHA-256 in workspace.json.

Primary manual search: https://www.gamingalexandria.com/highquality/NES/Conflict/Conflict%20-%20Manual.pdf
Search excerpt establishes D-pad cursor, A selection, B twice exits selection;
production through factory picture, and restrictions after all units moved.
Full PDF fetch failed because 36.8MB exceeds browsing fetch limit; no ROM/assets downloaded.
Secondary game outline: https://www.gamesdatabase.org/game/nintendo-nes/conflict
Reports 16 scenarios, strategic hex map plus tactical battle module, auto battle,
final scenario password accumulated across earlier maps. These are external hypotheses,
not structural evidence. Next experiments: title/menu, scenario selection, tactical battle,
later scenario via menu/password or RAM stage contract, and terminal victory/defeat.

Observed natural boot: reset $C000, NMI/IRQ $C01B. Initial request timed out after32
frames; subsequent boot replay reached79 and durable frontier2 snapshot reached342.
At342 partial logo visible. Trace-disabled wait to342 retained hardware state but
provides no fetch observations for that interval. Boot trace aggregates saved separately.
Button sequence Start press2 frames/release20, A press2/release20 is being tested.
No later gameplay milestone reached yet; no RAM patches made yet.

Full title screenshot title-finish.png now confirms Conflict ©1989 Vic Tokai Inc.
Corrected NES input schema is {port:1,buttons:["start"]}, release buttons:[].
Earlier generic button/pressed calls silently applied empty input; see NES-DOC-001.
Start accepted after deliberate15-frame hold: start-held.snapshot RAM $003E=1,
$0037=67 (title delay counter). Static $86F1-$870D branch sets accepted flag and
increments37 until$90 before state05 becomes3; waiting through this supported
transition is required. This is a concrete timing contract, no RAM patch needed.

Natural menu sequence confirmed: title -> 1/2 player +3/all units -> Fame Blue
Level1/2/3 -> map1-8 selection. Default A choices yield map selection by frame1268.
Primary manual search excerpt confirms scenarios1-15 selectable; scenario16 reserved
for one-player password access. Final map password hypothesis from
https://gamefaqs.gamespot.com/nes/587197-conflict/cheats : HEXASOURYOKUSEN (default
level uncertain); alternate level2/3 codes exist. Prefer menu/RAM inspected contract
for targeted later maps and final map; do not brute-force campaign victories.

Map selection $8EB8-$8F40 has a second deliberate delay: accepted flag$3E and
counter$41 advance to$C0, then copy selected zero-based map$34 to$0B and set
state$05=$FF for ordinary map initialization (map15 index triggers password state5).
A3-frame input did work: map1-enter snapshot has$3E=1,$41=33,$34=0, state05=4.
A later15-frame hold was redundant; screenshots before192-frame delay still show
selector. Use wait or patch this *identified* delay rather than guessing input.

Field command text printing blocks/consumes inputs for many frames; do not repeatedly
press while messages are still being drawn. Natural post-intro cursor-left visibly
moved hand, but12-frame hold traversed beyond intended x6,y20 tile. Snapshot
map1-unit-menu still shows no selected unit/mirror. Targeted artificial cursor
patch04=6,05=20 from clean idle snapshot now tests observed unit record ID6 at that
coordinate; exact edits and actual accepted fetches retained separately.
Manual unit-functions search excerpt confirms Start toggles selected-unit information,
and MOVE command then direction/A commits movement. Production requires factory.
Source URL remains same primary manual. Battle/auto-battle and terminal victory
remain unvisited; no unverified routine entry or arbitrary health flag patch used.

Tactical shortcut succeeded: battle-setup-records.json confirms$6FFE/FFF=[133,131],
$7000 blue[6,22,8,87,248,0,16,129],$7008 red[101,23,7,164,240,2,32,129].
Ordinary combat initialization switched PRG context[0,1,14,15], then battle-ready.png
shows BLUE AH-1S versus RED INFANTRYMEN, both displayed strength15, landscape/units,
command areas and enemy-found message. The ID6 infantry portrait guess was wrong;
use observed AH-1S label. Entire state remains artificial via attack/cursor shortcut.
Tactical A6/release15 now being exercised after180-frame untraced constructor wait.

Corrected combat route retained selected unit world coordinates and strength1/ammo14;
BLUE M1A1 vs RED T80 was visually confirmed. Combat damaged T80, it attempted retreat
and was caught; later M1A1 reached0 and BLUE HAS BEEN DESTROYED. World return was
verified: count16->15, blue M1A1 absent, redT80 HP/ammo5D remains. No campaign
terminal occurred; returned to map unit selection. These are candidate objective tanks
only. Special coordinate bit40 and M1A1/T80 labels do not independently establish
flag eligibility, and earlier unqualified flag-tank wording is corrected. Objective
metadata/turn-loss checks remain research, not a reason to invent terminal flags.
Original relocated-mirror fights reached valid tactical displays/results but failed
world commit atF3D2 because result coords did not match world coords. Preserve them
as narrower tactical evidence, not successful field-return routes.

## Redirect round1 confirmed routes

An additional [Japanese manual transcription](https://setsumei.cloudfree.jp/famicom/conflict/conflict.html) describes three versus all units per turn, two factory kinds and an H-marked command tank. Its regional applicability remains a hypothesis. Local source independently proves the3/16 quota and production mechanism; the H mark alone is not the eligibility proof.

Actual password comparator accepted its ROM-selected key after an explicit buffer/latch edit. The final field expansion exactly matches the older bypass for960 bytes. Factory selection was reached with ordinary inputs after an artificial cursor/cache warp and proper parent refresh; production queued a descriptor and a source-backed artificial turn boundary inserted it into the world. Corrected saved battle tile codes plus the retained dead result reached the defeat screen. A separate explicit outcome-state shortcut visited the final ending, staff credits and END/restart, with intro/page timer skips. These are artificial scenario visits, not earned victories. See redirect/r1-static-continuations/summary.json and the new R1-RD scenarios.
