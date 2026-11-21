# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Coordinator log (goal.md orchestration)

| Round | Phase | Status | Input checkpoint | Output checkpoint | Verification | Leads consumed/produced | Handoff |
|---|---|---|---|---|---|---|---|
| 1 | disassembly | complete (easy-gain exhaustion) | (none; fresh workspace) | pass-001 | byte-identical ok; 65.78% (107792/163856; 56064 unknown) | produced L-001..L-009 | analysis/handoffs/round-1-disassembly.md |
| 1 | commentary | complete (gate checked by coordinator: verify ok/byte-identical, reading copy refreshed, no address-only labels; in-routine block comments thinner outside core systems -> final-acceptance backlog) | pass-001 | commentary/source (from pass-001) | commentary/verification.json ok | produced L-010..L-013; updated L-001,L-002,L-004,L-005,L-007 | analysis/handoffs/round-1-commentary.md |
| 1 | redirect | complete -> NEXT ROUND (attempt r1-a1; proposal 48538 bytes in analysis/redirect/r1-a1/proposal.json; residual ledger 7526 bytes) | pass-001 + round-1 commentary | (research only) | map/state/source unchanged | consumed L-001..L-013 review; ready L-010,L-005,L-015,L-014,L-001,L-011,L-012,L-013,L-007,L-008,L-016 (+partial L-002,L-009,L-003,L-006) | analysis/handoffs/round-1-redirect.md |
| 2 | disassembly | complete (easy-gain exhaustion) | pass-001 | pass-002 | byte-identical ok; 99.99% (163833/163856; 23 unknown) | consumed r1-a1 proposal: all leads accepted (48,515 bytes; 23 left unknown); L-002 superseded; produced L-017 | analysis/handoffs/round-2-disassembly.md |
| 2 | commentary | complete (gate checked: verify ok/byte-identical, reading copy refreshed, round-1 preserved at analysis/round-1/commentary/, 1313/1313 review entries) | pass-002 | commentary/source (from pass-002) | commentary/verification.json ok | produced L-018,L-019; updated L-017 | analysis/handoffs/round-2-commentary.md |
| 2 | redirect | complete -> NEXT ROUND (attempt r2-a1; proposal 12 bytes code L-020; 11 bytes residual with rejection review in handoff §2; L-018 verified naturally SC-016) | pass-002 + round-2 commentary | (research only) | map/state/source unchanged | consumed L-017,L-018; produced L-020 (12B code), L-021 (commentary password-value fix); carried L-019 | analysis/handoffs/round-2-redirect.md |
| 3 | disassembly | complete (easy-gain exhaustion) | pass-002 | pass-003 | byte-identical ok; 99.99% (163845/163856; 11 unknown) | consumed L-020 (accepted 12B), L-019 (split, 0 net); L-017 remains (11B) | analysis/handoffs/round-3-disassembly.md |
| 3 | commentary | complete (gate checked: verify ok/byte-identical, reading copy refreshed, round-2 preserved at analysis/round-2/commentary/, 1315/1315 reviewed; L-021 applied). Final-acceptance backlog noted by coordinator: many branch labels are mechanical `<Routine>_SkipN/_LoopN`; 33 long routines rely on header + one comment | pass-003 | commentary/source (from pass-003) | commentary/verification.json ok | consumed L-021; no new leads | analysis/handoffs/round-3-commentary.md |
| 3 | redirect | complete -> TERMINAL incomplete (attempt r3-a1; coordinator-requested checks 1–3 all negative; updated rejection review §2) | pass-003 + round-3 commentary | (research only) | map/state/source unchanged | L-017 remains (11B); no ready leads | analysis/handoffs/round-3-redirect.md |

### Coordinator terminal decision (2026-11-21)
- Inspected round-3 rejection review and decisive artifacts (analysis/redirect/r3-a1/habit.txt, refscan.txt, ffret.txt, copies.txt). Counterexample scope measured (0/25 never-falling loops with dead RTS), reference forms covered incl. RTS-dispatch(-1) and split tables, same standard as accepted L-020. Sound; no corrected inference/test remains → stop **incomplete**.
- ROM classification: 99.99% (163845 / 163856 bytes classified; 11 bytes unknown). Residuals: phys 6173+1, 51444+3, 85771+5, 117501+1, 118162+1.
- Final checkpoint pass-003 (byte-identical); commentary verify ok; reading copy commentary/reading-after.asm; editorial audit pending (final acceptance).
- archive-options.json written (documents / omitted_documents; all analysis/ and commentary/ Markdown accounted for). Archive not executed.
- Final handoff: analysis/handoffs/final-terminal.md. Workflow termination is not ROM completion.

## Round 1 disassembly pass (2026-11-21)

- Baseline created (`python3 run.py baseline`, maps/baseline.json, verified byte-identical:
  analysis/baseline-verification.json).
- Checkpoint **pass-001**: maps/pass-001.json, generated/pass-001,
  analysis/pass-001-verification.json — ok=true, byte_identical=true
  (map sha256 d19c663aa267651c5c913ba1afea699b782c07f0c5439f9bf0a19de7c79f6815).
  analysis/state.json saved, maps/current.json exported, map.validate ok.

ROM classification: 65.78% (107792 / 163856 bytes classified; 56064 bytes unknown).

Totals: code 32,938 (Observed 18,986; Static 13,952), data+image 40,253, text 2,201,
padding 32,400. Regions: 648 Code, 90 Data, 140 Text, 8 Padding, 1 Image.

| Bank | Code | Data/Image | Text | Padding | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 | 0 |
| prg00 | 5292 | 4 | 294 | 0 | 2602 |
| prg01 | 0 | 2304 | 0 | 1716 | 4172 |
| prg02 | 3 | 1747 | 0 | 0 | 6442 |
| prg03 | 53 | 1221 | 0 | 1351 | 5567 |
| prg04 | 3 | 1216 | 0 | 0 | 6973 |
| prg05 | 53 | 8 | 0 | 771 | 7360 |
| prg06 | 6570 | 0 | 148 | 0 | 1474 |
| prg07 | 3216 | 93 | 0 | 771 | 4112 |
| prg08 | 1723 | 0 | 241 | 4389 | 1839 |
| prg09 | 0 | 8 | 0 | 8184 | 0 |
| prg10 | 5229 | 85 | 966 | 0 | 1912 |
| prg11 | 6025 | 8 | 514 | 1378 | 267 |
| prg12 | 956 | 730 | 0 | 0 | 6506 |
| prg13 | 0 | 29 | 0 | 1716 | 6447 |
| prg14 | 3815 | 0 | 38 | 3948 | 391 |
| prg15 | 0 | 16 | 0 | 8176 | 0 |
| chr | 0 | 32768 | 0 | 0 | 0 |

Evidence and methods: analysis/investigation.md. Scenarios: analysis/scenarios.json
(SC-001..SC-011). Execution coverage (separate metric): 11,737 contextual instruction
roots observed. Status: **incomplete** (56,064 bytes unknown). Pass ended at easy-gain
exhaustion for this round (gameplay gains had dropped to +45..+540 roots per mode).
Toolkit findings: 3 missing features (TK-001..TK-003), no blockers — see
analysis/toolkit-issues.md. Handoff: analysis/handoffs/round-1-disassembly.md.

## Round 1 commentary (worker section, 2026-11-21)

- Prepared commentary/ from pass-001 (frozen before/, baseline verified). Commented
  source: commentary/source/game.asm (built from before/ + commentary/tools/notes.txt by
  commentary/tools/build.py; bytes unchanged). Reading copy: commentary/reading-after.asm.
- Scope: all classified source (648 Code / 90 Data / 140 Text / 8 Padding / 1 Image
  regions) plus notes on 331 unknown gaps; review.json 1218/1218 entries reviewed,
  915 units; editorial_audit pending (final acceptance).
- Verification: `commentary.py verify` ok=true, byte_identical=true.
- Classification unchanged (65.78%, 56,064 bytes unknown). Leads produced L-010..L-013;
  updated L-001, L-002, L-004, L-005, L-007. Toolkit finding TK-004 (minor).
- Handoff: analysis/handoffs/round-1-commentary.md.

## Round 1 redirect (worker section, attempt r1-a1, 2026-11-21)

- Input pass-001 (unchanged); research-only (no map/state/source/ROM/toolkit edits).
- Proposal: 48,538 of 56,064 unknown bytes (analysis/redirect/r1-a1/proposal.json): DPCM samples/tables
  (L-010), bank 12/13 sound data parsed from SongTable (L-005), bank 0/1 screen-object chain (L-015),
  bank 7 trick-shot scripts + bank-5 residue (L-014), code closures (L-001 reopened via never-taken
  BNE at $9025, L-007, L-011, L-012, L-013). Residual ledger: 7,526 bytes (residual-ledger.json, L-016).
- Gameplay: TourStage RAM patch reached ending / sound test / voice test (artificial; SC-008, SC-012, SC-013).
- leads.json: produced L-014..L-016; updated L-001..L-013 (L-004 superseded). Recommendation: NEXT ROUND.
- Handoff: analysis/handoffs/round-1-redirect.md.

## Round 2 disassembly pass (2026-11-21)

- Input pass-001 (state restored with map.load; existing classifications preserved; baseline not
  re-created). Output checkpoint **pass-002**: maps/pass-002.json, generated/pass-002,
  analysis/pass-002-verification.json — ok=true, byte_identical=true (map sha256
  671ba03cfdce93bc009541110a705db3def36fbcbcee488907d39e628de2863d). map.validate ok;
  analysis/state.json saved; maps/current.json exported.

ROM classification: 99.99% (163833 / 163856 bytes classified; 23 bytes unknown).

Totals: code 35,293 (Observed 19,530; Static 15,763), data 61,171, image 32,768, text 2,201,
padding 32,400. Regions: 720 Code, 435 Data, 140 Text, 8 Padding, 1 Image.

| Bank | Code | Data/Image | Text | Padding | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 | 0 |
| prg00 | 6202 | 1692 | 294 | 0 | 4 |
| prg01 | 0 | 6476 | 0 | 1716 | 0 |
| prg02 | 9 | 8183 | 0 | 0 | 0 |
| prg03 | 53 | 6788 | 0 | 1351 | 0 |
| prg04 | 9 | 8183 | 0 | 0 | 0 |
| prg05 | 53 | 7368 | 0 | 771 | 0 |
| prg06 | 6931 | 1110 | 148 | 0 | 3 |
| prg07 | 3315 | 4106 | 0 | 771 | 0 |
| prg08 | 1937 | 1625 | 241 | 4389 | 0 |
| prg09 | 0 | 8 | 0 | 8184 | 0 |
| prg10 | 5587 | 1628 | 966 | 0 | 11 |
| prg11 | 6103 | 197 | 514 | 1378 | 0 |
| prg12 | 1004 | 7188 | 0 | 0 | 0 |
| prg13 | 0 | 6476 | 0 | 1716 | 0 |
| prg14 | 4090 | 111 | 38 | 3948 | 5 |
| prg15 | 0 | 16 | 0 | 8176 | 0 |
| chr | 0 | 32768 | 0 | 0 | 0 |

- Accepted (after independent review) all r1-a1 proposal entries: 48,591 bytes (incl. the optional
  bank-7 - item, accepted as residue data at confidence 0.7). Then the residual ledger
  (L-016/L-008/L-009): 7,450 bytes (6,712 data, 738 Static code from 16 clean unreferenced/fallthrough
  routines). 23 bytes left unknown (9 tiny code-shaped fragments). Details: analysis/investigation.md.
- Gameplay: SC-014 Trick Shot (traced), SC-015 tour cities 1-4 + natural ending through the in-game
  A+B+SELECT match-result shortcut (natural inputs, no RAM edits; traced stage-won + ending start).
  Execution coverage (separate metric): 5,044 contextual roots this round, 0 in Data/unknown bytes;
  23 Static code regions (544 bytes) upgraded to Observed.
- Toolkit: no new toolkit issues found this pass (TK-001..TK-004 unchanged; analysis/toolkit-issues.md).
- Status: **incomplete** (23 bytes unknown). Pass ended at easy-gain exhaustion. Handoff:
  analysis/handoffs/round-2-disassembly.md.

## Round 3 disassembly pass (2026-11-21)

- Input pass-002 (analysis/state.json restored with map.load; existing classifications preserved;
  baseline not re-created). Output checkpoint **pass-003**: maps/pass-003.json, generated/pass-003,
  analysis/pass-003-verification.json — ok=true, byte_identical=true (map sha256
  e4d17a5803fbf13fd0a3d931c2cfe936784e571914921c0ebec229276930e0ff). map.validate ok;
  analysis/state.json saved; maps/current.json exported (identical to pass-003 map). ROM unchanged.

ROM classification: 99.99% (163845 / 163856 bytes classified; 11 bytes unknown).

Totals: code 35,305 (Observed 19,530; Static 15,775). Regions: 724 Code, 437 Data, 140 Text, 8 Padding, 1 Image.

| Bank | Code | Data/Image | Text | Padding | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 | 0 |
| prg00 | 6205 | 1692 | 294 | 0 | 1 |
| prg01 | 0 | 6476 | 0 | 1716 | 0 |
| prg02 | 9 | 8183 | 0 | 0 | 0 |
| prg03 | 53 | 6788 | 0 | 1351 | 0 |
| prg04 | 9 | 8183 | 0 | 0 | 0 |
| prg05 | 53 | 7368 | 0 | 771 | 0 |
| prg06 | 6931 | 1110 | 148 | 0 | 3 |
| prg07 | 3315 | 4106 | 0 | 771 | 0 |
| prg08 | 1937 | 1625 | 241 | 4389 | 0 |
| prg09 | 0 | 8 | 0 | 8184 | 0 |
| prg10 | 5593 | 1628 | 966 | 0 | 5 |
| prg11 | 6103 | 197 | 514 | 1378 | 0 |
| prg12 | 1004 | 7188 | 0 | 0 | 0 |
| prg13 | 0 | 6476 | 0 | 1716 | 0 |
| prg14 | 4093 | 111 | 38 | 3948 | 2 |
| prg15 | 0 | 16 | 0 | 8176 | 0 |
| chr | 0 | 32768 | 0 | 0 | 0 |

- L-020 verified/accepted: 12 bytes Static code (4 "JMP ENDIF" fragments at phys 4964, 84584, 84607,
  117621; region ids 1328-1331; reachability not claimed). L-019 verified: bank-10 Data region split
  (88856+32 / 88888+624 RackLayouts / 89512+5 string framing), 0 net bytes. L-021 left for commentary.
  L-017 (11 bytes) not retried (no new evidence). Details: analysis/investigation.md "Round 3".
- Gameplay: not exercised this pass (broad tour/ending/passwords/trick shot already covered in
  SC-014..SC-016; no unknown byte depends on an unvisited state).
- Toolkit: no new toolkit issues found this pass (TK-001..TK-004 unchanged; analysis/toolkit-issues.md).
- Status: **incomplete** (11 bytes unknown). Pass ended at easy-gain exhaustion. Handoff:
  analysis/handoffs/round-3-disassembly.md. Tools: traces/round-3/tools/.
