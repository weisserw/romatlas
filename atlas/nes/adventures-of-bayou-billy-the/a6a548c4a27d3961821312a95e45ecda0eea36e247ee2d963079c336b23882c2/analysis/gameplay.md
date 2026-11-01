# Game familiarization
Title frame confirms The Adventures of Bayou Billy (Konami 1989), international NES.
Manual transcription: https://www.world-of-nintendo.com/manuals/nes/bayou_billy.shtml
Manual control/progression hypotheses: Select cycles title options; Game B uses
controller aim for shooting. A kick/B punch/A+B jump kick. Shooting A fire with pad
aim; driving up/down speed, left/right steering, A dynamite/B gun. Practice offers
all three mechanics. Nine stages: brawl 1/3/6/8/9, shooting 2/7, driving 4/5.
No ROM/extracted assets downloaded. Title confirmed; controls/milestones require
local confirmation. Prior down input was an incorrect menu hypothesis, superseded
by documented Select. Practice is promising supported shortcut to mechanics.

## Round 01 redirect: locally confirmed ending shortcut
The saved ending dialogue does not require a long input trial to reach credits.
Restore `analysis/evidence/round01/ending.snapshot`, release buttons, set RAM
$34=5/$35=10, clear $40..$48 and $55, and run bounded 20-frame chunks. The supported
ending state-10 initializer advances to the credits roll; a moving staff/PROGRAM
screen was captured at 1800 frames. From the resulting credits snapshot, setting
$44=0/$46=104/$47=0 selects the independently identified credits terminator and
reaches THE END after 200 frames. Start for one frame, then release for 20, exits
to the Stage 1 controller screen. These are artificial progression shortcuts;
natural boss defeat and natural ending reachability remain unproved. Exact local
recipes, snapshots and selected captures are in `analysis/redirect/round01-20261031-a/`
and `analysis/scenarios.json`. No additional external guide lookup was needed.
