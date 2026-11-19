# Round 2 — disassembly phase handoff (2026-11-19)

ROM classification: 100.00% (327695 / 327696 bytes classified; 1 bytes unknown).

Status: incomplete (1 unknown byte). Rounded 100.00% is not completion.

## Checkpoint
- pass-002: maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json — byte_identical true,
  map sha256 05fd28c2... Internal store analysis/state.json (1,496 regions), export maps/current.json.
- Input checkpoint pass-001 (95.56%, 14,557 unknown) preserved.

## Lead outcomes (details: analysis/investigation.md, analysis/leads.json)
- Verified and annotated: L-015 code 5,226 (incl. Data->Code $1B7C2+138), L-016 250, L-017 2,252,
  L-018 1,472, L-019 2,240, L-020 140, L-009 70, L-021 28, L-022 15, L-023 541 -> 12,096 bytes (projection matched).
- L-024 reopened and verified on corrected premise (structural evidence without an entry): 2,427 bytes
  Code/Static (confidence 0.75, reachability unresolved) + 33 bytes Data. Rejected 1 byte ($9CAC lone RTS) -> L-028.
- Unchanged: L-025 (commentary score label swap $0418 home / $0426 visitor), L-026 (ending stall), L-005..L-008,
  L-012..L-014. New: L-027 (commentary for round-2 code), L-028 (1 unknown byte).

## Per-bank summary (8K banks; full table in analysis/progress.md)
All 8K PRG banks 0-31, header and CHR have 0 unknown except 8K bank 12 (32K bank 3 $8000-$9FFF): 1 byte.
Largest gains: 8K banks 12/13 (+2,425/+2,497), 8 (+2,412), 7 (+1,617), 1 (+1,414), 14 (+961).
By kind: Code Observed 31,968; Code Static 27,550; Data 136,137; Padding 66,492; Image 65,536; Text 12.

## Findings
- FarCall-balanced static traversal (r1-a1 F1) reproduces exactly; no new invalid/overlap flags.
- Unreferenced bank-3/bank-0 routines are structurally code: RAM operand profile matches classified code
  (control on classified data: 0.55 mean, 1/135 false positive). They include controller-2 debug shortcuts
  and the InitGameRam -> bank 1 $F074 chain.

## Unresolved questions / deferred work
- L-028: is $9CAC (phys $19CBC) referenced anywhere (disabled-hook entry?) — needs a reference or a policy
  decision; only remaining unknown byte.
- Reachability of L-024 code (no entry known) — commentary should state "reachability unresolved".
- L-025 label swap and L-027 naming for commentary; L-026 natural ending behaviour (redirect, low).
- Gameplay: no new runs this pass (checklist reviewed; see analysis/scenarios.json notes_round2_disassembly).

## Toolkit
analysis/toolkit-issues.md: no new toolkit issues found this pass (TK-001..TK-004 unchanged, none blocking).
Server stopped; scratch pruned (round-2 WIP state copies removed); disk ~4.9 GB free.
