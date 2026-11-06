# Gameplay research

No research performed yet. Follow workflow.md's Game familiarization section. Record source URLs, version uncertainty, actionable routes and local confirmation separately.

## Round 001 familiarization
Local startup title confirms ASTYANAX (NES mapper4, exact region/revision not yet identified). External cheats are hypotheses until tested on this image.
Source https://www.gamemite.com/games/astyanax/cheats/ (opened 2026-11-06; page updated 2023-08-16), plus https://gamefaqs.gamespot.com/nes/587097-astyanax/cheats search excerpt (page fetch restricted). GameMite gives title stage select Up Down Left Right B B B B Start; demo select Up Up Up Down Down Down A B Start; inside demo select ending sequence B B A A B A Up Down Start. Invincibility Up Up Up Up Down Left Right Up Start. Use press/release edges and inspect acceptance. Alternative sources differ on initial Up, so code acceptance must be observed. No Game Genie/ROM patches used.

Confirmed stage selector uses Select (RAM $03E4 index0..9), A to enter; Dpad/Start do not select. Stage2-1 Redroad gameplay confirmed. Demo selector and ending cheat confirmed natural input route; ending dialogue visible, credits pending. Stage6 shortcut patches known selector RAM, artificial status must remain explicit.
