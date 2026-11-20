# Disassembly progress

## Coordinator state (goal.md orchestration)

- Round: 4
- Current phase: TERMINAL — workflow terminated INCOMPLETE (3 unknown bytes; editorial audit passed). Final handoff: analysis/handoffs/final-coordinator.md. archive-options.json written. ROM classification NOT complete.
- Coordinator note (round 3, historical; superseded — residual is now 3 bytes): 5-byte residual (RTS $ADB6/$B2EA/$B343 bank1, NOP $E0F6/$E1B3) — review consistent with evidence-policy; coordinator will return a corrected-test question to redirect round 3 (RTS-trick address-1 refs, split hi/lo tables, script native-call operand encodings) before accepting terminal.
- Latest verified checkpoint: pass-004 (100.00% rounded; 262157/262160; 3 unknown: bank1 0x02DC6, 0x032FA, 0x03353)
- Input checkpoint: none (fresh workspace; baseline not yet run)
- Handoff convention: each phase worker writes analysis/handoffs/round-NN-<phase>.md
  (scope, checkpoint, verification, findings, unresolved questions, deferred work,
  lead IDs consumed/produced) and appends a short entry under "Phase log" below.

## Phase log

### Round 1 disassembly — 2026-11-20 — DONE
- Baseline created and verified (maps/baseline.json, analysis/baseline-verification.json, byte_identical true).
- Checkpoint pass-001: maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json — ok, byte_identical true.
- ROM classification: 93.23% (244422 / 262160 bytes classified; 17738 bytes unknown).
- Code 24646 (Observed 18537 / Static 6109), Data 67641, Text 13065, Padding 7998, Image(CHR) 131072. Per-bank:
  analysis/evidence/round-01/perbank.md. Execution coverage: 9011 ROM opcode addresses observed (separate metric).
- Scenarios: boot/intro/title, name entry, weather, cockpit, harbor/sea navigation, pause, COMMAND menus, password
  (wrong), artificial bite ($0496=1) -> fight + SELECT options + line cut / lost fish, hidden SOUND TEST (SELECT+START at
  boot). Checklist: analysis/scenarios.json. Leads produced L-001..L-008 (analysis/leads.json).
- Toolkit findings TK-001..TK-004 (analysis/toolkit-issues.md); no blockers.
- Handoff: analysis/handoffs/round-01-disassembly.md. Status: INCOMPLETE (unknown bytes remain).

### Round 1 commentary — 2026-11-20 — DONE
- Prepared commentary/ from pass-001; complete current-source readability pass over all classified source
  (626 code regions, all data/text/padding) plus notes for all 777 unknown ranges. Classification unchanged (93.23%).
- Source: commentary/source/game.asm built from commentary/before/game.asm + commentary/notes/annotations.txt
  (analysis/tools/commentary_build.py); review.json filled (3,170 entries reviewed, 837 units; editorial_audit pending).
- Verification: commentary/verification.json — ok true, byte_identical true. Reading copy: commentary/reading-after.asm.
- Redirect questions Q1-Q3 -> leads L-009..L-011; L-006 resolved statically ($06E1 APU helper); notes added to
  L-001/L-002/L-005/L-008. Handoff: analysis/handoffs/round-01-commentary.md. No toolkit issues found in this phase.

### Round 1 commentary continuation — 2026-11-20 — DONE
- Readability gate follow-up: all generic/numbered local branch labels replaced (557 locals; 0 @skip*,
  0 numbered @skipN/@loopN; lone @loop 45 / @return 81) via condition/target naming in
  analysis/tools/commentary_build.py plus 148 hand names in commentary/notes/annotations.txt.
- Fight events 1-9 named from the bank-12 event script texts (FEV_* constants for all 14 values);
  fight fish model helpers/tables renamed by updated value with mechanical headers; sound stream
  commands documented and envelope/sweep/scale misnomers fixed.
- Verification: commentary/verification.json ok, byte_identical true; reading-after.asm refreshed;
  review.json regenerated (3,170 reviewed, 864 units; editorial_audit pending).
- Lead produced: L-012 (fight event script, bank 15 $E5DC-$E5FD). No toolkit issues found.
  Handoff: analysis/handoffs/round-01-commentary.md ("Continuation").


### Round 1 redirect (attempt r01-a) — 2026-11-20 — DONE
- Input checkpoint pass-001 (17,738 unknown). Research dir analysis/redirect/r01-a/ (README.md). No map/source changes.
- Method: bounded traced RAM-patch scenarios from round-1 snapshots (coverage -> r01-a/observed-r01a.json.gz);
  script-engine parser (r01-a/script_parse.py); static descent with script/hook/observed roots (static_rd_r01a.py);
  MMC3 R6/R7 bank-select audit for never-executed banks.
- Milestones visited (artificial): land fish + weigh-in (S11), end of day/results/password (S12), Hawaii 1 (S13),
  all-tournaments ending + FIN screen (S14), fight sea event (S16).
- Results: L-011 resolved (fight entry banks 0/1); rule-R data found overlapping executed code; L-004 callers found
  (script $38/$04/$05 operands); static proposal validated by later traces (0 misaligned of 11,151 contexts).
  Projected unknown after the ready batch: ~4,080 (code/script -6,189; small data -80; banks 3/5/8-11 data -7,389).
- Leads: consumed L-001/2/3/7/11; ready L-004, L-005, L-009, L-010, L-012, L-013, L-015, L-016; L-008 deferred.
- Handoff: analysis/handoffs/round-01-redirect.md. No new toolkit issues (TK-001/TK-003 still apply).

### Round 2 disassembly — 2026-11-20 — DONE
- Input pass-001 (17,738 unknown). Consumed ready leads L-004, L-005, L-009, L-010, L-012, L-013, L-015, L-016 (all
  verified/accepted after re-checks; one 1-byte remainder $ADB6 left unknown); produced and applied L-018 (banks 2/3
  rule-T Text -> Data role correction); produced L-017 (code-shaped unreferenced residues). Tools: analysis/tools/r02/.
- Batch A (leads): +5,235 Code (Observed 1,820 / Static 3,415), +759 Text, +9,205 Data (incl. L-016 7,389), 40 overlapped
  Data regions trimmed. Batch B (residual inspection): +6 Code ($ED52/$EE09 alternate entries), +424 Text, +2,601 Data.
- Scenario S15 (artificial sea events $05C4=1/2/3, traced): bank-0 event wrappers executed; 5,208 observed opcodes, 0 off
  pass-002 instruction starts. Evidence analysis/evidence/round-02/.
- Checkpoint pass-002: maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json — ok, byte_identical
  true, map sha256 c6d84b24d5a969b1353e5443aca4b1ca2be098aa0ce21a7019dddadf53f5d425 (= maps/current.json).
- ROM classification: 99.60% (261116 / 262160 bytes classified; 1044 bytes unknown).
- Code 29,856 (Observed 20,357 / Static 9,499), Data 78,813, Text 13,377, Padding 7,998, Image 131,072, Unknown 1,044.
  Per-bank: analysis/evidence/round-02/perbank.md (unknown: b0 72, b1 769, b2 9, b12 6, b13 8, b14 171, b15 9; others 0).
- Toolkit: no new issues; TK-001/TK-003/TK-004 still apply (analysis/toolkit-issues.md). Status: INCOMPLETE.
- Handoff: analysis/handoffs/round-02-disassembly.md.

### Round 2 commentary — 2026-11-20 — DONE
- Map changed: round-1 commentary preserved in analysis/round-1/ (exact copy + tools + handoff); commentary/ re-prepared
  from pass-002. Annotations carried by bank:CPU position and re-checked against both listings
  (analysis/evidence/round-02/commentary-carry-check.txt); invalid round-1 records (L-009 label, Q2/Q3 notes, $8B81-$8D15
  and $E5DC unknown notes) rewritten. Tools: analysis/tools/commentary_build.py (symbol precedence, ramx, evidence notes),
  commentary_review.py (carried vs new review), analysis/tools/r02c/ (carry, view, findops, scriptrefs, scriptctx, labstats).
- All new code named/documented (bank-0 windows, bank-1 fight-fish setup, bank-12/13 script natives incl. keep, weigh-in,
  records, level-ups, name/password entry and password pack/verify, fixed-bank helpers/hooks); round-1 misnomers corrected
  (HookedFishType, FishSize, KeepFishType, weight records, Skill/MusclePower/BodyStrength levels, TextSlots/InfoText_*).
- Counts: 0 address-only names, 0 numbered generic labels, 0 raw RAM/ROM operands, 680 routine headers; 25 unknown ranges
  all noted; review.json 3,268 entries reviewed (2,299 carried, 969 new), 1,054 units, editorial_audit pending.
- Verification: commentary/verification.json ok, byte_identical true; reading copy commentary/reading-after.asm refreshed.
  Classification unchanged (99.60%). L-017 notes updated. No toolkit issues found.
  Handoff: analysis/handoffs/round-02-commentary.md.

### Round 2 redirect (attempt r02-a) — 2026-11-20 — DONE
- Input checkpoint pass-002 (maps/current.json sha256 c6d84b24...; 1,044 unknown / 25 ranges). Research dir
  analysis/redirect/r02-a/ (r01-a untouched). No map/state/source/ROM changes.
- Method change: corrected premise for the L-017 conflict — round-1 readtrace.py counted 6502 page-crossing branch dummy
  reads as data reads. Static audit (dummyreads.py: 39 of 63 dummy addresses coincide with recorded reads) + bounded
  runtime trace (snapshots sea/hawaii_sea, 30 frames, no patches) confirm $A20E/$A51C/$A801 reads are branch dummy reads.
- Outcome: all 25 residual ranges inspected (ledger.md). Ready L-019 (bank-1 fish code set: Unknown->Code 734,
  Unknown->Data 32, Data->Code 48) and L-020 (banks 0/2/12/13/14/15 twins/stubs + data: Unknown->Code 263,
  Unknown->Data 10); validate.py structural check all ok. Residual 5 lone bytes (RTS x3, NOP x2) with rejection review.
- Leads: L-017 superseded, L-006 resolved, L-008 still deferred. Scenarios: all visited; notes added. Server stopped.
  No toolkit issue (workspace readtrace.py filter defect noted in the handoff).
- Handoff: analysis/handoffs/round-02-redirect.md.

### Round 3 disassembly — 2026-11-20 — DONE
- Input pass-002 (1,044 unknown). Consumed ready leads L-019 and L-020 (both verified, applied as proposed, no narrowing).
  Checks: r02-a validate.py re-run on pass-002 = identical to validate.txt (every proposal decodes exactly, all
  branch/JSR/JMP targets known or proposed instruction starts); each decode and boundary hand-reviewed with r2dis.py;
  overlapped rule-R Data 0x0221E/0x0252C/0x02811 = branch page-cross dummy reads (r02-a dummyreads.txt/dummytest-results.txt);
  0x0221F-0x0222B (rule R "around clusters", no direct read) decodes as the tail of $A1FB-$A21B.
- Map edits (analysis/tools/r03/plan.py -> scratch/r03_plan.json, applied with analysis/tools/r02/apply.py): removed Data
  ids 766, 767, 774, 775, 787; re-added 0x02550-0x0256F as Data (16-word jump table $A540 for the $A52E `jmp ($0016)`
  dispatch); added 24 Code regions (1,045 B, all Static, confidence 0.85, reachability not claimed) and 5 Data regions (42 B).
  Net: Unknown->Code 997, Data->Code 48, Unknown->Data 42.
- No gameplay run (all S01-S16 visited; no remaining unknown is reachable by a known path). No emulator experiments.
- Checkpoint pass-003: maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json — ok, byte_identical
  true, map sha256 703aba695f360c89b35583fabe94ca17e4d2f4bc4437837f76069504c92be4c4 (= maps/current.json = analysis/state.json).
- ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).
- Code 30,901 (Observed 20,357 / Static 10,544), Data 78,807, Text 13,377, Padding 7,998, Image 131,072, Unknown 5.
  Per-bank: analysis/evidence/round-03/perbank.md (unknown: b1 3, b15 2; others 0).
- Remaining unknown: 0x02DC6, 0x032FA, 0x03353 (bank-1 lone RTS $ADB6/$B2EA/$B343), 0x1E106, 0x1E1C3 (bank-15 lone NOP
  $E0F6/$E1B3) -> lead L-021 for redirect. Status: INCOMPLETE (rounded 100.00% is not completion).
- Toolkit: no new issues; TK-001..TK-004 unchanged (analysis/toolkit-issues.md). Server stopped.
- Handoff: analysis/handoffs/round-03-disassembly.md.

### Round 3 commentary — 2026-11-20 — DONE
- Map changed (pass-002 -> pass-003): round-2 commentary preserved in analysis/round-2/ (diff -r identical + tools + handoff);
  commentary/ re-prepared from pass-003; annotations carried by bank:CPU and re-checked
  (analysis/evidence/round-03/commentary-carry-check.txt); 21 stale pass-002 unknown-range notes replaced.
- All ~1,087 bytes of new Static code/data named and documented as statically inferred (no caller known), e.g.
  FishShadow_TryBiteAlt, Bite_RollChance(AndHook), FishAheadHandlers (.word table), Mul16x8, BitArray_*, FormatByte3_*,
  QueueWordDecimal_Pad*, ScriptSub_ShowAndWaitButton, StopLoop; 5 unknown bytes noted (L-021).
- Counts: 0 address-only names, 0 numbered generic labels, 0 raw RAM operands, 705 routine headers; review.json 3,272
  entries reviewed (3,242 carried, 30 new), 1,079 units, editorial_audit pending.
- Verification: commentary.py verify ok, byte_identical true; reading-after.asm refreshed. No toolkit issues.
  Handoff: analysis/handoffs/round-03-commentary.md.


### Round 3 redirect — 2026-11-20 — DONE (ready leads: L-022, L-023)
- Input checkpoint pass-003 (5 unknown; L-021). Attempt dir analysis/redirect/r03-a/ (research only; no map/source/toolkit change, no emulator run).
- Method (coordinator questions): addr-2..+2 words, consumer-anchored split lo/hi tables (634 indexed-load pairs, idx 0-255),
  imm pairs within 16 B, RTS-dispatch existence (only $DE2D inline-arg helper), script operand/hook domains, bank-relative
  encodings; twin/analogue endings; whole-ROM consistency census (JMP;RTS followers, NOP sites, inter-code islands, fill value).
- Outcome: no reference/consumer in any encoding; tail-call twin model negative (analogues end in JMP without RTS); no filler
  mechanism in ROM (Padding all $FF, no alignment, other islands consumer-anchored); inconsistency found: StopLoop NOP $E0F1
  accepted as Static Code with the same evidence class as $E0F6/$E1B3.
- Leads: L-021 superseded -> L-022 ready (NOPs $E0F6/$E1B3, Unknown->Code 2, Static 0.6), L-023 ready (RTS $ADB6/$B2EA/$B343,
  Unknown->Code 3, Static 0.55; if rejected, these 3 bytes are terminal residual). No servers started. No toolkit issue.
- Handoff: analysis/handoffs/round-03-redirect.md.

### Round 4 disassembly — 2026-11-20 — DONE (INCOMPLETE: 3 unknown)
- Input pass-003. Independent evidence-policy review of L-022/L-023; r03-a refcheck/twins re-run unchanged; new control-flow
  xref (analysis/evidence/round-04/xref.txt): no JMP/JSR/branch to any residual.
- L-022 accepted: b15 $E0F6 (0x1E106), $E1B3 (0x1E1C3) Unknown->Code 2, Static 0.6 (NOP falling into observed code; exact bounds;
  no filler mechanism; consistent with accepted $E0F1, which is kept). Reachability not claimed.
- L-023 rejected: b1 RTS $ADB6/$B2EA/$B343 stay Unknown ("An unreferenced return opcode ... lacks sufficient structure on its own";
  no control/stack contract, twin test negative, no precedent) -> terminal residual with review in the handoff.
- Checkpoint pass-004: maps/pass-004.json, generated/pass-004, analysis/pass-004-verification.json — ok, byte_identical true,
  map sha256 4582999d1633428d4649053bfd38a65f0bc93eb15ee2b6491cbfe7281175979a (= maps/current.json = analysis/state.json).
- ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).
- Code 30,903 (Observed 20,357 / Static 10,546), Data 78,807, Text 13,377, Padding 7,998, Image 131,072, Unknown 3.
  Per-bank: analysis/evidence/round-04/perbank.md (unknown: b1 3; others 0).
- Remaining unknown: 0x02DC6, 0x032FA, 0x03353. No toolkit issues found (TK-001..TK-004 unchanged). Server stopped.
- Handoff: analysis/handoffs/round-04-disassembly.md.

### Round 4 commentary — 2026-11-20 — DONE
- Map changed (pass-003 -> pass-004): round-3 commentary preserved in analysis/round-3/ (diff -r identical + tools + handoff);
  commentary/ re-prepared from pass-004; annotations carried by bank:CPU and re-checked
  (analysis/evidence/round-04/commentary-carry-check.txt; all records valid).
- Only change: the two former-unknown bank-15 NOP notes became code headers NopBeforeWaitFrame/NopBeforeIrq (L-022; Static,
  no known reference, not claimed unused); the 3 bank-1 RTS unknown notes now cite the L-023 rejection.
- Counts: 0 address-only names, 0 numbered generic labels, 0 unreviewed scaffold entries (3,272) / units (1,081); 3/3 unknown bytes noted.
- Verification: commentary.py verify ok, byte_identical true (game.asm sha256 938f3bed...); reading-after.asm refreshed. No toolkit issues.
  Handoff: analysis/handoffs/round-04-commentary.md.

### Round 4 redirect (attempt r04-a) — 2026-11-20 — DONE (no ready lead; INCOMPLETE: 3 unknown)
- Input pass-004. Coordinator question: data role of 0x02DC6/0x032FA/0x03353 via runtime consumers ((zp),y, (zp,x), computed).
- Method: analysis/redirect/r04-a/readwatch.py (+readwatch2.py): Read+Instruction+Dma trace on physical [0x2DC6,0x3354) from
  same-build snapshots; 20 runs, 9,714 frames across S01, S04-S16 (fight x7 incl. Hawaii; landing, weigh-in, day end, results,
  ending FIN, Hawaii). Result: 0 events at the 3 offsets (no read, fetch or DMA) while preceding tail JMPs ran ($B2E7 ~2,600x,
  $B340 ~760x, $ADA9 fish_away 180x artificial). Hi-byte immediate scan for (zp),y bases: none near the targets.
- Outcome: consumer-anchored data role ruled out in exercised states; dead-epilogue vs stray byte undiscriminated; L-023 stays
  rejected (attempt appended); final rejection review in the handoff; terminal residual. No map/source changes.
- Server started and stopped; no toolkit issues (TK-003/TK-004 reproduced). Handoff: analysis/handoffs/round-04-redirect.md.

### Final acceptance (editorial audit) — 2026-11-20 — DONE (editorial audit passed; ROM INCOMPLETE: 3 unknown)
- Targeted final editorial pass on pass-004 commentary (review credit reused; no whole-source reread). Classification
  unchanged: 262157/262160, 3 unknown bytes (bank 1 RTS 0x02DC6 $ADB6, 0x032FA $B2EA, 0x03353 $B343).
- Fixed via annotations.txt/builder: 4 address-only RAM names (EngineLevel $07AF, RamPage05 $0500, HarbourReturnFlag
  $05F6 tentative, TourWrapFlags $04A0); ScriptArgC6/C8/CA roles; ScrCmd_End "unused entries" -> slots $16/$17; $ADB6
  "dead epilogue" wording; FatigueDrainFlag/BitArrayPtr "no writer found"; L-016 bank notes "no bank-select path maps";
  bank-12 $98B5 probable script bytecode note; uncertainties.md superseded note + final section.
- commentary.py audit: ok, byte_identical, editorial_audit true, 3,272 regions / 1,081 units reviewed (commentary/audit.json;
  game.asm sha256 54734f86...). reading-after.asm refreshed. Report: commentary/editorial-audit.md.
- analysis/current-audit.json / analysis/annotations.json not produced (no documented command/workflow step in this
  workspace defines them). No toolkit issues. Handoff: analysis/handoffs/final-acceptance.md.

### Coordinator terminal decision — 2026-11-20
- Accepted round-4 redirect rejection review for 3-byte residual (bank1 0x02DC6, 0x032FA, 0x03353); final acceptance editorial audit passed (commentary/audit.json). Final handoff analysis/handoffs/final-coordinator.md; archive-options.json prepared. Lead IDs: consumed L-004/005/009/010/012/013/015/016/019/020/022 (accepted), L-023 (rejected); L-021/L-017 superseded.

### User-directed residual fallback — 2026-11-20 — APPLIED (batch-only user exception)
- Explicit user authorization (batch-only, residual <= 10 bytes): mark remaining unclassified bytes generic Data, unknown
  meaning, Manual provenance; not an observed or statically proven role. Prior evidence and the L-023 rejection preserved.
- Complement re-verified from analysis/state.json (pass-004): exactly 3 bytes, phys 0x02DC6/11718, 0x032FA/13050,
  0x03353/13139 (bank 1 $ADB6/$B2EA/$B343, each $60 after a tail `jmp`); matches the user list.
- Applied: regions 3315-3317 Data/Manual/confidence 0.0 (analysis/evidence/final-fallback/apply_manual_fallback.py);
  validate ok; state/current sha256 e401cd2f...; checkpoint pass-005 ok, byte_identical true.
- Counts: 0 unknown; 262,157 evidence-backed + 3 Manual fallback bytes. Code 30,903, Data 78,810 (3 Manual), Text 13,377,
  Padding 7,998, Image 131,072. Handoff: analysis/handoffs/final-fallback.md.
- Next: commentary refresh for pass-005 (carry round-4 commentary, replace 3 unknown notes with fallback notes), then
  final review + strict editorial audit.

### Round 5 commentary + final review/strict editorial audit — 2026-11-20 — DONE
- Round-4 commentary preserved in analysis/round-4/ (diff -r identical); commentary/ re-prepared from pass-005; carry via
  analysis/tools/r05c/ (analysis/evidence/round-05/commentary-carry-check.txt): only the 3 unknown notes invalidated, replaced by
  Manual-fallback notes (unknown meaning; no label; prior L-023 ambiguity cited).
- review.json 3,272 entries (3,269 carried, 3 re-reviewed) / 1,081 units; commentary.py verify ok, byte_identical (game.asm
  sha256 e06730f4...); reading-after.asm refreshed; commentary.py audit ok, byte_identical, editorial_audit true (re-verified by
  coordinator). Handoff: analysis/handoffs/round-05-commentary.md.

### Coordinator final decision (post-fallback) — 2026-11-20
- Final checkpoint pass-005 (byte_identical). 0 unknown bytes: 262,157 evidence-backed + 3 Manual user-directed fallback
  (0x02DC6/0x032FA/0x03353), reported separately; the 3 bytes' code-vs-data role remains unproven. Final handoff addendum in
  analysis/handoffs/final-coordinator.md; archive-options.json updated (all analysis/commentary Markdown accounted for).
  Nothing archived, deleted or published; toolkit untouched.
