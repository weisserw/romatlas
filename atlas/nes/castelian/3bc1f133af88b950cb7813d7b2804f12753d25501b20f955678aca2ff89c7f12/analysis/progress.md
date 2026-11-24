# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Coordinator log

- Coordinator: goal.md (sequential phases disassembly -> commentary -> redirect).
- Disk at start: 3.0G free of 29G (90% used) — keep scratch compact.

### Round 1
- Phase: disassembly — IN PROGRESS (worker launched 2026-11-24). Input checkpoint: none (fresh workspace, empty map).

#### Round 1 disassembly pass (worker report, 2026-11-24)
- Baseline created: maps/baseline.json, generated/baseline, analysis/baseline-verification.json (byte_identical true).
- Output checkpoint: **pass-001** (maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json):
  ok=true, byte_identical=true, map_sha256 21a4a6bf92e38b70721466baef5bb1678a40d11854f5996071bc531931e1a946.
  analysis/state.json saved; maps/current.json exported and validated (`romatlas map validate`).
- ROM classification: 68.00% (89140 / 131088 bytes classified; 41948 bytes unknown).
- By kind/provenance: Code Observed 26,621; Code Static 9,087; Data Static 53,432 (incl. 16-byte header); 495 regions.
- Per 8 KiB PRG bank (code / data / unknown bytes); header 16 bytes data:

| 8K bank | 16K bank | CPU | code | data | unknown |
|---|---|---|---|---|---|
| 0 | 0 | $8000 | 0 | 8192 | 0 |
| 1 | 0 | $A000 | 0 | 8192 | 0 |
| 2 | 1 | $8000 | 0 | 8192 | 0 |
| 3 | 1 | $A000 | 0 | 8192 | 0 |
| 4 | 2 | $8000 | 0 | 8192 | 0 |
| 5 | 2 | $A000 | 225 | 7967 | 0 |
| 6 | 3 | $8000 | 894 | 0 | 7298 |
| 7 | 3 | $A000 | 3182 | 598 | 4412 |
| 8 | 4 | $8000 | 7698 | 186 | 308 |
| 9 | 4 | $A000 | 3550 | 1959 | 2683 |
| 10 | 5 | $8000 | 32 | 99 | 8061 |
| 11 | 5 | $A000 | 1706 | 8 | 6478 |
| 12 | 6 | $8000 | 3216 | 179 | 4797 |
| 13 | 6 | $A000 | 550 | 739 | 6903 |
| 14 | 7 (fixed) | $C000 | 7560 | 74 | 558 |
| 15 | 7 (fixed) | $E000 | 7095 | 647 | 450 |

- Execution coverage (separate metric): 11,498 unique ROM instruction addresses observed (8,445 in natural play), scenarios S01-S11 visited incl. all 8 towers, bonus stage, ending, continue/game over, 2P GAME DRAWN (several via RAM/register shortcuts; see analysis/scenarios.json).
- Leads: consumed none (fresh workspace); produced L1-01..L1-08 (analysis/leads.json); none verified/rejected this pass.
- Toolkit: 4 findings in analysis/toolkit-issues.md - TK-001 confirmed bug (NES emulator.input ignores documented button/pressed, silently clears input), TK-002 minor API (trace kinds null), TK-003 missing feature (static discovery stops at every JSR/indexed store on mapper 2), TK-004 missing feature (12x tracing overhead). No blockers.
- Status: pass completed, ROM incomplete (41,948 unknown bytes). Handoff: analysis/handoffs/round-1-disassembly.md.

#### Coordinator summary — round 1 disassembly
- Disassembly COMPLETE (pass completed, incomplete ROM). ROM classification: 68.00% (89140 / 131088 bytes classified; 41948 bytes unknown). Printed to user.
- Output checkpoint: pass-001 (maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json; byte_identical=true).
- Leads produced: L1-01..L1-08. Consumed: none. Handoff: analysis/handoffs/round-1-disassembly.md.
- Phase: commentary — IN PROGRESS (worker launched).

#### Round 1 commentary pass (worker report, 2026-11-24)
- Phase: commentary — COMPLETE (current-source readability for all classified source; editorial audit pending).
- Prepared commentary/ from pass-001 (maps/current.json unchanged). All 459 code regions, 36 data regions and 67 unknown ranges annotated (names, RAM symbols, block comments, unknown-range notes; no classification change: 68.00%, 41,948 unknown).
- Verification: commentary/verification.json ok=true, byte_identical=true; reading copy commentary/reading-after.asm refreshed.
- Tools/records: commentary/tools/notes.txt + overlay.py (source generator from before/), review.py; commentary/progress.md, uncertainties.md; review.json 562 entries reviewed, 689 units.
- Redirect questions (consumer-bounded data in banks 3/4/5/6 and fixed-bank tables, RAM routine origin bank 4 $B692, attract trigger): analysis/handoffs/round-1-commentary.md.
- Toolkit: TK-005 (NES generated data labels in .reloc mode) added; no blockers.

#### Coordinator summary — round 1 commentary
- Commentary COMPLETE (all classified source readable; audit reserved for final acceptance). Verification commentary/verification.json byte-identical; reading copy commentary/reading-after.asm. Handoff analysis/handoffs/round-1-commentary.md. TK-005 added.
- Phase: redirect — IN PROGRESS (worker launched).

#### Round 1 redirect (worker report, 2026-11-24)
- Phase: redirect - COMPLETE (attempt analysis/redirect/r1-a1; input checkpoint pass-001, map sha256 21a4a6bf...1a946). No map/state/source changes.
- Method: static bank-aware consumer/extent analysis (refs/imm/disasm tools), sound-data walker, exact unknown-complement ledger, one bounded idle-title runtime run on a store copy.
- Result: proposals for 41,946 of 41,948 unknown bytes (ledger.json); ready leads L1-01..L1-08 (updated) and L1-09 (new). Residual 2 bytes (bank 3 $B6B0, bank 4 $B685).
- Notable: L1-05 bank = 5 (explicit lda #$05); bank 5 $A9E5-$BFE4 bonus maps (tower-map hypothesis rejected); bank 6 chained tables; bank 3 tower palettes $BD38; chain blocks $909D/$9864 via overlapping RTS tables; attract mode visited.
- Recommendation: another round (disassembly of the ready batch). Handoff: analysis/handoffs/round-1-redirect.md.

#### Coordinator summary — round 1 redirect
- Redirect COMPLETE (attempt analysis/redirect/r1-a1/). Ready leads L1-01..L1-09 cover 41,946/41,948 unknown bytes (2 bytes residual: bank3 $B6B0, bank4 $B685). Demo mode visited. Recommendation: new round. Handoff analysis/handoffs/round-1-redirect.md.

### Round 2
- Phase: disassembly — IN PROGRESS (worker launched). Input checkpoint pass-001; consuming leads L1-01..L1-09.

#### Round 2 disassembly pass (worker report, 2026-11-24)
- ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown).
- Input pass-001 (68.00%, 41,948 unknown). Output checkpoint **pass-002**: maps/pass-002.json,
  generated/pass-002, analysis/pass-002-verification.json: ok=true, byte_identical=true,
  map_sha256 9a7a6642983ec3c5be698fa7c04cb47ec5e35dab796b8cc0e5a776595837f0f5.
  analysis/state.json saved (571 regions), maps/current.json exported; `romatlas map validate` ok.
- By kind/provenance: Code Observed 27,134; Code Static 9,164; Data Static 94,788 (incl. 16-byte header).
- Per 8 KiB PRG bank (code / data / unknown); header 16 bytes data:

| 8K bank | 16K bank | CPU | code | data | unknown |
|---|---|---|---|---|---|
| 0 | 0 | $8000 | 0 | 8192 | 0 |
| 1 | 0 | $A000 | 0 | 8192 | 0 |
| 2 | 1 | $8000 | 0 | 8192 | 0 |
| 3 | 1 | $A000 | 0 | 8192 | 0 |
| 4 | 2 | $8000 | 0 | 8192 | 0 |
| 5 | 2 | $A000 | 225 | 7967 | 0 |
| 6 | 3 | $8000 | 894 | 7298 | 0 |
| 7 | 3 | $A000 | 3182 | 5009 | 1 |
| 8 | 4 | $8000 | 8006 | 186 | 0 |
| 9 | 4 | $A000 | 3605 | 4586 | 1 |
| 10 | 5 | $8000 | 32 | 8160 | 0 |
| 11 | 5 | $A000 | 1881 | 6311 | 0 |
| 12 | 6 | $8000 | 3216 | 4976 | 0 |
| 13 | 6 | $A000 | 550 | 7642 | 0 |
| 14 | 7 (fixed) | $C000 | 7560 | 632 | 0 |
| 15 | 7 (fixed) | $E000 | 7147 | 1045 | 0 |

- Unknown: bank 3 $B6B0 (phys 63168, $60) and bank 4 $B685 (phys 79509, $EA) -> lead L2-01 (blocked).
- Lead outcomes (analysis/leads.json): L1-01, L1-02, L1-03, L1-04, L1-08 verified (Data Static, full
  ranges); L1-05, L1-09 verified (code Observed in natural attract run S12; pattern table Data);
  L1-06 verified incl. orphans $E4B3/$E585 as Static code confidence 0.6 after critical review,
  $B692 RAM-routine image as Static code (storage view); L1-07 verified (route, reused). None rejected.
  Correction: leads.json phys bounds for L1-01 sub-ranges after $9BAE were off by 768 (ledger used).
- Verification of leads: analysis/evidence/round2/tools/verify_leads.py (no observed fetch in any
  Data proposal; all fixed-bank JSR/JMP into proposed data windows land on code of other banks;
  consumers re-checked); apply script apply_leads.py. Details analysis/investigation.md Round 2.
- Gameplay: S13 PLAYER 1 WINS visited (artificial RAM patch; route in analysis/scenarios.json;
  analysis/evidence/round2/s13-player1-wins.png); $F4ED-$F4FC upgraded to Observed, $F4FD
  (PLAYER 2 WINS branch) stays Static. No high-score name-entry exists (HISCORE display only).
- Toolkit: no new toolkit issues this pass (TK-001..TK-005 unchanged, analysis/toolkit-issues.md);
  note run_until per-call budget <=1M operations / 30 s is documented behaviour, worked around by looping.
- Status: pass completed; ROM incomplete (2 unknown bytes). Handoff analysis/handoffs/round-2-disassembly.md.

#### Coordinator summary — round 2 disassembly
- Disassembly COMPLETE. ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown). Printed to user (rounded; NOT complete).
- Checkpoint pass-002 (maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json; byte-identical). Leads consumed L1-01..L1-09 (all accepted; L1-06 $E4B3/$E585 conf 0.6 static). Produced L2-01 (2 residual bytes bank3 $B6B0, bank4 $B685; blocked/ambiguous). S13 PLAYER 1 WINS visited. Handoff analysis/handoffs/round-2-disassembly.md.
- Phase: commentary — IN PROGRESS (worker launched). Map changed vs round-1 baseline → preserve round-1 commentary under analysis/round-1/.

#### Round 2 commentary pass (worker report, 2026-11-24)
- Map changed vs round-1 baseline: full round-1 commentary preserved at analysis/round-1/commentary/;
  fresh baseline prepared from pass-002 (573 scaffold entries).
- Carried round-1 names/comments/review by physical identity (tools/notes.txt + overlay.py); rewrote only
  affected records; documented all 79 new/changed regions (12 code, 65 data, 2 unknown bytes), 130 new data
  labels, ~240 raw ROM operands and ~130 raw comment addresses replaced by names, defects fixed
  (TowerParamTables+N split, OccludeMaskTableC bounds, misleading DrawObjects_next,y operand, stale notes).
- review.json: 573/573 reviewed, 831 units; editorial_audit pending (final acceptance).
- Verification: commentary.py verify ok, byte_identical; reading copy refreshed.
- Status: COMPLETE (current-source readability). Classification still 131086/131088 (2 unknown, L2-01).
  Handoff analysis/handoffs/round-2-commentary.md.

#### Coordinator summary — round 2 commentary
- Commentary COMPLETE on pass-002 (573/573 reviewed; round-1 commentary preserved at analysis/round-1/commentary/). Byte-identical verification; reading copy refreshed. Handoff analysis/handoffs/round-2-commentary.md.
- Phase: redirect — IN PROGRESS (worker launched). Focus: L2-01 residual 2 bytes, rejection review, $F4FD PLAYER 2 WINS, bank3 $A500-$AEFF unreferenced data.

#### Round 2 redirect (worker report, attempt r2-a1, 2026-11-24)
- Input pass-002 (131086/131088; 2 unknown). No map/source changes. Research analysis/redirect/r2-a1/.
- L2-01 terminal rejection review (notes.md): no reference/entry, no ROM-wide dead-rts or pad
  convention, neither byte belongs to the adjacent data format; unknown retained (2 bytes).
- Gameplay: S13b PLAYER 2 WINS visited ($F4FD fetched); new S14 HERO game (HERO-only paths incl.
  $CDB1 executed). All checklist milestones now visited (analysis/scenarios.json).
- Leads: L2-02 ready (provenance-only Observed upgrades, 0 bytes); L2-01 and L1-06 attempts appended.
- Recommendation: terminal for classification (2 bytes); optional L2-02 application; then final
  audit. Handoff analysis/handoffs/round-2-redirect.md.

#### Coordinator summary — round 2 redirect
- Redirect COMPLETE (attempt analysis/redirect/r2-a1/). L2-01 rejection review (r2-a1/notes.md §L2-01) inspected by coordinator: sound — both residual bytes are lone valid opcodes with no reference, no format membership, no ROM-wide pad/jmp+rts convention; both readings predict non-execution; no discriminating test. Accepted as terminal for those 2 bytes on unchanged premises.
- S13b (PLAYER 2 WINS, $F4FD observed) and S14 (HERO difficulty) visited; all checklist milestones visited.
- Ready lead L2-02 (observed-provenance upgrade for 7 regions/180 B + split 9 partially executed regions; coverage saved). Concrete useful step → round 3.

### Round 3
- Phase: disassembly — IN PROGRESS (worker launched). Input checkpoint pass-002; consuming L2-02.

#### Round 3 disassembly pass (worker report, 2026-11-24)
ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown).
- Rounded figure only; NOT complete: unknown bank 3 $B6B0 (phys 63168) and bank 4 $B685 (phys 79509), L2-01 (terminal, untouched; no new evidence).
- Input pass-002 restored from analysis/state.json (571 regions); all classifications preserved. Output 599 regions.
- Per 16 KiB PRG bank (Observed code / Static code / data / unknown); header 16 B classified:

| 16K bank | code Obs | code Static | data | unknown |
|---|---|---|---|---|
| 0 | 0 | 0 | 16384 | 0 |
| 1 | 0 | 0 | 16384 | 0 |
| 2 | 225 | 0 | 16159 | 0 |
| 3 | 3853 | 223 | 12307 | 1 |
| 4 | 9238 | 2373 | 4772 | 1 |
| 5 | 1697 | 216 | 14471 | 0 |
| 6 | 3211 | 555 | 12618 | 0 |
| 7 (fixed) | 10032 | 4675 | 1677 | 0 |

- Lead L2-02 VERIFIED (provenance only, 0 classification bytes): saved toolkit analysis.coverage lists
  (analysis/redirect/r2-a1/evidence/s13b-p2wins-coverage.json, s14-hero-coverage.json) applied with
  analysis/evidence/round3/tools/apply_l202.py; no re-run needed. 7 fully fetched Static code regions
  (146 B) -> Observed; 9 partially fetched regions split at instruction starts into 37 Observed/Static
  runs (existing per-run convention). Code bytes Observed 27134 -> 28256, Static 9164 -> 8042.
  Evidence text records that S13b/S14 are artificial RAM-patched states: observed execution, not natural
  reachability. Change log analysis/evidence/round3/l2-02-changes.json.
- L2-01: not reopened (no new evidence). No gameplay run this pass (all milestones visited).
- Verification: map.validate ok; maps/current.json exported; checkpoint pass-003 (maps/pass-003.json,
  generated/pass-003, analysis/pass-003-verification.json) byte_identical=true, map sha256 9baf6bef...b6;
  input/game.nes sha256 unchanged (3bc1f133...).
- Toolkit: no new toolkit issues (analysis/toolkit-issues.md, TK-001..TK-005 unchanged).
- Status: pass completed; ROM incomplete (2 unknown bytes). Handoff analysis/handoffs/round-3-disassembly.md.

#### Coordinator summary — round 3 disassembly
- Disassembly COMPLETE. ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown). Printed to user (rounded; NOT complete).
- Checkpoint pass-003 (byte-identical). L2-02 consumed/verified: 7 regions Static->Observed, 9 regions split into 37 pieces (IDs 1938-1965); 1122 code bytes now Observed (artificial RAM-patched scenarios). No ready leads remain. Handoff analysis/handoffs/round-3-disassembly.md.
- Phase: commentary — IN PROGRESS (worker launched).

#### Round 3 commentary pass (worker report, 2026-11-24)
- COMPLETE on pass-003. Round-2 commentary preserved at analysis/round-2/commentary/ (verified identical);
  fresh baseline (601 entries); carry-over by physical identity, no lost names/comments.
- Affected records updated ($F4FD S13b note, CheckExtraLifeHero static-only award, split notes, provenance-split
  preamble); whole-source sweep replaced ~70 raw addresses in prose with names and fixed wrong round-1 descriptions;
  11 trampolines renamed; raw RAM/APU operands symbolised. review.json 601/601 reviewed (857 units); audit pending.
- Verify: ok, byte_identical (game.asm 063b8aa6...); reading copy refreshed. Classification unchanged (2 unknown bytes).
- Handoff: analysis/handoffs/round-3-commentary.md.

#### Coordinator summary — round 3 commentary
- Commentary COMPLETE on pass-003 (601/601 reviewed; round-2 commentary preserved at analysis/round-2/commentary/). Byte-identical verification; reading copy refreshed. Handoff analysis/handoffs/round-3-commentary.md.
- Phase: redirect — IN PROGRESS (worker launched).

#### Round 3 redirect (worker report, attempt r3-a1, 2026-11-24)
- COMPLETE on pass-003. Checked only changed evidence since r2-a1 (round-3 provenance splits, commentary prose
  corrections, uncertainties.md): no new reference/inference for L2-01, orphan code $A019/$A0EC/$E4B3/$E585 or bank 3
  $A500-$AEFF. New split-table/pha-pha-rts/jmp(vector) decode: negative. r2-a1 rejection review reused (premises unchanged).
- No ready leads; leads.json pass + L2-01 attempt appended. Recommendation: terminal incomplete (2 unknown bytes,
  phys 63168/79509); proceed to final editorial audit. Handoff analysis/handoffs/round-3-redirect.md; notes analysis/redirect/r3-a1/notes.md.

#### Coordinator summary — round 3 redirect / TERMINAL
- Redirect COMPLETE (r3-a1): no changed evidence; r2-a1 rejection review reused; coordinator re-inspected and accepts terminal stop on unchanged sound premises.
- WORKFLOW TERMINAL — INCOMPLETE ROM. ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown).
- Final checkpoint pass-003; commentary current (byte-identical); editorial audit reserved for final acceptance/archive.
- Final handoff: analysis/handoffs/final-terminal.md. archive-options.json written (documents: handoffs, redirect notes, historical round commentary progress/uncertainties; omitted: duplicated toolkit instruction copies analysis/round-{1,2}/commentary/{AGENTS,prompt,workflow}.md, byte-identical to commentary/ versions). No archive executed.

### Final batch exception — user-directed manual fallback (2026-11-24)
- Explicit user authorization (batch-only exception, valid only when <=10 physical bytes remain unclassified):
  mark the residual bytes as generic Data with UNKNOWN meaning, Manual provenance. This is a user-directed
  fallback, NOT an observed or statically proven data role; it overrides the workspace structural-proof
  requirement for these residual bytes only. No redirect research reopened.
- Exact complement verified before editing (analysis/state.json, pass-003, map sha256 9baf6bef...b6; 599 regions,
  no overlaps): exactly 2 bytes — phys 63168 (16K bank 3 CPU $B6B0) and phys 79509 (16K bank 4 CPU $B685), lead L2-01.
- Prior ambiguity (preserved, unchanged): r2-a1 rejection review (analysis/redirect/r2-a1/notes.md §L2-01, reused
  r3-a1): no reference (incl. split/RTS/vector tables), no membership in adjacent data formats, no ROM-wide pad or
  jmp+rts convention; code and padding readings both predict non-execution; code-vs-data materially ambiguous.
- Applied: analysis/evidence/final-fallback/tools/apply_fallback.py -> map.annotate kind Data, provenance Manual,
  confidence 1.0 (assertion only), length 1 each; region ids 1966 (phys 63168) and 1967 (phys 79509);
  change log analysis/evidence/final-fallback/changes.json. ROM bytes unchanged (sha256 3bc1f133... verified).
- map.validate ok; map.save analysis/state.json (601 regions, 131088 bytes); map.export maps/current.json;
  `romatlas map validate --rom` ok. Checkpoint **pass-004** (maps/pass-004.json, generated/pass-004,
  analysis/pass-004-verification.json): ok=true, byte_identical=true, map sha256 c6868c2f...2f43.
- Classification accounting (report separately):
  - Evidence-backed (Observed/Static) classification: 131086 / 131088 bytes (unchanged from pass-003).
  - Manual user-directed fallback (generic Data, meaning unknown, no evidence-backed role): 2 bytes.
  - Unknown: 0 bytes. Structural map coverage 100% (131088/131088), of which 2 bytes are manual fallback only.
- Phase: commentary (final) — fallback reconciliation + final review + strict editorial audit — IN PROGRESS.

#### Final commentary + editorial audit (worker report, 2026-11-24)
- COMPLETE; strict audit PASSED. Round-3 commentary preserved at analysis/round-3/commentary/ (diff -r identical);
  fresh baseline from pass-004 (map sha256 c6868c2f...2f43; 601 entries); names/comments/review carried by physical
  identity, only the two 1-byte records invalidated.
- Phys 63168 (bank 3 $B6B0) / 79509 (bank 4 $B685): source notes and review summaries state generic data, meaning
  unknown, manual user-directed fallback (not evidence-backed), prior L2-01 code-vs-pad ambiguity; no label/role.
- Final review fixes: stale RAM aliases ($0614 SoundTempo, $0615 SoundTempoCopy, $0618 SoundRestartFlag,
  ObjDrawFrame, ShotTimer, BoxAngle), unused symbols removed, Ch1/Ch2_Command names.
- `commentary.py audit`: ok, byte_identical, 601 regions / 857 units reviewed, editorial_audit reviewed;
  game.asm sha256 65b0b106...c451c; reading copy refreshed. input/game.nes unchanged (3bc1f133...). No toolkit issues.
- Handoff: analysis/handoffs/final-commentary-audit.md.

#### Coordinator summary — final batch exception / FINAL
- Commentary worker report (above) inspected; coordinator re-ran `commentary.py audit`: ok, byte_identical, 601/601 regions,
  857/857 units, editorial_audit true; archive.editorial_status complete=true. ROM sha256 unchanged; maps/current.json ==
  pass-004 == commentary/baseline.json (c6868c2f...2f43); source game.asm 65b0b106...451c.
- Classification report: evidence-backed 131086/131088 bytes; MANUAL user-directed fallback 2 bytes (phys 63168, 79509;
  generic Data, meaning unknown, not evidence-backed); unknown 0.
- Retained final conclusions: addendum in analysis/handoffs/final-terminal.md; analysis/handoffs/final-commentary-audit.md.
- archive-options.json updated: + final-commentary-audit.md, round-3 commentary progress/uncertainties (documents);
  round-3 AGENTS/prompt/workflow copies (byte-identical to commentary/ versions) omitted; final_fallback_record added.
  All analysis/ and commentary/ Markdown accounted for under archive.py rules. No archive, cleanup or toolkit edits.
- Disposable scratch left in place: scratch/commentary-r3-working (identical duplicate of analysis/round-3/commentary/).
- Orchestration goal reached terminal handoff with passing final audit.
