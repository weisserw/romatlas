# Round 2 redirect handoff

Scope: redirect round 2 on checkpoint pass-002 + round-2 commentary. ROM, maps/current.json, maps/pass-002.json and
analysis/state.json hashes unchanged (analysis/redirect/r2-a1/input-hashes.txt; map sha256 79bafc80...). No map/source edits.
Classification: 100.00% (131088/131088; 0 unknown). Exact unknown complement: empty.
Attempt dir: analysis/redirect/r2-a1/ (findings.json G1-G7, scripts, coverage files, logs, 3 PNG frames).

## Findings (details: analysis/redirect/r2-a1/findings.json)
- G1 code/data audit (audit.txt): 0 executed opcodes in non-Code across the 7 r1-a1 windows plus 6 new windows; 0 static
  branch/JSR/JMP targets into non-Code; 57 fall-throughs into data, all `jsr FarCall $C852` + 3-byte inline argument triple;
  0 decode problems (no unofficial opcodes, no instruction crossing a region end).
- G2 generic data with no reader: fixed $D2F9-$D7FF, $DDA9-$DFFF, $FF68-$FFDF and all bank tails are 100% $FF. Non-fill generic
  regions scored for code-likeness from 3 decode phases (datacode.txt): none code-like (music banks 45-46% invalid opcodes).
  Generic classification stays sound. No reader needed for fills; no new consumer found.
- G3 code islands (islands.txt): only $C149/$C21B (no word anywhere) and $C8DA/$C92A/bank-5 $A94A (only chance words in
  CHR/DMC data) lack an incoming edge. This is exactly the L-006 set. All five are coherent code; reachability unresolved.
- G4 ending through the ordinary stage-clear path: truck snapshot, World $4B=5/Stage $4C=7, StageClearTimer $B7=1 ->
  StageCleared -> World 6 -> RunEndingAndReset -> ending street walk. No PC edit. This improves the round-1 `pc=$CA5B` route.
  It runs StageCleared $CA31 and the UpdatePlayers clear branch $816F, both Static until now.
- G5 stage sweep (2-1, 3-1, 4-1, 5-1, 6-3; 200 traced frames each): enemy kinds 1, 3-6 and 9-15 are active. 17 Static regions
  ran for the first time (EnemyCollision, EnemyDeathAnim, K05/K06/K11/K13/K14 AI branches, ...).
- G6 kind 22 (L-011 recipe executed): EnemyKind has only two writers (SpawnEnemy; EnemyDeathAnim -> $11). No SpawnEnemy source
  yields $16, and no `0E 16` script pair exists in bank 0. Kind 22 has no producer.
- G7 $020B/$0210: no direct or indexed read anywhere. They are written only by SndCmdSet020B/SndCmdSet0210 (never executed) and
  by sound init. Meaning unknown; no structural impact.

## Ready leads
- L-012 (low, optional; provenance/commentary only, no classification change): mark the Static regions run in the r2-a1
  artificial windows as Observed (artificial). That is 25 regions, at most 555 B; several are only partly covered. Recipe: reuse
  saved r2-a1 end68-cov.txt and st*-cov.txt (no replay needed) with the pass-002 L-009 split/provenance scripts. Mark the
  evidence "artificial World/Stage pokes; natural reachability not claimed". Then verify byte-identical output. Commentary may
  note that the StageCleared ending entry was exercised, and cite observed activity for K12-K15.

## Rejected / blocked / deferred (updated in analysis/leads.json)
- L-002 rejected, reconfirmed on pass-002 (G1/G2).
- L-006 blocked: a caller is still missing (G3).
- L-011 blocked: the producer recipe ran with a negative result (G6).
- L-007 now verified: applied in round-2 commentary.
- Deferred: (1) the meaning of $020B/$0210, which needs a music byte-code stream parse to find songs using those two commands and has no structural use.
  (2) Battle mode, which needs Four Score; an artificial visit already exists. (3) Boss kinds, absent from normal stage lists.

## Rejection review (compact, for a terminal stop)
- Claim "misclassified code hides in generic data": decisive evidence = audit.txt (0 targets, 0 executed opcodes in data, all
  fall-throughs explained) and datacode.txt (no code-like decode). This rules out referenced or executed code in data. It does
  not rule out unreferenced code, which no evidence could support. Competing interpretation (data: $FF fills, music, CHR,
  tables) fits all constraints. No revised test remains.
- Claim "Static code is actually data": every Static region decodes cleanly within bounds, is reached by control flow or a
  dispatch table, or is one of the 5 coherent L-006 entries. 0 counterexamples. Provenance only (L-012).
- L-006/L-011: the missing fact is a caller or producer. The whole-ROM word search and the writer/source enumeration are
  complete. The structural claims do not depend on it.

## Recommendation
(c) Proceed to final acceptance: all bytes are classified, and the editorial audit plus final verification remain. L-012 is
optional provenance polish. It does not need another full round unless the coordinator wants Observed provenance for those
555 B.
Blockers: none. Toolkit: TK-006 added (run_until timeout returns an error instead of reason "timeout"). Server stopped;
scratch/r2a1 pruned (cited frames copied into the attempt directory).
