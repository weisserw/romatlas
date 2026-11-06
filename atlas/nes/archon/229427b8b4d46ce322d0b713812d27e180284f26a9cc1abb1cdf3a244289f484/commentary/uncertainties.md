# Current commentary limits

All currently classified source is explained in source/game.asm. Structural classification remains separate from readability: 23122 Code bytes and107958 Data bytes are classified; eight bytes in six intervals remain Unknown. Each residual has an exact physical boundary and an honest local note. The remaining residuals still lack a discriminating entry or data contract; the former two-byte residual at $00ABF is now explained as the sign-selected zero Y-origin adjustment table.

Natural reachability remains narrower than the static inventory. Mechanically supported menu/display tests, audio demonstrations, alternate loaders and finite dispatch handlers are described without claiming normal gameplay execution. Effective execution views retain physical PRG bank context; CPU aliases are not storage identities.

Generic non-code data remains in bank envelopes and in actor graphics-slot tails. Whole-bank bitplane/repetition inspection and existing upload consumers support Data, but complete original tile/map/music container subdivisions are unresolved. The twenty actor graphics prefixes are exactly $0300 bytes as proven by the buffered uploader; the distinct direct uploader copies $0800 and is not prefix-extent evidence.

Music commands and effect streams have mechanical names and bounded consumers. Complete song/instrument identities, exact strategic intent of several computer thresholds and some actor/spell meanings remain tentative. Source explains established control flow and state changes without requiring the author's original terminology.

Shared CPU RAM changes role by phase. AI candidate arrays overlap private audio-demonstration fields; movement counters also hold survivor counts, and common scratch pointers serve board, terrain, patterns and text. Context-specific aliases are reading aids, not independent storage allocations. Physical $1F684 remains the observed alternate TXA entry inside the BIT operand at $1F683.

Carried coordinate aliases were rechecked: $61=row,$63=column; $60/$62=selected row/column; $6B/$6C=candidate row/column. ActorPixelX=$0660 and ActorPixelY=$0664 are separate from board coordinates. New source uses explicit offsets for understood second-actor fields.

Cancellation is the eighth visible pointer at CPU $DC69, targeting $E202, but there are seven spells. CPY7/BEQ $DCE2 bypasses the used-spell write and still reaches the dispatch. The target was already classified; this corrects wording without structural gain. See ../analysis/round02-commentary-questions.md.

The Data label at physical $004CF originally suggested obstacle placement. Its three twenty-byte arrays actually feed computer reach/avoidance fields at CPU $82C6/$82D4/$82ED. Source uses ComputerArenaDistanceParameters and retains the established table aliases. The current map now agrees with this name and consumer evidence. The third array supplies $067A,X and then $0678,X by adding three.

Final editorial acceptance remains pending. No final audit, publication or archive was performed; all source/assets/evidence remain local.
