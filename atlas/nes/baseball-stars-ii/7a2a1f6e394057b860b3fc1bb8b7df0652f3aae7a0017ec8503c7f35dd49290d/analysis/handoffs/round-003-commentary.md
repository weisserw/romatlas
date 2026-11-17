# Round 3 commentary handoff - COMPLETE (current-source readability on the pass-003 baseline)

## Scope and checkpoint
- Checkpoint: maps/pass-003.json = maps/current.json = analysis/state.json (sha256 c36cb8af...), 4,101 regions,
  25 unknown bytes in 6 runs (analysis/pass-003-remaining.json). commentary/ was re-prepared from it
  (manifest map_sha256 c36cb8af...; review scaffold 4,107 entries).
- The previous commentary is preserved whole under analysis/round-002/commentary (pass-002 baseline, ebcf3314...).
  Round 1 remains under analysis/round-001/commentary. Nothing prior was overwritten.
- Carry-over: notes are keyed by (PRG bank, CPU address). All round-1/2 notes rendered on the pass-003 listing
  without stale keys, and that carried source verified byte-identical before any round-3 edit.
- Source is generated from notes by commentary/tools/render.py. Pipeline order: gencopies, genscreens,
  genlocals, render, review. Round-3 notes: notes/99c_round3_new.py (new code/data, residue, RAM aliases) and
  notes/99d_round3_locals.py (branch labels). Stale prose was corrected in place in notes 80, 88, 89, 91, 93, 94,
  95 and 99.

## Verification
- `commentary.py verify` gives byte_identical true. It was run fresh after the final render, and the result is in
  commentary/verification.json.
- `commentary.py read` refreshed commentary/reading-after.asm.
- review.json: 4,107 reviewed / 0 pending, 1,967 units. Round-2 credit is carried for entries whose
  (offset, length, kind) did not change. The 118 pass-003-changed entries carry round-3 summaries.
  editorial_audit stays pending until final acceptance.

## What changed
- Pass-003 bytes (116 runs, 1,725 bytes; tools/diffbase.py, tools/shownew.py): every code run is a named
  routine with a mechanical description and the "no known caller (pass-003)" fact. Examples:
  - DrawSideRosterSheet and PrintRosterNameRow
  - PrintEditedTeamNameBox
  - DrawPlayerIcon
  - FormatMoneyLine, PutThreeDigits and PutCommaThenDigits
  - SaveSideTeamBlock
  - GridCellVramAddr and MainMenuReenterAtCell (bank 3)
  - GenRenumberRecords and GenRemapLowBits (bank 10)
  - LeaderValueDigits (bank 11)
  - CheckRunnerAtThrowBase (bank 12)
  - InfoNumberToDigits and Divide16By8 (bank 15)
  - QueueRemoveTeamMarks (bank 24)
  - ReloadBgChrSet, ReloadSprChrSet, JumpBigTextDirect and FlushPpuDirectFixed/ClearPpuDirectFixed (bank 30)

  The data runs have consumer labels (GenLowBitsMap, InfoRectPtrs, PlayerIconTiles/Vram, DollarSignPos,
  SideTeamBlockBase, FarCallEntry14) or REGION role notes. Examples of role notes:
  - the RandomTable tail
  - the main-menu label strings
  - the EmptySlotFlagPtrs duplicate
  - the TeamBlockDstPtrs tail

  None of the pass-003 bytes is left with a mechanical summary, an address-only label or a generic-data note.
- Unknown residue (25 bytes): each run has a note covering its decode, its neighbours and why it stays unknown.
  - Bank 2: $80DA RTS pair, $9988 dead load, $9B48 lone RTS.
  - Bank 9: $A165 lone RTS.
  - Bank 30: $C0C0 effectless LDA/STA ChrBankTemp.
  - Bank 31: $F6C2 Temp0C += 18 with a broken carry.
- Editorial residue: 1,270 branch labels were read and named routine by routine in banks 0-3, 8-15, 24, 25, 30
  and 31.
  - Generated labels went from 888 @skip / 381 @loop to 0 / 0.
  - 242 @return labels remain. Each marks an RTS target; 16 of them are numbered.
- RAM aliases:
  - OamWriteOffset ($048A, global), with DigitOutIndex in bank 9.
  - RosterSetupSide, RosterSetupIndex and RosterFillCandidate (bank 8 $074A-$074C).
  - TeamPointsPtr (bank 25 $055A).
  - InfoDigits (bank 15 $0777, 7 bytes).

  Raw scratch operands went from 1,573 to 1,518.
- Tool fixes (commentary-local only):
  - genlocals now keys labels of routines that cross into the next bank correctly, which removed the old
    bank 10 $A0EA render warning.
  - db.py accepts REGION.
  - review.py carries credit across rounds.

## Findings
- The bank 2 inline words at $8C52-$8C53 ($2204) and $8CA0-$8CA1 ($00BA) inside PitcherListDrawAlt are still
  classified as code. A ROM-wide scan found no other inline-parameter bytes decoded as code: no JSR
  $C6D6/$C6FC, and no FarCall Y=4/8 record. Filed as lead L025.
- $AFCE in bank 11 is a separate decimal-digit routine (LeaderValueDigits), not the champion highlight that the
  round-1 note assumed.
- $C9A8 in bank 30 jumps directly through FarCall table entry 14 (FarDrawBigTextInline) without the pair switch.

## Unresolved questions (commentary/uncertainties.md, round 3 section)
- What the static pass-003 routines are for, given that none has a caller. Examples: MainMenuReenterAtCell,
  GenRenumberRecords (the RunsHome=$84 gate), CheckRunnerAtThrowBase, QueueRemoveTeamMarks, LineupCursorAdjust,
  and DrawPlayerIcon (what player-record byte 5 "1"-"5" means).
- The source field of FormatMoneyLine. It reads $683D + 16*(SideTeam-8) + 5, which does not match the
  12-digit points layout, so the "dollar amount" reading is tentative.
- Whether the 25 residual bytes are code or data (lead L021).
- The game meaning of some branch names that only describe values, e.g. @select_7/@select_8 in bank 24 and
  @zone_0/@zone_2 in bank 13.

## Deferred work (not blocking)
- Multi-role scratch RAM keeps raw operands. Examples: $0540 in bank 2, $074A/$074B in bank 11, $0660 outside
  pair 12, and zero page $BA/$C0/$C1/$DB.
- 171 generic-data notes on phase-1 data that has no known consumer.
- The final editorial audit (`commentary.py audit`) is reserved for archive acceptance.

## Leads (analysis/leads.json, pass commentary-003)
- Produced L025 (open, low): split the two bank 2 inline words out of the Code region as Static Data. The
  expected result is 4 Code bytes becoming Data, with no change to the unknown count.
- Updated L021: an attempt record notes that all 6 residual runs carry round-3 notes. No new entry or consumer
  was found.
