# Gameplay research

## Identification
- Title from ROM: "THE BLUE MARLIN" (c)1991 HOT-B USA; internal tag "BLUE MARINE Ver1" at file 0x1fff0.
  Mapper 4 (MMC3), 128 KiB PRG / 128 KiB CHR, NTSC (USA). Confirmed locally (title screen frame ~4960).

## Sources (round 1, light research)
- Wikipedia https://en.wikipedia.org/wiki/The_Blue_Marlin : four tournament rounds (Florida, Hawaii x3),
  biggest billfish wins; password given after each round restores attributes. (sourced, not all confirmed)
- Manual scans exist (archive.org NESManuals, nesfiles.com PDF) but are image-only; 64scener text returned 403.
  Sourced controls (unconfirmed): Up = brake line, B = drag, Left/Right moves lure.

## Locally confirmed (round 1)
- emulator.input takes `buttons` array (full held set), port 1.
- Boot: HOT-B logo -> long intro ("HOT-B PRESENTS", "THE ENDLESS SUMMER...HAWAII", marlin) -> title ~frame 4960.
- Title: START -> "START GAME / RESUME GAME" (resume = password). START GAME -> name entry (A on END
  accepts default NONAME) -> confirm (A) -> weather/sunset screen (A) -> boat cockpit "WHERE SHALL WE GO TODAY?
  FISHING POINT / PALM BEACH" (A) -> top-down harbor/sea navigation.
- Sea navigation: A = throttle, D-pad steers; START = pause (shows line length / lure depth);
  SELECT = COMMAND menu (INFORMATION [player/result/status], SELECT THE LINE, SET THE LINE LENGTH,
  SELECT THE LURE, SET THE DEPTH OF LURE, EXIT).
- No natural bite in ~3000 frames of trolling (round 1). RAM shortcut: writing $0496=1 (cpu RAM) during the
  navigation loop ($E37E loop in bank 15 tests $0496) produces "HEY, NONAME! I FEEL A BITE ON YOUR ROD."
  then the fight screen (banks 12/13 at $8000/$A000). Artificial state; natural bite trigger unresolved.
- Fight: SELECT opens "WHAT WILL YOU DO?" menu (wind reel / swing rod / do nothing / thumb line / ...).
- Hidden SOUND TEST: hold SELECT+START at power-on (code $E315 tests pad $46 & $30 == $30 -> $FA27). UP/DOWN change
  $EA, A plays, B stops ($FD), SELECT returns to normal boot. Confirmed locally.
- Fight outcomes seen: long A hold -> "TOO BAD! THE LINE HAS BEEN CUT."; pulsed reeling -> "WELL, NEXT TIME YOU'LL DO
  BETTER." sunset scene, then back to sea. Landing a fish not yet achieved (lead L-001).

## Redirect round 1 (r01-a, 2026-11-20) — progression via RAM patches (artificial scenarios)
All from round-1 snapshots (scratch/snap_*.bin, same build); exact recipes in analysis/scenarios.json S11-S14 and
analysis/redirect/r01-a/README.md. Observed results (local emulator, this ROM):
- Landing a fish: in the fight, hold A and write LineOut ($075D-$075F) = 0,0,0 for 2 frames -> LineOut_Check sets
  FishOnLine=$80 -> "GOT HIM !!" (banks 12/13) -> weigh-in: "THIS FISH IS A DORADO AND IT WEIGHS ABOUT .. POUNDS",
  "WILL YOU KEEP THE FISH? YES/NO", then back to sea (banks 0/1). Species of the round-1 forced bite: dorado.
- End of day: at sea write ClockSeconds/Minutes/Hours ($0755-$0757) = 59,59,15 -> after ~4 s "OH SHOOT! TIME IS UP."
  -> menu WEIGHING / GO FISHING AGAIN / RESULTS FOR TODAY (A selects; DOWN moves after the menu is drawn).
  RESULTS -> bodybuilder weigh screen with counting weight, "CONGRATULATIONS, <name>, THE BIGGEST ...", "WELL, BETTER
  LUCK NEXT TIME." (or round win), "WRITE DOWN PASSWORD" + 30-char password, then next day's Round_InfoScreen.
- Hawaii: with RoundCleared $079E-$07A1 nonzero before Round_InfoScreen (AllRoundsFlag $E9 set), SELECT on the
  info screen cycles TournamentRound ($0499): round 1 = "HAWAII 1", cockpit "FISHING POINT KAWAIHAE", AreaIndex 4,
  Hawaii sea map with cliffs. (Supported in-game transition; flags set artificially.)
- Ending: at the day-end menu with RoundCleared[1..3]=$FF and best weight $0777/$0778 = $01B0 (> TargetWeight
  $07FA=$01AA in that state) choose RESULTS -> "CONGRATULATIONS ... THE BIGGEST", "YOU HAVE WON ALL TOURNAMENTS.",
  password, then a "FIN" ending screen (man holding a marlin) that stays. No credits roll seen within ~20 s.
- Easter egg text "MEGUMI.MY.LOVE" at bank 15 $E0E1 (static only).
- Round 2 (disassembly r2): sea events — at sea (scratch/snap_sea.bin) write SeaEventFlag $05C4 = 1, 2 or 3; the
  navigation loop dispatches Nav_RunEvent ($E3E9 table) -> Nav_RunEvent0/1/2Script -> bank-0 $8BF5/$8C33/$8C71
  (window-text event messages, A advances); event 3 (index 2) reloads the area (Day_LoadArea). Traced; the bank-0
  wrappers predicted statically in pass-002 executed. Screens captured only at the end (back at sea).

## Redirect round 2 (r02-a) notes
- No new milestone needed: S01-S16 all visited. Sea and Hawaii-sea snapshots re-run for 30 frames (no patches) to confirm
  that the rule-R "data reads" at bank 1 $A20E/$A51C/$A801 are dummy reads of page-crossing branches ($A2FB, Fish_Wander
  $A5E0, $A8EF) executed every frame by the fish movement code (analysis/redirect/r02-a/dummytest-results.txt).
- Bank 1 contains an older/unused-looking fish probe and shadow-bite routine set ($A3F5-$A5DD) next to the live
  Fish_Probe*/FishShadow_TryBite code; no reference to it was found (static classification proposed in L-019).
