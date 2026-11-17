# Round 4 disassembly handoff (pass-004)

ROM classification: 100.00% (393224 / 393232 bytes classified; 8 bytes unknown). Status: incomplete (8 unknown bytes
remain even though the percentage rounds to 100.00%).

## Checkpoint
- maps/pass-004.json -> generated/pass-004.
- analysis/pass-004-verification.json: ok, byte_identical.
- Map sha256: 077d58cd3516eacb079e709c2c6466478f5edea3cdb5afa4d9efbe6213faf837. It equals analysis/state.json and
  maps/current.json (4,107 regions).
- The ROM sha is unchanged (7a2a1f6e...).
- The pre-edit store copy is scratch/p4/state.pass-003.json.
- One stdio serve session was used. No server is left running.

## Applied (r3-redirect-a batch, reviewed independently in scratch/p4/plan.py; plan in scratch/p4/plan.json)
| Item | Lead | Range | Change | Notes |
|---|---|---|---|---|
| C01 | L021 | 0x03C0D0+5, b30 $C0C0 | unknown -> Code/Static 0.65 | label LoadChrBankTemp. Start bounded by RandomTable (NextRandom $C5D4, AND #$3F). End at classified $C0C5. Sibling LDA zp/STA zp entries at $C0C5 and $C0FD |
| C02 | L021 | 0x03F6D2+12, b31 $F6C2 | unknown -> Code/Static 0.6 | label AddRecordStrideTemp0C. Lies between consumer-bounded table $F6B6 and its duplicate D63. Stride 18 = record loop CMP #$12 at $F6A5. Defective high-byte add |
| F01 | L025 | 0x004C62+2, 0x004CB0+2, b2 $8C52/$8CA0 | Code -> Data (inline words $2204 / $00BA) | region 4006 (0x004C3A+245) split 40/2/76/2/125; contracts of $C6D6/$C6FC re-read |
- Reachability of C01 and C02: no known caller, 0 observed starts. Not claimed unused.
- Observation merge: there are 21 Static regions with r3 S26-S27 natural-input starts (r3-observed.json); all starts were
  re-checked against the static decode.
  - 11 fully observed regions became Observed (318 bytes).
  - 10 partly observed regions stay Static, with an observation note.

## Residue (analysis/pass-004-remaining.json)
- The 4 runs are b2 0x0040EA+2 ($80DA), 0x005998+4 ($9988) and 0x005B58+1 ($9B48), plus b9 0x012175+1 ($A165).
- The rejection review in analysis/handoffs/round-003-redirect.md was reused. Its premises are unchanged: no reference,
  no discriminating consumer, sibling or domain, and 0 fetches in S1-S27.
- Strongest competing reading: padding or dead code. Classifying it would need a new reference, an observed fetch or a
  discriminating structure.

## Leads
- Consumed / verified: L025.
- L021 is blocked. Its 17 bytes were applied; the 8-byte residue stays as above.
- L006 is verified, with the merge note added.
- Produced: none.

## Gameplay
Not run. S1-S27 cover every menu milestone and both league-champion endings, with 0 residue fetches, so there is no
substantial prospect.

## Toolkit
No new toolkit issues found (analysis/toolkit-issues.md: TI-001 to TI-003).

## Next
- Commentary needs to cover:
  - the new regions C01 and C02 (labels LoadChrBankTemp and AddRecordStrideTemp0C);
  - the F01 split in bank 2 PitcherListDrawAlt, whose inline words are now `.incbin` Data;
  - the 21 regions whose provenance or evidence changed.
- After that comes the redirect or coordinator decision on the terminal 8-byte residue.
