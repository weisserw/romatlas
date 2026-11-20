# Blades of Steel — round 1 familiarization

ROM title screen confirms Konami Blades of Steel 1988, North American Nintendo license.
Manual transcription: https://www.world-of-nintendo.com/manuals/nes/blades_of_steel.shtml
(NES-VS-USA; applies to visible menus). Its practical instructions were used as
hypotheses: Select toggles player count/mode/difficulty/team, A confirms, Start
enters from title; offense A shoots/B passes; faceoff B; defense B changes player.
Manual says repeated checks can yield fights/penalties, and tied final scores yield
five-shot overtime then sudden death. Tournament requires three wins to cup (other
sources still hypotheses). Manual menus have not all been visually confirmed:
round 1 presses proceeded through brief transition screens and reached NY/Chicago match.

External cheat compilation, hypotheses only until local checks:
https://etherealgames.com/nes/b/blades-of-steel/preformatted-cht-file-action-replay-game-genie-us/
Lists score RAM 07E2..07E9, timer Game Genie GEUGTTYA, ending YASGITLA.
No codes applied to ROM. Decoding timer address C6B6 leads to actual local routine
C6AD..C6DF: checks RAM 6C, decrements 63 prescaler, decimal digit countdown C0..C3.
Matched visible 19:25 countdown with RAM [1,9,2,3] (a small time advance between captures).
RAM-only [C0..C3]=[0,0,0,1], [63]=0 successfully ended first period and displayed
scoreboard; actual observed fetches after this patch are artificial, not natural route proof.

Milestones: natural faceoff/hockey visited; patched first period ending/scoreboard
visited; later periods, intermissions, overtime, tournament cup, fights/penalties
remain in progress. Replay log analysis/round01/rpc.jsonl includes all input presses,
releases, execution budgets and state patches; long waits often run without trace,
short transition slices are traced. Durable clean snapshots match.snapshot and
played.snapshot allow branching without needing a title replay.

## Reachability correction during same pass

State machine plus repetition show earlier match/played snapshots are attract/demo
(scene RAM18=2); directional/A/B inputs were supplied but player control was not
proved. Earlier 'natural hockey' claim narrows to naturally reached attract hockey.
The RAM countdown patch itself was successful but demo then returns to title instead
of proving later competitive periods. Period2/third filenames are historical labels,
not evidence those milestones were actually reached. Reset to stable input-ready
title (RAM18=1) and capture actual configuration menus before further progression.
All instruction fetch classification remains valid; only gameplay interpretation changes.

## Round 2 saved-evidence review

Round1 redirect corrected the timer/period interpretation using counter34=0->1->2
and retained real later-period/intermission routes. Its exact artificial patches
and controls are in analysis/scenarios.json and redirect/r01-structural-contracts/.
Second/third live periods, arcadeNICE SHOT, fightpower response and tiedgame shootout
inputs were visited; natural triggering/completedfight/shot unproved. Prior artificial
scene7 ending remains valid; tournament wins unvisited. Round2 reused these routes
without unchanged replay; no new gameplay or naturalreachability claim.
