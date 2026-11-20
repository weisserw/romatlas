# Round 1 redirect handoff (attempt r01-a1) — READY-FOR-DISASSEMBLY

- Scope: redirect after COMPLETE round-1 commentary. Static analysis of dispatch tables,
  decompressor/consumer formats and gap ledger, plus bounded runtime checks (RAM
  routine origin, stage progression). Research only; map/state/source unchanged
  (analysis/state.json untouched; ROM untouched).
- Input checkpoint: pass-001 (18.50%, 48,496 / 262,160 classified; 213,664 unknown).
- Research dir: analysis/redirect/r01-a1/ (tools + proposed_data.json, code_proposals.json,
  b0_residual_gaps_after_L010.txt). Ledger: `python3 analysis/redirect/r01-a1/ledger.py 40`.

## Ready leads for the next disassembly pass (proposed, not accepted)
| Lead | Content | Est. bytes |
|---|---|---|
| L-010 | Bank 0 ObjTypeHandlers $9A71-$9B28 (92 words) + 62 handler roots; jsr $DFCF returns; then bank 0 table residuals | ~10.3 KB code + ~2.3 KB data |
| L-014 | Bank 6 BG handler table $9C4E (41 words) + roots; same-bank bank-select continuations (bank 2 $F3EB, bank 6 x7) | ~2.5 KB code |
| L-012 | CHR set tables/headers/streams banks 1, 7, 4 (+ bank 1 prop tables, bank 7 portraits) | ~81 KB data |
| L-013 | Bank 2 player frame tiles $8030-$EECF + pointer table $FE07-$FF68 | ~28.7 KB data |
| L-011 | Bank 7 OAM $804F-$805E, RAM-routine image $805F-$80DC ($0782, runtime-verified) | 142 B |
| L-015 | Bank 3 data blocks (11 ranges) | ~29 KB data |
| L-016 | Bank 6 metasprites $A73A-$E45D and text/menu tables | ~17 KB data |
| L-017 | Bank 5 maps/metatiles (5 ranges) | ~24 KB data |
| L-018 | Bank 4 screen descriptors / tables | ~9.7 KB data |
| L-019 | Small caller-less code fragments (bank 3 $89A9-$89AA computed-jump target is strong; others reviewer's call) | ~0.4 KB |
Total proposed ≈ 12.9 KB code + ≈ 190 KB data; ledger residual after batch ≈ 11.2 KB
(mostly small consumer-referenced tables; disassembly can absorb many with the same method).
Each data claim has range, consumer and rationale in proposed_data.json; overlap check
against current map and proposed code: none.

## Other findings
- L-003 (RAM code): $0782 resolved (L-011); $0600/$068C/$0532/$0720/$0728 are runtime-built buffers.
- L-007 (progression): $DA=$FF is the native stage-end trigger. Order $03→$05→$02→$04→$08→$09→$06→$0B (ending "ULTIMATE TEAM / TOTAL TIME"). Commentary's "$08 = title/attract" is wrong ($08 = level 5): commentary should correct it next cycle. Route: analysis/redirect/r01-a1/route_ending.py; scenarios.json updated.
- L-001/L-002/L-005 superseded by the leads above.

## Deferred avenues (reasons)
- Game over / continue / 2P (L-009): not run (time); contract known (PlayerLives,x negative → LoadStage5 $D21B).
- Credits after the ending screen: Start + 2400 frames on the ending ($0B) changed nothing (loader cmd 4 waits); next try: inspect bank 6 loader command 4 frame handler for its exit condition. Ending snapshot traces/r01a1_snap_ending.json.
- Per-record format proofs (music scripts, anim scripts, metasprites, map dimensions): not needed for generic-data classification; would need the 84 sound / 31 anim command lengths. Diminishing return for classification.
- Broad runtime data-read coverage: trace mode ~4 fps (measured), low value vs static consumers.
- L-004 (invalid slots/IRQ), L-006 (stub tails), L-008 (small gaps): unchanged; small, handled in disassembly review.
