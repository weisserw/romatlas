# Coordinator terminal handoff

The coordinator workflow has reached its documented **terminal incomplete** handoff. Structural ROM classification remains incomplete. All final editorial, fidelity and document-accounting evidence is recorded separately; this outcome is not blocked and does not claim whole-ROM completion.

ROM classification: 99.99% (262140 / 262160 bytes classified; 20 bytes unknown).

Checkpoint: **pass-009-final**. The final source is readable across all classified bytes, with 14686 full-ROM review entries and 2404 distinct routine/structure units reviewed, no classified readability backlog, and explicit uncertainty for every remaining range. Official final editorial audit and fresh configured-binary rebuild passed byte-identically; the reading copy is current.

## Current artifacts and identities

- Supported ROM-free mapping: [maps/current.json](../maps/current.json), format romatlas-nes-map. Internal state, ROM, source/assets, traces and saves remain local.
- Readable current source: [game.asm](../commentary/source/game.asm); [reading copy](../commentary/reading-after.asm).
- Final editorial assessment: [editorial-audit.md](../commentary/editorial-audit.md); [official audit](../commentary/audit.json); [acceptance handoff](final-acceptance-handoff.md).
- Exact structural inventory: [current-audit.json](current-audit.json); [unknown ledger](round009/unknown-ledger.json).
- Terminal reasoning: [redirect handoff](redirect/round009-20261126-a/handoff.md), [primary observations](redirect/round009-20261126-a/primary-residual-observations.json), [coordinator rejection review](coordinator-rejection-review.md).
- Archive preparation: [archive-options.json](../archive-options.json) and [complete document accounting](archive-document-accounting.json). These contain selection decisions only; no archive operation was executed.
- Requirement-by-requirement final checks: [completion-audit.json](completion-audit.json).

Input ROM SHA256: `1913226b60916aee9cb2e45c9c5e0d6a658880e41bcd8837b69ff3d8b32fef85`. Current map/state/checkpoint SHA256: `112cb6e3f56c21efd18004c1768366d7c906852dd046d247de11795c6f3a92b0`. Final source SHA256: `4201260c152256ce179e63bb4ebd96f52adfd7c7e9e2a5e8e3dcfbea4170566d`. Configured binary: `/root/romatlas/target/release/romatlas`; no toolkit modification or substitute build was used.

The final audit command was `python3 analysis/final-acceptance.py`, calling the unchanged official commentary audit with immutable source-read caching. It retained all scaffold, path/line, unit, bounds, editorial and byte-rebuild checks. Completed session2471 accepted14686regions and15154unit records; independent unique-unit count2404. Source stayed immutable during the audit. Fresh supported map.validate/analysis.unknown results and independent physical complement agree. Detailed method: [final-editorial-validation.json](final-editorial-validation.json).

## Sequential workflow ledger

Every round used separate sequential disassembly, commentary and redirect workers, with its own prompt and compact checkpoint/handoff. Every disassembly percentage was printed to the user before commentary. All nine checkpoints validate and rebuild byte-identically; each round's complete current-source commentary gate passed before redirect. No competing phase writers remain. Historical source/review credit and investigations were preserved, with source changes merged by physical identity and changed dependencies.

|Round|Input checkpoint|Output checkpoint|Consumed leads|Redirect outcome|Classified / Unknown|
|---|---|---|---|---|---|
|1|initialized / verified baseline|pass-001|startup|L001–L006|148886 / 113274|
|2|pass-001|pass-002-final|L001–L006|L007/L008|199961 / 62199|
|3|pass-002-final|pass-003-final|L007/L008|L009|231210 / 30950|
|4|pass-003-final|pass-004-final|L009|L010|242561 / 19599|
|5|pass-004-final|pass-005-final|L010|L011|246470 / 15690|
|6|pass-005-final|pass-006-final|L011|L012|249050 / 13110|
|7|pass-006-final|pass-007-final|L012|L013|260667 / 1493|
|8|pass-007-final|pass-008-final|L013|L014|262106 / 54|
|9|pass-008-final|pass-009-final|L014|L015 reviewed-incomplete|262140 / 20|

Phase handoffs and independent primary reviews are under analysis/roundNNN*, analysis/roundNNN/ and analysis/redirect/roundNNN-20261126-a/. Prior full commentary is frozen under analysis/round-002 through round-009. Earlier numerical/mapper/pointer assumptions are retained as history with superseding corrections: D107 ROM rectangle, shop16/240 loop, actual synthetic RTS/frame mechanics, post-selector banks, actual EOR/CMP bodies, narrower PPU/actor suffixes and source-relative review paths. L015 includes every current physical interval/CPU context and specific reopening conditions; no ready lead remains.

## Exact residual and stopping rationale

|Physical start|End exclusive|Bytes|
|---:|---:|---:|
|35348|35349|1|
|36120|36121|1|
|44616|44620|4|
|58392|58393|1|
|82207|82208|1|
|98618|98619|1|
|113182|113183|1|
|114532|114533|1|
|114553|114554|1|
|115870|115871|1|
|125938|125940|2|
|125962|125965|3|
|130916|130917|1|
|130937|130938|1|

The remainder comprises seven isolated NOP/RTS bytes, four isolated2B prefixes, four unconstrained numeric/glyph-like bytes, two5C03 prefix bytes and threeA930A0 bytes. The coordinator inspected actual ROM bytes, bank-aware decodes, synthetic frame behavior and current neighboring structures. The review permits overlap, unofficial opcodes, conditional entry contracts and unresolved natural reachability. Neither legal decoding nor a missing apparent original opcode decides a role. Clear useful suffixes were split and accepted; the remainder still has genuine competing Code/Data interpretations. No useful new discriminating method survived primary rejection review. No byte is retained Unknown merely because its data meaning is unnamed.

Resume only with a new independently supported local control edge/entry, consumer/format boundary, noncode pattern or meaningful algorithmic contract for a specific interval. Use the configured server and explicitly load analysis/state.json; durable scenarios/snapshots and attempt records remain available. Unchanged broad closure, arbitrary PC/RAM sweeps or gameplay replay do not add role evidence.

## Gameplay and remaining limits

[scenarios.json](scenarios.json) retains35 cited evidence files and bounded press/release/patch routes. Natural title, route1 action, inventory/map and timed dialogue were visited. Initialized routes3/9/11/39/40/42 and representative movement/shooting, final special scene and terminal THE END at frame5709 were visited artificially using explicit RAM/stack/entry contracts. Natural ending, boss defeat and every special mechanic were not proved. The42-route table bound superseded the failed route43 premise. No new supported progression contract gives a useful residual experiment; no exhaustive gameplay or compatibility claim follows.

## Toolkit and archive preparation

[toolkit-issues.md](toolkit-issues.md) records two nonblocking throughput feature requests: TK001 atomic batch annotation, TK002 cached source-location indexing. The original large annotation batch and the official final audit were waited to completion; no correctness blocker was demonstrated. Toolkit implementation was not changed.

The archive.py document-selection contract was inspected directly. Additional final handoffs, reviews, research plans/results and historical progress/uncertainty documents are explicitly selected. Only exact duplicated instructional templates in frozen commentary copies are classified for omission; all remain physically retained. The validator accounts for every Markdown file under analysis/ and commentary/ according to the actual scanner exclusions and special recognized documents. Toolkit findings are also summarized here because the archive contract recognizes the separate report but forbids it as an additional document.

No archive, workspace cleanup/deletion, publication or commercial-ROM/asset download was performed. Input, all durable evidence/checkpoints/history and current readable source/assets remain local.
