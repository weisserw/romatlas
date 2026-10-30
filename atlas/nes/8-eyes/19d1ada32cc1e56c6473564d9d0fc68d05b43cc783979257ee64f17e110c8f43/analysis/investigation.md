# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round01 progress
Baseline and round01-initial/gameplay/data checkpoints verified. Startup/title,
attract demo and real menu stage routes yield accepted fetch contexts (capacity0,
no aggregate drops; ring drops expected). NES-001 explains ignored early inputs.
Physical banks0–7 Data classification is structural (grids/records/arrays), with
consumer/schema semantics deferred; see round01/data-inspection.md. Long FF spans
in higher banks classified independently. Static inferred code distinguished from
Observed; no natural reachability claim for unfetched closure or artificial routes.
Natural A-cycling1/4/7 reaches distinct stage interiors. $0050=7 at stage selector
before Start reaches a distinct Ruth final-stage exterior with falcon and enemies.
$0050 stage selector is supported by $B3DD etc and reset/init reads; $0051 room
zeroed at $B419 and tested against7 at $EEAC. Next practical experiment: break at
$EE00 after stage selection, patch $51=7 and resume initializer using natural call
stack. This should expose boss rooms without raw arbitrary-state brute force.

## Round 01 single disassembly pass — finished, ROM incomplete
ROM classification: 86.96% (227971 / 262160 bytes classified; 34189 bytes unknown).

11629 Code, 85270 Data, 131072 Image bytes; 508 evidence-bearing regions.
Classified bytes have structural annotations; unknown complement remains explicit
in [round01/unknown-ranges.json](round01/unknown-ranges.json). Commentary/readable
naming remains a separate next phase: only three semantic region labels exist.
4293 distinct fetched physical ROM opcode offsets retained across saved scenario
contexts (lower-bound execution metric, separate from classified operand bytes).
Trace detail capacity0 intentionally drops ring events; all examined summary
aggregate_dropped=0. Accepted fetches include attract, player-input and artificial
scenarios, with no natural reachability claims for artificial paths.

Final checkpoint [round01-disassembly verification](round01-disassembly-verification.json):
map validation and ca65 rebuild passed, byte_identical=true. Frozen map
maps/round01-disassembly.json; generated/round01-disassembly/game.asm; current
analysis/state.json and maps/current.json agree. Input ROM/toolkit unchanged.

| Section | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 0 | 8192 | 0 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 0 | 8192 | 0 |
| PRG 03 | 0 | 8192 | 0 |
| PRG 04 | 0 | 8192 | 0 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 0 | 8192 | 0 |
| PRG 08 | 1995 | 517 | 5680 |
| PRG 09 | 0 | 1128 | 7064 |
| PRG 10 | 0 | 3616 | 4576 |
| PRG 11 | 0 | 6254 | 1938 |
| PRG 12 | 0 | 6498 | 1694 |
| PRG 13 | 2499 | 0 | 5693 |
| PRG 14 | 2101 | 0 | 6091 |
| PRG 15 | 5034 | 1705 | 1453 |
| CHR | 0 | 131072 | 0 |

Broad gameplay visited natural distinct stages and artificial Ruth/later-room
layouts plus altar puzzle; no boss defeat or ending/credits claim. Exact routes,
patches, contexts, operation/frame budgets and outcomes: scenarios.json and
round01 scripts. Validator success at D582 is supported; terminal transition
remains unresolved after bounded test. Stop is ordinary first-pass easy-gain
handoff, not terminal research exhaustion: call-return bank contracts, remaining
code/data boundaries and ending transition need coherent redirect investigation.
No initial leads existed; R01 lead outcomes now persisted in leads.json.

Toolkit findings: one confirmed documentation/API bug NES-001, zero suspected
bugs, zero missing features, no blockers; see [toolkit-issues.md](toolkit-issues.md).
Disk inspected at handoff: approximately 11GiB free, round01 evidence101MiB before
final small reports. No cleanup/deletion/archive/publication performed.

Primary handoff: [round01-disassembly-handoff.md](round01-disassembly-handoff.md).

## Round02 disassembly — reviewed redirect batch accepted

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).

Rounded100.00% is not completion:5unknown bytes remain. Single pass independently accepted34184new bytes:8164Code(3646instructions),25576Data,32inline pointer bytes,412selected dispatch/index bytes. Final totals19793Code/111290Data/131072CHR, 881regions with evidence. See [independent review](round02-disassembly/review.md), [mechanical review](round02-disassembly/mechanical-review.json), [metrics](round02-disassembly/metrics.json), [exact residual](round02-disassembly/unknown-ranges.json). No proposal rejected; specific-schema claims narrowed to generic Data where full layouts remain unproven. Conditional return summaries and mode3 structural contract do not assert natural reachability or universal bank preservation.

| Section | Code | Data | CHR image | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 0 | 0 |
| PRG 00 | 0 | 8192 | 0 | 0 |
| PRG 01 | 0 | 8192 | 0 | 0 |
| PRG 02 | 0 | 8192 | 0 | 0 |
| PRG 03 | 0 | 8192 | 0 | 0 |
| PRG 04 | 0 | 8192 | 0 | 0 |
| PRG 05 | 0 | 8192 | 0 | 0 |
| PRG 06 | 0 | 8192 | 0 | 0 |
| PRG 07 | 0 | 8192 | 0 | 0 |
| PRG 08 | 1995 | 6197 | 0 | 0 |
| PRG 09 | 0 | 8192 | 0 | 0 |
| PRG 10 | 0 | 8192 | 0 | 0 |
| PRG 11 | 0 | 8192 | 0 | 0 |
| PRG 12 | 0 | 8192 | 0 | 0 |
| PRG 13 | 3966 | 4226 | 0 | 0 |
| PRG 14 | 7885 | 306 | 0 | 1 |
| PRG 15 | 5947 | 2241 | 0 | 4 |
| CHR | 0 | 0 | 131072 | 0 |

Data intermediate checkpoint maps/round02-data-intermediate.json:253991classified/8169unknown. Final state/map agree; frozen maps/round02-disassembly-verified.json and generated/round02-disassembly-verified/game.asm passed fresh validation and byte-identical ca65 rebuilding, report [verification](round02-disassembly-verified-verification.json). Prior commentary sourceSHA d0e1fb0cdc7d722a499e4863e8b347c616d5eeb35b13e7c2836ced65d3dcf102 preserved; expanded current-source commentary pending separate phase, final editorial gate pending.

Leads R01-CALLS/ROOMS/ENDING/DATA now verified for accepted structural slices, histories retained in leads.json. Scenario checklist reviewed: representative stages/later rooms and artificial congratulations/terminal password/Start-return already visited; natural bosses/alternate endings remain unverified. No new promising gameplay recipe targets the five residual bytes; no unchanged route replay. Retained contextual fetch-file union4499unique physical opcode offsets (includes attract/natural/artificial, not execution-byte coverage);171 newly unknown-at-input roots are separately saved in redirect. No runtime coverage claims inferred from Static annotations.

Residuals: physical116286 bank14 CPUC62E byte60, and123710..123713 bank15 CPUE32E A900855F. Literal/entry alternatives remain materially ambiguous; no raw-valid-decode shortcut. Ordinary disassembly handoff, not terminal exhaustion; commentary then redirect reviews accepted boundaries/remaining subclaims. No new toolkit issues found; persistent [toolkit report](toolkit-issues.md) retains1confirmedNES-001,0suspected,0missing features,no blockers. Input/toolkit untouched; no archive/cleanup/deletion/publication.
