# Round002 residual contracts and ending presentation

Input: maps/round-002.json, current-source commentary handoff and verification.
Classification remains130691/131088,397 unknown in28 ranges. No annotations or
source edits are made here. checkpoint-hashes.json identifies immutable inputs.
proposals.json proposes254 Static Code plus143 Static Data, exact disjoint slices
covering397 current unknown bytes; this is not accepted classification. Fresh
configured-binary decode.json/txt and exact residual-ledger.json accompany it.
Bank6 lower-window proposals use[12,13,14,15]. Fixed-window proposals are fixed
physical banks14/15; no dependence on lower windows is silently resolved.

## Structural findings

9A80 explicitly points at9AB6. Nine four-byte records parse as destination word,
nonzero count and first tile; final zero word terminates exactly at9ADC. This
consumer, complete record consistency and independently accepted9ADC routine
prove38 Data bytes; decoder output is visibly not the role proof.

A830 supplies an explicit28-byte copy into OAM throughA858. A84C supplies six
coordinate pairs through B0C3/B0C8 (X increments twice and CMP0C bounds the loop).
The combined40-byte gap splits at A84C; no guessed sprite schema is required.
A70C eight signed valuesFD..04 has actual indexed readers with masked0..7 in
AC69. A714 and A71A are separate three-byte alias slots, never extended into the
signed table. B4EC initializer ends RTS B500; four bytes B501..B504 are its
indexed source, not code fallthrough. Its calling precondition is A0..3; natural
caller remains unresolved.

C014/C01A/C01D/C028/C037/C03A/C03D, CE0C/CE12 and F506/F509 are three-byte
aliases in independently bounded blocks with accepted neighboring aliases and
accepted target entry contracts. This is combined layout/target evidence, not a
pointer-value-only claim. E570 specializes accepted E55D clear/hide helper to
one sprite and is also targeted by proposed CE0C. FC99 is LDY10/BNEFC9F; its
immediate nonzero condition proves alternate setup flow into accepted shared code.

C19C is a self-contained palette upload loop, DA70 an OAM-Y hiding loop, E82C a
16-iteration shift/add multiplication routine, FF70 a PPU-status/controller-strobe
helper. Their explicit register/pointer preconditions, complete data/control flow,
finite backedges or hardware operations and RTS boundaries support static roles
without inventing natural callers. This is stronger than legal decoding alone.
E82C shifts1E/1D/1C sixteen times and conditionally adds1B to1E on outgoing1C
carry, propagating into1D. DA70 advances Y exactly four per sprite and decrementsX;
caller supplies F4/F5,Y,X. FF70 strobe4016 receives1 then0 and routine returnsX0.

DC40 initializes current object Y10, sprite7A and motion through accepted DACF,
then assigns field5=98. Both yielding loops call accepted render/scroll/contact
helpers; first waits forY70, then uses direction motion. Contact destroys via
DB8D. C023 saves the object continuation rather than ordinary immediate return.
EE00 selects02A0, scans seven32-byte slots, inspects active records' field10=14,
and balances its saved Y on both branches. Matching records yield/rescan; none
activatesEE30. This is a complete finite scan with scheduler-aware continuation,
not a claim a natural spawn selector chooses it.

Other data slices have direct consumers or finite repeated non-instruction content
and exact adjacent routine boundaries. Generic Data is proposed where schemas or
natural selector extents remain unproved. In particular, C742 is an explicit text
source origin with count20 and the unknown four-byte prefix00/04/08/0C; its
reader also overlaps later accepted code, so no exclusive32-byte data object is
claimed. E6F2 is an explicit masked-sprite source, count04 with hidden-Y FF and
zero fields. Its fourth hidden record can skip fields overlapping later code:
only the fourteen existing unknown bytes are proposed Data, not a17-byte object.
D94A..D94C excludes accepted D94D code despite the baseD949 indexed reader.
Tiny B58B/CF8B and FA45 remain generic bounded parameter-pattern proposals;
no invented consumer or semantic names are supplied. Independent review should
adjudicate these narrow claims independently rather than treating the batch as
one indivisible proposal.

## Ending shortcut and failed alternatives

Prior raw-scroll failure was retrieved, not replayed. Source tracing establishes:
WrapVerticalScroll produces player bit40, score/action counters feed83/84,
B03A requires83>=3 or84>=0F, B047 rejects eligible active main objects, B02A
chooses presentation only for area3. Active player sign bit must remain set.
The saved stage15 snapshot is in globalA1 death/restart state with a pending
continuation005D and empty object pool. First player-only40 patch lost active
sign and restart overwrote it. Correcting player toE0 alone still lost to global
restart. AFC6/A700 intercept attempts did not match within300000 operations;
these bounds do not prove those routines unreachable.

Changed justified method: restart codeC8FF/C906 clears player and delays before
restoring checkpoint. Normal active global state81 is independently established
by source (AND9F/ORA01 at restart exit). Restore original stage15, release input,
set0040=81,005D=0,0060=E0,0062=80,0064=10,0083=03,0088=0,008D=0,008F=0,
054B=0,049B=0. Main object pool was already all zero, so no object edits needed.
Indices remain mission0F/area03. No PC/register or ROM writes occur.

ending-global-contract.py/json records all calls and budgets. Actual run_until
matchedB08B after17897 operations at frame641, banks12/13/14/15. Then30-frame
calls exercise timed sprite animation and final messages. Selected frame30
(after930 more frames) visibly reads CONGRATULATIONS! YOUR ALL MISSION IS ALL
OVER! in ending-global-contract-30.ppm.png. This is an artificial ending-screen
visit, not natural boss defeat, secret ending, staff credits or compatibility.
All failed methods and captures remain preserved. Final state is retained locally.

## Independent avenue assessment

Reviewed all commentary uncertainty bullets, all28 unknown gaps, explicit generic
and uncommented boundaries, and unresolved-helper prose throughout current source.
Main/secondary/spawn/audio dispatch/table boundaries were proven in the prior
connected attempt; no new evidence widens their natural selector domains. Seven
projectile allocations versus six ordinary updater slots remains a semantic
question without structural unknown bytes or a new discriminating practical test.
Decimal score life-threshold behavior is fully classified; intended reward design
is not a structural prerequisite. Standalone fill/debug natural callers are not
required for their already accepted structure. Broad source unresolved-helper
claims at lines9343,9390,9416,9581,9721,9864,10806,10813,10948,12071,12607,
12970,13055 mostly refer to now named/accepted targets and complete intervening
code. They provide an editorial correction frontier, not new missing byte graphs.

No identified concrete structural avenue is deferred outside this combined batch.
Secret ending condition StatusCount=3 and ADC GrenadeCountPacked=28 is a
nonessential alternative presentation; no exhaustive permutations or artificial
special-status sweep proposed. Representative later stages, combat/fortress and
now final presentation cover broad milestones without asserting boss defeat.
No terminal exhaustion review is invoked: ready proposals require independent
disassembly followed by full current-source commentary and redirect.

Final observation: at2100frames afterB08B, ending-global-contract-69.ppm.png shows
the completion sprites removed and RAM mission/area=0/0, statusFF. The normal
presentation reset branch executed in this artificial scenario; the following
global transition is retained in ending-global-contract-after.snapshot. No staff
credits or secret ending observed. input-preservation.json confirms all ROM,
map/state and canonical source/symbol hashes unchanged.
