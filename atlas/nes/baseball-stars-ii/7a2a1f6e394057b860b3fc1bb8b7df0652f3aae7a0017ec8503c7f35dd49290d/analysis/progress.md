# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 / disassembly: BLOCKED (environment)

- Date: 2026-11-14 (round 1, pass-001).
- Every Bash invocation (including the trivial `echo ok`) fails before execution with:
  `apply-seccomp: write /proc/self/setgroups (nested userns is capability-restricted; caller must provide CAP_SYS_ADMIN): Permission denied`
- Consequence: baseline, serve and checkpoint (all invoked via `python3 run.py`) cannot run. No romatlas toolkit command could be executed.
- No workarounds outside the sandbox were attempted. input/game.nes, the toolkit, and all other workspace files were left untouched; only this section was appended.
- No classification exists. ROM classification: 0.00% (unknown counts not computable without tooling).
- No checkpoint, no findings, no leads, no commentary produced.
- Resume instruction: rerun round 1 disassembly (pass-001) from scratch once shell execution works in the sandbox.

## Coordinator state (goal.md orchestration)

- Round: 1. Current phase: disassembly (not started successfully; BLOCKED by environment).
- Input checkpoint: none (no baseline). Output checkpoint: none.
- Verification: none possible. Leads consumed/produced: none.
- Commentary and redirect phases: not started (they depend on a disassembly checkpoint).
- The coordinator's own shell hit the identical seccomp error, so this is not worker-specific.
- archive-options.json: not written; terminal handoff has not been reached (blocked, not terminal).
- Resume: once shell commands work in the sandbox, restart at round 1 disassembly
  (prompts/disassembly.txt), then commentary, then redirect. Do not treat this as terminal incomplete.

## Coordinator resume (2026-11-15)

- Shell execution now works (seccomp blocker cleared; `df` and toolkit binary checks succeed).
- Round 1 restarted: phase = disassembly (pass-001), launched as a separate worker.
- Input checkpoint: none (baseline to be established by the worker).
- Interrupted (worker context ended ~01:25) before any map.annotate: state.json still has 0 regions,
  no pass-001 checkpoint. Left: analysis/gameplay.md (game identified, menu routes), scratch tooling
  (rpc.py/daemon.py socket client, flow.py, observed.json execution export, PNG captures) and a live
  daemon (pid 1213778, /tmp/bs2rpc.sock) wrapping `run.py serve` (pid 1213781, 35k frames in).

## Coordinator resume #2 (2026-11-15 03:52)

- Round 1, phase = disassembly (pass-001) relaunched as a fresh separate worker that resumes from the
  scratch frontier above (reuse live daemon/observed.json; do not reinitialize the baseline).

## Round 1 / disassembly pass-001 (completed 2026-11-15)

- Resumed the interrupted worker's live daemon (scratch/daemon.py -> `run.py serve`, store analysis/state.json);
  reused its scratch flow analysis and observed export (scratch/observed.json, now 30,483 observed instruction starts).
- Gameplay added this pass: league creation, standings, simulated league games, LEAGUE CHAMPION ending, VIEW TEAM,
  MAKE TEAM, RELEASE/ACQUIRE PLAYER, 1P vs CPU fielding and TIME menu, extra innings and GAME OVER via RAM patch
  ($0497/$0499). Routes: analysis/scenarios.json. Gameplay notes: analysis/gameplay.md.
- Map rebuilt through map.annotate (scratch/annotate.py --data): 2,953 regions.
- Checkpoint: maps/pass-001.json, generated/pass-001; verification analysis/pass-001-verification.json:
  ok=true, byte_identical=true, map_sha256 6a6059573e49e4d8a47bfaf36b8bd6ab5f91ac8da43f131c2c7e7404105682ba.

ROM classification: 91.90% (361379 / 393232 bytes classified; 31853 bytes unknown).

- By kind/provenance: Code Observed 48,621; Code Static 22,457; Data Static 159,229 (includes iNES header 16,
  CPU vectors 6, inline call parameters, 14 $C316 jump tables, generic pattern data); Image (CHR ROM) 131,072.
- Execution coverage (separate metric): 30,483 observed ROM instruction starts; 0 RAM-executed instructions.
- Per-bank PRG bytes (code / data / unknown):
  b0 2243/5325/624, b1 2750/5014/428, b2 6077/671/1444, b3 4793/1508/1891, b4 0/7584/608, b5 1184/6504/504,
  b6 0/7681/511, b7 0/7807/385, b8 3543/4212/437, b9 3092/3649/1451, b10 4416/3284/492, b11 4555/1792/1845,
  b12 7449/651/92, b13 6497/1595/100, b14 1357/6626/209, b15 1332/5141/1719, b16 0/7857/335, b17 0/6869/1323,
  b18 0/7519/673, b19 0/6805/1387, b20 0/7899/293, b21 0/6984/1208, b22 0/7402/790, b23 0/6507/1685,
  b24 3433/977/3782, b25 4897/1201/2094, b26 0/7520/672, b27 0/6528/1664, b28 0/7437/755, b29 0/6528/1664,
  b30 6932/677/583, b31 6528/1459/205. Header 16 data; CHR 131072 image, 0 unknown.
- Evidence and methods: analysis/investigation.md (round 1 section). Leads produced: L001-L006 (analysis/leads.json);
  none consumed (file was empty).
- Toolkit: one finding, TI-001 (missing feature: no aggregate data-read coverage; tracing ~9x slowdown); not blocking.
  See analysis/toolkit-issues.md.
- Status: incomplete (31,853 unknown bytes). Handoff: analysis/handoffs/round-001-disassembly.md.

## Coordinator (2026-11-15) round 1 transition

- Disassembly pass-001 accepted (verified). Percentage printed to user: 91.90% (31,853 unknown).
- Round 1, phase = commentary, input checkpoint pass-001, handoff analysis/handoffs/round-001-disassembly.md.

## Round 1 commentary (2026-11-15) - INTERRUPTED

- Prepared commentary/ from pass-001 (byte-identical baseline). Source rendered by commentary/tools/render.py
  from before/ + commentary/notes/*.py; fresh verify ok/byte_identical (commentary/verification.json); reading
  copy commentary/reading-after.asm refreshed.
- Covered: global labelling/symbols for all source; banks 30 and 31 (parts), bank 0 entry explained; all
  classified game-mode handlers named. Remaining: 692 routines in banks 1-3, 5, 8-15, 24, 25 (+ bank 31
  $E371-$F2DD, bank 0 $8200+) still mechanical. review.json 1,094 reviewed / 2,655 pending.
- Leads: L007-L009 added, L001/L003 annotated. Toolkit: TI-002 added (missing feature).
- Handoff: analysis/handoffs/round-001-commentary.md. Classification unchanged (91.90%, 31,853 unknown).

## Coordinator (2026-11-15) round 1 commentary continuation

- Commentary worker #1 INTERRUPTED: verified (commentary/verification.json byte-identical), render pipeline
  commentary/tools/render.py + notes/*.py; 692 routines remaining (see analysis/handoffs/round-001-commentary.md).
  Leads produced: L007-L009; L001/L003 updated. Not advancing to redirect per commentary gate.
- Relaunching commentary continuation worker #2 on the remaining ranges.

## Coordinator resume #3 (2026-11-15 08:51) round 1 commentary

- Commentary worker #2 INTERRUPTED (~05:44) without updating handoff. Its output: notes 70_bank31_read.py,
  71_bank00_read.py, 72_bank01_read.py, 73_bank02_read.py, 74_bank03_read.py; source re-rendered,
  commentary/verification.json byte-identical at 05:44.
- Remaining mechanical routines ("Purpose not yet established" count in rendered source): bank 2: 2, bank 3: 4,
  bank 5: 18, bank 8: 56, 9: 53, 10: 51, 11: 52, 12: 99, 13: 84, 14: 21, 15: 14, 24: 45, 25: 45 (544 total).
- Phase stays commentary (round 1). Launching continuation worker #3 on the remaining banks.

## Coordinator (2026-11-15 09:16) round 1 commentary worker #3 result

- Worker #3 reported COMPLETE at routine level: 0 mechanical summaries (was 544), review.json 3,749 reviewed / 0 pending,
  verify ok/byte_identical (09:15), reading-after.asm refreshed. Fixed prior names (BatterForm/PitcherForm,
  CallBank15_A29B, $BA02/$BA53 scroller targets, ScoreRun index). Leads produced L010-L012.
- Coordinator gate check: 268 address-named code labels (Loc<bank>_<addr>) remain (banks 2,24,25,10 mostly) and
  174 address-named Tbl labels; ~2,250 mechanical @skip/@loop locals. Naming gap -> resume commentary (worker #4,
  targeted naming of Loc/Tbl labels and key-branch locals), not redirect yet.

## Coordinator (2026-11-15 09:32) round 1 commentary ACCEPTED

- Worker #4: Loc labels 268->0, Tbl labels 186->0, 621 meaningful locals renamed (remaining 1,634 @-locals are in
  trivial display/menu helpers/loops; acceptable for round handoff, final audit revisits). Fresh verify ok/byte_identical,
  reading-after.asm refreshed. Lead produced L013. Handoff: analysis/handoffs/round-001-commentary.md (COMPLETE).
- Round 1, phase = redirect. Input: checkpoint pass-001 + commentary handoff; leads L001-L013.

## Coordinator resume #4 (2026-11-15 13:55) round 1 redirect

- Redirect worker (attempt analysis/redirect/r1-redirect-a) was INTERRUPTED (~09:32) with only groundwork:
  ledger.py (exact unknown complement), unknown-raw.json, refscan.py, refs.json. No redirect handoff.
- Phase stays redirect (round 1). Launching a redirect worker to resume r1-redirect-a from that frontier.
  Disk: 5.1G free; scratch 88M.

## Coordinator resume #5 (2026-11-16 00:58) round 1 redirect

- Found analysis/handoffs/round-001-redirect.md written (14:30, recommends disassembly round 2, leads L014-L020 ready,
  L021 open) but with unfilled placeholders (finalize.py never run, no progress section). After the handoff the worker
  continued an L021 time-out-menu probe (timeout.py 14:39; r1-observed.json updated 14:49) and was interrupted.
  rdaemon.py (pid 1249979, /tmp store copy) still alive.
- Phase stays redirect (round 1). Launching a redirect continuation worker to fold in the post-handoff probe results,
  finalize the handoff/proposals and append the redirect progress section.

## Round 1 redirect (2026-11-15) - COMPLETE (attempt analysis/redirect/r1-redirect-a)

- Input: checkpoint pass-001 (91.90%, 31,853 unknown), commentary handoff complete; resumed r1-redirect-a groundwork.
- Method: bank-aware static closure from newly established entries (game modes 16-19/30/31/36/37/40/41, FarCall 7/14,
  trampolines, FarCall Y=26 fall-through, JMP()/RTS dispatch tables), consumer-derived data formats (stadium map,
  screen records/metatile sets, ScreenDataTable), identity scans (stale code copies), referenced tables, text/grid residue;
  bounded traced runs (stadium picker + City/Country/Dome via picker cursor write, REMOVE TEAM, TRADE PLAYER via
  GameMode=36, RELEASE/VIEW TEAM/ACQUIRE) on a store copy.
- Result: 29,037 unknown bytes proposed (Code 17,007, Data 12,030); residual 2,816; 2,448 observed instruction starts in former
  unknown bytes, all inside Code proposals; 15 phase-1 Data regions to split (L014).
- Leads: produced L014-L020 and L022 (ready), L021 (open); superseded L001-L005, L007-L013; L006 updated. Scenarios S14-S20 added.
- Continuation (2026-11-16): post-handoff probes (time-out sub-steps, PLAYER SELECT, RELEASE+ACQUIRE ROOKIE/STAR) found
  0 observed starts in residual unknown or Data proposals; static review split 4 code islands (569 bytes) into L022.
- Handoff: analysis/handoffs/round-001-redirect.md. Recommendation: next: disassembly round 2.

## Coordinator (2026-11-16) round 1 redirect ACCEPTED -> round 2 disassembly

- Redirect continuation finished: handoff filled (29,037 proposed: Code 17,007 / Data 12,030; residual 2,816, or 2,247
  with L022), ready leads L014-L020, L022; L021 open. Recommendation disassembly round 2 accepted (ready batch).
- Round 2, phase = disassembly (pass-002). Input: checkpoint pass-001, analysis/handoffs/round-001-redirect.md,
  proposals analysis/redirect/r1-redirect-a/proposals.json (+ l022.json).

## Round 2 disassembly pass-002 (2026-11-16)

ROM classification: 99.56% (391505 / 393232 bytes classified; 1727 bytes unknown).

- Input: checkpoint pass-001 (91.90%, 31,853 unknown) + redirect batch r1-redirect-a (proposals.json, l022.json).
- Applied after independent review: L014-L020 (29,037 bytes) and L022 (569 bytes) -> 29,606 bytes; L014 split 15 phase-1
  Data regions (+1 region for L022 $9F83). Static supplement on L021 residue: 520 bytes (13 data runs, 2 small
  consumer-bounded routines). Nothing rejected; see analysis/investigation.md for checks.
- Map: 3,985 regions. Code 89,010 (Observed 53,697 / Static 35,313), Data 171,423, Image (CHR) 131,072, header 16 (Data).
- Gameplay: S21 ('?' speciality MAKE TEAM), S22 (VS 1P vs 2P, ERROR ON, port-2 input, into fielding): 0 fetches in
  residual unknown bytes; gameplay stopped (no substantial gain).
- Per bank classified/unknown (PRG banks of 8,192 bytes; header and CHR fully classified):
  b0 8184/8 | b1 8182/10 | b2 8132/60 | b3 7943/249 | b4 8192/0 | b5 8192/0 | b6 8192/0 | b7 8192/0 | b8 8153/39 | b9 8129/63 | b10 8106/86 | b11 8083/109 | b12 8169/23 | b13 8174/18 | b14 8180/12 | b15 8010/182 | b16 8192/0 | b17 8192/0 | b18 8192/0 | b19 8192/0 | b20 8192/0 | b21 8192/0 | b22 8192/0 | b23 8192/0 | b24 8088/104 | b25 7524/668 | b26 8192/0 | b27 8192/0 | b28 8192/0 | b29 8192/0 | b30 8125/67 | b31 8163/29
- Checkpoint maps/pass-002.json -> generated/pass-002; verification analysis/pass-002-verification.json: ok, byte_identical
  (map sha256 ebcf3314...). Residue list: analysis/pass-002-remaining.json (83 runs).
- Leads: consumed L014-L020, L022 (verified); updated L021 (open, 1,727 bytes), L006 (deferred); none produced.
- Toolkit: no new issues (TI-001 traced-frame slowdown observed again); analysis/toolkit-issues.md.
- Status: incomplete (1,727 unknown bytes). Handoff analysis/handoffs/round-002-disassembly.md.

## Coordinator (2026-11-16) round 2 transition

- Disassembly pass-002 accepted (pass-002-verification byte_identical). Percentage printed to user: 99.56%
  (391,505 / 393,232; 1,727 unknown). Leads consumed L014-L020, L022; L021 open; none produced.
- Round 2, phase = commentary. Input checkpoint pass-002, handoff analysis/handoffs/round-002-disassembly.md.

## Coordinator resume #6 (2026-11-16 05:41) round 2 commentary

- Found round-2 commentary worker interrupted (~02:11): round-001 commentary preserved in analysis/round-001/, fresh
  baseline prepared from pass-002 (01:48), notes 87-96 (round2 fixed/misc/banks 0,30,9,2,3,10,11,24,25) written,
  verification.json byte_identical at 02:11, but no round-2 entry in commentary/progress.md and no handoff.
- Phase stays commentary (round 2). Launching continuation worker to finish remaining scope and write
  analysis/handoffs/round-002-commentary.md.

## Coordinator (2026-11-16) round 2 commentary ACCEPTED -> redirect

- Worker: COMPLETE. 0 mechanical summaries, 0 address labels; review.json 4,068 reviewed / 0 pending; fresh verify
  ok/byte_identical; reading-after.asm refreshed. Remaining generated locals (888 @skip/381 @loop/238 @return) in
  documented helpers: same standard as round 1, deferred to final editorial audit. Leads: L023 produced, L021 updated.
  Handoff analysis/handoffs/round-002-commentary.md.
- Round 2, phase = redirect. Input: checkpoint pass-002 + round-002 commentary handoff; leads L021, L023, L006.

## Round 2 redirect (2026-11-16, attempt analysis/redirect/r2-redirect-a) - COMPLETE

- Input pass-002 (1,727 unknown in 83 runs). Proposal: **1,702 bytes** (Code/Static 1,128, Data/Static 574) plus 23 phase-1
  Data bytes reclassified to Code (bank 25 $A033-$A03C FarCall sequence; $B88C-$B89A routine tail). Residual 25 bytes
  in 6 runs (lone RTS bytes, b2 $9988, b30 $C0C0, b31 $F6C2), code versus data ambiguous.
- Leads: L024 produced (ready); L023 ready; L021 superseded/updated; L006 updated (VETERAN, trade completion, 6-team
  league visited: S23-S25; 0 fetches in unknown or proposed bytes).
- Handoff analysis/handoffs/round-002-redirect.md. Recommendation: next: disassembly round 3.

## Coordinator (2026-11-16) round 2 redirect ACCEPTED -> round 3 disassembly

- Redirect r2-redirect-a: 1,702 bytes proposed (Code 1,128 / Data 574; 23 Data->Code corrections), residual 25 bytes
  in 6 runs. Leads L023, L024 ready; L021 superseded; L006 deferred. Scenarios S23-S25 (trade, VETERAN, 6-team league).
- Round 3, phase = disassembly (pass-003). Input: checkpoint pass-002, analysis/handoffs/round-002-redirect.md,
  proposals analysis/redirect/r2-redirect-a/proposals.json.

## Round 3 disassembly pass-003 (2026-11-16)

ROM classification: 99.99% (393207 / 393232 bytes classified; 25 bytes unknown).

- Input: checkpoint pass-002 (99.56%, 1,727 unknown) + redirect batch r2-redirect-a (proposals.json, leads L023/L024).
- Applied after independent review (scratch/p3/review.py): 1,702 unknown bytes (Code/Static 1,128, Data/Static 574) + 23 phase-1
  Data bytes reclassified to Code (bank 25 $A033-$A03C, $B88C-$B88D, $B890-$B89A); phase-1 regions 0x03203B+21 and 0x03389C+22 split.
  Weak items accepted with reduced confidence (C18 0.6; C02/C31/C32/C33/D03 0.7). Rejected: none. D62 left unlabeled (mid-table).
- Map: 4,101 regions. Code 90,161 (Observed 53,697 / Static 36,464), Data 171,974, Image (CHR) 131,072.
- Per bank classified/unknown: b0 8192/0 | b1 8192/0 | b2 8185/7 | b3 8192/0 | b4-b8 8192/0 | b9 8191/1 | b10-b29 8192/0 | b30 8187/5 | b31 8180/12;
  header and CHR fully classified.
- Checkpoint maps/pass-003.json -> generated/pass-003; verification analysis/pass-003-verification.json: ok, byte_identical
  (map sha256 c36cb8af...). Residue: analysis/pass-003-remaining.json (6 runs, 25 bytes).
- Gameplay: none this pass (prior rounds: 0 fetches in residue; no substantial prospect).
- Leads: consumed L023, L024 (verified); L021 updated -> blocked (25-byte residue); none produced.
- Toolkit: no new toolkit issues found (TI-001, TI-002 existing); analysis/toolkit-issues.md.
- Status: incomplete (25 bytes unknown). Handoff analysis/handoffs/round-003-disassembly.md.

## Coordinator (2026-11-16) round 3 transition

- Disassembly pass-003 accepted (byte_identical). Percentage printed to user: 99.99% (393,207 / 393,232; 25 unknown).
  Leads consumed L023, L024; L021 blocked (25-byte residue); L006 deferred; none produced.
- Round 3, phase = commentary. Input checkpoint pass-003, handoff analysis/handoffs/round-003-disassembly.md.

## Coordinator resume #7 (2026-11-16 10:45) round 3 commentary

- No round-3 commentary output yet (commentary/ still holds round-2 baseline from pass-002). Disk 5.0G free; scratch 112M.
- Launching round-3 commentary worker: input checkpoint pass-003, handoff analysis/handoffs/round-003-disassembly.md.

## Round 3 commentary (2026-11-16)

- Baseline pass-003 (c36cb8af...). The round-2 commentary is preserved under analysis/round-002/commentary, and the
  fresh baseline carries all notes by physical range.
- All 1,725 changed bytes are documented (named routines/data, role notes), and the 25 unknown bytes (6 runs) have
  inspection notes. Generated @skip/@loop labels went from 888/381 to 0/0. New RAM aliases are in place.
- commentary.py verify: byte_identical true; reading-after.asm refreshed; review.json 4,107/4,107 reviewed, 1,967 units.
- Leads: L025 produced (bank 2 $8C52/$8CA0 inline words classified as code); L021 updated.
- Status COMPLETE. Handoff analysis/handoffs/round-003-commentary.md.

## Coordinator (2026-11-16 11:10) round 3 commentary ACCEPTED -> redirect

- Gate check: verification.json ok/byte_identical (11:06); reading-after.asm refreshed (11:04); 0 "Purpose not yet"
  summaries; generated @skip/@loop 0 (242 @return remain, each on a return instruction). review.json 4,107/4,107.
- Round 3, phase = redirect. Input: checkpoint pass-003 + analysis/handoffs/round-003-commentary.md; leads L021 (blocked,
  25-byte residue), L025 (open, inline words in code), L006 (deferred gameplay).

## Coordinator resume #8 (2026-11-16 15:42) round 3 redirect

- Round-3 redirect worker (attempt analysis/redirect/r3-redirect-a) INTERRUPTED (~11:42) without handoff. Frontier:
  proposals.json draft (17 of 25 residual bytes proposed as Code under L021; L025 split of bank 2 $8C2A-$8D1E with 4
  code->data bytes; 8 bytes remaining), fidelity3.json, rtsidiom.json, refs3.json, gameplay frames (end_*, g*, l6/lp*, nl*)
  from L006 league-progression probes; rdaemon.py (pid 1303183) + serve (pid 1303185, store /tmp/bs2_r3_store.json) alive.
- Phase stays redirect (round 3). Launching continuation worker to resume r3-redirect-a from that frontier. Disk 4.9G free.

## Round 3 redirect (2026-11-16, attempt analysis/redirect/r3-redirect-a, resumed after interruption)

- Checkpoint pass-003 (c36cb8af...), unchanged. The attempt resumed from its frontier with no new baseline and no replayed
  scans.
- Proposals (r3-redirect-a/proposals.json):
  - 17 of 25 unknown bytes become Code/Static: C01 bank 30 $C0C0 (5 bytes) and C02 bank 31 $F6C2 (12 bytes), both
    under L021.
  - F01 under L025 corrects 4 Code bytes to Data: the bank 2 inline words at $8C52 and $8CA0.
  - Residual: 8 bytes ($80DA-$80DB, $9988-$998B and $9B48 in bank 2; $A165 in bank 9). They have a rejection review in
    the handoff.
- Gameplay: a natural 6-team league was played to completion, ending on the LA SUPERSTARS league-champion screen
  (scenarios S26-S27).
  - 6,961 instruction starts were observed and 21 Static Code regions gained observed starts.
  - There were 0 starts in the residue.
  - L006 is verified.
- Leads: L021 and L025 are ready; L006 is verified.
- Daemon stopped, /tmp scratch removed, 5.0G free.
- Status COMPLETE; handoff analysis/handoffs/round-003-redirect.md. Next: disassembly round 4.

## Coordinator (2026-11-16) round 3 redirect ACCEPTED -> round 4 disassembly

- r3-redirect-a: proposals C01 (b30 $C0C0, 5 B Code), C02 (b31 $F6C2, 12 B Code), F01 (b2 $8C52/$8CA0 4 B Code->Data).
  Residual 8 bytes in 4 runs with rejection review in analysis/handoffs/round-003-redirect.md. Leads L021, L025 ready;
  L006 verified (6-team league to LEAGUE CHAMPION ending; no credits sequence; S26-S27).
- Round 4, phase = disassembly (pass-004). Input checkpoint pass-003 + analysis/handoffs/round-003-redirect.md.

## Round 4 disassembly pass-004

ROM classification: 100.00% (393224 / 393232 bytes classified; 8 bytes unknown)

- Status: incomplete. 8 bytes in 4 runs remain unknown, even though the percentage rounds to 100.00%.
- Input: pass-003 (c36cb8af...) plus the r3-redirect-a batch.
- Checkpoint: maps/pass-004.json -> generated/pass-004.
  - analysis/pass-004-verification.json: ok, byte_identical.
  - Map sha256: 077d58cd3516eacb079e709c2c6466478f5edea3cdb5afa4d9efbe6213faf837.
  - analysis/state.json is identical to maps/current.json, with 4,107 regions.
- Applied after independent review (scratch/p4/plan.py, apply.py):
  - C01: b30 $C0C0-$C0C4, 5 bytes, Code/Static, conf 0.65.
  - C02: b31 $F6C2-$F6CD, 12 bytes, Code/Static, conf 0.6.
  - F01: b2 $8C52-$8C53 and $8CA0-$8CA1, 4 bytes changed from Code to Data (inline words); region 0x004C3A+245 was split.
  - Reachability for the new code was not observed; it is not claimed unused.
- Observation merge from the r3 S26-S27 natural-input session:
  - 11 Static Code regions (318 bytes) became Observed.
  - 10 partly observed regions got a note and stay Static.
- Totals: Code 90,174 (Observed 54,015 / Static 36,159); Data 171,978 (includes the 16-byte header); Image (CHR) 131,072.
- Per bank: b2 8185/7 (classified/unknown), b9 8191/1. Banks 30 and 31 are now 8192/0. Every other PRG bank is 8192/0.
  The header and CHR are fully classified.
- Remaining unknowns (analysis/pass-004-remaining.json): 0x0040EA+2, 0x005998+4 and 0x005B58+1 in bank 2, and 0x012175+1
  in bank 9. The rejection review in analysis/handoffs/round-003-redirect.md is unchanged.
- Leads:
  - L025 is verified and consumed.
  - L021 is blocked: the 8-byte residue needs a reference, a fetch or a discriminating structure.
  - L006 is updated with the merge note.
  - No new leads.
- Gameplay: not run. S1-S27 already reach the league-champion ending and show 0 fetches in the residue.
- Toolkit: no new toolkit issues found. The existing entries TI-001 to TI-003 are in analysis/toolkit-issues.md.
- Handoff: analysis/handoffs/round-004-disassembly.md.

## Coordinator (2026-11-16) round 4 transition

- Disassembly pass-004 accepted (byte_identical, map 077d58cd...). Percentage printed to user: 100.00% rounded
  (393,224 / 393,232; 8 unknown). L025 consumed; L021 blocked (8 bytes); 318 bytes Static->Observed merged.
- Round 4, phase = commentary. Input checkpoint pass-004, handoff analysis/handoffs/round-004-disassembly.md.

## Round 4 commentary

Status: COMPLETE (current-source readability). Handoff: analysis/handoffs/round-004-commentary.md.

- Checkpoint: maps/pass-004.json = maps/current.json = analysis/state.json (077d58cd...). commentary/ was re-prepared
  from it (review scaffold of 4,111 entries).
- The round-3 commentary is preserved whole under analysis/round-003/commentary. Notes are carried by (bank, CPU
  address), and the carried source verified byte-identical before any edit.
- Covered:
  - C01 LoadChrBankTemp (bank 30 $C0C0).
  - C02 AddRecordStrideTemp0C (bank 31 $F6C2; defective high-byte add documented).
  - F01 inline-word data notes (bank 2 $8C52/$8CA0), with the PitcherListDrawAlt prose rewritten.
  - The 21 evidence-changed regions, re-checked with no contradiction.
  - The 8 residual unknown bytes, whose notes were re-checked.
  - Ten address-only trampoline labels renamed after their targets (e.g. CallStadiumPicker, FarDrawTeamEmblemBoard).
- Verification: `commentary.py verify` gives byte_identical true (fresh), and reading-after.asm was refreshed.
  review.json has 4,111 reviewed / 0 pending, 1,969 units. There are 0 mechanical summaries and no address-only
  labels. editorial_audit is pending until final acceptance.
- Leads: an L021 attempt record was added (no new evidence). There are no new leads.

## Coordinator (2026-11-16 16:05) round 4 commentary ACCEPTED -> redirect

- Gate check: commentary/verification.json ok/byte_identical (16:00), manifest map 077d58cd... (pass-004); reading-after.asm
  refreshed 16:00; 0 "Purpose not yet" summaries; 0 address-only Loc/Tbl labels; remaining @skip_/@loop_ labels are descriptive.
  Round-3 commentary preserved at analysis/round-003/commentary. Handoff analysis/handoffs/round-004-commentary.md.
- Round 4, phase = redirect. Input: checkpoint pass-004 + round-004 commentary handoff; leads L021 (blocked, 8 bytes), L006.

## Round 4 redirect

Status: COMPLETE. Attempt analysis/redirect/r4-redirect-a. Handoff: analysis/handoffs/round-004-redirect.md.

- Input: checkpoint pass-004 (077d58cd...), 8 unknown bytes in 4 runs. The ROM, map, state and source are unchanged.
- Coordinator question 1, stale copies:
  - The r3 copy scans did cover these exact windows. A new fuzzy context search found one relevant copy: the $80B0
    routine's bank-3 twin C06, which ends on a single RTS followed by data. So the extra `60 60` at $80DA is not part
    of the routine.
  - No other run has a related context anywhere in PRG.
- Coordinator question 2, padding convention: every classified lone `$60` has a reference, a fetch or a consumer, and
  the ROM's inter-region fill is `$00`. The standard has been applied consistently, and there is no `$60`-fill
  convention.
- Coordinator question 3, runtime: a data-read probe (10 snapshots x 4 runs x 120 frames) found 0 reads and 0
  executions. Its positive control, the RTS dummy read of $A165, was detected.
- Result: no proposals. L021 stays blocked; a redirect-004 attempt was recorded.
- Recommendation: terminal incomplete with 8 residual bytes (b2 $80DA-$80DB, $9988-$998B, $9B48; b9 $A165). Final
  editorial audit and verification are still required.

## Coordinator (2026-11-16) TERMINAL HANDOFF - justified terminal INCOMPLETE (not ROM completion)

- Round 4 redirect (analysis/redirect/r4-redirect-a, handoff analysis/handoffs/round-004-redirect.md): no proposal.
  Coordinator rejection-review check, applying evidence-policy.md: the review covers each run with its claim, the decisive
  artifacts (refs3.json, rtsidiom.json, fuzzy4.json/copies3.rerun.txt, readprobe4.json), what each rules out, and the
  competing reading. The prerequisites are actual, not inherited:
  - no reference of any kind (including RTS-trick pushes);
  - the only real copy (bank 3 $B0B3-$B0DD) ends on a single RTS;
  - the ROM's padding uses $00, and every classified lone or paired $60 has a reference or consumer;
  - the data-read trace over 10 snapshots found 0 reads or fetches, and the method was shown able to see these bytes.
  This standard is the same one used to accept C01/C02 (consumer-bounded plus a contract or sibling link).
  - No corrected inference or new discriminating test was found. The review is sound, so the workflow stops incomplete
    rather than cycling on unchanged leads.
- Final checkpoint: maps/pass-004.json = maps/current.json = analysis/state.json (sha256 077d58cd...).
  Verification: analysis/pass-004-verification.json ok/byte_identical.
- ROM classification: 100.00% rounded (393,224 / 393,232 bytes classified; 8 bytes unknown). Exact residual runs
  (analysis/pass-004-remaining.json):
  - b2 0x0040EA-0x0040EB ($80DA, 2 B)
  - b2 0x005998-0x00599B ($9988, 4 B)
  - b2 0x005B58 ($9B48, 1 B)
  - b9 0x012175 ($A165, 1 B)
- Commentary/readability: round 4 COMPLETE on pass-004 (analysis/handoffs/round-004-commentary.md).
  - commentary/verification.json is ok/byte_identical, and reading-after.asm was refreshed.
  - review.json: 4,111 reviewed / 0 pending.
  - 0 mechanical summaries and 0 address-only labels.
  - editorial_audit is pending for final acceptance/archive (not run here).
- Leads: analysis/leads.json. L021 is blocked (the 8 bytes). L006 is verified (gameplay through the league-champion
  ending; no credits sequence). No ready leads. Gameplay checklist: analysis/scenarios.json S1-S27.
- archive-options.json written:
  - documents: 12 round handoffs analysis/handoffs/round-00{1..4}-{disassembly,commentary,redirect}.md.
  - omitted_documents: 15 superseded per-round commentary snapshots, analysis/round-00{1,2,3}/commentary/
    {AGENTS,prompt,workflow,progress,uncertainties}.md. They are copies of tool boilerplate, or are superseded by the
    cumulative commentary/progress.md and commentary/uncertainties.md.
  - Every unrecognized Markdown file in analysis/ and commentary/ is accounted for, checked against the archive.py
    known_docs filter.
- No archive, cleanup or publish was executed. Workflow termination is NOT whole-ROM completion; ROM classification
  is not marked complete.

## Workspace rehydrated from archive (2026-11-17)

- The original workspace was deleted by batch `--cleanup` after an archive that skipped the final
  editorial audit (batch runner bug, since fixed). This workspace was rebuilt from
  atlas/nes/baseball-stars-ii/7a2a1f6e…/ plus the original ROM: fresh initialize, archived analysis/ and
  maps/current.json, `commentary prepare` on the identical final map (manifest matches the archive), the
  archived review.json/progress.md/uncertainties.md, and commentary/source restored exactly from editorial.json.
- Verified: commentary source byte-identical; editorial status identical to the archive; re-archiving this
  workspace to a probe directory reproduces every archived artifact hash. archive-options.json restored.
- NOT recoverable (never archived): analysis/leads.json (reset to the empty template; the final state was
  L021 blocked on the 8 residual bytes, L006 verified, no ready leads), traces/snapshots, intermediate
  maps/pass-00N.json checkpoints, analysis/pass-004-*.json, analysis/redirect/ evidence, and the
  commentary/tools + notes render pipeline. Edit commentary/source directly; do not expect the old renderer.
- Current state: pass-004 map, 8 unknown bytes (b2 $80DA 2 B, $9988 4 B, $9B48 1 B; b9 $A165 1 B),
  4,111/4,111 regions reviewed, 1,356 Code regions without reviewed units, editorial_audit pending.

## Manual residual fallback (2026-11-17) - USER-DIRECTED, not evidence-backed classification

- Authorization: the user explicitly authorized a batch-only exception. If at most 10 physical ROM bytes remain
  unclassified after the terminal incomplete handoff, mark them as generic Data with unknown meaning, Manual
  provenance. This overrides the further-structural-proof rule for these bytes only. It is not an observed or
  statically proven data role.
- Complement re-verified first. A fresh server loaded analysis/state.json (pass-004, sha256 077d58cd...): 4,107 regions,
  and the unclassified complement was exactly 8 bytes in 4 runs, matching the authorized list.
  - physical 16618 (+2) = 0x0040EA-0x0040EB, bank 2 CPU $80DA-$80DB
  - physical 22936 (+4) = 0x005998-0x00599B, bank 2 CPU $9988-$998B
  - physical 23384 (+1) = 0x005B58, bank 2 CPU $9B48
  - physical 74101 (+1) = 0x012175, bank 9 CPU $A165
- Prior ambiguity, preserved in analysis/handoffs/round-004-redirect.md, lead L021 and the source notes: three runs
  are RTS-valued `$60` bytes after a final RTS or table, and $9988 is an LDA #$80 / LDA #$00 fragment that
  CpuDefenseChangeDrive overwrites at once. Across rounds 1-4 none of these bytes had a reference, consumer, sibling
  copy, read or fetch, and there was no `$60` padding convention. Code versus data remained undecided.
- Action: map.annotate added regions 5155-5158, all kind Data, provenance Manual, confidence 1.0. Their evidence and
  comment fields mark them "USER-DIRECTED FALLBACK ... not evidence-backed". They have no label and no invented
  semantics, and the ROM is unmodified. map.validate passed. The annotation was saved to analysis/state.json and
  exported to maps/current.json (sha256 d09c3179...).
- Checkpoint: `run.py checkpoint pass-005` ok/byte_identical (analysis/pass-005-verification.json, maps/pass-005.json,
  generated/pass-005). The unclassified complement is now 0 bytes.
- Classification report (kept separate):
  - Evidence-backed (Observed/Static): 393,224 / 393,232 bytes (99.998%).
  - User-directed Manual fallback: 8 bytes.
  - Unknown: 0.
  - Whole-ROM evidence-backed classification is NOT complete. The 8 fallback bytes are not observed or proven data.
- Commentary: round-4 commentary is preserved at analysis/round-004/commentary/. A fresh prepare was run on pass-005,
  carrying source, review credit and notes by physical range (commentary/progress.md "Round 5 carry-over").
  - verify ok/byte_identical; reading copy refreshed.
  - Next: a commentary worker finishes the readability/unit backlog (1,356 Code scaffold entries without units), then
    final review and editorial audit.

## Round 5 commentary (2026-11-17) ACCEPTED -> final review / editorial audit

- Handoff analysis/handoffs/round-005-commentary.md.
- Coordinator gate check: verify ok/byte_identical; 4,111/4,111 entries reviewed; 0 Code entries without units
  (1,484 units added, 3,453 total); 105 previously uncommented code blocks now have comments.
- Label fix: the bank-25 pitcher/batter card labels were swapped, and the related bank-18/20 records were renamed to
  match.
- editorial_audit pending. The 8 manual-fallback site headers are intact.
- Next phase: final review plus strict editorial audit (separate worker).

## Coordinator (2026-11-17) FINAL - final audit PASSED; terminal handoff with user-directed fallback

- Final-acceptance worker report: analysis/handoffs/round-005-final-audit.md.
  - `commentary.py audit`: ok, byte_identical, 4,111 regions, 3,453 units, editorial_audit true
    (commentary/audit.json).
  - The coordinator re-ran `commentary.py verify` independently: ok/byte_identical.
  - archive.editorial_status: complete True, no reasons.
- Fixes in the final pass:
  - PendingPitcherChange role corrected.
  - Card-label consistency (ScreenBatterCard, MetatilesPlayerCard).
  - About 130 truncated summaries restored; 904 fragment summaries and 4,078 entry summaries tightened.
  - 33 hyphenation artifacts and 18 ram.inc wording fixes.
- Classification, reported separately:
  - 393,224 bytes evidence-backed.
  - 8 bytes user-directed Manual fallback (regions 5155-5158).
  - 0 unknown.
- Whole-ROM evidence-backed classification is NOT complete.
- Final conclusions: analysis/handoffs/round-005-terminal.md.
- archive-options.json:
  - 16 handoffs are selected.
  - 5 analysis/round-004/commentary boilerplate or superseded documents are omitted.
  - A manual_fallback record is included.
  - All Markdown files are accounted for.
- No archive, delete, publish or toolkit edit was performed.
