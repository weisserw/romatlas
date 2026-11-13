# Round 001 commentary handoff

Complete current-source readability; final editorial acceptance remains pending.
Input/output structural checkpoint unchanged: pass-001-final, 84 regions;
map/state SHA256 401371b3a2cf4bc85c43e2696f369e3fefa76e0e853acccd920a0e82aed52db9.
ROM classification: 100.00% (65552 / 65552 bytes classified; 0 unknown).
9478 Code / 23306 Data / 32768 Image. No deferred/unread readability range.

All 84 review entries and 261 units reviewed (219 Code, 42 Data/Image).
Buildable local source: `commentary/source/game.asm`, `commentary/source/symbols.inc`.
Reading copy: `commentary/reading-after.asm`.
Fresh byte-identical verification: `commentary/verification.json`, also retained at
`analysis/commentary/round001/verification.json`. Current map validation passed.
Source hashes: game.asm
68d3eaa59715e9c5a553f22d2563ae7cefe9f97f5016408982eced6b7a10471a;
symbols.inc efc7b1888e0856aad909d3f1431fce653cd4a7a1994f88520ce77d491a85bce9.
ROM and both frozen/current map/state files hash-checked unchanged.

Source explains NMI stack continuations, setup/course/club/score/shot states,
RAM/player/sound fields, decimal/wide math, compact/chained PPU runs, ball physics,
wind/terrain/cup handling and sound command/channel priorities. 216 added explicit
routine/handler contracts plus the standalone helper and two additional overlay
unit contracts; 393 branch-target names. All dispatch tables and vectors use words.
Three operand-entry tricks retain exact canonical bytes, alternate symbols and
stack explanations. Comments preserve uncertainty in original units and meanings.

Independent classification review: `structural-review.md`, `data-windows.json`;
no code/data or boundary contradiction found. Broad mixed tail data checked against
primary windows, consumers and uniform-run bounds; no exact subformat invented.
CHR PPU data reads do not create CPU execution claims. Scope/readability assessment:
`readability-assessment.json`; inventory/source locations: `commentary/review.json`
and `source-locations.json`. Source semantic limits: `commentary/uncertainties.md`.

## Required redirect metadata correction

Physical $0556..$0557 / CPU $8546..$8547 / fixed PRG [0,1,2,3]. Current label
HandicapCursorPositions and evidence "Two sprite coordinate values $44/$6A consumed
by $850B indexed load" are incorrect semantics. CPU $850B loads the byte, $850E
TAY selects it as a zero-page destination, and $8511 STA $0000,Y writes menuSelection
to $44 twoPlayerMode or $6A teeChoice. Source now calls it SetupChoiceDestinationBytes.

Make a metadata-only current-map/internal-state correction to SetupChoiceDestinations
and destination-address evidence. Keep Data, Static, offset1366/length2 unchanged;
no new runtime experiment or classification shift needed. Preserve this frozen
baseline and current source; carry review credit by unchanged physical identity.
The old annotation inconsistency must be reconciled before final acceptance.

No other concrete structural question requires redirect research. Natural callers,
full selector domains, original data subformats/physical units and exact third-sprite
meaning remain honest semantic limits, not skipped readability work. Prior natural/
artificial gameplay distinction remains intact. No full compatibility claim.

Final editorial_audit stays pending in review.json. No audit command was run;
reserve a targeted acceptance review and fresh verification for final acceptance.
No toolkit issues found (`analysis/toolkit-issues.md`). Configured binary used:
/root/romatlas/target/release/romatlas. No toolkit edits, archive, deletion/cleanup,
publication or commits. Free disk ~6.6 GiB; no new trace/capture scratch. All source,
ROM, assets and evidence remain local. This worker is finished; no writer remains.
