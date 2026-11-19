# Round 2: redirect phase handoff (attempt r2-redirect-01)

Status: **READY-FOR-DISASSEMBLY**. There is one ready lead, L-R4-01: 240 bytes (238 Code
and 2 inline Data). After it is applied, 5 Unknown bytes would remain, and their
rejection review is below.

## Scope and checkpoint
- The input is `maps/pass-004.json` (sha256 c1ceb883b29f...), which equals
  `analysis/state.json`. Classification is 99.91% (261915 / 262160; 245 Unknown).
- Redirect changed research and lead records only. The map, the ROM
  (a71e4cc8... unchanged), the source and the toolkit were not touched, and no
  server was started.
- Research directory: `analysis/redirect/r2-redirect-01/`
  - `tools/dis.py`, `dup.py`, `check.py`, `jmprts.py` and `entries.py`
  - `proposal-r2.json`
  - the `*-output.txt` files
  - `unknown-complement.txt`
  - `leads-before.json` and `scenarios-before.json`, the pre-edit copies.

## Exact unknown complement (pass-004, recomputed from region gaps)
| phys | bank / CPU | bytes | disposition |
|---|---|---|---|
| $06C46 | b3 $AC36-$AC46 | 17 | L-R4-01 ready |
| $06C83 | b3 $AC73-$AC97 | 37 | L-R4-01 ready |
| $074A6 | b3 $B496-$B4C4 | 47 | L-R4-01 ready (45 Code + 2 inline .addr) |
| $0F830 | b7 $B820-$B83A | 27 | L-R4-01 ready |
| $12CD2 | b9 $ACC2-$ACD4 | 19 | L-R4-01 ready |
| $18DF7 | b12 $8DE7-$8DFD | 23 | L-R4-01 ready |
| $1C136 | b14 $C126-$C16B | 70 | L-R4-01 ready |
| $15B18 | b10 $9B08 | 1 | L-R3-01 deferred (rejection review) |
| $19036 | b12 $9026-$9028 | 3 | L-R3-01 deferred (rejection review) |
| $1A08A | b13 $A07A | 1 | L-R2-04 blocked (TK-003) |

## Findings
1. **Corrected premise for L-R2-08.** The earlier rejection
   (`analysis/investigation.md`, Round 2) treated a caller or entry as the missing
   prerequisite. Evidence-policy separates two claims: structural code and
   reachability. Static code is allowed when the routine contracts are independently
   supported, so each fragment was re-evaluated on that basis.
   - First, every non-obvious entry mechanism was checked again, and all were
     negative:
     - LE words in all banks (only an operand coincidence at b3 $B9D8 and
       other-bank data)
     - split immediate hi/lo pairs for each target and target-1, covering RTS
       dispatch, pushed addresses and JMP (ind) vectors
     - the $C178 and $FF80 word tables
     - control-flow edges from classified code into Unknown bytes
       (`entries-output.txt`: 0 valid; the 2 hits are odd banks at $8000, which is
       impossible in 16K mode)
     - fall-through: every fragment is preceded by rts or jmp
   - None of the fragments was fetched in any S01-S17 trace. **Reachability stays
     unresolved.** This is not a claim that the code is unused.
2. **Structural evidence per fragment** (`check-output.txt`: decode ends aligned on
   every range, 0 unofficial opcodes, 0 BRK, every branch, JSR and JMP target is
   classified code or lies inside the fragment):
   - b3 $AC36-$AC46: byte-identical (17/17) to SetupSprite0Split3 $A8AC. As at the
     original, it is followed by a ClearSplitMode copy (classified $AC47
     ClearSplitMode2, called by State6), so the whole pair $A8AC-$A8C2 is duplicated
     at $AC36-$AC4C. The ROM holds 4 copies of ClearSplitMode, which shows that
     helpers are duplicated per bank. The operand $8BF1 is MenuMiscTable in paired
     bank 2.
   - b3 $AC73-$AC97: byte-identical (37/37) to TileXYToPpuAddr in b2 $8CA2. The
     $B496 fragment calls it with exactly that contract.
   - b3 $B496-$B4C4: a main-menu "erase the chosen item" handler:
     - It sets MenuSubStep = 0.
     - It computes Ptr0C from the cursor cell ($0551/$0552) via $AC73.
     - It calls `InlinePtrTo04 .addr $AFCE`. That address is TxtBlanksThenVs, whose
       first 6 spaces are the documented item eraser, and the handler draws exactly
       6 characters with DrawTextWide.
     - It appends MenuList to the skip list `MenuCount[CursorMin++]`, the same idiom
       as State6 b3 $AEA0-$AEA8.
     - It sets JoyPressed = 4 (Down) and does `jmp MainMenu_Cursor`, which is
       handler 0 of the same $C178 table.
     - $B4A9-$B4AA are inline-argument Data, as with every other InlinePtrTo04
       argument.
   - b7 $B820-$B83A (the FIRE bank):
     - It scans the 18 LeagueSlotStats bytes from SlotStatsBase. 18 is the record
       stride.
     - If all 18 have bit 7 set, it marks the team slot empty:
       `sta CustomTeamFlags-8,TeamForSide`. FIRE uses the same index at b7 $B7A5.
     - The pair `ldy TeamLogo+8,x / sta LogoUsed-8,y` is byte-identical to REMOVE at
       $B655.
     - Quirk: X is the team number here but the slot in REMOVE, which looks like a
       latent bug in unreferenced code. This is a mechanical note only.
   - b9 $ACC2-$ACD4: `cmp #$10 / bcs PutHexByte` jumps to $ACD5, the next classified
     routine. Below $10 it writes a blank and a digit to PpuBuffer,x with the same
     X-append convention.
   - b12 $8DE7-$8DFD: an X-indexed runner-object check:
     - It tests ObjFlagsB.
     - It compares ObjNextBase with ThrowTargetBase, the same comparison as $8F8F.
     - It sets UmpireCall = 1 (SAFE) and an ObjSlot = 60 timer, the same pattern as
       $87FE.
   - b14 $C126-$C16B:
     - Its inline protocol (PpuBufBusy, JumpPtr, PpuBufLen) and epilogue are
       identical to PpuBufAppendInline $C0EE.
     - It calls PpuBufCopyInlineByte and NibbleToHexTile.
     - Its 20-byte hex body equals PutHexByte in b9.
     - It is a hex print with an inline PPU address. Mechanically, `pla/pha` re-reads
       the same stacked byte X times.
   - The competing interpretation is "data that happens to decode". Byte-identical
     multi-instruction copies of called routines, and operands that land exactly on
     the matching classified tables, RAM fields and helpers in the correct bank
     context, are independent constraints that rule it out. No copy or upload
     consumer was found either: there are no word or immediate references, so the
     "RAM template" alternative is also unsupported.
3. **L-C2-01 closed statically.** These are all writers of MenuStep ($0530) in the
   PRG:
   - State 14 init $EB4E (= 0)
   - $EBC7 inc (0 -> 1)
   - b2 $9328 inc in SubMenu_Show (1 -> 2)
   - b2 $9292 dec in SubMenu_Nothing (1 -> 0)
   - b2 $90FA dec, followed by a re-open (2 -> 1 -> 2/0)
   - the remaining writers belong to other states (b1 $A191/$A1BA, b3 $AD26,
     b11 $BDA8/$BE48)

   State 15 is entered only from state 14, which sets MenuStep to 0, or from state 16,
   whose dec does not touch MenuStep. So step 3 (State15_RestartStep) is never
   selected, and the PRG-4 + `jsr $9D22` path cannot occur. Runtime agrees: there were
   0 fetches at $EB87 in the S11/S14 substitution traces. There is no byte impact.
4. **L-C2-02 closed.** S08 already observed END PLAY leading to the "WARNING!! hold
   RESET before power-off" screen, which matches the static scroll-to-warning-page
   mechanics. This is semantic only.
5. **Gameplay review** (scenarios.json S18). Every terminal or later milestone has
   been visited: CPU, 1P and 2P games, both substitution sides, league season end
   with the CHAMPION screen, END PLAY, and all custom-team menus. The game has no
   credits or other ending, and no unvisited distinct milestone is connected to the
   residuals.

## Ready lead (for the next disassembly pass)
- **L-R4-01** (ready, medium): 240 B over the 7 ranges above. Exact entries are in
  `analysis/redirect/r2-redirect-01/proposal-r2.json`, with physical offset, length,
  CPU start, prg_banks and kind.
  - Recipe:
    1. Run `python3 analysis/redirect/r2-redirect-01/tools/check.py`; it should
       print ALL OK.
    2. Re-check byte identity with `tools/dup.py`.
    3. Annotate with Static provenance. The evidence text should state "no caller
       found; structural by duplicate/contract".
  - The fragments are independent, so the reviewer may reject any one of them
    without affecting the others.

## Rejection review (residual after L-R4-01: 5 bytes)
- **b10 $9B08 (1 B, $60).**
  - Claim: dead RTS code.
  - Decisive evidence: it follows `jmp $9D79` and precedes GridRightWrap, whose index
    domain 0..4 is proven to start at $9B09. There are no word or immediate
    references.
  - New measured check (`jmprts-output.txt`): all 20 jmp+$60 pairs inside classified
    code are branch targets, so the ROM gives no precedent for a dead RTS after a
    JMP.
  - Competing interpretation: a pad byte or data.
  - It is a single byte with no contract, and the policy says an "unreferenced return
    opcode lacks structure". It stays Unknown.
  - Revised test: none without a reference.
- **b12 $9026-$9028 (3 B, `jmp RunnerMoveAlongPath`).**
  - The target is a real JMP destination: the same 3 bytes appear in classified code
    at $912C. It mirrors the preceding `jmp $8FAF`, and both destinations are
    `jsr RunnerPastBase` points, which suggests an edited-out alternative exit.
  - However, it is one instruction with no entry and no consumer contract, which the
    policy calls a "plausible pointer". It is not discriminated from data, so it
    stays Unknown.
  - Revised test: none without a reference.
- **b13 $A07A (1 B).**
  - Its role is established: it is the opcode byte of the fall-through `bcc`, whose
    operand overlaps the instruction stream entered at $A07B.
  - The map cannot represent it (TK-003, unchanged, toolkit not modified). It is
    blocked on the toolkit only.

## Remaining avenues
None actionable beyond L-R4-01. A future reference or selector discovery would reopen
L-R3-01, and a TK-003 toolkit feature would release L-R2-04.
