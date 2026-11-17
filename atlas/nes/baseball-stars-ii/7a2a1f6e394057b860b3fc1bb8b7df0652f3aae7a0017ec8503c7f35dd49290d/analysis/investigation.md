# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 / disassembly pass-001 (2026-11-15)

Checkpoint: maps/pass-001.json (verified byte-identical, analysis/pass-001-verification.json).

Structural model used (scratch/flow.py, verified against observed contexts):
- MMC3 PRG mode 0. $C16A (`asl a`, R6=2A, R7=2A+1) maps an even/odd pair at $8000/$A000; $C000=bank 30,
  $E000=bank 31 fixed. Every observed switchable-bank context is a pair (2k, 2k+1) except transient mixed
  states observed inside $C16A itself and NMI arriving mid-switch (6 hits each).
- $C316: inline jump table after JSR (pulls return address, indexes by A*2, JMP ($35)). Table extents taken up
  to the next code instruction/entry target or first non-PRG word; 14 tables checked by hand (targets that land
  right after their table, following data read by `lda $845E,y`, etc.).
- $C32D: far call (A = pair index, Y = index into $FEB4 word table, JMP ($35)); inline byte counts per Y learned
  from observed returns (Y=4 and Y=8 take 6 inline bytes; others 0).
- $C6D6/$C6FC take 2 inline bytes; $C271 takes an inline text record (2-byte header, bytes to $FF, $FE = new header).
- Routines flagged as stack-manipulating ($FC10, bank 9 $A3A8) checked: only PHA/PLA save/restore, fall-through valid.
- JSR $C16A from banked code: only re-selects the routine's own pair (A=6 in bank 12/13) or is a tail JMP; no
  static context error.

Data classification (scratch/dataregions.py): generic Data only where a linear sweep hits >=2 unofficial/BRK/KIL
encodings at most 16 bytes apart, or uniform runs >=16 bytes (not $EA); segment tails trimmed when code that
runs cleanly to RTS/RTI/JMP could start within the last 8 bytes (example: bank 30 $D17C text ends before
`20 D8 C5 A9 00 85 32 60`). Code-looking islands, monotonic tables that decode as opcodes and text runs stay unknown.

Unknown remainder (31,853 bytes): code-like islands without a known entry (lead L001 bank 17 $BCAD-$BFFF 851
bytes; L002 fixed-bank islands), opcode-valid tables/text (L004), residual islands in data banks (L005).
Table $FEB4 extent unresolved (L003). See analysis/leads.json.

## Round 2 / disassembly pass-002 (2026-11-16)

Checkpoint maps/pass-002.json (verified byte-identical). ROM classification 99.56% (391,505 / 393,232; 1,727 unknown).

Batch review (scripts scratch/p2/chk1-4.py, plan.py, plan2.py, apply.py; plan files scratch/p2/plan.json, plan2.json):
- Reproducibility: r1-redirect-a write_props.py re-run on a copy reproduces proposals.json byte-identically from pass-001.
- A-code (L015/L014): closure has 0 bad paths (BRK/unofficial), 0 self-overlaps, 0 instructions partially overlapping
  existing code. Code runs split by provenance: starts fetched in r1-redirect-a traces (r1-observed.json) -> Observed
  (evidence notes artificial scenarios S15-S19), else Static. Each region's evidence names its closure roots (seed reason).
- L014: 15 phase-1 generic data regions removed and re-covered exactly: new code + inline params + JT words + Data
  remnants (bank 14 $9BDD-$9FFF 1,059 bytes, bank 24 $9479 24, bank 9 $B2A0 15 "TEAM" text, bank 2 $9D2B 1).
  Three inline jump tables ($9D75, $9DAE bank 2; $A189 bank 3) partially lie in phase-1 data; only unknown words added.
- Data groups (B/D/E/F/G/I, C tables/fills): 0 r1-observed starts inside; sampled I-islands (bank 0/1 sound/pointer data)
  and E/G text consistent with stated rationale. Accepted as Data/Static.
- Copies (L017/H): byte identity re-checked (live banks 23/27/29 differ from bank 5 only at operand $BB99; stale banks
  15/17/19/21/25 and bank 11 vs 13 identical) and instruction alignment re-checked by linear decode ending exactly at
  each range end. Stale copies: Code/Static, conf 0.8, reachability none known (alternative generic data noted).
- L022: 4 islands accepted Code/Static (closure exact fit re-probed); $9F73-$9F82 kept Data.
- Rejected/narrowed: none (A-jt partial tables narrowed to unknown words only).

Static supplement on L021 residue (Static, evidence prefix "[pass-002 static review of L021 residue]"):
- Data: bank 3 $ACD3 text; bank 24 $82B7 text-record head, $8C57/$88AC/$95BB coordinate pairs, $83C7-$83C8 and
  $83D6-$83EA; bank 9 $B929 tile pattern, $B762 sprite records; bank 25 $AEAD, $AD8C-$ADB2 (records addressed by
  pointer table at $AD7C), $AE0F-$AE1D text; bank 31 $EBB1; bank 11 $AC54 word table head; bank 1 $A3B4-$A439
  (pointer-table tail + 8x16-byte records, extent ends at code $A43A).
- Code (no known entry, reachability unknown): bank 24 $83B9-$83C6 (OAM copy of table $83C7), bank 25 $ADB3-$AE0E
  (text/number print; inline $C6FC bytes $ADE6-$ADE7 as Data). Closures fit exactly, official opcodes only.
- Left unknown (code-vs-data or bounds ambiguous, no entry): see analysis/pass-002-remaining.json, lead L021.

Gameplay (pass-002, traced, configured binary): S21 '?' speciality team; S22 2P + ERROR ON into fielding.
No fetch in residual unknown bytes. VETERAN acquisition / >2-team league deferred (shared code, low prospect).


## Pass-003 (round 3 disassembly, 2026-11-16)

Checkpoint maps/pass-003.json (verified byte-identical). ROM classification 99.99% (393,207 / 393,232; 25 unknown).

Batch review of r2-redirect-a (scripts scratch/p3/review.py, plan.py, apply.py; plan scratch/p3/plan.json):
- Coverage: the 116 items plus the 25-byte residue equal the pass-002 unknown set; the 23 reclassified bytes lie in Data.
- Code items: official opcodes, aligned ends, all control-flow targets are instruction starts; non-terminal ends only at
  $C6D6/$C6FC/$C32D(Y=8) inline contracts followed by the matching inline Data. Data items: no code reference into them.
- Corrected premise (r2 key fact 1): phase-1 region 0x03203B+21 was not one table; $A033 is a FarCall sequence identical to
  bank 24 $9803-$9812. Region split into table $A02B-$A032, Code $A033-$A03C, inline record $A03D-$A042.
- Region 0x03389C+22 split: JSR $C6FC at $B88B (Code 0x03389B+3), inline $B88E-$B88F, Code $B890-$B89A, table $B89B-$B8A1.
- Weak items accepted at reduced confidence (see handoff). D62 unlabeled because TeamBlockDstPtrs starts at $CF50.
- Structural claims Static; reachability not observed for all new code (not claimed unused).
- Residue (6 runs, 25 bytes): rejection review in analysis/handoffs/round-003-disassembly.md; kept unknown (lone RTS bytes,
  effectless/dead-load fragments, no references, no classified copies).

## Pass-004 (round 4 disassembly, 2026-11-16)

Checkpoint maps/pass-004.json (sha256 077d58cd...), verified byte-identical. ROM classification 100.00% rounded
(393,224 / 393,232; 8 unknown). Incomplete.

Independent review of r3-redirect-a proposals (scratch/p4/plan.py, apply.py; plan scratch/p4/plan.json; pre-edit store
scratch/p4/state.pass-003.json):
- C01 b30 $C0C0-$C0C4 (LDA $22/STA $22/RTS): accepted as Code/Static, conf 0.65, label LoadChrBankTemp. Re-checked from
  the ROM: NextRandom at $C5D4 (AND #$3F; LDA $C080,X) bounds RandomTable to $C080-$C0BF, all 64 values 0..7. $C0C5 a5 23 85 23
  and $C0FD a5 24 85 24 are sibling entries. `a5 22 85 22` occurs nowhere else in the ROM. No data reading fits.
- C02 b31 $F6C2-$F6CD (Temp0C/0D += 18, buggy high byte): accepted as Code/Static, conf 0.6, label AddRecordStrideTemp0C.
  Re-checked from the ROM: $F646-$F6A9 reads $F6AA/$F6B6,X with X = Temp01*2 (Temp01 < 6), and its loop writes 18 bytes
  per record (CMP #$12 at $F6A5). The table words step by 18. The data reading gives words in no adjacent domain. The
  consumer routine itself uses pointer $16/$17, not $0C/$0D, so the stride link is semantic support, not a caller.
- F01 b2 $8C52/$8CA0: accepted. The callee bytes at $C6D6 and $C6FC were re-read: each reads ret+1/ret+2 and resumes at
  ret+3 via JMP ($0006). Region 4006 was replaced by Code 40 / Data 2 / Code 76 / Data 2 / Code 125, with the original
  evidence and confidence kept on the code pieces. The generated source now emits `.incbin` for both words.
- Observation merge (r3-redirect-a/r3-observed.json, natural-input S26-S27): there are 21 Static regions. All observed
  starts lie on the statically decoded starts (re-checked).
  - 11 fully observed regions became Observed (conf 1.0, 318 bytes).
  - 10 partly observed regions keep Static provenance with an "N of M starts observed" note.
- Residual 8 bytes: the round-3 redirect rejection review was reused, since its premises are unchanged. The bytes are
  b2 $80DA-$80DB, $9988-$998B and $9B48, and b9 $A165. L021 is blocked.
