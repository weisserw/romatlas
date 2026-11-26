# Commentary progress

## Round 3 (input checkpoint pass-003) - COMPLETE, final acceptance

Map changed since round 2 (pass-002 -> pass-003: L-015 classified six bytes as Static code,
confidence 0.5). The entire round-2 commentary/ was moved to analysis/round-2/commentary/
(only copy; untouched except its regenerable tools/__pycache__) and a fresh baseline was
prepared (before/ byte_identical true, 772 scaffold entries; only the five L-015 entries
changed kind Unknown -> Code).

Carry: tools/ copied from round 2; annotate.py regenerated source/game.asm from the new
before/ with the round-2 notes. Diff vs the round-2 source: only the five L-015 regions.
Invalidated: their round-2 "still unknown" @REGION notes (both copies removed).
New: UnrefReturnCarrySet (6:$9B8B sec; rts), UnrefReturnStub_A1E4 (6:$A1E4),
UnrefReturnStub_C916, UnrefReturnStub_CA5B and local @UnrefReturnStub ($C405, kept local so
State08_StageExit's labels keep their scope; annotate.py gained support for local labels at
unreferenced instruction starts). Each states no reference, confidence 0.5, reachability unknown.

Final editorial fixes (targeted, no whole-source reread):
- Data_B6_9005 -> UnusedFillRomTarget (with format note); $07F0 -> UnusedFillEntry.
- Scratch $4C/$4D/$4F -> Temp4C/Temp4D/Temp4F (operands and headers); $00-$17 scratch
  convention documented once in the RAM symbol header.
- AddDamageTo0514 -> AddDamageToAux0514, AddDamageTo0584 -> AddDamageToObjKind ($0584 alias).
- Stale source-header sentence about remaining address-default names rewritten.
- review_update.py: continuation units use a region note when a local stub label exists.

Counts: 941 global labels (0 address-default), 1240 descriptive locals, 151 data labels
(0 address-default), 177 RAM symbols. review.json 772 entries / 1156 units all reviewed;
editorial_audit reviewed.

Verification: audit ok (commentary/audit.json): byte_identical true, 772 regions, 1156 units,
editorial_audit true; game.asm sha256 a9fcd4da933024085615cb6e7cee1955e2ca05979acf7cb0914dfc9218356880.
Reading copy refreshed by audit: commentary/reading-after.asm.

Remaining: none in readability/editorial scope; semantic unknowns in uncertainties.md.

Round-1 and round-2 histories are preserved with their whole phase directories in
analysis/round-1/commentary/ and analysis/round-2/commentary/ (round 2 is repeated below).

## Round 2 (input checkpoint pass-002) - COMPLETE

Map and phase-1 state changed since the round-1 baseline (maps/current.json =
maps/pass-002.json differs from analysis/round-1/commentary/baseline.json), so the entire
round-1 commentary/ directory was moved to analysis/round-1/commentary/ (only copy, not
modified) and a fresh baseline was prepared (scripts/commentary.py prepare; before/
byte_identical true, 772 scaffold entries).

Method (unchanged): source/game.asm is generated from the frozen before/game.asm by
tools/annotate.py with the curated database tools/notes.txt; review.json by
tools/review_update.py. Round-1 notes are keyed by physical offset, so names, headers,
local labels, inline notes and RAM symbols carried forward unchanged wherever the code
did not move; the carried build was verified byte-identical before any edit.
annotate.py gained @JUMPTABLE (force a data region to .word entries), a warning for local
labels promoted to globals by new region splits, and suppression of the phase-1
"Small table read by adjacent ..." one-liners where a curated @DATA note exists.
review_update.py now adds a continuation unit for code regions without a routine start.

Carried and revalidated:
- Stale round-1 statements invalidated by reclassification were rewritten: unknown-range
  notes for all regions that became code/data (bank 0 $8008/$8185, $C453, $CF5F, $D87B,
  $E9AC, $EC64, $ED38, $EE4B, $EF74, $FA90, $FCCC, $FF06, bank 6 $90A1, the L-003
  fragments), "unclassified bank-5 code" notes, CallStageSplashRoutine/SpawnSplashDropsAlt
  bank context (bank 5, previously resolved to bank 6), Spawn14_Chain head/base roles,
  Spawn01_EdgeFlyer -> Spawn01_EdgeWalker (it creates ObjType 1), FlyerCounter ->
  EdgeSpawnCounter, Hit_Boss/Hit_Type1A/1C/1D and boss-spawner headers (boss identities,
  Dracula's head hit box).
- Six round-1 local labels split by new region boundaries were re-named as globals
  (CheckPause_Return, EntryTrigger_Return, StoreHearts, TestTerrain_Right/_ToWorld,
  StartGame_Finish).

New source (round-2 classification, about 8.6 KB):
- Bank 5 $80C7-$9A34 (2780 instructions): all 19 banked ObjectHandlerTable handlers and
  their 18 inline state tables (.word with state names), the StageSpecialHandler and
  ThrowBoneDispatch targets, 25 small data tables named (BatHoverTimes,
  MedusaFloatTimes, ChainMaxOffsets, DraculaTeleportOffsets, DraculaDebrisRecords, ...),
  DraculaFireAimOffsets and DemoScript0-2. About 200 routine/state names and 220 local
labels added; every state
  routine has a header.
- Fixed bank: TestTerrainByWalkDir, MoveVerticalByAux04DC, SpawnAimedShot,
  ThrowBoneDispatch (bank-5 site banks), MoveXPagedByWalkDir, TestFloorBelow8,
  HideAtScreenEdge, EraseBossStatueTiles + BossStatueEraseRecords, Bank5Sound42/12/13/1D,
  DebugStageSkip, UnusedRecordInput, DmcSample, unused fragments with mechanical notes.
- Bank 6: UnusedFill9005, UnusedQueueRows/UnusedQueueRow and tables, TextScreen4Streams.
- RAM: BossPatternIndex ($55), BatTargetX ($59).

Counts: 937 global labels (0 address-default Sub_/Loc_/Handler_/Entry_ names), 1240
descriptive locals (0 structural @loop/@skip), 151 data labels (one address-named:
Data_B6_9005, the ROM write target of UnusedFill9005), 173 RAM symbols.
review.json: 772 entries all reviewed (594 Code, 163 Data, 10 Padding, 5 Unknown with
notes); 1150 units all reviewed; editorial_audit pending (final acceptance only).

Verification: commentary/verification.json ok true, byte_identical true, game.asm sha256
86912353e2d15b79a0b7ad1b5115978bfb92bc94053edbb8193579198b00705b. Reading copy refreshed: commentary/reading-after.asm.

Remaining: none in the current-source readability scope. Unresolved meanings are listed in
uncertainties.md; final editorial audit and whole-ROM classification (6 unknown bytes)
remain separate gates.
