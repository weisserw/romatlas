# Round 3 redirect handoff (attempt r3-a1)

Input checkpoint: pass-003 (byte-identical; maps/current.json = maps/pass-003.json).
Classification is unchanged at 163,845 / 163,856 bytes, with 11 bytes unknown (L-017).
This redirect did not change the map, state, commentary, ROM or toolkit. It changed:
- analysis/leads.json (L-017 attempt and note appended; round-3 redirect pass record added);
- analysis/redirect/r3-a1/ (new). Its contents:
  - leads.before.json;
  - read-only tools: habit.py, refscan.py, ffret.py, copies.py, phfix.py;
  - their outputs: habit.txt, refscan.txt, ffret.txt, copies.txt.

**Recommendation: TERMINAL (incomplete).**
- No ready leads.
- 11 bytes stay unknown: phys 6173+1, 51444+3, 85771+5, 117501+1, 118162+1.
- The updated rejection review is section 2. It addresses coordinator points 1-3.

## 1. Findings

### Point 1: habit frequency, measured against the L-020 standard (habit.txt)

**Population.** These counts come from all classified code at pass-003.
- There are 209 backward (loop-closing) JMPs.
- In 28 of them, the loop never falls through to the next byte, meaning no branch or JMP
  inside the loop targets the byte after the JMP. That next byte is:
  - a referenced routine or label in 12 cases;
  - unreferenced classified code in 8 cases, mostly table-dispatched sound handlers in
    bank 12;
  - data or text in 5 cases;
  - one of the 3 unknown RTS bytes in the remaining 3 cases.
- The candidates have an exact shape: a loop with an inline RTS exit, like DelayFrames_Body,
  SquareRoot_Body and $CBC7. This shape occurs 23 times. In the 21 classified cases, a
  trailing RTS follows **0** times.
- Within bank 14 alone, 2 of 4 such loops have the trailing byte.
- Across all classified code there is **no** unreferenced RTS directly after any
  unconditional transfer (JMP, RTS or RTI).

**Conclusion.**
- The "dead trailing RTS" habit appears only in the candidates themselves, so using it as
  evidence would be circular.
- A narrower bank-14 claim (2 of 4) does not separate code from data.

**Symmetry with L-020.** L-020 was accepted on three independent facts:
- 460 classified instances of the IF/ELSE skeleton, 12 of them directly after another
  unconditional transfer;
- a 16-bit ENDIF operand that was predicted for each fragment and matched exactly;
- consistent nesting in the bank-10 pair.

A lone RTS has neither a classified precedent nor an operand prediction. Applying the same
standard therefore rejects it, so the two decisions are not asymmetric.

**The competing reading has no precedent either.** The only 3 short (5 bytes or fewer)
non-code gaps between code regions (49715+5, 66074+3, 68698+4) all have classified
consumers. No unread 1-byte separator exists anywhere. Both readings are unprecedented, so
the code-versus-data question stays materially ambiguous.

### Point 2: reference-form coverage (refscan.txt)

Each of these forms was scanned, for every instruction start inside each fragment (and
start-1 where an RTS-dispatch target applies):
- **Raw relative branches** from every byte position (any classification, including data
  and never-taken paths) in the fragment's 16K pair and the fixed bank: no hits.
- **Raw words and JMP/JSR/JMP-indirect operands** anywhere in PRG, limited to banks that can
  be mapped together with the fragment (MMC1 control $1F: 16K at $8000, last bank fixed).
  Every hit is coincidental:
  - seven hits are the two-byte zero-page instruction `SBC $88` (the bytes read as word
    $88E5);
  - phys 536 is the known straddle.
- **Split lo/hi tables**, anchored on consumers: 91 pairs of indexed loads within 48 bytes
  of each other, every index 0-255, both byte orders. The only hits fall outside each
  table's domain:
  - TourStageTextTable / TourRouteTable at index 108 or 109, where TourStage is 7 or less;
  - the DPCM table $C009 at index 205, where the table has 7 entries.
- **Immediate pairs** within 8 instructions, covering the PHA/PHA RTS trick (as used by
  TryShotType), FarCall target setups and ZP pointer setups: no hits.
- **JMP-indirect sites** (8) were already covered in r2-a1. They are filled from classified
  tables or symbolic immediates.

**Correction to r2-a1 tooling.** `frag.ph()` dropped references between the two 8K halves
of one 16K PRG bank. For example, it missed a reference from bank 7 to a bank-6 $8xxx
address.
- The raw word and branch scans did not use it, so the earlier "no reference" result
  stands.
- The counts were recomputed with phfix.py and are unchanged. The only difference is that
  RackOver now shows its 10 references.

### Point 3: conventions and leftover copies (ffret.txt, copies.txt)

**51444 (LDA #$FF / RTS)**
- $FF is this ROM's success return: "shot found" in TryShotType and the ShotType*
  routines, and "rack over" in RackOver.
- CpuChooseShot has one caller: CpuPlanShot tail-calls it as a CpuThinkTable phase handler,
  reached through Far_CpuThink. At none of the three Far_CpuThink FarCall sites does the
  code test A afterwards. So no caller constrains the return value.
- The sibling CpuChooseShotEightBall returns $FF through TryShotType and has no equivalent
  tail.
- In classified code, LDA #$FF / RTS follows a JMP or RTS 8 times, and all 8 are
  referenced.
- No byte-identical copy longer than 4 bytes exists anywhere in PRG.
- An "orphaned success exit from an earlier version" is a plausible story, but nothing in
  it predicts these exact bytes.

**85771 (LDA #$80 / STA RoundState / RTS)**
- It resembles RackOver at bank 11 $A0AE, which is in the same 16K PRG bank 5. RackOver is
  LDA #$80 / STA RoundState / LDA #$FF / RTS and has 10 references.
- It is not a byte copy: the 5-byte sequence occurs nowhere else.
- No reference or slot predicts it.

**Leftover-copy standard.** The map's leftover-copy precedent is regions 994/995: a
contiguous, byte-identical copy of bank 5 in bank 7, ending exactly where the source code
ends. Under that standard:
- 6173, 117501 and 118162 match elsewhere only within identical windows of 6 bytes or fewer, and every such window starts at the fragment itself;
- 51444 shares 4 bytes at most;
- 85771 shares none.

None of the fragments qualifies as leftover-copied data.

## 2. Updated compact rejection review (L-017, 11 bytes)

The common claim for every fragment is: Code (an instruction tail of the neighbouring
routine), with exact bounds equal to the gap, versus non-code.

| Fragment | Decisive evidence | What it rules out | Independent constraints considered together | Strongest competitor | Result |
|---|---|---|---|---|---|
| 6173+1, bank 0 $980D (RTS after EndingSequence's endless JMP) | habit.txt: 0 of 25 classified never-falling loops have a dead RTS; refscan.txt has no reference | Code via an epilogue habit (no independent precedent); code via any reference form | The bounded gap and the decode only. These restate the byte. | A 1-byte unread separator. This is also unprecedented. | Insufficient. Stays unknown. |
| 117501+1 and 118162+1, bank 14 (RTS after the DelayFrames_Body and SquareRoot_Body loops) | Same habit result. Same-shape classified sites: 0 of 21 with RTS. The same-bank counterexample $CBC7 stands. | Same as above | Same as above. The narrower bank-14 rate (2 of 4) is not discriminating. | Same as above | Insufficient |
| 51444+3, bank 6 $88E4 (LDA #$FF / RTS after CpuChooseShot's tail JMP) | ffret.txt: all 8 classified post-transfer occurrences are referenced. No caller consumes A. No copy. | Code via an idiom precedent; code via a caller convention; leftover-copy data | The $FF success convention exists, but it is a property of the byte pattern and is not tied to this position. | A 3-byte unread constant | Insufficient |
| 85771+5, bank 10 $8EFB (LDA #$80 / STA RoundState / RTS before a string header) | ffret.txt and copies.txt: similar to RackOver but not a copy; no reference | Leftover copy; code via a reference | The RoundState $80 idiom is the byte pattern again. The string header at 85776 starts its own record. | String-area residue | Insufficient |

**Why no revised test remains.**
- Every candidate generator rule has now been measured and found absent in classified code:
  - an epilogue rule or habit;
  - a caller return convention;
  - a byte-identical leftover copy;
  - an IF/ELSE slot.
- Every reference form came back negative:
  - branches, including from non-code bytes;
  - absolute and indirect operands;
  - words and RTS-dispatch targets (address-1);
  - split tables;
  - immediate pairs and FarCall setups.
- Forcing execution in the emulator would only show the mechanics, not intended code.
- Classifying any fragment would therefore need a premise the evidence does not supply.
  This review reuses the r2-a1 review, whose premises were corrected where noted (the
  `ph()` scope).

## 3. Other avenues (deferred, with reasons)

These are reachability questions only, and every byte involved is already classified.

- **WATCH and 2-player modes (SC-010).** None of the 11 unknown bytes has an incoming edge
  of any form, so play in these modes cannot execute them. Observing these modes could only
  upgrade Static to Observed elsewhere. They stay deferred with no classification prospect.
- **Staff-credits display.** No reader of the credits text exists in any reference form
  scanned in rounds 2-3. There is no candidate writer to trace.
- **The 16 caller-less Static routines.** The round-3 split-table and immediate-pair scans
  add no selector for them. Their classification stands. They are not claimed dead.

## Housekeeping

- Disk free is about 2.5 GB, and r3-a1 is under 100 KB.
- No emulator or serve processes were started, and no scratch was left behind
  (`__pycache__` was removed).
- input/game.nes, maps/current.json, analysis/state.json, the commentary and
  analysis/progress.md were not modified.
