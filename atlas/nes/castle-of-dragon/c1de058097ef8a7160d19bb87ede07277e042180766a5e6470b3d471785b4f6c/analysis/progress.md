# Disassembly progress

Workspace initialized. Baseline and classification have not run (at coordinator start).

## Round 1 disassembly pass (2026-11-24)

Baseline created (`python3 run.py inspect`, `python3 run.py baseline`; maps/baseline.json, generated/baseline, verified byte-identical). Game: Castle of Dragon (SETA 1989, US), mapper 2, 8x16 KiB PRG, CHR-RAM.

ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).

Not complete: 1 byte ($C415, phys 0x1C425) remains unknown; 100.00% is rounded (99.9992%).

| Bank (phys) | Code | Image | Data/Text | Unknown |
|---|---|---|---|---|
| header 0x0000-0x000F | 0 | 0 | 16 | 0 |
| 0 (0x00010) | 0 | 16384 | 0 | 0 |
| 1 (0x04010) | 0 | 16384 | 0 | 0 |
| 2 (0x08010) | 0 | 16112 | 272 | 0 |
| 3 (0x0C010) | 0 | 16128 | 256 | 0 |
| 4 (0x10010) | 0 | 0 | 16384 | 0 |
| 5 (0x14010) | 0 | 0 | 16384 | 0 |
| 6 (0x18010) | 10490 | 0 | 5894 | 0 |
| 7 fixed (0x1C010) | 14241 | 0 | 2142 (incl. 32 Text) | 1 |

Code provenance: Observed 13122 bytes (opcode fetches at every instruction start), Static 11609 bytes (direct flow from observed code/vectors/task entries/bounded dispatch tables; includes 306 bytes of unreferenced fragments at confidence 0.75, lead L-002). Execution coverage (separate metric): 5709 unique observed ROM instruction addresses across scenarios S-A, S-B, S0-S7, T1, T2 (scenarios.json).

Evidence summary: tile graphics from LoadTileSet descriptor extents ($C87A/$C97C); palettes $BC00 bank 4; banks 4/5 generic data (data reads only, never executed); fixed/bank-6 tables tied to referencing instructions; details in analysis/investigation.md and map evidence strings. Durable artifacts: analysis/evidence/round-1-disassembly/ (tools, gzipped coverage/read traces, snapshots, frames).

Lead outcomes: no input leads. Produced L-001..L-006 (analysis/leads.json).

Verification: checkpoint pass-001b ok, byte_identical true (analysis/pass-001b-verification.json, map sha256 0ee481b5...). pass-001 (earlier map of the same pass, before splitting the two shared bank-2/3 pages out of Image) also verified and kept; superseded.

Toolkit: 3 findings (analysis/toolkit-issues.md): TI-001 missing feature (static discovery stops at every JSR on UxROM), TI-002 suspected bug (Read trace events include fetches), TI-003 missing feature (trace throughput). None blocking.

Pass ended normally: easy gains exhausted; remaining work is the 1-byte decision, review of static fragments, ending route and data semantics (redirect/commentary).

## Coordinator orchestration log

Coordinator: goal.md sequential phase workers (disassembly -> commentary -> redirect).
Handoffs live in analysis/handoffs/round-<N>-<phase>.md.
Disk note (2026-11-24 start): ~2.9 GB free on /, keep scratch small.

| Round | Phase | Status | Input checkpoint | Output checkpoint | Verification | Leads consumed/produced | Handoff |
|---|---|---|---|---|---|---|---|
| 1 | disassembly | done (easy gains exhausted) | baseline | pass-001b | byte-identical OK | produced L-001..L-006 | analysis/handoffs/round-1-disassembly.md |
| 1 | commentary | done (all classified source commented) | pass-001b | pass-001b (commentary/source) | byte-identical OK (commentary/verification.json) | informs L-001..L-006 | analysis/handoffs/round-1-commentary.md |
| 1 | redirect | done (attempt r1-a1; recommends cycle) | pass-001b + commentary | - | map unchanged | consumed L-001..L-006; ready L-002,L-003,L-004,L-006,L-007; L-001 blocked (rejection review), L-005 resolved | analysis/handoffs/round-1-redirect.md |
| 2 | disassembly | done (easy gains exhausted) | pass-001b | pass-002b | byte-identical OK | verified L-003,L-004,L-006,L-007; L-002 wording->commentary; L-001 blocked | analysis/handoffs/round-2-disassembly.md |
| 2 | commentary | done (all classified source commented) | pass-002b | pass-002b (commentary/source) | byte-identical OK (commentary/verification.json) | L-002 wording applied; redirect findings L-003/L-006/L-007 in comments | analysis/handoffs/round-2-commentary.md |
| 2 | redirect | done (attempt r2-a1; recommends cycle) | pass-002b + commentary | - | map unchanged | ready L-008 (provenance), L-009 (commentary); L-001 blocked (review re-confirmed incl. full branch scan) | analysis/handoffs/round-2-redirect.md |
| 3 | disassembly | done (easy gains exhausted) | pass-002b | pass-003 | byte-identical OK | verified L-008, L-010 (new, test-mode sub-options); L-009 left for commentary; L-001 blocked | analysis/handoffs/round-3-disassembly.md |
| 3 | commentary | done (all classified source commented) | pass-003 | pass-003 (commentary/source) | byte-identical OK (commentary/verification.json) | L-009 applied (SpawnLiftPlatform rename, stage 5/6 notes, S-PROG3 closed); L-010 noted | analysis/handoffs/round-3-commentary.md |
| 3 | redirect | done (attempt r3-a1; recommends terminal incomplete) | pass-003 + commentary | - | map unchanged | no ready leads; L-011 verified (static-code audit, ObjFieldD/F); L-001 blocked (rejection review r3); L-002 closed (no method) | analysis/handoffs/round-3-redirect.md |
Coordinator printed to user after round-1 disassembly: ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).

## Round 1 commentary pass (2026-11-24)

Complete. Prepared commentary/ from pass-001b and brought all classified source up to
the naming/comment standard: all bank-6 and bank-7 code, code-embedded tables, bank 0-6
data/image blocks, vectors/footer and an honest note for the unknown byte $C415.
236 routine/table labels, 1221 named branch labels, 257 symbols (commentary/source/symbols.inc).
review.json: 497/497 scaffold entries reviewed, 632 units; editorial audit pending (final acceptance).
Verification: byte-identical (commentary/verification.json); reading copy commentary/reading-after.asm.
ROM classification unchanged: 100.00% (131087 / 131088 bytes; 1 unknown).
Key findings for redirect: static ending route (armour + quest items, QuestItems bit 4 for
the ending), L-005 resolved (bank 6 DamagePlayer), bank 4/5 internal structure (map, block
sets, metasprite sizes, collision map, sound tables). Handoff: analysis/handoffs/round-1-commentary.md.

## Round 1 redirect pass (2026-11-24, attempt r1-a1)

Research only (analysis/redirect/r1-a1/); map/source unchanged. ROM classification unchanged:
100.00% (131087 / 131088; 1 unknown byte $C415).
- Ending reached in an artificial state (items/stage patched, one BossHitPoints patch): dragon
  defeat -> PlayEnding scenes 1-2, staff roll to THE END. Corrected the commentary patch ($053B
  must stay < $7F). 17 Static code regions gained fetches (artificial context).
- Test mode (natural route): armour via WEAPON $80 + EXIT; STAGE row edits MaxEnergy; 7 Static
  regions fetched.
- Bank 4/5 split proposals with extent proofs (block attr/def counts match; zero fills never read);
  bank 6 frame pointer table/frame grids/staff-roll frames end exactly at code $90D8 (new L-007).
- L-002 structural claims reviewed sound (wording fix only); L-001 stays unknown (rejection review
  in handoff); L-005 resolved.
Recommendation: (a) disassembly/commentary cycle with ready leads L-002, L-003, L-004, L-006, L-007.
Handoff: analysis/handoffs/round-1-redirect.md.

## Round 2 disassembly pass (2026-11-24)

Restored analysis/state.json (map.load); all prior classifications preserved. Checkpoint pass-002b: byte_identical true
(analysis/pass-002b-verification.json, map sha256 e30a89ef...). pass-002 = intermediate map of this pass (leads only, also
verified byte-identical), superseded by pass-002b.

ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).

Not complete: 1 byte ($C415, phys 0x1C425) remains unknown (L-001 blocked; 100.00% is rounded, 99.9992%).

| Bank (phys) | Code Observed | Code Static | Image | Data/Text | Unknown |
|---|---|---|---|---|---|
| header 0x0000-0x000F | 0 | 0 | 0 | 16 | 0 |
| 0 (0x00010) | 0 | 0 | 16384 | 0 | 0 |
| 1 (0x04010) | 0 | 0 | 16384 | 0 | 0 |
| 2 (0x08010) | 0 | 0 | 16112 | 272 | 0 |
| 3 (0x0C010) | 0 | 0 | 16128 | 256 | 0 |
| 4 (0x10010) | 0 | 0 | 0 | 16384 (12 typed regions + PaletteTable) | 0 |
| 5 (0x14010) | 0 | 0 | 0 | 16384 (9 typed regions) | 0 |
| 6 (0x18010) | 5527 | 4963 | 0 | 5894 | 0 |
| 7 fixed (0x1C010) | 12126 | 2115 | 0 | 2142 (incl. 32 Text) | 1 |

Code provenance: Observed 17653 bytes (was 13122), Static 7078 (was 11609). 643 regions (was 496).
Observed upgrades distinguish scenario type in each region's evidence: S-END (r1-a1 ending, artificial), S-TM
(r1-a1 test mode, natural route), S-WPN / S-PROG3 / S-PROG4 (round 2, artificial RAM assists). Partially fetched Static
regions were split at fetched/unfetched instruction boundaries; unfetched runs keep Static provenance.

Lead outcomes:
- L-003 verified: 17 Static regions from cov_ending upgraded/split (artificial-state execution; natural reachability not claimed).
- L-006 verified: 7 test-mode regions upgraded ($D01C split).
- L-004 verified: Bank4Data -> Bank4MapLayout, Bank4BlockAttrSet0-3, Bank4BlockDefSet0-3, Bank4ZeroFill_B381, MsprWidthTbl,
  MsprHeightTbl; Bank5Data -> Bank5CollisionMap, Bank5ZeroFill_8E40, SndLengthTbl, Bank5ZeroFill_9040, SndEnvPtrTbl,
  SndSeqPtrTbl, SndEnvelopeData, SndSequenceData, Bank5ZeroFill_BC68 (extent proofs re-checked in tools/apply_r2.py).
- L-007 verified: Bank6Data_8000 -> MsprFramePtrTbl $8000-$81FF + MsprFrameData $8200-$90D7 (staff frames not split off: frames
  $E6-$EF overlap them; old evidence error "LDA $9000,Y $CC5A" corrected - that reads bank-5 SndLengthTbl).
- L-002: map wording already neutral; $CCE6-$CD06 confidence 0.85; label renames remain for commentary.
- L-001: blocked, not retried (no new evidence).
New gameplay: S-WPN alternate weapons (+1920 Observed bytes), S-PROG3/S-PROG4 stage progression with RAM assists (+686/+231).
Stage 3-6 boss rooms deferred (scenarios.json S-BOSS3-6). Evidence: analysis/evidence/round-2-disassembly/.
Toolkit: no new findings; TI-001..TI-003 unchanged (analysis/toolkit-issues.md).
Pass ended normally (easy gains exhausted; diminishing returns in gameplay coverage).

Coordinator printed to user after round-2 disassembly: ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).

## Round 2 commentary pass (2026-11-24)

Complete. Baseline changed (pass-001b -> pass-002b): round-1 commentary preserved at
analysis/round-1/commentary/, commentary/ re-prepared from pass-002b (644 entries) and all
names/comments carried by address (byte-identical on first rebuild). Added notes for the new
typed bank-4/5/6 data regions and data labels used as operands, StaffRollFrames, ending scene 2,
L-002 *_Unreferenced renames, redirect findings (stage-7 setup rule, test-mode rows, weapon
cycling) and comments for remaining continuation entry points. 644/644 reviewed, 766 units;
editorial audit pending. Verification: byte-identical (commentary/verification.json); reading
copy commentary/reading-after.asm. ROM classification unchanged: 100.00% (131087 / 131088; 1
unknown byte $C415). Handoff: analysis/handoffs/round-2-commentary.md.

## Round 2 redirect pass (2026-11-24, attempt r2-a1)

Input pass-002b + round-2 commentary. No map/source edits. ROM classification unchanged: 100.00% (131087 / 131088; 1 unknown
byte $C415). Stage warp tool (SetupStage PC $E626 + ScrollXPage) and assisted play/boss controllers added
(analysis/redirect/r2-a1/tools). Visited: stage 3 and 4 boss rooms (natural door, defeated, stage clear -> map), stage 5/6
boss rooms by forced LoadStage entry (no natural entry: door test skips StageIndex 5/6; defeat never clears), 8 later
guardian pages + stage 6 end-page clear, stage 7 palace walk guardian $16 and natural door to the dragon, stage 2 type-$24
platform spawn, map route selection. S-PROG3 drop explained (pit death -> game over -> Reset). 62 Static code regions got
fetches (~3.76 KB, artificial). Ready: L-008 (provenance batch), L-009 (commentary corrections: $C5D4 is the type-$24
platform spawner, stage 5/6 boss content notes, MsprFrameData $8A1D note). L-001 rejection review re-affirmed with a new
branch/jump target scan. Toolkit: TI-004 (run_until PC throughput). Handoff: analysis/handoffs/round-2-redirect.md.

## Round 3 disassembly pass (2026-11-24)

Restored analysis/state.json (map.load); all classifications preserved (only Static->Observed provenance changes and splits of
partially fetched Static code runs). Checkpoint pass-003: byte_identical true (analysis/pass-003-verification.json, map sha256
fc23d9a5...). input/game.nes unchanged (sha256 c1de0580...).

ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).

Not complete: 1 byte ($C415, phys 0x1C425) remains unknown (L-001 blocked; 100.00% is rounded, 99.9992%).

| Bank (phys) | Code Observed | Code Static | Image | Data/Text | Unknown |
|---|---|---|---|---|---|
| header 0x0000-0x000F | 0 | 0 | 0 | 16 | 0 |
| 0 (0x00010) | 0 | 0 | 16384 | 0 | 0 |
| 1 (0x04010) | 0 | 0 | 16384 | 0 | 0 |
| 2 (0x08010) | 0 | 0 | 16112 | 272 | 0 |
| 3 (0x0C010) | 0 | 0 | 16128 | 256 | 0 |
| 4 (0x10010) | 0 | 0 | 0 | 16384 | 0 |
| 5 (0x14010) | 0 | 0 | 0 | 16384 | 0 |
| 6 (0x18010) | 8348 | 2142 | 0 | 5894 | 0 |
| 7 fixed (0x1C010) | 13614 | 627 | 0 | 2142 (incl. 32 Text) | 1 |

Code provenance: Observed 21962 bytes (was 17653), Static 2769 (was 7078); code total unchanged 24731. 751 regions (was 643).

Lead outcomes:
- L-008 verified: 13 round-2 redirect dumps merged per instruction start (tools/apply_l008.py); 112 fetched runs / 3760 bytes
  Static->Observed, partially fetched regions split at instruction boundaries. Evidence names the scenario ids; all are artificial
  contexts (RAM-patched warp/assists). 13 runs / 108 bytes fetched only in S-BOSS5F/S-BOSS6F (BossStage5/6 pieces $AE1F..$AEB7,
  BossAiStage5/6 pieces $B4C2..$B541, LoadStage $E1B6/$E1BC) are explicitly marked FORCED PC entry (registers.set $90D8->$E129),
  not natural reachability. Log: analysis/evidence/round-3-disassembly/l008_upgrades.json.
- L-010 (new) verified: test-mode SPTEST/BGTEST sub-options exercised through the natural cheat route (S-TMSUB, no patches;
  tools/tm_sub.py from round-1 snap_cheat, 2358 frames traced). 13 runs / 549 bytes Static->Observed (natural context):
  SpriteTest input handling $DDCF-$DE11 and sprite-$18 companion $DD9F-$DDB7, BgTest $DE64-$DEE3/$DEFD-$DF3A, SetupBgTest
  $DFA2-$E030, $E0DA-$E128, menu row 6 / Right-Left $DA9E/$DB3A/$DB76/$DB85, $DD5E, $C253, $C5BE. Log tmsub_upgrades.json.
  Observation: TestBgNum ($0515) stayed 0 through BGTEST Up/Down (title map shown); no new claim made from that.
- L-009: commentary-only, left for the commentary phase. L-002: unchanged (commentary wording). L-001: blocked, not retried
  (no new evidence; this pass's traces again do not fetch $C415).
Gameplay review: all stages, bosses 1-7 (5/6 forced), guardians, map select, ending and test mode are visited (scenarios.json).
Remaining Static code (2769 bytes) is scattered branch paths (guardian/boss AI variants, L-002 unreferenced fragments, IRQ $C0D8,
ending fade $D770 after endless yield); further play yields provenance only, at high trace cost (TI-003). Stopped as diminishing.
Evidence: analysis/evidence/round-3-disassembly/. Toolkit: no new findings; TI-001..TI-004 unchanged (analysis/toolkit-issues.md).
Pass ended normally (easy gains exhausted).

Coordinator printed to user after round-3 disassembly: ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).

## Round 3 commentary pass (2026-11-24)

Complete. Map changed (pass-002b -> pass-003), so round-2 commentary was preserved at
analysis/round-2/commentary/ and commentary/ re-prepared from pass-003 (752 entries).
Names/comments carried by physical address; static-only markers regenerated (2769 bytes).
L-009 applied: $C5D4 SpawnStage7BossBody -> SpawnLiftPlatform; stage 5/6 boss screens noted
as forced-entry only (boss door skips them, never cleared by boss defeat, stages finish on
last page); stage-clear rules rewritten; S-PROG3 closed (pit fall -> game over -> Reset);
MsprFrameData $8A1D-$8A40 reworded. L-010 test-mode sub-screens noted as observed.
752/752 entries reviewed, 854 units; editorial audit pending (final acceptance only).
Verification: byte-identical (commentary/verification.json); reading copy refreshed.
ROM classification unchanged 100.00% (131087/131088; $C415 unknown).
Handoff: analysis/handoffs/round-3-commentary.md.

## Round 3 redirect pass (2026-11-24, attempt r3-a1)

Static-only research on pass-003 (no map/source/ROM edits, no emulator runs). Tools/evidence: analysis/redirect/r3-a1/.
- Static-code audit: all 192 Static Code regions (2769 bytes) decode cleanly; 0 control-flow edges land off instruction
  starts; 184 regions / 2466 bytes reached from Observed code or dispatch tables; the 8 unreached (303 bytes) are exactly
  the L-002 fragments. No structurally doubtful code claim.
- New reference forms for $C415 and L-002 (immediate address pairs; all JMP-indirect/RTS-trick/task dispatch sources):
  none. ObjFieldD/F: write-only (6 absolute writes + whole-record copies), meaning unrecoverable.
- Leads: none ready; L-011 added (verified); L-001 blocked with round-3 rejection review; L-002 closed for redirect.
ROM classification unchanged: 100.00% (131087 / 131088; unknown $C415 / phys 0x1C425).
Recommendation: (c) terminal incomplete. Handoff: analysis/handoffs/round-3-redirect.md.

## Coordinator terminal handoff (2026-11-24)

Orchestration terminal state: justified terminal INCOMPLETE after rejection review (round-3
redirect r3-a1, inspected and accepted by coordinator). Not whole-ROM completion.
ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown). Unknown: $C415 / 0x1C425.
Final checkpoint pass-003 (byte-identical); commentary complete and byte-identical for pass-003;
final editorial audit pending for archive acceptance. archive-options.json written (14 documents
selected, 6 duplicated instruction copies omitted; no unrecognized Markdown). No archive run.
Final handoff: analysis/handoffs/final-coordinator.md.

## Final batch exception: user-directed manual Data fallback (2026-11-24)

Authority: explicit user instruction for this batch (exception applies only when <=10 physical bytes remain
unclassified; overrides further-structural-proof rules for those residual bytes only). Not an evidence-backed classification.
- Verified complement first: map.load analysis/state.json, analysis.unknown -> exactly 1 range
  {offset 115749 (0x1C425), length 1} = CPU $C415, fixed bank 7. Matches the user-validated list. <=10, so exception applies.
- Prior ambiguity (preserved, L-001; round-1/2/3 redirect rejection reviews): single unreferenced $60 between
  ClearSpritesAndObjects' RTS $C414 and InitFramePool $C416; orphan-RTS code vs stray data byte not discriminable from ROM evidence.
- Action: map.annotate {id 0 -> new region 1245, offset 115749, length 1, kind Data, provenance Manual, confidence 1.0,
  no label, evidence/comment stating USER-DIRECTED FALLBACK, meaning unknown}. map.validate ok; analysis.unknown total 0;
  map.save analysis/state.json; map.export maps/current.json; CLI map validate ok. Pre-fallback state copy:
  analysis/evidence/final-fallback/state-pass-003-before-fallback.json; log: analysis/evidence/final-fallback/apply-log.txt.
- Note: checkpoint pass-004 was created by mistake after a rejected first annotate call (lowercase enum); its map is
  identical to pass-003 (map sha256 fc23d9a5...) and it is retained only as a no-op record. Do not cite it as the fallback.
- Checkpoint pass-005: byte_identical true (analysis/pass-005-verification.json, map sha256 1c95fbac...).
  input/game.nes unchanged (sha256 c1de0580...).

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
Of these, 131087 bytes carry evidence-backed (Observed/Static) classification and 1 byte ($C415 / 0x1C425) is a
Manual user-directed generic Data fallback with unknown meaning (no proven data role).
Next: commentary phase (map changed pass-003 -> pass-005: preserve round-3 commentary under analysis/round-3/commentary/,
re-prepare, carry work), then final review and strict editorial audit.

## Round 4 commentary pass (2026-11-24)

Complete (phase worker). Map changed pass-003 -> pass-005 (only $C415 fallback region 1245), so round-3 commentary was
preserved at analysis/round-3/commentary/ (43 files, identical copy) and commentary/ re-prepared from pass-005; names/comments
carried by physical address; 751 entries keep round-3 credit. $C415 documented as Manual user-directed generic Data, meaning
unknown (source comment, review entry region-1245, uncertainties.md). Fixes: ObjTypeExpTbl/ObjTypeHpTbl renames, 14 branch
labels, several table notes. 752/752 entries, 855 units reviewed; editorial_audit pending (final acceptance next).
Verification: byte-identical (commentary/verification.json); reading copy refreshed. Handoff: analysis/handoffs/round-4-commentary.md.

## Final editorial audit and terminal handoff (2026-11-24)

Final acceptance worker: strict editorial review (fixes include PauseFlags -> BossScreenMode, LoadStage -> LoadBossScreen,
leftover raw addresses, wrong table names, data notes; details analysis/handoffs/final-editorial-audit.md,
commentary/editorial-audit.md). `commentary.py audit` passed; coordinator reran it independently: ok, byte_identical,
752 regions, 855 units, editorial_audit true. The review inventory equals the updated map (commentary/phase1-state.json ==
analysis/state.json; baseline.json == maps/current.json == maps/pass-005.json). store/map validate ok; generated/pass-005 verify
byte-identical; input/game.nes unchanged.
ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). Of these, 131087 are evidence-backed and 1 byte
($C415 / 0x1C425) is a Manual user-directed generic Data fallback with unknown meaning (final-manual-fallback.md).
archive-options.json updated (fallback record included). No archive run.
