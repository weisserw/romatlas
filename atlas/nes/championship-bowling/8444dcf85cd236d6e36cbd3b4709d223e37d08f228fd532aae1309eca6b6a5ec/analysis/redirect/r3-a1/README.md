# Redirect attempt r3-a1 (round 3)

Input checkpoint: pass-003 (analysis/state.json = maps/current.json, 511 regions); commentary
round 3 (commentary/source/game.asm, verified byte-identical). Map, state, commentary source,
ROM, toolkit and configured binary were not modified. Binary: /root/romatlas/target/release/romatlas
(through run.py serve, socket /tmp/r3a1.sock; stopped at the end). Prior attempts r1-a1 and r2-a1
are reused by reference only.

## Layout

- tools/
  - static: dec.py (linear 6502 decode of every Code region of analysis/state.json), roots.py
    (closed-world control-flow closure from the vectors and the sound RTS table; enumeration of
    every dynamic transfer), rng.py (offline model of Random $F88D, seed from power-on 0),
    gain.py (Static code regions covered by coverage dumps; copied from r2-a1).
  - runtime: rpcd.py/rpc.py/lib.py/cov.py/strike.py (copied from r2-a1, paths changed),
    e14.py, e15.py.
- evidence/: closure.json, rng_model_k.txt, scorecard_mark_tiles.json, e14_* (result, console,
  coverage, montage, snapshot before the frame-3 strike), gain_e14.json, e15_* (see below),
  inputs.log (every natural input, snapshot save/load and import).

## Static work

| ID | Question | Method | Result |
|---|---|---|---|
| S1 | Is the round-2 L-001/L-002 rejection review sound? Its search was open-world (byte patterns for entry/entry-1, immediate pairs within 16 bytes) and could miss split lo/hi tables, computed pushes, RAM vectors, script opcodes or debug entries. | Corrected method: instead of searching for references, enumerate every instruction that can transfer control to a computed address in the code closure (roots.py on all 6665 decoded instructions) | No `jmp (ind)` outside the three uncalled inline-argument routines; no BRK; no JSR/JMP into RAM; only RTS dispatch with pushed computed words is SoundCmdDispatch ($E17A-$E185, index (cmd-$F0)*2, cmd >= $F0 -> 16 entries = table size); the only other stack writes are Random's A slot ($0100,X) and the sound driver's stack switch (restored); one RTI (NMI). Closure from RESET/IRQ $F22A, NMI $F260 and the 16 handlers = 5904 instructions; the complement is exactly the 23 L-001/L-002 runs, 1461 bytes (closure.json). Every reachable branch target is an instruction start and no reachable instruction falls through into non-code. |
| S2 | Can the random double-strike variant (CelebrationTilesA) be reached naturally? | Read the four Random call sites and Reset; model Random offline | RandomSeed ($05E5) is zeroed only at power-on and changes only inside Random. Callers: Random(7) per throw when PlayerBallWeight != PreferredBallWeight[character] ([2,3,1,0]), Random(4) per throw at StrikeStreak >= 3, Random(10) at double/turkey celebrations. The variant is a deterministic function of the call history. Model check: snap_f10_last.json seed (212,2) = model after 19 Random(7) calls. Model: after exactly 4 Random(7) calls the double's Random(10) returns 7 (>= 6 -> @doubleB). |
| S3 | FrameMarks tile $43: "G" or "-" (commentary uncertainty)? | Render tiles $42-$45/$11 from all CHR banks/halves | In CHR banks 1 and 2, pattern half $1000 (the only half where $11 is the digit "1" and $44/$45 are spare/strike boxes), $42 = "-", $43 = "G", $44 = spare box, $45 = strike box (scorecard_mark_tiles.json). Which bank/half the scorecard uses is inferred from the digit glyph match, not from PPU state. |

## Runtime experiments (emulator.step unit=frame, max_operations 1e8 per call; natural inputs only)

| ID | Start | Inputs | Result |
|---|---|---|---|
| E14 | analysis/snapshots/snap_lane1.json (seed 0, 1P lane 1, character 0, weight 0) | frame 1: two throws Right 40 / hook 6 / power 16 (0 + 0 pins); frames 2 and 3: Left 60 / hook 6 / power 16 (strikes, first try each); the frame-3 strike replayed traced from evidence/e14_before_f3.json | Seeds after each throw match the model exactly ((129,0), (12,3), (168,0); after the celebration (40,7) = model Random(10) result 7). CelebrationTilesA ($BAA6, region 241, 67 bytes, 30/30 starts) and @doubleB ($B847, region 395, 6 bytes) executed (gain_e14.json). Montage e14_double_celebration.png. |
| E15 | r2-a1/evidence/e11_frame10_start.json (3P lane 1, all at frame 10) | per-ball snapshot retry among natural inputs (untraced search; e15_search.log, e15_fix.log), then one traced replay of the kept inputs (e15_kept.json; e15_replay.log: every ball reproduced its search result). Targets: bowler 0 X,9,/; bowler 1 7,/,X; bowler 2 7,/,- | 10th-frame marks $A9 $B1 / $7B $A0 / $7B $C0, totals 289/277/257 (e15_scorecard.png shows X 9 / , 7 / X , 7 / -). Fully covered: $A800, $A86D, $A878, $A8C9, $A8DE, $AB91, $ABA3, $ABB8 (55 bytes); $A845 4 of 5 starts (its `jmp @countBall` needs a third ball after an open 10th, which the frame logic never allows). |
| E16 | snap_lane1.json | one traced throw Left 60 / hook 6 / power 6 | PowerLevel 6 < 8 -> ReleaseBall SFX_ROLL_SLOW branch $86C7 (5 bytes) covered. |

E15 attempt 1 (e15_search_attempt1.log) was stopped: a low-power first ball left 4 pins and 21
blind spare retries failed. The restart used the round-1 9-pin recipe (Left 28/hook 4/power 16)
and strike recipes first for spares, which converted within 1-5 tries. The first kept third ball
for bowler 2 scored 8, so e15.py fix replayed the first 8 balls and retried it (Right 120 / hook 8
-> miss, mark $C0).

## Combined result (gain_r3.json)

11 Static code regions have every instruction start covered by natural traces (133 bytes):
ids 29 $86C7 (5), 133 $A800 (5), 381 $A86D (4), 383 $A878 (10), 385 $A8C9 (7), 145 $A8DE (13),
388 $AB91 (2), 389 $ABA3 (7), 391 $ABB8 (7), 395 $B847 (6), 241 $BAA6 (67). Partial: 379 $A845 (4/5).
If accepted: Code Observed 11775 -> 11908, Static 2084 -> 1951. No boundary change.

## Residual Static code after this attempt (1951 bytes if accepted)

- 1461 bytes: L-001/L-002, outside the vector-rooted closure (S1).
- 490 bytes: statically reachable residual branches. Largest groups are NMI/library jobs gated by RAM
  flags that no reachable direct store opens (gate_writers.json: AttrRowPending, VramReadMode,
  FillPending, SpriteCycleMode 1-$EF, WaitSecondCount); others are fade-delay variants, page-crossing
  carries ($AAF7, $AB07, $BDEB, $E267), sound envelope restart/volume branches, DrawObject $FD jump and
  sprite clipping, the left-gutter test $8A2D and $A845's impossible open-10th third ball.
