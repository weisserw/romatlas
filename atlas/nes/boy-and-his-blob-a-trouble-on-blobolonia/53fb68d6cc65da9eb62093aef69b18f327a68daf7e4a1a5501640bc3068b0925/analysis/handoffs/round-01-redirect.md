# Round 1 — redirect handoff (COMPLETE) — recommendation: CYCLE

- Attempt directory: analysis/redirect/r01-a/ (ledger.md, unknown-complement-pass002.txt, tools rom.py,
  unk.py, musicwalk.py, streamwalk.py, rpc.py, exp_pharmacy.py, exp_treasure.py, exp_ending.py,
  music-touched.json, stream-touched.json, evidence/).
- Input checkpoint: pass-002 (maps/pass-002.json == maps/current.json, byte_identical true) and
  commentary-001 (commentary/source/game.asm). No map, state, source, ROM or toolkit changes.
- Classification unchanged in this phase: 98.48% (258166/262160; 3994 unknown). Proposed: 3993 bytes.

## Scope
Exact unknown complement (141 ranges) reviewed group by group against consumers in the commented
source; L-010..L-016 verified/refined; new connected pieces found (L-017..L-021); bounded pointer
walks for music and screen streams; gameplay milestones pharmacy, treasure, ending reached by RAM
shortcuts. Every range has a disposition in analysis/redirect/r01-a/ledger.md.

## Findings (all static unless noted)
- Tables bounded by consumer index domains: RoomFloorPtrs 256 words (lists contiguous to $C75C);
  metasprite tables $A000-$A2E7 (+records $A2E8-$BD3D); NotePeriodTable 64 words $85F2-$8671; 31-type
  object tables (hitbox/AnimScripts); RoomRowTable/SceneObjSet/SceneMintBase/RoomPickupIndex 256 each;
  bank-2 scene tables 5x256 + palettes 12x42 + status tiles ending exactly at RectStreamPtrLo $8878;
  PharmacyLabelText 7x16 ASCII; FormExitDY 28; BoyLadderFrameTable to $86E7; HummingbirdDY 16;
  GridSlotX $9A3B (read as $9A38,y y=3..6).
- Sample data: DmcSampleTable covers bank 7 $E000-$FFC0 contiguously; PcmSegmentTable covers bank 5
  $80A9-$B1DB (all bytes $14-$60).
- Pointer walks: music songs 1-4 (song 3 has 7 sections selected by StartMusic X=$80+n) touch every
  bank-1 music gap; stream walker (op byte = 2*handler index of JMP ($B9xx)) decodes all 256 scenes
  with zero errors and covers all bank 2/3/4 gaps; bank-6 gaps lie inside 110-byte object-set templates.
- Code correction: bank 7 $C088-$C08F (now Data) = LDA #0 / JSR SetPRGBank16K / JMP $8000; targeted by
  JMP $C088 at bank 1 $8000 and bank 6 $8000 (both unknown). Reachability unknown.
- Commentary corrections to carry: records start $A2E8 (MetaspritePtrsAltB/Data_A301 are record bytes);
  "music data $863B" is still NotePeriodTable to $8671; SetScriptGrid_DropLoopEnd,y operand = GridSlotX;
  L-005 text wrongly cites L-018 for music (it is L-016).
- Runtime (artificial, ingame0.snap + RAM writes; recipes in analysis/scenarios.json S10-S12):
  pharmacy (assortment 1 window, labels from $9024 displayed), treasure pickup (room 8), ending
  (EndingCountdown $B4=1 -> Emperor scene $FD, bank 5 PCM player executed). All new instruction
  fetches (64/178/178 addresses) fall in existing Static code; no data/unknown conflicts.

## Ready leads (analysis/leads.json) — recipe: annotate per ledger bounds, Static provenance
| Lead | Content | Bytes |
|---|---|---|
| L-010 | b7 $C3E2-$C522 RoomFloorPtrs tail + first list byte | 321 |
| L-011 | b7 19 DPCM gaps $E0AC-$FF74 | 642 |
| L-012 | b5 26 PCM gaps $80A9-$B035 | 872 |
| L-013 | b0 $856A-$856E SparkleSpriteTable | 5 |
| L-014 | code b1 $8000/$8009, b6 $8000 (+ reclass b7 $C088-$C08F Data->Code) | 9 (+8) |
| L-015a-g | metasprite ptrs 237, period table 29, type tables 99, RoomRowTable 98, scene tables 551, pharmacy text 111, FormExitDY 1 | 1126 |
| L-016a-c | music 292, streams 597, templates 54 (run musicwalk.py --sections / streamwalk.py) | 943 |
| L-017 | b0 $869B-$86C6 BoyLadderFrameTable | 44 |
| L-018 | b0 $BA3E-$BA41 HummingbirdDY (+$BA42-$BA43 pattern-only, reviewer decides) | 4 (+2) |
| L-019 | b6 $9A3B-$9A3E GridSlotX | 4 |
| L-020 | b6 per-set/per-scene table slots (10 ranges) | 16 |
| L-021 | b7 $CBB7-$CBBB "BOYH",$FF text | 5 |
Total 3993 unknown bytes (3991 structure-backed, 2 pattern-only) + 8-byte Data->Code correction.

## Rejected / deferred
- b0 $9AB8 (1 byte, $60 after an RTS, no reference/branch target): retained unknown; a lone return
  opcode is insufficient for code and nothing supports data. Not actionable without a caller.
- L-006 (duplicate bank tails), L-007 (criterion spot-check), L-008 (reachability of debug islands),
  L-009 (overlap $8312/$8313): verification/reachability items, no unknown bytes; left as is.
- Natural ending trigger (jack form in SpecialRoom) and the post-music restart not reproduced: the
  ending code ran from the supported $B4 entry; further runs add no classification evidence.
- Root beer rocket (S09) still attempted only; no unknown bytes depend on it.

## Recommendation
CYCLE: disassembly pass should apply L-010..L-021 (expected 98.48% -> ~99.9996%, 1 byte unknown),
then full commentary (carry corrections above), then redirect for the final residual/terminal review.
