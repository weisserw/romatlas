# Round 001 disassembly handoff

ROM classification: 29.93% (39239 / 131088 bytes classified; 91849 bytes unknown).

Incomplete: 8,178 code /31,061 data. Final internal state analysis/state.json; portable local map maps/current.json. Exact per-bank counts in round-001/counts.json and progress.md. Final byte-identical checkpoint: maps/pass-001-final.json, generated/pass-001-final, analysis/pass-001-final-verification.json.

Scope: reset/NMI contextual static discovery; natural title/start/running/action; attempted world4/world8 shortcuts; artificial ending-like SQUARE animation via EE8/PC8043. Preserve routes/checklist in scenarios.json and gameplay.md. No natural ending, complete boss coverage or credits claim. No original leads; new verified/blocked/ready outcomes in leads.json.

Evidence: discovery-large*.json/gameplay*.json, 92-root direct-call callee closure (+986bytes), explicit durable3352 ROM-root closure (+839bytes), uniform-runs.json (+31,039genericdata). Classification role and natural reachability kept separate. Remaining91,849bytes require investigation; graphics candidates are not automatically classified.

Next phase: full current-source commentary. Redirect: (1) establish world transition state and supported stage/bonus/boss init, (2) inspect nonuniform graphic/VRAM copy boundaries, (3) prove RAM upload origin/extent before uploaded views, (4) derive return bank contracts for remaining static continuation. This handoff is easy-gain scoped, not terminal exhaustion.

Toolkit report: one confirmed bug NES-001, no blocking issue; analysis/toolkit-issues.md. Workaround already applied; toolkit unchanged. Baseline/pass checkpoints exact. Current-source readability/editorial gates incomplete. No archive, publish, cleanup or ROM edits.
