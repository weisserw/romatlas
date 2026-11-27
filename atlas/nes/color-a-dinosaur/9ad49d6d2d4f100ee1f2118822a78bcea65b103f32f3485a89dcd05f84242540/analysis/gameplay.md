# Game familiarization — round 1

The local copyright framebuffer identifies **Color a Dinosaur**, copyright 1993 Virgin Games, licensed by Nintendo. Mapper 2, 128 KiB PRG, CHR RAM. Region/revision beyond local header/hash is unconfirmed.

Manual source consulted: https://www.thegameisafootarcade.com/wp-content/uploads/2017/02/Color-A-Dinosaur-Game-Manual.pdf . The manual describes Start to start/exit a picture; directional cursor movement; sixteen dinosaur choices; A selects point-to-point mode and fills selected areas; B selects freehand mode/moves palette cursor; Select displays different palettes. These are hypotheses until individually tested against this ROM.

This is a coloring activity. Representative progression milestones are title, dinosaur grid, distinct dinosaur selections, point-to-point painting, freehand painting, palette changes and returning to the grid. Conventional levels, bosses and a win/credits ending are not established and should not be assumed.

Historical startup observation: copyright and an animated dinosaur/title were visible; at that point short Start presses caused transitions but the grid was not yet verified. Loading/animation waits require bounded follow-up frames. No RAM shortcut contract is known yet.

Round1 result: grid and full stegosaurus with point cursor/palette confirmed. PaletteB/Select interactions observed; fill success and Start return remain unconfirmed. Exact route/statuses in scenarios.json. $4D flag shortcut failed to establish complete freehand entry contract; do not repeat unchanged. Clean natural snapshot restored before handoff.

Round2 evidence reconciliation: reuse redirect runtime5/6 and scenarios.json. Busy0/inputC1F0 Start schedules200-frame return; timer-shortened menu then Right/B and timer-shortened initialization establish picture1 freehand, actual directional movement and visible completed painting at1378. Artificial shortcuts qualify reachability; prior failed $4D-only shortcut remains historical. Coloring milestones include grid return and alternate/freehand painting; conventional ending/bosses remain unestablished.
