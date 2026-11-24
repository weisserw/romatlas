# Gameplay research

Game identified locally from title screen and ROM text: "Castle of Dragon" (c)1989 SETA U.S.A.,
licensed by Nintendo of America (US release; Japanese title "Dragon Unit" appears as ASCII at
$FFE5). No online sources were consulted in round 1; everything below is local confirmation
against this ROM (sha256 c1de0580...). External guides remain unconsulted (version uncertainty n/a).

## Locally confirmed controls/flow (round 1)
- Title: PUSH START. Start -> intro castle scene; Start -> world map (Wenlary Castle / Heresy Forest / Darklarza Castle).
- Map: A (or Start for index>=3) enters the stage; for stage index >=3 Up/Down/Right choose route (code $E960-$E9E2: Right -> index 7, Down/Up step between 4..6).
- In stage: Left/Right walk, B attack, A jump, Down crouch, Start pause. Energy bar at left.
- Stage 1 (index 0) skeleton blocks progress when holding right+B; player loses energy -> GAME OVER -> title.
- Hidden TEST MODE: at title press A,A,Left,Left eight times (table $D0CA) -> menu EXIT / (player) / WEAPON / STAGE / SOUND / SPTEST / BGTEST. Confirmed (scenarios T1/T2).

## RAM contract (artificial shortcuts)
- $052A = stage index 0..7 (0 castle hall, 1 castle exterior, 2 Heresy forest, 3 tower field,
  4 palace hall, 5 arcade corridor, 6 dark forest, 7 dragon boss). Writing it on the map screen
  then confirming loads that stage (all 8 confirmed by frames; artificial state, see scenarios.json S0-S7).
- $050C = main mode: $00 title, $F0 game, $FF test mode.
- $06C1..$06CF = controller held/new-press bytes (4 pads OR-ed); bit order A=$80 B=$40 Sel=$20 Start=$10 Up=$08 Down=$04 Left=$02 Right=$01.

## Not yet reached
- Dragon defeat / ending credits (credits text at bank 6 $8F92-$90D7). See leads L-003.

## Round 2 local confirmations
- In stage, Start switches weapon (sword -> weapon A -> weapon B) once WeaponsOwned $06FC has the bits (artificially set $03);
  the first Start press after entering did not switch (scenario S-WPN).
- Object slots are 16 x 16 bytes at $0300 (slot 2 guardian: ObjHP $032A); GuardianDefeated bit array at $0480 per map page.
- Stage 4 pages 26->30 traversed with energy refills (S-PROG4); stage 3 run dropped to the title (S-PROG3, cause unknown).
