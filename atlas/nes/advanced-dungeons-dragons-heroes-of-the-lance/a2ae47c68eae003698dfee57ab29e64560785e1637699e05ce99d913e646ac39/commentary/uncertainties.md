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


## Round 3
Newly classified ranges are under active review; prior uncertainties retain their historical context.

## Round 3 current-source resolution

The full pass008 classified source is now reviewed. Historical references above to unclassified tables or active Round 2/3 review describe earlier checkpoints, not a current source backlog. The result inventory weights now have exactly eleven words (22 bytes); the object contribution lookup has fifty words. Shape, meter, palette and mixed-container selector domains remain limited to their actual consumers. Original item/spell/actor/artwork/music names and complete mixed-data schemas are unresolved where source uses mechanical names or bounded generic notes.

Only physical $008345, $011CF8, $018267 and $018D4D remain Unknown, one $60 byte each. An RTS and a separator remain competing roles. Neighbor code and pointers do not establish a supported entry or distinguishing consumer. Current source explains all four boundaries without classifying them. This is a localized structural question for the separate redirect phase; it does not require another semantic-research pass over the classified source. Reuse analysis/disassembly/r03-20261031/residual-check.json and analysis/redirect/r02-20261031/residual-review.md for the next bounded rejection assessment.

Further concrete source behaviors are preserved: bank-four save-load field $19 stores live $031D into the slot; inventory subcopies transfer 88 contiguous bytes without a sixteen-byte party stride; nearby-record collection has no eight-entry insertion clamp. Chained ADC contributor sums propagate carry into the next contributor. The second effect's second-hit damage path indexes $03AC instead of its own $03AD; the second action-three initializer clears $0368 rather than $0369. These are mechanical observations, not repairs or toolkit bugs. Validity and input bounds remain explicit caller contracts.

The sixteen sequence commands and shared single return slot are complete mechanically; exact musical meaning of fields $16/$18 remains unresolved. All eleven map/entrance transition hook records and target bodies are explained, but natural invocation of every hook and a clean natural quest/dragon/win route remain unproved. Effective physical PRG contexts follow literal switches and returned bank state; a CPU alias alone does not identify storage. Numeric aliases remain when no unique target follows from the available context.

No new emulator experiment, classification edit or toolkit change was needed. No pending classified-source readability ranges remain. Final editorial acceptance remains pending for the coordinator after redirect's terminal assessment.
