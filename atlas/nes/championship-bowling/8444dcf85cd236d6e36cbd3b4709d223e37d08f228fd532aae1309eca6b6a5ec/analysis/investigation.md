# Investigation ledger

## Round 1 disassembly (checkpoint pass-001)

Method summary (scripts were transient in analysis/scratch, removed; key artefacts kept in
analysis/evidence/pass-001/):

1. Vectors validated: NMI $F260, RESET $F22A, IRQ $F22A (= RESET). CNROM: fixed 32 KiB PRG, CPU
   $8000-$FFFF = file 16..32783, effective prg_banks [0,1,2,3]; no PRG switching exists, so static
   bank context is exact. CHR selected by `lda $F200,x / sta $F200,x` (bus-conflict table).
2. Static discovery (analysis.discover_context, max_instructions 100000, use_observed=false) from
   RESET+NMI: 5851 instructions, 0 diagnostics. Traced gameplay coverage added only $E1B6-$E248,
   reached through the RTS dispatch at $E16F-$E188 (table $E459, index (cmd-$F0)*2, 16 entries).
   Adding the 16 table targets as roots: 5904 instructions / 12398 bytes, covering every observed
   opcode fetch (3868 natural observed instruction starts). Only one push/push/RTS dispatch and no
   reachable JMP (ind) exist (byte-pattern search over PRG).
3. Gap probing: for each unknown gap that starts right after a terminal RTS/JMP, discovery from the
   gap start; accepted when the whole flow used official opcodes, no BRK, no overlap with known
   boundaries, no unresolved decode (indirect returns allowed). 36 roots accepted (1327 bytes);
   4 rejected on review ($9E5B/$9E5C/$9E5F = table bytes `60 50 50 60 60 ...`, $A26B tile data).
   Net +1309 bytes; mostly an unreferenced math/PPU helper library at $F3C9-$FF09 plus inline-argument
   routines $F5C4/$FD29/$FE7F (pop return address, jmp ($xx) after the inline block). Lead L-001.
4. Hidden-code scan of every provisional Data region (linear official-opcode runs ending in RTS/JMP
   whose JSR/JMP targets are known instruction starts and absolute operands plausible): 4 islands
   accepted as Static code ($9D74-$9D8B, $9FFB-$A029, $A045-$A073, $A1DB-$A1FC; +152 bytes, lead
   L-002). Remaining hits ($821C, $8226, $8235, $8EEF, $92F4, $96F9, $A26E, $A38E) reviewed and
   rejected: short `cld/eor (zp),y/php/bvc/beq/rts` style runs inside sprite records.
5. Plausibility checks on all code: no BRK, (zp,x), SED/CLI/CLV, RTI outside NMI in static-only code;
   no absolute operands into $0800-$1FFF, $2008-$3FFF or $4018-$7FFF. All gaps are preceded by a
   terminal instruction (or file start) and no control-flow reference lands inside any gap.
6. Non-code: every remaining PRG gap became Data/Text/Padding with per-region evidence (consumer
   instructions such as `lda $8CBD,x`, immediate pointer setups, self-relative pointer tables,
   record patterns, first non-instruction byte in a linear decode). Specific tables:
   SoundCmdJumpTable $E459 (32), NotePeriodTable $E479 (168, 84 APU periods), SoundPtrTable_E548 (88),
   SoundPtrTable_E5A0 (128), ChrBankSelectTable $F200 (8), Vectors $FFFA (6), texts $8000/$F208/
   $FF0A/$FFE0, $FF/$00 padding runs ($E00B, $F1C3, $F1F6, $FF2C, $FFF0). iNES header as Data.
   CHR-ROM 4 x 8 KiB as Image (PPU-only on CNROM; cannot be CPU code).

## Result

All 65552 bytes classified (code 13859: Observed 8077 / Static 5782; data 18548 incl. 16-byte header;
text 116; padding 261; image 32768). Classification is structural; many data blocks are generic
(format not decoded: leads L-003, L-004). Reachability unresolved for L-001/L-002 code.

## Rejected / not retried

- $9E5B-$9E63 and $A26B-$A2F9 as code (rejected: data patterns, consumers `lda $9E5B,x`, `cmp $9E60,y`).
- Hi-score via $C2-$C7 patch (S8): not the hi-score store; retry only with a traced HUD write source.

## Remaining unknowns

None at byte level. Semantic/format and reachability questions: leads L-001..L-008 in analysis/leads.json.

## Round 1 redirect (attempt analysis/redirect/r1-a1, input checkpoint pass-001)

No map/state/source change. Records: analysis/redirect/r1-a1/README.md (experiment table), evidence/,
tools/; handoff analysis/handoffs/round-1-redirect.md.

Static checks:
- L-011: `lda $A26A,x` (BD 6A A2 at $A205, x = 0..$8F) in Observed DrawLaneOverviewFromRom; table row 0
  60 BE BF 61 06 06 equals LaneTopRows row 0 ($A396), the generated version of the same overview, so
  the RTS byte $60 at $A26A is deliberately shared as tile 0. Keep code; annotate (ready).
- L-002: island immediates point at the data immediately following each island with matching sizes;
  the $9FFA word is a frame pointer to the $FE byte before the code, with an independent twin at
  $BB33 -> $BB77 ($FE) before code $BB78 (naturally executed in E4t). Structure supported; no reference
  to $9D74/$9FFB/$A045/$A1DB entries anywhere (word search for entry and entry-1).
- L-001: word-pattern caller search over PRG for all Static entries not preceded by code: only the
  four islands and the F3C9-FF09 library lack callers. Reachability unresolved; no selector exists to test.
- SpinMeterUnused ($065C-$065F) is OamBuffer sprite 23 (spin meter marker), copied to OAM by
  CopySpritesToDmaPage -> commentary correction L-013.

Runtime (budgets/inputs in r1-a1/evidence/inputs.log):
- E1/E1t natural: B closes the final scorecard; results screen; A -> title (S11).
- E2/E2t ARTIFICIAL totals: hi-score insertion x3 and the 300/275/250 award scenes (S13).
- E3 natural: title Left/Right = players, Up/Down = lane (L-009 confirmed); 4P selection with
  P1/P3 on pad 1, P2/P4 on pad 2 (L-007 confirmed) (S14).
- strike.py grid + E4/E4t natural: strike recipe and celebrations, spare (S12).
- Coverage: natural traces executed 65 Static regions (2809 region bytes, 922 instruction starts);
  artificial-only 4 regions (526 bytes). evidence/static_regions_observed.json -> ready lead L-012.

Negative/method notes: emulator.run_until with per-operation memory conditions was too slow for
frame-scale waits (timed out at 30 s per call); per-frame polling via emulator.step unit=frame was used
instead (not a toolkit bug; documented per-operation semantics). Title inputs within ~370 frames of
snap_title are ignored (first E3 attempt aborted, recorded in inputs.log). Meter presses shorter than
the 4-frame poll interval can be missed.

## Round 2 disassembly (pass-002; input checkpoint pass-001)

Evidence dir: analysis/evidence/pass-002/ (coverage dumps, snapshots, montages, inputs.log, logs/, tools/,
apply_log.json). Server: tools/rpcd.py bridge to `run.py serve` (socket /tmp/r2.sock), stopped at the end.

Leads:
- L-012 verified. tools/apply.py decoded every Static Code region linearly (prg_banks [0,1,2,3]);
  each decode ended exactly at the region end and every covered offset was an instruction start
  (asserted). Fully covered regions -> Observed (87 regions, 2397 bytes); 11 partly covered regions
  split at instruction boundaries into 46 pieces (covered pieces Observed, the rest Static with a
  note). Coverage: natural entries (context.artificial filtered out) of r1-a1 E1t/E3/E4t/E6t and
  pass-002 E7t/E7t2/E9a/E9b/E8/E10t. Code Observed 8077 -> 11631, Static 5782 -> 2228. Per-region
  actions: evidence/pass-002/apply_log.json. Structure unchanged; reachability evidence only.
- Artificial vs natural: round-1 E2t (RAM-patched totals) was the only evidence for Award275 $C288,
  SetTilePairsFromList $C3D5, Award250 $C7AB, SndCmdTranspose $E1F7. This pass reached them with
  natural inputs (299 and 267 games), so their Observed provenance rests on natural runs; E2t remains
  cited only as execution under artificial state.
- L-011 verified: evidence notes on regions 94 (code ending with the $A26A RTS) and 95 (table tail).
- L-013: no map evidence/label used the SpinMeterUnused claim; remains a commentary task.

Runtime (natural inputs; per-ball snapshot retries only choose among natural input sequences):
- E7/E7r/t4: per-ball recipe search -> natural 300 game (S16). E7t/E7t2 traced 10th frame and
  results/award (car). E9a 299 -> 275 award; E9b 267 -> 250 award (S17/S18).
- E8: power-on unskipped title animation; lane 4; selection wrap paths; scorecard during aiming;
  lane-4 throws (S19/S20). E10: 2P game end (S21).
- Mishaps recorded: the first E7 run used a wrong success test (hit==10 for second balls) and kept a
  poor last retry; it was aborted and replaced by E7b with the PinsDownInFrame-based need (log
  overwritten; inputs.log keeps its presses). E9a's client was killed by the harness time limit at
  k=11; e9fin.py finished it in the same session with tracing still on. A 4-frame B press after the
  final ball was not accepted in E7t (E7t2 continued with an 8-frame hold).

Remaining Static code (2228 bytes, 67 regions): L-001/L-002 uncalled library/islands (1461 bytes;
no references, deferred) and ~0.77 KB of small unobserved branches (lead L-014: hi-score ranks 2/3,
lane 5, 10th-frame residual marks, CelebrationTilesA, TitleMenu branches, fade-group paths).
Classification unchanged at 100.00%; no unknown bytes.

## Round 2 redirect (attempt analysis/redirect/r2-a1, input checkpoint pass-002)

No map/state/source/toolkit change. Records: analysis/redirect/r2-a1/README.md, evidence/, tools/;
handoff analysis/handoffs/round-2-redirect.md.

Static:
- Data labels: datalabels.py walked the byte-identical commentary source (438 labels inside Data
  regions, walk ends match every region marker); labelrefs.py: 101 labels referenced by code operands,
  269 only by .word, 68 unreferenced by name (sound streams reached via data, plus 4 tables).
- Raw immediates name 5 unlabelled tables: $8A90 (HookCurve pointers), $B161 (BallWeightDigits),
  $C3EE / $C87A (PlayerCharacter*4: two (tile, attr) list pointers per character for the 275/250 award
  portraits; lists 26 / 22 pairs) and $CD37 (CurrentPlayer*4: two 20-tile lists per player for the sign
  girl). Extents = index domain x record size, asserted against ROM words (refine.py). PerfectFramePtrs'
  raw words $BFE4/$BFEA/$BFF0 are per-character 3-word frame tables.
- Sound (L-004): independent parse of all headers/streams/envelopes: envelopes $E620-$E70D,
  headers+streams $E70E-$F191, $F192-$F19F unreached $FF fill; durations used 1-27; envelope ids used
  <= $1B; unreferenced header $EFC7 and three stream fragments; no $80-$EF command byte.
- Result: L-016 (map split/naming plan, 158 pieces in 54 regions, one 14-byte Padding piece) and
  L-017 (commentary symbols/notes), both ready.
- L-001/L-002: immediate-pair search (new method) + word search for entry and entry-1; indirect jumps
  enumerated (3 inline-argument returns inside uncalled routines; sound RTS table). Only intra-group
  calls and operand-byte coincidences. Deferral kept (l001_ref_search.json).

Runtime (natural inputs only; second server used for E12/E13 while E11 ran):
- E12: lanes 2/4/5 traced throws with spin index 3 -> $9DA9, $9DBB, $9DC7 (S24).
- E13: title wraps traced -> $B51D, $B545 (S25).
- E11/E11b: natural 3P game 300/278/257 -> ranks 1/2/3 insertion, three award scenes (S23); traced only
  from B. $BD33, $BD6C, $BDC6 executed.
- gain_r2.json: 8 Static regions (144 bytes) with every instruction start covered -> L-014 ready
  (provenance only).

Negative/method notes: title inputs from snap_title are ignored until ~420 frames (first E11 setup run
pressed too early; rerun recorded). E11 coverage was not traced during frames 1-10 (cost: ~3.5 traced
frames/s, TK-004), so random celebration variants seen there are not evidence.

## Round 3 disassembly (pass-003; input checkpoint pass-002)

| Lead | Evidence | Status | Result | Retry? |
|---|---|---|---|---|
| L-016 | r2-a1 l016_split_plan_refined.json; independent recheck pass-003/verify_plan.json (abs/indexed operands and #lo/#hi pairs decoded from current Code regions; B/R starts by stored pointer words at template +8 / descriptor words / pointer lists) | verified | 54 Data regions -> 158 pieces, 142 named; region count 407 -> 511 (pass-003/apply_log.json) | no |
| L-004 | r2-a1 sound_parse.json; bytes $F192-$F19F all $FF, code at $F1A0 | resolved | sound block split into envelopes / headers+streams; 14-byte Padding | no (stream splitting = commentary) |
| L-014 | r2-a1 gain_r2.json + coverage files (S23-S25, natural) | 8 regions verified; rest deferred | Observed +144 bytes (11775 / Static 2084) | only via cheap targeted route |

- Extent notes: R-anchored and "declared" pieces have proven extents (index domain x record size or
  declared structure). Other A/B pieces have proven starts; their extent runs to the next anchored start
  or region end and is not claimed as a single table length (stated in each region's evidence).
- Unnamed remainders stay generic Data: sprite-record areas after frame tables, $AD8C-$AD9B, $CDE7-$CE0C
  (19 (tile,$02) pairs, no reference), $C5A2/$CE0D/$BFF6 frame sprite records.
- Gameplay: no new runs (S26 review); L-001/L-002 not repeated (deferral stands, round-2 redirect review).

## Round 3 redirect (input checkpoint pass-003; attempt analysis/redirect/r3-a1/)

| Lead | Evidence | Status | Result | Retry? |
|---|---|---|---|---|
| L-001/L-002 | r3-a1 closure.json (roots.py over dec.py decode of all Code regions) | resolved-unreachable | Corrected premise: the round-2 open-world pattern search is replaced by a closed-world enumeration of every computed control transfer. Only SoundCmdDispatch's RTS dispatch (16 entries = index domain), the NMI RTI and jmp (ind) inside the uncalled routines exist; no BRK, no RAM code. The vector-rooted closure's complement is exactly the 1461 L-001/L-002 bytes. Classification unchanged. | no |
| L-014 | e14/e15/e16 coverage, gain_r3.json | ready | RNG contract + model (rng.py) gave a natural route to CelebrationTilesA (E14, prediction confirmed). Natural 10th-frame patterns (E15) and a low-power throw (E16) covered 11 regions, 133 bytes. | rest deferred |
| L-010 | - | open (low) | formatting only | editorial |
| L-018 (new) | e14_result.json, scorecard_mark_tiles.json, closure.json | ready | commentary updates (RNG/celebration, tiles $42/$43, L-001/L-002 wording) | - |

- Remaining reachable Static branches (490 bytes if L-014 is accepted) are mostly NMI/library jobs gated
  by flags that no reachable direct store opens (gate_writers.json). The rest are carries, fade, sound
  and DrawObject variants and $A845's impossible branch. These are deferred because they give
  provenance only.
- Negative: E15 attempt 1 (blind spare grid after a 6-pin first ball) failed in 21 tries. The
  9-pin recipe and strike recipes converted spares within 1-5 tries.

## Round 4 disassembly (pass-004; input checkpoint pass-003)

| Lead | Evidence | Status | Result | Retry? |
|---|---|---|---|---|
| L-014 | r3-a1 gain_r3.json + e14_double/e15_tenth/e16_slowroll coverage (natural, no RAM edits); starts re-decoded with analysis.decode (prg_banks [0,1,2,3]) in analysis/evidence/pass-004/tools/apply.py | verified | 11 regions (ids 29,133,381,383,385,145,388,389,391,395,241; 133 bytes) Static -> Observed. Region 379 split at the instruction boundary: $A845-$A84D (9 bytes, 4/4 starts covered) Observed; new id 512 $A84E-$A850 (`jmp $A888`, never fetched: needs a third ball after an open 10th) Static. Code Observed 11775 -> 11917, Static 2084 -> 1942 (apply_log.json) | no (residual is provenance only) |
| L-001/L-002 | r3-a1 closure.json | resolved-unreachable (recorded) | Evidence text of all 23 regions (1461 bytes) now states "resolved-unreachable from vectors per computed-transfer census (r3-a1); structure still code". No kind/provenance/boundary change | no |
| L-018 | - | ready (commentary) | not consumed; commentary also needs one new marker for the id-512 split | - |
| L-010 | - | open (low) | formatting only, deferred to editorial | editorial |

- Structural classification unchanged: 100.00%, 0 unknown; 511 -> 512 regions (one split, no kind change).
- Gameplay: no new runs (S31 review). Residual reachable Static code (481 bytes) is gated by flags no reachable
  store opens, specific carries, or variants with no cheap natural trigger; left-gutter $8A2D stays an
  unverified possibility. Provenance only.
- Verification: run.py checkpoint pass-004 -> validate ok, generate, verify ok=true byte_identical=true
  (analysis/pass-004-verification.json; map sha256 d0ea19c3...2db6137).

## Round 4 redirect (input checkpoint pass-004; attempt analysis/redirect/r4-a1/)

| Lead | Evidence | Status | Result | Retry? |
|---|---|---|---|---|
| L-019 (new) | r4-a1 e17_leftgutter_coverage.json, gain_r4.json (natural) | ready | Corrected the round-3 premise "left aim clamped above $C0": aiming stops at $C4, but HookCurve0-3 drift the ball left. E17 (snap_lane1, Left 110 frames / spin 0 / power 16): LaneBallX clamped at $C0, region 49 $8A2D covered (3 bytes). Also: region 315 $F656 (FillNametable2/3) is unreachable, because its only predecessor is an always-taken `bne` after `ldx #$24` (residual_preds.json) | - |
| L-020 (new) | e18_sign2.png, e18_sign_tiles.txt, indexed_stores.json | ready | Sign first tile = bowler digit (CHR bank 2, sprite half $0000; "2PLAY" seen); the arc gauge is labelled CONTROL; gated NMI jobs are dormant (no reachable direct or indexed/indirect store opens them; StripMode is only ever 1); FillNametable2/3 are unreachable | - |
| L-010 | - | open (low) | formatting only | editorial |

- The indexed store census closes the r3-a1 gate_writers caveat:
  - sound zp,x stores use X = $DF;
  - indirect stores target the APU or the $04xx digit buffers;
  - Reset writes 0.
- Residual reachable Static code (478 bytes if L-019 is accepted) has a disposition for each group in
  analysis/handoffs/round-4-redirect.md. All of it is provenance only.
- Negative/neutral: rendering the CHR tiles alone was inconclusive for the sign digits until the
  on-screen cell was matched against CHR.

## Round 5 disassembly (pass-005; input checkpoint pass-004)

| Lead | Evidence | Status | Result | Retry? |
|---|---|---|---|---|
| L-019 | r4-a1 e17_leftgutter_coverage.json / gain_r4.json (natural left gutter, no RAM edits); start re-decoded with analysis.decode (prg_banks [0,1,2,3]) in analysis/evidence/pass-005/tools/apply.py | verified | Region 49 phys 2621..2623 ($8A2D `jmp $8A37`, 1 start, 3 hits) Static -> Observed; whole region covered, no split. Code Observed 11917 -> 11920, Static 1942 -> 1939 (analysis/evidence/pass-005/apply_log.json) | no |
| r4 dispositions | r3-a1 gate_writers.json, r4-a1 indexed_stores.json, residual_preds.json | recorded (evidence text only) | Regions 292,294,296,306,308,311,357,361 ($F2E6,$F319,$F330,$F4D8,$F520,$F588,$FDD4,$FE4C; 264 bytes): dormant by data flow. Region 315 ($F656, 6 bytes): unreachable (no reference; preceding bne always taken). Still Code/Static | no |
| L-020 | - | ready (commentary) | not consumed by disassembly | - |
| L-010 | - | open (low) | formatting only, editorial | editorial |

- Structural classification unchanged: 100.00%, 0 unknown; 512 regions (no split, no kind change).
- Gameplay: none (r4 S32 review found no unvisited practical milestone).
- Verification: run.py checkpoint pass-005 -> validate ok, generate, verify ok=true byte_identical=true
  (analysis/pass-005-verification.json; map sha256 951ffb73...).

## Round 5 redirect (input checkpoint pass-005; attempt analysis/redirect/r5-a1/)

| Lead | Evidence | Status | Result | Retry? |
|---|---|---|---|---|
| L-021 (new) | r5-a1 e19_open_miss_coverage.json, gain_r5.json (natural); fadedelay_flow.json, fade_closures.json, clamp_sim*.json, residual_static.json | ready | Corrected the round-4 premise for $A92B ("needs an out-of-range value"): the first-ball-miss mark $D0 has nibble $D >= $0A. E19 (snap_lane1, two left gutters) gave FrameMarks $D4 and covered region 386 (2 bytes). Static checks turn the deferred fade (160 B), sound (8 B), carry (6 B) and $FD (14 B) residuals into data-flow/data dispositions | - |
| L-022 (new) | r5-a1 README | ready (commentary) | Notes for the L-021 dispositions at each site + uncertainties.md | - |
| L-010 | residual_static.json | open (low) | formatting only; side fact: no ROM record list uses $FD | editorial |

- Negative results: the attenuation clamp and envelope restart are not produced by any of the 64 sound headers
  (simulation with initial SndDuration 0 and 255). The 7 $FD bytes in sprite data never precede a ROM address.
- Only remaining unresolved provenance question: DrawObject top-edge clip $FC23 (15 B), which depends on runtime
  object positions; not seen in S1-S33, and there is no constant to prove it either way.

## Round 6 disassembly (pass-006; input checkpoint pass-005)

| Lead | Evidence | Status | Result | Retry? |
|---|---|---|---|---|
| L-021 | r5-a1 e19_open_miss_coverage.json / gain_r5.json (natural, no RAM/register edits); start re-decoded with analysis.decode (prg_banks [0,1,2,3]) in analysis/evidence/pass-006/tools/apply.py | verified | Region 386 phys 10555..10556 ($A92B `lda #$00`, 1 start, 1 hit) Static -> Observed; whole region covered, no split. Code Observed 11920 -> 11922, Static 1939 -> 1937 (analysis/evidence/pass-006/apply_log.json) | no |
| r5 dispositions | r5-a1 fadedelay_flow.json, fade_closures.json, residual_static.json, attenuation_cmds.json, clamp_sim*.json | recorded (evidence text only) | Fade 405,407,335,333,337 (160 B) dormant by FadeDelay data flow; sound 270,272,276 (8 B) dormant by ROM sound data; carries 166,168,403 (6 B) impossible by arithmetic; 340 $FB8F (14 B) $FD jump unused by ROM sprite lists; 348 $FC23 (15 B) top-edge clip unobserved, deferred (no discriminating test). All still Code/Static | no |
| L-022 | - | ready (commentary) | not consumed by disassembly | - |
| L-010 | - | open (low) | formatting only, editorial | editorial |

- Structural classification unchanged: 100.00%, 0 unknown; 512 regions (no split, no kind/boundary change).
- Gameplay: none (r5 S33 review: no unvisited practical milestone). Experiments: none new (saved natural trace reused).
- Verification: run.py checkpoint pass-006 -> validate ok, generate, verify ok=true byte_identical=true
  (analysis/pass-006-verification.json; map sha256 865f4d5a...).

## Round 6 redirect (input checkpoint pass-006; attempt analysis/redirect/r6-a1/)

| Lead | Evidence | Status | Result | Retry? |
|---|---|---|---|---|
| L-023 (new) | r6-a1 fc23_bounds.json, objy_writers.json, frame_dy_ranges.json | resolved (static disposition); editorial fix pending | Two r5 premises were wrong. **Path scope:** $FC23 runs for any sprite Y sum outside 0..255, not only the top edge. **Unbounded Y:** every ObjY/ObjFlags writer is a ROM template (Y high $00), the Reset clear, or bounded constant/loop arithmetic. Across 17 object contexts, Y + dy' stays within 9..232, so $FC23 is dormant by data flow. Frame tables come only from ROM, which also closes the "RAM-built list" scope of the $FB8F $FD disposition | no |
| L-010 | - | open (low) | editorial formatting only | editorial |

- Static ledger: all 1937 B are dispositioned (r6-a1 static_ledger.json).
  - Unreachable: 1461 + 6 = 1467 B.
  - Dormant by data flow or data: 264 + 160 + 8 + 14 + 15 = 461 B.
  - Impossible: 6 + 3 = 9 B.
- No open provenance question remains. No map change was proposed.
- Gameplay: no new run. The S34 review finds no unvisited practical milestone.
