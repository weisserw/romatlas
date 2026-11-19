# Gameplay research — round 01

Local title identifies The Battle of Olympus; NES English screen, mapper 1, 128 KiB PRG. Exact release/revision not established. External descriptions remain hypotheses until tested.

[Manual transcription](https://www.world-of-nintendo.com/manuals/nes/battle_of_olympus.shtml), consulted 2026-11-19: A jumps; B attacks/talks; Start opens inventory; Select uses selected special item. New game requires two six-character names then Start at the regional map. Death offers Continue (area start, fewer olives) or Retry (last temple). Progression includes Arcadia, Attica, Argolis, Peloponnesus, Laconia, Crete, Phrygia, Phthia and Tartarus. Weapons, temple conversations, travel instruments, ceiling-jump sandals and bosses provide representative mechanics to visit. Applicability: English NES, pending local control confirmation.

[Data Crystal RAM hypotheses](https://datacrystal.tcrf.net/wiki/The_Battle_of_Olympus_(NES)/RAM_map), search-index content retrieved 2026-11-19 (page itself robots restricted): $4A current health, $DF map location, $EF maximum health, $F0-$F4 quest flags. $FD brackets mapper changes; $FE/$FF hold PPU settings. Version unspecified. Use local consumers to establish actual contracts before progression patches; external addresses alone do not justify annotations.

[Password compilation](https://retromaggedon.com/index.php/the-battle-of-olympus-passwords-nes/), consulted 2026-11-19: Continue menu enters passwords, A selects symbols. Final all-items password reported `6!8SQW RjNOmKB 3fmrzC 5WFz0Uk`, starts Arcadia; intermediate Laconia and Phthia passwords also available. These are untested; compilation warns some historical passwords may have localization differences. Useful alternative if local RAM transition contract is difficult.

Local startup: title scrolls slowly; Start before menu readiness does not advance. At frame 349 artificial $02=1 attempted to shorten frame-delay routine ($D40F-$D418), but twenty-frame observation merely continued title animation. No gameplay reached yet. See evidence captures and forthcoming scenarios.json.

## Round02 local evidence reconciliation

The startup-only note above is historical. Existing successful routes in scenarios.json reached gameplay, representative later areas, inventory, special-room/NPC dialogue and artificial rescue/final couple scenes. Round02 reused these artifacts without replay. NPC dialogue displayed the Arcadia/Zeus sentence and returned game_mode0; the earlier checklist attempted milestone is superseded for conversation (password entry remains untested). Source EF54 loops EF33; natural boss defeat and full terminal fade/text sequence remain unresolved. See redirect/round02-a01/review.md and scenarios.json progression_reviews. No new external factual claims or ROM assets retrieved.
