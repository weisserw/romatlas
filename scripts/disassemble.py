#!/usr/bin/env python3
"""Prepare isolated, local agent workspaces for owned Game Boy ROMs."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]

AGENTS = """# ROM disassembly workspace

Read evidence-policy.md, workflow.md, workspace.json, analysis/progress.md, and
analysis/toolkit-issues.md before work.
Read and maintain analysis/gameplay.md for sourced gameplay familiarity and
targeted experiment routes, following workflow.md's Game familiarization section.
The toolkit repository is {toolkit}. Read its project.md, docs/progress.md,
docs/protocol.md and docs/sym.md. Use its existing binary; this workspace is
for disassembling this game, not changing the toolkit.
New workspaces default to target/release/romatlas. Rebuild the configured release
binary with `cargo build --locked --release` after toolkit changes; restart live
servers to load a rebuilt binary. Do not silently substitute another build.

Objective: classify ROM regions with evidence, export a portable stock .sym map,
and generate a bank-organized assembly listing that rebuilds byte-for-byte.
Investigate structure and semantics together; name and explain routines where
useful. Required annotations describe structural role and supporting evidence;
instruction-by-instruction paraphrases are unnecessary. The commentary phase must
produce fully commented, labeled, readable source for porting, including variables
and branches; structural classification alone does not satisfy final acceptance.
Do not stop after a raw-byte baseline: fidelity and classification completeness
are separate results. Report unknown bytes and unresolved bank contexts honestly.

Use python3 run.py inspect/baseline/serve/checkpoint as described in workflow.md.
Keep input/game.rom unchanged. Never download ROMs. Keep input, internal state,
traces, source and extracted assets local; only stock .sym is distributable.
Never commit the entire workspace. .gitignore is a guard, not authorization.

Use bounded execution, discovery and paged queries. Trace addresses are physical;
retain CPU aliases and effective bank context. Prefer analysis.discover_context.
Observed instruction fetches prove execution; reached targets and pointer values
do not. Unexecuted bytes may be classified using static or semantic evidence;
unsupported bytes stay unknown. Export's raw-byte gaps are not data evidence.
Structural roles/bounds and reachability are separate claims. A live caller or
ordinary gameplay route is not required when independent static evidence establishes
the role, exact bounds and bank context. Record reachability uncertainty separately.
Inspect candidates before map.annotate. Split existing regions before inserting
smaller regions. Validate maps after edits. Save internal state with map.save
to analysis/state.json, and export stock .sym to maps/current.sym.

Record evidence, confidence, scenario input sequences, remaining unknown ranges,
and exact tested verification results in analysis/progress.md after each batch.
Snapshots are session-local; save replayable input sequences for later sessions.
Use snapshot export/import for persistence; do not assume named snapshots persist. Do not invent unsupported snapshot persistence or claim full hardware compatibility.

Maintain analysis/toolkit-issues.md as you encounter toolkit bugs or missing
features, following workflow.md's reporting requirements. Preserve findings
across sessions. Continue ROM analysis using workarounds unless blocked; do not
fix the toolkit in this workspace. At the end of every pass, summarize toolkit
findings and link the report, or explicitly say "no toolkit issues found" if empty.
"""

WORKFLOW = """# Per-game workflow

Read goal.md and evidence-policy.md. The coordinator gives each separate phase worker only its own prompt
and compact checkpoint/handoff; workers retrieve relevant evidence as needed. The old provider-specific
/goal startup/resume workflow is deprecated. Each disassembly pass stops at easy-gain
exhaustion; the coordinator runs commentary and redirect. Every redirect investigates
promising questions thoroughly but reasonably before deciding whether to continue.

1. Run `python3 run.py inspect`, then `python3 run.py baseline` once. Baseline
   creates maps/baseline.sym and generated/baseline and verifies the rebuild.
   It marks raw bytes for export only; it is not a classified disassembly.
   Use targeted Game familiarization for an easy gain or a promising redirect investigation.
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
8. Follow the easy-gain stopping rule below. End this single pass with classified
   code/data/unknown byte counts from the internal store, uncertainty, execution
   scenarios exercised, verification evidence, and concrete next tasks.
   Include a summary of toolkit findings and a link to analysis/toolkit-issues.md;
   explicitly say "no toolkit issues found" when the report has no findings.

## Game familiarization

When needed for an easy gain or redirect investigation, consult available online manuals, walkthroughs, FAQs, gameplay
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
alone does not prevent taking available local easy gains.

## Structural evidence and reachability

Structural classification and reachability are separate claims. Require evidence
for the claimed structural role, exact boundaries and effective bank context;
require reachability evidence only when claiming reachability. A live caller,
observed instruction fetch, naturally produced selector or ordinary gameplay route
is useful evidence, not a universal prerequisite for classifying code or data.
Apply this distinction from the first disassembly pass and throughout redirect
investigation. Independently supported routine contracts, control/data flow and
bounded record/table formats can justify static classification while natural use
remains unresolved. Record structural evidence and reachability status separately;
do not label statically inferred bytes Observed or claim they are unused/dead
merely because no caller is known.

Caller/selector tracing remains useful for resolving roles, bounds and bank context.
When selected indices are the only evidence for a table's extent, retain unsupported
entries as unknown until an independent extent proof exists. Valid decoding, pointer
values, adjacent records, alignment and pattern resemblance alone do not establish
structure. Artificial execution can test consumer mechanics but does not establish
intended code, complete table extent or natural reachability by itself. Keep these
evidence safeguards and all classification, fidelity and editorial completion gates.

## Completion and investigation contract

This is one easy-gain pass, not an exhaustive investigation. Read prompts/disassembly.txt.
Easy gains include substantial discoveries from static analysis, debugger-driven normal
gameplay scenarios, and bounded verification of specific leads in analysis/leads.json.
Continue ordinary gameplay while it yields substantial evidence or usable understanding;
do not restrict it to startup/basic inputs or require a preexisting lead. Record routes
and gains, try obvious productive alternatives and stop repetitive play when gains diminish.
End the disassembly pass when these opportunities are exhausted; every redirect
then investigates promising questions thoroughly but reasonably, as specified in
prompts/redirect.txt. Groundwork and intermediate understanding may justify useful
next steps without premeasured substantial byte gains. Avoid rabbit holes, slim
chances and repeated stalled work. No separate investigation mode or fixed research
quota applies. No per-range exhaustion proof is required; the full classification
contract remains unchanged.

Connected-investigation handoff: redirect should follow productive handlers, state
transitions and related data before returning a coherent supported batch. A first
ready lead or the end of its initial range does not alone justify another cycle.
Require reasons for deferring obvious continuations and resume interrupted productive
redirect work in that phase. Disassembly may use evidence-backed static code and
saved experiments; do not require every branch to execute. Preserve uncertainty,
classification/fidelity gates and avoid byte quotas or exhaustive recursive closure.


Redirect phase completion requires more than closure of its first selected graph.
If its handoff lists other concrete promising actionable avenues, continue redirect
with those avenues and accumulate findings before one disassembly/commentary cycle.
Do not accept 'independent investigation' or 'outside selected scope' as a deferral
reason by itself. Require an actual missing prerequisite, negative evidence or
reduced prospect. Preserve research across context rollover in the same phase;
no byte quota, exhaustive unknown sweep or weakening of evidence standards applies.

Maintain analysis/investigation.md with attempted leads, outcomes and remaining unknowns.
Do not repeat unchanged failed leads. Research gameplay for an easy gain or a promising redirect question;
do not require manuals, late-game routes or exhaustive method reviews before stopping.

Complete still means every physical ROM byte has an evidence-backed structural role,
required annotations, valid internal/stock maps and byte-identical generated assembly.
Raw emission never proves classification. At either outcome save analysis/state.json,
export maps/current.sym, validate, and make a new verified checkpoint when possible.
Report complete or incomplete, per-bank code/data/unknown counts, annotation coverage,
execution coverage separately, checkpoint links, untested hypotheses and toolkit findings.
Never declare the ROM complete just because a pass or byte-identical rebuild succeeded.

## Continuous investigation in redirect

Investigation is ordinary redirect work at every classification percentage. There
is no percentage threshold, separate research phase or fixed number of passes.
Continue while supported findings, useful new constraints or a justified concrete
next test provide a credible prospect of progress. A ready redirect lead is not
required to begin designing methods against residual barriers.

Use the latest commentary/redirect handoffs, exact unknown complement and prior
accepted/rejected outcomes. Preserve historical investigation records, including
legacy analysis/final-investigation/ attempts, by reference; never overwrite them.
On resume, continue any unfinished legacy investigation as redirect from its saved
frontier, without replaying completed experiments or creating a new baseline.
Keep new research in unique analysis/redirect/<attempt-id>/ directories and record
phase, checkpoint identity, method changes and outcomes in analysis/progress.md.
Completed attempt records remain immutable; interruptions resume the same research.

When proposals are ready after productive connected and independent avenues have
been investigated, hand the combined batch to disassembly, then full current-source
commentary and redirect. If useful intermediate evidence or a credible improved
method exists without a classification proposal, continue redirect directly,
without an empty disassembly/commentary cycle. Context rollover and a completed
attempt are checkpoints, not reasons to stop useful research. No minimum byte gain
or fixed pass quota applies.

Apply evidence-policy.md before inheriting a rejection or accepting terminal
exhaustion. Corrected premises, narrower claims and independently justified models
can reopen reasoning on unchanged ROM evidence; a new experiment is not required.
Do not automatically repeat an exhausted method: explain the changed inference or
prerequisite and expected discriminating result. A new round or renamed method alone
is insufficient. A missing-fact list is a starting point, not proof of exhaustion.

Before accepting a terminal incomplete handoff, inspect the worker's compact
rejection review and its decisive primary artifacts under evidence-policy.md.
Check the scope of counterexamples, inferred prerequisites and consistency with
accepted evidence standards; verifying hashes or counting previous attempts does
not assess the stopping rationale. If the review is missing or exposes a concrete
corrected inference/test, return that question to redirect yourself. Do not wait for a user
to propose it. Reuse a sound review on unchanged premises; when it supports exhaustion,
stop incomplete rather than cycling automatically. Do not relaunch planning on an
unchanged sound terminal assessment without new evidence, a justified corrected
inference/method or explicit user reassessment. Report exact residuals, checkpoints,
readability and verification separately. Workflow termination is not ROM completion.
If every byte becomes classified, final editorial audit and byte-identical verification
still precede completion. Preparation writes instructions, not a background process.


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
not evidence that classification is complete.
"""

TEMPLATES = ROOT / "scripts/workflow-template"

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

ROOT = Path(__file__).resolve().parent

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["inspect", "baseline", "serve", "checkpoint"])
    parser.add_argument("name", nargs="?")
    args = parser.parse_args()
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
    def call(*values, capture=False):
        result = subprocess.run([binary, *map(str, values)], cwd=ROOT,
                                check=True, text=True, capture_output=capture)
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
    if any(p.exists() or p.is_symlink() for p in [output, saved_map, report]):
        raise ValueError("checkpoint paths already exist; choose a new name")
    if name == "baseline":
        call("map", "init", rom, "--output", saved_map)
    else:
        shutil.copyfile(ROOT / "maps/current.sym", saved_map)
    call("map", "validate", saved_map, "--rom", rom)
    call("generate", rom, "--map", saved_map, "--out", output)
    result = call("verify", rom, "--generated", output, capture=True)
    if result.get("ok") is not True or result.get("byte_identical") is not True:
        raise ValueError("verification did not confirm byte identity")
    result.update(rom_sha256=meta["sha256"],
                  map_sha256=hashlib.sha256(saved_map.read_bytes()).hexdigest(),
                  map=str(saved_map.relative_to(ROOT)),
                  generated=str(output.relative_to(ROOT)))
    report.write_text(json.dumps(result, indent=2) + "\\n")
    print(json.dumps(result))

if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(json.dumps({"ok": False, "error": str(error)}), file=sys.stderr)
        sys.exit(1)
'''


def initialize(source, destination, binary, platform="auto"):
    source = source.resolve(strict=True)
    is_gg = platform == "gg" or platform == "auto" and source.suffix.lower() == ".gg"
    is_sms = platform == "sms" or platform == "auto" and source.suffix.lower() == ".sms"
    is_nes = platform == "nes" or platform == "auto" and source.suffix.lower() == ".nes"
    is_snes = platform == "snes" or platform == "auto" and source.suffix.lower() in {".sfc", ".smc"}
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
        rom = work / ("input/game.sfc" if is_snes else "input/game.nes" if is_nes else "input/game.gg" if is_gg else "input/game.sms" if is_sms else "input/game.rom")
        shutil.copyfile(source, rom)
        inspected = subprocess.run([str(binary), "--platform", platform, "inspect", str(rom)],
                                   check=True, capture_output=True, text=True)
        metadata = json.loads(inspected.stdout)
        if metadata.get("platform") == "gg" and not is_gg:
            # A Sega header can identify GG even when a file uses the SMS suffix.
            renamed = work / "input/game.gg"
            rom.rename(renamed)
            rom = renamed
            is_gg, is_sms = True, False
        identity = hashlib.sha256(rom.read_bytes()).hexdigest()
        files = {
            "AGENTS.md": AGENTS.format(toolkit=json.dumps(str(ROOT))),
            "workflow.md": WORKFLOW,
            "evidence-policy.md": (TEMPLATES / "evidence-policy.md").read_text(),
            "prompts/disassembly.txt": (TEMPLATES / "disassembly.txt").read_text(),
            "prompts/commentary.txt": (TEMPLATES / "commentary.txt").read_text(),
            "prompts/redirect.txt": (TEMPLATES / "redirect.txt").read_text(),
            "goal.md": (TEMPLATES / "goal.md").read_text(),
            "analysis/leads.json": json.dumps({"version": 1, "passes": [], "leads": []}, indent=2) + "\n",
            "analysis/progress.md": "# Disassembly progress\n\nWorkspace initialized. Baseline and classification have not run.\n",
            "analysis/scenarios.json": "[]\n",
            "analysis/investigation.md": "# Investigation ledger\n\nNo regions reviewed yet. Follow workflow.md's completion and investigation contract.\n",
            "analysis/gameplay.md": "# Gameplay research\n\nNo research performed yet. Follow workflow.md's Game familiarization section. Record source URLs, version uncertainty, actionable routes and local confirmation separately.\n",
            "analysis/toolkit-issues.md": TOOLKIT_ISSUES,
            "run.py": RUNNER,
            ".gitignore": "*\n",
            "workspace.json": json.dumps({"version": 1, "toolkit": str(ROOT),
                                          "binary": str(binary), "rom": "input/game.rom",
                                          "sha256": identity, "inspection": metadata}, indent=2) + "\n",
        }
        if is_snes:
            import snes_workflow
            files = snes_workflow.files(files, metadata)
        if is_gg:
            import gg_workflow
            files = gg_workflow.files(files, metadata)
        if is_sms:
            import sms_workflow
            files = sms_workflow.files(files, metadata)
        if is_nes:
            import nes_workflow
            files = nes_workflow.files(files, metadata)
        for name, content in files.items():
            (work / name).write_text(content)
        # Refuse replacement if another initializer created the destination meanwhile.
        destination.mkdir()
        # Retain a partial workspace for inspection on failure; never overwrite it.
        for item in work.iterdir():
            shutil.move(str(item), destination / item.name)
    return {"ok": True, "workspace": str(destination), "sha256": identity,
            "goal_prompt": str(destination / "goal.md"),
            "prompts": [str(destination / "prompts" / name) for name in
                        ["disassembly.txt", "commentary.txt", "redirect.txt"]]}


def print_prompt_location(record):
    print(f"Workspace ready: {record['workspace']}\nRead {record['goal_prompt']}",
          file=sys.stderr, flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("roms", type=Path, nargs="+", help="local owned ROM files")
    parser.add_argument("--platform", choices=["auto", "gb", "snes", "nes", "sms", "gg"], default="auto")
    parser.add_argument("--root", type=Path, default=Path.cwd(),
                        help="parent for game workspaces (default: current directory)")
    parser.add_argument("--workspace", type=Path, help="exact directory; only for one ROM")
    parser.add_argument("--binary", type=Path, default=ROOT / "target/release/romatlas")
    parser.add_argument("--json", action="store_true",
                        help="print JSON results only; suppress prompt locations")
    args = parser.parse_args()
    if args.workspace and len(args.roms) != 1:
        parser.error("--workspace requires exactly one ROM")
    binary = args.binary.resolve(strict=True)
    destinations = [args.workspace or args.root / rom.stem for rom in args.roms]
    if len({str(p.resolve()) for p in destinations}) != len(destinations):
        parser.error("ROM filenames produce duplicate workspaces; use distinct names or --workspace")
    for source, destination in zip(args.roms, destinations):
        record = initialize(source, destination, binary, args.platform)
        print(json.dumps(record), flush=True)
        if not args.json:
            print_prompt_location(record)


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(json.dumps({"ok": False, "error": str(error)}), file=sys.stderr)
        sys.exit(1)
