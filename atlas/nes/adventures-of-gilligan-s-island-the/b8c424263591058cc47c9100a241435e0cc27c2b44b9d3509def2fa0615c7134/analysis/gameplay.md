# Gameplay research

No research performed yet. Follow workflow.md's Game familiarization section. Record source URLs, version uncertainty, actionable routes and local confirmation separately.

## Round 01 familiarization
Local startup shows BANDAI splash and GILLIGAN character portrait; likely The Adventures of Gilligan’s Island (NES US; hash in workspace.json). Source https://gamefaqs.gamespot.com/nes/587305-the-adventures-of-gilligans-island/faqs/61684 describes four episodes, passwords LJJGDMPC / MIIJCDAD / PKKCNMPD for 2/3/4; these are hypotheses pending ROM confirmation. https://www.world-of-nintendo.com/manuals/nes/adventures_giligan_island.shtml gives start twice, A to advance dialogue. Another password site gives conflicting codes, so use walkthrough/manual and verify locally. Ending follows Evil Curse boss, club attacks; later exploration should use password UI or supported RAM episode transition.

GameHacking source https://gamehacking.org/game/28896 lists episode modifier RAM0081 (00–03), health007E=08, time007C=63 /007B=3D, and weapon039A (value domain unspecified). Episode81 was independently confirmed0/1/2/3 after accepted password transitions. Health/time are sourced hypotheses until patched and observed. No ROM/Game Genie patches used. https://www.neoseeker.com/nes-hardware/faqs/139429-nes-game-endings.html says episode4 final skeleton Evil Curse drops Stone Statue; picking it up leads to final dialogue, ice-cream island, episode-clear and staff credits. No live ending reached yet.
