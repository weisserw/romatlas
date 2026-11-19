# Final coordinator handoff — terminal INCOMPLETE (2026-11-18)

Workflow (goal.md) reached its documented terminal handoff: justified terminal
incomplete after the required rejection review. This is **not** whole-ROM completion.

## Result
- Game: Baseball (Nintendo, 1984), NES NROM, 16 KiB PRG ($C000-$FFFF), 8 KiB CHR.
- ROM classification: 100.00% (24591 / 24592 bytes classified; 1 bytes unknown).
  Exact: 99.996%. Rounded 100.00% does NOT establish completion.
- Per bank (classified/unknown): header 16/0; PRG bank 0 ($C000-$DFFF) 8192/0;
  PRG bank 1 ($E000-$FFFF) 8191/1; CHR 8192/0.
- Sole residual: physical 0x2604 ($E5F4), an unreferenced $60 (RTS) byte between
  DrawTitleCursor's RTS and Random — lead L-009.

## Checkpoints and verification
- Final map checkpoint: pass-003 (maps/pass-003.json == maps/current.json;
  analysis/pass-003-verification.json: ok, byte_identical). Earlier: pass-001, pass-002, baseline.
- Commentary: commentary/source/game.asm; `commentary.py verify` fresh at handoff:
  ok, byte_identical (game.asm sha256 a0f6b465aeb5042f…). Reading copy commentary/reading-after.asm.
- Review: 472/472 entries reviewed (commentary/review.json). Final editorial audit
  (`commentary.py audit`) intentionally reserved for final acceptance/archive.
- input/game.nes sha256 unchanged (421399f2…).

## Rejection review (residual 0x2604)
- Round-1 review: analysis/handoffs/round-1-redirect.md. Coordinator critique returned a
  revised test (entry-1 RTS dispatch, split hi/lo tables, inline args, indirect/stack
  entries, padding convention). Round-2 review (analysis/handoffs/round-2-redirect.md,
  evidence analysis/redirect/r2-a/) found all negative; ROM padding convention is $FF;
  evidence-policy states an unreferenced return opcode alone lacks structure.
  Coordinator assessment: SOUND; reused in round 3 (unchanged premises). Reopen only if
  a new reference/consumer or padding convention appears.

## Rounds
- R1: disassembly 98.20% (443 unknown) → commentary → redirect (L-001..L-005, L-008 ready).
- R2: disassembly applied all → 99.996% → commentary (round-1 copy in analysis/round-1/) →
  redirect (L-009 retested; L-011, L-012 editorial).
- R3: disassembly (L-011 metadata) → commentary (L-012; round-2 copy in analysis/round-2/) →
  redirect (no structural leads; L-013, then r3-b runtime test → L-014) → commentary
  continuations applying L-013, L-014.

## Remaining documented uncertainties (commentary/uncertainties.md)
HitPower/HitAngle table roles; song-to-event map; purposes of statically classified
code fragments with no known caller (reachability unknown, not claimed unused);
return-to-base runner path (static reading only); bases-awarded path not exercised.

## Links
- Phase handoffs: analysis/handoffs/round-{1,2,3}-{disassembly,commentary,redirect}.md
- Leads: analysis/leads.json; progress: analysis/progress.md; ledger: analysis/investigation.md
- Gameplay: analysis/gameplay.md, analysis/scenarios.json (GAME SET reached via RAM patch;
  extra innings unavailable by game rule).
- Toolkit issues: analysis/toolkit-issues.md (TK-001, TK-002; non-blocking).

## Addendum 2026-11-18 — user-directed residual fallback and final acceptance

Superseding the "Result"/"Checkpoints" lines above for current state; the text above is
preserved as the terminal-incomplete record for pass-003.

- Explicit user authorization (batch-only exception, <= 10 residual bytes): the sole residual
  byte 0x2604 ($E5F4, 1 byte, verified exact complement) was annotated generic Data, Manual
  provenance, meaning unknown. **This is a user-directed fallback, not evidence-backed
  classification**; L-009's code-vs-data ambiguity remains open. Record:
  analysis/handoffs/final-fallback.md.
- Classification now reported in two parts:
  - Evidence-backed: 24591 / 24592 bytes (99.996%); per bank unchanged except PRG bank 1
    8191 evidence-backed + 1 fallback.
  - User-directed Manual fallback: 1 byte (0x2604). Unclassified: 0.
  - Evidence-backed whole-ROM classification is NOT complete.
- Final checkpoint: pass-004-fallback (maps/pass-004-fallback.json == maps/current.json ==
  analysis/state.json, sha256 fc5d69ba…); analysis/pass-004-fallback-verification.json ok,
  byte_identical. ROM sha256 unchanged (421399f2…).
- Commentary: round-3 commentary preserved at analysis/round-3/commentary/; fresh baseline and
  carry-over (analysis/handoffs/round-4-commentary.md). Final editorial review done;
  `commentary.py audit` PASSED (commentary/audit.json: ok, byte_identical, 472 reviewed regions,
  620 reviewed units, editorial_audit true); game.asm sha256 62a7a702…; reading copy refreshed.
  Coordinator re-ran audit independently: passed, same hashes.
- No redirect research was reopened for the residual byte (per user instruction).
- Toolkit issues: TK-001, TK-002 (non-blocking), TK-003 new (map.annotate docs mismatch).
- archive-options.json updated; no archive/cleanup executed.
