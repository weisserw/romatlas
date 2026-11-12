# Game familiarization — round 001

Local embedded identification: Back to the Future V1.21, TVN 3/5/1989,
Beam Software. iNES mapper 3, PRG 32 KiB, CHR 32 KiB; US-like English screens.
External guidance is a hypothesis, not classification evidence.

Sources consulted:
- https://etherealgames.com/nes/b/back-to-the-future/preformatted-cht-file-action-replay-game-genie-us/ — proposes stage selector $00D2, lives $0393, driving speed $0660=$58 and minigame win counters. Local code confirms D2 indexes stage layout pointers at $A0AA and main mode selectors $81/$82/$83/$84 at $05C8. Speed patch alone did not produce visible driving output; other cheat counters remain untested.
- https://datacrystal.tcrf.net/wiki/Back_to_the_Future_(NES)/RAM_map — lives $0393 and ASCII timer digits $04EA..$04EC. Local code initializes those values. Timer patch during artificial completion did not clear all bonus countdown state.
- https://www.thegameisafootarcade.com/wp-content/uploads/2017/02/Back-to-the-Future-Game-Manual.pdf — search result identifies distinct driving game; full manual was not downloaded.

Tested: Start after title; A/Start confirms map selection; Up/A/B inputs in street.
Cafe, school and dance have distinct screens and initialization routines $BE58,
$C5D1 and $CC15 selected by main flow $8183 onward. Ending success screen is
message index $0C entered at $820F; artificial entry displayed congratulations.
Driving attempted via main entry $81FB; black output remains unresolved. Do not
claim natural late progression, driving completion, full compatibility or fidelity.

## Round 001 redirect local driving confirmation

Source-grounded artificial route (no new external gameplay claims): importing
round-001/active.snapshot and entering RunDrivingMode $D1FC initializes StageIndex
$11, DrivingNmi $D669, road/car/HUD and speed30 within240 frames. Main $81FB includes
a lengthy introductory story card, so early black samples were not evidence that
its driving initializer failed. Up/right45 frames moved X$6E->$8C and speed30->38.
A $CD=$FB patch at the initialized mode reaches StreetActive=0 against StreetEndRow
$FC. This is artificial representative driving/progression, not a natural ending.
Direct $D1FC SP=$80 does not build a caller return frame: for future main outcome
research use PC=$8200, the real JSR$D1FC. Success also needs type12 interaction
handler $B3A7 at speed >=$58 to set $A9. Exact calls/patches/budgets and selected
frames are in analysis/redirect/round-001-attempt-001; scenarios.json updated.
