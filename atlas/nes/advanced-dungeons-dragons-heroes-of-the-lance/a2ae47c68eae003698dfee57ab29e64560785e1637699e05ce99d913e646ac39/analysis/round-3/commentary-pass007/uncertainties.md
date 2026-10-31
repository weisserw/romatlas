# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Concrete later redirect questions from commentary

- At physical $01E115 (CPU $E105 under effective fixed slots [14,15]), the classified path after animation-sequence sentinel handling compares MapId($3E) with $2B, selects MainState($60)=$11, Substate($C4)=0, clears $03AB and hides sprites. Entry into map $2B alone therefore does not test this sequence-boundary trigger. Inspect the state-$11 dispatch and the ordinary action/sequence preconditions before attempting a bounded clean-snapshot test; this may clarify the unreached boss/ending route, but its meaning is not yet established. Source: HandleSpecialSequenceBoundary; immutable code and handoff transition snapshot remain local. No map annotation proposed in commentary.
- The transition and render paths now expose exact consumer formats: PRG-zero $8200 + MapId*6 holds three pointer words; entrance records are indexed by EntranceIndex*8; mutable actor records are cartridge RAM $6400 + MapId*8. These are separate storage spaces. A later structural investigation may combine consumer/index bounds with existing evidence, without promoting selected table entries to a complete table automatically.


## Round 2
Newly classified ranges are under active review; prior uncertainties retain their historical context.

## Round 2 resolution and remaining meanings

The historical state-$11 lead above now resolves to the result-screen dispatcher and staged ending/result display. New source documents its input latches, party/inventory arithmetic, saturating totals, row writers and text selection. Artificial scenario evidence does not establish natural dragon defeat, item possession or a winning route. The frame-boundary trigger remains a useful contract rather than a gameplay-completion claim.

Mechanical oddities are preserved: CPU $C507 reads sixteen palette bytes without a PPUDATA store; PRG5 $96F7 resets Y and emits the same sixteen source bytes twice; dragon sprite Y jitter is overwritten before output while X jitter remains; the last DMC hardware sample includes the first $FD80 opcode. These are game-source observations, not toolkit bugs or authorization to repair bytes.

New generic room/entrance, sequence/sprite, audio and tile containers reuse the bounded reference/pattern/boundary inspections linked in phase-1 evidence. Full mixed-record schemas, original tile/animation/song/sample identities and exact palette themes remain unresolved. Generic non-code is intentional where specific record boundaries are unproven. Numeric switchable-bank calls in fixed code retain encoded CPU aliases when a single target is not established by their local context.

Concrete residual consumer questions, without structural edits: PRG2 $9D0F shape-pointer domain and record extents; PRG4 $B865 partial-meter selector bounds; PRG5 $942C/$9658 weight selectors and unclassified table extents; fixed $F3FC+8*AreaSet palette selector bounds. The relevant source/Unknown boundary notes distinguish consumer knowledge from classification. Reuse current residual intervals and existing snapshots before new bounded work. No pending classified-source readability remains.
