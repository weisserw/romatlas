# Alien 3 NES familiarization — round 1

Confirmed locally by copyright/ALIEN3 title screens; region/revision not independently identified.
NES manual: https://www.gamingalexandria.com/highquality/NES/Alien%203/Alien%203%20-%20Manual%20(Searchable).pdf
Manual hypotheses: Start at title opens options; Start item begins game. Difficulty and
music/sound samples available. Directions move/climb/use doors and elevators; Select
changes weapon; Start pauses. Stages require rescuing prisoners under a timer. Radar
requires batteries; four weapon types and overheating mechanic. Hold directional
buttons for continuous movement. Face-hugger escape uses alternating left/right.

Shortcut hypothesis from https://www.gamemite.com/games/alien-3/cheats/ :
NES paused level skip Left Right Up Down Left Right A B A B. Untested as of title.
SNES passwords and Genesis codes on that page do NOT apply. No ROM assets downloaded.

Startup has lengthy unskippable-looking logos/title animation; Start pulses at
frames 160 and 970 did not immediately open a menu. Frame 1692 full title observed;
subsequent Start entered a black transition/wait state. Continue inspection.

Controlled menu confirmed at frame2642. Corrected pause shortcut confirmed by $A9=10
and subsequent stage $A8 increment 0->1. Each input must remain held through the
5-frame debounce in bank13 CPU B3F0; tested 8 frames held + 8 released, after pause
10-frame debounce. Two-frame taps fail. Table B437..B440 has ten masks matching
source guide. $A8 is zero-based stage sequence; fixed code physical1E291 compares
it with12 for terminal flow. Alternate regular/guardian progression needs runtime
confirmation. A clean controlled-pause snapshot permits stage-index shortcuts.

Broad progression confirmed artificially via supported initialization: stage index2
first guardian,8 later guardian,9 later normal map,11 final guardian area. Short
movement/fire/jump and weapon selection runs exercised distinct states, not just RAM
writes. Indices2/8/11 are guardian milestones inferred from every-third-stage pattern;
first guardian boss visible, later guardian boss contact not established.

Ending warning discovered locally: pause cheat bypasses terminal check and stage12
initialization is invalid. Normal completion tail CPU E267 (physical1E277) includes
bonus accounting, increments stage and tests12. Artificial entry at E267 with
$A8=11, stackFF,p20, unpaused$64=0 and delay$52=0 reaches WELL DONE bonus followed
by Ripley escape ending. Earlier p24 and paused flag attempts stalled; preserved.
No ordinary final exit route or final guardian victory established.
