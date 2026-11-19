# Round 1 — disassembly handoff (2026-11-18)

Status: INCOMPLETE (easy gains exhausted for this pass).

## Scope
First pass on a fresh workspace: baseline, static discovery from vectors, traced
gameplay (title, 1P team select, batting, fielding, RAM-shortcut to GAME SET, 2P start),
inline-argument call contracts, task-entry roots, data/padding classification of the
large table area.

## Checkpoint and verification
- Checkpoint pass-001: maps/pass-001.json, generated/pass-001,
  analysis/pass-001-verification.json (ok, byte_identical). analysis/state.json and
  maps/current.json saved/validated.
- ROM classification: 98.20% (24149 / 24592 bytes classified; 443 bytes unknown).
- Per bank and kind counts: analysis/progress.md (Round 1 section).

## Key findings (details: analysis/investigation.md)
- Game: Baseball (1984), NROM-128; code views $C000-$FFFF, prg_banks [0,1,0,1].
- Inline-argument call conventions (E99A family, EA17) with per-routine lengths; 84
  sites annotated as Data (traces/r1-inline-sites.json). Cooperative task scheduler
  (EB0A/E29B create, EB94 dispatch, EB41/EB48 yield) — task entry words used as roots.
- Sprite lists (C8BB format) and PPU string lists (EA63 format) parsed to exact bounds.
- RAM: $33 half-inning counter (18 = game end), $0F/$10 controller roles, $64 balls.

## Unresolved / deferred
- 443 unknown bytes in 17 ranges: leads L-001..L-005 (entry evidence missing for code
  fragments / mixed code+table slices). L-006 static-only code reachability; L-007 data
  semantics for commentary.
- Gameplay not visited: extra innings / C6F7 path, substitution menus, other teams.
- Static-only code (1942 bytes) relies on traversal rules documented in investigation.md;
  commentary should review it.

## Artifacts
- analysis/scenarios.json, analysis/leads.json, analysis/toolkit-issues.md (TK-001, TK-002)
- traces/r1-coverage-offsets.json, traces/r1-inline-sites.json, traces/snapshots/*.json
  (same-build snapshots)
- analysis/tools-r1/ (client, traversal, annotation scripts used this pass)
- Persistent RPC daemon helper: analysis/tools-r1/rpcd.py (socket /tmp/romatlas-bb.sock)
