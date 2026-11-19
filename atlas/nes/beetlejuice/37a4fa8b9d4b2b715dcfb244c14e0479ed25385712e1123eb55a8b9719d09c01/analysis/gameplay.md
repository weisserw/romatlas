# Beetlejuice NES familiarization

On-screen Rare/LJN credits identify Beetlejuice (1990/1991). Natural local run confirms title, showtime, world map, side-view house, inventory and jump/movement. NES input uses full held buttons arrays.

[Speed Demos Archive mechanics](https://kb.speeddemosarchive.com/Beetlejuice_(NES)/Game_Mechanics_and_Techniques) supplies hypotheses: RAM $55 room, $1AD invincibility, $3BD–$3C2 player position, $1AA health, $3D3 lives. Start opens inventory; A jumps; B attacks; Select uses/drops items. Room9 reportedly begins overhead gameplay; room9 edit alone did not initialize it locally. Room contract remains under investigation. [Any% route](https://kb.speeddemosarchive.com/Beetlejuice_(NES)/Any%25) and local code can guide follow-up.

GameFAQs result labeled level-select GB and [GameMite](https://www.gamemite.com/games/beetlejuice/cheats/) confirms its level-select belongs to Game Boy; do not apply that code to NES. No ROM modifications/Game Genie codes used. External descriptions do not classify ROM. Full exact routes and tested failures are in scenarios.json and round1-rpc.jsonl.

## Round2 source-backed ending gate
Bank3CPU A891checksroom7,frame&3=2,X058D..05A7,Y00D0..00FB,85!=86;85=3Ftakescurtainsequenceandactualscript24thenFDC3title. TestedartificiallywithRAMstage/position/maskandrecognizedanimationcycle6->7shortcut; no gatePC/scriptpointeredit. Actualframe1877titlescreenandfetchesinanalysis/redirect/r2-20261119-b/room7-cycle7-rpc.json;report.mdhasexactrecipeandnegativepose-onlycorrection. ThisislocalROMconfirmationofgateandreturn,notfullnaturalplayorcredits.
