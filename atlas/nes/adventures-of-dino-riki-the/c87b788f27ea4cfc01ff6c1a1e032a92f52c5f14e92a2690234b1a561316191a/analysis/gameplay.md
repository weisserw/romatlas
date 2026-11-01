# Gameplay familiarization — round 001

Locally confirmed title: Adventures of Dino Riki, ©1989 Hudson Soft, Nintendo
license screen. NTSC CNROM image; exact commercial revision not independently named.
Natural Start enters stage1-1; D-pad movement and held B produces projectiles;
A used with movement in representative jumping experiments.

Sources consulted 2026-10-31 (external descriptions are hypotheses):
- https://gamefaqs.gamespot.com/nes/587071-adventures-of-dino-riki/cheats
  Up plus Start (any D-pad per this source) continues after game over. Not yet tested.
- https://github-wiki-see.page/m/RetroAchievements/guides/wiki/Adventures-of-Dino-Riki,-The-(NES)
  Four stages, final stage has four substages; first three bosses repeat, final
  hornet boss. Reports return to first stage after clear, with no ending screen.
  Locally confirmed distinct land/desert/swamp, Pterodactyl/T-Rex/Cobra/Hornet
  graphics and later stage variants through artificial selector initialization.
- https://www.thegameisafootarcade.com/wp-content/uploads/2017/02/Adventures-of-Dino-Riki-Game-Manual.pdf
  Manual available; PDF text unavailable in browser extraction. No assets downloaded.

Source contract: physical $04DC clears selector $48 for a fresh game; $050B
(CPU$84FB) loads it and initializes stage tables. $06C3 increments selector;
$0E wraps to0 and increments $C0. Table at CPU$882C has alternating nonzero
exploration and zero boss entries. This concrete contract permits bounded later
state experiments. See scenarios.json for exact replay and patch routes.
