# Round 2 disassembly handoff

- Status: **INCOMPLETE** (6 bytes unknown); all ready redirect leads consumed. Input checkpoint
  pass-001 (state restored from analysis/state.json, existing classifications preserved; baseline untouched).
- Output checkpoint **pass-002**: `python3 run.py checkpoint pass-002` ok, byte_identical true
  ([analysis/pass-002-verification.json](../pass-002-verification.json), maps/pass-002.json = maps/current.json).

ROM classification: 100.00% (131082 / 131088 bytes classified; 6 bytes unknown).

| 8K bank | classified | unknown |
|---|---|---|
| header | 16 | 0 |
| 0-11 | 8192 each | 0 |
| 12 | 8190 | 2 |
| 13 | 8191 | 1 |
| 14 | 8189 | 3 |
| 15 | 8192 | 0 |

Kinds: Code 30182 (Observed 12483 / Static 17699), Data 84627, Padding 16273.

## Scope and method

Applied [redirect r1-a1 proposals](../redirect/r1-a1/proposals.json) via `scratch/r2/apply.py` after
re-running `closure2.py`/`entries.py` on the pass-001 map (identical results) and spot-decoding each
"other" range. Added $FAAB-$FACA (32-B VRAM-queue table copied by $FA90) missing from proposals.json.
Details: [analysis/investigation.md](../investigation.md) "Round 2 disassembly".

## Lead outcomes

| Lead | Outcome | Bytes |
|---|---|---|
| L-001 bank-5 handlers + fixed fragments | verified (Static code, tables, demo scripts) | 7138 |
| L-014 corrections | verified ($EBF6 code; table $EBE6 now 16 B; bank-10 fill starts $9E49) | 0 net |
| L-009 item records | verified | 420 |
| L-010 bank 6 $90A1-$934B | verified (routines have no known caller) | 683 |
| L-011 $FF06-$FFF9 | verified (routine, DPCM, fills) | 244 |
| L-012 $CF5F-$CF7B | verified | 29 |
| L-013 fragments | verified | 14 |
| L-003 dead-entry fragments | verified, low confidence 0.6 | 87 |
| L-002 bank-6 umbrella | superseded by L-010/L-013 | - |
| L-007 bosses/ending | verified (visited) | 0 |

## Scenarios visited (artificial RAM shortcuts; [analysis/scenarios.json](../scenarios.json))

- SC-BOSS3: Phantom Bat room via camera/position patch + GameState-5 restart; fight, defeat, orb,
  stage clear, map scene, stage 4.
- SC-WARP-CLEAR: write $28=N-1 during stage clear -> stage N with correct graphics (used for 18).
- SC-BOSS18: Dracula, both forms (BossHealth capped), final orb.
- SC-ENDING2: ending, credits, second-quest start (Quest=1) with correct graphics.
- Runtime cross-check: 623 new observed starts, all in existing Static code (213 in bank 10, all at
  closure instruction starts, context [10,11,14,15]); no Data/Padding/unknown executed.

## Unresolved / deferred

- 6 unknown bytes: lone $60 at $C405, $C916, $CA5B, bank-6 $A1E4 and `sec; rts` at bank-6 $9B8B-$9B8C;
  an isolated return opcode is insufficient evidence (decide in redirect/commentary if any new caller evidence).
- Callers of unreferenced routines ($90A1, $910A, $CF61, $FF06, L-003 fragments): reachability unknown.
- Observed provenance not propagated into Static regions (would need region splitting; optional).
- Bosses of stages 6/9/12/15 not visited (recipe exists: StageCameraLockTable $FC3A).
- L-004/L-005/L-006 semantic only; L-008 blocked on toolkit (TK-NES-004).
- Commentary: commentary/ not modified; the next commentary phase must carry the ~9 KB of newly
  classified regions (bank 10 handlers, demo scripts, fixed fragments) into the source.

## Toolkit issues

No new toolkit issues found this pass; TK-NES-001..005 unchanged in
[analysis/toolkit-issues.md](../toolkit-issues.md) (input still requires the `buttons` array; tracing ~10x slower).

Artifacts: scratch/r2/ (apply.py, stats.py, cmpcov.py, cov_*.json, snapshots snap_s3boss/s18h1/s18orb/end_a, screenshots).
