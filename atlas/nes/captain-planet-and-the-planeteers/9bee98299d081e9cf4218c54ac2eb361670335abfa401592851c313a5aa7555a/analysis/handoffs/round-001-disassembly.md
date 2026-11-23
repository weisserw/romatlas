# Round 1 disassembly handoff (pass-001)

Scope: first single easy-gain pass on Captain Planet and the Planeteers (NES, MMC3, 128K PRG/128K CHR).
Baseline created; pass-001 checkpoint: maps/pass-001.json, generated/pass-001 (local),
analysis/pass-001-verification.json — ok true, byte_identical true. Internal state analysis/state.json; export maps/current.json.

ROM classification: 92.64% (242872 / 262160 bytes classified; 19288 bytes unknown).
Status: pass complete; ROM incomplete.

## Findings
- Bank model: MMC3 PRG mode 0 only; $C000/$E000 fixed banks 14/15; R6 via $FFC5, R7 via $FFCE (only selector writes at $FFD7).
- Code executes at $A000 from banks 3,6,7,8,12,13; banks 0,1,2,4,5,9,10,11 are data-only (whole banks annotated Data with selector/consumer evidence).
- NMI-driven cooperative task system ($FEEE create with inline entry word, $FF47 yield, $FF63 sleep, dispatcher $FE9F, task bank $052B).
  Inline-argument helpers $FEEE/$FF57/$FD7A (+2) and $FB19 (palette string). $C000-$C129 JMP vector table; per-bank JMP stub tables.
- Password table bank 0 $8024 (10 passwords -> all 10 stages incl. ending); see analysis/gameplay.md.
- CHR ROM annotated as Image; header and vectors as Data.

## Artifacts
analysis/progress.md, analysis/investigation.md, analysis/leads.json (L001-L006), analysis/scenarios.json, analysis/gameplay.md,
analysis/toolkit-issues.md. Local helpers (not deliverables): tools_local/ (rpcd.py/rpc.py JSON-RPC wrapper, trav.py traversal,
gapclass.py, annotate.py rebuilds the map from regions_code/data/gap JSON, pw.py password scenario, snaps/password_screen.json and snaps/cp_stage1.json same-build snapshots, observed_roots.json saved contextual roots).

## Unresolved questions / deferred work
- L001: bank 8/13 Captain Planet stage code (13.8 KiB unknown) — needs longer traced CP gameplay (levels 1-5) and/or static task-word resolution.
- L002: JMP ($0000) pointer-table dispatchers (bank 3 $A4A3, bank 6 $A6E2, bank 12 $A904) — prove table extents.
- L003: fixed-bank gaps ($E5F1, $E655-$E6C0, $F3D9-$F6FA, pointer tables $F06F/$E786/$E8A9).
- L004/L005: review heuristic gap Data/Text and same-bank task inference (assumptions, Static provenance).
- Not visited: bosses, game over/continue, CP gameplay for levels 2,3,5.

## Toolkit issues (analysis/toolkit-issues.md)
TI-001 missing feature: NES discovery never follows JSR on banked mappers (worked around with local traversal).
TI-002 confirmed bug: discover_context param names differ from docs. TI-003 confirmed bug: emulator.input silently ignores {button,pressed}.
TI-004 performance: tracing ~9x slower (capacity 0). No blockers.

Resume point: none needed (pass complete). Next disassembly round can start from analysis/state.json + tools_local/trav_extra.json roots.
