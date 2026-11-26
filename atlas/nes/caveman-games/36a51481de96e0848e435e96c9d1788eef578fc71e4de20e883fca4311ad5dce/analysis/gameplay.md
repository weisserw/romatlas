# Gameplay research

Game identified from in-ROM text as **Caveman Games** (Data East, NES, 1990; title screen
"(c) Electronic Arts 1990 ... Manufactured by Data East U.S.A."). Port of Caveman Ugh-lympics.

## Sources (external, hypotheses until confirmed locally)
- NES manual scan: https://archive.org/download/NESManuals/Caveman%20Games%20%28USA%29_text.pdf
  (fetched 2026-11-24; text layer not extractable by the fetch tool — controls per event unread).
- Event list / modes: https://en.wikipedia.org/wiki/Caveman_Ughlympics ,
  https://takeontheneslibrary.com/tag/crudla/ (Practice = any single event; Start The Games = all six in sequence;
  several events are A-button mashing).

## Locally confirmed (this ROM, round 1)
- Attract loop: intro picture (cavemen) <-> title. START does not leave the title; **A** does
  (joypad byte $0558 = newly pressed, bit7 = A; see ROM $C564 read routine, $0562 raw).
- Menu flow (A each): Number of players (1) -> character select (Glunk/Crudla/Thag/Ugha/Gronk/Vincent)
  -> biography ("That's my boy!") -> main menu: Practice / Start the Games / Restart / Caves of Fame.
- Practice event select: cursor index RAM $0657/$049D; inputs closer than ~12 frames are ignored.
  Order with DOWN presses spaced 25 frames: 0 Saber Race (16K bank 4), 1 Mate Toss (bank 1),
  2 Clubbing (bank 5), 3 Fire Starting (bank 2), 4 Dino Race (bank 3), 5 Dino Vault (bank 6).
  Bank 0 = menus/bios/results; fixed bank 7 = engine.
- Start the Games order observed: Clubbing -> Mate Toss -> Dino Vault -> Dino Race -> ... (mashing A/B with
  left/right advances events; each event lasts ~2000-7000 frames with that input pattern).
- Event controls not yet understood beyond mashing; finishing events/placings reached via generic mashing.
