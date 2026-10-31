# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Concrete later redirect questions from commentary

- At physical $01E115 (CPU $E105 under effective fixed slots [14,15]), the classified path after animation-sequence sentinel handling compares MapId($3E) with $2B, selects MainState($60)=$11, Substate($C4)=0, clears $03AB and hides sprites. Entry into map $2B alone therefore does not test this sequence-boundary trigger. Inspect the state-$11 dispatch and the ordinary action/sequence preconditions before attempting a bounded clean-snapshot test; this may clarify the unreached boss/ending route, but its meaning is not yet established. Source: HandleSpecialSequenceBoundary; immutable code and handoff transition snapshot remain local. No map annotation proposed in commentary.
- The transition and render paths now expose exact consumer formats: PRG-zero $8200 + MapId*6 holds three pointer words; entrance records are indexed by EntranceIndex*8; mutable actor records are cartridge RAM $6400 + MapId*8. These are separate storage spaces. A later structural investigation may combine consumer/index bounds with existing evidence, without promoting selected table entries to a complete table automatically.
