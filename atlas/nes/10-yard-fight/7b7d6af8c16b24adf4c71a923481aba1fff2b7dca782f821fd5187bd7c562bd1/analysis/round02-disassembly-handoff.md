# Round02 disassembly handoff

**All six ready leads verified; whole-ROM structural classification now complete. Overall workflow incomplete pending refreshed full-source commentary and final editorial audit.**

ROM classification: 100.00% (40976 / 40976 bytes classified; 0 bytes unknown).

Fresh checkpoint **pass-003** validates and rebuilds byte-identically. Current/internal/saved map SHA-256 `a2c637a340703bcd0cbafb8a4e8106d75d5ddbfa82060fd22bd82d615ca09960`. [Verification](pass-003-verification.json), [current map](../maps/current.json), [generated source](../generated/pass-003/game.asm). Internal state saved at analysis/state.json. Input unchanged SHA-256 7b7d6af8c16b24adf4c71a923481aba1fff2b7dca782f821fd5187bd7c562bd1.

Accepted **1,884 Code +216 Data** in 35 exact nonoverlapping complement segments, all Static; totals Code13,174/Data19,610/Image8,192 in **133 internal regions**. Header16, all four8192-byte PRG banks and8192-byte CHR fully classified. Exact per-bank kinds/counts: [counts](evidence/round02/counts.json). R01-R01..R01-R06 verified with appended attempt histories. No natural reachability newly asserted; no execution coverage gain/replay.

[Independent review and individual lead decisions](evidence/round02/review.md), [fresh exact instruction decodes](evidence/round02/fresh-decodes.json), [immutable Data bytes](evidence/round02/data-bytes.json), [annotation results and protected commentary hashes](evidence/round02/acceptance.json). Previous redirect findings supply primary contract reasoning; exact physical/bank endpoints independently redecoded with configured binary and effective banks [0,1,2,3]. Reconnaissance decodes not annotated wholesale. Generic Data used for bounded patterned animation bytes whose full indexed domain remains unproved.

Short sentinel and mode-gated audio wrappers independently supported by matched shared returns, fallthrough endpoints and established consumer semantics. Anomalous $CC0E STA $CC18 prefix accepted narrowly as code: target is exactly high operand of following indexed LDA, adjacent to finite64-byte RAM-to-OAM loop and established PPU restore tail. On immutable mapper0 the store does not change source page. No working self-modification, RAM relocation, historical intent or natural caller claim. Caller must supply OAMADDR; this copies64 bytes, not full256-byte OAM.

**Commentary files all preserved untouched.** Existing commentary remains the pass-002 view with old opaque sections; next worker must refresh against pass-003 and perform full current-source coverage review. Existing source hash7685e3a7f6e442e1e929ea05d0baea2b7641c0e699902c8239f88921f448c560 and reading-after hash27d87ced29077a79b033285880674ba3de7185bf6ca67493c73c4fdc130ca081 unchanged. Previous commentary work must be retained/reconciled. Final editorial audit remains mandatory; zero unknown is structural result only.

Broad gameplay routes/evidence already cover all five normal skill levels plus artificial terminal win and were reused; no changed question warranted replay. No further structural residual or interrupted investigation. Semantic state identities, natural production of all selectors, unlinked routine callers and prefix history remain honest uncertainties, not unclassified bytes.

No new toolkit issues found. Existing [NES-INPUT-001](toolkit-issues.md) confirmed nonblocking input API mismatch remains. No toolkit edits, publication/archive/deletion, new snapshots or asset downloads. All ROM/assets/state/source local; no compatibility claim. Handoff disk12GiB free, existing scratch92MiB, this evidence batch524KiB.
