# Residual reasoning review — seven proposed-unclassified bytes

This is a ready-batch handoff, not terminal whole-ROM exhaustion. The current map still has27999 Unknown bytes; these7 remain outside the27992-byte proposal. Independent disassembly may reject or narrow proposals. Structural role, natural use and semantic meaning remain separate claims.

## Isolated RTS bytes (five bytes)
Physical108fb (CPU88eb),10e2c (CPU8e1c),14422..14424 (CPU8412/8413),14558 (CPU8548). Decisive primary evidence: immutable ROM/current source neighbors; actor-main-setters.json, actor-dispatch-tables.json, menu-table-contracts.json; actor-cfg.txt and combined-proposal code views. All reviewed supported handlers either return before these bytes or jump over them. Complete main/state/callback and menu action pointer inventories do not supply an independent entry here; direct control references in the reviewed graph likewise do not. This rules out those particular ordinary fallthrough/dispatch routes, not static code or unknown callers in general.

Strongest code interpretation: intentional empty or redundant return entries. Strongest competing interpretation: single-byte data/gap values. A return opcode alone and its adjacency to code do not discriminate. Artificially jumping to each byte would merely test RTS mechanics and supply no intended-role evidence, so it is not a useful proposed test. A future independent entry/vector/format constraint could resolve them. This is a code-versus-data ambiguity, not a missing game name or demand for natural execution.

Consistency check: fixed IRQ stub physical1c7e5 is now positively proposed Code from hardware vector2000e=>C7D5 and a supported IRQ stack contract (irq-vector-proof.json). Conversely physical1f0bb byte60 is positively Data through direct indexed metatile consumer and exact two-byte bounds (f0-boundary.json). Neither opcode identity nor lack of fetch decides the residual returns.

## Isolated zero bytes (two bytes)
Physical184c3 (CPU84b3 in sound bank6) and1fd8e (CPUfd7e fixed). Primary evidence: immutable bytes/current neighboring source, combined-proposal.json boundaries, full reviewed sound handlers and ResetEndingDisplayState. Both follow returns and precede established code or already-classified uniform data; the inspected consumers do not anchor either as a particular field. Zero could encode BRK, a terminator, reserved data or a gap. One zero alone supplies neither a repeated non-code pattern nor an independently supported execution/record boundary. Retain Unknown; no exact generator/semantic explanation is being demanded for otherwise clear non-code.

The fd7f onward uniformff suffix was already classified Data in pass-001. Its supported role remains accepted; unresolved bytefd7e does not contaminate that independent interval. Repeating unchanged artificial PC tests or relabeling the prefix as fill would not discriminate the current alternatives.

## Other unresolved behavior
Rescue initialization/alignment was tested successfully; no repeated boss flag patch. Genuine type70 descriptor initializes health121 and the improved position contract keeps it active30frames, but the inspected frame does not unambiguously identify a boss or natural boss route. The direct supported initialization and representative firing test are preserved. More actor/room sweeps would require unsupported encounter-identity assumptions and currently add no credible unknown-structure gain beyond this ready batch. Natural progression/route meaning is deferred for that specific reason; broad later surfaces/cave/ending/credits evidence remains valid.

No productive classification experiment is left pending in this attempt. Proceed to independent disassembly review of the combined batch; do not interpret these residual notes as final editorial acceptance or ROM completion.
