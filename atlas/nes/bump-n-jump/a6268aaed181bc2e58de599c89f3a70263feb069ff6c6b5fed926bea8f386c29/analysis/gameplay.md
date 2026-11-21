# Gameplay familiarization — Bump 'n' Jump, NES

Title verified locally in pass-001/title.png: Vic Tokai 1988, licensed from Data East.
Mapper 3 fixed 32 KiB PRG, four 8 KiB CHR banks. USA attribution is likely from
title/licensing, not independently matched to a revision database.

External research (2026-11-21), hypotheses until tested locally:
- [Transcribed instruction manual](https://www.world-of-nintendo.com/manuals/nes/bump_n_jump.shtml):
  A jumps, B brakes; directions steer or change speed; Start begins/pauses.
- [StrategyWiki NES version](https://strategywiki.org/wiki/Bump_%27n%27_Jump/Versions):
  sixteen courses with several environments, fuel pickups, road hazards, and a
  pit pickup that can bypass much of a course. Exact continue input was omitted
  by the retrieved page renderer and is not used as a recipe.
- [NES ending FAQ](https://www.neoseeker.com/nes-hardware/faqs/139429-nes-game-endings.html):
  final course has a boss and rescue ending followed by a restart. Untested boss
  route; actionable hypothesis is stage index 15 and ending initialization state21.

Local controls/routes: no buttons 120 frames gives title; Start 2, released120
frames gives Player Start; Right+A180 gives first seaside pre-race sequence.
RAM $14 is main state; $15/$16 state-local counters; $56 zero-based course index.
$3d controls attract/play mode. Stage initialization sequence6→7→8→9 loads stage
CHR and level data. Artificial indices7/15 produced distinct course scenery.
State11 pre-race can be shortened by $a9=$f0 and $15=$30; then bounded up/A/B
sequences enter state12 driving. See scenarios.json and pass-001 artifacts.
A press alone at 80 speed need not prove a successful jump; jump threshold and
fuel/finish/boss contracts remain redirect questions. No downloaded ROM/assets.

## Redirect round 1 local contracts (2026-11-21)

Final course route D3A0(file53B0) contains FF atindex63, then final chunk08; cursor62 plus chunkoffset0C and clear roadflags lets normal AdvanceRoadStream consume the finish marker. From initialized stage15 snapshot this reached main13/opponent update in4frames; opponent movement/contact captured. Auxiliary state2→3..6→main21 is the defeat continuation; direct state2/counter patches tested it artificially. A naive playerstate1 overlap patch clears tostate0 before the auxiliary gate, so it does not prove a tenth hit or natural defeat.

Ending selector0500 follows0→1→2→3; text index0501 visits21records, with a separate128frame X-loop atindex0B. Accelerated position/timer changes and a documented X-loop shortening displayed all21records and returned through main4 to main5/course0, PLAYER START verified in ending-return-verified.png. Long unedited-after-restore ending replay was interrupted; no ordinary-timing full ending claim. Main22 initialized rescue, actual opponent visit, defeat continuation and full accelerated text/return are distinct tests. See analysis/redirect/round-1/runtime-results.md and scenarios.json.

Jump eligibility is speed-derived playerstate40bit; maxspeed RAMD8=7F then an A edge selected state5 and enlarged airborne sprite, with lowstate5→6→7 executed. Airborne mechanic visited; landing and directed pit/fuel/truck interactions remain untested. These unexplored permutations have no specific remaining unknown structure after the current static proposal; no ROM/assets downloads or compatibility claim.
