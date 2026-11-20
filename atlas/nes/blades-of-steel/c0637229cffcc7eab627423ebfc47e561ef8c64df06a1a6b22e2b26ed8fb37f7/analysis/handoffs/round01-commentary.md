# Round 1 commentary handoff

Current-source readability complete; ROM classification remains incomplete.
Checkpoint round01-disassembly-final unchanged; map SHA256
`ecde055d22cf42971290adbfd33853bdcb7abe0d42907ed8ad330a9693f68c62`.
ROM unchanged. No disassembly, redirect, toolkit edits, publication or cleanup.

Covered all15,218 Code and33,020 Data bytes, plus honest exact-boundary notes for
82,850 Unknown bytes. All7,626 scaffold records reviewed,381 hand-inspected
meaningful units with source locations,493 descriptive routine/branch labels.
No remaining readability gaps; exact pending-ranges.json is empty. Final editorial
acceptance is separate:review.json editorial_audit remains pending; no audit run.

Editable source:commentary/source/game.asm and symbols.inc. Frozen before/baseline
and historical evidence retained. Fresh final byte-identical verification passed,
commentary/verification.json. Reading copy refreshed:commentary/reading-after.asm.
Full scope assessment and hashes:commentary/scope-assessment.json. Source SHA256
`68b8914ded989b70c0debd79399f6c01de17bf159616bab9ea69cdb346d6efd4`;
symbols SHA256 `b5cc634f03a7bdb8b9b58cbaa3292a4af57cbf9f775e34bbfb22c787414110c9`.

Examples in source/game.asm:820 metasprite expansion;21819 sound-channel
arbitration;28288 NMI/bank protection;30650 stable controller edges;31917 inline
JSR-table dispatch;32277 banked PPU token decoder;42384 packed collision probes;
46362 scaled squared-distance computation. Other code covers scene/match-state
fragments, menu highlights, raster scroll, motion/drag, animation and angle helpers.
Unknown game meanings use mechanical names and ordinary tentative prose. Reused
scene RAM60/61 has neutral global names; explicit widths and bank contexts retained.

Redirect questions (details/competing interpretations in commentary/uncertainties.md):
- C01:bank0 physical000328..0003F1 lies within consumed $8316 sprite-pointer table
  yet frozen Code. Inspect bounded selector/table extent and actual fetch evidence;
  review through the renderer tail without assuming pointer referents are code.
- C02:bank1 physical0040BF..0040CE overlaps inline PPU template $80AF copied until
  FF; physical0043EB is KIL at an indexed-table edge. Review exact payload ends and
  independent instruction evidence, preserving neighboring genuine handlers.
- C03:fixed physical01DF49..01DF4B is SLO decoding at animation table base $DF39,
  consumed by direction/frame lookups after an unconditional branch. Bound indices
  and separate data interpretation from any genuine fetch evidence.

Also retain disassembly's independent promising avenues:clock/faceoff/period gate,
callee return/bank-preservation contracts and graphic-stream boundaries. Current
comments explain established mechanics but do not resolve these structural/runtime
questions. Speech phrase names, menu team indices/assets, collision event meanings
and natural later-period progression remain uncertain. No toolkit issues found;
analysis/toolkit-issues.md. Commentary artifacts and all assets stay local.
