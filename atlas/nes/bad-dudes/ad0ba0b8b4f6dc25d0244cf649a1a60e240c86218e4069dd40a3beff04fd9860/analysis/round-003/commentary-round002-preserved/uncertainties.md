# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Bank-zero sound interpreter

The current Code byte at physical$576 (CPU$8566) is BRK, while the supported
pulse control helper at CPU$8515 reads$8566,Y as an alternate duty table.
This does not prove BRK reachability. Redirect should inspect its discovery
predecessor/entry contract and the bounded four-byte neighbor interval before
changing Code/Data classification. See source DecodedBrkAtDutyTable and
SelectAlternatePulseDuty. No structural edit was made in commentary.

Unknown spans interrupt primary request comparison, note-envelope initialization,
period output and duty-counter update. Source comments explain only classified
mechanics; bank-zero pointer/command tables remain unknown, despite consumers.
Sound period offset and sweep logic are preserved literally where the conventional
signed-add or channel-two exclusion interpretation would contradict instructions.

## Fixed frame services and runtime helpers

Controller reads and OAM/CHR/PPU handshakes are mechanically explained. Numeric
path/action/terrain selectors, special HUD tiles, persistent counter gameplay role
and some stage raster modes retain limited names. Raw dispatch tables and gaps
interrupt several path, response and rendering routines; comments describe their
classified fragments without asserting the missing bodies or table bounds.

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

Bank9 response boundary helper CPU$88A3 calls fixed CPU$C09F with A=2 before comparing actor Y. That fixed address remains inside the current Unknown span; its effect/ABI are unresolved. Local clamp/type30 conditions are explained without assigning movement semantics to this call.

Bank9 repeat/range response callbacks and response-path initializer call fixed CPU$C0E4, still inside an Unknown fixed-bank span. The neighboring $C0EA first-shape disable contract is established; it does not establish $C0E4. Source leaves that call unresolved and explains surrounding state/path/lifecycle effects.

## ROUND002 bank10 response shape overlap

The complete first-family loader at CPU$84E7 (physical83191) indexes$863B and$8649 by the response low nibble without a clamp. The class14 path visibly selects that nibble, so it reads$8649 and$8657: the next classified first-shape field and paired-state-selector bytes. Current14-byte field boundaries and source bytes are retained. This may be an intentional cross-use of adjacent data; a broader table layout is not established. A bounded follow-up can compare the literal class14 entry/shape values and downstream contact-shape consumers under bank10 at$8000. The source records actual reads; no map expansion is proposed. Fixed$C1EF called by the loader remains in an Unknown fixed-bank span, and$C0E4 in health setup retains its unresolved contract.

## ROUND002 coordinated response table check

CPU9B5B maps physical88939, exactly the classified23-word return table. The apparent overlap in a draft investigation came from a16-byte arithmetic error and is rejected in [the corrected check](../analysis/round-002/commentary-bank10-response-return-question.md). The selector and masked dispatcher share that base; no structural issue follows.

## ROUND002 bank11 response and pattern helpers

Shared response, slot-path, coordinate and interval helper bodies are now named and their effects carried into callers. Fixed C09F/C0E4/C1EF remain inside Unknown fixed-bank spans. Their neighboring named entry contracts do not establish their effects. Bank11 forward probes return1 when collision low3 bits are zero; callers deliberately use both polarities, now described literally rather than assigned an obstacle meaning.

The state72/74 middle-phase path computes a phase value without storing it; six-step and four-unit secondary-parameter subtraction can retain wrapped underflow; companion creation does not check the allocator result; the byte divider requires a nonzero divisor. Source explains these mechanics without repairing or asserting natural reachability of exceptional inputs. Global02A3 bit5, the counter gate's gameplay role and specific actor identities remain semantic uncertainties, not skipped readability work. No new structural reclassification is proposed. See the current-source scope assessment and preserved dependency evidence.
