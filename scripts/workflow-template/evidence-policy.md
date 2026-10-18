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

An inferred bounded table or fill region is acceptable when the combined evidence
supports its role, exact extent and bank context. Pattern or successful rebuilding
alone remains insufficient. Use Static provenance, explain the inference and its
limits, and leave unsupported competing interpretations unknown. Confidence scores
do not substitute for the evidence. Independent disassembly review may reject it.

## Localize uncertainty and counterexamples

Split a rejected claim into independently testable subclaims when its evidence has
different scopes. An unresolved prefix does not automatically reject a later
uniform bank-tail candidate; evaluate its boundary and fill role separately without
assuming a trailer or treating uniformity alone as proof. A failed full-table or
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
Semantic consistency still needs independent role and extent evidence.

For example, a failed reconstruction of an exact waveform generator does not alone
reject a consumer-anchored, bounded quantized waveform table. A long uniform suffix
after an unresolved tag is a separate fill hypothesis, not an automatic fill claim.
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
