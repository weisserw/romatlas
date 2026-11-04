# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly in progress
Baseline established and byte-identical: [verification](baseline-verification.json).
Configured release binary is in use. Runtime session started; default tracing was disabled,
then enabled with capacity zero to retain bounded coverage/edge aggregates.
Initial HAL America 1987/1989 splash observed after 180 frames. Investigation continues.
Session replay and RPC evidence: `analysis/round-001/rpc-log.jsonl`.

### Broad progression and first classification batch
Identified title: Adventures of Lolo 2 (HAL America, USA-style title).
Built-in passwords successfully visited floor 5 (HRKJ), floor 10 (QTDV), final boss
(VQTD) and expert room (PROA). Movement and A/B inputs exercised in each;
initial room movement and Select restart/life loss also observed. Routes and snapshots
are in `analysis/round-001/late.py`, `*.snapshot.json`, `visit-*.png`, and
`mechanics-*.png`. Stage index RAM $78 values respectively 20,45,50,53; $79 floor
values 4,9,10,0. These are naturally selected by passwords, not debug edits.
First classified batch: 7,108 PRG code bytes from 3,540 supported instruction starts,
plus 16-byte header, 6-byte vectors, 32,768-byte CHR pattern backing. 2,357 distinct
observed starts supplied roots. State saved and map validated; fresh final checkpoint
will follow ending experiment and data-boundary review. Disassembly still in progress.

## Round 001 disassembly complete — incomplete ROM
ROM classification: 63.07% (41346 / 65552 bytes classified; 24206 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG0 | 1358 | 0 | 6834 |
| PRG1 | 1947 | 1195 | 5050 |
| PRG2 | 3936 | 0 | 4256 |
| PRG3 | 65 | 61 | 8066 |
| CHR0 | 0 | 8192 | 0 |
| CHR1 | 0 | 8192 | 0 |
| CHR2 | 0 | 8192 | 0 |
| CHR3 | 0 | 8192 | 0 |

Fresh checkpoint: `maps/round-001-disassembly.json`, generated source
`generated/round-001-disassembly`, verification
`analysis/round-001-disassembly-verification.json`: valid map, byte-identical rebuild.
Current state `analysis/state.json` and portable mapping `maps/current.json` saved.
2,431 distinct physical opcode starts observed across sampled milestone frames;
3,625 starts structurally classified (7,306 code bytes). Sampling excludes untraced
transit; runtime and classification coverage are separate. All classified code decoded
to official instructions with coherent control flow; no unsupported opcode paths adopted.
All regions have structural evidence; routine/variable commentary remains next phase.
Ending: artificial clear-transition entry from final boss yielded castle/rescue montage
through 3,180 frames; no natural boss-defeat or final terminal-credits claim.
One missing feature TK-001, no confirmed/suspected bugs, no blocker; see
[toolkit issues](toolkit-issues.md).
Handoff: [round-001-disassembly-handoff.md](round-001-disassembly-handoff.md).
Next phase: commentary, then redirect on inline-script/bank-effect contracts.

## Round 001 — commentary in progress
Preparing first commentary baseline from the verified disassembly. Full current classified scope is 7,306 code bytes plus header, classified data and CHR; unknown bytes remain unresolved. Final editorial audit deferred to final acceptance.

### Commentary full-scope source batch
All 341 code blocks now have functional explanations and names; classified data/image regions and 141 unknown intervals have source notes. First edited-source rebuild was byte-identical. Shared RAM overlay and source-inventory precision checks remain in this phase. See commentary/progress.md.

## Round 001 commentary complete — incomplete ROM
Full current-source readability complete: 341 code regions / 7,306 bytes, five
classified structures and 141 unknown intervals explained. All 487 scaffold entries
reviewed; 86 meaningful units, 635 code labels, 202 RAM/register symbols. Original
classification/state/checkpoint and ROM unchanged: 63.07% classified, 24,206 unknown.
Fresh final commentary verification passed byte-identically; reading copy refreshed.
Scope/hashes: `analysis/round-001-commentary-scope.json`; handoff:
`analysis/round-001-commentary-handoff.md`. No readability gaps deferred.
Final editorial audit reserved for final acceptance. L001/L002 and other precise
uncertainties are in `commentary/uncertainties.md`. Existing TK-001 missing feature,
no bugs/blockers or new findings: `analysis/toolkit-issues.md`.
Coordinator readability review precedes redirect; this worker did not run redirect.

### Commentary coordinator gate repair
Corrected source references in review metadata to `game.asm:line`, relative to
`commentary/source`; all 573 region/unit references resolve to valid source lines.
Fresh byte-identical verification and reading-copy refresh passed with source and
frozen baseline unchanged. Scope evidence refreshed; final audit remains pending.

### Commentary unit metadata gate repair
Added reviewed unit records for 255 already explained code sub-blocks. Current count
is 341 routine/sub-block units, zero code regions without units, and 828 validated
source references, superseding the earlier 86-unit/573-reference inventory counts.
Unit physical bounds and source paths/lines checked. Fresh verification/read refresh
passed; source/frozen baseline unchanged. Final editorial audit still pending.

## Redirect round 001, active contracts investigation
Started unique attempt `analysis/redirect/round001-contracts/` from accepted round-001 commentary/disassembly handoffs. Input checkpoint unchanged (41346 classified, 24206 unknown). Configured decoder evidence establishes counted return-adjacent pointer initialization at $801A and separate fixed two-byte PPU descriptor parameter mechanism. Connected consumers and conservative call summaries are being investigated before combined handoff. Status/primary evidence persisted in attempt directory. No classification changes or toolkit edits.

Redirect checkpoint 6: resumed scene callback frontier; proved eight ending callback contracts and eight-panel tutorial selector/descriptor route; followed supported object dispatch closure under explicit finite initialized-state precondition. Primary evidence `analysis/redirect/round001-contracts/scene-contracts.md`. Map/source/state remain unchanged. Continuing residual concrete consumers; no phase stop.

## Redirect round001 complete — combined proposal pending independent review
Input unchanged:41346 classified/24206 unknown; ROM/map/state/source hashes verified. Combined static proposal8422 new code+15775 data, potential residual9 bytes in four ambiguous unreferenced gaps. Tutorial, ending scene callbacks, finite object dispatch, audio VM, compressed resources, sprites, room/metatile helpers and adjacent lookup/palette structures investigated together. Exact proof links, conditional bank/stack contracts and deferred-gap reasons: `analysis/redirect/round001-contracts/handoff.md`; primary audit/unknown ledger alongside it. `analysis/leads.json`: L001/L002 updated, L003..L005 added ready. Independent disassembly must accept/narrow classifications, then full commentary and redirect. No classification, final audit, archive, cleanup or publication. Existing TK-001 missing feature; no new issues or blockers. No live process.

## Round 002 disassembly active — independent review
Input `maps/round-001-disassembly.json` (41346 classified/24206 unknown), accepted round001 commentary and `analysis/redirect/round001-contracts/handoff.md` read. No proposal is accepted yet. Reviewing primary consumer instructions, bank/stack contracts, inline barriers, bounded record formats and generic data boundaries independently; proposal audit is a hypothesis. No unchanged gameplay replay planned; prior broad progression/scenario evidence is being checked. Review frontier/evidence will be retained under `analysis/round-002/`.

Round002 review frontier: fresh configured decoding of3922 proposed instructions and independent record/extent checks passed (`analysis/round-002/mechanical-review.json`). Written acceptance/rejection review saved in `analysis/round-002/independent-review.md`. Rejected three negative-offset palette claims caused by timed8 argument order; corrected palette targets $2445/$2449/$244D lie in already proposed palette data. Followed indexed origins to eight additional residual data bytes ($2B63..65,$441A..1C,$4ED1..D2); exact references and limited claims recorded. $3C70 remains ambiguous. Preparing supported map.annotate calls, then export/validation/fresh checkpoint; no live emulator/RPC process currently.

## Round 002 disassembly complete — incomplete ROM
ROM classification: 100.00% (65551 / 65552 bytes classified; 1 bytes unknown).
Rounded100.00% is not completion: physical $3C70 remains Unknown.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG0 | 3770 | 4422 | 0 |
| PRG1 | 5437 | 2754 | 1 |
| PRG2 | 6328 | 1864 | 0 |
| PRG3 | 193 | 7999 | 0 |
| CHR0 | 0 | 8192 | 0 |
| CHR1 | 0 | 8192 | 0 |
| CHR2 | 0 | 8192 | 0 |
| CHR3 | 0 | 8192 | 0 |

Accepted8422 new code+15783 new data bytes (24205 unique gain). Existing classifications and commentary preserved exactly. Corrected three invalid negative palette claims (timed8 argument order); independently resolved eight proposed residual bytes through indexed references. Primary review: `analysis/round-002/independent-review.md`; mechanical checks/fresh decode and exact counts/hashes alongside it. L001..L005 verified, L006 records remaining isolated RTS ambiguity and reopening condition. No new execution/natural-reachability claims. Prior broad gameplay checklist and evidence reviewed without unchanged replay.

Map validated and fresh final checkpoint `maps/round-002-disassembly-reviewed.json` generated under `generated/round-002-disassembly-reviewed`; `analysis/round-002-disassembly-reviewed-verification.json`: ok=true,byte_identical=true. State/current/checkpoint SHA256 fe00480c889d06946c5bc8401fdc293cc27b68246470b265b89d49d2c783a0b5. 5347 new annotation regions; old346 regions preserved exactly. Earlier `round-002-disassembly` is an unchanged-input verification accidentally run after a safely rejected annotation invocation; preserved as history, not final checkpoint. Annotation creation required id0; failed invocation logs retained, no toolkit defect.

Full ensuing commentary must review all current source while preserving prior accepted explanations. Current code15728 bytes. No final editorial audit, archive, cleanup/deletion, toolkit edit or publication. Existing TK-001 missing feature, zero new issues or blockers; see `analysis/toolkit-issues.md`. Disk9.5GiB free; attempt9.5MiB before final documentation; no live process. Handoff `analysis/round-002-disassembly-handoff.md`.

## Round 002 commentary complete — incomplete ROM

Accepted current-source readability handoff: `analysis/round-002-commentary-handoff.md`.
Scope covers all5694 scaffold entries,563 code regions/15728 code bytes and all
classified data/image with explained sole unknown$3C70.947 reviewed units,6641
independently validated source references; zero remaining readability gaps. Fresh
byte-identical verification and reading refresh passed. Source SHA256
 d4f18bf8710a41a77bb6080ab6df23569efc1c10f3d447840f7771dfda78b989.
Map/state checkpoint unchanged fe00480c889d06946c5bc8401fdc293cc27b68246470b265b89d49d2c783a0b5.
L001..L005 resolved finite contracts incorporated; L006 remains for redirect.
Separate round002 redirect worker active on exact residual and current uncertainties;
terminal rejection review and final editorial acceptance pending. No whole-ROM
completion claim. Existing TK-001 persists; no new issues.

## Redirect round002 complete — incomplete, reasoning review pending
Unique attempt `analysis/redirect/round002-residual/`, unchanged checkpoint fe00480c889d06946c5bc8401fdc293cc27b68246470b265b89d49d2c783a0b5. 65551 classified/1 unknown; exact physical$3C70 RTS ambiguity persists. New combined7547-instruction reference/indexed-origin/CPU-alias and pointer-encoding review found no independent code/data role discriminator. Decisive instruction coincidences and neighboring tail/handler contracts inspected; missing caller not imposed as a universal prerequisite. Mandatory compact `rejection-review.md` compares static callable and generic-data alternatives, localizes negative evidence and records why no useful revised test survives. `unknown-ledger.json` is exact map complement; `avenues.md` reviews remaining source semantics and broad progression/ending hold without unchanged replay. L006 history and passes preserved/updated; scenarios records progression review. Proposed/accepted0; no ready leads or interrupted productive frontier. Coordinator must inspect rejection reasoning, then perform reserved final editorial audit before final acceptance. ROM/map/state/source/configured binary unchanged; no toolkit edits/archive/deletion/publication. Existing TK-001 persists; no new toolkit issues or blocker. No live process. Handoff `analysis/redirect/round002-residual/handoff.md`.

## Terminal incomplete handoff — final acceptance complete

Round002 redirect produced no ready leads; L006 retains exact one-byte ambiguity
and evidence-based reopening condition. Coordinator inspected compact rejection
review and decisive raw ROM/source/reference artifacts; no corrected inference or
useful discriminating test remains on current premises. Accepted justified terminal
incomplete, not ROM completion or a blocked-success substitution.

ROM classification: 100.00% (65551 / 65552 bytes classified; 1 byte unknown).
Final checkpoint `maps/round-002-disassembly-reviewed.json`; whole-ROM classification
remains incomplete at physical$3C70. Commentary full readability and final editorial
acceptance complete. Official audit:5694 reviewed regions,947 units,ok=true,
byte_identical=true,editorial_audit=true; reading refreshed. Map/store validations
passed; input hash unchanged. Final artifacts/limitations/reopening guidance:
`analysis/terminal-handoff.md`, `analysis/final-invariants.json`, commentary/audit.json.
Archive document selection refreshed separately; no archive or cleanup executed.
Existing TK-001 missing feature; no new issue/blocker. All phase workers completed.
