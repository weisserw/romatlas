# r3-redirect-01 notes (complete)
Input pass-005 (407f32c4...). Handoff analysis/handoffs/round3-redirect.md.
- L-R2-04: corrected premise. TK-003 only showed a 1-byte Code region is rejected (truncated). Extending Code
  region 3514 back to $A07A (view $A07A: bcc/rts/jmp...) validates, generates and verifies byte-identical
  (check-output.txt). Alternate $A07B instruction start documented in evidence/commentary. Ready.
- L-R3-01 split: $9026-$9028 -> L-R5-01 ready (state-0/body pairing constraint, new); $9B08 stays Unknown.
- deadafter.py: no unreferenced instruction after an unconditional transfer inside contiguous classified code.
- FIRE trace for $B820 skipped: no reference/indirect jump to $B820 in PRG (r2 entries-output; no jmp() in bank 7);
  reachability-only, already classified.
- Scratch (optA/optB/optC maps, generated output) removed after recording results. No server started.
