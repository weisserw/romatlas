# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Coordinator log

### 2026-11-14 — Round 1, phase 1 (disassembly): BLOCKED before start

- Round: 1. Current phase: disassembly (prompts/disassembly.txt), not yet started.
- Input checkpoint: none (no baseline, no maps/baseline.json, no analysis/state.json).
- Output checkpoint: none. Lead IDs consumed/produced: none.
- Blocker (execution environment, not toolkit): every Bash command fails before the
  command runs, in both the coordinator session and a spawned worker agent:
  `apply-seccomp: write /proc/self/setgroups (nested userns is capability-restricted;
  caller must provide CAP_SYS_ADMIN): Permission denied`. The OS sandbox (bubblewrap)
  can't create its nested user namespace, and this session can't turn the sandbox off.
  Because of that, `python3 run.py inspect/baseline/serve/checkpoint` and the romatlas
  binary can't run, so no phase worker can do any work.
- No workspace files besides this log were changed. input/game.nes was not touched.
- Resume recipe: once commands run in this environment (the sandbox needs to allow nested
  user namespaces / CAP_SYS_ADMIN, or run without the bubblewrap sandbox), start at
  workflow.md step 1 (`python3 run.py inspect`, then `python3 run.py baseline` once). Then
  launch the round-1 disassembly worker with prompts/disassembly.txt.
- archive-options.json was not written: there is no terminal handoff, and no final
  documents exist yet to choose between.

### 2026-11-18 — Coordinator resume: environment unblocked

- Bash/binary execution works again (prior seccomp blocker cleared).
- Coordinator ran workflow step 1: `python3 run.py inspect` (mapper 4, PRG 256 KiB,
  CHR 128 KiB, horizontal, no battery/trainer) and `python3 run.py baseline` once:
  analysis/baseline-verification.json reports ok/byte_identical true
  (maps/baseline.json sha256 c8bb23d8…dead). Baseline is raw export only, not classification.
- Next: Round 1, phase 1 disassembly worker (prompts/disassembly.txt), input checkpoint = baseline.

### 2026-11-18 — Round 1, phase disassembly (pass-001): complete

- Input checkpoint: baseline. Output checkpoint: pass-001 (maps/pass-001.json, generated/pass-001).
- Verification: map.validate ok; checkpoint verify ok, byte_identical true.
- ROM classification: 61.07% (240154 / 393232 bytes classified; 153078 bytes unknown).
- Code 33,004 (Observed 26,459, Static 6,545); Data 141,575 (header, CHR, DPCM, tables, bank IDs,
  jump tables); Padding 65,575. Per-bank counts: analysis/handoffs/round-001-disassembly.md.
- Scenarios: S-001..S-007 (WATCH full game, 1P menus/batting/pitching, extra innings shortcut, EDIT).
- Leads consumed: none. Produced: L-001..L-006. Toolkit issues: TK-001 (bug), TK-002, TK-003 (missing features).
- Outcome: easy gains taken; ROM incomplete. Handoff: analysis/handoffs/round-001-disassembly.md.

### 2026-11-18 — Coordinator: Round 1 classification reported to user; starting commentary

- Printed to user: ROM classification: 61.07% (240154 / 393232 bytes classified; 153078 bytes unknown).
- Round 1, phase 2 (commentary, prompts/commentary.txt) starting on checkpoint pass-001.

### 2026-11-18 — Round 1, phase commentary: complete (readability scope)

- Input checkpoint: pass-001. Prepared commentary/ (verified baseline), rebuilt commentary/source/game.asm
  from a bank:CPU-keyed name/comment database (analysis/tools/round-001-commentary/).
- Covered all classified source: 932 routine labels (923 hand-named with comments), 1,122 branch labels,
  ~150 named RAM symbols + Var_ equates, MODE_/OBJ_ constants, FarCall target notes, notes on 181 unknown gaps.
- Verification: commentary.py verify byte_identical true; reading copy commentary/reading-after.asm refreshed.
- Many banked names are tentative (documented). Redirect questions: commentary/uncertainties.md.
- ROM classification unchanged (61.07%). No toolkit issues found. Handoff: analysis/handoffs/round-001-commentary.md.

### 2026-11-18 — Coordinator: Round 1 commentary accepted; starting redirect

- Commentary handoff analysis/handoffs/round-001-commentary.md: all classified source covered
  (923 hand-named routines, RAM/consts symbolised), commentary verify byte_identical true,
  reading copy commentary/reading-after.asm refreshed. Noted depth limit in banks 20-28
  (tentative names, mechanical branch labels) for final-acceptance editorial work.
- Round 1, phase 3 (redirect, prompts/redirect.txt) starting; attempt dir analysis/redirect/r001-a/.

### 2026-11-18 — Round 1, phase redirect (attempt r001-a): complete

- Input checkpoint pass-001; classification unchanged (61.07%). Research in analysis/redirect/r001-a/.
- Ready leads: L-007 FarCall tables, L-008 static closure (~9.9 KB code+tables), L-009 orphan routines,
  L-010 banks 0/4 copies (stadium->bank hypothesis refuted), L-011 bank-31 tables/header,
  L-012 field data, L-013 bulk residual data, L-014 bank 6. L-001..L-004 superseded; L-005 partial.
- Gameplay: 1P win -> level/rating screen -> next game, 1P loss -> GAME OVER (S-009/S-010, RAM-patched);
  2P/ending deferred (S-011/S-012). Toolkit: TK-004 added.
- Outcome: next: disassembly round 2. Handoff: analysis/handoffs/round-001-redirect.md.

### 2026-11-18 — Coordinator: Round 1 redirect complete; starting Round 2 disassembly

- Redirect handoff analysis/handoffs/round-001-redirect.md (attempt analysis/redirect/r001-a/).
  Ready leads produced: L-007, L-008, L-009, L-010, L-011, L-012, L-013, L-014 (L-001..L-004
  superseded, L-005 partial). New scenarios S-009/S-010 (1P win/loss flow via RAM), S-011 2P route.
  Toolkit TK-004 added. Recommendation: next disassembly round 2.
- Round 2, phase 1 (disassembly) starting from pass-001 with leads L-007..L-014.

### 2026-11-18 — Round 2, phase disassembly (pass-002): complete

- Input checkpoint: pass-001. Output checkpoint: pass-002 (maps/pass-002.json, generated/pass-002).
- Verification: map.validate ok; checkpoint verify ok, byte_identical true (map sha256 9a0cd81b…7069).
- ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown).
- Code 48,576 (Observed 26,459, Static 22,117), Data 279,078, Padding 65,575. Unknown: three lone
  unreferenced duplicate RTS bytes (23:$B402, 28:$8620, 28:$9883). Per-bank counts in the handoff.
- Leads: L-007, L-008, L-009, L-010, L-011, L-012, L-013, L-014 verified; L-005 verified (artificial). None produced.
- Scenarios: S-012 ending reached via RAM ($66=4, $03FB=100 -> "YOU'RE NUMBER 1", credits); S-011 2P visited.
- Toolkit: no new toolkit issues found (TK-001..004 remain; analysis/toolkit-issues.md).
- Outcome: ROM incomplete by 3 bytes (policy question). Handoff: analysis/handoffs/round-002-disassembly.md.

### 2026-11-18 — Coordinator: Round 2 disassembly reported; starting Round 2 commentary

- Printed to user: ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown).
  Unknown: lone RTS bytes 23:$B402, 28:$8620, 28:$9883. Checkpoint pass-002 byte_identical.
- Leads L-005, L-007..L-014 verified; no new leads. Ending reached via RAM (S-012); 2P (S-011).
- Round 2, phase 2 (commentary) starting on pass-002; must preserve round-1 commentary under
  analysis/round-1/ before fresh prepare (per prompts/commentary.txt).

### 2026-11-18 — Round 2, phase commentary (pass-002): complete

- Round-1 commentary preserved at analysis/round-1/commentary/; fresh prepare on pass-002.
- All classified source commented: 1,426 routines named (≈470 new; 0 address-only routine labels),
  FarCall/jump tables as .addr lists, data notes with consumers, 3 unknown-byte notes.
- Corrections: stadium->bank claim, Level/RatingTotal, pitcher fatigue, wall collision (was Throw*),
  fielder jump/dive actions, defensive target points.
- Verification: commentary.py verify ok, byte_identical true; reading copy refreshed.
- Handoff: analysis/handoffs/round-002-commentary.md. Tools: analysis/tools/round-002-commentary/.

### 2026-11-18 — Coordinator: Round 2 commentary accepted; starting Round 2 redirect

- Handoff analysis/handoffs/round-002-commentary.md: round-1 commentary preserved at
  analysis/round-1/commentary/; all 1,426 routines named/commented on pass-002; verify
  byte_identical true; reading copy refreshed. Shallow areas noted for final acceptance.
- Round 2, phase 3 (redirect) starting; focus: 3 lone RTS bytes decision + open semantics.
- Round 2, phase 3 (redirect r002-a) done on pass-002: 3 lone RTS bytes rejected as genuinely ambiguous
  (L-015, rejection review in analysis/handoffs/round-002-redirect.md); L-016 commentary-only finding
  (PitchOutcome 2 = late close-game variant of batter group 2); L-013 data spot-check clean.
  Recommendation: terminal incomplete (393229/393232), unless idiom-based classification is accepted.

### 2026-11-18 — Coordinator rejection-review assessment of round-2 redirect (r002-a)

- r002-a recommended terminal incomplete (3 lone RTS bytes; L-015 rejected; L-016 semantic lead).
- Coordinator review under evidence-policy.md "Challenge a terminal decision":
  - 28:$8620 (after rts $861F) and 28:$9883 (after rts $9882, skipped by bne +2): no static edge,
    reader, pointer or fetch; policy's "unreferenced return opcode" example applies. Review sound.
  - 23:$B402: premise error. The byte is the static fall-through successor of conditional
    `bcc +1` at $B400 inside classified code; it is not "unreferenced". "Always taken" (C=0
    after the untaken `bcs` at $B3FE) is a flag-state *reachability* argument, which
    workflow/evidence-policy keep separate from structural classification. Competing
    "inline data skipped by an always-taken branch" reading has no reader of the byte.
  - Action: return 23:$B402 to redirect (attempt r002-b, continuing round 2 redirect) to
    evaluate this corrected inference and decide; reuse r002-a evidence, no replay.

### 2026-11-18 — Round 2, phase redirect (continuation r002-b): complete

- Re-evaluated 23:$B402 on the coordinator's corrected premise (fall-through successor of `bcc` at
  $B400). Toolkit decode/discover (static-only) treats it as a Static successor candidate; no
  predecessor of $B400 besides the untaken `bcs` at $B3FE (C=0) -> statically dead fall-through;
  no reader/alternate entry for a data or skip-idiom reading. -> L-017 ready (Code, Static, 1 byte,
  phys 193554, ctx [22,23,30,31]).
- 28:$8620 and 28:$9883 follow terminal RTS; no control edge; L-015 rejection stands for them.
- Handoff: analysis/handoffs/round-002-redirect-b.md; research: analysis/redirect/r002-b/.
  Recommendation: next: disassembly round 3 (apply L-017). ROM/map/state unchanged.

### 2026-11-18 — Coordinator: round-2 redirect (r002-a + r002-b) closed; starting Round 3 disassembly

- r002-b handoff analysis/handoffs/round-002-redirect-b.md: L-017 ready (23:$B402 Code, Static,
  dead fall-through of bcc). 28:$8620, 28:$9883 remain unknown (L-015 rejection review sound).
- Round 3, phase 1 (disassembly) from pass-002 with L-017 (+ optional L-016 runtime check, L-006).

### 2026-11-18 — Round 3, phase disassembly (pass-003): done, incomplete

- L-017 verified (toolkit decode/discover re-run) and applied: 23:$B402 (phys 193554) Code, Static,
  region 1251, ctx [22,23,30,31]; reachability statically dead, unfetched.
- L-015 not retried (no new evidence); L-016, L-006 deferred (no classification gain).
- Checkpoint pass-003: ok, byte_identical true (analysis/pass-003-verification.json).
- Per bank: bank 23 now 8192/8192; bank 28 8190/8192 (unknown phys 230960, 235667); all others full.
- Toolkit: TK-005 added (map.annotate id/enum casing doc mismatch); see analysis/toolkit-issues.md.
- Handoff: analysis/handoffs/round-003-disassembly.md.
ROM classification: 100.00% (393230 / 393232 bytes classified; 2 bytes unknown).

### 2026-11-18 — Coordinator: Round 3 disassembly reported; starting Round 3 commentary

- Printed to user: ROM classification: 100.00% (393230 / 393232 bytes classified; 2 bytes unknown).
  Checkpoint pass-003 byte_identical. L-017 verified/applied; L-015 unchanged; TK-005 logged.

## Round 3 — commentary (pass-003)
- Round-2 commentary preserved at analysis/round-2/commentary/ (verified identical); fresh prepare on
  pass-003; all names/comments carried by bank:CPU; 23:$B402 now NextLevelFrom4_DeadRts.
- Backlog closed: 76 Var_ RAM names and 81 Tbl_ tables named, 41 generic data blocks given consumer
  notes, 247 routine headers rewritten/corrected; L-016 folded into BatContact/BatterGroup2Dispatch.
- verify ok, byte_identical; reading copy refreshed. Remaining: 180 terse one-line headers, mechanical
  local labels, tentative game meanings. Handoff: analysis/handoffs/round-003-commentary.md.

### 2026-11-18 — Coordinator: Round 3 commentary accepted; starting Round 3 redirect

- analysis/handoffs/round-003-commentary.md: round-2 commentary preserved at analysis/round-2/commentary/;
  all Var_/Tbl_ named; 247 headers revised; verify byte_identical; reading copy refreshed.
  Residual editorial depth (180 terse headers, generic branch labels, block-level large data notes,
  runtime-unconfirmed meanings) carried to final acceptance.
- Round 3, phase 3 (redirect) attempt r003-a.

### 2026-11-18 — Round 3, phase redirect (r003-a): complete, terminal incomplete

- Runtime confirmations (S-013..S-015): scoreboard HR/AV/ERA and pitch speed in MPH; object 2 = catcher;
  L-016 = hit by pitch, outcome 2 = bench-clearing brawl scene (L-016 verified); PlayResult 1-4 = home-run
  pictures (wall ids = cleared wall). Lineup slot model and InitRamTables blocks resolved (L-018,
  commentary-only: EDIT team order/pitcher records, $ADC4 per-team starting orders). Object 10 umpire
  still tentative.
- 28:$8620/28:$9883: L-015 review reused, not retried. No classification lead.
- Handoff: analysis/handoffs/round-003-redirect.md. Recommendation: terminal incomplete -> final acceptance.
ROM classification: 100.00% (393230 / 393232 bytes classified; 2 bytes unknown).

### 2026-11-18 — Coordinator: Round 3 redirect accepted; terminal classification assessment; final editorial acceptance

- analysis/handoffs/round-003-redirect.md (r003-a): no classification lead; L-016 verified (HBP;
  outcome 2 = brawl scene); L-018 commentary-only (lineup/EDIT tables; some names wrong); scoreboard
  RAM confirmed; S-013..S-015. Recommendation terminal incomplete → final acceptance.
- Coordinator: L-015 rejection review (28:$8620, 28:$9883) re-checked earlier against primary bytes and
  evidence-policy; premises unchanged, no corrected inference remains → accept terminal incomplete for
  classification (2 unknown bytes). No further disassembly/redirect cycle.
- Next: final editorial acceptance commentary pass on pass-003 commentary (map unchanged; resume, no
  re-prepare): fold r003-a corrections, fix known editorial issues, record editorial audit, run
  commentary.py audit. Then archive-options.json and terminal handoff.

### 2026-11-18 — Final editorial acceptance (commentary, pass-003): complete

- Resumed the pass-003 commentary in place (map unchanged).
- Folded in r003-a/L-016/L-018: scoreboard HR/AVG/ERA/MPH names, catcher, brawl scene, home-run wall ids,
  lineup model and EDIT team tables.
- Unified the duplicate table equates and corrected misleading names and headers (see
  commentary/editorial-audit.md).
- commentary.py audit: ok, byte_identical true, 1232 regions and 1937 units reviewed, editorial_audit true.
- Handoff: analysis/handoffs/final-editorial.md.
- Whole-ROM classification stays incomplete: 2 unknown bytes (28:$8620, 28:$9883).
ROM classification: 100.00% (393230 / 393232 bytes classified; 2 bytes unknown).

### 2026-11-18 — Coordinator: final editorial acceptance done; TERMINAL INCOMPLETE handoff

- analysis/handoffs/final-editorial.md: commentary.py audit ok, byte_identical true, 1232 regions /
  1937 units reviewed, editorial_audit true (commentary/audit.json, commentary/editorial-audit.md).
- Final checkpoint pass-003 (= maps/current.json), byte_identical. ROM classification: 100.00%
  (393230 / 393232 bytes classified; 2 bytes unknown: 28:$8620, 28:$9883; L-015 rejection review accepted).
- Terminal handoff: analysis/handoffs/terminal.md. archive-options.json written (all analysis/ and
  commentary/ Markdown accounted for). Archive not executed (not authorized). ROM classification is
  NOT complete; workflow reached its documented terminal incomplete handoff.

### 2026-11-18 — Coordinator: USER-DIRECTED MANUAL FALLBACK for 2 residual bytes (batch exception)

- The user explicitly authorized this batch-only exception (at most 10 residual bytes). It applies only to these 2 bytes.
- Verified the complement before editing. analysis.unknown on the pass-003 store gave exactly
  phys 230960 (28:$8620) and 235667 (28:$9883), 1 byte each, matching the user's list.
- map.annotate added both as Data, provenance Manual, confidence 0.5, with no label and meaning
  unknown (region ids 1252 and 1253). Bank context is recorded in the evidence text because Data
  does not accept execution views. No ROM byte changed. No other region changed.
- Prior ambiguity is preserved. The L-015 rejection stands as an evidence claim, and the
  code-vs-data role remains unproven. leads.json L-015 gained a user_directed_fallback record and
  a pass entry.
- Checkpoint pass-004-fallback: ok, byte_identical true (analysis/pass-004-fallback-verification.json).
  maps/current.json = maps/pass-004-fallback.json.
- Record: analysis/handoffs/manual-fallback.md.
- Classification: 393230 / 393232 evidence-backed, plus 2 bytes as a manual user-directed fallback.
  0 bytes unknown. Whole-ROM evidence-backed classification is NOT complete.
- Next: final commentary acceptance on pass-004-fallback. The map changed, so the commentary must
  be rebased onto it, followed by the final review, the strict editorial audit and fresh
  byte-identical verification.

### 2026-11-18 — Final acceptance on pass-004-fallback: complete (terminal handoff updated)

- Commentary worker (analysis/handoffs/final-editorial-pass-004.md):
  - Preserved the pass-003 commentary at analysis/round-4/commentary/ (identical copy).
  - Ran a fresh prepare and carried all review credit over (analysis/tools/round-004-commentary/carry.py).
  - Gave the 2 bytes honest manual-fallback notes with no label.
  - Updated uncertainties, progress and editorial-audit.
- The coordinator reran `commentary.py audit` and it passed: ok, byte_identical true, 1232 regions and
  1937 units reviewed, editorial_audit true.
- ROM, map and state hashes were confirmed unchanged after the commentary pass.
- archive-options.json was updated. All analysis/ and commentary/ Markdown is accounted for. The
  archive was not run.
- Classification: 393230 bytes evidence-backed plus 2 bytes by manual user-directed fallback (0 unknown).
- Terminal handoff: analysis/handoffs/terminal.md, which has an update section.
