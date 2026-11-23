# Round 1 disassembly handoff

Status: pass finished; ROM incomplete (1 byte unknown; several claims at reduced confidence).

- Scope: baseline, vector/bank-contract static traversal, traced scenarios S1-S6, gap review, annotation.
- Checkpoint: maps/round1-disasm-v2.json, generated/round1-disasm-v2,
  analysis/round1-disasm-v2-verification.json (ok, byte_identical). State: analysis/state.json; export maps/current.json.
- ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown) -- rounded, not complete.
- Per-bank counts and metrics: analysis/progress.md "Round 1 disassembly".

## Findings
- Caesars Palace (NES, UxROM, CHR RAM). Code only in 16K banks 0 (sound), 6 (all game screens), 7 (fixed).
  16K banks 1-5 = data (+ $FF fill), each bank ends with a copy of the fixed $FF80-$FFFF template.
- Key routines: SelectPrgBank $FF80 (X=16K bank, $0305 mirror), NMI $F955, ReadJoypad $FAF2 -> $0436,
  UpdateButtonStates $FD0C -> $03CE-$03DD, WaitNextFrame $D9E5 (waits on $0F), screen dispatch $C44C.
- Details/method: analysis/investigation.md. Tools: analysis/round1-tools/ (trav.py, build.py, frag.py, ...).
- Scenarios/routes: analysis/scenarios.json; snapshots analysis/snapshots/*.snap.json (same-build only).

## Unresolved questions
- File 0x18010 ($EA at bank-6 $8000): code or data (L-003).
- Natural callers of the unreferenced fragments (L-004); exact extents/meaning of data tables (L-005).
- How betting / coin insertion works (slot and blackjack UIs did not respond to simple A/B/drag).

## Deferred work
- Visit other games, cash-out and ending via the $C44C dispatch Y contract (L-001); not done because
  emulation with tracing runs ~3 frames/s (TK-002) and the UI contract is unknown.
- Data-format decoding of banks 1-5 (L-002).

## Toolkit
TK-001..TK-004 in analysis/toolkit-issues.md (one confirmed bug: NES emulator.input ignores the documented button/pressed form; use {buttons:[...]}).
