# Redirect attempt r4-a1 (round 4)

- **Input:** checkpoint pass-004 (analysis/state.json = maps/current.json, 512 regions) and the round-4
  commentary (byte-identical).
- **Not modified:** map, state, commentary source, ROM, toolkit, configured binary.
- **Runtime:** /root/romatlas/target/release/romatlas through run.py serve, socket /tmp/r4a1.sock. The
  server was stopped at the end and the scratch directory /tmp/r4scratch was removed.
- **Reused by reference:** r3-a1 (dec.py, closure.json, gate_writers.json).
- **Copied from r3-a1** with only their paths changed: rpcd/rpc/lib/cov/strike/gain.

## Layout

**tools/**
- idxstores.py: census of reachable indexed and indirect stores.
- preds.py: predecessors of the residual Static regions.
- chrtiles.py: ASCII renderer for CHR tiles.
- signmatch.py: matches a screen cell to a CHR tile.
- e17.py, e18.py, e18b.py: the runtime experiments.
- Runtime helpers.

**evidence/**
- indexed_stores.json
- residual_preds.json
- e17_leftgutter_coverage.json, e17_console.log
- gain_r4.json
- e18_sign2.png, e18_sign_tiles.txt
- inputs.log: every natural input and snapshot import.

## Static work

### S1. Can an indexed or indirect store open the residual NMI gates?

**Question.** r3-a1 gate_writers.json says no reachable direct store opens these gates, but it left
"unresolved indexed stores" as a caveat. Could such a store open AttrRowPending $54, VramReadMode $46,
FillPending $2B, WaitSecondCount $5D or SpriteCycleMode $6C?

**Method.** idxstores.py enumerated all 72 reachable zp,x / abs,x / abs,y / (zp,x) / (zp),y stores and
read-modify-writes (indexed_stores.json).

**Result: none can.**
- The sound driver's zp,x stores run with X = $DF (SoundUpdateBody sets S = X = $DF, and the driver reloads
  X from SoundFrameBase). They therefore write $DF-$EA. StartSound uses X only for absolute,x stores.
- `sta (SND_REG,x)` writes to APU $4000+4n.
- `sta (DigitPtr),y` writes to MathB buffers at $041B+ / $0456+. The four callers all set MathB_Hi = $04
  (+ carry), and Y <= 2.
- `sta (VramReadDest),y` is inside the gated job itself.
- Reset's `sta $0000,y` writes 0, and Random's `sta $0100,x` writes to the stack page.

**Same method applied to StripMode $40.** All 8 reachable stores write #$01, so the NmiCopyStrip column
mode at $FE4C is dormant as well.

### S2. Predecessors of every reachable residual Static region

**Method.** preds.py (residual_preds.json).

**Result.** Region 315 at $F656-$F65B (the FillNametable2/3 entries) has no reference. Its only
predecessor is the fall-through after `ldx #$24 / bne FillNametableBody`, which is always taken.

**Conclusion.** These 6 bytes are unreachable from the vectors. The r3-a1 closure kept them because it does
not evaluate constant flags. They are still code (same status as L-001).

### S3. Sign tiles in CHR

Tiles $84/$94/$95/$96 were rendered from every bank and half (chrtiles.py). On their own the renders were
inconclusive, because the active bank and pattern half were unknown. E18 resolved this.

## Runtime experiments (natural inputs only, no RAM writes)

Settings: emulator.step unit=frame, max_operations 1e8.

### E17. Left gutter ($8A2D)

**Changed inference.** The round-3 note said "left aim seems clamped above $C0". That premise was wrong:
- aiming stops at LaneBallX $C4 (left limit $C5);
- HookCurve0-3 then add -1..-3 per step after HookStartY.

**Run.**
- Start: snap_lane1. Step 300 frames.
- Hold Left for 110 frames, which puts the ball X at $C4.
- Spin meter 0 (HookCurveIndex 1 after AdjustHookForLane), power 16.
- First untraced, then a traced replay.

**Result.**
- LaneBallX was clamped at $C0 in both runs.
- The traced coverage covers $8A2D (region 49, 3 bytes; gain_r4.json).

### E18 / E18b. Bowler announcement sign

**Run.**
- Start: snap_2p_ready. Step 300 frames.
- Bowler 1 throws Left 60 / spin 6 / power 16.
- Frames are then captured until bowler 2's AnnounceBowler.

**Result.**
- The sign reads "2PLAY" (e18_sign2.png).
- The first sign cell matches tile $94 of CHR bank 2, sprite half $0000. In that bank and half:
  - $84, $94, $95 and $96 are the digits 1, 2, 3 and 4 inside the sign border, each with the left edge of "P"
    (e18_sign_tiles.txt).
- The same screenshots show the labels: "POWER" under the vertical bar and "CONTROL" under the arc gauge that
  carries the spin marker.

## Proposals

- **L-019 (disassembly, provenance).** Change region 49 from Static to Observed.
  - Observed 11917 -> 11920; Static 1942 -> 1939.
  - Add an evidence-text note on region 315 (unreachable).
- **L-020 (commentary).** Document:
  - the left gutter clamp;
  - the sign digit tiles;
  - the CONTROL label;
  - the dormant gated jobs;
  - that FillNametable2/3 are unreachable.

## Residual reachable Static code after L-019 (478 bytes)

See the round-4 redirect handoff for each group's disposition.
