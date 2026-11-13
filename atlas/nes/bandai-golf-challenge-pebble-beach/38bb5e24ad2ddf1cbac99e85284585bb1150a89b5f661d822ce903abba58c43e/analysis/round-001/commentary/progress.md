# Round 001 commentary

Input: verified pass-001-final, map SHA256
401371b3a2cf4bc85c43e2696f369e3fefa76e0e853acccd920a0e82aed52db9.
Prepared frozen baseline and editable source using configured release binary.
Initial verification byte identical. Full scope: 84 regions, 9478 code bytes,
23306 data bytes, 32768 CHR image bytes. No prior commentary review credit.

Current-source inspection read all 5076 original assembly lines. Naming and
purpose explanations are pending across the entire code; data notes and independent
boundary/role review are in progress. No completeness claim follows from coverage.
Final editorial audit remains pending for final acceptance.

## Full source edit batch verified

Applied naming and purpose comments throughout all current source: 216 routine/
handler contracts, 393 branch-target names, a symbols include for CPU RAM, hardware,
player fields, sound fields and data-address bases; all eleven target tables and
interrupt vectors are readable words. Every Data/Image interval has a source role/
boundary/uncertainty note. Corrected a misleading coordinate interpretation of
setup destination bytes and the compressed-stream continuation mechanics.

Independent structural review is saved at
`analysis/commentary/round001/structural-review.md`, with bounded primary-byte
window summaries in `data-windows.json`. No code/data boundary contradiction found;
three operand-entry streams are preserved and explained. No map/state edits.

Latest coherent batch passed fresh byte-identical verification and refreshed
`reading-after.asm`. Entire source has been read; remaining work is inventory
reconciliation, source-location consistency and explicit uncertainty/handoff notes.
No unread or deferred ROM range remains. Final editorial acceptance stays pending;
no `audit` command has been run. No toolkit issues found.

## Phase handoff: current-source readability complete

Reviewed all 84 scaffold regions and 261 meaningful units, including 219 code units
and 42 Data/Image units. Covered all 9478 Code, 23306 Data and 32768 Image bytes;
no unread, unnamed-mechanics backlog or deferred readability range remains.
Review inventory/source locations reconciled in `review.json`; full-ROM scaffold
is unchanged. Role evidence and unresolved meanings are assessed in
`analysis/commentary/round001/readability-assessment.json` and `uncertainties.md`.

Examples: exact nonlocal NMI returns, inline-word dispatch, three overlapping
operand entries, restoring division, curve/wind/ground physics, PPU compact/chained
run decoding, counted sound loops and channel ownership guards. Unknown original
units/formats are explained mechanically without invented game meanings.

Fresh final `verification.json`: ok=true, byte_identical=true. Reading copy
`reading-after.asm` refreshed after final source edits. ROM, frozen baseline,
current map and internal state hashes are unchanged. Current map validation passed.
Source SHA256: game.asm
68d3eaa59715e9c5a553f22d2563ae7cefe9f97f5016408982eced6b7a10471a;
symbols.inc efc7b1888e0856aad909d3f1431fce653cd4a7a1994f88520ce77d491a85bce9.

Known upstream annotation correction before final acceptance: physical $0556..$0557
is setup RAM destinations $44/$6A, not coordinates. Source is correct; frozen map
prose/label remains historical and current map/state need a narrow metadata-only
correction in redirect. Exact old/new claim and consumer are in uncertainties.md.
This does not change Data role, boundaries or current-source readability coverage.
Final targeted editorial audit remains pending and was deliberately not run.
No toolkit issues found. No toolkit edits, archive, cleanup, publication or commits.
Free disk 6.6 GiB; no new trace/capture scratch created (prior scratch unchanged).
