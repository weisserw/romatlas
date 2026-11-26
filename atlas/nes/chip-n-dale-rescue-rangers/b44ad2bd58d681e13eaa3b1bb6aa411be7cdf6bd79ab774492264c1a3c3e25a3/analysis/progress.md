# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Coordinator log (goal.md orchestration)

- Round: 1
- Current phase: disassembly (pass 1) — STARTED 2026-11-26
- Input checkpoint: none (baseline to be established by disassembly worker)
- Output checkpoint: pending
- Leads consumed/produced: none yet
- Notes: disk free ~2.5 GiB at start; keep scratch bounded.

## Round 1 disassembly pass 1 (2026-11-26) - INCOMPLETE (easy gains exhausted for this pass)

- Baseline: maps/baseline.json, generated/baseline, analysis/baseline-verification.json (byte-identical).
- Checkpoints: pass-001 (intermediate, 93.66%) and **pass-001-final** (final for this pass):
  maps/pass-001-final.json, generated/pass-001-final, analysis/pass-001-final-verification.json
  (ok=true, byte_identical=true, map sha256 8404126a16c49e7e...). analysis/state.json and
  maps/current.json saved/exported; map.validate ok.

ROM classification: 97.60% (255857 / 262160 bytes classified; 6303 bytes unknown).

Breakdown by kind/provenance: Code Observed 13437, Code Static 15831, Data Static 225472
(incl. CHR 131072, header 16), Padding Static 1117. 827 regions.

| Bank | Code | Data | Padding | Classified | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 16 | 0 |
| PRG00 | 0 | 7298 | 144 | 7442 | 750 |
| PRG01 | 8 | 7800 | 0 | 7808 | 384 |
| PRG02 | 210 | 7289 | 421 | 7920 | 272 |
| PRG03 | 8 | 7759 | 414 | 8181 | 11 |
| PRG04 | 1697 | 5983 | 0 | 7680 | 512 |
| PRG05 | 282 | 7464 | 0 | 7746 | 446 |
| PRG06 | 2107 | 5865 | 38 | 8010 | 182 |
| PRG07 | 8 | 8184 | 0 | 8192 | 0 |
| PRG08 | 394 | 7276 | 100 | 7770 | 422 |
| PRG09 | 8 | 8184 | 0 | 8192 | 0 |
| PRG10 | 480 | 7139 | 0 | 7619 | 573 |
| PRG11 | 8 | 8120 | 0 | 8128 | 64 |
| PRG12 | 6867 | 202 | 0 | 7069 | 1123 |
| PRG13 | 5064 | 2656 | 0 | 7720 | 472 |
| PRG14 | 7185 | 634 | 0 | 7819 | 373 |
| PRG15 | 4942 | 2531 | 0 | 7473 | 719 |
| CHR (16x8 KiB) | 0 | 131072 | 0 | 131072 | 0 |

(PRGnn = physical 8 KiB PRG bank nn; 16 KiB bank k = PRG 2k,2k+1; PRG14/15 fixed at $C000.)

- Evidence: analysis/investigation.md, analysis/evidence/pass-001/, scenarios S01-S09 in
  analysis/scenarios.json (execution coverage: 6150+ unique ROM opcode addresses observed).
- Leads: L-001, L-002, L-006 verified; L-003 (boss/ending RAM contract), L-004 (6303 unknown
  bytes, more dispatch tables), L-005 (refine generic data) ready for redirect.
- Toolkit: 3 findings in analysis/toolkit-issues.md (TK-001 confirmed bug: NES input
  button/pressed ignored; TK-002 suspected perf bug: trace aggregates slow down; TK-003
  missing feature: MMC1 static discovery stops at every JSR/indexed store). None blocking.
- Handoff: analysis/handoffs/round-1-disassembly.md

## Coordinator log — round 1 disassembly COMPLETE
- Output checkpoint: pass-001-final (verified byte-identical). Printed to user:
  ROM classification: 97.60% (255857 / 262160 bytes classified; 6303 bytes unknown).
- Leads verified: L-001, L-002, L-006. Produced/ready: L-003, L-004, L-005.
- Handoff: analysis/handoffs/round-1-disassembly.md
- Coordinator note for redirect: ~75 KiB generic data was classified at confidence 0.8 on
  "non-decodable" pattern grounds; redirect should sanity-check that rationale (evidence-policy).

## Coordinator log — round 1 commentary STARTED
- Input checkpoint: pass-001-final

## Coordinator log - round 1 commentary COMPLETE
- Checkpoint pass-001-final; commentary/ prepared and fully commented (all classified
  source + unknown notes); verify byte_identical true; reading copy refreshed.
- Handoff: analysis/handoffs/round-1-commentary.md (redirect questions: bank 6 type-$01
  sub-handler table targets, bank 3 inline tables, bank 2 $8900/$A410, consumed data).
- Round 1 commentary COMPLETE: all classified source commented (1932 labels); verification
  commentary/verification.json byte_identical=true; reading copy commentary/reading-after.asm;
  handoff analysis/handoffs/round-1-commentary.md.

## Coordinator log — round 1 redirect STARTED
- Input: checkpoint pass-001-final + commentary round 1; attempt dir analysis/redirect/r1-a1/
- Round 1 redirect COMPLETE (analysis/redirect/r1-a1/, handoff analysis/handoffs/round-1-redirect.md).
  Ready leads L-007..L-014 (6301 of 6303 unknown bytes proposed; L-011 includes 8-byte Data->Code
  correction $FC41-$FC48). L-003 verified (boss/ending warps visited via RAM edits). Recommendation:
  round 2 disassembly.

## Coordinator log — round 2 disassembly STARTED
- Input checkpoint: pass-001-final; leads to consume: L-007..L-014

## Round 2 disassembly pass 2 (2026-11-26) - INCOMPLETE (1 byte unknown)

ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown).
(Exact: 99.9996%. The rounded 100.00% does NOT establish completion: fixed $F62C, physical
0x1F63C, remains unknown.)

- Input checkpoint pass-001-final (97.60%, 6303 unknown); restored analysis/state.json, all
  existing classifications preserved (one existing Data region, id 1582, shortened at its start
  by 8 bytes for the L-011 Data->Code correction $FC41-$FC48).
- Output checkpoint **pass-002**: maps/pass-002.json, generated/pass-002,
  analysis/pass-002-verification.json (ok=true, byte_identical=true, map sha256 8bc887dd...).
- New/edited regions: 204 (6686 B incl. the 376-B re-annotated Data region). Added code:
  2115 B (197 B Observed - every instruction fetched in r1-a1 S10/S11; 1441 B Static 0.95 with
  table/caller entries; 477 B Static 0.75 structure-only unreferenced L-011/L-012). Added data:
  1429 B consumer-identified tables (0.9), 2467 B pattern data (0.8), 298 B filler (0.7),
  1 B $803F filler (0.6, overlap argument).
- Verification method (analysis/evidence/pass-002/, tools/): toolkit analysis.decode with the
  ledger's prg_banks context on every candidate code range (exact tiling, official opcodes
  only, terminal or falls into classified code, all branch/JSR/JMP targets are instruction
  starts); table words of L-007/L-008/L-010 all resolve to instruction starts; successor scan
  of all 556 Code regions finds no control-flow target inside any new Data range.
- No new emulator scenarios this pass (static verification sufficed; r1-a1 S10-S14 fetch
  evidence reused). Gameplay milestones (bosses, ending, select menu, tally) already visited.

| Bank | Code | Data | Padding | Classified | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 16 | 0 |
| PRG00 | 0 | 8048 | 144 | 8192 | 0 |
| PRG01 | 8 | 8184 | 0 | 8192 | 0 |
| PRG02 | 210 | 7561 | 421 | 8192 | 0 |
| PRG03 | 8 | 7770 | 414 | 8192 | 0 |
| PRG04 | 2040 | 6152 | 0 | 8192 | 0 |
| PRG05 | 688 | 7504 | 0 | 8192 | 0 |
| PRG06 | 2183 | 5971 | 38 | 8192 | 0 |
| PRG07 | 8 | 8184 | 0 | 8192 | 0 |
| PRG08 | 394 | 7698 | 100 | 8192 | 0 |
| PRG09 | 8 | 8184 | 0 | 8192 | 0 |
| PRG10 | 480 | 7712 | 0 | 8192 | 0 |
| PRG11 | 8 | 8184 | 0 | 8192 | 0 |
| PRG12 | 7656 | 536 | 0 | 8192 | 0 |
| PRG13 | 5064 | 3128 | 0 | 8192 | 0 |
| PRG14 | 7318 | 874 | 0 | 8192 | 0 |
| PRG15 | 5310 | 2881 | 0 | 8191 | 1 |
| CHR (16x8 KiB) | 0 | 131072 | 0 | 131072 | 0 |

- Leads: L-007, L-008, L-009, L-010, L-011, L-012, L-013, L-014 verified and annotated
  (analysis/leads.json). Kept unknown: $F62C (RTS after JMP; ambiguous code vs filler).
- Toolkit: TK-004 added (map.annotate NES region form requires `id`, docs say optional);
  TK-001..TK-003 unchanged. See analysis/toolkit-issues.md.
- Handoff: analysis/handoffs/round-2-disassembly.md
- Round 2 disassembly COMPLETE: checkpoint pass-002 (verified byte-identical). Printed to user:
  ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown).
  Leads L-007..L-014 verified; bank-4 $803F -> filler data. Remaining unknown: $F62C (phys 0x1F63C) lone RTS.
  Handoff analysis/handoffs/round-2-disassembly.md.

## Coordinator log — round 2 commentary STARTED
- Input checkpoint: pass-002 (map changed vs commentary round-1 baseline -> preserve round-1 commentary under analysis/round-1/)
- Round 2 commentary COMPLETE: round-1 commentary preserved at analysis/round-1/commentary/; all classified
  source on pass-002 commented; commentary/verification.json byte_identical; reading copy refreshed;
  handoff analysis/handoffs/round-2-commentary.md. 28 operands with raw table addresses noted as gaps.

## Coordinator log — round 2 redirect STARTED
- Input: pass-002 + round-2 commentary; attempt dir analysis/redirect/r2-a1/
- Round 2 redirect COMPLETE (analysis/redirect/r2-a1/, handoff analysis/handoffs/round-2-redirect.md).
  Ready: L-015 ($F62C as Static code, conf 0.6, filler fallback), L-016 (provenance upgrade for 230 B
  observed in 2-player scenario; no byte change). Recommendation: round 3 disassembly.

## Coordinator log — round 3 disassembly STARTED
- Input checkpoint: pass-002; leads L-015, L-016

## Round 3 disassembly pass 3 (checkpoint pass-003)

- Input pass-002 (262159/262160). Restored analysis/state.json; all classifications preserved.
- Output checkpoint **pass-003**: maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json
  (ok=true, byte_identical=true, map sha256 a47d91b2...). map.validate ok; state saved; maps/current.json exported.
  input/game.nes sha256 unchanged (b44ad2bd...).
- L-015 verified: phys 0x1F63C (fixed $F62C, ctx [null,null,14,15]) -> Code Static 0.6, region 1811
  `PlayerBump_DeadRts` (vestigial `clc ; rts` epilogue after an inserted JMP). Reachability NOT claimed.
- L-016 verified: 14 code regions (230 B) Static -> Observed from the natural 2-player route (r2-a1 twop-cov.json);
  re-decoded and checked in analysis/evidence/pass-003/l016-check.txt. No classification change.
- No new scenarios (gameplay milestones incl. bosses/ending/2P already visited; no concrete gap found).

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Breakdown: Code Observed 13864, Code Static 17520, Data Static 229659 (incl. CHR 131072, header 16),
Padding Static 1117. 1031 regions, all with evidence text (annotation coverage 100% of bytes/regions).
Execution coverage: 13864 / 31384 code bytes Observed (44.2%); Static remainder is structural inference.

| Bank | Code | Data | Padding | Classified | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 16 | 0 |
| PRG00 | 0 | 8048 | 144 | 8192 | 0 |
| PRG01 | 8 | 8184 | 0 | 8192 | 0 |
| PRG02 | 210 | 7561 | 421 | 8192 | 0 |
| PRG03 | 8 | 7770 | 414 | 8192 | 0 |
| PRG04 | 2040 | 6152 | 0 | 8192 | 0 |
| PRG05 | 688 | 7504 | 0 | 8192 | 0 |
| PRG06 | 2183 | 5971 | 38 | 8192 | 0 |
| PRG07 | 8 | 8184 | 0 | 8192 | 0 |
| PRG08 | 394 | 7698 | 100 | 8192 | 0 |
| PRG09 | 8 | 8184 | 0 | 8192 | 0 |
| PRG10 | 480 | 7712 | 0 | 8192 | 0 |
| PRG11 | 8 | 8184 | 0 | 8192 | 0 |
| PRG12 | 7656 | 536 | 0 | 8192 | 0 |
| PRG13 | 5064 | 3128 | 0 | 8192 | 0 |
| PRG14 | 7318 | 874 | 0 | 8192 | 0 |
| PRG15 | 5311 | 2881 | 0 | 8192 | 0 |
| CHR (16x8 KiB) | 0 | 131072 | 0 | 131072 | 0 |

- Toolkit: no new toolkit issues found this pass; TK-001..TK-004 unchanged (analysis/toolkit-issues.md).
- Handoff: analysis/handoffs/round-3-disassembly.md
- Round 3 disassembly COMPLETE: checkpoint pass-003 verified; ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown). L-015, L-016 verified. Handoff analysis/handoffs/round-3-disassembly.md
- Round 3 disassembly COMPLETE: checkpoint pass-003 (verified byte-identical). Printed to user:
  ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
  L-015 verified ($F62C Static code conf 0.6, reachability not claimed); L-016 verified (230 B -> Observed).
  Handoff analysis/handoffs/round-3-disassembly.md.

## Coordinator log — round 3 commentary + final editorial audit STARTED
- Input checkpoint: pass-003. Every byte classified -> commentary must finish readability, resolve
  pending review/editorial issues (28 raw table operands etc.) and run the final editorial audit.
- Round 3 commentary + final editorial acceptance COMPLETE: round-2 commentary preserved at
  analysis/round-2/commentary/; `commentary.py audit` passes (commentary/audit.json: ok, byte_identical,
  1031/1031 regions and 681 units reviewed, editorial_audit reviewed). Handoff analysis/handoffs/round-3-commentary.md.

## FINAL COORDINATOR HANDOFF (terminal: whole-ROM classified, 2026-11-26)

- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
  Code 31384 B (13864 Observed / 17520 Static), Data 229659 B (incl. CHR 131072, header 16), Padding 1117 B;
  1031 regions, no gaps/overlaps (coordinator recount of maps/current.json). Every PRG bank 8192/8192.
- Final checkpoint: pass-003 (maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json,
  map sha256 a47d91b2...). Coordinator re-verification checkpoint `final-check` (same map sha256) is
  byte-identical (analysis/final-check-verification.json). maps/current.json == maps/pass-003.json.
- Commentary: commentary/source/game.asm + symbols.inc; reading copy commentary/reading-after.asm;
  audit commentary/audit.json; residual semantic uncertainties (object/item identities, sound commands 5-7,
  packed pitch format; ~260 auto-named local branch labels) in commentary/uncertainties.md.
- Lowest-confidence classifications (documented, reachability not claimed): $F62C PlayerBump_DeadRts
  (conf 0.6), L-011/L-012 caller-less routines (0.75), ~75 KiB pattern-based generic data (0.8; sanity-checked
  in redirect r1-a1, only counterexample $FC41-$FC48 corrected).
- Redirect after round 3 not run: no unknown bytes, no ready/pending leads, and round-2 redirect's
  rejection review (analysis/handoffs/round-2-redirect.md) closed the caller searches; a further cycle would
  have no classification target. Reopen only on new contradicting evidence.
- Gameplay: title, zones 0-14 via RAM, bosses, final boss, ending, 2-player (natural), game over visited
  (analysis/scenarios.json).
- Toolkit findings TK-001..TK-004 in analysis/toolkit-issues.md (none blocking; toolkit unmodified).
- Archive document selection: archive-options.json (no archive executed, nothing published or deleted).
