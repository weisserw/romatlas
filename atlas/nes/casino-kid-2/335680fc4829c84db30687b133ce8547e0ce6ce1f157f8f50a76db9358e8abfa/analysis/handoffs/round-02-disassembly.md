# Round 2 / disassembly handoff

Scope: one disassembly pass on pass-001 state: consume ready leads R1-L009..R1-L020, re-review
round-1 rejections with the always-taken-branch correction, residual review, bounded final-match
gameplay attempt. Baseline not re-created; input/game.nes unchanged.

Checkpoint: **pass-002** — maps/pass-002.json (sha256 a228e8149fdfca8fa42a9e2d1c864ea639b1b863b189cc4a0f29d812e4c60d36),
generated/pass-002, analysis/pass-002-verification.json.
Verification: map.validate ok; maps/current.json regions == analysis/state.json; `run.py checkpoint
pass-002`: ok=true, byte_identical=true.

ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown).

Status: **incomplete** (2 B unknown, bank 4 $BB30-$BB31). Per-bank counts: analysis/progress.md
("Round 2 / disassembly"). Code 37,924 B, data/padding 93,162 B.

Findings
- All 12 redirect leads verified as specified (11,783 B): graphics sets, card blocks, bank-4 poker
  handlers (Static code + Observed $914D-$91C4), bank-5 $831B routine, text/VRAM tables, DPCM, palettes.
- Corrected always-taken-branch rule: no other unknown byte is a control target; no further routines.
- R2-D01 (311 B): 7 unreferenced but coherent code fragments (Static conf 0.5-0.6, reachability
  unresolved) incl. $90E5 routine (data region 2247 narrowed to $9108) and $C1B3 jmp $C1FD;
  28 small consumer/pattern data pieces.
- R2-D02 (4,545 B): Data-flanked unknown windows classified generic data (conf 0.75, signatures noted).
- Gameplay: $06D2 (set at spin start) is not the roulette result; natural ending still not reached.

Leads consumed: R1-L009..R1-L020 (verified). Produced: R2-L021 ($BB30 2 B, ambiguous),
R2-L022 (roulette result variable -> natural ending route).

Unresolved / for commentary and redirect
- Commentary: carry round-1 ann/ forward; rename TextPointerTable -> CardBlockTable; label new code
  (bank-4 $8DEA-$90D0 raise/chip handlers, $914D yes/no handlers, bank-5 $831B, $90E5); the UNKNOWN
  boundary notes for consumed ranges are obsolete.
- Redirect: specific roles for R2-D02 generic windows (bank-6 music streams via command length table,
  bank-2/5 BuildVramStrings block walks, bank-6 name lists); confirm or reject low-confidence fragments;
  R2-L021; R2-L022 (trace B5_ResolveBets during a spin).

Artifacts: analysis/investigation.md, analysis/leads.json, analysis/scenarios.json (S11),
analysis/tools/round02/ (scripts + applied-annotations.jsonl.gz), traces/round02/ (spin frames).
Toolkit: analysis/toolkit-issues.md — new TK-005 (map.annotate needs region.id = 0 for new regions;
docs mismatch). Existing: 3 confirmed bugs (TK-001, TK-002, TK-005), 2 missing features (TK-003, TK-004); none blocking.
