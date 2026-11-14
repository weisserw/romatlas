# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 active findings

- Vectors inspected at physical $2000A..$2000F; reset $E600, NMI $F098, IRQ $F087. Correct physical roots $1E610/$1F0A8/$1F097, effective slots [0,1,14,15]. Bank-aware CFG and observed edges saved in round-001/discover-*.json; first accepted instruction batch is accepted.json. Intermediate pass-001 verified byte-identical.
- Cartridge header bounds CHR at $20010..$4000F as 128 KiB PPU pattern image storage, independent of executable PRG. Classified entire CHR as Image with Static provenance.
- Uniform $FF PRG blocks inspected for exact run boundaries, surrounding bytes and CFG conflict; accepted as generic Data, not a claimed semantic fill/table. Checks recorded in round-001/uniform-checks.json. Other uniform zero/RTS runs deliberately left for redirect because they need contextual inspection.
- Initial Arkista guess and related RAM patches rejected: unrelated temporary PNG, actual ROM is Baseball Simulator 1.000. Preserve run.log/run-v2.log and patch-stage evidence as mismatch experiments, not progression. No structure depends on their external claims.
- TK-001 input schema mismatch caused initial routes to be attract mode; corrected full-buttons routes reach Exhibition setup and Edit submenu. Actual baseball title and mode captures are uniquely converted adjacent PNGs, never reused /tmp assets.
- Bounded counter experiment: plausible strike/ball/out state at RAM $A0/$A1/$A2 inferred from bank12 code increments/thresholds (three/four/three). Patch bytes [2,0,2] at $A0 after pregame snapshot; 120-frame continuation reaches visible batting field with two outs. Does not yet demonstrate half-inning or ending. Clean snapshot restored afterward. Need full source-supported inning and transition contract, not arbitrary RAM guesses.

## Round 001 final accounting and handoff

ROM classification: 62.45% (163717 / 262160 bytes classified; 98443 bytes unknown).

Code 21,899; Data 10,746; Image 131,072; 98,443 unknown. Final grouped acceptance: 7,112 + 421 newly accepted complete instructions in 713 + 37 grouped regions; zero conflicts, exact view/provenance grouping. Final checkpoint and verification are linked in progress.md. No saved observed ROM opcode start remains unclassified. Wrong-offset decode experiments were not annotation roots; early no-input routes are attract evidence only.

Per-bank evidence: header/CHR boundaries from iNES inspection; PRG code from validated vectors and saved contextual accepted fetch contracts plus connected static CFG; generic data from uniform-checks.json exact byte runs and code conflict check. Reachability claims are restricted to recorded scenario origin.

Lead outcomes: L001 ready (bank-call inline dispatch requires contract/extent research); L002 ready (actual inning/side transition and finite Season AUTO route); L003 ready (bounded adjacent record/pointer/data inspection). Arkista stage hypotheses rejected for actual-title mismatch, with historical evidence preserved; do not retry unchanged. Input protocol issue corrected and documented TK-001. No terminal exhaustion assertion.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2068 | 0 | 6124 |
| PRG 01 | 0 | 663 | 7529 |
| PRG 02 | 3951 | 0 | 4241 |
| PRG 03 | 1229 | 149 | 6814 |
| PRG 04 | 1518 | 0 | 6674 |
| PRG 05 | 0 | 7833 | 359 |
| PRG 06 | 1607 | 247 | 6338 |
| PRG 07 | 343 | 978 | 6871 |
| PRG 08 | 0 | 1 | 8191 |
| PRG 09 | 0 | 0 | 8192 |
| PRG 10 | 900 | 0 | 7292 |
| PRG 11 | 0 | 0 | 8192 |
| PRG 12 | 2530 | 0 | 5662 |
| PRG 13 | 1211 | 0 | 6981 |
| PRG 14 | 2166 | 0 | 6026 |
| PRG 15 | 4376 | 859 | 2957 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |



## Round002 disassembly — complete single pass; incomplete ROM, nonterminal

ROM classification: 94.41% (247503 / 262160 bytes classified; 14657 bytes unknown).

Accepted gain83,786 bytes: frozen master union83,752 (51,013 Static Code /32,739 Static Data), plus34 Observed Code bytes from15 accepted artificial opcode fetches during initialized9INBOT->Start/A defense-menu route. Counts: Code72,934; Data43,497; Image131,072. Exact C00112-byte correction changes role only. Structural review and independent audit: analysis/round-002-disassembly/structural-review.md, independent-review.json, annotation-log.json; new relative-period/audio APU branch physical0676..0698 has configured decoder and observed-annotation-log.json. Static acceptance is distinct from execution reachability; new audio fetches are explicitly artificial.

Final map/state validated and exported. Fresh verified checkpoint **round-002-disassembly-final** (generated/round-002-disassembly-final; maps/round-002-disassembly-final.json), verification analysis/round-002-disassembly-final-verification.json: ok=true, byte_identical=true. Map SHA2567dd308535281fec582e2294a84bb62b73b1d9cd258bf94823b9434f84a8bd0f4; immutable ROM SHA256eeebc807e4a53336b871b4195eef25df0c80a558156742c3411a83225d4f909b. Earlier successful round-002-disassembly checkpoint remains historical. Maintained commentary source unchanged and requires current-map carry-forward in next phase.

Leads L001/L003/L004/L005/L006 verified as the overlapping coherent supported batch, not summed separately. L002 match progression remains verified with prior Season limits. Natural Ultra team initialization and special pitching now visited (S009): icon selection/delivery and3-point consumption confirmed; no successful special hit claimed. Artificial defensive CHANGE/MOVE interface exercised (S008), B exit/12 released frames returned to live9INBOT; no exact reposition/live-contact catch claim. B-only field-view and mistimed Ultra half-inning patch negatives preserved; do not retry unchanged. Season stored165/failed SKIP-init contract remains unchanged. Runtime detail capacity0 intentionally drops detail, aggregate_dropped=0; accepted contextual summaries retained. Two early over-limit coverage queries were corrected to paginated limit1000; one bounded step timeout retained partial state, not claimed complete.

Active frontier:249 exact unknown intervals totaling14,657 bytes in round-002-disassembly/unknown-frontier.json; per-bank counts below/bank-counts.json. Largest mixed residuals include physical09506..0A010,0D62B..0DF19,0F810..0FDDD and fixed-bank fragments. No blanket gap classification. Low-bank object callbacks still need installing caller's effective selection; other mixed fragments need local entry/format bounds. Next commentary must carry forward current code/data roles and BIT alternate symbols D0BB/D0BE; then redirect may inspect concrete residual contracts. No terminal exhaustion or whole-ROM/readability completion claim. Current best-effort mechanics gaps are documented rather than converted to classification blockers.

Toolkit: existing confirmed input schema bug TK-001, workaround full-buttons arrays; no new toolkit issue. Preserve analysis/toolkit-issues.md. No toolkit edits, ROM edits, source replacement, cleanup/archive/deletion/publication.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2229 | 5588 | 375 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 7692 | 238 | 262 |
| PRG 03 | 7339 | 211 | 642 |
| PRG 04 | 4144 | 306 | 3742 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 4424 | 537 | 3231 |
| PRG 07 | 5546 | 982 | 1664 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8192 | 0 |
| PRG 10 | 7566 | 335 | 291 |
| PRG 11 | 7588 | 313 | 291 |
| PRG 12 | 7689 | 233 | 270 |
| PRG 13 | 6625 | 477 | 1090 |
| PRG 14 | 6955 | 565 | 672 |
| PRG 15 | 5137 | 928 | 2127 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |


## Round003 disassembly — complete single pass, incomplete ROM classification

Accepted the full independently reviewed frozen r002-copy-contracts master union:14478 unique new bytes,3562 Static Code/10916 Static Data. Exact C003 correction moves three previouslyCode bytes toData and retains nextJSR atC592. Final structural totals:Code76493,Data54416,Image131072. Physical bytes and supported CPU/effective contexts are distinct; aliases do not increase counts. Prior C001 correction and canonical BIT/LDA D0BB/D0BE alternatives retained. Independent review `analysis/round-003-disassembly/structural-review.md`, `independent-review.json`, `data-review.json`, `existing-continuation-boundaries.json`; mutation evidence `annotation-log.json` (335 RPC calls). All160 pattern intervals raw/neighbors checked,33 wrapper contracts,1712 instruction entries and1686 graph views inspected; reused clean configured release decoder115 calls. Three old continuation sources confirmed by actual canonical boundary walks and raw opcodes, not mere Code containment.

State/current validated and exported; fresh unique checkpoint `round-003-disassembly-final` verified byte-identically: `analysis/round-003-disassembly-final-verification.json`, `generated/round-003-disassembly-final`, `maps/round-003-disassembly-final.json`. Current map/state/checkpoint SHA25688b2a2d0856b4c936e59a043fa821c303fc91600e1b4183e3afb7711e12f9eac. ROM unchanged eeebc807e4a53336b871b4195eef25df0c80a558156742c3411a83225d4f909b. Maintained commentary source unchanged687c97b04b3bf6c0c9f20428543007070ea3964c83bba113c93fba07a9ee86c5; no source regeneration/mutation in this phase.

LeadsL001/L003/L005/L006 verified for overlapping full union; histories preserved. Broad gameplay/scenarios reused: Exhibition/Edit, natural Ultra pitching selection/delivery/points and artificial9INBOT pitching/defensiveMOVE/return/GAMESET. No unchanged Season replay, no new special-hit/catch/Season-terminal claim. Practical late-game shortcuts already attempted and documented in scenariosS005/S006/S008/S010; unresolved Season widget/parent-transition contract remains a separate semantic/gameplay question, not a reason to withhold structural claims.

Active next frontier: current-map commentary for all new/affected classifications and C003 explanation, then redirect over exact179 unknown bytes/39 spans (`analysis/round-003-disassembly/unknown-frontier.json`). Reused `residual-review.json` and `actor-tail-review.json` actual rejection scope: actor helper targets fail offered clone semantics by entering operands, while isolated returns/jumps/EA/branch fragments still lack discriminating roles; no blanket unknown-toData conversion or missing-natural-caller prerequisite. No positive continuation withheld for another round. This is an ordinary easy-gain handoff, not whole-ROM/readability/terminal exhaustion success. ROM classification remains incomplete.

Toolkit: no new toolkit issues found. Existing confirmed TK-001 and input-array workaround preserved in `analysis/toolkit-issues.md`. No toolkit/ROM edits, history deletion, cleanup/archive or publication.

ROM classification: 99.93% (261981 / 262160 bytes classified; 179 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2520 | 5672 | 0 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 7745 | 447 | 0 |
| PRG 03 | 7368 | 815 | 9 |
| PRG 04 | 4150 | 4042 | 0 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 6425 | 1766 | 1 |
| PRG 07 | 5569 | 2618 | 5 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8192 | 0 |
| PRG 10 | 7614 | 539 | 39 |
| PRG 11 | 7600 | 576 | 16 |
| PRG 12 | 7710 | 482 | 0 |
| PRG 13 | 6636 | 1555 | 1 |
| PRG 14 | 7144 | 1032 | 16 |
| PRG 15 | 6012 | 2088 | 92 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |


## Round003 redirect — localized fragment contracts

Input map/state88b2a2d0856b4c936e59a043fa821c303fc91600e1b4183e3afb7711e12f9eac, source4acccb9ce71d6c7479ddf89f0239e1a2d3c9ea65b8882797731d39737594c77a. Actual261981/262160classified;179unknown/39spans unchanged. Unique attempt `analysis/redirect/r003-fragment-contracts/` revisits the short-unrooted inference using independent field/consumer/parallel-entry constraints, not unchanged runtime retries. L007 ready:53 Static Code bytes/10spans,22instructions,10configured decoder calls,0faults. Coherent action/priority veneers, fourth direction emitter, full height predicate, possession-gated collision/draw and modifier0/2 shared entries all reviewed with existing continuations. Each subclaim remains independently reviewable. Exact residual ledger and localized non-code/actor rejection scopes in contracts.md; no genericData fallback or first-ready stop. C004 appended research correction C05D->CFB4 rendering-disable, notD0B8 initializer. Gameplay checklist reused with actual GAMESET/artificial defense and naturalUltra pitch/point evidence; unchanged Season attempts not replayed. Research/lead records only; no ROM/map/state/source/toolkit edits. No new toolkit issues found; existing TK-001 nonblocking. Handoff `analysis/round-003-redirect-handoff.md`; nonterminal, no goal completion or terminal exhaustion claim.


## Round004 disassembly — independent L007 review and verified checkpoint

ROM classification: 99.95% (262031 / 262160 bytes classified; 129 bytes unknown).

Accepted50 unique Static Code bytes/nine spans (F01,F02 both spans,F03,F05..F09); F04 three-byte isolated duplicate JMP retained Unknown. Exact role/control/stack/field/consumer/bank/boundary rationale: `analysis/round-004-disassembly/structural-review.md`; fresh configured decoder9 calls/21instructions/0diagnostics corroborates exact boundaries. No previous classified byte changed kind; no Data gain. Code76543, Data54416, CHR131072. Residual129bytes/30spans in `analysis/round-004-disassembly/unknown-frontier.json`. Complete connected accepted continuations reviewed; no ready fragment deferred.

Fresh checkpoint `maps/round-004-disassembly-final.json` SHA256 808eeb38ef5b3411a7764ae95b6c98939e707d53b791fc646322facbbfc3b026; `analysis/round-004-disassembly-final-verification.json` confirms byte_identical=true,ok=true. State/map/checkpoint identical; immutable ROM identity preserved. Maintained commentary source unchanged4acccb9ce71d6c7479ddf89f0239e1a2d3c9ea65b8882797731d39737594c77a; changed-map readability and final2719-row inventory metadata remain pending commentary, separate from classification/fidelity. C004 correct current semantics preserved.

Reviewed/reused broad gameplay S005 artificial9INBOT/pitch/GAMESET, S008 defensiveMOVE/exit, S009 naturalUltra pitch/point evidence and failed S006Season widget/parent routes. No changed practical shortcut arose; no unchanged replay or new natural/Observed claim. Nonterminal easy-gain stopping point, not terminal exhaustion. Coordinator flagged EA singleton inference for later localized review: regular6byte sibling spacing plus bounded5byte service payload may itself supply positive padding evidence; NOP-prefix possibility must be weighed rather than imposed as a universal veto. Not adjudicated beyond this supported batch.

Existing TK-001 confirmed input-schema bug remains nonblocking in `analysis/toolkit-issues.md`; no new toolkit issues found (0new confirmed/suspected/missing features). Toolkit unchanged. Disk5.9GiB free/scratch52KiB checked; no cleanup/deletion.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2520 | 5672 | 0 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 7745 | 447 | 0 |
| PRG 03 | 7374 | 815 | 3 |
| PRG 04 | 4150 | 4042 | 0 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 6425 | 1766 | 1 |
| PRG 07 | 5569 | 2618 | 5 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8192 | 0 |
| PRG 10 | 7644 | 539 | 9 |
| PRG 11 | 7614 | 576 | 2 |
| PRG 12 | 7710 | 482 | 0 |
| PRG 13 | 6636 | 1555 | 1 |
| PRG 14 | 7144 | 1032 | 16 |
| PRG 15 | 6012 | 2088 | 92 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

## Round004 redirect r004-wrapper-spacing

Input808eeb38; research only. Revised primary reasoning finds rooted NOP-prefix service family and two independently complete actor conditional-copy blocks; combined34 Static Code bytes/16spans proposed via L008/L009,0accepted. Exact primary evidence, local rejection scopes, gameplay review and deferred-avenue reasons: analysis/redirect/r004-wrapper-spacing/contracts.md; proposal-batch.json and unknown-ledger.json. No terminal/whole-ROM claim, no map/state/source/input/toolkit changes. ExistingTK-001 nonblocking; no new toolkit issues found.


## Round005 disassembly — independent L008/L009 review

ROM classification: 99.96% (262065 / 262160 bytes classified; 95 bytes unknown).

Accepted34 Static Code bytes in16spans:14 NOP entry prefixes W01 and two10byte actor coordinate conditional-copy blocks A01/A02. No Data gain, no prior classified kind changes. Code76577, Data54416 including header, CHR131072. Configured decoder16calls/22instructions/0diagnostics corroborates exact boundaries. Structural proof and localized negative scope: `analysis/round-005-disassembly/structural-review.md`; primary rooted caller115814->115195 and canonical actor123175 verified against immutable ROM. No candidate natural/Observed reachability claimed. Existing inline Data preserved.

Fresh byte-identical checkpoint `maps/round-005-disassembly-final.json` SHA256 289c2300d7444e697915ba512406de08776cdff0d5ddab5749b40510a2118dc4; verification `analysis/round-005-disassembly-final-verification.json`, ok=true,byte_identical=true. State/current/checkpoint identical; source remains2e1d15500bb065d4660a258a956596844f0ebe60bb415cf8e38f190820093a93 unchanged. Changed-map commentary pending; final812-row unit metadata frontier remains separate. C004 semantic correction intact.

Broad gameplay S005/S008/S009 and S006 Season barrier reused, without unchanged replay. This static finding supplies no improved transition/RAM contract; successful catch/contact, Season terminal and natural ending remain limited. F04 isolated duplicate JMP and helper-dependent actor residuals remain unknown. Ready batch/connected continuations reviewed; ordinary easy-gain stopping point, no terminal exhaustion claim. Exact95byte/18span frontier in `analysis/round-005-disassembly/unknown-frontier.json`. ExistingTK-001 nonblocking; no new toolkit issues found; see `analysis/toolkit-issues.md`. ROM/toolkit/history preserved; disk5.9GiB free/scratch52KiB; no cleanup/publication/archive.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2520 | 5672 | 0 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 7745 | 447 | 0 |
| PRG 03 | 7374 | 815 | 3 |
| PRG 04 | 4150 | 4042 | 0 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 6425 | 1766 | 1 |
| PRG 07 | 5569 | 2618 | 5 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8192 | 0 |
| PRG 10 | 7644 | 539 | 9 |
| PRG 11 | 7614 | 576 | 2 |
| PRG 12 | 7710 | 482 | 0 |
| PRG 13 | 6636 | 1555 | 1 |
| PRG 14 | 7158 | 1032 | 2 |
| PRG 15 | 6032 | 2088 | 72 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

## Round005 redirect r005-actor-patterns

Checkpoint289c2300; exact95byte/18span complement preserved. Corrected needed-claim review distinguishes recognizable field/mask/control machine-code fragments from invalid whole-handler contracts. L010 proposes62 Static Code bytes/3spans;0accepted here. Primary bytes, canonical comparison, malformed target scope, weaker subclaims, other residual dispositions and broad gameplay review: analysis/redirect/r005-actor-patterns/contracts.md, actor-primary.json, unknown-ledger.json. No useful connected continuation omitted; ordinary ready handoff, no terminal or ROM-completion claim. Map/state/source/input/toolkit unchanged; existing TK-001 nonblocking, no new toolkit issues found.


## Round006 disassembly — independent L010 actor pattern review

ROM classification: 99.99% (262127 / 262160 bytes classified; 33 bytes unknown).

Accepted62 Static Code bytes in3spans P01/P02/P03, physical123199..123229,123239..123259,123269..123281, bank15 effective[null,null,14,15]. Code76639/Data54416/CHR131072. Prior annotated kinds preserved. Structural rationale and weaker subclaim review: analysis/round-006-disassembly/structural-review.md. Immutable ROM/context/decode checks: independent-review.json; configured release25instructions/0diagnostics. Repeated field/test/mask/local branch topology supports recognizable fragments; malformed contemporary helper destinations remain explicit. No functioning whole handler, relocation, obsolete origin, natural entry or Observed claim. Orphan2804 remains unknown. Exact62byte diff/33unknown/16span audit and per-bank counts retained.

Fresh maps/round-006-disassembly-final.json SHA9693711a84a622e470d5e1ae492735ca2404fffe6b7778d316054494c7c21637; analysis/round-006-disassembly-final-verification.json confirms ok=true,byte_identical=true. State/current/checkpoint identical, input ROM and commentary source unchanged SHA95d55c01310dcf6570dfc279f45016dbd8ecb068a4f94bbb7fa695b20d13a136. Full current-source commentary next; final812-row metadata frontier pending.

Broad progression S005 actual artificial GAMESET/S008 MOVE/S009 natural Ultra reused; Season widget/parent transition and failed S010 limits unchanged. No new practical RAM/transition contract, no unchanged replay. Remaining isolated transfers/returns, numeric trailer,E608 split and orphan retain localized ambiguity; no automatic extension. Ordinary easy-gain handoff, not terminal exhaustion. Existing TK-001 nonblocking; no new toolkit issues found, see analysis/toolkit-issues.md. Disk5.9GiB free; no scratch batch, cleanup,archive,publication or toolkit changes.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2520 | 5672 | 0 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 7745 | 447 | 0 |
| PRG 03 | 7374 | 815 | 3 |
| PRG 04 | 4150 | 4042 | 0 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 6425 | 1766 | 1 |
| PRG 07 | 5569 | 2618 | 5 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8192 | 0 |
| PRG 10 | 7644 | 539 | 9 |
| PRG 11 | 7614 | 576 | 2 |
| PRG 12 | 7710 | 482 | 0 |
| PRG 13 | 6636 | 1555 | 1 |
| PRG 14 | 7158 | 1032 | 2 |
| PRG 15 | 6094 | 2088 | 10 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |


## Round006 redirect — residual review, terminal incomplete recommendation

New attempt `analysis/redirect/r006-residual-review/`; input checkpoint/current/state9693711a84a622e470d5e1ae492735ca2404fffe6b7778d316054494c7c21637, readable source19180bcbe08750c7974381bb8beeefbd9e6c9a57bb9759906b90d8a967eab69a. Corrected Code-role criterion from actor acceptance tested against isolated transfers/returns; positive generic-Data patterns and independent bounds considered locally. Fresh exact33byte/16span ledger and limited accepted-Code direct-edge audit saved; no direct edges found, not an unreachability conclusion. Source-lead scan inspected dispatch/table/bank/semantic uncertainties and generic labels; actual source constraints and primary bytes weighed in compact rejection-review.md. No supported proposal or justified new discriminating test survives. Recommend stop incomplete pending coordinator independent inspection, not ROM/goal completion; no empty disassembly cycle. Proposed0/accepted0; Code76639, non-CHRData54416,CHR131072,Unknown33.

Broad gameplay primary review rechecked actual GAMESET/Ultra/MOVE images and failed Season shortcut/timer/Start/A-intercept logs; no new active-widget/parent/entry contract justifies unchanged replay or arbitrary RAM. Season terminal/live contact/credits remain unvisited. Current-source readability complete; final812Code-row metadata frontier and archive acceptance remain pending. No new toolkit issues found; existing confirmed nonblockingTK-001 retained. No ROM/map/state/source/toolkit/history edits, cleanup or publication. Durable review/primary links and exact residuals in `analysis/round-006-redirect-handoff.md`.
