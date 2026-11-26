# Round 1 disassembly handoff

- Scope: one disassembly pass (attempt 2; attempt 1 was cut off after the baseline). Baseline reused, not rerun.
- Checkpoint: **pass-001**. Files: maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json.
- Verification: ok=true, byte_identical=true. Map sha256 f73ff2c88aa064ce2761a8abc7c26566b187a2219a6906f9829a79840ab35c3a,
  identical to analysis/state.json and maps/current.json. Generator diagnostics: none.
- ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).
  Project status is still incomplete: commentary/readability and editorial gates have not run, and many Data regions are generic.
- Pass ended normally. The serve process was stopped.

## Findings

- CNROM: one fixed 32 KiB PRG. CPU $8000-$FFFF maps to file offsets 16..32783, so prg_banks [0,1,2,3] is exact. Vectors: NMI $F260, RESET/IRQ $F22A.
  CHR bank select is `lda $F200,x / sta $F200,x`, which uses a bus-conflict identity table ($F200-$F207).
- Code totals 13859 bytes (Observed 8077, Static 5782):
  - Static discovery from the vectors plus the 16 targets of the sound-command RTS dispatch (table $E459, index (cmd-$F0)*2) found every observed fetch.
  - Gap probing added 1309 bytes of uncalled routines. Most are a math/PPU helper library at $F3C9-$FF09, including inline-argument routines $F5C4/$FD29/$FE7F.
  - A hidden-code scan added 4 code islands ($9D74, $9FFB, $A045, $A1DB; 152 bytes).
- Data and other non-code:
  - Data is 18548 bytes, including the iNES header. Each region's evidence lists its consumer instructions, pointer setups or patterns, and the first non-instruction byte.
  - Named tables: SoundCmdJumpTable, NotePeriodTable (84 APU periods), SoundPtrTable_E548/E5A0, ChrBankSelectTable, Vectors.
  - Text (116 bytes): katakana/ASCII "Another Ltd." signatures and build date "1989/2/27".
  - Padding: 261 bytes.
  - CHR-ROM: 32768 bytes, marked Image.
- Gameplay: title, menu editing, 1P/2P selection (P2 uses controller port 2), lane play, and a full 1P game to the final scorecard were reached naturally.
  RAM facts: $01 = players-1, $02 = lane-1, $92 = frame number.
  Details: analysis/scenarios.json, analysis/gameplay.md. Snapshots: analysis/snapshots/. Evidence: analysis/evidence/pass-001/.

## Unresolved questions / deferred work

- Reachability of the uncalled routines and islands (L-001, L-002). Header word $9FFA at $9F97 looks like an RTS-style pointer to $9FFB.
- Formats inside the generic Data blocks: the metasprite grammar at $FB57, descriptor headers, and sound streams (L-003, L-004).
- The sound path for $80-$EF bytes hangs at `bcc $E171` (L-008).
- Gameplay not reached:
  - strike/spare and hi score (L-006; $C2-$C7 is not the hi score);
  - 3P/4P selection (L-007);
  - leaving the final scorecard;
  - tracing the end of a 2P game (L-005).

## Leads

- Produced: L-001 to L-008 (analysis/leads.json).
- Consumed: none (the lead list was empty at the start).

## Toolkit issues (analysis/toolkit-issues.md)

- Confirmed bugs (doc/API mismatches):
  - TK-001: emulator.input ignores button/pressed.
  - TK-002: discovery reads max_instructions, not budget.
  - TK-003 (new): the discover_context entrypoint shape on NES differs from the docs.
- Missing feature:
  - TK-004 (new): tracing is about 12x slower even with capacity 0. A coverage-only mode is needed.
- No blockers. Workarounds are recorded in the report.

## Reproduction

The helper scripts are in analysis/tools/pass-001/ and were run from the removed analysis/scratch dir. rpcd.py bridges `run.py serve` to a unix socket. make_map.py rebuilds the map from discovery output plus spec.py.
The input log is analysis/evidence/pass-001/inputs.log.
