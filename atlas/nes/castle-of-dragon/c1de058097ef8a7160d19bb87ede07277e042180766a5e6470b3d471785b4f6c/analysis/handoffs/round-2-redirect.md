# Round 2 redirect handoff (attempt r2-a1)

- Attempt dir: analysis/redirect/r2-a1/ (tools/, evidence/{traces,logs,frames,snapshots,static_regions_now_fetched.txt},
  leads.before.json, scenarios.before.json). r1-a1 untouched. Scratch /root/r2a1scratch removed.
- Input checkpoint: pass-002b + round-2 commentary source (byte-identical). No map/source edits.
- ROM classification unchanged: 100.00% (131087/131088; unknown $C415 / phys 0x1C425, L-001).
- Methods: static control-flow review of commented source; bounded runtime via `romatlas serve` (configured release
  binary) with RAM/PC assists. New reusable tools: warp.py (start stage k at any map page: sentinel $F3, run_until PC
  $E626 in SetupStage, write ScrollXPage so the game draws the page itself), play.py (page play with HP/guardian/pit
  assists; boss_fight() aiming at BossHitBoxTbl weak spots), stagerun.py / guardians.py / bossclear.py, newobs.py,
  fetchbytes.py, branchscan.py. All patches recorded in scenarios.json entries and logs.

## Findings
1. Boss rooms 3 and 4 visited and defeated (natural door, natural hits; one BossHitPoints=0 assist after the first hit):
   stage clear -> StagesCleared bit -> all-buttons abort -> map screen (S-BOSS3, S-BOSS4).
2. Stage 5/6 boss rooms have NO natural entry: SpawnPageObjects skips the $7F door test for StageIndex 5/6, and
   LoadStage's other callers are stage-7 only. Forced entry (PC $90D8 -> $E129, artificial) shows intact bosses
   (hydra / dragon) whose defeat never clears (UpdateHudAndStageEnd uses the end-page check for 5/6; BossHitPoints wraps).
   Stages 5/6 instead clear on their last page (observed stage 6 page 48). Cut/disabled content; structure unchanged.
3. Stage-3 title drop (S-PROG3) = pit death: cov_prog3 edges $FCE8 -> $FCF7 @GameOver, frame $D0 figure, JMP Reset.
4. "SpawnStage7BossBody" ($C5D4) is the HalfPageSpawnTbl type-$24 handler (pages 14/15/22/34/35/38, never stage 7);
   observed spawning the floating platforms on stage 2 page 14 (lift loop $C6E7 follows). Misleading label.
5. Map route select semantics (Down/Up clamp 3..6, Right 7, from 7 Down->5/Up->4) observed.
6. MsprFrameData $8A1D-$8A40 is a unique tile-grid-like block (not a code copy); $8F89 = $00 at zero-size frame
   pointer $D1-$E5. Stays inside MsprFrameData; note wording only.
7. Coverage: 62 Static code regions received fetches (1566 starts, ~3760 of 7078 Static bytes), all artificial contexts.

## READY leads (analysis/leads.json)
- L-008 provenance batch (13 gz dumps, recipe = round-2 apply_cov.py; mark b5/b6 as forced entry).
- L-009 commentary corrections/semantics (findings 2-6 and boss-hitbox / level-up / guardian $16 notes).
- L-002: no change (fragments still unfetched). L-001 blocked (review below).

## Scenarios visited (scenarios.json)
S-BOSS3, S-BOSS4 (visited), S-BOSS5F, S-BOSS6F (forced), S-GUARD (8 guardian pages + stage 6 clear), S7-WALK2 (guardian
$16, natural door to dragon, level-up), S2-LIFT, S-MAPSEL; S6-PROG attempted (terrain loop page 44). S-BOSS3-6 superseded.
Ending already visited in r1-a1 (END).

## Deferred / remaining avenues (with reasons)
- Remaining unfetched Static clusters (~3.3 KB; largest: test-mode sprite/BG-test paths $DE64/$DFA2/$E0DA/$DDCF/$DD9F,
  L-002 fragments $B44B/$A870/$CCE6, guardian fighter blade $9CA3, dragon input branches $B1ED/$B1AB, EnemyWobbleFly
  $ABF5, AltWeaponB left $F2DF, type-$0E contact $B9C1, FlushVramQueue copy loop $C3C0, IRQ $C0D8 (mapper 2 never
  raises IRQ), ending fade $D770 (after endless yield)). All are already classified code; only provenance would change.
  Concrete continuation if wanted: test-mode BGTEST/SPTEST sub-options from snap_cheat (round-1), guardian pages 4/13/19/24
  types $04/$0A/$0C fighter variants with guardians.py. Diminishing value (provenance only); not required for a cycle.
- Natural (unpatched) play-through: not attempted; RAM shortcuts sufficed; pits make long unassisted runs costly.
- Object/sound identity naming: semantic only; no new consumer found beyond the commentary's tables.
- Stage 6 page 44 terrain loop: player controller limitation, no code question attached.

## Rejection review (L-001, $C415, the only unknown byte)
- Claim needed: $C415 (bank 7 fixed, phys 0x1C425) is code (orphan RTS stub) or data/padding.
- Decisive evidence: byte $60 between ClearSpritesAndObjects' real end RTS $C414 (beq $C40E targets $C414) and
  InitFramePool $C416 (Reset JSR). Round 1: no absolute/RTS-trick/split-table reference. NEW this round: instruction-aligned
  scan of all 577 Code regions (1168 relative branches, all JMP/JSR/JMP-indirect operands; tools/branchscan.py) finds no
  target $C415; never fetched in any trace incl. 13 new r2-a1 scenarios.
- What it rules out: any static entry by branch, jump, call or table; it does not distinguish "dead stub" from "stray byte".
- Constraints together: contiguous code bank, existing dead fragments (L-002) make an orphan stub plausible; bank fill is
  $00 not $60, so no fill-run role; no alignment role ($C416 unaligned).
- Strongest competing interpretation: assembler leftover RTS (code) vs one stray data byte; a single unreferenced return
  opcode lacks structure per evidence-policy.md. Remaining reason: code-vs-data materially ambiguous -> keep unknown.
  No revised test exists inside the ROM (needs external source/toolchain evidence).

## Recommendation
(a) One more disassembly/commentary cycle with ready leads L-008 (provenance, ~3.7 KB Static->Observed in artificial
context) and L-009 (label/notes corrections incl. stage 5/6 cut-content notes). Classification stays 99.9992% ($C415);
afterwards redirect is expected to be terminal-incomplete for $C415 with the review above unless new external evidence.
Processes: all romatlas serve instances started here were stopped.
