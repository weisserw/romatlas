# Round 001 disassembly handoff

Incomplete. ROM classification: 97.91% (256690 / 262160 bytes classified; 5470 bytes unknown).

Restore [state.json](state.json). Current portable mapping: [maps/current.json](../maps/current.json).
Final verified checkpoint: [maps/pass-001-final.json](../maps/pass-001-final.json),
[generated source](../generated/pass-001-final/game.asm),
[verification](pass-001-final-verification.json). Exact counts and residual ranges:
[round-001/counts.json](round-001/counts.json), [unknown-ranges.json](unknown-ranges.json).
All 599 regions have evidence; current-source commentary has not run.

This pass classified reset/NMI and contextual startup/gameplay closure; a bounded
103-command table with 29 handler entry contracts; fixed-bank transition, death,
player swap, rendering, collision and score routines; stage graphics/map/placement
and selector data; sprite/palette and encoded sequence data; seven MMC1 reset-transfer
helpers. Inline data barriers remain explicit. Generic Data does not claim a full
schema, original source layout or natural use. Static provenance conservatively
covers structural inference; accepted fetch evidence remains separate.

Progression: natural title/Mountain; artificial actual Bridges, Prairie, City,
Airfield, late Airfield with enemy vehicles, congratulations and aircraft ending
animation. Exact patches, inputs, budgets, outcomes and snapshots:
[scenarios.json](scenarios.json). Use game-clean.snapshot.json as the natural game
base; the earlier misnamed mountain-clean snapshot is still title. Reassert buttons
after import. The recorded aircraft scene uses the actual JSR $c45b at $c12c after
establishing its calling contract. No live server is required.

Next phases: finish current-source commentary, then investigate connected R001
command-handler returned-call/inline-data contracts and R002 audio/aircraft
selectors. R003 small fixed islands need neighboring consumer and boundary checks.
See [leads.json](leads.json). Use saved roots/snapshots and exact unknown complement;
repeating broad inputs now offers little. Do not require fetches for supported
static code or generic data. Post-animation next-loop transition remains deferred,
with a practical snapshot continuation; no global unavailability claim is made.

Compact rejection review: a blanket bank-1 code continuation competes with real
inline payloads. Audio ordinary-return assumptions are contradicted locally by
PLA/PLA/RTS at CPU $9e42; this rules out blind fallthrough, not the handlers' code
role. Tiny fixed gaps compete with encoded sprite/lookup data and misaligned
instruction boundaries. Narrower data slices remain admissible under the evidence
policy. These concrete tests justify redirect, not terminal exhaustion.

No toolkit issues found; zero findings and blockers:
[toolkit-issues.md](toolkit-issues.md). Input unchanged; no toolkit edits, archive,
cleanup, publication or deletion. All finite runtime batches exited.
