# Current unresolved meanings and boundaries

Round 3 current-source readability is complete. Classification remains 65522/65552 bytes (99.95%); 30 bytes retain genuine structural ambiguity. Physical offsets include the 16-byte header. CPU aliases use effective PRG slots [0,1,2,3]; mapper values are separate from physical bank indices. Prior commentary is preserved in full under analysis/round-3/commentary-r2-preserved.

- Unknown physical 0x1A4E..0x1A51: three-byte JSR prefix performs two writes by returning into the target setup. The first clobbers the second coordinates; the independent single-write contract does not cover this prefix.
- Unknown 0x2707..0x2708: isolated $10 conflicts with the next supported instruction boundary. Neither execution nor data schema is established.
- Unknown 0x5E61..0x5E7A: 25-byte scratch/flag-locking prefix lacks an independent complete entry contract or data schema.
- Unknown 0x5E83..0x5E84: isolated RTS after a terminal loop lacks an established entry.

IncrementSelectedAnswerCursor at 0x2617 now has a static incoming-Y cursor contract. TogglePpuMaskForever at 0x5E7A..0x5E83 initializes A and alternates PPUMASK $18/$99 forever; both results keep Z clear. These established mechanics do not prove natural callers or an original diagnostic purpose. See analysis/r3-disassembly/review.md.

The newly accepted mixed data has practical bounded consumer/pattern review, summarized in source. Remaining format uncertainties concern exact grouping of controller masks, short lookup rows, auxiliary sprite lists, category transforms and individual audio substream extents. The rectangle/layout block has per-consumer width/height and row-stride contracts, not one schema for its entire storage. No per-byte semantic recovery is claimed.

The CHR-resident 503-entry high-first puzzle catalog partitions 93/112/114 CHR puzzles and 184 PRG puzzles. Static independent decoding matches every PRG record end to the next start and final 0x7F62 hook. This establishes the accepted data representation, without proving a complete naturally selected playthrough. CHR includes catalog/payload storage as well as tiles.

AnswerCompletionFrame, BoardFeatureIntroFrame and BoardFeatureStatusFrame now have explicit source mechanics and installer context. Their static contracts do not claim observed natural execution. A complete correct typed answer/natural win and the precise gameplay feature activation rule remain separate runtime questions; reuse preserved recipes rather than replaying unchanged early-game scenarios.

Sound commands have mechanical stream/register/modulation/envelope descriptions. Exact intended instruments, musical meanings, all selector domains and original audio organization remain uncertain. Computer response policy is described by category/square searches and difficulty-scaled timing; intended playing strength is tentative.

Zero-page scratch, winner/drawing tile fields, feature counters and OAM/history storage are reused by different phases. Caller-specific aliases document the established view, not exclusive ownership. Object arrays have six slots, player kinds four populated player slots plus sentinels, and sound records use a four-byte stride. AnsweringPlayer is one-based; CurrentPlayer is zero-based.

Concrete redirect questions remain the four residual structural fragments above. A materially stronger entry/return or data-consumer contract is needed before changing their roles; unchanged all-root discovery and early-game replay add no evidence. No phase redirect, terminal exhaustion, full compatibility or final editorial acceptance is implied. Final audit is intentionally pending.
