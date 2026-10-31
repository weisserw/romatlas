# DragonStrike NES familiarization

ROM visual title confirms DragonStrike; mapper 4, revision/region not independently identified. Initial logo persists over 214 frames. A 900-frame no-button continuation from frame 214 entered overhead dragon combat by frame ~1103. Whether attract demo or full player game remains unresolved: RAM $A2=$80 and no HUD except a left health bar suggest a demo flag; those early button calls were ignored (NES-001); all early motion was attract playback.

Search 2026-10-30: https://gamefaqs.gamespot.com/nes/563407-advanced-dungeons-and-dragons-dragonstrike/cheats search snippet supplies level 14 Abyss bronze password FKHMPQ. https://gamefaqs.gamespot.com/nes/563407-advanced-dungeons-and-dragons-dragonstrike/faqs/38110 and https://www.neoseeker.com/advanced-dungeons-dragons-dragonstrike/faqs/255047-dragonstrike.html identify controls/progression guides, but direct retrieval failed (restricted / 403). The FKHMPQ password was subsequently confirmed locally through RAM injection into the built-in password buffer; other source claims need local confirmation. https://1upheart.com/nes/advanced-dungeons-dragons-dragonstrike/ search snippet describes 14 overhead shooter missions, briefing/objectives, bosses and password continuation. No ROM/assets downloaded.

Local tested replay recipes, snapshots and captures are in analysis/round-001. Later-level shortcut contract under investigation; do not treat FKHMPQ or a RAM stage write as a successful visit without frame/execution evidence.

## Confirmed round 001 routes and RAM contracts

Correct NES input is a complete held-button array, e.g. `{buttons:["start"],port:1}`; releases use `buttons:[]`. All pre-menu-real single-button calls were ignored, so earlier combat was attract playback, not player control.

Natural valid controls passed the logo/title, player-count/password menu, bronze dragon selection, easy difficulty, world map and first briefing into first-mission combat. A+Up and B+Right were exercised with real fetch coverage; detailed gameplay meaning remains under review.

Built-in password screen: from password-menu.snapshot use Down2 / release2 / Start2 / release60, then allow scrolling to finish (password14.snapshot is the stable reusable seed; it carries an earlier ineffective artificial patch). Buffer is $0106..$010B with zero-based ASCII letter indices. Cursor $0114=$1C selects END, then A3/release. Confirmed bronze passwords FKHMPQ (14/Abyss), FMHMSD (8), FMHMRB (3), FKHMKJ (5), FMHMVF (11). Exact recipes are named JSON files in round-001. These visits are artificial because the buffer was edited; actual subsequent CPU fetches are observed.

Mission 14 enters internal scene $15 and banks14/15. Supported successful-scene continuation is $F503: expects one disposable frame marker on stack and carry set from completion predicate. After saving a snapshot and finishing one operation, set PC=$F503, SP=$FE, P=$25, and $01FF=0. This pops marker, increments scene with ADC #0/carry, switches to menu/transition bank and follows ordinary initialization. This exposes scene $16 Queen boss, victory text scene $17, post-victory home flight in banks18/19, and eventually returns to logo after advancing to $18. Victory text is confirmed in queen.png. No separate credits screen was demonstrated. This tests artificial progression, not legitimate mission completion.

A preliminary objective shortcut ($D6=0,$02F9=0,$05E2/$05E3=0, health $05E7=$2F) did not complete the stage: enemy count $D6 recomputed to39 and position/state changed through ordinary update. Do not repeat this patch without understanding object bookkeeping; use the supported success-continuation recipe for representative progression.

Source refresh: the GameFAQs search result returned its full password list and documented sound-test directions / Japanese-mode input sequence. Local password confirmation supersedes initial retrieval limits for the tested passwords. Sound test remains untested; language flag shortcut is being checked separately. Health-address suggestion $05E7=$2F independently appears in https://github.com/FrodeSolheim/cheats/blob/master/nes.cht; local patches establish only changed RAM, not general invulnerability.

All routes preserve input/game.nes. Snapshots are local same-build ROM-bearing evidence and must not be shared. Executed-routes.json indexes actual RPC results, including partial timeout/budget stops; use saved snapshots and recorded actual cycles rather than assuming every requested frame budget completed.


## Round001 redirect: ending/keyboard confirmations
Local source plus bounded experiments establish the four-direction title gate is a two-row music/effect sound test and the twelve-direction gate toggles language bit0 then warm resets; natural sequence activation was not replayed. See /root/romatlas-batch/6a63874d732c02578966ac1db491e8b8d5ef36016d0b2f99df9429bc3383419b/workspace/analysis/redirect/round001-20261030-a/resolved-behaviors.md.
Ordinary idle continuation from saved artificial-route ending-home state visibly scrolls designer/art/music/sound/special-thanks credits. Recorded300/600-frame calls through4200 frames, no new RAM/register writes; trace-disabled workaround avoids15s traced-call timeout. Final event cursor42 consumes21 credit records, but a distinct visible THE END capture remains unverified. Earlier skip-to-next-scene route did not demonstrate credits. Selected captures/checkpoints and exact routes are linked in scenarios.json.
Active password probe waits60 additional ready frames, then right/A/B: key1 stores letter1 and B moves back without erasing. Earlier initialization-only presses are not credited as edits.
