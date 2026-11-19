# Final handoff — Round 4 (user-authorized residual fallback + final audit), 2026-11-19

Supersedes the current-state statements of analysis/handoffs/final-terminal.md (retained unchanged as the
historical Round-3 terminal-incomplete record). Workflow termination is not a claim of hardware/game compatibility.

## Classification
- Checkpoint pass-004 (maps/pass-004.json == maps/current.json == exported analysis/state.json):
  analysis/pass-004-verification.json ok true, byte_identical true. input/game.nes sha256 unchanged.
- ROM classification: 100.00% (393232 / 393232 bytes classified).
  - Evidence-backed (Observed/Static provenance): 393229 bytes (regions 1..1906, unchanged from pass-003).
  - Manual user-directed fallback (reported separately; NOT evidence-backed): 3 bytes.

## Manual fallback record (explicit user authorization)
- Authorization: explicit batch-only user instruction (2026-11-19) permitting, when at most 10 physical ROM
  bytes remain unclassified after the terminal incomplete handoff, marking them as generic Data with unknown
  meaning using map.annotate and Manual provenance. It overrides the further-structural-proof rule for these
  residual bytes only. It is a user-directed fallback, not an observed or statically proven data role.
- Exact ranges (verified complement before editing, 3 <= 10):
  - phys 0x14CCA (85194), length 1, bank 10 CPU $8CBA -> region 1907
  - phys 0x17BC6 (97222), length 1, bank 11 CPU $BBB6 -> region 1908
  - phys 0x1FD5A (130394), length 1, bank 15 CPU $FD4A -> region 1909
  Kind Data, provenance Manual, confidence 0.3, no label; no execution view (toolkit allows views only on Code; TK-006).
- Prior ambiguity (preserved): each byte is a lone unreferenced RTS-valued byte at a routine boundary; the
  code-vs-data decision was materially ambiguous. Seven methods were non-discriminating (analysis/handoffs/
  round-03-redirect.md rejection review; analysis/redirect/r02-a/, r03-a/; analysis/tools-round3/; lead L017).
  No semantics are claimed; reachability is not claimed. Script/records: analysis/tools-fallback/.

## Commentary / editorial
- Round-3 commentary preserved at analysis/round-3/ (round-1/2 also retained). Fresh baseline on pass-004,
  carried forward by physical range. Handoff analysis/handoffs/round-04-commentary-final.md.
- Final strict editorial audit (commentary.py audit): PASSED — ok, byte_identical, 1909/1909 regions,
  2117/2117 units, editorial_audit true. Coordinator re-ran commentary.py verify: byte_identical true,
  source game.asm sha256 712a9ad5f3c6dd06532de821c59f9a7b699886f75d77220c0729b6f8779ee8b9.
  archive.editorial_status: complete true.
- Remaining semantic uncertainties (documented, mechanical descriptions): commentary/uncertainties.md.

## Other leads (unchanged, remain open/blocked)
- L018 blocked (442 B no-caller routines, Static conf 0.7, no reachability claim).
- L006 ending reached only with assisted fight; password screen arrival unconfirmed (analysis/scenarios.json).
- Toolkit issues: analysis/toolkit-issues.md (TK-001..TK-006, none blocking). No archive executed.
