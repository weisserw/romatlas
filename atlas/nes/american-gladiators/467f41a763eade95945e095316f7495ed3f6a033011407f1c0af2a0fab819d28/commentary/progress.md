# Round 02 commentary — current-source readability complete

Covered the full enlarged verified baseline:29527 code,101560 data and131072 CHR
bytes, plus an honest note for the single unknown physical byte$CBDF. There are no
pending current-source readability ranges. Final editorial_audit remains pending
for terminal acceptance; readability does not classify the unknown byte.

The complete previous commentary is preserved under
[commentary-round-01-preserved](../analysis/round-02/commentary-round-01-preserved/).
Validated325 exact physical source intervals against unchanged current region
identities, lengths and kinds before carrying their prose,689 code/branch names
and shared RAM/hardware definitions. Valid review credit is retained; affected
old claims that adjacent code/data was unclassified were revised locally. The
IRQ-vector note now identifies the independently supported RTI.

All591 newly exposed code fragments now have functional/contract explanations.
Additional internal labels explain multi-entry signed-overlap, gravity/motion,
field-reader, scroll-compensation, random/multiply and sound handlers. Source
covers inline stack parameters, actor creation and scheduling, titles/briefings,
event control and completion, bank10 ending, PPU probes/queueing and bounded
scene/graphics decoders. Sound streams use named opcode constants tied to the
actual handler entries; aliases distinguish CPU execution values from physical
data labels. Memory operands use RAM aliases instead of equal-valued button masks.

Data source notes explain finite animation/metasprite/rectangle/template records,
palettes, descriptions, compressed media, sound/envelopes/DMC, motion paths and
per-range inspected generic reservoirs. Small records expose literal byte values;
large local binary streams retain named boundaries, formats and consumer limits.
CHR's full planar tile allocation retains its prior explanation. Actual unknown
meanings and the literal differing utility behaviors are in
[uncertainties.md](uncertainties.md).

review.json preserves the20257-entry full-ROM scaffold and has2258 compact source
units; every region/unit is reviewed for current readability. Contiguous sound
records share meaningful sequence units instead of one inventory unit per opcode.
[Scope check](../analysis/round-02/commentary-work/scope-check.json) records the
separate classification/readability counts; source line locations are refreshed.

Fresh [verification](verification.json) is ok and byte_identical against the
unchanged input ROM using the configured release binary. The
[reading copy](reading-after.asm) is refreshed. Final source SHA256:
`5a129504441af40b3df1b9887303a36d1f6c0d1fe61d3821af82ad53dea18cc8`;
symbols SHA256:`48b08175939aa83bb1dd8c9cebddc9386817e3c71fed9c81ec2f7cf1bcea4f64`.
Frozen/current map and state remain
`c2f9133cdb39a2b157df5db1758ea669e25f441c8e97d127cee34af3b47d819d`.

Classification remains262159/262160 (99.99961855355508%, rounded100.00%):one byte
unknown. No new runtime execution coverage, map annotations, toolkit edits,
cleanup, archive or publishing. Existing accepted research/gameplay was reused.
No toolkit issues found; no blockers. Next workflow phase is redirect residual
review, with final editorial acceptance still reserved for the terminal gate.

Review metadata correction: resumed this phase before redirect to replace each
region's string unit reference with its full reviewed unit object. All20257 nested
occurrences represent2258 distinct source units. Source paths now use the toolkit's
required commentary/source-relative form. The
[correction check](../analysis/round-02/commentary-work/review-schema-correction.json)
validates scaffold identity, every region/unit, exact range coverage and every
source location. Source/definitions/reading copy were unchanged, so the recorded
byte-identical verification remains applicable. Final editorial audit is still
pending; this was a metadata validation, not terminal acceptance.

## Coordinator final acceptance complete

Final editorial acceptance is now reviewed in editorial-audit.md and review.json. The configured toolkit final audit passed with byte-identical rebuild, all20257 regions/nested unit occurrences reviewed, and refreshed reading-after.asm. Current source and definition hashes match audit.json. This supersedes the phase handoff's intentionally pending final audit; prior phase history remains unchanged.

The terminal redirect rejection review was inspected against primary bytes and sound control-flow/producer constraints. No ready leads remain on current evidence. ROM classification is incomplete:262159/262160 bytes, one unknown at physicalCBDF. See ../analysis/handoffs/terminal.md for final artifacts and reopening criteria. No archive, cleanup or publishing performed.
