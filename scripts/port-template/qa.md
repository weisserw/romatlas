# Stage 5: short smoke test and exact-frame comparisons

Execute or resume when the pipeline invokes QA: Read debug/goal.md.
Use python3 run.py debug for the native transport.

Read DEBUGGING.md for transport/tool usage, debug/acceptance.json for this stage's
scope, and the current implementation handoff. This pipeline uses only the short
initial sanity suite; the standalone guide's two whole-game passes do not apply.
Work headlessly in the port workspace. Preserve existing evidence and unrelated work.
Do not publish, commit or launch other agents.

Plan and execute one bounded smoke test of startup and a short representative
interaction, plus the initial original-fidelity comparisons for rendering,
scrolling, animation and audio. Use exact frame comparisons at selected meaningful
frames against the original with documented deterministic inputs, starting state
and frame alignment. Review selected captures and investigate mismatches; record
source-backed reasons for intentional differences. Do not turn this suite into a
whole-game traversal, broad progression checklist or second QA pass.

Use reusable scripts and validated setup/checkpoints. Keep comparisons in a separate
original-fidelity-sanity manifest with explicit enabled/reason configuration and
run the harness with --include-original-fidelity-sanity. Record expected and observed
results, capture paths and review judgments. Never learn expectations from port
output or silently disable failing comparisons. Existing reviewed equivalent initial
sanity evidence may be reused when its build/assets and scope remain valid.

Record the bounded scope in debug/plan.json using stable checks with reproducible
setup, expectations, applicable aspects and coverage links. Mark the plan ready.
Record one successful execution of that plan in debug/qa.json (passes number 1),
including build identity and local evidence per check, plan SHA-256 and a report.
Preserve debug/qa-policy.json and its separate sanity manifest/result record.
Fix failures in place and rerun failed/affected checks; preserve unaffected results.
A blocked or unreviewed check remains incomplete with an exact resumption point.

After repairs run relevant pinned Rust 1.90.0 locked checks, python3 run.py check,
python3 run.py audit and the toolkit's scripts/port_debug.py audit WORKSPACE.
Completion requires the short smoke/comparison suite, resolved blockers, required
capture reviews and both administrative audits. Report tested scope and limitations;
this does not establish whole-game QA coverage. Stop when these checks pass.
No full-game QA passes or automatic modernization follow this stage.
