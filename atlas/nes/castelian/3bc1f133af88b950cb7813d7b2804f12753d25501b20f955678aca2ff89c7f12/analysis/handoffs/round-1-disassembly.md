# Round 1 disassembly handoff

- Scope: first disassembly pass on a fresh workspace (Castelian, NES UxROM mapper 2, 128 KiB PRG, CHR-RAM).
- Checkpoint: pass-001 — maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json
  (ok, byte_identical; map sha256 21a4a6bf92e38b70721466baef5bb1678a40d11854f5996071bc531931e1a946).
  Internal store analysis/state.json; portable map maps/current.json (validated). Baseline also created.
- ROM classification: 68.00% (89140 / 131088 bytes classified; 41948 bytes unknown).
  Per-bank table in analysis/progress.md (Round 1).

## Findings
- Fixed bank (16K bank 7, $C000-$FFFF): Reset $C154, NMI stub $C008 = jmp ($0000) (handler vector in RAM $00/$01;
  handlers $C991, $C8DC, $FBDA, $FEC5, bank3 $B781, bank5 $A8B9, bank6 $8761), IRQ $C991.
  SwitchPrgBank $C00B (A&7 -> sta $C000,x, bus-conflict table $C000-$C007).
  Main loop $C2A1..: jsr $C5ED gameplay loop; $46 tower complete -> $F6EE tally, inc $0A,x, bank 6 bonus at $8000;
  $0A==8 -> $F469 ending. Print routine $FD17 (pointer $18/$19, X/Y screen pos).
- 16K banks 0-2 are data (graphics/tile and map data, leftover C64 source text) except an executed routine at bank 2 $BA00-$BAE0.
- Bank 4 = main game/render code incl. unrolled PPU copy chains ($909D.., $9864..) entered through RTS-dispatch tables.
- Bank 6 = bonus stage + tower-name tables. Bank 5 = partly code ($A0xx-$A9xx), rest unresolved.
- Annotated regions: 459 Code (Observed/Static, explicit views: switchable [2b,2b+1,14,15], fixed [null,null,14,15]) and 36 Data.

## Evidence / artifacts
- analysis/scenarios.json (S01-S13 routes, RAM/register patch recipes).
- analysis/evidence/round1/observed-contexts.json (observed ROM/RAM instruction contexts, artificial flag),
  rom-data-reads-sampled.json (sampled ROM data reads), unknown-ranges.json, tools/ (local analyzer + scripts; uses /root/rtools paths).
- analysis/investigation.md (method, rules, caveats); analysis/toolkit-issues.md (TK-001..TK-004).

## Unresolved questions / deferred work (leads L1-01..L1-08 in analysis/leads.json)
- Bank 3 interior (~11.7 KB): code vs data; whole bank read by checksum, reads don't help.
- Bank 5 (~14.5 KB) and bank 6 (~11.7 KB) unknown data-like areas (tower-row patterns, likely level/music data): need consumer tracing with range-filtered Read capture.
- Bank of jmp ($0063)/($01EB) targets; small unreached code fragments; $3F bit7 mode; small tables.
- Power-on Reset path $C154-$C17D is annotated Static (not present in captured coverage aggregates).
- Not visited: PLAYER 1/2 WINS (unequal scores), high-score entry, demo mode.

## Leads
- Consumed: none. Produced: L1-01, L1-02, L1-03, L1-04, L1-05, L1-06, L1-07, L1-08.
- Pass status: completed (easy gains taken); ROM incomplete.
