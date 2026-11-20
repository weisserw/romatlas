# Round 1 commentary handoff

Round=1; phase=commentary; status=completed full current-source readability. Next coordinator phase is separate redirect. No redirect, archive, deletion, publishing or toolkit edits performed. ROM and phase-1 classification unchanged.

Covered all 146,708 classified bytes: 9,323 code, 137,385 data/text. All 417 scaffold entries retained and reviewed, including boundary explanations for 246,524 unknown bytes. 216 compact routine/handler units, 351 control-flow labels and consistent RAM/I/O/table/field names. No current-source readability gaps remain. Final editorial_audit remains pending for final acceptance.

Buildable local source: commentary/source/game.asm and symbols.inc; refreshed reading copy commentary/reading-after.asm. Frozen before/, baseline.json and phase1-state.json preserved. Fresh byte-identical verification commentary/verification.json, ROM SHA256 f442bde3a19e6070fd55a7f025d1fbf463a43a960a62f4c44d6fed19fcd9c6d1. Final game.asm hash 7d5882aae5b3ffce40e3a269fdd7078eb996bf0d6f6173b4e7effb219ac85e00; symbols.inc 9dde331a43b43b4bd082b05b12ba5f551c81f08e6d98044c8f3a89757ad7cf63.

Examples: movement commands and five-byte frames; graphics queue/DMA format and resource flags; ten negative 24-bit job delays; text escape parser and character translation; fifteen sound sequence slots, six-byte events, 50-byte packet ring and four APU channels; mapper/NMI context; speech selector directory and packed descriptor limits. Pointer-setter $C73B and command-string printer $C740 are separately named with correct callers. Comments retain physical effective-bank context and artificial-observation limitations.

Unresolved meanings and bounded redirect questions: commentary/uncertainties.md. Prior leads remain in analysis/leads.json: speech packed sample extents/CHR consumer, actual six-game continuation contract, mixed graphics/script bounds, final probe static continuation. Generic uniform data remains generic; table consumer aliases do not classify unknown backing bytes. No claim of natural reachability follows from artificial selector/timer observations.

Classification remains 37.31% (146708/393232), separate from readability. Structural checkpoint remains maps/round-001-disassembly-handoff.json and generated/round-001-disassembly-handoff; commentary has its independent edited source. No toolkit issues found; analysis/toolkit-issues.md updated. All ROM/source/assets/traces/internal state remain local.
