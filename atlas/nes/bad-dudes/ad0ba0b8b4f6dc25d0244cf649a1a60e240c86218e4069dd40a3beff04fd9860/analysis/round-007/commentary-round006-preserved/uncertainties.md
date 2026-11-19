# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Bank-zero sound interpreter

The current Code byte at physical$576 (CPU$8566) is BRK, while the supported
pulse control helper at CPU$8515 reads$8566,Y as an alternate duty table.
This does not prove BRK reachability. Redirect should inspect its discovery
predecessor/entry contract and the bounded four-byte neighbor interval before
changing Code/Data classification. See source DecodedBrkAtDutyTable and
SelectAlternatePulseDuty. No structural edit was made in commentary.

The former request-comparison, note-envelope, period-output and duty-countdown
gaps are now classified and explained. The software-stack pointers, compatibility
bit masks and51command words have explicit data roles; the independent byte at
physical1222 remains Unknown. The header-origin read at CPU$8AFE falls in a
classified uniform$FFrun, so a valid natural sound-ID domain and full conflict
table extent remain unresolved. Sound period offset and sweep logic are preserved
literally where signed-add or channel-two exclusion interpretations contradict
the instructions.

## Fixed frame services and runtime helpers

Controller reads and OAM/CHR/PPU handshakes are mechanically explained. Numeric
path/action/terrain selectors, special HUD tiles, persistent counter gameplay role
and some stage raster modes retain limited names. ROUND003 now explains the complete
fixed actor-distance response scan, path displacement scalers, template constructors,
background life-transition fade and contact-effect OAM tails. Remaining Unknown gaps
retain their exact boundaries; classification does not establish natural use.

The interval consumer at physical128231 uses only$03E5 as a frame duration; its
records are flags/count pairs. Earlier two-byte interval interpretations in banks8,
13 and fixed helpers were corrected after this direct consumer review.

Actor render scans at CPU$FDD6/$FE07/$FE3A compare negative active type bytes
literally against positive$1E/$1F without masking bit7. The equalities cannot hold
for the same loaded byte. Comments preserve and explain this apparent ineffective
special-type selection rather than silently repairing it. No structural change is
implied, and this is not a toolkit issue.

The forward actor render scan retries its active slot on a zero motion byte; the
backward scan advances instead. This asymmetry is explained literally in source.

Worker C bank9 conversion contract: B9ConvertResponseActor at physical $012710 (CPU $8700, physical bank9 at $8000) uses response low3 minus1 to index four state/type byte pairs at physical $0127CC / CPU $87BC. The local source explains the supported selector1..4 ABI; mask7 alone cannot establish it for arbitrary callers. Current type8/9/10 contact helper and response43 transition are the concrete consumers inspected. Natural selector coverage and exact actor identities remain unresolved; a later bounded writer-domain review could clarify them without reclassifying the table or inventing extra records.

ROUND003 resolves the fixed C09F mechanical contract: it steps actor Y by unsigned A according to direction bit 3, then tests world X on the subtract path or the Y limit on the add path. The coordinate change is retained; callers here ignore its test result and perform their own completion/margin checks. The asymmetric tests are literal, not a claim of a common geometric boundary.

ROUND003 resolves fixed C0E4 as clearing bit 7 of the second contact-shape field while preserving its index. Fixed C1EF sets that bit. Together with the existing first-shape entries, callers explicitly enable/disable both shapes; exact gameplay purposes remain separately unresolved.

## ROUND002 bank10 response shape overlap

The complete first-family loader at CPU$84E7 (physical83191) indexes$863B and$8649 by the response low nibble without a clamp. The class14 path visibly selects that nibble, so it reads$8649 and$8657: the next classified first-shape field and paired-state-selector bytes. Current14-byte field boundaries and source bytes are retained. This may be an intentional cross-use of adjacent data; a broader table layout is not established. A bounded follow-up can compare the literal class14 entry/shape values and downstream contact-shape consumers under bank10 at$8000. The source records actual reads; no map expansion is proposed. ROUND003 resolves C1EF/C0E4 as second contact-shape enable/disable; this does not enlarge the adjacent 14-byte table fields.

## ROUND002 coordinated response table check

CPU9B5B maps physical88939, exactly the classified23-word return table. The apparent overlap in a draft investigation came from a16-byte arithmetic error and is rejected in [the corrected check](../analysis/round-002/commentary-bank10-response-return-question.md). The selector and masked dispatcher share that base; no structural issue follows.

## ROUND002 bank11 response and pattern helpers

Shared response, slot-path, coordinate and interval helper bodies are now named and their effects carried into callers. ROUND003 body inspection resolves C09F directional Y-step/asymmetric tests and C0E4/C1EF second contact-shape disable/enable. Their callers now use those exact mechanical meanings. Bank11 forward probes return1 when collision low3 bits are zero; callers deliberately use both polarities, now described literally rather than assigned an obstacle meaning.

The state72/74 middle-phase path computes a phase value without storing it; six-step and four-unit secondary-parameter subtraction can retain wrapped underflow; companion creation does not check the allocator result; the byte divider requires a nonzero divisor. Source explains these mechanics without repairing or asserting natural reachability of exceptional inputs. Global02A3 bit5, the counter gate's gameplay role and specific actor identities remain semantic uncertainties, not skipped readability work. No new structural reclassification is proposed. See the current-source scope assessment and preserved dependency evidence.


## ROUND003 bank8 action application and fixed response mechanics

C018 multiplies unsigned A by the big-endian word at $021C/$021D; it does not read a record. Bank8 uses multiplier 4 and the product low byte as the four-byte action-record offset. Bank10 uses multiplier 3 on actor/player distance before dividing by 10. The fixed 16-bit divider treats $0216 as high and $0217 as low; aliases now match its actual shift order.

The response scan tests exact player Y, accumulates three X-distance bands and keeps a strict nearest actor. Its numeric action identities remain unresolved. Stage6 depth adjustments retain unsigned CMP #0/BCC tests that cannot take those branches. The action-motion initializer reads one table byte and copies it to two destinations despite INY; no second-byte interpretation is inferred. The special actor render zero-motion continuation jumps to ordinary backward-scan advancement at $FDFF. These are literal mechanics, with no source or ROM repair. See current source and analysis/round-003/commentary-batch-007-fixed-engine-b.json through commentary-batch-013-bank8-dependencies-b.json.

## ROUND003 bank6/7/13 and dependency completion

Options, briefing, player-choice confirmations, timed scroll layers and player
collision response are mechanically named in source. Input-pattern gameplay
meanings, glyph/text wording, enemy/scene identities and several indirect
lower-bank origins remain unresolved. Busy waits depend on interrupt transfer
and timer consumers; no arbitrary-state liveness or natural reachability is claimed.

The E6A9radius, E705path-control and E71Fboundary-terrain consumers establish
separate CPU views but not every selector bound or disjoint physical table extent.
ROUND004 classifies the exact eight-byte paired-store continuation at physical109978.
With normal MMC3 mode0 and unsigned(camera+255) high H satisfying H&$40=0 and
H&7<=5, the record byte5/byte4 writes select only CHR registers and preserve PRG.
The record is ROM: readback stays unchanged, retry may loop and no natural $FF
record, writable mirror or successful update is established. Defined records lack
$FF byte5. This restricted contract is not a general writable-record API.

D2E5is a stage sound-request table, rather than PPU controls. D959contains64HUD
template tiles and48actor-award offsets followed by amount payloads. These
consumer-based role corrections change commentary only. See ROUND003 batches019–029
and the current scope assessment for the preserved reviewed inventory.


## ROUND004 bank5 partial mechanics and residual boundaries

The bank5 motion/clamp/countdown helpers and 225-byte stride-record initializer
prefix are named by mechanics. A000-slot successors and copy/table origins remain
unresolved; the constant encoded-origin comparison is not a native mapping.
C607 is SBC $60,X followed by the44-byte $02A2->$02FA copy. C8BD stores incoming
A to the final CHR latch, clears requests and releases OAM. C5CA is SAX/KIL,
not a returning dispatcher. The native helper names do not infer an animation API.
ROUND006 recognizes the last15-byte predicate as a truncated repeated Code gate.
Opaque call-list prefixes remain Unknown; their ambiguity is explained locally.

All44 detached epilogues and18 fixed continuations are now classified and
explained. Ordinary caller frames, synthetic RTS dispatch words and the saved NMI
hardware frame remain distinct. Busy waits still require their interrupt consumers.
At ROUND004, eleven exact Unknown intervals totaled114 bytes; unresolved meanings elsewhere are
not unread source or omitted classification work.

## ROUND005 actor-health override prefix

Physical68569..68596 is now a completed local Code computation, using the
nine-record search's preserved CMP Z and matching X to choose nonzero health or1.
The13-byte local type selector and seven-stage pointer boundary establish its
end independently. This resolves the prior whole-fragment rejection only for
that bounded computation: natural entry, original packaging, ordinary return
and executable successor remain unresolved. The following bytes remain Data.
At ROUND005 structural ambiguity was87bytes in ten intervals.

## ROUND006 truncated third gate

Physical49152..49167 repeats the established two sign/low7 gates and ends at
the independently established bank5 identity byte. This supports Code structure,
not a complete countdown, safe execution, successor or natural entry. Negative
transfers to unresolvedA76C, zero low7 to unresolvedA005; positive falls into
identity Data. Current72Unknown bytes in nine intervals retain local explanations.
