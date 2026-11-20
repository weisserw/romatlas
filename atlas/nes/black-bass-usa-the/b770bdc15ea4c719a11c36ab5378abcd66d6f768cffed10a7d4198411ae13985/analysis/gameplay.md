# Gameplay research

Game: The Black Bass USA (Hot-B, 1989; NA release, title screen "THE BLACK BASS USA").
Image: mapper 2 UxROM, 128 KiB PRG, CHR-RAM, sha256 b770bdc1...

## Sources (hypotheses until confirmed locally)
- US manual text: https://archive.org/download/NESManuals/Black%20Bass%2C%20The%20%28USA%29_text.pdf
  (via search summary; not fully read): A starts power meter, second A stops it and casts;
  B thumbs line; A reels.
- Japanese-version FAQ (The Black Bass, different version): https://gopherproxy.meulie.net/tilde.pink/0/~bencollver/gamefaqs/text/nes/the-black-bass-japan/FAQ-by--Darkstar-Ripclaw.txt
  — controls may differ; not applied.
- https://en.wikipedia.org/wiki/The_Black_Bass_(1988_video_game) (naming only).

## Locally confirmed (round 1)
- Title -> Start -> name entry (A accepts) -> menu RANKING PLAYER (password "standing code") /
  BEGINNER / LOOK AT MY RECORD.
- Beginner: tournament intro at a lake ("PLEASE ENJOY YOURSELF..."), lake info sheet, lake map.
  Lakes (ROM strings): SAN LAKE, LAKE MORE, JAPAN LAKE, LAKE AMADA. Beginner start = Lake Amada.
- Map: A goes to casting view; d-pad moves boat via MOVE menu option.
- Casting: A starts power meter, A casts (confirmed). Tapping A reels lure slowly. Once lure is
  back, point menu appears: SELECT LURE / DATA FOR THIS POINT / YOUR RESULTS / MOVE / CASTING / END.
- END -> "ARE YOU SURE" YES -> CEREMONY(CODE) / GO BACK HOME(NO CODE) / LOOK TODAY'S / LOOK TOTAL.
  Ceremony shows record then a 16-char standing code (e.g. 2BCPIMZBPM5J4DNJ for player "A").
- Password entry: up/down cycle a character (A wraps to digits '5'), right moves cursor; A submits;
  invalid -> "WARNING... THIS CODE IS INCORRECT!".
- RAM: $0301 main mode (index into jump table $C17D), $0303 hour, $0304 minute,
  $030D lake index (0 San, 3 Amada; data bank = $030D+1), $17/$18 joypad 1/2 (A=bit7).
- Patch: $030D=0 on the intro screen loads San Lake correctly (artificial).

## Not confirmed / open
- Fish strike/fight/catch (no strike seen with pencil bait at 0 ft), multi-day progression,
  ceremony winner/prize screens ("YOU ARE WINNER IN A...", "2ND PRIZE"), ending. See leads L-004.

## Round 1 redirect (r001-a7k2) — late game reached (artificial-state aids recorded)
- Standing code model verified offline: analysis/redirect/r001-a7k2/tools/standing_code.py
  decodes the game-produced '2BCPIMZBPM5J4DNJ' (seed $E0, Day 1, RankPoints 200, checksum ok).
- Generated code LIUBBEPZECTRON4A = Day 11, RankPoints 10, total 500, heaviest 80,
  lure 1/colour 1, 10 bass. Written into PasswordText ($0434, 16 ASCII bytes) on the
  RANKING PLAYER entry screen instead of typing it, then A: accepted (CodeInvalid 0),
  StartDay -> Day 12, intro "SAN LAKE 9/25 6:00-16:30" (day-12 lake chosen by the game).
- Fish strike: rewriting ActionScore $03C7=$64 and LureAttraction $03C5=$FF every 3 frames
  while retrieving -> FishingPhase 2 (approach), FishState 1 then 2: "OH! THE FISH HITS! FIGHT!".
- Fight: hold A with LineTension $0411=0 and FishStamina $040B=0 rewritten every 2 frames ->
  line reeled in ~1850 frames, FightLanding 2, "YOU GOT IT!", catch sheet (PIKE 1.4->2.1 Lb,
  6:11, fight time 0:08, PENCIL BAIT RED), "CONGRATULATIONS", back to the point menu.
- END -> YES -> end menu; CEREMONY: record sheet "PARTICIPANT RANKING 50TH, CLASS A ...
  SORRY! YOU MOVED FROM CLASS A TO B", then (Day 12) "THE FISHING TOURNAMENT OF THIS YEAR IS
  ALL OVER", then the title screen (GoHome soft reset). This is the game's ending text.
- Winner ceremony: CeremonyScreen needs RankPoints < 6 after SubClassPoints; reached with
  RankPoints $0445=1 patched on the end menu (see scenarios S-015).

## Round 2 redirect (r002-q5m8) — remaining menu/lake/prize milestones
- RankPoints is the participant ranking (overall place 1..200, lower better); class A 1-50,
  B 51-100, C 101-200; the class picks the lake: A San Lake/Lake More (day parity),
  B Japan Lake, C Lake Amada. Generated codes (analysis/redirect/r001-a7k2/tools/standing_code.py):
  LITRVEPZECTRON4J (Day 10, 20th) -> Lake More day 11; LIUEFEPZECTRON5S (Day 11, 60th) -> Japan Lake day 12.
- Ceremony: in-class places 1-5; place 1 "YOU ARE WINNER IN A CLASS x", 2-5 "YOU GOT A nTH PRIZE"
  (captured 3RD/5TH with patched RankPoints). Bottom 5 of a class swap with the top 5 of the next.
- LOOK AT MY RECORD (class B greeting renders "Mr. <name> ,"); name "HBMAX " enables the debug HUD
  and Start ends the power meter at full power.
- Details/frames: analysis/redirect/r002-q5m8/findings.txt, frames/; scenarios S-015..S-017.
