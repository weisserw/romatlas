# Gameplay research

Game identified from ROM text: Caesars Palace (NES, Virgin Games; in-ROM legal text "Copyright (c) 1990 Virgin games, Inc."; sources date the NES release 1992).

Sources (round 1, one web search; no manual contents retrieved):
- https://www.giantbomb.com/caesars-palace/3030-1542/releases/ (NES 1992, Virgin) -- lists Roulette, Blackjack, Slot Machines.
- https://gamesdatabase.org/game/nintendo-nes/caesars-palace (manual download link; not read).
- https://www.jammaplay.com/en/game/19808-caesars-palace-usa (lists blackjack, slots, roulette, horse racing, keno, video poker, red dog -- unconfirmed for this ROM).
Game list is unconfirmed; treat as hypotheses.

Locally confirmed (round 1):
- Controls: d-pad moves an arrow cursor ($04EA x, $04EB y); A selects. On the casino floor, A at a location makes the player walk there and the floor scrolls. Start skips the taxi intro.
- Joypad: $0436 raw byte (A=bit7); expanded per-button flags $03CE-$03D5 (right,left,down,up,start,select,B,A) with previous frame at $03D6-$03DD (routine $FD0C).
- Cashier gives $1000 credit (text). Slots ($1/$5/$25/$100) and blackjack table are reachable by walking.
- Unresolved: how to bet/insert coins (A, B, A-drag tried on slot and blackjack without effect).
