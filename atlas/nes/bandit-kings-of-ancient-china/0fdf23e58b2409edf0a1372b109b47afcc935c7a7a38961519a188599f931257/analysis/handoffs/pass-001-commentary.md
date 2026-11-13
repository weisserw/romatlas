# pass-001 commentary handoff

Current-source readability complete; ROM structurally incomplete. Final editorial
acceptance remains pending. No audit invoked, no redirect/archival/publication performed.

Baseline: pass-001-extended, frozen commentary/baseline.json and phase1-state.json.
Current map/state unchanged; original input SHA-256 unchanged. Classified scope:
3,358 code, 46,282 data, 14,162 text, 131,072 CHR image bytes (194874/393232, 49.56%).
All 732 review scaffold intervals reviewed: 513 classified, 219 explained unknown.
676 reviewed compact units; no prior review credit existed. Exact unfinished
readability ranges: none. Exact unknown complement remains the disassembly record;
unknown bytes are opaque source spans, not newly classified data.

Local outputs: ../../commentary/source/game.asm, ../../commentary/reading-after.asm,
../../commentary/review.json, ../../commentary/progress.md,
../../commentary/uncertainties.md. Every classified opcode-table target has an
entry label. Routine contracts, branches, VM RAM pairs, constants, transfer fields,
text previews, uniform data and 2bpp tile banks covered. CPU aliases for raw-storage
tables are explicit constants, distinct from physical source labels.

Examples: EnterScriptFrame saves caller context and consumes inline displacement;
VmDivideUnsigned restores remainder through sixteen shifts; signed comparisons
return boolean words; TransferRowsToPpu documents budget, copy/fill and stride;
SetPrgBankWithNmiGuard describes register selectors without inventing physical banks.

Fresh final commentary verify passed byte-identical; reading copy refreshed afterward.
Source SHA-256 0aac66b70f93c089a3cc592cab01c12b2c16a3737244f3a6e431724c6096fdd8.
Inventory partition/units and frozen map/state/input hash checks passed. No cleanup.

Remaining meanings: VM values have no recovered game type; counter random purpose
is tentative; text slices do not imply adjacent full format objects. L002–L004 remain
productive redirect questions: PRG mode/audio banking and restoration, bounded VM
script objects, extended selector table extent and native helper contracts. See
uncertainties.md and existing leads.json/investigation.md; no terminal rejection.

All evidence is Static, observed execution zero. Mapper5 runtime unavailable.
Toolkit findings unchanged: 1 missing feature NES-M5-001, 0 confirmed bugs,
0 suspected bugs; runtime-only blocker, static work unblocked. See ../toolkit-issues.md.
Source/assets/internal state stay local. The ROM is not complete merely because
readability and byte-identical reconstruction passed.
