# Round 3 disassembly handoff

- Input pass-002 -> output checkpoint pass-003 (validate ok, generate/verify byte_identical true;
  analysis/pass-003-verification.json). State analysis/state.json saved; maps/current.json exported.
- ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown).
- L017 (status verified, split): applied 0x08743, 0x0B1BE, 0x0B7A2 (RTS inside an enclosing branch span of the same
  routine) and 0x1C1E2-0x1C1E4 (JMP $FBCC reset entry, exact gap) as Code/Static 0.6, no label, reachability not claimed
  (regions 1903-1906). Rejected, kept unknown: 0x14CCA (b10 $8CBA), 0x17BC6 (b11 $BBB6), 0x1FD5A (b15 $FD4A) - lone
  unreferenced RTS with no enclosing structure/twin/consumer. Evidence: analysis/tools-round3/l017-span.json,
  l017-regions.json; analysis/redirect/r02-a/evidence/l017-refs.json, l017-reads.json. Details: analysis/investigation.md.
- L018 blocked (unchanged); L019 ready for commentary (no map change). No gameplay run (no concrete gain).
- Commentary: 4 new tiny regions (1903-1906) need carry-forward/comments ("unreferenced instruction after JMP; reachability
  not claimed"); L019 facts.
- Terminal residual: the 3 rejected bytes; do not retry without new evidence (reference, fetch, consumer, twin).
- No server left running. Toolkit: no new toolkit issues found (analysis/toolkit-issues.md).
