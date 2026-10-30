# Round01 commentary handoff

Current-source readability complete; ROM classification incomplete and unchanged.
All 636 scaffold intervals have reviewed readability notes: 478 Code fragments
(11,629 bytes), 29 Data regions, one CHR region and 128 explained unknown intervals.
193 purpose/mechanics units and 852 entry/branch symbols cover all 5,523 classified
instructions. Shared hardware, controller, object-field, resource/buffer and audio
symbols are used in operands. Reused scratch storage remains numeric where no
single stable role is supported. No classified-source readability frontier remains.

Examples include channel-owning audio streams, controller held/edge semantics,
country selector, player/falcon movement, fixed-point position/collision helpers,
metasprite/OAM construction, map/metatile/attribute rendering, inline pointer and
dispatch stack contracts, NMI tasks/banking, and altar order validation.
Generic data and unknown intervals have honest boundary/role notes; this does not
promote any structural classification or claim original names/game intent.

Authoritative edited source: [game.asm](../commentary/source/game.asm).
Refreshed [reading copy](../commentary/reading-after.asm).
Fresh commentary [verification](../commentary/verification.json): byte_identical=true,
source SHA256 d0e1fb0cdc7d722a499e4863e8b347c616d5eeb35b13e7c2836ced65d3dcf102.
[Review inventory](../commentary/review.json),
[scope reconciliation](round01-commentary/scope-check.json),
[progress](../commentary/progress.md).
Final editorial audit remains pending and was not run.

Fresh structural checkpoint [verification](round01-commentary-verification.json)
and [frozen map](../maps/round01-commentary.json) validate/rebuild the unchanged
phase-1 map. Its canonical generated source is separate from the edited commentary
source. `analysis/state.json` and `maps/current.json` match the frozen commentary
baseline. Classification: 227971/262160 (86.96%), 34189 unknown; code11629,
data85270, CHR131072. See [per-bank totals](round01/per-bank.md) and
[unknown complement](round01/unknown-ranges.json).

Concrete questions and source contracts:
[uncertainties/redirect](../commentary/uncertainties.md).
R01-CALLS should continue past returning helpers with proper bank context,
distinguishing inline pointer words from inline tail-dispatch tables.
R01-ENDING is narrowed: F85A is a known countdown entry, and A=$3C means 60
NMI decrements. Success continuation at physical120215..120271 and scene prefix
120531..120539 remain unknown. Inspect actual return/stack/NMI mode and sprite-zero
polling before another scene probe; do not claim the previous stall's cause.
R01-ROOMS still needs a supported boss/controller/object initialization contract.
R01-DATA now has documented consumers for room records, descriptors, frame tables,
metatiles, palettes and audio streams; consumer semantics do not alone prove extents.

No emulator run or new gameplay evidence in this phase. Previous artificial and
natural scenario distinctions remain intact. No new toolkit issues found; persistent
report has one confirmed API/documentation bug NES-001, zero suspected bugs or
missing features, no blockers: [toolkit issues](toolkit-issues.md).
Input/toolkit unchanged. Approximately11GiB free at handoff. No cleanup, deletion,
archive or publication performed. Continue redirect; this is not terminal exhaustion
or whole-ROM/editorial acceptance.
