# Commentary progress — round 01

Complete readability pass over the frozen pass-001-final source: 15706 code bytes
(7527 instruction regions), 4586 non-image data bytes and 131072 CHR bytes.
Every current classified instruction was inspected using a compact physical/CPU
listing; 350 curated routine/control-block explanations cover the full code inventory.
Curated interpretations are retained in research/round01-units.txt. Source has 1194
routine/branch entries plus named inline target tables, mode-specific RAM/object
fields, hardware registers and controller/OAM constants. Forced operand widths and
explicit per-instruction execution addresses are retained.

Examples include E116's return-address-consuming inline dispatch, audio ownership
and software decimal arithmetic, brawl terrain/height integration and attack slots,
shooting hit rectangles, driving fixed-point projection/steering/ballistic shots,
PPU packet/RLE formats, palette steps, MMC1 bank-stack wrappers, NMI and raster timing.
Every unknown gap has an honest boundary note. All 7821 scaffold records have review
summaries and source locations; code records link their reviewed logical units.
Unit spans may include unknown gaps, explicitly without classifying them.

No current-source readability ranges remain deferred. Unresolved meanings and the
bank10 static-prefix discrepancy are in uncertainties.md; no structural map edits
were made. Classification remains incomplete at 57.74% (151364/262160), separate
from readability. Existing phase-1 evidence, maps and frozen source were preserved.

The first transformed build exposed misplaced label locations before .org; corrected
labels now follow the original CPU origin. Subsequent byte-identical rebuilds passed.
Final verification and refreshed reading copy are recorded in verification.json and
reading-after.asm. Final editorial audit remains pending by phase contract; it was
not invoked as a substitute for review. No toolkit changes or new toolkit defects.
