# Round 2 - commentary phase handoff

Status: FINISHED (current-source readability pass complete for all classified source; final
editorial audit intentionally not run - reserved for final acceptance).

## Input / output identity
- Input checkpoint: maps/round02-final.json (sha256 53e08b7ba4231a38a737bbb86021112b6a9e2d287ea74e08841aad195b565bfa);
  maps/current.json and analysis/state.json unchanged by this phase.
- Previous commentary preserved: analysis/round-1/commentary/ (full round-1 commentary dir, annotations,
  source, review) and analysis/round-1/tools/ (round-1 cmt_*.py). Fresh baseline prepared in commentary/
  (2291 scaffold entries, byte-identical baseline).
- Output source: commentary/source/game.asm sha256 2ed461a9a774b74363d5186d99fbaa619c7eea9f4eab63581a26d7c6f296e021,
  built from commentary/annotations.txt (sha256 390b384e...) by analysis/tools/cmt_apply.py.
- Review inventory: commentary/review.json (sha256 30975b29...), 2291/2291 entries reviewed, 981 units.
- Verification: analysis/round02-commentary-verification.json -> ok:true, byte_identical:true.
- Reading copy: commentary/reading-after.asm (refreshed after the final verify).

## Classification (unchanged)
ROM classification: 99.89% (130939 / 131088 bytes classified; 149 bytes unknown in 18 gaps, lead L-R2-01).

## Scope covered
- All classified source: round-1 commentary carried by bank/CPU address; every changed range reviewed.
- New code (32 regions): reset path (ResetVector, PowerOnReset, ClearRamAtReset, InitHighScoresAndRamCode),
  menu actions (MenuStartEventAtCursor, MenuConfirmYes/No), surfing score screen (SurfScoreFrame,
  DrawSurfScorePicture, BuildSurfScoreBackdrop + helpers, AdvanceToNextPlayer), DrawLicenceText,
  course-script commands (12 CourseCmd*), sound commands 2/3/7/8/9 and driver entries, SkateState4,
  SurfState5-7, SurfEndRideWait, BMX trick bonuses. All with prose and descriptive branch labels.
- Invalidated round-1 notes rewritten (gap notes now code, table descriptions, ShowEventScore role,
  SurfStart split, surf overlay scope that mislabelled BMX code).
- Backlog: all 44 address-style routine names replaced (half pipe, foot bag, flying disk, bank 6/2/3).
- Data: new data regions carry lead role notes; 18 unknown gaps carry decode/uncertainty notes.
- Remaining unreviewed ranges: none.

## Readability assessment
Good for system code (reset/boot, NMI/frame dispatch, banking, sound driver incl. all commands,
text/menu system, results/high scores) and event top levels. Event internals are named with prose
where mechanics were clear; foot bag/flying disk/half pipe internals use tentative role names, and
overlay variables there stay numeric. Branch labels outside the round-2 code remain structural
(Owner_Skip/Loop/Return). Adequate for porting navigation.

## Findings
- Warm reset: zero page $00-$20 is not cleared; $12-$20 hold " PAUL & RACHEL " as a signature so high
  scores survive reset. $07-$09 (RNG entropy) are never written (uninitialised RAM).
- RamRoutineImage = two bank-switching tile copy routines uploaded to $0310 (plain) and $042A (RLE).
- ShowEventScore/SurfScoreFrame are surfing-only (five rating digits, probably judges' cards).
- Course scripts (bank 6) have a command byte $08 escape with call/return/repeat commands.

## Unresolved / leads
- commentary/uncertainties.md (round 2 section): $61/$4D unread flags, menu kinds 4/5 screen mapping,
  course command value meanings, surf states 5-7, foot bag/disk actor roles.
- L-R2-01 residual unknown bytes unchanged (no commentary-side evidence of callers).
- No new toolkit issues (analysis/toolkit-issues.md unchanged). Workspace tool fixes only (cmt_apply.py,
  cmt_routines.py).
