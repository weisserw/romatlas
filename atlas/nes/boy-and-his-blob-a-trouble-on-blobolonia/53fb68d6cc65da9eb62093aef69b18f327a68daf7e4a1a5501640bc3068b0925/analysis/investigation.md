# Investigation ledger

## Round 1 disassembly (checkpoints pass-001, pass-002)

Method summary (scripts in analysis/pass-001/):
1. Coverage: traced natural play + artificial warps -> traces/coverage-pass001.json.gz (10130 contextual
   ROM/RAM roots, 9704 unique instructions; RAM execution only at $04AA = self-modified `JMP ($B9xx)`).
2. Static recursive descent (an.py/run2.py): roots = vectors, all observed fetches; bank model: 16K mode
   (control $1E), fixed bank 7 at $C000; switch bank only via `LDA #n; JSR $C05A` (or JMP); constant-flag
   branch pruning; stops at RTS/RTI/JMP (ind)/BRK/invalid. No path reached an invalid opcode or BRK.
3. Jump tables (jt.py): tables found from `LDA tbl,X/Y -> STA zp; JMP (zp)`; entries accepted until a
   target fails clean decode; far-call table $C283 (11 entries -> bank 1 jmp vector $800F..); PPU
   unrolled-store entry table $B900 (65 entries, banks 2/3/4).
4. Mirrored engine: banks 2/3/4 are byte-identical $B900-$BFFF and the engine switches its own
   bank (`JSR $C05A` at $BCC4 with A=$04A8&7, continues at $BCC7 in the new bank); all three copies
   were observed executing; unobserved instructions were mirrored only where bytes are identical.
5. Overlap: one BIT-skip idiom (bank 1 $8312 `BIT $02A0` hides entry $8313 `LDY #$02`); the map keeps the
   $8310 decode; entry $8313 noted in leads L-009.
6. Islands (islands.py, reviewed manually): bank 0 $BB80 APU helper and bank 7 $CB82 debug hex printer
   accepted as unreferenced Static code; 6 other candidate islands rejected (data-like: repeated
   `ror $65`, `cmp $de6a,x`, pha runs).
7. Generic data (classify.py): bytes not reached by code with no >=24-byte clean linear decode at any
   alignment -> Data; uniform $FF runs (>=8) -> Padding; $00/$AA/$55 runs -> Data; <8-byte fragments
   without data refs and clean windows stay Unknown. Conflict check: no code branch/JSR/JMP target
   lands in a non-code region (conflicts.py -> 0).
8. Specific tables: per-room tables $BD14/$BE14 (bank 0; room index $23,X as Y), word tables
   $C322/$C422 (fixed; read from bank 6), warp table $928B (4 x {x,y,room,world}), vectors, CHR ROM.

Remaining unknowns: analysis/pass-001/unknown-ranges.txt (157 ranges, 3994 bytes) — see leads L-001..L-009.

Lead outcomes this pass: none consumed (leads file was empty); L-001..L-009 produced.
Not a terminal handoff; no rejection review required.

## Round 1 redirect (attempt analysis/redirect/r01-a; input pass-002 / commentary-001)

Method: exact unknown complement (unk.py -> unknown-complement-pass002.txt, 141 ranges / 3994 bytes),
then consumer-anchored static checks per structure (rom.py helpers), two bounded pointer-chain walkers
(musicwalk.py: SongTable -> order lists -> pattern lists -> 3-byte sequences, song 3 with 7 option
sections; streamwalk.py: 256 scenes through the StreamOpTable contract, op byte = 2*handler index, zero
decode errors), and three artificial runtime scenarios (pharmacy, treasure, ending; exp_*.py).
Result: every unknown range has a disposition (ledger.md). 3991 bytes consumer/structure-backed data or
code, 2 bytes pattern-only generic data ($BA42-$BA43), 1 byte retained unknown (bank 0 $9AB8 lone RTS).
Corrections found: $C088-$C08F is a code idiom (select bank 0, JMP ColdStart) referenced by JMPs at bank 1
and bank 6 $8000; metasprite alt table ends $A2E7 (records start $A2E8); NotePeriodTable is 64 words to
$8671; HummingbirdDY is 16 bytes; GridSlotX $9A3B read via $9A38,y; DPCM sample 23 reads $FFBF-$FFC0
(reset stub bytes). Runtime: new fetches in all three scenarios lie only in existing Static code (no
conflicts with data/unknown). Leads: L-010..L-021 ready (see analysis/leads.json).

## Round 2 disassembly (input pass-002; output checkpoint pass-003)

Method: static, bounded. analysis/pass-003/verify_leads.py re-derives every ready-lead claim from ROM bytes
against the pass-002 unknown complement (28 checks, all PASS: analysis/pass-003/verify.log) and writes the
annotation plan (analysis/pass-003/plan.json, 145 regions / 3993 bytes); analysis/pass-003/apply.py applies it
through romatlas serve (map.load/annotate/validate/save/export). No emulator runs, no gameplay (all remaining
unknowns were statically decidable; scenarios.json unchanged).

- L-010: floor lists parse contiguously $C522-$C75C; all 256 RoomFloorPtrs words hit record boundaries.
  Refinement: $C5B4/$C69B are shared list tails (record boundaries), not list starts.
- L-011/L-012: DPCM spans (23 records) cover b7 $E000-$FFC0; PCM segment 1 $80A9+$3133 ends $B1DB, all bytes $14-$60.
- L-015a: record lists parse contiguously $A2E8-$BD3D; all 372 words in $A000-$A2E7 are record boundaries
  (alt word $A296 -> $BC19 is a shared tail). L-015b..g, L-017, L-019, L-020: consumer index domains and values rechecked.
- L-016a/b/c: music and stream walk outputs (r01-a) cover every gap byte; template spans (18 pointers incl. $8DBF) cover 31 gaps / 54 B.
- L-014: b1 $8000/$8009 and b6 $8000 annotated Code (stride-3 JMP vector heads); b7 $C088-$C08F Data->Code
  (RestartViaBank0: LDA #0 / JSR $C05A / JMP $8000). Reachability unknown.
- L-018: $BA3E-$BA41 table entries; $BA42-$BA43 accepted as generic data (pattern-only, conf 0.7).
  Rationale: exact continuation of the 01/FF pair pattern; no consumer/word/branch reference; a code reading
  needs an entry point into ORA ($FF,X) that would fall into the unrelated routine at $BA44.
- PharmacyLabelText and "BOYH",$FF annotated with kind Text.
- L-007 spot check (analysis/pass-003/l007_spotcheck.py, l007.log): 95 Data blocks adjacent to code; zero code
  control-flow targets into Data. Clean-to-terminator starts: $8D22 (ASCII), b6 $883D (VitaminSpeed table) rejected;
  b7 $C088 (L-014); b1 $85E0-$85E7 reclassified Code AddAToZpWord (L-022: CLC/ADC $00,X/BCC +2/INC $01,X/RTS,
  self-consistent branch, no data read of $85E0-$85E7, no caller; conf 0.75). Mid-block hidden routines not searched exhaustively.
- $9AB8 (L-023): rechecked for JSR/JMP/branch, word $9AB8/$9AB7 and LDA#/PHA pairs: none. Retained unknown.

Result: ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown).

## Round 2 redirect (attempt analysis/redirect/r02-a; input pass-003 / commentary-002)

Method: static surveys (RTS,RTS pairs and their references, absolute/immediate operands into page $9A, small
bank-0 non-code gaps and their consumers, padding fill values, bank-6 template/TypeBounces structure, song end
markers, RAM operand scans) plus four bounded runtime scenarios from ingame0.snap (post-ending restart, root
beer rocket, jack ending trigger x2). No map/source/state changes.
- L-023 reopened by combining independent constraints (consumer exclusion, no fill/alignment convention, enclosure
  in Observed code, 16 referenced RTS,RTS precedents): proposal Code/Static conf 0.6 for b0 $9AB8 (l023-review.md).
- L-024 verified: 12 referenced, whole-copied templates and the 31-entry TypeBounces table contain the $FF runs.
- L-022: no reference of any form; reachability unknown (classification unaffected).
- Runtime: post-ending restart and rocket flight/landing (Earth -> space -> Blobolonia room $A0) visited; jack
  trigger in room $CF blocked (caged blob). 629 new fetch addresses, all in Static code; no conflicts.
Result: 2 ready leads (L-023 1 byte, L-024 kind correction); recommendation CYCLE
(analysis/handoffs/round-02-redirect.md).

## Round 3 disassembly (pass-004; input pass-003 state, byte_identical)

Method: static review and application only (no emulator runs; gameplay already broad and no unknown bytes
needed runtime evidence). Scripts/logs: analysis/pass-004/ (apply.py asserts every check before annotating;
apply.log, verify.log, gapcons_all.py/.log, stats.log; pre-apply state backup state.pre-pass-004.json;
leads backup analysis/leads.pre-disassembly-r3.json).
- L-023 review (b0 $9AB8, physical 0x1AC8, $60, prg_banks [0,1,14,15]) - ACCEPTED as Code, Static, conf 0.6,
  label UpdateBoyStop_OrphanRts, region id 2581, explicitly no reachability claim.
  Claim needed: structural role "return instruction" for one byte; no entry/caller claim. Decisive evidence:
  r02-a rtsrefs.log (16 other RTS,RTS pairs in code; each second RTS referenced by branch/JMP/JSR or a word
  table - rechecked: $992C/$994A/$810C/$863A/$9ADC are word-referenced), absrefs.log + cov9ab8.log (no consumer,
  never read/fetched), new all-bank survey gapcons_all.log (25 small non-code gaps enclosed by code: 24
  consumer-read, the remaining one b7 $CBB7 is visibly non-instruction ASCII "BOYH"+$FF). Why sufficient: the
  policy's "unreferenced return opcode on its own" is insufficient; here the opcode is combined with an
  in-ROM layout idiom (doubled RTS) that is independent of the byte value's decodability, while the competing
  unread-datum reading has no analogue anywhere in PRG (unread non-code between routines exists only as visibly
  non-code text) and no fill convention. Limits: intent of an unreferenced, unread byte cannot be observed;
  revisit if a consumer or read of $9AB8 is ever found. Earlier rejections (round-2 disassembly) tested only
  references and stand as records of that narrower scope.
- L-024 - verified and applied: exactly 25 Padding regions in bank 6 $8DBF-$959A / $A0A6-$A0C4 (203 B, all $FF);
  24 x 8 B at template+$3A/+$50 of the 12 ObjSetTemplates-referenced templates (copy loop b6 $8937:
  ldy #$6D / lda (TempA),y / sta $03A7,y / dey / bpl) and 11 B TypeBounces $A0B3-$A0BD (31-entry table read by
  ObjectsHitFloor, followed by TypeShootable $A0C5). Re-kinded Data/Static 0.9, region ids kept (not merged).
Result: ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

## Round 3 redirect (analysis/redirect/r03-a; input pass-004 / commentary-003)
- Static sanity sweep: all Code regions decode cleanly; every abs JSR/JMP/branch lands on a Code instruction start
  (fixed-bank far calls resolved via their SetPRGBank16K select) or the documented $04AA RAM trampoline; every saved
  instruction fetch (9 exports) is a Code instruction start. No contradiction for $9AB8, $BA42-$BA43, $85E0.
- Ending: $CE key-carry transition reached by game play after a teleport (S14); natural jack delivery to the caged blob
  still fails; artificial eat-state injection shows jack -> EndingCountdown -> ending scene (S13 visited, artificial).
  274 new fetches, all in Static code.
Result: no ready leads; L-006/L-008 deferred; recommendation TERMINAL (analysis/handoffs/round-03-redirect.md).
