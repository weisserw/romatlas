# Round 3 disassembly - handoff (pass 3)

## Checkpoint
pass-003: maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json (ok, byte_identical,
map sha256 a47d91b2...). analysis/state.json saved, maps/current.json exported, map.validate ok, ROM unchanged.

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

## Changes
- L-015 verified: phys 0x1F63C / fixed $F62C (ctx [null,null,14,15]) -> Code, Static, conf 0.6, region 1811
  label PlayerBump_DeadRts, comment "unreachable epilogue remnant; reachability not claimed". Review reasoning
  in analysis/investigation.md (dead clc + carry contract + negative data checks; filler rejected as weaker).
- L-016 verified: 230 B in 14 code regions Static -> Observed (natural 2-player route evidence,
  analysis/evidence/pass-003/l016-check.txt). Partial regions 925,1192,1367,1370,1422,1449 not split.
- Tools: analysis/evidence/pass-003/tools/ (apply.py, check.py, cli.py).

## Metrics
Code Observed 13864, Code Static 17520, Data 229659, Padding 1117; 1031 regions all with evidence.
Execution coverage 44.2% of code bytes Observed. Per-bank table in analysis/progress.md.

## Next
- Commentary touch-up: replace the old "Unknown" comment block at $F62C with the PlayerBump_DeadRts
  explanation (code, reachability not claimed); mark L-016 regions as Observed in commentary.
- Optional: split partial 2P-fetched regions for finer Observed provenance (no classification effect).
- Final editorial audit/verification. No redirect avenues remain for classification.
- Toolkit: no new toolkit issues found; TK-001..TK-004 in analysis/toolkit-issues.md.
