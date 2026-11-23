# Disassembly progress

Workspace initialized. Baseline and classification had not run before round 1.

## Coordinator log (goal.md orchestration)

| Round | Phase | Status | Input checkpoint | Output checkpoint | Verification | Leads consumed/produced | Handoff |
|---|---|---|---|---|---|---|---|
| 1 | disassembly | done, incomplete (2026-11-22) | none (fresh) | maps/round1-disasm-v2.json | byte-identical OK | 0 / 5 (L-001..L-005) | analysis/handoffs/round-1-disassembly.md |
| 1 | commentary | done, complete for current source (2026-11-22) | maps/round1-disasm-v2.json | commentary/source/game.asm (reading copy commentary/reading-after.asm) | byte-identical OK (commentary/verification.json) | 0 / questions on L-001..L-005 | analysis/handoffs/round-1-commentary.md |
| 1 | redirect | done, ready leads (2026-11-22) | maps/round1-disasm-v2.json | (unchanged; research only) | n/a (no map change) | updated L-001..L-005 / added L-006; ready L-001..L-006 | analysis/handoffs/round-1-redirect.md |
| 2 | disassembly | done; all bytes classified, ROM incomplete pending commentary/editorial gates (2026-11-22) | maps/round1-disasm-v2.json | maps/round2-disasm.json | byte-identical OK (analysis/round2-disasm-verification.json) | 6 / 0 (L-001..L-006 verified; L-005 partial subset) | analysis/handoffs/round-2-disassembly.md |
| 2 | commentary | done, complete for current source (2026-11-22) | maps/round2-disasm.json | commentary/source/game.asm (reading copy commentary/reading-after.asm; round-1 commentary preserved at analysis/round-1/commentary/) | byte-identical OK (commentary/verification.json) | 0 / 4 questions (songs $81/$82, $F3DE-$F8F7, screen $02/$10-$15, slot symbols) | analysis/handoffs/round-2-commentary.md |
| 2 | redirect | done, ready leads (2026-11-22) | maps/round2-disasm.json | (unchanged; research only) | n/a (no map change) | updated L-005 / added L-007, L-008, L-009; ready L-007..L-009 | analysis/handoffs/round-2-redirect.md |
| 2 | redirect | done, ready leads (r2-a2 continuation, 2026-11-22) | maps/round2-disasm.json | (unchanged; research only) | n/a (no map change) | updated L-007, L-008, L-009 / added L-010; ready L-007..L-010 | analysis/handoffs/round-2-redirect-a2.md |
| 3 | disassembly | done; all bytes classified, ROM incomplete pending commentary/editorial gates (2026-11-22) | maps/round2-disasm.json | maps/round3-disasm.json | byte-identical OK (analysis/round3-disasm-verification.json) | 3 / 0 (L-007, L-008, L-010 verified; L-009 passed to commentary) | analysis/handoffs/round-3-disassembly.md |
| 3 | commentary | done, complete for current source (2026-11-22) | maps/round3-disasm.json | commentary/source/game.asm (reading copy commentary/reading-after.asm; round-2 commentary preserved at analysis/round-2/commentary/) | byte-identical OK (commentary/verification.json) | L-009 incorporated / 3 optional questions (clock temperature, $C0D3 figure + slot symbols, sfx $0D/$10) | analysis/handoffs/round-3-commentary.md |
| 3 | redirect | done, ready lead (commentary only) + structural rejection review (2026-11-22) | maps/round3-disasm.json | (unchanged; research only) | n/a (no map change) | updated L-009 (verified) / added L-011; ready L-011 | analysis/handoffs/round-3-redirect.md |
| 4 | disassembly | pass-through by coordinator decision (2026-11-22): only ready lead L-011 is commentary-only; no structural proposal; map unchanged | maps/round3-disasm.json | maps/round3-disasm.json (sha 24099db1…) | reuses round-3 verification | L-011 forwarded to commentary | this log |
| 4 | commentary | done, complete for current source (2026-11-22); resumed round-3 commentary (map unchanged) | maps/round3-disasm.json | commentary/source/game.asm (reading copy commentary/reading-after.asm) | byte-identical OK (commentary/verification.json) | L-011 incorporated / 0 new; editorial_audit pending | analysis/handoffs/round-4-commentary.md |
| 4 | redirect | not relaunched by coordinator decision (2026-11-22): round-3 rejection review is sound on unchanged premises; round-4 commentary changed no structure and produced no new evidence | maps/round3-disasm.json | unchanged | n/a | L-011 verified (incorporated) | analysis/handoffs/round-3-redirect.md |
| final | acceptance | done, PASSED (2026-11-22): editorial audit recorded; 4 labels + 1 comment + review-summary clipping fixed | maps/round3-disasm.json | maps/round3-disasm.json (unchanged, sha 24099db1…); commentary/source/game.asm (sha afbfa4bc…) | commentary.py audit ok + verify byte-identical (commentary/audit.json, commentary/verification.json); map/store validate ok | none | analysis/handoffs/final-acceptance.md |

Current: TERMINAL HANDOFF reached (2026-11-22): analysis/handoffs/terminal.md. ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). Final acceptance passed (commentary/audit.json ok+byte_identical+editorial_audit; commentary/verification.json byte-identical). archive-options.json written: 17 final documents selected (all phase handoffs, terminal handoff, prior-round commentary progress/uncertainties); 6 omitted (round-1/round-2 copies of commentary AGENTS/prompt/workflow templates, byte-identical to retained commentary/ copies). No archive execution or cleanup performed (not authorized).

## Round 1 disassembly

ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).

(Rounded; NOT complete: 1 byte unknown at file 0x18010. Exact 99.9992%.)

Baseline: maps/baseline.json + generated/baseline, verified byte-identical (raw export only, not classification).
Checkpoint: maps/round1-disasm-v2.json (map sha256 324985b8e9b5df8fb4833fc4a4e3f8a5470ebce62c73a226e619940bb27cdcdb),
generated/round1-disasm-v2, report analysis/round1-disasm-v2-verification.json: ok=true, byte_identical=true.
Internal store: analysis/state.json (469 regions); export maps/current.json; map.validate ok.

Per 8K PRG bank (bytes; header 16 bytes Data):

| 8K bank | CPU | Code | Data | Padding | Unknown |
|---|---|---|---|---|---|
| 0 | 16K bank 0 $8000 | 919 | 7216 | 57 | 0 |
| 1 | 16K bank 0 $A000 | 65 | 8002 | 125 | 0 |
| 2 | 16K bank 1 $8000 | 0 | 8192 | 0 | 0 |
| 3 | 16K bank 1 $A000 | 65 | 7923 | 204 | 0 |
| 4 | 16K bank 2 $8000 | 0 | 8192 | 0 | 0 |
| 5 | 16K bank 2 $A000 | 65 | 7933 | 194 | 0 |
| 6 | 16K bank 3 $8000 | 0 | 8192 | 0 | 0 |
| 7 | 16K bank 3 $A000 | 65 | 7711 | 416 | 0 |
| 8 | 16K bank 4 $8000 | 0 | 8192 | 0 | 0 |
| 9 | 16K bank 4 $A000 | 65 | 7521 | 606 | 0 |
| 10 | 16K bank 5 $8000 | 0 | 6674 | 1518 | 0 |
| 11 | 16K bank 5 $A000 | 65 | 22 | 8105 | 0 |
| 12 | 16K bank 6 $8000 | 7866 | 325 | 0 | 1 |
| 13 | 16K bank 6 $A000 | 5434 | 2713 | 45 | 0 |
| 14 | fixed $C000 | 5665 | 2399 | 128 | 0 |
| 15 | fixed $E000 | 6014 | 1801 | 377 | 0 |

Code total 26,313 bytes (Observed provenance 8,004; rest Static incl. 390 bytes of template copies and
unreferenced fragments at confidence 0.7). Execution coverage (separate metric): 5,377 unique observed
instruction contexts in scenarios S1-S6 (startup, title, taxi, cashier, casino floor, $25 slot, blackjack table).
Annotation coverage: every region carries evidence text; labels only on key routines (Reset, NmiHandler,
SelectPrgBank, ReadJoypad, WaitNextFrame, SelectGameBank, DispatchGameScreen, Sound_Entry0-4, ...).
Commentary/readability review not performed (separate phase).

Leads produced: L-001 (game dispatch RAM contract / visit all games + ending), L-002 (data bank formats),
L-003 (last unknown byte), L-004 (verify unreferenced fragments), L-005 (split large data tables). Consumed: none.
Toolkit: 4 findings (TK-001 confirmed bug: emulator.input button/pressed silently ignored; TK-002 tracing
slowdown; TK-003 NES discovery stops at JSR / undocumented max_instructions; TK-004 paging termination).
See analysis/toolkit-issues.md. Handoff: analysis/handoffs/round-1-disassembly.md.

## Round 1 redirect (attempt r1-a1)

Research only (analysis/redirect/r1-a1/); map/state/source unchanged. Classification still 131087/131088.
- Milestones: every game type (video poker, blackjack, roulette, Big Six, slot kinds 9/10/11 + $500 machine),
  clock, restroom, pay-table info screens, cash status, quit dialog, EndingTally with all six ratings and the
  broke ending visited via the DispatchGameScreen ($C438, Y=hotspot) register contract plus RAM chip patches;
  betting/coin contracts documented (analysis/scenarios.json S5-S9).
- L-003: combined static evidence for the last unknown byte (bank-6 leader byte, no possible entry) -> ready.
- L-002/L-006: consumer-matching decoders give exact extents for 63 records (85,306 bytes) tiling all content
  of data banks 0-5; 25 bytes currently Padding are consumed data (floor attributes, screen end markers).
- L-004: 967 of 1642 "unreferenced fragment" bytes are reachable from established callers; 675-byte residue
  has no possible entry (indirect transfers enumerated).
- L-001: bounded traced replay -> analysis/redirect/r1-a1/obs-r1a1.json.gz for Observed upgrades.
Handoff: analysis/handoffs/round-1-redirect.md.

## Round 2 disassembly

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

(Exact: every physical byte has an evidence-backed structural role in the internal map. Workflow completion is still
NOT claimed: the new current source (928 regions, new labels/splits) has not had commentary/editorial review.)

Input: maps/round1-disasm-v2.json (sha256 324985b8...). Checkpoint: maps/round2-disasm.json (map sha256
6b48e02b2ea873679e2fe7efab7b0f3dafe6e04981839bc37ae4cba1ff0ce055), generated/round2-disasm, report
analysis/round2-disasm-verification.json: ok=true, byte_identical=true; generator diagnostics empty.
Internal store analysis/state.json = maps/current.json (same sha256); map.validate ok; 928 regions, 93 labels.

Per 8K PRG bank (bytes; 16-byte iNES header is Data and not in the table):

| 8K bank | CPU | Code | Data | Padding | Unknown |
|---|---|---|---|---|---|
| 0 | 16K bank 0 $8000 | 919 | 7216 | 57 | 0 |
| 1 | 16K bank 0 $A000 | 65 | 8022 | 105 | 0 |
| 2 | 16K bank 1 $8000 | 0 | 8192 | 0 | 0 |
| 3 | 16K bank 1 $A000 | 65 | 7924 | 203 | 0 |
| 4 | 16K bank 2 $8000 | 0 | 8192 | 0 | 0 |
| 5 | 16K bank 2 $A000 | 65 | 7934 | 193 | 0 |
| 6 | 16K bank 3 $8000 | 0 | 8192 | 0 | 0 |
| 7 | 16K bank 3 $A000 | 65 | 7712 | 415 | 0 |
| 8 | 16K bank 4 $8000 | 0 | 8192 | 0 | 0 |
| 9 | 16K bank 4 $A000 | 65 | 7522 | 605 | 0 |
| 10 | 16K bank 5 $8000 | 0 | 6675 | 1517 | 0 |
| 11 | 16K bank 5 $A000 | 65 | 22 | 8105 | 0 |
| 12 | 16K bank 6 $8000 | 7866 | 326 | 0 | 0 |
| 13 | 16K bank 6 $A000 | 5434 | 2713 | 45 | 0 |
| 14 | fixed $C000 | 5665 | 2399 | 128 | 0 |
| 15 | fixed $E000 | 6014 | 1801 | 377 | 0 |

Totals: Code 26,288 (Observed 19,753; Static 6,535), Data 93,050 (incl. header), Padding 11,750, Unknown 0.
Changes vs round 1: +1 Data (L-003, 0x18010), 25 Padding -> Data (L-006), banks 0-5 generic data split into
63 labeled consumer-decoded records + 5 leader bytes (L-002), Static->Observed for 11,749 code bytes (L-001),
fragment re-grading (L-004: 797 reached bytes Observed/0.9, 675 residue 0.8 "statically unreferenced"),
consumer-bounded table splits (L-005 subset: Hotspot*, AttrRowAddr0/1, PokerPayTable/PokerCoinValue,
FourReelPayTable/FourReelColumns/FourReelStripPtrs). Execution coverage (separate metric): observed contexts from
round-1 S1-S6 + r1-a1 traced replay; no new gameplay run this pass (all milestones already visited, see scenarios.json).
Lead outcomes: L-001..L-006 verified (L-005 partial subset). Toolkit: no new findings this pass; existing TK-001..TK-004
in analysis/toolkit-issues.md. Tools: analysis/round2-tools/. Handoff: analysis/handoffs/round-2-disassembly.md.

## Round 2 redirect (attempt r2-a1)

Static-only redirect on maps/round2-disasm.json (sha256 6b48e02b...). New immediate-pair pointer scan plus consumer index-domain analysis:
- L-008 (high): 7:$CFCF-$D024 (86 bytes, currently Data in the MetaspriteTable region) decodes as two coherent unreferenced text routines; MetaspriteTable is 13 pointers ($CFB5-$CFCE); $D084-$D12B typed.
- L-007 (medium): 76 consumer-bounded records tiling 6:$B0EB-$BAC7, 7:$D6FD-$D9CA, 7:$F378-$F8F7 and 7:$DCF0-$DD7F (analysis/redirect/r2-a1/splits.json); answers question 2 ($F418-$F8F7 = CardFaceTiles 52x24) and fixes 8 equate sizes.
- L-009 (medium): commentary facts (songs $81/$82 via RebuildFloorWithMusic, sfx $05-$09 coin sounds, screens $10-$15 entrance pictures, DEC8 = mod 10).
Deferred: metasprite frame extents (per-writer SprDef/SprFrame stride mapping), screen $02 capture, no-consumer spans (582 bytes). Handoff: analysis/handoffs/round-2-redirect.md.

## Round 2 redirect (attempt r2-a2, continuation)

Checkpoint unchanged (maps/round2-disasm.json, 100.00%). Static plus bounded runtime (read-watch over the slot routes and startup, artificial screen decode):
- L-010 (new): metasprite definitions 7:$C049-$C286 split into 13 consumer-bounded SprDef records and a 436-byte unreferenced 14-frame block (analysis/redirect/r2-a2/metasprite-splits.json).
- L-007 amended: win texts via WinTextPtrs ($B5B4/$B678/$B754), FourReelCoinTiles 24, FourReelDenomText 12, 7:$D777-$D8BC = two 64-byte 4-reel stop tables + three 66-byte kind-10 stop tables. No-consumer bytes in scope 582 -> 88.
- L-008 sanity-checked (no indirect/RAM path). L-009 adds screen $02 = developer placeholder text, $E195 inference, metasprite format facts.
- Negative: 6:$85CE never read in 4 slot machines; $E195 only RTS dummy read. TK-005 recorded. Handoff: analysis/handoffs/round-2-redirect-a2.md.

## Round 3 disassembly

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

(Every physical byte has an evidence-backed structural role in the internal map. Workflow completion is still NOT
claimed: the new current source (1021 regions, 92 new labels, 86 bytes reclassified as code) needs commentary/editorial review.)

Input: maps/round2-disasm.json (sha256 6b48e02b...). Checkpoint: maps/round3-disasm.json (map sha256
24099db1a8311da17f7abf3fe5598c5af5380e2c4c0e9fb1cbb6c13210afbed3), generated/round3-disasm, report
analysis/round3-disasm-verification.json: ok=true, byte_identical=true; generator diagnostics empty. Internal store
analysis/state.json = maps/current.json (same sha256); map.validate ok; 1021 regions (7 removed, 100 added), 185 labels.
input/game.nes unchanged (sha256 15306865...).

Per 8K PRG bank (bytes; 16-byte iNES header is Data and not in the table):

| 8K bank | CPU | Code | Data | Padding | Unknown |
|---|---|---|---|---|---|
| 0 | 16K bank 0 $8000 | 919 | 7216 | 57 | 0 |
| 1 | 16K bank 0 $A000 | 65 | 8022 | 105 | 0 |
| 2 | 16K bank 1 $8000 | 0 | 8192 | 0 | 0 |
| 3 | 16K bank 1 $A000 | 65 | 7924 | 203 | 0 |
| 4 | 16K bank 2 $8000 | 0 | 8192 | 0 | 0 |
| 5 | 16K bank 2 $A000 | 65 | 7934 | 193 | 0 |
| 6 | 16K bank 3 $8000 | 0 | 8192 | 0 | 0 |
| 7 | 16K bank 3 $A000 | 65 | 7712 | 415 | 0 |
| 8 | 16K bank 4 $8000 | 0 | 8192 | 0 | 0 |
| 9 | 16K bank 4 $A000 | 65 | 7522 | 605 | 0 |
| 10 | 16K bank 5 $8000 | 0 | 6675 | 1517 | 0 |
| 11 | 16K bank 5 $A000 | 65 | 22 | 8105 | 0 |
| 12 | 16K bank 6 $8000 | 7866 | 326 | 0 | 0 |
| 13 | 16K bank 6 $A000 | 5434 | 2713 | 45 | 0 |
| 14 | fixed $C000 | 5751 | 2313 | 128 | 0 |
| 15 | fixed $E000 | 6014 | 1801 | 377 | 0 |

Totals: Code 26,374 (Observed 19,753; Static 6,621), Data 92,964 (incl. header), Padding 11,750, Unknown 0.
Changes vs round 2 (8K banks 13, 14 and 15 only):
- L-008: 7:$CFCF-$D024 (phys 0x1CFDF-0x1D034, 86 bytes) Data -> Code, Static 0.8, statically unreferenced (not dead).
  MetaspriteTable is now $CFB5-$CFCE (13 words). $D084-$D12B is split into CharToTileTail, TextCursorColBase/RowBase/Width
  and two generic no-consumer spans.
- L-007: 77 consumer-bounded records (74 labeled) in 6:$B0EB-$BAC7, 7:$D6FD-$D9CA, 7:$F378-$F8F7 and 7:$DCF0-$DD7F,
  with the r2-a2 amendments (win texts, FourReelCoinTiles 24, the five reel stop tables).
- L-010: 13 metasprite records (MsprCursor..MsprBounceBall) plus a generic 436-byte 14-frame block at 7:$C0D3-$C286.
Verification: consumers were re-checked independently by decoding (sample list in analysis/leads.json), the redirect split
generators were rerun deterministically with their asserts passing, and the plan asserts exact re-tiling of the 7 parents
and unique labels (analysis/round3-tools/plan.py, apply.py).
No gameplay this pass: every milestone was already visited (scenarios.json), so more play would only add provenance.
Lead outcomes: L-007, L-008 and L-010 verified; L-009 passed through to commentary. Toolkit: no new findings this pass;
TK-001..TK-005 are unchanged in analysis/toolkit-issues.md. Handoff: analysis/handoffs/round-3-disassembly.md.

## Round 3 redirect (r3-a1)

Input checkpoint: maps/round3-disasm.json (sha256 24099db1...), 100.00% classified, unchanged; research records only.
- Clock alternate phase captured: shows the temperature ('85' + degree sign at 8:00 = ClockHourTable[48]).
- Slot symbol ids mapped to pictures for all three machine kinds and checked on screen. Pay-line offsets 0/2/4 are the top/middle/bottom rows.
- Sound ids $0D and $10: no caller among all 54 call sites (statically unreferenced).
- 7:$C0D3 block renders as a knobbed rod swinging and back, plus a mirrored copy: possibly an unused sprite slot-handle animation (inference).
- New split hi/lo pointer-table scan over the residual spans: negative.
Leads: L-011 ready (commentary only, 0 bytes); L-009 marked verified. Structural residuals reviewed (rejection review in the handoff); none is actionable.
Evidence: analysis/redirect/r3-a1/ (findings.json, splitscan.json, shots/, tools/). Scenarios: S11 added; no unvisited milestones.
