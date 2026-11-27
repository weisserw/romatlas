# Round001 commentary

Current-source readability complete; whole-ROM classification incomplete.
Baseline: ../maps/round-001.json and ../analysis/round-001-verification.json.
Frozen before/, baseline.json and phase1-state.json remain intact. No map or ROM
byte changes were made. All3495 scaffold entries were inspected and have source
locations and review summaries;282 compact functional units cover meaningful
routine/control/format fragments. Unknown boundaries remain explicitly unresolved.
Final editorial_audit stays pending; no final completion audit was invoked.

Working source: source/game.asm and source/symbols.inc. All3303 instruction records
were reviewed in physical/bank order. Contiguous compatible instructions now read
as blocks rather than separate per-instruction ORG sections.540 entry/branch labels,
shared hardware/RAM names and object/pool field symbols replace generator labels
and established numeric operands. Comments describe audio channel sequencing,
player/input/terrain movement, thrown/projectile updates, cooperative continuation
scheduling, controller unlock matching, sprite formats/masking, spawn stream parsing,
background row generation and packed terrain queries. Generic data retains concise
pattern/boundary evidence; tile-bank format and hardware vectors are explained.

Semantic corrections during review: field7 is a shared counter, not universally
age; background palette source is $9A20 plus its request byte; projectile pointer
fields are not mislabeled as sprite pointers. Existing useful classification prose
was retained. Known uncertainties and palette-classification contradiction are in
uncertainties.md. There are no remaining classified-source readability ranges.

Fresh source verification passes byte_identical=true in verification.json; the
source hashes identify this exact working copy. reading-after.asm is refreshed
for handoff. Original input SHA256 remains
42664d5b34381a00f6d92214620a73ab851c8679c7a805a1f4239b8e9b1726e5.
Classified101725/131088 bytes, unknown29363; neither this phase nor successful
rebuilding establishes full classification or hardware/game compatibility.

Toolkit: no new issues in commentary; existing nonblocking confirmed documentation
mismatch/missing schemas NES-001 remains in ../analysis/toolkit-issues.md.
