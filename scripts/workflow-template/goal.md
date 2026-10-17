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
and redirect even when disassembly or semantics remain incomplete. Intermediate commentary is incremental and may hand off pending review; reserve
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
stop and report incomplete with unknown counts, final checkpoint, commentary and
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
