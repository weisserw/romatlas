# Redirect attempt r5-a1 (round 5)

- **Input:** checkpoint pass-005 (512 regions, 100.00% classified) and the round-5 commentary (byte-identical).
- **Not modified:** map, state, commentary source, ROM, toolkit, configured binary.
- **Runtime:** configured binary through run.py serve (socket /tmp/r5a1.sock, stopped); scratch /tmp/r5scratch removed.
- **Reused by reference:** r3-a1 dec.py and closure.json; r2-a1 sound_parse.json; r4-a1 runtime helpers
  (copied into tools/ with only paths changed: rpcd, rpc, lib, cov, strike, gain).

## Static checks (tools/)

| Tool | Output | Finding |
|---|---|---|
| fadeflow.py | evidence/fadedelay_flow.json | FadeDelay $02C5: 5 direct stores ($8029 #$06, $81A7 #$0F, $81C4 restore, $CCAD #$FF, $CD12 restore). Every reachable indexed store into page 2 is bounded below $02C5 (AttrBuffer loops <= $02BF, palette loops < $20, Reset clears it to 0 before init). Callers of FadeInSpriteGroups: only $CCB0/$CD0E, both inside the FadeDelay=$FF window |
| callclosure.py | evidence/fade_closures.json | Nothing called in the $FF window (FadeInSpriteGroups, DrawObject, SetTilesFromList, WaitFrames, LoadPaletteSetAndChr) and nothing in the NMI reaches FadeOutAll, FadeInAll or ShowLaneScreen; the NMI writes no PaletteBuffer |
| ctx.py | (console) | decode windows used for the index bounds above |
| attn.py, clampsim.py | evidence/attenuation_cmds.json, clamp_sim.json, clamp_sim_dur255.json | $F6 attenuation appears only in sounds $0D-$10 channel 7 (values 5/8/10) with 1-frame notes on envelope 0 (first value $FE). Per-note simulation of all 64 headers: no note reaches an envelope value below its attenuation, no envelope restart |
| residual_static.py | evidence/residual_static.json | All 41 envelopes end with a $FF hold; no EnvelopeTable pointer has low byte $FF; RevealHighScoreName low bytes are only $36-38/$76-78/$B6-B8/$F6-F8; digit pointer low byte <= $92; the 7 $FD bytes in the sprite data are followed by RAM addresses only (they are tiles, offsets or a pointer's low byte) |

Conclusions (structure unchanged, all still Code):
- Fade residuals (regions 405, 407, 333, 335, 337; 160 B): dormant by data flow.
  - BrightenColor's cap needs nibble 3. Its only live caller, FadeInAll, brightens twice from nibble 0.
- $E2BE envelope restart, $E267 envelope-pointer carry and $E3D4 attenuation clamp (8 B): dormant by ROM sound data.
- $AAF7, $AB07 and $BDEB carries (6 B): impossible by arithmetic.
- $FB8F DrawObject $FD jump (14 B): no ROM record list encodes it. Lists built in RAM were not considered; none is known.

## Runtime experiment (natural inputs only, no RAM writes)

**E19: first-ball miss followed by an open frame.**
- **Changed premise.** ScoreBall @firstMiss stores $D0. Its nibble, $D, is >= $0A, so the frame-total @open path
  executes `lda #$00` at $A92B. Round 4 had assumed this needed an out-of-range value.
- **Run.** Start from snap_lane1 and step 300 frames, then make two left-gutter throws (Left 110 frames, spin 0,
  power 16, settle 700 frames each). The run was done untraced first, then traced (emulator.step unit=frame,
  max_operations 1e8).
- **Result.** FrameMarks[0] = $D4, and region 386 is fully covered (evidence/gain_r5.json, e19_open_miss_coverage.json).
- **Other files.** e19_console.log, e19_end.png and inputs.log.
