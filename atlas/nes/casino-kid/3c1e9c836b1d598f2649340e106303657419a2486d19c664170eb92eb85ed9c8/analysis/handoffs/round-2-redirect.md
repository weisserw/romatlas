# Round 2 redirect handoff (attempt r2-a1)

## Scope and identity
- Input and output checkpoint: pass-003 (maps/current.json, sha256 9e2a7bca...22f2).
  - The ROM is 100.00% classified: 0 unknown bytes and 1582 regions (r2-a1/refscan.txt).
  - Code is 31825 bytes: 26201 Observed and 5624 Static. Data is 99263 bytes.
- Redirect changed only the research and lead records. ROM, map, state, commentary/source and
  the configured binary are unchanged.
- There were no runtime sessions. Research is in analysis/redirect/r2-a1/:
  - notes.md, sections A-D;
  - refscan.py and refscan.txt;
  - leads.before.json.
- Proposed bytes: 0. Accepted bytes: n/a.

## Findings (L15, all from static reading)
- **Password packing (bank 5 PackMoneyAndProgress, bank 6 UnpackPasswordFields).**
  - Money, PokerOppMoneySaved and BjDealerMoneySaved each keep bits 2..26 (25 bits).
  - They are packed without overlap into PasswordBytes 0-9, at bits 0-24, 25-49 and 50-74.
  - Bytes 10-13 are OpponentProgressBits, stored verbatim.
  - The "OR merge" only joins the shared edge bits, which are masked on decode.
  - This resolves the open uncertainty.
- **Constant fields with no reader.**
  - PkOpponentTriples +0 has only two readers, and both use +1/+2.
  - BjDealerRecordTable +10 has no operand reader in the ROM.
  - Each field holds the same value in every record. No meaning is claimed.
- **Blackjack result codes 1, 3 and 8.** Each has an explicit static writer:
  - BjSurrenderAction writes 1;
  - BjSettleHand writes 3 (dealer blackjack);
  - BjCompareHands @doubled_win writes 8.
  - A runtime check would only add provenance.

## Ready leads
- **L15-residual-field-semantics (low).** Commentary only, with no map or region change. The
  recipe is in leads.json. It fits the final editorial audit and does not need a disassembly
  pass.
- **Structural or label leads for disassembly:** none.

## Rejection review (evidence-policy.md "Challenge a terminal decision")
The unknown complement is empty, so no candidate leaves a code-versus-data question unresolved.
Each residual is reviewed below for its actual claim.

1. **5624 static-only code bytes** (mainly bank 4 AI/King branches, bank 5 floor/people, bank 7).
   - Claim needed: Code role. Static flow, the stack/inline contracts and the round-1 audits
     already establish it: 0 observed starts fall off instruction boundaries
     (r1-a1/coverage-all.json, inlinecheck.txt, targetaudit.txt).
   - Strongest competing reading: some of these bytes are data. No observation, jump target or
     inline contract supports that.
   - More traced play changes provenance only, at about 10x cost (TK-004).
   - No proposal results.
2. **L4/L7 unreferenced fragments and the $CFE6-$CFF6 remnant.**
   - These are classified by internal structure: the stack bracket and the JMP/BNE targets
     (round-2 disassembly L8).
   - The decisive negative was the r1-a1 byte scan, which found no JSR/JMP operand. It rules
     out a direct caller only. It does not rule out code, and it does not claim the fragments
     are dead.
   - The only open question is reachability. The remaining method would be an indirect
     dispatch, and none was found in the 126 audited inline jump-table words.
   - No reclassification is possible or needed.
3. **L5/L6 data formats** (CHR/screen records in banks 0-2, fixed-bank tables, music command
   bytes).
   - These bytes are already Data, and the consumers anchor the table bounds where known.
   - Decoding them would only add semantic labels. Generic data is a sufficient role under
     the policy.
   - The bounded inspection is recorded in r1-a1 notes and in the round-2 commentary.
   - What would change this: a consumer-anchored record decode. That gives labels only and is
     not a classification gain.
4. **L15** was resolved statically (above). It is now a commentary edit.
5. **Gameplay.** The following were visited (analysis/scenarios.json):
   - every milestone, including blackjack, the King, the ending/credits, GAME OVER, the debug
     monitor and the password round trip.
   - Remaining gameplay items:
     - S9, the RICHMAN cheat: deferred as low value. Its effect is already read statically
       (CheckCheatPasswords).
     - FINAL HAND timing and the natural "all beaten -> King" trigger: the King match itself
       was observed, so these are reachability or provenance only.

Conclusion: no revised inference or test changes classification, boundaries or region
types/labels. The only material improvement left is L15 commentary wording.

## Remaining requirements before final acceptance
- Apply L15 in the commentary notes (tools/cmt.py build, then commentary.py verify,
  byte-identical).
- Run the reserved final editorial audit: review.json editorial_audit is currently pending.
  Include the naming/comment gaps in previously reviewed ranges.
- Final verification:
  - map.validate and generate/verify on pass-003;
  - byte-identical rebuild;
  - commentary.py verify;
  - export the ROM-free portable map only if it is a requested deliverable.

## Recommendation
Terminal for classification and research: no disassembly or commentary cycle is needed for
structure. Proceed to final acceptance (editorial audit with L15 applied, then final
verification).
