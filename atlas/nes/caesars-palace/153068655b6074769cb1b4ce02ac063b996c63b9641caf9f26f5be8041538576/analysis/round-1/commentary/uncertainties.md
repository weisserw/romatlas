# Commentary uncertainties

Carried phase-1 ambiguities and interpretation limits after round-1 commentary.

## Structural (from phase 1, unchanged)
- File 0x18010 (bank 6 $8000) remains UNKNOWN; noted in source (L-003).
- Unreferenced fragments (L-004) are labelled *_Unref / Unreferenced_*: UnusedShowScreen0 $C679,
  BuildAnimatedSprite_Unref $E296 (a generic flip/clip animation builder using SprAnimPtrs; no caller),
  Load1bppFont_Copy_Unref $E49F (duplicate of Load1bppFont), CopyPtr16ToPpu_Unref $FB10, Nmi_Unref_* , several
  tiny math helpers ($CB42, $CB56, $CBE9, $CC1E, $CC4C, $DED1), FourReelSlot_Unref $8AAE/$8BD7, sound mute/silence entries.
- 15-byte table at $E195 (phys 123301) and 32-byte table at bank 6 $85CE: consumers not found.

## Semantic (tentative wording used in source)
- Sound ids: roles inferred from call sites only (09/0A hold toggles, 0B card, 0C refused, 0E win, 0F chip,
  11 tick, 12 message, 13/14 reel stops, 15/16 closed-message jingles, 80 title, 81/82 floor, 83 ending).
- Sound driver command handlers SndCmdNN: named by command index; instrument/musical meaning not decoded.
  SndPitchSlide indexing overlaps neighbouring variables (layout unverified).
- Slot machine kinds 9/10/11: pay tables and symbol meanings not decoded; reel strip/table sizes inferred
  from the Mod22/Mod40/Mod44/Mod66 helpers.
- Blackjack hotspot 4 interpreted as surrender (returns half the bet); the halving uses LSR/ROL (possible
  original bug for bets >= 256). Insurance/double/split conditions read from code; exact payout ratios for
  insurance not verified.
- HotspotTest copies the player's map Y into NumValue while B is held -- apparently a debug leftover.
- Day/night palette on the clock screen (7:30-19:30) is an interpretation of the $3E palette writes.
- Rating thresholds of EndingTally are exact; the text of messages $10-$15 was not read.
- Text/tile table names in bank 6 ($B234-$BAB3) are from their consumers; internal layouts not decoded.
- InfoScreenAtTopEdge: meaning of the info screens (pay tables?) not confirmed visually.
