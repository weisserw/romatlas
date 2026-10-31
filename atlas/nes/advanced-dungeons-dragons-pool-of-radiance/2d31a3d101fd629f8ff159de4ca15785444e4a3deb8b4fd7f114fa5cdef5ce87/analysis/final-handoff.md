# Terminal handoff — justified incomplete ROM classification

ROM classification: 100.00% (655371 / 655376 bytes classified; 5 bytes unknown).

The exact percentage is 99.9992370792%. Rounded 100.00% is not whole-ROM completion. The sequential coordinator workflow reached a justified terminal incomplete outcome after six disassembly/commentary/redirect rounds and a primary-evidence rejection review. This is an epistemic classification limit, not a capability blocker. Current-source editorial acceptance and verification are recorded separately below.

## Current checkpoint and residuals

Pass-006 is current. `maps/current.json`, `maps/pass-006.json` and `analysis/state.json` share SHA256 `5912c6e9eaf7bf3a0902fbaa91908d491543e8c2971ccb192fea203197c2c3df`. The map has 12158 evidence-backed regions: Code 71562, Data 356710, Text 96027 and Image 131072 bytes. Header bytes count toward the total. All Code execution views independently map to their physical storage offsets.

| Physical offset | Length | Effective CPU view | Raw bytes | Remaining ambiguity |
|---:|---:|---|---|---|
| 10160 | 1 | bank 1:$A7A0; [0,1,62,63] | 60 | Additional native return versus scalar/residue |
| 432955 | 1 | bank 52:$BB2B; [58,52,62,63] | 60 | Redundant native return versus scalar/residue |
| 460056 | 3 | bank 56:$8508; [56,53,62,63] | A2 00 60 | Zero-X native helper versus noncode component |

These bytes remain opaque in source and absent from classified map regions. Legal instruction decoding, lack of a caller and successful rebuilding do not settle their roles. Nearby helpers independently terminate; the actual finite service/display interfaces and fixed synthetic returns do not supply the proposed residual contracts. Generic Data lacks a positive noncode pattern here. The coordinator challenged those premises against primary bytes, consumers and bank translations rather than accepting an absence-of-execution argument. See [coordinator review](coordinator-rejection-review.md), [R6 rejection review](redirect/r6-20261031-a/rejection-review.md) and [R6 handoff](redirect/r6-20261031-a/handoff.md).

No ready lead or useful unchanged replay remains. A typed native slot/output contract, independently bounded constant component or genuine contextual instruction fetch could reopen the relevant interval. This does not claim the bytes are unreachable, unused or dead.

## Results and investigation history

Successive accepted classified totals were 382424, 551040, 623279, 655353, 655365 and 655371 bytes. The intervening redirect batches resolved connected native contracts, VM/dispatch consumers, text and graphics pools, record/list bounds, startup PPU pages and generic noncode regions. Later primary reviews established descriptor-field prefixes, service-40 bank translation, explicit/default range-limit arguments, the BRK saved-PC partition, and the six-byte descriptor-six clear/retreat wrapper. Earlier failed interpretations and their narrower corrections remain preserved. Each disassembly pass was independently followed by current-source commentary and redirect; percentages were reported before commentary. See round1 through round6 coordinator gate records, [progress](progress.md), [investigation](investigation.md) and [current disassembly handoff](round6/handoff.md).

Natural gameplay covered menu/prebuilt party, Phlan city, statistics/equipment, slums/orcs, tactical battle and Sleep. Documented coherent artificial routes exercised overworld, dragon battle interaction and STAFF/credits with return to mode 1. Natural final victory and the full ending montage remain unverified. Failed partial patches and timing attempts are retained with their specific limits. No full game/hardware compatibility is claimed. Natural observed execution remains 9223 instruction starts / 18950 PRG bytes (3.6144%); structural classification is a separate result. See [gameplay](gameplay.md) and local `analysis/scenarios.json`.

## Readable source and verification

The local porting deliverable is `commentary/source/game.asm` with companion includes; `commentary/reading-after.asm` is the refreshed reading copy. All 12161 scaffold entries and 4272 units are reviewed, with mechanical explanations and explicit unknown meanings. Prior valid C1–C6 review credit is retained. Final manual acceptance checked naming, field/context distinctions, branch and synthetic-return contracts, the descriptor-six lifecycle and all remaining unknown boundaries. The final inventory repairs normalize source-root-relative references and attach existing units to their containing scaffold entries; all 4272 records were preserved. See [editorial audit](../commentary/editorial-audit.md) and the final editorial handoff under `analysis/final-editorial/20261031-a/`.

The final supported audit passed: `ok=true`, `byte_identical=true`, `editorial_audit=true`, 12161 reviewed regions and 4272 reviewed units. Fresh verification hashes match every current assembly/include file. Final game.asm SHA256 is `b624cebc9d65e6b9c9a27002991a0fed3ae9ebf080be263d28e9c6f3b760c5d6`; refreshed reading-after.asm SHA256 is `c8086199aa44d77dfba6b906cbac4dce946e8ead0fa0079f8f53b2c04dfdd991`. The actual audit is `commentary/audit.json`; [final editorial handoff](final-editorial/20261031-a/handoff.md) records exact artifacts.

Supported internal-store and portable-map validation both passed. Independent evidence, bank-view and exact-complement checks are in `analysis/final-map-invariants.json`. The original ROM SHA256 is `2d31a3d101fd629f8ff159de4ca15785444e4a3deb8b4fd7f114fa5cdef5ce87`; it and the configured release binary remain unchanged. No toolkit issues found; inventory errors were local editorial defects, corrected without toolkit edits.

## Preservation and scope

`archive-options.json` explicitly selects additional substantive Markdown conclusions, handoffs and history and accounts for disposable duplicate operational instructions. The selection review is `analysis/archive-document-review.json`; the inspected contract is `analysis/archive-contract-inspection.json`. No archive or restore operation was run. No publication, deletion, cleanup, commercial ROM download or toolkit modification was performed. ROM, source, assets, internal state, captures and traces remain local. The supported ROM-free portable mapping is `maps/current.json`; internal state is not a portable-map sharing deliverable.
