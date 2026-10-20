# Unfinished disassembly and reference corrections

The frozen archive is a verified starting point, not a claim that every byte or
comment is understood. Keep reference/ unchanged. Port completion uses explained
best-effort gameplay implementations and focused unit tests. Full
classification, source-proof exhaustion and runtime fidelity validation are
outside that completion gate. Follow prompts/start.md's architecture, debugger
and stopping boundaries throughout this procedure.

## Assess uncertainty as the port develops

uncertainties.json starts with every archived Unknown interval, including tiny
fragments. Preserve its IDs, physical ranges and source_region_ids. Add semantic
uncertainties in classified source with unique IDs and ranges. Each entry has:

- impact: unknown initially, then behavior, data or none;
- status: pending, then resolved (implemented assessment), preserved (data only) or excluded (no impact only);
- summary: interpretation, consequences and the reason for the disposition;
- consumers: routine/table/asset references and effective CPU/bank context;
- ledger_ids: affected original or discovered behavior region/unit IDs;
- evidence: source/evidence locations, reasoning and any reference revision IDs;
- validation: focused unit tests and actual check results; any source-question debugger experiment must obey prompts/start.md;
- validation_limits: untested routes and residual uncertainty, or an explicit none.

These are port-impact decisions. Resolved means the relevant port behavior is
implemented and accounted for using source reasoning or an explicitly documented
reasonable implementation assumption. It does not assert that the original
semantics are proven or runtime-equivalent. For an assumption record its basis,
chosen behavior, alternatives and residual uncertainty in
summary/evidence/validation_limits; do not invent observations. Preserved means
a supported data consumer loads the original ROM-backed values with correct
bounds; it does not mean unknown logic has been implemented. Excluded requires
an explained no-impact case. A gap's size, lack of observed reads or a build
success alone supports none of these dispositions. The audit checks records, not
their semantic truth. Missing required implementations cannot be hidden under
preserved/excluded or a generic differences note. Implemented assumptions can be
resolved for port accounting while source/fidelity uncertainty stays explicit.
Make a reasonable choice and continue rather than indefinitely investigating
confidence. Actual inability to implement required behavior is a concrete
blocker; unverified similarity is not. Related fragments can share one consumer
analysis and evidence report; link that report from their entries instead of
repeating the investigation for each byte.

## Correct the reference when a discovery warrants it

This procedure is optional and applies only to supported source corrections that
help implementation. Do not create reference revisions to prove the native port
correct or reclassify the whole ROM. Implementation assumptions may remain in
port records without rewriting the reference. Debugger use needs a specific
unresolved source question, a bounded experiment and a stopping condition;
native frame/audio/state/scenario comparisons are not permitted as port
validation.

1. Run `python3 run.py reference-init` once. It creates research/working/source/,
   state.json, reading.asm and changes.json from the frozen local reference. Resume
   that working copy thereafter; do not initialize over it. It contains local ROM
   material and is ignored by Git. No original archive or other analysis workspace
   is modified. Only one writer should edit/adopt a reference revision at a time.
2. Record the discovery and inspect the original bytes/consumers with effective bank
   context. Read the toolkit's project.md, scripts/workflow-template/evidence-policy.md and platform
   documentation (docs/sym.md for Game Boy, docs/snes.md for SNES). Distinguish a
   structural inference, observed execution and a conditional calling contract.
   Useful static evidence does not require a natural caller; valid decoding or a
   plausible pattern alone is insufficient. If evidence is still weak, retain the
   hypothesis in uncertainties.json rather than publishing it as established fact.
3. Edit maintained assembly comments, labels and symbols in research/working/source/.
   For structural changes, use the toolkit map APIs against research/working/state.json:
   open a dedicated `serve` process with the configured binary, explicit --platform
   gb or snes and --rom input/game.rom; map.load the working state, inspect/annotate,
   map.validate and map.save back to that file. Split regions before inserting
   smaller ones. Keep annotations/provenance and edited assembly consistent. The
   verify command exports the map from this state; no hand-edited second map is needed.
   Decode new routines into real assembly where justified; keep unsupported bytes
   raw. Preserve instruction widths/aliases and bank/upload context. Never regenerate
   over maintained commentary; generate inspection output only into a new directory.
4. Maintain changes.json as cumulative evidence-linked corrections from the frozen
   baseline. Use version=1, rom_sha256 from workspace.json and a changes array. Each
   correction has id, kind (commentary or classification), offset, length, before,
   after, port_impact and evidence (nonempty strings). Describe uncertainty honestly.
   Put supporting local files in research/working/evidence/ so revisions retain them.
   The initial working/reading.asm is a baseline convenience; use the freshly expanded
   reading.asm in the next verified revision after edits, and never edit reading copies
   independently of their source. Verify coherent correction batches, not every line.
5. Run `python3 run.py reference-verify --revision findings-001` with a new name.
   It snapshots the working state/source/changes/evidence, validates and exports the
   map, independently generates/rebuilds structural output and rebuilds the edited
   source against the ROM. Success publishes an immutable research/revisions/ snapshot
   and adopts it through research/current.json. A failure publishes no revision and
   leaves the previous selection unchanged; fix the working copy and retry. The
   snapshot contains a refreshed reading.asm and both verification results. Neither
   verification assesses semantic correctness or proves map/comment consistency.
6. Link the revision, corrected source and evidence in uncertainties.json and the
   corresponding ledger.json entries. Keep original source IDs/ranges intact; add
   newly discovered units with physical ranges instead of rewriting the frozen
   inventory. Update native code and meaningful tests when interpretation changes.
   Record what changed and remaining limitations in progress.md/differences.md.

The active reference is the frozen baseline plus the adopted revision. The
baseline remains authoritative for provenance; the revision supplies explicit
corrections. Read the selected revision on resume, and update focused native
unit tests when changing an implemented interpretation. Do not edit published
revisions in place; new discoveries produce new revisions. Keep all
source/assets/evidence local. Transfer corrections back upstream as an
evidence-linked handoff (changes.json, mapping, revision identity and
verification); never overwrite the source archive or a concurrent analysis run.
Only portable maps and ROM-free research metadata are candidates for sharing.
