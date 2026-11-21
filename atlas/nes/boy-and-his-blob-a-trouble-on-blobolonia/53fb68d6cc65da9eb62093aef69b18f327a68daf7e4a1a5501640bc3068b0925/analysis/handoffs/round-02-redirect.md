# Round 2 — redirect handoff (COMPLETE) — recommendation: CYCLE

- Attempt directory: analysis/redirect/r02-a/ (l023-review.md; static scripts rtsrts.py, rtsrefs.py, absrefs.py,
  gapcons.py, l024_check.py, songend.py, ramrefs.py with .log outputs; cov9ab8.log; runtime exp_postending.py,
  exp_postending2.py, exp_rocket.py, exp_jackending.py; evidence/).
- Input checkpoint: pass-003 (maps/current.json == analysis/state.json, sha256 b5c79502...b7d01a) and commentary-002
  (commentary/source/game.asm, byte_identical). No map, state, source, ROM or toolkit changes. leads.json backed up
  to analysis/leads.pre-redirect-r2.json before editing.
- Classification unchanged: 262159/262160 (1 unknown: physical 0x1AC8, bank 0 $9AB8, prg_banks [0,1,14,15]).
  Proposed: that 1 byte (L-023) + a 203-byte Padding->Data kind correction (L-024, no % change).

## Exact unknown complement
| Range | Context | Neighbours | Disposition |
|---|---|---|---|
| 0x1AC8 (1 B, $60) | bank 0 $9AB8 | Observed RTS $9AB7 (end of UpdateBoyStop) / Observed LandOnFloors $9AB9 | L-023 ready: Code (orphaned RTS), Static, conf 0.6 |

## Findings
1. **L-023 reopened with newly combined constraints** (analysis/redirect/r02-a/l023-review.md). Prior rejection tested only
   for a live reference. New: (a) no consumer can address the byte - all $9A00-$9AC0 operands in bank-0/7 code are routine
   entries or BlobAnim tables indexed by a 5-state variable, no `#$9A` immediate in any code, no $9AB7/$9AB8 word, and
   0x1AC8 is never touched in 7 coverage exports while 0x1AC7/0x1AC9 have thousands of hits; (b) no fill/alignment
   convention fits ($60 value, all ROM padding is $FF bank tails, next routine at odd $9AB9); (c) the ROM has 16 other
   RTS,RTS pairs in code and in every one the second RTS is a separately referenced label (author style
   `...; rts / Done: rts`), $9AB8 being the one orphaned instance; (d) all 10 small bank-0 non-code gaps between code
   are consumer-read tables (gapcons.log). Strongest competitor: an unread 1-byte data value - not excluded logically but
   supported by nothing.
2. **L-024 verified** (l024_check.log): the 12 templates $8E4D..$9307 holding $FF runs at +$3A/+$50 are all
   ObjSetTemplates targets copied whole (LoadObjectSet `ldy #$6D` loop); TypeBounces (31 types) holds only $00/$FF and
   already-classified entries contain $FF. Kind correction Padding -> Data is consumer-backed.
3. **L-022** (AddAToZpWord, b1 $85E0): no JSR/JMP/word/immediate-pair reference (the only $85E0 word is the
   coincidental `lda #$E0 / sta $A6` at b0 $9113); body unique; never fetched in any coverage. Reachability stays
   unknown; classification (Code 0.75) unaffected. No further method (no indirect table contains it).
4. **Gameplay (artificial starts, natural continuations; scenarios.json):**
   - S11b post-ending restart **visited**: song 2's order lists all end ($00FF END, songend.log); music stops ~3180
     frames after GO=$FF and UpdateGameOverEnding_WaitMusic -> jmp ColdStart runs (title screen).
   - S09 root beer rocket **visited**: one RAM write (SelectedBean=9), then throw, board, climb through space rooms
     $67-$60, WorldFlag 0->4 at RocketExitRoom, descent to Blobolonia room $A0 and landing. 535 newly observed
     addresses, all already Static code.
   - S13 natural ending trigger **attempted**: room $CF (Emperor chamber) with SpecialRoom=$CF; blob is caged at X=$85,
     boy blocked at X=$33, jack bean never reaches the blob. Real prerequisite is the room $CE key-carry transition
     (boy -> room $FE) whose subsequent mechanics are not understood. Ending scene itself was already visited (S11).
   - All new fetches (5 + 535 + 89) fall in existing Static code; none in Data/Padding/unknown; $9AB8 and $85E0 never fetched.
5. Uncertainties review: TorchFlag $0420 has no reader in any abs/abs,x/abs,y/zp form (ramrefs.log; indexed $040A
   accesses use 11-slot indices); UnwrittenFlag92 has only `ora $92` (b0 $83C6) and no zp store. Both remain
   mechanical-only notes (no unknown bytes).

## Ready leads (analysis/leads.json)
| Lead | Recipe | Bytes |
|---|---|---|
| L-023 | annotate 0x1AC8 len 1 Code/Static conf 0.6, label e.g. UpdateBoyStop_OrphanRts, comment per l023-review.md; no reachability claim | 1 |
| L-024 | annotate 25 regions Padding->Data/Static: b6 $A0B3-$A0BD; template+$3A and +$50 (8 B each) for the 12 templates listed in leads.json | 0 (203 re-kinded) |
Disassembly reviewer decides L-023 independently. If rejected, keep unknown: l023-review.md then serves as the
terminal rejection record and the next redirect should report TERMINAL (incomplete, 1 byte) without new experiments.

## Rejected / deferred (with reasons)
- L-022, L-008 reachability (AddAToZpWord, restart vector heads, debug printer, warp routine): no static reference, no
  indirect dispatch table contains them, not fetched in the 7 saved coverage exports; nothing left to test statically; no unknown bytes.
- L-006 (duplicate bank tails), L-009 ($8312/$8313 overlap): verification-only, no unknown bytes; unchanged.
- S13 jack ending trigger via the room $CE key transition: requires understanding the boy-in-room-$FE / caged-blob
  mechanic; it affects only provenance of already-classified Static code, so prospects for classification are nil.
- SetScriptGrid/SetScriptHazard scenes, edge markers, pharmacy offer tables, BlobEat/FormFlag74 semantics: commentary
  meaning only, no unknown bytes; observation would only change Static->Observed provenance.
- Music/stream per-entry decoding, RectTileData byte ownership: documentation granularity; all bytes already classified.

## Compact rejection review (for the residual, if L-023 is rejected)
- Candidate: physical 0x1AC8 (b0 $9AB8, $60) = Code, orphaned RTS.
- Decisive evidence: l023-review.md; rtsrefs.log (16 RTS,RTS pairs, all second RTS referenced except $9AB8);
  absrefs.log / gapcons.log (no consumer); cov9ab8.log (never fetched or read; neighbours hit thousands of times).
- What prior rejection ruled out: only live references. Independent constraints considered together: consumer
  exclusion, fill/alignment conventions, enclosure by Observed code, author RTS,RTS style.
- Strongest competing interpretation: unread data/filler byte equal to $60.
- Remaining reason if insufficient: no evidence source can observe intent for a byte that is neither referenced nor
  read; a style precedent is the only discriminator, and no runtime or static test can add more.

## Recommendation
CYCLE: disassembly applies/reviews L-023 (1 byte -> 262160/262160 if accepted) and L-024 (kind correction), then
commentary updates the $9AB8 and template/TypeBounces notes plus the new observed paths (rocket flight, post-ending
restart), then final redirect/terminal or archive audit. Disk: ~3.5 GB free; scratch removed.
