# Gameplay research

Game identified from title screen and ROM strings: **Bigfoot** (NES, Acclaim 1990, NTSC; mapper 1). Version/revision not otherwise confirmed.

## Sources (hypotheses until confirmed locally)
- https://en.wikipedia.org/wiki/Bigfoot_(video_game) — LA-to-New-York tour; events car crushing, mud bog, tractor pull, hill climb, drag race; cross-country race between stops collects money/points; CPU opponents The Growler (green), The Charger (yellow); 2P truck red. No control details.
- Search results (nesguide.com, myvideogamelist.com) add only "qualifying" top-down race and parts purchasing. No button mapping found.

## Locally confirmed (round 1, debugger input via `emulator.input {buttons:[...],port:1}`)
- Title: START (after fade-in, ~frame 400) -> ONE PLAYER / TWO PLAYERS menu (up/down, START) -> "START BIGFOOT RALLY" screen (START) -> USA map showing next stop (wait ~150 frames, then A) -> top-down cross-country race.
- Cross-country race: rapid A tapping accelerates (holding A does not); left/right steer; B appears to be nitro. Race lasts roughly 3000+ frames of mashing; ends at result screen then parts shop.
- Shop ("NEXT EVENT: CAR CRUSH"): up/down select part, A buys, START continues; CPU opponent then shops automatically (slow, ~800 frames).
- Side-view events (car crush, mud race, hill climb, tractor pull, drag race): alternate LEFT/RIGHT rapidly to accelerate (A alone only revs). Events have heats ("HEAT ONE", "HEAT TWO"). "ENGINE" damage message appears.
- RAM: $0426/$0427 joypad 1/2 raw bits; $041D = tour stop index (0 Mesa car crunch, 1 Reno mud race, 2 Yakima hill climb, 3 Fargo tractor pull, ... 8 Oyster Bay drag race; names table pointers at bank 9 $B3A7); $0423 nonzero together with $041D=8 makes bank 9 $A9E3 call the championship routine $AF03.
- Shortcuts used: write $041D at the shop snapshot then START repeatedly -> the chosen event; write $041D=8,$0423=1 during car-crush heat two -> results, championship text, ending picture, GAME OVER.
