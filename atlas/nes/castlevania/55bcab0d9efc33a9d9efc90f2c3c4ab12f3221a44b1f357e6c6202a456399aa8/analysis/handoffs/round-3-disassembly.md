# Round 3 disassembly handoff

- Status: **COMPLETE for classification** (0 bytes unknown); not a final acceptance: commentary
  naming of the new stubs and the editorial audit remain. Input checkpoint pass-002 (state restored
  from analysis/state.json via map.load; all existing classifications preserved; baseline untouched).
- Scope: the single ready lead L-015 (6 residual bytes). No gameplay this pass (all milestones in
  analysis/scenarios.json already visited; replay would yield no structural evidence for these bytes).
- Output checkpoint **pass-003**: `python3 run.py checkpoint pass-003` -> ok, byte_identical true
  ([analysis/pass-003-verification.json](../pass-003-verification.json), maps/pass-003.json = maps/current.json,
  map sha256 d9f5dde8c6f8e3e29c7d685a2423470235f0a3d06ef6dc44dc44abdf38d1f3cc). map.validate ok.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| 8K bank | classified | unknown |
|---|---|---|
| header | 16 | 0 |
| 0-15 | 8192 each | 0 |

Kinds: Code 30188 (Observed 12483 / Static 17705), Data 84627, Padding 16273; 772 regions, no overlaps.

## L-015 outcome: verified (accepted, Static, confidence 0.5, reachability unknown)

| Phys | CPU (context) | Bytes | Preceded by | Followed by |
|---|---|---|---|---|
| 115733 | $C405 ([-,-,14,15]) | rts | jmp $C360 | instr $C406 (Static) |
| 117030 | $C916 ([-,-,14,15]) | rts | rts $C915 | instr $C917 (Observed) |
| 117355 | $CA5B ([-,-,14,15]) | rts | jmp $C156 | instr $CA5C (Static) |
| 105371-105372 | 6:$9B8B-$9B8C ([12,13,14,15]) | sec; rts | clc;rts $9B89 | data $9B8D (absolute operands lda $9B8D/$9B8E) |
| 106996 | 6:$A1E4 ([12,13,14,15]) | rts | jmp $CC10 | instr $A1E5 (Observed) |

Independent review (evidence-policy.md), reasoning:
- Re-checked the listings and neighbouring regions myself: each interval is bounded exactly by an
  unconditional transfer that ends classified code and an independently established instruction start
  (or, for $9B8B, a data table whose start is fixed by absolute operands at $9B8D, so the table cannot
  absorb $9B8B-$9B8C). Bounds are therefore not ambiguous.
- Data alternative has no support: refs.py census (refs_out.txt) finds no branch/JSR/JMP target, no
  absolute operand, and only indexed windows whose bases are earlier bounded tables that would read across
  Observed code. Word-1 values matching $A1E3 exist only in 8K bank 0/1 data, and the ROM has no
  RTS-trick (pha/pha/rts) dispatcher (indirect.py), so they are not consumed as pointers. Not fill
  (all Padding is $FF; these are $60/$38).
- Code role is supported by more than the opcode value: placement inside contiguous code streams,
  in-ROM precedent of identical return-only stubs ($E3B2, $FAFE ReturnOnly), the bank-6 paired
  clc;rts/sec;rts carry-exit idiom, and the jsr X;rts -> jmp X / per-sub-state rts templates.
  The strongest competitor (stray non-code bytes) has no consumer, format or fill convention.
- Re-ran refs.py on the new map (scratch/r3/refs_after.txt): identical empty reference sets
  (only the classified-instruction count changed 14047 -> 14053).
- Recorded separately in each region's evidence: structural evidence; "Reachability: unknown (no known
  caller, never observed executed); not claimed dead or Observed". Provenance Static, confidence 0.5,
  comment "Unreferenced return stub; reachability unknown". Applied by scratch/r3/apply.py.

## Remaining questions

- Commentary: name/describe the five stubs (e.g. UnusedReturnStub_xxxx) with the low-confidence note;
  they currently show only the internal comment.
- Reachability still unknown for L-015 stubs, L-003 fragments and unreferenced routines ($90A1, $910A,
  $CF61, $FF06); none blocks classification.
- Semantic-only leads L-004/L-005/L-006, L-008 (toolkit-blocked TK-NES-004), Q3 spawner list lengths.
- Toolkit: no new issues this pass ([analysis/toolkit-issues.md](../toolkit-issues.md) unchanged).
- Servers: only a private `run.py serve` child of apply.py (exited); none left running.
