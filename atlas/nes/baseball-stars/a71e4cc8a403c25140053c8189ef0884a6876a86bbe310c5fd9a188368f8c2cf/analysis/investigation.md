# Investigation ledger

Follow workflow.md's completion and investigation contract. Newest round last.

## Round 1 disassembly (2026-11-16)

### Platform/bank facts established
- MMC1, reset writes control $1E: 16 KiB PRG mode with $C000-$FFFF fixed to 8 KiB banks 14/15,
  4 KiB CHR mode. A PRG-register value n selects 16 KiB bank n, i.e. 8 KiB pair (2n, 2n+1) at
  $8000/$A000 (an initial pair = n&$0E assumption was falsified by observed contexts: `lda #4;
  jsr $C056; jsr $AC24` executes $AC24 in bank 9; after correction 43 statically bank-tracked call
  targets agree with observed contexts, 0 conflict).
- MMC1 serial writers: $C000/$C002 (control, $9FFF), $C01C/$C01E (CHR0, $BFFF), $C038/$C03A
  (CHR1, $DFFF), $C054/$C056 (PRG, $FFFF); $C054 reloads A from $25 (current PRG value).
- Vectors (bank 15): NMI $CFB3, RESET $CC69, IRQ $D026.

### Inline-argument / stack-return helpers (fixed bank)
| Helper | Contract (from code) | Static check |
|---|---|---|
| $C53A | 2-byte pointer after JSR -> $04/$05, resume after | observed resume points agree |
| $C560 | 2-byte pointer -> $0C/$0D | observed |
| $C791 | jmp ($FF9C) = $CBB7: 2-byte pointer -> $14/$15 | observed (bank 9 $B18C) |
| $C0EE | stream: 2-byte PPU address + bytes; $FE starts new address group; $FF ends (code $C0FA-$C109) | stream at bank 10 $977E ends exactly at next observed instruction (+75) |
| $C178 | jump table after JSR, A*2+1 index, jmp ($35); never falls through | extent bounded by first entry target / next known instruction |
| $C18F | far call: A = PRG value, Y = index into word table $FF80; $2D saves bank | targets $C90A (Y=8) and $C87E (Y=4) consume 6 inline bytes (observed resume points) |

### Method (reproducible, scratch scripts)
- Coverage: sampled traces (TK-001 slowdown) of scenarios S01-S09; `analysis.contexts` dumps
  scratch/ctx*.json (offset, cpu address, prg_banks, hits). All observed contexts matched the
  home mapping (banks 0-13 at $8000+(b&1)*$2000, 14/15 at $C000/$E000).
- Static: scratch/an.py + build.py — bank-aware recursive descent from vectors and observed
  instruction starts; observed instruction bytes are reserved before static traversal; stops at
  unresolved banked targets, indirect jumps, unknown stack-return callees; bank tracking only for
  `lda #imm; jsr $C056` in fixed-bank code (reset after other fixed-bank calls).
- Generic data: scratch/dataclass.py — gaps split into code-suspect zones (>=10-byte valid linear
  decode runs, inline-helper aware, containing >=2 code idioms: JSR/JMP to known fixed entries,
  load/store pairs, in-run branches, RTS; +/-4-byte margin) and remaining runs; a run becomes generic
  Data only if >=16 bytes and either uniform, >=1 invalid opcode per 16 bytes, or dominated by one
  repeated byte. Validation: applied to all 42-50k bytes of known code -> 0 bytes would be data;
  data classified from the earlier coverage set had 0 bytes contradicted by later coverage
  (league/menus added ~3.3k new instruction starts).
- Committed with map.annotate (scratch/commit.py); checkpoints pass-001, pass-002, pass-003 (final).

### Corrections and classifier refinements during the pass (history preserved)
1. Bank mapping n&$0E -> 2n (above). pass-001 contained bank-tracked static code built on the wrong
   pair; superseded by pass-002/003 (maps/pass-001.json kept as history).
2. $C0EE stream length: first model (2 bytes per non-terminator step) was wrong; code at $C0FA-$C109
   reads 1 more byte per step; corrected model matches the observed resume point.
3. Generic-data classifier tightened twice after holdout tests (classify with one coverage subset,
   compare against code found with all coverage): (a) runs of repetitive data such as `55 55`/`50 50`
   were being held as code-suspect -> require a strong idiom (JSR/JMP to known instruction start,
   or store to RAM/IO); (b) pass-002 had classified bank 2 $932C-$936D (66 bytes) as generic data, of
   which 18 bytes ($933A `lda $0533; jsr $C178` and similar dispatch stubs between jump tables) were
   later observed executing -> runs containing any JSR/JMP byte pattern to a known fixed-bank entry are
   now excluded. pass-003 removes that error. After the refinements every holdout subset (ctx.json,
   ctx2.json, ctx2b.json alone) yields 0 contradicted data bytes, and 0 of 54.6k known code bytes
   would be classified as data.

### Remaining unknown (pass-003): 9,483 bytes, all PRG; 16 clusters >= 64 bytes hold 8,137
b2 $8A1B-$8A5C (66); b2 $8FD4-$909E (203); b2 $96FD-$9849 (333); b2 $985C-$99B0 (341);
b2 $99F5-$9D21 (813); b3 $A290-$A478 (489); b3 $A58B-$A82E (676); b3 $B746-$B8A2 (349);
b7 $B36B-$B5C9 (607); b7 $B62F-$BA40 (1042); b7 $BB17-$BD25 (527); b7 $BD36-$BE83 (334);
b11 $BA95-$BC74 (480); b12 $813B-$81BC (130); b14 $C126-$C16B (70, unreferenced $C0EE-like
stream helper variant); b15 $EE05-$F491 (1677: data $EE05-$EE12 then dispatch `lda $0485; jsr $C178`).
Nearby strings tie them to features not reached: bank 7 hire/fire/remove team/power-up/buy
("WHAT STYLE OF PLAYER?", "SIGN CONTRACT", "FIRE KEEP", "THIS TEAM IS IN A LEAGUE"), bank 15 trade
("TRADE COMPLETED"), bank 2 in-game substitutions (RELIEF PITCHER / PINCH RUNNER), bank 11 league
leader pages. The rest are <64-byte fragments (classifier minimum 16 bytes, code-suspect margins).

### Open questions / leads (see analysis/leads.json)
- L-R1-01..: unreached code clusters in banks 2, 3, 7, 15 (code-suspect zones), unbounded jump
  tables, conditional menu items (TRADE/HIRE/FIRE PLAYER, REMOVE TEAM), indirect jumps.

## Round 2 disassembly (pass-004, 2026-11-18)

Input pass-003 (9483 Unknown). Tools: scratch/r2/ (proposal.py + desc.py = reproduction of
r1-redirect-01 proposal; verify.py, roots.py, resid.py, ctl.py, fall.py, build1.py, build2.py, apply.py).

- L-R2-01/02/03/05 verified independently: reproduction identical to proposal-static.json; every root
  referenced by a classified handler table (MainStateHandlers $CCE1 words 30-41, StateBankTable-selected
  banks) or by a proposed `jsr $C178` / indexed+`jmp (ind)` table; linear decode of every code range ends on
  its boundary, 0 unofficial opcodes, 0 BRK; every inline range follows a JSR to an inline-argument helper or
  far call; every table entry is a descent instruction start; 2232 runtime-observed starts inside, 0 misaligned.
  Applied 7848 bytes (+69 Data->Code corrections b2 $8A78-$8A90, b3 $BC04-$BC2F; both end in rts and are
  JSR targets). Fix: proposal range $8A40-$8A90 overlapped existing code $8A5D-$8A77 (allow-list artefact),
  applied as two ranges.
- L-R2-09: 3 x `lda $2x` prologues accepted (structural, parallel to ReloadPrgBank).
- L-R2-04: structurally accepted but not representable (TK-003); 1 byte stays Unknown.
- Residual scan (resid.py/ctl.py/fall.py): no branch/jump/call from classified code into any residual;
  only two residuals are entered by fall-through: b13 $A07A (L-R2-04) and b3 $B39E.
- CORRECTION (L-R2-08 premise): b3 $B39E-$B3AE follows far call `lda #4; ldy #$0E; jsr $C18F` -> $CB30, which
  pulls the return address and returns through `jmp ($35)` to return+1 (normal return; r1 heuristic treated it
  as stack-consuming). Classified Code; later observed 7/7 in the natural 2PLAYER team-select route (S17).
- L-R2-06/07 applied as Data with per-range consumer evidence (118 ranges, 1469 bytes). Notable roles:
  b2 $8CE5-$8CEA RAM routine template (copied to $0742, patched, executed in RAM); b10 $9B09-$9B0D table with
  index domain 0..4; b0 $87E7 SndMusicTable pointer head; b5 $BFFE-$BFFF bank tail (no code start possible).
- Kept Unknown: b10 $9B08 (1), b12 $9026-$9028 (3, `jmp $8DB8` unreachable) -> L-R3-01; L-R2-08 remainder
  240 bytes (re-searched $FF80 far table and LE words: none; not fetched in S17).
- S17 2-player game (3000 traced frames): 9687 starts, 0 in Unknown/Data, 0 misaligned.

### Rejection review (remaining 245 Unknown bytes)
- L-R2-08 fragments: claim = code. Decisive evidence: no JSR/JMP/word/far-table reference, no fall-through
  predecessor (all preceded by rts/jmp), no fetch in any trace including S17. This rules out every *known*
  entry path, not code-ness; competing interpretation (dead code vs. data that decodes) is not discriminated by
  decoding alone. $C126 contract similarity to PpuBufAppendInline is suggestive but no consumer supplies its
  inline argument. Remaining reason: missing entry evidence; revised test would be a new selector/indirect
  jump discovery, not available from current tables.
- $9B08/$9026: single code-shaped remnants between a terminal jump and a proven table; ambiguous.
- $A07A: role established; blocked only by toolkit representation (TK-003).

## Round 2 redirect r2-redirect-01 (2026-11-18, input pass-004)
- CORRECTED PREMISE (L-R2-08 rejection above): the review required entry evidence; policy allows static code from
  independent routine contracts. Entry mechanisms re-searched (LE words, split immediates incl. target-1, $C178/$FF80,
  edges into Unknown, fall-through): all negative; reachability stays unresolved. Structural evidence per fragment
  (duplicates: $AC36 == SetupSprite0Split3, $AC73 == TileXYToPpuAddr; contracts for $B496, $B820, $ACC2, $8DE7, $C126)
  -> ready lead L-R4-01 (240 B). Details/rejection review for the remaining 5 B: analysis/handoffs/round2-redirect.md.
- L-C2-01 closed statically (MenuStep in state 15 is {0,1,2}); L-C2-02 closed by S08.

## Round 3 disassembly (pass-005, 2026-11-18, input pass-004)
- L-R4-01 independently judged per fragment (evidence-policy: structural claim separate from reachability).
  Re-ran analysis/redirect/r2-redirect-01/tools/check.py (ALL OK) and dup.py; own disassembly of each fragment
  with surrounding context:
  - b3 $AC36-$AC46: 17/17 identical to SetupSprite0Split3 $A8AC, followed (as at $A8BD) by ClearSplitMode copy $AC47. ACCEPT.
  - b3 $AC73-$AC97: 37/37 identical to TileXYToPpuAddr b2 $8CA2; callee of $B4A3. ACCEPT.
  - b3 $B496-$B4C4: menu erase handler, jsr $AC73 / jsr $C53A + inline .addr $AFCE (Data $B4A9-$B4AA) /
    jsr $C58A / skip-list append / jmp $AF71. ACCEPT (45 Code + 2 Data).
  - b7 $B820-$B83A: 18-byte LeagueSlotStats scan, CustomTeamFlags/LogoUsed writes matching FIRE/REMOVE idioms. ACCEPT.
  - b9 $ACC2-$ACD4: <$10 blank+digit PpuBuffer append else bcs PutHexByte $ACD5. ACCEPT.
  - b12 $8DE7-$8DFD: runner check, UmpireCall SAFE + timer 60 pattern. ACCEPT.
  - b14 $C126-$C16B: PpuBufAppendInline protocol/epilogue with hex body (== b9 PutHexByte body). ACCEPT.
  All Static, confidence 0.8, evidence text "no caller found; structural by duplicate/contract; reachability unresolved".
- L-R3-01 / L-R2-04 not retried (no new evidence; prior rejection review in round2-redirect handoff still applies).

## Round 4 disassembly (pass-006, 2026-11-18, input pass-005)
- Recipe check: `python3 analysis/redirect/r3-redirect-01/tools/check.py` -> validate ok, byte_identical, 1 unknown.
- L-R2-04 (b13 $A07A, 1 B) ACCEPTED: region 3514 extended to phys $1A08A len 62, view $A07A [12,13,14,15], Static.
  Decode confirms $A078 `sbc #$01` (non-terminal, classified) -> $A07A `bcc $A07D` / $A07C `rts` / $A07D `jmp $A165`;
  the old stream realigns at $A07D. Fidelity judgment: every byte's role (Code) is correct in both streams; only the
  observed alternate start $A07B (`ora ($60,x)`, FieldCommonUpdate `jsr $A07B`) is not a map instruction boundary.
  It is stated in region evidence; generated source uses numeric `jsr a:$a07b`. No toolkit metric flags it
  (analysis.conflicts unsupported on NES; coverage is execution-only) -> TK-003 update. Original region evidence kept.
- L-R5-01 (b12 $9026-$9028, 3 B) ACCEPTED: new Code region 3988, view $9026 [12,13,14,15], Static, `jmp $8DB8`
  (RunnerMoveAlongPath). Both bounds fixed (end of `jmp $8FAF` at $9025; table base $9029 by `and $9029,y`).
  Accepted on combined constraints (exact gap, operand = verified entry, ObjState 0/body pairing) - more than a bare
  plausible pointer; data reading has no consumer. Not reachable as written; no reachability claimed.
- b10 $9B08 (1 B) kept Unknown: r3-redirect-01 rejection review still applies (no reference/consumer/execution).
- Edits: scratch/r4/build.py -> scratch/r4/batch.json, applied by scratch/r4/apply.py via scratch/broker.py.
  No gameplay this pass (no residual tied to an unvisited state; scenarios.json unchanged).

## Batch exception: user-directed Manual fallback for the last residual byte (pass-007, 2026-11-18)
- Under an explicit, batch-only user authorization (applies only when at most 10 physical bytes remain unclassified),
  the single residual byte phys $15B18 (offset 88856, length 1; b10 $9B08) was annotated as generic Data with
  UNKNOWN MEANING, provenance Manual, confidence 0.5, no label (region 3989). ROM bytes unchanged.
- This is NOT evidence-backed classification. The prior ambiguity stands: lone unreferenced `$60` after
  NameGrid_Right's `jmp PlaceGridCursor`, before GridRightWrap ($9B09, extent closed by X=0..4); leftover `rts` vs pad
  byte both consistent; no reference/consumer/execution (L-R3-01; rejection review in analysis/handoffs/round3-redirect.md).
- Prior Unknown records above (pass-004..pass-006) are preserved as history. Checkpoint maps/pass-007.json.
