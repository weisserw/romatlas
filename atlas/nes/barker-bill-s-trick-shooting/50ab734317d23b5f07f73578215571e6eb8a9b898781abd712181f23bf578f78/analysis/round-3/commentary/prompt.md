# Incremental commentary and final editorial review

Read commentary/AGENTS.md and workflow.md. Work in commentary/source/, preserving
phase-1 maps/evidence and the frozen before/ baseline. Read the current checkpoint,
phase handoff and relevant analysis records; retrieve details only as needed.

Readable assembly is a required deliverable and the porting agent's source input.
Produce a fully commented and labeled listing: explain every meaningful routine,
control-flow block and data structure, with descriptive routine and branch labels,
RAM variable names, object-field offsets and constants used consistently in operands.
Comments explain purpose, algorithms, inputs/outputs, state changes and relevant
bank/format constraints; they need not paraphrase every instruction. When game
meaning is unknown, name the established mechanical role and explain the remaining
uncertainty nearby. Do not invent meanings or treat an address-only label or raw
address as an adequate explanation of an understood role.

Before falling back to generic data, make a practical attempt to identify its role:
inspect available references and consumers, nearby code and labels, and obvious
formats or repeated record layouts. Use supported specific region types, meaningful
labels and concise format/role explanations when the evidence permits. Generic data
is a fallback after this bounded inspection, not the default shortcut. Record briefly
what was checked and what remains unresolved; reuse prior sound inspection rather
than repeating it. Follow concrete promising leads, but do not require exhaustive
semantic research or block classification when further investigation is unproductive.
Never invent a specific meaning or label just to avoid the generic fallback.

Generic data blocks may remain unlabeled when their format or purpose is unknown.
A short boundary/uncertainty note is sufficient for these blocks; semantic research,
consumer identification and per-byte commentary are not completion prerequisites.
Preserve established useful data names and explanations.

Every commentary invocation must bring ALL currently classified source up to this
readability standard before handing off. Catch up the entire existing readability
backlog, including older uncommented code and concrete defects in reviewed material;
do not choose a small backlog slice. Unknown ROM regions need honest boundary/role
and uncertainty notes, not fabricated decoding or invented meanings. Unknown game
purpose does not excuse skipping explanation of established mechanics.
Preserve valid prior review credit and good existing prose; later invocations may
reuse that work and focus edits on changes, affected dependencies and remaining gaps.
Inspect the full current source inventory for gaps, without cosmetically rewriting
sound material. Report classification and source readability separately.

Use descriptive routine, branch, RAM variable and constant names wherever useful.
Accept the best coherent static interpretation from instructions, callers and
consumers; runtime proof and the original author's exact intent are not required.
Replace numeric operands with symbols when their roles are understood, preserving
exact values. Keep address-based identifiers where roles remain unknown. Explain
purpose and algorithms rather than translating instructions. Document formats,
units, bounds, bank context and consumers once, near the relevant structure.
Write clear, light prose: short functional comments in natural language, with
only the detail needed to understand purpose, behavior or a material assumption.
Avoid compressed address-heavy shorthand and mechanical opcode narration.
Rewrite existing explanations rather
than stacking duplicate old/new comments; remove redundant opcode paraphrases.

Use ordinary tentative prose for material hypotheses, e.g. 'Probably selects the
landing animation; also called during damage recovery.' Separate exact mechanics
from inferred purpose. No graded question-mark taxonomy or marker quota is needed;
preserve useful older uncertainty notes until affected evidence merits revision.
Record significant assumptions and concrete unresolved questions, not every possible
doubt. Revise names/comments when contradicted. Do not relabel phase-1 ambiguity
or weaken evidence-backed structural classification to finish commentary.

Use bounded debugging only for a concrete question likely to clarify useful behavior.
Diminishing returns on semantic research permit an explicit unresolved explanation,
not an early exit from naming/commenting the rest of the source. Publish redirect
questions as they arise, then continue commentary until the current-source scope
is complete. A small repair, useful finding or prepared lead is not a stop condition.
If interrupted by context/time limits, save exact remaining ranges and resume the
same commentary phase; report interrupted, not successful or ready for redirect.

Keep review.json's full-ROM scaffold intact. Add compact records for meaningful
routines/structures with physical ranges, bank context, source locations, status
and useful summaries. Do not inventory every loop/branch by default or duplicate
an explanation across progress, inventory and uncertainties. Keep unreviewed work
pending. Update commentary/progress.md with this pass's scope, names/interpretations
changed, unresolved dependencies, deferred ranges and actual verification results.
Publish concrete questions for redirect as soon as available; link supporting
source/ranges and suggest bounded checks without requiring a full lead dossier.

Verify after coherent source-edit batches and at phase handoff; refresh the reading
copy at handoff. Do not repeatedly rebuild after tiny comment edits or run the final
completion audit while deliberately leaving review pending. A verified handoff is successful only after the current-source readability scope
is complete; this does not establish whole-ROM classification or final acceptance.

At final acceptance, address remaining pending review and known editorial issues:
missing structures, stale numeric aliases, wrong boundaries, contradictory names,
redundant comments and misleading certainty. Reuse valid reviews; do not schedule
a mandatory whole-source reread or cosmetic rewrite. Revisit reviewed material only
when changed evidence, dependencies or a concrete inconsistency warrants it.
Every scaffold entry and meaningful discovered unit must then be reviewed; an
explained unknown may remain unresolved. Run audit for full inventory/editorial
acceptance plus a fresh byte-identical rebuild and refreshed reading copy.
The audit cannot judge semantic or prose quality. Preserve upstream notices;
all ROMs, generated source/assets and traces remain local. Do not fix the toolkit.

## NES instruction and address context

Use physical file offsets including the iNES/NES 2.0 header and optional trainer.
PRG ROM, CHR ROM, CPU RAM, cartridge RAM, PPU VRAM, OAM and palette are separate
spaces. CPU aliases are not physical identities. For banked code supply effective
physical 8 KiB PRG bank indices in context.prg_banks for CPU slots $8000/$A000/
$C000/$E000; mapper register values are not bank indices. Static supported entry
contracts may establish context. Unknown banks remain unresolved locally.
6502 instruction lengths do not depend on status flags. All official and
unofficial opcode lengths decode; ambiguous unofficial encodings use exact byte
directives with mnemonic comments for rebuilding. Pointers are hypotheses, not
proof of code. Mapper writes/calls may invalidate static bank context. Discover
from validated reset/NMI/IRQ vectors and supported entry contracts, supplementing
with observed context/edges where useful. Structural classification, reachability,
whole-ROM classification, full current-source readability and editorial gates
remain separate and unchanged. Unknown bytes stay unknown.
Portable maps use romatlas-nes-map JSON and explicit CPU execution views. Read
docs/nes.md. Never use Game Boy .sym or SNES Asar syntax. Use bounded headless
emulator.step and immutable inspection. RAM execution does not prove a ROM upload
origin. Native helpers in romatlas-port::nes are independent MIT code; do not
copy GPL core code into the native runtime. Baseline debugger targets NTSC,
standard controllers and mappers 0/1/2/3/4/7. Other mapper/peripheral support is
explicitly deferred. No full hardware/game compatibility claim follows.
