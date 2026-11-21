# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 001 reviewed limitations

All current classified source has local mechanics/format explanations. Remaining
uncertainty is recorded rather than filled with speculative game identities.

- Bank 1 object commands use six-byte records and a synthesized RTS continuation.
  Many classified fragments end before an unresolved continuation or inline payload.
  Their mechanical names describe movement/phase/rendering behavior; exact enemy
  identities and complete handler lifetimes remain unresolved. The $b995 sound-rule
  consumers are understood, but unknown slices interrupt the rule storage.
- Audio and ending code uses effective PRG context [12,13,14,15]. Audio tokens
  dispatch through $9dd8; stop handlers deliberately discard return frames. The
  $9e3a handler removes one return address, and $a05d removes three. These local
  contracts do not prove that every other handler has the same stack behavior.
  Audio/actor selector meanings and the full sequence table subdivision remain open.
- Fixed terrain tables have distinct roles: bank-local $8500 supplies response enums;
  $8600 supplies geometric occupancy selectors. SampleTerrain returns response in X
  and shape in Y. Numeric terrain enum meanings stay mechanical where no stronger
  game interpretation follows. This distinction prevents accidental schema reversal.
- Fixed metasprites have count/extent headers and four-byte OAM components. Shared
  scratch variables and overlapping audio register arrays have local aliases; a name
  does not assert exclusive allocation or the original author's naming.
- Generic non-code slices retain inspected physical bounds and useful known consumer
  roles without claiming a complete internal schema. Unknown slices remain unknown;
  short apparent code, legal opcode decoding and pointer-shaped values do not settle
  their role. See ../analysis/unknown-ranges.json for the exact complement.

No readability backlog remains for currently classified bytes. Final editorial
acceptance is still pending; future classification changes require carrying valid
comments/review by physical identities and rechecking affected dependencies.
