# Round 3: redirect phase handoff (attempt r3-redirect-01)

Status: **READY-FOR-DISASSEMBLY**. Two ready leads cover 4 of the 5 residual Unknown bytes
(L-R2-04: 1 B, L-R5-01: 3 B). The remaining 1 byte (b10 $9B08) stays Unknown; its rejection review is below.

## Scope and checkpoint
- Input: `maps/pass-005.json` (sha256 407f32c4ca37...) == `analysis/state.json` == `maps/current.json`;
  262155/262160 classified, 5 Unknown. ROM sha256 a71e4cc8... unchanged.
- Redirect changed research/lead records only (leads.json, toolkit-issues.md note, this handoff, progress.md).
  Map, state, source and toolkit untouched. No server started. Scratch maps/generated output removed.
- Research dir `analysis/redirect/r3-redirect-01/`: `proposal-r3.json`, `tools/check.py` (builds pass-005 +
  proposal in a scratch copy and runs validate/generate/verify with the configured binary),
  `check-output.txt`, `tools/deadafter.py` + `deadafter-output.txt`, `unknown-complement.txt`,
  `leads-before.json`, `notes.md`.
- Combined scratch result (both proposals): `map validate` ok, `verify` byte_identical true, 1 Unknown byte left.

## Exact unknown complement (pass-005)
| phys | bank / CPU | bytes | disposition |
|---|---|---|---|
| $15B18 | b10 $9B08 | 1 | L-R3-01 kept Unknown (rejection review) |
| $19036 | b12 $9026-$9028 | 3 | L-R5-01 ready (Code) |
| $1A08A | b13 $A07A | 1 | L-R2-04 ready (Code, corrected premise) |

## Ready leads
1. **L-R2-04 (b13 $A07A, 1 B): corrected premise.**
   - Prior decisive artifact (TK-003, round-2 disassembly): `map.annotate` rejected a *1-byte* Code region at
     $A07A ("truncated instruction"). Reproduced (scratch optC). That falsifies only the 1-byte representation,
     not every representation of the established role.
   - Read-only docs check (/root/romatlas/docs/nes.md, protocol.md): no overlapping-view feature exists; regions
     cannot overlap; a view must resolve every instruction and operand byte.
   - Supported representation: extend Code region 3514 ($A07B-$A0B7) back to $A07A with view cpu $A07A,
     prg_banks [12,13,14,15]. Decode: `bcc $A07D` (90 01) / `rts` / `jmp ThrowToBase` ..., which realigns with the
     old stream at $A07D, so the rest of the region is unchanged. validate/generate/verify pass byte-identically.
     (Merging into region 3513 from $A000, length 184, also validates; the extension is the smaller edit.)
   - Trade-off: the observed alternate entry $A07B (`ora ($60,x)`, FieldCommonUpdate `jsr $A07B`) becomes an
     instruction start the view cannot show. Record it in region evidence, and in commentary as
     `FieldUpdateThrowEntry = * - 1` after the `bcc`. The generated source uses numeric `jsr a:$a07b`, so
     fidelity is unaffected.
2. **L-R5-01 (b12 $9026-$9028, 3 B, `jmp RunnerMoveAlongPath`): newly combined constraints.**
   - Round 2 rejected it as "one instruction, no entry, plausible pointer". New independent constraint: state/body
     pairing. RunnerInput_Return has just stored ObjState = 0. State 0's movement body is RunnerMoveAlongPath
     (RunnerState0_Batter = `jsr RunnerInput_Advance` then RunnerMoveAlongPath). Every other set-state-then-jmp
     site pairs a state with its own body:
     - $912C: 0 -> RunnerMoveAlongPath
     - $90CF: 0 -> RunnerState0_Batter
     - $90A3: 1 -> RunnerState1_Running
     - $8F38 (sibling RunnerInput_Advance): 1 -> RunnerRun_Move

     The live `jmp RunnerRun_Move` at $9023 is the only exception, and $9026 restores the pairing. This fits a
     superseded tail.
   - Combined with:
     - the exact 3-byte gap between $9025 and PadBitForBaseBack, whose base $9029 is fixed by its consumer
       `and $9029,y` with Y in 0..3
     - an operand equal to a verified entry, byte-identical to the classified `jmp` at $912C
     - the sibling ending with a single jmp before its own table
   - Negative checks: no LE word $9025/$9026/$9027 in PRG. $9028 appears only as an opcode coincidence
     (`28 90` = plp/bcc at b12 $88C5). No branch or fall-through entry, and no data consumer.
   - Claim: structural Code (Static) only. It is **not reachable as written**, and no reachability is claimed.
   - Reviewer may reject independently of L-R2-04.
   - Recipe for both: `python3 analysis/redirect/r3-redirect-01/tools/check.py` (expect validate ok,
     byte_identical, 1 unknown), then annotate per `proposal-r3.json` (evidence strings included).

## Rejection review: b10 $9B08 (1 B, $60), kept Unknown
- **Exact claim tested:** phys $15B18 is Code (a stray `rts`) or generic Data, in context [10,11,14,15].
- **Decisive evidence:**
  - It lies between NameGrid_Right's final `jmp PlaceGridCursor` ($9B05) and GridRightWrap. The table's base is
    fixed at $9B09 by `lda $9B09,x`, and X = BallPosX+1 & 7 is bounded to 0..4 by `cmp #$4D`/`cmp #$48`, so the
    table cannot include $9B08.
  - No LE word $9B07/$9B08, no branch target, no fall-through.
- **What that rules out:** membership in GridRightWrap and any entry. It does not decide between code and data.
- **New facts this round:**
  - (a) The same name-grid code uses `jsr PlaceGridCursor / rts` at $9BB7. A `jmp` followed by `$60` therefore
    fits a jsr->jmp tail-call edit that left the rts behind.
  - (b) The sibling NameGrid_Left ends `jmp PlaceGridCursor` immediately before GridLeftWrap, with no extra byte.
    That fits an edit on one side only, but it equally fits a pad byte.
  - (c) `deadafter.py`: no contiguous classified code contains an unreferenced instruction after an unconditional
    transfer. Round 2 found that all 20 jmp+$60 pairs inside classified code are branch targets. There is no
    accepted precedent for either reading.
- **Strongest competing interpretation:** a 1-byte pad or data byte. Unlike L-R5-01, the byte carries no operand,
  so there is no content constraint beyond the single value $60. The idiom story constrains edit history, not
  structure. The policy names an "unreferenced return opcode" as insufficient on its own, and nothing independent
  makes it clearly non-code either.
- **Result:** materially ambiguous, so it stays Unknown. The only revised test that would change this is a
  reference, consumer or execution of $9B08. Nothing in the source points to one, and the round-2 entry-mechanism
  searches (words, split immediates, RTS dispatch, $C178/$FF80 tables) were all negative.

## Other avenues
- **FIRE-last-player trace for bank 7 $B820 (MarkTeamEmptyIfAllFired):** skipped.
  - It would be reachability evidence only; the bytes are already classified.
  - Statically excluded: no word, immediate, branch or fall-through reference (r2 entries-output), and bank 7
    has no `jmp (ind)`.
  - Not worth the cost of a server and a RAM-patched roster.
- **Gameplay:** scenarios.json S18 review unchanged. The CHAMPION screen is the terminal milestone, there are no
  credits, and no residual is connected to an unvisited state.
- **After these leads are applied:** 1 Unknown byte ($9B08) remains, under the rejection review above. Whole-ROM
  completion would then depend only on that byte.
