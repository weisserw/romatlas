# Gameplay research

Game identified from title screen and ROM strings: **David Crane's A Boy and His Blob: Trouble on
Blobolonia** (NES, Absolute Entertainment 1989, USA; MMC1, 128K PRG / 128K CHR). Revision not
independently identified beyond the input SHA-256.

## Sources (external = hypotheses until confirmed locally)
- Almar's Guides Action Replay codes: https://almarsguides.com/retro/walkthroughs/nes/games/DavidCranesABoyAndHisBlobTroubleOnBlobolonia/ActionReplay/
  (lives $00A2, "stage/area modifier" $00EE, bean counts $02BF..$02D7 odd addresses).
- MASTERNO save-state hex guide (PDF): https://www.video-games-museum.com/en/faqs/Nes/5055-Boy-and-his-Blob-A-Trouble-on-Blobolonia.pdf
  ($004B fall speed, $0071 holding, $0073 throw lock, $0075 bubble, $007A trampoline height,
  $0082/$0083/$0086 blob sprite/form-state/form, $00A2 lives, $00AF mints, $00B2 pharmacy flag,
  $02BF~$02D9 beans (BCD), $02DA-$02DF vitamins, $0356 treasure count, $00A9-$00AE score).
- Giant Bomb / TV Tropes summaries (bean forms; orange bean = VitaBlaster).

## Locally confirmed (this ROM, pass 1)
- Controls: Start on title -> credits -> Start -> game. Select cycles bean flavour (skips flavours
  with count 0; selected flavour in $0098). A throws the selected bean. Start pauses.
- Bean counts BCD at $02BF + 2*flavour (licorice 72, strawberry 14, coconut 06, cola 18, cinnamon 24,
  apple 12, lime 00, vanilla 30, tangerine 42, root beer 06, honey 52, ketchup 06, punch 54 at start).
- Bean names text (ASCII-0x36 encoding) at bank 0 $8D22.. (LICORICE ... ORANGE, LADDER ... JELLYBEAN).
- Joypad read routine bank 0 $87CB (double read, $EA/$EB compare, result $15, masked $14).
- World flag $16 (0 = Earth, nonzero = Blobolonia), checked e.g. at $B7B8 trampoline limit ($F4 vs $4F).
- Position/room: $5A (x), $5B (y), $5C (room) loaded by bank 0 routine $926F from the 4-record
  table $928B {x,y,room,world}; $A4=1 requests the re-entry. $926F has no static caller (debug/unused warp?).
- $00EE is a temporary in fixed-bank map lookup ($C0C0), not a persistent stage selector here
  (writing it had no effect) — the Action Replay "stage modifier" description does not apply directly.
- Lives $A2 confirmed (set 0 -> death leads to GAME OVER).
- Earth surface (house, subway entrance), caverns (rooms $1B, $42, $43), underwater caverns
  (room $50), Blobolonia jungle (warp entry 0, world 1), Blobolonia factory-like rooms ($88 world 1)
  were displayed after RAM/register warps (artificial).

## Routes / shortcuts usable in later rounds
See analysis/scenarios.json. Key artificial shortcut: at main-loop top ($808A, bank 0) push return
$8089 and set PC=$926F, A=index 0..3 (or write $5A/$5B/$5C/$16 and $A4=1 directly).

## Unconfirmed / open
- Pharmacy/vitamin shop, treasure collection, Emperor encounter and ending not reached.
- Root beer rocket did not launch from the start screen (likely gated by progress).

## Round 1 redirect findings (artificial RAM shortcuts; see scenarios S10-S12)
- Room 7 (Earth) is the HEALTH FOODS store: with a nonzero score, holding Right at its door enters the
  vitamin shop (PharmacyState $B1); the offered assortment depends on treasures left / score
  (labels from bank 0 $9024). Confirmed on this ROM (artificial score/position).
- Treasures are room pickups (bank 6 RoomPickupIndex/RoomPickups); collecting one in room 8 decremented
  TreasuresLeft $0356 and set the respawn point. Confirmed (artificial placement).
- Ending: EndingCountdown $B4 nonzero (set by the jack form raised in SpecialRoom) leads to the Emperor
  celebration scene $FD with PCM speech/music (bank 5). Natural trigger not reproduced.
