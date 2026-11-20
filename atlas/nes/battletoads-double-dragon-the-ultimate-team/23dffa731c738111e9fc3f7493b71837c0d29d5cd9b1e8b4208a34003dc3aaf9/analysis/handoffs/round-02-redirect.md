# Round 2 redirect handoff (attempt r02-a1): READY-FOR-DISASSEMBLY

- Input checkpoint: pass-002 (maps/current.json == maps/pass-002.json == analysis/state.json,
  sha256 9c36a833...; unchanged by this phase). 260,469 / 262,160 classified (99.35%),
  1,691 unknown bytes in 99 runs (analysis/redirect/r02-a1/unknown_runs_pass002.txt).
- Research only. ROM, map, state and source are unchanged. Attempt dir: analysis/redirect/r02-a1/
  (notes.md = evidence summary; proposals.json = the batch; build_proposals.py rebuilds it and
  checks it against the map: 0 conflicts, 0 internal overlaps).
- Proposed: 1,677 of the 1,691 bytes. Residual if all accepted: 14 bytes (L-027). If the
  tier-B items are rejected: 14 + 70 (L-026 B) + 14 (L-025 B) + 9 (L-023 B) = 107 bytes.

## Ready leads (analysis/leads.json; ranges, rationale and tier per range in proposals.json)
| Lead | Content | Bytes |
|---|---|---|
| L-023 | Bank 6 StepBgObjScript motion scripts, 15 ranges (tier A 291); $EFA6-$EFAE same format, no pointer (tier B, generic) | 300 |
| L-024 | Consumer tables with proven extents: b6 $8042-$809C per-ObjType draw-hook table (Y = type $01-$5B); b6 $EEE0-$EF1A ASCII-to-tile ($20-$5A); b6 $9746-$974F stage tables (Y $07-$0B); b0 $A2DC-$A2E7 (Y $40-$43); b0 $E86A-$E872 (search Y $1F-$23); b0 $84F0-$854F (24 x 4-byte records); b4 $B933-$B946 (Y $70-$83); b4 $B97A-$B986 RAM-routine template; b2 $FC09-$FC0A per-player target X | 312 |
| L-025 | Generic data: consumer-indexed tables with unproven domains (b0 $8253, $8389, $9635, $9C80, $AA05, $CCF3, $E34C, $E522-$E535; b3 $8996; b4 $B462), b2 $FC94-$FD33 bit patterns, b5 $BB49-$BC46/$BC67-$BCD6/$C90F-$C916 stage-map bytes, $FF fills (tier A 654); single-byte / short non-code bytes (tier B 14) | 668 |
| L-026 | Uncalled static code fragments: tier A 85 (WaitVBlank b0 $D260/b4 $C5BE, delay loop b5 $831E, camera step b4 $B8BE-$B8D0, b0 $AB91, $F097, b5 $80BB-$80C2, b5 $FF50-$FF6B); tier B 70 (one/two-instruction prefixes and lone jmp stubs, reviewer call) | 155 |
| L-006 | Bank-switch module copies $FF6E-$FFA5 in all banks (byte-identical to the bank-0 template except the per-bank jsr/jmp operands) | 236 |
| L-004 | Jump-table slots b3 $8024-$8026, b6 $800C-$800E (jmp to invalid targets: annotate without following, or Data) | 6 |

Recipe (one disassembly pass): run `python3 analysis/redirect/r02-a1/build_proposals.py`
(prints conflicts and residual), `motion.py` (L-023), `codecheck.py` (L-026); review each
range's `why` in proposals.json against the cited consumer instructions; annotate via
map.annotate with Static provenance. Code fragments: do not follow jsr/jmp operands that point
at fill (L-006 banks 1,2,4,5,6,7; L-004). Independent review may reject tier B items.

## Corrected premises / new inferences (evidence-policy review)
- L-021 "$B8BD-$B8D0 overlaps the table read by ldx $B8C3,y": false. The loop is ldy #$70 ...
  cpy #$84, so the read covers only $B933-$B946 (the separately unknown 20-byte run).
- Pass-2 missed base-1 tables: the indexed read's base is the last byte of the preceding
  instruction, so the gap matcher attributed it to code. Index bounds now proven where listed.
- L-020 "no reader found" for $8042-$809C: DrawAllSprites lda $8041,y with Y = ObjType.
  Consequence for commentary: DrawFrameAndClip ($80CE) is selected by object type $4F.
- L-006: earlier blocked on "caller-bank/target evidence"; byte identity with the executed
  template plus the module contracts (which store maps which bank) establishes the role.

## Residual 14 bytes (L-027, blocked; rejection review)
| Range | Claim considered | Decisive evidence | Why it stays unknown |
|---|---|---|---|
| b0 $8D92, b5 $ACFA, b4 $B979, b6 $9745, b6 $EFE5 | dead rts vs pad/table byte | prevnext_pass002.txt (each follows jmp/always-taken branch); refs_pass002.txt (no reference); $9745 is index 6 of a table read only for Y >= 7 | A lone unreferenced $60 is equally a dead rts or an unread data byte. Policy: an unreferenced return opcode lacks structure on its own; no data consumer either. |
| b6 $FE24-$FE2C | code fragment (jsr $FE19 / stx OAMADDR / lda #$02 / $8D) | codecheck.py: the last opcode's operand overlaps the classified ldx #$00 at $FE2D | Either an overlapping-entry trick (sta $00A2 when entered at $FE2C) or a patched leftover; no caller; bounds and role ambiguous. |
Strongest competing interpretation for all: generic data. Not proposed because nothing
discriminates it from code. Reopen only with a new reference.

## Gameplay (analysis/scenarios.json updated)
- Post-ending exit visited without RAM edits from traces/r01a1_snap_ending.json: after the
  ending music stops ($0301 = 0, ~900 frames) a Start press fades out and StageId becomes $00
  (Licensed by Nintendo, then title). There is no separate credits roll on this path.
- 2 PLAYER GAME A visited (both pads, StageId $03): traces/r02a1_snap_2p_gameA.json; 240 traced
  frames, 3,277 ROM opcode addresses, none in unknown runs. Game B not separately visited
  (same select path, $16 = 2). L-009 verified.

## Deferred avenues
- Exact index domains for the L-025 per-stage tables (StageId) and the $E4EB/$E4F5 frame maps:
  not needed for generic data. A specific-layout claim needs a StageId/frame domain proof.
- Semantics of L-024/L-025 tables (sound requests at $84F0, the $0682 values): commentary work.
- Servers: the r02-a1 daemon (socket /tmp/r02a1.sock) was shut down.
- Toolkit: no new toolkit issues found in this phase.
