# Round 2 redirect handoff (attempt r2-a1)

Input checkpoint: pass-002 (byte-identical; maps/current.json = maps/pass-002.json).
Classification is unchanged at 163,833 / 163,856 bytes, with 23 unknown. This redirect
did not change the map, state, commentary, ROM or toolkit. It changed:
- analysis/leads.json (pass record appended; L-017/L-018/L-019 updated; L-020 and L-021 added);
- analysis/scenarios.json (SC-016 appended).

Research files are in analysis/redirect/r2-a1/. The before-copies of the leads and
scenarios files are kept there.

**Recommendation: NEXT ROUND.**

| Lead | Status | Proposal |
|---|---|---|
| L-020 | ready | 12 bytes, Code, Static |
| L-021 | ready | commentary correction, 0 bytes |
| L-019 | ready | metadata split, carried over, 0 bytes |

If L-020 is accepted, the expected result is 163,845 / 163,856, with 11 bytes left unknown.
L-017 now covers only those 11 bytes. It is blocked; the rejection review is below.

## 1. Residual fragments (L-017): a structural-constraint test

**Question.** Can any of the nine post-JMP/RTS fragments be classified from constraints
that do not depend on their own byte pattern?

**Method.** All scripts are read-only, in r2-a1/tools/, with output in r2-a1/*.txt.
- `frag.py` decodes classified code and finds IF/ELSE skeletons: a forward conditional
  branch to T, with a `JMP E` (E > T) immediately before T. For each fragment it lists
  the instruction before it, the branches that target its end, and the decoded target.
- `follow.py` and `epi.py` work out what follows unconditional transfers and backward
  (loop-closing) JMPs.
- `immref.py` and a word scan search for references.

**Findings.**

1. **The IF/ELSE skeleton is pervasive.** It appears 460 times in classified code. In 12
   of those, the JMP directly follows another unconditional transfer, and all 12 JMPs
   are referenced (precedent-dead-jmps.txt).

2. **Four JMP fragments fit the skeleton exactly, including the ENDIF target.** In each
   case the fragment sits between a classified unconditional JMP and the ELSE label of a
   classified forward branch. Its single `JMP abs` targets the first instruction after
   that ELSE block:

   | Fragment | ELSE branch | ENDIF target |
   |---|---|---|
   | 4964 ($9354) | `BEQ $9357` at $934E | $935D, the RTS after `PLA/SEC/SBC/BNE` |
   | 84584 ($8A58) | `BMI $8A5B` at $8A40 | $8A6F, the end of PowerMeterRise (20 bytes) |
   | 84607 ($8A6F) | `BNE $8A72` at $8A38 | $8A74, the RTS after the 2-byte `INC` |
   | 117621 ($CB65) | `BNE $CB68` at $CB56 | $CB8F, the end of the escape/letter else block and the routine exit |

   The bank-10 pair nests consistently. The inner ENDIF ($8A6F) is the outer JMP ENDIF,
   and the outer ELSE branch encloses the inner one.

   For unreferenced data to look like this, each triple would have to be $4C plus that
   exact address by chance.

3. **No reference to any fragment exists.**
   - The word scan found three hits for an address or address-1, and all are
     coincidental:
     - phys 536 lies across the last byte of the table at 507..537 and the first opcode
       byte of the code at 537. This refutes the `$980C` idea in the old L-017 recipe.
     - phys 67504 is inside bank-8 records.
     - phys 47527 (and its copy at 63911) is inside DPCM data.
   - The immediate-pair scan (FarCall and ZP pointer setups) found nothing.
   - There is no RTS-trick dispatch. The JMP-indirect sites use ZP $00, $30 or $036C,
     and those are filled from classified tables or symbolic immediates.

4. **RTS epilogues are not systematic.** The 209 backward JMPs in classified code break
   down as:
   - 180 are followed by their own loop exit;
   - 24 are followed by something else;
   - 4 are followed by unknown bytes;
   - 1 targets another bank.

   One of the 24 is a direct counterexample in bank 14: the routine at $CBBD loops with
   an internal `BNE/RTS` exit and closes with `JMP $CBBD` at $CBC7. The next JSR-entered
   routine starts at $CBCA with no RTS between them (epilogue-counterexamples.txt). This
   is the same shape as DelayFrames_Body and SquareRoot_Body.

5. **The return idiom has no dead precedent.** `LDA #$FF / RTS` occurs 19 more times in
   bank 6. Every occurrence after a JMP or RTS is referenced.

**Result.** L-020 is ready, covering the 4 JMP fragments (12 bytes). Its recipe and
rejection criteria are in leads.json. The other 11 bytes stay unknown, as shown below.

## 2. Rejection review for the 11 remaining bytes (L-017, blocked)

Each fragment below was tested for the same claim: the bytes are Code (instructions
assembled as part of the neighbouring routine) rather than data, with exact bounds equal
to the gap.

Common evidence for all five:
- Every bounding instruction and routine is classified.
- No reference was found by the word scan, immediate pairs or indirect dispatch.
- None was executed in any round-1/2 trace (traces/round-2/cov.json).

**6173+1, bank 0 $980D: RTS after EndingSequence's `JMP $97C6` loop; EndingPictureAll
follows.**
- Constraints: the gap is bounded; it is one RTS; there is no reference.
- The epilogue-rule counterexample (point 4 above) rules out "a loop-closing JMP always
  gets an RTS". It does not rule out code.
- Competing reading: a 1-byte data or filler byte between routines.
- Nothing distinguishes the two readings. An unreferenced RTS alone is explicitly
  insufficient under evidence-policy.md.
- No remaining test: there is no consumer, no generator rule and no reference.

**117501+1, bank 14 $CAED, and 118162+1, bank 14 $CD82: RTS after the loop JMPs of
DelayFrames_Body and SquareRoot_Body.**
- This is the same claim, and the same-bank counterexample at $CBC7 applies directly.
- Same conclusion as 6173.

**51444+3, bank 6 $88E4: `LDA #$FF / RTS` after CpuChooseShot's tail
`JMP AimAtTargetBall`; CpuChooseShot_Done (16 JMPs) follows.**
- The idiom is common in bank 6, but this is a restatement of the byte pattern, not an
  independent constraint.
- Every classified occurrence after an unconditional transfer is referenced. Unlike
  L-020, no enclosing structure predicts these bytes.
- Competing reading: a 3-byte data constant with no reader. That reading is also
  unsupported.
- Decided by: none. It stays ambiguous.

**85771+5, bank 10 $8EFB: `LDA #$80 / STA RoundState / RTS` between
PracticePlaceBalls_Done's RTS and the "YOU MAY MOVE ANY BALL" string header.**
- The sequence `LDA #$80 / STA $A3` occurs in classified code only in bank 11 (phys
  90302 and 90483), not in bank 10.
- The bytes are not valid string framing: they are not text codes, and the header at
  85776 starts its own record.
- Strongest competing reading: string-area residue.
- No consumer, no reference and no structural slot predicts it.

**Why no revised inference remains.** For these five, the only constraints are the
decode and the bounded gap. Both are restatements of the bytes themselves. The one
candidate generator rule (an epilogue after a loop) was falsified within the same banks.
No pointer, dispatch or consumer exists to anchor them. A runtime experiment that forced
execution would only show the mechanics, not intended code, so it would not decide
anything.

These 11 bytes should stay unknown unless new code evidence appears, such as a newly
found indirect reference. If L-020 is accepted and the coordinator wants a terminal
assessment, this review is the one to inspect.

## 3. L-018: runtime check of the passwords (verified with natural input)

The driver is r2-a1/tools/pw.py. It runs a fresh emulator per run, with no RAM edits,
and uses a scratch store copy that has since been deleted. The route is in scenarios
SC-016, and the screenshots are in r2-a1/evidence/.

- **Round trip.**
  - Along the SC-015 route, the screen after winning stage 1 shows a 7-consonant password
    at frame 3188. Its ASCII stays at $05EA.
  - Entering it through tour CONTINUE (menu choice 0) restores Player1Name and sets
    TourStage = 1 straight away.
  - The tour map then resumes with the NY->Milwaukee route. During the route animation
    TourStage briefly reads 0, then returns to 1.
- **Special passwords.** Each of the three, entered as 4 letters plus END, gives
  TourStage 5, 6 and 7. The screens shown are the sound test, the ending picture and the
  voice test.
- **Commentary defect (L-021).**
  - NameEntryInput returns `GridLetterMetatiles[code]`, which is the letter number with
    A=1. It does not return the grid code.
  - So the entry buffer, SpecialPasswords and the PasswordCharValue index all use letter
    numbers. The special passwords are consonant-only words, which is why they can be
    typed even though the vowel cells are refused.
  - The runtime buffer values confirm this.
  - Side question for the uncertainties file: NameEntry's `$25 -> '.'` test compares the
    returned metatile, and $25 is index 28, which is outside the 7x4 grid.

## 4. Other avenues and why they were deferred

- **L-019 (bank 10 region split).** The arithmetic was rechecked and the lead is still
  ready. It is metadata only and the commentary already documents it.
- **WATCH and 2-player modes, and staff-credits display.** These are reachability and
  gameplay questions only. Every byte involved is already classified, and no unknown
  range depends on them. No caller of the credits text was found in rounds 1-2. The
  missing fact is a selector that reaches it, and there is no candidate writer to trace.
  The prospect for classification is nil, and broad gameplay already reached the ending.
- **The 16 caller-less Static routines.** Their classification already stands.
  Promoting them to Observed or proving them reachable needs an indirect selector, and
  none was found in the round-2 scans or in this redirect's reference scans. They are
  not claimed dead.
- **Exact generic table layouts.** These carry no classification gain. They need
  per-consumer extent proofs, and there is no concrete question to answer.

## Housekeeping

- Disk free is about 2.6 GB. The r2-a1 directory is about 110 KB. The scratch directory
  has been removed.
- No serve processes are left running.
- input/game.nes is unchanged (sha256 prefix e1c918223e882805), and analysis/state.json
  and maps/current.json are untouched.
