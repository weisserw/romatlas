# Round2 independent structural review

This review uses the unchanged input ROM and accepted round01 map. Prior commentary
is retained in unchanged commentary/source and the superseded map records retained
in data-changes.json. Instruction comments for wrongly decoded data are historical,
not endorsements of their old interpretation.

Data:
- C01 physical000328..00115C: renderer81B4/82AD indexes two paired-pointer spans.
  Independently decoded142 distinct count+4N sprite records tile00047A..00115C
  exactly; endpoints and paired target spans agree. 000326..328 is genuine90 F2 BCC
  at renderer tail (CPU8316, also dummy selector-zero pointer alias), preserved.
- C02 physical0040BF..40CF: consumer809D copies80AF,Y throughFF; preceding BNE is
  unconditional after LDA10.0043EB..43ED is signed coordinate delta data; direct
  indexed ADC83DB,Y and preceding unconditional branch discriminate data from code.
- C03 physical01DF49..1E063: DE1E/DEAE/DEBE access direction/frame/attribute and
  descriptors; 64 initial direction-record bytes, then mixed patterned animation
  data ending at independently supported E053. Do not impose one overlapping schema.
  Exactly2 existing Code bytes are wrong, not3.
- PPU streams: independently reparsed15 finite streams from actual pointer/bank
  tables, checked every retarget destination, token length and FF endpoint. Token7F
  retargets, high-bit literal copies length&7F bytes, other tokens repeat one byte.
  Stream0 fixed CPUC8CA maps fixed physical01C8DA regardless source-bank table0.
- PPU chunks: separately decoded bank2 CPU8001 and bank5 CPU9759. Each compares
  accumulated source cursor to encoded endpoint and clamps final copy. All17+6
  encoded source/count extents have contiguous union up to physical00A99E/017799.
  Bank2 selectors include overlapping/suffix source aliases; no ordered-record
  claim.17 spec entries plus payload start0080A7;6 specs start0157FF.
- Templates: actual C8FD/C91A comparisons recognize FF return, FE terminator and FD
  continuation.36 pointers /35 starts include suffix alias. Independent parse ends
  exactly at01CC9F, following supported routine. Not an FF-only scan.
- Dispatch: C79A PLA/PLA consumes its caller return, loads a selected LE word and
  indirect-jumps; there is no ordinary return into inline data. Accepted bounded
  lists use independent next-handler endpoints and coherent selected graphs.
  Pointer-pattern prefixes are generic non-code; no original full selector domain
  or intended use is inferred. ActorAE42 uses13 entries; sceneC47A8 before JMP C48A.
  Tiny masks/deltas/palette selectors/effect arrays use direct indexed consumers.
  See primary manifests for exact offsets and contexts; every claim retained.

Code review frontier: all6674 unique proposed instruction starts (6789 explicit
contexts;14500 unique bytes) match independent configured-toolkit decoding, with
zero overlap with data or accepted regions.762 starts have exact physical+PC+bank
fetch matches from retained artificial scenarios. Static contracts still require
independent graph/return audit below; matching decoding alone is not acceptance.

Indexed-write caveat: the old proof rejects literal stack-page stores, but a generic
absolute indexed write below0100 could cross into page1 or RAM mirrors. Actual
C6E3/C6E7 and CFC6 use STA0080,Y after LDY0F with DEY/BPL, so writes are0080..008F.
These local loop bounds close this actual instance; they do not certify arbitrary
indexed writes. Generic proof absence is not a veto on supported local paths.

Return/entry audit complete:
-304 ordinary graphs independently rewalked at instruction boundaries and stack
  depths;647 proposed JSR return edges have a corresponding audited summary and
  exact source JSR bytes. No BRK/KIL/RTI/TXS, unresolved indirect stores/control or
  mapper stores appear in ordinary summaries. Exact local indexed-stack exceptions
  above are independently byte-checked in graph-audit.json; this audit is stronger
  than reusing the earlier literal-page-only checker unchanged.
-10 contextual conditional graphs comprise C816/C818 plus C373/C3CB/C3D8/C3E5/C44E.
  C373 selectsbank1->85C0, C3E5 bank4->AF53, C44E bank4->AFBC. Direct/transitive
  write sets are disjoint from saved07E0: AF53's6C0,X usesX3F..0 (not all255) and
  caller queue guard; AFBC writes PPU only;85C0 writes zero-page state. Fixed
  C2F1 stores old bank read at8000 to07E0; C2FE reloads07E0 then tailC2F7.
- C3CB/C3D8 save A with PHA, bank select and PLA, invoke two clamped-copy routines
  then tailrestore. Domain selector0..16/0..5; each chunk is<=50hex. Queue cursor
  precondition<=80hex gives last write including terminator<=07D4 (4-byte command
  prefix,<=50 payload,FF); saved07E0 is disjoint. These are conditional structural
  entry contracts, not a universal claim that arbitrary debugger registers/cursors
  satisfy them. Existing state-machine callers use literal valid selectors or
  initialized37/76/7B selector states; D1B1 reads nonnegative7B for six-spec consumer,
  CE4E uses bounded76+5. Graph branches/returns remain valid under these contracts.
- C816 fixesX0; C818 requires evenX0..28. Opened callers use literal validX or
  A3AE,Y ->04/08/0A. C818 PHA saves old bank, finite parsed streams make the pointer
  loop well formed, C895 watchdog returns, PLA/TAY/JMP C2F7 restores bank. C2C8
  disables rendering/NMI through2000 mask before stream loop; C298 reenables near
  exit. Watchdog/helper stack is balanced; general ordinary caller stack contract
  is retained. No shared07E0 slot is used by this stream wrapper itself.
- Per-call supported continuations are accepted even though no universal sound
  page1 reservation proof exists. Calls through C31F/C340 and unknown wrappers
  remain barriers unless actual matching post-call fetches or another local
  independently supported entry exists. No entire wrapper family is certified.

All new code structural evidence combines an accepted/inspected dispatch entry or
actual fetched root with ordinary control flow and these explicit return summaries.
Legal decoding/pointers alone would not suffice. Static annotations do not assert
natural selection or deadness of remaining Unknown bytes.

Connected easy gains after batch acceptance:
- Bank5 physical0140DC..015769 independently tiles10 eight-byte descriptors,
  two pointer/script sets (12+10 scripts), destination arrays23/50 words, then
  six23x16-byte and four50x16-byte tile sets. Mask/next selector fields and all
  script indices checked;40 flush and80 next-state discriminate both terminators.
  Every tile/source gap is covered; final descriptor's50 tiles end exactly at9759.
- Bank1 physical00785F..007FCA:94 LE pointers end at firstrecordB90B. Header lower7
  bits count payload; bit7 selects raw queue copy, not an absent payload. A bounded
  first parser incorrectly treated highbit as no payload, producing gaps; actual
  B7F8/B80A branch skips command prefix, retains literal copy loop. Corrected parse
  yields contiguous union of all94 header+payload records exactly791B..7FCA.
  Last record'sFF overlaps prior genericFF tail by1byte;1898 newUnknown accepted.
  This is consumer-derived correction of a hypothesis before classification.
- Bank1 B778..B7E7 block reader112bytes is statically supported by its complete
  typed PPU queue-copy mechanics, paired ROM references and bounded input domain;
  caller reachability unresolved. It readsA879 paired8 block pointers, eight adjacent
  cellmapsA889..AA09, AA09 LE tile originAA0B, scales tile ID by16 and uses two
  balanced PHP/PLP row/column rollover loops.60 instruction starts.00 block0..7,
  01 within selected map,02 byte0..15,0F positive transfer count form the supported
  entry contract. Unknown prior code remains a reachability frontier, not a veto
  on this independently coherent rendering routine or its data.
- Physical006889..006A1B paired blockpointers/cellmaps/sourcepointer and006A1B..00744B
  tile-indexed repeated glyph/nametable payload accepted asData. Full cell values
  checked, maxA2 ends744B. Missing selected index16 and other internal patterns are
  generic non-code; no claim of a complete original tile table from selected IDs.
  The attribute-like remainder beyond744B has unresolved physical endpoint.

Last bounded residual pattern check: physical019F56..019FB4 is dense LE ROM-pointer
pattern adjoining34 repeated65-byte low-valued records019FB4..01A856. Pointer strides
41hex corroborate record starts (some aliases/out-of-order);13 later targets use10hex
stride in differently packed content beyondA846. Clear repeating smooth/plateau
patterns, all2210 record bytes<=3F, and transition at pointerA846 establish non-code
without reconstructing any exact waveform generator or claiming originalfulltable.
Consumer meaning/index extent remains unresolved: retain genericData for this2304
byte block, no Observed or formal complete waveform layout claim. The first threshold
scan suggested2216 bytes throughA84C; pointer/record inspection narrows by6 to exact
65-byte-pattern boundaryA846. Differently packed continuation beyondA846 is left
Unknown for role/boundary investigation rather than assuming the same schema.

Final bounded continuation review (after first verified checkpoint):
- FixedC68C is an independently proved bounded clear, despite the generic indirect
  store rejection: zero-page2C..EF then pointer00/01=0300, page loop through06FF,
  X=07 stop after pointerhigh increments to07. No page1, saved07E0 or mapper write;
  no local stack operation; ordinary RTS. Exact33-byte pattern checked in
  clear_contracts.py and decode-c68c.json. Global graph closure on this one new
  summary added0bytes; all existing-root post-call continuations already classified.
- A separate exact25-byte initializer lies physical00B79E..00B7B7, CPU B78E bank2
  effective[4,5,14,15], between an accepted PPU graphics stream and priorFFtail.
  It savesRAM1D by PHA acrossC68C, restores1D byPLA, sets1C=1, ensures2A|2Bnonzero,
  calls already proved CC8F and RTS. Fully coherent functional routine and exact
  data/code/data boundaries plus known helper effects support static entry contract;
  natural caller is unresolved. C68C disjointness justifies saved byte and return,
  CC8F ordinary summary leaves bank unchanged.13 starts/25bytes accepted Static.
  No inferred universal sound or mapper-wrapper contract. Initial scratch rootA98E
  was a decimal-offset reading error and correctly blocked as existingData before
  any annotation; correctedphysical00B79E is authoritative. Two standalone CLC/RTS
  tails in banks2/5 remainUnknown: isolated return pattern lacks enough role proof.
