# Redirect attempt r03-a1 (round 3) — research record

Input checkpoint: pass-003 (maps/current.json == maps/pass-003.json; analysis/state.json), 100.00% classified
(262160/262160, 0 Unknown). No map/state/source/ROM change in this attempt; no emulator/server run (static only).
Scripts in this directory run from the workspace root (read-only on maps/current.json, commentary/source, input ROM).

Pre-retry record: r02-a1 audited the Code graph (JSR/JMP/branch targets, fall-in/islands). It did not audit the
role of non-code kinds (Padding vs Data vs Text) against consumers. Changed inference: L-014 showed Text bounds can
be wrong, and the phase-1 Padding/Text heuristics (uniform run / ASCII range) never consulted consumers. Expected
discriminating result: consumer reads or anchored structures inside Padding/Text regions. Stop condition: no anchors.

## 1. Complementary Code audit (audit2.py -> audit2.txt)
- H (absolute reads of ROM addresses that resolve into Code): 0. P (non-uniform Padding): 0. T (non-text bytes in
  Text): 0.
- I (indexed reads with base inside Code): 4, all benign:
  - bank 5 $8074 `LDA $8002,X` with X=0 after CL_Delay: discarded 4-cycle timing read in CL_SplitScreen.
  - bank 5 $93B4/$93BA `LDA $93DB/$93DC,X`, X=2*NumPlayers>=2: CL_BracketHandlers addressed from $93DB (already commented).
  - bank 6 $AD22 `CMP $AE2B,X` = DV_OutcomeTable-12 (X=DvVar043E <= $11): rows below 12 would read code bytes
    $AE2C-$AE36; a game quirk, not a boundary error. DV_OutcomeTable $AE37-$AE4F (25 B) unchanged.
- Code view contexts: all consistent. The only irregular contexts are the 35 per-bank reset-stub copies at $FF00
  ([None,None,None,odd bank]), which is correct for MMC1 power-on.

## 2. Pointer anchors into "no consumer" generic data (ptrscan.py, ptrscan2.py, anchors.py)
- **Bank 2 $8A9D-$8D00 (612 B)**. FS_FlameFramePtrs ($937A) holds 19 words: $8A9D,$8AC1,...,$8CDD (stride $24).
  Entries 10/11 repeat 12/13, so there are 17 distinct frames. The consumer chain is FS_SetFlameFrame2 ->
  MsprData -> DrawMetasprite (FS_DrawFlames).
  - Each frame is 9 OAM records (Y, tile, attr, X); attr bit 4 hides a sprite (DrawMetasprite `and #$10`), and the
    unused slots are `00 FF 10 00`.
  - $8CDD+$24 = $8D01, which is the start of Static Code. The extent therefore matches exactly.
  - The commentary's "no reference found in this bank" is false.
- **Bank 2 FS_FlameFramePtrs extent**. It is really $937A-$939F (19 words), not 210 B. FS_PoseFramePtrs targets
  $93A0..$9410 (stride 16), so $93A0-$941F = 8 pose frames x 4 OAM records. $9420-$944B (8 OAM-like records + 12 B)
  has no consumer found.
- **Bank 5 $AD28-$BEFF, a complete consumer partition**:
  - map $AD28-$AD67 (64 B, CL_DrawBackground).
  - set $AD68-$AEFF (max map index 23 -> 24 x 17 B ends exactly at $AEFF).
  - CL_MsprFramePtrs: 32 words $AF00..$BE80, stride $80 -> $AF00-$BEFF, ending at the reset-stub copy $BF00.
  - The commentary labels B5_SfxStreamPtrs (175 B) over $ACBA-$AD68 and leaves $AD79-$BEFF unlabelled, under a
    misplaced "propagated from bank 0 $FF00" header.
- **Bank 1 MT_MsprFrames**. There are 50 pointer words $A8AD-$A910, and the frames run $A911-$BD58: the last
  target $BCD9 + $80 = $BD59, the start of Code.
  - The commentary block ends at $B973 and calls the rest "Generic data after the frame records".
  - The map Text regions $B974+13 and $B982+9 are OAM records inside frame $B959.

## 3. Metatile maps/sets (mtile.py -> mtile.txt)
The consumer is UploadMetatileScreen (8x8 map, 64 entries all read via PrepareMetatileRow). MetatilePtrFromA uses
17-byte records; the attribute byte is read first, then 16 tiles. The scan found every `LDA #lo/STA $A4/LDA #hi/STA $A5`
(map) paired with the set load in effect, verified by reading each routine (bank 0 DrawTitleScreens,
DrawNumPlayersScreens, DrawCharSelectScreens, DrawMainMenuScreens, DrawCavesOfFameScreen, DrawMenuLowerScreen).
- 17 map/set uses in banks 0-6. Set end = set + 17*(max index+1).
- Independent end checks:
  - bank 1 set ends $A8AC, just before MT_MsprFrames $A8AD.
  - bank 5 set ends $AEFF, just before the frames.
  - The map is adjacent to the set in banks 2, 5 and 6.
- No Code lies inside any derived extent.
- Padding/Text regions inside: rolefix.py -> rolefix.json (81 regions; 1080 Padding + 272 Text bytes, 3 partial).
- **Stride proof (stride17.txt)**:
  - Every 16-byte Padding region after bank 3 set $9F23 (23 of 23) starts at set + 17k + 1, the tile bytes of
    record k, up to record 102.
  - The same holds after bank 4 set $A666 (48 of 48, up to record 145).
  - These "padding" runs are uniform metatiles (16 equal tiles).
  - Bank 3's commentary calls this range DR_MsprFrames_43, and bank 4's calls it B4_SfxStreamPtrs. Both labels are
    wrong (they are inherited labels).

## 4. Padding read directly by code (padanchor.py -> padanchor.txt)
- bank 0 maps/sets as above.
- bank 1 $8B59+26: MT_CavemanYOfs, `ADC $8B5A,X`.
- bank 3 $8599+42: inside DR_ActionRiderY/X, the 83-entry tables proved in L-013.
- bank 4 $8689+90 and $86F3+44: SR_RunnerDX/DY, `ADC $8689/$868A/$8717,Y`.
- bank 4 $8798+50: SR_ChaserDX, `ADC $8798/$8799,X`.

The commentary already says "not padding" for bank 3, but the map kind is still Padding.

## 5. Text regions that are numeric tables (textaudit.py -> textaudit.txt)
The phase-1 ASCII heuristic matched value runs ($43='C', $46='F', $50='P'). These are read arithmetically:
- bank 3 $84CF-$8567 (8 regions, 128 B; LDA ...,Y / ADC).
- bank 5 CL_PoseY $A7DC+34.
- bank 2 FS_AmbientY $8188+10.
- bank 6 DV_CavemanDX rows $8BF5/$8C33/$8C4A.
- bank 6 $8312-$8741 (12 regions, inside the anchored metatile set $8256-$88A4).
- bank 1 $9D79/$9D92: repeated ascending rows inside sound data.
- bank 7 $D257: the 6th StandingsNameOffsets entry $32 (the table is 00 0A 14 1E 28 32), the same error type as L-014.
- bank 0 tile runs 'ABCDEFG...' inside the anchored sets.

Borderline cases:
- bank 0 $A3E5 'CAVEU.DF.FAME' and $A3FA: letter tiles inside set $A2C8-$A580 records.
- bank 7 $D899 'EFMNOPWXYZU': after set $D5C8's derived end $D880, 7 bytes into would-be record 42. Unanchored, so
  it is left as optional.

Consolidated list: l019_candidates.py -> l019_candidates.json. It has 125 regions, 1700 Padding + 505 Text bytes,
and no Code.

## 6. Commentary-only observations (for the editorial/commentary pass)
- Auto-propagated sound-driver labels: in banks 1-6, B*_SfxStreamPtrs (4 B in bank 0) swallows the following
  blocks.
  - B1: 693 B. B2: includes $B000 3840 B. B4: the $A666 metatile set. B5: the $AD28 map.
  - The "propagated from bank 0 $FF00" header is misplaced before them.
- DV_ActorDefaults (584 B) spans the bank 6 maps $80E6/$81D6/$8216 and set $8256.
- bank 5 `lda $8002,x`: dummy timing read.
- bank 6 `cmp $ae2b,x` = DV_OutcomeTable-12.

## 7. Avenues considered and not pursued
- L-004 roots: classification sound. Each has a coherent contract with established variables (MT_RankByDistance,
  UpdateBestScoresHigh, etc.).
  - No new reference was found by anchors.py (imm-pair/abs scans).
  - A natural Games-mode run could only add reachability, so it was not repeated (S16 reason unchanged).
- CL_Bracket4/5: same selector as the demonstrated cases; not rerun.
- Runtime confirmation of the flame/CL frame reads: not needed. The static pointer chain plus exact extents
  suffices for the Data role. An optional recipe is in L-020.
