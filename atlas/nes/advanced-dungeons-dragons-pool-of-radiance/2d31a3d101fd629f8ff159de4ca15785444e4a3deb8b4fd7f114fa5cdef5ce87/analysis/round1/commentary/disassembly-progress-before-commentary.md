# Disassembly progress

Round1 single easy-gain disassembly pass finished incomplete. Next: full current-source commentary, then redirect; this is not terminal investigation exhaustion. See [compact handoff](round1/handoff.md), [scenario checklist](scenarios.json), [investigation ledger](investigation.md), and [leads](leads.json).

ROM classification: 58.35% (382424 / 655376 bytes classified; 272952 bytes unknown).

Structural totals: Code22,128; Data140,607; Image131,072; Text88,617. All5,299 regions have evidence. Exact residual complement [round1/unknown-ranges.json](round1/unknown-ranges.json). Raw fallback does not count.

Fresh checkpoint: [verification](pass-001-verification.json), `maps/pass-001.json`, `generated/pass-001`; byte-identical true and ROM identity unchanged. Internal map validated and saved to state.json, exported to maps/current.json. Baseline and pass-001a/b/c preserved.

Natural gameplay reached prebuilt party, full Phlan tour, character stats/equipment view, city movement, slums encounter, tactical battle and Goldleaf Sleep cast. Later areas, major boss and ending deferred pending coherent stage/initialization contract; bounded isolated $6D patch failed and was restored. Exact inputs, execution budgets, effective banks, artificial-state distinction and durable snapshots recorded in scenario/RPC artifacts. Snapshot names are historical: title.snapshot.json is copyright screen, not main menu.

Execution coverage separately:9,223 unique natural ROM instruction starts;18,950 unique instruction bytes (3.6144% PRG);20,429 contextual roots. All saved natural fetched bytes overlap Code (0 conflicts). Capture-disabled waits are excluded; artificial area patch excluded; enabled aggregates had0 overflow. Capacity0 detail drops are expected.

Current static successor/direct-call closures exhausted without unresolved budgets or competing boundaries; final new batches259/41 bytes. Additional ordinary discovery needs fresh roots, callee effects or dispatch contracts. Repeated battle idle sample0 and spell-resolution256 bytes indicate diminished gains in current representative scenario. Broader gameplay is not globally exhausted.

Full current-source commentary has not run; readability remains a separate gate. No complete compatibility or original-layout claim.

Toolkit: no toolkit issues found; [report](toolkit-issues.md),0 confirmed bugs/0 suspected/0 missing features, no toolkit blockers. A file-bridge process received SIGTERM without toolkit error; durable saved classifications survived. Self-contained bounded batches and snapshots avoided recurrence. This environment interruption is not evidence exhaustion.

## Per-bank structural counts

ROM classification: 58.35% (382424 / 655376 bytes classified; 272952 bytes unknown).

| Physical section/bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 339 | 2217 | 5636 |
| PRG 01 | 775 | 2507 | 4910 |
| PRG 02 | 0 | 2145 | 6047 |
| PRG 03 | 0 | 2373 | 5819 |
| PRG 04 | 0 | 2990 | 5202 |
| PRG 05 | 0 | 3847 | 4345 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 0 | 8192 | 0 |
| PRG 08 | 0 | 3459 | 4733 |
| PRG 09 | 0 | 4839 | 3353 |
| PRG 10 | 0 | 3827 | 4365 |
| PRG 11 | 0 | 4130 | 4062 |
| PRG 12 | 0 | 1064 | 7128 |
| PRG 13 | 0 | 1741 | 6451 |
| PRG 14 | 0 | 3466 | 4726 |
| PRG 15 | 0 | 3658 | 4534 |
| PRG 16 | 0 | 8192 | 0 |
| PRG 17 | 0 | 5120 | 3072 |
| PRG 18 | 0 | 1192 | 7000 |
| PRG 19 | 0 | 0 | 8192 |
| PRG 20 | 0 | 4141 | 4051 |
| PRG 21 | 0 | 5341 | 2851 |
| PRG 22 | 0 | 3424 | 4768 |
| PRG 23 | 0 | 3980 | 4212 |
| PRG 24 | 0 | 0 | 8192 |
| PRG 25 | 0 | 8192 | 0 |
| PRG 26 | 0 | 0 | 8192 |
| PRG 27 | 0 | 1710 | 6482 |
| PRG 28 | 0 | 4018 | 4174 |
| PRG 29 | 0 | 4408 | 3784 |
| PRG 30 | 0 | 3529 | 4663 |
| PRG 31 | 0 | 4180 | 4012 |
| PRG 32 | 0 | 3101 | 5091 |
| PRG 33 | 0 | 5325 | 2867 |
| PRG 34 | 0 | 3761 | 4431 |
| PRG 35 | 0 | 5893 | 2299 |
| PRG 36 | 0 | 3406 | 4786 |
| PRG 37 | 0 | 4419 | 3773 |
| PRG 38 | 0 | 3261 | 4931 |
| PRG 39 | 0 | 5072 | 3120 |
| PRG 40 | 0 | 2736 | 5456 |
| PRG 41 | 0 | 3888 | 4304 |
| PRG 42 | 0 | 4320 | 3872 |
| PRG 43 | 0 | 4458 | 3734 |
| PRG 44 | 0 | 8192 | 0 |
| PRG 45 | 0 | 8192 | 0 |
| PRG 46 | 0 | 8192 | 0 |
| PRG 47 | 0 | 8192 | 0 |
| PRG 48 | 0 | 8192 | 0 |
| PRG 49 | 71 | 4027 | 4094 |
| PRG 50 | 528 | 351 | 7313 |
| PRG 51 | 0 | 7635 | 557 |
| PRG 52 | 2679 | 551 | 4962 |
| PRG 53 | 0 | 1478 | 6714 |
| PRG 54 | 0 | 0 | 8192 |
| PRG 55 | 859 | 667 | 6666 |
| PRG 56 | 1358 | 510 | 6324 |
| PRG 57 | 334 | 1727 | 6131 |
| PRG 58 | 2421 | 1212 | 4559 |
| PRG 59 | 2597 | 300 | 5295 |
| PRG 60 | 1162 | 3593 | 3437 |
| PRG 61 | 99 | 176 | 7917 |
| PRG 62 | 4605 | 0 | 3587 |
| PRG 63 | 4301 | 307 | 3584 |
| CHR | 0 | 131072 | 0 |
