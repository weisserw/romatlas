# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## R001 input-contract correction
Initial routes in driver/explore/start-game used shared-protocol `button/pressed`. NES accepted those RPCs but ignored those fields; actual inputs were empty. Those runs establish title-only natural execution, no attempted attack/menu reachability. Source inspection establishes NES `buttons` full held-set array. Corrected experiment `round-001/accepted-start.py` reopens game-start on changed method. NES-001 records the documentation/validation defect.

Maximal uniform runs >=64 bytes classified generic Data after physical boundaries/neighbor inspection and code-conflict check (`round-001/uniform-run-review.json`). No schema/fill purpose inferred; structural non-code only.

## R001 progression contract and connected tables
- Natural one-player stage1 practice/fighting observed after fixing input full-array schema. `fight-action.ppm.png` shows player and enemies; contexts preserve accepted fetches.
- Artificial later-stage reinitialization: RAM $22 (stage index0..14), PC $D014, stack $FF, context [12,13,14,15], restored from stage1 fight snapshot. D03E copies selector into $A1; D881 derives stage pointers; D268 clamps normal advance below15. Index4/9/14 visibly renders stages5/10/15. Initial short action windows captured intro scrolling; `later-action.py` adds240f untraced wait and80f traced actions, visually confirms active enemies/player attacks at stages10/15 (`stage10-active.ppm.png`, `stage15-active.ppm.png`). These do not establish natural later-stage reachability.
- Connected bank12 text structures verified: AC6F selects quote by stage; ACE6 uses32-pointer screen text table, names by stage+17, player by15/16. Separate15-pointer digit table at CPU B359 selects8-byte records; 4-by-2 tile write loop plus stage-domain endpoint proves bounds. Glyph translation loop and text controls support bounded data regions. `classify-text-tables.py`, `decode-0xac6f.json`, `decode-0xd238.json` preserve proof.
- Static closure from all accepted contextual fetch roots gained808 bytes; `static-closure-candidates.json` and reports record hypotheses inspected/accepted without claiming execution. Stops preserve mapper/call/indirect barriers.
- Ending experiment: valid stage15 snapshot; align instruction, PC $D255 / stack $FF enters post-stage dispatch. D27B[14]=3 -> AA1D. This bypasses stage completion/score animation and is artificial. Pending bounded capture result.

## R001 final outcome
ROM classification: 12.98% (17021 / 131088 bytes classified; 114067 bytes unknown).

| PRG 8KiB bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 861 | 0 | 7331 |
| 1 | 0 | 96 | 8096 |
| 2 | 0 | 0 | 8192 |
| 3 | 0 | 125 | 8067 |
| 4 | 0 | 0 | 8192 |
| 5 | 0 | 812 | 7380 |
| 6 | 0 | 0 | 8192 |
| 7 | 0 | 133 | 8059 |
| 8 | 0 | 0 | 8192 |
| 9 | 0 | 390 | 7802 |
| 10 | 0 | 0 | 8192 |
| 11 | 0 | 1119 | 7073 |
| 12 | 657 | 90 | 7445 |
| 13 | 1240 | 2447 | 4505 |
| 14 | 5765 | 149 | 2278 |
| 15 | 2306 | 815 | 5071 |

Ending dispatch verified visually: `round-001/ending-entry.ppm.png` and `ending-later.ppm.png` show Duke waving from car with money; START exits (`ending-start.snapshot.json`). This is an artificial post-stage dispatcher entry, not proof of natural completion. Bounded vector/observed-root static closure and representative gameplay/shortcuts completed; residual stream/graphics structures and exact boss transitions go to redirect. Final state and checkpoint links/counts are in progress. No terminal exhaustion claim is made.

## Round001 redirect combined research completed
See `round-001-redirect-handoff.md` and `redirect/r001-connected-20261112/frontier.md`. Connected and independent avenues investigated through supported stage/VM handlers, roots/native routines, source containers and representative boss/clear behavior. Combined research-only proposals114060newbytes plus1StaticCode->Data correction; accepted map unchanged. Rejected/incomplete model records retained and localized; no terminal exhaustion claimed. `leads.json` has8ready IDs with exact recipe/evidence/history, and `unknown-ledger.json` preserves input complement and7hypothetical residual bytes. Independent review required before annotation; don't replay unchanged completed routes.

## Round002 disassembly completed — incomplete ROM

ROM classification: 99.99% (131081 / 131088 bytes classified; 7 bytes unknown).

Accepted114060new bytes (3158code/110902data), plus one prior StaticCode byte corrected toData. Total code13986/data117095 (including16header bytes); Observed code9692, Static code4294. Structural coverage and execution are separate; all new classifications Static. Exact primary review: `round-002/review.md`, `round-002/primary-review.json`, `round-002/mixed-source-review.json`; annotation journal and precise count/complement files in same directory.

| PRG 8KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 888 | 7304 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 1454 | 6737 | 8191 | 1 |
| 13 | 1901 | 6290 | 8191 | 1 |
| 14 | 7002 | 1188 | 8190 | 2 |
| 15 | 2741 | 5448 | 8189 | 3 |

State/map SHA256 `d2310ef19eec8c395e3be0f6ba1377a1511f35e9223a5cc0e7792067765808df`. `map.validate` passed; new `maps/pass-002.json`, `generated/pass-002`, `pass-002-verification.json` verify byte-identical ROM rebuild. Existing commentary/source SHA256 remains `c5b3798e82718eed19827dab9d47efd3ab9eed533b59a395f3a6ea9a3927ecc6`; current-source commentary refresh belongs to next phase. No new runtime replay; prior representative progression/boss/score/ending evidence reused. Seven unknown bytes at CPU805A,AC1F,DBA3,DDB5,F822..F824 retained; no whole-ROM completion/terminal exhaustion claim. All8ready lead outcomes verified with structural/reachability limits, new R002-RESIDUAL-7 records next question.

Toolkit: no new findings; existing confirmed NES-001 only, zero suspected/missing-feature findings, no blocker. See [toolkit-issues.md](toolkit-issues.md). Configured binary unchanged; no toolkit edits. ROM and historical evidence preserved; no publication/archive/cleanup.

## Round003 disassembly completed — incomplete ROM

ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown).

Rounded100.00% does not establish completion: unknown3 bytes remain. Accepted4Static genericData bytes atphysical1806A length1 and1F832 length3. Totals13986code/117099data (includes16header), Observedcode9692/Staticcode4294 unchanged. No new execution coverage or reachability claim. Independent evidence and alternatives: round-003/review.md and apply.py; all prior regions/metadata preserved.

| PRG 8KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 888 | 7304 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 1454 | 6738 | 8192 | 0 |
| 13 | 1901 | 6290 | 8191 | 1 |
| 14 | 7002 | 1188 | 8190 | 2 |
| 15 | 2741 | 5451 | 8192 | 0 |

Unknown exactphysical1AC2F/1DBB3/1DDC5, CPUAC1F/DBA3/DDB5, each60 under[12,13,14,15]. Counts/complement: round-003/classification-counts.json and unknown-complement.json. State/current/pass-003 mapSHA `67db4c98e95d432b6caf57006f381944c440f7c1a726f9fda64957ee07acd308`; validation passed, fresh generated/pass-003 and pass-003-verification.json confirm byte-identical rebuild. Source/commentarySHA f6689b39eaf8419ac688c9df372283aa250160349091cb3e616a2bcbc725df4b preserved; next commentary must integrate4bytes and seven stale-scope editorial notes before later redirect/editorial assessment. Prior gameplay/static research reused without unchanged replay. Both ready lead outcomes verified; residual lead partially verified with3unknown remaining. No terminal exhaustion claim.

Toolkit: existing confirmedNES-001 only, zero suspected/missingfeatures, no blocker/new finding; [toolkit-issues.md](toolkit-issues.md). Configured binary/ROM unchanged; no toolkit edits/publication/archive/cleanup. Disk6.4GiBfree, scratch24KiB; durable historical evidence preserved.
