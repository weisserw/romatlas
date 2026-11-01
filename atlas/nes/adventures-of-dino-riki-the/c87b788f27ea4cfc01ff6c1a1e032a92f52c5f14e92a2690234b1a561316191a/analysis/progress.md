# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly in progress
Baseline established and verified byte-identical: maps/baseline.json,
generated/baseline, analysis/baseline-verification.json. Physical denominator 65552.
Configured release binary retained. ROM unchanged. Current live session traced 52
startup frames before a bounded 10-second execution timeout; partial state retained.
Vector discovery is candidate evidence only and not yet accepted. Runtime/scenarios,
classification and current checkpoint remain in progress; this is resumable, not exhaustion.
Durable RPC evidence: analysis/round-001/rpc.jsonl.

### Batch 1 checkpoint
Natural title identified Adventures of Dino Riki (Hudson US title screen); natural
Start, stage 1-1 introduction, movement and shooting observed. Clean gameplay snapshot
exported at analysis/round-001/clean_gameplay.snapshot. Artificial stage selector
$48=4 with PC=$84FB/SP=$FF successfully initialized distinct green swamp stage;
RAM contract from init/progression decode, not guessed arbitrary RAM. Later
milestones remain active work. Accepted reviewed fixed-PRG closure: 4255 official
6502 instructions / 8662 code bytes, 40 physical runs. Explicit object-data barrier
prevents misleading fallthrough from always-nonzero object code into parameter arrays.
Header, CHR backing and vectors classified; state/current map durably saved.
Toolkit finding NES-DOC-001 records ignored legacy parameter names and workaround.

### Batch 2 checkpoint
ROM classification: 63.38% (41544 / 65552 bytes classified; 24008 bytes unknown).
8662 code bytes; 32882 other classified bytes. This is intermediate, pass ongoing.
Common stage initialization CPU $84FB consumes selector RAM $48 in domain 0..13;
physical $06C3-$06DA increments it and wraps $0E to 0 while incrementing $C0.
Clean snapshot at frame386 PC$8708/SP$FD; step completed an operation before save.
Artificial visits restore it, reassert buttons, patch $48, set PC$84FB/SP$FF,
run 220 untraced initialization/gameplay frames plus 12 traced mechanics frames.
Selectors 1/3/5 (boss states) and 6/8 (later stage substates) reached; captures and
RAM/state evidence under analysis/round-001/milestone-NN.*. Remaining visits active.
No natural reachability claim is made for patched visits. No toolkit blocker.

### Batch 3 checkpoint
ROM classification: 96.09% (62991 / 65552 bytes classified; 2561 bytes unknown).
This intermediate count precedes the final boss-dispatch correction and audio closure.
Inspected generic data batches include pitch/music/maps, palette/update pointers,
14-stage initialization records, sprite/layout/movement data and presentation/audio
records; specific schemas remain partial. Natural and artificial reachability remain
separate. Added 1348 code bytes from later gameplay closure. Whole-game milestone
captures verify all four landscapes/substages and four distinct bosses. Final boss
hit-array shortcut did not complete; restoring final_boss and setting defeated flag
$A5=$84 then running12 traced+480 untraced frames returned to first landscape,
$48=0 and loop counter $C0=1. No credits claimed. Evidence: loop-confirmed.ppm and
RPC. Current batch: statically validate complete 44-entry sound-dispatch contract
(indexed return-minus-one pointers, bounded even indices <88), then checkpoint.
Review caught $2AFE false code from an impossible LDY#2/BNE fallthrough; corrected
3 code bytes to eight-byte split boss render dispatch data. Historical candidate
listing retained as candidate evidence, not accepted truth. Phase remains in progress.

## Round 001 — disassembly phase completed; ROM incomplete

ROM classification: 100.00% (65551 / 65552 bytes classified; 1 bytes unknown).
Exact percentage: 99.99847449353186%. Rounded 100.00% does not establish completion.
Single unknown: physical$39C7, CPU$B9B7, bank1, one byte; L005 ready for redirect.
Checkpoint: maps/round-001-disassembly.json; generated/round-001-disassembly;
analysis/round-001-disassembly-verification.json. Internal map validation passed,
stock JSON validation/generation passed, fresh ca65 rebuild byte-identical.
Input SHA256 remains c87b788f27ea4cfc01ff6c1a1e032a92f52c5f14e92a2690234b1a561316191a.

| Physical bank | Code | Data/image/header | Unknown |
| --- | ---: | ---: | ---: |
| header | 0 | 16 | 0 |
| PRG0 | 6867 | 1325 | 0 |
| PRG1 | 4970 | 3221 | 1 |
| PRG2 | 0 | 8192 | 0 |
| PRG3 | 0 | 8192 | 0 |
| CHR0 | 0 | 8192 | 0 |
| CHR1 | 0 | 8192 | 0 |
| CHR2 | 0 | 8192 | 0 |
| CHR3 | 0 | 8192 | 0 |

150/150 regions have role/evidence text and labels;11837 code bytes, 53714 other
classified bytes. Provenance is Static; natural/artificial execution is recorded
separately.3353 unique physical opcode starts fetched (2421 natural,3066 artificial,
932 artificial-only); 5487 aggregate rows include provenance duplicates. Detail
capacity 0 intentionally drops detail; aggregate_dropped 0. No full execution claim.

Lead outcomes L001–L004 verified; L005 ready unresolved. Generic data classifications
are structurally complete despite partial semantics; do not treat their meaning as
unknown classification. Review data annotations if later contradictory code emerges.
Broad progression visited representative stages, all 4 boss types and final loop;
exact inputs/patches/budgets/snapshots in scenarios.json. No further repetitive play
needed for this pass. This is an ordinary phase handoff, not terminal exhaustion.

Readability: structural labels/evidence only. Commentary must review all current
source: all 11837 code bytes plus 53714 data/header/image bytes and current labels;
numeric RAM references and broad code labels remain. No full readability/editorial
acceptance claim. Toolkit: 1 confirmed documentation/parameter-validation bug
NES-DOC-001,0 suspected bugs,0 separate missing features,0 blockers; see
analysis/toolkit-issues.md. Configured toolkit commit 0e58894598c67a60e344092fed1d3545873b4749.
Disk at handoff 9.7 GiB+ free; scratch4 KiB; durable round evidence under 64 MiB.

## Round 001 — commentary phase started
Prepared commentary with supported scripts/commentary.py prepare; frozen baseline
matches round-001-disassembly and preparation rebuild is byte-identical. No prior
commentary existed. Entire current-source inventory remains pending: physical
$0000-$1000F, including 11837 code bytes and 53714 classified non-code bytes;
sole unknown $39C7 will retain an explanatory boundary note. Working source is
commentary/source/game.asm. Final audit remains reserved for later acceptance.

Commentary batch 1 persisted source, symbol include and research notes. Reviewed
startup/background/player/map animation/enemy handlers through CPU $9FBA;
fresh commentary verification passed byte-identical. Remainder CPU $9FC2-$BCB0,
all data descriptions, local branch refinement and inventory remain in this phase.

Commentary batch 2 persisted through final code at CPU $BCCB: all enemy/boss/HUD,
presentation, sound effect and music mechanics now have functional notes. A fresh
commentary verify passed byte-identical. Consolidation remains in progress: data
names/notes, local branch naming, audio RAM symbols and review inventory. No final
audit or redirect performed.

## Round 001 — commentary phase complete; ROM incomplete
All currently classified source is readable: 11837 code + 53714 non-code bytes.
commentary/source/game.asm and ram.inc/data.inc provide 295 functional notes,
consistent routine/local branch/RAM/field/table names and data format explanations.
151 scaffold entries and 350 units reviewed, including an honest note for unknown
physical $39C7; classification remains 65551/65552. No readability ranges deferred.
Fresh commentary/verification.json passed byte-identical; commentary/reading-after.asm
refreshed. ROM and baseline map/state identities unchanged. Final audit pending;
no audit, toolkit edits, archive, publication or redirect performed.
Compact handoff: analysis/handoffs/round-001-commentary.md. No new toolkit findings;
existing report remains 1 confirmed documentation/parameter issue, 0 suspected,
0 separate missing features, 0 blockers (analysis/toolkit-issues.md).

## Round 001 — redirect complete; coordinator rejection review pending
Attempt analysis/redirect/round-001-a reviewed sole unknown physical$39C7/CPU$B9B7
in fixed banks[0,1,2,3], wrapper/callee stack contract, all decoded direct references,
encoded return targets, existing dispatch contracts and12 comparison JMP/RTS pairs.
No supported code/data distinction emerged; generic data fallback considered and
rejected only because this isolated$60 lacks independent non-code evidence.
L005 blocked with history preserved. L006 verified static semantic work: full shared
music scheduler across9 initialized tracks repeats within2561 modeled frames and
uses no EF-F7 commands; superseded channel-independent model errors retained.
Direct source uncertainty/generic-label triage and broad progression ledger reviewed.
No ready classification proposal, no interrupted frontier;65551/65552 unchanged.
Compact primary rejection review: analysis/redirect/round-001-a/rejection-review.md.
Handoff: analysis/handoffs/round-001-redirect.md. Coordinator must independently
inspect stopping rationale. ROM/maps/state/source hashes preserved; final editorial
audit pending. No toolkit edits, publication, deletion or runtime state mutations.

## Round 001 — final editorial acceptance completed; terminal incomplete retained
Coordinator accepted redirect's scoped rejection rationale. Targeted final review
reused all151 scaffold/350 unit reviews; corrected audit-relative source locations
and refreshed line numbers, and updated residual/music comments from redirect.
commentary/editorial-audit.md records substantive findings. editorial_audit now
reviewed. Supported commentary audit passed and a separate fresh verify confirmed
byte identity; reading copy refreshed. Exact preservation checks in
analysis/final-editorial/round-001-checks.json; ROM/map/state/baseline and executable
assembly statements unchanged. Source hash ddb4ecd2a2f11eaf546b1f9b1cdd52d541df4681406415d7dda406a1bec56c46.
Scope:11837 code+53714 non-code bytes readable; sole Unknown physical$39C7 remains,
65551/65552 classified. No pending readability ranges or final editorial items.
Compact handoff analysis/handoffs/round-001-final-acceptance.md. No new toolkit
issues; existing report1 confirmed,0 suspected,0 separate missing features,0 blockers.
No classification/toolkit changes, deletion, archive execution or publication.

Current coordinator status supersedes the earlier historical 'rejection review
pending' heading: terminal-incomplete rejection rationale accepted in
analysis/coordinator/rejection-review-acceptance.md. Remaining orchestration is
coordinator document/archive-options selection and final requirements audit.
All region/unit source references now resolve relative to commentary/source;
reading-after.asm is refreshed from the final accepted source.

## Coordinator terminal handoff — justified incomplete
Sequential round001 phases and final acceptance completed. Coordinator inspected primary rejection evidence and accepted terminal incomplete; no ready lead or productive interrupted frontier remains. ROM classification: 100.00% (65551 / 65552 bytes classified; 1 byte unknown), exact99.99847449353186%. Unknown$39C7 remains; ROM is not complete. Current map/store validation passed, source hashes match fresh verification, final reading copy matches source, all151 entries/350 units reviewed. Final handoff analysis/handoffs/terminal.md; final requirement evidence analysis/coordinator/completion-audit.json. Archive document selection is explicit in archive-options.json and accounted in analysis/coordinator/archive-selection-audit.json. No archive, cleanup, toolkit edit or publication executed.
