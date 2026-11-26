# Independent pass 002 structural review

All proposals accepted; none rejected. Fixed CNROM effective PRG slots [0,1,2,3]. Physical offsets include header. Structural Static annotations assert no new runtime/natural reachability.

## D001 — accepted

Attract reader at physical286/292 consumes prefix as duration/input pairs; noncode prefix ends at two 13-byte setup helpers. Helpers set fill tile $33 and high address $20/$28, constant BNE to accepted $81AD fill consumer; sibling setup comparison. Full attract selector extent and natural use unresolved.

## D002 — accepted

Exact X<32 palette reader; bounded OAM setup and 32/8/6/7-byte consumers tile data interval; local score renderer calls shared six-tile writer, accepted numeric conversion and balanced nibble helper. All boundaries independently checked, external reachability unresolved.

## D003 — accepted

Complete adjacent 14-byte pointer increment clone with zero-page operands translated +3; carry propagates low to high, balanced RTS and no inline data. Calling contract static, caller/natural use unresolved.

## D004 — accepted

Patch setup encodes JSR $C69B in exact existing 3-NOP NMI slot and JSR $9DFE at accepted NMI instruction boundary, returns via reset. Internal hook and frame&3 four-handler finite domain establish complete OAM-quarter uploader. PRG backing immutable on CNROM; historical debug purpose inferred, natural use unresolved.

## C001 — accepted

Twelve-word pitch reader domain note&15<12 doubled gives exact24-byte big-endian timer span. LDA #0; STA $11; BEQ $A475 cannot fall into this prefix. Correct previously false Code prefix; no accepted conflicting entry.

## C002 — accepted

Frame&15 doubled reads sixteen exact little-endian road pointers. Y&7 indexes eight nonzero one-hot masks; OR sets nonzero, STA preserves flags, BNE $8622 cannot fall into first pointer byte. Correct false Code tail.

Primary review: independently decoded ROM slices in independent-decodes.json plus redirect bounded routine decodes checked against input/game.nes and maintained source consumer bodies. New unknown coverage429 Code/101 Data, corrections13 Code to Data. No gameplay replay: saved broad stage-loop scenarios already answer progression question. No terminal/editorial claim.
