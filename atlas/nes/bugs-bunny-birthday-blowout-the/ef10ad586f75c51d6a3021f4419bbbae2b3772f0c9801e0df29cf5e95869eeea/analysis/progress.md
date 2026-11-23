# Disassembly progress

Workspace initialized. Baseline created in round 1 (analysis/baseline-verification.json: ok, byte_identical).

## Coordinator log (goal.md orchestration)

Coordinator keeps only round/phase summaries here; phase workers append their own
detailed sections and compact handoffs (analysis/handoffs/).

| Round | Phase | Status | Input checkpoint | Output checkpoint | Handoff | Notes |
|---|---|---|---|---|---|---|
| 1 | disassembly | completed (incomplete ROM) 2026-11-21 | baseline (created this pass) | pass-001 (ok, byte_identical) | analysis/handoffs/round-1-disassembly.md | 99.69% classified; 814 B unknown |
| 1 | commentary | completed 2026-11-21 | pass-001 | commentary/source (verify ok, byte_identical) | analysis/handoffs/round-1-commentary.md | all banks commented; 987/987 review entries; ROM classification unchanged 99.69% |
| 1 | redirect | completed 2026-11-21 | pass-001 + commentary round 1 (verify ok) | - (research only; no map change) | analysis/handoffs/round-1-redirect.md | ready L-007..L-014: 811 B proposed (735 code, 76 data) + 2 B code->data fix; residual 3 B ($C43F-$C440, $C587); bonus game + game over visited; boss deferred (TK-006). Further disassembly round warranted |
| 2 | disassembly | completed (incomplete ROM) 2026-11-21 | pass-001 | pass-002 (ok, byte_identical) | analysis/handoffs/round-2-disassembly.md | L-007..L-014 all verified (811 B added: 735 code, 76 data; L-014 2 B code->data); 100.00% displayed, 262157/262160; 3 B unknown ($C43F-$C440, $C587); boss attempt (S8) no boss reached |
| 2 | commentary | completed 2026-11-21 | pass-002 | commentary/source (verify ok, byte_identical; round-1 commentary preserved in analysis/round-1/commentary) | analysis/handoffs/round-2-commentary.md | fresh baseline from pass-002, notes carried by physical range; all pass-2 regions commented; backlog: 85 RAM symbols (raw RAM operands 788->8), 172 routine headers, symbolic pointers/constants; 993/993 review entries; classification unchanged 100.00% rounded (3 B unknown) |
| 2 | redirect | completed (terminal incomplete) 2026-11-22 | pass-002 + commentary round 2 (verify ok) | - (research only; attempt r2-b3d9; no map change) | analysis/handoffs/round-2-redirect.md | no ready leads (0 B); L-005 verified (all 6 bosses reached by RAM door-warp, 1-4/2-4 defeated); L-015 3 B residual terminal after rejection review; L-016 (orphan reachability) deferred |
| 2 | coordinator | TERMINAL HANDOFF 2026-11-22 | pass-002 | pass-002 (commentary verify re-run 2026-11-22: ok, byte_identical, game.asm 4dc10a2d...) | this file, section below | workflow terminal incomplete; ROM NOT complete (3 B unknown); archive-options.json written; no archive run |
| 3 | manual fallback | completed 2026-11-22 (user-authorized exception) | pass-002 | pass-003 (ok, byte_identical; map sha256 be088f79...) | analysis/handoffs/round-3-fallback.md | 3 residual B (L-015) annotated generic Data, Manual provenance, conf 0.1 (regions 1908, 1909); NOT evidence-backed; unknown 0; 262160/262160 (262157 proven + 3 manual fallback) |
| 3 | commentary + final editorial audit | completed 2026-11-22 | pass-003 | commentary/source (audit ok, byte_identical; game.asm 0ae28422...; round-2 commentary preserved in analysis/round-2/commentary) | analysis/handoffs/round-3-commentary-final.md | baseline re-prepared from pass-003, carried by physical range (only regions 1908/1909 changed); residual 3 B documented as UNKNOWN-MEANING Manual fallback (not evidence-backed); 16 Tbl_ + 2 address labels named, stale/wrong notes fixed, 20 helper headers; 993/993 regions, 1360 units reviewed; editorial_audit reviewed; no archive |

## Round 1 disassembly pass

ROM classification: 99.69% (261346 / 262160 bytes classified; 814 bytes unknown).

Status: incomplete (814 bytes unknown). Checkpoint `pass-001`: maps/pass-001.json,
generated/pass-001, analysis/pass-001-verification.json -> ok=true, byte_identical=true
(map sha256 54a37984707e248d2a5298374cf7c1e7209ac86d0908b277769171a1aab67dd1).
Internal store analysis/state.json (968 regions), export maps/current.json, map.validate ok.

Game: Kemco 1990 Bugs Bunny ("Happy Birthday Bugs" title; NA: The Bugs Bunny Birthday Blowout), MMC3.

### Per-bank counts (bytes; Data column includes CHR Image)

| Bank | Size | Code | Data | Unknown | Classified |
|---|---|---|---|---|---|
| header | 16 | 0 | 16 | 0 | 16 |
| PRG00 | 8192 | 0 | 8192 | 0 | 8192 |
| PRG01 | 8192 | 0 | 8192 | 0 | 8192 |
| PRG02 | 8192 | 0 | 8192 | 0 | 8192 |
| PRG03 | 8192 | 0 | 8192 | 0 | 8192 |
| PRG04 | 8192 | 0 | 8192 | 0 | 8192 |
| PRG05 | 8192 | 3436 | 4739 | 17 | 8175 |
| PRG06 | 8192 | 0 | 8192 | 0 | 8192 |
| PRG07 | 8192 | 178 | 8014 | 0 | 8192 |
| PRG08 | 8192 | 578 | 7614 | 0 | 8192 |
| PRG09 | 8192 | 4931 | 3248 | 13 | 8179 |
| PRG10 | 8192 | 780 | 7412 | 0 | 8192 |
| PRG11 | 8192 | 6180 | 1995 | 17 | 8175 |
| PRG12 | 8192 | 5843 | 2349 | 0 | 8192 |
| PRG13 | 8192 | 6707 | 1485 | 0 | 8192 |
| PRG14 | 8192 | 6545 | 1635 | 12 | 8180 |
| PRG15 | 8192 | 5383 | 2054 | 755 | 7437 |
| CHR | 131072 | 0 | 131072 | 0 | 131072 |

Totals: Code 40561 (Observed 21041, Static 19520); Data 89713 (incl. 16-byte iNES header;
structured tables/streams + generic); Image (CHR) 131072; Unknown 814.
Execution coverage (separate metric): 10192 unique executed opcode addresses, all inside
classified code (52.7% of statically classified instructions).

### Evidence
- Static recursive descent from vectors + bank-pair dispatcher contracts + inline jump
  tables + inline-parameter calls (details: analysis/investigation.md); toolkit-validated
  regions via map.annotate; observed contexts from round-1 traces
  (analysis/evidence/round1-observed-coverage.json).
- Generic data criteria and checks: analysis/investigation.md "Results".
- Unknown ranges: analysis/evidence/round1-unknown-ranges.json (leads L-001..L-003).

### Scenarios exercised (analysis/scenarios.json)
Power-on/title/intro, Stage 1 play, round clear + bonus board, all six stages via RAM
shortcuts ($0420/$0421/$043F=8), ending and The End screen. Deferred: bosses, bonus game,
game over. Progression log: analysis/evidence/round1-progression.log; screens in
analysis/evidence/screens/.

### Leads
Created L-001..L-006 (analysis/leads.json); none pre-existing, none attempted beyond creation.

### Verification
`python3 run.py checkpoint pass-001` -> {"ok":true,"byte_identical":true}.

### Toolkit findings (analysis/toolkit-issues.md)
Confirmed bugs: TK-001 (NES emulator.input ignores documented button/pressed), TK-003
(discover_context ignores `budget`, undocumented entry shape). Missing features: TK-002
(tracing ~5-10x slowdown even at capacity 0), TK-004 (MMC3 static discovery stops at every
JSR/indexed store). No blockers (workarounds used).

## Round 1 commentary pass

Complete for current source (details: commentary/progress.md, handoff
analysis/handoffs/round-1-commentary.md).  Verification: commentary.py verify ok,
byte_identical; reading copy commentary/reading-after.asm.  ROM classification unchanged
(99.69%, 814 unknown).  Structural questions for redirect listed in the handoff
(bank 13 $A024 data-as-code, motion-script unknowns, stage-select reachability).

## Round 2 disassembly pass

ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).

Status: incomplete (3 bytes unknown; the rounded 100.00% is not completion). Checkpoint
`pass-002`: maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json ->
ok=true, byte_identical=true (map sha256 88ab22c692879ae492f6e3910e735172c7b3995429a5419d629f11d1aa61961e).
Internal store analysis/state.json (991 regions; restored with map.load from pass-001 state,
all existing classifications preserved except the L-014 shrink of region 1438), export
maps/current.json, map.validate ok. Script: analysis/evidence/round2-tools/apply_r2.py.

### Per-bank counts (bytes; Data column includes CHR Image)

| Bank | Size | Code | Data | Unknown | Classified |
|---|---|---|---|---|---|
| header | 16 | 0 | 16 | 0 | 16 |
| PRG00-04, PRG06 | 8192 each | 0 | 8192 | 0 | 8192 |
| PRG05 | 8192 | 3436 | 4756 | 0 | 8192 |
| PRG07 | 8192 | 178 | 8014 | 0 | 8192 |
| PRG08 | 8192 | 578 | 7614 | 0 | 8192 |
| PRG09 | 8192 | 4935 | 3257 | 0 | 8192 |
| PRG10 | 8192 | 780 | 7412 | 0 | 8192 |
| PRG11 | 8192 | 6180 | 2012 | 0 | 8192 |
| PRG12 | 8192 | 5843 | 2349 | 0 | 8192 |
| PRG13 | 8192 | 6705 | 1487 | 0 | 8192 |
| PRG14 | 8192 | 6554 | 1635 | 3 | 8189 |
| PRG15 | 8192 | 6105 | 2087 | 0 | 8192 |
| CHR | 131072 | 0 | 131072 | 0 | 131072 |

Totals: Code 41294 (Observed 21041, Static 20253); Data 89791 (Static); Image (CHR) 131072;
Unknown 3. Execution coverage unchanged from round 1/redirect (new code is Static, no fetch).

### Lead outcomes (all reviewed independently; details analysis/investigation.md)
- L-007 verified: 2x17 B ApplyMotionScript records (Type12HitMotion, Type00MotionScript).
- L-008 verified: 3x3 B metasprite / tile-rect-fill records in bank 9.
- L-009 verified (confidence 0.7): $A097 LDA #0/STA $58 prefix of WaitButtonBlinking.
- L-010 verified: 135 B fixed-bank helper variants + 4 B data ($FFF6-$FFF9).
- L-011 verified: 148 B second-nametable DrawInlineScreen/RLE twins.
- L-012 verified: 5 B data + 12 B duplicate code (confidence 0.75) + 4 B $FF data.
- L-013 verified: 436 B helper library + 16 B AttrPaletteBits + 4 B AttrClearMasks.
- L-014 verified: $A024-$A025 code->data StageSelectLimits (region 1438 shortened).
- Residual unknown: PRG14 $C43F-$C440 (phys 0x1C44F-0x1C450, `01 00`) and $C587 (0x1C597, `60`).
- No new code is claimed reachable; all structural claims are Static.

### Scenarios
S8 boss: new bounded attempt (ordinary right+jump play in stage 6 round 4 with health
refills, 25x60 frames) stopped at a wall (PlayerX 467, CameraX $0100) with spike/conveyor
platform route; no boss reached (analysis/evidence/round2-boss-attempt-s6r4.png). Deferred.

### Verification
`python3 run.py checkpoint pass-002` -> {"ok":true,"byte_identical":true}.

### Toolkit findings
No new toolkit issues found this pass; existing TK-001..TK-006 unchanged
(analysis/toolkit-issues.md).

## Round 2 commentary pass

Complete for current source (details: commentary/progress.md, handoff
analysis/handoffs/round-2-commentary.md).  Round-1 commentary preserved in
analysis/round-1/commentary/; fresh baseline prepared from pass-002.  Verification:
commentary.py verify ok, byte_identical (game.asm sha256 4dc10a2d...); reading copy
commentary/reading-after.asm refreshed.  ROM classification unchanged (100.00% rounded;
262157/262160, 3 B unknown, L-015).  Redirect questions: L-015 bytes, reachability of the
pass-2 orphan code and hidden stage select, boss coverage.

## Round 2 redirect (attempt r2-b3d9, input checkpoint pass-002, 2026-11-22)

Result: incomplete. There are no ready classification leads, and the 3 B residual (L-015) is
terminal after the rejection review in analysis/handoffs/round-2-redirect.md. No changes were
made to the map, source, ROM or binary. Classification is still 100.00% rounded
(262157/262160); the unknown complement was re-queried and is unchanged (0x1C44F+2, 0x1C597+1).
- Bosses (L-005 verified, S8 visited, artificial): a door-warp shortcut that patches internal
  RAM only ($0460-$0467 bounds, PlayerX/Y at the door cell, $043F=2) reached all six round-4
  boss rooms. The 1-4 and 2-4 bosses were defeated, reaching the $57 goal and RoundState 8;
  1-4 then went on to Stage 2. The run observed 1375 new opcode offsets, all inside Static
  Code regions; there were 0 conflicts.
- L-015: the only bus reads of the residual bytes are RTS dummy reads. No branch, word or
  split-table reference reaches them, and they were never fetched. The bytes stay unknown.
- L-016 (new, deferred): no observed fetch in the pass-2 orphan code or the stage select. This
  is a reachability question only.
- Leads: L-005 and L-015 updated, L-016 added. Scenarios: S8 updated, S10 (mid-boss rooms)
  added as deferred. Evidence: analysis/redirect/r2-b3d9/.

## Coordinator terminal review (round 2, 2026-11-22)

Rejection review in analysis/handoffs/round-2-redirect.md inspected against
evidence-policy.md and its decisive artifacts (analysis/redirect/r2-b3d9/evidence.json,
residual-rts-dummy-reads.json). Premises checked: (1) runtime "reads" of $C43F/$C587 are
the RTS PC+1 dummy read, so absence of a consumer is not inferred from execution alone;
(2) static negative scope covers relative branches, absolute/indirect words, word-1
(RTS-dispatch) targets, immediate pairs and split tables across all PRG; (3) the code
criterion applied to $C587 matches the only other accepted RTS,RTS case ($AD5E, a word
target), so no conflicting acceptance standard; (4) generic-data fallback is not
supported because neither byte group shows a non-instruction pattern and lack of
execution alone does not prove data. No corrected inference or discriminating test was
exposed, so the coordinator accepts terminal incomplete without another cycle.

Final status: workflow terminal, ROM classification INCOMPLETE.
- ROM classification: 100.00% displayed (262157 / 262160 bytes; 3 bytes unknown:
  PRG14 $C43F-$C440 phys 0x1C44F-0x1C450, $C587 phys 0x1C597; lead L-015).
- Checkpoint: pass-002 (maps/pass-002.json, analysis/pass-002-verification.json ok,
  byte_identical).
- Commentary: round 2 current-source readability complete (analysis/handoffs/round-2-commentary.md);
  commentary.py verify re-run at terminal handoff: ok, byte_identical; reading copy
  commentary/reading-after.asm. Final editorial audit reserved for archive acceptance (not run).
- Leads: analysis/leads.json (L-015 open/terminal, L-016 deferred reachability, L-006 semantics).
- Scenarios: analysis/scenarios.json (ending visited round 1; all six bosses visited round 2;
  S10 mid-boss rooms deferred, coverage-only).
- Archive document selection: archive-options.json (6 handoffs, round-1 unknown ledger and
  round-1 commentary progress/uncertainties preserved; round-1 copies of commentary
  AGENTS/prompt/workflow instructions omitted as identical boilerplate). No archive executed.
- Scratch: /tmp/rb2 (31 MB uncited boss-room snapshots outside workspace) removed at handoff; cited evidence kept in analysis/redirect/r2-b3d9/.
- Resume: reopen only on a new caller/consumer for the residual bytes or explicit user
  reassessment.

## Round 3 manual fallback (user-authorized exception)

Date 2026-11-22. Input checkpoint pass-002 (map sha256 88ab22c6...). Output checkpoint
pass-003.

- Exact residual complement (analysis.unknown on the loaded pass-002 state, re-queried
  before editing): 2 ranges, 3 B, identical to the handoff listing:
  - file 0x1C44F (115791), length 2: PRG14 fixed bank, CPU $C43F-$C440.
  - file 0x1C597 (116119), length 1: PRG14 fixed bank, CPU $C587.
- Prior ambiguity: both groups sit directly after an unconditional RTS. No branch, word,
  word-1, immediate-pair, split-table or indexed reference reaches them. The only runtime
  bus accesses are 6502 RTS dummy reads (PC+1), and they were never fetched. Code vs data
  stayed materially ambiguous, and round 2 ended terminal incomplete. See the rejection
  review in [round-2-redirect.md](handoffs/round-2-redirect.md) and lead L-015.
- Authorization: the user explicitly authorized a batch-only exception. When at most 10
  physical ROM bytes remain unclassified, they may be marked as generic Data with
  UNKNOWN MEANING through the supported annotation API with Manual provenance. This
  overrides the structural-proof requirement for these residual bytes only. No further
  redirect research was done.
- Annotations, made with map.annotate in region form (id 0 = new) inside existing
  unknown gaps; no other region was altered:
  - region 1908: offset 115791, length 2, kind Data, provenance Manual, confidence 0.1.
  - region 1909: offset 116119, length 1, kind Data, provenance Manual, confidence 0.1.
  - Neither region has a label or a semantic role. Their evidence/comment text records
    the user-directed fallback and refers back to L-015.
- **This is NOT evidence-backed classification.** It is a user-directed fallback, not an
  observed or statically proven data role. The code-vs-data question for these 3 bytes
  is still open.
- New counts (internal store, analysis/state.json):
  - ROM classification: 100.00% (262160 / 262160 bytes; 0 unknown).
  - Evidence-backed (Static/Observed): 262157 B. Manual fallback: 3 B.
  - By kind: Code 41294 (Observed 21041, Static 20253); Data 89794 (Static 89791,
    Manual 3); Image 131072 (Static, CHR); Unknown 0.
  - Regions: 993 (next_id 1910).
- Verification:
  - map.validate ok; map.save analysis/state.json; map.export maps/current.json.
  - analysis.unknown re-queried after save and after a fresh reload: total 0.
  - `run.py checkpoint pass-003`: ok=true, byte_identical=true. maps/pass-003.json,
    maps/current.json and analysis/state.json all have sha256
    be088f7905b9e184cdcf32b34a57ec061e39af752fece9848856eb378eb7bc3f.
    Report: analysis/pass-003-verification.json; source in generated/pass-003.
  - input/game.nes sha256 is unchanged (ef10ad58...).
- Leads: L-015 status is now resolved-by-manual-fallback, with its history kept.
- Handoff: analysis/handoffs/round-3-fallback.md.
- Toolkit: one documentation mismatch was logged (TK-007 in analysis/toolkit-issues.md).
  It does not block anything.
- Note: a stale `run.py serve` (PID 1823535, started 2026-11-21, store analysis/state.json)
  from an earlier phase was left running and untouched. It holds a pass-002-or-older
  in-memory map, so do not send it map.save. This phase's own servers were shut down
  (stdin EOF). Scratch /tmp/rf3 was removed.

## Round 3 commentary + final editorial audit (2026-11-22)

- Input: pass-003 (sha256 be088f79...; analysis/pass-003-verification.json ok,
  byte_identical, re-checked).
- Baseline: the map changed, so the entire round-2 commentary/ was copied to
  analysis/round-2/commentary/ (diff -r identical) before `commentary.py prepare`
  re-prepared from pass-003.
  - Scaffold: 993 entries. Only unknown-01c44f -> region-1908 and unknown-01c597 ->
    region-1909 changed.
  - tools/, notes/, progress.md and uncertainties.md were carried by physical range, and
    the source was regenerated, not copied.
  - Before any edits, the regenerated source differed from round 2 only at the two
    residual ranges.
- Manual fallback (reported separately): PRG14 $C43F-$C440 (phys 0x1C44F, 2 B) and $C587
  (phys 0x1C597, 1 B).
  - Emitted as unlabeled data. The comments say UNKNOWN MEANING, user-directed manual
    fallback (L-015) and not evidence-backed.
  - The comments also restate the prior code-vs-data ambiguity: no references, only RTS
    dummy reads.
- Final editorial review:
  - 16 address-only Tbl_ sub-tables and 2 address-only routine labels were given names.
  - EffectFrameTables was renamed EffectXSteps.
  - Stale label references in comments were fixed.
  - A wrong "indexed readers" claim about $E000-$E6F3 was corrected.
  - The 20 largest name-only helpers got headers.
  - Spot checks found no other contradictions.
- Results:
  - `commentary.py audit`: ok, byte_identical, 993 regions, 1360 units, editorial_audit
    reviewed.
  - A fresh `verify` afterwards was ok and byte_identical; game.asm sha256 0ae28422...
- Classification: 262157 B evidence-backed + 3 B Manual fallback (not 100% evidence-backed).
- Handoff: analysis/handoffs/round-3-commentary-final.md. No archive or publication was
  done.


## Coordinator final handoff (round 3, user-authorized exception, 2026-11-22)

The user explicitly authorized a batch-only exception: when at most 10 physical ROM bytes
remain unclassified, mark them as generic Data of unknown meaning with Manual provenance.
The exact complement was re-verified on pass-002 before annotating: 3 B, matching the
authorized ranges, so the exception applied. No further redirect research was done on these
bytes.

- **Evidence-backed classification (Static/Observed): 262157 / 262160 bytes (99.9989%).**
  The round-1/2 evidence is unchanged.
- **Manual fallback, user-directed: 3 bytes.** Not an observed or statically proven role:
  - PRG14 $C43F-$C440, phys 0x1C44F, 2 B: region 1908.
  - PRG14 $C587, phys 0x1C597, 1 B: region 1909.

  Both are Data with Manual provenance and confidence 0.1. Before the fallback the
  code-vs-data question was materially ambiguous (L-015; analysis/handoffs/round-2-redirect.md):
  there are no references, and runtime saw only RTS dummy reads.
- **Map coverage: 100.00% (262160 / 262160; 0 unknown).** This counts the manual fallback,
  so it is not whole-ROM evidence-backed completion.
- **Checkpoint pass-003:** maps/pass-003.json = maps/current.json = analysis/state.json
  (sha256 be088f79...). analysis/pass-003-verification.json: ok, byte_identical.
- **Commentary:** reconciled to pass-003; round-2 copy preserved in analysis/round-2/commentary/.
  - `commentary.py audit`: ok, byte_identical, 993 reviewed regions, 1360 reviewed units,
    editorial_audit reviewed.
  - The coordinator re-ran `commentary.py verify` afterwards: ok, byte_identical
    (game.asm 0ae28422...).
- **Archive selection (no archive run):** archive-options.json selects all handoffs,
  including round-3-fallback.md (the fallback record) and round-3-commentary-final.md, the
  r1 unknown ledger, and the round-1/round-2 commentary progress/uncertainties copies. The
  round-1/round-2 AGENTS/prompt/workflow copies are omitted as byte-identical tool
  boilerplate. Every Markdown file under analysis/ and commentary/ is accounted for.
- **Open items (not blocking):**
  - L-016: orphan-code reachability.
  - L-006: semantics of the generic data banks.
  - S10: mid-boss rooms.
  - TK-001..TK-007 in analysis/toolkit-issues.md.
  - A stale `run.py serve` (PID 1823535, from 2026-11-21) is still running with an old
    in-memory map. Do not send it map.save.
