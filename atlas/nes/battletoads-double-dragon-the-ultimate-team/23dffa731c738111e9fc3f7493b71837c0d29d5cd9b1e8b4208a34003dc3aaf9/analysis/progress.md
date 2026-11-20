# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Coordinator state

- Round: 1
- Phase log:
  - R1 disassembly pass 1: DONE. Input: baseline. Output checkpoint: pass-001 (ok, byte_identical).
    ROM classification: 18.50% (48496 / 262160 bytes classified; 213664 bytes unknown).
    Leads produced L-001..L-009; consumed none. Handoff: analysis/handoffs/round-01-disassembly.md
  - R1 commentary: DONE 2026-11-19 on pass-001 (COMPLETE readability; commentary/verification.json ok+byte_identical; reading copy commentary/reading-after.asm; handoff analysis/handoffs/round-01-commentary.md)
  - R1 redirect: DONE — READY-FOR-DISASSEMBLY. Attempt analysis/redirect/r01-a1/. Ready leads L-010..L-019
    (~10.3 KB+2.5 KB code, ~190 KB data proposals). Superseded L-001/L-002/L-005; partial L-003/L-007.
    Ending reached via RAM $DA=$FF stage-end chain. Handoff: analysis/handoffs/round-01-redirect.md
- Round: 2
  - R2 disassembly pass 2: DONE 2026-11-19. Output checkpoint pass-002 (ok, byte_identical).
    ROM classification: 99.35% (260469 / 262160 bytes classified; 1691 bytes unknown).
    Consumed L-010..L-019 (all verified); produced L-020..L-022. Handoff analysis/handoffs/round-02-disassembly.md
  - R2 commentary: DONE — COMPLETE readability on pass-002 (round-1 commentary preserved at analysis/round-1/commentary/;
    commentary/verification.json ok+byte_identical; reading copy commentary/reading-after.asm). Handoff analysis/handoffs/round-02-commentary.md
  - R2 redirect: DONE — READY-FOR-DISASSEMBLY. Attempt analysis/redirect/r02-a1/. Ready leads L-023 (300 B),
    L-024 (312 B), L-025 (668 B), L-026 (155 B), L-006 (236 B), L-004 (6 B); residual L-027 14 B blocked.
    No credits roll after ending; 2P Game A visited. Handoff analysis/handoffs/round-02-redirect.md
- Round: 3
  - R3 disassembly pass 3: DONE. Output checkpoint pass-003 (ok, byte_identical).
    ROM classification: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).
    L-023/L-024/L-025/L-006/L-004 verified; L-026 verified 153 B (2 B moved to L-027); L-027 blocked, 16 B.
    Handoff analysis/handoffs/round-03-disassembly.md
  - R3 commentary: DONE — COMPLETE readability on pass-003 (round-2 commentary preserved at analysis/round-2/commentary/;
    verification ok+byte_identical; reading copy commentary/reading-after.asm). Handoff analysis/handoffs/round-03-commentary.md
  - R3 redirect: DONE — READY-FOR-DISASSEMBLY. Attempt analysis/redirect/r03-a1/. L-028 (11 B: b6 $FE24-$FE2B code,
    b6 $FE2C data judgement, b0 $E536-$E537 code judgement); L-027 still blocked for 5 lone $60 bytes (per-byte
    rejection review in handoff). 2P Game B visited. Handoff analysis/handoffs/round-03-redirect.md
- Round: 4
  - R4 disassembly pass 4: DONE. Output checkpoint pass-004 (ok, byte_identical).
    ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown) - rounded; incomplete.
    L-028 verified (11 B); L-027 still blocked (5 B). Handoff analysis/handoffs/round-04-disassembly.md
  - R4 commentary: DONE — COMPLETE readability on pass-004 (round-3 commentary preserved at analysis/round-3/commentary/;
    verification ok+byte_identical; reading copy refreshed). Handoff analysis/handoffs/round-04-commentary.md
  - Coordinator rejection-review check (L-027, round-03-redirect.md): review evaluates each $60 byte in isolation;
    it did not test the combined constraint "orphan bytes after terminal instructions in this ROM's code are
    always $60 and no other inter-routine padding exists", which would discriminate dead-rts vs pad.
    Returned to redirect as concrete question Q-R4-1.
  - R4 redirect: DONE — TERMINAL (incomplete). Q-R4-1 tested (analysis/redirect/r04-a1/): premises false
    ($FF is fill incl. lone $FF after rts at b5 $8093; orphans not uniformly $60; no dead-rts habit). No ready leads.
    Updated rejection review: analysis/handoffs/round-04-redirect.md
  - Coordinator terminal review 2026-11-20: inspected the updated rejection review and spot-checked primary ROM
    bytes for the five $60 sites and b5 $8093; reasoning sound under evidence-policy.md (no reference/consumer,
    competing remnant-byte class is real in this ROM). Accepted TERMINAL INCOMPLETE; no new corrected inference.
  - archive-options.json written (documents/omitted_documents; all analysis/commentary Markdown accounted for).
    No archive executed. Final handoff: analysis/handoffs/final-handoff.md
- Current phase: TERMINAL HANDOFF (workflow terminated; ROM classification NOT complete)
- Final: ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown) [rounded; 99.998%]
- Final editorial audit: not run (reserved for final acceptance/archive)
- Latest verified checkpoint: pass-004

## Blocker record (2026-11-15, coordinator session)

Status: BLOCKED (environment), resumable.

Every Bash command in the coordinator session fails before execution with:
`apply-seccomp: write /proc/self/setgroups (nested userns is capability-restricted;
caller must provide CAP_SYS_ADMIN): Permission denied` (reproduced on `echo`, `df`
and the no-op `true`). The sandbox override is disabled for this session, so no
process can be started. As a result `python3 run.py inspect|baseline|serve|checkpoint`,
the configured romatlas release binary, and the emulator are all unavailable.
Phase workers started with the Agent tool would run under the same sandbox, so no
phase was launched. A worker that cannot run commands would only produce empty
handoffs or analysis it cannot check.

Nothing in the workspace was modified except this file. input/game.nes is untouched.

Resume recipe: once shell execution works (sandbox can create user namespaces, or
it runs unsandboxed), start the coordinator from goal.md at round 1 → phase
prompts/disassembly.txt (run `python3 run.py inspect` then `python3 run.py baseline`).
archive-options.json has not been written yet because no final analysis documents
exist yet. It is due at the terminal handoff.

## Blocker resolved (2026-11-19, coordinator session 2)

Shell execution works again (echo/df/ls succeed; release binary present). The
2026-11-15 sandbox blocker is historical. Round 1 disassembly worker launched.

## Round 1 / disassembly pass 1 (2026-11-19)

- Baseline: created (maps/baseline.json, generated/baseline, verified byte-identical).
- Checkpoint: pass-001 (maps/pass-001.json, generated/pass-001,
  analysis/pass-001-verification.json). Verification: ok=true, byte_identical=true,
  map sha256 d6766e2b6bf92c974b82083899e4107c28f2a51190e087c7bb092f3eeafa91e9.
- ROM classification: 18.50% (48496 / 262160 bytes classified; 213664 bytes unknown).
- Code 47,443 bytes (Observed-provenance regions 23,651; Static 23,792); Data 1,053 bytes.
- Per bank (classified code / data / unknown):
  - header (16 B): data 16
  - bank 0 file $00010-$0800F: 19843 / 92 / 12833
  - bank 1 file $08010-$1000F: 792 / 38 / 31938
  - bank 2 file $10010-$1800F: 1651 / 30 / 31087
  - bank 3 file $18010-$2000F: 3414 / 262 / 29092
  - bank 4 file $20010-$2800F: 3560 / 30 / 29178
  - bank 5 file $28010-$3000F: 6559 / 334 / 25875
  - bank 6 file $30010-$3800F: 10905 / 213 / 21650
  - bank 7 file $38010-$4000F: 719 / 38 / 32011
- Evidence: trace coverage (traces/round01/coverage-contexts.json) + local static
  recursive descent and bounded dispatch tables (analysis/investigation.md,
  analysis/tools/round01/). Execution coverage is separate from classification.
- Leads consumed: none (empty at start). Produced: L-001..L-009.
- Scenarios visited: power-on, title, mode menu, character select, intro, stage 1
  ($10=$03, natural), stages $04 $08 $02 $06 $05 $09 via RAM patch of $0010
  (artificial), pause. Attempted: $01 (crash), $07 (black), $0A-$0D (garbage).
  Deferred: ending/credits, game over, 2-player. See analysis/scenarios.json.
- Toolkit: 1 confirmed bug (TK-001 emulator.input params), 2 missing features
  (TK-002 static discovery stops at JSR/indexed stores; TK-003 trace slowdown);
  no blocker. analysis/toolkit-issues.md.
- Status: incomplete. Handoff: analysis/handoffs/round-01-disassembly.md.

## Round 1 / commentary (2026-11-19)

- Prepared commentary/ from pass-001 (baseline verified byte-identical). Source is
  built from commentary/before + commentary/notes by commentary/tools/annot.py.
- Result: COMPLETE current-source readability: 696 routines named, 624 block-commented,
  RAM/object/sound symbols, notes on all 260 unknown gaps; review.json 1532/1532
  entries reviewed (624 units); editorial audit pending (final acceptance only).
- Verification: commentary verify ok=true, byte_identical=true; reading copy
  commentary/reading-after.asm refreshed.
- Classification unchanged (18.50%). Redirect questions (ObjTypeHandlers $9A71 with ~42
  unclassified handler targets, animation/sound/metasprite tables, decompressor sets):
  analysis/handoffs/round-01-commentary.md.
- Toolkit: no new toolkit issues found in this phase.

## Round 1 / redirect (2026-11-19, attempt r01-a1)

- Input: pass-001 + COMPLETE commentary. Research only (no map/state/source edits).
- Result: READY-FOR-DISASSEMBLY. Ready leads L-010..L-019 (≈12.9 KB code + ≈190 KB data
  proposed); L-001/L-002/L-005 superseded, L-003/L-007 partially resolved.
- Gameplay: native stage-end trigger $DA=$FF; all 7 levels loaded in order and the ending
  screen ($0B) reached (artificial trigger, natural transitions).
- Handoff: analysis/handoffs/round-01-redirect.md; evidence analysis/redirect/r01-a1/.

## Round 2 / disassembly pass 2 (2026-11-19)

- Input: pass-001 (analysis/state.json restored with map.load; all existing regions preserved).
- Checkpoint: pass-002 (maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json).
  Verification: ok=true, byte_identical=true, map sha256
  9c36a8338d10761572a773cd94fab29905e6b32afb12d3ffe385e31655079852. state.json regions == maps/current.json.
- ROM classification: 99.35% (260469 / 262160 bytes classified; 1691 bytes unknown).
- Code 60,714 bytes (+13,271 Static); Data 199,755 bytes (+198,702). 281 new regions via map.annotate.
- Per bank (classified code / data / unknown):
  - header (16 B): data 16
  - bank 0: 30278 / 2179 / 311
  - bank 1: 804 / 31929 / 35
  - bank 2: 1773 / 30794 / 201
  - bank 3: 3416 / 29322 / 30
  - bank 4: 3560 / 29105 / 103
  - bank 5: 6770 / 25529 / 469
  - bank 6: 13382 / 18884 / 502
  - bank 7: 731 / 31997 / 40
- Leads consumed: L-010 verified (+10,278 code, tables $9A71-$9C3B); L-011 verified; L-012 verified;
  L-013 verified; L-014 verified (+2,529 code); L-015..L-018 verified (data); L-019 verified
  (5 fragments as Static code, reachability unresolved). L-003 note updated. Produced L-020
  (bank 6 record-like residuals), L-021 (ambiguous code/table fragments), L-022 (all other residuals).
- Additional: ~95 consumer-referenced residual tables (all banks), 7 $FF fills, bank 2 $F140-$F3DF
  tile-like data, 12 uncalled code fragments (analysis/investigation.md round 2).
- Scenario: game over (artificial: $0011=0, $051B=0 from snap_lvset) -> death, lives $FF, game-over
  state, attract/title. 5,965 traced fetches all on classified code (none in Data); 59 on pass-002
  static code. 2P / credits deferred.
- Tools: analysis/tools/round02/ (codeplan, datacheck, residual, gapprop, tailcheck, sweep, plan,
  apply, gameover). Unknown runs: scratch/r02_unknown.txt.
- Toolkit: no new toolkit issues found (TK-001..TK-003 unchanged; analysis/toolkit-issues.md).
- Status: incomplete (1,691 unknown bytes). Handoff: analysis/handoffs/round-02-disassembly.md.

## Round 2 / commentary (2026-11-19)

- Map changed (pass-001 -> pass-002): round-1 commentary preserved intact at
  analysis/round-1/commentary/; commentary/ freshly prepared from pass-002 (baseline
  byte-identical); round-1 notes carried by physical identity and reverified.
- Result: COMPLETE current-source readability: 977 routines named (901 block-commented,
  all >= 4 instructions), all 92 ObjTypeHandlers and 41 BgObjHandlers targets named,
  notes on all 186 data regions and 99 unknown runs; review.json 1649/1649 reviewed
  (901 units); editorial audit pending (final acceptance only).
- Corrections: StageId $08 = level 5; round-1 ScriptedEnemyA/B_Update, bank 6
  BuildColumnData/NextColumnIndex/ScrollLayerA/B renamed per mechanics.
- Verification: commentary verify ok=true, byte_identical=true; reading copy
  commentary/reading-after.asm refreshed.
- Classification unchanged (99.35%). Redirect: bank 6 L-020 residuals identified as
  screen-actor motion scripts (~270 B); other small table-tail questions.
  Handoff: analysis/handoffs/round-02-commentary.md.
- Toolkit: no new toolkit issues found in this phase.

## Round 2 / redirect (2026-11-20, attempt r02-a1)

- Input: pass-002 (99.35%, 1,691 unknown in 99 runs). Map/state/source/ROM unchanged.
- Result: READY-FOR-DISASSEMBLY. Proposed 1,677 bytes: L-023 motion scripts 300, L-024 proven
  tables 312, L-025 generic data 668, L-026 code fragments 155 (tier B 70 = reviewer call),
  L-006 bank-switch module copies 236, L-004 jmp slots 6. Residual 14 bytes (L-027 blocked).
- Superseded L-008, L-020, L-021, L-022; L-009 verified (2P Game A visited).
- Gameplay: post-ending exit -> title (no credits roll); 2P Game A; scenarios.json updated;
  snapshot traces/r02a1_snap_2p_gameA.json.
- Evidence: analysis/redirect/r02-a1/ (notes.md, proposals.json). Handoff:
  analysis/handoffs/round-02-redirect.md. Server shut down. Toolkit: no new issues.

## Round 3 / disassembly pass 3 (2026-11-20)

- Input: pass-002 (analysis/state.json restored with map.load, 1,550 regions preserved; baseline
  already present). Ready batch from analysis/handoffs/round-02-redirect.md /
  analysis/redirect/r02-a1/proposals.json independently re-checked before map.annotate.
- Checkpoint: pass-003 (maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json).
  Verification: ok=true, byte_identical=true, map sha256
  92863db235ffffa6db86f231482ea339e26745cb06f5d39c3628dfa318452733. state.json regions (1,653) ==
  maps/current.json == maps/pass-003.json.
- ROM classification: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).
- Code 61,109 bytes (+395 Static); Data 201,035 bytes (+1,280 Static). 103 new regions via map.annotate.
- Per bank (classified code / data / unknown):
  - header (16 B): data 16
  - bank 0: 30378 / 2387 / 3
  - bank 1: 839 / 31929 / 0
  - bank 2: 1812 / 30956 / 0
  - bank 3: 3441 / 29327 / 0
  - bank 4: 3620 / 29147 / 1
  - bank 5: 6842 / 25925 / 1
  - bank 6: 13411 / 19346 / 11
  - bank 7: 766 / 32002 / 0
- Verification method (analysis/tools/round03/): verify.py re-decodes every classified code region
  and lists, per proposal, the preceding/following instruction, bytes, and every abs/abs,x/abs,y
  operand from classified code that can reach the range; dis6502.py for local listings;
  immptr.py re-finds the StepBgObjScript pointer immediates (all 18 scripts incl. $F32E at $F25E);
  module-copy byte diff for L-006; codecheck.py (r02-a1) rerun: all fragments aligned, targets on
  classified instruction starts. apply.py is the exact annotation batch.
- Lead outcomes:
  - L-023 verified: 300 B Data (291 B motion scripts labelled MotionScript_B6_xxxx; $EFA6-$EFAE
    generic data: unofficial NOP/BRK run, no pointer).
  - L-024 verified: 312 B Data (consumer loops and index bounds re-read; labels ObjDrawHookTable,
    AsciiToTileTable, StageNextTables_B6, Tables_A2DC, ThresholdSearchTables_E86A,
    OffsetTable_B4_B933, RamRoutineTemplate_B4, PlayerTargetX_B2, RequestRecords_84F0).
  - L-025 verified: 668 B generic Data (tier B single bytes accepted: each $FF/$92/$22 would
    overlap the following classified instruction or is an illegal/KIL opcode; b6 $8C13 8B 8E = XAA).
  - L-026 verified in part: 153 B Static code (reachability unresolved). Rejected for now:
    b0 $E536-$E537 (lda #$02 before called entry $E538) because the frame maps
    PlayerFrameMapA/B ($E4EB,y/$E4F5,y with Y < $5B) can index into it; moved to L-027.
  - L-006 verified: 236 B Static code (module copies; jsr/jmp operands into fill not followed).
  - L-004 verified: 6 B Static code (jmp slots b3 $8024, b6 $800C; targets not followed).
  - L-027 re-examined, still blocked (now 16 B incl. $E536-$E537).
- Remaining unknown ranges (16 B):
  - b0 $8D92 (1, $60): lone byte after beq/bne pair; dead rts vs unread pad, no reference.
  - b0 $E536-$E537 (2, A9 02): code prefix to $E538 vs tail of PlayerFrameMapA/B (index domain unproven).
  - b4 $B979 (1, $60): after jmp $B987, before RamRoutineTemplate_B4; no reference.
  - b5 $ACFA (1, $60): after jmp $AFB9; no reference.
  - b6 $9745 (1, $60): index 6 of StageNextTables (never read: cpy #$07); dead rts vs table byte.
  - b6 $EFE5 (1, $60): after always-taken bne $F00C, before MotionScript_B6_EFE6.
  - b6 $FE24-$FE2C (9): jsr $FE19/stx OAMADDR/lda #$02/$8D whose operand overlaps classified ldx #$00 at $FE2D.
- Gameplay: bounded 2P Game A replay (traces/r02a1_snap_2p_gameA.json, 1,200 frames, Read+Instruction
  detail filtered to phys $064FB-$06557): no access to PlayerFrameMapA/B or $E536 (the latch-on-player
  path TryLatchOntoPlayer -> MirrorPlayerAppearance was not exercised). No new milestones needed.
- Toolkit: no new toolkit issues found (TK-001..TK-003 unchanged; analysis/toolkit-issues.md).
- Status: incomplete (16 unknown bytes). Handoff: analysis/handoffs/round-03-disassembly.md.

## Round 3 / commentary (2026-11-20)
- Map changed (pass-002 -> pass-003): round 2 commentary preserved at analysis/round-2/commentary/;
  fresh commentary/ prepared from pass-003 (byte-identical baseline); round-2 notes/tools carried by
  physical range and reverified.
- Covered all 103 pass-003 regions (49 data notes, 54 code regions named/commented), replaced 149 stale
  gap notes, added symbolic table operands, renamed DecodeGfxToPpu -> WriteAsciiStringsToPpu,
  documented the 7 remaining unknown runs (16 B, L-027).
- review.json 1660/1660 reviewed, 908 units; editorial_audit pending.
- Verification: commentary verify ok, byte_identical=true; reading copy commentary/reading-after.asm.
- Status: COMPLETE (current-source readability). Handoff: analysis/handoffs/round-03-commentary.md.

## Round 3 / redirect (2026-11-20, attempt r03-a1, checkpoint pass-003)
- Research only (ROM/map/state/source unchanged). Evidence: analysis/redirect/r03-a1/notes.md.
- L-028 ready (11 B): b6 $FE24-$FE2B Code (truncated sprite-clear + OAM-DMA idiom, tier A);
  b6 $FE2C Data (opcode remnant, tier B); b0 $E536-$E537 Code (`lda #$02` entry of
  TryLatchOntoPlayer, tier B; artificial latch run shows linked frames $39/$3A/$3B/$5B and a
  consumer read of code byte $E546, so the frame-map-tail alternative is unsupported).
- L-027 narrowed to the 5 lone $60 bytes; re-reviewed, still blocked (rejection review in handoff).
- Gameplay: 2P Game B (artificial $16 = 2) and clinging-enemy latch visited; no unknown bytes executed.
- Toolkit: no new issues. Server shut down.
- Status: READY-FOR-DISASSEMBLY. Handoff: analysis/handoffs/round-03-redirect.md.

## Round 4 / disassembly pass 4 (2026-11-20)

- Input: pass-003 (analysis/state.json restored with map.load, 1,653 regions preserved; baseline present).
- Checkpoint: pass-004 (maps/pass-004.json, generated/pass-004, analysis/pass-004-verification.json).
  Verification: ok=true, byte_identical=true, map sha256
  514387bdc53a8e49f6875de65927c74a17786ecfaceec7059505676594bd3b96. state.json regions (1,656) ==
  maps/current.json == maps/pass-004.json.
- ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).
  (99.998% rounded; NOT complete while 5 bytes remain unknown.)
- Code 61,119 bytes (+10 Static); Data 201,036 bytes (+1 Static). 3 new regions via map.annotate
  (analysis/tools/round04/apply.py, the exact batch).
- Per bank (classified code / data / unknown):
  - header (16 B): data 16
  - bank 0: 30380 / 2387 / 1
  - bank 1: 839 / 31929 / 0
  - bank 2: 1812 / 30956 / 0
  - bank 3: 3441 / 29327 / 0
  - bank 4: 3620 / 29147 / 1
  - bank 5: 6842 / 25925 / 1
  - bank 6: 13419 / 19347 / 2
  - bank 7: 766 / 32002 / 0
- Lead outcomes:
  - L-028 verified (11 B, Static, reachability unresolved):
    - b6 $FE24-$FE2B (phys 0x37E34, 8 B) Code conf 1.0: jsr $FE19 HideSprite / stx $2003 / lda #$02;
      the idiom 8E 03 20 A9 02 8D 14 40 is complete at b0 $FE0A and b4 $C5F5 (byte search); predecessor
      is rts $FE23; three complete instructions fit the region exactly. No jsr/jmp/word reference to $FE24.
    - b6 $FE2C (phys 0x37E3C, 1 B) generic Data conf 0.8: truncated $8D (sta $4014 without operand).
      Executed it would be sta $00A2 over ldx #$00 at $FE2D, clobbering RunRamColumnCode input $A2
      (read by lda $009F,y, Y 0..3) before use, so the overlapping path is not a coherent entry; the
      map cannot represent overlapping code anyway. Non-executing remnant byte -> generic data.
    - b0 $E536-$E537 (phys 0x06546, 2 B) Code conf 0.8: lda #$02 falling into TryLatchOntoPlayer $E538
      (sta $D1 hit-box id; sole caller $E5D1 does lda #$03 / jsr $E538; $D1 consumers ldy $D1 /
      lda $8B05,y/$8B23,y/$8B41,y and cmp #$03/#$08 accept id 2). Frame-map-tail rival judged
      unsupported: static linked-frame writer $B8FE-$B91D ($39-$3B), observed $5B discarded after
      cpy #$5B (read lands on code $E546), $97 no-store sentinel run $E531-$E535 ends the meaningful
      entries, and $A9/$02 fall outside the observed $99-$9D tile values. Not an exhaustive frame-domain proof.
  - L-027 not retried (no new reference/method); still blocked (5 B).
- Remaining unknown ranges (5 B, all lone $60, L-027; dead rts vs unread pad, no reference/consumer):
  b0 $8D92 (phys 0x00DA2), b4 $B979 (0x23989), b5 $ACFA (0x2AD0A), b6 $9745 (0x31755), b6 $EFE5 (0x36FF5).
  Rejection review: analysis/handoffs/round-03-redirect.md (premises unchanged).
- Gameplay: no new scenarios needed (all milestones visited in earlier rounds; scenarios.json unchanged).
- Toolkit: no new toolkit issues found (analysis/toolkit-issues.md; TK-001..TK-003 unchanged).
- Commentary: 3 new regions need commentary carry-over from pass-003 (labels: none added).
- Status: incomplete (5 unknown bytes). Handoff: analysis/handoffs/round-04-disassembly.md.

## Round 4 / commentary

- Status: COMPLETE (current-source readability on pass-004). Handoff: analysis/handoffs/round-04-commentary.md.
- Round-3 commentary preserved at analysis/round-3/commentary/; fresh baseline prepared from pass-004;
  notes carried by physical range and reverified byte-identical.
- New commentary: TryLatchOntoPlayer_Id2 (b0 $E536), HideSpritesThenOamDma_Fragment (b6 $FE24),
  data note for b6 $FE2C; stale unknown notes removed. 1661/1661 review entries reviewed, 910 units.
- verify ok=true, byte_identical=true (game.asm sha256 2894d99b...); reading-after.asm refreshed.
- Remaining: 5 unknown bytes (L-027, noted); editorial audit pending (final acceptance only).

## Round 4 / redirect

- Status: TERMINAL (incomplete, 5 unknown bytes). Handoff: analysis/handoffs/round-04-redirect.md. Attempt: analysis/redirect/r04-a1/.
- Q-R4-1 tested: ROM-wide survey of terminals and following bytes, padding values, authoring analogues and
  neighbour extents. Orphans are not uniformly $60 ($FF/$8B8E/$8D remnants exist), $FF fill exists between
  routines (incl. a lone $FF after rts), no dead-rts habit (0/1,829; local twins lack it). Not discriminating.
- leads.json: L-027 updated (still blocked); pass record round-04-redirect added (no ready leads).
- Map/state/source/ROM unchanged; no server started; scratch 3.2 MB, ~4.1 GB free.

## Round 5 / batch final acceptance — USER-DIRECTED MANUAL FALLBACK (2026-11-20)

- Authorization: explicit user batch-only exception (<= 10 residual bytes) to mark remaining
  unknown bytes as generic Data, unknown meaning, Manual provenance. Not evidence-backed.
- Verified complement first: exactly 5 unknown bytes, physical 3490, 145801, 175370, 202581,
  225269 (1 byte each; b0 $8D92, b4 $B979, b5 $ACFA, b6 $9745, b6 $EFE5) == user list.
  Prior ambiguity: L-027 dead rts vs unread pad/table byte (reviews round-03/round-04 redirect).
- Annotated (analysis/tools/round05/apply_fallback.py): Data/Manual/conf 0.5, ids 1657-1661.
- Checkpoint pass-005: ok=true, byte_identical=true, map sha256 a4669c669caae551...;
  state.json == maps/current.json == maps/pass-005.json.
- ROM classification: evidence-backed 262155 / 262160 (99.998%); manual fallback 5 bytes;
  map coverage 100.00% (262160 / 262160, 0 unknown). Fallback bytes are reported separately.
- Commentary: round-4 copy preserved at analysis/round-4/commentary/; re-prepared on pass-005,
  notes carried; final strict editorial audit passed (commentary/audit.json ok,
  byte_identical, 1661 regions / 1761 units).
- Handoff: analysis/handoffs/round-05-manual-fallback.md. Status: final acceptance done with
  manual fallback; no other blockers. Archive not run (archive-options.json prepared).
