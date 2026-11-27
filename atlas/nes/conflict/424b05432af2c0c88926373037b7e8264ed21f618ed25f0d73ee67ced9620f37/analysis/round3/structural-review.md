# Round3 independent structural review

Input: pass-002, current map SHA256
38b30eb21a0c8e9c601f7d1b4d065e37185fec9a4855e2dc331582b1e632f959.
This review accepts structural roles, not natural selection, universal termination,
original toolchain layouts, or hardware compatibility. All new annotations are Static.
The existing commentary source remains intact for the next full commentary phase.

## Independent checks and acceptance

`review.py` starts the configured release server, restores the internal map and freshly
decodes all 6702 proposed instruction views in 223 bounded contiguous groups. It
independently translates every CPU address through its four physical 8KiB banks,
checks every length/text against the proposal, checks operand ownership and accepted
regions, and recomputes unique Code/Data intersections. No conflicting instruction
starts, Code/Data overlap or accepted-Code crossing occurs. One challenged accepted
Data byte is separately localized at physical $5836. Fresh results are retained in
`independent-review.json`. Legal decoding is a consistency check; the role proofs below
are the grounds for acceptance.

* R2-L02: The existing A3E8 dispatcher and A4B5 inline-return consumer discriminate
  handler words from fallthrough instructions. The latter pulls the JSR return,
  builds the absolute target from its inline little-endian words and jumps; the
  table is not a returning-call continuation. The 75 command words end at the
  independently accepted A481 body. I reviewed all 24 nested phase tables against
  their targets: initialization/increment of actor7108, decrements of7109, actor
  activation/clear in7100, repeated aliases, side slot1A/1B restores and service
  tail calls explain their state-machine role. Tables end exactly at independently
  coherent target bodies; later phase entries repeat finite transition mechanics.
  No table bytes enter the candidate instruction graph. Isolated RTS targets are
  supported table-selected terminal phases, not standalone legal-byte guesses.
  The 9826 RTS repair has both BNE981B and JSR9823/A48A return contracts; A48A's
  balanced actor-state helper distinguishes this from a printable numeric suffix.
  Split only $5836 and preserve $5837..$5847 as the prior Data suffix.
* R2-L03: Independently reparsed all 176 sprite-directory words and 171 unique
  records directly from the immutable ROM. Every 1..32 piece count gives exactly
  1+4*n bytes; every end equals the next encoded root, with the final eight-piece
  record ending BA1E at the previously accepted FF suffix. A54A/A571/A5D0 consumers
  use the count and four-byte piece stride. This separates a bounded sprite record
  container from arbitrary pointer-looking bytes; all-frame natural use is unclaimed.
* R2-L04: The numeric neighborhoods have indexed reads in their coherent actor
  consumers, recurring 47-byte equipment rows, bounded coordinate/timer arrays and
  exact neighboring handler boundaries. Specific rows are retained where bounded;
  original formula/curve partitions remain generic Data. No code decision is based
  on nonexecution. The graph and data intervals are disjoint after the RTS repair.
* R2-L05: Read configured MMC1 high-bit-reset implementation (read-only) and checked
  seven resetwords plus all seven SEI/INCFFFD byte sequences independently. Under
  the explicit mode2 entry [0,1,2b,2b+1], the old C0 high vector byte written by INC
  forces mode3; the next C004 fetch is physical $1C014. First four bytes are a
  coherent reset shim, while each source +4 EA remains outside this role proof.
  Prefix/footer Data is separately supported by zero/FF alternating patterns and
  actual bank-end encoded words; no false C01B mirror or padding generator asserted.
* R2-L06: Accepted source call/branch continuations, finite inline dispatch and
  explicit banked wrappers support the independent graph. Checked C549 literal4 /
  E06F / AC27 / E072 / C552: middle calls require [8,9,14,15], final RTS restores
  [12,13,14,15]. DA94 selects4 until DABC/DB4B; D52A selects0 until D535. Bank4
  AE63/AE8D and B0E8/B12F/B146 save/select/restore4, not field6. F7EA/F7F6 wrappers
  similarly carry their changed middle views. CA96 is the initialized ordinary
  battle-return continuation and restores field at CA9B; ending reset sinks do not
  establish a returning ending. AF13->FFD7 is nonreturn and does not prove $1FFEF.
  BD9F has three static score-result tail anchors (953D/95C1/9677), 7020+side
  saturation/decimal-output flow, nested effect-state preservation and BE47 end.
  Small standalone helpers have fixed OAM/palette/PPU/serializer interfaces and
  independently bounded next entries; 82A5 return requires F3&7=2. A5FD's matching
  coordinate restores saved X/Y; its unmatched exit is not claimed universally
  returning. D72C similarly admits the 0418!=0 return and matched-lookup path;
  two saved bytes on its failed lookup remain an explicit limitation. These
  localized partial-return contracts establish Code role without inventing callers.
  E006/E009/E00C use the fresh bank5 [10,11,14,15] overrides: the row-directory,
  B418 stream and BC2D patterns agree with their actual services. Exploratory
  field6 low-address references are excluded as Data role evidence.
* R2-L07: Independently parsed battle asset directory/descriptors/101 PPU roots;
  their union covers exactly 3525 bytes to A8E1. Reparsed nine setup packets and
  all seven credits PPU packets with the distinct grammars: BF29 bit40 is fill;
  81D9 bit40 is literal. Count0 means256; FF terminates. Independently checked all
  seven credits OAM terminators and 4-byte stride and all seven two-count tile
  texts. Their known consumers/count domains and next-object boundaries support
  these extents beyond mere pointer values. 91E1's 68-byte copy is split36-byte
  packet/32 targetpalette, not one packet. 9467's 56-byte copy overlaps949E Code;
  only55 useful helper-source bytes are Data. BC0C's40 words include4 RAM aliases;
  03-terminated ROM strings end at BD9F, whose score code stays Code. Queued-target
  16stage directory/declared records end at E2A0; stage0's extra46 bytes remain
  generic record-like Data rather than a falsely selected12-record list.
  Three 00/FF/isolated80 slabs were checked independently as non-code patterns,
  split before the distinct $1E00D tag and supported JMP directory.
* R2-L08: Independently recomputed all64 unit roots: 9322+8*n ends9522, consistent
  with mask3F and copy8 consumers. The16 shared pointers and58 distinct type
  triples end95F0 with NO FF sentinel; only matched-type return is supported.
  Positively checked identical18-byte CDA1/D71A selector rows, supported by reads
  at D71A/D720/D726; an apparent pointer toD71A cannot override this Data role.
  D1B6's66-byte word material stays generic Data without a universal nested
  selector partition. The remaining88 final neighborhoods were reviewed against
  immutable bytes, source/graph direct-read hypotheses, recurring palette/OAM/
  direction/button/constant patterns and both neighboring Code/Data boundaries.
  Generic non-code roles are accepted where specific schema/game meaning remains
  unresolved, after practical consumer and format checks, rather than raw export.

The independent machine audit corroborates 15344 unique Unknown Code bytes and
22966 unique Unknown Data bytes, plus one accepted Data byte changed to Code.
Existing Data/Text islands inside proposed whole containers are preserved.

## Residuals and connected scope

The supported connected batch includes all packaged command/phase handlers,
independent battle/setup/field controllers, banked wrappers and associated data.
No productive ready handler is reserved for a later phase. Current local residuals
are the seven source-template EAs, $982D..$9830, $1E00D..$1E010, $1FFEF and
$20000..$2000A: 24 bytes/11 intervals. I reviewed the actual claims and competing
interpretations in `../redirect/r2-tactical-frontiers/residual-review.md`, and the
prior reset-sink reasoning in `../round2/structural-review.md`. Reset remapping
skips each EA; the ending continuation targets its own operand after a reset sink;
the tag/footer lack a specific entry or discriminating data-format constraint;
$1FFEF lacks the independent returning edges that repaired $5836. These are local
role ambiguities, not missing-natural-caller requirements or terminal exhaustion.
No arbitrary-PC execution would discriminate the competing static interpretations.
The next full source review/redirect can examine new evidence for these exact claims.

## Gameplay and limits

Reviewed `analysis/scenarios.json`, `analysis/gameplay.md` and saved milestone routes.
Representative later/final fields, password validation, command selection, tactical
result/world commit, factory/pendingproduction/birth, defeat, default ending,
credits/final scene/END/reset are already visited. Their exact state edits, bank
contexts, budget stops and captures are retained. Natural campaign victory and all
credit-page rendering are not claimed. No unchanged gameplay replay or speculative
RAM sweep is useful for these structural residuals. No new runtime evidence/patches
were produced; existing Observed classifications remain untouched.

No new toolkit issues found. NES-DOC-001 remains in `../toolkit-issues.md`.
No toolkit edit/build, ROM change, source/commentary rewrite, cleanup/deletion,
archive, publication or whole-workspace commit was performed.

## Checkpoint outcome and later toolkit finding

Initial pass-003 file validation failed before generation: map.save/export had
produced18,225,559-byte pretty JSON, beyond Mapping::load's16,777,216-byte limit.
This supersedes the earlier "no new toolkit issues" statement for this phase.
NES-MAP-002 is recorded in analysis/toolkit-issues.md; no toolkit was modified.
The failed maps/pass-003.json is preserved intact. Whitespace-only compaction of
analysis/state.json and maps/current.json produced13,263,962 bytes; independently
parsed objects compare equal, including all evidence, views, IDs and labels.
Fresh pass-003b validated, generated and rebuilt byte-identically. Details in
checkpoint-failure.json and ../pass-003b-verification.json. Actual saved map SHA256
is d3932aa1bbf71f8200a2af01bac65ca0665f4f545b40d864d59dbb1805aba9e3.
At handoff2.6GiBfree, analysis/scratch12KiB, round3 durableevidence13MiB. No cleanup.
All review/application/checkpoint handles have completed and their servers closed.
