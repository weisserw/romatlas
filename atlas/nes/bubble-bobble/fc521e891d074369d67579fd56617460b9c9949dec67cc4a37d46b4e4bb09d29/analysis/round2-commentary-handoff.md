# Round2 commentary handoff — current-source readability complete

Classification unchanged:150757/163856 bytes=92.01%; Code20196, Data97793,
Image32768, Unknown13099. This completes current-source readability on pass-004;
it does not establish whole-ROM classification or final editorial/archive acceptance.

Covered all9798 scaffold entries:9465 Code instructions,177 Data regions,1 CHR
Image and155 explained Unknown gaps. Exact Code ownership totals20196 bytes across
610 mechanical routine/fragment units;153 specific data structure records give763
meaningful units. All293 prior unit names retained,77 affected summaries corrected.
No remaining source-review range. Exact assessment:
analysis/round2-commentary/assessment.json; full scaffold/context/source locations:
commentary/review.json. editorial_audit remains pending deliberately.

Prior commentary preserved ENTIRELY at analysis/round-2/commentary before fresh
pass-004 preparation. Applied prior sound names/comments by physical identities
and dependencies to the fresh baseline, preserving all prior unit names and branch
names where valid. Current source was rebuilt from fresh instructions, not copied
stale instruction bodies. Local source:commentary/source/game.asm; refreshed reading
copy:commentary/reading-after.asm. Local helper evidence is analysis/round2-commentary/.

Source now has1678 bank-aware routine/branch labels with shared/contextual RAM,
record-field, hardware and table aliases. New explanations cover finite sound/effect/
palette dispatch, caller-inline words and nonlocal stack exits, sound $FE duration/
$FF pointer commands, actor wall/pattern/diagonal map collision, six-digit decimal
score carry/saturation, player life/response, projectile capture/award chains,
three special projectile state machines and four-entry trail coordinate history,
boss hit palette/zone redraw and completed stage pointer carries. Data notes retain
bounded consumer formats and generic-data limits; Unknown gaps stay Unknown.

Fresh final verify passed ok=true, byte_identical=true:
commentary/verification.json. Final source SHA256:
530c7d0404b310ffba3ec0271547890ed590187da93e12e1a6604679632c8047.
Input SHA256 remains:
fc521e891d074369d67579fd56617460b9c9949dec67cc4a37d46b4e4bb09d29.
Current maps/current.json and analysis/state.json exactly equal the frozen baseline
and phase1-state. Reading copy refreshed after that verification.

Remaining meanings/contracts are localized in commentary/uncertainties.md. FA81
bank-shadow restoration, CEE3 task-stack/bank context, residual pointer domains
and bank10 consumers remain redirect work; original frontier/leads are preserved.
Exact actor/item/sound identities and some data subdivisions remain tentative;
mechanics are explained locally. No replay of completed boss/ending scenarios,
structural classification changes or terminal-exhaustion claim in this phase.

Toolkit:existing NES-001 remains; no new issue/blocker, no toolkit edits.
No final audit, archive/publication, cleanup/deletion, live server or goal completion.
Disk checked at handoff ~3.2GiB free; phase helper/evidence directory728KiB.
All ROM/source/assets/state remain local; only the platform's supported ROM-free
portable map is a sharing deliverable.
