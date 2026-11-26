# Commentary progress

## Round 5 (checkpoint pass-005, 512 regions)

- Baseline: the map changed (pass-004 -> pass-005: region 49 $8A2D Static -> Observed;
  evidence text added to 8 gated NMI-job regions and region 315 FillNametable2/3). The whole
  round-4 commentary was moved to analysis/round-4/commentary/ and `commentary.py prepare`
  made a fresh verified baseline (byte-identical).
- Carry (analysis/tools/commentary-round5/carry.py, the round-4 method): round-4 game.asm
  copied, 1 region marker rewritten (region 49 provenance). No insert or .incbin split.
  Marker list equals before/game.asm; verified byte-identical before any edit.
- Edits (analysis/tools/commentary-round5/batches/01-l020.txt, applied with replace.py),
  L-020: UpdateLaneBall gutter clamps (left reached naturally, $8A2D commented);
  AimAndWaitForThrow aim range corrected to $C4-$E4 (defect: said $C5); RunSpinMeter
  CONTROL label; SignGirlTiles bowler digit tiles; header paragraph on dormant NMI jobs and
  FillNametable2/3; dormant wording at NmiUploadAttrRow, NmiVramRead, NmiFillRect,
  NmiCopyStrip column path, CopySpritesToDmaPage reversed copy, the NMI WaitSeconds countdown,
  and the StripMode/SpriteCycleMode symbol comments; FillNametable2/3 unreachable note.
- review.json regenerated with cm.py review: 512/512 reviewed, 988 units; 509 entries equal
  round 4, region-48 (UpdateLaneBall unit) and region-315 summaries updated, region-512's
  hand-written summary carried. editorial_audit stays pending.
- Verification: `commentary.py verify` ok, byte_identical after the carry, after the batch
  and at handoff; reading copy refreshed.
- Deferred ranges: none for readability. Final editorial audit reserved for final acceptance.
