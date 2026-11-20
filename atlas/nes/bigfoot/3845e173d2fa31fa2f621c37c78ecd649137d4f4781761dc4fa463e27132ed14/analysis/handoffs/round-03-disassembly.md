# Round 3 disassembly handoff

Status: COMPLETE (pass finished; project still incomplete only editorially).
Checkpoint: pass-003 (maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json): validate ok, byte_identical true, map sha256 fb2e34078159abd7b15384fd7b05f582e3423e3a43954cbc0b7607b0211cfd24.

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

## What changed (118 map.annotate edits; no kind changes; 1,385 -> 1,398 regions)
- Bank 0: region 675 -> TerrainEffectA/B/C/D ($9BA9/$9BD4/$9BFF/$9C2A, 43 B each) + MetatileTerrainMap $9C55-$9FFF; region 676 ($A000-$A844) = MetatileTerrainMap continuation (191 x 16 total).
- Bank 2 (16K): region 758 -> RaceSprOffsetX/Y/XFlip/YFlip ($9E91/$9EC1/$9EF1/$9F21, 48 B each), RaceSpriteLayoutPtrs $9F51-$9FB6 (51 words), RaceSpriteLayoutData $9FB7-$9FFF; region 759 trimmed to $A000-$AC36 (continuation, layouts to 49); new 1-byte generic $AC37.
- Bank 2 tail: region 762 -> PaletteScriptData continuation $B082-$B0B6, TitleRevealScriptPtrs $B0B7-$B0BE, TitleRevealRowTiles $B0BF-$B0E6, generic $B0E7-$B4A9.
- Fixed: region 962 labelled SecretButtonSequences ($FE12-$FE35, 9 rows x 4 sequences).
- L-021: 72 Static code regions (2,176 B) -> Observed (every instruction start fetched in r02 traces); 27 (3,452 B) keep Static with "partly executed k of n" evidence. Observed code now 26,456 B, Static 11,451 B.
- Plan/tools: analysis/disasm-r03/plan.json (each entry tagged with its lead), tools/verify.py, l021.py, plan.py, apply.py.

## Lead outcomes
L-018, L-019, L-020, L-021, L-022: verified and applied (details in analysis/leads.json). Rejected/blocked: none. Do not retry.

## For commentary round 3
- New map labels (generated source now defines them): TerrainEffectA-D, MetatileTerrainMap, RaceSprOffsetX/Y/XFlip/YFlip, RaceSpriteLayoutPtrs (at $9F51, where round-2 commentary placed `RaceSpriteLayouts`), RaceSpriteLayoutData ($9FB7), TitleRevealScriptPtrs, TitleRevealRowTiles, SecretButtonSequences. Reconcile notes to avoid duplicate/conflicting names.
- Provenance changes split/merge .org blocks differently from pass-002; re-prepare from pass-003.
- Use r02 G-series notes (handoffs/round-02-redirect.md) for semantics.

## Open (not classification blockers)
- Roles of generic 2:$B0E7-$B4A9, 2:$AC37, 4:$A474-$A873, 5:$8388 blocks.
- Natural reachability of remaining Static code (288 regions).
- No toolkit issues found this pass (TK-001..TK-006 unchanged; analysis/toolkit-issues.md).
