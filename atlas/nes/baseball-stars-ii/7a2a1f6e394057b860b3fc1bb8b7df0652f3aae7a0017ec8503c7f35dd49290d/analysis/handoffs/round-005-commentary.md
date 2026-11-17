# Round 5 commentary handoff (2026-11-17): COMPLETE (editorial audit still pending)

## Scope
All classified source in commentary/source was covered: banks 0-31, header and CHR. The full inventory in
commentary/review.json (4,111 scaffold entries) was reconciled with the pass-005 map. Nothing outside commentary/
was edited except this file. maps/, state.json, analysis/round-004/, input/ and the toolkit are untouched.

## Method
1. Scratch scripts are in /tmp/cw, not in the workspace. One script parses every bank file and gives each source
   line its physical offset. It checks the result against all 4,108 region headers, and every header matched.
2. Labelled blocks without comments: the script listed global labels in code that have no comment and no unit. 105
   of them start a new block, because the line before them is an rts, jmp, data or inline parameters. Each of these
   got a short comment written from its code, callers and enclosing routine. Line refs in review.json were shifted to
   match the inserted lines.
3. Units for the 1,356 Code entries that had none:
   - Fragment starts mid-routine: the unit is named after the nearest preceding commented or unit-bearing code
     label. The search continues into the previous bank of the pair for routines that cross from $8000 into $A000.
     The summary is "Part of <Routine> (<observed|static> fragment, CPU range, bank): <routine comment>", plus the
     branch labels and notes in the fragment. The source refs are the fragment's region header and the routine label.
   - Commented labels inside an entry each start their own unit. The summary is the label's comment and the source
     refs are the comment and label lines.
   - Spot checks: the farthest a fragment sits from its routine label is about 440 bytes (RankStandings, bank 10->11).
     Four fragments sit after a labelled table and were checked by hand. Their routines (LeagueTeamRemove,
     CpuThrowIfLate, FielderTryJump, FielderLowCatchFrame) now have comments and units of their own.
4. Prior review credit and prose were all kept. The 1,969 existing units and all entry summaries are unchanged
   apart from line-number shifts and the card-label rename below.

## Examples
- region-2754 -> unit FielderStartMove (observed fragment $A5C2-$A5EF, bank 13), from the routine header.
- region-4595 -> FieldColRightMove_B23 (bank 23 stadium-pair copy of bank 5 FieldColRightMove).
- New source comments, for example:
  - bank10 LeagueTeamRemove: B removes the last league team and restores its grid cell.
  - bank12 CpuThrowOrRun: the meaning of $04F0/$04F1 is stated as unresolved.
  - bank30 JudgeBallFour/JudgeStrike: panel messages via DrawPanelMessage in pair 4.
- Name fix: the bank-25 pitcher/batter card labels were swapped. ObjTask+1 = 0 means pitcher, as set by
  PlayerSelectPickPlayer for places 0-5, and DrawPlayerCard jumped to the old "DrawBatterCard" in that case.
  - DrawBatterCard became DrawPitcherCard and @pitcher_card became @batter_card.
  - The PitcherCard* and BatterCard* labels swapped names.
  - The bank-18/20 records became ScreenPitcherCard and MetatilesPitcherCard.
- Cross-pair raw calls in bank 30/31 (14 sites) now have inline target notes, for example "pair 6
  (CallMode3PitchLoop): bank 12 PitchFlightFrame". Operands stay raw because the switchable-slot context is
  unresolved in phase 1. $8274 at bank30 lines ~3669/~4049 is marked "probably pair 6": the caller's pair was not
  checked.

## Counts
- Units: 1,484 added, 3,453 in total. Code entries without units: 0. Entries not reviewed: 0.
- A local simulation of the audit's structural checks passes: unchanged scaffold, reviewed entries and units,
  non-empty summaries, valid file:line refs and units inside the ROM. Every unit has its name on at least one of its
  source lines.
- editorial_audit is left "pending", as instructed. `commentary.py audit` was not run.

## Remaining gaps and notes
- No unreviewed ranges remain.
- Not done: a semantic pass over the many raw RAM operands. Roles that are still unknown keep address-based
  operands, as earlier rounds allowed.
- 127 global labels without comments remain inside routines as fall-through or branch targets. Their names describe
  them and their routine headers explain them.
- The 8 manual-fallback bytes (regions 5155-5158) are unchanged. They stay generic Data of unknown meaning, added by
  user-directed fallback and not backed by evidence.
- Fragment unit summaries repeat the routine-level explanation. The final editorial audit may want to tighten the
  wording, but no meaning was invented.

## Verification
`commentary.py verify` gives byte_identical true and ok true. It was run after the edit batches and again at the end.
`commentary.py read` refreshed commentary/reading-after.asm.
