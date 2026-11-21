# Round 2 independent disassembly review

Accepted R01-003, physical [0x3609,0x3614), CPU F5F9..F603, fixed
NROM effective banks [0,1,0,1]. Current decode persisted in rpc-results.json.
Five instructions LDA $05A2 / BEQ $F611 / JSR $F6BA / BCS $F611 / RTS.
Independently inspected MusicRequest load/stop writers, mask helper, adjacent
priority guard and shared loader in current commentary source. The helper uses
(request-1)/8 plus three for header mask offset, uses low three bits for the mask,
preserves X and $91/$92, returns carry permission. Both guard branches land at
F611 and bypass slot switching. Refusal RTS and loader RTS consume the original
caller frame. Previous guard ends in RTS; F604 is a separate supported entry.
These combined semantic/control/stack constraints discriminate code from arbitrary
data; no table consumer or framing competes. Annotated eleven bytes Static only.
Precondition is ordinary caller frame, X selected effect slot 38/66, incoming
header in $91/$92 and request in SoundRequest. This supported contract establishes
structure; actual entry/natural reachability remain unresolved, not claimed dead.
Connected loader, music writers, helper, slot switching, controls and interpreters
are already classified. No additional unknown continuation or bank transition.

Accepted R01-004 metadata corrections without changing physical boundaries:
region76 first parameter initializes extra-life step/BCD thresholds; remaining
three meanings unresolved. region81 bonus descriptors stride eight from setup
and pointer increment; separate player-mode family stride seven. region84 five
interleaved pepper X/Y pairs then twelve direction remaps; unsupported opposing
input indices remain unresolved. region97 stage-capped one-based pepper stun
countdown table via table-minus-one operand. Historical checkpoint evidence remains.
Current source consumer excerpts and exact bytes agree with these corrections.

R01-001 and R01-002 retain four unknown bytes. Reused localized rejection review
in ../redirect/round01-contract-review/findings.md: isolated RTS and JMP lack the
independent structural constraints present in R01-003; boundaries alone do not
resolve code/data. No new evidence justifies artificial execution or repeated scans.
Gameplay evidence/checklist reused; representative later layouts and terminal
screen already exercised, no new concrete runtime question or supported ending
route. No redundant startup or emulator experiment was run.

Map annotated through configured release RPC, validated, saved and exported.
Fresh authoritative checkpoint pass-002-reviewed verifies byte-identical rebuild.
pass-002-final was accidentally created before successful edits while resolving
map.annotate's id=0 insertion convention; it verifies the unchanged round1 map and
is retained as a superseded attempt, never the round2 result. Two rejected RPC
insertions changed no state (missing id, then unknown nonzero id); not toolkit bugs.
No toolkit issues found. No toolkit edits, cleanup, archive or publication.

Existing commentary/source and review records are preserved. Existing review credit
remains valid for unchanged units, with full current-source commentary follow-up
needed for the new eleven-byte code slice and updated metadata. Do not reset from
new generated source over the editorial work.
