# Handoff: round 2, phase 2 (commentary)

**Status:** the current-source readability gate is met for all classified source on the
round-2 map, with the remaining uncertainties documented. The final editorial audit has not
been run; it is reserved for final acceptance, and review.json `editorial_audit` is still
pending. Whole-ROM classification is a separate requirement and remains incomplete:
393,222 of 393,232 bytes are classified and 10 bytes are unknown (L-016). This pass did not
change the classification.

## Checkpoint
- Map: maps/round2-disassembly.json (= maps/current.json, sha256 8b43ff3e...). analysis/state.json
  was not modified. input/game.nes is unchanged and the toolkit was not modified.
- The round-1 commentary is preserved in full in analysis/round-1/commentary/ (source, specs,
  tools, review, progress, uncertainties, before/). The new baseline was made with
  `commentary.py prepare`. commentary/before/ is frozen.
- Source: commentary/source/game.asm (sha256 36b37cc1...).
- Specs: commentary/specs/r2-01 to r2-11.
- Tools:
  - commentary/tools/merge_r2.py (carry-over by physical range)
  - promote.py (cheap-local scope repair with exact branch binding)
  - ramplus.py (Name+k RAM operands)
  - datanote.py (data role notes)
  - r2check.py (new-code doc coverage)
  - shownew.py (viewer)
  - edit.py now runs promote.py and refuses duplicate names.

## Verification
- `commentary.py verify`: ok, byte_identical true. This was the final run after the last edit.
- `commentary.py read`: commentary/reading-after.asm refreshed.
- review.json was regenerated:
  - 2,208 of 2,208 scaffold entries reviewed
  - 1,150 units
  - editorial_audit pending

## Scope and readability
- **Carried over:** 1,728 regions, including 5 unknown bytes, with unchanged range and kind.
  They keep their round-1 text verbatim.
- **Newly commented:** 480 new or changed regions, of which 266 are code, 212 data and 2 unknown.
  - r2check shows all 266 new code regions belong to documented routines.
  - No Sub_/J_/L_ address names remain on code.
  - New branch labels have role names.
  - The 16 index-named B02_Task0605_NN handlers were renamed FrontEnd_<role>.
- **Examples:**
  - NMI tasks: NmiTask_01/04/06/08, ClearNametablesStep
  - Front end: FrontEnd_PitcherCheck/CatcherCheck/FielderCheck, FrontEnd_BatterSubstitution,
    FrontEnd_BiorhythmA, FinalScore_SeasonCheck
  - Bank-4 scripts: B04_ScriptA..E_*
  - Bank-11 CPU input states: CpuState18_CheckSwing through CpuState79_Reset,
    CpuSide_SteerFielder, CpuPitcherDecision
  - Bank 15: PlayState_26/42/45-4A/51-53/61-67/70, MercyCheck_SideA/B, ForceHandlers,
    TagUpHandlers, AdvanceRunnersTwoBases
- **L-015 static code:** each fragment says "no caller is known" and that reachability is
  unresolved.
- **Data:**
  - 151 "Used by" consumer notes.
  - 81 L-014 islands carry a context note.
  - 15 role notes for clear tables, such as player categories, CPU count masks, voice-sample
    tables and field marker positions.
  - Jump tables are rendered as `.word` label lists: the 23 L-012 tables, plus ForceHandlers,
    TagUpHandlers and CpuSideModeJumpTable as presentation only.
  - BaseCoordTable is labelled.
- **Stale round-1 text fixed:** dispatcher notes that called tables "unclassified",
  unknown-range notes (now L-016 notes) and the preamble.

## Findings
- **Lead L-017 (L-015 fragments are table-reachable):** the bank-15 force and tag-up
  fragments ($F95B-$F982, $F9DC-$FA13) are entries of two word tables, $F92A and $F947, read
  by CheckForceAtBase's JMP (Tmp0). The map still classifies both tables as generic data.
- **Lead L-017 (CpuSideMode table):** bank 11 $D08E is the 128-word CpuSideMode jump table.
  Its entries are SelectSide, SteerFielder, FielderAction, SetButton and Idle. It is still
  generic data in the map.
- **Lead L-018 (debug-looking code):**
  - NMI task 8 is a freeze/pause. Its only entry is DebugPauseCheck, which is disabled by
    AND #$00.
  - DebugForceGameEnd ($C606) gives one side 16 runs at the bottom of the 9th.
  - Bank 2 has editors for StartSeasonDay and SeasonWins.
- **Game logic:**
  - The CPU picks substitutes by biorhythm: the best total, or Biorhythm[CpuSubstitution].
  - The mercy rule ends the game at 100 runs or a lead of 9 or more.
  - The end-of-game season check needs 75 wins to reach the ending sequence; days $82-$88
    look like a best-of-7 series (tentative).
  - The lineup-check screens force a catcher or fielder fix. The pitcher check is dead
    because ReturnA2 always returns 2.

## Leads
- **Consumed:** none closed. L-010 is applied by the round-2 map and reflected in the source.
- **Produced:** L-017 and L-018, both in analysis/leads.json.

## Unresolved questions
These are in commentary/uncertainties.md:
- Which events select front-end screens $10, $18, $28 and $40/$41.
- The meaning of player categories and the position rules.
- The "Right" menu input pressed by the CPU.
- The PlayResult bits behind states $42 and $46-$48.
- The $06C5-$06C7 counters.
- About 330 unnamed raw RAM operands.
- Natural reachability of the remaining L-015 fragments.

## Deferred work
- Final editorial audit (`commentary.py audit`) at archive acceptance.
- Retype the L-017 tables in a disassembly or redirect pass.
- Optional runtime checks for L-018 and the season-end logic.
- Toolkit: no new issues. TK-001 to TK-005 are unchanged (analysis/toolkit-issues.md).
