"""Portable behavioral obligations shared by disassembly, commentary and porting.

These checks enforce accounting shape and inheritance, not semantic correctness.
Evidence fields contain durable explanatory statements, not scratch-file links alone.
"""
import copy
import json


GUIDE = """
## Inherited behavioral completion

No byte-classification percentage determines completion. behavior.json is the
machine-checked companion to the narrative behavioral inventory. Preserve inherited
item IDs and unresolved obligations; add newly discovered modes, mechanics,
progression, terminal states, implementation and asset dependencies. Derive scope
from game documentation plus static/runtime evidence, not only visited gameplay.
For each item record id, summary, status, evidence, dependencies (other item IDs),
unresolved questions and phase-specific source/equivalents/tests. Evidence must
include durable explanatory statements; scratch links alone are insufficient.
For inherited unresolved questions, add resolutions records with the original
question, resolution and durable evidence; silently clearing questions is insufficient.
Set scope.status=reviewed with scope.summary/evidence after the inventory audit.
Set outcome.status=complete with notes only at actual phase completion. An inherited
accounted item still needs this phase's review/implementation, so preparation resets
its status to pending while preserving evidence and unresolved questions.

Continue concrete investigations that can distinguish behaviorally relevant
hypotheses. Runtime needs a named evidence target; advancing gameplay or labeling
more bytes is not itself sufficient. Use static interpretation, coherent-code
review and consumer/format families without demanding execution of every branch.
Later phases may discover/refine code, data and extents. Preserve frozen originals;
record old/new physical ranges, bank context, evidence and affected obligations in
refinements.md. Reopen affected behavior/dispositions, reconcile editable source
and mappings, validate and byte-verify structural corrections. Never regenerate
over edited source or change frozen references. A proposed correction is not applied
until reconciled; unresolved reconciliation blocks completion. A native port keeps
new analysis/maps and optional assembly in analysis/ outside its frozen reference;
use normal ROMAtlas APIs for maps and verify any generated assembly against the ROM.

Opaque bytes can be preserved without semantic explanation only after triage finds
no unresolved behavioral dependency, no unexamined credible code/asset hypothesis,
and no concrete follow-up resolving a reconstruction question. Record durable
rationale/evidence and reopen conditions. Nonexecution alone is insufficient; do
not call opaque regions proved unused. Review inherited dispositions against source
and new findings rather than repeating unchanged exhausted investigations. Known
routines require semantic review even when their exact game meaning is tentative.

Complete requires reviewed behavioral scope, every inherited and discovered item
accounted for, relevant control/asset uncertainties resolved or evidence-bounded,
all physical regions treated, reconciled refinements and the phase's validation.
Each phase adds its own obligations: commentary needs useful source explanations,
a native port needs implemented equivalents and useful passing behavioral tests.
Physical byte coverage proves preservation, not behavioral understanding.

If required conditions remain unmet, stop incomplete_blocked only when every concrete
investigation addressing them is blocked by a recorded dependency; stop
incomplete_exhausted only when applicable methods have recorded results and no
remaining distinguishing experiment is identified. Audit scope, dependencies and
all opaque dispositions before either handoff. Record unmet conditions, attempts
and reopen triggers in outcome.notes and progress. These are report outcomes, not
new provider-tool statuses. Never mark incomplete work complete or pause without
user authorization; respect provider status rules and report any limitation.
Resume saved work and reopen changed dependencies; do not restart unchanged
exhausted experiments. Low yield, effort and raw-byte fidelity alone justify neither
success nor incomplete termination. Audits check records, not semantic truth.
"""


def new(identity, phase):
    return dict(version=1, revision=1, rom_sha256=identity, phase=phase,
                scope=dict(status="pending", summary="", evidence=[]), items=[],
                outcome=dict(status="pending", notes=""))


def load(path, identity, phase):
    if not path.exists():
        return new(identity, phase)
    result = json.loads(path.read_text())
    if not isinstance(result, dict) or result.get("version") != 1 or result.get("rom_sha256") != identity or result.get("phase") != phase:
        raise ValueError("behavior inventory identity/version mismatch")
    if not isinstance(result.get("items"), list):
        raise ValueError("behavior inventory requires items")
    if not isinstance(result.get("scope"), dict) or not isinstance(result.get("outcome"), dict) or any(not isinstance(item, dict) for item in result["items"]):
        raise ValueError("invalid behavior inventory structure")
    ids = [item.get("id") for item in result["items"]]
    if any(not isinstance(i, str) or not i.strip() for i in ids) or len(set(ids)) != len(ids):
        raise ValueError("behavior inventory IDs must be unique nonempty strings")
    return result


def inherit(original, phase):
    result = copy.deepcopy(original)
    result["phase"] = phase
    result["scope"]["status"] = "pending"
    result["outcome"] = dict(status="pending", notes="")
    for item in result["items"]:
        item.update(status="pending", source=[], equivalents=[], tests=[])
    return result


def nonempty(value):
    return isinstance(value, str) and bool(value.strip())


def strings(value):
    return isinstance(value, list) and bool(value) and all(isinstance(v, str) and v.strip() for v in value)


def validate(current, original, phase, source_check=None):
    if current.get("rom_sha256") != original["rom_sha256"] or current.get("version") != 1 or current.get("phase") != phase:
        raise ValueError("behavior inventory identity/phase changed")
    if type(current.get("revision")) is not int or current["revision"] < 1:
        raise ValueError("behavior inventory revision is required")
    scope = current.get("scope", {})
    if scope.get("status") != "reviewed" or not nonempty(scope.get("summary")) or not strings(scope.get("evidence")):
        raise ValueError("behavioral scope audit is pending")
    items = current.get("items", [])
    if not isinstance(items, list) or not items:
        raise ValueError("behavior inventory is empty; recover scope before completion")
    if any(not isinstance(i, dict) for i in items):
        raise ValueError("invalid behavior item")
    identities = [i.get("id") for i in items]
    if any(not isinstance(i, str) or not i.strip() for i in identities) or len(set(identities)) != len(identities):
        raise ValueError("invalid/duplicate behavior ID")
    if not {i["id"] for i in original["items"]}.issubset(identities):
        raise ValueError("inherited behavioral obligations were removed")
    inherited_by_id = {item["id"]: item for item in original["items"]}
    for item in items:
        if item.get("status") != "accounted" or not nonempty(item.get("summary")) or not strings(item.get("evidence")):
            raise ValueError(f"unaccounted behavior: {item['id']}")
        if item.get("unresolved") != []:
            raise ValueError(f"unresolved behavioral questions: {item['id']}")
        if not isinstance(item.get("dependencies"), list) or any(d not in identities for d in item["dependencies"]):
            raise ValueError(f"unresolved behavior dependency: {item['id']}")
        inherited_questions = inherited_by_id.get(item["id"], {}).get("unresolved", [])
        resolutions = item.get("resolutions", [])
        if not isinstance(resolutions, list) or any(not isinstance(r, dict) for r in resolutions):
            raise ValueError("invalid behavioral resolution records")
        for question in inherited_questions:
            resolved = [r for r in resolutions if r.get("question") == question]
            if len(resolved) != 1 or not nonempty(resolved[0].get("resolution")) or not strings(resolved[0].get("evidence")):
                raise ValueError(f"inherited question lacks resolution evidence: {item['id']}")
        if phase == "commentary":
            if not strings(item.get("source")):
                raise ValueError(f"behavior lacks reviewed source: {item['id']}")
            for location in item["source"]:
                source_check(location)
        if phase == "port" and (not strings(item.get("equivalents")) or not strings(item.get("tests"))):
            raise ValueError(f"behavior lacks port equivalents/test evidence: {item['id']}")
    outcome = current.get("outcome", {})
    if outcome.get("status") != "complete" or not nonempty(outcome.get("notes")):
        raise ValueError("behavioral completion is pending")
    return len(items)


def preservation(record):
    if not isinstance(record, dict) or not nonempty(record.get("rationale")) or not strings(record.get("evidence")) or not nonempty(record.get("reopen_when")):
        raise ValueError("preservation requires durable rationale, evidence and reopen condition")


def refinements(phase):
    return dict(version=1, phase=phase, entries=[])


def validate_refinements(current, original=None):
    entries = current.get("entries")
    if current.get("version") != 1 or not isinstance(entries, list):
        raise ValueError("invalid refinements inventory")
    if any(not isinstance(r, dict) for r in entries):
        raise ValueError("invalid refinement record")
    ids = [r.get("id") for r in entries]
    if any(not isinstance(i, str) or not i.strip() for i in ids) or len(set(ids)) != len(ids):
        raise ValueError("invalid/duplicate refinement ID")
    if original and not {r['id'] for r in original['entries']}.issubset(ids):
        raise ValueError("inherited refinements were removed")
    for entry in entries:
        if entry.get("status") != "reconciled" or not nonempty(entry.get("summary")) or not strings(entry.get("evidence")):
            raise ValueError("outstanding structural refinement/reconciliation")
    return len(entries)
