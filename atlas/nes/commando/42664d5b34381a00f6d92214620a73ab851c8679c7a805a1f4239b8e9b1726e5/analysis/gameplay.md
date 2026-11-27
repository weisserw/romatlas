# Gameplay familiarization — Commando (NES)
Title framebuffer confirms Commando, Capcom/Nintendo 1986; US-looking title.
Natural route: restore title at frame150; hold Start for60 frames, release; wait
360 frames for helicopter departure, then Up+A+B for bounded combat/scrolling.
Short 2-frame Start was insufficient in tested title timing. Input API uses buttons
arrays; old singular button/pressed records are ineffective and not gameplay evidence.

Sources consulted for practical later progression:
- https://www.nesmaps.com/maps/Commando/Commando.html — title controller2 sequence
  Left,Right,A,B,A,Down,Up; hold controller1 A and press Select to choose area.
  Also supplies RAM invulnerability $0060=$80 and lives $049C=$03; these are
  hypotheses until confirmed locally. No assets downloaded.
- https://strategywiki.org/wiki/Commando/NES_Walkthrough — search snippet suggests
  radio pickup helicopter warp; not yet tested against ROM.

Local confirmation: controller2 sequence with8 frames press/8 release, followed
A+Select, changes display to 1-2. RAM $049D/$049E incremented to1. Static routine
physical $1CC63 increments $049D modulo16 and $049E modulo4, supporting mission/
area indices independently. Newgame clears both at $1B1EC/$1B1EF, explaining failed
pre-start patches. Active-state edits without reinitialization corrupt terrain.
See round001 handoff/scenarios for exact current frontier.
