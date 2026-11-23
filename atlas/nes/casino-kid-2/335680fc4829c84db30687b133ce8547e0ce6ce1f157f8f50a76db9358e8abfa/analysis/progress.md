# Disassembly progress

Workspace initialized 2026-11-23 (mapper 2 / UxROM, 128 KiB PRG, CHR RAM).

## Coordinator state
- Current: round 6 commentary STARTED on pass-006 (preserve round-5 commentary under analysis/round-5/). Then round-6 redirect check, final editorial acceptance + archive-options.json.
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
- Per bank (code / data / padding / unknown): b0 3/16090/291/0, b1 3/16142/239/0, b2 1546/14407/431/0, b3 4689/9425/2270/0, b4 10102/5625/657/0, b5 7124/8085/1175/0, b6 3023/11845/1516/0, b7 11436/4581/367/0, header 0/16/0/0.
- Regions 2,946 -> 2,636 (1,614 code, 961 data, 61 padding). 2,973 B Padding -> Data (inside graphics sets / nametable pictures); 0 code change. No open ready leads. Toolkit: no new issues.

## Coordinator state (snapshot at round 6 disassembly start; superseded by the top section)
- Current: round 6 disassembly STARTED on pass-005 (apply R5-L034..R5-L036: bank-2 scene blocks, padding->data inside graphics sets, mid-record cuts). Then commentary round 6, final editorial acceptance + archive-options.json.
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). (pass-005, verified byte-identical; unchanged by redirect)
- Findings: bank 2 $B6EF-$BB62 = ptr table + 8x122-B picture blocks + attr ptrs + 4 attr rows (+44 B unpointed); 65 Padding regions (2,973 B) lie inside LoadTileGraphics/LoadNametableFromBank extents; B2_TitleMetasprite, B6_InstrumentData, ChipPanelVramStructs, RouletteSpriteSets, DpcmSample1 cut by windows. Gameplay S01-S16 complete (S15 deferred). Toolkit: no new issues.

## Coordinator state (snapshot at round 5 redirect start; superseded by the top section)
- Current: round 5 redirect STARTED (r05-a1): bank-2 $B6EF-$BACE scene-block boundaries + pre-acceptance consistency check. Then final editorial acceptance + archive-options.json.
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). (pass-005, verified byte-identical)
- Per bank (code / data+padding / unknown): b0 3/16381/0, b1 3/16381/0, b2 1546/14838/0, b3 4689/11695/0, b4 10102/6282/0, b5 7124/9260/0, b6 3023/13361/0, b7 11436/4948/0, header 0/16/0.
- Commentary: 2,946/2,946 review entries reviewed, 1,872 units; handoff analysis/handoffs/round-05-commentary.md (remaining editorial issues listed for the final audit). Toolkit: no new issues.

## Coordinator state (snapshot before round 5 commentary; superseded by the top section)
- Current: round 5 commentary STARTED on pass-005 (preserve round-4 commentary under analysis/round-4/). Then round-5 redirect check, final editorial acceptance + archive-options.json.
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). (pass-005, verified byte-identical)
- Per bank (code / data+padding / unknown): b0 3/16381/0, b1 3/16381/0, b2 1546/14838/0, b3 4689/11695/0, b4 10102/6282/0, b5 7124/9260/0, b6 3023/13361/0, b7 11436/4948/0, header 0/16/0.
- R4-L030..R4-L033 verified and applied (0 B change; bank-3 $9291 boundary correction). No open ready leads. Toolkit: no new issues.

## Coordinator state (snapshot before round 5 disassembly; superseded by the top section)
- Current: round 5 disassembly STARTED on pass-004 (apply R4-L030..R4-L033: bank-3 $9270-$9974 structure, bank-2 $B323-$B6EE, bank-5 $BAEA-$BC21 text pointer tables, S16 observed provenance). Then commentary round 5, redirect, final editorial acceptance + archive-options.json.
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). (pass-004, verified byte-identical)
- Redirect r04-a1 findings: bank 3 $9270-$9974, bank 2 $B323-$B6EE and bank 5 $BAEA-$BC21 are exact pointer+string structures, but the commentary falsely says they have "no code reference"; 0 code/data conflicts; game over visited (S16, GameOutcome 2, 42 Static regions observed).
- Toolkit: no new issues.

## Coordinator state (snapshot before round 4 redirect; superseded by the top section)
- Current: round 4 redirect STARTED (r04-a1): boundary/role check (bank 3 $92B0/$92D0 string-table split, weak 0.75 data windows, low-conf code). Then final editorial acceptance + archive-options.json.
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). (pass-004, verified byte-identical)
- Per bank unknown: all 0. $BB30-$BB31 accepted as leftover code (R3-L027, Static, conf 0.5, reachability unresolved).
- Leads R3-L027..R3-L029 consumed in round 4; no open ready leads. Natural ending reached (S13).
- Toolkit: TK-006 added (minor); none blocking.

## Coordinator state (snapshot before round 3 disassembly; superseded by the top section)
- Current: round 3 disassembly STARTED (applying R2-L023..R2-L026 on pass-002). Then round 3 commentary, then redirect/final-acceptance decision.
- Checkpoint pass-002 (2 B unknown at bank 4 $BB30-$BB31, R2-L021 retained with rejection review). R2-L022 verified; natural ending reached (scenarios S13).
- Redirect evidence: analysis/redirect/r02-a1/, traces/redirect-r02/. Toolkit: no new issues.

## Round log

### Round 1
- disassembly: complete — analysis/handoffs/round-01-disassembly.md
- commentary: complete — analysis/handoffs/round-01-commentary.md
- redirect: complete — analysis/handoffs/round-01-redirect.md (12 ready leads R1-L009..L020, 11,783 B expected)

### Round 2
- disassembly: complete — analysis/handoffs/round-02-disassembly.md (pass-002, 2 B unknown)
- commentary: complete — analysis/handoffs/round-02-commentary.md (gate met, verify byte-identical)
- redirect: complete — analysis/handoffs/round-02-redirect.md (ready R2-L023..L026, 0 B; R2-L022 verified; natural ending S13)

### Round 3
- disassembly: complete — analysis/handoffs/round-03-disassembly.md (pass-003, 2 B unknown; R2-L023..L026 applied)
- commentary: complete — analysis/handoffs/round-03-commentary.md (gate met, verify byte-identical; round-2 commentary preserved at analysis/round-2/commentary/)
- redirect: complete — analysis/handoffs/round-03-redirect.md (ready R3-L027 $BB30 code proposal, R3-L028, R3-L029)

### Round 4
- disassembly: complete — analysis/handoffs/round-04-disassembly.md (pass-004, 0 B unknown)
- commentary: complete — analysis/handoffs/round-04-commentary.md (gate met, verify byte-identical; round-3 commentary preserved at analysis/round-3/commentary/)
- redirect: complete — analysis/handoffs/round-04-redirect.md (ready R4-L030..R4-L033, 0 B; game over S16)

### Round 5
- disassembly: complete — analysis/handoffs/round-05-disassembly.md (pass-005, 0 B unknown; R4-L030..L033 applied, role/boundary/provenance only)
- commentary: complete — analysis/handoffs/round-05-commentary.md (gate met, verify byte-identical; round-4 commentary preserved at analysis/round-4/commentary/)
- redirect: complete — analysis/handoffs/round-05-redirect.md (ready R5-L034..R5-L036, boundary/kind fixes, 0 B)

### Round 6
- disassembly: complete — analysis/handoffs/round-06-disassembly.md (pass-006, 0 B unknown; R5-L034..L036 applied, boundary/role only; 2,973 B Padding->Data)
- commentary: in progress

## Round 1 / disassembly

Date 2026-11-23. Toolkit binary /root/romatlas/target/release/romatlas (per workspace.json).
Baseline: maps/baseline.json + generated/baseline (raw, verified; not classification).
Checkpoint: **pass-001** — maps/pass-001.json (sha256 78c994c3...1dcdd), generated/pass-001,
analysis/pass-001-verification.json (ok, byte_identical true). Internal store analysis/state.json
== maps/current.json (map.validate ok).

ROM classification: 87.31% (114447 / 131088 bytes classified; 16641 bytes unknown).

Status: **incomplete** (16,641 bytes unknown).

Per-area counts (bytes; "PRG bank N" = 16 KiB bank N; data column includes padding):

| Area | Code | Data/padding | Unknown |
| header | 0 | 16 | 0 |
| PRG bank 0 (file 0x00010, CPU $8000) | 3 | 15229 | 1152 |
| PRG bank 1 (file 0x04010, CPU $8000) | 3 | 15309 | 1072 |
| PRG bank 2 (file 0x08010, CPU $8000) | 1545 | 13299 | 1540 |
| PRG bank 3 (file 0x0C010, CPU $8000) | 4689 | 10151 | 1544 |
| PRG bank 4 (file 0x10010, CPU $8000) | 9396 | 3960 | 3028 |
| PRG bank 5 (file 0x14010, CPU $8000) | 6924 | 5933 | 3527 |
| PRG bank 6 (file 0x18010, CPU $8000) | 3016 | 11413 | 1955 |
| PRG bank 7 (file 0x1C010, CPU $C000) | 11396 | 2165 | 2823 |

By provenance: Observed code 19,637; Static code 17,335; Static data 67,556; Static padding 9,919.
Regions: 2,460 (1,414 code, 920 data, 126 padding).

Execution coverage (separate metric): 9,635 unique ROM opcode addresses observed in traced
boot/title, story, world map, travel, blackjack (all 5 choices), roulette betting, password
screen and locked-poker map message. Banks 0/1 and $A000-$BFFF of bank 3 never executed.

Evidence summary: see analysis/investigation.md (method, bank-switch contract $E30E/$14,
inline-parameter callees, dispatch tables, rejection rules, data window rule, weaknesses).
Scenarios: analysis/scenarios.json. Leads: analysis/leads.json (R1-L001..L008 produced; none consumed).
Toolkit findings: analysis/toolkit-issues.md — 2 confirmed bugs (TK-001 NES input params ignored,
TK-002 discover budget param ignored), 2 missing features (TK-003 static discovery stops at JSR on
mapper 2, TK-004 tracing slowdown). None blocking.

## Round 1 / commentary

Date 2026-11-23. Prepared commentary/ from pass-001 (state == current.json). All 1,414 code regions
labelled and commented (descriptive routine names, generated Calls/RAM summaries, 402 RAM/hardware
symbols, inline parameters and 13 dispatch tables as .word symbols, data consumer notes, UNKNOWN
boundary notes). review.json 2,786/2,786 entries reviewed, 1,458 units; editorial audit pending.
Generator: analysis/tools/round01-commentary/build.sh (ann/ annotations). Verification:
commentary.py verify ok, byte_identical true (commentary/verification.json). Reading copy:
commentary/reading-after.asm. Classification unchanged (87.31%). Uncertainties:
commentary/uncertainties.md. Toolkit: no new issues.

## Round 1 / redirect

Date 2026-11-23. Attempt analysis/redirect/r01-a1 on checkpoint pass-001 (no map changes; 87.31%).
Ready leads R1-L009..R1-L020 (expected 11,783 of 16,641 unknown bytes; residual ledger 4,858 B).
R1-L001/L002/L003/L007 superseded, R1-L004/L005 verified, R1-L006/L008 updated. Gameplay: opponent
money RAM found; Australia (blackjack) and Brazil/Ailton (poker) defeated with money patches; all
locations + KING final unlocked via OpponentResultBits; final roulette reached (not won); credits via
artificial entry. Evidence traces/redirect-r01/. Handoff analysis/handoffs/round-01-redirect.md.

## Coordinator state (snapshot after round 2 disassembly; superseded by the top section)
- Current: round 2 disassembly complete; next = round 2 commentary on pass-002 (carry round-1 ann/ forward).
- Checkpoint pass-002 (100.00% rounded; 2 B unknown at bank 4 $BB30-$BB31). Open leads: R2-L021 (2 B), R2-L022 (ending route).
- Toolkit: TK-005 added (map.annotate id contract, minor).

## Round 2 / disassembly

Date 2026-11-23. Binary /root/romatlas/target/release/romatlas (workspace.json). Restored
analysis/state.json (pass-001); baseline untouched. Checkpoint **pass-002** — maps/pass-002.json
(sha256 a228e814...0d36), generated/pass-002, analysis/pass-002-verification.json (ok, byte_identical true).
map.validate ok; maps/current.json regions identical to analysis/state.json.

ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown).

Status: **incomplete** (2 bytes unknown: bank 4 $BB30-$BB31, lead R2-L021). Exact ratio 99.9985%.

| Area | Code | Data/padding | Unknown |
| header | 0 | 16 | 0 |
| PRG bank 0 (file 0x00010, CPU $8000) | 3 | 16381 | 0 |
| PRG bank 1 (file 0x04010, CPU $8000) | 3 | 16381 | 0 |
| PRG bank 2 (file 0x08010, CPU $8000) | 1546 | 14838 | 0 |
| PRG bank 3 (file 0x0C010, CPU $8000) | 4689 | 11695 | 0 |
| PRG bank 4 (file 0x10010, CPU $8000) | 10100 | 6282 | 2 |
| PRG bank 5 (file 0x14010, CPU $8000) | 7124 | 9260 | 0 |
| PRG bank 6 (file 0x18010, CPU $8000) | 3023 | 13361 | 0 |
| PRG bank 7 (file 0x1C010, CPU $C000) | 11436 | 4948 | 0 |

Regions 2,877 (1,469 code, 1,282 data, 126 padding); provenance Static 2,181 / Observed 696 regions.
Newly classified this pass: 16,639 previously unknown bytes. Leads R1-L009..L020 11,783 B (code 881 B incl.
122 B Observed; rest data); R2-D01 residual review 311 B new (71 B low-confidence Static code incl. 3 B
reclassified from data region 2247, now narrowed to $9108; 243 B consumer/pattern data); R2-D02 4,545 B generic data windows (Data-flanked holes with no
control-flow target, word-table/text/run signatures; conf 0.75). Per-annotation log:
analysis/tools/round02/applied-annotations.jsonl.gz; scripts analysis/tools/round02/.
Execution coverage unchanged except reuse of redirect traces (122 B newly Observed-labelled).
Gameplay: final roulette win attempt (S11, artificial) failed; $06D2 is not the result variable (R2-L022).
Toolkit: TK-005 (minor doc/API mismatch); TK-001..TK-004 unchanged; none blocking.

## Round 2 / commentary

Date 2026-11-23. Map changed (pass-001 -> pass-002): round-1 commentary preserved at
analysis/round-1/commentary/, fresh baseline prepared from pass-002. Names/comments carried by
physical range via analysis/tools/round02-commentary (gen.py + ann/), revalidated by a byte-identical
rebuild. All 1,469 code regions and all data named/explained (new poker selector/hold handlers,
B5_UpdateWheelAttributes, MainLoop_GameFinished, 7 low-confidence fragments, card blocks, VRAM-struct
lists, DPCM, gfx sets, pictures, sound tables); round-1 bank-4 mis-segmentation and several wrong
roles fixed; unknown $BB30-$BB31 explained. review.json 2,878/2,878 reviewed, 1,515 units; editorial
audit pending. Resumed per coordinator: 0 raw-address operands (120 mechanical bank-qualified table
labels with reader notes), 184 generic blocks with mechanical notes. Verification (fresh): commentary.py
verify ok, byte_identical true; reading copy refreshed. Handoff analysis/handoffs/round-02-commentary.md. Classification unchanged (2 B unknown).

## Coordinator state (snapshot after round 2 commentary; superseded by the top section)
- Current: round 2 commentary COMPLETE after coordinator-requested resume (no raw-address operands; 184 data blocks given mechanical notes; fresh verify byte-identical; reading copy refreshed). Next = round 2 redirect (not yet started).
- Checkpoint pass-002 (2 B unknown at bank 4 $BB30-$BB31). Open leads: R2-L021 (2 B), R2-L022 (ending route).
- Commentary pipeline: analysis/tools/round02-commentary/build.sh; round-1 copy analysis/round-1/commentary/.
- Toolkit: TK-005 added (map.annotate id contract, minor); no new issues in commentary.

## Round 2 / redirect

Date 2026-11-23. Attempt analysis/redirect/r02-a1 on checkpoint pass-002 (no map/ROM/source changes;
131,086 / 131,088 classified, unknown bank 4 $BB30-$BB31). Binary per workspace.json via a local
socket bridge with a scratch store copy. Findings: roulette winning number $0718 chosen at round init
(B5_OpponentBet via $A8F6 lists), settled at Start by B5_SettleMatchMoney via $9ED1 cell lists
(R2-L022 verified); natural ending reached by betting on the preset number and winning the
blackjack/poker stages with King money patches (FinalMatch_Ending $C1FD by fall-through); coverage
of 4 runs: 0 opcodes in data, 167 Static code regions observed; $9D debug menu confirmed; exact
extents for bank-5 roulette tables, bank-4 message tables/credits, bank-6 headers + 113 streams
($8DE7-$9F6A); three R1-L020 text windows corrected to number lists. Ready leads R2-L023..R2-L026
(0 B). R2-L021 unchanged (rejection review in handoff). Handoff analysis/handoffs/round-02-redirect.md.

## Round 3 / disassembly

Date 2026-11-23. Binary /root/romatlas/target/release/romatlas (workspace.json). Restored
analysis/state.json (pass-002); baseline untouched. Checkpoint **pass-003** — maps/pass-003.json
(sha256 4f2a310b...b149), generated/pass-003, analysis/pass-003-verification.json (ok, byte_identical true).
map.validate ok; analysis/state.json == maps/current.json (identical sha256).

ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown).

Status: **incomplete** (2 bytes unknown: bank 4 $BB30-$BB31, R2-L021). Exact ratio 99.9985%.

| Area | Code | of which Observed | Data/padding | Unknown |
| header | 0 | 0 | 16 | 0 |
| PRG bank 0 (file 0x00010, CPU $8000) | 3 | 0 | 16381 | 0 |
| PRG bank 1 (file 0x04010, CPU $8000) | 3 | 0 | 16381 | 0 |
| PRG bank 2 (file 0x08010, CPU $8000) | 1546 | 1468 | 14838 | 0 |
| PRG bank 3 (file 0x0C010, CPU $8000) | 4689 | 2644 | 11695 | 0 |
| PRG bank 4 (file 0x10010, CPU $8000) | 10100 | 4684 | 6282 | 2 |
| PRG bank 5 (file 0x14010, CPU $8000) | 7124 | 4300 | 9260 | 0 |
| PRG bank 6 (file 0x18010, CPU $8000) | 3023 | 2648 | 13361 | 0 |
| PRG bank 7 (file 0x1C010, CPU $C000) | 11436 | 8571 | 4948 | 0 |

No byte changed classification (per-byte kinds identical to pass-002). Regions 2,877 -> 2,976
(1,603 code, 1,247 data, 126 padding); Observed code 19,759 -> 24,315 B (artificial-scenario caveat).
Leads: R2-L023 (bank-5 roulette tables + new message ptr table/strings $AA75-$B634), R2-L024 (bank-4
poker strings/pointer tables — $A0C1 are hand-name records — and credits incl. new line pointer table
$B761), R2-L025 (bank-6 header ptrs, headers, 113 stream-entry regions), R2-L026 (104 regions Observed,
63 split) all verified. R2-L021 not retried. No gameplay needed. Evidence analysis/investigation.md
"Round 3"; scripts/logs analysis/tools/round03/. Toolkit: analysis/toolkit-issues.md (new TK-006 minor;
TK-001..TK-005 unchanged; none blocking).

## Round 3 / commentary

Date 2026-11-23. Map changed (pass-002 -> pass-003): round-2 commentary preserved at
analysis/round-2/commentary/ (diff identical), fresh baseline prepared from pass-003; names/comments
carried by CPU address with analysis/tools/round03-commentary/ (build.sh, snd.py, patches/).
Renames from new evidence: roulette (B5_PickWinningNumber, B5_SettleBets, chip-panel bet builder,
RltWinningNumber/RltWheelPos/RltBets...), poker hand names/showdown (B4_DrawHandName(s),
B4_CompareHands, B4_EvaluateHandRank...), ending staff roll (B4_EndingCredits, RunEndingCredits);
29 new tables named; bank-6 sound headers/streams rendered as labelled directives.
review.json 2,977/2,977 reviewed, 1,830 units. verify ok, byte_identical true (game.asm
30541c91...755c); reading copy refreshed. Classification unchanged (131,086/131,088).

## Round 3 / redirect

Date 2026-11-23. Attempt analysis/redirect/r03-a1 on checkpoint pass-003 (no map/ROM/source changes;
static only). R2-L021 reopened with combined constraints (gap census: every data gap has a reader or positive
non-code evidence; $7F = travel-scene state byte zpLocal3; executed redundant-load precedent bank5 $8883;
fragment-to-entry precedent) -> R3-L027 ready: $BB30-$BB31 as Static code (leftover lda zpLocal3, conf 0.5).
$9DB2 table is 4 bytes (index = card div 13, 0..3; result discarded) -> R3-L028. Bank-5 $8798-$87CB are
panel-label records reading B5_PanelLabelStrings -> R3-L029. B4_LuckyStrings: no reader (negative).
Handoff analysis/handoffs/round-03-redirect.md.

### Coordinator state
- Checkpoint pass-003; 131,086/131,088 classified; proposal R3-L027 would reach 131,088/131,088.
- Next: disassembly round 4 (R3-L027 independent review, R3-L028, R3-L029), then commentary; if fully
  classified, final editorial audit/acceptance. scenarios.json unchanged (ending reached, S13).

## Round 4 / disassembly

Date 2026-11-23. Binary /root/romatlas/target/release/romatlas (workspace.json). Restored
analysis/state.json (pass-003); baseline untouched; input/game.nes unchanged (sha256 335680fc...). Checkpoint
**pass-004** — maps/pass-004.json (sha256 eb7719e3...9519), generated/pass-004, analysis/pass-004-verification.json
(ok, byte_identical true). map.validate ok; analysis/state.json == maps/current.json == maps/pass-004.json.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Area | Code | of which Observed | Data/padding | Unknown |
| header | 0 | 0 | 16 | 0 |
| PRG bank 0 (file 0x00010, CPU $8000) | 3 | 0 | 16381 | 0 |
| PRG bank 1 (file 0x04010, CPU $8000) | 3 | 0 | 16381 | 0 |
| PRG bank 2 (file 0x08010, CPU $8000) | 1546 | 1468 | 14838 | 0 |
| PRG bank 3 (file 0x0C010, CPU $8000) | 4689 | 2644 | 11695 | 0 |
| PRG bank 4 (file 0x10010, CPU $8000) | 10102 | 4684 | 6282 | 0 |
| PRG bank 5 (file 0x14010, CPU $8000) | 7124 | 4300 | 9260 | 0 |
| PRG bank 6 (file 0x18010, CPU $8000) | 3023 | 2648 | 13361 | 0 |
| PRG bank 7 (file 0x1C010, CPU $C000) | 11436 | 8571 | 4948 | 0 |

Every byte now has a structural classification. The weakest claim is $BB30-$BB31: Static code at confidence 0.5,
with reachability unresolved. Classification is complete, but commentary and the editorial/acceptance gates
are still open. Regions 2,976 -> 2,977 (1,604 code, 1,247 data, 126 padding); Observed code is unchanged (24,315 B).
Leads: R3-L027 accepted after independent review (bank 4 $BB30-$BB31 = leftover `lda zpLocal3`, Static 0.5,
structure only). R3-L028 verified ($9DB2-$9DB5 merged into one 4-byte suit table). R3-L029 verified (bank 5
$8798 "YOU" string + $879C-$87CB 8x6 panel-label records; reader noted on $A8B5-$A8D5). No gameplay was needed,
because the ending was already reached in S13. Evidence: analysis/investigation.md "Round 4"; script/log
analysis/tools/round04/. Toolkit: analysis/toolkit-issues.md (no new toolkit issues found; TK-001..TK-006 unchanged).
Handoff analysis/handoffs/round-04-disassembly.md.

### Coordinator state
- Checkpoint pass-004; 131,088/131,088 classified (0 unknown).
- Next: **commentary round 4** (rebuild from pass-004, carry names by CPU address; regions changed are listed in
  the handoff). Then the final editorial audit/acceptance. scenarios.json unchanged (ending reached, S13).

## Round 4 / commentary

Date 2026-11-23. Map changed (pass-003 -> pass-004): round-3 commentary preserved at
analysis/round-3/commentary/ (diff identical), fresh baseline prepared from pass-004; carried by
CPU address with analysis/tools/round04-commentary/ (build.sh, patches/). Renames: $BB30
B4_UnreferencedStateLoad (reachability unresolved), B4_SuitOrderTable/B4_SuitOrderLookup,
B4_OpponentPickHolds, B5/B3_PrintPanelLabels and B5/B3_PrintPanelAmounts with their record
tables (two former generic blocks now named), B5_PanelLayouts, B3_OpponentNamePtrs/Names,
BjOpponentName; B4_CompareHighCards possible original bug documented. review.json 2,977/2,977
reviewed, 1,840 units. verify ok, byte_identical true (game.asm 6cb1e1ee...c387); reading copy
refreshed. Classification 131,088/131,088.


## Round 4 / redirect (r04-a1)
Checkpoint pass-004 (unchanged; 0 unknown). Static scans: immpair.py (adc#lo..lda#hi pairs), strtile.py, b5intro.py,
ctlcheck.py (4,293 edges, 0 conflicts). Runtime: game-over scenario S16 (artificial money/number patch), coverage
5,289 opcodes, 0 in non-code, 42 Static regions observed. Ready leads R4-L030 (b3 message block), R4-L031 (b2 attract
text lists), R4-L032 (b5 intro text blocks), R4-L033 (provenance). Handoff: analysis/handoffs/round-04-redirect.md.

## Round 5 / disassembly

Date 2026-11-23. Binary /root/romatlas/target/release/romatlas (workspace.json). Restored analysis/state.json (pass-004,
sha256 eb7719e3...9519); baseline untouched; input/game.nes unchanged (sha256 335680fc...). Checkpoint **pass-005** —
maps/pass-005.json (sha256 c79c8cdf...bc298), generated/pass-005, analysis/pass-005-verification.json (ok, byte_identical
true). map.validate ok; analysis/state.json == maps/current.json == maps/pass-005.json; no overlaps (local check).

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Area | Code | of which Observed | Data/padding | Unknown |
| header | 0 | 0 | 16 | 0 |
| PRG bank 0 (file 0x00010, CPU $8000) | 3 | 0 | 16381 | 0 |
| PRG bank 1 (file 0x04010, CPU $8000) | 3 | 0 | 16381 | 0 |
| PRG bank 2 (file 0x08010, CPU $8000) | 1546 | 1468 | 14838 | 0 |
| PRG bank 3 (file 0x0C010, CPU $8000) | 4689 | 3311 | 11695 | 0 |
| PRG bank 4 (file 0x10010, CPU $8000) | 10102 | 4684 | 6282 | 0 |
| PRG bank 5 (file 0x14010, CPU $8000) | 7124 | 4879 | 9260 | 0 |
| PRG bank 6 (file 0x18010, CPU $8000) | 3023 | 2678 | 13361 | 0 |
| PRG bank 7 (file 0x1C010, CPU $C000) | 11436 | 8651 | 4948 | 0 |

0 classification-byte change and 0 kind changes (byte-wise comparison with pass-004). Regions 2,977 -> 2,946 (1,614 code,
1,206 data, 126 padding). Observed code 24,315 -> 25,671 B (artificial S16 caveat). Leads: R4-L030 verified with a
boundary correction (opponent names end $9290; label/menu strings $9291-$92ED, "MONEY" first and unreferenced),
R4-L031, R4-L032 and R4-L033 verified as proposed. No gameplay this pass: the natural ending (S13) and game over (S16)
were already visited and no question needed new runtime evidence. Evidence: analysis/investigation.md "Round 5";
script/log/changes analysis/tools/round05/. Toolkit: analysis/toolkit-issues.md (no new toolkit issues found).
Status: classification complete; commentary and editorial/acceptance gates still open.

## Round 6 / disassembly

Date 2026-11-23. Binary /root/romatlas/target/release/romatlas (workspace.json). Restored analysis/state.json (pass-005,
sha256 c79c8cdf...bc298); baseline untouched; input/game.nes unchanged (sha256 335680fc...). Checkpoint **pass-006** -
maps/pass-006.json (sha256 dd3b5f7b...4a6c), generated/pass-006, analysis/pass-006-verification.json (ok, byte_identical
true). map.validate ok; analysis/state.json == maps/current.json == maps/pass-006.json; no overlaps (local check).

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Area | Code | of which Observed | Data | Padding | Unknown |
| header | 0 | 0 | 16 | 0 | 0 |
| PRG bank 0 (file 0x00010, CPU $8000) | 3 | 0 | 16090 | 291 | 0 |
| PRG bank 1 (file 0x04010, CPU $8000) | 3 | 0 | 16142 | 239 | 0 |
| PRG bank 2 (file 0x08010, CPU $8000) | 1546 | 1468 | 14407 | 431 | 0 |
| PRG bank 3 (file 0x0C010, CPU $8000) | 4689 | 3311 | 9425 | 2270 | 0 |
| PRG bank 4 (file 0x10010, CPU $8000) | 10102 | 4684 | 5625 | 657 | 0 |
| PRG bank 5 (file 0x14010, CPU $8000) | 7124 | 4879 | 8085 | 1175 | 0 |
| PRG bank 6 (file 0x18010, CPU $8000) | 3023 | 2678 | 11845 | 1516 | 0 |
| PRG bank 7 (file 0x1C010, CPU $C000) | 11436 | 8651 | 4581 | 367 | 0 |

0 classification-byte change; the only kind change is Padding -> Data for 2,973 B (byte-wise comparison with pass-005);
Code regions identical. Regions 2,946 -> 2,636 (1,614 code, 961 data, 61 padding). Leads R5-L034, R5-L035, R5-L036
verified and applied (R5-L035 with the optional one-region-per-graphics-set re-split; R5-L036 with a DPCM correction:
DMC record 5 $FD40-$FDF0 ends one byte past DpcmSample1, so $FC40-$FDF0 is one union region). No gameplay this pass (no
lead needed runtime evidence; S01-S16 already cover the milestones). Evidence: analysis/investigation.md "Round 6";
script/log/changes analysis/tools/round06/ (apply.py, applied.jsonl, changes.json, new-regions.txt, stats.py).
Toolkit: analysis/toolkit-issues.md (no new toolkit issues found).

### Coordinator state (after round 6 disassembly; resumed session)
- Checkpoint pass-006; 131,088/131,088 classified (0 unknown). Percentage printed to user: 100.00%.
- Round 6 commentary found substantially done in commentary/ (commentary/progress.md "Round 6", verify ok,
  game.asm e04a7ba6...) but with no analysis/handoffs/round-06-commentary.md and no progress entry ->
  resuming the **commentary round 6** phase (gate check + handoff), then redirect round 6.

## Round 6 / commentary

Date 2026-11-23. The map changed (pass-005 -> pass-006).
- Round-5 commentary was preserved at analysis/round-5/commentary/, and a fresh baseline was
  prepared from pass-006.
- Names were carried by CPU address with analysis/tools/round06-commentary/.
- The first worker was interrupted after the carry and review. A resumed session confirmed the
  source was current, ran the handoff gate and fixed the bank-3 blackjack gaps:
  - B3_ShuffleShoe;
  - B3_Decision* in menu order, B3_CheckCanDouble/Split/Surrender;
  - B3_Settle* and the payout rules;
  - B3_FlashBlackjackWin;
  - BJH_* hand-record fields, 9 new Bj RAM names and 9 named data tables.
- 24 of 25 over-long lines were fixed.

review.json: 2,636/2,636 reviewed, 1,900 units, editorial_audit pending. verify ok,
byte_identical true (game.asm 17c20b11...2d0c, symbols.asm 4833fb53...92). The reading copy is
refreshed. Classification is 131,088/131,088.

Handoff: analysis/handoffs/round-06-commentary.md. Next: redirect round 6.

### Coordinator state (after round 6 commentary)
- Commentary gate accepted: all 2,636 regions reviewed, fresh verify byte-identical, reading copy refreshed;
  remaining items (address-based RAM/table names, generic branch labels, numeric operands) are the documented
  final-acceptance editorial backlog. Next: **redirect round 6 (attempt r06-a1)**.

## Round 6 / redirect (r06-a1)

Date 2026-11-23. Checkpoint pass-006 (sha256 dd3b5f7b...4a6c), unchanged; 131,088/131,088 classified.

Status: proposal ready. Four ready leads, R6-L037..R6-L040, all role/boundary only (0 classification bytes):
- R6-L037: the bank-6 world-map data chain $A000-$BDE6. The commentary had misattributed its readers to bank 7. Read
  trace observed with context [12,13,14,15]. 944 B Padding->Data.
- R6-L038: sprite attribute lookup tables with independent extents. 286 B.
- R6-L039: the bank-5 message-window lists $844D-$86BC. 80 B.
- R6-L040: B6_PeriodTableHi is 65 entries. 28 B.

These reopen the r05-a1 "no reader" deferrals with readers that were found this round.

Research: analysis/redirect/r06-a1/. Handoff: analysis/handoffs/round-06-redirect.md. The scratch directory has been
removed and the server stopped. No toolkit issues.

### Coordinator state (after round 6 redirect)
- r06-a1 proposal ready: R6-L037..R6-L040 (Padding->Data 1,338 B in 45 regions + edge corrections; 0 classification
  change). Next: **round 7 disassembly** applying the batch, then commentary round 7, redirect round 7.

## Round 7 / disassembly

Date 2026-11-23. Setup:
- Binary: /root/romatlas/target/release/romatlas (from workspace.json).
- Restored analysis/state.json (pass-006, sha256 dd3b5f7b...4a6c). The baseline is untouched, and input/game.nes is
  unchanged (sha256 335680fc...).

Checkpoint **pass-007**:
- maps/pass-007.json (sha256 bea35cf5...be7f);
- generated/pass-007;
- analysis/pass-007-verification.json (ok, byte_identical true).

map.validate ok. analysis/state.json == maps/current.json == maps/pass-007.json, with no overlaps (local check).

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Area | Code | of which Observed | Data | Padding | Unknown |
|---|---|---|---|---|---|
| header | 0 | 0 | 16 | 0 | 0 |
| PRG bank 0 (file 0x00010, CPU $8000) | 3 | 0 | 16090 | 291 | 0 |
| PRG bank 1 (file 0x04010, CPU $8000) | 3 | 0 | 16142 | 239 | 0 |
| PRG bank 2 (file 0x08010, CPU $8000) | 1546 | 1468 | 14407 | 431 | 0 |
| PRG bank 3 (file 0x0C010, CPU $8000) | 4689 | 3311 | 9425 | 2270 | 0 |
| PRG bank 4 (file 0x10010, CPU $8000) | 10102 | 4684 | 5731 | 551 | 0 |
| PRG bank 5 (file 0x14010, CPU $8000) | 7124 | 4879 | 8263 | 997 | 0 |
| PRG bank 6 (file 0x18010, CPU $8000) | 3023 | 2678 | 12817 | 544 | 0 |
| PRG bank 7 (file 0x1C010, CPU $C000) | 11436 | 8651 | 4663 | 285 | 0 |

Changes against pass-006 (byte-wise comparison):
- 0 classification bytes changed. The only kind change is Padding -> Data for 1,338 B (43 Padding regions).
- Code regions are identical.
- Regions went from 2,636 to 2,534 (1,614 code, 902 data, 18 padding).

Leads R6-L037, R6-L038, R6-L039 and R6-L040 were each re-verified independently (operand scan over map Code regions plus
ROM assertions), then applied:
- R6-L037: bank-6 world-map chain, 30 regions; 944 B.
- R6-L038: five attribute tables, including $DFD8 at conf 0.8; 286 B.
- R6-L039: bank-5 message-window lists; 80 B.
- R6-L040: B6_PeriodTableHi, 65 entries; 28 B.

No gameplay this pass: no lead needed runtime evidence. No new leads.

References:
- Evidence: analysis/investigation.md, "Round 7".
- Scripts: analysis/tools/round07/ (refs.py, apply.py, applied.jsonl, changes.json, new-regions.txt).
- Handoff: analysis/handoffs/round-07-disassembly.md.
- Toolkit: analysis/toolkit-issues.md (no new toolkit issues found).

### Coordinator state (after round 7 disassembly)
- Checkpoint pass-007 (sha256 bea35cf5...be7f); 100.00% printed to user. R6-L037..L040 consumed/verified, no new leads.
  Next: **commentary round 7** (preserve round-6 commentary under analysis/round-6/, prepare from pass-007).

## Round 7 / commentary
Status: **complete** for the round-7 current-source readability gate (editorial_audit still pending, final acceptance only).
- Checkpoint pass-007 (sha256 bea35cf5...be7f) == maps/current.json == analysis/state.json == commentary/phase1-state.json.
- Round-6 commentary preserved in full at analysis/round-6/commentary/ (diff -r identical); fresh baseline prepared
  from pass-007 (2,534 regions, byte-identical). Pipeline: analysis/tools/round07-commentary/ (round07-ann.diff,
  round07-gen.diff record the edits).
- Carried: all pass-007 changed regions named and described (bank-6 world-map chain, five sprite attribute tables,
  bank-5 panel lists, B6_PeriodTableHi). Fixed world-map routines re-read (portrait swap, info sequencer, palette).
- Backlog: 45 mechanical data labels -> named structures, 25 message labels named after their text, generic-data notes
  110 -> 0, address-based RAM names 116 -> 59, several misread routines renamed (GAME OVER animation, poker AI,
  roulette wheel spin, sound envelope, board-tile copy).
- `commentary.py verify`: ok, byte_identical true; game.asm ad2beb8ad1596767e442fb878e78b33f8da6d644fe8f905104b51b1e711d6e63,
  symbols.asm a8a11730a33e8f35cf33a6e47a12d699b79f1f14d1bc1ac1c52aaa937ec33e6c. Reading copy refreshed.
- review.json 2,534/2,534 reviewed, 1,974 units. ROM classification unchanged: 131,088/131,088 (0 unknown).
- Handoff: analysis/handoffs/round-07-commentary.md. No new toolkit issues found.

### Coordinator state (after round 7 commentary)
- Commentary gate accepted (2,534/2,534 reviewed, verify byte-identical game.asm ad2beb8a..., reading copy refreshed;
  remaining backlog = final-acceptance editorial items). Next: **redirect round 7 (r07-a1)**.

## Round 7 / redirect (r07-a1)
Status: **complete, proposal ready** (not terminal). Checkpoint pass-007 (sha256 bea35cf5...be7f), 131,088/131,088
classified; map/state/ROM/source/toolkit unchanged.
- Method: artificial final-match poker runs from traces/redirect-r02/f_pk0.snap (PkrHandCount $0746=$18/$27, King
  money $075B patched) with range traces and coverage; static store scan for PkrHandFlag; map-vs-commentary evidence scan.
- Results: B4_ScriptedCards = royal flush in spades for the player at hands 25/40 (terminator dealt as ace without used
  check, duplicate observed); showdown ROYAL FLUSH vs HIGH CARD confirms $0756 player rank, $0755 opponent rank,
  $0754 1 = player; odd GameVar0726 = player hold flags; 53 Static bank-4 code regions (1,662 B) observed, 0 conflicts.
  PkrHandFlag 2/3 never stored by classified code. 52 Data regions carry stale generic map evidence.
- Leads: R7-L041 (ready, medium, provenance 1,662 B), R7-L042 (ready, low, metadata); R1-L006 closed (resolved).
- Gameplay: scenarios.json S17 added (visited, artificial).
- Evidence: analysis/redirect/r07-a1/, traces/redirect-r07/. Handoff: analysis/handoffs/round-07-redirect.md.
- Scratch /root/rascratch-r7r removed; own server stopped. No new toolkit issues found.

### Coordinator state (after round 7 redirect)
- r07-a1 proposal ready: R7-L041 (53 bank-4 regions Static->Observed, artificial S17), R7-L042 (52 stale map notes);
  R1-L006 closed. Next: **round 8 disassembly** applying R7-L041 (+R7-L042 notes), then commentary 8, redirect 8.

## Round 8 / disassembly
Status: **incomplete overall** (this pass is complete). Every byte is classified, but commentary refresh and final
editorial acceptance are still pending. One pass on pass-007, applying the ready leads R7-L041 and R7-L042.
- analysis/state.json was restored. The baseline was not re-created.
- input/game.nes is unchanged, and there were no toolkit edits.
- I used the configured binary through a temporary socket bridge. Scratch /root/rascratch-r8d has been removed and my
  server stopped.

Checkpoint **pass-008**:
- maps/pass-008.json (sha256 569ab31cb21db29e91ce4d6505dda53cccad47a07eaf286aeb0fd044bc57bbdb);
- generated/pass-008;
- analysis/pass-008-verification.json (ok, byte_identical true).

map.validate ok. analysis/state.json == maps/current.json == maps/pass-008.json. A local byte-wise check against
pass-007 found:
- no overlaps;
- an identical byte set for each of Code, Data and Padding;
- identical bank context;
- every coverage opcode lies in Observed code.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Area | Code | of which Observed | Data | Padding | Unknown |
|---|---|---|---|---|---|
| header | 0 | 0 | 16 | 0 | 0 |
| PRG bank 0 (file 0x00010, CPU $8000) | 3 | 0 | 16090 | 291 | 0 |
| PRG bank 1 (file 0x04010, CPU $8000) | 3 | 0 | 16142 | 239 | 0 |
| PRG bank 2 (file 0x08010, CPU $8000) | 1546 | 1468 | 14407 | 431 | 0 |
| PRG bank 3 (file 0x0C010, CPU $8000) | 4689 | 3311 | 9425 | 2270 | 0 |
| PRG bank 4 (file 0x10010, CPU $8000) | 10102 | 5852 | 5731 | 551 | 0 |
| PRG bank 5 (file 0x14010, CPU $8000) | 7124 | 4879 | 8263 | 997 | 0 |
| PRG bank 6 (file 0x18010, CPU $8000) | 3023 | 2678 | 12817 | 544 | 0 |
| PRG bank 7 (file 0x1C010, CPU $C000) | 11436 | 8651 | 4663 | 285 | 0 |

Regions went from 2,534 to 2,590 (1,670 code, 902 data, 18 padding). Observed code went from 25,671 to 26,839 B
(execution coverage is a separate metric from classification).

Leads:
- **R7-L041 verified.**
  - Coverage was re-checked: 4,431 opcodes, 0 in non-code, 0 misaligned.
  - 53 bank-4 Static regions (1,662 B) were hit. 38 were upgraded whole and 15 were split into Observed/Static runs
    (new ids 3374-3444).
  - Every upgraded run carries the S17 artificial-scenario caveat.
  - 2187 B4_ScriptedCards: evidence refreshed, conf 0.9.
- **R7-L042 verified.** The evidence text of 51 Data regions was refreshed from the round-7 commentary, plus a consumer
  scan. Confidence went to 0.85 for 40 of them; kinds and bounds are unchanged.

No gameplay this pass: no lead needed fresh runtime evidence. No new leads.

References:
- Evidence: analysis/investigation.md, "Round 8".
- Scripts: analysis/tools/round08/.
- Handoff: analysis/handoffs/round-08-disassembly.md.
- Toolkit: analysis/toolkit-issues.md (no new toolkit issues found).

### Coordinator state (after round 8 disassembly)
- Checkpoint pass-008 (sha256 569ab31c...7bbdb); 100.00% printed to user. R7-L041/L042 consumed, no new leads.
  Next: **commentary round 8** (preserve round-7 commentary under analysis/round-7/).

## Round 8 / commentary
Status: **complete** for the round-8 current-source readability gate (editorial_audit still pending, final acceptance only).
- Checkpoint pass-008 (sha256 569ab31c...bbdb) == maps/current.json == analysis/state.json == commentary/phase1-state.json.
- Round-7 commentary preserved in full at analysis/round-7/commentary/ (diff -r identical); fresh baseline prepared
  from pass-008 (2,590 regions, byte-identical). Pipeline: analysis/tools/round08-commentary/ (round08-ann.diff,
  round08-gen.diff, round08-snd.diff record the edits).
- Carried: regeneration matched round 7 except region headers (provenance + bank-4 splits). Redirect r07-a1 meanings
  folded in (scripted royal flush, confirmed hand-rank/winner orientation, PkrHoldFlags, PkrHandFlag 2/3 unused).
- Backlog: address-based RAM names 59 -> 9; sound engine command set/fade/vibrato decoded and routines renamed;
  roulette cursor/scroll code and blackjack bet entry re-read (misread names fixed, e.g. B6_Return ->
  B6_ChannelRegOffset, B5_WheelAnimation -> B5_BuildMarkerSprites, B3_ConfirmQuit -> B3_ConfirmBet).
- `commentary.py verify`: ok, byte_identical true; game.asm 3d312a13460c1757d0b56656fedc4ba1dd368dafc883e3eeb33232b36ae51c34,
  symbols.asm 8dfe7cac4129911aecf2e02db8f799353720c78987d53c3bdddb493cfbff16cd. Reading copy refreshed.
- review.json 2,590/2,590 reviewed, 2,042 units. ROM classification unchanged: 131,088/131,088 (0 unknown).
- Handoff: analysis/handoffs/round-08-commentary.md. No new toolkit issues found.

### Coordinator state (after round 8 commentary)
- Commentary gate accepted (2,590/2,590 reviewed, verify byte-identical game.asm 3d312a13..., reading copy refreshed).
  Next: **redirect round 8 (r08-a1)**.

## Round 8 / redirect (r08-a1)
Status: **proposal ready (metadata/editorial only); terminal stop recommended for structural research.**
- Input checkpoint pass-008 (sha256 569ab31c...7bbdb); map/state/ROM/source/toolkit unchanged; no server or scratch used.
- Sound streams re-walked with the corrected $FE length (4 bytes): identical to round 3 (113 streams, 28 headers), no
  $FE byte in $8DE7-$9F6A, 113 map regions match the stream starts. No boundary effect.
- R8-L043 (ready, low): 20 continuation Data segments (1,425 B) still carry generic evidence; extents ROM-checked;
  metadata refresh recipe in analysis/redirect/r08-a1/r8-l043-proposal.jsonl.
- R8-L044 (ready, low): 3 dangling names in commentary prose (lines 27111, 1732, 22645).
- Weak-region census: no new evidence; rejection review in analysis/handoffs/round-08-redirect.md.
- Research: analysis/redirect/r08-a1/ (findings.json, tools/). leads.json: pass appended, R8-L043/L044 added.

### Coordinator state (after round 8 redirect)
- Inspected r08-a1 rejection review (round-08-redirect.md): decisive artifacts cited (FE=4 re-walk identical, no $FE
  byte; weak-region census with no new caller/reader/trace hit; padding review r06 on unchanged premises). Scope and
  inferences consistent with evidence-policy; no corrected inference/test exposed -> accept terminal assessment for
  structural research. Ready metadata/prose leads R8-L043 (map notes) and R8-L044 (commentary names).
- Plan: round 9 disassembly (apply R8-L043, metadata only) -> round 9 commentary = final editorial cleanup +
  R8-L044 + `commentary.py audit` (final acceptance) -> archive-options.json -> terminal handoff. No further redirect
  unless round 9 produces new evidence (structure unchanged by design).

## Round 9 / disassembly
Status: **complete for this pass (metadata only).** Classification complete. No unknown bytes remain.
- Input pass-008 restored; output checkpoint **pass-009** (sha256 18763c39...95de1) == maps/current.json ==
  analysis/state.json; `run.py checkpoint pass-009` ok, byte_identical true; map validate ok.
- R8-L043 verified and applied: 20 continuation Data regions (1,425 B) got refreshed evidence after an independent ROM
  and consumer re-check (analysis/tools/round09/apply.py). Confidence went to 0.85 for 17 of them; 2833, 2434 and 2438
  are unchanged.
- Byte-wise vs pass-008: 0 kind, 0 provenance, 0 bank-context and 0 boundary change; only evidence/confidence of the
  20 ids differ (compare.py).
- R8-L044 is left for commentary. No gameplay, and scenarios.json is unchanged. No new toolkit issues found
  (analysis/toolkit-issues.md).
- Handoff: analysis/handoffs/round-09-disassembly.md.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Bank | Code | of which Observed | Data | Padding | Unknown |
|---|---|---|---|---|---|
| header | 0 | 0 | 16 | 0 | 0 |
| b0 | 3 | 0 | 16090 | 291 | 0 |
| b1 | 3 | 0 | 16142 | 239 | 0 |
| b2 | 1546 | 1468 | 14407 | 431 | 0 |
| b3 | 4689 | 3311 | 9425 | 2270 | 0 |
| b4 | 10102 | 5852 | 5731 | 551 | 0 |
| b5 | 7124 | 4879 | 8263 | 997 | 0 |
| b6 | 3023 | 2678 | 12817 | 544 | 0 |
| b7 | 11436 | 8651 | 4663 | 285 | 0 |

### Coordinator state (after round 9 disassembly)
- Checkpoint pass-009 (sha256 18763c39...95de1); 100.00% printed to user. R8-L043 consumed; metadata only.
  Next: **commentary round 9 = final editorial acceptance** (R8-L044, backlog, commentary.py audit).

## Round 9 / commentary (final acceptance)
Status: **complete**. Final editorial acceptance passed: `commentary.py audit` ok.
- Checkpoint pass-009 (sha256 18763c39...95de1) == maps/current.json == analysis/state.json == commentary/phase1-state.json.
- Round-8 commentary preserved in full at analysis/round-8/commentary/ (diff -r identical). A fresh baseline was
  prepared from pass-009 (2,590 regions, byte-identical).
- Pipeline: analysis/tools/round09-commentary/ (round09-ann.diff). The first regeneration was identical to round 8,
  so all review credit carried.
- R8-L044 applied and marked verified in leads.json (pass record appended). danglingnames.py now finds only history
  notes.
- Editorial:
  - 6 of the 7 remaining address-based RAM names were named from code; PwdVar057B is kept.
  - Misread bank-5 "highlight"/"ball" routines were renamed. They are the marker cover offset and the post-spin
    board scroll.
  - The password Start/Select handler names did not match their behaviour and were renamed.
  - Terse headers were rewritten (blackjack compare/split, B5_MatchEndDialogue, password intro).
  - uncertainties.md: superseded cross-references and a Round 9 section.
- Audit: byte_identical true, 2,590/2,590 regions and 2,042 units reviewed, editorial_audit true.
  - game.asm 7efbc79749707da4d3a76e181d12987a015d8b7b98f77ccc3adf32a496239651
  - symbols.asm 7db4abb2c2f6cecb41d746327e5093678dca511c709575050644a0ae548c5def
  - The reading copy was refreshed.
- ROM classification unchanged: 131,088/131,088 (0 unknown). No toolkit issues. No server started.
- Handoff: analysis/handoffs/round-09-commentary.md.

### Coordinator state (terminal, after round 9 commentary / final acceptance)
- commentary.py audit ok, byte_identical, 2,590/2,590 regions + 2,042 units reviewed, editorial_audit reviewed.
- No ready leads; r08-a1 rejection review reused on unchanged premises (round 9 metadata/prose only) -> no redirect 9.
- Terminal handoff: analysis/handoffs/final-handoff.md. ROM classification 100.00% (131088/131088; 0 unknown).
- archive-options.json written; archive/cleanup not executed (not authorized).
