# Current status — round 001 disassembly complete

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

Structural classification is complete. Overall workflow remains incomplete pending
independent commentary/current-source readability, classification review and final
editorial gates. Final state/map/checkpoint verified byte identical:
`analysis/state.json`, `maps/current.json`, `maps/pass-001-final.json`,
`generated/pass-001-final`, `analysis/pass-001-final-verification.json`.
Final map SHA256: `401371b3a2cf4bc85c43e2696f369e3fefa76e0e853acccd920a0e82aed52db9`.
ROM unchanged. 84 annotated regions: 9478 Code,23306 Data,32768 Image.
No raw fallback or unknown gaps counted. Evidence roles/provenance are Static;
actual accepted instruction-fetch evidence and natural/artificial status remain separate.

| Physical unit | Code | Data | Image | Classified | Unknown |
|---|---:|---:|---:|---:|---:|
| Header | 0 | 16 | 0 | 16 | 0 |
| PRG0 | 5598 | 2594 | 0 | 8192 | 0 |
| PRG1 | 3880 | 4312 | 0 | 8192 | 0 |
| PRG2 | 0 | 8192 | 0 | 8192 | 0 |
| PRG3 | 0 | 8192 | 0 | 8192 | 0 |
| CHR0 | 0 | 0 | 8192 | 8192 | 0 |
| CHR1 | 0 | 0 | 8192 | 8192 | 0 |
| CHR2 | 0 | 0 | 8192 | 8192 | 0 |
| CHR3 | 0 | 0 | 8192 | 8192 | 0 |

Physical banks are8 KiB; exact intervals in `analysis/disassembly/round001/bank-counts.json`.
Fixed CPU PRG context[0,1,2,3]; CHR banking is separate.

Broad progression fulfilled with representative visits: natural setup/hole1/club and
swing meter, artificial full strike/physics, later holes9/18, zoomed putting green/PT
and actual putt motion, final scorecard and Start return to title. Scores from patches
are artificial. Exact routes/patches/results/evidence in `analysis/scenarios.json` and
`analysis/disassembly/round001/handoff.md`. Saved contextual roots union3053 distinct
ROM starts; none outside Code. This is not whole-game execution coverage.

No initial leads; verified R001-D01,D02,G01,G03; G02 manual timing superseded.
Remaining work: independent classification audit and readable explanations across
current source; data subformats, several RAM meanings, some caller/selector
reachability unresolved. No unknown byte ranges remain. No toolkit issues found;
see `analysis/toolkit-issues.md`. No toolkit modification, publishing, archive or
cleanup performed. No full compatibility claim.

Detailed handoff: `analysis/disassembly/round001/handoff.md`.

# Historical round 001 progress (preserved)

# Disassembly progress

Round 001 disassembly in progress. Baseline established and byte-identical verified
(`analysis/baseline-verification.json`, `generated/baseline`, `maps/baseline.json`).
Configured binary `/root/romatlas/target/release/romatlas`; mapper 3 CNROM,
physical PRG 0x10..0x800f, CHR 0x8010..0x1000f, fixed PRG context [0,1,2,3].
Emulated title identifies Bandai Golf: Challenge Pebble Beach (Bandai 1988).
Startup at frame 44 observed 850 instruction starts; static/observed vector-root
candidate discovery found 1289 instructions, pending inspection/classification.
No annotations yet. ROM classification not yet calculated; pass not complete.
Persistent RPC subprocess is controlled by analysis/scratch/rpc.py and rpcd.py;
use current process only while live; snapshots must be exported before handoff.
ROM unchanged. Broad gameplay progression and bounded static batches continuing.

## Round 001 durable structural batch

Natural setup through frame 272 reached complete 18-hole scorecard after one-player,
regular-tee and handicap-zero selection. `analysis/disassembly/round001/actions.jsonl`
records actions from frame 206 onward; `hole1.snapshot.json` is actually scorecard
state (historical filename retained). Earlier route: title idle to frame44; Start3,
release60; Start3/release30; B3/release30; A3/release30; A3/release30;
A3/release30. No direct RAM edits yet.

Identified $8D31 inline return-address dispatch; ten exact pointer-table barriers
now annotated, alongside header, interrupt vectors and four CHR pattern banks.
See `analysis/disassembly/round001/dispatch-evidence.md`. These are structural
Static classifications; natural selector coverage remains distinct. Saved state
and current map now exist. Barrier-aware connected static discovery is running;
its candidates remain hypotheses until inspected. No completion claim.

## Verified structural checkpoint

`pass-001-structural` validated and rebuilt byte identically; report
`analysis/pass-001-structural-verification.json`. Classified internal regions:
{'Data': 194, 'Image': 32768, 'Code': 9253}.
ROM classification: 64.40% (42215 / 65552 bytes classified; 23337 bytes unknown).

38 connected code ranges accepted after table barriers and official-opcode/control-flow
review, including intentional overlapping operand-entry tricks. Current code remains
Static provenance; startup fetch evidence is retained separately. Broad progression
continues from scorecard toward shot states and later-hole RAM shortcut.

## Whole-ROM structural checkpoint (gameplay work continues)

`pass-001-classified` validates and rebuilds byte identically. Internal map counts
{'Data': 23306, 'Image': 32768, 'Code': 9478}.
ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

`analysis/disassembly/round001/data-inspection.md` records pattern/consumer/boundary
review of every non-code interval. Terrain-response indirect handler graph adds
220 code bytes; display-mask helper adds five. Exact structural coverage is now
complete, while gameplay milestone exploration and later commentary/editorial gates
are still pending. Natural hole1 overview and club selection visited; up changes club.
Swing timing sequence was interrupted by tool-process lifetime before flight capture;
restart from durable shot-menu snapshot and replay, not counted as ball-flight visit.
No toolkit issues found; process lifetime is host orchestration, not toolkit defect.

Broad progression: artificial supported hole-loader visited hole9 and hole18,
including distinct course layouts and club screens, using $61=8/17,$2E=2,$2F=0
from clean course1 snapshot. Durable snapshots/captures saved for both. Exact
patches/actions linked from scenarios.json. Completion-handler test initialized
$61=17,$0200=$80,$0240=$80,$2E=9,$2F=5 and A, but host process lifetime
interrupted final idle/capture; terminal attempt must resume from clean snapshot.

## Coordinator phase transition: round 001 commentary

Disassembly handoff accepted at `analysis/disassembly/round001/handoff.md`;
worker stopped before commentary writer starts. Input checkpoint for commentary:
`pass-001-final` (`analysis/pass-001-final-verification.json`, byte identical),
map SHA256 `401371b3a2cf4bc85c43e2696f369e3fefa76e0e853acccd920a0e82aed52db9`.
ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).
Coordinator printed this percentage before commentary. Leads produced/verified:
R001-D01, R001-D02, R001-G01, R001-G03; R001-G02 superseded.
Current phase: commentary, full current-source readability and independent
classification review pending. Structural coverage alone is not final acceptance.

## Round 001 commentary complete

Full current-source readability complete, separately from classification/final
acceptance. `commentary/review.json`: all 84 regions and 261 units reviewed;
9478 Code, 23306 Data, 32768 Image bytes, no deferred readability ranges.
`commentary/source/game.asm` and `symbols.inc` explain routines, meaningful branches,
RAM/player/sound fields, constants, every data/CHR block and all overlapping entries.
Independent structural review found no code/data or physical boundary contradiction;
see `analysis/commentary/round001/structural-review.md` and `data-windows.json`.

`commentary/verification.json` and frozen phase copy
`analysis/commentary/round001/verification.json` confirm a fresh byte-identical
reassembly; `commentary/reading-after.asm` refreshed. Current map validation passed.
ROM, current map/state and frozen commentary baseline unchanged. No toolkit issues
found; no toolkit changes or archive/publication/cleanup performed.

Redirect metadata correction required before final acceptance: physical $0556..$0557
has $44/$6A zero-page setup destinations, consumed by CPU $850B/$850E/$8511;
current HandicapCursorPositions label and coordinate evidence are misleading.
Readable source corrects the meaning. Data/Static/two-byte bounds remain valid.
Exact correction and other unresolved meanings are in commentary/uncertainties.md.
Final targeted editorial audit remains pending; no audit command run.
Compact phase handoff: analysis/commentary/round001/handoff.md.

## Coordinator phase transition: round 001 redirect

Commentary handoff accepted: `analysis/commentary/round001/handoff.md`.
All 84 regions and 261 units reviewed; fresh verification hashes match current
source. No unread/deferred readability range. `commentary/reading-after.asm`
refreshed; final editorial acceptance intentionally pending.
Current structural checkpoint remains `pass-001-final`, zero unknown bytes.
Required metadata defect at physical $0556..$0557 is source-corrected but remains
in current map/state. Redirect must preserve its exact primary evidence and route
an actionable correction to disassembly; redirect does not edit classification.
Current phase: redirect. Commentary writer stopped before next worker starts.


## Round001 redirect attempt001

Input pass-001-final,0 unknown; map/state/source/ROM unchanged (preservation-check.json).
Consumer-based setup destination correction R001-R01 and bounded sound Data-schema
refinement R001-R02 ready together for independent disassembly;0 new classified bytes.
Connected sound stream paths and independent semantic avenues investigated; actual
limits and reusable static check in redirect/round001-attempt001/findings.md.
Handoff: redirect/round001-attempt001/handoff.md. No final acceptance claimed.
No toolkit issues found.

## Coordinator phase transition: round 002 disassembly

Accepted productive redirect handoff:
`analysis/redirect/round001-attempt001/handoff.md`. Inspected connected setup/sound
findings and independent avenue dispositions. Ready leads R001-R01 and R001-R02
provide exact primary evidence and bounded verification recipes for metadata/schema
refinement, not new coverage. No terminal claim; zero unknown complement retained.
Input remains pass-001-final with current readable commentary preserved.
Current phase: round 002 disassembly; prior redirect writer finished. Reuse sound
progression evidence and avoid replay without changed question. Apply only independently
supported batch, checkpoint and return to current-source commentary.

## Round002 disassembly completed

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).
Counts9478Code/23306Data/32768Image,88regions; zero new classified bytes.
Per-bank: header16Data; PRG0 5598Code/2594Data; PRG1 3880Code/4312Data;
PRG2/3 each8192Data; CHR0..3 each8192Image; each0unknown.
R001-R01 verified corrected setup destination metadata. R001-R02 verified940 existing
Data bytes as12period words,22descriptors,22bounded streams; final FF3664 included.
Unsupported gaps generic Data. Exact review/evidence/per-bank counts in
analysis/disassembly/round002; handoff.md identifies output pass-002-refined.
Map validation and fresh byte-identical checkpoint passed; map hash
ff10526069bb622c0e919dc445090ae1f0a08f25b67aefde0bde2018d6a3a356.
ROM/current commentary/frozen history preserved; prior gameplay evidence reused.
Next commentary must reconcile physical identities and new88region layout; no final
acceptance claimed. No toolkit issues found; analysis/toolkit-issues.md unchanged.

## Coordinator phase transition: round 002 commentary

Accepted independent disassembly handoff `analysis/disassembly/round002/handoff.md`.
Output `pass-002-refined` validates/rebuilds byte identically; authoritative map hash
ff10526069bb622c0e919dc445090ae1f0a08f25b67aefde0bde2018d6a3a356.
Both R001-R01/R001-R02 verified; counts unchanged,88 regions,0 unknown.
ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).
Percentage printed before commentary. Prior commentary preserved unchanged; next
worker must preserve full prior round before preparation and reconcile by physical
identity, retaining sound review credit. Current phase: round002 commentary.

## Coordinator phase transition: round002 redirect

Accepted `analysis/commentary/round002/handoff.md`:88 regions/287 units reviewed,
no readability gap; source hashes match fresh byte-identical verification.
Current commentary baseline matches pass-002-refined and entire prior commentary
preserved under analysis/round-001/commentary. Setup metadata inconsistency resolved.
Current phase round002 redirect; commentary writer finished. Final targeted editorial
acceptance/audit remains pending even if redirect finds no further ready lead.

## Round002 redirect — round002-attempt001

Input pass-002-refined (ff10526069bb622c0e919dc445090ae1f0a08f25b67aefde0bde2018d6a3a356),
88-region/287-unit current commentary gate accepted. Exact empty unknown complement
refreshed. Narrowed prior sound parameter-domain deferral using22 bounded streams,
consumer masks/phase widths, biased-base overlap and descriptor endpoint. R002-R01
ready:88 existing Data bytes, connected mask/vibrato/envelope schemas plus correction
of eight-step phase comment to sixteen-phase. Other unchanged independent avenues
and broad gameplay reviewed without repetitive experiments. Combined productive
handoff analysis/redirect/round002-attempt001/handoff.md; no structural/source state
changes; final editorial acceptance pending.

## Coordinator phase transition: round003 disassembly

Read round002 redirect handoff/findings and accepted coherent ready R002-R01 for
independent review. Localized corrected inference combines finite sound selectors,
masked consumer widths, representation and adjacent endpoints for88 existing Data
bytes; envelope comment correction is concrete. Prior broad deferral was not a veto.
No terminal claim. Input pass-002-refined; zero unknown; current readability preserved.
Current phase round003 disassembly; redirect writer finished before next starts.

## Round003 disassembly

R002-R01 verified and applied via supported map API: existing88-byte Data/Static
physical3279..32D0 split into channel enable masks4,capability flags4,vibrato48,
nonzero envelope amplitudes32. Fixed PRG[0,1,2,3]. Independent review/checker:
disassembly/round003/review.md and parameter-verification.json. Encoded selector
domain distinguished from unrestricted engine possibilities; no new fetch/natural
selection claim. Biased envelope baseB291 overlaps third vibrato bank; ID0 bypasses.
Four phase-loop iterations establish16 phases; source eight-step correction handed
to commentary. Source and frozen histories preserved (preservation-check.json).

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

9478Code/23306Data/32768Image,91regions;0 new classified bytes.
Per-bank Code/Data/Image/unknown: Header0/16/0/0; PRG0 5598/2594/0/0;
PRG1 3880/4312/0/0; PRG2/3 each0/8192/0/0; CHR0..3 each0/0/8192/0.
Exact physical intervals/counts: disassembly/round003/bank-counts.json.
map.validate passed; fresh pass-003-parameters source rebuild byte-identical,
map SHA256 dcc1c4d2dae47ec707179797494a6d429eae158214ff594e39ad154c354d993e.
Input ROM unchanged. Broad progression reused from scenarios.json; no changed
concrete gameplay question. Generic exterior unchanged. No toolkit issues found;
[toolkit issues](toolkit-issues.md) retained. Overall editorial completion pending.
Handoff: disassembly/round003/handoff.md.

## Coordinator phase transition: round003 commentary

Accepted round003 disassembly handoff and pass-003-parameters verification.
Map dcc1c4d2dae47ec707179797494a6d429eae158214ff594e39ad154c354d993e,
91regions; R002-R01 verified, counts unchanged. Prior source/history preserved.
ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).
Percentage printed before commentary. Current phase round003 commentary: preserve
prior full tree/review, reconcile four sound parameter regions and sixteen-phase
comment before next redirect. Final acceptance remains pending.

## Coordinator phase transition: round003 redirect

Accepted analysis/commentary/round003/handoff.md; current91region/290unit inventory
all reviewed, verification hashes current, refreshed reading copy. Prior full trees
preserved, parameter consumer comment corrected, no readability gap. Input structural
checkpoint pass-003-parameters, zero unknown. Current phase round003 redirect;
commentary writer finished. Reuse existing independent investigation outcomes and
reassess changed evidence only; final targeted editorial audit remains pending.

## Round003 redirect complete

pass-003-parameters unchanged; exact internal-store unknown complement empty,65552 classified bytes. No new ready leads; prior accepted sound/setup refinements preserved. Scoped primary-evidence/rejection review and independent avenue assessment: analysis/redirect/round003-attempt001/findings.md. Whole-ROM path awaits final targeted editorial audit/verification; concrete zoom-threshold value-alias naming target handed off. No source/map/state/ROM/runtime changes or toolkit issues. Handoff: analysis/redirect/round003-attempt001/handoff.md.

## Coordinator transition: final editorial acceptance

Accepted round003 redirect handoff after inspecting findings/reasoning and exact
empty unknown complement. No ready structural leads; previously corrected sound
premise remains localized, remaining semantic limits do not undermine classification.
Current checkpoint pass-003-parameters. Final acceptance must inspect misleading
value-based CLUB_PUTTER/HOLE_COUNT aliases in zoom thresholds and related uses,
reuse valid91region/290unit review credit, repair concrete editorial defects, and
run final audit/fresh verification/refreshed reading. Redirect writer finished.
No terminal success yet. Archive document selection must be refreshed afterward.

## Terminal coordinator handoff: whole-ROM complete

Final editorial acceptance and independent coordinator completion audit passed.
Current structural checkpoint pass-003-parameters; map/state SHA256
`dcc1c4d2dae47ec707179797494a6d429eae158214ff594e39ad154c354d993e`.
ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).
Counts Code9478/Data23306/Image32768,91regions; all290 source units reviewed.
Current source audit passes byte-identical, map/store validate, reading copy current;
ROM hash unchanged. No ready lead or required review work remains. Known original
units/names/natural reachability/finer subformats are documented semantic limits,
not unknown bytes or skipped source work. No compatibility/fidelity claim.

Terminal handoff: `analysis/terminal-handoff.md`; requirement audit:
`analysis/coordinator-completion-audit.md`; independently recomputed identities,
coverage/editorial/source/scenario checks: `analysis/coordinator-final-check.json`.
All phase handoffs and prior commentary/checkpoint/evidence history retained.
Archive contract inspected; `archive-options.json` and
`analysis/archive-selection-check.json` account for all41 Markdown files,
select25 additional final documents and omit6 duplicate generated instruction
scaffolds containing no conclusions. No archive execution or cleanup authorized
or performed; all source/assets/evidence local. No toolkit issues found.
Workflow terminal handoff is ready; classification and editorial completion were
independently verified rather than inferred from orchestration status.
