# Commentary progress

Final editorial acceptance is now complete against pass-002-reviewed. The final
audit reviewed all 112 scaffold entries and 323 units, rebuilt byte-identically
and refreshed reading-after.asm. Classification remains incomplete at four
unknown bytes. See editorial-audit.md and audit.json.

## Preserved round2 readability phase record

Round 2 current-source readability is complete against **pass-002-reviewed**.
Classification is separate: 24,588/24,592 bytes (99.98%), with four unknown bytes
at [005D,005E) and [3485,3488). No readability ranges are deferred.

Preserved the entire previous phase under
analysis/round02/commentary-round01-final/ before refreshing the frozen baseline.
The original before/ directory is also retained under
analysis/round02/commentary-round01-before-retained/. Generated the current
baseline separately and installed only baseline artifacts; maintained source was
never regenerated. Current baseline, phase1 state, source mapping and review
metadata match maps/current.json and analysis/state.json.

Reviewed the complete current physical inventory: 112 scaffold intervals, including
62 code intervals (11,307 bytes), 48 data/image intervals (13,281 bytes), and two
explained unknown intervals (four bytes). Carried all 322 valid earlier unit
reviews by unchanged physical identity/dependencies, and added the reviewed
CheckMusicReplacementPriority unit for [3609,3614), now five named instructions.
Full code-unit coverage, all review source locations, all source differences,
known metadata corrections and source-wide operand naming were checked. No raw
numeric address operands remain. Current source has 7,576 lines plus symbols.
Full readability reuses sound round1 prose and review, without cosmetic rewrites.

The new entry tests the incoming header's replacement mask using the active music
ID. Zero music or permission loads the selected effect slot; refusal returns to
the original caller. Comments give slot/header/request/stack assumptions, helper
preservation and fixed NROM bank context. Natural entry remains unresolved;
no observed execution is claimed. Both other unknowns retain boundary notes.

Reconciled four round2 map corrections with existing source explanations: initial
BCD extra-life parameters; eight-byte bonus descriptors versus separate seven-byte
player indicators; five pepper coordinate pairs plus twelve remaps; one-based,
stage-capped pepper stun countdowns. Corrected the older initialization comment
that still called the first parameter a pacing value, removed resolved stale-map
complaints and renamed the unused generic EntityStatusValues alias. Source-line
references were remapped after edits, preserving valid review credit.

Fresh verification rebuilt and linked source/game.o and confirmed whole-file byte
identity. The independently generated before/ baseline also verified byte-identical.
reading-after.asm was refreshed afterward; reading-before.asm now reflects the
new frozen baseline, with its original retained in the round1 archive. Evidence:
analysis/round02/commentary-final-verification.json and
analysis/round02/commentary-readability-review.json. No toolkit issues found.

Remaining meanings and assumptions are in uncertainties.md: natural priority entry,
negative sound request domain, opposing directional inputs, segment-wrapper
fallthrough, precise generic data/audio extents and the two structural unknowns.
These are documented limitations, not deferred readability work. Final
editorial_audit stays pending for terminal editorial/archive acceptance. No final
audit, redirect, toolkit edits, archive publication or cleanup ran in this phase.

## Final editorial acceptance continuation

Reused valid full-source review credit and inspected current inventory, changed
priority entry/dependencies, four metadata corrections, known mechanics, naming
and uncertainty notes. Clarified effect-slot hexadecimal offsets in comments;
revised the uncertainty note to distinguish unknown animation intent from pending
research. The first audit found inherited source/ prefixes in review locations;
normalized every location relative to commentary/source and reran successfully.
All 112 entries / 323 units are reviewed; editorial_audit is reviewed. No source
readability scope or concrete editorial defect is deferred.

Fresh final toolkit audit passed with byte identity and refreshed reading-after.
Evidence: analysis/round02/editorial-final-audit.json, editorial-final-verification.json
and editorial-final-artifacts.json. Pre-acceptance records are retained in
analysis/round02/editorial-acceptance-input/. Four structurally unknown bytes and
semantic limitations remain honest and unresolved. No archive execution, cleanup,
toolkit edits, new runtime execution or publication ran.

## Round 3 — pass-003-user-fallback

Preserved entire previous commentary under analysis/round03/commentary-before-fallback-refresh/ before generating a separate verified current baseline. Carried all valid unchanged review credit; reconciled all 112 scaffold intervals and 323 units, with no pending readability work. Replaced the two fallback boundary notes and refreshed source/mapping.json, baseline/state/manifest, review and uncertainties. No assembly bytes or symbols changed. Inspected scroll return/demo and final ladder/audio boundaries plus retained rejection reviews; unresolved roles stay explicit.

Assignment is 24,588 evidence-backed bytes plus 4 Manual generic Data fallback bytes, 100.00% assigned and 99.98% evidence-backed. No new data-role proof or execution claim. Fresh byte-identical verification passed and reading copy refreshed; final editorial findings are in editorial-audit.md, strict audit result retained in analysis/round03/editorial-final-audit.json. No deferred source ranges, redirect, toolkit edits, archive or publication.
