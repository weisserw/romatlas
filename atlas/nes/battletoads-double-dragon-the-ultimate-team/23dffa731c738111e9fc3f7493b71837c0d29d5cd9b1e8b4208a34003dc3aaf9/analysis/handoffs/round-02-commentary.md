# Round 2 commentary handoff - COMPLETE

Status: COMPLETE for current-source readability on checkpoint pass-002 (maps/current.json
== maps/pass-002.json == analysis/state.json; sha256 9c36a833...). Final editorial audit
NOT run (reserved for final acceptance). ROM classification unchanged: 99.35%
(260,469 / 262,160; 1,691 bytes unknown in 99 runs).

## Scope
- The map changed since round 1, so the entire round-1 commentary directory was moved
  unchanged to analysis/round-1/commentary/ and commentary/ was freshly prepared from
  pass-002 (baseline byte-identical). Round-1 notes/tools were copied in and carried by
  physical identity (all 1,269 pass-001 regions survive unchanged in pass-002; bank:CPU
  keys therefore address the same bytes). Carried edits were rebuilt and reverified
  byte-identical before new work.
- Source is built by commentary/tools/annot.py from before/ + notes/ (see
  commentary/progress.md). Round-2 tool changes: follow ObjTypeHandlers/BgObjHandlers
  tables as routine roots, notes for data regions, automatic consumer notes, stale-note
  detection, later r2_* notes override round-1 entries.
- Coverage: 977 routine entries all named (901 block-commented: every routine >= 4
  instructions); all 186 data regions and all 99 unknown runs have notes; 31 new
  table constants; review.json 1649/1649 reviewed with 901 units.

## Examples
- ObjTypeHandlers $9A71: 92 types now all named, e.g. ObjType08_56_HazardSpawner
  ($9638), ObjType25_PlayerReticle ($C7FA), ObjType26_HomingShot ($C919),
  ObjType33_Boss ($E87A), ObjType0B_Pouncer ($FB49, with anchor type $0A),
  ObjType14_15_PlayerVehicle ($C1D4). Shared helpers: HazardHitPlayers ($E094),
  AimThrowAtSlot/ApplyThrowVelocity ($BFDF/$BF6D), BossIntroLoadScreen ($F860),
  CarriedEnemyUpdate ($BDB3), SpawnBlastPieces ($E724).
- Bank 6 screen actors: UpdateBackgroundObjects with record layout $06DA-$0739,
  StepBgObjScript (4-byte motion records), AnimateBgObj (lists at $A031), 41 handlers
  BgObj00..BgObj28.
- Corrections: $08 = level 5 (not title/attract); round-1 ScriptedEnemyA/B_Update and
  BuildColumnData/NextColumnIndex/ScrollLayerA/B renamed per actual mechanics.

## Unresolved meanings (commentary/uncertainties.md)
Game identity of most object types/bosses (mechanical roles only); ObjTypeAttr tables
$9B29/$9B85 and most $9BE0 bits; many Obj_XXXX fields (reused per type as velocities,
timers); which screen uses which actor type; internal formats of sound/animation
scripts, metasprites, spawn lists and maps; reachability of ~17 uncalled fragments.

## Structural questions for redirect (concrete, bounded)
1. Bank 6 L-020 residuals are screen-actor motion scripts: handlers pass the pointer as
   immediates to StepBgObjScript ($9F4B; low byte via ObjHandlerPtr, high in A), which
   reads 4-byte records (duration, flags, X vel, Y vel) until a 0 duration. Decoding from
   each pointer ends exactly at the end of the unknown run for $9F31(+$9F3E), $E903,
   $EAE1(+$EAF6), $EC90(+$EC9D), $ECB0, $ECC7, $EE47, $EE62, $F003, $F229, $F347-$F35B,
   $F458, $F492, and $EFE6-$EFEE. ~270 bytes ready to classify as Data (check $EFE5 and
   $F32E-$F346, $F43A-$F43E separately). Check: `python3` loop decoding 4-byte records
   from each pointer (see commentary/notes/r2_gaps.txt).
2. Bank 0 $9635-$9637: probable tail of the WarpToAreaFromTable destination table
   (lda $9634,y); bound Y from the $EA producers.
3. Bank 0 $E522-$E537: probably entries of PlayerFrameMapA/B ($E4EB/$E4F5) reached with
   large player frame numbers (lda $E4EB,y with Y = $03B6,x); bound the frame range.
4. Bank 0 $AA05-$AA0D (tail of the $AA03,y per-stage table) and bank 3 $8996-$8999
   (tail of the PCM rate table $8995,y): index-range proofs would classify them.
5. Remaining L-021/L-022 items unchanged (bank 4 $B8BD/$B979 code-vs-table overlap,
   bank 2 $FC09/$FC94, bank 5 $BB49/$BC67, stub tails L-006).

## Remaining gaps
None for current-source readability. Address-based branch labels L_Bb_AAAA remain for
local control flow; Obj_XXXX fields keep address names where roles vary.

## Verification
`python3 /root/romatlas/scripts/commentary.py verify <workspace>`: ok=true,
byte_identical=true (commentary/verification.json, game.asm sha256 881e6a2f...).
Reading copy refreshed: commentary/reading-after.asm (`commentary.py read`).
Toolkit: no new toolkit issues found in this phase (TK-001..TK-003 unchanged).
