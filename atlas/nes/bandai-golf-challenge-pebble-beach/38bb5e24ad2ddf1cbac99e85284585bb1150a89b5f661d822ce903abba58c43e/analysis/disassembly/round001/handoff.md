# Round 001 disassembly handoff

Phase complete: one full disassembly pass. Structural classification complete;
overall workflow remains incomplete pending independent commentary/readability,
classification review and final editorial gates. No toolkit issues found.

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

## Checkpoints and verification

Input: initialized workspace, no baseline/state classifications, empty lead list.
Established baseline once: `maps/baseline.json`, `generated/baseline`,
`analysis/baseline-verification.json` (byte identical).
Intermediate verified checkpoints `pass-001-structural` (42215 classified) and
`pass-001-classified` (65552 classified) preserved.
Output: `analysis/state.json`, `maps/current.json`, `maps/pass-001-final.json`,
`generated/pass-001-final`, `analysis/pass-001-final-verification.json`.
Final map.validate passed; generated source reassembled byte identically.
Map SHA256 `401371b3a2cf4bc85c43e2696f369e3fefa76e0e853acccd920a0e82aed52db9`.
ROM SHA256 unchanged from workspace identity
`38bb5e24ad2ddf1cbac99e85284585bb1150a89b5f661d822ce903abba58c43e`.
Configured binary used throughout: `/root/romatlas/target/release/romatlas`.
No toolkit edits, archive, cleanup, publication or commits performed.

84 internal regions: Code 9478, Data 23306, Image 32768; every byte has evidence.
All structural inference uses Static provenance. Actual accepted fetch evidence is
recorded separately and never manufactured from pointers or debugger edits.

| Physical unit | Code | Data | Image | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 0 | 0 |
| PRG 0 | 5598 | 2594 | 0 | 0 |
| PRG 1 | 3880 | 4312 | 0 | 0 |
| PRG 2 | 0 | 8192 | 0 | 0 |
| PRG 3 | 0 | 8192 | 0 | 0 |
| CHR 0 | 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 0 | 8192 | 0 |
| CHR 2 | 0 | 0 | 8192 | 0 |
| CHR 3 | 0 | 0 | 8192 | 0 |

PRG/CHR units are physical 8 KiB units; file offsets include 16-byte header.
Fixed CPU PRG context `[0,1,2,3]`; CHR banking is separate. CNROM mapper3.
`bank-counts.json` supplies exact physical intervals.

## Structural findings and evidence

- NMI/reset/IRQ validated as $8436/$83D3/$8A77. Discoveries preserve fixed bank
  context and exact physical byte identity.
- Dispatcher $8D31 consumes JSR return address and indexes inline target words,
  then tail jumps. Ten exact inline table barriers prevent false fallthrough.
  `dispatch-evidence.md`, `discovery-barriers-*.json`, `candidate-listing.txt`.
- Terrain dispatch $AC97..$ACB2 uses 19 target words at $ACB5..$ACDA;
  supported connected handlers classified, `batch2-listing.txt`.
- Intentional alternate instruction entries within operand bytes occur at
  $87D5/$87D6, $8CDF/$8CE0 and $A040/$A041. Preserve these overlapping flows;
  a single assembly decoding stream is not a claim that alternate entry is dead.
- Every data gap inspected for available references/consumers and patterns.
  `data-inspection.md` and per-region evidence document exact ranges. Largest
  combined data interval physical $3261..$8009 contains sound tables/scripts,
  compressed course/UI data, lookup records and uniform runs; combined Data
  annotation deliberately does not assert one exact subformat across that span.
  CHR bytes classified Image from iNES extent and NES pattern-bank role.
- Data pointers at $9CF1/$9CFF feed sprite drawing $8BD6; they are not code roots.
  Standalone $8B2B..$8B2F display-mask helper has independently coherent contract;
  its natural caller is unresolved.

## Gameplay and observation

Title identifies Bandai Golf: Challenge Pebble Beach, Bandai1988. Sources and
controls in `analysis/gameplay.md`; tested routes supersede the FAQ's differing
A/B naming. `analysis/scenarios.json` is the progression checklist.
`actions.jsonl` records button press/release, frame budgets and actual resulting
states; `patches.jsonl` plus scripts record exact artificial changes.

Natural visits: title/player/tee/handicap menus; initial scorecard; hole1 overview;
club changes; golfer and power meter. Manual swing timing was attempted, then
superseded by understood state/parameter edits rather than repeated indefinitely.
Artificial visits confirmed by frames and accepted execution:
- Hole9 and hole18, with distinct coastal layouts, course and club screens.
  Restore `course1.snapshot.json`, release buttons; patch $61=8/17,
  $2E=2,$2F=0; idle55,A3,idle40,A3,idle25. Snapshots `hole9-*.snapshot.json`,
  `hole18-*.snapshot.json`; captures `.png`/`.ppm`.
- Full strike/ball physics: `physics.py` replay sets $2E=5,$2F=8,$30=2,
  $5B..$5F=[24,24,0,24,16],$60=0, then release2,A3,release30.
  Actual roots $A136/$8951/$A755 confirmed; final main state8.
- Putting: near-cup player block from target $75..$77=[122,1,54],
  $0216..$021D=[13,0,118,1,0,54,0,0],$2E=3,$2F=0; idle35,A3,idle35.
  Naturally resolved terrain $06CD=$48 and club13; `putting-club.png` shows
  zoomed green and CLUB PT. Set phase2 and meter parameters16, then
  release3,A3,release35; actual $A136/$8951 fetches and main8 prove putt motion.
- Terminal: `terminal.py` restores hole18-club; $61=17,$0200=$80,$0240=$80,
  $2E=9,$2F=5; release2,A3,release55. Supported handler increments hole17->18,
  shows final scorecard (main7/sub7), then Start3/release30 returns title.
  `terminal-scorecard.png`, `terminal-return.png` and durable snapshots.
  Artificial scores do not represent a naturally completed 18-hole round.

Selected saved contexts union: 3053 distinct physical ROM instruction starts;
none lies outside classified Code (`execution-map-audit.json`). This mixes natural
and artificial scenarios and is not whole-game execution coverage. Startup alone
reported850 starts. Root pages and per-scenario summaries retain provenance;
zero detail capacity intentionally drops detail events while aggregate_dropped=0.
Initial all-event detail capacity2000 dropped3,413,842 detail events; summary
retained850 startup roots. No natural reachability claim follows from edited runs.

Snapshots are local same-build emulator snapshots and include ROM-backed state;
never share them as portable maps. Session-local names are not needed: persistent
files are available. New serve session uses map.load then emulator.start and
snapshot.import, followed by explicit intended input. Host tool processes expired
between some batches; interrupted attempts were recorded, not counted as visits.

## Leads and remaining work

No input leads consumed. Produced/verified R001-D01, R001-D02, R001-G01,
R001-G03; R001-G02 manual timing superseded. Full outcomes and retry conditions
in `analysis/leads.json`. No unknown physical ranges remain.

Next: independent commentary worker should review every accepted boundary/role,
including broad generic data and overlapping entries, and provide readable routine,
branch, RAM and data explanations across all current source. Semantic table
subdivision, original music/UI formats and some RAM meanings remain uncertain;
these are semantic/editorial questions, not hidden unknown bytes. Natural callers
and full selector domains remain separate open reachability questions.

No boss system identified. Bunker/water/out-of-bounds and two-player permutations
are deferred: representative swing, later coastal holes, green putting and terminal
flow were visited; exhaustive gameplay permutations are not needed for this pass.
No separate credits screen identified; final scorecard/title return is confirmed.
No full hardware/game compatibility or gameplay-fidelity claim.

Toolkit report: `analysis/toolkit-issues.md` — no toolkit issues found; no blockers.
Free disk at handoff about6.6 GiB; scratch76 KiB before final report writes.
Durable evidence about67 MiB before final putt/context saves. No files pruned.
