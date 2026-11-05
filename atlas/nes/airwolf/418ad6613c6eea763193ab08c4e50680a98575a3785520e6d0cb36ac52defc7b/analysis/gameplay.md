# Gameplay research

No research performed yet. Follow workflow.md's Game familiarization section. Record source URLs, version uncertainty, actionable routes and local confirmation separately.

## Round 1 — Airwolf (NES), 2026-11-05

Local licensing screen identifies AIRWOLF, Acclaim 1988, programming 1987; exact regional/revision identity remains unresolved beyond workspace SHA256. Local NTSC emulator runs mapper1 with effective PRG banks [0,1,2,3].

Sources consulted (guidance only, not classification evidence):
- https://www.gamingalexandria.com/highquality/NES/Airwolf/Airwolf%20-%20Manual.pdf — manual search excerpt: Start begins game and increases speed; Select decreases speed; directional controls altitude/heading. Rescue all prisoners before next level. Applicable NES version, exact region not established.
- https://gamefaqs.gamespot.com/nes/587082-airwolf/faqs/42411 — American NES guide: A machine gun, B missiles; rescue prisoners then exit map border; separate landing view. Controls confirmed by cockpit input experiments, rescue/exit route unconfirmed.

Local routes: `analysis/round-001/probe2.py` runs 60 frames after reset; `play.py` advances licensing/title and enters story; `continue-play.py` and `advance.py` advance alternating story/map panels into cockpit. Early frames called landing in filenames actually show story interstitials, not landing gameplay. `cockpit.py` tests A/B weapons, Start speed and Up/Right flight; cockpit captures confirm speed/altitude/heading change. All inputs/ticks/snapshots persisted. Untraced waits intentionally provide no execution evidence.

Progression shortcut from local control flow (`progression-decode.json`): $80 mission index, $E2 additional mission-number offset; CPU $CE65 increments index; comparison with $14 at $CE7B selects completion entry $CE85. CD39 initializes mission. Artificial experiment restores cockpit-flight snapshot, steps one operation to a boundary, writes RAM $80=19, sets PC=$CD39 and stack=$FF; alternative $80=20 PC=$CE85 stack=$FF. No ROM patches. These top-level jump targets are backed by the stage loop; D37D initializes local stage variables and needs no input registers. Persistent cockpit RAM supplies other preconditions. This does not prove full original entry-state equivalence. Late mission map and completion text observed; graphics disturbance in ending capture may reflect entering outside natural presentation state and must not be treated as a toolkit fault. Follow-up captures assess stability.

Remaining practical research: determine exact prisoner/base coordinates, collision/landing trigger and rescue/exit state contract from the mission initializer / $AE comparison with $C219[$80]. Source indicates a 20-entry mission cycle then stage10 with $E2 +=10; no claim that a guide's nominal mission total exactly matches this ROM. Later missions and terminal display are artificial-state exploration, not ordinary gameplay reachability proof.

## Round 001 redirect — service and real return contracts

Local source and bounded experiments now confirm the separate landing view and successful rescue/base mechanics. Type $10 map-cell service passes caller Y=0; type $20 uses Y=1. Artificial source-aligned setup entries $84DF/$85A9 display the side-view landing system. From saved service snapshots, setting position Y=$B8, pose below 4, X inside the landing interval ($27 <= X < $31) and vertical-speed high byte below $12 produces success. The selected tests set $03/$04 to 37/184 (rescue) or 127/184 (base), $12=0 and $01A5:$01A6=0, then run 160 frames with no input. Rescue increments AE and CD:CE; base increments CB:CC. Source and scripts are under analysis/redirect/r001-contracts/. These artificial edits do not prove the unpatched landing trigger or route.

The mission-return question is resolved: $8003 initializes cockpit graphics and returns; the main $8276 loop returns A=2 through barrier-response $86DA when AE reaches `$C219[mission]`, or after four premature exit warnings. A clean saved cockpit reaches $8456 naturally; controlled RAM-only map/cell/heading/objective edits then execute the real return to $CE12 without PC/stack edits. The source's auxiliary $98 path belongs to attract/demo mode, not the normal success condition.

Using that contract on the existing artificially initialized mission-19 snapshot, objective 7 and a supported barrier event return through the actual caller, advance mission to 20 and display a clean thank-you ending after 300 frames. `analysis/redirect/r001-contracts/ending-route.png` and its script/state records supersede the earlier direct-PC experiment's presentation limitation for this artificial scenario. Natural unpatched reachability and full game/hardware fidelity remain unproven. No extra online research or downloads were needed for these local source-backed tests.
