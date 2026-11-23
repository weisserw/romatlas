# Round 3 disassembly handoff (pass-003)

- Scope: one pass applying ready lead L009 (static; reused r02-a runtime evidence). No gameplay (S01-S12
  already reach the ending). L002 blocked (toolkit), L005 semantics-only - not retried.
- Input: pass-002 / analysis/state.json (pre-pass copy scratch/state-before-pass003.json).
- Checkpoint: pass-003 (maps/pass-003.json == maps/current.json, generated/pass-003).
- Verification: analysis/pass-003-verification.json ok true, byte_identical true, map_sha256 08865b21...76a3.
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). Code 35307 / Data 95781.

## Exact region changes vs pass-002 (for commentary carry-over)
Removed (all Data, unlabeled): 1:$D600-$D62F (phys 54800+48), 1:$D630-$D648 (54848+25),
1:$D649-$D66F (54873+39), 1:$D670-$D67F (54912+16), 3:$D76E-$D79E (120702+49), 3:$D79F-$D7B7 (120751+25),
3:$D7B8-$D7DE (120776+39), 3:$D7DF-$D7EE (120815+16).
Added:
- Data 1:$D600-$D67F phys 54800+128, label BlockSubstTables: 8 records x 16 B indexed by Mission (0-7),
  copied to BlockSubst $05E1 by LoadBlockSubst 1:$C73A; records 4-7 == 0-3; records 3,7 all zero.
- Code 3:$D76E phys 120702+1 (unlabeled): lone rts after GroundUnderAt rts $D76D; Static conf 0.55,
  L007-consistent weak decision.
- Data 3:$D76F-$D7EE phys 120703+128, label BlockSubstTablesCopyB3: byte-identical unread copy of
  the bank-1 table; ends before UpdateGroundObjects $D7EF.
Commentary: replace DataB1_D630/D649/D670 and DataB3_D76E/D79F/D7B8/D7DF notes; remove 1:$D649,
3:$D7B8, 3:$D76E from uncertainties item 6 "no consumer" list (bank-3 copy remains "no reader found");
also pick up S11/S12 points from round-02-redirect.md.

## Leads
- L009 verified (analysis/leads.json). L002 blocked; L005 open (semantics only). No other ready leads.
- Evidence: analysis/evidence/round-03/ (inspect_l009.txt, leads-before.json, tools/inspect_l009.py,
  tools/apply_l009.py).
- Toolkit issues unchanged (TK-001, TK-002); no new toolkit issues found.
