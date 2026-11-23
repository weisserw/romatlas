# Round 2 disassembly handoff

Status: pass finished. Every byte is now classified, but the ROM is still incomplete: the new current source has not had commentary or editorial review.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

## Scope
- One disassembly pass over the round-1 checkpoint. I re-checked each ready redirect lead (L-001..L-006) myself before applying it.
- All edits came from one computed plan (analysis/round2-tools/plan.py). It replaced 126 regions with 585 and asserts tiling and unique labels. apply.py applied it through map.remove/map.annotate on `run.py serve`.
- No gameplay this pass. r1-a1 had already visited every milestone, including the ending, so the only thing left to gain from more play is provenance.

## Checkpoint and verification
- Input: maps/round1-disasm-v2.json (sha256 324985b8...).
- Output: maps/round2-disasm.json = maps/current.json = analysis/state.json (sha256 6b48e02b2ea873679e2fe7efab7b0f3dafe6e04981839bc37ae4cba1ff0ce055); 928 regions, 93 labels.
- `python3 run.py checkpoint round2-disasm` gave ok=true, byte_identical=true (analysis/round2-disasm-verification.json). Output is in generated/round2-disasm; diagnostics are empty. map.validate ok.
- input/game.nes unchanged (sha256 153068655b...).

## Findings / lead outcomes (analysis/leads.json)
- L-003 verified: 0x18010 (bank 6 $8000, $EA) is Data, the bank leader byte (Static 0.8). I rescanned 11,895 classified instructions: none has operand $8000 or a branch to it, and the only indirect jump is the bounded bank-0 one. A NOP prologue is the competing reading; it is recorded in the region evidence.
- L-006 verified: 25 bytes move from Padding to Data. They are the FloorBlockAttrs entries for blocks 13-17 (all 28 block ids are used) and the $FF end markers of screens 5/10/25/26/29.
- L-002 verified: the generic data in banks 0-5 is now 63 labeled, consumer-decoded records (SongData0-3, SfxScript04-22, Font1bppTiles, Floor*, SpriteSetData, ScreenData00-29) at 0.95. The bank 1-5 leader bytes are split out at 0.9. I re-decoded all 30 screens with a decoder checked against ScreenDecode/DecodeNibbleRuns, and the records leave no gaps.
- L-001 verified: 11,749 code bytes upgraded from Static to Observed, matched by context (0 mismatches). Observed code is now 19,753 bytes and Static 6,535 (322 regions).
- L-004 verified: the 0.7 fragments are split into two groups. Reached parts become Observed, or Static 0.9 with their callers listed (7 callers spot-checked). The 675-byte residue becomes Static 0.8, "statically unreferenced, no known entry"; it is not called dead.
- L-005 partly applied: tables split where the consumer sets the bounds:
  - HotspotX/W/Y/H
  - AttrRowAddr0/1
  - PokerPayTable/PokerCoinValue
  - FourReelPayTable (54 bytes), FourReelColumns, FourReelStripPtrs

  This corrects the commentary's 66-byte FourReelPayTable equate. Its +54..+59 references are really FourReelColumns/FourReelStripPtrs.

## Unresolved
- No unknown bytes.
- Weakest claims: 0x18010 (one byte, classified by layout analogy) and the 675-byte unreferenced code residue (no entry known).
- 6,535 Static code bytes were never fetched (provenance only). Largest: 7:$D1D6 (410), 6:$9E1E (246), 7:$E296 (185), 7:$F213 (164), 7:$CE61 (149), 7:$E49F (147).
- Generic data still without a specific layout:
  - the rest of 6:$B0EB-$BAD8
  - metasprite definitions 7:$C049-$C286 and $D084-$D12B
  - 7:$F3DE-$F8F7

## Deferred (with reasons)
- Commentary/editorial review of the new source is the next phase. It should replace the commentary equates with the new map labels and fix the FourReelPayTable size. Region count went from 469 to 928 because of the Observed/Static splits.
- More traced gameplay to raise Observed coverage: provenance only, and tracing is slow (TK-002).
- Remaining L-005 table layouts: generic data is enough for classification, and each layout needs per-writer frame-size proof.

## Links
- Progress: analysis/progress.md ("Round 2 disassembly"); ledger: analysis/investigation.md; scenarios: analysis/scenarios.json (notes_r2_disasm).
- Tools: analysis/round2-tools/ (plan.py, apply.py, verify_leads.py, l001.py, l003.py). Evidence: analysis/redirect/r1-a1/.
- Toolkit: analysis/toolkit-issues.md (TK-001 confirmed bug, TK-002/TK-003 missing features, TK-004 suspected doc gap). No new toolkit issues were found this pass.
