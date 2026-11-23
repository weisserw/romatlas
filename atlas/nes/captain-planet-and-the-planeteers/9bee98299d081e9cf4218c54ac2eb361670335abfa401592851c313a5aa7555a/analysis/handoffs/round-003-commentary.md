# Round 3 commentary handoff (input checkpoint pass-003)

Status: COMPLETE. Current-source readability is done for all classified source. A fresh
byte-identical verification passed and the reading copy was refreshed. This is not ROM
completion: 4 bytes are still Unknown. It is not final editorial acceptance either:
editorial_audit is still pending.

## Scope
- The map changed from pass-002 to pass-003, so I moved the entire round-2 commentary
  unchanged to analysis/round-2/commentary/. I then prepared a fresh baseline
  (commentary/before, 1940 scaffold entries).
- Round-2 tools, names, comments and review summaries were carried by physical address.
  Only the 3 changed ranges were re-annotated (tools/mapdiff.py):
  - bank 6 $A175 and $A18B (Code -> Unknown): the Bank6StubReturn labels were replaced by
    honest Unknown notes.
  - bank 13 $BA5D-$BA5F (Unknown -> Code): now `PusherTask_deadJump`, with a note.
  - bank 8 $B20E split: now `EmptySpriteDef8`.
- L010 applied:
  - EmptySpriteDef8 is labelled and explained, including the byte it shares with the
    empty MapSpawnTable list.
  - TransformFrames is rendered as `.word TransformSpriteN` + `.byte dx, dy` records.
  - The stale "no reader" note was removed.
- L011 applied:
  - bank 7 $B0CD is rendered as `.word $A015` (entry $A015 of CpStageBank), with a block
    comment on the boss-zone entry.
  - CreateInitialObjects(13) -> CreateBossZoneObjects(13); LevelInitTask* -> BossZoneTask*.
  - AfterPasswordEnding -> StageEndDispatch, StartEndingSequence -> StartGaiaMessage,
    EndingTask -> GaiaMessageTask, EndingPictureTask -> GameOverMenuTask, and the
    Ending* screen helpers -> Gaia*.
  - RingLevel -> Lives; EndingFlag -> StageEndFlag, with notes on its values
    (0/1/2/$FF).
  - The Level3BossTask header now describes escape vs defeat (local label @bossEscaped).
  - The CpPlayerDeath note was updated.
  - One deviation from the L011 recipe: StartGaiaMessage instead of
    StartGameOverSequence, because StageClearTask (after the final CP stage) and the
    ending password also reach it.
- Other concrete defects fixed:
  - 23 owner+address branch labels given descriptive names.
  - Stale TerrainHitCount text removed.
  - The wrong "empty list" note on MapSpawnTable13 was corrected.
  - New RAM names: AccelCounterX/Y, AccelDelay, CpLastStepX/Y, TextReadOffsetLo/Hi,
    TextEndPause.
  - PendingPalette11/15 and CpPalette are now 3-byte arrays.
  - Multi-line RAM notes are rendered as comment blocks.
- No resume ranges.

## Residual counts (commentary/tools/metrics.py)
- 1398 code labels. Address-only globals: 0. 20 labels keep an _A0xx/_XXXX suffix; these
  are the CallCpStageBank_A0xx trampolines, which are named by a variable-bank entry.
- 726 labels have routine comments.
- Raw RAM operands: 0. Generic locals: 0 (29 flag-only conditions).
- 109 Unk_* RAM symbols, each with a "role uncertain" note.
- review.json: 1940 entries reviewed, 2184 units, 4 Unknown entries with notes.

## Unresolved meanings (commentary/uncertainties.md)
- Game identities of the CP-stage and Geo objects: names describe mechanics only.
- The level-3 boss defeat path is a static reading only (the escape path was observed in
  r002-a). Unexplained: why the life loss is skipped when CpHurtCount >= $10.
- AccelDelay values; StageSummary use of Level-1 while StageEndFlag is set; the Gaia
  message texts (quoted from runs, not decoded).
- SuperModeTimer game name; bank 3 special types $20/$21.

## Structural questions for redirect
1. The 4 Unknown bytes ($0C185, $0C19B, $1FB79, $1FDD0), each a lone $60 after an RTS.
   Reopen them only with a reference, a fetch, or a model that discriminates dead code
   from padding.
2. L008 static-only routines (the list is in uncertainties.md): reachability unknown.
3. Optional gameplay check: a played level-3 boss defeat (StageEndFlag+1, score 50), and
   what the CpHurtCount >= $10 skip does.

## Follow-up (coordinator: 4 code regions without units)
- Regions 1634 ($11A90), 1718 ($1BB58), 1025 ($1E010) and 1026 ($1E012) had no units.
  The cause: tools/review.py cut off the unit of a bank's last label at 1 byte.
  - review.py now extends that unit over its contiguous code run (Code regions plus
    inline words of 4 bytes or less), clipped to the bank end.
  - Regions 1634 and 1718 now have the units SpawnDebrisHere and RisingOrbTask.
  - Regions 1025 and 1026 now have the new commented label `PasswordInputLoop_bank15`
    (bank 14's loop falls through into $E000), plus `PasswordInputLoop_checkUp`.
- Units: 2184. archive.editorial_status now gives only one reason: "final editorial
  audit is pending". The audit was not run.

## Verification
`python3 /root/romatlas/scripts/commentary.py verify <ws>` gave ok=true and
byte_identical=true (commentary/verification.json; game.asm sha256 c74a7949475...).
`commentary.py read` refreshed commentary/reading-after.asm.

## Artifacts (local only)
- commentary/source/game.asm, commentary/reading-after.asm, commentary/review.json,
  commentary/progress.md, commentary/uncertainties.md, commentary/verification.json.
- commentary/tools/: the build.py/review.py/metrics.py/mapdiff.py pipeline;
  ann/r03.py holds the round-3 edits.
- Archives: analysis/round-2/commentary/ and analysis/round-1/commentary/.
- leads.json: L010 and L011 are marked as applied.
- No toolkit issues were found in this phase.
