# Localized terminal rejection review

Input: round-02-disassembly checkpoint plus completed current-source commentary.
This is a reasoning review of the surviving one byte, not an unchanged replay of
r01 gameplay or a requirement for observed execution. Final editorial audit is
separate and does not gate this redirect.

Exact claim tested: physical [0x1CAF1,0x1CAF2), CPU $CAE1, permanent physical
8KiB bank14 ($C000 slot; bank15 at $E000), byte $60 is a standalone empty return
routine, or a structurally redundant final RTS of DefaultNmiFrame. Unknown lower
banks do not affect this interval. Alternate generic-data claim also considered.

Primary evidence: [audit](primary-audit.json), reproducible [script](audit.py),
[current source](../../../commentary/source/game.asm:10357),
[scheduler](../../../commentary/source/game.asm:12708),
[callback installers](../../../commentary/source/game.asm:12988),
[prior localized decision](../r01-connected-20261127/residual-contracts.md:67).
Audit uses all 12426 independently decoded accepted instruction starts; byte
coverage equals all26849 accepted code bytes. No direct decoded reference targets
CAE1. No literal little-endian E1 CA word occurs anywhere in the physical ROM.
These results exclude those specific encoded reference forms only. They do not
exclude arithmetic-generated addresses, split immediates, RAM continuations,
modified returns or unrecorded execution. No negative reachability claim follows.

Changed method versus r01: combine full current-source callback-writer review,
complete accepted-instruction-reference coverage and actual stack continuation
semantics; explicitly test whether a tail-call's return requires the nearby RTS.
This corrects any interpretation of the earlier barrier as requiring a live caller.
Expected discriminant: an independently constrained return/dispatch path to CAE1
would support code without observing it; conversely a real format/byte-pattern
constraint could support data. Stop after local contract and exact-reference
assessment if both interpretations remain compatible.

Combined independent constraints: CADE..CAE0 is JMP $D4F9 (4C F9 D4), not JSR;
CAE2 begins the independently vector-rooted IRQ routine (PHA ... PLA RTI). NMI
CAA6 saves A/X/Y and JSRs CABA; CABA JMPs through $0404/$0405. Default CABD
falls into CAC8 display tail, whose JMP transfers to RunObjectTasks. That routine
calls each continuation with JSR $D532, then ends JMP ($0402). ResetTaskSystem
installs $D07C sprite builder as post-task callback. This tail chain preserves the
return from CAAF's JSR, so a normal downstream RTS returns to CAB2 (restore Y/X/A,
DEC gate, RTI). The CPU never pushes CAE0 for the tail JMP. Thus a nearby return
is not demanded by the NMI stack contract. NMI callback writers are inline installer
D5E6 and argument wrapper D8DF; saved inline default is CABD, inventory E146 sets
CABD, map EDAD sets F399. The independent IRQ vector is CAE2, not CAE1.
No local inline-word consumer extends across CAE1; no adjoining data pattern or
bounded record schema establishes it as non-code. Exact boundaries are known;
structural role is the actual missing discriminant.

Strongest code interpretation: unreachable redundant RTS after an optimized
JMP, or intentionally available no-op callback. It is locally plausible and
compatible with every measured fact. However, it predicts no additional mechanical
behavior beyond the single valid $60 decoding and supplied hypothetical return
stack. Neither surrounding routine needs it; assigning an arbitrary routine entry
and compatible stack would equally make an isolated $60 data byte pass. A concrete
supported precondition/role (not necessarily an actual caller) remains absent.
The strongest competing interpretation is a spare byte/data left between routines;
that is also compatible, and the single byte has no uniform-run or recognizable
noninstruction-pattern support. Adjacency alone does not distinguish them. We do
not assert historical optimization, dead code, original layout or padding.

Disposition: retain exactly one unknown byte. No supported structural proposal or
useful revised bounded test remains identified. Artificially executing RTS with a
fabricated stack would demonstrate known opcode behavior, not discriminate the
claims. Arithmetic callback sweeps, arbitrary RAM patches and repeated gameplay
lack a specific supported mechanism selecting this address. Stages0..7, ending
and inventory were already exercised (analysis/scenarios.json); natural boss
victories remain deferred for lack of connection to this byte, not lack of effort.
Generic-label/semantic questions throughout fully reviewed source concern already
classified bytes and add no residual classification method. No data/table extent
question remains in the exact complement. No new runtime experiment was warranted.

This review is terminal only on the present evidence. Reopen for a concrete
continuation producer, independently justified routine contract, format constraint
or corrected inference that discriminates this byte; observations are welcome but
are not a universal prerequisite. Coordinator must inspect this review and its
primary source/audit before accepting the incomplete stopping decision.
