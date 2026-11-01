# Round 001 final editorial acceptance

Accepted the current readable source, including its explained unknown boundary.
This is editorial acceptance of an incomplete classification: 65551/65552 bytes
classified, with physical [$39C7,$39C8), CPU $B9B7, effective PRG [0,1,2,3] Unknown.
The coordinator accepted the redirect's terminal-incomplete rationale separately;
this audit neither changes that classification nor claims whole-ROM completion.

Reused the completed current-source review on the unchanged baseline: 96 code
regions (11837 bytes), 53 data regions plus one CHR image region (53714 bytes),
and one explained unknown entry. All 151 scaffold entries and 350 units retain
reviewed status, physical ranges, useful summaries and source locations. No
readability ranges remain pending. A whole-source reread or regeneration was not
performed; the existing substantive review remains valid.

Targeted findings and fixes:

- Corrected every review location from `source/game.asm:line` to the audit's
  source-directory-relative `game.asm:line`, refreshing line positions after edits.
  The earlier paths could identify files to a reader but would fail the supported
  audit's local-path resolution. Prior review and source are preserved under
  `analysis/final-editorial/round-001-before/`.
- Updated the unknown-byte note and review summary with redirect's exact scope:
  no independent entry or supported payload format distinguishes redundant return
  residue from spare data. No missing-reference argument is promoted to data proof.
- Updated music comments and uncertainties from the accepted static investigation.
  `$FE` reloads all four channel pointers, so independent channel parsing loses
  restart ordering. A shared-scheduler model of all nine source-selected tracks
  repeats within 2561 modeled frames and uses `$F8–$FF`; this does not establish
  observed execution or valid use of `$EF–$F7` beyond the eight-entry directory.
- Retained meaningful routine/control/field/table names and light functional prose.
  Checked key reset/NMI/IRQ, queue, pulse-fade and music entry labels, include
  references, physical/banked identity notes, partial data schemas and local-only
  payload/notices. No generic address-only routine or Block-label fallbacks remain.
  Shared per-kind enemy fields and unidentified game/audio/CHR associations retain
  their explicit mechanical interpretation and uncertainty.

Validation:

- Supported `commentary.py audit .`: `ok=true`, `editorial_audit=true`,
  151 reviewed regions, 350 reviewed units, byte-identical rebuild.
- Separate fresh `commentary.py verify .`: `ok=true`, `byte_identical=true`.
  `commentary.py read .` refreshed the final reading copy afterward.
- Preservation checks confirm unchanged ROM, current map, internal state and
  frozen baseline; unchanged executable assembly statements; preserved scaffold,
  unit names/ranges and review credit. See
  [checks](../analysis/final-editorial/round-001-checks.json),
  [audit](audit.json), [verification](verification.json) and
  [reading copy](reading-after.asm).
- Final `source/game.asm` SHA256:
  `ddb4ecd2a2f11eaf546b1f9b1cdd52d541df4681406415d7dda406a1bec56c46`.

Remaining limitations are recorded in [uncertainties](uncertainties.md). The sole
structural unknown is accepted as unresolved; semantic uncertainties within accepted
data are not classification gaps. No full hardware/game compatibility or execution
coverage claim is made. No new toolkit issues were found; the existing report has
one confirmed documentation/parameter issue, no suspected issues, no separate
missing features and no blockers. No toolkit edits, deletion, archive execution,
publication or classification changes occurred. The coordinator retains archive
option selection and final orchestration checks.
