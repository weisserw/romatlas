# Round002 disassembly handoff

Exactly one disassembly pass; no commentary/redirect executed. Both ready leads
R001-R01/R001-R02 independently verified and applied together. Input pass-001-final
map401371b3a2cf4bc85c43e2696f369e3fefa76e0e853acccd920a0e82aed52db9;
output pass-002-refined map
ff10526069bb622c0e919dc445090ae1f0a08f25b67aefde0bde2018d6a3a356.
Internal analysis/state.json saved, maps/current.json exported, map.validate passed;
analysis/pass-002-refined-verification.json confirms byte-identical rebuild.

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).
9478 Code /23306 Data /32768 Image;88 regions. Per-bank exact counts bank-counts.json:
header16 Data; PRG0 5598Code/2594Data; PRG1 3880Code/4312Data;
PRG2/3 each8192Data; CHR0..3 each8192Image; all units0 unknown.
No newly classified bytes or new reachability claim. Static provenance retained.

physical0556..0557 corrected SetupChoiceDestinations (44/6A are RAM addresses).
Sound refined existing Data:3261..3278 twelve period words,32D1..3328 twenty-two
four-byte descriptors,3329..3664 twenty-two bounded streams. First6 loop-tailFF
not claimed reached; last stream consumes3664FF. Gaps3279..32D0 and3665..8009
retain generic Data. Independent review.md, fresh sound-verification.json, API actions
and verification-summary.json give primary decision/evidence and exact preservation.

Full commentary tree and frozen round001 baseline byte-identical to input; do not
regenerate over them. Next commentary reconciles five replacement Data regions by
physical identity, retaining existing full explanations and enhancing exact sound
boundaries/schemas; source already has setup correction. Prior84region/261unit gate
must be carried/reviewed against current88regions, then final editorial work remains.
No classification unknowns; original units/generators, some caller/selector domains,
remaining data subformats and RAM intent remain honestly unresolved.

Broad gameplay reused unchanged prior scenarios (later9/18,flight,putting,terminal).
No replay warranted for static metadata claims. No toolkit issues found; see
analysis/toolkit-issues.md. No toolkit changes, archive, cleanup, deletion, publication,
commits or live server. ROM hash unchanged and all evidence local.

Preserved pass-002-final is an unchanged-input checkpoint accidentally requested after
an unsaved API missing-id rejection; pass-002-refined is the authoritative output.
