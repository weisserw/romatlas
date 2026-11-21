# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 disassembly active
Baseline established and byte-identical: maps/baseline.json, generated/baseline,
analysis/baseline-verification.json. No input leads. Mapper 0, 16 KiB PRG,
8 KiB CHR. Validated reset $FD24 and NMI $FF7F, banks [0,1,0,1].
Initial static discovery: 715 instructions, two indirect-dispatch frontiers.
Runtime title identifies BurgerTime; startup reached frame 179, 667 unique
instruction roots. Work active; baseline itself is not classification.

### Saved first classification batch
Internal state saved and validated; maps/current.json exported. 17,261 / 24,592
bytes classified (70.19%), 7,331 unknown: 9,033 Code, 8,192 Image, 36 Data.
46 code intervals from reviewed 4,179 instructions. Natural title/menu/first-stage
ready route and first level rendering seen. First RAM completion-flag shortcut
was attempted during initialization and produced blank transition/ready state;
not counted as a successful later-stage visit. A second supported dispatcher
initialization entry experiment is in progress; exact recipe will be saved.
Expanded indirect handlers supplied 1,064 additional inspected instructions;
classification pending insertion. No input leads existed.

### Verified checkpoint pass-001
ROM classification: 99.94% (24577 / 24592 bytes classified; 15 bytes unknown).
Internal map: Code 11296; Data 5089; Image 8192. All classified intervals have
Static structural evidence; execution coverage is separately retained in traces.
Map validation passed; maps/pass-001.json and generated/pass-001 rebuild
byte-identically (analysis/pass-001-verification.json).
Successful artificial sixth-stage initialization confirmed actual rendered layout,
$0400=6 and $0402/$0403=5; 500-frame trace-disabled settling followed by traced
mechanics experiment. Full reproducible recipe and final per-bank counts pending
end-of-pass record. Residuals [0x5D,0x5E), [0x3485,0x3488), [0x3609,0x3614).

## Round 1 final checkpoint and handoff
ROM classification: 99.94% (24577 / 24592 bytes classified; 15 bytes unknown).

| Physical section | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| Header [0,16) | 0 | 16 | 16 | 0 |
| PRG 8 KiB bank0 [16,8208) | 6755 | 1436 | 8191 | 1 |
| PRG 8 KiB bank1 [8208,16400) | 4541 | 3637 | 8178 | 14 |
| CHR bank0 [16400,24592) | 0 | 8192 | 8192 | 0 |

Totals Code11296, Data5089, Image8192. 109 evidence-bearing regions; no overlaps,
no explicit Unknown regions counted. Structural classification incomplete:
[0x005D,0x005E), [0x3485,0x3488), [0x3609,0x3614). Execution metric:
3863 unique accepted instruction starts /8167 fetched instruction bytes, mixed
natural/artificial scenarios; aggregate_dropped0. Details intentionally discarded
with trace capacity0; trace-disabled settling clearly separated in scenarios.

Fresh checkpoint maps/pass-001-final.json, generated/pass-001-final,
analysis/pass-001-final-verification.json passed map validation and byte-identical
rebuild. Saved state analysis/state.json and maps/current.json match checkpoint.
ROM hash unchanged; configured binary unchanged. No toolkit issues found;
[report](toolkit-issues.md). No blockers.

Visited natural startup/menu/first layout and artificial third/sixth layouts with
movement/pepper inputs; artificial GAME OVER screen verified. Six-layout loop
proved statically; no separate ending or boss contract identified. Successful
snapshots and exact patches retained in scenarios.json. Failed premature
completion-flag shortcut superseded with supported stage-init dispatcher contract.
Leads R01-001..003 defer small independent structural contract reviews to redirect;
no terminal exhaustion claim. Commentary has current 5243-instruction source to
make fully readable before redirect. Full evidence and per-lead rationale in
[investigation](investigation.md); compact [handoff](round01/disassembly-handoff.md).
Disk3.3GiB free, scratch80KiB, selected evidence36MiB; no deletion/cleanup done.

## Round 1 commentary active
Prepared commentary/ from verified pass-001-final, preserving frozen before/ and phase1-state.json.
Read complete current-source inventory (5,745 lines); drafting names/contracts for title/main state,
player/enemy/ingredient mechanics, PPU transfers, software BCD scoring and audio interpreters.
All entries remain pending until edited source passes alignment/consistency review.
Maps/state/ROM unchanged; no redirect begun. See commentary/progress.md for current frontier.

## Round 1 commentary active
Prepared commentary/ from verified pass-001-final, preserving frozen before/ and phase1-state.json.
Read complete current-source inventory (5,745 lines); drafting names/contracts for title/main state,
player/enemy/ingredient mechanics, PPU transfers, software BCD scoring and audio interpreters.
All entries remain pending until edited source passes alignment/consistency review.
Maps/state/ROM unchanged; no redirect begun. See commentary/progress.md for current frontier.

## Round 1 commentary complete — commentary-round01-final
Current-source readability gate completed across all currently classified code/data
and explained unknowns. All 112 scaffold entries and 322 units reviewed (61 code
intervals / 11,296 bytes, 48 non-code/image intervals / 13,281 bytes, three unknown
intervals / 15 bytes). 274 routine/block contracts, 727 control labels and 387
address symbols cover main state/title, PPU/NMI, player/pepper, entity/geometry,
enemy priorities, ingredients/queues, bonus/scoring and audio interpreters. PRG
payloads are visible byte/word records; dispatch tables name handler symbols.

Fresh final `commentary.py verify` rebuilt and linked source with byte identity;
`commentary.py read` refreshed reading-after.asm afterward. Exact checkpoint/hash
record: [commentary-final-verification.json](round01/commentary-final-verification.json).
Source SHA-256 game.asm e2ff0601f9c346ef43966ca2cbb886628ac9b17f21b4ba250756b265f1132626;
symbols.inc e70b95c51d619fa74ef936ed0235a34e0b048664c72e66bf4125cfbb5888a65b.
Frozen baseline, maps/current.json, analysis/state.json and input ROM unchanged.
Structural checkpoint remains pass-001-final, 24,577/24,592 classified; unknown
[005D,005E), [3485,3488), [3609,3614). Final editorial audit remains pending.

[Compact handoff](round01/commentary-handoff.md) and
[uncertainties](../commentary/uncertainties.md) record exact audio flag contracts,
ingredient selector fallthrough, and four stale map-semantic annotations requiring
reconciliation without speculative structural changes. No remaining readability
frontier; next phase may investigate structural residuals. No redirect prompt was
read or executed. No toolkit issues found; [report](toolkit-issues.md). No toolkit
modifications, archive/publication or cleanup. Disk3.3GiB free, scratch80KiB.

## Round 1 redirect complete — combined ready batch
Input commentary-round01-final / pass-001-final. Unique attempt:
[round01-contract-review](redirect/round01-contract-review/findings.md).
Inspected all three exact residuals, connected audio helper/loader/music/control
contracts, final ladder record extent, full saved decoded references, exact PRG
address words and commentary's concrete semantic questions. Complete unknown
complement and neighbor dispositions persisted in attempt/unknown-ledger.json.

Updated R01-001/002 unresolved with localized competing interpretations; improved
R01-003 ready (+11 proposed Static code bytes) through independently supported
MusicRequest/helper/stack/shared-loader contract. Added R01-004 ready (+0 bytes):
four stale map semantic annotations reconciled against actual consumer strides,
indices and stores. No productive unfinished continuation; independent avenues
triaged and reasons documented in findings. Next phase round2 disassembly reviews
R01-003/004 together, then commentary and redirect. No terminal exhaustion claim.

Accepted classification unchanged: 24,577/24,592 **99.94%**, 15 unknown bytes:
[005D,005E), [3485,3488), [3609,3614). Proposed +11; if accepted four remain.
ROM/map/state/source hashes checked unchanged; fresh existing commentary verification
still applies. No map/source/checkpoint modifications in redirect. Compact
[handoff](round01/redirect-handoff.md); [leads](leads.json); all outcomes and recipes
persisted. No toolkit issues found; report unchanged. Disk3.3GiB free, scratch80KiB;
no cleanup, toolkit modification, archive or publication.

## Round 2 disassembly — independent ready-lead review

ROM classification: 99.98% (24588 / 24592 bytes classified; 4 bytes unknown).

Accepted R01-003: +11 Static code bytes, physical [3609,3614), CPU F5F9..F603,
fixed banks [0,1,0,1]. Music identity, mask-helper preservation/carry protocol,
exact loader convergence and balanced stack establish structural role;
natural reachability remains unresolved. Accepted R01-004 four consumer-backed
semantic metadata corrections, no boundary changes. R01-001/002 remain unresolved
at [005D,005E), [3485,3488); unchanged primary review reused without repetition.

Per-bank classified/unknown: Header16/0; PRG0 8191/1; PRG1 8189/3; CHR0 8192/0.
Code totals PRG0 6755, PRG1 4552; data/image 1436/3637 respectively.
[Independent review](round02/review.md), [exact counts](round02/counts.json),
[RPC evidence](round02/rpc-results.json), [handoff](round02/disassembly-handoff.md).
State/map saved and validated. Authoritative fresh checkpoint **pass-002-reviewed**
[verifies byte identity](pass-002-reviewed-verification.json), map SHA256
8e905a4bcaf69c2d53002bf4d9711944102b3fcc7bda6e47ebcdf14606b26422.
Superseded pass-002-final preserves unchanged pre-edit map; documented in review.

ROM/source/symbol hashes unchanged; commentary review and editorial work preserved.
Next commentary integrates new slice and semantic updates with valid review credit;
then redirect. Gameplay routes/checklist reused; no new concrete runtime question,
startup replay or execution claim. Classification incomplete, editorial gates pending.
No toolkit issues found; [report](toolkit-issues.md). Disk3.3GiB free, scratch80KiB.
No toolkit changes, cleanup, archive or publication.

## Round 2 commentary — complete current-source readability

Completed readability against pass-002-reviewed: all 112 current scaffold entries
and 323 units reviewed, carrying 322 valid earlier reviews. Integrated the eleven
new Static code bytes as CheckMusicReplacementPriority and reconciled four metadata
corrections. Corrected concrete prior initialization prose and stale aliases;
no deferred readability ranges. Full current inventory, code-unit coverage,
source changes, operand naming and review source locations checked.

Classification remains **24,588/24,592 (99.98%)**, four unknown bytes; natural
priority-entry reachability is unresolved. Fresh source rebuild byte-identical;
reading-after refreshed. Baseline/state/source mapping and review match the current
map. Entire prior commentary retained before baseline refresh; source never
regenerated. [Handoff](round02/commentary-handoff.md),
[review](round02/commentary-readability-review.json),
[verification](round02/commentary-final-verification.json).
Next redirect is outside this phase. Final editorial audit remains pending for
terminal acceptance. No redirect, toolkit changes, publication or cleanup ran.

## Round 2 redirect — localized terminal recommendation

Input/output pass-002-reviewed and completed round02 current-source commentary.
Unique attempt [round02-residual-review](redirect/round02-residual-review/rejection-review.md).
Retrieved decisive primary bytes/reference scan and current scrolling, ladder FF,
audio and progression contracts. Compared accepted priority-guard constraints and
latest commentary changes; unchanged localized ambiguity survives, without imposing
live callers as prerequisites. No new runtime replay or exhaustive ROM scan.
R01-001/002 history updated; R01-003/004 remain verified; produced no ready leads.

Exact complement freshly computed from current internal map: [005D,005E),
[3485,3488), **24,588/24,592 (99.98%)**, four unknown. Accepted/proposed +0;
map SHA2568e905a4bcaf69c2d53002bf4d9711944102b3fcc7bda6e47ebcdf14606b26422
and ROM hash unchanged. [Ledger](redirect/round02-residual-review/unknown-ledger.json),
[handoff](round02/redirect-handoff.md), [leads](leads.json). Readability complete and
fresh commentary byte identity remains valid; final editorial acceptance pending.

Recommendation incomplete, subject to independent coordinator primary-artifact
rejection audit. No unfinished productive frontier or justified discriminating
next test found; do not automatically retry unchanged residuals. Broad gameplay
checklist and ending obstacle reviewed. No toolkit/map/source/state edits, archive,
publication or cleanup. Disk3.3GiB free, scratch80KiB; cited evidence preserved.


## Coordinator terminal handoff — workflow complete, ROM classification incomplete

Independent rejection review accepted against decisive primary bytes/references and current scroll/ladder/audio source; see [review](coordinator-rejection-review.md). No credible corrected inference/test remains; no unfinished productive frontier or ready leads. This is justified terminal incomplete, not blocked or full-ROM success.

ROM classification: 99.98% (24588 / 24592 bytes classified; 4 bytes unknown).
Final checkpoint pass-002-reviewed; exact unknowns [005D,005E), [3485,3488).
Header 16/0, PRG0 8191/1, PRG1 8189/3, CHR0 8192/0 classified/unknown.
Independent current store/map validation passed; state/map identical, ROM unchanged.
Final editorial acceptance completed after redirect/coordinator review: 112 regions,
323 units, no pending readability ranges, fresh byte-identical rebuild and refreshed
reading copy. [Final editorial handoff](round02/editorial-final-handoff.md),
[coordinator checks](coordinator-verification.json), [completion audit](coordinator-audit.md).

Archive contract inspected read-only; archive-options.json selects additional final
Markdown and omits only redundant historical instruction copies. Every unrecognized
Markdown accounted by [selection check](document-selection-verification.json).
No archive execution, cleanup, publication, toolkit or input modification.
[Final handoff](final-handoff.md). No toolkit issues found; [report](toolkit-issues.md).

## Round 3 disassembly — explicit user fallback exception

ROM classification: 100.00% (24592 / 24592 bytes classified; 0 bytes unknown).

This assignment count comprises **24,588 evidence-backed bytes (99.98%) and four
user-authorized Manual fallback bytes**, not newly proven data. Fresh exact internal
complement was [{offset:93,length:1},{offset:13445,length:3}]. Only physical
[0x005D,0x005E) and [0x3485,0x3488) were annotated generic Data, unknown meaning,
Manual provenance through map.annotate under explicit <=10-byte batch fallback
authorization. Code/data ambiguity remains: isolated RTS versus non-code; possible
audio JMP wrapper versus unframed data. Historical evidence/rejections preserved;
no redirect research or new execution claims. This exception does not change the
ordinary evidence-backed classification contract for other bytes.

Per-bank assigned/unknown: Header16/0, PRG0 8192/0, PRG1 8192/0, CHR0 8192/0;
Manual fallback counts 0/1/3/0 respectively. All prior regions unchanged. State/map
saved, validated and equal; ROM and editorial source hashes unchanged. Fresh
checkpoint **pass-003-user-fallback** verified byte-identical;
[verification](pass-003-user-fallback-verification.json),
[fallback record](round03/fallback-record.json), [counts](round03/counts.json),
[handoff](round03/disassembly-handoff.md). Current-source commentary and final
strict editorial audit are pending the separate phase. No toolkit issues found;
[report](toolkit-issues.md). Disk3.3GiB free, scratch80KiB; no toolkit changes,
cleanup, archive or publication.

## Round 3 final coordinator acceptance — authorized fallback recovery complete

Sequential commentary completed after round03 disassembly. All current source is
readable: 112 inventory intervals and 323 meaningful units reviewed, zero pending.
Prior commentary was retained whole in round03/commentary-before-fallback-refresh/;
current baseline/state, manifest, source mapping and review match the updated map.
Only the two residual boundary notes changed in assembly; labels, instructions and
assets preserved. Final editorial review inspected the full inventory and changed
neighbor dependencies; strict supported audit passed with a fresh byte-identical
rebuild and refreshed reading copy. [Commentary handoff](round03/commentary-handoff.md),
[audit](round03/editorial-final-audit.json), [verification](round03/editorial-final-verification.json).

ROM map assignment: **100.00% (24592/24592; zero unclassified)**, explicitly
**24,588 evidence-classified + four user-directed Manual fallback bytes**.
No proven code/data meaning or observed execution is claimed for the fallback.
Exact authorization, ranges and prior ambiguity are retained in
[final fallback record](round03/fallback-record.md). Independent completion checks
verified allowed-only changes, before/after complement, native map/store validation,
current audit hashes, reading copy, checkpoint and complete editorial status:
[coordinator verification](round03/coordinator-verification.json).

Final checkpoint pass-003-user-fallback; ROM unchanged. Current authoritative
[final handoff](round03/final-handoff.md) and [requirement audit](round03/coordinator-audit.md)
supersede the historical terminal-incomplete signoff without removing evidence.
No new redirect phase/research was warranted under the explicit exception; original
redirect/rejection reviews and gameplay limits remain retained. No known blocker remains.
archive-options.json includes the final fallback and all required historical/final
Markdown; [contract-equivalent check](round03/document-selection-verification.json)
accounts for every unrecognized document. Six redundant frozen instruction copies
are omitted only from future archive selection, still on disk. No archive, cleanup,
deletion, publication, ROM or toolkit edits. No toolkit issues found.
