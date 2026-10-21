# Structural inference and rejection review

Read this policy before proposing, rejecting or declaring exhaustion of a structural
lead. It applies throughout disassembly and redirect investigation and to the
coordinator's stopping decision. Preserve evidence-backed whole-ROM classification,
exact bank-aware boundaries, readable source and byte-identical rebuilding.

## Evaluate the claim actually needed

State the physical interval, structural role, relevant processor/bank context and
the supporting observations separately from inferred premises. Distinguish a role
claim from actual entry, natural selection, original source layout or the original
toolchain's generation algorithm. Establish only the claims needed for the proposed
annotation; record other uncertainties without making them extra prerequisites.

Combine independent constraints: consumer semantics, control/stack contracts,
encoded origins, independently established endpoints, finite record domains and
complete semantic consistency can jointly establish structure. Check whether these
constraints discriminate the proposed role and exact bounds from concrete competing
interpretations. Several restatements of one byte pattern are not independent facts.
An encoded count, live caller, observed read, serializer declaration or historical
linker proof is required only when the particular inference depends on it. Name
that dependency instead of making any one evidence source a universal requirement.
For a static routine, distinguish a supported calling precondition from a claim
that an actual caller supplies it; legal decoding alone still does not prove code.

Before falling back to generic data, make a practical attempt to identify its role:
inspect available references and consumers, nearby code and labels, and obvious
formats or repeated record layouts. Use supported specific region types, meaningful
labels and concise format/role explanations when the evidence permits. Generic data
is a fallback after this bounded inspection, not the default shortcut. Record briefly
what was checked and what remains unresolved; reuse prior sound inspection rather
than repeating it. Follow concrete promising leads, but do not require exhaustive
semantic research or block classification when further investigation is unproductive.
Never invent a specific meaning or label just to avoid the generic fallback.

Classify code versus data before trying to recover data semantics. Generic unlabeled
data is a sufficient structural role and counts toward whole-ROM completion. Clear
repeated byte patterns, uniform runs or visibly non-instruction content can support
that decision after boundary inspection and a check for conflicting code evidence.
Record exact physical ranges, a brief rationale and Static provenance with map.annotate.
No known consumer, semantic label, table schema, fill purpose or original generator
is required. Unknown meaning is not unknown classification and must not cause a
blocked handoff. Lack of execution or successful rebuilding alone is insufficient.
An inferred bounded table or fill region needs stronger evidence only to claim that
specific role; if that inference fails, retain generic data where non-code is clear.
Explain the inference and its limits; retain unknown where code versus data remains
materially ambiguous. Confidence scores
do not substitute for the evidence. Independent disassembly review may reject it.

## Localize uncertainty and counterexamples

Split a rejected claim into independently testable subclaims when its evidence has
different scopes. An unresolved prefix does not automatically reject a later
uniform bank-tail candidate; evaluate its boundary and fill role separately without
assuming a trailer. Uniformity may justify generic data without proving a fill role.
A failed full-table or
whole-gap proposal does not reject every supported subinterval. Conversely, a
supported subinterval does not establish the rest of the object.

Test the semantic representation needed by the consumer. Exact reproduction of an
unknown original generator is a separate, stronger claim. For quantized values,
derive permissible error, saturation and field width from the representation and
known consumers before assessing missing cells. Report the actual discrepancies
and whether they occur in already classified or unknown cells. Compare the proposed
criterion consistently across both. An exact rounding test may falsify that rounding
algorithm without falsifying a bounded quantized table. Do not fit arbitrary error
bands or piecewise exceptions to make unknown cells pass; if a revised model lacks
independent justification or fails relevant constraints, keep the claim unresolved.
A specific semantic interpretation still needs independent role and extent evidence;
generic data does not require that interpretation.

For example, a failed reconstruction of an exact waveform generator does not alone
reject a consumer-anchored, bounded quantized waveform table. A long uniform suffix
after an unresolved tag may be generic data even when its specific fill role remains
unproven.
An unreferenced return opcode or plausible pointer sequence still lacks sufficient
structure on its own. Apply the same reasoning to unfamiliar representations.

## Revisit reasoning without repeating experiments

History records observations and decisions, not irrevocable vetoes. Before reusing
a rejection, retrieve its decisive test or source evidence and identify exactly
which claim it falsified. Separate a measured counterexample from an untested
alternative, a missing fact and a worker's sufficiency judgment. Propagate negative
results only within that scope. Missing one proof route does not disprove a role
supported by another combination of constraints.

A corrected premise, narrower interval, justified semantic model or newly combined
independent evidence is a materially different method even with unchanged ROM and
traces. Explain the old error or changed inference and its discriminating check;
new runtime evidence and user intervention are not prerequisites. Preserve the old
record and append the correction. Renaming the same test or loosening its threshold
without justification is an unchanged retry and should stop.

## Challenge a terminal decision

Before terminal exhaustion, examine the strongest remaining candidates and the
premises behind their rejections against primary evidence. Focus on suspicious
requirements, broad negative-result scope and conflicting acceptance criteria;
this is a reasoning review, not another whole-ROM scan or gameplay replay.
Record a compact rejection review with links: exact candidate claim; decisive
evidence; what it actually rules out; independent constraints considered together;
strongest competing interpretation; and a supported proposal, a concrete revised
test, or the specific remaining reason the inference is insufficient. A list of
missing facts or prior method names does not perform this review.

The coordinator must inspect this reasoning before accepting a terminal handoff;
the worker's statement that no credible method remains is not enough. Correct an
overbroad rejection or unsupported prerequisite autonomously and return a concrete
question to the appropriate phase. Ordinary easy-gain handoffs need not complete
this terminal review. Reuse a sound completed review while its premises are
unchanged; do not create an endless review loop. If the critique exposes no useful
revised inference or test, stop incomplete with exact residuals and honest limits.
Neither the review nor pressure to progress authorizes speculative classification.

## Disk and evidence retention

Keep durable evidence separate from disposable scratch. Retain maps, readable
source, current state, plan/result history, concise observations, reproducible
routes and the selected captures/checkpoints actually cited by those records.
Preserving superseded attempts means retaining their plans/results and explanation;
it does not require retaining every intermediate dump or duplicate capture.

Use one reusable scratch directory for exploratory output. Before each batch and
at handoff, inspect free disk space and scratch size; remove superseded scratch
files after recording useful conclusions and promoting cited evidence. Never prune
cited evidence, source, ROMs, frozen references, saves or active-process output.
Save selected frames/audio intervals rather than continuous captures. Keep bulk
pixel/sample arrays and checkpoint payloads out of transport logs; explicitly save
needed captures/checkpoints once and link them. Bound trace/log bytes and stderr,
not just execution time. Reuse build targets; avoid a fresh Cargo target directory
for each probe and remove owned temporary test workspaces when finished.

When disk is tight, compact disposable output before another experiment. If safe
cleanup is insufficient, leave an incomplete resumable handoff rather than filling
the filesystem. This is storage hygiene, not a weaker acceptance criterion or a
limit on required game coverage. Do not delete another task's files.
