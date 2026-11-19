# Investigation ledger

## Round 1 disassembly (pass-001)

Method:
- Persistent `run.py serve` via a local socket driver (analysis/tools/round-001/daemon.py, rpc.py;
  paths inside the scripts point to the round-1 scratch dir and need editing to reuse).
- Observed code: actual opcode fetches with effective PRG contexts from `analysis.contexts`
  (dumps: traces/round-001/ctx_a/ctx_b/ctx_f.json.gz) over scenarios S-001..S-007.
- Static extension (classify.py): from each observed/vector instruction, bounded 6502 flow closure
  in the same effective bank context (switch slot bank + fixed 30/31). JSR assumed to return except
  to detected jump engines ($F292, $ED2C in bank 31; $86B5 in bank 16). A closure is accepted only if
  every instruction uses an official opcode, no BRK/KIL, no overlap with observed or own
  instructions, no cross-window operands; otherwise the whole closure is rejected (4 rejected,
  all after branch-always idioms; lead L-004). Toolkit discovery was not usable (TK-002).
- Every code region was revalidated by map.annotate/map.validate decoding with explicit views.

Results (internal map, 839 regions): Code 33,004 bytes (26,459 Observed, 6,545 Static),
Data 141,575, Padding 65,575. Unknown 153,078.

Data classifications and evidence:
- iNES header (16), CHR ROM 16 x 8 KiB (header-defined, PPU-only).
- DPCM samples $C000-$E7FF (bank 30 entire + bank 31 $E000-$E7FF): bounds from the 10-record
  DMC sample table $F5BD (addr word, length L; DMC length L*16+1) consumed by $F552-$F59C which
  feeds $4012/$4013 at $F529-$F541; the records tile $C000-$E800 contiguously. Tables $F59D
  (32-entry sound->DMC map, bounded by cmp #$20) and $F5BD (10 x 4) classified.
- Warm-boot signature $FF5D (16, compared with $68-$77), CPU vectors $FFFA.
- Even banks 0-28: byte 0 equals the bank index (BankIdNN, 15 bytes); read relative to $8001 by far-call helper.
- 11 inline jump tables after jump-engine JSRs whose extent ends at known code and whose every
  entry is known code (120 bytes). 25 other tables left unknown (L-001).
- Padding: $FF runs >= 64 bytes with no classified code inside (banks 10,11,13,15 entirely; tails
  of others; bank 31 $FB9B-$FEFF). Start boundary uncertainty noted in each evidence string.

Not classified / open: 25 jump tables and their unexecuted handlers; banks 0,4,6 and data
parts of 1-9,17-19,24-29; bank 31 remaining 561 bytes; rejected closures. See analysis/leads.json.

Lead outcomes: no input leads existed (none consumed). Produced L-001..L-006.

## Round 2 disassembly (pass-002)

Method: redirect r001-a pipeline re-run on pass-001 (reproduced candidates.json byte-for-byte),
plans built offline (analysis/tools/round-002/plan1-3.py) and applied via map.annotate; all Code
decode-checked; Data checked against all static control-flow targets and all recorded coverage.
Results: L-007/L-011 tables and header; L-008 static closure (9,222 code + 676 tables); L-009 orphan
routines (+4 new); L-010 bank 0/4 copies; L-012 field data; L-013 generic data split at consumer
anchors; L-014 bank 6/7 older bank-16 variant. Gameplay: ending (S-012, level contract $66/$03FB,
bank 23 $B386) and 2P (S-011). HomeTeam 6->5 = team-select editor (bank 21 $A818), not progression.
ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown).
Remaining unknown: 3 lone unreferenced duplicate RTS bytes (policy). Details: analysis/handoffs/round-002-disassembly.md.

## Round 3 disassembly (pass-003)

- L-017 verified and applied: phys 193554 (23:$B402, ctx [22,23,30,31]) Code/Static, new region 1251.
  Structural evidence: toolkit analysis.decode of `bcc $B403` at $B400 gives successors [$B402,$B403];
  $60 decodes as terminal `rts`; static-only analysis.discover from NextLevelFrom4 ($B3E9) emits the
  1-byte candidate with no diagnostics; exact bounds between Code 955 and 956; no reader/pointer.
  Reachability (separate): statically dead fall-through (C=0 on the only path), never fetched.
- L-015 (28:$8620, 28:$9883) not retried: no new evidence; rejection review stands. L-016/L-006 deferred.
ROM classification: 100.00% (393230 / 393232 bytes classified; 2 bytes unknown).
Remaining unknown: physical 230960 (28:$8620) and 235667 (28:$9883). Details: analysis/handoffs/round-003-disassembly.md.
