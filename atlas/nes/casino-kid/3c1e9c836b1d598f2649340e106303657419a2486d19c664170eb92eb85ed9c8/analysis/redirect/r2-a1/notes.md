# Redirect round 2, attempt r2-a1 (input checkpoint pass-003)

Input: maps/current.json == pass-003 (1582 regions, 0 unknown bytes; refscan.txt line 1).
Exact unknown complement: empty, so there is no ledger to keep. Only research and lead records
changed. ROM, map, state, commentary and binary are unchanged. Runtime was not used:
every question below was settled by static reading, or needed only provenance.

## A. L15 (2) Password packing: PackMoneyAndProgress (bank 5 $8429) and UnpackPasswordFields (bank 6 $891B)
Static read of the encoder and the decoder together:
- Each of Money ($055D), PokerOppMoneySaved ($0561) and BjDealerMoneySaved ($0565) keeps
  bits 2..26 only. The decoder masks the low byte with $FC and the top byte with $07, so each
  field is 25 bits. Amounts are in cents, so they are multiples of 4, and they stay below 2^27.
- Packed layout in PasswordBytes $05B0-$05B9 (bit 0 = $05B0 bit 0):
  - Money bits 2..26 go to packed bits 0..24 (Money >> 2).
  - PokerOppMoneySaved >> 1 goes to the 4 bytes from $05B3. Its bits 2..26 land in packed bits
    25..49. $05B3 bit 0 takes PokerOpp bit 1, and that bit is masked on decode.
  - BjDealerMoneySaved is ORed in unshifted at $05B6-$05B9, so its bits 2..26 land in packed
    bits 50..74. Its bits 0..1 share $05B6 bits 0..1 with PokerOpp bits 25..26. Those bits are
    masked on decode, and they are zero for amounts that are multiples of 4.
  - Decoder field i copies PasswordBytes[3i..3i+3] and shifts left by 2-i.
  - Encoder detail: @clear_merge zeroes only CasTmp7..9 before the OR.
    CasTmp6 keeps the top PokerOpp byte.
- So the "OR merge" is a non-overlapping concatenation of three 25-bit fields (75 bits used of
  80). PasswordBytes+10..13 is OpponentProgressBits, copied verbatim.
- Result: the uncertainty "exact bit layout not traced" is resolved, for commentary only.

## B. L15 (3) PkOpponentTriples +0 and BjDealerRecord +10
- refscan.py checks absolute operands with readable or writable opcodes.
- PkOpponentTriples ($9F57, bank 4):
  - Only 2 references exist in banks 4 and 7, both `lda PkOpponentTriples,x`.
  - The X values are 3*Opponent+1 and 3*Opponent+2 (source @force_player_all_in /
    @force_opp_all_in).
  - There is no word or immediate pair pointing at $9F57 (byte scan).
  - LoadOpponentRecord's pointer read covers 9 x 9 bytes, which end at $9F56.
  - Field +0 is zero in all 9 entries.
- BjDealerRecord ($07A9 RAM copy):
  - Bank 3 reads by operand: +0 (copy loop, and the ,y read with Y=$11), +8, +9, +11..+16.
  - +10 ($07B3) has no reader anywhere in the ROM.
  - The other bank 4-7 hits are reuses of the RAM in other scenes, or byte coincidences inside
    other instructions. Two bank 3 hits, at physical 51569 and 52662, lie inside other
    instructions, not at instruction starts.
  - +10 is the same value in all 8 records.
- Result: both fields are constants with no reader found. That is a mechanical description,
  not a meaning. This is commentary wording only, with no region change: the table extents are
  fixed by their consumers.

## C. L15 (1) Blackjack result codes 1/3/8
The writers are explicit in static code:
- 1: BjSurrenderAction @do_surrender, after halving BjBet.
- 3: BjSettleHand, when the dealer hand status is 2 and the player's is not.
- 8: BjCompareHands @doubled_win, when BjDealerResult is nonzero, meaning a doubled hand that
  won by a dealer bust or a higher total.
Runtime confirmation would only add execution provenance for code that is already classified.
It was not run (TK-004 cost; not decision-relevant).

## D. Re-check of the other open leads (no new evidence)
- L4/L7 unreferenced fragments: reachability only. Their code classification rests on static
  structure (round-1 targetaudit.txt, inlinecheck.txt). No new reference method exists.
- L5/L6 format semantics:
  - L6 fixed-bank tables already carry region comments and cursor-table notes in the round-2
    commentary.
  - The record formats in banks 0-2 and the music command bytes are labels and semantics only,
    with no code-versus-data question. Bounded inspection was done in r1-a1 and commentary
    round 2.
- 5624 static-only code bytes: provenance only (round-2 disassembly handoff). They have no
  contradicting observation.
