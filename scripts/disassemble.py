#!/usr/bin/env python3
"""Prepare isolated, local agent workspaces for owned Game Boy ROMs."""
import argparse
import hashlib
import json
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import tempfile

import behavior

ROOT = Path(__file__).resolve().parents[1]

EFFICIENCY = """
## Efficient investigation and resumption

No context-management plugin is required. Use local scripts and focused searches
to summarize large traces, JSON and logs; return relevant counts, excerpts,
failures and evidence paths instead of complete dumps. Use bounded/paginated
queries and direct tools for short reads or edits. Inspect exact source, bytes
and bank context before accepting conclusions; summaries are not proof. Keep
full evidence on disk and concise progress/next-action records. Retrieve older
sections on demand, avoid unchanged repeat queries and redundant reports, and
never reduce scope or verification to save context.

Use the configured release binary. If it is unavailable, have the toolkit rebuilt
with `cargo build --locked --release`; do not silently substitute an older binary.
Read analysis/frontier.md and the latest progress/batch record first on resume.
Read older ledger sections only for the selected intervals and their dependencies;
preserve the full history without repeatedly loading or rewriting it.

Run `python3 analysis/work.py refresh` after loading/saving the current state to
refresh a compact queue and per-bank audit. Each unknown interval remains tracked.
Behavioral relevance takes priority; size is a secondary triage heuristic. Record
a concrete hypothesis, experiment,
expected evidence, behavioral relevance, dependencies and revisit condition via
`python3 analysis/work.py plan PLAN.json`. Prefer high-value structural decisions;
a small helper is worthwhile when it unlocks a larger data/code family. Do not
repeat a failed scan without a changed prerequisite or a distinct hypothesis.

When a structural hypothesis stalls on a specific semantic dependency, make
explaining that subsystem the next experiment: inspect connected callers/callees,
RAM state, selectors, formats and bounds. State which classification decision the
explanation should unlock; do not start an unrelated whole-game commentary sweep.
Preserve useful names, behavioral summaries, invariants, physical ranges/bank
contexts, evidence and competing interpretations in analysis/investigation.md
and supported region metadata. Link the concise result from the queue/batch so
the later commentary agent can reuse it. A failed experiment must identify the
remaining dependency and reopen condition. Semantics are already permitted;
this is a dependency-driven way to use them, not a new phase or completion rule.

If commentary already exists, incorporate its evidence-backed structural findings
through normal validated map APIs and fresh verification. Coordinate a single
writer for shared maps/state. Preserve commentary/source and its frozen baseline;
never regenerate over edited assembly. Record any unreconciled correction and
its affected ranges explicitly. Structural and editorial completion stay separate.

The installed analysis/session.py preserves `from session import call` clients.
Start it with `python3 analysis/session.py --journal traces/session-NEW.jsonl`.
It binds to this workspace's ROM and binary. Cache hits for analysis.decode avoid
RPC entirely, keyed by actual ROM/binary identity and all parameters; use bounded
multi-instruction reads when useful. Nothing else is cached: map-sensitive
discovery, bank contexts, runtime observations and mapped memory must stay fresh.
Rebuild/replacement of the binary requires restarting the bridge. A new bridge
still needs map.load, emulator.start and input replay; old snapshots are gone.
`python3 analysis/session.py --cache-stats` reports local cache storage.

Page references and diagnostics independently with reference_offset and
diagnostic_offset in analysis.discover_context. Reuse explicitly reviewed
ROM-bound inline-dispatch/non-mapper-store profiles as documented in
docs/protocol.md; one observed register value is not an all-arrival invariant.
No profile or reached target proves execution or overrides a data barrier.

Group related micro-investigations into coherent passes. Validate/save/export
accepted map batches, then rebuild/verify once per pass and at final handoff.
Never waive validation, evidence, or the final byte-identical check. The standard
checkpoint now handles large uncoalesced maps; coalescing remains optional.
Avoid a new reporting program per pass. Use
`python3 analysis/work.py record BATCH.json` with id, summary, evidence paths,
optional verification report and ticket_updates. It records current accounting
and concise ledger/progress deltas while retaining historical entries. The
verification report must match the current ROM and exported map to be called
current. No helper promotes candidates or certifies behavioral completion.

Plan runtime around a named unresolved consumer/selector or new scene, with a
replayable route and expected observations. Consult analysis/gameplay.md and
scenarios.json; pursue the recorded untested routes where they address a current
hypothesis. Avoid unguided long runs and repeated read-only inspection of the same
state. Nonexecution, entropy, duplicate bytes and bank tails are not data/padding
proof. Existing public disassemblies and format notes need local confirmation.

Keep the existing economical model/reasoning for routine extraction, reviewed
edits and deterministic reporting. A stronger-reasoning trial should address a
specific difficult invariant or high-value region, using the same starting state,
tools and experiment allowance as the cheaper trial. Record accepted progress,
resolved dependencies, corrections, actual metered usage and total time including
cooldowns in the batch's model_trial field. The prompts do not change model
settings automatically. Do not launch parallel model workers under usage limits.
Operational experiment bounds are never a total investigation or completion budget.
"""

AGENTS = """# ROM disassembly workspace

Read workflow.md, workspace.json, analysis/progress.md, and
analysis/toolkit-issues.md before work.
Read and maintain analysis/gameplay.md for sourced gameplay familiarity and
targeted experiment routes, following workflow.md's Game familiarization section.
The toolkit repository is {toolkit}. Read its project.md, docs/progress.md,
docs/protocol.md and docs/sym.md. Use its existing binary; this workspace is
for disassembling this game, not changing the toolkit.

Objective: account for released-game behavior, classify supported regions with evidence,
export a portable stock .sym map,
and generate a bank-organized assembly listing that rebuilds byte-for-byte.
Investigate structure and semantics together; name and explain routines where
useful. Required annotations describe structural role and supporting evidence;
exhaustive explanations of every instruction are optional.
Do not stop after a raw-byte baseline: fidelity and behavioral accounting
are separate results. Maintain analysis/behavior.md under the current ending
contract. Report unknown bytes and unresolved bank contexts honestly.

Use python3 run.py inspect/baseline/serve/checkpoint as described in workflow.md.
Keep input/game.rom unchanged. Never download ROMs. Keep input, internal state,
traces, source and extracted assets local; only stock .sym is distributable.
Never commit the entire workspace. .gitignore is a guard, not authorization.

Use bounded execution, discovery and paged queries. Trace addresses are physical;
retain CPU aliases and effective bank context. Prefer analysis.discover_context.
Observed instruction fetches prove execution; reached targets and pointer values
do not. Unexecuted bytes may be classified using static or semantic evidence;
unsupported bytes stay unknown. Export's raw-byte gaps are not data evidence.
Inspect candidates before map.annotate. Split existing regions before inserting
smaller regions. Validate maps after edits. Save internal state with map.save
to analysis/state.json, and export stock .sym to maps/current.sym.

Record evidence, confidence, scenario input sequences, remaining unknown ranges,
and exact tested verification results in analysis/progress.md after each batch.
Snapshots are session-local; save replayable input sequences for later sessions.
Do not invent snapshot persistence or claim full hardware compatibility.

Maintain analysis/toolkit-issues.md as you encounter toolkit bugs or missing
features, following workflow.md's reporting requirements. Preserve findings
across sessions. Continue ROM analysis using workarounds unless blocked; do not
fix the toolkit in this workspace. At the end of every pass, summarize toolkit
findings and link the report, or explicitly say "no toolkit issues found" if empty.
"""
AGENTS += """
Follow workflow.md's Efficient investigation and resumption section. Read
analysis/frontier.md when present and use analysis/work.py for current accounting,
plans and batch reports. Use analysis/session.py for cached immutable decoding.
Preserve all historical evidence; update current frontiers with deltas. Follow the
current behavioral completion contract over superseded historical goals. prompts/goal.md contains
the persistent objective; prompts/resume.md is also binding on resumed passes.
"""

WORKFLOW = """# Per-game workflow

Run your agent in this directory. Give it prompts/start.md for a first pass or
prompts/resume.md for a later pass. No particular agent provider is required.
Both prompts pursue the same behavioral reconstruction objective. A pass is a
checkpoint, not goal completion. Follow the completion and investigation contract below.

1. Run `python3 run.py inspect`, then `python3 run.py baseline` once. Baseline
   creates maps/baseline.sym and generated/baseline and verifies the rebuild.
   It marks raw bytes for export only; it is not a classified disassembly.
   After inspection identifies the game and likely version, perform the Game
   familiarization step below before planning runtime scenarios.
2. Run `python3 run.py serve` as a persistent JSON-RPC subprocess. Use
   api.describe. A new session has empty analysis state. Load analysis/state.json
   with map.load when resuming. Do not import the raw baseline as classification.
3. Discover from entry offset 256 and valid interrupt/restart entry points after
   inspecting them. Supply effective CPU/bank context and bounded budgets to
   analysis.discover_context. Inspect candidates, references and diagnostics.
4. Start the emulator, inspect capabilities, enable bounded tracing, and run
   reproducible startup/menu/gameplay inputs. Save named snapshots for in-session
   branching. Record both presses and releases and execution budgets in
   analysis/scenarios.json. Clear traces before replay for a forward timeline.
   Restart/reset disables tracing; re-enable it. Inspect drops and budget stops.
5. Merge observed roots/edges with static discovery. Inspect data references and
   explicit analysis.pointers hypotheses. Confirm only supported regions with
   map.annotate, preserving confidence/provenance; leave other bytes unknown.
6. Call map.validate, map.save to analysis/state.json, and map.export to
   maps/current.sym (include_sym=false). These paths are relative to this workspace.
7. Run `python3 run.py checkpoint pass-001` (use a new name each time). It validates
   current.sym, generates from it, and verifies the new source directory. Each
   successful checkpoint saves a copy of the exact map used and a verification
   report. Keep the internal store and progress notes alongside these results.
8. Repeat for actionable behavioral dependencies and unknown-region triage. Report
   inventory coverage and resolved behavioral questions first, followed by classified
   code/data/unknown byte counts from the internal store, uncertainty, execution
   scenarios exercised, verification evidence, and concrete next tasks.
   Include a summary of toolkit findings and a link to analysis/toolkit-issues.md;
   explicitly say "no toolkit issues found" when the report has no findings.

## Game familiarization

Consult available online manuals, walkthroughs, FAQs (such as GameFAQs), gameplay
guides and relevant technical resources after identifying the game and likely
platform/region/revision. Do not rely on training-set familiarity. Record concise,
actionable findings in analysis/gameplay.md with source URLs, applicable version,
uncertainty and confirmation status. Preserve and update this record on resume;
create it if absent. Record controls, menus, modes, progression and distinctive
mechanics; routes to later levels, bosses, bonus stages, endings and unusual states;
and passwords or built-in shortcuts that enable practical targeted experiments.
Do not download ROMs or extracted game assets as part of research.

Turn findings into specific runtime hypotheses, evidence sought and reproducible
input routes in analysis/scenarios.json; cross-link unresolved investigation entries.
External descriptions are hypotheses until confirmed against this ROM and do not
independently justify structural classification. Distinguish sourced instructions
from tested input sequences and observed results, including version mismatches.
Prioritize research that changes an experiment or resolves an obstacle, rather than
plot summaries or exhaustive background collection. If sources or browsing are
unavailable, record the limitation and continue local analysis; lack of a guide
alone does not justify terminating the whole investigation.

Revisit research when it could resolve a specific obstacle. Before exhausting a
gameplay-dependent investigation path, check for documented controls, passwords,
shortcuts and progression routes that could make the experiment feasible. New
actionable findings reopen affected ledger entries under the completion contract.

## Completion and investigation contract

Objective: produce a byte-identical, structurally annotated disassembly accounting
for the released game's behavior and supporting assets. Preserve unexplained bytes
verbatim. Historical identification of unused material is not required. There is
no classification-percentage target; unknown bytes do not automatically prevent
completion. This contract supersedes older whole-ROM classification instructions
in historical progress records and previous prompts.

### Required behavioral accounting

Maintain a versioned inventory in analysis/behavior.md and its structured companion
analysis/behavior.json with stable item IDs for
modes, major mechanics, progression paths and terminal states; implementation entry
points, dispatch mechanisms and asset-loading systems; and unresolved questions
that could change reconstructed behavior. Derive it from local analysis, observed
execution and available game documentation, not only encountered gameplay. Record
sources, version applicability, implementation evidence, validation status,
dependencies and remaining questions per item. Update it when discoveries reveal
additional behavior. Preserve history and link experiments in investigation.md.
Static analysis and proven consumer/format rules may account for entire families;
runtime observation of every branch or asset is not required. Inventory coverage
is an engineering acceptance record, not a mathematical completeness proof.

### Unexplained regions

Track every unknown physical interval in analysis/investigation.md or linked
queue records. Triage static control flow, incoming references
including computed dispatch and banking, and known loaders, pointer tables,
formats and allocation boundaries. Record findings, limitations, applicable
methods and justified exclusions; reuse existing evidence rather than repeat scans.

Prefer consumer/decoder/selector analysis that
resolves whole families over visiting every asset through gameplay. Reserved fields
may belong to an established record allocation without proof each field is read;
record the allocation evidence and avoid extending boundaries by appearance alone.

An interval may remain unexplained at completion only when no unresolved behavioral
dependency points into it, no credible code or asset hypothesis remains unexamined,
and triage identifies no concrete follow-up capable of resolving a reconstruction
question. Record that disposition, evidence, limitations and reopen conditions.
Call it unexplained with no established behavioral dependency, not proved unused.
Lack of observed access alone never satisfies this rule. Do not relabel unknown
bytes as data or padding to improve counts. Reached code is not observed execution.

### Complete

Declare success only when all of these conditions hold:

1. Every behavior-inventory item has an evidence-backed implementation account.
2. Relevant control-flow and asset-reference uncertainties are resolved or bounded
   with evidence explaining why they cannot change that account.
3. Every remaining unknown interval satisfies the unexplained-region rule above.
4. Accepted regions have role/evidence annotations, internal and exported stock
   maps validate, and final generated assembly rebuilds byte-identically.
5. A final report records the inventory audit, validation coverage, verification,
   unexplained-region dispositions and material limitations with evidence links.

A checkpoint or raw-byte rebuild alone is not goal completion. Byte counts and
execution coverage are supporting diagnostics, not primary progress or acceptance
metrics. Cosmetic names, exhaustive instruction commentary and historical purpose
of unused material are not required. Native-port implementation and behavioral
tests belong to the later port contract, not this disassembly's success claim.

### Continue

Continue while a concrete unblocked investigation could resolve missing behavior,
an implementation dependency or a credible hypothesis. Before each experiment,
record its question, distinguishing evidence sought and expected consequence for
the behavioral account or map; afterward record its result and next dependency.
Prioritize behavioral relevance and family-level leverage, not bytes alone.
Runtime requires a named evidence target and replayable route; advancing through
the game is not itself sufficient justification. Repeated zero-relevance gameplay
must trigger strategy review. Bound calls, finish paginated queries and inspect
trace drops; operational bounds do not authorize stopping.

A retry requires changed prerequisites or a materially different hypothesis.
Repeated traces, cosmetic edits and unchanged hypotheses are zero progress.
Speculative possibilities alone do not justify endless retries. Reopen entries
when new evidence or capabilities change their dependencies. Check APIs and bounded
workarounds before claiming a tool blocker; preserve toolkit findings separately.

### Stop incomplete

Stop incomplete, blocked when a required condition is unmet and missing capability,
input or external dependency blocks every concrete investigation addressing it.
Stop incomplete, investigation exhausted when required conditions remain unmet,
applicable methods have recorded results, and no remaining distinguishing experiment
is identified. Mixed cases must identify which questions are blocked or exhausted.
Neither outcome is success. Elapsed effort, low yield and a raw-byte rebuild do not
independently justify either success or incomplete termination.

Before either incomplete outcome, audit the full behavior inventory, unresolved
dependencies and all unknown-region dispositions. Save state, export/validate the
map and produce a final verified checkpoint where possible. Report exact unmet
conditions and evidence or capabilities that would reopen them. Counts must still
cover the ROM exactly without overlap; preserve unsupported bytes as unknown.

### Resume and goal status

Resume from the saved inventory and investigation ledger. Reopen work affected by
new evidence or capabilities; do not automatically repeat exhausted experiments or
restart an unchanged incomplete run. On upgrade, create behavior.md if absent and
reassess existing unknown tickets under this contract; old progress statements do
not reinstate the superseded every-byte objective. Do not erase historical evidence.
Report complete or the explicit incomplete outcome. Never mark incomplete work as
successful. Provider status rules still apply: do not invent statuses, pause without
user authorization, or misuse complete to stop automatic continuation. If the
provider cannot represent the outcome yet, report that limitation and preserve the
handoff; do not manufacture new experiments to fill continuation turns.

## Toolkit issues

Keep a separate, persistent report in analysis/toolkit-issues.md. Read it when
starting or resuming; create it if absent. Record findings when encountered,
not just at completion, so interrupted sessions retain them. Update existing
entries when evidence changes and avoid duplicates; never overwrite earlier
findings on resume. Keep ROM classification evidence in analysis/progress.md.

For each finding, record:
- A stable ID and short title.
- Type: confirmed bug, suspected bug, or missing feature.
- Affected toolkit command/API/component and toolkit version or commit if known.
- Evidence: the concrete analysis task, expected versus actual behavior for
  bugs, reproduction commands/API parameters where available, and local links
  to relevant traces, logs or analysis artifacts. If not reproduced, explain
  the uncertainty and keep the bug suspected.
- Impact on analysis speed, inference quality or reliability of conclusions.
- Workaround, or explicitly state that none is known and whether progress is blocked.
- Suggested fix or improvement, without implementing it during disassembly.

Missing features must be grounded in a concrete example from this ROM analysis
and explain how the functionality would substantially improve reverse engineering
performance or inference. Exclude speculative wish lists and cosmetic preferences.
Check toolkit documentation before treating a limitation as a defect. Distinguish
documented limits from newly demonstrated bugs; a documented limit can still be
a missing feature if it meets the evidence and substantial-impact requirements.
Missing execution coverage or an unresolved ROM region alone is not a toolkit bug.

Continue the disassembly using bounded workarounds unless an issue blocks it.
If blocked, record the blocker and save available progress before reporting it.
At every pass completion or blocked handoff, summarize the report's findings by
type, identify blockers and link the report. If there are no findings, explicitly
say "no toolkit issues found"; this describes the pass, not proof of no defects.

To resume, a new serve process needs map.load and emulator.start, then replay the
recorded inputs; named snapshots from the old process are unavailable. Inspection
uses the ROM identity captured at initialization. run.py rejects changed input.
Generated files remain local. A verified raw-byte listing is a fidelity baseline,
not evidence that behavioral accounting is complete.
"""

START = """Read AGENTS.md and workflow.md. Establish a verified baseline, then
account for released-game behavior and supporting assets under workflow.md's
completion and investigation contract. Do not stop at the baseline: a checkpoint is
not goal completion. Follow Game familiarization; record sourced findings in
analysis/gameplay.md and confirm external claims against this ROM. Create the
versioned inventory in analysis/behavior.md from research, static analysis and
observations. Investigate semantics wherever useful and inspect consumer
families before choosing targeted runtime experiments. Keep unsupported bytes
unknown and preserve them verbatim. Save analysis/state.json, export maps/current.sym
and verify coherent passes. Maintain analysis/investigation.md, progress.md and
toolkit-issues.md. Continue actionable investigations; Stop incomplete only under
the documented blocked/exhausted rules, never as success. Report behavioral
accounting, unknown dispositions, validation and toolkit findings with links, or
say "no toolkit issues found" if empty. Do not fix the toolkit or download ROMs/assets.
"""

RESUME = """Read AGENTS.md, workflow.md, workspace.json and prompts/goal.md.
Resume behavioral reconstruction accounting under the current completion and
investigation contract, which supersedes older every-byte classification goals.
Read analysis/behavior.md (create it if absent), frontier.md, the latest progress
batch, investigation.md and toolkit-issues.md; retrieve older evidence as needed.
Restore analysis/state.json in a fresh session. Preserve evidence and review prior
unknown-region dispositions and dependencies. Follow Game familiarization and
preserve analysis/gameplay.md; confirm external claims against this ROM. Investigate
semantics wherever useful. Reopen entries only when new evidence or capabilities
justify it; do not repeat unchanged exhausted experiments. Prioritize behavior,
consumer/asset families over raw byte gains. Runtime needs
a named evidence target; replay only when needed and do not claim durable snapshots.
Save/export validated changes and verify coherent passes. A checkpoint is not goal
completion. Continue actionable investigations; Stop incomplete only under the
blocked/exhausted rules, never as success. Report behavioral accounting, unexplained
regions, verification and toolkit findings with links, or "no toolkit issues found".
Do not fix the toolkit or download ROMs/assets.
"""

WORKFLOW = WORKFLOW.replace("## Completion and investigation contract", EFFICIENCY + "\n## Completion and investigation contract")

SEMANTIC_HANDOFF = """
Use dependency-driven semantic experiments and preserve reusable names, invariants
and evidence for commentary. Reconcile map corrections without overwriting edited
source. Follow Efficient investigation and resumption, using analysis/work.py and
cached analysis/session.py. Operational bounds do not authorize premature stopping.
"""
START += SEMANTIC_HANDOFF
RESUME += SEMANTIC_HANDOFF

GOAL = """Read AGENTS.md, workflow.md and prompts/resume.md (or prompts/start.md
for a new workspace). Complete behavioral reconstruction accounting and a verified
structural disassembly under workflow.md's completion and investigation contract.
Success requires an evidence-backed behavior inventory, resolved or evidence-bounded
relevant control/asset dependencies, documented triage/dispositions for every
remaining unknown interval, annotated accepted regions, valid internal/stock maps
and a final byte-identical rebuild. No classification-percentage target applies.
Preserve unexplained bytes verbatim; lack of observed access does not prove nonuse.
Maintain analysis/behavior.md, investigation.md, progress.md, gameplay.md and
toolkit-issues.md. Prioritize behavioral dependencies and consumer families; runtime
requires a named evidence target. Use the release binary, cached immutable decoding,
ranked dependency queue and coherent verified passes. Continue concrete actionable
investigations. Operational bounds do not authorize stopping. Stop incomplete only
under the documented blocked/exhausted rules; never mark incomplete work complete
or restart unchanged exhausted work. Report inventory coverage, unknown dispositions,
validation, verification and toolkit findings with links (or no toolkit issues found).
Do not fix the toolkit, download ROMs/assets or commit generated files.
"""

BEHAVIOR = """# Behavior inventory

Version: 1. Not yet audited; no behavior is accounted for by initialization.

Follow workflow.md's Required behavioral accounting and ending contract. Use stable
IDs; record sources/version, modes/mechanics/progression/terminal states, implementation
entry points and loaders, evidence, validation status, dependencies and unresolved
questions. Update the version and preserve change history as the inventory evolves.

Keep behavior.json synchronized for handoff: version=1, revision, rom_sha256,
phase=disassembly, scope (status=reviewed, summary, evidence), items (id, summary,
status=accounted/pending, evidence, dependencies as item IDs, unresolved questions),
and outcome (status=pending/complete/incomplete_blocked/incomplete_exhausted, notes).
Evidence includes durable explanatory statements, not just links to scratch files.
All items must be accounted, unresolved=[] and scope reviewed before completion.
Later phases inherit IDs/questions and add their own source/implementation review.
"""

TOOLKIT_ISSUES = """# Toolkit issues

No findings recorded yet; ROM analysis has not started.

Preserve and update this report across passes. Follow workflow.md's Toolkit issues
section for evidence requirements and entry fields. Use one entry per finding with
a stable ID; distinguish confirmed bugs, suspected bugs, and missing features.
"""

RUNNER = '''#!/usr/bin/env python3
"""Local workspace commands; no shell expansion or agent-provider dependency."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["inspect", "baseline", "serve", "checkpoint"])
    parser.add_argument("name", nargs="?")
    parser.add_argument("--timeout-ms", type=int, default=120000)
    args = parser.parse_args()
    if not 1 <= args.timeout_ms <= 300000:
        parser.error("timeout-ms must be within 1..300000")
    if args.command == "checkpoint":
        if not args.name or not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_-]*", args.name):
            parser.error("checkpoint requires a simple new name, e.g. pass-001")
        if args.name == "baseline":
            parser.error("baseline is reserved")
    elif args.name:
        parser.error("only checkpoint accepts a name")
    meta = json.loads((ROOT / "workspace.json").read_text())
    rom = ROOT / meta["rom"]
    if hashlib.sha256(rom.read_bytes()).hexdigest() != meta["sha256"]:
        raise ValueError("input ROM changed; create a separate workspace for another image")
    binary = meta["binary"]
    timings = {}
    def call(*values, capture=False):
        started = time.monotonic()
        result = subprocess.run([binary, *map(str, values)], cwd=ROOT,
                                check=True, text=True, capture_output=capture)
        timings[" ".join(str(v) for v in values[:2] if v in ("map", "validate", "store", "generate", "verify"))] = time.monotonic() - started
        return json.loads(result.stdout) if capture else None
    if args.command == "inspect":
        call("inspect", rom)
        return
    if args.command == "serve":
        call("serve", "--rom", rom)
        return
    name = "baseline" if args.command == "baseline" else args.name
    output = ROOT / "generated" / name
    saved_map = ROOT / "maps" / (name + ".sym")
    report = ROOT / "analysis" / (name + "-verification.json")
    saved_state = ROOT / "analysis" / (name + "-state.json")
    state = ROOT / "analysis/state.json"
    if any(p.exists() or p.is_symlink() for p in [output, saved_map, report, saved_state]):
        raise ValueError("checkpoint paths already exist; choose a new name")
    state_hash = None
    if state.exists():
        call("store", "validate", rom, "--store", state)
        state_hash = hashlib.sha256(state.read_bytes()).hexdigest()
        shutil.copyfile(state, saved_state)
    if name == "baseline":
        call("map", "init", rom, "--output", saved_map)
    else:
        shutil.copyfile(ROOT / "maps/current.sym", saved_map)
    call("map", "validate", saved_map, "--rom", rom)
    call("generate", rom, "--map", saved_map, "--out", output, "--timeout-ms", args.timeout_ms)
    result = call("verify", rom, "--generated", output, "--timeout-ms", args.timeout_ms, capture=True)
    if result.get("ok") is not True or result.get("byte_identical") is not True:
        raise ValueError("verification did not confirm byte identity")
    if hashlib.sha256(rom.read_bytes()).hexdigest() != meta["sha256"]:
        raise ValueError("input ROM changed during checkpoint")
    if state_hash is not None and hashlib.sha256(state.read_bytes()).hexdigest() != state_hash:
        raise ValueError("state changed during checkpoint; verify a coherent new pass")
    result.update(rom_sha256=meta["sha256"],
                  map_sha256=hashlib.sha256(saved_map.read_bytes()).hexdigest(),
                  map=str(saved_map.relative_to(ROOT)),
                  generated=str(output.relative_to(ROOT)), state_sha256=state_hash,
                  binary_sha256=hashlib.sha256(Path(binary).read_bytes()).hexdigest(),
                  tool_timeout_ms=args.timeout_ms, timings_seconds=timings)
    report.write_text(json.dumps(result, indent=2) + "\\n")
    print(json.dumps(result))

if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(json.dumps({"ok": False, "error": str(error)}), file=sys.stderr)
        sys.exit(1)
'''


def initialize(source, destination, binary):
    source = source.resolve(strict=True)
    destination = destination.absolute()
    if destination.exists() or destination.is_symlink():
        raise ValueError(f"workspace already exists: {destination}")
    resolved = destination.resolve()
    if resolved == ROOT or ROOT in resolved.parents:
        raise ValueError("game workspaces must be outside the toolkit repository")
    if not source.is_file():
        raise ValueError(f"not a ROM file: {source}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix=".romatlas-", dir=destination.parent) as temp:
        work = Path(temp) / "workspace"
        work.mkdir()
        for directory in ["input", "analysis", "maps", "generated", "prompts", "traces"]:
            (work / directory).mkdir()
        rom = work / "input/game.rom"
        shutil.copyfile(source, rom)
        inspected = subprocess.run([str(binary), "inspect", str(rom)],
                                   check=True, capture_output=True, text=True)
        metadata = json.loads(inspected.stdout)
        identity = hashlib.sha256(rom.read_bytes()).hexdigest()
        files = {
            "AGENTS.md": AGENTS.format(toolkit=json.dumps(str(ROOT))),
            "workflow.md": WORKFLOW,
            "prompts/start.md": START,
            "prompts/resume.md": RESUME,
            "prompts/goal.md": GOAL,
            "analysis/progress.md": "# Disassembly progress\n\nWorkspace initialized. Baseline and classification have not run.\n",
            "analysis/scenarios.json": "[]\n",
            "analysis/behavior.md": BEHAVIOR,
            "analysis/behavior.json": json.dumps(behavior.new(identity, "disassembly"), indent=2) + "\n",
            "analysis/investigation.md": "# Investigation ledger\n\nNo regions reviewed yet. Follow workflow.md's completion and investigation contract.\n",
            "analysis/gameplay.md": "# Gameplay research\n\nNo research performed yet. Follow workflow.md's Game familiarization section. Record source URLs, version uncertainty, actionable routes and local confirmation separately.\n",
            "analysis/toolkit-issues.md": TOOLKIT_ISSUES,
            "run.py": RUNNER,
            "analysis/session.py": (ROOT / "scripts/disassembly_session.py").read_text(),
            "analysis/work.py": (ROOT / "scripts/disassembly_work.py").read_text(),
            "analysis/frontier.md": "No classification state yet. Establish the baseline, then run analysis/work.py refresh.\n",
            ".gitignore": "*\n",
            "workspace.json": json.dumps({"version": 1, "workflow_revision": 4, "toolkit": str(ROOT),
                                          "binary": str(binary), "rom": "input/game.rom",
                                          "sha256": identity, "inspection": metadata}, indent=2) + "\n",
        }
        for name, content in files.items():
            (work / name).write_text(content)
        # Refuse replacement if another initializer created the destination meanwhile.
        destination.mkdir()
        # Retain a partial workspace for inspection on failure; never overwrite it.
        for item in work.iterdir():
            shutil.move(str(item), destination / item.name)
    return {"ok": True, "workspace": str(destination), "sha256": identity,
            "start_prompt": str(destination / "prompts/start.md"),
            "resume_prompt": str(destination / "prompts/resume.md")}


def print_codex_instructions(record):
    workspace = shlex.quote(record["workspace"])
    print(f"""
Workspace ready: {record['workspace']}

1. Run this in your terminal:
   codex --enable goals -C {workspace}

2. Once Codex opens, paste this into its input and press Enter:
   /goal Read AGENTS.md, workflow.md and prompts/start.md. Follow prompts/goal.md and the behavioral completion and investigation contract in workflow.md. Continue beyond the raw-byte baseline; a checkpoint is not goal completion. Account for released-game behavior and dependencies, triage unknown regions, preserve unexplained bytes and verify byte identity. Stop incomplete only under the documented blocked/exhausted rules; never mark incomplete work complete. Preserve evidence and report the outcome with links. Do not fix the toolkit.

3. For a later session without an existing unfinished goal, open the same workspace and paste:
   /goal Read AGENTS.md, workflow.md and prompts/resume.md. Follow prompts/goal.md and the behavioral completion and investigation contract in workflow.md. Continue beyond the raw-byte baseline; a checkpoint is not goal completion. Account for released-game behavior and dependencies, triage unknown regions, preserve unexplained bytes and verify byte identity. Stop incomplete only under the documented blocked/exhausted rules; never mark incomplete work complete. Preserve evidence and report the outcome with links. Do not fix the toolkit.

Inside Codex, /goal shows status; /goal pause pauses and /goal resume resumes
an existing paused goal. These are Codex input commands, not shell commands.
If /goal is unavailable, use a Codex version with goal support.
""", file=sys.stderr, flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("roms", type=Path, nargs="+", help="local owned .gb/.gbc files")
    parser.add_argument("--root", type=Path, default=Path.cwd(),
                        help="parent for game workspaces (default: current directory)")
    parser.add_argument("--workspace", type=Path, help="exact directory; only for one ROM")
    parser.add_argument("--binary", type=Path, default=ROOT / "target/release/romatlas",
                        help="toolkit executable (default: release; build with cargo build --locked --release)")
    parser.add_argument("--json", action="store_true",
                        help="print JSON results only; suppress Codex startup instructions")
    args = parser.parse_args()
    if args.workspace and len(args.roms) != 1:
        parser.error("--workspace requires exactly one ROM")
    binary = args.binary.resolve(strict=True)
    destinations = [args.workspace or args.root / rom.stem for rom in args.roms]
    if len({str(p.resolve()) for p in destinations}) != len(destinations):
        parser.error("ROM filenames produce duplicate workspaces; use distinct names or --workspace")
    for source, destination in zip(args.roms, destinations):
        record = initialize(source, destination, binary)
        print(json.dumps(record), flush=True)
        if not args.json:
            print_codex_instructions(record)


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(json.dumps({"ok": False, "error": str(error)}), file=sys.stderr)
        sys.exit(1)
