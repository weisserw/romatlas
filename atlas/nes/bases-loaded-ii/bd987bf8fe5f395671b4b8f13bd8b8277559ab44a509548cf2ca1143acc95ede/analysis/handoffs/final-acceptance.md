# Handoff: final acceptance

**Verdict: whole-ROM complete. Every gate passes:** evidence-backed classification of every
byte, required annotations, valid maps, final commentary audit and byte-identical
verification. No gate fails. Semantic uncertainties remain and are listed below; they do not
affect structural classification.

## Checkpoint
- The map is unchanged by this pass: maps/round6-disassembly.json = maps/current.json =
  analysis/state.json (sha256 a5b53604...).
- input/game.nes is unchanged (sha256 bd987bf8...). The toolkit was not modified.
- Nothing was archived, published or cleaned up.

## Classification and annotations
- All 393,232 of 393,232 bytes are classified (100.00%); 0 bytes are unknown. 2,248 regions
  partition the image with no gaps or overlaps.
- Byte counts by kind: code 55,746, data 123,102, padding 83,298, text 14, CHR image 131,072.
- Annotation coverage is 2,248/2,248 regions. Every region has non-empty evidence, a
  provenance and a confidence:
  - Provenance: 834 regions are Observed and 1,414 are Static.
  - Every Code region has a CPU view with bank context.
- Low-confidence static code is kept, each region with its own evidence text:
  - L-015 fragments: 34 regions at confidence 0.6 (no caller known; reachability unresolved).
  - 9 regions (23 bytes) at confidence 0.5. These are the L-016 stray returns/epilogue
    (10 bytes, unreachable as far as known) plus two round-2 sibling fragments.
  - Source comments state this status at each site.

## Map validation
- `romatlas map validate --rom input/game.nes maps/current.json` returned `{"ok":true}`.
- `romatlas store validate --store analysis/state.json input/game.nes` returned `{"ok":true}`.

## Editorial review (targeted; earlier review credit reused)
- No ROM-address numeric operands remain in code.
- Stale round-6 aliases are gone: D_B02_E811/E812/ED89, D_B14_C0FA/C5A2, D_B02_FAE9 and
  Layout_*Hi.
- Every identifier in comments resolves to a source symbol. The only exceptions are
  intentional family references (TeamRosterNN and similar) and "Previously named" notes.
- No adjacent duplicate comment blocks and no TODO notes or stale "unclassified" notes.
- Weak-evidence sites are worded with explicit uncertainty. Checked: D_B07_CD0D, D_B06_CC82,
  the $D0D9 table, the RREC_* fields and the L-015/L-016 code.
- One fix: the DrawUmpireMarker comment contradicted the routine name ("base marker"
  sprites). It now separates the mechanics from the unconfirmed umpire interpretation.
- review.json was regenerated with commentary/tools/review.py. Only line numbers shifted.
- editorial_audit is recorded as reviewed, with findings in its notes.

## Audit and verification
- `commentary.py audit`: ok, byte_identical true. 2,248 regions and 1,883 units reviewed,
  editorial_audit true. Output: commentary/audit.json and commentary/verification.json.
  Source sha256 is 75da08cb...
- The reading copy commentary/reading-after.asm was refreshed by the audit.
- The map-generated assembly was rebuilt fresh: `romatlas verify input/game.nes --generated
  generated/round6-disassembly` returned byte_identical true
  (analysis/final-acceptance-verification.json).
- The audit cannot judge prose quality. The editorial review above is the agent's own.

## Residual uncertainty (semantic only)
Details are in commentary/uncertainties.md (round-6 and final-acceptance sections) and
analysis/leads.json.
- D_B07_CD0D: entries 12-23 rest only on the pointer stride.
- D_B06_CC82: the index bound was not traced. Its extent rests on the parallel table
  D_B06_CCA2.
- The $D0D9 word table (batting-average text) is not decoded. It lies in a generic block.
- These names are tentative: roster fields $02-$05, $0A and $0B; byte-0 bit5; lineup position
  numbering; and the PREC_* interpretations.
- Not established: which close-up side table goes with which AtBatFlags bit7 state, and the
  record boundaries from bank 14 $CA4A.
- Deferred runtime confirmations: L-009 (counters/password), L-018 (debug paths) and L-020
  (stamina relief reachability).
- Toolkit: TK-001..TK-005 are unchanged (analysis/toolkit-issues.md). No new issues.
