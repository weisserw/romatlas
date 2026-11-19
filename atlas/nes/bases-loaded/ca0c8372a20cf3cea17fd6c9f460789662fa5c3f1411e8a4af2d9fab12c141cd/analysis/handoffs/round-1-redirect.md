# Round 1 — redirect phase handoff (attempt r1-a1, 2026-11-19)

## Scope and checkpoint
- Input: checkpoint pass-001 (byte_identical true), commentary handoff round-1-commentary.md,
  leads L-001..L-014. ROM classification at input: 95.56% (313,139 / 327,696; 14,557 unknown in
  185 ranges — exact ledger analysis/redirect/r1-a1/unknown-ledger-input.json).
- Redirect changed research/lead records only: maps/, analysis/state.json, commentary/, input/ and
  the configured binary are untouched. Research: analysis/redirect/r1-a1/ (notes.md = findings F1-F9,
  proposals.json = exact ranges per lead, residual-after-all-proposals.json, probe/linear/resview tools,
  experiments/e1-e7.py + logs, shots/, obs-r1a1.json + obs-eval.json).

## Key findings
1. FarCall fall-through (F1): the round-1 static traversal stopped after every call reaching FarCall
   ($817D), but FarCall restores the caller's banks before RTS. Correcting this, plus the bounded
   AtBatStateTable and called $FF00 vector entries, reaches 5,088 unknown bytes as code with no
   invalid/overlap/barrier conflict (flags identical to the uncorrected run).
2. AtBatStateTable = $B199-$B298 (128 words, bounded by first handler $B299; unused -> RTS $B198);
   new handlers incl. $C000 (state $62 change sides / end-of-game test).
3. L-002 "data with spurious islands" was wrong for bank 2 $9551: it is a vector-called screen loader
   plus attribute/nametable/tile images. Bank 1 $E8EF+1472 = CPU pitch-selection records; 448-byte
   sound tables in bank 2 and banks 4-7.
4. Heuristic generic-data rule (L-003): one counterexample — bank 3 $B7B2-$B83B are 15-byte code
   stubs called through AND #3 table $A057 (Data->Code, 138 bytes). No other heuristic block is
   entered by reachable control flow.
5. RAM contract: $0418 is the bottom-half (home) total, $0426 the top-half total (commentary labels
   swapped, L-025); PennantCounterA $0766 = games (season ends at $84 = 132), PennantCounterB
   $0767 = wins (ending requires >= $50). Unreferenced bank-3 debug routines confirm the shortcuts.

## Ready leads for the next disassembly pass (exact ranges: analysis/redirect/r1-a1/proposals.json)
| Lead | Kind | New bytes | Recipe summary |
|---|---|---|---|
| L-015 | code | 5,088 (+138 Data->Code) | probe.py with FarCall-as-balanced-call rule from all code roots + proposal-extra-roots.txt; 117 runs; 352 starts observed (artificial E1-E7) |
| L-016 | data | 250 | AtBatStateTable $031A9-$032A8 as one 128-word table |
| L-017 | data | 2,252 | bank 2 attr 64 / nametables 2x1024 / tile block 140 (code $9551 in L-015) |
| L-018 | data | 1,472 | bank 1 $E8EF-$EEAE 8-byte records, bases from $E828-$E8B8 |
| L-019 | data | 2,240 | sound tables bank 2 $ABCE and banks 4-7 $A4AC (448 each) |
| L-020 | code | 140 | banks 4-7 $A000-$A022 copies of bank-0 $9E23 helpers (skip trick) |
| L-009 | code | 70 | $8173 EnterPitchingMode stub (live in bank 1; copies in 0,2-7) |
| L-021 | code | 28 | NMI busy-path continuation after same-bank SetPrgBank ($85D5, $8389 x3) |
| L-022 | code | 15 | $B920 4-entry table, handlers $B931/$B93A |
| L-023 | data | 541 | 21 consumer-anchored small tables incl. metasprite $AC8E+113 |
| L-025 | commentary | — | swap HomeScore/VisitorScore symbols |
Total proposed: 12,096 unknown bytes -> projected unknown 2,461 (99.25% if all accepted).

## Rejected / superseded / blocked
- Superseded by the above: L-001, L-002, L-010, L-011 (history appended). L-003 reviewed (see 4).
- L-024 blocked (2,461 bytes / 50 ranges, mostly bank 3): clean RTS/JMP-terminated routines with valid
  callees but no entry — no JSR/JMP/branch from an instruction start, no immediate pointer pair, no
  LE word or RTS-dispatch (addr-1) pair in data, no unresolved indirect jump whose table can reach them
  (all 45 indirect sites have resolved tables or immediate-only vectors). Several follow an RTS that
  live callers target (disabled debug hooks, e.g. $9DEE/$9E47/$9E5B/$9FD3). Runtime coverage of game
  end, season end and ending (E1-E7) hit 0 residual bytes. Missing fact: an entry; acceptance would
  need a policy decision on exact-boundary + known-callee + RAM-symbol consistency without entry.
- L-012 (password) deferred: progression achieved via RAM; one natural pair recorded (EDCBAAK after
  1 game / 1 win, Philly). L-006/L-008/L-013/L-014 label-only, unchanged (L-014: new $7000 writer in
  $C000). L-026 (new, low): scene-12 stall after SaveGameState clears DpcmRequest.

## Gameplay milestones (analysis/scenarios.json)
- S10 end of game: visited (artificial; HalfInning=$10, OutCount=2, scores, strikes forced) -> Pennant
  Race screen with password EDCBAAK.
- S11 season end + ending: visited (artificial; PennantCounterA=$84, B=$55): "132 GAMES 86 WIN" season
  screen, then scene 12 stalled (DpcmRequest=0); with extra $077B=$FF the ending celebration ran.
- S13 state $62 change-sides handler executed. Snapshots: scratch/snapshots/r1a1-e3-62.snap,
  r1a1-e3-end.snap, r1a1-e4-end.snap, r1a1-e7.snap.

## Remaining avenues and why deferred
- L-024 entries: no further static method found (searches listed above exhausted); scene sweeps would
  only replay statically known table handlers.
- Natural confirmation of L-026 needs a Write trace of $077B through scene 17 (low value, no bytes).
- Toolkit: TK-004 added (frame-step budget limits exceed documented maximums). Servers stopped.

## Recommendation
Start round 2 disassembly with ready leads L-009, L-015..L-023, L-025 (~12.1 KB). Not terminal.
