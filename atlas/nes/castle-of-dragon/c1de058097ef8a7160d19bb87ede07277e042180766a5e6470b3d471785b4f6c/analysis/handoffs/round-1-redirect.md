# Round 1 redirect handoff (attempt r1-a1)

- Attempt dir: analysis/redirect/r1-a1/ (tools/, evidence/, l004_extents.json,
  static_regions_now_fetched.json, leads.before.json). Scratch removed.
- Input checkpoint: pass-001b + commentary source (byte-identical). No map/source edits made.
- ROM classification unchanged: 100.00% (131087/131088; unknown $C415 / phys 0x1C425).
- Methods: static source/ROM review (consumer formulas, pointer/extent checks, reference
  searches), bounded runtime via romatlas serve (configured release binary), RAM patches
  for the ending, natural test-mode route. Trace = aggregates only (capacity 0).

## Findings
1. L-003 ENDING REACHED (artificial state). Correction: $053B=$7F is wrong - SetupStage only
   loads the dragon directly when StagesCleared < $7F (with $7F the palace walk loads, S7-WALK).
   Recipe: snap_map0, $0511=$F0, $0512=$80, $052A=7, enter; fight with $030A refills; first
   natural hit -> write $03CA=0; next natural hit -> FinalBossDefeated -> PlayEnding scene 1 and
   scene 2 staff roll (slot 8 ObjFrame $F6..$FF) to "THANK YOU FOR PLAYING / THE END" (endless
   yield). 17 Static code regions got fetches (contexts tagged artificial=true), incl.
   $D71B-$DA5F (318 starts), $E3E4-$E4C8 (CheckBossHit), $AEC0-$B44A dragon AI.
   List: static_regions_now_fetched.json; coverage evidence/cov_ending.json.gz.
2. Staff-text consumer resolved (L-007): bank 6 $8000-$81FF = 256 frame pointers, all into
   $8200-$90CA; grids sized by bank-4 $BA00/$BB00; frame $FF ends at $90D7 = byte before code
   $90D8; frames $F6-$FF tile $8F8B-$90D7 (the ASCII staff roll), drawn by DrawMetasprite.
3. L-004 bounded split proposals (l004_extents.json, read-trace cross-check: zero regions never read):
   | bank | range | role | extent evidence |
   |---|---|---|---|
   | 4 | $8000-$8E3F | map layout, 57 pages x 64 | page*64 formula; next base $8E40 |
   | 4 | $8E40/$8F1C/$8F9B/$9017-$9070 | block attribute sets (220/127/124/90) | BlockAttrBase table |
   | 4 | $9071/$9E31/$A621/$ADE1-$B380 | 16-byte block defs (220/127/124/90) | def counts == attr counts; max map id per page group 0-17/18-33/34-49/50-56 = 219/126/123/89 |
   | 4 | $B381-$B9FF | zero fill | all $00, never read |
   | 4 | $BA00-$BAFF / $BB00-$BBFF | metasprite width / height | indexed by ObjFrame (256) |
   | 5 | $8000-$8E3F | collision map (57 pages) | GetTerrain formula |
   | 5 | $8E40-$8FFF | zero fill | all $00, never read |
   | 5 | $9000-$903F (+zero to $90FF) | duration table | consumer index AND #$3F |
   | 5 | $9100-$91FF | envelope ptr table (72 used) | targets $9280-$98D0 |
   | 5 | $9200-$927F | sound header ptr table (64) | StartSound ASL index |
   | 5 | $9280-$98EA | envelope data | first header target $98EB |
   | 5 | $98EB-$BC67 / $BC68-$BFFF | headers+track data / zero fill | last nonzero $BC67 |
4. L-006: natural test-mode run: WEAPON +$80 then EXIT gives armour (PlayerEquip $80); the
   "STAGE" row edits MaxEnergy (4..8), no stage select. 7 Static regions fetched (natural).
5. L-002 review: structural claims sound (see leads.json); e.g. $CCE6-$CD06 exactly fills the
   gap between SoundCmdRtsTbl handlers ending $CCE5 / starting $CD07. Reachability unknown;
   only wording issue: *_Unused labels claim reachability -> rename to neutral names.
6. L-001: no change (rejection review below). L-005 marked resolved (commentary, static).

## READY leads (analysis/leads.json)
L-003 (provenance upgrade, artificial context), L-006 (provenance upgrade, natural), L-004 and
L-007 (typed splits of already-classified data, 0 byte gain, role/extent evidence),
L-002 (commentary wording). Blocked: L-001.

## Deferred / rejected avenues
- Natural dragon win without HP patch: head position $14 blocks hits and my controller could not
  raise it; diminishing value since the defeat path itself ran naturally after one patch.
- Natural route to quest items/armour (without test mode): not needed for classification; stage
  item rewards exist (CheckReward) but full play-through is long - deferred, low value.
- ObjFieldD/F, enemy/sound identities: semantic only, no classification impact; no concrete
  consumer beyond clears found in round-1 commentary - not actionable now.
- Ending @FadeAway part of PlayEnding: not executed because scene 2 ends in an endless yield
  (UpdateEndingScene2 step 1); stays Static (structurally sound, unreached in practice).

## Rejection review (L-001, the only unknown byte)
- Claim needed: $C415 (phys 0x1C425, bank 7 fixed) is code (empty RTS stub) or data/padding.
- Decisive evidence: byte $60 after ClearSpritesAndObjects' RTS $C414 (beq $C40E targets $C414,
  so $C414 is the real routine end); InitFramePool starts $C416 (Reset JSR). No absolute or
  split/RTS-trick reference ($15 $C4 none; $14 $C4 only in bank-1 tile image 0x60FB); no
  indexed table base nearby; never fetched in any trace incl. ending and test mode.
- What it rules out: a live/table entry into $C415; a fill-byte role matching ROM fill ($00 at
  bank 6/7 tails); alignment ($C416 not aligned).
- Constraints together: code banks are otherwise contiguous; dead routines do exist (L-002), so
  an orphan empty stub is plausible; a stray assembler byte is equally plausible.
- Strongest competing interpretations: dead 1-byte RTS stub vs leftover data/padding byte.
  Neither has structure beyond one opcode; evidence-policy explicitly says an unreferenced return
  opcode lacks sufficient structure. Remaining reason: code-vs-data materially ambiguous ->
  keep unknown. A revised test would need an external reference (none exists in ROM).

## Recommendation
(a) Another disassembly/commentary cycle with the ready batch (L-003, L-006 provenance
upgrades; L-004/L-007 typed splits; L-002 wording). Classification % will not change ($C415
remains unknown by design); the gains are observed provenance within 24 Static code regions (~2.4 KB, partly fetched) and
specific region roles for banks 4/5/6 data. After that, redirect is likely terminal-incomplete
(only $C415 left, review above).

Processes: all romatlas serve instances started here were stopped.
