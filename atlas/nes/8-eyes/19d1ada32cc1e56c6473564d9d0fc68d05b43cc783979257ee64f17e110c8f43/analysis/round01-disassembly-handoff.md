# Round01 disassembly handoff

ROM classification: 86.96% (227971 / 262160 bytes classified; 34189 bytes unknown).
Single disassembly pass finished; ROM incomplete. 11629 code, 85270 data,
131072 CHR image bytes, 508 annotated regions. Exact per-bank totals:
[round01/per-bank.md](round01/per-bank.md); unknown complement:
[round01/unknown-ranges.json](round01/unknown-ranges.json).

Final verified checkpoint: [report](round01-disassembly-verification.json),
[frozen map](../maps/round01-disassembly.json),
[source](../generated/round01-disassembly/game.asm). Validation and byte-identical
rebuild passed. State saved, portable map exported; input and toolkit unchanged.
Readable routine names/behavior commentary still pending next phase.

Evidence: [data inspection](round01/data-inspection.md),
[static candidates](round01/static-candidates.json), scenario contextual fetch
files and bounded append-only [RPC ledger](round01/rpc.jsonl). 4293 retained unique
physical fetched opcode offsets; aggregate drops zero. Detail capacity0 drops
are intentional, separate from execution aggregates. Static closure includes
unfetched branches without claiming natural reachability. Early button/pressed
calls silently released inputs: those stage captures were attract demo, corrected
in all current Observed evidence. [NES-001](toolkit-issues.md) is one confirmed
API/documentation bug; no blockers, suspected bugs or missing features.

Gameplay: [scenarios](scenarios.json), [familiarization](gameplay.md). Real input
buttons arrays reached menu and stage selector; A1/A4/A7 branches visited three
castle exteriors and representative movement/sword/jump/falcon mechanics.
$0050=7 before Start visited Ruth artificially. Natural initializer stop at
PC=$EE00 matched1053operationcalls; patch $0051=7 loads later rooms for stage1/7.
No boss presence or defeat confirmed. Artificial PC=$D4E6 from initialized Ruth
context[5,13,14,15], original stack253, displays altar puzzle. This is supported by
stage7 completion JMP and scene initialization semantics. Puzzle snapshot preserved.

Eight-slot validator compares $05F0..7 to mapped bank13 $A91E bytes
[2,0,1,6,7,4,3,5]. Direct $D56A entry with these patched slots reaches success
PC=$D582 in204operationcalls. A120frame untraced +10frame traced continuation
remains altar; no terminal/credits claim. Earlier $D564 entry also stayed altar.
[Probe](round01/validator-probe.py), [result](round01/validator-result.png),
[snapshot](round01/validator-result-snapshot.json) preserve exact evidence.

Redirect questions: R01-CALLS inspect helper bank-preserving return contracts and
post-call code, particularly banks13/14; discovery conservatively stops at calls.
R01-ENDING inspect $F85A delay/task/stack contract and $D582->$D6C3 transition
before another ending probe. R01-ROOMS inspect boss spawn/door/position contract
before claiming boss encounter. R01-DATA inspect remaining tables in banks9–12,
localize boundaries and retain generic Data where repeated patterns justify it.
Full schemas are not prerequisites for code/data separation. Low banks0–7 are
Data on repeated grids/arrays/records, not absence of fetches alone.

This is easy-gain handoff, not terminal exhaustion. Remaining work has concrete
methods and therefore belongs in commentary/redirect, without another startup
baseline or unchanged gameplay replay. No cleanup/archive/publication authorized.
