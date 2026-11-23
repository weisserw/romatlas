# Commentary uncertainties

Carried from round-1 commentary (analysis/round-1/commentary/uncertainties.md) and revised for the round-2 map
(maps/round2-disasm.json). Only items whose evidence changed were rewritten.

## Structural (phase 1)
- Round 2 classifies every byte. File 0x18010 (bank 6 $8000, value $EA) is now Data, the bank-6 leader byte
  (Static 0.8, by analogy with the $FF leaders of banks 0-5; no instruction reads, jumps or branches to it). The
  competing reading, an unentered NOP prologue, is noted in source (lead L-003).
- Unreferenced code (L-004): the 675-byte residue is "statically unreferenced, no known entry" (Static 0.8), not
  claimed dead. Labels keep the *_Unref / *_Unused / Unused* names from round 1: UnusedShowScreen0 $C679,
  BuildAnimatedSprite_Unref $E296, Load1bppFont_Copy_Unref $E49F, CopyPtr16ToPpu_Unref $FB10, Nmi_Unref_*,
  small math helpers (Lsr16x8_Unused, Negate_Unused, Times10_Unused, Mod6_Unused ...), FourReelSlot_Unref/Unref2,
  GetFloorAuxPtr_Unref, StopSound_Unref, MusicActive_Unref. Parts of round-1 fragments that round 2 showed reachable
  are now Observed or Static 0.9 with callers listed in the map evidence; their names were already descriptive.
- Generic data without a decoded layout: rest of 6:$B0EB-$BAD8 (named by consumer equates only), metasprite
  definitions 7:$C049-$C286 and $D084-$D12B (reached through MetaspriteTable pointers), 7:$F3DE-$F8F7 (no absolute
  operand reference; begins with a descending nametable-1 row-address table, then small byte records),
  the 32-byte table 6:$85CE and the 15-byte table $E195 (consumers not found).

## Data-record interpretation (round 2)
- Screen ids are from literal call sites (ScreenDecode/DecodeScreen*, ShowFlashingMessage, InfoScreenAtTopEdge
  callers). Screen $02 has no direct caller; $10-$15 are assigned to EndingTally ratings by the threshold code,
  and $11 is also shown after the title. The text of these message screens was not read.
- Songs $81/$82 (SongData1/2) have no literal PlaySound; round-1 call-site reading suggested floor music, unconfirmed.
  Each song stream ends with $FE pointing at its own id, so all four loop.
- Sound effect roles for ids $04-$08, $0D, $10 are unknown; the others follow the round-1 call-site names.
- FloorMapRows orientation (8 x 9 blocks) follows MapWidth/MapStride; row vs column order was not re-checked.
- FloorAux (10 zero bytes) is reached only through the unreferenced GetFloorAuxPtr_Unref.
- SpriteSetData's four animation tables are consumed only by the unreferenced BuildAnimatedSprite_Unref.
- Round 1 said LoadSpriteAnimSet decodes sprite CHR to $1000; the chunk-type table gives PPU $0000 (fixed in source).

## Semantic (tentative wording used in source; carried from round 1)
- Sound driver command handlers SndCmdNN: named by command index; instrument meaning not decoded.
- Slot machine kinds 9/10/11: pay tables and symbol meanings not decoded (branch names such as
  EvaluateLineTypeB_CheckSymbol4Pair use symbol numbers, not pictures). FourReelPayTable is 54 bytes (27 words);
  the round-1 66-byte equate overlapped FourReelColumns/FourReelStripPtrs and is corrected.
- Pay-line offsets 0/2/4 used by SpinThreeReels* (labels *_EvalLine0) are not mapped to screen rows.
- Blackjack hotspot 4 interpreted as surrender (returns half the bet); the halving uses LSR/ROL (possible original
  bug for bets >= 256). Insurance payout ratio not verified.
- Roulette bet-spot areas (zero column, column bets, number grid, dozens row, even-money row) are inferred from the
  RouletteFindBetSpot geometry and bet ids.
- HotspotTest copies the player's map Y into NumValue while B is held -- apparently a debug leftover.
- Day/night palette on the clock screen (7:30-19:30) is an interpretation of the $3E palette writes.
- InfoScreenAtTopEdge screens ($18-$1D) are per-game info pages; content not confirmed visually.
