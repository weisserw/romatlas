# Remaining meanings and structural questions

Current-source readability covers pass-002. Classification remains 65,550/65,552
bytes; rounded 100.00% is not completion. CPU PRG context is fixed [0,1,2,3].
Prior uncertainty history is preserved in
analysis/round-002/commentary-round-001/uncertainties.md.

- Physical $2991/CPU $A981 and physical $44B2/CPU $C4A2 each store $60.
  Source retains raw bytes and explains spare RTS versus data/padding ambiguity.
  No supported entry contract discriminates those roles. Reuse the localized
  analysis/redirect/r001-static-contracts/residual-review.md; artificially executing
  RTS would not establish its intended storage role.
- Primary/secondary lifecycle, initializer, effect, condition and movement callback
  structure is now classified and explained. Natural coverage of every type/effect/
  event selector remains separate. Recovered mechanical names do not identify all
  entity types, item names, outcomes or game-purpose meanings of type-specific fields.
- Inventory producers $A769..$A781 explicitly encode values 1..7 into three
  bitplanes. The six attribute bytes at CPU $8C36..$8C3B do not cover 6/7; those
  selectors read the following code bytes $A9/$10 at $8C3C/$8C3D literally.
  Natural effect selection remains unresolved. Preserve the code/data boundary.
  CPU $9CA7 still reads its mask from zero-page $FC (bytes $25,$FC), whose value
  contract is unknown; no repair or immediate-mask substitution was made.
- Ownership condition entries $A94B/$A94F/$A953 preset Y, but $A955 immediately
  overwrites it with A and tests incoming A against ownership. Do not describe
  these as unconditional tests of the preset bit. Lives condition $A93D compares
  against five while increment helper $A6CB caps at six; both literal behaviors
  are explained without guessing a correction or intent.
- $B986 retains exact unofficial immediate NOP bytes $89,$20. Its following
  branch tests the zero flag from INY, not a frame-byte sentinel. $BA92 is accepted
  under an extra stacked-return precondition; no actual caller is claimed.
- Generic animation parameters $B37D..$B3D4 and unproved secondary record fields
  retain bounded data explanations. $C47C..$C4A1 is repeated generic non-code
  storage, not a proven callback table. $C469 is shared by tile-seed and scrolling
  stream reads. The 256-byte scalar table has established consumers/extent but no
  recovered generator or natural distribution.
- Prior stage/audio/CHR asset review remains valid. Source explains known consumer
  formats, while complete asset subtable layouts and artwork identities remain
  unresolved. Static code structure does not establish natural execution of all
  paths or full hardware/game compatibility.

These are explained limitations, not deferred readability ranges. Final editorial
acceptance remains pending for the terminal stage. No new toolkit issue; NES-001's
buttons-array workaround and history remain in analysis/toolkit-issues.md.

Final editorial acceptance completed separately; no unresolved readability ranges
remain. Supported audit and fresh byte-identical rebuild passed, without source
changes. The semantic/structural uncertainties above remain as stated. Latest
residual rejection review is analysis/redirect/r002-residual-contract-review/rejection-review.md.
