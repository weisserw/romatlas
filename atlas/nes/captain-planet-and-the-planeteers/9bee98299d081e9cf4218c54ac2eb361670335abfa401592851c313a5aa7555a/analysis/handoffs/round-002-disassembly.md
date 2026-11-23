# Round 2 disassembly handoff (pass-002)

Scope: single easy-gain pass applying redirect r001-a ready leads (L001-L005, L007, L008) to Captain Planet and the Planeteers (NES, MMC3).
Resumed from analysis/state.json (pass-001); no new baseline. Prior internal state archived as analysis/state.pass-001.json.gz.

## Checkpoint / verification
- maps/pass-002.json, generated/pass-002 (local), analysis/pass-002-verification.json: ok true, byte_identical true (map sha256 295a9b18...).
- analysis/state.json saved, maps/current.json exported, map.validate ok.

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).
Status: pass complete; ROM INCOMPLETE (5 unknown bytes; the percentage is rounded).

## Findings
- Code 48907 B (Observed 18178, Static 30729), Data 80115, Text 2061, CHR Image 131072; 1933 regions.
- Banks 8/13 Captain Planet object/task code classified from MapSpawnTable/LevelInitTasks entry words (TaskBank constant written after CreateTaskAt).
- Banks 3/6 handler code from SpecialObjectHandlers ($A3BD, 8 words) and EventObjectHandlers ($A2FE, 26 words); both tables now separate labelled Data regions.
- Fixed bank: $F1E6 level-clear portrait tasks, $F3D9 credits sprite tasks; consumer-proven data ($C639 MetatileLookup offsets, $D252, $E814, $E9AB, $F218, $F494, $F4B2-$F6FA).
- L004 corrections: bank 3 $AECC-$AEE0 and bank 8 $B213-$B21B (SetTaskBank8) are code; heuristic regions 1404/1437 split.
- L007: 70 data rows (sprite-definition chains, consumer-read tables, pointer tables) + 10 briefing-text bytes merged into neighbouring Text.
- L008: 256 B code (unreferenced but complete routines, JMP-table slots; Static, reachability unknown) + 52 B data; 5 B rejected.
- New labelled regions for commentary to carry forward: SpecialObjectHandlers, EventObjectHandlers. Region ids changed in banks 3/6/8/12 where regions were split/merged
  (1402, 1404, 1414, 1437 and Text 1453-1463 edited/removed); commentary should re-anchor by address, not id.

## Remaining unknown (5 bytes; lead L009, low)
- $1BA6D-$1BA6F bank 13 $BA5D: 'jmp $B4C2' directly after 'jmp $B4C2 / rts'.
- $1FB79 bank 15 $FB69: single $60 after PpuBufferBegin's rts.
- $1FDD0 bank 15 $FDC0: single $60 after FlushPpuBuffer's rts.
No 16-bit reference anywhere in PRG; never fetched in saved scenarios. Code vs data ambiguous (lone return/jump opcode).

## Unresolved questions / deferred work
- L009: is there a systematic assembler pattern (duplicate rts / tail jumps) that justifies classifying the 5 bytes as dead code?
- Reachability of L008 Static routines (no caller known) and of Static code generally (execution coverage 18178/48907 code bytes observed).
- Gameplay deferred: bosses (L5 CP attempt ~3600 frames did not reach one), real game over (CP health RAM contract unknown), L2 CP stage.
  Next step: identify CP health/position variables in bank 7 CpPlayerTask, then patch to reach boss/game-over.
- L005 leftovers: bank 12 $A256 and bank 7 $B77C bank-context review (no byte depends on them).

## Toolkit issues
No new toolkit issues found this pass. Existing analysis/toolkit-issues.md: TI-001 missing feature (NES discovery stops at JSR on MMC3; worked around),
TI-002/TI-003 confirmed API/doc bugs, TI-004 tracing performance. No blockers.

Artifacts: tools_local/pass002_annotate.py (annotation script), analysis/leads.json, analysis/scenarios.json, analysis/progress.md, analysis/investigation.md.
