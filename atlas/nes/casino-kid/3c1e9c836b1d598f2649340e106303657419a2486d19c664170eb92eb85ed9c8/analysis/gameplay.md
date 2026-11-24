# Gameplay research

Game identified locally from title screen: "CASINO KID" (c)1989 SOFEL, licensed by
Nintendo of America (US release; footer ASCII "CASINO KID" at CPU $FFE0 in PRG bank 7).

## External sources (round 1)

- https://en.wikipedia.org/wiki/Casino_Kid (read via fetch agent): goal is to earn money and
  beat the "King of the Casino"; games are blackjack and five-card draw poker; bunny-costume
  waitresses give hints. No controls/passwords listed. Unconfirmed beyond what is noted below.
- GameFAQs and StrategyWiki returned HTTP 403 (do not retry); no passwords found externally.

## Locally confirmed (this ROM, debugger)

- Controls: input must be sent as `emulator.input {"buttons":[...],"port":1}` (see TK-001).
  Joypad raw/new-press bytes: $F7/$F8 = held (P1/P2), $F5/$F6 = newly pressed; bit order
  A=$80 B=$40 Select=$20 Start=$10 Up=8 Down=4 Left=2 Right=1. NMI copies $F7 or $F5 to $26
  depending on mode byte $25.
- Title: menu NEW GAME / SAVED GAME; title code polls $26 == $10 every 6 frames, so Start
  must be held >= ~12 frames. Down/Select moves cursor. NEW GAME -> intro text (Start, A)
  -> casino overworld (8-direction walk; the screen scrolls with the player at screen centre;
  A talks to an adjacent person, giving a portrait dialogue screen).
- SAVED GAME -> password grid (A-Z, REVERSE/FORWARD/END), 30-character buffer at $05DC.
  Wrong password -> "IT'S WRONG!".
- Money: 32-bit LE at $055D (display = value/100; new game 50,000 = $C350 internally).
  King's money in the final match: 32-bit LE at $075B.
- Debug/cheat path (bank 5 $936B, called on overworld entry at $87D4): with A+Start held
  ($F7 == $90) and password buffer $05DC equal to the 30-byte string at $93DF
  ("I AM A RICHMAN", letters A=$0A..Z=$23, space=0) money becomes 10,000,000; equal to the
  string at $93FD ("I WANNA SEE THE KING") money becomes 100,000,000, $AA=1 and control jumps
  to $C0F4 -> final King match (bank 4 poker vs King, "sweet dealer").
  Confirmed by RAM-writing the KING string into $05DC on the password screen, choosing END and
  holding A+Start (artificial state; natural password typing not completed this round).
- Poker controls (bank 4): A advances text/selects; on "WHICH CARDS DO YOU WANT TO HOLD?"
  Left/Right move, A toggles HELD, B draws. "SEE, RAISE OR FOLD" choices via A (default).
- Ending/credits (bank 6, "CG DESIGN T.NOMURA", "PROGRAMMER K.TAKAHASHI") reached after
  RAM-setting King's money $075B to 1,000,000 and finishing the hand (King broke).

## Open gameplay questions

- Which overworld NPCs are blackjack/poker opponents (two NPCs talked to were hint/flavour
  characters). $055B/$055C/$0569-$056C appear to track opponent progress (bank 5 $A534-$A6A8).
- Blackjack (bank 3 entered via $C0E8/$C19F) not yet exercised.
- Natural password typing: vertical cursor moves were unreliable with 6-10 frame presses.
