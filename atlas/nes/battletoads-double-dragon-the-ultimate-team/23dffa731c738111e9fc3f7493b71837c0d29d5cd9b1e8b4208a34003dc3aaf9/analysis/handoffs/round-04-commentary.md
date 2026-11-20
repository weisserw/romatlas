# Round 4 commentary handoff: COMPLETE (current-source readability)

- Map: pass-004 (maps/current.json == analysis/state.json, sha256 514387bd...94bd3b96;
  1,661 scaffold regions; 262155 / 262160 bytes classified, 5 unknown). Classification
  remains incomplete; readability and classification are reported separately.
- Previous commentary preserved: analysis/round-3/commentary/ (full pass-003 copy);
  analysis/round-1/ and round-2/ untouched.
- Fresh baseline: commentary/ prepared from pass-004 (before/ byte-identical). Round-3
  notes/tools/uncertainties carried by physical range (pass 4 only added regions over
  former unknown runs), rebuilt and verified before edits.

## Scope covered
- 3 new pass-004 regions: b0 $E536 `lda #$02` -> TryLatchOntoPlayer_Id2 (unused alternate
  entry, hit-box id 2); b6 $FE24-$FE2B -> HideSpritesThenOamDma_Fragment (truncated
  sprite-clear + OAM-DMA prefix, unreferenced); b6 $FE2C data note (leftover $8D opcode;
  would clobber RunRamColumnCode's $A2 input if executed).
- Obsolete round-3 unknown notes for these bytes removed; $E522 note updated.
- All other classified source: round-3 review credit reused (unchanged regions/notes);
  1661/1661 review entries reviewed, 910 units; no Sub_Bn_/region_ labels remain.
- 5 lone $60 unknown bytes (L-027: b0 $8D92, b4 $B979, b5 $ACFA, b6 $9745, b6 $EFE5) keep
  honest notes.

## Unresolved
- Reachability of the three L-028 regions (no caller/reference found); L-027 dead-rts vs
  padding. See commentary/uncertainties.md "Round 4". Older round-3 items unchanged.

## Gaps
- None in current-source readability. Classification gap: 5 unknown bytes. Final editorial
  audit not run (reserved for final acceptance).

## Verification
- `commentary.py verify`: ok=true, byte_identical=true (game.asm sha256
  2894d99b9399be48aa197184dd135c4a8e52c91cb6c20259b8e6c5756d874576).
- `commentary.py read`: ok=true -> commentary/reading-after.asm.
- Rebuild: (in commentary/) python3 tools/annot.py build; commentary.py verify;
  python3 tools/fill_review.py; commentary.py read.
