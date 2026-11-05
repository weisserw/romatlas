# Alpha Mission gameplay notes — round 001

Local title capture identifies the SNK 1987 Nintendo of America NES release;
CNROM mapper3. External sources are hypotheses, not ROM classification evidence.

- [Guide controls](https://gamefaqs.gamespot.com/nes/587088-alpha-mission/faqs/79420/controls-and-gameplay), guide v1.0 (2021): directional movement, B airborne laser, A ground missiles, Select armor menu, Start pause. Guide describes selecting armor by moving ship over it and pressing Select again. Local correct `buttons` requests confirmed Start, Select menu, simultaneous weapons and movement.
- [Ending FAQ](https://www.neoseeker.com/nes-hardware/faqs/139429-nes-game-endings.html), v5.7: areas1–6 repeat as7–12; area13 loops into area1, without a distinct conventional ending. [Cheats list](https://gamefaqs.gamespot.com/nes/587088-alpha-mission/cheats) also describes area13. Not established as this ROM's full ending behavior.

Local progression contract: `$28` fractional screen scroll0..239; `$29/$2A` screen
counter. 13 checkpoint triples at CPU `$92E2-$9308` contain word scroll plus area
ID1..13: 0002,0018,002E,0044,005A,0070,008C,00A4,00BA,00D0,00E6,00FC,011A.
Restart through state `$25=$CC` preserves real-player bit3 and invokes `$82C5` to
initialize objects, palette, tile streaming and area banner. Snapshots and exact
patch recipes are in scenarios.json.

Boss command positions from `$FB78+scroll`: 0017,002D,0043,0059,006F,008A,
00A3,00B9,00CF,00E5,00FB,0116. Consumer `$863C` only executes command when `$28=0`;
normal progression increments `$28` before testing. To expose a command after a
clean initialized stage, patch `$28=$EF` and `$29/$2A=command_scroll-1` before one
frame. Final-base command0116 set `$E8=6,$ED=1` and displayed the base; however the
synthetic route lacks all six initialized destruction objects, so 240 further firing
frames leave boss state frozen. This is an attempted terminal shortcut, not a cleared
final base or proof of a naturally reached ending. Redirect should investigate the
spawn/terrain/object initialization contract rather than repeat this stalled patch.

Armor inventory `$DA` is bitset, energy `$DB`. Menu cell lookup `$8DFB` maps grid
indices to armorIDs1..8 or none0; `$8DD3` provides inventory bits. Patching `$DA=$FF`,
`$DB=$80` enabled menu. Center selection chose none; correcting ship position
`$B4=$40,$B5=$40` targets occupied cell1. Captures preserve attempted and accepted
outcomes separately.
