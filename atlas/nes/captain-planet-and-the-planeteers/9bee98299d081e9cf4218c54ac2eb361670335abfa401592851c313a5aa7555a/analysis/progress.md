# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Coordinator log

Coordinator: goal.md workflow (disassembly -> commentary -> redirect, repeat on ready leads).
Each phase runs in a separate sequential worker; workers leave compact handoffs in
analysis/handoffs/round-NNN-<phase>.md and append their own sections below.

| Round | Phase | Status | Input checkpoint | Output checkpoint | Handoff | Notes |
|---|---|---|---|---|---|---|
| 1 | disassembly | complete pass (incomplete ROM) 2026-11-23 | baseline | pass-001 | analysis/handoffs/round-001-disassembly.md | ROM classification 92.64% (242872/262160; 19288 unknown); leads L001-L006 |
| 1 | commentary | COMPLETE 2026-11-23 (after gate resume) | pass-001 | commentary/verification.json ok/byte_identical | analysis/handoffs/round-001-commentary.md | 0 raw RAM operands, 0 generic @skip/@loop; 139 Unk_* mechanical RAM names; reading copy refreshed |
| 1 | redirect | COMPLETE 2026-11-23 NEXT=disassembly | pass-001 + commentary | analysis/redirect/r001-a/ | analysis/handoffs/round-001-redirect.md | ready leads L001,L002,L003,L004,L005(check),L007,L008(low); L006 superseded; ~18.97 KiB of 19288 unknown |
| 2 | disassembly | complete pass (incomplete ROM) 2026-11-23 | pass-001 | pass-002 | analysis/handoffs/round-002-disassembly.md | ROM classification 100.00% rounded (262155/262160; 5 unknown); L001-L005,L007 accepted, L008 mostly accepted, L009 produced (5 B) |
| 2 | commentary | COMPLETE 2026-11-23 | pass-002 | commentary/verification.json ok/byte_identical | analysis/handoffs/round-002-commentary.md | round-1 commentary preserved at analysis/round-1/commentary/; 0 raw RAM/generic labels |
| 2 | redirect | COMPLETE 2026-11-23 NEXT=disassembly | pass-002 + commentary | analysis/redirect/r002-a/ | analysis/handoffs/round-002-redirect.md | leads L009a/L009b (consistency rule for lone rts/jmp incl. bank 6 $A175/$A18B), L010 (label fix, data), L011 (commentary fixes); game over, L3/L4 bosses, ending visited via RAM shortcuts. INCIDENT: worker scratch cleanup ran rm -rf /tmp/*.json (may have removed non-workspace /tmp files of other tasks); no workspace evidence affected |
| 3 | disassembly | complete pass (incomplete ROM) 2026-11-23 | pass-002 | pass-003 | analysis/handoffs/round-003-disassembly.md | ROM classification 100.00% rounded (262156/262160; 4 unknown lone RTS: $0C185,$0C19B,$1FB79,$1FDD0); L009a accepted, L009b rejected, bank-6 pair returned to Unknown, L010 relabeled |
| 3 | commentary | COMPLETE 2026-11-23 | pass-003 | commentary/verification.json ok/byte_identical | analysis/handoffs/round-003-commentary.md | round-2 commentary preserved at analysis/round-2/commentary/; L010/L011 applied |
| 3 | redirect | COMPLETE 2026-11-23 NEXT=terminal | pass-003 + commentary | analysis/redirect/r003-a/ | analysis/handoffs/round-003-redirect.md | no ready leads; rejection review for 4 lone-RTS bytes; coordinator inspected review + primary bytes and accepted terminal incomplete (see final-coordinator.md). Gameplay: L0 boss defeat, L1 barrier/clear, L2 CP stage visited via RAM shortcuts |
| 3 | commentary (follow-up) | COMPLETE 2026-11-23 | pass-003 | commentary/verification.json ok/byte_identical (game.asm c74a7949...) | analysis/handoffs/round-003-commentary.md | 2184/2184 units; only final editorial audit pending (reserved for archive) |
| - | coordinator | TERMINAL INCOMPLETE 2026-11-23 | pass-003 | - | analysis/handoffs/final-coordinator.md | ROM classification: 100.00% (262156 / 262160 bytes classified; 4 bytes unknown). Rejection review accepted; archive-options.json written; not ROM completion |
| 4 (batch exception) | manual fallback | COMPLETE 2026-11-23 | pass-003 | pass-004 | analysis/progress.md (Pass-004 section) | USER-DIRECTED: 4 residual bytes -> Data, Manual provenance, unknown meaning (not evidence-backed); 0 unknown |
| 4 (batch exception) | commentary reconcile + final audit | COMPLETE 2026-11-23 | pass-004 | commentary/audit.json ok/byte_identical (game.asm c3041b93...) | analysis/handoffs/round-004-commentary-final.md | round-3 commentary preserved in analysis/round-3/commentary/; strict audit passed 1940/1940 regions, 2184/2184 units; coordinator re-ran audit + editorial_status complete |
| - | coordinator | FINAL 2026-11-23 | pass-004 | - | analysis/handoffs/manual-fallback-record.md; final-coordinator.md addendum | ROM classification: 100.00% (262160 / 262160; 0 unknown) = 262156 evidence-backed + 4 user-directed Manual fallback. archive-options.json updated; no archive/cleanup performed |

## Round 1 — disassembly pass-001 (2026-11-23)

ROM classification: 92.64% (242872 / 262160 bytes classified; 19288 bytes unknown).

Status: incomplete (single easy-gain pass). Baseline created (generated/baseline, byte-identical). Checkpoint pass-001:
maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json (ok, byte_identical true).

Kinds: Code 36564, Data 73185, Text 2051, Image 131072 (CHR ROM). Per region (classified / unknown):
| Area | Code | Data/Text | Unknown |
|---|---|---|---|
| header | 0 | 16 | 0 |
| PRG 0,1,2,4,5,9,10,11 | 0 | 8192 each | 0 |
| PRG 3 | 5292 | 1462 | 1438 |
| PRG 6 | 4935 | 1995 | 1262 |
| PRG 7 | 7026 | 408 | 758 |
| PRG 8 | 1230 | 205 | 6757 |
| PRG 12 | 5092 | 2788 | 312 |
| PRG 13 | 992 | 88 | 7112 |
| PRG 14 | 6284 | 1486 | 422 |
| PRG 15 | 5713 | 1252 | 1227 |
| CHR | 0 | 131072 (Image) | 0 |

Evidence: observed opcode fetches (20656 contextual roots) from boot/title/credits, attract demo, password screen,
all ten password stages (Geo-Cruiser L2-L5 traced, L1 and L4 Captain Planet gameplay traced, ending traced) plus static
traversal (tools_local/trav.py) and conservative gap classification; see analysis/investigation.md.
Scenarios: analysis/scenarios.json; gameplay notes: analysis/gameplay.md. Leads produced: L001-L006 (analysis/leads.json).
Toolkit issues: TI-001..TI-004 in analysis/toolkit-issues.md (1 missing feature blocking toolkit discovery on MMC3, worked around; 2 confirmed API/doc bugs; 1 performance).
Handoff: analysis/handoffs/round-001-disassembly.md.

## Round 1 — commentary (2026-11-23)

Status: COMPLETE (current-source readability for all classified source; ROM classification unchanged at 92.64%).
Input: pass-001 (maps/current.json identical). Prepared commentary/ (frozen before/, baseline.json, phase1-state.json).
Source commentary/source/game.asm regenerated from before/ + commentary/tools/ann/*.py by commentary/tools/build.py:
1132 descriptive global labels (530 with routine/block comments), ~150 RAM symbols + bank aliases, hardware/MMC3 names,
named JMP-stub tables, `.word` inline task/sprite arguments, notes on all 71 Unknown ranges. review.json: all 1568
scaffold entries reviewed, 1779 routine units; editorial_audit pending.
Verification: commentary.py verify ok, byte_identical true (commentary/verification.json); reading-after.asm refreshed.
Redirect questions (JMP-table code roots, SpawnTaskGroup entries, consumer-proven data tables, banks 8/13 spawn tables):
analysis/handoffs/round-001-commentary.md. No toolkit issues found in this phase.
Commentary gate follow-up (same phase): 0 raw RAM operands (139 mechanical Unk_<owner>_<addr> symbols, role uncertain),
0 generic @skip/@loop local labels (condition/iteration names; 20 flag-only); re-verified byte_identical true, reading copy refreshed.

## Round 1 — redirect attempt r001-a (2026-11-23)
Input checkpoint pass-001 (unchanged; redirect changes research/lead records only). Attempt dir analysis/redirect/r001-a/ (README.md).
Method: entry-table decoding (bank 8/13 spawn/init words, bank 3/6 handler tables, $F1E6/$F3D9), bounded static traversal, overlap/extent checks,
artificial password/task-start runtime corroboration. Result: ready leads L001-L005, L007, L008 (L006 superseded); projected +~12.1 KiB code and
~6.6 KiB data/text of the 19288 unknown bytes; ~314 bytes left for review. Gameplay: L3 and L5 Captain Planet gameplay, level-clear password screen
and credits scene visited (artificial). Handoff: analysis/handoffs/round-001-redirect.md. Recommendation NEXT=disassembly.

## Round 2 — disassembly pass-002 (2026-11-23)

Status: pass complete; ROM incomplete (5 unknown bytes remain).
Input: analysis/state.json (pass-001, loaded with map.load; no new baseline). Previous internal state kept as analysis/state.pass-001.json.gz.
Method: re-ran the redirect r001-a traversal (analysis/redirect/r001-a/tools/trav2.py, EXTRA=roots_all.json; output identical to
runs/trav_r001a_all.json.gz; 0 overlaps except the two L004 corrections), independently checked the entry contracts (bank 8 spawner $B0F2-$B196
writes TaskBank 8 after CreateTaskAt; bank 13 reader $A62F/$A659; bank 3/6 JMP ($0000) dispatchers and table bounds), confirmed the residual
ledger matches the post-traversal complement exactly (100 rows), then applied everything with tools_local/pass002_annotate.py (map.annotate/map.remove).

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).

(100.00% is rounded: 5 bytes remain unknown, so the ROM is not complete.)

Totals: Code 48907 (Observed 18178, Static 30729), Data 80115, Text 2061, Image (CHR) 131072, Unknown 5; 1933 regions.

| Bank | Code | Data/Text | Unknown |
|---|---|---|---|
| header | 0 | 16 | 0 |
| PRG 0,1,2,4,5,9,10,11 | 0 | 8192 each | 0 |
| PRG 3 | 6667 | 1525 (13 Text) | 0 |
| PRG 6 | 5704 | 2488 | 0 |
| PRG 7 | 7032 | 1160 | 0 |
| PRG 8 | 5928 | 2264 | 0 |
| PRG 12 | 5141 | 3051 (2048 Text) | 0 |
| PRG 13 | 6109 | 2080 | 3 ($BA5D-$BA5F) |
| PRG 14 | 6376 | 1816 | 0 |
| PRG 15 | 5950 | 2240 | 2 ($FB69, $FDC0) |
| CHR | 0 | 131072 (Image) | 0 |

Lead outcomes (analysis/leads.json): L001 verified (+banks 8/13 task code), L002 verified (+banks 3/6 handler code; SpecialObjectHandlers
$A3BD-$A3CC and EventObjectHandlers $A2FE-$A331 labelled), L003 verified ($F1E6/$F3D9 code + bank 14/15 data), L004 verified (regions 1404/1437 split,
30 B Data->Code), L005 verified (validation, no byte change), L007 verified (6634 B data + 10 text bytes merged into briefing text), L008 partly
accepted (256 B code, 52 B data; 5 B rejected -> new low-priority L009). L006 stays superseded.
Gameplay: one bounded L5 Captain Planet progression attempt (snapshot l5cp_start.json, ~3600 untraced frames); no boss/game over reached, no new task
code (analysis/scenarios.json). Execution coverage is a separate metric: Observed code 18178 B of 48907 B.
Verification: python3 run.py checkpoint pass-002 -> analysis/pass-002-verification.json ok true, byte_identical true
(map maps/pass-002.json sha256 295a9b18...; generated/pass-002). maps/current.json exported; map.validate ok.
Toolkit: no new toolkit issues found this pass (TI-001..TI-004 still apply; analysis/toolkit-issues.md).
Handoff: analysis/handoffs/round-002-disassembly.md.

## Round 2 — commentary (2026-11-23)

Status: COMPLETE for current-source readability on pass-002 (not ROM completion; 5 Unknown bytes remain).
Round-1 commentary preserved unchanged in analysis/round-1/commentary/; fresh baseline prepared from maps/pass-002.json.
Carried round-1 names/comments by physical address; documented all pass-002 code (banks 8/13 CP object tasks, bank 3/6 handler
tables, fixed-bank level-clear/credits tasks, L008/L004 code) and data tables.  Residuals: 0 address-only globals, 0 raw RAM
operands, 0 generic locals, 130 Unk_* RAM symbols with uncertainty notes.
Verification: commentary.py verify ok=true, byte_identical=true; reading copy refreshed.
Handoff: analysis/handoffs/round-002-commentary.md.  No toolkit issues found in this phase.

## Round 2 — redirect attempt r002-a (2026-11-23)

Input checkpoint pass-002 (unchanged; 5 Unknown bytes). Research only; ROM/map/state/source/binary untouched.
- L009 reopened with combined evidence (negative consumer search incl. indexed bases, non-alignment, exact duplicate jmp,
  consistency conflict with pass-002's accepted bank 6 lone-RTS stubs $0C185/$0C19B); split L009a/L009b, ready (low).
- L010 new: bank 8 $B20E-$B212 = empty 1x1 sprite def referenced 10x by TransformFrames (data role; region already Data).
- L011 new: commentary corrections ($B0CA -> CpStageBank:$A015 CreateInitialObjects = boss-zone objects; game-over vs
  ending task names; RingLevel = lives; EndingFlag values; Level3BossTask loss condition).
- Soundness review: no control-flow target or observed/executed opcode inside Data/Text; 4499 executed offsets all at Code
  instruction starts. L008 routines not executed in any route.
- Gameplay: real game over + CONTINUE/END menu, level-3 boss arena, level-4 boss + victory/ending, level-1 CP stage
  visited (artificial shortcuts; scenarios.json).
Handoff: analysis/handoffs/round-002-redirect.md (NEXT=disassembly). Attempt: analysis/redirect/r002-a/.

## Round 3 — disassembly pass-003 (2026-11-23)

Input: analysis/state.json (pass-002, map.load; no new baseline). Tool: tools_local/pass003_annotate.py (map.annotate/map.remove).
Scope: redirect r002-a ready leads L009 (a/b + bank-6 consistency) and L010; L011 left for commentary. No gameplay run (redirect already
visited game over, L3/L4 bosses and ending; no unvisited milestone offered a classification gain).

ROM classification: 100.00% (262156 / 262160 bytes classified; 4 bytes unknown).

(100.00% is rounded: 4 bytes remain unknown, so the ROM is not complete.)

Totals: Code 48908 (Observed 18178, Static 30730), Data 80115, Text 2061, Image (CHR) 131072, Unknown 4; 1936 regions.

| Bank | Code | Data/Text | Unknown |
|---|---|---|---|
| header | 0 | 16 | 0 |
| PRG 0,1,2,4,5,9,10,11 | 0 | 8192 each | 0 |
| PRG 3 | 6667 | 1525 | 0 |
| PRG 6 | 5702 | 2488 | 2 ($A175, $A18B) |
| PRG 7 | 7032 | 1160 | 0 |
| PRG 8 | 5928 | 2264 | 0 |
| PRG 12 | 5141 | 3051 | 0 |
| PRG 13 | 6112 | 2080 | 0 |
| PRG 14 | 6376 | 1816 | 0 |
| PRG 15 | 5950 | 2240 | 2 ($FB69, $FDC0) |
| CHR | 0 | 131072 (Image) | 0 |

Lead outcomes (criterion, applied to all candidates): unreferenced, never-fetched bytes are Static Code only with an independent structural
constraint beyond legal decoding (operand resolving to an established code entry in the same effective bank, or a coherent multi-instruction
routine contract); a lone single-byte RTS lacks it (evidence-policy.md), so it stays Unknown.
- L009a verified: bank 13 $1BA6D-$1BA6F `jmp $B4C2` -> Static Code (target DebrisSpawner entry; duplicate of jmp at $BA59; reachability not claimed).
- L009b rejected: bank 15 $1FB79, $1FDD0 stay Unknown (consumers.py rerun: no hits).
- Consistency correction: region 1853 split; bank 6 $0C185/$0C19B (lone RTS, pass-002's bank-3 parallel nonexistent) returned to Unknown.
  Net: +3 -2 = +1 byte classified (5 -> 4 unknown).
- L010 verified: region 1500 split; $1121E-$11222 Data `EmptySpriteDef8` (10 TransformFrames words, 5-byte sprite-def format, shared levels 3-4
  empty spawn list byte). 0 B change. L011 commentary-only (untouched).
Verification: python3 run.py checkpoint pass-003 -> analysis/pass-003-verification.json ok true, byte_identical true
(maps/pass-003.json sha256 ba45ade7...; generated/pass-003). map.validate ok; analysis/state.json saved; maps/current.json exported.
Toolkit: no new toolkit issues found this pass (TI-001..TI-004 unchanged; analysis/toolkit-issues.md).
Handoff: analysis/handoffs/round-003-disassembly.md.

## Commentary round 3 (pass-003) - COMPLETE
Map changed since the round-2 baseline, so round-2 commentary was preserved whole in analysis/round-2/commentary/ and a fresh
baseline prepared (1940 entries).  Names/comments carried by physical address; the 3 changed ranges re-annotated (bank 6 lone RTS
bytes as Unknown notes, bank 13 PusherTask_deadJump, bank 8 EmptySpriteDef8).  L010 and L011 applied (boss-zone $A015 task word,
CreateBossZoneObjects/BossZoneTasks, StageEndDispatch/StartGaiaMessage/GaiaMessageTask/GameOverMenuTask, Lives, StageEndFlag,
Level3BossTask escape/defeat); 23 owner+address labels and several stale notes fixed.  Residuals: 0 raw RAM operands, 0 generic
locals, 0 address-only globals, 109 Unk_* RAM with notes.  commentary.py verify ok/byte_identical (game.asm 5f4f505d...); reading
copy refreshed.  Handoff: analysis/handoffs/round-003-commentary.md.

## Redirect round 3 (attempt r003-a, input pass-003) - TERMINAL (incomplete)
Research only (ROM/map/state/source/binary unchanged). Rejection review of the 4 lone-RTS Unknown bytes ($0C185, $0C19B,
$1FB79, $1FDD0): no consumer (consumers4.py), no module-copy parallel, $60 not a fill value, never fetched -> neither code nor
generic data supported; remain Unknown. Gameplay (artificial setup): level-0 boss zone + target defeated by played A hits,
level-1 goal barrier/minion + clear, level-2 CP stage + goal rule; 0 contradictions (cov_r003a.json). No ready leads.
Handoff: analysis/handoffs/round-003-redirect.md; record analysis/redirect/r003-a/README.md. NEXT=terminal.

## Pass-004 — USER-DIRECTED MANUAL FALLBACK (batch exception, 2026-11-23)

**Not evidence-backed classification.** The user explicitly authorized a batch-only exception for at most 10 residual
bytes after the terminal-incomplete round-3 handoff. Verified complement first: analysis.unknown on analysis/state.json
(pass-003) returned exactly 4 one-byte ranges = the authorized list: file offsets 49541 ($0C185, bank 6 $A175),
49563 ($0C19B, bank 6 $A18B), 129913 ($1FB79, bank 15 $FB69), 130512 ($1FDD0, bank 15 $FDC0). Each is a lone $60.
Prior ambiguity (preserved, unchanged): r003-a rejection review + final-coordinator.md — no reference, never fetched,
not alignment/fill; a lone unreferenced RTS lacks structure for Code and nothing positively supports Data.
Action: tools_local/pass004_manual_fallback.py annotated each as kind Data, provenance **Manual**, comment
"Unknown meaning; user-directed manual fallback (not evidence-backed)". No semantics/labels invented; ROM unchanged.
Prior state kept as analysis/state.pass-003.json.gz. map.validate ok; map.save analysis/state.json; maps/current.json exported.
Checkpoint: python3 run.py checkpoint pass-004 -> analysis/pass-004-verification.json ok true, byte_identical true
(maps/pass-004.json sha256 3debb849...; generated/pass-004).

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown) — of which 262156 bytes are
evidence-backed (Observed/Static/structural) and 4 bytes are the user-directed Manual Data fallback above.
Regions 1940 (Code 1447, Data 483, Text 9, Image 1); provenance Static 1326, Observed 610, Manual 4.
Next: commentary reconciliation of the 4 changed ranges + final review and strict editorial audit.

## Round 4 — commentary reconciliation + final editorial audit (pass-004, 2026-11-23) - COMPLETE
Round-3 commentary preserved whole in analysis/round-3/commentary/; fresh baseline prepared from pass-004 (1940 entries).
Names/comments/units carried by physical address (tools/mapdiff.py: only the 4 one-byte Unknown -> Manual Data changes).
The 4 bytes ($0C185, $0C19B, $1FB79, $1FDD0) are documented as generic Data, meaning unknown, user-directed Manual
fallback (not an observed/statically proven role; prior code-vs-data ambiguity kept in uncertainties.md). Editorial fixes:
symbolised RAM prose, MapScrollLeft/Down/Up, SpawnLog/SpawnLogLo aliases, BriefingSpeakerIndex, SetPriorityIfInWater, review summaries.
`commentary.py audit` PASSED: ok, byte_identical, 1940/1940 entries, 2184/2184 units, editorial_audit reviewed (game.asm c3041b93...);
archive.editorial_status complete. Handoff: analysis/handoffs/round-004-commentary-final.md.

## Coordinator final (2026-11-23, batch exception)
Coordinator independently re-ran `commentary.py audit` (ok, byte_identical, 1940 regions / 2184 units, editorial_audit true),
archive.editorial_status (complete, no reasons), analysis.unknown on analysis/state.json (0), maps/current.json == maps/pass-004.json ==
commentary baseline map (sha256 3debb849...). ROM sha256 unchanged. archive-options.json now also selects round-004 commentary handoff,
manual-fallback-record.md and round-3 commentary progress/uncertainties; omits round-3 copied instruction templates. Every Markdown file
under analysis/ and commentary/ is accounted for. No archive, cleanup, publishing or toolkit change performed.
