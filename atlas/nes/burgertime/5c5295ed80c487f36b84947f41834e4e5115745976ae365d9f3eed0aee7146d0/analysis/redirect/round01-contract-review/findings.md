# Round 1 redirect: contract and residual review

Input: commentary-round01-final / pass-001-final, map SHA256
75d8105e19a68e00cad655743bcbc11b76ec970b0aad9b97fafe83352f60c04f.
No classification, source or ROM changes in redirect. Static NROM CPU C000–FFFF
view has effective banks [0,1,0,1]; physical = CPU - BFF0.
Primary evidence: [RPC inspection](rpc-inspection.json),
[physical/reference evidence](physical-and-reference-evidence.json),
[reproducible inspection](inspect.py), [source excerpts](source-excerpts.txt).

## R01-003: supported eleven-byte static routine proposal

Claim only: [3609,3614), CPU F5F9..F603, is an alternate music-priority guard
for an incoming effect, with a subroutine return contract. Do not claim an actual
caller, observed execution, or original entry name.

Five instructions: LDA $05A2; BEQ $F611; JSR $F6BA; BCS $F611; RTS.
Independent constraints, beyond legal decoding:

1. $05A2 is the active MusicRequest, independently written by LoadMusicStreams
   from SoundRequest and cleared by StopMusic/control commands. The test bypasses
   priority precisely when there is no current music.
2. F6BA TestAudioReplacementMask consumes A=current request identity, preserves X
   and incoming header pointer $91/$92, reads the selected priority bit in the
   incoming header starting at offset three, and returns permission in carry.
   Thus this slice's JSR/BCS tests the music identity in exactly the same priority
   representation as the immediately preceding effect-identity guard F5F0..F5F8.
3. Both independent conditionals land exactly on F611 LoadEffectState, past the
   slot-switching entry F604. X remains the selected effect slot. The loader
   consumes the preserved incoming header pointer and SoundRequest; both paths
   and the refusal RTS have a balanced original caller return address. No mapper
   writes or inline data intervene. The preceding guard ends in RTS and following
   entry starts independently at F604, establishing [F5F9,F604) boundaries.

Supported precondition: ordinary subroutine stack; X=selected effect slot (38/66),
$91/$92 points to incoming effect header with priority-mask bytes appropriate for
nonzero MusicRequest (ordinary IDs 01..11 require offsets 3..5), SoundRequest is
incoming identity. This is a structural contract, not a claim anyone supplies it.
MusicRequest zero or permitted bit set tail-enters existing effect loader; otherwise
returns without loading. Header controls and negative request selectors are *not*
required to manufacture an incoming edge. Known ConsumeAudioRequest branches
still bypass this slice. Reachability remains unresolved.

Competing data interpretation must explain a MusicRequest-specific priority test,
the adjacent guard's identical helper/carry/return protocol, and two exact convergent
branches into the established loader without table framing. No known consumer reads
these bytes as records; complete pattern/control/stack consistency discriminates
static code more strongly than the lone opcodes in R01-001/002. Independent
review should verify this combination, not require a live caller.

Connected investigation followed loader F611, alternate slots F604, controls F63C,
music setup/stop, mask helper F6BA, bit table F6DC and downstream effect interpreter
F6E4. All are already classified and explained. There is no additional unknown
continuation, inline payload or bank transition. Caller tracing is negative evidence
for reachability only (saved reference scan finds none), not a structural veto.

## R01-002: three-byte residual remains materially ambiguous

[3485,3488), CPU F475..F477 = JMP F493. Final ladder table pointer F44D
selects thirteen triples ending at physical 3484=FF; the three bytes are outside
that record stream. SearchLadderGeometry stops on FF before consuming them. Next
entry F478 resets audio, F493 services audio, so a public service trampoline here
is plausible. Full decoded reference scan and all aligned/unaligned PRG word scans
find neither F475 nor B475 entry reference.

Boundary proof rules out membership in the *selected final ladder record*. It
does not establish whether the following three bytes are a wrapper or unrelated
bytes. A JMP tail contract imposes no register/stack/data semantics beyond the
known target: unlike R01-003 there is no multi-operation semantic consistency to
corroborate code. Strongest competing model is three non-code bytes between data
and engine; neither its non-code role nor a meaningful pattern is independently
supported. No repeated/uniform run exists to justify generic data. Retain unknown;
no caller prerequisite is imposed, but an additional independent structural
constraint would be needed. Arbitrary artificial execution would only show JMP
mechanics and cannot discriminate these alternatives. No changed useful test found.

## R01-001: one-byte residual remains materially ambiguous

[005D,005E), CPU C04D = RTS after scrolling routine's own RTS C04C, before
InitializeDemo C04E. No branch/call reference or exact C04D/804D word occurs in
inspected PRG; this scan cannot prove absence of computed entries. Neighboring
scroll routine has complete return paths without this byte. Alternate no-op return
and a non-code byte are both consistent. Duplicate RTS alone is a legal-opcode
pattern, not a meaningful routine contract or visibly non-code run. Boundary is
exact but role is ambiguous. Generic data would merely relabel uncertainty. No
promising discriminating experiment remains on current evidence; future independent
entry/consumer evidence could reopen it. Do not call it dead/unused.

## Other concrete avenues inspected

Four stale map annotations have actionable corrections (R01-004). Evidence in
source excerpts corroborates commentary; no physical bounds change. Control table
has five interleaved signed X/Y pairs at D268 plus twelve direction remaps D272;
initial CB6B byte 02 seeds extra-life BCD thresholds, remaining three meanings
unresolved; E6A6..E6AD are pepper stun countdown values using stage-capped one-based
E6A5,Y addressing; bonus records advance eight bytes, separate player-mode records
seven. Preserve original historical evidence when updating live metadata.

Ingredient selector-one/two wrappers fall through after nonzero LDA, ultimately
selecting three. Exact branch behavior is already classified/readable; intention
cannot add structure and no new unknown continuation exists. Negative sound IDs
pass the signed gate, and ASL(request-1) carry survives BIT; that carry changes
music-stop/control routing, not entry into F5F9. Without an independently supported
negative-request domain/header pointer, artificial arbitrary IDs only dereference
unproved headers. This avenue has diminished structural prospect, not a universal
selector requirement. Internal layout/audio stream extents, fill purpose and CHR
meanings are already classified with bounded generic roles, and uncertain exact
semantics do not create unknown bytes. All current source unknown annotations,
explicit commentary questions, generic data summaries and exposed indirect-dispatch
barriers were triaged; there is no other concrete unclassified continuation.

Gameplay checklist reviewed: natural first, artificial third/sixth layouts and
GAME OVER already visited with input mechanics. Six-layout wrapping completion
path excludes a simple later-stage ending selector; no boss/credits entry identified.
Shared layout2/4/5 initialization has no independent unknown-structure opportunity;
existing stage recipe remains usable, no unchanged replay needed. No claim of
exhaustive gameplay or natural late-stage reachability.

## Outcome and frontier

Return one coherent batch to disassembly: R01-003 (+11 proposed code bytes) and
R01-004 (four metadata corrections, +0 bytes). Accepted bytes remain unchanged:
24,577/24,592 (99.94%), 15 unknown. If accepted, 24,588/24,592, four unknown bytes.
R01-001/002 are unresolved with actual localized competing interpretations above;
this is not a terminal incomplete recommendation, because a useful ready proposal
exists. Next redirect after classification/commentary can reuse this review on
unchanged premises, while coordinator independently inspects evidence standards.
No unfinished productive continuation. No toolkit issues found. Disk ~3.3GiB free,
analysis/scratch 80KiB; no cleanup, toolkit mutation, archive or publication.
