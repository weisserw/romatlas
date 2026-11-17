# Round 1 disassembly handoff (pass-001)

Scope: one disassembly pass on Baseball Stars II (NES MMC3, 256 KiB PRG / 128 KiB CHR), resumed from an
interrupted worker's scratch frontier; baseline reused unchanged.

Checkpoint: maps/pass-001.json -> generated/pass-001. Verification: analysis/pass-001-verification.json
(ok, byte_identical). Internal store: analysis/state.json (2,953 regions); export maps/current.json.

ROM classification: 91.90% (361379 / 393232 bytes classified; 31853 bytes unknown). Status: incomplete.
Per-bank counts: analysis/progress.md (round 1 pass-001 section).

Findings
- Bank model, dispatchers ($C16A pair select, $C316 inline jump tables, $C32D far calls via $FEB4, inline-param
  calls $C6D6/$C6FC/$C271): analysis/investigation.md.
- Code: 71,078 bytes (48,621 observed, 22,457 static recursive descent). Data: 159,229 bytes (structured
  inline/jump-table/header/vector regions + generic pattern data). CHR ROM: Image.
- Gameplay milestones visited incl. LEAGUE CHAMPION ending and GAME OVER (artificial RAM patch); RAM contract for
  innings/score: analysis/gameplay.md; routes/snapshots: analysis/scenarios.json, scratch/snaps/.

Unresolved questions / leads (analysis/leads.json)
- L001 bank 17 $BCAD-$BFFF code-like island (851 bytes), possible far-call entry $FEB4[116]=$BD1D with pair 16/17.
- L002 code-like islands without entries in fixed banks and banks 2/3, 9, 11, 15, 24/25.
- L003 $FEB4 table extent and the adjacent $FED8/$FF02 tables.
- L004 opcode-valid tables/text left unknown by the generic heuristic (consumer-bounded extents needed).
- L005 data-bank format via the $CB07/$CB25 PPU stream routine (banks 16-29).
- L006 deferred menu variants (2P, ERROR ON, stadiums, VETERAN/STAR, '?').

Deferred work: commentary on all regions (only a few labels: Reset, Nmi, Irq, SelectPrgPair, JumpTableDispatch,
FarCall, InlineTextCall, InlineParamCallA/B, INesHeader, CpuVectors, ChrRom); data-region semantics.

Toolkit: TI-001 (missing feature, not blocking) in analysis/toolkit-issues.md.

Process state: the serve process (scratch/daemon.py wrapper) was saved (map.save to analysis/state.json) and
stopped at the end of the pass. Scratch tools: scratch/annotate.py (rebuild map), flow.py/survey.py (static
closure), dataregions.py (generic data), collect.py (observed export), go2.py/route.py (input routes).
