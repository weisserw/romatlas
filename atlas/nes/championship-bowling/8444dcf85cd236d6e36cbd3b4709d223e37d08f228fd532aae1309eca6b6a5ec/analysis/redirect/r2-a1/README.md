# Redirect attempt r2-a1 (round 2)

Input checkpoint: pass-002 (maps/current.json = analysis/state.json = maps/pass-002.json, sha256
1476b029...830fb, 407 regions); commentary round 2 (commentary/source/game.asm, verified
byte-identical). Map, state, commentary source, ROM and toolkit were not modified.
Binary: /root/romatlas/target/release/romatlas (through run.py serve). Prior attempt r1-a1 is
reused by reference only.

## Layout

- tools/
  - static: datalabels.py (walks the commentary source and computes the physical offset of every
    label inside Data regions, cross-checked against the region markers), labelrefs.py (how each
    label is referenced: code operand vs .word), splitplan.py (L-016 split/naming plan),
    refine.py (adds tables decoded here, with assertions against ROM bytes), soundparse.py
    (independent parse of envelopes, headers and streams), gain.py (Static code regions covered by
    coverage dumps, instruction-start check), update_leads.py (leads.json append).
  - runtime: rpcd.py/rpc.py (socket bridge to run.py serve; socket from $R2SOCK, default
    /tmp/r2a1.sock), lib.py/strike.py/cov.py (copied from pass-002 tools; inputs appended to
    evidence/inputs.log), e11setup.py, e11.py, e11b.py, e12.py, e13.py.
- evidence/: data_labels.json, data_label_refs.json, l016_split_plan.json,
  l016_split_plan_refined.json, sound_parse.json, l001_ref_search.json, coverage dumps
  (e12_lane2/4/5, e13_title, e11t_results), gain_r2.json (Static regions fully covered: 8 regions, 144 bytes), logs/ (e11, e11b, e12 console logs), snapshots (e11_3p_lane1.json,
  e11_frame10_start.json, e11_scorecard_3p.json), montages, inputs.log.

inputs.log note: E12/E13 ran on a second server (/tmp/r2a1b.sock) while E11 was running, so their
lines interleave in inputs.log; every script is an exact replay on its own.

## Static work

| ID | Question | Method | Result |
|---|---|---|---|
| D1 | Can generic Data regions be split/named with consumer extent proofs (L-003/L-010)? | datalabels.py + labelrefs.py: 438 data labels; 101 referenced by code operands (abs/indexed or #</#>), the rest by .word | l016_split_plan.json: top-level pieces anchored by code refs (A) or by template/descriptor pointers (B) |
| D2 | Do raw immediates in code name unlabelled tables? | scan of lo/hi immediate pairs in commentary source | 5 raw table addresses: $8A90 HookCurve pointers, $B161 BallWeightDigits, $C3EE/$C87A/$CD37 per-character/per-player list tables (ScreenDescPtr reused as list pointer) |
| D3 | Extents of the D2 tables | consumer index domain x record size, asserted on ROM words (refine.py) | $C3EE: 4 chars x 2 ptrs -> 8 x 26 (tile, attr) pairs $C402-$C5A1; $C87A -> 8 x 22 pairs $C88A-$C9E9; $CD37 -> 8 x 20 tiles $CD47-$CDE6; $8A90 9 ptrs -> 9 x 16; PerfectFramePtrs -> 4 per-character 3-word frame tables $BFDE-$BFF5 |
| D4 | Sound data block bounds (L-004) | soundparse.py, driver grammar from StartSound/SoundCmdDispatch/SndCmd*/StartEnvelope/StepEnvelope | envelopes $E620-$E70D; headers+streams $E70E-$F191; $F192-$F19F unreached $FF; no $80-$EF command bytes; unreferenced header $EFC7 and fragments listed in sound_parse.json |
| D5 | L-001/L-002 reachability: corrected premise? | immediate-pair search (new) + word search, entry and entry-1; indirect transfer enumeration | no code-consistent reference; deferral kept (l001_ref_search.json) |

## Runtime experiments (emulator.step unit=frame, max_operations 1e8 per call; traced runs use trace.configure enabled=true capacity=0)

| ID | Start | Inputs | Natural? | Result |
|---|---|---|---|---|
| E11 setup | analysis/snapshots/snap_title.json | 250 f, Right until accepted (+1), Start, each bowler A, A x3, A (P1/P3 port 1, P2 port 2) | natural | 3P lane 1, all character 0; evidence/e11_3p_lane1.json |
| E11 | e11_3p_lane1.json | per-ball aim/hook search from an in-session snapshot (natural inputs only) | natural | frames 1-9: 26 strikes + one spare (P3 frame 1); evidence/e11_frame10_start.json |
| E11b | e11_frame10_start.json | 10th frames with plan retry (max/mid/nine/low); untraced; then traced B, results, A | natural | totals 300/278/257; car, SPORTS NEWS and beach scenes; HighScores 300/278/257 (ranks 1, 2, 3 inserted); title. e11_3p_results.png, e11t_results_coverage.json, e11_scorecard_3p.json |
| E12 | snap_title.json | Up until accepted to lane 2/4/5, Start, A x5, one throw: Left 60, spin pressed at HookCurveIndex 2 (released 3), power 16; traced throw only | natural | covered $9DA9 (lane 2), $9DBB (lane 4), $9DC7 (lane 5); 8 pins each |
| E13 | snap_title.json | traced: Up until accepted, Up x4 (wrap to lane 1), Right x4 (wrap to 1 player), Down x2 (wrap to lane 5, then 4) | natural | covered $B51D (17) and $B545 (12) fully |
