# Pass004 independent structural review

Accepted L015/L016 combined batch: 7431 Static Code and 13139 Static Data bytes,
186 new physical regions (75 code runs,111 data slices). Fresh decoding compared
3292 explicit CPU/bank instruction views against the configured release binary,
including instruction length/text, physical slot translation, immutable ROM bytes,
old-map complement, overlaps and alias retention. Exact results: fresh-code-review.json,
data-review.json, accepted.json and rpc.jsonl. Validation succeeds. Static structure
is not observed execution or a natural selection claim.

Primary body evidence reviewed: redirect/round03-01/routine-body-facts.json,
final-code-flow.json, manual-code-claims.json, findings.md, data-proposals-final.json,
audit.py/audit.json and residual-dispositions.json. The redirect's graph walk alone
is insufficient role proof: its explicit service exceptions and body preconditions
must hold. Grouped body/field/loop/stack contracts supply that discrimination here.

Actor banks8/9 use X valid slot, ordinary return stack and actual object pages0300,
0400,0500. The8511 timer decrements040F and on expiration writes state0301/facing0509,
with both branches returning. 8CE4 allocates through9410, reads retained0506 through
8D16 to set facing0409, calls animation92E5 and resets motion through an established
helper; state decrement and timer reset are coherent with the neighboring update
family. A64A branches on040D, selects four motion bytes A6D7..DA, increments saved
index by4 and bounds it at60; the mirrored branch negates horizontal low/high words
with carry. A7BB/A9A8/B387 mask retained class07 to split pointers, advance stream
indices and process terminated command records into velocity/count/animation fields.
AECD scans10/20/30/40 under C6 gate, compares signed coordinates and uses direction
mask07 plus FF rejection before writing0301. These are semantic control/data
contracts, not stored dispatch words or merely legal instructions. No actor class
selection is claimed; dynamic RAM fields do not change PRG mapping. The slot scan at
physical12CF9 is preserved as Code; only12CE9..12CF9 is repeated Data.

Terrain banks2/3: A500 mask1E with paired reads A510/A511 supplies16 stored words,
ending exactly at A530's distinct predicate body. Handler nibble tests, key50..53,
coordinate converter C031/D916, checkpoint return7A/7B, inventory0770 and animation
9903 jointly establish routine roles and bounds. C061 is JMPFA5B. Immutable bytes at
1FA6B are A20A20A4FE200BB7A2024CA4FE: select10, callFEA4, callB70B, select02,
tailFEA4. B70B's six-byte record loop compares four fields and advances6. Thus exact
return banks2/3 are supported for this service only; there is no general mapper
permission. Graph stops at other mapper effects stay conservative.

Fixed helpers D8F0/D905 use facing carry to write0400/0500 and RTS; D916's row wrap,
shifts and nametable fields match independent actor coordinate conversion. F7F9/F82A
read2002, write address2006, use X4/Y256 loops writing2007 then64 attributes and RTS.
F787's finite subtraction100/10 formats decimal fields; F779's page02 DMA and F680's
32 palette fill have exact hardware/RAM contracts. F5E1 is one byte through a RAM
pointer, increments B2/B3, returns below2000 and traps at20; it is not a whole RAM
fill loop. Its RAM pointer is an explicit supported precondition. Fixed service
trampolines have three-byte JMP stride and meaningful implementations. C000's
NOP/self-JMP is self-contained static loop, without a reset/reachability assertion.
Audio86AE consumes the count saved by86AA PHA after86AB JSR86B5, decrements/repeats
and RTS; this supplies the seven-byte continuation contract independently of a new
playback selector. Existing audio inline words remain Data barriers.

All111 Data slices were checked against exact ROM bytes/hash and current unknown
boundaries; primary per-slice consumers, neighboring routines and repeated formats
are retained in the annotation. Boolean runs, signed fixed-point quadruples, bank5
four-byte low-domain records, palette groups, PPU address/index cycles, and24-byte
scene strips supply noncode evidence. Generic containers do not claim complete inner
schemas or every selected record. Tiny cursor domains have independently finite
writers. Projectile state20->21 selects the storedD8 high byte at1F083; following RTS
stays unknown. Physical1FE78/CPUFE68 is04, notEA: continuation of the eleven-byte
060A040406060604040404 metadata immediately before FE69 A000. No stage10 domain is
asserted. Prior prose is historical and the next commentary must correct it.

Residual review: 48 bytes in21 fragments. Exact primary bytes and neighbors were
cross-checked against residual-decode.json/residual-references.json, the new accepted
complement and scoped dispositions. Isolated RTS values lack independent role
constraints even where dispatch points there. F0ED at186D is after RTS and bypassed
by the earlier branch. A42F at7C03 is a two-byte alternate palette entry versus word;
0009 at7C7B lacks a corroborated trailer schema. A907 at18518 is overwritten by the
successor load and supplies no meaningful input contract. 1E59A's12 coordinate-copy
bytes match fields/table but run directly into established tile Data at1E5A6, without
exit after the preceding initializer's RTS. Tiny load/tailcall stubs have coherent
possible mechanics but no discriminating constraint beyond decoding. These are
localized code-versus-data ambiguities, not blanket missing-caller requirements,
dead-code claims or a terminal exhaustion verdict. Redirect may investigate new
specific discrimination after commentary. Replaying completed broad gameplay adds
no changed condition; late areas, bosses and ending evidence are reused.

Nine conservative graph stops are existing service/mapper code. No unknown following
byte opens an obvious coherent new batch beyond the accumulated actor, terrain,
scene, helper and repeated-data proposals. Exact post-annotation complement remains
the21 tiny fragments. No further productive easy gain was identified in this pass;
no proposal was rejected solely for lacking a natural caller. Source readability
requires the next full current-source commentary. Final editorial audit reserved.
