# Exact engine board/history RAM stores

Ten store sites have a supported ordinary engine calling precondition that bounds Y independently of arbitrary8-bitvalue.

8266/826D/828F (ApplyMove) and82AF/82B8/82CE (UndoMove): Y comes from generated source/destination or the castling/displaced-capture helper. Supported position precondition has valid0x88source/destination squares0..77; adjacentcastlingfiles remaininhome-rank and displacedcapture removesone validrank. Generator explicitly rejectsAND88nonzero. Therefore addresses0004+Y stayRAM0004..007B, neverCPUstack. This is a routineprecondition, not claim all arbitrarypatchedframes satisfyit.

8485/848A: X<69 gate bounds currentframe/depth in initialized38+10n searchframes; Y loaded from8B84+depth. Four correspondingtablevalues C2/92/A4/B6 all≤C2; destinations0004/0005+Y≤C7. NoCPUstack/mapper.

84B9/84BF: CommitCandidateState setsY0A, two ShiftHistoryWords increments eachbytwo, advances0C and checks6A; writes04+Y/05+Y for boundedhistorywordslots≤69. Unknownhelper84C3 has separate source/dest fieldcopy throughzero-pageindexed addressing(wraps), notabsoluteCPUstack.

Consumer/body/listing: reading-after.asm EngineApplyMove,EngineUndoMove,EnginePushMoveFrame,EngineCommitCandidateState,EngineShiftHistoryWords; immutable exact ten-siteSTA opcodes asserted. Only these ten absoluteindexedstores override prior blanketindexrange; no arbitraryabsolute/indirectwrites permitted. Allotherguardsremain.
