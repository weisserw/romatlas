# Round 3 disassembly handoff

## Scope

- One disassembly pass. Input: checkpoint pass-002 (state = maps/current.json, sha256 1476b029...830fb,
  407 regions), restored with map.load; all classifications preserved.
- Consumed ready leads: L-016, L-004 (via L-016) and L-014. L-001/L-002 were not repeated (deferral stands).
  L-017 was left to commentary.
- Changes are map-only (split, name, provenance and one kind change). No code bytes changed kind.
  ROM, toolkit and commentary/ were not modified.
- Tools and evidence: analysis/evidence/pass-003/
  - tools/verify_plan.py, tools/apply.py;
  - verify_plan.json, apply_log.json.

## Checkpoint and verification

- **pass-003**: maps/pass-003.json = maps/current.json = analysis/state.json (sha256 d9f78600...afbd5cf, 511 regions).
- map.validate ok.
- `run.py checkpoint pass-003`: validate, generate and verify all passed (ok=true, byte_identical=true).
  Report: analysis/pass-003-verification.json.
- ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).
- Kind byte totals:

  | Kind | Bytes | Notes |
  |---|---|---|
  | Code | 13859 | Observed 11775, Static 2084 |
  | Data | 18534 | |
  | Padding | 275 | +14 |
  | Text | 116 | |
  | Image | 32768 | |

- Per-bank counts are in analysis/progress.md.

## Findings

- **Region count 407 → 511.** 54 Data regions became 158 pieces: 30 regions split, 142 pieces named.
- **Independent recheck.** verify_plan.py re-derived every named piece start from ROM bytes, without the commentary source:
  - 99 A starts: absolute/indexed operands, or `#lo`/`#hi` pairs, in the current Code regions;
  - 28 B starts: pointer words in their template (+8) or screen descriptors;
  - R tables: their immediates and pointer lists.
  - Result: 0 unverified starts and 0 label collisions.
- **Table extents.**
  - Extents are proven for R tables (index domain × record size) and for "declared" structures.
  - Other named pieces have proven starts only. Their extent runs to the next anchor or the region end,
    and each region's evidence text says so.
- **Region splits and type changes.** The main groups are below; see apply_log.json for the full list.

  | Area | Pieces |
  |---|---|
  | Sprite object templates/frames | DeckPin*, CloseBall*, LanePin*, Bowler0-3Frames, Pinsetter*, Title*, Celeb*, Perfect*, Award275*, Award250*, SignGirl* |
  | Panels | Block9FFB/BlockA045 Tiles/Attrs, Left/Lower panels, scorecard headers/row templates |
  | Hook tables | HookCurvePtrs, HookCurves ($8A90-$8B31) |
  | Ball weight | BallColorTable, BallWeightDigits, BallWeightPpuAddr ($B15C-$B172) |
  | Portrait lists | Award275PortraitLists/Pairs ($C3EE/$C402), Award250PortraitLists/Pairs ($C87A/$C88A), PlayerSignColors / SignGirlTileLists / SignGirlTiles ($CD33-$CDE6) |
  | Screen data | 7 ScreenDesc, PaletteSets, 5 screens × Attrs/Rle, LaneScreenNametable/TenthRows |
  | Sound | SoundEnvelopes $E620-$E70D, SoundHeadersAndStreams $E70E-$F191 |

- **Sound labels renamed.** SoundPtrTable_E548 → EnvelopeTable and SoundPtrTable_E5A0 → SoundHeaderTable.
- **Type change.** $F192-$F19F (14 × $FF, beyond the last parsed stream byte, before code at $F1A0) changed from Data to **Padding**.
  - The byte pattern and the absence of any header, Call or Jump target justify it.
  - $FF is also the stream End command; this is noted in the region's evidence.
- **L-014.** Regions 373, 375, 377, 393, 227, 397, 399 and 401 (144 bytes) changed from Static to Observed.
  - Every instruction start was re-decoded with prg_banks [0,1,2,3] and found in the saved natural coverage (S23-S25).
  - The evidence text was corrected to drop the earlier "no observed fetch" and "kept Static" clauses.

## Unresolved

- Unnamed generic Data remains:
  - sprite-record areas after frame tables;
  - $AD8C-$AD9B;
  - $CDE7-$CE0C: 19 (tile, $02) pairs with no reference;
  - frame sprite records at $BFF6-$C287, $C5A2-$C60A and $CE0D-$CE5D.
- Remaining L-014 residual branches (about 0.62 KB): provenance only.

## Deferred

- L-001/L-002: 1461 bytes of uncalled Static code. The round-2 redirect review stands.
- CelebrationTilesA (random) and CheckPause (no caller).
- Gameplay: no new play this pass. Review S26 found every practical milestone already visited;
  the game has no levels, bosses or credits.

## Leads

- **Consumed:** L-016 (verified), L-004 (resolved), L-014 (8 regions verified; remainder deferred).
- **Produced:** none.
- **Updated:** L-017 (ready, commentary). The round-2 source must be carried onto the 511-region pass-003 map
  by physical ranges (104 new entries) and adopt the map labels, including the two renamed sound tables.

## Toolkit

- New: TK-005. NES map.annotate needs `id: 0` for new regions, although the docs call `id` optional. This is a documentation mismatch.
- TK-001..TK-004 kept. Details: analysis/toolkit-issues.md.

## Housekeeping

- No servers left running and no scratch files outside analysis/evidence/pass-003.
- Markdown created: this file only.
