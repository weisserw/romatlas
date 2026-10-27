# Round026 redirect: accepted main-state directory exposes an omitted contract

Input: round-026-disassembly; current-source commentary complete. No ROM, toolkit,
map, state, source or runtime changes. Disk14GiB free; no bulk scratch created.
Primary facts: [audit](primary-audit.json), [source excerpts](source-excerpts.txt).

## Changed inference and discriminating result

Prior FI025 A7 and Round026 isolated-return rationale treated all five bytes as
having no independent routine contract. The direct-control audit only checks decoded
instruction targets and ordinary fallthrough; it explicitly does not resolve computed
pointers. Reassess accepted directories plus their stack transformations rather than
repeating RET execution. Stop when a supported directory entry/stack contract is
found or the apparent references reduce to undeclared pointer resemblance.

Physical0728 is not merely an unreferenced C9. Accepted GameStateHandlers begins
physical044f; its ten words occupy044f..0462. Word7 at045d..045e is0728. This complete
ten-entry object was already accepted, distinct from sparse actor-type words.
DispatchGameFrame at0433 reads wGameState, tests03 and dispatches this directory.
For supported precondition wGameState=7, no synthetic early-state continuation is
pushed. MainFrameLoop CALL leaves its return on stack; RST00 pushes044f; POP HL at
0000 consumes that inline-table pointer; RST08 resolves HL[7], preserving the
CALLer's stack; JP HL selects0728. Its C9 returns directly to the frame loop.
This exact empty-handler contract explains the byte, entry and endpoint together;
0727 is the preceding routine's RET and0729 begins an independent completion handler.
The pointer is in an independently established consumed code-dispatch object, not
an arbitrary plausible word. Proposed Static Code0728 length1; no Observed claim,
no naturally produced state7 claim, no proof of complete selector domain required.
Strongest alternative is an accidental/stale table member referring to storedC9.
It is mechanically possible but does not explain the already accepted directory's
handler semantics as well as an empty handler. Existing directory acceptance plus
bank-independent stack contract discriminates this case from legal decode alone.
Disassembly should independently assess this inference, especially if accepted
map provenance proves weaker than the current source's declared object extent.

## Connected and independent follow-through

State6 is a nested death/continue dispatcher; it retries at4 or changes substate,
not a proven producer of7. Main-state7 is therefore structurally supported but
natural use unresolved. State8 at0729 has its own nested completion directory;
no fallthrough from preceding RET or directory crosses0728. No classification
proposal beyond the single byte follows from these continuations.

The four other residuals were compared separately against the same stronger method:
* 2f09: no little-endian2f09 anywhere in ROM; preceding retirement tail JP3199
  does not fall through. PersistentActorHandlers has five words pointing elsewhere;
  adjacent conversion handler begins2f0a. RET alone supplies no entry contract.
* 3420: one encoded pointer,33ae, exactly runtime-type0 cell. This differs from0728:
  33ae is now genericData, not a separately established directory member. The outer
  scan gates occupancy (field0), whereas dispatch selects type (field1), so occupancy
  alone neither proves nor excludes type0. Current accepted producer/consumer source
  establishes runtime1 and54..56, not occupied general-pool type0. The candidate
 33ae word plus3420 could describe a null handler, but both ownership and entry
  would be inferred solely from one plausible word and one opcode. Save this
  narrower candidate, without requiring natural execution as a universal rule.
* 34af: no encoded34af word. HideCurrentActorHorizontally tail-jumps backward to
  StoreCurrentActorScreenX; CopyActorLocalXDuringTransition starts34b0. Existing
  conditional CALL to HideCurrentActorHorizontally returns through34aa, not34af.
  There is no hidden extra pop/return contract covering the isolated byte.
* ad6a (bank2 CPU6d6a): no encoded6d6a word. EndingSceneHandlers is an independently
  established15-entry directory and points to6d6b for state9, not6d6a. EraseFinalEndingCreditBlock
  ends JP6c10. Renderer returns to original frame/ending caller, never6d6a. The fresh
  ending/credits progression supplies no new fetch. Later held-A phase uses other
  already classified handlers; it has no identified connection to these residuals.

Raw-word absence is only a negative constraint on ordinary encoded references; it
cannot exclude computed/RAM-built targets, natural use or all possible code roles.
No artificial call would distinguish intended routine from storedC9. No whole-ROM
selector closure, runtime sweep or unchanged gameplay replay was performed.

## Compact scoped rejection review (four remaining candidates)

Exact claims are one-byte Static RET routines at2f09,3420,34af,ad6a with above bank
contexts. Decisive primary artifacts are saved source excerpts and raw pointer audit,
plus accepted-code [conflict audit](../../round-026/conflict-audit.json) and fresh
[runtime audit](../../round-026/runtime-audit.json). Direct/fallthrough rejection
covers only decoded ordinary edges; no claim that lack of fetching disproves code.
Independent constraints combined are exact neighboring terminal flows, table
ownership, selected target identity and caller-stack behavior. For3420 the strongest
alternative is a type0 null handler, weaker only because its supporting directory
cell itself lacks independent role evidence; the occupancy/type distinction prevents
an overbroad 'type0 impossible' rejection. For the other three, a detached no-op stub
and stored byte remain indistinguishable. GenericData is also unsupported: a single
C9 between terminating code is not a non-instruction pattern. Keep Unknown.

This is a ready1byte proposal, not terminal exhaustion or an acceptance decision.
After independent review, revisit only if the review exposes a stronger structural
constraint; no useful revised experiment is currently identified for the other four.
Broad progression's boss/final-held-A gaps remain explicit; nothing here claims
exhaustive gameplay or natural later-game reachability. ExistingconfirmedT001 only;
zero new toolkit findings and no blocker.
