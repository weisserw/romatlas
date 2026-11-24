# Round 3 commentary handoff

Status: COMPLETE (current-source readability on checkpoint pass-003).
Editorial audit pending (final acceptance only). Classification unchanged:
100.00% (131086/131088; 2 unknown bytes, L2-01 terminal) - not complete.

## Scope
- Map changed vs round-2 baseline -> entire round-2 commentary preserved at
  analysis/round-2/commentary/ (diff -r identical); fresh baseline prepared
  (601 scaffold entries). Names/comments carried by bank+CPU address via
  commentary/tools/notes.txt + overlay.py (0 lost; only new provenance headers).
- Affected records: $F4FD note (now S13b-observed), CheckExtraLifeHero doc
  (static-only award block), split-point notes ($9439/$9C00/$A514), listing
  preamble explains in-routine provenance headers; bottom-row chain docs
  fixed (rows 20-11), CopyRowsBottom2400_Chain documented.
- Whole-source sweep: ~70 raw ROM addresses in prose -> names (tools/r3/),
  fixing several wrong round-1 descriptions (game-logic phase, gameplay NMI,
  PlayerState_Walk, SetupTower, game-over screen); 11 address-named
  trampolines renamed; raw operands $0661/$400D/$01F0 symbolised.
- review.json: 601/601 reviewed, 857 units.

## Verification
`python3 /root/romatlas/scripts/commentary.py verify .` -> ok, byte_identical
(game.asm sha256 063b8aa6...). Reading copy refreshed (commentary/reading-after.asm).
input/game.nes unchanged (sha256 3bc1f133...).

## Unresolved / remaining gaps
None in current-source readability. Open meanings in commentary/uncertainties.md
(bank 3 $A500-$AEFF bitmaps, SoundMisc+6, orphan routines, $6391 entry,
write-only $01F0/$0214, InitTowerVars06B3). Observed evidence for S13b/S14
is artificial (execution, not natural reachability). No toolkit issues.
Next: final editorial audit/acceptance.
