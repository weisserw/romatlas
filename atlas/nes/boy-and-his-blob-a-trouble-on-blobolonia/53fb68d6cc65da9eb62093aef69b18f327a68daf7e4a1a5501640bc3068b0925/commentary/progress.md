# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round 2 (checkpoint commentary-002, input map pass-003)
- Round-1 commentary preserved whole under analysis/round-1/commentary/ (and its tools under
  analysis/round-1/commentary-tools/); re-prepared from pass-003 with commentary.py prepare.
- Notes (analysis/commentary-tools/notes/*.txt, keyed by bank + CPU address) carried forward unchanged where the
  physical range was unchanged; cmt.py now also parses Text regions, emits '(continues X)' pointers for split
  table blocks and splits spanning units across scaffold entries in review.json.
- Rewritten for pass-003: all former-unknown notes (RoomFloorPtrs tail, DPCM/PCM, SparkleSpriteTable, pharmacy text,
  FormExitDY, BoyThrowFrameTable, HummingbirdDY, NotePeriodTable to $8671, music data, metasprite tables
  (records start $A2E8; MetaspritePtrsAltB/Data_A301 removed), scene tables, screen streams, object-type tables,
  AnimScripts, RoomPickupIndex, RoomRowTable, GridSlotX, templates), new code (SndVecRestart, SndVecStopSfx,
  ObjVecRestart, RestartViaBank0, AddAToZpWord), DebugTextBoyH, $9AB8 residual.
- Redirect r1 fixes applied: records start $A2E8; NotePeriodTable to $8671; GridSlotX operand (GridSlotX-3,y);
  stale L-018 reference removed (music text no longer cites leads).
- All 69 generic _SkipN/_LoopN labels renamed (48 BitmapStream bit steps, 21 others).
- Prose fixes: '&BridgeAtAnchor' substitution artefact -> '&$7F' (4 places); raw RAM addresses in prose replaced by
  names (ResetPlayerAfterLife, fuse/torch, grid, LandOnFloors, AnimatePalettes); 'Unused*' names that only lacked a
  caller renamed (WhistleToneBody, UnwrittenFlag92).
- Verification: commentary.py verify byte_identical true (source sha256 ebbee43d...e5f2); reading copy refreshed.
- review.json: 1365/1365 entries reviewed, 396 units, every Code entry has a unit; editorial_audit pending.

## Round 3 (checkpoint commentary-003, input map pass-004) — final editorial acceptance
- Round-2 commentary preserved whole under analysis/round-2/commentary/ (tools under analysis/round-2/commentary-tools/);
  re-prepared from pass-004 with commentary.py prepare. Frozen before/ differs from round 2 only at b0 $9AB8 (now Code)
  and 25 bank-6 regions (Padding -> Data); all other notes carried forward unchanged by bank + CPU address.
- Changed regions: UpdateBoyStop_OrphanRts (structural only, no reachability claim), template ObjAnimTimer/ObjAnimOn
  slot 3-10 runs labelled inline (24), TypeBounces $A0B3 note, file header updated, 'Padding' caveats removed.
- Editorial fixes: 28 prose corrections (7 value->RAM-name substitution artefacts, JackActive -> FormFlag74, raw RAM
  addresses in prose -> symbols), EnterRoomRight redundant inline shortened, '(bubble trap?)' reworded.
  cmt.py review: terse unit summaries now keep a second sentence.
- Verification: commentary.py audit ok, byte_identical true, 1365 regions / 1453 units reviewed, editorial_audit
  reviewed (commentary/audit.json; copy analysis/commentary-003-audit.json); source sha256 3cdcb947...7131; reading copy refreshed.
