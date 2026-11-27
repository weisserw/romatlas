# Cobra Triangle familiarization

Title frame confirms Cobra Triangle, Rare copyright 1988, Nintendo of America license. Region/revision remains unconfirmed; iNES mapper 7, 128 KiB PRG, CHR RAM.

Manual transcription: https://www.world-of-nintendo.com/manuals/nes/cobra_triangle.shtml . A fires, B accelerates, left/right steer, Select activates upgrades, Start begins/pauses. Manual describes 25 stages with race, pods, mines, monster, people defense, hazard, waterfall and target mechanics. These define representative progression milestones, not independent classification evidence.

US cheat compilation (hypotheses pending local tests): https://etherealgames.com/nes/c/cobra-triangle/preformatted-cht-file-action-replay-game-genie-us/ . Stage select RAM pair $0702/$0737; health $05F5 or $05F6; lives $0738; timer $0713/$0714; boss health $05FF. Stage values and transition contract must be confirmed against this image. No ROM patches used.

First attempted Start routes used shared protocol's button/pressed schema, which NES silently ignores; actual NES implementation requires buttons array plus port. Corrected route is logged in round-001/rpc.jsonl.

Ending guide checked: https://www.world-of-nintendo.com/game_endings/nes/cobra_triangle.shtml . It describes stage 25 as an automatic rescue of people on five downed planes, followed by credits and a terminal message. Local selector49 / map26 / type9 matches the rescue tableau; three 240-frame waits and three 600-frame waits (short traced samples between, exact RPC log retained) advanced to the staff credits, visually confirmed in round-001/credits-2.ppm. This is an artificial entry, not a natural full-game completion. The earlier `$05FF=0` experiment in this state was incorrectly labeled boss HP zero and proves no boss defeat.

Locally validated stage-init recipe: import skip-2 snapshot; clear inputs; advance 10,000 CPU cycles then finish one operation to avoid its pending NMI; patch `$0737` sequence index, `$0702` preceding display count, `$ED=0`, `$0738=9`, `$05F5=12`; set PC `$816D` in effective banks [0,1,2,3]. This entry disables rendering/NMI and initializes stage geometry/type from sequence data. Representative selectors 25–32 and49 were tested; exact outcomes differ by type and snapshots. Old immediate-PC tests were interrupted by NMI and do not establish stage visits.
