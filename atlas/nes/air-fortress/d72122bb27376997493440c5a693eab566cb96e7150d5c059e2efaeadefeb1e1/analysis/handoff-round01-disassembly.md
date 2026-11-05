# Round 01 disassembly handoff

Outcome: substantive easy-gain pass complete; ROM incomplete.

ROM classification: 41.45% (67911 / 163856 bytes classified; 95945 bytes unknown).

9950 code bytes, 23281 generic/structured data bytes, 1912 Text bytes, 32768 Image bytes. All 663 regions carry structural evidence; code views preserve physical identity and effective banks. Semantic labels/readability remain commentary work. Exact unknown complement and per-bank counts: analysis/round01-counts.json.

| Physical section | Code | Data/text/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2450 | 1972 | 3770 |
| PRG 01 | 799 | 3350 | 4043 |
| PRG 02 | 0 | 0 | 8192 |
| PRG 03 | 0 | 1448 | 6744 |
| PRG 04 | 912 | 0 | 7280 |
| PRG 05 | 935 | 1631 | 5626 |
| PRG 06 | 0 | 0 | 8192 |
| PRG 07 | 0 | 631 | 7561 |
| PRG 08 | 0 | 0 | 8192 |
| PRG 09 | 0 | 274 | 7918 |
| PRG 10 | 0 | 319 | 7873 |
| PRG 11 | 0 | 3584 | 4608 |
| PRG 12 | 0 | 2821 | 5371 |
| PRG 13 | 0 | 8181 | 11 |
| PRG 14 | 4854 | 0 | 3338 |
| PRG 15 | 0 | 966 | 7226 |
| CHR | 0 | 32768 | 0 |

Execution is separate: 6033 unique physical opcode starts; 8097 contextual roots, 6669 physical edges, aggregate_dropped=0. Detail capacity0 intentionally drops detail; request/reply evidence bounded at ~31MiB, no raw instruction stream. Replay/snapshot restoration means aggregate frame/cycle history is not one monotonic natural playthrough. Observed bytes have fetched-root provenance; connected static instructions remain Static.

Verified final checkpoint: maps/round01-disassembly.json, generated/round01-disassembly, analysis/round01-disassembly-verification.json. map.validate succeeded; generation and fresh assembly verification byte_identical=true. Map SHA256 84eaf5c077665aeb5d3090b27778b7c129630df53d8b5ae41b114074e19c1397. Input SHA256 unchanged d72122bb27376997493440c5a693eab566cb96e7150d5c059e2efaeadefeb1e1. Current internal state analysis/state.json and maps/current.json saved. Baseline and initial checkpoint preserved.

Lead outcomes: R01-L1 password table verified+annotated; R01-L2 C8 stage premise superseded by consumer proof ($45 index,$C7 quest). R01-L3..L6 pending redirect: room data/bounds, reactor+escape state, credit route, static call/bank contracts. No prior ready leads existed. Middle fortress/boss/core progression deferred with missing contracts in scenarios.json; representative natural stage1 shooter, artificial final-stage shooter/interior and both ending branches visited. Final congratulations remained stable after Start+600frames; final20frames produced no new code. No repeated long waits warranted. Detailed room/bank semantics require connected redirect research, not another unchanged gameplay sequence. This is a phase handoff, not terminal investigation exhaustion.

Toolkit findings: one confirmed documentation/API mismatch NES-001 (shared input button/pressed silently ignored on NES), zero suspected issues, zero missing-feature entries; no blockers after buttons-array workaround. See analysis/toolkit-issues.md. No toolkit edits, archive, deletion or publishing.

Final live frontier: artificial congratulations scene, frame2204, cycles65633956, CPU PC$D3CD mid-instruction, effective banks[0,1,14,15]; durable snapshot analysis/round01-final.snapshot. New server: map.load current state, emulator.start, snapshot.import with path, reassert buttons[], enable desired tracing. Never assume named snapshots persist. Source restoration/readability audit belongs to next commentary phase.

Primary compact next-phase inputs: analysis/leads.json, analysis/scenarios.json, analysis/round01-counts.json, maps/round01-disassembly.json and generated/round01-disassembly. Historical primary evidence: analysis/round01-rpc.jsonl (all candidate pages/decodes/state edits), round01-candidates-*.json, uniform/text evidence lists and progression snapshots. The mutable round01-candidates.json is latest discovery only; historical earlier candidates remain in immutable request/reply log and checkpoint map.

Useful source facts: inline argument readers at $D4BF/$D4B9 mean raw linear decoding across call operands is unsafe; constrained discovery stops at unproved calls and observed returns skip their inline data. Preserve this barrier in commentary/redirect. No raw export gap counted. Uniform FF bank13 excludes 11-byte nonuniform trailer; no blanket fill-purpose claim.
