# Commentary uncertainties (final acceptance, checkpoint pass-003)

These are limitations that have been investigated. Each one is explained in the source,
next to the code or data it concerns.
Items that were resolved in round 2 are listed at the end, so their history is kept.

## Open

- **Fixed-bank library.** Many conventions match the Famicom Disk System BIOS: the $FF/$FE
  mirrors, $FC/$FD scroll, the VRAM buffer at $0300, the VramStructWrite format, and the
  FDS_* register writes in InitHardware. Behaviour was read from the code. The FDS origin
  is a hypothesis.
- **$CFE6-$CFF6 (text printer remnant).** This is now fully mapped as static code
  ($CFF6 PHA, lead L8). Nothing reaches it, so its reachability is unresolved.
- **Unread constant fields.** BjDealerRecordTable +10 (2 in all 8 records) and
  PkOpponentTriples +0 (0 in all 9 entries) have no reader found by operand and pointer
  scans (analysis/redirect/r2-a1/notes.md B). They are described mechanically; their
  meaning is unknown.
- **Bank 4 poker variables $0714-$075A.** Their roles (pot, bets, streaks, personality
  nibbles, RiggedDeal) are inferred from arithmetic call patterns. The HandOutcome and
  HandWinner codes are only partly inferred.
- **Music driver per-channel arrays ($0520-$0558).** These are now named by their use in
  the code. The meaning of bits 4-6 in MusicAttackBits, MusicDecayBits and MusicRestBits
  ("stage running" / "envelope frozen") is tentative. Command bytes beyond notes, rests,
  end, jump and loop are not decoded.
- **Data formats.** These are described at region or table level only, and per-record
  formats remain undecoded (leads L5, L6):
  - banks 0-2 graphics;
  - bank 3/4 screens;
  - bank 5 map/metatile data;
  - bank 6 music and credit data;
  - the sound-effect step tables, which are now labelled per effect.
- **Unlabelled text spans.** All 3 spans named in the round-2 handoff now have consumers
  and labels: BjUiStrings, the PkText* UI strings and the PkTextGreeting group. Some
  strings inside them are reached only through pointer tables (BjLabelArgs). Their
  layout bytes are not decoded.

## Resolved in round 2

- **L9.** EvaluateHand category 8 is the intended "ROYAL STRAIGHT" rank. It is not a bug.
- **L10.** Blackjack was observed:
  - The hit-assist swap (BjHitAssist, formerly BjCheatActive) helps the player's HIT.
  - The Select menu is BET ALL MONEY / FINAL HAND.
  - The payout routines were renamed after their status codes and factors.
- **L11.** On the debug monitor's music and SFX pages, A moves to the next number and
  B to the previous one.
- **L12.** Bank 6 $8013 (Api_EnterBrokeScreen) shows GAME OVER. The KING password uses
  Api_EnterKingMatch.
- **L13.** There are 4 cheat strings, each 30 bytes long; 28 of them are compared. They
  are at bank 6 $87E2 (PresetPw*) and bank 5 $93DF (CheatPw*). A match only lets the
  password screen accept the entry; bank 5 CheckCheatPasswords applies the effect.
- **Branch labels.** All 966 mechanical @loop_/@skip_/@case_ labels from round 1 have
  descriptive names.

## Resolved at final acceptance (lead L15, static reading)

- **Blackjack result codes.** Every code 1-9 has an explicit static writer: 1
  BjSurrenderAction, 2 BjDrawCardToHand, 3 BlackjackSession (dealer-only blackjack after
  the deal), 8 BjCompareHands @doubled_win, the rest in BjCompareHands/BjResolveHand.
  Codes 1, 3 and 8 were not each seen at runtime; that would only add provenance.
- **Password bit layout.** PackMoneyAndProgress keeps bits 2..26 of Money,
  PokerOppMoneySaved and BjDealerMoneySaved and packs them without overlap into bits
  0-24, 25-49 and 50-74 of PasswordBytes 0-9; bytes 10-13 are OpponentProgressBits.
  The OR merge only joins the shared edge byte, whose overlapping low bits the decoder
  (UnpackPasswordFields) masks.
