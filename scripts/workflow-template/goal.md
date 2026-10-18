# Iterative disassembly, commentary and redirect

Read AGENTS.md and workflow.md. Act as a small coordinator: launch a separate
worker context for each phase, sequentially, with only its phase prompt, checkpoint
and compact handoff. Do not feed all phase prompts or the entire investigation
history into every worker. Workers retrieve relevant durable evidence as needed.
Use separate agents/processes when supported; otherwise provide a separate-session
handoff and report orchestration pending rather than silently doing all phases in
one growing context. Never run competing phase writers concurrently.
Run these single-pass workflows in order:
1. prompts/disassembly.txt
2. prompts/commentary.txt
3. prompts/redirect.txt

Persist the round number, current phase, input/output checkpoints, verification,
lead IDs consumed/produced and outcomes in analysis/progress.md. Keep coordinator
context to those summaries. Each worker leaves a compact phase handoff with scope,
checkpoint, verification, findings, unresolved questions and deferred work; link
artifacts instead of copying logs. Phase boundaries do not require new goal tools. Resume the recorded
phase after interruption; do not initialize over existing work. Each disassembly
pass drives normal gameplay scenarios while they yield substantial gains, alongside
static analysis and lead verification, and reports incomplete when these gains end.
Do not restrict disassembly to startup or basic inputs. Continue through commentary
and redirect even when disassembly or semantics remain incomplete. Commentary must complete current-source readability before handoff; reserve
targeted final acceptance checks and audit for final acceptance/archive. Preserve prior
commentary and lead history across rounds as described in the prompts.

After every disassembly pass, before starting commentary, explicitly print the ROM
classification completion percentage to the user; recording it only in a file is
insufficient. Follow the calculation and report format in prompts/disassembly.txt.
Include this percentage again in the final handoff.

Every redirect pass investigates promising unresolved questions thoroughly but
reasonably, following prompts/redirect.txt. Groundwork and intermediate understanding
are valid progress without an immediate substantial byte gain. There is no separate
investigation mode, one-time allowance or fixed hypothesis/experiment quota.
After redirect, repeat the sequence when new or materially improved ready leads
provide a concrete useful next step, even if the expected byte gain is small or
not yet measurable. A ready lead needs evidence, an actionable recipe and a credible
connection to resolving ROM structure or behavior; an unanswered question alone
is insufficient. Do not cycle on unchanged leads or repeat startup exploration
without new evidence or a materially different justified method.
If redirect has no useful actionable next step after reasonable investigation,
apply the final-investigation scheduling rule below before deciding to stop. If
no useful next step remains afterward, report incomplete with unknown counts, final checkpoint, commentary and
leads links and verification results. Workflow termination is not whole-ROM completion.
Resume interrupted research from recorded findings rather than repeating it. When
resuming an older easy-gain stop that has not used this redirect policy, run redirect
again from the existing checkpoint/commentary handoff before deciding to terminate.
When every byte is classified, address remaining pending review and known editorial
issues before claiming completion. Reuse valid review credit; do not require a
whole-source reread. Revisit reviewed material only for changed evidence or concrete
inconsistencies.
Declare complete only when every byte has evidence-backed classification, required
annotations, valid maps, final commentary audit and byte-identical verification.
Do not use the deprecated per-script /goal start/resume instructions. This file
is the single coordinator prompt; it does not require any particular goal tool.

Readable, fully commented and labeled assembly is a required final deliverable for
porting, separate from structural classification. Commentary must explain meaningful
routines, branches and data structures and use consistent variable/field/constant
symbols. Each round completes the current readability backlog; final acceptance
addresses remaining naming/comment gaps even in previously reviewed ranges. Unknown
meanings must have useful mechanical descriptions and explicit uncertainty.

Commentary handoff gate: the worker must finish readable explanations and consistent
symbols across ALL currently classified source, including older backlog. A small
repair or ready lead is not completion. Require a scope/readability assessment and
fresh byte-identical verification plus refreshed reading copy. If the worker leaves
unreviewed classified source or naming/comment gaps, resume that same phase with its
remaining-range handoff; do not advance to redirect. Context/time interruption is
not phase completion. Reuse sound prior work; unresolved meanings need mechanical
explanations and explicit uncertainty, not indefinite research or invented semantics.

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

## Final investigation before a near-complete stop

After redirect has no useful actionable next step, automatically schedule
prompts/final-investigation.txt before terminating when 0 < unknown bytes <= 16384
AND unknown bytes / total physical ROM bytes <= 0.05. Compute these values from
the current internal map, excluding unknown/raw fallback from classified bytes;
do not use rounded percentages. These are scheduling thresholds, not evidence
standards. Explicit user requests may run this prompt at any remaining size.
This is coordinator scheduling, not a background process launched by run.py.

Apply this rule on resume of an older stopped workspace too. Require completed
current-source commentary first. Track phase, checkpoint identity, attempt IDs,
methods and outcomes in analysis/progress.md and analysis/final-investigation/.
Resume an interrupted final investigation in that phase.

Final investigation is iterative, not a one-shot allowance. After accepted proposals
pass through disassembly, commentary and redirect, automatically return to final
investigation while the threshold still holds and unknown bytes remain. Use the
latest redirect handoff, previous final-investigation handoff and residual ledger
as the jumping-off point. Do not require the redirect to have already supplied a
ready structural lead: the next investigation begins by designing concrete methods
against its recorded barriers. If a pass produces useful intermediate evidence or
a concrete improved experiment but no classification proposal, continue final
investigation directly without an empty disassembly/commentary cycle.

Before substantive retries, compare the proposed method and prerequisites with all
previous attempts. A targeted whole-table extent proof, ending source/bank/copy-length
trace or graphics-container boundary proof may replace a stalled live-selector search
when justified by the residual evidence. Develop such alternatives proactively;
a handoff's missing-fact list is a research starting point, not proof of exhaustion.
Do not automatically repeat an exhausted method on unchanged evidence. A new round,
checkpoint, restated question or renamed method alone is insufficient. Every repeat
must identify new evidence or a materially different concrete method and explain
which recorded barrier it can resolve and what observation would distinguish outcomes.

Continue passes while they produce supported proposals, useful new constraints or
credible untried methods addressing residual structure. No minimum byte gain or
fixed pass quota applies. When method planning finds no credible untried alternative,
or results repeat without useful learning or changed prerequisites, stop and report
incomplete with exact unknown ranges, exhausted methods and missing facts. Do not
relaunch planning on that unchanged terminal handoff; resume requires new evidence,
a materially improved concrete method or an explicit user request to reassess.
If no unknown bytes remain, proceed to final editorial/archive acceptance instead.
Preserve all evidence, fidelity and completion gates; iteration never authorizes
speculative classification. This is coordinator scheduling, not a background job.

## Structural evidence and reachability

Structural classification and reachability are separate claims. Require evidence
for the claimed structural role, exact boundaries and effective bank context;
require reachability evidence only when claiming reachability. A live caller,
observed instruction fetch, naturally produced selector or ordinary gameplay route
is useful evidence, not a universal prerequisite for classifying code or data.
Apply this distinction from the first disassembly pass, not only during final
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
