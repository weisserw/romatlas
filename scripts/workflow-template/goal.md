# Iterative disassembly, commentary and redirect

Read evidence-policy.md, AGENTS.md and workflow.md. Act as a small coordinator: launch a separate
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
Useful intermediate evidence or a justified concrete next method continues redirect
directly, even without a ready classification lead. Before a terminal stop, apply
the continuous-investigation and rejection-review rules below. If no useful next
step remains afterward, report incomplete with unknown counts, final checkpoint,
commentary and leads links and verification results. Workflow termination is not whole-ROM completion.
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
