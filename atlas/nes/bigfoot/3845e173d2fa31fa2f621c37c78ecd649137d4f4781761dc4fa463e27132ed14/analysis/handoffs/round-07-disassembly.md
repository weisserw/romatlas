# Round 7 disassembly handoff

Status: complete (editorial map-note fix only). Restored analysis/state.json (= pass-006); no re-baseline.

Checkpoint **pass-007** (maps/pass-007.json, generated/pass-007, analysis/pass-007-verification.json): map.validate ok, byte_identical true, map sha256 9687eb97f8ec0b554548d20c2500a9002543a94722d45459fcc2b30f19c14e02.

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Kinds unchanged: Code 37,907 (26,456 Observed, 11,451 Static), Data 93,181, Image 131,072; 1,400 regions.

## Changed regions
- 742 (bank 2 $85C6-$860F, phys $085D6-$0861F, Data, Static): evidence text now says "tail of SongPatternBases" (48 words $85B0-$860F, per-request pattern base address, consumers 2:801D/2:8022) instead of the obsolete "SongHeaders". Offset/length/kind/provenance/confidence/label (null) unchanged.

## Scan
All 1,400 region label/comment/evidence texts were searched for names retired or corrected by commentary rounds 5-6 (SongHeaders, SoundDataLow, InstrumentNoiseBytes "1329", ChampionPictureData, MetatileTileIndex, NotePeriods length, removed split addresses $83D4/$8B45/$B63D, PieceTable, story/truck/spark/palette notes). No other region contradicts current evidence (739, 743, 764, 1791, 2025, 2028-2031 checked).

## For commentary round 7
Map changed only in region 742 evidence text (pass-006 -> pass-007). Carry notes forward; the commentary name SongPatternBases now matches the map evidence. No new labels.

## Open
Unchanged from round-06-redirect.md rejection review. No leads attempted (none ready). Toolkit: no new issues (TK-001..TK-006, analysis/toolkit-issues.md).

Evidence: analysis/disasm-r07/ (plan.json, tools/apply.py, state-before-r07.json).
