# Behavioral accounting across phases

Every phase inherits the goal of accounting for released-game behavior and its
implementation/asset dependencies. No byte-classification percentage determines
completion. Static analysis in any phase may identify additional code, data,
formats or extents. A later discovery reopens affected behavioral obligations and
opaque-region dispositions; an earlier acceptance is evidence to review, not an
immutable assertion of truth.

Disassembly establishes structural/behavioral accounts and byte-identical source.
Commentary adds useful semantic explanations linked to the behavior inventory.
Native porting adds implementation equivalents and useful behavioral tests.
A later phase may resolve an earlier unanswered question. Preparation preserves
upstream outcomes but never promotes an incomplete account to success. Each
phase must satisfy its current obligations before reporting complete.

## Inventory and ending records

`analysis/behavior.md` holds the disassembly narrative; `analysis/behavior.json`
is its machine-readable companion. Commentary freezes these claims and existing
deferred-ticket records in `commentary/handoff.json`, then creates an editable
`commentary/behavior.json`. The archive retains both. Port preparation copies the
archived inventory to `behavior.json`, preserving IDs, evidence and unresolved
questions but resetting item status, scope review and outcome for the new phase.

Missing legacy inventories start empty and pending. Recover scope from existing
research/source/evidence before claiming completion; a source-region checklist
alone is not a behavior inventory. Modes, mechanics, progression and terminal
states must be considered, including unvisited behavior identified by research
or static analysis. Every inherited ID must remain; refinements may improve its
summary, evidence and dependencies. Add IDs for new findings rather than erase
obligations. Inherited unresolved questions require explicit `resolutions` records
with the original `question`, a `resolution` explanation and nonempty `evidence`
statements; silently deleting a question cannot satisfy the audit. Dependencies
reference other behavior IDs; unresolved questions are
explicit, not hidden in a source exclusion.

The JSON schema is version 1:

```json
{
  "version": 1,
  "revision": 1,
  "rom_sha256": "matching local ROM SHA-256",
  "phase": "commentary",
  "scope": {
    "status": "reviewed",
    "summary": "Scope and version applicability considered in this audit",
    "evidence": ["Durable account of research, static analysis and tested observations"]
  },
  "items": [{
    "id": "B001",
    "summary": "Movement and its state transitions",
    "status": "accounted",
    "evidence": ["Explain the implementation evidence and bounded uncertainties here"],
    "dependencies": [],
    "unresolved": [],
    "regions": ["region-1"],
    "source": ["bank_000.asm:12"]
  }],
  "outcome": {"status": "complete", "notes": "Acceptance audit and limitations"}
}
```

Use actual identities and source locations, not the example's placeholders.
`phase` is `disassembly`, `commentary` or `port`. Commentary items require valid
source locations and links to semantically reviewed scaffold regions. Port items
require links to implemented/merged/replaced ledger regions, nonempty `equivalents`
and `tests` descriptions connecting actual implementation and useful validation.
Passing schema checks cannot judge test usefulness or explanation quality.
Disassembly items need structural implementation evidence; the later fields are
added when their phases begin. `revision` tracks inventory edits.

Before success all items must be accounted, dependencies resolved, unresolved
questions empty (evidence-bounded interpretations belong in evidence), scope
reviewed and outcome complete with notes. Native-port completion also needs the
finished source ledger, useful passing tests and its documented handoff. Commentary
also needs source review, editorial audit, byte identity and refreshed reading copy.

Continue concrete distinguishing investigations. Runtime needs a named behavioral
evidence target. Stop `incomplete_blocked` only when required questions have only
blocked investigations; stop `incomplete_exhausted` only after applicable methods
leave no identified distinguishing experiment. Audit scope, dependencies and all
unknown dispositions; save work and describe attempts, unmet conditions and reopen
triggers in outcome notes/progress. Mixed cases identify each question's condition.
Do not repeat unchanged exhausted work on resume. These are report outcomes, not
new provider-tool statuses: respect provider status/pause rules and report limits.
Effort, low byte yield and a raw-byte rebuild alone establish neither outcome.

## Physical preservation and semantic review

Commentary's nonoverlapping full-ROM scaffold remains a preservation/navigation
inventory, not a requirement to explain opaque bytes. Every row has reviewed
status, summary, valid source locations and one of two treatments:

- `semantic`: review known routines and meaningful data structures. Code and newly
  interpreted Unknown regions need reviewed units; the frozen kind does not change.
- `preserved`: carry opaque material verbatim with a `preservation` object containing
  `rationale`, `evidence` (durable statements) and `reopen_when`. Known Code or rows
  with semantic units cannot use this treatment to skip reading.

Preservation requires triage finding no unresolved behavioral dependency, no
unexamined credible code/asset hypothesis and no concrete follow-up answering a
reconstruction question. Nonexecution alone is insufficient. Check inherited
triage against source and new findings without repeating unchanged experiments.
Call these ranges unexplained, not proved unused. An uncertain routine purpose can
still be explained operationally; an unknown gameplay-affecting selector is a
behavioral dependency that must be resolved or evidence-bounded before success.

Port ledgers retain original treatments and preservation rationale. A row inherited
as preserved may receive `disposition=preserved` after a fresh documented review,
without an implementation equivalent. A legacy Unknown row lacking an inherited
disposition can receive preservation only after new documented triage. It remains
part of accounting. Source units
require normal implementation/merge/replacement/exclusion accounting. New findings
can turn a preserved row into implementation work; behavioral IDs cannot be waived
by excluding or preserving the underlying region.

## Refining ROM understanding later

Use `refinements.json` with version 1, phase and entries containing unique `id`,
`status` (`pending` or `reconciled`), `summary` and durable `evidence`. Explain in
`refinements.md` the old/proposed kinds and bounds, CPU/bank context, affected
behavior IDs, map checkpoints and source/asset-recipe reconciliation. Pending
entries block completion. Porting carries archived reconciled entries forward;
they cannot be removed. Audits check record state, not the truth of reconciliation.

In commentary, snapshot shared maps/state, apply supported corrections through
normal ROMAtlas APIs and verify a fresh checkpoint. Keep the frozen baseline and
handoff intact. Reconcile edited source manually, preserving comments and bytes;
never regenerate over it. Units can express refined interpretations inside the
original coverage scaffold. Reverify commentary and refresh the reading copy.

In a port workspace keep `reference/` immutable. Put new analysis, maps and optional
assembly under `analysis/`, using ROMAtlas with the same local ROM and bank context.
Byte-verify new structural source, add discovered source units, update behavioral
accounts and reconcile the native implementation/asset recipes. Frozen original
ledger identities remain historical anchors, not restrictions on new analysis.

## Archives and migrations

Archives retain narrative and structured inventory, frozen handoff, reviewed opaque
dispositions and refinement records. Acceptance rationale must be self-contained
in these records; links to excluded scratch files alone are insufficient. Original
scratch traces and extracted assets remain excluded. Older archived results are not
rewritten; restoring them still works, and new port preparation marks missing
behavioral accounting pending.

For stopped existing workspaces, metadata-only migrations preserve source edits,
ROM/maps, frozen reference and historical evidence, with backups and fresh audits:

```sh
python3 scripts/upgrade-disassembly.py /path/to/game --apply
python3 scripts/commentary.py upgrade /path/to/game
python3 scripts/port.py upgrade /path/to/port
```

The latter commands migrate existing phases, not prepare replacements. Existing
inventories are preserved, missing ones are seeded pending, old review/ledger work
remains, and completion audits must be renewed. They do not update provider-stored
goals or launch/resume agents. Replace obsolete stored goals through the provider's
supported interface before resuming; use the current generated goal/prompt.
