# Disassembly round01 handoff

Single assigned pass complete; whole-ROM classification incomplete. Next: full current-source commentary, then redirect on leads.json.

ROM classification: 31.93% (41854 / 131088 bytes classified; 89234 bytes unknown).

Code9625; data32229. Header16 classified separately from PRG banks. Exact per-bank counts: [counts](evidence/round01-final-counts.json), [progress](progress.md). Unknown intervals: [ranges](evidence/round01-unknown-ranges.json).

Checkpoint `pass-001-final`: [verification](pass-001-final-verification.json) ok=true/byte_identical=true; map `maps/pass-001-final.json` SHA256 `9307713dd31e9c9a4e42b03c88e433207a4620942ae1194ae93a6de4a710cb57`; source `generated/pass-001-final/game.asm`. Current state `analysis/state.json` and map `maps/current.json` validated/saved. ROM unchanged. Fidelity and readability are separate; source commentary/editorial review remains.

Main results: observed startup/gameplay code with exact bank views; coherent alternate paths Static; header, FF tails and trailer/vector data;17 consumer-bounded graphics streams;13 fixed1024-byte area tile streams;12 area descriptors/header/metatile/attribute records and supported transition-array prefixes. Strong evidence: [graphics](evidence/graphics-parse-proposals.json), [area tiles](evidence/stage-graphics-proposals.json), [area headers](evidence/area-header-proposals.json). No full fourth transition-array extent or original generator claims.

Gameplay: title/menu/names/Greece map, Arcadia movement/jump/attack/inventory; valid areas1/4/5/8/11 with representative actions/enemies; rescue/final-couple terminal scene. Source-informed artificial edits shorten title/fades and invoke supported initialization contracts; natural boss defeat/ending unproven. Named boss identity, actual NPC dialogue, special travel/water/ceiling mechanics and remaining areas remain open. Two house coordinate/Up/B tests yielded no dialogue/new code; do not repeat unchanged. Full routes/budgets/patches/snapshots and checklist: [scenarios](scenarios.json), research/version limits: [gameplay](gameplay.md).

Concrete redirect questions: room-column/actor bodies from CFBC pointers; bounded inline D7A2 dispatch/object handlers; NPC/door/quest/boss contracts; bank4 audio grammar; residual bank3 F431..FE2E and mirrored reset-stub mapping contracts. These need research before ready proposals; see R01-L01..05 in [leads](leads.json). No global terminal exhaustion claimed.

Toolkit: one confirmed documentation/API-validation bug NES-DOC-001, no suspected/missing-feature findings or blockers: [report](toolkit-issues.md). NES input uses `buttons:[...]`/`buttons:[]`, port1; memory writes use `address`, not `offset`. Configured binary retained, no toolkit edits. Commit3bb1dda0d069721ed9ad44088f172dbd01ae5216.

Resume with batch-local persistent `python3 run.py serve`, map.load current state, emulator.start then snapshot.import chosen durable snapshot; explicitly reassert buttons. No live server needed; old bridge sessions ended143. `scratch/batch.py`/named scenario scripts retained. Keep snapshots local (contain ROM). Disk4.4GiB free; evidence211MiB, scratch304KiB; no deletion/archive/publish/commit.

Retention note: second static capture reused canonical review filenames; accepted first42 facts redecoded from retained state/unchanged ROM and combined with second111 candidates. [Recovered accepted review](evidence/all-static-initial-accepted-review.json), [second frozen review](evidence/all-static-round01b-review.json). Investigation ledger preserves first819-root/70-view totals and scope. The round01a-named files are second-capture copies, not first originals. Do not infer new runtime coverage from reconstructed static review.
