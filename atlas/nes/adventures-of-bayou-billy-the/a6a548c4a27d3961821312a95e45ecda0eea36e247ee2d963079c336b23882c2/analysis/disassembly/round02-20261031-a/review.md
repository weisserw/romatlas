# Independent disassembly review — round 02

Input: pass-001-final; SHA256 687e9f29ec9f58f16b94a00c952536b7f9464a679019f8d37fe13100509a097b.
Review is against immutable input/game.nes and the redirect primary artifacts, not
acceptance based on aggregate counts, legal decoding, conflict checks or rebuilding.
`primary-consumer-review.json` records fresh configured-binary decoding and
`independent-format-check.json` records separate byte-based format checks.
`independent-audit.json` records every proposed data interval's patterns and exact
physical/context checks. All artifacts remain local.

## C01 and bank preservation

Fresh E892 decoding loads mode36, indexes E8B6, and passes effective low-pair
selector through DDE4. E8A6 branches for modes0/1 to8000, otherwise800F.
The bytes E8B6=[06,08,0A] independently correlate mode2 with lowpair10/11.
Consequently the old bank10 prefix static path arose from losing this correlation.
Fresh81EF loads8000,Y and8003,Y with variant63. Saved805F/80D7 consumers load
8006,Y and8009,X/800C,X; all five origins are three bytes apart, followed by
independently supported800F code. Accept15bytes Data; remove exactly the five
old Static instruction records covering13bytes. Preserve Observed800F and later.
Variant production/full natural third-variant use is not asserted.

Fresh DDE4/DE17 show software bank stack2B/07D5, serial MMC1 writes and balanced
push/pop;30busy=0 is required. The graph wrappers select lowpairs, invoke their
bank-local callee and restore throughDE17. The complete known external-wrapper
contracts in redirect findings/graphs were reused, with balanced stack and busy0
preconditions. Fixed helper code uses [null,null,14,15]; no placeholder lowpair is
promoted to inferred live context. Mapper writes are traversal stops. These are
structural supported preconditions, not assertions every natural caller supplies them.

## Coherent code and finite dispatch

Accept graph-connected code in audio, brawl, shooting, driving/rendering, sprite,
terrain, ending, text and fixed helpers. Roots combine existing accepted entries,
finite consumer-linked handler tables, complete field/stack helper contracts and
saved actual ending/credits execution. Do not turn the whole graph into Observed.
Leaf and fragment contracts were separately read against the instruction bodies:
16bit negation/addition propagates borrow/carry across the matching object fields;
90EC conversion wrappers store both halves;90D7 saves/restores sign flags;
90FC sign-extends into19 before accumulation;packed score/health paths use the
established BCD helper and clamps;timer/queue paths complete the adjacent known
state/packet contracts. Prefix1E0D7 forms a complete timer/substate initializer
parallel to the next helper;102B3 calls E69D then the complete target-clear loop.
These discriminate routines from arbitrary legal byte sequences without requiring
an observed caller. Caller-frame removal atABD0/EADC requires the documented
additional call frame. No independent-entry/reachability claim follows.

The shooting8349 primary body rereads actor04C1 after8371, returns onzero,
then indexes A26D usingtype04D1 and the state table using04C1. Thus ignored state0
words may overlap preceding instructions; retain that exclusion. InlineE116 freshly
confirmed: ASL selector, pop return address into02/03, index words, indirectJMP.
All14sites have data barriers and no proposed fallthrough instruction. Extend the
player table's independently bounded final wordC324 (8783) before nextRTSC326.
New F707/F133/7CA6 word blocks follow their respective7/2/6 state contracts.
Four branch corrections are substantive: immediate nonzero loads before BNE at
14514/10B7D, and exhaustive BCC/BCS at15F97/11AAA. Their flags remove false paths
into constants, not arbitrary discovery exceptions. Audit confirms no partial
instruction overlaps or Code/Data conflicts.

## Data consumers and bounds

Seventeen screen containers: freshE69F derives source/table bank, little-endian
pointer, PPU destination and count commands. E6F5..E733 distinguishesFFend,
7Faddress reset, literal low7count and repeatcount; bounded independent reparse
matches all saved ends and yields1024bytes each except selector0's2048.
The bankselector/pointer tables end at independent neighboring code. Accept Data
with screen-container labels, not raw graphics export/fill assumptions.

Sprite80B6..816F directly proves family word selection, recordheader&3F count,
80redirect plus four-byte OAM components and decremented count. Separate reparse
validated370bank2 and158bank12 records, including shared redirects. Five valid
bank2 families close against their own first payloads; mode2's three invalid
bank2 addresses are excluded, rather than decoded as records. Bank12's two tables
have complete128/32-word extents and matching payload closure. Family/table/record
Data is supported; natural selection of every sprite is unresolved.

F18D freshly bounds coarse rows to0..47, fetches column origin viaF2FC, reads
metatile ID, multiplies by16 and addsF312 origin. F328 independently supplies
attributes. Rechecked11scene pools: tileorigin+16*attribute_count=attributeorigin
and all column IDs fit. The previously identified scene7/8 cross-table origin is
not a48byte column. Unselected metatiles remain valid Data through independent
origin/boundary constraints. Between-pool zero/repeated constant gaps are generic
Data after consumer/neighbor inspection; exact padding purpose is unresolved.

Text:23scripts and38tile strings independently endFF exactly at the next target;
first payloads bound the word arrays.101PPUtile-string words/81unique strings
have the same bounded terminators/FE row convention.109PPUcommand words point
into a pool with distinct palette/address/tile encodings: fresh B559/B5A1 confirms
the consumer is not anFF-only parser. Retain generic command-pool Data and its
independent nextRLEcontainer boundary; reject a falsely exact full parser/schema.
Credits selector bytes use saved moving roll/terminal observations, not inferred
human ASCII strings. Embedded identification marker is separately recognized.

Driving projection: saved8BCD/depth consumer, freshly inspected8BF3/8C04/8C13,
encodedAFB3 origin and32*128 stride constrain the array. Independent byte check
confirms32monotonic quantized rows and a separate61byteFFsuffix. Index128 aliases
the next row/finalborder. Do not claim an exact generator/rounding algorithm.
Three driving event/road streams have consumer-derived3/2byte records and sentinel
ends; six brawl event streams have4byte records endingFF exactly against the next
source/code. DemosA62D freshly proves input/duration pairs andFFend; separately
checked3streams' counts/ends. Intervening repeated/padding bytes remain generic
Data without invented record fields. Referenced small numeric/control constants
are supported by saved direct/indexed consumers and neighboring completed code;
short terminalFF bytes are kept Data in their stream/control context.

Audio descriptors have151three-byte entries at the encoded originA99F, all channel
fields0..5, with the independently adjacentAB64code endpoint. Stream pointerFFFF
record0 remains exceptional, not evidence for code. Dense repeated channel/music
command patterns plus real parser references distinguish the large audio pool
and envelope tails from instructions; finer stream schemas remain generic Data.
Finite handler/offset/envelope word arrays have coherent consumers and endpoints.
FreshB33C reads C000..C003,Y into4010..4013; eleven four-byte descriptors fit the
known selector remap and next packed-pattern boundary. Do not infer complete DMC
sample spans: some apparent start/length combinations cross code under the saved
context. The8992/6768byte slabs have packed alternating-bit/AA/55/00/FF runs
throughout and code-boundary neighbors; accept generic encoded Data independently
of unknown sample identity or routing. Histograms alone are supplemental, not the
sole evidence. Practical consumer/format inspection is complete enough for generic
fallback without claiming every byte is a particular audio sample.

Fresh8432 hasLDX99 andcopiesDAE5,Y to0600,Y untilDEXzero: independent153byteROM
upload source. Accept135previouslyunknown source bytes as Data/code template;
retain18overlapping Observed ROM-helper bytes Code. RAMexecution alone was not
used as source proof, nor are unsupported uploaded views added.

Six footer copies exactly match the finalbank observed reset stub and threevectors;
one allFFpair12/13 is excluded. Accept8bytes copied reset-code template and6bytes
vector words per copy. Keep pre-INC local contexts, copiedJMP local decoding view,
and actual post-INC fetch in finalbank conceptually separate. Code template role
is supported by exact complete replicated observed routine/vector structure;
local runtime fallthrough across MMC1 reset is explicitly not claimed.

## Five residuals and next phase

Independently reread primary neighboring graph bodies and immutable bytes for
C1E2..C1E5,C7A6,14379,1ACD0,1EE00. C1E2 isJMP E84B between completed routines;
others are isolated60bytes after a return/tailjump or before separately complete
helpers/constants. None completes an open stack/control contract; no indexed
finite table/consumer/template covers them. New fullgraph acceptance does not
supply an entry or larger template constraint here. A scalar/constant/padding
interpretation competes with isolated RTS/JMP code. Both Code and Data remain
insufficiently supported. Retain all7bytes Unknown; valid destination/decoding
and artificial entry would not discriminate. This agrees with the localized
prior review for its actual scope, not because historical rejection is a veto.

Broad gameplay evidence already includes natural startup/stage1 and representative
artificial shooting/driving/city/mansion/bosscombat/credits/THEEND/Start exit.
Reviewed deferred similar-stage/practice milestones: no concrete untested selection
or residual-entry hypothesis makes unchanged gameplay productive in this pass.
Reuse saved evidence; no repeat execution. Normal gameplay/static/ready easy gains
are exhausted for this single pass. Hand off incomplete to full current-source
commentary, then redirect to reconsider any semantic/entry constraints discovered
there. This is not terminal coordinator acceptance or full gameplay compatibility.
