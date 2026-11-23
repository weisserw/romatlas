# Round 3 - redirect phase handoff (attempt r03-a1)

Status: FINISHED for this redirect pass. Recommendation: **(a) another commentary cycle** to apply the ready
commentary-semantic leads L-R3R-01..04. No classification proposals exist. The disassembly step of that cycle
has nothing to apply, apart from optionally re-validating the unchanged map.

## Scope and checkpoint
- **Input:**
  - Checkpoint: maps/round03-final.json = maps/current.json = analysis/state.json (sha256 aab2fb6c...).
    - Classified: 131085/131088.
    - Unknown: 3 B.
  - Source: commentary/source/game.asm (b70bdbb2...), with the commentary handoff gate met (round-03-commentary.md).
- **What was unchanged:**
  - ROM, map, state.json (re-checked against analysis/redirect/r03-a1/state.before.sha), source and the binary.
  - Only emulator calls were made.
- **Research directory:** analysis/redirect/r03-a1/. It contains:
  - NOTES.md, which holds the ledger.
  - tools/: refscan, disk_natural, evwatch, zpuse and excerpt.
  - evidence/
  - leads.before.json
- **Other records:** r01-a1 and r02-a1 are untouched. leads.json and scenarios.json were appended; no earlier
  entries were rewritten.

## Residual ledger (exact complement, refreshed)
| Physical | CPU/bank | Bytes | Disposition |
|---|---|---|---|
| 0x400E-0x400F | b0 $BFFE-$BFFF | BD 49 | TK-04 blocked (L-R1-01) |
| 0x1C010 | b7 $C000 | C1 | Same instruction, TK-04 blocked |

- **Neighbours:**
  - Code Observed 0x3FFD+17 (BmxFrameSprites, ends at $BFFD)
  - Data 0x4010 (bank 1)
  - Data 0x1BFF0+32 (bank 6 tail)
  - Code Observed 0x1C011 ($C001, BmxFrameTail)
- **Proposed versus accepted bytes this pass:** 0 proposed and 0 accepted.

## Ready leads (analysis/leads.json; full hypotheses and recipes there)
| Lead | Kind | Content |
|---|---|---|
| L-R3R-01 (high) | commentary-semantic | Flying disk RAM contract. It shows that the "catcher" routines are actually the **two-stage throw meter**: Left starts the back-swing and Right turns it, which sets the power. That sets $B8 and the disk X speed $A7/$A8. The marker then returns; Left or reaching $DF stops it, which sets the launch speed $A9/$AA and $B7. The lead also covers the disk object, the thrower and the catcher variables. It gives about 25 byte names and 10 routine renames. |
| L-R3R-02 (medium) | commentary-semantic | Half pipe: two 8-byte skater records, $91-$98 and its per-frame saved copy $99-$A0 (+$B1->$B9). Fields: ramp position, pipe side, horizontal direction, 8.8 speed, sprite X/Y. |
| L-R3R-03 (low) | commentary-semantic | $EE7D = first special text line per TextScreenKind ($FF = none). $EE86 = cursor sprite X per kind. Each has 9 entries, inferred from the caller domain 0-8. |
| L-R3R-04 (low) | commentary-observation | The Static flying-disk compete retry block $AFD8 and the rising-flight block $B387 behave as read. This is a RAM-level observation; provenance is unchanged. |

Evidence for all four leads:
- Watch excerpts: evidence/watch-disk{,2,3}.txt and watch-pipe.txt.
- Replay scripts: evidence/watch-scripts.txt.
- Static operand index: evidence/zp-overlay-use.json.

All of it is reproducible with `tools/evwatch.py EVENT NAME TOKENS`. It uses the practice snapshot and makes no RAM writes.

## Findings
- **S12 natural ending reached** (artificial=false, no writes):
  - Cause of the r02 stall: a flying-disk round ends only when the disk lands ($AE >= $A8 in DiskFlightStep) and
    has no timer, so mashing never completes the meter throw.
  - Route: a deterministic replay of r02 run 2 to flying disk, then the throw recipe applied three times
    (EventTries 3->0).
  - Then one A press at EventIntroFrame, giving CurrentEvent 6 and the FINAL RESULTS trophy screen
    (evidence/natural-final-results.png). The game returned to the main menu after the timeout.
  - Run 1 confirmed the static S12 explanation: an extra press at CurrentEvent 6 skips FINAL RESULTS.
  - Snapshots: scratch/snap_r03_disk_nat.json and snap_r03_final_nat.json.
- **S09 correction:** the old recipe turns the meter at $B5 >= $70, so power is 0 and the disk drops (score 0).
  A real throw recipe is recorded in watch-disk3 and scenarios.json.
- **Reachability of low-confidence code** (L-R2R-02 lone JMPs, L-R3-01 $8008, and the other round-3 no-caller
  entries):
  - A whole-PRG operand/RTS-trick pair scan found no reference (evidence/refscan.json).
  - Status remains "no caller/selector known". The structural classification is unaffected.
- **Small BMX corroboration** (watch-bmx.txt, wipe-out run). These match the static prose and are usable as
  tentative names:
  - $C2 counts 0->$30 during the crash slide.
  - $C1 counts down from $14 before the retry.
  - $C3 = $40 after the restart.
  - $C7/$C8/$C9 = rider sprite X/Y/mirror.

## Compact rejection review (TK-04: 0x400E-0x400F + 0x1C010)
- **Candidate claim:** these 3 bytes are one Code instruction, `lda $C149,x`, executed with prg_banks [0,1,14,15].
  - Opcode at b0 $BFFE.
  - Final operand byte at fixed b7 $C000.
- **Decisive evidence:**
  - Phase-1 observed opcode fetch at physical 0x400E (L-R1-01).
  - Execution continues at $C001 (Observed region 0x1C011).
  - Bytes are BD 49 | C1.
- **What it rules out:** any non-code role for these bytes. Data, Text, Padding and Image all contradict the fetch.
  It also rules out an alternative alignment, because Code at 0x1C010 alone would decode C1 as a 2-byte
  `cmp (zp,x)`, which contradicts the observed fetch at $C001.
- **Representations checked against the toolkit source:**
  - RegionKind = Code/Data/Text/Padding/Image/Unknown (romatlas-core lib.rs:16).
  - Only Code carries views.
  - decode_region (romatlas-nes mapping.rs ~205-232) requires each instruction byte at physical offset+byte under
    the view.
  - 0x400F->0x1C010 is non-adjacent under every prg_banks value.
  - A 2-byte Code region cannot hold the 3-byte instruction.
  - The old recipe idea of annotating only $C000 as Data with a comment is rejected: it is an operand byte, so the
    annotation would be a false structural claim.
  - The only supported non-contradicting option is an explicit RegionKind::Unknown region carrying the evidence
    text in the portable map. It is documentation only (0 bytes gained) and optional.
- **Independent constraints considered together:** the observed fetch, the successor fetch at $C001, the bytes,
  and the validator contract. They agree with one another; none of them supports a representable alternative.
- **Strongest competitor:** none is credible. The instruction is physically fixed.
- **Result:** the inference is sound; the representation is unavailable. These bytes remain Unknown until the
  toolkit gets an authorized multi-segment/cross-window instruction feature (TK-04). No revised test can change
  this.

## Deferred avenues (with reasons)
- **Remaining numeric overlay bytes for BMX ($CA-$D4 trick counters), skating obstacle ($C9-$D6) and surfing
  extras:**
  - Each single watch gave only value ranges. In the skating watch the obstacle never spawned ($D0=$FF for 250
    frames).
  - Next recipe: a longer evwatch (event 4, >= 900 frames, right-heavy) for skating, and BMX airborne trick input
    (up/left while airborne) for $CA-$CF.
  - The static prose already describes these mechanically. Expected gain is naming only, and the commentary phase
    can derive most of it from source.
- **Half pipe trick-time divergence of records 0 and 8:** not observed ($AB stayed $FF). The recipe is in
  L-R3R-02.
- **Foot bag side object $DE-$E2:** not watched this pass. It has 4 dedicated routines (FootBagSideObject*) with
  clear static data flow, so commentary can name the fields mechanically. A watch is optional.
- **Optional Observed-provenance upgrade of the Static flying-disk blocks** (traced window from
  snap_r03_disk_nat.json): it gives no classification gain and is not needed.

## Gameplay milestones review
All 15 milestones in analysis/scenarios.json are now `visited`:
- S12 natural ending is new this pass.
- S14 was visited artificially earlier and has now also been reached naturally.

There are no unvisited later-game milestones; the game has no separate credits sequence (FINAL RESULTS returns
to the menu).

## Recommendation
**(a) Another commentary cycle** (the disassembly step has no proposals) to apply L-R3R-01..04, plus optionally
the BMX corroboration names.

The classification residual is terminal-blocked by TK-04, as the review above shows. After that commentary
cycle, unless toolkit support for TK-04 is authorized:
- the next redirect should report terminal incomplete (3 B);
- final editorial audit/acceptance can follow.

This pass is finished; nothing needs resuming.

## Disk
About 3.0G free. Scratch additions:
- 2 snapshots (about 4 MB each; cited).
- r03_watch_*.jsonl (disposable).
- PNGs.
