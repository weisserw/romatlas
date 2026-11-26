# Round001 disassembly handoff

Scope: exactly one disassembly pass from initialized workspace, baseline established and verified. ROM remains incomplete.

ROM classification: 89.56% (117404 / 131088 bytes classified; 13684 bytes unknown).

Checkpoint: analysis/state.json; maps/current.json; frozen maps/pass-001.json; generated/pass-001; analysis/pass-001-verification.json. Internal and portable validation and byte-identical reconstruction pass.12980code,104424data,13684unknown; per-bank counts and exact162interval complement in analysis/evidence/round001/classification-counts.json and progress.md. Observed annotations9695bytes lower-bound; static role/reachability kept separate.

Evidence: round001 discovery-candidates/references/diagnostics, full data inspection and16 suspicious-decode reviews. Followed directJSR entry contracts without assuming mapper-safe returns. Generic body boundaries deliberately exclude mixed prefixes and common FFxx tails. No overlap conflicts.

Gameplay: natural title/village display; artificial airport/river/runway/desert/temple/final5-4 scenery and illustrated ending. Exact inputs/patches/budgets in analysis/scenarios.json and retained batch scripts; durable snapshots/captures in analysis/evidence/round001. D2=level1..5,D3=scene1..4; A456 computes08=2*(4*D2-5+D3). Initial scene0 tests are artificial previous-scene displays, not valid natural visits. RAM08-only patch rejected as overwritten. Boss and active combat initiation remain unresolved despite PC817C/SP255 attempt and300frame delay. EndingPC8196/SP255,D2=5,D3=4,08=38 is visually confirmed, artificial only. Highscore terminal deferred.

Leads: R001-stage-direct08 rejected; R001-ending verified with reachability limits; progression-wait, BRK dispatch, mixed prefixes/tails ready for redirect. No supplied prior leads. Important missing fact: why normal Start/scene initialization remains in A493 NMI wait, and dependent actor state needed for boss entry. Do not repeat unchanged long inputs. Broader static data and dispatch research is credible; this is not terminal exhaustion.

Next: full current-source commentary, then redirect connected investigations above. Semantic annotations/readability and final editorial audit are not claimed complete.

Toolkit: no toolkit issues found; analysis/toolkit-issues.md. Session exit143/sandbox socket restriction worked around using bounded self-contained subprocess sessions and snapshot persistence. No toolkit changes, cleanup/deletion, publishing or archival. Disk checked; about3.2GiB free.

Final consistency:5018unique actual ROM fetch starts; zero overlap with classified Data (fetch-data-conflict-check.json). Final retained evidence83MiB/scratch132KiB; no pruning. Final static call closure completed, not interrupted; no pending phase work beyond listed redirect questions.
