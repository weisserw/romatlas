# Game familiarization

ROM title confirms Bandai Golf: Challenge Pebble Beach, copyright 1988. Mapper 3,
32 KiB fixed PRG, 32 KiB banked CHR. Region/revision not independently established.

Sources consulted 2026-11-13:
- https://www.digitpress.com/library/manuals/nes/Bandai%20Golf.pdf — original
  manual; 18 holes, one/two players, wind, terrain, club and spin choices.
- https://gamefaqs.gamespot.com/nes/587314-bandai-golf-challenge-pebble-beach/faqs/23121
  — Daniel Gordon FAQ search excerpt: pre-shot up/down chooses SHOT/COURSE/SCORE;
  B confirms. Hole/tee lengths and handicap vary. Full fetch unavailable.

Sourced descriptions are experiment hypotheses, not ROM structural evidence.
Confirmed: Start from title enters player-count selection (frame 107).

Local confirmation during round001:
- A confirms menu entries and selects SHOT; B alone did not activate the current
  shot option in the tested state. The FAQ's B-select instruction may reflect
  an A/B naming mismatch; follow the saved tested routes for this ROM.
- Natural route visits title, player count, regular/back tee selection, handicap,
  initial scorecard, hole1 overview, club selection and golfer/power meter.
- Supported hole-loader shortcut: restore course1.snapshot.json; release buttons;
  RAM $61=zero-based hole index, $2E=2,$2F=0. Run55 frames, A3/release40
  closes mandatory scorecard, then A3/release25 enters club choice.
  Confirmed distinct OUT9 (465 yards/par4) and IN18 (558 yards/par5) layouts.
- Final completion shortcut: from hole18-club snapshot set $61=17,
  $0200=$80,$0240=$80 (finished flags), $2E=9,$2F=5; release2,A3,
  release55. Handler increments hole to18 and enters final scorecard. Start3,
  release30 returns to title. Artificial scores are not a played-round result.
- Golf has no boss milestone. Final scorecard/title return is the observed terminal
  flow; no separate credits screen was identified. This does not prove absence
  of every alternate ending.

Working RAM contracts for later readable-source investigation (local static and
runtime evidence, names provisional): $2E major state, $2F substate, $30 shot
meter phase, $61 zero-based hole/current terminal index18, $53 player block
base0/$40, $5A menu cursor, $60 club, $5B live meter, $5C captured meter,
$5E second captured meter, $5F aim/timing parameter. $0216/$0256 hold player club;
$0217..$021D/$0257..$025D player position/state copied to $9C..$A2.
$0200/$0240 flag bit7 gates finished-player/next-hole flow. These flags have
other bits whose meanings are only partially recovered.
