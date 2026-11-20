# Round 1 commentary handoff — COMPLETE

Status: COMPLETE for current-source readability on checkpoint pass-001 (map unchanged:
maps/current.json == maps/pass-001.json; frozen copy commentary/baseline.json,
commentary/phase1-state.json). Final editorial audit NOT run (reserved for final
acceptance). ROM classification unchanged: 18.50% (48,496 / 262,160 bytes).

## Scope covered
- Prepared commentary/ (toolkit scripts/commentary.py prepare). Buildable source:
  commentary/source/game.asm, generated from before/ + commentary/notes/ by
  commentary/tools/annot.py (instruction bytes untouched; see commentary/progress.md).
- All 8 banks: 696 routine entries named; 624 routine block comments (every routine
  of >= 4 instructions); bank banners; ~70 RAM/zero-page symbols, 34 object-table
  fields (15 slots), sound-driver RAM rows, hardware registers and the AxROM
  BankSelectTable used symbolically in operands; far calls/bank-service calls carry
  automatic "far call -> bank N slot: Target" comments; 260 unknown/unclassified gaps
  carry boundary/uncertainty notes (custom notes for stub-area gaps).
- review.json: 1532/1532 scaffold entries reviewed with summaries/source lines; 624 units.

## Examples
- Main thread as NMI-resumed coroutine: WaitForNmiThenResume ($85CF) -> NmiEntry_B0
  / Nmi_Common resets the stack and jumps to MainThreadResume (GameFrameLoop $8397,
  StageLoad_NextFrame $D241).
- Shared bank stubs: FarCall_Bn ($FFCB, X = slot, Y = bank), CallBankServiceHook
  ($FF78, continues at $FF7B in the target bank), CallBank3Service ($FF84, sound
  driver), ReadBank3Hook ($FF6E -> bank 3 ReadAnimScriptBytes).
- Object engine: UpdateAllObjects -> ObjTypeHandlers ($9A71) ; MoveObjAtSpeed,
  ApplyGravity/AddZVelocity, FindAttackTarget/ApplyHitReaction, AdvanceAnimScript +
  AnimCommandTable ($D2C6, commands $E0-$FE named AnimCmdXX_*).
- Loader: StartStageLoad (bank 0) + bank 6 LoaderStep/LoaderFrame, 14 commands
  (Loader0..LoaderC init/frame pairs; 3 = stage intro after a stage, 5 = game over).
- Sound: StartMusic, UpdateSound, UpdateMusicChannel/UpdateSfxChannel, 84 script
  commands (SndCmdXX_*), PCM sample player writing DMC_RAW (PlayPcmSample).

## Unresolved meanings (commentary/uncertainties.md)
Many object fields (Obj_03E0/04B2/04DF/04EE/0539/0548/0575/0584/05A2/05B1), stage-id
to level mapping beyond $03/$08, boss/scripted-enemy identities, $05F0 vs $05F2
(selection/character values), RAM code origin ($0532/$0600/$0720/$0728/$0782, L-003).

## Structural questions for redirect (concrete, bounded)
1. ObjTypeHandlers word table at bank 0 $9A71 (indexed by ObjType*2 in
   UpdateAllObjects $86BB): ~42 of the first 63 entries point into UNCLASSIFIED bank 0
   bytes (e.g. type 3 $D923, 8 $9638, $0D $C05C, $1B $BA53, $1D $DB1F, $20-$23
   $C2D5-$C3A1, $25 $C7FA, $26 $C919, $28 $C5A4, $29 $9E3B, $3A-$3F $E5BB-$DEDF).
   Prove the table extent (largest type stored to ObjType; types up to $5B are
   referenced) and add these handler roots: likely the biggest code gain left in bank 0.
2. Animation-script pointer tables bank 3 $D773/$D873 (ReadAnimScriptBytes) and the
   script data they address: extent/format for data classification.
3. Sound data in bank 3: tempo table $9D89, channel start pointers $9DA1 (8 bytes/id),
   effect pointers $944D, $9120/$9314/$9324/$9334 tables; follow pointers to classify
   music/effect scripts.
4. Bank 6 metasprite tables $C093/$C244 and frame tables $C2F6/$C319 (renderer), plus
   bank 4 sprite-group table $BCA8 and bank 5 map tables $A4D5/$A4E2.
5. Gameplay tables inside bank 0 gaps: HitReactionTable $88B8 (5-byte records),
   DirectionStepTable $9056, ScoreValueTable $9236, steering grids $A86E/$A887,
   attack hit-box tables $8B05/$8B23/$8B41, DpadDirTable $CAB0/$CAC0.
6. Graphics decompressor sets: bank 1 $8193 and bank 7 $825B pointer tables give exact
   compressed-graphics extents (lead L-002).
7. Unclassified bytes inside routines (260 gaps, many 1-24 bytes): several are the
   per-bank stub bytes executed only in another bank (documented); others are untaken
   paths (L-008).
8. Bank 1 slot $8006 -> $F4AB is a lone RTI that is also read as table data by $F513;
   check whether it should be data.

## Remaining gaps
None for current-source readability. Address-based branch labels (L_Bb_AAAA) remain
where local control flow has no established role. Editorial audit pending (final).

## Verification
`python3 /root/romatlas/scripts/commentary.py verify <workspace>`: ok=true,
byte_identical=true (commentary/verification.json, game.asm sha256 e3589842...).
Reading copy refreshed: commentary/reading-after.asm.
Toolkit: no new toolkit issues found in this phase (existing TK-001..003 unchanged).
