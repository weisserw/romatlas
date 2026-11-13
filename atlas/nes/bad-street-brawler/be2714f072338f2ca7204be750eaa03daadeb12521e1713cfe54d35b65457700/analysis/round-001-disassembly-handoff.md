# Round 001 disassembly handoff

Status: single pass finished; ROM incomplete. Next phase commentary, then redirect. No live process or unfinished same-phase experiment.

ROM classification: 12.98% (17021 / 131088 bytes classified; 114067 bytes unknown).

10829 code, 6192 data (includes16-byte header); code provenance9692 Observed /1137 Static. Observed includes artificial later-stage/ending experiments and does not imply natural reachability. 5071 regions have evidence; labels and readable routine commentary are unfinished.

Authority: [state](state.json), [portable map](../maps/current.json), [verified checkpoint](pass-001-verification.json), [source](../generated/pass-001/game.asm). Byte-identical=true. ROM unchanged; checkpoint map SHA256 de7e76a6020c20ccef358832942b43f6196edf82499e8ab77dcbddcd0bc83945. [Exact bank counts](round-001/classification-counts.json), [unknown complement](round-001/unknown-complement.json), [progress](progress.md).

Game: Bad Street Brawler, probable USA, Beam/Mattel1989. [Gameplay research](gameplay.md), [scenarios](scenarios.json). Natural menu/stage1 practice and fighting observed. Shared input documentation was misleading: NES requires `buttons:[...]`, release `buttons:[]`; earlier button/pressed attempts were ignored and count as title-only execution. See confirmed [NES-001](toolkit-issues.md). No toolkit edits; configured release commit a942a38.

Artificial progression: restore `round-001/fight-action.snapshot.json`, release input, operation1 to boundary, write RAM22=4/9/14, set PC D014 and stack FF, effective PRG [12,13,14,15]. Initialization refreshes dependent state. Stage5/10/15 practice and active fights visually confirmed; after intro240f untraced, right+A50f/up+B30f traced. [Driver](round-001/later-action.py), [stage15 active](round-001/stage15-active.ppm.png). Stage6/12 special bosses remain deferred pending concrete spawn/position contract.

Ending: stage15-active snapshot, align operation, set PC D255 / stack FF; valid stage22=14 selects D27B[14]=3 -> AA1D. [Driver](round-001/ending.py), [ending frame](round-001/ending-entry.ppm.png), [later frame](round-001/ending-later.ppm.png), START exits. Artificial post-stage entry skips completion/scoring; natural ending reachability unproved.

Structural gains: accepted contextual fetches decoded in exact views; bounded vector paths; static root closure +808 then112 bytes; uniform runs >=64 classified generic Data after boundary/conflict inspection; text/glyph consumers prove menu/quote/name and digit-display tables. [Decoded observations](round-001/reviewed-observed-decodes.json), [static closure](round-001/static-closure-candidates.json), [uniform review](round-001/uniform-run-review.json), [text proof](round-001/text-structure-review.json), [connected text tables](round-001/classify-text-tables.py). Boundary refinements in `fix-text-boundaries.py` supersede initial text-review extents; current map is authoritative.

[Leads](leads.json): INPUT/PROGRESSION/ENDING verified, no unchanged retries needed. STREAMS: RAM16/17 from physical1804C, investigate D881/D7A6/DF8D consumers for finite stream records/bounds. ASSETS: trace AA1D screen loads/DCD1 bank transfer data reads to establish graphics/non-code roles across residual banks0..11; bank0 also contains code. BOSS-CLEAR: stage6/12 spawn/world-state and B1 clear flag consumers D22B/D238. These are connected research opportunities for redirect, not terminal exhaustion. Representative gameplay and readily grounded static continuations have ceased yielding substantial easy gains.

Toolkit report: one confirmed docs/validation bug, zero suspected bugs/missing features; workaround validated, no blocker. About6.6GiB free, scratch24KiB. Snapshots/source/evidence remain local; no publication/archive/cleanup.
