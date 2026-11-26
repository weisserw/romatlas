# Round 4 — disassembly handoff

- Input: pass-003. Output checkpoint: **pass-004** (maps/pass-004.json == maps/current.json; generated/pass-004;
  analysis/pass-004-verification.json ok, byte_identical true; store analysis/state.json). input/game.nes unchanged.
- Code is identical to pass-003. Only Data/Text/Padding kinds, splits/merges, labels and evidence changed.
  Regions went from 5551 to 4306.

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

## Leads
- **L-019: accepted, all 125 candidates.** 2189 B became Data (Padding 1700, Text 489).
  - Re-checked with tools/r04_verify.py; results in analysis/round-4/l019_verify.json.
  - Bank 3 set $9F23 records 76..103 rest on stride evidence only (conf 0.8).
  - Bank 7 $D899 is unchanged (Text).
- **L-020: disassembly part accepted.** The commentary part is still ready (see below).
- Rejected: none.

## Changed ranges for commentary
Full list: analysis/round-4/changes.json.

**New labelled Data regions:**

| Bank | CPU range | File range | Label | Content |
|---|---|---|---|---|
| 2 | $8A9D-$8D00 | 0x8AAD-0x8D10 | FS_FlameCels | 17 x 36 B flame frames |
| 2 | $93A0-$941F | 0x93B0-0x942F | FS_PoseCels | 8 x 16 B pose frames |
| 5 | $AD28-$AD67 | 0x16D38-0x16D77 | CL_ArenaMap | arena metatile map |
| 5 | $AD68-$AEFF | 0x16D78-0x16F0F | CL_ArenaMetatiles | 24 x 17 B metatile records |
| 5 | $AF00-$BEFF | 0x16F10-0x17F0F | CL_MsprCels | 32 x 128 B fighter frames |
| 1 | $A911-$BD58 | 0x6921-0x7D68 | MT_MsprCels | Mate Toss metasprite frames |

**Unlabelled pointer tables split out:**
- Bank 2 $9358-$9379: FS_PoseFramePtrs, 17 words.
- Bank 2 $937A-$939F: FS_FlameFramePtrs, 19 words, 38 B. The commentary's 210 B extent is wrong.
- Bank 1 $A8AD-$A910: MT_MsprFrames, 50 words.

Existing commentary label names were kept and no map label was added for these tables.

**Metatile map/set Data regions (unlabelled, consumer evidence in the map):**

| Bank | Maps | Sets |
|---|---|---|
| 0 | $A0C8-$A107; $A148-$A2C7 (6 maps) | $A2C8-$A580; $A593-$A8A0; $A9D9-$AAC6; $AAE4-$ADBE |
| 1 | $9C8B-$9CCA | $9EFB-$A8AC |
| 2 | $96A6-$96E5 | $96E6-$9C24 |
| 3 | $9561-$95A0 | $9F23-$A60A (104 records) |
| 4 | $9C4E-$9C8D | $A666-$B347 |
| 6 | $81D6-$8255 (2 maps) | $8256-$88A4 |
| 7 | $D923-$D962 | $D5C8-$D880; $D963-$DBD7 |

The commentary labels below span these sets/maps and must be re-cut:
- DR_MsprFrames_43 (bank 3).
- B4_SfxStreamPtrs (bank 4).
- B1/B2/B5 propagated SfxStreamPtrs blocks.
- DV_ActorDefaults (bank 6, $80E6-$8255 + set).
- B5_SfxStreamPtrs ($ACBA-$AD68 over the arena map).

**Kind-only changes (L-019):**
- Text -> Data:
  - Bank 3 DR_RiderDY/DR_RiderDX/DR_DinoY pieces $84CF-$8567.
  - Bank 6 DV_CavemanDX rows $8BF5/$8C33/$8C4A.
  - Bank 5 CL_PoseY $A7DC+34.
  - Bank 2 FS_AmbientY $8188+10.
  - Bank 1 $9D79+10 and $9D92+10 (generic data, consumer unknown).
- Padding -> Data:
  - Bank 1 MT_CavemanYOfs $8B59+26.
  - Bank 3 $8599-$85BA and $85BB-$85C2 (split at the RiderY/RiderX boundary).
  - Bank 4 SR tables $8689+90, $86F3+44 and $8798+50.
- Bank 7 $D257 (1 B) became Data, the 6th StandingsNameOffsets entry. The Text region now starts at $D258.
- Bank 0 leading zero runs stay Padding: $A581+18, $A9AC+45 and $AAC7+29.

**Still ready for commentary (L-020):**
- Fix the false "no reference found" text for bank 2 $8A9D and the "Generic data after the frame records" text after
  MT frames.
- Fix the misplaced "propagated from bank 0 $FF00" headers.
- Add operand notes: bank 5 `lda $8002,x` is a dummy timing read, and bank 6 `cmp $ae2b,x` is DV_OutcomeTable-12.

## Open (no classification effect)
- Bank 3 course map $944B-$9560: the extent is unproven and it was left as generic pieces.
- Bank 6 map $80E6: the set comes from the caller, so it was not merged.
- Bank 2 $9420-$944B and bank 1 $9D6B-$9DA7: consumer unknown.
- L-004 (reachability) and the remainder of L-005.
- Toolkit: no new issues found. No processes left running.
