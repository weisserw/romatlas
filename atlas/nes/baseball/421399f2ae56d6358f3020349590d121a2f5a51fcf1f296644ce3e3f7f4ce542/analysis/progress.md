# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## 2026-11-14 — Round 1, phase disassembly — status: BLOCKED

- Blocker: the shell sandbox cannot execute commands, so the romatlas toolkit
  cannot be run. Every Bash invocation (including a trivial `true`) fails with:
  `apply-seccomp: write /proc/self/setgroups (nested userns is capability-restricted; caller must provide CAP_SYS_ADMIN): Permission denied`
- Work done: none. No baseline, classification, checkpoint or handoff was produced;
  input/game.nes untouched.
- Classification completion: not measured (0% classified; baseline never ran).
- Resumable: yes. Re-run round 1 disassembly from the start once the sandbox
  allows command execution.

### Coordinator note (2026-11-14)
- Current phase: round 1 disassembly (not started; blocked). Commentary and
  redirect not started. No lead IDs consumed or produced.
- The coordinator shell fails with the same error, so no phase can proceed and
  archive-options.json was not written (archive.py could not be inspected by
  shell; terminal handoff not reached).
- Resume point: launch round 1 disassembly worker from scratch once Bash works.

## 2026-11-18 — Coordinator resume

- Shell execution now works (sandbox blocker of 2026-11-14 cleared). State is
  still empty (no regions, no baseline, no checkpoints, no leads).
- Round 1, phase disassembly: STARTING (worker launched sequentially). Commentary
  and redirect pending.

## 2026-11-18 — Round 1, phase disassembly — status: INCOMPLETE (easy gains taken)

- Baseline: created this pass (`run.py baseline`, verified byte-identical; maps/baseline.json).
- Checkpoint: **pass-001** — maps/pass-001.json, generated/pass-001,
  analysis/pass-001-verification.json: ok=true, byte_identical=true,
  map sha256 3dd8ead630188f9f9de65ebc563e3ae61fe60b7e288680e3585b4ed3fde14fc8.
  analysis/state.json saved; maps/current.json exported and validated.
- ROM classification: 98.20% (24149 / 24592 bytes classified; 443 bytes unknown).
- By kind/provenance: Code Observed 8939, Code Static 1942, Data Static 4991
  (incl. 314 inline-argument bytes, header 16, vectors 6), Padding 85, Image (CHR) 8192.
- Per bank: header 16/0 unknown; PRG 8K bank 0 ($C000-$DFFF) 7986 classified / 206 unknown;
  PRG 8K bank 1 ($E000-$FFFF) 7955 / 237 unknown; CHR 8192 / 0.
- Execution coverage (separate metric): 4264 observed opcode addresses
  (traces/r1-coverage-offsets.json); 1942 code bytes static-only.
- Scenarios: startup/title, 1P team select, batting, fielding, artificial jump to
  bottom 9th -> GAME SET -> title, 2P start (analysis/scenarios.json).
- Leads produced: L-001..L-007 (analysis/leads.json); none consumed (none existed).
- Toolkit: TK-001 suspected bug (NES emulator.input ignores documented button/pressed
  form silently), TK-002 performance limitation (tracing ~10x slower) — see
  analysis/toolkit-issues.md. Neither blocking.
- Handoff: analysis/handoffs/round-1-disassembly.md

### Coordinator (2026-11-18)
- Classification percentage reported to user after round 1 disassembly: 98.20% (443 unknown).
- Round 1, phase commentary: STARTING (worker launched sequentially). Input checkpoint pass-001.

## 2026-11-18 — Round 1, phase commentary — status: COMPLETE (current source)

- Prepared commentary/ via toolkit commentary.py prepare from pass-001 (baseline verified).
- All 456 review entries reviewed (270 units); 0 address-only labels; RAM/constants
  symbolized; inline args rendered symbolically; 17 Unknown ranges annotated honestly.
- Verification: commentary.py verify ok, byte_identical true (game.asm sha256 d3e2436a...).
  Reading copy: commentary/reading-after.asm. Editorial audit pending (final acceptance).
- Corrections: $62 strikes/$63 balls/$64 outs; $0598.. = runner ObjRole entries.
- Structural questions SQ-1..SQ-4 for redirect (EAF8 inline args mis-decoded as code;
  unreferenced coherent routines; consumer-backed tables inside Unknown ranges).
- Handoff: analysis/handoffs/round-1-commentary.md. No new toolkit issues found.

### Coordinator (2026-11-18)
- Round 1 commentary accepted as phase-complete (all classified source reviewed; verify byte-identical;
  reading copy commentary/reading-after.asm). Noted lighter line-level comments in sound driver,
  runner/fielder decision branches and CORDIC helpers — carry to final acceptance.
- Structural questions SQ-1..SQ-4 handed to redirect.
- Round 1, phase redirect: STARTING (worker launched sequentially). Input checkpoint pass-001.

## 2026-11-18 — Round 1, phase redirect (attempt r1-a) — status: READY LEADS

- Input pass-001 (443 unknown). Static investigation of all 17 ranges + SQ-1..SQ-4;
  ledger analysis/redirect/r1-a/ledger.json.
- Ready: L-001..L-005 updated with exact physical pieces (Static, reachability unknown),
  L-008 map correction EAF8-EAFE inline args (0 net), L-010 commentary corrections.
  Proposed 442 bytes -> 99.996%. Residual: E5F4 lone RTS (L-009 blocked, rejection review).
- New facts: $CB = camera frame offset (0/6) from CameraTables; extra innings do not exist
  (scenario unavailable); EA05 missing from inline-contract list (artificial test passed).
- Map/state/ROM unchanged. Handoff: analysis/handoffs/round-1-redirect.md

### Coordinator (2026-11-18)
- Round 1 redirect complete: ready leads L-001..L-005, L-008 (map correction), L-010 (commentary);
  residual L-009 ($E5F4, 1 byte) with rejection review in analysis/handoffs/round-1-redirect.md.
- Round 2, phase disassembly: STARTING. Input checkpoint pass-001; consumes L-008, L-001..L-005.

## 2026-11-18 — Round 2, phase disassembly — status: INCOMPLETE (1 unknown byte)

- Restored analysis/state.json (pass-001 map) into a fresh serve session; baseline untouched.
- Verified and applied ready leads L-008, L-001, L-002, L-003, L-004, L-005 (each boundary
  re-checked: preceding unconditional transfer / data end, exact official-opcode tiling,
  branch targets in-slice or accepted entries via analysis/redirect/r1-a/tilecheck.py,
  inline contracts re-derived for EA05, C963, E29B). Script: analysis/tools-r2/apply_leads.py.
- All new regions: Static provenance, confidence 0.9, context prg_banks [0,1,0,1];
  evidence text records "reachability unknown (not claimed unused)"; none labeled Observed.
- L-008 map correction: region 0x2AEE+44 split (EAF8-EAFE now inline Data; RTS at EAFF restored);
  EA05 registered as an inline-argument routine in the workspace traversal (inline.py/trav.py).
- No gameplay this pass: the only remaining unknown (E5F4) has no possible ROM-encoded
  reference, so play cannot produce classification evidence; scenarios unchanged.
- Checkpoint pass-002: maps/pass-002.json, generated/pass-002,
  analysis/pass-002-verification.json (ok true, byte_identical true,
  map sha256 9728112890a1cbfc86bd35b78461aa61daa6f023e13bd14e069188c183e02ef9).
  map.validate ok; state saved; maps/current.json exported.

ROM classification: 100.00% (24591 / 24592 bytes classified; 1 bytes unknown).

(Exact: 99.996%; rounded 100.00% does NOT mean complete — 1 byte unknown.)
- Per bank: header 16/0; PRG bank0 ($C000-$DFFF) 8192 classified / 0 unknown;
  PRG bank1 ($E000-$FFFF) 8191 / 1 unknown (0x2604 = $E5F4); CHR 8192 / 0.
- By kind: Code Observed 8939, Code Static 2278, Data Static 5097, Padding 85, CHR Image 8192.
- Lead outcomes: L-008, L-001..L-005 consumed/verified; L-009 not reopened (no new evidence);
  L-010 left for commentary; L-006/L-007 unchanged.
- Toolkit issues: none new this pass (existing TK-001, TK-002 in analysis/toolkit-issues.md).
- Handoff: analysis/handoffs/round-2-disassembly.md

### Coordinator (2026-11-18)
- Round 2 disassembly: classification reported to user: 100.00% rounded (24591/24592; 1 unknown, 0x2604 $E5F4).
- Round 2, phase commentary: STARTING. Input checkpoint pass-002; consumes L-010. Map changed vs round-1
  commentary baseline, so preserve round-1 commentary under analysis/round-1/ per prompt.

## Round 2 -- commentary (2026-11-18)
- Map changed vs round-1 baseline: round-1 commentary preserved in analysis/round-1/commentary/
  (+ analysis/round-1/tools-commentary-r1/); fresh baseline prepared from pass-002 (472 entries).
- Source re-rendered by analysis/tools-commentary-r2/apply.py; round-1 names/comments carried
  by address; 438 review entries carried, 34 reviewed anew; 619 units; editorial audit pending.
- New regions (30 formerly unknown + 0x2AEE split) named/commented; L-010 applied (consumed).
- Backlog completed: sound driver, CORDIC, runner/fielder decisions, pitch/bat/message tasks.
- Corrections: $82 PitchCaught, $66 HalfInningRuns, SetPpuAddrInline relative claim, sound
  effect meanings. Uncertainties: commentary/uncertainties.md (camera naming, HitPower role).
- Verification: commentary.py verify ok, byte_identical true (game.asm sha256 d0c097d9...);
  reading copy commentary/reading-after.asm refreshed; input/game.nes unchanged.
- Structural questions for redirect: E5F4 residual; $DF92-$DFC3 (InterceptBall tail + SquareByte) has no effective entry.
- Handoff: analysis/handoffs/round-2-commentary.md. Status: COMPLETE (current-source readability).

### Coordinator (2026-11-18)
- Round 2 commentary accepted phase-complete (472/472 reviewed; verify byte-identical; reading copy
  refreshed; round-1 commentary preserved in analysis/round-1/commentary/). L-010 consumed.
- Coordinator review of L-009 rejection review (round-1 redirect): consistent with evidence-policy
  ("unreferenced return opcode alone lacks structure"), but its reference search covered only the direct
  word F4E5. Concrete revised test returned to redirect: RTS-dispatch (entry-1 = $E5F3) words, split
  hi/lo address tables, task-entry/inline-arg encodings, and neighbouring-routine structure.
- Round 2, phase redirect: STARTING. Input checkpoint pass-002. Questions: L-009, SQ-R2-1..3.

## Round 2 -- redirect (2026-11-18), attempt r2-a
- Input pass-002 (24591/24592). Research: analysis/redirect/r2-a/. Map/state/ROM/source unchanged.
- L-009 ($E5F4): revised tests (entry-1/BE words, split hi/lo tables, inline/resume arithmetic,
  stack-built returns, JMP (InlinePtr), adjacent-RTS/padding convention; ROM pads with $FF) all
  negative -> kept Unknown; rejection review updated in the handoff.
- SQ-R2-2: $DF92-$DFC3 Code structurally sound; no effective entry (BEQ $DF90 always taken);
  L-011 = metadata-only evidence-text fix for region 648.
- SQ-R2-3: runtime check (natural snapshots) shows pitching uses the second camera record
  ($CB=6, nametable $2800) = infield; first record = full field. L-012 commentary renaming.
- Ready leads: L-011, L-012 (0 bytes each; no full cycle warranted on their own).
- Handoff: analysis/handoffs/round-2-redirect.md. Status: INCOMPLETE (1 byte residual, honest limit).

### Coordinator (2026-11-18) — rejection review assessment
- Inspected round-2 redirect rejection review for L-009 (0x2604/$E5F4) and its decisive artifact
  (analysis/redirect/r2-a/l009_tests.out per handoff). Scope covers every ROM dispatch encoding
  (direct, entry-1 task create, split tables, inline args, stack-built, indirect JMP); ROM padding
  convention is $FF; policy explicitly states an unreferenced return opcode alone lacks structure.
  Assessment: SOUND. No corrected inference/test remains; L-009 stays Unknown (terminal residual).
- Ready leads L-011 (map evidence-text correction region 648, 0 bytes) and L-012 (camera symbol
  inversion, commentary) are known editorial defects; apply them before terminal handoff.
- Round 3, phase disassembly: STARTING (apply L-011 only; checkpoint pass-003).

## Round 3 -- disassembly (2026-11-18), checkpoint pass-003
- Input pass-002; restored analysis/state.json (471 regions), all classifications preserved.
- L-011 applied via map.annotate (analysis/tools-r3/apply_l011.py): region 648 (0x1FA2-0x1FD3,
  $DF92-$DFC3) evidence text only; Code/Static/boundaries/views unchanged. Diff vs pre-pass state:
  only region 648 'evidence' field changed. L-011 consumed (verified).
- L-009 not reopened (no new evidence); L-012 left for commentary; no gameplay needed.
- Verification: map.validate ok; state saved; maps/current.json exported; checkpoint pass-003
  ok true, byte_identical true (map sha256 a60ef9e5ef1a4eed8f6de39ef26f41e6b7f3360e965fd7d8ae7401d2030ae2d0);
  input/game.nes sha256 unchanged.

ROM classification: 100.00% (24591 / 24592 bytes classified; 1 bytes unknown).

(Exact: 99.996%; rounded 100.00% does NOT mean complete — 1 byte unknown.)
- Per bank: header 16/0; PRG bank0 ($C000-$DFFF) 8192 classified / 0 unknown;
  PRG bank1 ($E000-$FFFF) 8191 / 1 unknown (0x2604 = $E5F4); CHR 8192 / 0.
- By kind: unchanged from pass-002 (Code Observed 8939, Code Static 2278, Data Static 5097, Padding 85, CHR Image 8192).
- Toolkit issues: none new this pass (existing TK-001, TK-002 in analysis/toolkit-issues.md).
- Handoff: analysis/handoffs/round-3-disassembly.md. Status: INCOMPLETE (1-byte terminal residual L-009).

### Coordinator (2026-11-18)
- Round 3 disassembly: classification reported to user: 100.00% rounded (24591/24592; 1 unknown 0x2604).
- Round 3, phase commentary: STARTING. Input checkpoint pass-003; consumes L-012.

## Round 3 -- commentary (2026-11-18), checkpoint pass-003
- Baseline changed (region 648 evidence only), so the round-2 commentary was preserved in
  analysis/round-2/commentary/ (+ tools) and a fresh pass-003 baseline was prepared; names,
  comments and review credit carried by physical range (425 r1 + 29 r2 carried, 18 re-reviewed).
- L-012 applied (consumed): first camera record = full-field view ($CB=0, nametable $2000),
  second = infield close-up while pitching ($CB=6, nametable $2800). Symbols, comments and
  dependent tables updated; two inverted fielder-frame comments fixed;
  SetFirstCameraHideSprY -> SetFullFieldViewHideSprY.
- Region 648: label SumOfSquaresXY, "no effective entry found (BEQ at $DF90 always taken)", not
  called dead; SquareByte comment corrected (4*|A|^2).
- review.json 472/472 reviewed, 619 units; editorial audit pending. verify byte_identical true
  (game.asm d2739ac3...); reading copy refreshed; input/game.nes unchanged.
- Handoff: analysis/handoffs/round-3-commentary.md. Status: COMPLETE (current-source readability).

### Coordinator (2026-11-18)
- Round 3 commentary accepted phase-complete (472/472; verify byte-identical; round-2 commentary
  preserved in analysis/round-2/). L-011, L-012 consumed.
- Round 3, phase redirect: STARTING (short; reuse sound L-009 review unless round-3 changes create new premises).

## Round 3 -- redirect (2026-11-18), attempt r3-a, checkpoint pass-003
- Structural audit of current source (analysis/redirect/r3-a/struct_audit.py, ptr_audit.py): all control
  targets on decoded instruction starts; 11 inline-argument consumers (GetInlineArgs/QueueInlinePpuBytes)
  all registered, 83 inline sites with exactly sized Data per exit-Y contract; no unregistered
  stack-inspecting routine; branch-ended Code regions end in always-taken branches. No classification lead.
- L-009 (0x2604/$E5F4) not reopened: no changed premise; round-2 rejection review reused (attempt appended).
- L-013 (ready, low, commentary-only): ForceCheck returns N set = FORCED (exact model, 0 mismatches over 20
  configurations; 719 natural calls traced from r1-fielding-start). Source comments are inverted. Runner
  progress encoding confirmed (0 home, 1 batter running, 2k on base k, 2k+1 leading/running); exact $D981
  arithmetic A = 2*(ceil(p/2)+3-BasesAwarded).
- Deferred: retreat/advance direction and PointPrevBase/NextBase naming (missing a traced full advance /
  home run), HitPower/HitAngle rename (editorial), unreferenced fragment purposes (no reference).
- Map/ROM/state/source/binary unchanged (pre-hashes OK). Scenario r3-runner-forcecheck added.
- ROM classification unchanged: 24591/24592 (99.996%); 1 unknown byte 0x2604.
- Handoff: analysis/handoffs/round-3-redirect.md. Status: TERMINAL for classification (INCOMPLETE, 1 byte);
  no further full cycle warranted; L-013 for editorial/commentary touch-up.

### Coordinator (2026-11-18)
- Round 3 redirect: no structural leads; L-009 review reused (sound, unchanged premises).
  Commentary-only ready lead L-013 (ForceCheck N-flag inversion, $D981 formula, runner progress encoding).
  No classification cycle warranted (map unchanged; an empty disassembly pass would be pointless).
- Round 3 commentary continuation (map unchanged, resume existing commentary): STARTING to apply L-013
  so known editorial defects are fixed before terminal handoff.

## Round 3 -- commentary continuation (L-013) (2026-11-18), checkpoint pass-003
- Map/state unchanged: resumed existing commentary (pre-edit copies in analysis/round-3/pre-l013/).
- ForceCheck contract corrected (N set = forced), ForceCheckSelf Y = slot/2 - 2, all dependent caller
  comments fixed; RunnerDecision_stay -> RunnerDecision_keepN (forced path returns N set = go);
  TryTagRunnerAtBase -> TryForceOutAtBase; exact $D981 formula; RunnerRole progress encoding documented.
- Fixed: RunnerFlags+1,x (X = slot/2) addresses $99-$9C -> new RunnerCommitted; $95 renamed
  RunnerOnStartBase, TryForceAtBase -> TryTagUpOutAtBase (tentative tag-up reading).
- Open: runner direction naming (static reading says Runner_retreat/PointPrevBase advance; names kept),
  HitPower/HitAngle unchanged.
- verify byte_identical (game.asm b89dd66c...), reading copy refreshed, review.json 472/472 (42 round-3),
  L-013 consumed. Handoff: analysis/handoffs/round-3-commentary.md "Continuation (L-013)".

### Coordinator (2026-11-18)
- L-013 commentary continuation complete (verify byte-identical). Open editorial question: runner
  direction names (Runner_retreat/Runner_backToBase, PointPrevBase/PointNextBase) possibly inverted;
  a concrete bounded test exists (trace one runner advancing after a hit). Returning it to redirect
  (round 3 continuation, research in analysis/redirect/r3-b/) rather than leaving a suspected-wrong name.

### Redirect continuation r3-b: runner direction (2026-11-18)
- Natural route (r1-fielding-start, 1800 frames, buttons:["a"] pulses, no RAM writes) traced twice:
  runner labels ($D885-$DA5F) and Point* plus Y ($E481-$E4B7). The names are INVERTED:
  Runner_retreat/Runner_backToBase run the runner one base forward (PointPrevBase = base ahead; slot 8
  3->4->5->6 first->second->third, slot 6 1->2->3->4). Runner_advance holds him at the PointNextBase
  point just past the base he reached. No decreasing-progress path was exercised.
- Lead L-014 (ready, low, commentary-only, exact renames) produced. Map/state/source/ROM unchanged.
  Evidence: analysis/redirect/r3-b/ (dir_runner.out, dir_point.runners.out, hashes.txt).
  Handoff: analysis/handoffs/round-3-redirect.md "Continuation r3-b: runner direction".

### Commentary continuation L-014 (2026-11-18)
- Runner direction renames applied (Runner_runNextBase/runToBase/holdPastBase, PointBaseAhead/
  PointPastBase, plus settleOnBase/arrivedNoAward consistency renames); comments now runtime-confirmed
  (r3-b), return-to-base path kept static-only. verify byte-identical (game.asm a0f6b465...), reading
  copy and review.json refreshed, uncertainties item resolved, L-014 consumed.
  Handoff: analysis/handoffs/round-3-commentary.md "Continuation (L-014)".

## 2026-11-18 — Coordinator terminal handoff: INCOMPLETE (justified terminal; not ROM completion)

- Final round 3; current phase: terminal handoff. Final checkpoint pass-003 (== maps/current.json).
- ROM classification: 100.00% (24591 / 24592 bytes classified; 1 bytes unknown). Exact 99.996%;
  residual 0x2604 ($E5F4), L-009, rejection review assessed sound (round-2 redirect; reused round 3).
- Verification at handoff: commentary.py verify ok, byte_identical (game.asm a0f6b465…);
  pass-003 verification ok/byte_identical; ROM sha256 unchanged. Review 472/472; final editorial
  audit reserved for archive acceptance.
- Leads: consumed L-001..L-005, L-008, L-010..L-014; L-006/L-007 informational; L-009 residual.
- archive-options.json written: documents = analysis/handoffs/*.md (incl. final-handoff.md);
  omitted_documents = superseded prior-round commentary copies (analysis/round-1/, round-2/) and
  pre-edit uncertainties backups (analysis/round-3/pre-l013, pre-l014). All Markdown accounted for.
- Final handoff: analysis/handoffs/final-handoff.md. No archive/cleanup run.

## 2026-11-18 — User-authorized residual fallback (batch-only exception; NOT evidence-backed)
- Authorization: explicit user instruction for this batch: when <= 10 physical bytes remain unclassified,
  mark them generic Data with unknown meaning, Manual provenance, as a user-directed fallback.
- Complement re-verified from analysis/state.json before editing (sha a60ef9e5…, == pass-003):
  exactly 1 unknown byte, physical 0x2604 (offset 9732, length 1), CPU $E5F4 (PRG bank 1), no overlaps.
- Prior ambiguity (preserved, unchanged): L-009 — unreferenced byte between DrawTitleCursor's RTS and
  Random; rounds 1-3 found no reference/consumer/dispatch/inline-arg/stack entry, padding convention $FF;
  rejection review assessed sound (analysis/handoffs/round-2-redirect.md, analysis/redirect/r2-a/).
  The code-vs-data decision remains materially ambiguous on the evidence; that is NOT changed by this edit.
- Action: map.annotate region id 796 {offset 9732, length 1, kind Data, provenance Manual, confidence 1,
  no label}; comment/evidence state the user-directed fallback. Script: scratch/fallback/annotate.py;
  pre-edit state copy scratch/fallback/state.pre-fallback.json (== maps/pass-003.json).
- map.validate ok; map.save analysis/state.json; map.export maps/current.json (sha fc5d69ba…).
- Checkpoint pass-004-fallback: ok, byte_identical (analysis/pass-004-fallback-verification.json).
  ROM sha256 unchanged (421399f2…).
- Counts: 24592/24592 bytes now carry a region kind; of these 24591 are evidence-backed (Static/Observed
  or prior Manual structure) and 1 byte (0x2604) is the user-directed Manual fallback.
  Evidence-backed whole-ROM classification is therefore NOT claimed complete.
- Next: commentary phase (integrate the fallback byte in source/review inventory), final review and
  strict editorial audit, fresh verification, archive-options.json update.

## 2026-11-18 — Round 4 commentary: final acceptance on pass-004-fallback (audit PASSED)
- Map changed (fallback byte only), so round-3 commentary preserved in analysis/round-3/commentary/
  (+ round-3/tools-commentary-r3/), fresh baseline prepared, all names/comments/review carried by
  physical range (tools: analysis/tools-commentary-r4/).
- $E5F4 source note/review entry state the USER-DIRECTED FALLBACK (generic Data, unknown meaning,
  not evidence-backed, L-009 unresolved); neutral label Data_E5F4.
- Editorial fixes: BaseApproachOffsets/ThrowSideX labels, $E01D throw tables and HomeRunFlashColors
  format comments, OutAtBase comment, data-table continuation records in review.json.
- `commentary.py audit` ok, byte_identical, 472 regions / 620 units, editorial_audit reviewed;
  game.asm sha256 62a7a702…. Evidence-backed classification still 24591/24592 (+1 fallback byte).
  Handoff: analysis/handoffs/round-4-commentary.md.

### Coordinator — final acceptance after user-directed fallback (2026-11-18)
- Commentary worker (round 4): preserved round-3 commentary to analysis/round-3/commentary/, re-prepared
  from pass-004-fallback, carried all review credit, fallback note at Data_E5F4, editorial fixes;
  handoff analysis/handoffs/round-4-commentary.md.
- Coordinator re-ran `commentary.py audit`: ok, byte_identical, 472 regions / 620 units, editorial_audit
  true; game.asm sha256 62a7a702…; map fc5d69ba…; ROM 421399f2… unchanged.
- Classification: evidence-backed 24591/24592; user-directed Manual fallback 1 byte (0x2604); unknown 0.
  Evidence-backed whole-ROM completion NOT claimed. Fallback record: analysis/handoffs/final-fallback.md.
- TK-003 recorded (docs mismatch). archive-options.json updated (final-fallback, round-4 handoff selected;
  round-3 commentary copy omitted as superseded). Workflow at terminal handoff; no archive run.
