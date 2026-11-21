# Game familiarization

Locally rendered copyright screen identifies Bram Stoker's Dracula, NES, 1993 Sony Imagesoft. Region/revision not independently established.

Sources consulted 2026-11-21:
- https://www.digitpress.com/library/manuals/nes/Bram%20Stoker%27s%20Dracula.pdf — search excerpt describes main-menu difficulty, sound/music and control configuration; not yet confirmed.
- https://takeontheneslibrary.com/finished/28-bram-stokers-dracula/ — guide describes seven scenes with day/night stages (14 levels); hypothesis until runtime confirmation.
- https://www.gamemite.com/games/bram-stokers-dracula/cheats/ — NES section provides Game Genie ROM patches only; nearby level-select button code is explicitly Genesis and unsuitable here. No ROM edits used.

Controls/progression experiments persist in round-001 scripts and RPC log. Later-stage RAM contract remains to be determined from local code; no arbitrary brute-force writes authorized by inference.

Guide read in full for actionable controls/progression: A jumps, B attacks, Down crouches; Down during jump smashes breakable floors. Lanterns act as checkpoints. Hard difficulty is reportedly required to reach all seven scenes and the ending; Easy/Medium truncate progression. These are external hypotheses until tested. Source: https://takeontheneslibrary.com/finished/28-bram-stokers-dracula/ . No NES stage-select route established; Genesis code rejected as wrong platform.

NES input API requires entire held-button arrays: `{port:1,buttons:["start"]}` then release with `buttons:[]`; singular generic protocol inputs are ignored (NES-001). Tested Start at title fades to animated Vampyre book; later menu not yet confirmed. Bulk stepping split into 60-frame segments after 180/240-frame timeout under concurrent work, with state queried after timeout and durable snapshots between segments.
