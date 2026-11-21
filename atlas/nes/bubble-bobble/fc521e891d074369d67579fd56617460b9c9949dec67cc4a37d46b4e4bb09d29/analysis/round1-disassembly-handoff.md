# Round 1 disassembly handoff — incomplete

ROM classification: 81.25% (133132 / 163856 bytes classified; 30724 bytes unknown).

Exact calculation: 100 × 133132 / 163856 = 81.249389708036%. Unique internal-map
intervals count once; no raw export gaps count. Code 8970, generic/formatted
Data 91394, Image 32768; header 16 and CHR 32768 are fully
classified. Provenance bytes: {'Observed': 7486, 'Static': 125646}. 4283 evidence-annotated regions.

| PRG 8 KiB bank | Code | Data | Unknown |
| --- | ---: | ---: | ---: |
| 0 | 0 | 8192 | 0 |
| 1 | 0 | 8192 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 646 | 1805 | 5741 |
| 9 | 0 | 8192 | 0 |
| 10 | 832 | 822 | 6538 |
| 11 | 0 | 8192 | 0 |
| 12 | 1068 | 0 | 7124 |
| 13 | 152 | 6744 | 1296 |
| 14 | 4529 | 0 | 3663 |
| 15 | 1743 | 87 | 6362 |

Physical execution coverage (separate metric): 3600 unique accepted opcode
starts across 6536 contextual roots in retained capture batches; this is
not byte classification and includes artificial-state observations. Trace aggregates
reported no drops; capacity-zero detail intentionally discards events while preserving
bounded roots/edges. Initial 180-frame startup hit timeout; other budgets/stops are
recorded in rpc-log.jsonl and per-batch output logs. No compatibility claim follows.

## Verification and scope

Latest state analysis/state.json and supported portable map maps/current.json.
Fresh checkpoint maps/pass-003.json, generated/pass-003/game.asm and
analysis/pass-003-verification.json validate and rebuild byte-identically. Input SHA256
fc521e891d074369d67579fd56617460b9c9949dec67cc4a37d46b4e4bb09d29 unchanged.
Earlier baseline/pass-001/pass-002 and evidence preserved. Only current.json is the
ROM-free sharing deliverable; state, source, logs, screenshots and snapshots stay local.
No publication/archive/toolkit modification performed. No live server remains.

Accepted structural evidence: validated header/vector/CHR extents; actual bank-aware
opcode fetch boundaries; conservative branch/fallthrough traversal; direct JSR target
contracts expanded through a finite closure without assuming post-return banks;
repeated packed data bodies and exact uniform FF intervals. See data-inspection.md,
observed-decodes.json, static-candidates.json, call-closure.json,
confirmed-call-closure.json and rpc-log.jsonl. Direct JSR closure exhausted its
supported connected frontier (last generation zero gains). No pointer-only code claims.
Unknown code/data boundaries and heterogeneous executable-bank gaps remain unresolved.

## Gameplay and lead outcomes

Consumed initial leads: none. Natural title/menu/story and Round01 visited with
movement, jump and bubble firing. Exact inputs, releases, budgets and snapshots are
in analysis/scenarios.json, driver.py and next.py. Initial single-button API trials
were attract-only and are explicitly excluded as gameplay routes.

Stage-only $0401 edits30/80/111/112 were tested and rejected as complete level-init
recipes: different enemies with stale Round01 geometry/invalid records. They establish
selector mechanics only. A materially different menu route patches highest/current
round ($049C,$0401), selects 1P Continue, and confirms Start twice; final confirmation
successfully reaches a giant final-boss arena with player/projectiles and different
bank10 code. confirmedB4.ppm and confirmedB4.snapshot preserve actual artificial
boss gameplay. B4 raw114 is a tested local menu value, not inferred from written RAM
alone. Right/B30 and A/B30 exercised the boss state. The earlier B2 black screen and
B4 selected-menu captures do not independently count as gameplay visits.

Ending/credits not visited. Progression to ending now needs boss HP/defeat and item/
player-state contracts; arbitrary RAM/PC forcing was avoided when no sound entry
contract existed. Ending lead is concrete and useful for redirect; no terminal
exhaustion is claimed. Broad best-effort exploration attempted middle and later
milestones and reached a distinct final boss; no exhaustive rare-interaction search.

Produced leads are in analysis/leads.json: boss defeat/ending, middle-stage Continue
recipe, remaining heterogeneous-bank structure, and tile/record consumers. Ready
static call/vector and generic-data batches were consumed in this pass rather than
handing off only their initial ranges. Further semantic/source commentary and
redirect must investigate these remaining contracts. Whole-ROM completeness and
current-source readability/editorial completion remain unmet and separate.

Toolkit findings: one confirmed documentation/API validation bug NES-001, no blockers.
See analysis/toolkit-issues.md. Workaround uses complete NES buttons arrays.
Disk checked: about3.5 GiB free; round1 evidence about93 MiB, scratch under1 MiB.
No cited evidence or workspace files deleted.
