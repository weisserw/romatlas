# Round 2 — disassembly phase handoff (pass-002)

Status: **complete (easy gains + all ready leads taken); ROM incomplete by 3 bytes.**

ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown).

- Input checkpoint: pass-001 (byte_identical). Output checkpoint: **pass-002** (maps/pass-002.json,
  generated/pass-002, analysis/pass-002-verification.json: ok, byte_identical true,
  map sha256 9a0cd81b…7069). map.validate ok; analysis/state.json and maps/current.json = pass-002.
- Kinds: Code 48,576 (Observed 26,459 unchanged; Static 22,117, +15,572 this pass), Data 279,078
  (+137,503), Padding 65,575 (unchanged). 1,229 regions (was 839).
- Unknown (3 bytes): lone unreferenced duplicate RTS opcodes 23:$B402 (after always-taken
  `bcc $B403` at $B400), 28:$8620 (after `rts` $861F) and 28:$9883 (after `rts` $9882,
  skipped by `bne $9884`). No branch/jump/word reference; kept unknown under the policy
  "an unreferenced return opcode lacks sufficient structure on its own". Not worth another pass
  unless a reviewer accepts "dead duplicate RTS" as code by idiom.

## Per-bank counts (bytes; 8 KiB PRG banks; header 16/16 Data; CHR 131,072 Data)
```
bank  0: code  1771 data  6421 pad     0 unknown    0
bank  1: code     0 data  6640 pad  1552 unknown    0
bank  2: code  1771 data  6421 pad     0 unknown    0
bank  3: code     0 data  6401 pad  1791 unknown    0
bank  4: code  1771 data  6421 pad     0 unknown    0
bank  5: code     0 data  6576 pad  1616 unknown    0
bank  6: code  1364 data  6828 pad     0 unknown    0
bank  7: code     0 data  6770 pad  1422 unknown    0
bank  8: code  1167 data  7025 pad     0 unknown    0
bank  9: code     0 data  5417 pad  2775 unknown    0
bank 10: code     0 data     1 pad  8191 unknown    0
bank 11: code     0 data     0 pad  8192 unknown    0
bank 12: code     0 data     1 pad  8191 unknown    0
bank 13: code     0 data     0 pad  8192 unknown    0
bank 14: code   367 data  1630 pad  6195 unknown    0
bank 15: code     0 data     0 pad  8192 unknown    0
bank 16: code  1469 data  6723 pad     0 unknown    0
bank 17: code     0 data  8002 pad   190 unknown    0
bank 18: code   155 data  8037 pad     0 unknown    0
bank 19: code  1216 data  6771 pad   205 unknown    0
bank 20: code  6442 data  1750 pad     0 unknown    0
bank 21: code  2272 data  5920 pad     0 unknown    0
bank 22: code  4613 data  3579 pad     0 unknown    0
bank 23: code  3190 data  2732 pad  2269 unknown    1
bank 24: code   239 data  7953 pad     0 unknown    0
bank 25: code   178 data  6797 pad  1217 unknown    0
bank 26: code  7635 data   557 pad     0 unknown    0
bank 27: code  1261 data  6396 pad   535 unknown    0
bank 28: code  6643 data  1547 pad     0 unknown    2
bank 29: code    64 data  4147 pad  3981 unknown    0
bank 30: code     0 data  8192 pad     0 unknown    0
bank 31: code  4988 data  2335 pad   869 unknown    0
CHR {'Data': 131072}
```

## Method / verification
Tools: analysis/tools/round-002/ (copied redirect pipeline with scratch /tmp/ra67s; plan1.py,
plan2.py, plan3.py build annotation plans from analysis/state.json; apply.py applies them over
`run.py serve` via daemon.py/rpc.py). Every Code region was decode-checked (official opcodes,
exact fill) before map.annotate and revalidated by the toolkit. Checks for the generic-data batch:
all 2,511 static control-flow targets of classified code land in Code (none in any new Data);
all 13,955 observed instruction starts from round-1/redirect scenarios plus 1,608 (ending) and
2,668 (2P) starts from this pass fall on Code instruction starts (covcheck.py).

## Lead outcomes
- **L-007 verified**: 13 FarCall word tables (104 B; lead's "76" was a sum slip). Labels FarCallTable_NN.
- **L-011 verified**: bank-31 tables/RTS/header (187 B); header checksums recomputed ($0E8B/$98CE).
- **L-008 verified**: candidates reproduced exactly; 9,222 code + 676 table bytes. Inline-parameter
  check of every JSR target with PLA/TSX (only the jump engines consume inline data). 24 regions
  carry counts of artificial-run observed starts in evidence (provenance kept Static).
- **L-009 verified (+extended)**: 1,231 code + 84 table bytes "no caller found"; new this pass:
  27:$A38A (163 B), 26:$9D8C (28), 20:$84FA (9), 28:$9D9F (4). Three 1-byte RTS kept unknown.
- **L-010 verified**: banks 0/4 copy code (3,448 + 2x40 B), $86E9 routine in banks 0/2/4,
  Tbl_26_82AB (12 B; every stadium -> bank 2).
- **L-012 verified**: field data banks 0/2/4 split into palette, 2 CHR lists, metatile sets A/B,
  map (90 rows x 96) and attribute/remaining field data (exact attribute extent unverified);
  bank 3/5 $0F runs. 38,865 B.
- **L-013 verified**: 83,983 B generic Data split at consumer anchors (no labels, to avoid
  collisions with commentary equates); suspect chunks/tails inspected (see leads.json).
- **L-014 verified**: bank 6/7 = unreferenced older variant of bank 16: 1,364 code bytes
  (entries $8003/$809E/$820D match bank-16 FarCall entries), 16-word jump table, 13.6 KB data.
- **L-005 verified (artificial)**: see scenarios below. No new leads produced.

## Scenarios
- **S-012 visited (artificial)**: ending reached. Level contract: zp $66 (0..5, LEVEL $66+1),
  rating $03FB, decision bank 23 $B386 (thresholds 60/70/80/95; 100 only at $66=4 -> 5).
  Recipe in scenarios.json; snapshot analysis/tools/round-002/snapshots/perf_lvl.snap.gz;
  screenshots evidence/ending_congrats.png, ending_credits.png, level5_rating100.png.
  242 new observed starts (banks 19/20/23/31), all inside Static code classified this pass.
- **S-011 visited (ordinary inputs)**: 2P mode (port 2 controls) to in-game play; no new code.
- Correction: redirect's HomeTeam 6->5 was the team-select cursor editor (bank 21 $A818), not progression.

## For commentary / redirect
- New map labels: FarCallTable_NN, RunModeInitTable, RunModeUpdateTable, Metasprite_FA72/FA77,
  CartHeaderTitle/Info, Field{Palette,ChrSetA,ChrSetB,MetatilesA,MetatilesB,Map,Attr}_NN (NN=00/02/04).
  Labels that collided with commentary equates (BcdTable, BankSwitchRestartTable,
  NametableGapTable, InningMusicTable, Tbl_26_82AB) were removed from the map to keep emit.py working.
- 22,117 Static code bytes (most new) need routine names/comments; banks 0/4/6 and orphan
  routines should be described as unreferenced copies/variants with no caller found.
- Correct commentary "Stadium -> bank 0/2/4" (all bank 2) and name $66 Level, $03FB RatingTotal.
- Unresolved (non-blocking): semantics of bulk generic data (music in 8/9, screens in 14/16-19/24-27,
  rosters); exact field attribute-table extent; the 3 RTS bytes.

## Deferred
Nothing actionable remains for classification except the 3-byte policy question. Optional: Observed
provenance upgrades by splitting Static regions at observed starts (not required).

## Toolkit
No new toolkit issues found this pass; TK-001..TK-004 still apply (analysis/toolkit-issues.md).
Scratch /tmp/ra67s removed; serve daemon stopped.
