# Gameplay research

Game identified from its own title/credit screens (local observation, frame ~125-570 after reset):
"Captain Planet and the Planeteers" (Mindscape presents; software (c) 1991 Chris Gray Enterprises),
NES, mapper 4 (MMC3), 128 KiB PRG + 128 KiB CHR, SHA-256 9bee9829...

## External sources (hypotheses until confirmed locally)
- Wikipedia: https://en.wikipedia.org/wiki/Captain_Planet_and_the_Planeteers_(video_game) — NES version:
  five levels, each split into two stage types; in the second the player controls Captain Planet.
  (Version applicability: NES, North America 1991. Confirmed locally: alternating Geo-Cruiser/Captain Planet stages.)
- Search did not locate a password list; passwords below were recovered from this ROM instead.

## Locally confirmed facts (this ROM)
- Controls: title waits for Start; Start pressed during the title/credits cycle sets $0793=1, and at the end
  of the credits ($DF47) the game starts with the password screen (task $DF5E) instead of the attract demo ($DD6F).
  Without Start the attract demo plays level-1 Geo-Cruiser gameplay automatically (an AI demo, not player input).
- Password screen: Up/Down change the digit under the cursor, Left/Right move the cursor, Start submits
  (submission polled every 8 frames; hold Start ~10 frames). Digits stored at $057B-$0580 as value+4.
- Password table: PRG bank 0 at $8024 (file $34), ten 8-byte ASCII records:
  k=0 763754, 1 955783, 2 637511, 3 148574, 4 786565, 5 920272, 6 799274, 7 344551, 8 829443, 9 506210.
  Check routine $E060: k -> $FC=(k+1)>>1 (level 0-4), $0799=(k&1)^1 (0 = Geo-Cruiser stage, 1 = Captain Planet stage);
  k=9 sets $0715=1 and goes to the ending. Confirmed: k=9 shows the ending ("EARTH IS SAVED! THANK YOU, PLANETEERS",
  final score screen); k=1 starts level-2 Geo-Cruiser stage (forest/canyon); k=0 starts the level-1 Captain Planet
  intro (Planeteer briefing text, ring summoning "FIRE!/WATER!", transformation).
- RAM: $FC level index; $0799 stage type; $07D3 briefing-text done flag; $07D1 title phase; $0793 start-game flag;
  task table $0486/$0498 (resume address), $04AA (status: $80 run, $C0 sleeping), $04BC (sleep frames), $052B (task PRG bank for $A000, $FF = none).
- Geo-Cruiser stage: d-pad moves the cruiser, A/B fire. Captain Planet stages begin with long briefing and
  transformation sequences (hundreds of frames; Start skips the briefing text once $07D3 != 0).

## Shortcut recipes
- Password via RAM: restore the password-screen snapshot, write $057B-$0580 = digits+4, hold Start 10 frames
  (tools_local/pw.py K). This edits only the entry digits; the ROM's own check selects the stage.
- Direct stage start (artificial, used once): at the idle loop ($FA44-$FA47) write $FC, $0799, clear $04AA-$04BB,
  set task 0 = {$04AA=$80, $0486/$0498=$E418, $04BC=0, $052B=$FF} (mirrors $E391 creating the stage-start task).

## Open questions
- Captain Planet stage gameplay (after transformation) was only briefly reached; bank 8/13 CP-stage code is largely untraced.
- Boss encounters, game-over/continue screens not yet visited.
