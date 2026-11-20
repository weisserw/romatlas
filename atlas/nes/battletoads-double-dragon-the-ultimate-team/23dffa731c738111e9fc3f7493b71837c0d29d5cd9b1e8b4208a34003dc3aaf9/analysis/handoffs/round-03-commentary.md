# Round 3 commentary handoff: COMPLETE (current-source readability)

- Map: pass-003 (maps/current.json == analysis/state.json, sha256 92863db2...452733; 1,660
  scaffold regions; 99.99% classified, 16 bytes unknown). Classification and source
  readability are reported separately; whole-ROM classification is still incomplete.
- Previous commentary preserved: analysis/round-2/commentary/ (pass-002, complete copy,
  moved before prepare); analysis/round-1/commentary/ untouched.
- Fresh baseline: commentary/ prepared from pass-003 (before/ byte-identical). Round-2 notes
  and tools were carried by physical range (pass-003 only added regions; no pass-002 region
  changed), rebuilt and reverified before any new edits.
- Reading copy: commentary/reading-after.asm (refreshed). Source: commentary/source/game.asm.

## Scope covered
- All 103 pass-003 regions: 49 data regions with role/format notes (e.g. SoundEventTable
  records, stage tables, ObjDrawHookTable, AsciiToTileTable, 15 motion scripts, map rows,
  fill) and 54 code regions (bank-switch module copies explained per bank, two unused
  jmp-table slots, 30 uncalled fragments with names or block comments).
- 149 obsolete round-2 "unknown" notes on these bytes removed/replaced; stale automatic
  note for TileUploadTables_B3 and the bank 1 $FF5D note updated.
- 7 remaining unknown runs (16 bytes) have honest boundary/alternative notes.
- Indexed table reads now use symbolic operands (Table-1 / Table-2 / Table-$70 forms) for
  15 reader bases.
- Whole-source check: no Sub_Bn_ names; every routine of >= 4 instructions has a block
  comment (EveryOtherFrame was the one gap; fixed). 145 older data regions keep the
  automatic consumer/evidence note accepted in round 2.
- review.json: 1660/1660 entries reviewed, 908 units; editorial_audit pending.

## Examples
- `ldx a:StageNextTables_B6-7,y` (bank 6 $972C) with the table note explaining the 0 =
  fall back to the $903A table case.
- WriteAsciiStringsToPpu (renamed from DecodeGfxToPpu): string format and AsciiToTileTable.
- DrawHookSlots / DrawHook_Type54 ... named from ObjDrawHookTable values.
- StepCameraYTowardA8 (bank 4 $B8BE, uncalled), LeftoverBankSnippets_B5 ($FF50).
- Bank 1/2/4/7 $FF78 block comments: why the module copy is not executed there.

## Unresolved (see commentary/uncertainties.md "Round 3")
- L-027 bytes: b0 $8D92, b4 $B979, b5 $ACFA, b6 $9745, b6 $EFE5 (lone $60), b0 $E536-$E537,
  b6 $FE24-$FE2C. New detail: $FE24 decodes as jsr HideSprite / stx OAMADDR / lda #2 /
  sta $00A2, where the sta operand is the A2 00 of ldx #$00 at $FE2D, rejoining $FE2F.
- Meanings of RAM targets of the stage tables ($C2, $A9, Obj_0584 step), $FC94 bit patterns,
  and record formats of scripts/maps remain mechanical.

## Questions for redirect
- $E536-$E537: bound the player frame index seen by MirrorPlayerAppearance (recipe in
  analysis/handoffs/round-03-disassembly.md).
- b6 $FE24: search for a computed/pointer reference to $FE24 (e.g. a $24/$FE immediate pair
  or a RAM-stored vector) to decide the overlapping-entry hypothesis.

## Gaps
- None in current-source readability. Classification gap: 16 unknown bytes. Final
  editorial audit not run (reserved for final acceptance).

## Verification
- `commentary.py verify`: ok=true, byte_identical=true (commentary/verification.json;
  game.asm sha256 9efe76ece157717077d63424f5370c369723fb2579751bb32ca4bf6d029ad36a).
- `commentary.py read`: ok=true -> commentary/reading-after.asm.
