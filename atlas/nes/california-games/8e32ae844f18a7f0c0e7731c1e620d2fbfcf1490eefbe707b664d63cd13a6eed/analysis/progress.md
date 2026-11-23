# Disassembly progress

Workspace initialized. Baseline and classification have not run (initial state before round 1).

## Coordinator log (goal.md orchestration)

Handoff directory: analysis/handoffs/ (one compact handoff per phase: round-NN-<phase>.md).
Disk at coordinator start (2026-11-22): 3.2G free of 29G (89% used) — keep scratch bounded.

Round 1 disassembly result: ROM classification: 83.73% (109762 / 131088 bytes classified; 21326 bytes unknown). Reported to user.
Round 2 disassembly result: ROM classification: 99.89% (130939 / 131088 bytes classified; 149 bytes unknown). Reported to user.
Round 3 disassembly result: ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown). Reported to user. Residual 3 B = TK-04 cross-window instruction.

| Round | Phase | Status | Input checkpoint | Output checkpoint | Verification | Leads consumed/produced | Handoff |
|---|---|---|---|---|---|---|---|
| 1 | disassembly | finished | (none: fresh workspace) | maps/round01-final.json (b4a8fa61…) | byte-identical (analysis/round01-final-verification.json) | produced L-R1-01..08 | analysis/handoffs/round-01-disassembly.md |
| 1 | commentary | finished (gate passed: 2265/2265 reviewed) | maps/round01-final.json | commentary/source/game.asm (c0b3595f…) | byte-identical (analysis/round01-commentary-verification.json) | produced L-R1C-01..03 | analysis/handoffs/round-01-commentary.md |
| 1 | redirect | finished (attempt r01-a1) | maps/round01-final.json + commentary c0b3595f… | unchanged map; proposals analysis/redirect/r01-a1/evidence/proposals.json | n/a (no map change) | consumed L-R1-*/L-R1C-*; produced ready L-R1R-01..10, deferred L-R1R-11 | analysis/handoffs/round-01-redirect.md |
| 2 | disassembly | finished | maps/round01-final.json | maps/round02-final.json (53e08b7b…) | byte-identical (analysis/round02-final-verification.json) | consumed L-R1R-01..10 (accepted), L-R1R-11 superseded; produced L-R2-01 | analysis/handoffs/round-02-disassembly.md |
| 2 | commentary | finished (gate passed: 2291/2291 reviewed) | maps/round02-final.json | commentary/source/game.asm (2ed461a9…); round-1 commentary preserved in analysis/round-1/ | byte-identical (analysis/round02-commentary-verification.json) | - | analysis/handoffs/round-02-commentary.md |
| 2 | redirect | finished (attempt r02-a1) | maps/round02-final.json + commentary 2ed461a9… | unchanged map; proposals analysis/redirect/r02-a1/evidence/proposals.json | n/a | consumed L-R2-01; produced ready L-R2R-01..04 (143 B); residual 6 B judged exhausted ($8008 ambiguous, TK-04) | analysis/handoffs/round-02-redirect.md |
| 3 | disassembly | finished | maps/round02-final.json | maps/round03-final.json (aab2fb6c…) | byte-identical | consumed L-R2R-01..04 (accepted; -02 at 0.6 conf); produced L-R3-01 ($8008 classified code 0.55 by cross-bank layout) | analysis/handoffs/round-03-disassembly.md |
| 3 | commentary | finished (gate passed: 2293/2293 reviewed; round-2 commentary preserved in analysis/round-2/) | maps/round03-final.json | commentary/source/game.asm (b70bdbb2…) | byte-identical (analysis/round03-commentary-verification.json) | - | analysis/handoffs/round-03-commentary.md |
| 3 | redirect | attempt r03-a1 finished; continuing (coordinator: deferred avenues lack missing-prerequisite reasons for some items) | maps/round03-final.json + commentary b70bdbb2… | unchanged map | n/a | TK-04 rejection review done (sound); produced commentary leads L-R3R-01..04; S12 natural ending visited | analysis/handoffs/round-03-redirect.md |
| 3 | redirect (cont.) | finished attempt r03-a2 | same | unchanged map | checksums rechecked | produced commentary leads L-R3R-05..09; remaining deferrals have reasons (RNG-luck brute force, names unaffected) | analysis/handoffs/round-03-redirect-a2.md |
| 4 | disassembly | skipped by coordinator: no classification proposals (only commentary leads); map unchanged maps/round03-final.json; ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown) | maps/round03-final.json | maps/round03-final.json | unchanged | - | - |
| 4 | commentary | finished (2293/2293 reviewed; raw RAM operands 922→62) | maps/round03-final.json + commentary b70bdbb2… | commentary/source/game.asm (e12e1cad…) | byte-identical (analysis/round04-commentary-verification.json) | consumed L-R3R-01..09 | analysis/handoffs/round-04-commentary.md |
| 4 | redirect | finished attempt r04-a1 | maps/round03-final.json + commentary e12e1cad… | unchanged map | hashes rechecked | TK-04 review reused (sound); produced commentary leads L-R4R-01 (bank-6 overlap names), L-R4R-02 (10 in-code table aliases, 5 wrong comments) | analysis/handoffs/round-04-redirect.md |
| 5 | disassembly | skipped by coordinator: no classification proposals; ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown) | maps/round03-final.json | unchanged | - | - | - |
| 5 | commentary | finished (2293/2293 reviewed) | commentary e12e1cad… | commentary/source/game.asm (a2b9634c…) | byte-identical (analysis/round05-commentary-verification.json) | consumed L-R4R-01..02 | analysis/handoffs/round-05-commentary.md |
| 5 | redirect | not relaunched: unchanged sound terminal assessment (round-04-redirect.md rejection review inspected and accepted by coordinator) | - | - | - | - | - |
| final | terminal handoff | TERMINAL INCOMPLETE (3 B TK-04); not ROM completion | maps/round03-final.json (aab2fb6c…) | commentary a2b9634c… | byte-identical | L-R1-01 remains blocked | analysis/handoffs/final-terminal.md |
| final | user-directed fallback (batch-only exception) | done: 3 B (16398+2, 114704+1) -> Data/Manual, unknown meaning; not proven classification | maps/round03-final.json (aab2fb6c…) | maps/final-fallback.json (212cccb6…) | byte-identical (analysis/final-fallback-verification.json) | L-R1-01/TK-04 still open | analysis/progress.md (Final user-directed fallback) |
| 6 | commentary (final review + strict editorial audit) | finished: audit passed (2293 regions, 1237 units, editorial_audit true); round-5 preserved in analysis/round-5/ | maps/final-fallback.json | commentary/source/game.asm (e5b27876…) | byte-identical (analysis/round06-final-commentary-audit.json; coordinator re-ran audit: pass) | - | analysis/handoffs/final-commentary.md |
| final | terminal handoff (updated) | FINAL with manual fallback; proven 131085/131088, fallback 3 B Manual | maps/final-fallback.json | commentary e5b27876… | byte-identical + audit pass | TK-04 open | analysis/handoffs/final-terminal.md |

Archive preparation: archive-options.json written (scripts/archive.py contract inspected). Selected: all phase handoffs, final-terminal.md, redirect NOTES.md records, round-1/round-2 commentary progress/uncertainties. Omitted as disposable: analysis/round-{1,2}/commentary/{AGENTS,prompt,workflow}.md (duplicated generated phase instructions). All Markdown under analysis/ and commentary/ accounted for. Archive not executed; no cleanup performed (scratch/ 43M left in place).

## Round 1 disassembly phase (worker section)

ROM classification: 83.73% (109762 / 131088 bytes classified; 21326 bytes unknown).

- Baseline: maps/baseline.json, generated/baseline, analysis/baseline-verification.json (byte-identical).
- Output checkpoint: maps/round01-final.json (sha256 b4a8fa61eed48625a58da3d0af895c73cb61bce2d3ff43eae33f5868a6c437e2),
  generated/round01-final, analysis/round01-final-verification.json: ok, byte_identical true.
  Intermediate verified checkpoints: round01-code, round01-data1, round01-data2.
- Regions: 1,481 (Code 579: 19,940 B observed + 4,746 B static; Data 902: 45,190 B observed-read + 39,886 B static).

| Bank | CPU | Code | Data | Unknown |
|---|---|---|---|---|
| header | - | 0 | 16 | 0 |
| 0 | $8000 | 11379 | 2880 | 2125 |
| 1 | $8000 | 448 | 13694 | 2242 |
| 2 | $8000 | 1021 | 13252 | 2111 |
| 3 | $8000 | 173 | 14799 | 1412 |
| 4 | $8000 | 103 | 16052 | 229 |
| 5 | $8000 | 673 | 9912 | 5799 |
| 6 | $8000 | 1733 | 9705 | 4946 |
| 7 | $C000 | 9156 | 4766 | 2462 |

Evidence/method: analysis/investigation.md. Scenarios: analysis/scenarios.json. Leads: analysis/leads.json.
Toolkit findings: analysis/toolkit-issues.md (2 confirmed doc/API bugs TK-01/TK-02, 2 missing features TK-03/TK-04,
1 suspected performance limitation TK-05; none blocking). Status: incomplete (unknown bytes remain).
Handoff: analysis/handoffs/round-01-disassembly.md.

## Round 1 commentary phase (worker section)

- Input: maps/round01-final.json (b4a8fa61...), unchanged classification: 83.73% (109762 / 131088; 21326 unknown).
- Prepared commentary/ (scripts/commentary.py prepare). Source built from commentary/annotations.txt via
  analysis/tools/cmt_apply.py; review inventory via analysis/tools/cmt_review.py.
- Coverage: all classified source reviewed (579 code regions, 902 data regions) plus notes on all 784 unknown gaps;
  review.json 2265/2265 entries reviewed, 876 code units; editorial_audit pending (final acceptance only).
- Verification: analysis/round01-commentary-verification.json ok, byte_identical (game.asm sha256 c0b3595f...).
  Reading copy: commentary/reading-after.asm.
- Leads produced: L-R1C-01..03 (analysis/leads.json). No toolkit issues found in this phase.
- Status: finished. Handoff: analysis/handoffs/round-01-commentary.md.

## Round 1 redirect phase (worker section, attempt r01-a1)

- Input: maps/round01-final.json (b4a8fa61...) + commentary c0b3595f...; map/state/ROM unchanged (classification still 83.73%).
- Research: analysis/redirect/r01-a1/ (tools, evidence, proposals.json). Proposed 16,480 of 21,326 unknown bytes
  (+1 Data->Code correction at 0xC01E) in ready leads L-R1R-01..10; frontier L-R1R-11 (~4.8K residual, mostly bank 6 course scripts).
- Runtime (bounded, artificial where noted): power-on boot trace; surf end via RAM $87=1; compete FINAL RESULTS via RAM $3C=5.
- Leads consumed/superseded: L-R1-03..08, L-R1C-01..03; L-R1-01 still blocked (TK-04). No new toolkit issues.
- Recommendation: (a) next cycle disassembly with ready leads. Status: finished. Handoff: analysis/handoffs/round-01-redirect.md.

## Round 2 disassembly phase (worker section)

ROM classification: 99.89% (130939 / 131088 bytes classified; 149 bytes unknown).

- Input: maps/round01-final.json (b4a8fa61...); state restored from analysis/state.json (baseline not reinitialized).
- Output checkpoint: maps/round02-final.json (sha256 53e08b7ba4231a38a737bbb86021112b6a9e2d287ea74e08841aad195b565bfa),
  generated/round02-final, analysis/round02-final-verification.json: ok, byte_identical true. Intermediate verified
  checkpoint maps/round02-leads.json (70848b6d..., 96.30%). analysis/state.json and maps/current.json match round02-final.
- Totals: Code 25,977 B, Data 104,962 B (incl. 16-byte header).

| Bank | CPU | Code | Data | Unknown |
|---|---|---|---|---|
| header | - | 0 | 16 | 0 |
| 0 | $8000 | 11568 | 4785 | 31 |
| 1 | $8000 | 651 | 15733 | 0 |
| 2 | $8000 | 1274 | 15102 | 8 |
| 3 | $8000 | 234 | 16150 | 0 |
| 4 | $8000 | 103 | 16281 | 0 |
| 5 | $8000 | 675 | 15709 | 0 |
| 6 | $8000 | 1813 | 14571 | 0 |
| 7 | $C000 | 9659 | 6615 | 110 |

- Leads: L-R1R-01..10 verified and applied (16,480 B + 0xC01E-0xC020 Data->Code correction); L-R1R-11 superseded by
  the round-2 residual gap screen (4,697 B generic data incl. 3 manual record-pattern gaps); L-R1-01 still blocked (TK-04).
  Produced L-R2-01 (149 B residual fragments). Evidence/method: analysis/investigation.md (Round 2 section).
- Gameplay: S13 compete-some visited naturally (no new code). Workspace tool fix: analysis/tools/closure.py forced_branch.
- Toolkit: no new toolkit issues found (existing TK-01..05 in analysis/toolkit-issues.md). Status: incomplete (149 unknown bytes).
  Handoff: analysis/handoffs/round-02-disassembly.md.

## Round 2 commentary phase (worker section)

- Input maps/round02-final.json (53e08b7b...), classification unchanged at 99.89% (149 unknown bytes).
- Round-1 commentary preserved under analysis/round-1/; fresh commentary baseline prepared; annotations carried forward.
- Commented all 32 new code regions, rewrote invalidated round-1 notes, replaced all 44 address-style routine names,
  added data/RAM symbols and decode notes for the 18 residual unknown gaps.
- Verification: commentary verify ok, byte_identical true; source game.asm sha256 2ed461a9...; reading copy refreshed.
- review.json 2291/2291 reviewed (981 units); editorial audit pending (final acceptance). Remaining unreviewed ranges: none.
- Status: finished. Handoff: analysis/handoffs/round-02-commentary.md.

## Round 2 redirect phase (worker section, attempt r02-a1)

- Input maps/round02-final.json (53e08b7b...), 99.89% (149 B unknown in 18 gaps). Map, state, ROM, source and binary are unchanged.
- Every residual gap now has a decision (analysis/redirect/r02-a1/NOTES.md). Ready leads L-R2R-01..04 propose 143 B
  (Code 38: contract-supported preambles, CallBank0 chain link, a self-closed loop, 3 lone JMP stubs; Data 105:
  4 consumer-bounded table tails and 5 generic non-code ranges). Evidence: analysis/redirect/r02-a1/evidence/proposals.json.
- Considered exhausted (rejection review in the handoff): b0 $8008 (3 B, ambiguous vestigial JSR entry) and the
  TK-04 bytes (3 B; exact toolkit blocker recorded in TK-04 update).
- Gameplay: two bounded natural compete-all continuations reached flying disk but not the ending (S12 attempt recorded).
  Surf frame handlers $1A/$18 were observed naturally.
- Commentary corrections queued: PlayerNames stride 7; FrameHandlerTable extent note; new table labels.
- Recommendation: (a) next disassembly cycle. Status: finished. Handoff: analysis/handoffs/round-02-redirect.md.

## Round 3 disassembly phase (worker section)

- Input maps/round02-final.json (53e08b7b...), restored from analysis/state.json (identical); existing classifications preserved; baseline not reinitialized.
- Applied ready leads L-R2R-01..04 (143 B) after independent re-check against ROM bytes and commentary/source/game.asm
  (tool: analysis/tools/r03_apply.py). L-R2R-01 Code 29 B incl. $F2BA CallBank0 re-split (region 578 removed;
  $F2BA-$F2BE Static, $F2BF-$F2F0 re-added Observed with original evidence); L-R2R-02 Code 9 B (conf 0.6);
  L-R2R-03 table-tail Data 18 B; L-R2R-04 generic Data 87 B. All verified.
- Coordinator $8008 check: banks 1-6 all have [00..07][N x 3-byte JMP entries at $8008, Observed code][copyright text]
  (N = 3,9,3,2,1,6). Bank 0 $8008 = 20 93 D8 (jsr UpdateSound) in the same slot -> classified Code Static 0.55
  (new lead L-R3-01, verified) with caveat: JSR return would fall into text; no caller; not claimed working/reachable.
- Output checkpoint maps/round03-final.json sha256 aab2fb6c6b8120365d94d57a881bad3c0da726c7f5ccf5126a5504d8367b297b;
  run.py checkpoint: validate ok, generate ok, verify ok + byte_identical true (analysis/round03-final-verification.json);
  generated diagnostics empty. analysis/state.json and maps/current.json identical to the checkpoint.

ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown).

- Per bank (classified code/data, unknown): header 16/0; b0 11585/4797, 2; b1 651/15733, 0; b2 1274/15110, 0;
  b3 234/16150, 0; b4 103/16281, 0; b5 675/15709, 0; b6 1813/14571, 0; b7 9683/6700, 1. Totals Code 26018, Data 105067.
- Remaining unknown: 0x400E-0x400F (b0 $BFFE-$BFFF) + 0x1C010 (b7 $C000) = observed `lda $C149,x` spanning the
  bank-0/fixed-bank window boundary; blocked by toolkit (TK-04, L-R1-01). Rounded 100.00% is not completion.
- Gameplay: none this round (no lead needed it; S12/S14 status unchanged in analysis/scenarios.json).
- Toolkit: no new toolkit issues found (TK-01..05 in analysis/toolkit-issues.md; TK-04 is the residual blocker).
- Status: finished for this pass; incomplete overall (3 B, toolkit-blocked). Handoff: analysis/handoffs/round-03-disassembly.md.

## Round 3 commentary phase (worker section)

- Map changed since round-2 commentary, so round-2 commentary was preserved under analysis/round-2/ (commentary/ +
  tools/) and a fresh byte-identical baseline was prepared (2293 scaffold entries). Annotations carried by bank/CPU address.
- All new round-3 regions commented and labelled (Bank0Entry0, CallBank0, PlaySfxSlot6, FillTerrainTableForever,
  UploadStatusRow2360, SetStatusModeOneAndClear, StubJump* x3, table tails, generic data notes); TK-04 bytes explain the
  exact toolkit blocker. Redirect corrections applied (PlayerNames stride 7, FrameHandlerTable extent).
- Backlog: 28 global RAM names, 29 foot bag/flying disk overlay names, 6 data symbols; $61/$4D "unread" claim corrected;
  foot bag prose corrected (FootBagPlayerMove -> FootBagStepPose). Raw zero-page operands 1212 -> 922 (remaining are event
  overlay bytes with roles not separated; listed in commentary/uncertainties.md).
- Verify: ok, byte_identical true, game.asm b70bdbb2f5fe38db109efca1bed1eb3f995450188e442d3e0395d55e15e50869
  (analysis/round03-commentary-verification.json). Reading copy refreshed. review.json 2293/2293 reviewed, audit pending.
- Status: finished. Handoff: analysis/handoffs/round-03-commentary.md.

## Round 3 redirect phase (worker section, attempt r03-a1)

- Input checkpoint maps/round03-final.json aab2fb6c... (unchanged; state.json re-checked), source b70bdbb2...; only
  emulator calls; research in analysis/redirect/r03-a1/.
- Ledger refreshed: 3 B unknown = 0x400E-0x400F (b0 $BFFE-$BFFF) + 0x1C010 (b7 $C000), one observed cross-window
  instruction; compact rejection review re-done against toolkit RegionKind/decode_region source -> still TK-04 blocked.
- S12 natural ending reached (no RAM writes): flying-disk stall = no throw (round ends only on disk landing); replay +
  meter throw x3 -> EventIntroFrame -> one A -> FINAL RESULTS. All 15 scenarios visited.
- New ready commentary leads: L-R3R-01 flying disk throw meter / disk / catcher RAM contract (corrects "catcher"
  routine names), L-R3R-02 half pipe skater record $91-$98 + saved copy $99-$A0, L-R3R-03 text tables $EE7D/$EE86,
  L-R3R-04 Static disk blocks observed at RAM level. Reachability scan for round-3 no-caller code: negative.
- No classification proposals. Recommendation: (a) commentary cycle for L-R3R-01..04; afterwards terminal incomplete
  (3 B, TK-04) unless toolkit work is authorized. Handoff: analysis/handoffs/round-03-redirect.md. Status: finished.

## Round 3 redirect phase (worker section, attempt r03-a2)

- Continuation of r03-a1 deferred avenues; checkpoint aab2fb6c... / source b70bdbb2... unchanged (re-checked); only
  emulator calls; research in analysis/redirect/r03-a2/.
- New ready commentary leads: L-R3R-05 BMX air/trick contract (+ corrections: BmxLand = takeoff, bonus-table order
  reversed vs labels, BmxScoreTrick = finish, hazard tiles, air-score index-21 overrun), L-R3R-06 skating bouncing ball
  + 4-byte spawn-mask table indexed 0-15, L-R3R-07 surfing shark fin / aerial / carve power (+ misplaced header),
  L-R3R-08 foot bag seagull $DE-$E2, L-R3R-09 half pipe trick/fall contract (corrects L-R3R-02 save gate).
- No classification proposals (3 B TK-04). Recommendation: one commentary cycle for L-R3R-01..09, then terminal
  incomplete unless TK-04 authorized. Handoff: analysis/handoffs/round-03-redirect-a2.md. Status: finished.

## Round 4 commentary phase (worker section)

- Map/state unchanged vs the frozen commentary baseline (all aab2fb6c...), so existing commentary was resumed.
- Leads L-R3R-01..09 applied (status -> consumed); whole-source naming sweep of overlay RAM, RAM arrays and tables;
  corrections: BmxCallBank4 = random comment, LoadSurfColumnSet -> LoadBmxRiderTiles, LoadTerrainStrip -> LoadGullTiles.
- Raw RAM operands 922 -> 62 (50 intentional base-pair helpers), raw ROM operands 415 -> ~120.
- Verify ok/byte-identical (analysis/round04-commentary-verification.json), source e12e1cad..., reading copy refreshed.
  Classification unchanged: 131085/131088 (3 B TK-04). Handoff: analysis/handoffs/round-04-commentary.md. Status: finished.

## Round 4 redirect phase (worker section, attempt r04-a1)

- Checkpoint aab2fb6c... / source e12e1cad... / ROM unchanged (re-checked); only emulator calls + read-only static
  scripts; research in analysis/redirect/r04-a1/ (NOTES.md, tools/xwatch.py, tools/static_checks.py, evidence/).
- Ledger recomputed: 3 B unknown = TK-04 cross-window instruction (0x400E-0x400F + 0x1C010); r03-a1 rejection review
  reused (premises unchanged) and restated compactly in the handoff.
- Bank-6 $A9-$C1 overlap resolved: BMX object stream ($97C2, 642 records) has only CourseObjType 0/$FF, so marker /
  palette-slot writes are skating-only; confirmed by BMX vs skating instruction watches (L-R4R-02).
- All 10 in-code table bases are index-offset aliases; real extents established (static gates + observed X sets +
  gapless packing), all inside existing Data, no classification conflict; 10 further alias labels found
  (incl. PipeBackwardsBoard $97D5-$97DD); 5 wrong comments identified (L-R4R-01).
- Tentative names deferred (naming only, not discriminable by short watches). Proposed/accepted bytes 0/0.
- Recommendation: (a) one commentary cycle for L-R4R-01/02 (disassembly: nothing to apply), then terminal incomplete
  (3 B, TK-04) and final editorial audit. Handoff: analysis/handoffs/round-04-redirect.md. Status: finished.

## Round 5 commentary phase (worker section)

- **Baseline check:** maps/current.json, analysis/state.json and commentary/phase1-state.json are all
  aab2fb6c... (unchanged), so the existing commentary was resumed with no re-prepare.
- **L-R4R-01 and L-R4R-02 applied.** Each consumer was re-checked in the source before the change.
  - 20 index-offset aliases are now emitted as Name = Name_XX - $XX, with a Name_XX label at the first real
    byte.
  - The 5 wrong comments were fixed.
  - The CourseObjects Writes list now reads CourseMarkerXHi/X; the cause was an off-by-one scope boundary.
  - The BMX non-overlap note was added to CourseObjects/CourseObjectPalette.
- **Branch labels:** 63 structural branch labels in 7 key routines were given descriptive names.
- **Tool changes (workspace only):**
  - cmt_apply.py: the "D ... @real" alias syntax.
  - cmt_review.py: alias real-label units.
- **Verification:** byte-identical (analysis/round05-commentary-verification.json); source a2b9634c....
  The reading copy was refreshed.
- **review.json:** 2293/2293 reviewed; editorial_audit pending.
- **Handoff:** analysis/handoffs/round-05-commentary.md. Status: finished.

## Final user-directed fallback (coordinator, 2026-11-23) — batch-only exception

- **Authorization:** the user explicitly authorized a batch-only exception: when at most 10 physical ROM bytes remain
  unclassified, mark them as generic Data with unknown meaning using Manual provenance. This overrides workspace rules
  requiring further structural proof for these residual bytes only. It is a user-directed fallback, NOT an observed or
  statically proven data role.
- **Complement verified first** (analysis/state.json aab2fb6c…, before the change): exactly 3 unclassified bytes,
  physical 0x400E-0x400F (offset 16398, length 2; bank 0 $BFFE-$BFFF) and 0x1C010 (offset 114704, length 1; fixed bank 7
  $C000). Matches the user's list; no other unknown bytes.
- **Prior ambiguity (unchanged evidence):** these bytes form one observed cross-window instruction (opcode fetch at
  0x400E, operand bytes 0x400F + 0x1C010, successor fetch at $C001). The toolkit cannot represent one Code region over
  non-adjacent physical bytes (TK-04, analysis/toolkit-issues.md; lead L-R1-01). Rejection reviews: round-03-redirect.md,
  round-04-redirect.md. That evidence is preserved; the fallback does not contradict or replace it.
- **Applied:** analysis/tools/final_fallback_apply.py -> map.annotate regions id 2509 (16398+2) and 2510 (114704+1),
  kind Data, provenance Manual, confidence 0.1, no label, evidence text naming the user authorization and TK-04.
  ROM bytes unchanged (input/game.nes sha256 8e32ae84…). map.validate ok; map.save analysis/state.json; map.export
  maps/current.json.
- **Checkpoint:** `run.py checkpoint final-fallback` -> maps/final-fallback.json (sha256 212cccb66dfad0ff…),
  generated/final-fallback, analysis/final-fallback-verification.json: ok, byte_identical true.
- **Classification accounting:** 131088 / 131088 bytes carry a region kind, of which 131085 are evidence-backed
  (Observed/Static) and 3 are the Manual user-directed fallback. Proven classification remains 131085 B (100.00% rounded,
  3 B short); whole-ROM evidence-backed completion is NOT claimed.
- **Next:** final commentary phase (map changed vs commentary baseline aab2fb6c…): preserve round-5 commentary,
  reconcile inventory with the updated map, final review + strict editorial audit, fresh byte-identical verification.

## Final commentary phase (worker section, 2026-11-23)
- The map had changed (fallback 212cccb6...), so the round-5 commentary was preserved whole in
  analysis/round-5/{commentary,tools} and a fresh baseline was prepared (byte-identical, 2293 entries).
- Annotations and review were carried by physical range. Only the 2 records over the 3 fallback bytes were
  invalidated; they are re-described as generic Data with UNKNOWN meaning (Manual user-directed fallback,
  not proven) and the observed cross-window instruction and TK-04 explanation are kept.
- Final editorial review:
  - 84 branch labels renamed;
  - stale numeric prose aliases replaced;
  - one misleading description fixed;
  - uncertainties brought current;
  - editorial_audit recorded as reviewed in review.json.
- `commentary.py audit`: ok, byte_identical, 2293 regions and 1237 units reviewed. game.asm e5b27876....
  Handoff: analysis/handoffs/final-commentary.md.
- Proven classification is still 131085/131088. The 3 Manual bytes are reported separately.

## Final coordinator wrap-up (2026-11-23)

- Final commentary phase (worker) finished; coordinator re-ran `commentary.py audit`: ok, byte_identical true,
  source game.asm e5b27876…, 2293 regions / 1237 units reviewed, editorial_audit true.
- analysis/handoffs/final-terminal.md updated (earlier record preserved verbatim below the new header).
- archive-options.json updated: added final-commentary.md, gameplay/investigation are toolkit-recognised, round-5
  commentary progress/uncertainties selected; round-5 AGENTS/prompt/workflow copies omitted as duplicated generated
  instructions. Archive not executed; no cleanup performed.
- Reporting: proven classification 131085/131088 B; manual user-directed fallback 3 B (Data, Manual, meaning unknown).
