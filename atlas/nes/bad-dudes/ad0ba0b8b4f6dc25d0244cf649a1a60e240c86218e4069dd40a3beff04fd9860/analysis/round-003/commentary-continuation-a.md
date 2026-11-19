# ROUND003 COMMENTARY continuation A — incomplete, resume commentary

Stopped for sequential capacity continuation. This is **not** a successful commentary handoff and does not authorize REDIRECT. ROM classification remains **99.54% (260956 / 262160 classified; 1204 Unknown)**. Final editorial audit remains pending and was not run.

Current source is buildable and freshly byte-identical; `reading-after.asm` is refreshed. Checkpoint/map identity remains `round-003-disassembly-final`, SHA256 `4f2424c84e630ea624df17aa8dfa32fbdd5cb1422ca3a095e04a5b9354b22f12`. All input/state/source/assets remain local and input is unchanged.

The entire prior commentary tree is preserved at `analysis/round-003/commentary-round002-preserved`; `commentary-preservation.json` records all26files with matching size/SHA inventories. Fresh supported prepare established the changed-map baseline. Exact physical range/kind/CPU-view carry retained1833records and useful source; post-carry verification passed. Historical scripts under earlier rounds were not rerun. New scripts here are historical **non-idempotent transformation records; do not rerun them**. `commentary-tools/batch_support.py` is a reusable edit/location helper if useful.

Current review scaffold:2373records,2208reviewed/165pending;2738units. The physical partition covers the ROM exactly, every reviewed Code scaffold has units, and source-tree-relative metadata checks report0errors (`commentary-scope-assessment-a.json`). Review credit for unchanged prior work remains valid; the scoped dependency inventory separately gates completion.

Completed:
- All84Unknown intervals /1204bytes have honest boundary/role notes. C607 is a legal alternate copy entry, C5CA rejection is scoped, and bank8Unknown includes STA at physical10BF1; Data starts10BF4. No structural classification changed.
- All236newData scaffolds /10762bytes reviewed with421meaningful format/field/stream units. All295accepted underlying subclaims were matched to actual local byte hashes and reused consumer/boundary contracts. Shared/overlapping 37-animation streams are explicit, as are 197shape-ID/motion fields,74/25rectangle bounds, palette/OAM/PPU formats, RAM versus ROM pointers and ordinary/synthetic/IRQ RTS tables. Generic residual data retains explicit schema/consumer limits. D283 at1E95D is now correctly named thirty actor path-stream origins.
-55complete newCode runs /785bytes: both bank5RAM helpers (94bytes),21fixed helper runs with27routine contracts (409bytes),32fixed API runs with94individual slot units (282bytes). Existing API source was carried, so all newly accepted slots now have meaningful target references.
- An additional23-byte reviewed suffix at physical **1FBED..1FC04** (`F15QueueBackgroundPaletteBuffer`) is tracked inside pending scaffold **1FB93..1FC04**. Its first90bytes **1FB93..1FBED** remain unread. `commentary-partial-coverage-a.json` preserves this exact credit.
- C09F now tail-calls the directional Y-step helper: subtract A with direction bit3 set, add otherwise; retain coordinate mutation, then return an asymmetric world-X test on subtraction or Y-limit test on addition. C0E4 clears second contact-shape bit7; C1EF sets it, preserving the index.25affected paragraph fragments and21actual call operands were updated; caller code ignores C09F result where documented. Uncertainties were revised without erasing historical research.
- Corrected stale stage palette, five-phase RTS, OAM RTS, thirty-path-pointer and copied path-flag claims. Dependency candidates DEP002/DEP040/DEP138 reviewed; other API caller fixes are independently recorded in `commentary-dependency-three-api-a.json`.

## Exact remaining current-source scope

`commentary-pending.json` is authoritative for all165pending physical Code ranges and IDs; `commentary-pending-by-bank-a.json` groups their exact endpoints. Pending scaffold bytes7551 include the23-byte suffix already reviewed, so truly unread Code bytes are7528. No pending Data/Image/Unknown records remain, but concrete affected dependencies can still require Data/prose corrections.

| Physical 8KiB PRG bank | Pending Code scaffolds | Scaffold bytes |
|---|---:|---:|
|6|24|2130|
|7|29|1090|
|8|41|1211|
|13|29|1171|
|14|29|1008|
|15|13|941|

Next coherent frontier: complete the remaining fixed engine/life/IRQ/OAM/fade bodies or bank6screen/life graph, then all bank7stage, bank8event, bank13animation/player graphs. All165ranges are required; this is not a small optional repair pass. Large scaffolds need multiple actual routine/field/control-flow units and exact partial coverage until complete, not one generic label. New Code has no inferred natural-use claim. Retain bank/frame contracts from `analysis/round-003-disassembly/adjudication.md` and primary redirect contracts as needed. Fixed mode bit6-clear preserves fixed banks14/15; lower-bank preservation is a separate contract. Sound wrappers temporarily map0/1; IRQ saved frames differ from ordinary callers.

`commentary-dependency-candidates-a.json` contains139scoped candidates from **exact carried blocks**, with current source anchors, unresolved prose and numeric operands that resolve into new Code/Data under supported effective views. **136remain pending**. These are candidates, not136proven defects; check each against new bodies/formats and retain sound uncertainty. Avoid a whole-source reread or cosmetic rewrite. The completed three-API callers are separate evidence; inspect any remaining concrete contradictions.

A concrete next dependency issue is **DEP066**, physical10525: old `B8LoadSecondClassActionRecord` prose calls C018 a four-byte record read, but the fixed API actually jumps to `F15Multiply8By16`. Follow the adjacent arithmetic/continuation and correct the mechanical account, rather than inventing a table reader. Other cloned API units may contain stale metadata inherited from reviewed body summaries; check actual source/body when affected. Newly completed code should enable sound caller/table uncertainty updates mechanically.

Final source hashes at this continuation:
- `game.asm`: `28a5ed5e7d4ab1d8113b2e59cbf38afe064a48346b6f9a9dc51e340491eeab44`
- `symbols.inc`: `425f47881fb4782f457a44f1fbd5214b1b0cad6f860d471612c7ca98da0f6ff8`

The first Data rebuild exposed repeated fragment label names; unique labels corrected them. A fixed-helper rebuild caught two mistaken numeric alias substitutions; both were corrected before the fresh successful verify. No toolkit issue or change resulted. All recorded final checks pass. Keep these historical transformation recipes unrerun.

Update `commentary/progress.md`, `analysis/progress.md`, `analysis/investigation.md`, pending inventory, partial credit and dependency status after coherent batches. Verify/read each batch and final successful handoff. Complete all current-source readability before writing `analysis/round-003-commentary-handoff.md`; **no such successful handoff exists yet**. Stop writing before the next sequential worker starts.
