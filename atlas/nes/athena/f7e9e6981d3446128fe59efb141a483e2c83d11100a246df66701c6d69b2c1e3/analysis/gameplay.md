# Athena familiarization — round 1
Local title screen reads Athena, SNK 1987 (Micronics credit), NES mapper 2. Region/revision not independently established.

Sources consulted 2026-11-06:
- https://raw.githubusercontent.com/FrodeSolheim/cheats/master/nes.cht Athena (USA) section suggests health RAM $95=$0F, lives $9B=$02, time $92=$3B. These are experiment hypotheses, not structural proof. Health/time one-shot patch tested; sustaining invulnerability is unverified.
- https://strategywiki.org/wiki/Athena/Walkthrough search excerpt describes forest and lower routes, but direct page returned 403. No precise route adopted.
- https://www.gamingalexandria.com/highquality/NES/Athena/Athena%20-%20Manual%20%28Searchable%29.pdf browser rejected large response; no manual downloaded.

Tested: title Start press/release enters initialization. Right/A/B and Right/B exercise action input; initial loading takes hundreds of frames. Natural frame716 capture shows a tile scene; the player/normal active phase was not independently confirmed. Later stage experiments change graphics after RAM $9C patch with PC=$E73C; these are artificial initialization attempts, not demonstrated successful gameplay progression. See scenarios.json.

RAM $9C has strong local level-selection evidence: $E604 indexes graphics selector $BE27,Y; $E629 doubles it to select setup pointers $BE0B/$BE0C; $C8CE computes 14-byte configuration index; $E6A3 compares it to 7 for terminal routing. RAM $94 is a separate initialization/play-mode flag, not the level. Initial state has $94=0 and player spawn logic $F200 eventually sets it to $FF at $F267. Need resolve this concurrent task initialization before claiming later stages playable.

## Round 2 evidence carry-forward

The scene-only/missing-player limitation above was resolved in round1 redirect by complete task04 E460/stack47 initialization with94=1. Initialized active stages0/2/4/6/13, composite boss37 and artificial post-boss throne chamber are documented in scenarios.json and analysis/redirect/r01-connected/handoff.md. Round2 independently reviewed their source/stack/bank contracts and reused saved evidence without unchanged replay. These artificial routes do not establish natural progression, defeat or credits. No separate credits transition is supported beyond the stage13/world0B terminal yield loop; no arbitrary flag sweep performed.
