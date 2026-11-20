# Round 1 disassembly handoff

Scope: first disassembly pass on fresh workspace (The Black Bass USA, UxROM 128K, CHR-RAM).
Baseline created and verified (maps/baseline.json).

Checkpoint: pass-001 — maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json.
Verification: map.validate ok; generate ok; verify byte_identical=true.

ROM classification: 98.90% (129652 / 131088 bytes classified; 1436 bytes unknown).

Per-bank counts: analysis/progress.md (Round 1 table). All unknown bytes are in fixed bank 7.

Findings (detail: analysis/investigation.md):
- Bank select $F854 (X=bank, table $EC00). Sound trampolines $C030-$C039 -> bank 0.
- Inline-arg routines $F06C (5 B) and $F0F7 (2 B); display-list format decoded; all lists and
  their source strings/tile maps classified from consumers.
- Main-mode jump table $C17D; sound jump table bank0 $B025.
- Banks 1-4 = per-lake data (30-pointer header), bank 5 = tile data, bank 0 $8000-$9FFF lake map data.
- RAM: $0301 mode, $0303/$0304 clock, $030D lake, $17/$18 pads.

Gameplay: analysis/scenarios.json (S-001..S-011), analysis/gameplay.md. Reached title, name,
menus, beginner lake intro/info/map, boat move, casting, point menu, day end, ceremony code,
password failure, San Lake via $030D patch. Not reached: fish catch, valid password, later days,
winner/ending (L-004).

Unresolved questions / deferred work:
- L-001: 1436 B of unreferenced code-like routines in $C000-$F853 (callers? dead library code?).
- L-002: stray single bytes after terminal instructions; $C040 jmp ($FFFC) vector slot.
- L-003: data semantics (lake headers, sound data, bank 6 tables) for commentary.
- L-004: fish strike/catch RAM recipe; password entry; multi-day / ending.
- L-005: observe static-only code (7113 B).
- Code regions are unlabeled (commentary phase). Local descent scripts were scratch only;
  method is described in investigation.md and reproducible from the map.

Lead IDs: L-001, L-002, L-003, L-004, L-005 (analysis/leads.json).
Toolkit issues: TI-001 (NES input form silently ignored, confirmed), TI-002 (discover param
names, confirmed), TI-003 (static discovery stops at jsr/indexed stores on UxROM, missing
feature), TI-004 (trace slowdown, missing feature), TI-005 (coverage artificial duplicates,
suspected doc gap). See analysis/toolkit-issues.md.
