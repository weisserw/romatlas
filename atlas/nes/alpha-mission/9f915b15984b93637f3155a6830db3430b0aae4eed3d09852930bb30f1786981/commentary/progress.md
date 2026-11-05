# Round 001 commentary — active

Baseline: disassembly-001-final; map SHA256 71f91f57da5affd908db67474f695f700a6275f1130d1e261c14d63f50b9f736.
Prepared with supported commentary.py prepare; initial reconstruction byte-identical.
Frozen before/ and baseline.json are preserved. Final editorial audit remains pending.

Reviewed the complete generated source (7,056 lines) in address order. Drafted 240
meaningful routine/handler contracts in analysis/commentary-001/routines.txt, covering
all classified code, physical $00052–$04225, CPU $8042–$C214 with interleaved data.
This includes every code-region start, JSR target and inspected RTS-directory target.
Remaining: apply routine and branch labels, consistent hardware/RAM/object-field names,
add all non-code format/boundary notes, check transformed source and inventory, fresh
verification and reading copy. No current-source readability completion claimed yet.

Concrete uncertainties retained: standalone demo recording writes hit CPU ROM/mapper
addresses $CBxx on CNROM; natural entry/use unresolved. Final base setup sets six parts
without allocating them, preserving D001-L05. Incoming carry and intentional stack
unwinds are meaningful contracts throughout direction/collision/dispatcher helpers.

Applied full-source batch: 240 routine/handler contracts, 437 named control targets,
385 RAM/object-slot aliases, hardware and common actor-field constants, and notes on
all 135 structural regions. Binary data includes were split at consumer-established
reference starts, with explicit CPU origins to keep data symbols correct. Fresh
commentary verification passes byte-identically (input SHA unchanged). Final semantic
consistency check, concise per-structure units, uncertainties and reading refresh remain.
An initial failed local verification exposed relocatable data labels; corrected by
explicit CPU origins before accepting this batch. No structural map or ROM changes.

## Round 001 — current-source readability complete

All 65,552 classified physical bytes reviewed through the 135-entry scaffold;
12,664 code bytes have 240 local routine/handler contracts and 437 descriptive
control labels. All memory operands, hardware accesses and supported object-field
indexes have consistent names. Small data arrays are visible byte directives;
RTS tables use symbolic high/low target-minus-one expressions. Broad encoded data
keeps honest local format/boundary limits. Review inventory includes 335 meaningful
routine/structure units; its lengths are review allocations, not new map claims.

No remaining current-source readability ranges. `uncertainties.md` records actor
identity, overloaded scratch/fields, recorder hardware limits and D001-L05 without
weakening supported classification. No further runtime research was needed to
explain established mechanics. Final audit remains pending and was not invoked.

Fresh final verification: byte_identical=true, source/game.asm SHA256
62718df9373e8ec7e1f7d06ad1f2c4d9487fafe6656ad042ff86a6a5db3d86b3.
`reading-after.asm` refreshed after the final source changes. Input ROM and structural
map SHA256 match the original disassembly handoff. No toolkit edits, publication,
archive or deletion. One existing confirmed toolkit issue NES-001, no new issues or
blockers; see ../analysis/toolkit-issues.md.

## Round 001 — final editorial acceptance completed

Redirect-supported final-base findings integrated without structural map/state edits.
Five initial terrain parts plus generated core, the six-candidate scan ceiling/$FF
termination and both artificial loops are documented in source, uncertainties and
review summaries. Added two bounded data structures, bringing reviewed units to337;
all135 scaffold entries reviewed. Prior review credit preserved. Corrected literal
stream-offset branch names that misleadingly described actor fields, and normalized
review source paths to be relative to source/ (old redundant prefixes rejected by
initial supported audit, corrected before acceptance).

Targeted quality review and physical/source-location checks passed. Final supported
commentary.py audit: ok=true, byte_identical=true, editorial_audit=true, reviewed_regions
135, reviewed_units337; refreshed reading-after.asm. Source SHA256:
1d7f8cbac526c6224f3036d0695a40288817744f98b7af807a6013695e7f57a8.
No pending editorial/readability ranges. Evidence and limitations in editorial-audit.md;
final acceptance handoff ../analysis/handoff-final-acceptance-001.md. Earlier handoffs
and failed artificial route remain preserved. No archive/publication/toolkit edits.
